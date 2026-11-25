//! Staging module oc668: dispatcher-shaped bodies converted from the generated
//! code, each with a differential test. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc668`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x00122ee8, rider_dispatch_query_table, "Rider_DispatchQueryTable"),
    (0x00123f38, rider_get_event_switch_start_pos, "Rider_GetEventSwitchStartPos"),
    (0x0012fff8, rider_board_press_sub_handle_airborne, "RiderBoardPressSub_HandleAirborne"),
    (0x001391a8, rider_handplant_state_update, "RiderHandplantState_Update"),
    (0x001453d0, be_new_race_int_get_target_score, "BENewRaceInt_GetTargetScore"),
    (0x00148158, be_char_int_get_saved_attrib, "BECharInt_GetSavedAttrib"),
    (0x0014d4c8, bolts_count_children, "Bolts_CountChildren"),
    (0x00155420, be_score_int_update_records, "BEScoreInt_UpdateRecords"),
    (0x00156390, be_post_race_get_interface, "BEPostRace_GetInterface"),
    (0x00156c70, be_reward_int_pick_random_bolt, "BERewardInt_PickRandomBolt"),
    (0x00157080, be_reward_int_pick_random_locked_reward, "BERewardInt_PickRandomLockedReward"),
    (0x00157f20, be_reward_int_unlock_reward, "BERewardInt_UnlockReward"),
    (0x0015af40, be_reward_int_get_reward_type_text, "BERewardInt_GetRewardTypeText"),
    (0x001606d8, manual_chase_camera_algorithm_reset_for_rider, "ManualChaseCameraAlgorithm_ResetForRider"),
    (0x0016ca18, camera_trigger_man_remove_trigger_regions, "CameraTriggerMan_RemoveTriggerRegions"),
    (0x0016e1d8, camera_trigger_apply_action, "CameraTrigger_ApplyAction"),
    (0x0017a158, debug_menu_dispatch_item_op, "DebugMenu_DispatchItemOp"),
];

/// `Rider_DispatchQueryTable` (0x00122ee8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_dispatch_query_table(c: &mut Cpu) {
    if c.pc != 0x00122ee8 {
        return ee::s00::f_00122ee8__generated(c);
    }
    guest(c, |c| {

'b_12310c: {
'b_123108: {
c.cycles += 8;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.g[V0][0] = (c.g[A1][0] < 0x13u64) as u64;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_123108;
} else {
'b_123100: {
'b_1230a8: {
'b_12309c: {
'b_123090: {
'b_123084: {
'b_123078: {
'b_123058: {
'b_123050: {
'b_123048: {
'b_123038: {
'b_12300c: {
'b_122fe0: {
'b_122fb4: {
'b_122fa0: {
'b_122f90: {
'b_122f48: {
'b_122f34: {
c.cycles += 7;
c.g[V0][0] = 0x450000;
c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7e30));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00122f24 => {
c.cycles += 4;
c.g[RA][0] = sx(0x00122f2c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00122f2c)?;
let t = true;
c.g[V0][0] = c.g[V0][0] ^ 0x2;
if t {
break 'b_123050;
} else {
break 'b_122f34;
}
}
0x00122f34 => {
break 'b_122f34;
}
0x00122f48 => {
break 'b_122f48;
}
0x00122f90 => {
break 'b_122f90;
}
0x00122fa0 => {
break 'b_122fa0;
}
0x00122fb4 => {
break 'b_122fb4;
}
0x00122fe0 => {
break 'b_122fe0;
}
0x0012300c => {
break 'b_12300c;
}
0x00123038 => {
break 'b_123038;
}
0x00123048 => {
break 'b_123048;
}
0x00123058 => {
break 'b_123058;
}
0x00123078 => {
break 'b_123078;
}
0x00123084 => {
break 'b_123084;
}
0x00123090 => {
break 'b_123090;
}
0x0012309c => {
break 'b_12309c;
}
0x001230a8 => {
break 'b_1230a8;
}
0x00123108 => {
break 'b_123108;
}
_ => { c.jump_out(t, 0x00122ee8, 0x00123124); return Ok(()); }
}
}
c.cycles += 5;
c.g[RA][0] = sx(0x00122f3c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0011fee8, f: ee::s00::Rider_GetSubState }.call(c, 0x00122f3c)?;
c.g[V0][0] = c.g[V0][0] ^ 0x9;
let t = true;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_12310c;
} else {
break 'b_122f48;
}
}
'b_122f88: {
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00122f54);
c.g[S2][0] = 0u64;
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00122f54)?;
c.g[S1][0] = 0x1;
if c.g[V0][0] == c.g[S1][0] {
c.g[S2][0] = 0x1;
break 'b_122f88;
} else {
c.cycles += 5;
c.g[RA][0] = sx(0x00122f68);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00122f68)?;
c.g[V1][0] = 0x2;
let t = c.g[V0][0] != c.g[V1][0];
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_12310c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
let t = c.g[V1][0] != c.g[S1][0];
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_12310c;
} else {
c.cycles += 1;
c.g[S2][0] = 0x1;
break 'b_122f88;
}
}
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_12310c;
} else {
break 'b_122f90;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00122f98);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00122f98)?;
let t = true;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_12310c;
} else {
break 'b_122fa0;
}
}
c.cycles += 5;
c.g[RA][0] = sx(0x00122fa8);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00122fa8)?;
c.g[V0][0] = c.g[V0][0] ^ 0x4;
let t = true;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_12310c;
} else {
break 'b_122fb4;
}
}
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00122fc0);
c.g[S1][0] = 0u64;
Callee { addr: 0x0011fee8, f: ee::s00::Rider_GetSubState }.call(c, 0x00122fc0)?;
c.g[V1][0] = 0x5;
let t = c.g[V0][0] != c.g[V1][0];
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_12310c;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.g[RA][0] = sx(0x00122fd8);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x230));
call_ptr(c, 0x00135eb0, 0x00122fd8)?;
let t = true;
c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_123100;
} else {
break 'b_122fe0;
}
}
}
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00122fec);
c.g[S1][0] = 0u64;
Callee { addr: 0x0011fee8, f: ee::s00::Rider_GetSubState }.call(c, 0x00122fec)?;
c.g[V1][0] = 0x5;
let t = c.g[V0][0] != c.g[V1][0];
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_12310c;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.g[RA][0] = sx(0x00123004);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x230));
call_ptr(c, 0x00135ef0, 0x00123004)?;
let t = true;
c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_123100;
} else {
break 'b_12300c;
}
}
}
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00123018);
c.g[S1][0] = 0u64;
Callee { addr: 0x0011fee8, f: ee::s00::Rider_GetSubState }.call(c, 0x00123018)?;
c.g[V1][0] = 0x5;
let t = c.g[V0][0] != c.g[V1][0];
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_12310c;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
c.g[RA][0] = sx(0x00123030);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x230));
call_ptr(c, 0x00135f30, 0x00123030)?;
let t = true;
c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_123100;
} else {
break 'b_123038;
}
}
}
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x330));
c.g[V0][0] = c.g[V0][0] ^ 0x1;
let t = true;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_12310c;
} else {
break 'b_123048;
}
}
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x330));
c.g[V0][0] = c.g[V0][0] ^ 0x2;
break 'b_123050;
}
c.cycles += 2;
let t = true;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_12310c;
} else {
break 'b_123058;
}
}
c.cycles += 6;
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x2fc));
c.mtc1(0, 0);
c.c_eq_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = 0x1;
if t {
break 'b_12310c;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = 0u64;
if t {
break 'b_12310c;
} else {
break 'b_123078;
}
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
let t = true;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x198));
if t {
break 'b_12310c;
} else {
break 'b_123084;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
let t = true;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x138));
if t {
break 'b_12310c;
} else {
break 'b_123090;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
let t = true;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x13c));
if t {
break 'b_12310c;
} else {
break 'b_12309c;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
let t = true;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x19c));
if t {
break 'b_12310c;
} else {
break 'b_1230a8;
}
}
c.cycles += 7;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x784));
c.g[A1][0] = 0x1;
c.g[RA][0] = sx(0x001230b8);
c.g[S1][0] = 0u64;
call_ptr(c, 0x00311ae8, 0x001230b8)?;
c.g[V1][0] = 0xd;
let t = c.g[V0][0] != c.g[V1][0];
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_12310c;
} else {
c.cycles += 8;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x784));
c.g[RA][0] = sx(0x001230d0);
c.g[A1][0] = 0x1;
call_ptr(c, 0x00311b20, 0x001230d0)?;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
c.g[RA][0] = sx(0x001230dc);
c.g[A1][0] = 0u64;
call_ptr(c, 0x001446a0, 0x001230dc)?;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.g[S1][0];
break 'b_12310c;
} else {
c.cycles += 7;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x784));
c.g[RA][0] = sx(0x001230f0);
c.g[A1][0] = 0x1;
call_ptr(c, 0x00311b20, 0x001230f0)?;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
c.g[RA][0] = sx(0x001230fc);
c.g[A1][0] = 0x1;
call_ptr(c, 0x001446a0, 0x001230fc)?;
c.g[S1][0] = (c.g[V0][0] < 0x1u64) as u64;
break 'b_123100;
}
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_12310c;
} else {
break 'b_123108;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0u64;
break 'b_12310c;
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return Ok(());
    });
}

