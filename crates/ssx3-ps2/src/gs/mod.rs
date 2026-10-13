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

use crate::Qword;

pub struct Gs {
    pub vram: Vec<u32>,
    priv_regs: [u64; 0x20],
}

impl Default for Gs {
    fn default() -> Self {
        Self::new()
    }
}

impl Gs {
    pub fn new() -> Self {
        Self { vram: vec![0; 1024 * 1024], priv_regs: [0; 0x20] }
    }
    pub fn write_priv(&mut self, offset: u32, value: u64) {
        let i = ((offset >> 4) & 0xf) as usize | if offset >= 0x1000 { 0x10 } else { 0 };
        self.priv_regs[i] = value;
    }
    pub fn read_priv(&mut self, offset: u32) -> u64 {
        let i = ((offset >> 4) & 0xf) as usize | if offset >= 0x1000 { 0x10 } else { 0 };
        self.priv_regs[i]
    }
    pub fn gif_write(&mut self, _path: usize, _data: &[Qword]) {}
    pub fn write_reg(&mut self, _addr: u8, _value: u64) {}
    pub fn irq_pending(&self) -> bool {
        false
    }
    pub fn vblank(&mut self, _field: u32) {}
    pub fn display(&self, out: &mut Vec<u32>) -> (u32, u32) {
        out.clear();
        out.resize(640 * 448, 0);
        (640, 448)
    }
    /// Pop up to `max` quadwords of a pending local->host transfer.
    pub fn read_local(&mut self, _max: usize) -> Vec<Qword> {
        Vec::new()
    }
}
