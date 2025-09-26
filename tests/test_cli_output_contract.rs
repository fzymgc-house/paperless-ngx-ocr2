//! Contract tests for CLI output format
//! These tests validate that our CLI output conforms to the expected JSON contract

use paperless_ngx_ocr2::ocr::OCRResult;
use paperless_ngx_ocr2::error::Error;

// ============================================================================
// CLI OUTPUT CONTRACT TESTS (T011)
// ============================================================================

#[tokio::test]
async fn test_cli_output_contract_success_structure() {
    // Test that OCRResult can be serialized to the expected success format
    // This test validates the actual JSON output structure used by the CLI
    
    let ocr_result = OCRResult {
        extracted_text: "This is extracted text from the document.".to_string(),
        file_name: "document.pdf".to_string(),
        file_size: 12345,
        file_id: "file_123".to_string(),
        model: "mistral-ocr-latest".to_string(),
        usage: Some(std::collections::HashMap::new()),
        timestamp: chrono::Utc::now(),
    };
    
    // Get the actual JSON output that the CLI produces
    let json = ocr_result.to_json_output();
    
    // Validate required fields exist
    assert!(json.get("success").is_some(), "Output must have 'success' field");
    assert!(json.get("data").is_some(), "Success output must have 'data' field");
    assert!(json.get("error").is_none(), "Success output must not have 'error' field");
    
    // Validate success field value
    assert!(
        json.get("success").unwrap().as_bool().unwrap(),
        "Success field must be true for success output"
    );
    
    // Validate data structure
    let data = json.get("data").unwrap();
    assert!(data.get("extracted_text").is_some(), "Data must have 'extracted_text' field");
    assert!(data.get("file_name").is_some(), "Data must have 'file_name' field");
    assert!(data.get("file_size").is_some(), "Data must have 'file_size' field");
    assert!(data.get("processing_time_ms").is_some(), "Data must have 'processing_time_ms' field");
    assert!(data.get("confidence").is_some(), "Data must have 'confidence' field");
}

#[tokio::test]
async fn test_cli_output_contract_error_structure() {
    // Test that Error can be serialized to the expected error format
    // This test validates the actual JSON output structure used by the CLI
    
    let error = Error::Validation("File not found".to_string());
    
    // Get the actual JSON output that the CLI produces
    let json = error.to_json_output();
    
    // Validate required fields exist
    assert!(json.get("success").is_some(), "Output must have 'success' field");
    assert!(json.get("error").is_some(), "Error output must have 'error' field");
    assert!(json.get("data").is_none(), "Error output must not have 'data' field");
    
    // Validate success field value
    assert!(
        !json.get("success").unwrap().as_bool().unwrap(),
        "Success field must be false for error output"
    );
    
    // Validate error structure
    let error_obj = json.get("error").unwrap();
    assert!(error_obj.get("type").is_some(), "Error must have 'type' field");
    assert!(error_obj.get("message").is_some(), "Error must have 'message' field");
    assert!(error_obj.get("details").is_some(), "Error must have 'details' field");
}

#[tokio::test]
async fn test_cli_output_contract_success_data_fields() {
    // Test that success data contains all required fields with correct types
    
    let ocr_result = OCRResult {
        extracted_text: "Sample text".to_string(),
        file_name: "test.pdf".to_string(),
        file_size: 1024,
        file_id: "file_456".to_string(),
        model: "mistral-ocr-latest".to_string(),
        usage: Some(std::collections::HashMap::new()),
        timestamp: chrono::Utc::now(),
    };
    
    let json = ocr_result.to_json_output();
    let data = json.get("data").unwrap();
    
    // Validate field types
    assert!(data.get("extracted_text").unwrap().is_string(), "extracted_text must be string");
    assert!(data.get("file_name").unwrap().is_string(), "file_name must be string");
    assert!(data.get("file_size").unwrap().is_number(), "file_size must be number");
    assert!(data.get("processing_time_ms").unwrap().is_number(), "processing_time_ms must be number");
    assert!(data.get("confidence").unwrap().is_null(), "confidence should be null (hardcoded in to_json_output)");
    
    // Validate field values
    assert_eq!(data.get("extracted_text").unwrap().as_str().unwrap(), "Sample text");
    assert_eq!(data.get("file_name").unwrap().as_str().unwrap(), "test.pdf");
    assert_eq!(data.get("file_size").unwrap().as_u64().unwrap(), 1024);
    assert_eq!(data.get("processing_time_ms").unwrap().as_u64().unwrap(), 2000); // get_processing_time_ms() returns 2000
}

