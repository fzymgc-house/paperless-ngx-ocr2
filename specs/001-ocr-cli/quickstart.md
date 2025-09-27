# Quickstart Guide: OCR CLI Tool

## Installation

### Option 1: From Source

```bash
# Clone the repository
git clone <repository-url>
cd paperless-ngx-ocr2

# Build the tool
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Option 2: Using Docker

```bash
# Pull the multi-architecture image
docker pull ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest

# Run directly
docker run --rm -v $(pwd):/workspace ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest --file /workspace/document.pdf

# Create an alias for easier usage
alias paperless-ocr='docker run --rm -v $(pwd):/workspace ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest'
```

## Configuration

### Native Installation

Create a configuration file at `~/.config/paperless-ngx-ocr2/config.toml`:

### Docker Usage

For Docker usage, mount a config file or use environment variables:

```toml
api_key = "your_mistral_ai_api_key_here"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 30
max_file_size_mb = 100
log_level = "info"
```

## Basic Usage

### Extract text from a PDF file

```bash
./target/release/paperless-ngx-ocr2 --file document.pdf
```

### Extract text from an image file

```bash
./target/release/paperless-ngx-ocr2 --file image.png
```

### Use custom API key and base URL

```bash
# Native
./target/release/paperless-ngx-ocr2 \
  --file document.pdf \
  --api-key "your_api_key" \
  --api-base-url "https://api.mistral.ai"

# Docker
docker run --rm -v $(pwd):/workspace \
  ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest \
  --file /workspace/document.pdf \
  --api-key "your_api_key"
```

### Get JSON output

```bash
./target/release/paperless-ngx-ocr2 --file document.pdf --json
```

### Verbose logging

```bash
./target/release/paperless-ngx-ocr2 --file document.pdf --verbose
```

## Environment Variables

You can override configuration using environment variables:

```bash
export PAPERLESS_OCR_API_KEY="your_api_key"
export PAPERLESS_OCR_API_BASE_URL="https://api.mistral.ai"
export PAPERLESS_OCR_TIMEOUT="60"
export PAPERLESS_OCR_MAX_FILE_SIZE="100"

./target/release/paperless-ngx-ocr2 --file document.pdf
```

## Supported File Formats

- PDF files (.pdf) up to 100MB
- PNG images (.png) up to 100MB
- JPEG images (.jpg, .jpeg) up to 100MB

**Note**: All files are uploaded to Mistral AI's Files API first, then processed via their OCR API using the file ID.

## Error Handling

The tool provides clear error messages for common issues:

- **File not found**: "Error: File 'document.pdf' not found"
- **Invalid file format**: "Error: Unsupported file format. Supported: pdf, png, jpg, jpeg"
- **File too large**: "Error: File size (150MB) exceeds maximum allowed size (100MB)"
- **API authentication failed**: "Error: Authentication failed. Check your API key"
- **Network error**: "Error: Failed to connect to API endpoint"
- **File upload error**: "Error: Failed to upload file to Mistral AI Files API"
- **File processing error**: "Error: Failed to process file with Mistral AI OCR API"

## Exit Codes

- `0`: Success
- `2`: Validation error (invalid file, configuration)
- `3`: I/O error (file read/write issues)
- `4`: Configuration error (missing API key, invalid config)
- `5`: Internal error (unexpected errors)

## Examples

### Process multiple files

```bash
for file in *.pdf; do
  echo "Processing $file..."
  ./target/release/paperless-ngx-ocr2 --file "$file" > "${file%.pdf}.txt"
done
```

### Save output to file

```bash
./target/release/paperless-ngx-ocr2 --file document.pdf > extracted_text.txt
```

### Get structured output for scripting

```bash
result=$(./target/release/paperless-ngx-ocr2 --file document.pdf --json)
echo "$result" | jq -r '.data.extracted_text'
```

## Troubleshooting

### Common Issues

1. **"Configuration file not found"**
   - Create the config file at `~/.config/paperless-ngx-ocr2/config.toml`
   - Or use command-line arguments to override

2. **"API key not provided"**
   - Set the API key in the config file
   - Or use the `--api-key` argument
   - Or set the `PAPERLESS_OCR_API_KEY` environment variable

3. **"File too large"**
   - The file exceeds the maximum size limit (default 100MB)
   - Increase the limit in config or use a smaller file

4. **"Network timeout"**
   - The API request timed out
   - Check your internet connection
   - Increase the timeout in configuration

5. **"File upload error"**
   - Failed to upload file to Mistral AI Files API
   - Check API key and network connectivity
   - Verify file format and size limits

### Debug Mode

Enable debug logging to see detailed information:

```bash
RUST_LOG=debug ./target/release/paperless-ngx-ocr2 --file document.pdf --verbose
```

This will show:

- Configuration loading
- File validation steps
- API request details
- Response processing
