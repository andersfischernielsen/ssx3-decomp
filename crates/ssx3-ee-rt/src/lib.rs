//! # ssx3-ee-rt
//!
//! The stable core that recompiled EE code (`generated/ee/*`) is compiled
//! against. It holds only what generated functions reference directly:
//!
//! - [`Cpu`]: R5900 registers, guest RAM/scratchpad with inline fast paths,
//!   and the instruction semantics (`cpu`, `fpu`, `mmi`, `cop2`, `float`);
//! - the address → function table and `call_indirect` ([`funcs`]);
//! - the [`Platform`] trait object through which everything else is reached:
//!   memory-mapped I/O, VU0 (COP2 macro mode, [`vu0::Vu0`]), and the
//!   [`Hooks`] (syscalls, scheduled events, fatal errors).
//!
//! The PS2 hardware and firmware model (`ssx3-ps2`: DMAC, INTC, timers,
//! VIF/VU, GS, IPU, SIF/IOP, kernel HLE) implements [`Platform`]. Because
//! the generated crates depend only on this crate, a change to the hardware
//! model rebuilds `ssx3-ps2` and relinks; it does not recompile the ~80 MB
//! of generated code. Keep this crate small and change it rarely:
//! `docs/investigation/crate-split.md`.

pub mod bus;
pub mod cop2;
pub mod cpu;
pub mod float;
pub mod fpu;
pub mod funcs;
pub mod mmi;
pub mod platform;
pub mod vu0;

pub use cpu::{Cpu, Hooks};
pub use platform::Platform;

/// One 128-bit quadword as moved by DMA, GIF and VIF.
pub type Qword = u128;
