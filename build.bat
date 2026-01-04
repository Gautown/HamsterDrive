@echo off
echo 正在检查Rust环境...

rustc --version >nul 2>&1
if %errorlevel% == 0 (
    echo Rust已安装，正在构建项目...
    cargo build
    if %errorlevel% == 0 (
        echo 构建成功！
        echo 运行项目：cargo run
    ) else (
        echo 构建失败，请检查代码。
    )
) else (
    echo 未检测到Rust环境。
    echo 请从 https://www.rust-lang.org/tools/install 下载并安装Rust。
    echo 安装完成后重新运行此脚本。
)

pause