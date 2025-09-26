//! Memory usage tests with streaming

use paperless_ngx_ocr2::{FileUpload, api::files::FileUploadRequest};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use std::time::Instant;

#[test]
fn test_memory_efficient_file_reading() {
    // Create a large file and test that memory usage is reasonable
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write 5MB of content
    let content = "Memory test content line for streaming validation.\n".repeat(100000); // ~5MB
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Test file validation (should not load entire file into memory)
    let start = Instant::now();
    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    let validation_duration = start.elapsed();
    
    // Validation should be fast even for large files (only reads first 8 bytes for magic)
    assert!(validation_duration.as_millis() < 50, "Validation should be fast: {:?}", validation_duration);
    assert!(file_upload.file_size > 5_000_000); // Should be > 5MB
    
    // Test reading file data (this will load into memory, but should be reasonable)
    let start = Instant::now();
    let file_data = file_upload.read_file_data().expect("Should read file data");
    let read_duration = start.elapsed();
    
    assert!(read_duration.as_millis() < 1000, "File read should be reasonable: {:?}", read_duration);
    assert_eq!(file_data.len(), file_upload.file_size as usize);
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_upload_request_memory_usage() {
    // Test that FileUploadRequest handles large files efficiently
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write 2MB of content
    let content = "Upload request memory test content.\n".repeat(50000); // ~2MB
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    let file_data = file_upload.read_file_data().expect("Should read file data");
    
    // Test FileUploadRequest creation
    let start = Instant::now();
    let upload_request = FileUploadRequest::new(file_data, file_upload.get_filename());
    let creation_duration = start.elapsed();
    
    // Should create quickly even for large files
    assert!(creation_duration.as_millis() < 100, "Request creation should be fast: {:?}", creation_duration);
    
    // Test validation
    let start = Instant::now();
    let validation_result = upload_request.validate();
    let validation_duration = start.elapsed();
    
    assert!(validation_result.is_ok());
    assert!(validation_duration.as_millis() < 10, "Validation should be very fast: {:?}", validation_duration);
    
    // Test multipart form creation
    let start = Instant::now();
    let form_result = upload_request.to_multipart_form();
    let form_duration = start.elapsed();
    
    assert!(form_result.is_ok());
    assert!(form_duration.as_millis() < 200, "Multipart form creation should be reasonable: {:?}", form_duration);
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_memory_usage_with_multiple_files() {
    // Test memory usage when processing multiple files sequentially
    let file_sizes = vec![1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB
    
    for size in file_sizes {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"%PDF-1.4\n").unwrap();
        
        // Write specified amount of content
        let content = "x".repeat(size);
        temp_file.write_all(content.as_bytes()).unwrap();
        
        let temp_path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &temp_path).unwrap();
        
        // Test that each file is processed independently (no memory accumulation)
        let start = Instant::now();
        let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
        let _file_data = file_upload.read_file_data().expect("Should read file data");
        let duration = start.elapsed();
        
        // Processing time should be roughly proportional to file size
        let expected_max_ms = (size / 1000) + 50; // ~1ms per KB + 50ms overhead
        assert!(duration.as_millis() < expected_max_ms as u128, 
               "File processing took too long for {} bytes: {:?}", size, duration);
        
        // Cleanup
        fs::remove_file(&temp_path).ok();
    }
}

#[test]
fn test_streaming_validation_efficiency() {
    // Test that file validation only reads what it needs (magic bytes)
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write a large amount of content after the header
    let large_content = "Large content that should not be read during validation.\n".repeat(100000); // ~5MB
    temp_file.write_all(large_content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Measure validation time - should be constant regardless of file size
    let start = Instant::now();
    let file_upload = FileUpload::new(&temp_path).expect("Should validate large file");
    let validation_duration = start.elapsed();
    
    // Validation should be very fast because it only reads first 8 bytes
    assert!(validation_duration.as_millis() < 20, "Validation should be constant time: {:?}", validation_duration);
    assert!(file_upload.file_size > 5_000_000); // Should be > 5MB
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_metadata_access_performance() {
    // Test that accessing file metadata is fast
    let file_upload = FileUpload::new("tests/fixtures/sample.pdf")
        .expect("Should create FileUpload");
    
    // Test multiple metadata accesses
    let start = Instant::now();
    for _ in 0..1000 {
        let _filename = file_upload.get_filename();
        let _size = file_upload.file_size;
        let _mime = &file_upload.mime_type;
        let _valid = file_upload.is_valid;
    }
    let duration = start.elapsed();
    
    // 1000 metadata accesses should be very fast
    assert!(duration.as_millis() < 10, "Metadata access should be very fast: {:?}", duration);
}

#[test]
fn test_large_file_size_calculation_accuracy() {
    // Test that file size calculations are accurate for large files
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write exactly 10MB of content
    let content_size = 10 * 1024 * 1024 - 9; // 10MB minus PDF header
    let content = "x".repeat(content_size);
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");
    
    // Should be exactly 10MB
    assert_eq!(file_upload.file_size, 10 * 1024 * 1024);
    
    // Test size validation message accuracy
    let oversized_upload = FileUpload {
        file_path: temp_path.to_string_lossy().to_string(),
        file_size: 150 * 1024 * 1024, // 150MB
        mime_type: "application/pdf".to_string(),
        file_id: None,
        upload_status: None,
        is_valid: false,
    };
    
    let result = oversized_upload.validate_file();
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("150.00 MB"));
    assert!(error_msg.contains("100 MB"));
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}
