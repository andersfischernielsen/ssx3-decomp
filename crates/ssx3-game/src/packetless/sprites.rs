//! Sprites (slots 78 / 79) without packets: the EE projects them and sends
//! GIF A+D-free REGLIST blocks with `DIRECT`; here those blocks are the
//! recorded draw (`Step::Gif`, as `crate::capture` reads the packet).

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::vu0::{self, V};
use super::*;
use crate::ps2f::FpuF32;

/// A 32-bit value as the generated code holds it in a 64-bit register
/// (`lw` / `mfc1` sign-extend).
fn sx64(x: u32) -> u64 {
    x as i32 as i64 as u64
}

/// Two words loaded with `lw` and joined `lo | hi << 32` (the low word's
/// sign extension stays in the upper half).
fn pair(lo: u32, hi: u32) -> u64 {
    sx64(lo) | (hi as u64) << 32
}

fn qw(lo: u64, hi: u64) -> u128 {
    lo as u128 | (hi as u128) << 64
}

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// The colour word both sprite draws build from four floats `s` (times
/// 128): `s1 | s2 << 8 | s3 << 16 | s0 << 24 | 1.0 << 32` (Q = 1), as the
/// generated code's 64-bit shifts of the sign-extended integers.
fn rgbaq(s: [FpuF32; 4]) -> u64 {
    let k = FpuF32(0x4300_0000);
    let i = |v: FpuF32| sx64((v * k).to_i32() as u32);
    i(s[1]) | i(s[2]) << 8 | i(s[3]) << 16 | i(s[0]) << 24 | (vu0::ONE as u64) << 32
}

/// XYZ2 from integers: `x | y << 16 | z << 32` (sign-extended operands).
fn xyz(x: i32, y: i32, z: i32) -> u64 {
    sx64(x as u32) | sx64(y as u32) << 16 | sx64(z as u32) << 32
}

/// The clip code of a corner against the guard box at `gm+0x6b70..`
/// (x: 0x6b70 / 0x6b80, y: 0x6b74 / 0x6b84, z: 0x6b78 / 0x6b88), `^ 0x2f`
/// behind the eye (w < 0).
fn clip_code(c: &mut Cpu, gm: u32, v: &V) -> u32 {
    let [x, y, z, w] = v.map(FpuF32);
    let (x0, x1) = (f(c, gm + 0x6b70), f(c, gm + 0x6b80));
    let (y0, y1) = (f(c, gm + 0x6b74), f(c, gm + 0x6b84));
    let mut k = 0;
    if x.lt(x0) {
        k = 2;
    }
    if x1.lt(x) {
        k |= 1;
    }
    if y.lt(y0) {
        k |= 8;
    }
    if y1.lt(y) {
        k |= 4;
    }
    if w.lt(FpuF32::ZERO) {
        return k ^ 0x2f;
    }
    if f(c, gm + 0x6b78).lt(z) {
        k |= 0x20;
    }
    if z.lt(f(c, gm + 0x6b88)) {
        k |= 0x10;
    }
    k
}

/// `PS2GraphicsMan_DrawSprite3D(gm, pos, size, st0, st1, rgba)` (slot 78,
/// 0x377cf0): `pos` through the matrix at `gm+0x6b30`, divided by w; the
/// sprite's corners that ± `size` scaled by `gm+0x6af0` / `+0x6b04`;
/// skipped when both corners are outside the same guard plane; else one
/// SPRITE (PRIM 0x5e: ST, RGBAQ, XYZ2 twice) in one record. The render
/// state stays as it is.
pub fn draw_sprite_3d(c: &mut Cpu) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, pos, size, st0) = (g(c, A0), g(c, A1), g(c, A2), g(c, A3));
    let (st1, col) = (g(c, T0), g(c, T1));
    ensure_view_matrices(c, gm, 0x0037_7d38);
    let m = qm(c, gm + 0x6b30);
    let p = vu0::xform(&m, &qv(c, pos));
    // (FPU, generated order: inv = 1 / w; sx = gm[0x6af0] * size.x * inv;
    // sy = gm[0x6b04] * size.y * inv; x, y times inv.)
    let inv = FpuF32::ONE.div(FpuF32(p[3]));
    let sx = f(c, gm + 0x6af0) * f(c, size);
    let sy = f(c, gm + 0x6b04) * f(c, size + 4);
    let y = FpuF32(p[1]) * inv;
    let sy = sy * inv;
    let x = FpuF32(p[0]) * inv;
    let sx = sx * inv;
    let a: V = [x.0, y.0, p[2], inv.0];
    let b: V = [sx.0, sy.0, 0, 0];
    let lo = vu0::sub(&a, &b);
    let hi = vu0::add(&a, &b);
    if clip_code(c, gm, &lo) & clip_code(c, gm, &hi) != 0 {
        return ret(c, 120);
    }
    let colour = rgbaq(std::array::from_fn(|k| f(c, col + 4 * k as u32)));
    let corner = |v: &V| {
        let z = (FpuF32(v[2]) * FpuF32(v[3])).to_i32();
        xyz(FpuF32(v[0]).to_i32(), FpuF32(v[1]).to_i32(), z)
    };
    let s0 = pair(r32(c, st0), r32(c, st0 + 4));
    let s1 = pair(r32(c, st1), r32(c, st1 + 4));
    let gif = vec![
        qw(0x6400_0000_0000_8001, 0x52_5120),
        qw(0x5e, s0),
        qw(colour, corner(&lo)),
        qw(s1, corner(&hi)),
    ];
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(vec![Step::Gif(gif)]));
    add_record(c, gm, start, start, false);
    ret(c, 260);
}

