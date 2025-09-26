//! CLI command implementations

use crate::api::{files::FilesClient, ocr::OCRClient, MistralClient};
use crate::config::Config;
use crate::credentials::APICredentials;
use crate::error::{Error, Result};
use crate::file::FileUpload;
use crate::ocr::OCRResult;
use std::path::Path;

/// Process OCR command
pub async fn process_ocr_command(
    input_file_path: &str,
    app_config: &Config,
    enable_json_output: bool,
    enable_verbose_logging: bool,
) -> Result<String> {
    if enable_verbose_logging {
        tracing::info!("Processing OCR command for file: {}", input_file_path);
    }

    // Validate file exists and is supported format
    let file_upload = FileUpload::new(input_file_path)?;

    if enable_verbose_logging {
        tracing::debug!(
            "File validation passed: {} ({} bytes, {})",
            file_upload.get_filename(),
            file_upload.file_size,
            file_upload.mime_type
        );
    }

    // Check file size against configuration
    let max_size_bytes = app_config.max_file_size_mb * 1024 * 1024;
    if file_upload.file_size > max_size_bytes {
        return Err(Error::Validation(format!(
            "File size ({:.2} MB) exceeds maximum allowed size ({} MB)",
            file_upload.file_size as f64 / (1024.0 * 1024.0),
            app_config.max_file_size_mb
        )));
    }

    // Create API credentials and client
    let api_credentials = APICredentials::from_config(app_config)?;
    let mistral_client = MistralClient::new(api_credentials, app_config.timeout_seconds)?;

    if enable_verbose_logging {
        tracing::debug!("API client initialized");
    }

    // Upload file to Mistral AI Files API
    let files_client = FilesClient::new(mistral_client.clone());
    let upload_response = files_client.upload_file(&file_upload).await?;

    if enable_verbose_logging {
        tracing::info!("File uploaded successfully: {}", upload_response.id);
    }

    // Process with OCR API
    let ocr_client = OCRClient::new(mistral_client);
    let ocr_response = ocr_client.process_ocr(&upload_response.id).await?;

    if enable_verbose_logging {
        tracing::info!("OCR processing completed");
    }

    // Create result from API response
    let result = OCRResult::from_extracted_text(
        ocr_response.get_extracted_text(),
        upload_response.id,
        ocr_response.model,
        file_upload.get_filename(),
        file_upload.file_size,
        {
            let mut usage_map = std::collections::HashMap::new();
            usage_map.insert(
                "pages_processed".to_string(),
                ocr_response.usage_info.pages_processed as i64,
            );
            usage_map.insert(
                "doc_size_bytes".to_string(),
                ocr_response.usage_info.doc_size_bytes as i64,
            );
            Some(usage_map)
        },
    );

    // Format output based on user preference
    let output = if enable_json_output {
        serde_json::to_string_pretty(&result.to_json_output())
            .map_err(|e| Error::Internal(format!("Failed to serialize JSON: {}", e)))?
    } else {
        result.to_human_readable()
    };

    Ok(output)
}

/// Validate input file path and format
pub fn validate_file_path(input_file_path: &str) -> Result<()> {
    let file_path = Path::new(input_file_path);

    // Check if file exists
    if !file_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", input_file_path),
        )));
    }

    // Check if it's a file (not directory)
    if !file_path.is_file() {
        return Err(Error::Validation(format!(
            "Path is not a file: {}",
            input_file_path
        )));
    }

    // Check file extension
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("pdf") | Some("png") | Some("jpg") | Some("jpeg") => Ok(()),
        Some(ext) => Err(Error::Validation(format!(
            "Unsupported file format: .{}. Supported formats: pdf, png, jpg, jpeg",
            ext
        ))),
        None => Err(Error::Validation(
            "File has no extension. Supported formats: pdf, png, jpg, jpeg".to_string(),
        )),
    }
}

/// Show version information
pub fn show_version() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Show help information with examples
pub fn show_help_with_examples() -> String {
    format!(
        r#"{} {}

USAGE:
    paperless-ngx-ocr2 [OPTIONS] --file <FILE>

EXAMPLES:
    # Extract text from a PDF file
    paperless-ngx-ocr2 --file document.pdf --api-key your-api-key

    # Extract text and output as JSON
    paperless-ngx-ocr2 --file image.png --api-key your-api-key --json

    # Use environment variables for configuration
    export PAPERLESS_OCR_API_KEY=your-api-key
    paperless-ngx-ocr2 --file document.pdf

    # Enable verbose logging
    paperless-ngx-ocr2 --file document.pdf --api-key your-api-key --verbose

    # Use custom API endpoint
    paperless-ngx-ocr2 --file document.pdf --api-key your-api-key --api-base-url https://custom.api.url

SUPPORTED FILE FORMATS:
    - PDF files (.pdf) up to 100MB
    - PNG images (.png) up to 100MB  
    - JPEG images (.jpg, .jpeg) up to 100MB

CONFIGURATION:
    Configuration can be provided via:
    1. Command-line arguments (highest priority)
    2. Environment variables
    3. TOML configuration file at ~/.config/paperless-ngx-ocr2/config.toml

ENVIRONMENT VARIABLES:
    PAPERLESS_OCR_API_KEY          Mistral AI API key
    PAPERLESS_OCR_API_BASE_URL     API base URL (default: https://api.mistral.ai)
    PAPERLESS_OCR_TIMEOUT          Request timeout in seconds (default: 30)
    PAPERLESS_OCR_MAX_FILE_SIZE    Maximum file size in MB (default: 100)
    PAPERLESS_OCR_LOG_LEVEL        Log level (default: info)

EXIT CODES:
    0    Success
    2    Validation error (invalid file, configuration)
    3    I/O error (file read/write issues)
    4    Configuration error (missing API key, invalid config)
    5    Internal error (API errors, network issues)
"#,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
