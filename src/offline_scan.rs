use crate::error::HamsterError;
use crate::scan::{get_system_info, scan_hardware};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 离线扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineScanResult {
    pub scan_time: String,
    pub system_info: Vec<String>,
    pub hardware_info: Vec<String>,
    pub os_version: String,
    pub machine_guid: String,
}

/// 执行离线扫描
pub fn perform_offline_scan() -> Result<OfflineScanResult, HamsterError> {
    let scan_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let system_info = get_system_info()?;
    let hardware_info = scan_hardware()?;
    
    let os_version = get_os_version()?;
    let machine_guid = get_machine_guid()?;
    
    Ok(OfflineScanResult {
        scan_time,
        system_info,
        hardware_info,
        os_version,
        machine_guid,
    })
}

/// 保存离线扫描结果到文件
pub fn save_offline_scan_result(result: &OfflineScanResult, output_path: &str) -> Result<(), HamsterError> {
    let json_data = serde_json::to_string_pretty(result)
        .map_err(|e| HamsterError::ScanError(format!("序列化失败: {}", e)))?;
    
    fs::write(output_path, json_data)
        .map_err(|e| HamsterError::ScanError(format!("保存文件失败: {}", e)))?;
    
    println!("离线扫描结果已保存到: {}", output_path);
    Ok(())
}

/// 从文件加载离线扫描结果
pub fn load_offline_scan_result(input_path: &str) -> Result<OfflineScanResult, HamsterError> {
    let json_data = fs::read_to_string(input_path)
        .map_err(|e| HamsterError::ScanError(format!("读取文件失败: {}", e)))?;
    
    let result: OfflineScanResult = serde_json::from_str(&json_data)
        .map_err(|e| HamsterError::ScanError(format!("反序列化失败: {}", e)))?;
    
    Ok(result)
}

/// 生成离线扫描报告
pub fn generate_offline_report(result: &OfflineScanResult) -> String {
    let mut report = String::new();
    
    report.push_str("========================================\n");
    report.push_str("        HamsterDrive 离线扫描报告\n");
    report.push_str("========================================\n\n");
    
    report.push_str(&format!("扫描时间: {}\n\n", result.scan_time));
    report.push_str(&format!("操作系统: {}\n\n", result.os_version));
    report.push_str(&format!("机器GUID: {}\n\n", result.machine_guid));
    
    report.push_str("----------------------------------------\n");
    report.push_str("系统信息:\n");
    report.push_str("----------------------------------------\n");
    for info in &result.system_info {
        report.push_str(&format!("{}\n", info));
    }
    
    report.push_str("\n----------------------------------------\n");
    report.push_str("硬件信息:\n");
    report.push_str("----------------------------------------\n");
    for info in &result.hardware_info {
        report.push_str(&format!("{}\n", info));
    }
    
    report.push_str("\n========================================\n");
    report.push_str("报告结束\n");
    report.push_str("========================================\n");
    
    report
}

/// 获取默认的离线扫描文件路径
pub fn get_default_offline_scan_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("hamsterdrive_offline_scan.json");
    path
}

/// 获取操作系统版本
fn get_os_version() -> Result<String, HamsterError> {
    use std::process::Command;
    
    let output = Command::new("cmd")
        .args(&["/c", "ver"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取OS版本失败: {}", e)))?;
    
    let version = String::from_utf8_lossy(&output.stdout);
    Ok(version.trim().to_string())
}

/// 获取机器GUID
fn get_machine_guid() -> Result<String, HamsterError> {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["csproduct", "get", "UUID"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取机器GUID失败: {}", e)))?;
    
    let guid = String::from_utf8_lossy(&output.stdout);
    let guid = guid.lines()
        .skip(1)
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    
    Ok(guid)
}

/// 导出离线扫描为CSV格式
pub fn export_to_csv(result: &OfflineScanResult, output_path: &str) -> Result<(), HamsterError> {
    let mut csv = String::new();
    
    csv.push_str("类型,信息\n");
    csv.push_str(&format!("扫描时间,{}\n", result.scan_time));
    csv.push_str(&format!("操作系统,{}\n", result.os_version));
    csv.push_str(&format!("机器GUID,{}\n", result.machine_guid));
    
    for info in &result.system_info {
        csv.push_str(&format!("系统信息,{}\n", info));
    }
    
    for info in &result.hardware_info {
        csv.push_str(&format!("硬件信息,{}\n", info));
    }
    
    fs::write(output_path, csv)
        .map_err(|e| HamsterError::ScanError(format!("保存CSV文件失败: {}", e)))?;
    
    println!("CSV文件已保存到: {}", output_path);
    Ok(())
}

/// 导出离线扫描为TXT格式
pub fn export_to_txt(result: &OfflineScanResult, output_path: &str) -> Result<(), HamsterError> {
    let report = generate_offline_report(result);
    
    fs::write(output_path, report)
        .map_err(|e| HamsterError::ScanError(format!("保存TXT文件失败: {}", e)))?;
    
    println!("TXT文件已保存到: {}", output_path);
    Ok(())
}
