//! Staging module oc434 (wave wave221): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc434`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_b310,
        path_data_read_track_paths,
        "PathData_ReadTrackPaths",
    ),
    (
        0x0011_9608,
        rider_metrix_award_bonus_points,
        "RiderMetrix_AwardBonusPoints",
    ),
    (
        0x0023_a668,
        gm_handler1_setup_rider_slots,
        "GMHandler1_SetupRiderSlots",
    ),
    (0x0022_9180, crowd_man2d_ctor, "CrowdMan2d_ctor"),
    (0x003a_7e98, world_cache_free, "cWorldCache_Free"),
    (
        0x0027_8210,
        script_engine_load_master,
        "ScriptEngine_LoadMaster",
    ),
    (0x0026_b410, path_data_read_ai_paths, "PathData_ReadAIPaths"),
    (
        0x0028_0520,
        rider_script_obj_get_anim_bank_entry,
        "RiderScriptObj_GetAnimBankEntry",
    ),
    (
        0x0028_1f30,
        script_engine_create_script_database,
        "ScriptEngine_CreateScriptDatabase",
    ),
    (0x0035_1030, instances_man_ctor, "InstancesMan_ctor"),
    (
        0x0015_8e30,
        be_reward_int_begin_reward_tracking,
        "BERewardInt_BeginRewardTracking",
    ),
    (
        0x0027_5a20,
        data_instance_man_remove_data,
        "DataInstanceMan_RemoveData",
    ),
    (
        0x0012_dcb0,
        rider_wipeout_sub_switch_to_anim_set_b,
        "RiderWipeoutSub_SwitchToAnimSetB",
    ),
    (
        0x003a_dcb0,
        w_script_cache_get_globals,
        "tWScriptCache_GetGlobals",
    ),
    (
        0x003a_6bd8,
        world_get_section_shutdown_funcs,
        "World_GetSectionShutdownFuncs",
    ),
    (
        0x0027_53b8,
        script_anim_bank_man_ctor,
        "ScriptAnimBankMan_ctor",
    ),
    (
        0x0032_0db0,
        input_map_create_parser,
        "InputMap_CreateParser",
    ),
    (0x003a_6d50, c_stream_man_ctor, "cStreamMan_ctor"),
    (0x0026_efb8, replay_ctor, "Replay_ctor"),
    (
        0x0027_9ef8,
        script_engine_find_sound_bank,
        "ScriptEngine_FindSoundBank",
    ),
];

