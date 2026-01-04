use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub windows_version: String,
    pub windows_edition: String,
    pub windows_activation_status: String,
    pub directx_version: String,
    pub manufacturer: String,
    pub model: String,
    pub motherboard: String,
    pub cpu: String,
    pub memory_info: String,
    pub gpu: String,
}

impl SystemInfo {
    pub fn new() -> Result<Self> {
        Ok(SystemInfo {
            windows_version: get_windows_version()?,
            windows_edition: get_windows_edition()?,
            windows_activation_status: get_windows_activation_status()?,
            directx_version: get_directx_version()?,
            manufacturer: get_manufacturer()?,
            model: get_model()?,
            motherboard: get_motherboard()?,
            cpu: get_cpu_info()?,
            memory_info: get_memory_info()?,
            gpu: get_gpu_info()?,
        })
    }
}

fn get_windows_version() -> Result<String> {
    let output = Command::new("cmd")
        .args(&["/C", "wmic os get Version /value"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    for line in output_str.lines() {
        if line.contains("Version=") {
            return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
        }
    }
    Ok("未知".to_string())
}

fn get_windows_edition() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-ComputerInfo).WindowsProductName"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let edition = output_str.trim();
    if edition.is_empty() {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic os get Caption /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output_str.lines() {
            if line.contains("Caption=") {
                return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
            }
        }
    }
    Ok(edition.to_string())
}

fn get_windows_activation_status() -> Result<String> {
    // 使用PowerShell命令替代slmgr，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-CimInstance SoftwareLicensingProduct | Where-Object { $_.Name -like '*Windows*' -and $_.PartialProductKey }).LicenseStatus"])
        .output();
    
    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            let status_line = output_str.trim();
            
            // 如果PowerShell命令成功执行，检查返回的状态
            if status_line.contains("0") {
                Ok("未激活".to_string())
            } else if status_line.contains("1") {
                Ok("已激活".to_string())
            } else {
                // 如果PowerShell命令失败，尝试使用slmgr命令
                let fallback_output = Command::new("cmd")
                    .args(&["/C", "slmgr", "/xpr"])
                    .output();
                
                match fallback_output {
                    Ok(fallback_output) => {
                        let fallback_str = String::from_utf8_lossy(&fallback_output.stdout).to_string();
                        let fallback_trimmed = fallback_str.trim();
                        
                        // 检查原始输出中是否包含激活相关信息
                        if fallback_trimmed.contains("Licensed") || fallback_trimmed.contains("已许可") {
                            Ok("已激活".to_string())
                        } else {
                            Ok("未激活".to_string())
                        }
                    }
                    Err(_) => Ok("未知".to_string()),
                }
            }
        }
        Err(_) => {
            // 如果PowerShell命令失败，使用slmgr命令作为备选
            let fallback_output = Command::new("cmd")
                .args(&["/C", "slmgr", "/xpr"])
                .output();
            
            match fallback_output {
                Ok(fallback_output) => {
                    let fallback_str = String::from_utf8_lossy(&fallback_output.stdout).to_string();
                    let fallback_trimmed = fallback_str.trim();
                    
                    // 检查原始输出中是否包含激活相关信息
                    if fallback_trimmed.contains("Licensed") || fallback_trimmed.contains("已许可") {
                        Ok("已激活".to_string())
                    } else {
                        Ok("未激活".to_string())
                    }
                }
                Err(_) => Ok("未知".to_string()),
            }
        }
    }
}

fn get_manufacturer() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-WmiObject Win32_ComputerSystem).Manufacturer"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let manufacturer = output_str.trim();
    if manufacturer.is_empty() {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic computersystem get Manufacturer /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output_str.lines() {
            if line.contains("Manufacturer=") {
                return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
            }
        }
    }
    Ok(manufacturer.to_string())
}

