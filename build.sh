#!/bin/bash

set -e

build_gamescope() {
    local enable_openvr=$1
    if [ ! -f deps/gamescope/build/src/gamescope ]; then
        echo "Building gamescope submodule..."
        (cd deps/gamescope && meson setup build/ -Denable_openvr_support=$enable_openvr && ninja -C build/)
    fi
}

install_steamdeck_deps() {
    if [ -x scripts/install_steamdeck_deps.sh ]; then
        echo "Installing Steam Deck dependencies..."
        scripts/install_steamdeck_deps.sh
    fi
}

echo "Select build variant:"
echo " 1) Steam Deck (no libbex / Vulkan)"
echo " 2) Standard with dual mouse/keyboard"
read -p "Choice [1/2]: " choice

case "$choice" in
    2)
        build_gamescope true
        ;;
    *)
        install_steamdeck_deps
        build_gamescope false
        ;;
esac

cargo build --release

rm -rf build/partydeck-rs
mkdir -p build/ build/res
cp target/release/partydeck-rs res/PartyDeckKWinLaunch.sh build/
cp res/splitscreen_kwin.js res/splitscreen_kwin_vertical.js build/res
cp deps/gamescope/build/src/gamescope build/res
