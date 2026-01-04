// 简单的测试程序，用于验证爬虫功能
use hamster_drive::matcher::scraper::{HardwareScraper, HardwareDriverInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试硬件驱动爬虫功能...");
    
    let scraper = HardwareScraper::new();
    
    // 测试识别厂商
    let vendor = scraper.identify_vendor_from_hardware_id("PCI\\VEN_10DE&DEV_1C82");
    println!("识别的厂商: {}", vendor);
    
    // 测试爬取驱动信息
    if let Ok(Some(driver_info)) = scraper.search_generic_driver("PCI\\VEN_10DE&DEV_1C82").await {
        println!("获取到驱动信息:");
        println!("  硬件ID: {}", driver_info.hardware_id);
        println!("  设备名称: {}", driver_info.device_name);
        println!("  制造商: {}", driver_info.manufacturer);
        println!("  驱动名称: {}", driver_info.driver_name);
        println!("  驱动版本: {}", driver_info.driver_version);
        println!("  下载链接: {}", driver_info.driver_url);
    } else {
        println!("未能获取驱动信息");
    }
    
    println!("测试完成");
    Ok(())
}