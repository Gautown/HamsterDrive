//! 进度条组件
//!
//! 用于显示任务进度的UI组件

use crate::utils::error::Result;

pub struct ProgressBar {
    pub current: u64,
    pub total: u64,
    pub message: String,
    pub is_indeterminate: bool,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            current: 0,
            total,
            message: String::new(),
            is_indeterminate: total == 0, // 当total为0时，使用不确定进度条
        }
    }

    /// 创建不确定进度条（用于未知总任务量的情况）
    pub fn new_indeterminate() -> Self {
        Self {
            current: 0,
            total: 0,
            message: String::new(),
            is_indeterminate: true,
        }
    }

    /// 更新进度
    pub fn update(&mut self, current: u64) -> Result<()> {
        if !self.is_indeterminate {
            self.current = current.min(self.total);
        } else {
            self.current = current;
        }
        Ok(())
    }

    /// 设置消息
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// 增加进度
    pub fn increment(&mut self, amount: u64) -> Result<()> {
        if !self.is_indeterminate {
            self.current = (self.current + amount).min(self.total);
        } else {
            self.current += amount;
        }
        Ok(())
    }

    /// 获取进度百分比
    pub fn get_percentage(&self) -> f64 {
        if self.is_indeterminate || self.total == 0 {
            0.0 // 不确定进度条不显示百分比
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }

    /// 检查是否完成
    pub fn is_complete(&self) -> bool {
        !self.is_indeterminate && self.current >= self.total
    }

    /// 重置进度条
    pub fn reset(&mut self, total: Option<u64>) {
        self.current = 0;
        if let Some(new_total) = total {
            self.total = new_total;
            self.is_indeterminate = new_total == 0;
        }
    }

    /// 获取当前进度值
    pub fn get_current(&self) -> u64 {
        self.current
    }

    /// 获取总进度值
    pub fn get_total(&self) -> u64 {
        self.total
    }

    /// 检查是否为不确定进度条
    pub fn is_indeterminate(&self) -> bool {
        self.is_indeterminate
    }
}