use core::fmt;
use serde::{de::Error, Deserialize, Serialize};
use std::str::FromStr;

use super::message::{MessageResponse, StopReason};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StreamEvent {
    Ping,
    MessageStart { message: MessageResponse },
    MessageDelta(MessageDelta),
    MessageStop,
    ContentBlockStart(ContentBlockStart),
    ContentBlockDelta(ContentBlockDelta),
    ContentBlockStop(ContentBlockStop),
}

impl FromStr for StreamEvent {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: serde_json::Value = serde_json::from_str(s)?;
        let event_type = value["type"]
            .as_str()
            .ok_or_else(|| serde_json::Error::custom("Missing or invalid 'type' field"))?;

        match event_type {
            "ping" => Ok(StreamEvent::Ping),
            "message_start" => {
                let message: MessageResponse = serde_json::from_value(value["message"].clone())?;
                Ok(StreamEvent::MessageStart { message })
            }
            "content_block_start" => {
                let message: ContentBlockStart = serde_json::from_value(value)?;
                Ok(StreamEvent::ContentBlockStart(message))
            }
            "content_block_delta" => {
                let message: ContentBlockDelta = serde_json::from_value(value)?;
                Ok(StreamEvent::ContentBlockDelta(message))
            }
            "content_block_stop" => {
                let message: ContentBlockStop = serde_json::from_value(value)?;
                Ok(StreamEvent::ContentBlockStop(message))
            }
            "message_delta" => {
                let message: MessageDelta = serde_json::from_value(value)?;
                Ok(StreamEvent::MessageDelta(message))
            }
            "message_stop" => Ok(StreamEvent::MessageStop),
            _ => Ok(StreamEvent::MessageStop),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageDelta {
    pub delta: MessageDeltaStop,
    pub usage: StreamUsageTokens,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageDeltaStop {
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamUsageTokens {
    pub output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBlockStart {
    pub index: i64,
    pub content_block: ContentBlock,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBlockDelta {
    pub index: i64,
    pub delta: ContentBlock,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBlockStop {
    pub index: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub kind: ContentBlockKind,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentBlockKind {
    Text,
    TextDelta,
}

impl fmt::Display for ContentBlockKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::TextDelta => write!(f, "text_delta"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{completion::message::RoleResponse, models::claude::ClaudeModel};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_deserialize_ping_event() {
        let raw = r#"{"type": "ping"}"#;
        let event: StreamEvent = raw.parse().unwrap();
        assert_eq!(event, StreamEvent::Ping);
    }

    #[test]
    fn should_deserialize_message_start_event() {
        let raw = r#"{"type":"message_start","message":{"id":"msg_0117mpmR7a2JEj2Z1G4jqjkf","type":"message","role":"assistant","model":"claude-3-5-sonnet-20240620","content":[],"stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":9,"output_tokens":3}}}"#;
        let event: StreamEvent = raw.parse().unwrap();

        if let StreamEvent::MessageStart { message } = event {
            assert_eq!(message.id, "msg_0117mpmR7a2JEj2Z1G4jqjkf");
            assert_eq!(message.role, RoleResponse::Assistant);
            assert_eq!(message.model, ClaudeModel::Claude35Sonnet);
            assert_eq!(message.content.is_empty(), true);
            assert_eq!(message.stop_reason, None);
            assert_eq!(message.stop_sequence, None);
            assert_eq!(message.usage.input_tokens, 9);
            assert_eq!(message.usage.output_tokens, 3);
        } else {
            panic!("Expected 'message_start' event");
        }
    }

    #[test]
    fn should_deserialize_content_block_start_event() {
        let raw =
            r#"{"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}"#;
        let event: StreamEvent = raw.parse().unwrap();

        if let StreamEvent::ContentBlockStart(content) = event {
            assert_eq!(content.index, 0);
            assert_eq!(content.content_block.kind, ContentBlockKind::Text);
            assert_eq!(content.content_block.text, "");
        } else {
            panic!("Expected 'content_block_start' event");
        }
    }

    #[test]
    fn should_deserialize_content_block_delta_event() {
        let raw = r#"{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello!"}}"#;
        let event: StreamEvent = raw.parse().unwrap();

        if let StreamEvent::ContentBlockDelta(content) = event {
            assert_eq!(content.index, 0);
            assert_eq!(content.delta.kind, ContentBlockKind::TextDelta);
            assert_eq!(content.delta.text, "Hello!");
        } else {
            panic!("Expected 'content_block_delta' event");
        }
    }

    #[test]
    fn should_deserialize_content_block_stop_event() {
        let raw = r#"{"type":"content_block_stop","index":0}"#;
        let event: StreamEvent = raw.parse().unwrap();

        if let StreamEvent::ContentBlockStop(content) = event {
            assert_eq!(content.index, 0);
        } else {
            panic!("Expected 'content_block_stop' event");
        }
    }

    #[test]
    fn should_deserialize_message_delta_event() {
        let raw = r#"{"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null},"usage":{"output_tokens":30}}"#;
        let event: StreamEvent = raw.parse().unwrap();

        if let StreamEvent::MessageDelta(content) = event {
            assert_eq!(content.delta.stop_reason, StopReason::EndTurn);
            assert_eq!(content.delta.stop_sequence, None);
            assert_eq!(content.usage.output_tokens, 30);
        } else {
            panic!("Expected 'message_delta' event");
        }
    }

    #[test]
    fn should_deserialize_message_stop_event() {
        let raw = r#"{"type":"message_stop"}"#;
        let event: StreamEvent = raw.parse().unwrap();
        assert_eq!(event, StreamEvent::MessageStop);
    }
}
