//! 硬件扫描服务 - 独立进程用于执行WMI操作
use wmi::{COMLibrary, Variant, WMIConnection};
use std::collections::HashMap;
extern crate serde_json;

fn main() {
    use std::env;
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 1 {
        &args[1]
    } else {
        "--disk"  // 默认模式
    };
    
    // 初始化COM库（在独立进程中不会有冲突）
    let com_lib = match COMLibrary::without_security() {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("COM库初始化失败: {}", e);
            println!("{{\"error\": \"COM库初始化失败: {}\"}}", e);
            return;
        }
    };
    
    let wmi_con = match WMIConnection::new(com_lib) {
        Ok(con) => con,
        Err(e) => {
            eprintln!("WMI连接失败: {}", e);
            println!("{{\"error\": \"WMI连接失败: {}\"}}", e);
            return;
        }
    };
    
    // 根据模式执行不同的硬件扫描
    let result = match mode {
        "--gpu" => scan_gpu_info(wmi_con),
        "--activation" => scan_activation_status(wmi_con),
        "--motherboard" => scan_motherboard_info(wmi_con),
        "--os" => scan_os_info(wmi_con),
        "--cpu" => scan_cpu_info(wmi_con),
        "--memory" => scan_memory_info(wmi_con),
        _ => scan_hardware_info(wmi_con),  // 默认为磁盘信息
    };
    
    match result {
        Ok(output) => {
            // 输出JSON格式的结果到stdout
            // 使用println!会添加\n，并且自动处理UTF-8编码
            println!("{}", output);
        },
        Err(e) => {
            eprintln!("硬件扫描失败: {}", e);
            println!("{{\"error\": \"硬件扫描失败: {}\"}}", e);
        }
    }
}

/// 扫描硬件信息
fn scan_hardware_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let mut disks = Vec::new();
    
    // 查询硬盘信息
    match query_disk_drives(&wmi_con) {
        Ok(disk_info) => disks.extend(disk_info),
        Err(e) => eprintln!("查询硬盘信息失败: {}", e)
    }
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "disks": disks
    });
    
    Ok(json_result.to_string())
}

/// 扫描Windows激活状态
fn scan_activation_status(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let activation_status = query_activation_status(&wmi_con)?;
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "activation_status": activation_status
    });
    
    Ok(json_result.to_string())
}

/// 扫描主板信息
fn scan_motherboard_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let motherboard_info = query_motherboard_info(&wmi_con)?;
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "motherboard": motherboard_info
    });
    
    Ok(json_result.to_string())
}

/// 扫描操作系统信息
fn scan_os_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let os_info = query_os_info(&wmi_con)?;
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "os_info": os_info
    });
    
    Ok(json_result.to_string())
}

/// 查询Windows激活状态
fn query_activation_status(wmi_con: &WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    // 查询SoftwareLicensingProduct类获取激活状态
    // 我们查找Name以"Windows"开头且PartialProductKey不为空的条目
    let query = "SELECT Name, LicenseStatus, PartialProductKey FROM SoftwareLicensingProduct WHERE Name LIKE '%Windows%' AND PartialProductKey IS NOT NULL";
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(query)?;
    
    // 如果没有找到结果，返回未知状态
    if results.is_empty() {
        return Ok("未知".to_string());
    }
    
    // 获取第一个结果
    let product = &results[0];
    
    // 获取许可证状态
    let license_status = product.get("LicenseStatus").map_or(0u32, |v| {
        match v {
            Variant::I4(val) => *val as u32,
            _ => 0
        }
    });
    
    // 根据许可证状态返回简洁的激活状态
    let status_text = match license_status {
        1 => "已激活",
        _ => "未激活"
    };
    
    Ok(status_text.to_string())
}

