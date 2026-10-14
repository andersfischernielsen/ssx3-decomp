//! # ssx3-ps2
//!
//! The PlayStation 2 platform layer for the SSX 3 recompilation: everything
//! that is hardware or firmware rather than game code. Game code (EE and VU
//! microcode) is translated to Rust by `ssx3-recomp`; it runs against [`Cpu`]
//! and calls into this crate for instruction semantics, memory-mapped I/O,
//! the kernel, and the IOP services. See `docs/plans/playable-push.md`.

pub mod bus;
pub mod cop2;
pub mod cpu;
pub mod elf;
pub mod fpu;
pub mod gifstats;
pub mod gs;
pub mod hw;
pub mod ipu;
pub mod iso;
pub mod kernel;
pub mod machine;
pub mod mmi;
pub mod pad;
pub mod sif;
pub mod vif;
pub mod vu;

pub use cpu::Cpu;

/// One 128-bit quadword as moved by DMA, GIF and VIF.
pub type Qword = u128;
