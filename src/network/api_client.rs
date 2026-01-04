//! API客户端
//!
//! 负责与厂商服务器和云端API通信

use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::types::driver_types::{DriverInfo, DriverVersion};
use crate::types::hardware_types::DeviceInfo;
use crate::utils::error::{HamsterError, Result};

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverQuery {
    pub hardware_id: String,
    pub device_name: String,
    pub current_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverResponse {
    pub hardware_id: String,
    pub available_drivers: Vec<DriverInfo>,
    pub latest_version: Option<DriverVersion>,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    /// 查询硬件的可用驱动
    pub async fn query_drivers(&self, query: &DriverQuery) -> Result<DriverResponse> {
        let url = format!("{}/api/drivers/query", self.base_url);
        
        let mut request = self.client.post(&url)
            .json(query);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| HamsterError::NetworkError(format!("API请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(HamsterError::NetworkError(format!(
                "API请求失败，状态码: {}", response.status()
            )));
        }
        
        let driver_response: DriverResponse = response.json().await
            .map_err(|e| HamsterError::NetworkError(format!("解析API响应失败: {}", e)))?;
        
        Ok(driver_response)
    }

    /// 获取驱动下载链接
    pub async fn get_download_url(&self, driver_info: &DriverInfo) -> Result<String> {
        let url = format!("{}/api/drivers/download", self.base_url);
        
        let request_data = serde_json::json!({
            "driver_name": driver_info.name,
            "version": driver_info.current_version.to_string(),
            "hardware_id": driver_info.hardware_id
        });
        
        let mut request = self.client.post(&url)
            .json(&request_data);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| HamsterError::NetworkError(format!("下载URL请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(HamsterError::NetworkError(format!(
                "下载URL请求失败，状态码: {}", response.status()
            )));
        }
        
        let download_response: serde_json::Value = response.json().await
            .map_err(|e| HamsterError::NetworkError(format!("解析下载响应失败: {}", e)))?;
        
        if let Some(download_url) = download_response.get("download_url").and_then(|v| v.as_str()) {
            Ok(download_url.to_string())
        } else {
            Err(HamsterError::NetworkError("API响应中未包含下载URL".to_string()))
        }
    }

    /// 检查API服务是否可用
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/health", self.base_url);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| HamsterError::NetworkError(format!("健康检查请求失败: {}", e)))?;
        
        Ok(response.status().is_success())
    }

    /// 上传硬件信息到云端
    pub async fn upload_hardware_info(&self, devices: &[DeviceInfo]) -> Result<()> {
        let url = format!("{}/api/hardware/upload", self.base_url);
        
        let mut request = self.client.post(&url)
            .json(devices);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| HamsterError::NetworkError(format!("硬件信息上传失败: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(HamsterError::NetworkError(format!(
                "硬件信息上传失败，状态码: {}", response.status()
            )));
        }
        
        Ok(())
    }
}