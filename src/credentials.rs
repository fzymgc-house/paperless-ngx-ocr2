//! API credentials entity and validation

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APICredentials {
    /// Mistral AI API key
    pub api_key: String,

    /// Mistral AI API base URL
    pub api_base_url: String,

    /// Whether credentials are valid
    pub is_valid: bool,
}

impl APICredentials {
    /// Create new API credentials
    pub fn new(api_key: String, api_base_url: String) -> Result<Self> {
        let mut credentials = Self { api_key, api_base_url, is_valid: false };

        credentials.validate()?;
        credentials.is_valid = true;

        Ok(credentials)
    }

    /// Validate credentials according to data model rules
    pub fn validate(&self) -> Result<()> {
        // Validate API key is not empty and doesn't contain whitespace
        if self.api_key.is_empty() {
            return Err(Error::Config("API key must not be empty".to_string()));
        }

        if self.api_key.contains(char::is_whitespace) {
            return Err(Error::Config("API key must not contain whitespace".to_string()));
        }

        // Validate API base URL
        let url = Url::parse(&self.api_base_url).map_err(|_| Error::Config("API base URL must be a valid URL".to_string()))?;

        // Ensure it's HTTPS
        if url.scheme() != "https" {
            return Err(Error::Config("API base URL must use HTTPS".to_string()));
        }

        // Validate it points to Mistral AI API
        if let Some(host) = url.host_str() {
            if !host.contains("mistral") && !host.contains("api.mistral.ai") {
                tracing::warn!("API base URL does not appear to be Mistral AI: {}", host);
            }
        }

        Ok(())
    }

    /// Get authorization header value
    pub fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    /// Check if credentials are for Mistral AI
    pub fn is_mistral_api(&self) -> bool {
        self.api_base_url.contains("mistral")
    }

    /// Redact API key for logging (security requirement)
    pub fn redacted_key(&self) -> String {
        if self.api_key.len() > 8 {
            format!("{}***", &self.api_key[..4])
        } else {
            "***".to_string()
        }
    }

    /// Create credentials from configuration
    pub fn from_config(config: &crate::Config) -> Result<Self> {
        Self::new(config.api_key.clone(), config.api_base_url.clone())
    }
}
