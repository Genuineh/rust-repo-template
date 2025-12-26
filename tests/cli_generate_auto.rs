use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn generate_yes_without_allow_delete_fails() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();
    // create an extra file that should be deleted
    let extra = out.join("EXTRA_FILE_TO_DELETE_AUTO.txt");
    fs::create_dir_all(&out)?;
    fs::write(&extra, "remove me")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("generate")
        .arg("--template")
        .arg("default")
        .arg("--category")
        .arg("plan")
        .arg("--apply")
        .arg("--yes")
        .arg("--out-dir")
        .arg(out.as_path());

    cmd.assert().failure().stderr(predicate::str::contains("--allow-delete"));
    Ok(())
}

#[test]
fn generate_yes_with_allow_delete_passes() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();
    // create an extra file that should be deleted
    let extra = out.join("EXTRA_FILE_TO_DELETE_AUTO2.txt");
    fs::create_dir_all(&out)?;
    fs::write(&extra, "remove me")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("generate")
        .arg("--template")
        .arg("default")
        .arg("--category")
        .arg("plan")
        .arg("--apply")
        .arg("--yes")
        .arg("--allow-delete")
        .arg("--out-dir")
        .arg(out.as_path());

    cmd.assert().success();
    assert!(!extra.exists(), "extra file should have been deleted by --allow-delete");
    assert!(out.join("plan/todo.toml").exists(), "todo.toml should be created in out dir");
    Ok(())
}
