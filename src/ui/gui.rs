//! 图形用户界面
use crate::utils::error::Result;
use eframe::egui;

// Windows API 相关导入
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
#[cfg(windows)]
use winapi::um::winuser::{NONCLIENTMETRICSW, SystemParametersInfoW, SPI_GETNONCLIENTMETRICS};
#[cfg(windows)]
use winapi::shared::minwindef::UINT;

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
            // 获取系统默认字体
            let mut fonts = egui::FontDefinitions::default();
            
            #[cfg(windows)]
            {
                if let Some(system_font) = get_system_default_font_family() {
                    // 为比例字体族添加系统字体作为首选
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, system_font.clone());
                    
                    // 为等宽字体族添加系统字体作为首选
                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .insert(0, system_font);
                }
            }
            
            // 应用字体设置
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(HamsterDriveApp::default()))
        }),
    )
    .map_err(|e| crate::utils::error::HamsterError::Unknown(e.to_string()))
}

/// 获取系统默认字体族
#[cfg(windows)]
fn get_system_default_font_family() -> Option<String> {
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
            
            // 验证字体名称是否有效
            if !font_name.is_empty() && !font_name.contains('\0') {
                return Some(font_name);
            }
        }
    }
    
    None
}

// 非Windows平台的备选实现
#[cfg(not(windows))]
fn get_system_default_font_family() -> Option<String> {
    None // 在非Windows平台上返回None，使用egui默认字体
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
