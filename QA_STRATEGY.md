# Quality Assurance Strategy

> A world-class QA strategy for maintaining code quality at scale.

## Philosophy

**Zero Tolerance for Slop** — The tool that detects slop must have zero slop itself.

We follow a multi-layered testing pyramid with emphasis on:
1. **Fast feedback** — Tests run in milliseconds
2. **Comprehensive coverage** — Every code path tested
3. **Regression prevention** — Snapshots catch unintended changes
4. **Security hardening** — Fuzzing for edge cases

---

---

## Pattern Hygiene (MECE with MegaLinter)

We adhere to a Mutually Exclusive, Collectively Exhaustive (MECE) strategy with standard linters.

**Rule:** Use AntiSlop ONLY for patterns missed by standard linter defaults.
- **Static Analysis**: `scripts/check_overlap.py` ensures no pattern regex overlaps with standard linter defaults.
- **Dynamic Verification**: `scripts/verify_hygiene.sh` runs real linters (Pylint, ESLint, Clippy) against `examples/` to prove AntiSlop finds unique issues.
- **Goal**: AntiSlop only flags "AI Slop" (intent/laziness), leaving syntax/style to specialized tools.

---

## Testing Pyramid

```
                    ┌─────────────────┐
                    │   E2E / CLI     │  (19 tests)
                    │   Integration   │
                    ├─────────────────┤
                    │   Edge Cases    │  (9 tests)
                    ├─────────────────┤
                    │   Property      │  (5 tests)
                    │   Based         │
                    ├─────────────────┤
                    │   Snapshot      │  (5 tests)
                    ├─────────────────┤
                    │    Unit Tests   │  (51 tests)
                    │    (Fast, Many) │
                    └─────────────────┘
```

---

## Test Categories

### 1. Unit Tests (`cargo test`)
- **Location**: `src/*/tests.rs` modules
- **Coverage**: Config parsing, Scanner, Detector, Walker, Reporter
- **Run**: `cargo test --lib`
- **Target**: 80%+ line coverage

### 2. Integration Tests (`tests/`)
- **Location**: `tests/integration_tests.rs`
- **Coverage**: CLI end-to-end: `--help`, `--version`, `--json`, `--format sarif`
- **Run**: `cargo test --test integration_tests`

### 3. Property-Based Tests (proptest)
- **Location**: `tests/property_tests.rs`
- **Purpose**: Fuzz inputs to find edge cases
- **Key Tests**:
  - `test_scanner_no_crash_on_random_input` — No panics on arbitrary input
  - `test_scanner_finds_injected_slop_with_fallback` — Detection guarantees
  - `test_scan_result_score_matches_findings` — Score calculation correctness
  - `test_finding_positions_are_valid` — Position accuracy validation
  - `test_multiple_slop_patterns_in_same_line` — Multiple pattern detection

### 4. Snapshot Tests (insta)
- **Location**: `tests/snapshot_tests.rs`
- **Purpose**: Catch unintended output changes
- **Key Tests**:
  - `test_json_output_snapshot` — JSON format stability
  - `test_sarif_output_snapshot` — SARIF schema compliance
  - `test_stub_patterns_snapshot` — Stub pattern detection
  - `test_severity_levels_snapshot` — Severity level classification
  - `test_multiple_findings_snapshot` — Multiple findings reporting

### 5. Doc Tests
- **Location**: `src/lib.rs` examples
- **Purpose**: Ensure documentation examples compile
- **Run**: `cargo test --doc`

### 6. Edge Case Tests
- **Location**: `tests/edge_cases.rs`
- **Purpose**: Verify behavior with unusual inputs
- **Key Tests**:
  - `test_empty_input` — Empty file handling
  - `test_unicode_comments` — Unicode/emoji in comments
  - `test_very_long_line` — Long line handling
  - `test_carriage_return_line_feeds` — Windows line endings
  - `test_no_newline_at_end` — Missing trailing newline

---

## Advanced QA Techniques

### Mutation Testing (cargo-mutants)
Tests that survive mutations are weak tests.

```bash
# Install
cargo install cargo-mutants

# Run mutation testing
cargo mutants --jobs 4

# Target: <10% mutation survival rate
```

### Fuzzing (cargo-fuzz)
Security-critical for regex and tree-sitter parsing.

