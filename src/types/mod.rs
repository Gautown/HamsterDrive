//! 全局类型定义模块
//!
//! 本模块包含项目中使用的所有核心类型定义

pub mod hardware_types;
pub mod driver_types;
pub mod system_types;
pub mod ui_types;

// 导出所有类型
pub use hardware_types::*;
pub use driver_types::*;
pub use system_types::*;
pub use ui_types::*;
