---
name: icon-creator
description: Product-aware SVG icon/logo designer. Understands product meaning and feature intent, forms an aesthetic thesis and symbol strategy, then engineers production-ready SVG assets and export variants. Use for Logo, App Icon, Functional Icon Set, icon review, SVG optimization, and delivery export.
argument-hint: 'Describe the product/feature, intended users and context, desired feeling or constraints. Existing context.md / DESIGN.md / feature design files will be reused when available.'
user-invocable: true
---

# Icon Creator V2 — Meaning Before Form

## Purpose

Translate **product meaning + aesthetic intent** into a distinctive visual symbol, then engineer that symbol into production-ready vector assets.

The skill is not a style vending machine and not a geometry exercise.

```text
Product Meaning
      ↓
Aesthetic Thesis
      ↓
Symbol Strategy
      ↓
Concept Exploration
      ↓
Form Language
      ↓
Vector Engineering
      ↓
Contextual Review
      ↓
Delivery
```

## Core Principles

1. **Meaning before Form** — understand what the product/feature changes before drawing.
2. **Aesthetic Thesis before Style Tags** — “precise × human” is more useful than “modern / tech / minimal”.
3. **Transformation over Object** — prefer the change created by the product over a literal industry object.
4. **Visual Tension over Generic Adjectives** — good identity often lives between two qualities.
5. **Symbol Territory before Symbol** — explore semantic territories before selecting a literal mark.
6. **Geometry serves Intent** — grids, ratios and Gestalt are tools, never the aesthetic goal.
7. **Distinction must be earned** — uniqueness should come from product meaning, not arbitrary weirdness.
8. **Contextual Review over Isolated Beauty** — the asset must work in the real product and usage size.

## Asset Modes

### Logo / Brand Mark

Primary objective: durable identity and differentiation.

Priority:

```text
Meaning + Distinction > literal recognizability
```

A logo may be abstract if its logic is strongly connected to the product/brand.

### App Icon

Primary objective: fast launcher recognition plus product identity.

Priority:

```text
Recognition ≈ Distinction
```

Do not simply place a complex full logo into a rounded square.

### Functional Icon Set

Primary objective: interaction clarity and consistency.

Priority:

```text
Recognition > Originality
```

For common actions, preserve established conventions. Functional icons are not mini logos.

### Audit + Revision

Understand the existing intent before changing geometry. Preserve valuable identity; repair semantic, optical, consistency or production problems.

### Delivery Export

If design is already approved, skip conceptual design and use `svg2icon` for checks and export.

---

# 1. Context Intake — Reuse Before Asking

Before asking the user for information, inspect available project context when accessible.

Preferred evidence:

1. `context.md` — Project Mission, strategic goals, users, terminology, Architecture/Product Intent.
2. `DESIGN.md` — existing visual language, UX principles, tokens, brand character.
3. `user_plan/<feature>/<feature>.md` — current feature Goal Model / Desired Outcome.
4. `user_plan/<feature>/design.md` — current feature Experience Intent.
5. existing logo/icon assets and adjacent UI.
6. the user's current description.

Do not make the user repeat information already established in these sources.

If context conflicts, surface the conflict instead of silently choosing one version.

---

# 2. Product Meaning Model

Before concept generation, establish the smallest useful model of what this asset represents.

Answer only the relevant questions:

```text
What is it?
Who is it for?
What job is the user trying to accomplish?
What transformation does the product/feature create?
What makes it meaningfully different?
What should it feel like?
What should it never feel like?
Where will this asset actually appear?
```

The most important fields are **Transformation**, **Differentiator**, and **Anti-Meaning**.

Example:

```text
Weak:
Product: backup tool
Object: hard drive

Better:
Before: valuable data feels vulnerable
After: data feels recoverable and protected
Transformation: uncertain → protected
Differentiator: automatic continuous recovery, not manual copying
```

Use `references/05-product-meaning-and-aesthetic-thesis.md` for deeper guidance.

## Clarification Policy

