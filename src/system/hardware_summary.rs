//! 硬件摘要信息

use crate::types::system_types::*;
use crate::utils::error::{HamsterError, Result};
use std::process::Command;

/// 获取完整的系统摘要
pub fn get_system_summary() -> Result<SystemSummary> {
    let mut summary = SystemSummary::new();
    
    // 获取操作系统信息
    if let Ok(os_info) = crate::system::os_info::get_os_info() {
        summary.os = os_info;
    }
    
    // 获取CPU信息
    if let Ok(cpu_info) = get_cpu_info() {
        summary.cpu = Some(cpu_info);
    }
    
    // 获取内存信息
    if let Ok(memory_info) = get_memory_info() {
        summary.memory = Some(memory_info);
    }
    
    // 获取主板信息
    if let Ok(motherboard_info) = get_motherboard_info() {
        summary.motherboard = Some(motherboard_info);
    }
    
    // 获取显卡信息
    if let Ok(gpus) = get_gpu_info() {
        summary.gpus = gpus;
    }
    
    // 获取磁盘信息
    if let Ok(disks) = get_disk_info() {
        summary.disks = disks;
    }
    
    Ok(summary)
}

/// 获取CPU信息
#[cfg(windows)]
pub fn get_cpu_info() -> Result<CpuInfo> {
    let output = Command::new("wmic")
        .args(&["cpu", "get", "Name,Manufacturer,NumberOfCores,NumberOfLogicalProcessors,MaxClockSpeed", "/format:list"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取CPU信息失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut cpu_info = CpuInfo {
        name: "Unknown CPU".to_string(),
        vendor: "Unknown".to_string(),
        cores: 0,
        threads: 0,
        base_clock: 0,
        architecture: crate::system::os_info::get_architecture(),
    };
    
    for line in stdout.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "Name" => cpu_info.name = value.trim().to_string(),
                "Manufacturer" => cpu_info.vendor = value.trim().to_string(),
                "NumberOfCores" => cpu_info.cores = value.trim().parse().unwrap_or(0),
                "NumberOfLogicalProcessors" => cpu_info.threads = value.trim().parse().unwrap_or(0),
                "MaxClockSpeed" => cpu_info.base_clock = value.trim().parse().unwrap_or(0),
                _ => {}
            }
        }
    }
    
    Ok(cpu_info)
}

#[cfg(not(windows))]
pub fn get_cpu_info() -> Result<CpuInfo> {
    Ok(CpuInfo {
        name: "Unknown CPU".to_string(),
        vendor: "Unknown".to_string(),
        cores: 0,
        threads: 0,
        base_clock: 0,
        architecture: Architecture::Unknown,
    })
}

/// 获取内存信息
#[cfg(windows)]
pub fn get_memory_info() -> Result<MemoryInfo> {
    let total = crate::utils::system_utils::get_total_memory()?;
    let available = crate::utils::system_utils::get_available_memory()?;
    
    Ok(MemoryInfo {
        total_physical: total,
        available_physical: available,
        total_virtual: 0,
        available_virtual: 0,
        slots: Vec::new(),
    })
}

#[cfg(not(windows))]
pub fn get_memory_info() -> Result<MemoryInfo> {
    Ok(MemoryInfo {
        total_physical: 0,
        available_physical: 0,
        total_virtual: 0,
        available_virtual: 0,
        slots: Vec::new(),
    })
}

