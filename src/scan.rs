use crate::error::HamsterError;

/// 扫描系统硬件组件
pub fn scan_hardware() -> Result<Vec<String>, HamsterError> {
    let mut hardware_list = Vec::new();
    
    // 获取计算机名称
    match winsafe::GetComputerName() {
        Ok(name) => {
            hardware_list.push(format!("计算机名称: {}", name));
        },
        Err(_) => {
            hardware_list.push("计算机名称: 未知".to_string());
        }
    }
    
    // 添加默认硬件信息
    add_default_hardware_info(&mut hardware_list);
    
    Ok(hardware_list)
}









/// 添加默认硬件信息
fn add_default_hardware_info(hardware_list: &mut Vec<String>) {
    hardware_list.push("CPU: 未知".to_string());
    hardware_list.push("内存: 未知".to_string());
    hardware_list.push("主板: 未知".to_string());
    hardware_list.push("显卡: 未知".to_string());
    hardware_list.push("声卡: 未知".to_string());
    hardware_list.push("网卡: 未知".to_string());
    hardware_list.push("USB控制器: 未知".to_string());
}



/// 扫描已安装的驱动程序
pub fn scan_installed_drivers() -> Result<Vec<String>, HamsterError> {
    // 枚举已安装驱动
    let mut driver_list = Vec::new();
    
    // 示例驱动列表
    driver_list.push("usbhub.sys".to_string());
    driver_list.push("tcpip.sys".to_string());
    driver_list.push("dxgkrnl.sys".to_string());
    
    // 实际实现中，这里会查询注册表或使用SetupAPI来获取真实的驱动列表
    // 例如查询 HKLM\SYSTEM\CurrentControlSet\Services
    
    Ok(driver_list)
}
