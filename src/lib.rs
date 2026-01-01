//! # HamsterDrive
//!
//! Windows 驱动管理工具库
//!
//! 本库提供以下核心功能：
//! - 硬件扫描和识别
//! - 驱动程序版本检测
//! - 与厂商服务器比较驱动版本
//! - 驱动程序下载和安装
//! - 驱动备份和恢复
//!
//! ## 模块结构
//!
//! - `core`: 核心控制器模块
//! - `system`: 系统信息采集模块
//! - `hardware`: 硬件扫描模块
//! - `driver`: 驱动相关模块（匹配、获取、安装）
//! - `download`: 下载管理模块
//! - `database`: 数据库模块
//! - `network`: 网络相关模块
//! - `config`: 配置管理模块
//! - `ui`: 用户界面模块
//! - `types`: 全局类型定义
//! - `utils`: 工具函数模块

// 核心模块
pub mod core;
pub mod types;
pub mod utils;

// 系统和硬件模块
pub mod system;
pub mod hardware;

// 驱动相关模块
pub mod driver;

// 下载管理模块
pub mod download;

// 数据库模块
pub mod database;

// 网络模块
pub mod network;

// 配置模块
pub mod config;

// UI模块
pub mod ui;

// 导出常用类型和函数
pub use types::{
    hardware_types::{DeviceInfo, DeviceClass, HardwareId},
    driver_types::{DriverInfo, DriverStatus, DriverVersion},
    system_types::OSInfo,
};

pub use utils::error::{HamsterError, Result};
pub use core::controller::DriverUpdaterCore;

/// 库版本号
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 初始化日志系统
pub fn init_logging() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .try_init()
        .map_err(|e| HamsterError::InitError(format!("日志初始化失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
