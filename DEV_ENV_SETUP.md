# Developer Environment Setup

This guide explains how to configure the environment for faster builds and smoother development with PartyDeck.

## Dependencies
- Install the latest stable Rust toolchain (`rustup install stable`).
- Ensure `meson` and `ninja` are available for the `deps/gamescope` build.
- On Debian-based distros, you can run:
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential meson ninja-build libx11-dev libxext-dev
  ```
- For additional dependencies, see the `README.md` build section.

## Caching
- Use Rust's incremental compilation (`CARGO_INCREMENTAL=1`) to speed up builds.
- Consider using `sccache` for distributed caching if builds take a long time.

## Quick Build Steps
1. Initialize submodules:
   ```bash
   git submodule update --init
   ```
2. Build Gamescope for keyboard/mouse support:
   ```bash
   (cd deps/gamescope && meson setup build/ && ninja -C build/)
   ```
3. Build the main project:
   ```bash
   ./build.sh
   ```

## Notes
- The `AGENTS.md` file in this repository outlines guidelines for the assistant about when to run builds and tests.
- For minor documentation or single-line code changes, you can skip running the build script to save time.

