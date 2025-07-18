#!/bin/bash

set -e

build_gamescope() {
    local enable_openvr=$1
    if [ ! -f deps/gamescope/build/src/gamescope ]; then
        echo "Building gamescope submodule..."
        git submodule update --init --recursive
        (
            cd deps/gamescope && \
            meson setup build/ -Denable_openvr_support="$enable_openvr" && \
            ninja -C build/
        )
    fi
}

ensure_tools() {
    for t in meson ninja; do
        if ! command -v "$t" >/dev/null; then
            echo "$t missing. Run scripts/install_steamdeck_deps.sh first." >&2
            exit 1
        fi
    done
}

install_steamdeck_deps() {
    if command -v pacman >/dev/null; then
        echo "Installing Steam Deck dependencies..."
        sudo steamos-readonly disable >/dev/null 2>&1 || true
        sudo pacman -Syu --needed --noconfirm \
            base-devel meson ninja cmake git \
            clang glslang libcap \
            pipewire sdl2 vulkan-headers libdrm libx11 libxmu \
            libxcomposite libxrender libxres libxtst libxkbcommon \
            libinput wayland wayland-protocols hwdata \
            libxdamage libdecor wlroots libffi libarchive \
            xorg-xwayland benchmark \
            libavif libheif aom rav1e luajit
    fi
}

echo "Select build variant:"
echo " 1) Steam Deck (optimized)"
echo " 2) Steam Deck with keyboard/mouse"
read -p "Choice [1/2]: " choice

install_steamdeck_deps
ensure_tools

if [ "$choice" = "2" ]; then
    build_gamescope true
else
    build_gamescope false
fi


cargo build --release

rm -rf build/partydeck-rs
mkdir -p build/ build/res
cp target/release/partydeck-rs res/PartyDeckKWinLaunch.sh build/
cp res/splitscreen_kwin.js res/splitscreen_kwin_vertical.js build/res
cp deps/gamescope/build/src/gamescope build/res

if command -v pacman >/dev/null; then
    sudo steamos-readonly enable >/dev/null 2>&1 || true
fi
