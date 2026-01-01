//! 操作按钮组件
//!
//! 用于显示设备/驱动操作按钮的UI组件

use crate::utils::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionButtonType {
    Scan,
    Update,
    Install,
    Backup,
    Restore,
    Uninstall,
    Details,
    Ignore,
}

pub struct ActionButton {
    pub button_type: ActionButtonType,
    pub text: String,
    pub enabled: bool,
    pub tooltip: String,
}

pub struct ActionButtons {
    pub buttons: Vec<ActionButton>,
}

impl ActionButtons {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    /// 创建扫描按钮
    pub fn add_scan_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Scan,
            text: "扫描".to_string(),
            enabled: true,
            tooltip: "扫描系统中的硬件设备".to_string(),
        });
        self
    }

    /// 创建更新按钮
    pub fn add_update_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Update,
            text: "更新".to_string(),
            enabled: false, // 默认禁用，需要有可更新的驱动时才启用
            tooltip: "更新选中的驱动".to_string(),
        });
        self
    }

    /// 创建安装按钮
    pub fn add_install_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Install,
            text: "安装".to_string(),
            enabled: false, // 默认禁用，需要有可安装的驱动时才启用
            tooltip: "安装选中的驱动".to_string(),
        });
        self
    }

    /// 创建备份按钮
    pub fn add_backup_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Backup,
            text: "备份".to_string(),
            enabled: true,
            tooltip: "备份当前驱动".to_string(),
        });
        self
    }

    /// 创建还原按钮
    pub fn add_restore_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Restore,
            text: "还原".to_string(),
            enabled: false, // 默认禁用
            tooltip: "还原已备份的驱动".to_string(),
        });
        self
    }

    /// 创建卸载按钮
    pub fn add_uninstall_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Uninstall,
            text: "卸载".to_string(),
            enabled: false, // 默认禁用
            tooltip: "卸载选中的驱动".to_string(),
        });
        self
    }

    /// 创建详情按钮
    pub fn add_details_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Details,
            text: "详情".to_string(),
            enabled: false, // 默认禁用
            tooltip: "查看设备详细信息".to_string(),
        });
        self
    }

    /// 创建忽略按钮
    pub fn add_ignore_button(&mut self) -> &mut Self {
        self.buttons.push(ActionButton {
            button_type: ActionButtonType::Ignore,
            text: "忽略".to_string(),
            enabled: false, // 默认禁用
            tooltip: "忽略选中的驱动更新".to_string(),
        });
        self
    }

    /// 启用特定类型的按钮
    pub fn enable_button(&mut self, button_type: ActionButtonType) -> &mut Self {
        if let Some(button) = self.buttons.iter_mut().find(|b| b.button_type == button_type) {
            button.enabled = true;
        }
        self
    }

    /// 禁用特定类型的按钮
    pub fn disable_button(&mut self, button_type: ActionButtonType) -> &mut Self {
        if let Some(button) = self.buttons.iter_mut().find(|b| b.button_type == button_type) {
            button.enabled = false;
        }
        self
    }

    /// 检查是否有按钮被启用
    pub fn has_enabled_buttons(&self) -> bool {
        self.buttons.iter().any(|b| b.enabled)
    }

    /// 获取启用的按钮数量
    pub fn enabled_button_count(&self) -> usize {
        self.buttons.iter().filter(|b| b.enabled).count()
    }

    /// 获取按钮列表的可变引用
    pub fn get_buttons_mut(&mut self) -> &mut Vec<ActionButton> {
        &mut self.buttons
    }

    /// 获取按钮列表的引用
    pub fn get_buttons(&self) -> &Vec<ActionButton> {
        &self.buttons
    }

    /// 清空所有按钮
    pub fn clear(&mut self) -> &mut Self {
        self.buttons.clear();
        self
    }
}