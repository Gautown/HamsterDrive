//! 应用程序状态管理

use std::time::Instant;
use crate::types::{
    DeviceInfo, DriverInfo, SystemSummary,
    ui_types::{CurrentView, ProgressInfo, Notification, UISettings},
};

/// 应用程序状态
#[derive(Clone)]
pub struct AppState {
    /// 系统摘要信息
    pub system_summary: SystemSummary,
    /// 检测到的设备列表
    pub devices: Vec<DeviceInfo>,
    /// 需要更新的驱动列表
    pub outdated_drivers: Vec<DriverInfo>,
    /// 所有已安装的驱动列表
    pub installed_drivers: Vec<DriverInfo>,
    /// 当前视图
    pub current_view: CurrentView,
    /// 是否正在扫描
    pub is_scanning: bool,
    /// 是否正在检查更新
    pub is_checking_updates: bool,
    /// 是否正在下载
    pub is_downloading: bool,
    /// 是否正在安装
    pub is_installing: bool,
    /// 是否正在备份
    pub is_backing_up: bool,
    /// 是否正在恢复
    pub is_restoring: bool,
    /// 上次扫描时间
    pub last_scan_time: Option<Instant>,
    /// 上次更新检查时间
    pub last_update_check_time: Option<Instant>,
    /// 当前进度信息
    pub progress: ProgressInfo,
    /// 通知列表
    pub notifications: Vec<Notification>,
    /// UI设置
    pub ui_settings: UISettings,
    /// 错误消息
    pub error_message: Option<String>,
    /// 是否已初始化
    pub initialized: bool,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new() -> Self {
        Self {
            system_summary: SystemSummary::new(),
            devices: Vec::new(),
            outdated_drivers: Vec::new(),
            installed_drivers: Vec::new(),
            current_view: CurrentView::default(),
            is_scanning: false,
            is_checking_updates: false,
            is_downloading: false,
            is_installing: false,
            is_backing_up: false,
            is_restoring: false,
            last_scan_time: None,
            last_update_check_time: None,
            progress: ProgressInfo::new(),
            notifications: Vec::new(),
            ui_settings: UISettings::default(),
            error_message: None,
            initialized: false,
        }
    }

    /// 检查是否有任何操作正在进行
    pub fn is_busy(&self) -> bool {
        self.is_scanning
            || self.is_checking_updates
            || self.is_downloading
            || self.is_installing
            || self.is_backing_up
            || self.is_restoring
    }

    /// 重置所有操作状态
    pub fn reset_operation_states(&mut self) {
        self.is_scanning = false;
        self.is_checking_updates = false;
        self.is_downloading = false;
        self.is_installing = false;
        self.is_backing_up = false;
        self.is_restoring = false;
        self.progress = ProgressInfo::new();
    }

    /// 添加通知
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    /// 移除已过期的通知
    pub fn cleanup_notifications(&mut self) {
        self.notifications.retain(|n| !n.is_expired());
    }

    /// 设置错误消息
    pub fn set_error(&mut self, message: &str) {
        self.error_message = Some(message.to_string());
        self.add_notification(Notification::error("错误", message));
    }

    /// 清除错误消息
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// 获取需要更新的驱动数量
    pub fn outdated_driver_count(&self) -> usize {
        self.outdated_drivers.len()
    }

