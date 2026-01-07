# Usage

## Basic Usage

```bash
# Scan current directory
antislop

# Scan specific paths
antislop src/ tests/

# Scan single file
antislop examples/sloppy.py
```

## Output Formats

```bash
# Human-readable (default)
antislop src/

# JSON output
antislop --json src/

# SARIF for GitHub Security
antislop --format sarif > results.sarif
```

## Profiles

Antislop follows the Unix philosophy: **minimal defaults**, extensible via profiles.

By default, only **Core** patterns (stubs, critical placeholders) are enabled.
For most projects, we recommend the `antislop-standard` profile.

```bash
# Recommended baseline (Standard profile)
antislop --profile antislop-standard src/

# Strict checking (All patterns)
antislop --profile antislop-strict src/

# List available profiles
antislop --list-profiles
```

## Category Control

Fine-tune your scan by enabling/disabling specific pattern categories:

```bash
# Disable specific categories
antislop --disable hedging src/
antislop --disable hedging,deferral src/

# Only scan specific categories
antislop --only stub src/
antislop --only stub,placeholder src/
```

## Profile Management

```bash
# Load a local profile file
antislop --profile .antislop/profiles/custom.toml src/

# Load a remote profile
antislop --profile https://example.com/profiles/strict.toml src/
```

## Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Path to config file |
| `--profile <NAME>` | Load a community profile (file, URL, or name) |
| `--list-profiles` | List available profiles |
| `--disable <CATS>` | Disable categories (comma-separated) |
| `--only <CATS>` | Only enable categories (comma-separated) |
| `--list-profiles` | List available profiles |
| `--json` | Output in JSON format |
| `--format <FMT>` | Output format: `text`, `json`, `sarif` |
| `-m, --max-size <KB>` | Maximum file size to scan (default: 1024) |
| `-e, --extensions <EXT>` | File extensions to scan (comma-separated) |
| `-v, --verbose` | Verbose output (use -vv, -vvv for more) |
| `--completions <SHELL>` | Generate shell completions |
| `--list-languages` | List supported languages |
| `--print-config` | Print default configuration |

## Examples

### JSON Output

```bash
antislop --json src/ > results.json
```

### SARIF for GitHub Security

```bash
antislop --format sarif > results.sarif
```

### Custom Extensions

```bash
antislop -e .py,.rs,.js src/
```

### Verbose Mode

```bash
antislop -vv src/
```

### Shell Completions

```bash
# For bash
antislop --completions bash > ~/.local/share/bash-completion/completions/antislop

# For zsh
antislop --completions zsh > ~/.zfunc/_antislop

# For fish
antislop --completions fish > ~/.config/fish/completions/antislop.fish
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | No slop detected |
| `1` | Slop found |
| `2` | Error (config, file access, etc.) |

## Integration

### Pre-commit Hook

Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/skew202/antislop
    rev: v0.1.0
    hooks:
      - id: antislop
```

### GitHub Action

```yaml
- uses: skew202/antislop@v1
  with:
    args: src/
```
