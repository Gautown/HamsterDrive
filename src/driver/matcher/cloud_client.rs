//! 云端匹配服务客户端
//!
//! 负责与云端服务通信进行驱动匹配

use crate::network::ApiClient;
use crate::types::hardware_types::HardwareId;
use crate::types::driver_types::DriverInfo;
use crate::utils::error::{HamsterError, Result};


pub struct CloudClient {
    api_client: ApiClient,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CloudMatchRequest {
    pub hardware_id: String,
    pub device_name: String,
    pub current_driver_version: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CloudMatchResponse {
    pub hardware_id: String,
    pub matched_drivers: Vec<DriverInfo>,
    pub confidence: f64, // 匹配置信度 (0.0-1.0)
    pub source: String,  // 匹配来源
}

impl CloudClient {
    pub fn new(api_client: ApiClient) -> Self {
        Self {
            api_client,
        }
    }

    /// 与云端服务匹配驱动
    pub async fn match_driver(&self, hardware_id: &HardwareId, device_name: &str) -> Result<Option<CloudMatchResponse>> {
        if !self.api_client.health_check().await? {
            return Err(HamsterError::NetworkError("云端服务不可用".to_string()));
        }

        let request = CloudMatchRequest {
            hardware_id: hardware_id.full_id.clone(),
            device_name: device_name.to_string(),
            current_driver_version: None, // TODO: 获取当前驱动版本
        };

        // 使用 API 客户端发送请求
        let query = crate::network::api_client::DriverQuery {
            hardware_id: request.hardware_id,
            device_name: request.device_name,
            current_version: request.current_driver_version,
        };

        match self.api_client.query_drivers(&query).await {
            Ok(response) => {
                if response.available_drivers.is_empty() {
                    Ok(None)
                } else {
                    let cloud_response = CloudMatchResponse {
                        hardware_id: response.hardware_id,
                        matched_drivers: response.available_drivers,
                        confidence: 0.9, // 假设高置信度
                        source: "Cloud Database".to_string(),
                    };
                    Ok(Some(cloud_response))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// 批量匹配驱动
    pub async fn batch_match_drivers(&self, requests: &[CloudMatchRequest]) -> Result<Vec<CloudMatchResponse>> {
        let mut results = Vec::new();

        for request in requests {
            if let Some(response) = self.match_driver(&HardwareId::parse(&request.hardware_id), &request.device_name).await? {
                results.push(response);
            }
        }

        Ok(results)
    }

    /// 检查云端服务是否可用
    pub async fn health_check(&self) -> Result<bool> {
        self.api_client.health_check().await
    }

    /// 上传硬件信息到云端
    pub async fn upload_hardware_info(&self, hardware_ids: &[HardwareId]) -> Result<()> {
        // 将 HardwareId 转换为 DeviceInfo 格式
        let device_infos: Vec<crate::types::hardware_types::DeviceInfo> = hardware_ids
            .iter()
            .map(|hw_id| crate::types::hardware_types::DeviceInfo {
                instance_id: hw_id.full_id.clone(),
                name: hw_id.full_id.clone(), // 使用硬件ID作为名称
                description: String::new(),
                device_class: crate::types::hardware_types::DeviceClass::Other("Unknown".to_string()),
                hardware_ids: vec![hw_id.clone()],
                compatible_ids: vec![],
                vendor_name: None,
                driver_version: None,
                driver_date: None,
                driver_provider: None,
                inf_name: None,
                status: crate::types::hardware_types::DeviceStatus::Unknown,
                problem_code: None,
                has_problem: false,
            })
            .collect();

        self.api_client.upload_hardware_info(&device_infos).await
    }
}