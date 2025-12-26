use predicates::prelude::*;

#[test]
fn ai_doctor_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("ai").arg("doctor");
    cmd.assert().success().stdout(predicate::str::contains("AI/LLM configuration"));
    Ok(())
}

#[test]
fn ai_eval_unavailable_without_feature() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("ai").arg("eval");
    cmd.assert().failure().stderr(predicate::str::contains("not enabled in this build"));
    Ok(())
}