/// 查询主板信息
fn query_motherboard_info(wmi_con: &WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    // 查询Win32_BaseBoard类获取主板信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Manufacturer, Product, SerialNumber FROM Win32_BaseBoard")?;
    
    // 如果没有找到结果，返回未知状态
    if results.is_empty() {
        return Ok("未知主板信息".to_string());
    }
    
    // 获取第一个结果
    let motherboard = &results[0];
    
    // 获取制造商和产品信息
    let manufacturer = motherboard.get("Manufacturer").map_or("未知制造商".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    let product = motherboard.get("Product").map_or("未知型号".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    Ok(format!("制造商: {}, 型号: {}", manufacturer, product))
}

/// 查询操作系统信息
fn query_os_info(wmi_con: &WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    // 查询Win32_OperatingSystem类获取操作系统信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Caption, Version, BuildNumber, OSLanguage FROM Win32_OperatingSystem")?;
    
    // 如果没有找到结果，返回未知状态
    if results.is_empty() {
        return Ok("未知操作系统信息".to_string());
    }
    
    // 获取第一个结果
    let os = &results[0];
    
    // 获取操作系统名称、版本和构建号
    let caption = os.get("Caption").map_or("未知系统".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    let version = os.get("Version").map_or("未知版本".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    let build_number = os.get("BuildNumber").map_or("未知构建".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    Ok(format!("{} (版本: {}, 构建: {})", caption, version, build_number))
}

/// 扫描GPU信息
fn scan_gpu_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let mut gpus = Vec::new();
    
    // 查询GPU信息
    match query_gpu_info(&wmi_con) {
        Ok(gpu_info) => gpus.extend(gpu_info),
        Err(e) => eprintln!("查询GPU信息失败: {}", e)
    }
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "gpus": gpus
    });
    
    Ok(json_result.to_string())
}

/// 查询GPU信息
fn query_gpu_info(wmi_con: &WMIConnection) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut gpu_list = Vec::new();
    
    // 查询Win32_VideoController类获取GPU信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Name, AdapterRAM FROM Win32_VideoController")?;
    
    for gpu in results {
        let name = gpu.get("Name").map_or("未知型号".to_string(), |v| {
            match v {
                Variant::String(s) => s.clone(),
                _ => format!("{:?}", v),
            }
        });
        
        let adapter_ram = gpu.get("AdapterRAM").map_or("未知显存".to_string(), |v| {
            match v {
                Variant::UI4(bytes) => {
                    let mb = *bytes as f64 / (1024.0 * 1024.0);
                    format!("{:.0} MB", mb)
                },
                Variant::UI8(bytes) => {
                    let mb = *bytes as f64 / (1024.0 * 1024.0);
                    format!("{:.0} MB", mb)
                },
                _ => format!("{:?}", v),
            }
        });
        
        // 按照指定格式构造GPU信息
        let gpu_info = serde_json::json!({
            "display": format!("显卡: {} ({})", name, adapter_ram),
            "name": name,
            "memory": adapter_ram
        });
        
        gpu_list.push(gpu_info);
    }
    
    Ok(gpu_list)
}

/// 查询硬盘驱动器信息
fn query_disk_drives(wmi_con: &WMIConnection) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut disk_list = Vec::new();
    
    // 查询Win32_DiskDrive类获取硬盘信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Model, Size FROM Win32_DiskDrive")?;
    
    for disk in results {
        let model = disk.get("Model").map_or("未知型号".to_string(), |v| {
            match v {
                Variant::String(s) => s.clone(),
                _ => format!("{:?}", v),
            }
        });
        
        let size = disk.get("Size").map_or("未知容量".to_string(), |v| {
            match v {
                Variant::UI8(bytes) => {
                    let gb = *bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    format!("{:.1} GB", gb)
                },
                Variant::I8(bytes) => {
                    let gb = *bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    format!("{:.1} GB", gb)
                },
                Variant::UI4(bytes) => {
                    let gb = *bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    format!("{:.1} GB", gb)
                },
                Variant::I4(bytes) => {
                    let gb = *bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    format!("{:.1} GB", gb)
                },
                _ => format!("{:?}", v),
            }
        });
        
        // 按照指定格式构造硬盘信息
        let disk_info = serde_json::json!({
            "display": format!("硬盘: {} 容量: {}", model, size),
            "model": model,
            "size": size
        });
        
        disk_list.push(disk_info);
    }
    
    Ok(disk_list)
}