/// `Rider_GetEventSwitchStartPos` (0x00123f38): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_get_event_switch_start_pos(c: &mut Cpu) {
    if c.pc != 0x00123f38 {
        return ee::s00::f_00123f38__generated(c);
    }
    guest(c, |c| {

'b_124198: {
'b_123f68: {
c.cycles += 10;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1b0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x190));
c.g[S0][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1a0));
c.sd((c.g[SP][0] as u32).wrapping_add(0x180), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e4));
let t = (c.g[V0][0] as i64) > 0;
c.g[S2][0] = c.g[A0][0];
if t {
break 'b_123f68;
} else {
c.cycles += 2;
let t = true;
c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x110));
if t {
break 'b_124198;
} else {
break 'b_123f68;
}
}
}
'b_1240a0: {
c.cycles += 9;
c.mtc1(0, 20);
c.g[RA][0] = sx(0x00123f74);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
call_ptr(c, 0x0027c958, 0x00123f74)?;
c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x110));
c.sq(3, (c.g[SP][0] as u32));
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5));
c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x450000;
if t {
break 'b_1240a0;
} else {
'b_124078: {
'b_12405c: {
'b_123fdc: {
'b_123fc0: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7e80));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00123fa4 => {
c.cycles += 7;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
c.g[A0][0] = 0x4d0000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00123fb8);
c.g[A1][0] = 0x1;
call_ptr(c, 0x0026b5e0, 0x00123fb8)?;
let t = true;
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
if t {
break 'b_124078;
} else {
break 'b_123fc0;
}
}
0x00123fc0 => {
break 'b_123fc0;
}
0x00123fdc => {
break 'b_123fdc;
}
0x0012405c => {
break 'b_12405c;
}
0x001240a0 => {
break 'b_1240a0;
}
_ => { c.jump_out(t, 0x00123f38, 0x001241bc); return Ok(()); }
}
}
c.cycles += 7;
c.g[A0][0] = 0x4d0000;
c.g[A1][0] = 0x2;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00123fd4);
c.g[A2][0] = 0u64;
call_ptr(c, 0x0026b5e0, 0x00123fd4)?;
let t = true;
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
if t {
break 'b_124078;
} else {
break 'b_123fdc;
}
}
'b_124044: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
c.g[V0][0] = ((c.g[V1][0] as i64) < 14i64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = 0x4d0000;
break 'b_124044;
} else {
'b_124040: {
'b_124028: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
break 'b_124028;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < 22i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x2;
if t {
break 'b_124040;
} else {
c.cycles += 6;
c.g[A0][0] = 0x4d0000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00124020);
c.g[A2][0] = 0u64;
call_ptr(c, 0x0026b5e0, 0x00124020)?;
let t = true;
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
if t {
break 'b_124078;
} else {
break 'b_124028;
}
}
}
}
c.cycles += 6;
c.g[A0][0] = 0x4d0000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00124038);
c.g[A1][0] = 0x2;
call_ptr(c, 0x0026b5e0, 0x00124038)?;
let t = true;
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
if t {
break 'b_124078;
} else {
break 'b_124040;
}
}
c.cycles += 1;
c.g[A0][0] = 0x4d0000;
break 'b_124044;
}
}
c.cycles += 6;
c.g[A1][0] = 0x2;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00124054);
c.g[A2][0] = 0x1;
call_ptr(c, 0x0026b5e0, 0x00124054)?;
let t = true;
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
if t {
break 'b_124078;
} else {
break 'b_12405c;
}
}
c.cycles += 7;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
c.g[A0][0] = 0x4d0000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
c.g[RA][0] = sx(0x00124070);
c.g[A1][0] = 0x2;
call_ptr(c, 0x0026b5e0, 0x00124070)?;
c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x78a8));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
break 'b_124078;
}
c.cycles += 10;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32));
break 'b_1240a0;
}
}
'b_124168: {
c.cycles += 49;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x78a4));
c.g[S1][0] = 0x500000;
c.mfc1(2, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[AT][0] = 0x43480000;
c.mtc1(1, 1);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x78a0));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.g[S0][0];
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A3][0] = 0x2;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mfc1(3, 1);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = sx(0x00124134);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
call_ptr(c, 0x0032e100, 0x00124134)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
c.g[RA][0] = sx(0x00124150);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
call_ptr(c, 0x00336850, 0x00124150)?;
c.mtc1(0, 1);
c.c_le_s(1, 0);
let t = !c.fcc();
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
if t {
break 'b_124168;
} else {
c.cycles += 1;
c.sq(2, (c.g[SP][0] as u32));
break 'b_124168;
}
}
c.cycles += 12;
c.mfc1(2, 20);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32));
c.lq(2, (c.g[SP][0] as u32));
break 'b_124198;
}
c.cycles += 9;
c.sq(2, (c.g[S2][0] as u32));
c.g[V0][0] = c.g[S2][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1b0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1a0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x190));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x180));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
c.pc = t; return Ok(());
    });
}

/// `RiderBoardPressSub_HandleAirborne` (0x0012fff8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_board_press_sub_handle_airborne(c: &mut Cpu) {
    if c.pc != 0x0012fff8 {
        return ee::s00::f_0012fff8__generated(c);
    }
    guest(c, |c| {

'b_130210: {
c.cycles += 11;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x00130014);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
Callee { addr: 0x0011fe98, f: ee::s00::Rider_GetMainState }.call(c, 0x00130014)?;
c.g[S1][0] = c.g[V0][0];
c.g[V0][0] = 0x1;
let t = c.g[S1][0] != c.g[V0][0];
c.g[V0][0] = 0u64;
if t {
break 'b_130210;
} else {
'b_130044: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x328));
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_130044;
} else {
c.cycles += 4;
c.sw((c.g[V1][0] as u32).wrapping_add(0x328), 0u64);
c.g[RA][0] = sx(0x00130040);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
Callee { addr: 0x00116930, f: ee::s00::Rider_EmptyStanceHook }.call(c, 0x00130040)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_130044;
}
}
'b_130198: {
c.cycles += 8;
c.g[A1][0] = 0x2;
c.g[RA][0] = sx(0x00130050);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x00312aa0, 0x00130050)?;
c.g[A0][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = 0x2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25));
if t {
break 'b_130198;
} else {
'b_130124: {
'b_13011c: {
'b_1300a8: {
c.cycles += 3;
c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_1300a8;
} else {
c.cycles += 14;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[A1][0] = 0x1;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76c8));
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x300));
c.sw((c.g[V1][0] as u32).wrapping_add(0x288), 0u64);
c.mul_s(0, 0, 1);
c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76c4));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x284));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
c.abs_s(0, 0);
c.c_le_s(0, 2);
let t = true;

if t {
break 'b_13011c;
} else {
break 'b_1300a8;
}
}
}
c.cycles += 4;
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d));
c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_130124;
} else {
'b_1300f8: {
'b_1300e8: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.mtc1(0, 1);
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
c.c_lt_s(0, 1);
let t = !c.fcc();
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
if t {
break 'b_1300e8;
} else {
c.cycles += 5;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76c0));
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 2);
let t = true;
c.mul_s(0, 0, 1);
if t {
break 'b_1300f8;
} else {
break 'b_1300e8;
}
}
}
c.cycles += 4;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76bc));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.mul_s(0, 0, 1);
break 'b_1300f8;
}
c.cycles += 9;
c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x288));
c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x284));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[A1][0] = 0x1;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76b8));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
c.abs_s(0, 0);
c.c_le_s(1, 0);
break 'b_13011c;
}
}
c.cycles += 2;
if !c.fcc() {
c.g[A1][0] = 0u64;
break 'b_130124;
} else {
break 'b_130124;
}
}
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_130210;
} else {
c.cycles += 27;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x330));
c.g[RA][0] = sx(0x0013013c);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
Callee { addr: 0x00119ad8, f: ee::s00::RiderMetrix_SetPressSpin }.call(c, 0x0013013c)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.mov_s(12, 0);
c.g[RA][0] = sx(0x0013014c);
c.g[A1][0] = 0x1;
Callee { addr: 0x0010e098, f: ee::s00::Rider_AddBoost }.call(c, 0x0013014c)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[RA][0] = sx(0x00130158);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
Callee { addr: 0x00119a38, f: ee::s00::RiderMetrix_FinishBoardPress }.call(c, 0x00130158)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.mov_s(12, 0);
c.g[RA][0] = sx(0x00130168);
c.g[A1][0] = 0x1;
Callee { addr: 0x0010e098, f: ee::s00::Rider_AddBoost }.call(c, 0x00130168)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[A1][0] = 0x11f;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x00130184);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
call_ptr(c, 0x003128e8, 0x00130184)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[RA][0] = sx(0x00130190);
c.g[A1][0] = 0x5;
Callee { addr: 0x0011fec8, f: ee::s00::Rider_SetSubState }.call(c, 0x00130190)?;
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_130210;
} else {
break 'b_130198;
}
}
}
}
'b_13020c: {
'b_1301c4: {
'b_1301bc: {
c.cycles += 3;
c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1301bc;
} else {
'b_1301b8: {
c.cycles += 3;
c.g[V0][0] = 0x1d;
let t = c.g[A0][0] == c.g[V0][0];
c.g[V0][0] = 0x1e;
if t {
break 'b_1301b8;
} else {
c.cycles += 2;
if c.g[A0][0] != c.g[V0][0] {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_1301c4;
} else {
break 'b_1301b8;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0x2;
break 'b_1301bc;
}
}
c.cycles += 2;
let t = true;
c.sw((c.g[S0][0] as u32), c.g[V0][0]);
if t {
break 'b_13020c;
} else {
break 'b_1301c4;
}
}
'b_130204: {
'b_1301ec: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x330));
let t = c.g[V0][0] != c.g[S1][0];
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
if t {
break 'b_1301ec;
} else {
c.cycles += 7;
c.g[A1][0] = 0x1b;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[RA][0] = sx(0x001301e4);
c.g[A2][0] = 0u64;
call_ptr(c, 0x003128e8, 0x001301e4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
if t {
break 'b_130204;
} else {
break 'b_1301ec;
}
}
}
c.cycles += 6;
c.g[A1][0] = 0x23;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[RA][0] = sx(0x00130200);
c.g[A2][0] = 0u64;
call_ptr(c, 0x003128e8, 0x00130200)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_130204;
}
c.cycles += 2;
c.g[RA][0] = sx(0x0013020c);
c.g[A1][0] = 0x5;
Callee { addr: 0x0011fec8, f: ee::s00::Rider_SetSubState }.call(c, 0x0013020c)?;
break 'b_13020c;
}
c.cycles += 1;
c.g[V0][0] = 0x1;
break 'b_130210;
}
}
c.cycles += 5;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
    });
}

