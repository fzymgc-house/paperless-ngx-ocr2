//! Contract validation utilities
//!
//! This module provides utilities for validating API contracts and CLI output
//! against expected schemas and formats.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashSet;

/// Validates CLI output against the expected JSON contract
pub fn validate_cli_output_contract(json: &Value) -> Result<(), String> {
    if let Some(success) = json.get("success") {
        if !success.is_boolean() {
            return Err("'success' field must be a boolean".to_string());
        }

        if success.as_bool().unwrap_or(false) {
            validate_success_data(json)?;
        } else {
            validate_error_data(json)?;
        }
    } else {
        return Err("Missing required 'success' field".to_string());
    }

    Ok(())
}

/// Validates the success data structure
fn validate_success_data(json: &Value) -> Result<(), String> {
    let data = json
        .get("data")
        .ok_or("Missing 'data' field for success response")?;

    if !data.is_object() {
        return Err("'data' field must be an object".to_string());
    }

    // Validate required fields
    let required_fields = ["extracted_text", "file_name", "file_size"];
    for field in &required_fields {
        if data.get(field).is_none() {
            return Err(format!("Missing required field 'data.{}'", field));
        }
    }

    // Validate field types
    if !data.get("extracted_text").unwrap().is_string() {
        return Err("'data.extracted_text' must be a string".to_string());
    }

    if !data.get("file_name").unwrap().is_string() {
        return Err("'data.file_name' must be a string".to_string());
    }

    if !data.get("file_size").unwrap().is_number() {
        return Err("'data.file_size' must be a number".to_string());
    }

    // Validate optional fields
    if let Some(processing_time) = data.get("processing_time_ms") {
        if !processing_time.is_number() {
            return Err("'data.processing_time_ms' must be a number".to_string());
        }
    }

    if let Some(confidence) = data.get("confidence") {
        if !confidence.is_number() {
            return Err("'data.confidence' must be a number".to_string());
        }
        let conf_value = confidence.as_f64().unwrap_or(0.0);
        if !(0.0..=1.0).contains(&conf_value) {
            return Err("'data.confidence' must be between 0.0 and 1.0".to_string());
        }
    }

    Ok(())
}

/// Validates the error data structure
fn validate_error_data(json: &Value) -> Result<(), String> {
    let error = json
        .get("error")
        .ok_or("Missing 'error' field for error response")?;

    if !error.is_object() {
        return Err("'error' field must be an object".to_string());
    }

    // Validate required fields
    let required_fields = ["type", "message"];
    for field in &required_fields {
        if error.get(field).is_none() {
            return Err(format!("Missing required field 'error.{}'", field));
        }
    }

    // Validate error type enum
    let error_type = error.get("type").unwrap().as_str().unwrap_or("");
    let valid_types: HashSet<&str> = ["validation", "network", "api", "file_io", "internal"]
        .iter()
        .cloned()
        .collect();
    if !valid_types.contains(error_type) {
        return Err(format!(
            "Invalid error type '{}'. Must be one of: {:?}",
            error_type, valid_types
        ));
    }

    // Validate field types
    if !error.get("type").unwrap().is_string() {
        return Err("'error.type' must be a string".to_string());
    }

    if !error.get("message").unwrap().is_string() {
        return Err("'error.message' must be a string".to_string());
    }

    // Validate optional fields
    if let Some(details) = error.get("details") {
        if !details.is_string() {
            return Err("'error.details' must be a string".to_string());
        }
    }

    Ok(())
}

/// Validates API error response contract
pub fn validate_api_error_contract(json: &Value) -> Result<(), String> {
    // Check required fields
    if json.get("error").is_none() {
        return Err("Missing required 'error' field".to_string());
    }

    let error = json.get("error").unwrap();
    if !error.is_string() {
        return Err("'error' field must be a string".to_string());
    }

    // Validate optional fields
    if let Some(code) = json.get("code") {
        if !code.is_number() {
            return Err("'code' field must be a number".to_string());
        }
    }

    if let Some(details) = json.get("details") {
        if !details.is_string() {
            return Err("'details' field must be a string".to_string());
        }
    }

    Ok(())
}

/// Validates file upload request contract
pub fn validate_file_upload_request_contract(json: &Value) -> Result<(), String> {
    // Check required fields
    let required_fields = ["file_data", "filename", "purpose"];
    for field in &required_fields {
        if json.get(field).is_none() {
            return Err(format!("Missing required field '{}'", field));
        }
    }

    // Validate field types
    if !json.get("file_data").unwrap().is_array() {
        return Err("'file_data' field must be an array".to_string());
    }

    if !json.get("filename").unwrap().is_string() {
        return Err("'filename' field must be a string".to_string());
    }

    if !json.get("purpose").unwrap().is_string() {
        return Err("'purpose' field must be a string".to_string());
    }

    // Validate purpose enum
    let purpose = json.get("purpose").unwrap().as_str().unwrap_or("");
    if purpose != "ocr" {
        return Err(format!("Invalid purpose '{}'. Must be 'ocr'", purpose));
    }

    Ok(())
}

