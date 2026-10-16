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
//! - [`mat3`], [`mat6`]: VU1 programs d159716b (type 3) and 9fa47842 (types
//!   6/7, riders) in readable Rust, bit-exact with the translated microcode.
//! - [`mat1`], [`mat2`]: the Bezier terrain programs f4babac5 (type 1,
//!   uniform LOD) and 9790369f (type 2, stitched LOD edges), likewise; the
//!   strip emission, guard test and clipper they share are in [`bezier`].
//! - [`mat4`], [`mat5`]: the particle program f55ee634 (type 4: emitters,
//!   ring emitters, patch glitter) and the SnowBox program e72540c9 (type
//!   5), likewise, with the VU LFSR reproduced exactly ([`mat4::Lfsr`]).
//!
//! Later stages implement each VU1 material natively on wgpu from these
//! calls.

pub mod bezier;
pub mod chain;
pub mod mat1;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod mat5;
pub mod mat6;
pub mod vif;
