//! Graphics Synthesizer (GS) model constants shared across the workspace.
//!
//! The GS has 4 MiB of embedded DRAM (VRAM) that holds both the framebuffer
//! and textures. The renderer crate presents VRAM; the runtime fills it by
//! executing GIF packets (the DMA path from EE RAM to the GS).

/// GS VRAM size: 4 MiB, addressed as 1024 pages of 8 KiB (256x256 bytes).
pub const VRAM_SIZE: usize = 4 * 1024 * 1024;
pub const VRAM_PAGE: usize = 256;
pub const VRAM_WIDTH: usize = 2048;
pub const VRAM_HEIGHT: usize = 2048;

/// GS privileged register block base (0x12000000..=0x12001FFF).
pub const REG_BASE: u32 = 0x1200_0000;

/// A few well-known privileged registers (offset from [`REG_BASE`]).
pub mod reg {
    pub const PMODE: u32 = 0x1200_0000;
    pub const SMODE1: u32 = 0x1200_0010;
    pub const SMODE2: u32 = 0x1200_0020;
    pub const SRFSH: u32 = 0x1200_0030;
    pub const SYNCH1: u32 = 0x1200_0040;
    pub const SYNCH2: u32 = 0x1200_0050;
    pub const SYNCV: u32 = 0x1200_0060;
    pub const DISPFB1: u32 = 0x1200_0070;
    pub const DISPLAY1: u32 = 0x1200_0080;
    pub const DISPFB2: u32 = 0x1200_0090;
    pub const DISPLAY2: u32 = 0x1200_00a0;
    pub const BGCOLOR: u32 = 0x1200_00e0;
}

/// The two framebuffer circuits the GS can display (or interlace together).
#[derive(Debug, Clone, Copy, Default)]
pub struct DispFb {
    /// Frame buffer base pointer in units of 2048 words (8 KiB blocks).
    pub fbp: u32,
    /// Frame buffer width in units of 64 pixels.
    pub fbw: u32,
    /// Packed 24-bit format.
    pub psm: u32,
    /// Frame buffer position (x, y).
    pub fbx: u32,
    pub fby: u32,
}

impl DispFb {
    /// Byte offset of this framebuffer inside VRAM.
    pub fn vram_offset(&self) -> usize {
        (self.fbp as usize * 8192) & (crate::gs::VRAM_SIZE - 1)
    }
}