/// `RiderHandplantState_Update` (0x001391a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_handplant_state_update(c: &mut Cpu) {
    if c.pc != 0x001391a8 {
        return ee::s01::f_001391a8__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x001391a8;
    loop {
        match pc {
            0x001391a8 => {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7204));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x88));
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x300));
                c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x84));
                c.mul_s(4, 1, 0);
                c.add_s(0, 2, 4);
                c.c_le_s(0, 3);
                let t = !c.fcc(); 
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x88));
                if t { pc = 0x00139300; continue; }
                pc = 0x001391f0; continue;
            }
            0x001391f0 => {
                c.cycles += 68;
                c.div_s(0, 0, 3);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 1);
                c.lqc2(3, (c.g[S2][0] as u32));
                c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x30));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 3);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sub_s(1, 0, 1);
                c.mul_s(0, 0, 1);
                c.mfc1(3, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[S2][0] as u32));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32));
                c.sq(2, (c.g[A2][0] as u32).wrapping_add(0x110));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x11c));
                c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x80));
                c.mul_s(12, 12, 4);
                c.g[RA][0] = sx(0x00139284);
                c.mul_s(12, 12, 3);
                call_ptr(c, 0x0031be50, 0x00139284)?;
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x70));
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x74));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x78));
                c.mul_s(1, 0, 1);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.mul_s(2, 0, 2);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                c.mul_s(0, 0, 4);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[SP][0] as u32));
                c.lqc2(4, (c.g[SP][0] as u32));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x120));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = true; 
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x120));
                if t { pc = 0x00139500; continue; }
                pc = 0x00139300; continue;
            }
            0x00139300 => {
                c.cycles += 13;
                c.lq(2, (c.g[S2][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x500000;
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
                c.sq(2, (c.g[A2][0] as u32).wrapping_add(0x110));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                c.lq(2, (c.g[A1][0] as u32));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_sub(0xee0));
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.c_eq_s(0, 1);
                let t = !c.fcc(); 
                c.g[A3][0] = 0u64;
                if t { pc = 0x00139374; continue; }
                pc = 0x00139334; continue;
            }
            0x00139334 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.c_eq_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00139374; continue; }
                pc = 0x00139348; continue;
            }
            0x00139348 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.c_eq_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00139374; continue; }
                pc = 0x0013935c; continue;
            }
            0x0013935c => {
                c.cycles += 6;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x1c));
                c.c_eq_s(0, 1);
                
                if c.fcc() { c.g[A3][0] = 0x1; pc = 0x00139374; continue; }
                pc = 0x00139374; continue;
            }
            0x00139374 => {
                c.cycles += 2;
                
                if c.g[A3][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0)); pc = 0x00139460; continue; }
                pc = 0x0013937c; continue;
            }
            0x0013937c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00139384);
                c.g[A0][0] = c.g[S2][0];
                Callee { addr: 0x00139548, f: ee::s01::RiderHandplantState_UpdateVelocity }.call(c, 0x00139384)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = 0x500000;
                if t { pc = 0x00139500; continue; }
                pc = 0x0013938c; continue;
            }
            0x0013938c => {
                c.cycles += 30;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
                c.mfc1(2, 0);
                c.lqc2(4, (c.g[V1][0] as u32));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x60));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[S1][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7200));
                c.g[A2][0] = c.g[S0][0];
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S3][0] = 0u64;
                c.lqc2(3, (c.g[S2][0] as u32).wrapping_add(0x30));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.sq(3, (c.g[S2][0] as u32).wrapping_add(0x50));
                c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x60));
                c.sq(3, (c.g[SP][0] as u32));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                c.g[RA][0] = sx(0x001393fc);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
                call_ptr(c, 0x001135b8, 0x001393fc)?;
                let t = true; 
                c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
                if t { pc = 0x00139424; continue; }
                pc = 0x00139404; continue;
            }
            0x00139404 => {
                c.cycles += 1;
                pc = 0x00139408; continue;
            }
            0x00139408 => {
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x71fc));
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
                c.g[RA][0] = sx(0x00139420);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00113648, 0x00139420)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
                pc = 0x00139424; continue;
            }
            0x00139424 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                if t { if back_edge_unwinds(c, 0x00139408) { return Ok(()); } pc = 0x00139408; continue; }
                pc = 0x0013942c; continue;
            }
            0x0013942c => {
                c.cycles += 5;
                c.g[A0][0] = 0x1;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                let t = c.g[V1][0] == c.g[A0][0]; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x0013944c; continue; }
                pc = 0x00139440; continue;
            }
            0x00139440 => {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let t = c.g[V1][0] != c.g[V0][0]; 
                
                if t { pc = 0x00139450; continue; }
                pc = 0x0013944c; continue;
            }
            0x0013944c => {
                c.cycles += 1;
                c.g[A1][0] = 0x1;
                pc = 0x00139450; continue;
            }
            0x00139450 => {
                c.cycles += 2;
                
                if c.g[A1][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0)); if back_edge_unwinds(c, 0x00139408) { return Ok(()); } pc = 0x00139408; continue; }
                pc = 0x00139458; continue;
            }
            0x00139458 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x00139500; continue; }
                pc = 0x00139460; continue;
            }
            0x00139460 => {
                c.cycles += 6;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = 0x1;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                let t = c.g[V1][0] == c.g[A0][0]; 
                c.g[A3][0] = c.g[V0][0];
                if t { pc = 0x00139484; continue; }
                pc = 0x00139478; continue;
            }
            0x00139478 => {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let t = c.g[V1][0] != c.g[V0][0]; 
                
                if t { pc = 0x00139488; continue; }
                pc = 0x00139484; continue;
            }
            0x00139484 => {
                c.cycles += 1;
                c.g[A1][0] = 0x1;
                pc = 0x00139488; continue;
            }
            0x00139488 => {
                c.cycles += 2;
                
                if c.g[A1][0] == 0u64 { c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x788)); pc = 0x001394f0; continue; }
                pc = 0x00139490; continue;
            }
            0x00139490 => {
                c.cycles += 15;
                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x788));
                c.mtc1(0, 0);
                c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.lqc2(5, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 1);
                c.c_lt_s(0, 1);
                let t = !c.fcc(); 
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t { pc = 0x00139500; continue; }
                pc = 0x001394cc; continue;
            }
            0x001394cc => {
                c.cycles += 9;
                c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x300));
                c.g[A0][0] = c.g[A3][0];
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x71f8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                c.add_s(12, 0, 0);
                c.g[RA][0] = sx(0x001394e8);
                c.mul_s(13, 0, 13);
                call_ptr(c, 0x00121aa0, 0x001394e8)?;
                let t = true; 
                
                if t { pc = 0x00139500; continue; }
                pc = 0x001394f0; continue;
            }
            0x001394f0 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x71f4));
                c.g[RA][0] = sx(0x00139500);
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x60));
                call_ptr(c, 0x00113648, 0x00139500)?;
                pc = 0x00139500; continue;
            }
            0x00139500 => {
                c.cycles += 9;
                c.g[RA][0] = sx(0x00139508);
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa0));
                call_ptr(c, 0x0011e098, 0x00139508)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001391a8, 0x00139524); return Ok(()); }
        }
    }
    });
}

/// `BENewRaceInt_GetTargetScore` (0x001453d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_new_race_int_get_target_score(c: &mut Cpu) {
    if c.pc != 0x001453d0 {
        return ee::s01::f_001453d0__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x001453d0;
    loop {
        match pc {
            0x001453d0 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[T1][0] = 0x1;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                let t = c.g[A3][0] != 0u64; 
                if c.g[A1][0] != 0 { c.g[T1][0] = c.g[A1][0]; }
                if t { pc = 0x00145484; continue; }
                pc = 0x001453f4; continue;
            }
            0x001453f4 => {
                c.cycles += 2;
                let t = c.g[T0][0] == 0u64; 
                c.g[V1][0] = 0x530000;
                if t { pc = 0x00145484; continue; }
                pc = 0x001453fc; continue;
            }
            0x001453fc => {
                c.cycles += 3;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6e18));
                let t = true; 
                c.g[V0][0] = 0x440000;
                if t { pc = 0x001454a4; continue; }
                pc = 0x00145408; continue;
            }
            0x00145408 => {
                c.cycles += 17;
                c.g[A0][0] = 0x64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                c.mult(0, 2, 2, 4);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(16, 1);
                c.g[RA][0] = sx(0x00145438);
                call_ptr(c, 0x00237cd8, 0x00145438)?;
                c.g[V1][0] = 0xc8;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6e14));
                c.divu(0, 2, 3);
                
                if c.g[V1][0] == 0u64 {  pc = 0x0014544c; continue; }
                pc = 0x0014544c; continue;
            }
            0x0014544c => {
                c.cycles += 14;
                c.g[A1][0] = 0x14;
                c.g[A0][0] = c.hi[0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x64));
                c.mult(0, 4, 16, 4);
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.div(0, 16, 5);
                c.g[V0][0] = c.hi[0];
                let t = true; 
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                if t { pc = 0x001454e4; continue; }
                pc = 0x00145484; continue;
            }
            0x00145484 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = c.g[A3][0] != c.g[V0][0]; 
                c.g[V1][0] = 0x530000;
                if t { pc = 0x001454a0; continue; }
                pc = 0x00145490; continue;
            }
            0x00145490 => {
                c.cycles += 2;
                let t = c.g[T0][0] == 0u64; 
                c.g[V0][0] = 0x440000;
                if t { pc = 0x001454a4; continue; }
                pc = 0x00145498; continue;
            }
            0x00145498 => {
                c.cycles += 2;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6e10));
                c.g[V1][0] = 0x530000;
                pc = 0x001454a0; continue;
            }
            0x001454a0 => {
                c.cycles += 1;
                c.g[V0][0] = 0x440000;
                pc = 0x001454a4; continue;
            }
            0x001454a4 => {
                c.cycles += 5;
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb38));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c08));
                c.g[A0][0] = 0u64;
                c.g[V1][0] = c.g[A3][0];
                pc = 0x001454b8; continue;
            }
            0x001454b8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[V1][0] as u32));
                let t = c.g[V0][0] != c.g[A1][0]; 
                c.g[A2][0] = sx((c.g[A0][0] as u32) << 4);
                if t { pc = 0x001454d0; continue; }
                pc = 0x001454c4; continue;
            }
            0x001454c4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
                
                if c.g[V0][0] == c.g[T1][0] { c.g[V1][0] = sx((c.g[T0][0] as u32) << 1); if back_edge_unwinds(c, 0x00145408) { return Ok(()); } pc = 0x00145408; continue; }
                pc = 0x001454d0; continue;
            }
            0x001454d0 => {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 30i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                if t { if back_edge_unwinds(c, 0x001454b8) { return Ok(()); } pc = 0x001454b8; continue; }
                pc = 0x001454e0; continue;
            }
            0x001454e0 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                pc = 0x001454e4; continue;
            }
            0x001454e4 => {
                c.cycles += 4;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001453d0, 0x001454f4); return Ok(()); }
        }
    }
    });
}

