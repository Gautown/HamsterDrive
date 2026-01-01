//! 驱动匹配器主类

use crate::types::hardware_types::{DeviceInfo, HardwareId};
use crate::types::driver_types::{DriverInfo, DriverPackage, DriverStatus, DriverVersion};
use crate::utils::error::{HamsterError, Result};
use std::collections::HashMap;

/// 驱动匹配器
pub struct DriverMatcher {
    /// 本地驱动数据库缓存
    local_cache: HashMap<String, Vec<DriverPackage>>,
    /// 匹配阈值
    match_threshold: u32,
}

impl DriverMatcher {
    /// 创建新的驱动匹配器
    pub fn new() -> Self {
        Self {
            local_cache: HashMap::new(),
            match_threshold: 100,
        }
    }

    /// 为设备匹配驱动
    pub async fn match_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        let hardware_id = device.primary_hardware_id()
            .ok_or_else(|| HamsterError::ScanError("设备没有硬件ID".to_string()))?;

        // 首先在本地缓存中查找
        if let Some(driver) = self.find_in_local_cache(hardware_id) {
            return Ok(Some(driver));
        }

        // 如果本地没有，查询云端服务
        // 这里的实际实现将调用云端API
        Ok(None)
    }

    /// 在本地缓存中查找驱动
    fn find_in_local_cache(&self, hardware_id: &HardwareId) -> Option<DriverInfo> {
        if let Some(short_id) = hardware_id.short_id() {
            if let Some(packages) = self.local_cache.get(&short_id) {
                if let Some(package) = packages.first() {
                    return Some(self.package_to_driver_info(package));
                }
            }
        }
        None
    }

    /// 将驱动包转换为驱动信息
    fn package_to_driver_info(&self, package: &DriverPackage) -> DriverInfo {
        DriverInfo {
            name: package.name.clone(),
            device_name: package.name.clone(),
            hardware_id: package.supported_hardware_ids.first()
                .cloned()
                .unwrap_or_default(),
            current_version: DriverVersion::default(),
            latest_version: Some(package.version.clone()),
            download_url: Some(package.download_url.clone()),
            file_size: Some(package.file_size),
            release_date: Some(package.release_date.format("%Y-%m-%d").to_string()),
            release_notes: package.release_notes.clone(),
            status: DriverStatus::Outdated,
            provider: Some(package.vendor.clone()),
            driver_type: crate::types::driver_types::DriverType::Other,
            is_critical: false,
            needs_reboot: package.needs_reboot,
            sha256: Some(package.sha256.clone()),
        }
    }

    /// 批量匹配设备驱动
    pub async fn match_drivers_batch(&self, devices: &[DeviceInfo]) -> Result<Vec<DriverInfo>> {
        let mut results = Vec::new();

        for device in devices {
            if let Ok(Some(driver)) = self.match_driver(device).await {
                results.push(driver);
            }
        }

        Ok(results)
    }

    /// 设置匹配阈值
    pub fn set_match_threshold(&mut self, threshold: u32) {
        self.match_threshold = threshold;
    }

    /// 清除本地缓存
    pub fn clear_cache(&mut self) {
        self.local_cache.clear();
    }

    /// 加载本地驱动数据库
    pub async fn load_local_database(&mut self) -> Result<()> {
        // 实际实现将从SQLite数据库加载驱动信息
        tracing::info!("加载本地驱动数据库...");
        Ok(())
    }
}

impl Default for DriverMatcher {
    fn default() -> Self {
        Self::new()
    }
}
