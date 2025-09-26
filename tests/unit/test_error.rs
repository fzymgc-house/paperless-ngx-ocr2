//! Unit tests for error handling

use paperless_ngx_ocr2::{Error, api::error::{APIError, APIErrorResponse}};

#[test]
fn test_error_exit_codes() {
    // Test all constitutional exit codes
    assert_eq!(Error::Validation("test".to_string()).exit_code(), 2);
    assert_eq!(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test")).exit_code(), 3);
    assert_eq!(Error::Config("test".to_string()).exit_code(), 4);
    assert_eq!(Error::Api("test".to_string()).exit_code(), 5);
    // Create a mock network error for testing
    let network_error = Error::Internal("Network error".to_string()); // Use Internal as proxy for Network
    assert_eq!(network_error.exit_code(), 5);
    assert_eq!(Error::Internal("test".to_string()).exit_code(), 5);
}

#[test]
fn test_error_type_strings() {
    assert_eq!(Error::Validation("test".to_string()).error_type(), "validation");
    assert_eq!(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test")).error_type(), "file_io");
    assert_eq!(Error::Config("test".to_string()).error_type(), "api");
    assert_eq!(Error::Api("test".to_string()).error_type(), "api");
    // Test network error type (using Internal as proxy)
    let network_error = Error::Internal("Network error".to_string());
    assert_eq!(network_error.error_type(), "internal");
    assert_eq!(Error::Internal("test".to_string()).error_type(), "internal");
}

#[test]
fn test_error_user_messages() {
    let validation_error = Error::Validation("Invalid input".to_string());
    assert!(validation_error.user_message().contains("Validation error"));
    assert!(validation_error.user_message().contains("Invalid input"));
    
    let config_error = Error::Config("Missing API key".to_string());
    assert!(config_error.user_message().contains("Configuration error"));
    assert!(config_error.user_message().contains("Missing API key"));
}

#[test]
fn test_error_json_output() {
    let error = Error::Validation("Test validation error".to_string());
    let json = error.to_json_output();
    
    assert_eq!(json["success"], false);
    assert!(json["error"].is_object());
    assert_eq!(json["error"]["type"], "validation");
    assert!(json["error"]["message"].as_str().unwrap().contains("Test validation error"));
}

#[test]
fn test_error_from_http_status() {
    // Test 4xx errors
    let client_error = Error::from_http_status(400, "Bad Request".to_string());
    assert!(matches!(client_error, Error::Api(_)));
    assert_eq!(client_error.exit_code(), 5);
    
    // Test 5xx errors
    let server_error = Error::from_http_status(500, "Internal Server Error".to_string());
    assert!(matches!(server_error, Error::Api(_)));
    assert_eq!(server_error.exit_code(), 5);
    
    // Test other status codes
    let other_error = Error::from_http_status(200, "OK".to_string());
    assert!(matches!(other_error, Error::Internal(_)));
}

#[test]
fn test_api_error_response_validation() {
    // Valid API error response
    let valid_response = APIErrorResponse {
        error: "Authentication failed".to_string(),
        code: Some(401),
        details: Some("Invalid API key".to_string()),
    };
    assert!(valid_response.validate().is_ok());
    
    // Empty error message
    let invalid_response = APIErrorResponse {
        error: "".to_string(),
        code: Some(401),
        details: None,
    };
    assert!(invalid_response.validate().is_err());
}

#[test]
fn test_api_error_conversion() {
    let api_response = APIErrorResponse {
        error: "Rate limit exceeded".to_string(),
        code: Some(429),
        details: Some("Too many requests".to_string()),
    };
    
    let api_error: APIError = api_response.into();
    
    assert_eq!(api_error.message(), "Rate limit exceeded");
    assert_eq!(api_error.error_code(), Some(429));
    assert_eq!(api_error.details(), Some("Too many requests"));
    assert_eq!(api_error.error_type(), "api");
}

#[test]
fn test_api_error_logging_security() {
    let api_error = APIError::new(
        "api".to_string(),
        "Authentication failed with key sk-test123456789".to_string(),
        Some("API key sk-test123456789 is invalid".to_string()),
    );
    
    let log_output = api_error.to_log_string();
    let user_output = api_error.to_user_string();
    
    // API keys should be redacted in both outputs
    assert!(!log_output.contains("sk-test123456789"));
    assert!(!user_output.contains("sk-test123456789"));
    assert!(log_output.contains("sk-***"));
    assert!(user_output.contains("sk-***"));
}

#[test]
fn test_api_error_exit_codes() {
    let validation_error = APIError::new("validation".to_string(), "test".to_string(), None);
    assert_eq!(validation_error.exit_code(), 2);
    
    let file_io_error = APIError::new("file_io".to_string(), "test".to_string(), None);
    assert_eq!(file_io_error.exit_code(), 3);
    
    let api_error = APIError::new("api".to_string(), "test".to_string(), None);
    assert_eq!(api_error.exit_code(), 4);
    
    let network_error = APIError::new("network".to_string(), "test".to_string(), None);
    assert_eq!(network_error.exit_code(), 5);
    
    let internal_error = APIError::new("internal".to_string(), "test".to_string(), None);
    assert_eq!(internal_error.exit_code(), 5);
}