/// `BECharInt_GetSavedAttrib` (0x00148158): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_char_int_get_saved_attrib(c: &mut Cpu) {
    if c.pc != 0x00148158 {
        return ee::s01::f_00148158__generated(c);
    }
    guest(c, |c| {

'b_1482bc: {
'b_1482b8: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.g[A1][0];
c.g[S1][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x00148178);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0014a080, f: ee::s01::BE_GetRiderCharacter }.call(c, 0x00148178)?;
c.g[A3][0] = c.g[V0][0];
c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x460000;
if t {
break 'b_1482b8;
} else {
'b_148290: {
'b_148268: {
'b_148240: {
'b_148218: {
'b_1481f0: {
'b_1481c8: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5d70));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x001481a0 => {
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbdf));
if t {
break 'b_1482bc;
} else {
break 'b_1481c8;
}
}
0x001481c8 => {
break 'b_1481c8;
}
0x001481f0 => {
break 'b_1481f0;
}
0x00148218 => {
break 'b_148218;
}
0x00148240 => {
break 'b_148240;
}
0x00148268 => {
break 'b_148268;
}
0x00148290 => {
break 'b_148290;
}
_ => { c.jump_out(t, 0x00148158, 0x001482d0); return Ok(()); }
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe0));
if t {
break 'b_1482bc;
} else {
break 'b_1481f0;
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe1));
if t {
break 'b_1482bc;
} else {
break 'b_148218;
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe2));
if t {
break 'b_1482bc;
} else {
break 'b_148240;
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe3));
if t {
break 'b_1482bc;
} else {
break 'b_148268;
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe4));
if t {
break 'b_1482bc;
} else {
break 'b_148290;
}
}
c.cycles += 10;
c.g[V1][0] = 0xf88;
c.g[A0][0] = 0x9b50;
c.mult(0, 3, 7, 3);
c.mult(0, 4, 16, 4);
c.g[V0][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbe5));
if t {
break 'b_1482bc;
} else {
break 'b_1482b8;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0xffffffffffffffff;
break 'b_1482bc;
}
c.cycles += 5;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
    });
}

/// `Bolts_CountChildren` (0x0014d4c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn bolts_count_children(c: &mut Cpu) {
    if c.pc != 0x0014d4c8 {
        return ee::s01::f_0014d4c8__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x0014d4c8;
    loop {
        match pc {
            0x0014d4c8 => {
                c.cycles += 6;
                c.g[T0][0] = c.g[A0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                
                if c.g[V0][0] != 0u64 { c.g[V1][0] = sx((c.g[A2][0] as u32) << 1); pc = 0x0014d548; continue; }
                pc = 0x0014d4e0; continue;
            }
            0x0014d4e0 => {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = 0x38;
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                c.g[V0][0] = c.g[V1][0];
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                c.g[T1][0] = 0u64;
                c.mult(0, 4, 7, 4);
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                let t = true; 
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if t { pc = 0x0014d518; continue; }
                pc = 0x0014d50c; continue;
            }
            0x0014d50c => {
                c.cycles += 1;
                pc = 0x0014d510; continue;
            }
            0x0014d510 => {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                pc = 0x0014d514; continue;
            }
            0x0014d514 => {
                c.cycles += 1;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x0014d518; continue;
            }
            0x0014d518 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                
                if t { pc = 0x0014d540; continue; }
                pc = 0x0014d524; continue;
            }
            0x0014d524 => {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
                let t = c.g[V0][0] != c.g[A2][0]; 
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if t { pc = 0x0014d538; continue; }
                pc = 0x0014d530; continue;
            }
            0x0014d530 => {
                c.cycles += 2;
                let t = true; 
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x0014d510) { return Ok(()); } pc = 0x0014d510; continue; }
                pc = 0x0014d538; continue;
            }
            0x0014d538 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                if t { if back_edge_unwinds(c, 0x0014d514) { return Ok(()); } pc = 0x0014d514; continue; }
                pc = 0x0014d540; continue;
            }
            0x0014d540 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[T1][0];
                c.pc = t; return Ok(());
            }
            0x0014d548 => {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lh((c.g[V1][0] as u32));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0014d4c8, 0x0014d554); return Ok(()); }
        }
    }
    });
}



/// `BEScoreInt_UpdateRecords` (0x00155420): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_score_int_update_records(c: &mut Cpu) {
    if c.pc != 0x00155420 {
        return ee::s01::f_00155420__generated(c);
    }
    guest(c, |c| {

'b_1557b0: {
'b_1557ac: {
c.cycles += 19;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V0][0] = 0x530000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S1][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S2][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S4][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
let t = c.g[V0][0] != 0u64;
c.g[S5][0] = c.g[A3][0];
if t {
break 'b_1557ac;
} else {
c.cycles += 6;
c.g[RA][0] = sx(0x00155474);
Callee { addr: 0x0014a0e0, f: ee::s01::BE_GetRiderProfile }.call(c, 0x00155474)?;
c.g[S3][0] = c.g[V0][0];
c.g[V0][0] = (c.g[S3][0] < 0x3u64) as u64;
if c.g[V0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1557b0;
} else {
'b_1554f8: {
'b_1554f0: {
c.cycles += 24;
c.g[RA][0] = sx(0x0015548c);
c.g[A0][0] = c.g[S1][0];
Callee { addr: 0x0014a080, f: ee::s01::BE_GetRiderCharacter }.call(c, 0x0015548c)?;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[A3][0] = 0x64;
c.g[V1][0] = 0xf88;
c.g[A2][0] = 0x9b50;
c.mult(0, 5, 5, 7);
c.mult(0, 2, 2, 3);
c.g[A0][0] = 0x440000;
c.g[V1][0] = 0x4a0000;
c.mult(0, 6, 19, 6);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26b0));
c.g[S6][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
c.g[FP][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[S0][0] = c.lw((c.g[A1][0] as u32));
c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.g[S0][0] ^ 0x16;
c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0xde4));
let t = c.g[S2][0] == 0u64;
if c.g[V0][0] == 0 { c.g[S0][0] = 0u64; }
if t {
break 'b_1554f0;
} else {
c.cycles += 3;
c.g[V0][0] = 0xc;
let t = c.g[S6][0] != c.g[V0][0];
c.g[S3][0] = c.g[A3][0];
if t {
break 'b_1554f8;
} else {
break 'b_1554f0;
}
}
}
c.cycles += 2;
c.g[A3][0] = 0x8;
c.g[S3][0] = c.g[A3][0];
break 'b_1554f8;
}
'b_155790: {
c.cycles += 3;
c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
if t {
break 'b_155790;
} else {
c.cycles += 1;
'l_155508: loop {
'b_155784: {
'b_155780: {
c.cycles += 3;
c.g[V1][0] = (c.g[S3][0] < 0xau64) as u64;
let t = c.g[V1][0] == 0u64;
c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
if t {
break 'b_155780;
} else {
'b_155728: {
'b_1556f8: {
'b_1556b0: {
'b_155668: {
'b_155620: {
'b_1555ec: {
'b_1555b8: {
'b_155584: {
'b_155564: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5790));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00155530 => {
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x0015555c);
c.g[A1][0] = 0x3;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x0015555c)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_155564;
}
}
}
0x00155564 => {
break 'b_155564;
}
0x00155584 => {
break 'b_155584;
}
0x001555b8 => {
break 'b_1555b8;
}
0x001555ec => {
break 'b_1555ec;
}
0x00155620 => {
break 'b_155620;
}
0x00155668 => {
break 'b_155668;
}
0x001556b0 => {
break 'b_1556b0;
}
0x001556f8 => {
break 'b_1556f8;
}
0x00155728 => {
break 'b_155728;
}
_ => { c.jump_out(t, 0x00155420, 0x001557dc); return Ok(()); }
}
}
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 3;
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = true;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
if t {
break 'b_155780;
} else {
break 'b_155584;
}
}
}
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001555b0);
c.g[A1][0] = 0x5;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001555b0)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_1555b8;
}
}
}
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001555e4);
c.g[A1][0] = 0x4;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001555e4)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_1555ec;
}
}
}
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x00155618);
c.g[A1][0] = 0x7;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155618)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_155620;
}
}
}
c.cycles += 10;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 1);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x5c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.add_s(0, 0, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x00155660);
c.g[A1][0] = 0x1;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155660)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_155668;
}
}
}
c.cycles += 10;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 1);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x58));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.add_s(0, 0, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001556a8);
c.g[A1][0] = 0u64;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001556a8)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_1556b0;
}
}
}
c.cycles += 10;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 1);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.add_s(0, 0, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001556f0);
c.g[A1][0] = 0x2;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001556f0)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_1556f8;
}
}
}
c.cycles += 2;
if (c.g[S5][0] as i64) <= 0 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 2;
if c.g[S6][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
'b_15571c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
if t {
break 'b_15571c;
} else {
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
break 'b_15571c;
}
}
}
c.cycles += 3;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
let t = true;
c.sw((c.g[A1][0] as u32), c.g[S5][0]);
if t {
break 'b_155780;
} else {
break 'b_155728;
}
}
}
}
c.cycles += 2;
let t = (c.g[S4][0] as i64) <= 0;
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
if t {
break 'b_155780;
} else {
'b_15574c: {
c.cycles += 3;
c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
break 'b_15574c;
} else {
c.cycles += 3;
c.g[V0][0] = 0x6;
if c.g[FP][0] != c.g[V0][0] {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
break 'b_15574c;
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
'b_155770: {
c.cycles += 6;
c.g[V0][0] = 0x64;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.div(0, 20, 2);
c.sw((c.g[A1][0] as u32), c.g[S4][0]);
if c.g[V0][0] == 0u64 {
break 'b_155770;
} else {
break 'b_155770;
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lo[0];
c.g[RA][0] = sx(0x00155780);
c.g[A1][0] = 0x6;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155780)?;
break 'b_155780;
}
}
}
}
c.cycles += 1;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
if t {
if event_due(c) { deliver_event_here(c, 0x00155508); } continue 'l_155508;
} else {
break 'b_155790;
}
}
}
}
c.cycles += 2;
let t = c.g[S2][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1557b0;
} else {
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88));
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x50));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.sw((c.g[S7][0] as u32).wrapping_add(0x50), c.g[A1][0]);
break 'b_1557b0;
} else {
break 'b_1557ac;
}
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1557b0;
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
    });
}

