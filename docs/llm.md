# LLM Integration (experimental)

This repository provides a feature-gated scaffold for integrating an LLM provider into `cosmos`.

## Status
- The `llm` Cargo feature enables the `llm` module (`src/llm.rs`).
- A simple **stub provider** is included for testing and demonstration.

## Using the stub provider

1. Build `cosmos` with the LLM feature enabled:

```bash
cargo build --features llm --bin cosmos
```

2. Configure the stub provider and run evaluation:

```bash
export LLM_PROVIDER=stub
cargo run --features llm --bin cosmos -- ai-eval --mode llm
```

When the stub runs successfully it will create a `.cosmos_llm_report.txt` file in the repository root.

## Implementing a real provider

- Implement provider logic in `src/llm.rs` behind the `llm` feature flag.
- Follow secure practices: **never** hardcode API keys; expect providers to be configured via environment variables or CI secrets.
- Add tests that are gated with `#[cfg(feature = "llm")]` so they only run when the feature is enabled.

