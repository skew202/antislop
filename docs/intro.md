<div class="ascii-hero">
     █████╗ ███╗   ██╗████████╗██╗███████╗██╗      ██████╗ ██████╗ 
    ██╔══██╗████╗  ██║╚══██╔══╝██║██╔════╝██║     ██╔═══██╗██╔══██╗
    ███████║██╔██╗ ██║   ██║   ██║███████╗██║     ██║   ██║██████╔╝
    ██╔══██║██║╚██╗██║   ██║   ██║╚════██║██║     ██║   ██║██╔═══╝ 
    ██║  ██║██║ ╚████║   ██║   ██║███████║███████╗╚██████╔╝██║     
    ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚══════╝╚══════╝ ╚═════╝ ╚═╝     
</div>

# Introduction

**AntiSlop** is a blazing-fast, multi-language linter for detecting AI-generated code slop.

## What is Slop?

Slop is code where the AI model **cuts corners** to save tokens, reduce inference time, or "just make it compile." It is not about style or bugs, but about **laziness** and **incompleteness**.

AntiSlop detects:

- **Placeholders**: `TODO`, `FIXME`, `HACK` (deferring work)
- **Stubbing**: Empty functions, `pass`, `return null` (saving tokens)
- **Error Suppression**: `unwrap()`, `@ts-ignore`, `_ = err` (avoiding complexity)
- **Hedging**: "hopefully", "should work" (hallucination uncertainty)
- **Deferrals**: "for now", "temporary fix" (intent gap)

Unlike standard linters, AntiSlop focuses specifically on these artifacts of AI generation.
