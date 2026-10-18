//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for board and rider effects (wake, sparks, streamers, trails,
//! sprays), each with a differential test against the generated body.
//!
//! The render functions here share one shape: push a copy of the graphics
//! manager's current GS state record (`gfx+0xe84` points at the top of a
//! stack of 0x14-byte records), load the default record over it, edit its
//! bit fields (blend, test, texture), fill vertex arrays on the stack (0x30
//! bytes per vertex: four control words, RGBA as integers at +0x10, the
//! position quadword at +0x20) and hand them to the manager's draw method
//! (vtable `+0x10d8`, slot 0x238: `Draw(gfx, count, verts, 0)`), then pop
//! the record.
//!
//! The functions keep the register-level operations of the original (the
//! FPU and VU0 registers they leave set, the unaligned record copies through
//! GPRs, the 128-bit `lq`/`sq` through v0/v1): the draw calls see every GPR,
//! and VU0's registers and flags are visible to later code.
//!
//! The update functions (`AttachedEmitterFX_Update`, `cBoardSparkFX_Update`)
//! decide per frame whether their emitters spawn and set up the particle
//! descriptors; their callees (`DynamicEmitter_Spawn*`, `Emitter_*`,
//! `Noise_Fractal1D`, the bone lookups) are called as the original does.
//!
//! Not here:
//! - `cBoardWakeFX_Update` (0x2dd0b8) is a `match pc` dispatcher (its loop
//!   back edges return on an unwind request and it is re-entered at three
//!   loop heads) with a computed jump out; left generated.
//! - `cDynamicSprayFX_Update` (0x2df920) mostly dispatches to eleven other
//!   large spray functions (`SetupEmitters`, `FindBones`, `QueueBurst`,
//!   `UpdateEmitterEnables`, `UpdateEmitter0..9`); a differential test that
//!   reaches its branches would have to run all of them on fabricated
//!   spray state, so it stays generated.

use crate::abi::r32;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_b198, rider_base_render_power_up_trail, "RiderBase_RenderPowerUpTrail"),
    (0x002e_f950, c_air_streamer_fx_render, "cAirStreamerFX_Render"),
    (0x002e_3af8, c_player_pulse_fx_render, "cPlayerPulseFX_Render"),
    (0x002e_7a10, c_speed_boost_streamer_fx_render, "cSpeedBoostStreamerFX_Render"),
    (0x002f_1a08, attached_emitter_fx_update, "AttachedEmitterFX_Update"),
    (0x002d_abc8, c_board_spark_fx_update, "cBoardSparkFX_Update"),
];

// ---------------------------------------------------------------------------
// The graphics manager and its GS state stack.

/// `*(gp - GFX_MAN)`: the graphics manager.
const GFX_MAN: u32 = 0x854;
/// `*(gp - APP)`: the application (`+0x84` the game module).
const APP: u32 = 0x848;
/// `gfx + GS_TOP`: pointer to the top GS state record.
const GS_TOP: u32 = 0xe84;
/// Size of a GS state record.
const RECORD: u32 = 0x14;
/// The default GS state record (`0x500000 + 0x1420`).
const GS_DEFAULT: u32 = 0x0050_1420;
/// `gfx + GFX_VTABLE`: the manager's vtable.
const GFX_VTABLE: u32 = 0x10d8;
/// Vtable slot of `Draw(gfx, count, verts, flags)`.
const DRAW: u32 = 0x238;

/// `base + off` as a guest address (`addiu`/`lw` offsets).
#[inline(always)]
fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// The graphics manager pointer (`lw gp-0x854`).
#[inline]
fn gfx(c: &mut Cpu) -> u64 {
    let a = r32(c, GP).wrapping_sub(GFX_MAN);
    c.lw(a)
}

/// `rd = sx(rd + k)` on a 32-bit register value.
#[inline(always)]
fn addiu(c: &mut Cpu, rd: usize, rs: usize, k: u32) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(k));
}

/// Copy a GS state record from `src` to `dst` as gcc does: two unaligned
/// doublewords through `lo` and `hi` (`ldl`/`ldr`, `sdl`/`sdr`) and the last
/// word through `word`.
fn copy_record(c: &mut Cpu, src: u32, dst: u32, lo: usize, hi: usize, word: usize) {
    c.ldl(lo, src.wrapping_add(7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(8));
    c.g[word][0] = c.lw(src.wrapping_add(0x10));
    c.sdl(lo, dst.wrapping_add(7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(8));
    let w = c.g[word][0];
    c.sw(dst.wrapping_add(0x10), w);
}

/// One bit-field edit of the top GS record: `top = gfx->gs_top` (into
/// `top_reg`), `top[off] = (top[off] & keep) | set` (through v0).
fn gs_field(c: &mut Cpu, gfx: u64, top_reg: usize, off: u32, keep: u64, set: u64) {
    let top = c.lw(at(gfx, GS_TOP));
    c.g[top_reg][0] = top;
    let v = (c.lw(at(top, off)) & keep) | set;
    c.g[V0][0] = v;
    c.sw(at(top, off), v);
}

/// Call method `slot` of the graphics manager whose pointer is in
/// `gfx_reg`: the vtable goes to `vt_reg`, the method to `fn_reg`, the
/// adjusted `this` to a0 (the caller has set a1-a3).
fn gfx_method(c: &mut Cpu, gfx_reg: usize, slot: u32, vt_reg: usize, fn_reg: usize, ret: u32) -> Flow {
    let gfx = c.g[gfx_reg][0];
    let vt = c.lw(at(gfx, GFX_VTABLE));
    c.g[vt_reg][0] = vt;
    let delta = c.lh(at(vt, slot));
    c.g[A0][0] = delta;
    let f = c.lw(at(vt, slot + 4));
    c.g[fn_reg][0] = f;
    c.g[A0][0] = sx((gfx as u32).wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}

/// `Draw(gfx, count, verts, flags)` (method 0x238), see [`gfx_method`].
fn draw(c: &mut Cpu, gfx_reg: usize, vt_reg: usize, fn_reg: usize, ret: u32) -> Flow {
    gfx_method(c, gfx_reg, DRAW, vt_reg, fn_reg, ret)
}

/// Pop the GS state record pushed at the start (`gfx->gs_top -= 0x14`), the
/// manager in `gfx_reg`, the new top in v0.
fn pop_record(c: &mut Cpu, gfx_reg: usize) {
    let gfx = gfx(c);
    c.g[gfx_reg][0] = gfx;
    let top = c.lw(at(gfx, GS_TOP));
    c.g[V0][0] = sx((top as u32).wrapping_sub(RECORD));
    let v = c.g[V0][0];
    c.sw(at(gfx, GS_TOP), v);
}

/// An event check at a loop back edge (structured loop).
#[inline]
fn back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// `vf5 = vf4 * vf3.x` with `vf4 = *src` and `vf3 = gpr` (`qmtc2`, all 128
/// bits: the float was moved in with `mfc1` into the low word), stored to
/// `dst`. The scale `f` goes through `gpr` as the original does.
fn vmul_bc(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, src);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `vf5 = *a + *b` (or `- *b`) through vf3/vf4, stored to `dst`.
fn vadd(c: &mut Cpu, a: u32, b: u32, dst: u32, sub: bool) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}

/// `lq v0, src; sq v0, dst` (a quadword copied through v0).
fn copy_quad(c: &mut Cpu, src: u32, dst: u32) {
    c.lq(V0, src);
    c.sq(V0, dst);
}

// ---------------------------------------------------------------------------
// RiderBase_RenderPowerUpTrail.

/// `RiderBase` fields used by the power-up trail.
mod trail {
    /// Number of trail sections.
    pub const COUNT: u32 = 0xc;
    /// Ring index of the first section (sections are `index % 3`).
    pub const FIRST: u32 = 0x10;
    /// Ring of 3 sections, 0x40 bytes each: four points from +0x20.
    pub const POINTS: u32 = 0x20;
    /// The two points of the trail's head.
    pub const HEAD_A: u32 = 0xe0;
    pub const HEAD_B: u32 = 0xf0;
}

/// Stack layout of the trail: four vertex arrays of 7 vertices (0x150
/// bytes) from sp+0, their counts at sp+0x540.., the colour at sp+0x550
/// (alpha, r, g, b).
mod trail_sp {
    pub const ARRAY: u32 = 0x150;
    pub const COUNTS: u32 = 0x540;
    pub const ALPHA: u32 = 0x550;
    pub const R: u32 = 0x554;
    pub const G: u32 = 0x558;
    pub const B: u32 = 0x55c;
}

/// `void RiderBase_RenderPowerUpTrail(RiderBase *r)` (0x2eb198): with at
/// least two trail sections, push a GS state (texture `gfx+0x1048`, alpha
/// blend), build four triangle strips (one per edge of the trail's
/// cross-section: the head points `+0xe0`/`+0xf0`, then for each section
/// `i` the points of ring slot `(first + i) % 3` with `v = 1 - i/(n-1)`)
/// and draw them; pop the state.
///
/// Frame 0x5b0: s0-s3 at +0x5a0..+0x570, ra at +0x560.
pub fn rider_base_render_power_up_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x5b0);
        frame.save(c, S0, 0x5a0);
        frame.save(c, S1, 0x590);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x580);
        frame.save(c, S3, 0x570);
        frame.save_ra(c, 0x560);
        let n = c.lw(at(c.g[S0][0], trail::COUNT));
        c.g[V0][0] = ((n as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x5a0);
        } else {
            trail_draw(c, frame)?;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x590);
        frame.restore(c, S2, 0x580);
        frame.restore(c, S3, 0x570);
        frame.restore_ra(c, 0x560);
        frame.close(c);
        Ok(())
    })
}

/// The trail's drawing (from the GS state push to the pop and the `s0`
/// reload).
fn trail_draw(c: &mut Cpu, frame: Frame) -> Flow {
    use trail_sp::*;
    let sp = r32(c, SP);
    c.cycles += 95;
    // Push: copy the top record up, load the default into the new top.
    let g = gfx(c);
    c.g[V1][0] = g;
    c.g[A1][0] = 0x50_0000;
    c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
    c.g[T0][0] = 0xffff_ffff_ffcf_ffff;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T1][0] = 0xffff_ffff_fff0_0fff;
    copy_record(c, top as u32, at(top, RECORD), A0, A3, T2);
    c.g[A3][0] = 0x1_4000;
    c.g[T4][0] = 0xffff_ffff_ffff_fffc;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[T2][0] = 0xffff_ffff_e000_03ff;
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
    c.g[T6][0] = 0xffff_ffff_ffff_ff83;
    c.g[T3][0] = 0xffff_ffff_ffbf_ffff;
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V0][0] = top;
    c.g[T7][0] = 0x40_0000;
    c.g[S1][0] = 3;
    c.g[A0][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, S2, S3, V1);
    c.g[T8][0] = sx(sp.wrapping_add(COUNTS));
    c.g[S2][0] = u64::MAX;
    // Texture and state bits.
    let g = gfx(c);
    c.g[V1][0] = g;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = top;
    let tex = c.lw(at(g, 0x1048));
    c.g[V0][0] = tex;
    c.sh(at(top, 0x10), tex);
    let g = gfx(c);
    c.g[A1][0] = g;
    gs_field(c, g, V1, 0, u64::MAX, 0xc);
    gs_field(c, g, A0, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, A2, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    // gcc's empty constructor loops over the 4 x 7 vertices.
    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(1));
    loop {
        c.cycles += 2;
        c.g[V0][0] = 6;
        loop {
            c.cycles += 7;
            addiu(c, V0, V0, u32::MAX);
            let more = c.g[V0][0] != c.g[S2][0];
            c.g[S1][0] = c.g[V1][0];
            if !more {
                break;
            }
            back_edge(c, 0x002e_b348);
        }
        c.cycles += 2;
        let more = c.g[S1][0] != c.g[S2][0];
        addiu(c, V1, S1, u32::MAX);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b340);
    }
    // Zero the four counts.
    c.cycles += 3;
    c.g[V1][0] = 3;
    addiu(c, V0, T8, 0xc);
    loop {
        c.cycles += 7;
        let a = r32(c, V0);
        c.sw(a, 0);
        addiu(c, V1, V1, u32::MAX);
        let more = (c.g[V1][0] as i64) >= 0;
        addiu(c, V0, V0, (-4i32) as u32);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b378);
    }
    // The head: one vertex each into strips 1 and 3 (colour 1.0, 255).
    c.cycles += 68;
    c.g[V0][0] = c.lw(sp.wrapping_add(COUNTS + 4));
    c.g[A0][0] = 0x30;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.g[A1][0] = 0xff;
    c.mult(0, A2, V0, A0);
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 3);
    addiu(c, V0, V0, 1);
    for off in [ALPHA, G, B, R] {
        c.swc1(2, sp.wrapping_add(off));
    }
    c.g[V1][0] = sx(r32(c, A2).wrapping_add(sp));
    let v0 = c.g[V0][0];
    c.sw(sp.wrapping_add(COUNTS + 4), v0);
    addiu(c, V1, V1, ARRAY);
    let v = r32(c, V1);
    let a1 = c.g[A1][0];
    c.sw(v.wrapping_add(0x10), a1);
    let this = c.g[S0][0];
    c.lq(A1, at(this, trail::HEAD_A));
    c.lwc1(1, sp.wrapping_add(G));
    c.lq(V0, at(this, trail::HEAD_B));
    c.mul_s(1, 1, 3);
    c.g[A3][0] = c.lw(at(this, trail::COUNT));
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 5);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 4);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x14));
    c.lwc1(0, sp.wrapping_add(B));
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x18));
    c.lwc1(1, sp.wrapping_add(ALPHA));
    c.sq(V0, v.wrapping_add(0x20));
    c.mul_s(1, 1, 5);
    c.sw(v.wrapping_add(4), 0);
    c.swc1(2, v.wrapping_add(8));
    c.swc1(4, v);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x1c));
    c.g[V0][0] = c.lw(sp.wrapping_add(COUNTS + 0xc));
    c.lwc1(1, sp.wrapping_add(R));
    c.mult(0, V1, V0, A0);
    c.mul_s(1, 1, 3);
    addiu(c, V0, V0, 1);
    let v0 = c.g[V0][0];
    c.sw(sp.wrapping_add(COUNTS + 0xc), v0);
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(sp));
    addiu(c, A0, A0, 3 * ARRAY);
    let v = r32(c, A0);
    trail_color(c, sp, v);
    c.sq(A1, v.wrapping_add(0x20));
    c.sw(v.wrapping_add(4), 0);
    c.swc1(2, v.wrapping_add(8));
    c.swc1(4, v);
    let none = (c.g[A3][0] as i64) <= 0;
    c.g[T4][0] = 0;
    if !none {
        c.cycles += 5;
        c.mov_s(6, 4);
        c.g[T5][0] = 3;
        c.mov_s(4, 2);
        c.g[T3][0] = 0x30;
        loop {
            trail_section(c, sp);
            let more = c.g[A0][0] != 0;
            if !more {
                break;
            }
            back_edge(c, 0x002e_b4b8);
        }
    }
    // Draw the four strips.
    c.cycles += 4;
    c.g[S1][0] = c.g[T8][0];
    c.g[S0][0] = c.g[SP][0];
    c.g[S2][0] = 3;
    c.g[T0][0] = gfx(c);
    loop {
        c.cycles += 13;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[A3][0] = 0;
        addiu(c, S1, S1, 4);
        addiu(c, S0, S0, ARRAY);
        addiu(c, S2, S2, u32::MAX);
        draw(c, T0, V1, V0, 0x002e_b8bc)?;
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[T0][0] = gfx(c);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b890);
    }
    c.cycles += 5;
    pop_record(c, V1);
    frame.restore(c, S0, 0x5a0);
    Ok(())
}

/// The trail colour of vertex `v` from sp+0x550.. (alpha * f5 = 128,
/// rgb * f3 = 255, as integers at v+0x10..0x1c), r already scaled in f1.
fn trail_color(c: &mut Cpu, sp: u32, v: u32) {
    use trail_sp::*;
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x10));
    c.lwc1(0, sp.wrapping_add(G));
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(1, sp.wrapping_add(B));
    c.mul_s(1, 1, 3);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x18));
    c.lwc1(0, sp.wrapping_add(ALPHA));
    c.mul_s(0, 0, 5);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x1c));
}

