# Benchmark Suite

This directory contains the comprehensive benchmark suite for antislop.
The fixtures focuses on **AI shortcuts** (laziness), not general code quality issues.

## Philosophy

We benchmark against code that exhibits:
- **Token saving**: Stubs, empty catches, `pass`, `return null`
- **Error suppression**: `unwrap()`, `@ts-ignore`, `try/catch/pass`
- **Laziness**: `TODO`, `FIXME`, incomplete implementations

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench -- scan/python
cargo bench -- scan/scaling
```

## Benchmark Groups

| Group | Description |
|-------|-------------|
| `scan/python` | Python clean vs sloppy |
| `scan/javascript` | JavaScript clean vs sloppy |
| `scan/typescript` | TypeScript clean vs sloppy |
| `scan/go` | Go clean vs sloppy |
| `scan/rust` | Rust clean vs sloppy |
| `scan/scaling` | 100 to 50K lines scaling |
| `scan/regex_fallback` | Regex-only mode |
| `scan/mode_comparison` | Tree-sitter vs regex |
