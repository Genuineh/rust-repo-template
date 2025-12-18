# AI Collaboration

This repository is designed to make AI-assisted development transparent and repeatable.

Key points:

- If you use AI (Copilot / LLM) to generate or assist with code, **disclose it in PRs**: include the prompt and a short evaluation of the output.
- Use the prompt templates in `.github/ai/prompt_templates.md` as starting points to produce reproducible prompts.
- Add tests for AI-generated code and verify determinism (no randomness, network calls, or time-dependent behavior in tests).
- Follow the `CONTRIBUTING.md` guidelines for AI disclosures and reviews.

See `.github/ai/ai-guidelines.md` for more details and recommended policies.
