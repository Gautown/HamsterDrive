//! Intel驱动解析器
//!
//! 负责解析Intel官方网站的驱动信息

use crate::driver::fetcher::parsers::DriverParser;
use crate::types::driver_types::{DriverInfo, DriverVersion, DriverStatus, DriverType};
use crate::utils::error::Result;
use async_trait::async_trait;

pub struct IntelParser;

impl IntelParser {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DriverParser for IntelParser {
    fn name(&self) -> &str {
        "Intel"
    }

    fn supported_vendor_ids(&self) -> Vec<&str> {
        vec!["8086"] // Intel的PCI厂商ID
    }

    fn supports(&self, hardware_id: &str) -> bool {
        hardware_id.to_uppercase().contains("VEN_8086") || 
        hardware_id.to_lowercase().contains("intel")
    }

    async fn fetch_driver(&self, hardware_id: &str) -> Result<Option<DriverInfo>> {
        // 在实际实现中，这将从Intel网站获取驱动信息
        // 这里我们只是模拟实现
        if self.supports(hardware_id) {
            let mut driver_info = DriverInfo::new("Intel Graphics Driver", hardware_id);
            driver_info.current_version = DriverVersion::parse("31.0.101.4146");
            driver_info.latest_version = Some(DriverVersion::parse("31.0.101.4268"));
            driver_info.status = DriverStatus::Outdated;
            driver_info.driver_type = DriverType::Graphics;
            driver_info.provider = Some("Intel Corporation".to_string());
            
            Ok(Some(driver_info))
        } else {
            Ok(None)
        }
    }

    async fn get_download_url(&self, driver: &DriverInfo) -> Result<Option<String>> {
        // 在实际实现中，这将返回Intel驱动的下载URL
        // 这里我们只是模拟实现
        Ok(Some(format!("https://www.intel.com/content/www/us/en/download-center/home.html?driver={}", driver.name)))
    }
}