use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::core::DriverUpdaterCore;
use crate::os_info::SystemInfo;


pub struct HamsterDriveApp {
    core: Arc<Mutex<Option<DriverUpdaterCore>>>,
    current_view: View,
    system_info: Option<SystemInfo>,
    scan_results: String,
    update_candidates: String,
    download_progress: String,
    is_scanning: bool,
    scan_progress: f32,
    progress_text: String,
}

#[derive(Debug, Clone, PartialEq)]
enum View {
    SystemInfo,
    HardwareScan,
    DriverUpdates,
    Settings,
    About,
}

impl HamsterDriveApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 设置中文字体支持
        let mut fonts = egui::FontDefinitions::default();
        
        // 添加NotoSansSC字体支持
        fonts.font_data.insert(
            "noto_sans_sc".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/font/NotoSansSC-Thin.otf"))
        );
        
        // 配置字体族，将NotoSansSC作为中文字体选项
        fonts.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap_or(&mut Vec::new())
            .insert(0, "noto_sans_sc".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap_or(&mut Vec::new())
            .insert(0, "noto_sans_sc".to_owned());
        
        cc.egui_ctx.set_fonts(fonts);
        
        // 安装图像加载器以支持图片显示
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            core: Arc::new(Mutex::new(None)),
            current_view: View::SystemInfo,
            system_info: None,
            scan_results: String::new(),
            update_candidates: String::new(),
            download_progress: String::new(),
            is_scanning: false,
            scan_progress: 0.0,
            progress_text: String::new(),
        }
    }

    fn initialize_core(&mut self) {
        if self.core.lock().unwrap().is_none() {
            // 使用内存路径而不是实际的数据库文件，避免文件锁定问题
            let db_path = ":memory:";
            let core_result = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    DriverUpdaterCore::new(db_path, "./downloads").await
                })
            });
            
            match core_result {
                Ok(mut core) => {
                    // 初始化core
                    let init_result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            core.initialize().await
                        })
                    });
                    
                    if let Err(e) = init_result {
                        eprintln!("初始化核心失败: {}", e);
                    }
                    
                    // 更新系统信息到UI
                    self.system_info = core.system_info.clone();
                    
                    // 最后设置core
                    *self.core.lock().unwrap() = Some(core);
                }
                Err(e) => eprintln!("初始化核心失败: {}", e),
            }
        }
    }
    

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        // 使用egui::Image::from_bytes API直接显示图片
        ui.add(egui::Image::from_bytes("HamsterDriveLogo", include_bytes!("../../assets/images/HamsterDrive64.png")).max_width(128.0));
        
        // 导航菜单
        self.sidebar_button(ui, "系统信息", View::SystemInfo);
        self.sidebar_button(ui, "硬件扫描", View::HardwareScan);
        self.sidebar_button(ui, "驱动更新", View::DriverUpdates);
        
        ui.separator();
        
        // 设置按钮放在分隔符下方
        self.sidebar_button(ui, "设置", View::Settings);
        
        // 关于按钮放在设置按钮下方
        self.sidebar_button(ui, "关于", View::About);
    }

    fn sidebar_button(&mut self, ui: &mut egui::Ui, label: &str, view: View) {
        let selected = self.current_view == view;
        let response = ui.selectable_label(selected, label);
        if response.clicked() {
            self.current_view = view;
        }
    }

    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
            
        // 初始化核心
        self.initialize_core();
            
        // 在进入UI闭包之前获取当前状态
        let current_is_scanning = self.is_scanning;
        let current_progress_text = self.progress_text.clone();
        let current_scan_progress = self.scan_progress;
                
        // 定义需要执行的操作
        let mut scan_clicked = false;
        let mut find_updates_clicked = false;
        let mut update_all_clicked = false;
                
        // 显示系统摘要、仪表盘和硬件摘要
        if let Ok(core_guard) = self.core.lock() {
            if let Some(ref core) = *core_guard {
                if let Some(summary) = core.get_system_summary() {
                            
                    ui.heading("系统摘要");
                    ui.label(&summary);
                    ui.separator();
                }
                        
                ui.heading("仪表盘");
                        
                egui::Grid::new("dashboard_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("功能:");
                        ui.label("状态:");
                        ui.end_row();
                                
                        if ui.button("扫描硬件").clicked() {
                            scan_clicked = true;
                        }
                        ui.label(if current_is_scanning { "正在扫描..." } else { "就绪" });
                        ui.end_row();
                                
                        if ui.button("查找驱动更新").clicked() {
                            find_updates_clicked = true;
                        }
                        ui.label("就绪");
                        ui.end_row();
                                
                        if ui.button("更新所有驱动").clicked() {
                            update_all_clicked = true;
                        }
                        ui.label("就绪");
                        ui.end_row();
                    });
                        
                // 显示进度
                if current_is_scanning {
                    ui.label(&current_progress_text);
                    ui.add(egui::ProgressBar::new(current_scan_progress).show_percentage());
                }
                        
                if let Some(hardware_summary) = core.get_hardware_summary() {
                    ui.separator();
                    ui.heading("硬件摘要");
                    ui.label(&hardware_summary);
                }
            }
        }
                
        // 在UI更新后执行需要可变借用的方法
        if scan_clicked {
            self.start_hardware_scan();
        }
        if find_updates_clicked {
            self.find_driver_updates();
        }
        if update_all_clicked {
            self.update_all_drivers();
        }
    }



    fn render_hardware_scan(&mut self, ui: &mut egui::Ui) {
        ui.heading("硬件扫描");
        
        if ui.button("开始扫描").clicked() {
            self.start_hardware_scan();
        }
        
        if self.is_scanning {
            ui.label(&self.progress_text);
            ui.add(egui::ProgressBar::new(self.scan_progress).show_percentage());
        } else {
            ui.horizontal(|ui| {
                if ui.button("刷新结果").clicked() {
                    self.refresh_scan_results();
                }
                
                if ui.button("导出结果").clicked() {
                    // 导出功能待实现
                }
            });
        }
        
        ui.separator();
        
        ui.label(&self.scan_results);
    }

    fn render_driver_updates(&mut self, ui: &mut egui::Ui) {
        ui.heading("驱动更新");
        
        ui.horizontal(|ui| {
            if ui.button("查找更新").clicked() {
                self.find_driver_updates();
            }
            
            if ui.button("更新所有").clicked() {
                self.update_all_drivers();
            }
        });
        
        ui.separator();
        
        // 显示更新候选
        ui.label(&self.update_candidates);
        
        // 显示下载进度
        if !self.download_progress.is_empty() {
            ui.separator();
            ui.heading("下载进度");
            ui.label(&self.download_progress);
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");
        
        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                let download_dir = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        if let Ok(core_guard) = self.core.lock() {
                            if let Some(ref core) = *core_guard {
                                return core.driver_fetcher.lock().await.download_dir.clone();
                            }
                        }
                        "./downloads".to_string()
                    })
                });
                ui.label("下载目录:");
                ui.label(&download_dir);
                ui.end_row();
                
                ui.label("数据库路径:");
                ui.label("drivers.db");
                ui.end_row();
                
                ui.label("Aria2端口:");
                ui.label("6800");
                ui.end_row();
            });
        
        ui.separator();
        
        if ui.button("保存设置").clicked() {
            // 保存设置功能待实现
        }
        
    }

    fn render_about(&mut self, ui: &mut egui::Ui) {
        ui.heading("关于");
        ui.label(format!("HamsterDrivers - Windows驱动管理工具"));
        ui.label(format!("版本: {}", env!("CARGO_PKG_VERSION")));
        ui.label("作者: Gautown");
        ui.label("许可证: MIT");
        ui.label("这是一个功能强大的Windows驱动管理工具，旨在帮助用户自动扫描、识别、比较厂商服务器上的驱动版本，并下载安装最新驱动。");
    }

    fn start_hardware_scan(&mut self) {
        self.is_scanning = true;
        self.scan_progress = 0.0;
        self.progress_text = "开始扫描...".to_string();
        
        // 直接执行扫描
        let needs_refresh = {
            if let Ok(mut core_guard) = self.core.lock() {
                if let Some(ref mut core) = *core_guard {
                    // 使用简单的回调函数
                    let progress_callback = |_progress: crate::core::ScanProgress| {
                        // 简单的回调，不修改UI状态
                    };
                    
                    // 在当前线程中执行扫描
                    match tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            core.scan_system(progress_callback).await
                        })
                    }) {
                        Ok(result) => {
                            println!("硬件扫描完成，发现 {} 个设备", result.devices.len());
                            // 直接更新core中的扫描结果
                            core.scan_result = Some(result);
                            true // 需要刷新
                        },
                        Err(e) => {
                            eprintln!("硬件扫描失败: {}", e);
                            false
                        }
                    }
                } else {
                    false
                }
            } else {
                false
            }
        };
        
        // 在锁外刷新扫描结果
        if needs_refresh {
            self.refresh_scan_results();
        }
        
        self.scan_progress = 100.0;
        self.progress_text = "扫描完成".to_string();
        self.is_scanning = false;
    }

    fn find_driver_updates(&mut self) {
        // 查找驱动更新
        if let Ok(mut core_guard) = self.core.lock() {
            if let Some(ref mut core) = *core_guard {
                match tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        core.find_driver_updates().await
                    })
                }) {
                    Ok(candidates) => {
                        println!("找到 {} 个驱动更新候选", candidates.len());
                        
                        // 更新UI显示
                        let mut result = String::new();
                        result.push_str(&format!("找到 {} 个驱动更新:\n\n", candidates.len()));
                        
                        for (i, candidate) in candidates.iter().enumerate() {
                            let hw_info = &candidate.hardware_info;
                            if let Some(ref driver) = candidate.matched_driver {
                                result.push_str(&format!(
                                    "{}. {} ({})\n   当前版本: {}\n   新版本: {}\n   下载链接: {}\n\n",
                                    i + 1,
                                    hw_info.device_name,
                                    hw_info.manufacturer,
                                    candidate.current_version,
                                    driver.driver_version,
                                    driver.driver_url
                                ));
                            } else {
                                result.push_str(&format!(
                                    "{}. {} ({}) - 未找到更新\n\n",
                                    i + 1,
                                    hw_info.device_name,
                                    hw_info.manufacturer
                                ));
                            }
                        }
                        
                        self.update_candidates = result;
                    },
                    Err(e) => {
                        eprintln!("查找驱动更新失败: {}", e);
                        self.update_candidates = format!("查找驱动更新失败: {}", e);
                    }
                }
            }
        }
    }

    fn update_all_drivers(&mut self) {
        // 由于方法需要可变引用，我们暂时跳过
        // 稍后通过其他方式实现
        println!("更新所有驱动...");
        self.download_progress = "驱动更新功能待实现".to_string();
    }

    fn refresh_scan_results(&mut self) {
        if let Ok(core_guard) = self.core.lock() {
            if let Some(ref core) = *core_guard {
                if let Some(ref scan_result) = core.scan_result {
                    let mut result = String::new();
                    result.push_str(&format!("扫描到 {} 个设备:\n", scan_result.devices.len()));
                    for (i, device) in scan_result.devices.iter().enumerate() {
                        result.push_str(&format!(
                            "{}. {} - {}\n   硬件ID: {}\n   驱动版本: {}\n",
                            i + 1,
                            device.device_name,
                            device.manufacturer,
                            device.hardware_id,
                            device.driver_version
                        ));
                    }
                    self.scan_results = result;
                } else {
                    self.scan_results = "未进行扫描".to_string();
                }
            }
        }
    }
}

impl eframe::App for HamsterDriveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建左右分栏布局
        egui::SidePanel::left("sidebar")
            .min_width(150.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });
        

        
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::SystemInfo => self.render_dashboard(ui),
                View::HardwareScan => self.render_hardware_scan(ui),
                View::DriverUpdates => self.render_driver_updates(ui),
                View::Settings => self.render_settings(ui),
                View::About => self.render_about(ui),
            }
        });
        
        // 定期更新UI
        ctx.request_repaint();
    }
}