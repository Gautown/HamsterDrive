//! SetupAPI深度扫描

use crate::types::hardware_types::{DeviceInfo, DeviceClass, DeviceStatus, HardwareId};
use crate::utils::error::{HamsterError, Result};

/// 使用SetupAPI扫描设备
#[cfg(windows)]
pub fn scan_devices_setupapi() -> Result<Vec<DeviceInfo>> {
    // 使用pnputil作为替代方案
    scan_devices_pnputil()
}

#[cfg(not(windows))]
pub fn scan_devices_setupapi() -> Result<Vec<DeviceInfo>> {
    Ok(Vec::new())
}

/// 使用pnputil扫描设备
#[cfg(windows)]
fn scan_devices_pnputil() -> Result<Vec<DeviceInfo>> {
    use std::process::Command;

    let output = Command::new("pnputil")
        .args(&["/enum-devices", "/connected"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("pnputil执行失败: {}", e)))?;

    if !output.status.success() {
        return Err(HamsterError::ScanError("pnputil命令执行失败".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_pnputil_devices(&stdout)
}

/// 解析pnputil设备输出
fn parse_pnputil_devices(output: &str) -> Result<Vec<DeviceInfo>> {
    let mut devices = Vec::new();
    let mut current_device: Option<DeviceInfo> = None;

    for line in output.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            if let Some(device) = current_device.take() {
                if !device.instance_id.is_empty() {
                    devices.push(device);
                }
            }
            continue;
        }

        // 查找冒号分隔的键值对
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();

            let device = current_device.get_or_insert_with(|| DeviceInfo {
                instance_id: String::new(),
                name: String::new(),
                description: String::new(),
                device_class: DeviceClass::Other(String::new()),
                hardware_ids: Vec::new(),
                compatible_ids: Vec::new(),
                vendor_name: None,
                driver_version: None,
                driver_date: None,
                driver_provider: None,
                inf_name: None,
                status: DeviceStatus::Unknown,
                problem_code: None,
                has_problem: false,
            });

            match key {
                "Instance ID" | "实例 ID" => {
                    device.instance_id = value.to_string();
                    device.hardware_ids.push(HardwareId::parse(value));
                }
                "Device Description" | "设备描述" => {
                    device.name = value.to_string();
                    device.description = value.to_string();
                }
                "Class Name" | "类名" => {
                    device.device_class = match value.to_lowercase().as_str() {
                        "display" | "显示" => DeviceClass::Display,
                        "net" | "网络" => DeviceClass::Network,
                        "media" | "音频" | "声音" => DeviceClass::Sound,
                        "usb" => DeviceClass::USB,
                        "diskdrive" | "磁盘" => DeviceClass::Storage,
                        "system" | "系统" => DeviceClass::System,
                        "processor" | "处理器" => DeviceClass::Processor,
                        "hid" | "hidclass" | "输入" => DeviceClass::Input,
                        "bluetooth" | "蓝牙" => DeviceClass::Bluetooth,
                        "camera" | "image" | "摄像头" => DeviceClass::Camera,
                        _ => DeviceClass::Other(value.to_string()),
                    };
                }
                "Driver Name" | "驱动程序名" => {
                    device.inf_name = Some(value.to_string());
                }
                "Status" | "状态" => {
                    let status_lower = value.to_lowercase();
                    if status_lower.contains("started") || status_lower.contains("已启动") {
                        device.status = DeviceStatus::Working;
                    } else if status_lower.contains("problem") || status_lower.contains("问题") {
                        device.status = DeviceStatus::Problem;
                        device.has_problem = true;
                    } else if status_lower.contains("disabled") || status_lower.contains("已禁用") {
                        device.status = DeviceStatus::Disabled;
                    }
                }
                _ => {}
            }
        }
    }

    // 处理最后一个设备
    if let Some(device) = current_device {
        if !device.instance_id.is_empty() {
            devices.push(device);
        }
    }

    Ok(devices)
}

/// 获取特定设备的详细信息
#[cfg(windows)]
pub fn get_device_details_setupapi(instance_id: &str) -> Result<Option<DeviceInfo>> {
    use std::process::Command;

    let output = Command::new("pnputil")
        .args(&["/enum-devices", "/instanceid", instance_id])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("pnputil执行失败: {}", e)))?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let devices = parse_pnputil_devices(&stdout)?;
    
    Ok(devices.into_iter().next())
}

#[cfg(not(windows))]
pub fn get_device_details_setupapi(_instance_id: &str) -> Result<Option<DeviceInfo>> {
    Ok(None)
}

/// 禁用设备
#[cfg(windows)]
pub fn disable_device(instance_id: &str) -> Result<()> {
    use std::process::Command;

    let output = Command::new("pnputil")
        .args(&["/disable-device", instance_id])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("禁用设备失败: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HamsterError::ScanError(format!("禁用设备失败: {}", stderr)));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn disable_device(_instance_id: &str) -> Result<()> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}

/// 启用设备
#[cfg(windows)]
pub fn enable_device(instance_id: &str) -> Result<()> {
    use std::process::Command;

    let output = Command::new("pnputil")
        .args(&["/enable-device", instance_id])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("启用设备失败: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HamsterError::ScanError(format!("启用设备失败: {}", stderr)));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn enable_device(_instance_id: &str) -> Result<()> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}

/// 重启设备
#[cfg(windows)]
pub fn restart_device(instance_id: &str) -> Result<()> {
    use std::process::Command;

    let output = Command::new("pnputil")
        .args(&["/restart-device", instance_id])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("重启设备失败: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HamsterError::ScanError(format!("重启设备失败: {}", stderr)));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn restart_device(_instance_id: &str) -> Result<()> {
    Err(HamsterError::ScanError("仅支持Windows系统".to_string()))
}
