Custom CI hooks

This directory allows repository owners to insert custom CI logic into a fixed pipeline.

## How hooks are discovered

The main workflow `.github/workflows/ci.yml` looks for optional hook scripts under `.github/custom/` using this naming convention:

- `before-<stage>.sh` — runs before the stage's main command
- `after-<stage>.sh` — runs after the stage's main command

Currently supported stages:

- `build` (around `cargo build --release`)
- `test` (around `cargo test`)
- `security` (around `cargo audit`)
- `docs` (around `cargo doc`)

Note: formatting/lint is handled by `pre-commit` (see `.pre-commit-config.yaml`). There is no separate `fmt`/`clippy` stage hook in the CI workflow.

## Requirements

- Hooks must be non-interactive and exit non-zero on failure.
- CI will `chmod +x` the hook before executing it.
- Keep hooks short and deterministic.

## Example

Create `.github/custom/before-test.sh` to run extra checks before tests:

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "Running extra checks before tests"
```

Security note: hooks run in CI and have access to the repository contents and runner environment. Do not hardcode secrets; use GitHub Secrets.
