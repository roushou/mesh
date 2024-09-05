use crate::{
    client::{AnthropicVersion, ApiVersion},
    error::AnthropicError,
};

const DEFAULT_API_BASE_URL: &str = "https://api.anthropic.com";
const API_KEY_ENV_VAR: &str = "ANTHROPIC_API_KEY";

#[derive(Debug, Clone)]
pub struct Config {
    pub anthropic_version: AnthropicVersion,
    pub api_key: String,
    pub api_version: ApiVersion,
    pub base_url: String,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            anthropic_version: AnthropicVersion::default(),
            api_key: api_key.into(),
            api_version: ApiVersion::default(),
            base_url: DEFAULT_API_BASE_URL.to_string(),
        }
    }

    pub fn with_anthropic_version(mut self, version: AnthropicVersion) -> Self {
        self.anthropic_version = version;
        self
    }

    pub fn with_api_version(mut self, version: ApiVersion) -> Self {
        self.api_version = version;
        self
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn from_env() -> Result<Self, AnthropicError> {
        let api_key = std::env::var(API_KEY_ENV_VAR)
            .map_err(|_| AnthropicError::MissingApiKey(API_KEY_ENV_VAR))?;
        Ok(Self::new(api_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_use_default_values() {
        let api_key = "anthropic-api-key";
        let config = Config::new(api_key);

        assert_eq!(config.anthropic_version, AnthropicVersion::default());
        assert_eq!(config.api_key, api_key);
        assert_eq!(config.api_version, ApiVersion::default());
        assert_eq!(config.base_url, DEFAULT_API_BASE_URL);
    }

    #[test]
    fn should_set_anthropic_version() {
        let api_key = "anthropic-api-key";

        let config = Config::new(api_key).with_anthropic_version(AnthropicVersion::Latest);
        assert_eq!(config.anthropic_version, AnthropicVersion::Latest);

        let config = Config::new(api_key).with_anthropic_version(AnthropicVersion::Initial);
        assert_eq!(config.anthropic_version, AnthropicVersion::Initial);
    }

    #[test]
    fn should_set_custom_url() {
        let api_key = "anthropic-api-key";

        let config = Config::new(api_key).with_base_url("https://custom.api.anthropic.com");
        assert_eq!(config.base_url, "https://custom.api.anthropic.com");
    }
}
