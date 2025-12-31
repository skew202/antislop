# Introduction

Antislop is a blazing-fast, multi-language linter that detects AI-generated code slop.

## What is Slop?

Slop is low-effort, incomplete, or hedged code commonly produced by quantized LLMs and AI coding assistants. Examples include:

- Untracked TODO comments without issues
- Temporary code that becomes permanent
- Uncertainty language like "hopefully this works"
- Empty stub functions

## Why Antislop?

Unlike traditional linters that check syntax and style, Antislop identifies patterns that indicate rushed or AI-generated code that needs human review.
