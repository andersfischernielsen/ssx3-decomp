//! The post passes `RenderFrame` runs at layer threshold 9 (bloom, colour
//! filter) without packets. The generated bodies build GIF blocks in
//! scratchpad (`SprStream` mode 5), copy them to the GifBuff cursor and
//! put one PATH3 window into the VIF chain; `crate::capture::capture_post`
//! reads the GIF data of those CNT transfers back. Here the transfers are
//! built as qwords (tags included) and read the same way; the recorded
//! `Cmd::Post` is what the capture would read. Neither cursor moves, no
//! packet is written and the scratchpad stream is left alone; the per-view
//! parameter resets and the graphics manager's virtual getters stay.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, PostKind, Recorded};

use super::*;
use crate::ps2f::FpuF32;

/// A word as a register holds it after `lw` / `mfc1` (sign-extended).
fn sx64(v: i32) -> u64 {
    v as i64 as u64
}

fn qw(lo: u64, hi: u64) -> u128 {
    lo as u128 | (hi as u128) << 64
}

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// `div` as the runtime does it (quotient only).
fn idiv(n: i32, d: i32) -> i32 {
    if d == 0 {
        if n < 0 {
            1
        } else {
            -1
        }
    } else {
        n.wrapping_div(d)
    }
}

/// `x / 32` rounded toward zero (`(x < 0 ? x + 31 : x) >> 5`).
fn div32(x: i32) -> i32 {
    (if x < 0 { x.wrapping_add(0x1f) } else { x }) >> 5
}

/// `max(1, x / 64)` rounded toward zero: a width in 64-pixel units (TBW /
/// FBW).
fn units64(x: i32) -> i32 {
    let u = (if x < 0 { x.wrapping_add(0x3f) } else { x }) >> 6;
    if u > 0 {
        u
    } else {
        1
    }
}

/// The graphics manager's virtual getter at `slot` (0x54: frame width,
/// 0x5c: frame height); `ret` the generated caller's return address.
fn vget(c: &mut Cpu, gm: u32, slot: u32, ret: u32) -> u32 {
    let (this, func) = model::vmethod(c, gm, slot);
    guest_call(c, func, &[this], ret)
}

/// The GIF data of a chain of CNT transfers, as `capture_post` reads it.
fn gif_data(stream: &[u128]) -> Vec<u128> {
    let mut out = Vec::with_capacity(stream.len());
    let mut t = 0usize;
    while t < stream.len() {
        let qwc = (stream[t] & 0xffff) as usize;
        let end = (t + 1 + qwc).min(stream.len());
        out.extend_from_slice(&stream[t + 1..end]);
        t += 1 + qwc;
    }
    out
}

fn record_post(vif: u32, kind: PostKind, gif: Vec<u128>, windows: u32) {
    direct::record(Recorded { start: vif, end: vif, first_tag: 0, cmd: Cmd::Post { kind, gif, windows } });
}

/// 127.5: the colour scale of the post passes.
const K127_5: FpuF32 = FpuF32(0x42ff_0000);
/// The GIF tag lo word bit NREG = 1 (A+D).
const NREG1: u64 = 0x1000_0000_0000_0000;
const CNT: u64 = 0x1000_0000;

// ---------------------------------------------------------------------------
// Colour filter.

/// `PS2GraphicsMan_ResetColorFilter(gm, view)` (0x390458): the view's six
/// filter floats (`gm + view*0x18 + 0x6ca4`) from the defaults at
/// `gp+0x14f4..0x1508`.
fn reset_color_filter(c: &mut Cpu, gm: u32, view: u32, gp: u32) {
    let p = gm.wrapping_add(view.wrapping_mul(0x18)).wrapping_add(0x6ca4);
    for k in 0..6 {
        let v = r32(c, gp + 0x14f4 + 4 * k);
        w32(c, p + 4 * k, v);
    }
}

