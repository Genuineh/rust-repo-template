# Acceptance report — Task 0002

## Summary
Documented Plan usage in `README.md` (short overview + link) and `docs/AGENT_INSTRUCTIONS.md` (detailed step-by-step examples and agent guidance). Ensured CI validates plan structure via existing `.github/workflows/validate-plan.yml` which runs `scripts/validate_plan.py` on PRs affecting `plan/**` or `scripts/**`.

## Verification
- `cosmos plan validate` passes locally.
- The repository includes `docs/AGENT_INSTRUCTIONS.md` with explicit command examples and recommended workflow.
- `README.md` references plan docs and guidance.

## Evidence
- `plan/README.md` contains CLI examples and lifecycle descriptions.
- `docs/AGENT_INSTRUCTIONS.md` includes step-by-step guidance and quick commands.
- `.github/workflows/validate-plan.yml` present and configured to run `scripts/validate_plan.py` on PRs.

## Notes
- No code changes were required; this task focused on documentation and CI integration.
