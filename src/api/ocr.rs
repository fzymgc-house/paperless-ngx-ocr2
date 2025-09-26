//! Mistral AI OCR API client
//!
//! This module implements the OCR API client for Mistral AI.
//! Documentation: https://docs.mistral.ai/api/#ocr
//!
//! The OCR API requires a two-step process:
//! 1. Upload file via Files API (/v1/files)
//! 2. Process file via OCR API (/v1/ocr) using the file ID

use crate::api::MistralClient;
use crate::error::{Error, Result};
use crate::metrics::GLOBAL_METRICS;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// OCR request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRRequest {
    pub model: String,
    pub document: DocumentChunk,
}

/// Document chunk structure for OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    #[serde(rename = "type")]
    pub chunk_type: String,
    pub file_id: String,
}

impl OCRRequest {
    /// Create a new OCR request
    pub fn new(file_id: String) -> Self {
        Self { 
            model: "mistral-ocr-latest".to_string(),
            document: DocumentChunk {
                chunk_type: "file".to_string(),
                file_id,
            }
        }
    }

    /// Validate the OCR request
    pub fn validate(&self) -> Result<()> {
        if self.document.file_id.is_empty() {
            return Err(Error::Validation("File ID cannot be empty".to_string()));
        }

        if self.model != "mistral-ocr-latest" {
            return Err(Error::Validation("Invalid model for OCR processing".to_string()));
        }

        if self.document.chunk_type != "file" {
            return Err(Error::Validation("Invalid document type for OCR processing".to_string()));
        }

        Ok(())
    }
}

/// Page dimensions for OCR response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub dpi: i32,
    pub height: i32,
    pub width: i32,
}

/// Page structure for OCR response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub index: i32,
    pub markdown: String,
    pub images: Vec<String>,
    pub dimensions: Dimensions,
}

/// Usage information for OCR response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub pages_processed: i32,
    pub doc_size_bytes: i32,
}

/// OCR response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRResponse {
    pub pages: Vec<Page>,
    pub model: String,
    pub document_annotation: Option<String>,
    pub usage_info: UsageInfo,
}

impl OCRResponse {
    /// Get extracted text from the response
    pub fn get_extracted_text(&self) -> String {
        self.pages
            .iter()
            .map(|page| page.markdown.clone())
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    /// Validate the OCR response
    pub fn validate(&self) -> Result<()> {
        // Validate model field
        if self.model.is_empty() {
            return Err(Error::Validation("Response model cannot be empty".to_string()));
        }
        
        // Validate model name format
        if !self.model.starts_with("mistral-") {
            return Err(Error::Validation(format!(
                "Invalid model name format: expected 'mistral-*', got '{}'",
                self.model
            )));
        }

        // Validate pages array
        if self.pages.is_empty() {
            return Err(Error::Validation("Response must contain at least one page".to_string()));
        }

        // Validate each page structure and content
        for (i, page) in self.pages.iter().enumerate() {
            if page.index != i as i32 {
                return Err(Error::Validation(format!(
                    "Page index mismatch: expected {}, got {}",
                    i, page.index
                )));
            }
            
            // Validate page content
            if page.markdown.is_empty() {
                tracing::warn!("Page {} has empty markdown content", page.index);
            }
            
            // Validate dimensions if present
            if page.dimensions.width <= 0 || page.dimensions.height <= 0 {
                return Err(Error::Validation(format!(
                    "Invalid page dimensions: width={}, height={}",
                    page.dimensions.width, page.dimensions.height
                )));
            }
            
            // Validate DPI is reasonable
            if page.dimensions.dpi < 50 || page.dimensions.dpi > 600 {
                tracing::warn!("Unusual DPI value: {}", page.dimensions.dpi);
            }
        }

        // Validate usage info if present
        if self.usage_info.pages_processed != self.pages.len() as i32 {
            return Err(Error::Validation(format!(
                "Usage info pages_processed ({}) doesn't match actual pages ({})",
                self.usage_info.pages_processed, self.pages.len()
            )));
        }
        
        if self.usage_info.doc_size_bytes <= 0 {
            return Err(Error::Validation(format!(
                "Invalid document size in usage info: {} bytes",
                self.usage_info.doc_size_bytes
            )));
        }

        Ok(())
    }
}

/// OCR API client
pub struct OCRClient {
    client: MistralClient,
}

impl OCRClient {
    /// Create a new OCR API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Process a file with OCR
    pub async fn process_ocr(&self, file_id: &str) -> Result<OCRResponse> {
        let url = self.client.build_url("v1/ocr");
        
        self.client.log_request("POST", &url);

        // Create OCR request
        let ocr_request = OCRRequest::new(file_id.to_string());
        ocr_request.validate()?;

        // Get authorization headers
        let auth_headers = crate::api::auth::AuthHandler::new(
            crate::credentials::APICredentials::new(
                self.client.credentials.api_key.clone(),
                self.client.credentials.api_base_url.clone(),
            )?
        ).get_auth_headers()?;

        // Send request with retry logic and metrics
        let start_time = Instant::now();
        let response = self.client.execute_with_retry(|| {
            let client = self.client.client().clone();
            let url = url.clone();
            let auth_headers = auth_headers.clone();
            let ocr_request = ocr_request.clone();
            
            async move {
                let response = client
                    .post(&url)
                    .headers(auth_headers)
                    .json(&ocr_request)
                    .send()
                    .await
                    .map_err(Error::Network)?;
                
                MistralClient::handle_response(response).await
            }
        }).await;

        // Record metrics
        let duration = start_time.elapsed();
        match &response {
            Ok(_) => {
                GLOBAL_METRICS.record_success(duration, 0, 0).await;
            }
            Err(_) => {
                GLOBAL_METRICS.record_failure(duration).await;
            }
        }

        let response = response?;

        // Parse response
        let status = response.status().as_u16();
        let response_text = response.text().await
            .map_err(Error::Network)?;

        self.client.log_response(status, Some(response_text.len()));

        // Debug: Log the raw response for troubleshooting
        tracing::debug!("Raw OCR response: {}", response_text);

        let ocr_response: OCRResponse = serde_json::from_str(&response_text)
            .map_err(|e| Error::Api(format!("Failed to parse OCR response: {}", e)))?;

        ocr_response.validate()?;

        Ok(ocr_response)
    }
}
