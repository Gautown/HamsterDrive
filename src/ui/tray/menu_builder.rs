//! 托盘菜单构建器
//!
//! 负责构建系统托盘右键菜单

use crate::utils::error::{HamsterError, Result};

pub struct MenuBuilder;

impl MenuBuilder {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// 构建默认托盘菜单
    pub fn build_default_menu(&self) -> Result<Vec<MenuItem>> {
        Ok(vec![
            MenuItem::new("扫描硬件", MenuAction::ScanHardware),
            MenuItem::new("检查更新", MenuAction::CheckUpdates),
            MenuItem::new("打开主界面", MenuAction::OpenMainWindow),
            MenuItem::new("设置", MenuAction::OpenSettings),
            MenuItem::new("退出", MenuAction::Exit),
        ])
    }

    /// 构建带驱动更新选项的菜单
    pub fn build_update_menu(&self, update_count: usize) -> Result<Vec<MenuItem>> {
        Ok(vec![
            MenuItem::new(&format!("发现 {} 个驱动更新", update_count), MenuAction::CheckUpdates),
            MenuItem::new("安装所有更新", MenuAction::InstallAllUpdates),
            MenuItem::new("忽略更新", MenuAction::IgnoreUpdates),
            MenuItem::separator(),
            MenuItem::new("扫描硬件", MenuAction::ScanHardware),
            MenuItem::new("打开主界面", MenuAction::OpenMainWindow),
            MenuItem::new("设置", MenuAction::OpenSettings),
            MenuItem::new("退出", MenuAction::Exit),
        ])
    }

    /// 构建扫描中状态的菜单
    pub fn build_scanning_menu(&self) -> Result<Vec<MenuItem>> {
        Ok(vec![
            MenuItem::new("正在扫描...", MenuAction::None),
            MenuItem::separator(),
            MenuItem::new("取消扫描", MenuAction::CancelScan),
            MenuItem::new("打开主界面", MenuAction::OpenMainWindow),
            MenuItem::new("设置", MenuAction::OpenSettings),
            MenuItem::new("退出", MenuAction::Exit),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub text: String,
    pub action: MenuAction,
    pub enabled: bool,
    pub is_separator: bool,
}

impl MenuItem {
    pub fn new(text: &str, action: MenuAction) -> Self {
        Self {
            text: text.to_string(),
            action,
            enabled: true,
            is_separator: false,
        }
    }

    pub fn separator() -> Self {
        Self {
            text: String::new(),
            action: MenuAction::None,
            enabled: false,
            is_separator: true,
        }
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    ScanHardware,
    CheckUpdates,
    InstallAllUpdates,
    IgnoreUpdates,
    OpenMainWindow,
    OpenSettings,
    CancelScan,
    Exit,
    None, // 用于分隔符或禁用项
}