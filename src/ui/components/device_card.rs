//! 设备卡片组件
//!
//! 用于显示单个硬件设备信息的UI组件

use crate::types::hardware_types::DeviceInfo;
use crate::types::driver_types::DriverStatus;
use crate::utils::error::Result;

pub struct DeviceCard {
    pub device_info: DeviceInfo,
    pub driver_status: DriverStatus,
    pub is_selected: bool,
}

impl DeviceCard {
    pub fn new(device_info: DeviceInfo) -> Self {
        let driver_status = DriverStatus::Unknown;

        Self {
            device_info,
            driver_status,
            is_selected: false,
        }
    }

    /// 获取设备名称
    pub fn get_device_name(&self) -> &str {
        &self.device_info.name
    }

    /// 获取设备ID
    pub fn get_device_id(&self) -> &str {
        self.device_info.primary_hardware_id().map_or("", |h| h.full_id.as_str())
    }

    /// 获取设备类型
    pub fn get_device_type(&self) -> String {
        self.device_info.device_class.to_string()
    }

    /// 获取驱动状态
    pub fn get_driver_status(&self) -> &DriverStatus {
        &self.driver_status
    }

    /// 检查设备是否被选中
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    /// 选择/取消选择设备
    pub fn toggle_selection(&mut self) {
        self.is_selected = !self.is_selected;
    }

    /// 更新设备信息
    pub fn update_device_info(&mut self, new_info: DeviceInfo) {
        self.device_info = new_info;
        self.driver_status = DriverStatus::Unknown;
    }

    /// 检查是否有可用更新
    pub fn has_update_available(&self) -> bool {
        matches!(self.driver_status, DriverStatus::Outdated)
    }

    /// 获取驱动版本信息
    pub fn get_driver_version(&self) -> Option<String> {
        self.device_info.driver_version.clone()
    }

    /// 获取设备描述
    pub fn get_description(&self) -> String {
        self.device_info.description.clone()
    }

    /// 设置选中状态
    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
}