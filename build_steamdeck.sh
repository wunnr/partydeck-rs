#!/usr/bin/env bash

# Build script optimized for Steam Deck
set -e

echo "🎮 Building partydeck-rs for Steam Deck..."

# Set up Rust environment
if [[ -f "$HOME/.cargo/env" ]]; then
    source "$HOME/.cargo/env"
elif [[ -f "/usr/local/cargo/env" ]]; then
    source "/usr/local/cargo/env"
fi

# Use system SSL to avoid compilation issues
export OPENSSL_DIR=/usr
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH

# Build with Steam Deck optimizations
echo "🔨 Starting compilation..."
cargo build --release

if [[ $? -eq 0 ]]; then
    echo "✅ Build successful!"
    echo "📁 Binary location: target/release/partydeck-rs"
    
    # Copy to build directory for the launcher
    mkdir -p build
    cp target/release/partydeck-rs build/
    
    echo "🚀 You can now use: ./build/partydeck-launcher.sh <game_command>"
else
    echo "❌ Build failed!"
    exit 1
fi
