# Philosophy

AntiSlop is built on **First Principles**:

1.  **Code is Liability**: Every line of code is a future maintenance cost.
2.  **Intent != Implementation**: Comments like `TODO` or `for now` signal a gap between what was intended and what was built.
3.  **Speed is a Feature**: Verification must be instant to be useful.

We believe that AI generated code should be treated with **Zero Trust**. Verify everything.

## Zero False Positives

AntiSlop follows a **Zero False Positive** philosophy for its default core.

- **Core** (Default): Critical stubs & placeholders only. Zero false positives.
- **Strict** (`--profile antislop-strict`): Maximum coverage. Detects all forms of slop.

## Comparison

| Feature | AntiSlop | Standard Linters (ESLint/Clippy) | AI Refactors |
|:--------|:---------|:---------------------------------|:-------------|
| **Focus** | **Intent & Completeness** | Syntax & Best Practices | Improvements |
| **Speed** | **Milliseconds** | Seconds/Minutes | Slow |
| **Parsing** | **Hybrid (Regex + AST)** | AST Only | AST/LLM |
| **Target** | **AI Slop** | Bugs/Style | Refactoring |
| **LSP** | **Yes** | Yes | Sometimes |
