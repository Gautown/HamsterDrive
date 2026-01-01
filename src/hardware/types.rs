//! 硬件扫描相关类型

use crate::types::hardware_types::{DeviceClass, DeviceStatus};
use serde::{Deserialize, Serialize};

/// 扫描结果
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// 扫描是否成功
    pub success: bool,
    /// 发现的设备数量
    pub device_count: usize,
    /// 扫描耗时（毫秒）
    pub duration_ms: u64,
    /// 错误信息
    pub errors: Vec<String>,
}

/// 设备摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSummary {
    /// 设备名称
    pub name: String,
    /// 设备类别
    pub device_class: String,
    /// 厂商名称
    pub vendor: String,
    /// 驱动版本
    pub driver_version: String,
    /// 设备状态
    pub status: String,
}

/// 扫描进度
#[derive(Debug, Clone)]
pub struct ScanProgress {
    /// 当前步骤
    pub current_step: u32,
    /// 总步骤数
    pub total_steps: u32,
    /// 当前操作描述
    pub description: String,
    /// 进度百分比
    pub percentage: f32,
}

impl ScanProgress {
    pub fn new(total_steps: u32) -> Self {
        Self {
            current_step: 0,
            total_steps,
            description: String::new(),
            percentage: 0.0,
        }
    }

    pub fn update(&mut self, step: u32, description: &str) {
        self.current_step = step;
        self.description = description.to_string();
        if self.total_steps > 0 {
            self.percentage = (step as f32 / self.total_steps as f32) * 100.0;
        }
    }
}

/// 设备过滤条件
#[derive(Debug, Clone, Default)]
pub struct DeviceFilter {
    /// 按设备类别过滤
    pub device_class: Option<DeviceClass>,
    /// 按厂商ID过滤
    pub vendor_id: Option<String>,
    /// 按设备状态过滤
    pub status: Option<DeviceStatus>,
    /// 是否只显示有问题的设备
    pub only_problems: bool,
    /// 是否包含隐藏设备
    pub include_hidden: bool,
    /// 名称搜索关键字
    pub name_filter: Option<String>,
}

impl DeviceFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_class(mut self, class: DeviceClass) -> Self {
        self.device_class = Some(class);
        self
    }

    pub fn with_vendor(mut self, vendor_id: &str) -> Self {
        self.vendor_id = Some(vendor_id.to_string());
        self
    }

    pub fn only_problems(mut self) -> Self {
        self.only_problems = true;
        self
    }

    pub fn include_hidden(mut self) -> Self {
        self.include_hidden = true;
        self
    }

    pub fn with_name_filter(mut self, filter: &str) -> Self {
        self.name_filter = Some(filter.to_string());
        self
    }
}
