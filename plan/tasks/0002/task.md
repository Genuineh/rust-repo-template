# Add plan usage docs to README and AGENT_INSTRUCTIONS

## Goal
Document how to use the repository `plan/` lifecycle (create/review/start/test/accept/finish) in:
- `README.md` (short overview + link)
- `docs/AGENT_INSTRUCTIONS.md` (step-by-step commands and examples)

## Background
This task existed as a legacy single-file entry. It is migrated into the directory-based plan task layout so `cosmos plan` lifecycle commands work consistently.

## Acceptance criteria
- README contains a short Plan overview and a link to the detailed docs.
- `docs/AGENT_INSTRUCTIONS.md` includes step-by-step usage examples.
- CI runs `scripts/validate_plan.py` during PRs (or equivalent workflow is present).

## Tests
- `cosmos plan validate` passes.
- CI workflow that validates plan structure is present and green.
