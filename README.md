# 🎨 Icon Creator

一个专业级的 SVG 图标生成工具链。包含：

- **svg2icon** — Rust CLI 工具，将标准 SVG 资产转换为多尺寸交付文件（PNG / JPEG / ICO / ICNS）
- **SKILL.md** — AI 辅助的 SVG 图标 / 标志设计指南

## 快速开始

### 下载可执行文件

从 [GitHub Releases](https://github.com/yann-yee/icon-creator/releases) 下载对应平台的二进制文件：

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

## CLI 的定位

`svg2icon` 是 skill 的**内置交付工具**，负责把已经设计完成、并符合规范的 SVG 资产导出为可交付的位图与平台图标文件。

它不负责设计判断，而负责：

- 在导出前执行 **SVG 质量检查**
- 导出 `primary / mono / reversed` 三类正式交付变体
- 生成多尺寸、多格式交付文件
- 依据输入 SVG 文件名自动生成规范化输出文件名

## 输出格式

| 格式 | 扩展名 | 说明 |
|------|--------|------|
| PNG  | `.png` | 网络 / 通用用途 |
| JPEG | `.jpg` | 照片级输出（无透明，默认白底） |
| ICO  | `.ico` | Windows 图标（多尺寸打包） |
| ICNS | `.icns` | macOS 应用图标 |

## 主要能力

- **导出前质量检查**
  - 检查 `viewBox="0 0 512 512"`
  - 检查是否包含 `<image>` 位图嵌入
  - 检查是否存在外部资源引用
  - 对 `width / height`、`shape-rendering`、`<text>` 等给出警告
- **正式变体导出**
  - `primary`：原始主版本
  - `mono`：单色黑色版本
  - `reversed`：反白版本
- **自动命名**
  - 输入 `logo.svg`
  - 输出示例：
    - `logo-primary-512x512.png`
    - `logo-mono-128x128.png`
    - `logo-reversed.ico`
- **高质量缩放**
  - 支持超采样导出（默认 2×，可提高到 3×/4×）

## 参考文献

- [References Index](references/00-reference-index.md)
- [Brief & Decision Tree](references/01-brief-and-decision-tree.md)
- [Positive / Negative Examples](references/02-positive-negative-examples.md)
- [SVG Contract & Quality Gates](references/03-svg-contract-and-quality-gates.md)
- [Delivery Recipes](references/04-delivery-recipes.md)
- [CLI 使用说明](references/cli-usage.md)
- [标志设计流程](references/logo-design-process.md)
- [图标风格指南](references/app-icon-styles-guide.md)
- [功能图标网格](references/functional-icon-grid.md)
- [设计资源](references/design-resources.md)
- [图标与 Logo 区分指南](references/icon-vs-logo-distinction.md)
- [图标设计指南](references/icon-design-guide.md)
- [Logo 设计指南](references/logo-design-guide.md)
- [大厂设计规范](references/big-company-design-specs.md)

## License

MIT