/// `BEPostRace_GetInterface` (0x00156390): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_post_race_get_interface(c: &mut Cpu) {
    if c.pc != 0x00156390 {
        return ee::s01::f_00156390__generated(c);
    }
    guest(c, |c| {

'l_156390: loop {
'b_1564a4: {
'b_15649c: {
c.cycles += 4;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.g[V0][0] = (c.g[A1][0] < 0xeu64) as u64;
let t = c.g[V0][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_15649c;
} else {
'b_15648c: {
'b_15647c: {
'b_15646c: {
'b_15645c: {
'b_15644c: {
'b_15643c: {
'b_15642c: {
'b_15641c: {
'b_15640c: {
'b_1563fc: {
'b_1563ec: {
'b_1563dc: {
'b_1563cc: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5740));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00156390 => {
if event_due(c) { deliver_event_here(c, 0x00156390); } continue 'l_156390;
}
0x001563bc => {
c.cycles += 4;
c.g[RA][0] = sx(0x001563c4);
Callee { addr: 0x001448d8, f: ee::s01::BENewRaceInt_GetInstance }.call(c, 0x001563c4)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_1563cc;
}
}
0x001563cc => {
break 'b_1563cc;
}
0x001563dc => {
break 'b_1563dc;
}
0x001563ec => {
break 'b_1563ec;
}
0x001563fc => {
break 'b_1563fc;
}
0x0015640c => {
break 'b_15640c;
}
0x0015641c => {
break 'b_15641c;
}
0x0015642c => {
break 'b_15642c;
}
0x0015643c => {
break 'b_15643c;
}
0x0015644c => {
break 'b_15644c;
}
0x0015645c => {
break 'b_15645c;
}
0x0015646c => {
break 'b_15646c;
}
0x0015647c => {
break 'b_15647c;
}
0x0015648c => {
break 'b_15648c;
}
_ => { c.jump_out(t, 0x00156390, 0x001564ac); return Ok(()); }
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x001563d4);
Callee { addr: 0x00145970, f: ee::s01::BENewPlayerInt_GetInstance }.call(c, 0x001563d4)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_1563dc;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x001563e4);
Callee { addr: 0x0014edd8, f: ee::s01::BE_GetCharDBInterface }.call(c, 0x001563e4)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_1563ec;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x001563f4);
Callee { addr: 0x00147f78, f: ee::s01::BECharInt_GetInstance }.call(c, 0x001563f4)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_1563fc;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156404);
Callee { addr: 0x0014f250, f: ee::s01::BEOptionInt_GetInstance }.call(c, 0x00156404)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15640c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156414);
Callee { addr: 0x00152638, f: ee::s01::BESaveInt_GetInstance }.call(c, 0x00156414)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15641c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156424);
Callee { addr: 0x0014e0e0, f: ee::s01::BENetworkInt_GetInstance }.call(c, 0x00156424)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15642c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156434);
Callee { addr: 0x00154948, f: ee::s01::BEScoreInt_GetInstance }.call(c, 0x00156434)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15643c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156444);
Callee { addr: 0x0014abe0, f: ee::s01::BEBoltInt_GetInstance }.call(c, 0x00156444)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15644c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156454);
Callee { addr: 0x0014f900, f: ee::s01::BEBAGTInt_GetInstance }.call(c, 0x00156454)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15645c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156464);
Callee { addr: 0x001508d8, f: ee::s01::BEEconInt_GetInstance }.call(c, 0x00156464)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15646c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156474);
Callee { addr: 0x001532a0, f: ee::s01::BEMissionInt_GetInstance }.call(c, 0x00156474)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15647c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156484);
Callee { addr: 0x00155a50, f: ee::s01::BEAggressionInt_GetInstance }.call(c, 0x00156484)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15648c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00156494);
Callee { addr: 0x001569c0, f: ee::s01::BERewardInt_GetInstance }.call(c, 0x00156494)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_1564a4;
} else {
break 'b_15649c;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0u64;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
break 'b_1564a4;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return Ok(());
}
    });
}

/// `BERewardInt_PickRandomBolt` (0x00156c70): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_pick_random_bolt(c: &mut Cpu) {
    if c.pc != 0x00156c70 {
        return ee::s01::f_00156c70__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x00156c70;
    loop {
        match pc {
            0x00156c70 => {
                c.cycles += 30;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[A0][0] = 0xf88;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = 0x9b50;
                c.g[S3][0] = c.g[A2][0];
                c.mult(0, 5, 5, 2);
                c.mult(0, 4, 19, 4);
                c.g[V1][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x00156cc0);
                c.g[S0][0] = 0u64;
                Callee { addr: 0x0014bdb8, f: ee::s01::Bolts_GetDB }.call(c, 0x00156cc0)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00156cd0);
                c.g[A0][0] = c.g[S4][0];
                Callee { addr: 0x0014d988, f: ee::s01::Bolts_GetCharBoltCount }.call(c, 0x00156cd0)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[S5][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00156ce0);
                c.g[A1][0] = c.g[S3][0];
                Callee { addr: 0x0014d998, f: ee::s01::Bolts_GetCharBolts }.call(c, 0x00156ce0)?;
                let t = (c.g[S5][0] as i64) <= 0; 
                c.g[A0][0] = c.g[V0][0];
                if t { pc = 0x00156dac; continue; }
                pc = 0x00156ce8; continue;
            }
            0x00156ce8 => {
                c.cycles += 4;
                c.g[A3][0] = 0x1;
                c.g[A2][0] = 0x2;
                c.g[A1][0] = c.g[S5][0];
                pc = 0x00156cf8; continue;
            }
            0x00156cf8 => {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64; 
                
                if t { pc = 0x00156d34; continue; }
                pc = 0x00156d00; continue;
            }
            0x00156d00 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64; 
                
                if t { pc = 0x00156d34; continue; }
                pc = 0x00156d10; continue;
            }
            0x00156d10 => {
                c.cycles += 1;
                c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                pc = 0x00156d14; continue;
            }
            0x00156d14 => {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                let t = (c.g[V0][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t { pc = 0x00156d80; continue; }
                pc = 0x00156d2c; continue;
            }
            0x00156d2c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x00156d90; continue; }
                pc = 0x00156d34; continue;
            }
            0x00156d34 => {
                c.cycles += 2;
                let t = c.g[S2][0] != c.g[A3][0]; 
                
                if t { pc = 0x00156d4c; continue; }
                pc = 0x00156d3c; continue;
            }
            0x00156d3c => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x200;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4)); if back_edge_unwinds(c, 0x00156d14) { return Ok(()); } pc = 0x00156d14; continue; }
                pc = 0x00156d4c; continue;
            }
            0x00156d4c => {
                c.cycles += 2;
                
                if c.g[S2][0] != c.g[A2][0] { c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1)); pc = 0x00156da4; continue; }
                pc = 0x00156d54; continue;
            }
            0x00156d54 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x400;
                
                if c.g[V0][0] == 0u64 { c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1)); pc = 0x00156da4; continue; }
                pc = 0x00156d64; continue;
            }
            0x00156d64 => {
                c.cycles += 7;
                c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t { pc = 0x00156d8c; continue; }
                pc = 0x00156d80; continue;
            }
            0x00156d80 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                let t = true; 
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if t { pc = 0x00156d90; continue; }
                pc = 0x00156d8c; continue;
            }
            0x00156d8c => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x00156d90; continue;
            }
            0x00156d90 => {
                c.cycles += 5;
                c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                if c.g[V0][0] == 0 { c.g[S0][0] = c.g[V1][0]; }
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                pc = 0x00156da4; continue;
            }
            0x00156da4 => {
                c.cycles += 2;
                let t = c.g[A1][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                if t { if back_edge_unwinds(c, 0x00156cf8) { return Ok(()); } pc = 0x00156cf8; continue; }
                pc = 0x00156dac; continue;
            }
            0x00156dac => {
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0; 
                c.g[V0][0] = 0xffffffffffffffff;
                if t { pc = 0x00156ebc; continue; }
                pc = 0x00156db4; continue;
            }
            0x00156db4 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x00156dbc);
                call_ptr(c, 0x003177f0, 0x00156dbc)?;
                c.divu(0, 2, 16);
                
                if c.g[S0][0] == 0u64 {  pc = 0x00156dc8; continue; }
                pc = 0x00156dc8; continue;
            }
            0x00156dc8 => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[V1][0] = c.hi[0];
                c.g[RA][0] = sx(0x00156ddc);
                c.g[S0][0] = c.g[V1][0];
                Callee { addr: 0x0014d998, f: ee::s01::Bolts_GetCharBolts }.call(c, 0x00156ddc)?;
                c.g[A0][0] = c.g[V0][0];
                let t = (c.g[S5][0] as i64) <= 0; 
                c.g[A2][0] = 0u64;
                if t { pc = 0x00156eb8; continue; }
                pc = 0x00156de8; continue;
            }
            0x00156de8 => {
                c.cycles += 2;
                c.g[T0][0] = 0x1;
                c.g[A3][0] = 0x2;
                pc = 0x00156df0; continue;
            }
            0x00156df0 => {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64; 
                
                if t { pc = 0x00156e2c; continue; }
                pc = 0x00156df8; continue;
            }
            0x00156df8 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64; 
                
                if t { pc = 0x00156e2c; continue; }
                pc = 0x00156e08; continue;
            }
            0x00156e08 => {
                c.cycles += 1;
                c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                pc = 0x00156e0c; continue;
            }
            0x00156e0c => {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                let t = (c.g[V0][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t { pc = 0x00156e78; continue; }
                pc = 0x00156e24; continue;
            }
            0x00156e24 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x00156e88; continue; }
                pc = 0x00156e2c; continue;
            }
            0x00156e2c => {
                c.cycles += 2;
                let t = c.g[S2][0] != c.g[T0][0]; 
                
                if t { pc = 0x00156e44; continue; }
                pc = 0x00156e34; continue;
            }
            0x00156e34 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x200;
                
                if c.g[V0][0] != 0u64 { c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4)); if back_edge_unwinds(c, 0x00156e0c) { return Ok(()); } pc = 0x00156e0c; continue; }
                pc = 0x00156e44; continue;
            }
            0x00156e44 => {
                c.cycles += 2;
                
                if c.g[S2][0] != c.g[A3][0] { c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1)); pc = 0x00156eac; continue; }
                pc = 0x00156e4c; continue;
            }
            0x00156e4c => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x400;
                
                if c.g[V0][0] == 0u64 { c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1)); pc = 0x00156eac; continue; }
                pc = 0x00156e5c; continue;
            }
            0x00156e5c => {
                c.cycles += 7;
                c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t { pc = 0x00156e84; continue; }
                pc = 0x00156e78; continue;
            }
            0x00156e78 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                let t = true; 
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if t { pc = 0x00156e88; continue; }
                pc = 0x00156e84; continue;
            }
            0x00156e84 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x00156e88; continue;
            }
            0x00156e88 => {
                c.cycles += 4;
                c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                
                if c.g[V0][0] != 0u64 { c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1)); pc = 0x00156eac; continue; }
                pc = 0x00156e98; continue;
            }
            0x00156e98 => {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                
                if (c.g[S0][0] as i64) >= 0 { c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1)); pc = 0x00156eac; continue; }
                pc = 0x00156ea4; continue;
            }
            0x00156ea4 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = c.g[A1][0];
                if t { pc = 0x00156ebc; continue; }
                pc = 0x00156eac; continue;
            }
            0x00156eac => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                if t { if back_edge_unwinds(c, 0x00156df0) { return Ok(()); } pc = 0x00156df0; continue; }
                pc = 0x00156eb8; continue;
            }
            0x00156eb8 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                pc = 0x00156ebc; continue;
            }
            0x00156ebc => {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00156c70, 0x00156ee0); return Ok(()); }
        }
    }
    });
}

