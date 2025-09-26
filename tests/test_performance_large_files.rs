//! Performance tests for large files (up to 100MB)

use paperless_ngx_ocr2::{Error, FileUpload};
use std::fs;
use std::io::Write;
use std::time::Instant;
use tempfile::NamedTempFile;

#[test]
fn test_large_file_validation_performance() {
    // Create a moderately large PDF file (1MB) to test performance
    let mut temp_file = NamedTempFile::new().unwrap();

    // Write PDF header
    temp_file.write_all(b"%PDF-1.4\n").unwrap();

    // Write 1MB of content
    let content = "Test content line for performance testing.\n".repeat(25000); // ~1MB
    temp_file.write_all(content.as_bytes()).unwrap();

    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();

    // Measure validation time
    let start = Instant::now();
    let result = FileUpload::new(&temp_path);
    let duration = start.elapsed();

    // Should complete quickly (under 100ms for 1MB file)
    assert!(
        duration.as_millis() < 100,
        "File validation took too long: {:?}",
        duration
    );
    assert!(result.is_ok(), "Large file validation should succeed");

    let file_upload = result.unwrap();
    assert!(file_upload.file_size > 1_000_000); // Should be > 1MB
    assert!(file_upload.is_valid);

    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_file_size_limit_enforcement() {
    // Test that 100MB limit is properly enforced
    let file_upload = FileUpload {
        file_path: "test.pdf".to_string(),
        file_size: 101 * 1024 * 1024, // 101MB
        mime_type: "application/pdf".to_string(),
        file_id: None,
        upload_status: None,
        is_valid: false,
    };

    let result = file_upload.validate_file();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));
}

#[test]
fn test_file_size_boundary_conditions() {
    // Test exactly 100MB (should pass)
    let file_upload_100mb = FileUpload {
        file_path: "tests/fixtures/sample.pdf".to_string(),
        file_size: 100 * 1024 * 1024, // Exactly 100MB
        mime_type: "application/pdf".to_string(),
        file_id: None,
        upload_status: None,
        is_valid: false,
    };

    // Note: This will fail because the actual file doesn't exist at that size,
    // but it tests the size validation logic
    let result = file_upload_100mb.validate_file();
    // The error should be about file existence, not size
    if let Err(e) = result {
        assert!(!e.to_string().contains("exceeds maximum"));
    }

    // Test 100MB + 1 byte (should fail due to size)
    let file_upload_over = FileUpload {
        file_path: "tests/fixtures/sample.pdf".to_string(),
        file_size: (100 * 1024 * 1024) + 1, // 100MB + 1 byte
        mime_type: "application/pdf".to_string(),
        file_id: None,
        upload_status: None,
        is_valid: false,
    };

    let result = file_upload_over.validate_file();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("exceeds maximum"));
    }
}

#[test]
fn test_magic_bytes_validation_performance() {
    // Test that magic byte validation is fast
    let start = Instant::now();

    // Test multiple files quickly
    for _ in 0..100 {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"%PDF-1.4\nQuick test").unwrap();
        let temp_path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &temp_path).unwrap();

        let _ = FileUpload::new(&temp_path);

        fs::remove_file(&temp_path).ok();
    }

    let duration = start.elapsed();

    // Should complete 100 validations quickly (under 1 second)
    assert!(
        duration.as_millis() < 1000,
        "Magic byte validation too slow: {:?}",
        duration
    );
}

#[test]
fn test_file_read_performance() {
    // Create a medium-sized file and test read performance
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();

    // Write 10MB of content
    let content = "Performance test content line.\n".repeat(350000); // ~10MB
    temp_file.write_all(content.as_bytes()).unwrap();

    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();

    let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");

    // Measure file read time
    let start = Instant::now();
    let file_data = file_upload.read_file_data().expect("Should read file data");
    let duration = start.elapsed();

    // Should read 10MB quickly (under 500ms)
    assert!(
        duration.as_millis() < 500,
        "File read took too long: {:?}",
        duration
    );
    assert!(file_data.len() > 10_000_000); // Should be > 10MB
    assert!(file_data.starts_with(b"%PDF"));

    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[test]
fn test_concurrent_file_validation() {
    use std::thread;
    // use std::sync::Arc; // Not needed for this test

    // Test concurrent file validation (simulating multiple files)
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let mut temp_file = NamedTempFile::new().unwrap();
                temp_file
                    .write_all(format!("%PDF-1.4\nConcurrent test {}", i).as_bytes())
                    .unwrap();
                let temp_path = temp_file.path().with_extension("pdf");
                fs::copy(temp_file.path(), &temp_path).unwrap();

                let start = Instant::now();
                let result = FileUpload::new(&temp_path);
                let duration = start.elapsed();

                fs::remove_file(&temp_path).ok();

                (result.is_ok(), duration)
            })
        })
        .collect();

    // Wait for all threads and check results
    for handle in handles {
        let (success, duration) = handle.join().unwrap();
        assert!(success, "Concurrent file validation should succeed");
        assert!(
            duration.as_millis() < 50,
            "Concurrent validation should be fast"
        );
    }
}
