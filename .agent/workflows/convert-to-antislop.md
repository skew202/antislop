---
description: Convert any agent instruction files (CLAUDE.md, cursor rules, AGENTS.md, codex.toml, etc.) to antislop profiles
---

# Convert Agent Instructions to Antislop Profiles

This skill teaches you how to find and convert AI coding agent instruction files into antislop detection profiles.

## What to Search For

Search the user's project for agent instruction files. These contain coding guidelines that can be converted to antislop patterns.

### File Patterns to Look For

1. **Claude/Anthropic Files**
   - `CLAUDE.md`
   - `.claude/*`
   - `claude.md`

2. **Cursor AI Files**
   - `.cursorrules`
   - `.cursor/*`
   - `cursor.md`

3. **Antigravity/Gemini Files**
   - `.agent/workflows/*.md`
   - `.gemini/*`

4. **OpenAI Codex/ChatGPT Files**
   - `CODEX.md`
   - `codex.toml`
   - `.openai/*`

5. **Generic Agent Files**
   - `AGENTS.md`
   - `CONTRIBUTING.md` (often has coding standards)
   - `.github/AGENTS.md`
   - `STYLE_GUIDE.md`

### Search Commands

```bash
# Find all potential agent instruction files
find . -type f \( \
  -name "CLAUDE.md" -o \
  -name ".cursorrules" -o \
  -name "AGENTS.md" -o \
  -name "CODEX.md" -o \
  -name "codex.toml" -o \
  -name "STYLE_GUIDE.md" -o \
  -name "cursor.md" -o \
  -name "claude.md" \
\) 2>/dev/null

# Also check directories
ls -la .cursor/ .claude/ .agent/ .gemini/ .openai/ 2>/dev/null
```

## How to Extract Patterns

### Step 1: Identify Rule Patterns

Look for these keywords in the instruction files:

| Keyword | Meaning | Antislop Severity |
|---------|---------|-------------------|
| **MUST** | Required | high |
| **NEVER** | Forbidden | critical |
| **ALWAYS** | Required | high |
| **DO NOT** | Forbidden | high |
| **AVOID** | Discouraged | medium |
| **PREFER** | Suggested | low |

### Step 2: Map to Detectable Patterns

For each rule, determine if it can be detected via regex/AST:

#### NEVER/DO NOT Rules (Easier to Detect)

| Rule Text | Regex Pattern |
|-----------|---------------|
| "NEVER use `.unwrap()`" | `\.unwrap\(\)` |
| "NEVER use `unsafe`" | `unsafe\s*\{` |
| "NEVER use emoji" | `[✓✗✅❌🔥💀]` |
| "NEVER use `println!` in production" | `println!\(` |
| "NEVER use `var`" | `\bvar\s+` |
| "DO NOT use `jQuery`" | `(?i)\bjquery\b` |
| "NEVER hardcode secrets" | `(?:password\|secret\|api_key)\s*=\s*["'][^"']+["']` |
| "NEVER use wildcard imports" | `use\s+\w+::\*` |
| "NEVER use `Any` type" | `:\s*Any\b` |
| "DO NOT commit TODO comments" | `(?i)TODO:` |

#### MUST/ALWAYS Rules (Harder - Often Skip)

These require detecting *absence* of something, which is harder:
- "MUST have doc comments" - requires AST analysis
- "MUST use Result\<T, E\>" - requires type analysis
- "ALWAYS handle errors" - complex semantic analysis

**Strategy**: Focus on NEVER/DO NOT rules first.

### Step 3: Generate TOML Profile

Create the profile at `.antislop/profiles/<name>.toml`:

```toml
[metadata]
name = "my-guidelines"
version = "1.0.0"
description = "Auto-generated from CLAUDE.md"
author = "agent"

# Each detected pattern becomes a [[patterns]] entry
[[patterns]]
regex = '\.unwrap\(\)'
severity = "critical"
message = "Violation: NEVER use .unwrap() in library code"
category = "stub"

[[patterns]]
regex = 'unsafe\s*\{'
severity = "critical"
message = "Violation: NEVER use unsafe without documentation"
category = "stub"

[[patterns]]
regex = '[✓✗✅❌]'
severity = "high"
message = "Violation: NEVER use emoji in code"
category = "hedging"
```

### TOML Escaping Rules

When generating TOML:
1. **Use single-quoted strings** for regex: `regex = '\.unwrap\(\)'`
2. Single-quoted strings don't process backslash escapes
3. If regex contains single quote, use double quotes and escape: `regex = "can\\'t"`

## Conversion Workflow

1. **Search** for agent instruction files in the project
2. **Read** each file and extract MUST/NEVER/ALWAYS rules
3. **Map** rules to known regex patterns (see table above)
4. **Generate** `.antislop/profiles/<name>.toml`
5. **Validate** by running: `antislop --profile <name> .`

## Example Conversion

### Input: Excerpt from CLAUDE.md

```markdown
## Error Handling

- **NEVER** use `.unwrap()` in production code paths
- **MUST** use `Result<T, E>` for fallible operations
- **NEVER** use `panic!` except in tests

## Security

- **NEVER** store secrets in code
- **ALWAYS** use environment variables for sensitive config
```

### Output: Generated Profile

```toml
[metadata]
name = "rust-guidelines"
version = "1.0.0"
description = "Generated from CLAUDE.md"

[[patterns]]
regex = '\.unwrap\(\)'
severity = "critical"
message = "Violation: NEVER use .unwrap() in production code"
category = "stub"

[[patterns]]
regex = 'panic!\('
severity = "critical"
message = "Violation: NEVER use panic! except in tests"
category = "stub"

[[patterns]]
regex = '(?:password|secret|api_key)\s*=\s*["\'][^"\']+["\']'
severity = "critical"
message = "Violation: NEVER store secrets in code"
category = "stub"
```

## Verification

After creating the profile, verify it works:

```bash
# List available profiles
antislop --list-profiles

# Test the profile
antislop --profile <name> src/

# Run with verbose logging
antislop -v --profile <name> .
```

## Pattern Reference

### Rust Patterns

| Violation | Regex |
|-----------|-------|
| .unwrap() | `\.unwrap\(\)` |
| .expect() | `\.expect\(` |
| panic!() | `panic!\(` |
| todo!() | `todo!\(` |
| unimplemented!() | `unimplemented!\(` |
| unsafe block | `unsafe\s*\{` |
| println! | `println!\(` |
| dbg! | `dbg!\(` |

### Python Patterns

| Violation | Regex |
|-----------|-------|
| bare except | `except:` |
| mutable default | `def\s+\w+\([^)]*=\s*\[\]` |
| Any type | `:\s*Any\b` |
| pass placeholder | `pass\s*#.*TODO` |
| NotImplementedError | `raise NotImplementedError` |

### JavaScript/TypeScript Patterns

| Violation | Regex |
|-----------|-------|
| var keyword | `\bvar\s+` |
| console.log | `console\.log\(` |
| == instead of === | `[^=!]==[^=]` |
| any type | `:\s*any\b` |

### Universal Patterns

| Violation | Regex |
|-----------|-------|
| Hardcoded secret | `(?:password\|secret\|api_key)\s*=\s*["']` |
| Emoji | `[✓✗✅❌🎉💀🔥]` |
| TODO comment | `(?i)TODO:` |
| FIXME comment | `(?i)FIXME:` |
| XXX marker | `(?i)XXX` |
