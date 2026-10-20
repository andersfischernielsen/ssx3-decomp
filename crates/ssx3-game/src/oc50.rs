//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the transition manager's per-frame update, the two
//! spline-modifier updates, a move-node query and the world painter's
//! screen-tint / screen-fill / fog virtual getters, each with a differential
//! test against the generated body.
//!
//! - `cTransitionMan_Update` (0x2e46c8): the board-wake fade context. When a
//!   fade is running, it polls the effect clock and stops the fade once three
//!   ticks have passed; then it advances the screen fade (from an explicit
//!   value, a target surface colour, or the frame rate) and stops the
//!   transition when the fade reaches its total.
//! - `MultiSplineModifier_DebugDrawPath` (0x345430) and
//!   `MultiSplineModifier_Update` (0x35a560).
//! - `ModObj_v23` (0x356d48): the move object's block state, when its node is
//!   in mode 2.
//! - `ScriptViewObj_Update` (0x281100): copies the bound rider's position and
//!   rebinds the camera slot when the script object has no camera yet.
//! - Ten `WorldPainter_Get*` getters: two families that index the
//!   0x4fa370 table (`entry = &table[index]`, `obj = *entry->field`,
//!   `vt = obj->vtable`), call vtable slot `off` and return the float it
//!   points at; the tint/fill family returns 1.0 or 0.0 when the painter's
//!   global at `gp+0xa70` is set.
//!
//! FPU arithmetic uses [`FpuF32`]; the virtual calls go through the function
//! table ([`call_ptr`]) exactly as the original's `jalr`s do.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_46c8, c_transition_man_update, "cTransitionMan_Update"),
    (
        0x0034_5430,
        multi_spline_modifier_debug_draw_path,
        "MultiSplineModifier_DebugDrawPath",
    ),
    (
        0x0035_a560,
        multi_spline_modifier_update,
        "MultiSplineModifier_Update",
    ),
    (0x0035_6d48, mod_obj_v23, "ModObj_v23"),
    (
        0x002e_ec30,
        world_painter_get_screen_tint_blue,
        "WorldPainter_GetScreenTintBlue",
    ),
    (
        0x002e_ebd0,
        world_painter_get_screen_tint_green,
        "WorldPainter_GetScreenTintGreen",
    ),
    (
        0x002e_eb70,
        world_painter_get_screen_tint_red,
        "WorldPainter_GetScreenTintRed",
    ),
    (
        0x002e_ed50,
        world_painter_get_screen_fill_blue,
        "WorldPainter_GetScreenFillBlue",
    ),
    (
        0x002e_ecf0,
        world_painter_get_screen_fill_green,
        "WorldPainter_GetScreenFillGreen",
    ),
    (
        0x002e_ec90,
        world_painter_get_screen_fill_red,
        "WorldPainter_GetScreenFillRed",
    ),
    (0x0028_1100, script_view_obj_update, "ScriptViewObj_Update"),
    (0x002e_eb28, world_painter_get_fog30, "WorldPainter_GetFog30"),
    (0x002e_eae0, world_painter_get_fog28, "WorldPainter_GetFog28"),
    (0x002e_ea98, world_painter_get_fog20, "WorldPainter_GetFog20"),
    (0x002e_ea50, world_painter_get_fog18, "WorldPainter_GetFog18"),
];

// ---------------------------------------------------------------------------
// Callees and globals.

mod callee {
    use super::{ee, Callee};

    /// `void cTransitionMan_Stop(cTransitionMan*)` (0x2e4578): ends the fade.
    pub const TRANSITION_MAN_STOP: Callee =
        Callee { addr: 0x002e_4578, f: ee::s09::cTransitionMan_Stop };
    /// `int SplineModifier_IsNodeVisible(SplineModifierNode*)` (0x345638).
    pub const SPLINE_MODIFIER_IS_NODE_VISIBLE: Callee =
        Callee { addr: 0x0034_5638, f: ee::s11::SplineModifier_IsNodeVisible };
    /// `void ScriptViewObj_BindCamSlot(ScriptViewObj*, int, int)` (0x281ac8).
    pub const SCRIPT_VIEW_OBJ_BIND_CAM_SLOT: Callee =
        Callee { addr: 0x0028_1ac8, f: ee::s07::ScriptViewObj_BindCamSlot };
}

