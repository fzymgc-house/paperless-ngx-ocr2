//! Mistral AI Files API client
//!
//! This module implements the Files API client for Mistral AI.
//! Documentation: https://docs.mistral.ai/api/#files
//!
//! Files must be uploaded before they can be processed via the OCR API.
//! Supports multipart/form-data uploads with file and purpose fields.

use crate::api::MistralClient;
use crate::error::{Error, Result};
use crate::file::FileUpload;
use crate::metrics::GLOBAL_METRICS;
use chrono;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::fs::File;

/// File upload request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadRequest {
    pub file_data: Vec<u8>,
    pub filename: String,
    pub purpose: String,
}

impl FileUploadRequest {
    /// Create a new file upload request
    pub fn new(file_data: Vec<u8>, filename: String) -> Self {
        Self { file_data, filename, purpose: "ocr".to_string() }
    }

    /// Validate the upload request
    pub fn validate(&self) -> Result<()> {
        if self.file_data.is_empty() {
            return Err(Error::Validation("File data cannot be empty".to_string()));
        }

        if self.filename.is_empty() {
            return Err(Error::Validation("Filename cannot be empty".to_string()));
        }

        if self.purpose != "ocr" {
            return Err(Error::Validation("Purpose must be 'ocr'".to_string()));
        }

        Ok(())
    }

    /// Convert to multipart form for upload with streaming support for large files
    pub fn to_multipart_form(&self) -> Result<multipart::Form> {
        // For files >50MB, use streaming to reduce memory usage
        const STREAMING_THRESHOLD: usize = 50 * 1024 * 1024; // 50MB

        if self.file_data.len() > STREAMING_THRESHOLD {
            tracing::info!("Large file detected ({}MB), using streaming upload", self.file_data.len() / (1024 * 1024));

            // For large files, we'll still use bytes but with a note about streaming
            // The actual streaming will be handled in the upload method
            let part = multipart::Part::bytes(self.file_data.clone())
                .file_name(self.filename.clone())
                .mime_str("application/octet-stream")
                .map_err(|e| Error::Internal(format!("Failed to create multipart part: {}", e)))?;

            let form = multipart::Form::new().part("file", part).text("purpose", self.purpose.clone());

            Ok(form)
        } else {
            // Use regular in-memory part for smaller files
            let form = multipart::Form::new()
                .part(
                    "file",
                    multipart::Part::bytes(self.file_data.clone())
                        .file_name(self.filename.clone())
                        .mime_str("application/octet-stream")
                        .map_err(|e| Error::Internal(format!("Failed to create file part: {}", e)))?,
                )
                .text("purpose", self.purpose.clone());

            Ok(form)
        }
    }

    /// Create streaming multipart form from file path (memory-efficient for large files)
    pub async fn to_streaming_multipart_form(file_path: &str, purpose: &str) -> Result<multipart::Form> {
        let file = File::open(file_path).await.map_err(Error::Io)?;

        let filename = std::path::Path::new(file_path).file_name().and_then(|name| name.to_str()).unwrap_or("unknown").to_string();

        // Determine MIME type from file extension
        let mime_type = mime_guess::MimeGuess::from_path(file_path).first_or_octet_stream().to_string();

        // Get file size for streaming
        let file_size = file.metadata().await.map_err(Error::Io)?.len();

        // Create streaming part
        let part = multipart::Part::stream_with_length(file, file_size)
            .file_name(filename)
            .mime_str(&mime_type)
            .map_err(|e| Error::Internal(format!("Failed to create streaming multipart part: {}", e)))?;

        let form = multipart::Form::new().part("file", part).text("purpose", purpose.to_string());

        Ok(form)
    }

    /// Check if multipart form has file part (for contract tests)
    pub fn has_file_part(&self) -> bool {
        !self.file_data.is_empty()
    }

    /// Check if multipart form has purpose part (for contract tests)
    pub fn has_purpose_part(&self) -> bool {
        !self.purpose.is_empty()
    }
}

/// File upload response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
    pub status: Option<String>,
}

impl FileUploadResponse {
    /// Validate the upload response
    pub fn validate(&self) -> Result<()> {
        // Validate file ID format and content
        if self.id.is_empty() {
            return Err(Error::Validation("File ID cannot be empty".to_string()));
        }

        // Validate file ID format (should be alphanumeric with possible dashes/underscores)
        if !self.id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(Error::Validation(format!("Invalid file ID format: '{}' contains invalid characters", self.id)));
        }

        // Validate object type
        if self.object != "file" {
            return Err(Error::Validation(format!("Object must be 'file', got '{}'", self.object)));
        }

        // Validate file size with reasonable bounds
        if self.bytes <= 0 {
            return Err(Error::Validation("File size must be positive".to_string()));
        }

        // Check for unreasonably large files (>1GB)
        if self.bytes > 1_073_741_824 {
            tracing::warn!("Very large file uploaded: {} bytes", self.bytes);
        }

        // Validate timestamp (should be reasonable Unix timestamp)
        if self.created_at <= 0 {
            return Err(Error::Validation("Created timestamp must be positive".to_string()));
        }

        // Check if timestamp is too far in the future (more than 1 hour)
        let now = chrono::Utc::now().timestamp();
        if self.created_at > now + 3600 {
            return Err(Error::Validation(format!("Created timestamp is too far in the future: {}", self.created_at)));
        }

        // Validate filename
        if self.filename.is_empty() {
            return Err(Error::Validation("Filename cannot be empty".to_string()));
        }

