use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn plan_create_adds_task_and_file() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/tasks"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir)
        .arg("plan")
        .arg("create")
        .arg("--title")
        .arg("New Task")
        .arg("--content")
        .arg("Hello body");

    cmd.assert().success().stdout(predicate::str::contains("Created task"));

    // read todo and ensure task present
    let todo = fs::read_to_string(dir.join("plan/todo.toml"))?;
    assert!(todo.contains("New Task"));

    // ensure tasks file exists
    // find the id in todo
    let id_line = todo.lines().find(|l| l.trim_start().starts_with("id = ")).unwrap();
    let id = id_line.split_once('=').unwrap().1.trim().trim_matches('"').to_string();
    let tf = format!("plan/tasks/{}/task.md", id);
    assert!(dir.join(&tf).exists(), "task file should exist: {}", tf);
    let body = fs::read_to_string(dir.join(&tf))?;
    assert!(body.contains("Hello body"));

    Ok(())
}
