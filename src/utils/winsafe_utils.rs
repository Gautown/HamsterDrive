//! WinSafe中文处理工具模块
//! 标准写法：使用Rust标准库的encode_utf16()方法转换UTF-8到UTF-16



/// 通用的WinSafe字符串转换函数 - 一行调用解决所有动态中文问题
/// 
/// 核心原理:
/// 1. Rust的String/&str都实现了encode_utf16()方法，可以将UTF-8字符串转为UTF-16编码的u16迭代器
/// 2. Windows的所有字符串都要求以0作为终止符，所以需要在迭代器末尾追加一个0
/// 3. 最后将迭代器转为Vec<u16>，即可直接传给WinSafe的所有API
pub fn win_string(s: &str) -> Vec<u16> {
    s.encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

/// 验证包含中文的路径是否有效
pub fn validate_path_for_chinese_chars(path: &str) -> bool {
    // 确保路径可以被正确转换为宽字符串
    let wide_path = win_string(path);
    !wide_path.is_empty() && wide_path.len() < 32768 // Windows最大路径长度限制
}

/// 将Rust字符串安全转换为Windows兼容的宽字符串
pub fn to_windows_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_conversion() {
        let chinese_text = "仓鼠驱动管家";
        let wide = win_string(chinese_text);
        assert!(!wide.is_empty());
        assert_eq!(wide[wide.len()-1], 0); // 确保以0结尾
    }

    #[test]
    fn test_path_validation() {
        let path = "C:\\驱动程序\\仓鼠驱动管家\\driver.inf";
        assert!(validate_path_for_chinese_chars(path));
    }
}