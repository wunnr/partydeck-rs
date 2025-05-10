#!/bin/bash

cargo build --release && \
rm -rf build/partydeck-rs
mkdir -p build/ build/res && \
cp target/release/partydeck-rs PartyDeckKWinLaunch.sh build/ && \
cp res/PartyDeckKWinLaunch.sh build/ && \
cp res/splitscreen_kwin.js res/wine_disable_hidraw.reg build/res
