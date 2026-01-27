#!/bin/bash

# Build script for plugin bots

set -e  # Exit on error

echo "================================"
echo "Hnefatafl Arena - Plugin Builder"
echo "================================"
echo ""

# Check if a plugin name was provided
if [ -z "$1" ]; then
    echo "Usage: ./build_plugin.sh <plugin_name> [--release]"
    echo ""
    echo "Examples:"
    echo "  ./build_plugin.sh greedy_bot_plugin"
    echo "  ./build_plugin.sh my_bot --release"
    echo ""
    echo "Available plugins:"
    ls -1 plugins/ 2>/dev/null | grep -v "^$" || echo "  (none found)"
    exit 1
fi

PLUGIN_NAME=$1
RELEASE_FLAG=""
BUILD_TYPE="debug"

if [ "$2" == "--release" ]; then
    RELEASE_FLAG="--release"
    BUILD_TYPE="release"
fi

PLUGIN_DIR="plugins/$PLUGIN_NAME"

# Check if plugin directory exists
if [ ! -d "$PLUGIN_DIR" ]; then
    echo "❌ Error: Plugin directory '$PLUGIN_DIR' not found!"
    echo ""
    echo "Available plugins:"
    ls -1 plugins/ 2>/dev/null | grep -v "^$" || echo "  (none found)"
    exit 1
fi

echo "📦 Building plugin: $PLUGIN_NAME"
echo "   Build type: $BUILD_TYPE"
echo ""

# Build the plugin
cd "$PLUGIN_DIR"
cargo build $RELEASE_FLAG

# Get the library file name
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so"
    LIB_PREFIX="lib"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
    LIB_PREFIX="lib"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    LIB_EXT="dll"
    LIB_PREFIX=""
else
    echo "⚠️  Warning: Unknown OS type, assuming Linux (.so)"
    LIB_EXT="so"
    LIB_PREFIX="lib"
fi

LIB_FILE="target/$BUILD_TYPE/${LIB_PREFIX}${PLUGIN_NAME}.${LIB_EXT}"

echo ""
echo "✅ Build successful!"
echo ""
echo "📂 Plugin location:"
echo "   $PLUGIN_DIR/$LIB_FILE"
echo ""
echo "🎮 To use this plugin:"
echo "   let bot = PluginBot::load(\"$PLUGIN_DIR/$LIB_FILE\")?;"
echo ""
echo "🔒 To distribute: Share only the compiled library file (not the source code)"
echo "   File: $LIB_FILE"
