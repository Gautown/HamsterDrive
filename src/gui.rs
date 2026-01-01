use eframe::egui;
use crate::{scan, backup, restore, update, list};

/// 运行GUI应用程序
pub fn run() -> Result<(), eframe::Error> {
    let app = HamsterDriveApp::default();
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true)  // 启用拖放功能
            .with_decorations(true)   // 启用窗口装饰（系统默认标题栏）
            .with_inner_size((1024.0, 768.0))  // 设置初始窗口大小
            .with_min_inner_size((800.0, 600.0)),  // 设置最小窗口大小
        ..Default::default()
    };
    
    eframe::run_native(
        "仓鼠驱动管家",
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

    // 创建字体定义
    let mut fonts = egui::FontDefinitions::default();
    
    // 为确保中文字符能正确显示，添加中文字符集支持
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "emoji-icon-font".to_owned()); // 使用支持中文的字体
    
    // 应用字体设置
    ctx.set_fonts(fonts);
    
    // 设置文本样式
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
use std::sync::mpsc;
use std::thread;
use crate::scan::DriverStatus;

/// 定义当前显示的视图类型
#[derive(Debug, Clone, PartialEq, Default)]
enum CurrentView {
    #[default]
    SystemInfo,
    DriverUpdate,
    DriverBackup,
    DriverRestore,
    DriverList,
    DriverScan,
}



// 检查异步操作结果
impl HamsterDriveApp {
    fn check_async_operations(&mut self) {
        // 检查系统信息获取结果
        if let Some(ref rx) = self.system_info_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(info) => {
                        self.system_info = info;
                        self.getting_system_info = false;
                    },
                    Err(_e) => {
                        self.system_info.clear();
                        self.system_info.push(format!("错误: {}", _e));
                        self.getting_system_info = false;
                    }
                }
                self.system_info_rx = None;
            }
        }
        
        // 检查驱动扫描结果
        if let Some(ref rx) = self.driver_scan_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(drivers) => {
                        println!("扫描完成，找到 {} 个驱动", drivers.len());
                        for (i, driver) in drivers.iter().enumerate() {
                            println!("驱动 {}: {} - 版本: {}", i, driver.name, driver.current_version);
                        }
                        self.driver_scan_results = drivers;
                        self.scanning = false;
                    },
                    Err(_e) => {
                        println!("扫描错误: {}", _e);
                        self.driver_scan_results.clear();
                        // 添加错误信息作为驱动信息
                        self.driver_scan_results.push(scan::DriverInfo {
                            name: "扫描错误".to_string(),
                            current_version: "".to_string(),
                            latest_version: "".to_string(),
                            hardware_id: "".to_string(),
                            download_url: "".to_string(),
                            size: "".to_string(),
                            release_date: "".to_string(),
                            status: scan::DriverStatus::NotInstalled,
                        });
                        self.scanning = false;
                    }
                }
                self.driver_scan_rx = None;
            }
        }
        
        // 检查硬件扫描结果
        if let Some(ref rx) = self.scan_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(hardware) => {
                        self.hardware_info = hardware;
                        self.scanning = false;
                    },
                    Err(_e) => {
                        self.hardware_info.clear();
                        self.hardware_info.push(format!("错误: {}", _e));
                        self.scanning = false;
                    }
                }
                self.scan_rx = None;
            }
        }
        
        // 检查驱动更新结果
        if let Some(ref rx) = self.update_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(updates) => self.update_list = updates,
                    Err(_e) => {
                        self.update_list.clear();
                        // 添加错误信息作为驱动信息
                        self.update_list.push(scan::DriverInfo {
                            name: "更新检查错误".to_string(),
                            current_version: "".to_string(),
                            latest_version: "".to_string(),
                            hardware_id: "".to_string(),
                            download_url: "".to_string(),
                            size: "".to_string(),
                            release_date: "".to_string(),
                            status: scan::DriverStatus::NotInstalled,
                        });
                    }
                }
                self.update_rx = None;
                self.checking_updates = false;
            }
        }
        
        // 检查备份结果
        if let Some(ref rx) = self.backup_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(_) => self.backup_status = "备份成功".to_string(),
                    Err(_e) => self.backup_status = format!("备份失败: {}", _e),
                }
                self.backup_rx = None;
                self.backing_up = false;
            }
        }
        
        // 检查恢复结果
        if let Some(ref rx) = self.restore_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(_) => self.restore_status = "恢复成功".to_string(),
                    Err(_e) => self.restore_status = format!("恢复失败: {}", _e),
                }
                self.restore_rx = None;
                self.restoring = false;
            }
        }
        
        // 检查驱动列表结果
        if let Some(ref rx) = self.list_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(drivers) => self.driver_list = drivers,
                    Err(_e) => {
                        self.driver_list.clear();
                        // 添加错误信息作为驱动信息
                        self.driver_list.push(scan::DriverInfo {
                            name: "列表错误".to_string(),
                            current_version: "".to_string(),
                            latest_version: "".to_string(),
                            hardware_id: "".to_string(),
                            download_url: "".to_string(),
                            size: "".to_string(),
                            release_date: "".to_string(),
                            status: scan::DriverStatus::NotInstalled,
                        });
                    }
                }
                self.list_rx = None;
                self.loading_drivers = false;
            }
        }
    }
    

    
    // 开始检查更新
    fn start_check_updates(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.update_rx = Some(rx);
        self.checking_updates = true;
        
        std::thread::spawn(move || {
            let result = update::check_updates();
            let _ = tx.send(result);
        });
    }
    
    // 开始备份驱动
    fn start_backup_drivers(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.backup_rx = Some(rx);
        self.backing_up = true;
        
        std::thread::spawn(move || {
            let result = backup::backup_drivers(true);
            let _ = tx.send(result);
        });
    }
    
    // 开始恢复驱动
    fn start_restore_drivers(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.restore_rx = Some(rx);
        self.restoring = true;
        
        std::thread::spawn(move || {
            let result = restore::restore_drivers();
            let _ = tx.send(result);
        });
    }
    
    // 开始显示驱动列表
    fn start_show_installed_drivers(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.list_rx = Some(rx);
        self.loading_drivers = true;
        
        thread::spawn(move || {
            let result = list::show_installed_drivers();
            let _ = tx.send(result);
        });
    }
    
    // 开始获取系统信息
    fn start_get_system_info(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.system_info_rx = Some(rx);
        self.getting_system_info = true;
        
        thread::spawn(move || {
            let result = scan::get_system_info();
            let _ = tx.send(result);
        });
    }
}



