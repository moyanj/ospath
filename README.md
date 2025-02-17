# ospath

`ospath` 是一个简单的命令行工具，用于获取当前操作系统的各种路径和系统信息。

## 功能

`ospath` 支持以下命令来获取系统信息：

- **路径相关**：
  - `home`：获取用户主目录路径。
  - `config`：获取配置目录路径。
  - `data`：获取数据目录路径。
  - `cache`：获取缓存目录路径。
  - `state`：获取状态目录路径。
  - `audio`：获取音频目录路径。
  - `font`：获取字体目录路径。
  - `picture`：获取图片目录路径。
  - `video`：获取视频目录路径。
  - `document`：获取文档目录路径。
  - `download`：获取下载目录路径。
  - `public`：获取公共目录路径。
  - `template`：获取模板目录路径。
  - `desktop`：获取桌面目录路径。
  - `runtime`：获取运行时目录路径。
  - `config_local`：获取本地配置目录路径。
  - `data_local`：获取本地数据目录路径。
  - `executable`：获取可执行文件目录路径。
  - `preferences`：获取偏好设置目录路径。
  - `current_dir`：获取当前工作目录路径。
  - `temp_dir`：获取临时目录路径。

- **系统信息**：
  - `user`：获取当前用户名。
  - `os_name`：获取操作系统名称。
  - `os_family`：获取操作系统家族。
  - `os_arch`：获取操作系统架构。
  - `args`：获取命令行参数。
  - `time`：获取当前系统时间（格式：`YYYY-MM-DD HH:MM:SS`）。
  - `lib_ext`：获取动态库文件扩展名。
  - `exe_ext`：获取可执行文件扩展名。
  - `path_sep`：获取路径分隔符（Windows 为 `\`，其他系统为 `/`）。

## 安装

1. 确保已安装 [Rust](https://www.rust-lang.org/) 工具链。
2. 克隆项目仓库：
   ```bash
   git clone https://github.com/moyanj/ospath.git
   ```
3. 进入项目目录并构建：
   ```bash
   cd ospath
   cargo build --release
   ```
4. 构建完成后，可执行文件位于 `target/release/ospath`。

## 使用

运行 `ospath` 并指定命令来获取相应的系统信息。例如：

```bash
./ospath home          # 获取主目录路径
./ospath user          # 获取当前用户名
./ospath os_name       # 获取操作系统名称
./ospath time          # 获取当前系统时间
```

如果命令无效，程序会返回 `Invalid command`。

## 输出格式

`ospath` 支持以下输出格式：

- **Text**：默认格式，以可读的文本形式输出结果。
- **Json**：以 JSON 格式输出结果，适合程序化处理。

可以通过 `-f` 或 `--format` 参数指定输出格式。例如：

```bash
./ospath home -f json  # 以 JSON 格式输出主目录路径
```

## 依赖

- `clap`：用于解析命令行参数。
- `dirs`：用于获取系统目录路径。
- `chrono`：用于获取和格式化系统时间。
- `serde` 和 `serde_json`：用于 JSON 序列化。

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

