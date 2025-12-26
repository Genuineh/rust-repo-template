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
    fs::write(dir.join("plan/tasks/0001/task.md"), "This task includes Acceptance criteria:\n- Should do X\n\nTest plan:\n- Add tests under tests/ or provide test plan\n")?;
    Ok(())
}

#[test]
fn illegal_start_fails_with_hint() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("start").arg("--id").arg("0001");

    cmd.assert().failure().stderr(
        predicate::str::contains("Hint: Ensure the task was accepted")
            .or(predicate::str::contains("queued")),
    );

    Ok(())
}

#[test]
fn finish_without_accept_fails_with_hint() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    // try to finish directly
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("finish").arg("--id").arg("0001");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Hint: Ensure acceptance report is present"));

    Ok(())
}

#[test]
fn review_accept_appends_history_and_advances_status() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir)
        .arg("plan")
        .arg("review")
        .arg("--id")
        .arg("0001")
        .arg("--decision")
        .arg("accept")
        .arg("--message")
        .arg("LGTM")
        .arg("--author")
        .arg("bob");
    cmd.assert().success();

    // status should be queued
    let todo = fs::read_to_string(dir.join("plan/todo.toml"))?;
    assert!(todo.contains("status = \"queued\""));

    // history file should contain message and author; check both tasks/ and archive/ as fallback
    let cand1 = dir.join("plan/tasks/0001/history");
    let cand2 = dir.join("plan/archive/0001/history");
    let mut found = false;
    for cand in [cand1, cand2].iter() {
        if cand.exists() {
            for e in fs::read_dir(cand)? {
                let p = e?.path();
                if p.is_file() {
                    let body = fs::read_to_string(&p)?;
                    if body.contains("LGTM") && body.contains("author: bob") {
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }
    }
    assert!(found, "history entry with message and author not found");
    Ok(())
}

#[test]
fn full_cycle_reopen_moves_files_back() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    // accept, start, test, accept, finish
    // review requires a decision
    // accept, start, test, accept, finish
    let mut cmdr = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmdr.current_dir(dir)
        .arg("plan")
        .arg("review")
        .arg("--id")
        .arg("0001")
        .arg("--decision")
        .arg("accept");
    cmdr.assert().success();

    // start and test
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

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("finish").arg("--id").arg("0001");
    cmd.assert().success();

    // archive should exist and tasks/0001 should not
    assert!(dir.join("plan/archive/0001/task.md").exists());
    assert!(!dir.join("plan/tasks/0001/task.md").exists());

    // reopen
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("reopen").arg("--id").arg("0001");
    cmd.assert().success();

    // now tasks/0001 should exist
    assert!(dir.join("plan/tasks/0001/task.md").exists());
    let todo = fs::read_to_string(dir.join("plan/todo.toml"))?;
    assert!(todo.contains("status = \"pending_review\""));

    Ok(())
}