/// 获取主板信息
#[cfg(windows)]
pub fn get_motherboard_info() -> Result<MotherboardInfo> {
    let output = Command::new("wmic")
        .args(&["baseboard", "get", "Manufacturer,Product,Version,SerialNumber", "/format:list"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取主板信息失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut info = MotherboardInfo {
        manufacturer: "Unknown".to_string(),
        product: "Unknown".to_string(),
        version: "Unknown".to_string(),
        serial_number: "Unknown".to_string(),
        bios_version: "Unknown".to_string(),
        bios_date: "Unknown".to_string(),
    };
    
    for line in stdout.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "Manufacturer" => info.manufacturer = value.trim().to_string(),
                "Product" => info.product = value.trim().to_string(),
                "Version" => info.version = value.trim().to_string(),
                "SerialNumber" => info.serial_number = value.trim().to_string(),
                _ => {}
            }
        }
    }
    
    // 获取BIOS信息
    if let Ok(bios_output) = Command::new("wmic")
        .args(&["bios", "get", "SMBIOSBIOSVersion,ReleaseDate", "/format:list"])
        .output()
    {
        let bios_stdout = String::from_utf8_lossy(&bios_output.stdout);
        for line in bios_stdout.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "SMBIOSBIOSVersion" => info.bios_version = value.trim().to_string(),
                    "ReleaseDate" => {
                        let date_str = value.trim();
                        if date_str.len() >= 8 {
                            info.bios_date = format!(
                                "{}-{}-{}",
                                &date_str[0..4],
                                &date_str[4..6],
                                &date_str[6..8]
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    Ok(info)
}

#[cfg(not(windows))]
pub fn get_motherboard_info() -> Result<MotherboardInfo> {
    Ok(MotherboardInfo {
        manufacturer: "Unknown".to_string(),
        product: "Unknown".to_string(),
        version: "Unknown".to_string(),
        serial_number: "Unknown".to_string(),
        bios_version: "Unknown".to_string(),
        bios_date: "Unknown".to_string(),
    })
}

/// 获取显卡信息
#[cfg(windows)]
pub fn get_gpu_info() -> Result<Vec<GpuInfo>> {
    let output = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "Name,AdapterRAM,DriverVersion,DriverDate,PNPDeviceID", "/format:list"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取显卡信息失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpus = Vec::new();
    let mut current_gpu: Option<GpuInfo> = None;
    
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(gpu) = current_gpu.take() {
                gpus.push(gpu);
            }
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let gpu = current_gpu.get_or_insert_with(|| GpuInfo {
                name: "Unknown GPU".to_string(),
                vendor: "Unknown".to_string(),
                vram_size: 0,
                driver_version: "Unknown".to_string(),
                driver_date: "Unknown".to_string(),
                hardware_id: String::new(),
            });
            
            match key.trim() {
                "Name" => {
                    gpu.name = value.trim().to_string();
                    // 根据名称推断厂商
                    if gpu.name.to_lowercase().contains("nvidia") {
                        gpu.vendor = "NVIDIA".to_string();
                    } else if gpu.name.to_lowercase().contains("amd") || gpu.name.to_lowercase().contains("radeon") {
                        gpu.vendor = "AMD".to_string();
                    } else if gpu.name.to_lowercase().contains("intel") {
                        gpu.vendor = "Intel".to_string();
                    }
                }
                "AdapterRAM" => gpu.vram_size = value.trim().parse().unwrap_or(0),
                "DriverVersion" => gpu.driver_version = value.trim().to_string(),
                "DriverDate" => {
                    let date_str = value.trim();
                    if date_str.len() >= 8 {
                        gpu.driver_date = format!(
                            "{}-{}-{}",
                            &date_str[0..4],
                            &date_str[4..6],
                            &date_str[6..8]
                        );
                    }
                }
                "PNPDeviceID" => gpu.hardware_id = value.trim().to_string(),
                _ => {}
            }
        }
    }
    
    if let Some(gpu) = current_gpu {
        gpus.push(gpu);
    }
    
    Ok(gpus)
}

#[cfg(not(windows))]
pub fn get_gpu_info() -> Result<Vec<GpuInfo>> {
    Ok(Vec::new())
}

/// 获取磁盘信息
#[cfg(windows)]
pub fn get_disk_info() -> Result<Vec<DiskInfo>> {
    let output = Command::new("wmic")
        .args(&["diskdrive", "get", "Model,SerialNumber,Size,InterfaceType,MediaType", "/format:list"])
        .output()
        .map_err(|e| HamsterError::ScanError(format!("获取磁盘信息失败: {}", e)))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut disks = Vec::new();
    let mut current_disk: Option<DiskInfo> = None;
    
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(disk) = current_disk.take() {
                disks.push(disk);
            }
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let disk = current_disk.get_or_insert_with(|| DiskInfo {
                model: "Unknown Disk".to_string(),
                serial_number: "Unknown".to_string(),
                total_size: 0,
                interface_type: "Unknown".to_string(),
                media_type: MediaType::Unknown,
                partitions: Vec::new(),
            });
            
            match key.trim() {
                "Model" => disk.model = value.trim().to_string(),
                "SerialNumber" => disk.serial_number = value.trim().to_string(),
                "Size" => disk.total_size = value.trim().parse().unwrap_or(0),
                "InterfaceType" => disk.interface_type = value.trim().to_string(),
                "MediaType" => {
                    let media = value.trim().to_lowercase();
                    disk.media_type = if media.contains("ssd") || media.contains("solid") {
                        MediaType::SSD
                    } else if media.contains("nvme") {
                        MediaType::NVMe
                    } else if media.contains("fixed") || media.contains("hdd") {
                        MediaType::HDD
                    } else {
                        MediaType::Unknown
                    };
                }
                _ => {}
            }
        }
    }
    
    if let Some(disk) = current_disk {
        disks.push(disk);
    }
    
    Ok(disks)
}

#[cfg(not(windows))]
pub fn get_disk_info() -> Result<Vec<DiskInfo>> {
    Ok(Vec::new())
}
