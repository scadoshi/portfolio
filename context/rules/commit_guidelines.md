## Commit Guidelines

- Concise, one-line messages (multi-line only when many changes)
- Group related files logically
- No emojis
- Use `git diff` to understand changes before committing
- **Never** include AI-agent signatures in your commits
    - Example: "Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
    - Never commit with attribution metadata

## CI: how your commits get checked (run these BEFORE you push)

Pushing to `main` triggers the GitHub Pages deploy
(`.github/workflows/deploy.yml`, `dx build --release --ssg`). Tests and clippy now
gate the deploy: a red check means the site does not update. Formatting is still
local discipline, kept identical to zwipe's so this crate and the shared
`zwipe-components` (consumed via git dep) hold the same bar.

### 1. Format with **nightly** — the one that bites
`rustfmt.toml` enables `imports_granularity = "Crate"`, an *unstable* option, so
**stable `cargo fmt` silently skips it** — code looks formatted locally but drifts
from zwipe's style. Always:

```bash
cargo +nightly fmt        # NOT `cargo fmt` — stable can't apply the Crate imports rule
```

### 2. Clippy — warnings are errors
```bash
cargo clippy --all-targets -- -D warnings
```
Some code is wasm-only (e.g. `theme_store`), so also lint the browser target:
```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

### 3. Tests
```bash
cargo test
```
Mostly markup, so the bar for a new test is high: real logic (parsing, non-trivial
data transforms) or a guardrail on something hand-maintained that has already
drifted. `sitemap_lists_every_prerendered_route` is the second kind. Don't stand up
a suite for markup.

### Deploy
Push to `main` deploys production (scottyfermo.com via GitHub Pages). Test and
Clippy run before Build, so either one failing stops the deploy and the live site
stays on the old version. `cargo +nightly fmt` is not in the workflow: it needs a
nightly toolchain on every deploy and bad formatting cannot break the site. That
one is on you.
