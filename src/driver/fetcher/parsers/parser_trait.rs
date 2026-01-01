//! 驱动解析器Trait定义

use crate::types::driver_types::DriverInfo;
use crate::utils::error::Result;
use async_trait::async_trait;

/// 驱动解析器Trait
#[async_trait]
pub trait DriverParser: Send + Sync {
    /// 获取解析器名称
    fn name(&self) -> &str;

    /// 获取支持的厂商ID列表
    fn supported_vendor_ids(&self) -> Vec<&str>;

    /// 检查是否支持指定的硬件ID
    fn supports(&self, hardware_id: &str) -> bool;

    /// 解析并获取驱动信息
    async fn fetch_driver(&self, hardware_id: &str) -> Result<Option<DriverInfo>>;

    /// 获取下载URL
    async fn get_download_url(&self, driver: &DriverInfo) -> Result<Option<String>>;

    /// 获取厂商名称
    fn get_vendor(&self) -> String {
        self.name().to_string()
    }
}
