use crate::error::HamsterError;
use winsafe::{GetLogicalDrives, GetDiskFreeSpaceEx};
use std::process::Command;

/// 获取主板信息
fn get_motherboard_info() -> Result<String, Box<dyn std::error::Error>> {
    // 通过硬件服务获取主板信息
    let motherboard_info = get_motherboard_info_from_service()?;
    Ok(motherboard_info)
}

/// 获取CPU详细信息
fn get_cpu_details() -> Result<String, Box<dyn std::error::Error>> {
    // 通过硬件服务获取CPU信息
    let cpu_info = get_cpu_info_from_service()?;
    Ok(cpu_info)
}

/// 获取内存详细信息
fn get_memory_info() -> Result<String, Box<dyn std::error::Error>> {
    // 通过硬件服务获取内存信息
    let memory_info = get_memory_info_from_service()?;
    Ok(memory_info)
}

/// 通过调用硬件服务进程获取CPU信息
fn get_cpu_info_from_service() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取CPU信息
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--cpu")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取CPU信息
    let cpu_info = json_value.get("cpu_info").and_then(|v| v.as_str()).unwrap_or("未知CPU信息");
    
    Ok(cpu_info.to_string())
}

/// 通过调用硬件服务进程获取内存信息
fn get_memory_info_from_service() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取内存信息
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--memory")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取内存信息
    let memory_info = json_value.get("memory_info").and_then(|v| v.as_str()).unwrap_or("未知内存信息");
    
    Ok(memory_info.to_string())
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
    // 使用简单的方法获取系统信息
    // 在实际项目中，这里可以扩展为更详细的版本信息获取
    
    // 创建一个临时硬件服务来获取操作系统信息
    let os_info = get_os_info_from_service()?;
    Ok(os_info)
}

/// 通过调用硬件服务进程获取操作系统信息
fn get_os_info_from_service() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取操作系统信息
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--os")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取操作系统信息
    let os_info = json_value.get("os_info").and_then(|v| v.as_str()).unwrap_or("未知操作系统信息");
    
    Ok(os_info.to_string())
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
        .arg("--disk")
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

/// 通过调用硬件服务进程获取主板信息
fn get_motherboard_info_from_service() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // 启动硬件服务进程获取主板信息
    let output = Command::new("target/debug/hardware_service.exe")
        .arg("--motherboard")
        .output()?;

    if !output.status.success() {
        return Err(format!("硬件服务进程执行失败: {:?}", output.status).into());
    }
    
    // 解析JSON输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_value: serde_json::Value = serde_json::from_str(&stdout)?;
    
    // 提取主板信息
    let motherboard_info = json_value.get("motherboard").and_then(|v| v.as_str()).unwrap_or("未知主板信息");
    
    Ok(motherboard_info.to_string())
}

/// 获取显卡详细信息
fn get_gpu_details() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // 使用硬件服务获取显卡信息
    let gpu_info = get_gpu_info_from_service()?;
    Ok(gpu_info)
}

// 获取系统信息（启动时显示）
pub fn get_system_info() -> Result<Vec<String>, HamsterError> {
    let mut system_info = Vec::new();
    
    // 获取操作系统版本
    match get_os_version() {
        Ok(os_info) => system_info.push(os_info),
        Err(e) => system_info.push(format!("获取OS版本失败: {}", e))
    }
    
    // 获取Windows激活状态
    match get_windows_activation_status() {
        Ok(activation_info) => system_info.push(format!("Windows激活状态: {}", activation_info)),
        Err(e) => system_info.push(format!("获取激活状态失败: {}", e))
    }
    
    // 获取主板信息
    match get_motherboard_info() {
        Ok(mb_info) => system_info.push(mb_info),
        Err(e) => system_info.push(format!("获取主板信息失败: {}", e))
    }
    
    // 获取CPU信息
    match get_cpu_details() {
        Ok(cpu_info) => system_info.push(cpu_info),
        Err(e) => system_info.push(format!("获取CPU信息失败: {}", e))
    }
    
    // 获取内存信息
    match get_memory_info() {
        Ok(memory_info) => system_info.push(memory_info),
        Err(e) => system_info.push(format!("获取内存信息失败: {}", e))
    }
    
    // 获取显卡信息
    match get_gpu_details() {
        Ok(gpu_info_list) => {
            if !gpu_info_list.is_empty() {
                system_info.extend(gpu_info_list);
            } else {
                system_info.push("显卡信息: 未检测到显卡".to_string());
            }
        },
        Err(e) => system_info.push(format!("获取显卡信息失败: {}", e))
    }  
    // 获取硬盘物理信息
    match get_physical_disk_info() {
        Ok(physical_disk_info) => {
            if !physical_disk_info.is_empty() {
                system_info.push("硬盘信息:".to_string());
                system_info.extend(physical_disk_info);
            }
        },
        Err(e) => {
            // 硬盘物理信息获取失败不致命，继续其他信息
            eprintln!("获取硬盘物理信息失败: {}", e);
        }
    }
    
    // 如果系统信息为空，添加提示信息
    if system_info.is_empty() {
        system_info.push("系统信息: 暂时无法获取系统信息".to_string());
    }
    
    Ok(system_info)
}

