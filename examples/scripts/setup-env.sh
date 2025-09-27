#!/bin/bash

# Environment setup script for paperless-ngx-ocr2
# This script sets up environment variables for development and testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to display usage
usage() {
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -k, --api-key KEY     Set Mistral AI API key"
    echo "  -u, --url URL         Set API base URL"
    echo "  -t, --timeout SEC     Set timeout in seconds"
    echo "  -s, --size MB         Set max file size in MB"
    echo "  -l, --log-level LEVEL Set log level"
    echo "  -f, --file FILE       Load from .env file"
    echo "  -e, --export          Export variables (for sourcing)"
    echo "  -h, --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --api-key sk-... --export"
    echo "  $0 --file .env --export"
    echo "  source $0 --api-key sk-..."
}

# Default values
API_KEY=""
API_URL=""
TIMEOUT=""
FILE_SIZE=""
LOG_LEVEL=""
ENV_FILE=""
EXPORT_MODE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -k|--api-key)
            API_KEY="$2"
            shift 2
            ;;
        -u|--url)
            API_URL="$2"
            shift 2
            ;;
        -t|--timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        -s|--size)
            FILE_SIZE="$2"
            shift 2
            ;;
        -l|--log-level)
            LOG_LEVEL="$2"
            shift 2
            ;;
        -f|--file)
            ENV_FILE="$2"
            shift 2
            ;;
        -e|--export)
            EXPORT_MODE=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
        *)
            echo "Unexpected argument: $1"
            usage
            exit 1
            ;;
    esac
done

# Function to set environment variable
set_env_var() {
    local var_name="$1"
    local var_value="$2"
    local description="$3"

    if [[ -n "$var_value" ]]; then
        if [[ "$EXPORT_MODE" == true ]]; then
            echo "export $var_name=\"$var_value\""
        else
            export "$var_name"="$var_value"
            print_success "Set $var_name=$var_value"
        fi
    elif [[ -n "$description" ]]; then
        print_warning "$description"
    fi
}

# Function to load .env file
load_env_file() {
    local file="$1"

    if [[ ! -f "$file" ]]; then
        print_error "Environment file '$file' does not exist"
        return 1
    fi

    print_status "Loading environment from: $file"

    # Read .env file and export variables
    while IFS= read -r line || [[ -n "$line" ]]; do
        # Skip empty lines and comments
        if [[ -z "$line" || "$line" =~ ^[[:space:]]*# ]]; then
            continue
        fi

        # Check if line contains =
        if [[ "$line" =~ ^[^=]+=.* ]]; then
            if [[ "$EXPORT_MODE" == true ]]; then
                echo "export $line"
            else
                export "$line"
            fi
        else
            print_warning "Invalid line in .env file: $line"
        fi
    done < "$file"

    print_success "Loaded environment from: $file"
}

# Main execution
if [[ "$EXPORT_MODE" == true ]]; then
    print_status "Generating export commands..."
    echo "# Environment variables for paperless-ngx-ocr2"
    echo "# Generated on: $(date)"
    echo ""
fi

# Load from .env file if specified
if [[ -n "$ENV_FILE" ]]; then
    load_env_file "$ENV_FILE"
fi

# Set individual variables
set_env_var "PAPERLESS_OCR_API_KEY" "$API_KEY" "API key not set - you may need to set PAPERLESS_OCR_API_KEY"
set_env_var "PAPERLESS_OCR_API_BASE_URL" "$API_URL" "API URL not set - using default: https://api.mistral.ai"
set_env_var "PAPERLESS_OCR_TIMEOUT" "$TIMEOUT" "Timeout not set - using default: 30 seconds"
set_env_var "PAPERLESS_OCR_MAX_FILE_SIZE_MB" "$FILE_SIZE" "File size limit not set - using default: 100 MB"
set_env_var "PAPERLESS_OCR_LOG_LEVEL" "$LOG_LEVEL" "Log level not set - using default: info"

if [[ "$EXPORT_MODE" == false ]]; then
    print_success "Environment setup complete!"
    print_status "You can now run: paperless-ngx-ocr2 --file <file>"

    # Check if API key is set
    if [[ -z "$PAPERLESS_OCR_API_KEY" ]]; then
        print_warning "Remember to set your Mistral AI API key:"
        print_warning "  export PAPERLESS_OCR_API_KEY=\"your-api-key-here\""
    fi
fi
