use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

use super::Dictionary;
use crate::types::{DataSource, DictionaryEntry};

/// ECDICT 本地 SQLite 词典
pub struct EcdictDictionary {
    conn: Connection,
}

impl EcdictDictionary {
    pub fn open(db_path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("无法打开词库: {}", db_path.display()))?;

        // 性能优化
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA cache_size=8000;")?;

        Ok(Self { conn })
    }
}

impl Dictionary for EcdictDictionary {
    fn lookup(&self, query: &str) -> Result<Option<DictionaryEntry>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT word, phonetic, definition, translation, pos, exchange, tag \
             FROM stardict WHERE word = ?1 COLLATE NOCASE LIMIT 1",
        )?;

        let result = stmt.query_row([query], |row| {
            Ok(DictionaryEntry {
                word: row.get(0)?,
                phonetic: row.get(1)?,
                definition: row.get(2)?,
                translation: row.get(3)?,
                pos: row.get(4)?,
                exchange: row.get(5)?,
                tag: row.get(6)?,
                examples: Vec::new(),
                source: DataSource::Local,
            })
        });

        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn fuzzy_search(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        // 先尝试前缀匹配（利用索引，快速）
        let mut stmt = self.conn.prepare_cached(
            "SELECT word FROM stardict WHERE word LIKE ?1 COLLATE NOCASE ORDER BY length(word) LIMIT ?2"
        )?;
        let prefix_pattern = format!("{}%", query);
        let mut results: Vec<String> = stmt
            .query_map(rusqlite::params![prefix_pattern, limit * 2], |row| {
                row.get(0)
            })?
            .filter_map(|r| r.ok())
            .collect();

        // 如果前缀匹配不够，用 LIKE 模糊搜索补充
        if results.len() < limit {
            let mut stmt2 = self.conn.prepare_cached(
                "SELECT word FROM stardict WHERE word LIKE ?1 COLLATE NOCASE ORDER BY length(word) LIMIT ?2"
            )?;
            let fuzzy_pattern = format!("%{}%", query);
            let extra: Vec<String> = stmt2
                .query_map(rusqlite::params![fuzzy_pattern, limit * 3], |row| {
                    row.get(0)
                })?
                .filter_map(|r| r.ok())
                .filter(|w: &String| !results.contains(w))
                .collect();
            results.extend(extra);
        }

        // 用编辑距离排序，取最接近的
        use strsim::levenshtein;
        results.sort_by_key(|w| levenshtein(&w.to_lowercase(), &query.to_lowercase()));
        results.truncate(limit);

        Ok(results)
    }

    fn is_available(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "ECDICT"
    }
}
