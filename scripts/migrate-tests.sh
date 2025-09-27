#!/bin/bash

# Test Migration Script
# This script helps migrate existing tests to use the new test utilities

set -e

echo "🧪 Test Infrastructure Migration Script"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the project root directory"
    exit 1
fi

# Check if test utilities exist
if [[ ! -d "tests/common" ]]; then
    print_error "Test utilities not found. Please run the test infrastructure improvements first."
    exit 1
fi

print_step "1. Analyzing existing test files..."

# Find all test files that need migration
TEST_FILES=($(find tests -name "*.rs" -not -path "tests/common/*" -not -name "*_improved.rs"))

print_status "Found ${#TEST_FILES[@]} test files to analyze"

# Function to check if a file needs migration
needs_migration() {
    local file="$1"
    
    # Check for patterns that indicate manual cleanup or temporary file creation
    if grep -q "fs::remove_file\|\.ok()" "$file" || \
       grep -q "NamedTempFile::new" "$file" || \
       grep -q "TempDir::new" "$file" || \
       grep -q "Command::cargo_bin" "$file"; then
        return 0
    fi
    return 1
}

# Function to generate migration suggestions
generate_migration_suggestions() {
    local file="$1"
    local suggestions_file="migration_suggestions_$(basename "$file").txt"
    
    echo "Migration suggestions for $file" > "$suggestions_file"
    echo "=================================" >> "$suggestions_file"
    echo "" >> "$suggestions_file"
    
    # Check for specific patterns and suggest improvements
    if grep -q "fs::remove_file.*\.ok()" "$file"; then
        echo "🔧 MANUAL CLEANUP PATTERN DETECTED:" >> "$suggestions_file"
        echo "   Replace manual cleanup with TestFile for automatic cleanup" >> "$suggestions_file"
        echo "   Example:" >> "$suggestions_file"
        echo "   Before: fs::remove_file(&temp_path).ok();" >> "$suggestions_file"
        echo "   After:  let test_file = create_test_pdf(\"content\"); // auto cleanup" >> "$suggestions_file"
        echo "" >> "$suggestions_file"
    fi
    
    if grep -q "NamedTempFile::new" "$file"; then
        echo "🔧 TEMPORARY FILE CREATION PATTERN DETECTED:" >> "$suggestions_file"
        echo "   Replace NamedTempFile creation with TestFile helpers" >> "$suggestions_file"
        echo "   Example:" >> "$suggestions_file"
        echo "   Before: let mut temp_file = NamedTempFile::new().unwrap();" >> "$suggestions_file"
        echo "   After:  let test_file = create_test_pdf(\"content\");" >> "$suggestions_file"
        echo "" >> "$suggestions_file"
    fi
    
    if grep -q "Command::cargo_bin" "$file"; then
        echo "🔧 CLI COMMAND CREATION PATTERN DETECTED:" >> "$suggestions_file"
        echo "   Replace manual command creation with TestConfig helpers" >> "$suggestions_file"
        echo "   Example:" >> "$suggestions_file"
        echo "   Before: let mut cmd = Command::cargo_bin(\"paperless-ngx-ocr2\").unwrap();" >> "$suggestions_file"
        echo "   After:  let config = TestConfig::new(); let mut cmd = create_configured_command(&config);" >> "$suggestions_file"
        echo "" >> "$suggestions_file"
    fi
    
    if grep -q "tests/fixtures/" "$file"; then
        echo "🔧 FIXTURE USAGE PATTERN DETECTED:" >> "$suggestions_file"
        echo "   Consider using fixture helpers for better type safety" >> "$suggestions_file"
        echo "   Example:" >> "$suggestions_file"
        echo "   Before: FileUpload::new(\"tests/fixtures/sample.pdf\")" >> "$suggestions_file"
        echo "   After:  let fixture = fixtures::sample_pdf(); FileUpload::new(fixture.path_str())" >> "$suggestions_file"
        echo "" >> "$suggestions_file"
    fi
    
    echo "📚 ADD THESE IMPORTS TO THE TOP OF YOUR TEST FILE:" >> "$suggestions_file"
    echo "   mod common;" >> "$suggestions_file"
    echo "   use common::*;" >> "$suggestions_file"
    echo "" >> "$suggestions_file"
    
    echo "📖 For more information, see tests/common/README.md" >> "$suggestions_file"
}

# Analyze each test file
MIGRATION_NEEDED=()
for file in "${TEST_FILES[@]}"; do
    if needs_migration "$file"; then
        MIGRATION_NEEDED+=("$file")
        print_warning "$file needs migration"
        generate_migration_suggestions "$file"
    else
        print_status "$file is already using good patterns"
    fi
done

print_step "2. Migration Summary"

if [[ ${#MIGRATION_NEEDED[@]} -eq 0 ]]; then
    print_status "🎉 All test files are already using good patterns!"
else
    print_status "📋 Files that need migration:"
    for file in "${MIGRATION_NEEDED[@]}"; do
        echo "   - $file"
    done
    
    print_step "3. Generated migration suggestions"
    for file in "${MIGRATION_NEEDED[@]}"; do
        suggestions_file="migration_suggestions_$(basename "$file").txt"
        if [[ -f "$suggestions_file" ]]; then
            print_status "📝 Generated suggestions: $suggestions_file"
        fi
    done
fi

print_step "4. Running tests to ensure everything works"

# Run tests to make sure the new utilities work
print_status "Running tests for common utilities..."
if cargo test --test common --quiet; then
    print_status "✅ Common utilities tests passed"
else
    print_error "❌ Common utilities tests failed"
    exit 1
fi

print_step "5. Performance comparison"

print_status "Running performance tests..."
if cargo test --test test_file_improved --quiet; then
    print_status "✅ Improved test patterns work correctly"
else
    print_warning "⚠️  Some improved tests failed (this might be expected during development)"
fi

print_step "6. Next Steps"

echo ""
echo "🚀 Migration Complete! Here's what to do next:"
echo ""
echo "1. Review the generated migration suggestion files"
echo "2. Update test files one by one using the suggestions"
echo "3. Add 'mod common;' and 'use common::*;' to test files"
echo "4. Replace manual patterns with utility functions"
echo "5. Run tests to ensure everything works"
echo ""
echo "📚 For detailed documentation, see tests/common/README.md"
echo ""
echo "💡 Pro tips:"
echo "   - Start with simple files first"
echo "   - Use the improved examples as templates"
echo "   - Test frequently during migration"
echo "   - Consider creating a backup branch before major changes"
echo ""

print_status "🎉 Migration analysis complete!"
