//! Staging module oc675 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc675`.
//!
//! Note for the reorg: `game/be/belibrary.rs` still holds dead, untested
//! ports named `bolts_collect_by_group` and `bolt_int_collect_equipped_texture_names`
//! (same names as here); delete them before merging this module.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0013_0228,
        rider_board_press_sub_update_start,
        "RiderBoardPressSub_UpdateStart",
    ),
    (
        0x0013_03e0,
        rider_board_press_sub_update_press,
        "RiderBoardPressSub_UpdatePress",
    ),
    (
        0x0013_0dd0,
        rider_board_press_sub_update_balance,
        "RiderBoardPressSub_UpdateBalance",
    ),
    (0x0014_46f8, mission_rider_dtor, "MissionRider_dtor"),
    (
        0x0014_6f88,
        be_new_player_int_get_display_name,
        "BENewPlayerInt_GetDisplayName",
    ),
    (
        0x0014_a188,
        be_library_serialize_save,
        "BELibrary_SerializeSave",
    ),
    (
        0x0014_b7a8,
        bolt_int_collect_equipped_texture_names,
        "BEBoltInt_CollectEquippedTextureNames",
    ),
    (0x0014_d7e8, bolts_collect_by_group, "Bolts_CollectByGroup"),
    (
        0x0014_e2c8,
        be_network_int_pack_player_data,
        "BENetworkInt_PackPlayerData",
    ),
    (
        0x0014_e5c8,
        be_network_int_unpack_player_data,
        "BENetworkInt_UnpackPlayerData",
    ),
];