/// Append one vertex to strip `strip` (count at sp+0x540+4*strip, bumped;
/// the offset through `rd`): colour, `v` (f2) at +4, the point at `src` (via
/// v0) at +0x20, f4 at +8 and, when `one`, at +0 (else 0). v1 = the vertex.
fn trail_vertex(c: &mut Cpu, sp: u32, strip: u32, rd: usize, src: u32, one: bool) {
    use trail_sp::*;
    let count = sp.wrapping_add(COUNTS + 4 * strip);
    c.g[V0][0] = c.lw(count);
    c.lwc1(1, sp.wrapping_add(R));
    c.mult(0, rd, V0, T3);
    c.mul_s(1, 1, 3);
    addiu(c, V0, V0, 1);
    let n = c.g[V0][0];
    c.sw(count, n);
    let v = r32(c, rd).wrapping_add(sp).wrapping_add(strip * ARRAY);
    c.g[V1][0] = sx(v);
    trail_color(c, sp, v);
    c.swc1(4, v.wrapping_add(8));
    c.lq(V0, src);
    c.swc1(2, v.wrapping_add(4));
    c.sq(V0, v.wrapping_add(0x20));
    if one {
        c.swc1(4, v);
    } else {
        c.sw(v, 0);
    }
}

/// One trail section (loop body at 0x2eb4b8): section `i = t4` of ring
/// slot `(first + i) % 3`, two vertices per strip. Leaves a0 = `i+1 < n`.
fn trail_section(c: &mut Cpu, sp: u32) {
    use trail_sp::*;
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c.g[S0][0], trail::FIRST));
    c.cycles += 239;
    // Strip 0 gets the section's first vertex: alpha 0x7f, v = 1 - i/(n-1).
    c.g[V1][0] = c.lw(sp.wrapping_add(COUNTS));
    c.g[A1][0] = 0x7f;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T4)));
    c.swc1(6, sp.wrapping_add(ALPHA));
    c.div(0, V0, T5);
    c.swc1(6, sp.wrapping_add(G));
    c.swc1(6, sp.wrapping_add(B));
    c.mtc1(T4, 2);
    c.cvt_s_w(2, 2);
    c.swc1(6, sp.wrapping_add(R));
    addiu(c, T4, T4, 1);
    c.g[A2][0] = c.hi[0];
    c.mult(0, V0, V1, T3);
    c.g[A2][0] = sx(r32(c, A2) << 6);
    addiu(c, V1, V1, 1);
    let n = c.g[V1][0];
    c.sw(sp.wrapping_add(COUNTS), n);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(sp));
    let v = r32(c, A0);
    let a1 = c.g[A1][0];
    c.sw(v.wrapping_add(0x10), a1);
    addiu(c, V0, A3, u32::MAX);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lwc1(0, sp.wrapping_add(G));
    c.mul_s(0, 0, 3);
    c.div_s(2, 2, 1);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(0, sp.wrapping_add(B));
    c.sub_s(2, 4, 2);
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x18));
    c.lwc1(1, sp.wrapping_add(ALPHA));
    c.swc1(4, v.wrapping_add(8));
    c.mul_s(1, 1, 5);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x1c));
    let section = sx(r32(c, A2).wrapping_add(r32(c, S0)));
    for r in [V0, A3, A1] {
        c.g[r][0] = section;
    }
    // The section's four points.
    let p = (section as u32).wrapping_add(trail::POINTS);
    let point = |k: u32| p.wrapping_add(0x10 * k);
    c.lq(V1, point(0));
    c.swc1(2, v.wrapping_add(4));
    c.sq(V1, v.wrapping_add(0x20));
    c.sw(v, 0);
    for r in [T2, T0, T1, A2, A0] {
        c.g[r][0] = section;
    }
    // The rest of the section: (strip, point) pairs.
    trail_vertex(c, sp, 0, T6, point(1), true);
    trail_vertex(c, sp, 1, A1, point(1), true);
    trail_vertex(c, sp, 1, A1, point(2), false);
    trail_vertex(c, sp, 2, A1, point(2), false);
    trail_vertex(c, sp, 2, A1, point(3), true);
    trail_vertex(c, sp, 3, A1, point(3), true);
    c.g[A3][0] = c.lw(at(c.g[S0][0], trail::COUNT));
    c.g[A0][0] = ((c.g[T4][0] as i64) < (c.g[A3][0] as i64)) as u64;
    trail_vertex(c, sp, 3, A1, point(0), false);
}

// ---------------------------------------------------------------------------
// Streamers (cAirStreamerFX, cSpeedBoostStreamerFX).

/// Where a streamer keeps its data: the effect pointer's register, its
/// point list and point count fields, and the stack layout (a temporary
/// block at `sp+tmp`: +0 the offset added to the points, +0x10 the colour
/// (alpha, r, g, b), +0x20/+0x30 the two points of a segment, +0x50 the
/// half width, +0x60/+0x70 scratch; two vertex arrays `half` bytes apart
/// from sp+0).
struct Streamer {
    this: usize,
    points: u32,
    count: u32,
    tmp: u32,
    half: u32,
}

/// One streamer vertex at `v`: position `*p ± *(t+0x50)` (through
/// vf3-vf5, t+0x70, v0 and t+0x60), colour from t+0x10.. (rgb * f2 = 255,
/// alpha * f6 = 128), f3 at +8, f4 at +4 and f`first` at +0.
fn streamer_vertex(c: &mut Cpu, t: u32, p: u32, sub: bool, v: u32, first: usize) {
    vadd(c, p, t.wrapping_add(0x50), t.wrapping_add(0x70), sub);
    copy_quad(c, t.wrapping_add(0x70), t.wrapping_add(0x60));
    c.lwc1(1, t.wrapping_add(0x14));
    c.mul_s(1, 1, 2);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x10));
    c.lwc1(0, t.wrapping_add(0x18));
    c.mul_s(0, 0, 2);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(1, t.wrapping_add(0x1c));
    c.mul_s(1, 1, 2);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x18));
    c.lwc1(0, t.wrapping_add(0x10));
    c.swc1(3, v.wrapping_add(8));
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x1c));
    c.lq(V0, t.wrapping_add(0x60));
    c.sq(V0, v.wrapping_add(0x20));
    c.swc1(first, v);
    c.swc1(4, v.wrapping_add(4));
}

/// One streamer segment (the 137-cycle loop body both streamers share):
/// point `a2` (index into the 32-byte point pairs, then `a2 -= 1`) plus
/// the offset gives the segment's two points; each array gets the pair
/// `p - w`, `p + w` (v from f4, advanced by f8; f7 += 1). a3 counts the
/// segments, s0 the vertices per array, a1 walks the second array. Returns
/// whether `a3 < count`.
fn streamer_segment(c: &mut Cpu, s: &Streamer) -> bool {
    let t = r32(c, SP).wrapping_add(s.tmp);
    c.cycles += 137;
    c.g[V0][0] = c.lw(at(c.g[s.this][0], s.points));
    c.g[A0][0] = sx(r32(c, A2) << 5);
    addiu(c, A3, A3, 1);
    addiu(c, S0, S0, 2);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    addiu(c, A2, A2, u32::MAX);
    let p0 = r32(c, V0);
    vadd(c, p0, t, t.wrapping_add(0x70), false);
    c.add_s(7, 7, 3);
    c.g[V1][0] = c.lw(at(c.g[s.this][0], s.points));
    c.lq(V0, t.wrapping_add(0x70));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    c.sq(V0, t.wrapping_add(0x60));
    c.sq(V0, t.wrapping_add(0x20));
    let p1 = r32(c, A0).wrapping_add(0x10);
    vadd(c, p1, t, t.wrapping_add(0x70), false);
    copy_quad(c, t.wrapping_add(0x70), t.wrapping_add(0x60));
    c.sq(V0, t.wrapping_add(0x30));
    let a = r32(c, A1);
    let (pa, pb) = (t.wrapping_add(0x20), t.wrapping_add(0x30));
    streamer_vertex(c, t, pa, true, a.wrapping_sub(s.half), 5);
    streamer_vertex(c, t, pb, true, a, 5);
    let a = a.wrapping_add(0x30);
    c.g[A1][0] = sx(a);
    streamer_vertex(c, t, pa, false, a.wrapping_sub(s.half), 3);
    streamer_vertex(c, t, pb, false, a, 3);
    let n = c.lw(at(c.g[s.this][0], s.count));
    c.g[V1][0] = ((c.g[A3][0] as i64) < (n as i64)) as u64;
    c.add_s(4, 4, 8);
    let more = c.g[V1][0] != 0;
    c.g[A1][0] = sx(a.wrapping_add(0x30));
    more
}

/// `cAirStreamerFX` fields.
mod air {
    /// The rider (`+0x780` model, `+0x89c`/`+0x8a4` bone indices, `+0x2ec`
    /// speed).
    pub const RIDER: u32 = 0;
    /// Point pairs (32 bytes per index).
    pub const POINTS: u32 = 0x4;
    /// Index of the newest point.
    pub const FIRST: u32 = 0x8;
    /// Number of segments.
    pub const COUNT: u32 = 0xc;
    /// Alpha scale (textured mode).
    pub const ALPHA: u32 = 0x10;
    /// Length used for the v step.
    pub const LENGTH: u32 = 0x14;
}

const AIR: Streamer = Streamer { this: T6, points: air::POINTS, count: air::COUNT, tmp: 0x1380, half: 0x9c0 };

/// `void cAirStreamerFX_Render(cAirStreamerFX *fx)` (0x2ef950): with at
/// least two segments, push a GS state (textured with `gfx+0x1044` when
/// the global switch `gp+0x1630` is set or the rider is moving, else flat
/// with `gfx+0x104c`), build the streamer's two triangle strips (each
/// point pair offset along the rider's bone, faded by `1 - (i/len)²`) and
/// draw them; pop the state.
///
/// Frame 0x1430: s0 +0x1420, s1 +0x1410, ra +0x1400.
pub fn c_air_streamer_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x1430);
        c.g[T6][0] = c.g[A0][0];
        frame.save(c, S0, 0x1420);
        frame.save(c, S1, 0x1410);
        frame.save_ra(c, 0x1400);
        let n = c.lw(at(c.g[T6][0], air::COUNT));
        c.g[V0][0] = ((n as i64) < 2) as u64;
        let skip = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x1420);
        if !skip {
            air_draw(c, frame)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x1410);
        frame.restore_ra(c, 0x1400);
        frame.close(c);
        Ok(())
    })
}

