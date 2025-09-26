#!/bin/bash

# Security scanning script for paperless-ngx-ocr2
# This script runs various security checks on the codebase

set -e

echo "🔒 Running security checks for paperless-ngx-ocr2..."

# Check if cargo-audit is available
if command -v cargo-audit &> /dev/null; then
    echo "📋 Running cargo audit for vulnerability scanning..."
    cargo audit
else
    echo "⚠️  cargo-audit not found. Install with: cargo install cargo-audit"
    echo "   Skipping vulnerability scan..."
fi

# Run clippy with security-focused lints
echo "🔍 Running clippy security checks..."
cargo clippy --all-targets --all-features -- -D warnings

# Check for hardcoded secrets (basic check, excluding test code)
echo "🔐 Checking for potential hardcoded secrets..."
# Check for real API keys (not test keys like sk-test123)
if grep -r "sk-[a-zA-Z0-9]" src/ --exclude-dir=target | grep -v "sk-test" 2>/dev/null; then
    echo "❌ Potential real API keys found in source code!"
    exit 1
else
    echo "✅ No real API keys found in source code (only test keys detected)"
fi

# Check for unsafe code usage
echo "🛡️  Checking for unsafe code usage..."
if grep -r "unsafe" src/ --exclude-dir=target 2>/dev/null; then
    echo "⚠️  Unsafe code usage detected. Review for security implications."
    grep -r "unsafe" src/ --exclude-dir=target
else
    echo "✅ No unsafe code usage found"
fi

# Check for proper error handling patterns
echo "🔧 Checking error handling patterns..."
if grep -r "unwrap()" src/ --exclude-dir=target 2>/dev/null; then
    echo "⚠️  unwrap() calls found. Consider using proper error handling."
    grep -r "unwrap()" src/ --exclude-dir=target
else
    echo "✅ No unwrap() calls found"
fi

# Check for proper API key redaction
echo "🔑 Checking API key redaction patterns..."
if grep -r "redacted_key\|redact" src/ --exclude-dir=target 2>/dev/null; then
    echo "✅ API key redaction patterns found"
else
    echo "⚠️  No API key redaction patterns found"
fi

# Check for HTTPS usage
echo "🌐 Checking HTTPS usage..."
if grep -r "https://" src/ --exclude-dir=target 2>/dev/null; then
    echo "✅ HTTPS usage confirmed"
else
    echo "⚠️  No HTTPS usage found"
fi

echo "✅ Security checks completed!"