/// `PS2GraphicsMan_EmitColorFilterPacket(gm, view, &block)` (0x3905e8): a
/// CNT transfer of one A+D list (frame/test/scissor setup, the view's
/// multiply pass as 32-pixel sprite strips with the RGB scale, an additive
/// pass with the RGB bias when it is not zero), then a CNT of 4 (FRAME,
/// XYOFFSET, SCISSOR back).
fn color_filter_stream(c: &mut Cpu, gm: u32, view: u32) -> Vec<u128> {
    let p = gm.wrapping_add(view.wrapping_mul(0x18)).wrapping_add(0x6ca4);
    let vo = gm.wrapping_add(view << 3);
    let fbp = r32(c, gm + 0x5a84);
    let w80 = r32(c, gm + 0x5a80);
    let h = c.bus.read16(vo + 0x59b6) as i32;
    let w = c.bus.read16(vo + 0x59b4) as i32;
    let x0 = c.bus.read16(vo + 0x59b0) as i32;
    let y0 = c.bus.read16(vo + 0x59b2) as i32;
    // (Generated: f0 = p[3] * 127.5, f3 = p[4] * 127.5, f1 = p[5] * 127.5.)
    let c3 = (f(c, p + 0xc) * K127_5).to_i32();
    let c4 = (f(c, p + 0x10) * K127_5).to_i32();
    let c5 = (f(c, p + 0x14) * K127_5).to_i32();
    let half = |v: i32| (v.wrapping_add((v as u32 >> 31) as i32)) >> 1;
    let fp = half(0x1000i32.wrapping_sub(w)).wrapping_add(x0) << 4;
    let s7 = half(0x1000i32.wrapping_sub(h)).wrapping_add(y0) << 4;
    let n = w >> 5;
    let flag = (c3 > 0 || c4 > 0 || c5 > 0) as i32;
    let s0 = y0.wrapping_add(h);
    let mut s: Vec<u128> = vec![0];
    let nloop = sx64((n << 2).wrapping_add(0xa)).wrapping_add(sx64(((n << 1).wrapping_add(3)).wrapping_mul(flag)));
    s.push(qw(nloop | NREG1, 0xe));
    let v = vget(c, gm, 0x54, 0x0039_077c);
    s.push(qw(fbp as u64 | ((v >> 6) as u64) << 16, 0x4c));
    s.push(qw((w80 | 0x100_0000) as u64 | 0x1_0000_0000, 0x4e));
    s.push(qw(0x30000, 0x47));
    s.push(qw(sx64(fp) | sx64(s7) << 32, 0x18));
    let sc = sx64(x0) | sx64(x0.wrapping_add(w).wrapping_sub(1)) << 16 | sx64(y0) << 32 | sx64(s0.wrapping_sub(1)) << 48;
    s.push(qw(sc, 0x40));
    s.push(qw(5, 8));
    s.push(qw(0, 0x14));
    let v = vget(c, gm, 0x54, 0x0039_085c);
    s.push(qw((fbp << 5) as u64 | ((v >> 6) as u64) << 14 | 0x6_a800_0000, 6));
    s.push(qw(0x116, 0));
    // (Generated: f2 = p[1] * k, f3 = p[2] * k, f0 = p[0] * k.)
    let g = sx64((f(c, p + 4) * K127_5).to_i32());
    let b = sx64((f(c, p + 8) * K127_5).to_i32());
    let r = sx64((f(c, p) * K127_5).to_i32());
    s.push(qw(r | g << 8 | b << 16 | 0x3f80_0000_8000_0000, 1));
    let ff = 0xffffu64 << 32;
    let xs = x0 << 4;
    if n != 0 {
        let t7 = sx64((y0 << 4).wrapping_add(8)) << 16;
        let s4 = sx64(s7.wrapping_add(y0 << 4)) << 16;
        let s1 = sx64(s7.wrapping_add(s0 << 4)) << 16;
        let t6 = sx64((s0 << 4).wrapping_add(8)) << 16;
        let (mut t1, mut a2) = (xs.wrapping_add(8), xs.wrapping_add(fp));
        let (mut a3, mut t2) = (xs.wrapping_add(0x208), xs.wrapping_add(0x200).wrapping_add(fp));
        for _ in 0..n {
            s.push(qw(sx64(t1) | t7, 3));
            s.push(qw(sx64(a2) | s4 | ff, 5));
            s.push(qw(sx64(a3) | t6, 3));
            s.push(qw(sx64(t2) | s1 | ff, 5));
            a3 = a3.wrapping_add(0x200);
            t1 = t1.wrapping_add(0x200);
            t2 = t2.wrapping_add(0x200);
            a2 = a2.wrapping_add(0x200);
        }
    }
    if flag != 0 {
        s.push(qw(0x48, 0x42));
        s.push(qw(0x46, 0));
        s.push(qw(sx64(c3) | sx64(c4) << 8 | sx64(c5) << 16 | 0x3f80_0000_8000_0000, 1));
        if n != 0 {
            let t0 = sx64(s7.wrapping_add(s0 << 4)) << 16;
            let s0q = sx64(s7.wrapping_add(y0 << 4)) << 16;
            let (mut a0, mut a1) = (xs.wrapping_add(fp), xs.wrapping_add(0x200).wrapping_add(fp));
            for _ in 0..n {
                s.push(qw(sx64(a0) | s0q | ff, 5));
                s.push(qw(sx64(a1) | t0 | ff, 5));
                a1 = a1.wrapping_add(0x200);
                a0 = a0.wrapping_add(0x200);
            }
        }
    }
    s[0] = qw(sx64(s.len() as i32 - 1) | CNT, 0);
    s.push(qw(CNT | 4, 0));
    s.push(qw(0x1000_0000_0000_8003, 0xe));
    let v = vget(c, gm, 0x54, 0x0039_0b20);
    s.push(qw(fbp as u64 | ((v >> 6) as u64) << 16, 0x4c));
    s.push(xyoffset(c, gm, 0x0039_0b58, 0x0039_0b78));
    let (sx, sy) = (c.bus.read16(gm + 0x5900) as i32, c.bus.read16(gm + 0x5902) as i32);
    let (sw, sh) = (c.bus.read16(gm + 0x5904) as i32, c.bus.read16(gm + 0x5906) as i32);
    let sc = sx64(sx) | sx64(sx.wrapping_sub(1).wrapping_add(sw)) << 16 | (sy as u64) << 32 | sx64(sy.wrapping_sub(1).wrapping_add(sh)) << 48;
    s.push(qw(sc, 0x40));
    s
}

