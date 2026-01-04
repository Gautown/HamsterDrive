# HamsterDrivers - Windows驱动管理工具

HamsterDrivers 是一个功能强大的Windows驱动管理工具，使用Rust语言编写，旨在帮助用户自动扫描、识别、比较厂商服务器上的驱动版本，并下载安装最新驱动。

## 项目特点

- **智能驱动管理**：自动识别硬件设备并查找最新驱动版本
- **安全可靠**：下载官方驱动，避免第三方驱动风险
- **用户友好**：直观的图形界面，操作简单
- **功能全面**：支持驱动更新、硬件信息查看、系统概览等功能
- **性能优异**：使用Rust语言开发，运行高效稳定

## 功能特性

- **硬件扫描**：自动扫描系统中的硬件设备，获取详细硬件信息
- **驱动检测**：智能识别当前驱动版本并与官方版本比较
- **一键更新**：支持一键下载并安装最新驱动程序
- **驱动回滚**：支持回滚到之前的驱动版本
- **系统信息**：显示详细的系统硬件和软件信息
- **定时更新**：支持定时自动检查驱动更新
- **批量操作**：支持批量更新多个驱动

## 技术栈

- **编程语言**：[Rust](https://www.rust-lang.org/) - 内存安全、高性能的系统编程语言
- **图形界面**：[egui](https://github.com/emilk/egui) - 简单易用的即时模式GUI库
- **Windows API**：[winsafe](https://github.com/rodrigocfd/winsafe) - 安全的Windows API包装库
- **硬件扫描**：WMI (Windows Management Instrumentation) - 获取硬件信息
- **下载工具**：[aria2](https://aria2.github.io/) - 高速多协议下载工具
- **异步运行时**：[Tokio](https://tokio.rs/) - Rust异步运行时

## 系统要求

- Windows 10 或更高版本
- 至少2GB可用内存
- 至少500MB可用磁盘空间
- Rust 工具链（如需从源码构建）

## 安装与使用

### 环境要求

- Windows 10 或更高版本
- Rust 工具链
- Windows SDK

### 安装步骤

1. 安装 Rust 工具链：访问 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. 安装 Windows C++ 构建工具：Visual Studio 或 Build Tools
3. 克隆项目仓库：
   ```bash
   git clone https://github.com/Gautown/HamsterDrivers.git
   cd HamsterDrivers
   ```
4. 构建并运行项目：
   ```bash
   cargo run
   ```

或者直接使用批处理文件：

- `run.bat` - 构建并运行项目
- `build.bat` - 仅构建项目

### 使用说明

1. 启动应用后，系统信息面板将显示当前系统信息
2. 点击"硬件扫描"按钮扫描系统硬件
3. 在驱动更新面板查看可更新的驱动
4. 选择需要更新的驱动并点击更新按钮

## 技术架构

```text
应用架构：Rust + egui + WMI + aria2
核心模块：
  - 硬件扫描：WMI + sysinfo + Windows API
  - 驱动管理：驱动匹配 + 下载管理 + 安装引擎
  - 用户界面：egui现代化界面
  - 系统信息：Windows版本 + 硬件信息 + DirectX信息
```
## 开发指南

### 项目结构

```
HamsterDrivers/
├── src/                    # Rust源代码
│   ├── core/              # 核心业务逻辑
│   ├── ui/                # 用户界面
│   ├── os_info/           # 系统信息获取
│   ├── hardware/          # 硬件扫描模块
│   ├── driver/            # 驱动管理模块
│   └── utils/             # 工具函数
├── assets/                # 静态资源
│   ├── font/              # 字体文件
│   └── images/            # 图片资源
├── Cargo.toml             # 项目配置
├── README.md              # 项目说明
├── INSTALL.md             # 安装指南
├── LICENSE                # 许可证
└── run.bat                # 运行脚本
```

### 核心模块

- `core/` - 驱动更新核心逻辑
- `ui/` - egui图形用户界面
- `os_info/` - 系统和硬件信息获取
- `hardware/` - 硬件扫描功能
- `driver/` - 驱动安装和管理

### 开发环境设置

```bash
# 安装Rust工具链
rustup install stable

# 安装Windows构建目标
rustup target add x86_64-pc-windows-msvc

# 安装构建依赖
cargo install cargo-make

# 运行开发版本
cargo run
```

## 贡献

欢迎提交Issue和Pull Request来改进项目。在提交之前，请确保：

1. 代码遵循Rust风格指南
2. 添加了适当的测试（如适用）
3. 更新了相关文档

## 许可证

本项目采用 [MIT 许可证](LICENSE)，详见 [LICENSE](LICENSE) 文件。

## 支持与反馈

- 📖 [使用文档](INSTALL.md)
- 🐛 [问题报告](https://github.com/Gautown/HamsterDrivers/issues)
- 💬 讨论区
- 📧 邮箱：zencomcn@live.com