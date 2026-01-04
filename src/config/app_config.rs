//! 应用程序配置
use crate::utils::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub data_dir: String,
    pub temp_dir: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            app_name: "HamsterDrivers".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: "".to_string(),
            temp_dir: "".to_string(),
            log_level: "info".to_string(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
