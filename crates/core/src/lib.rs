use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteCommand {
    Fix,
    Improve,
    Enhance,
    Shorten,
    Formal,
    Casual,
}

impl RewriteCommand {
    pub fn from_token(token: &str) -> Result<Self, CommandParseError> {
        let normalized = token.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "?fix" | "fix" => Ok(Self::Fix),
            "?improve" | "improve" => Ok(Self::Improve),
            "?enhance" | "enhance" => Ok(Self::Enhance),
            "?shorten" | "shorten" => Ok(Self::Shorten),
            "?formal" | "formal" => Ok(Self::Formal),
            "?casual" | "casual" => Ok(Self::Casual),
            _ => Err(CommandParseError::UnsupportedCommand(token.to_string())),
        }
    }

    pub fn system_instruction(self) -> &'static str {
        match self {
            Self::Fix => "Correct grammar, spelling, and punctuation only. Keep meaning unchanged.",
            Self::Improve => "Improve clarity and flow while preserving original meaning.",
            Self::Enhance => "Enhance readability and style while keeping factual content unchanged.",
            Self::Shorten => "Make the text concise while preserving key intent and facts.",
            Self::Formal => "Rewrite in a formal, professional tone.",
            Self::Casual => "Rewrite in a casual, friendly tone.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteRequest {
    pub command: RewriteCommand,
    pub source_text: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandParseError {
    #[error("unsupported command: {0}")]
    UnsupportedCommand(String),
}

pub fn build_prompt(request: &RewriteRequest) -> String {
    format!(
        "You are a text rewriting assistant.\nInstruction: {}\nReturn only rewritten text.\nInput:\n{}",
        request.command.system_instruction(), request.source_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_tokens() {
        assert_eq!(RewriteCommand::from_token("?fix"), Ok(RewriteCommand::Fix));
        assert_eq!(RewriteCommand::from_token("IMPROVE"), Ok(RewriteCommand::Improve));
    }

    #[test]
    fn rejects_unknown_token() {
        assert!(matches!(
            RewriteCommand::from_token("?magic"),
            Err(CommandParseError::UnsupportedCommand(_))
        ));
    }

    #[test]
    fn prompt_contains_instruction_and_input() {
        let request = RewriteRequest {
            command: RewriteCommand::Enhance,
            source_text: "this is a sentence".to_string(),
        };

        let prompt = build_prompt(&request);
        assert!(prompt.contains("Enhance readability and style"));
        assert!(prompt.contains("this is a sentence"));
    }
}
