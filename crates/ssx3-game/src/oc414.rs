//! Staging module oc414 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc414`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x00171690, camera_trigger_read_bound_obj, "CameraTrigger_ReadBoundObj"),
    (0x00151600, be_init_char_profile, "BE_InitCharProfile"),
    (0x0011d390, rider_reset, "Rider_Reset"),
    (0x00150558, bebagt_int_init_default_tricks, "BEBAGTInt_InitDefaultTricks"),
    (0x00302778, w_script_fn_is_event_type, "WScriptFn_IsEventType"),
    (0x0034b038, c_flag_manager_group_add_flag, "cFlagManager_GroupAddFlag"),
    (0x00172e18, camera_trigger_read_sphere_trigger, "CameraTrigger_ReadSphereTrigger"),
    (0x0011c138, rider_load_bolt_models, "Rider_LoadBoltModels"),
    (0x0012f398, rider_reset_sub_handle_input, "RiderResetSub_HandleInput"),
    (0x0034e698, anim_object_v42, "AnimObject_v42"),
    (0x00229910, crowd_man_2d_init_slot, "CrowdMan2d_InitSlot"),
    (0x00112180, rider_init_track_paths, "Rider_InitTrackPaths"),
    (0x00324138, input_parser_parse_function_2, "InputParser_ParseFunction2"),
    (0x00272308, in_seq_heap_alloc, "InSeq_HeapAlloc"),
    (0x001513b8, be_apply_default_char_profile_bolts, "BE_ApplyDefaultCharProfileBolts"),
    (0x002792e0, script_engine_is_hud_allowed, "ScriptEngine_IsHUDAllowed"),
    (0x0012be20, rider_gate_sub_enter, "RiderGateSub_Enter"),
    (0x00166c60, chase_camera_algorithm_reset_for_rider, "ChaseCameraAlgorithm_ResetForRider"),
    (0x00231cb0, app_module_update, "AppModule_Update"),
    (0x0011b678, rider_load_list_compare, "Rider_LoadListCompare"),
];

/// `int CameraTrigger_ReadBoundObj(Reader* reader, BoundObj** out)` (0x171690): read a
/// bound-object type word (0 ellipse, 1 box, 2 line, 3 point), allocate the
/// matching object (0x2c bytes; the line 0x44), let its own reader fill it,
/// store it in `*out` and return the bytes consumed. An unknown type leaves
/// `*out` alone and returns just the type word's size.
pub fn camera_trigger_read_bound_obj(c: &mut Cpu) {
    const CAMERA_BOX_BOUND_OBJ_READ: Callee = Callee { addr: 0x00171b10, f: ee::s02::f_00171b10 };
    const CAMERA_ELLIPSE_BOUND_OBJ_READ: Callee = Callee { addr: 0x00171a20, f: ee::s02::f_00171a20 };
    const CAMERA_LINE_BOUND_OBJ_READ: Callee = Callee { addr: 0x00171c00, f: ee::s02::f_00171c00 };
    const CAMERA_TRIGGER_READ_POINT_BOUND_OBJ: Callee = Callee { addr: 0x00171d10, f: ee::s02::f_00171d10 };
    guest(c, |c| {
'b_171a00: {
'b_1719f4: {
'b_171954: {
'b_171848: {
'b_1717a4: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S3][0] = c.g[A0][0];
c.g[S4][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
call_ptr(c, 0x002d2558, 0x001716bc)?;
c.g[S2][0] = c.g[V0][0];
c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = 0x1;
let t = c.g[S1][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
if t {
break 'b_1717a4;
} else {
'b_171700: {
'b_1716e8: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1716e8;
} else {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_171700;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_171a00;
} else {
break 'b_1716e8;
}
}
}
}
c.cycles += 2;
let t = c.g[S1][0] == c.g[V0][0];
c.g[V0][0] = 0x3;
if t {
break 'b_171848;
} else {
c.cycles += 2;
let t = c.g[S1][0] == c.g[V0][0];
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_171954;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_171a00;
} else {
break 'b_171700;
}
}
}
}
c.cycles += 41;
c.g[A1][0] = 0x460000;
c.g[A2][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ef0));
c.g[A3][0] = 0u64;
c.g[A0][0] = 0x2c;
call_ptr(c, 0x00317d70, 0x00171718)?;
c.g[V1][0] = 0x460000;
c.g[S0][0] = c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A1][0] = 0x500000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d58));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S3][0];
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[A1][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(9, (c.g[SP][0] as u32));
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
CAMERA_ELLIPSE_BOUND_OBJ_READ.call(c, 0x0017179c)?;
let t = true;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
if t {
break 'b_1719f4;
} else {
break 'b_1717a4;
}
}
}
c.cycles += 41;
c.g[A1][0] = 0x460000;
c.g[A3][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ed8));
c.g[A2][0] = 0u64;
c.g[A0][0] = 0x2c;
call_ptr(c, 0x00317d70, 0x001717bc)?;
c.g[V1][0] = 0x460000;
c.g[S0][0] = c.g[V0][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A1][0] = 0x500000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d98));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S3][0];
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[T2][0] as u32));
c.g[T0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(7, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[A1][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
CAMERA_BOX_BOUND_OBJ_READ.call(c, 0x00171840)?;
let t = true;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
if t {
break 'b_1719f4;
} else {
break 'b_171848;
}
}
c.cycles += 40;
c.g[A1][0] = 0x460000;
c.g[A0][0] = 0x44;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ec0));
c.g[A2][0] = 0u64;
c.g[A3][0] = 0u64;
call_ptr(c, 0x00317d70, 0x00171860)?;
c.g[S0][0] = c.g[V0][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V0][0] = 0x460000;
c.g[V1][0] = 0x500000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3c10));
c.g[A3][0] = c.g[V1][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.g[A0][0] = 0xffffffffffffffff;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3dd8));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.g[S0][0];
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(9, (c.g[T0][0] as u32));
c.g[T2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(9, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[V1][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(9, (c.g[SP][0] as u32));
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
'l_1718e8: loop {
c.cycles += 7;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = c.g[V1][0] != c.g[A0][0];
c.g[V0][0] = 0x2;
if t {
if event_due(c) { deliver_event_here(c, 0x001718e8); } continue 'l_1718e8;
} else {
c.cycles += 20;
c.g[A0][0] = c.g[S3][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
c.ldr(9, (c.g[V1][0] as u32));
c.g[T2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x33));
c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x2c));
c.sw((c.g[A2][0] as u32).wrapping_add(0x34), c.g[T2][0]);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(8, (c.g[V0][0] as u32));
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x3f));
c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x38));
c.sw((c.g[A2][0] as u32).wrapping_add(0x40), c.g[T1][0]);
c.g[A1][0] = c.g[S0][0];
CAMERA_LINE_BOUND_OBJ_READ.call(c, 0x0017194c)?;
let t = true;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
if t {
break 'b_1719f4;
} else {
break 'b_171954;
}
}
}
}
c.cycles += 40;
c.g[A1][0] = 0x460000;
c.g[A2][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ea8));
c.g[A3][0] = 0u64;
c.g[A0][0] = 0x2c;
call_ptr(c, 0x00317d70, 0x0017196c)?;
c.g[V1][0] = 0x460000;
c.g[S0][0] = c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A1][0] = 0x500000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3e18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S3][0];
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[A1][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(9, (c.g[SP][0] as u32));
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
CAMERA_TRIGGER_READ_POINT_BOUND_OBJ.call(c, 0x001719f0)?;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
break 'b_1719f4;
}
c.cycles += 3;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.g[V0][0] = c.g[S2][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
break 'b_171a00;
}
c.cycles += 7;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t;
Ok(())
    });
}

/// `void BE_InitCharProfile(CharProfile* self, int k, int reload)` (0x151600):
/// reset a character's economy profile: optionally reload the bolt map
/// (`BE_InitCharProfileBoltMap`), set the unlock bits, clear the slot and
/// entry tables, copy the 88 mission flags from the table at 0x43ee10, take
/// the colour from the table at 0x5308d9, then reset the rivalries and the
/// default tricks (kind at +0xbc0) and clear the counters.
pub fn be_init_char_profile(c: &mut Cpu) {
    const BEBAGT_INT_INIT_DEFAULT_TRICKS: Callee = Callee { addr: 0x00150558, f: ee::s01::f_00150558 };
    const BE_INIT_CHAR_PROFILE_BOLT_MAP: Callee = Callee { addr: 0x00151a88, f: ee::s01::f_00151a88 };
    const BE_RESET_CHAR_RIVALRIES: Callee = Callee { addr: 0x001519e0, f: ee::s01::f_001519e0 };
    guest(c, |c| {
'b_151640: {
c.cycles += 14;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S3][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[FP][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[A2][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
if t {
break 'b_151640;
} else {
c.cycles += 2;
BE_INIT_CHAR_PROFILE_BOLT_MAP.call(c, 0x00151640)?;
break 'b_151640;
}
}
c.cycles += 78;
c.g[V0][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x278));
c.g[V1][0] = 0xfffffffffffffffe;
c.g[A0][0] = 0xfffffffffffffffd;
c.g[A1][0] = 0xfffffffffffffffb;
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[A2][0] = 0xfffffffffffffff7;
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V1][0] = 0xffffffffffffffef;
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.g[A0][0] = 0xffffffffffffffdf;
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.g[A1][0] = 0x1000;
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[A2][0] = 0x2000;
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V1][0] = 0x40;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[A0][0] = 0x80;
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.g[A1][0] = 0x100;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A2][0] = 0x200;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = 0x400;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[A0][0] = 0x800;
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.g[A1][0] = 0x4000;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A2][0] = 0x8000;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = 0x10000;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[A0][0] = 0x20000;
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.g[A1][0] = 0x40000;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A2][0] = 0x80000;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.sh((c.g[S3][0] as u32).wrapping_add(0x284), c.g[V1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sw(c.g[S3][0] as u32, c.g[V1][0]);
c.sd((c.g[S3][0] as u32).wrapping_add(0x278), c.g[V0][0]);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xba0));
c.sh((c.g[S3][0] as u32).wrapping_add(0x280), c.g[V1][0]);
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbdf));
c.sh((c.g[S3][0] as u32).wrapping_add(0x282), 0u64);
c.g[A1][0] = 0u64;
c.sh((c.g[S3][0] as u32).wrapping_add(0x286), 0u64);
c.g[A2][0] = 0x18;
c.sw((c.g[S3][0] as u32).wrapping_add(0xacc), 0u64);
c.g[S4][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
call_ptr(c, 0x00416210, 0x00151720)?;
c.g[V1][0] = sx((c.g[FP][0] as u32) << 4);
c.g[A0][0] = c.g[SP][0];
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x8;
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
call_ptr(c, 0x00416210, 0x00151738)?;
c.g[V0][0] = 0xffffffffffffffff;
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbe6));
c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[V0][0]);
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbb8));
c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0xde4));
c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf08));
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf28));
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf30));
c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A3][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xad0));
'l_151778: loop {
c.cycles += 2;
c.g[V1][0] = 0x19;
'l_151780: loop {
c.cycles += 7;
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(4, (c.g[SP][0] as u32));
c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.sdr(4, (c.g[V0][0] as u32));
let t = (c.g[V1][0] as i64) >= 0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x00151780); } continue 'l_151780;
} else {
c.cycles += 4;
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[S4][0] as i64) < 17i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xad0));
if t {
if event_due(c) { deliver_event_here(c, 0x00151778); } continue 'l_151778;
} else {
c.cycles += 5;
c.g[S4][0] = 0u64;
c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
c.g[S6][0] = 0x8;
c.g[S5][0] = 0u64;
'l_1517c0: loop {
c.cycles += 4;
c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S6][0] as u32));
c.g[S2][0] = 0x16;
'l_1517d0: loop {
c.cycles += 9;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x8;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
call_ptr(c, 0x00416210, 0x001517e4)?;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
c.sb(c.g[S1][0] as u32, 0u64);
let t = (c.g[S2][0] as i64) >= 0;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
if t {
if event_due(c) { deliver_event_here(c, 0x001517d0); } continue 'l_1517d0;
} else {
c.cycles += 4;
c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
let t = (c.g[S4][0] as i64) <= 0;
c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc));
if t {
if event_due(c) { deliver_event_here(c, 0x001517c0); } continue 'l_1517c0;
} else {
c.cycles += 9;
c.g[V0][0] = 0x440000;
c.g[T1][0] = 0xfffffffffffffffe;
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11f0));
c.g[T0][0] = 0xfffffffffffffffb;
c.g[A3][0] = 0xfffffffffffffff7;
c.g[A2][0] = 0xffffffffffffffef;
c.g[A0][0] = c.g[S3][0];
c.g[S4][0] = 0x57;
'l_151828: loop {
c.cycles += 19;
c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x1a));
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x118));
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V1][0] = c.g[V1][0] & c.g[T1][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] | 0x2;
c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x118), c.g[V1][0]);
c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x1b));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x24));
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x118), c.g[V1][0]);
let t = (c.g[S4][0] as i64) >= 0;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00151828); } continue 'l_151828;
} else {
c.cycles += 39;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
c.g[A2][0] = 0x7;
c.g[A1][0] = 0x1;
call_ptr(c, 0x00416210, 0x00151884)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[A0][0] = c.g[S3][0];
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[FP][0] as u32));
c.g[V0][0] = 0x530000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d9));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.lwl(5, (c.g[V1][0] as u32).wrapping_add(0x3));
c.lwr(5, (c.g[V1][0] as u32));
c.g[A2][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[A3][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5));
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x6));
c.swl(5, (c.g[S3][0] as u32).wrapping_add(0xbe2));
c.swr(5, (c.g[S3][0] as u32).wrapping_add(0xbdf));
c.sb((c.g[S3][0] as u32).wrapping_add(0xbe3), c.g[A2][0]);
c.sb((c.g[S3][0] as u32).wrapping_add(0xbe4), c.g[A3][0]);
c.sb((c.g[S3][0] as u32).wrapping_add(0xbe5), c.g[V0][0]);
c.sw((c.g[S3][0] as u32).wrapping_add(0xac4), 0u64);
c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0xbc0));
c.sw((c.g[S3][0] as u32).wrapping_add(0xac8), 0u64);
BE_RESET_CHAR_RIVALRIES.call(c, 0x001518d4)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
c.g[A0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0xbc0));
BEBAGT_INT_INIT_DEFAULT_TRICKS.call(c, 0x001518e0)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x8;
call_ptr(c, 0x00416210, 0x001518f0)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x54;
call_ptr(c, 0x00416210, 0x00151900)?;
c.sb((c.g[S3][0] as u32).wrapping_add(0xf04), 0u64);
c.g[A3][0] = 0x1;
c.g[A2][0] = 0u64;
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xe38));
'l_151910: loop {
c.cycles += 10;
c.sw(c.g[A1][0] as u32, 0u64);
c.g[V0][0] = sx((c.g[A3][0] as u32) << ((c.g[A2][0] as u32) & 31));
c.g[V0][0] = !(0u64 | c.g[V0][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf00));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = ((c.g[A2][0] as i64) < 25i64) as u64;
c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
let t = c.g[A0][0] != 0u64;
c.sw((c.g[S3][0] as u32).wrapping_add(0xf00), c.g[V1][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00151910); } continue 'l_151910;
} else {
c.cycles += 4;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A2][0] = 0u64;
c.g[T1][0] = 0xffffffffffffffff;
c.g[T0][0] = 0x1;
'l_151948: loop {
c.cycles += 14;
c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1f));
if c.g[V0][0] != 0 { c.g[V1][0] = c.g[A2][0]; }
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
c.g[A0][0] = sx((c.g[T0][0] as u32) << ((c.g[A2][0] as u32) & 31));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[A0][0] = !(0u64 | c.g[A0][0]);
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lw(c.g[V1][0] as u32);
c.g[A1][0] = ((c.g[A2][0] as i64) < 61i64) as u64;
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
let t = c.g[A1][0] != 0u64;
c.sw(c.g[V1][0] as u32, c.g[V0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00151948); } continue 'l_151948;
} else {
c.cycles += 23;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x4;
call_ptr(c, 0x003e6448, 0x00151990)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x50;
call_ptr(c, 0x003e6448, 0x001519a0)?;
c.g[V0][0] = 0x11;
c.sw((c.g[S3][0] as u32).wrapping_add(0xf80), 0u64);
c.sw((c.g[S3][0] as u32).wrapping_add(0x27c), c.g[V0][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t; return Ok(());
}
}
}
}
}
}
}
}
}
}
}
}
}
}
    });
}