/// XYOFFSET_1 back to the frame's centre (`((0x1000 - size) >> 1) << 4`
/// from the width and height getters), as an A+D qword.
fn xyoffset(c: &mut Cpu, gm: u32, ret_w: u32, ret_h: u32) -> u128 {
    let vw = vget(c, gm, 0x54, ret_w);
    let x = (0x1000u32.wrapping_sub(vw) >> 1) << 4;
    let vh = vget(c, gm, 0x5c, ret_h);
    let y = (0x1000u32.wrapping_sub(vh) >> 1) << 4;
    qw(sx64(x as i32) | (y as u64) << 32, 0x18)
}

/// `PS2GraphicsMan_RenderColorFilter(gm, view, &vifCursor, &gifCursor)`
/// (0x3904a0, `PostKind::ColorFilter`): when enabled (`gp+0x14e8`;
/// `gp+0x14ec` resets the view's filter first) and the filter is not the
/// identity (scale 1, bias 0), the filter's GIF transfers and one PATH3
/// window.
pub fn render_color_filter(c: &mut Cpu) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, view, vif) = (g(c, A0), g(c, A1), g(c, A2));
    let gp = g(c, GP);
    let vifv = r32(c, vif);
    let (mut gif, mut windows) = (Vec::new(), 0);
    if r32(c, gp + 0x14e8) != 0 {
        if r32(c, gp + 0x14ec) != 0 {
            reset_color_filter(c, gm, view, gp);
        }
        let p = gm.wrapping_add(view.wrapping_mul(0x18)).wrapping_add(0x6ca4);
        let identity = f(c, p).eq_s(FpuF32::ONE)
            && f(c, p + 4).eq_s(FpuF32::ONE)
            && f(c, p + 8).eq_s(FpuF32::ONE)
            && f(c, p + 0xc).eq_s(FpuF32::ZERO)
            && f(c, p + 0x10).eq_s(FpuF32::ZERO)
            && f(c, p + 0x14).eq_s(FpuF32::ZERO);
        if !identity {
            let s = color_filter_stream(c, gm, view);
            gif = gif_data(&s);
            windows = 1;
        }
    }
    let n = gif.len() as u64;
    record_post(vifv, PostKind::ColorFilter, gif, windows);
    ret(c, 60 + 8 * n);
}

