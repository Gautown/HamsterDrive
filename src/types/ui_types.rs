//! UI相关类型定义

use serde::{Deserialize, Serialize};
use std::fmt;

/// 当前显示的视图类型
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CurrentView {
    /// 系统信息视图
    #[default]
    SystemInfo,
    /// 驱动扫描视图
    DriverScan,
    /// 驱动更新视图
    DriverUpdate,
    /// 驱动备份视图
    DriverBackup,
    /// 驱动恢复视图
    DriverRestore,
    /// 驱动列表视图
    DriverList,
    /// 设置视图
    Settings,
    /// 关于视图
    About,
}

impl fmt::Display for CurrentView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurrentView::SystemInfo => write!(f, "系统信息"),
            CurrentView::DriverScan => write!(f, "驱动扫描"),
            CurrentView::DriverUpdate => write!(f, "驱动更新"),
            CurrentView::DriverBackup => write!(f, "驱动备份"),
            CurrentView::DriverRestore => write!(f, "驱动恢复"),
            CurrentView::DriverList => write!(f, "驱动列表"),
            CurrentView::Settings => write!(f, "设置"),
            CurrentView::About => write!(f, "关于"),
        }
    }
}

/// 操作状态
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OperationState {
    #[default]
    Idle,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 进度信息
#[derive(Debug, Clone, Default)]
pub struct ProgressInfo {
    /// 当前进度 (0.0 - 1.0)
    pub progress: f32,
    /// 状态消息
    pub message: String,
    /// 当前步骤
    pub current_step: u32,
    /// 总步骤数
    pub total_steps: u32,
    /// 操作状态
    pub state: OperationState,
}

impl ProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }

    /// 更新进度
    pub fn update(&mut self, current: u32, total: u32, message: &str) {
        self.current_step = current;
        self.total_steps = total;
        self.message = message.to_string();
        if total > 0 {
            self.progress = current as f32 / total as f32;
        }
    }

    /// 获取百分比
    pub fn percentage(&self) -> f32 {
        self.progress * 100.0
    }

    /// 设置完成状态
    pub fn complete(&mut self, message: &str) {
        self.progress = 1.0;
        self.message = message.to_string();
        self.state = OperationState::Completed;
    }

    /// 设置失败状态
    pub fn fail(&mut self, message: &str) {
        self.message = message.to_string();
        self.state = OperationState::Failed;
    }
}

/// 通知类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// 通知消息
#[derive(Debug, Clone)]
pub struct Notification {
    /// 通知ID
    pub id: u64,
    /// 通知类型
    pub notification_type: NotificationType,
    /// 标题
    pub title: String,
    /// 消息内容
    pub message: String,
    /// 是否已读
    pub read: bool,
    /// 创建时间
    pub created_at: std::time::Instant,
    /// 持续时间（秒），None表示永久
    pub duration: Option<u64>,
}

impl Notification {
    pub fn new(notification_type: NotificationType, title: &str, message: &str) -> Self {
        Self {
            id: rand_id(),
            notification_type,
            title: title.to_string(),
            message: message.to_string(),
            read: false,
            created_at: std::time::Instant::now(),
            duration: Some(5),
        }
    }

    pub fn info(title: &str, message: &str) -> Self {
        Self::new(NotificationType::Info, title, message)
    }

    pub fn success(title: &str, message: &str) -> Self {
        Self::new(NotificationType::Success, title, message)
    }

    pub fn warning(title: &str, message: &str) -> Self {
        Self::new(NotificationType::Warning, title, message)
    }

    pub fn error(title: &str, message: &str) -> Self {
        Self::new(NotificationType::Error, title, message)
    }

    /// 检查通知是否已过期
    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.created_at.elapsed().as_secs() >= duration
        } else {
            false
        }
    }
}

/// UI主题
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    System,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "浅色"),
            Theme::Dark => write!(f, "深色"),
            Theme::System => write!(f, "跟随系统"),
        }
    }
}

/// UI设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// 主题
    pub theme: Theme,
    /// 字体大小
    pub font_size: f32,
    /// 是否显示系统托盘
    pub show_tray: bool,
    /// 最小化到托盘
    pub minimize_to_tray: bool,
    /// 启动时最小化
    pub start_minimized: bool,
    /// 启动时检查更新
    pub check_updates_on_start: bool,
    /// 语言
    pub language: String,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            font_size: 14.0,
            show_tray: true,
            minimize_to_tray: true,
            start_minimized: false,
            check_updates_on_start: true,
            language: "zh-CN".to_string(),
        }
    }
}

/// 驱动列表项
#[derive(Debug, Clone)]
pub struct DriverListItem {
    /// 驱动名称
    pub name: String,
    /// 设备名称
    pub device_name: String,
    /// 当前版本
    pub current_version: String,
    /// 最新版本
    pub latest_version: String,
    /// 状态
    pub status: String,
    /// 状态颜色
    pub status_color: StatusColor,
    /// 是否选中
    pub selected: bool,
    /// 硬件ID
    pub hardware_id: String,
    /// 下载URL
    pub download_url: Option<String>,
}

/// 状态颜色
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusColor {
    Green,
    Yellow,
    Red,
    Gray,
}

impl StatusColor {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            StatusColor::Green => (0, 200, 0),
            StatusColor::Yellow => (255, 200, 0),
            StatusColor::Red => (255, 0, 0),
            StatusColor::Gray => (128, 128, 128),
        }
    }
}

/// 窗口状态
#[derive(Debug, Clone, Default)]
pub struct WindowState {
    /// 窗口宽度
    pub width: f32,
    /// 窗口高度
    pub height: f32,
    /// 窗口X位置
    pub x: i32,
    /// 窗口Y位置
    pub y: i32,
    /// 是否最大化
    pub maximized: bool,
}

/// 生成随机ID
fn rand_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// 表格列配置
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// 列名
    pub name: String,
    /// 宽度
    pub width: f32,
    /// 是否可排序
    pub sortable: bool,
    /// 是否可见
    pub visible: bool,
}

/// 表格排序方向
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// 表格状态
#[derive(Debug, Clone, Default)]
pub struct TableState {
    /// 排序列索引
    pub sort_column: Option<usize>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
    /// 选中行索引
    pub selected_rows: Vec<usize>,
    /// 滚动位置
    pub scroll_position: f32,
}