/// `void Rider_Reset(cRider* self)` (0x11d390): when the rider's load step
/// (+0x880) is 7 (loaded), reset it for a new race: metrix, anim controller,
/// track paths, then put it at the start point or a marker depending on the
/// game mode/path (0x535bc8) and the location, or set the motion states;
/// finally clear the timers and counters, reset the FX and stop transitions.
pub fn rider_reset(c: &mut Cpu) {
    const RIDER_BASE_RESET_FX: Callee = Callee { addr: 0x00111890, f: ee::s00::f_00111890 };
    const RIDER_INIT_TRACK_PATHS: Callee = Callee { addr: 0x00112180, f: ee::s00::f_00112180 };
    const RIDER_METRIX_INTERRUPT_TRICK: Callee = Callee { addr: 0x00119e38, f: ee::s00::f_00119e38 };
    const RIDER_METRIX_RESET: Callee = Callee { addr: 0x00117540, f: ee::s00::f_00117540 };
    const RIDER_PLACE_AT_MARKER: Callee = Callee { addr: 0x0011de60, f: ee::s00::f_0011de60 };
    const RIDER_PLACE_AT_START: Callee = Callee { addr: 0x0011dd98, f: ee::s00::f_0011dd98 };
    const RIDER_RESET_BOOST_METER: Callee = Callee { addr: 0x00120d58, f: ee::s00::f_00120d58 };
    const RIDER_RESET_MOTION: Callee = Callee { addr: 0x0011df18, f: ee::s00::f_0011df18 };
    const RIDER_SET_MAIN_STATE: Callee = Callee { addr: 0x0011fe78, f: ee::s00::f_0011fe78 };
    const RIDER_SET_SUB_STATE: Callee = Callee { addr: 0x0011fec8, f: ee::s00::f_0011fec8 };
    const RIDER_STOP_TRANSITION: Callee = Callee { addr: 0x00125038, f: ee::s00::f_00125038 };
    guest(c, |c| {
'b_11d628: {
c.cycles += 11;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V1][0] = 0x7;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd(c.g[SP][0] as u32, c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x880));
if c.g[V0][0] != c.g[V1][0] {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_11d628;
} else {
'b_11d57c: {
'b_11d564: {
'b_11d4fc: {
'b_11d4e4: {
'b_11d4d4: {
'b_11d4d0: {
c.cycles += 42;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.g[V0][0] = 0x3;
c.g[A0][0] = 0xd;
c.g[A1][0] = 0xffffffffffffffff;
c.sw((c.g[V1][0] as u32).wrapping_add(0xde0), c.g[V0][0]);
c.g[S3][0] = 0x530000;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.sw((c.g[V0][0] as u32).wrapping_add(0xde4), c.g[A0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xdc), 0u64);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
RIDER_METRIX_RESET.call(c, 0x0011d3f0)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x00311a50, 0x0011d3f8)?;
call_ptr(c, 0x0014dc80, 0x0011d400)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0x2;
call_ptr(c, 0x0014dd58, 0x0011d40c)?;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x0014ef70, 0x0011d418)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x324), c.g[V0][0]);
RIDER_INIT_TRACK_PATHS.call(c, 0x0011d428)?;
call_ptr(c, 0x0014dc80, 0x0011d430)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x0011d43c)?;
c.g[S2][0] = c.g[V0][0];
call_ptr(c, 0x0014dc80, 0x0011d444)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0x1;
call_ptr(c, 0x0014dd58, 0x0011d450)?;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x00145c38, 0x0011d45c)?;
let t = c.g[V0][0] == 0u64;
c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d4d0;
} else {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d4d4;
} else {
c.cycles += 6;
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00144bc0, 0x0011d478)?;
c.g[V1][0] = c.lw(c.g[V0][0] as u32);
c.g[V0][0] = 0xe;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d4d4;
} else {
c.cycles += 4;
c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = 0x4;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d4e4;
} else {
c.cycles += 14;
c.g[A0][0] = c.g[S0][0];
RIDER_PLACE_AT_START.call(c, 0x0011d4a0)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0x1;
RIDER_SET_MAIN_STATE.call(c, 0x0011d4ac)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0x4;
RIDER_SET_SUB_STATE.call(c, 0x0011d4b8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0u64;
RIDER_METRIX_INTERRUPT_TRICK.call(c, 0x0011d4c8)?;
let t = true;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d57c;
} else {
break 'b_11d4d0;
}
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
break 'b_11d4d4;
}
c.cycles += 4;
c.g[A0][0] = 0x4;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
let t = c.g[V1][0] == c.g[A0][0];
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d4fc;
} else {
break 'b_11d4e4;
}
}
c.cycles += 4;
c.g[V1][0] = 0x5;
c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
let t = c.g[A0][0] == c.g[V1][0];
c.g[V0][0] = 0x6;
if t {
break 'b_11d4fc;
} else {
c.cycles += 2;
let t = c.g[A0][0] != c.g[V0][0];
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_11d564;
} else {
break 'b_11d4fc;
}
}
}
'b_11d548: {
'b_11d544: {
'b_11d540: {
c.cycles += 6;
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00144bc0, 0x0011d504)?;
c.g[V1][0] = c.lw(c.g[V0][0] as u32);
c.g[V0][0] = ((c.g[V1][0] as i64) < 14i64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = c.g[S0][0];
break 'b_11d540;
} else {
'b_11d534: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
break 'b_11d534;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < 22i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_11d540;
} else {
c.cycles += 2;
let t = true;
c.g[A1][0] = 0u64;
if t {
break 'b_11d544;
} else {
break 'b_11d534;
}
}
}
}
c.cycles += 3;
c.g[A0][0] = c.g[S0][0];
let t = true;
c.g[A2][0] = 0x1;
if t {
break 'b_11d548;
} else {
break 'b_11d540;
}
}
}
c.cycles += 1;
c.g[A1][0] = 0x1;
break 'b_11d544;
}
c.cycles += 1;
c.g[A2][0] = 0x2;
break 'b_11d548;
}
c.cycles += 7;
RIDER_PLACE_AT_MARKER.call(c, 0x0011d550)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0u64;
RIDER_RESET_MOTION.call(c, 0x0011d55c)?;
let t = true;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
if t {
break 'b_11d57c;
} else {
break 'b_11d564;
}
}
c.cycles += 6;
c.g[A1][0] = 0x3;
RIDER_SET_MAIN_STATE.call(c, 0x0011d56c)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0x6;
RIDER_SET_SUB_STATE.call(c, 0x0011d578)?;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
break 'b_11d57c;
}
'b_11d590: {
c.cycles += 3;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
let t = c.g[V1][0] == 0u64;

if t {
break 'b_11d590;
} else {
c.cycles += 2;
c.g[A0][0] = c.g[S0][0];
RIDER_RESET_BOOST_METER.call(c, 0x0011d590)?;
break 'b_11d590;
}
}
c.cycles += 38;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 0);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x470));
c.sw((c.g[S0][0] as u32).wrapping_add(0x3fc), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x2f0), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x2f4), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x480), 0u64);
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c0));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.sw((c.g[S0][0] as u32).wrapping_add(0x478), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x474), c.g[A0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x47c), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x59c), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x2e8), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a0), 0u64);
c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x0011d5ec)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
RIDER_BASE_RESET_FX.call(c, 0x0011d5f4)?;
c.g[V1][0] = 0xffffffffffffffff;
c.g[V0][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a4), 0u64);
c.g[A0][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a8), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x5ac), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0xf0), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0xf4), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V1][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x100), c.g[V0][0]);
RIDER_STOP_TRANSITION.call(c, 0x0011d620)?;
c.sw((c.g[S0][0] as u32).wrapping_add(0x5b0), 0u64);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_11d628;
}
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld(c.g[SP][0] as u32);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t;
Ok(())
    });
}