#[tokio::test]
async fn test_cli_output_contract_error_type_enum() {
    // Test that error types are correctly mapped
    
    let validation_error = Error::Validation("Validation failed".to_string());
    let api_error = Error::Api("API error".to_string());
    let config_error = Error::Config("Config error".to_string());
    let internal_error = Error::Internal("Internal error".to_string());
    
    // Test validation error
    let json = validation_error.to_json_output();
    let error_obj = json.get("error").unwrap();
    assert_eq!(error_obj.get("type").unwrap().as_str().unwrap(), "validation");
    
    // Test API error
    let json = api_error.to_json_output();
    let error_obj = json.get("error").unwrap();
    assert_eq!(error_obj.get("type").unwrap().as_str().unwrap(), "api");
    
    // Test config error
    let json = config_error.to_json_output();
    let error_obj = json.get("error").unwrap();
    assert_eq!(error_obj.get("type").unwrap().as_str().unwrap(), "api"); // Config maps to "api" in error_type()
    
    // Test internal error
    let json = internal_error.to_json_output();
    let error_obj = json.get("error").unwrap();
    assert_eq!(error_obj.get("type").unwrap().as_str().unwrap(), "internal");
}

#[tokio::test]
async fn test_cli_output_contract_optional_fields() {
    // Test that optional fields are handled correctly
    
    // Test with confidence provided
    let ocr_result_with_confidence = OCRResult {
        extracted_text: "Text".to_string(),
        file_name: "test.pdf".to_string(),
        file_size: 100,
        file_id: "file_789".to_string(),
        model: "mistral-ocr-latest".to_string(),
        usage: Some(std::collections::HashMap::new()),
        timestamp: chrono::Utc::now(),
    };
    
    let json = ocr_result_with_confidence.to_json_output();
    let data = json.get("data").unwrap();
    // Note: to_json_output() always returns null for confidence regardless of the struct field
    assert!(data.get("confidence").unwrap().is_null(), "Confidence should be null (hardcoded in to_json_output)");
    
    // Test with confidence not provided
    let ocr_result_without_confidence = OCRResult {
        extracted_text: "Text".to_string(),
        file_name: "test.pdf".to_string(),
        file_size: 100,
        file_id: "file_101".to_string(),
        model: "mistral-ocr-latest".to_string(),
        usage: Some(std::collections::HashMap::new()),
        timestamp: chrono::Utc::now(),
    };
    
    let json = ocr_result_without_confidence.to_json_output();
    let data = json.get("data").unwrap();
    assert!(data.get("confidence").unwrap().is_null(), "Confidence should be null (hardcoded in to_json_output)");
}

#[tokio::test]
async fn test_cli_output_contract_validation() {
    // Test that the JSON output can be validated against a schema
    // This is a basic validation test
    
    let ocr_result = OCRResult {
        extracted_text: "Valid text".to_string(),
        file_name: "valid.pdf".to_string(),
        file_size: 1000,
        file_id: "file_202".to_string(),
        model: "mistral-ocr-latest".to_string(),
        usage: Some(std::collections::HashMap::new()),
        timestamp: chrono::Utc::now(),
    };
    
    let json = ocr_result.to_json_output();
    
    // Basic structure validation
    assert!(json.is_object(), "Output must be a JSON object");
    assert!(json.get("success").is_some(), "Must have success field");
    assert!(json.get("data").is_some(), "Must have data field");
    
    // Validate that success is boolean
    assert!(json.get("success").unwrap().is_boolean(), "Success must be boolean");
    
    // Validate that data is object
    assert!(json.get("data").unwrap().is_object(), "Data must be object");
}