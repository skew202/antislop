# Introduction

**Antislop** is a blazing-fast, multi-language linter for detecting AI-generated code slop.

## What is Slop?

Slop is code where the AI model **cuts corners** to save tokens, reduce inference time, or "just make it compile." It is not about style or bugs, but about **laziness** and **incompleteness**.

Antislop detects:

- **Placeholders**: `TODO`, `FIXME`, `HACK` (deferring work)
- **Stubbing**: Empty functions, `pass`, `return null` (saving tokens)
- **Error Suppression**: `unwrap()`, `@ts-ignore`, `_ = err` (avoiding complexity)
- **Hedging**: "hopefully", "should work" (hallucination uncertainty)
- **Deferrals**: "for now", "temporary fix" (intent gap)

## Philosophy

Antislop is built on **First Principles**:

1. **Code is Liability**: Every line of code is a future maintenance cost.
2. **Intent ≠ Implementation**: Comments like `TODO` signal unfinished work.
3. **Not a Linter**: We do not check syntax, style, or best practices. We check for **shortcuts**.
4. **Speed is a Feature**: Verification must be instant.

We believe that AI generated code should be treated with **Zero Trust**. Verify everything.

## Comparison

| Feature | Antislop | Standard Linters | AI Refactors |
|:--------|:---------|:-----------------|:-------------|
| **Focus** | Intent & Completeness | Syntax & Best Practices | Improvements |
| **Speed** | Milliseconds | Seconds/Minutes | Slow |
| **Parsing** | Hybrid (Regex + AST) | AST Only | AST/LLM |
| **Target** | AI Slop | Bugs/Style | Refactoring |
| **LSP** | Yes | Yes | Sometimes |

## Performance

| File Type | Lines | Time |
|:----------|:------|:-----|
| Python (Clean) | ~50 | 45 µs |
| Python (Sloppy) | ~50 | 52 µs |
| Large File | 1,000 | 4.3 ms |
| Huge File | 10,000 | ~45 ms |

*Benchmarks run on standard laptop hardware.*