    /// 获取设备数量
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// 切换视图
    pub fn switch_view(&mut self, view: CurrentView) {
        self.current_view = view;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// 状态变更事件
#[derive(Debug, Clone)]
pub enum StateEvent {
    /// 扫描开始
    ScanStarted,
    /// 扫描完成
    ScanCompleted(Vec<DeviceInfo>),
    /// 扫描失败
    ScanFailed(String),
    /// 更新检查开始
    UpdateCheckStarted,
    /// 更新检查完成
    UpdateCheckCompleted(Vec<DriverInfo>),
    /// 更新检查失败
    UpdateCheckFailed(String),
    /// 下载开始
    DownloadStarted(String),
    /// 下载进度更新
    DownloadProgress(String, f32),
    /// 下载完成
    DownloadCompleted(String),
    /// 下载失败
    DownloadFailed(String, String),
    /// 安装开始
    InstallStarted(String),
    /// 安装进度更新
    InstallProgress(String, f32),
    /// 安装完成
    InstallCompleted(String),
    /// 安装失败
    InstallFailed(String, String),
    /// 备份开始
    BackupStarted,
    /// 备份完成
    BackupCompleted(String),
    /// 备份失败
    BackupFailed(String),
    /// 恢复开始
    RestoreStarted,
    /// 恢复完成
    RestoreCompleted,
    /// 恢复失败
    RestoreFailed(String),
    /// 视图切换
    ViewChanged(CurrentView),
}

/// 状态事件处理器
pub trait StateEventHandler {
    /// 处理状态事件
    fn handle_event(&mut self, event: StateEvent);
}

impl StateEventHandler for AppState {
    fn handle_event(&mut self, event: StateEvent) {
        match event {
            StateEvent::ScanStarted => {
                self.is_scanning = true;
                self.progress = ProgressInfo::new();
                self.progress.message = "正在扫描硬件...".to_string();
            }
            StateEvent::ScanCompleted(devices) => {
                self.is_scanning = false;
                self.devices = devices;
                self.last_scan_time = Some(Instant::now());
                self.progress.complete("扫描完成");
            }
            StateEvent::ScanFailed(error) => {
                self.is_scanning = false;
                self.progress.fail(&error);
                self.set_error(&error);
            }
            StateEvent::UpdateCheckStarted => {
                self.is_checking_updates = true;
                self.progress = ProgressInfo::new();
                self.progress.message = "正在检查驱动更新...".to_string();
            }
            StateEvent::UpdateCheckCompleted(drivers) => {
                self.is_checking_updates = false;
                self.outdated_drivers = drivers;
                self.last_update_check_time = Some(Instant::now());
                self.progress.complete("更新检查完成");
            }
            StateEvent::UpdateCheckFailed(error) => {
                self.is_checking_updates = false;
                self.progress.fail(&error);
                self.set_error(&error);
            }
            StateEvent::DownloadStarted(name) => {
                self.is_downloading = true;
                self.progress = ProgressInfo::new();
                self.progress.message = format!("正在下载: {}", name);
            }
            StateEvent::DownloadProgress(name, progress) => {
                self.progress.progress = progress;
                self.progress.message = format!("正在下载: {} ({:.0}%)", name, progress * 100.0);
            }
            StateEvent::DownloadCompleted(name) => {
                self.is_downloading = false;
                self.progress.complete(&format!("下载完成: {}", name));
            }
            StateEvent::DownloadFailed(name, error) => {
                self.is_downloading = false;
                self.progress.fail(&format!("下载失败: {} - {}", name, error));
                self.set_error(&format!("下载失败: {}", error));
            }
            StateEvent::InstallStarted(name) => {
                self.is_installing = true;
                self.progress = ProgressInfo::new();
                self.progress.message = format!("正在安装: {}", name);
            }
            StateEvent::InstallProgress(name, progress) => {
                self.progress.progress = progress;
                self.progress.message = format!("正在安装: {} ({:.0}%)", name, progress * 100.0);
            }
            StateEvent::InstallCompleted(name) => {
                self.is_installing = false;
                self.progress.complete(&format!("安装完成: {}", name));
                self.add_notification(Notification::success("安装完成", &name));
            }
            StateEvent::InstallFailed(name, error) => {
                self.is_installing = false;
                self.progress.fail(&format!("安装失败: {} - {}", name, error));
                self.set_error(&format!("安装失败: {}", error));
            }
            StateEvent::BackupStarted => {
                self.is_backing_up = true;
                self.progress = ProgressInfo::new();
                self.progress.message = "正在备份驱动...".to_string();
            }
            StateEvent::BackupCompleted(path) => {
                self.is_backing_up = false;
                self.progress.complete("备份完成");
                self.add_notification(Notification::success("备份完成", &path));
            }
            StateEvent::BackupFailed(error) => {
                self.is_backing_up = false;
                self.progress.fail(&error);
                self.set_error(&error);
            }
            StateEvent::RestoreStarted => {
                self.is_restoring = true;
                self.progress = ProgressInfo::new();
                self.progress.message = "正在恢复驱动...".to_string();
            }
            StateEvent::RestoreCompleted => {
                self.is_restoring = false;
                self.progress.complete("恢复完成");
                self.add_notification(Notification::success("恢复完成", "驱动已成功恢复"));
            }
            StateEvent::RestoreFailed(error) => {
                self.is_restoring = false;
                self.progress.fail(&error);
                self.set_error(&error);
            }
            StateEvent::ViewChanged(view) => {
                self.current_view = view;
            }
        }
    }
}
