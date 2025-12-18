use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn generate_basis_dry_run_then_apply() -> Result<(), Box<dyn std::error::Error>> {
    // dry run
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--category").arg("basis");
    cmd.assert().success().stdout(predicate::str::contains("Matched"));

    // apply to temp dir
    let td = tempdir()?;
    let dest = td.path().join("out");
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--category").arg("basis").arg("--apply").arg("--out-dir").arg(dest.to_str().unwrap());
    cmd.assert().success().stdout(predicate::str::contains("Files written"));

    // Check that Cargo.toml got copied
    assert!(dest.join("Cargo.toml").exists());
    Ok(())
}
