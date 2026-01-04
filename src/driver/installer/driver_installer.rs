//! 驱动安装器主类
use crate::types::driver_types::{DriverInfo, InstallResult, DriverVersion};
use crate::utils::error::Result;
use std::path::Path;

pub struct DriverInstaller;

impl DriverInstaller {
    pub fn new() -> Self {
        Self
    }

    pub async fn install_driver(&self, driver: &DriverInfo, path: &Path) -> Result<InstallResult> {
        tracing::info!("开始安装驱动: {}", driver.name);
        
        // 根据文件扩展名选择安装方法
        let result = if path.extension().map_or(false, |ext| ext == "inf") {
            self.install_inf_driver(path, &driver.name).await
        } else if path.extension().map_or(false, |ext| ext == "exe") {
            self.install_exe_driver(path, &driver.name).await
        } else {
            self.install_generic_driver(path, &driver.name).await
        };

        tracing::info!("驱动安装完成: {}", driver.name);
        result
    }

    async fn install_inf_driver(&self, path: &Path, name: &str) -> Result<InstallResult> {
        #[cfg(windows)]
        {
            use crate::utils::process_utils::run_command_silent;
            
            let output = run_command_silent(
                "pnputil",
                &["/add-driver", &path.to_string_lossy(), "/install"],
            )?;

            if output.status.success() {
                Ok(InstallResult {
                    driver_name: name.to_string(),
                    success: true,
                    error_message: None,
                    installed_version: Some(DriverVersion::parse("1.0.0.0")),
                    needs_reboot: false,
                })
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Ok(InstallResult {
                    driver_name: name.to_string(),
                    success: false,
                    error_message: Some(stderr.to_string()),
                    installed_version: None,
                    needs_reboot: false,
                })
            }
        }
        
        #[cfg(not(windows))]
        {
            Ok(InstallResult {
                driver_name: name.to_string(),
                success: false,
                error_message: Some("仅支持Windows系统".to_string()),
                installed_version: None,
            })
        }
    }

    async fn install_exe_driver(&self, path: &Path, name: &str) -> Result<InstallResult> {
        #[cfg(windows)]
        {
            use tokio::process::Command;
            
            let output = Command::new(path)
                .args(&["/S", "/VERYSILENT", "/NORESTART"])
                .output()
                .await
                .map_err(|e| crate::utils::error::HamsterError::InstallError(e.to_string()))?;

            if output.status.success() {
                Ok(InstallResult {
                    driver_name: name.to_string(),
                    success: true,
                    error_message: None,
                    installed_version: Some(DriverVersion::parse("1.0.0.0")),
                    needs_reboot: false,
                })
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Ok(InstallResult {
                    driver_name: name.to_string(),
                    success: false,
                    error_message: Some(stderr.to_string()),
                    installed_version: None,
                    needs_reboot: false,
                })
            }
        }
        
        #[cfg(not(windows))]
        {
            Ok(InstallResult {
                driver_name: name.to_string(),
                success: false,
                error_message: Some("仅支持Windows系统".to_string()),
                installed_version: None,
            })
        }
    }

    async fn install_generic_driver(&self, _path: &Path, name: &str) -> Result<InstallResult> {
        Ok(InstallResult {
            driver_name: name.to_string(),
            success: false,
            error_message: Some("不支持的驱动格式".to_string()),
            installed_version: None,
            needs_reboot: false,
        })
    }
}

impl Default for DriverInstaller {
    fn default() -> Self {
        Self::new()
    }
}
