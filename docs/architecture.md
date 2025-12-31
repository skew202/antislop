# Architecture

## Overview

Antislop is built as a library with a CLI binary, enabling programmatic use and extensibility.

```
┌─────────────────────────────────────────────────────────┐
│                         CLI                            │
│  (args parsing, shell completion, output formatting)    │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                       Antislop Lib                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌──────────┐  ┌─────────┐  ┌─────────┐  │
│  │ Walker  │──│ Scanner  │──│Patterns │──│ Reporter │  │
│  └─────────┘  └──────────┘  └─────────┘  └─────────┘  │
├─────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────┐  │
│  │           Detector Module                         │  │
│  │  ┌─────────────┐  ┌──────────────────────────┐   │  │
│  │  │ Tree-sitter │  │   Regex Fallback         │   │  │
│  │  │  (Python,   │  │   (all languages)        │   │  │
│  │  │   JS, TS,   │  │                           │   │  │
│  │  │   Rust...)  │  │                           │   │  │
│  │  └─────────────┘  └──────────────────────────┘   │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Modules

### `walker`
Parallel file traversal with gitignore support using the `ignore` crate.

### `detector`
Core detection engine with:
- Pattern registry and compilation
- Tree-sitter integration for accurate comment extraction
- Regex fallback for unsupported languages
- Language detection from file extensions

### `config`
Configuration management with TOML support and layered defaults.

### `report`
Output formatting with human-readable colored terminal output and JSON export.

## Language Support

| Language | Tree-sitter | Regex Fallback |
|----------|-------------|----------------|
| Python | ✓ | ✓ |
| JavaScript | ✓ | ✓ |
| TypeScript | ✓ | ✓ |
| Rust | ✓ | ✓ |
| Go | ✓ | ✓ |
| Java | ✓ | ✓ |
| C/C++ | ✓ | ✓ |
| Ruby | ✗ | ✓ |
| PHP | ✗ | ✓ |
| Shell | ✗ | ✓ |
| Kotlin | ✗ | ✓ |
| Swift | ✗ | ✓ |
