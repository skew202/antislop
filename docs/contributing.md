# Contributing

We welcome contributions! This page summarizes key info, see the full [CONTRIBUTING.md](https://github.com/skew202/antislop/blob/main/CONTRIBUTING.md) for complete details.

## Development Setup

```bash
git clone https://github.com/skew202/antislop.git
cd antislop

# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build --release
cargo test --all-features

# Lint
cargo clippy --all-features -- -D warnings
cargo fmt
```

## Quality Requirements

All contributions must pass:

| Check | Command |
|-------|---------|
| Format | `cargo fmt --check` |
| Clippy | `cargo clippy -- -D warnings` |
| Tests | `cargo test --all-features` |
| Docs | `cargo doc --no-deps` |

See [QA_STRATEGY.md](https://github.com/skew202/antislop/blob/main/QA_STRATEGY.md) for complete QA approach.

## Running Tests

```bash
# All tests (99 total)
cargo test --all

# Unit tests only (51 tests)
cargo test --lib

# Integration tests (19 tests)
cargo test --test integration_tests

# Property-based tests (5 tests)
cargo test --test property_tests

# Snapshot tests (5 tests)
cargo test --test snapshot_tests

# Edge case tests (9 tests)
cargo test --test edge_cases

# CLI output tests (9 tests)
cargo test --test cli_output_tests

# Update snapshots
cargo insta review
```

## Code Style

- Follow `rustfmt` formatting
- No clippy warnings allowed
- Document all public APIs with `///` docs
- Write tests for new functionality
- Add property tests for edge cases

## Adding Languages

1. Add `Language` variant in `src/detector/mod.rs`
2. Add tree-sitter dependency in `Cargo.toml` (optional)
3. Implement extractor in `src/detector/tree_sitter.rs`
4. Add tests for the new language
5. Update `docs/architecture.md` language table

## Adding Patterns

1.  **Check Pattern Hygiene**: Run `python3 scripts/check_overlap.py`. We follow a MECE strategy with standard linters.
2.  **Update Config**: Edit files in `config/patterns/` with your pattern definition:

```toml
[[patterns]]
regex = "(?i)your_pattern"
severity = "medium"
message = "Description of what was found"
category = "placeholder"  # or deferral, hedging, stub
```
