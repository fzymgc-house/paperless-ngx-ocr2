//! Unit tests for file validation

mod common;

use common::*;
use paperless_ngx_ocr2::{Error, FileUpload};

#[test]
fn test_file_upload_new_valid_pdf() {
    // Use TestFile helper for automatic cleanup
    let test_file = create_test_pdf("Test content");

    let file_upload =
        FileUpload::new(test_file.path()).expect("Should create FileUpload for valid PDF");

    assert_eq!(file_upload.mime_type, "application/pdf");
    assert!(file_upload.is_valid);
    assert!(file_upload.file_size > 0);
    assert_eq!(file_upload.get_filename(), "tmp");
    // Automatic cleanup on drop
}

#[test]
fn test_file_upload_new_valid_png() {
    // Use fixture helper for better type safety
    let fixture = fixtures::fixtures::sample_png();
    let file_upload =
        FileUpload::new(fixture.path_str()).expect("Should create FileUpload for valid PNG");

    assert_eq!(file_upload.mime_type, "image/png");
    assert!(file_upload.is_valid);
    assert!(file_upload.file_size > 0);
    assert_eq!(file_upload.get_filename(), "sample.png");
}

#[test]
fn test_file_upload_new_nonexistent_file() {
    let result = FileUpload::new("nonexistent_file.pdf");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Io(_)));
}

#[test]
fn test_file_upload_new_invalid_format() {
    // Use fixture helper for invalid format testing
    let fixture = fixtures::fixtures::invalid_txt();
    let result = FileUpload::new(fixture.path_str());

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_new_corrupted_pdf() {
    // Use fixture helper for corrupted file testing
    let fixture = fixtures::fixtures::corrupted_pdf();
    let result = FileUpload::new(fixture.path_str());

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_validate_file_size() {
    // Use TestFile for temporary content
    let test_file = create_test_pdf("Small file");

    let file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    // File should be valid (small size)
    assert!(file_upload.validate_file().is_ok());
    // Automatic cleanup on drop
}

#[test]
fn test_file_upload_magic_bytes_validation() {
    // Test PDF magic bytes with TestFile
    let pdf_file = create_test_pdf("Valid PDF");
    let pdf_upload = FileUpload::new(pdf_file.path()).expect("Should validate PDF magic bytes");
    assert!(pdf_upload.is_valid);

    // Test invalid magic bytes with corrupted file
    let corrupted_file = create_corrupted_pdf();
    let fake_result = FileUpload::new(corrupted_file.path());
    assert!(fake_result.is_err());
    // Automatic cleanup on drop
}

#[test]
fn test_file_upload_set_file_id() {
    let test_file = create_test_pdf("Test");

    let mut file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    assert!(file_upload.file_id.is_none());

    file_upload.set_file_id("file-123".to_string());
    assert_eq!(file_upload.file_id, Some("file-123".to_string()));
    // Automatic cleanup on drop
}

#[test]
fn test_file_upload_set_upload_status() {
    let test_file = create_test_pdf("Test");

    let mut file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    // Test valid status
    file_upload.set_upload_status("uploaded".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string()));

    // Test invalid status (should be ignored)
    file_upload.set_upload_status("invalid_status".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string())); // Should remain unchanged
                                                                         // Automatic cleanup on drop
}

#[test]
fn test_file_upload_read_file_data() {
    // Use fixture for stable test data
    let fixture = fixtures::fixtures::sample_pdf();
    let file_upload = FileUpload::new(fixture.path_str()).expect("Should create FileUpload");

    let file_data = file_upload.read_file_data().expect("Should read file data");

    assert!(!file_data.is_empty());
    assert!(file_data.starts_with(b"%PDF"));
}

#[test]
fn test_file_upload_get_filename() {
    let fixture = fixtures::fixtures::sample_pdf();
    let file_upload = FileUpload::new(fixture.path_str()).expect("Should create FileUpload");

    assert_eq!(file_upload.get_filename(), "sample.pdf");
}
