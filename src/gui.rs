use eframe::egui;
use crate::{scan, backup, restore, update, list};
use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

// 定义当前显示的视图类型
#[derive(Debug, Clone, PartialEq, Default)]
enum CurrentView {
    #[default]
    SystemInfo,
    HardwareScan,
    DriverUpdate,
    DriverBackup,
    DriverRestore,
    DriverList,
    DriverScan,
}

pub fn run() -> Result<(), eframe::Error> {
    let app = HamsterDriveApp::default();
    let mut native_options = eframe::NativeOptions::default();
    
    // 配置字体以支持中文显示
    native_options.renderer = eframe::Renderer::Glow;
    
    // 禁用窗口装饰但启用拖放功能
    native_options.viewport = egui::ViewportBuilder::default()
        .with_drag_and_drop(true)  // 启用拖放功能
        .with_decorations(false)   // 禁用窗口装饰
        .with_inner_size((1024.0, 768.0))  // 设置初始窗口大小
        .with_min_inner_size((800.0, 600.0))  // 设置最小窗口大小
        ;
    
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
    
    // 尝试加载 Windows 系统字体
    #[cfg(windows)]
    {
        if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/msyh.ttc") {
            fonts.font_data.insert(
                "Microsoft YaHei".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            fonts.families.entry(Proportional).or_default().insert(0, "Microsoft YaHei".to_owned());
        } else if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/simhei.ttf") {
            fonts.font_data.insert(
                "SimHei".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            fonts.families.entry(Proportional).or_default().insert(0, "SimHei".to_owned());
        }
        // 如果都失败了，使用默认字体配置
    }
    
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

// 窗体拖动事件枚举
#[derive(Debug, Clone)]
enum WindowDragEvent {
    DragStart { x: i32, y: i32 },
    DragMove { x: i32, y: i32 },
    DragEnd,
    MoveWindow { delta_x: i32, delta_y: i32 },
}

// 窗体拖动状态
struct WindowDragState {
    is_dragging: bool,
    last_x: f32,
    last_y: f32,
    start_x: f32,
    start_y: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Default for WindowDragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            last_x: 0.0,
            last_y: 0.0,
            start_x: 0.0,
            start_y: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl Drop for HamsterDriveApp {
    fn drop(&mut self) {
        // 在应用程序关闭时清理拖动线程
        self.stop_window_drag_listener();
    }
}

#[derive(Default)]
struct HamsterDriveApp {
    hardware_info: Vec<String>,
    system_info: Vec<String>,
    driver_list: Vec<String>,
    update_list: Vec<String>,
    backup_status: String,
    restore_status: String,
    initialized: bool,
    current_view: CurrentView,
    // 用于异步操作的通道
    scan_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    system_info_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    backup_rx: Option<std::sync::mpsc::Receiver<Result<(), crate::error::HamsterError>>>,
    restore_rx: Option<std::sync::mpsc::Receiver<Result<(), crate::error::HamsterError>>>,
    update_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    list_rx: Option<std::sync::mpsc::Receiver<Result<Vec<String>, crate::error::HamsterError>>>,
    // 标记操作是否在进行中
    scanning: bool,
    getting_system_info: bool,
    backing_up: bool,
    restoring: bool,
    checking_updates: bool,
    loading_drivers: bool,
    // 窗体拖动功能相关
    window_drag_tx: Option<std::sync::mpsc::Sender<WindowDragEvent>>,
    drag_state: Arc<Mutex<WindowDragState>>,
    drag_thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl eframe::App for HamsterDriveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 检查是否有异步操作完成
        self.check_async_operations();
        
        // 设置窗体拖动处理
        self.setup_window_drag_handling(ctx);
        
        // 初始化时自动获取系统信息
        if !self.initialized {
            // 使用异步方式获取系统信息以避免界面冻结
            self.current_view = CurrentView::SystemInfo;
            self.start_get_system_info();
            // 启动窗口拖动监听线程
            self.start_window_drag_listener();
            self.initialized = true;
        }
        
        // 创建左侧边栏
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("仓鼠驱动管家");
            ui.separator(); // 添加分隔线
            
            // 左侧菜单按钮
            if ui.button("驱动扫描").clicked() && !self.scanning {
                self.current_view = CurrentView::DriverScan;
                
                let (tx, rx) = std::sync::mpsc::channel();
                self.scan_rx = Some(rx);
                self.scanning = true;
                
                std::thread::spawn(move || {
                    let result = scan::scan_outdated_drivers();
                    let _ = tx.send(result);
                });
            }
            
            if ui.button("硬件扫描").clicked() && !self.scanning {
                self.current_view = CurrentView::HardwareScan;
                self.start_scan_hardware();
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
            
            // 显示操作状态
            if self.scanning {
                ui.label("🔍 扫描中...");
            }
            if self.checking_updates {
                ui.label("🔄 检查更新中...");
            }
            if self.backing_up {
                ui.label("💾 备份中...");
            }
            if self.restoring {
                ui.label("📂 恢复中...");
            }
            if self.loading_drivers {
                ui.label("📋 加载中...");
            }
            
            ui.add_space(10.0);
            ui.separator();
            ui.label("状态信息");
        });
        
        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            // 在右上角添加窗口控制按钮
            egui::Frame::none().show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0); // 添加一些空间以避免边缘贴边
                        if ui.button("X").on_hover_text("关闭").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("□").on_hover_text("最大化").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!ctx.input(|i| i.viewport().maximized.unwrap_or(false))));
                        }
                        if ui.button("-").on_hover_text("最小化").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });
            
            // 添加一些空间以避免按钮遮挡内容
            ui.add_space(10.0);
            
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
                CurrentView::HardwareScan => {
                    if !self.hardware_info.is_empty() {
                        ui.label("硬件扫描结果:");
                        for item in &self.hardware_info {
                            ui.label(item);
                        }
                        ui.add_space(5.0);
                    }
                },
                CurrentView::DriverUpdate => {
                    if !self.update_list.is_empty() {
                        ui.label("可用更新:");
                        for update in &self.update_list {
                            ui.label(update);
                        }
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
                        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                            for driver in &self.driver_list {
                                ui.label(driver);
                            }
                        });
                    }
                },
                CurrentView::DriverScan => {
                    if !self.hardware_info.is_empty() {
                        ui.label("驱动扫描结果:");
                        for item in &self.hardware_info {
                            ui.label(item);
                        }
                        ui.add_space(5.0);
                    }
                },
            }
        });
    }
}

