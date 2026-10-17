//! Work package E8: world cache, sections, CPU riders, animation objects
//! (`docs/investigation/phase2-candidates-3.md`, structs in
//! `docs/investigation/structs/{world,rider}.md`).
//!
//! - **Section streamer views** (`cWorldCache` section streamer, embedded
//!   at `cWorldCache+0x10`): per view (0x50 bytes at `+0x240 + 0x50*view`)
//!   an active flag, a position (+0x250), the view's bounding box
//!   (+0x260/+0x270), its range (+0x280), the clearance found (+0x284) and
//!   its minimum (+0x288). Updating a view walks every section's node tree
//!   (`Section+0x1c`), requests the hulls of leaves near the view with a
//!   priority, and keeps the nearest unloaded distance as the clearance.
//! - **AnimController** layers: six animation sequence lists at `+0x50`
//!   (8 bytes each) and the current animation id per layer at `+4*layer`
//!   (0x1b6 = none): layer queries, fading a layer out, the end-of-sequence
//!   event dispatch, rotating the root bones.
//! - **AnimSequence** time advance with trigger firing (loop/clamp modes).
//! - **AnimObject** (animated world object) and **MultiSplineModifier**
//!   virtual methods: save, update/apply passes over the object's nodes.
//! - **ComputerActorRider** (CPU racer): the control-input dispatcher over
//!   the rider sub-state, AI path marker searches, the reset heuristics.
//!
//! The replacements follow the call protocol of [`crate::call`] (frame,
//! registers at each call, unwind check, cycles between calls) and the
//! structured-loop event rule of [`crate::events`]. Loop state lives in the
//! guest registers the original uses and is read back after each back-edge
//! check.

use crate::abi::{addu, c_lt_s, r32, ret, set_fpr};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::guest::Addr;
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::{vu0_scratch, FpuF32, Vec4};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_9800, streamer_get_section_count, "cWorldCache_StreamerGetSectionCount"),
    (0x003a_9d60, streamer_update_view, "cWorldCache_StreamerUpdateView"),
    (0x0031_1b20, anim_controller_get_layer_sequence, "AnimController_GetLayerSequence"),
    (0x0031_1ae8, anim_controller_get_layer_anim_entry, "AnimController_GetLayerAnimEntry"),
    (0x0031_2ae8, anim_controller_is_layer_anim_finished, "AnimController_IsLayerAnimFinished"),
    (0x0031_1e88, anim_controller_fade_out_layer, "AnimController_FadeOutLayer"),
    (0x0031_1b48, anim_controller_rotate_roots, "AnimController_RotateRoots"),
    (0x0035_a5d8, multi_spline_modifier_v03, "MultiSplineModifier_v03"),
    (0x0035_abf0, multi_spline_modifier_v04, "MultiSplineModifier_v04"),
    (0x0036_1c20, multi_spline_modifier_v18, "MultiSplineModifier_v18"),
    (0x0034_dc90, anim_object_v51, "AnimObject_v51"),
    (0x0034_2150, anim_object_save, "AnimObject_Save"),
    (0x0034_e3d8, anim_object_base_save, "AnimObject_BaseSave"),
    (0x0034_1d48, anim_object_v15, "AnimObject_v15"),
];

// ---------------------------------------------------------------------------
// Small register helpers.

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A gcc 2.9x virtual call `vt[slot](obj + delta, ...)`: reads the slot
/// (`lh` delta, `lw` function), leaves the function in `func_reg` and the
/// adjusted `this` in a0, as the originals do, and calls through the
/// function table.
#[inline]
fn virtual_call(c: &mut Cpu, obj: u64, vt: u64, slot: u32, func_reg: usize, ret: u32) -> Flow {
    let m = VSlot::read(c, vt, slot);
    c.g[func_reg][0] = m.func;
    c.g[A0][0] = m.this(obj);
    call_ptr(c, m.func as u32, ret)
}

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const ANIM_SEQ_LIST_GET_AT: Callee = Callee { addr: 0x0031_4760, f: ee::s10::AnimSeqList_GetAt };
    pub const ANIM_TABLE_GET_ENTRY: Callee = Callee { addr: 0x0031_15c8, f: ee::s10::AnimTable_GetEntry };
    pub const ANIM_SEQ_LIST_REMOVE_EVENT_TRIGGER: Callee =
        Callee { addr: 0x0031_4718, f: ee::s10::AnimSeqList_RemoveEventTrigger };
    pub const ANIM_SEQ_LIST_FADE_OUT_ALL: Callee =
        Callee { addr: 0x0031_46d0, f: ee::s10::AnimSeqList_FadeOutAll };
    pub const MATH_SIN_COS: Callee = Callee { addr: 0x0031_be50, f: ee::s10::Math_SinCos };
    pub const ANIM_CONTROLLER_TRANSFORM_ROOTS: Callee =
        Callee { addr: 0x0031_1bf0, f: ee::s10::AnimController_TransformRoots };

    pub const STREAMER_UPDATE_VIEW: Callee = Callee { addr: 0x003a_9d60, f: ee::s13::f_003a9d60 };
    pub const STREAMER_UPDATE_VIEW_NODE: Callee =
        Callee { addr: 0x003a_a028, f: ee::s13::f_003aa028 };
    pub const STREAMER_POINT_BOX_DISTANCE: Callee =
        Callee { addr: 0x003a_9e50, f: ee::s13::cWorldCache_PointBoxDistance };
    pub const STREAMER_IS_HULL_RESIDENT: Callee =
        Callee { addr: 0x003a_8618, f: ee::s13::cWorldCache_IsHullResident };
    pub const STREAMER_REQUEST_HULL: Callee =
        Callee { addr: 0x003a_8448, f: ee::s13::cWorldCache_RequestHull };

    pub const RIDER_GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    pub const RIDER_GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    pub const ANIM_SEQ_FIRE_TRIGGERS_AT_TIME: Callee =
        Callee { addr: 0x0031_3938, f: ee::s10::AnimSequence_FireTriggersAtTime };
    pub const ANIM_SEQ_FIRE_END_TRIGGERS: Callee =
        Callee { addr: 0x0031_39a8, f: ee::s10::AnimSequence_FireEndTriggers };

    pub const MULTI_SPLINE_MODIFIER_UPDATE_NODES: Callee =
        Callee { addr: 0x0035_ac20, f: ee::s11::MultiSplineModifier_UpdateNodes };
    pub const ANIM_OBJECT_BASE_SAVE: Callee = Callee { addr: 0x0034_e3d8, f: ee::s11::AnimObject_BaseSave };
    pub const MOD_OBJ_SAVE: Callee = Callee { addr: 0x0035_6b30, f: ee::s11::ModObj_Save };
    pub const ANIM_OBJECT_STEP_LOOP: Callee = Callee { addr: 0x0034_1e48, f: ee::s11::AnimObject_StepLoop };
    pub const ANIM_OBJECT_STEP_PING_PONG: Callee =
        Callee { addr: 0x0034_1ec0, f: ee::s11::AnimObject_StepPingPong };
    pub const ANIM_OBJECT_STEP_CLAMP: Callee = Callee { addr: 0x0034_1f38, f: ee::s11::AnimObject_StepClamp };
    /// Unnamed 0x34eba0: runs the object's script hook, true to go on.
    pub const ANIM_OBJECT_SCRIPT_CONTINUES: Callee = Callee { addr: 0x0034_eba0, f: ee::s11::f_0034eba0 };
    /// Unnamed 0x345430: runs a spline modifier's attached script.
    pub const RUN_SPLINE_SCRIPT: Callee = Callee { addr: 0x0034_5430, f: ee::s11::f_00345430 };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
mod table_callee {
    pub const WORLD_GET_NASH: u32 = 0x002d_1be0;
    pub const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;
    /// Unnamed 0x1446a0: `(*(u64*)a0 >> a1) & 1`.
    pub const TEST_BIT64: u32 = 0x0014_46a0;
}

// ---------------------------------------------------------------------------
// Section streamer.

/// `SectionStreamer` (embedded at `cWorldCache+0x10`).
mod streamer {
    /// The section database header (`+8`: number of sections).
    pub const DB: u32 = 0x04;
    pub const DB_SECTION_COUNT: u32 = 0x08;
}

/// `int cWorldCache_StreamerGetSectionCount(SectionStreamer*)` (0x3a9800):
/// the number of sections in the bound database, 0 without one.
pub fn streamer_get_section_count(c: &mut Cpu) {
    c.cycles += 3;
    let db = lw(c, at(c, A0, streamer::DB));
    c.g[V0][0] = db;
    c.cycles += 2;
    c.g[V0][0] = if db == 0 { 0 } else { lw(c, (db as u32).wrapping_add(streamer::DB_SECTION_COUNT)) };
    ret(c);
}

