#!/bin/bash
# Script to build a bootable ISO for Aegis-Rust
# Usage: ./build_iso.sh

set -e

# 1. Check for bootimage
if ! command -v bootimage &> /dev/null; then
    echo "bootimage not found. Installing..."
    cargo install bootimage
fi

# 2. Compile and Build Bootable Image
echo "Compiling kernel and creating bootable image..."
# 'cargo bootimage' (which is 'bootimage' invoked via cargo) handles the build and image creation
cargo bootimage -Z build-std=core,compiler_builtins,alloc --target x86_64-unknown-none --release

echo "Build complete. The bootable image is located in target/x86_64-unknown-none/release/bootimage-aegis_rust.bin"
