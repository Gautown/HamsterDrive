use crate::error::HamsterError;
use crate::scan::scan_outdated_drivers;
use crate::update::download_and_install_driver;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 一键更新所有驱动
pub async fn update_all_drivers() -> Result<UpdateResult, HamsterError> {
    let outdated_drivers = scan_outdated_drivers()?;
    
    if outdated_drivers.is_empty() {
        return Ok(UpdateResult {
            total: 0,
            success: 0,
            failed: 0,
            messages: vec!["没有检测到需要更新的驱动".to_string()],
        });
    }
    
    let total = outdated_drivers.len();
    let success = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let mut messages = Vec::new();
    
    messages.push(format!("开始更新 {} 个驱动程序...", total));
    
    for driver in &outdated_drivers {
        let success_count = Arc::clone(&success);
        let failed_count = Arc::clone(&failed);
        let driver_clone = driver.clone();
        
        match download_and_install_driver(&driver_clone, None, None).await {
            Ok(install_result) => {
                if install_result.success {
                    success_count.fetch_add(1, Ordering::SeqCst);
                    messages.push(format!("✓ 成功更新: {} (版本: {})", driver.name, driver.latest_version));
                } else {
                    failed_count.fetch_add(1, Ordering::SeqCst);
                    messages.push(format!("✗ 更新失败 {}: {}", driver.name, install_result.error_message.unwrap_or("未知错误".to_string())));
                }
            },
            Err(e) => {
                failed_count.fetch_add(1, Ordering::SeqCst);
                messages.push(format!("✗ 更新失败 {}: {}", driver.name, e));
            }
        }
    }
    
    let success_final = success.load(Ordering::SeqCst);
    let failed_final = failed.load(Ordering::SeqCst);
    
    messages.push(format!("更新完成: 成功 {}, 失败 {}, 总计 {}", success_final, failed_final, total));
    
    Ok(UpdateResult {
        total,
        success: success_final,
        failed: failed_final,
        messages,
    })
}

/// 更新结果统计
#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub messages: Vec<String>,
}

/// 检查并显示更新摘要
pub fn get_update_summary(result: &UpdateResult) -> String {
    format!(
        "驱动更新摘要:\n总计: {}\n成功: {}\n失败: {}\n",
        result.total, result.success, result.failed
    )
}

/// 获取更新进度百分比
pub fn get_update_progress(current: usize, total: usize) -> f32 {
    if total == 0 {
        100.0
    } else {
        (current as f32 / total as f32) * 100.0
    }
}
