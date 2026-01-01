//! 核心控制器模块
//!
//! 本模块包含应用程序的核心控制逻辑

pub mod controller;
pub mod state;
pub mod event_loop;

pub use controller::DriverUpdaterCore;
pub use state::AppState;
