# CI Hooks

This template provides a fixed pipeline with the following *stages*:

- fmt (rustfmt check)
- clippy (linting)
- build (release build)
- test (unit/integration tests)
- security (cargo-audit)
- docs (cargo doc)

For each stage, the CI supports optional local hooks you can add under `.github/custom/` using this naming convention:

- `before-<stage>.sh` — runs before the stage's main command
- `after-<stage>.sh` — runs after the stage's main command

Examples:
- `.github/custom/before-fmt.sh` runs before `cargo fmt --check`
- `.github/custom/after-test.sh` runs after `cargo test`

Notes & best practices:
- Hooks should be non-interactive and return a non-zero exit code on failure.
- Hooks are only executed if the corresponding file exists; otherwise the CI skips them silently.
- Keep hooks simple and fast; for long-running tasks consider running them in their own workflow.
- The pipeline preserves backward compatibility with legacy single hooks like `fmt.sh` and `clippy.sh`.

See `.github/custom/README.md` for more examples and security notes.