/// `float World_GetSecondsPerTick(void)` (0x2d1c70): a cross-shard call the
/// generator emitted as `call_indirect`.
const WORLD_GET_SECONDS_PER_TICK: u32 = 0x002d_1c70;

/// The table of world-painter view records (`D_004FA370`); each entry is
/// 0xf0 bytes.
const WP_VIEWS: u32 = 0x004f_a370;
const WP_ENTRY_SIZE: u32 = 0xf0;
/// The screen tint/fill overrides switch (`D_004A3B60`, `gp+0xa70`).
const SCREEN_OVERRIDES: u32 = 0xa70;
/// The frame-rate state (`D_004A5B64`, `gp+0x2a74`).
const APP_STATE: i32 = 0x2a74;
/// The board-wake effect world (`D_004A5B80`, `gp+0x2a90`).
const BOARD_WAKE_WORLD: i32 = 0x2a90;
/// The game world (`D_004A28A8`, `gp-0x848`).
const WORLD: i32 = -0x848;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A `lw` result as an address/value.
#[inline(always)]
fn lw32(c: &mut Cpu, a: u32) -> u32 {
    c.lw(a) as u32
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// `swc1 fN, a`.
#[inline(always)]
fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = FpuF32(c.f[n]);
    Addr(a).write_fpu(c, v);
}

/// `c.lt.s`/`c.le.s` then the FPU condition bit.
#[inline(always)]
fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

// ---------------------------------------------------------------------------
// cTransitionMan_Update.

