# Acceptance report — Task 0004

## Summary
Added a new `cosmos project gha-outputs` subcommand that parses `project.toml` and emits GitHub Actions outputs compatible with the existing `scripts/parse_project_toml.py` behavior.

## Verification
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --workspace`

## Evidence
- New integration test `tests/cli_project_gha_outputs.rs` verifies output keys/values via `$GITHUB_OUTPUT`.