/// `bool cWorldCache_StreamerGetViewClearance(streamer, view, Vec4 *out,
/// float *clearance)` (0x3a96e0): updates the view, copies the view position
/// (`+0x250`) to `*out` and `clearance - 0.5` to `*clearance`, and returns
/// whether 0 is below that value. Inactive views (`+0x240 == 0`) return 0
/// without writing the outputs.
pub fn streamer_get_view_clearance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x50;
        let frame = Frame::open(c, 0x40);
        c.mult(0, V1, A1, V0);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = addu(c.g[V1][0], c.g[A0][0]);
        c.g[V0][0] = lw(c, at(c, S0, 0x240));
        let active = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if !active {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 14;
            callee::STREAMER_UPDATE_VIEW.call(c, 0x003a_9720)?;
            c.lq(V0, r32(c, S0).wrapping_add(0x250));
            c.g[V1][0] = 1;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[AT][0] as u32;
            c.sq(V0, r32(c, S1));
            c.f[1] = c.g[ZERO][0] as u32;
            c.f[0] = Addr(r32(c, S0).wrapping_add(0x284)).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
            let below = c_lt_s(c, FpuF32(c.f[1]), FpuF32(c.f[0]));
            Addr(r32(c, S2)).write_fpu(c, FpuF32(c.f[0]));
            if !below {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `View` record fields (`+0x240 + 0x50*view`): active flag, position, the
/// two bounding boxes, the range, the clearance found and its minimum.
mod view {
    pub const ACTIVE: u32 = 0x240;
    pub const POSITION: u32 = 0x250;
    pub const BOX_MIN: u32 = 0x260;
    pub const BOX_MAX: u32 = 0x270;
    pub const RANGE: u32 = 0x280;
    pub const CLEARANCE: u32 = 0x284;
    pub const MINIMUM: u32 = 0x288;
    pub const VIEW_BYTES: u32 = 0x50;
}

/// `void cWorldCache_StreamerUpdateView(streamer, view)` (0x3a9d60):
/// resets the view's clearance (and minimum) to its range, then for every
/// active section (`+0x14 + 8*i` class 5 or 6) asks
/// `cWorldCache_StreamerUpdateViewNode(streamer, node, view, id == 6)` to
/// tighten it. Writes the graphics manager's counter to `+0x248` and clears
/// `+0x244`. Inactive views are left alone.
pub fn streamer_update_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x50;
        frame.save(c, S4, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.mult(0, V1, S4, V0);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = addu(c.g[V1][0], c.g[S0][0]);
        c.g[V1][0] = lw(c, at(c, A0, view::ACTIVE));
        let active = c.g[V1][0] != 0;
        c.g[V0][0] = lw(c, at(c, GP, 0x2a74));
        if !active {
            c.cycles += 9;
            frame.restore(c, S0, 0x60);
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            frame.restore(c, S4, 0x20);
            frame.restore(c, S5, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.f[0] = Addr(at(c, A0, view::RANGE)).read_fpu(c).0;
        c.g[S5][0] = lw(c, at(c, V0, 0x18));
        Addr(at(c, A0, view::MINIMUM)).write_fpu(c, FpuF32(c.f[0]));
        Addr(at(c, A0, view::CLEARANCE)).write_fpu(c, FpuF32(c.f[0]));
        c.g[V0][0] = lw(c, at(c, S0, 0x4));
        c.g[V1][0] = lw(c, at(c, V0, 0x8));
        let has = c.g[V1][0] != 0;
        c.g[S2][0] = 0;
        if has {
            c.cycles += 2;
            c.g[S3][0] = 0;
            c.g[S1][0] = sx(at(c, S0, 0x14));
            loop {
                c.cycles += 5;
                c.g[A3][0] = lw(c, r32(c, S1));
                c.g[V0][0] = sx(r32(c, A3).wrapping_sub(5));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let skip = c.g[V0][0] == 0;
                c.g[A3][0] = c.g[A3][0] ^ 6;
                if !skip {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c, S0, 0x8));
                    c.g[A3][0] = (c.g[A3][0] < 1) as u64;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A1][0] = lw(c, at(c, V0, 0x1c));
                    callee::STREAMER_UPDATE_VIEW_NODE.call(c, 0x003a_9df8)?;
                }
                c.cycles += 7;
                c.g[V1][0] = lw(c, at(c, S0, 0x4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                c.g[V0][0] = lw(c, at(c, V1, 0x8));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_9dc8);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x50;
        c.mult(0, V1, S4, V0);
        c.g[V0][0] = addu(c.g[V1][0], c.g[S0][0]);
        Addr(r32(c, V0)).off(0x248).sw(c, c.g[S5][0]);
        Addr(r32(c, V0)).off(0x244).sw(c, 0);
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimController.

/// `AnimController` fields.
mod anim_controller {
    /// `int animId[6]`: the animation playing on each layer.
    pub const ANIM_ID: u32 = 0x00;
    /// `AnimSeqList* layers` (8 bytes per layer).
    pub const LAYERS: u32 = 0x50;
    pub const LAYER_SIZE_SHIFT: u32 = 3;
    /// `animId` of an empty layer.
    pub const NO_ANIM: u64 = 0x1b6;
    /// `AnimSequence+0xc0`: the sequence has finished.
    pub const SEQ_FINISHED: u32 = 0xc0;
}

/// `AnimSequence* AnimController_GetLayerSequence(ctrl, layer)` (0x311b20):
/// the first sequence of the layer's list.
pub fn anim_controller_get_layer_sequence(c: &mut Cpu) {
    guest(c, |c| {
        layer_first_sequence(c, 10, 0x0031_1b3c)?;
        Frame { size: 0x10 }.restore_ra(c, 0);
        Frame { size: 0x10 }.close(c);
        Ok(())
    })
}

/// The shared prologue of the two layer queries: a 0x10 frame and
/// `AnimSeqList_GetAt(&ctrl->layers[layer], 0)`.
fn layer_first_sequence(c: &mut Cpu, cycles: u64, ret_addr: u32) -> Flow {
    use anim_controller::*;
    c.cycles += cycles;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(r32(c, A1) << LAYER_SIZE_SHIFT);
    frame.save_ra(c, 0);
    c.g[A1][0] = 0;
    let layers = lw(c, at(c, A0, LAYERS));
    c.g[A0][0] = addu(layers, c.g[V0][0]);
    callee::ANIM_SEQ_LIST_GET_AT.call(c, ret_addr)
}

/// `bool AnimController_IsLayerAnimFinished(ctrl, layer)` (0x312ae8): the
/// finished flag of the layer's first sequence.
pub fn anim_controller_is_layer_anim_finished(c: &mut Cpu) {
    guest(c, |c| {
        layer_first_sequence(c, 11, 0x0031_2b04)?;
        c.g[V0][0] = lw(c, at(c, V0, anim_controller::SEQ_FINISHED));
        Frame { size: 0x10 }.restore_ra(c, 0);
        Frame { size: 0x10 }.close(c);
        Ok(())
    })
}

/// `AnimTableEntry AnimController_GetLayerAnimEntry(ctrl, layer)`
/// (0x311ae8): the first word of the animation table entry of the layer's
/// animation, 0 when the layer has none.
pub fn anim_controller_get_layer_anim_entry(c: &mut Cpu) {
    use anim_controller::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = addu(c.g[A0][0], c.g[A1][0]);
        c.g[V0][0] = NO_ANIM;
        let id = lw(c, at(c, A0, ANIM_ID));
        c.g[A0][0] = id;
        if id == NO_ANIM {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            callee::ANIM_TABLE_GET_ENTRY.call(c, 0x0031_1b10)?;
            c.g[V0][0] = lw(c, r32(c, V0));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimController_FadeOutLayer(ctrl, layer, float time)` (0x311e88):
/// removes the layer's end-event triggers (`AnimSeqList_RemoveEventTrigger(
/// list, 0x3f)`), fades all its sequences out over `time` and marks the
/// layer empty.
pub fn anim_controller_fade_out_layer(c: &mut Cpu) {
    use anim_controller::*;
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(r32(c, S1) << LAYER_SIZE_SHIFT);
        frame.save_fpr(c, 20, 0x40);
        c.g[A1][0] = 0x3f;
        c.f[20] = c.f[12];
        let layers = lw(c, at(c, S0, LAYERS));
        c.g[A0][0] = addu(layers, c.g[S2][0]);
        callee::ANIM_SEQ_LIST_REMOVE_EVENT_TRIGGER.call(c, 0x0031_1ec0)?;
        let layers = lw(c, at(c, S0, LAYERS));
        c.f[12] = c.f[20];
        c.g[A0][0] = addu(layers, c.g[S2][0]);
        callee::ANIM_SEQ_LIST_FADE_OUT_ALL.call(c, 0x0031_1ed0)?;
        c.g[S1][0] = sx(r32(c, S1) << 2);
        c.g[V0][0] = NO_ANIM;
        c.g[S0][0] = addu(c.g[S0][0], c.g[S1][0]);
        frame.restore(c, S2, 0x10);
        Addr(r32(c, S0)).sw(c, NO_ANIM);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The zero translation and the root rotation axis `AnimController_
/// RotateRoots` uses (globals at 0x4ff130 and 0x4ff160).
const ROOT_TRANSLATION: u32 = 0x004f_f130;
const ROOT_AXIS: u32 = 0x004f_f160;

/// `void AnimController_RotateRoots(ctrl, float angle)` (0x311b48): rotates
/// the root bones by `-angle` about the root axis: builds the transform
/// `{translation, quaternion (axis * sin(-angle/2), cos(-angle/2))}` on the
/// stack and calls `AnimController_TransformRoots(ctrl, &transform)`.
pub fn anim_controller_rotate_roots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x80);
        let neg_angle = -FpuF32(c.f[12]);
        c.f[0] = neg_angle.0;
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0x40);
        c.g[V0][0] = sx(ROOT_TRANSLATION);
        c.g[S0][0] = 0x0050_0000;
        c.g[S2][0] = c.g[A0][0];
        c.g[AT][0] = 0x3f00_0000;
        c.g[A0][0] = sx(frame.at(c, 0x30));
        c.lq(V1, ROOT_TRANSLATION);
        c.g[A1][0] = sx(frame.at(c, 0x34));
        let half = neg_angle * FpuF32(0x3f00_0000);
        c.f[12] = half.0;
        c.g[S1][0] = sx(ROOT_AXIS);
        let translation_at = frame.at(c, 0);
        c.sq(V1, translation_at);
        // Math_SinCos(half, &sin, &cos).
        callee::MATH_SIN_COS.call(c, 0x0031_1b94)?;
        let axis_z = Addr(ROOT_AXIS + 8).read_fpu(c);
        c.g[A0][0] = c.g[S2][0];
        let sin = Addr(frame.at(c, 0x30)).read_fpu(c);
        c.g[A1][0] = c.g[SP][0];
        let axis_x = Addr(ROOT_AXIS).read_fpu(c);
        let axis_y = Addr(ROOT_AXIS + 4).read_fpu(c);
        let (qz, qx, qy) = (sin * axis_z, sin * axis_x, sin * axis_y);
        let cos = Addr(frame.at(c, 0x34)).read_fpu(c);
        // Leftovers: f1 = qz, f2 = qx, f4 = axis y, f3 = cos, f0 = qy.
        set_fpr(c, 1, qz);
        set_fpr(c, 2, qx);
        set_fpr(c, 4, axis_y);
        set_fpr(c, 3, cos);
        set_fpr(c, 0, qy);
        let q = frame.at(c, 0x20);
        Addr(q + 0xc).write_fpu(c, cos);
        Addr(q + 8).write_fpu(c, qz);
        Addr(q).write_fpu(c, qx);
        Addr(q + 4).write_fpu(c, qy);
        c.lq(V0, q);
        let rot = frame.at(c, 0x10);
        c.sq(V0, rot);
        callee::ANIM_CONTROLLER_TRANSFORM_ROOTS.call(c, 0x0031_1bd8)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MultiSplineModifier / AnimObject virtual methods.

/// `MultiSplineModifier` fields.
mod multi_spline {
    /// Owner `ModObj*`; `owner+0xc` is the world object, whose vtable
    /// (`+0xc`) slot 0x198 is called by `v04`.
    pub const OWNER: u32 = 0x40;
    /// Non-zero when the spline set must be rebuilt first (slot 0x18).
    pub const DIRTY: u32 = 0x30;
    /// The spline array (`v18` returns `&spline[0]->data`).
    pub const SPLINES: u32 = 0x44;
    pub const SLOT_REBUILD: u32 = 0x18;
}

/// `MultiSplineModifier_v03(this)` (0x35a5d8): forwards to
/// `MultiSplineModifier_UpdateNodes(this)`.
pub fn multi_spline_modifier_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MULTI_SPLINE_MODIFIER_UPDATE_NODES.call(c, 0x0035_a5e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MultiSplineModifier_v04(this)` (0x35abf0): calls virtual slot 0x198 of
/// the owner's world object (`this->owner->object`).
pub fn multi_spline_modifier_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let owner = lw(c, at(c, A0, multi_spline::OWNER));
        c.g[V0][0] = owner;
        let object = lw(c, (owner as u32).wrapping_add(0xc));
        c.g[A1][0] = object;
        let vt = lw(c, (object as u32).wrapping_add(0xc));
        c.g[V1][0] = vt;
        virtual_call(c, object, vt, 0x198, V0, 0x0035_ac14)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MultiSplineModifier_v18(this)` (0x361c20): rebuilds the splines if
/// dirty (virtual slot 0x18), then returns `spline[0] + 0x10`.
pub fn multi_spline_modifier_v18(c: &mut Cpu) {
    use multi_spline::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, at(c, S0, DIRTY));
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            let vt = lw(c, r32(c, S0));
            c.g[V0][0] = vt;
            let this = c.g[S0][0];
            virtual_call(c, this, vt, SLOT_REBUILD, V1, 0x0036_1c50)?;
        }
        c.g[V1][0] = lw(c, at(c, S0, SPLINES));
        c.cycles += 6;
        frame.restore(c, S0, 0x10);
        let first = lw(c, r32(c, V1));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((first as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `AnimObject_v51(this)` (0x34dc90): updates the object's animation and
/// applies it: `vt[0x1a0](this)`, then the animation component's (`+0x14`,
/// vtable `this->vt`) slot 0xc0, then its slot 0x108 method with
/// `(result, this->+0x44)`; finally clears bit 0 of the flags at +0x26.
pub fn anim_object_v51(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let vt = lw(c, at(c, S1, 0x20));
        c.g[V0][0] = vt;
        let this = c.g[S1][0];
        virtual_call(c, this, vt, 0x1a0, V1, 0x0034_dcbc)?;
        let vt = lw(c, at(c, S1, 0x20));
        c.g[S0][0] = vt;
        let component = sx(at(c, S1, 0x14));
        c.g[V1][0] = component;
        let apply_delta = Addr((vt as u32).wrapping_add(0x108)).lh(c);
        c.g[S2][0] = addu(component, apply_delta);
        let get = VSlot::read(c, vt, 0xc0);
        c.g[V0][0] = get.func;
        c.g[A0][0] = get.this(component);
        call_ptr(c, get.func as u32, 0x0034_dcdc)?;
        let func = lw(c, at(c, S0, 0x10c));
        c.g[V1][0] = func;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = lw(c, at(c, S1, 0x44));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, func as u32, 0x0034_dcf0)?;
        let flags = Addr(at(c, S1, 0x26)).lhu(c) & 0xfffe;
        c.g[V0][0] = flags;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        Addr(at(c, S1, 0x26)).sh(c, flags);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Write `size` bytes at `data` through the stream's virtual `Write`
/// (`stream->vt` slot 8: `Write(stream, data, size)`).
fn stream_write(c: &mut Cpu, stream: u64, data: u64, size: u64, ret_addr: u32) -> Flow {
    let vt = lw(c, stream as u32);
    c.g[V1][0] = vt;
    c.g[A1][0] = data;
    c.g[A2][0] = size;
    virtual_call(c, stream, vt, 0x8, V0, ret_addr)
}

/// `AnimObject_Save(this, stream)` (0x342150): saves the base part
/// (`AnimObject_BaseSave(this + 0x1c, stream)`), then writes the first 0x1c
/// bytes of the object.
pub fn anim_object_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(at(c, S1, 0x1c));
        callee::ANIM_OBJECT_BASE_SAVE.call(c, 0x0034_2170)?;
        let (stream, this) = (c.g[S0][0], c.g[S1][0]);
        stream_write(c, stream, this, 0x1c, 0x0034_218c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimObject_BaseSave(this, stream)` (0x34e3d8): `ModObj_Save(this + 0x14,
/// stream)`, writes the first 0x14 bytes, then 4 bytes from the stack slot
/// at `sp` (whatever the frame holds there; the original passes its own
/// uninitialised local).
pub fn anim_object_base_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(at(c, S1, 0x14));
        callee::MOD_OBJ_SAVE.call(c, 0x0034_e3f8)?;
        let (stream, this) = (c.g[S0][0], c.g[S1][0]);
        stream_write(c, stream, this, 0x14, 0x0034_e414)?;
        let (stream, local) = (c.g[S0][0], c.g[SP][0]);
        stream_write(c, stream, local, 0x4, 0x0034_e430)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `if (this->dirty) this->vt[0x18](this)` (the rebuild check at the start
/// of the MultiSplineModifier methods; `this` in `this_reg`).
fn rebuild_if_dirty(c: &mut Cpu, this_reg: usize, ret_addr: u32) -> Flow {
    c.g[V0][0] = lw(c, at(c, this_reg, multi_spline::DIRTY));
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        let vt = lw(c, r32(c, this_reg));
        c.g[V0][0] = vt;
        let this = c.g[this_reg][0];
        virtual_call(c, this, vt, multi_spline::SLOT_REBUILD, V1, ret_addr)?;
    }
    Ok(())
}

/// `AnimObject` node array (`v52`): `count` at +0x40, `nodes` (0xd0 bytes
/// each) at +0x48; each node's animation component at +0x84.
mod anim_nodes {
    pub const TIME: u32 = 0x04;
    pub const UPDATED: u32 = 0x10;
    pub const COUNT: u32 = 0x40;
    pub const NODES: u32 = 0x48;
    pub const NODE_SIZE: u32 = 0xd0;
    pub const NODE_COMPONENT_VPTR: u32 = 0x84;
    pub const SLOT_SET_TIME: u32 = 0x10;
}

/// `AnimObject_v52(this)` (0x34e348): sets every node's animation time
/// (`node->vt[0x10](node, this->time)`, vtable at node+0x84) and marks the
/// object updated (`+0x10 = 1`).
pub fn anim_object_v52(c: &mut Cpu) {
    use anim_nodes::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let nodes = lw(c, at(c, S0, NODES));
        c.g[V0][0] = 1;
        if nodes != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, at(c, S0, COUNT));
            c.g[S1][0] = 0;
            if c.g[V0][0] as i64 > 0 {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.g[A1][0] = lw(c, at(c, S0, NODES));
                loop {
                    // s1 = index + 1, s2 = byte offset of the node, a1 = nodes.
                    c.cycles += 13;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    c.f[12] = Addr(at(c, S0, TIME)).read_fpu(c).0;
                    c.g[A1][0] = addu(c.g[S2][0], c.g[A1][0]);
                    let vt = lw(c, at(c, A1, NODE_COMPONENT_VPTR));
                    c.g[V0][0] = vt;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(NODE_SIZE));
                    let node = c.g[A1][0];
                    virtual_call(c, node, vt, SLOT_SET_TIME, V1, 0x0034_e3a4)?;
                    let count = lw(c, at(c, S0, COUNT));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < count as i64) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = lw(c, at(c, S0, NODES));
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e380);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x20);
        Addr(at(c, S0, UPDATED)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimObject` playback fields (`v15`).
mod anim_playback {
    /// `short mode`: 1 loop, 2 ping-pong, other clamp.
    pub const MODE: u32 = 0x00;
    pub const PLAYING: u32 = 0x04;
    /// Non-zero: the time is driven from outside (no stepping).
    pub const EXTERNAL: u32 = 0x08;
    /// Frames to wait before stepping (negative: never step).
    pub const DELAY: u32 = 0x0c;
    pub const TIME: u32 = 0x1c;
    pub const PUBLISHED_TIME: u32 = 0x20;
    pub const NEXT_EVENT: u32 = 0x24;
    pub const NEXT_EVENT_2: u32 = 0x28;
    /// The animation component (vtable at +0x3c).
    pub const COMPONENT: u32 = 0x30;
    pub const COMPONENT_VPTR: u32 = 0x3c;
    pub const FLAGS: u32 = 0x42;
    pub const SLOT_SET_DIRTY: u32 = 0x110;
}

/// The "no event" time (1e10) at gp-0x2b80.
const NEVER_GP: u32 = 0x2b80;

/// `bool AnimObject_v15(this)` (0x341d48): one playback step. Clears the
/// next event times (1e10); unless driven externally: counts the start
/// delay down or steps the time by the mode (`AnimObject_StepLoop` /
/// `StepPingPong` / `StepClamp`) and asks the script (0x34eba0) whether to
/// go on (0 stops, returning 0); then publishes the time, sets flag bit 0
/// and, for an externally driven object, calls the component's slot 0x110
/// with 1 (returning 0). Returns 1 otherwise.
pub fn anim_object_v15(c: &mut Cpu) {
    use anim_playback::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let never = Addr(r32(c, GP).wrapping_sub(NEVER_GP)).read_fpu(c);
        c.f[0] = never.0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        Addr(this + NEXT_EVENT_2).write_fpu(c, never);
        c.g[V0][0] = lw(c, this + EXTERNAL);
        Addr(this + NEXT_EVENT).write_fpu(c, never);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else if step(c, this)? {
            publish_time(c, this)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `v15` stepping: true when the time is to be published (v0 = flags),
/// false when the script stopped the animation.
fn step(c: &mut Cpu, this: u32) -> Result<bool, crate::call::Unwound> {
    use anim_playback::*;
    let flags = |c: &mut Cpu| c.g[V0][0] = Addr(this + FLAGS).lhu(c);
    c.cycles += 3;
    c.g[V0][0] = lw(c, this + PLAYING);
    if c.g[V0][0] == 0 {
        flags(c);
        return Ok(true);
    }
    c.cycles += 3;
    c.g[V0][0] = lw(c, this + DELAY);
    if (c.g[V0][0] as i64) < 0 {
        flags(c);
        return Ok(true);
    }
    c.cycles += 2;
    let delay = c.g[V0][0];
    c.g[V0][0] = sx((delay as u32).wrapping_sub(1));
    if delay as i64 > 0 {
        c.cycles += 2;
        Addr(this + DELAY).sw(c, c.g[V0][0]);
        c.cycles += 1;
        flags(c);
        return Ok(true);
    }
    c.cycles += 5;
    let time = Addr(this + TIME).read_fpu(c);
    c.f[0] = time.0;
    c.g[V0][0] = 1;
    c.g[V1][0] = Addr(this + MODE).lh(c);
    Addr(this + NEXT_EVENT).write_fpu(c, time);
    c.g[A0][0] = sx(this);
    if c.g[V1][0] == 1 {
        c.cycles += 4;
        callee::ANIM_OBJECT_STEP_LOOP.call(c, 0x0034_1dcc)?;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 2;
        if c.g[V1][0] != 2 {
            c.cycles += 2;
            callee::ANIM_OBJECT_STEP_CLAMP.call(c, 0x0034_1ddc)?;
        } else {
            c.cycles += 4;
            callee::ANIM_OBJECT_STEP_PING_PONG.call(c, 0x0034_1dbc)?;
        }
    }
    c.cycles += 4;
    c.g[A0][0] = sx(this + TIME);
    callee::ANIM_OBJECT_SCRIPT_CONTINUES.call(c, 0x0034_1de4)?;
    if c.g[V0][0] != 0 {
        flags(c);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// `v15` publishing (v0 = flags on entry; the result in v0).
fn publish_time(c: &mut Cpu, this: u32) -> Flow {
    use anim_playback::*;
    c.cycles += 7;
    c.g[A2][0] = sx(this + COMPONENT);
    let time = Addr(this + TIME).read_fpu(c);
    c.f[0] = time.0;
    c.g[V0][0] |= 1;
    c.g[V1][0] = lw(c, this + EXTERNAL);
    Addr(this + FLAGS).sh(c, c.g[V0][0]);
    Addr(this + PUBLISHED_TIME).write_fpu(c, time);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 8;
        let vt = lw(c, this + COMPONENT_VPTR);
        c.g[V0][0] = vt;
        c.g[A1][0] = 1;
        let component = c.g[A2][0];
        virtual_call(c, component, vt, SLOT_SET_DIRTY, V1, 0x0034_1e2c)?;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// `MultiSplineModifier` node data: `count` at +4, node array at +0x3c
/// (0x60 bytes each, bounds min +0x30 / max +0x40), per-node spline objects
/// at +0x44 (position at +0x40, bounds copies at +0x60 / +0x6c), the half
/// size at +0x18, the event target at +0x38.
mod spline_nodes {
    pub const COUNT: u32 = 0x04;
    pub const HALF_SIZE: u32 = 0x18;
    pub const SCRIPT: u32 = 0x1c;
    pub const TARGET: u32 = 0x38;
    pub const NODES: u32 = 0x3c;
    pub const SPLINES: u32 = 0x40;
    pub const OBJECTS: u32 = 0x44;
    pub const NODE_SIZE: u32 = 0x60;
}

/// The graphics manager pointer at gp+0x2a90 (vtable at +0x10d8).
const GRAPHICS_GP_PLUS: u32 = 0x2a90;
const GRAPHICS_VPTR: u32 = 0x10d8;

/// `MultiSplineModifier_v24(this)` (0x35a918): rebuilds if dirty, then for
/// nodes 1..count re-centres the node's bounds on its object's position
/// (`pos ± half`), moves the object in the Nash spatial hash
/// (`SpatialHash_Update(World_GetNash(), 0, obj, &newBounds, &oldBounds)`)
/// and copies the bounds into the object (+0x60 / +0x6c).
pub fn multi_spline_modifier_v24(c: &mut Cpu) {
    use spline_nodes::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save_ra(c, 0x60);
        rebuild_if_dirty(c, S3, 0x0035_a958)?;
        c.g[V0][0] = lw(c, at(c, S3, COUNT));
        c.cycles += 4;
        c.g[S4][0] = 1;
        c.g[V0][0] = (1 < c.g[V0][0] as i64) as u64;
        frame.restore(c, S0, 0xb0);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[S1][0] = NODE_SIZE as u64;
            loop {
                c.cycles += 80;
                recentre_node(c, frame)?;
                c.g[V0][0] = lw(c, at(c, S3, COUNT));
                c.g[V0][0] = ((c.g[S4][0] as i64) < c.g[V0][0] as i64) as u64;
                c.g[S1][0] = NODE_SIZE as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0xb0);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a970);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `a ± b` lane by lane (VSUB / VADD with all lanes).
fn vsub(a: Vec4, b: Vec4) -> Vec4 {
    Vec4 { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w }
}
fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    Vec4 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w }
}

/// One iteration of `v24` (node index in s4, s1 = 0x60 on entry).
fn recentre_node(c: &mut Cpu, frame: Frame) -> Flow {
    use spline_nodes::*;
    let sp = frame.at(c, 0);
    let nodes = lw(c, at(c, S3, NODES));
    c.g[A1][0] = nodes;
    let lo = crate::abi::mult(c, r32(c, S4), r32(c, S1));
    c.g[S1][0] = sx(lo);
    let half = Addr(at(c, S3, HALF_SIZE)).read_fpu(c);
    c.f[0] = half.0;
    let objects = lw(c, at(c, S3, OBJECTS));
    c.g[S2][0] = sx(r32(c, S4) << 2);
    c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
    c.g[A0][0] = addu(c.g[S2][0], objects);
    c.g[A1][0] = addu(c.g[S1][0], c.g[A1][0]);
    let node = r32(c, A1);
    // The old bounds, kept at sp (min) and sp+0x10 (max).
    c.lq(V0, node + 0x30);
    c.sq(V0, sp);
    c.lq(V1, node + 0x40);
    for off in [0x28, 0x20, 0x24] {
        Addr(sp + off).write_fpu(c, half);
    }
    c.sq(V1, sp + 0x10);
    Addr(sp + 0x2c).sw(c, 0);
    let half4 = Addr(sp + 0x20).read_vec4(c);
    // New min = object position - half.
    let obj = lw(c, r32(c, A0));
    c.g[V1][0] = obj;
    c.lq(V0, (obj as u32).wrapping_add(0x40));
    c.sq(V0, sp + 0x40);
    let pos = Addr(sp + 0x40).read_vec4(c);
    let min = vsub(pos, half4);
    Addr(sp + 0x50).write_vec4(c, min);
    c.lq(V0, sp + 0x50);
    c.sq(V0, sp + 0x30);
    c.sq(V0, node + 0x30);
    // New max = object position + half.
    let objects = lw(c, at(c, S3, OBJECTS));
    let nodes = lw(c, at(c, S3, NODES));
    c.g[V1][0] = addu(c.g[S2][0], objects);
    let obj = lw(c, r32(c, V1));
    c.g[A0][0] = obj;
    c.g[A1][0] = addu(c.g[S1][0], nodes);
    let node = r32(c, A1);
    c.lq(V0, (obj as u32).wrapping_add(0x40));
    c.sq(V0, sp + 0x40);
    let pos = Addr(sp + 0x40).read_vec4(c);
    let max = vadd(pos, half4);
    vu0_scratch::set_vf(c, 3, 0b1111, pos);
    vu0_scratch::set_vf(c, 4, 0b1111, half4);
    vu0_scratch::set_vf(c, 5, 0b1111, max);
    Addr(sp + 0x50).write_vec4(c, max);
    c.lq(V0, sp + 0x50);
    c.sq(V0, sp + 0x30);
    c.sq(V0, node + 0x40);
    let objects = lw(c, at(c, S3, OBJECTS));
    c.g[V1][0] = addu(c.g[S2][0], objects);
    c.g[S0][0] = lw(c, r32(c, V1));
    call_ptr(c, table_callee::WORLD_GET_NASH, 0x0035_aa20)?;
    let nodes = lw(c, at(c, S3, NODES));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = sx((nodes as u32).wrapping_add(r32(c, S1)).wrapping_add(0x30));
    c.g[T0][0] = c.g[SP][0];
    call_ptr(c, table_callee::SPATIAL_HASH_UPDATE, 0x0035_aa40)?;
    // Copy the new bounds into the object: min -> +0x60, max -> +0x6c.
    let nodes = lw(c, at(c, S3, NODES));
    let objects = lw(c, at(c, S3, OBJECTS));
    let min_at = (nodes as u32).wrapping_add(r32(c, S1)).wrapping_add(0x30);
    c.g[V0][0] = sx(min_at);
    c.g[V1][0] = addu(c.g[S2][0], objects);
    let x = Addr(min_at).read_fpu(c);
    let obj = lw(c, r32(c, V1));
    c.g[A0][0] = obj;
    Addr((obj as u32) + 0x60).write_fpu(c, x);
    let y = Addr(min_at + 4).read_fpu(c);
    Addr((obj as u32) + 0x64).write_fpu(c, y);
    let z = Addr(min_at + 8).read_fpu(c);
    Addr((obj as u32) + 0x68).write_fpu(c, z);
    let nodes = lw(c, at(c, S3, NODES));
    c.g[V0][0] = nodes;
    let objects = lw(c, at(c, S3, OBJECTS));
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(nodes as u32).wrapping_add(0x40));
    c.g[S2][0] = addu(c.g[S2][0], objects);
    let max_at = r32(c, S1);
    let x2 = Addr(max_at).read_fpu(c);
    let obj = lw(c, r32(c, S2));
    c.g[V1][0] = obj;
    Addr((obj as u32) + 0x6c).write_fpu(c, x2);
    let y2 = Addr(max_at + 4).read_fpu(c);
    Addr((obj as u32) + 0x70).write_fpu(c, y2);
    let z2 = Addr(max_at + 8).read_fpu(c);
    Addr((obj as u32) + 0x74).write_fpu(c, z2);
    c.f[0] = z2.0;
    c.f[1] = y2.0;
    Ok(())
}

/// `MultiSplineModifier_v11(this)` (0x35b418): rebuilds if dirty, then
/// culls nodes 1..count against the view: the graphics manager fills a
/// frustum on the stack (slot 0x158) and classifies the node's bounds
/// (slot 0x2e8: 0 outside, 1 inside, else partial). For a node not fully
/// inside, the event target (`splines->+0xc`) gets the node's object
/// (slot 0x108) and an event (slot 0x28: 0x400 outside, 0x420 partial).
/// Finally runs the attached script (0x345430) if there is one.
pub fn multi_spline_modifier_v11(c: &mut Cpu) {
    use spline_nodes::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S4, 0x80);
        frame.save(c, S0, 0xc0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x40);
        rebuild_if_dirty(c, S4, 0x0035_b464)?;
        c.g[V1][0] = lw(c, at(c, S4, SPLINES));
        c.cycles += 6;
        c.g[S6][0] = 1;
        let count = lw(c, at(c, S4, COUNT));
        c.g[S5][0] = lw(c, at(c, V1, 0xc));
        c.g[V0][0] = (1 < count as i64) as u64;
        c.g[S7][0] = lw(c, at(c, GP, GRAPHICS_GP_PLUS));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = NODE_SIZE as u64;
            loop {
                c.cycles += 23;
                cull_node(c)?;
                c.cycles += 4;
                c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
                let more = (c.g[S6][0] as i64) < c.g[V0][0] as i64;
                c.g[V0][0] = NODE_SIZE as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_b488);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, S4, SCRIPT));
        c.g[A1][0] = sx(at(c, S4, 0x20));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(at(c, S4, 0x48));
            callee::RUN_SPLINE_SCRIPT.call(c, 0x0035_b578)?;
        }
        c.cycles += 11;
        for (reg, off) in [(S0, 0xc0), (S1, 0xb0), (S2, 0xa0), (S3, 0x90), (S4, 0x80), (S5, 0x70), (S6, 0x60), (S7, 0x50)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// One iteration of `v11` (node index in s6, v0 = 0x60 on entry; leaves
/// v0 = count).
fn cull_node(c: &mut Cpu) -> Flow {
    use spline_nodes::*;
    let gfx_vt = lw(c, at(c, S7, GRAPHICS_VPTR));
    c.g[S1][0] = gfx_vt;
    let lo = crate::abi::mult(c, r32(c, S6), r32(c, V0));
    c.g[V0][0] = sx(lo);
    let nodes = lw(c, at(c, S4, NODES));
    let classify_delta = Addr(at(c, S1, 0x2e8)).lh(c);
    c.g[A0][0] = c.g[SP][0];
    let frustum = VSlot::read(c, gfx_vt, 0x158);
    c.g[V1][0] = frustum.func;
    c.g[S2][0] = addu(c.g[S7][0], classify_delta);
    let node = addu(nodes, c.g[V0][0]);
    c.g[A1][0] = frustum.this(c.g[S7][0]);
    c.g[S3][0] = sx((node as u32).wrapping_add(0x40));
    c.g[S0][0] = sx((node as u32).wrapping_add(0x30));
    call_ptr(c, frustum.func as u32, 0x0035_b4bc)?;
    let classify = lw(c, at(c, S1, 0x2ec));
    c.g[V0][0] = classify;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = c.g[SP][0];
    call_ptr(c, classify as u32, 0x0035_b4d4)?;
    c.g[S0][0] = c.g[V0][0];
    let inside = c.g[S0][0] == 1;
    c.g[V0][0] = sx(r32(c, S6) << 2);
    if !inside {
        c.cycles += 12;
        let objects = lw(c, at(c, S4, OBJECTS));
        c.g[V1][0] = objects;
        let target_vt = lw(c, at(c, S5, 0xc));
        c.g[A3][0] = target_vt;
        c.g[V0][0] = addu(c.g[V0][0], objects);
        c.g[A2][0] = lw(c, at(c, S4, TARGET));
        let obj = lw(c, r32(c, V0));
        c.g[A1][0] = sx((obj as u32).wrapping_add(0x10));
        let target = c.g[S5][0];
        virtual_call(c, target, target_vt, 0x108, V0, 0x0035_b50c)?;
        c.g[A2][0] = 0x420;
        let (event, ret_addr) = if c.g[S0][0] != 0 {
            c.cycles += 8;
            (None, 0x0035_b52c)
        } else {
            c.cycles += 7;
            (Some(0x400), 0x0035_b550)
        };
        let vt = lw(c, at(c, S5, 0xc));
        c.g[V0][0] = vt;
        if let Some(e) = event {
            c.g[A2][0] = e;
        }
        c.g[A1][0] = lw(c, at(c, S4, TARGET));
        let target = c.g[S5][0];
        virtual_call(c, target, vt, 0x28, V1, ret_addr)?;
        if event.is_some() {
            c.cycles += 1;
        }
    } else {
        c.cycles += 1;
    }
    c.g[V0][0] = lw(c, at(c, S4, COUNT));
    Ok(())
}

/// `AnimController_DispatchEndEvents(ctrl)` (0x312490): collects every
/// sequence of the six layers whose trigger bit 63 (end event) is set
/// (`(seq+0xb0) bit 0x3f`, 0x1446a0) as `{seq, list}` pairs on the stack,
/// then calls the controller's handler (vtable at +0x58, slot 0x18) with
/// `(ctrl + delta, list, seq)` for each.
pub fn anim_controller_dispatch_end_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x170);
        frame.save(c, S0, 0x160);
        frame.save(c, S2, 0x140);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x130);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x150);
        c.g[S3][0] = 0;
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save_ra(c, 0x100);
        c.g[A0][0] = 0;
        // s0 = layer (then sequence), s4 = layer * 8, s5 = layer + 1,
        // s3 = pairs found, s1 = next pair slot.
        loop {
            c.cycles += 9;
            let layers = lw(c, at(c, S2, anim_controller::LAYERS));
            c.g[V0][0] = layers;
            c.g[S4][0] = c.g[A0][0];
            c.g[A1][0] = 0;
            c.g[A0][0] = addu(layers, c.g[A0][0]);
            c.g[S5][0] = sx(r32(c, S0).wrapping_add(1));
            callee::ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_24d8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, S3) << 3);
            if c.g[S0][0] != 0 {
                c.cycles += 3;
                c.g[S1][0] = addu(c.g[V0][0], c.g[SP][0]);
                c.g[A0][0] = sx(at(c, S0, 0xb0));
                loop {
                    c.cycles += 4;
                    c.g[A1][0] = 0x3f;
                    call_ptr(c, table_callee::TEST_BIT64, 0x0031_24f8)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 7;
                        let layers = lw(c, at(c, S2, anim_controller::LAYERS));
                        c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                        Addr(r32(c, S1)).sw(c, c.g[S0][0]);
                        c.g[V0][0] = addu(layers, c.g[S4][0]);
                        Addr(at(c, S1, 4)).sw(c, c.g[V0][0]);
                        c.g[S1][0] = sx(at(c, S1, 8));
                    }
                    c.g[S0][0] = lw(c, at(c, S0, 0xc8));
                    c.cycles += 2;
                    c.g[A0][0] = sx(at(c, S0, 0xb0));
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_24f0);
                    }
                }
            }
            c.cycles += 4;
            c.g[S0][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6) as u64;
            c.g[A0][0] = sx(r32(c, S0) << 3);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_24c0);
            }
        }
        c.cycles += 2;
        c.g[S1][0] = c.g[S3][0];
        if c.g[S3][0] as i64 > 0 {
            c.cycles += 3;
            c.g[S0][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, at(c, S2, 0x58));
            loop {
                c.cycles += 10;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A1][0] = lw(c, at(c, S0, 4));
                let handler = VSlot::read(c, c.g[V0][0], 0x18);
                c.g[A2][0] = lw(c, r32(c, S0));
                c.g[V1][0] = handler.func;
                c.g[A0][0] = handler.this(c.g[S2][0]);
                c.g[S0][0] = sx(at(c, S0, 8));
                call_ptr(c, handler.func as u32, 0x0031_2568)?;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[V0][0] = lw(c, at(c, S2, 0x58));
                if event_due(c) {
                    deliver_event_here(c, 0x0031_2548);
                }
            }
        }
        c.cycles += 9;
        for (reg, off) in [(S0, 0x160), (S1, 0x150), (S2, 0x140), (S3, 0x130), (S4, 0x120), (S5, 0x110)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{covered, fake_call, machine, FAKE, LOG_COUNT, LOG_START};
    use crate::difftest::{check_lohi, fill, fresh, require_paths, Rng, DATA};
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use ssx3_ee_rt::funcs::GuestFn;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    // ------------------------------------------------------------- fakes

    /// `fakes::fake_call`, then a varied result: null, negative or the
    /// fake's pointer (fakes alone never return 0).
    fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match (h >> 20) & 3 {
            0 => c.g[V0][0] = 0,
            1 => c.g[V0][0] = sx(h as u32 | 0x8000_0000),
            _ => {}
        }
    }

    /// One vtable whose every slot 0x00..0x340 is a distinct fake with a
    /// small random `this` delta.
    const VT: u32 = DATA + 0xf000;
    fn put_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        for off in (0..0x340).step_by(8) {
            let delta = ((r.next_u32() as i16 as i32 % 64) & !3) as u16;
            c.bus.write16(VT + off, delta);
            c.bus.write32(VT + off + 4, FAKE + 2 * off);
        }
        VT
    }

    /// The return addresses the fakes were called with, over all seeds
    /// (read from the fakes' RAM log): proves each call site was reached.
    fn require_call_sites(gen: GuestFn, seeds: u32, sites: &[u32], setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            gen(&mut c);
            let n = c.bus.read32(LOG_COUNT);
            for i in 0..n {
                let p = LOG_START + 0x220 * i;
                seen.insert(c.bus.read32(p + 0x10 + 16 * 31));
            }
        }
        for s in sites {
            assert!(seen.contains(s), "call site returning to {s:#x} never reached (seen {seen:x?})");
        }
    }

    /// Calls compared with unwinding fakes (`covered`), the fake call sites
    /// reached, and (for loops) events delivered mid-loop.
    fn calls_and_events(
        gen: GuestFn,
        hand: GuestFn,
        entry: Option<u32>,
        salt: u64,
        calls: u32,
        sites: &[u32],
        setup: &dyn Fn(&mut Cpu, &mut Rng, u32),
    ) {
        let with_calls = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, calls);
            c.hooks.miss = fake_varied;
            setup(c, &mut r, seed);
        };
        covered(gen, hand, SEEDS, &with_calls);
        let no_unwind = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 0);
            c.hooks.miss = fake_varied;
            setup(c, &mut r, seed);
        };
        require_call_sites(gen, SEEDS, sites, &no_unwind);
        if let Some(entry) = entry {
            check_events(gen, hand, SEEDS, &|c, seed| -> Events {
                let mut r = machine(c, seed, salt ^ 0xe7e7, 0);
                c.hooks.miss = fake_varied;
                setup(c, &mut r, seed);
                schedule(c, &mut r, seed, entry)
            });
        }
    }

    /// Calls only through real (direct) callees: plain `check` with the fake
    /// hooks installed, plus the listed total-cycle paths.
    fn real_calls(gen: GuestFn, hand: GuestFn, salt: u64, paths: &[u64], setup: &dyn Fn(&mut Cpu, &mut Rng, u32)) {
        let s = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 0);
            c.hooks.miss = fake_varied;
            setup(c, &mut r, seed);
        };
        check_lohi(gen, hand, SEEDS, &s);
        require_paths(gen, SEEDS, paths, &s);
    }

    // ------------------------------------------------------ section streamer

    #[test]
    fn streamer_get_section_count() {
        real_calls(ssx3_ee::s13::f_003a9800, super::streamer_get_section_count, 0x5343, &[5], &|c, r, seed| {
            let db = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(DATA + 4, db);
            c.bus.write32(DATA + 0x108, r.next_u32());
            c.g[A0][0] = DATA as u64;
        });
    }

    /// A `cWorldCache` streamer at `DATA` with a graphics manager and one
    /// section database of up to three sections (`+0x14 + 8*i` classed).
    fn streamer(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u32, u32) {
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        let gfx = DATA + 0x3000;
        c.bus.write32(crate::difftest::GAME_GP + 0x2a74, gfx);
        c.bus.write32(gfx + 0x18, r.next_u32());
        let view = (seed % 4) as u32;
        let rec = DATA + 0x50 * view;
        let db = DATA + 0x800;
        let sections = DATA + 0x4000;
        let n = [0u32, 1, 2, 3][((seed / 4) % 4) as usize];
        c.bus.write32(DATA + 4, db);
        c.bus.write32(db + 8, n);
        c.bus.write32(DATA + 8, sections);
        c.bus.write32(rec + 0x240, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(rec + 0x280, r.f32(100.0));
        // A huge view bound so the node's early-out compare always returns 0.
        c.bus.write32(rec + 0x260, r.f32(1000.0));
        for i in 0..n {
            let id = [3u32, 5, 6, 7][((seed / 16 + i) % 4) as usize];
            c.bus.write32(DATA + 0x14 + 8 * i, id);
            let node = DATA + 0x6000 + 0x100 * i;
            c.bus.write32(sections + 0x58 * i + 0x1c, node);
            c.bus.write32(node + 0x10, 0);
        }
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = view as u64;
        (rec, view)
    }

    #[test]
    fn streamer_get_view_clearance() {
        real_calls(ssx3_ee::s13::f_003a96e0, super::streamer_get_view_clearance, 0x5344, &[], &|c, r, seed| {
            let (rec, view) = streamer(c, r, seed);
            for i in 0..4 {
                c.bus.write32(rec + 0x250 + 4 * i, r.edge_f32());
            }
            c.bus.write32(rec + 0x284, r.edge_f32());
            c.g[A1][0] = view as u64;
            c.g[A2][0] = (DATA + 0x1000) as u64;
            c.g[A3][0] = (DATA + 0x2000) as u64;
        });
    }

    #[test]
    fn streamer_update_view() {
        real_calls(ssx3_ee::s13::f_003a9d60, super::streamer_update_view, 0x5345, &[], &|c, r, seed| {
            streamer(c, r, seed);
        });
    }

    // ------------------------------------------------------- AnimController

    const CTRL: u32 = DATA;
    const LISTS: u32 = DATA + 0x400;
    const SEQS: u32 = DATA + 0x1000;

    /// An AnimController with six layer lists of 0..3 sequences (0xd0
    /// bytes, linked at +0xc8), random animation ids (some 0x1b6).
    fn anim_controller(c: &mut Cpu, r: &mut Rng) {
        fill(c, r, CTRL, 0x20);
        c.bus.write32(CTRL + 0x50, LISTS);
        for layer in 0..6 {
            let id = if r.next_u32() % 3 == 0 { 0x1b6 } else { r.next_u32() % 0x300 };
            c.bus.write32(CTRL + 4 * layer, id);
            let list = LISTS + 8 * layer;
            let n = r.next_u32() % 4;
            c.bus.write32(list, 0);
            let mut head = 0;
            for i in 0..n {
                let s = SEQS + 0x100 * (layer * 4 + i);
                fill(c, r, s, 0x34);
                c.bus.write32(s + 0xc8, head);
                head = s;
            }
            c.bus.write32(list + 4, head);
        }
    }

    #[test]
    fn anim_controller_get_layer_sequence() {
        real_calls(ssx3_ee::s10::f_00311b20, super::anim_controller_get_layer_sequence, 0x4153, &[], &|c, r, _| {
            anim_controller(c, r);
            c.g[A0][0] = CTRL as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    #[test]
    fn anim_controller_is_layer_anim_finished() {
        real_calls(ssx3_ee::s10::f_00312ae8, super::anim_controller_is_layer_anim_finished, 0x4146, &[], &|c, r, _| {
            anim_controller(c, r);
            // Every layer non-empty (the result is dereferenced).
            for layer in 0..6 {
                if c.bus.read32(LISTS + 8 * layer + 4) == 0 {
                    c.bus.write32(LISTS + 8 * layer + 4, SEQS + 0x100 * (24 + layer));
                }
            }
            c.g[A0][0] = CTRL as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    #[test]
    fn anim_controller_get_layer_anim_entry() {
        real_calls(ssx3_ee::s10::f_00311ae8, super::anim_controller_get_layer_anim_entry, 0x4145, &[11, 20], &|c, r, _| {
            anim_controller(c, r);
            fill(c, r, 0x0044_6990, 0x300 * 7);
            c.g[A0][0] = CTRL as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    #[test]
    fn anim_controller_fade_out_layer() {
        // The real list walkers run (sequence fade-out / trigger removal on
        // random sequences); empty and non-empty layers.
        real_calls(ssx3_ee::s10::f_00311e88, super::anim_controller_fade_out_layer, 0x464f, &[], &|c, r, _| {
            anim_controller(c, r);
            c.f[12] = r.edge_f32();
            c.g[A0][0] = CTRL as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    #[test]
    fn anim_controller_rotate_roots() {
        real_calls(ssx3_ee::s10::f_00311b48, super::anim_controller_rotate_roots, 0x5252, &[], &|c, r, _| {
            anim_controller(c, r);
            // Empty layers: TransformRoots walks nothing.
            for layer in 0..6 {
                c.bus.write32(LISTS + 8 * layer, 0);
            }
            for i in 0..12 {
                c.bus.write32(super::ROOT_TRANSLATION + 4 * i, r.f32(2.0));
            }
            c.f[12] = r.f32(7.0);
            c.g[A0][0] = CTRL as u64;
        });
    }

    // ------------------------------------------- MultiSpline / AnimObject

    const OBJ: u32 = DATA + 0x200;

    #[test]
    fn multi_spline_modifier_v03() {
        // UpdateNodes on a modifier with no spline nodes.
        real_calls(ssx3_ee::s11::f_0035a5d8, super::multi_spline_modifier_v03, 0x7633, &[], &|c, r, _| {
            fill(c, r, OBJ, 0x20);
            for off in [0x4, 0x30] {
                c.bus.write32(OBJ + off, 0);
            }
            c.bus.write32(OBJ + 0x1c, 0);
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn multi_spline_modifier_v04() {
        calls_and_events(ssx3_ee::s11::f_0035abf0, super::multi_spline_modifier_v04, None, 0x7634, 1, &[0x0035_ac14], &|c, r, _| {
            put_vtable(c, r);
            let owner = DATA + 0x300;
            let object = DATA + 0x380;
            c.bus.write32(OBJ + 0x40, owner);
            c.bus.write32(owner + 0xc, object);
            c.bus.write32(object + 0xc, VT);
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn multi_spline_modifier_v18() {
        calls_and_events(ssx3_ee::s12::f_00361c20, super::multi_spline_modifier_v18, None, 0x7618, 1, &[0x0036_1c50], &|c, r, seed| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x20);
            c.bus.write32(OBJ, VT);
            c.bus.write32(OBJ + 0x30, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(OBJ + 0x44, DATA + 0x400);
            c.bus.write32(DATA + 0x400, r.next_u32());
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn anim_object_v51() {
        calls_and_events(ssx3_ee::s11::f_0034dc90, super::anim_object_v51, None, 0x4f51, 3, &[0x0034_dcbc, 0x0034_dcdc, 0x0034_dcf0], &|c, r, _| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x20);
            c.bus.write32(OBJ + 0x20, VT);
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn anim_object_save() {
        // BaseSave and ModObj_Save run for real; ModObj_Save's writes go
        // through the stream's (fake) vtable.
        calls_and_events(ssx3_ee::s11::f_00342150, super::anim_object_save, None, 0x4f53, 6, &[0x0034_218c, 0x0034_e414, 0x0034_e430], &|c, r, _| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x40);
            // No modifier blocks to save.
            c.bus.write32(OBJ + 0x30, 0);
            c.bus.write32(OBJ + 0x4c, 0);
            let stream = DATA + 0x500;
            c.bus.write32(stream, VT);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = stream as u64;
        });
    }

    #[test]
    fn anim_object_base_save() {
        calls_and_events(ssx3_ee::s11::f_0034e3d8, super::anim_object_base_save, None, 0x4f42, 5, &[0x0034_e414, 0x0034_e430], &|c, r, _| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x40);
            // No modifier blocks to save.
            c.bus.write32(OBJ + 0x30, 0);
            c.bus.write32(OBJ + 0x4c, 0);
            let stream = DATA + 0x500;
            c.bus.write32(stream, VT);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = stream as u64;
        });
    }

    // ------------------------------------------------- batch 2: loops

    #[test]
    fn anim_object_v52() {
        calls_and_events(ssx3_ee::s11::f_0034e348, super::anim_object_v52, Some(0x0034_e348), 0x4f52, 6, &[0x0034_e3a4], &|c, r, seed| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x20);
            let nodes = DATA + 0x1000;
            c.bus.write32(OBJ + 0x48, if seed % 7 == 0 { 0 } else { nodes });
            let n = [0u32, 1, 2, 5, 0xffff_fffe][(seed / 7 % 5) as usize];
            c.bus.write32(OBJ + 0x40, n);
            for i in 0..6 {
                c.bus.write32(nodes + 0xd0 * i + 0x84, VT);
            }
            c.g[A0][0] = OBJ as u64;
        });
    }

    #[test]
    fn anim_object_v15() {
        // The real step functions and script hook (0x34eba0) run.
        real_calls(ssx3_ee::s11::f_00341d48, super::anim_object_v15, 0x4f15, &[], &|c, r, seed| {
            put_vtable(c, r);
            fill(c, r, OBJ, 0x20);
            c.bus.write16(OBJ, [1u16, 2, 3, 0][(seed % 4) as usize]);
            c.bus.write32(OBJ + 4, (seed / 4 % 3 != 0) as u32);
            c.bus.write32(OBJ + 8, (seed / 12 % 4 == 0) as u32);
            c.bus.write32(OBJ + 0xc, [0u32, 0, 0, 3, 0xffff_ffff][(seed / 48 % 5) as usize]);
            for off in [0x10u32, 0x14, 0x18, 0x1c] {
                c.bus.write32(OBJ + off, r.f32(10.0));
            }
            c.bus.write32(OBJ + 0x3c, VT);
            // Script hook: (OBJ+0x48)->+0xc == OBJ+0x30 means "go on".
            let p = DATA + 0x600;
            c.bus.write32(OBJ + 0x48, p);
            c.bus.write32(p + 0xc, if r.next_u32() % 2 == 0 { OBJ + 0x30 } else { 0 });
            c.g[A0][0] = OBJ as u64;
        });
    }

    /// Fake for `v11`: the frustum test (vtable slot 0x2e8) answers 0, 1
    /// or 2 (outside / inside / partial).
    fn fake_classify(c: &mut Cpu, target: u32) {
        fake_varied(c, target);
        if target == FAKE + 2 * 0x2e8 {
            c.g[V0][0] = c.g[V1][0] % 3;
        }
    }

    fn spline_modifier(c: &mut Cpu, r: &mut Rng, seed: u32) {
        put_vtable(c, r);
        fill(c, r, OBJ, 0x30);
        c.bus.write32(OBJ, VT);
        c.bus.write32(OBJ + 0x30, (seed % 3 == 0) as u32);
        c.bus.write32(OBJ + 4, [0u32, 1, 2, 4, 6][(seed / 3 % 5) as usize]);
        c.bus.write32(OBJ + 0x18, r.f32(50.0));
        let (splines, target, nodes, objs) = (DATA + 0x400, DATA + 0x480, DATA + 0x1000, DATA + 0x2000);
        c.bus.write32(OBJ + 0x40, splines);
        c.bus.write32(splines + 0xc, target);
        c.bus.write32(target + 0xc, VT);
        c.bus.write32(OBJ + 0x3c, nodes);
        c.bus.write32(OBJ + 0x44, DATA + 0x500);
        for i in 0..6 {
            c.bus.write32(DATA + 0x500 + 4 * i, objs + 0x100 * i);
            for k in 0..0x20 {
                c.bus.write32(objs + 0x100 * i + 4 * k, r.f32(1000.0));
            }
            for k in 0..0x18 {
                c.bus.write32(nodes + 0x60 * i + 4 * k, r.f32(1000.0));
            }
        }
    }

    #[test]
    fn multi_spline_modifier_v11() {
        calls_and_events(
            ssx3_ee::s11::f_0035b418,
            super::multi_spline_modifier_v11,
            Some(0x0035_b418),
            0x7611,
            12,
            &[0x0035_b464, 0x0035_b4bc, 0x0035_b4d4, 0x0035_b50c, 0x0035_b52c, 0x0035_b550],
            &|c, r, seed| {
                spline_modifier(c, r, seed);
                c.hooks.miss = fake_classify;
                let gfx = DATA + 0x10000;
                c.bus.write32(super::GRAPHICS_GP_PLUS.wrapping_add(crate::difftest::GAME_GP), gfx);
                c.bus.write32(gfx + 0x10d8, VT);
                // The script (0x345430) with no entries.
                c.bus.write32(OBJ + 0x1c, (seed % 4 == 1) as u32);
                let (p, q, rr) = (DATA + 0x700, DATA + 0x780, DATA + 0x800);
                c.bus.write32(OBJ + 0x50, p);
                c.bus.write32(p + 0x68, q);
                c.bus.write32(q + 0x20, 0);
                c.bus.write32(gfx + 0xe84, rr);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v24() {
        // World_GetNash and SpatialHash_Update run for real (on an empty hash).
        calls_and_events(
            ssx3_ee::s11::f_0035a918,
            super::multi_spline_modifier_v24,
            Some(0x0035_a918),
            0x7624,
            1,
            &[0x0035_a958],
            &|c, r, seed| {
                spline_modifier(c, r, seed);
                // Node bounds already at pos -/+ half (1 ulp off for some
                // seeds), so the real SpatialHash_Update sees the same key.
                use crate::ps2f::VuF32;
                let half = VuF32(c.bus.read32(OBJ + 0x18));
                for i in 1..6u32 {
                    let (obj, node) = (DATA + 0x2000 + 0x100 * i, DATA + 0x1000 + 0x60 * i);
                    for l in 0..4u32 {
                        let p = VuF32(c.bus.read32(obj + 0x40 + 4 * l));
                        let h = if l == 3 { VuF32::ZERO } else { half };
                        let nudge = (r.next_u32() % 4 == 0) as u32;
                        c.bus.write32(node + 0x30 + 4 * l, (p - h).0 ^ nudge);
                        c.bus.write32(node + 0x40 + 4 * l, (p + h).0);
                    }
                }
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn anim_controller_dispatch_end_events() {
        calls_and_events(
            ssx3_ee::s10::f_00312490,
            super::anim_controller_dispatch_end_events,
            Some(0x0031_2490),
            0x4445,
            8,
            &[0x0031_2568],
            &|c, r, _| {
                put_vtable(c, r);
                anim_controller(c, r);
                c.bus.write32(CTRL + 0x58, VT);
                c.g[A0][0] = CTRL as u64;
            },
        );
    }
}
