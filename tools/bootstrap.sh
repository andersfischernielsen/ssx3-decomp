#!/bin/sh
# Fresh checkout: the workspace references generated crates (generated/ee/*,
# generated/iop/modules) that recomp-ee produces. Write empty stub crates so
# cargo can load the workspace, then run the real generation.
#   tools/bootstrap.sh [recomp-ee args...]
set -e
cd "$(dirname "$0")/.."
if [ ! -f generated/ee/game/Cargo.toml ]; then
  mkdir -p generated/ee/game/src
  printf '[package]\nname = "ssx3-ee"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nssx3-ee-rt = { path = "../../../crates/ssx3-ee-rt" }\n' > generated/ee/game/Cargo.toml
  printf 'pub static FUNCTIONS: [ssx3_ee_rt::funcs::FnEntry; 0] = [];\npub const HOST_THREADS: bool = false;\n' > generated/ee/game/src/lib.rs
fi
if [ ! -f generated/ee-unwind/game/Cargo.toml ]; then
  # The browser build's copy (recomp-ee --unwind); only its manifest is needed here.
  mkdir -p generated/ee-unwind/game/src
  printf '[package]\nname = "ssx3-ee-unwind"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nssx3-ee-rt = { path = "../../../crates/ssx3-ee-rt" }\n' > generated/ee-unwind/game/Cargo.toml
  printf 'pub static FUNCTIONS: [ssx3_ee_rt::funcs::FnEntry; 0] = [];\npub const HOST_THREADS: bool = false;\n' > generated/ee-unwind/game/src/lib.rs
fi
if [ ! -f generated/iop/modules/Cargo.toml ]; then
  mkdir -p generated/iop/modules/src
  printf '[package]\nname = "ssx3-iop-gen"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nssx3-iop = { path = "../../../crates/ssx3-iop" }\n' > generated/iop/modules/Cargo.toml
  printf 'pub static GENERATED: ssx3_iop::Generated = ssx3_iop::Generated { functions: &[], stubs: &[], modules: &[] };\n' > generated/iop/modules/src/lib.rs
fi
if [ ! -f generated/vu/mod.rs ] || grep -q '^//! Placeholder' generated/vu/mod.rs; then
  # Empty registry so ssx3-vu builds; vu_recomp then translates the microcode.
  # A placeholder left by a failed run counts as missing.
  mkdir -p generated/vu
  printf '//! Placeholder; tools/bootstrap.sh replaces it.\nuse crate::vu::ProgramEntry;\n\npub static PROGRAMS: &[ProgramEntry] = &[];\n' > generated/vu/mod.rs
  cargo run --release -p ssx3-ps2 --bin vu_recomp
fi
cargo run --release -p ssx3-recomp --bin recomp-ee -- "$@"
