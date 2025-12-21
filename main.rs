mod scan;
mod backup;
mod restore;
mod update;
mod driver_db;
mod signature;
mod error;
mod list;
mod gui;

fn main() {
    // 初始化日志、错误处理等
    // ...existing code...
    let driver_path = "C:\\Windows\\System32\\drivers\\example.sys";
    match signature::verify_driver_signature(driver_path) {
        Ok(valid) => println!("驱动签名验证结果: {}", valid),
        Err(e) => println!("错误: {}", e),
    }
    gui::run();
}