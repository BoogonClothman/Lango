mod cli;
mod dict;
mod formatter;
mod setup;
mod types;

use anyhow::Result;
use clap::Parser;
use std::time::Instant;

use cli::{Cli, Commands};
use dict::DictionaryService;
use dict::ecdict::EcdictDictionary;
use dict::online::OnlineDictionary;
use types::LookupOptions;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 处理子命令
    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Setup { import_path } => {
                if let Some(path) = import_path {
                    setup::import_db(path)?;
                } else {
                    setup::interactive_setup()?;
                }
                return Ok(());
            }
        }
    }

    // 拼接查询词
    let query = cli.query.join(" ");
    if query.is_empty() {
        use clap::CommandFactory;
        Cli::command().print_help()?;
        println!();
        return Ok(());
    }

    // 检查词库是否存在
    if !setup::is_db_installed() && !cli.force_online {
        setup::interactive_setup()?;
    }

    // 初始化词典服务
    let local = if let Ok(path) = setup::db_path() {
        if path.exists() {
            EcdictDictionary::open(&path).ok()
        } else {
            None
        }
    } else {
        None
    };

    let online = Some(OnlineDictionary::new());
    let service = DictionaryService::new(local, online);

    // 在线模式默认显示英文定义（因为在线API无中文翻译）
    let show_english = cli.show_english || cli.force_online;

    let options = LookupOptions {
        show_english,
        show_examples: cli.show_examples,
        force_online: cli.force_online,
        max_examples: cli.max_examples,
    };

    // 执行查询并计时
    let start = Instant::now();
    let result = service.lookup(&query, &options)?;
    let elapsed = start.elapsed();

    // 格式化输出
    formatter::print_result(
        &result,
        &query,
        options.show_english,
        options.show_examples,
        elapsed,
    );

    Ok(())
}
