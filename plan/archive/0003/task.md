# Define project.toml manifest schema + contract

## Goal
Make `project.toml` an intentional, stable manifest for `cosmos` and CI by introducing a small schema/version contract and clarifying the source-of-truth rules.

## Background
Today `project.toml` exists and is used mainly by CI (via `scripts/parse_project_toml.py`). `cosmos` itself does not parse/manage it yet.

## Scope
- Define a versioned schema surface (minimal v1) for the subset we will actually support.
- Decide and document source-of-truth policy between `Cargo.toml` and `project.toml`.
- Identify which fields are:
  - authoritative in `project.toml`
  - derived from `Cargo.toml`
  - deprecated / informational only

## Proposed schema (v1)
- `[cosmos] schema_version = 1`
- `[project]` name, version, type
- `[ci]` run_build/run_tests/run_security/run_docs, quick_gate
- `[artifact] outputs`
- `[docker]` enabled, image

(Other sections remain reserved but not yet enforced.)

## Acceptance criteria
- `docs/cosmos.md` (or a new `docs/project-toml.md`) documents:
  - schema_version
  - supported fields and defaults
  - source-of-truth rules
- `project.toml` in repo root includes `schema_version`.
- A small compatibility note exists for existing repos that already have `project.toml` without schema_version.

## Tests
- N/A (documentation/schema definition only). A follow-up task will add parser + validation tests.