impl HamsterDriveApp {
    // 检查异步操作结果
    fn check_async_operations(&mut self) {
        // 检查系统信息获取结果
        if let Some(ref rx) = self.system_info_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(info) => {
                        self.system_info = info;
                        self.getting_system_info = false;
                    },
                    Err(e) => {
                        self.system_info.clear();
                        self.system_info.push(format!("错误: {}", e));
                        self.getting_system_info = false;
                    }
                }
                self.system_info_rx = None;
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
                    Err(e) => {
                        self.hardware_info.clear();
                        self.hardware_info.push(format!("错误: {}", e));
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
                    Err(e) => {
                        self.update_list.clear();
                        self.update_list.push(format!("错误: {}", e));
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
                    Err(e) => self.backup_status = format!("备份失败: {}", e),
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
                    Err(e) => self.restore_status = format!("恢复失败: {}", e),
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
                    Err(e) => {
                        self.driver_list.clear();
                        self.driver_list.push(format!("错误: {}", e));
                    }
                }
                self.list_rx = None;
                self.loading_drivers = false;
            }
        }
    }
    
    // 开始硬件扫描
    fn start_scan_hardware(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.scan_rx = Some(rx);
        self.scanning = true;
        
        thread::spawn(move || {
            let result = scan::scan_hardware();
            let _ = tx.send(result);
        });
    }
    
    // 开始检查更新
    fn start_check_updates(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.update_rx = Some(rx);
        self.checking_updates = true;
        
        thread::spawn(move || {
            let result = update::check_updates();
            let _ = tx.send(result);
        });
    }
    
    // 开始备份驱动
    fn start_backup_drivers(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.backup_rx = Some(rx);
        self.backing_up = true;
        
        thread::spawn(move || {
            let result = backup::backup_drivers(true);
            let _ = tx.send(result);
        });
    }
    
    // 开始恢复驱动
    fn start_restore_drivers(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.restore_rx = Some(rx);
        self.restoring = true;
        
        thread::spawn(move || {
            let result = restore::restore_drivers();
            let _ = tx.send(result);
        });
    }
    
    // 开始显示驱动列表
    fn start_show_installed_drivers(&mut self) {
        let (tx, rx) = mpsc::channel();
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

    // 启动窗体拖动监听线程
    fn start_window_drag_listener(&mut self) {
        if self.window_drag_tx.is_some() {
            return; // 已经在运行
        }

        let (tx, rx) = mpsc::channel();
        self.window_drag_tx = Some(tx);
        
        let drag_state = Arc::clone(&self.drag_state);
        
        // 在独立线程中处理窗体拖动事件
        let handle = thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        match event {
                            WindowDragEvent::DragStart { x, y } => {
                                let mut state = drag_state.lock().unwrap();
                                state.is_dragging = true;
                                state.last_x = x as f32;
                                state.last_y = y as f32;
                                state.start_x = x as f32;
                                state.start_y = y as f32;
                                
                                println!("开始拖动窗口: ({}, {})", x, y);
                            },
                            WindowDragEvent::DragMove { x, y } => {
                                let mut state = drag_state.lock().unwrap();
                                if state.is_dragging {
                                    let delta_x = x as f32 - state.last_x;
                                    let delta_y = y as f32 - state.last_y;
                                    state.last_x = x as f32;
                                    state.last_y = y as f32;
                                    
                                    println!("窗口拖动: Δx={:.1}, Δy={:.1}", delta_x, delta_y);
                                }
                            },
                            WindowDragEvent::DragEnd => {
                                let mut state = drag_state.lock().unwrap();
                                state.is_dragging = false;
                                
                                println!("结束拖动窗口");
                            },
                            WindowDragEvent::MoveWindow { delta_x, delta_y } => {
                                // 窗口移动逻辑已在主线程中处理
                                println!("窗口移动: Δx={}, Δy={}", delta_x, delta_y);
                            },
                        }
                    },
                    Err(_) => {
                        break; // 通道关闭，退出线程
                    }
                }
            }
        });
        
        self.drag_thread_handle = Some(handle);
    }

    // 设置窗体拖动处理
    fn setup_window_drag_handling(&mut self, ctx: &egui::Context) {
        // 在右上角添加窗口控制按钮（移到顶部）
        egui::TopBottomPanel::top("window_controls")
            .show_separator_line(false)
            .resizable(false)
            .min_height(30.0)
            .show(ctx, |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui: &mut egui::Ui| {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.add_space(5.0); // 添加一些空间以避免边缘贴边
                        if ui.button("X").on_hover_text("关闭").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("□").on_hover_text("最大化").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!ctx.input(|i| i.viewport().maximized.unwrap_or(false))));
                        }
                        if ui.button("-").on_hover_text("最小化").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });
        
        // 处理窗口拖动逻辑 - 现在整个窗口都可以拖动
        self.handle_window_drag(ctx);
    }

    // 处理窗口拖动
    fn handle_window_drag(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.pointer.clone());
        
        // 获取当前鼠标位置
        let current_pos = input.hover_pos();
        
        // 获取或初始化窗口拖动状态
        {
            let mut state = self.drag_state.lock().unwrap();
            
            // 检测鼠标按下事件（开始拖动）
            if input.any_pressed() {
                if let Some(pos) = current_pos {
                    // 检查是否在窗口控制按钮区域外（允许拖动整个窗口，但排除按钮区域）
                    let is_in_button_area = pos.x > 1024.0 - 100.0 && pos.y < 35.0;
                    
                    if !is_in_button_area { // 如果不在按钮区域，则可以拖动整个窗口
                        state.is_dragging = true;
                        state.last_x = pos.x;
                        state.last_y = pos.y;
                        state.start_x = pos.x;
                        state.start_y = pos.y;
                        
                        println!("开始拖动窗口: 鼠标=({:.0}, {:.0})", pos.x, pos.y);
                    }
                }
            }
            
            // 检测鼠标移动事件（拖动中）
            if let Some(pos) = current_pos {
                if state.is_dragging && input.any_down() {
                    // 计算鼠标移动的偏移量
                    let delta_x = pos.x - state.last_x;
                    let delta_y = pos.y - state.last_y;
                    
                    if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
                        state.last_x = pos.x;
                        state.last_y = pos.y;
                        
                        // 更新偏移量
                        state.offset_x += delta_x;
                        state.offset_y += delta_y;
                        
                        // 窗口拖动检测逻辑
                        println!("窗口拖动检测: Δx={:.1}, Δy={:.1}", delta_x, delta_y);
                        println!("鼠标在整个窗口区域拖动");
                        
                        // 实际移动窗口
                        self.move_window(delta_x as i32, delta_y as i32);
                    }
                }
            }
            
            // 检测鼠标释放事件（结束拖动）
            if input.any_released() {
                if state.is_dragging {
                    state.is_dragging = false;
                    println!("结束拖动窗口");
                }
            }
        }
    }
    
    // 移动窗口
    fn move_window(&self, delta_x: i32, delta_y: i32) {
        // 直接使用Windows API移动窗口
        #[cfg(windows)]
        {
            use winapi::um::winuser::{GetForegroundWindow, SetWindowPos, GetWindowRect, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOSIZE, SWP_NOZORDER};
            use winapi::shared::windef::RECT;
            use winapi::ctypes::c_int;
            
            unsafe {
                let hwnd = GetForegroundWindow();
                if !hwnd.is_null() {
                    let mut rect: RECT = std::mem::zeroed();
                    if GetWindowRect(hwnd, &mut rect as *mut RECT) == 1 {
                        let new_x = rect.left + delta_x;
                        let new_y = rect.top + delta_y;
                        
                        // 移动窗口
                        SetWindowPos(
                            hwnd,
                            HWND_NOTOPMOST,
                            new_x,
                            new_y,
                            0, // 宽度不变
                            0, // 高度不变
                            SWP_NOSIZE | SWP_NOZORDER
                        );
                        
                        println!("窗口已移动: Δx={}, Δy={}", delta_x, delta_y);
                    }
                }
            }
        }
    }

    // 停止窗体拖动监听线程
    fn stop_window_drag_listener(&mut self) {
        if let Some(tx) = self.window_drag_tx.take() {
            drop(tx); // 关闭通道，这会导致监听线程退出
        }
        
        if let Some(handle) = self.drag_thread_handle.take() {
            if let Err(e) = handle.join() {
                eprintln!("窗体拖动线程退出失败: {:?}", e);
            }
        }
    }
}

