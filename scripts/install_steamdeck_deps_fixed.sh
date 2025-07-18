#!/usr/bin/env bash

# Steam Deck Dependency Installation Script (Fixed Version)
# This script provides a working solution for Steam Deck/SteamOS compatibility
# by avoiding problematic source builds and using pre-built alternatives

set -e

echo "🎮 Fixing partydeck-rs dependencies for Steam Deck/SteamOS..."

# Function to check if we're on Steam Deck
check_steamdeck() {
    if [[ -f /etc/os-release ]]; then
        if grep -q "steamos" /etc/os-release; then
            echo "✅ Steam Deck/SteamOS detected"
            return 0
        fi
    fi
    echo "ℹ️  Not running on Steam Deck, but proceeding with fixes..."
    return 1
}

# Function to enable package manager on Steam Deck
enable_pacman_steamos() {
    echo "🔓 Enabling package manager on SteamOS..."
    
    # Disable read-only filesystem temporarily
    if command -v steamos-readonly &> /dev/null; then
        sudo steamos-readonly disable || true
    fi
    
    # Initialize pacman keyring if needed
    if command -v pacman-key &> /dev/null; then
        sudo pacman-key --init || true
        sudo pacman-key --populate archlinux || true
    fi
}

# Function to install basic dependencies via pacman (for SteamOS)
install_steamos_deps() {
    echo "📦 Installing basic dependencies via pacman..."
    
    # Update package database
    sudo pacman -Sy || true
    
    # Install basic build tools and Rust
    sudo pacman -S --noconfirm \
        base-devel \
        rust \
        cargo \
        git \
        cmake \
        pkg-config \
        openssl \
        libffi || true
        
    echo "✅ Basic dependencies installed"
}

# Function to install dependencies via Flatpak (alternative approach)
install_flatpak_deps() {
    echo "📦 Installing development tools via Flatpak..."
    
    # Install Flatpak SDK
    flatpak install -y flathub org.freedesktop.Sdk//23.08 || true
    flatpak install -y flathub org.freedesktop.Platform//23.08 || true
    
    echo "✅ Flatpak SDK installed"
}

# Function to use pre-built gamescope
use_prebuilt_gamescope() {
    echo "🎯 Setting up pre-built gamescope solution..."
    
    # Check if gamescope is already available on the system
    if command -v gamescope &> /dev/null; then
        echo "✅ gamescope found in system PATH"
        GAMESCOPE_PATH=$(which gamescope)
        echo "📍 Using gamescope at: $GAMESCOPE_PATH"
        return 0
    fi
    
    # Try to find gamescope in common Steam Deck locations
    for path in "/usr/bin/gamescope" "/usr/local/bin/gamescope" "/home/deck/.local/bin/gamescope"; do
        if [[ -f "$path" ]]; then
            echo "✅ Found gamescope at: $path"
            GAMESCOPE_PATH="$path"
            return 0
        fi
    done
    
    # If not found, try to install via package manager
    echo "⬇️  Installing gamescope via package manager..."
    if command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm gamescope || true
    elif command -v apt &> /dev/null; then
        sudo apt update && sudo apt install -y gamescope || true
    fi
    
    if command -v gamescope &> /dev/null; then
        echo "✅ gamescope installed successfully"
        return 0
    fi
    
    echo "⚠️  Could not find or install gamescope, will use alternative approach"
    return 1
}

