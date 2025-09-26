# Examples

This directory contains example files and configurations for the paperless-ngx-ocr2 OCR CLI tool.

## Directory Structure

```
examples/
├── README.md                 # This file
├── configs/                  # Configuration examples
│   ├── basic.toml           # Basic configuration
│   ├── production.toml      # Production configuration
│   └── development.toml     # Development configuration
├── sample-files/            # Sample files for testing
│   ├── sample.pdf          # Sample PDF document
│   ├── sample.png          # Sample PNG image
│   └── sample.jpg          # Sample JPEG image
├── scripts/                 # Helper scripts
│   ├── batch-process.sh    # Batch processing script
│   ├── docker-run.sh       # Docker execution script
│   └── setup-env.sh        # Environment setup script
└── docker/                  # Docker examples
    ├── Dockerfile.example  # Custom Dockerfile example
    └── docker-compose.yml  # Docker Compose example
```

## Quick Start Examples

### Basic Usage

```bash
# Process a single file
paperless-ngx-ocr2 --file sample-files/sample.pdf --api-key YOUR_API_KEY

# Get JSON output
paperless-ngx-ocr2 --file sample-files/sample.pdf --api-key YOUR_API_KEY --json

# Use custom configuration
paperless-ngx-ocr2 --file sample-files/sample.pdf --config configs/basic.toml
```

### Environment Setup

```bash
# Set up environment variables
source scripts/setup-env.sh

# Process files
paperless-ngx-ocr2 --file sample-files/sample.pdf
```

### Docker Usage

```bash
# Run with Docker
./scripts/docker-run.sh sample-files/sample.pdf

# Use Docker Compose
docker-compose -f docker/docker-compose.yml up
```

### Shell Completion

Generate shell completion scripts directly from the CLI tool:

```bash
# Generate bash completion
paperless-ngx-ocr2 --generate-completions bash > paperless-ngx-ocr2.bash

# Generate zsh completion
paperless-ngx-ocr2 --generate-completions zsh > paperless-ngx-ocr2.zsh

# Generate fish completion
paperless-ngx-ocr2 --generate-completions fish > paperless-ngx-ocr2.fish

# Generate PowerShell completion
paperless-ngx-ocr2 --generate-completions powershell > paperless-ngx-ocr2.ps1
```

Install completions:

```bash
# Bash
source paperless-ngx-ocr2.bash

# Zsh
source paperless-ngx-ocr2.zsh

# Fish
source paperless-ngx-ocr2.fish
```

## Configuration Examples

### Basic Configuration (`configs/basic.toml`)

```toml
api_key = "your-api-key-here"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 30
max_file_size_mb = 100
log_level = "info"
```

### Production Configuration (`configs/production.toml`)

```toml
api_key = "your-production-api-key"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 60
max_file_size_mb = 50
log_level = "warn"
```

### Development Configuration (`configs/development.toml`)

```toml
api_key = "your-dev-api-key"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 30
max_file_size_mb = 10
log_level = "debug"
```

## Sample Files

The `sample-files/` directory contains example files for testing:

- `sample.pdf`: A simple PDF document with text
- `sample.png`: A PNG image with text
- `sample.jpg`: A JPEG image with text

## Helper Scripts

### Batch Processing (`scripts/batch-process.sh`)

Process multiple files in a directory:

```bash
./scripts/batch-process.sh /path/to/documents/ --api-key YOUR_API_KEY
```

### Docker Execution (`scripts/docker-run.sh`)

Run the tool in Docker with proper volume mounting:

```bash
./scripts/docker-run.sh sample-files/sample.pdf
```

### Environment Setup (`scripts/setup-env.sh`)

Set up environment variables for development:

```bash
source scripts/setup-env.sh
```

## Docker Examples

### Custom Dockerfile (`docker/Dockerfile.example`)

Example of a custom Dockerfile with additional tools:

```dockerfile
FROM ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest

# Add additional tools
RUN apk add --no-cache curl jq

# Set custom entrypoint
ENTRYPOINT ["paperless-ngx-ocr2"]
```

### Docker Compose (`docker/docker-compose.yml`)

Example Docker Compose configuration:

```yaml
version: '3.8'
services:
  ocr:
    image: ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest
    environment:
      - PAPERLESS_OCR_API_KEY=${MISTRAL_API_KEY}
    volumes:
      - ./sample-files:/workspace
    command: --file /workspace/sample.pdf --verbose
```

## Testing

Use the sample files to test the tool:

```bash
# Test PDF processing
paperless-ngx-ocr2 --file sample-files/sample.pdf --api-key YOUR_API_KEY

# Test image processing
paperless-ngx-ocr2 --file sample-files/sample.png --api-key YOUR_API_KEY

# Test JSON output
paperless-ngx-ocr2 --file sample-files/sample.jpg --api-key YOUR_API_KEY --json
```

## Troubleshooting

### Common Issues

1. **API Key Not Set**: Make sure to set your Mistral AI API key
2. **File Not Found**: Check that the file path is correct
3. **Permission Denied**: Ensure you have read permissions for the file
4. **Network Error**: Check your internet connection and API endpoint

### Debug Mode

Use verbose logging to debug issues:

```bash
paperless-ngx-ocr2 --file sample-files/sample.pdf --api-key YOUR_API_KEY --verbose
```

## Contributing

To add new examples:

1. Create your example file in the appropriate directory
2. Update this README.md with documentation
3. Test your example thoroughly
4. Submit a pull request

## License

These examples are provided under the same MIT License as the main project.