Ask only when an unresolved answer would materially change the concept.

Prefer 1–3 high-value questions. Do not run a branding questionnaire by default.

If enough evidence exists, proceed with explicit assumptions rather than blocking.

---

# 3. Aesthetic Thesis

Aesthetic direction must be expressed as a **thesis**, not a bag of style adjectives.

Good:

```text
Precise, but not sterile.
Quiet, but unmistakably technical.
```

```text
Playful intelligence without childishness.
```

Bad:

```text
modern / premium / tech / blue / minimal
```

## Required Components

For identity-oriented work, establish:

### Emotional Promise
What should the user feel when seeing it?

### Visual Tension
Choose the tension that best captures the product, for example:

- Technical × Human
- Powerful × Quiet
- Precise × Organic
- Playful × Serious
- Dense × Calm
- Familiar × Distinctive
- Stable × Dynamic

### Anti-Aesthetic
Explicitly state what the work must not drift toward, such as:

- generic SaaS
- crypto/web3 cliché
- gaming aggression
- childish toy aesthetic
- enterprise legacy software
- generic AI sparkle/brain/network cliché
- over-luxury black-and-gold branding

### Formal Consequences
Translate the thesis into form decisions:

- silhouette
- geometry
- symmetry/asymmetry
- corner character
- visual weight
- negative space
- rhythm
- color behavior
- texture/gradient allowance
- motion behavior if relevant

Style labels may be used only after this derivation.

---

# 4. Symbol Strategy

Do not jump directly from product keywords to a symbol.

First generate 2–4 **Symbol Territories**: semantic spaces that could represent the product.

Example for a knowledge product:

```text
Connection — relationships, recombination, graph
Memory — preservation, recall, continuity
Emergence — fragments becoming structure
Navigation — finding a path through complexity
```

Then choose an abstraction level:

- `LITERAL` — object/action directly depicted
- `METONYMIC` — related concept stands for the whole
- `ABSTRACT` — form expresses an underlying idea
- `LETTERFORM` — identity derived from name/initial
- `HYBRID` — combines two of the above with one dominant idea

Rules:

- Functional icons default toward `LITERAL` / conventional symbols.
- App icons often benefit from `METONYMIC` or restrained `HYBRID` logic.
- Brand marks may use `ABSTRACT` / `LETTERFORM` when ownability is stronger.
- Never combine multiple symbols simply to include every product keyword.

Read `references/06-symbol-strategy-and-concept-critique.md` when concept selection is non-trivial.

---

# 5. Concept Exploration

For meaningful identity work, explore distinct concepts before vector production.

Each concept should contain:

```text
Concept name
Core idea
Product connection
Aesthetic connection
Symbol territory
Abstraction level
Visual anchor
Why it could be ownable
Small-size behavior
Primary risk / likely misreading
```

Concepts must differ in **idea**, not merely color or corner radius.

Do not rank by a simplistic total score alone. Use critique:

- Is the concept specifically connected to this product?
- Does it embody the Aesthetic Thesis?
- Is the visual anchor memorable without explanation?
- Does it become generic when the brand name is removed?
- What is the most likely wrong interpretation?
- What survives at 16–32px?

If the user explicitly asks for immediate generation, select the strongest concept internally and briefly explain the rationale instead of forcing a multi-round approval ceremony.

---

# 6. Form Language

Only after concept selection should the design establish a visual grammar.

Define relevant dimensions:

- dominant primitive or contour logic
- filled vs outlined
- stroke character
- corner system
- symmetry / controlled asymmetry
- optical center
- visual weight
- positive/negative-space relationship
- spacing rhythm
- color hierarchy
- depth/gradient policy

## Geometry Policy

Use grids and ratios when they support the concept and consistency.

Never claim that a design is good because it uses φ, Fibonacci, √2, or another mathematical ratio.

```text
Intent → visual relationship → candidate geometry → optical correction
```

not:

```text
golden ratio → therefore good design
```

