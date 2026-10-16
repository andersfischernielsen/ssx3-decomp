//! PCRTC output: PMODE / DISPFB / DISPLAY -> RGBA8 image.

use super::swizzle::{self, Surface};
use super::Gs;

struct Circuit {
    surf: Surface,
    is16: bool,
    dbx: u32,
    dby: u32,
    w: u32,
    h: u32,
}

fn circuit(gs: &Gs, i: usize) -> Option<Circuit> {
    let fb = gs.pr.dispfb[i];
    let disp = gs.pr.display[i];
    let fbp = (fb & 0x1FF) as u32;
    let fbw = ((fb >> 9) & 0x3F) as u32;
    let psm = ((fb >> 15) & 0x1F) as u32;
    let dbx = ((fb >> 32) & 0x7FF) as u32;
    let dby = ((fb >> 43) & 0x7FF) as u32;
    let magh = ((disp >> 23) & 0xF) as u32 + 1;
    let magv = ((disp >> 27) & 3) as u32 + 1;
    let dw = ((disp >> 32) & 0xFFF) as u32 + 1;
    let dh = ((disp >> 44) & 0x7FF) as u32 + 1;
    let w = dw / magh;
    let h = dh / magv;
    if w == 0 || h == 0 || w > 2048 || h > 2048 {
        return None;
    }
    let psm = match psm {
        swizzle::PSMCT24 | swizzle::PSMCT16 | swizzle::PSMCT16S => psm,
        _ => swizzle::PSMCT32,
    };
    Some(Circuit {
        surf: Surface::new(psm, fbp * 32, fbw),
        is16: matches!(psm, swizzle::PSMCT16 | swizzle::PSMCT16S),
        dbx,
        dby,
        w,
        h,
    })
}

impl Circuit {
    /// RGBA8 (alpha kept) of output pixel (x, y); `row_div` halves rows in field mode.
    #[inline]
    fn fetch(&self, vram: &[u32], x: u32, y: u32, row_div: u32) -> u32 {
        let c = self.surf.read(vram, self.dbx + x, self.dby + y / row_div);
        if self.is16 {
            ((c & 0x1F) << 3)
                | ((c & 0x3E0) << 6)
                | ((c & 0x7C00) << 9)
                | if c & 0x8000 != 0 { 0x8000_0000 } else { 0 }
        } else if self.surf.psm == swizzle::PSMCT24 {
            c | 0x8000_0000
        } else {
            c
        }
    }
}

pub(super) fn render(gs: &Gs, out: &mut Vec<u32>) -> (u32, u32) {
    let pmode = gs.pr.pmode;
    let en = [pmode & 1 != 0, pmode & 2 != 0];
    let c = [circuit(gs, 0), circuit(gs, 1)];
    let primary = if en[0] && c[0].is_some() {
        0
    } else if en[1] && c[1].is_some() {
        1
    } else {
        // Nothing displayed: black at the configured (or default) size.
        let (w, h) = c[0].as_ref().map(|c| (c.w, c.h)).unwrap_or((640, 448));
        out.clear();
        out.resize((w * h) as usize, 0xFF00_0000);
        return (w, h);
    };
    let p = c[primary].as_ref().unwrap();
    let (w, h) = (p.w, p.h);
    // SMODE2: INT=1, FFMD=1 (field mode) reads one framebuffer line per field
    // line, so the frame-height output line-doubles the buffer.
    let row_div = if gs.pr.smode2 & 3 == 3 { 2 } else { 1 };
    out.clear();
    out.resize((w * h) as usize, 0);
    let merge = en[0] && en[1] && c[0].is_some() && c[1].is_some();
    let slbg = pmode & 0x80 != 0;
    let mmod = pmode & 0x20 != 0;
    let alp = ((pmode >> 8) & 0xFF) as u32;
    let bg = gs.pr.bgcolor as u32 & 0xFF_FFFF;
    let vram = &gs.vram[..];
    for y in 0..h {
        for x in 0..w {
            let mut px = p.fetch(vram, x, y, row_div);
            if merge {
                let c2 = if slbg {
                    bg
                } else {
                    let c2 = c[1].as_ref().unwrap();
                    if x < c2.w && y < c2.h {
                        c2.fetch(vram, x, y, row_div)
                    } else {
                        0
                    }
                };
                let a = if mmod { alp } else { ((px >> 24) * 2).min(255) };
                let mut o = 0;
                for sh in [0, 8, 16] {
                    let s1 = (px >> sh) & 0xFF;
                    let s2 = (c2 >> sh) & 0xFF;
                    o |= ((s1 * a + s2 * (255 - a) + 127) / 255) << sh;
                }
                px = o;
            }
            out[(y * w + x) as usize] = px | 0xFF00_0000;
        }
    }
    (w, h)
}

/// The display as GPU parameters (`gpu::DisplaySetup`), or `Err((w, h))`
/// when nothing is displayed (black at that size).
pub(super) fn gpu_setup(gs: &Gs) -> Result<super::gpu::DisplaySetup, (u32, u32)> {
    let pmode = gs.pr.pmode;
    let en = [pmode & 1 != 0, pmode & 2 != 0];
    let c = [circuit(gs, 0), circuit(gs, 1)];
    let primary = if en[0] && c[0].is_some() {
        0
    } else if en[1] && c[1].is_some() {
        1
    } else {
        return Err(c[0].as_ref().map(|c| (c.w, c.h)).unwrap_or((640, 448)));
    };
    let p = c[primary].as_ref().unwrap();
    let merge = en[0] && en[1] && c[0].is_some() && c[1].is_some();
    let words = |c: &Circuit| super::gpu::circuit_words(&c.surf, c.is16, c.dbx, c.dby, c.w, c.h);
    let mut circuits = [0u32; 16];
    circuits[..8].copy_from_slice(&words(p));
    if let Some(c2) = &c[1] {
        circuits[8..].copy_from_slice(&words(c2));
    }
    Ok(super::gpu::DisplaySetup {
        w: p.w,
        h: p.h,
        row_div: if gs.pr.smode2 & 3 == 3 { 2 } else { 1 },
        flags: merge as u32 | ((pmode & 0x80 != 0) as u32) << 1 | ((pmode & 0x20 != 0) as u32) << 2,
        alp: ((pmode >> 8) & 0xFF) as u32,
        bg: gs.pr.bgcolor as u32 & 0xFF_FFFF,
        circuits,
        hires: [Some(hires_key(p)), c[1].as_ref().map(hires_key)],
    })
}

/// The buffer a circuit shows and the pages of its displayed rectangle,
/// for the N× copies (`gpu/hires.rs`).
fn hires_key(c: &Circuit) -> (super::gpu::hires::Key, super::raster::Pages) {
    let key = super::gpu::hires::Key { bp: c.surf.base / 64, bw: c.surf.pages_per_row, psm: c.surf.psm, depth: false };
    (key, super::raster::Pages::rect(&c.surf, c.dbx, c.dby, c.dbx + c.w - 1, c.dby + c.h - 1))
}
