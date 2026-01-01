//! 数据库连接管理
use crate::utils::error::Result;

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}
