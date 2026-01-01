use hamsterdrive::{init_logging, DriverUpdaterCore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HamsterDrive - Windows驱动管理工具（GUI版本）");
    
    // 初始化日志
    init_logging()?;
    
    // 创建核心控制器
    let mut core = DriverUpdaterCore::new();
    core.initialize().await?;
    
    // 启动GUI
    hamsterdrive::ui::run_gui()?;
    
    Ok(())
}
