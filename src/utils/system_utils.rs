//! 系统工具模块

use crate::utils::error::{HamsterError, Result};
use crate::types::system_types::{OSInfo, Architecture};

/// 获取操作系统信息
#[cfg(windows)]
pub fn get_os_info() -> Result<OSInfo> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["os", "get", "Caption,Version,BuildNumber,OSArchitecture", "/format:list"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取系统信息失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut os_info = OSInfo::new();
    
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "Caption" => os_info.name = value.trim().to_string(),
                "Version" => os_info.version = value.trim().to_string(),
                "BuildNumber" => os_info.build = value.trim().to_string(),
                "OSArchitecture" => {
                    os_info.architecture = if value.contains("64") {
                        Architecture::X64
                    } else if value.contains("ARM") {
                        Architecture::ARM64
                    } else {
                        Architecture::X86
                    };
                }
                _ => {}
            }
        }
    }
    
    // 获取激活状态
    os_info.is_activated = check_windows_activation().unwrap_or(false);
    os_info.activation_status = if os_info.is_activated {
        "已激活".to_string()
    } else {
        "未激活".to_string()
    };
    
    Ok(os_info)
}

#[cfg(not(windows))]
pub fn get_os_info() -> Result<OSInfo> {
    let mut os_info = OSInfo::new();
    os_info.name = "Unknown OS".to_string();
    Ok(os_info)
}

/// 检查Windows激活状态
#[cfg(windows)]
pub fn check_windows_activation() -> Result<bool> {
    use std::process::Command;
    
    let output = Command::new("cscript")
        .args(&["//nologo", "C:\\Windows\\System32\\slmgr.vbs", "/xpr"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // 如果输出包含"永久激活"或"permanently activated"，则已激活
            Ok(stdout.contains("永久") || 
               stdout.contains("permanently") || 
               stdout.contains("will expire"))
        }
        Err(_) => Ok(false)
    }
}

#[cfg(not(windows))]
pub fn check_windows_activation() -> Result<bool> {
    Ok(false)
}

/// 创建系统还原点
#[cfg(windows)]
pub fn create_restore_point(description: &str) -> Result<()> {
    use crate::utils::process_utils::run_command_silent;
    
    let script = format!(
        r#"
        $description = "{}"
        Checkpoint-Computer -Description $description -RestorePointType "APPLICATION_INSTALL"
        "#,
        description
    );
    
    let output = run_command_silent(
        "powershell",
        &["-Command", &script],
    )?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HamsterError::Unknown(format!(
            "创建还原点失败: {}",
            stderr
        )));
    }
    
    Ok(())
}

#[cfg(not(windows))]
pub fn create_restore_point(_description: &str) -> Result<()> {
    Err(HamsterError::Unknown(
        "还原点功能仅支持Windows系统".to_string()
    ))
}

/// 获取可用内存大小（字节）
#[cfg(windows)]
pub fn get_available_memory() -> Result<u64> {
    use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use std::mem::zeroed;
    
    unsafe {
        let mut mem_info: MEMORYSTATUSEX = zeroed();
        mem_info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut mem_info) == 0 {
            return Err(HamsterError::ScanError("获取内存信息失败".to_string()));
        }
        
        Ok(mem_info.ullAvailPhys)
    }
}

#[cfg(not(windows))]
pub fn get_available_memory() -> Result<u64> {
    Ok(0)
}

/// 获取总物理内存大小（字节）
#[cfg(windows)]
pub fn get_total_memory() -> Result<u64> {
    use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use std::mem::zeroed;
    
    unsafe {
        let mut mem_info: MEMORYSTATUSEX = zeroed();
        mem_info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut mem_info) == 0 {
            return Err(HamsterError::ScanError("获取内存信息失败".to_string()));
        }
        
        Ok(mem_info.ullTotalPhys)
    }
}

#[cfg(not(windows))]
pub fn get_total_memory() -> Result<u64> {
    Ok(0)
}

/// 获取系统正常运行时间（秒）
#[cfg(windows)]
pub fn get_system_uptime() -> Result<u64> {
    use winapi::um::sysinfoapi::GetTickCount64;
    
    unsafe {
        let ticks = GetTickCount64();
        Ok(ticks / 1000)
    }
}

#[cfg(not(windows))]
pub fn get_system_uptime() -> Result<u64> {
    Ok(0)
}

/// 重启计算机
#[cfg(windows)]
pub fn restart_computer(delay_seconds: u32) -> Result<()> {
    use std::process::Command;
    
    Command::new("shutdown")
        .args(&["/r", "/t", &delay_seconds.to_string()])
        .spawn()
        .map_err(|e| HamsterError::Unknown(format!("重启失败: {}", e)))?;
    
    Ok(())
}

#[cfg(not(windows))]
pub fn restart_computer(_delay_seconds: u32) -> Result<()> {
    Err(HamsterError::Unknown(
        "重启功能仅支持Windows系统".to_string()
    ))
}

/// 取消计划的重启
#[cfg(windows)]
pub fn cancel_restart() -> Result<()> {
    use std::process::Command;
    
    Command::new("shutdown")
        .args(&["/a"])
        .spawn()
        .map_err(|e| HamsterError::Unknown(format!("取消重启失败: {}", e)))?;
    
    Ok(())
}

#[cfg(not(windows))]
pub fn cancel_restart() -> Result<()> {
    Err(HamsterError::Unknown(
        "取消重启功能仅支持Windows系统".to_string()
    ))
}

/// 获取Windows目录路径
#[cfg(windows)]
pub fn get_windows_dir() -> Result<String> {
    std::env::var("WINDIR")
        .or_else(|_| std::env::var("SystemRoot"))
        .map_err(|_| HamsterError::Unknown("无法获取Windows目录".to_string()))
}

#[cfg(not(windows))]
pub fn get_windows_dir() -> Result<String> {
    Err(HamsterError::Unknown(
        "仅支持Windows系统".to_string()
    ))
}

/// 获取System32目录路径
#[cfg(windows)]
pub fn get_system32_dir() -> Result<String> {
    let windows_dir = get_windows_dir()?;
    Ok(format!("{}\\System32", windows_dir))
}

#[cfg(not(windows))]
pub fn get_system32_dir() -> Result<String> {
    Err(HamsterError::Unknown(
        "仅支持Windows系统".to_string()
    ))
}

/// 获取Program Files目录路径
#[cfg(windows)]
pub fn get_program_files_dir() -> Result<String> {
    std::env::var("ProgramFiles")
        .map_err(|_| HamsterError::Unknown("无法获取Program Files目录".to_string()))
}

#[cfg(not(windows))]
pub fn get_program_files_dir() -> Result<String> {
    Err(HamsterError::Unknown(
        "仅支持Windows系统".to_string()
    ))
}

/// 获取计算机名称
pub fn get_computer_name() -> Result<String> {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .map_err(|_| HamsterError::Unknown("无法获取计算机名称".to_string()))
}

/// 获取当前用户名
pub fn get_current_user() -> Result<String> {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .map_err(|_| HamsterError::Unknown("无法获取用户名".to_string()))
}
