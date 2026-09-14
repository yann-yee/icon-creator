# References Index — Icon Creator V2

`SKILL.md` 负责决策协议；`references/` 提供设计证据、方法和生产细则。

V2 的核心变化是把设计过程从“风格 + 几何 + SVG”前移为：

```text
Product Meaning
→ Aesthetic Thesis
→ Symbol Strategy
→ Concept Critique
→ Form Language
→ Vector Engineering
→ Contextual Review
```

## 推荐阅读路径

### 新 Logo / App Icon

1. `05-product-meaning-and-aesthetic-thesis.md` — 先理解产品与美学内核。
2. `06-symbol-strategy-and-concept-critique.md` — 建立 Symbol Territory、概念候选和 critique。
3. `01-brief-and-decision-tree.md` — 判断资产类型、最少追问和上下文复用。
4. `logo-design-guide.md` / `app-icon-styles-guide.md` — 对应资产类型的深度设计资料。
5. `02-positive-negative-examples.md` — 检查常见反模式。
6. `03-svg-contract-and-quality-gates.md` — 生产 SVG。
7. `04-delivery-recipes.md` — 导出交付物。

### Functional Icon Set

1. `01-brief-and-decision-tree.md` — 明确功能语义与真实使用位置。
2. `icon-design-guide.md` — 认知和交互原则。
3. `functional-icon-grid.md` — family grammar、网格、线宽和状态。
4. `03-svg-contract-and-quality-gates.md` — SVG 合规。
5. `04-delivery-recipes.md` — 导出。

Functional Icon 的优先级是识别与一致性，不需要强行做品牌式抽象。

### 优化已有 SVG

1. 先判断问题在哪一层：Meaning / Aesthetic / Symbol / Form / Vector / Context。
2. 如果只是生产问题，直接用 `03-svg-contract-and-quality-gates.md`。
3. 如果是视觉/语义问题，回到 `05` / `06`，不要只做几何抛光。
4. 使用 `02-positive-negative-examples.md` 验证反模式。

### 只做格式导出

直接使用：

- `cli-usage.md`
- `04-delivery-recipes.md`

不要无意义重新设计已经批准的资产。

## 文件职责

| 文件 | 职责 |
|---|---|
| `01-brief-and-decision-tree.md` | 上下文复用、资产类型判定、最少追问 |
| `02-positive-negative-examples.md` | 正反案例、视觉反模式 |
| `03-svg-contract-and-quality-gates.md` | SVG 文件合约与质量门禁 |
| `04-delivery-recipes.md` | 导出配方 |
| `05-product-meaning-and-aesthetic-thesis.md` | Product Meaning Model、Aesthetic Thesis、Visual Tension、Anti-Aesthetic |
| `06-symbol-strategy-and-concept-critique.md` | Symbol Territories、抽象层级、Concept Critique |
| `logo-design-guide.md` | Logo 认知、记忆、品牌表达 |
| `icon-design-guide.md` | 功能图标语义、交互与认知 |
| `app-icon-styles-guide.md` | App Icon 风格与平台适配 |
| `functional-icon-grid.md` | Functional Icon family grammar |
| `icon-vs-logo-distinction.md` | Logo / App Icon / Functional Icon 边界 |
| `big-company-design-specs.md` | 主流设计体系参考 |
| `design-resources.md` | 设计资源 |
| `cli-usage.md` | `svg2icon` CLI 完整用法 |

## 总原则

1. **Meaning before Form**。
2. `SKILL.md` 不重复塞入所有技术细节；需要时再读对应 reference。
3. Product/feature context 优先复用项目已有 `context.md`、`DESIGN.md`、feature goal/design 文件。
4. 数学比例和网格是工具，不是审美正确性的证明。
5. Functional Icon 与 Logo 的目标不同，不共用同一种“独特性”标准。
6. CLI 只负责检查和导出，不做审美判断。
7. 默认 SVG source canvas 继续使用 `512×512`，这是生产规范，不是设计哲学。