fn get_model() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-WmiObject Win32_ComputerSystem).Model"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let model = output_str.trim();
    if model.is_empty() {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic computersystem get Model /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output_str.lines() {
            if line.contains("Model=") {
                return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
            }
        }
    }
    Ok(model.to_string())
}

fn get_motherboard() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-WmiObject Win32_BaseBoard | Select-Object -ExpandProperty Manufacturer) + ' ' + (Get-WmiObject Win32_BaseBoard | Select-Object -ExpandProperty Product)"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let motherboard = output_str.trim();
    if motherboard.is_empty() || motherboard == "  " {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic baseboard get Manufacturer, Product /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let mut manufacturer = "未知".to_string();
        let mut product = "未知".to_string();

        for line in output_str.lines() {
            if line.contains("Manufacturer=") {
                manufacturer = line.split('=').nth(1).unwrap_or("未知").trim().to_string();
            } else if line.contains("Product=") {
                product = line.split('=').nth(1).unwrap_or("未知").trim().to_string();
            }
        }

        if &manufacturer == "未知" && &product == "未知" {
            Ok("未知".to_string())
        } else {
            Ok(format!("{} {}", manufacturer, product))
        }
    } else {
        Ok(motherboard.to_string())
    }
}

fn get_cpu_info() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-WmiObject Win32_Processor).Name"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let cpu = output_str.trim();
    if cpu.is_empty() {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic cpu get Name /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output_str.lines() {
            if line.contains("Name=") {
                return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
            }
        }
    }
    Ok(cpu.to_string())
}

fn get_total_memory_gb() -> Result<u64> {
    let output = Command::new("cmd")
        .args(&["/C", "wmic computersystem get TotalPhysicalMemory /value"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    for line in output_str.lines() {
        if line.contains("TotalPhysicalMemory=") {
            let memory_str = line.split('=').nth(1).unwrap_or("0").trim();
            let memory_bytes: u64 = memory_str.parse().unwrap_or(0);
            // 转换为GB
            return Ok(memory_bytes / (1024 * 1024 * 1024));
        }
    }
    Ok(0)
}

fn get_memory_info() -> Result<String> {
    // 使用PowerShell获取更准确的内存信息，优先使用CIM实例
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "(Get-CimInstance -ClassName Win32_PhysicalMemory | ForEach-Object { \
            $manufacturer = if ($_.Manufacturer -and $_.Manufacturer.Trim() -ne '') { $_.Manufacturer.Trim() } else { 'Unknown' }; \
            $partNumber = if ($_.PartNumber -and $_.PartNumber.Trim() -ne '') { $_.PartNumber.Trim() } else { 'Unknown' }; \
            $speed = if ($_.ConfiguredSpeed -or $_.Speed) { if($_.ConfiguredSpeed) { $_.ConfiguredSpeed } else { $_.Speed } } else { '0' }; \
            $capacity = if ($_.Capacity) { [math]::Round([long]$_.Capacity/1GB) } else { '0' }; \
            $memoryType = if ($_.MemoryType -and $_.MemoryType -ne 0) { $_.MemoryType } else { '0' }; \
            $formFactor = if ($_.FormFactor -and $_.FormFactor -ne 0) { $_.FormFactor } else { '0' }; \
            \"$manufacturer|$partNumber|$speed|$capacity|$memoryType|$formFactor\" \
            }) -join ';;'"
        ])
        .output();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let memory_info = output_str.trim();
        
        if !memory_info.is_empty() && memory_info != " " {
            // 解析PowerShell返回的内存信息，使用分隔符 ;; 分隔不同内存条
            let memory_parts: Vec<&str> = memory_info.split(";;").collect();
            if !memory_parts.is_empty() {
                // 获取第一条内存的信息
                let first_memory = memory_parts[0];
                // 使用 | 分隔各个字段
                let parts: Vec<&str> = first_memory.split('|').collect();
                
                if parts.len() >= 6 {
                    let manufacturer = parts[0];
                    let _part_number = parts[1];
                    let speed = parts[2];
                    let capacity = parts[3];
                    let _memory_type = parts[4];
                    let _form_factor = parts[5];
                    
                    // 直接使用内存速度推断DDR类型
                    let ddr_type = if speed.parse::<u32>().unwrap_or(0) >= 4000 {
                        "DDR5".to_string()
                    } else if speed.parse::<u32>().unwrap_or(0) >= 2133 && speed.parse::<u32>().unwrap_or(0) < 4000 {
                        "DDR4".to_string()
                    } else if speed.parse::<u32>().unwrap_or(0) >= 1333 && speed.parse::<u32>().unwrap_or(0) < 2133 {
                        "DDR3".to_string()
                    } else {
                        "DDR".to_string()  // 默认值
                    };
                    
                    // 构建内存信息字符串
                    let brand = if manufacturer == "Unknown" || manufacturer.is_empty() { "Unknown" } else { manufacturer };
                    let speed_mhz = if speed == "0" || speed.is_empty() || speed == "Unknown" { 
                        "Unknown" 
                    } else { 
                        &format!("{}MHz", speed) 
                    };
                    let capacity_gb = if capacity == "0" || capacity.is_empty() || capacity == "Unknown" { 
                        get_total_memory_gb()?
                    } else { 
                        capacity.parse::<u64>().unwrap_or_else(|_| get_total_memory_gb().unwrap_or(0))
                    };
                    
                    return Ok(format!("{} {} {} {}GB", brand, ddr_type, speed_mhz, capacity_gb));
                }
            }
        }
    }
    
    // 如果PowerShell也失败，使用WMIC作为回退
    get_memory_info_fallback_wmic()
}

