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
        _ => scan_hardware_info(wmi_con),  // 默认为磁盘信息
    };
    
    match result {
        Ok(output) => {
            // 输出JSON格式的结果到stdout
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