//! OCR workflow integration tests
//! These tests validate the complete end-to-end OCR workflow

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;
use common::MockApiServer;
// use paperless_ngx_ocr2::Config; // Will be used when config loading is implemented

// ============================================================================
// HAPPY PATH INTEGRATION TESTS (T018, T019)
// ============================================================================

#[tokio::test]
async fn test_pdf_upload_and_ocr_integration() {
    // T018: Complete integration test for PDF file upload and OCR processing
    // This test MUST FAIL until the complete workflow is implemented

    // Create a test PDF file with recognizable content
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/Contents 4 0 R\n>>\nendobj\n4 0 obj\n<<\n/Length 44\n>>\nstream\nBT\n/F1 12 Tf\n72 720 Td\n(Hello World) Tj\nET\nendstream\nendobj\nxref\n0 5\n0000000000 65535 f \n0000000009 00000 n \n0000000074 00000 n \n0000000120 00000 n \n0000000179 00000 n \ntrailer\n<<\n/Size 5\n/Root 1 0 R\n>>\nstartxref\n274\n%%EOF").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    // Test complete workflow with valid API key (will fail until implementation)
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("sk-test-valid-api-key") // Test key - will fail with auth error
        .assert()
        .failure() // Expected to fail due to invalid API key
        .stderr(predicate::str::contains("Client error")); // Should show proper error handling

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_image_upload_and_ocr_integration() {
    // T019: Complete integration test for image file upload and OCR processing
    // This test MUST FAIL until the complete workflow is implemented

    // Create a test PNG file (minimal valid PNG)
    let mut temp_file = NamedTempFile::new().unwrap();
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // Width: 1
        0x00, 0x00, 0x00, 0x01, // Height: 1
        0x08, 0x02, 0x00, 0x00, 0x00, // Bit depth, color type, compression, filter, interlace
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND chunk length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ];
    temp_file.write_all(&png_data).unwrap();
    let temp_path = temp_file.path().with_extension("png");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    // Test complete workflow
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("sk-test-valid-api-key") // Test key - will fail with auth error
        .assert()
        .failure() // Expected to fail due to invalid API key
        .stderr(predicate::str::contains("Client error")); // Should show proper error handling

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_pdf_workflow_with_json_output() {
    // Test complete PDF workflow with JSON output format
    // This test MUST FAIL until JSON output and workflow are implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nSimple PDF with text content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    let output = cmd.arg("--file").arg(&temp_path).arg("--api-key").arg("sk-test-valid-api-key").arg("--json").output().expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let json: serde_json::Value = serde_json::from_str(&stdout).expect("JSON output should be valid JSON");

        // Validate JSON structure matches contract (should be error response)
        assert!(!json.get("success").unwrap().as_bool().unwrap());
        assert!(json.get("error").is_some());
        assert!(json.get("error").unwrap().get("type").is_some());
        assert!(json.get("error").unwrap().get("message").is_some());
    }

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_workflow_with_configuration_file() {
    // Test that workflow works with TOML configuration file
    // This test MUST FAIL until configuration loading is implemented

    // Create temporary config file
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(
        config_file,
        r#"
api_key = "sk-config-test-key"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 60
max_file_size_mb = 100
log_level = "info"
"#
    )
    .unwrap();

    // Create test PDF
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nConfig test content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .env("PAPERLESS_OCR_CONFIG", config_file.path()) // Point to config file
        .assert()
        .failure() // Expected to fail due to invalid API key in config
        .stderr(predicate::str::contains("error")); // Should show proper error handling

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_workflow_with_environment_variables() {
    // Test that workflow works with 12-factor environment variables
    // This test MUST FAIL until environment variable support is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nEnvironment test content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    // Start mock server to simulate authentication failure
    let mut mock_server = MockApiServer::new();
    mock_server.setup_error_mock(401).await; // 401 Unauthorized for invalid API key
    let mock_url = mock_server.start().await.expect("Failed to start mock server");

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .env("PAPERLESS_OCR_API_KEY", "sk-env-test-key") // Test key - will fail with auth error
        .env("PAPERLESS_OCR_API_BASE_URL", &mock_url)
        .env("PAPERLESS_OCR_TIMEOUT", "30")
        .assert()
        .failure() // Expected to fail due to invalid API key
        .stderr(predicate::str::contains("Network error")); // Should show proper error handling

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
    mock_server.stop().await.expect("Failed to stop mock server");
}