/// `int PathData_ReadTrackPaths(PathData* self, char* data)` (0x26b310): reads the
/// track path table from a path file: the count (kept at `+0x10`), a `count * 0x3c`
/// byte array ("TrackPaths", pointer at `+0x14`) whose entries get their vtable
/// (`+0x34`) and are then read with `Path_Read`. Returns the bytes consumed.
pub fn path_data_read_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        'b_26b3ac: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.g[S3][0] = c.g[A0][0];
            c.g[A0][0] = c.g[SP][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S4][0] = c.g[S1][0];
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b344)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = 0x3c;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0u64;
            c.mult(0, 0, 16, 4);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[S0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1438));
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0026b370)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[A1][0];
            if t {
                break 'b_26b3ac;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1448));
                'l_26b390: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b390);
                        }
                        continue 'l_26b390;
                    } else {
                        break 'b_26b3ac;
                    }
                }
            }
        }
        'b_26b3e8: {
            c.cycles += 4;
            c.sw(c.g[S2][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_26b3e8;
            } else {
                c.cycles += 1;
                c.g[S2][0] = 0u64;
                'l_26b3c0: loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    Callee {
                        addr: 0x0026a180,
                        f: ee::s07::Path_Read,
                    }
                    .call(c, 0x0026b3d4)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b3c0);
                        }
                        continue 'l_26b3c0;
                    } else {
                        break 'b_26b3e8;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S4][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `float RiderMetrix_AwardBonusPoints(RiderMetrix* self, int pts)` (0x119608): turns
/// `pts` into points (`pts * *(float*)(self + 0x1c4) + 0.5`), then either shows the
/// two HUD number effects (35 and 0x18) or, in career mode (event kind 4, game type
/// 0), awards points / 500 (at most 20) as cash. Adds the points to `self + 0x198`
/// and returns 0.0.
pub fn rider_metrix_award_bonus_points(c: &mut Cpu) {
    guest(c, |c| {
        'b_119674: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.mtc1(5, 0);
            c.cvt_s_w(0, 0);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 2);
            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x1c4));
            c.mul_s(0, 0, 1);
            c.add_s(0, 0, 2);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            call_ptr(c, 0x0014dc80, 0x00119648)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00119654)?;
            c.g[V1][0] = 0x530000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[A0][0] = 0x4;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = 0u64;
            if t {
                break 'b_119674;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
                break 'b_119674;
            }
        }
        'b_1196e4: {
            'b_1196b8: {
                c.cycles += 2;
                let t = c.g[A1][0] != 0u64;
                c.g[V0][0] = 0x1f4;
                if t {
                    break 'b_1196b8;
                } else {
                    c.cycles += 15;
                    c.g[AT][0] = 0x40200000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x23;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x00117b88,
                        f: ee::s00::RiderMetrix_ShowHUDNumber,
                    }
                    .call(c, 0x00119698)?;
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7a20));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x18;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x00117b88,
                        f: ee::s00::RiderMetrix_ShowHUDNumber,
                    }
                    .call(c, 0x001196b0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x198));
                    if t {
                        break 'b_1196e4;
                    } else {
                        break 'b_1196b8;
                    }
                }
            }
            'b_1196c8: {
                c.cycles += 4;
                c.g[V1][0] = 0x14;
                c.div(0, 16, 2);
                if c.g[V0][0] == 0u64 {
                    break 'b_1196c8;
                } else {
                    break 'b_1196c8;
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.lo[0];
            c.g[V0][0] = ((c.g[A2][0] as i64) < 21i64) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            Callee {
                addr: 0x00119ef8,
                f: ee::s00::RiderMetrix_AwardPoints,
            }
            .call(c, 0x001196e0)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x198));
            break 'b_1196e4;
        }
        c.cycles += 8;
        c.mtc1(0, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x198), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void GMHandler1_SetupRiderSlots(GMHandler1* self)` (0x23a668): fills the game mode
/// manager's six character slots (`+0x18`, ids from the heat's row at `self + 0x10 +
/// 0x18 * heat`) and cheat character slots (`+0x40`): ids of 10 or more become the
/// human rider's character, the cheat slot keeping the id; the human's own slot gets
/// the alternate character.
pub fn gm_handler1_setup_rider_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0023a688)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023a694)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00146e98, 0x0023a6a4)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00147398, 0x0023a6b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x00147410, 0x0023a6c4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[T1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x18;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        c.g[T0][0] = 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0u64;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0u64;
        }
        c.mult(0, 2, 2, 5);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        'l_23a700: loop {
            'b_23a718: {
                c.cycles += 4;
                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw(c.g[A0][0] as u32, 0u64);
                    break 'b_23a718;
                } else {
                    c.cycles += 2;
                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.g[S3][0];
                    break 'b_23a718;
                }
            }
            'b_23a72c: {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let t = c.g[T0][0] != c.g[S2][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                if t {
                    break 'b_23a72c;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                    break 'b_23a72c;
                }
            }
            c.cycles += 6;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0023a700);
                }
                continue 'l_23a700;
            } else {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `CrowdMan2d* CrowdMan2d_ctor(CrowdMan2d* self)` (0x229180): clears the 128 + 40
/// id words (`-1`) of the crowd tables, publishes `self` in the global at `gp - 0x6f0`,
/// builds the 2D crowd renderer and its animation set, sets up the first animation
/// record (`0x536690`) and resets the manager. Returns `self`.
pub fn crowd_man2d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = 0xffffffffffff0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x7f;
        c.g[S1][0] = 0x530000;
        c.g[A1][0] = 0xffffffffffffffff;
        'l_2291b0: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002291b0);
                }
                continue 'l_2291b0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x7f;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = 0x27;
                c.g[V1][0] = 0xffffffffffffffff;
                'l_2291e0: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A2][0] = 0x20000000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002291e0);
                        }
                        continue 'l_2291e0;
                    } else {
                        c.cycles += 31;
                        c.g[A1][0] = 0x480000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4ce0));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x10;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6f0), c.g[S0][0]);
                        call_ptr(c, 0x00317d70, 0x00229214)?;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x002db6d8, 0x0022921c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        call_ptr(c, 0x002db980, 0x00229228)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6690));
                        c.g[A2][0] = 0xffffffffffffffff;
                        c.g[A3][0] = 0x20000;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S0][0];
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x6), c.g[A2][0]);
                        c.sh((c.g[S1][0] as u32).wrapping_add(0x6690), c.g[A1][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                        Callee {
                            addr: 0x002292e0,
                            f: ee::s05::CrowdMan2d_Reset,
                        }
                        .call(c, 0x00229260)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void cWorldCache_Free(cWorldCache* self)` (0x3a7e98): frees the world cache: releases
/// all hulls, deletes the chunk resolver (`+0x3e8`, virtual destructor with flag 3) and
/// the shared table (`+0x4`), deactivates every loaded section and frees the section
/// table (`+0x8`), the data header (`+0x0`) and the world memory manager (`+0xc`).
pub fn world_cache_free(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a7ed0: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            Callee {
                addr: 0x003a8330,
                f: ee::s13::cWorldCache_ReleaseAllHulls,
            }
            .call(c, 0x003a7eb0)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3e8));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_3a7ed0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x003a7ed0)?;
                break 'b_3a7ed0;
            }
        }
        'b_3a7ee8: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            if c.g[A0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                break 'b_3a7ee8;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                Callee {
                    addr: 0x003a7d80,
                    f: ee::s13::SectionTable_dtor,
                }
                .call(c, 0x003a7ee4)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                break 'b_3a7ee8;
            }
        }
        'b_3a7f54: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                break 'b_3a7f54;
            } else {
                'b_3a7f3c: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_3a7f3c;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                        'l_3a7f08: loop {
                            'b_3a7f28: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x003a9890,
                                    f: ee::s13::cWorldCache_StreamerIsSectionUnloaded,
                                }
                                .call(c, 0x003a7f10)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_3a7f28;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x003a8230,
                                        f: ee::s13::cWorldCache_FreeSectionTable,
                                    }
                                    .call(c, 0x003a7f24)?;
                                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_3a7f28;
                                }
                            }
                            c.cycles += 5;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a7f08);
                                }
                                continue 'l_3a7f08;
                            } else {
                                break 'b_3a7f3c;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if c.g[A0][0] == 0u64 {
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    break 'b_3a7f54;
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x00317e98, 0x003a7f50)?;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    break 'b_3a7f54;
                }
            }
        }
        'b_3a7f68: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3a7f68;
            } else {
                c.cycles += 3;
                call_ptr(c, 0x00317e98, 0x003a7f64)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3a7f68;
            }
        }
        'b_3a7f7c: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_3a7f7c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                Callee {
                    addr: 0x003a7aa8,
                    f: ee::s13::WorldMemman_dtor,
                }
                .call(c, 0x003a7f78)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_3a7f7c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void ScriptEngine_LoadMaster(ScriptEngine* self)` (0x278210): loads the script master
/// table (file names at `gp + 0x3a4` and `gp + 0x3a8`): `+0x52c` header, `+0x530`
/// script entries (0xc bytes each, offset at +8), `+0x534` the debug base. Offsets are
/// rebased on the debug file, or cleared without one. Then sets up the BX engine
/// (`ScriptEngine_CreateScriptDatabase(self, count, 10, 0x4000)`).
pub fn script_engine_load_master(c: &mut Cpu) {
    guest(c, |c| {
        'b_2782d8: {
            'b_2782d4: {
                'b_278298: {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A1][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x3a4));
                    call_ptr(c, 0x003e1908, 0x0027822c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x3a8));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x52c), c.g[V0][0]);
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x530), c.g[V1][0]);
                    call_ptr(c, 0x003e1908, 0x00278244)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x534), c.g[V0][0]);
                    if t {
                        break 'b_278298;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2782d4;
                        } else {
                            c.cycles += 1;
                            c.g[A2][0] = 0u64;
                            'l_278260: loop {
                                c.cycles += 12;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x534));
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00278260);
                                    }
                                    continue 'l_278260;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                                    if t {
                                        break 'b_2782d8;
                                    } else {
                                        break 'b_278298;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_2782d4;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0u64;
                    'l_2782b0: loop {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002782b0);
                            }
                            continue 'l_2782b0;
                        } else {
                            break 'b_2782d4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
            break 'b_2782d8;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x554), 0u64);
        c.g[A2][0] = 0xa;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x620), 0u64);
        c.g[A3][0] = 0x4000;
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x00281f30,
            f: ee::s07::ScriptEngine_CreateScriptDatabase,
        }
        .call(c, 0x002782f4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int PathData_ReadAIPaths(PathData* self, char* data)` (0x26b410): the same for the
/// AI path table: the count (kept at `+0x8`), a `count * 0x40` byte array ("AIPaths",
/// pointer at `+0xc`), each entry with its vtable at `+0x34`, read with `Path_Read`.
/// Returns the bytes consumed.
pub fn path_data_read_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        'b_26b4a4: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.g[SP][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S4][0] = c.g[S1][0];
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b444)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3408));
            c.g[A2][0] = 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0026b468)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[A1][0];
            if t {
                break 'b_26b4a4;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1478));
                'l_26b488: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b488);
                        }
                        continue 'l_26b488;
                    } else {
                        break 'b_26b4a4;
                    }
                }
            }
        }
        'b_26b4e0: {
            c.cycles += 4;
            c.sw(c.g[S3][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_26b4e0;
            } else {
                c.cycles += 1;
                'l_26b4b8: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x0026a180,
                        f: ee::s07::Path_Read,
                    }
                    .call(c, 0x0026b4d0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b4b8);
                        }
                        continue 'l_26b4b8;
                    } else {
                        break 'b_26b4e0;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S4][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void RiderScriptObj_GetAnimBankEntry(RiderScriptObj* self, int index, int* entry, int* count)`
/// (0x280520): `*count` = field `+4` of the bank at `self + 0x1c`, `*entry` = its entry
/// `index` (array at `+0x10`).
pub fn rider_script_obj_get_anim_bank_entry(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A2][0] as u32, c.g[V0][0]);
    c.pc = t;
}

