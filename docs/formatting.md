# Formatting Rules

This section documents the project's formatting conventions and how to apply them.

Key settings

- rustfmt config: `rustfmt.toml` at the repository root. Important entries:
  - `max_width = 100`
  - `edition = "2021"`
  - `tab_spaces = 4` and `indent_size = 4`
  - `merge_imports = true`, `reorder_imports = true`

Editor integration

- Recommended editors/IDE:
  - VS Code with `rust-analyzer` (see `.vscode/settings.json`) — enables `Format on Save` and runs `clippy` on save if configured.
  - Ensure your editor uses `rustfmt` as the default Rust formatter.

Local hooks & pre-commit

- We provide local hooks and a `pre-commit` config that includes `cargo fmt` and `cargo clippy`. To get consistent behavior locally:
  1. Install rustfmt and clippy: `rustup component add rustfmt clippy`
  2. (Optional) Install pre-commit: `pip install --user pre-commit`
  3. Run the hook installer: `./scripts/install-git-hooks.sh` (this will try to install pre-commit hooks)

CI behavior

- The CI `pre-commit` job runs `pre-commit run --all-files` as a quick gate to ensure formatting and linting are satisfied before heavier jobs run.

Tips

- If `pre-commit` is not available locally, the included `.githooks/pre-commit` will run a fallback: `cargo fmt --all` (stages changes) and `cargo clippy` (blocks commit on failure).
- Keep `rustfmt.toml` small and project-wide to avoid per-file inconsistencies.
