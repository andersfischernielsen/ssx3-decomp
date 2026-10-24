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
//! - [`frame`]: stage 0b, one frame as typed draws: the DeferMan buckets
//!   (decoded [`frame::RenderState`], textures, MatrixRecord) with their
//!   payloads from the chain walk, plus the depth-fog / phase / bloom /
//!   colour-filter markers. [`gif`] reads the A+D writes of GIF streams.
//!
//! Later stages implement each VU1 material natively on wgpu from these
//! calls.

pub mod bezier;
pub mod chain;
pub mod direct;
pub mod frame;
pub mod gif;
pub mod gpu;
pub mod mat1;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod mat5;
pub mod mat6;
pub mod vif;
pub mod vujob;
pub mod world_casters;

/// Run a VU1 microprogram call natively if its segment (`hash`) and entry
/// are implemented: kicks go to `kick`, VU1 data memory is updated exactly
/// as the microprogram would. False when the call is not covered (the
/// caller then runs the translated microcode).
pub fn run_native(hash: u64, entry: u32, top: u32, itop: u32, mem: &mut [u8; 16384], kick: &mut dyn FnMut(&[u128])) -> bool {
    match hash {
        mat1::HASH => mat1::implemented(entry) && mat1::run(entry, top, itop, mem, &mut Default::default(), kick),
        mat2::HASH => mat2::implemented(entry) && mat2::run(entry, top, itop, mem, &mut Default::default(), kick),
        mat3::HASH => mat3::implemented(entry) && mat3::run(entry, top, itop, mem, &mut Default::default(), kick),
        mat4::HASH => mat4::implemented(entry) && mat4::run(entry, top, itop, mem, &mut Default::default(), kick),
        mat5::HASH => mat5::implemented(entry) && mat5::run(entry, top, itop, mem, &mut Default::default(), kick),
        mat6::HASH => mat6::implemented(entry) && mat6::run(entry, top, itop, mem, &mut Default::default(), kick),
        _ => false,
    }
}
