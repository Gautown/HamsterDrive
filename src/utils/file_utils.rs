//! 文件操作工具模块

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use crate::utils::error::{HamsterError, Result};
use walkdir::WalkDir;

/// 确保目录存在，如不存在则创建
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 获取应用程序数据目录
pub fn get_app_data_dir() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        if let Some(app_data) = std::env::var_os("LOCALAPPDATA") {
            let path = PathBuf::from(app_data).join("HamsterDrive");
            ensure_dir(&path)?;
            return Ok(path);
        }
    }
    
    // 备用路径：当前目录下的data文件夹
    let path = std::env::current_dir()?.join("data");
    ensure_dir(&path)?;
    Ok(path)
}

/// 获取临时目录
pub fn get_temp_dir() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("HamsterDrive");
    ensure_dir(&temp_dir)?;
    Ok(temp_dir)
}

/// 获取驱动备份目录
pub fn get_backup_dir() -> Result<PathBuf> {
    let backup_dir = get_app_data_dir()?.join("backups");
    ensure_dir(&backup_dir)?;
    Ok(backup_dir)
}

/// 获取驱动下载目录
pub fn get_download_dir() -> Result<PathBuf> {
    let download_dir = get_app_data_dir()?.join("downloads");
    ensure_dir(&download_dir)?;
    Ok(download_dir)
}

/// 获取日志目录
pub fn get_log_dir() -> Result<PathBuf> {
    let log_dir = get_app_data_dir()?.join("logs");
    ensure_dir(&log_dir)?;
    Ok(log_dir)
}

/// 获取数据库目录
pub fn get_database_dir() -> Result<PathBuf> {
    let db_dir = get_app_data_dir()?.join("database");
    ensure_dir(&db_dir)?;
    Ok(db_dir)
}

/// 复制文件
pub fn copy_file(src: &Path, dst: &Path) -> Result<u64> {
    // 确保目标目录存在
    if let Some(parent) = dst.parent() {
        ensure_dir(parent)?;
    }
    
    let bytes_copied = fs::copy(src, dst)?;
    Ok(bytes_copied)
}

/// 移动文件
pub fn move_file(src: &Path, dst: &Path) -> Result<()> {
    // 确保目标目录存在
    if let Some(parent) = dst.parent() {
        ensure_dir(parent)?;
    }
    
    // 尝试重命名（同一文件系统上更快）
    if fs::rename(src, dst).is_ok() {
        return Ok(());
    }
    
    // 如果重命名失败，使用复制+删除
    fs::copy(src, dst)?;
    fs::remove_file(src)?;
    Ok(())
}

/// 递归复制目录
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    ensure_dir(dst)?;
    
    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry.map_err(|e| HamsterError::IoError(e.to_string()))?;
        let relative_path = entry.path().strip_prefix(src)
            .map_err(|e| HamsterError::IoError(e.to_string()))?;
        let target_path = dst.join(relative_path);
        
        if entry.file_type().is_dir() {
            ensure_dir(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                ensure_dir(parent)?;
            }
            fs::copy(entry.path(), &target_path)?;
        }
    }
    
    Ok(())
}

/// 删除目录及其内容
pub fn remove_dir_recursive(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// 获取目录大小
pub fn get_dir_size(path: &Path) -> Result<u64> {
    let mut total_size = 0u64;
    
    for entry in WalkDir::new(path) {
        let entry = entry.map_err(|e| HamsterError::IoError(e.to_string()))?;
        if entry.file_type().is_file() {
            total_size += entry.metadata()
                .map(|m| m.len())
                .unwrap_or(0);
        }
    }
    
    Ok(total_size)
}

/// 读取文件内容为字符串
pub fn read_file_string(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// 读取文件内容为字节
pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

/// 写入字符串到文件
pub fn write_file_string(path: &Path, content: &str) -> Result<()> {
    // 确保父目录存在
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 写入字节到文件
pub fn write_file_bytes(path: &Path, content: &[u8]) -> Result<()> {
    // 确保父目录存在
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}

/// 查找目录中的文件
pub fn find_files(dir: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let pattern_lower = pattern.to_lowercase();
    
    for entry in WalkDir::new(dir) {
        let entry = entry.map_err(|e| HamsterError::IoError(e.to_string()))?;
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            if file_name.contains(&pattern_lower) {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    
    Ok(files)
}

/// 查找特定扩展名的文件
pub fn find_files_by_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let ext_lower = extension.to_lowercase();
    
    for entry in WalkDir::new(dir) {
        let entry = entry.map_err(|e| HamsterError::IoError(e.to_string()))?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == ext_lower {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    
    Ok(files)
}

/// 安全删除文件（移动到临时目录而不是直接删除）
pub fn safe_delete_file(path: &Path) -> Result<()> {
    if path.exists() {
        let temp_dir = get_temp_dir()?.join("deleted");
        ensure_dir(&temp_dir)?;
        
        let file_name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let target_path = temp_dir.join(format!("{}_{}", timestamp, file_name));
        move_file(path, &target_path)?;
    }
    Ok(())
}