/// `void ScriptEngine_CreateScriptDatabase(ScriptEngine* self, int count, int maxAnimPlayers, int size)`
/// (0x281f30): allocates the InSeq heap (`size` bytes, `+0x2bc`) and the InSeq (`+0x2b8`),
/// stores `count` (`+0x2b4`) and allocates and clears the `count` 8-byte script slots
/// (`+0x2b0`).
pub fn script_engine_create_script_database(c: &mut Cpu) {
    guest(c, |c| {
        'b_282004: {
            c.cycles += 41;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A3][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A2][0];
            c.g[S3][0] = c.g[A0][0];
            c.g[A1][0] = 0x480000;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f48));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x00281f70)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = 0x20;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34f0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x2bc), c.g[V0][0]);
            call_ptr(c, 0x00317d70, 0x00281f8c)?;
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2bc));
            c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2ac));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S2][0];
            Callee {
                addr: 0x00272788,
                f: ee::s07::InSeq_ctor,
            }
            .call(c, 0x00281fa4)?;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x2b4), c.g[S1][0]);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f58));
            c.sw((c.g[S3][0] as u32).wrapping_add(0x2b8), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[S1][0] as u32) << 3);
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x00281fc4)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2b4));
            c.g[A1][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x2b0), c.g[V0][0]);
            if t {
                break 'b_282004;
            } else {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2b0));
                'l_281fd8: loop {
                    c.cycles += 11;
                    c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2b0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sh((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2b4));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2b0));
                        if event_due(c) {
                            deliver_event_here(c, 0x00281fd8);
                        }
                        continue 'l_281fd8;
                    } else {
                        break 'b_282004;
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `InstancesMan* InstancesMan_ctor(InstancesMan* self, int sectionId, int count)`
/// (0x351030): allocates `count` 0xb0-byte instance records, resets their keyword fields
/// (`+0x78` -1, `+0x7c` 0, `+0x7d` 0xff, `+0x7e` -1), and chains them into the free list
/// (`self + 0x8`) through their first word. Returns `self`.
pub fn instances_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_3510b4: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A3][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A2][0];
            c.g[A0][0] = 0xb0;
            c.mult(0, 0, 16, 4);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = 0x20000000;
            c.sw(c.g[S1][0] as u32, c.g[A1][0]);
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = c.lo[0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1740));
            call_ptr(c, 0x00317e30, 0x00351078)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[V0][0];
            if t {
                break 'b_3510b4;
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[T0][0] = 0xff;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[A3][0] = 0xffffffffffffffff;
                c.g[A2][0] = 0xffffffffffffffff;
                'l_351098: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x7d), c.g[T0][0]);
                    c.sh((c.g[V1][0] as u32).wrapping_add(0x7e), c.g[A3][0]);
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xb0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00351098);
                        }
                        continue 'l_351098;
                    } else {
                        break 'b_3510b4;
                    }
                }
            }
        }
        'b_351100: {
            c.cycles += 5;
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_351100;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0u64;
                'l_3510d0: loop {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003510d0);
                        }
                        continue 'l_3510d0;
                    } else {
                        break 'b_351100;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int BERewardInt_BeginRewardTracking(BERewardInt* self)` (0x158e30): clears the new
/// rewards, looks up the first rider's character and stores the next earnings threshold
/// (`gp - 0x1e90`), the three collectible thresholds (`0x440f48`) and the three mission
/// thresholds (`0x440f58`), then resets the reward interface's counters
/// (`+0x10` = 0, `+0x14` = -1). Leaves v0 = -1.
pub fn be_reward_int_begin_reward_tracking(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = 0x4c0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ef0));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = 0u64;
        Callee {
            addr: 0x0015a510,
            f: ee::s01::BERewardInt_ClearNewRewards,
        }
        .call(c, 0x00158e5c)?;
        c.g[A0][0] = 0u64;
        Callee {
            addr: 0x0014a130,
            f: ee::s01::BE_FindRiderByProfile,
        }
        .call(c, 0x00158e64)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0014a080,
            f: ee::s01::BE_GetRiderCharacter,
        }
        .call(c, 0x00158e6c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00158be0,
            f: ee::s01::BERewardInt_GetNextPointsThreshold,
        }
        .call(c, 0x00158e80)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e90), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf48));
        'l_158e90: loop {
            c.cycles += 10;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.g[S2][0];
            Callee {
                addr: 0x00158c80,
                f: ee::s01::BERewardInt_GetNextCollectibleThreshold,
            }
            .call(c, 0x00158ea4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00158e90);
                }
                continue 'l_158e90;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[S0][0] = 0u64;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf58));
                'l_158ec8: loop {
                    c.cycles += 10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00158d58,
                        f: ee::s01::BERewardInt_GetNextMissionThreshold,
                    }
                    .call(c, 0x00158edc)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00158ec8);
                        }
                        continue 'l_158ec8;
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `int DataInstanceMan_RemoveData(DataInstanceMan* self, Node* entry)` (0x275a20): removes
/// an entry and its chain of linked nodes (`+0x14`): each is passed to the manager's
/// virtual `remove` (vtable slot at `+0x1c` or `+0x2c` of the manager's vtable) and
/// returned to the free list (`DataInstanceMan_FreeInfo`). Returns the key of the
/// last node (`+0x8`).
pub fn data_instance_man_remove_data(c: &mut Cpu) {
    guest(c, |c| {
        'b_275adc: {
            'b_275ab4: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                let t = c.g[A1][0] == 0u64;
                c.g[S2][0] = c.g[A0][0];
                if t {
                    break 'b_275ab4;
                } else {
                    c.cycles += 2;
                    c.g[S1][0] = c.g[A1][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    'l_275a50: loop {
                        c.cycles += 15;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A3][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V1][0] as u32;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        call_ptr(c, t, 0x00275a74)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x00275cd0,
                            f: ee::s07::DataInstanceMan_FreeInfo,
                        }
                        .call(c, 0x00275a80)?;
                        c.g[S1][0] = c.g[S0][0];
                        let t = c.g[S1][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00275a50);
                            }
                            continue 'l_275a50;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                            let t = c.g[V1][0] as u32;
                            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                            call_ptr(c, t, 0x00275aac)?;
                            let t = true;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_275adc;
                            } else {
                                break 'b_275ab4;
                            }
                        }
                    }
                }
            }
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
            let t = c.g[V1][0] as u32;
            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            call_ptr(c, t, 0x00275ad8)?;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            break 'b_275adc;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        Callee {
            addr: 0x00275cd0,
            f: ee::s07::DataInstanceMan_FreeInfo,
        }
        .call(c, 0x00275ae8)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int tWScriptCache_GetGlobals(tWScriptCacheEntry* self)` (0x3adcb0): the entry's global
/// script pointer (`+0x14` of its data at `+0x8`) when the entry has type 3 and data, else 0.
pub fn w_script_cache_get_globals(c: &mut Cpu) {
    'b_3adcd4: {
        c.cycles += 4;
        c.g[V0][0] = c.lh(c.g[A0][0] as u32);
        c.g[V1][0] = 0x3;
        let t = c.g[V0][0] != c.g[V1][0];
        if t {
            break 'b_3adcd4;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;
            if t {
                break 'b_3adcd4;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                c.pc = t;
                return;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
}

/// `int World_GetSectionShutdownFuncs(World* self, int i)` (0x3a6bd8): the global scripts of
/// the world script cache's entry `i` (entries 0xc bytes each, at `+4` of the cache at
/// `self + 4`).
pub fn world_get_section_shutdown_funcs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0xc;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mult(0, 5, 5, 2);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        Callee {
            addr: 0x003adcb0,
            f: ee::s13::tWScriptCache_GetGlobals,
        }
        .call(c, 0x003a6bf8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptAnimBankMan* ScriptAnimBankMan_ctor(ScriptAnimBankMan* self, int flags, int count, int base)`
