//! 通用驱动解析器
//!
//! 负责处理不特定于任何厂商的驱动信息

use crate::driver::fetcher::parsers::DriverParser;
use crate::types::driver_types::{DriverInfo, DriverVersion, DriverStatus, DriverType};
use crate::utils::error::Result;
use async_trait::async_trait;

pub struct GenericParser;

impl GenericParser {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DriverParser for GenericParser {
    fn name(&self) -> &str {
        "Generic"
    }

    fn supported_vendor_ids(&self) -> Vec<&str> {
        vec![] // 通用解析器不特定于任何厂商ID
    }

    fn supports(&self, _hardware_id: &str) -> bool {
        // 通用解析器可以处理任何硬件ID
        true
    }

    async fn fetch_driver(&self, hardware_id: &str) -> Result<Option<DriverInfo>> {
        // 通用解析器返回基本的驱动信息
        let mut driver_info = DriverInfo::new("Generic Driver", hardware_id);
        driver_info.current_version = DriverVersion::default();
        driver_info.status = DriverStatus::Unknown;
        driver_info.driver_type = DriverType::Unknown;
        
        Ok(Some(driver_info))
    }

    async fn get_download_url(&self, _driver: &DriverInfo) -> Result<Option<String>> {
        // 通用解析器无法提供特定的下载URL
        Ok(None)
    }
}