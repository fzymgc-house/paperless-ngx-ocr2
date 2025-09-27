# Docker Usage Examples

This directory contains Docker-related examples and configurations for the paperless-ngx-ocr2 CLI tool.

## Files

- `docker-compose.yml` - Main Docker Compose configuration for local testing
- `Dockerfile.example` - Example Dockerfile for custom builds
- `README.md` - This file

## Quick Start

### Using Docker Compose (Recommended)

1. **Set up environment variables:**

   ```bash
   # Copy the example environment file
   cp .env.example .env

   # Edit .env and add your Mistral AI API key
   nano .env
   ```

2. **Build and run the container:**

   ```bash
   # Build the image
   docker-compose build

   # Run with help
   docker-compose run --rm paperless-ngx-ocr2 --help

   # Process a file
   docker-compose run --rm paperless-ngx-ocr2 --file /path/to/your/file.pdf
   ```

3. **Interactive mode:**

   ```bash
   # Start an interactive container
   docker-compose run --rm paperless-ngx-ocr2 bash
   ```

### Using Docker directly

1. **Build the image:**

   ```bash
   docker build -t paperless-ngx-ocr2:latest .
   ```

2. **Run the container:**

   ```bash
   # With environment variables
   docker run --rm \
     -e PAPERLESS_OCR_API_KEY=your_api_key_here \
     -v $(pwd):/workspace:ro \
     paperless-ngx-ocr2:latest \
     --file /workspace/your_file.pdf
   ```

## Development Mode

For development with hot reload:

```bash
# Use the development profile
docker-compose --profile dev up paperless-ngx-ocr2-dev

# Or run directly
docker-compose run --rm paperless-ngx-ocr2-dev
```

## Multi-Architecture Builds

The Dockerfile supports multi-architecture builds for both AMD64 and ARM64:

```bash
# Build for multiple architectures
docker buildx build --platform linux/amd64,linux/arm64 -t paperless-ngx-ocr2:latest .
```

## Configuration

### Environment Variables

- `PAPERLESS_OCR_API_KEY` - Your Mistral AI API key (required)
- `PAPERLESS_OCR_API_BASE_URL` - Mistral AI API base URL (default: <https://api.mistral.ai>)
- `RUST_LOG` - Logging level (default: info)

### Volume Mounts

- `/workspace` - Working directory for file processing
- `/app/config.toml` - Configuration file (optional)

## Security Features

- Runs as non-root user (`appuser`)
- Minimal Alpine Linux base image
- Static binary compilation
- Resource limits applied
- No unnecessary packages installed

## Troubleshooting

### Common Issues

1. **Permission denied errors:**

   ```bash
   # Ensure the container has read access to your files
   docker-compose run --rm paperless-ngx-ocr2 --file /workspace/your_file.pdf
   ```

2. **API key not found:**

   ```bash
   # Check environment variables
   docker-compose run --rm paperless-ngx-ocr2 env | grep PAPERLESS
   ```

3. **File not found:**

   ```bash
   # Ensure files are mounted in /workspace
   docker-compose run --rm paperless-ngx-ocr2 ls -la /workspace
   ```

### Debug Mode

Enable debug logging:

```bash
docker-compose run --rm -e RUST_LOG=debug paperless-ngx-ocr2 --verbose --file /workspace/your_file.pdf
```

## Testing

Run the container integration tests:

```bash
# Run all tests
cargo test --test test_docker

# Run specific test
cargo test test_docker_build
```

## Production Deployment

For production use, consider:

1. **Using a specific tag instead of `latest`**
2. **Setting resource limits**
3. **Using secrets management for API keys**
4. **Implementing health checks**
5. **Using a reverse proxy if needed**

Example production docker-compose.yml:

```yaml
version: '3.8'
services:
  paperless-ngx-ocr2:
    image: paperless-ngx-ocr2:v1.0.0
    environment:
      - PAPERLESS_OCR_API_KEY_FILE=/run/secrets/api_key
    secrets:
      - api_key
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '1.0'

secrets:
  api_key:
    external: true
```
