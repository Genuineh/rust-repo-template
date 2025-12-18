# Plan / Task Lifecycle

This directory contains a simple, audit-friendly task plan used to coordinate work on this repository.

Structure:
- `todo.toml` — current task list (open/in-progress/done)
- `tasks/` — per-task markdown records (created when work begins)
- `archive/` — completed task records
- `next_id.txt` — next numeric task id

Usage (summary):
- Use `scripts/plan_create.py "Short title" --assignee alice --labels "docs,process"` to create a new task
- Work on the task in a branch; include `plan: 000X` in commit/PR messages
- When done, use `scripts/plan_close.py 000X --resolution "completed"` which will move the task file to `archive/` and update `todo.toml`

See `scripts/` for helpers and `docs/AGENT_INSTRUCTIONS.md` for longer guidance.