/// `BERewardInt_PickRandomLockedReward` (0x00157080): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_pick_random_locked_reward(c: &mut Cpu) {
    if c.pc != 0x00157080 {
        return ee::s01::f_00157080__generated(c);
    }
    guest(c, |c| {

'b_1571ec: {
c.cycles += 25;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S0][0] = c.g[A0][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S1][0] = c.g[A3][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S5][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S4][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x001570b8);
c.g[A1][0] = c.g[S1][0];
Callee { addr: 0x00157210, f: ee::s01::BERewardInt_GetRewardTypeCount }.call(c, 0x001570b8)?;
c.g[S3][0] = 0xffffffffffffffff;
c.g[S2][0] = c.g[V0][0];
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S5][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001570d4);
c.g[A3][0] = c.g[S1][0];
Callee { addr: 0x00157518, f: ee::s01::BERewardInt_CountUnlocked }.call(c, 0x001570d4)?;
c.g[S0][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0xffffffffffffffff;
if t {
break 'b_1571ec;
} else {
'b_157110: {
c.cycles += 11;
c.g[RA][0] = sx(0x001570ec);
call_ptr(c, 0x003177f0, 0x001570ec)?;
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
c.g[A0][0] = 0xf88;
c.divu(0, 2, 3);
c.g[A1][0] = 0x9b50;
c.g[V0][0] = 0x4a0000;
c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x3));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
if c.g[V1][0] == 0u64 {
break 'b_157110;
} else {
break 'b_157110;
}
}
'b_157170: {
c.cycles += 8;
c.g[T0][0] = 0u64;
c.g[V1][0] = (c.g[A3][0] < 0x5u64) as u64;
c.g[A2][0] = c.hi[0];
c.mult(0, 4, 20, 4);
c.mult(0, 5, 21, 5);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = c.g[V1][0] == 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
if t {
break 'b_157170;
} else {
'b_15716c: {
'b_157164: {
'b_15715c: {
'b_157154: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x56b0));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x0015714c => {
c.cycles += 2;
let t = true;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf30));
if t {
break 'b_157170;
} else {
break 'b_157154;
}
}
0x00157154 => {
break 'b_157154;
}
0x0015715c => {
break 'b_15715c;
}
0x00157164 => {
break 'b_157164;
}
0x0015716c => {
break 'b_15716c;
}
_ => { c.jump_out(t, 0x00157080, 0x00157210); return Ok(()); }
}
}
c.cycles += 2;
let t = true;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf36));
if t {
break 'b_157170;
} else {
break 'b_15715c;
}
}
c.cycles += 2;
let t = true;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf45));
if t {
break 'b_157170;
} else {
break 'b_157164;
}
}
c.cycles += 2;
let t = true;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf52));
if t {
break 'b_157170;
} else {
break 'b_15716c;
}
}
c.cycles += 1;
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf53));
break 'b_157170;
}
}
c.cycles += 2;
c.g[A1][0] = 0u64;
'l_157178: loop {
'b_1571e0: {
'b_1571a8: {
c.cycles += 8;
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = 0u64;
c.g[V0][0] = c.lb((c.g[V1][0] as u32));
c.g[A3][0] = c.g[V1][0];
c.g[V0][0] = c.g[V0][0] ^ 0x1;
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
break 'b_1571a8;
} else {
c.cycles += 2;
if c.g[A2][0] != 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
break 'b_1571a8;
} else {
c.cycles += 2;
let t = true;
c.g[S3][0] = sx((c.g[A1][0] as u32) << 3);
if t {
break 'b_1571e0;
} else {
break 'b_1571a8;
}
}
}
}
'l_1571a8: loop {
c.cycles += 1;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
'l_1571ac: loop {
c.cycles += 3;
c.g[V0][0] = ((c.g[A0][0] as i64) < 8i64) as u64;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_1571e0;
} else {
c.cycles += 6;
c.g[V0][0] = c.lb((c.g[A3][0] as u32));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
c.g[V0][0] = c.g[V0][0] ^ 0x1;
c.g[V0][0] = c.g[V0][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x001571ac); } continue 'l_1571ac;
} else {
c.cycles += 2;
if c.g[A2][0] != 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
if event_due(c) { deliver_event_here(c, 0x001571a8); } continue 'l_1571a8;
} else {
c.cycles += 2;
c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
break 'b_1571e0;
}
}
}
}
}
}
c.cycles += 2;
let t = (c.g[S3][0] as i64) < 0;
c.g[A1][0] = c.g[V1][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00157178); } continue 'l_157178;
} else {
c.cycles += 1;
c.g[V0][0] = c.g[S3][0];
break 'b_1571ec;
}
}
}
}
c.cycles += 9;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t; return Ok(());
    });
}

/// `BERewardInt_UnlockReward` (0x00157f20): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_unlock_reward(c: &mut Cpu) {
    if c.pc != 0x00157f20 {
        return ee::s01::f_00157f20__generated(c);
    }
    guest(c, |c| {

'b_157fc8: {
'b_157fc0: {
c.cycles += 8;
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3));
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.g[A3][0] = c.g[T0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[T2][0] = c.g[A0][0];
c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.g[T1][0];
if t {
break 'b_157fc0;
} else {
'b_157fac: {
'b_157f9c: {
'b_157f8c: {
'b_157f7c: {
'b_157f6c: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5630));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00157f5c => {
c.cycles += 4;
c.g[RA][0] = sx(0x00157f64);
c.g[A0][0] = c.g[T2][0];
Callee { addr: 0x00157fd0, f: ee::s01::BERewardInt_UnlockPoster }.call(c, 0x00157f64)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157f6c;
}
}
0x00157f6c => {
break 'b_157f6c;
}
0x00157f7c => {
break 'b_157f7c;
}
0x00157f8c => {
break 'b_157f8c;
}
0x00157f9c => {
break 'b_157f9c;
}
0x00157fac => {
break 'b_157fac;
}
_ => { c.jump_out(t, 0x00157f20, 0x00157fd0); return Ok(()); }
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00157f74);
c.g[A0][0] = c.g[T2][0];
Callee { addr: 0x001580f8, f: ee::s01::BERewardInt_UnlockTradingCard }.call(c, 0x00157f74)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157f7c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00157f84);
c.g[A0][0] = c.g[T2][0];
Callee { addr: 0x00158220, f: ee::s01::BERewardInt_UnlockConceptArt }.call(c, 0x00157f84)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157f8c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00157f94);
c.g[A0][0] = c.g[T2][0];
Callee { addr: 0x00158470, f: ee::s01::BERewardInt_UnlockVideo }.call(c, 0x00157f94)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157f9c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00157fa4);
c.g[A0][0] = c.g[T2][0];
Callee { addr: 0x00158348, f: ee::s01::BERewardInt_UnlockToy }.call(c, 0x00157fa4)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157fac;
}
}
c.cycles += 5;
c.g[A0][0] = c.g[T2][0];
c.g[RA][0] = sx(0x00157fb8);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa));
Callee { addr: 0x00158618, f: ee::s01::BERewardInt_UnlockRiderItem }.call(c, 0x00157fb8)?;
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_157fc8;
} else {
break 'b_157fc0;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0u64;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
break 'b_157fc8;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return Ok(());
    });
}

/// `BERewardInt_GetRewardTypeText` (0x0015af40): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_get_reward_type_text(c: &mut Cpu) {
    if c.pc != 0x0015af40 {
        return ee::s01::f_0015af40__generated(c);
    }
    guest(c, |c| {

'b_15afe0: {
c.cycles += 3;
c.g[V0][0] = (c.g[A1][0] < 0xau64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A3][0] = c.g[A0][0];
if t {
break 'b_15afe0;
} else {
'b_15af74: {
'b_15af70: {
'b_15afd4: {
'b_15afc8: {
'b_15afbc: {
'b_15afb0: {
'b_15afa4: {
'b_15af98: {
'b_15af8c: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5470));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x0015af68 => {
c.cycles += 2;
c.g[V0][0] = 0xc;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
break 'b_15af70;
}
0x0015af80 => {
c.cycles += 3;
c.g[V0][0] = 0xc;
let t = true;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
if t {
break 'b_15af70;
} else {
break 'b_15af8c;
}
}
0x0015af8c => {
break 'b_15af8c;
}
0x0015af98 => {
break 'b_15af98;
}
0x0015afa4 => {
break 'b_15afa4;
}
0x0015afb0 => {
break 'b_15afb0;
}
0x0015afbc => {
break 'b_15afbc;
}
0x0015afc8 => {
break 'b_15afc8;
}
0x0015afd4 => {
break 'b_15afd4;
}
0x0015afe0 => {
break 'b_15afe0;
}
_ => { c.jump_out(t, 0x0015af40, 0x0015afe8); return Ok(()); }
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
if t {
break 'b_15af74;
} else {
break 'b_15af98;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afa4;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afb0;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afbc;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x24));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afc8;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x20));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afd4;
}
}
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x28));
let t = true;
c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
if t {
break 'b_15af74;
} else {
break 'b_15afe0;
}
}
c.cycles += 1;
c.mult(0, 2, 6, 2);
break 'b_15af74;
}
c.cycles += 3;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = c.g[RA][0] as u32;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.pc = t; return Ok(());
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = 0u64;
c.pc = t; return Ok(());
    });
}

