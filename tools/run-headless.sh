#!/bin/sh
# Regenerate (optional), build and run the recompiled game headless.
#   tools/run-headless.sh [--regen] FRAMES [--shot-every N]
set -e
cd "$(dirname "$0")/.."
if [ "$1" = "--regen" ]; then
  shift
  cargo run -q --release -p ssx3-recomp --bin recomp-ee
fi
cargo build -q --profile game -p ssx3 --bin ssx3-headless
exec perl -e 'alarm shift; exec @ARGV' "${SSX3_TIMEOUT:-600}" ./target/game/ssx3-headless "$@"
