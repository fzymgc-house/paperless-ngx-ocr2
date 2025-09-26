#!/bin/bash

# Generate man page script for paperless-ngx-ocr2
# This script generates a man page from the clap help text

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if we're in the project root
if [[ ! -f "Cargo.toml" ]]; then
    echo "Error: This script must be run from the project root directory"
    exit 1
fi

print_status "Generating man page for paperless-ngx-ocr2..."

# Generate the man page using build script
print_status "Generating man page..."
GENERATE_MAN=1 cargo build --release

# Check if man page was generated
if [[ -f "paperless-ngx-ocr2.1" ]]; then
    print_success "Man page generated: paperless-ngx-ocr2.1"
    
    # Show man page info
    print_status "Man page info:"
    echo "  File: paperless-ngx-ocr2.1"
    echo "  Size: $(wc -c < paperless-ngx-ocr2.1) bytes"
    echo "  Lines: $(wc -l < paperless-ngx-ocr2.1)"
    
    # Install man page (optional)
    if [[ "$1" == "--install" ]]; then
        print_status "Installing man page..."
        sudo cp paperless-ngx-ocr2.1 /usr/local/share/man/man1/
        sudo mandb
        print_success "Man page installed to /usr/local/share/man/man1/"
        print_status "You can now view it with: man paperless-ngx-ocr2"
    else
        print_status "To install the man page, run: $0 --install"
    fi
else
    echo "Error: Man page generation failed"
    exit 1
fi
