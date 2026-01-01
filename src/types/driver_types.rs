//! 驱动相关类型定义

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// 驱动版本
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriverVersion {
    /// 版本字符串
    pub version_string: String,
    /// 主版本号
    pub major: u32,
    /// 次版本号
    pub minor: u32,
    /// 修订号
    pub patch: u32,
    /// 构建号
    pub build: u32,
}

impl DriverVersion {
    /// 从版本字符串解析
    pub fn parse(version_str: &str) -> Self {
        let parts: Vec<u32> = version_str
            .split(|c| c == '.' || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        Self {
            version_string: version_str.to_string(),
            major: parts.first().copied().unwrap_or(0),
            minor: parts.get(1).copied().unwrap_or(0),
            patch: parts.get(2).copied().unwrap_or(0),
            build: parts.get(3).copied().unwrap_or(0),
        }
    }

    /// 比较两个版本，返回是否比另一个新
    pub fn is_newer_than(&self, other: &DriverVersion) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }
        if self.minor != other.minor {
            return self.minor > other.minor;
        }
        if self.patch != other.patch {
            return self.patch > other.patch;
        }
        self.build > other.build
    }
}

impl fmt::Display for DriverVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version_string)
    }
}

impl Default for DriverVersion {
    fn default() -> Self {
        Self {
            version_string: "0.0.0.0".to_string(),
            major: 0,
            minor: 0,
            patch: 0,
            build: 0,
        }
    }
}

impl PartialOrd for DriverVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DriverVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.major.cmp(&other.major) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.minor.cmp(&other.minor) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.patch.cmp(&other.patch) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.build.cmp(&other.build)
    }
}

/// 驱动状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverStatus {
    /// 已是最新
    UpToDate,
    /// 有更新可用
    Outdated,
    /// 未安装
    NotInstalled,
    /// 正在下载
    Downloading,
    /// 正在安装
    Installing,
    /// 安装失败
    InstallFailed,
    /// 需要重启
    NeedsReboot,
    /// 未知状态
    Unknown,
}

impl fmt::Display for DriverStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverStatus::UpToDate => write!(f, "最新"),
            DriverStatus::Outdated => write!(f, "需更新"),
            DriverStatus::NotInstalled => write!(f, "未安装"),
            DriverStatus::Downloading => write!(f, "下载中"),
            DriverStatus::Installing => write!(f, "安装中"),
            DriverStatus::InstallFailed => write!(f, "安装失败"),
            DriverStatus::NeedsReboot => write!(f, "需要重启"),
            DriverStatus::Unknown => write!(f, "未知"),
        }
    }
}

impl Default for DriverStatus {
    fn default() -> Self {
        DriverStatus::Unknown
    }
}

/// 驱动信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverInfo {
    /// 驱动名称
    pub name: String,
    /// 设备名称
    pub device_name: String,
    /// 硬件ID
    pub hardware_id: String,
    /// 当前版本
    pub current_version: DriverVersion,
    /// 最新版本
    pub latest_version: Option<DriverVersion>,
    /// 下载URL
    pub download_url: Option<String>,
    /// 文件大小（字节）
    pub file_size: Option<u64>,
    /// 发布日期
    pub release_date: Option<String>,
    /// 发布说明
    pub release_notes: Option<String>,
    /// 驱动状态
    pub status: DriverStatus,
    /// 驱动提供商
    pub provider: Option<String>,
    /// 驱动类型
    pub driver_type: DriverType,
    /// 是否为关键驱动
    pub is_critical: bool,
    /// 是否需要重启
    pub needs_reboot: bool,
    /// SHA256校验和
    pub sha256: Option<String>,
}

impl DriverInfo {
    /// 创建新的驱动信息
    pub fn new(name: &str, hardware_id: &str) -> Self {
        Self {
            name: name.to_string(),
            device_name: name.to_string(),
            hardware_id: hardware_id.to_string(),
            current_version: DriverVersion::default(),
            latest_version: None,
            download_url: None,
            file_size: None,
            release_date: None,
            release_notes: None,
            status: DriverStatus::Unknown,
            provider: None,
            driver_type: DriverType::Unknown,
            is_critical: false,
            needs_reboot: false,
            sha256: None,
        }
    }

