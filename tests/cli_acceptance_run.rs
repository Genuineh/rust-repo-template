use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn plan_ai_validate_runs_before_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/tasks/0001"))?;
    fs::create_dir_all(dir.join("scripts/plan-hooks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "T"
status = "pending_review"
task_file = "tasks/0001/task.md"
"#,
    )?;
    fs::write(
        dir.join("plan/tasks/0001/task.md"),
        "Acceptance criteria:\n- ok\n\nTest plan:\n- ok\n",
    )?;

    // hook asserts that AI validation ran and produced a report before hooks.
    fs::write(
        dir.join("scripts/plan-hooks/pre_review_accept.py"),
        r#"#!/usr/bin/env python3
import os, sys
p = os.environ.get('PLAN_AI_VALIDATION_PATH')
if not p:
    print('missing PLAN_AI_VALIDATION_PATH')
    sys.exit(2)
if not os.path.exists(p):
    print(f'ai report not found: {p}')
    sys.exit(2)
print('ai report ok')
sys.exit(0)
"#,
    )?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir)
        .arg("plan")
        .arg("--ai-validate")
        .arg("review")
        .arg("--id")
        .arg("0001")
        .arg("--decision")
        .arg("accept");
    cmd.assert().success().stdout(predicate::str::contains("accepted and queued"));
    Ok(())
}
