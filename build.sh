#!/bin/bash

cargo build --release && \
rm -rf build/partydeck-rs
mkdir -p build/ build/res && \
cp target/release/partydeck-rs build/ && \
cp res/splitscreen_kwin.js res/splitscreen_kwin_vertical.js build/res && \
cp deps/gamescope/build/src/gamescope build/res