/// `void BEBAGTInt_InitDefaultTricks(int mode, BagtEntry* e)` (0x150558):
/// clear the 15 six-byte trick entries (0x5a bytes), fill the first five and
/// the tenth from the per-mode (0..8, else default) table, then derive the
/// unlock masks from the per-trick counts at 0x45aeb8. A jump-table function:
/// other resume addresses go to the generated entry dispatcher.
pub fn bebagt_int_init_default_tricks(c: &mut Cpu) {
    if c.pc != 0x00150558 { return ee::s01::f_00150558__entries(c); }
    guest(c, |c| {
'b_1507e8: {
'b_1507cc: {
'b_1507a8: {
c.cycles += 14;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.g[A2][0] = 0x5a;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A1][0];
c.g[S1][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd(c.g[SP][0] as u32, c.g[RA][0]);
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = 0u64;
call_ptr(c, 0x003e6448, 0x00150584)?;
c.g[V0][0] = (c.g[S1][0] < 0x9u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x460000;
if t {
break 'b_1507a8;
} else {
'b_150794: {
'b_150764: {
'b_150730: {
'b_1506fc: {
'b_1506ec: {
'b_1506e0: {
'b_1506a8: {
'b_150680: {
'b_15063c: {
'b_150600: {
'b_1505ec: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5910));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.g[A0][0] = c.lw(c.g[V1][0] as u32);
let t = c.g[A0][0] as u32;
match t {
0x001505a8 => {
c.cycles += 17;
c.g[A0][0] = 0x1;
c.g[V1][0] = 0x2;
c.g[A1][0] = 0x3;
c.g[V0][0] = 0x7;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A1][0]);
c.sb(c.g[S0][0] as u32, c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
if t {
break 'b_1507e8;
} else {
break 'b_1505ec;
}
}
0x001505ec => {
break 'b_1505ec;
}
0x00150600 => {
break 'b_150600;
}
0x0015063c => {
break 'b_15063c;
}
0x00150680 => {
break 'b_150680;
}
0x001506a8 => {
break 'b_1506a8;
}
0x001506ec => {
break 'b_1506ec;
}
0x00150730 => {
break 'b_150730;
}
0x00150764 => {
break 'b_150764;
}
_ => { c.jump_out(t, 0x00150558, 0x001508d4); return Ok(()); }
}
}
c.cycles += 5;
c.g[V0][0] = 0x2;
c.g[A0][0] = 0x1;
c.g[A1][0] = 0x3;
let t = true;
c.g[V1][0] = 0x9;
if t {
break 'b_1506fc;
} else {
break 'b_150600;
}
}
c.cycles += 15;
c.g[V1][0] = 0x1;
c.g[A0][0] = 0x3;
c.g[A1][0] = 0x2;
c.g[V0][0] = 0x8;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A0][0]);
c.sb(c.g[S0][0] as u32, c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[A1][0]);
if t {
break 'b_1506e0;
} else {
break 'b_15063c;
}
}
c.cycles += 17;
c.g[V0][0] = 0x1;
c.g[V1][0] = 0x2;
c.g[A1][0] = 0x3;
c.g[A0][0] = 0x6;
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A0][0]);
c.sb(c.g[S0][0] as u32, c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[V1][0]);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
if t {
break 'b_1507e8;
} else {
break 'b_150680;
}
}
c.cycles += 10;
c.g[V1][0] = 0x1;
c.g[A0][0] = 0x2;
c.g[A1][0] = 0x3;
c.g[V0][0] = 0xa;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A1][0]);
let t = true;
c.sb(c.g[S0][0] as u32, c.g[V1][0]);
if t {
break 'b_1507cc;
} else {
break 'b_1506a8;
}
}
c.cycles += 14;
c.g[V0][0] = 0x2;
c.g[A1][0] = 0x3;
c.g[V1][0] = 0x1;
c.g[A0][0] = 0x7;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A0][0]);
c.sb(c.g[S0][0] as u32, 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[V0][0]);
break 'b_1506e0;
}
c.cycles += 3;
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), 0u64);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
if t {
break 'b_1507e8;
} else {
break 'b_1506ec;
}
}
c.cycles += 4;
c.g[V0][0] = 0x2;
c.g[A0][0] = 0x1;
c.g[A1][0] = 0x3;
c.g[V1][0] = 0x8;
break 'b_1506fc;
}
c.cycles += 13;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[V1][0]);
c.sb(c.g[S0][0] as u32, 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), 0u64);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), 0u64);
if t {
break 'b_1507e8;
} else {
break 'b_150730;
}
}
c.cycles += 13;
c.g[V0][0] = 0x1;
c.g[V1][0] = 0x2;
c.g[A1][0] = 0x3;
c.g[A0][0] = 0x9;
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[V1][0]);
c.sb(c.g[S0][0] as u32, c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[V1][0]);
if t {
break 'b_150794;
} else {
break 'b_150764;
}
}
c.cycles += 12;
c.g[V0][0] = 0x1;
c.g[V1][0] = 0x3;
c.g[A1][0] = 0x2;
c.g[A0][0] = 0xa;
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A0][0]);
c.sb(c.g[S0][0] as u32, c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[A1][0]);
break 'b_150794;
}
c.cycles += 5;
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), 0u64);
if t {
break 'b_1507e8;
} else {
break 'b_1507a8;
}
}
}
c.cycles += 9;
c.g[V1][0] = 0x1;
c.g[A0][0] = 0x3;
c.g[A1][0] = 0x2;
c.g[V0][0] = 0x6;
c.sb((c.g[S0][0] as u32).wrapping_add(0x13), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x19), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x36), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x37), c.g[A1][0]);
c.sb(c.g[S0][0] as u32, 0u64);
break 'b_1507cc;
}
c.cycles += 7;
c.sb((c.g[S0][0] as u32).wrapping_add(0x1), c.g[A1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[A0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x12), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
break 'b_1507e8;
}
c.cycles += 10;
c.g[V0][0] = 0x460000;
c.g[T2][0] = 0u64;
c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5148));
c.g[T5][0] = 0x6;
c.g[T9][0] = c.g[S2][0];
c.g[T8][0] = 0x14;
c.g[S1][0] = 0x1;
c.g[T7][0] = 0x6;
c.g[T6][0] = 0x14;
'l_150810: loop {
'b_150880: {
c.cycles += 10;
c.mult(0, 2, 10, 24);
c.g[A3][0] = 0u64;
c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
c.mult(0, 2, 10, 13);
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0xc));
c.sh((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
let t = c.g[V0][0] == 0u64;
c.sh((c.g[V1][0] as u32).wrapping_add(0x2), 0u64);
if t {
break 'b_150880;
} else {
c.cycles += 8;
c.mult(0, 2, 10, 15);
c.mult(0, 3, 10, 14);
c.g[T3][0] = 0x1;
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T9][0] as u32));
c.g[A2][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x2));
c.g[T1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc));
c.g[V1][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x4));
'l_150858: loop {
c.cycles += 8;
c.g[V0][0] = sx((c.g[T3][0] as u32) << ((c.g[A3][0] as u32) & 31));
c.g[A1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = c.g[A2][0] | c.g[V0][0];
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
c.g[A2][0] = c.g[A0][0];
c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = c.g[A1][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00150858); } continue 'l_150858;
} else {
c.cycles += 2;
c.sh((c.g[T0][0] as u32).wrapping_add(0x2), c.g[A0][0]);
c.sh((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
break 'b_150880;
}
}
}
}
c.cycles += 15;
c.mult(0, 2, 10, 13);
c.g[T2][0] = c.g[T4][0];
c.g[A2][0] = ((c.g[T2][0] as i64) < 15i64) as u64;
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[S1][0] as u32) << ((c.g[V0][0] as u32) & 31));
c.g[V1][0] = c.g[V1][0] & 0xfffc;
c.g[V0][0] = !(0u64 | c.g[V0][0]);
c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
c.g[A0][0] = c.g[A0][0] & 0xfffc;
c.sh((c.g[A1][0] as u32).wrapping_add(0x2), c.g[V1][0]);
let t = c.g[A2][0] != 0u64;
c.sh((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00150810); } continue 'l_150810;
} else {
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld(c.g[SP][0] as u32);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return Ok(());
}
}
    });
}

/// `Result* WScriptFn_IsEventType(Result* ret, int n, Arg* e)` (0x302778):
/// bind the script call's arguments into a 2-word record (defaults
/// `{-1, ...}` set once), convert the float ones, store `IsEventType(v[0])`
/// as the integer result and return `ret`.
pub fn w_script_fn_is_event_type(c: &mut Cpu) {
    const W_SCRIPT_FN_EVENT_TYPE_MATCHES: Callee = Callee { addr: 0x00303df8, f: ee::s10::f_00303df8 };
    guest(c, |c| {
'b_3027bc: {
c.cycles += 11;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2660));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S1][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S2][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S3][0] = c.g[A2][0];
let t = c.g[V0][0] != 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
if t {
break 'b_3027bc;
} else {
c.cycles += 6;
c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x265c));
c.g[V1][0] = 0xffffffffffffffff;
c.g[A0][0] = 0x1;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2658), c.g[V1][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2660), c.g[A0][0]);
c.sb(c.g[SP][0] as u32, c.g[V0][0]);
break 'b_3027bc;
}
}
'b_302830: {
c.cycles += 7;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A2][0] = 0x8;
c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2658));
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x003e6574, 0x003027d0)?;
let t = (c.g[S1][0] as i64) <= 0;
c.g[A2][0] = c.g[S3][0];
if t {
break 'b_302830;
} else {
c.cycles += 4;
c.g[A1][0] = c.g[S1][0];
c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xbe8));
c.g[A3][0] = 0x2;
'l_3027e8: loop {
'b_302824: {
'b_302820: {
c.cycles += 8;
c.g[V0][0] = c.lw(c.g[A2][0] as u32);
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
c.g[V0][0] = c.lw(c.g[V0][0] as u32);
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
break 'b_302820;
} else {
c.cycles += 2;
if c.g[V0][0] != c.g[A3][0] {
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
break 'b_302820;
} else {
c.cycles += 4;
c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
c.cvt_s_w(0, 0);
let t = true;
c.swc1(0, (c.g[A0][0] as u32));
if t {
break 'b_302824;
} else {
break 'b_302820;
}
}
}
}
c.cycles += 1;
c.sw(c.g[A0][0] as u32, c.g[V0][0]);
break 'b_302824;
}
c.cycles += 3;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] != 0u64;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003027e8); } continue 'l_3027e8;
} else {
break 'b_302830;
}
}
}
}
c.cycles += 16;
c.g[A0][0] = c.lw(c.g[S0][0] as u32);
W_SCRIPT_FN_EVENT_TYPE_MATCHES.call(c, 0x00302838)?;
c.g[V1][0] = 0x1;
c.g[S0][0] = c.g[V0][0];
c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00226610, 0x0030284c)?;
c.sw(c.g[V0][0] as u32, c.g[S0][0]);
c.g[V0][0] = c.g[S2][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t;
Ok(())
    });
}

/// `void cFlagManager_GroupAddFlag(FlagGroup* self, Def* def, Mesh* mesh)`
/// (0x34b038): the first flag copies the 0x40-byte definition into the group,
/// takes the mesh's first word and builds the group mesh; every flag is
/// appended to the mesh table at +0x94 and counted at +0x5c.
pub fn c_flag_manager_group_add_flag(c: &mut Cpu) {
    const C_FLAG_MANAGER_GROUP_BUILD_MESH: Callee = Callee { addr: 0x0034b228, f: ee::s11::f_0034b228 };
    guest(c, |c| {
'b_34b13c: {
c.cycles += 8;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.g[A0][0];
c.sd(c.g[SP][0] as u32, c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] != 0u64;
c.g[S1][0] = c.g[A2][0];
if t {
break 'b_34b13c;
} else {
'b_34b0f0: {
'b_34b0c0: {
c.cycles += 4;
c.g[V0][0] = c.g[A1][0] | c.g[S0][0];
c.g[V0][0] = c.g[V0][0] & 0x7;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = c.g[S0][0];
if t {
break 'b_34b0c0;
} else {
c.cycles += 1;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
'l_34b06c: loop {
c.cycles += 19;
c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(4, (c.g[A1][0] as u32));
c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(4, (c.g[V1][0] as u32));
c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
let t = c.g[A1][0] != c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x0034b06c); } continue 'l_34b06c;
} else {
c.cycles += 2;
let t = true;

if t {
break 'b_34b0f0;
} else {
break 'b_34b0c0;
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
'l_34b0c4: loop {
c.cycles += 11;
c.g[A0][0] = c.ld(c.g[A1][0] as u32);
c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
c.sd(c.g[V1][0] as u32, c.g[A0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
let t = c.g[A1][0] != c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x0034b0c4); } continue 'l_34b0c4;
} else {
break 'b_34b0f0;
}
}
}
c.cycles += 19;
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[V1][0] as u32));
c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S1][0];
c.g[V1][0] = c.lw(c.g[V0][0] as u32);
c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
C_FLAG_MANAGER_GROUP_BUILD_MESH.call(c, 0x0034b138)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
break 'b_34b13c;
}
}
c.cycles += 11;
c.g[RA][0] = c.ld(c.g[SP][0] as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.sw((c.g[V0][0] as u32).wrapping_add(0x94), c.g[S1][0]);
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t;
Ok(())
    });
}

