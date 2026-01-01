//! 哈希验证器
//!
//! 负责验证下载文件完整性的组件

use std::fs::File;
use std::io::{Read, BufReader};
use sha2::{Sha256, Digest};
use crate::utils::error::{HamsterError, Result};

pub struct HashVerifier;

impl HashVerifier {
    pub fn new() -> Self {
        Self
    }

    /// 计算文件的SHA256哈希值
    pub fn calculate_file_hash<P: AsRef<std::path::Path>>(file_path: P) -> Result<String> {
        let file = File::open(file_path)
            .map_err(|e| HamsterError::IoError(format!("打开文件失败: {}", e)))?;
        let mut reader = BufReader::new(file);
        
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192]; // 8KB buffer
        
        loop {
            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| HamsterError::IoError(format!("读取文件失败: {}", e)))?;
            
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
        
        let hash_result = hasher.finalize();
        Ok(format!("{:x}", hash_result))
    }

    /// 验证文件哈希值
    pub fn verify_file_hash<P: AsRef<std::path::Path>>(file_path: P, expected_hash: &str) -> Result<bool> {
        let actual_hash = Self::calculate_file_hash(file_path)?;
        Ok(actual_hash.to_lowercase() == expected_hash.to_lowercase())
    }

    /// 计算字节数组的SHA256哈希值
    pub fn calculate_bytes_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash_result = hasher.finalize();
        format!("{:x}", hash_result)
    }

    /// 验证字节数组的哈希值
    pub fn verify_bytes_hash(data: &[u8], expected_hash: &str) -> bool {
        let actual_hash = Self::calculate_bytes_hash(data);
        actual_hash.to_lowercase() == expected_hash.to_lowercase()
    }

    /// 支持的哈希算法
    pub fn supported_algorithms() -> Vec<&'static str> {
        vec!["SHA256", "MD5", "SHA1"] // 可以扩展支持更多算法
    }

    /// 验证文件的MD5哈希值（如果需要）
    #[allow(dead_code)]
    pub fn verify_file_md5<P: AsRef<std::path::Path>>(file_path: P, expected_md5: &str) -> Result<bool> {
        
        let file = File::open(file_path)
            .map_err(|e| HamsterError::IoError(format!("打开文件失败: {}", e)))?;
        let mut reader = BufReader::new(file);
        
        let mut context = md5::Context::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| HamsterError::IoError(format!("读取文件失败: {}", e)))?;
            
            if bytes_read == 0 {
                break;
            }
            
            context.consume(&buffer[..bytes_read]);
        }
        
        let hash_result = context.compute();
        let actual_md5 = format!("{:x}", hash_result);
        
        Ok(actual_md5.to_lowercase() == expected_md5.to_lowercase())
    }
}

// 为方便使用，提供一个简单的函数
pub fn verify_file_integrity<P: AsRef<std::path::Path>>(file_path: P, expected_hash: &str) -> Result<bool> {
    HashVerifier::verify_file_hash(file_path, expected_hash)
}