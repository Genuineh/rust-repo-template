Git hooks for this repository

This repository includes local git hooks under `.githooks/` designed to run automatically for contributors.

Included hooks:
- `pre-commit` — formats code (`cargo fmt --all`), stages formatting changes, and runs `cargo clippy --all-targets --all-features -- -D warnings`. If clippy fails, the commit is blocked.

How to enable (recommended):
1. Run: `./scripts/install-git-hooks.sh`
   - This sets `git config core.hooksPath .githooks` in this repository and marks scripts executable.
2. From then on, local commits will trigger the pre-commit behavior automatically.

Bypassing hooks:
- Use `git commit --no-verify` to skip hooks when necessary.

Notes:
- Hooks run locally only and are not enforced by the server; CI still verifies formatting and clippy.
- If you prefer to manage hooks differently (e.g., copy to `.git/hooks`), feel free to modify the installer.
