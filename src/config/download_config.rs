//! 下载配置
//!
//! 定义下载相关的配置参数

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub max_concurrent_downloads: usize,
    pub download_speed_limit: Option<u64>, // 速度限制（字节/秒），None表示无限制
    pub download_directory: String,
    pub temp_directory: String,
    pub retry_count: u32,
    pub timeout: u64, // 超时时间（秒）
    pub chunk_size: u64, // 分块下载大小（字节）
    pub verify_checksum: bool,
    pub use_resume: bool, // 是否使用断点续传
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: 3,
            download_speed_limit: None,
            download_directory: "./downloads".to_string(),
            temp_directory: "./temp".to_string(),
            retry_count: 3,
            timeout: 30,
            chunk_size: 1024 * 1024, // 1MB
            verify_checksum: true,
            use_resume: true,
        }
    }
}

impl DownloadConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置最大并发下载数
    pub fn with_max_concurrent_downloads(mut self, max: usize) -> Self {
        self.max_concurrent_downloads = max;
        self
    }

    /// 设置下载速度限制
    pub fn with_speed_limit(mut self, limit: Option<u64>) -> Self {
        self.download_speed_limit = limit;
        self
    }

    /// 设置下载目录
    pub fn with_download_directory(mut self, dir: String) -> Self {
        self.download_directory = dir;
        self
    }

    /// 设置临时目录
    pub fn with_temp_directory(mut self, dir: String) -> Self {
        self.temp_directory = dir;
        self
    }

    /// 设置重试次数
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.max_concurrent_downloads == 0 {
            return Err("最大并发下载数不能为0".to_string());
        }

        if self.retry_count == 0 {
            return Err("重试次数不能为0".to_string());
        }

        if self.timeout == 0 {
            return Err("超时时间不能为0".to_string());
        }

        if self.chunk_size == 0 {
            return Err("分块大小不能为0".to_string());
        }

        Ok(())
    }
}