/// Validates file upload response contract
pub fn validate_file_upload_response_contract(json: &Value) -> Result<(), String> {
    // Check required fields
    let required_fields = ["id", "object", "bytes", "created_at", "filename", "purpose"];
    for field in &required_fields {
        if json.get(field).is_none() {
            return Err(format!("Missing required field '{}'", field));
        }
    }

    // Validate field types
    if !json.get("id").unwrap().is_string() {
        return Err("'id' field must be a string".to_string());
    }

    if !json.get("object").unwrap().is_string() {
        return Err("'object' field must be a string".to_string());
    }

    if !json.get("bytes").unwrap().is_number() {
        return Err("'bytes' field must be a number".to_string());
    }

    if !json.get("created_at").unwrap().is_number() {
        return Err("'created_at' field must be a number".to_string());
    }

    if !json.get("filename").unwrap().is_string() {
        return Err("'filename' field must be a string".to_string());
    }

    if !json.get("purpose").unwrap().is_string() {
        return Err("'purpose' field must be a string".to_string());
    }

    // Validate object enum
    let object = json.get("object").unwrap().as_str().unwrap_or("");
    if object != "file" {
        return Err(format!("Invalid object '{}'. Must be 'file'", object));
    }

    // Validate purpose enum
    let purpose = json.get("purpose").unwrap().as_str().unwrap_or("");
    if purpose != "ocr" {
        return Err(format!("Invalid purpose '{}'. Must be 'ocr'", purpose));
    }

    // Validate optional status field
    if let Some(status) = json.get("status") {
        if !status.is_string() {
            return Err("'status' field must be a string".to_string());
        }
        let status_value = status.as_str().unwrap_or("");
        let valid_statuses: HashSet<&str> = ["uploaded", "processing", "processed", "error"]
            .iter()
            .cloned()
            .collect();
        if !valid_statuses.contains(status_value) {
            return Err(format!(
                "Invalid status '{}'. Must be one of: {:?}",
                status_value, valid_statuses
            ));
        }
    }

    Ok(())
}

/// Validates OCR API request contract
pub fn validate_ocr_request_contract(json: &Value) -> Result<(), String> {
    // Check required fields
    let required_fields = ["model", "document"];
    for field in &required_fields {
        if json.get(field).is_none() {
            return Err(format!("Missing required field '{}'", field));
        }
    }

    // Validate model field
    let model = json.get("model").unwrap().as_str().unwrap_or("");
    if model != "mistral-ocr-latest" {
        return Err(format!(
            "Invalid model '{}'. Must be 'mistral-ocr-latest'",
            model
        ));
    }

    // Validate document structure
    let document = json.get("document").unwrap();
    if !document.is_object() {
        return Err("'document' field must be an object".to_string());
    }

    let doc_required_fields = ["type", "file_id"];
    for field in &doc_required_fields {
        if document.get(field).is_none() {
            return Err(format!("Missing required field 'document.{}'", field));
        }
    }

    // Validate document type
    let doc_type = document.get("type").unwrap().as_str().unwrap_or("");
    if doc_type != "file" {
        return Err(format!(
            "Invalid document type '{}'. Must be 'file'",
            doc_type
        ));
    }

    Ok(())
}

/// Validates OCR API response contract
pub fn validate_ocr_response_contract(json: &Value) -> Result<(), String> {
    // Check required fields
    let required_fields = ["id", "object", "created", "model", "choices"];
    for field in &required_fields {
        if json.get(field).is_none() {
            return Err(format!("Missing required field '{}'", field));
        }
    }

    // Validate field types
    if !json.get("id").unwrap().is_string() {
        return Err("'id' field must be a string".to_string());
    }

    if !json.get("object").unwrap().is_string() {
        return Err("'object' field must be a string".to_string());
    }

    if !json.get("created").unwrap().is_number() {
        return Err("'created' field must be a number".to_string());
    }

    if !json.get("model").unwrap().is_string() {
        return Err("'model' field must be a string".to_string());
    }

    if !json.get("choices").unwrap().is_array() {
        return Err("'choices' field must be an array".to_string());
    }

    // Validate object enum
    let object = json.get("object").unwrap().as_str().unwrap_or("");
    if object != "chat.completion" {
        return Err(format!(
            "Invalid object '{}'. Must be 'chat.completion'",
            object
        ));
    }

    // Validate choices array
    let choices = json.get("choices").unwrap().as_array().unwrap();
    if choices.is_empty() {
        return Err("'choices' array cannot be empty".to_string());
    }

    for (i, choice) in choices.iter().enumerate() {
        validate_choice_item(choice, i)?;
    }

    Ok(())
}

