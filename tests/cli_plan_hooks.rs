use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn write_min_plan(dir: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("plan/tasks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "Old"
status = "pending_review"
task_file = "tasks/0001/task.md"
"#,
    )?;
    fs::create_dir_all(dir.join("plan/tasks/0001"))?;
    fs::write(
        dir.join("plan/tasks/0001/task.md"),
        "This is a task with Acceptance criteria:\n- Should do X\n\nTest plan:\n- Add unit tests\n",
    )?;
    Ok(())
}

#[test]
fn custom_pre_finish_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    // add acceptance report after acceptance step; we'll omit it and expect pre_finish to block
    // progress to under_acceptance
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir)
        .arg("plan")
        .arg("review")
        .arg("--id")
        .arg("0001")
        .arg("--decision")
        .arg("accept");
    cmd.assert().success();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("start").arg("--id").arg("0001");
    cmd.assert().success();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("test").arg("--id").arg("0001");
    cmd.assert().success();

    // create a test report to satisfy accept check
    fs::create_dir_all(dir.join("plan/tasks/0001/reports"))?;
    fs::write(dir.join("plan/tasks/0001/reports/test-report.txt"), "all tests passed")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("accept").arg("--id").arg("0001");
    cmd.assert().success();

    // write a blocking pre_finish hook into the test repo
    fs::create_dir_all(dir.join("scripts/plan-hooks"))?;
    fs::write(
        dir.join("scripts/plan-hooks/pre_finish.py"),
        r#"#!/usr/bin/env python3
import os
import sys
TASK_ID = os.environ.get('PLAN_TASK_ID')
REPO = os.environ.get('PLAN_REPO_ROOT')
ai_path = os.environ.get('PLAN_AI_VALIDATION_PATH')
if not ai_path or not os.path.exists(ai_path):
    print('pre-finish check: missing ai validation report')
    sys.exit(2)
report = os.path.join(REPO, 'plan', 'tasks', TASK_ID, 'reports', 'acceptance.md')
if not os.path.exists(report):
    print(f'pre-finish check: acceptance report not found at {report}')
    sys.exit(1)
print('pre-finish hook: OK')
sys.exit(0)
"#,
    )?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    let res = cmd
        .current_dir(dir)
        .arg("plan")
        .arg("--ai-validate")
        .arg("finish")
        .arg("--id")
        .arg("0001")
        .assert();
    res.failure().stderr(predicate::str::contains("pre-finish check: acceptance report not found"));

    // create the acceptance report and try again
    fs::create_dir_all(dir.join("plan/tasks/0001/reports"))?;
    fs::write(dir.join("plan/tasks/0001/reports/acceptance.md"), "accepted")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("--ai-validate").arg("finish").arg("--id").arg("0001");
    cmd.assert().success();

    Ok(())
}

#[test]
fn default_review_check_blocks_missing_acceptance() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/tasks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    // task without acceptance criteria (short)
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0002"
title = "Short"
status = "pending_review"
task_file = "tasks/0002/task.md"
"#,
    )?;
    fs::create_dir_all(dir.join("plan/tasks/0002"))?;
    fs::write(dir.join("plan/tasks/0002/task.md"), "short")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    let res = cmd
        .current_dir(dir)
        .arg("plan")
        .arg("review")
        .arg("--id")
        .arg("0002")
        .arg("--decision")
        .arg("accept")
        .assert();
    res.failure().stderr(predicate::str::contains(
        "review check failed: task appears to be missing acceptance criteria",
    ));

    Ok(())
}