fn air_draw(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    c.cycles += 34;
    // Push the GS state.
    let g = gfx(c);
    c.g[A0][0] = g;
    c.g[A2][0] = 0x50_0000;
    c.lwc1(10, gp.wrapping_sub(0x3938));
    let top = c.lw(at(g, GS_TOP));
    c.g[V1][0] = top;
    c.mtc1(ZERO, 1);
    copy_record(c, top as u32, at(top, RECORD), V0, A1, A3);
    let top = c.lw(at(g, GS_TOP));
    c.g[A1][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    let top = c.lw(at(c.g[A1][0], GS_TOP));
    c.g[V1][0] = top;
    c.g[A3][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, V0, A0, A1);
    c.g[V0][0] = c.lw(gp.wrapping_add(0x1630));
    c.g[T9][0] = 0;
    let mut textured = c.g[V0][0] != 0;
    if !textured {
        c.cycles += 6;
        let rider = c.lw(at(c.g[T6][0], air::RIDER));
        c.g[V0][0] = rider;
        c.lwc1(0, at(rider, 0x2ec));
        c.c_lt_s(1, 0);
        textured = c.fcc();
        c.g[V1][0] = gfx(c);
    }
    if textured {
        c.cycles += 15;
        let g = gfx(c);
        c.g[V1][0] = g;
        c.g[A1][0] = 0xffff_ffff_ffff_fff3;
        c.g[T9][0] = 1;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 10);
        let top = c.lw(at(g, GS_TOP));
        c.g[A0][0] = top;
        let tex = c.lw(at(g, 0x1044));
        c.g[V0][0] = tex;
        c.sh(at(top, 0x10), tex);
        let g = gfx(c);
        c.g[V1][0] = g;
        gs_field(c, g, A0, 0, 0xffff_ffff_ffff_fff3, 0);
        c.lwc1(1, at(c.g[T6][0], air::ALPHA));
    } else {
        c.cycles += 7;
        let g = c.g[V1][0];
        gs_field(c, g, A0, 0, u64::MAX, 0xc);
        let top = c.lw(at(g, GS_TOP));
        c.g[A1][0] = top;
        let tex = c.lw(at(g, 0x104c));
        c.g[V0][0] = tex;
        c.sh(at(top, 0x10), tex);
    }
    c.cycles += 57;
    let g = gfx(c);
    c.g[A1][0] = g;
    c.g[V1][0] = 0xffff_ffff_ffbf_ffff;
    c.g[T2][0] = 0x40_0000;
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
    c.g[A3][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T0][0] = 0xffff_ffff_fff0_0fff;
    c.g[T1][0] = 0x1_4000;
    c.g[T3][0] = 0xffff_ffff_ffff_fffc;
    c.g[T2][0] = 0xffff_ffff_e000_03ff;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    c.g[T5][0] = 0xffff_ffff_ffff_ff83;
    c.g[T7][0] = 0x33;
    c.g[S1][0] = sx(sp.wrapping_add(AIR.half));
    c.g[T8][0] = u64::MAX;
    gs_field(c, g, A0, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_fffc, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, V1, 8, 0xffff_ffff_ffff_fc1f, 0x100);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    // gcc's empty constructor loops over the 2 x 52 vertices.
    loop {
        c.cycles += 7;
        addiu(c, T7, T7, u32::MAX);
        let more = c.g[T7][0] != c.g[T8][0];
        c.g[V0][0] = 0x33;
        if !more {
            break;
        }
        back_edge(c, 0x002e_fb50);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(at(c.g[T6][0], air::RIDER));
    c.g[A0][0] = c.lw(at(c.g[T6][0], air::LENGTH));
    c.g[V1][0] = u64::MAX;
    loop {
        c.cycles += 7;
        addiu(c, V0, V0, u32::MAX);
        let more = c.g[V0][0] != c.g[V1][0];
        c.mov_s(4, 1);
        if !more {
            break;
        }
        back_edge(c, 0x002e_fb78);
    }
    // Colour 1.0, the offset along the rider's bone, 1/(len-1)² and
    // 1/count.
    c.cycles += 42;
    let rider = c.g[A2][0];
    c.g[A1][0] = c.lw(at(rider, 0x780));
    addiu(c, A0, A0, u32::MAX);
    c.g[V0][0] = c.lw(at(rider, 0x89c));
    c.mult(0, A0, A0, A0);
    c.g[V1][0] = c.lw(at(c.g[A1][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.lwc1(1, gp.wrapping_sub(0x3934));
    let bone = r32(c, V1);
    c.lq(V0, bone.wrapping_add(0x30));
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(ZERO, 7);
    c.g[S0][0] = 0;
    let t = sp.wrapping_add(AIR.tmp);
    c.sq(V0, t.wrapping_add(0x10));
    vmul_bc(c, V1, 1, t.wrapping_add(0x10), t.wrapping_add(0x20));
    c.div_s(9, 3, 0);
    c.lwc1(0, at(c.g[T6][0], air::COUNT));
    c.cvt_s_w(0, 0);
    c.lq(V0, t.wrapping_add(0x20));
    for k in 0..3 {
        c.swc1(3, t.wrapping_add(0x10 + 4 * k));
    }
    c.div_s(8, 3, 0);
    c.swc1(3, t.wrapping_add(0x1c));
    c.sq(V0, t);
    let rider = c.lw(at(c.g[T6][0], air::RIDER));
    c.g[V0][0] = rider;
    c.g[AT][0] = 0x4170_0000;
    c.mtc1(AT, 1);
    c.g[A1][0] = c.lw(at(rider, 0x780));
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[A1][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let axis = r32(c, A0).wrapping_add(0x20);
    c.lq(V0, axis);
    let flat = c.g[T9][0] == 0;
    c.sq(V0, t.wrapping_add(0x40));
    if !flat {
        // Textured: colour (1, k, k, k) with k from gp+0xa84, width 37.5.
        c.cycles += 15;
        c.lwc1(0, gp.wrapping_add(0xa84));
        c.swc1(3, t.wrapping_add(0x50));
        c.swc1(0, t.wrapping_add(0x5c));
        c.swc1(0, t.wrapping_add(0x54));
        c.swc1(0, t.wrapping_add(0x58));
        c.g[AT][0] = 0x4216_0000;
        c.mtc1(AT, 1);
        c.ldl(T0, t.wrapping_add(0x57));
        c.ldr(T0, t.wrapping_add(0x50));
        c.ldl(V0, t.wrapping_add(0x5f));
        c.ldr(V0, t.wrapping_add(0x58));
        c.sdl(T0, t.wrapping_add(0x17));
        c.sdr(T0, t.wrapping_add(0x10));
        c.sdl(V0, t.wrapping_add(0x1f));
        c.sdr(V0, t.wrapping_add(0x18));
    }
    // Half width: the bone axis * width/2.
    c.cycles += 14;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.g[A3][0] = 0;
    c.mul_s(0, 1, 0);
    vmul_bc(c, V1, 0, t.wrapping_add(0x40), t.wrapping_add(0x60));
    c.lq(V0, t.wrapping_add(0x60));
    c.g[V1][0] = c.lw(at(c.g[T6][0], air::COUNT));
    c.sq(V0, t.wrapping_add(0x50));
    let none = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = c.lw(at(c.g[T6][0], air::FIRST));
    if !none {
        c.cycles += 6;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 2);
        c.g[T0][0] = 0x18;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 6);
        c.g[A1][0] = sx(sp.wrapping_add(AIR.half));
        loop {
            // alpha = scale * (1 - f7² / (len-1)²), clamped at 0.
            c.cycles += 12;
            c.mul_s(0, 7, 7);
            c.mtc1(ZERO, 5);
            c.g[V0][0] = (-1i64 < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T0][0];
            }
            c.mul_s(0, 0, 9);
            c.sub_s(0, 3, 0);
            c.mul_s(0, 10, 0);
            c.c_lt_s(0, 5);
            let clamp = c.fcc();
            c.swc1(0, t.wrapping_add(0x10));
            if clamp {
                c.cycles += 1;
                c.swc1(5, t.wrapping_add(0x10));
            }
            if !streamer_segment(c, &AIR) {
                break;
            }
            back_edge(c, 0x002e_fcc8);
        }
    }
    c.cycles += 23;
    c.g[T0][0] = gfx(c);
    c.g[A2][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = 0;
    draw(c, T0, V0, V1, 0x002e_ff44)?;
    c.g[T0][0] = gfx(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    draw(c, T0, V0, V1, 0x002e_ff68)?;
    pop_record(c, A0);
    frame.restore(c, S0, 0x1420);
    Ok(())
}

// ---------------------------------------------------------------------------
// cPlayerPulseFX_Render.

/// `BELibrary_GetInstance` and `BELibrary_IsMode`-style query (0x14dd58:
/// `phase interface->method 8(a1)`), reached through the function table.
const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
const BE_LIBRARY_QUERY: u32 = 0x0014_dd58;
/// Game globals the pulse reads: a "split screen" switch and the game mode
/// byte (modes 4 and 5 are the event modes).
const SPLIT_SWITCH: u32 = 0x0053_4b30;
const GAME_MODE: u32 = 0x0053_5c12;

/// The rider's "is the local player" query: method 0x40 of the rider's
/// interface at `+0x6c0` (`this` = `*(s0) + 0x6c0`, adjusted).
fn pulse_rider_query(c: &mut Cpu, ret: u32) -> Flow {
    let rider = c.lw(at(c.g[S0][0], 0));
    c.g[A0][0] = rider;
    let vt = c.lw(at(rider, 0x6c0));
    c.g[V1][0] = vt;
    c.g[A0][0] = sx(at(rider, 0x6c0));
    c.g[A1][0] = c.lh(at(vt, 0x40));
    let f = c.lw(at(vt, 0x44));
    c.g[V0][0] = f;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, f as u32, ret)
}

/// `void cPlayerPulseFX_Render(cPlayerPulseFX *fx)` (0x2e3af8): decide
/// whether the pulse around the player shows and draw it. With one player
/// it shows when the rider's query (method 0x40 of `rider+0x6c0`) answers
/// 0, the game mode byte is 4 or 5 and the mode object's id is 14..16
/// (after `BELibrary_GetInstance` and a back-end query); in split screen
/// when the query answers non-zero, there are at least two players and
/// the player id differs from `rider+0x870`, or, when the split switch is
/// set, when `rider+0x87c` is 0. Never while the rider is hidden
/// (`+0xac4`). The pulse is one 8-vertex strip: four points on the
/// camera's y axis (scales gp-0x3a24, gp-0x3a20, 130 and 0) around the
/// rider's bone, each split in two along the camera's x axis (* 85),
/// coloured from `fx+0x28..0x34` (rgb * 255, alpha * 128).
///
/// Frame 0x250: s0 +0x240, s1 +0x230, ra +0x220.
pub fn c_player_pulse_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 12;
        let frame = Frame::open(c, 0x250);
        let app = c.lw(gp.wrapping_sub(APP));
        c.g[V0][0] = app;
        frame.save(c, S0, 0x240);
        frame.save(c, S1, 0x230);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x220);
        c.g[V1][0] = c.lw(at(app, 0x84));
        c.g[A0][0] = c.lw(at(c.g[V1][0], 0xc));
        let players = c.lw(at(c.g[A0][0], 0x7c));
        c.g[V0][0] = ((players as i64) < 2) as u64;
        let single = c.g[V0][0] != 0;
        c.g[S1][0] = 0;
        if !single {
            c.cycles += 9;
            pulse_rider_query(c, 0x002e_3b44)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
            let mut direct = false;
            if !none {
                c.cycles += 6;
                let module = c.lw(at(c.g[V0][0], 0x84));
                c.g[V1][0] = module;
                c.g[V1][0] = c.lw(at(module, 0x84));
                let n = c.lw(at(c.g[V1][0], 0x10));
                c.g[V0][0] = (n < 2) as u64;
                direct = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
                if !direct {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
                    c.g[V1][0] = c.lw(at(c.g[V1][0], 0x14));
                    c.g[A0][0] = c.lw(at(c.g[V0][0], 0x870));
                    c.g[V1][0] ^= c.g[A0][0];
                    c.g[S1][0] = (0 < c.g[V1][0]) as u64;
                }
            }
            if !direct {
                c.cycles += 1;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
            }
            c.cycles += 7;
            c.g[A1][0] = 7;
            c.g[A0][0] = c.lw(at(c.g[V0][0], 0x78));
            call_ptr(c, BE_LIBRARY_QUERY, 0x002e_3b88)?;
            c.g[V1][0] = 0x53_0000;
            let split = c.lw(SPLIT_SWITCH);
            c.g[V0][0] = split;
            c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            if split != 0 {
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c.g[V0][0], 0x87c));
                c.g[S1][0] = (c.g[V1][0] < 1) as u64;
            }
        } else {
            c.cycles += 9;
            pulse_rider_query(c, 0x002e_3bc0)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            } else {
                c.cycles += 11;
                call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x002e_3bd0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, BE_LIBRARY_QUERY, 0x002e_3bdc)?;
                c.g[V1][0] = 0x53_0000;
                let mode = c.lb(GAME_MODE);
                c.g[V0][0] = sx((mode as u32).wrapping_sub(4));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let event = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
                if event {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c.g[V0][0], 0x84));
                    c.g[A0][0] = c.lw(at(c.g[V1][0], 0x78));
                    let id = c.lw(at(c.g[A0][0], 0x1bc));
                    c.g[V0][0] = sx((id as u32).wrapping_sub(0xe));
                    c.g[S1][0] = (c.g[V0][0] < 3) as u64;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xac4));
        if c.g[V1][0] != 0 {
            c.g[S1][0] = 0;
        }
        let show = c.g[S1][0] != 0;
        c.g[A1][0] = gfx(c);
        if show {
            pulse_draw(c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x240);
        frame.restore(c, S1, 0x230);
        frame.restore_ra(c, 0x220);
        frame.close(c);
        Ok(())
    })
}

/// The pulse strip (the 381-cycle block): GS state push, the camera matrix
/// (method 0x118), eight vertices at sp+0x80, draw, pop.
fn pulse_draw(c: &mut Cpu) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    let s = |off: u32| sp.wrapping_add(off);
    c.cycles += 381;
    let g = c.g[A1][0];
    c.g[T0][0] = 0x50_0000;
    c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
    c.g[T1][0] = 0x40_0000;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T2][0] = 0xffff_ffff_ffff_ff83;
    c.g[T3][0] = 0xffff_ffff_ffff_fffc;
    copy_record(c, top as u32, at(top, RECORD), V1, A0, A3);
    c.g[A3][0] = 0xffff_ffff_e000_03ff;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V1][0] = top;
    c.g[A1][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, T5, V0, A0);
    let g = gfx(c);
    c.g[A0][0] = g;
    gs_field(c, g, V1, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    gs_field(c, g, A1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    gs_field(c, g, A2, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A1, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    let top = c.lw(at(g, GS_TOP));
    c.g[V1][0] = top;
    let tex = c.lw(at(g, 0xffc));
    c.g[V0][0] = tex;
    c.sh(at(top, 0x10), tex);
    // The camera matrix.
    c.g[A1][0] = gfx(c);
    gfx_method(c, A1, 0x118, V1, V0, 0x002e_3d34)?;
    let m = r32(c, V0);
    // sp+0: the camera's x axis, sp+0x10 its y axis (w = 0).
    c.lwc1(2, m);
    c.g[T0][0] = sx(s(0x20));
    c.lwc1(0, m.wrapping_add(0x10));
    c.g[A2][0] = sx(s(0x80));
    c.lwc1(1, m.wrapping_add(0x20));
    c.g[T1][0] = u64::MAX;
    c.swc1(2, s(0));
    c.g[A3][0] = 7;
    c.swc1(0, s(4));
    c.swc1(1, s(8));
    c.sw(s(0xc), 0);
    c.g[AT][0] = 0x42aa_0000;
    c.mtc1(AT, 3);
    c.lwc1(2, m.wrapping_add(0x24));
    c.lwc1(0, m.wrapping_add(4));
    c.lwc1(1, m.wrapping_add(0x14));
    let rider = c.lw(at(c.g[S0][0], 0));
    c.g[A1][0] = rider;
    c.swc1(0, s(0x10));
    c.swc1(1, s(0x14));
    c.swc1(2, s(0x18));
    c.sw(s(0x1c), 0);
    // The centre: the rider's bone (+0x8a8) position at sp+0x20.
    c.lwc1(0, gp.wrapping_sub(0x3a24));
    c.g[A0][0] = c.lw(at(rider, 0x780));
    c.g[V0][0] = c.lw(at(rider, 0x8a8));
    c.g[V1][0] = c.lw(at(c.g[A0][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.lwc1(1, gp.wrapping_sub(0x3a20));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[AT][0] = 0x4302_0000;
    c.mtc1(AT, 2);
    let bone = r32(c, V1);
    copy_quad(c, bone.wrapping_add(0x30), s(0x20));
    // Ring points: centre + y * f0 / f1 / f2 (sp+0x30/0x40/0x50), the
    // split offset x * 85 at sp+0x60.
    vmul_bc(c, V1, 0, s(0x10), s(0x50));
    copy_quad(c, s(0x50), s(0x40));
    vadd(c, s(0x20), s(0x40), s(0x50), false);
    copy_quad(c, s(0x50), s(0x30));
    vmul_bc(c, V1, 1, s(0x10), s(0x60));
    copy_quad(c, s(0x60), s(0x50));
    vadd(c, s(0x20), s(0x50), s(0x60), false);
    copy_quad(c, s(0x60), s(0x40));
    vmul_bc(c, V1, 2, s(0x10), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x20), s(0x60), s(0x70), false);
    copy_quad(c, s(0x70), s(0x50));
    vmul_bc(c, V1, 3, s(0), s(0x70));
    c.lq(V0, s(0x70));
    // The rider's colour (fx+0x2c..0x34) with alpha 0 at sp+0x70.
    let fx = c.g[S0][0];
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.sq(V0, s(0x60));
    c.swc1(0, s(0x74));
    c.swc1(1, s(0x78));
    c.sw(s(0x70), 0);
    c.swc1(2, s(0x7c));
    // Vertex 0 (sp+0x80): centre - offset, colour sp+0x70.
    vadd(c, s(0x20), s(0x60), s(0x210), true);
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 4);
    c.lwc1(0, s(0x74));
    c.g[A1][0] = 8;
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 6);
    c.g[A3][0] = 0;
    c.lwc1(1, s(0x78));
    c.mul_s(0, 0, 4);
    c.lwc1(2, s(0x7c));
    c.lwc1(3, s(0x70));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x90));
    c.sq(V0, s(0x200));
    c.sq(V0, s(0xa0));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x94));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x98));
    c.swc1(5, s(0x88));
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x9c));
    c.sw(s(0x80), 0);
    c.swc1(5, s(0x84));
    // Vertex 1 (sp+0xb0): centre + offset.
    vadd(c, s(0x20), s(0x60), s(0x210), false);
    c.lwc1(1, s(0x74));
    c.lwc1(2, s(0x78));
    c.lwc1(3, s(0x7c));
    c.mul_s(1, 1, 4);
    c.lwc1(0, s(0x70));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 4);
    c.lq(V0, s(0x210));
    c.mul_s(0, 0, 6);
    c.swc1(5, s(0xb8));
    c.cvt_w_s(7, 1);
    c.swc1(7, s(0xc0));
    c.sq(V0, s(0x200));
    c.cvt_w_s(1, 2);
    c.swc1(1, s(0xc4));
    c.cvt_w_s(1, 3);
    c.swc1(1, s(0xc8));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0xcc));
    c.sq(V0, s(0xd0));
    c.swc1(5, s(0xb0));
    c.swc1(5, s(0xb4));
    // Vertex 2 (sp+0xe0): ring point 3 - offset, colour fx+0x28.
    vadd(c, s(0x50), s(0x60), s(0x210), true);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(2, at(fx, 0x34));
    c.lwc1(3, at(fx, 0x28));
    c.mul_s(0, 0, 4);
    c.lwc1(1, at(fx, 0x30));
    c.mul_s(2, 2, 4);
    c.lq(V0, s(0x210));
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 4);
    c.swc1(5, s(0xe8));
    c.sq(V0, s(0x200));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0xf0));
    c.sq(V0, s(0x100));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0xf8));
    c.sw(s(0xe0), 0);
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0xfc));
    c.sw(s(0xe4), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0xf4));
    // Vertex 3 (sp+0x110): ring point 3 + offset.
    vadd(c, s(0x50), s(0x60), s(0x210), false);
    c.lwc1(1, at(fx, 0x2c));
    c.lwc1(2, at(fx, 0x30));
    c.lwc1(3, at(fx, 0x34));
    c.mul_s(1, 1, 4);
    c.lwc1(0, at(fx, 0x28));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 4);
    c.lq(V0, s(0x210));
    c.mul_s(0, 0, 6);
    c.cvt_w_s(7, 1);
    c.swc1(7, s(0x120));
    c.sq(V0, s(0x200));
    c.cvt_w_s(1, 2);
    c.swc1(1, s(0x124));
    c.cvt_w_s(1, 3);
    c.swc1(1, s(0x128));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0x12c));
    c.sq(V0, s(0x130));
    c.swc1(5, s(0x118));
    c.swc1(5, s(0x110));
    c.sw(s(0x114), 0);
    // Vertex 4 (sp+0x140): ring point 2 - offset.
    vadd(c, s(0x40), s(0x60), s(0x210), true);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.mul_s(0, 0, 4);
    c.lwc1(3, at(fx, 0x28));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.swc1(5, s(0x148));
    c.sq(V0, s(0x200));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x150));
    c.sq(V0, s(0x160));
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x15c));
    c.sw(s(0x140), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x154));
    c.swc1(5, s(0x144));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x158));
    // Vertex 5 (sp+0x170): ring point 2 + offset.
    vadd(c, s(0x40), s(0x60), s(0x210), false);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.mul_s(0, 0, 4);
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.cvt_w_s(3, 0);
    c.swc1(3, s(0x180));
    c.sq(V0, s(0x200));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x184));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x188));
    c.lwc1(0, at(fx, 0x28));
    c.sq(V0, s(0x190));
    c.mul_s(0, 0, 6);
    c.swc1(5, s(0x178));
    c.swc1(5, s(0x170));
    c.swc1(5, s(0x174));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0x18c));
    // Vertex 6 (sp+0x1a0): ring point 1 - offset, colour sp+0x70.
    vadd(c, s(0x30), s(0x60), s(0x210), true);
    c.lwc1(0, s(0x74));
    c.lwc1(1, s(0x78));
    c.lwc1(2, s(0x7c));
    c.mul_s(0, 0, 4);
    c.lwc1(3, s(0x70));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.swc1(5, s(0x1a8));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x1b0));
    c.sq(V0, s(0x200));
    c.sq(V0, s(0x1c0));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x1b8));
    c.sw(s(0x1a0), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1b4));
    c.sw(s(0x1a4), 0);
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x1bc));
    // Vertex 7 (sp+0x1d0): ring point 1 + offset.
    vadd(c, s(0x30), s(0x60), s(0x210), false);
    c.lwc1(0, s(0x74));
    c.lwc1(1, s(0x78));
    c.mul_s(0, 0, 4);
    c.lq(V0, s(0x210));
    c.mul_s(1, 1, 4);
    c.sq(V0, s(0x200));
    c.cvt_w_s(2, 0);
    c.swc1(2, s(0x1e0));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1e4));
    c.lwc1(0, s(0x7c));
    c.lwc1(1, s(0x70));
    c.mul_s(0, 0, 4);
    c.sq(V0, s(0x1f0));
    c.mul_s(1, 1, 6);
    c.swc1(5, s(0x1d0));
    c.swc1(5, s(0x1d8));
    c.cvt_w_s(2, 0);
    c.swc1(2, s(0x1e8));
    c.sw(s(0x1d4), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1ec));
    // Draw the strip (a1 = 8, a2 = sp+0x80, a3 = 0), pop.
    c.g[T0][0] = gfx(c);
    draw(c, T0, V0, V1, 0x002e_4200)?;
    pop_record(c, A0);
    Ok(())
}

// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX_Render.

/// `cSpeedBoostStreamerFX` fields.
mod boost {
    /// The rider (`+0x780` model, `+0x89c` bone index, `+0x1a0` direction).
    pub const RIDER: u32 = 0;
    /// Age: drawn from 3.0 on; also the flames' fade length.
    pub const FADE: u32 = 0x4;
    /// Alpha scale.
    pub const ALPHA: u32 = 0x8;
    /// Width scale and v start of the streamers.
    pub const WIDTH: u32 = 0xc;
    pub const V_START: u32 = 0x10;
    /// Number of flame segments (the tri strip), its alpha clamp.
    pub const FLAMES: u32 = 0x14;
    pub const FLAME_CLAMP: u32 = 0x18;
    /// Flame ring buffer (0x1e entries of 0x1c bytes from +0x50) and its
    /// head.
    pub const FLAME_HEAD: u32 = 0x30;
    pub const FLAME_RING: u32 = 0x50;
    /// Texture index (`gfx + 0xf50 + 4*i`).
    pub const TEXTURE: u32 = 0x398;
    /// Newest point index, point count, point pairs.
    pub const FIRST: u32 = 0x39c;
    pub const COUNT: u32 = 0x3a0;
    pub const POINTS: u32 = 0x3ac;
}

const BOOST: Streamer = Streamer { this: S2, points: boost::POINTS, count: boost::COUNT, tmp: 0xfc0, half: 0x7e0 };

/// `*(gp + GRAPHICS_MAN)`: the PS2 graphics manager (`BeginTriStrip`).
const GRAPHICS_MAN: u32 = 0x2a90;
const BEGIN_TRI_STRIP: u32 = 0x0037_a260;
const END_TRI_STRIP: u32 = 0x0037_a430;

/// `void cSpeedBoostStreamerFX_Render(cSpeedBoostStreamerFX *fx)`
/// (0x2e7a10): when the effect has points, is at least 3.0 old,
/// its alpha is above the global minimum (gp-0x39d8) and boosts are shown
/// (`gp+0x1634`), push a GS state (texture `gfx+0xf50+4*fx->texture`),
/// build the two streamer strips (point pairs offset along the rider's
/// bone, widened along its direction, faded by `1 - (i/count)²`; drawn when
/// they have at least 4 vertices) and, with at least two flame segments,
/// one flame strip written straight into a `BeginTriStrip` packet (ring
/// entries `(head + i) % 0x1e`, alpha clamped to [0, 1]); pop the state.
///
/// Frame 0x1090: s0 +0x1080, s1 +0x1070, s2 +0x1060, s3 +0x1050, ra
/// +0x1040.
pub fn c_speed_boost_streamer_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 10;
        let frame = Frame::open(c, 0x1090);
        frame.save(c, S2, 0x1060);
        frame.save(c, S0, 0x1080);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x1070);
        frame.save(c, S3, 0x1050);
        frame.save_ra(c, 0x1040);
        let fx = c.g[S2][0];
        c.g[V0][0] = c.lw(at(fx, boost::COUNT));
        let empty = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x1080);
        // How far the epilogue's restores start: 0 = s1, s2, s3; 1 = s2,
        // s3; 2 = s3.
        let restored = if empty {
            0
        } else {
            c.cycles += 7;
            c.lwc1(1, at(fx, boost::FADE));
            c.g[AT][0] = 0x4040_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(1, 0);
            let fading = c.fcc();
            frame.restore(c, S1, 0x1070);
            if fading {
                1
            } else {
                c.cycles += 6;
                c.lwc1(1, at(fx, boost::ALPHA));
                c.lwc1(0, gp.wrapping_sub(0x39d8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    frame.restore(c, S2, 0x1060);
                    2
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp.wrapping_add(0x1634));
                    let hidden = c.g[V0][0] == 0;
                    c.g[V1][0] = gfx(c);
                    if hidden {
                        1
                    } else {
                        boost_draw(c, frame)?;
                        0
                    }
                }
            }
        };
        if restored == 0 {
            c.cycles += 1;
            frame.restore(c, S1, 0x1070);
        }
        if restored <= 1 {
            c.cycles += 1;
            frame.restore(c, S2, 0x1060);
        }
        c.cycles += 4;
        frame.restore(c, S3, 0x1050);
        frame.restore_ra(c, 0x1040);
        frame.close(c);
        Ok(())
    })
}

fn boost_draw(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    c.cycles += 96;
    // Push the GS state.
    let g = c.g[V1][0];
    c.g[A1][0] = 0x50_0000;
    c.g[T5][0] = 0xffff_ffff_ffff_fff3;
    c.g[A3][0] = 0xffff_ffff_ffbf_ffff;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T6][0] = 0x40_0000;
    c.g[T1][0] = 0xffff_ffff_fe7f_ffff;
    copy_record(c, top as u32, at(top, RECORD), A0, A2, T0);
    c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T3][0] = 0xffff_ffff_fff0_0fff;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    c.g[T0][0] = 0x1_4000;
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    c.g[T7][0] = 0xffff_ffff_ffff_fffc;
    c.g[T4][0] = 0xffff_ffff_e000_03ff;
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V0][0] = top;
    c.g[S0][0] = 0xffff_ffff_ffff_fc1f;
    c.g[S1][0] = 0xffff_ffff_ffff_ff83;
    c.g[A2][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, S3, V1, A0);
    c.g[T8][0] = 0x29;
    c.g[T9][0] = u64::MAX;
    // Texture `gfx + 0xf50 + 4 * fx->texture`.
    let index = c.lw(at(c.g[S2][0], boost::TEXTURE));
    let g = gfx(c);
    c.g[A0][0] = g;
    c.g[V0][0] = sx((g as u32).wrapping_add((index as u32) << 2));
    let top = c.lw(at(g, GS_TOP));
    c.g[A1][0] = top;
    let tex = c.lw(at(c.g[V0][0], 0xf50));
    c.g[V1][0] = tex;
    c.sh(at(top, 0x10), tex);
    let g = gfx(c);
    c.g[A2][0] = g;
    gs_field(c, g, V1, 0, 0xffff_ffff_ffff_fff3, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    gs_field(c, g, V1, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    // gcc's empty constructor loops over the 2 x 42 vertices.
    loop {
        c.cycles += 7;
        addiu(c, T8, T8, u32::MAX);
        let more = c.g[T8][0] != c.g[T9][0];
        c.g[V0][0] = 0x29;
        if !more {
            break;
        }
        back_edge(c, 0x002e_7bf8);
    }
    c.cycles += 3;
    c.g[A0][0] = c.lw(at(c.g[S2][0], boost::COUNT));
    c.g[A2][0] = c.lw(at(c.g[S2][0], boost::RIDER));
    c.g[V1][0] = u64::MAX;
    loop {
        c.cycles += 7;
        addiu(c, V0, V0, u32::MAX);
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[A3][0] = 0;
        if !more {
            break;
        }
        back_edge(c, 0x002e_7c20);
    }
    // Colour 1.0, the offset along the rider's bone, 1/(count-1)² and
    // 1/count; the half width along the rider's direction.
    c.cycles += 49;
    let t = sp.wrapping_add(BOOST.tmp);
    let rider = c.g[A2][0];
    c.g[A1][0] = c.lw(at(rider, 0x780));
    addiu(c, A0, A0, u32::MAX);
    c.g[V0][0] = c.lw(at(rider, 0x89c));
    c.mult(0, A0, A0, A0);
    c.g[V1][0] = c.lw(at(c.g[A1][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.lwc1(1, gp.wrapping_sub(0x39d4));
    let bone = r32(c, V1);
    c.lq(V0, bone.wrapping_add(0x30));
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.g[S0][0] = 0;
    c.sq(V0, t.wrapping_add(0x10));
    vmul_bc(c, V1, 1, t.wrapping_add(0x10), t.wrapping_add(0x20));
    c.div_s(9, 3, 0);
    c.lq(V0, t.wrapping_add(0x20));
    c.lwc1(0, at(c.g[S2][0], boost::COUNT));
    c.cvt_s_w(0, 0);
    c.swc1(3, t.wrapping_add(0x10));
    c.sq(V0, t);
    c.swc1(3, t.wrapping_add(0x14));
    c.div_s(8, 3, 0);
    c.swc1(3, t.wrapping_add(0x18));
    c.swc1(3, t.wrapping_add(0x1c));
    c.mtc1(ZERO, 7);
    let rider = c.lw(at(c.g[S2][0], boost::RIDER));
    c.g[V1][0] = rider;
    c.lwc1(0, at(c.g[S2][0], boost::WIDTH));
    c.lq(V0, at(rider, 0x1a0));
    c.lwc1(4, at(c.g[S2][0], boost::V_START));
    c.mul_s(0, 0, 2);
    c.sq(V0, t.wrapping_add(0x40));
    vmul_bc(c, V1, 0, t.wrapping_add(0x40), t.wrapping_add(0x60));
    c.lq(V0, t.wrapping_add(0x60));
    c.g[V1][0] = c.lw(at(c.g[S2][0], boost::COUNT));
    c.sq(V0, t.wrapping_add(0x50));
    let none = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = c.lw(at(c.g[S2][0], boost::FIRST));
    if !none {
        c.cycles += 6;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 2);
        c.g[T0][0] = 0x13;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 6);
        c.g[A1][0] = sx(sp.wrapping_add(BOOST.half));
        loop {
            // alpha = fx->alpha * (1 - f7² / (count-1)²), clamped at 0.
            c.cycles += 13;
            c.mul_s(0, 7, 7);
            c.lwc1(1, at(c.g[S2][0], boost::ALPHA));
            c.mtc1(ZERO, 5);
            c.g[V0][0] = (-1i64 < (c.g[A2][0] as i64)) as u64;
            c.mul_s(0, 0, 9);
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T0][0];
            }
            c.sub_s(0, 3, 0);
            c.mul_s(1, 1, 0);
            c.c_lt_s(1, 5);
            let clamp = c.fcc();
            c.swc1(1, t.wrapping_add(0x10));
            if clamp {
                c.cycles += 1;
                c.swc1(5, t.wrapping_add(0x10));
            }
            if !streamer_segment(c, &BOOST) {
                break;
            }
            back_edge(c, 0x002e_7d18);
        }
    }
    // The two streamer strips, when they have at least 4 vertices.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 19;
        c.g[T0][0] = gfx(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        draw(c, T0, V0, V1, 0x002e_7fa4)?;
        c.g[T0][0] = gfx(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp.wrapping_add(BOOST.half));
        c.g[A3][0] = 0;
        draw(c, T0, V0, V1, 0x002e_7fc8)?;
    }
    c.g[A1][0] = c.lw(at(c.g[S2][0], boost::FLAMES));
    // The flame strip.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
    let few = c.g[V0][0] != 0;
    c.g[V1][0] = gfx(c);
    if !few {
        c.cycles += 12;
        let gm = c.lw(gp.wrapping_add(GRAPHICS_MAN));
        c.g[S0][0] = gm;
        c.g[A1][0] = sx(r32(c, A1) << 1);
        c.g[A0][0] = gm;
        call_ptr(c, BEGIN_TRI_STRIP, 0x002e_7fe8)?;
        boost_flames(c, sp);
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, END_TRI_STRIP, 0x002e_81bc)?;
        c.g[V1][0] = gfx(c);
    }
    c.cycles += 4;
    let g = c.g[V1][0];
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = sx((top as u32).wrapping_sub(RECORD));
    let v0 = c.g[V0][0];
    c.sw(at(g, GS_TOP), v0);
    frame.restore(c, S0, 0x1080);
    Ok(())
}

/// The flame strip's vertices, two per ring entry, written from the packet
/// pointer `BeginTriStrip` returned (v0): colour 255 with alpha
/// `clamp(fx->alpha * (1 - (f8 / min(fade, clamp))), 0, 1) * 128`, v from
/// `fx+0x10` plus the entry's v offset, positions from the entry's two
/// points (+4, +0x10).
fn boost_flames(c: &mut Cpu, sp: u32) {
    let fx = c.g[S2][0];
    c.g[V1][0] = c.g[V0][0];
    c.lwc1(0, at(fx, boost::FADE));
    c.lwc1(1, at(fx, boost::FLAME_CLAMP));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(at(fx, boost::FLAMES));
    c.mtc1(ZERO, 8);
    let none = (c.g[V0][0] as i64) <= 0;
    c.min_s(10, 0, 1);
    if none {
        return;
    }
    c.cycles += 14;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(0x30));
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 6);
    c.g[T0][0] = 0x1c;
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 9);
    c.mov_s(5, 8);
    c.g[A3][0] = 0x1e;
    c.g[T2][0] = 0;
    c.g[A2][0] = sx(at(fx, boost::FLAME_RING));
    c.g[T1][0] = 0xff;
    c.lwc1(0, r32(c, A2));
    loop {
        c.cycles += 11;
        c.lwc1(1, at(c.g[S2][0], boost::V_START));
        c.add_s(8, 8, 0);
        c.lwc1(2, at(c.g[S2][0], boost::ALPHA));
        c.div_s(0, 8, 10);
        c.add_s(7, 1, 0);
        c.sub_s(0, 4, 0);
        c.mul_s(2, 2, 0);
        c.c_le_s(5, 2);
        let above = c.fcc();
        c.mov_s(3, 5);
        if above {
            c.cycles += 1;
            c.min_s(3, 2, 4);
        }
        c.cycles += 4;
        c.c_le_s(5, 3);
        let above = c.fcc();
        c.mov_s(0, 5);
        if above {
            c.cycles += 1;
            c.min_s(0, 3, 4);
        }
        // Ring entry (head + i) % 0x1e.
        c.cycles += 12;
        c.g[V0][0] = c.lw(at(c.g[S2][0], boost::FLAME_HEAD));
        addiu(c, A0, A0, 0x30);
        c.swc1(0, sp);
        addiu(c, A2, A2, 0x1c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.swc1(4, sp.wrapping_add(4));
        c.div(0, V0, A3);
        c.swc1(4, sp.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0xc));
        addiu(c, A1, A1, 1);
        c.cycles += 64;
        c.g[V1][0] = c.hi[0];
        c.mult(0, T3, V1, T0);
        c.g[V0][0] = sx(r32(c, T3).wrapping_add(r32(c, S2)));
        c.g[V1][0] = c.g[V0][0];
        addiu(c, V0, V0, 0x54);
        addiu(c, V1, V1, 0x60);
        let (pa, pb) = (r32(c, V0), r32(c, V1));
        // First vertex: point +0x54.
        c.lwc1(0, pa);
        c.swc1(0, sp.wrapping_add(0x10));
        c.lwc1(1, pa.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(0x14));
        c.lwc1(0, pa.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0x1c));
        c.swc1(0, sp.wrapping_add(0x18));
        let v = r32(c, A0);
        let t1 = c.g[T1][0];
        c.sw(v.wrapping_add(0x10), t1);
        c.lwc1(0, sp.wrapping_add(8));
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x14));
        c.lwc1(1, sp.wrapping_add(0xc));
        c.mul_s(1, 1, 6);
        c.cvt_w_s(0, 1);
        c.swc1(0, v.wrapping_add(0x18));
        c.lwc1(0, sp);
        c.swc1(4, v.wrapping_add(8));
        c.mul_s(0, 0, 9);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x1c));
        c.lq(V0, sp.wrapping_add(0x10));
        c.swc1(5, v);
        c.sq(V0, v.wrapping_add(0x20));
        c.swc1(7, v.wrapping_add(4));
        // Second vertex: point +0x60.
        addiu(c, A0, A0, 0x30);
        let v = r32(c, A0);
        c.lwc1(0, pb);
        c.lwc1(2, sp.wrapping_add(4));
        c.swc1(0, sp.wrapping_add(0x10));
        c.mul_s(2, 2, 6);
        c.lwc1(0, pb.wrapping_add(4));
        c.swc1(0, sp.wrapping_add(0x14));
        c.lwc1(1, pb.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0x1c));
        c.swc1(1, sp.wrapping_add(0x18));
        c.cvt_w_s(0, 2);
        c.swc1(0, v.wrapping_add(0x10));
        c.lwc1(0, sp.wrapping_add(8));
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x14));
        c.lwc1(1, sp.wrapping_add(0xc));
        c.mul_s(1, 1, 6);
        c.cvt_w_s(0, 1);
        c.swc1(0, v.wrapping_add(0x18));
        c.lwc1(0, sp);
        c.mul_s(0, 0, 9);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x1c));
        c.swc1(4, v.wrapping_add(8));
        c.lq(V0, sp.wrapping_add(0x10));
        c.swc1(7, v.wrapping_add(4));
        c.sq(V0, v.wrapping_add(0x20));
        c.swc1(4, v);
        let n = c.lw(at(c.g[S2][0], boost::FLAMES));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (n as i64)) as u64;
        if c.g[V0][0] == 0 {
            return;
        }
        c.lwc1(0, r32(c, A2));
        back_edge(c, 0x002e_8040);
    }
}