/// `int CameraTrigger_ReadSphereTrigger(Reader* reader, Volume** out)`
/// (0x172e18): read a volume type word (0 ellipse, 1 box), allocate and read
/// the matching volume and return the bytes consumed. Any other type pushes
/// the stream back by the type word and returns 0.
pub fn camera_trigger_read_sphere_trigger(c: &mut Cpu) {
    const CAMERA_TRIGGER_READ_BOX_TRIGGER_DATA: Callee = Callee { addr: 0x001730d0, f: ee::s02::f_001730d0 };
    const CAMERA_TRIGGER_READ_SPHERE_TRIGGER_DATA: Callee = Callee { addr: 0x00172fe0, f: ee::s02::f_00172fe0 };
    guest(c, |c| {
'b_172fc0: {
'b_172fbc: {
'b_172fac: {
'b_172fa4: {
'b_172f08: {
'b_172e64: {
c.cycles += 14;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S2][0] = c.g[A0][0];
c.g[S4][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
call_ptr(c, 0x002d2558, 0x00172e44)?;
c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[S1][0] == 0u64;
c.g[S3][0] = c.g[V0][0];
if t {
break 'b_172e64;
} else {
c.cycles += 3;
c.g[V0][0] = 0x1;
let t = c.g[S1][0] == c.g[V0][0];
c.g[A1][0] = 0x460000;
if t {
break 'b_172f08;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = c.lw(c.g[S2][0] as u32);
if t {
break 'b_172fac;
} else {
break 'b_172e64;
}
}
}
}
c.cycles += 41;
c.g[A1][0] = 0x460000;
c.g[A2][0] = 0u64;
c.g[A3][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e78));
c.g[A0][0] = 0x2c;
call_ptr(c, 0x00317d70, 0x00172e7c)?;
c.g[V1][0] = 0x460000;
c.g[S0][0] = c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A1][0] = 0x500000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3cb0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S2][0];
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[A1][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
CAMERA_TRIGGER_READ_SPHERE_TRIGGER_DATA.call(c, 0x00172f00)?;
let t = true;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
if t {
break 'b_172fa4;
} else {
break 'b_172f08;
}
}
c.cycles += 39;
c.g[A2][0] = 0u64;
c.g[A3][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e60));
c.g[A0][0] = 0x2c;
call_ptr(c, 0x00317d70, 0x00172f1c)?;
c.g[V1][0] = 0x460000;
c.g[S0][0] = c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A1][0] = 0x500000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = 0x460000;
c.swc1(0, (c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d00));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S2][0];
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[S0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.g[A1][0] = c.g[S0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
CAMERA_TRIGGER_READ_BOX_TRIGGER_DATA.call(c, 0x00172fa0)?;
c.sw(c.g[S4][0] as u32, c.g[S0][0]);
break 'b_172fa4;
}
c.cycles += 2;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
if t {
break 'b_172fbc;
} else {
break 'b_172fac;
}
}
c.cycles += 4;
c.g[V0][0] = 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S3][0] as u32));
let t = true;
c.sw(c.g[S2][0] as u32, c.g[V1][0]);
if t {
break 'b_172fc0;
} else {
break 'b_172fbc;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S3][0];
break 'b_172fc0;
}
c.cycles += 8;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t;
Ok(())
    });
}

/// `void Rider_LoadBoltModels(cRider* self, void* mdfData)` (0x11c138): open
/// the MDF archive, and for each equipped bolt (up to four model names each)
/// look the model part up and add it as LOD 0 to the rider's anim model
/// (+0x780); then close the archive.
pub fn rider_load_bolt_models(c: &mut Cpu) {
    const RIDER_GET_EQUIPPED_BOLTS: Callee = Callee { addr: 0x0011bbe8, f: ee::s00::f_0011bbe8 };
    guest(c, |c| {
'b_11c1fc: {
c.cycles += 22;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S4][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[A0][0] = c.g[SP][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.g[S0][0] = c.g[A1][0];
call_ptr(c, 0x00314978, 0x0011c168)?;
c.g[A0][0] = c.g[SP][0];
c.g[A1][0] = c.g[S0][0];
call_ptr(c, 0x00314a18, 0x0011c174)?;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[S4][0];
c.g[S5][0] = c.g[A1][0];
RIDER_GET_EQUIPPED_BOLTS.call(c, 0x0011c184)?;
c.g[S3][0] = c.g[V0][0];
let t = (c.g[S3][0] as i64) <= 0;
c.g[A0][0] = 0u64;
if t {
break 'b_11c1fc;
} else {
c.cycles += 2;
c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
'l_11c198: loop {
c.cycles += 6;
c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.g[S1][0] = 0u64;
c.g[V0][0] = c.lw(c.g[V1][0] as u32);
c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
'l_11c1b0: loop {
'b_11c1e0: {
c.cycles += 3;
c.g[A1][0] = c.lw(c.g[S0][0] as u32);
if c.g[A1][0] == 0u64 {
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_11c1e0;
} else {
'b_11c1dc: {
c.cycles += 4;
c.g[A0][0] = c.g[SP][0];
call_ptr(c, 0x00314c00, 0x0011c1c4)?;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = c.g[V0][0];
if t {
break 'b_11c1dc;
} else {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x780));
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = 0u64;
call_ptr(c, 0x0030d8b8, 0x0011c1dc)?;
break 'b_11c1dc;
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_11c1e0;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S1][0] as i64) < 4i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x0011c1b0); } continue 'l_11c1b0;
} else {
c.cycles += 4;
c.g[A0][0] = c.g[S2][0];
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S3][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
if t {
if event_due(c) { deliver_event_here(c, 0x0011c198); } continue 'l_11c198;
} else {
break 'b_11c1fc;
}
}
}
}
}
}
c.cycles += 12;
c.g[A0][0] = c.g[SP][0];
c.g[A1][0] = 0x2;
call_ptr(c, 0x00314988, 0x0011c208)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
c.pc = t;
Ok(())
    });
}

/// `void RiderResetSub_HandleInput(ResetSub* self)` (0x12f398): move the
/// fade percentage (+4) towards 1.0 by `rider.speed * 0.025`; when it crosses
/// 0.5 put the rider back on the reset path (snap, put-at, boost), and when it
/// reaches 1.0 finish the transition (stop the transition manager, metrix
/// takeoff, anim, states).
pub fn rider_reset_sub_handle_input(c: &mut Cpu) {
    const RIDER_ADD_BOOST: Callee = Callee { addr: 0x0010e098, f: ee::s00::f_0010e098 };
    const RIDER_METRIX_INTERRUPT_TRICK: Callee = Callee { addr: 0x00119e38, f: ee::s00::f_00119e38 };
    const RIDER_METRIX_ON_RIDER_RESET: Callee = Callee { addr: 0x00119368, f: ee::s00::f_00119368 };
    const RIDER_PLACE: Callee = Callee { addr: 0x0011d660, f: ee::s00::f_0011d660 };
    const RIDER_RESET_MOTION: Callee = Callee { addr: 0x0011df18, f: ee::s00::f_0011df18 };
    const RIDER_RESET_SUB_IS_LOCAL_PLAYER: Callee = Callee { addr: 0x0012f588, f: ee::s00::f_0012f588 };
    const RIDER_SET_MAIN_STATE: Callee = Callee { addr: 0x0011fe78, f: ee::s00::f_0011fe78 };
    const RIDER_SET_SUB_STATE: Callee = Callee { addr: 0x0011fec8, f: ee::s00::f_0011fec8 };
    const RIDER_SNAP_TO_NEAREST_AI_PATH: Callee = Callee { addr: 0x00112d58, f: ee::s00::f_00112d58 };
    guest(c, |c| {
'b_12f3fc: {
'b_12f3e8: {
c.cycles += 18;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x771c));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x4));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
c.mul_s(2, 0, 2);
c.add_s(1, 2, 4);
c.c_lt_s(1, 3);
if !c.fcc() {
c.sub_s(0, 4, 2);
break 'b_12f3e8;
} else {
c.cycles += 2;
let t = true;
c.sub_s(1, 3, 2);
if t {
break 'b_12f3fc;
} else {
break 'b_12f3e8;
}
}
}
c.cycles += 4;
c.c_lt_s(3, 0);
let t = !c.fcc();
c.mov_s(1, 4);
if t {
break 'b_12f3fc;
} else {
c.cycles += 1;
c.add_s(1, 3, 2);
break 'b_12f3fc;
}
}
'b_12f4f0: {
'b_12f4ec: {
c.cycles += 6;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
c.c_le_s(3, 0);
let t = !c.fcc();
c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
if t {
break 'b_12f4ec;
} else {
c.cycles += 4;
c.c_lt_s(0, 1);
if !c.fcc() {
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
break 'b_12f4f0;
} else {
'b_12f480: {
'b_12f47c: {
c.cycles += 16;
c.g[AT][0] = 0x43480000;
c.mtc1(1, 20);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
RIDER_SNAP_TO_NEAREST_AI_PATH.call(c, 0x0012f434)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.g[SP][0];
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xab8));
c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x4c4));
call_ptr(c, 0x0026a8b8, 0x0012f448)?;
c.g[S1][0] = c.g[SP][0];
call_ptr(c, 0x0014dc80, 0x0012f450)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x0012f45c)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x530000;
if t {
break 'b_12f47c;
} else {
c.cycles += 4;
c.g[A0][0] = 0x2;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
if c.g[V1][0] != c.g[A0][0] {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
break 'b_12f480;
} else {
c.cycles += 2;
c.g[AT][0] = 0x447a0000;
c.mtc1(1, 20);
break 'b_12f47c;
}
}
}
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
break 'b_12f480;
}
'b_12f4d0: {
c.cycles += 11;
c.g[A2][0] = c.g[S1][0];
c.mov_s(12, 20);
c.g[A3][0] = 0x11f;
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x490));
RIDER_PLACE.call(c, 0x0012f494)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0x1;
RIDER_RESET_MOTION.call(c, 0x0012f4a0)?;
c.g[V0][0] = c.lw(c.g[S0][0] as u32);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
if t {
break 'b_12f4d0;
} else {
c.cycles += 9;
c.g[A1][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
RIDER_METRIX_ON_RIDER_RESET.call(c, 0x0012f4b8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.mov_s(12, 0);
c.g[A1][0] = 0x1;
RIDER_ADD_BOOST.call(c, 0x0012f4c8)?;
let t = true;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
if t {
break 'b_12f4f0;
} else {
break 'b_12f4d0;
}
}
}
c.cycles += 7;
c.g[A1][0] = 0x1;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
RIDER_METRIX_ON_RIDER_RESET.call(c, 0x0012f4dc)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.mov_s(12, 0);
c.g[A1][0] = 0x1;
RIDER_ADD_BOOST.call(c, 0x0012f4ec)?;
break 'b_12f4ec;
}
}
}
c.cycles += 1;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
break 'b_12f4f0;
}
'b_12f574: {
c.cycles += 6;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 20);
c.c_eq_s(0, 20);
if !c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
break 'b_12f574;
} else {
'b_12f530: {
c.cycles += 4;
c.g[A0][0] = c.g[S0][0];
RIDER_RESET_SUB_IS_LOCAL_PLAYER.call(c, 0x0012f510)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_12f530;
} else {
c.cycles += 6;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x870));
call_ptr(c, 0x00230698, 0x0012f528)?;
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x002e4578, 0x0012f530)?;
break 'b_12f530;
}
}
c.cycles += 17;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = 0u64;
c.g[A1][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
RIDER_METRIX_INTERRUPT_TRICK.call(c, 0x0012f544)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0x2;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x00311b20, 0x0012f554)?;
c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x90));
c.g[A1][0] = 0x4;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
RIDER_SET_SUB_STATE.call(c, 0x0012f564)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0x1;
RIDER_SET_MAIN_STATE.call(c, 0x0012f570)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
break 'b_12f574;
}
}
c.cycles += 5;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t;
Ok(())
    });
}

/// `void AnimObject_v42(AnimObject* self, Body* body)` (0x34e698): update the
/// intersect info and, for a body bound to a bone, add the bone's velocity
/// (position delta plus angular x offset) and angular velocity to the body.
pub fn anim_object_v42(c: &mut Cpu) {
    const ANIM_OBJECT_COMPUTE_NODE_VELOCITIES: Callee = Callee { addr: 0x0034e798, f: ee::s11::f_0034e798 };
    const ANIM_OBJECT_GET_NODE_POSITION: Callee = Callee { addr: 0x0034e600, f: ee::s11::f_0034e600 };
    const MOD_OBJ_V42: Callee = Callee { addr: 0x00356ae0, f: ee::s11::f_00356ae0 };
    guest(c, |c| {
'b_34e780: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = c.g[A1][0];
c.g[S2][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
MOD_OBJ_V42.call(c, 0x0034e6bc)?;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
if (c.g[S1][0] as i64) < 0 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_34e780;
} else {
'b_34e6e0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
if c.g[V0][0] == 0u64 {
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
break 'b_34e6e0;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
ANIM_OBJECT_COMPUTE_NODE_VELOCITIES.call(c, 0x0034e6dc)?;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
break 'b_34e6e0;
}
}
c.cycles += 40;
c.g[A0][0] = c.g[S2][0];
c.g[A2][0] = c.g[S1][0];
c.g[A3][0] = c.g[SP][0];
ANIM_OBJECT_GET_NODE_POSITION.call(c, 0x0034e6f0)?;
c.lqc2(3, (c.g[S0][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A1][0] = sx((c.g[S1][0] as u32) << 5);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[A1][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(0x30));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_34e780;
}
}
c.cycles += 5;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t;
Ok(())
    });
}

