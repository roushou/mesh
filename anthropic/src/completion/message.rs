use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::models::claude::ClaudeModel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged, rename_all = "lowercase")]
pub enum System {
    Text(String),
    Structured(SystemPrompt),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemPrompt {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: CacheType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CacheType {
    Ephemeral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    /// The model that will complete your prompt e.g. Claude 3.5 Sonnet
    pub model: ClaudeModel,

    /// The maximum number of tokens to generate before stopping.
    ///
    /// Defaults to 1000 tokens.
    ///
    /// Note that models may stop before reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
    pub max_tokens: u32,

    /// Input messages.
    pub messages: Vec<Message>,

    /// An object describing metadata about the request.
    pub metadata: Option<MessageMetadata>,

    /// Custom text sequences that will cause the model to stop generating.
    pub stop_sequences: Option<Vec<String>>,

    /// Whether to incrementally stream the response using server-sent events.
    pub stream: bool,

    /// System prompt.
    ///
    /// A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<System>,

    /// Amount of randomness injected into the response.
    ///
    /// Defaults to 1.0. Ranges from 0.0 to 1.0.
    ///
    /// Use temperature closer to 0.0 for analytical / multiple choice, and closer to 1.0 for creative and generative tasks.
    /// Note that even with temperature of 0.0, the results will not be fully deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses. Learn more technical details here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i8>,

    /// Use nucleus sampling.
    ///
    /// In nucleus sampling, we compute the cumulative distribution over all the options for each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by top_p. You should either alter temperature or top_p, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i8>,
}

impl MessageRequest {
    pub fn new(model: ClaudeModel, max_tokens: u32, messages: Vec<Message>) -> Self {
        Self {
            model,
            max_tokens,
            messages,
            ..Default::default()
        }
    }

    pub fn with_metadata(mut self, metadata: MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn with_system(mut self, system: System) -> Self {
        self.system = Some(system);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_k(mut self, top_k: i8) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_top_p(mut self, top_p: i8) -> Self {
        self.top_p = Some(top_p);
        self
    }
}

impl Default for MessageRequest {
    fn default() -> Self {
        Self {
            model: ClaudeModel::Claude35Sonnet,
            max_tokens: 1000,
            messages: Vec::new(),
            metadata: None,
            stop_sequences: None,
            stream: false,
            system: None,
            temperature: None,
            top_k: None,
            top_p: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageMetadata {
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub role: RoleResponse,
    pub content: Vec<Content>,
    pub model: ClaudeModel,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoleResponse {
    Assistant,
}

impl RoleResponse {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assistant => "assistant",
        }
    }
}

impl Display for RoleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndTurn => write!(f, "end_turn"),
            Self::MaxTokens => write!(f, "max_tokens"),
            Self::StopSequence => write!(f, "stop_sequence"),
            Self::ToolUse => write!(f, "tool_use"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_input_tokens: Option<u32>,
    pub cache_read_input_tokens: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_set_metadata() {
        let request = MessageRequest::default();
        assert_eq!(request.metadata, None);

        let metadata = MessageMetadata {
            user_id: Some("user-id".to_string()),
        };
        let request = request.with_metadata(metadata.clone());
        assert_eq!(request.metadata, Some(metadata));
    }

    #[test]
    fn should_set_stop_sequences() {
        let request = MessageRequest::default();
        assert_eq!(request.stop_sequences, None);

        let stop_sequences: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
        let request = request.with_stop_sequences(stop_sequences.clone());
        assert_eq!(request.stop_sequences, Some(stop_sequences));
    }

    #[test]
    fn should_set_stream() {
        let request = MessageRequest::default();
        assert_eq!(request.stream, false);

        let request = request.with_stream(true);
        assert_eq!(request.stream, true);

        let request = request.with_stream(false);
        assert_eq!(request.stream, false);
    }

    #[test]
    fn should_set_system() {
        let request = MessageRequest::default();
        assert_eq!(request.system, None);

        let system = System::Structured(SystemPrompt {
            text: "You are an experienced software engineer".into(),
            content_type: ContentType::Text,
            cache_control: Some(CacheControl {
                cache_type: CacheType::Ephemeral,
            }),
        });
        let request = request.with_system(system.clone());
        assert_eq!(request.system, Some(system));
    }

    #[test]
    fn should_set_temperature() {
        let request = MessageRequest::default();
        assert_eq!(request.temperature, None);

        let temperature: f32 = 0.9;
        let request = request.with_temperature(temperature);
        assert_eq!(request.temperature, Some(temperature));
    }

    #[test]
    fn should_set_top_k() {
        let request = MessageRequest::default();
        assert_eq!(request.top_k, None);

        let top_k: i8 = 1;
        let request = request.with_top_k(top_k);
        assert_eq!(request.top_k, Some(top_k));
    }

    #[test]
    fn should_set_top_p() {
        let request = MessageRequest::default();
        assert_eq!(request.top_p, None);

        let top_p: i8 = 1;
        let request = request.with_top_p(top_p);
        assert_eq!(request.top_p, Some(top_p));
    }

    #[test]
    fn should_serialize_message() {
        let message = Message {
            role: Role::User,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }],
        };
        assert_eq!(
            serde_json::to_value(&message).unwrap(),
            serde_json::json!({
                "role": "user",
                "content": [{
                    "type": "text",
                    "text": "Hello World"
                }],
            })
        );

        let message = Message {
            role: Role::Assistant,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }],
        };
        assert_eq!(
            serde_json::to_value(&message).unwrap(),
            serde_json::json!({
                "role": "assistant",
                "content": [{
                    "type": "text",
                    "text": "Hello World"
                }],
            })
        );
    }

    #[test]
    fn should_deserialize_message() {
        let json = serde_json::json!({
            "role": "user",
            "content": [{
                "type": "text",
                "text": "Hello World",
            }]
        });
        let message: Message = serde_json::from_value(json).unwrap();
        assert_eq!(message.role, Role::User);
        assert_eq!(
            message.content,
            vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }]
        );

        let json = serde_json::json!({
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": "Hello World",
            }]
        });
        let message: Message = serde_json::from_value(json).unwrap();
        assert_eq!(message.role, Role::Assistant);
        assert_eq!(
            message.content,
            vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }]
        );
    }
}
