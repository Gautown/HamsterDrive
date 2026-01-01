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
            let mut fonts = egui::FontDefinitions::default();
            
            // 尝试获取系统默认字体作为后备
            #[cfg(windows)]
            {
                if let Some(system_font) = get_system_font_family() {
                    // 将系统字体添加为后备字体，而不是完全替换默认字体
                    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                        family.insert(0, system_font.clone());
                    } else {
                        fonts.families.insert(
                            egui::FontFamily::Proportional,
                            vec![system_font.clone()]
                        );
                    }
                    
                    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                        family.insert(0, system_font);
                    } else {
                        fonts.families.insert(
                            egui::FontFamily::Monospace,
                            vec![system_font]
                        );
                    }
                }
            }
            
            // 应用字体设置
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(HamsterDriveApp::default()))
        }),
    )
    .map_err(|e| crate::utils::error::HamsterError::Unknown(e.to_string()))
}

#[cfg(windows)]
fn get_system_font_family() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::winuser::{NONCLIENTMETRICSW, SystemParametersInfoW};
    use winapi::um::winuser::SPI_GETNONCLIENTMETRICS;
    use winapi::shared::minwindef::UINT;
    
    unsafe {
        let mut ncm: NONCLIENTMETRICSW = std::mem::zeroed();
        ncm.cbSize = std::mem::size_of::<NONCLIENTMETRICSW>() as UINT;
        
        if SystemParametersInfoW(
            SPI_GETNONCLIENTMETRICS,
            std::mem::size_of::<NONCLIENTMETRICSW>() as UINT,
            &mut ncm as *mut _ as *mut _,
            0,
        ) != 0 {
            // 将字体名称转换为字符串
            let font_name = OsString::from_wide(&ncm.lfMessageFont.lfFaceName)
                .to_string_lossy()
                .to_string();
            
            // 验证字体名称是否有效，如果包含空字符或无效字符，则跳过
            if !font_name.is_empty() && !font_name.contains('\0') {
                // 检查是否为常见的系统字体，如果不在可用字体列表中，则返回None
                return Some(font_name);
            }
        }
    }
    
    // 如果无法获取系统字体，返回None让egui使用默认字体
    None
}

#[cfg(not(windows))]
fn get_system_font_family() -> Option<String> {
    // 对于非Windows系统，返回None让egui使用默认字体
    None
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
