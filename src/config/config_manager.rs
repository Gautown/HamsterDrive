//! 配置管理器
//!
//! 负责加载、保存和管理应用程序配置

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use toml;
use crate::utils::error::{HamsterError, Result};
use crate::config::app_config::AppConfig;
use crate::config::download_config::DownloadConfig;
use crate::config::scanner_config::ScannerConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigManager {
    pub app_config: AppConfig,
    pub download_config: DownloadConfig,
    pub scanner_config: ScannerConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            app_config: AppConfig::default(),
            download_config: DownloadConfig::default(),
            scanner_config: ScannerConfig::default(),
        }
    }

    /// 从文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| HamsterError::ConfigError(format!("读取配置文件失败: {}", e)))?;
        
        let config: ConfigManager = toml::from_str(&content)
            .map_err(|e| HamsterError::ConfigError(format!("解析配置文件失败: {}", e)))?;
        
        Ok(config)
    }

    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| HamsterError::ConfigError(format!("序列化配置失败: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| HamsterError::ConfigError(format!("写入配置文件失败: {}", e)))?;
        
        Ok(())
    }

    /// 从默认值创建配置
    pub fn with_defaults() -> Self {
        Self::new()
    }

    /// 设置应用程序配置
    pub fn with_app_config(mut self, config: AppConfig) -> Self {
        self.app_config = config;
        self
    }

    /// 设置下载配置
    pub fn with_download_config(mut self, config: DownloadConfig) -> Self {
        self.download_config = config;
        self
    }

    /// 设置扫描配置
    pub fn with_scanner_config(mut self, config: ScannerConfig) -> Self {
        self.scanner_config = config;
        self
    }

    /// 验证所有配置的有效性
    pub fn validate(&self) -> Result<()> {
        self.download_config.validate()
            .map_err(|e| HamsterError::ConfigError(format!("下载配置验证失败: {}", e)))?;
        
        self.scanner_config.validate()
            .map_err(|e| HamsterError::ConfigError(format!("扫描配置验证失败: {}", e)))?;
        
        Ok(())
    }

    /// 获取默认配置路径
    pub fn default_config_path() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        path.push("config");
        path.push("app.toml");
        path
    }

    /// 初始化默认配置文件（如果不存在）
    pub fn init_default_config<P: AsRef<Path>>(path: P) -> Result<()> {
        let config = Self::new();
        if !path.as_ref().exists() {
            config.save_to_file(path)?;
        }
        Ok(())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}