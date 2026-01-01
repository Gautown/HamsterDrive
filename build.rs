//! HamsterDrive 构建脚本
//! 用于设置 Windows 资源文件（图标、版本信息等）

fn main() {
    // 仅在 Windows 平台上设置资源
    #[cfg(windows)]
    {
        // 检查是否存在资源文件
        let res_path = std::path::Path::new("assets/manifests/app.manifest");
        let icon_path = std::path::Path::new("assets/icons/app_icon.ico");

        let mut res = winres::WindowsResource::new();

        // 设置版本信息
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0000_0000_0001_0000);
        res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0000_0000_0001_0000);

        // 设置应用程序信息
        res.set("ProductName", "HamsterDrive");
        res.set("FileDescription", "Windows驱动管理工具 - 扫描、识别、更新驱动程序");
        res.set("LegalCopyright", "Copyright © 2024 HamsterDrive Team");
        res.set("CompanyName", "HamsterDrive Team");
        res.set("OriginalFilename", "HamsterDrive.exe");

        // 如果图标文件存在，设置应用程序图标
        if icon_path.exists() {
            res.set_icon(icon_path.to_str().unwrap());
        }

        // 如果清单文件存在，设置应用程序清单
        if res_path.exists() {
            res.set_manifest_file(res_path.to_str().unwrap());
        } else {
            // 使用内置清单请求管理员权限
            res.set_manifest(
                r#"
                <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
                    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                        <security>
                            <requestedPrivileges>
                                <requestedExecutionLevel level="asInvoker" uiAccess="false" />
                            </requestedPrivileges>
                        </security>
                    </trustInfo>
                    <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
                        <application>
                            <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
                            <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
                            <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>
                            <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}"/>
                            <supportedOS Id="{e2011457-1546-43c5-a5fe-008deee3d3f0}"/>
                        </application>
                    </compatibility>
                    <application xmlns="urn:schemas-microsoft-com:asm.v3">
                        <windowsSettings>
                            <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
                            <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
                        </windowsSettings>
                    </application>
                </assembly>
                "#,
            );
        }

        // 编译资源
        if let Err(e) = res.compile() {
            eprintln!("警告: 编译Windows资源失败: {}", e);
            // 不使构建失败，只是警告
        }
    }

    // 打印重新编译触发器
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/icons/app_icon.ico");
    println!("cargo:rerun-if-changed=assets/manifests/app.manifest");
}
