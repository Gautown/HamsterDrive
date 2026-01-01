//! 驱动卡片组件
//!
//! 用于显示驱动信息的UI组件

use crate::types::driver_types::{DriverInfo, DriverStatus};
use crate::utils::error::Result;

pub struct DriverCard {
    pub driver_info: DriverInfo,
    pub status: DriverStatus,
    pub is_selected: bool,
    pub can_update: bool,
}

impl DriverCard {
    pub fn new(driver_info: DriverInfo) -> Self {
        let status = driver_info.status.clone();
        let can_update = matches!(status, DriverStatus::Outdated);

        Self {
            driver_info,
            status,
            is_selected: false,
            can_update,
        }
    }

    /// 获取驱动名称
    pub fn get_name(&self) -> &str {
        &self.driver_info.name
    }

    /// 获取驱动版本
    pub fn get_version(&self) -> String {
        self.driver_info.current_version.to_string()
    }

    /// 获取驱动供应商
    pub fn get_vendor(&self) -> &str {
        self.driver_info.provider.as_ref().map(|s| s.as_str()).unwrap_or("")
    }

    /// 获取驱动状态
    pub fn get_status(&self) -> &DriverStatus {
        &self.status
    }

    /// 检查是否可更新
    pub fn can_update(&self) -> bool {
        self.can_update
    }

    /// 检查是否被选中
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    /// 选择/取消选择驱动
    pub fn toggle_selection(&mut self) {
        self.is_selected = !self.is_selected;
    }

    /// 设置选中状态
    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    /// 更新驱动信息
    pub fn update_driver_info(&mut self, new_info: DriverInfo) {
        self.driver_info = new_info;
        self.status = self.driver_info.status.clone();
        self.can_update = matches!(self.status, DriverStatus::Outdated);
    }

    /// 获取驱动描述
    pub fn get_description(&self) -> &str {
        self.driver_info.release_notes.as_ref().map(|s| s.as_str()).unwrap_or("")
    }

    /// 获取驱动文件路径
    pub fn get_file_path(&self) -> &str {
        ""
    }

    /// 获取驱动发布日期
    pub fn get_release_date(&self) -> Option<&String> {
        self.driver_info.release_date.as_ref()
    }

    /// 检查驱动是否已安装
    pub fn is_installed(&self) -> bool {
        !matches!(self.status, DriverStatus::NotInstalled)
    }
}