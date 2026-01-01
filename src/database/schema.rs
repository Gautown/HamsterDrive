//! 数据库模式定义
//!
//! 定义数据库表结构和关系

use serde::{Deserialize, Serialize};

/// 厂商信息表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
    pub website: String,
    pub api_endpoint: Option<String>,
}

/// 硬件映射表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMapping {
    pub id: i32,
    pub vendor_id: i32,
    pub hardware_id: String,  // 如 "PCI\\VEN_10DE&DEV_1C82"
    pub device_name: String,
    pub category: String,
    pub last_updated: chrono::NaiveDateTime,
}

/// 驱动缓存表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverCache {
    pub id: i32,
    pub hardware_id: String,
    pub driver_name: String,
    pub version: String,
    pub url: String,
    pub file_size: i64,
    pub hash: String,
    pub release_date: chrono::NaiveDate,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// 安装日志表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationLog {
    pub id: i32,
    pub hardware_id: String,
    pub driver_name: String,
    pub old_version: Option<String>,
    pub new_version: String,
    pub status: String,  // "success", "failed", "cancelled"
    pub timestamp: chrono::NaiveDateTime,
    pub notes: Option<String>,
}

// 使用 Diesel 定义表结构
#[cfg(feature = "database")]
table! {
    vendors (id) {
        id -> Integer,
        name -> Text,
        website -> Text,
        api_endpoint -> Nullable<Text>,
    }
}

#[cfg(feature = "database")]
table! {
    hardware_mappings (id) {
        id -> Integer,
        vendor_id -> Integer,
        hardware_id -> Text,
        device_name -> Text,
        category -> Text,
        last_updated -> Timestamp,
    }
}

#[cfg(feature = "database")]
table! {
    driver_cache (id) {
        id -> Integer,
        hardware_id -> Text,
        driver_name -> Text,
        version -> Text,
        url -> Text,
        file_size -> BigInt,
        hash -> Text,
        release_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[cfg(feature = "database")]
table! {
    installation_logs (id) {
        id -> Integer,
        hardware_id -> Text,
        driver_name -> Text,
        old_version -> Nullable<Text>,
        new_version -> Text,
        status -> Text,
        timestamp -> Timestamp,
        notes -> Nullable<Text>,
    }
}

#[cfg(feature = "database")]
joinable!(hardware_mappings -> vendors (vendor_id));

#[cfg(feature = "database")]
allow_tables_to_appear_in_same_query!(
    vendors,
    hardware_mappings,
    driver_cache,
    installation_logs,
);