/// `void RiderBoardPressSub_UpdateStart(RiderBoardPressSub* self, tBoardPressCmd* in)`
/// (0x130228): the board press' "start" state (reference
/// `cBoardPressControl_handleIntoState`). With the press stick at rest and the
/// pivot stick under 0.5 it targets slew 10 back to 0, goes to state 3 and
/// plays the release animation (33 in PressMode 2, else 25). Otherwise it
/// drives slew 11 to 0.5 and slew 10 to 1; once slew 10 has reached 1 it
/// reports the press start to the metrix, adds a boost, goes to state 1 and
/// plays the press animation (36 / 28) with slew 12 set accordingly.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn rider_board_press_sub_update_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_1303d0: {
            'b_1302f0: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76b4));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.mtc1(0, 3);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 26);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 20);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                c.mtc1(3, 1);
                c.cvt_s_w(1, 1);
                c.mul_s(0, 0, 2);
                c.c_eq_s(0, 3);
                let t = !c.fcc();
                c.mul_s(2, 1, 2);
                if t {
                    break 'b_1302f0;
                } else {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.abs_s(0, 2);
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();
                    c.g[V1][0] = 0x3;
                    if t {
                        break 'b_1302f0;
                    } else {
                        'b_1302d8: {
                            'b_1302d4: {
                                c.cycles += 13;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76b0));
                                c.g[A0][0] = 0x2;
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
                                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x270));
                                c.add_s(0, 0, 0);
                                c.mul_s(0, 0, 1);
                                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x26c));
                                c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x330));
                                let t = c.g[V0][0] != c.g[A0][0];
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
                                if t {
                                    break 'b_1302d4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A1][0] = 0x21;
                                    if t {
                                        break 'b_1302d8;
                                    } else {
                                        break 'b_1302d4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x19;
                            break 'b_1302d8;
                        }
                        c.cycles += 6;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 12);
                        c.g[RA][0] = sx(0x001302e8);
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x003128e8, 0x001302e8)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1303d0;
                        } else {
                            break 'b_1302f0;
                        }
                    }
                }
            }
            c.cycles += 22;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76ac));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x300));
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 1);
            c.mul_s(0, 0, 2);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x27c));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x278));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
            c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x270));
            c.add_s(0, 0, 0);
            c.mul_s(0, 0, 2);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x26c));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x268));
            c.c_eq_s(0, 20);
            if !c.fcc() {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1303d0;
            } else {
                'b_1303cc: {
                    'b_1303a8: {
                        c.cycles += 14;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x330));
                        c.g[RA][0] = sx(0x00130354);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
                        call_ptr(c, 0x001199f8, 0x00130354)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.mov_s(12, 0);
                        c.g[RA][0] = sx(0x00130364);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0010e098, 0x00130364)?;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                        c.g[V1][0] = 0x2;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
                        if t {
                            break 'b_1303a8;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = 0x24;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[RA][0] = sx(0x00130394);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x003128e8, 0x00130394)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x280));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x284), 0u64);
                            let t = true;
                            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x288));
                            if t {
                                break 'b_1303cc;
                            } else {
                                break 'b_1303a8;
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[A1][0] = 0x1c;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x001303bc);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x003128e8, 0x001303bc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x284), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x288), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x280), 0u64);
                    break 'b_1303cc;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1303d0;
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void RiderBoardPressSub_UpdatePress(RiderBoardPressSub* self, tBoardPressCmd* in)`
/// (0x1303e0): the board press' held state. Runs the switch, jump, spin and
/// balance checks in turn and stops at the first that takes over; otherwise,
/// when the release check fires, it clears the press slew target and plays
/// the release animation (33 in PressMode 2, else 25) and enters state 3.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn rider_board_press_sub_update_press(c: &mut Cpu) {
    guest(c, |c| {
        'b_1304bc: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001303fc);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x001306b0, 0x001303fc)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1304bc;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00130410);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001307b8, 0x00130410)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_1304bc;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00130424);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001308d8, 0x00130424)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_1304bc;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00130438);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00130dd0, 0x00130438)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1304bc;
                        } else {
                            'b_1304b8: {
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0013044c);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x00131200, 0x0013044c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = 0x2;
                                if t {
                                    break 'b_1304b8;
                                } else {
                                    'b_1304b4: {
                                        'b_13049c: {
                                            c.cycles += 11;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x76a8));
                                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x270), 0u64);
                                            c.add_s(0, 0, 0);
                                            c.mul_s(0, 0, 1);
                                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x26c));
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
                                            if t {
                                                break 'b_13049c;
                                            } else {
                                                c.cycles += 7;
                                                c.g[A1][0] = 0x21;
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[RA][0] = sx(0x00130494);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x003128e8, 0x00130494)?;
                                                let t = true;
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1304b4;
                                                } else {
                                                    break 'b_13049c;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x19;
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 12);
                                        c.g[RA][0] = sx(0x001304b0);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, 0x003128e8, 0x001304b0)?;
                                        c.g[V0][0] = 0x3;
                                        break 'b_1304b4;
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                    break 'b_1304b8;
                                }
                            }
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1304bc;
                        }
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int RiderBoardPressSub_UpdateBalance(RiderBoardPressSub* self, tBoardPressCmd* in)`
/// (0x130dd0): the balance part of a board press. Advances the balance and
/// off-balance clocks, turns the press/pivot sticks into a desired press
/// (clamped by the balance time), picks the slew-11 rate from the two balance
/// curves, and past one second off balance starts a wipeout (anim 358, or 359
/// in PressMode 2) and returns 1; otherwise 0.
///
/// Converted from the generated body (labelled blocks, every register, cycle
/// and call kept).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn rider_board_press_sub_update_balance(c: &mut Cpu) {
    guest(c, |c| {
        'b_130e30: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x762c));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
            c.mul_s(22, 0, 3);
            c.add_s(1, 1, 22);
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x274));
            c.c_eq_s(0, 2);
            if !c.fcc() {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_130e30;
            } else {
                c.cycles += 3;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.add_s(0, 0, 22);
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                break 'b_130e30;
            }
        }
        'b_130e68: {
            c.cycles += 13;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x2;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = c.g[V0][0];
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7628));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x330));
            let t = c.g[V0][0] != c.g[A0][0];
            c.mul_s(20, 1, 0);
            if t {
                break 'b_130e68;
            } else {
                c.cycles += 1;
                c.neg_s(20, 20);
                break 'b_130e68;
            }
        }
        'b_130ef8: {
            'b_130ea4: {
                c.cycles += 9;
                c.mtc1(0, 0);
                c.max_s(20, 20, 0);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.c_le_s(0, 3);
                let t = !c.fcc();
                c.mov_s(6, 0);
                if t {
                    break 'b_130ea4;
                } else {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 6, 1);
                    c.add_s(0, 0, 1);
                    let t = true;
                    c.min_s(20, 20, 0);
                    if t {
                        break 'b_130ef8;
                    } else {
                        break 'b_130ea4;
                    }
                }
            }
            'b_130ed4: {
                c.cycles += 12;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 20);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7624));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                c.mov_s(2, 20);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
                c.abs_s(0, 0);
                c.c_le_s(20, 0);
                if c.fcc() {
                    c.mov_s(2, 0);
                    break 'b_130ed4;
                } else {
                    break 'b_130ed4;
                }
            }
            'b_130ef0: {
                c.cycles += 7;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.mtc1(0, 20);
                c.c_lt_s(0, 2);
                if c.fcc() {
                    c.mov_s(20, 3);
                    break 'b_130ef0;
                } else {
                    break 'b_130ef0;
                }
            }
            c.cycles += 2;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 1);
            break 'b_130ef8;
        }
        'b_1310cc: {
            'b_130f4c: {
                'b_130f48: {
                    'b_130f38: {
                        'b_130f14: {
                            c.cycles += 3;
                            c.c_le_s(20, 1);
                            let t = !c.fcc();

                            if t {
                                break 'b_130f14;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x274));
                                c.c_lt_s(1, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_130f38;
                                } else {
                                    break 'b_130f14;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.c_le_s(1, 20);
                        let t = !c.fcc();
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fdc));
                        if t {
                            break 'b_130f48;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x274));
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x274));
                                break 'b_130f4c;
                            } else {
                                break 'b_130f38;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    let t = true;
                    c.mul_s(1, 21, 22);
                    if t {
                        break 'b_1310cc;
                    } else {
                        break 'b_130f48;
                    }
                }
                c.cycles += 1;
                c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x274));
                break 'b_130f4c;
            }
            'b_130fe0: {
                'b_130fd0: {
                    'b_130fcc: {
                        'b_130fac: {
                            c.cycles += 5;
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.c_lt_s(4, 3);
                            if !c.fcc() {
                                c.lwc1(1, (c.g[V0][0] as u32));
                                break 'b_130fac;
                            } else {
                                'b_130f9c: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                        break 'b_130f9c;
                                    } else {
                                        'b_130f90: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 3);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                break 'b_130f90;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_130fe0;
                                                } else {
                                                    break 'b_130f90;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.sub_s(3, 3, 1);
                                        let t = true;
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_130fcc;
                                        } else {
                                            break 'b_130f9c;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.sub_s(3, 3, 4);
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_130fd0;
                                } else {
                                    break 'b_130fac;
                                }
                            }
                        }
                        'b_130fc4: {
                            c.cycles += 4;
                            c.c_lt_s(3, 1);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                                break 'b_130fc4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_130fe0;
                                } else {
                                    break 'b_130fc4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sub_s(3, 3, 1);
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                        break 'b_130fcc;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_130fd0;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(5, 2, 0);
                break 'b_130fe0;
            }
            'b_13107c: {
                'b_13106c: {
                    'b_131068: {
                        'b_131048: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fd4));
                            c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x274));
                            c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.c_lt_s(4, 3);
                            if !c.fcc() {
                                c.lwc1(1, (c.g[V1][0] as u32));
                                break 'b_131048;
                            } else {
                                'b_131038: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc));
                                        break 'b_131038;
                                    } else {
                                        'b_13102c: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 3);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x14));
                                                break 'b_13102c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13107c;
                                                } else {
                                                    break 'b_13102c;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.sub_s(3, 3, 1);
                                        let t = true;
                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_131068;
                                        } else {
                                            break 'b_131038;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.sub_s(3, 3, 4);
                                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_13106c;
                                } else {
                                    break 'b_131048;
                                }
                            }
                        }
                        'b_131060: {
                            c.cycles += 4;
                            c.c_lt_s(3, 1);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                                break 'b_131060;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_13107c;
                                } else {
                                    break 'b_131060;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sub_s(3, 3, 1);
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                        break 'b_131068;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_13106c;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(4, 2, 0);
                break 'b_13107c;
            }
            'b_1310c8: {
                c.cycles += 14;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7620));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.mul_s(0, 6, 0);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x761c));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x274));
                c.min_s(0, 0, 1);
                c.sub_s(1, 1, 0);
                c.mul_s(0, 4, 0);
                c.c_lt_s(3, 2);
                c.mul_s(1, 5, 1);
                let t = !c.fcc();
                c.add_s(21, 1, 0);
                if t {
                    break 'b_1310c8;
                } else {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001310bc);
                    call_ptr(c, 0x0028b180, 0x001310bc)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x001310c8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002a1560, 0x001310c8)?;
                    break 'b_1310c8;
                }
            }
            c.cycles += 1;
            c.mul_s(1, 21, 22);
            break 'b_1310cc;
        }
        'b_1311e4: {
            'b_1311e0: {
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x27c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x278));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.c_lt_s(2, 0);
                let t = !c.fcc();
                c.g[A0][0] = 0x500000;
                if t {
                    break 'b_1311e0;
                } else {
                    'b_13119c: {
                        'b_131194: {
                            c.cycles += 28;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.vu0().sqrt(4, 0);
                            c.vu0().waitq();
                            c.cfc2(3, 22);
                            c.mtc1(3, 2);
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 1);
                            c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x110));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xea0));
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7618));
                            c.sq(3, (c.g[SP][0] as u32));
                            c.mfc1(2, 1);
                            c.lqc2(4, (c.g[A0][0] as u32).wrapping_sub(0xea0));
                            c.qmtc2(2, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.c_lt_s(0, 2);
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                            let t = !c.fcc();
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_131194;
                            } else {
                                c.cycles += 12;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.mfc1(4, 2);
                                c.qmtc2(4, 3);
                                c.vu0().div(0, 3, 3, 0);
                                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                                c.vu0().waitq();
                                c.vu0().mul(0b1111, 4, 4, Src::Q);
                                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = true;
                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_13119c;
                                } else {
                                    break 'b_131194;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.lq(2, (c.g[A1][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_13119c;
                    }
                    'b_1311c8: {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[V1][0] = 0x2;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x438));
                        if t {
                            break 'b_1311c8;
                        } else {
                            c.cycles += 6;
                            c.g[A1][0] = 0x167;
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001311c0);
                            c.g[T0][0] = c.g[SP][0];
                            call_ptr(c, 0x0010eb30, 0x001311c0)?;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1311e4;
                            } else {
                                break 'b_1311c8;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x166;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001311d8);
                    c.g[T0][0] = c.g[SP][0];
                    call_ptr(c, 0x0010eb30, 0x001311d8)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_1311e4;
                    } else {
                        break 'b_1311e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_1311e4;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void MissionRider_dtor(MissionRider* self, int flags)` (0x1446f8): points the
/// object and its rider at the MissionRider vtables (copying the vtable to the
/// stack with relocated deltas when `flags` is 0), runs `Player_dtor`, then
/// `Rider_dtor` for flag 2 and `delete` for flag 1.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn mission_rider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_14485c: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.g[V0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[V1][0] = 0x460000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7cc8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7dc0));
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
            c.g[T1][0] = c.g[V1][0];
            c.sw((c.g[A2][0] as u32).wrapping_add(0x6e8), c.g[A3][0]);
            c.g[V1][0] = 0x460000;
            c.g[T0][0] = c.g[V1][0];
            c.g[S1][0] = c.g[A1][0];
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7d78));
            c.sw((c.g[A2][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_14485c;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.g[A3][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_144768: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld((c.g[V1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A1][0] as u32), c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00144768);
                        }
                        continue 'l_144768;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7dc0));
                        c.g[A2][0] = c.ld((c.g[V1][0] as u32));
                        c.sd((c.g[A1][0] as u32), c.g[A2][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7d78));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A1][0] = c.g[T3][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe10));
                        c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_1447cc: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] != c.g[A3][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001447cc);
                                }
                                continue 'l_1447cc;
                            } else {
                                c.cycles += 25;
                                c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7d78));
                                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xa0));
                                c.g[A3][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x80));
                                c.g[T0][0] =
                                    sx((c.g[T0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[A3][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x90));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[A2][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.sw((c.g[T1][0] as u32).wrapping_add(0x6c0), c.g[T3][0]);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[A2][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A0][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[T0][0]);
                                break 'b_14485c;
                            }
                        }
                    }
                }
            }
        }
        'b_144884: {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00144868);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00125eb8, 0x00144868)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if t {
                break 'b_144884;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[RA][0] = sx(0x00144880);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0011b978, 0x00144880)?;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                break 'b_144884;
            }
        }
        'b_144898: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                break 'b_144898;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00144894);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x00144894)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                break 'b_144898;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    });
}

