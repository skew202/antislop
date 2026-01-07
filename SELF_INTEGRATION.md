# Antislop Self-Integration Guide
#
# This file documents how antislop integrates with itself
# during development without infinite recursion.

## Exclusion Strategy

# Exclude antislop's own test/example files and patterns
exclude = [
    # Test and example files (intentionally sloppy for testing)
    "examples/**",
    "tests/**",
    "benches/**",
    
    # Pattern definition files (contain regex patterns, not code)
    "config/patterns/**",
    ".antislop/profiles/**",
    
    # Generated/build artifacts
    "target/**",
    
    # External dependencies
    "vendor/**",
    "node_modules/**",
]

## Recommended Usage

# Run on source code only (production code)
# antislop src/

# For CI, use the standard profile with exclusions
# antislop --profile antislop-standard src/

# For strict checking before release
# antislop --profile antislop-strict src/ --disable naming

## Pre-commit Hook Integration

# Add to .pre-commit-config.yaml:
# repos:
#   - repo: local
#     hooks:
#       - id: antislop
#         name: antislop
#         entry: antislop --profile antislop-standard
#         language: system
#         files: \.(rs|py|js|ts|go)$
#         exclude: ^(examples/|tests/|benches/)

## CI Integration

# Add to .github/workflows/ci.yml:
# - name: Run antislop
#   run: |
#     cargo install antislop
#     antislop --profile antislop-standard src/
#     # Exit code 0 = clean, 1 = findings, 2 = error

## Development Workflow

# 1. Normal development: scan src/ only
#    antislop src/

# 2. Before commit: run with standard profile
#    antislop --profile antislop-standard src/

# 3. Before release: run strict (expect some findings)
#    antislop --profile antislop-strict src/ --disable naming

# 4. Suppress known findings: add to antislop.toml
#    [suppress]
#    files = ["src/detector/tree_sitter.rs"]
#    patterns = ["(?i)todo"]
