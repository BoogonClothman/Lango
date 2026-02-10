use colored::Colorize;
use std::time::Duration;

use crate::types::{DictionaryEntry, LookupResult};

/// 格式化并输出查询结果
pub fn print_result(
    result: &LookupResult,
    query: &str,
    show_english: bool,
    show_examples: bool,
    elapsed: Duration,
) {
    match result {
        LookupResult::Found(entry) => print_entry(entry, show_english, show_examples, elapsed),
        LookupResult::NotFound => print_not_found(query),
        LookupResult::Suggestions(suggestions) => print_suggestions(query, suggestions),
    }
}

fn print_entry(
    entry: &DictionaryEntry,
    show_english: bool,
    show_examples: bool,
    elapsed: Duration,
) {
    println!();

    // 单词 + 音标
    let word_display = format!("  {}", entry.word).bold().bright_blue();
    if let Some(ref phonetic) = entry.phonetic {
        let ph = if phonetic.starts_with('/') || phonetic.starts_with('[') {
            phonetic.clone()
        } else {
            format!("/{}/", phonetic)
        };
        println!("{}  {}", word_display, ph.yellow());
    } else {
        println!("{}", word_display);
    }

    println!();

    // 中文释义
    if let Some(ref translation) = entry.translation {
        println!("  {}", "中文释义".bright_white().underline());
        for line in translation.lines() {
            let line = line.trim();
            if !line.is_empty() {
                println!("    {}", line.green());
            }
        }
        println!();
    }

    // 英文释义 (需 -e 标志)
    if show_english {
        if let Some(ref definition) = entry.definition {
            println!("  {}", "英文定义".bright_white().underline());
            for (i, line) in definition.lines().enumerate() {
                let line = line.trim();
                if !line.is_empty() && i < 5 {
                    println!("    {}", line.cyan());
                }
            }
            println!();
        }
    }

    // 例句 (需 -x 标志)
    if show_examples && !entry.examples.is_empty() {
        println!("  {}", "例句".bright_white().underline());
        for (i, ex) in entry.examples.iter().enumerate() {
            println!("    {}. {}", i + 1, ex.english);
            if let Some(ref zh) = ex.chinese {
                println!("       {}", zh.dimmed());
            }
        }
        println!();
    }

    // 词形变换
    if let Some(ref exchange) = entry.exchange {
        if !exchange.is_empty() {
            let formatted = format_exchange(exchange);
            if !formatted.is_empty() {
                println!("  {}", "词形变换".bright_white().underline());
                println!("    {}", formatted.dimmed());
                println!();
            }
        }
    }

    // 来源 + 耗时
    let ms = elapsed.as_micros() as f64 / 1000.0;
    println!(
        "  {} {} {}",
        "──".dimmed(),
        entry.source.to_string().dimmed(),
        format!("· {:.1}ms", ms).dimmed()
    );
    println!();
}

fn print_not_found(query: &str) {
    println!();
    println!("  {} 未找到 \"{}\"", "✗".red(), query.yellow());
    println!();
    println!("  {}", "提示: 使用 --online 尝试在线查询".dimmed());
    println!();
}

fn print_suggestions(query: &str, suggestions: &[String]) {
    println!();
    println!("  {} 未找到 \"{}\"", "✗".red(), query.yellow());
    println!();
    println!("  {}", "你是不是要找:".bright_white());
    for s in suggestions {
        println!("    {} {}", "→".cyan(), s);
    }
    println!();
    println!("  {}", "提示: 使用 --online 尝试在线查询".dimmed());
    println!();
}

/// 解析 ECDICT 的 exchange 字段
/// 格式: "p:went/d:gone/i:going/3:goes/s:goes"
fn format_exchange(exchange: &str) -> String {
    let mut parts = Vec::new();
    for item in exchange.split('/') {
        let mut kv = item.splitn(2, ':');
        if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
            let label = match key {
                "p" => "过去式",
                "d" => "过去分词",
                "i" => "现在分词",
                "3" => "第三人称",
                "s" => "复数",
                "r" => "比较级",
                "t" => "最高级",
                "0" => "原型",
                "1" => "原型变换",
                _ => continue,
            };
            parts.push(format!("{}: {}", label, value));
        }
    }
    parts.join("  ")
}
