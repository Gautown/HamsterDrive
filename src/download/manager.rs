//! 下载管理器主类
use crate::utils::error::Result;
use std::path::Path;

pub struct DownloadManager;

impl DownloadManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn download_file(&self, url: &str, dest_path: &Path) -> Result<()> {
        // 使用reqwest下载文件
        let response = reqwest::get(url).await?;
        
        if !response.status().is_success() {
            return Err(crate::utils::error::HamsterError::DownloadError(
                format!("下载失败: HTTP {}", response.status())
            ));
        }

        let content = response.bytes().await?;
        tokio::fs::write(dest_path, content).await?;
        
        Ok(())
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
