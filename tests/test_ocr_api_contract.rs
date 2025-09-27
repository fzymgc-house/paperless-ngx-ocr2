//! Contract tests for Mistral AI OCR API
//! These tests validate that our OCR API requests and responses conform to the expected contracts

use paperless_ngx_ocr2::api::ocr::{Dimensions, OCRRequest, OCRResponse, Page, UsageInfo};

// ============================================================================
// OCR API REQUEST CONTRACT TESTS (T009)
// ============================================================================

#[tokio::test]
async fn test_ocr_request_contract_structure() {
    // Test that OCRRequest can be serialized to the expected format

    let request = OCRRequest::new("file-abc123".to_string());

    // Serialize to JSON to validate structure
    let json = serde_json::to_value(&request).expect("Should serialize to JSON");

    // Validate required fields exist
    assert!(json.get("model").is_some(), "Request must have 'model' field");
    assert!(json.get("document").is_some(), "Request must have 'document' field");

    // Validate model field value
    assert_eq!(json.get("model").unwrap().as_str().unwrap(), "mistral-ocr-latest", "Model must be 'mistral-ocr-latest'");

    // Validate document structure
    let document = json.get("document").unwrap();
    assert!(document.get("type").is_some(), "Document must have 'type' field");
    assert!(document.get("file_id").is_some(), "Document must have 'file_id' field");

    assert_eq!(document.get("type").unwrap().as_str().unwrap(), "file", "Document type must be 'file'");

    assert_eq!(document.get("file_id").unwrap().as_str().unwrap(), "file-abc123", "File ID must match expected value");
}

#[tokio::test]
async fn test_ocr_request_contract_required_fields() {
    // Test that all required fields are present according to the JSON schema

    let request = OCRRequest::new("file-xyz789".to_string());

    let json = serde_json::to_value(&request).expect("Should serialize to JSON");

    // Validate JSON schema compliance - model and document are required fields
    assert!(json.get("model").is_some(), "Required field 'model' missing from request");

    assert!(json.get("document").is_some(), "Required field 'document' missing from request");

    let model = json.get("model").unwrap().as_str().unwrap();
    assert!(!model.is_empty(), "Model must not be empty, got '{}'", model);

    let document = json.get("document").unwrap();
    let file_id = document.get("file_id").unwrap().as_str().unwrap();
    assert!(!file_id.is_empty(), "File ID must not be empty, got '{}'", file_id);
}

#[tokio::test]
async fn test_ocr_request_contract_validation() {
    // Test that OCRRequest validates according to contract

    // Test with valid file_id
    let valid_request = OCRRequest::new("file-valid123".to_string());

    assert!(valid_request.validate().is_ok(), "Valid file_id should pass validation");

    // Test with empty file_id (should be invalid)
    let invalid_request = OCRRequest::new("".to_string());

    assert!(invalid_request.validate().is_err(), "Empty file_id should fail validation");
}

// ============================================================================
// OCR API RESPONSE CONTRACT TESTS (T010)
// ============================================================================

#[tokio::test]
async fn test_ocr_response_contract_structure() {
    // Test that OCRResponse can be deserialized from Mistral AI OCR API response

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "This is the extracted text from the document.",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 1,
            "doc_size_bytes": 469
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    // Validate required fields
    assert_eq!(response.pages.len(), 1, "Should have one page");
    assert_eq!(response.model, "mistral-ocr-2505-completion", "Model field must match");
    assert_eq!(response.pages[0].index, 0, "Page index must be 0");
    assert_eq!(response.pages[0].markdown, "This is the extracted text from the document.", "Markdown content must match");
}

#[tokio::test]
async fn test_ocr_response_contract_required_fields() {
    // Test that all required fields according to JSON schema are present

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "Extracted text content here.",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 1,
            "doc_size_bytes": 469
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    // Validate all required fields from schema
    assert!(!response.model.is_empty(), "Model is required and must not be empty");
    assert!(!response.pages.is_empty(), "Pages array is required and must not be empty");
    assert_eq!(response.pages[0].index, 0, "Page index must be 0");
    assert!(!response.pages[0].markdown.is_empty(), "Page markdown content is required");
}

