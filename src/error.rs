use std::fmt;

#[derive(Debug)]
pub enum HamsterError {
    ScanError(String),
    BackupError(String),
    RestoreError(String),
    UpdateError(String),
    SignatureError(String),
    NetworkError(String),
    IoError(String),
    Unknown(String),
}

impl fmt::Display for HamsterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HamsterError::ScanError(msg) => write!(f, "硬件扫描错误: {}", msg),
            HamsterError::BackupError(msg) => write!(f, "驱动备份错误: {}", msg),
            HamsterError::RestoreError(msg) => write!(f, "驱动恢复错误: {}", msg),
            HamsterError::UpdateError(msg) => write!(f, "驱动更新错误: {}", msg),
            HamsterError::SignatureError(msg) => write!(f, "驱动签名错误: {}", msg),
            HamsterError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            HamsterError::IoError(msg) => write!(f, "IO错误: {}", msg),
            HamsterError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for HamsterError {}

// 为std::io::Error实现From trait，方便转换
impl From<std::io::Error> for HamsterError {
    fn from(error: std::io::Error) -> Self {
        HamsterError::IoError(error.to_string())
    }
}

// 为Box<dyn std::error::Error>实现From trait，方便转换
impl From<Box<dyn std::error::Error>> for HamsterError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        HamsterError::Unknown(error.to_string())
    }
}
