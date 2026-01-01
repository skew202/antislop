# antislop

[![CI](https://github.com/skew202/antislop/actions/workflows/ci.yml/badge.svg)](https://github.com/skew202/antislop/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/antislop.svg)](https://crates.io/crates/antislop)
[![npm](https://img.shields.io/npm/v/antislop.svg)](https://www.npmjs.com/package/antislop)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/skew202/antislop)

**A blazing-fast, multi-language linter for detecting AI-generated code slop.**

Antislop helps you maintain code quality by identifying lazily generated code, deferrals, hedging, and placeholders often left behind by AI coding assistants.

## What is Slop?

AI models often produce code that works but is littered with signs of hesitation or incompleteness. Antislop detects:

- **Placeholders**: `TODO`, `FIXME`, `HACK`, `XXX` comments
- **Deferrals**: "for now", "temporary fix", "quick implementation"
- **Hedging**: "hopefully", "should work", "assumes"
- **Stubs**: Empty functions or macro stubs like `todo!()`
- **Noise**: Redundant comments like `// increments i`

## Philosophy

Antislop is built on **First Principles**:
1.  **Code is Liability**: Every line of code is a future maintenance cost.
2.  **Intent != Implementation**: Comments like `TODO` or `for now` signal a gap between what was intended and what was built.
3.  **Speed is a Feature**: Verification must be instant to be useful.

We believe that AI generated code should be treated with **Zero Trust**. Verify everything.

### Pattern Hygiene (MECE)

We follow a **Mutually Exclusive, Collectively Exhaustive** strategy with standard linters like MegaLinter.
*   **Antislop**: Detects AI shortcuts (stubs, hallucinated API usage, hedging).
*   **Standard Linters**: Detect syntax errors, style issues, and bugs.
*   **Rule**: If `eslint` or `clippy` catches it by default, Antislop will **not** cover it (unless explicitly whitelisted).

## Comparison

| Feature | Antislop | Standard Linters (ESLint/Clippy) | AI Refactors |
|:--------|:---------|:---------------------------------|:-------------|
| **Focus** | **Intent & Completeness** | Syntax & Best Practices | Improvements |
| **Speed** | **Milliseconds** | Seconds/Minutes | Slow |
| **Parsing** | **Hybrid (Regex + AST)** | AST Only | AST/LLM |
| **Target** | **AI Slop** | Bugs/Style | Refactoring |
| **LSP** | **Yes** | Yes | Sometimes |

## Demo

```
$ antislop src/

  src/main.rs:42:15: CRITICAL [stub]
      ! Empty function body found
      → fn calculate_metrics() { todo!() }

  src/legacy.py:10:5: MEDIUM [deferral]
      ! Deferral phrase detected
      → # for now we just return True

  src/utils.js:5:1: LOW [noise]
      ! Redundant comment
      → // utility function

  ────────────────────────────────────────────────────────────
  📁 15 scanned, 3 with findings
  ⚠ 3 total findings
  💀 75 sloppy score

  ⚠⚠⚠ High slop detected
```

## Performance

Antislop uses tree-sitter AST parsing for accurate detection. Regex-only mode is ~10x faster.

| Language | Mode | Time | Throughput |
|:---------|:-----|:-----|:-----------|
| **Python** | AST | **4.0 ms** | 416 KiB/s |
| **JavaScript** | AST | **1.5 ms** | 856 KiB/s |
| **TypeScript** | AST | **4.9 ms** | 381 KiB/s |
| **Go** | AST | **1.3 ms** | 1.2 MiB/s |
| **Rust** | AST | **3.6 ms** | 606 KiB/s |
| **Python** | Regex | **0.47 ms** | — |
| **Rust** | Regex | **0.54 ms** | — |

**Scaling:**
| Lines | Time |
|:------|:-----|
| 1,000 | **10.5 ms** |
| 10,000 | **78 ms** |
| 50,000 | **392 ms** |

*Benchmarks run on standard laptop hardware (Linux x86_64). Run `cargo bench` to reproduce.*

## Installation

### From Pre-built Binaries (Recommended)

```bash
# via cargo-dist (Shell)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/skew202/antislop/releases/latest/download/antislop-installer.sh | sh

# via Homebrew
brew install skew202/tap/antislop

# via PowerShell
powershell -c "irm https://github.com/skew202/antislop/releases/latest/download/antislop-installer.ps1 | iex"
```

### From Source

```bash
cargo install antislop
```

## Usage

```bash
# Scan current directory
antislop

# Scan specific paths
antislop src/ tests/

# JSON output (for integration)
antislop --json

# Custom config
antislop -c custom-config.toml
antislop -c custom-config.toml
```

## Installation Options

Antislop is modular. You can optimize for binary size by choosing specific languages:

```bash
# Default (Standard Languages)
cargo install antislop

# Minimal (Regex only, no AST)
cargo install antislop --no-default-features --features parallel

# Specific Languages (Tiny binary)
cargo install antislop --no-default-features --features python,rust

# All Languages (inc. C#, PHP, Ruby, Kotlin, etc.)
cargo install antislop --features all-langs
```

## Configuration

Create `antislop.toml` in your project root to customize patterns:

```toml
file_extensions = [".py", ".rs", ".js", ".ts"]

[[patterns]]
regex = "(?i)TODO:"
severity = "medium"
message = "Placeholder comment found"
category = "placeholder"
```

## Language Support

| Language | Extension | Support Level |
|----------|-----------|---------------|
| C/C++ | `.c`, `.cpp` | Full (AST + Regex) |
| C# | `.cs` | Full (AST + Regex) |
| Go | `.go` | Full (AST + Regex) |
| Go | `.go` | Full (AST + Regex) |
| Haskell | `.hs` | Full (AST + Regex) |
| Java | `.java` | Full (AST + Regex) |
| JavaScript | `.js` | Full (AST + Regex) |
| Kotlin | `.kt` | Regex Only |
| Lua | `.lua` | Full (AST + Regex) |
| Perl | `.pl` | Regex Only |
| PHP | `.php` | Regex Only |
| Python | `.py` | Full (AST + Regex) |
| R | `.r` | Regex Only |
| Ruby | `.rb` | Full (AST + Regex) |
| Rust | `.rs` | Full (AST + Regex) |
| Scala | `.scala` | Full (AST + Regex) |
| Swift | `.swift` | Regex Only |
| TypeScript | `.ts` | Full (AST + Regex) |

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
