# Multi-stage Dockerfile for paperless-ngx-ocr2
# Supports multi-architecture builds (AMD64, ARM64)

# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

# Set up working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application with static linking
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Set target based on architecture
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
        "linux/amd64") \
            rustup target add x86_64-unknown-linux-musl && \
            cargo build --release --target x86_64-unknown-linux-musl \
            ;; \
        "linux/arm64") \
            rustup target add aarch64-unknown-linux-musl && \
            cargo build --release --target aarch64-unknown-linux-musl \
            ;; \
        *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac

# Runtime stage
FROM alpine:3.19 AS runtime

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    tzdata

# Create non-root user
RUN addgroup -g 1000 ocr && \
    adduser -D -s /bin/sh -u 1000 -G ocr ocr

# Copy the binary from builder stage (architecture-aware)
ARG TARGETPLATFORM
COPY --from=builder /app/target/*/release/paperless-ngx-ocr2 /usr/local/bin/

# Set up working directory
WORKDIR /workspace

# Change ownership to non-root user
RUN chown -R ocr:ocr /workspace

# Switch to non-root user
USER ocr

# Set environment variables for 12-factor app compliance
ENV PAPERLESS_OCR_API_BASE_URL=https://api.mistral.ai
ENV RUST_LOG=paperless_ngx_ocr2=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD paperless-ngx-ocr2 --help > /dev/null || exit 1

# Default entrypoint
ENTRYPOINT ["paperless-ngx-ocr2"]

# Default command (show help)
CMD ["--help"]

# Labels for metadata
LABEL org.opencontainers.image.title="paperless-ngx-ocr2"
LABEL org.opencontainers.image.description="OCR CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction"
LABEL org.opencontainers.image.vendor="fzymgc-house"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL org.opencontainers.image.source="https://github.com/fzymgc-house/paperless-ngx-ocr2"
