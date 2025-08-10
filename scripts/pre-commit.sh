#!/bin/bash
# Pre-commit hook for Rust projects
# This script runs linting checks before each commit

set -e

echo "Running pre-commit checks..."

# Check formatting
echo "🔍 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting issues found. Run 'cargo fmt' to fix them."
    exit 1
fi

# Run clippy
echo "🔍 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy found issues. Please fix them before committing."
    exit 1
fi

# Run tests
echo "🔍 Running tests..."
if ! cargo test; then
    echo "❌ Tests failed. Please fix them before committing."
    exit 1
fi

# Check for common issues
echo "🔍 Checking for common issues..."
if ! cargo check --all-targets --all-features; then
    echo "❌ Compilation issues found. Please fix them before committing."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
