# Contributing

Thanks for considering contributing! This repository includes a template for how to accept contributions.

- Please use the provided Issue Forms for bug reports and feature requests.
- Before opening a PR, run:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

- If you used AI to generate or assist code, disclose it in the PR and include the prompt and a short evaluation.

For more details, see the root `CONTRIBUTING.md` and `.github/ai/ai-guidelines.md`.