/// (0x2753b8): stores `flags` (`+0`), `count` (`+4`) and the vtable (`0x4825d8`, `+0xc`),
/// allocates `count` 0x14-byte bank records (`+8`) and initialises them (id -1, index
/// `base + i`, the rest 0). Returns `self`.
pub fn script_anim_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_27547c: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[T0][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x14;
            c.g[V0][0] = 0x480000;
            c.mult(0, 0, 8, 4);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x25d8));
            c.g[S0][0] = c.g[A3][0];
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = c.g[V1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a80));
            c.sw(c.g[S1][0] as u32, c.g[V1][0]);
            c.g[A0][0] = c.lo[0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A3][0] = 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
            call_ptr(c, 0x00317e30, 0x0027540c)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            if t {
                break 'b_27547c;
            } else {
                c.cycles += 3;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A0][0] = 0u64;
                'l_275428: loop {
                    c.cycles += 21;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00275428);
                        }
                        continue 'l_275428;
                    } else {
                        break 'b_27547c;
                    }
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void InputMap_CreateParser(InputMap* self, char* mem, int len)` (0x320db0): allocates the
/// 0x109ec-byte input parser (256-aligned) and constructs it over `mem` / `len`; the
/// parser goes to `self + 0x8`.
pub fn input_map_create_parser(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[A3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x22f0));
        c.g[A0][0] = c.g[A0][0] | 0x9ec;
        c.g[A2][0] = 0x100;
        Callee {
            addr: 0x00317d70,
            f: ee::s10::EA_MemAllocNamed,
        }
        .call(c, 0x00320dec)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x00321428,
            f: ee::s10::InputParser_ctor,
        }
        .call(c, 0x00320dfc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cStreamMan* cStreamMan_ctor(cStreamMan* self)` (0x3a6d50): sets the BX chunk resource id
/// (`+0xbc`) to -1, sizes and allocates the stream buffer (`+0x84`, at most 0x80000 bytes),
/// creates the stream (`+0x88`), clears `+0x9c`, allocates the decompress buffer (`+0x8c`,
/// 0x14000 bytes) and resets the manager. Returns `self`.
pub fn c_stream_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 52;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0xffffffffffff0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x2;
        c.g[A1][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x003e06b0, 0x003a6d80)?;
        c.g[V1][0] = 0x10000;
        c.g[A1][0] = 0x50000;
        c.g[V1][0] = c.g[V1][0] | 0x8000;
        c.g[A0][0] = 0x80000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A1][0] = 0x490000;
        if c.g[A0][0] == 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d90));
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x003a6dbc)?;
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = 0x2;
        c.g[A2][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x84), c.g[V0][0]);
        call_ptr(c, 0x003e06d8, 0x003a6dd8)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x9c), 0u64);
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4da8));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x88), c.g[V0][0]);
        c.g[A0][0] = c.g[A0][0] | 0x4000;
        call_ptr(c, 0x00317e30, 0x003a6dfc)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8c), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x003a6ed8,
            f: ee::s13::cStreamMan_Reset,
        }
        .call(c, 0x003a6e08)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `Replay* Replay_ctor(Replay* self)` (0x26efb8): constructs the 16 replay frames (0x38
/// bytes from `+0x2c`), the two snapshot lists (`+0x3b0`, `+0x3bc`) and the two highlight
/// slots (0xb4 bytes from `+0x494`), enters replay state 15 and clears the two cache
/// words at `+0x48c`. Returns `self`.
pub fn replay_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0xf;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_26efe8: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x0026d5e8,
                f: ee::s07::WorldStateStream_ctor,
            }
            .call(c, 0x0026eff4)?;
            let t = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0026efe8);
                }
                continue 'l_26efe8;
            } else {
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3b0));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x494));
                Callee {
                    addr: 0x0026e5a8,
                    f: ee::s07::SnapshotList_ctor,
                }
                .call(c, 0x0026f008)?;
                c.g[S1][0] = 0x1;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3bc));
                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48c));
                Callee {
                    addr: 0x0026e5a8,
                    f: ee::s07::SnapshotList_ctor,
                }
                .call(c, 0x0026f018)?;
                c.g[S3][0] = 0xffffffffffffffff;
                'l_26f020: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00272288,
                        f: ee::s07::Replay_HighlightSlotInit,
                    }
                    .call(c, 0x0026f02c)?;
                    let t = c.g[S1][0] != c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026f020);
                        }
                        continue 'l_26f020;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0xf;
                        Callee {
                            addr: 0x0026f4a0,
                            f: ee::s07::Replay_SetState,
                        }
                        .call(c, 0x0026f040)?;
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = 0x1;
                        'l_26f048: loop {
                            c.cycles += 7;
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0026f048);
                                }
                                continue 'l_26f048;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = c.g[S2][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `void ScriptEngine_FindSoundBank(ScriptEngine* self)` (0x279ef8): forwards to
/// `ScriptSoundBankMan_FindBank(self + 0xa38)`.
pub fn script_engine_find_sound_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa38));
        Callee {
            addr: 0x002836d8,
            f: ee::s07::ScriptSoundBankMan_FindBank,
        }
        .call(c, 0x00279f08)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

