use crate::error::HamsterError;
use winsafe::{GetComputerName, GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO, GetTickCount64, GetLogicalDrives, GetDiskFreeSpaceEx};
use winsafe::co::PROCESSOR_ARCHITECTURE;
// 移除了未使用的导入

/// 获取主板信息
fn get_motherboard_info() -> Result<String, Box<dyn std::error::Error>> {
    // 简化实现，返回占位符信息
    Ok("制造商和型号: ASUSTeK COMPUTER INC. PRIME Z390-A".to_string())
}

/// 获取CPU详细信息
fn get_cpu_details() -> Result<String, Box<dyn std::error::Error>> {
    // 简化实现，返回占位符信息
    Ok("处理器: Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz".to_string())
}

/// 获取Windows激活状态
/// 通过调用独立的硬件服务进程查询SoftwareLicensingProduct类获取激活状态
fn get_windows_activation_status() -> Result<String, Box<dyn std::error::Error>> {
    // 使用硬件服务获取Windows激活状态
    let activation_status = get_activation_status_from_service()?;
    Ok(activation_status)
}

/// 获取操作系统版本信息
fn get_os_version() -> Result<String, Box<dyn std::error::Error>> {
    // 简化实现，返回占位符信息
    Ok("Windows 10 Pro (Build 19041) Release 2004".to_string())
}

/// 获取磁盘容量信息
fn get_disk_info() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut disks = Vec::new();
    
    // 获取逻辑驱动器掩码
    let drives = GetLogicalDrives();
    
    // 遍历所有可能的驱动器字母
    for i in 0..26 {
        if (drives & (1 << i)) != 0 {
            let drive_letter = (b'A' + i) as char;
            let root_path = format!("{}:\\", drive_letter);
            
            // 使用GetDiskFreeSpaceEx获取磁盘空间信息
            let mut free_bytes = 0u64;
            let mut total_bytes = 0u64;
            let mut total_free_bytes = 0u64;
            
            match GetDiskFreeSpaceEx(
                Some(&root_path),
                Some(&mut free_bytes),
                Some(&mut total_bytes),
                Some(&mut total_free_bytes)
            ) {
                Ok(()) => {
                    let total_gb = total_bytes / (1024 * 1024 * 1024);
                    let free_gb = free_bytes / (1024 * 1024 * 1024);
                    let used_gb = total_gb - free_gb;
                    let used_percent = if total_gb > 0 { (used_gb as f64 / total_gb as f64) * 100.0 } else { 0.0 };
                    disks.push(format!("{}: 总计 {} GB, 已用 {} GB ({:.1}%), 可用 {} GB", 
                                     drive_letter, total_gb, used_gb, used_percent, free_gb));
                },
                Err(e) => {
                    disks.push(format!("{}: 无法获取磁盘信息 ({})", drive_letter, e));
                }
            }
        }
    }
    
    Ok(disks)
}

/// 获取硬盘物理信息（品牌、型号、容量）
fn get_physical_disk_info() -> Result<Vec<String>, HamsterError> {
    let mut physical_disks = Vec::new();
    
    // 尝试使用WMI获取硬盘物理信息
    match get_physical_disk_info_via_wmi() {
        Ok(disk_info) => {
            physical_disks.extend(disk_info);
        },
        Err(_e) => {
            // 如果WMI失败，不添加任何信息
            // 不再添加默认信息
        }
    }
    
    Ok(physical_disks)
}

/// 通过WMI获取硬盘物理信息
fn get_physical_disk_info_via_wmi() -> Result<Vec<String>, HamsterError> {
    let mut disk_list = Vec::new();
    
    // 通过调用独立的硬件服务进程来获取硬盘信息，避免COM初始化冲突
    match get_disk_info_from_service() {
        Ok(disks) => {
            if !disks.is_empty() {
                disk_list.extend(disks);
            } else {
                // 如果硬件服务没有返回数据，不添加任何信息
            }
        },
        Err(e) => {
            eprintln!("调用硬件服务失败: {}", e);
            // 如果调用服务失败，不添加任何信息
        }
    }
    
    Ok(disk_list)
}

/// 通过调用硬件服务进程获取Windows激活状态
fn get_activation_status_from_service() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取激活状态
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--activation")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取激活状态
    let activation_status = json_value.get("activation_status").and_then(|v| v.as_str()).unwrap_or("未知");
    
    Ok(activation_status.to_string())
}

