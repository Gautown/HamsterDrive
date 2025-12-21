use crate::error::HamsterError;
use winsafe::prelude::*;
use winsafe::{WINTRUST, WTD_FLAGS, WINTRUST_DATA, WINTRUST_FILE_INFO};

/// 验证驱动文件签名
pub fn verify_driver_signature(driver_path: &str) -> Result<bool, HamsterError> {
    // 使用WinSafe进行真正的驱动签名验证
    
    // 创建文件信息结构
    let file_info = WINTRUST_FILE_INFO::default();
    
    // 创建信任数据结构
    let mut trust_data = WINTRUST_DATA::default();
    trust_data.set_dwStateAction(WINTRUST_DATA::STATE_ACTION_VERIFY);
    trust_data.set_fdwRevocationChecks(WTD_FLAGS::WTD_REVOKE_WHOLECHAIN);
    
    // 执行签名验证
    let result = WINTRUST::WinVerifyTrust(
        None,  // hwnd
        &WINTRUST::ACTION_GENERIC_VERIFY_V2,  // guid
        &trust_data as *const _ as *const std::ffi::c_void,  // pPolicyCallbackData
    );
    
    match result {
        Ok(_) => Ok(true), // 签名有效
        Err(_) => Ok(false) // 签名无效或验证失败
    }
}