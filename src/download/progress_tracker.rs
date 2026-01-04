//! 进度跟踪器
//!
//! 负责跟踪下载进度的组件

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::utils::error::{HamsterError, Result};

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub speed: u64, // 字节/秒
    pub percentage: f64,
    pub status: DownloadStatus,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

pub struct ProgressTracker {
    progresses: Arc<Mutex<HashMap<String, DownloadProgress>>>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            progresses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 开始跟踪下载
    pub fn start_tracking(&self, download_id: String, total_size: u64) -> Result<()> {
        let mut progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        progresses.insert(download_id, DownloadProgress {
            downloaded: 0,
            total: total_size,
            speed: 0,
            percentage: 0.0,
            status: DownloadStatus::Downloading,
            message: "开始下载".to_string(),
        });

        Ok(())
    }

    /// 更新下载进度
    pub fn update_progress(&self, download_id: &str, downloaded: u64) -> Result<()> {
        let mut progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        if let Some(progress) = progresses.get_mut(download_id) {
            let old_downloaded = progress.downloaded;
            progress.downloaded = downloaded.min(progress.total);
            progress.percentage = if progress.total > 0 {
                (progress.downloaded as f64 / progress.total as f64) * 100.0
            } else {
                0.0
            };
            
            // 简单的速度计算（实际应用中可能需要更复杂的速度计算）
            let time_elapsed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            if time_elapsed > 0 {
                progress.speed = (downloaded - old_downloaded) / std::cmp::max(time_elapsed, 1);
            }
        }

        Ok(())
    }

    /// 设置下载状态
    pub fn set_status(&self, download_id: &str, status: DownloadStatus, message: String) -> Result<()> {
        let mut progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        if let Some(progress) = progresses.get_mut(download_id) {
            progress.status = status;
            progress.message = message;
        }

        Ok(())
    }

    /// 获取下载进度
    pub fn get_progress(&self, download_id: &str) -> Result<Option<DownloadProgress>> {
        let progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        Ok(progresses.get(download_id).cloned())
    }

    /// 获取所有下载进度
    pub fn get_all_progress(&self) -> Result<Vec<(String, DownloadProgress)>> {
        let progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        Ok(progresses.iter()
            .map(|(id, progress)| (id.clone(), progress.clone()))
            .collect::<Vec<_>>())
    }

    /// 移除下载跟踪
    pub fn remove_tracking(&self, download_id: &str) -> Result<()> {
        let mut progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        progresses.remove(download_id);
        Ok(())
    }

    /// 检查下载是否完成
    pub fn is_complete(&self, download_id: &str) -> Result<bool> {
        let progresses = self.progresses.lock()
            .map_err(|_| HamsterError::Unknown("锁获取失败".to_string()))?;
        
        Ok(if let Some(progress) = progresses.get(download_id) {
            matches!(progress.status, DownloadStatus::Completed)
        } else {
            false
        })
    }
}