    /// 检查是否有可用更新
    pub fn has_update(&self) -> bool {
        if let Some(ref latest) = self.latest_version {
            latest.is_newer_than(&self.current_version)
        } else {
            false
        }
    }

    /// 格式化文件大小
    pub fn formatted_file_size(&self) -> String {
        match self.file_size {
            Some(size) => format_file_size(size),
            None => "未知".to_string(),
        }
    }
}

impl fmt::Display for DriverInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (当前: {}, 最新: {})",
            self.name,
            self.current_version,
            self.latest_version
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "未知".to_string())
        )
    }
}

/// 驱动类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverType {
    /// 显卡驱动
    Graphics,
    /// 网卡驱动
    Network,
    /// 声卡驱动
    Audio,
    /// 芯片组驱动
    Chipset,
    /// USB驱动
    USB,
    /// 存储驱动
    Storage,
    /// 输入设备驱动
    Input,
    /// BIOS/固件
    Firmware,
    /// 其他驱动
    Other,
    /// 未知类型
    Unknown,
}

impl fmt::Display for DriverType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverType::Graphics => write!(f, "显卡驱动"),
            DriverType::Network => write!(f, "网卡驱动"),
            DriverType::Audio => write!(f, "声卡驱动"),
            DriverType::Chipset => write!(f, "芯片组驱动"),
            DriverType::USB => write!(f, "USB驱动"),
            DriverType::Storage => write!(f, "存储驱动"),
            DriverType::Input => write!(f, "输入设备驱动"),
            DriverType::Firmware => write!(f, "BIOS/固件"),
            DriverType::Other => write!(f, "其他驱动"),
            DriverType::Unknown => write!(f, "未知类型"),
        }
    }
}

/// 驱动包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverPackage {
    /// 包ID
    pub id: String,
    /// 驱动名称
    pub name: String,
    /// 版本
    pub version: DriverVersion,
    /// 厂商
    pub vendor: String,
    /// 下载URL
    pub download_url: String,
    /// 文件大小
    pub file_size: u64,
    /// SHA256校验和
    pub sha256: String,
    /// 支持的硬件ID列表
    pub supported_hardware_ids: Vec<String>,
    /// 支持的操作系统
    pub supported_os: Vec<String>,
    /// 发布日期
    pub release_date: DateTime<Utc>,
    /// 发布说明
    pub release_notes: Option<String>,
    /// 是否需要重启
    pub needs_reboot: bool,
    /// 静默安装参数
    pub silent_install_args: Option<String>,
}

/// 下载结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    /// 驱动名称
    pub driver_name: String,
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件大小
    pub file_size: u64,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 安装结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    /// 驱动名称
    pub driver_name: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
    /// 安装后的版本
    pub installed_version: Option<DriverVersion>,
    /// 是否需要重启
    pub needs_reboot: bool,
}

/// 格式化文件大小
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 驱动匹配分数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverMatchScore {
    /// 总分
    pub total_score: u32,
    /// 硬件ID匹配分数
    pub hardware_id_score: u32,
    /// 版本匹配分数
    pub version_score: u32,
    /// 日期匹配分数
    pub date_score: u32,
    /// 厂商匹配分数
    pub vendor_score: u32,
}

impl DriverMatchScore {
    /// 创建新的匹配分数
    pub fn new() -> Self {
        Self {
            total_score: 0,
            hardware_id_score: 0,
            version_score: 0,
            date_score: 0,
            vendor_score: 0,
        }
    }

    /// 计算总分
    pub fn calculate_total(&mut self) {
        self.total_score = self.hardware_id_score
            + self.version_score
            + self.date_score
            + self.vendor_score;
    }
}

impl Default for DriverMatchScore {
    fn default() -> Self {
        Self::new()
    }
}
