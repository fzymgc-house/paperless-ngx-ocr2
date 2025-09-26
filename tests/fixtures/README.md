# Test Fixtures

This directory contains example test input files for both happy path and failure scenarios.

## Happy Path Files

### `sample.pdf`
- Valid PDF file with simple text content: "Hello World Sample Text"
- Small size (~300 bytes) for fast testing
- Contains proper PDF structure and text objects
- Use for testing successful OCR workflows

### `sample.png`
- Valid PNG file (1x1 pixel minimal image)
- Proper PNG signature and structure
- Small size for fast testing
- Use for testing image OCR workflows

## Failure Path Files

### `invalid.txt`
- Plain text file with .txt extension
- Should be rejected due to unsupported file format
- Use for testing file format validation

### `corrupted.pdf`
- File with .pdf extension but invalid PDF content
- Should be rejected by magic byte validation
- Use for testing file content validation

## Usage in Tests

```rust
use std::path::Path;

// Happy path testing
let pdf_path = Path::new("tests/fixtures/sample.pdf");
let png_path = Path::new("tests/fixtures/sample.png");

// Error path testing  
let invalid_path = Path::new("tests/fixtures/invalid.txt");
let corrupted_path = Path::new("tests/fixtures/corrupted.pdf");
```

## File Specifications

- **sample.pdf**: 274 bytes, contains "Hello World Sample Text"
- **sample.png**: ~50 bytes, 1x1 pixel transparent PNG
- **invalid.txt**: Plain text, unsupported format
- **corrupted.pdf**: Text content with PDF extension, invalid magic bytes
