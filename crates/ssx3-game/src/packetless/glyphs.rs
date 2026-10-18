//! Font glyphs (slots 80 `DrawGlyphs`, 81 `DrawGlyphsW`) without packets:
//! the glyph sprites as the GIF data the generated bodies send `DIRECT`
//! (`docs/investigation/n2-capture/direct2d.md` §6), one `Step::Gif`.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::vu0::{self, V};
use super::*;
use crate::ps2f::FpuF32;

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// A word as a register holds it after `lw` / `mfc1` (sign-extended).
fn sx64(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// XYZ2 as the generated code ORs it (sign-extended words, 64-bit shifts).
fn xyz(x: i32, y: i32, z: i32) -> u64 {
    sx64(x as u32) | sx64(y as u32) << 16 | sx64(z as u32) << 32
}

/// ST from two float words (the S word sign-extended, as ORed).
fn st(s: FpuF32, t: FpuF32) -> u64 {
    sx64(s.0) | (t.0 as u64) << 32
}

/// The font's glyph for `code` (0: none): below the first direct code
/// none; up to the last direct code by index; else a binary search of the
/// rest of the table, the default glyph when not found.
fn glyph(c: &mut Cpu, font: u32, code: i32) -> u32 {
    let first = r32(c, font + 0xc) as i32;
    if code < first {
        return 0;
    }
    let last = r32(c, font + 0x10) as i32;
    let table = r32(c, font + 4);
    if code <= last {
        return table.wrapping_add((code as u32).wrapping_mul(12)).wrapping_sub((first as u32).wrapping_mul(12));
    }
    let mut lo = last.wrapping_sub(first);
    let mut hi = r32(c, font) as i32;
    loop {
        let mid = lo.wrapping_add(hi) >> 1;
        let e = (mid as u32).wrapping_mul(12).wrapping_add(table);
        let ec = c.bus.read16(e) as i32;
        if code == ec {
            return e;
        }
        if code < ec {
            hi = mid;
        } else {
            lo = mid.wrapping_add(1);
        }
        if lo >= hi {
            return r32(c, font + 8);
        }
    }
}

/// `DrawGlyphs(W)(f12 x, f13 y, f14 sx, f15 sy, f16 uScale, f17 vScale,
/// f18 z, gm, font, text, colour)`: per glyph a sprite {ST, XYZ2, ST,
/// XYZ2}; 2D (screen) or 3D (projected through gm+0x6b30) by the view's
/// font mode. The packet header is always there (NLOOP 0 for no glyphs).
fn draw_glyphs_with(c: &mut Cpu, wide: bool, view_ret: u32) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, font, mut s, col) = (g(c, A0), g(c, A1), g(c, A2), g(c, A3));
    let ff = |c: &Cpu, r: usize| FpuF32(c.f[r]);
    let (mut x, y, sx, sy) = (ff(c, 12), ff(c, 13), ff(c, 14), ff(c, 15));
    let (us, vs, z) = (ff(c, 16), ff(c, 17), ff(c, 18));
    ensure_view_matrices(c, gm, view_ret);
    let view = r32(c, gm + 0x10dc);
    let mode = r32(c, view.wrapping_mul(0x230).wrapping_add(gm).wrapping_add(0x6d40));
    let (ka, kb) = (f(c, gm + 0x6af0), f(c, gm + 0x6b04));
    let (ox, oy) = (f(c, gm + 0x6b20), f(c, gm + 0x6b24));
    // 2D: Z from the font Z scale / offset.
    let zi = (f(c, gm + 0x6b18) * z + f(c, gm + 0x6b28)).to_i32();
    let m = qm(c, gm + 0x6b30);
    let half = FpuF32(0x3f00_0000);
    let mut data: Vec<u128> = Vec::new();
    let mut n = 0u64;
    loop {
        let code = if wide { c.bus.read16(s) as u32 } else { c.bus.read8(s) as u32 };
        if code == 0 {
            break;
        }
        s = s.wrapping_add(if wide { 2 } else { 1 });
        let gl = glyph(c, font, code as i32);
        if gl == 0 {
            continue;
        }
        n += 1;
        let (w, h) = (c.bus.read8(gl + 2) as i32, c.bus.read8(gl + 3) as i32);
        let (u, v) = (c.bus.read16(gl + 4) as i32, c.bus.read16(gl + 6) as i32);
        let adv = c.bus.read8(gl + 8) as i8 as i32;
        let (xoff, yoff) = (c.bus.read8(gl + 9) as i8 as i32, c.bus.read8(gl + 0xa) as i8 as i32);
        let st0 = st(FpuF32::from_i32(u) * us, FpuF32::from_i32(v) * vs);
        let st1 = st(FpuF32::from_i32(u + w) * us, FpuF32::from_i32(v + h) * vs);
        let (p0, p1) = if mode != 0 {
            let x0 = (x + FpuF32::from_i32(xoff) * sx) * ka + ox;
            let y0 = (y + FpuF32::from_i32(yoff) * sy) * kb + oy;
            let x1 = x0 + FpuF32::from_i32(w) * sx * ka;
            let y1 = y0 + FpuF32::from_i32(h) * sy * kb;
            (xyz(x0.to_i32(), y0.to_i32(), zi), xyz(x1.to_i32(), y1.to_i32(), zi))
        } else {
            let hw = FpuF32::from_i32(w) * sx * half;
            let hh = FpuF32::from_i32(h) * sy * half;
            let cx = x + FpuF32::from_i32(xoff) * sx + hw;
            let cy = y + FpuF32::from_i32(yoff) * sy + hh;
            let p = vu0::xform(&m, &[cx.0, cy.0, z.0, vu0::ONE]);
            let iw = FpuF32::ONE.div(FpuF32(p[3]));
            let (dx, dy) = (ka * hw * iw, kb * hh * iw);
            let a: V = [(FpuF32(p[0]) * iw).0, (FpuF32(p[1]) * iw).0, p[2], iw.0];
            let d: V = [dx.0, dy.0, 0, 0];
            let corner = |q: V| xyz(FpuF32(q[0]).to_i32(), FpuF32(q[1]).to_i32(), (FpuF32(q[2]) * FpuF32(q[3])).to_i32());
            (corner(vu0::sub(&a, &d)), corner(vu0::add(&a, &d)))
        };
        data.push(st0 as u128 | (p0 as u128) << 64);
        data.push(st1 as u128 | (p1 as u128) << 64);
        x = x + FpuF32::from_i32(adv) * sx;
    }
    // RGBAQ: colour {a, r, g, b}, RGB × 204, A × 128, Q 1.
    let (k204, k128) = (FpuF32(0x434c_0000), FpuF32(0x4300_0000));
    let r = (f(c, col + 4) * k204).to_i32() as u32;
    let gg = (f(c, col + 8) * k204).to_i32() as u32;
    let b = (f(c, col + 0xc) * k204).to_i32() as u32;
    let a = (f(c, col) * k128).to_i32() as u32;
    let rgbaq = sx64(r) | sx64(gg) << 8 | sx64(b) << 16 | sx64(a) << 24 | 0x3f80_0000u64 << 32;
    // GIFtag {PRIM, RGBAQ}, PRIM 0x56 (sprite, TME, ABE), the colour, then
    // the glyphs' tag (EOP, REGLIST ST XYZ2 ST XYZ2).
    let mut gif = vec![
        0x2400_0000_0000_0001u128 | 0x10 << 64,
        0x56 | (rgbaq as u128) << 64,
        (n | 0x8000 | 0x4400 << 48) as u128 | 0x5252 << 64,
    ];
    gif.extend(data);
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(vec![Step::Gif(gif)]));
    add_record(c, gm, start, start, false);
    ret(c, 80 + 60 * n);
}

/// `PS2GraphicsMan_DrawGlyphs` (slot 80, 0x378808): byte text.
pub fn draw_glyphs(c: &mut Cpu) {
    draw_glyphs_with(c, false, 0x0037_8878)
}

/// `PS2GraphicsMan_DrawGlyphsW` (slot 81, 0x379028): 16-bit text (the
/// generated body builds in a SprStream block; this form leaves the
/// stream alone).
pub fn draw_glyphs_w(c: &mut Cpu) {
    draw_glyphs_with(c, true, 0x0037_909c)
}
