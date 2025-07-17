#!/bin/bash
# Install build dependencies for PartyDeck and the bundled gamescope fork on SteamOS / Steam Deck.
# Run this once before building.

set -e

if ! command -v pacman >/dev/null; then
    echo "pacman not found. This script is intended for SteamOS" >&2
    exit 1
fi

# allow writes to the system and update packages
sudo steamos-readonly disable || true

sudo pacman -Syu --needed --noconfirm \
    base-devel meson ninja cmake git \
    clang glslang libcap \
    pipewire sdl2 vulkan-headers libdrm libx11 libxmu \
    libxcomposite libxrender libxres libxtst libxkbcommon \
    libinput wayland wayland-protocols hwdata \
    libxdamage libdecor wlroots \
    xorg-xwayland benchmark \
    libavif libheif aom rav1e luajit

cat <<'EOM'
Dependencies installed. Build gamescope with:
  cd deps/gamescope && meson setup build && ninja -C build
Then run ./build.sh in the project root.
EOM
