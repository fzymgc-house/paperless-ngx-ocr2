# paperless-ngx-ocr2

A command-line tool for extracting text from PDF and image files using
Mistral AI's OCR capabilities. Supports TOML configuration, 12-factor app
principles, and provides both human-readable and JSON output formats.

## Features

- **Multi-format Support**: Process PDF, PNG, JPEG, and JPG files
- **Mistral AI Integration**: Leverages Mistral AI's advanced OCR
  capabilities
- **Flexible Configuration**: TOML config files, environment variables,
  and CLI arguments
- **12-Factor App**: Follows 12-factor app principles for
  configuration management
- **Multiple Output Formats**: Human-readable text or structured
  JSON output
- **Robust Error Handling**: Comprehensive error handling with
  constitutional exit codes
- **Retry Logic**: Automatic retry with exponential backoff for
  rate limits
- **Security Features**: Password-protected PDF detection and API
  key redaction
- **Cross-Platform**: Works on macOS and Linux
- **Containerized**: Docker support with multi-architecture builds

## Installation

### From Source

1. **Prerequisites**:
   - Rust 1.80 or later
   - Git

2. **Clone and Build**:

   ```bash
   git clone https://github.com/fzymgc-house/paperless-ngx-ocr2.git
   cd paperless-ngx-ocr2
   cargo build --release
   ```

3. **Setup Pre-commit Hooks** (recommended):

   ```bash
   ./scripts/setup-pre-commit.sh
   ```

4. **Install** (optional):

   ```bash
   cargo install --path .
   ```

### Using Docker

1. **Pull the image**:

   ```bash
   docker pull ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest
   ```

2. **Run with Docker**:

   ```bash
   docker run --rm -v $(pwd):/workspace ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest --file /workspace/document.pdf --api-key YOUR_API_KEY
   ```

### Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/fzymgc-house/paperless-ngx-ocr2/releases).

## Quick Start

