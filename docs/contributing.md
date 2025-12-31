# Contributing

We welcome contributions! See [CONTRIBUTING.md](https://github.com/user/antislop/blob/main/CONTRIBUTING.md) for details.

## Development Setup

```bash
git clone https://github.com/user/antislop.git
cd antislop
cargo build --release
cargo test --all-features
```

## Running Tests

```bash
# All tests
cargo test --all-features

# With output
cargo test --all-features -- --nocapture

# Specific test
cargo test test_name -- --exact
```

## Code Style

- `cargo fmt` for formatting
- `cargo clippy --all-features -- -D warnings` for linting
- Document all public APIs

## Adding Languages

1. Add `Language` variant in `src/detector/mod.rs`
2. Add tree-sitter dependency in `Cargo.toml`
3. Implement extractor in `src/detector/tree_sitter.rs`
4. Add tests

## Adding Patterns

Edit `config/default.toml` with your pattern definition.
