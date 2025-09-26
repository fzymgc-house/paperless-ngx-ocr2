//! Authentication handling for Mistral AI API
//!
//! This module handles authentication for Mistral AI APIs.
//! Documentation: https://docs.mistral.ai/api/#authentication
//!
//! Authentication is performed using Bearer tokens in the Authorization header.
//! API keys can be obtained from the Mistral AI dashboard.

use crate::credentials::APICredentials;
use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, ACCEPT_ENCODING};

/// Authentication handler for Mistral AI API
#[derive(Debug, Clone)]
pub struct AuthHandler {
    credentials: APICredentials,
}

impl AuthHandler {
    /// Create a new authentication handler
    pub fn new(credentials: APICredentials) -> Self {
        Self { credentials }
    }

    /// Get authorization headers for API requests with compression support
    pub fn get_auth_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // Add authorization header
        let auth_value = HeaderValue::from_str(&self.credentials.get_auth_header())
            .map_err(|e| Error::Config(format!("Invalid API key format: {}", e)))?;
        
        headers.insert(AUTHORIZATION, auth_value);

        // Add content type for JSON requests
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add compression support headers
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

        Ok(headers)
    }

    /// Get headers for multipart file upload with compression support
    pub fn get_multipart_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // Add authorization header
        let auth_value = HeaderValue::from_str(&self.credentials.get_auth_header())
            .map_err(|e| Error::Config(format!("Invalid API key format: {}", e)))?;
        
        headers.insert(AUTHORIZATION, auth_value);

        // Add compression support headers
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

        // Note: Content-Type for multipart is set automatically by reqwest

        Ok(headers)
    }

    /// Validate API key format
    pub fn validate_api_key(&self) -> Result<()> {
        self.credentials.validate()
    }

    /// Get redacted key for logging
    pub fn redacted_key(&self) -> String {
        self.credentials.redacted_key()
    }

    /// Check if this is a Mistral AI endpoint
    pub fn is_mistral_endpoint(&self) -> bool {
        self.credentials.is_mistral_api()
    }
}
