# {{project-name}}

> Rust repo template optimized for fast starts and smooth AI collaboration.


## Features ✅

- Rust 2021 edition
- CI: fmt, clippy, test
- Issue / PR templates and contributor guide
- AI collaboration helpers: prompt templates, scripts, guidelines
- Devcontainer / Codespaces ready


## Quick start 🚀

1. Customize `Cargo.toml`: `name`, `version`, `description`, `repository` and `license`.
2. Update `README.md` and `CONTRIBUTING.md` for your project specifics.
3. Local development:

```bash
cargo build
cargo test
```


## AI collaboration 💡

- If AI (Copilot/LLM) was used to generate or assist with code, disclose it in PRs and include the prompt and a short evaluation.
- We provide Issue Forms that capture whether AI was used and a place for the prompt; please fill those out when relevant.

> Note: Detailed agent operation guides were removed from the template docs to keep the template minimal; maintainers can add project-specific guidance to `docs/` if needed.


## Project decisions & notes 🧾

- `CHANGELOG.md` is included for releases. Please keep it up-to-date in the `Unreleased` section.
- `Cargo.lock`: keeping `Cargo.lock` in the template helps reproducible builds for example projects; for libraries you may prefer to remove it after creating a repo.


## Development guidelines 🔧

- Run formatting and lint checks before committing:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

- See `docs/` for more details on CI hooks, release process and AI collaboration.


## Examples

See `examples/hello.rs` and `tests/` for sample usage and test patterns.


## Template placeholders

- This README is a template; replace `{{project-name}}` and other placeholders in files under `templates/default/` when generating a project.


---

Generated from the `cosmos` `default` template.