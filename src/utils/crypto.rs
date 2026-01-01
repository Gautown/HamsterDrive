//! 加密工具模块

use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::utils::error::{HamsterError, Result};

/// 计算字符串的SHA256哈希
pub fn sha256_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// 计算字节数组的SHA256哈希
pub fn sha256_bytes(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// 计算文件的SHA256哈希
pub fn sha256_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// 验证文件的SHA256哈希
pub fn verify_sha256(path: &Path, expected_hash: &str) -> Result<bool> {
    let actual_hash = sha256_file(path)?;
    Ok(actual_hash.eq_ignore_ascii_case(expected_hash))
}

/// Base64编码
pub fn base64_encode(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

/// Base64解码
pub fn base64_decode(input: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(input)
        .map_err(|e| HamsterError::ParseError(format!("Base64解码失败: {}", e)))
}

/// 生成随机字符串
pub fn generate_random_string(length: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    
    let mut result = String::with_capacity(length);
    let mut state = seed;
    
    for _ in 0..length {
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (state as usize) % chars.len();
        result.push(chars[index]);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_string() {
        let hash = sha256_string("hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_base64() {
        let original = b"Hello, World!";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }
}