#[cfg(not(feature = "ee-unwind"))]
pub(crate) fn rider_wipeout_sub_switch_to_anim_set_b_resume(c: &mut Cpu) {
    ee::s00::f_0012dcb0__entries(c)
}

#[cfg(feature = "ee-unwind")]
pub(crate) fn rider_wipeout_sub_switch_to_anim_set_b_resume(c: &mut Cpu) {
    ee::s00::f_0012dcb0__generated(c)
}

/// `RiderWipeoutSub` set-B switch: the original's jump table lives at `0x458030` (index
/// `cur - 362`, 26 entries); its targets `0x12dd10 + 8 * k` select set-B animation `0x175 + k`.
pub(crate) const WIPEOUT_SET_B_TABLE: u32 = 0x0046_0000 - 0x7fd0;

/// `void RiderWipeoutSub_SwitchToAnimSetB(RiderWipeoutSub* self)` (0x12dcb0): maps the
/// rider's current tumble animation (layer 2) to its set-B twin (362..371 to 373..382, ...,
/// 383 when the rider is in root pose) and plays it (`AnimController_PlayAnim(anim, next, 0,
/// -1.0)`), then updates the animation speed. The map is the original's jump table at
/// `0x458030 + 4 * (cur - 362)`; each target is one set-B animation.
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn rider_wipeout_sub_switch_to_anim_set_b(c: &mut Cpu) {
    if c.pc != 0x0012_dcb0 {
        return rider_wipeout_sub_switch_to_anim_set_b_resume(c);
    }
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x0031_2aa0, 0x0012_dcd0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x150));
        c.g[V0][0] ^= 0x1;
        let root_pose = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[A1][0];
        if root_pose {
            // 'b_12dd60: the rider is in root pose: animation 383.
            c.cycles += 1;
            c.g[A1][0] = 0x17f;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a));
            c.g[V0][0] = (c.g[V1][0] < 0x1a) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0x46_0000;
            if !out_of_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7fd0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    // Set-B animations 0x175..=0x17e.
                    0x0012_dd10 | 0x0012_dd18 | 0x0012_dd20 | 0x0012_dd28 | 0x0012_dd30
                    | 0x0012_dd38 | 0x0012_dd40 | 0x0012_dd48 | 0x0012_dd50 | 0x0012_dd58 => {
                        c.cycles += 2;
                        c.g[A1][0] = 0x175 + ((target - 0x0012_dd10) / 8) as u64;
                    }
                    // Already a set-B animation: keep it.
                    0x0012_dd64 => {}
                    _ => {
                        c.jump_out(target, 0x0012_dcb0, 0x0012_dd98);
                        return Ok(());
                    }
                }
            }
        }
        c.cycles += 2;
        let unchanged = c.g[A1][0] == c.g[A2][0];
        c.g[A2][0] = 0;
        if !unchanged {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
            call_ptr(c, 0x0031_28e8, 0x0012_dd80)?;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0012_e528,
                f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed,
            }
            .call(c, 0x0012_dd88)?;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Control words of the callee fakes, in the test machine's own RAM: the gate
    /// (a fake does nothing unless the fixture wrote the magic), the bump allocator.
    pub(crate) const OC434_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC434_MAGIC: u32 = 0x0c43_4a11;
    /// Next address the allocator fakes hand out (word at `OC434_CTRL + 8`, 0: the start).
    pub(crate) const OC434_HEAP: u32 = DATA + 0x30_0000;
    /// Bit mask of the sections `cWorldCache_StreamerIsSectionUnloaded` calls unloaded.
    pub(crate) const OC434_UNLOADED: u32 = OC434_CTRL + 0x10;

    /// Post-call hooks of the fakes: `(machine, return address, a0, a1)` before the
    /// fake clobbered the argument registers.
    fn oc434_plain(_c: &mut Cpu, _ra: u32, _a0: u32, _a1: u32) {}

    /// An allocator: a fresh zeroed block from the bump heap (a0 = size).
    fn oc434_bump(c: &mut Cpu, ra: u32, size: u32, _a1: u32) {
        if c.pc != ra {
            return;
        }
        let mut p = c.bus.read32(OC434_CTRL + 8);
        if p == 0 {
            p = OC434_HEAP;
        }
        c.g[V0][0] = p as u64;
        let size = (size.min(0x10_0000) + 0xff) & !0xff;
        c.bus.write32(OC434_CTRL + 8, p + size.max(0x100));
    }

    /// A call that fails (returns 0).
    fn oc434_zero(c: &mut Cpu, ra: u32, _a0: u32, _a1: u32) {
        if c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    /// `StreamerIsSectionUnloaded(table, i)`: bit `i` of the fixture's mask.
    fn oc434_unloaded(c: &mut Cpu, ra: u32, _a0: u32, section: u32) {
        if c.pc == ra {
            c.g[V0][0] = ((c.bus.read32(OC434_UNLOADED) >> (section & 31)) & 1) as u64;
        }
    }

    /// Gated fakes in the override slots of callees that already have a hand port
    /// (each logs its call like `difftest::fakes::fake`; the gate is off in every
    /// other test of the binary, where the slot runs what it held before).
    macro_rules! oc434_gated {
        ($($fake:ident => $addr:literal, $slot:path, $generated:path, $post:path;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC434_CTRL) != OC434_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
                    fakes::fake(c, $addr, ra);
                    $post(c, ra, a0, a1);
                }
            )*

            pub(crate) fn oc434_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc434_gated! {
        oc434_fake_new => 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, oc434_bump;
        oc434_fake_alloc => 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, oc434_bump;
        oc434_fake_free => 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, oc434_plain;
        oc434_fake_be_instance => 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, oc434_plain;
        oc434_fake_be_interface => 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, oc434_plain;
        oc434_fake_human_rider => 0x0014_6e98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated, oc434_plain;
        oc434_fake_character => 0x0014_7398, ee::s01::OVERRIDE_00147398, ee::s01::f_00147398__generated, oc434_plain;
        oc434_fake_alt_character => 0x0014_7410, ee::s01::OVERRIDE_00147410, ee::s01::f_00147410__generated, oc434_plain;
        oc434_fake_clear_new_rewards => 0x0015_a510, ee::s01::OVERRIDE_0015A510, ee::s01::f_0015a510__generated, oc434_plain;
        oc434_fake_find_rider => 0x0014_a130, ee::s01::OVERRIDE_0014A130, ee::s01::f_0014a130__generated, oc434_plain;
        oc434_fake_rider_character => 0x0014_a080, ee::s01::OVERRIDE_0014A080, ee::s01::f_0014a080__generated, oc434_plain;
        oc434_fake_next_points => 0x0015_8be0, ee::s01::OVERRIDE_00158BE0, ee::s01::f_00158be0__generated, oc434_plain;
        oc434_fake_next_collectible => 0x0015_8c80, ee::s01::OVERRIDE_00158C80, ee::s01::f_00158c80__generated, oc434_plain;
        oc434_fake_next_mission => 0x0015_8d58, ee::s01::OVERRIDE_00158D58, ee::s01::f_00158d58__generated, oc434_plain;
        oc434_fake_layer_anim => 0x0031_2aa0, ee::s10::OVERRIDE_00312AA0, ee::s10::f_00312aa0__generated, oc434_plain;
        oc434_fake_play_anim => 0x0031_28e8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated, oc434_plain;
        oc434_fake_hud_number => 0x0011_7b88, ee::s00::OVERRIDE_00117B88, ee::s00::f_00117b88__generated, oc434_plain;
        oc434_fake_update_anim_speed => 0x0012_e528, ee::s00::OVERRIDE_0012E528, ee::s00::f_0012e528__generated, oc434_plain;
        oc434_fake_crowd_reset => 0x0022_92e0, ee::s05::OVERRIDE_002292E0, ee::s05::f_002292e0__generated, oc434_plain;
        oc434_fake_world_state_stream => 0x0026_d5e8, ee::s07::OVERRIDE_0026D5E8, ee::s07::f_0026d5e8__generated, oc434_plain;
        oc434_fake_snapshot_list => 0x0026_e5a8, ee::s07::OVERRIDE_0026E5A8, ee::s07::f_0026e5a8__generated, oc434_plain;
        oc434_fake_highlight_slot => 0x0027_2288, ee::s07::OVERRIDE_00272288, ee::s07::f_00272288__generated, oc434_plain;
        oc434_fake_replay_state => 0x0026_f4a0, ee::s07::OVERRIDE_0026F4A0, ee::s07::f_0026f4a0__generated, oc434_plain;
        oc434_fake_free_info => 0x0027_5cd0, ee::s07::OVERRIDE_00275CD0, ee::s07::f_00275cd0__generated, oc434_plain;
        oc434_fake_find_bank => 0x0028_36d8, ee::s07::OVERRIDE_002836D8, ee::s07::f_002836d8__generated, oc434_plain;
        oc434_fake_parser_ctor => 0x0032_1428, ee::s10::OVERRIDE_00321428, ee::s10::f_00321428__generated, oc434_plain;
        oc434_fake_stream_reset => 0x003a_6ed8, ee::s13::OVERRIDE_003A6ED8, ee::s13::f_003a6ed8__generated, oc434_plain;
        oc434_fake_section_table_dtor => 0x003a_7d80, ee::s13::OVERRIDE_003A7D80, ee::s13::f_003a7d80__generated, oc434_plain;
        oc434_fake_section_unloaded => 0x003a_9890, ee::s13::OVERRIDE_003A9890, ee::s13::f_003a9890__generated, oc434_unloaded;
        oc434_fake_world_memman_dtor => 0x003a_7aa8, ee::s13::OVERRIDE_003A7AA8, ee::s13::f_003a7aa8__generated, oc434_plain;
        oc434_fake_free_section_allocator => 0x003a_7b98, ee::s13::OVERRIDE_003A7B98, ee::s13::f_003a7b98__generated, oc434_plain;
        oc434_fake_queue_close => 0x003d_d5a0, ee::s14::OVERRIDE_003DD5A0, ee::s14::f_003dd5a0__generated, oc434_zero;
        oc434_fake_queue_open => 0x003d_d4e8, ee::s14::OVERRIDE_003DD4E8, ee::s14::f_003dd4e8__generated, oc434_zero;
    }

    pub(crate) fn oc434_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Scrambled machine with the fakes on, `pc` at the function's entry; at most
    /// `calls` fake calls, one of which may unwind.
    fn oc434_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc434_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC434_CTRL, OC434_MAGIC);
        c.pc = entry;
        r
    }

    /// Force the result of fake call `n` (non-zero values only).
    fn oc434_force(c: &mut Cpu, n: u32, v: u32) {
        c.bus.write32(LOG_RESULTS + 4 * n, v);
    }

    /// A vtable whose slots `slot` (the method address word at `slot`, the `this`
    /// adjustment halfword at `slot - 4`) point at fake methods.
    fn oc434_vtable_slot(c: &mut Cpu, vt: u32, slot: u32, delta: u16, fake: u32) {
        c.bus.write16(vt + slot - 4, delta);
        c.bus.write32(vt + slot, fake);
    }

    /// Shared fixture of the two path table readers: a path file with `seed % 4`
    /// empty paths (every 4th..7th seed with one 8-byte item in the first).
    fn oc434_read_paths_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _ = oc434_setup(c, seed, salt, 4, entry);
        let (this, data) = (DATA + 0x100, DATA + 0x1000);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = data as u64;
        let count = seed % 4;
        oc434_w(c, data, count);
        if seed % 8 >= 4 && count > 0 {
            oc434_w(c, data + 4, 1);
            oc434_w(c, data + 8, 7);
            oc434_w(c, data + 12, 8);
        }
        oc434_force(c, 0, DATA + 0x4000);
    }

    /// `PathData_ReadTrackPaths`: 0..3 paths (allocation, vtable stores, `Path_Read`
    /// with its virtual calls and unwinds); the count read and `Path_Read` are real.
    #[test]
    fn path_data_read_track_paths_matches() {
        fakes::covered(
            ee::s07::f_0026b310__generated,
            path_data_read_track_paths,
            24,
            &|c, seed| oc434_read_paths_setup(c, seed, 0x26b310, 0x0026_b310),
        );
    }

    /// `PathData_ReadAIPaths`: the same for the AI path table.
    #[test]
    fn path_data_read_ai_paths_matches() {
        fakes::covered(
            ee::s07::f_0026b410__generated,
            path_data_read_ai_paths,
            24,
            &|c, seed| oc434_read_paths_setup(c, seed, 0x26b410, 0x0026_b410),
        );
    }

    /// `RiderMetrix_AwardBonusPoints`: HUD numbers (not career, or career with game type
    /// 1), or cash (career: amount 0, small, capped at 20 with the busy world so the
    /// real `RiderMetrix_AwardPoints` returns at once).
    #[test]
    fn rider_metrix_award_bonus_points_matches() {
        fakes::covered(
            ee::s00::f_00119608__generated,
            rider_metrix_award_bonus_points,
            48,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x119608, 4, 0x0011_9608);
                let this = DATA + 0x100;
                let pts: [u32; 8] = [0, 100, 499, 500, 10_000, 10_500, 50_000, 123_456];
                let scale: [u32; 4] = [0x3f80_0000, 0x3f00_0000, 0x4040_0000, 0];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = pts[(seed / 4 % 8) as usize] as u64;
                oc434_w(c, this + 0x1c4, scale[(seed / 3 % 4) as usize]);
                oc434_w(c, this + 0x198, seed.wrapping_mul(7919));
                let (kind, game) = match seed % 4 {
                    0 => (0, 0),
                    1 | 3 => (4, 0),
                    _ => (4, 1),
                };
                c.bus.write8(0x0053_5bc8 + 0x48, kind);
                c.bus.write8(0x0053_5bc8 + 0x49, game);
                // The busy world: `AwardPoints` does nothing.
                let (world, x, y) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                oc434_w(c, GAME_GP.wrapping_sub(0x848), world);
                oc434_w(c, world + 0x84, x);
                oc434_w(c, x + 0x28, y);
                oc434_w(c, y, 5);
            },
        );
    }

    /// `GMHandler1_SetupRiderSlots`: heat 0..3 (rows of ids below and above 10), the
    /// human rider in slot 1..3 or past the last slot.
    #[test]
    fn gm_handler1_setup_rider_slots_matches() {
        fakes::covered(
            ee::s06::f_0023a668__generated,
            gm_handler1_setup_rider_slots,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x23a668, 5, 0x0023_a668);
                let (this, mgr) = (DATA + 0x100, DATA + 0x2000);
                c.g[A0][0] = this as u64;
                oc434_w(c, GAME_GP.wrapping_sub(0x480), mgr);
                oc434_w(c, mgr, seed % 5);
                for i in 0..24 {
                    oc434_w(c, this + 0x10 + 4 * i, r.next_u32() % 20);
                }
                oc434_force(c, 0, DATA + 0x3000);
                oc434_force(c, 1, DATA + 0x3100);
                oc434_force(c, 2, [1, 2, 3, 6][(seed % 4) as usize]);
                oc434_force(c, 3, 5 + seed % 4);
                oc434_force(c, 4, 20 + seed % 3);
            },
        );
    }

    /// `CrowdMan2d_ctor`: the real 2D renderer and animation constructors run (the texture
    /// file load fails: the file queue fake returns 0), the allocations are fakes.
    #[test]
    fn crowd_man2d_ctor_matches() {
        fakes::covered(ee::s05::f_00229180__generated, crowd_man2d_ctor, 16, &|c, seed| {
            let _ = oc434_setup(c, seed, 0x229180, 6, 0x0022_9180);
            c.g[A0][0] = (DATA + 0x100) as u64;
        });
    }

    /// `cWorldCache_Free`: with and without the chunk resolver (virtual destructor), the
    /// shared table, the section table (sections loaded or not, by mask), the data
    /// header and the world memory manager.
    #[test]
    fn world_cache_free_matches() {
        fakes::covered(ee::s13::f_003a7e98__generated, world_cache_free, 64, &|c, seed| {
            let _ = oc434_setup(c, seed, 0x3a7e98, 8, 0x003a_7e98);
            let this = DATA + 0x100;
            let (chunk, vt, shared, table, header, memman) = (
                DATA + 0x500,
                DATA + 0x600,
                DATA + 0x800,
                DATA + 0x900,
                DATA + 0x700,
                DATA + 0xa00,
            );
            c.g[A0][0] = this as u64;
            oc434_w(c, this, if seed & 8 != 0 { header } else { 0 });
            oc434_w(c, header + 8, seed / 4 % 4);
            oc434_w(c, this + 4, if seed & 2 != 0 { shared } else { 0 });
            oc434_w(c, this + 8, if seed & 4 != 0 { table } else { 0 });
            oc434_w(c, this + 0xc, if seed & 16 != 0 { memman } else { 0 });
            oc434_w(c, this + 0x3e8, if seed & 1 != 0 { chunk } else { 0 });
            oc434_w(c, chunk + 8, vt);
            oc434_vtable_slot(c, vt, 0xc, 0x10, FAKE);
            // ReleaseAllHulls (real): no hull pages.
            oc434_w(c, this + 0x3ec, 0);
            // Section table entries: allocated or not.
            oc434_w(c, table, if seed & 32 != 0 { DATA + 0xb00 } else { 0 });
            oc434_w(c, table + 4, DATA + 0xb40);
            oc434_w(c, table + 8, 0);
            oc434_w(c, OC434_UNLOADED, (seed / 8) % 8);
        });
    }

    /// `ScriptEngine_LoadMaster`: the master table file fails to load (the file queue
    /// fake returns 0), so the header is the words at address 0: its count of script
    /// entries (0..3) is cleared (no debug base), then the real
    /// `ScriptEngine_CreateScriptDatabase` runs.
    #[test]
    fn script_engine_load_master_matches() {
        fakes::covered(
            ee::s07::f_00278210__generated,
            script_engine_load_master,
            16,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x278210, 8, 0x0027_8210);
                c.g[A0][0] = (DATA + 0x100) as u64;
                oc434_w(c, GAME_GP.wrapping_add(0x3a4), DATA + 0x2000);
                oc434_w(c, GAME_GP.wrapping_add(0x3a8), DATA + 0x2100);
                oc434_w(c, 0, seed % 4);
                for i in 0..4 {
                    oc434_w(c, 4 + 12 * i + 8, 0x1111_1111);
                }
            },
        );
    }

    /// `RiderScriptObj_GetAnimBankEntry`: a leaf; reads two fields of the bank.
    #[test]
    fn rider_script_obj_get_anim_bank_entry_matches() {
        check(
            ee::s07::f_00280520__generated,
            rider_script_obj_get_anim_bank_entry,
            32,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x280520, 0, 0x0028_0520);
                let (obj, bank, entries) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (DATA + 0x400) as u64;
                c.g[A3][0] = (DATA + 0x410) as u64;
                oc434_w(c, obj + 0x1c, bank);
                oc434_w(c, bank + 4, r.next_u32());
                oc434_w(c, bank + 0x10, entries);
                for i in 0..5 {
                    oc434_w(c, entries + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `ScriptEngine_CreateScriptDatabase`: 0..4 script slots, a few player counts and
    /// heap sizes (the real `InSeq_ctor` builds the sequence heap on the bump allocation).
    #[test]
    fn script_engine_create_script_database_matches() {
        fakes::covered(
            ee::s07::f_00281f30__generated,
            script_engine_create_script_database,
            24,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x281f30, 4, 0x0028_1f30);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (1 + seed / 5 % 3) as u64;
                c.g[A3][0] = [0x4000, 0x8000][(seed / 3 % 2) as usize];
            },
        );
    }

    /// `InstancesMan_ctor`: 0..4 instance records.
    #[test]
    fn instances_man_ctor_matches() {
        fakes::covered(ee::s11::f_00351030__generated, instances_man_ctor, 20, &|c, seed| {
            let _ = oc434_setup(c, seed, 0x351030, 2, 0x0035_1030);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (seed.wrapping_mul(31) % 1000) as u64;
            c.g[A2][0] = (seed % 5) as u64;
        });
    }

    /// `BERewardInt_BeginRewardTracking`: all callees are fakes (ten calls).
    #[test]
    fn be_reward_int_begin_reward_tracking_matches() {
        fakes::covered(
            ee::s01::f_00158e30__generated,
            be_reward_int_begin_reward_tracking,
            24,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x158e30, 10, 0x0015_8e30);
                c.g[A0][0] = (DATA + 0x100) as u64;
                oc434_w(c, GAME_GP.wrapping_sub(0x1e8c), DATA + 0x800);
            },
        );
    }

    /// `DataInstanceMan_RemoveData`: an entry alone, or with a chain of 1..3 linked nodes.
    #[test]
    fn data_instance_man_remove_data_matches() {
        fakes::covered(
            ee::s07::f_00275a20__generated,
            data_instance_man_remove_data,
            32,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x275a20, 10, 0x0027_5a20);
                let (this, entry, vt) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = entry as u64;
                oc434_w(c, this + 0xc, vt);
                c.bus.write16(vt + 0x18, 0x20);
                oc434_w(c, vt + 0x1c, FAKE);
                c.bus.write16(vt + 0x28, 0x30);
                oc434_w(c, vt + 0x2c, FAKE + 0x10);
                let chain = seed % 4;
                let node = |i: u32| DATA + 0x400 + 0x20 * i;
                oc434_w(c, entry + 0x14, if chain > 0 { node(0) } else { 0 });
                for i in 0..chain {
                    oc434_w(
                        c,
                        node(i) + 0x14,
                        if i + 1 < chain { node(i + 1) } else { 0 },
                    );
                }
                for f in [0, 4, 8, 0xc] {
                    oc434_w(c, entry + f, r.next_u32());
                    for i in 0..3 {
                        oc434_w(c, node(i) + f, r.next_u32());
                    }
                }
            },
        );
    }

    /// `RiderWipeoutSub_SwitchToAnimSetB`: every tumble animation 361..388 (the jump table
    /// is written into RAM), root pose, a bad table entry (the jump-out), and unwinds.
    #[test]
    fn rider_wipeout_sub_switch_to_anim_set_b_matches() {
        fakes::covered(
            ee::s00::f_0012dcb0__generated,
            rider_wipeout_sub_switch_to_anim_set_b,
            96,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x12dcb0, 3, 0x0012_dcb0);
                let (this, rider, anim) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.g[A0][0] = this as u64;
                oc434_w(c, this + 0x80, rider);
                oc434_w(c, rider + 0x784, anim);
                oc434_w(
                    c,
                    rider + 0x150,
                    if seed % 7 == 0 { 1 } else { seed % 3 * 2 },
                );
                let cur = 361 + seed % 29;
                oc434_force(c, 0, cur);
                for idx in 0..26u32 {
                    let target = match idx {
                        0..=9 => 0x0012_dd10 + 8 * idx,
                        10..=21 => 0x0012_dd64,
                        _ => 0x0012_dd10 + 8 * (6 + idx - 22),
                    };
                    let target = if seed % 11 == 5 && cur == 362 + idx {
                        0x0012_dd60
                    } else {
                        target
                    };
                    oc434_w(c, WIPEOUT_SET_B_TABLE + 4 * idx, target);
                }
            },
        );
    }

    /// `tWScriptCache_GetGlobals`: a leaf; type 3 with and without data, other types.
    #[test]
    fn w_script_cache_get_globals_matches() {
        check(
            ee::s13::f_003adcb0__generated,
            w_script_cache_get_globals,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x3adcb0, 0, 0x003a_dcb0);
                let (entry, data) = (DATA + 0x100, DATA + 0x200);
                c.g[A0][0] = entry as u64;
                c.bus.write16(entry, if seed % 3 == 0 { 2 } else { 3 });
                oc434_w(c, entry + 8, if seed % 4 == 1 { 0 } else { data });
                oc434_w(c, data + 0x14, r.next_u32());
            },
        );
    }

    /// `World_GetSectionShutdownFuncs`: entry `i` of the script cache (the real
    /// `tWScriptCache_GetGlobals` decides).
    #[test]
    fn world_get_section_shutdown_funcs_matches() {
        check(
            ee::s13::f_003a6bd8__generated,
            world_get_section_shutdown_funcs,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x3a6bd8, 0, 0x003a_6bd8);
                let (this, cache, entries, data) =
                    (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
                oc434_w(c, this + 4, cache);
                oc434_w(c, cache + 4, entries);
                for i in 0..3u32 {
                    let e = entries + 0xc * i;
                    c.bus
                        .write16(e, if (seed / 3 + i) % 3 == 0 { 2 } else { 3 });
                    oc434_w(
                        c,
                        e + 8,
                        if (seed / 5 + i) % 4 == 1 {
                            0
                        } else {
                            data + 0x20 * i
                        },
                    );
                    oc434_w(c, data + 0x20 * i + 0x14, r.next_u32());
                }
            },
        );
    }

    /// `ScriptAnimBankMan_ctor`: 0..4 bank records.
    #[test]
    fn script_anim_bank_man_ctor_matches() {
        fakes::covered(
            ee::s07::f_002753b8__generated,
            script_anim_bank_man_ctor,
            20,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x2753b8, 2, 0x0027_53b8);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (r.next_u32() % 100) as u64;
            },
        );
    }

    /// `InputMap_CreateParser`: allocation and parser constructor are fakes.
    #[test]
    fn input_map_create_parser_matches() {
        fakes::covered(
            ee::s10::f_00320db0__generated,
            input_map_create_parser,
            12,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x320db0, 2, 0x0032_0db0);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// `cStreamMan_ctor`: the real `StreamFile_CalcMemSize` / `StreamFile_Create`, the
    /// allocations and the reset are fakes.
    #[test]
    fn c_stream_man_ctor_matches() {
        fakes::covered(ee::s13::f_003a6d50__generated, c_stream_man_ctor, 12, &|c, seed| {
            let _ = oc434_setup(c, seed, 0x3a6d50, 6, 0x003a_6d50);
            c.g[A0][0] = (DATA + 0x100) as u64;
        });
    }

    /// `Replay_ctor`: 16 frames, two snapshot lists, two highlight slots, the state
    /// change (21 fake calls, unwinding in any of them).
    #[test]
    fn replay_ctor_matches() {
        fakes::covered(ee::s07::f_0026efb8__generated, replay_ctor, 24, &|c, seed| {
            let _ = oc434_setup(c, seed, 0x26efb8, 21, 0x0026_efb8);
            c.g[A0][0] = (DATA + 0x100) as u64;
        });
    }

    /// `ScriptEngine_FindSoundBank`: one forwarded call.
    #[test]
    fn script_engine_find_sound_bank_matches() {
        fakes::covered(
            ee::s07::f_00279ef8__generated,
            script_engine_find_sound_bank,
            12,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x279ef8, 1, 0x0027_9ef8);
                c.g[A0][0] = (DATA + (r.next_u32() & 0xf_f000)) as u64;
            },
        );
    }
}