/// 扫描CPU信息
fn scan_cpu_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let cpu_info = query_cpu_info(&wmi_con)?;
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "cpu_info": cpu_info
    });
    
    Ok(json_result.to_string())
}

/// 扫描内存信息
fn scan_memory_info(wmi_con: WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    let memory_info = query_memory_info(&wmi_con)?;
    
    // 构造JSON响应
    let json_result = serde_json::json!({
        "memory_info": memory_info
    });
    
    Ok(json_result.to_string())
}

/// 查询CPU信息
fn query_cpu_info(wmi_con: &WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    // 查询Win32_Processor类获取CPU信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Name, MaxClockSpeed, NumberOfCores, NumberOfLogicalProcessors FROM Win32_Processor")?;
    
    // 如果没有找到结果，返回未知状态
    if results.is_empty() {
        return Ok("未知CPU信息".to_string());
    }
    
    // 获取第一个结果
    let processor = &results[0];
    
    // 获取CPU信息
    let name = processor.get("Name").map_or("未知CPU".to_string(), |v| {
        match v {
            Variant::String(s) => s.clone(),
            _ => format!("{:?}", v),
        }
    });
    
    let max_clock_speed = processor.get("MaxClockSpeed").map_or("未知主频".to_string(), |v| {
        match v {
            Variant::UI4(speed) => format!("{} MHz", speed),
            _ => format!("{:?}", v),
        }
    });
    
    let cores = processor.get("NumberOfCores").map_or("未知核心数".to_string(), |v| {
        match v {
            Variant::UI4(count) => format!("{} 核心", count),
            _ => format!("{:?}", v),
        }
    });
    
    let logical_processors = processor.get("NumberOfLogicalProcessors").map_or("未知线程数".to_string(), |v| {
        match v {
            Variant::UI4(count) => format!("{} 线程", count),
            _ => format!("{:?}", v),
        }
    });
    
    Ok(format!("{} ({} 主频: {} 核心: {} 线程)", name, cores, max_clock_speed, logical_processors))
}

/// 查询内存信息
fn query_memory_info(wmi_con: &WMIConnection) -> Result<String, Box<dyn std::error::Error>> {
    // 查询Win32_OperatingSystem类获取总内存信息
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT TotalVisibleMemorySize, FreePhysicalMemory FROM Win32_OperatingSystem")?;
    
    // 如果没有找到结果，返回未知状态
    if results.is_empty() {
        return Ok("未知内存信息".to_string());
    }
    
    // 获取第一个结果
    let os = &results[0];
    
    // 获取总内存大小
    let total_memory = os.get("TotalVisibleMemorySize").map_or(0u64, |v| {
        match v {
            Variant::UI8(bytes) => *bytes,
            Variant::UI4(bytes) => *bytes as u64,
            _ => 0,
        }
    });
    
    // 获取可用内存大小
    let free_memory = os.get("FreePhysicalMemory").map_or(0u64, |v| {
        match v {
            Variant::UI8(bytes) => *bytes,
            Variant::UI4(bytes) => *bytes as u64,
            _ => 0,
        }
    });
    
    // 计算已使用内存
    let used_memory = total_memory - free_memory;
    
    // 转换为更友好的格式
    let total_gb = total_memory as f64 / (1024.0 * 1024.0);
    let used_gb = used_memory as f64 / (1024.0 * 1024.0);
    let free_gb = free_memory as f64 / (1024.0 * 1024.0);
    
    Ok(format!("总内存: {:.1} GB, 已使用: {:.1} GB, 可用: {:.1} GB", total_gb, used_gb, free_gb))
}