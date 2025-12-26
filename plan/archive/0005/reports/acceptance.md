# Acceptance report — Task 0005

## Summary
Migrated GitHub Actions workflows to parse `project.toml` via `cosmos project gha-outputs` instead of the Python script.

## Changes
- `.github/workflows/ci.yml`: prepare job now installs Rust and runs `cargo run --bin cosmos -- project gha-outputs`.
- `.github/workflows/release.yml`: parsing now uses the same `cosmos` command.
- Mirrored the same changes into `templates/default/.github/workflows/*`.
- Updated CI hooks docs to reference `cosmos project gha-outputs`.

## Verification
- `cargo test --workspace` (includes template sync tests)

Notes:
- `scripts/parse_project_toml.py` is still present for now; it can be removed in a later cleanup once downstream users have migrated.
