//! 云端同步
//!
//! 负责与云端服务同步数据

use crate::network::ApiClient;
use crate::types::hardware_types::DeviceInfo;
use crate::types::driver_types::DriverInfo;
use crate::utils::error::{HamsterError, Result};
use crate::network::api_client::DriverQuery;

pub struct CloudSync {
    api_client: ApiClient,
}

#[derive(Debug)]
pub struct SyncConfig {
    pub enabled: bool,
    pub sync_interval: u64, // 同步间隔（秒）
    pub auto_upload: bool,  // 是否自动上传硬件信息
    pub api_key: Option<String>,
}

impl CloudSync {
    pub fn new(api_client: ApiClient) -> Self {
        Self {
            api_client,
        }
    }

    /// 同步硬件信息到云端
    pub async fn sync_hardware_info(&self, devices: &[DeviceInfo]) -> Result<()> {
        if !self.api_client.health_check().await? {
            return Err(HamsterError::NetworkError("云端服务不可用".to_string()));
        }

        self.api_client.upload_hardware_info(devices).await?;
        Ok(())
    }

    /// 从云端获取驱动信息
    pub async fn get_cloud_driver_info(&self, device: &DeviceInfo) -> Result<Option<Vec<DriverInfo>>> {
        if !self.api_client.health_check().await? {
            return Err(HamsterError::NetworkError("云端服务不可用".to_string()));
        }

        let query = crate::network::api_client::DriverQuery {
            hardware_id: device.primary_hardware_id().map_or(device.instance_id.clone(), |h| h.full_id.clone()),
            device_name: device.name.clone(),
            current_version: device.driver_version.clone(),
        };

        match self.api_client.query_drivers(&query).await {
            Ok(response) => {
                if response.available_drivers.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(response.available_drivers))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// 同步配置
    pub async fn sync_config(&self, _config: &SyncConfig) -> Result<()> {
        // TODO: 实现配置同步逻辑
        Ok(())
    }

    /// 检查云端服务状态
    pub async fn check_service_status(&self) -> Result<bool> {
        self.api_client.health_check().await
    }
}