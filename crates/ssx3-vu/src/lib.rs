//! # ssx3-vu
//!
//! The PS2 Vector Units (VU0 and VU1), split out of `ssx3-ps2` for build
//! times; it is used as `ssx3_ps2::vu` (re-exported there). It holds the
//! `Vu` register file, pipeline and timing model, the per-op semantic library,
//! macro mode (EE COP2), the microcode translator, VU0's program written by
//! hand (`vu::vu0lib`), and, with feature `microcode`, the statically
//! translated microcode in `generated/vu` (`vu::generated`: the VU1 programs
//! of the emulated GS path, and the reference for tests). See `vu/mod.rs`
//! and `docs/investigation/crate-split.md`.

pub mod vu;

pub use ssx3_ee_rt::Qword;
