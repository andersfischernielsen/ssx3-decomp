//! Graphics Synthesizer: GIF paths, registers, swizzled VRAM, rasterizer.
//!
//! INTERFACE CONTRACT (other modules code against exactly this API):
//! - `Gs::new()`
//! - `write_priv(offset, value)` / `read_priv(offset)`: privileged registers;
//!   `offset` is relative to `0x1200_0000` (CSR is `0x1000`, IMR `0x1010`, ...).
//! - `gif_write(path, data)`: feed whole quadwords to GIF PATH1/2/3 (index 1..=3).
//!   Tag state persists per path across calls (packets may be split).
//! - `write_reg(addr, value)`: a single GS register write (as A+D does).
//! - `irq_pending()`: true while CSR has an unmasked SIGNAL/FINISH/HSYNC/VSYNC/EDW event.
//! - `vblank(field)`: called by the machine at VBlank start; sets CSR.VSINT/FIELD.
//! - `display(&mut out) -> (w, h)`: RGBA8 image of what PMODE/DISPFB/DISPLAY show.
//! - `read_local(...)`: local->host transfer (TRXDIR=1) output, drained by GIF FIFO reads.
//!
//! Implementation: a CPU software GS. See `docs/investigation/gs.md`.
//! Swizzle tables and several semantics are ported from ps2-web-recomp
//! (MIT, https://github.com/snuri00/ps2-web-recomp) and PCSX2 GSdx behaviour.
#![cfg_attr(not(feature = "emu-gs"), allow(dead_code))]

// The guest-visible GS (always): registers, events, GIF tag tracking,
// PATH3 arbitration, swizzled layouts.
mod gif;
pub mod mtgs;
pub mod swizzle;

// The GS core (verification builds, `emu-gs`).
#[cfg(feature = "emu-gs")]
pub mod batch;
#[cfg(feature = "emu-gs")]
pub mod capture;
#[cfg(feature = "emu-gs")]
mod core;
#[cfg(feature = "emu-gs")]
mod display;
#[cfg(feature = "emu-gs")]
pub mod gpu;
#[cfg(feature = "emu-gs")]
mod pool;
#[cfg(feature = "emu-gs")]
mod raster;
#[cfg(feature = "emu-gs")]
mod region;
#[cfg(feature = "emu-gs")]
mod texcache;
#[cfg(feature = "emu-gs")]
mod transfer;
#[cfg(feature = "emu-gs")]
pub use self::core::*;

#[cfg(all(test, feature = "emu-gs"))]
mod diff_tests;
#[cfg(all(test, feature = "emu-gs"))]
mod mtgs_tests;
#[cfg(all(test, feature = "emu-gs"))]
mod ref_raster;
#[cfg(all(test, feature = "emu-gs"))]
mod tests;

#[cfg(feature = "emu-gs")]
pub use mtgs::NativeJob;
pub use mtgs::{Mtgs, MtgsStats};

/// CSR event bits.
pub const CSR_SIGNAL: u64 = 1 << 0;
pub const CSR_FINISH: u64 = 1 << 1;
pub const CSR_HSINT: u64 = 1 << 2;
pub const CSR_VSINT: u64 = 1 << 3;
pub const CSR_EDWINT: u64 = 1 << 4;
const CSR_RESET: u64 = 1 << 9;
const CSR_FIELD: u64 = 1 << 13;
/// FIFO empty (bits 15:14 = 01), REV 0x1B (23:16), ID 0x55 (31:24).
const CSR_FIXED: u64 = 0x551B_4000;
/// Privileged (memory-mapped) registers.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Priv {
    pub pmode: u64,
    pub smode1: u64,
    pub smode2: u64,
    pub srfsh: u64,
    pub synch1: u64,
    pub synch2: u64,
    pub syncv: u64,
    pub dispfb: [u64; 2],
    pub display: [u64; 2],
    pub extbuf: u64,
    pub extdata: u64,
    pub extwrite: u64,
    pub bgcolor: u64,
    pub csr: u64,
    pub imr: u64,
    pub busdir: u64,
    pub siglblid: u64,
}

/// triangles with the hardware rasterizer (phase 3, stage 2B).
/// `SSX3_RENDER_NATIVE` contains `type3`: draw the type-3 VU1 program's
pub fn render_native_type3() -> bool {
    render_native_type3_cached()
}

/// `SSX3_RENDER_NATIVE` lists `name` (comma separated).
fn render_native_has(name: &str) -> bool {
    std::env::var("SSX3_RENDER_NATIVE")
        .is_ok_and(|v| v.split(',').any(|x| x.trim() == name || x.trim() == "all"))
}

/// The VU1 program with this segment hash is drawn natively
/// (`SSX3_RENDER_NATIVE`: `type3`, `terrain` (types 1/2), `rider` (types
/// 6/7), `particles` (types 4/5), or `all`). Cached per hash.
pub fn render_native_hash(hash: u64) -> bool {
    use ssx3_render::{mat1, mat2, mat3, mat4, mat5, mat6};
    static E: std::sync::OnceLock<[bool; 4]> = std::sync::OnceLock::new();
    let on = E.get_or_init(|| {
        [
            render_native_has("type3"),
            render_native_has("terrain"),
            render_native_has("rider"),
            render_native_has("particles"),
        ]
    });
    match hash {
        mat3::HASH => on[0],
        mat1::HASH | mat2::HASH => on[1],
        mat6::HASH => on[2],
        mat4::HASH | mat5::HASH => on[3],
        _ => false,
    }
}

/// Any material is drawn natively (enables the native pipelines).
fn render_native_type3_cached() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| {
        ["type3", "terrain", "rider", "particles", "2d", "post"]
            .iter()
            .any(|n| render_native_has(n))
    })
}

/// Blended PATH2 (2D) primitives use the exact destination-read blend
/// (the blend unit rounds differently by 1-2 on UI edges);
/// `SSX3_N3_EXACT_2D=0` uses the blend unit.
fn exact_2d() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_N3_EXACT_2D").map_or(true, |v| v != "0"))
}

/// `SSX3_RENDER_NATIVE` lists `2d` (or `all`): VIF1 DIRECT/DIRECTHL
/// packets (PATH2: UI, HUD, fonts, menus) are drawn natively.
pub fn render_native_2d() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| render_native_has("2d"))
}

/// `SSX3_RENDER_NATIVE` lists `post` (or `all`): GIF DMA packets (PATH3:
/// post effects such as depth fog, bloom, colour filter) are drawn natively.
pub fn render_native_post() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| render_native_has("post"))
}

fn clamp_trace() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_TRACE_CLAMP").is_ok())
}

/// `SSX3_PATH3_MASK=0` lets PATH3 DMA through immediately, ignoring
/// MSKPATH3/M3R (the old behaviour).
fn path3_mask_enabled() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_PATH3_MASK").map_or(true, |v| v != "0"))
}
