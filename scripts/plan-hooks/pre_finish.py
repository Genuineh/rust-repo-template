#!/usr/bin/env python3
"""Plan hook (pre-finish) — example implementation following the unified ctx->res interface.

Contract:
- implement `def run(ctx: dict) -> dict` returning a dict like {'ok': True, 'message': '...'}.
- when invoked as a script it reads a JSON `ctx` from stdin and prints JSON `res` to stdout.
- exit code 0 for success, non-zero for failure (when res['ok'] is False).

Available legacy environment vars (fallback only):
 - PLAN_TASK_ID
 - PLAN_REPO_ROOT
 - PLAN_CURRENT_STATUS
 - PLAN_TASK_FILE

This example checks for an acceptance report at `plan/tasks/<id>/reports/acceptance.md`.
"""
import json
import os
import sys
from typing import Dict


def run(ctx: Dict) -> Dict:
    """Check that an acceptance report exists for the given task.

    ctx keys (recommended):
      - task_id: str
      - repo_root: str
      - task_file: str (optional)
    """
    task_id = ctx.get("task_id") or os.environ.get("PLAN_TASK_ID")
    repo = ctx.get("repo_root") or os.environ.get("PLAN_REPO_ROOT") or "."

    if not task_id:
        return {"ok": False, "message": "missing task_id in ctx (or PLAN_TASK_ID)"}

    report = os.path.join(repo, "plan", "tasks", task_id, "reports", "acceptance.md")
    if not os.path.exists(report):
        return {"ok": False, "message": f"acceptance report not found at {report}"}

    return {"ok": True, "message": "acceptance report found"}


if __name__ == "__main__":
    try:
        raw = sys.stdin.read()
        ctx = json.loads(raw) if raw and raw.strip() else {}
    except Exception:
        # if stdin is not valid JSON, fall back to empty ctx and rely on env vars
        ctx = {}

    res = run(ctx)
    # Print structured result for callers to parse
    print(json.dumps(res))
    if not res.get("ok"):
        # non-zero exit to indicate failure to the caller (cosmos will show stdout/stderr)
        sys.exit(1)
    sys.exit(0)
