# Configuration

AntiSlop uses layered configuration: built-in defaults → config file → CLI flags.

## Config File Locations

AntiSlop searches for configuration in this order:

1. `--config <FILE>` if provided
2. `antislop.toml`
3. `.antislop.toml`
4. `.antislop`

## Config File Format

```toml
# File extensions to scan
file_extensions = [".py", ".rs", ".js", ".ts"]

# Maximum file size in KB
max_file_size_kb = 1024

# Paths to exclude (glob patterns)
exclude = [
    "node_modules/**",
    "target/**",
    "venv/**",
]

# Detection patterns
[[patterns]]
regex = "(?i)TODO:"
severity = "medium"
message = "Placeholder comment: untracked TODO found"
category = "placeholder"

[[patterns]]
regex = "(?i)for now"
severity = "low"
message = "Deferral: temporary solution with no plan to revisit"
category = "deferral"
```

## Pattern Options

| Field | Type | Description |
|-------|------|-------------|
| `regex` | string | Regular expression to match (use `(?i)` for case-insensitive) |
| `severity` | string | One of: `low`, `medium`, `high`, `critical` |
| `message` | string | Human-readable description |
| `category` | string | One of: `placeholder`, `deferral`, `hedging`, `stub` |

## Severity Scores

| Severity | Score |
|----------|-------|
| low | 1 |
| medium | 5 |
| high | 15 |
| critical | 50 |

## Community Profiles

Profiles are reusable pattern collections stored in `.antislop/profiles/`.

### Profile Locations

AntiSlop searches for profiles in:
1. `.antislop/profiles/<name>.toml` (project-local)
2. `~/.config/antislop/profiles/<name>.toml` (user)
3. `~/.cache/antislop/profiles/<name>.toml` (cached remote)

### Built-in Profiles

| Profile | Description |
|---------|-------------|
| `antislop-standard` | Language-agnostic base config (recommended) |
| `no-stubs` | Strict anti-stub patterns |
| `strict-comments` | No deferral language allowed |

### Profile Format

```toml
[metadata]
name = "my-profile"
version = "1.0.0"
description = "My custom patterns"
extends = ["antislop-standard"]  # Optional: inherit from other profiles

[[patterns]]
regex = '(?i)my pattern'
severity = "medium"
message = "Description"
category = "deferral"
```

### Using Profiles

```bash
# Load by name
antislop --profile antislop-standard src/

# Load from file
antislop --profile ./my-profile.toml src/

# Load from URL (cached for 24h)
antislop --profile https://example.com/profile.toml src/
```

