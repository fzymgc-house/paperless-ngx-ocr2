#!/bin/bash
# Setup script for pre-commit hooks
# This script installs and configures pre-commit hooks for the project

set -e

echo "🔧 Setting up pre-commit hooks for paperless-ngx-ocr2..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "❌ pre-commit is not installed. Installing..."

    # Try different installation methods
    if command -v pip3 &> /dev/null; then
        echo "📦 Installing pre-commit via pip3..."
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        echo "📦 Installing pre-commit via pip..."
        pip install pre-commit
    elif command -v brew &> /dev/null; then
        echo "📦 Installing pre-commit via Homebrew..."
        brew install pre-commit
    else
        echo "❌ Could not install pre-commit. Please install it manually:"
        echo "   https://pre-commit.com/#installation"
        exit 1
    fi
else
    echo "✅ pre-commit is already installed"
fi

# Check if .pre-commit-config.yaml exists
if [ ! -f ".pre-commit-config.yaml" ]; then
    echo "❌ .pre-commit-config.yaml not found in current directory"
    exit 1
fi

# Install the pre-commit hooks
echo "🔗 Installing pre-commit hooks..."
pre-commit install

# Install commit-msg hook for additional validation
echo "🔗 Installing commit-msg hook..."
pre-commit install --hook-type commit-msg

# Update pre-commit hooks to latest versions
echo "🔄 Updating pre-commit hooks to latest versions..."
pre-commit autoupdate

# Run pre-commit on all files to test the setup
echo "🧪 Testing pre-commit hooks on all files..."
if pre-commit run --all-files; then
    echo "✅ Pre-commit setup completed successfully!"
    echo ""
    echo "🎉 Your repository is now protected by pre-commit hooks!"
    echo ""
    echo "The following checks will run on every commit:"
    echo "  • Rust code formatting (rustfmt)"
    echo "  • Rust linting (clippy)"
    echo "  • Rust tests (cargo test)"
    echo "  • Security audit (cargo audit)"
    echo "  • File quality checks (trailing whitespace, etc.)"
    echo "  • YAML/JSON/TOML syntax validation"
    echo "  • Large file detection"
    echo ""
    echo "To run hooks manually on all files: pre-commit run --all-files"
    echo "To run hooks manually on staged files: pre-commit run"
    echo "To skip hooks for a commit: git commit --no-verify"
else
    echo "⚠️  Pre-commit hooks found some issues. Please fix them and try again."
    echo "   You can run 'pre-commit run --all-files' to see all issues."
    exit 1
fi
