//! File upload entity and validation

use crate::error::{Error, Result};
use mime_guess::MimeGuess;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    /// Path to the file to upload
    pub file_path: String,

    /// File size in bytes
    pub file_size: u64,

    /// MIME type of the file
    pub mime_type: String,

    /// Mistral AI file ID after upload
    pub file_id: Option<String>,

    /// File upload status
    pub upload_status: Option<String>,

    /// Whether file passes validation
    pub is_valid: bool,
}

impl FileUpload {
    /// Create a new FileUpload from a file path
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let path = file_path.as_ref();
        let file_path_str = path.to_string_lossy().to_string();

        // Check if file exists and is readable
        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", file_path_str))));
        }

        // Get file metadata
        let metadata = fs::metadata(path).map_err(Error::Io)?;

        let file_size = metadata.len();

        // Determine MIME type
        let mime_type = MimeGuess::from_path(path).first_or_octet_stream().to_string();

        let mut file_upload = Self { file_path: file_path_str, file_size, mime_type, file_id: None, upload_status: None, is_valid: false };

        // Validate the file
        file_upload.validate_file()?;
        file_upload.is_valid = true;

        Ok(file_upload)
    }

    /// Validate file according to data model rules
    pub fn validate_file(&self) -> Result<()> {
        // Validate file path exists and is readable
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Err(Error::Validation(format!("File does not exist: {}", self.file_path)));
        }

        // Validate file size (convert MB to bytes for comparison)
        let max_size_bytes = 100 * 1024 * 1024; // 100MB in bytes
        if self.file_size > max_size_bytes {
            return Err(Error::Validation(format!("File size ({:.2} MB) exceeds maximum allowed size (100 MB)", self.file_size as f64 / (1024.0 * 1024.0))));
        }

        // Validate MIME type
        let supported_types = ["application/pdf", "image/png", "image/jpeg", "image/jpg"];

        if !supported_types.contains(&self.mime_type.as_str()) {
            return Err(Error::Validation(format!("Unsupported file format: {}. Supported: pdf, png, jpg, jpeg", self.mime_type)));
        }

        // Validate file content by checking magic bytes
        self.validate_file_content()?;

        Ok(())
    }

    /// Validate file content by checking magic bytes
    fn validate_file_content(&self) -> Result<()> {
        let mut file = fs::File::open(&self.file_path).map_err(Error::Io)?;

        let mut buffer = [0; 8];
        use std::io::Read;
        let bytes_read = file.read(&mut buffer).map_err(Error::Io)?;

        if bytes_read < 4 {
            return Err(Error::Validation("File too small to determine format".to_string()));
        }

        // Check magic bytes for supported formats
        match &buffer[..4] {
            [0x25, 0x50, 0x44, 0x46] => {
                // PDF: %PDF - check for password protection
                self.check_pdf_password_protection()?;
                Ok(())
            }
            [0x89, 0x50, 0x4E, 0x47] => Ok(()), // PNG
            [0xFF, 0xD8, 0xFF, _] => Ok(()),    // JPEG
            _ => Err(Error::Validation(format!("File does not appear to be a valid PDF, PNG, or JPEG file: {}", self.file_path))),
        }
    }

    /// Set the file ID after successful upload
    pub fn set_file_id(&mut self, file_id: String) {
        self.file_id = Some(file_id);
    }

    /// Set the upload status
    pub fn set_upload_status(&mut self, status: String) {
        // Validate status value
        let valid_statuses = ["uploaded", "processing", "processed", "error"];
        if valid_statuses.contains(&status.as_str()) {
            self.upload_status = Some(status);
        }
    }

    /// Get file data as bytes
    pub fn read_file_data(&self) -> Result<Vec<u8>> {
        fs::read(&self.file_path).map_err(Error::Io)
    }

    /// Get filename from path
    pub fn get_filename(&self) -> String {
        Path::new(&self.file_path).file_name().and_then(|name| name.to_str()).unwrap_or("unknown").to_string()
    }

    /// Check if PDF is password-protected by looking for encryption dictionary
    fn check_pdf_password_protection(&self) -> Result<()> {
        use std::io::Read;

        let mut file = fs::File::open(&self.file_path).map_err(Error::Io)?;
        let mut buffer = [0; 8192]; // Read first 8KB
        let bytes_read = file.read(&mut buffer).map_err(Error::Io)?;

        // Convert to string, ignoring invalid UTF-8 sequences
        let content = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Check for common password protection indicators
        if content.contains("/Encrypt")
            || content.contains("/P -")
            || content.contains("/U ")
            || content.contains("/O ")
            || content.contains("/Filter/Standard")
        {
            return Err(Error::Validation("Password-protected PDF detected. Please provide an unprotected PDF file.".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_valid_pdf() {
        // Create a temporary PDF file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
        let temp_path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &temp_path).unwrap();

        let file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload for valid PDF");

        assert_eq!(file_upload.mime_type, "application/pdf");
        assert!(file_upload.is_valid);
        assert!(file_upload.file_size > 0);

        // Cleanup
        fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_new_nonexistent_file() {
        let result = FileUpload::new("nonexistent_file.pdf");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Io(_)));
    }

    #[test]
    fn test_magic_bytes_validation() {
        // Test PDF magic bytes
        let mut pdf_file = NamedTempFile::new().unwrap();
        pdf_file.write_all(b"%PDF-1.4\nValid PDF").unwrap();
        let pdf_path = pdf_file.path().with_extension("pdf");
        fs::copy(pdf_file.path(), &pdf_path).unwrap();

        let pdf_upload = FileUpload::new(&pdf_path);
        assert!(pdf_upload.is_ok());

        // Test invalid magic bytes with PDF extension
        let mut fake_pdf = NamedTempFile::new().unwrap();
        fake_pdf.write_all(b"Not a PDF file").unwrap();
        let fake_path = fake_pdf.path().with_extension("pdf");
        fs::copy(fake_pdf.path(), &fake_path).unwrap();

        let fake_result = FileUpload::new(&fake_path);
        assert!(fake_result.is_err());

        // Cleanup
        fs::remove_file(&pdf_path).ok();
        fs::remove_file(&fake_path).ok();
    }

    #[test]
    fn test_set_file_id() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"%PDF-1.4\nTest").unwrap();
        let temp_path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &temp_path).unwrap();

        let mut file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");

        assert!(file_upload.file_id.is_none());

        file_upload.set_file_id("file-123".to_string());
        assert_eq!(file_upload.file_id, Some("file-123".to_string()));

        // Cleanup
        fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_set_upload_status() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"%PDF-1.4\nTest").unwrap();
        let temp_path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &temp_path).unwrap();

        let mut file_upload = FileUpload::new(&temp_path).expect("Should create FileUpload");

        // Test valid status
        file_upload.set_upload_status("uploaded".to_string());
        assert_eq!(file_upload.upload_status, Some("uploaded".to_string()));

        // Test invalid status (should be ignored)
        file_upload.set_upload_status("invalid_status".to_string());
        assert_eq!(file_upload.upload_status, Some("uploaded".to_string()));

        // Cleanup
        fs::remove_file(&temp_path).ok();
    }
}
