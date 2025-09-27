//! Unit tests for API client methods

use paperless_ngx_ocr2::{
    api::{
        auth::AuthHandler,
        files::{FileUploadRequest, FileUploadResponse, FilesClient},
        ocr::{Dimensions, OCRClient, OCRRequest, OCRResponse, Page, UsageInfo},
        MistralClient,
    },
    APICredentials, Error,
};

#[test]
fn test_file_upload_request_new() {
    let file_data = b"Test file content".to_vec();
    let filename = "test.pdf".to_string();

    let request = FileUploadRequest::new(file_data.clone(), filename.clone());

    assert_eq!(request.file_data, file_data);
    assert_eq!(request.filename, filename);
    assert_eq!(request.purpose, "ocr");
}

#[test]
fn test_file_upload_request_validation() {
    // Valid request
    let valid_request = FileUploadRequest {
        file_data: b"Valid data".to_vec(),
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
    };
    assert!(valid_request.validate().is_ok());

    // Empty file data
    let empty_data_request = FileUploadRequest {
        file_data: vec![],
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
    };
    assert!(empty_data_request.validate().is_err());

    // Empty filename
    let empty_filename_request = FileUploadRequest {
        file_data: b"Valid data".to_vec(),
        filename: "".to_string(),
        purpose: "ocr".to_string(),
    };
    assert!(empty_filename_request.validate().is_err());

    // Invalid purpose
    let invalid_purpose_request = FileUploadRequest {
        file_data: b"Valid data".to_vec(),
        filename: "test.pdf".to_string(),
        purpose: "invalid".to_string(),
    };
    assert!(invalid_purpose_request.validate().is_err());
}

#[test]
fn test_file_upload_request_multipart_form() {
    let request = FileUploadRequest {
        file_data: b"Test content".to_vec(),
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
    };

    let form = request
        .to_multipart_form()
        .expect("Should create multipart form");

    // Test helper methods
    assert!(request.has_file_part());
    assert!(request.has_purpose_part());
}

#[test]
fn test_file_upload_response_validation() {
    // Valid response
    let valid_response = FileUploadResponse {
        id: "file-123".to_string(),
        object: "file".to_string(),
        bytes: 1024,
        created_at: 1640995200,
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
        status: Some("uploaded".to_string()),
    };
    assert!(valid_response.validate().is_ok());

    // Empty ID
    let empty_id_response = FileUploadResponse {
        id: "".to_string(),
        object: "file".to_string(),
        bytes: 1024,
        created_at: 1640995200,
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
        status: None,
    };
    assert!(empty_id_response.validate().is_err());

    // Invalid object type
    let invalid_object_response = FileUploadResponse {
        id: "file-123".to_string(),
        object: "invalid".to_string(),
        bytes: 1024,
        created_at: 1640995200,
        filename: "test.pdf".to_string(),
        purpose: "ocr".to_string(),
        status: None,
    };
    assert!(invalid_object_response.validate().is_err());
}

#[test]
fn test_ocr_request_validation() {
    // Valid request
    let valid_request = OCRRequest::new("file-123".to_string());
    assert!(valid_request.validate().is_ok());

    // Empty file ID
    let empty_request = OCRRequest::new("".to_string());
    assert!(empty_request.validate().is_err());
}

#[test]
fn test_ocr_response_get_extracted_text() {
    let response = OCRResponse {
        pages: vec![Page {
            index: 0,
            markdown: "Extracted text content".to_string(),
            images: vec![],
            dimensions: Dimensions {
                dpi: 200,
                height: 2200,
                width: 1700,
            },
        }],
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo {
            pages_processed: 1,
            doc_size_bytes: 469,
        },
    };

    assert_eq!(response.get_extracted_text(), "Extracted text content");
}

#[test]
fn test_ocr_response_validation() {
    // Valid response
    let valid_response = OCRResponse {
        pages: vec![Page {
            index: 0,
            markdown: "Test content".to_string(),
            images: vec![],
            dimensions: Dimensions {
                dpi: 200,
                height: 2200,
                width: 1700,
            },
        }],
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo {
            pages_processed: 1,
            doc_size_bytes: 469,
        },
    };
    assert!(valid_response.validate().is_ok());

    // Empty pages
    let empty_pages_response = OCRResponse {
        pages: vec![],
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo {
            pages_processed: 0,
            doc_size_bytes: 0,
        },
    };
    assert!(empty_pages_response.validate().is_err());

    // Invalid page index
    let invalid_index_response = OCRResponse {
        pages: vec![Page {
            index: 1, // Should be 0 for first page
            markdown: "Test content".to_string(),
            images: vec![],
            dimensions: Dimensions {
                dpi: 200,
                height: 2200,
                width: 1700,
            },
        }],
        model: "mistral-ocr-2505-completion".to_string(),
        document_annotation: None,
        usage_info: UsageInfo {
            pages_processed: 1,
            doc_size_bytes: 469,
        },
    };
    assert!(invalid_index_response.validate().is_err());
}

#[test]
fn test_auth_handler_get_auth_headers() {
    let credentials = APICredentials::new(
        "sk-test123".to_string(),
        "https://api.mistral.ai".to_string(),
    )
    .expect("Should create credentials");

    let auth_handler = AuthHandler::new(credentials);
    let headers = auth_handler
        .get_auth_headers()
        .expect("Should get auth headers");

    assert!(headers.contains_key("authorization"));
    assert!(headers.contains_key("content-type"));
}

#[test]
fn test_auth_handler_redacted_key() {
    let credentials = APICredentials::new(
        "sk-test123456789".to_string(),
        "https://api.mistral.ai".to_string(),
    )
    .expect("Should create credentials");

    let auth_handler = AuthHandler::new(credentials);
    let redacted = auth_handler.redacted_key();

    assert!(redacted.contains("sk-t***"));
    assert!(!redacted.contains("123456789"));
}

#[tokio::test]
async fn test_mistral_client_creation() {
    let credentials = APICredentials::new(
        "sk-test123".to_string(),
        "https://api.mistral.ai".to_string(),
    )
    .expect("Should create credentials");

    let client = MistralClient::new(credentials, 30).expect("Should create MistralClient");

    assert_eq!(client.base_url(), "https://api.mistral.ai");
    assert!(client.auth_header().starts_with("Bearer "));
}

#[test]
fn test_mistral_client_build_url() {
    let credentials = APICredentials::new(
        "sk-test123".to_string(),
        "https://api.mistral.ai".to_string(),
    )
    .expect("Should create credentials");

    let client = MistralClient::new(credentials, 30).expect("Should create MistralClient");

    assert_eq!(
        client.build_url("v1/files"),
        "https://api.mistral.ai/v1/files"
    );
    assert_eq!(client.build_url("/v1/ocr"), "https://api.mistral.ai/v1/ocr");
}
