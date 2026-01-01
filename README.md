# HamsterDrive - Windows驱动管理工具

HamsterDrive 是一个Rust编写的功能强大的Windows驱动管理工具，旨在帮助用户扫描、识别、比较厂商服务器上的驱动版本，并下载安装最新驱动。
驱动程序自动更新工具，能够自动识别硬件、查找并安装最新的官方驱动。

## 功能特性

- 🔍 **自动硬件扫描**：支持PCI、USB、显示、网络、音频等设备识别
- 🗃️ **智能驱动匹配**：本地数据库+云端服务双重匹配机制
- ⚡ **多线程加速下载**：集成aria2支持多线程断点续传
- 🔄 **一键式更新**：自动下载、验证并安装驱动
- 💾 **安全备份**：自动创建系统还原点，支持驱动回滚
- 🎨 **现代化界面**：使用egui构建的响应式用户界面
- 📊 **系统信息展示**：详细的硬件和系统信息报告

## 系统要求

- Windows 10/11 (64位)
- 管理员权限（用于驱动安装）
- 至少2GB可用内存
- 至少500MB可用磁盘空间

## 快速开始

### 安装方式

1. **直接下载安装包**（推荐）
   - 从[Releases](https://github.com/driver-updater/windows-driver-updater/releases)下载最新安装包
   - 运行安装程序，按照向导完成安装

2. **从源代码构建**
   ```bash
   # 克隆项目
   git clone https://github.com/driver-updater/windows-driver-updater.git
   cd windows-driver-updater
   
   # 构建发布版本
   cargo build --release
   
   # 运行程序
   cd target/release
   driver-updater.exe
   使用说明
## 首次使用
- 1.以管理员身份运行程序  
- 2.点击"扫描硬件"按钮  
- 3.查看检测到的设备列表  
- 4.点击"检查更新"查找可用驱动  
- 5.选择要更新的设备，点击"更新"按钮  

### 主要功能
- *系统信息*：查看详细的硬件和Windows系统信息

- *批量更新*：支持同时更新多个设备的驱动

- *计划任务*：可设置定时自动检查更新

- *驱动备份*：自动备份当前驱动，支持一键回滚

- *日志查看*：详细的安装和更新日志

## 技术架构
```text
应用架构：Rust + egui + SQLite + aria2
核心模块：
  - 硬件扫描：sysinfo + WMI + SetupAPI
  - 驱动匹配：本地数据库 + 官网解析器
  - 下载管理：aria2多线程 + HTTP回退
  - 安装引擎：INF/EXE安装 + 系统还原点
```
## 安全特性
- 🔒 驱动验证：SHA256哈希验证 + WHQL数字签名检查  
- 🛡️ 权限控制：最小权限原则，仅在安装时需要管理员权限  
- 📜 透明操作：所有操作记录详细日志  
- 🔄 安全回滚：自动创建系统还原点，支持驱动回退
## 开发者文档
### 项目结构
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
````
## 贡献指南
- 我们欢迎各种形式的贡献！请参阅CONTRIBUTING.md了解详情。

## 许可证
- 本项目采用MIT许可证 - 详见LICENSE文件。
## 支持与反馈
- 📖 使用文档
- 🐛 问题报告
- 💬 讨论区
- 📧 邮箱：zencomcn@live.com