#[derive(Default)]
struct HamsterDriveApp {
    hardware_info: Vec<String>,
    system_info: Vec<String>,
    driver_list: Vec<scan::DriverInfo>,
    update_list: Vec<scan::DriverInfo>,
    backup_status: String,
    restore_status: String,
    initialized: bool,
    current_view: CurrentView,
    // 用于异步操作的通道
    scan_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    driver_scan_rx: Option<std::sync::mpsc::Receiver<Result<Vec<scan::DriverInfo>, crate::error::HamsterError>>>,
    system_info_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    backup_rx: Option<std::sync::mpsc::Receiver<Result<(), crate::error::HamsterError>>>,
    restore_rx: Option<std::sync::mpsc::Receiver<Result<(), crate::error::HamsterError>>>,
    update_rx: Option<std::sync::mpsc::Receiver<Result<Vec<scan::DriverInfo>, crate::error::HamsterError>>>,
    list_rx: Option<std::sync::mpsc::Receiver<Result<Vec<scan::DriverInfo>, crate::error::HamsterError>>>,
    // 标记操作是否在进行中
    scanning: bool,
    getting_system_info: bool,
    backing_up: bool,
    restoring: bool,
    checking_updates: bool,
    loading_drivers: bool,
    // 存储驱动信息
    driver_scan_results: Vec<scan::DriverInfo>,

}

