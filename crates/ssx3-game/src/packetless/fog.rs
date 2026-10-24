//! The depth fog post pass (`PS2GraphicsMan_RenderDepthFog`, 0x36ac00)
//! without packets. The generated pass builds, per view, the fog CLUT (an
//! alpha ramp over depth, kept in static memory and rebuilt only when its
//! parameters change), then queues in GifBuff (through the scratchpad
//! stream) the CLUT upload, the Z buffer copy into the CLUT-indexed
//! texture and the fog blend, and puts one PATH3 window into the VIF chain.
//! Here the CLUT and every game-visible effect stay (the parameter reset,
//! the CLUT cache words, the VRAM block allocated and freed); the GIF data
//! is the recorded `Cmd::Post` (as `crate::capture::capture_post` reads
//! it), and neither cursor moves.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, PostKind, Recorded};

use super::vu0::{self, V};
use super::*;
use crate::ps2f::FpuF32;

const VRAM_HEAP_ALLOC: u32 = 0x0036_6ce0;
const VRAM_HEAP_FREE_BLOCK: u32 = 0x0036_63d8;
/// The per-view fog CLUTs (0x400 bytes each).
const CLUTS: u32 = 0x0050_7b80;
/// The default fog colour `ResetDepthFogParams` copies.
const DEFAULT_COLOUR: u32 = 0x0050_4720;

