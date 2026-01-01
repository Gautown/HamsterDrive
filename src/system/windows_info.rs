//! Windows特定信息采集

use crate::utils::error::{HamsterError, Result};

/// Windows产品类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsProductType {
    Home,
    HomeN,
    Pro,
    ProN,
    Education,
    Enterprise,
    Server,
    Other(String),
}

/// 获取Windows产品类型
#[cfg(windows)]
pub fn get_windows_product_type() -> Result<WindowsProductType> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "Caption", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取产品类型失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    
    if stdout.contains("home n") {
        Ok(WindowsProductType::HomeN)
    } else if stdout.contains("home") {
        Ok(WindowsProductType::Home)
    } else if stdout.contains("pro n") {
        Ok(WindowsProductType::ProN)
    } else if stdout.contains("pro") || stdout.contains("professional") {
        Ok(WindowsProductType::Pro)
    } else if stdout.contains("education") {
        Ok(WindowsProductType::Education)
    } else if stdout.contains("enterprise") {
        Ok(WindowsProductType::Enterprise)
    } else if stdout.contains("server") {
        Ok(WindowsProductType::Server)
    } else {
        Ok(WindowsProductType::Other("Unknown".to_string()))
    }
}

#[cfg(not(windows))]
pub fn get_windows_product_type() -> Result<WindowsProductType> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}

/// 获取Windows安装日期
#[cfg(windows)]
pub fn get_install_date() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "InstallDate", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取安装日期失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("InstallDate=") {
            let date_str = line.trim_start_matches("InstallDate=").trim();
            // 格式: 20231215123456.000000+480
            if date_str.len() >= 8 {
                let year = &date_str[0..4];
                let month = &date_str[4..6];
                let day = &date_str[6..8];
                return Ok(format!("{}-{}-{}", year, month, day));
            }
        }
    }
    
    Ok("Unknown".to_string())
}

#[cfg(not(windows))]
pub fn get_install_date() -> Result<String> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}

/// 获取上次启动时间
#[cfg(windows)]
pub fn get_last_boot_time() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "LastBootUpTime", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取启动时间失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("LastBootUpTime=") {
            let date_str = line.trim_start_matches("LastBootUpTime=").trim();
            if date_str.len() >= 14 {
                let year = &date_str[0..4];
                let month = &date_str[4..6];
                let day = &date_str[6..8];
                let hour = &date_str[8..10];
                let minute = &date_str[10..12];
                let second = &date_str[12..14];
                return Ok(format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second));
            }
        }
    }
    
    Ok("Unknown".to_string())
}

#[cfg(not(windows))]
pub fn get_last_boot_time() -> Result<String> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}

/// 获取系统语言
#[cfg(windows)]
pub fn get_system_locale() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "Locale", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取系统语言失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("Locale=") {
            return Ok(line.trim_start_matches("Locale=").trim().to_string());
        }
    }
    
    Ok("Unknown".to_string())
}

#[cfg(not(windows))]
pub fn get_system_locale() -> Result<String> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}
