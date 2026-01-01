//! 解析器工厂
//!
//! 负责创建和管理不同厂商的驱动解析器

use std::collections::HashMap;
use crate::driver::fetcher::parsers::{DriverParser, nvidia_parser::NvidiaParser, intel_parser::IntelParser, amd_parser::AmdParser, realtek_parser::RealtekParser, generic_parser::GenericParser};
use std::boxed::Box;

pub struct ParserFactory;

impl ParserFactory {
    /// 根据厂商名称获取对应的解析器
    pub fn get_parser(vendor: &str) -> Box<dyn DriverParser + Send + Sync> {
        match vendor.to_lowercase().as_str() {
            "nvidia" | "英伟达" => Box::new(NvidiaParser::new()),
            "intel" | "英特尔" => Box::new(IntelParser::new()),
            "amd" | "超威半导体" => Box::new(AmdParser::new()),
            "realtek" | "瑞昱" => Box::new(RealtekParser::new()),
            _ => Box::new(GenericParser::new()),
        }
    }

    /// 获取所有支持的厂商列表
    pub fn get_supported_vendors() -> Vec<&'static str> {
        vec!["nvidia", "intel", "amd", "realtek"]
    }

    /// 检查是否支持特定厂商
    pub fn supports_vendor(vendor: &str) -> bool {
        Self::get_supported_vendors()
            .iter()
            .any(|&v| v == vendor.to_lowercase().as_str())
    }

    /// 根据硬件ID自动选择合适的解析器
    pub fn get_parser_by_hardware_id(hardware_id: &str) -> Box<dyn DriverParser + Send + Sync> {
        let hardware_id_lower = hardware_id.to_lowercase();
        
        if hardware_id_lower.contains("nvidia") || hardware_id_lower.contains("10de") {
            // 10de 是 NVIDIA 的 PCI vendor ID
            Box::new(NvidiaParser::new())
        } else if hardware_id_lower.contains("intel") || hardware_id_lower.contains("8086") {
            // 8086 是 Intel 的 PCI vendor ID
            Box::new(IntelParser::new())
        } else if hardware_id_lower.contains("amd") || hardware_id_lower.contains("1002") {
            // 1002 是 AMD 的 PCI vendor ID
            Box::new(AmdParser::new())
        } else if hardware_id_lower.contains("realtek") || hardware_id_lower.contains("10ec") {
            // 10ec 是 Realtek 的 PCI vendor ID
            Box::new(RealtekParser::new())
        } else {
            Box::new(GenericParser::new())
        }
    }

    /// 创建所有解析器的映射
    pub fn create_all_parsers() -> HashMap<String, Box<dyn DriverParser + Send + Sync>> {
        let mut parsers: HashMap<String, Box<dyn DriverParser + Send + Sync>> = HashMap::new();
        
        parsers.insert("nvidia".to_string(), Box::new(NvidiaParser::new()) as Box<dyn DriverParser + Send + Sync>);
        parsers.insert("intel".to_string(), Box::new(IntelParser::new()) as Box<dyn DriverParser + Send + Sync>);
        parsers.insert("amd".to_string(), Box::new(AmdParser::new()) as Box<dyn DriverParser + Send + Sync>);
        parsers.insert("realtek".to_string(), Box::new(RealtekParser::new()) as Box<dyn DriverParser + Send + Sync>);
        parsers.insert("generic".to_string(), Box::new(GenericParser::new()) as Box<dyn DriverParser + Send + Sync>);
        
        parsers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parser() {
        let nvidia_parser = ParserFactory::get_parser("nvidia");
        assert!(nvidia_parser.get_vendor().to_lowercase().contains("nvidia"));

        let intel_parser = ParserFactory::get_parser("intel");
        assert!(intel_parser.get_vendor().to_lowercase().contains("intel"));

        let generic_parser = ParserFactory::get_parser("unknown_vendor");
        assert!(generic_parser.get_vendor().to_lowercase().contains("generic"));
    }

    #[test]
    fn test_get_parser_by_hardware_id() {
        let nvidia_parser = ParserFactory::get_parser_by_hardware_id("PCI\\VEN_10DE&DEV_1C82");
        assert!(nvidia_parser.get_vendor().to_lowercase().contains("nvidia"));

        let intel_parser = ParserFactory::get_parser_by_hardware_id("PCI\\VEN_8086&DEV_1234");
        assert!(intel_parser.get_vendor().to_lowercase().contains("intel"));
    }

    #[test]
    fn test_supports_vendor() {
        assert!(ParserFactory::supports_vendor("nvidia"));
        assert!(ParserFactory::supports_vendor("intel"));
        assert!(!ParserFactory::supports_vendor("unknown"));
    }
}