/// `void CrowdMan2d_InitSlot(CrowdMan2d* self, int index, int rid)`
/// (0x229910): store the model handle for a crowd slot, look its model up
/// and derive the slot's billboard position and axes from the model's quad
/// corners (graphics vtable slot 97).
pub fn crowd_man_2d_init_slot(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S2][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[A0][0] = 0x3;
c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = 0xffffffffffffffff;
'l_229950: loop {
c.cycles += 7;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
let t = c.g[A0][0] != c.g[V0][0];
c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
if t {
if event_due(c) { deliver_event_here(c, 0x00229950); } continue 'l_229950;
} else {
'b_2299dc: {
'b_2299b8: {
c.cycles += 17;
c.g[S3][0] = sx((c.g[A1][0] as u32) << 6);
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
c.g[V1][0] = 0x3;
c.g[A0][0] = 0xffffffffffffffff;
c.g[V1][0] = 0x3;
c.g[A0][0] = 0xffffffffffffffff;
c.g[V1][0] = 0x3;
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw(c.g[V0][0] as u32);
c.g[V1][0] = c.g[A2][0] & 0xff;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
c.g[V1][0] = c.lw(c.g[V1][0] as u32);
if c.g[V1][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
break 'b_2299b8;
} else {
c.cycles += 2;
let t = true;
c.g[S0][0] = 0u64;
if t {
break 'b_2299dc;
} else {
break 'b_2299b8;
}
}
}
'b_2299d8: {
c.cycles += 7;
c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
c.g[V1][0] = c.lw(c.g[V0][0] as u32);
c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_2299d8;
} else {
c.cycles += 1;
c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
break 'b_2299d8;
}
}
c.cycles += 1;
c.g[S0][0] = c.g[A1][0];
break 'b_2299dc;
}
c.cycles += 15;
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
c.g[A3][0] = c.g[S1][0];
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = 0u64;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x308));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x002299fc)?;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[A3][0] = c.g[S1][0];
c.g[A0][0] = c.g[SP][0];
c.g[T0][0] = 0x3;
'l_229a18: loop {
c.cycles += 24;
c.lwc1(1, (c.g[A3][0] as u32));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x8));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xec));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[A0][0] as u32));
let t = (c.g[T0][0] as i64) >= 0;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00229a18); } continue 'l_229a18;
} else {
c.cycles += 69;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x51f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.mfc1(4, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.qmtc2(4, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[AT][0] = 0x43160000;
c.mtc1(1, 2);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x210));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mfc1(4, 1);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.qmtc2(4, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x220));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mfc1(4, 1);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.qmtc2(4, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x230));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd8));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), 0u64);
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x210));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x210));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
c.pc = t; return Ok(());
}
}
}
}
    });
}

/// `void Rider_InitTrackPaths(cRider* self, int place)` (0x112180): find the
/// rider's start marker; with `place` put the rider there; then bind the
/// rider's two path objects (+0xab4/+0xab8) and compute its track and
/// pace distances.
pub fn rider_init_track_paths(c: &mut Cpu) {
    const RIDER_GET_START_POSE_ANGLE: Callee = Callee { addr: 0x00115b08, f: ee::s00::f_00115b08 };
    const RIDER_PLACE: Callee = Callee { addr: 0x0011d660, f: ee::s00::f_0011d660 };
    guest(c, |c| {
'b_112250: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S0][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S1][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A0][0] = 0x4d0000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[A1][0] = 0x1;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x86c));
call_ptr(c, 0x0026b5e0, 0x001121b4)?;
c.g[S2][0] = c.g[V0][0];
let t = c.g[S0][0] == 0u64;
c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_112250;
} else {
c.cycles += 36;
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x14));
c.g[A0][0] = c.g[S1][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.swc1(1, (c.g[SP][0] as u32));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x1c));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
RIDER_GET_START_POSE_ANGLE.call(c, 0x00112208)?;
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mtc1(0, 12);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A1][0] = c.g[S3][0];
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.g[SP][0];
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0x5;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
RIDER_PLACE.call(c, 0x00112250)?;
break 'b_112250;
}
}
'b_1122c8: {
c.cycles += 28;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xabc));
c.g[V1][0] = 0xffffffffffffffff;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = c.g[SP][0];
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac0));
c.g[T1][0] = 0x1;
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac0));
c.sw((c.g[S1][0] as u32).wrapping_add(0xab4), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.sw((c.g[S1][0] as u32).wrapping_add(0xab8), c.g[V0][0]);
c.sw(c.g[SP][0] as u32, 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
call_ptr(c, 0x0026a638, 0x001122a8)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab4));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x38));
c.c_lt_s(1, 0);
if !c.fcc() {
c.sub_s(0, 1, 0);
break 'b_1122c8;
} else {
c.cycles += 2;
c.mtc1(0, 0);
c.sub_s(0, 1, 0);
break 'b_1122c8;
}
}
c.cycles += 27;
c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xabc));
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c8));
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x490));
c.g[T1][0] = 0x1;
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4d0));
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4d4));
call_ptr(c, 0x0026a638, 0x001122ec)?;
c.g[AT][0] = 0x44470000;
c.mtc1(1, 12);
c.g[A0][0] = c.g[S3][0];
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
c.g[A2][0] = 0u64;
c.add_s(12, 0, 12);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c4));
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c0));
call_ptr(c, 0x0026ab20, 0x00112310)?;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x4a0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t;
Ok(())
    });
}

/// `Node* InputParser_ParseFunction2(Parser* self, int op)` (0x324138): parse
/// the `( a , b )` arguments of a two-argument expression function; with two
/// constant operands fold `min`/`max`/`pow`/`atan2` (ops 21, 20, 22, 23),
/// else build a binary node. Errors free the parsed operands and return 0.
/// Kept as the generated block dispatcher (`pc` is the block address).
pub fn input_parser_parse_function_2(c: &mut Cpu) {
    const INPUT_PARSER_ALLOC_NODE: Callee = Callee { addr: 0x003215f8, f: ee::s10::f_003215f8 };
    const INPUT_PARSER_ERROR: Callee = Callee { addr: 0x00321500, f: ee::s10::f_00321500 };
    const INPUT_PARSER_FREE_NODE: Callee = Callee { addr: 0x00321638, f: ee::s10::f_00321638 };
    const INPUT_PARSER_NEXT_TOKEN: Callee = Callee { addr: 0x00321af0, f: ee::s10::f_00321af0 };
    const INPUT_PARSER_PARSE_CONDITIONAL: Callee = Callee { addr: 0x00323268, f: ee::s10::f_00323268 };
    const MATH_ATAN: Callee = Callee { addr: 0x0031c228, f: ee::s10::f_0031c228 };
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00324138 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[V0][0] = 0x9;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[S3][0] = c.g[A1][0];
                if t { pc = 0x00324184; continue; }
                pc = 0x00324170; continue;
            }
            0x00324170 => {
                c.cycles += 3;
                c.g[A1][0] = 0x490000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e68));
INPUT_PARSER_ERROR.call(c, 0x0032417c)?;
                pc = 0x0032417c; continue;
            }
            0x0032417c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x0032436c; continue; }
                pc = 0x00324184; continue;
            }
            0x00324184 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_NEXT_TOKEN.call(c, 0x0032418c)?;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_PARSE_CONDITIONAL.call(c, 0x00324194)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64; 
                c.g[V0][0] = 0x8;
if t { if back_edge_unwinds(c, 0x0032417c) { return Ok(()); } pc = 0x0032417c; continue; }
                pc = 0x003241a0; continue;
            }
            0x003241a0 => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[A1][0] = 0x490000;
                if t { pc = 0x003241c4; continue; }
                pc = 0x003241ac; continue;
            }
            0x003241ac => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e48));
INPUT_PARSER_ERROR.call(c, 0x003241b8)?;
                c.g[A0][0] = c.g[S0][0];
                let t = true; 
                c.g[A1][0] = c.g[S1][0];
                if t { pc = 0x0032420c; continue; }
                pc = 0x003241c4; continue;
            }
            0x003241c4 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_NEXT_TOKEN.call(c, 0x003241cc)?;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_PARSE_CONDITIONAL.call(c, 0x003241d4)?;
                c.g[S2][0] = c.g[V0][0];
                let t = c.g[S2][0] == 0u64; 
                c.g[V0][0] = 0xa;
if t { if back_edge_unwinds(c, 0x0032417c) { return Ok(()); } pc = 0x0032417c; continue; }
                pc = 0x003241e0; continue;
            }
            0x003241e0 => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[A1][0] = 0x490000;
                if t { pc = 0x0032421c; continue; }
                pc = 0x003241ec; continue;
            }
            0x003241ec => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f48));
INPUT_PARSER_ERROR.call(c, 0x003241f8)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_FREE_NODE.call(c, 0x00324204)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                pc = 0x0032420c; continue;
            }
            0x0032420c => {
                c.cycles += 4;
INPUT_PARSER_FREE_NODE.call(c, 0x00324214)?;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x0032436c; continue; }
                pc = 0x0032421c; continue;
            }
            0x0032421c => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_NEXT_TOKEN.call(c, 0x00324224)?;
                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                c.g[V0][0] = 0x4;
                let t = c.g[V1][0] != c.g[V0][0]; 
                
                if t { pc = 0x00324354; continue; }
                pc = 0x00324234; continue;
            }
            0x00324234 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                let t = c.g[V0][0] != c.g[V1][0]; 
                c.g[V0][0] = 0x15;
                if t { pc = 0x00324354; continue; }
                pc = 0x00324240; continue;
            }
            0x00324240 => {
                c.cycles += 2;
                let t = c.g[S3][0] == c.g[V0][0]; 
                c.g[V0][0] = ((c.g[S3][0] as i64) < 22i64) as u64;
                if t { pc = 0x0032427c; continue; }
                pc = 0x00324248; continue;
            }
            0x00324248 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x14;
                if t { pc = 0x00324260; continue; }
                pc = 0x00324250; continue;
            }
            0x00324250 => {
                c.cycles += 2;
                let t = c.g[S3][0] == c.g[V0][0]; 
                c.g[A0][0] = c.g[S0][0];
                if t { pc = 0x0032428c; continue; }
                pc = 0x00324258; continue;
            }
            0x00324258 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x00324344; continue; }
                pc = 0x00324260; continue;
            }
            0x00324260 => {
                c.cycles += 3;
                c.g[V0][0] = 0x16;
                let t = c.g[S3][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x17;
                if t { pc = 0x0032429c; continue; }
                pc = 0x0032426c; continue;
            }
            0x0032426c => {
                c.cycles += 2;
                let t = c.g[S3][0] == c.g[V0][0]; 
                c.g[A0][0] = c.g[S0][0];
                if t { pc = 0x003242b8; continue; }
                pc = 0x00324274; continue;
            }
            0x00324274 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x00324344; continue; }
                pc = 0x0032427c; continue;
            }
            0x0032427c => {
                c.cycles += 4;
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.min_s(0, 0, 1);
                if t { pc = 0x0032433c; continue; }
                pc = 0x0032428c; continue;
            }
            0x0032428c => {
                c.cycles += 4;
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.max_s(0, 0, 1);
                if t { pc = 0x0032433c; continue; }
                pc = 0x0032429c; continue;
            }
            0x0032429c => {
                c.cycles += 7;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x4));
call_ptr(c, 0x0040da10, 0x003242a8)?;
                c.mul_s(12, 0, 20);
call_ptr(c, 0x0040d758, 0x003242b0)?;
                let t = true; 
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                if t { pc = 0x00324340; continue; }
                pc = 0x003242b8; continue;
            }
            0x003242b8 => {
                c.cycles += 6;
                c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.mtc1(0, 22);
                c.c_eq_s(20, 22);
                let t = !c.fcc(); 
                c.lwc1(21, (c.g[S1][0] as u32).wrapping_add(0x4));
                if t { pc = 0x003242fc; continue; }
                pc = 0x003242d0; continue;
            }
            0x003242d0 => {
                c.cycles += 4;
                c.c_eq_s(21, 22);
                let t = c.fcc(); 
                c.mov_s(0, 21);
                if t { pc = 0x0032433c; continue; }
                pc = 0x003242e0; continue;
            }
            0x003242e0 => {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e3c));
                
                if !c.fcc() { c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4)); pc = 0x00324340; continue; }
                pc = 0x003242f0; continue;
            }
            0x003242f0 => {
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e38));
                let t = true; 
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                if t { pc = 0x00324340; continue; }
                pc = 0x003242fc; continue;
            }
            0x003242fc => {
                c.cycles += 7;
                c.div_s(12, 21, 20);
MATH_ATAN.call(c, 0x00324308)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc(); 
                c.mov_s(12, 0);
                if t { pc = 0x00324338; continue; }
                pc = 0x00324318; continue;
            }
            0x00324318 => {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                let t = !c.fcc(); 
                
                if t { pc = 0x00324330; continue; }
                pc = 0x00324324; continue;
            }
            0x00324324 => {
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e34));
                let t = true; 
                c.add_s(12, 12, 0);
                if t { pc = 0x00324338; continue; }
                pc = 0x00324330; continue;
            }
            0x00324330 => {
                c.cycles += 2;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e30));
                c.sub_s(12, 12, 0);
                pc = 0x00324338; continue;
            }
            0x00324338 => {
                c.cycles += 1;
                c.mov_s(0, 12);
                pc = 0x0032433c; continue;
            }
            0x0032433c => {
                c.cycles += 1;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                pc = 0x00324340; continue;
            }
            0x00324340 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                pc = 0x00324344; continue;
            }
            0x00324344 => {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
INPUT_PARSER_FREE_NODE.call(c, 0x0032434c)?;
                let t = true; 
                c.g[V0][0] = c.g[S1][0];
                if t { pc = 0x0032436c; continue; }
                pc = 0x00324354; continue;
            }
            0x00324354 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
