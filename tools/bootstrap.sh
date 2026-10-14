#!/bin/sh
# Fresh checkout: the workspace references generated crates (generated/ee/*,
# generated/iop/modules) that recomp-ee produces. Write empty stub crates so
# cargo can load the workspace, then run the real generation.
#   tools/bootstrap.sh [recomp-ee args...]
set -e
cd "$(dirname "$0")/.."
if [ ! -f generated/ee/game/Cargo.toml ]; then
  mkdir -p generated/ee/game/src
  printf '[package]\nname = "ssx3-ee"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nssx3-ps2 = { path = "../../../crates/ssx3-ps2" }\n' > generated/ee/game/Cargo.toml
  printf 'pub static FUNCTIONS: [ssx3_ps2::machine::FnEntry; 0] = [];\npub const HOST_THREADS: bool = false;\n' > generated/ee/game/src/lib.rs
fi
if [ ! -f generated/iop/modules/Cargo.toml ]; then
  mkdir -p generated/iop/modules/src
  printf '[package]\nname = "ssx3-iop-gen"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nssx3-iop = { path = "../../../crates/ssx3-iop" }\n' > generated/iop/modules/Cargo.toml
  printf 'pub static GENERATED: ssx3_iop::Generated = ssx3_iop::Generated { functions: &[], stubs: &[], modules: &[] };\n' > generated/iop/modules/src/lib.rs
fi
cargo run --release -p ssx3-recomp --bin recomp-ee -- "$@"
