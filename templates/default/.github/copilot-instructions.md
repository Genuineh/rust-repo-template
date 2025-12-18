# Copilot & AI Instructions (template)

This file provides guidance for Copilot or other AI assistants when contributing to repositories generated from this template.

## Build & Test
- Build: `cargo build --workspace`
- Test: `cargo test --workspace`
- Format check: `cargo fmt -- --check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`

## What to include in changes
- Add or update unit/integration tests for bug fixes or new features.
- Keep changes small and well-scoped; prefer multiple small PRs over a single large change.
- Update `README.md`, `docs/`, or `examples/` when behaviour or public APIs change.

## Tasks suited for Copilot
- Small bug fixes with clear reproduction steps
- Improving or adding tests
- Documentation updates and examples
- Low-risk refactors limited to a single module

## Tasks not suited for Copilot
- Large, cross-cutting refactors or architectural redesigns
- Security-sensitive changes, authentication, or PII handling
- Incident response or production-critical fixes without human oversight
- Ambiguous or open-ended tasks lacking acceptance criteria

## Environment & CI
- Ensure changes pass the repository CI and do not introduce Clippy warnings.
- Use the provided `copilot-setup-steps.yml` to preinstall build/test dependencies in Copilot's ephemeral environment (if applicable).

## Contact & Review
- If the change affects broader design or may break compatibility, open an issue or request human review before merging.
- When iterating on a PR, maintainers can mention `@copilot` in review comments to request updates.

- Be conservative with code changes; prefer to propose code in small, reviewable commits.
- When using AI to generate code, include the prompt and a short self-check in PR description.
- Include tests for generated or changed behavior.
- Use `copilot-setup-steps.yml` for ephemeral environment setup if available.

Example:

```
AI-Assist: prompt used here
Check: unit tests or example run
```