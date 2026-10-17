#!/bin/sh
# Build the browser version into crates/ssx3-web/www (serve that directory:
# crates/ssx3-web/serve.py). Needs the unwind form of the EE code
# (./target/release/recomp-ee --unwind --out generated/ee-unwind
# --package-prefix ssx3-ee-unwind) and wasm-bindgen-cli matching the
# wasm-bindgen in Cargo.lock.
set -e
cd "$(dirname "$0")/../.."
# Deep guest call chains: a 32 MiB stack.
CARGO_TARGET_DIR=target-web \
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C link-arg=-zstack-size=33554432 -C target-feature=+simd128" \
  cargo build --profile web --target wasm32-unknown-unknown -p ssx3-web -j "${JOBS:-4}"
wasm-bindgen --target web --no-typescript --out-dir crates/ssx3-web/www/pkg \
  target-web/wasm32-unknown-unknown/web/ssx3_web.wasm
ls -la crates/ssx3-web/www/pkg
