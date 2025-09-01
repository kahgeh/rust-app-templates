#!/bin/bash

# Build script for Rust Web Starter

set -e

echo "🚀 Building Rust Web Starter..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first."
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build the project
echo "🔨 Building project..."
cargo build --release

# Run tests
echo "🧪 Running tests..."
cargo test

# Check formatting
echo "📝 Checking formatting..."
cargo fmt -- --check || echo "⚠️  Some files need formatting. Run 'cargo fmt' to fix."

# Run clippy
echo "🔍 Running clippy..."
cargo clippy -- -D warnings || echo "⚠️  Clippy found some issues."

echo "✅ Build complete! Binary available at: target/release/rust-web-starter"
echo ""
echo "To run the application:"
echo "  ./target/release/rust-web-starter"
echo ""
echo "Or in development mode:"
echo "  cargo run"