INPUT_PARSER_ALLOC_NODE.call(c, 0x0032435c)?;
                c.g[V1][0] = c.g[V0][0];
                c.sw(c.g[V1][0] as u32, c.g[S3][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                pc = 0x0032436c; continue;
            }
            0x0032436c => {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00324138, 0x00324394); return Ok(()); }
        }
    }
    });
}

/// `void* InSeq_HeapAlloc(Heap* heap, uint size)` (0x272308): first-fit
/// allocation from the free list at +8 (size rounded up to 4; a block is
/// split when it leaves 16 bytes more); returns the block's data or null.
pub fn in_seq_heap_alloc(c: &mut Cpu) {
'b_27239c: {
c.cycles += 10;
c.g[V0][0] = 0xffffffffffff0000;
c.g[T1][0] = c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
c.g[V0][0] = c.g[V0][0] | 0xfffc;
c.g[A1][0] = c.g[A1][0] & c.g[V0][0];
c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[A3][0] = 0u64;
let t = c.g[A2][0] == 0u64;
c.g[A0][0] = 0u64;
if t {
break 'b_27239c;
} else {
c.cycles += 2;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw(c.g[A2][0] as u32);
'l_272338: loop {
'b_272390: {
c.cycles += 3;
c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
if c.g[V0][0] != 0u64 {
c.g[A3][0] = c.g[A2][0];
break 'b_272390;
} else {
'b_27236c: {
c.cycles += 3;
c.g[V0][0] = (c.g[T2][0] < c.g[V1][0]) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
if t {
break 'b_27236c;
} else {
c.cycles += 7;
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
c.sw(c.g[A0][0] as u32, c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
c.sw(c.g[A2][0] as u32, c.g[A1][0]);
break 'b_27236c;
}
}
'b_272388: {
'b_272380: {
c.cycles += 2;
let t = c.g[A3][0] != 0u64;
c.g[A0][0] = c.g[A2][0];
if t {
break 'b_272380;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
let t = true;
c.sw((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
if t {
break 'b_272388;
} else {
break 'b_272380;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
break 'b_272388;
}
c.cycles += 2;
let t = true;
c.sw((c.g[A2][0] as u32).wrapping_add(0x4), 0u64);
if t {
break 'b_27239c;
} else {
break 'b_272390;
}
}
}
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
if c.g[A2][0] != 0u64 {
c.g[V1][0] = c.lw(c.g[A2][0] as u32);
if event_due(c) { deliver_event_here(c, 0x00272338); } continue 'l_272338;
} else {
break 'b_27239c;
}
}
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
c.pc = t;
}

/// `void BE_ApplyDefaultCharProfileBolts(CharProfile* self)` (0x1513b8):
/// equip the character's default bolts: mark the table-4 prizes, set
/// flags 0x26 on the unlocked items, mark and equip the free (and the
/// `-1` priced) bolts, then update the item flags.
pub fn be_apply_default_char_profile_bolts(c: &mut Cpu) {
    const BE_SET_CHAR_PROFILE_BOLT_EQUIPPED: Callee = Callee { addr: 0x00151c90, f: ee::s01::f_00151c90 };
    const BOLTS_GET_CHAR_BOLTS: Callee = Callee { addr: 0x0014d998, f: ee::s01::f_0014d998 };
    const BOLTS_GET_CHAR_BOLT_COUNT: Callee = Callee { addr: 0x0014d988, f: ee::s01::f_0014d988 };
    const BOLTS_GET_CHAR_TABLE4: Callee = Callee { addr: 0x0014dc50, f: ee::s01::f_0014dc50 };
    const BOLTS_GET_CHAR_TABLE4_COUNT: Callee = Callee { addr: 0x0014dc40, f: ee::s01::f_0014dc40 };
    guest(c, |c| {
'b_151424: {
c.cycles += 19;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.g[V0][0] = 0x4a0000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
c.sd(c.g[SP][0] as u32, c.g[RA][0]);
c.g[A0][0] = c.g[S3][0];
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
BOLTS_GET_CHAR_TABLE4_COUNT.call(c, 0x001513ec)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
c.g[S0][0] = c.g[V0][0];
c.g[A0][0] = c.g[S3][0];
BOLTS_GET_CHAR_TABLE4.call(c, 0x001513fc)?;
let t = (c.g[S0][0] as i64) <= 0;
c.g[S1][0] = c.g[V0][0];
if t {
break 'b_151424;
} else {
c.cycles += 1;
'l_151408: loop {
c.cycles += 7;
c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
c.g[A0][0] = c.g[S2][0];
c.g[A2][0] = 0x1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
BE_SET_CHAR_PROFILE_BOLT_EQUIPPED.call(c, 0x0015141c)?;
let t = c.g[S0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x00151408); } continue 'l_151408;
} else {
break 'b_151424;
}
}
}
}
'b_151460: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
let t = (c.g[V0][0] as i64) <= 0;
c.g[A1][0] = 0u64;
if t {
break 'b_151460;
} else {
c.cycles += 2;
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x290));
'l_151438: loop {
'b_15144c: {
c.cycles += 4;
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.g[V1][0] & 0x10;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[V1][0] | 0x26;
if t {
break 'b_15144c;
} else {
c.cycles += 1;
c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
break 'b_15144c;
}
}
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00151438); } continue 'l_151438;
} else {
break 'b_151460;
}
}
}
}
'b_15154c: {
c.cycles += 10;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
c.g[A0][0] = c.g[S3][0];
BOLTS_GET_CHAR_BOLT_COUNT.call(c, 0x0015146c)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
c.g[A0][0] = c.g[S3][0];
c.g[S1][0] = c.g[V0][0];
BOLTS_GET_CHAR_BOLTS.call(c, 0x0015147c)?;
c.g[S0][0] = c.g[V0][0];
let t = (c.g[S1][0] as i64) <= 0;
c.g[A0][0] = 0u64;
if t {
break 'b_15154c;
} else {
c.cycles += 2;
c.g[S3][0] = 0xa;
c.g[S4][0] = 0xffffffffffffffff;
'l_151490: loop {
'b_1514a0: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
let t = (c.g[V0][0] as i64) <= 0;
c.g[V1][0] = c.g[V0][0];
if t {
break 'b_1514a0;
} else {
c.cycles += 1;
c.mult(0, 3, 2, 19);
break 'b_1514a0;
}
}
'b_151544: {
'b_151540: {
'b_1514e4: {
c.cycles += 2;
let t = c.g[V1][0] != 0u64;

if t {
break 'b_1514e4;
} else {
'b_1514d4: {
'b_1514d0: {
c.cycles += 7;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x288));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.g[V0][0] = c.lh(c.g[V0][0] as u32);
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
if t {
break 'b_1514d0;
} else {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
let t = true;
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
if t {
break 'b_1514d4;
} else {
break 'b_1514d0;
}
}
}
c.cycles += 1;
c.g[V1][0] = 0u64;
break 'b_1514d4;
}
c.cycles += 4;
c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.g[V0][0] | 0x2;
let t = true;
c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
if t {
break 'b_151540;
} else {
break 'b_1514e4;
}
}
}
'b_1514ec: {
c.cycles += 2;
if (c.g[V0][0] as i64) > 0 {
c.mult(0, 2, 2, 19);
break 'b_1514ec;
} else {
break 'b_1514ec;
}
}
c.cycles += 2;
if c.g[V0][0] != c.g[S4][0] {
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
break 'b_151544;
} else {
'b_151520: {
'b_15151c: {
c.cycles += 7;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x288));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.g[V0][0] = c.lh(c.g[V0][0] as u32);
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
if t {
break 'b_15151c;
} else {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
let t = true;
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
if t {
break 'b_151520;
} else {
break 'b_15151c;
}
}
}
c.cycles += 1;
c.g[V1][0] = 0u64;
break 'b_151520;
}
c.cycles += 8;
c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[A0][0] = c.g[S2][0];
c.g[A2][0] = 0x1;
c.g[V0][0] = c.g[V0][0] | 0x2;
c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
BE_SET_CHAR_PROFILE_BOLT_EQUIPPED.call(c, 0x0015153c)?;
c.g[A0][0] = 0x1;
break 'b_151540;
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
break 'b_151544;
}
c.cycles += 2;
let t = c.g[S1][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
if t {
if event_due(c) { deliver_event_here(c, 0x00151490); } continue 'l_151490;
} else {
break 'b_15154c;
}
}
}
}
'b_151598: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
break 'b_151598;
} else {
'b_151594: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
let t = (c.g[V0][0] as i64) <= 0;
c.g[A1][0] = 0u64;
if t {
break 'b_151594;
} else {
c.cycles += 2;
c.g[A2][0] = 0xfffffffffffffffb;
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x290));
'l_151568: loop {
'b_15157c: {
c.cycles += 4;
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.g[V1][0] & 0x10;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.g[V1][0] | 0x6;
if t {
break 'b_15157c;
} else {
c.cycles += 1;
c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
break 'b_15157c;
}
}
c.cycles += 6;
c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00151568); } continue 'l_151568;
} else {
break 'b_151594;
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
break 'b_151598;
}
}
c.cycles += 7;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld(c.g[SP][0] as u32);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t;
Ok(())
    });
}

/// `bool ScriptEngine_IsHUDAllowed(ScriptEngine* self)` (0x2792e0): true
/// while a NIS list is playing (index 2); else false while frames are
/// no-skip, otherwise whether the current list can be skipped and is not in
/// play state 3.
pub fn script_engine_is_hud_allowed(c: &mut Cpu) {
    const SCRIPT_LIST_CAN_SKIP: Callee = Callee { addr: 0x00277060, f: ee::s07::f_00277060 };
    const SCRIPT_LIST_GET_PLAY_STATUS: Callee = Callee { addr: 0x00276bd8, f: ee::s07::f_00276bd8 };
    guest(c, |c| {
'b_279354: {
'b_279310: {
c.cycles += 10;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.g[V0][0] = 0x2;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd(c.g[SP][0] as u32, c.g[RA][0]);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x550));
if c.g[A0][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x554));
break 'b_279310;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_279354;
} else {
break 'b_279310;
}
}
}
'b_279350: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[S2][0] = 0u64;
if t {
break 'b_279350;
} else {
c.cycles += 7;
c.g[S1][0] = 0xcc;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54c));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
SCRIPT_LIST_CAN_SKIP.call(c, 0x0027932c)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_279354;
} else {
c.cycles += 7;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x550));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54c));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
SCRIPT_LIST_GET_PLAY_STATUS.call(c, 0x00279348)?;
c.g[V0][0] = c.g[V0][0] ^ 0x3;
c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_279350;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S2][0];
break 'b_279354;
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld(c.g[SP][0] as u32);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t;
Ok(())
    });
}

/// `void RiderGateSub_Enter(GateSub* self)` (0x12be20): reset the rider's
/// anim controller for the start gate (facing from +0x324, fakie turns it
/// by pi), set the facing quaternion, play the gate anim (1 when the state
/// is 2, else 0 and the state cleared), and clear the gate timers.
pub fn rider_gate_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
'b_12be80: {
c.cycles += 22;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S3][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x324));
c.sw((c.g[A0][0] as u32).wrapping_add(0x320), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x00311a50, 0x0012be54)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.mtc1(0, 0);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x320));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x320));
if c.g[V0][0] == 0u64 {
c.neg_s(0, 0);
break 'b_12be80;
} else {
c.cycles += 2;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7844));
c.neg_s(0, 0);
break 'b_12be80;
}
}
'b_12bf3c: {
'b_12bf1c: {
c.cycles += 31;
c.g[V0][0] = 0x500000;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 12);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
c.g[S1][0] = 0x500000;
c.lq(3, (c.g[V0][0] as u32));
c.mul_s(12, 0, 12);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
c.sq(3, (c.g[S2][0] as u32).wrapping_add(0x30));
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
call_ptr(c, 0x0031be50, 0x0012beb4)?;
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = 0x2;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_sub(0xea0));
c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x4));
c.mul_s(1, 0, 1);
c.mul_s(2, 0, 2);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
c.mul_s(0, 0, 4);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
c.swc1(2, (c.g[SP][0] as u32));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lq(2, (c.g[SP][0] as u32));
c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw(c.g[S3][0] as u32);
let t = c.g[V1][0] != c.g[A0][0];
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
if t {
break 'b_12bf1c;
} else {
c.cycles += 8;
c.g[A1][0] = 0x1;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[A2][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x003128e8, 0x0012bf14)?;
let t = true;
c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
if t {
break 'b_12bf3c;
} else {
break 'b_12bf1c;
}
}
}
c.cycles += 8;
c.g[A1][0] = 0u64;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[A2][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x003128e8, 0x0012bf34)?;
c.sw(c.g[S3][0] as u32, 0u64);
c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
break 'b_12bf3c;
}
c.cycles += 10;
c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t;
Ok(())
    });
}

