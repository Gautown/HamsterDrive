//! 下载管理模块
//!
//! 本模块负责驱动程序的下载管理

pub mod manager;
pub mod aria2_manager;
pub mod http_downloader;
pub mod progress_tracker;
pub mod speed_limiter;
pub mod hash_verifier;

pub use manager::DownloadManager;
