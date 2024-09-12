use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum O1 {
    O1Preview(Option<String>),
    O1Mini(Option<String>),
}

impl Display for O1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            O1::O1Preview(None) => write!(f, "o1-preview"),
            O1::O1Preview(Some(date)) => write!(f, "o1-preview-{}", date),
            O1::O1Mini(None) => write!(f, "o1-mini"),
            O1::O1Mini(Some(date)) => write!(f, "o1-mini-{}", date),
        }
    }
}

impl std::str::FromStr for O1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "o1-preview" => Ok(O1::O1Preview(None)),
            "o1-mini" => Ok(O1::O1Mini(None)),
            _ if s.starts_with("o1-preview-") => {
                let version = s
                    .strip_prefix("o1-preview-")
                    .ok_or_else(|| format!("Invalid model version {}", s))?;
                Ok(O1::O1Preview(Some(version.to_string())))
            }
            _ if s.starts_with("o1-mini-") => {
                let version = s
                    .strip_prefix("o1-mini-")
                    .ok_or_else(|| format!("Invalid model version {}", s))?;
                Ok(O1::O1Mini(Some(version.to_string())))
            }
            _ => Err(format!("Unknown GPT model: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_display() {
        assert_eq!(O1::O1Preview(None).to_string(), "o1-preview");
        assert_eq!(
            O1::O1Preview(Some("2024-09-12".to_string())).to_string(),
            "o1-preview-2024-09-12"
        );
        assert_eq!(O1::O1Mini(None).to_string(), "o1-mini");
        assert_eq!(
            O1::O1Mini(Some("2024-09-12".to_string())).to_string(),
            "o1-mini-2024-09-12"
        );
    }

    #[test]
    fn should_parse_str() {
        assert_eq!(O1::from_str("o1-preview"), Ok(O1::O1Preview(None)));
        assert_eq!(
            O1::from_str("o1-preview-2024-09-12"),
            Ok(O1::O1Preview(Some("2024-09-12".to_string())))
        );
        assert_eq!(
            O1::from_str("o1-preview-extra-part"),
            Ok(O1::O1Preview(Some("extra-part".to_string())))
        );

        assert_eq!(O1::from_str("o1-mini"), Ok(O1::O1Mini(None)));
        assert_eq!(
            O1::from_str("o1-mini-2024-09-12"),
            Ok(O1::O1Mini(Some("2024-09-12".to_string())))
        );

        assert!(O1::from_str("invalid-model").is_err());
    }
}