/// `void ChaseCameraAlgorithm_ResetForRider(ChaseCam* self, int rider)`
/// (0x166c60): when the camera manager's current rider is `rider`, reset the
/// whole chase state (eye/target vectors, spring and lag fields, tuning from
/// the algorithm's vtable) and re-seat the camera behind the rider.
pub fn chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
'b_166f10: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
c.g[S1][0] = c.g[A0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
call_ptr(c, t, 0x00166c9c)?;
let t = c.g[S0][0] != c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_166f10;
} else {
c.cycles += 117;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.g[SP][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 20);
c.g[V0][0] = c.lw(c.g[A2][0] as u32);
c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
call_ptr(c, t, 0x00166cc8)?;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lw(c.g[A2][0] as u32);
c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
let t = c.g[V1][0] as u32;
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
call_ptr(c, t, 0x00166ce4)?;
c.g[AT][0] = 0x41200000;
c.mtc1(1, 0);
c.g[V0][0] = 0x1388;
c.sw((c.g[S1][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
c.g[S0][0] = 0x500000;
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x1e0));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xee0));
c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x1dc));
c.sw((c.g[S1][0] as u32).wrapping_add(0x1e4), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2d0), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2d8), 0u64);
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.lq(3, (c.g[S0][0] as u32));
c.lq(4, (c.g[SP][0] as u32));
c.sq(3, (c.g[S1][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0xd0));
c.lq(3, (c.g[S0][0] as u32));
c.sq(4, (c.g[S1][0] as u32).wrapping_add(0x80));
c.sq(3, (c.g[S1][0] as u32).wrapping_add(0xe0));
c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x68));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x00166d44)?;
c.g[V1][0] = 0x500000;
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x1d4));
c.sw((c.g[S1][0] as u32).wrapping_add(0x1e8), 0u64);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
c.sw((c.g[S1][0] as u32).wrapping_add(0x1ec), 0u64);
c.g[A2][0] = 0x1;
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[SP][0] as u32));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x68a0));
c.lq(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V0][0] = 0xffffffffffffffff;
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x224));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(5, (c.g[S1][0] as u32).wrapping_add(0xf0));
c.sw((c.g[S1][0] as u32).wrapping_add(0x2e4), c.g[A2][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x228));
c.sw((c.g[S1][0] as u32).wrapping_add(0x1f0), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2c8), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2dc), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x1f8), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2e0), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x204), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x208), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x210), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x20c), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x220), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x21c), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x214), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2e8), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2cc), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x23c), 0u64);
c.g[AT][0] = 0x437a0000;
c.mtc1(1, 0);
c.lq(2, (c.g[S0][0] as u32));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x170));
c.sw((c.g[S1][0] as u32).wrapping_add(0x244), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x240), 0u64);
c.sq(3, (c.g[S1][0] as u32).wrapping_add(0x190));
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 1);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[S1][0] as u32).wrapping_add(0x300), c.g[A2][0]);
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x1a0));
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x248));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[S1][0] as u32).wrapping_add(0x24c), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x250), 0u64);
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.lw(c.g[A2][0] as u32);
c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
call_ptr(c, t, 0x00166e48)?;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[S1][0] as u32).wrapping_add(0x2ec), 0u64);
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x340));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x1b0));
c.sw((c.g[S1][0] as u32).wrapping_add(0x2b4), 0u64);
c.g[V0][0] = 0x4;
c.sw((c.g[S1][0] as u32).wrapping_add(0x2b8), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2f0), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x2bc), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x37c), 0u64);
'l_166e78: loop {
c.cycles += 7;
c.sw(c.g[V1][0] as u32, 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0u64);
c.sw((c.g[V1][0] as u32).wrapping_add(0x28), 0u64);
let t = (c.g[V0][0] as i64) >= 0;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00166e78); } continue 'l_166e78;
} else {
c.cycles += 31;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.lw(c.g[A2][0] as u32);
c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
call_ptr(c, t, 0x00166eac)?;
c.g[AT][0] = 0x40000000;
c.mtc1(1, 0);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[S1][0] as u32).wrapping_add(0x1c0));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x100));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[S1][0] as u32).wrapping_add(0x108), 0u64);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x00166ef4)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[A1][0] = 0x1;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x00166f0c)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
break 'b_166f10;
}
}
}
}
c.cycles += 5;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.pc = t;
Ok(())
    });
}

/// `void AppModule_Update(AppModule* self)` (0x231cb0): forwards to
/// `AppModule_UpdateFade`.
pub fn app_module_update(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_1d18,
            f: ee::s06::f_00231d18,
        }),
        0x0023_1cc0,
    );
}


