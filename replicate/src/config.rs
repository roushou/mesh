use crate::error::Error;

const DEFAULT_API_BASE_URL: &str = "https://api.replicate.com/v1/";
const API_KEY_ENV_VAR: &str = "REPLICATE_API_KEY";

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_API_BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn from_env() -> Result<Self, Error> {
        let api_key =
            std::env::var(API_KEY_ENV_VAR).map_err(|_| Error::MissingApiKey(API_KEY_ENV_VAR))?;
        Ok(Self::new(api_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_use_default_values() {
        let api_key = "replicate-api-key";
        let config = Config::new(api_key);

        assert_eq!(config.api_key, api_key);
        assert_eq!(config.base_url, DEFAULT_API_BASE_URL);
    }

    #[test]
    fn should_set_custom_url() {
        let api_key = "replicate-api-key";

        let config = Config::new(api_key).with_base_url("https://custom-api.replicate.com");
        assert_eq!(config.base_url, "https://custom-api.replicate.com");
    }
}