// 解析CSV行的辅助函数
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut inside_quotes = false;
    
    for c in line.chars() {
        match c {
            '"' => inside_quotes = !inside_quotes,
            ',' if !inside_quotes => {
                fields.push(current_field.clone());
                current_field.clear();
            }
            _ => current_field.push(c),
        }
    }
    
    fields.push(current_field);
    fields.into_iter().map(|s| s.trim_matches('"').to_string()).collect()
}

// WMIC回退函数
fn get_memory_info_fallback_wmic() -> Result<String> {
    // 使用WMIC命令获取内存信息作为回退选项
    let wmic_output = Command::new("cmd")
        .args(&["/C", "wmic", "memorychip", "get", "Manufacturer,PartNumber,Speed,Capacity,FormFactor,MemoryType", "/format:csv"])
        .output();
    
    match wmic_output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            let lines: Vec<&str> = output_str.lines().collect();
            
            if lines.len() > 1 {
                // 第一行是标题，第二行开始是数据
                for line in lines.iter().skip(1) {
                    if !line.trim().is_empty() {
                        // 解析CSV格式的输出
                        let fields = parse_csv_line(line);
                        if fields.len() >= 6 {
                            let manufacturer = fields[0].as_str();
                            let _part_number = fields[1].as_str();
                            let speed = fields[2].as_str();
                            let capacity_bytes_str = fields[3].as_str();
                            let _form_factor_str = fields[4].as_str();
                            let _memory_type_str = fields[5].as_str();
                            
                            // 跳过空行或无效行
                            if manufacturer.is_empty() && _part_number.is_empty() {
                                continue;
                            }
                            
                            // 转换容量从字节到GB
                            let capacity_gb = if let Ok(bytes) = capacity_bytes_str.parse::<u64>() {
                                bytes / (1024 * 1024 * 1024)
                            } else {
                                get_total_memory_gb()?
                            };
                            
                            // 直接使用内存速度推断DDR类型
                            let ddr_type = if speed.parse::<u32>().unwrap_or(0) >= 4000 {
                                "DDR5".to_string()
                            } else if speed.parse::<u32>().unwrap_or(0) >= 2133 && speed.parse::<u32>().unwrap_or(0) < 4000 {
                                "DDR4".to_string()
                            } else if speed.parse::<u32>().unwrap_or(0) >= 1333 && speed.parse::<u32>().unwrap_or(0) < 2133 {
                                "DDR3".to_string()
                            } else {
                                "DDR".to_string()  // 默认值
                            };
                            
                            // 构建内存信息字符串
                            let brand = if manufacturer.is_empty() || manufacturer == "" { "Unknown" } else { manufacturer };
                            let speed_mhz = if speed.is_empty() || speed == "" || speed == "0" { "Unknown" } else { &format!("{}MHz", speed) };
                            
                            return Ok(format!("{} {} {} {}GB", brand, ddr_type, speed_mhz, capacity_gb));
                        }
                    }
                }
            }
        }
        Err(_) => {
            // 如果WMIC也失败，返回总内存大小
        }
    }
    
    // 如果WMIC也失败，回退到总内存大小
    Ok(format!("{}GB", get_total_memory_gb()?))
}







