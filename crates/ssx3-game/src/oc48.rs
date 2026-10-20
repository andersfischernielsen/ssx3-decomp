//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small world/irradiance/particle/stream helpers, each with a differential test against the generated
//! body.
//!
//! - **Riders**: `Rider_IsControlMode3` (a control-mode test) and
//!   `Rider_ComputeLOD` (distance along the view axis turned into an LOD).
//! - **Particles / world**: `Emitter_Update` (an emitter's countdown and
//!   progress), `cDynamicSprayFX_Render` (render the live wakes),
//!   `BoaderShadow_Submit` (submit a shadow quad through the gfx manager),
//!   `BezierPatchCache_MarkUsed` (set the used bits of cached patches).
//! - **Render state**: `WorldPainter_GetSunFlareIntensity` and
//!   `WorldPainter_GetLighting30` (virtual getters on the lens-fx / render
//!   state entries).
//! - **Irradiance**: `Irradiance_AddScaledW` (`dst += src * dt` for ten
//!   vectors).
//! - **VU0 context**: `PS2ExecMan_v11` / `PS2ExecMan_v12` (restore / save the
//!   VU0 context of a memory block manager).
//! - **Modifiers**: `ModObj_v45` and `ModifierBlock_GetTransformRadius`.
//! - **Replay**: `ReplayStream_WriteObjectRef`.
//!
//! The replacements keep their state in the guest registers the originals
//! use where callers, callees or event handlers can see them, follow the
//! call protocol of [`crate::call`] and the event rules of [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X, Y, Z, W};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_48a8, rider_is_control_mode3, "Rider_IsControlMode3"),
    (0x0012_1f30, rider_compute_lod, "Rider_ComputeLOD"),
    (0x0022_7e98, ssx_app_next_tick, "SSXApp_NextTick"),
    (0x0026_ecd8, replay_stream_write_object_ref, "ReplayStream_WriteObjectRef"),
    (0x002e_24d0, dynamic_spray_fx_render, "cDynamicSprayFX_Render"),
    (0x002e_e268, world_painter_sun_flare_intensity, "WorldPainter_GetSunFlareIntensity"),
    (0x002e_efa8, world_painter_get_lighting30, "WorldPainter_GetLighting30"),
    (0x0031_a9d8, ps2_exec_man_v11, "PS2ExecMan_v11"),
    (0x0031_aa18, ps2_exec_man_v12, "PS2ExecMan_v12"),
    (0x0035_2bf8, modifier_block_get_transform_radius, "ModifierBlock_GetTransformRadius"),
    (0x0035_6a00, mod_obj_v45, "ModObj_v45"),
    (0x0037_0788, emitter_update, "Emitter_Update"),
    (0x0037_4a88, bezier_patch_cache_mark_used, "BezierPatchCache_MarkUsed"),
    (0x0037_5890, boader_shadow_submit, "BoaderShadow_Submit"),
    (0x0038_9620, irradiance_add_scaled_w, "Irradiance_AddScaledW"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const EMITTER_UPDATE_DESCRIPTOR: Callee =
        Callee { addr: 0x0036_d428, f: ee::s12::Emitter_UpdateDescriptor };
    pub const EMITTER_ADVANCE_DESC_TIME: Callee =
        Callee { addr: 0x0036_d3e8, f: ee::s12::Emitter_AdvanceDescTime };
    pub const PAD_QUEUE_ADVANCE_WRITE: Callee =
        Callee { addr: 0x0032_6b48, f: ee::s10::PadQueue_AdvanceWrite };
    pub const PAD_QUEUE_GET_PAD_STATUS: Callee =
        Callee { addr: 0x0032_6ca0, f: ee::s10::PadQueue_GetPadStatus };
    pub const PAD_QUEUE_GET_PAD_DATA: Callee =
        Callee { addr: 0x0032_6cc8, f: ee::s10::PadQueue_GetPadData };
    pub const INPUT_PAD_UPDATE: Callee = Callee { addr: 0x0032_1298, f: ee::s10::InputPad_Update };
    pub const COMM_SYSTEM_UPDATE: Callee = Callee { addr: 0x0025_5a20, f: ee::s06::cCommSystem_Update };
    pub const USB_KEYBOARD_UPDATE: Callee =
        Callee { addr: 0x0026_68e8, f: ee::s07::cUsbKeyboard_Update };
    pub const MATH_SIN_COS: Callee = Callee { addr: 0x0031_be50, f: ee::s10::Math_SinCos };
    pub const EMITTER_RENDER: Callee = Callee { addr: 0x0037_1688, f: ee::s12::mEmitter_Render };
    pub const VU0_SAVE_CONTEXT: Callee = Callee { addr: 0x0031_a308, f: ee::s10::VU0_SaveContext };
    pub const VU0_RESTORE_CONTEXT: Callee =
        Callee { addr: 0x0031_a3c0, f: ee::s10::VU0_RestoreContext };
    pub const MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2b88, f: ee::s11::ModifierBlock_GetTransformBounds };
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `v = sx(v + n)` on a GPR (an `addiu`/`daddiu`).
#[inline(always)]
fn addi(c: &mut Cpu, reg: usize, n: u32) {
    c.g[reg][0] = sx(r32(c, reg).wrapping_add(n));
}