impl eframe::App for HamsterDriveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 检查是否有异步操作完成
        self.check_async_operations();
        
        // 初始化时自动获取系统信息
        if !self.initialized {
            // 使用异步方式获取系统信息以避免界面冻结
            self.current_view = CurrentView::SystemInfo;
            self.start_get_system_info();
            self.initialized = true;
        }
        
        // 创建左侧边栏
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(180.0)
            .show(ctx, |ui| {
            // 左侧菜单按钮
            if ui.button("系统信息").clicked() && !self.scanning {
                self.current_view = CurrentView::SystemInfo;
                self.start_get_system_info();
            }
            
            if ui.button("驱动扫描").clicked() && !self.scanning {
                self.current_view = CurrentView::DriverScan;
                
                let (tx, rx) = std::sync::mpsc::channel();
                self.driver_scan_rx = Some(rx);
                self.scanning = true;
                
                std::thread::spawn(move || {
                    let result = scan::scan_outdated_drivers();
                    let _ = tx.send(result);
                });
            }
            
            if ui.button("驱动更新").clicked() && !self.checking_updates {
                self.current_view = CurrentView::DriverUpdate;
                self.start_check_updates();
            }
            
            if ui.button("驱动备份").clicked() && !self.backing_up {
                self.current_view = CurrentView::DriverBackup;
                self.start_backup_drivers();
            }
            
            if ui.button("驱动恢复").clicked() && !self.restoring {
                self.current_view = CurrentView::DriverRestore;
                self.start_restore_drivers();
            }
            
            if ui.button("驱动列表").clicked() && !self.loading_drivers {
                self.current_view = CurrentView::DriverList;
                self.start_show_installed_drivers();
            }
            
            ui.add_space(10.0);
            ui.separator();
            ui.label("状态信息");
            
            ui.add_space(10.0); // 添加一些空间
            ui.separator(); // 添加分隔线
            ui.heading("仓鼠驱动管家"); // 将标题移到底部
        });
        
        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示操作状态
            if self.scanning {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "🔍 正在扫描驱动程序...");
                ui.add_space(10.0);
            }
            if self.checking_updates {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "🔄 正在检查驱动更新...");
                ui.add_space(10.0);
            }
            if self.backing_up {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "💾 正在备份驱动...");
                ui.add_space(10.0);
            }
            if self.restoring {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "📂 正在恢复驱动...");
                ui.add_space(10.0);
            }
            if self.loading_drivers {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "📋 正在加载驱动列表...");
                ui.add_space(10.0);
            }
            if self.getting_system_info {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "ℹ️ 正在获取系统信息...");
                ui.add_space(10.0);
            }
            
            // 根据当前视图显示相应的内容
            match &self.current_view {
                CurrentView::SystemInfo => {
                    if !self.system_info.is_empty() {
                        ui.label("计算机的基本信息:");
                        for item in &self.system_info {
                            ui.label(item);
                        }
                        ui.add_space(5.0);
                    }
                },

                CurrentView::DriverUpdate => {
                    if !self.update_list.is_empty() {
                        ui.label("可用更新:");
                        
                        // 显示更新列表表格
                        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .striped(true)
                                .column(egui_extras::Column::exact(150.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(80.0))
                                .column(egui_extras::Column::remainder())
                                .header(20.0, |mut header| {
                                    header.col(|ui| { ui.strong("驱动名称"); });
                                    header.col(|ui| { ui.strong("当前版本"); });
                                    header.col(|ui| { ui.strong("最新版本"); });
                                    header.col(|ui| { ui.strong("状态"); });
                                    header.col(|ui| { ui.strong("操作"); });
                                })
                                .body(|mut body| {
                                    for driver in &self.update_list {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| { ui.label(&driver.name); });
                                            row.col(|ui| { ui.label(&driver.current_version); });
                                            row.col(|ui| { ui.label(&driver.latest_version); });
                                            row.col(|ui| { 
                                                let status_text = match driver.status {
                                                    DriverStatus::Outdated => "需更新",
                                                    DriverStatus::UpToDate => "最新",
                                                    DriverStatus::NotInstalled => "未安装",
                                                };
                                                ui.label(status_text);
                                            });
                                            row.col(|ui| { 
                                                if driver.status == DriverStatus::Outdated && !driver.download_url.is_empty() && ui.button("更新").clicked() {
                                                    // 在这里可以触发驱动更新操作
                                                    println!("准备更新驱动: {}", driver.name);
                                                }
                                            });
                                        });
                                    }
                                });
                        });
                        ui.add_space(5.0);
                    }
                },
                CurrentView::DriverBackup => {
                    if !self.backup_status.is_empty() {
                        ui.label(&self.backup_status);
                        ui.add_space(5.0);
                    }
                },
                CurrentView::DriverRestore => {
                    if !self.restore_status.is_empty() {
                        ui.label(&self.restore_status);
                        ui.add_space(5.0);
                    }
                },
                CurrentView::DriverList => {
                    if !self.driver_list.is_empty() {
                        ui.label("已安装驱动:");
                        
                        // 显示驱动列表表格
                        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .striped(true)
                                .column(egui_extras::Column::exact(150.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(80.0))
                                .column(egui_extras::Column::remainder())
                                .header(20.0, |mut header| {
                                    header.col(|ui| { ui.strong("驱动名称"); });
                                    header.col(|ui| { ui.strong("当前版本"); });
                                    header.col(|ui| { ui.strong("最新版本"); });
                                    header.col(|ui| { ui.strong("状态"); });
                                    header.col(|ui| { ui.strong("操作"); });
                                })
                                .body(|mut body| {
                                    for driver in &self.driver_list {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| { ui.label(&driver.name); });
                                            row.col(|ui| { ui.label(&driver.current_version); });
                                            row.col(|ui| { ui.label(&driver.latest_version); });
                                            row.col(|ui| { 
                                                let status_text = match driver.status {
                                                    DriverStatus::Outdated => "需更新",
                                                    DriverStatus::UpToDate => "最新",
                                                    DriverStatus::NotInstalled => "未安装",
                                                };
                                                ui.label(status_text);
                                            });
                                            row.col(|ui| { 
                                                if driver.status == DriverStatus::Outdated && !driver.download_url.is_empty() && ui.button("更新").clicked() {
                                                    // 在这里可以触发驱动更新操作
                                                    println!("准备更新驱动: {}", driver.name);
                                                }
                                            });
                                        });
                                    }
                                });
                        });
                    }
                },
                CurrentView::DriverScan => {
                    if !self.driver_scan_results.is_empty() {
                        ui.label("驱动扫描结果:");
                        
                        // 显示驱动扫描结果表格
                        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .striped(true)
                                .column(egui_extras::Column::exact(150.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(100.0))
                                .column(egui_extras::Column::exact(200.0))
                                .column(egui_extras::Column::exact(80.0))
                                .column(egui_extras::Column::remainder())
                                .header(20.0, |mut header| {
                                    header.col(|ui| { ui.strong("驱动名称"); });
                                    header.col(|ui| { ui.strong("当前版本"); });
                                    header.col(|ui| { ui.strong("最新版本"); });
                                    header.col(|ui| { ui.strong("硬件ID"); });
                                    header.col(|ui| { ui.strong("状态"); });
                                    header.col(|ui| { ui.strong("操作"); });
                                })
                                .body(|mut body| {
                                    for driver in &self.driver_scan_results {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| { ui.label(&driver.name); });
                                            row.col(|ui| { ui.label(&driver.current_version); });
                                            row.col(|ui| { ui.label(&driver.latest_version); });
                                            row.col(|ui| { 
                                                // 只显示硬件ID的简短部分
                                                let short_hwid = if driver.hardware_id.len() > 30 {
                                                    format!("{}...", &driver.hardware_id[..30])
                                                } else {
                                                    driver.hardware_id.clone()
                                                };
                                                ui.label(short_hwid); 
                                            });
                                            row.col(|ui| { 
                                                let status_text = match driver.status {
                                                    DriverStatus::Outdated => "需更新",
                                                    DriverStatus::UpToDate => "最新",
                                                    DriverStatus::NotInstalled => "未安装",
                                                };
                                                ui.label(status_text);
                                            });
                                            row.col(|ui| { 
                                                if driver.status == DriverStatus::Outdated && !driver.download_url.is_empty() && ui.button("更新").clicked() {
                                                    // 在这里可以触发驱动更新操作
                                                    println!("准备更新驱动: {}", driver.name);
                                                }
                                            });
                                        });
                                    }
                                });
                        });
                        ui.add_space(5.0);
                    } else {
                        ui.label("正在扫描驱动程序...");
                    }
                },
            }
        });
    }
}



