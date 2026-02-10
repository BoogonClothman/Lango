use crate::types::{DictionaryEntry, LookupOptions, LookupResult};
use anyhow::Result;

pub mod ecdict;
pub mod online;

/// 词典后端 trait
pub trait Dictionary {
    fn lookup(&self, query: &str) -> Result<Option<DictionaryEntry>>;
    fn fuzzy_search(&self, query: &str, limit: usize) -> Result<Vec<String>>;
    fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}

/// 词典查询服务：编排本地 + 在线查询
pub struct DictionaryService {
    local: Option<ecdict::EcdictDictionary>,
    online: Option<online::OnlineDictionary>,
}

impl DictionaryService {
    pub fn new(
        local: Option<ecdict::EcdictDictionary>,
        online: Option<online::OnlineDictionary>,
    ) -> Self {
        Self { local, online }
    }

    pub fn lookup(&self, query: &str, options: &LookupOptions) -> Result<LookupResult> {
        let query = query.trim().to_lowercase();

        // --online 模式：直接走在线
        if options.force_online {
            if let Some(ref online) = self.online {
                if let Some(mut entry) = online.lookup(&query)? {
                    entry.examples.truncate(options.max_examples);
                    return Ok(LookupResult::Found(entry));
                }
            }
            return Ok(LookupResult::NotFound);
        }

        // 默认模式：先查本地
        if let Some(ref local) = self.local {
            if let Some(mut entry) = local.lookup(&query)? {
                // 如果需要例句/英文释义但本地没有，尝试在线补充
                let needs_online = (options.show_examples && entry.examples.is_empty())
                    || (options.show_english && entry.definition.is_none());

                if needs_online {
                    if let Some(ref online) = self.online {
                        if let Some(online_entry) = online.lookup(&query)? {
                            if entry.definition.is_none() {
                                entry.definition = online_entry.definition;
                            }
                            if entry.examples.is_empty() {
                                entry.examples = online_entry.examples;
                            }
                        }
                    }
                }

                entry.examples.truncate(options.max_examples);
                return Ok(LookupResult::Found(entry));
            }

            // 本地未找到 → 模糊匹配
            let suggestions = local.fuzzy_search(&query, 5)?;
            if !suggestions.is_empty() {
                return Ok(LookupResult::Suggestions(suggestions));
            }
        }

        // 本地没结果，尝试在线兜底
        if let Some(ref online) = self.online {
            if let Some(mut entry) = online.lookup(&query)? {
                entry.examples.truncate(options.max_examples);
                return Ok(LookupResult::Found(entry));
            }
        }

        Ok(LookupResult::NotFound)
    }
}