// ---------------------------------------------------------------------------
// AttachedEmitterFX_Update.

/// `AttachedEmitterFX` fields.
mod attached {
    /// Set once the bone indices below have been looked up.
    pub const BONES_FOUND: u32 = 0;
    /// Bone index per attachment point (0x19 of them, from +4).
    pub const BONES: u32 = 4;
    /// The ten emitters (0 = unused), from +0x6c.
    pub const EMITTERS: u32 = 0x6c;
    pub const EMITTERS_END: u32 = 0x28;
    /// The rider model (`+0x30` bone matrices, `+0x140` scale).
    pub const MODEL: u32 = 0x94;
    /// The rider (`+0x6c0` interface, `+0x370` velocity).
    pub const RIDER: u32 = 0x98;
    /// Set when the "random sparkle" attachment (0x19) spawned.
    pub const SPARKLED: u32 = 0xa0;
}

/// `DynamicEmitter` fields the update reads: attachment point, "always
/// on", noise phase, descriptor.
mod emitter {
    pub const POINT: u32 = 0x200;
    pub const ALWAYS: u32 = 0x204;
    pub const PHASE: u32 = 0x208;
    pub const DESC: u32 = 0x20c;
}

/// The global random seed (`seed = (seed * 0x18fcd + 0xe9507c)`, the float
/// `1.0 | seed & 0x7fffff` in [1, 2)).
const RAND_SEED: u32 = 0xa0c;
/// The zero vector with w = 1 (`0x500000 - 0xee0`).
const ORIGIN: u32 = 0x004f_f120;
const DYNAMIC_EMITTER_SPAWN: u32 = 0x0037_10d0;

mod attached_callee {
    use super::{ee, Callee};
    pub const FIND_BONES: Callee = Callee { addr: 0x002f_22c8, f: ee::s09::AttachedEmitterFX_FindBones };
    pub const NOISE: Callee = Callee { addr: 0x002d_1928, f: ee::s09::Noise_Fractal1D };
}

/// `|*src|` on VU0 as the original issues it (`vf4 = vf3 * vf3`, the sum
/// through ACC, `vsqrt`); Q goes through `gpr` to f`f`.
fn vu0_length(c: &mut Cpu, src: u32, gpr: usize, f: usize) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
    c.mtc1(gpr, f);
}

/// The emitter's bone matrix (`model->bones[this->bones[em->point]]`, the
/// emitter in v1, the bone table base in t0) copied to s3 through vf1-vf4,
/// and the origin row (address through `origin_reg`) to sp+0x70.
fn attached_bone(c: &mut Cpu, sp: u32, origin_reg: usize) {
    let model = c.lw(at(c.g[S2][0], attached::MODEL));
    c.g[A0][0] = model;
    c.g[V0][0] = c.lw(at(c.g[V1][0], emitter::POINT));
    c.g[A1][0] = c.lw(at(model, 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    let (m, s3) = (r32(c, V1), r32(c, S3));
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, m.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, s3.wrapping_add(0x10 * i));
    }
    c.g[origin_reg][0] = sx(ORIGIN);
    copy_quad(c, ORIGIN, sp.wrapping_add(0x70));
}

/// `vf12 = M * *p` with M's rows at s3 (vf4-vf7), stored to `out`.
fn attached_transform(c: &mut Cpu, p: u32, out: u32) {
    let s3 = r32(c, S3);
    c.lqc2(8, p);
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, s3.wrapping_add(0x10 * i));
    }
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, out);
}

/// One step of the global random seed (in v0) into v0 as the original
/// does it through registers: `rd = v0 * k`, `v0 = (rd + add) & mask |
/// one` (k = 0x18fcd, add = 0xe9507c, mask = 0x7fffff, one = 1.0f).
fn rand_step(c: &mut Cpu, rd: usize, k: usize, add: usize, mask: usize, one: usize) {
    c.mult(0, rd, V0, k);
    c.g[V0][0] = sx(r32(c, rd).wrapping_add(r32(c, add)));
    c.g[V0][0] &= c.g[mask][0];
    c.g[V0][0] |= c.g[one][0];
}

/// `void AttachedEmitterFX_Update(AttachedEmitterFX *fx)` (0x2f1a08): look
/// up the attachment bones once, then for each of the ten emitters decide
/// whether it spawns this frame (always-on emitters; others only within
/// the descriptor's speed window `+0xfc..+0x100` of the rider's speed;
/// then a random chance `+0x104` and a noise threshold `+0x108` with the
/// phase advancing by `+0x10c`), place the spawn point at the bone plus
/// the descriptor's offset (attachment 0x19 sparkles at a random offset,
/// 0x17/0x18 shift by ±75 in x), scaled by the model's scale and
/// transformed by the bone, and spawn: with a velocity (the rider's
/// direction and velocity weighted by `+0xd8..+0xe0` plus the bone-space
/// direction `+0xf0`) when it fires, else an idle spawn (no velocity).
///
/// Frame 0x180: s0-s7, fp at +0x140..+0xc0, ra +0xb0, f20-f25 at
/// +0x150..+0x178.
pub fn attached_emitter_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let sp = r32(c, SP).wrapping_sub(0x180);
        let gp = r32(c, GP);
        c.cycles += 21;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S2, 0x120);
        frame.save(c, S0, 0x140);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x130);
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        frame.save(c, S5, 0xf0);
        frame.save(c, S6, 0xe0);
        frame.save(c, S7, 0xd0);
        frame.save(c, FP, 0xc0);
        frame.save_ra(c, 0xb0);
        for (f, off) in [(25, 0x178), (24, 0x170), (23, 0x168), (22, 0x160), (21, 0x158), (20, 0x150)] {
            frame.save_fpr(c, f, off);
        }
        let found = c.lw(at(c.g[S2][0], attached::BONES_FOUND));
        c.g[V0][0] = 0x50_0000;
        if found == 0 {
            c.cycles += 3;
            attached_callee::FIND_BONES.call(c, 0x002f_1a64)?;
            c.g[V0][0] = 0x50_0000;
        }
        // The rider's direction (sp+0) and velocity (sp+0x10, sp+0x20).
        c.cycles += 12;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[S0][0] = sx(ORIGIN);
        let rider = c.lw(at(c.g[S2][0], attached::RIDER));
        c.g[A1][0] = rider;
        c.lq(V0, ORIGIN);
        c.sw(at(c.g[S2][0], attached::SPARKLED), 0);
        c.sq(V0, sp);
        c.sw(sp.wrapping_add(0x10), 0);
        c.sw(sp.wrapping_add(0x14), 0);
        c.swc1(20, sp.wrapping_add(0x18));
        c.swc1(20, sp.wrapping_add(0x1c));
        if rider != 0 {
            c.cycles += 12;
            let vt = c.lw(at(rider, 0x6c0));
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(at(rider, 0x6c0));
            c.g[A0][0] = c.lh(at(vt, 0x10));
            let f = c.lw(at(vt, 0x14));
            c.g[V0][0] = f;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, f as u32, 0x002f_1ab0)?;
            c.lq(V1, r32(c, V0));
            c.g[A0][0] = c.lw(at(c.g[S2][0], attached::RIDER));
            c.sq(V1, sp);
            let vel = at(c.g[A0][0], 0x370);
            c.lq(V0, vel);
            c.sq(V0, sp.wrapping_add(0x10));
            c.sq(V0, sp.wrapping_add(0x20));
        }
        // Speed = |velocity|, the random constants, the emitter walk.
        c.cycles += 26;
        vu0_length(c, sp, V0, 21);
        c.g[FP][0] = sx(at(c.g[S2][0], attached::EMITTERS));
        c.g[S6][0] = 0x1_8fcd;
        c.g[S5][0] = 0xe9_507c;
        c.g[S4][0] = 0x7f_ffff;
        c.g[AT][0] = 0xffff_ffff_c2aa_0000;
        c.mtc1(AT, 25);
        c.g[AT][0] = 0xffff_ffff_c160_0000;
        c.mtc1(AT, 24);
        c.mov_s(22, 21);
        c.lwc1(23, gp.wrapping_sub(0x392c));
        c.g[S3][0] = sx(sp.wrapping_add(0x40));
        c.g[S1][0] = c.g[FP][0];
        c.g[V1][0] = c.lw(r32(c, S1));
        loop {
            attached_emitter(c, sp, gp)?;
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, FP).wrapping_add(attached::EMITTERS_END));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = c.lw(r32(c, S1));
            back_edge(c, 0x002f_1b30);
        }
        c.cycles += 18;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        for (f, off) in [(25, 0x178), (24, 0x170), (23, 0x168), (22, 0x160), (21, 0x158), (20, 0x150)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// One emitter (loop body at 0x2f1b30; the emitter in v1, its slot at s1,
/// advanced by 4).
fn attached_emitter(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    c.cycles += 2;
    let em = c.g[V1][0];
    c.g[S7][0] = 0;
    if em == 0 {
        c.cycles += 1;
        addiu(c, S1, S1, 4);
        return Ok(());
    }
    // Does it fire? Always-on, or within the speed window.
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(em, emitter::ALWAYS));
    let always = c.g[V0][0] != 0;
    let desc = c.lw(at(em, emitter::DESC));
    c.g[S0][0] = desc;
    let mut chance_loaded = false;
    if !always {
        c.cycles += 7;
        c.lwc1(0, at(desc, 0xfc));
        c.lwc1(1, gp.wrapping_sub(0x3928));
        c.mul_s(0, 0, 1);
        c.c_le_s(0, 21);
        if !c.fcc() {
            c.lwc1(1, at(desc, 0x104));
            chance_loaded = true;
        } else {
            c.cycles += 6;
            c.lwc1(0, at(desc, 0x100));
            c.mul_s(0, 0, 1);
            c.c_le_s(21, 0);
            if c.fcc() {
                c.g[S7][0] = 1;
            }
        }
    }
    if !chance_loaded {
        c.cycles += 1;
        c.lwc1(1, at(desc, 0x104));
    }
    // The random chance.
    c.cycles += 4;
    c.c_lt_s(1, 23);
    let roll = c.fcc();
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    if roll {
        c.cycles += 12;
        c.g[A0][0] = 0x3f80_0000;
        rand_step(c, V1, S6, S5, S4, A0);
        let seed = c.g[V0][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.lwc1(0, gp.wrapping_add(RAND_SEED));
        c.sub_s(0, 0, 20);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[S7][0] = 0;
        }
    }
    // The noise threshold.
    c.cycles += 5;
    c.lwc1(0, at(desc, 0x108));
    c.c_lt_s(0, 23);
    if !c.fcc() {
        c.g[A2][0] = c.lw(r32(c, S1));
    } else {
        c.cycles += 15;
        c.lwc1(1, at(desc, 0x10c));
        c.lwc1(0, gp.wrapping_sub(0x3924));
        c.g[V0][0] = c.lw(r32(c, S1));
        c.mul_s(1, 1, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(AT, 2);
        let phase = at(c.g[V0][0], emitter::PHASE);
        c.lwc1(0, phase);
        c.add_s(0, 0, 1);
        c.swc1(0, phase);
        c.g[V0][0] = c.lw(r32(c, S1));
        let phase = at(c.g[V0][0], emitter::PHASE);
        c.lwc1(0, phase);
        c.c_le_s(2, 0);
        let wrap = c.fcc();
        c.g[A0][0] = 3;
        if wrap {
            c.cycles += 3;
            c.sub_s(0, 0, 2);
            c.swc1(0, phase);
            c.g[V0][0] = c.lw(r32(c, S1));
        }
        c.cycles += 11;
        c.lwc1(12, at(c.g[V0][0], emitter::PHASE));
        attached_callee::NOISE.call(c, 0x002f_1c20)?;
        c.add_s(0, 0, 20);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 1);
        c.lwc1(2, at(desc, 0x108));
        c.mul_s(0, 0, 1);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.g[S7][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.lw(r32(c, S1));
    }
    // The spawn point: the bone's position (sp+0x20) and the offset
    // (sp+0x30).
    c.cycles += 25;
    addiu(c, A1, S2, attached::BONES);
    let model = c.lw(at(c.g[S2][0], attached::MODEL));
    c.g[V1][0] = model;
    c.g[T0][0] = c.g[A1][0];
    c.g[V0][0] = c.lw(at(c.g[A2][0], emitter::POINT));
    c.g[A3][0] = 0x19;
    c.g[A0][0] = c.lw(at(model, 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 6);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, A1));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    copy_quad(c, bone.wrapping_add(0x30), s(0x20));
    c.lwc1(4, at(desc, 0xe4));
    c.lwc1(5, at(desc, 0xec));
    c.lwc1(0, at(desc, 0xe8));
    c.swc1(4, s(0x30));
    c.swc1(0, s(0x34));
    c.swc1(5, s(0x38));
    c.swc1(6, s(0x3c));
    c.g[V1][0] = c.lw(at(c.g[A2][0], emitter::POINT));
    c.g[V0][0] = 0x17;
    if c.g[V1][0] == c.g[A3][0] {
        // Attachment 0x19: a random offset (x += 170 * r - 85, z += 28 * r
        // - 14), and the sparkle flag.
        c.cycles += 32;
        c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
        c.g[V1][0] = 1;
        c.sw(at(c.g[S2][0], attached::SPARKLED), 1);
        c.g[A0][0] = 0x3f80_0000;
        c.mult(0, V1, V0, S6);
        c.g[AT][0] = 0x432a_0000;
        c.mtc1(AT, 2);
        c.g[AT][0] = 0x41e0_0000;
        c.mtc1(AT, 3);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
        c.g[V0][0] &= c.g[S4][0];
        c.g[V0][0] |= c.g[A0][0];
        c.mult(0, A0, V0, S6);
        let seed = c.g[V0][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.g[V0][0] = 0x3f80_0000;
        c.lwc1(1, gp.wrapping_add(RAND_SEED));
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
        c.sub_s(1, 1, 6);
        c.g[V1][0] &= c.g[S4][0];
        c.g[V1][0] |= c.g[V0][0];
        let seed = c.g[V1][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.mul_s(1, 1, 2);
        c.lwc1(0, gp.wrapping_add(RAND_SEED));
        c.add_s(1, 1, 25);
        c.sub_s(0, 0, 6);
        c.add_s(1, 4, 1);
        c.mul_s(0, 0, 3);
        c.swc1(1, s(0x30));
        c.add_s(0, 0, 24);
        c.add_s(0, 5, 0);
        c.swc1(0, s(0x38));
        c.cycles += 1;
        c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
    } else {
        // Attachments 0x17 / 0x18: x + 75 / x - 75.
        c.cycles += 2;
        let left = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x18;
        let mut shifted = true;
        if left {
            c.cycles += 4;
            c.g[AT][0] = 0x4296_0000;
            c.mtc1(AT, 0);
            c.add_s(0, 4, 0);
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
                shifted = false;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x4296_0000;
                c.mtc1(AT, 0);
                c.sub_s(0, 4, 0);
            }
        }
        if shifted {
            c.cycles += 1;
            c.swc1(0, s(0x30));
            c.cycles += 1;
            c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
        }
    }
    // Scale the offset by the model's scale; unless it is zero, transform
    // it by the bone and add it to the position.
    c.cycles += 15;
    let model = c.g[V0][0];
    c.lwc1(1, s(0x30));
    c.lwc1(0, at(model, 0x140));
    c.lwc1(2, s(0x34));
    c.mul_s(1, 1, 0);
    c.lwc1(3, s(0x38));
    c.mtc1(ZERO, 4);
    c.swc1(1, s(0x30));
    c.c_eq_s(1, 4);
    c.lwc1(0, at(model, 0x144));
    c.mul_s(2, 2, 0);
    c.swc1(2, s(0x34));
    c.lwc1(0, at(model, 0x148));
    c.mul_s(3, 3, 0);
    let x_zero = c.fcc();
    c.swc1(3, s(0x38));
    let mut transform = true;
    let mut reload = true;
    if x_zero {
        c.cycles += 4;
        c.c_eq_s(2, 4);
        if !c.fcc() {
            c.g[V1][0] = c.lw(r32(c, S1));
            reload = false;
        } else {
            c.cycles += 3;
            c.c_eq_s(3, 4);
            transform = !c.fcc();
        }
    }
    if transform {
        if reload {
            c.cycles += 1;
            c.g[V1][0] = c.lw(r32(c, S1));
        }
        c.cycles += 37;
        attached_bone(c, sp, V1);
        attached_transform(c, s(0x30), s(0x90));
        copy_quad(c, s(0x90), s(0x30));
        c.sq(V0, s(0x80));
        vadd(c, s(0x20), s(0x30), s(0x20), false);
    }
    c.cycles += 2;
    if c.g[S7][0] == 0 {
        // Idle spawn: no velocity.
        c.g[A0][0] = c.lw(r32(c, S1));
        c.cycles += 5;
        c.g[A1][0] = sx(s(0x20));
        c.lwc1(12, gp.wrapping_sub(0x391c));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, DYNAMIC_EMITTER_SPAWN, 0x002f_202c)?;
        c.cycles += 1;
        addiu(c, S1, S1, 4);
        return Ok(());
    }
    // Fire: velocity = dir * d8 + vel * dc + vel * e0 * speed (sp+0x30),
    // plus the descriptor's direction in bone space.
    c.cycles += 49;
    c.lwc1(0, at(desc, 0xd8));
    vmul_bc(c, V0, 0, s(0), s(0x60));
    c.mtc1(ZERO, 1);
    copy_quad(c, s(0x60), s(0x50));
    c.lwc1(0, at(desc, 0xdc));
    vmul_bc(c, V1, 0, s(0x10), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x50), s(0x60), s(0x70), false);
    copy_quad(c, s(0x70), s(0x40));
    c.lwc1(0, at(desc, 0xe0));
    vmul_bc(c, V1, 0, s(0x10), s(0x90));
    copy_quad(c, s(0x90), s(0x80));
    vmul_bc(c, V1, 22, s(0x80), s(0x90));
    copy_quad(c, s(0x90), s(0x70));
    vadd(c, s(0x40), s(0x70), s(0x90), false);
    copy_quad(c, s(0x90), s(0x30));
    // Unless the direction (+0xf0) is zero, add it in bone space.
    c.lwc1(0, at(desc, 0xf0));
    c.c_eq_s(0, 1);
    let mut turn = true;
    let mut reload = false;
    if c.fcc() {
        c.cycles += 5;
        c.lwc1(0, at(desc, 0xf4));
        c.c_eq_s(0, 1);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, at(desc, 0xf8));
            c.c_eq_s(0, 1);
            if c.fcc() {
                c.g[A0][0] = c.lw(r32(c, S1));
                turn = false;
            } else {
                reload = true;
            }
        }
    }
    if turn {
        if reload {
            c.cycles += 1;
        }
        c.g[V1][0] = c.lw(r32(c, S1));
        c.cycles += 45;
        attached_bone(c, sp, A0);
        c.lwc1(2, at(desc, 0xf8));
        c.lwc1(0, at(desc, 0xf0));
        c.lwc1(1, at(desc, 0xf4));
        c.swc1(0, s(0x80));
        c.swc1(1, s(0x84));
        c.swc1(2, s(0x88));
        c.swc1(20, s(0x8c));
        attached_transform(c, s(0x80), s(0xa0));
        copy_quad(c, s(0xa0), s(0x80));
        c.sq(V0, s(0x90));
        vadd(c, s(0x30), s(0x80), s(0x30), false);
        c.g[A0][0] = c.lw(r32(c, S1));
    }
    c.cycles += 7;
    c.g[A1][0] = sx(s(0x20));
    c.lwc1(12, gp.wrapping_sub(0x3920));
    c.g[A2][0] = sx(s(0x30));
    c.g[A3][0] = 1;
    call_ptr(c, DYNAMIC_EMITTER_SPAWN, 0x002f_2010)?;
    addiu(c, S1, S1, 4);
    Ok(())
}

