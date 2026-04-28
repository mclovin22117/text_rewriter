use thiserror::Error;
use serde::{Deserialize, Serialize};

use text_rewriter_core::{build_prompt, RewriteRequest};
use text_rewriter_secure_store::InMemorySecretStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: String,
    pub provider_name: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key_ref: Option<String>,
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
    #[error("network error: {0}")]
    Network(String),
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
            return Err(ProviderError::Misconfigured("model cannot be empty".to_string()));
        }

        // The mock implementation proves command/prompt flow before real API wiring.
        let prompt = build_prompt(request);
        let rewritten = format!(
            "[mock:{}] {}",
            self.config.model,
            prompt.lines().last().unwrap_or("")
        );

        Ok(RewriteResult {
            rewritten_text: rewritten,
            model_used: self.config.model.clone(),
        })
    }
}

// Ollama provider: local LLM server (defaults to http://localhost:11434)
pub struct OllamaProvider {
    config: ProviderConfig,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }
}

impl RewriteProvider for OllamaProvider {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn rewrite(&self, request: &RewriteRequest) -> Result<RewriteResult, ProviderError> {
        let base = self
            .config
            .base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        if self.config.model.trim().is_empty() {
            return Err(ProviderError::Misconfigured("model cannot be empty".to_string()));
        }

        // Build a minimal request for Ollama's /api/generate endpoint.
        let endpoint = format!("{}/api/generate", base.trim_end_matches('/'));
        let prompt = build_prompt(request);

        let client = reqwest::blocking::Client::new();
        let body = serde_json::json!({
            "model": self.config.model,
            "prompt": prompt,
        });

        let resp = client
            .post(&endpoint)
            .json(&body)
            .send()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(ProviderError::Unavailable);
        }

        let json: serde_json::Value = resp.json().map_err(|e| ProviderError::Network(e.to_string()))?;
        // Try to extract a textual response from common fields
        let rewritten = json
            .get("text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                json.get("output")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "".to_string());

        Ok(RewriteResult {
            rewritten_text: rewritten,
            model_used: self.config.model.clone(),
        })
    }
}

// OpenAI-compatible provider (works with OpenAI/Azure/local OpenAI-compatible endpoints)
pub struct OpenAICompatibleProvider {
    config: ProviderConfig,
    secret_store: Option<InMemorySecretStore>,
}

impl OpenAICompatibleProvider {
    pub fn new(config: ProviderConfig, secret_store: Option<InMemorySecretStore>) -> Self {
        Self { config, secret_store }
    }
}

impl RewriteProvider for OpenAICompatibleProvider {
    fn name(&self) -> &'static str {
        "openai_compatible"
    }

    fn rewrite(&self, request: &RewriteRequest) -> Result<RewriteResult, ProviderError> {
        let base = self
            .config
            .base_url
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.openai.com");

        if self.config.model.trim().is_empty() {
            return Err(ProviderError::Misconfigured("model cannot be empty".to_string()));
        }

        // Retrieve api key from secret store if configured
        let api_key = self
            .config
            .api_key_ref
            .as_ref()
            .and_then(|key| {
                self.secret_store
                    .as_ref()
                    .and_then(|s| s.get(key).map(|s| s.to_string()))
            })
            .or_else(|| self.config.api_key_ref.clone());

        if api_key.as_deref().unwrap_or("").is_empty() {
            return Err(ProviderError::Misconfigured("api key missing".to_string()));
        }

        let endpoint = format!("{}/v1/chat/completions", base.trim_end_matches('/'));
        let prompt = build_prompt(request);

        let client = reqwest::blocking::Client::new();
        let body = serde_json::json!({
            "model": self.config.model,
            "messages": [
                {"role":"system", "content": "You are a concise text rewriting assistant."},
                {"role":"user", "content": prompt}
            ],
            "max_tokens": 1024
        });

        let resp = client
            .post(&endpoint)
            .bearer_auth(api_key.unwrap())
            .json(&body)
            .send()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(ProviderError::Unavailable);
        }

        let json: serde_json::Value = resp.json().map_err(|e| ProviderError::Network(e.to_string()))?;
        let rewritten = json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|first| first.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(RewriteResult {
            rewritten_text: rewritten,
            model_used: self.config.model.clone(),
        })
    }
}

// Simple factory for tests and wiring
pub fn create_provider_from_config(
    cfg: &ProviderConfig,
    secret_store: Option<InMemorySecretStore>,
) -> Box<dyn RewriteProvider> {
    match cfg.provider_type.as_str() {
        "ollama" => Box::new(OllamaProvider::new(cfg.clone())),
        "openai_compatible" => Box::new(OpenAICompatibleProvider::new(cfg.clone(), secret_store)),
        _ => Box::new(MockProvider::new(cfg.clone())),
    }
}

#[cfg(test)]
mod tests {
    use text_rewriter_core::{RewriteCommand, RewriteRequest};

    use super::*;

    #[test]
    fn mock_provider_returns_rewrite() {
        let provider = MockProvider::new(ProviderConfig {
            provider_type: "mock".to_string(),
            provider_name: Some("mock".to_string()),
            model: "test-model".to_string(),
            base_url: None,
            api_key_ref: None,
        });

        let request = RewriteRequest {
            command: RewriteCommand::Fix,
            source_text: "teh sentence".to_string(),
        };

        let result = provider.rewrite(&request).expect("mock provider should succeed");
        assert!(result.rewritten_text.contains("teh sentence"));
        assert_eq!(result.model_used, "test-model");
    }

    #[test]
    fn openai_provider_fails_when_no_key() {
        let cfg = ProviderConfig {
            provider_type: "openai_compatible".to_string(),
            provider_name: Some("openai".to_string()),
            model: "gpt-4o-mini".to_string(),
            base_url: None,
            api_key_ref: None,
        };

        let provider = OpenAICompatibleProvider::new(cfg, None);
        let request = RewriteRequest {
            command: RewriteCommand::Fix,
            source_text: "test".to_string(),
        };

        let res = provider.rewrite(&request);
        assert!(res.is_err());
    }
}