/// `PS2GraphicsMan_DrawRotatedSprites(f12 angle, gm, pos, scale, st0, st1,
/// rgba, count)` (slot 79, 0x3781a0): `pos` and its two offsets along the
/// view matrix's (`gm+0x13e4`) axes rotated by `angle` through the
/// graphics manager's projection (virtual method 0x2e4); a quad
/// (PRIM 0x54: RGBAQ, ST and XYZ2 four times) about the projected centre,
/// at the screen offset of `gm+0x5a30` / `+0x5a34`, sent `count` times;
/// one record. The render state stays as it is.
pub fn draw_rotated_sprites(c: &mut Cpu) {
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, pos, scale, st0) = (g(c, A0), g(c, A1), g(c, A2), g(c, A3));
    let (st1, col) = (g(c, T0), g(c, T1));
    let count = c.g[T2][0] as i64;
    let angle = c.f[12];
    ensure_view_matrices(c, gm, 0x0037_81fc);
    // The generated frame: the callees write into it.
    let sp0 = c.g[SP][0];
    let sp = (sp0 as u32).wrapping_sub(0x170);
    c.g[SP][0] = sx64(sp);
    let (this, func) = model::vmethod(c, gm, 0x2e4);
    guest_call(c, func, &[sp, this, pos], 0x0037_8220);
    c.f[12] = angle;
    guest_call(c, 0x0031_be50, &[sp + 0xd0, sp + 0xd4], 0x0037_8230);
    let (a, b) = (f(c, sp + 0xd0), f(c, sp + 0xd4));
    let na = -a;
    let mt = r32(c, gm + 0x13e4);
    let mut r0 = [0u32; 4];
    let mut r1 = [0u32; 4];
    for i in 0..4u32 {
        let (m0, m1) = (f(c, mt + 0x10 * i), f(c, mt + 0x10 * i + 4));
        r0[i as usize] = (b * m0 + a * m1).0;
        r1[i as usize] = (na * m0 + b * m1).0;
    }
    let p = qv(c, pos);
    let mut p1 = vu0::add(&p, &vu0::mul(&r0, &[r32(c, scale); 4]));
    p1[3] = vu0::ONE;
    let mut p2 = vu0::add(&p, &vu0::mul(&r1, &[r32(c, scale + 4); 4]));
    p2[3] = vu0::ONE;
    wv(c, sp + 0x30, &p1);
    wv(c, sp + 0x40, &p2);
    let (this, func) = model::vmethod(c, gm, 0x2e4);
    guest_call(c, func, &[sp + 0x60, this, sp + 0x30], 0x0037_83b8);
    let v = qv(c, sp);
    let d1 = vu0::sub(&qv(c, sp + 0x60), &v);
    let (this, func) = model::vmethod(c, gm, 0x2e4);
    guest_call(c, func, &[sp + 0x70, this, sp + 0x40], 0x0037_83ec);
    let d2 = vu0::sub(&qv(c, sp + 0x70), &v);
    c.g[SP][0] = sp0;
    let corners = [
        vu0::sub(&vu0::sub(&v, &d1), &d2),
        vu0::sub(&vu0::add(&v, &d1), &d2),
        vu0::add(&vu0::sub(&v, &d1), &d2),
        vu0::add(&vu0::add(&v, &d1), &d2),
    ];
    let ox = (0x800u32.wrapping_sub(r32(c, gm + 0x5a30) >> 1) << 4) as i32;
    let oy = (0x800u32.wrapping_sub(r32(c, gm + 0x5a34) >> 1) << 4) as i32;
    let z = FpuF32(v[2]).to_i32();
    let sixteen = FpuF32(0x4180_0000);
    let xy: Vec<u64> = corners
        .iter()
        .map(|k| {
            let x = ox.wrapping_add((FpuF32(k[0]) * sixteen).to_i32());
            let y = oy.wrapping_add((FpuF32(k[1]) * sixteen).to_i32());
            xyz(x, y, z)
        })
        .collect();
    let colour = rgbaq(std::array::from_fn(|k| f(c, col + 4 * k as u32)));
    let mut steps = Vec::new();
    if count > 0 {
        let (s0, t0) = (r32(c, st0), r32(c, st0 + 4));
        let (s1, t1) = (r32(c, st1), r32(c, st1 + 4));
        let gif = vec![
            qw(0xa400_0000_0000_8001, 0x52_5252_5210),
            qw(0x54, colour),
            qw(pair(s0, t0), xy[0]),
            qw(pair(s1, t0), xy[1]),
            qw(pair(s0, t1), xy[2]),
            qw(pair(s1, t1), xy[3]),
        ];
        steps = (0..count).map(|_| Step::Gif(gif.clone())).collect();
    }
    let n = steps.len() as u64;
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, false);
    ret(c, 600 + 35 * n);
}
