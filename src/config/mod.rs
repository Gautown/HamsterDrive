//! 配置管理模块
//!
//! 本模块负责应用程序配置的管理

pub mod app_config;
pub mod download_config;
pub mod scanner_config;
pub mod config_manager;

pub use app_config::AppConfig;
pub use config_manager::ConfigManager;