/// `char* BENewPlayerInt_GetDisplayName(self, int idx)` (0x146f88): the name shown
/// for player `idx`. In a network game (idx < 2) it is one of the two network
/// names (swapped when not host); otherwise the temp player's user name, which
/// is first filled with "PLAYER n" in the current language when empty.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn be_new_player_int_get_display_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_147118: {
            'b_147000: {
                'b_146ffc: {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = 0x7;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[RA][0] = sx(0x00146fb4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x00146fb4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    if t {
                        break 'b_146ffc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_147000;
                        } else {
                            'b_146fec: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_146fec;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] != 0u64;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x498));
                                    if t {
                                        break 'b_147118;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                                        if t {
                                            break 'b_147118;
                                        } else {
                                            break 'b_146fec;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                            if t {
                                break 'b_147118;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x498));
                                if t {
                                    break 'b_147118;
                                } else {
                                    break 'b_146ffc;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                break 'b_147000;
            }
            'b_147100: {
                c.cycles += 10;
                c.g[RA][0] = sx(0x00147008);
                c.g[S2][0] = 0x1c;
                call_ptr(c, 0x00147798, 0x00147008)?;
                c.mult(0, 2, 2, 18);
                c.g[V1][0] = 0x530000;
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
                c.g[S3][0] = c.g[V1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                let t = c.g[V1][0] != 0u64;

                if t {
                    break 'b_147100;
                } else {
                    'b_1470e0: {
                        'b_1470cc: {
                            'b_1470b8: {
                                'b_1470a0: {
                                    c.cycles += 12;
                                    c.g[RA][0] = sx(0x00147030);
                                    call_ptr(c, 0x0014dc80, 0x00147030)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x0014703c);
                                    c.g[A1][0] = 0x4;
                                    call_ptr(c, 0x0014dd58, 0x0014703c)?;
                                    c.g[V1][0] = 0x530000;
                                    c.g[A0][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5610));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
                                    c.g[V1][0] = c.g[V0][0] & 0x7;
                                    let t = c.g[V1][0] == c.g[A0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_1470a0;
                                    } else {
                                        'b_147088: {
                                            'b_147070: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_147070;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == 0u64;

                                                    if t {
                                                        break 'b_147088;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_147100;
                                                        } else {
                                                            break 'b_147070;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1470b8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_1470e0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_147100;
                                                    } else {
                                                        break 'b_147088;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x00147090);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00147798, 0x00147090)?;
                                        c.mult(0, 3, 2, 18);
                                        c.g[A1][0] = 0x460000;
                                        let t = true;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5dd8));
                                        if t {
                                            break 'b_1470cc;
                                        } else {
                                            break 'b_1470a0;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001470a8);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00147798, 0x001470a8)?;
                                c.mult(0, 3, 2, 18);
                                c.g[A1][0] = 0x460000;
                                let t = true;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5dc8));
                                if t {
                                    break 'b_1470cc;
                                } else {
                                    break 'b_1470b8;
                                }
                            }
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x001470c0);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00147798, 0x001470c0)?;
                            c.mult(0, 3, 2, 18);
                            c.g[A1][0] = 0x460000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5db8));
                            break 'b_1470cc;
                        }
                        c.cycles += 5;
                        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[RA][0] = sx(0x001470d8);
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        call_ptr(c, 0x00417828, 0x001470d8)?;
                        let t = true;

                        if t {
                            break 'b_147100;
                        } else {
                            break 'b_1470e0;
                        }
                    }
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001470e8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00147798, 0x001470e8)?;
                    c.mult(0, 3, 2, 18);
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5da8));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00147100);
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    call_ptr(c, 0x00417828, 0x00147100)?;
                    break 'b_147100;
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x00147108);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147798, 0x00147108)?;
            c.g[V1][0] = 0x1c;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4fe0));
            c.mult(0, 2, 2, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            break 'b_147118;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void BELibrary_SerializeSave(char* data)` (0x14a188): copies the race DB,
/// the player DB, the profile blocks of rider 0 and (if present) rider 1, the
/// 0x9b50-byte block and the rules into the save buffer, one after another
/// (8-byte-aligned fast path or unaligned `ldl/ldr` path, per block).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn be_library_serialize_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_14a258: {
            'b_14a210: {
                c.cycles += 7;
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.g[A0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                c.g[V0][0] = c.g[A1][0] | c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A1][0];
                if t {
                    break 'b_14a210;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = 0x530000;
                    c.g[T2][0] = 0x4a0000;
                    c.g[T3][0] = 0x4c0000;
                    c.g[T4][0] = 0x530000;
                    'l_14a1bc: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(3, (c.g[T0][0] as u32));
                        c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x17));
                        c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.ldl(13, (c.g[T0][0] as u32).wrapping_add(0x1f));
                        c.ldr(13, (c.g[T0][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, (c.g[A3][0] as u32));
                        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(13, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(13, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                        let t = c.g[T0][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a1bc);
                            }
                            continue 'l_14a1bc;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = c.g[A0][0];
                            if t {
                                break 'b_14a258;
                            } else {
                                break 'b_14a210;
                            }
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            c.g[T1][0] = 0x530000;
            c.g[T2][0] = 0x4a0000;
            c.g[T3][0] = 0x4c0000;
            c.g[T4][0] = 0x530000;
            'l_14a228: loop {
                c.cycles += 11;
                c.g[T6][0] = c.ld((c.g[T0][0] as u32));
                c.g[T7][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A3][0] as u32), c.g[T6][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[T7][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                let t = c.g[T0][0] != c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a228);
                    }
                    continue 'l_14a228;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[A0][0];
                    break 'b_14a258;
                }
            }
        }
        'b_14a31c: {
            'b_14a2e8: {
                c.cycles += 13;
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
                c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
                c.ldl(13, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[T0][0] as u32));
                c.lwl(14, (c.g[T0][0] as u32).wrapping_add(0xb));
                c.lwr(14, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.sdl(13, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(13, (c.g[A3][0] as u32));
                c.swl(14, (c.g[A3][0] as u32).wrapping_add(0xb));
                c.g[V1][0] = c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.swr(14, (c.g[A3][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_14a2e8;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    'l_14a294: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(7, (c.g[A2][0] as u32));
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(13, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(13, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(14, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(14, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[V1][0] as u32));
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a294);
                            }
                            continue 'l_14a294;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a31c;
                            } else {
                                break 'b_14a2e8;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
            'l_14a2f0: loop {
                c.cycles += 11;
                c.g[T7][0] = c.ld((c.g[A2][0] as u32));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd((c.g[V1][0] as u32), c.g[T7][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a2f0);
                    }
                    continue 'l_14a2f0;
                } else {
                    break 'b_14a31c;
                }
            }
        }
        'b_14a3e4: {
            'b_14a3b0: {
                c.cycles += 14;
                c.ldl(13, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[A2][0] as u32));
                c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6ca8));
                c.sdr(13, (c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
                c.g[V1][0] = 0xf88;
                c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x11));
                c.mult(0, 3, 2, 3);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[A1][0];
                if t {
                    break 'b_14a3b0;
                } else {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                    'l_14a35c: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(4, (c.g[V1][0] as u32));
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[A2][0] as u32));
                        c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a35c);
                            }
                            continue 'l_14a35c;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a3e4;
                            } else {
                                break 'b_14a3b0;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
            'l_14a3b8: loop {
                c.cycles += 11;
                c.g[T5][0] = c.ld((c.g[V1][0] as u32));
                c.g[T6][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A2][0] as u32), c.g[T5][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a3b8);
                    }
                    continue 'l_14a3b8;
                } else {
                    break 'b_14a3e4;
                }
            }
        }
        'b_14a4c8: {
            c.cycles += 8;
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, (c.g[V1][0] as u32));
            c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
            c.sdr(5, (c.g[A2][0] as u32));
            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2d));
            let t = (c.g[V0][0] as i64) < 0;
            c.g[A1][0] = c.g[A3][0];
            if t {
                break 'b_14a4c8;
            } else {
                'b_14a4b8: {
                    'b_14a484: {
                        c.cycles += 9;
                        c.g[A0][0] = 0xf88;
                        c.g[V1][0] = 0x4b0000;
                        c.mult(0, 4, 2, 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f8));
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.g[A1][0];
                        if t {
                            break 'b_14a484;
                        } else {
                            c.cycles += 2;
                            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                            'l_14a430: loop {
                                c.cycles += 19;
                                c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(14, (c.g[V1][0] as u32));
                                c.ldl(15, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(15, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(14, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.sdr(14, (c.g[A2][0] as u32));
                                c.sdl(15, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.sdr(15, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0014a430);
                                    }
                                    continue 'l_14a430;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_14a4b8;
                                    } else {
                                        break 'b_14a484;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                    'l_14a48c: loop {
                        c.cycles += 11;
                        c.g[T0][0] = c.ld((c.g[V1][0] as u32));
                        c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[T5][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[A2][0] as u32), c.g[T0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a48c);
                            }
                            continue 'l_14a48c;
                        } else {
                            break 'b_14a4b8;
                        }
                    }
                }
                c.cycles += 4;
                c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(14, (c.g[V1][0] as u32));
                c.sdl(14, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(14, (c.g[A2][0] as u32));
                break 'b_14a4c8;
            }
        }
        'b_14a56c: {
            'b_14a538: {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x5cb8));
                c.g[V0][0] = c.g[A3][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[A3][0];
                if t {
                    break 'b_14a538;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x9b40;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    'l_14a4e4: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, (c.g[A1][0] as u32));
                        c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, (c.g[V1][0] as u32));
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a4e4);
                            }
                            continue 'l_14a4e4;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a56c;
                            } else {
                                break 'b_14a538;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x9b40;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            'l_14a540: loop {
                c.cycles += 11;
                c.g[T5][0] = c.ld((c.g[A1][0] as u32));
                c.g[T6][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[V1][0] as u32), c.g[T5][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a540);
                    }
                    continue 'l_14a540;
                } else {
                    break 'b_14a56c;
                }
            }
        }
        c.cycles += 29;
        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(6, (c.g[A1][0] as u32));
        c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(6, (c.g[V1][0] as u32));
        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = 0x9b50;
        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8b8));
        c.ldl(13, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(13, (c.g[V0][0] as u32));
        c.ldl(14, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.ldr(14, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.ldl(15, (c.g[V0][0] as u32).wrapping_add(0x17));
        c.ldr(15, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.sdl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.sdr(13, (c.g[A1][0] as u32));
        c.sdl(14, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.sdr(14, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(15, (c.g[A1][0] as u32).wrapping_add(0x17));
        c.sdr(15, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.ldl(13, (c.g[V0][0] as u32).wrapping_add(0x1f));
        c.ldr(13, (c.g[V0][0] as u32).wrapping_add(0x18));
        c.sdl(13, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.sdr(13, (c.g[A1][0] as u32).wrapping_add(0x18));
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    });
}

/// `void BENetworkInt_PackPlayerData(NetPlayerData* out, int player)` (0x14e2c8):
/// fills the network packet with the race globals, the six player DBs, this
/// player's record and character id, the rule triples and the profile blob,
/// moves, tag and key tables of this player's profile.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn be_network_int_pack_player_data(c: &mut Cpu) {
    guest(c, |c| {
        'b_14e3a0: {
            'b_14e364: {
                c.cycles += 14;
                c.g[T1][0] = c.g[A2][0];
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
                c.g[V0][0] = 0x530000;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.sq(16, (c.g[SP][0] as u32));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f34));
                c.g[V0][0] = c.g[T1][0] | c.g[A2][0];
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc3c), c.g[V1][0]);
                c.g[T3][0] = c.g[A1][0];
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc40), c.g[A0][0]);
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[T1][0];
                if t {
                    break 'b_14e364;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                    c.g[A3][0] = 0x530000;
                    c.g[T4][0] = 0x530000;
                    c.g[T5][0] = 0x4a0000;
                    'l_14e310: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(4, (c.g[A2][0] as u32));
                        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[V1][0] as u32));
                        c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014e310);
                            }
                            continue 'l_14e310;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14e3a0;
                            } else {
                                break 'b_14e364;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[A3][0] = 0x530000;
            c.g[T4][0] = 0x530000;
            c.g[T5][0] = 0x4a0000;
            'l_14e374: loop {
                c.cycles += 11;
                c.g[T6][0] = c.ld((c.g[A2][0] as u32));
                c.g[T7][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd((c.g[V1][0] as u32), c.g[T6][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T7][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014e374);
                    }
                    continue 'l_14e374;
                } else {
                    break 'b_14e3a0;
                }
            }
        }
        c.cycles += 10;
        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.ldr(5, (c.g[A2][0] as u32));
        c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(5, (c.g[V1][0] as u32));
        c.g[V0][0] = 0x530000;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x68));
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa8));
        'l_14e3c8: loop {
            c.cycles += 18;
            c.ldl(15, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(15, (c.g[A0][0] as u32));
            c.ldl(16, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.ldr(16, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x17));
            c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
            c.sdl(15, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(15, (c.g[V1][0] as u32));
            c.sdl(16, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(16, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A2][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0014e3c8);
                }
                continue 'l_14e3c8;
            } else {
                c.cycles += 48;
                c.g[V0][0] = 0x1c;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5b20));
                c.mult(0, 2, 11, 2);
                c.g[V1][0] = 0x9b50;
                c.g[A1][0] = 0xf88;
                c.mult(0, 3, 11, 3);
                c.g[A2][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8d0));
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f38));
                c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x316));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[T2][0] = 0x20c;
                c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[V0][0] as u32));
                c.ldl(14, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(14, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.ldl(15, (c.g[V0][0] as u32).wrapping_add(0x17));
                c.ldr(15, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.sdl(7, (c.g[T1][0] as u32).wrapping_add(0x53));
                c.sdr(7, (c.g[T1][0] as u32).wrapping_add(0x4c));
                c.sdl(14, (c.g[T1][0] as u32).wrapping_add(0x5b));
                c.sdr(14, (c.g[T1][0] as u32).wrapping_add(0x54));
                c.sdl(15, (c.g[T1][0] as u32).wrapping_add(0x63));
                c.sdr(15, (c.g[T1][0] as u32).wrapping_add(0x5c));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x64), c.g[S0][0]);
                c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x11));
                c.mult(0, 5, 7, 5);
                c.sb((c.g[T1][0] as u32).wrapping_add(0x110), c.g[A3][0]);
                c.g[S0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8d0));
                c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[S0][0] as u32));
                c.sdl(2, (c.g[T1][0] as u32).wrapping_add(0xc3b));
                c.sdr(2, (c.g[T1][0] as u32).wrapping_add(0xc34));
                c.ldl(2, (c.g[A2][0] as u32).wrapping_sub(0x11));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_sub(0x18));
                c.g[T4][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x10));
                c.sdl(2, (c.g[T1][0] as u32).wrapping_add(0xc23));
                c.sdr(2, (c.g[T1][0] as u32).wrapping_add(0xc1c));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc24), c.g[T4][0]);
                c.ldl(16, (c.g[A2][0] as u32).wrapping_sub(0x5));
                c.ldr(16, (c.g[A2][0] as u32).wrapping_sub(0xc));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x4));
                c.sdl(16, (c.g[T1][0] as u32).wrapping_add(0xc2f));
                c.sdr(16, (c.g[T1][0] as u32).wrapping_add(0xc28));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc30), c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                'l_14e4d0: loop {
                    c.cycles += 8;
                    c.lwl(15, (c.g[V1][0] as u32).wrapping_add(0x3));
                    c.lwr(15, (c.g[V1][0] as u32));
                    c.swl(15, (c.g[T0][0] as u32).wrapping_add(0x3));
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                    c.swr(15, (c.g[T0][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                    let t = (c.g[T2][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014e4d0);
                        }
                        continue 'l_14e4d0;
                    } else {
                        c.cycles += 10;
                        c.g[V1][0] = 0x9b50;
                        c.g[A0][0] = 0xf88;
                        c.mult(0, 3, 11, 3);
                        c.mult(0, 4, 7, 4);
                        c.g[V0][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xb4c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7778));
                        c.g[A2][0] = 0x19;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        'l_14e518: loop {
                            c.cycles += 8;
                            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(2, (c.g[V1][0] as u32));
                            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.sdr(2, (c.g[A1][0] as u32));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                            let t = (c.g[A2][0] as i64) >= 0;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014e518);
                                }
                                continue 'l_14e518;
                            } else {
                                c.cycles += 22;
                                c.g[V1][0] = 0x9b50;
                                c.g[A0][0] = 0xf88;
                                c.mult(0, 4, 7, 4);
                                c.mult(0, 3, 11, 3);
                                c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add(0x7887));
                                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x118));
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7));
                                c.g[T0][0] = 0x54;
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.lwl(4, (c.g[V0][0] as u32).wrapping_add(0x3));
                                c.lwr(4, (c.g[V0][0] as u32));
                                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4));
                                c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5));
                                c.g[T2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x6));
                                c.swl(4, (c.g[T1][0] as u32).wrapping_add(0x114));
                                c.swr(4, (c.g[T1][0] as u32).wrapping_add(0x111));
                                c.sb((c.g[T1][0] as u32).wrapping_add(0x115), c.g[A1][0]);
                                c.sb((c.g[T1][0] as u32).wrapping_add(0x116), c.g[A3][0]);
                                c.sb((c.g[T1][0] as u32).wrapping_add(0x117), c.g[T2][0]);
                                'l_14e590: loop {
                                    c.cycles += 10;
                                    c.lwl(12, (c.g[V1][0] as u32).wrapping_add(0x3));
                                    c.lwr(12, (c.g[V1][0] as u32));
                                    c.g[T6][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.swl(12, (c.g[A2][0] as u32).wrapping_add(0x3));
                                    c.swr(12, (c.g[A2][0] as u32));
                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                                    c.sh((c.g[A2][0] as u32).wrapping_add(0x4), c.g[T6][0]);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6));
                                    let t = (c.g[T0][0] as i64) >= 0;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0014e590);
                                        }
                                        continue 'l_14e590;
                                    } else {
                                        c.cycles += 3;
                                        c.lq(16, (c.g[SP][0] as u32));
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.pc = t;
                                        return Ok(());
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

/// `void BENetworkInt_UnpackPlayerData(NetPlayerData* in, int player)` (0x14e5c8):
/// the inverse of the pack: restores the race globals, the other players' DBs
/// (not as host), this player's record and rules from the packet, re-inits the
/// character profile and rivalries and copies the profile blob, moves, tag and
/// keys back, calling the BE interface `discard` hooks (0, 1, 6) in between.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn be_network_int_unpack_player_data(c: &mut Cpu) {
    guest(c, |c| {
        'b_14e6d0: {
            'b_14e68c: {
                c.cycles += 22;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = 0x530000;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A2][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = c.g[S3][0] | c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S5][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.g[V0][0] & 0x7;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc3c));
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc40));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f34), c.g[A0][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[S3][0];
                if t {
                    break 'b_14e68c;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = 0x530000;
                    c.g[S2][0] = 0x530000;
                    c.g[S6][0] = 0x530000;
                    c.g[S1][0] = 0x4a0000;
                    c.g[S7][0] = 0x4a0000;
                    'l_14e638: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(4, (c.g[V1][0] as u32));
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[A2][0] as u32));
                        c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(7, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014e638);
                            }
                            continue 'l_14e638;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14e6d0;
                            } else {
                                break 'b_14e68c;
                            }
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = 0x530000;
            c.g[S2][0] = 0x530000;
            c.g[S6][0] = 0x530000;
            c.g[S1][0] = 0x4a0000;
            c.g[S7][0] = 0x4a0000;
            'l_14e6a4: loop {
                c.cycles += 11;
                c.g[T1][0] = c.ld((c.g[V1][0] as u32));
                c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A2][0] as u32), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014e6a4);
                    }
                    continue 'l_14e6a4;
                } else {
                    break 'b_14e6d0;
                }
            }
        }
        'b_14e77c: {
            c.cycles += 19;
            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(7, (c.g[V1][0] as u32));
            c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.sdr(7, (c.g[A2][0] as u32));
            c.g[RA][0] = sx(0x0014e6ec);
            c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            call_ptr(c, 0x0014dc80, 0x0014e6ec)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014e6f8);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x0014e6f8)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014e70c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0014e70c)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4b30));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            let t = c.g[V1][0] != 0u64;
            c.g[V0][0] = 0x1c;
            if t {
                break 'b_14e77c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa8));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
                'l_14e730: loop {
                    c.cycles += 18;
                    c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[A0][0] as u32));
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[V1][0] as u32));
                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014e730);
                        }
                        continue 'l_14e730;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1c;
                        break 'b_14e77c;
                    }
                }
            }
        }
        'b_14e810: {
            c.cycles += 31;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5b20));
            c.mult(0, 2, 21, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.ldl(9, (c.g[S3][0] as u32).wrapping_add(0x53));
            c.ldr(9, (c.g[S3][0] as u32).wrapping_add(0x4c));
            c.ldl(10, (c.g[S3][0] as u32).wrapping_add(0x5b));
            c.ldr(10, (c.g[S3][0] as u32).wrapping_add(0x54));
            c.ldl(3, (c.g[S3][0] as u32).wrapping_add(0x63));
            c.ldr(3, (c.g[S3][0] as u32).wrapping_add(0x5c));
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x64));
            c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.sdr(9, (c.g[V0][0] as u32));
            c.sdl(10, (c.g[V0][0] as u32).wrapping_add(0xf));
            c.sdr(10, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x17));
            c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.g[RA][0] = sx(0x0014e7c8);
            call_ptr(c, 0x0014dc80, 0x0014e7c8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014e7d4);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x0014e7d4)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014e7e8);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0014e7e8)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4b30));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            let t = c.g[V1][0] != 0u64;
            c.g[S4][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_14e810;
            } else {
                c.cycles += 6;
                c.g[V0][0] = 0x530000;
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d0));
                c.ldl(5, (c.g[S3][0] as u32).wrapping_add(0xc3b));
                c.ldr(5, (c.g[S3][0] as u32).wrapping_add(0xc34));
                c.sdl(5, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[T0][0] as u32));
                break 'b_14e810;
            }
        }
        c.cycles += 28;
        c.g[V1][0] = 0xc;
        c.g[S2][0] = 0xf88;
        c.mult(0, 3, 21, 3);
        c.mult(0, 18, 20, 18);
        c.g[S0][0] = 0x9b50;
        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8b8));
        c.mult(0, 16, 21, 16);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 0x1;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xc23));
        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0xc1c));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc24));
        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(7, (c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        c.g[RA][0] = sx(0x0014e864);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, 0x00151600, 0x0014e864)?;
        c.g[RA][0] = sx(0x0014e86c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        call_ptr(c, 0x00156810, 0x0014e86c)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x290));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x316));
        c.g[V1][0] = 0x20c;
        'l_14e880: loop {
            c.cycles += 8;
            c.lwl(4, (c.g[V0][0] as u32).wrapping_add(0x3));
            c.lwr(4, (c.g[V0][0] as u32));
            c.swl(4, (c.g[S0][0] as u32).wrapping_add(0x3));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.swr(4, (c.g[S0][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0014e880);
                }
                continue 'l_14e880;
            } else {
                c.cycles += 10;
                c.g[V1][0] = 0x9b50;
                c.g[A0][0] = 0xf88;
                c.mult(0, 3, 21, 3);
                c.mult(0, 4, 20, 4);
                c.g[V0][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xb4c));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7778));
                c.g[A2][0] = 0x19;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                'l_14e8c8: loop {
                    c.cycles += 8;
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[A1][0] as u32));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sdr(2, (c.g[V1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014e8c8);
                        }
                        continue 'l_14e8c8;
                    } else {
                        c.cycles += 22;
                        c.g[V1][0] = 0x9b50;
                        c.g[A0][0] = 0xf88;
                        c.mult(0, 4, 20, 4);
                        c.mult(0, 3, 21, 3);
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x7887));
                        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7));
                        c.g[A3][0] = 0x54;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.lwl(4, (c.g[S3][0] as u32).wrapping_add(0x114));
                        c.lwr(4, (c.g[S3][0] as u32).wrapping_add(0x111));
                        c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x115));
                        c.g[T0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x116));
                        c.g[T1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x117));
                        c.swl(4, (c.g[V0][0] as u32).wrapping_add(0x3));
                        c.swr(4, (c.g[V0][0] as u32));
                        c.sb((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                        c.sb((c.g[V0][0] as u32).wrapping_add(0x5), c.g[T0][0]);
                        c.sb((c.g[V0][0] as u32).wrapping_add(0x6), c.g[T1][0]);
                        'l_14e940: loop {
                            c.cycles += 10;
                            c.lwl(10, (c.g[A2][0] as u32).wrapping_add(0x3));
                            c.lwr(10, (c.g[A2][0] as u32));
                            c.g[V0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.swl(10, (c.g[V1][0] as u32).wrapping_add(0x3));
                            c.swr(10, (c.g[V1][0] as u32));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                            c.sh((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6));
                            let t = (c.g[A3][0] as i64) >= 0;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014e940);
                                }
                                continue 'l_14e940;
                            } else {
                                c.cycles += 21;
                                c.g[RA][0] = sx(0x0014e970);
                                call_ptr(c, 0x0014dc80, 0x0014e970)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0014e97c);
                                c.g[A1][0] = 0x6;
                                call_ptr(c, 0x0014dd58, 0x0014e97c)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                let t = c.g[A1][0] as u32;
                                c.g[RA][0] = sx(0x0014e990);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0014e990)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `int BEBoltInt_CollectEquippedTextureNames(self, profile, ch, out)` (0x14b7a8): for
