# Research Findings: OCR CLI Tool

## Mistral AI File Upload API Specifications

### Decision: Use Mistral AI Files API for all file uploads
**Rationale**: Based on the official Mistral AI API documentation at [https://docs.mistral.ai/api/](https://docs.mistral.ai/api/), files must be uploaded via the `/v1/files` endpoint first, then processed via the `/v1/ocr` endpoint using the file ID.
**Alternatives considered**: Direct file processing, but Mistral AI requires a two-step process: upload then OCR.

### Decision: File size limit of 100MB for direct upload
**Rationale**: Updated requirement to support files up to 100MB. Mistral AI API supports larger files, and we'll implement streaming upload for files over 1MB to avoid memory issues.
**Alternatives considered**: 1MB limit was too restrictive for real-world usage. 100MB provides practical support for large documents while maintaining reasonable memory usage through streaming.

### Decision: Use Authorization Bearer header for authentication
**Rationale**: Based on the official Mistral AI API documentation, authentication uses `Authorization: Bearer <api_key>` header format.
**Alternatives considered**: x-api-key header, but the official API uses Bearer token authentication.

## Mistral AI OCR API Response Format

### Decision: Expect chat completion format response with extracted text in message content
**Rationale**: Based on the official Mistral AI API documentation, the OCR API returns a chat completion format with the extracted text in `choices[0].message.content`.
**Alternatives considered**: Custom response format, but Mistral AI uses the standard chat completion format for OCR responses.

### Decision: Processing time varies (typically 2-10 seconds)
**Rationale**: OCR processing time depends on file size and complexity, typically 2-10 seconds for standard documents.
**Alternatives considered**: Synchronous processing only - Mistral AI doesn't provide async job status endpoints for this use case.

## Supported File Formats

### Decision: Support PDF, PNG, JPG, JPEG formats
**Rationale**: These are the most common formats supported by Mistral AI OCR API based on documentation.
**Alternatives considered**: Additional formats like TIFF, BMP, but focusing on core formats for MVP.

### Decision: Validate file extensions and MIME types
**Rationale**: Pre-validate files before API calls to provide better error messages and avoid unnecessary API usage.
**Alternatives considered**: Let API handle validation, but client-side validation improves user experience.

## Rate Limiting and Error Handling

### Decision: Implement exponential backoff for rate limiting
**Rationale**: Mistral AI has rate limits (specific limits not publicly documented), so implement standard retry logic.
**Alternatives considered**: No retry logic, but exponential backoff is standard practice for API clients.

### Decision: Handle common HTTP status codes (400, 401, 429, 500)
**Rationale**: Standard API error handling for validation errors, authentication failures, rate limiting, and server errors.
**Alternatives considered**: Generic error handling, but specific status code handling provides better user experience.

## Mistral AI API Workflow

### Decision: Two-step process: upload file then process with OCR
**Rationale**: Based on the official Mistral AI API documentation, the workflow is: 1) Upload file via `/v1/files` endpoint, 2) Process file via `/v1/ocr` endpoint using the file ID.
**Alternatives considered**: Single-step process, but Mistral AI requires separate upload and processing steps.

### Decision: Use multipart/form-data for file uploads
**Rationale**: The official Mistral AI Files API expects file uploads via multipart/form-data with the file and purpose fields.
**Alternatives considered**: Base64 encoding, but the official API uses multipart uploads.

## Configuration and 12-Factor Compliance

### Decision: Use TOML configuration with environment variable overrides
**Rationale**: TOML provides human-readable configuration, environment variables allow 12-factor compliance for deployment.
**Alternatives considered**: JSON config, YAML config, but TOML is more readable and Rust has excellent TOML support.

### Decision: Default configuration file location: ~/.config/paperless-ngx-ocr2/config.toml
**Rationale**: Follows XDG Base Directory Specification for user configuration files.
**Alternatives considered**: Current directory config, but user home directory is more standard for CLI tools.

### Decision: Use official Mistral AI API base URL: https://api.mistral.ai
**Rationale**: Based on the official Mistral AI API documentation, the base URL is `https://api.mistral.ai` with endpoints `/v1/files` and `/v1/ocr`.
**Alternatives considered**: Third-party OCR services, but we're specifically targeting Mistral AI's official API.

## Performance and Resource Management

### Decision: Stream file reading for memory efficiency with 100MB support
**Rationale**: Large files up to 100MB could consume significant memory if loaded entirely into RAM. Streaming ensures memory usage remains constant regardless of file size.
**Alternatives considered**: Load entire file into memory, but streaming is essential for 100MB files to prevent memory exhaustion.

### Decision: Set 30-second timeout for API requests
**Rationale**: OCR processing can take time, but 30 seconds provides reasonable upper bound for user experience.
**Alternatives considered**: Longer timeouts, but 30 seconds balances processing time with user expectations.

## Security Considerations

### Decision: Never log API keys in debug output
**Rationale**: API keys are sensitive and should not appear in logs or error messages.
**Alternatives considered**: Logging API keys for debugging, but security best practice is to never log credentials.

### Decision: Validate file paths to prevent directory traversal
**Rationale**: Prevent security issues with malicious file paths.
**Alternatives considered**: Trust user input, but file path validation is essential for security.

## Containerization Requirements

### Decision: Multi-architecture container image support
**Rationale**: Support for both AMD64 and ARM64 architectures to enable deployment on various platforms including Apple Silicon and ARM-based cloud instances.
**Alternatives considered**: Single architecture, but multi-arch provides broader deployment compatibility.

### Decision: Use Alpine Linux as base image
**Rationale**: Alpine Linux provides a minimal, secure base image with small size suitable for CLI tools.
**Alternatives considered**: Ubuntu/Debian, but Alpine provides better security posture and smaller image size for CLI applications.

### Decision: Static binary compilation for container deployment
**Rationale**: Static Rust binaries eliminate runtime dependencies and reduce container size and attack surface.
**Alternatives considered**: Dynamic linking, but static compilation provides better portability and security.
