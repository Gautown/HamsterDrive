//! 驱动缓存模型
//!
//! 定义驱动缓存数据模型和相关操作

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::driver_types::{DriverInfo, DriverVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverCacheModel {
    pub id: i32,
    pub hardware_id: String,
    pub driver_info: DriverInfo,
    pub url: String,
    pub file_size: u64,
    pub hash: String,
    pub release_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DriverCacheModel {
    pub fn new(hardware_id: String, driver_info: DriverInfo, url: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // 由数据库自动生成
            hardware_id,
            driver_info,
            url,
            file_size: 0,
            hash: String::new(),
            release_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新缓存信息
    pub fn update_cache(&mut self, driver_info: DriverInfo, url: String) {
        self.driver_info = driver_info;
        self.url = url;
        self.updated_at = Utc::now();
    }

    /// 检查缓存是否过期（默认30天）
    pub fn is_expired(&self, days: i64) -> bool {
        let expiry_time = self.updated_at + chrono::Duration::days(days);
        Utc::now() > expiry_time
    }

    /// 获取驱动版本
    pub fn version(&self) -> &DriverVersion {
        &self.driver_info.current_version
    }

    /// 获取驱动名称
    pub fn name(&self) -> &str {
        &self.driver_info.name
    }
}