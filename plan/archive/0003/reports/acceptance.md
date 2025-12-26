# Acceptance report — Task 0003

## Summary
Introduced a versioned schema header for `project.toml` and added documentation for the manifest.

## Changes
- `project.toml`: added `[cosmos].schema_version = 1`.
- Template `templates/default/project.toml`: aligned to the same manifest shape (with `[project]`, `[ci]`, `[artifact]`, etc.) and added schema_version.
- Docs: added `docs/project-toml.md` and wired it into `mkdocs.yml` (also mirrored in `templates/default/`).

## Verification
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --workspace`

All checks passed locally.
