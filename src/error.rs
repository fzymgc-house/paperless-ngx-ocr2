//! Error types and handling

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Get the appropriate exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::Validation(_) => 2,
            Error::Io(_) => 3,
            Error::Config(_) => 4,
            Error::Api(_) | Error::Network(_) | Error::Internal(_) => 5,
        }
    }

    /// Get the error type as string for JSON output
    pub fn error_type(&self) -> &'static str {
        match self {
            Error::Validation(_) => "validation",
            Error::Io(_) => "file_io",
            Error::Config(_) => "api",
            Error::Api(_) => "api",
            Error::Network(_) => "network",
            Error::Internal(_) => "internal",
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Error::Validation(msg) => format!("Validation error: {}", msg),
            Error::Io(e) => format!("File error: {}", e),
            Error::Config(msg) => format!("Configuration error: {}", msg),
            Error::Api(msg) => format!("API error: {}", msg),
            Error::Network(e) => format!("Network error: {}", e),
            Error::Internal(msg) => format!("Internal error: {}", msg),
        }
    }

    /// Format error for JSON output
    pub fn to_json_output(&self) -> serde_json::Value {
        serde_json::json!({
            "success": false,
            "error": {
                "type": self.error_type(),
                "message": self.user_message(),
                "details": self.to_string()
            }
        })
    }

    /// Create API error from HTTP status code
    pub fn from_http_status(status: u16, message: String) -> Self {
        match status {
            400..=499 => Error::Validation(format!("Client error ({}): {}", status, message)),
            500..=599 => Error::Api(format!("Server error ({}): {}", status, message)),
            _ => Error::Internal(format!("Unexpected HTTP status ({}): {}", status, message)),
        }
    }
}
