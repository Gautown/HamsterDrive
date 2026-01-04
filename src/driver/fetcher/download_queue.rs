//! 下载队列管理器
//!
//! 负责管理驱动下载队列

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::driver_types::DriverInfo;
use crate::utils::error::{HamsterError, Result};

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub driver_info: DriverInfo,
    pub download_url: String,
    pub file_path: String,
    pub status: DownloadStatus,
    pub progress: f64, // 0.0 to 100.0
    pub created_at: std::time::SystemTime,
}

pub struct DownloadQueue {
    tasks: Arc<Mutex<HashMap<String, DownloadTask>>>,
    max_concurrent_downloads: usize,
    active_downloads: Arc<Mutex<usize>>,
}

impl DownloadQueue {
    pub fn new(max_concurrent_downloads: usize) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_downloads,
            active_downloads: Arc::new(Mutex::new(0)),
        }
    }

    /// 添加下载任务
    pub async fn add_task(&self, driver_info: DriverInfo, download_url: String, file_path: String) -> Result<String> {
        let task_id = self.generate_task_id(&driver_info);
        
        let task = DownloadTask {
            id: task_id.clone(),
            driver_info,
            download_url,
            file_path,
            status: DownloadStatus::Queued,
            progress: 0.0,
            created_at: std::time::SystemTime::now(),
        };

        let mut tasks = self.tasks.lock().await;
        tasks.insert(task_id.clone(), task);
        drop(tasks);

        Ok(task_id)
    }

    /// 开始下载任务
    pub async fn start_task(&self, task_id: &str) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get_mut(task_id) {
            if matches!(task.status, DownloadStatus::Queued) {
                let mut active_count = self.active_downloads.lock().await;
                if *active_count < self.max_concurrent_downloads {
                    task.status = DownloadStatus::Downloading;
                    *active_count += 1;
                    Ok(())
                } else {
                    Err(HamsterError::DownloadError("达到最大并发下载数".to_string()))
                }
            } else {
                Err(HamsterError::DownloadError("任务状态不允许开始下载".to_string()))
            }
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 更新下载进度
    pub async fn update_progress(&self, task_id: &str, progress: f64) -> Result<()> {
        let tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get(task_id) {
            let mut _task = task.clone();
            drop(tasks);
            
            let mut tasks = self.tasks.lock().await;
            if let Some(existing_task) = tasks.get_mut(task_id) {
                existing_task.progress = progress;
            }
            Ok(())
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 完成下载任务
    pub async fn complete_task(&self, task_id: &str, success: bool) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = if success {
                DownloadStatus::Completed
            } else {
                DownloadStatus::Failed
            };
            
            // 减少活跃下载数
            let mut active_count = self.active_downloads.lock().await;
            if *active_count > 0 {
                *active_count -= 1;
            }
            Ok(())
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 获取任务状态
    pub async fn get_task_status(&self, task_id: &str) -> Result<DownloadStatus> {
        let tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get(task_id) {
            Ok(task.status.clone())
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 获取任务进度
    pub async fn get_task_progress(&self, task_id: &str) -> Result<f64> {
        let tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get(task_id) {
            Ok(task.progress)
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 获取所有任务
    pub async fn get_all_tasks(&self) -> Result<Vec<DownloadTask>> {
        let tasks = self.tasks.lock().await;
        Ok(tasks.values().cloned().collect())
    }

    /// 获取特定状态的任务
    pub async fn get_tasks_by_status(&self, status: &DownloadStatus) -> Result<Vec<DownloadTask>> {
        let tasks = self.tasks.lock().await;
        Ok(tasks.values()
            .filter(|task| matches!(task.status, ref s if std::mem::discriminant(s) == std::mem::discriminant(status)))
            .cloned()
            .collect())
    }

    /// 取消任务
    pub async fn cancel_task(&self, task_id: &str) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = DownloadStatus::Cancelled;
            Ok(())
        } else {
            Err(HamsterError::DownloadError("任务不存在".to_string()))
        }
    }

    /// 生成任务ID
    fn generate_task_id(&self, driver_info: &DriverInfo) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        driver_info.name.hash(&mut hasher);
        driver_info.current_version.to_string().hash(&mut hasher);
        driver_info.hardware_id.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }

    /// 获取活跃下载数量
    pub async fn get_active_download_count(&self) -> usize {
        let active_count = self.active_downloads.lock().await;
        *active_count
    }

    /// 获取队列大小
    pub async fn get_queue_size(&self) -> usize {
        let tasks = self.tasks.lock().await;
        tasks.len()
    }
}