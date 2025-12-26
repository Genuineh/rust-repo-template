use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_min_repo(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("plan/tasks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(dir.join("README.md"), "dummy")?;
    Ok(())
}

#[test]
fn plan_list_and_validate_ok() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    write_min_repo(td.path())?;
    // write a simple todo.toml with one done task in archive and one open in tasks
    fs::create_dir_all(td.path().join("plan/archive"))?;
    fs::write(
        td.path().join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "Done task"
status = "finished"
task_file = "archive/0001.md"

[[task]]
id = "0002"
title = "Open task"
status = "pending_review"
task_file = "tasks/0002/task.md"
"#,
    )?;
    fs::write(td.path().join("plan/archive/0001.md"), "done")?;
    fs::create_dir_all(td.path().join("plan/tasks/0002"))?;
    fs::write(td.path().join("plan/tasks/0002/task.md"), "open")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(td.path()).arg("plan").arg("list");
    cmd.assert().success().stdout(predicate::str::contains("Tasks (2)"));

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(td.path()).arg("plan").arg("validate");
    cmd.assert().success().stdout(predicate::str::contains("Plan validation OK"));

    Ok(())
}

#[test]
fn plan_validate_fails_on_missing_files() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    write_min_repo(td.path())?;
    // missing archive file for done task
    fs::create_dir_all(td.path().join("plan/tasks"))?;
    fs::write(
        td.path().join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
status = "finished"
task_file = "archive/0001.md"
"#,
    )?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(td.path()).arg("plan").arg("validate");
    cmd.assert().failure().stdout(predicate::str::contains("Plan validation found"));
    Ok(())
}
