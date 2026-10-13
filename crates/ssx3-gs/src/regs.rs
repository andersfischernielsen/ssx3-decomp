//! GS register addresses and GIF constants used by the packet interpreter.
//!
//! The GS exposes two register namespaces:
//!
//! * The **general** 7-bit namespace (`0x00..=0x7F`). A GIF packet addresses
//!   these through its [`crate::gif::GifTag`] register list, or, for addresses
//!   outside the low nibble, through the `A+D` register (`0x0E`). The constants
//!   below are those addresses.
//! * The **privileged** block (`0x1200_0000..`), handled by
//!   [`crate::Gs::write_privileged`]. Registers such as `BGCOLOR` live only
//!   there and are *not* reachable from a GIF packet.
//!
//! These offsets are declared locally (rather than in `ssx3-core`) so the GIF
//! interpreter can grow without perturbing the machine model.

/// `GifTag::flg` values.
pub mod flg {
    /// `NLOOP * NREG` 128-bit packed register words.
    pub const PACKED: u32 = 0;
    /// `NLOOP * NREG` 64-bit register values, registers listed in the tag.
    pub const REGLIST: u32 = 1;
    /// `NLOOP` 128-bit words of raw image data.
    pub const IMAGE: u32 = 2;
    /// Same payload as [`IMAGE`]; seen in a few games and treated identically.
    pub const IMAGE2: u32 = 3;
}

/// `TRXDIR.XDIR`: host (EE) to GS local memory (VRAM).
pub const TRXDIR_HOST_TO_LOCAL: u32 = 0;
/// `TRXDIR.XDIR`: GS local memory to host.
pub const TRXDIR_LOCAL_TO_HOST: u32 = 1;
/// `TRXDIR.XDIR`: GS local memory to GS local memory.
pub const TRXDIR_LOCAL_TO_LOCAL: u32 = 2;
/// `TRXDIR.XDIR`: deactivated; transfers are ignored.
pub const TRXDIR_DEACTIVATED: u32 = 3;

// --- General registers (subset we model, plus the common ones) -------------

/// Drawing primitive setting.
pub const PRIM: u8 = 0x00;
/// Vertex colour plus `Q`.
pub const RGBAQ: u8 = 0x01;
/// Vertex texture coordinates `(S, T)`.
pub const ST: u8 = 0x02;
/// Vertex texel coordinates `(U, V)`.
pub const UV: u8 = 0x03;
/// Vertex coordinate plus fog, drawing kick.
pub const XYZF2: u8 = 0x04;
/// Vertex coordinate, drawing kick.
pub const XYZ2: u8 = 0x05;
/// Texture information, context 1.
pub const TEX0_1: u8 = 0x06;
/// Texture information, context 2.
pub const TEX0_2: u8 = 0x07;
/// Texture wrap mode, context 1.
pub const CLAMP_1: u8 = 0x08;
/// Texture wrap mode, context 2.
pub const CLAMP_2: u8 = 0x09;
/// Vertex fog value.
pub const FOG: u8 = 0x0a;
/// Vertex coordinate plus fog, no kick.
pub const XYZF3: u8 = 0x0c;
/// Vertex coordinate, no kick.
pub const XYZ3: u8 = 0x0d;
/// Register-address / data port used to reach the rest of the namespace.
pub const A_D: u8 = 0x0e;
/// No operation.
pub const NOP: u8 = 0x0f;

/// Buffer transfer setting.
pub const BITBLTBUF: u8 = 0x50;
/// Transfer area position.
pub const TRXPOS: u8 = 0x51;
/// Transfer area size.
pub const TRXREG: u8 = 0x52;
/// Transfer activation.
pub const TRXDIR: u8 = 0x53;
/// Transfer data port.
pub const HWREG: u8 = 0x54;

/// `SIGNAL` event request.
pub const SIGNAL: u8 = 0x60;
/// `FINISH` event request.
pub const FINISH: u8 = 0x61;
/// `LABEL` event request.
pub const LABEL: u8 = 0x62;
