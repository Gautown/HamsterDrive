//! 驱动解析器模块

pub mod parser_trait;
pub mod parser_factory;
pub mod nvidia_parser;
pub mod intel_parser;
pub mod amd_parser;
pub mod realtek_parser;
pub mod generic_parser;

pub use parser_trait::DriverParser;
