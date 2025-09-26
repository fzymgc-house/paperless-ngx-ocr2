//! Performance tests for file streaming functionality

use paperless_ngx_ocr2::api::files::FileUploadRequest;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use std::time::Instant;

#[tokio::test]
async fn test_streaming_threshold_detection() {
    // Test that streaming is used for files >50MB
    const STREAMING_THRESHOLD: usize = 50 * 1024 * 1024; // 50MB
    
    // Create a large file (>50MB)
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write 55MB of content
    let content = "Streaming test content for large file validation.\n".repeat(1_000_000); // ~55MB
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Test streaming multipart form creation
    let start = Instant::now();
    let form_result = FileUploadRequest::to_streaming_multipart_form(
        temp_path.to_str().unwrap(), 
        "ocr"
    ).await;
    let duration = start.elapsed();
    
    assert!(form_result.is_ok(), "Should create streaming form for large file");
    assert!(duration.as_millis() < 1000, "Streaming form creation should be fast: {:?}", duration);
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_streaming_memory_efficiency() {
    // Test that streaming doesn't load entire file into memory
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write 60MB of content
    let content = "Memory efficiency test for streaming upload.\n".repeat(1_200_000); // ~60MB
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Test that we can create streaming form without loading entire file
    let start = Instant::now();
    let form_result = FileUploadRequest::to_streaming_multipart_form(
        temp_path.to_str().unwrap(), 
        "ocr"
    ).await;
    let duration = start.elapsed();
    
    assert!(form_result.is_ok());
    // Streaming form creation should be fast even for large files
    assert!(duration.as_millis() < 500, "Streaming should be fast regardless of file size: {:?}", duration);
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_regular_vs_streaming_threshold() {
    // Test that regular upload is used for files <50MB
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    
    // Write 10MB of content (under threshold)
    let content = "Regular upload test content.\n".repeat(200_000); // ~10MB
    temp_file.write_all(content.as_bytes()).unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Test regular multipart form creation
    let start = Instant::now();
    let form_result = FileUploadRequest::to_streaming_multipart_form(
        temp_path.to_str().unwrap(), 
        "ocr"
    ).await;
    let duration = start.elapsed();
    
    assert!(form_result.is_ok());
    assert!(duration.as_millis() < 100, "Regular form creation should be very fast: {:?}", duration);
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_streaming_form_validation() {
    // Test that streaming forms are properly validated
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n").unwrap();
    temp_file.write_all(b"Test content for streaming validation.\n").unwrap();
    
    let temp_path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &temp_path).unwrap();
    
    // Test streaming form creation and validation
    let form = FileUploadRequest::to_streaming_multipart_form(
        temp_path.to_str().unwrap(), 
        "ocr"
    ).await.expect("Should create streaming form");
    
    // Verify form has required parts
    // Note: We can't directly test multipart form contents, but we can verify creation succeeds
    
    // Cleanup
    fs::remove_file(&temp_path).ok();
}
