use anyhow::Result;
use text_rewriter_core::{RewriteCommand, RewriteRequest};
use text_rewriter_providers::{MockProvider, ProviderConfig, RewriteProvider};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let _binary = args.next();

    let command_token = args.next().unwrap_or_else(|| "?fix".to_string());
    let source_text = args
        .next()
        .unwrap_or_else(|| "teh quick brown fox jump over teh lazy dog".to_string());

    let command = RewriteCommand::from_token(&command_token)?;

    let request = RewriteRequest {
        command,
        source_text,
    };

    let provider = MockProvider::new(ProviderConfig {
        provider_name: "mock".to_string(),
        model: "starter-model".to_string(),
        base_url: None,
    });

    let result = provider.rewrite(&request)?;
    println!("provider={} model={}", provider.name(), result.model_used);
    println!("rewritten_text={}", result.rewritten_text);

    Ok(())
}
