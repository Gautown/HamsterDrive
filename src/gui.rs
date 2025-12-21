use eframe::egui;
use crate::{scan, backup, restore, update, list};

pub fn run() -> Result<(), eframe::Error> {
    let app = HamsterDriveApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "HamsterDrive 驱动管理",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HamsterDrive 驱动管理");
            
            // 创建一个水平布局来放置按钮
            ui.horizontal(|ui| {
                // 扫描硬件按钮
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
                
                // 检测驱动更新按钮
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
                
                // 驱动备份按钮
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
                
                // 驱动恢复按钮
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
                
                // 驱动列表按钮
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
            });
            
            // 显示硬件信息
            if !self.hardware_info.is_empty() {
                ui.separator();
                ui.label("硬件信息:");
                for item in &self.hardware_info {
                    ui.label(item);
                }
            }
            
            // 显示更新信息
            if !self.update_list.is_empty() {
                ui.separator();
                ui.label("可用更新:");
                for update in &self.update_list {
                    ui.label(update);
                }
            }
            
            // 显示备份状态
            if !self.backup_status.is_empty() {
                ui.separator();
                ui.label(&self.backup_status);
            }
            
            // 显示恢复状态
            if !self.restore_status.is_empty() {
                ui.separator();
                ui.label(&self.restore_status);
            }
            
            // 显示驱动列表
            if !self.driver_list.is_empty() {
                ui.separator();
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
