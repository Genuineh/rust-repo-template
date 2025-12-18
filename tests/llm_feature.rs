#![cfg(feature = "llm")]

use std::env;
use std::path::PathBuf;

#[test]
fn llm_stub_creates_report() -> Result<(), Box<dyn std::error::Error>> {
    // run with stub provider
    env::set_var("LLM_PROVIDER", "stub");
    let repo_root = PathBuf::from(".");
    // call the API directly
    rust_repo_template::llm::evaluate_with_llm(&repo_root)?;
    assert!(repo_root.join(".cosmos_llm_report.txt").exists());
    Ok(())
}
