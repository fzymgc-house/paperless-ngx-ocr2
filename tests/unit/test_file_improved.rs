//! Unit tests for file validation - Improved version using test utilities
//!
//! This file demonstrates the improved test patterns using the common test utilities.
//! It shows how to use TestFile for automatic cleanup, fixtures for stable test data,
//! and performance testing for validation operations.

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
    // Use fixture for stable test data
    let fixture = fixtures::sample_png();
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
    // Use fixture for invalid format testing
    let fixture = fixtures::invalid_txt();
    let result = FileUpload::new(fixture.path_str());

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_new_corrupted_pdf() {
    // Use fixture for corrupted file testing
    let fixture = fixtures::corrupted_pdf();
    let result = FileUpload::new(fixture.path_str());

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_upload_validate_file_size() {
    // Use TestFile for temporary content
    let test_file = create_test_pdf("Small file content");

    let file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    // File should be valid (small size)
    assert!(file_upload.validate_file().is_ok());
}

#[test]
fn test_file_upload_magic_bytes_validation() {
    // Test PDF magic bytes with TestFile
    let pdf_file = create_test_pdf("Valid PDF content");
    let pdf_upload = FileUpload::new(pdf_file.path()).expect("Should validate PDF magic bytes");
    assert!(pdf_upload.is_valid);

    // Test invalid magic bytes with corrupted file
    let corrupted_file = create_corrupted_pdf();
    let fake_result = FileUpload::new(corrupted_file.path());
    assert!(fake_result.is_err());
}

#[test]
fn test_file_upload_set_file_id() {
    let test_file = create_test_pdf("Test content");
    let mut file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    assert!(file_upload.file_id.is_none());

    file_upload.set_file_id("file-123".to_string());
    assert_eq!(file_upload.file_id, Some("file-123".to_string()));
}

#[test]
fn test_file_upload_set_upload_status() {
    let test_file = create_test_pdf("Test content");
    let mut file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");

    // Test valid status
    file_upload.set_upload_status("uploaded".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string()));

    // Test invalid status (should be ignored)
    file_upload.set_upload_status("invalid_status".to_string());
    assert_eq!(file_upload.upload_status, Some("uploaded".to_string())); // Should remain unchanged
}

#[test]
fn test_file_upload_read_file_data() {
    // Use fixture for stable test data
    let fixture = fixtures::sample_pdf();
    let file_upload = FileUpload::new(fixture.path_str()).expect("Should create FileUpload");

    let file_data = file_upload.read_file_data().expect("Should read file data");

    assert!(!file_data.is_empty());
    assert!(file_data.starts_with(b"%PDF"));
}

#[test]
fn test_file_upload_get_filename() {
    let fixture = fixtures::sample_pdf();
    let file_upload = FileUpload::new(fixture.path_str()).expect("Should create FileUpload");

    assert_eq!(file_upload.get_filename(), "sample.pdf");
}

// Performance tests using the new utilities
#[test]
fn test_file_validation_performance() {
    let test_file = create_test_pdf("Performance test content");

    // Measure file validation performance
    measure_performance("file_validation", Duration::from_millis(100), || {
        let _file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");
    });
}

#[test]
fn test_file_validation_benchmark() {
    let test_file = create_test_pdf("Benchmark test content");

    // Run benchmark for file validation
    let results = Benchmark::new("file_validation_benchmark")
        .iterations(100)
        .warmup_iterations(10)
        .run(|| {
            let _file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");
        });

    // Assert performance requirements
    results.assert_avg_time_less_than(Duration::from_millis(50));
    results.assert_p95_time_less_than(Duration::from_millis(100));
}

#[test]
fn test_large_file_performance() {
    // Create a larger file for performance testing
    let large_file = create_large_test_pdf(1); // 1MB

    // Test that validation is still fast for larger files
    measure_performance("large_file_validation", Duration::from_millis(200), || {
        let _file_upload =
            FileUpload::new(large_file.path()).expect("Should create FileUpload for large file");
    });
}

#[test]
fn test_memory_usage_with_file_validation() {
    use performance::memory::*;

    // Reset memory tracking
    reset_memory_tracking();
    let memory_test = MemoryTest::new().with_max_increase(1024 * 1024); // 1MB max increase

    // Create and validate multiple files
    for _ in 0..10 {
        let test_file = create_test_pdf("Memory test content");
        let _file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");
    }

    // Assert memory usage is within limits
    memory_test.assert_memory_usage();
}

#[test]
fn test_file_validation_stress() {
    let test_file = create_test_pdf("Stress test content");

    // Run stress test for file validation
    let results = stress::stress_test("file_validation_stress", 100, || {
        let _file_upload = FileUpload::new(test_file.path()).expect("Should create FileUpload");
    });

    // Assert stress test results
    results.assert_error_rate_less_than(0.01); // < 1% error rate
    results.assert_avg_time_less_than(Duration::from_millis(50));
}

#[test]
fn test_multiple_file_types_performance() {
    // Test performance across different file types
    let file_types = vec![
        ("PDF", create_test_pdf("PDF content")),
        ("PNG", create_test_png()),
    ];

    for (file_type, test_file) in file_types {
        let results = Benchmark::new(&format!("{}_validation", file_type))
            .iterations(50)
            .warmup_iterations(5)
            .run(|| {
                let _file_upload =
                    FileUpload::new(test_file.path()).expect("Should create FileUpload");
            });

        // All file types should validate quickly
        results.assert_avg_time_less_than(Duration::from_millis(100));
    }
}
