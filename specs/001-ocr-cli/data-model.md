# Data Model: OCR CLI Tool

## Configuration Entity

### Fields
- `api_key`: String (required) - Mistral AI API key
- `api_base_url`: String (required, default: "https://api.mistral.ai") - Mistral AI API base URL
- `timeout_seconds`: u64 (optional, default: 30) - Request timeout in seconds
- `max_file_size_mb`: u64 (optional, default: 100) - Maximum file size in MB
- `log_level`: String (optional, default: "info") - Logging level (error, warn, info, debug, trace)

### Validation Rules
- `api_key` must not be empty
- `api_base_url` must be a valid URL
- `timeout_seconds` must be between 1 and 300
- `max_file_size_mb` must be between 1 and 100
- `log_level` must be one of: error, warn, info, debug, trace

### State Transitions
- Configuration loaded from file → validated → ready for use
- Environment variable override → merged with file config → validated

## File Upload Entity

### Fields
- `file_path`: String (required) - Path to the file to upload
- `file_size`: u64 (required) - File size in bytes
- `mime_type`: String (required) - MIME type of the file
- `file_id`: Option<String> (optional) - Mistral AI file ID after upload
- `upload_status`: Option<String> (optional) - File upload status (uploaded, processing, processed, error)
- `is_valid`: bool (computed) - Whether file passes validation

### Validation Rules
- `file_path` must exist and be readable
- `file_size` must not exceed max_file_size_mb * 1024 * 1024 (up to 100MB)
- `mime_type` must be one of: application/pdf, image/png, image/jpeg, image/jpg
- `file_id` must not be empty (if present)
- `upload_status` must be one of: uploaded, processing, processed, error (if present)

### State Transitions
- File path provided → file read → validation → upload to Mistral AI Files API → get file ID → ready for OCR API call

## OCR Result Entity

### Fields
- `extracted_text`: String (required) - The OCR extracted text from choices[0].message.content
- `file_id`: String (required) - Mistral AI file ID used for OCR
- `model`: String (required) - Model used for OCR processing
- `usage`: Option<Map<String, Integer>> (optional) - Token usage information
- `file_name`: String (required) - Original file name
- `file_size`: u64 (required) - Original file size in bytes
- `timestamp`: DateTime<Utc> (required) - When OCR was performed

### Validation Rules
- `extracted_text` must not be empty (empty string indicates no text found)
- `file_id` must not be empty
- `model` must not be empty
- `file_name` must not be empty
- `file_size` must be positive

### State Transitions
- API request sent → processing → result received → validation → ready for output

## API Credentials Entity

### Fields
- `api_key`: String (required) - Mistral AI API key
- `api_base_url`: String (required) - Mistral AI API base URL
- `is_valid`: bool (computed) - Whether credentials are valid

### Validation Rules
- `api_key` must not be empty and must not contain whitespace
- `api_base_url` must be a valid HTTPS URL
- `api_base_url` must point to Mistral AI API (https://api.mistral.ai)

### State Transitions
- Credentials provided → validation → ready for API calls

## Error Entity

### Fields
- `error_type`: String (required) - Type of error (validation, network, api, file_io)
- `message`: String (required) - Human-readable error message
- `details`: Option<String> (optional) - Additional error details
- `exit_code`: i32 (required) - CLI exit code
- `timestamp`: DateTime<Utc> (required) - When error occurred

### Validation Rules
- `error_type` must be one of: validation, network, api, file_io, internal
- `message` must not be empty
- `exit_code` must be between 1 and 255
- `exit_code` must follow constitution standards (2=validation, 3=I/O, 4=config, 5=internal)

### State Transitions
- Error condition detected → error created → logged → user notified → process exits