1. **Get a Mistral AI API key** from [Mistral AI](https://console.mistral.ai/)

2. **Process a file**:

   ```bash
   paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY
   ```

3. **Get JSON output**:

   ```bash
   paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY --json
   ```

## Configuration

### Environment Variables

Set your API key and other options via environment variables:

```bash
export PAPERLESS_OCR_API_KEY="your-api-key-here"
export PAPERLESS_OCR_API_BASE_URL="https://api.mistral.ai"
export PAPERLESS_OCR_TIMEOUT="30"
export PAPERLESS_OCR_MAX_FILE_SIZE_MB="100"
export PAPERLESS_OCR_LOG_LEVEL="info"
```

### TOML Configuration File

Create a `config.toml` file in your current directory or `~/.config/paperless-ngx-ocr2/`:

```toml
api_key = "your-api-key-here"
api_base_url = "https://api.mistral.ai"
timeout_seconds = 30
max_file_size_mb = 100
log_level = "info"
```

### Configuration Priority

The tool loads configuration in the following order (later values override earlier ones):

1. Default values
2. TOML configuration file
3. `.env` file (if present)
4. Environment variables
5. CLI arguments

## Usage

### Basic Usage

```bash
# Process a PDF file
paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY

# Process an image file
paperless-ngx-ocr2 --file image.png --api-key YOUR_API_KEY

# Get JSON output
paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY --json

# Enable verbose logging
paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY --verbose
```

### Advanced Usage

```bash
# Use custom configuration file
paperless-ngx-ocr2 --file document.pdf --config /path/to/config.toml

# Override API base URL
paperless-ngx-ocr2 --file document.pdf --api-key YOUR_API_KEY --api-base-url https://custom.api.com

# Process with environment variables
PAPERLESS_OCR_API_KEY=your-key paperless-ngx-ocr2 --file document.pdf

# Generate shell completions
paperless-ngx-ocr2 --generate-completions bash > paperless-ngx-ocr2.bash
```

### Command Line Options

```text
USAGE:
    paperless-ngx-ocr2 [OPTIONS] --file <FILE>

OPTIONS:
    -f, --file <FILE>
            Path to the PDF or image file to process

    -a, --api-key <KEY>
            Mistral AI API key (can also be set via environment variable)
            [env: PAPERLESS_OCR_API_KEY=]

        --api-base-url <URL>
            Mistral AI API base URL
            [env: PAPERLESS_OCR_API_BASE_URL=]
            [default: https://api.mistral.ai]

        --config <PATH>
            Path to custom configuration file

        --json
            Output result in JSON format instead of human-readable text

    -v, --verbose
            Enable verbose logging output

    -h, --help
            Print help (see a summary with '-h')

    -V, --version
            Print version
```

## Supported File Formats

- **PDF**: Unprotected PDF files (password-protected PDFs are rejected)
- **Images**: PNG, JPEG, JPG files
- **Size Limit**: Up to 100MB per file

## Error Handling

The tool provides comprehensive error handling with specific exit codes:

- **0**: Success (including warnings for empty text)
- **2**: Validation error (invalid file format, size, etc.)
- **3**: File I/O error (file not found, permission denied, etc.)
- **4**: Configuration error (missing API key, invalid config, etc.)
- **5**: API or network error (authentication failed, network timeout, etc.)

## Shell Completions

The tool includes built-in shell completion generation for bash, zsh, fish, and PowerShell:

```bash
# Generate completion scripts
paperless-ngx-ocr2 --generate-completions bash > paperless-ngx-ocr2.bash
paperless-ngx-ocr2 --generate-completions zsh > paperless-ngx-ocr2.zsh
paperless-ngx-ocr2 --generate-completions fish > paperless-ngx-ocr2.fish
paperless-ngx-ocr2 --generate-completions powershell > paperless-ngx-ocr2.ps1

# Install completions
source paperless-ngx-ocr2.bash  # For bash
source paperless-ngx-ocr2.zsh   # For zsh
source paperless-ngx-ocr2.fish  # For fish
```bash

## Development

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality and consistency. The hooks automatically run:

- **Rust formatting** (`rustfmt`)
- **Rust linting** (`clippy`)
- **Rust tests** (`cargo test`)
- **Security audit** (`cargo audit`)
- **File quality checks** (trailing whitespace, end-of-file, etc.)
- **YAML/JSON/TOML syntax validation**
- **Large file detection**
- **Docker linting** (`hadolint`)
- **Shell script linting** (`shellcheck`)
- **Markdown linting** (`markdownlint`)

#### Setup

```bash
# Install pre-commit hooks
./scripts/setup-pre-commit.sh

# Or manually:
pip install pre-commit  # or brew install pre-commit
pre-commit install
pre-commit autoupdate
```

#### Usage

```bash
# Run hooks on all files
pre-commit run --all-files

# Run hooks on staged files only
pre-commit run

# Skip hooks for a commit (not recommended)
git commit --no-verify
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test --test test_cli_basic

# Run with output
cargo test -- --nocapture
```

## Examples

### Basic Text Extraction

```bash
$ paperless-ngx-ocr2 --file sample.pdf --api-key sk-...
Extracted text from sample.pdf (245760 bytes):

This is the extracted text from the PDF document.
It includes multiple paragraphs and formatting.
```

### JSON Output

```bash
$ paperless-ngx-ocr2 --file sample.pdf --api-key sk-... --json
{
  "success": true,
  "data": {
    "extracted_text": "This is the extracted text from the PDF document.",
    "file_name": "sample.pdf",
    "file_size": 245760,
    "processing_time_ms": 2000,
    "confidence": null
  }
}
```

### Error Handling

```bash
$ paperless-ngx-ocr2 --file password-protected.pdf --api-key sk-...
Error: Validation error: Password-protected PDF detected. Please provide an unprotected PDF file.
```

## Development

### Building from Source

```bash
git clone https://github.com/fzymgc-house/paperless-ngx-ocr2.git
cd paperless-ngx-ocr2
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test test_cli_basic
cargo test --test test_config_loading
cargo test --lib
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run all quality checks
cargo test-all
```

## Docker

### Multi-Architecture Builds

The project supports multi-architecture Docker builds for AMD64 and ARM64:

```bash
# Build for current platform
docker build -t paperless-ngx-ocr2 .

# Build for multiple architectures
docker buildx build --platform linux/amd64,linux/arm64 -t paperless-ngx-ocr2 .
```

### Docker Compose

```yaml
version: '3.8'
services:
  ocr:
    image: ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest
    environment:
      - PAPERLESS_OCR_API_KEY=${MISTRAL_API_KEY}
    volumes:
      - ./documents:/workspace
    command: --file /workspace/document.pdf --verbose
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Mistral AI](https://mistral.ai/) for providing the OCR API
- [Rust](https://www.rust-lang.org/) for the excellent language and ecosystem
- [Clap](https://github.com/clap-rs/clap) for CLI argument parsing
- [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP client functionality

## Support

- **Issues**: [GitHub Issues](https://github.com/fzymgc-house/paperless-ngx-ocr2/issues)
- **Discussions**: [GitHub Discussions](https://github.com/fzymgc-house/paperless-ngx-ocr2/discussions)
- **Documentation**: [Project Wiki](https://github.com/fzymgc-house/paperless-ngx-ocr2/wiki)