// ---------------------------------------------------------------------------
// cBoardSparkFX_Update.

/// `cBoardSparkFX` fields.
mod spark {
    /// The rider (`+0x438` player index / character, `+0x1e0` velocity,
    /// `+0x460` position, `+0x1a0` direction, `+0x2f0` speed).
    pub const RIDER: u32 = 0;
    /// The spark source point (written each frame).
    pub const SOURCE: u32 = 0x10;
    /// The dynamic emitter the sparks spawn from, and the argument of its
    /// restart method.
    pub const EMITTER: u32 = 0x20;
    pub const RESTART_ARG: u32 = 0x24;
    /// The particle descriptor (`Emitter_*`).
    pub const DESC: u32 = 0x28;
    /// The burst's three points and random scales (written).
    pub const BURST_POINTS: u32 = 0x30;
    pub const BURST_SCALES: u32 = 0x60;
    /// Restart the emitter this frame.
    pub const RESTART: u32 = 0x6c;
    /// Spraying (the player is carving), bursting (landed carving), grinding.
    pub const SPRAY: u32 = 0x70;
    pub const BURST: u32 = 0x74;
    pub const GRIND: u32 = 0x78;
}

/// The character id that always grinds sparks.
const SPARK_CHARACTER: u64 = 9;
/// Board colour table: 0xf0 bytes per board (`0x500000 - 0x5c68`).
const BOARD_COLOURS: u32 = 0x004f_a398;
const EMITTER_UPDATE_DESCRIPTOR: u32 = 0x0036_d428;
const EMITTER_SET_VELOCITY_CURVE: u32 = 0x0036_d318;
const EMITTER_SET_PATH_SCALED: u32 = 0x0036_d1f0;
const EMITTER_SET_PATH: u32 = 0x0036_cef8;
const DYNAMIC_EMITTER_SPAWN_AT: u32 = 0x0037_17c0;
const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

/// `void cBoardSparkFX_Update(cBoardSparkFX *fx)` (0x2dabc8): advance the
/// descriptor by the frame time, restart the emitter when asked; from the
/// rider's speed, main state (4) and the player record's carve/grind
/// switches decide spraying, bursting and grinding; spawn one spark at the
/// board (grinding: with a velocity from the rider's direction and speed
/// and the board's colour, else an idle spark); while spraying set up the
/// descriptor's velocity curve and path from the rider's bone axes
/// (hot sparks when boosting or fast); when bursting, roll three random
/// burst points around the source.
///
/// Frame 0x130: s0-s7 at +0x100..+0x90, ra +0x80, f20-f22 at +0x110..+0x120.
pub fn c_board_spark_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        let sp = r32(c, SP).wrapping_sub(0x130);
        let s = |off: u32| sp.wrapping_add(off);
        c.cycles += 33;
        let frame = Frame::open(c, 0x130);
        let clock = c.lw(gp.wrapping_add(0x2a74));
        c.g[V0][0] = clock;
        frame.save(c, S1, 0xf0);
        c.g[A2][0] = 0xb0;
        frame.save(c, S2, 0xe0);
        frame.save(c, S0, 0x100);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 22, 0x120);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        let fx = c.g[S2][0];
        // dt = 1 / frame rate; the player record (0xb0 bytes each).
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[A0][0] = rider;
        c.lwc1(0, at(clock, 0x10));
        c.cvt_s_w(0, 0);
        let app = c.lw(gp.wrapping_sub(APP));
        c.g[V0][0] = app;
        c.g[V1][0] = c.lw(at(rider, 0x438));
        c.div_s(12, 12, 0);
        c.g[A1][0] = c.lw(at(app, 0x84));
        c.mult(0, V1, V1, A2);
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[V0][0] = c.lw(at(c.g[A1][0], 0x44));
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        call_ptr(c, EMITTER_UPDATE_DESCRIPTOR, 0x002d_ac40)?;
        c.g[V0][0] = c.lw(at(fx, spark::RESTART));
        if c.g[V0][0] != 0 {
            // emitter->restart(arg, emitter->rate) (method 0x18 of +0x1f8).
            c.cycles += 11;
            let em = c.lw(at(fx, spark::EMITTER));
            c.g[A0][0] = em;
            c.g[A1][0] = c.lw(at(fx, spark::RESTART_ARG));
            let vt = c.lw(at(em, 0x1f8));
            c.g[V0][0] = vt;
            c.lwc1(12, at(em, 0x58));
            c.g[V1][0] = c.lh(at(vt, 0x18));
            let f = c.lw(at(vt, 0x1c));
            c.g[A2][0] = f;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            call_ptr(c, f as u32, 0x002d_ac6c)?;
            c.g[V0][0] = 1;
            c.sw(at(fx, spark::RESTART), 1);
        }
        c.sw(at(fx, spark::SPRAY), 0);
        // Fast enough? (|velocity| > gp-0x3c94)
        c.cycles += 19;
        c.g[S0][0] = 1;
        c.sw(at(fx, spark::BURST), 0);
        c.sw(at(fx, spark::GRIND), 0);
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[A0][0] = rider;
        c.lwc1(0, gp.wrapping_sub(0x3c94));
        vu0_length(c, at(rider, 0x1e0), V0, 20);
        c.c_lt_s(0, 20);
        let fast = c.fcc();
        c.mov_s(22, 20);
        if !fast {
            c.cycles += 1;
            c.g[S0][0] = 0;
        }
        // Main state 4 (s7); the player record's switches.
        c.cycles += 5;
        call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_acd0)?;
        c.g[V0][0] ^= 4;
        let moving = c.g[S0][0] != 0;
        c.g[S7][0] = (c.g[V0][0] < 1) as u64;
        let mut reload = true;
        if moving {
            c.cycles += 3;
            let carving = c.lw(at(c.g[S1][0], 0x8c));
            c.g[V0][0] = 1;
            let mut check_grind = true;
            let mut check_char = false;
            if carving != 0 {
                c.cycles += 2;
                let plain = c.g[S7][0] == 0;
                c.sw(at(fx, spark::SPRAY), 1);
                if plain {
                    check_grind = false;
                    check_char = true;
                } else {
                    c.cycles += 1;
                    c.sw(at(fx, spark::BURST), 1);
                }
            }
            let mut grind = false;
            let mut char_loaded = false;
            if check_grind {
                c.cycles += 2;
                if c.g[S7][0] == 0 {
                    c.g[V0][0] = c.lw(at(fx, spark::RIDER));
                    char_loaded = true;
                } else {
                    c.cycles += 3;
                    let rail = c.lw(at(c.g[S1][0], 0x88));
                    c.g[V0][0] = 1;
                    if rail != 0 {
                        grind = true;
                    } else {
                        check_char = true;
                    }
                }
            }
            if check_char {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(fx, spark::RIDER));
                char_loaded = true;
            }
            if char_loaded {
                c.cycles += 4;
                c.g[A0][0] = SPARK_CHARACTER;
                c.g[V1][0] = c.lw(at(c.g[V0][0], 0x438));
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[V1][0] = c.lw(at(fx, spark::RIDER));
                    reload = false;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    grind = true;
                }
            }
            if grind {
                c.cycles += 1;
                let v0 = c.g[V0][0];
                c.sw(at(fx, spark::GRIND), v0);
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(fx, spark::RIDER));
        }
        // The source: the rider's position + velocity * gp-0x3c90.
        c.cycles += 18;
        let rider = c.g[V1][0];
        c.lwc1(0, gp.wrapping_sub(0x3c90));
        vmul_bc(c, A0, 0, at(rider, 0x1e0), s(0x20));
        c.lq(V0, s(0x20));
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[V1][0] = rider;
        c.sq(V0, s(0x10));
        vadd(c, at(rider, 0x460), s(0x10), s(0x20), false);
        c.lq(V0, s(0x20));
        c.g[V1][0] = c.lw(at(fx, spark::GRIND));
        c.sq(V0, at(fx, spark::SOURCE));
        c.sq(V0, sp);
        if c.g[V1][0] != 0 {
            spark_grind(c, sp, gp)?;
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.lw(at(fx, spark::EMITTER));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(12, gp.wrapping_sub(0x3c68));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002d_afe8)?;
        }
        c.g[V0][0] = c.lw(at(fx, spark::SPRAY));
        // Spraying: the descriptor's velocity curve and path.
        c.cycles += 2;
        let spraying = c.g[V0][0] != 0;
        c.g[S3][0] = sx(s(0x10));
        if spraying {
            spark_spray(c, sp, gp)?;
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = c.lw(at(fx, spark::BURST));
        // Bursting: three random points around the source.
        c.cycles += 2;
        let bursting = c.g[V0][0] != 0;
        c.g[V1][0] = c.lw(gp.wrapping_add(RAND_SEED));
        if !(bursting && spark_burst(c, sp, gp)) {
            c.cycles += 2;
            c.sw(at(fx, spark::BURST), 0);
        }
        frame.restore(c, S0, 0x100);
        c.cycles += 13;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0x120);
        frame.restore_fpr(c, 21, 0x118);
        frame.restore_fpr(c, 20, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The grinding spark: the board colour (character 9: the constant colour
/// at gp-0x3c88, else the colour of the board the rider's method 0x38
/// names, doubled with alpha 1) at sp+0x10, the velocity along the rider's
/// direction (sp+0x30), a random size, spawned at the source.
fn spark_grind(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    c.cycles += 6;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[A1][0] = rider;
    c.g[V1][0] = SPARK_CHARACTER;
    c.lwc1(21, gp.wrapping_sub(0x3c8c));
    c.g[V0][0] = c.lw(at(rider, 0x438));
    if c.g[V0][0] != c.g[V1][0] {
        let vt = c.lw(at(rider, 0x6c0));
        c.g[V1][0] = vt;
        c.cycles += 38;
        c.g[A1][0] = sx(at(rider, 0x6c0));
        c.g[A0][0] = c.lh(at(vt, 0x38));
        let f = c.lw(at(vt, 0x3c));
        c.g[V0][0] = f;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, f as u32, 0x002d_add0)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x50_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(BOARD_COLOURS);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(BOARD_COLOURS));
        let colour = r32(c, V0);
        c.ldl(V1, colour.wrapping_add(7));
        c.ldr(V1, colour);
        c.ldl(A0, colour.wrapping_add(0xf));
        c.ldr(A0, colour.wrapping_add(8));
        c.sdl(V1, s(0x27));
        c.sdr(V1, s(0x20));
        c.sdl(A0, s(0x2f));
        c.sdr(A0, s(0x28));
        // (a, r, g, b) -> (r, g, b, a).
        c.lwc1(0, s(0x24));
        c.lwc1(1, s(0x28));
        c.lwc1(2, s(0x2c));
        c.lwc1(3, s(0x20));
        c.swc1(0, s(0x30));
        c.swc1(1, s(0x34));
        c.swc1(2, s(0x38));
        c.swc1(3, s(0x3c));
        copy_quad(c, s(0x30), s(0x10));
        vmul_bc(c, V1, 4, s(0x10), s(0x10));
        c.swc1(5, s(0x1c));
    } else {
        c.cycles += 13;
        c.lwc1(0, gp.wrapping_sub(0x3c88));
        c.lwc1(1, gp.wrapping_sub(0x3c84));
        c.lwc1(2, gp.wrapping_sub(0x3c80));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        c.swc1(0, s(0x20));
        c.swc1(1, s(0x24));
        c.swc1(2, s(0x28));
        c.swc1(3, s(0x2c));
        c.lwc1(21, gp.wrapping_sub(0x3c7c));
        copy_quad(c, s(0x20), s(0x10));
    }
    // The velocity: direction * speed * gp-0x3c78 ... plus a random
    // sideways part; the spark size f3 (gp-0x3c8c or the constant).
    c.cycles += 72;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[V1][0] = rider;
    c.g[A3][0] = 0x1_8fcd;
    c.g[AT][0] = 0x3e80_0000;
    c.mtc1(AT, 1);
    c.lq(V0, at(rider, 0x1a0));
    c.g[A2][0] = 0xe9_507c;
    c.lwc1(2, gp.wrapping_sub(0x3c78));
    c.sq(V0, s(0x40));
    c.g[A1][0] = 0x7f_ffff;
    vmul_bc(c, V0, 22, s(0x40), s(0x50));
    c.lwc1(0, gp.wrapping_sub(0x3c74));
    c.lq(V0, s(0x50));
    c.div_s(0, 20, 0);
    c.g[T1][0] = 0x3f80_0000;
    c.sq(V0, s(0x30));
    c.mov_s(3, 21);
    vmul_bc(c, V1, 1, s(0x30), s(0x50));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.lq(V0, s(0x50));
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[V1][0] = rider;
    c.sq(V0, s(0x20));
    vmul_bc(c, A0, 2, at(rider, 0x1e0), s(0x60));
    c.lwc1(1, gp.wrapping_sub(0x3c70));
    c.lq(V0, s(0x60));
    c.mul_s(1, 0, 1);
    c.sq(V0, s(0x50));
    vadd(c, s(0x50), s(0x20), s(0x60), false);
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    c.c_lt_s(1, 21);
    c.lq(V1, s(0x60));
    c.mult(0, V0, V0, A3);
    c.sq(V1, s(0x40));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[T1][0];
    let seed = c.g[V0][0];
    c.sw(gp.wrapping_add(RAND_SEED), seed);
    c.lwc1(0, gp.wrapping_add(RAND_SEED));
    c.sub_s(0, 0, 4);
    c.add_s(0, 0, 0);
    vmul_bc(c, V1, 0, s(0x20), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x40), s(0x60), s(0x70), true);
    c.lq(V0, s(0x70));
    let small = c.fcc();
    c.sq(V0, s(0x30));
    if small {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    // A random "hot" flag: (random - 1) < size.
    c.cycles += 12;
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    rand_step(c, V1, A3, A2, A1, T1);
    let seed = c.g[V0][0];
    c.sw(gp.wrapping_add(RAND_SEED), seed);
    c.lwc1(0, gp.wrapping_add(RAND_SEED));
    c.sub_s(0, 0, 4);
    c.c_lt_s(0, 3);
    let hot = c.fcc();
    c.g[T0][0] = 1;
    if !hot {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 8;
    c.g[A0][0] = c.lw(at(fx, spark::EMITTER));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(12, gp.wrapping_sub(0x3c6c));
    c.g[A2][0] = sx(s(0x30));
    c.g[A3][0] = sx(s(0x10));
    call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002d_afc4)
}