/// Validates a single choice item in the OCR response
fn validate_choice_item(choice: &Value, index: usize) -> Result<(), String> {
    if !choice.is_object() {
        return Err(format!("Choice {} must be an object", index));
    }

    // Check required fields
    let required_fields = ["index", "message", "finish_reason"];
    for field in &required_fields {
        if choice.get(field).is_none() {
            return Err(format!(
                "Missing required field 'choices[{}].{}'",
                index, field
            ));
        }
    }

    // Validate message structure
    let message = choice.get("message").unwrap();
    if !message.is_object() {
        return Err(format!("'choices[{}].message' must be an object", index));
    }

    let msg_required_fields = ["role", "content"];
    for field in &msg_required_fields {
        if message.get(field).is_none() {
            return Err(format!(
                "Missing required field 'choices[{}].message.{}'",
                index, field
            ));
        }
    }

    // Validate role
    let role = message.get("role").unwrap().as_str().unwrap_or("");
    if role != "assistant" {
        return Err(format!(
            "Invalid role '{}' in choice {}. Must be 'assistant'",
            role, index
        ));
    }

    // Validate finish_reason
    let finish_reason = choice.get("finish_reason").unwrap().as_str().unwrap_or("");
    let valid_reasons: HashSet<&str> = ["stop", "length", "content_filter"]
        .iter()
        .cloned()
        .collect();
    if !valid_reasons.contains(finish_reason) {
        return Err(format!(
            "Invalid finish_reason '{}' in choice {}. Must be one of: {:?}",
            finish_reason, index, valid_reasons
        ));
    }

    Ok(())
}

/// Validates a JSON string against a contract
pub fn validate_json_contract(json_str: &str, contract_type: ContractType) -> Result<(), String> {
    let json: Value = serde_json::from_str(json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

    match contract_type {
        ContractType::CliOutput => validate_cli_output_contract(&json),
        ContractType::ApiError => validate_api_error_contract(&json),
        ContractType::FileUploadRequest => validate_file_upload_request_contract(&json),
        ContractType::FileUploadResponse => validate_file_upload_response_contract(&json),
        ContractType::OcrRequest => validate_ocr_request_contract(&json),
        ContractType::OcrResponse => validate_ocr_response_contract(&json),
    }
}

/// Types of contracts that can be validated
#[derive(Debug, Clone, Copy)]
pub enum ContractType {
    CliOutput,
    ApiError,
    FileUploadRequest,
    FileUploadResponse,
    OcrRequest,
    OcrResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cli_output_success() {
        let json = serde_json::json!({
            "success": true,
            "data": {
                "extracted_text": "Hello World",
                "file_name": "test.pdf",
                "file_size": 1024,
                "processing_time_ms": 1500,
                "confidence": 0.95
            }
        });

        assert!(validate_cli_output_contract(&json).is_ok());
    }

    #[test]
    fn test_validate_cli_output_error() {
        let json = serde_json::json!({
            "success": false,
            "error": {
                "type": "validation",
                "message": "Invalid file format",
                "details": "File must be PDF or image"
            }
        });

        assert!(validate_cli_output_contract(&json).is_ok());
    }

    #[test]
    fn test_validate_cli_output_missing_success() {
        let json = serde_json::json!({
            "data": {
                "extracted_text": "Hello World",
                "file_name": "test.pdf",
                "file_size": 1024
            }
        });

        assert!(validate_cli_output_contract(&json).is_err());
    }

    #[test]
    fn test_validate_file_upload_request() {
        let json = serde_json::json!({
            "file_data": [1, 2, 3, 4],
            "filename": "test.pdf",
            "purpose": "ocr"
        });

        assert!(validate_file_upload_request_contract(&json).is_ok());
    }

    #[test]
    fn test_validate_file_upload_response() {
        let json = serde_json::json!({
            "id": "file-123",
            "object": "file",
            "bytes": 1024,
            "created_at": 1640995200,
            "filename": "test.pdf",
            "purpose": "ocr",
            "status": "uploaded"
        });

        assert!(validate_file_upload_response_contract(&json).is_ok());
    }

    #[test]
    fn test_validate_ocr_request() {
        let json = serde_json::json!({
            "model": "mistral-ocr-latest",
            "document": {
                "type": "file",
                "file_id": "file-123"
            }
        });

        assert!(validate_ocr_request_contract(&json).is_ok());
    }

    #[test]
    fn test_validate_ocr_response() {
        let json = serde_json::json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1640995200,
            "model": "mistral-ocr-latest",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Extracted text"
                },
                "finish_reason": "stop"
            }]
        });

        assert!(validate_ocr_response_contract(&json).is_ok());
    }
}
