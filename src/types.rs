use std::fmt;

/// 词典查询结果条目
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub translation: Option<String>,
    pub definition: Option<String>,
    #[allow(dead_code)]
    pub pos: Option<String>,
    pub exchange: Option<String>,
    #[allow(dead_code)]
    pub tag: Option<String>,
    pub examples: Vec<Example>,
    pub source: DataSource,
}

/// 例句
#[derive(Debug, Clone)]
pub struct Example {
    pub english: String,
    pub chinese: Option<String>,
}

/// 数据来源
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSource {
    Local,
    Online,
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSource::Local => write!(f, "ECDICT (本地)"),
            DataSource::Online => write!(f, "Free Dictionary API (在线)"),
        }
    }
}

/// 查询结果
#[derive(Debug)]
pub enum LookupResult {
    Found(DictionaryEntry),
    NotFound,
    Suggestions(Vec<String>),
}

/// 查询选项
#[derive(Debug, Clone)]
pub struct LookupOptions {
    pub show_english: bool,
    pub show_examples: bool,
    pub force_online: bool,
    pub max_examples: usize,
}
