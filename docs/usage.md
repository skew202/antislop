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

## Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Path to config file |
| `--json` | Output in JSON format |
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
```

## Exit Codes

- `0`: No slop detected
- `1`: Slop found
