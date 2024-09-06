use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum GptModel {
    #[serde(rename(serialize = "gpt-4o"))]
    GPT4o,
    #[serde(rename(serialize = "gpt-4o-mini"))]
    GPT4oMini,
}

impl<'de> Deserialize<'de> for GptModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GptModelVisitor;

        impl<'de> serde::de::Visitor<'de> for GptModelVisitor {
            type Value = GptModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a GPT model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "gpt-4o" => Ok(GptModel::GPT4o),
                    "gpt-4o-mini" => Ok(GptModel::GPT4oMini),
                    // The order is important
                    _ if value.starts_with("gpt-4o-mini-") => Ok(GptModel::GPT4oMini),
                    _ if value.starts_with("gpt-4o-") => Ok(GptModel::GPT4o),
                    _ => Err(E::custom(format!("Unknown GPT model: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GptModelVisitor)
    }
}

impl FromStr for GptModel {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4o" => Ok(Self::GPT4o),
            _ => Err(crate::error::Error::ModelNotSupported(s.to_string())),
        }
    }
}

impl fmt::Display for GptModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GPT4o => write!(f, "gpt-4o"),
            Self::GPT4oMini => write!(f, "gpt-4o-mini"),
        }
    }
}
