use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn validate_quick() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("validate").arg("--level").arg("quick");
    cmd.assert().success().stdout(predicate::str::contains("Validation summary"));
    Ok(())
}

#[test]
fn validate_exits_nonzero_on_errors() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(td.path()).arg("validate").arg("--level").arg("quick");
    cmd.assert().failure().code(2);
    Ok(())
}