/// `ManualChaseCameraAlgorithm_ResetForRider` (0x001606d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn manual_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    if c.pc != 0x001606d8 {
        return ee::s01::f_001606d8__generated(c);
    }
    guest(c, |c| {

'b_160750: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S1][0] = c.g[A1][0];
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00160710);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x00160710)?;
if c.g[S1][0] != c.g[V0][0] {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_160750;
} else {
c.cycles += 14;
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x00160724);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00166c60, 0x00160724)?;
c.g[AT][0] = 0x42c80000;
c.mtc1(1, 12);
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00160738);
c.mov_s(13, 12);
call_ptr(c, 0x00166550, 0x00160738)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0016074c);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0016074c)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_160750;
}
}
c.cycles += 4;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
    });
}

/// `CameraTriggerMan_RemoveTriggerRegions` (0x0016ca18): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn camera_trigger_man_remove_trigger_regions(c: &mut Cpu) {
    if c.pc != 0x0016ca18 {
        return ee::s02::f_0016ca18__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x0016ca18;
    loop {
        match pc {
            0x0016ca18 => {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[S2][0] = 0u64;
                if t { pc = 0x0016cb9c; continue; }
                pc = 0x0016ca44; continue;
            }
            0x0016ca44 => {
                c.cycles += 3;
                c.g[S4][0] = 0xffffffffffffffff;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                pc = 0x0016ca50; continue;
            }
            0x0016ca50 => {
                c.cycles += 42;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0016ca88);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x0016ca88)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[RA][0] = sx(0x0016caa0);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                call_ptr(c, 0x00328f28, 0x0016caa0)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                if c.g[V0][0] != 0 { c.g[V1][0] = c.g[S0][0]; }
                c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A0][0] != 0 { c.g[V0][0] = c.g[V1][0]; }
                c.g[A1][0] = 0x2;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                if c.g[T0][0] != 0 { c.g[S0][0] = c.g[V0][0]; }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x0016cae8);
                c.g[T0][0] = c.g[S0][0];
                call_ptr(c, 0x003284b8, 0x0016cae8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x0016cb4c; continue; }
                pc = 0x0016caf8; continue;
            }
            0x0016caf8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x0016cb4c; continue; }
                pc = 0x0016cb04; continue;
            }
            0x0016cb04 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x0016cb4c; continue; }
                pc = 0x0016cb10; continue;
            }
            0x0016cb10 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x0016cb4c; continue; }
                pc = 0x0016cb1c; continue;
            }
            0x0016cb1c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] != 0u64; 
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                if t { pc = 0x0016cb4c; continue; }
                pc = 0x0016cb28; continue;
            }
            0x0016cb28 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x0016cb4c; continue; }
                pc = 0x0016cb34; continue;
            }
            0x0016cb34 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x0016cb74; continue; }
                pc = 0x0016cb40; continue;
            }
            0x0016cb40 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                
                if c.g[V0][0] == 0u64 { c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20)); pc = 0x0016cb54; continue; }
                pc = 0x0016cb4c; continue;
            }
            0x0016cb4c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x0016cb74; continue; }
                pc = 0x0016cb54; continue;
            }
            0x0016cb54 => {
                c.cycles += 1;
                c.g[A0][0] = 0u64;
                pc = 0x0016cb58; continue;
            }
            0x0016cb58 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x0016cb4c) { return Ok(()); } pc = 0x0016cb4c; continue; }
                pc = 0x0016cb64; continue;
            }
            0x0016cb64 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x0016cb58) { return Ok(()); } pc = 0x0016cb58; continue; }
                pc = 0x0016cb70; continue;
            }
            0x0016cb70 => {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                pc = 0x0016cb74; continue;
            }
            0x0016cb74 => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4)); pc = 0x0016cb8c; continue; }
                pc = 0x0016cb7c; continue;
            }
            0x0016cb7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0016cb84);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x00317e50, 0x0016cb84)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                pc = 0x0016cb8c; continue;
            }
            0x0016cb8c => {
                c.cycles += 4;
                c.g[S2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V1][0] = c.lw((c.g[S3][0] as u32)); if back_edge_unwinds(c, 0x0016ca50) { return Ok(()); } pc = 0x0016ca50; continue; }
                pc = 0x0016cb9c; continue;
            }
            0x0016cb9c => {
                c.cycles += 16;
                c.g[RA][0] = sx(0x0016cba4);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                Callee { addr: 0x0016d200, f: ee::s02::CameraTriggerStack_Clear }.call(c, 0x0016cba4)?;
                c.g[RA][0] = sx(0x0016cbac);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                Callee { addr: 0x0016d200, f: ee::s02::CameraTriggerStack_Clear }.call(c, 0x0016cbac)?;
                c.g[RA][0] = sx(0x0016cbb4);
                c.g[A0][0] = c.g[S3][0];
                Callee { addr: 0x0016bf40, f: ee::s02::CameraTriggerMan_FreeTriggers }.call(c, 0x0016cbb4)?;
                c.g[RA][0] = sx(0x0016cbbc);
                call_ptr(c, 0x0015dc18, 0x0016cbbc)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0016ca18, 0x0016cbdc); return Ok(()); }
        }
    }
    });
}

/// `CameraTrigger_ApplyAction` (0x0016e1d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn camera_trigger_apply_action(c: &mut Cpu) {
    if c.pc != 0x0016e1d8 {
        return ee::s02::f_0016e1d8__generated(c);
    }
    guest(c, |c| {

    let mut pc: u32 = 0x0016e1d8;
    loop {
        match pc {
            0x0016e1d8 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A2][0] = c.g[A0][0];
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x870));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A3][0] = c.lw((c.g[A2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = c.g[A3][0] == c.g[A1][0]; 
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                if t { pc = 0x0016e27c; continue; }
                pc = 0x0016e210; continue;
            }
            0x0016e210 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;
                
                if c.g[V0][0] == 0u64 { c.g[V0][0] = 0x2; pc = 0x0016e22c; continue; }
                pc = 0x0016e21c; continue;
            }
            0x0016e21c => {
                c.cycles += 2;
                let t = c.g[A3][0] == 0u64; 
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x0016e2c4; continue; }
                pc = 0x0016e224; continue;
            }
            0x0016e224 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x0016e554; continue; }
                pc = 0x0016e22c; continue;
            }
            0x0016e22c => {
                c.cycles += 2;
                let t = c.g[A3][0] != c.g[V0][0]; 
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x0016e554; continue; }
                pc = 0x0016e234; continue;
            }
            0x0016e234 => {
                c.cycles += 5;
                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.mtc1(0, 0);
                c.c_eq_s(2, 0);
                let t = c.fcc(); 
                
                if t { pc = 0x0016e260; continue; }
                pc = 0x0016e248; continue;
            }
            0x0016e248 => {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6448));
                c.div_s(0, 0, 2);
                let t = true; 
                c.mul_s(12, 0, 1);
                if t { pc = 0x0016e268; continue; }
                pc = 0x0016e260; continue;
            }
            0x0016e260 => {
                c.cycles += 2;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                pc = 0x0016e268; continue;
            }
            0x0016e268 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[T0][0];
                c.g[RA][0] = sx(0x0016e274);
                c.g[A1][0] = 0x5c;
                call_ptr(c, 0x00162060, 0x0016e274)?;
                let t = true; 
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x0016e554; continue; }
                pc = 0x0016e27c; continue;
            }
            0x0016e27c => {
                c.cycles += 5;
                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.mtc1(0, 0);
                c.c_eq_s(2, 0);
                let t = c.fcc(); 
                
                if t { pc = 0x0016e2a8; continue; }
                pc = 0x0016e290; continue;
            }
            0x0016e290 => {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6444));
                c.div_s(0, 0, 2);
                let t = true; 
                c.mul_s(12, 0, 1);
                if t { pc = 0x0016e2b0; continue; }
                pc = 0x0016e2a8; continue;
            }
            0x0016e2a8 => {
                c.cycles += 2;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                pc = 0x0016e2b0; continue;
            }
            0x0016e2b0 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[T0][0];
                c.g[RA][0] = sx(0x0016e2bc);
                c.g[A1][0] = 0x5b;
                call_ptr(c, 0x00162060, 0x0016e2bc)?;
                let t = true; 
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x0016e554; continue; }
                pc = 0x0016e2c4; continue;
            }
            0x0016e2c4 => {
                c.cycles += 6;
                c.mtc1(0, 12);
                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.c_eq_s(2, 12);
                let t = !c.fcc(); 
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                if t { pc = 0x0016e2ec; continue; }
                pc = 0x0016e2dc; continue;
            }
            0x0016e2dc => {
                c.cycles += 4;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                let t = true; 
                c.g[A1][0] = c.g[V0][0];
                if t { pc = 0x0016e304; continue; }
                pc = 0x0016e2ec; continue;
            }
            0x0016e2ec => {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6440));
                c.div_s(0, 0, 2);
                c.mul_s(12, 0, 1);
                c.g[A1][0] = c.g[V0][0];
                pc = 0x0016e304; continue;
            }
            0x0016e304 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x4cu64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x460000;
                if t { pc = 0x0016e540; continue; }
                pc = 0x0016e310; continue;
            }
            0x0016e310 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4080));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            0x0016e328 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x4;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e330; continue;
            }
            0x0016e330 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e338; continue;
            }
            0x0016e338 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e340; continue;
            }
            0x0016e340 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e348; continue;
            }
            0x0016e348 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x7;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e350; continue;
            }
            0x0016e350 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x5;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e358; continue;
            }
            0x0016e358 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x6;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e360; continue;
            }
            0x0016e360 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x8;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e368; continue;
            }
            0x0016e368 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x9;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e370; continue;
            }
            0x0016e370 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xa;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e378; continue;
            }
            0x0016e378 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xb;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e380; continue;
            }
            0x0016e380 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xc;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e388; continue;
            }
            0x0016e388 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xd;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e390; continue;
            }
            0x0016e390 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xe;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e398; continue;
            }
            0x0016e398 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0xf;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3a0; continue;
            }
            0x0016e3a0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x10;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3a8; continue;
            }
            0x0016e3a8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x11;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3b0; continue;
            }
            0x0016e3b0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x12;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3b8; continue;
            }
            0x0016e3b8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x13;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3c0; continue;
            }
            0x0016e3c0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x14;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3c8; continue;
            }
            0x0016e3c8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x15;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3d0; continue;
            }
            0x0016e3d0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x16;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3d8; continue;
            }
            0x0016e3d8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x17;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3e0; continue;
            }
            0x0016e3e0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x18;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3e8; continue;
            }
            0x0016e3e8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x19;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3f0; continue;
            }
            0x0016e3f0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1a;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e3f8; continue;
            }
            0x0016e3f8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1b;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e400; continue;
            }
            0x0016e400 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1c;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e408; continue;
            }
            0x0016e408 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1d;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e410; continue;
            }
            0x0016e410 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1e;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e418; continue;
            }
            0x0016e418 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x1f;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e420; continue;
            }
            0x0016e420 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x20;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e428; continue;
            }
            0x0016e428 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x29;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e430; continue;
            }
            0x0016e430 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e438; continue;
            }
            0x0016e438 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2a;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e440; continue;
            }
            0x0016e440 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2b;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e448; continue;
            }
            0x0016e448 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2c;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e450; continue;
            }
            0x0016e450 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2d;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e458; continue;
            }
            0x0016e458 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2e;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e460; continue;
            }
            0x0016e460 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x2f;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e468; continue;
            }
            0x0016e468 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x30;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e470; continue;
            }
            0x0016e470 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x31;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e478; continue;
            }
            0x0016e478 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x32;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e480; continue;
            }
            0x0016e480 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x33;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e488; continue;
            }
            0x0016e488 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x34;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e490; continue;
            }
            0x0016e490 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x35;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e498; continue;
            }
            0x0016e498 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x36;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4a0; continue;
            }
            0x0016e4a0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x37;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4a8; continue;
            }
            0x0016e4a8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x38;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4b0; continue;
            }
            0x0016e4b0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x39;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4b8; continue;
            }
            0x0016e4b8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3a;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4c0; continue;
            }
            0x0016e4c0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3b;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4c8; continue;
            }
            0x0016e4c8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3c;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4d0; continue;
            }
            0x0016e4d0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3d;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4d8; continue;
            }
            0x0016e4d8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3e;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4e0; continue;
            }
            0x0016e4e0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x3f;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4e8; continue;
            }
            0x0016e4e8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x40;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4f0; continue;
            }
            0x0016e4f0 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x41;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e4f8; continue;
            }
            0x0016e4f8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x42;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e500; continue;
            }
            0x0016e500 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x43;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e508; continue;
            }
            0x0016e508 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x44;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e510; continue;
            }
            0x0016e510 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x45;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e518; continue;
            }
            0x0016e518 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x46;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e520; continue;
            }
            0x0016e520 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x47;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e528; continue;
            }
            0x0016e528 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x48;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e530; continue;
            }
            0x0016e530 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x49;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e538; continue;
            }
            0x0016e538 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x4c;
                if t { pc = 0x0016e544; continue; }
                pc = 0x0016e540; continue;
            }
            0x0016e540 => {
                c.cycles += 1;
                c.g[A1][0] = 0u64;
                pc = 0x0016e544; continue;
            }
            0x0016e544 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[T0][0];
                c.g[RA][0] = sx(0x0016e550);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x00162060, 0x0016e550)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                pc = 0x0016e554; continue;
            }
            0x0016e554 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0016e1d8, 0x0016e55c); return Ok(()); }
        }
    }
    });
}



