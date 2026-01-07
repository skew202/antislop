---
description: Report on code hygiene tool setup (linters, formatters, static analysis)
---

# Hygiene Check - Static Analysis Tool Survey

Survey the project's code hygiene setup and identify gaps where antislop provides unique value.

## What to Check

### 1. Linters & Formatters

| Tool | Config Files to Check |
|------|----------------------|
| **ESLint** | `.eslintrc`, `.eslintrc.js`, `.eslintrc.json`, `eslint.config.js` |
| **Prettier** | `.prettierrc`, `prettier.config.js` |
| **Clippy** | `.cargo/config.toml`, `Cargo.toml` |
| **Pylint/Flake8** | `.pylintrc`, `.flake8`, `pyproject.toml` |
| **Ruff** | `ruff.toml`, `pyproject.toml` |
| **RuboCop** | `.rubocop.yml` |
| **golangci-lint** | `.golangci.yml` |

### 2. Static Analysis Tools

| Tool | Config Files |
|------|--------------|
| **SonarQube/SonarCloud** | `sonar-project.properties`, `sonar.properties` |
| **CodeClimate** | `.codeclimate.yml` |
| **Semgrep** | `.semgrep.yml`, `.semgrep/` |
| **Snyk** | `.snyk` |
| **Dependabot** | `.github/dependabot.yml` |

### 3. CI/CD Checks

Check for:
- `.github/workflows/*.yml`
- `.gitlab-ci.yml`
- `Jenkinsfile`
- `.circleci/config.yml`

### 4. Pre-commit Hooks

| Tool | Config |
|------|--------|
| **pre-commit** | `.pre-commit-config.yaml` |
| **husky** | `.husky/` |
| **lefthook** | `lefthook.yml` |

## Antislop's Unique Value (MECE)

Report what antislop catches that these tools DON'T:

| Category | What Antislop Catches | Standard Tools Miss |
|----------|----------------------|---------------------|
| **Stubs** | `todo!()`, `pass`, `NotImplementedError` | Some catch syntax, not intent |
| **Deferrals** | "for now", "temporary", "quick hack" | Never caught |
| **Hedging** | "hopefully", "should work", "seems to" | Never caught |
| **AI Artifacts** | overconfident comments, numbered suffixes | Never caught |


## Report Template

After surveying, generate a report:

```markdown
# Code Hygiene Survey

## Tools Found
- [ ] Linter: <name>
- [ ] Formatter: <name>
- [ ] Static analysis: <name>
- [ ] CI: <platform>
- [ ] Pre-commit: <name>

## Antislop Unique Value
Based on your setup, antislop will catch:
- ✅ Stub implementations (`todo!()`, `pass`)
- ✅ Deferral language ("for now", "temporary")
- ✅ Hedging in comments ("hopefully works")
- ✅ AI-generated shortcuts

## Recommendations
1. Add antislop to pre-commit hooks
2. Add antislop to CI pipeline
3. Consider SonarQube for dead code analysis
```

## Commands to Verify Setup

```bash
# Check if antislop is in pre-commit
grep -r "antislop" .pre-commit-config.yaml 2>/dev/null

# Check CI workflows for antislop
grep -r "antislop" .github/workflows/ 2>/dev/null

# Check for SonarQube config
cat sonar-project.properties 2>/dev/null

# Run antislop hygiene check
antislop --format sarif . | jq '.runs[0].results | length'
```