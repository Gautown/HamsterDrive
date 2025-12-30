use crate::error::HamsterError;
use std::fs;
use std::path::Path;
use std::process::Command;
use chrono::DateTime;
use chrono::Local;

/// 验证驱动程序签名
pub fn verify_driver_signature(driver_path: &str) -> Result<bool, HamsterError> {
    // 检查文件是否存在
    let path = Path::new(driver_path);
    if !path.exists() {
        return Err(HamsterError::SignatureError(format!("驱动文件不存在: {}", driver_path)))
    }
    
    // 使用signtool验证驱动签名
    #[cfg(windows)]
    {
        let output = Command::new("signtool")
            .args(&["verify", "/v", "/pa", driver_path])
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    
                    // 检查签名验证结果
                    if stdout.contains("Successfully verified") {
                        println!("驱动签名验证成功: {}", driver_path);
                        Ok(true)
                    } else if stdout.contains("SignTool Error") {
                        let error_msg = stdout.lines()
                            .find(|line| line.contains("Error"))
                            .unwrap_or("未知错误");
                        Err(HamsterError::SignatureError(format!("签名验证失败: {}", error_msg)))
                    } else {
                        Err(HamsterError::SignatureError(format!("签名验证失败: 未知错误")))
                    }
                } else {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    Err(HamsterError::SignatureError(format!("执行签名验证失败: {}", error_msg)))
                }
            },
            Err(e) => Err(HamsterError::SignatureError(format!("执行签名验证命令失败: {}", e)))
        }
    }
    
    #[cfg(not(windows))]
    {
        Err(HamsterError::SignatureError("驱动签名验证仅支持Windows系统".to_string()))
    }
}

/// 批量验证多个驱动签名
pub fn batch_verify_driver_signatures(driver_paths: &[String]) -> Result<Vec<SignatureResult>, HamsterError> {
    let mut results = Vec::new();
    
    for driver_path in driver_paths {
        match verify_driver_signature(driver_path) {
            Ok(is_valid) => {
                results.push(SignatureResult {
                    driver_path: driver_path.clone(),
                    is_valid,
                    message: "签名有效".to_string(),
                });
            },
            Err(e) => {
                results.push(SignatureResult {
                    driver_path: driver_path.clone(),
                    is_valid: false,
                    message: format!("签名验证失败: {}", e),
                });
            }
        }
    }
    
    Ok(results)
}

/// 获取驱动文件的详细信息
pub fn get_driver_file_info(driver_path: &str) -> Result<DriverFileInfo, HamsterError> {
    let path = Path::new(driver_path);
    
    if !path.exists() {
        return Err(HamsterError::SignatureError(format!("驱动文件不存在: {}", driver_path)))
    }
    
    let metadata = fs::metadata(path)
        .map_err(|e| HamsterError::SignatureError(format!("获取文件信息失败: {}", e)))?;
    
    let file_size = metadata.len();
    let modified_time = metadata.modified()
        .map_err(|e| HamsterError::SignatureError(format!("获取修改时间失败: {}", e)))?;
    
    let datetime: DateTime<Local> = modified_time.into();
    
    Ok(DriverFileInfo {
        path: driver_path.to_string(),
        size: file_size,
        modified_time: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

/// 验证驱动包签名
pub fn verify_driver_package_signature(package_name: &str) -> Result<bool, HamsterError> {
    #[cfg(windows)]
    {
        let output = Command::new("pnputil")
            .args(&["/verify-driver", package_name])
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    
                    // 检查签名验证结果
                    if stdout.contains("Driver package is signed") || stdout.contains("Signature verification passed") {
                        println!("驱动包签名验证成功: {}", package_name);
                        Ok(true)
                    } else {
                        Err(HamsterError::SignatureError(format!("驱动包签名验证失败: {}", package_name)))
                    }
                } else {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    Err(HamsterError::SignatureError(format!("签名验证失败: {}", error_msg)))
                }
            },
            Err(e) => Err(HamsterError::SignatureError(format!("执行签名验证命令失败: {}", e)))
        }
    }
    
    #[cfg(not(windows))]
    {
        Err(HamsterError::SignatureError("驱动包签名验证仅支持Windows系统".to_string()))
    }
}

/// 签名验证结果
#[derive(Debug, Clone)]
pub struct SignatureResult {
    pub driver_path: String,
    pub is_valid: bool,
    pub message: String,
}

/// 驱动文件信息
#[derive(Debug, Clone)]
pub struct DriverFileInfo {
    pub path: String,
    pub size: u64,
    pub modified_time: String,
}
