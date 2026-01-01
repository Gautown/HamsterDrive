//! 硬件相关类型定义

use serde::{Deserialize, Serialize};
use std::fmt;

/// 硬件标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardwareId {
    /// 完整的硬件ID字符串
    pub full_id: String,
    /// 厂商ID (VEN_XXXX)
    pub vendor_id: Option<String>,
    /// 设备ID (DEV_XXXX)
    pub device_id: Option<String>,
    /// 子系统ID (SUBSYS_XXXXXXXX)
    pub subsys_id: Option<String>,
    /// 修订版本 (REV_XX)
    pub revision: Option<String>,
}

impl HardwareId {
    /// 从完整的硬件ID字符串解析
    pub fn parse(full_id: &str) -> Self {
        let upper_id = full_id.to_uppercase();
        
        let vendor_id = Self::extract_field(&upper_id, "VEN_", 4);
        let device_id = Self::extract_field(&upper_id, "DEV_", 4);
        let subsys_id = Self::extract_field(&upper_id, "SUBSYS_", 8);
        let revision = Self::extract_field(&upper_id, "REV_", 2);

        Self {
            full_id: full_id.to_string(),
            vendor_id,
            device_id,
            subsys_id,
            revision,
        }
    }

    fn extract_field(id: &str, prefix: &str, length: usize) -> Option<String> {
        if let Some(pos) = id.find(prefix) {
            let start = pos + prefix.len();
            if start + length <= id.len() {
                return Some(id[start..start + length].to_string());
            }
        }
        None
    }

    /// 获取用于匹配的短ID (VEN_XXXX&DEV_XXXX)
    pub fn short_id(&self) -> Option<String> {
        match (&self.vendor_id, &self.device_id) {
            (Some(ven), Some(dev)) => Some(format!("VEN_{}&DEV_{}", ven, dev)),
            _ => None,
        }
    }
}

impl fmt::Display for HardwareId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_id)
    }
}

/// 设备类别
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceClass {
    /// 显示适配器
    Display,
    /// 网络适配器
    Network,
    /// 声音设备
    Sound,
    /// USB控制器
    USB,
    /// 存储控制器
    Storage,
    /// 系统设备
    System,
    /// 处理器
    Processor,
    /// 输入设备
    Input,
    /// 打印机
    Printer,
    /// 蓝牙设备
    Bluetooth,
    /// 摄像头
    Camera,
    /// 生物识别设备
    Biometric,
    /// 其他设备
    Other(String),
}

impl DeviceClass {
    /// 从设备类别GUID解析
    pub fn from_guid(guid: &str) -> Self {
        match guid.to_uppercase().as_str() {
            "{4D36E968-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Display,
            "{4D36E972-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Network,
            "{4D36E96C-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Sound,
            "{36FC9E60-C465-11CF-8056-444553540000}" => DeviceClass::USB,
            "{4D36E97B-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Storage,
            "{4D36E97D-E325-11CE-BFC1-08002BE10318}" => DeviceClass::System,
            "{50127DC3-0F36-415E-A6CC-4CB3BE910B65}" => DeviceClass::Processor,
            "{4D36E96B-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Input,
            "{4D36E979-E325-11CE-BFC1-08002BE10318}" => DeviceClass::Printer,
            "{E0CBF06C-CD8B-4647-BB8A-263B43F0F974}" => DeviceClass::Bluetooth,
            "{CA3E7AB9-B4C3-4AE6-8251-579EF933890F}" => DeviceClass::Camera,
            "{53D29EF7-377C-4D14-864B-EB3A85769359}" => DeviceClass::Biometric,
            _ => DeviceClass::Other(guid.to_string()),
        }
    }

    /// 获取设备类别的显示名称
    pub fn display_name(&self) -> &str {
        match self {
            DeviceClass::Display => "显示适配器",
            DeviceClass::Network => "网络适配器",
            DeviceClass::Sound => "声音设备",
            DeviceClass::USB => "USB控制器",
            DeviceClass::Storage => "存储控制器",
            DeviceClass::System => "系统设备",
            DeviceClass::Processor => "处理器",
            DeviceClass::Input => "输入设备",
            DeviceClass::Printer => "打印机",
            DeviceClass::Bluetooth => "蓝牙设备",
            DeviceClass::Camera => "摄像头",
            DeviceClass::Biometric => "生物识别设备",
            DeviceClass::Other(_) => "其他设备",
        }
    }
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备实例ID
    pub instance_id: String,
    /// 设备名称
    pub name: String,
    /// 设备描述
    pub description: String,
    /// 设备类别
    pub device_class: DeviceClass,
    /// 硬件ID列表
    pub hardware_ids: Vec<HardwareId>,
    /// 兼容ID列表
    pub compatible_ids: Vec<String>,
    /// 厂商名称
    pub vendor_name: Option<String>,
    /// 当前驱动版本
    pub driver_version: Option<String>,
    /// 驱动日期
    pub driver_date: Option<String>,
    /// 驱动提供商
    pub driver_provider: Option<String>,
    /// INF文件名
    pub inf_name: Option<String>,
    /// 设备状态
    pub status: DeviceStatus,
    /// 设备问题代码
    pub problem_code: Option<u32>,
    /// 是否有问题
    pub has_problem: bool,
}

impl DeviceInfo {
    /// 获取主硬件ID
    pub fn primary_hardware_id(&self) -> Option<&HardwareId> {
        self.hardware_ids.first()
    }

    /// 获取厂商ID
    pub fn vendor_id(&self) -> Option<&str> {
        self.primary_hardware_id()
            .and_then(|h| h.vendor_id.as_deref())
    }
}

/// 设备状态
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceStatus {
    /// 正常工作
    Working,
    /// 已禁用
    Disabled,
    /// 有问题
    Problem,
    /// 未知状态
    Unknown,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Working => write!(f, "正常"),
            DeviceStatus::Disabled => write!(f, "已禁用"),
            DeviceStatus::Problem => write!(f, "有问题"),
            DeviceStatus::Unknown => write!(f, "未知"),
        }
    }
}

/// 厂商信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorInfo {
    /// 厂商ID
    pub vendor_id: String,
    /// 厂商名称
    pub name: String,
    /// 厂商官网
    pub website: Option<String>,
    /// 驱动下载页面
    pub driver_page: Option<String>,
    /// 支持的设备类别
    pub device_classes: Vec<DeviceClass>,
}

/// 已知厂商ID映射
pub fn get_vendor_name(vendor_id: &str) -> Option<&'static str> {
    match vendor_id.to_uppercase().as_str() {
        "10DE" => Some("NVIDIA"),
        "1002" => Some("AMD/ATI"),
        "8086" => Some("Intel"),
        "14E4" => Some("Broadcom"),
        "10EC" => Some("Realtek"),
        "1969" | "1B4B" => Some("Marvell"),
        "168C" => Some("Qualcomm Atheros"),
        "17AA" => Some("Lenovo"),
        "1028" => Some("Dell"),
        "103C" => Some("HP"),
        "1043" => Some("ASUS"),
        "1458" => Some("Gigabyte"),
        "1462" => Some("MSI"),
        "1179" => Some("Toshiba"),
        "104C" => Some("Texas Instruments"),
        "1217" => Some("O2 Micro"),
        "1180" => Some("Ricoh"),
        _ => None,
    }
}
