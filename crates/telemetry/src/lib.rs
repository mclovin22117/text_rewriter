#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteLogEvent {
    pub command: String,
    pub provider: String,
    pub model: String,
    pub success: bool,
}

pub fn summarize_event(event: &RewriteLogEvent) -> String {
    format!(
        "command={} provider={} model={} success={}",
        event.command, event.provider, event.model, event.success
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_contains_core_fields() {
        let event = RewriteLogEvent {
            command: "?fix".to_string(),
            provider: "mock".to_string(),
            model: "test-model".to_string(),
            success: true,
        };

        let summary = summarize_event(&event);
        assert!(summary.contains("command=?fix"));
        assert!(summary.contains("success=true"));
    }
}
