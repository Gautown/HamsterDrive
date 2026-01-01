//! 操作系统信息采集

use crate::types::system_types::{OSInfo, Architecture};
use crate::utils::error::{HamsterError, Result};

/// 获取完整的操作系统信息
pub fn get_os_info() -> Result<OSInfo> {
    crate::utils::system_utils::get_os_info()
}

/// 获取操作系统名称
#[cfg(windows)]
pub fn get_os_name() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "Caption", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取系统名称失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("Caption=") {
            return Ok(line.trim_start_matches("Caption=").trim().to_string());
        }
    }
    
    Ok("Windows".to_string())
}

#[cfg(not(windows))]
pub fn get_os_name() -> Result<String> {
    Ok("Unknown OS".to_string())
}

/// 获取操作系统版本
#[cfg(windows)]
pub fn get_os_version() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "Version", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取系统版本失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("Version=") {
            return Ok(line.trim_start_matches("Version=").trim().to_string());
        }
    }
    
    Ok("Unknown".to_string())
}

#[cfg(not(windows))]
pub fn get_os_version() -> Result<String> {
    Ok("Unknown".to_string())
}

/// 获取系统构建号
#[cfg(windows)]
pub fn get_build_number() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "BuildNumber", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取构建号失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("BuildNumber=") {
            return Ok(line.trim_start_matches("BuildNumber=").trim().to_string());
        }
    }
    
    Ok("Unknown".to_string())
}

#[cfg(not(windows))]
pub fn get_build_number() -> Result<String> {
    Ok("Unknown".to_string())
}

/// 获取系统架构
pub fn get_architecture() -> Architecture {
    #[cfg(target_arch = "x86_64")]
    {
        Architecture::X64
    }
    #[cfg(target_arch = "x86")]
    {
        Architecture::X86
    }
    #[cfg(target_arch = "aarch64")]
    {
        Architecture::ARM64
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    {
        Architecture::Unknown
    }
}

/// 判断是否是64位系统
pub fn is_64bit() -> bool {
    matches!(get_architecture(), Architecture::X64 | Architecture::ARM64)
}
