use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn plan_log_appends_history() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/tasks/0001/history"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "LogTask"
status = "pending_review"
task_file = "tasks/0001/task.md"
"#,
    )?;
    fs::write(dir.join("plan/tasks/0001/task.md"), "initial")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir)
        .arg("plan")
        .arg("log")
        .arg("--id")
        .arg("0001")
        .arg("--message")
        .arg("Started work")
        .arg("--author")
        .arg("alice");

    cmd.assert().success().stdout(predicate::str::contains("Logged event"));

    // check that a history file exists and contains message
    let hdir = dir.join("plan/tasks/0001/history");
    let entries: Vec<_> = std::fs::read_dir(hdir)?.filter_map(|e| e.ok()).collect();
    assert!(!entries.is_empty(), "history entry should exist");
    let content = fs::read_to_string(entries[0].path())?;
    assert!(content.contains("Started work"));
    assert!(content.contains("author: alice"));
    Ok(())
}
