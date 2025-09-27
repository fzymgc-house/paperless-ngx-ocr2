//! Memory usage tests with streaming

mod common;

use common::*;
use paperless_ngx_ocr2::{api::files::FileUploadRequest, FileUpload};
use std::time::Duration;

#[test]
fn test_memory_efficient_file_reading() {
    // Use large test file for memory testing - test with a substantial file size
    let large_file = create_large_test_pdf(25); // 25MB - substantial enough to test memory handling

    // Test file validation with performance measurement
    measure_performance("file_validation", Duration::from_millis(50), || {
        let _file_upload = FileUpload::new(large_file.path()).expect("Should create FileUpload");
    });

    let file_upload = FileUpload::new(large_file.path()).expect("Should create FileUpload");
    assert!(file_upload.file_size > 20_000_000); // Should be > 20MB

    // Test reading file data with performance measurement
    measure_performance("file_reading", Duration::from_millis(1000), || {
        let _file_data = file_upload.read_file_data().expect("Should read file data");
    });

    let file_data = file_upload.read_file_data().expect("Should read file data");
    assert_eq!(file_data.len(), file_upload.file_size as usize);
    // Automatic cleanup on drop
}

#[test]
fn test_file_upload_request_memory_usage() {
    // Test that FileUploadRequest handles large files efficiently
    let large_file = create_large_test_pdf(10); // 10MB - test with substantial file size

    let file_upload = FileUpload::new(large_file.path()).expect("Should create FileUpload");
    let file_data = file_upload.read_file_data().expect("Should read file data");

    // Test FileUploadRequest creation with performance measurement
    let upload_request =
        measure_performance("request_creation", Duration::from_millis(100), || FileUploadRequest::new(file_data.clone(), file_upload.get_filename()));

    // Test validation with performance measurement
    let validation_result = measure_performance("request_validation", Duration::from_millis(10), || upload_request.validate());

    assert!(validation_result.is_ok());

    // Test multipart form creation with performance measurement
    let form_result = measure_performance("multipart_form_creation", Duration::from_millis(200), || upload_request.to_multipart_form());

    assert!(form_result.is_ok());
    // Automatic cleanup on drop
}

#[test]
fn test_memory_usage_with_multiple_files() {
    // Test memory usage when processing multiple files sequentially
    // Use files up to the maximum allowed size (100MB) to test memory handling
    let file_sizes_mb = vec![1, 10, 50, 95]; // 1MB, 10MB, 50MB, 95MB (just under 100MB limit)

    for size_mb in file_sizes_mb {
        let test_file = create_large_test_pdf(size_mb);

        // Test that each file is processed independently (no memory accumulation)
        let expected_max_ms = (size_mb * 100) + 1000; // ~100ms per MB + 1s overhead
        measure_performance("file_processing", Duration::from_millis(expected_max_ms as u64), || {
            let file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");
            let _file_data = file_upload.read_file_data().expect("Should read file data");
        });

        // Automatic cleanup on drop
    }
}

#[test]
fn test_streaming_validation_efficiency() {
    // Test that file validation only reads what it needs (magic bytes)
    let large_file = create_large_test_pdf(30); // 30MB - test with substantial file size

    // Measure validation time - should be constant regardless of file size
    let file_upload =
        measure_performance("streaming_validation", Duration::from_millis(20), || FileUpload::new(large_file.path()).expect("Should validate large file"));

    // Validation should be very fast because it only reads first 8 bytes
    assert!(file_upload.file_size > 25_000_000); // Should be > 25MB
                                                 // Automatic cleanup on drop
}

#[test]
fn test_file_metadata_access_performance() {
    // Test that accessing file metadata is fast
    let test_file = create_test_pdf("Sample metadata performance test PDF");

    let file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    // Test multiple metadata accesses with performance measurement
    measure_performance("metadata_access", Duration::from_millis(10), || {
        for _ in 0..1000 {
            let _filename = file_upload.get_filename();
            let _size = file_upload.file_size;
            let _mime = &file_upload.mime_type;
            let _valid = file_upload.is_valid;
        }
    });

    // Automatic cleanup on drop
}

#[test]
fn test_large_file_size_calculation_accuracy() {
    // Test that file size calculations are accurate for large files
    let large_file = create_large_test_pdf(50); // 50MB

    let file_upload = FileUpload::new(large_file.path()).expect("Should create FileUpload");

    // Should be approximately 50MB (allowing for PDF overhead)
    assert!(file_upload.file_size > 45 * 1024 * 1024); // At least 45MB
    assert!(file_upload.file_size < 55 * 1024 * 1024); // At most 55MB

    // Test size validation message accuracy with a file that exceeds the limit
    let oversized_upload = FileUpload {
        file_path: large_file.path().to_string_lossy().to_string(),
        file_size: 150 * 1024 * 1024, // 150MB - exceeds the 100MB limit
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

    // Test with a file that's exactly at the limit (should be valid)
    let max_size_upload = FileUpload {
        file_path: large_file.path().to_string_lossy().to_string(),
        file_size: 100 * 1024 * 1024, // Exactly 100MB - should be valid
        mime_type: "application/pdf".to_string(),
        file_id: None,
        upload_status: None,
        is_valid: true,
    };

    let result = max_size_upload.validate_file();
    assert!(result.is_ok(), "Files at the maximum size limit should be valid");

    // Automatic cleanup on drop
}