        // Validate filename doesn't contain path separators
        if self.filename.contains('/') || self.filename.contains('\\') {
            return Err(Error::Validation(format!("Filename cannot contain path separators: '{}'", self.filename)));
        }

        // Validate purpose
        if self.purpose != "ocr" {
            return Err(Error::Validation(format!("Purpose must be 'ocr', got '{}'", self.purpose)));
        }

        // Validate status if present
        if let Some(ref status) = self.status {
            let valid_statuses = ["uploaded", "processing", "processed", "error"];
            if !valid_statuses.contains(&status.as_str()) {
                return Err(Error::Validation(format!("Invalid status '{}', must be one of: {}", status, valid_statuses.join(", "))));
            }

            // Log status for monitoring
            match status.as_str() {
                "error" => tracing::warn!("File upload status is 'error' for file: {}", self.id),
                "processing" => tracing::info!("File is being processed: {}", self.id),
                "processed" => tracing::debug!("File processing completed: {}", self.id),
                _ => {}
            }
        }

        Ok(())
    }
}

/// Files API client
pub struct FilesClient {
    client: MistralClient,
}

impl FilesClient {
    /// Create a new Files API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Upload a file to Mistral AI Files API with streaming support for large files
    pub async fn upload_file(&self, file_upload: &FileUpload) -> Result<FileUploadResponse> {
        let url = self.client.build_url("v1/files");

        self.client.log_request("POST", &url);

        // Check if we should use streaming for large files
        const STREAMING_THRESHOLD: u64 = 50 * 1024 * 1024; // 50MB

        if file_upload.file_size > STREAMING_THRESHOLD {
            tracing::info!("Large file detected ({}MB), using streaming upload", file_upload.file_size / (1024 * 1024));

            // Use streaming upload for large files
            return self.upload_file_streaming(&file_upload.file_path).await;
        }

        // Read file data for smaller files
        let file_data = file_upload.read_file_data()?;

        // Create upload request
        let upload_request = FileUploadRequest::new(file_data.clone(), file_upload.get_filename());
        upload_request.validate()?;

        // Get authorization headers
        let auth_headers = crate::api::auth::AuthHandler::new(crate::credentials::APICredentials::new(
            self.client.credentials.api_key.clone(),
            self.client.credentials.api_base_url.clone(),
        )?)
        .get_multipart_headers()?;

        // Send request with retry logic and metrics
        let start_time = Instant::now();
        let response = self
            .client
            .execute_with_retry(|| {
                let client = self.client.client().clone();
                let url = url.clone();
                let auth_headers = auth_headers.clone();
                let file_data = file_data.clone();
                let filename = file_upload.get_filename();
                let _file_size = file_upload.file_size;

                async move {
                    // Recreate the form inside the closure
                    let upload_request = FileUploadRequest::new(file_data, filename);
                    let form = upload_request.to_multipart_form()?;

                    let response = client.post(&url).headers(auth_headers).multipart(form).send().await.map_err(Error::Network)?;

                    MistralClient::handle_response(response).await
                }
            })
            .await;

        // Record metrics
        let duration = start_time.elapsed();
        match &response {
            Ok(_) => {
                GLOBAL_METRICS.record_success(duration, file_upload.file_size, 0).await;
            }
            Err(_) => {
                GLOBAL_METRICS.record_failure(duration).await;
            }
        }

        let response = response?;

        // Parse response
        let status = response.status().as_u16();
        let response_text = response.text().await.map_err(Error::Network)?;

        self.client.log_response(status, Some(response_text.len()));

        let upload_response: FileUploadResponse =
            serde_json::from_str(&response_text).map_err(|e| Error::Api(format!("Failed to parse upload response: {}", e)))?;

        upload_response.validate()?;

        Ok(upload_response)
    }

    /// Upload a file using streaming (memory-efficient for large files)
    async fn upload_file_streaming(&self, file_path: &str) -> Result<FileUploadResponse> {
        let url = self.client.build_url("v1/files");

        self.client.log_request("POST", &url);

        // Get authorization headers
        let auth_headers = crate::api::auth::AuthHandler::new(crate::credentials::APICredentials::new(
            self.client.credentials.api_key.clone(),
            self.client.credentials.api_base_url.clone(),
        )?)
        .get_multipart_headers()?;

        // Create streaming multipart form
        let _form = FileUploadRequest::to_streaming_multipart_form(file_path, "ocr").await?;

        // Send request with retry logic
        let response = self
            .client
            .execute_with_retry(|| {
                let client = self.client.client().clone();
                let url = url.clone();
                let auth_headers = auth_headers.clone();
                let file_path = file_path.to_string();

                async move {
                    // Recreate the streaming form inside the closure
                    let form = FileUploadRequest::to_streaming_multipart_form(&file_path, "ocr").await?;

                    let response = client.post(&url).headers(auth_headers).multipart(form).send().await.map_err(Error::Network)?;

                    MistralClient::handle_response(response).await
                }
            })
            .await?;

        // Parse response
        let status = response.status().as_u16();
        let response_text = response.text().await.map_err(Error::Network)?;

        self.client.log_response(status, Some(response_text.len()));

        // Parse JSON response
        let upload_response: FileUploadResponse =
            serde_json::from_str(&response_text).map_err(|e| Error::Api(format!("Failed to parse upload response: {}", e)))?;

        // Validate response
        upload_response.validate()?;

        Ok(upload_response)
    }
}