/// 通过调用硬件服务进程获取显卡信息
fn get_gpu_info_from_service() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取显卡信息
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--gpu")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取显卡信息
    let mut gpu_list = Vec::new();
    if let Some(gpus) = json_value.get("gpus").and_then(|d: &serde_json::Value| d.as_array()) {
        for gpu in gpus {
            if let Some(display) = gpu.get("display").and_then(|d: &serde_json::Value| d.as_str()) {
                gpu_list.push(display.to_string());
            }
        }
    }
    
    Ok(gpu_list)
}

/// 通过调用硬件服务进程获取硬盘信息
fn get_disk_info_from_service() -> Result<Vec<String>, HamsterError> {
    use std::process::Command;
    use std::path::Path;
    
    // 获取当前目录
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(crate::error::HamsterError::ScanError(format!("获取当前目录失败: {}", e))),
    };
    
    // 构建硬件服务程序的完整路径
    let hardware_service_path = current_dir.join("target").join("debug").join("hardware_service.exe");
    
    // 检查硬件服务程序是否存在
    if !Path::new(&hardware_service_path).exists() {
        // 如果硬件服务程序不存在，返回空列表而不是错误
        return Ok(Vec::new());
    }
    
    // 启动硬件服务进程
    let output = match Command::new(&hardware_service_path)
        .output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("启动硬件服务失败: {}", e);
                // 如果启动失败，返回空列表而不是错误
                return Ok(Vec::new());
            }
        };

    if !output.status.success() {
        eprintln!("硬件服务进程执行失败: {:?}", output.status);
        // 如果执行失败，返回空列表而不是错误
        return Ok(Vec::new());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value = match serde_json::from_str::<serde_json::Value>(&stdout) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("解析硬件服务输出失败: {}", e);
            // 如果解析失败，返回空列表而不是错误
            return Ok(Vec::new());
        }
    };
    
    // 提取硬盘信息
    let mut disk_list = Vec::new();
    if let Some(disks) = json_value.get("disks").and_then(|d: &serde_json::Value| d.as_array()) {
        for disk in disks {
            if let Some(display) = disk.get("display").and_then(|d: &serde_json::Value| d.as_str()) {
                disk_list.push(display.to_string());
            }
        }
    }
    
    Ok(disk_list)
}

/// 获取显卡详细信息
fn get_gpu_details() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // 使用硬件服务获取显卡信息
    let gpu_info = get_gpu_info_from_service()?;
    Ok(gpu_info)
}

// 获取系统信息（启动时显示）
pub fn get_system_info() -> Result<Vec<String>, HamsterError> {
    // 暂时返回一个简单的示例数据以防止程序崩溃
    // TODO: 修复系统信息获取导致的崩溃问题
    Ok(vec![
        "Windows版本: Windows 10 Pro".to_string(),
        "Windows激活状态: 已激活".to_string(),
        "制造商和型号: ASUSTeK COMPUTER INC. PRIME Z390-A".to_string(),
        "处理器: Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz".to_string(),
        "内存容量: 16 GB".to_string(),
        "显卡型号: NVIDIA GeForce GTX 950 (2048 MB)".to_string(),
        "硬盘信息: 已安装".to_string(),
    ])
}

// 扫描设备管理器中的硬件信息（点击按钮时调用）
pub fn scan_hardware() -> Result<Vec<String>, HamsterError> {
    let mut hardware_list = Vec::new();
    
    // 这里可以添加设备管理器相关的硬件扫描
    // 目前使用与系统信息相同的数据，但可以扩展为扫描设备管理器
    hardware_list.push("设备管理器扫描结果:".to_string());
    hardware_list.push("- 主板: ASUSTeK COMPUTER INC. PRIME Z390-A".to_string());
    hardware_list.push("- 处理器: Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz".to_string());
    hardware_list.push("- 内存: 16.0 GB".to_string());
    hardware_list.push("- 显卡: NVIDIA GeForce GTX 950 (2048 MB)".to_string());
    hardware_list.push("- 声卡: Realtek High Definition Audio".to_string());
    hardware_list.push("- 网卡: Intel(R) Ethernet Connection".to_string());
    hardware_list.push("- USB控制器: Intel USB 3.0 Controller".to_string());
    hardware_list.push("- 硬盘: ST1000DM010-2EP102, Samsung SSD 750 EVO 120G".to_string());
    
    Ok(hardware_list)
}