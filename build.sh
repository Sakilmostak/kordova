#!/bin/bash

# Build script for Library Management System
echo "🦀 Building Library Management System..."

cd ~/.openclaw/shared-project/library-management-system

# Check for Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

# Build the project
echo "📦 Compiling Rust code..."
cargo check 2>&1

if [ $? -eq 0 ]; then
    echo "✅ Rust code compilation successful!"
    echo ""
    echo "🔧 To run the application:"
    echo "1. Set up PostgreSQL and Redis"
    echo "2. Copy .env.example to .env and configure"
    echo "3. Run: cargo run"
    echo ""
    echo "📡 Server will start on http://localhost:3000"
else
    echo "❌ Compilation failed. Check the errors above."
    exit 1
fi