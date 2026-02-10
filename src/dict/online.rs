use anyhow::Result;
use serde::Deserialize;

use crate::types::{DataSource, DictionaryEntry, Example};
use super::Dictionary;

/// Free Dictionary API 在线词典
pub struct OnlineDictionary {
    client: reqwest::blocking::Client,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    word: String,
    phonetic: Option<String>,
    phonetics: Option<Vec<ApiPhonetic>>,
    meanings: Option<Vec<ApiMeaning>>,
}

#[derive(Deserialize, Debug)]
struct ApiPhonetic {
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ApiMeaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: Option<String>,
    definitions: Option<Vec<ApiDefinition>>,
}

#[derive(Deserialize, Debug)]
struct ApiDefinition {
    definition: Option<String>,
    example: Option<String>,
}

impl OnlineDictionary {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .user_agent("lango-cli/0.1")
            .build()
            .unwrap_or_default();
        Self { client }
    }

    fn parse_response(&self, resp: Vec<ApiResponse>) -> Option<DictionaryEntry> {
        let first = resp.into_iter().next()?;

        // 提取音标
        let phonetic = first.phonetic.or_else(|| {
            first.phonetics.as_ref()?.iter()
                .find_map(|p| p.text.clone())
        });

        // 提取英文释义和例句
        let mut definitions = Vec::new();
        let mut examples = Vec::new();
        let mut pos_list = Vec::new();

        if let Some(meanings) = first.meanings {
            for meaning in meanings {
                if let Some(pos) = &meaning.part_of_speech {
                    if !pos_list.contains(pos) {
                        pos_list.push(pos.clone());
                    }
                }
                if let Some(defs) = meaning.definitions {
                    for def in defs {
                        if let Some(d) = def.definition {
                            definitions.push(d);
                        }
                        if let Some(ex) = def.example {
                            examples.push(Example {
                                english: ex,
                                chinese: None,
                            });
                        }
                    }
                }
            }
        }

        let definition = if definitions.is_empty() {
            None
        } else {
            Some(definitions.join("\n"))
        };

        let pos = if pos_list.is_empty() {
            None
        } else {
            Some(pos_list.join(", "))
        };

        Some(DictionaryEntry {
            word: first.word,
            phonetic,
            translation: None, // 在线 API 无中文翻译
            definition,
            pos,
            exchange: None,
            tag: None,
            examples,
            source: DataSource::Online,
        })
    }
}

impl Dictionary for OnlineDictionary {
    fn lookup(&self, query: &str) -> Result<Option<DictionaryEntry>> {
        let url = format!(
            "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
            query
        );

        let resp = match self.client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return Ok(None), // 网络错误静默失败
        };

        if !resp.status().is_success() {
            return Ok(None);
        }

        let data: Vec<ApiResponse> = match resp.json() {
            Ok(d) => d,
            Err(_) => return Ok(None),
        };

        Ok(self.parse_response(data))
    }

    fn fuzzy_search(&self, _query: &str, _limit: usize) -> Result<Vec<String>> {
        // 在线 API 不支持模糊搜索
        Ok(Vec::new())
    }

    fn is_available(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "Free Dictionary API"
    }
}
