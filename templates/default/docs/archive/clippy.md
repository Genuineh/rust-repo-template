# Clippy configuration

> Archived: this page describes an older configuration approach.
> Current Clippy versions do not accept a `[lints]` table in `clippy.toml`.

This repository supports configuring Clippy via `clippy.toml` at the repository root.

Key usage

- `msrv` — specify the Minimum Supported Rust Version to help certain lints produce correct suggestions.
- `[lints].allow` — list of clippy lint names to `allow` globally for this project. The CI and pre-commit hook will read this and pass the corresponding `-A` flags to `cargo clippy`.

Example `clippy.toml`

```toml
msrv = "1.70.0"

[lints]
allow = [
  "clippy::module_name_repetitions",
  "clippy::needless_pass_by_value",
]
```

How it works

- The `pre-commit` hook and CI's `pre-commit` job dynamically parse `clippy.toml` and append `-A <lint>` flags to `cargo clippy`, so you don't need to remember to pass these flags in CI. This keeps local and CI behavior consistent.

Guidelines

- Prefer limiting the allow-list to a small set of well-justified exceptions. Over-allowing lints reduces Clippy's value.
- When silencing a lint, prefer applying it to a narrow scope with `#[allow(...)]` in code if it's a localized concern.
- Keep `msrv` updated when bumping the project's MSRV to ensure lints behave correctly.
