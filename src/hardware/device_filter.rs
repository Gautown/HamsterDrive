//! 设备过滤和分类

use crate::types::hardware_types::{DeviceInfo, DeviceClass, DeviceStatus};
use crate::hardware::types::DeviceFilter;

/// 按过滤条件过滤设备
pub fn filter_devices(devices: &[DeviceInfo], filter: &DeviceFilter) -> Vec<DeviceInfo> {
    devices.iter()
        .filter(|device| {
            // 按设备类别过滤
            if let Some(ref class) = filter.device_class {
                if &device.device_class != class {
                    return false;
                }
            }

            // 按厂商ID过滤
            if let Some(ref vendor_id) = filter.vendor_id {
                let has_vendor = device.hardware_ids.iter().any(|h| {
                    h.vendor_id.as_ref()
                        .map(|v| v.eq_ignore_ascii_case(vendor_id))
                        .unwrap_or(false)
                });
                if !has_vendor {
                    return false;
                }
            }

            // 按设备状态过滤
            if let Some(ref status) = filter.status {
                if &device.status != status {
                    return false;
                }
            }

            // 只显示有问题的设备
            if filter.only_problems && !device.has_problem {
                return false;
            }

            // 不包含隐藏设备
            if !filter.include_hidden && device.status == DeviceStatus::Disabled {
                return false;
            }

            // 名称搜索
            if let Some(ref name_filter) = filter.name_filter {
                let name_lower = device.name.to_lowercase();
                let filter_lower = name_filter.to_lowercase();
                if !name_lower.contains(&filter_lower) {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

/// 按设备类别分组
pub fn group_by_class(devices: &[DeviceInfo]) -> std::collections::HashMap<DeviceClass, Vec<DeviceInfo>> {
    let mut groups = std::collections::HashMap::new();
    
    for device in devices {
        groups.entry(device.device_class.clone())
            .or_insert_with(Vec::new)
            .push(device.clone());
    }
    
    groups
}

/// 按厂商分组
pub fn group_by_vendor(devices: &[DeviceInfo]) -> std::collections::HashMap<String, Vec<DeviceInfo>> {
    let mut groups = std::collections::HashMap::new();
    
    for device in devices {
        let vendor = device.vendor_name.clone()
            .unwrap_or_else(|| "Unknown".to_string());
        groups.entry(vendor)
            .or_insert_with(Vec::new)
            .push(device.clone());
    }
    
    groups
}

/// 按状态分组
pub fn group_by_status(devices: &[DeviceInfo]) -> std::collections::HashMap<DeviceStatus, Vec<DeviceInfo>> {
    let mut groups = std::collections::HashMap::new();
    
    for device in devices {
        groups.entry(device.status.clone())
            .or_insert_with(Vec::new)
            .push(device.clone());
    }
    
    groups
}

/// 获取需要驱动的设备（没有驱动或有问题的设备）
pub fn get_devices_needing_drivers(devices: &[DeviceInfo]) -> Vec<DeviceInfo> {
    devices.iter()
        .filter(|d| {
            d.has_problem || d.driver_version.is_none()
        })
        .cloned()
        .collect()
}

/// 按优先级排序设备（显卡、网卡、声卡优先）
pub fn sort_by_priority(devices: &mut [DeviceInfo]) {
    devices.sort_by(|a, b| {
        let priority_a = get_device_priority(&a.device_class);
        let priority_b = get_device_priority(&b.device_class);
        priority_a.cmp(&priority_b)
    });
}

/// 获取设备类别优先级（数字越小优先级越高）
fn get_device_priority(class: &DeviceClass) -> u8 {
    match class {
        DeviceClass::Display => 1,      // 显卡最高优先级
        DeviceClass::Network => 2,      // 网卡次之
        DeviceClass::Sound => 3,        // 声卡
        DeviceClass::USB => 4,          // USB控制器
        DeviceClass::Storage => 5,      // 存储控制器
        DeviceClass::Bluetooth => 6,    // 蓝牙
        DeviceClass::Camera => 7,       // 摄像头
        DeviceClass::Input => 8,        // 输入设备
        DeviceClass::Biometric => 9,    // 生物识别
        DeviceClass::Processor => 10,   // 处理器
        DeviceClass::System => 11,      // 系统设备
        DeviceClass::Printer => 12,     // 打印机
        DeviceClass::Other(_) => 99,    // 其他设备最低优先级
    }
}

/// 搜索设备
pub fn search_devices(devices: &[DeviceInfo], query: &str) -> Vec<DeviceInfo> {
    let query_lower = query.to_lowercase();
    
    devices.iter()
        .filter(|d| {
            d.name.to_lowercase().contains(&query_lower) ||
            d.description.to_lowercase().contains(&query_lower) ||
            d.instance_id.to_lowercase().contains(&query_lower) ||
            d.vendor_name.as_ref().map(|v| v.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
            d.hardware_ids.iter().any(|h| h.full_id.to_lowercase().contains(&query_lower))
        })
        .cloned()
        .collect()
}
