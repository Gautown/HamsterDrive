use serde::{Deserialize, Serialize};

/// 系统架构类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Architecture {
    X86,
    X64,
    ARM,
    ARM64,
}

/// 操作系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub version: String,
    pub edition: String,
    pub build: String,
    pub architecture: Architecture,
    pub is_activated: bool,
    pub activation_status: String,
}

impl OSInfo {
    pub fn new() -> Self {
        OSInfo {
            version: String::new(),
            edition: String::new(),
            build: String::new(),
            architecture: Architecture::X64, // 默认值
            is_activated: false,
            activation_status: String::new(),
        }
    }
}