# HamsterDrive 环境安装指南

## 必需环境

1. **Rust 工具链**
2. **Windows开发环境** (Visual Studio C++构建工具或MinGW/MSYS2)

## 安装步骤

### 1. 安装 Rust 工具链

**方法一：手动下载安装（推荐）**
1. 访问 https://www.rust-lang.org/zh-CN/tools/install
2. 点击"立即安装"按钮
3. 下载适用于Windows的rustup-init.exe文件
4. 双击下载的文件并按照安装向导完成安装
5. 安装完成后，关闭并重新打开命令提示符
6. 验证安装：
   ```bash
   rustc --version
   cargo --version
   ```

**方法二：使用winget安装（适用于Windows 10/11）**
1. 打开PowerShell
2. 运行命令：
   ```bash
   winget install --id Rustlang.Rustup
   ```
3. 安装完成后，关闭并重新打开命令提示符

**方法三：使用Chocolatey安装（如果您已安装Chocolatey）**
1. 打开命令提示符
2. 运行命令：
   ```bash
   choco install rust
   ```
3. 安装完成后，关闭并重新打开命令提示符

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
# 进入项目目录
cd G:\GitHub\HamsterDrive

# 构建并运行项目
cargo run
```

### 构建生产版本
```bash
# 进入项目目录
cd G:\GitHub\HamsterDrive

# 构建优化版本
cargo build --release

# 运行优化版本
cargo run --release
```

### 使用批处理文件运行
您也可以直接双击项目目录中的批处理文件：
- [run.bat](file:///g%3A/GitHub/HamsterDrive/run.bat) - 构建并运行项目
- [build.bat](file:///g%3A/GitHub/HamsterDrive/build.bat) - 仅构建项目

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