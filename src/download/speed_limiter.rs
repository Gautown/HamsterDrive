//! 速度限制器
//!
//! 负责限制下载速度的组件

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::utils::error::{HamsterError, Result};

pub struct SpeedLimiter {
    max_speed: Option<u64>, // 最大速度（字节/秒），None表示无限制
    bytes_in_period: Arc<Mutex<(u64, Instant)>>,
    period: Duration,
}

impl SpeedLimiter {
    pub fn new(max_speed: Option<u64>) -> Self {
        Self {
            max_speed,
            bytes_in_period: Arc::new(Mutex::new((0, Instant::now()))),
            period: Duration::from_millis(1000), // 1秒为一个周期
        }
    }

    /// 检查是否需要限速
    pub fn should_limit(&self, bytes_to_download: u64) -> Result<bool> {
        if let Some(max_speed) = self.max_speed {
            let mut data = self.bytes_in_period.lock()
                .map_err(|_| HamsterError::InitError("锁获取失败".to_string()))?;
            
            let now = Instant::now();
            if now.duration_since(data.1) > self.period {
                // 重置周期
                data.0 = 0;
                data.1 = now;
            }

            // 计算在当前周期内下载bytes_to_download后总字节数
            let total_bytes = data.0 + bytes_to_download;
            
            // 计算当前周期内允许的最大字节数
            let max_bytes_in_period = max_speed; // 每秒最大字节数
            
            Ok(total_bytes > max_bytes_in_period)
        } else {
            // 没有限制
            Ok(false)
        }
    }

    /// 记录已下载的字节数
    pub fn record_download(&self, bytes: u64) -> Result<()> {
        let mut data = self.bytes_in_period.lock()
            .map_err(|_| HamsterError::InitError("锁获取失败".to_string()))?;
        
        let now = Instant::now();
        if now.duration_since(data.1) > self.period {
            // 重置周期
            data.0 = 0;
            data.1 = now;
        }

        data.0 += bytes;
        Ok(())
    }

    /// 等待直到可以继续下载（如果需要限速）
    pub fn wait_if_needed(&self, bytes_to_download: u64) -> Result<()> {
        if self.should_limit(bytes_to_download)? {
            // 简单的等待策略：如果超出速度限制，就等待一个周期
            let mut data = self.bytes_in_period.lock()
                .map_err(|_| HamsterError::InitError("锁获取失败".to_string()))?;
            
            let now = Instant::now();
            if now.duration_since(data.1) < self.period {
                // 等待当前周期结束
                let remaining = self.period - now.duration_since(data.1);
                std::thread::sleep(remaining);
            }
            
            // 重置周期
            data.0 = 0;
            data.1 = Instant::now();
        }
        
        Ok(())
    }

    /// 设置最大速度
    pub fn set_max_speed(&mut self, max_speed: Option<u64>) {
        self.max_speed = max_speed;
    }

    /// 获取当前最大速度
    pub fn get_max_speed(&self) -> Option<u64> {
        self.max_speed
    }

    /// 重置计数器
    pub fn reset(&self) -> Result<()> {
        let mut data = self.bytes_in_period.lock()
            .map_err(|_| HamsterError::InitError("锁获取失败".to_string()))?;
        data.0 = 0;
        data.1 = Instant::now();
        Ok(())
    }
}