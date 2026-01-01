//! 系统相关类型定义

use serde::{Deserialize, Serialize};
use std::fmt;

/// 操作系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    /// 操作系统名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 构建号
    pub build: String,
    /// 架构 (x64, x86, ARM64)
    pub architecture: Architecture,
    /// 服务包版本
    pub service_pack: Option<String>,
    /// 产品类型
    pub product_type: ProductType,
    /// 是否已激活
    pub is_activated: bool,
    /// 激活状态详情
    pub activation_status: String,
    /// 安装日期
    pub install_date: Option<String>,
    /// 最后更新日期
    pub last_update: Option<String>,
}

impl OSInfo {
    /// 创建默认的OS信息
    pub fn new() -> Self {
        Self {
            name: "Windows".to_string(),
            version: "Unknown".to_string(),
            build: "Unknown".to_string(),
            architecture: Architecture::X64,
            service_pack: None,
            product_type: ProductType::Workstation,
            is_activated: false,
            activation_status: "未知".to_string(),
            install_date: None,
            last_update: None,
        }
    }

    /// 获取完整的版本字符串
    pub fn full_version(&self) -> String {
        format!(
            "{} {} (Build {})",
            self.name, self.version, self.build
        )
    }

    /// 判断是否是Windows 10或更高版本
    pub fn is_windows_10_or_later(&self) -> bool {
        if let Ok(build) = self.build.parse::<u32>() {
            build >= 10240 // Windows 10 起始构建号
        } else {
            false
        }
    }

    /// 判断是否是Windows 11
    pub fn is_windows_11(&self) -> bool {
        if let Ok(build) = self.build.parse::<u32>() {
            build >= 22000 // Windows 11 起始构建号
        } else {
            false
        }
    }
}

impl Default for OSInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OSInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_version())
    }
}

/// 系统架构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    X86,
    X64,
    ARM64,
    Unknown,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::X86 => write!(f, "32-bit"),
            Architecture::X64 => write!(f, "64-bit"),
            Architecture::ARM64 => write!(f, "ARM64"),
            Architecture::Unknown => write!(f, "Unknown"),
        }
    }
}

/// 产品类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    /// 工作站
    Workstation,
    /// 服务器
    Server,
    /// 域控制器
    DomainController,
    /// 未知
    Unknown,
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductType::Workstation => write!(f, "工作站"),
            ProductType::Server => write!(f, "服务器"),
            ProductType::DomainController => write!(f, "域控制器"),
            ProductType::Unknown => write!(f, "未知"),
        }
    }
}

/// CPU信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// CPU名称
    pub name: String,
    /// 厂商
    pub vendor: String,
    /// 核心数
    pub cores: u32,
    /// 线程数
    pub threads: u32,
    /// 基础频率 (MHz)
    pub base_clock: u32,
    /// 架构
    pub architecture: Architecture,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} 核心, {} 线程, {} MHz)",
            self.name, self.cores, self.threads, self.base_clock
        )
    }
}

/// 内存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// 总物理内存（字节）
    pub total_physical: u64,
    /// 可用物理内存（字节）
    pub available_physical: u64,
    /// 总虚拟内存（字节）
    pub total_virtual: u64,
    /// 可用虚拟内存（字节）
    pub available_virtual: u64,
    /// 内存槽信息
    pub slots: Vec<MemorySlot>,
}

impl MemoryInfo {
    /// 获取已用物理内存
    pub fn used_physical(&self) -> u64 {
        self.total_physical.saturating_sub(self.available_physical)
    }

    /// 获取内存使用率
    pub fn usage_percent(&self) -> f64 {
        if self.total_physical > 0 {
            (self.used_physical() as f64 / self.total_physical as f64) * 100.0
        } else {
            0.0
        }
    }

    /// 格式化总内存大小
    pub fn formatted_total(&self) -> String {
        format_bytes(self.total_physical)
    }
}

/// 内存槽信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySlot {
    /// 槽位置
    pub slot: String,
    /// 容量（字节）
    pub capacity: u64,
    /// 速度 (MHz)
    pub speed: u32,
    /// 类型
    pub memory_type: String,
    /// 厂商
    pub manufacturer: String,
}

/// 磁盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// 磁盘型号
    pub model: String,
    /// 序列号
    pub serial_number: String,
    /// 总容量（字节）
    pub total_size: u64,
    /// 接口类型
    pub interface_type: String,
    /// 媒体类型（SSD/HDD）
    pub media_type: MediaType,
    /// 分区列表
    pub partitions: Vec<PartitionInfo>,
}

impl DiskInfo {
    /// 格式化磁盘大小
    pub fn formatted_size(&self) -> String {
        format_bytes(self.total_size)
    }
}

impl fmt::Display for DiskInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.model, self.formatted_size(), self.media_type)
    }
}

/// 媒体类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    SSD,
    HDD,
    NVMe,
    Unknown,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::SSD => write!(f, "SSD"),
            MediaType::HDD => write!(f, "HDD"),
            MediaType::NVMe => write!(f, "NVMe"),
            MediaType::Unknown => write!(f, "未知"),
        }
    }
}

/// 分区信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    /// 盘符
    pub drive_letter: String,
    /// 卷标
    pub label: String,
    /// 文件系统
    pub file_system: String,
    /// 总容量（字节）
    pub total_size: u64,
    /// 可用空间（字节）
    pub free_space: u64,
}

impl PartitionInfo {
    /// 获取已用空间
    pub fn used_space(&self) -> u64 {
        self.total_size.saturating_sub(self.free_space)
    }

    /// 获取使用率
    pub fn usage_percent(&self) -> f64 {
        if self.total_size > 0 {
            (self.used_space() as f64 / self.total_size as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// 显卡信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// 显卡名称
    pub name: String,
    /// 厂商
    pub vendor: String,
    /// 显存大小（字节）
    pub vram_size: u64,
    /// 驱动版本
    pub driver_version: String,
    /// 驱动日期
    pub driver_date: String,
    /// 硬件ID
    pub hardware_id: String,
}

impl fmt::Display for GpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, 驱动: {})",
            self.name,
            format_bytes(self.vram_size),
            self.driver_version
        )
    }
}

/// 主板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotherboardInfo {
    /// 厂商
    pub manufacturer: String,
    /// 型号
    pub product: String,
    /// 版本
    pub version: String,
    /// 序列号
    pub serial_number: String,
    /// BIOS版本
    pub bios_version: String,
    /// BIOS日期
    pub bios_date: String,
}

impl fmt::Display for MotherboardInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} (BIOS: {})", self.manufacturer, self.product, self.bios_version)
    }
}

/// 系统摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSummary {
    /// 操作系统信息
    pub os: OSInfo,
    /// CPU信息
    pub cpu: Option<CpuInfo>,
    /// 内存信息
    pub memory: Option<MemoryInfo>,
    /// 主板信息
    pub motherboard: Option<MotherboardInfo>,
    /// 显卡列表
    pub gpus: Vec<GpuInfo>,
    /// 磁盘列表
    pub disks: Vec<DiskInfo>,
}

impl SystemSummary {
    pub fn new() -> Self {
        Self {
            os: OSInfo::new(),
            cpu: None,
            memory: None,
            motherboard: None,
            gpus: Vec::new(),
            disks: Vec::new(),
        }
    }
}

impl Default for SystemSummary {
    fn default() -> Self {
        Self::new()
    }
}

/// 格式化字节数
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
