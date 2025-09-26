#!/bin/bash

# Batch processing script for paperless-ngx-ocr2
# Usage: ./batch-process.sh <directory> [options]

set -e

# Default values
API_KEY=""
VERBOSE=false
JSON_OUTPUT=false
CONFIG_FILE=""
OUTPUT_DIR=""

# Function to display usage
usage() {
    echo "Usage: $0 <directory> [options]"
    echo ""
    echo "Options:"
    echo "  -k, --api-key KEY     Mistral AI API key"
    echo "  -c, --config FILE     Configuration file"
    echo "  -o, --output DIR      Output directory for results"
    echo "  -j, --json            Output in JSON format"
    echo "  -v, --verbose         Enable verbose logging"
    echo "  -h, --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 /path/to/documents --api-key sk-..."
    echo "  $0 /path/to/documents --config config.toml --json"
    echo "  $0 /path/to/documents --api-key sk-... --output /path/to/results"
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
        -o|--output)
            OUTPUT_DIR="$2"
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
            if [[ -z "$INPUT_DIR" ]]; then
                INPUT_DIR="$1"
            else
                echo "Multiple input directories specified"
                usage
                exit 1
            fi
            shift
            ;;
    esac
done

# Check if input directory is provided
if [[ -z "$INPUT_DIR" ]]; then
    echo "Error: Input directory is required"
    usage
    exit 1
fi

# Check if input directory exists
if [[ ! -d "$INPUT_DIR" ]]; then
    echo "Error: Input directory '$INPUT_DIR' does not exist"
    exit 1
fi

# Create output directory if specified
if [[ -n "$OUTPUT_DIR" ]]; then
    mkdir -p "$OUTPUT_DIR"
fi

# Build command arguments
CMD_ARGS=()
if [[ -n "$API_KEY" ]]; then
    CMD_ARGS+=("--api-key" "$API_KEY")
fi
if [[ -n "$CONFIG_FILE" ]]; then
    CMD_ARGS+=("--config" "$CONFIG_FILE")
fi
if [[ "$JSON_OUTPUT" == true ]]; then
    CMD_ARGS+=("--json")
fi
if [[ "$VERBOSE" == true ]]; then
    CMD_ARGS+=("--verbose")
fi

# Supported file extensions
SUPPORTED_EXTENSIONS=("pdf" "png" "jpg" "jpeg")

# Process files
echo "Processing files in: $INPUT_DIR"
echo "Supported formats: ${SUPPORTED_EXTENSIONS[*]}"
echo ""

PROCESSED_COUNT=0
ERROR_COUNT=0

for file in "$INPUT_DIR"/*; do
    if [[ -f "$file" ]]; then
        # Get file extension
        extension="${file##*.}"
        extension="${extension,,}"  # Convert to lowercase
        
        # Check if file extension is supported
        if [[ " ${SUPPORTED_EXTENSIONS[*]} " =~ " ${extension} " ]]; then
            echo "Processing: $(basename "$file")"
            
            # Build output file path if output directory is specified
            if [[ -n "$OUTPUT_DIR" ]]; then
                base_name=$(basename "$file" ".$extension")
                if [[ "$JSON_OUTPUT" == true ]]; then
                    output_file="$OUTPUT_DIR/${base_name}.json"
                else
                    output_file="$OUTPUT_DIR/${base_name}.txt"
                fi
            else
                output_file="/dev/stdout"
            fi
            
            # Run the OCR tool
            if paperless-ngx-ocr2 --file "$file" "${CMD_ARGS[@]}" > "$output_file" 2>/dev/null; then
                echo "  ✓ Success"
                ((PROCESSED_COUNT++))
            else
                echo "  ✗ Failed"
                ((ERROR_COUNT++))
            fi
        else
            echo "Skipping: $(basename "$file") (unsupported format: $extension)"
        fi
    fi
done

echo ""
echo "Batch processing complete:"
echo "  Processed: $PROCESSED_COUNT files"
echo "  Errors: $ERROR_COUNT files"

if [[ $ERROR_COUNT -gt 0 ]]; then
    exit 1
fi
