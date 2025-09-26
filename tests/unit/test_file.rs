//! Unit tests for file validation

use paperless_ngx_ocr2::{FileUpload, Error};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_file_upload_new_valid_pdf() {
    // Create a temporary PDF file
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload for valid PDF");
    
    assert_eq!(file_upload.mime_type, "application/pdf");
    assert!(file_upload.is_valid);
    assert!(file_upload.file_size > 0);
    assert_eq!(file_upload.get_filename(), "tmp");
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_upload_new_valid_png() {
    // Use the test fixture PNG
    let file_upload = FileUpload::new("tests/fixtures/sample.png")
        .expect("Should create FileUpload for valid PNG");
    
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
    // Use the test fixture invalid text file
    let result = FileUpload::new("tests/fixtures/invalid.txt");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_new_corrupted_pdf() {
    // Use the test fixture corrupted PDF
    let result = FileUpload::new("tests/fixtures/corrupted.pdf");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_validate_file_size() {
    // Create a small file and test size validation logic
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nSmall file").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    
    // File should be valid (small size)
    assert!(file_upload.validate_file().is_ok());
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_upload_magic_bytes_validation() {
    // Test PDF magic bytes
    let mut pdf_file = NamedTempFile::new().unwrap();
    pdf_file.write_all(b"%PDF-1.4\nValid PDF").unwrap();
    let pdf_path = pdf_file.path().with_extension("pdf");
    fs::copy(pdf_file.path(), &pdf_path).unwrap();
    
    let pdf_upload = FileUpload::new(&pdf_path).expect("Should validate PDF magic bytes");
    assert!(pdf_upload.is_valid);
    
    // Test invalid magic bytes with PDF extension
    let mut fake_pdf = NamedTempFile::new().unwrap();
    fake_pdf.write_all(b"Not a PDF file").unwrap();
    let fake_path = fake_pdf.path().with_extension("pdf");
    fs::copy(fake_pdf.path(), &fake_path).unwrap();
    
    let fake_result = FileUpload::new(&fake_path);
    assert!(fake_result.is_err());
    
    // Cleanup
    fs::remove_file(&pdf_path).ok();
    fs::remove_file(&fake_path).ok();
}

#[test]
fn test_file_upload_set_file_id() {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    
    assert!(file_upload.file_id.is_none());
    
    file_upload.set_file_id("file-123".to_string());
    assert_eq!(file_upload.file_id, Some("file-123".to_string()));
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_upload_set_upload_status() {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    
    // Test valid status
    file_upload.set_upload_status("uploaded".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string()));
    
    // Test invalid status (should be ignored)
    file_upload.set_upload_status("invalid_status".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string())); // Should remain unchanged
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_upload_read_file_data() {
    // Use test fixture
    let file_upload = FileUpload::new("tests/fixtures/sample.pdf")
        .expect("Should create FileUpload");
    
    let file_data = file_upload.read_file_data().expect("Should read file data");
    
    assert!(!file_data.is_empty());
    assert!(file_data.starts_with(b"%PDF"));
}

#[test]
fn test_file_upload_get_filename() {
    let file_upload = FileUpload::new("tests/fixtures/sample.pdf")
        .expect("Should create FileUpload");
    
    assert_eq!(file_upload.get_filename(), "sample.pdf");
}
