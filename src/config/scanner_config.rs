//! 扫描器配置
//!
//! 定义硬件扫描相关的配置参数

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub deep_scan_enabled: bool,           // 是否启用深度扫描
    pub scan_timeout: u64,                // 扫描超时时间（秒）
    pub include_disabled_devices: bool,   // 是否包含禁用的设备
    pub include_system_devices: bool,     // 是否包含系统设备
    pub scan_interval: u64,               // 自动扫描间隔（秒）
    pub cache_scan_results: bool,         // 是否缓存扫描结果
    pub max_scan_depth: u32,              // 最大扫描深度
    pub scan_network_devices: bool,       // 是否扫描网络设备
    pub scan_usb_devices: bool,           // 是否扫描USB设备
    pub scan_bluetooth_devices: bool,     // 是否扫描蓝牙设备
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            deep_scan_enabled: true,
            scan_timeout: 120,
            include_disabled_devices: false,
            include_system_devices: true,
            scan_interval: 3600, // 1小时
            cache_scan_results: true,
            max_scan_depth: 10,
            scan_network_devices: true,
            scan_usb_devices: true,
            scan_bluetooth_devices: true,
        }
    }
}

impl ScannerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// 启用深度扫描
    pub fn with_deep_scan(mut self, enabled: bool) -> Self {
        self.deep_scan_enabled = enabled;
        self
    }

    /// 设置扫描超时时间
    pub fn with_scan_timeout(mut self, timeout: u64) -> Self {
        self.scan_timeout = timeout;
        self
    }

    /// 设置是否包含禁用设备
    pub fn with_include_disabled_devices(mut self, include: bool) -> Self {
        self.include_disabled_devices = include;
        self
    }

    /// 设置是否包含系统设备
    pub fn with_include_system_devices(mut self, include: bool) -> Self {
        self.include_system_devices = include;
        self
    }

    /// 设置自动扫描间隔
    pub fn with_scan_interval(mut self, interval: u64) -> Self {
        self.scan_interval = interval;
        self
    }

    /// 设置是否缓存扫描结果
    pub fn with_cache_scan_results(mut self, cache: bool) -> Self {
        self.cache_scan_results = cache;
        self
    }

    /// 设置最大扫描深度
    pub fn with_max_scan_depth(mut self, depth: u32) -> Self {
        self.max_scan_depth = depth;
        self
    }

    /// 设置是否扫描网络设备
    pub fn with_scan_network_devices(mut self, scan: bool) -> Self {
        self.scan_network_devices = scan;
        self
    }

    /// 设置是否扫描USB设备
    pub fn with_scan_usb_devices(mut self, scan: bool) -> Self {
        self.scan_usb_devices = scan;
        self
    }

    /// 设置是否扫描蓝牙设备
    pub fn with_scan_bluetooth_devices(mut self, scan: bool) -> Self {
        self.scan_bluetooth_devices = scan;
        self
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.scan_timeout == 0 {
            return Err("扫描超时时间不能为0".to_string());
        }

        if self.scan_interval == 0 {
            return Err("扫描间隔不能为0".to_string());
        }

        if self.max_scan_depth == 0 {
            return Err("最大扫描深度不能为0".to_string());
        }

        Ok(())
    }
}