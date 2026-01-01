//! 硬件扫描器主类

use crate::types::hardware_types::{DeviceInfo, DeviceClass, DeviceStatus, HardwareId};
use crate::utils::error::Result;
use std::collections::HashMap;

/// 硬件扫描器
pub struct HardwareScanner {
    /// 缓存的设备列表
    cached_devices: Vec<DeviceInfo>,
    /// 上次扫描时间
    last_scan_time: Option<std::time::Instant>,
    /// 扫描配置
    config: ScannerConfig,
}

/// 扫描器配置
#[derive(Clone)]
pub struct ScannerConfig {
    /// 是否使用WMI扫描
    pub use_wmi: bool,
    /// 是否使用SetupAPI扫描
    pub use_setupapi: bool,
    /// 是否包含隐藏设备
    pub include_hidden: bool,
    /// 要扫描的设备类别
    pub device_classes: Option<Vec<DeviceClass>>,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            use_wmi: true,
            use_setupapi: true,
            include_hidden: false,
            device_classes: None,
        }
    }
}

impl HardwareScanner {
    /// 创建新的硬件扫描器
    pub fn new() -> Self {
        Self {
            cached_devices: Vec::new(),
            last_scan_time: None,
            config: ScannerConfig::default(),
        }
    }

    /// 使用自定义配置创建硬件扫描器
    pub fn with_config(config: ScannerConfig) -> Self {
        Self {
            cached_devices: Vec::new(),
            last_scan_time: None,
            config,
        }
    }

    /// 扫描所有硬件设备
    pub fn scan_all(&mut self) -> Result<Vec<DeviceInfo>> {
        tracing::info!("开始扫描硬件设备...");
        
        let mut all_devices = Vec::new();

        // 使用WMI扫描
        if self.config.use_wmi {
            match crate::hardware::wmi_scanner::scan_devices_wmi() {
                Ok(devices) => {
                    tracing::debug!("WMI扫描发现 {} 个设备", devices.len());
                    all_devices.extend(devices);
                }
                Err(e) => {
                    tracing::warn!("WMI扫描失败: {}", e);
                }
            }
        }

        // 使用SetupAPI扫描
        if self.config.use_setupapi {
            match crate::hardware::setupapi_scanner::scan_devices_setupapi() {
                Ok(devices) => {
                    tracing::debug!("SetupAPI扫描发现 {} 个设备", devices.len());
                    // 合并设备，避免重复
                    for device in devices {
                        if !all_devices.iter().any(|d| d.instance_id == device.instance_id) {
                            all_devices.push(device);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("SetupAPI扫描失败: {}", e);
                }
            }
        }

        // 过滤设备类别
        if let Some(ref classes) = self.config.device_classes {
            all_devices.retain(|d| classes.contains(&d.device_class));
        }

        // 过滤隐藏设备
        if !self.config.include_hidden {
            all_devices.retain(|d| d.status != DeviceStatus::Disabled);
        }

        // 更新缓存
        self.cached_devices = all_devices.clone();
        self.last_scan_time = Some(std::time::Instant::now());

        tracing::info!("硬件扫描完成，共发现 {} 个设备", all_devices.len());
        Ok(all_devices)
    }

    /// 按类别扫描设备
    pub fn scan_by_class(&mut self, device_class: DeviceClass) -> Result<Vec<DeviceInfo>> {
        let all_devices = self.scan_all()?;
        Ok(all_devices.into_iter()
            .filter(|d| d.device_class == device_class)
            .collect())
    }

    /// 扫描显卡设备
    pub fn scan_graphics_devices(&mut self) -> Result<Vec<DeviceInfo>> {
        self.scan_by_class(DeviceClass::Display)
    }

    /// 扫描网卡设备
    pub fn scan_network_devices(&mut self) -> Result<Vec<DeviceInfo>> {
        self.scan_by_class(DeviceClass::Network)
    }

    /// 扫描声卡设备
    pub fn scan_audio_devices(&mut self) -> Result<Vec<DeviceInfo>> {
        self.scan_by_class(DeviceClass::Sound)
    }

    /// 扫描有问题的设备
    pub fn scan_problem_devices(&mut self) -> Result<Vec<DeviceInfo>> {
        let all_devices = self.scan_all()?;
        Ok(all_devices.into_iter()
            .filter(|d| d.has_problem)
            .collect())
    }

    /// 扫描需要驱动更新的设备
    pub fn scan_outdated_devices(&mut self) -> Result<Vec<DeviceInfo>> {
        let _all_devices = self.scan_all()?;
        // 这里可以添加逻辑来检查每个设备是否有更新的驱动
        // 目前先返回空列表
        Ok(Vec::new())
    }

    /// 获取缓存的设备列表
    pub fn get_cached_devices(&self) -> &[DeviceInfo] {
        &self.cached_devices
    }

    /// 通过硬件ID查找设备
    pub fn find_device_by_hardware_id(&self, hardware_id: &str) -> Option<&DeviceInfo> {
        self.cached_devices.iter().find(|d| {
            d.hardware_ids.iter().any(|h| h.full_id.eq_ignore_ascii_case(hardware_id))
        })
    }

    /// 通过实例ID查找设备
    pub fn find_device_by_instance_id(&self, instance_id: &str) -> Option<&DeviceInfo> {
        self.cached_devices.iter().find(|d| d.instance_id.eq_ignore_ascii_case(instance_id))
    }

    /// 获取设备统计信息
    pub fn get_device_statistics(&self) -> HashMap<DeviceClass, usize> {
        let mut stats = HashMap::new();
        for device in &self.cached_devices {
            *stats.entry(device.device_class.clone()).or_insert(0) += 1;
        }
        stats
    }

    /// 检查是否需要重新扫描
    pub fn needs_rescan(&self, max_age_seconds: u64) -> bool {
        match self.last_scan_time {
            Some(time) => time.elapsed().as_secs() > max_age_seconds,
            None => true,
        }
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cached_devices.clear();
        self.last_scan_time = None;
    }
}

impl Default for HardwareScanner {
    fn default() -> Self {
        Self::new()
    }
}
