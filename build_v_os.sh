#!/bin/bash
set -e

echo "--- V-OS War-Rig: Build Sequence ---"

# 1. Build Kernel (Can be stable, but needs x86_64-unknown-none)
echo "[1/2] Compiling Rust Kernel..."
# Ensure we have target for stable? Or just use nightly for everything to be safe.
# Stable has the target installed.
cargo build --release --target x86_64-unknown-none

# 2. Build Disk Image (Needs Nightly for bootloader crate)
echo "[2/2] Generating Bootable ISO (UEFI)..."
KERNEL_BIN="target/x86_64-unknown-none/release/v_os_war_rig"

if [ ! -f "$KERNEL_BIN" ]; then
    echo "Error: Kernel binary not found at $KERNEL_BIN"
    exit 1
fi

cd builder
# Use nightly for the builder run
cargo +nightly run --release -- "../$KERNEL_BIN"
cd ..

echo "--- SUCCESS ---"
echo "Image: V-OS-WarRig.iso"
if [ -f "builder/V-OS-WarRig.iso" ]; then
    mv builder/V-OS-WarRig.iso .
    ls -lh V-OS-WarRig.iso
    echo "Ready for deployment."
else
    echo "Error: ISO not found."
    exit 1
fi