// ---------------------------------------------------------------------------
// Riders.

/// `bool Rider_IsControlMode3(Rider* rider)` (0x1448a8): the control mode
/// word at +0x70 is 3 (the comparison is on the sign-extended 64-bit value,
/// as the original's is).
pub fn rider_is_control_mode3(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(abi::arg(c, 0), 0x70));
    c.g[V0][0] ^= 0x3;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// Rider fields used by `Rider_ComputeLOD`.
mod rider {
    /// The rider's vtable pointer (`&rider->mVtbl`): slot 5 returns the
    /// rider's position (`lq` of 4 floats).
    pub const VTBL: u32 = 0x6c0;
    /// The current level of detail.
    pub const LOD: u32 = 0x898;
}

/// The global object `D_004A289C` (`gp-0x854`) and the float table read from
/// it (its vtable at +0x10d8: slot 35 returns the view-projection matrix,
/// slot 29 the field of view).
const GLOBAL: i32 = -0x854;
const GLOBAL_VTBL: u32 = 0x10d8;
/// The `0.01` scale factor the LOD distance is multiplied by (`gp-0x78c4`).
const LOD_SCALE: i32 = -0x78c4;

/// `void Rider_ComputeLOD(cRider* self)` (0x121f30): project the rider's
/// position into view space, scale z by tan/`tan(cot)` of the field of view
/// and 0.01, then pick the LOD: 0 below 6 units, 1 below 10, else 2.
///
/// State lives in the original's registers: s0 the matrix, s1 `self`, f20 the
/// view depth.
pub fn rider_compute_lod(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 53;
        let frame = Frame::open(c, 0x70);
        c.g[A1][0] = c.lw(gp(c, GLOBAL));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save_fpr(c, 20, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[V1][0] = c.lw(at(c, A1, GLOBAL_VTBL));
        c.g[A0][0] = c.lh(at(c, V1, 0x118));
        c.g[V0][0] = c.lw(at(c, V1, 0x11c));
        // Register at the call: a0 = the global plus its slot 35 delta,
        // v0 the slot's function.
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1f60)?;
        // The rider's position through its own vtable slot 5.
        c.g[V1][0] = c.lw(at(c, S1, rider::VTBL));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(rider::VTBL));
        c.g[A0][0] = c.lh(at(c, V1, 0x28));
        c.g[V0][0] = c.lw(at(c, V1, 0x2c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1f7c)?;
        // view = matrix * position, keeping z.
        c.lqc2(8, r32(c, V0));
        c.lqc2(4, r32(c, S0));
        c.lqc2(5, at(c, S0, 0x10));
        c.lqc2(6, at(c, S0, 0x20));
        c.lqc2(7, at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        let sp = sp(c);
        c.sqc2(12, p(sp, 0x10));
        c.lq(2, p(sp, 0x10));
        c.g[A1][0] = c.lw(gp(c, GLOBAL));
        c.sq(2, sp);
        c.g[V1][0] = c.lw(at(c, A1, GLOBAL_VTBL));
        c.lwc1(20, p(sp, 0x8));
        c.g[A0][0] = c.lh(at(c, V1, 0xe8));
        c.g[V0][0] = c.lw(at(c, V1, 0xec));
        // The field of view.
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1fc8)?;
        // (sin, cos) of the field of view into the frame's scratch area.
        c.mov_s(12, 0);
        c.g[A0][0] = sx(sp.wrapping_add(0x20));
        c.g[A1][0] = sx(sp.wrapping_add(0x24));
        callee::MATH_SIN_COS.call(c, 0x0012_1fd8)?;
        c.lwc1(0, p(sp, 0x20));
        c.lwc1(1, p(sp, 0x24));
        c.lwc1(3, gp(c, LOD_SCALE));
        c.div_s(0, 0, 1);
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 2);
        c.mul_s(20, 20, 0);
        c.mul_s(20, 20, 3);
        c.c_lt_s(20, 2);
        let near = c.fcc();
        if near {
            // Below 6 units.
            c.cycles += 2;
            c.sw(p(r32(c, S1), rider::LOD), 0);
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0);
            c.c_lt_s(20, 0);
            let middle = c.fcc();
            c.g[V0][0] = 0x2;
            if middle {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            c.cycles += 1;
            c.sw(p(r32(c, S1), rider::LOD), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The application tick.

/// `int SSXApp_NextTick(App* self)` (0x227e98): advance the pad queue and
/// hand both pads to the input system. Returns 0 when there is no app or the
/// read side of the queue is still ahead of the game, else 1.
pub fn ssx_app_next_tick(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, -0x850));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let no_app = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut ticked = false;
        if !no_app {
            c.cycles += 4;
            let queue = c.g[V0][0];
            c.g[A0][0] = queue;
            callee::PAD_QUEUE_ADVANCE_WRITE.call(c, 0x0022_7ec0)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gp(c, -0x238));
            if !idle {
                // The comm system and the USB keyboard, when present.
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                } else {
                    c.cycles += 3;
                    callee::COMM_SYSTEM_UPDATE.call(c, 0x0022_7ed8)?;
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                }
                c.cycles += 2;
                let no_keyboard = c.g[A0][0] == 0;
                c.g[S2][0] = sx(r32(c, S0).wrapping_add(0xa8));
                if !no_keyboard {
                    c.cycles += 2;
                    callee::USB_KEYBOARD_UPDATE.call(c, 0x0022_7eec)?;
                }
                // Both pads.
                c.cycles += 3;
                c.g[S1][0] = 0;
                c.g[A0][0] = c.lw(gp(c, -0x850));
                loop {
                    c.cycles += 15;
                    c.g[A1][0] = c.g[S1][0];
                    callee::PAD_QUEUE_GET_PAD_STATUS.call(c, 0x0022_7f00)?;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                    callee::PAD_QUEUE_GET_PAD_DATA.call(c, 0x0022_7f14)?;
                    c.g[A0][0] = c.lw(r32(c, S2));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    addi(c, S2, 0x4);
                    callee::INPUT_PAD_UPDATE.call(c, 0x0022_7f28)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        ticked = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_7ef8);
                    }
                }
            }
        }
        if !ticked {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spray wakes.

/// Wake fields: the emitter, its live particle count and its age.
mod wake {
    pub const EMITTER: u32 = 0x174;
    pub const LIVE: u32 = 0x1e0;
}

/// `void cDynamicSprayFX_Render(Spray* self)` (0x2e24d0): if the owner is
/// live, render each of the ten wakes that has an emitter and live particles.
pub fn dynamic_spray_fx_render(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[V1][0] = c.lw(at(c, V0, 0xb18));
        let idle = c.g[V1][0] == 0;
        c.g[S1][0] = 0;
        if !idle {
            c.cycles += 2;
            c.g[S0][0] = 0x9;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(at(c, S2, 0x28));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V1][0] = c.lw(at(c, A0, wake::EMITTER));
                if c.g[V1][0] == 0 {
                    addi(c, S0, 0xffff_ffff);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, A0, wake::LIVE));
                    if (c.g[V0][0] as i64) <= 0 {
                        addi(c, S0, 0xffff_ffff);
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = 0x7;
                        callee::EMITTER_RENDER.call(c, 0x002e_2528)?;
                        addi(c, S0, 0xffff_ffff);
                    }
                }
                c.cycles += 2;
                let more = (c.g[S0][0] as i64) >= 0;
                addi(c, S1, 0x210);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_2500);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shadow quads.