// ---------------------------------------------------------------------------
// Bloom.

/// `PS2GraphicsMan_ResetBloomParams(gm, view)` (0x36c740): the view's seven
/// bloom floats (`gm + view*0x1c + 0x6cd4`) from the defaults at
/// `gp+0x12e4..0x12f4`, `0x1300`, `0x1304`.
fn reset_bloom_params(c: &mut Cpu, gm: u32, view: u32, gp: u32) {
    let p = gm.wrapping_add(view.wrapping_mul(0x1c)).wrapping_add(0x6cd4);
    let src = [0x12e4, 0x12e8, 0x12ec, 0x12f0, 0x12f4, 0x1300, 0x1304];
    for (k, &o) in src.iter().enumerate() {
        let v = r32(c, gp + o);
        w32(c, p + 4 * k as u32, v);
    }
}

/// `PS2GraphicsMan_EmitBlurDownsample(f12 scale, gm, src, srcW, x, y, w, h,
/// dst, dw, dh, offset, passes, &gif)` (0x36b9d8) as its transfers: one
/// CNT (setup into `dst`, `w`×`h` of `src` at (`x`,`y`) drawn as 32-pixel
/// strips into `dw`×`dh`, `passes` (≤ 4) times with the texel offset ±
/// `offset`, at `scale / passes`). `p` = the integer arguments a1..t3 and
/// the stack words (`p[0]` = src … `p[10]` = passes), as the generated body
/// reads them.
fn blur_downsample(c: &mut Cpu, s: &mut Vec<u128>, gm: u32, scale: FpuF32, p: [i32; 11]) {
    let [p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13] = p;
    let w80 = r32(c, gm + 0x5a80) as i32;
    let i2 = (p5 << 4).wrapping_add(p12.wrapping_add(8));
    let n = div32(p10);
    let i8 = (p5.wrapping_add(p7) << 4).wrapping_sub(p12.wrapping_sub(8)).wrapping_sub(i2);
    let i3 = (p6 << 4).wrapping_add(p12.wrapping_add(8));
    let i9 = (p6.wrapping_add(p8) << 4).wrapping_sub(p12.wrapping_sub(8));
    let fp = p10 << 4;
    // (Generated: f20 = f12 / (float)passes; cvt.w.s.)
    let u = sx64(scale.div(FpuF32::from_i32(p13)).to_i32());
    let t2 = n.wrapping_mul(p13 << 2);
    s.push(qw(sx64(t2.wrapping_add(0xe)) | CNT, 0));
    s.push(qw(sx64(t2.wrapping_add(0xd)) | NREG1, 0xe));
    s.push(qw(sx64(p9 >> 5) | sx64(units64(p10)) << 16 | 0xff00_0000_0100_0000, 0x4c));
    s.push(qw(sx64(w80) | 0x100_0000 | 0x1_0000_0000, 0x4e));
    s.push(qw(0x30000, 0x47));
    s.push(qw(0, 0x18));
    s.push(qw(0x80_0000_0068, 0x42));
    s.push(qw(0, 0x3f));
    s.push(qw(sx64(p10.wrapping_sub(1)) << 16 | sx64(p11.wrapping_sub(1)) << 48, 0x40));
    s.push(qw(0x60, 0x14));
    s.push(qw(sx64(p3) | sx64(units64(p4)) << 14 | 0x6_a810_0000, 6));
    let xyuv = sx64(p5) << 4
        | sx64(p5.wrapping_add(p7).wrapping_sub(1)) << 14
        | 0xa
        | sx64(p6) << 24
        | sx64(p6.wrapping_add(p8).wrapping_sub(1)) << 34;
    s.push(qw(xyuv, 8));
    s.push(qw(0x116, 0));
    s.push(qw(u | u << 8 | u << 16 | 0x3f80_0000_8000_0000, 1));
    // One pass: the source strip's U offset by `xs * offset`, V by
    // `ys * offset`.
    let pass = |s: &mut Vec<u128>, xs: i32, ys: i32| {
        if n <= 0 {
            return;
        }
        let ox = if xs > 0 { p12 } else { p12.wrapping_neg() };
        let oy = if ys > 0 { p12 } else { p12.wrapping_neg() };
        let s0 = sx64(p11 << 4) << 16;
        let ya = sx64(i3.wrapping_add(oy)) << 16;
        let yb = sx64(i9.wrapping_add(oy)) << 16;
        let t6 = i8 << 9;
        let (mut t0, mut a3, mut t2) = (0x200i32, 0i32, 0i32);
        for _ in 0..n {
            let a2 = t0.wrapping_add(0x200);
            let lo1 = idiv(t2, fp);
            let lo2 = idiv(t0.wrapping_mul(i8), fp);
            s.push(qw(sx64(i2.wrapping_add(lo1).wrapping_add(ox)) | ya, 3));
            s.push(qw(sx64(a3), 5));
            s.push(qw(sx64(i2.wrapping_add(lo2).wrapping_add(ox)) | yb, 3));
            s.push(qw(sx64(t0) | s0, 5));
            t0 = if a2 < fp { a2 } else { fp };
            t2 = t2.wrapping_add(t6);
            a3 = a3.wrapping_add(0x200);
        }
    };
    pass(&mut *s, -1, -1);
    s.push(qw(0x156, 0));
    if p13 >= 2 {
        pass(&mut *s, 1, 1);
        if p13 >= 3 {
            pass(&mut *s, 1, -1);
            if p13 >= 4 {
                pass(&mut *s, -1, 1);
            }
        }
    }
}

