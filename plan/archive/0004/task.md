# Add cosmos command: project gha-outputs

## Goal
Implement a `cosmos` subcommand that reads `project.toml` and emits GitHub Actions outputs (key=value) compatible with the current `scripts/parse_project_toml.py`.

## Background
CI currently depends on `scripts/parse_project_toml.py` to produce:
- run_build/run_tests/run_security/run_docs
- quick_gate_precommit
- outputs_contains_docker
- docker_image
- project_version
(and optionally project_name / project_type)

This should be owned by `cosmos` to avoid split-brain logic.

## Scope
- Add a new command (name TBD):
  - `cosmos project gha-outputs` (preferred)
  - or `cosmos ci gha-outputs`
- Read `project.toml` using Rust (toml deserialize or dynamic lookup).
- Write outputs to `$GITHUB_OUTPUT` when present, otherwise print to stdout (same behavior as the python script).
- Match output keys and boolean formatting ("true"/"false").

## Acceptance criteria
- Running the command locally prints the expected key/value pairs.
- When `GITHUB_OUTPUT` is set, it appends outputs correctly.
- Default values match current python behavior.
- A short help text documents purpose and the output keys.

## Tests
- Add an integration test that:
  - creates a temp repo root with a minimal `project.toml`
  - runs `cosmos project gha-outputs` with `GITHUB_OUTPUT` set
  - asserts the file contains expected keys and values.