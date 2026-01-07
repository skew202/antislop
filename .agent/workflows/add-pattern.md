---
description: Add a new antislop pattern from code that wasn't flagged
---

# Add Pattern from Unflagged Code

When you see code that antislop should have flagged but didn't, use this skill to create a new pattern.

## Input Format

The user will provide code like:

```
/add-pattern

```python
# this is fine for now
result = quick_hack()
```
```

Or simply paste the problematic code after invoking the skill.

## Your Task

1. **Analyze the code** to identify what makes it "sloppy"
2. **Create a regex pattern** that detects this
3. **Determine the category**: placeholder, stub, deferral, hedging, noise, or naming_convention
4. **Set appropriate severity**: critical, high, medium, or low
5. **Add to the user's profile** at `.antislop/profiles/custom.toml`

## Pattern Categories

| Category | Description | Examples |
|----------|-------------|----------|
| `placeholder` | TODO/FIXME markers | `TODO:`, `FIXME:`, `XXX` |
| `stub` | Incomplete implementations | `pass`, `todo!()`, `NotImplementedError` |
| `deferral` | Temporary code language | "for now", "temporary", "quick hack" |
| `hedging` | Uncertainty in comments | "hopefully", "should work", "seems to" |
| `noise` | Redundant comments | "// increment i", self-describing comments |
| `naming_convention` | AI-generated names | numbered suffixes, `_v2`, `_backup` |

## Severity Guidelines

| Severity | When to Use |
|----------|-------------|
| `critical` | Definitely broken: stubs, unimplemented, empty functions |
| `high` | Likely broken: explicit TODOs, urgent markers |
| `medium` | Code smell: hedging, deferrals, WIP markers |
| `low` | Style issue: noise comments, minor concerns |

## Regex Construction

### Step 1: Identify the Pattern

From the user's code, extract the key phrase:
- `"this is fine for now"` → key phrase is `for now`
- `"quick_hack()"` → key phrase is `quick.*hack` or `hack`
- `"# probably works"` → key phrase is `probably`

### Step 2: Make it Case-Insensitive

Always use `(?i)` prefix for text patterns:
```
(?i)for now
(?i)quick.*hack
(?i)probably
```

### Step 3: Add Word Boundaries if Needed

Use `\b` to avoid false positives:
```
(?i)\bhack\b
(?i)\bprobably\b
```

### Step 4: Handle Special Characters

Escape these: `. * + ? [ ] ( ) { } ^ $ | \`

| Original | Escaped |
|----------|---------|
| `.unwrap()` | `\.unwrap\(\)` |
| `func()` | `func\(\)` |
| `[TODO]` | `\[TODO\]` |

## Output Format

Add to `.antislop/profiles/custom.toml`:

```toml
[[patterns]]
regex = '(?i)your pattern here'
severity = "medium"
message = "Description of what's wrong"
category = "deferral"
```

## Examples

### Example 1: User pastes hedging comment

**Input:**
```python
# this should probably work in most cases
```

**Your response:**
```toml
[[patterns]]
regex = '(?i)probably work'
severity = "medium"
message = "Hedging: 'probably work' indicates uncertainty"
category = "hedging"
```

### Example 2: User pastes stub function

**Input:**
```typescript
function processData(data: any) {
    // TODO: implement later
    return null;
}
```

**Your response:**
```toml
[[patterns]]
regex = '(?i)implement later'
severity = "high"
message = "Stub: 'implement later' is a deferred TODO"
category = "stub"
```

### Example 3: User pastes temporary code marker

**Input:**
```rust
// HACK: this is a workaround until we fix the real issue
```

**Your response:**
```toml
[[patterns]]
regex = '(?i)workaround until'
severity = "high"
message = "Deferral: temporary workaround with no timeline"
category = "deferral"
```

## Action Steps

1. Parse the user's code sample
2. Identify the sloppy pattern(s)
3. Create regex for each pattern
4. Determine category and severity
5. Check if `.antislop/profiles/custom.toml` exists:
   - If not, create it with metadata
   - If yes, append new pattern
6. Verify the pattern works: `antislop --profile custom <file>`
7. Report what was added

## Creating the Custom Profile

If `.antislop/profiles/custom.toml` doesn't exist:

```toml
# Custom Patterns
#
# User-defined patterns added via /add-pattern skill

[metadata]
name = "custom"
version = "1.0.0"
description = "Custom patterns added from code samples"
author = "user"

# Patterns below are auto-generated
```

## Verification

After adding patterns, verify:
```bash
# Test the new pattern
antislop --profile custom path/to/file

# Show available profiles
antislop --list-profiles
```

## Integration with Other Profiles

Users can combine custom patterns with other profiles using extends:

```toml
[metadata]
name = "my-project"
extends = ["no-stubs", "custom"]
```
