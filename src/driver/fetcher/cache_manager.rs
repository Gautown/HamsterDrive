//! 缓存管理器
use crate::types::driver_types::DriverInfo;
use crate::utils::error::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CacheManager {
    cache: HashMap<String, (DriverInfo, u64)>,
    ttl_seconds: u64,
}

impl CacheManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: HashMap::new(),
            ttl_seconds: 3600, // 1小时
        })
    }

    pub fn get_cached_driver(&self, key: &str) -> Result<Option<DriverInfo>> {
        if let Some((driver, timestamp)) = self.cache.get(key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now - timestamp <= self.ttl_seconds {
                return Ok(Some(driver.clone()));
            }
        }
        Ok(None)
    }

    pub fn cache_driver(&mut self, key: &str, driver: &DriverInfo) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.cache.insert(key.to_string(), (driver.clone(), now));
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.cache.clear();
        Ok(())
    }
}
