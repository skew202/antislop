# Installation

## From Pre-built Binaries (Recommended)

```bash
# Shell (Linux/macOS)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/skew202/antislop/releases/latest/download/antislop-installer.sh | sh

# Homebrew
brew install skew202/tap/antislop

# PowerShell (Windows)
powershell -c "irm https://github.com/skew202/antislop/releases/latest/download/antislop-installer.ps1 | iex"
```

## From crates.io

```bash
cargo install antislop
```

## From Source

```bash
git clone https://github.com/skew202/antislop.git
cd antislop
cargo build --release
```

The binary will be available at `target/release/antislop`.

## Feature Flags

Antislop is modular. Optimize for binary size by choosing specific languages:

```bash
# Default (Standard Languages)
cargo install antislop

# Minimal (Regex only, no AST)
cargo install antislop --no-default-features --features parallel

# Specific Languages
cargo install antislop --no-default-features --features python,rust

# All Languages
cargo install antislop --features all-langs
```
