//! 驱动获取器主类

use crate::types::hardware_types::DeviceInfo;
use crate::types::driver_types::{DriverInfo, DriverStatus};
use crate::utils::error::{HamsterError, Result};
use async_trait::async_trait;

/// 驱动获取器
pub struct DriverFetcher {
    /// HTTP客户端
    client: reqwest::Client,
    /// 缓存管理器
    cache: std::sync::Arc<tokio::sync::Mutex<super::cache_manager::CacheManager>>,
}

impl DriverFetcher {
    /// 创建新的驱动获取器
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| HamsterError::NetworkError(format!("创建HTTP客户端失败: {}", e)))?;

        let cache = std::sync::Arc::new(tokio::sync::Mutex::new(super::cache_manager::CacheManager::new()?));
        
        Ok(Self {
            client,
            cache,
        })
    }

    /// 获取设备的最新驱动信息
    pub async fn fetch_latest_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        // 从缓存获取
        let cached = self.cache.lock().await.get_cached_driver(&device.instance_id)?;
        if let Some(cached) = cached {
            return Ok(Some(cached));
        }

        // 根据厂商ID选择解析器
        let vendor_id = device.vendor_id().unwrap_or("");
        
        let driver_info = match vendor_id.to_uppercase().as_str() {
            "10DE" => self.fetch_nvidia_driver(device).await?,
            "1002" => self.fetch_amd_driver(device).await?,
            "8086" => self.fetch_intel_driver(device).await?,
            "10EC" => self.fetch_realtek_driver(device).await?,
            _ => self.fetch_generic_driver(device).await?,
        };

        // 缓存结果
        if let Some(ref driver) = driver_info {
            self.cache.lock().await.cache_driver(&device.instance_id, driver)?;
        }

        Ok(driver_info)
    }

    /// 获取NVIDIA驱动
    async fn fetch_nvidia_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        tracing::debug!("获取NVIDIA驱动: {}", device.name);
        // 实际实现将解析NVIDIA官网
        Ok(None)
    }

    /// 获取AMD驱动
    async fn fetch_amd_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        tracing::debug!("获取AMD驱动: {}", device.name);
        // 实际实现将解析AMD官网
        Ok(None)
    }

    /// 获取Intel驱动
    async fn fetch_intel_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        tracing::debug!("获取Intel驱动: {}", device.name);
        // 实际实现将解析Intel官网
        Ok(None)
    }

    /// 获取Realtek驱动
    async fn fetch_realtek_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        tracing::debug!("获取Realtek驱动: {}", device.name);
        // 实际实现将解析Realtek官网
        Ok(None)
    }

    /// 获取通用驱动
    async fn fetch_generic_driver(&self, device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        tracing::debug!("获取通用驱动: {}", device.name);
        // 通用驱动查询逻辑
        Ok(None)
    }

    /// 批量获取驱动
    pub async fn fetch_drivers_batch(&self, devices: &[DeviceInfo]) -> Result<Vec<DriverInfo>> {
        let mut results = Vec::new();

        for device in devices {
            if let Ok(Some(driver)) = self.fetch_latest_driver(device).await {
                results.push(driver);
            }
        }

        Ok(results)
    }

    /// 清除缓存
    pub async fn clear_cache(&self) -> Result<()> {
        self.cache.lock().await.clear()
    }
}

impl Default for DriverFetcher {
    fn default() -> Self {
        Self::new().expect("创建DriverFetcher失败")
    }
}
