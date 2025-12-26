use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn plan_show_displays_fields_and_content() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    fs::create_dir_all(dir.join("plan/archive"))?;
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n")?;
    fs::write(
        dir.join("plan/todo.toml"),
        r#"[[task]]
id = "0001"
title = "ShowMe"
status = "finished"
task_file = "archive/0001.md"
"#,
    )?;
    fs::write(dir.join("plan/archive/0001.md"), "details here")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(dir).arg("plan").arg("show").arg("--id").arg("0001");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ShowMe"))
        .stdout(predicate::str::contains("details here"));
    Ok(())
}
