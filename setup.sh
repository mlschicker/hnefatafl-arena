#!/bin/bash

# Hnefatafl Arena - Quick Start Script

set -e

echo "🎮 Hnefatafl Bot Arena Setup"
echo "=============================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust/Cargo found"
echo ""

# Build the project
echo "🔨 Building project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "📚 Available commands:"
echo "----------------------"
echo ""
echo "Run a quick demo match:"
echo "  cargo run --release --example simple_match"
echo ""
echo "Test custom bot example:"
echo "  cargo run --release --example custom_bot"
echo ""
echo "Run main tournament system:"
echo "  cargo run --release"
echo ""
echo "View documentation:"
echo "  cat README.md"
echo "  cat API_REFERENCE.md"
echo "  cat TOURNAMENT.md"
echo ""
echo "🎯 For students:"
echo "  1. Copy examples/bot_template.rs"
echo "  2. Implement your strategy in get_move()"
echo "  3. Test with: cargo run --release --example your_bot"
echo ""
echo "🏆 Ready to create amazing bots!"
