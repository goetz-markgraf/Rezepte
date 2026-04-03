#!/usr/bin/env bash
set -e

TARGET=armv7-unknown-linux-gnueabihf

echo "Rust-Target sicherstellen..."
rustup target add "$TARGET"

echo "Baue Binary für NAS (ARMv7)..."
cargo zigbuild --release --target "$TARGET"

echo ""
echo "Fertig: target/$TARGET/release/rezepte"
