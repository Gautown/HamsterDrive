use crate::error::HamsterError;
use crate::scan::DriverInfo;
use std::process::Command;

pub fn uninstall_driver(driver_name: &str) -> Result<(), HamsterError> {
    let output = Command::new("pnputil")
        .args(&["/delete-driver", driver_name, "/uninstall", "/force"])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("成功卸载驱动: {}", driver_name);
                Ok(())
            } else {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                Err(HamsterError::ScanError(format!("卸载驱动失败: {}", error_msg)))
            }
        },
        Err(e) => Err(HamsterError::ScanError(format!("执行卸载命令失败: {}", e)))
    }
}

pub fn uninstall_multiple_drivers(drivers: &[DriverInfo]) -> Result<Vec<String>, HamsterError> {
    let mut results = Vec::new();
    
    for driver in drivers {
        match uninstall_driver(&driver.name) {
            Ok(_) => {
                results.push(format!("成功卸载: {}", driver.name));
            },
            Err(e) => {
                results.push(format!("卸载失败 {}: {}", driver.name, e));
            }
        }
    }
    
    Ok(results)
}

pub fn get_installed_driver_packages() -> Result<Vec<String>, HamsterError> {
    let output = Command::new("pnputil")
        .args(&["/enum-drivers"])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let mut packages = Vec::new();
                
                for line in stdout.lines() {
                    if line.contains("Published Name:") {
                        let package_name = line.split(':').nth(1)
                            .map(|s| s.trim().to_string())
                            .unwrap_or_default();
                        if !package_name.is_empty() {
                            packages.push(package_name);
                        }
                    }
                }
                
                Ok(packages)
            } else {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                Err(HamsterError::ScanError(format!("获取驱动包列表失败: {}", error_msg)))
            }
        },
        Err(e) => Err(HamsterError::ScanError(format!("执行命令失败: {}", e)))
    }
}

pub fn find_driver_by_hardware_id(hardware_id: &str) -> Result<Option<String>, HamsterError> {
    let packages = get_installed_driver_packages()?;
    
    for package in packages {
        let output = Command::new("pnputil")
            .args(&["/driver-info", &package])
            .output();
        
        if let Ok(result) = output {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                if stdout.contains(hardware_id) {
                    return Ok(Some(package));
                }
            }
        }
    }
    
    Ok(None)
}
