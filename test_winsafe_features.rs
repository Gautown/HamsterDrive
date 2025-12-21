/// 测试WinSafe功能的独立程序
use winsafe::prelude::*;
use winsafe::{HWND, HINSTANCE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试WinSafe功能...");
    
    // 获取当前实例句柄
    let hinst = HINSTANCE::GetModuleHandle(None)?;
    println!("当前实例句柄: {:?}", hinst);
    
    // 获取桌面窗口句柄
    let hwnd_desktop = HWND::GetDesktopWindow();
    println!("桌面窗口句柄: {:?}", hwnd_desktop);
    
    // 获取窗口标题
    let title = hwnd_desktop.GetWindowText()?;
    println!("桌面窗口标题: {}", title);
    
    println!("WinSafe功能测试完成！");
    Ok(())
}