A mathematically perfect shape may require optical correction. Visual balance has priority over numerical purity.

---

# 7. Vector Engineering Contract

Detailed production rules live in references rather than bloating this skill.

Primary references:

- `references/03-svg-contract-and-quality-gates.md`
- `references/functional-icon-grid.md`
- `references/logo-design-guide.md`
- `references/app-icon-styles-guide.md`

Default source canvas remains:

```xml
<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512" shape-rendering="geometricPrecision">
```

Production invariants:

- vector-only; no `<image>` or embedded bitmap;
- no external runtime resources;
- semantic groups where useful;
- avoid gratuitous path complexity;
- maintain small-size readability;
- support mono/reversed when relevant;
- use optical correction rather than blindly forcing geometric centering;
- do not add mathematical comments unless they explain a real construction decision.

A 512 source canvas is a production convention, not the aesthetic foundation.

---

# 8. Contextual Review

Do not validate only on a blank white canvas.

Review in the actual expected context where possible.

## Meaning

- Does the mark still connect to the product/feature goal?
- Did implementation drift away from the selected concept?
- Does it accidentally communicate an unwanted category or promise?

## Aesthetic Thesis

- Does the final form still express the intended tension?
- Did it collapse into a generic trend?
- Does it violate the Anti-Aesthetic?

## Distinction

- Is there a clear visual anchor?
- Would a reasonable competitor plausibly use the same mark unchanged?
- Is distinctiveness coming from meaning/form, not decoration?

## Functional Quality

- small-size test;
- silhouette test;
- mono / reversed test;
- dark / light context;
- optical balance;
- icon-set consistency if applicable;
- platform crop/safe-area behavior if applicable.

## Product Context

For functional icons, review them next to adjacent controls rather than individually.
For app icons, review launcher-scale behavior.
For brand marks, review realistic header/favicon/social/mono contexts.

---

# 9. Delivery

`svg2icon` is the delivery engine, not the designer.

Use it after SVG design is conceptually approved.

Typical pipeline:

```text
approved SVG
   ↓
SVG quality gate
   ↓
primary / mono / reversed
   ↓
PNG / JPEG / ICO / ICNS
```

See:

- `references/cli-usage.md`
- `references/04-delivery-recipes.md`

Do not let export convenience change the visual concept.

---

# 10. Output Contracts

## Design / New Asset

Keep the visible output proportional to the request. A useful compact design rationale contains:

```text
Product Meaning
Aesthetic Thesis
Selected Symbol Strategy
Key form decisions
Known risk / tradeoff
```

Then provide or save the SVG/assets requested.

## Audit

Report issues by layer:

```text
MEANING
AESTHETIC
SYMBOL
FORM
VECTOR
CONTEXT
```

Do not repair a semantic problem with geometric polish.

## Functional Icon Set

Document the shared family rules once, then focus each icon on semantic clarity.

---

# 11. Replan Triggers

Return to an earlier layer when:

- new product context changes the meaning;
- a concept depends on a false product assumption;
- the chosen symbol is too generic or misleading;
- the concept cannot survive required display sizes;
- real product context conflicts with the assumed form language;
- an existing design system provides a stronger, more coherent pattern;
- user feedback changes the desired emotional promise rather than a surface preference.

Do not patch a failed concept indefinitely. Revisit the layer where the failure originated.

---

# 12. Completion Criteria

A design is ready when relevant conditions hold:

- the represented product/feature intent is understood;
- the asset has an explicit Aesthetic Thesis or intentionally inherits one from `DESIGN.md`;
- the symbol strategy has a defensible relationship to product meaning;
- the form language follows from the thesis rather than arbitrary style selection;
- small-size/context tests appropriate to the asset type pass;
- SVG production checks pass;
- remaining semantic or similarity risks are explicit;
- requested delivery variants are produced or a valid export path is provided.

The final question is not “Is the SVG mathematically neat?”

It is:

> **Does this symbol feel inevitable for this product, and does it still work as a real production asset?**
