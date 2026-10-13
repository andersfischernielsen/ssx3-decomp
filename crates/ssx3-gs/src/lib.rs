//! # ssx3-gs
//!
//! Model of the PlayStation 2 **Graphics Synthesizer** plus a `wgpu` presenter.
//!
//! The GS owns 4 MiB of VRAM. That is exactly `1024 * 1024 * 4` bytes, so the
//! presenter can reinterpret VRAM as a `1024x1024` `RGBA8` texture with no copy
//! and blit it to the window — a convenient debug view while the real GIF
//! packet interpreter and colour-format handling are built out.

use ssx3_core::gs as gsmodel;

pub use gsmodel::{DispFb, VRAM_SIZE};

/// The Graphics Synthesizer state that matters to the runtime.
pub struct Gs {
    /// 4 MiB of embedded DRAM.
    pub vram: Vec<u8>,
    /// Display/environment registers we currently track.
    pub dispfb: [DispFb; 2],
    pub bgcolor: u32,
}

impl Default for Gs {
    fn default() -> Self {
        Self::new()
    }
}

impl Gs {
    pub fn new() -> Self {
        Self {
            vram: vec![0; VRAM_SIZE],
            dispfb: [DispFb::default(); 2],
            bgcolor: 0,
        }
    }

    /// View VRAM as `RGBA8` texels (1024x1024) without copying.
    pub fn vram_as_rgba(&self) -> &[u8] {
        &self.vram
    }

    /// Handle a write to a GS privileged register (`0x12000000..`).
    pub fn write_privileged(&mut self, addr: u32, val: u32) {
        match addr {
            gsmodel::reg::BGCOLOR => self.bgcolor = val,
            gsmodel::reg::DISPFB1 => {
                self.dispfb[0] = decode_dispfb(val);
            }
            gsmodel::reg::DISPFB2 => {
                self.dispfb[1] = decode_dispfb(val);
            }
            _ => {}
        }
    }
}

/// Decode the low 32 bits of a `DISPFB` register. The `fbx`/`fby` fields live in
/// the upper 32 bits, which we do not model yet.
fn decode_dispfb(v: u32) -> DispFb {
    DispFb {
        fbp: v & 0x1ff,
        fbw: (v >> 9) & 0x3f,
        psm: (v >> 15) & 0x1f,
        fbx: 0,
        fby: 0,
    }
}
