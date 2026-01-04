use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareDevice {
    pub device_id: String,
    pub device_name: String,
    pub hardware_id: String,
    pub driver_version: String,
    pub driver_date: String,
    pub manufacturer: String,
    pub device_class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareScanResult {
    pub devices: Vec<HardwareDevice>,
}

pub struct HardwareScanner;

impl HardwareScanner {
    pub fn new() -> Self {
        HardwareScanner
    }

    pub fn scan_hardware(&self) -> Result<HardwareScanResult> {
        // 使用PowerShell命令获取硬件信息
        let mut devices = Vec::new();
        
        // 通过PowerShell获取PnP设备信息
        devices.extend(self.scan_with_powershell()?);
        
        Ok(HardwareScanResult { devices })
    }

    fn scan_with_powershell(&self) -> Result<Vec<HardwareDevice>> {
        let output = Command::new("powershell")
            .args(&["-Command", "Get-PnpDevice -PresentOnly | Where Status -eq 'OK' | Select-Object FriendlyName, InstanceId | ConvertTo-Json -Compress"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 解析PowerShell输出的JSON
        let mut devices = Vec::new();
        
        // 直接解析完整的JSON输出
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&output_str) {
            match &json_value {
                serde_json::Value::Array(arr) => {
                    for item in arr {
                        if let (Some(friendly_name), Some(instance_id)) = 
                            (item["FriendlyName"].as_str(), item["InstanceId"].as_str()) {
                            // 跳过空的设备名称
                            if friendly_name.is_empty() {
                                continue;
                            }
                            let device = HardwareDevice {
                                device_id: instance_id.to_string(),
                                device_name: friendly_name.to_string(),
                                hardware_id: self.extract_hardware_id(instance_id).unwrap_or_else(|| "Unknown".to_string()),
                                driver_version: "Unknown".to_string(),
                                driver_date: "Unknown".to_string(),
                                manufacturer: self.extract_manufacturer(friendly_name).unwrap_or("Unknown".to_string()),
                                device_class: self.get_device_class_from_name(friendly_name).unwrap_or_else(|_| "其他设备".to_string()),
                            };
                            devices.push(device);
                        }
                    }
                },
                serde_json::Value::Object(obj) => {
                    if let (Some(friendly_name), Some(instance_id)) = 
                        (obj["FriendlyName"].as_str(), obj["InstanceId"].as_str()) {
                        let device = HardwareDevice {
                            device_id: instance_id.to_string(),
                            device_name: friendly_name.to_string(),
                            hardware_id: self.extract_hardware_id(instance_id).unwrap_or_else(|| "Unknown".to_string()),
                            driver_version: "Unknown".to_string(),
                            driver_date: "Unknown".to_string(),
                            manufacturer: self.extract_manufacturer(friendly_name).unwrap_or("Unknown".to_string()),
                            device_class: self.get_device_class_from_name(friendly_name).unwrap_or_else(|_| "其他设备".to_string()),
                        };
                        devices.push(device);
                    }
                },
                _ => {
                    // 如果JSON解析失败，尝试简单的文本解析
                    for line in output_str.lines() {
                        let line = line.trim();
                        if !line.is_empty() && !line.starts_with('{') {
                            // 简单解析非JSON格式的输出
                            // 这里可以添加备用解析逻辑
                        }
                    }
                }
            }
        }
        
        Ok(devices)
    }
    
    /// 从设备ID中提取硬件ID
    fn extract_hardware_id(&self, device_id: &str) -> Option<String> {
        // 设备ID格式通常为: USB\VID_1234&PID_5678\... 或 PCI\VEN_1234&DEV_5678&SUBSYS_...
        // 提取主要的硬件标识符部分
        if device_id.contains("\\\\") {
            let parts: Vec<&str> = device_id.split("\\\\").collect();
            if !parts.is_empty() {
                return Some(parts[0].to_string());
            }
        } else if device_id.contains("\\") {
            let parts: Vec<&str> = device_id.split("\\").collect();
            if parts.len() > 1 {
                return Some(parts[0].to_string());
            }
        }
        
        Some(device_id.to_string())
    }
    
    /// 从设备名称中提取制造商
    fn extract_manufacturer(&self, device_name: &str) -> Option<String> {
        let name_lower = device_name.to_lowercase();
        
        if name_lower.contains("nvidia") {
            Some("NVIDIA".to_string())
        } else if name_lower.contains("amd") || name_lower.contains("ati") {
            Some("AMD".to_string())
        } else if name_lower.contains("intel") {
            Some("Intel".to_string())
        } else if name_lower.contains("realtek") {
            Some("Realtek".to_string())
        } else if name_lower.contains("microsoft") {
            Some("Microsoft".to_string())
        } else if name_lower.contains("sony") {
            Some("Sony".to_string())
        } else if name_lower.contains("logitech") {
            Some("Logitech".to_string())
        } else if name_lower.contains("hp") {
            Some("HP".to_string())
        } else if name_lower.contains("dell") {
            Some("Dell".to_string())
        } else if name_lower.contains("lenovo") {
            Some("Lenovo".to_string())
        } else if name_lower.contains("asus") {
            Some("ASUS".to_string())
        } else if name_lower.contains("acer") {
            Some("Acer".to_string())
        } else if name_lower.contains("toshiba") {
            Some("Toshiba".to_string())
        } else if name_lower.contains("samsung") {
            Some("Samsung".to_string())
        } else if name_lower.contains("broadcom") {
            Some("Broadcom".to_string())
        } else if name_lower.contains("qualcomm") {
            Some("Qualcomm".to_string())
        } else if name_lower.contains("mediatek") || name_lower.contains("mtk") {
            Some("MediaTek".to_string())
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn get_hardware_id_from_pnp(&self, device_id: &str) -> Result<String> {
        // 通过PowerShell获取硬件ID
        let command = format!("Get-PnpDeviceProperty -InstanceId \"{}\" -KeyName \"DEVPKEY_Device_HardwareIds\"", device_id);
        let output = Command::new("powershell")
            .args(&["-Command", &command])
            .output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                // 解析PowerShell输出以获取硬件ID
                for line in output_str.lines() {
                    if line.contains("Data") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() > 1 {
                            return Ok(parts[1].trim().to_string());
                        }
                    }
                }
                Ok("未知".to_string())
            }
            Err(_) => Ok("未知".to_string()),
        }
    }

    #[allow(dead_code)]
    fn get_driver_version(&self, device_id: &str) -> Result<String> {
        // 通过PowerShell获取驱动版本
        let command = format!("Get-PnpDeviceProperty -InstanceId \"{}\" -KeyName \"DEVPKEY_Device_DriverVersion\"", device_id);
        let output = Command::new("powershell")
            .args(&["-Command", &command])
            .output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                for line in output_str.lines() {
                    if line.contains("Data") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() > 1 {
                            return Ok(parts[1].trim().to_string());
                        }
                    }
                }
                Ok("未知".to_string())
            }
            Err(_) => Ok("未知".to_string()),
        }
    }

    #[allow(dead_code)]
    fn get_driver_date(&self, device_id: &str) -> Result<String> {
        // 通过PowerShell获取驱动日期
        let command = format!("Get-PnpDeviceProperty -InstanceId \"{}\" -KeyName \"DEVPKEY_Device_DriverDate\"", device_id);
        let output = Command::new("powershell")
            .args(&["-Command", &command])
            .output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                for line in output_str.lines() {
                    if line.contains("Data") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() > 1 {
                            return Ok(parts[1].trim().to_string());
                        }
                    }
                }
                Ok("未知".to_string())
            }
            Err(_) => Ok("未知".to_string()),
        }
    }

    #[allow(dead_code)]
    fn get_manufacturer_from_pnp(&self, device_name: &str) -> Result<String> {
        // 从设备名称中提取制造商信息
        let parts: Vec<&str> = device_name.split(' ').collect();
        if !parts.is_empty() {
            Ok(parts[0].to_string())
        } else {
            Ok("未知".to_string())
        }
    }

    #[allow(dead_code)]
    fn get_device_class_from_name(&self, device_name: &str) -> Result<String> {
        // 根据设备名称判断设备类型
        let name_lower = device_name.to_lowercase();
        
        if name_lower.contains("display") || name_lower.contains("graphics") || name_lower.contains("nvidia") || name_lower.contains("amd") || name_lower.contains("intel") || name_lower.contains("gpu") {
            Ok("显示适配器".to_string())
        } else if name_lower.contains("audio") || name_lower.contains("sound") || name_lower.contains("realtek") || name_lower.contains("hd audio") || name_lower.contains("audio controller") {
            Ok("声音设备".to_string())
        } else if name_lower.contains("network") || name_lower.contains("ethernet") || name_lower.contains("wireless") || name_lower.contains("wifi") || name_lower.contains("wlan") {
            Ok("网络适配器".to_string())
        } else if name_lower.contains("usb") {
            Ok("USB设备".to_string())
        } else if name_lower.contains("disk") || name_lower.contains("storage") || name_lower.contains("hard disk") || name_lower.contains("ssd") {
            Ok("存储设备".to_string())
        } else if name_lower.contains("bluetooth") {
            Ok("蓝牙设备".to_string())
        } else if name_lower.contains("camera") || name_lower.contains("webcam") {
            Ok("摄像头".to_string())
        } else if name_lower.contains("printer") {
            Ok("打印机".to_string())
        } else if name_lower.contains("monitor") {
            Ok("显示器".to_string())
        } else if name_lower.contains("keyboard") {
            Ok("键盘".to_string())
        } else if name_lower.contains("mouse") {
            Ok("鼠标".to_string())
        } else if name_lower.contains("touchpad") || name_lower.contains("ps/2") {
            Ok("输入设备".to_string())
        } else if name_lower.contains("system") || name_lower.contains("motherboard") {
            Ok("系统设备".to_string())
        } else {
            Ok("其他设备".to_string())
        }
    }
}

// 使用sysinfo库获取系统信息的补充方法
use sysinfo::System;

pub fn get_system_info_sysinfo() -> Result<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut info = String::new();
    
    info.push_str("系统信息获取中...\n");
    info.push_str(&format!("总内存: {} GB\n", sys.total_memory() / (1024 * 1024 * 1024)));
    info.push_str(&format!("可用内存: {} GB\n", sys.available_memory() / (1024 * 1024 * 1024)));
    
    // CPU信息
    if let Some(cpu) = sys.cpus().first() {
        info.push_str(&format!("CPU: {}\n", cpu.name()));
        info.push_str(&format!("CPU 频率: {} MHz\n", cpu.frequency()));
        info.push_str(&format!("CPU 核心数: {}\n", sys.cpus().len()));
    }
    
    Ok(info)
}