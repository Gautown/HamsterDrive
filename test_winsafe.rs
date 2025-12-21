/// 测试WinSafe功能的独立程序
use winsafe::prelude::*;
use winsafe::{HWND, co};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试WinSafe功能...");
    
    // 获取桌面窗口句柄
    let hwnd_desktop = HWND::GetDesktopWindow();
    println!("桌面窗口句柄: {:?}", hwnd_desktop);
    
    // 获取窗口标题
    let title = hwnd_desktop.GetWindowText()?;
    println!("窗口标题: {}", title);
    
    println!("WinSafe测试完成！");
    Ok(())
}
