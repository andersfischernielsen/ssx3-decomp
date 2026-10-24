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
pub mod structure;
pub mod symbols;

/// Write `contents` to `path` only if it differs from what is there. Cargo
/// decides what to rebuild by file mtime, so regenerating identical output
/// (or output where only some shards changed) must not touch the other files.
pub fn write_if_changed(
    path: impl AsRef<std::path::Path>,
    contents: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    let (path, contents) = (path.as_ref(), contents.as_ref());
    if std::fs::read(path).is_ok_and(|old| old == contents) {
        return Ok(());
    }
    std::fs::write(path, contents)
}

pub use boundaries::boundaries;
pub use symbols::{index, Function, FunctionIndex};
