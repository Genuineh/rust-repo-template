use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn generate_from_template_folder() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().join("out");

    // dry-run: should list files
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--template").arg("this_repo");
    cmd.assert().success().stdout(predicate::str::contains("Template 'this_repo' matched"));

    // apply to out dir
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--template").arg("this_repo").arg("--apply").arg("--out-dir").arg(out.to_str().unwrap());
    cmd.assert().success().stdout(predicate::str::contains("Template files written to"));

    // check some files copied
    assert!(out.join("Cargo.toml").exists());
    assert!(out.join("docs/getting-started.md").exists());
    assert!(out.join("scripts/validate_plan.py").exists());

    Ok(())
}
