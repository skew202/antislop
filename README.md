# antislop

A blazing-fast, multi-language linter for detecting AI-generated code slop.

## What is Slop?

Antislop detects these common AI coding anti-patterns:

- **Placeholders**: TODO, FIXME, HACK, NOTE, XXX comments
- **Deferrals**: "for now", "temporary", "quick implementation"
- **Hedging**: "hopefully", "should work", "this is a simple"
- **Stubs**: Empty functions and placeholder code

## Installation

```bash
cargo install antislop
```

## Usage

```bash
# Scan current directory
antislop

# Scan specific paths
antislop src/ tests/

# JSON output
antislop --json

# Custom config
antislop -c custom-config.toml

# Specific file extensions
antislop -e .py,.rs,.js

# Verbose output
antislop -v
```

## Configuration

Create `antislop.toml` in your project root:

```toml
file_extensions = [".py", ".rs", ".js"]

[[patterns]]
regex = "(?i)TODO:"
severity = "medium"
message = "Placeholder comment found"
category = "placeholder"
```

## License

MIT OR Apache-2.0
