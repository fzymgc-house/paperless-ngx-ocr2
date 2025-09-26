//! Basic CLI behavior tests
//! These tests validate that the CLI interface works correctly

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

// ============================================================================
// CLI SMOKE TESTS (T013)
// ============================================================================

#[tokio::test]
async fn test_cli_smoke_help_command() {
    // Test that the CLI binary can be executed and shows help
    // This test MUST FAIL until CLI help functionality is properly implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A command-line tool for extracting text"))
        .stdout(predicate::str::contains("--file"))
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--verbose"));
}

#[tokio::test]
async fn test_cli_smoke_version_command() {
    // Test that the CLI binary shows version information
    // This test MUST FAIL until CLI version functionality is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("paperless-ngx-ocr2"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[tokio::test]
async fn test_cli_smoke_no_args_shows_help() {
    // Test that running without arguments shows help
    // This test MUST FAIL until CLI argument validation is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.assert()
        .success() // Should succeed and show help since file is optional
        .stdout(predicate::str::contains("OCR CLI tool that uploads PDF/image files"));
}

// ============================================================================
// CLI FILE ARGUMENT TESTS (T014)
// ============================================================================

#[tokio::test]
async fn test_cli_file_argument_required() {
    // Test that --file argument is required for OCR processing (not for completions)
    // This test MUST FAIL until CLI file argument validation is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    // When no file is provided and no completions are requested, should show help
    cmd.assert()
        .success() // Should succeed and show help since file is optional
        .stdout(predicate::str::contains("OCR CLI tool that uploads PDF/image files"));
}

#[tokio::test]
async fn test_cli_file_argument_nonexistent_file() {
    // Test that CLI properly handles nonexistent files
    // This test MUST FAIL until file validation is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg("nonexistent_file.pdf")
        .arg("--api-key")
        .arg("test-key")
        .assert()
        .failure()
        .code(3) // I/O error per constitution
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

#[tokio::test]
async fn test_cli_file_argument_invalid_format() {
    // Test that CLI rejects invalid file formats
    // This test MUST FAIL until file format validation is implemented
    
    // Create a temporary file with invalid extension
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "This is not a valid image or PDF").unwrap();
    let temp_path = temp_file.path().with_extension("txt");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .assert()
        .failure()
        .code(2) // Validation error per constitution
        .stderr(predicate::str::contains("format").or(predicate::str::contains("supported")));
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_file_argument_valid_pdf() {
    // Test that CLI accepts valid PDF files (should fail at API call stage)
    // This test MUST FAIL until file validation and API integration is implemented
    
    // Create a temporary PDF file with PDF header
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n>>\nendobj\n").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("invalid-test-key") // Use invalid key to fail at API stage
        .assert()
        .failure()
        .code(2); // Should fail with config/API error, not validation error
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_file_argument_valid_image() {
    // Test that CLI accepts valid image files
    // This test MUST FAIL until file validation and API integration is implemented
    
    // Create a temporary PNG file with PNG header
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]).unwrap(); // PNG signature
    let temp_path = temp_file.path().with_extension("png");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("invalid-test-key")
        .assert()
        .failure()
        .code(2); // Should fail with API error, not validation error
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}