/// `PS2GraphicsMan_EmitBlendedFill(gm, buf, w, h, alpha, fog, &gif)`
/// (0x36c188) as its transfer: a `w`×`h` fill of `buf` in 32-pixel strips.
fn blended_fill(s: &mut Vec<u128>, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) {
    let m = div32(p3);
    let step = idiv(p3, m) << 4;
    s.push(qw(sx64((m << 1).wrapping_add(7)) | CNT, 0));
    s.push(qw(sx64((m << 1).wrapping_add(6)) | NREG1, 0xe));
    s.push(qw(sx64(p2 >> 5) | sx64(units64(p3)) << 16 | 0xff00_0000_0100_0000, 0x4c));
    s.push(qw(0, 0x18));
    s.push(qw(sx64(p6) << 32 | 0xa1, 0x42));
    s.push(qw(0, 0x3f));
    s.push(qw(0x146, 0));
    let a = sx64(p5);
    s.push(qw(a | a << 8 | a << 16 | 0x3f80_0000_8000_0000, 1));
    if m > 0 {
        let a0 = sx64(p4 << 4) << 16;
        let (mut a2, mut a1) = (0i32, step);
        for _ in 0..m {
            s.push(qw(sx64(a2), 5));
            s.push(qw(sx64(a1) | a0, 5));
            a2 = a2.wrapping_add(step);
            a1 = a1.wrapping_add(step);
        }
    }
}

