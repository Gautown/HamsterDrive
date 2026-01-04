//! 驱动匹配评分算法

use crate::types::hardware_types::HardwareId;
use crate::types::driver_types::{DriverMatchScore, DriverVersion};

/// 计算硬件ID匹配分数
pub fn calculate_hardware_id_score(device_id: &HardwareId, driver_id: &HardwareId) -> u32 {
    crate::hardware::identifier::calculate_match_score(device_id, driver_id)
}

/// 计算版本匹配分数
pub fn calculate_version_score(current: &DriverVersion, available: &DriverVersion) -> u32 {
    if available.is_newer_than(current) {
        // 版本越新分数越高
        let version_diff = (available.major - current.major) * 100
            + (available.minor - current.minor) * 10
            + (available.patch - current.patch);
        std::cmp::min(version_diff, 100)
    } else {
        0
    }
}

/// 计算日期匹配分数
pub fn calculate_date_score(_driver_date: &str, days_old: u32) -> u32 {
    // 越新的驱动分数越高
    if days_old < 30 {
        100
    } else if days_old < 90 {
        80
    } else if days_old < 180 {
        60
    } else if days_old < 365 {
        40
    } else {
        20
    }
}

/// 计算厂商匹配分数
pub fn calculate_vendor_score(device_vendor: &str, driver_vendor: &str) -> u32 {
    if device_vendor.eq_ignore_ascii_case(driver_vendor) {
        100
    } else {
        0
    }
}

/// 计算综合匹配分数
pub fn calculate_total_score(
    hardware_id_score: u32,
    version_score: u32,
    date_score: u32,
    vendor_score: u32,
) -> DriverMatchScore {
    DriverMatchScore {
        total_score: hardware_id_score + version_score + date_score + vendor_score,
        hardware_id_score,
        version_score,
        date_score,
        vendor_score,
    }
}
