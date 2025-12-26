# Migrate CI workflows from Python parser to cosmos

## Goal
Update GitHub workflows to use `cosmos project gha-outputs` instead of `python scripts/parse_project_toml.py`, removing the Python dependency for CI gating.

## Scope
- Update `.github/workflows/ci.yml`:
  - replace setup-python + python step with installing/running `cosmos` (via `cargo install --path . --bin cosmos` or `cargo run --bin cosmos -- ...`).
  - ensure the `prepare` job outputs remain identical.
- Update `.github/workflows/release.yml` similarly.
- Decide whether to keep `scripts/parse_project_toml.py`:
  - keep temporarily with deprecation note
  - or remove after migration (preferred once stable)

## Acceptance criteria
- Workflow logic remains functionally equivalent (same jobs enabled/disabled for the same `project.toml`).
- No Python setup is needed solely for parsing `project.toml`.
- Docs reflect the new mechanism.

## Tests
- Local verification: `cosmos project gha-outputs` produces same outputs as python script for the repo's `project.toml`.
- CI should pass on the default template.