/// each bolt of character `ch` that is active and whose profile entry has flag
/// 0x10, stores the bolt's texture file name (`BEBoltInt_GetBoltTextureName`)
/// in the next 256-byte slot of `out` unless an earlier slot already holds it
/// (`strcmp`). Returns the number of slots used.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn bolt_int_collect_equipped_texture_names(c: &mut Cpu) {
    guest(c, |c| {
        'b_14b8e8: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A3][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S7][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sw((c.g[SP][0] as u32), c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0014b7ec);
            c.g[S3][0] = 0u64;
            call_ptr(c, 0x0014bdb8, 0x0014b7ec)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0014b800);
            c.g[A2][0] = c.g[S7][0];
            call_ptr(c, 0x0014ad28, 0x0014b800)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S7][0];
            c.g[RA][0] = sx(0x0014b810);
            c.g[FP][0] = c.g[V0][0];
            call_ptr(c, 0x0014d998, 0x0014b810)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014b820);
            c.g[A1][0] = c.g[S7][0];
            call_ptr(c, 0x0014d988, 0x0014b820)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_14b8e8;
            } else {
                c.cycles += 1;
                'l_14b830: loop {
                    'b_14b8d4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                        c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_14b8d4;
                        } else {
                            'b_14b86c: {
                                'b_14b868: {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x288));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    if t {
                                        break 'b_14b868;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                        let t = true;
                                        c.g[V0][0] =
                                            sx((c.g[FP][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        if t {
                                            break 'b_14b86c;
                                        } else {
                                            break 'b_14b868;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_14b86c;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                            if t {
                                break 'b_14b8d4;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[S3][0] as u32) << 8);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                                c.g[S2][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A3][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x0014b8a0);
                                c.g[T0][0] = c.g[S2][0];
                                call_ptr(c, 0x0014b988, 0x0014b8a0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                if t {
                                    break 'b_14b8d4;
                                } else {
                                    c.cycles += 2;
                                    c.g[S0][0] = c.g[S1][0];
                                    'l_14b8b0: loop {
                                        'b_14b8d0: {
                                            c.cycles += 2;
                                            let t = (c.g[S0][0] as i64) < 0;
                                            c.g[A0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_14b8d0;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 8);
                                                c.g[RA][0] = sx(0x0014b8c4);
                                                c.g[A1][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                call_ptr(c, 0x004165a8, 0x0014b8c4)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0014b8b0);
                                                    }
                                                    continue 'l_14b8b0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S3][0] = c.g[S1][0];
                                                    break 'b_14b8d0;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'b_14b8d4;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S4][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014b830);
                        }
                        continue 'l_14b830;
                    } else {
                        break 'b_14b8e8;
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `int Bolts_CollectByGroup(self, g, key, out, depth, all)` (0x14d7e8): collects the
/// bolt nodes of group `g` whose parent is `key` into `out` and descends into
/// each match (recursively, `depth - 1`, unless it is 0x20-flagged and `all` is
/// clear). Returns how many pointers were written.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn bolts_collect_by_group(c: &mut Cpu) {
    guest(c, |c| {
        'b_14d8d0: {
            c.cycles += 30;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S7][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x38;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.g[V1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[FP][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S5][0] = c.g[T0][0];
            c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            c.g[S4][0] = c.g[T1][0];
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            c.g[S3][0] = 0u64;
            c.mult(0, 4, 18, 4);
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if t {
                break 'b_14d8d0;
            } else {
                c.cycles += 2;
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                'l_14d868: loop {
                    'b_14d8c8: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] != c.g[FP][0] {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_14d8c8;
                        } else {
                            'b_14d8c4: {
                                c.cycles += 4;
                                c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                let t = c.g[S5][0] == 0u64;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_14d8c4;
                                } else {
                                    'b_14d8a0: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x20;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A2][0] =
                                                c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                            break 'b_14d8a0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S4][0] == 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_14d8c8;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                break 'b_14d8a0;
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[A0][0] = c.g[S6][0];
                                    c.g[A1][0] = c.g[S7][0];
                                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                    c.g[RA][0] = sx(0x0014d8b8);
                                    c.g[T1][0] = c.g[S4][0];
                                    call_ptr(c, 0x0014d7e8, 0x0014d8b8)?;
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[S3][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[S1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    break 'b_14d8c4;
                                }
                            }
                            c.cycles += 1;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_14d8c8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014d868);
                        }
                        continue 'l_14d868;
                    } else {
                        break 'b_14d8d0;
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
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
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc675 fakes, then one result word per fake slot.
    const OC675_CTRL: u32 = DATA + 0xf0000;
    const OC675_MAGIC: u32 = 0x0675_0675;
    /// Result word meaning "keep the fake's own (logged) result".
    const OC675_KEEP: u32 = 0x5a5a_5a5a;

    /// A gated fake for the callee at `$target`: logs the call (the RAM log is
    /// part of the comparison) and returns the result word the fixture left at
    /// `CTRL + 4 + 4 * $slot`. Gate off: the body the slot held before.
    macro_rules! oc675_fake {
        ($fake:ident, $prev:ident, $target:literal, $generated:path, $slot:literal) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC675_CTRL) != OC675_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                let want = c.bus.read32(OC675_CTRL + 4 + 4 * $slot);
                fake(c, $target, ra);
                if c.pc == ra && want != OC675_KEEP {
                    c.g[V0][0] = sx(want);
                }
            }
        };
    }

    oc675_fake!(
        oc675_fake_start_press,
        OC675_PREV_START_PRESS,
        0x0011_99f8,
        ee::s00::f_001199f8__generated,
        0
    );
    oc675_fake!(
        oc675_fake_add_boost,
        OC675_PREV_ADD_BOOST,
        0x0010_e098,
        ee::s00::f_0010e098__generated,
        1
    );
    oc675_fake!(
        oc675_fake_anim_play,
        OC675_PREV_ANIM_PLAY,
        0x0031_28e8,
        ee::s10::f_003128e8__generated,
        2
    );
    oc675_fake!(
        oc675_fake_check_switch,
        OC675_PREV_CHECK_SWITCH,
        0x0013_06b0,
        ee::s00::f_001306b0__generated,
        3
    );
    oc675_fake!(
        oc675_fake_check_jump,
        OC675_PREV_CHECK_JUMP,
        0x0013_07b8,
        ee::s00::f_001307b8__generated,
        4
    );
    oc675_fake!(
        oc675_fake_check_spin,
        OC675_PREV_CHECK_SPIN,
        0x0013_08d8,
        ee::s00::f_001308d8__generated,
        5
    );
    oc675_fake!(
        oc675_fake_check_release,
        OC675_PREV_CHECK_RELEASE,
        0x0013_1200,
        ee::s00::f_00131200__generated,
        6
    );
    oc675_fake!(
        oc675_fake_bx_audio,
        OC675_PREV_BX_AUDIO,
        0x0028_b180,
        ee::s07::f_0028b180__generated,
        7
    );
    oc675_fake!(
        oc675_fake_speech_on,
        OC675_PREV_SPEECH_ON,
        0x0029_f0a0,
        ee::s08::f_0029f0a0__generated,
        8
    );
    oc675_fake!(
        oc675_fake_start_wipeout,
        OC675_PREV_START_WIPEOUT,
        0x0010_eb30,
        ee::s00::f_0010eb30__generated,
        9
    );
    oc675_fake!(
        oc675_fake_player_dtor,
        OC675_PREV_PLAYER_DTOR,
        0x0012_5eb8,
        ee::s00::f_00125eb8__generated,
        10
    );
    oc675_fake!(
        oc675_fake_rider_dtor,
        OC675_PREV_RIDER_DTOR,
        0x0011_b978,
        ee::s00::f_0011b978__generated,
        11
    );
    oc675_fake!(
        oc675_fake_delete,
        OC675_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated,
        12
    );
    oc675_fake!(
        oc675_fake_get_interface,
        OC675_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated,
        13
    );
    oc675_fake!(
        oc675_fake_get_slot,
        OC675_PREV_GET_SLOT,
        0x0014_7798,
        ee::s01::f_00147798__generated,
        14
    );
    oc675_fake!(
        oc675_fake_get_instance,
        OC675_PREV_GET_INSTANCE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated,
        15
    );
    oc675_fake!(
        oc675_fake_sprintf,
        OC675_PREV_SPRINTF,
        0x0041_7828,
        ee::s15::f_00417828__generated,
        16
    );
    oc675_fake!(
        oc675_fake_init_profile,
        OC675_PREV_INIT_PROFILE,
        0x0015_1600,
        ee::s01::f_00151600__generated,
        17
    );
    oc675_fake!(
        oc675_fake_reset_rivalries,
        OC675_PREV_RESET_RIVALRIES,
        0x0015_6810,
        ee::s01::f_00156810__generated,
        18
    );

    static OC675_PREV_BOLT_TEXTURE: AtomicUsize = AtomicUsize::new(0);

    /// `BEBoltInt_GetBoltTextureName(self, profile, ch, def, out)`: bolts whose
    /// `def+0x34` is 3 mod 4 have no texture (result 0, empty name); the others
    /// get the name "t0".."t2" in `out` (result 1). The name is a function of
    /// `def+0x34`, so repeated names occur and the collector's `strcmp` loop runs.
    fn oc675_fake_bolt_texture(c: &mut Cpu) {
        if c.bus.read32(OC675_CTRL) != OC675_MAGIC {
            return fall_back(c, &OC675_PREV_BOLT_TEXTURE, ee::s01::f_0014b988__generated);
        }
        let ra = c.g[RA][0] as u32;
        let out = c.g[T0][0] as u32;
        let def = c.g[A3][0] as u32;
        let k = c.bus.read32(def + 0x34) % 4;
        fake(c, 0x0014_b988, ra);
        if c.pc == ra {
            if k == 3 {
                c.bus.write8(out, 0);
                c.g[V0][0] = 0;
            } else {
                c.bus.write8(out, b't');
                c.bus.write8(out + 1, b'0' + k as u8);
                c.bus.write8(out + 2, 0);
                c.g[V0][0] = 1;
            }
        }
    }

    fn oc675_install() {
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001199F8),
            oc675_fake_start_press,
            &OC675_PREV_START_PRESS,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E098),
            oc675_fake_add_boost,
            &OC675_PREV_ADD_BOOST,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            oc675_fake_anim_play,
            &OC675_PREV_ANIM_PLAY,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001306B0),
            oc675_fake_check_switch,
            &OC675_PREV_CHECK_SWITCH,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001307B8),
            oc675_fake_check_jump,
            &OC675_PREV_CHECK_JUMP,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001308D8),
            oc675_fake_check_spin,
            &OC675_PREV_CHECK_SPIN,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00131200),
            oc675_fake_check_release,
            &OC675_PREV_CHECK_RELEASE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc675_fake_bx_audio,
            &OC675_PREV_BX_AUDIO,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F0A0),
            oc675_fake_speech_on,
            &OC675_PREV_SPEECH_ON,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010EB30),
            oc675_fake_start_wipeout,
            &OC675_PREV_START_WIPEOUT,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00125EB8),
            oc675_fake_player_dtor,
            &OC675_PREV_PLAYER_DTOR,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc675_fake_rider_dtor,
            &OC675_PREV_RIDER_DTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc675_fake_delete,
            &OC675_PREV_DELETE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc675_fake_get_interface,
            &OC675_PREV_GET_INTERFACE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147798),
            oc675_fake_get_slot,
            &OC675_PREV_GET_SLOT,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc675_fake_get_instance,
            &OC675_PREV_GET_INSTANCE,
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc675_fake_sprintf,
            &OC675_PREV_SPRINTF,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00151600),
            oc675_fake_init_profile,
            &OC675_PREV_INIT_PROFILE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00156810),
            oc675_fake_reset_rivalries,
            &OC675_PREV_RESET_RIVALRIES,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014B988),
            oc675_fake_bolt_texture,
            &OC675_PREV_BOLT_TEXTURE,
        );
    }

    /// A machine with the fakes gated on (every result "keep" except the
    /// `results` the test forces as `(slot, word)`), entered at `entry`.
    fn oc675_begin(c: &mut Cpu, seed: u32, entry: u32, calls: u32, results: &[(u32, u32)]) -> Rng {
        oc675_install();
        let r = machine(c, seed, 0x0675_0000 ^ entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC675_CTRL, OC675_MAGIC);
        for slot in 0..24u32 {
            c.bus.write32(OC675_CTRL + 4 + 4 * slot, OC675_KEEP);
        }
        for &(slot, word) in results {
            c.bus.write32(OC675_CTRL + 4 + 4 * slot, word);
        }
        r
    }

    /// The board press control at `DATA` (rider at +0x1000, the rider's anim at
    /// +0x2000, its metrix at +0x3000) and the command at `DATA + 0x100`; the
    /// game image gives the `.lit4` constants.
    const OC675_SELF: u32 = DATA;
    const OC675_IN: u32 = DATA + 0x100;
    const OC675_RIDER: u32 = DATA + 0x1000;
    const OC675_CURVE_A: u32 = DATA + 0x4000;
    const OC675_CURVE_B: u32 = DATA + 0x4040;

    fn oc675_press_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        elf_image(c);
        fill(c, r, OC675_SELF, 0x40);
        fill(c, r, OC675_RIDER, 0x200);
        fill(c, r, DATA + 0x2000, 0x40);
        fill(c, r, DATA + 0x3000, 0x40);
        c.bus.write32(OC675_SELF + 0x14, OC675_RIDER);
        c.bus.write32(OC675_RIDER + 0x784, DATA + 0x2000);
        c.bus.write32(OC675_RIDER + 0x790, DATA + 0x3000);
        let scale = [1.0f32, 2.0, 0.5, 0.0][(seed % 4) as usize];
        c.bus.write32(OC675_RIDER + 0x300, scale.to_bits());
        // Balance curves: four (x, y) pairs each (x at +0, +8, +0x10, +0x18).
        let xs = [0.0f32, 0.25, 0.5, 1.0];
        let ya = [1.0f32, 1.5, 2.0, 2.5];
        let yb = [3.0f32, 2.0, 1.0, 0.5];
        for i in 0..4u32 {
            c.bus
                .write32(OC675_CURVE_A + 8 * i, xs[i as usize].to_bits());
            c.bus
                .write32(OC675_CURVE_A + 8 * i + 4, ya[i as usize].to_bits());
            c.bus
                .write32(OC675_CURVE_B + 8 * i, xs[i as usize].to_bits());
            c.bus
                .write32(OC675_CURVE_B + 8 * i + 4, yb[i as usize].to_bits());
        }
        c.bus.write32(GAME_GP - 0x1fdc, OC675_CURVE_A);
        c.bus.write32(GAME_GP - 0x1fd4, OC675_CURVE_B);
        c.g[A0][0] = OC675_SELF as u64;
        c.g[A1][0] = OC675_IN as u64;
    }

    /// The command word: press stick in bits 0..5, pivot stick in bits 6..11
    /// (signed six-bit fields, scaled by 1/31 in the game), junk above.
    fn oc675_sticks(press: i32, pivot: i32, r: &mut Rng) -> u32 {
        (r.next_u32() & !0xfff) | ((pivot as u32 & 0x3f) << 6) | (press as u32 & 0x3f)
    }

    /// `RiderBoardPressSub_UpdateStart`: both stick/slew branches, PressMode 2
    /// or not, and the call-unwind paths of the animation, boost and metrix calls.
    #[test]
    fn rider_board_press_sub_update_start_matches() {
        check(
            ee::s00::f_00130228__generated,
            rider_board_press_sub_update_start,
            288,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0013_0228, 3, &[]);
                oc675_press_world(c, &mut r, seed / 72);
                let press = [0, 0, 5, -3][(seed % 4) as usize];
                let pivot = [0, 15, 16, -16, 31, -32][(seed / 4 % 6) as usize];
                let cur = [1.0f32, 0.0, 0.5][(seed / 24 % 3) as usize];
                let mode = [2u32, 0, 2, 1][(seed / 72 % 4) as usize];
                let sticks = oc675_sticks(press, pivot, &mut r);
                c.bus.write32(OC675_IN + 4, sticks);
                c.bus.write32(OC675_RIDER + 0x268, cur.to_bits());
                c.bus.write32(OC675_RIDER + 0x330, mode);
            },
        );
    }

    /// The shared balance fixture: balance clocks, slew-11 value, press mode.
    fn oc675_balance_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc675_press_world(c, r, seed);
        let balance = [0.0f32, 0.4, 1.0, 3.0][(r.next_u32() % 4) as usize];
        let off = [0.0f32, 0.5, 1.2, 5.0][(r.next_u32() % 4) as usize];
        let cur = [1.0f32, 0.9, 0.85, 0.7, 0.55, 0.45, 0.3, 0.0][(r.next_u32() % 8) as usize];
        let mode = [2u32, 1, 0][(r.next_u32() % 3) as usize];
        let press = [0, 6, 20, -6, -20][(r.next_u32() % 5) as usize];
        let pivot = [0, 10, -25, 31][(r.next_u32() % 4) as usize];
        let sticks = oc675_sticks(press, pivot, r);
        c.bus.write32(OC675_SELF + 4, balance.to_bits());
        c.bus.write32(OC675_SELF + 8, off.to_bits());
        c.bus.write32(OC675_RIDER + 0x274, cur.to_bits());
        c.bus.write32(OC675_RIDER + 0x330, mode);
        c.bus.write32(OC675_IN + 4, sticks);
        if r.next_u32() % 3 == 0 {
            // A resting rider: zero velocity.
            for i in 0..4u32 {
                c.bus.write32(OC675_RIDER + 0x1e0 + 4 * i, 0);
            }
        }
    }

    /// `RiderBoardPressSub_UpdateBalance`. Speech is off (the audio call after
    /// the slew is skipped inside `0x2a1560`); the wipeout, audio and curve
    /// paths are all reached by the random slew and clock values.
    #[test]
    fn rider_board_press_sub_update_balance_matches() {
        check(
            ee::s00::f_00130dd0__generated,
            rider_board_press_sub_update_balance,
            400,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0013_0dd0, 3, &[(8, 0)]);
                oc675_balance_world(c, &mut r, seed);
            },
        );
    }

    /// `RiderBoardPressSub_UpdatePress`: the switch, jump, spin and release
    /// checks are fakes with forced results (stage = seed % 6); the balance
    /// update runs for real on the balance fixture.
    #[test]
    fn rider_board_press_sub_update_press_matches() {
        check(
            ee::s00::f_001303e0__generated,
            rider_board_press_sub_update_press,
            288,
            &|c, seed| {
                let stage = seed % 6;
                let results = [
                    (3, (stage == 0) as u32),
                    (4, (stage == 1) as u32),
                    (5, (stage == 2) as u32),
                    (6, (stage >= 4) as u32),
                    (8, 0),
                ];
                let mut r = oc675_begin(c, seed, 0x0013_03e0, 9, &results);
                oc675_balance_world(c, &mut r, seed / 6);
            },
        );
    }

    /// `MissionRider_dtor`: flag 0 copies the vtables to the stack and
    /// relocates them; flags 2 and 1 add the rider destructor and the delete.
    #[test]
    fn mission_rider_dtor_matches() {
        check(
            ee::s01::f_001446f8__generated,
            mission_rider_dtor,
            160,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_46f8, 3, &[]);
                elf_image(c);
                fill(c, &mut r, DATA, 0x40);
                c.bus.write32(DATA + 0x18, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 0x400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    /// `BENewPlayerInt_GetDisplayName`: network names (host or guest), a
    /// filled name, and the empty name in each of the four languages.
    #[test]
    fn be_new_player_int_get_display_name_matches() {
        check(
            ee::s01::f_00146f88__generated,
            be_new_player_int_get_display_name,
            320,
            &|c, seed| {
                let slot = seed / 160 % 3 + seed % 2;
                let results = [(14, slot)];
                let mut r = oc675_begin(c, seed, 0x0014_6f88, 6, &results);
                let idx = [0u32, 1, 2, 5][(seed % 4) as usize];
                let net = (seed / 4) % 2;
                let host = (seed / 8) % 2;
                let named = (seed / 16) % 2;
                let lang = (seed / 32) % 5;
                c.bus.write32(0x0053_4b30, net);
                c.bus.write32(0x0053_4b38, host);
                c.bus
                    .write8(0x0053_4fe0 + slot * 0x1c, if named == 1 { b'x' } else { 0 });
                c.bus
                    .write32(0x0053_5610, (lang << 22) | (r.next_u32() & 0x003f_ffff));
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }

    /// Random bytes over the saved databases (race DB, player DB, profile
    /// library, rules) with the two character ids kept in range.
    fn oc675_fill_databases(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, 0x004a_0000, 0xc000);
        fill(c, r, 0x0053_0000, 0x4000);
        c.bus.write8(0x0053_5b20 + 0x11, (seed % 4) as u8);
        let second = [0xffu8, 0, 1, 3][(seed / 4 % 4) as usize];
        c.bus.write8(0x0053_5b20 + 0x2d, second);
    }

    /// `BELibrary_SerializeSave`: the destination's alignment decides between
    /// the doubleword path and the unaligned `ldl/ldr` path of each block;
    /// rider 1 present or absent.
    #[test]
    fn be_library_serialize_save_matches() {
        check(
            ee::s01::f_0014a188__generated,
            be_library_serialize_save,
            96,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_a188, 0, &[]);
                oc675_fill_databases(c, &mut r, seed);
                let off = [0u32, 4, 8, 1, 2, 16][(seed / 16 % 6) as usize];
                fill(c, &mut r, DATA + 0x10000, 0x6000);
                c.g[A0][0] = (DATA + 0x10000 + off) as u64;
            },
        );
    }

    /// `BENetworkInt_PackPlayerData(self, player, out)`.
    #[test]
    fn be_network_int_pack_player_data_matches() {
        check(
            ee::s01::f_0014e2c8__generated,
            be_network_int_pack_player_data,
            48,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_e2c8, 0, &[]);
                oc675_fill_databases(c, &mut r, seed);
                let off = [0u32, 4, 8, 1][(seed / 16 % 4) as usize];
                // The player's profile blocks live at ch * 0xf88 + player * 0x9b50.
                let player = seed % 6;
                // The character id of that player (signed byte at +0x11 of its record).
                c.bus
                    .write8(0x0053_5b20 + 0x1c * player + 0x11, (seed / 6 % 5) as u8);
                fill(c, &mut r, DATA + 0x10000, 0x600);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = player as u64;
                c.g[A2][0] = (DATA + 0x10000 + off) as u64;
            },
        );
    }

    /// `BENetworkInt_UnpackPlayerData(self, player, in)`: host and guest, the
    /// interface `discard` calls (a fake vtable method), and unwinds.
    #[test]
    fn be_network_int_unpack_player_data_matches() {
        check(
            ee::s01::f_0014e5c8__generated,
            be_network_int_unpack_player_data,
            64,
            &|c, seed| {
                let iface = DATA + 0x8000;
                let vt = DATA + 0x8100;
                let results = [(13, iface)];
                let mut r = oc675_begin(c, seed, 0x0014_e5c8, 11, &results);
                oc675_fill_databases(c, &mut r, seed);
                c.bus.write32(iface + 0xc, vt);
                c.bus.write16(vt + 0x10, 0);
                c.bus.write32(vt + 0x14, FAKE + 0x10);
                // `bIsHost` of the network options.
                c.bus.write32(0x0053_4b30 + 8, seed / 16 % 2);
                let off = [0u32, 4][(seed / 32 % 2) as usize];
                fill(c, &mut r, DATA + 0x10000, 0x600);
                // The packet's character id (signed byte at +0x110).
                c.bus.write8(DATA + 0x10000 + off + 0x110, (seed % 5) as u8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed / 5 % 6) as u64;
                c.g[A2][0] = (DATA + 0x10000 + off) as u64;
            },
        );
    }

    /// `BEBoltInt_CollectEquippedTextureNames`: the bolt database fixture of
    /// the file-name collector, with the texture-name callee faked.
    #[test]
    fn bolt_int_collect_equipped_texture_names_matches() {
        check(
            ee::s01::f_0014b7a8__generated,
            bolt_int_collect_equipped_texture_names,
            200,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_b7a8, 0, &[]);
                let profile = seed % 3;
                let ch = seed / 3 % 4;
                let count = seed / 12 % 4;
                crate::game::be::belibrary::tests::oc628_bolt_fixture(
                    c, &mut r, seed, profile, ch, count,
                );
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = (DATA + 0x6000) as u64;
            },
        );
    }

    /// `Bolts_CollectByGroup`: nodes of three groups with parent keys and
    /// 0x20 flags, depth 0..2 and the `all` flag.
    #[test]
    fn bolts_collect_by_group_matches() {
        check(
            ee::s01::f_0014d7e8__generated,
            bolts_collect_by_group,
            300,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_d7e8, 0, &[]);
                let group = seed % 3;
                let key = [u64::MAX, 7, 0x41][(seed / 3 % 3) as usize];
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = group as u64;
                c.g[A2][0] = key;
                c.g[A3][0] = (DATA + 0x6000) as u64;
                c.g[T0][0] = (seed / 9 % 3) as u64;
                c.g[T1][0] = (seed / 27 % 2) as u64;
                c.bus.write32(DATA + 4, DATA + 0x2000);
                for g in 0..3u32 {
                    c.bus.write32(DATA + g * 4 + 0x2c, 3);
                    c.bus.write32(DATA + g * 4 + 0xa4, 0);
                }
                for i in 0..3u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for w in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * w, r.next_u32());
                    }
                    c.bus
                        .write16(e + 4, [7u16, 0x41, 3][(r.next_u32() % 3) as usize]);
                    c.bus
                        .write16(e + 8, [0xffffu16, 7, 0x41][(r.next_u32() % 3) as usize]);
                    let flag = if r.next_u32() % 2 == 0 { 0x20 } else { 0 };
                    c.bus.write32(e + 0x34, (r.next_u32() & !0x20) | flag);
                }
            },
        );
    }
}
