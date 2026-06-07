# References Index — Icon Creator 参考手册总览

本目录是 `icon-creator` skill 的设计与交付知识库。`SKILL.md` 负责执行规则，`references/` 负责提供可查的设计指南、正反案例、SVG 合约与交付配方。

## 推荐阅读路径

### 1. 做新设计

1. `01-brief-and-decision-tree.md` — 判断用户到底需要 Logo、App Icon 还是 Functional Icon。
2. `02-positive-negative-examples.md` — 查看正反案例，避免常见误区。
3. `logo-design-guide.md` / `icon-design-guide.md` / `app-icon-styles-guide.md` — 深入对应资产类型。
4. `03-svg-contract-and-quality-gates.md` — 按 SVG 合约生产。
5. `04-delivery-recipes.md` — 使用 CLI 导出交付物。

### 2. 优化已有 SVG

1. `03-svg-contract-and-quality-gates.md` — 先判断文件是否合规。
2. `02-positive-negative-examples.md` — 判断设计问题属于哪类。
3. `functional-icon-grid.md` 或 `logo-design-guide.md` — 根据资产类型修正。
4. `cli-usage.md` — 导出验证。

### 3. 学习设计规范

- `big-company-design-specs.md` — 主流设计体系共性。
- `design-resources.md` — 工具、资源和参考源。
- `icon-vs-logo-distinction.md` — 资产边界。

## 文件职责

| 文件 | 职责 |
|---|---|
| `01-brief-and-decision-tree.md` | 需求澄清、资产类型判定、设计方向选择 |
| `02-positive-negative-examples.md` | 正反案例、反模式、修正策略 |
| `03-svg-contract-and-quality-gates.md` | SVG 文件合约、自动 / 人工质检标准 |
| `04-delivery-recipes.md` | logo、app icon、functional icon 的 CLI 导出配方 |
| `cli-usage.md` | `svg2icon` 完整命令用法 |
| `logo-design-guide.md` | Logo 设计原则、认知、记忆、品牌表达 |
| `icon-design-guide.md` | 功能图标设计原则、语义、交互、认知 |
| `app-icon-styles-guide.md` | App Icon 风格分类与平台适配 |
| `functional-icon-grid.md` | 成套功能图标的网格、线宽、状态规范 |
| `icon-vs-logo-distinction.md` | Icon 与 Logo 的边界与混用风险 |
| `big-company-design-specs.md` | Google / Apple / 阿里 / 腾讯 / 字节 / 微软规范共性 |
| `design-resources.md` | 灵感、配色、工具与格式资料 |

## 总原则

1. `SKILL.md` 定义行为，`references/` 提供证据和方法。
2. 设计判断属于 skill，不属于 CLI。
3. 文件检查、格式导出、命名生成属于 CLI。
4. 所有源 SVG 统一使用 `512×512` 标准画板。
5. 所有交付资产应至少能形成 `primary / mono / reversed` 三类版本。
