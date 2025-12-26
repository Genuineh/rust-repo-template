# Acceptance report — Task 0006

## Summary
Implemented `cosmos project validate`, which performs consistency and drift checks between `project.toml` and `Cargo.toml` and validates artifact-related settings (docker, binary outputs).

Key behavior:
- By default the command is blocking: it exits nonzero (code 2) on errors to allow CI to fail fast.
- Template-safe: drift checks (name/version) are skipped when `project.toml` contains template placeholders like `{{...}}` so template repos don't get blocked.
- `docker.enabled=true` requires `artifact.outputs` to contain `docker` and `docker.image` must be non-empty.
- `artifact.outputs` including `binary` requires at least one binary target (either `src/main.rs`, `[[bin]]` in Cargo.toml, or `[build].bins`).

## Verification
- Added unit tests for validation logic (see `src/bin/cosmos.rs` tests).
- Added integration tests `tests/cli_project_validate.rs` covering:
  - successful validation for concrete manifests
  - blocking on version drift
  - blocking when docker enabled but outputs missing
  - skipping drift checks when `project.toml` uses template placeholders
- Ran: `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --workspace` — all passing.

## Evidence
- `src/bin/cosmos.rs`: new `ProjectCmd::Validate` handling and validation logic (`collect_project_validation_issues`, `validate_project_manifest`).
- `tests/cli_project_validate.rs`: integration tests for the new command.
- `cosmos validate` now invokes project manifest checks when `project.toml` exists (so repo-wide validation is blocking in CI).

Notes:
- `scripts/parse_project_toml.py` remains in the repo for compatibility; it can be removed later once downstream consumers have migrated.
