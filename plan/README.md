# Plan / Task Lifecycle 📋

This directory contains a small, audit-friendly task plan used to coordinate work in this repository. It follows a per-task directory layout and records a history of lifecycle events for each task.

## Structure 🔧
- `todo.toml` — current task list (summary metadata)
- `plan/tasks/<id>/task.md` — the canonical task description and current work-in-progress
- `plan/tasks/<id>/history/` — chronological history entries (one file per event)
- `plan/archive/<id>/` — tasks moved here when **finished** for long-term storage
- `plan/next_id.txt` — next numeric task id

## Task model
- kind: `bug` or `feature` (use `--kind` when creating tasks)
- status: one of the lifecycle states listed below

## Statuses & recommended transitions ⚙️
The plan supports the following task statuses (recommended flow):

1. `pending_review` — task has been proposed and awaits triage/review
2. `queued` — task accepted and waiting to be worked on (scheduling)
3. `working` — active development in progress
4. `testing` — QA / automated tests running against the change
5. `under_acceptance` — acceptance verification (manual or automated checks)
6. `finished` — work complete; task directory is moved into `plan/archive/<id>/`

Recommended simple flow: `pending_review` → `queued` → `working` → `testing` → `under_acceptance` → `finished`.

> Note: The repository provides a `plan validate` command that enforces allowed statuses and kinds.

## Recording progress (history)
- Use `plan log --id <id> --message "..." [--author <name>]` to append a timestamped history entry to `plan/tasks/<id>/history/` (or archive history if already archived).
- Each history entry is a small markdown file containing time/author metadata and a message — use these to record reviews, CI results, test notes, or decisions.

## CLI operations (examples) 🧭
- Create a task:
  - `cosmos plan create --kind feature --title "Add widget" --content "Details..." --assignee alice`
  - Creates `plan/tasks/000X/task.md`, initializes `history/`, and adds the task to `todo.toml` with `status = "pending_review"`.

- Update a task (metadata and content only; status changes must use explicit lifecycle commands):
  - `cosmos plan update --id 000X --assignee bob`
  - `cosmos plan update --id 000X --content "New details..."`
  - Use lifecycle commands to transition status and record decisions:
    - `cosmos plan review --id 000X --decision accept` (pending_review -> queued)
    - `cosmos plan start --id 000X` (queued -> working)
    - `cosmos plan test --id 000X` (working -> testing)
    - `cosmos plan accept --id 000X` (testing -> under_acceptance)
    - `cosmos plan finish --id 000X` (under_acceptance -> finished; moves task dir to `plan/archive/`)
    - `cosmos plan reopen --id 000X` (finished -> pending_review; moves task back to `plan/tasks/`)

  Transition commands append a history entry (optionally include `--message` and `--author`) and validate the expected current state before applying the change.

## Migration & troubleshooting notes ⚠️

- If your workflow previously used `cosmos plan update --status <status>`, switch to the new lifecycle commands above.
- When a lifecycle command fails, the CLI prints a helpful hint (for example: "Hint: task must be 'queued' to start") to guide the next step — use this hint to recover instead of forcing direct edits.
- Use `cosmos plan log --id <id> --message "..." --author alice` to add ad-hoc history entries when you don't need a status change.
- If a task becomes stuck (no allowed transitions apply), inspect `plan/todo.toml` and the latest `plan/tasks/<id>/history/` entry to determine the last recorded event; apply the correct lifecycle command or add a history entry explaining manual remediation.

### Examples ✅

- Accept a task during review with a message:
  - `cosmos plan review --id 0001 --decision accept --message "LGTM" --author alice`

- Start work and record author note:
  - `cosmos plan start --id 0001 --message "Working on initial PR" --author bob`

- Move to testing and acceptance:
  - `cosmos plan test --id 0001`
  - `cosmos plan accept --id 0001`

- Finish and archive:
  - `cosmos plan finish --id 0001 --message "All checks passed" --author ci-bot`

- Reopen a finished task for follow-up:
  - `cosmos plan reopen --id 0001 --message "Follow-up needed" --author alice`

These examples show recommended, non-blocking flows and how to record context for future reviewers.

## Hooks (extensible checks & automation) 🪝

Each lifecycle transition runs default validation checks and then executes any user-provided Python hooks found under `scripts/plan-hooks/`.

Hook naming convention:
- `pre_review_accept`, `post_review_accept`, `post_review_reject`
- `pre_start`, `post_start`
- `pre_test`, `post_test`
- `pre_accept`, `post_accept`
- `pre_finish`, `post_finish`
- `pre_reopen`, `post_reopen`

Script lookup order:
- `scripts/plan-hooks/<hook>.py` (single file)
- `scripts/plan-hooks/<hook>/*.py` (all `.py` files sorted by filename)

Execution behavior:
- Hooks are executed with `python3` and receive context in environment variables:
  - `PLAN_TASK_ID` — task id (e.g., `0001`)
  - `PLAN_REPO_ROOT` — repository root path
  - `PLAN_CURRENT_STATUS` — current status of the task
  - `PLAN_TASK_FILE` — relative task_file path
- Hooks may print helpful messages to stdout/stderr.
- If a hook script exits with non-zero status, the transition is blocked and the script output is shown to the user.
- Hooks may be used to integrate CI checks, generate reports, notify services, or enforce repository-specific policies.

Sample hook (template):
- `scripts/plan-hooks/pre_finish.py` — example: check for an acceptance report under `plan/tasks/<id>/reports/acceptance.md`; exit non-zero to block `finish` until report exists.

Writing hooks (Python):
- Hooks should implement a unified interface: `def run(ctx: dict) -> dict` and when invoked as a script they should read a JSON `ctx` from stdin and print a JSON `res` to stdout.
- A minimal `res` example: `{"ok": true, "message": "all good"}`. When `ok` is false, the transition is blocked and `message` is shown to the user.
- Keep hooks idempotent and fast.
- Use environment variables (legacy support) or the JSON `ctx` to access repository state; use the `repo_root` key to read files.
- Basic checks provided by `cosmos plan hooks check`:
  - Syntax check using `python -m py_compile`.
  - Entrypoint check: module must define a callable `run(ctx: dict)`.
- Use `cosmos plan hooks add --name <name>` to create a new script from the template, `cosmos plan hooks list` to list hooks, and `cosmos plan hooks check [--name <name>]` to validate scripts.

Migration note:
- The default checks are conservative — if they don't match your workflow, add a custom `pre_*` hook to implement your own checks or extend CI.


- Delete a task (destructive):
  - `cosmos plan delete --id 000X --yes` — removes `plan/tasks/000X/`, `plan/archive/000X/` (if present), and its entry in `todo.toml`.

- Show details and history:
  - `cosmos plan show --id 000X` — prints task metadata, the current `task.md`, and any history entries.

- Add a history event:
  - `cosmos plan log --id 000X --message "Started implementation" --author alice`

- Validation:
  - `cosmos plan validate` — checks that tasks have valid `kind` and `status` values and that `finished` tasks have their files under `plan/archive/`.

## Team guidelines 💡
- Use history entries liberally to capture decisions, test outcomes, reviewer notes, and links to PRs or CI artifacts.
- Prefer explicit status transitions rather than free-form comments: set status with `cosmos plan update --status <status>` so validations and tooling can act on them.
- Keep task `task.md` focused on the goal, acceptance criteria, and short reproduction steps; use history for ephemeral logs.

---

For automation and policy, see `scripts/` and `docs/AGENT_INSTRUCTIONS.md` for how agents and CI should interact with plan tasks.
