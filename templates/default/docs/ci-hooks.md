# CI Hooks

This template ships with a fixed CI pipeline and a small “hook” mechanism so you can add extra logic **without** forking the workflows.

## Where hooks run

The main pipeline is defined in `.github/workflows/ci.yml`.

- It can run on:
  - PR approval reviews (`pull_request_review` with `approved`, non-draft)
  - Tag pushes `v*`
  - Manual `workflow_dispatch`
- A separate workflow `.github/workflows/cosmos-validate.yml` runs `cosmos validate` + tests on PRs / pushes to `main`.

Why the approval trigger?

- This template intentionally defers the “heavy” pipeline until review approval, so CI cost stays low on churn-heavy PRs.
- If you prefer conventional behavior (run on `pull_request` / `push`), adjust the `on:` section in `.github/workflows/ci.yml`.

## project.toml switches

The `prepare` job reads `project.toml` (via `cosmos project gha-outputs`) and exposes outputs that control which downstream jobs run.

See also: `project.toml` manifest overview in `docs/project-toml.md`.

Note: the manifest now includes `[cosmos].schema_version` to make the configuration surface explicit.

- Toggle jobs under `[ci]`: `run_build`, `run_tests`, `run_security`, `run_docs`
- Quick gate: `quick_gate = ["pre-commit"]` controls whether the `pre-commit` job must pass before heavier jobs run

## Hook naming and supported stages

Create executable shell scripts under `.github/custom/`:

- `before-build.sh` / `after-build.sh`
- `before-test.sh` / `after-test.sh`
- `before-security.sh` / `after-security.sh`
- `before-docs.sh` / `after-docs.sh`

If a hook file is missing, CI prints a “skipping” message and continues.

Notes:

- Formatting/lint runs via `pre-commit` (`.pre-commit-config.yaml`) in the `pre-commit` job.
- There is currently **no** dedicated hook point around `pre-commit` in the workflow.
- The `security` job runs `cargo audit || true` (advisory by default). Remove `|| true` if you want it to fail the build.

Tip:

- The template ships only `.github/custom/README.md` by default. Add your own `before-*.sh` / `after-*.sh` scripts as needed.

## Hook requirements

- Non-interactive, deterministic.
- Exit non-zero to fail the stage.
- CI will `chmod +x` the hook before running it.

## Example

Create `.github/custom/before-test.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "Running extra checks before tests"
```

See `.github/custom/README.md` for the latest notes.
