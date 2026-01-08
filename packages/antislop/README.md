# AntiSlop

[![npm](https://img.shields.io/npm/v/antislop.svg)](https://www.npmjs.com/package/antislop)
[![CI](https://github.com/skew202/antislop/actions/workflows/ci.yml/badge.svg)](https://github.com/skew202/antislop/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/skew202/antislop)

**A blazing-fast, multi-language linter for detecting AI-generated code slop.**

AntiSlop catches what standard linters miss: placeholders, deferrals, hedging, and stubs left behind by AI coding assistants.

## Installation

```bash
npm install -g antislop
```

This downloads a platform-specific prebuilt binary.

## Usage

```bash
# Scan current directory
antislop

# Scan specific paths
antislop src/ tests/

# Use recommended profile
antislop --profile antislop-standard .

# JSON output for CI integration
antislop --json

# Run hygiene survey
antislop --hygiene-survey
```

## What it Detects

| Category | Examples |
|:---------|:---------|
| **Placeholders** | `TODO`, `FIXME`, `HACK` comments |
| **Deferrals** | "for now", "temporary fix" |
| **Hedging** | "hopefully", "should work" |
| **Stubs** | Empty functions, `todo!()` macros |

## CI/CD Integration

```yaml
# GitHub Actions
jobs:
  antislop:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm install -g antislop
      - run: antislop --profile antislop-standard .
```

## Documentation

- [Full Documentation](https://skew202.github.io/antislop/)
- [GitHub Repository](https://github.com/skew202/antislop)
- [crates.io Package](https://crates.io/crates/antislop)

## License

MIT OR Apache-2.0
