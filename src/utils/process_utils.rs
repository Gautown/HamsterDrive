use crate::utils::error::{HamsterError, Result};
use std::process::Command;

/// 执行命令并返回输出
pub fn run_command(cmd: &str, args: &[&str]) -> Result<std::process::Output> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| HamsterError::IoError(e.to_string()))?;
    
    Ok(output)
}

/// 静默执行命令（不显示控制台窗口）
pub fn run_command_silent(cmd: &str, args: &[&str]) -> Result<std::process::Output> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| HamsterError::IoError(e.to_string()))?;
    
    Ok(output)
}

/// 执行PowerShell命令
pub fn run_powershell_command(script: &str) -> Result<std::process::Output> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(script)
        .output()
        .map_err(|e| HamsterError::IoError(e.to_string()))?;
    
    Ok(output)
}