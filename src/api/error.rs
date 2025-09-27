//! API error handling structures

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// API error response from Mistral AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIErrorResponse {
    pub error: String,
    pub code: Option<i32>,
    pub details: Option<String>,
}

impl APIErrorResponse {
    /// Validate API error response
    pub fn validate(&self) -> Result<()> {
        if self.error.is_empty() {
            return Err(Error::Validation("Error message cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl From<APIErrorResponse> for APIError {
    fn from(response: APIErrorResponse) -> Self {
        APIError {
            error_type: match response.code {
                Some(400) => "validation".to_string(),
                Some(401..=404) => "api".to_string(),
                Some(429) => "network".to_string(),
                Some(500) => "api".to_string(),
                Some(502..=503) => "network".to_string(),
                _ => "api".to_string(),
            },
            message: response.error,
            details: response.details,
            code: response.code,
        }
    }
}

/// Internal API error structure
#[derive(Debug, Clone)]
pub struct APIError {
    error_type: String,
    message: String,
    details: Option<String>,
    code: Option<i32>,
}

impl APIError {
    /// Create a new API error
    pub fn new(error_type: String, message: String, details: Option<String>) -> Self {
        Self { error_type, message, details, code: None }
    }

    /// Get error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get error code
    pub fn error_code(&self) -> Option<i32> {
        self.code
    }

    /// Get error details
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Get error type
    pub fn error_type(&self) -> &str {
        &self.error_type
    }

    /// Get exit code based on error type
    pub fn exit_code(&self) -> i32 {
        match self.error_type.as_str() {
            "validation" => 2,
            "file_io" => 3,
            "api" => 4,
            "network" => 5,
            "internal" => 5,
            _ => 5,
        }
    }

    /// Format for logging (with API key redaction)
    pub fn to_log_string(&self) -> String {
        let mut log_msg = format!("[{}] {}", self.error_type, self.message);

        // Redact API keys in log output (security requirement)
        if log_msg.contains("sk-") {
            log_msg = log_msg.replace("sk-", "***redacted***");
        }

        if let Some(ref details) = self.details {
            let mut redacted_details = details.clone();
            if redacted_details.contains("sk-") {
                redacted_details = redacted_details.replace("sk-", "***redacted***");
            }
            log_msg.push_str(&format!(" ({})", redacted_details));
        }

        log_msg
    }

    /// Format for user display (without sensitive details)
    pub fn to_user_string(&self) -> String {
        let mut user_msg = self.message.clone();

        // Remove any API key details from user message
        if user_msg.contains("sk-") {
            user_msg = user_msg.replace("sk-", "***");
        }

        user_msg
    }
}