/// The gfx manager singleton (`D_004A5B80`, `gp+0x2a90`).
const GFX_MAN: i32 = 0x2a90;
const GFX_MAN_VTBL: u32 = 0x10d8;

/// `void BoaderShadow_Submit(Shadow* self)` (0x375890): submit the quad
/// `(*(self+0xc), *(self+0x10), *(self+0x14), 1)` with the two vertices at
/// +0x90/+0xd0 and the texture at +0x170 through the gfx manager's virtual
/// `v67`, whose signature is `(Vec4 colour /*a1*/, void* uv /*a2*/,
/// void* pos /*a3*/, int tex /*t0*/, float alpha /*f12*/)`.
pub fn boader_shadow_submit(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw(gp(c, GFX_MAN));
        frame.save_ra(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        c.g[A1][0] = c.g[SP][0];
        c.g[T1][0] = c.lw(at(c, V0, GFX_MAN_VTBL));
        c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x90));
        c.lwc1(2, at(c, V1, 0xc));
        c.mov_s(12, 3);
        c.lwc1(0, at(c, V1, 0x10));
        c.g[A3][0] = sx(r32(c, V1).wrapping_add(0xd0));
        c.lwc1(1, at(c, V1, 0x14));
        c.g[A0][0] = c.lh(at(c, T1, 0x218));
        let sp = sp(c);
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.swc1(1, p(sp, 0x8));
        c.swc1(3, p(sp, 0xc));
        c.g[V0][0] = c.lw(at(c, T1, 0x21c));
        let target = r32(c, V0);
        c.g[T0][0] = c.lw(at(c, V1, 0x170));
        call_ptr(c, target, 0x0037_58ec)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bezier patch cache.

