# 🎨 Icon Creator V2

一个 **product-aware** 的 SVG 图标 / 标志设计工具链。

V2 不再从“风格 + 几何”直接开始，而是先理解产品与功能，再形成美学内核和符号策略：

```text
Product Meaning
→ Aesthetic Thesis
→ Symbol Strategy
→ Concept Critique
→ Form Language
→ Vector Engineering
→ Contextual Review
→ Delivery
```

## 组件

- **SKILL.md** — AI 设计协议：理解产品、建立美学内核、选择 symbol strategy、生成并评审 SVG。
- **references/** — 产品理解、美学、Logo/App Icon/Functional Icon、SVG 合约和交付知识库。
- **svg2icon** — Rust CLI：检查已完成 SVG 并导出 PNG / JPEG / ICO / ICNS。

## V2 的核心变化

### Meaning before Form

设计前优先回答：

- 产品/功能本质是什么？
- 用户正在完成什么任务？
- 使用前 → 使用后发生什么 transformation？
- 它与同类真正不同在哪里？

### Aesthetic Thesis before Style

不再把“科技、极简、高级、蓝色”当作设计方向。

更推荐：

```text
Precise, but not sterile.
Quiet, but unmistakably technical.
```

通过 `Visual Tension + Anti-Aesthetic + Formal Consequences` 推导形态。

### Symbol Territory before Symbol

先探索语义 territory，再决定 Literal / Metonymic / Abstract / Letterform / Hybrid。

避免：

```text
网络产品 = 六边形 + 节点
AI 产品 = sparkle + brain
安全产品 = shield
```

### Geometry serves Intent

黄金比例、Fibonacci、网格和 Gestalt 都是工具，不是审美正确性的证明。

视觉判断优先级：

```text
Intent > optical balance > system consistency > mathematical elegance
```

## 项目上下文复用

当 Icon Creator 在产品仓库中运行时，优先复用已有：

1. `context.md`
2. `DESIGN.md`
3. `user_plan/<feature>/<feature>.md`
4. `user_plan/<feature>/design.md`
5. existing icon/logo assets

这样不需要重复向用户询问已经明确的产品目标和设计语言。

## Asset Modes

| 类型 | 主要目标 |
|---|---|
| Logo / Brand Mark | Meaning + Distinction |
| App Icon | Recognition ≈ Distinction |
| Functional Icon | Recognition > Originality |
| Audit + Revision | 找到问题所属层级再修复 |
| Delivery Export | 不重新设计，直接检查并导出 |

## 快速开始：设计知识

推荐阅读：

1. [Product Meaning & Aesthetic Thesis](references/05-product-meaning-and-aesthetic-thesis.md)
2. [Symbol Strategy & Concept Critique](references/06-symbol-strategy-and-concept-critique.md)
3. [Brief & Decision Tree](references/01-brief-and-decision-tree.md)
4. [References Index](references/00-reference-index.md)

生产阶段继续使用：

- [SVG Contract & Quality Gates](references/03-svg-contract-and-quality-gates.md)
- [Logo Design Guide](references/logo-design-guide.md)
- [Icon Design Guide](references/icon-design-guide.md)
- [App Icon Styles](references/app-icon-styles-guide.md)
- [Functional Icon Grid](references/functional-icon-grid.md)

## CLI 快速开始

从 [GitHub Releases](https://github.com/yann-yee/icon-creator/releases) 下载对应平台二进制：

```bash
# Linux / macOS
chmod +x svg2icon-linux
./svg2icon-linux --svg logo.svg -f ico

# Windows
svg2icon-win.exe --svg logo.svg -f ico
```

自行编译：

```bash
cd svg2icon
cargo build --release
```

## CLI 的定位

`svg2icon` 是交付引擎，不是设计师。

它负责：

- SVG 结构/生产质量检查；
- `primary / mono / reversed` 变体；
- 多尺寸、多格式导出；
- 自动命名。

它不负责：

- 产品理解；
- Aesthetic Thesis；
- symbol strategy；
- concept selection；
- 审美判断。

## 输出格式

| 格式 | 扩展名 | 说明 |
|---|---|---|
| PNG | `.png` | 网络 / 通用用途 |
| JPEG | `.jpg` | 无透明位图输出 |
| ICO | `.ico` | Windows 图标 |
| ICNS | `.icns` | macOS 应用图标 |

## License

MIT
