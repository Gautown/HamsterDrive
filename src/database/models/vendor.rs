//! 厂商模型
//!
//! 定义厂商数据模型和相关操作

use serde::{Deserialize, Serialize};
use crate::types::hardware_types::HardwareId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorModel {
    pub id: i32,
    pub name: String,
    pub website: String,
    pub api_endpoint: Option<String>,
    pub supported_devices: Vec<String>, // 支持的硬件ID模式
}

impl VendorModel {
    pub fn new(name: String, website: String, api_endpoint: Option<String>) -> Self {
        Self {
            id: 0, // 由数据库自动生成
            name,
            website,
            api_endpoint,
            supported_devices: Vec::new(),
        }
    }

    /// 检查是否支持特定硬件
    pub fn supports_hardware(&self, hardware_id: &HardwareId) -> bool {
        self.supported_devices.iter().any(|pattern| {
            // 简单的模式匹配，检查硬件ID是否匹配厂商支持的模式
            hardware_id.full_id.contains(pattern)
        })
    }

    /// 添加支持的硬件模式
    pub fn add_supported_device(&mut self, pattern: String) {
        if !self.supported_devices.contains(&pattern) {
            self.supported_devices.push(pattern);
        }
    }
}