/// A word as a register holds it after `lw` / 32-bit ALU ops.
fn sx64(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// An A+D (or any) qword: `lo` data, `hi` register / upper half.
fn qw(lo: u64, hi: u64) -> u128 {
    lo as u128 | (hi as u128) << 64
}

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// The frame buffer width (graphics-manager virtual method 0x54).
fn width(c: &mut Cpu, gm: u32, ret: u32) -> u32 {
    let (this, func) = model::vmethod(c, gm, 0x54);
    guest_call(c, func, &[this], ret)
}

/// The frame buffer height (virtual method 0x5c).
fn height(c: &mut Cpu, gm: u32, ret: u32) -> u32 {
    let (this, func) = model::vmethod(c, gm, 0x5c);
    guest_call(c, func, &[this], ret)
}

/// `(0x1000 - v) / 2` in 12.4 fixed point (the GS offset of a centred
/// buffer), as the generated code's 32-bit `subu`, `srl 1`, `sll 4`.
fn centre(v: u32) -> u32 {
    (0x1000u32.wrapping_sub(v) >> 1) << 4
}

/// `PS2GraphicsMan_ResetDepthFogParams(gm, view)` (0x36aba0): the view's
/// record at `gm+0x6c64` back to the defaults (`gp+0x123c..0x1248`, the
/// colour at 0x504720).
fn reset_params(c: &mut Cpu, gm: u32, view: u32) {
    let gp = c.g[GP][0] as u32;
    let rec = gm.wrapping_add(view << 5).wrapping_add(0x6c64);
    for k in 0..4 {
        let v = r32(c, gp + 0x123c + 4 * k);
        w32(c, rec + 4 * k, v);
    }
    for k in 0..16 {
        let b = c.bus.read8(DEFAULT_COLOUR + k);
        c.bus.write8(rec + 0x10 + k, b);
    }
}

/// The CLUT index of entry `i` (CSM1: index bits 3 and 4 swapped).
fn swizzle(i: u32) -> u32 {
    i & 0xffff_ffe7 | (i & 8) << 1 | (i & 0x10) >> 1
}

/// `PS2GraphicsMan_BuildDepthFogClut(gm, clut, view)` (0x36a428): the
/// record's near / far (`+0`, `+4`) through the view's matrix
/// (`gm+0x5930+view*0x40`), divided by w; their z / 256 clamped to 0..255
/// (`hi` from near, `lo` from far); the colour (`+0x14..0x1c` × 255). When
/// any of these, the mode (`+8`) or the density (`+0xc`) differs from the
/// view's cache (`gp+0x10b8` valid, `+0x2938` hi, `+0x2940` lo, `+0x2948`
/// colour, `+0x2950` mode, `+0x2958` density), the cache is updated and
/// the CLUT rebuilt: entries below `hi` by the mode (0 linear, 1 exp,
/// 2 exp²), the rest opaque. The generated body builds it in a scratchpad
/// block and DMAs it to `clut`; this writes it there directly.
fn build_clut(c: &mut Cpu, gm: u32, clut: u32, view: u32) {
    let gp = c.g[GP][0] as u32;
    let rec = gm.wrapping_add(view << 5).wrapping_add(0x6c64);
    let mat = gm.wrapping_add(view << 6).wrapping_add(0x5930);
    let m = qm(c, mat);
    let near: V = [0, 0, r32(c, rec), vu0::ONE];
    let p0 = vu0::xform(&m, &near);
    let far: V = [0, 0, r32(c, rec + 4), vu0::ONE];
    let p1 = vu0::xform(&m, &far);
    let i0 = FpuF32::ONE.div(FpuF32(p0[3]));
    let p0 = vu0::mul(&p0, &[i0.0; 4]);
    let i1 = FpuF32::ONE.div(FpuF32(p1[3]));
    let p1 = vu0::mul(&p1, &[i1.0; 4]);
    let clamp = |z: u32| (FpuF32(z).to_i32() >> 8).clamp(0, 0xff);
    let hi = clamp(p0[2]);
    let lo = clamp(p1[2]);
    let span = {
        let s = hi.wrapping_sub(lo).wrapping_add(1);
        if s > 0 {
            s
        } else {
            1
        }
    };
    let k255 = FpuF32(0x437f_0000);
    let g = (f(c, rec + 0x18) * k255).to_i32() as u32;
    let r = (f(c, rec + 0x14) * k255).to_i32() as u32;
    let b = (f(c, rec + 0x1c) * k255).to_i32() as u32;
    let colour = r | g << 8 | b << 16;
    let inv_span = FpuF32::ONE.div(FpuF32::from_i32(span));
    let slot = view << 2;
    let mode = r32(c, rec + 8);
    let density = f(c, rec + 0xc);
    let same = r32(c, gp + 0x10b8 + slot) != 0
        && r32(c, gp + 0x2938 + slot) == hi as u32
        && r32(c, gp + 0x2940 + slot) == lo as u32
        && r32(c, gp + 0x2948 + slot) == colour
        && r32(c, gp + 0x2950 + slot) == mode
        && density.eq_s(f(c, gp + 0x2958 + slot));
    if same {
        return;
    }
    w32(c, gp + 0x10b8 + slot, 1);
    w32(c, gp + 0x2938 + slot, hi as u32);
    w32(c, gp + 0x2940 + slot, lo as u32);
    w32(c, gp + 0x2948 + slot, colour);
    w32(c, gp + 0x2950 + slot, mode);
    w32(c, gp + 0x2958 + slot, density.0);
    let put = |c: &mut Cpu, i: i32, v: u32| w32(c, clut.wrapping_add(swizzle(i as u32) << 2), v);
    match mode as i32 {
        0 => {
            for i in 0..hi {
                let d = (i.max(lo).wrapping_sub(lo) as u32) << 7;
                let a = (d as i32).wrapping_div(span).clamp(0, 0x80);
                put(c, i, colour | (a as u32) << 24);
            }
        }
        m @ (1 | 2) => {
            // (Generated order: f2 = density * k0 * (hi - max(i, lo) + 1)
            // * inv_span, squared for exp², negated; a Horner polynomial in
            // it; clamped to 0..1; × 128.)
            let base = if m == 1 {
                gp.wrapping_sub(0x289c)
            } else {
                gp.wrapping_sub(0x2884)
            };
            let k: [FpuF32; 6] = std::array::from_fn(|j| f(c, base + 4 * j as u32));
            let half = FpuF32(0x3f00_0000);
            let k128 = FpuF32(0x4300_0000);
            for i in 0..hi {
                let n = hi.wrapping_sub(i.max(lo)).wrapping_add(1);
                let mut x = f(c, rec + 0xc) * k[0];
                x = x * FpuF32::from_i32(n);
                x = x * inv_span;
                if m == 2 {
                    x = x * x;
                }
                let x = -x;
                let mut p = x * k[1];
                p = p + k[2];
                p = p * x;
                p = p + k[3];
                p = p * x;
                p = p + k[4];
                p = p * x;
                p = p + k[5];
                p = p * x;
                p = p + half;
                p = p * x;
                p = p + FpuF32::ONE;
                p = p * x;
                p = p + FpuF32::ONE;
                let t = if FpuF32::ZERO.le(p) {
                    p.min(FpuF32::ONE)
                } else {
                    FpuF32::ZERO
                };
                let a = (t * k128).to_i32() as u32;
                put(c, i, colour | a << 24);
            }
        }
        // (Other modes leave the entries below `hi` as the scratchpad block
        // held them; here they stay as the CLUT holds them.)
        _ => {}
    }
    for i in hi..0x100 {
        put(c, i, colour | 0x8000_0000);
    }
}

/// `PS2GraphicsMan_EmitDepthFogClutUpload(gm, clut, tbp, &cursor)`
/// (0x36aa60): CNT {BITBLTBUF (tbp, width 1), TRXPOS, TRXREG 16×16,
/// TRXDIR, an IMAGE tag of 0x40}, REF the CLUT, CNT {TEXFLUSH}.
fn clut_upload(c: &mut Cpu, clut: u32, tbp: u32, gif: &mut Vec<u128>) {
    gif.push(qw(0x1000_0000_0000_0004, 0xe));
    gif.push(qw(sx64(tbp) << 32 | 1 << 48, 0x50));
    gif.push(qw(0, 0x51));
    gif.push(qw(0x10_0000_0010, 0x52));
    gif.push(qw(0, 0x53));
    gif.push(qw(0x0800_0000_0000_0040, 0));
    gif.extend(qwords(c, clut, 0x40));
    gif.push(qw(0x1000_0000_0000_0001, 0xe));
    gif.push(qw(0, 0x3f));
}

/// `PS2GraphicsMan_EmitDepthFogZCopy(gm, view, b80, b84, h2, &cursor)`
/// (0x36ae20; `b80` / `b84` the words at `gm+0x5a80` / `+0x5a84`, `h2`
/// twice the buffer height): the depth buffer copied as a texture
/// (TEX0 from `b80`) into the frame at `b84`: FRAME, ZBUF, TEST, XYOFFSET, SCISSOR, TEXFLUSH, TEXA, TEX1,
/// TEX0, PRIM 0x116, RGBAQ, then sprites {UV, XYZ2, UV, XYZ2} in 16-pixel
/// columns (two per step) over the view's width (`gm+0x5a30`).
fn z_copy(c: &mut Cpu, gm: u32, b80: u32, b84: u32, h2: u32, gif: &mut Vec<u128>) {
    let w = r32(c, gm + 0x5a30) as i32;
    let h = r32(c, gm + 0x5a34);
    let x0 = centre(width(c, gm, 0x0036_ae80));
    let h1 = height(c, gm, 0x0036_aea4);
    let n = (if w > -1 { w } else { w.wrapping_add(7) }) >> 3;
    let zb = r32(c, gm + 0x5a80);
    gif.push(qw(
        sx64((n as u32).wrapping_shl(1).wrapping_add(0xb)) | 1 << 60,
        0xe,
    ));
    let y0 = centre(h1);
    let w2 = width(c, gm, 0x0036_af0c);
    let frame = sx64(b84) | ((w2 >> 6) as u64) << 16 | 0x3fff_0200_0000;
    let zbuf = (zb | 0x0100_0000) as u64 | 1 << 32;
    let xyoff = sx64(x0) | (y0 as u64) << 32;
    let scissor = sx64((w as u32).wrapping_sub(1)) << 16 | sx64((h << 1).wrapping_sub(1)) << 48;
    gif.push(qw(frame, 0x4c));
    gif.push(qw(zbuf, 0x4e));
    gif.push(qw(0x30000, 0x47));
    gif.push(qw(xyoff, 0x18));
    gif.push(qw(scissor, 0x40));
    gif.push(qw(0, 0x3f));
    gif.push(qw(0x80_0000_0000, 0x3b));
    gif.push(qw(0, 0x14));
    let w3 = width(c, gm, 0x0036_aff0);
    let tex0 = sx64(b80 << 5) | ((w3 >> 6) as u64) << 14 | 0xe_ab20_0000;
    gif.push(qw(tex0, 6));
    gif.push(qw(0x116, 0));
    gif.push(qw(0x3f80_0000_8080_8080, 1));
    if n > 0 {
        let v = h2 << 4;
        let t7 = sx64(y0) << 16;
        let t0 = sx64(y0.wrapping_add(v)) << 16;
        let t6 = sx64(v.wrapping_add(8)) << 16;
        let (mut a3, mut a2) = (x0.wrapping_add(0x80), x0.wrapping_add(0x100));
        let (mut t3, mut t2) = (0x88u32, 8u32);
        let mut k = 0;
        loop {
            gif.push(qw(sx64(t2) | 0x80000, 3));
            gif.push(qw(sx64(a3) | t7, 5));
            gif.push(qw(sx64(t3) | t6, 3));
            gif.push(qw(sx64(a2) | t0, 5));
            k += 2;
            t3 = t3.wrapping_add(0x100);
            t2 = t2.wrapping_add(0x100);
            a2 = a2.wrapping_add(0x100);
            a3 = a3.wrapping_add(0x100);
            if k >= n {
                break;
            }
        }
    }
}

/// `PS2GraphicsMan_EmitDepthFogBlend(gm, view, tbp, &cursor)` (0x36b158):
/// the CLUT-indexed copy of Z (at `tbp`) blended over the view's rectangle
/// (`gm+0x59b0+view*8`: x, y, w, h) in 32-pixel columns: FRAME, ZBUF,
/// TEST, XYOFFSET, ALPHA, TEXFLUSH, SCISSOR, TEX1, TEX0 (PSMT8, CLUT),
/// PRIM 0x156, RGBAQ, sprites {UV, XYZ2, UV, XYZ2}; then FRAME, XYOFFSET
/// and SCISSOR (`gm+0x5900`) back to the full buffer.
fn blend(c: &mut Cpu, gm: u32, view: u32, tbp: u32, gif: &mut Vec<u128>) {
    let r = gm.wrapping_add(view << 3);
    let y0 = c.bus.read16(r + 0x59b2) as u32;
    let x0 = c.bus.read16(r + 0x59b0) as u32;
    let w = c.bus.read16(r + 0x59b4) as u32;
    let y1 = y0 + c.bus.read16(r + 0x59b6) as u32;
    let x = (((0x1000u32.wrapping_sub(width(c, gm, 0x0036_b1d4))) >> 1).wrapping_add(x0)) << 4;
    let h1 = height(c, gm, 0x0036_b200);
    let n = w >> 5;
    let b84 = r32(c, gm + 0x5a84);
    let b80 = r32(c, gm + 0x5a80);
    gif.push(qw(sx64((n << 2).wrapping_add(0xb)) | 1 << 60, 0xe));
    let y = (((0x1000u32.wrapping_sub(h1)) >> 1).wrapping_add(y0)) << 4;
    let w2 = width(c, gm, 0x0036_b288);
    let frame = b84 as u64 | ((w2 >> 6) as u64) << 16;
    let zbuf = (b80 | 0x0100_0000) as u64 | 1 << 32;
    let xyoff = sx64(x) | (y as u64) << 32;
    let scissor = x0 as u64
        | sx64((x0 + w).wrapping_sub(1)) << 16
        | (y0 as u64) << 32
        | sx64(y1.wrapping_sub(1)) << 48;
    gif.push(qw(frame, 0x4c));
    gif.push(qw(zbuf, 0x4e));
    gif.push(qw(0x70000, 0x47));
    gif.push(qw(xyoff, 0x18));
    gif.push(qw(1, 0x42));
    gif.push(qw(0, 0x3f));
    gif.push(qw(scissor, 0x40));
    gif.push(qw(0, 0x14));
    let w3 = width(c, gm, 0x0036_b374);
    let tex0 =
        (b84 << 5) as u64 | ((w3 >> 6) as u64) << 14 | sx64(tbp) << 37 | 0xe_a9b0_0000 | 1 << 61;
    gif.push(qw(tex0, 6));
    gif.push(qw(0x156, 0));
    gif.push(qw(0x3f80_0000_8080_8080, 1));
    if n != 0 {
        let a2b = x0 << 4;
        let (v0, v1) = (y0 << 4, y1 << 4);
        let s0 = sx64(y.wrapping_add(v0)) << 16;
        let s1 = sx64(y.wrapping_add(v1)) << 16;
        let t7 = sx64(v0 + 8) << 16;
        let t6 = sx64(v1 + 8) << 16;
        let t4 = 0xffffu64 << 32;
        let mut a3 = a2b + 8;
        let mut t0 = a2b.wrapping_add(x);
        let mut t1 = (a2b + 0x200).wrapping_add(x);
        let mut a2 = a2b + 0x208;
        for _ in 0..n {
            gif.push(qw(sx64(a3) | t7, 3));
            gif.push(qw(sx64(t0) | s0 | t4, 5));
            gif.push(qw(sx64(a2) | t6, 3));
            gif.push(qw(sx64(t1) | s1 | t4, 5));
            a2 = a2.wrapping_add(0x200);
            a3 = a3.wrapping_add(0x200);
            t1 = t1.wrapping_add(0x200);
            t0 = t0.wrapping_add(0x200);
        }
    }
    // The second CNT: back to the full frame.
    gif.push(qw(0x1000_0000_0000_8003, 0xe));
    let b84 = r32(c, gm + 0x5a84);
    let w4 = width(c, gm, 0x0036_b4e8);
    gif.push(qw(b84 as u64 | ((w4 >> 6) as u64) << 16, 0x4c));
    let ox = centre(width(c, gm, 0x0036_b520));
    let oy = centre(height(c, gm, 0x0036_b540));
    gif.push(qw(sx64(ox) | (oy as u64) << 32, 0x18));
    let h16 = |c: &mut Cpu, o: u32| c.bus.read16(gm + o) as u32;
    let (sx0, sw, sy0, sh) = (
        h16(c, 0x5900),
        h16(c, 0x5904),
        h16(c, 0x5902),
        h16(c, 0x5906),
    );
    let sc = sx0 as u64
        | sx64(sx0.wrapping_sub(1).wrapping_add(sw)) << 16
        | (sy0 as u64) << 32
        | sx64(sy0.wrapping_sub(1).wrapping_add(sh)) << 48;
    gif.push(qw(sc, 0x40));
}

/// `PS2GraphicsMan_RenderDepthFog(gm, view, &vifCursor, &gifCursor)`
/// (0x36ac00, a `RenderFrame` post pass): off while `gp+0x1234` is 0; the
/// view's parameters reset first while `gp+0x1238` is set; nothing for a
/// zero density with a non-zero mode. Else the CLUT (`build_clut`), a
/// 0x400-byte VRAM block from TexMan's heap for the indexed Z copy (its TBP
/// in the upload, the copy's frame and the blend's texture), freed again
/// after, and one PATH3 window. Records `Cmd::Post` (empty when the pass
/// draws nothing, as the capture reads it).
pub fn render_depth_fog(c: &mut Cpu) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, view, vif) = (g(c, A0), g(c, A1), g(c, A2));
    let at = r32(c, vif);
    let gp = g(c, GP);
    let mut gif = Vec::new();
    let mut windows = 0;
    'pass: {
        if r32(c, gp + 0x1234) == 0 {
            break 'pass;
        }
        if r32(c, gp + 0x1238) != 0 {
            reset_params(c, gm, view);
        }
        let rec = gm.wrapping_add(view << 5).wrapping_add(0x6c64);
        if f(c, rec + 0xc).eq_s(FpuF32::ZERO) && r32(c, rec + 8) != 0 {
            break 'pass;
        }
        let clut = CLUTS.wrapping_add(view << 10);
        build_clut(c, gm, clut, view);
        let heap = r32(c, gm + 0x18f4).wrapping_add(0x1f60);
        let block = guest_call(
            c,
            VRAM_HEAP_ALLOC,
            &[heap, 0x400, 0x7fff_ffff, 0, 0, 1],
            0x0036_ace8,
        );
        let tm = r32(c, gm + 0x18f4);
        let node = r32(c, tm + 0x3f50).wrapping_add(block.wrapping_mul(0x1c));
        let tbp = (r32(c, node) as i32 >> 8) as u32;
        clut_upload(c, clut, tbp, &mut gif);
        let b80 = r32(c, gm + 0x5a80);
        let h = height(c, gm, 0x0036_ad60);
        let b84 = r32(c, gm + 0x5a84);
        z_copy(c, gm, b80, b84, h << 1, &mut gif);
        let tbp = (r32(c, node) as i32 >> 8) as u32;
        blend(c, gm, view, tbp, &mut gif);
        let heap = r32(c, gm + 0x18f4).wrapping_add(0x1f60);
        guest_call(c, VRAM_HEAP_FREE_BLOCK, &[heap, block], 0x0036_ade4);
        // (`TexMan_EmitPath3Window`: one REF in the VIF chain.)
        windows = 1;
    }
    ssx3_render::direct::record(Recorded {
        start: at,
        end: at,
        first_tag: 0,
        cmd: Cmd::Post {
            kind: PostKind::DepthFog,
            gif,
            windows,
        },
    });
    ret(c, if windows == 0 { 40 } else { 3000 });
}
