use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn plan_delete_removes_task_and_file() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/tasks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "ToDelete"
status = "open"
task_file = "tasks/0001/task.md"
"#,
    )?;
    fs::create_dir_all(dir.join("plan/tasks/0001"))?;
    fs::write(dir.join("plan/tasks/0001/task.md"), "will be deleted")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("delete").arg("--id").arg("0001").arg("--yes");

    cmd.assert().success().stdout(predicate::str::contains("Deleted task 0001"));

    // todo should no longer contain task
    let todo = fs::read_to_string(dir.join("plan/todo.toml"))?;
    assert!(!todo.contains("ToDelete"));
    // task directory removed
    assert!(!dir.join("plan/tasks/0001").exists());
    // also ensure archive entry not present
    assert!(!dir.join("plan/archive/0001").exists());

    Ok(())
}
