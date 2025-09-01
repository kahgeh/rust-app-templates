#!/bin/bash

# Development run script for Rust Web Starter

set -e

echo "🚀 Starting Rust Web Starter in development mode..."

# Set development environment
export RUN_ENVIRONMENT=${RUN_ENVIRONMENT:-development}
export RUST_LOG=${RUST_LOG:-info}

echo "📦 Environment: $RUN_ENVIRONMENT"
echo "📝 Log level: $RUST_LOG"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first."
    exit 1
fi

# Run with cargo watch if available, otherwise use cargo run
if command -v cargo-watch &> /dev/null; then
    echo "👀 Starting with cargo-watch (auto-reload on changes)..."
    cargo watch -x run
else
    echo "🏃 Starting server..."
    echo "💡 Tip: Install cargo-watch for auto-reload: cargo install cargo-watch"
    cargo run
fi