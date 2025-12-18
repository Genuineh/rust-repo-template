use anyhow::{Context, Result};

/// LLM plugin interface.
///
/// This module is feature-gated behind the `llm` Cargo feature. When the feature is
/// disabled, the public `evaluate_with_llm` function returns an error explaining that
/// LLM support isn't enabled.

/// Evaluate repository artifacts with an LLM provider. Returns Ok(()) when the
/// evaluation completes (placeholder), or an error when not available or on errors.
pub fn evaluate_with_llm(repo_root: &std::path::Path) -> Result<()> {
    // If the llm feature is enabled, implementation will go here.
    // For now, provide a clear error when the feature is not enabled.
    #[cfg(not(feature = "llm"))]
    {
        anyhow::bail!("LLM support not enabled: compile with --features llm and configure a provider to enable");
    }

    #[cfg(feature = "llm")]
    {
        // Basic provider stub: look for LLM_PROVIDER env var
        match std::env::var("LLM_PROVIDER") {
            Ok(p) if p.to_lowercase() == "stub" => {
                // write a simple report file to repo_root to indicate action
                let report = repo_root.join(".cosmos_llm_report.txt");
                std::fs::write(report, "LLM provider stub ran successfully\n").context("writing llm report")?;
                Ok(())
            }
            Ok(other) => anyhow::bail!("unsupported LLM_PROVIDER '{}', set to 'stub' for testing", other),
            Err(_) => anyhow::bail!("no LLM provider configured: set LLM_PROVIDER=stub for the stub provider"),
        }
    }
}
