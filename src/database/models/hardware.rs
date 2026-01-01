//! 硬件映射模型
//!
//! 定义硬件映射数据模型和相关操作

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::hardware_types::HardwareId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareModel {
    pub id: i32,
    pub vendor_id: i32,
    pub hardware_id: HardwareId,
    pub device_name: String,
    pub category: String,
    pub last_updated: DateTime<Utc>,
}

impl HardwareModel {
    pub fn new(vendor_id: i32, hardware_id: HardwareId, device_name: String, category: String) -> Self {
        Self {
            id: 0, // 由数据库自动生成
            vendor_id,
            hardware_id,
            device_name,
            category,
            last_updated: Utc::now(),
        }
    }

    /// 更新最后更新时间
    pub fn update_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }

    /// 检查硬件是否属于特定类别
    pub fn is_category(&self, category: &str) -> bool {
        self.category == category
    }
}