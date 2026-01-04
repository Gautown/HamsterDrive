use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{
    os_info::SystemInfo,
    hardware::{HardwareScanner, HardwareScanResult},
    matcher::{DriverMatcher, HardwareInfo as MatcherHardwareInfo},
    fetcher::{DriverFetcher, DownloadProgress, DownloadTask},
    installer::{DriverInstaller, InstallationResult, DriverInfo as InstallerDriverInfo},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCandidate {
    pub hardware_info: MatcherHardwareInfo,
    pub matched_driver: Option<crate::matcher::DriverInfo>,
    pub needs_update: bool,
    pub current_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current_step: String,
    pub progress: f32,
    pub total_steps: u32,
    pub current_step_number: u32,
}

pub struct DriverUpdaterCore {
    pub system_info: Option<SystemInfo>,
    pub hardware_scanner: HardwareScanner,
    pub driver_matcher: Arc<Mutex<DriverMatcher>>,
    pub driver_fetcher: Arc<Mutex<DriverFetcher>>,
    pub driver_installer: DriverInstaller,
    pub scan_result: Option<HardwareScanResult>,
    pub update_candidates: Vec<UpdateCandidate>,
}

impl DriverUpdaterCore {
    pub async fn new(_db_path: &str, download_dir: &str) -> Result<Self> {
        let hardware_scanner = HardwareScanner::new();
        // 不再使用数据库，直接创建驱动匹配器实例
        let driver_matcher = Arc::new(Mutex::new(DriverMatcher::new("dummy").await?));
        let driver_fetcher = Arc::new(Mutex::new(DriverFetcher::new(
            "localhost".to_string(),
            6800,
            download_dir.to_string(),
        )));
        let driver_installer = DriverInstaller::new();

        Ok(DriverUpdaterCore {
            system_info: None,
            hardware_scanner,
            driver_matcher,
            driver_fetcher,
            driver_installer,
            scan_result: None,
            update_candidates: Vec::new(),
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        println!("正在初始化驱动更新核心...");
        
        // 获取系统信息
        self.system_info = Some(SystemInfo::new()?);
        println!("系统信息获取完成");
        
        // 尝试启动Aria2 RPC服务器，如果失败则记录警告但不中断初始化
        {
            let fetcher = self.driver_fetcher.lock().await;
            match fetcher.start_aria2_rpc().await {
                Ok(_) => println!("Aria2 RPC服务器启动完成"),
                Err(e) => {
                    eprintln!("Aria2 RPC服务器启动失败: {}，将继续运行但下载功能可能受限", e);
                }
            }
        }
        
        Ok(())
    }

    pub async fn scan_system(&mut self, progress_callback: impl Fn(ScanProgress) -> ()) -> Result<HardwareScanResult> {
        println!("开始扫描系统硬件...");
        
        // 更新进度
        progress_callback(ScanProgress {
            current_step: "开始硬件扫描".to_string(),
            progress: 0.0,
            total_steps: 3,
            current_step_number: 1,
        });
        
        // 扫描硬件
        let scan_result = self.hardware_scanner.scan_hardware()?;
        self.scan_result = Some(scan_result.clone());
        
        progress_callback(ScanProgress {
            current_step: "硬件扫描完成".to_string(),
            progress: 33.3,
            total_steps: 3,
            current_step_number: 1,
        });
        
        // 将扫描结果添加到匹配器（现在使用爬虫，不需要存储到数据库）
        {
            let matcher = self.driver_matcher.lock().await;
            for device in &scan_result.devices {
                let hw_info = MatcherHardwareInfo {
                    hardware_id: device.hardware_id.clone(),
                    device_name: device.device_name.clone(),
                    manufacturer: device.manufacturer.clone(),
                    device_class: device.device_class.clone(),
                };
                
                // 现在匹配器使用爬虫直接获取驱动信息，不需要存储硬件信息到数据库
                // 保留此调用以确保兼容性，但实际不会存储到数据库
                matcher.add_hardware_info(&hw_info).await?;
            }
        }
        
        progress_callback(ScanProgress {
            current_step: "硬件信息入库完成".to_string(),
            progress: 66.6,
            total_steps: 3,
            current_step_number: 2,
        });
        
        progress_callback(ScanProgress {
            current_step: "扫描完成".to_string(),
            progress: 100.0,
            total_steps: 3,
            current_step_number: 3,
        });
        
        println!("系统硬件扫描完成，共发现 {} 个设备", scan_result.devices.len());
        Ok(scan_result)
    }

    pub async fn find_driver_updates(&mut self) -> Result<Vec<UpdateCandidate>> {
        println!("开始查找驱动更新...");
        
        if let Some(ref scan_result) = self.scan_result {
            let mut candidates = Vec::new();
            let matcher = self.driver_matcher.lock().await;
            
            for device in &scan_result.devices {
                // 创建MatcherHardwareInfo
                let hw_info = MatcherHardwareInfo {
                    hardware_id: device.hardware_id.clone(),
                    device_name: device.device_name.clone(),
                    manufacturer: device.manufacturer.clone(),
                    device_class: device.device_class.clone(),
                };
                
                // 匹配驱动
                let match_result = matcher.match_driver(&hw_info).await?;
                
                // 检查是否需要更新（这里简化为只要有匹配就认为需要更新）
                let needs_update = match_result.matched_driver.is_some();
                
                let candidate = UpdateCandidate {
                    hardware_info: hw_info,
                    matched_driver: match_result.matched_driver,
                    needs_update,
                    current_version: device.driver_version.clone(),
                };
                
                candidates.push(candidate);
            }
            
            self.update_candidates = candidates;
            println!("驱动更新查找完成，找到 {} 个更新候选", self.update_candidates.len());
            Ok(self.update_candidates.clone())
        } else {
            Err(anyhow::anyhow!("未进行硬件扫描，无法查找驱动更新"))
        }
    }

    pub async fn download_driver(&self, driver_info: &crate::matcher::DriverInfo, 
                                progress_callback: impl Fn(DownloadProgress) -> ()) -> Result<String> {
        // 创建下载任务
        let task = DownloadTask {
            id: format!("download_{}", driver_info.driver_id),
            url: driver_info.driver_url.clone(),
            file_path: "".to_string(), // 由fetcher决定
            file_name: format!("{}_{}.{}", 
                driver_info.driver_name.replace(" ", "_"), 
                driver_info.driver_version.replace(".", "_"), 
                "exe"), // 简化的文件名生成
            expected_size: Some(driver_info.file_size),
            checksum: Some(driver_info.checksum.clone()),
        };
        
        // 执行下载
        {
            let fetcher = self.driver_fetcher.lock().await;
            fetcher.download_driver_with_progress(&task, progress_callback).await?;
        }
        
        // 返回下载文件路径
        Ok(format!("{}/{}", 
            self.driver_fetcher.lock().await.download_dir, 
            task.file_name))
    }

    pub async fn install_driver(&self, driver_path: &str, hardware_id: &str) -> Result<InstallationResult> {
        let installer_info = InstallerDriverInfo {
            file_path: driver_path.to_string(),
            file_name: std::path::Path::new(driver_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("driver")
                .to_string(),
            hardware_id: hardware_id.to_string(),
            manufacturer: "Unknown".to_string(),
            driver_version: "1.0.0".to_string(), // 从驱动文件中获取更准确的版本
        };
        
        self.driver_installer.install_driver(&installer_info).await
    }

    pub async fn update_single_driver(&self, candidate: &UpdateCandidate) -> Result<InstallationResult> {
        if let Some(ref driver_info) = candidate.matched_driver {
            // 下载驱动
            let progress_callback = |progress: DownloadProgress| {
                println!("下载进度: {:.1}% - {}", progress.progress, progress.file_name);
            };
            
            let driver_path = self.download_driver(driver_info, progress_callback).await?;
            
            // 安装驱动
            let result = self.install_driver(&driver_path, &candidate.hardware_info.hardware_id).await?;
            
            Ok(result)
        } else {
            Err(anyhow::anyhow!("没有找到匹配的驱动"))
        }
    }

    pub async fn update_all_drivers(&self) -> Result<Vec<InstallationResult>> {
        let mut results = Vec::new();
        
        for candidate in &self.update_candidates {
            if candidate.needs_update {
                match self.update_single_driver(candidate).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        eprintln!("更新驱动失败: {}", e);
                        // 创建失败结果
                        results.push(InstallationResult {
                            success: false,
                            message: format!("更新失败: {}", e),
                            driver_version: candidate.current_version.clone(),
                            installed_at: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                }
            }
        }
        
        Ok(results)
    }

    pub async fn create_system_restore_point(&self, description: &str) -> Result<()> {
        self.driver_installer.create_system_restore_point(description).await
    }

    pub async fn validate_driver_file(&self, driver_path: &str) -> Result<bool> {
        self.driver_installer.validate_driver(driver_path).await
    }

    pub async fn get_driver_signature_status(&self, driver_path: &str) -> Result<String> {
        self.driver_installer.get_driver_signature_status(driver_path).await
    }

    pub async fn cleanup(&self) -> Result<()> {
        // 停止Aria2 RPC服务器
        {
            let fetcher = self.driver_fetcher.lock().await;
            fetcher.stop_aria2_rpc().await?;
        }
        
        Ok(())
    }

    pub fn get_system_summary(&self) -> Option<String> {
        if let Some(ref sys_info) = self.system_info {
            Some(format!(
                "Windows版本: {}\n版本: {}\n内部版本号: {}\n激活状态: {}\nDirectX: {}\n制造商: {}\n型号: {}\nCPU: {}\n内存: {}\n显卡: {}",
                sys_info.windows_edition,
                get_windows_version_code(&sys_info.windows_edition),
                sys_info.windows_version,
                sys_info.windows_activation_status,
                sys_info.directx_version,
                sys_info.manufacturer,
                sys_info.model,
                sys_info.cpu,
                sys_info.memory_info,
                sys_info.gpu
            ))
        } else {
            None
        }
    }

    pub fn get_hardware_summary(&self) -> Option<String> {
        if let Some(ref scan_result) = self.scan_result {
            Some(format!("扫描到 {} 个硬件设备", scan_result.devices.len()))
        } else {
            None
        }
    }

    pub fn get_update_summary(&self) -> String {
        let total = self.update_candidates.len();
        let need_update = self.update_candidates.iter()
            .filter(|c| c.needs_update)
            .count();
            
        format!("总共 {} 个设备，其中 {} 个需要更新", total, need_update)
    }
}

// 根据Windows版本名称推断版本代号（如21H1, 21H2等）
fn get_windows_version_code(windows_edition: &str) -> String {
    // 根据Windows版本名称判断版本代号
    if windows_edition.contains("2021") || windows_edition.contains("LTSC 2021") {
        "21H2".to_string()  // LTSC 2021基于21H2
    } else if windows_edition.contains("2019") || windows_edition.contains("LTSC 2019") {
        "1809".to_string()  // LTSC 2019基于1809
    } else if windows_edition.contains("20H2") {
        "20H2".to_string()
    } else if windows_edition.contains("2004") {
        "2004".to_string()
    } else if windows_edition.contains("1909") {
        "1909".to_string()
    } else if windows_edition.contains("1903") {
        "1903".to_string()
    } else if windows_edition.contains("1809") {
        "1809".to_string()
    } else if windows_edition.contains("1803") {
        "1803".to_string()
    } else if windows_edition.contains("1709") {
        "1709".to_string()
    } else if windows_edition.contains("1703") {
        "1703".to_string()
    } else if windows_edition.contains("1607") {
        "1607".to_string()
    } else {
        // 如果无法识别，尝试从版本字符串中提取
        extract_version_code_from_string(windows_edition)
    }
}

// 从Windows版本字符串中提取版本代号的辅助函数
fn extract_version_code_from_string(windows_edition: &str) -> String {
    // 检查是否包含Hx模式的版本号，如21H1, 21H2等
    for i in 0..windows_edition.len() {
        if i + 4 <= windows_edition.len() {
            let substr = &windows_edition[i..i+4];
            if substr.chars().nth(2).unwrap_or(' ') == 'H' {
                let first = substr.chars().nth(0).unwrap_or(' ');
                let second = substr.chars().nth(1).unwrap_or(' ');
                let fourth = substr.chars().nth(3).unwrap_or(' ');
                if first.is_ascii_digit() && second.is_ascii_digit() && fourth.is_ascii_digit() {
                    return substr.to_string();
                }
            }
        }
    }
    
    "未知".to_string()
}