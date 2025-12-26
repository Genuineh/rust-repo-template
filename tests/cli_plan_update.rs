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
    fs::write(dir.join("plan/tasks/0001/task.md"), "old content")?;
    Ok(())
}

#[test]
fn plan_update_modifies_fields_and_content() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path();
    write_min_plan(dir)?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    // update metadata and content
    cmd.current_dir(dir)
        .arg("plan")
        .arg("update")
        .arg("--id")
        .arg("0001")
        .arg("--title")
        .arg("NewTitle")
        .arg("--content")
        .arg("new body\n\nAcceptance criteria:\n- Do X\n\nTest plan:\n- Add unit tests\n");
    cmd.assert().success().stdout(predicate::str::contains("Updated task 0001"));

    // progress through the lifecycle to finish
    let mut cmd2 = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd2.current_dir(dir)
        .arg("plan")
        .arg("review")
        .arg("--id")
        .arg("0001")
        .arg("--decision")
        .arg("accept");
    cmd2.assert().success();

    let mut cmd3 = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd3.current_dir(dir).arg("plan").arg("start").arg("--id").arg("0001");
    cmd3.assert().success();

    let mut cmd4 = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd4.current_dir(dir).arg("plan").arg("test").arg("--id").arg("0001");
    cmd4.assert().success();

    // create a test report to satisfy accept
    fs::create_dir_all(dir.join("plan/tasks/0001/reports"))?;
    fs::write(dir.join("plan/tasks/0001/reports/test-report.txt"), "ok")?;

    let mut cmd5 = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd5.current_dir(dir).arg("plan").arg("accept").arg("--id").arg("0001");
    cmd5.assert().success();

    let mut cmd6 = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd6.current_dir(dir).arg("plan").arg("finish").arg("--id").arg("0001");
    cmd6.assert().success().stdout(predicate::str::contains("finished"));

    let todo = fs::read_to_string(dir.join("plan/todo.toml"))?;
    assert!(todo.contains("NewTitle"));
    assert!(todo.contains("status = \"finished\""));

    let body = fs::read_to_string(dir.join("plan/archive/0001/task.md"))?;
    assert!(body.contains("new body"));

    Ok(())
}
