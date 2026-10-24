//! Render-target switches without packets: slot 53
//! `SetRenderTargetTexture`, slot 54 `RestoreRenderTarget`, slot 55
//! `FillRenderTarget` (used around the rider shadow texture and at
//! EndFrame). Each is a GIF A+D (or REGLIST) block, as the packet builders
//! (`TexMan_BuildUploadPacket` / `BuildSpritePacket`, ported in
//! `crate::render`) write it.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::*;
use crate::ps2f::FpuF32;

/// A TexMan texture's TexInfo (`tm+8+4·id`) and its TEX0.
fn tex_info(c: &mut Cpu, gm: u32, id: u32) -> (u32, u64) {
    let tm = r32(c, gm + 0x18f4);
    let info = r32(c, tm + 8 + 4 * id);
    let tex0 = r32(c, info + 0x38) as u64 | (r32(c, info + 0x3c) as u64) << 32;
    (info, tex0)
}

fn gif_ad(regs: &[(u64, u8)]) -> Step {
    let mut q = vec![(regs.len() as u128 | 0x1000_0000_0000_8000) | 0xe << 64];
    q.extend(regs.iter().map(|&(v, r)| v as u128 | (r as u128) << 64));
    Step::Gif(q)
}

fn pow2(n: u64) -> u64 {
    1 << (n & 31)
}

fn sx(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// Slot 53 `SetRenderTargetTexture(gm, id)` (0x37d348): draws go into the
/// texture's VRAM: FRAME_1 (its block / 32, width / 64, PSM), SCISSOR_1
/// (the whole texture), XYOFFSET_1 (centred on 2048). Returns 1.
pub fn set_render_target_texture(c: &mut Cpu) {
    let (gm, id) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let (info, tex0) = tex_info(c, gm, id);
    let (tw, th, psm) = ((tex0 >> 26) & 0xf, (tex0 >> 30) & 0xf, (tex0 >> 20) & 0x3f);
    let tbp = r32(c, info + 0x28) as i32;
    let fbw = (pow2(tw) as i32 >> 6) as u32 as u64;
    let frame = (tbp as i64 >> 5) as u64 | fbw << 16 | psm << 24;
    let (x1, y1) = (sx(pow2(tw) as u32 - 1), sx(pow2(th) as u32 - 1));
    let ofx = sx((0x800u32.wrapping_sub((pow2(tw) as i32 >> 1) as u32)) << 4);
    let ofy = sx((0x800u32.wrapping_sub((pow2(th) as i32 >> 1) as u32)) << 4);
    let steps = vec![gif_ad(&[
        (frame, 0x4c),
        (x1 << 16 | y1 << 48, 0x40),
        (ofx | ofy << 32, 0x18),
    ])];
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, false);
    c.g[V0][0] = 1;
    ret(c, 120);
}

/// Slot 54 `RestoreRenderTarget(gm)` (0x37d450): back to the frame buffer
/// (gm+0x5a84, width gm+0x5a50, PSM gm+0x5a3c), the viewport scissor in
/// both contexts, the XYOFFSET for the frame size; a continuation record.
pub fn restore_render_target(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    let frame = r32(c, gm + 0x5a84) as u64
        | (r32(c, gm + 0x5a50) as u64) << 16
        | (r32(c, gm + 0x5a3c) as u64) << 24;
    let h16 = |c: &mut Cpu, o: u32| c.bus.read16(gm + o) as u32;
    let (x0, y0, w, h) = (
        h16(c, 0x5900),
        h16(c, 0x5902),
        h16(c, 0x5904),
        h16(c, 0x5906),
    );
    let scissor = x0 as u64
        | sx(x0.wrapping_sub(1).wrapping_add(w)) << 16
        | (y0 as u64) << 32
        | sx(y0.wrapping_sub(1).wrapping_add(h)) << 48;
    let (fw, fh) = (r32(c, gm + 0x5a30), r32(c, gm + 0x5a34));
    let ofx = (0x800u32.wrapping_sub(fw >> 1) << 4) as u64;
    let ofy = (0x800u32.wrapping_sub(fh >> 1) << 4) as u64;
    let steps = vec![gif_ad(&[
        (frame, 0x4c),
        (scissor, 0x40),
        (scissor, 0x41),
        (ofx | ofy << 32, 0x18),
    ])];
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, true);
    c.g[V0][0] = 0;
    ret(c, 110);
}

/// Slot 55 `FillRenderTarget(gm, id, argb)` (0x37d800): a sprite over the
/// whole texture in a flat colour (channels × 128, sign-extended and
/// OR-ed as the builder does), then SCISSOR_1 inset by a texel; a
/// continuation record.
pub fn fill_render_target(c: &mut Cpu) {
    let (gm, id, argb) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
    let (_, tex0) = tex_info(c, gm, id);
    let (tw, th) = ((tex0 >> 26) & 0xf, (tex0 >> 30) & 0xf);
    let k = FpuF32(0x4300_0000);
    let ch = |c: &mut Cpu, o: u32| sx((FpuF32(r32(c, argb + o)) * k).to_i32() as u32);
    let (g, b, a, r) = (ch(c, 8), ch(c, 0xc), ch(c, 0), ch(c, 4));
    let rgbaq = r | g << 8 | b << 16 | a << 24 | (FpuF32::ONE.0 as u64) << 32;
    let half = |n: u64| (pow2(n) as i32 >> 1) as u32;
    let z1 = 1u64 << 32;
    let x0 = sx(0x800u32.wrapping_sub(half(tw)) << 4);
    let y0 = sx(0x800u32.wrapping_sub(half(th)) << 4);
    let x1 = sx(half(tw).wrapping_add(0x801) << 4);
    let y1 = sx(half(th).wrapping_add(0x801) << 4);
    let scissor = sx(pow2(tw) as u32 - 2) << 16 | sx(pow2(th) as u32 - 2) << 48 | 0x1_0000_0001;
    let steps = vec![Step::Gif(vec![
        0x4400_0000_0000_0001 | 0x5510 << 64,
        0x10e | (rgbaq as u128) << 64,
        (x0 | y0 << 16 | z1) as u128 | ((x1 | y1 << 16 | z1) as u128) << 64,
        0x1000_0000_0000_8001 | 0xe << 64,
        scissor as u128 | 0x40 << 64,
    ])];
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, true);
    ret(c, 180);
}
