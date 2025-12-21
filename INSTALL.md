# HamsterDrive 环境安装指南

## 必需环境

1. **Rust 工具链**
2. **Windows开发环境** (Visual Studio C++构建工具或MinGW/MSYS2)

## 安装步骤

### 1. 安装 Rust 工具链
- 访问 https://www.rust-lang.org/zh-CN/tools/install
- 下载 rustup-init.exe
- 运行安装程序
- 按照提示完成安装
- 重启命令提示符
- 验证安装：`rustc --version` 和 `cargo --version`

### 2. 安装Windows开发工具

在Windows上编译Rust项目，您需要安装C++构建工具：

**选项1：Visual Studio C++构建工具**
- 下载 Visual Studio Community: https://visualstudio.microsoft.com/zh-hans/downloads/
- 安装时选择"C++桌面开发"工作负载

**选项2：MinGW/MSYS2**
- 下载 MSYS2: https://www.msys2.org/
- 安装完成后，在MSYS2终端中运行：
  ```bash
  pacman -S mingw-w64-x86_64-gcc
  pacman -S mingw-w64-x86_64-rust
  ```

## 运行项目

### 开发模式
```bash
# 构建并运行项目
cargo run
```

### 构建生产版本
```bash
# 构建优化版本
cargo build --release

# 运行优化版本
cargo run --release
```

## 常见问题

### 1. 如果遇到链接器错误
在Windows上，您可能需要安装Visual Studio C++构建工具：
- 下载 Visual Studio Community: https://visualstudio.microsoft.com/zh-hans/downloads/
- 安装时选择"C++桌面开发"工作负载

### 2. 如果使用GNU工具链
如果您更喜欢使用GNU工具链而不是MSVC：
```bash
rustup default stable-x86_64-pc-windows-gnu
```

## 项目结构

```
HamsterDrive/
├── src/              # Rust源代码
├── Cargo.toml        # Rust项目配置
└── README.md         # 项目说明
```