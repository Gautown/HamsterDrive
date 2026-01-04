//! 错误类型定义
//!
//! 本模块定义了项目中使用的所有错误类型

use thiserror::Error;


/// 项目统一错误类型
#[derive(Error, Debug)]
pub enum HamsterError {
    /// 硬件扫描错误
    #[error("硬件扫描错误: {0}")]
    ScanError(String),

    /// 驱动备份错误
    #[error("驱动备份错误: {0}")]
    BackupError(String),

    /// 驱动恢复错误
    #[error("驱动恢复错误: {0}")]
    RestoreError(String),

    /// 驱动更新错误
    #[error("驱动更新错误: {0}")]
    UpdateError(String),

    /// 驱动安装错误
    #[error("驱动安装错误: {0}")]
    InstallError(String),

    /// 驱动签名错误
    #[error("驱动签名错误: {0}")]
    SignatureError(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// IO错误
    #[error("IO错误: {0}")]
    IoError(String),

    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 解析错误
    #[error("解析错误: {0}")]
    ParseError(String),

    /// 权限错误
    #[error("权限错误: {0}")]
    PermissionError(String),

    /// 初始化错误
    #[error("初始化错误: {0}")]
    InitError(String),

    /// 下载错误
    #[error("下载错误: {0}")]
    DownloadError(String),

    /// 验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),

    /// 超时错误
    #[error("超时错误: {0}")]
    TimeoutError(String),

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 项目统一Result类型
pub type Result<T> = std::result::Result<T, HamsterError>;

// 实现从 std::io::Error 的转换
impl From<std::io::Error> for HamsterError {
    fn from(error: std::io::Error) -> Self {
        HamsterError::IoError(error.to_string())
    }
}

// 实现从 Box<dyn std::error::Error> 的转换
impl From<Box<dyn std::error::Error>> for HamsterError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        HamsterError::Unknown(error.to_string())
    }
}

// 实现从 reqwest::Error 的转换
impl From<reqwest::Error> for HamsterError {
    fn from(error: reqwest::Error) -> Self {
        HamsterError::NetworkError(error.to_string())
    }
}

// 实现从 serde_json::Error 的转换
impl From<serde_json::Error> for HamsterError {
    fn from(error: serde_json::Error) -> Self {
        HamsterError::ParseError(error.to_string())
    }
}

// 实现从 std::env::VarError 的转换
impl From<std::env::VarError> for HamsterError {
    fn from(error: std::env::VarError) -> Self {
        HamsterError::ConfigError(error.to_string())
    }
}

/// 错误上下文扩展trait
pub trait ErrorContext<T> {
    /// 添加错误上下文
    fn context(self, context: &str) -> Result<T>;
    
    /// 添加懒计算的错误上下文
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T>;
}

impl<T, E: std::error::Error> ErrorContext<T> for std::result::Result<T, E> {
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| HamsterError::Unknown(format!("{}: {}", context, e)))
    }
    
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        self.map_err(|e| HamsterError::Unknown(format!("{}: {}", f(), e)))
    }
}

impl<T> ErrorContext<T> for Option<T> {
    fn context(self, context: &str) -> Result<T> {
        self.ok_or_else(|| HamsterError::Unknown(context.to_string()))
    }
    
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        self.ok_or_else(|| HamsterError::Unknown(f()))
    }
}