// 扫描设备管理器中的硬件信息（点击按钮时调用）
pub fn scan_hardware() -> Result<Vec<String>, HamsterError> {
    let mut hardware_list = Vec::new();
    
    // 获取系统信息作为设备管理器扫描结果
    let system_info = get_system_info()?;
    
    hardware_list.push("设备管理器扫描结果:".to_string());
    hardware_list.extend(system_info);
    
    Ok(hardware_list)
}

pub fn scan_outdated_drivers() -> Result<Vec<DriverInfo>, HamsterError> {
    let all_drivers = get_all_drivers()?;
    
    let mut outdated = Vec::new();
    for driver in all_drivers {
        if driver.status == DriverStatus::Outdated {
            outdated.push(driver);
        }
    }
    
    Ok(outdated)
}

// 驱动信息结构体
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
    pub hardware_id: String,
    pub download_url: String,
    pub size: String,
    pub release_date: String,
    pub status: DriverStatus,
}

impl std::fmt::Display for DriverInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (版本: {} -> {})", self.name, self.current_version, self.latest_version)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DriverStatus {
    Outdated,
    UpToDate,
    NotInstalled,
}

// 获取所有驱动列表（包括已安装和未安装的）
pub fn get_all_drivers() -> Result<Vec<DriverInfo>, HamsterError> {
    #[cfg(windows)]
    {
        get_installed_drivers_via_pnputil()
    }
    #[cfg(not(windows))]
    {
        Ok(Vec::new())
    }
}

/// 使用pnputil获取已安装的驱动包信息
fn get_installed_drivers_via_pnputil() -> Result<Vec<DriverInfo>, HamsterError> {
    let mut drivers = Vec::new();
    
    let output = Command::new("pnputil")
        .args(&["/enum-drivers"])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                drivers = parse_pnputil_output(&stdout);
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                return Err(HamsterError::ScanError(format!("pnputil执行失败: {}", stderr)));
            }
        },
        Err(e) => {
            return Err(HamsterError::ScanError(format!("执行pnputil命令失败: {}", e)));
        }
    }
    
    Ok(drivers)
}

/// 解析pnputil输出
fn parse_pnputil_output(output: &str) -> Vec<DriverInfo> {
    let mut drivers = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    
    println!("解析pnputil输出，共 {} 行", lines.len());
    
    let mut current_driver: Option<DriverInfo> = None;
    
    for line in lines {
        let trimmed = line.trim();
        
        // 跳过空行和标题行
        if trimmed.is_empty() || trimmed.starts_with("Microsoft PnP") {
            continue;
        }
        
        // 查找第一个冒号的位置来分割键值对
        if let Some(colon_pos) = trimmed.find(':') {
            let key = trimmed[..colon_pos].trim();
            let value = trimmed[colon_pos + 1..].trim();
            
            println!("键: [{}], 值: [{}]", key, value);
            
            if key == "发布名称" || key == "Published Name" {
                if let Some(driver) = current_driver.take() {
                    drivers.push(driver);
                }
                
                println!("找到新驱动: {}", value);
                
                current_driver = Some(DriverInfo {
                    name: value.to_string(),
                    current_version: "未知版本".to_string(),
                    latest_version: "未知版本".to_string(),
                    hardware_id: value.to_string(),
                    download_url: String::new(),
                    size: "未知大小".to_string(),
                    release_date: "未知日期".to_string(),
                    status: DriverStatus::UpToDate,
                });
            } else if key == "原始名称" || key == "Original Name" {
                if let Some(ref mut driver) = current_driver {
                    println!("更新名称: {} -> {}", driver.name, value);
                    driver.name = value.to_string();
                }
            } else if key == "提供程序名称" || key == "Provider Name" {
                if let Some(ref mut driver) = current_driver {
                    println!("更新提供商: {} -> {}", driver.name, value);
                    driver.name = format!("{} - {}", value, driver.name);
                }
            } else if key == "驱动程序版本" || key == "Version" {
                if let Some(ref mut driver) = current_driver {
                    // 格式: "MM/DD/YYYY version" 或 "version"
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() >= 2 {
                        // 有日期和版本
                        driver.release_date = parts[0].to_string();
                        driver.current_version = parts[1].to_string();
                        driver.latest_version = parts[1].to_string();
                    } else if !parts.is_empty() {
                        // 只有版本
                        driver.current_version = parts[0].to_string();
                        driver.latest_version = parts[0].to_string();
                    }
                    println!("更新版本: {} - {}", driver.current_version, driver.release_date);
                }
            } else if key == "日期" || key == "Date" {
                if let Some(ref mut driver) = current_driver {
                    println!("更新日期: {}", value);
                    driver.release_date = value.to_string();
                }
            }
        }
    }
    
    // 保存最后一个驱动
    if let Some(driver) = current_driver {
        drivers.push(driver);
    }
    
    println!("解析完成，共找到 {} 个驱动", drivers.len());
    drivers
}