#!/bin/bash
# Custom build script for Typer Racer with clipboard support

# Set RUSTFLAGS for web_sys_unstable_apis
export RUSTFLAGS="--cfg=web_sys_unstable_apis"
echo "Building with RUSTFLAGS=$RUSTFLAGS"

# Build with trunk
trunk build --release

echo "Build completed!"
