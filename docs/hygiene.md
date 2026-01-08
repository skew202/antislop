# Pattern Hygiene (Orthogonal)

We follow a **Mutually Exclusive, Collectively Exhaustive** strategy with standard linters like MegaLinter.

*   **AntiSlop**: Detects AI shortcuts (stubs, hallucinated API usage, hedging).
*   **Standard Linters**: Detect syntax errors, style issues, and bugs.

**Rule**: If `eslint` or `clippy` catches it by default, AntiSlop will **not** cover it (unless explicitly whitelisted).

## Hygiene Survey

AntiSlop includes a hygiene survey feature to audit your project's tooling.

```bash
antislop --hygiene-survey
```

This command checks for the presence of:
-   **Linters**: ESLint, Ruff, Clippy, Checkstyle, etc.
-   **Formatters**: Prettier, Black, Rustfmt, Gofmt, etc.
-   **CI/CD**: GitHub Actions, GitLab CI, etc.
-   **Pre-commit Hooks**: `pre-commit`, Husky.

It provides a report on what is missing for your detected project languages.
