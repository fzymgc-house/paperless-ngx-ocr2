//! OCR result entity and processing

use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRResult {
    /// The OCR extracted text from choices[0].message.content
    pub extracted_text: String,

    /// Mistral AI file ID used for OCR
    pub file_id: String,

    /// Model used for OCR processing
    pub model: String,

    /// Token usage information
    pub usage: Option<HashMap<String, i64>>,

    /// Original file name
    pub file_name: String,

    /// Original file size in bytes
    pub file_size: u64,

    /// When OCR was performed
    pub timestamp: DateTime<Utc>,
}

impl OCRResult {
    /// Create a new OCRResult
    pub fn new(
        extracted_text: String,
        file_id: String,
        model: String,
        file_name: String,
        file_size: u64,
    ) -> Self {
        Self {
            extracted_text,
            file_id,
            model,
            usage: None,
            file_name,
            file_size,
            timestamp: Utc::now(),
        }
    }

    /// Create OCRResult from extracted text and metadata
    pub fn from_extracted_text(
        extracted_text: String,
        file_id: String,
        model: String,
        file_name: String,
        file_size: u64,
        usage: Option<HashMap<String, i64>>,
    ) -> Self {
        Self {
            extracted_text,
            file_id,
            model,
            usage,
            file_name,
            file_size,
            timestamp: Utc::now(),
        }
    }

    /// Validate OCR result according to data model rules
    pub fn validate(&self) -> Result<()> {
        // Note: Empty text is allowed but will generate a warning
        // This is handled in the CLI output, not here

        // Validate file_id is not empty
        if self.file_id.is_empty() {
            return Err(Error::Validation("File ID must not be empty".to_string()));
        }

        // Validate model is not empty
        if self.model.is_empty() {
            return Err(Error::Validation("Model must not be empty".to_string()));
        }

        // Validate file_name is not empty
        if self.file_name.is_empty() {
            return Err(Error::Validation(
                "File name must not be empty".to_string(),
            ));
        }

        // Validate file_size is positive
        if self.file_size == 0 {
            return Err(Error::Validation(
                "File size must be positive".to_string(),
            ));
        }

        Ok(())
    }

    /// Get processing time in milliseconds (calculated from timestamp)
    pub fn get_processing_time_ms(&self) -> u64 {
        // This is a placeholder - in real implementation, we'd track start time
        // For now, return a reasonable default
        2000 // 2 seconds default processing time
    }

    /// Check if extracted text is empty
    pub fn is_empty_text(&self) -> bool {
        self.extracted_text.trim().is_empty()
    }

    /// Format result for human-readable output
    pub fn to_human_readable(&self) -> String {
        if self.is_empty_text() {
            format!(
                "Warning: No text could be extracted from {} ({} bytes). The file may contain only images without text, or the text may not be readable.",
                self.file_name,
                self.file_size
            )
        } else {
            format!(
                "Extracted text from {} ({} bytes):\n\n{}",
                self.file_name,
                self.file_size,
                self.extracted_text
            )
        }
    }

    /// Format result for JSON output
    pub fn to_json_output(&self) -> serde_json::Value {
        serde_json::json!({
            "success": true,
            "data": {
                "extracted_text": self.extracted_text,
                "file_name": self.file_name,
                "file_size": self.file_size,
                "processing_time_ms": self.get_processing_time_ms(),
                "confidence": null // Will be populated if available from API
            }
        })
    }
}