/// `PS2GraphicsMan_EmitBloomComposite(gm, src, srcW, srcH, fbp32, x, y, w,
/// h, alpha, fix, &gif)` (0x36c398) as its transfer: `src` stretched over
/// the view's rectangle of the frame at `fbp32`, additive with `alpha`.
fn bloom_composite(c: &mut Cpu, s: &mut Vec<u128>, gm: u32, p: [i32; 10]) {
    let [p2, p3, p4, p5, p6, p7, p8, p9, p10, p11] = p;
    let m = div32(p8);
    let step = idiv(p3, m) << 4;
    let w80 = r32(c, gm + 0x5a80) as i32;
    s.push(qw(sx64((m << 2).wrapping_add(0xd)) | CNT, 0));
    s.push(qw(sx64((m << 2).wrapping_add(0xc)) | NREG1, 0xe));
    let mut frame = sx64(p5 >> 5);
    if vget(c, gm, 0x54, 0x0036_c48c) >> 6 == 0 {
        frame |= 0xff00_0000_0101_0000;
    } else {
        let v = vget(c, gm, 0x54, 0x0036_c4ac);
        frame |= ((v >> 6) as u64) << 16 | 0xff00_0000_0100_0000;
    }
    s.push(qw(frame, 0x4c));
    s.push(qw(sx64(w80) | 0x100_0000 | 0x1_0000_0000, 0x4e));
    s.push(qw(0x30000, 0x47));
    s.push(qw(0, 0x18));
    s.push(qw(sx64(p11) << 32 | 0x29, 0x42));
    s.push(qw(0, 0x3f));
    let y1 = p7.wrapping_add(p9);
    let sc = sx64(p6) | sx64(p6.wrapping_add(p8).wrapping_sub(1)) << 16 | sx64(p7) << 32 | sx64(y1.wrapping_sub(1)) << 48;
    s.push(qw(sc, 0x40));
    s.push(qw(0x60, 0x14));
    s.push(qw(sx64(p2) | sx64(units64(p3)) << 14 | 0x6_a810_0000, 6));
    s.push(qw(sx64(p3.wrapping_sub(1)) << 14 | sx64(p4.wrapping_sub(1)) << 34 | 0xa, 8));
    s.push(qw(0x156, 0));
    let a = sx64(p10);
    s.push(qw(a | a << 8 | a << 16 | a << 24 | 0x3f80_0000_0000_0000, 1));
    if m > 0 {
        let t2 = sx64(p7 << 4) << 16;
        let a3 = sx64(p4 << 4) << 16;
        let t3 = sx64(y1 << 4) << 16;
        let (mut t9, mut s7, mut t8) = (p6 << 4, 8i32, step.wrapping_add(8));
        for _ in 0..m {
            let v1 = t9.wrapping_add(0x200);
            s.push(qw(sx64(s7), 3));
            s.push(qw(sx64(t9) | t2, 5));
            s.push(qw(sx64(t8) | a3, 3));
            s.push(qw(sx64(v1) | t3, 5));
            t9 = v1;
            t8 = t8.wrapping_add(step);
            s7 = s7.wrapping_add(step);
        }
    }
}

