@echo off
echo 正在启动HamsterDrive GUI...

rustc --version >nul 2>&1
if %errorlevel% == 0 (
    echo Rust环境已就绪，正在启动GUI...
    cargo run
) else (
    echo 未检测到Rust环境。
    echo 请从 https://www.rust-lang.org/tools/install 下载并安装Rust。
)

pause