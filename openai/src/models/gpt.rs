use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum GptModel {
    #[serde(rename(serialize = "gpt-4"))]
    GPT4,
    #[serde(rename(serialize = "gpt-4o"))]
    GPT4o,
    #[serde(rename(serialize = "gpt-4o-mini"))]
    GPT4oMini,
    #[serde(rename(serialize = "gpt-4-turbo"))]
    GPT4Turbo,
    #[serde(rename(serialize = "gpt-3.5-turbo"))]
    GPT35Turbo,
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
                    "gpt-4" => Ok(GptModel::GPT4),
                    "gpt-4o" => Ok(GptModel::GPT4o),
                    "gpt-4o-mini" => Ok(GptModel::GPT4oMini),
                    "gpt-4-turbo" => Ok(GptModel::GPT4Turbo),
                    "chatgpt-4o-latest" => Ok(GptModel::GPT4o),
                    // The order is important for correct matching
                    _ if value.starts_with("gpt-3.5-turbo") => Ok(GptModel::GPT35Turbo),
                    _ if value.starts_with("gpt-4-turbo-") => Ok(GptModel::GPT4Turbo),
                    _ if value.starts_with("gpt-4-") => Ok(GptModel::GPT4),
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
            "gpt-4" => Ok(GptModel::GPT4),
            "gpt-4o" => Ok(GptModel::GPT4o),
            "gpt-4o-mini" => Ok(GptModel::GPT4oMini),
            "gpt-4-turbo" => Ok(GptModel::GPT4Turbo),
            "chatgpt-4o-latest" => Ok(GptModel::GPT4o),
            _ if s.starts_with("gpt-3.5-turbo") => Ok(GptModel::GPT35Turbo),
            // The order is important for correct matching
            _ if s.starts_with("gpt-4o-mini-") => Ok(GptModel::GPT4oMini),
            _ if s.starts_with("gpt-4o-") => Ok(GptModel::GPT4o),
            _ if s.starts_with("gpt-4-turbo-") => Ok(GptModel::GPT4Turbo),
            _ if s.starts_with("gpt-4-") => Ok(GptModel::GPT4),
            _ => Err(crate::error::Error::ModelNotSupported(s.to_string())),
        }
    }
}

impl fmt::Display for GptModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GPT4 => write!(f, "gpt-4"),
            Self::GPT4o => write!(f, "gpt-4o"),
            Self::GPT4oMini => write!(f, "gpt-4o-mini"),
            Self::GPT4Turbo => write!(f, "gpt-4-turbo"),
            Self::GPT35Turbo => write!(f, "gpt-3.5-turbo"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::from_str;

    #[test]
    fn should_deserialize_gpt_models() {
        // Test exact matches
        assert_eq!(from_str::<GptModel>(r#""gpt-4""#).unwrap(), GptModel::GPT4);
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4o""#).unwrap(),
            GptModel::GPT4o
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4o-mini""#).unwrap(),
            GptModel::GPT4oMini
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4-turbo""#).unwrap(),
            GptModel::GPT4Turbo
        );
        assert_eq!(
            from_str::<GptModel>(r#""chatgpt-4o-latest""#).unwrap(),
            GptModel::GPT4o
        );

        // Test prefix matches
        assert_eq!(
            from_str::<GptModel>(r#""gpt-3.5-turbo""#).unwrap(),
            GptModel::GPT35Turbo
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-3.5-turbo-0125""#).unwrap(),
            GptModel::GPT35Turbo
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4-0125-preview""#).unwrap(),
            GptModel::GPT4
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4o-2024-05-13""#).unwrap(),
            GptModel::GPT4o
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4o-mini-1234""#).unwrap(),
            GptModel::GPT4oMini
        );
        assert_eq!(
            from_str::<GptModel>(r#""gpt-4-turbo-2024-04-09""#).unwrap(),
            GptModel::GPT4Turbo
        );

        // Test error case
        assert!(from_str::<GptModel>(r#""unknown-model""#).is_err());
    }

    #[test]
    fn test_gpt_model_from_str() {
        // Test exact matches
        assert_eq!("gpt-4".parse::<GptModel>().unwrap(), GptModel::GPT4);
        assert_eq!("gpt-4o".parse::<GptModel>().unwrap(), GptModel::GPT4o);
        assert_eq!(
            "gpt-4o-mini".parse::<GptModel>().unwrap(),
            GptModel::GPT4oMini
        );
        assert_eq!(
            "gpt-4-turbo".parse::<GptModel>().unwrap(),
            GptModel::GPT4Turbo
        );
        assert_eq!(
            "chatgpt-4o-latest".parse::<GptModel>().unwrap(),
            GptModel::GPT4o
        );

        // Test prefix matches
        assert_eq!(
            "gpt-3.5-turbo".parse::<GptModel>().unwrap(),
            GptModel::GPT35Turbo
        );
        assert_eq!(
            "gpt-3.5-turbo-0125".parse::<GptModel>().unwrap(),
            GptModel::GPT35Turbo
        );
        assert_eq!("gpt-4-9012".parse::<GptModel>().unwrap(), GptModel::GPT4);
        assert_eq!("gpt-4o-5678".parse::<GptModel>().unwrap(), GptModel::GPT4o);
        assert_eq!(
            "gpt-4o-mini-1234".parse::<GptModel>().unwrap(),
            GptModel::GPT4oMini
        );
        assert_eq!(
            "gpt-4-turbo-2024-04-09".parse::<GptModel>().unwrap(),
            GptModel::GPT4Turbo
        );

        // Test error case
        assert!("unknown-model".parse::<GptModel>().is_err());
    }

    #[test]
    fn should_display_gpt_models() {
        assert_eq!(GptModel::GPT35Turbo.to_string(), "gpt-3.5-turbo");
        assert_eq!(GptModel::GPT4.to_string(), "gpt-4");
        assert_eq!(GptModel::GPT4o.to_string(), "gpt-4o");
        assert_eq!(GptModel::GPT4oMini.to_string(), "gpt-4o-mini");
        assert_eq!(GptModel::GPT4Turbo.to_string(), "gpt-4-turbo");
    }
}