/// `void BezierPatchCache_MarkUsed(Cache* self, int count, Item* items /*a2*/,
/// int idx /*a3*/, int bit /*t0*/, int stride /*t1*/)` (0x374a88): set bit
/// `bit` (and the two low bits) in the `+4` word of each item's block in
/// `self->lists[idx]`, skipping items whose short index at +0xc is negative.
pub fn bezier_patch_cache_mark_used(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x1;
    c.g[V0][0] = sx(1u32 << (r32(c, T0) & 31));
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[T0][0] = c.g[V0][0] | 0xc;
    if !empty {
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A3) << 2);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x18));
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T1][0] = 0xc;
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.lh(at(c, A2, 0xc));
            let skipped = (c.g[A0][0] as i64) < 0;
            c.mult(0, A0, A0, T1);
            if !skipped {
                c.cycles += 5;
                c.g[V1][0] = c.lw(r32(c, A3));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] |= c.g[T0][0];
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
            let more = c.g[A1][0] != 0;
            addi(c, A2, 0x10);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_4aa8);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World painter getters.

/// The lens-fx / render-state entry table `D_004FA370` (built as
/// `0x500000 - 0x5c90`) with stride 0xf0; the entry's object pointer is at
/// `+0x14` (lens fx) or `+0x1c` (render state).

/// `float WorldPainter_GetSunFlareIntensity(int i)` (0x2ee268): call virtual
/// slot 34 on the object of lens-fx entry `i` and return the float it
/// returns.
pub fn world_painter_sun_flare_intensity(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x50_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, 0x14));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 0x4));
        c.g[A0][0] = c.lh(at(c, A1, 0x110));
        c.g[V0][0] = c.lw(at(c, A1, 0x114));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002e_e2a0)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLighting30(void* p)` (0x2eefa8): the same through
/// render-state entry `i`'s object at `+0x1c`, virtual slot 42.
pub fn world_painter_get_lighting30(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x50_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, 0x1c));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 0x4));
        c.g[A0][0] = c.lh(at(c, A1, 0x150));
        c.g[V0][0] = c.lw(at(c, A1, 0x154));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002e_efe0)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0 context.

/// `void PS2ExecMan_v12(MemBlockMan* self)` (0x31aa18): save the VU0 context
/// into the block manager's area at `self+0x14080` when its flag at
/// `self+0x14048` is set.
pub fn ps2_exec_man_v12(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x4048));
        let empty = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = 0x1_0000;
            c.g[A0][0] |= 0x4080;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            callee::VU0_SAVE_CONTEXT.call(c, 0x0031_aa48)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void PS2ExecMan_v11(MemBlockMan* self)` (0x31a9d8): restore the VU0
/// context from `self+0x14080` under the same flag.
pub fn ps2_exec_man_v11(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x4048));
        let empty = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = 0x1_0000;
            c.g[A0][0] |= 0x4080;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            callee::VU0_RESTORE_CONTEXT.call(c, 0x0031_aa08)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Irradiance.

