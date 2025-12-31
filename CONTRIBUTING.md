# Contributing to Antislop

Thank you for your interest in contributing to Antislop!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/user/antislop.git
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

## Running Tests

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test --all-features -- --nocapture

# Run specific test
cargo test test_name -- --exact
```

## Code Style

- Follow `rustfmt` formatting
- No clippy warnings allowed
- Document all public APIs with `///` docs
- Write tests for new functionality

## Submitting Changes

1. Fork the repository
2. Create a branch for your changes
3. Make your changes and add tests
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

## Adding New Patterns

Edit `config/default.toml` to add new slop detection patterns. Each pattern needs:
- `regex`: The pattern to match (supports `(?i)` for case-insensitive)
- `severity`: One of: low, medium, high, critical
- `message`: Human-readable description
- `category`: One of: placeholder, deferral, hedging, stub

## Adding Language Support

1. Add the language to `Language` enum in `src/detector/mod.rs`
2. Add tree-sitter grammar in `Cargo.toml` (optional)
3. Update `src/detector/tree_sitter.rs` with language support
4. Add tests in `src/detector/tree_sitter.rs`
