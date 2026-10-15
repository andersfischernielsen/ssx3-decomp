//! # ssx3-render: native rendering (phase 3)
//!
//! Stage 0 of `docs/investigation/phase3-render.md`: read the frame the game
//! hands to the hardware at the EE level, without the hardware model.
//!
//! - [`chain`]: walk a VIF1 source DMA chain (CNT/NEXT/REF/CALL/RET/END) in
//!   EE memory, as the DMAC does.
//! - [`vif`]: interpret the VIFcodes into a virtual VU1 data memory and turn
//!   every microprogram start (MSCAL/MSCALF/MSCNT) into a [`vif::VuCall`]
//!   with exactly the input the program sees; DIRECT/DIRECTHL become GIF
//!   streams.
//!
//! Later stages implement each VU1 material natively on wgpu from these
//! calls.

pub mod chain;
pub mod mat3;
pub mod vif;