/// `void cTransitionMan_Update(cTransitionMan* self)` (0x2e46c8): the
/// board-wake fade. `+0x7c` counts fade ticks while `+0x44` is set: the
/// effect clock's frame count (`world->+0x10d8` vtable slot 114) minus
/// `+0x80` reaching 3 ends it (vtable slot 60 gets `self+0x68`) and clears
/// `+0x7c`. Then, unless `+0x44` (fading) or `+0x40` (paused) is zero, the
/// fade position `+0x48` advances: `+0x18 > 0` takes the explicit value
/// `+0x3c`, a non-null `+0x70` scales the surface colour by the three
/// weights, otherwise it adds one frame time (`D_004A5B64->+0x10`); when
/// the weights' sum is at most the new position the transition stops.
pub fn c_transition_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(0x7c).lw(c);
        let fading = c.g[V0][0] != 0;
        // ctx = D_004A5B80 (loaded in the original's delay slot).
        let ctx = lw32(c, gp(c, BOARD_WAKE_WORLD));
        c.g[S1][0] = sx(ctx);
        if fading {
            c.cycles += 10;
            let vt = lw32(c, p(ctx, 0x10d8));
            c.g[V1][0] = sx(vt);
            c.g[A0][0] = Addr(vt).off(0x390).lh(c);
            c.g[V0][0] = Addr(vt).off(0x394).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x002e_46fc);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_46fc)?;
            c.g[V1][0] = Addr(this).off(0x80).lw(c);
            let elapsed = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
            c.g[V0][0] = elapsed;
            let expired = (elapsed as i64) < 3;
            c.g[V0][0] = expired as u64;
            if expired {
                // The ticks path rejoins the fade code without the extra
                // cycle the other two paths pay.
                c.g[V0][0] = Addr(this).off(0x44).lw(c);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(ctx).off(0x10d8).lw(c);
                c.g[A1][0] = sx(p(this, 0x68));
                c.g[A0][0] = Addr(r32(c, V0)).off(0x1e0).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x1e4).lw(c);
                let target = r32(c, V1);
                let delta = r32(c, A0);
                c.g[RA][0] = sx(0x002e_4728);
                c.g[A0][0] = sx(ctx.wrapping_add(delta));
                call_ptr(c, target, 0x002e_4728)?;
                let v = 0u64;
                c.sw(p(this, 0x7c), v);
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(0x44).lw(c);
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(this).off(0x44).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            restore_s0(c, frame);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x40).lw(c);
            if c.g[V0][0] != 0 {
                restore_s0(c, frame);
            } else {
                fade_screen(c, this, frame)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lq s0, 0x20(sp)`: the paths that leave the fade early restore it before
/// the shared epilogue.
#[inline]
fn restore_s0(c: &mut Cpu, frame: Frame) {
    let a = frame.at(c, 0x20);
    c.lq(S0, a);
}

/// The fade-position update and the transition stop (0x2e479c onward).
fn fade_screen(c: &mut Cpu, this: u32, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(this).off(0x18).lw(c);
    if (c.g[V0][0] as i64) <= 0 {
        c.g[V0][0] = Addr(this).off(0x70).lw(c);
        // The surface colour or the frame time?
        let ptr = r32(c, V0);
        c.cycles += 2;
        if ptr == 0 {
            c.g[V0][0] = Addr(gp(c, APP_STATE)).lw(c);
            c.cycles += 7;
            // pos += 1.0 / (float)frame_time
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(2, p(r32(c, V0), 0x10));
            c.cvt_s_w(2, 2);
            c.lwc1(0, p(this, 0x48));
            c.div_s(1, 1, 2);
            c.add_s(0, 0, 1);
        } else {
            c.cycles += 8;
            // pos = *colour * (w0 + w1 + w2)
            let w0 = ldf(c, 1, p(this, 0x54));
            let w1 = ldf(c, 2, p(this, 0x58));
            let w2 = ldf(c, 3, p(this, 0x5c));
            setf(c, 1, w0 + w1);
            let v = ldf(c, 0, ptr);
            setf(c, 0, v);
            setf(c, 1, FpuF32(c.f[1]) + w2);
            setf(c, 0, FpuF32(c.f[0]) * FpuF32(c.f[1]));
        }
    } else {
        c.cycles += 2;
        c.lwc1(0, p(this, 0x3c));
    }

    // Store the new position and stop the transition once it reaches the
    // total (`w0 + w1 + w2 <= pos`).
    c.cycles += 11;
    stf(c, 0, p(this, 0x48));
    let w0 = ldf(c, 0, p(this, 0x54));
    let w1 = ldf(c, 1, p(this, 0x58));
    let w2 = ldf(c, 3, p(this, 0x5c));
    setf(c, 0, w0 + w1);
    let pos = ldf(c, 2, p(this, 0x48));
    let total = setf(c, 0, FpuF32(c.f[0]) + w2);
    if fcc(c, total.le(pos)) {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002e_47d0);
        c.g[A0][0] = c.g[S0][0];
        callee::TRANSITION_MAN_STOP.call(c, 0x002e_47d0)?;
    }
    restore_s0(c, frame);
    Ok(())
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_DebugDrawPath.

/// `void MultiSplineModifier_DebugDrawPath(void* self, void* arg)` (0x345430):
/// draws every visible node of the modifier's node list (`self->+8->+0x68`:
/// count at `+0x20`, first node at `+0x24`, next at `+0x64`). The wake
/// controller (`D_004A5B80->+0xe84`) is put in mode 1 (`+4` bits 2..6
/// cleared, bit 2 set) and its mode half at `+0x10` is cleared to -1; each
/// visible node is drawn through vtable slot 114 (`+0x258`) with the node
/// offset, `arg` and priority 10.
pub fn multi_spline_modifier_debug_draw_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = u64::MAX;
        frame.save(c, S2, 0x20);
        c.g[A3][0] = sx(0xffff_ff83);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        let w = lw32(c, gp(c, BOARD_WAKE_WORLD));
        c.g[S2][0] = sx(w);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[A0][0] = Addr(w).off(0xe84).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x68).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x20).lw(c);
        c.g[S0][0] = Addr(r32(c, V0)).off(0x24).lw(c);
        let ctrl = r32(c, A0);
        let minus_one = c.g[A2][0];
        Addr(ctrl).off(0x10).sh(c, minus_one);
        c.g[V1][0] = Addr(w).off(0xe84).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(4).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] | 4;
        let count = c.g[A1][0];
        let mode = c.g[V0][0];
        c.sw(p(r32(c, V1), 4), mode);
        if (count as i64) <= 0 {
            // No nodes.
        } else {
            c.cycles += 2;
            c.g[S1][0] = count;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::SPLINE_MODIFIER_IS_NODE_VISIBLE.call(c, 0x0034_5498)?;
                let hidden = c.g[V0][0] == 0;
                c.g[A1][0] = sx(p(r32(c, S0), 0x10));
                if !hidden {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(w).off(0x10d8).lw(c);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A3][0] = 0xa;
                    c.g[A0][0] = Addr(r32(c, V0)).off(0x258).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(0x25c).lw(c);
                    let target = r32(c, V1);
                    let delta = r32(c, A0);
                    c.g[RA][0] = sx(0x0034_54bc);
                    c.g[A0][0] = sx(w.wrapping_add(delta));
                    call_ptr(c, target, 0x0034_54bc)?;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = Addr(r32(c, S0)).off(0x64).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_5490);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_Update.

/// `void MultiSplineModifier_Update(void* self)` (0x35a560): advance the
/// modifier's position `+0x10` by `+0x14 * World_GetSecondsPerTick()`,
/// wrapping into `[0, +0x54)` (`+0x54` is the path length), then set the
/// dirty flag `+0x30`.
pub fn multi_spline_modifier_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, WORLD_GET_SECONDS_PER_TICK, 0x0035_a574)?;
        let this = r32(c, S0);
        // t = pos + rate * dt
        let rate = ldf(c, 1, p(this, 0x14));
        let pos = ldf(c, 2, p(this, 0x10));
        setf(c, 1, rate * FpuF32(c.f[0]));
        c.mtc1(0, 0);
        let t = setf(c, 2, pos + FpuF32(c.f[1]));
        let negative = fcc(c, t.lt(FpuF32(c.f[0])));
        stf(c, 2, p(this, 0x10));
        if negative {
            // Negative: wrap past the end.
            c.cycles += 3;
            let len = ldf(c, 0, p(this, 0x54));
            setf(c, 0, t + len);
            c.cycles += 2;
            stf(c, 0, p(this, 0x10));
            c.g[V0][0] = 1;
        } else {
            c.cycles += 5;
            let len = ldf(c, 0, p(this, 0x54));
            let over = fcc(c, len.le(t));
            c.g[V0][0] = 1;
            if over {
                c.cycles += 1;
                setf(c, 0, t - len);
                c.cycles += 2;
                stf(c, 0, p(this, 0x10));
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        let v = c.g[V0][0];
        c.sw(p(this, 0x30), v);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_v23.

/// `int ModObj_v23(ModObj* self)` (0x356d48): 0 unless the object's block
/// pointer (`+0x1c`) is set, its first node is set and that node's vtable
/// slot 26 (`+0xd0`) reports mode 2; then the node's state word `+0x34`.
pub fn mod_obj_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        let block = Addr(this).off(0x1c).lw(c);
        c.g[V1][0] = block;
        c.g[V0][0] = 0;
        let mut tail = true;
        if block != 0 {
            c.cycles += 3;
            let node = lw32(c, block as u32);
            c.g[A1][0] = sx(node);
            if node == 0 {
                let a = frame.at(c, 0x10);
                c.lq(S0, a);
                tail = false;
            } else {
                c.cycles += 8;
                let vt = lw32(c, node);
                c.g[V1][0] = sx(vt);
                c.g[A0][0] = Addr(vt).off(0xd0).lh(c);
                c.g[V0][0] = Addr(vt).off(0xd4).lw(c);
                let target = r32(c, V0);
                let delta = r32(c, A0);
                c.g[RA][0] = sx(0x0035_6d84);
                c.g[A0][0] = sx(node.wrapping_add(delta));
                call_ptr(c, target, 0x0035_6d84)?;
                c.g[V1][0] = 0x2;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this).off(0x1c).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
                }
            }
        }
        if tail {
            c.cycles += 1;
            let a = frame.at(c, 0x10);
            c.lq(S0, a);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptViewObj_Update.

/// `void ScriptViewObj_Update(ScriptViewObj* self)` (0x281100): while the
/// object is active (`+8 > 0`), copy the bound rider's position (`rider+0x20`,
/// rider = `world->race->riders[+0xf8]`) to `+0x100`, and when it has no
/// camera yet (`+0x18 == 0`) bind one (`ScriptViewObj_BindCamSlot(self, 0, 0)`).
pub fn script_view_obj_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[V0][0] = Addr(this).off(8).lw(c);
        let idle = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = Addr(gp(c, WORLD)).lw(c);
        if !idle {
            c.cycles += 10;
            let world = r32(c, V0);
            c.g[V1][0] = Addr(this).off(0xf8).lw(c);
            c.g[A0][0] = Addr(world).off(0x84).lw(c);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = Addr(this).off(0x18).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
            let rider = r32(c, A0);
            c.lq(3, p(rider, 0x20));
            let has_camera = c.g[A1][0] != 0;
            let dst = p(this, 0x100);
            c.sq(3, dst);
            if !has_camera {
                c.cycles += 4;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                callee::SCRIPT_VIEW_OBJ_BIND_CAM_SLOT.call(c, 0x0028_1150)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter virtual getters.

/// The 0x2ee*** family all do the same thing: index the global view table
/// (`&D_004FA370 + index * 0xf0`), look up `holder = table[index].field`,
/// `obj = *holder`, `vtable = obj->vtable`, then call
/// `vtable[vt_off/4](obj + delta)` through the function table and return the
/// float it points at.
fn world_painter_virtual_getter(c: &mut Cpu, table_off: u32, vt_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        let entry = r32(c, A0);
        let holder = Addr(entry).off(table_off).lw(c);
        c.g[V1][0] = holder;
        let holder = r32(c, V1);
        let obj = Addr(holder).lw(c);
        c.g[A2][0] = obj;
        let obj = r32(c, A2);
        let vt = Addr(obj).off(4).lw(c);
        c.g[A1][0] = vt;
        let vt = r32(c, A1);
        c.g[A0][0] = Addr(vt).off(vt_off).lh(c);
        c.g[V0][0] = Addr(vt).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        let delta = r32(c, A0);
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx(obj.wrapping_add(delta));
        call_ptr(c, target, ret)?;
        let r = r32(c, V0);
        c.lwc1(0, r);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The tint/fill family: like [`world_painter_virtual_getter`], but guarded
/// by the screen-override global (`gp+0xa70`): when it is set the function
/// returns a constant instead of calling.
fn world_painter_guarded_getter(
    c: &mut Cpu,
    vt_off: u32,
    ret: u32,
    default: GetterDefault,
) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, SCREEN_OVERRIDES as i32)).lw(c);
        let overridden = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if overridden {
            match default {
                GetterDefault::One => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                }
                GetterDefault::Zero => {
                    c.cycles += 3;
                    c.mtc1(0, 0);
                }
            }
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[V1][0] = 0xf0;
        c.g[V0][0] = 0x50_0000;
        c.mult(0, V1, A0, V1); // v1 = index * 0xf0
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let entry = r32(c, V1);
        c.g[A0][0] = Addr(entry).off(0xc).lw(c);
        let holder = r32(c, A0);
        c.g[A1][0] = Addr(holder).lw(c);
        let obj = r32(c, A1);
        c.g[V1][0] = Addr(obj).off(4).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(vt_off).lh(c);
        c.g[V0][0] = Addr(vt).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        let delta = r32(c, A0);
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx(obj.wrapping_add(delta));
        call_ptr(c, target, ret)?;
        let r = r32(c, V0);
        c.lwc1(0, r);
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// The constant a guarded getter returns when the screen overrides are on.
#[derive(Clone, Copy)]
enum GetterDefault {
    /// 1.0f (`mtc1 at = 0x3f800000`).
    One,
    /// 0.0f (`mtc1 zero`).
    Zero,
}

/// `float WorldPainter_GetFog18(int index)` (0x2eea50): vt slot `+0x48`.
pub fn world_painter_get_fog18(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x48, 0x002e_ea88);
}

/// `float WorldPainter_GetFog20(int index)` (0x2eea98): vt slot `+0x50`.
pub fn world_painter_get_fog20(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x50, 0x002e_ead0);
}

/// `float WorldPainter_GetFog28(int index)` (0x2eeae0): vt slot `+0x58`.
pub fn world_painter_get_fog28(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x58, 0x002e_eb18);
}

/// `float WorldPainter_GetFog30(int index)` (0x2eeb28): vt slot `+0x60`.
pub fn world_painter_get_fog30(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x60, 0x002e_eb60);
}

/// `float WorldPainter_GetScreenTintRed(int index)` (0x2eeb70): vt slot
/// `+0xa0`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_red(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xa0, 0x002e_ebc0, GetterDefault::One);
}

/// `float WorldPainter_GetScreenTintGreen(int index)` (0x2eebd0): vt slot
/// `+0xa8`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_green(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xa8, 0x002e_ec20, GetterDefault::One);
}

/// `float WorldPainter_GetScreenTintBlue(int index)` (0x2eec30): vt slot
/// `+0xb0`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_blue(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xb0, 0x002e_ec80, GetterDefault::One);
}

/// `float WorldPainter_GetScreenFillRed(int index)` (0x2eec90): vt slot
/// `+0xb8`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_red(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xb8, 0x002e_ecdc, GetterDefault::Zero);
}

/// `float WorldPainter_GetScreenFillGreen(int index)` (0x2eecf0): vt slot
/// `+0xc0`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_green(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xc0, 0x002e_ed3c, GetterDefault::Zero);
}

/// `float WorldPainter_GetScreenFillBlue(int index)` (0x2eed50): vt slot
/// `+0xc8`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_blue(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xc8, 0x002e_ed9c, GetterDefault::Zero);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};

    // ------------------------------------------------------------- fog getters

    /// `entry = &D_004FA370[index]`, `holder = entry[table_off/4]`,
    /// `obj = *holder`, `obj->vt = *obj+4`, slot `vt_off` a fake.
    fn table_getter_setup(c: &mut Cpu, seed: u32, salt: u64, table_off: u32, vt_off: u32) {
        let mut r = fakes::machine(c, seed, salt, 1);
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = WP_VIEWS.wrapping_add(index.wrapping_mul(WP_ENTRY_SIZE));
        let holder = fakes::ptr(&mut r);
        let obj = fakes::ptr(&mut r);
        let vt = fakes::ptr(&mut r);
        c.bus.write32(entry.wrapping_add(table_off), holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj.wrapping_add(4), vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt.wrapping_add(vt_off), delta as u16);
        c.bus.write32(vt.wrapping_add(vt_off).wrapping_add(4), FAKE);
    }

    fn fog_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        table_getter_setup(c, seed, 0x0e50_00a1 ^ vt_off as u64, 0x4, vt_off);
    }

    #[test]
    fn world_painter_get_fog18_matches() {
        fakes::covered(ee::s09::f_002eea50__generated, world_painter_get_fog18, 200, &|c, seed| {
            fog_setup(c, seed, 0x48);
        });
    }

    #[test]
    fn world_painter_get_fog20_matches() {
        fakes::covered(ee::s09::f_002eea98__generated, world_painter_get_fog20, 200, &|c, seed| {
            fog_setup(c, seed, 0x50);
        });
    }

    #[test]
    fn world_painter_get_fog28_matches() {
        fakes::covered(ee::s09::f_002eeae0__generated, world_painter_get_fog28, 200, &|c, seed| {
            fog_setup(c, seed, 0x58);
        });
    }

    #[test]
    fn world_painter_get_fog30_matches() {
        fakes::covered(ee::s09::f_002eeb28__generated, world_painter_get_fog30, 200, &|c, seed| {
            fog_setup(c, seed, 0x60);
        });
    }

    // ---------------------------------------------- tint / screen-fill getters

    fn tint_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        table_getter_setup(c, seed, 0x0e50_00b2 ^ vt_off as u64, 0xc, vt_off);
        // D_004A3B60: set on two seeds out of three (the constant path).
        let on = if seed % 3 == 0 { 0 } else { 1 };
        c.bus.write32(GAME_GP.wrapping_add(SCREEN_OVERRIDES), on);
    }

    #[test]
    fn world_painter_get_screen_tint_red_matches() {
        fakes::covered(
            ee::s09::f_002eeb70__generated,
            world_painter_get_screen_tint_red,
            200,
            &|c, seed| tint_setup(c, seed, 0xa0),
        );
    }

    #[test]
    fn world_painter_get_screen_tint_green_matches() {
        fakes::covered(
            ee::s09::f_002eebd0__generated,
            world_painter_get_screen_tint_green,
            200,
            &|c, seed| tint_setup(c, seed, 0xa8),
        );
    }

    #[test]
    fn world_painter_get_screen_tint_blue_matches() {
        fakes::covered(
            ee::s09::f_002eec30__generated,
            world_painter_get_screen_tint_blue,
            200,
            &|c, seed| tint_setup(c, seed, 0xb0),
        );
    }

    #[test]
    fn world_painter_get_screen_fill_red_matches() {
        fakes::covered(
            ee::s09::f_002eec90__generated,
            world_painter_get_screen_fill_red,
            200,
            &|c, seed| tint_setup(c, seed, 0xb8),
        );
    }

    #[test]
    fn world_painter_get_screen_fill_green_matches() {
        fakes::covered(
            ee::s09::f_002eecf0__generated,
            world_painter_get_screen_fill_green,
            200,
            &|c, seed| tint_setup(c, seed, 0xc0),
        );
    }

    #[test]
    fn world_painter_get_screen_fill_blue_matches() {
        fakes::covered(
            ee::s09::f_002eed50__generated,
            world_painter_get_screen_fill_blue,
            200,
            &|c, seed| tint_setup(c, seed, 0xc8),
        );
    }

    // --------------------------------------------------------------- ModObj_v23

    #[test]
    fn mod_obj_v23_matches() {
        fakes::covered(ee::s11::f_00356d48__generated, mod_obj_v23, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_00c3, 1);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let block = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
            c.bus.write32(this + 0x1c, block);
            let node = if seed % 4 == 0 { 0 } else { DATA + 0x3000 };
            c.bus.write32(DATA + 0x2000, node);
            let vt = DATA + 0x4000;
            c.bus.write32(DATA + 0x3000, vt);
            c.bus.write16(vt + 0xd0, 0x10);
            c.bus.write32(vt + 0xd4, FAKE);
            c.bus.write32(DATA + 0x3034, r.next_u32());
            // Half the calls report mode 2, the rest some other mode.
            let v = if seed % 2 == 0 { 2 } else { 0 };
            c.bus.write32(LOG_RESULTS, v);
        });
    }

    // -------------------------------------------------- MultiSplineModifier_Update

    #[test]
    fn multi_spline_modifier_update_matches() {
        check(ee::s11::f_0035a560__generated, multi_spline_modifier_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_00d4, 1);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            // World_GetSecondsPerTick reads world->+0x14.
            let world = DATA + 0x2000;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x14, r.edge_f32());
            c.bus.write32(this + 0x10, match seed % 4 {
                0 => 0,
                1 => 0x3f80_0000,
                _ => r.edge_f32(),
            });
            c.bus.write32(this + 0x14, match seed % 5 {
                0 => 0x7f80_0000, // largest, so the position overflows
                1 => 0x8000_0000, // -0.0
                2 => 0x3f80_0000,
                _ => r.edge_f32(),
            });
            c.bus.write32(this + 0x54, match seed % 3 {
                0 => 0,
                1 => 0x3f80_0000,
                _ => r.edge_f32(),
            });
            c.bus.write32(this + 0x30, r.next_u32());
        });
    }

    // ------------------------------------------ MultiSplineModifier_DebugDrawPath

    #[test]
    fn multi_spline_modifier_debug_draw_path_matches() {
        fakes::covered(
            ee::s11::f_00345430__generated,
            multi_spline_modifier_debug_draw_path,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0e50_00e5, 2);
                let this = DATA + 0x1000;
                let world = DATA + 0x2000;
                let ctrl = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x5000) as u64;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), world);
                c.bus.write32(world + 0xe84, ctrl);
                c.bus.write32(world + 0x10d8, DATA + 0x8000);
                c.bus.write32(this + 8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6000 + 0x68, list);
                let n = seed % 4;
                c.bus.write32(list + 0x20, n);
                // A chain of four nodes.
                let base = DATA + 0x9000;
                c.bus.write32(list + 0x24, base);
                for i in 0..4u32 {
                    let node = base + i * 0x80;
                    let next = if i + 1 < 4 { base + (i + 1) * 0x80 } else { 0 };
                    c.bus.write32(node + 0x64, next);
                    c.bus.write32(node + 0x6c, r.edge_f32());
                    c.bus.write32(node + 0x70, r.edge_f32());
                    c.bus.write32(node + 0x74, r.edge_f32());
                    c.bus.write32(node + 0x78, r.edge_f32());
                    c.bus.write32(node + 0x7c, r.edge_f32());
                    c.bus.write32(node + 0x80, r.edge_f32());
                }
                c.bus.write32(ctrl + 4, r.next_u32());
                c.bus.write32(ctrl + 0x10, r.next_u32());
                // The wake world's vtable: the draw slot, the visibility slot
                // and the slot inside SplineModifier_IsNodeVisible.
                let vt = DATA + 0x8000;
                for (i, off) in [0x258u32, 0x2e8, 0x158].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    // ------------------------------------------------------- ScriptViewObj_Update

    #[test]
    fn script_view_obj_update_matches() {
        check(ee::s07::f_00281100__generated, script_view_obj_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_00f6, 0);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 8, if seed % 3 == 0 { 0 } else { 1 + seed % 5 });
            let idx = seed % 4;
            c.bus.write32(this + 0xf8, idx);
            c.bus.write32(this + 0x18, if seed % 2 == 0 { 0 } else { 1 });
            let world = DATA + 0x2000;
            let rm = DATA + 0x2100;
            let riders = DATA + 0x2200;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, rm);
            c.bus.write32(rm + 0x84, riders);
            c.bus.write32(riders + 4 + 4 * idx, DATA + 0x3000);
            for i in 0..8 {
                c.bus.write32(DATA + 0x3000 + i * 4, r.next_u32());
            }
            // ScriptViewObj_BindCamSlot's short path: a negative slot at
            // +0xdc makes it bail out after scanning +0xd0/+0xd4.
            c.bus.write32(this + 0xdc, 0xffff_ffff);
            c.bus.write32(this + 0xd0, 0);
            c.bus.write32(this + 0xd4, 0);
        });
    }

    // --------------------------------------------------- cTransitionMan_Update

    #[test]
    fn c_transition_man_update_matches() {
        fakes::covered(ee::s09::f_002e46c8__generated, c_transition_man_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_0107, 2);
            let this = DATA + 0x1000;
            let ctx = DATA + 0x2000;
            let vt = DATA + 0x3000;
            c.g[A0][0] = this as u64;
            c.bus.write32(GAME_GP.wrapping_add(0x2a90), ctx);
            c.bus.write32(ctx + 0x10d8, vt);
            c.bus.write16(vt + 0x390, 0x10);
            c.bus.write32(vt + 0x394, FAKE);
            c.bus.write16(vt + 0x1e0, 0x18);
            c.bus.write32(vt + 0x1e4, FAKE + 0x10);
            // The effect clock returns DATA+0x8000; +0x80 is within three of it
            // on even seeds (fade over) and further away on odd ones.
            c.bus.write32(LOG_RESULTS, DATA + 0x8000);
            let start = if seed % 2 == 0 { DATA + 0x8000 - 1 } else { DATA + 0x8000 - 8 };
            c.bus.write32(this + 0x80, start);
            c.bus.write32(this + 0x7c, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x44, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x40, if seed % 7 == 0 { 1 } else { 0 });
            c.bus.write32(this + 0x18, match seed % 4 { 0 => 0, 1 => 1, _ => r.next_u32() as i32 as u32 });
            c.bus.write32(this + 0x70, if seed % 3 == 0 { 0 } else { DATA + 0x9000 });
            c.bus.write32(DATA + 0x9000, r.edge_f32());
            for off in [0x3c, 0x48, 0x54, 0x58, 0x5c] {
                c.bus.write32(this + off, r.edge_f32());
            }
            // The frame-rate state (D_004A5B64 -> +0x10 = the frame time).
            let app = DATA + 0xa000;
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), app);
            c.bus.write32(app + 0x10, (seed % 61 + 1) as u32);
            // Keep cTransitionMan_Stop on its short path (no further calls).
            c.bus.write32(this + 0x4c, 0);
            c.bus.write32(this + 0x50, 0);
            c.bus.write32(this + 0x6c, 0xffff_ffff);
        });
    }
}
