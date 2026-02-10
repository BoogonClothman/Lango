use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// 获取词库存储目录
pub fn data_dir() -> Result<PathBuf> {
    let base = dirs::data_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local").join("share")))
        .context("无法获取用户数据目录")?;
    Ok(base.join("lango"))
}

/// 获取词库文件路径
pub fn db_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("stardict.db"))
}

/// 检查词库是否存在
pub fn is_db_installed() -> bool {
    db_path().map(|p| p.exists()).unwrap_or(false)
}

/// 交互式引导下载词库
pub fn interactive_setup() -> Result<PathBuf> {
    let path = db_path()?;

    println!();
    println!("  Lango 需要 ECDICT 离线词库 (770,000+ 词条)");
    println!("  首次使用需要下载约 180MB 数据");
    println!();
    print!("  是否现在下载? [Y/n]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "n" || input == "no" {
        println!();
        println!("  你可以稍后运行 `lango setup` 下载词库");
        println!("  或手动下载后执行 `lango setup --import <路径>`");
        anyhow::bail!("用户取消下载");
    }

    download_db(&path)?;
    Ok(path)
}

/// 从本地文件导入词库
pub fn import_db(source: &PathBuf) -> Result<PathBuf> {
    let dest = db_path()?;

    if !source.exists() {
        anyhow::bail!("文件不存在: {}", source.display());
    }

    // 创建目标目录
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    // 判断是 zip 还是 db 文件
    let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext == "zip" {
        extract_zip(source, &dest)?;
    } else {
        fs::copy(source, &dest)
            .with_context(|| format!("复制文件失败: {} -> {}", source.display(), dest.display()))?;
    }

    // 验证 SQLite 文件
    validate_db(&dest)?;

    println!("  词库已导入: {}", dest.display());
    Ok(dest)
}

/// 下载词库
fn download_db(dest: &PathBuf) -> Result<()> {
    // 创建目标目录
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let url = "https://github.com/skywind3000/ECDICT/releases/download/1.0.28/ecdict-sqlite-28.zip";

    println!();
    println!("  正在下载 ECDICT 词库...");
    println!("  来源: {}", url);
    println!();

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .user_agent("lango-cli/0.1")
        .build()?;

    let resp = client.get(url).send().context("下载失败，请检查网络连接")?;

    if !resp.status().is_success() {
        anyhow::bail!("下载失败: HTTP {}", resp.status());
    }

    let total_size = resp.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    // 下载到临时 zip 文件
    let zip_path = dest.with_extension("zip.tmp");
    let mut file = fs::File::create(&zip_path)?;
    let mut downloaded: u64 = 0;
    let mut reader = resp;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("下载完成");
    drop(file);

    println!("  正在解压...");
    extract_zip(&zip_path, dest)?;

    // 删除临时 zip
    let _ = fs::remove_file(&zip_path);

    // 验证
    validate_db(dest)?;

    println!("  词库安装成功: {}", dest.display());
    println!();

    Ok(())
}

/// 从 zip 文件中提取 stardict.db
fn extract_zip(zip_path: &PathBuf, dest: &PathBuf) -> Result<()> {
    let file = fs::File::open(zip_path)
        .with_context(|| format!("无法打开文件: {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if name.ends_with(".db") {
            let mut out = fs::File::create(dest)?;
            io::copy(&mut entry, &mut out)?;
            return Ok(());
        }
    }

    anyhow::bail!("zip 文件中未找到 .db 文件");
}

/// 验证 SQLite 数据库文件
fn validate_db(path: &PathBuf) -> Result<()> {
    let conn = rusqlite::Connection::open(path)?;
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM stardict LIMIT 1", [], |row| {
            row.get(0)
        })
        .context("词库文件损坏或格式不正确")?;

    if count == 0 {
        anyhow::bail!("词库为空");
    }

    Ok(())
}