/// Spraying: the bone's axes at sp+0x10 (s3) / sp+0x20 (s4); the
/// descriptor's velocity curve from two colours (hot: (255, 0, 255, 153)
/// -> (255, 0, 255, 102); else (77, 77, 255, 153) -> (255, 51, 51, 102)),
/// the scaled path (400 / 600) and the path along the bone (x10, or x15
/// and x90 when not in state 4).
fn spark_spray(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    c.cycles += 19;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[A3][0] = rider;
    c.g[A2][0] = c.lw(gp.wrapping_add(0x163c));
    c.g[S4][0] = sx(s(0x20));
    c.g[V0][0] = c.lw(at(rider, 0x780));
    c.g[T0][0] = 0;
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[V0][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    copy_quad(c, bone.wrapping_add(0x20), s(0x10));
    c.g[A1][0] = c.lw(at(rider, 0x780));
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[A1][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    c.lq(V0, bone);
    let boosting = c.g[A2][0] != 0;
    c.sq(V0, s(0x20));
    let hot = if boosting {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0x1638));
        if c.g[V0][0] != 0 {
            c.g[T0][0] = 1;
            false
        } else {
            c.cycles += 5;
            c.lwc1(1, at(rider, 0x2f0));
            c.mtc1(ZERO, 0);
            c.c_lt_s(0, 1);
            c.fcc()
        }
    };
    if hot {
        c.cycles += 1;
        c.g[T0][0] = 1;
    }
    c.cycles += 2;
    let plain = c.g[T0][0] == 0;
    c.g[V0][0] = sx(s(0x30));
    if !plain {
        c.cycles += 37;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 0);
        c.g[AT][0] = 0x4319_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx(s(0x40));
        c.g[AT][0] = 0x42cc_0000;
        c.mtc1(AT, 2);
        c.g[S0][0] = sx(s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[S1][0] = sx(s(0x60));
        c.lwc1(12, gp.wrapping_sub(0x3c64));
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.swc1(1, s(0x3c));
        c.g[A2][0] = c.g[V1][0];
        c.swc1(0, s(0x48));
        c.g[S5][0] = c.g[V0][0];
        c.swc1(2, s(0x4c));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(0, s(0x30));
        c.sw(s(0x34), 0);
        c.swc1(0, s(0x38));
        c.swc1(0, s(0x40));
        c.sw(s(0x44), 0);
        for off in (0x50..0x70).step_by(4) {
            c.sw(s(off), 0);
        }
        call_ptr(c, EMITTER_SET_VELOCITY_CURVE, 0x002d_b0f4)?;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
    } else {
        c.cycles += 40;
        c.g[AT][0] = 0x429a_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 3);
        c.g[V1][0] = sx(s(0x40));
        c.g[AT][0] = 0x424c_0000;
        c.mtc1(AT, 1);
        c.g[S1][0] = sx(s(0x50));
        c.g[AT][0] = 0x4319_0000;
        c.mtc1(AT, 0);
        c.g[S0][0] = sx(s(0x60));
        c.g[AT][0] = 0x42cc_0000;
        c.mtc1(AT, 2);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[T0][0] = c.g[S0][0];
        c.lwc1(12, gp.wrapping_sub(0x3c60));
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[V1][0];
        c.swc1(4, s(0x34));
        c.swc1(0, s(0x3c));
        c.g[S5][0] = c.g[V0][0];
        c.swc1(3, s(0x40));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(1, s(0x48));
        c.swc1(2, s(0x4c));
        c.swc1(4, s(0x30));
        c.swc1(3, s(0x38));
        c.swc1(1, s(0x44));
        for off in (0x50..0x70).step_by(4) {
            c.sw(s(off), 0);
        }
        call_ptr(c, EMITTER_SET_VELOCITY_CURVE, 0x002d_b198)?;
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
    }
    // The scaled path: (0, 0, 400, 0), (600, 0, 0, 0), (0, 600, 0, 0),
    // (0, 0, 600, 0).
    c.cycles += 26;
    c.g[AT][0] = 0x4416_0000;
    c.mtc1(AT, 1);
    c.g[A1][0] = c.g[S5][0];
    c.g[AT][0] = 0x43c8_0000;
    c.mtc1(AT, 0);
    c.g[A2][0] = c.g[S6][0];
    c.g[A0][0] = c.lw(at(fx, spark::DESC));
    c.swc1(0, s(0x38));
    c.swc1(1, s(0x68));
    c.sw(s(0x30), 0);
    c.sw(s(0x34), 0);
    c.sw(s(0x3c), 0);
    c.swc1(1, s(0x40));
    for off in [0x44, 0x48, 0x4c, 0x50] {
        c.sw(s(off), 0);
    }
    c.swc1(1, s(0x54));
    for off in [0x58, 0x5c, 0x60, 0x64] {
        c.sw(s(off), 0);
    }
    c.sw(s(0x6c), 0);
    call_ptr(c, EMITTER_SET_PATH_SCALED, 0x002d_b200)?;
    let landed = c.g[S7][0] != 0;
    c.g[A2][0] = c.g[S5][0];
    // The path along the bone's axes.
    c.cycles += 22;
    let (axis_a, axis_b) = (r32(c, S3), r32(c, S4));
    if landed {
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0);
        vmul_bc(c, V0, 0, axis_a, s(0x40));
        c.g[A3][0] = c.g[S6][0];
        c.lq(V0, s(0x40));
        c.g[A1][0] = c.g[SP][0];
        c.sq(V0, s(0x30));
        vmul_bc(c, V1, 0, axis_b, s(0x50));
        c.lq(V0, s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.sq(V0, s(0x40));
        call_ptr(c, EMITTER_SET_PATH, 0x002d_b258)?;
    } else {
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 0);
        vmul_bc(c, V0, 0, axis_a, s(0x40));
        c.g[AT][0] = 0x42b4_0000;
        c.mtc1(AT, 1);
        c.lq(V0, s(0x40));
        c.g[A3][0] = c.g[S6][0];
        c.g[A1][0] = c.g[SP][0];
        c.sq(V0, s(0x30));
        vmul_bc(c, V1, 1, axis_b, s(0x50));
        c.lq(V0, s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.sq(V0, s(0x40));
        call_ptr(c, EMITTER_SET_PATH, 0x002d_b2b8)?;
        c.cycles += 1;
    }
    Ok(())
}

