//! 硬件标识符解析

use crate::types::hardware_types::{HardwareId, get_vendor_name};
use regex::Regex;

/// 解析硬件ID字符串
pub fn parse_hardware_id(id_string: &str) -> HardwareId {
    HardwareId::parse(id_string)
}

/// 从硬件ID提取厂商名称
pub fn get_vendor_from_hardware_id(hardware_id: &HardwareId) -> Option<String> {
    hardware_id.vendor_id.as_ref()
        .and_then(|vid| get_vendor_name(vid).map(|s| s.to_string()))
}

/// 判断两个硬件ID是否兼容
pub fn are_hardware_ids_compatible(id1: &HardwareId, id2: &HardwareId) -> bool {
    // 完全匹配
    if id1.full_id.eq_ignore_ascii_case(&id2.full_id) {
        return true;
    }

    // 厂商和设备ID匹配
    if let (Some(ven1), Some(dev1)) = (&id1.vendor_id, &id1.device_id) {
        if let (Some(ven2), Some(dev2)) = (&id2.vendor_id, &id2.device_id) {
            return ven1.eq_ignore_ascii_case(ven2) && dev1.eq_ignore_ascii_case(dev2);
        }
    }

    false
}

/// 计算硬件ID匹配分数
pub fn calculate_match_score(device_id: &HardwareId, driver_id: &HardwareId) -> u32 {
    let mut score = 0u32;

    // 完全匹配
    if device_id.full_id.eq_ignore_ascii_case(&driver_id.full_id) {
        return 1000;
    }

    // 厂商ID匹配
    if let (Some(dev_ven), Some(drv_ven)) = (&device_id.vendor_id, &driver_id.vendor_id) {
        if dev_ven.eq_ignore_ascii_case(drv_ven) {
            score += 100;
        }
    }

    // 设备ID匹配
    if let (Some(dev_dev), Some(drv_dev)) = (&device_id.device_id, &driver_id.device_id) {
        if dev_dev.eq_ignore_ascii_case(drv_dev) {
            score += 200;
        }
    }

    // 子系统ID匹配
    if let (Some(dev_sub), Some(drv_sub)) = (&device_id.subsys_id, &driver_id.subsys_id) {
        if dev_sub.eq_ignore_ascii_case(drv_sub) {
            score += 50;
        }
    }

    // 修订版本匹配
    if let (Some(dev_rev), Some(drv_rev)) = (&device_id.revision, &driver_id.revision) {
        if dev_rev.eq_ignore_ascii_case(drv_rev) {
            score += 10;
        }
    }

    score
}

/// 从设备实例ID提取设备类型
pub fn extract_device_type_from_instance_id(instance_id: &str) -> Option<String> {
    // 实例ID格式通常是: TYPE\HARDWARE_ID\INSTANCE
    let parts: Vec<&str> = instance_id.split('\\').collect();
    if !parts.is_empty() {
        return Some(parts[0].to_string());
    }
    None
}

/// 格式化硬件ID用于显示
pub fn format_hardware_id_for_display(hardware_id: &HardwareId) -> String {
    let mut parts = Vec::new();

    if let Some(ref ven) = hardware_id.vendor_id {
        if let Some(vendor_name) = get_vendor_name(ven) {
            parts.push(format!("厂商: {} ({})", vendor_name, ven));
        } else {
            parts.push(format!("厂商ID: {}", ven));
        }
    }

    if let Some(ref dev) = hardware_id.device_id {
        parts.push(format!("设备ID: {}", dev));
    }

    if let Some(ref sub) = hardware_id.subsys_id {
        parts.push(format!("子系统: {}", sub));
    }

    if let Some(ref rev) = hardware_id.revision {
        parts.push(format!("修订: {}", rev));
    }

    if parts.is_empty() {
        hardware_id.full_id.clone()
    } else {
        parts.join(", ")
    }
}

/// 从INF文件内容提取支持的硬件ID列表
pub fn extract_hardware_ids_from_inf(inf_content: &str) -> Vec<HardwareId> {
    let mut hardware_ids = Vec::new();
    
    // 匹配常见的硬件ID模式
    let patterns = [
        r"PCI\\VEN_([0-9A-Fa-f]{4})&DEV_([0-9A-Fa-f]{4})",
        r"USB\\VID_([0-9A-Fa-f]{4})&PID_([0-9A-Fa-f]{4})",
        r"ACPI\\([A-Z0-9]+)",
    ];

    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(inf_content) {
                if let Some(full_match) = cap.get(0) {
                    hardware_ids.push(HardwareId::parse(full_match.as_str()));
                }
            }
        }
    }

    hardware_ids
}

/// 标准化硬件ID（转换为大写，去除空格）
pub fn normalize_hardware_id(id: &str) -> String {
    id.to_uppercase()
        .replace(" ", "")
        .replace("\t", "")
}

/// 验证硬件ID格式是否有效
pub fn validate_hardware_id_format(id: &str) -> bool {
    let normalized = normalize_hardware_id(id);
    
    // 检查常见格式
    let valid_prefixes = ["PCI\\", "USB\\", "ACPI\\", "HDAUDIO\\", "ROOT\\", "HID\\", "BTH\\"];
    
    valid_prefixes.iter().any(|prefix| normalized.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pci_hardware_id() {
        let id = parse_hardware_id("PCI\\VEN_10DE&DEV_1C03&SUBSYS_12341234&REV_A1");
        assert_eq!(id.vendor_id, Some("10DE".to_string()));
        assert_eq!(id.device_id, Some("1C03".to_string()));
        assert_eq!(id.subsys_id, Some("12341234".to_string()));
        assert_eq!(id.revision, Some("A1".to_string()));
    }

    #[test]
    fn test_get_vendor_name() {
        let id = parse_hardware_id("PCI\\VEN_10DE&DEV_1C03");
        let vendor = get_vendor_from_hardware_id(&id);
        assert_eq!(vendor, Some("NVIDIA".to_string()));
    }

    #[test]
    fn test_match_score() {
        let id1 = parse_hardware_id("PCI\\VEN_10DE&DEV_1C03");
        let id2 = parse_hardware_id("PCI\\VEN_10DE&DEV_1C03");
        let score = calculate_match_score(&id1, &id2);
        assert!(score > 0);
    }
}