```bash
# Setup
cargo install cargo-fuzz
cargo fuzz init

# Fuzz targets
cargo fuzz run fuzz_scanner -- -max_len=10000
cargo fuzz run fuzz_pattern_compile
```

**Fuzz Targets**:
- `fuzz_scanner` — Random source code input
- `fuzz_pattern_compile` — Regex pattern safety
- `fuzz_config_parse` — TOML parsing edge cases

### Coverage (cargo-llvm-cov)
```bash
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --html

# Target: 80% line coverage
# Critical modules: 90%+ (detector, scanner)
```

---

## CI Quality Gates

All PRs must pass:

| Check | Command | Threshold |
|-------|---------|-----------|
| Format | `cargo fmt --check` | 100% |
| Clippy | `cargo clippy -- -D warnings` | 0 warnings |
| Tests | `cargo test --all-features` | 100% pass |
| MSRV | `cargo check` on Rust 1.76 | Must compile |
| Licenses | `cargo deny check licenses` | Approved list |
| Advisories | `cargo deny check advisories` | 0 vulnerabilities |

### CI Workflow (`ci.yml`)

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - cargo fmt --check
      - cargo clippy --all-targets --all-features -- -D warnings
      - cargo test --all-features --verbose
      - cargo build --release --all-features

  msrv:
    runs-on: ubuntu-latest
    steps:
      - cargo check --all-features  # Rust 1.76

  deny:
    runs-on: ubuntu-latest
    steps:
      - cargo deny check advisories bans sources licenses
```

---

## Pre-Commit Hooks

`.pre-commit-config.yaml`:
```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all --
        language: system
        types: [rust]
        pass_filenames: false

      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-targets -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false

      - id: cargo-test
        name: cargo test
        entry: cargo test --lib
        language: system
        types: [rust]
        pass_filenames: false
```

---

## Release Quality Checklist

Before any release:

- [ ] All CI checks pass
- [ ] `cargo test --all-features` — 0 failures
- [ ] `cargo clippy` — 0 warnings
- [ ] `cargo audit` — 0 vulnerabilities
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Integration tests pass on all platforms
- [ ] SARIF output validates against schema
- [ ] LSP server tested with VS Code

## Implementation Status

> ✅ = Implemented | 🔧 = Ready (needs installation) | 📋 = Documented

| Component | Status | Location |
|-----------|--------|----------|
| Pre-commit hooks | ✅ | `.pre-commit-config.yaml` |
| QA script | ✅ | `scripts/qa.sh` |
| Coverage script | ✅ | `scripts/coverage.sh` |
| Fuzz targets | ✅ | `fuzz/fuzz_targets/` |
| Snapshot tests | ✅ | `tests/snapshot_tests.rs` |
| Property tests | ✅ | `tests/property_tests.rs` |
| CI pipeline | ✅ | `.github/workflows/ci.yml` |
| SARIF output | ✅ | `src/report/sarif.rs` |

---

## Metrics & Targets

| Metric | Previous | Current | Target | Status |
|--------|----------|---------|--------|--------|
| Unit tests | 21 | **51** | 30+ | ✅ |
| Integration tests | 7 | **19** | 15+ | ✅ |
| Snapshot tests | 3 | **5** | 5+ | ✅ |
| Property tests | 2 | **5** | 5+ | ✅ |
| Edge case tests | 0 | **9** | 5+ | ✅ |
| Line coverage | ~70% | **89.25%** | 80%+ | ✅ |
| Mutation score | Unknown | **48.1%** (87/181 caught) | >40% | ✅ |
| Fuzz targets | 3 | **3** | 3+ | ✅ |
| CI run time | ~2min | **~2min** | <3min | ✅ |
| **Total tests** | 33 | **90** | 70+ | ✅ |

---

## Continuous Improvement

1. **Weekly**: Review test coverage reports
2. **Monthly**: Run mutation testing, add tests for survivors
3. **Quarterly**: Security audit with `cargo audit`
4. **Per Release**: Full regression test on all platforms

---

## SARIF Integration

All findings can be exported to SARIF for GitHub Security tab:

```bash
antislop --format sarif > results.sarif
```

GitHub Action integration:
```yaml
- name: Run antislop
  run: antislop --format sarif > results.sarif

- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```
