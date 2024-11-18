use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaudeModel {
    #[serde(rename = "claude-3-5-sonnet-20241022")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-5-haiku-20241022")]
    Claude35Haiku,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}

impl ClaudeModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Claude35Sonnet => "claude-3-5-sonnet-20241022",
            Self::Claude35Haiku => "claude-3.5-haiku-20241022",
            Self::Claude3Opus => "claude-3-opus-20240229",
            Self::Claude3Sonnet => "claude-3-sonnet-20240229",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }
}

impl Default for ClaudeModel {
    fn default() -> Self {
        Self::Claude35Sonnet
    }
}

impl FromStr for ClaudeModel {
    type Err = crate::error::AnthropicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "claude-3-5-sonnet-20241022" => Ok(Self::Claude35Sonnet),
            "claude-3-opus-20240229" => Ok(Self::Claude3Opus),
            "claude-3-sonnet-20240229" => Ok(Self::Claude3Sonnet),
            "claude-3-haiku-20240307" => Ok(Self::Claude3Haiku),
            _ => Err(crate::error::AnthropicError::ModelNotSupported(
                s.to_string(),
            )),
        }
    }
}

impl fmt::Display for ClaudeModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::AnthropicError;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_to_correct_model_names() {
        assert_eq!(
            ClaudeModel::Claude35Sonnet.as_str(),
            "claude-3-5-sonnet-20241022",
        );
        assert_eq!(ClaudeModel::Claude3Opus.as_str(), "claude-3-opus-20240229");
        assert_eq!(
            ClaudeModel::Claude3Sonnet.as_str(),
            "claude-3-sonnet-20240229"
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku.as_str(),
            "claude-3-haiku-20240307"
        );
    }

    #[test]
    fn should_deserialize_to_correct_models() {
        assert_eq!(
            ClaudeModel::Claude35Sonnet,
            ClaudeModel::from_str("claude-3-5-sonnet-20241022").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Opus,
            ClaudeModel::from_str("claude-3-opus-20240229").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Sonnet,
            ClaudeModel::from_str("claude-3-sonnet-20240229").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku,
            ClaudeModel::from_str("claude-3-haiku-20240307").unwrap(),
        );
    }

    #[test]
    fn should_return_error_for_invalid_model() {
        assert!(matches!(
            ClaudeModel::from_str("claude-invalid-model"),
            Err(AnthropicError::ModelNotSupported(_))
        ));
    }
}
