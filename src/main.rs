mod error;
mod gui;
mod scan;
mod backup;
mod restore;
mod update;
mod driver_db;
mod signature;
mod list;
mod uninstall;
mod batch_update;
mod offline_scan;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HamsterDrive - Windows驱动管理工具（GUI版本）");
    
    // 启动GUI
    gui::run()?;
    
    Ok(())
}
