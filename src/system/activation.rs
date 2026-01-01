//! Windows激活状态检查

use crate::utils::error::{HamsterError, Result};

/// 激活状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationStatus {
    /// 已激活
    Activated,
    /// 未激活
    NotActivated,
    /// 评估版
    Evaluation,
    /// 激活过期
    Expired,
    /// 未知状态
    Unknown,
}

/// 获取Windows激活状态
#[cfg(windows)]
pub fn get_activation_status() -> Result<ActivationStatus> {
    use std::process::Command;
    
    let output = Command::new("cscript")
        .args(&["//nologo", "C:\\Windows\\System32\\slmgr.vbs", "/xpr"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
            
            if stdout.contains("永久激活") || stdout.contains("permanently activated") {
                Ok(ActivationStatus::Activated)
            } else if stdout.contains("will expire") || stdout.contains("将过期") {
                Ok(ActivationStatus::Activated)
            } else if stdout.contains("evaluation") || stdout.contains("评估") {
                Ok(ActivationStatus::Evaluation)
            } else if stdout.contains("expired") || stdout.contains("过期") {
                Ok(ActivationStatus::Expired)
            } else if stdout.contains("not activated") || stdout.contains("未激活") {
                Ok(ActivationStatus::NotActivated)
            } else {
                Ok(ActivationStatus::Unknown)
            }
        }
        Err(_) => Ok(ActivationStatus::Unknown)
    }
}

#[cfg(not(windows))]
pub fn get_activation_status() -> Result<ActivationStatus> {
    Ok(ActivationStatus::Unknown)
}

/// 检查是否已激活
pub fn is_activated() -> bool {
    matches!(get_activation_status(), Ok(ActivationStatus::Activated))
}

/// 获取激活状态描述
pub fn get_activation_description() -> String {
    match get_activation_status() {
        Ok(ActivationStatus::Activated) => "已激活".to_string(),
        Ok(ActivationStatus::NotActivated) => "未激活".to_string(),
        Ok(ActivationStatus::Evaluation) => "评估版".to_string(),
        Ok(ActivationStatus::Expired) => "激活已过期".to_string(),
        Ok(ActivationStatus::Unknown) | Err(_) => "未知状态".to_string(),
    }
}

/// 获取产品密钥（部分隐藏）
#[cfg(windows)]
pub fn get_partial_product_key() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["path", "softwarelicensingservice", "get", "OA3xOriginalProductKey", "/format:value"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取产品密钥失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("OA3xOriginalProductKey=") {
            let key = line.trim_start_matches("OA3xOriginalProductKey=").trim();
            if !key.is_empty() {
                // 只显示最后5位
                let len = key.len();
                if len > 5 {
                    return Ok(format!("*****-*****-*****-*****-{}", &key[len-5..]));
                }
                return Ok(key.to_string());
            }
        }
    }
    
    Ok("未找到产品密钥".to_string())
}

#[cfg(not(windows))]
pub fn get_partial_product_key() -> Result<String> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}
