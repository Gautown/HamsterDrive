use eframe::egui;
use crate::{scan, backup, restore, update, list};

pub fn run() -> Result<(), eframe::Error> {
    let app = HamsterDriveApp::default();
    let mut native_options = eframe::NativeOptions::default();
    
    // 配置字体以支持中文显示
    native_options.renderer = eframe::Renderer::Glow;
    
    // 禁用拖放功能以避免COM初始化冲突
    native_options.viewport = egui::ViewportBuilder::default()
        .with_drag_and_drop(false);
    
    eframe::run_native(
        "仓鼠驱动护士",
        native_options,
        Box::new(|cc| {
            // 设置中文字体
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

/// 设置自定义字体以支持中文显示
fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontFamily::Proportional;
    use egui::FontId;
    use egui::TextStyle::*;
    
    // 获取默认字体定义
    let mut fonts = egui::FontDefinitions::default();
    
    // 重要：添加思源黑体字体支持
    fonts.font_data.insert(
        "SourceHanSans".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/SourceHanSansSC-Regular.otf")),
    );
    
    // 将思源黑体设置为默认字体
    fonts.families.entry(Proportional).or_default().insert(0, "SourceHanSans".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "SourceHanSans".to_owned());
    
    ctx.set_fonts(fonts);
    
    // 设置默认文本样式
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(18.0, Proportional)),
        (Body, FontId::new(14.0, Proportional)),
        (Monospace, FontId::new(12.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

#[derive(Default)]
struct HamsterDriveApp {
    hardware_info: Vec<String>,
    driver_list: Vec<String>,
    update_list: Vec<String>,
    backup_status: String,
    restore_status: String,
}

impl eframe::App for HamsterDriveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建顶部面板
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("仓鼠驱动护士");
        });
        
        // 创建左侧边栏
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // 左侧菜单按钮
            if ui.button("扫描硬件").clicked() {
                match scan::scan_hardware() {
                    Ok(hardware) => {
                        self.hardware_info = hardware;
                    },
                    Err(e) => {
                        self.hardware_info.clear();
                        self.hardware_info.push(format!("错误: {}", e));
                    }
                }
            }
            
            if ui.button("检测驱动更新").clicked() {
                match update::check_updates() {
                    Ok(updates) => {
                        self.update_list = updates;
                    },
                    Err(e) => {
                        self.update_list.clear();
                        self.update_list.push(format!("错误: {}", e));
                    }
                }
            }
            
            if ui.button("驱动备份").clicked() {
                match backup::backup_drivers(true) {
                    Ok(_) => {
                        self.backup_status = "备份成功".to_string();
                    },
                    Err(e) => {
                        self.backup_status = format!("备份失败: {}", e);
                    }
                }
            }
            
            if ui.button("驱动恢复").clicked() {
                match restore::restore_drivers() {
                    Ok(_) => {
                        self.restore_status = "恢复成功".to_string();
                    },
                    Err(e) => {
                        self.restore_status = format!("恢复失败: {}", e);
                    }
                }
            }
            
            if ui.button("驱动列表").clicked() {
                match list::show_installed_drivers() {
                    Ok(drivers) => {
                        self.driver_list = drivers;
                    },
                    Err(e) => {
                        self.driver_list.clear();
                        self.driver_list.push(format!("错误: {}", e));
                    }
                }
            }
            
            ui.add_space(10.0);
            ui.separator();
            ui.label("状态信息");
        });
        
        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示硬件信息
            if !self.hardware_info.is_empty() {
                ui.label("硬件信息:");
                for item in &self.hardware_info {
                    ui.label(item);
                }
                ui.add_space(5.0);
            }
            
            // 显示更新信息
            if !self.update_list.is_empty() {
                ui.label("可用更新:");
                for update in &self.update_list {
                    ui.label(update);
                }
                ui.add_space(5.0);
            }
            
            // 显示备份状态
            if !self.backup_status.is_empty() {
                ui.label(&self.backup_status);
                ui.add_space(5.0);
            }
            
            // 显示恢复状态
            if !self.restore_status.is_empty() {
                ui.label(&self.restore_status);
                ui.add_space(5.0);
            }
            
            // 显示驱动列表
            if !self.driver_list.is_empty() {
                ui.label("已安装驱动:");
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for driver in &self.driver_list {
                        ui.label(driver);
                    }
                });
            }
        });
    }
}
