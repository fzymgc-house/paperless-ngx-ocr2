//! Unit tests for CLI argument parsing

use paperless_ngx_ocr2::{
    cli::{Cli, CLIOutput, CLISuccessData, CLIErrorData},
    Error,
};
use clap::Parser;

#[test]
fn test_cli_parsing_required_arguments() {
    // Test with required file argument
    let args = vec!["paperless-ngx-ocr2", "--file", "test.pdf", "--api-key", "test-key"];
    let cli = Cli::try_parse_from(args).expect("Should parse required arguments");
    
    assert_eq!(cli.file, "test.pdf");
    assert_eq!(cli.api_key, Some("test-key".to_string()));
    assert!(!cli.json);
    assert!(!cli.verbose);
}

#[test]
fn test_cli_parsing_all_arguments() {
    let args = vec![
        "paperless-ngx-ocr2",
        "--file", "document.pdf",
        "--api-key", "sk-test123",
        "--api-base-url", "https://custom.api.url",
        "--json",
        "--verbose"
    ];
    
    let cli = Cli::try_parse_from(args).expect("Should parse all arguments");
    
    assert_eq!(cli.file, "document.pdf");
    assert_eq!(cli.api_key, Some("sk-test123".to_string()));
    assert_eq!(cli.api_base_url, Some("https://custom.api.url".to_string()));
    assert!(cli.json);
    assert!(cli.verbose);
}

#[test]
fn test_cli_parsing_missing_required_file() {
    let args = vec!["paperless-ngx-ocr2", "--api-key", "test-key"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err());
}

#[test]
fn test_cli_parsing_environment_variables() {
    std::env::set_var("PAPERLESS_OCR_API_KEY", "env-api-key");
    std::env::set_var("PAPERLESS_OCR_API_BASE_URL", "https://env.api.url");
    
    let args = vec!["paperless-ngx-ocr2", "--file", "test.pdf"];
    let cli = Cli::try_parse_from(args).expect("Should parse with env vars");
    
    assert_eq!(cli.file, "test.pdf");
    assert_eq!(cli.api_key, Some("env-api-key".to_string()));
    assert_eq!(cli.api_base_url, Some("https://env.api.url".to_string()));
    
    // Cleanup
    std::env::remove_var("PAPERLESS_OCR_API_KEY");
    std::env::remove_var("PAPERLESS_OCR_API_BASE_URL");
}

#[test]
fn test_cli_validation_valid_arguments() {
    let cli = Cli {
        file: "test.pdf".to_string(),
        api_key: Some("sk-test123".to_string()),
        api_base_url: Some("https://api.mistral.ai".to_string()),
        json: false,
        verbose: false,
    };
    
    assert!(cli.validate().is_ok());
}

#[test]
fn test_cli_validation_empty_file() {
    let cli = Cli {
        file: "".to_string(),
        api_key: Some("sk-test123".to_string()),
        api_base_url: None,
        json: false,
        verbose: false,
    };
    
    assert!(cli.validate().is_err());
    assert!(matches!(cli.validate().unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_cli_validation_empty_api_key() {
    let cli = Cli {
        file: "test.pdf".to_string(),
        api_key: Some("".to_string()),
        api_base_url: None,
        json: false,
        verbose: false,
    };
    
    assert!(cli.validate().is_err());
    assert!(matches!(cli.validate().unwrap_err(), Error::Config(_)));
}

#[test]
fn test_cli_validation_empty_api_base_url() {
    let cli = Cli {
        file: "test.pdf".to_string(),
        api_key: Some("sk-test123".to_string()),
        api_base_url: Some("".to_string()),
        json: false,
        verbose: false,
    };
    
    assert!(cli.validate().is_err());
    assert!(matches!(cli.validate().unwrap_err(), Error::Config(_)));
}

#[test]
fn test_cli_output_structure_validation() {
    // Valid success output
    let success_output = CLIOutput {
        success: true,
        data: Some(CLISuccessData {
            extracted_text: "Sample text".to_string(),
            file_name: "test.pdf".to_string(),
            file_size: 1024,
            processing_time_ms: 2000,
            confidence: Some(0.95),
        }),
        error: None,
    };
    assert!(success_output.validate().is_ok());
    
    // Valid error output
    let error_output = CLIOutput {
        success: false,
        data: None,
        error: Some(CLIErrorData {
            error_type: "validation".to_string(),
            message: "Test error".to_string(),
            details: None,
        }),
    };
    assert!(error_output.validate().is_ok());
    
    // Invalid output (both data and error)
    let invalid_output = CLIOutput {
        success: true,
        data: Some(CLISuccessData {
            extracted_text: "Text".to_string(),
            file_name: "test.pdf".to_string(),
            file_size: 1024,
            processing_time_ms: 1000,
            confidence: None,
        }),
        error: Some(CLIErrorData {
            error_type: "internal".to_string(),
            message: "Should not have both".to_string(),
            details: None,
        }),
    };
    assert!(invalid_output.validate().is_err());
}

#[test]
fn test_cli_output_serialization() {
    let success_output = CLIOutput {
        success: true,
        data: Some(CLISuccessData {
            extracted_text: "Test content".to_string(),
            file_name: "document.pdf".to_string(),
            file_size: 2048,
            processing_time_ms: 1500,
            confidence: Some(0.87),
        }),
        error: None,
    };
    
    let json_str = serde_json::to_string(&success_output).expect("Should serialize to JSON");
    let deserialized: CLIOutput = serde_json::from_str(&json_str).expect("Should deserialize from JSON");
    
    assert_eq!(deserialized.success, true);
    assert!(deserialized.data.is_some());
    assert!(deserialized.error.is_none());
    
    let data = deserialized.data.unwrap();
    assert_eq!(data.extracted_text, "Test content");
    assert_eq!(data.file_name, "document.pdf");
    assert_eq!(data.file_size, 2048);
    assert_eq!(data.processing_time_ms, 1500);
    assert_eq!(data.confidence, Some(0.87));
}
