use crate::error::HamsterError;
use crate::scan::{DriverInfo};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use futures_util::StreamExt;

/// 检查驱动更新
pub fn check_updates() -> Result<Vec<DriverInfo>, HamsterError> {
    let outdated_drivers = scan_outdated_drivers()?;
    Ok(outdated_drivers)
}

/// 安装驱动更新
pub fn install_updates(auto: bool) -> Result<(), HamsterError> {
    if auto {
        println!("自动安装驱动更新");
    } else {
        println!("手动安装驱动更新");
    }
    
    Ok(())
}

/// 下载进度回调类型
pub type DownloadProgressCallback = Arc<Mutex<Box<dyn Fn(usize, usize) + Send + Sync>>>;

/// 安装进度回调类型
pub type InstallProgressCallback = Arc<Mutex<Box<dyn Fn(String, usize, usize) + Send + Sync>>>;

/// 下载结果
#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub driver_name: String,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

/// 安装结果
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub driver_name: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub installed_version: Option<String>,
}

/// 下载并安装单个驱动（完整实现）
pub async fn download_and_install_driver(
    driver: &DriverInfo,
    download_callback: Option<DownloadProgressCallback>,
    install_callback: Option<InstallProgressCallback>
) -> Result<InstallResult, HamsterError> {
    if driver.download_url.is_empty() {
        return Ok(InstallResult {
            driver_name: driver.name.clone(),
            success: false,
            error_message: Some("驱动没有下载链接".to_string()),
            installed_version: None,
        });
    }
    
    println!("开始下载驱动: {} (版本: {})", driver.name, driver.latest_version);
    
    let download_result = download_driver_with_progress(
        &driver.download_url,
        &driver.name,
        &driver.latest_version,
        download_callback
    ).await?;
    
    if !download_result.success {
        return Ok(InstallResult {
            driver_name: driver.name.clone(),
            success: false,
            error_message: download_result.error_message,
            installed_version: None,
        });
    }
    
    println!("驱动已下载到: {:?}", download_result.file_path);
    
    let install_result = install_driver_from_file(
        &download_result.file_path,
        &driver.name,
        install_callback
    ).await?;
    
    Ok(install_result)
}

/// 下载驱动（带进度跟踪）
pub async fn download_driver_with_progress(
    url: &str,
    driver_name: &str,
    version: &str,
    callback: Option<DownloadProgressCallback>
) -> Result<DownloadResult, HamsterError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|e| HamsterError::NetworkError(format!("创建HTTP客户端失败: {}", e)))?;
    
    let response = client.get(url).send().await
        .map_err(|e| HamsterError::NetworkError(format!("下载失败: {}", e)))?;
    
    if !response.status().is_success() {
        return Ok(DownloadResult {
            driver_name: driver_name.to_string(),
            file_path: PathBuf::new(),
            file_size: 0,
            success: false,
            error_message: Some(format!("下载失败: HTTP {}", response.status())),
        });
    }
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded = 0usize;
    let mut file_content = Vec::new();
    
    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result
            .map_err(|e| HamsterError::NetworkError(format!("读取下载内容失败: {}", e)))?;
        
        downloaded += chunk.len();
        file_content.extend_from_slice(&chunk);
        
        if let Some(ref cb) = callback {
            let cb = cb.lock().await;
            cb(downloaded, total_size as usize);
        }
    }
    
    let temp_dir = std::env::temp_dir().join("hamsterdrive_drivers");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| HamsterError::UpdateError(format!("创建临时目录失败: {}", e)))?;
    
    let file_name = format!("{}_{}.exe", 
        driver_name.replace(" ", "_").replace("/", "_").replace("\\", "_"), 
        version.replace(".", "_")
    );
    let temp_file_path = temp_dir.join(&file_name);
    
    let file_len = file_content.len();
    fs::write(&temp_file_path, file_content)
        .map_err(|e| HamsterError::UpdateError(format!("保存文件失败: {}", e)))?;
    
    println!("驱动下载完成: {} (大小: {} bytes)", file_name, file_len);
    
    Ok(DownloadResult {
        driver_name: driver_name.to_string(),
        file_path: temp_file_path,
        file_size: file_len as u64,
        success: true,
        error_message: None,
    })
}

