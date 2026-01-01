//! 图形用户界面
use crate::utils::error::Result;
use eframe::egui;


pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "仓鼠驱动管家",
        options,
        Box::new(|cc| {
            // 创建自定义字体
            let fonts = egui::FontDefinitions::default();
            
            // 为确保中文字符能正确显示，使用默认字体设置
            // egui会自动处理中文字体显示
            // 不指定特定字体名称，避免找不到字体的问题
            
            // 应用字体设置
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(HamsterDriveApp::default()))
        }),
    )
    .map_err(|e| crate::utils::error::HamsterError::Unknown(e.to_string()))
}



#[derive(Default)]
struct HamsterDriveApp;

impl eframe::App for HamsterDriveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("仓鼠驱动管家");
            ui.label("欢迎使用驱动管理工具！");
            
            if ui.button("扫描驱动").clicked() {
                // 扫描驱动逻辑
            }
        });
    }
}
