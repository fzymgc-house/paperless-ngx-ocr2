#!/bin/bash

# Docker execution script for paperless-ngx-ocr2
# Usage: ./docker-run.sh <file> [options]

set -e

# Default values
API_KEY=""
VERBOSE=false
JSON_OUTPUT=false
CONFIG_FILE=""
DOCKER_IMAGE="ghcr.io/fzymgc-house/paperless-ngx-ocr2:latest"

# Function to display usage
usage() {
    echo "Usage: $0 <file> [options]"
    echo ""
    echo "Options:"
    echo "  -k, --api-key KEY     Mistral AI API key"
    echo "  -c, --config FILE     Configuration file"
    echo "  -i, --image IMAGE     Docker image to use"
    echo "  -j, --json            Output in JSON format"
    echo "  -v, --verbose         Enable verbose logging"
    echo "  -h, --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 document.pdf --api-key sk-..."
    echo "  $0 document.pdf --config config.toml --json"
    echo "  $0 document.pdf --api-key sk-... --verbose"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -k|--api-key)
            API_KEY="$2"
            shift 2
            ;;
        -c|--config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        -i|--image)
            DOCKER_IMAGE="$2"
            shift 2
            ;;
        -j|--json)
            JSON_OUTPUT=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
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
            if [[ -z "$INPUT_FILE" ]]; then
                INPUT_FILE="$1"
            else
                echo "Multiple input files specified"
                usage
                exit 1
            fi
            shift
            ;;
    esac
done

# Check if input file is provided
if [[ -z "$INPUT_FILE" ]]; then
    echo "Error: Input file is required"
    usage
    exit 1
fi

# Check if input file exists
if [[ ! -f "$INPUT_FILE" ]]; then
    echo "Error: Input file '$INPUT_FILE' does not exist"
    exit 1
fi

# Get absolute path of input file
INPUT_FILE_ABS=$(realpath "$INPUT_FILE")
INPUT_DIR=$(dirname "$INPUT_FILE_ABS")
INPUT_FILENAME=$(basename "$INPUT_FILE_ABS")

# Build Docker command
DOCKER_CMD="docker run --rm"
DOCKER_CMD="$DOCKER_CMD -v \"$INPUT_DIR:/workspace\""

# Add environment variables
if [[ -n "$API_KEY" ]]; then
    DOCKER_CMD="$DOCKER_CMD -e PAPERLESS_OCR_API_KEY=\"$API_KEY\""
fi

# Add Docker image
DOCKER_CMD="$DOCKER_CMD $DOCKER_IMAGE"

# Build tool arguments
TOOL_ARGS="--file /workspace/$INPUT_FILENAME"

if [[ -n "$CONFIG_FILE" ]]; then
    # Check if config file exists
    if [[ ! -f "$CONFIG_FILE" ]]; then
        echo "Error: Config file '$CONFIG_FILE' does not exist"
        exit 1
    fi
    
    # Get absolute path of config file
    CONFIG_FILE_ABS=$(realpath "$CONFIG_FILE")
    CONFIG_DIR=$(dirname "$CONFIG_FILE_ABS")
    CONFIG_FILENAME=$(basename "$CONFIG_FILE_ABS")
    
    # Mount config file
    DOCKER_CMD="$DOCKER_CMD -v \"$CONFIG_DIR:/config\""
    TOOL_ARGS="$TOOL_ARGS --config /config/$CONFIG_FILENAME"
fi

if [[ "$JSON_OUTPUT" == true ]]; then
    TOOL_ARGS="$TOOL_ARGS --json"
fi

if [[ "$VERBOSE" == true ]]; then
    TOOL_ARGS="$TOOL_ARGS --verbose"
fi

# Execute Docker command
echo "Running: $DOCKER_CMD $TOOL_ARGS"
echo ""

eval "$DOCKER_CMD $TOOL_ARGS"
