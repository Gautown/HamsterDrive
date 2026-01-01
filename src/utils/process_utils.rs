//! 进程工具模块

use std::process::{Command, Output, Stdio};
use std::time::Duration;
use crate::utils::error::{HamsterError, Result};

/// 执行命令并获取输出
pub fn run_command(program: &str, args: &[&str]) -> Result<Output> {
    Command::new(program)
        .args(args)
        .output()
        .map_err(|e| HamsterError::Unknown(format!("执行命令失败: {}", e)))
}

/// 执行命令并获取stdout字符串
pub fn run_command_stdout(program: &str, args: &[&str]) -> Result<String> {
    let output = run_command(program, args)?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HamsterError::Unknown(format!(
            "命令执行失败: {}",
            stderr
        )));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 静默执行命令（隐藏窗口）
#[cfg(windows)]
pub fn run_command_silent(program: &str, args: &[&str]) -> Result<Output> {
    use std::os::windows::process::CommandExt;
    
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    Command::new(program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| HamsterError::Unknown(format!("执行命令失败: {}", e)))
}

#[cfg(not(windows))]
pub fn run_command_silent(program: &str, args: &[&str]) -> Result<Output> {
    run_command(program, args)
}

/// 以管理员权限执行命令
#[cfg(windows)]
pub fn run_as_admin(program: &str, args: &str) -> Result<()> {
    use std::ptr::null_mut;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    // 使用 ShellExecuteW 以管理员权限运行
    let operation: Vec<u16> = OsStr::new("runas")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let file: Vec<u16> = OsStr::new(program)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let parameters: Vec<u16> = OsStr::new(args)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        let result = winapi::um::shellapi::ShellExecuteW(
            null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters.as_ptr(),
            null_mut(),
            winapi::um::winuser::SW_SHOWNORMAL,
        );
        
        if (result as isize) <= 32 {
            return Err(HamsterError::PermissionError(
                "无法以管理员权限运行程序".to_string()
            ));
        }
    }
    
    Ok(())
}

#[cfg(not(windows))]
pub fn run_as_admin(program: &str, args: &str) -> Result<()> {
    Err(HamsterError::PermissionError(
        "管理员权限运行仅支持Windows系统".to_string()
    ))
}

/// 检查当前进程是否具有管理员权限
#[cfg(windows)]
pub fn is_elevated() -> bool {
    use std::ptr::null_mut;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY, HANDLE};
    
    unsafe {
        let mut token_handle: HANDLE = null_mut();
        
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }
        
        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_size = 0u32;
        
        let success = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_size,
        );
        
        winapi::um::handleapi::CloseHandle(token_handle);
        
        success != 0 && elevation.TokenIsElevated != 0
    }
}

#[cfg(not(windows))]
pub fn is_elevated() -> bool {
    // 在非Windows系统上，检查是否是root用户
    unsafe { libc::geteuid() == 0 }
}

/// 终止进程
#[cfg(windows)]
pub fn kill_process(pid: u32) -> Result<()> {
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::processthreadsapi::TerminateProcess;
    use winapi::um::winnt::PROCESS_TERMINATE;
    
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return Err(HamsterError::Unknown("无法打开进程".to_string()));
        }
        
        let result = TerminateProcess(handle, 1);
        winapi::um::handleapi::CloseHandle(handle);
        
        if result == 0 {
            return Err(HamsterError::Unknown("无法终止进程".to_string()));
        }
    }
    
    Ok(())
}

#[cfg(not(windows))]
pub fn kill_process(pid: u32) -> Result<()> {
    use std::process::Command;
    
    Command::new("kill")
        .args(&["-9", &pid.to_string()])
        .output()
        .map_err(|e| HamsterError::Unknown(format!("无法终止进程: {}", e)))?;
    
    Ok(())
}

/// 等待进程退出
pub fn wait_for_process(pid: u32, timeout_seconds: u64) -> Result<bool> {
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_seconds);
    
    while start.elapsed() < timeout {
        if !is_process_running(pid) {
            return Ok(true);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    
    Ok(false)
}

/// 检查进程是否正在运行
#[cfg(windows)]
pub fn is_process_running(pid: u32) -> bool {
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
    
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
        if handle.is_null() {
            return false;
        }
        winapi::um::handleapi::CloseHandle(handle);
        true
    }
}

#[cfg(not(windows))]
pub fn is_process_running(pid: u32) -> bool {
    std::path::Path::new(&format!("/proc/{}", pid)).exists()
}
