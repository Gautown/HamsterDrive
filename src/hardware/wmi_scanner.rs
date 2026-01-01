//! WMI扫描层实现

use crate::types::hardware_types::{DeviceInfo, DeviceClass, DeviceStatus, HardwareId};
use crate::utils::error::{HamsterError, Result};
use std::process::Command;

/// 使用WMI扫描设备
#[cfg(windows)]
pub fn scan_devices_wmi() -> Result<Vec<DeviceInfo>> {
    let mut devices = Vec::new();

    // 扫描PnP设备
    if let Ok(pnp_devices) = scan_pnp_devices() {
        devices.extend(pnp_devices);
    }

    Ok(devices)
}

#[cfg(not(windows))]
pub fn scan_devices_wmi() -> Result<Vec<DeviceInfo>> {
    Ok(Vec::new())
}

/// 扫描PnP设备
#[cfg(windows)]
fn scan_pnp_devices() -> Result<Vec<DeviceInfo>> {
    let output = Command::new("wmic")
        .args(&[
            "path", "Win32_PnPEntity",
            "get", "Name,Description,DeviceID,ClassGuid,Manufacturer,Status,DriverVersion",
            "/format:list"
        ])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("WMI查询失败: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_wmi_pnp_output(&stdout)
}

/// 解析WMI PnP设备输出
fn parse_wmi_pnp_output(output: &str) -> Result<Vec<DeviceInfo>> {
    let mut devices = Vec::new();
    let mut current_device: Option<DeviceInfo> = None;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(device) = current_device.take() {
                if !device.name.is_empty() {
                    devices.push(device);
                }
            }
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
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

            match key.trim() {
                "Name" => device.name = value.trim().to_string(),
                "Description" => device.description = value.trim().to_string(),
                "DeviceID" => {
                    device.instance_id = value.trim().to_string();
                    device.hardware_ids.push(HardwareId::parse(value.trim()));
                }
                "ClassGuid" => {
                    device.device_class = DeviceClass::from_guid(value.trim());
                }
                "Manufacturer" => {
                    device.vendor_name = Some(value.trim().to_string());
                }
                "Status" => {
                    let status = value.trim().to_lowercase();
                    device.status = if status == "ok" {
                        DeviceStatus::Working
                    } else if status == "degraded" || status == "error" {
                        device.has_problem = true;
                        DeviceStatus::Problem
                    } else {
                        DeviceStatus::Unknown
                    };
                }
                "DriverVersion" => {
                    if !value.trim().is_empty() {
                        device.driver_version = Some(value.trim().to_string());
                    }
                }
                _ => {}
            }
        }
    }

    // 处理最后一个设备
    if let Some(device) = current_device {
        if !device.name.is_empty() {
            devices.push(device);
        }
    }

    Ok(devices)
}

/// 扫描特定类别的设备
#[cfg(windows)]
pub fn scan_devices_by_class_wmi(class_guid: &str) -> Result<Vec<DeviceInfo>> {
    let query = format!(
        "path Win32_PnPEntity where ClassGuid='{}' get Name,Description,DeviceID,Manufacturer,Status,DriverVersion /format:list",
        class_guid
    );

    let output = Command::new("wmic")
        .args(query.split_whitespace())
        .output()
        .map_err(|e| HamsterError::ScanError(format!("WMI查询失败: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = parse_wmi_pnp_output(&stdout)?;
    
    // 设置设备类别
    let device_class = DeviceClass::from_guid(class_guid);
    for device in &mut devices {
        device.device_class = device_class.clone();
    }

    Ok(devices)
}

#[cfg(not(windows))]
pub fn scan_devices_by_class_wmi(_class_guid: &str) -> Result<Vec<DeviceInfo>> {
    Ok(Vec::new())
}

/// 获取设备驱动信息
#[cfg(windows)]
pub fn get_device_driver_info_wmi(device_id: &str) -> Result<Option<DriverInfo>> {

    
    let escaped_id = device_id.replace("\\", "\\\\");
    let query = format!(
        "path Win32_PnPSignedDriver where DeviceID='{}' get DriverVersion,DriverDate,DriverProviderName,InfName /format:list",
        escaped_id
    );

    let output = Command::new("wmic")
        .args(query.split_whitespace())
        .output()
        .map_err(|e| HamsterError::ScanError(format!("WMI查询失败: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut driver_version = None;
    let mut driver_date = None;
    let mut driver_provider = None;
    let mut inf_name = None;

    for line in stdout.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "DriverVersion" => driver_version = Some(value.trim().to_string()),
                "DriverDate" => driver_date = Some(value.trim().to_string()),
                "DriverProviderName" => driver_provider = Some(value.trim().to_string()),
                "InfName" => inf_name = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }

    if driver_version.is_some() {
        Ok(Some(DriverInfo {
            version: driver_version,
            date: driver_date,
            provider: driver_provider,
            inf_name,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(not(windows))]
pub fn get_device_driver_info_wmi(_device_id: &str) -> Result<Option<DriverInfo>> {
    Ok(None)
}

/// WMI驱动信息
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub version: Option<String>,
    pub date: Option<String>,
    pub provider: Option<String>,
    pub inf_name: Option<String>,
}
