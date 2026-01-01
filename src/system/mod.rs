//! 系统信息模块
//!
//! 本模块负责采集系统信息

pub mod os_info;
pub mod windows_info;
pub mod activation;
pub mod hardware_summary;

pub use os_info::*;
pub use hardware_summary::*;
