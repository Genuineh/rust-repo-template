use anyhow::Result;

/// LLM plugin interface.
///
/// This module is feature-gated behind the `llm` Cargo feature. When the feature is
/// disabled, the public `evaluate_with_llm` function returns an error explaining that
/// LLM support isn't enabled.

/// Evaluate repository artifacts with an LLM provider. Returns Ok(()) when the
/// evaluation completes (placeholder), or an error when not available or on errors.
pub fn evaluate_with_llm(_repo_root: &std::path::Path) -> Result<()> {
    // If the llm feature is enabled, implementation will go here.
    // For now, provide a clear error when the feature is not enabled.
    #[cfg(not(feature = "llm"))]
    {
        anyhow::bail!("LLM support not enabled: compile with --features llm and configure a provider to enable");
    }

    #[cfg(feature = "llm")]
    {
        // Placeholder: real implementation should call configured provider, pass
        // evaluation prompts, gather evidence and return structured results.
        // We keep this intentionally minimal for the initial scaffold.
        Ok(())
    }
}