/// The burst (when a random roll beats gp-0x3c5c): three points
/// `source + (8 * (r1 - 1) - 4, ...)` with random scales `10 * (r - 1) +
/// 20`, at `fx+0x30..` and `fx+0x60..`. Returns whether it ran (else the
/// caller clears the burst flag).
fn spark_burst(c: &mut Cpu, sp: u32, gp: u32) -> bool {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    let seed_at = gp.wrapping_add(RAND_SEED);
    c.cycles += 21;
    c.g[V0][0] = 0x1_8fcd;
    c.g[A0][0] = 0xe9_0000;
    c.mult(0, V1, V1, V0);
    c.g[A0][0] = 0xe9_507c;
    c.g[V0][0] = 0x7f_ffff;
    c.g[A1][0] = 0x3f80_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.lwc1(1, gp.wrapping_sub(0x3c5c));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V1][0] &= c.g[V0][0];
    c.g[V1][0] |= c.g[A1][0];
    let seed = c.g[V1][0];
    c.sw(seed_at, seed);
    c.lwc1(0, seed_at);
    c.sub_s(0, 0, 2);
    c.c_lt_s(1, 0);
    let go = c.fcc();
    c.g[A2][0] = 0x1_0000;
    if !go {
        return false;
    }
    c.cycles += 18;
    c.g[T4][0] = 0xe9_507c;
    c.g[A1][0] = 0x7f_ffff;
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 7);
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(AT, 6);
    c.mov_s(3, 2);
    c.g[AT][0] = 0xffff_ffff_c080_0000;
    c.mtc1(AT, 5);
    c.g[A2][0] = 0x1_8fcd;
    c.g[AT][0] = 0x4100_0000;
    c.mtc1(AT, 4);
    c.g[T1][0] = 0x3f80_0000;
    c.g[T0][0] = sx(at(fx, spark::BURST_POINTS));
    c.g[A3][0] = sx(at(fx, spark::BURST_SCALES));
    c.g[T2][0] = 2;
    loop {
        c.cycles += 53;
        c.g[V0][0] = c.lw(seed_at);
        addiu(c, T2, T2, u32::MAX);
        c.sw(s(0x2c), 0);
        // scale = 10 * (r - 1) + 20
        rand_step(c, V1, A2, T4, A1, T1);
        c.mult(0, V1, V0, A2);
        let seed = c.g[V0][0];
        c.sw(seed_at, seed);
        c.lwc1(0, seed_at);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, T4)));
        c.sub_s(0, 0, 3);
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= c.g[T1][0];
        c.mult(0, A0, V0, A2);
        c.mul_s(0, 0, 6);
        c.add_s(0, 0, 7);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, T4)));
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= c.g[T1][0];
        let scale_at = r32(c, A3);
        c.swc1(0, scale_at);
        c.mult(0, T3, V1, A2);
        let seed = c.g[V0][0];
        c.sw(seed_at, seed);
        addiu(c, A3, A3, 4);
        // the offset (8 * (r - 1) - 4) per axis
        c.lwc1(2, seed_at);
        let seed = c.g[V1][0];
        c.sw(seed_at, seed);
        c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, T4)));
        c.sub_s(2, 2, 3);
        c.g[A0][0] &= c.g[A1][0];
        c.lwc1(1, seed_at);
        c.g[A0][0] |= c.g[T1][0];
        let seed = c.g[A0][0];
        c.sw(seed_at, seed);
        c.sub_s(1, 1, 3);
        c.mul_s(2, 2, 4);
        c.lwc1(0, seed_at);
        c.mul_s(1, 1, 4);
        c.sub_s(0, 0, 3);
        c.add_s(2, 2, 5);
        c.add_s(1, 1, 5);
        c.mul_s(0, 0, 4);
        c.swc1(2, s(0x20));
        c.swc1(1, s(0x24));
        c.add_s(0, 0, 5);
        c.swc1(0, s(0x28));
        vadd(c, at(fx, spark::SOURCE), s(0x20), s(0x30), false);
        c.lq(V0, s(0x30));
        let point = r32(c, T0);
        c.sq(V0, point);
        c.sq(V0, s(0x10));
        let more = (c.g[T2][0] as i64) >= 0;
        addiu(c, T0, T0, 0x10);
        if !more {
            break;
        }
        back_edge(c, 0x002d_b360);
    }
    c.cycles += 2;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 240;

    /// A real guest function that returns 0 (`UIState_TransitionIn`), for
    /// vtable slots whose 0 answer a fake cannot give.
    const RETURN_ZERO: u32 = 0x003a_5ab0;

    /// Vtable slot `off` = `{delta, func}`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// A random `this` adjustment.
    fn delta(r: &mut Rng) -> u32 {
        (r.next_u32() as i16 as i32 & !3) as u32
    }

    /// The graphics manager at DATA+0x4000: texture handles and state
    /// words around +0xf00..+0x1100, a GS state stack with a random top,
    /// the default record, and its vtable with `Draw` (FAKE) and the camera
    /// matrix query 0x118 (FAKE+0x10).
    fn gfx_fixture(c: &mut Cpu, r: &mut Rng) {
        let (gfx, vt, stack) = (DATA + 0x4000, DATA + 0x6000, DATA + 0x6800);
        c.bus.write32(GAME_GP - GFX_MAN, gfx);
        fill(c, r, gfx + 0xf00, 0x80);
        c.bus.write32(gfx + GS_TOP, stack + RECORD * (r.next_u32() % 4));
        fill(c, r, stack, 0x30);
        fill(c, r, GS_DEFAULT, 5);
        c.bus.write32(gfx + GFX_VTABLE, vt);
        slot(c, vt, DRAW, delta(r), FAKE);
        slot(c, vt, 0x118, delta(r), FAKE + 0x10);
    }

    #[test]
    fn rider_base_render_power_up_trail_matches() {
        covered(ssx3_ee::s09::f_002eb198__generated, rider_base_render_power_up_trail, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2eb1_98, 4);
            gfx_fixture(c, &mut r);
            let this = DATA;
            fill(c, &mut r, this, 0x40);
            // 0/1/negative: nothing drawn; 2 or 3 sections (3 fills the strips).
            let n = match seed % 4 {
                0 => [0, 1, (-3i32) as u32][(r.next_u32() % 3) as usize],
                1 => 2,
                _ => 3,
            };
            c.bus.write32(this + trail::COUNT, n);
            c.bus.write32(this + trail::FIRST, r.next_u32() % 16);
            c.g[A0][0] = sx(this);
        });
    }

    /// A streamer's rider (+0x780 model with bone tables at +0x30/+0x34,
    /// bone indices at `bone_a`/`bone_b`).
    fn streamer_rider(c: &mut Cpu, r: &mut Rng, rider: u32, bone_a: u32, bone_b: u32) {
        let (model, bones) = (rider + 0x1000, rider + 0x1100);
        c.bus.write32(rider + 0x780, model);
        c.bus.write32(rider + bone_a, r.next_u32() % 4);
        c.bus.write32(rider + bone_b, r.next_u32() % 4);
        c.bus.write32(model + 0x30, bones);
        c.bus.write32(model + 0x34, bones + 0x100);
        fill(c, r, bones, 0x80);
    }

    #[test]
    fn c_air_streamer_fx_render_matches() {
        covered(ssx3_ee::s09::f_002ef950__generated, c_air_streamer_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef9_50, 2);
            gfx_fixture(c, &mut r);
            let (this, rider, points) = (DATA, DATA + 0x100, DATA + 0x2000);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + air::RIDER, rider);
            c.bus.write32(this + air::POINTS, points);
            // Up to 26 segments (52 vertices per strip).
            let n = if seed % 5 == 0 { r.next_u32() % 2 } else { 2 + r.next_u32() % 25 };
            c.bus.write32(this + air::COUNT, n);
            c.bus.write32(this + air::FIRST, r.next_u32() % 0x19);
            c.bus.write32(this + air::ALPHA, r.f32(1.0));
            c.bus.write32(this + air::LENGTH, 1 + r.next_u32() % 30);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            c.bus.write32(rider + 0x2ec, r.f32(10.0));
            fill(c, &mut r, points, 0x19 * 8);
            c.bus.write32(GAME_GP + 0x1630, (seed % 3 == 0) as u32);
            fill(c, &mut r, GAME_GP - 0x3940, 8);
            c.bus.write32(GAME_GP + 0xa84, r.f32(1.0));
        });
    }

    #[test]
    fn c_player_pulse_fx_render_matches() {
        covered(ssx3_ee::s09::f_002e3af8__generated, c_player_pulse_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e3a_f8, 4);
            gfx_fixture(c, &mut r);
            let (fx, rider, rider_vt) = (DATA, DATA + 0x100, DATA + 0x2400);
            let (app, module, ai, players, mode) =
                (DATA + 0x2500, DATA + 0x2600, DATA + 0x2700, DATA + 0x2800, DATA + 0x2900);
            let (iface, iface_vt, lib) = (DATA + 0x2a00, DATA + 0x2b00, DATA + 0x2c00);
            fill(c, &mut r, fx, 0x10);
            c.g[A0][0] = sx(fx);
            c.bus.write32(fx, rider);
            // The rider: "local player" query (0 or a fake), switches, bone.
            c.bus.write32(rider + 0x6c0, rider_vt);
            let query = if r.next_u32() % 2 == 0 { RETURN_ZERO } else { FAKE + 0x20 };
            slot(c, rider_vt, 0x40, delta(&mut r), query);
            c.bus.write32(rider + 0x870, r.next_u32() % 2);
            c.bus.write32(rider + 0x87c, r.next_u32() % 2);
            c.bus.write32(rider + 0xac4, (r.next_u32() % 4 == 0) as u32);
            c.bus.write32(rider + 0x8a8, r.next_u32() % 4);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            // The app: game module, players, mode object.
            c.bus.write32(GAME_GP - APP, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(app + 0x78, r.next_u32());
            c.bus.write32(module + 0xc, ai);
            c.bus.write32(ai + 0x7c, 1 + seed % 2);
            c.bus.write32(module + 0x84, players);
            c.bus.write32(module + 0x78, mode);
            c.bus.write32(players + 0x10, r.next_u32() % 3);
            c.bus.write32(players + 0x14, r.next_u32() % 2);
            c.bus.write32(mode + 0x1bc, 0xd + r.next_u32() % 5);
            c.bus.write32(SPLIT_SWITCH, r.next_u32() % 2);
            c.bus.write8(GAME_MODE, (3 + r.next_u32() % 4) as u8);
            // BELibrary: built; the phase interface's method 8 is a fake.
            c.bus.write32(GAME_GP - 0x1f04, lib);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, iface);
            c.bus.write32(iface + 4, iface_vt);
            slot(c, iface_vt, 8, delta(&mut r), FAKE + 0x30);
            fill(c, &mut r, GAME_GP - 0x3a30, 8);
        });
    }

    /// The PS2 graphics manager `BeginTriStrip`/`EndTriStrip` run on (real
    /// callees): GS state, view matrices up to date (+0x6b90), the packet
    /// source (+0x5a00), the strip table (+0x18f0, full or nearly empty),
    /// and the scratchpad stream at gp+0x1384 (two slots, packets in RAM).
    fn graphics_man_fixture(c: &mut Cpu, r: &mut Rng) {
        let (gm, stack, table, stream) = (DATA + 0x2_0000, DATA + 0x7000, DATA + 0x4_0000, DATA + 0x4_8000);
        c.bus.write32(GAME_GP + GRAPHICS_MAN, gm);
        c.bus.write32(gm + GS_TOP, stack);
        fill(c, r, stack, 8);
        c.bus.write32(gm + 0x6b90, 1);
        c.bus.write32(gm + 0x5a00, DATA + 0x3_0000);
        c.bus.write32(gm + 0x18f0, table);
        c.bus.write32(table, if r.next_u32() % 2 == 0 { 3000 } else { r.next_u32() % 4 });
        c.bus.write32(GAME_GP + 0x1290, r.next_u32() % 2);
        c.bus.write32(GAME_GP + 0x1384, stream);
        let slot = r.next_u32() % 2;
        c.bus.write32(stream + 4, 2);
        c.bus.write32(stream + 8, slot);
        c.bus.write32(stream + 0xc, slot);
        c.bus.write32(stream + 0x10, r.next_u32() % 64);
        c.bus.write32(stream + 0x14, DATA + 0x5_0000);
        c.bus.write32(stream + 0x18, DATA + 0x5_1000);
    }

    #[test]
    fn c_speed_boost_streamer_fx_render_matches() {
        covered(ssx3_ee::s09::f_002e7a10__generated, c_speed_boost_streamer_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e7a_10, 2);
            gfx_fixture(c, &mut r);
            graphics_man_fixture(c, &mut r);
            let (fx, rider, points) = (DATA, DATA + 0x800, DATA + 0x2000);
            c.g[A0][0] = sx(fx);
            // Floats everywhere (the flame ring at +0x50..+0x398), then the
            // fields.
            for i in 0..0xec {
                c.bus.write32(fx + 4 * i, r.f32(4.0));
            }
            c.bus.write32(fx + boost::RIDER, rider);
            let count = if seed % 6 == 0 { 0 } else { 1 + r.next_u32() % 21 };
            c.bus.write32(fx + boost::COUNT, count);
            let age = if seed % 7 == 1 { 1.0f32 } else { 3.0 + (r.next_u32() % 8) as f32 };
            c.bus.write32(fx + boost::FADE, age.to_bits());
            c.bus.write32(fx + boost::FLAME_CLAMP, (0.5 + (r.next_u32() % 8) as f32).to_bits());
            c.bus.write32(fx + boost::ALPHA, r.f32(1.0));
            c.bus.write32(GAME_GP - 0x39d8, (-0.5f32).to_bits());
            c.bus.write32(GAME_GP + 0x1634, (seed % 5 != 2) as u32);
            c.bus.write32(fx + boost::TEXTURE, r.next_u32() % 8);
            c.bus.write32(fx + boost::FIRST, r.next_u32() % 0x14);
            c.bus.write32(fx + boost::POINTS, points);
            fill(c, &mut r, points, 0x14 * 8);
            let flames = if seed % 4 == 0 { r.next_u32() % 2 } else { 2 + r.next_u32() % 14 };
            c.bus.write32(fx + boost::FLAMES, flames);
            c.bus.write32(fx + boost::FLAME_HEAD, r.next_u32() % 0x1e);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            fill(c, &mut r, rider + 0x1a0, 4);
            c.bus.write32(GAME_GP - 0x39d4, r.f32(8.0));
        });
    }

    #[test]
    fn attached_emitter_fx_update_matches() {
        covered(ssx3_ee::s09::f_002f1a08__generated, attached_emitter_fx_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2f1a_08, 1);
            let (fx, model, rider, rider_vt) = (DATA, DATA + 0x8000, DATA + 0xa000, DATA + 0xa800);
            let (bones, lods, lod) = (DATA + 0x9100, DATA + 0x8100, DATA + 0x8200);
            c.g[A0][0] = sx(fx);
            // Bones looked up already, or not (then FindBones finds none:
            // every RiderModel_FindBone answers -1).
            c.bus.write32(fx + attached::BONES_FOUND, (seed % 5 != 0) as u32);
            for i in 0..0x1a {
                c.bus.write32(fx + attached::BONES + 4 * i, r.next_u32() % 4);
            }
            c.bus.write32(fx + attached::MODEL, model);
            c.bus.write32(model + 0x30, bones);
            fill(c, &mut r, bones - 0x100, 0x100);
            c.bus.write32(model + 0x1c, lods);
            c.bus.write32(model + 0xc, lod);
            c.bus.write32(lod + 0x44, 0);
            for (i, off) in [0x140, 0x144, 0x148].into_iter().enumerate() {
                let zero = (seed >> i) % 3 == 0;
                c.bus.write32(model + off, if zero { 0 } else { r.f32(2.0) });
            }
            c.bus.write32(fx + attached::RIDER, if seed % 4 == 3 { 0 } else { rider });
            c.bus.write32(rider + 0x6c0, rider_vt);
            slot(c, rider_vt, 0x10, delta(&mut r), FAKE);
            fill(c, &mut r, rider + 0x370, 4);
            // The emitters: attachment points 0x17-0x19 or random, always-on
            // or not, idle spawns (DynamicEmitter_Spawn returns at once).
            for k in 0..10u32 {
                let (em, desc) = (DATA + 0x1000 + 0x300 * k, DATA + 0x5000 + 0x200 * k);
                let used = r.next_u32() % 4 != 0;
                c.bus.write32(fx + attached::EMITTERS + 4 * k, if used { em } else { 0 });
                let point = match r.next_u32() % 5 {
                    0 => 0x17,
                    1 => 0x18,
                    2 => 0x19,
                    _ => r.next_u32() % 0x19,
                };
                c.bus.write32(em + emitter::POINT, point);
                c.bus.write32(em + emitter::ALWAYS, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(em + emitter::PHASE, ((r.next_u32() % 64) as f32 / 8.0).to_bits());
                c.bus.write32(em + emitter::DESC, desc);
                c.bus.write32(em + 0x174, 0);
                for off in (0xd8..0x110).step_by(4) {
                    c.bus.write32(desc + off, r.f32(4.0));
                }
                for off in [0xf0, 0xf4, 0xf8] {
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(desc + off, 0);
                    }
                }
            }
            c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
            for off in [0x392c, 0x3928, 0x3924, 0x3920, 0x391c] {
                c.bus.write32(GAME_GP - off, r.f32(2.0));
            }
            fill(c, &mut r, ORIGIN, 4);
            // Noise_Sample1D's table (0xa0 floats; negative indices too).
            for i in 0..0x140u32 {
                c.bus.write32(0x0044_5ab0 - 0x280 + 4 * i, r.f32(1.0));
            }
        });
    }

    #[test]
    fn c_board_spark_fx_update_matches() {
        covered(ssx3_ee::s09::f_002dabc8__generated, c_board_spark_fx_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2dab_c8, 3);
            let (fx, em, em_vt, desc) = (DATA, DATA + 0x1000, DATA + 0x1800, DATA + 0x2000);
            let (clock, app, module, players) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200, DATA + 0x3300);
            let (rider, state, rider_vt, model, bones) =
                (DATA + 0x4000, DATA + 0x4800, DATA + 0x4900, DATA + 0x5000, DATA + 0x5100);
            c.g[A0][0] = sx(fx);
            c.bus.write32(fx + spark::RIDER, rider);
            c.bus.write32(fx + spark::EMITTER, em);
            c.bus.write32(fx + spark::RESTART_ARG, r.next_u32());
            c.bus.write32(fx + spark::DESC, desc);
            c.bus.write32(fx + spark::RESTART, r.next_u32() % 2);
            // The dynamic emitter: restart method (fake), colour slots;
            // DynamicEmitter_Spawn returns at once (+0x174 = 0).
            c.bus.write32(em + 0x1f8, em_vt);
            slot(c, em_vt, 0x18, delta(&mut r), FAKE + 0x10);
            c.bus.write32(em + 0x58, r.f32(2.0));
            c.bus.write32(em + 0x174, 0);
            c.bus.write32(em + 0x17c, r.next_u32() % 4);
            c.bus.write32(em + 0x204, em + 0x400);
            // The descriptor: no catch-up loop in Emitter_UpdateDescriptor.
            for i in 0..0x50 {
                c.bus.write32(desc + 4 * i, r.f32(4.0));
            }
            c.bus.write32(desc, 1000);
            c.bus.write32(desc + 8, 0);
            c.bus.write32(desc + 0xc, 1.0f32.to_bits());
            c.bus.write32(desc + 0x4c, 1.0f32.to_bits());
            // Frame rate, app, player records (+0x88 rail, +0x8c carving).
            c.bus.write32(GAME_GP + 0x2a74, clock);
            c.bus.write32(clock + 0x10, if r.next_u32() % 2 == 0 { 30 } else { 60 });
            c.bus.write32(GAME_GP - APP, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(module + 0x44, players);
            for i in 0..10 {
                c.bus.write32(players + 0xb0 * i + 0x88, r.next_u32() % 2);
                c.bus.write32(players + 0xb0 * i + 0x8c, (r.next_u32() % 3 != 0) as u32);
            }
            // The rider: player index or the spark character, velocity,
            // position, direction, main state, board query (fake, forced
            // to a small board index), bone.
            let character = if r.next_u32() % 4 == 0 { 9 } else { r.next_u32() % 4 };
            c.bus.write32(rider + 0x438, character);
            for off in [0x1a0, 0x1e0, 0x460] {
                for k in 0..4 {
                    c.bus.write32(rider + off + 4 * k, r.f32(8.0));
                }
            }
            c.bus.write32(rider + 0x2f0, r.f32(1.0));
            c.bus.write32(rider + 0x77c, state);
            c.bus.write32(state + 0xde0, if r.next_u32() % 2 == 0 { 4 } else { r.next_u32() % 8 });
            c.bus.write32(rider + 0x6c0, rider_vt);
            slot(c, rider_vt, 0x38, delta(&mut r), FAKE);
            for n in 0..8 {
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS + 4 * n, 1 + r.next_u32() % 4);
            }
            for i in 0..0x140u32 {
                c.bus.write32(BOARD_COLOURS + 4 * i, r.f32(1.0));
            }
            c.bus.write32(rider + 0x780, model);
            c.bus.write32(rider + 0x8a4, r.next_u32() % 4);
            c.bus.write32(model + 0x34, bones);
            fill(c, &mut r, bones, 0x40);
            // Globals: the threshold (speed 5), constants, switches, seed.
            c.bus.write32(GAME_GP - 0x3c94, 5.0f32.to_bits());
            for off in (0x3c5c..0x3c94).step_by(4) {
                c.bus.write32(GAME_GP - off, r.f32(2.0));
            }
            c.bus.write32(GAME_GP + 0x163c, (r.next_u32() % 3 == 0) as u32);
            c.bus.write32(GAME_GP + 0x1638, (r.next_u32() % 3 == 0) as u32);
            c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
        });
    }
}
