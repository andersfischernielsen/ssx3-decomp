//! # ssx3-core
//!
//! Machine model for the PlayStation 2 "Emotion Engine" (R5900) as used by
//! **SSX 3 (USA)**, serial `SLUS_207.72`.
//!
//! The constants and segment layout in [`memory_map`] are transcribed from the
//! Ghidra program imported from `orig/SLUS_207.72` (language
//! `r5900:LE:32:default`, image base `0x00100000`, entry `0x00100008`).
//!
//! This crate intentionally has no dependencies so it can be reused by the
//! recompiler, the runtime, and tests alike.

pub mod devices;
pub mod elf;
pub mod gs;
pub mod memory;
pub mod memory_map;
pub mod syscall;

pub use devices::Devices;
pub use elf::Elf;
pub use memory::Memory;