/// `PS2GraphicsMan_RenderBloom(gm, view, &vifCursor, &gifCursor)`
/// (0x36c790, `PostKind::Bloom`): when enabled (`gp+0x12d8`; `gp+0x12dc`
/// resets the view's parameters first) and any of the four composite
/// alphas is not zero: the frame downsampled into the bloom buffers, a
/// blended fill, three blur levels (each composited when its alpha is
/// set), the last level's composite, FRAME / XYOFFSET back; one PATH3
/// window.
pub fn render_bloom(c: &mut Cpu) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, view, vif) = (g(c, A0), g(c, A1), g(c, A2));
    let gp = g(c, GP);
    let vifv = r32(c, vif);
    if r32(c, gp + 0x12d8) == 0 {
        record_post(vifv, PostKind::Bloom, Vec::new(), 0);
        return ret(c, 20);
    }
    if r32(c, gp + 0x12dc) != 0 {
        reset_bloom_params(c, gm, view, gp);
    }
    let p = gm.wrapping_add(view.wrapping_mul(0x1c)).wrapping_add(0x6cd4);
    let k = K127_5;
    // The composite alphas: two globals, the view's p[5], p[6] (each × 127.5).
    let alpha = [
        (f(c, gp + 0x12f8) * k).to_i32(),
        (f(c, gp + 0x12fc) * k).to_i32(),
        (f(c, p + 0x14) * k).to_i32(),
        (f(c, p + 0x18) * k).to_i32(),
    ];
    if alpha.iter().all(|&a| a == 0) {
        record_post(vifv, PostKind::Bloom, Vec::new(), 0);
        return ret(c, 40);
    }
    let buf0 = (r32(c, gm + 0x5a80) << 5) as i32;
    let bufs = [buf0, buf0.wrapping_add(0x400)];
    let offset = (f(c, gp + 0x1308) * FpuF32(0x4180_0000)).to_i32();
    let mut s1 = 1i32 << (r32(c, gp + 0x12e0) & 31);
    let mut s4 = s1;
    let vo = gm.wrapping_add(view << 3);
    let x0 = c.bus.read16(vo + 0x59b0) as i32;
    let y0 = c.bus.read16(vo + 0x59b2) as i32;
    let w = c.bus.read16(vo + 0x59b4) as i32;
    let fbp = r32(c, gm + 0x5a84) as i32;
    let h = c.bus.read16(vo + 0x59b6) as i32;
    let width = vget(c, gm, 0x54, 0x0036_c930) as i32;
    let fbp32 = fbp << 5;
    let mut s: Vec<u128> = Vec::new();
    let scale = f(c, p + 0xc) * k;
    blur_downsample(c, &mut s, gm, scale, [fbp32, width, x0, y0, w, h, buf0, s1, s1, 0, 1]);
    let fill_a = (f(c, p) * k).to_i32();
    let fill_f = (f(c, gp + 0x12e8) * k).to_i32();
    blended_fill(&mut s, buf0, s1, s1, fill_a, fill_f);
    let mut src = buf0;
    for lvl in 0..3usize {
        if alpha[lvl] != 0 {
            let fix = (f(c, p + 0x10) * k).to_i32();
            bloom_composite(c, &mut s, gm, [src, s4, s1, fbp32, x0, y0, w, h, alpha[lvl], fix]);
        }
        let dst = bufs[(lvl + 1) & 1];
        let scale = f(c, p + 8) * k;
        blur_downsample(c, &mut s, gm, scale, [src, s4, 0, 0, s4, s1, dst, s4 >> 1, s1 >> 1, offset, 4]);
        src = dst;
        s4 >>= 1;
        s1 >>= 1;
    }
    if alpha[3] != 0 {
        let fix = (f(c, p + 0x10) * k).to_i32();
        bloom_composite(c, &mut s, gm, [src, s4, s1, fbp32, x0, y0, w, h, alpha[3], fix]);
    }
    s.push(qw(CNT | 3, 0));
    s.push(qw(0x1000_0000_0000_8002, 0xe));
    let xy = xyoffset(c, gm, 0x0036_cb04, 0x0036_cb24);
    let v = vget(c, gm, 0x54, 0x0036_cb6c);
    s.push(qw(sx64(fbp) | ((v >> 6) as u64) << 16, 0x4c));
    s.push(xy);
    let gif = gif_data(&s);
    let n = gif.len() as u64;
    record_post(vifv, PostKind::Bloom, gif, 1);
    ret(c, 400 + 8 * n);
}
