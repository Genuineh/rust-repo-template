# Changelog

All notable changes to this project will be documented in this file.

## 0.2.0 - 2025-12-18

### Added
- Embed `templates/default/` into the `cosmos` binary so installed users can generate the default template without a local template repo. ✅
- `--verify` for `cosmos generate` that runs `cargo fmt --check`, `cargo clippy` and `cargo test` on generated projects. 🔧
- Feature-gated LLM scaffold with a testable `stub` provider and docs (`docs/llm.md`). 🤖
- `tests/template_sync.rs` to detect `.github` template drift. 📋

### Changed
- Skip Handlebars rendering for workflows and files containing GitHub `${{` expressions to avoid parsing errors. 🛡️
- Template and docs improvements; tests and CI coverage enhancements.

### Fixed
- Various test and formatting fixes discovered during implementation and tests. 🐛

---

## Unreleased

(ongoing work)

## Release process
See `docs/release.md` for the recommended release workflow and notes about tagging and publishing to crates.io.