/// `int Rider_LoadListCompare(const char* a, const char* b)` (0x11b678):
/// the load list's sort callback: `strcasecmp`.
pub fn rider_load_list_compare(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0041_aa88), 0x0011_b688);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::ai::aianimeventmap::tests::alloc_setup;

    /// A block of words that all equal the block's own address ("pointer
    /// soup"): any pointer field set to it chains to itself, every `lh`
    /// vtable delta reads 0 (its address has the low 16 bits clear) and every
    /// vtable function pointer is an address outside the function table, so
    /// the machine's fake hook answers the call.
    pub(crate) const OC414_SOUP: u32 = DATA + 0x1_0000;

    pub(crate) fn oc414_soup(c: &mut Cpu) {
        for i in 0..0x400u32 {
            c.bus.write32(OC414_SOUP + 4 * i, OC414_SOUP);
        }
    }

    /// The BELibrary dummy (`gp-0x1f04`), a one-rider world (`gp-0x848`) and
    /// the pointer soup.
    pub(crate) fn oc414_env(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x9_0000);
        let (w, ail, arr) = (DATA + 0xa_0000, DATA + 0xa_1000, DATA + 0xa_2000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0xa_4000);
        c.bus.write32(w + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0xa_3000);
        c.bus.write32(arr + 0x7c, 1);
        oc414_soup(c);
        oc414_allocator(c);
    }

    /// The engine's size-class pool allocator (`gp + 0xdd8`) and the dlmalloc
    /// arena behind it (heap 0x4ff250 on `DATA+0x40000..+0x80000`); registers
    /// and `pc` are restored afterwards.
    pub(crate) fn oc414_allocator(c: &mut Cpu) {
        let (g, pc) = (c.g, c.pc);
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            for n in 0..16u32 {
                let node = nodes + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, nodes);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        c.g = g;
        c.pc = pc;
    }


    /// `CameraTrigger_ReadBoundObj`: every type word (0..3 allocate and read
    /// an ellipse/box/line/point, others fall out), random payload.
    #[test]
    fn camera_trigger_read_bound_obj_matches() {
        check(
            ee::s02::f_00171690__generated,
            camera_trigger_read_bound_obj,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x171690, 0);
                elf_image(c);
                alloc_setup(c, &mut r);
                c.pc = 0x0017_1690;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                let ty = [0u32, 1, 2, 3, 4, 0x1_0000][(seed % 6) as usize];
                for (k, b) in ty.to_be_bytes().iter().enumerate() {
                    c.bus.write8(DATA + 0x100 + k as u32, *b);
                }
                for k in 4..0x80u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }


    /// `BE_InitCharProfile`: with and without the bolt-map reload, every
    /// character kind (default tricks per kind), colour table row `k`.
    #[test]
    fn be_init_char_profile_matches() {
        check(
            ee::s01::f_00151600__generated,
            be_init_char_profile,
            36,
            &|c, seed| {
                let _ = machine(c, seed, 0x151600, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0015_1600;
                let this = DATA + 0x100;
                c.bus.write8(this + 0xbc0, (seed % 11) as u8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 3 == 0) as u64;
            },
        );
    }


    /// `Rider_Reset`: not loaded (load step 3) and loaded riders in every
    /// game mode / path combination; the real callees run on a mostly zero
    /// rider, the metrix/anim/path objects are zeroed blocks.
    #[test]
    fn rider_reset_matches() {
        check(ee::s00::f_0011d390__generated, rider_reset, 60, &|c, seed| {
            let _ = machine(c, seed, 0x11d390, 0);
            oc414_env(c);
            c.pc = 0x0011_d390;
            let this = DATA + 0x100;
            c.bus
                .write32(this + 0x880, if seed % 6 == 0 { 3 } else { 7 });
            c.bus.write32(this + 0x77c, DATA + 0x2000);
            c.bus.write32(this + 0x790, DATA + 0x2400);
            c.bus.write32(this + 0x784, DATA + 0x2800);
            c.bus.write32(this + 0x6c0, DATA + 0x3000);
            c.bus.write16(DATA + 0x3080, 0);
            c.bus.write32(DATA + 0x3084, FAKE);
            c.bus.write32(this + 0xabc, DATA + 0x3400);
            c.bus.write32(this + 0xac0, DATA + 0x3440);
            c.bus.write32(this + 0x86c, seed % 4);
            // The race setup: game mode (+0x48) and game path (+0x49).
            let mode = [0u8, 4, 5, 6, 1, 9][(seed % 6) as usize];
            c.bus.write8(0x0053_5bc8 + 0x48, mode);
            c.bus.write8(0x0053_5bc8 + 0x49, (seed / 6 % 2) as u8);
            c.g[A0][0] = sx(this);
        });
    }


    /// `BEBAGTInt_InitDefaultTricks`: every mode (0..8 and the default).
    #[test]
    fn bebagt_int_init_default_tricks_matches() {
        check(
            ee::s01::f_00150558__generated,
            bebagt_int_init_default_tricks,
            44,
            &|c, seed| {
                let mut r = machine(c, seed, 0x150558, 0);
                elf_image(c);
                c.pc = 0x0015_0558;
                c.g[A0][0] = (seed % 11) as u64;
                c.g[A1][0] = sx(DATA + 0x100);
                for k in 0..0x60u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }


    /// `WScriptFn_IsEventType`: first call (the record defaults are set) and
    /// later ones, 0..3 arguments whose type matches the table or not
    /// (matching: integer store; table type 2: float conversion).
    #[test]
    fn w_script_fn_is_event_type_matches() {
        check(
            ee::s10::f_00302778__generated,
            w_script_fn_is_event_type,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x302778, 0);
                c.pc = 0x0030_2778;
                oc414_env(c);
                let gp = GAME_GP;
                c.bus.write32(gp + 0x2660, (seed % 2) as u32);
                c.bus.write8(gp + 0x265c, r.next_u32() as u8);
                for i in 0..4u32 {
                    c.bus.write32(gp + 0xbe8 + 4 * i, 2 + r.next_u32() % 2);
                }
                let args = DATA + 0x200;
                let n = seed % 4;
                for i in 0..n {
                    let rec = args + 16 * i;
                    c.bus.write32(rec, r.next_u32() % 2);
                    c.bus.write32(rec + 4, r.next_u32() % 100);
                    c.bus.write32(rec + 0xc, 2 + r.next_u32() % 2);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(args);
            },
        );
    }


    /// `cFlagManager_GroupAddFlag`: the first flag (copy the definition,
    /// aligned and unaligned, build the mesh: the graphics vtable's mesh
    /// call returns 0, so the build returns early) and later flags.
    #[test]
    fn c_flag_manager_group_add_flag_matches() {
        check(
            ee::s11::f_0034b038__generated,
            c_flag_manager_group_add_flag,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b038, 0);
                c.pc = 0x0034_b038;
                let this = DATA + 0x100;
                let def = DATA + 0x600 + (seed % 2);
                let mesh = DATA + 0x700;
                let count = if seed % 3 == 0 { 0 } else { seed % 5 };
                c.bus.write32(this + 0x5c, count);
                for k in 0..0x40u32 {
                    c.bus.write8(def + k, r.next_u32() as u8);
                }
                c.bus.write32(mesh + 0x80, DATA + 0x780);
                c.bus.write32(DATA + 0x780, r.next_u32());
                // The graphics manager: vtable slot 97 is a fake, slot 76 a
                // real leaf that returns 0 (no mesh).
                let mgr = DATA + 0x4000;
                let vt = DATA + 0x4100;
                c.bus.write32(GAME_GP + 0x2a90, mgr);
                c.bus.write32(mgr + 0x10d8, vt);
                c.bus.write16(vt + 0x308, 0);
                c.bus.write32(vt + 0x30c, FAKE);
                c.bus.write16(vt + 0x260, 0);
                c.bus.write32(vt + 0x264, 0x0014_06c0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(def);
                c.g[A2][0] = sx(mesh);
            },
        );
    }


    /// `CameraTrigger_ReadSphereTrigger`: ellipse and box volumes, and
    /// other type words (the stream is pushed back).
    #[test]
    fn camera_trigger_read_sphere_trigger_matches() {
        check(
            ee::s02::f_00172e18__generated,
            camera_trigger_read_sphere_trigger,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x172e18, 0);
                elf_image(c);
                alloc_setup(c, &mut r);
                c.pc = 0x0017_2e18;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                let ty = [0u32, 1, 2, 3, 0x1_0000][(seed % 5) as usize];
                for (k, b) in ty.to_be_bytes().iter().enumerate() {
                    c.bus.write8(DATA + 0x100 + k as u32, *b);
                }
                for k in 4..0x80u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }


    /// `Rider_LoadBoltModels` with the real callees on a mostly zero rider
    /// (the equipped-bolt list is empty unless the fixture's BE state
    /// provides some).
    #[test]
    fn rider_load_bolt_models_matches() {
        check(
            ee::s00::f_0011c138__generated,
            rider_load_bolt_models,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11c138, 0);
                oc414_env(c);
                alloc_setup(c, &mut r);
                c.pc = 0x0011_c138;
                let this = DATA + 0x100;
                c.bus.write32(this + 0x780, DATA + 0x2000);
                c.bus.write32(this + 0x86c, seed % 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }


    /// `RiderResetSub_HandleInput`: fade percentages below, at and above the
    /// 0.5 crossing and 1.0 (the transition finish), several rider speeds
    /// (fade steps) and both reset-sub modes; the callees run for real on a
    /// mostly zero rider.
    #[test]
    fn rider_reset_sub_handle_input_matches() {
        check(
            ee::s00::f_0012f398__generated,
            rider_reset_sub_handle_input,
            64,
            &|c, seed| {
                let _ = machine(c, seed, 0x12f398, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0012_f398;
                let this = DATA + 0x100;
                let rider = DATA + 0x1000;
                let prev = [0.0f32, 0.3, 0.45, 0.5, 0.7, 1.0, 1.5, 2.0][(seed % 8) as usize];
                let speed = [0.0f32, 4.0, 20.0, 40.0, 100.0][(seed / 8 % 5) as usize];
                c.bus.write32(this, (seed / 40 % 2) as u32);
                c.bus.write32(this + 4, prev.to_bits());
                c.bus.write32(this + 8, rider);
                c.bus.write32(rider + 0x300, speed.to_bits());
                c.bus.write32(rider + 0xab8, DATA + 0x1800);
                c.bus.write32(rider + 0x790, DATA + 0x1900);
                c.bus.write32(rider + 0x784, DATA + 0x1a00);
                c.bus.write32(rider + 0x870, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// `AnimObject_v42`: a body without a bone (early out) and with bones,
    /// the anim object with and without a mesh velocity pass.
    #[test]
    fn anim_object_v42_matches() {
        check(
            ee::s11::f_0034e698__generated,
            anim_object_v42,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34e698, 0);
                oc414_env(c);
                c.pc = 0x0034_e698;
                let this = DATA + 0x100;
                let body = DATA + 0x400;
                let bone = [-1i32, 0, 1, 2][(seed % 4) as usize];
                c.bus.write32(this + 0x10, (seed / 4 % 2) as u32);
                c.bus.write32(this + 0x4c, DATA + 0x800);
                c.bus.write32(body + 0x5c, bone as u32);
                c.bus.write32(body + 0x50, DATA + 0x600);
                for k in 0..0x60u32 {
                    c.bus.write32(DATA + 0x800 + 4 * k, r.f32(10.0));
                }
                for k in 0..0x20u32 {
                    c.bus.write32(body + 4 * k, r.f32(10.0));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(body);
            },
        );
    }


    /// `CrowdMan2d_InitSlot`: a model handle that resolves (a model block with a
    /// matrix) or not, several slot indices; the graphics vtable is the
    /// pointer soup, so the quad fetch is a fake call.
    #[test]
    fn crowd_man_2d_init_slot_matches() {
        check(
            ee::s05::f_00229910__generated,
            crowd_man_2d_init_slot,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229910, 0);
                oc414_env(c);
                c.pc = 0x0022_9910;
                let this = DATA + 0x100;
                // The handle: low byte selects a table slot, high bits a
                // sub-entry; the model block is at `model`.
                let id = (seed % 3) | ((seed % 5) << 8);
                let model = DATA + 0x3000;
                let (g, h, x, y) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200, DATA + 0x2300);
                c.bus.write32(GAME_GP + 0x16c8, g);
                c.bus.write32(g, h);
                if seed % 4 != 0 {
                    c.bus.write32(h + 8 + 4 * (id & 0xff), x);
                    c.bus.write32(x + 0x1c, y);
                    let z = if seed % 7 == 0 { 0 } else { ((model >> 2) << 8) | 1 };
                    c.bus.write32(y + 4 * (id >> 8), z);
                }
                for k in 0..0x30u32 {
                    c.bus.write32(model + 4 * k, r.f32(100.0));
                }
                c.bus.write32(GAME_GP + 0x2a90, OC414_SOUP);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = id as u64;
            },
        );
    }


    /// `Rider_InitTrackPaths`: with and without placing the rider; the start
    /// marker table at 0x4d33a0 holds one start point whose paths are zeroed
    /// path objects.
    #[test]
    fn rider_init_track_paths_matches() {
        check(
            ee::s00::f_00112180__generated,
            rider_init_track_paths,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x112180, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0011_2180;
                let this = DATA + 0x100;
                let marker = DATA + 0x2000;
                let idx = seed % 3;
                c.bus.write32(this + 0x86c, idx);
                c.bus.write32(this + 0xabc, DATA + 0x3800);
                c.bus.write32(this + 0xac0, DATA + 0x3840);
                c.bus.write32(0x004d_33a0, 1);
                c.bus.write32(0x004d_33a4, marker);
                c.bus.write32(marker, idx);
                c.bus.write32(marker + 4, 1);
                for k in 2..8u32 {
                    c.bus.write32(marker + 4 * k, r.f32(50.0));
                }
                c.bus.write32(marker + 0x20, DATA + 0x3000);
                c.bus.write32(marker + 0x24, DATA + 0x3400);
                c.bus.write32(DATA + 0x3038, r.f32(100.0));
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    /// `InputParser_ParseFunction2`: the first-token error (the current token
    /// is not `(`), and `(` followed by an empty input (the first operand
    /// fails to parse). The folding paths need a tokenised expression and are
    /// not reached by this fixture.
    #[test]
    fn input_parser_parse_function_2_matches() {
        check(
            ee::s10::f_00324138__generated,
            input_parser_parse_function_2,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x324138, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0032_4138;
                let this = DATA + 0x100;
                c.bus
                    .write32(this + 0x98, if seed % 3 == 0 { 5 } else { 9 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [20u64, 21, 22, 23][(seed % 4) as usize];
            },
        );
    }


    /// `InSeq_HeapAlloc`: free lists of 0..3 blocks of assorted sizes against
    /// requests that fit exactly, split a block, or fit nowhere.
    #[test]
    fn in_seq_heap_alloc_matches() {
        check(
            ee::s07::f_00272308__generated,
            in_seq_heap_alloc,
            120,
            &|c, seed| {
                let mut r = Rng(0x2723_0800 ^ seed as u64);
                scramble(c, &mut r);
                let heap = DATA;
                let blocks = [DATA + 0x100, DATA + 0x200, DATA + 0x300];
                let n = (seed % 4) as usize;
                c.bus.write32(heap + 8, if n == 0 { 0 } else { blocks[0] });
                for i in 0..n {
                    let size = [8u32, 16, 40, 100, 200, 4][r.next_u32() as usize % 6];
                    c.bus.write32(blocks[i], size);
                    c.bus
                        .write32(blocks[i] + 4, if i + 1 < n { blocks[i + 1] } else { 0 });
                }
                let want = [1u32, 4, 8, 13, 24, 40, 90, 150][r.next_u32() as usize % 8];
                c.g[A0][0] = sx(heap);
                c.g[A1][0] = if seed % 5 == 0 {
                    want as u64 | ((r.next_u32() as u64) << 32)
                } else {
                    want as u64
                };
            },
        );
    }


    /// `BE_ApplyDefaultCharProfileBolts`: every character kind, 0..6 items
    /// with random flags and a random item map; the bolt tables come from the
    /// game image.
    #[test]
    fn be_apply_default_char_profile_bolts_matches() {
        check(
            ee::s01::f_001513b8__generated,
            be_apply_default_char_profile_bolts,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1513b8, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0015_13b8;
                let this = DATA + 0x100;
                c.bus.write8(this + 0xbc0, (seed % 12) as u8);
                let n = seed % 7;
                c.bus.write32(this + 0x28c, n);
                c.bus.write32(this + 0x288, DATA + 0x800);
                for i in 0..n {
                    c.bus.write16(this + 0x290 + 4 * i, i as u16);
                    c.bus.write16(this + 0x292 + 4 * i, r.next_u32() as u16);
                }
                for i in 0..64u32 {
                    c.bus
                        .write16(DATA + 0x800 + 2 * i, ((r.next_u32() % (n + 2)) as u16).wrapping_sub(1));
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// `ScriptEngine_IsHUDAllowed`: the playing list is 2 (allowed), the
    /// no-skip frame count is set, or the list's skip/play state decides.
    #[test]
    fn script_engine_is_hud_allowed_matches() {
        check(
            ee::s07::f_002792e0__generated,
            script_engine_is_hud_allowed,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2792e0, 0);
                oc414_env(c);
                c.pc = 0x0027_92e0;
                let this = DATA + 0x100;
                let lists = DATA + 0x1000;
                c.bus.write32(this + 0x550, seed % 4);
                c.bus.write32(this + 0x554, (seed / 4 % 2) as u32);
                c.bus.write32(this + 0x54c, lists);
                for k in 0..(4 * 0xcc / 4) as u32 {
                    c.bus.write32(lists + 4 * k, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// `RiderGateSub_Enter`: a rider facing forward and fakie, the gate state
    /// 2 (play anim 1) or not (anim 0, state cleared).
    #[test]
    fn rider_gate_sub_enter_matches() {
        check(
            ee::s00::f_0012be20__generated,
            rider_gate_sub_enter,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12be20, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0012_be20;
                let this = DATA + 0x100;
                let rider = DATA + 0x1000;
                c.bus.write32(this, [2u32, 0, 1][(seed % 3) as usize]);
                c.bus.write32(this + 0x14, rider);
                c.bus.write32(rider + 0x324, (seed / 3 % 2) as u32);
                c.bus.write32(rider + 0x784, DATA + 0x2000);
                for k in 1..8u32 {
                    c.bus.write32(this + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// `ChaseCameraAlgorithm_ResetForRider`: the camera manager's current
    /// rider (the result of vtable call 0, forced) equals the argument or
    /// not; all other calls are fakes through the pointer soup.
    #[test]
    fn chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s02::f_00166c60__generated,
            chase_camera_algorithm_reset_for_rider,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x166c60, 40);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0016_6c60;
                let this = DATA + 0x100;
                c.bus.write32(this + 0x30, OC414_SOUP);
                c.bus.write32(this + 0x10, OC414_SOUP);
                let want = 0x11 + seed % 3;
                c.bus.write32(LOG_RESULTS, want);
                for k in 0..0x80u32 {
                    c.bus.write32(this + 0x80 + 4 * k, r.f32(100.0));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (if seed % 2 == 0 { want } else { want + 1 }) as u64;
            },
        );
    }


    /// `AppModule_Update` forwards to `AppModule_UpdateFade` (real callee on
    /// a zeroed module).
    #[test]
    fn app_module_update_matches() {
        check(
            ee::s06::f_00231cb0__generated,
            app_module_update,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x231cb0, 0);
                oc414_env(c);
                c.pc = 0x0023_1cb0;
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }


    /// `Rider_LoadListCompare` is `strcasecmp` of its two arguments.
    #[test]
    fn rider_load_list_compare_matches() {
        check(
            ee::s00::f_0011b678__generated,
            rider_load_list_compare,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11b678, 0);
                c.pc = 0x0011_b678;
                let (a, b) = (DATA + 0x100, DATA + 0x200);
                for k in 0..8u32 {
                    let ch = b'a' + (r.next_u32() % 3) as u8;
                    c.bus.write8(a + k, ch);
                    let other = if seed % 2 == 0 { ch.to_ascii_uppercase() } else { ch };
                    c.bus.write8(b + k, if k == 6 && seed % 3 == 0 { b'z' } else { other });
                }
                c.bus.write8(a + 8, 0);
                c.bus.write8(b + 8, 0);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
            },
        );
    }

}
