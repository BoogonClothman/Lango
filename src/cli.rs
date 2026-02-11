use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "lango",
    about = "快速英语词典查询工具",
    version,
    after_help = "示例:\n  lango hello              查询单词\n  lango \"machine learning\"  查询词组\n  lango -e hello            显示英文释义\n  lango -x hello            显示例句\n  lango --online hello      强制在线查询(默认显示英文释义)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// 要查询的单词或词组
    #[arg(value_name = "WORD", trailing_var_arg = true)]
    pub query: Vec<String>,

    /// 显示英文释义
    #[arg(short = 'e', long = "english", global = true)]
    pub show_english: bool,

    /// 显示例句
    #[arg(short = 'x', long = "examples", global = true)]
    pub show_examples: bool,

    /// 强制使用在线词典
    #[arg(long = "online", global = true)]
    pub force_online: bool,

    /// 例句数量上限
    #[arg(short = 'n', long = "num-examples", default_value = "3", global = true)]
    pub max_examples: usize,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 初始化/重新下载词库
    Setup {
        /// 从本地文件导入词库
        #[arg(long = "import")]
        import_path: Option<PathBuf>,
    },
}
