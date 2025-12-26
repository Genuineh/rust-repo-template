use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn hooks_add_list_check_and_failures() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
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
    fs::write(dir.join("plan/tasks/0001/task.md"), "task body")?;

    // add hook
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("hooks").arg("add").arg("--name").arg("sample_hook");
    cmd.assert().success().stdout(predicate::str::contains("Created hook"));

    // list should include it
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("hooks").arg("list");
    cmd.assert().success().stdout(predicate::str::contains("sample_hook"));

    // check should pass
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("hooks").arg("check").arg("--name").arg("sample_hook");
    cmd.assert().success().stdout(predicate::str::contains("OK:"));

    // create a bad script to test syntax failure
    let _ = fs::write(
        dir.join("scripts/plan-hooks/bad.py"),
        "def run(ctx):\n    return {\"ok\": True\n",
    );
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    let r = cmd
        .current_dir(dir)
        .arg("plan")
        .arg("hooks")
        .arg("check")
        .arg("--name")
        .arg("bad")
        .assert();
    r.failure().stderr(predicate::str::contains("syntax error"));

    Ok(())
}
