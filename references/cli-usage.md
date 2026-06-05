# 🎨 svg2icon CLI 使用说明

将 SVG 文件转换为多尺寸高清图标（PNG / JPEG / ICO / ICNS）。

## 1. 获取可执行文件

从 [GitHub Releases](https://github.com/yann-yee/icon-creator/releases) 页面下载对应平台的文件。

> ⚠️ **重要**：下载后请将可执行文件放置在**本技能的 `bin/` 目录**中（若不存在则手动创建）。  
> 这样便于在技能内统一管理工具，并在调用时使用一致的相对路径。

| 平台          | 文件名                            | 存放位置（示例）                  |
|---------------|-----------------------------------|----------------------------------|
| 🐧 Linux      | `svg2icon-linux`（静态链接）       | `技能目录/bin/svg2icon-linux`    |
| 🍎 macOS      | `svg2icon-mac`                  | `技能目录/bin/svg2icon-mac`   |
| 🪟 Windows    | `svg2icon-win.exe`                 | `技能目录/bin/svg2icon-win.exe`  |

### 赋予执行权限（Linux / macOS）

```bash
chmod +x 技能目录/bin/svg2icon-linux   # 请替换为实际路径
```

## 2. 基本用法（基于 bin/ 目录）

所有命令示例均假设当前工作目录为技能根目录，且可执行文件位于 `bin/` 文件夹下。

### 最简示例：生成 ICO 文件

```bash
# Linux / macOS
./bin/svg2icon-linux --svg logo.svg -f ico

# Windows
.\bin\svg2icon-win.exe --svg logo.svg -f ico
```

输出：`./icon.ico`（包含 1024×1024 ~ 32×32 共 6 种尺寸）

### 生成 PNG 并指定尺寸

```bash
./bin/svg2icon-linux --svg logo.svg -f png --sizes 256,128,64 -o ./output/
```

输出：`./output/icon_256x256.png`、`./output/icon_128x128.png`、`./output/icon_64x64.png`

### 生成 macOS ICNS

```bash
./bin/svg2icon-linux --svg logo.svg -f icns
```

输出：`./icon.icns`（可直接用于 macOS 应用图标）

### 生成高质量 JPEG（带白色背景）

```bash
./bin/svg2icon-linux --svg logo.svg -f jpg -q 90
```

## 3. 完整选项

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-s, --svg <PATH>` | 必填，输入的 SVG 文件路径 | — |
| `-f, --format <FMT>` | 输出格式：`png` / `jpg` / `ico` / `icns` | `png` |
| `--sizes <SIZES>` | 输出尺寸（逗号分隔），如 `256,128,64` | `1024,512,256,128,64,32` |
| `-o, --out-dir <DIR>` | 输出目录 | `.`（当前目录） |
| `-x, --supersample <N>` | 超采样倍率，越高越锐利但越慢（建议 2~4） | `2` |
| `-q, --quality <N>` | JPEG 质量（1–100），仅 `jpg` 格式生效 | `95` |
| `-h, --help` | 打印帮助信息 | — |
| `-V, --version` | 打印版本号 | — |

## 4. 进阶技巧

### 超采样提升清晰度

渲染母版尺寸 = 最大目标尺寸 × 超采样倍率。默认 2× 已足够；对需要极致锐度的场景可设为 3–4：

```bash
./bin/svg2icon-linux --svg logo.svg -f png -x 4 --sizes 256
```

### 批量生成多个 ICO

ICO 格式天然支持多尺寸，所有指定尺寸会打包到同一个 `.ico` 文件中：

```bash
./bin/svg2icon-linux --svg logo.svg -f ico --sizes 16,32,48,64,128,256
```

### 集成到构建脚本

```bash
# Makefile / package.json scripts 中调用（使用 bin/ 目录的相对路径）
./bin/svg2icon-linux --svg assets/icon.svg -f ico -o dist/
```

## 5. 错误排查

| 问题 | 原因与解决 |
|------|------------|
| SVG parse error | SVG 文件损坏或不兼容；尝试用浏览器另存为即可 |
| Unsupported ICNS size | ICNS 仅支持特定尺寸（16/32/48/128/256/512/1024）；调整 `--sizes` |
| Cannot create Pixmap | SVG 尺寸为 0 或解析失败；检查 SVG 是否正确 |
| Permission denied | 未赋予执行权限（Linux/macOS）：`chmod +x ./bin/…` |
| command not found | 未使用正确的相对路径，或未将 `bin/` 加入 PATH |

## 6. 各平台注意事项

- **macOS**：首次运行可能需要通过 Gatekeeper：
  ```bash
  xattr -d com.apple.quarantine ./bin/svg2icon-mac
  ```
- **Linux**：需要系统字体支持（含 emoji），部分无头服务器可能需要安装 `libfontconfig`
- **Windows**：直接双击运行会闪退，请在终端（cmd / PowerShell / Git Bash）中调用，并使用 `.\bin\svg2icon-win.exe`

> 💡 提示：如果你希望直接在命令行中使用 `svg2icon` 命令（而不加 `bin/` 路径），可以将 `技能目录/bin` 添加到系统的 PATH 环境变量中。但对于技能内调用，强烈建议使用 `./bin/` 相对路径，以确保可移植性。