/// 从文件安装驱动
pub async fn install_driver_from_file(
    file_path: &Path,
    driver_name: &str,
    callback: Option<InstallProgressCallback>
) -> Result<InstallResult, HamsterError> {
    if !file_path.exists() {
        return Ok(InstallResult {
            driver_name: driver_name.to_string(),
            success: false,
            error_message: Some(format!("驱动文件不存在: {:?}", file_path)),
            installed_version: None,
        });
    }
    
    println!("开始安装驱动: {}", driver_name);
    
    if let Some(ref cb) = callback {
        let cb = cb.lock().await;
        cb(format!("正在准备安装驱动: {}", driver_name), 1, 4);
    }
    
    let file_ext = file_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    match file_ext.to_lowercase().as_str() {
        "exe" => {
            install_exe_driver(file_path, driver_name, callback).await
        },
        "zip" | "7z" | "rar" => {
            install_archive_driver(file_path, driver_name, callback).await
        },
        "inf" => {
            install_inf_driver(file_path, driver_name, callback).await
        },
        _ => {
            Ok(InstallResult {
                driver_name: driver_name.to_string(),
                success: false,
                error_message: Some(format!("不支持的驱动文件格式: {}", file_ext)),
                installed_version: None,
            })
        }
    }
}

/// 安装EXE格式的驱动
async fn install_exe_driver(
    file_path: &Path,
    driver_name: &str,
    callback: Option<InstallProgressCallback>
) -> Result<InstallResult, HamsterError> {
    if let Some(ref cb) = callback {
        let cb = cb.lock().await;
        cb(format!("正在静默安装驱动: {}", driver_name), 2, 4);
    }
    
    let output = tokio::process::Command::new(file_path)
        .args(&["/S", "/VERYSILENT", "/NORESTART"])
        .output()
        .await;
    
    match output {
        Ok(result) => {
            if result.status.success() {
                if let Some(ref cb) = callback {
                    let cb = cb.lock().await;
                    cb(format!("驱动安装成功: {}", driver_name), 3, 4);
                }
                
                println!("驱动安装成功: {}", driver_name);
                
                Ok(InstallResult {
                    driver_name: driver_name.to_string(),
                    success: true,
                    error_message: None,
                    installed_version: Some("已安装".to_string()),
                })
            } else {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                Ok(InstallResult {
                    driver_name: driver_name.to_string(),
                    success: false,
                    error_message: Some(format!("安装失败: {}", error_msg)),
                    installed_version: None,
                })
            }
        },
        Err(e) => Ok(InstallResult {
            driver_name: driver_name.to_string(),
            success: false,
            error_message: Some(format!("执行安装命令失败: {}", e)),
            installed_version: None,
        })
    }
}

/// 安装压缩包格式的驱动
async fn install_archive_driver(
    file_path: &Path,
    driver_name: &str,
    callback: Option<InstallProgressCallback>
) -> Result<InstallResult, HamsterError> {
    if let Some(ref cb) = callback {
        let cb = cb.lock().await;
        cb(format!("正在解压驱动包: {}", driver_name), 2, 4);
    }
    
    let temp_dir = std::env::temp_dir().join("hamsterdrive_extracted");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| HamsterError::UpdateError(format!("创建解压目录失败: {}", e)))?;
    
    let extract_dir = temp_dir.join(driver_name.replace(" ", "_"));
    
    let output = tokio::process::Command::new("7z")
        .args(&["x", "-y", &file_path.to_string_lossy(), &format!("-o{}", extract_dir.to_string_lossy())])
        .output()
        .await;
    
    match output {
        Ok(result) => {
            if result.status.success() {
                if let Some(ref cb) = callback {
                    let cb = cb.lock().await;
                    cb(format!("正在查找INF文件: {}", driver_name), 3, 4);
                }
                
                let inf_file = find_inf_file(&extract_dir).await;
                
                match inf_file {
                    Some(inf_path) => {
                        install_inf_driver(&inf_path, driver_name, callback).await
                    },
                    None => Ok(InstallResult {
                        driver_name: driver_name.to_string(),
                        success: false,
                        error_message: Some("未找到INF文件".to_string()),
                        installed_version: None,
                    })
                }
            } else {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                Ok(InstallResult {
                    driver_name: driver_name.to_string(),
                    success: false,
                    error_message: Some(format!("解压失败: {}", error_msg)),
                    installed_version: None,
                })
            }
        },
        Err(e) => Ok(InstallResult {
            driver_name: driver_name.to_string(),
            success: false,
            error_message: Some(format!("执行解压命令失败: {}", e)),
            installed_version: None,
        })
    }
}