fn get_gpu_info() -> Result<String> {
    // 使用PowerShell命令替代WMIC，以避免字符编码问题
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-WmiObject Win32_VideoController).Name"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let gpu = output_str.trim();
    if gpu.is_empty() {
        // 如果PowerShell命令失败，回退到WMIC命令
        let output = Command::new("cmd")
            .args(&["/C", "wmic path win32_videocontroller get name /value"])
            .output()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output_str.lines() {
            if line.contains("Name=") {
                return Ok(line.split('=').nth(1).unwrap_or("未知").trim().to_string());
            }
        }
    }
    Ok(gpu.to_string())
}

fn get_directx_version() -> Result<String> {
    // 尝试通过PowerShell获取DirectX版本信息
    // 通过检查系统信息和DirectX诊断数据
    let output = Command::new("powershell")
        .args(&["-Command", "dxdiag /whql:off /t:dxdiag.txt; Get-Content dxdiag.txt | Select-String 'DirectX 版本'; Remove-Item dxdiag.txt -ErrorAction SilentlyContinue"])
        .output();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        if !output_str.is_empty() {
            // 尝试从输出中提取DirectX版本
            for line in output_str.lines() {
                if line.contains("DirectX") && (line.contains("Version") || line.contains("版本")) {
                    let cleaned = line.trim().replace("\"", "").replace("DirectX", "").replace("版本", "").replace(":", "").replace("=", "").trim().to_string();
                    if !cleaned.is_empty() {
                        return Ok(format!("DirectX {}", cleaned));
                    }
                }
            }
        }
    }
    
    // 如果PowerShell方法失败，尝试通过注册表获取
    let reg_output = Command::new("cmd")
        .args(&["/C", "reg query \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\DirectX\" /v \"Version\""])
        .output();
    
    if let Ok(reg_output) = reg_output {
        let reg_str = String::from_utf8_lossy(&reg_output.stdout).to_string();
        for line in reg_str.lines() {
            if line.contains("Version") && line.contains("REG_SZ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(version) = parts.last() {
                    if !version.is_empty() {
                        return Ok(format!("DirectX {}", version));
                    }
                }
            }
        }
    }
    
    // 如果以上方法都失败，返回默认值
    Ok("DirectX 未知".to_string())
}

// 使用WMI获取更详细的信息
pub fn get_system_info_wmi() -> Result<SystemInfo> {
    // 使用PowerShell命令替代WMI，因为WMI的使用较为复杂
    Ok(SystemInfo {
        windows_version: get_windows_version()?,
        windows_edition: get_windows_edition()?,
        windows_activation_status: get_windows_activation_status()?,
        directx_version: get_directx_version()?,
        manufacturer: get_manufacturer()?,
        model: get_model()?,
        motherboard: get_motherboard()?,
        cpu: get_cpu_info()?,
        memory_info: get_memory_info()?,
        gpu: get_gpu_info()?,
    })
}