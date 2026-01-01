//! 安装日志模型
//!
//! 定义安装日志数据模型和相关操作

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::driver_types::{DriverInfo, DriverVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationLogModel {
    pub id: i32,
    pub hardware_id: String,
    pub old_driver: Option<DriverInfo>,
    pub new_driver: DriverInfo,
    pub status: InstallationStatus,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub rollback_point: Option<String>, // 系统还原点ID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallationStatus {
    Success,
    Failed,
    Cancelled,
    Pending,
    Rollback,
}

impl InstallationLogModel {
    pub fn new_success(hardware_id: String, old_driver: Option<DriverInfo>, new_driver: DriverInfo) -> Self {
        Self {
            id: 0, // 由数据库自动生成
            hardware_id,
            old_driver,
            new_driver,
            status: InstallationStatus::Success,
            timestamp: Utc::now(),
            notes: None,
            rollback_point: None,
        }
    }

    pub fn new_failed(hardware_id: String, old_driver: Option<DriverInfo>, new_driver: DriverInfo, notes: Option<String>) -> Self {
        Self {
            id: 0,
            hardware_id,
            old_driver,
            new_driver,
            status: InstallationStatus::Failed,
            timestamp: Utc::now(),
            notes,
            rollback_point: None,
        }
    }

    /// 检查安装是否成功
    pub fn is_successful(&self) -> bool {
        matches!(self.status, InstallationStatus::Success)
    }

    /// 获取驱动版本变化
    pub fn version_change(&self) -> Option<(&DriverVersion, &DriverVersion)> {
        match (&self.old_driver, &self.new_driver) {
            (Some(old), new) => Some((&old.current_version, &new.current_version)),
            (None, _new) => None, // 新安装，没有旧版本
        }
    }

    /// 添加备注
    pub fn add_note(&mut self, note: String) {
        match self.notes {
            Some(ref mut notes) => {
                notes.push_str("; ");
                notes.push_str(&note);
            }
            None => {
                self.notes = Some(note);
            }
        }
    }
}