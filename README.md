# 🎨 Icon Creator

一个专业级的 SVG 图标生成工具链。包含：

- **svg2icon** — Rust CLI 工具，将 SVG 转换为多尺寸高清图标（PNG / JPEG / ICO / ICNS）
- **SKILL.md** — AI 辅助的 SVG 图标/标志设计指南

## 快速开始

### 下载可执行文件

从 [GitHub Releases](https://github.com/你的用户名/icon-creator/releases) 下载对应平台的二进制文件：

```bash
# Linux / macOS
chmod +x svg2icon-linux
./svg2icon-linux --svg logo.svg -f ico

# Windows
svg2icon-win.exe --svg logo.svg -f ico
```

> 详细用法见 [CLI 使用说明](references/cli-usage.md)

### 自行编译

```bash
cd svg2icon
cargo build --release
```

## 输出格式

| 格式 | 扩展名 | 说明                       |
|------|--------|----------------------------|
| PNG  | `.png` | 网络/通用用途              |
| JPEG | `.jpg` | 照片级输出（白色背景）     |
| ICO  | `.ico` | Windows 图标（多尺寸打包）  |
| ICNS | `.icns`| macOS 应用图标              |

默认生成六种尺寸：1024×1024 ~ 32×32，通过超采样技术确保缩放后保持锐利。

## 参考文献

- [CLI 使用说明](references/cli-usage.md)
- [标志设计流程](references/logo-design-process.md)
- [图标风格指南](references/app-icon-styles-guide.md)
- [功能图标网格](references/functional-icon-grid.md)
- [设计资源](references/design-resources.md)

## License

MIT