/// `void Irradiance_AddScaledW(Vec4* dst, const Vec4* src, float dt /*f12*/)`
/// (0x389620): `dst->w += src->w * dt` for each of the ten vectors.
pub fn irradiance_add_scaled_w(c: &mut Cpu) {
    c.cycles += 4;
    addi(c, A1, 0xc);
    addi(c, A0, 0xc);
    c.g[V0][0] = 0x9;
    loop {
        c.cycles += 9;
        c.lwc1(1, r32(c, A1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
        c.lwc1(0, r32(c, A0));
        addi(c, A1, 0x10);
        c.mul_s(1, 1, 12);
        c.add_s(0, 0, 1);
        c.swc1(0, r32(c, A0));
        let more = (c.g[V0][0] as i64) >= 0;
        addi(c, A0, 0x10);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_9630);
        }
    }
}

// ---------------------------------------------------------------------------
// Modifier nodes.

/// `int ModObj_v45(MoveNode* self)` (0x356a00): the transform bounds of the
/// modifier block at +0x1c (0 when there is none).
pub fn mod_obj_v45(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, A0, 0x1c));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 2;
            callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS.call(c, 0x0035_6a1c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float ModifierBlock_GetTransformRadius(ModifierBlock* mb)` (0x352bf8):
/// virtual slot 14 of the block's node, 0 when there is none.
pub fn modifier_block_get_transform_radius(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, A0));
        if c.g[A1][0] != 0 {
            c.g[V0][0] = c.lw(r32(c, A1));
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.lh(at(c, V0, 0x70));
        c.g[V1][0] = c.lw(at(c, V0, 0x74));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_2c28)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay stream.

/// `void ReplayStream_WriteObjectRef(Stream* out, void* obj)` (0x26ecd8):
/// write the object's id at +0x78 (or -1) as four bytes through the stream's
/// virtual slot 1.
pub fn replay_stream_write_object_ref(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 7;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        let none = c.g[A1][0] == 0;
        let sp = sp(c);
        c.sw(sp, c.g[V0][0]);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c, A1, 0x78));
            c.sw(sp, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, A3));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh(at(c, V0, 0x8));
        c.g[V1][0] = c.lw(at(c, V0, 0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_ed18)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitters.

/// Emitter (descriptor) fields used by `Emitter_Update`.
mod emitter {
    /// Set once the emitter has been (re)started / finished this frame.
    pub const STARTED: u32 = 0xc;
    /// The remaining life is at +0; then the progress accumulator and its
    /// limit, and the rate `dt` is scaled by.
    pub const PROGRESS: u32 = 0x184;
    pub const RATE: u32 = 0x188;
    pub const LIMIT: u32 = 0x180;
}

/// The frame clock object (`D_004A5B64`, `gp+0x2a74`); `+0x10` is the frame
/// rate as an integer.
const CLOCK: i32 = 0x2a74;

/// `void Emitter_Update(Emitter* e, float dt /*f12*/)` (0x370788): run the
/// emitter's countdown; when it has run out (a negative remaining time) the
/// descriptor is restarted, otherwise its clock is advanced and its remaining
/// life reduced, and the progress accumulator is advanced by `rate * dt`,
/// wrapping to 0 once it reaches the limit. Marks the emitter started.
///
/// State lives in the original's registers: f21 the constant 0, f22 `dt`.
pub fn emitter_update(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 22, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x20);
        c.mtc1(0, 21);
        c.lwc1(0, r32(c, S0));
        c.c_eq_s(0, 21);
        let dead = c.fcc();
        c.mov_s(22, 12);
        if !dead {
            c.cycles += 4;
            c.c_lt_s(0, 21);
            let expired = c.fcc();
            c.g[V0][0] = c.lw(gp(c, CLOCK));
            if expired {
                // Restart the descriptor: one frame's worth of time.
                c.cycles += 9;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 0, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_UPDATE_DESCRIPTOR.call(c, 0x0037_07e8)?;
                c.lwc1(0, at(c, S0, emitter::RATE));
            } else {
                // Advance its clock, then its remaining life.
                c.cycles += 17;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 20);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 20, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_ADVANCE_DESC_TIME.call(c, 0x0037_080c)?;
                c.g[V0][0] = c.lw(gp(c, CLOCK));
                c.lwc1(0, r32(c, S0));
                c.lwc1(1, at(c, V0, 0x10));
                c.cvt_s_w(1, 1);
                c.div_s(20, 20, 1);
                c.sub_s(0, 0, 20);
                c.c_lt_s(0, 21);
                let negative = c.fcc();
                c.swc1(0, r32(c, S0));
                if negative {
                    c.cycles += 1;
                    c.swc1(21, r32(c, S0));
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, emitter::RATE));
            }
            // progress += rate * dt, back to 0 once it reaches the limit.
            c.cycles += 9;
            c.lwc1(1, at(c, S0, emitter::PROGRESS));
            c.mul_s(0, 0, 22);
            c.g[V1][0] = c.lw(at(c, S0, emitter::LIMIT));
            c.add_s(1, 1, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let running = c.g[V0][0] != 0;
            c.swc1(1, at(c, S0, emitter::PROGRESS));
            if !running {
                c.cycles += 1;
                c.sw(at(c, S0, emitter::PROGRESS), 0);
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(at(c, S0, emitter::STARTED), c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable};
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn rider_is_control_mode3_matches() {
        check(ssx3_ee::s01::f_001448a8__generated, rider_is_control_mode3, 300, &|c, seed| {
            let mut r = Rng(0x1234_5678 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            // 3 and not-3, plus values whose sign extension matters.
            let mode = match seed % 4 {
                0 => 3,
                1 => 0,
                2 => r.next_u32(),
                _ => 0x8000_0003,
            };
            c.bus.write32(DATA + 0x70, mode);
        });
    }

    #[test]
    fn rider_compute_lod_matches() {
        covered(ssx3_ee::s00::f_00121f30__generated, rider_compute_lod, 150, &|c, seed| {
            let mut r = machine(c, seed, 17, 3);
            // D_004A289C and the global vtable (slots 35 and 29).
            let g = DATA + 0x200;
            c.bus.write32(GAME_GP - 0x854, g);
            let gvt = vtable(c, &mut r, &[0x118, 0xe8]);
            c.bus.write32(g + 0x10d8, gvt);
            // The rider and its own vtable (slot 5: the position).
            let rider = DATA + 0x300;
            let rvt = vtable(c, &mut r, &[0x28]);
            c.bus.write32(rider + 0x6c0, rvt);
            c.bus.write32(rider + 0x898, r.next_u32());
            // The 0.01 scale and Math_SinCos' polynomial coefficients.
            put_words(c, GAME_GP - 0x78c4, &[0x3c23_d70a]);
            put_words(
                c,
                GAME_GP - 0x2ec0,
                &[0x3f22_f983, 0x3fc9_0fdb, 0x3638_ef1f, 0xb950_0d03, 0x3c08_8889, 0xbe2a_aaab],
            );
            // A field of view of a plausible size half the time, random else.
            c.g[A0][0] = sx(rider);
        });
    }

    #[test]
    fn ssx_app_next_tick_matches() {
        check(ssx3_ee::s05::f_00227e98__generated, ssx_app_next_tick, 150, &|c, seed| {
            let mut r = machine(c, seed, 23, 0);
            let queue = DATA + 0x4_0000;
            let app = DATA + 0x300;
            // No app half the time; else a pad queue whose read cursor is
            // either still ahead (early out) or one step behind.
            if seed % 5 == 0 {
                c.bus.write32(GAME_GP - 0x850, 0);
            } else {
                c.bus.write32(GAME_GP - 0x850, queue);
                let cursor = if seed % 3 == 0 { r.next_u32() % 30 } else { 0 };
                let count = if seed % 3 == 0 { (cursor + 1) % 30 } else { r.next_u32() % 30 };
                c.bus.write32(queue + 0x2ee0, cursor);
                c.bus.write32(queue + 0x2ee4, count);
            }
            c.bus.write32(GAME_GP - 0x238, 0);
            c.bus.write32(GAME_GP + 0x300, 0);
            for i in 0..2u32 {
                c.bus.write32(app + 0xa8 + i * 4, DATA + 0x800 + i * 0x100);
            }
            c.g[A0][0] = sx(app);
        });
    }

    #[test]
    fn replay_stream_write_object_ref_matches() {
        covered(ssx3_ee::s07::f_0026ecd8__generated, replay_stream_write_object_ref, 150, &|c, seed| {
            let mut r = machine(c, seed, 41, 1);
            let out = DATA + 0x100;
            let vt = vtable(c, &mut r, &[0x8]);
            c.bus.write32(out, vt);
            if seed % 2 == 0 {
                let obj = DATA + 0x400;
                c.bus.write32(obj + 0x78, r.next_u32());
                c.g[A1][0] = sx(obj);
            } else {
                c.g[A1][0] = 0;
            }
            c.g[A0][0] = sx(out);
        });
    }

    #[test]
    fn dynamic_spray_fx_render_matches() {
        check(ssx3_ee::s09::f_002e24d0__generated, dynamic_spray_fx_render, 150, &|c, seed| {
            machine(c, seed, 53, 0);
            let spray = DATA + 0x1000;
            let owner = DATA + 0x2000;
            c.bus.write32(spray, owner);
            c.bus.write32(owner + 0xb18, if seed % 4 == 0 { 0 } else { 1 });
            let wakes = DATA + 0x3000;
            c.bus.write32(spray + 0x28, wakes);
            for i in 0..10u32 {
                let w = wakes + i * 0x210;
                match (seed + i) % 4 {
                    0 => {}
                    1 => {
                        c.bus.write32(w + 0x174, 1);
                    }
                    2 => {
                        c.bus.write32(w + 0x174, 1);
                        c.bus.write32(w + 0x1e0, 0xffff_ffff);
                    }
                    _ => {
                        c.bus.write32(w + 0x174, 1);
                        c.bus.write32(w + 0x1e0, 1);
                    }
                }
            }
            // mEmitter_Render leaves at its negative table entry.
            let g = DATA + 0x6000;
            c.bus.write32(GAME_GP + 0x2a90, g);
            for i in 0..0x20u32 {
                c.bus.write32(g + 0xf50 + i * 4, 0xffff_ffff);
            }
            c.g[A0][0] = sx(spray);
        });
    }

    #[test]
    fn world_painter_sun_flare_intensity_matches() {
        covered(ssx3_ee::s09::f_002ee268__generated, world_painter_sun_flare_intensity, 120, &|c, seed| {
            let mut r = machine(c, seed, 47, 1);
            let i = r.next_u32() % 3;
            let slot = 0x004f_a370 + i * 0xf0;
            let cell = DATA + 0x400 + i * 0x100;
            let obj = DATA + 0x800 + i * 0x100;
            let vt = vtable(c, &mut r, &[0x110]);
            c.bus.write32(obj, vt);
            c.bus.write32(cell, obj);
            c.bus.write32(slot + 0x14, cell);
            c.g[A0][0] = sx(i);
        });
    }

    #[test]
    fn world_painter_get_lighting30_matches() {
        covered(ssx3_ee::s09::f_002eefa8__generated, world_painter_get_lighting30, 120, &|c, seed| {
            let mut r = machine(c, seed, 59, 1);
            let i = r.next_u32() % 3;
            let slot = 0x004f_a370 + i * 0xf0;
            let cell = DATA + 0x400 + i * 0x100;
            let obj = DATA + 0x800 + i * 0x100;
            let vt = vtable(c, &mut r, &[0x150]);
            c.bus.write32(obj, vt);
            c.bus.write32(cell, obj);
            c.bus.write32(slot + 0x1c, cell);
            c.g[A0][0] = sx(i);
        });
    }

    #[test]
    fn ps2_exec_man_v12_matches() {
        check(ssx3_ee::s10::f_0031aa18__generated, ps2_exec_man_v12, 60, &|c, seed| {
            let mut r = machine(c, seed, 61, 0);
            c.bus.write32(DATA + 0x14048, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x14080, r.next_u32());
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn ps2_exec_man_v11_matches() {
        check(ssx3_ee::s10::f_0031a9d8__generated, ps2_exec_man_v11, 60, &|c, seed| {
            let mut r = machine(c, seed, 67, 0);
            c.bus.write32(DATA + 0x14048, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x14080, r.next_u32());
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn modifier_block_get_transform_radius_matches() {
        covered(
            ssx3_ee::s11::f_00352bf8__generated,
            modifier_block_get_transform_radius,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 71, 1);
                let mb = DATA + 0x100;
                if seed % 2 == 0 {
                    c.bus.write32(mb, 0);
                } else {
                    let node = DATA + 0x400;
                    let vt = vtable(c, &mut r, &[0x70]);
                    c.bus.write32(node, vt);
                    c.bus.write32(mb, node);
                }
                c.g[A0][0] = sx(mb);
            },
        );
    }

    #[test]
    fn mod_obj_v45_matches() {
        covered(ssx3_ee::s11::f_00356a00__generated, mod_obj_v45, 150, &|c, seed| {
            let mut r = machine(c, seed, 73, 1);
            let movenode = DATA + 0x100;
            if seed % 3 == 0 {
                c.bus.write32(movenode + 0x1c, 0);
            } else {
                let node = DATA + 0x400;
                let vt = vtable(c, &mut r, &[0x60]);
                c.bus.write32(node, vt);
                c.bus.write32(movenode + 0x1c, node);
            }
            c.g[A0][0] = sx(movenode);
        });
    }

    #[test]
    fn emitter_update_matches() {
        check(ssx3_ee::s12::f_00370788__generated, emitter_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 79, 0);
            let e = DATA;
            c.g[A0][0] = sx(e);
            c.f[12] = if seed % 2 == 0 { r.edge_f32() } else { r.f32(1.0) };
            // Remaining life: 0 (idle), negative (restart) or positive.
            let remain = match seed % 3 {
                0 => 0,
                1 => 0xbf80_0000 | (r.next_u32() & 0x003f_ffff),
                _ => r.f32(120.0),
            };
            c.bus.write32(e, remain);
            // The descriptor at e+0x10: all zero, so both callees take their
            // short paths (progress 0, limit 0, rate 0).
            for off in [
                0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
                0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c,
            ] {
                c.bus.write32(e + off, 0);
            }
            // Progress, its limit and the rate `dt` scales.
            let limit = (r.next_u32() % 6) as i32;
            c.bus.write32(e + 0x180, limit as u32);
            c.bus.write32(e + 0x184, if seed % 5 == 0 { 0 } else { r.edge_f32() });
            c.bus.write32(e + 0x188, if seed % 4 == 0 { 0 } else { r.edge_f32() });
            c.bus.write32(e + 0xc, r.next_u32());
            // The frame clock object.
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x310, if seed % 7 == 0 { 1 } else { 30 + r.next_u32() % 60 });
        });
    }

    #[test]
    fn bezier_patch_cache_mark_used_matches() {
        check(ssx3_ee::s12::f_00374a88__generated, bezier_patch_cache_mark_used, 200, &|c, seed| {
            let mut r = Rng(0x2f19_a4c3 ^ seed as u64);
            scramble(c, &mut r);
            let cache = DATA;
            let items = DATA + 0x1000;
            let lists = DATA + 0x2000;
            let idx = r.next_u32() % 4;
            let bit = r.next_u32() % 32;
            let count = seed % 5;
            for i in 0..4u32 {
                let v: i32 = match (seed + i) % 4 {
                    0 => 0,
                    1 => -((r.next_u32() % 4) as i32) - 1,
                    2 => 3,
                    _ => (r.next_u32() % 3) as i32,
                };
                c.bus.write16(items + i * 0x10 + 0xc, v as u16);
            }
            c.bus.write32(cache + 0x18 + idx * 4, lists);
            for k in 0..4u32 {
                c.bus.write32(lists + k * 0xc + 4, r.next_u32());
            }
            c.g[A0][0] = sx(cache);
            c.g[A1][0] = sx(count);
            c.g[A2][0] = sx(items);
            c.g[A3][0] = sx(idx);
            c.g[T0][0] = sx(bit);
            c.g[T1][0] = 0xc;
        });
    }

    #[test]
    fn boader_shadow_submit_matches() {
        covered(ssx3_ee::s12::f_00375890__generated, boader_shadow_submit, 150, &|c, seed| {
            let mut r = machine(c, seed, 83, 1);
            let g = DATA + 0x200;
            c.bus.write32(GAME_GP + 0x2a90, g);
            let vt = vtable(c, &mut r, &[0x218]);
            c.bus.write32(g + 0x10d8, vt);
            let s = DATA + 0x600;
            c.bus.write32(s + 0xc, r.f32(100.0));
            c.bus.write32(s + 0x10, r.f32(100.0));
            c.bus.write32(s + 0x14, r.f32(100.0));
            c.bus.write32(s + 0x170, r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn irradiance_add_scaled_w_matches() {
        check(ssx3_ee::s12::f_00389620__generated, irradiance_add_scaled_w, 200, &|c, seed| {
            let mut r = Rng(0x7a3f_19d5 ^ (seed as u64) << 5);
            scramble(c, &mut r);
            let src = DATA + 0x1000;
            let dst = DATA + 0x2000;
            for i in 0..10u32 {
                for lane in 0..3u32 {
                    c.bus.write32(src + i * 0x10 + lane * 4, r.next_u32());
                    c.bus.write32(dst + i * 0x10 + lane * 4, r.next_u32());
                }
                c.bus.write32(src + i * 0x10 + 0xc, r.edge_f32());
                c.bus.write32(dst + i * 0x10 + 0xc, r.edge_f32());
            }
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            c.f[12] = if seed % 3 == 0 { r.edge_f32() } else { r.f32(2.0) };
        });
    }
}
