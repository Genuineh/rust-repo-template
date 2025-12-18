use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn validate_quick() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("validate").arg("--level").arg("quick");
    cmd.assert().success().stdout(predicate::str::contains("Validation summary"));
    Ok(())
}
