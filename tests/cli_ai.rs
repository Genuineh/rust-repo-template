use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn ai_eval_rule_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("ai-eval").arg("--mode").arg("rule");
    cmd.assert().success().stdout(predicate::str::contains("AI heuristics"));
    Ok(())
}

#[test]
fn ai_eval_llm_unavailable() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("ai-eval").arg("--mode").arg("llm");
    cmd.assert().failure().stderr(predicate::str::contains("not enabled"));
    Ok(())
}
