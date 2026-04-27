use thiserror::Error;

use text_rewriter_core::{build_prompt, RewriteRequest};

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub provider_name: String,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RewriteResult {
    pub rewritten_text: String,
    pub model_used: String,
}

pub trait RewriteProvider {
    fn name(&self) -> &'static str;
    fn rewrite(&self, request: &RewriteRequest) -> Result<RewriteResult, ProviderError>;
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider misconfigured: {0}")]
    Misconfigured(String),
    #[error("provider unavailable")]
    Unavailable,
}

#[derive(Debug, Clone)]
pub struct MockProvider {
    config: ProviderConfig,
}

impl MockProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }
}

impl RewriteProvider for MockProvider {
    fn name(&self) -> &'static str {
        "mock"
    }

    fn rewrite(&self, request: &RewriteRequest) -> Result<RewriteResult, ProviderError> {
        if self.config.model.trim().is_empty() {
            return Err(ProviderError::Misconfigured(
                "model cannot be empty".to_string(),
            ));
        }

        // The mock implementation proves command/prompt flow before real API wiring.
        let prompt = build_prompt(request);
        let rewritten = format!("[mock:{}] {}", self.config.model, prompt.lines().last().unwrap_or(""));

        Ok(RewriteResult {
            rewritten_text: rewritten,
            model_used: self.config.model.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use text_rewriter_core::{RewriteCommand, RewriteRequest};

    use super::*;

    #[test]
    fn mock_provider_returns_rewrite() {
        let provider = MockProvider::new(ProviderConfig {
            provider_name: "mock".to_string(),
            model: "test-model".to_string(),
            base_url: None,
        });

        let request = RewriteRequest {
            command: RewriteCommand::Fix,
            source_text: "teh sentence".to_string(),
        };

        let result = provider.rewrite(&request).expect("mock provider should succeed");
        assert!(result.rewritten_text.contains("teh sentence"));
        assert_eq!(result.model_used, "test-model");
    }
}
