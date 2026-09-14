# Brief & Decision Tree V2 — Context First

本文件用于在开始设计前快速判断：**已有上下文告诉了我们什么、还缺什么、用户真正需要哪一类视觉资产。**

目标不是完成问卷，而是以最少认知成本建立足够可靠的设计输入。

## 1. Context Reuse First

如果在项目中调用，优先读取：

1. `context.md`
2. `DESIGN.md`
3. `user_plan/<feature>/<feature>.md`
4. `user_plan/<feature>/design.md`
5. 现有 logo/icon 资产与相邻 UI

从中提取：

- Project Mission / Product Essence
- 目标用户
- 当前 feature Desired Outcome
- 产品已有视觉语言
- 稳定设计规则
- 明确 Non-Goals / Anti-Aesthetic
- 实际使用位置与尺寸

已存在的信息不要重复询问用户。

## 2. Asset Type Decision

```text
用户是否只想转换/导出已有 SVG？
├─ 是 → Delivery Export
└─ 否
   ├─ 是否用于长期品牌识别？
   │  └─ Logo / Brand Mark
   ├─ 是否用于 App launcher / app store / desktop shortcut？
   │  └─ App Icon
   ├─ 是否表达 UI 操作、状态、导航或 feature action？
   │  └─ Functional Icon / Icon Set
   ├─ 是否已有视觉资产但觉得不对？
   │  └─ Audit + Revision
   └─ 信息不足 → 只追问会改变资产类型的关键问题
```

## 3. 三类资产的目标区别

| 类型 | 优先级 | 核心问题 |
|---|---|---|
| Logo | Meaning + Distinction | 为什么这个符号只适合这个品牌？ |
| App Icon | Recognition ≈ Distinction | 启动器里能否快速识别且有产品归属？ |
| Functional Icon | Recognition > Originality | 用户是否无需猜测就能理解操作？ |

不要把 Functional Icon 当成 Logo 设计，也不要把 Logo 降级成行业通用 pictogram。

## 4. Minimum Product Meaning Brief

只收集缺失项：

```text
What is it?
Who is it for?
User job:
Before → After transformation:
Meaningful differentiator:
Desired feeling:
Must NOT feel like:
Usage context:
```

对于 feature icon，重点不是“这个功能叫什么”，而是：

```text
用户当前处于什么状态？
触发这个功能后发生什么变化？
这个变化最值得被视觉化的是什么？
```

## 5. High-Value Clarification Questions

只有答案会改变概念时才问。

优先问题示例：

### Meaning

> 用户使用这个产品/功能前后，最核心的状态变化是什么？

### Differentiation

> 同类产品通常都能做什么，而你最希望用户记住你哪一点不同？

### Aesthetic

> 如果只能选一个张力，更接近“Technical × Human”还是“Powerful × Quiet”？

### Anti-Aesthetic

> 最不希望它看起来像哪类产品：通用 SaaS、游戏、Crypto、传统企业软件、AI cliché，还是其他？

### Context

> 这个图标最关键的真实使用位置和尺寸是什么？

默认最多追问 1–3 个问题。

## 6. Context Sufficiency Gate

满足以下条件即可进入 Aesthetic Thesis，不追求信息完美：

- 知道资产类型；
- 知道它代表的产品/feature；
- 知道最重要的 transformation 或 user job；
- 知道至少一个 differentiation / design constraint；
- 知道主要使用场景；
- 没有一个高影响未知会彻底改变语义。

低风险未知可以作为显式假设继续。

## 7. Compact Design Brief Output

不要输出长问卷。内部或用户可见 brief 建议压缩为：

```markdown
## Product Meaning
- Essence:
- User:
- Transformation:
- Differentiator:
- Usage context:

## Desired Feeling
- Emotional promise:
- Visual tension:
- Anti-aesthetic:

## Constraints
- Existing design language:
- Required platform/size:
- Existing symbols to preserve/avoid:
```

随后进入 `05-product-meaning-and-aesthetic-thesis.md`。
