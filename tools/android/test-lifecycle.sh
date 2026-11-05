#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
cargo test --profile game -p ssx3 --bin ssx3 android_policy::
cargo test --profile game -p ssx3-iop --lib audio::lifecycle_tests::
