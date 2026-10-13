//! # ssx3-ps2
//!
//! A PlayStation 2 machine model sufficient to run SSX 3 (`SLUS_207.72`).
//! See `docs/plans/playable-push.md` for the architecture.

pub mod gs;
pub mod vif;
pub mod vu;

/// One 128-bit quadword as moved by DMA, GIF and VIF.
pub type Qword = u128;
