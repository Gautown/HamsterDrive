//! 日志工具模块

use tracing::{info, warn, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use crate::utils::error::{HamsterError, Result};

/// 日志配置
pub struct LogConfig {
    /// 日志级别
    pub level: LogLevel,
    /// 是否输出到控制台
    pub console_output: bool,
    /// 是否输出到文件
    pub file_output: bool,
    /// 日志文件路径
    pub file_path: Option<String>,
    /// 是否包含时间戳
    pub include_timestamp: bool,
    /// 是否包含调用位置
    pub include_location: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console_output: true,
            file_output: false,
            file_path: None,
            include_timestamp: true,
            include_location: false,
        }
    }
}

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

/// 初始化日志系统
pub fn init_logging(config: &LogConfig) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));

    let subscriber = Registry::default().with(filter);

    if config.console_output {
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_level(true);
        
        let subscriber = subscriber.with(fmt_layer);
        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| HamsterError::InitError(format!("日志初始化失败: {}", e)))?;
    }

    Ok(())
}

/// 记录信息日志
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

/// 记录警告日志
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

/// 记录错误日志
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

/// 记录调试日志
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}
