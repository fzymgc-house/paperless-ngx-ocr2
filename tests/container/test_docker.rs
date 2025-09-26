use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;

/// Test that the Docker container can be built successfully
#[test]
fn test_docker_build() {
    let mut cmd = Command::new("docker");
    cmd.args(&["build", "-t", "paperless-ngx-ocr2:test", "."]);
    
    let output = cmd.output().expect("Failed to execute docker build");
    
    assert!(
        output.status.success(),
        "Docker build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test that the Docker container runs and shows help
#[test]
fn test_docker_container_help() {
    // First build the image
    let build_cmd = Command::new("docker")
        .args(&["build", "-t", "paperless-ngx-ocr2:test", "."])
        .output()
        .expect("Failed to execute docker build");
    
    assert!(build_cmd.status.success(), "Docker build failed");
    
    // Then run the container with --help
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "--help"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    assert!(
        output.status.success(),
        "Docker container help failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("paperless-ngx-ocr2"));
    assert!(stdout.contains("OCR CLI tool"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

/// Test that the Docker container shows version
#[test]
fn test_docker_container_version() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "--version"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    assert!(
        output.status.success(),
        "Docker container version failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("paperless-ngx-ocr2"));
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

/// Test that the Docker container can generate shell completions
#[test]
fn test_docker_container_completions() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "--completions",
        "bash"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    assert!(
        output.status.success(),
        "Docker container completions failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("complete -F _paperless_ngx_ocr2_completion"));
    assert!(stdout.contains("paperless-ngx-ocr2"));
}

/// Test that the Docker container handles missing API key gracefully
#[test]
fn test_docker_container_missing_api_key() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "--file",
        "/dev/null"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    // Should fail with configuration error
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Configuration error") || stderr.contains("API key"));
}

/// Test that the Docker container can process a file with API key
#[test]
fn test_docker_container_with_api_key() {
    // Skip this test if no API key is provided
    if std::env::var("PAPERLESS_OCR_API_KEY").is_err() {
        println!("Skipping test_docker_container_with_api_key: No API key provided");
        return;
    }
    
    // Create a temporary test file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Test content for OCR").expect("Failed to write test file");
    
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "-e",
        &format!("PAPERLESS_OCR_API_KEY={}", std::env::var("PAPERLESS_OCR_API_KEY").unwrap()),
        "-v",
        &format!("{}:/test.txt:ro", test_file.display()),
        "paperless-ngx-ocr2:test",
        "--file",
        "/test.txt"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    // This might succeed or fail depending on API availability
    // We just want to ensure the container can run with the API key
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should not crash with configuration error
    assert!(
        !stderr.contains("Configuration error") && !stderr.contains("API key must not be empty"),
        "Container failed with configuration error: {}",
        stderr
    );
}

/// Test Docker Compose service
#[test]
fn test_docker_compose_help() {
    let mut cmd = Command::new("docker-compose");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2",
        "--help"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker-compose run");
    
    assert!(
        output.status.success(),
        "Docker Compose help failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("paperless-ngx-ocr2"));
    assert!(stdout.contains("OCR CLI tool"));
}

/// Test Docker Compose with environment variables
#[test]
fn test_docker_compose_with_env() {
    let mut cmd = Command::new("docker-compose");
    cmd.args(&[
        "run",
        "--rm",
        "-e",
        "PAPERLESS_OCR_API_KEY=test-key",
        "paperless-ngx-ocr2",
        "--version"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker-compose run");
    
    assert!(
        output.status.success(),
        "Docker Compose with env failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test that the container has the correct architecture
#[test]
fn test_docker_container_architecture() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "uname",
        "-m"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    assert!(
        output.status.success(),
        "Docker container architecture check failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim();
    // Should be x86_64 or aarch64
    assert!(
        stdout == "x86_64" || stdout == "aarch64",
        "Unexpected architecture: {}",
        stdout
    );
}

/// Test container resource limits
#[test]
fn test_docker_container_resources() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "--memory=256m",
        "--cpus=0.5",
        "paperless-ngx-ocr2:test",
        "--help"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run with resource limits");
    
    assert!(
        output.status.success(),
        "Docker container with resource limits failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test container security (non-root user)
#[test]
fn test_docker_container_security() {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "run",
        "--rm",
        "paperless-ngx-ocr2:test",
        "id"
    ]);
    
    let output = cmd.output().expect("Failed to execute docker run");
    
    assert!(
        output.status.success(),
        "Docker container security check failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should not be running as root (uid=0)
    assert!(
        !stdout.contains("uid=0"),
        "Container is running as root: {}",
        stdout
    );
}
