//! Fog particles (`cPS2FogParticleMan_DrawParticles`) and the transition
//! screen copy (slot 61 `DrawTransition`) without packets: both build GIF
//! data in a SprStream block and send it `DIRECT`; here that data is the
//! recorded draw (`Step::Gif`, as `crate::capture` reads the packet). The
//! SprStream (`gp+0x1384`) is left alone.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::*;
use crate::ps2f::FpuF32;

/// A word as a register holds it after `lw` / `mfc1` (sign-extended).
fn sx64(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// Two float words joined `lo | hi << 32` (the low word's sign extension
/// stays in the upper half, as the generated code ORs them).
fn pair(lo: FpuF32, hi: FpuF32) -> u64 {
    sx64(lo.0) | (hi.0 as u64) << 32
}

/// XYZ2 from integers: `x | y << 16 | z << 32` (sign-extended operands).
fn xyz(x: i32, y: i32, z: i32) -> u64 {
    sx64(x as u32) | sx64(y as u32) << 16 | sx64(z as u32) << 32
}

fn qw(lo: u64, hi: u64) -> u128 {
    lo as u128 | (hi as u128) << 64
}

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// The fog particles' order: 8-byte entries, the particle index first.
const FOG_ORDER: u32 = 0x0053_7ac0;

/// `cPS2FogParticleMan_DrawParticles(this)` (0x2dc7b0, a virtual method of
/// the fog particle manager; the graphics manager is `gp+0x2a90`):
/// `*this` particles (0x30 bytes each from `this+0x10`, in the order of
/// the table at 0x537ac0) as sprites, in batches of what fits a SprStream
/// block (`(block qwords - 4) * 2 / 5 - 1`); per batch one record of GIF
/// data {PRIM 0x56; per particle ST, RGBAQ, XYZ2, ST, XYZ2 (REGLIST)}.
/// The render state stays as it is.
pub fn fog_particles(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    let gp = c.g[GP][0] as u32;
    let spr = r32(c, gp + 0x1384);
    let mut left = r32(c, this) as i32;
    let gm = r32(c, gp + 0x2a90);
    let size = ((r32(c, spr + 0x10) << 4) as i32) >> 4;
    let per = (size.wrapping_sub(4).wrapping_shl(1) / 5).wrapping_sub(1);
    let mut first = 0i32;
    let mut cycles = 30u64;
    // (A batch size below 1 would never finish in the generated body
    // either; not drawn.)
    while left > 0 && per > 0 {
        let n = if left < per { left } else { per };
        let end = first + n;
        left -= n;
        let mut d: Vec<u64> = Vec::with_capacity(5 * n as usize + 1);
        for k in first..end {
            let idx = r32(c, FOG_ORDER + 8 * k as u32);
            let p = idx.wrapping_mul(0x30).wrapping_add(0x10).wrapping_add(this);
            let w = f(c, p + 0xc);
            // (Generated order: ST {0, 0} times w; colour; first corner
            // with z * w; ST {1, 1} times w; second corner.)
            d.push(pair(FpuF32::ZERO * w, FpuF32::ZERO * w));
            let a = f(c, p + 0x20).to_i32() as u32;
            let r = f(c, p + 0x24).to_i32() as u32;
            let g = f(c, p + 0x28).to_i32() as u32;
            let b = f(c, p + 0x2c).to_i32() as u32;
            d.push(sx64(r) | sx64(g) << 8 | sx64(b) << 16 | sx64(a) << 24 | (w.0 as u64) << 32);
            let z = (f(c, p + 8) * w).to_i32();
            d.push(xyz(f(c, p).to_i32(), f(c, p + 4).to_i32(), z));
            d.push(pair(FpuF32::ONE * w, FpuF32::ONE * w));
            let z = (f(c, p + 0x18) * w).to_i32();
            d.push(xyz(f(c, p + 0x10).to_i32(), f(c, p + 0x14).to_i32(), z));
        }
        if d.len() % 2 == 1 {
            d.push(0);
        }
        let mut gif = vec![
            qw(0x1400_0000_0000_8001, 0),
            qw(0x56, 0),
            qw(n as u32 as u64 | 0x8000 | 0x5400_0000_0000_0000, 0x5_2512),
        ];
        gif.extend(d.chunks_exact(2).map(|h| qw(h[0], h[1])));
        let start = empty_packet(c, gm);
        record(start, Cmd::Vu(vec![Step::Gif(gif)]));
        add_record(c, gm, start, start, false);
        first = end;
        cycles += 60 + 82 * n as u64;
    }
    ret(c, cycles);
}

/// `PS2GraphicsMan_DrawTransition(gm, tex)` (slot 61, 0x37cc98): in a
/// pushed state (type and w3 cleared), one record of GIF A+D data that
/// copies the frame buffer into the transition texture (`tex+4` indexes
/// the texture table at `gm+0x18f4`; its VRAM address at `+0x28`): FRAME,
/// ZBUF, SCISSOR, TEST, XYOFFSET, ALPHA, TEXA, TEX1, TEX0 (the frame
/// buffer at `gm+0x5a88` / `+0x5a5c`), a SPRITE over `gm+0x5a54` x
/// `+0x5a58`, TEXFLUSH, the scissor back from `gm+0x5900..`. Sets
/// `gm+0xf44`.
pub fn draw_transition(c: &mut Cpu) {
    let (gm, tex) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    push_state(c, gm);
    clear_type(c, gm);
    let i = r32(c, tex + 4);
    let table = r32(c, gm + 0x18f4);
    let t = r32(c, table.wrapping_add(i << 2) + 8);
    let vram = r32(c, t + 0x28) as i32;
    let fbp = (if vram > -1 {
        vram
    } else {
        vram.wrapping_add(0x1f)
    }) >> 5;
    let frame = sx64(fbp as u32) | 0x40000;
    let zbuf = (r32(c, gm + 0x5a80) | 0x0100_0000) as u64 | 0x1_0000_0000;
    let tbp = sx64(r32(c, gm + 0x5a88) << 5);
    let tbw = sx64(r32(c, gm + 0x5a5c)) << 14;
    let tex0 = tbp | tbw | 0x2_a800_0000;
    let w = sx64((r32(c, gm + 0x5a54) << 4).wrapping_sub(8));
    let h = sx64((r32(c, gm + 0x5a58) << 4).wrapping_sub(8));
    let uv1 = w | h << 16;
    let h16 = |c: &mut Cpu, a: u32| c.bus.read16(gm + a) as u32;
    let (x0, y0, xw, yh) = (
        h16(c, 0x5900),
        h16(c, 0x5902),
        h16(c, 0x5904),
        h16(c, 0x5906),
    );
    let x1 = sx64(x0.wrapping_sub(1).wrapping_add(xw));
    let y1 = sx64(y0.wrapping_sub(1).wrapping_add(yh));
    let scissor = x0 as u64 | x1 << 16 | (y0 as u64) << 32 | y1 << 48;
    let gif = vec![
        qw(0x1000_0000_0000_8011, 0xe),
        qw(frame, 0x4c),
        qw(zbuf, 0x4e),
        qw(0x00ff_0000_00ff_0000, 0x40),
        qw(0x30000, 0x47),
        qw(0, 0x18),
        qw(0x2a, 0x42),
        qw(0x80_0000_0080, 0x3b),
        qw(0, 0x14),
        qw(tex0, 6),
        qw(0x116, 0),
        qw(0x3f80_0000_8080_8080, 1),
        qw(0x80008, 3),
        qw(0xffff_0000_0000, 5),
        qw(uv1, 3),
        qw(0xffff_1000_1000, 5),
        qw(0, 0x3f),
        qw(scissor, 0x40),
    ];
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(vec![Step::Gif(gif)]));
    add_record(c, gm, start, start, false);
    w32(c, gm + 0xf44, 1);
    pop_state(c, gm);
    ret(c, 260);
}
