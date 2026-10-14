//! # ssx3-recomp
//!
//! The R5900 → Rust recompiler for SSX 3.
//!
//! This crate starts with the **function inventory**: the full list of
//! functions Ghidra found in `SLUS_207.72` (name + address). That inventory is
//! the work list the translator generates against.
//!
//! The data comes from `generated/functions.tsv`, produced by
//! [`tools/export-ghidra-functions.py`] against the GhidraMCP backend. It is
//! embedded at compile time, so the inventory is available without any runtime
//! files.

pub mod boundaries;
pub mod codegen;
pub mod eegen;
pub mod flow;
pub mod iopgen;
pub mod program;
pub mod r5900;
pub mod symbols;

pub use boundaries::boundaries;
pub use symbols::{index, Function, FunctionIndex};
