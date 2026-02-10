# Lango

一个快速的命令行英语词典工具，基于本地 ECDICT 数据库，支持离线查询和在线补充。

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## 特性

- **极速查询** - 本地 SQLite 数据库，毫秒级响应
- **离线优先** - 77万+ 词条本地存储，无需联网即可使用
- **智能补全** - 本地无结果时自动尝试在线 API
- **模糊搜索** - 拼写错误时提供相似单词建议
- **双语支持** - 中文释义 + 英文定义（可选）
- **例句展示** - 支持显示真实例句
- **词形变换** - 自动显示动词时态、名词复数等

## 安装

### 从源码编译

```bash
git clone https://github.com/BoogonClothman/Lango.git
cd Lango
cargo build --release
```

编译完成后，可执行文件位于 `target/release/lango`。

### 首次使用

首次运行时需要下载词库（约 180MB）：

```bash
lango hello
```

或手动执行初始化：

```bash
lango setup
```

## 使用方法

### 基本查询

```bash
# 查询单词
lango hello

# 查询词组
lango "machine learning"

# 显示英文释义
lango -e hello

# 显示例句
lango -x hello

# 指定例句数量
lango -x -n 5 hello
```

### 在线模式

```bash
# 强制使用在线词典（无需本地词库）
lango --online hello
```

### 词库管理

```bash
# 重新下载/更新词库
lango setup

# 从本地文件导入词库
lango setup --import /path/to/stardict.db
```

## 词库文件位置

- **Windows**: `%APPDATA%\lango\stardict.db`
- **macOS**: `~/Library/Application Support/lango/stardict.db`
- **Linux**: `~/.local/share/lango/stardict.db`

## 技术栈

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [ECDICT](https://github.com/skywind3000/ECDICT) - 离线词库数据
- [Free Dictionary API](https://dictionaryapi.dev/) - 在线词典 API
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite 绑定

## 致谢

本项目基于以下优秀开源项目构建：

- **[ECDICT](https://github.com/skywind3000/ECDICT)** by [skywind3000](https://github.com/skywind3000) - 提供 77万+ 词条的离线英汉词典数据（MIT License）
- **[Free Dictionary API](https://dictionaryapi.dev/)** - 提供在线词典查询服务
- **Rust 社区** - 提供了丰富的高质量 crates

## 许可证

本项目采用 [MIT License](LICENSE) 开源许可证。

### 第三方依赖许可证

本项目使用了以下开源库（均为宽松许可证，与 MIT 兼容）：

| 依赖 | 许可证 | 用途 |
|------|--------|------|
| [anyhow](https://github.com/dtolnay/anyhow) | MIT OR Apache-2.0 | 错误处理 |
| [clap](https://github.com/clap-rs/clap) | MIT OR Apache-2.0 | 命令行解析 |
| [colored](https://github.com/colored-rs/colored) | MPL-2.0 | 终端彩色输出 |
| [dirs](https://github.com/dirs-dev/dirs-rs) | MIT OR Apache-2.0 | 系统目录获取 |
| [indicatif](https://github.com/console-rs/indicatif) | MIT | 进度条显示 |
| [reqwest](https://github.com/seanmonstar/reqwest) | MIT OR Apache-2.0 | HTTP 客户端 |
| [rusqlite](https://github.com/rusqlite/rusqlite) | MIT | SQLite 绑定 |
| [serde](https://github.com/serde-rs/serde) | MIT OR Apache-2.0 | 序列化 |
| [strsim](https://github.com/dguo/strsim-rs) | MIT | 字符串相似度 |
| [zip](https://github.com/zip-rs/zip2) | MIT | ZIP 解压 |

## 项目结构

```
src/
├── main.rs       # 程序入口
├── cli.rs        # 命令行参数定义
├── types.rs      # 数据类型定义
├── setup.rs      # 词库下载与初始化
├── formatter.rs  # 结果格式化输出
└── dict/
    ├── mod.rs      # 词典服务编排
    ├── ecdict.rs   # ECDICT 本地词典实现
    └── online.rs   # Free Dictionary API 在线词典实现
```

## AI 使用说明

本项目绝大部分代码由Qoder参与编写，不可避免地存在优化问题和预期不符等情况，如果您在使用时出现问题，或者您对本工具有更好的建议，欢迎您提交issue，也欢迎您自行分支克隆仓库进行再开发。