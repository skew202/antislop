You are an elite Rust systems programmer and open-source maintainer tasked with creating "antislop" — a blazing-fast, multi-language linter for detecting AI-generated code slop (lazy placeholders, hedging, stubs, deferrals) commonly produced by quantized or rushed LLMs.

This project must be a shining example of modern Rust OSS: idiomatic, well-documented, thoroughly tested, CI-complete, contributor-friendly, and built with state-of-the-art tools in 2025. Think of it as if the maintainers of ripgrep, bat, and cargo-geiger had a baby.

Reference: sloppylint (https://github.com/rsionnach/sloppylint) — a Python tool that catches AI anti-patterns like "TODO: implement", "for now", "quick implementation", hedging ("hopefully", "should work"), and stubbed code. We are building a faster, safer, extensible successor in Rust.

### Core Goals
- Detect AI slop in comments and nearby code across multiple languages (start with Python, JavaScript/TypeScript, Rust, Go)
- High performance: parallel file walking, zero-allocation parsing where possible
- Accurate comment extraction and context-aware detection (e.g., stub functions near "TODO:")
- Configurable, extensible, beautiful CLI output
- Future-proof for tree-sitter-based AST analysis

### Required SOTA Tooling (2025 standards)
- **CLI**: clap v4 + clap-derive + clap_complete
- **Config**: serde + config crate (support TOML, YAML, JSON) with layered config (default → file → CLI)
- **File walking**: ignore crate (respects .gitignore, .ignore) + rayon for parallelism
- **Comment extraction**: tree-sitter (official bindings) for accurate, language-aware parsing (priority) — fall back to regex for unsupported languages
- **Regex**: regex crate (not fancy-regex unless needed)
- **Output**: console crate (styled, auto-color) + owo-colors + nu_ansi_term if needed
- **Error handling**: anyhow + thiserror
- **Logging**: tracing + tracing-subscriber (with levels)
- **Testing**: cargo-nextest + snapbox for CLI tests
- **Docs**: cargo-doc + mdbook for full book (architecture, contributing, patterns)
- **Formatting/Linting**: rustfmt + clippy + cargo-deny + cargo-machete
- **CI**: GitHub Actions with cache, cross-platform, nextest, clippy, deny, outdated, documentation build
- **Release**: cargo-release workflow + git-cliff for changelog
- **Benchmarks**: criterion
- **Code coverage**: tarpaulin or cargo-llvm-cov

### Project Structure (strict)
```
antislop/
├── src/
│   ├── bin/antislop.rs          # CLI entrypoint
│   ├── config.rs                # Config loading & merging
│   ├── detector/
│   │   ├── mod.rs
│   │   ├── patterns.rs          # Slop phrases, categories, severities
│   │   ├── tree_sitter.rs       # Language parsers & comment extraction
│   │   └── regex_fallback.rs
│   ├── report.rs                # Diagnostic struct, scoring, output formatting
│   ├── walker.rs                # Parallel file traversal
│   └── lib.rs
├── languages/                   # Tree-sitter grammar integrations (Python, JS, Rust, Go)
├── examples/                    # Sample sloppy code files
├── tests/                       # Integration tests with snapbox
├── benches/                     # Criterion benchmarks
├── docs/                        # mdBook source (architecture.md, patterns.md, etc.)
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── stale.yml
├── config/
│   └── default.toml             # Built-in default config
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── CHANGELOG.md (git-cliff)
├── Cargo.toml
└── rust-toolchain.toml         # Pin stable + nightly for clippy/coverage
```

### First Implementation Phase
Start by:
1. Creating the Cargo project (binary + lib)
2. Setting up Cargo.toml with all required dependencies and features (e.g., "tree-sitter-python", "parallel")
3. Implementing clap CLI with subcommands: check, config, list-languages
4. Basic parallel file walker using ignore + rayon
5. Config loading with default embedded config
6. Tree-sitter integration for Python and JavaScript comment extraction
7. Core slop patterns (categorized: placeholder, deferral, hedging, stub) with severity levels
8. Diagnostic reporting with file:line, category, message, and total "Sloppy Score"
9. Beautiful terminal output with colors and summary

### Rules You Must Follow
- Write idiomatic, safe Rust (no unsafe unless justified and documented)
- Zero tolerance for clippy warnings
- All public APIs documented with /// docs
- Every new type/function has unit or integration tests
- No TODOs, no "for now", no hedging in code or comments
- Favor explicitness, performance, and maintainability
- Use workspaces if adding crates later (e.g., antislop-core)

Begin by outputting:
- Full Cargo.toml with all dependencies and features
- rust-toolchain.toml
- Initial src/main.rs with clap structure and --version/--help
- src/config.rs skeleton
- Default config.toml embedded via include_str!
- GitHub Actions CI workflow (test, clippy, fmt, deny)

This project must be ready to ship to crates.io and attract contributors on day one. No compromises.

Start now.

---

@examples/ and @languages folder are empty except @examples/sloppy_code.py - we want more languages: js, ts, rust, java, c++, c#, perl, go, kotlin, fortran, r, php, swift, ruby, lua, haskel, scala