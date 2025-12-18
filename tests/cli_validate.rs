use predicates::prelude::*;

#[test]
fn validate_quick() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("validate").arg("--level").arg("quick");
    cmd.assert().success().stdout(predicate::str::contains("Validation summary"));
    Ok(())
}
