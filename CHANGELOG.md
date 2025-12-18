# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Added
- New CLI tool `cosmos` for project generation, validation and plan management (generate / validate / plan list / plan validate / ai-eval). 🛠️
- `generate` command (dry-run default, `--apply` to write files) to copy template files locally. 📁
- `validate` command with structural, CI/workflow, plan checks and non-LLM AI heuristics. 🔍
- `plan` subcommands: `list` and `validate` to manage `plan/` lifecycle and detect inconsistencies. 🗂️
- `ai-eval` command with rule-based checks and scaffolded LLM plugin behind feature flag `llm`. 🤖
- Template manifest added: `templates/this_repo.toml`. 📦
- Tests for CLI subcommands and examples under `tests/cli_*.rs`. ✅
- Docs: `docs/cosmos.md` with usage and docs integration. 📚
- CI: `c/` workflow `cosmos-validate.yml` to run `cargo test` and quick `cosmos validate`. 🔁
- Release workflow: `.github/workflows/release.yml` to build artifacts and create GitHub releases on tag push. 🚀

### Changed
- Project README updated with `cosmos` usage examples and installation guidance. ✍️

### Fixed
- Various test and edge-case fixes discovered during implementation and tests. 🐛

---

## Release process
See `docs/release.md` for the recommended release workflow and notes about tagging and publishing to crates.io.
