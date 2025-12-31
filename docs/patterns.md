# Patterns

Antislop detects four categories of slop patterns.

## Placeholder

Untracked placeholders that indicate incomplete work:

- `TODO:` - Untracked TODO comment
- `FIXME:` - Indicates incomplete work
- `HACK:` - Poor solution that needs revisiting
- `NOTE: important` - Unnecessary NOTE comments
- `XXX` - Urgent problems

## Deferral

Language indicating temporary solutions:

- `for now` - Temporary solution with no plan to revisit
- `temp`, `temporary` - Temporary code that becomes permanent
- `quick implement` - Euphemism for incomplete
- `simplif` - Often skips edge cases
- `shortcut` - Creates technical debt

## Hedging

Uncertainty language in comments:

- `hopefully` - Uncertainty about code behavior
- `should work` - Unsure if code actually works
- `approximately`, `roughly` - Code should be precise
- `this is a simple` - Often means missing edge cases
- `basic implement` - Incomplete implementation warning
- `in a real world` - Code that would be different in production

## Stub

Empty or placeholder implementations:

- `placeholder`, `stub` - Instead of actual implementation
- `not implemented`, `unimplemented` - Explicitly unimplemented code

## Adding Custom Patterns

Add to your `antislop.toml`:

```toml
[[patterns]]
regex = "(?i)your-pattern-here"
severity = "medium"
message = "Your custom message"
category = "placeholder"
```
