//! 驱动更新器核心控制器
//!
//! DriverUpdaterCore 是应用程序的核心控制器，
//! 负责协调所有模块的工作

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{
    DeviceInfo, DriverInfo, DriverStatus,
    SystemSummary,
};
use crate::utils::error::{HamsterError, Result};
use crate::core::state::AppState;

/// 驱动更新器核心控制器
pub struct DriverUpdaterCore {
    /// 应用程序状态
    state: Arc<RwLock<AppState>>,
    /// 是否已初始化
    initialized: bool,
}

impl DriverUpdaterCore {
    /// 创建新的核心控制器实例
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::new())),
            initialized: false,
        }
    }

    /// 初始化核心控制器
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        tracing::info!("正在初始化驱动更新器核心...");

        // 初始化各个子系统
        self.init_database().await?;
        self.init_config().await?;
        self.load_system_info().await?;

        self.initialized = true;
        tracing::info!("驱动更新器核心初始化完成");

        Ok(())
    }

    /// 初始化数据库
    async fn init_database(&self) -> Result<()> {
        tracing::debug!("初始化数据库...");
        // 数据库初始化逻辑将在database模块中实现
        Ok(())
    }

    /// 初始化配置
    async fn init_config(&self) -> Result<()> {
        tracing::debug!("加载配置...");
        // 配置加载逻辑将在config模块中实现
        Ok(())
    }

    /// 加载系统信息
    async fn load_system_info(&self) -> Result<()> {
        tracing::debug!("加载系统信息...");
        
        let os_info = crate::utils::system_utils::get_os_info()?;
        
        let mut state = self.state.write().await;
        state.system_summary.os = os_info;
        
        Ok(())
    }

    /// 获取应用状态的只读引用
    pub async fn get_state(&self) -> AppState {
        self.state.read().await.clone()
    }

    /// 扫描系统硬件
    pub async fn scan_hardware(&self) -> Result<Vec<DeviceInfo>> {
        tracing::info!("开始扫描系统硬件...");
        
        let mut state = self.state.write().await;
        state.is_scanning = true;
        drop(state);

        // 调用硬件扫描模块
        let devices = self.perform_hardware_scan().await?;

        let mut state = self.state.write().await;
        state.devices = devices.clone();
        state.is_scanning = false;
        state.last_scan_time = Some(std::time::Instant::now());

        tracing::info!("硬件扫描完成，发现 {} 个设备", devices.len());
        Ok(devices)
    }

    /// 执行硬件扫描
    async fn perform_hardware_scan(&self) -> Result<Vec<DeviceInfo>> {
        // 这里将调用 hardware 模块的扫描功能
        // 目前返回空列表，实际实现将在 hardware 模块中完成
        Ok(Vec::new())
    }

    /// 检查驱动更新
    pub async fn check_driver_updates(&self) -> Result<Vec<DriverInfo>> {
        tracing::info!("开始检查驱动更新...");

        let mut state = self.state.write().await;
        state.is_checking_updates = true;
        drop(state);

        // 获取当前设备列表
        let state = self.state.read().await;
        let devices = state.devices.clone();
        drop(state);

        // 检查每个设备的驱动更新
        let mut outdated_drivers = Vec::new();
        
        for device in devices {
            if let Some(driver_info) = self.check_device_driver_update(&device).await? {
                if driver_info.status == DriverStatus::Outdated {
                    outdated_drivers.push(driver_info);
                }
            }
        }

        let mut state = self.state.write().await;
        state.outdated_drivers = outdated_drivers.clone();
        state.is_checking_updates = false;

        tracing::info!("驱动更新检查完成，发现 {} 个需要更新的驱动", outdated_drivers.len());
        Ok(outdated_drivers)
    }

    /// 检查单个设备的驱动更新
    async fn check_device_driver_update(&self, _device: &DeviceInfo) -> Result<Option<DriverInfo>> {
        // 这里将调用 driver/fetcher 模块来获取最新驱动信息
        // 目前返回 None，实际实现将在 driver 模块中完成
        Ok(None)
    }

    /// 下载并安装驱动更新
    pub async fn install_driver_update(&self, driver: &DriverInfo) -> Result<()> {
        tracing::info!("开始安装驱动: {}", driver.name);

        // 1. 创建系统还原点
        self.create_restore_point(&format!("安装驱动: {}", driver.name)).await?;

        // 2. 下载驱动
        let download_path = self.download_driver(driver).await?;

        // 3. 安装驱动
        self.install_driver_from_file(&download_path, driver).await?;

        tracing::info!("驱动安装完成: {}", driver.name);
        Ok(())
    }

    /// 创建系统还原点
    async fn create_restore_point(&self, description: &str) -> Result<()> {
        tracing::debug!("创建系统还原点: {}", description);
        crate::utils::system_utils::create_restore_point(description)
    }

    /// 下载驱动
    async fn download_driver(&self, _driver: &DriverInfo) -> Result<std::path::PathBuf> {
        // 这里将调用 download 模块来下载驱动
        // 目前返回临时路径，实际实现将在 download 模块中完成
        let download_dir = crate::utils::file_utils::get_download_dir()?;
        Ok(download_dir.join("driver.tmp"))
    }

    /// 从文件安装驱动
    async fn install_driver_from_file(
        &self,
        path: &std::path::Path,
        driver: &DriverInfo,
    ) -> Result<()> {
        // 这里将调用 driver/installer 模块来安装驱动
        // 目前只是占位，实际实现将在 driver 模块中完成
        tracing::debug!("从文件安装驱动: {:?}", path);
        Ok(())
    }

    /// 备份当前驱动
    pub async fn backup_drivers(&self) -> Result<std::path::PathBuf> {
        tracing::info!("开始备份驱动...");

        let backup_dir = crate::utils::file_utils::get_backup_dir()?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_path = backup_dir.join(format!("backup_{}", timestamp));

        // 创建备份目录
        crate::utils::file_utils::ensure_dir(&backup_path)?;

        // 执行驱动备份
        self.perform_driver_backup(&backup_path).await?;

        tracing::info!("驱动备份完成: {:?}", backup_path);
        Ok(backup_path)
    }

    /// 执行驱动备份
    async fn perform_driver_backup(&self, backup_path: &std::path::Path) -> Result<()> {
        // 使用 dism 或 pnputil 备份驱动
        #[cfg(windows)]
        {
            use crate::utils::process_utils::run_command_silent;
            
            let output = run_command_silent(
                "dism",
                &["/online", "/export-driver", "/destination", &backup_path.to_string_lossy()],
            )?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(HamsterError::BackupError(format!(
                    "驱动备份失败: {}",
                    stderr
                )));
            }
        }

        Ok(())
    }

    /// 恢复驱动
    pub async fn restore_drivers(&self, backup_path: &std::path::Path) -> Result<()> {
        tracing::info!("开始恢复驱动: {:?}", backup_path);

        if !backup_path.exists() {
            return Err(HamsterError::RestoreError("备份路径不存在".to_string()));
        }

        // 创建还原点
        self.create_restore_point("恢复驱动备份").await?;

        // 执行驱动恢复
        self.perform_driver_restore(backup_path).await?;

        tracing::info!("驱动恢复完成");
        Ok(())
    }

    /// 执行驱动恢复
    async fn perform_driver_restore(&self, backup_path: &std::path::Path) -> Result<()> {
        // 遍历备份目录中的 INF 文件并安装
        let inf_files = crate::utils::file_utils::find_files_by_extension(backup_path, "inf")?;

        for inf_file in inf_files {
            tracing::debug!("恢复驱动: {:?}", inf_file);
            
            #[cfg(windows)]
            {
                use crate::utils::process_utils::run_command_silent;
                
                let output = run_command_silent(
                    "pnputil",
                    &["/add-driver", &inf_file.to_string_lossy(), "/install"],
                )?;

                if !output.status.success() {
                    tracing::warn!("驱动恢复警告: {:?}", inf_file);
                }
            }
        }

        Ok(())
    }

    /// 获取系统摘要信息
    pub async fn get_system_summary(&self) -> SystemSummary {
        self.state.read().await.system_summary.clone()
    }

    /// 关闭核心控制器
    pub async fn shutdown(&mut self) -> Result<()> {
        tracing::info!("正在关闭驱动更新器核心...");
        
        // 保存状态
        // 关闭数据库连接
        // 清理临时文件
        
        self.initialized = false;
        tracing::info!("驱动更新器核心已关闭");
        
        Ok(())
    }
}

impl Default for DriverUpdaterCore {
    fn default() -> Self {
        Self::new()
    }
}
