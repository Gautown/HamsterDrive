use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::matcher::scraper::HardwareScraper;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareInfo {
    pub hardware_id: String,
    pub device_name: String,
    pub manufacturer: String,
    pub device_class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriverInfo {
    pub driver_id: String,
    pub hardware_id: String,
    pub driver_name: String,
    pub driver_version: String,
    pub driver_url: String,
    pub manufacturer: String,
    pub release_date: String,
    pub file_size: u64,
    pub checksum: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchResult {
    pub hardware_info: HardwareInfo,
    pub matched_driver: Option<DriverInfo>,
    pub confidence: f32, // 匹配置信度 (0.0 - 1.0)
    pub reason: String,  // 匹配原因
}

pub struct DriverMatcher {
    scraper: HardwareScraper,
}

impl DriverMatcher {
    pub async fn new(_db_path: &str) -> Result<Self> {
        // 不再使用数据库，直接返回带爬虫的实例
        Ok(DriverMatcher { 
            scraper: HardwareScraper::new(),
        })
    }



    pub async fn match_driver(&self, hw_info: &HardwareInfo) -> Result<MatchResult> {
        // 直接从硬件厂商官网爬取驱动信息
        if let Some(driver_info) = self.scraper.search_generic_driver(&hw_info.hardware_id).await? {
            // 将HardwareDriverInfo转换为DriverInfo
            let driver = DriverInfo {
                driver_id: format!("{}-{}", driver_info.hardware_id, driver_info.driver_version),
                hardware_id: driver_info.hardware_id,
                driver_name: driver_info.driver_name,
                driver_version: driver_info.driver_version,
                driver_url: driver_info.driver_url,
                manufacturer: driver_info.manufacturer,
                release_date: driver_info.release_date,
                file_size: 0, // 从网页可能无法直接获取精确大小
                checksum: driver_info.checksum,
            };
            
            return Ok(MatchResult {
                hardware_info: hw_info.clone(),
                matched_driver: Some(driver),
                confidence: 0.9, // 爬取到的驱动置信度较高
                reason: "从硬件厂商官网获取".to_string(),
            });
        }

        // 没有找到匹配的驱动
        Ok(MatchResult {
            hardware_info: hw_info.clone(),
            matched_driver: None,
            confidence: 0.0,
            reason: "未找到匹配的驱动".to_string(),
        })
    }

    // 以下方法不再使用数据库，而是直接通过爬虫获取信息
    pub async fn add_hardware_info(&self, _hw_info: &HardwareInfo) -> Result<()> {
        // 不再存储到数据库，直接返回成功
        Ok(())
    }

    pub async fn add_driver_info(&self, _driver_info: &DriverInfo) -> Result<()> {
        // 不再存储到数据库，直接返回成功
        Ok(())
    }

    pub async fn get_latest_driver_for_hardware(&self, hardware_id: &str) -> Result<Option<DriverInfo>> {
        // 通过爬虫获取最新的驱动信息
        if let Some(driver_info) = self.scraper.search_generic_driver(hardware_id).await? {
            Ok(Some(DriverInfo {
                driver_id: format!("{}-{}", driver_info.hardware_id, driver_info.driver_version),
                hardware_id: driver_info.hardware_id,
                driver_name: driver_info.driver_name,
                driver_version: driver_info.driver_version,
                driver_url: driver_info.driver_url,
                manufacturer: driver_info.manufacturer,
                release_date: driver_info.release_date,
                file_size: 0, // 从网页可能无法直接获取精确大小
                checksum: driver_info.checksum,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn search_drivers_by_name(&self, _driver_name: &str) -> Result<Vec<DriverInfo>> {
        // 暂时返回空列表，因为按名称搜索需要更复杂的爬虫实现
        Ok(Vec::new())
    }
}