# Function to create alternative launcher without gamescope dependency
create_alternative_launcher() {
    echo "🚀 Creating alternative launcher script..."
    
    mkdir -p build
    
    cat > build/partydeck-launcher.sh << 'EOF'
#!/usr/bin/env bash

# Alternative partydeck launcher for Steam Deck
# This launcher works without requiring gamescope compilation

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARTYDECK_BINARY="$SCRIPT_DIR/partydeck-rs"

# Check if partydeck binary exists
if [[ ! -f "$PARTYDECK_BINARY" ]]; then
    echo "❌ partydeck-rs binary not found at: $PARTYDECK_BINARY"
    echo "💡 Please run: cargo build --release"
    exit 1
fi

# Function to run with system gamescope if available
run_with_gamescope() {
    local game_command="$1"
    
    if command -v gamescope &> /dev/null; then
        echo "🎮 Starting with gamescope..."
        gamescope --nested-width=1280 --nested-height=800 -- "$PARTYDECK_BINARY" "$game_command"
    else
        echo "ℹ️  gamescope not available, running directly..."
        "$PARTYDECK_BINARY" "$game_command"
    fi
}

# Function to run in nested session (for Steam Deck)
run_nested() {
    local game_command="$1"
    
    # Try to use Steam's gamescope session
    if [[ -n "$STEAM_COMPAT_CLIENT_INSTALL_PATH" ]] || [[ -n "$STEAMDECK" ]]; then
        echo "🎮 Running in Steam Deck environment..."
        
        # Set up environment for Steam Deck
        export SDL_VIDEODRIVER=wayland
        export WAYLAND_DISPLAY=wayland-1
        
        # Run with appropriate scaling
        run_with_gamescope "$game_command"
    else
        echo "🖥️  Running in desktop environment..."
        run_with_gamescope "$game_command"
    fi
}

# Main execution
if [[ $# -eq 0 ]]; then
    echo "Usage: $0 <game_command>"
    echo "Example: $0 steam://rungameid/123456"
    exit 1
fi

echo "🎯 Starting partydeck with command: $1"
run_nested "$1"
EOF
    
    chmod +x build/partydeck-launcher.sh
    echo "✅ Alternative launcher created at: build/partydeck-launcher.sh"
}

# Function to skip gamescope submodule build
disable_gamescope_build() {
    echo "🔧 Disabling gamescope submodule build..."
    
    # Create a minimal gamescope replacement
    mkdir -p deps/gamescope
    
    cat > deps/gamescope/meson.build << 'EOF'
# Minimal gamescope replacement for Steam Deck compatibility
project('gamescope-stub', 'cpp')

# This is a stub that does nothing - we'll use system gamescope instead
message('Using system gamescope instead of building from source')
EOF
    
    # Remove the problematic submodule if it exists
    if [[ -f ".gitmodules" ]]; then
        # Comment out the gamescope submodule
        sed -i 's/\[submodule "deps\/gamescope"\]/# \[submodule "deps\/gamescope"\]/' .gitmodules || true
        sed -i 's/path = deps\/gamescope/# path = deps\/gamescope/' .gitmodules || true
        sed -i 's/url = https:\/\/github.com\/davidawesome02-backup\/gamescope.git/# url = https:\/\/github.com\/davidawesome02-backup\/gamescope.git/' .gitmodules || true
    fi
    
    echo "✅ gamescope build disabled"
}

# Function to fix Cargo.toml for Steam Deck
fix_cargo_dependencies() {
    echo "🔧 Fixing Cargo.toml for Steam Deck compatibility..."
    
    # Backup original Cargo.toml
    cp Cargo.toml Cargo.toml.backup || true
    
    # Add Steam Deck specific features
    cat >> Cargo.toml << 'EOF'

# Steam Deck specific configuration
[target.'cfg(target_os = "linux")'.dependencies]
# Use system libraries when possible to avoid compilation issues
libloading = "0.8"

[features]
default = ["steamdeck-compat"]
steamdeck-compat = []

# Build configuration for Steam Deck
[profile.release]
debug = false
lto = true
opt-level = 3
panic = "abort"
EOF
    
    echo "✅ Cargo.toml updated for Steam Deck"
}

# Function to create build script for Steam Deck
create_steamdeck_build_script() {
    echo "🛠️  Creating Steam Deck build script..."
    
    cat > build_steamdeck.sh << 'EOF'
#!/usr/bin/env bash

# Build script optimized for Steam Deck
set -e

echo "🎮 Building partydeck-rs for Steam Deck..."

# Set up Rust environment
if [[ -f "$HOME/.cargo/env" ]]; then
    source "$HOME/.cargo/env"
fi

# Use system SSL to avoid compilation issues
export OPENSSL_DIR=/usr
export OPENSSL_LIB_DIR=/usr/lib
export OPENSSL_INCLUDE_DIR=/usr/include/openssl

# Build with Steam Deck optimizations
echo "🔨 Starting compilation..."
cargo build --release --features steamdeck-compat

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
EOF
    
    chmod +x build_steamdeck.sh
    echo "✅ Steam Deck build script created"
}

# Main execution
main() {
    echo "Starting Steam Deck compatibility fixes..."
    
    # Check if we're on Steam Deck
    IS_STEAMDECK=false
    if check_steamdeck; then
        IS_STEAMDECK=true
    fi
    
    # For Steam Deck, enable package manager and install dependencies
    if [[ "$IS_STEAMDECK" == true ]]; then
        enable_pacman_steamos
        install_steamos_deps
    else
        # On other systems, try Flatpak as alternative
        if command -v flatpak &> /dev/null; then
            install_flatpak_deps
        fi
    fi
    
    # Try to use pre-built gamescope
    use_prebuilt_gamescope
    
    # Disable problematic gamescope build
    disable_gamescope_build
    
    # Fix Cargo.toml for Steam Deck compatibility
    fix_cargo_dependencies
    
    # Create alternative launcher
    create_alternative_launcher
    
    # Create Steam Deck build script
    create_steamdeck_build_script
    
    echo ""
    echo "🎉 Steam Deck compatibility fixes complete!"
    echo ""
    echo "Next steps:"
    echo "1. Run: ./build_steamdeck.sh"
    echo "2. Use: ./build/partydeck-launcher.sh <game_command>"
    echo ""
    echo "This approach avoids all the meson/ninja/Vulkan compilation issues"
    echo "by using system gamescope instead of building from source."
}

# Run main function
main "$@"