/// 安装INF格式的驱动
async fn install_inf_driver(
    file_path: &Path,
    driver_name: &str,
    callback: Option<InstallProgressCallback>
) -> Result<InstallResult, HamsterError> {
    if let Some(ref cb) = callback {
        let cb = cb.lock().await;
        cb(format!("正在通过pnputil安装驱动: {}", driver_name), 3, 4);
    }
    
    #[cfg(windows)]
    {
        let output = tokio::process::Command::new("pnputil")
            .args(&["/add-driver", &file_path.to_string_lossy(), "/install"])
            .output()
            .await;
        
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                
                if result.status.success() || stdout.contains("Driver package added successfully") || stdout.contains("驱动包已成功添加") {
                    if let Some(ref cb) = callback {
                        let cb = cb.lock().await;
                        cb(format!("驱动安装完成: {}", driver_name), 4, 4);
                    }
                    
                    println!("驱动安装成功: {}", driver_name);
                    
                    Ok(InstallResult {
                        driver_name: driver_name.to_string(),
                        success: true,
                        error_message: None,
                        installed_version: Some("已安装".to_string()),
                    })
                } else {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    Ok(InstallResult {
                        driver_name: driver_name.to_string(),
                        success: false,
                        error_message: Some(format!("安装失败: {}", error_msg)),
                        installed_version: None,
                    })
                }
            },
            Err(e) => Ok(InstallResult {
                driver_name: driver_name.to_string(),
                success: false,
                error_message: Some(format!("执行pnputil命令失败: {}", e)),
                installed_version: None,
            })
        }
    }
    
    #[cfg(not(windows))]
    {
        Ok(InstallResult {
            driver_name: driver_name.to_string(),
            success: false,
            error_message: Some("驱动安装仅支持Windows系统".to_string()),
            installed_version: None,
        })
    }
}

/// 查找INF文件
fn find_inf_file(dir: &Path) -> impl std::future::Future<Output = Option<PathBuf>> + '_ {
    async move {
        let entries = fs::read_dir(dir).ok()?;
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "inf" {
                        return Some(path);
                    }
                }
            } else if path.is_dir() {
                if let Some(found) = Box::pin(find_inf_file(&path)).await {
                    return Some(found);
                }
            }
        }
        
        None
    }
}

/// 批量更新驱动（带进度跟踪）
pub async fn batch_update_drivers(
    drivers: &[DriverInfo],
    download_callback: Option<DownloadProgressCallback>,
    install_callback: Option<InstallProgressCallback>
) -> Result<Vec<InstallResult>, HamsterError> {
    let mut results = Vec::new();
    
    for driver in drivers {
        let result = download_and_install_driver(driver, download_callback.clone(), install_callback.clone()).await?;
        results.push(result);
    }
    
    Ok(results)
}

/// 批量更新驱动（并行）
pub async fn batch_update_drivers_parallel(
    drivers: &[DriverInfo],
    max_concurrent: usize,
    download_callback: Option<DownloadProgressCallback>,
    install_callback: Option<InstallProgressCallback>
) -> Result<Vec<InstallResult>, HamsterError> {
    use futures::stream::{self, StreamExt};
    
    let results = Arc::new(Mutex::new(Vec::new()));
    
    stream::iter(drivers)
        .map(|driver| {
            let results = results.clone();
            let download_cb = download_callback.clone();
            let install_cb = install_callback.clone();
            
            async move {
                let result = download_and_install_driver(driver, download_cb, install_cb).await;
                let mut results = results.lock().await;
                results.push(result);
            }
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;
    
    let final_results = results.lock().await;
    let mut output_results = Vec::new();
    for result in final_results.iter() {
        match result {
            Ok(install_result) => output_results.push(install_result.clone()),
            Err(e) => {
                output_results.push(InstallResult {
                    driver_name: "未知驱动".to_string(),
                    success: false,
                    error_message: Some(e.to_string()),
                    installed_version: None,
                });
            }
        }
    }
    Ok(output_results)
}

/// 验证驱动安装结果
pub fn verify_driver_installation(driver_name: &str) -> Result<bool, HamsterError> {
    #[cfg(windows)]
    {
        let output = std::process::Command::new("pnputil")
            .args(&["/enum-drivers"])
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    Ok(stdout.to_lowercase().contains(&driver_name.to_lowercase()))
                } else {
                    Ok(false)
                }
            },
            Err(_) => Ok(false)
        }
    }
    
    #[cfg(not(windows))]
    {
        Ok(false)
    }
}

/// 获取驱动下载进度
pub fn get_download_progress(downloaded: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (downloaded as f64 / total as f64) * 100.0
    }
}

/// 格式化文件大小
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// 自动安装驱动更新
fn auto_install_driver_updates() -> Result<(), HamsterError> {
    let updates = check_updates()?;
    
    for update in updates {
        println!("正在安装更新: {}", update.name);
    }
    
    Ok(())
}

/// 手动安装驱动更新
fn manual_install_driver_updates() -> Result<(), HamsterError> {
    let updates = check_updates()?;
    
    for update in updates {
        println!("发现更新: {}，是否安装？(y/n)", update.name);
    }
    
    Ok(())
}

/// 扫描过时的驱动程序
pub fn scan_outdated_drivers() -> Result<Vec<DriverInfo>, HamsterError> {
    Ok(Vec::new())
}
