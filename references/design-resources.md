# Logo 与图标设计工具与资源指南

来源：两份知乎 PDF 中推荐的所有外部资源

---

## 灵感与趋势

| 资源 | 用途 | 来源 |
|------|------|------|
| **[LogoLounge](https://www.logolounge.com)** | 以 logo 为中心的展示网站；年度 logo 趋势报告 | Logo 设计指南 |
| **[Dribbble](https://dribbble.com)** | 设计师社区，搜索「logo」「icon」获取视觉研究素材 | Logo 设计指南 |
| **[Pinterest](https://www.pinterest.com)** | 搜索现成的配色方案，观察色彩趋势 | Logo 设计指南 |
| **设计杂志 & 博物馆** | 浏览艺术和设计相关数据，前往博物馆、画廊获取跨领域灵感 | Logo 设计指南 |

## 配色工具

| 资源 | 用途 | 来源 |
|------|------|------|
| **[Adobe Color CC](https://color.adobe.com)** | 使用色盘创建配色方案；浏览社区数千种颜色组合 | Logo 设计指南 |
| **[COLOURlovers](https://www.colourlovers.com)** | 全球艺术家和设计师分享配色方案的创意社区 | Logo 设计指南 |
| **[Ctrl + Paint](https://www.ctrlpaint.com)** | 概念艺术家 Matt Kohr 的视频教程，从数字绘画角度解释颜色选择和混合 | Logo 设计指南 |
| **Pinterest 配色板** | 搜索现成配色方案（如「brand color palette」） | Logo 设计指南 |

## 设计工具与插件

| 工具/插件 | 用途 | 来源 |
|-----------|------|------|
| **Astute Graphics - SubScribe 插件** | AI 矢量图形高效工具，免费的 SubScribe 插件集非常有用 | Logo 设计指南 |
| **Illustrator Pathfinder** | 使用相交的形状快速创建新的形状和符号 | Logo 设计指南 |
| **Illustrator Shape Builder** | 用好了千变万化 | Logo 设计指南 |
| **Illustrator Pen Tool** | 钢笔工具——「笔中有力量」 | Logo 设计指南 |
| **Sketch** | 快速将图像组织成情绪板情景拼图 | Logo 设计指南 |
| **Adobe Illustrator 对齐工具** | 高效将对象与精度对齐 | Logo 设计指南 |

## 学习教程

| 资源 | 内容 | 来源 |
|------|------|------|
| **Ctrl + Paint（数字绘画）** | 颜色选择和混合的「原因」和「如何选择」 | Logo 设计指南 |
| **Exercises To Fuel Creative Thinking** | Translating 头脑风暴方法 | Logo 设计指南 |
| **Paul Rand 对 IBM logo 的设计流程（视频）** | 经典 logo 设计案例研究 | Logo 设计指南 |

## 平台渲染差异

| 平台 | 渲染特征 | SVG 补偿策略 |
|------|----------|-------------|
| **Windows** | 矢量渲染偏粗 | `stroke-width` 降低 1 档（如 24px→20px） |
| **macOS** | 矢量渲染偏细 | 保持标准 `stroke-width` |
| **iOS** | 自动为图标添加圆角遮罩 | 交付直角矩形图标，由系统裁切 |
| **Android** | 支持自适应图标（前景+背景） | 确保前景图形在安全区内（居中 66%） |

## 文件格式速查

| 格式 | 最佳场景 | 是否矢量 |
|------|----------|----------|
| **SVG** | 网页/开发——自包含、可缩放、可编程 | ✅ |
| **EPS** | 专业打印 | ✅ |
| **AI** | 源文件——可编辑 | ✅ |
| **PDF** | 文档嵌入与打印 | ✅ |
| **JPEG** | 多色/渐变 logo（照片级渲染） | ❌ |
| **PNG** | 平坦颜色 logo，需要透明背景 | ❌ |
| **GIF** | 简单动画场景 | ❌ |
