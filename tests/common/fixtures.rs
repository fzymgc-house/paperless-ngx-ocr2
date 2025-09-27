//! Test fixture management utilities
//! 
//! This module provides utilities for creating and managing test files,
//! both temporary and fixture-based.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// A test file that automatically cleans up on drop
#[derive(Debug)]
pub struct TestFile {
    path: PathBuf,
    is_temporary: bool,
}

impl TestFile {
    /// Creates a new TestFile from an existing path (fixture)
    pub fn fixture(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            is_temporary: false,
        }
    }

    /// Creates a new TestFile from a temporary path
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            is_temporary: true,
        }
    }

    /// Gets the path to the test file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the path as a string
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }

    /// Checks if the file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Gets the file size in bytes
    pub fn size(&self) -> u64 {
        fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
    }

    /// Reads the file content
    pub fn read_content(&self) -> Result<Vec<u8>, std::io::Error> {
        fs::read(&self.path)
    }

    /// Reads the file content as a string
    pub fn read_string(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(&self.path)
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        // Only clean up temporary files, not fixtures
        if self.is_temporary && self.path.exists() {
            fs::remove_file(&self.path).ok();
        }
    }
}

/// Creates a test PDF file with the given content
pub fn create_test_pdf(content: &str) -> TestFile {
    let pdf_content = format!("%PDF-1.4\n{}", content);
    create_pdf_from_bytes(pdf_content.as_bytes())
}

/// Creates a test PDF file from raw bytes
pub fn create_pdf_from_bytes(content: &[u8]) -> TestFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(content).unwrap();
    let path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &path).unwrap();
    TestFile::new(path)
}

/// Creates a test PNG file with minimal valid PNG content
pub fn create_test_png() -> TestFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Minimal PNG header + basic structure
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // width: 1
        0x00, 0x00, 0x00, 0x01, // height: 1
        0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, etc.
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x00, // IDAT chunk length
        0x49, 0x44, 0x41, 0x54, // IDAT
        0x08, 0x99, 0x01, 0x01, 0x00, 0x00, 0x00, // compressed data
        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND chunk length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ];
    temp_file.write_all(&png_data).unwrap();
    let path = temp_file.path().with_extension("png");
    fs::copy(temp_file.path(), &path).unwrap();
    TestFile::new(path)
}

/// Creates a large test PDF file for performance testing
pub fn create_large_test_pdf(size_mb: usize) -> TestFile {
    let header = b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n";
    let content_size = size_mb * 1024 * 1024 - header.len();
    let content = "x".repeat(content_size);
    let full_content = format!("{}{}", String::from_utf8_lossy(header), content);
    create_pdf_from_bytes(full_content.as_bytes())
}

/// Creates a corrupted PDF file (invalid magic bytes)
pub fn create_corrupted_pdf() -> TestFile {
    let content = "This is not a valid PDF file content";
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(content.as_bytes()).unwrap();
    let path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &path).unwrap();
    TestFile::new(path)
}

/// Creates an invalid file (wrong extension)
pub fn create_invalid_file() -> TestFile {
    let content = "This is a text file with .txt extension";
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(content.as_bytes()).unwrap();
    let path = temp_file.path().with_extension("txt");
    fs::copy(temp_file.path(), &path).unwrap();
    TestFile::new(path)
}

/// Gets a reference to a test fixture
pub fn get_fixture(name: &str) -> TestFile {
    TestFile::fixture(&format!("tests/fixtures/{}", name))
}

/// Available test fixtures
pub mod fixtures {
    use super::get_fixture;

    /// Valid PDF fixture
    pub fn sample_pdf() -> crate::TestFile {
        get_fixture("sample.pdf")
    }

    /// Valid PNG fixture
    pub fn sample_png() -> crate::TestFile {
        get_fixture("sample.png")
    }

    /// Corrupted PDF fixture
    pub fn corrupted_pdf() -> crate::TestFile {
        get_fixture("corrupted.pdf")
    }

    /// Invalid text file fixture
    pub fn invalid_txt() -> crate::TestFile {
        get_fixture("invalid.txt")
    }

    /// Test itinerary PDF fixture
    pub fn test_itinerary_pdf() -> crate::TestFile {
        get_fixture("test-itinerary.pdf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_pdf() {
        let test_file = create_test_pdf("Test content");
        assert!(test_file.exists());
        assert!(test_file.size() > 0);
        let content = test_file.read_string().unwrap();
        assert!(content.contains("Test content"));
    }

    #[test]
    fn test_create_test_png() {
        let test_file = create_test_png();
        assert!(test_file.exists());
        assert!(test_file.size() > 0);
        let content = test_file.read_content().unwrap();
        assert!(content.starts_with(&[0x89, 0x50, 0x4E, 0x47])); // PNG signature
    }

    #[test]
    fn test_fixture_access() {
        let fixture = fixtures::sample_pdf();
        assert!(fixture.exists());
    }

    #[test]
    fn test_auto_cleanup() {
        let temp_path = {
            let test_file = create_test_pdf("Temp content");
            test_file.path().to_path_buf()
        }; // test_file goes out of scope here
        
        // File should be cleaned up
        assert!(!temp_path.exists());
    }
}
