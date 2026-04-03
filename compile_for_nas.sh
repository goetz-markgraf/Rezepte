#!/usr/bin/env bash
set -e

TARGET=armv7-unknown-linux-gnueabihf
# GLIBC 2.26 targeten (Version auf dem NAS)
ZIG_TARGET="${TARGET}.2.26"

echo "Rust-Target sicherstellen..."
rustup target add "$TARGET"

echo "Baue Binary für NAS (ARMv7, GLIBC 2.26, Cortex-A5)..."
# RUSTFLAGS: -C target-cpu=cortex-a5 für ältere ARMv7 ohne NEON
RUSTFLAGS="-C target-cpu=cortex-a5" cargo zigbuild --release --target "$ZIG_TARGET"

echo ""
echo "Fertig: target/$TARGET/release/rezepte"
