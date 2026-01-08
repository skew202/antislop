# Contributing to AntiSlop

Thank you for your interest in contributing to AntiSlop!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/skew202/antislop.git
cd antislop

# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build --release
cargo test --all-features

# Run clippy
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt
```

## Quality Requirements

All contributions must pass:

| Check | Command | Required |
|-------|---------|----------|
| Format | `cargo fmt --check` | ✓ |
| Clippy | `cargo clippy -- -D warnings` | ✓ |
| Tests | `cargo test --all-features` | ✓ |
| Docs | `cargo doc --no-deps` | ✓ |

See [QA_STRATEGY.md](QA_STRATEGY.md) for our complete quality assurance approach.

## Running Tests

```bash
# Run all tests
cargo test --all-features

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run property-based tests
cargo test --test property_tests

# Update snapshots
cargo insta review
```

## Code Style

- Follow `rustfmt` formatting
- No clippy warnings allowed
- Document all public APIs with `///` docs
- Write tests for new functionality
- Add property tests for edge cases

## Submitting Changes

1. Fork the repository
2. Create a branch for your changes
3. Make your changes and add tests
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

## Adding New Patterns

1.  **Check Pattern Hygiene**: Ensure your pattern is **MECE** with standard linters.
    *   Run `python3 scripts/check_overlap.py` to verify no overlap.
    *   If `pylint`, `eslint`, or `clippy` catches it by default, do NOT add it.
2.  **Edit Config**: Edit `config/default.toml` (or `config/patterns/*.toml`) to add new patterns. Each pattern needs:
    *   `regex`: The pattern to match (supports `(?i)` for case-insensitive)
    *   `severity`: One of: low, medium, high, critical
    *   `message`: Human-readable description
    *   `category`: One of: placeholder, deferral, hedging, stub

## Adding Language Support

1. Add the language to `Language` enum in `src/detector/mod.rs`
2. Add tree-sitter grammar in `Cargo.toml` (optional)
3. Update `src/detector/tree_sitter.rs` with language support
4. Add tests for the new language
5. Update `docs/architecture.md` language table
