

use hamster_drivers::ui::HamsterDriveApp;

#[tokio::main]
async fn main() {
    env_logger::init(); // 初始化日志
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(1200.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "仓鼠驱动管家",
        options,
        Box::new(|cc| {
            Box::new(HamsterDriveApp::new(cc))
        }),
    ).expect("启动GUI失败");
}