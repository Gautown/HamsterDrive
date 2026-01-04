use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::Path;
use tokio::fs;




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallationResult {
    pub success: bool,
    pub message: String,
    pub driver_version: String,
    pub installed_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriverInfo {
    pub file_path: String,
    pub file_name: String,
    pub hardware_id: String,
    pub manufacturer: String,
    pub driver_version: String,
}

pub struct DriverInstaller {
    pub needs_elevation: bool,
}

impl DriverInstaller {
    pub fn new() -> Self {
        DriverInstaller {
            needs_elevation: false,
        }
    }

    pub async fn install_driver(&self, driver_info: &DriverInfo) -> Result<InstallationResult> {
        // 检查是否需要提升权限
        if !self.has_admin_privileges() {
            return Err(anyhow::anyhow!("需要管理员权限来安装驱动程序"));
        }

        // 确定安装方法
        let file_extension = Path::new(&driver_info.file_path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_lowercase();

        let result = match file_extension.as_str() {
            "inf" => self.install_inf_driver(driver_info).await,
            "exe" => self.install_exe_driver(driver_info).await,
            "msi" => self.install_msi_driver(driver_info).await,
            _ => Err(anyhow::anyhow!("不支持的驱动文件格式: {}", file_extension)),
        };

        match result {
            Ok(_success) => {
                Ok(InstallationResult {
                    success: true,
                    message: format!("驱动程序安装成功: {}", driver_info.file_name),
                    driver_version: driver_info.driver_version.clone(),
                    installed_at: chrono::Utc::now().to_rfc3339(),
                })
            }
            Err(e) => {
                Ok(InstallationResult {
                    success: false,
                    message: format!("驱动程序安装失败: {}", e),
                    driver_version: driver_info.driver_version.clone(),
                    installed_at: chrono::Utc::now().to_rfc3339(),
                })
            }
        }
    }

    async fn install_inf_driver(&self, driver_info: &DriverInfo) -> Result<()> {
        // 验证路径长度
        if driver_info.file_path.is_empty() || driver_info.file_path.len() > 32767 {
            return Err(anyhow::anyhow!("驱动文件路径无效: {}", driver_info.file_path));
        }
        
        // 使用pnputil安装INF驱动
        let output = Command::new("pnputil")
            .args(&["/add-driver", &driver_info.file_path, "/install"])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("pnputil安装失败: {}", error_msg))
        }
    }

    async fn install_exe_driver(&self, driver_info: &DriverInfo) -> Result<()> {
        // 验证路径长度
        if driver_info.file_path.is_empty() || driver_info.file_path.len() > 32767 {
            return Err(anyhow::anyhow!("驱动文件路径无效: {}", driver_info.file_path));
        }
        
        // 使用静默参数安装EXE驱动
        let output = Command::new(&driver_info.file_path)
            .args(&["/S", "/SILENT", "/VERYSILENT"]) // 尝试多种静默安装参数
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            // 如果标准参数失败，尝试其他参数
            let output = Command::new(&driver_info.file_path)
                .args(&["/quiet", "/quietinstall"])
                .output()?;

            if output.status.success() {
                Ok(())
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!("EXE驱动安装失败: {}", error_msg))
            }
        }
    }

    async fn install_msi_driver(&self, driver_info: &DriverInfo) -> Result<()> {
        // 验证路径长度
        if driver_info.file_path.is_empty() || driver_info.file_path.len() > 32767 {
            return Err(anyhow::anyhow!("驱动文件路径无效: {}", driver_info.file_path));
        }
        
        // 使用msiexec安装MSI驱动
        let output = Command::new("msiexec")
            .args(&["/i", &driver_info.file_path, "/quiet", "/norestart"])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("MSI驱动安装失败: {}", error_msg))
        }
    }

    fn has_admin_privileges(&self) -> bool {
        // 检查是否具有管理员权限
        match Command::new("net").arg("session").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub async fn request_elevation(&self) -> Result<()> {
        // 在Windows上请求提升权限
        let current_exe = std::env::current_exe()?;
        let current_exe_str = current_exe.to_string_lossy().to_string();

        let output = Command::new("powershell")
            .args(&[
                "-WindowStyle", "Hidden",
                "-Command",
                &format!("Start-Process '{}' -Verb RunAs", current_exe_str)
            ])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("请求提升权限失败: {}", error_msg))
        }
    }

    pub async fn create_system_restore_point(&self, description: &str) -> Result<()> {
        // 创建系统还原点
        let ps_script = format!(
            r#"
            $description = "{}"
            $restorePointType = 12  # APPLICATION_INSTALL
            $eventType = 100  # INFO
            
            Checkpoint-Computer -Description $description -RestorePointType $restorePointType
            "#,
            description
        );

        let output = Command::new("powershell")
            .args(&["-Command", &ps_script])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("创建系统还原点失败: {}", error_msg))
        }
    }

    pub async fn validate_driver(&self, driver_path: &str) -> Result<bool> {
        // 验证驱动文件的有效性
        if !Path::new(driver_path).exists() {
            return Ok(false);
        }

        // 检查文件扩展名
        let extension: Option<String> = Path::new(driver_path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(|s| s.to_lowercase());

        match extension.as_deref() {
            Some("inf") => {
                // 验证INF文件
                self.validate_inf_file(driver_path).await
            }
            Some("exe") | Some("msi") => {
                // 对于可执行文件，检查文件是否有效
                let metadata = fs::metadata(driver_path).await?;
                Ok(metadata.len() > 0)
            }
            _ => Ok(false),
        }
    }

    async fn validate_inf_file(&self, inf_path: &str) -> Result<bool> {
        // 使用pnputil验证INF文件
        let output = Command::new("pnputil")
            .args(&["/enum-drivers", "/path", inf_path])
            .output()?;

        Ok(output.status.success())
    }

    pub async fn get_driver_signature_status(&self, driver_path: &str) -> Result<String> {
        // 检查驱动程序的数字签名
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                &format!("Get-AuthenticodeSignature -FilePath \"{}\"", driver_path)
            ])
            .output()?;

        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            // 解析签名状态
            for line in output_str.lines() {
                if line.contains("Status") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() > 1 {
                        return Ok(parts[1].trim().to_string());
                    }
                }
            }
            Ok("未知".to_string())
        } else {
            Ok("验证失败".to_string())
        }
    }

    pub async fn backup_current_driver(&self, hardware_id: &str, backup_dir: &str) -> Result<()> {
        // 备份当前驱动程序
        // 使用pnputil导出现有驱动
        let output = Command::new("pnputil")
            .args(&["/export-driver", hardware_id, backup_dir])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("备份驱动失败: {}", error_msg))
        }
    }

    pub async fn rollback_driver(&self, backup_path: &str) -> Result<()> {
        // 回滚到之前的驱动版本
        if Path::new(backup_path).extension().and_then(std::ffi::OsStr::to_str) == Some("inf") {
            self.install_inf_driver(&DriverInfo {
                file_path: backup_path.to_string(),
                file_name: Path::new(backup_path).file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("backup.inf").to_string(),
                hardware_id: "".to_string(),
                manufacturer: "".to_string(),
                driver_version: "".to_string(),
            }).await
        } else {
            Err(anyhow::anyhow!("不支持的备份文件格式"))
        }
    }
}

// Windows特定的权限和安装辅助函数
pub mod windows_utils {
    use anyhow::Result;

    pub fn is_running_as_admin() -> bool {
        // 检查是否以管理员身份运行
        // 通过尝试执行需要管理员权限的命令来判断
        match std::process::Command::new("net")
            .args(&["session"])
            .output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub fn run_as_admin(exe_path: &str, args: Option<&str>) -> Result<()> {
        let powershell_cmd = if let Some(arguments) = args {
            format!("Start-Process '{}' -ArgumentList '{}' -Verb RunAs", exe_path, arguments)
        } else {
            format!("Start-Process '{}' -Verb RunAs", exe_path)
        };

        let output = std::process::Command::new("powershell")
            .args(&["-Command", &powershell_cmd])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("以管理员身份运行失败: {}", error))
        }
    }
}