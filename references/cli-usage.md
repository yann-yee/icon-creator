# 🎨 svg2icon CLI 使用说明

将符合 skill 规范的 SVG 文件转换为多尺寸高清交付文件（PNG / JPEG / ICO / ICNS）。

> `svg2icon` 的定位是 **导出工具**，不是设计工具。它负责把 skill 生成的标准 SVG 资产转换为正式交付物。

## 1. 获取可执行文件

从 [GitHub Releases](https://github.com/yann-yee/icon-creator/releases) 页面下载对应平台的文件。

> ⚠️ **重要**：下载后请将可执行文件放置在**本技能的 `bin/` 目录**中（若不存在则手动创建）。
> 这样便于在技能内统一管理工具，并在调用时使用一致的相对路径。

| 平台 | 文件名 | 存放位置（示例） |
|------|--------|------------------|
| 🐧 Linux | `svg2icon-linux`（静态链接） | `技能目录/bin/svg2icon-linux` |
| 🍎 macOS | `svg2icon-mac` | `技能目录/bin/svg2icon-mac` |
| 🪟 Windows | `svg2icon-win.exe` | `技能目录/bin/svg2icon-win.exe` |

### 赋予执行权限（Linux / macOS）

```bash
chmod +x 技能目录/bin/svg2icon-linux   # 请替换为实际路径
```

## 2. 导出前质量检查

每次导出前，CLI 都会自动执行质量检查；如果存在阻断性错误，导出会直接终止。

当前会检查：

- 是否声明 `viewBox="0 0 512 512"`
- 是否包含 `<image>` 位图嵌入
- 是否存在外部资源引用 / `href`
- `width / height` 是否建议性对齐到 `512×512`
- 是否设置 `shape-rendering="geometricPrecision"`
- 是否存在 `<text>` 字体依赖

如果你只想检查、不导出，可使用：

```bash
./bin/svg2icon-linux --svg logo.svg --check-only
```

## 3. 基本用法（基于 bin/ 目录）

所有命令示例均假设当前工作目录为技能根目录，且可执行文件位于 `bin/` 文件夹下。

### 最简示例：生成 ICO 文件

```bash
# Linux / macOS
./bin/svg2icon-linux --svg logo.svg -f ico

# Windows
.\bin\svg2icon-win.exe --svg logo.svg -f ico
```

输出示例：`./logo-primary.ico`

### 生成 PNG 并指定尺寸

```bash
./bin/svg2icon-linux --svg logo.svg --sizes 512,256,128 -o ./output/
```

输出示例：

- `./output/logo-primary-512x512.png`
- `./output/logo-primary-256x256.png`
- `./output/logo-primary-128x128.png`

### 生成 macOS ICNS

```bash
./bin/svg2icon-linux --svg logo.svg -f icns --sizes 16,32,128,256,512,1024
```

输出示例：`./logo-primary.icns`

### 生成多格式（一次输出 PNG + ICO）

```bash
./bin/svg2icon-linux --svg logo.svg -f png,ico --sizes 512,256,128,64
```

输出：PNG 各尺寸文件 + `logo-primary.ico`

### 导出 mono / reversed 正式变体

```bash
./bin/svg2icon-linux --svg logo.svg --variants primary,mono,reversed --sizes 512,256 -f png,ico
```

输出示例：

- `logo-primary-512x512.png`
- `logo-mono-512x512.png`
- `logo-reversed-512x512.png`
- `logo-primary.ico`
- `logo-mono.ico`
- `logo-reversed.ico`

### 指定背景色（默认透明）

```bash
# 蓝色背景的 PNG
./bin/svg2icon-linux --svg logo.svg --bg '#1a73e8'

# 透明背景（默认）
./bin/svg2icon-linux --svg logo.svg --bg transparent
```

### 生成高质量 JPEG

```bash
./bin/svg2icon-linux --svg logo.svg -f jpg -q 90
```

> `jpg` 不支持透明背景；若未指定 `--bg`，CLI 会自动使用白色背景。

## 4. 文件命名规则

CLI 会根据**输入 SVG 文件名**自动生成输出文件名，无需额外提供名称参数。

若输入文件为：

```text
logo.svg
```

则输出名称示例为：

```text
logo-primary-512x512.png
logo-mono-256x256.png
logo-reversed-128x128.png
logo-primary.ico
logo-mono.icns
```

命名规则：

- 位图文件：`<svg文件名>-<variant>-<size>x<size>.<ext>`
- 打包文件：`<svg文件名>-<variant>.<ext>`

## 5. 完整选项

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-s, --svg <PATH>` | 必填，输入的 SVG 文件路径 | — |
| `-f, --format <FMT>` | 输出格式，支持逗号分隔多值：`png` / `jpg` / `ico` / `icns` | `png` |
| `--sizes <SIZES>` | 输出尺寸（逗号分隔），如 `512,256,128` | `512` |
| `--variants <VARIANTS>` | 输出变体（逗号分隔）：`primary` / `mono` / `reversed` | `primary` |
| `-o, --out-dir <DIR>` | 输出目录 | `.`（当前目录） |
| `-x, --supersample <N>` | 超采样倍率，越高越锐利但越慢（建议 2~4） | `2` |
| `-q, --quality <N>` | JPEG 质量（1–100），仅 `jpg` 格式生效 | `95` |
| `--bg <COLOR>` | 背景色，hex 格式 `#RRGGBB` 或 `#RRGGBBAA`；不指定则透明（JPEG 默认白色） | — |
| `--check-only` | 仅执行质量检查，不导出文件 | `false` |
| `-h, --help` | 打印帮助信息 | — |
| `-V, --version` | 打印版本号 | — |

## 6. 进阶技巧

### 超采样提升清晰度

渲染母版尺寸 = 最大目标尺寸 × 超采样倍率。默认 2× 已足够；对需要极致锐度的场景可设为 3–4：

```bash
./bin/svg2icon-linux --svg logo.svg -f png -x 4 --sizes 256
```

### 生成完整品牌交付包

```bash
./bin/svg2icon-linux \
  --svg brand-mark.svg \
  --variants primary,mono,reversed \
  --sizes 512,256,128,64,32 \
  -f png,ico \
  -o dist/
```

### 集成到构建脚本

```bash
# Makefile / package.json scripts 中调用（使用 bin/ 目录的相对路径）
./bin/svg2icon-linux --svg assets/icon.svg --variants primary,mono -f png,ico -o dist/
```

## 7. 错误排查

| 问题 | 原因与解决 |
|------|------------|
| `SVG 未通过质量检查，导出已终止` | 先查看质量检查报告，修复 `viewBox`、`<image>`、外部引用等问题 |
| `SVG parse error` | SVG 文件损坏或不兼容；尝试用浏览器 / Figma / Illustrator 重新导出 |
| `Unsupported ICNS size` | ICNS 仅支持特定尺寸（16 / 32 / 48 / 128 / 256 / 512 / 1024）；调整 `--sizes` |
| `Cannot create Pixmap` | SVG 尺寸为 0 或解析失败；检查 SVG 是否正确 |
| `Permission denied` | 未赋予执行权限（Linux/macOS）：`chmod +x ./bin/...` |
| `command not found` | 未使用正确的相对路径，或未将 `bin/` 加入 PATH |

## 8. 各平台注意事项

- **macOS**：首次运行可能需要通过 Gatekeeper：
  ```bash
  xattr -d com.apple.quarantine ./bin/svg2icon-mac
  ```
- **Linux**：需要系统字体支持（含 emoji），部分无头服务器可能需要安装 `libfontconfig`
- **Windows**：直接双击运行会闪退，请在终端（cmd / PowerShell / Git Bash）中调用，并使用 `.\bin\svg2icon-win.exe`

> 💡 提示：如果你希望直接在命令行中使用 `svg2icon` 命令（而不加 `bin/` 路径），可以将 `技能目录/bin` 添加到系统的 PATH 环境变量中。但对于 skill 内调用，强烈建议使用 `./bin/` 相对路径，以确保可移植性。