#[tokio::test]
async fn test_ocr_response_contract_pages_structure() {
    // Test that pages array structure matches the contract

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "This is the OCR extracted text.",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 1,
            "doc_size_bytes": 469
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    let page = &response.pages[0];
    assert_eq!(page.index, 0, "Page index must be 0");
    assert_eq!(page.markdown, "This is the OCR extracted text.", "Page markdown must match");
    assert_eq!(page.images.len(), 0, "Images array should be empty");
    assert_eq!(page.dimensions.dpi, 200, "DPI must match");
    assert_eq!(page.dimensions.height, 2200, "Height must match");
    assert_eq!(page.dimensions.width, 1700, "Width must match");
}

#[tokio::test]
async fn test_ocr_response_contract_usage_info() {
    // Test that usage_info field is properly structured

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "Test content",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 1,
            "doc_size_bytes": 469
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    assert_eq!(response.usage_info.pages_processed, 1, "Pages processed must match");
    assert_eq!(response.usage_info.doc_size_bytes, 469, "Document size must match");
}

#[tokio::test]
async fn test_ocr_response_contract_extracted_text_access() {
    // Test that we can extract the OCR text from pages
    // This test MUST FAIL until OCRResponse helper methods are implemented

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "This is the extracted OCR text from the document.",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 1,
            "doc_size_bytes": 469
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    // Test direct access
    let extracted_text = &response.pages[0].markdown;
    assert_eq!(extracted_text, "This is the extracted OCR text from the document.", "Extracted text must match page markdown");

    // Test helper method
    let extracted_via_helper = response.get_extracted_text();
    assert_eq!(extracted_via_helper, "This is the extracted OCR text from the document.", "Helper method should return same text as direct access");
}

#[tokio::test]
async fn test_ocr_response_contract_validation() {
    // Test that OCRResponse has validation methods

    let valid_response = OCRResponse {
        pages: vec![Page {
            index: 0,
            markdown: "Valid extracted text".to_string(),
            images: vec![],
            dimensions: Dimensions { dpi: 200, height: 2200, width: 1700 },
        }],
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo { pages_processed: 1, doc_size_bytes: 469 },
    };

    assert!(valid_response.validate().is_ok(), "Valid response should pass validation");

    // Test invalid response (empty pages)
    let invalid_response = OCRResponse {
        pages: vec![], // Empty pages should be invalid
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo { pages_processed: 0, doc_size_bytes: 0 },
    };

    assert!(invalid_response.validate().is_err(), "Invalid response (empty pages) should fail validation");
}

#[tokio::test]
async fn test_ocr_response_contract_multiple_pages() {
    // Test that OCRResponse can handle multiple pages

    let api_response_json = r#"{
        "pages": [
            {
                "index": 0,
                "markdown": "First page content",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            },
            {
                "index": 1,
                "markdown": "Second page content",
                "images": [],
                "dimensions": {
                    "dpi": 200,
                    "height": 2200,
                    "width": 1700
                }
            }
        ],
        "model": "mistral-ocr-2505-completion",
        "document_annotation": null,
        "usage_info": {
            "pages_processed": 2,
            "doc_size_bytes": 938
        }
    }"#;

    let response: OCRResponse = serde_json::from_str(api_response_json).expect("Should deserialize from API JSON");

    assert_eq!(response.pages.len(), 2, "Should have two pages");
    assert_eq!(response.pages[0].index, 0, "First page index must be 0");
    assert_eq!(response.pages[1].index, 1, "Second page index must be 1");
    assert_eq!(response.pages[0].markdown, "First page content", "First page content must match");
    assert_eq!(response.pages[1].markdown, "Second page content", "Second page content must match");

    // Test helper method with multiple pages
    let extracted_text = response.get_extracted_text();
    assert_eq!(extracted_text, "First page content\n\nSecond page content", "Helper method should concatenate multiple pages with newlines");
}