/// `DebugMenu_DispatchItemOp` (0x0017a158): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn debug_menu_dispatch_item_op(c: &mut Cpu) {
    if c.pc != 0x0017a158 {
        return ee::s02::f_0017a158__generated(c);
    }
    guest(c, |c| {

'b_17a214: {
'b_17a210: {
'b_17a20c: {
c.cycles += 4;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = (c.g[A1][0] as i64) < 0;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_17a20c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x8c;
if t {
break 'b_17a20c;
} else {
c.cycles += 5;
c.g[V1][0] = (c.g[A2][0] < 0x7u64) as u64;
c.mult(0, 7, 5, 2);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[V1][0] == 0u64;
c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
if t {
break 'b_17a20c;
} else {
'b_17a1e0: {
'b_17a1d0: {
'b_17a1cc: {
'b_17a1c8: {
'b_17a1c0: {
'b_17a1b8: {
'b_17a1b0: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3200));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x0017a1a8 => {
c.cycles += 2;
let t = true;
c.g[V0][0] = (c.g[S0][0] < 0x5u64) as u64;
if t {
break 'b_17a1d0;
} else {
break 'b_17a1b0;
}
}
0x0017a1b0 => {
break 'b_17a1b0;
}
0x0017a1b8 => {
break 'b_17a1b8;
}
0x0017a1c0 => {
break 'b_17a1c0;
}
0x0017a1c8 => {
break 'b_17a1c8;
}
0x0017a1e0 => {
break 'b_17a1e0;
}
_ => { c.jump_out(t, 0x0017a158, 0x0017a220); return Ok(()); }
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5));
if t {
break 'b_17a1cc;
} else {
break 'b_17a1b8;
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8));
if t {
break 'b_17a1cc;
} else {
break 'b_17a1c0;
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb));
if t {
break 'b_17a1cc;
} else {
break 'b_17a1c8;
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
break 'b_17a1cc;
}
c.cycles += 1;
c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
break 'b_17a1d0;
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_17a20c;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_17a214;
} else {
break 'b_17a1e0;
}
}
}
c.cycles += 11;
c.g[RA][0] = sx(0x0017a1e8);
call_ptr(c, 0x0014dc80, 0x0017a1e8)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0017a1f4);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x0017a1f4)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0017a200);
c.g[A1][0] = c.g[S0][0];
call_ptr(c, 0x00144cc0, 0x0017a200)?;
c.g[V1][0] = c.g[V0][0];
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_17a210;
} else {
break 'b_17a20c;
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = 0u64;
break 'b_17a210;
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_17a214;
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return Ok(());
    });
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn rider_dispatch_query_table_matches() {
        check(
            ee::s00::f_00122ee8__generated,
            rider_dispatch_query_table,
            200,
            &|c, seed| {
                machine(c, seed, 0x2ee8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00122ee8;
            },
        );
    }

    #[test]
    fn rider_get_event_switch_start_pos_matches() {
        check(
            ee::s00::f_00123f38__generated,
            rider_get_event_switch_start_pos,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f38, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00123f38;
            },
        );
    }

    #[test]
    fn rider_board_press_sub_handle_airborne_matches() {
        check(
            ee::s00::f_0012fff8__generated,
            rider_board_press_sub_handle_airborne,
            200,
            &|c, seed| {
                machine(c, seed, 0xfff8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0012fff8;
            },
        );
    }

    #[test]
    fn rider_handplant_state_update_matches() {
        check(
            ee::s01::f_001391a8__generated,
            rider_handplant_state_update,
            200,
            &|c, seed| {
                machine(c, seed, 0x91a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x001391a8;
            },
        );
    }

    #[test]
    fn be_new_race_int_get_target_score_matches() {
        check(
            ee::s01::f_001453d0__generated,
            be_new_race_int_get_target_score,
            200,
            &|c, seed| {
                machine(c, seed, 0x53d0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x001453d0;
            },
        );
    }

    #[test]
    fn be_char_int_get_saved_attrib_matches() {
        check(
            ee::s01::f_00148158__generated,
            be_char_int_get_saved_attrib,
            200,
            &|c, seed| {
                machine(c, seed, 0x8158, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00148158;
            },
        );
    }

    #[test]
    fn bolts_count_children_matches() {
        check(
            ee::s01::f_0014d4c8__generated,
            bolts_count_children,
            200,
            &|c, seed| {
                machine(c, seed, 0xd4c8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0014d4c8;
            },
        );
    }



    #[test]
    fn be_score_int_update_records_matches() {
        check(
            ee::s01::f_00155420__generated,
            be_score_int_update_records,
            200,
            &|c, seed| {
                machine(c, seed, 0x5420, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00155420;
            },
        );
    }

    #[test]
    fn be_post_race_get_interface_matches() {
        check(
            ee::s01::f_00156390__generated,
            be_post_race_get_interface,
            200,
            &|c, seed| {
                machine(c, seed, 0x6390, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00156390;
            },
        );
    }

    #[test]
    fn be_reward_int_pick_random_bolt_matches() {
        check(
            ee::s01::f_00156c70__generated,
            be_reward_int_pick_random_bolt,
            200,
            &|c, seed| {
                machine(c, seed, 0x6c70, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00156c70;
            },
        );
    }

    #[test]
    fn be_reward_int_pick_random_locked_reward_matches() {
        check(
            ee::s01::f_00157080__generated,
            be_reward_int_pick_random_locked_reward,
            200,
            &|c, seed| {
                machine(c, seed, 0x7080, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00157080;
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_reward_matches() {
        check(
            ee::s01::f_00157f20__generated,
            be_reward_int_unlock_reward,
            200,
            &|c, seed| {
                machine(c, seed, 0x7f20, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00157f20;
            },
        );
    }

    #[test]
    fn be_reward_int_get_reward_type_text_matches() {
        check(
            ee::s01::f_0015af40__generated,
            be_reward_int_get_reward_type_text,
            200,
            &|c, seed| {
                machine(c, seed, 0xaf40, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0015af40;
            },
        );
    }

    #[test]
    fn manual_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s01::f_001606d8__generated,
            manual_chase_camera_algorithm_reset_for_rider,
            200,
            &|c, seed| {
                machine(c, seed, 0x6d8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x001606d8;
            },
        );
    }

    #[test]
    fn camera_trigger_man_remove_trigger_regions_matches() {
        check(
            ee::s02::f_0016ca18__generated,
            camera_trigger_man_remove_trigger_regions,
            200,
            &|c, seed| {
                machine(c, seed, 0xca18, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0016ca18;
            },
        );
    }

    #[test]
    fn camera_trigger_apply_action_matches() {
        check(
            ee::s02::f_0016e1d8__generated,
            camera_trigger_apply_action,
            200,
            &|c, seed| {
                machine(c, seed, 0xe1d8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0016e1d8;
            },
        );
    }



    #[test]
    fn debug_menu_dispatch_item_op_matches() {
        check(
            ee::s02::f_0017a158__generated,
            debug_menu_dispatch_item_op,
            200,
            &|c, seed| {
                machine(c, seed, 0xa158, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0017a158;
            },
        );
    }


}
