//! Staging module oc433 (wave wave221): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc433`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_de60, game_camera_init, "GameCamera_Init"),
    (0x0022_7e68, ssx_app_flush_ticks, "SSXApp_FlushTicks"),
    (0x003a_6800, world_reset_sections, "World_ResetSections"),
    (
        0x0030_0e28,
        w_script_fn_define_collect_set,
        "WScriptFn_DefineCollectSet",
    ),
    (
        0x0028_07b0,
        script_audio_obj_deactivate,
        "ScriptAudioObj_Deactivate",
    ),
    (0x0023_5868, game_state_reset_exit, "GameStateReset_Exit"),
    (
        0x0028_1370,
        script_view_obj_activate,
        "ScriptViewObj_Activate",
    ),
    (
        0x003a_88a8,
        c_world_cache_flush_hull_pages,
        "cWorldCache_FlushHullPages",
    ),
    (0x0027_6270, script_list_prepare, "ScriptList_Prepare"),
    (
        0x0023_4ad0,
        game_state_countdown_enter,
        "GameStateCountdown_Enter",
    ),
    (
        0x003a_8330,
        c_world_cache_release_all_hulls,
        "cWorldCache_ReleaseAllHulls",
    ),
    (
        0x0027_6f48,
        script_list_skip_current,
        "ScriptList_SkipCurrent",
    ),
    (0x0015_dc70, game_camera_ctor, "GameCamera_ctor"),
    (0x0012_86a0, ai_ctor, "AI_ctor"),
    (
        0x0027_90a0,
        script_engine_list_stop,
        "ScriptEngine_ListStop",
    ),
    (0x0030_87d0, w_script_man_ctor, "WScriptMan_ctor"),
    (
        0x0027_b268,
        script_engine_load_bank_sync,
        "ScriptEngine_LoadBankSync",
    ),
    (
        0x003a_a2f0,
        c_ps2_chunk_resolve_ctor,
        "cPS2ChunkResolve_ctor",
    ),
    (
        0x0035_8020,
        multi_particle_create_emitter,
        "MultiParticle_CreateEmitter",
    ),
];

/// `void GameCamera_Init(GameCamera* self, int riderIndex, float view, float nearPlane, float farPlane)`
/// (0x15de60): store the view parameters and the rider index, point the manual
/// controller (`+0xa4`) at viewport 0 and select algorithm 81, point the chase
/// controller (`+0xa8`) at the rider's viewport, allocate a rider-camera target
/// (vtable `0x45b7a8`, the rider from the world's rider table), hand it to the
/// chase controller and the rider-camera algorithm, snap to the default chase
/// camera, clear the roll/shake words and set the script camera's viewport
/// according to the rider index (0 or 1; other indices leave `+0xb0` alone).
#[allow(unreachable_code)]
pub fn game_camera_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_15df6c: {
            'b_15df68: {
                'b_15df64: {
                    c.cycles += 60;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = 0x51;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S1][0] = sx((c.g[S3][0] as u32) << 2);
                    c.swc1(13, (c.g[S2][0] as u32).wrapping_add(0x10));
                    c.swc1(14, (c.g[S2][0] as u32).wrapping_add(0x14));
                    c.swc1(12, (c.g[S2][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x460), 0u64);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
                    Callee {
                        addr: 0x0015cf80,
                        f: ee::s01::ManualCamController_SelectAlgorithm,
                    }
                    .call(c, 0x0015deb4)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x460000;
                    c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa8));
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4870));
                    c.g[A0][0] = 0x8;
                    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    call_ptr(c, 0x00317d70, 0x0015dee0)?;
                    c.g[V1][0] = 0x460000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4858));
                    c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    Callee {
                        addr: 0x00161a60,
                        f: ee::s01::ChaseCamController_SetTarget,
                    }
                    .call(c, 0x0015df14)?;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa8));
                    c.g[A1][0] = 0x4c;
                    Callee {
                        addr: 0x00162060,
                        f: ee::s01::ChaseCamController_SetOverrideAlgorithm,
                    }
                    .call(c, 0x0015df2c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
                    Callee {
                        addr: 0x00162310,
                        f: ee::s01::RiderCameraAlgorithm_SetTarget,
                    }
                    .call(c, 0x0015df38)?;
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x0015e030,
                        f: ee::s01::GameCamera_ResetDefaultChase,
                    }
                    .call(c, 0x0015df40)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x454), 0u64);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x458), 0u64);
                    let t = c.g[S3][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x45c), 0u64);
                    if t {
                        break 'b_15df64;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[S3][0] == c.g[V0][0] {
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xb0), c.g[S3][0]);
                            break 'b_15df68;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xb4), 0u64);
                            if t {
                                break 'b_15df6c;
                            } else {
                                break 'b_15df64;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xb0), 0u64);
                break 'b_15df68;
            }
            c.cycles += 1;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xb4), 0u64);
            break 'b_15df6c;
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xac));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `int SSXApp_FlushTicks(void)` (0x227e68): when the pad-queue global
/// (`gp-0x850`) is set, call the queue flush (0x326c60) and return its result,
/// else return 0.
#[allow(unreachable_code)]
pub fn ssx_app_flush_ticks(c: &mut Cpu) {
    guest(c, |c| {
        'b_227e88: {
            'b_227e80: {
                c.cycles += 4;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
                let t = c.g[A0][0] != 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_227e80;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_227e88;
                    } else {
                        break 'b_227e80;
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x00326c60, 0x00227e88)?;
            break 'b_227e88;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `PlaybackMan* PlaybackMan_ctor(PlaybackMan* self, int a1)` (0x26c458): build the
/// replay playback manager: the embedded `WorldStateStream` (`+0x14`), the world
/// and rider-table words copied from the game world, two allocated helpers (a
/// pointer cell and a pair set with `ValuePair_SetFirst/SetSecond`) and a
/// `BigFile` opened from a formatted name; returns `self`.
// Not in `OVERRIDES` and without a differential test: its last call (0x316978, `BigFile`
// open) reaches the real EA file layer (`EAFile_Exists` -> request queue), which runs
// away on fixture data (pc 0x3de768) and has no override slot to fake. The port is
// otherwise a faithful transliteration of 0x26c458.
#[allow(dead_code, unreachable_code)]
pub fn playback_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S1][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee {
            addr: 0x0026d5e8,
            f: ee::s07::WorldStateStream_ctor,
        }
        .call(c, 0x0026c47c)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x2a4;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1550));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[T1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[T1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdf0));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32));
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[T0][0]);
        call_ptr(c, 0x00317d70, 0x0026c4e4)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V0][0]);
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0x20000000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3420));
        c.g[A0][0] = 0x8;
        c.sw((c.g[V0][0] as u32), 0u64);
        call_ptr(c, 0x00317d70, 0x0026c504)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00244400, 0x0026c514)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00244408, 0x0026c520)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x328));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x320));
        call_ptr(c, 0x00417828, 0x0026c534)?;
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3428));
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x00317d70, 0x0026c54c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00316898, 0x0026c554)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00316978, 0x0026c568)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void World_ResetSections(World* self)` (0x3a6800): for every resident
/// streamed section, walk its sub-entries and mark the matching table entries
/// (`word8 = word8 & 0xffff0000 | (word8 >> 16) | 2`). Dispatcher-shaped like
/// the original: its loop heads 0x3a6828 and 0x3a6878 are resume entries
/// after a delivered event.
#[allow(unreachable_code)]
pub fn world_reset_sections(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003a6800 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = 0xffffffffffff0000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[S0][0] | 0xffff;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S2][0] = c.g[A0][0];
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003a68f8;
                        continue;
                    }
                    pc = 0x003a6828;
                    continue;
                }
                0x003a6828 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003a9858,
                        f: ee::s13::cWorldCache_StreamerIsSectionResident,
                    }
                    .call(c, 0x003a6834)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003a68f4;
                        continue;
                    }
                    pc = 0x003a683c;
                    continue;
                }
                0x003a683c => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                    c.g[V1][0] = 0x58;
                    c.g[V0][0] = 0xffffffffffffff00;
                    c.mult(0, 3, 17, 3);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                    c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0] & 0xff;
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x26));
                    let t = (c.g[T1][0] as i64) <= 0;
                    c.g[S0][0] = c.g[S0][0] | c.g[A2][0];
                    if t {
                        pc = 0x003a68f4;
                        continue;
                    }
                    pc = 0x003a6870;
                    continue;
                }
                0x003a6870 => {
                    c.cycles += 2;
                    c.g[T3][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                    pc = 0x003a6878;
                    continue;
                }
                0x003a6878 => {
                    c.cycles += 13;
                    c.g[V1][0] = c.g[S0][0] & 0xff;
                    c.g[A1][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[A0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = c.g[V1][0] | c.g[A1][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[T0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x003a68cc;
                        continue;
                    }
                    pc = 0x003a68ac;
                    continue;
                }
                0x003a68ac => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003a68c8;
                        continue;
                    }
                    pc = 0x003a68c4;
                    continue;
                }
                0x003a68c4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    pc = 0x003a68c8;
                    continue;
                }
                0x003a68c8 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[A0][0];
                    pc = 0x003a68cc;
                    continue;
                }
                0x003a68cc => {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;

                    if t {
                        pc = 0x003a68ec;
                        continue;
                    }
                    pc = 0x003a68d4;
                    continue;
                }
                0x003a68d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003a68ec;
                    continue;
                }
                0x003a68ec => {
                    c.cycles += 2;

                    if c.g[T0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                        if event_due(c) && back_edge_unwinds(c, 0x003a6878) {
                            return Ok(());
                        }
                        pc = 0x003a6878;
                        continue;
                    }
                    pc = 0x003a68f4;
                    continue;
                }
                0x003a68f4 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[T2][0];
                    pc = 0x003a68f8;
                    continue;
                }
                0x003a68f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003a9800,
                        f: ee::s13::cWorldCache_StreamerGetSectionCount,
                    }
                    .call(c, 0x003a6904)?;
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                        if event_due(c) && back_edge_unwinds(c, 0x003a6828) {
                            return Ok(());
                        }
                        pc = 0x003a6828;
                        continue;
                    }
                    pc = 0x003a6910;
                    continue;
                }
                0x003a6910 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003a6800, 0x003a6928);
                    return Ok(());
                }
            }
        }
        Ok(())
    })
}

/// `CollectSet* WScriptFn_DefineCollectSet(CollectSet* out, int n, Entry* entries)`
/// (0x300e28): on first use initialise the static default set, copy it to the
/// stack, patch `n` (index, value, type) entries into it (type 2: store the
/// value as a float), define the collect set in the script manager when its first
/// word is not -1, and copy the 12-byte result to `out`.
#[allow(unreachable_code)]
pub fn w_script_fn_define_collect_set(c: &mut Cpu) {
    guest(c, |c| {
        'b_300e80: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25bc));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_300e80;
            } else {
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4498));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4498), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x25bc), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sb((c.g[SP][0] as u32), c.g[A1][0]);
                break 'b_300e80;
            }
        }
        'b_300ef0: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4498));
            c.g[A2][0] = 0xc;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x00300e94)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_300ef0;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xb70));
                c.g[A3][0] = 0x2;
                'l_300ea8: loop {
                    'b_300ee4: {
                        'b_300ee0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_300ee0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_300ee0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, (c.g[A0][0] as u32));
                                    if t {
                                        break 'b_300ee4;
                                    } else {
                                        break 'b_300ee0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                        break 'b_300ee4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00300ea8);
                        }
                        continue 'l_300ea8;
                    } else {
                        break 'b_300ef0;
                    }
                }
            }
        }
        'b_300f14: {
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V1][0] = 0x4d0000;
            if t {
                break 'b_300f14;
            } else {
                c.cycles += 4;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                Callee {
                    addr: 0x0030b928,
                    f: ee::s10::WScriptMan_DefineCollectSet,
                }
                .call(c, 0x00300f10)?;
                c.g[V1][0] = 0x4d0000;
                break 'b_300f14;
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, (c.g[A3][0] as u32));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, (c.g[S3][0] as u32));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void ScriptAudioObj_Deactivate(ScriptAudioObj* self)` (0x2807b0): deactivate
/// the base object, free the script audio events (`+0x1c`, `+0xec` of them) in
/// the script manager, stop the three audio handles (`+0xbc`) that are in use,
/// clear the object's counters and, when the audio-fields reader reports so,
/// clear the global at `gp+0xa70`.
#[allow(unreachable_code)]
pub fn script_audio_obj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        'b_280814: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[S1][0] = 0u64;
            Callee {
                addr: 0x00283440,
                f: ee::s07::ScriptObj_Deactivate,
            }
            .call(c, 0x002807e0)?;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xec));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
            if t {
                break 'b_280814;
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                'l_2807f0: loop {
                    c.cycles += 9;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00309e50, 0x00280804)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xec));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x002807f0);
                        }
                        continue 'l_2807f0;
                    } else {
                        break 'b_280814;
                    }
                }
            }
        }
        'b_280830: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf0));
            let t = c.g[V0][0] == 0u64;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbc));
            if t {
                break 'b_280830;
            } else {
                c.cycles += 4;
                Callee {
                    addr: 0x0028b180,
                    f: ee::s07::SSXAudioSystem_GetInstance,
                }
                .call(c, 0x00280828)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a1bd0, 0x00280830)?;
                break 'b_280830;
            }
        }
        c.cycles += 4;
        c.g[S4][0] = 0xffffffffffffffff;
        c.g[S1][0] = c.g[S2][0];
        c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0xec));
        'l_280840: loop {
            'b_280874: {
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                if (c.g[S0][0] as i64) < 0 {
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S4][0]);
                    break 'b_280874;
                } else {
                    c.cycles += 10;
                    Callee {
                        addr: 0x0028b180,
                        f: ee::s07::SSXAudioSystem_GetInstance,
                    }
                    .call(c, 0x00280854)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = c.g[S0][0];
                    c.mtc1(0, 12);
                    c.g[A2][0] = 0x1;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, 0x002ad5f0, 0x00280870)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S4][0]);
                    break 'b_280874;
                }
            }
            c.cycles += 6;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.sw((c.g[S1][0] as u32), c.g[S4][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
            let t = c.g[V0][0] != 0u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00280840);
                }
                continue 'l_280840;
            } else {
                'b_2808a4: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x0027cc28,
                        f: ee::s07::ScriptAudio_ReadFields,
                    }
                    .call(c, 0x00280898)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[GP][0] as u32).wrapping_add(0xa70), 0u64);
                        break 'b_2808a4;
                    } else {
                        break 'b_2808a4;
                    }
                }
                c.cycles += 13;
                c.sw((c.g[S3][0] as u32).wrapping_add(0xf8), 0u64);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xec), 0u64);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xf0), 0u64);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xf4), 0u64);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void GameStateReset_Exit(GameState* self)` (0x235868): leave the reset state:
/// stop the script manager, run the rider/camera resets, clear the state's
/// `+0x14`, set up the mode handler (HandlerV06 for modes 5 and 6) and clear a
/// word of the first rider when the world is in single-player mode.
#[allow(unreachable_code)]
pub fn game_state_reset_exit(c: &mut Cpu) {
    guest(c, |c| {
        'b_235914: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            call_ptr(c, 0x00308f38, 0x0023588c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308c60, 0x00235894)?;
            call_ptr(c, 0x0014dc80, 0x0023589c)?;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002358a8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f228, 0x002358b8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f7b8, 0x002358c4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
            c.g[V0][0] = 0x530000;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
            if c.g[V1][0] != 0u64 {
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                break 'b_235914;
            } else {
                c.cycles += 14;
                call_ptr(c, 0x0014dc80, 0x002358e4)?;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002358f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00144bc0, 0x002358fc)?;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x00145dd0, 0x00235910)?;
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                break 'b_235914;
            }
        }
        'b_235940: {
            'b_235928: {
                c.cycles += 3;
                c.g[V0][0] = 0x5;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x6;
                if t {
                    break 'b_235928;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_235940;
                    } else {
                        break 'b_235928;
                    }
                }
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
            Callee {
                addr: 0x00238510,
                f: ee::s06::GameModeMan_HandlerV06,
            }
            .call(c, 0x0023593c)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'b_235940;
        }
        'b_23597c: {
            c.cycles += 6;
            c.g[V1][0] = 0x1;
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_23597c;
            } else {
                c.cycles += 9;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x50;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xb0));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.mult(0, 2, 2, 6);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x250), 0u64);
                break 'b_23597c;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void ScriptViewObj_Activate(ScriptViewObj* self, int a1)` (0x281370): bind the
/// object filter in the script engine, reset the view object and, when the
/// sequence is playing, run the camera hook and set `+0x110`; then activate the
/// base `ScriptObj`.
#[allow(unreachable_code)]
pub fn script_view_obj_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_2813e0: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            Callee {
                addr: 0x0027d2e8,
                f: ee::s07::ScriptEngine_GetInstance,
            }
            .call(c, 0x0028138c)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x0027d0c0,
                f: ee::s07::ScriptEngine_SetObjBindingFilter,
            }
            .call(c, 0x0028139c)?;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x00281a50,
                f: ee::s07::ScriptViewObj_Reset,
            }
            .call(c, 0x002813a4)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x00282c18,
                f: ee::s07::ScriptEngine_GetSeqInstance,
            }
            .call(c, 0x002813b0)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = c.g[V0][0];
            Callee {
                addr: 0x00279370,
                f: ee::s07::ScriptEngine_IsPlayingScript,
            }
            .call(c, 0x002813bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_2813e0;
            } else {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x100));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                call_ptr(c, 0x001033b0, 0x002813d8)?;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x110), c.g[V0][0]);
                break 'b_2813e0;
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x00283430,
            f: ee::s07::ScriptObj_Activate,
        }
        .call(c, 0x002813ec)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void cWorldCache_FlushHullPages(cWorldCache* self)` (0x3a88a8): walk the
/// `+0x3ec` hull-page slots (0x18 bytes from `+0x3f0`) and free (`FreeHullPage`)
/// or retire (state 4) the ones whose state and age say so.
#[allow(unreachable_code)]
pub fn c_world_cache_flush_hull_pages(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a899c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
            let t = c.g[V1][0] == 0u64;
            c.g[S5][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
            if t {
                break 'b_3a899c;
            } else {
                c.cycles += 6;
                c.g[S4][0] = 0x1;
                c.g[S7][0] = 0x2;
                c.g[S3][0] = 0x4;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f0));
                c.g[S6][0] = 0x3;
                'l_3a8900: loop {
                    'b_3a898c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                        if c.g[V1][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                            break 'b_3a898c;
                        } else {
                            'b_3a8928: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0u64;
                                if t {
                                    break 'b_3a8928;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                                    if c.g[V0][0] == 0 {
                                        c.g[A0][0] = 0u64;
                                    }
                                    break 'b_3a8928;
                                }
                            }
                            'b_3a8988: {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[S7][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                if t {
                                    break 'b_3a8988;
                                } else {
                                    'b_3a897c: {
                                        'b_3a8970: {
                                            'b_3a8960: {
                                                'b_3a8948: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;

                                                    if t {
                                                        break 'b_3a8948;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[S4][0];

                                                        if t {
                                                            break 'b_3a8960;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x3ec));
                                                            if t {
                                                                break 'b_3a898c;
                                                            } else {
                                                                break 'b_3a8948;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[S6][0];

                                                if t {
                                                    break 'b_3a8970;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[S3][0];
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_3a897c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x3ec));
                                                        if t {
                                                            break 'b_3a898c;
                                                        } else {
                                                            break 'b_3a8960;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[A0][0] == 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                                break 'b_3a898c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw((c.g[S0][0] as u32), 0u64);
                                                if t {
                                                    break 'b_3a8988;
                                                } else {
                                                    break 'b_3a8970;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[A0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_3a8988;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32), c.g[S3][0]);
                                            break 'b_3a897c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x003a8528,
                                        f: ee::s13::cWorldCache_FreeHullPage,
                                    }
                                    .call(c, 0x003a8984)?;
                                    c.sw((c.g[S0][0] as u32), 0u64);
                                    break 'b_3a8988;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                            break 'b_3a898c;
                        }
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a8900);
                        }
                        continue 'l_3a8900;
                    } else {
                        break 'b_3a899c;
                    }
                }
            }
        }
        c.cycles += 11;
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
        Ok(())
    })
}

/// `int ScriptList_Prepare(ScriptList* self, int playOnReady)` (0x276270): when the
/// list is idle and has queued entries, prepare the first entry and set the
/// state (1: loading, 2: ready) from the entry's process state; returns the
/// state.
#[allow(unreachable_code)]
pub fn script_list_prepare(c: &mut Cpu) {
    guest(c, |c| {
        'b_2762a8: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = 0x1;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] != c.g[S3][0];
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_2762a8;
            } else {
                c.cycles += 3;
                Callee {
                    addr: 0x00277778,
                    f: ee::s07::ScriptList_PollLoading,
                }
                .call(c, 0x002762a4)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                break 'b_2762a8;
            }
        }
        'b_276370: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_276370;
            } else {
                'b_27636c: {
                    'b_2762d0: {
                        c.cycles += 6;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), c.g[S1][0]);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00283d28,
                            f: ee::s07::ScriptCmdQueue_Count,
                        }
                        .call(c, 0x002762c0)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2762d0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_27636c;
                            } else {
                                break 'b_2762d0;
                            }
                        }
                    }
                    'b_2762f8: {
                        c.cycles += 6;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        Callee {
                            addr: 0x00277450,
                            f: ee::s07::ScriptList_PrepareEntry,
                        }
                        .call(c, 0x002762dc)?;
                        c.g[S2][0] = c.g[V0][0];
                        let t = (c.g[S2][0] as i64) >= 0;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2762f8;
                        } else {
                            c.cycles += 4;
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x00276868,
                                f: ee::s07::ScriptList_Stop,
                            }
                            .call(c, 0x002762f0)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            if t {
                                break 'b_27636c;
                            } else {
                                break 'b_2762f8;
                            }
                        }
                    }
                    'b_276354: {
                        'b_276350: {
                            'b_276348: {
                                'b_276340: {
                                    c.cycles += 11;
                                    c.g[A1][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x00277598,
                                        f: ee::s07::ScriptList_PopEntries,
                                    }
                                    .call(c, 0x00276300)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0u64;
                                    Callee {
                                        addr: 0x00283d70,
                                        f: ee::s07::ScriptCmdQueue_At,
                                    }
                                    .call(c, 0x0027630c)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00277dd8,
                                        f: ee::s07::ScriptList_IsMovieEntry,
                                    }
                                    .call(c, 0x0027631c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        break 'b_276340;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                        Callee {
                                            addr: 0x00282798,
                                            f: ee::s07::ScriptEngine_GetProcessState,
                                        }
                                        .call(c, 0x00276330)?;
                                        let t = c.g[V0][0] == c.g[S3][0];
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_276348;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xa4),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_276354;
                                            } else {
                                                break 'b_276340;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = (c.g[V0][0] as i64) >= 0;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_276350;
                                } else {
                                    break 'b_276348;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
                            if t {
                                break 'b_276354;
                            } else {
                                break 'b_276350;
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                        break 'b_276354;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00277450,
                        f: ee::s07::ScriptList_PrepareEntry,
                    }
                    .call(c, 0x00276364)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    break 'b_27636c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_276370;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void GameStateCountdown_Enter(GameState* self)` (0x234ad0): enter the race
/// countdown: reset the world pointers, announce the start (`cGameComm_OnCountdownStart`),
/// optionally reset the scripts and the world, and for modes 4..6 go to the next
/// state, otherwise start the countdown audio and play the countdown script event.
#[allow(unreachable_code)]
pub fn game_state_countdown_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_234afc: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[A0][0] == 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0u64);
            if t {
                break 'b_234afc;
            } else {
                c.cycles += 2;
                Callee {
                    addr: 0x00258968,
                    f: ee::s06::cGameComm_OnCountdownStart,
                }
                .call(c, 0x00234afc)?;
                break 'b_234afc;
            }
        }
        'b_234b30: {
            c.cycles += 10;
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014de68, 0x00234b04)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x7;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            call_ptr(c, 0x0014dd58, 0x00234b14)?;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_234b30;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                Callee {
                    addr: 0x002300f0,
                    f: ee::s06::GameModule_ResetScriptsAndWorld,
                }
                .call(c, 0x00234b2c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_234b30;
            }
        }
        'b_234bdc: {
            'b_234b80: {
                c.cycles += 16;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x0026f228, 0x00234b38)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x0026f7b8, 0x00234b44)?;
                call_ptr(c, 0x0014dc80, 0x00234b4c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00234b58)?;
                c.g[V1][0] = 0x530000;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                if t {
                    break 'b_234b80;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233aa0,
                        f: ee::s06::GameState_GotoNext,
                    }
                    .call(c, 0x00234b78)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_234bdc;
                    } else {
                        break 'b_234b80;
                    }
                }
            }
            c.cycles += 23;
            c.g[A1][0] = 0x4;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x00234b8c)?;
            call_ptr(c, 0x0028b180, 0x00234b94)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0029c418, 0x00234b9c)?;
            call_ptr(c, 0x0028b180, 0x00234ba4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0029c420, 0x00234bb0)?;
            c.g[A0][0] = 0x480000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3fc8));
            call_ptr(c, 0x00317670, 0x00234bbc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Callee {
                addr: 0x00234be8,
                f: ee::s06::GameState_PlayScriptEvent,
            }
            .call(c, 0x00234bc8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039f840, 0x00234bd8)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_234bdc;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void cWorldCache_ReleaseAllHulls(cWorldCache* self)` (0x3a8330): for every hull
/// slot (`+0x3ec` of them, 0x18 bytes from `+0x3f0`) release what its state holds
/// (a jump table over states 0..4: cancel the stream, free the page) and clear the
/// slot. The function can be entered at resume points: those go to the generated
/// `__entries` form.
#[allow(unreachable_code)]
pub fn c_world_cache_release_all_hulls(c: &mut Cpu) {
    if c.pc != 0x003a8330 {
        return ee::s13::f_003a8330__generated(c);
    }
    guest(c, |c| {
        'b_3a842c: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_3a842c;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x18;
                'l_3a8358: loop {
                    'b_3a83e8: {
                        'b_3a83e4: {
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f0));
                            c.mult(0, 2, 17, 2);
                            c.g[S2][0] = c.g[V1][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                            c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x490000;
                            if t {
                                break 'b_3a83e4;
                            } else {
                                'b_3a83c4: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4e70));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x003a8390 => {
                                            c.cycles += 13;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x300));
                                            c.g[A1][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x003a7010,
                                                f: ee::s13::cStreamMan_Cancel,
                                            }
                                            .call(c, 0x003a839c)?;
                                            c.g[V0][0] = 0x18;
                                            c.g[V1][0] = 0x4;
                                            c.mult(0, 4, 17, 2);
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[S2][0] as u32)));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                            Callee {
                                                addr: 0x003a8528,
                                                f: ee::s13::cWorldCache_FreeHullPage,
                                            }
                                            .call(c, 0x003a83bc)?;
                                            let t = true;
                                            c.g[V1][0] = 0x18;
                                            if t {
                                                break 'b_3a83e8;
                                            } else {
                                                break 'b_3a83c4;
                                            }
                                        }
                                        0x003a83c4 => {
                                            break 'b_3a83c4;
                                        }
                                        0x003a83e4 => {
                                            break 'b_3a83e4;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x003a8330, 0x003a8444);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V0][0] = 0x18;
                                c.g[V1][0] = 0x4;
                                c.mult(0, 4, 17, 2);
                                c.g[A1][0] = c.g[S1][0];
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                c.g[A0][0] = c.g[S0][0];
                                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                Callee {
                                    addr: 0x003a8528,
                                    f: ee::s13::cWorldCache_FreeHullPage,
                                }
                                .call(c, 0x003a83e4)?;
                                break 'b_3a83e4;
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = 0x18;
                        break 'b_3a83e8;
                    }
                    c.cycles += 17;
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A1][0] = c.g[A0][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f8), 0u64);
                    c.g[V0][0] = c.g[A0][0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x3f4), 0u64);
                    c.g[V1][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x400), 0u64);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x404), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x3fc), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x18;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a8358);
                        }
                        continue 'l_3a8358;
                    } else {
                        break 'b_3a842c;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void ScriptList_SkipCurrent(ScriptList* self)` (0x276f48): when the playing
/// entry is skippable (`flags & 1`), mark the skip (`+0xc8`), re-prepare the next
/// entry (killing or stopping the pending one) and advance the list.
#[allow(unreachable_code)]
pub fn script_list_skip_current(c: &mut Cpu) {
    guest(c, |c| {
        'b_27704c: {
            'b_277048: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V1][0] = 0x3;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] != c.g[V1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_277048;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc8));
                    if c.g[V0][0] != 0u64 {
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_27704c;
                    } else {
                        c.cycles += 9;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00283d70,
                            f: ee::s07::ScriptCmdQueue_At,
                        }
                        .call(c, 0x00276f8c)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.g[V1][0] ^ 0x1;
                        c.g[V1][0] = c.g[V1][0] & 0x1;
                        if c.g[V1][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_277048;
                        } else {
                            'b_276fc8: {
                                c.cycles += 4;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
                                let t = (c.g[A1][0] as i64) >= 0;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                                if t {
                                    break 'b_276fc8;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x00277450,
                                        f: ee::s07::ScriptList_PrepareEntry,
                                    }
                                    .call(c, 0x00276fc0)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_277048;
                                    } else {
                                        break 'b_276fc8;
                                    }
                                }
                            }
                            'b_277034: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = 0x1;
                                Callee {
                                    addr: 0x00277450,
                                    f: ee::s07::ScriptList_PrepareEntry,
                                }
                                .call(c, 0x00276fd4)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
                                c.g[S2][0] = c.g[V0][0];
                                let t = c.g[S2][0] == c.g[A1][0];

                                if t {
                                    break 'b_277034;
                                } else {
                                    'b_277030: {
                                        'b_27701c: {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00283d70,
                                                f: ee::s07::ScriptCmdQueue_At,
                                            }
                                            .call(c, 0x00276fec)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00277dd8,
                                                f: ee::s07::ScriptList_IsMovieEntry,
                                            }
                                            .call(c, 0x00276ffc)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                break 'b_27701c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                Callee {
                                                    addr: 0x00282540,
                                                    f: ee::s07::ScriptEngine_KillProcess,
                                                }
                                                .call(c, 0x00277010)?;
                                                c.g[V1][0] = 0xffffffffffffffff;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_277030;
                                                } else {
                                                    break 'b_27701c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if (c.g[V0][0] as i64) < 0 {
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xbc),
                                                c.g[S2][0],
                                            );
                                            break 'b_277034;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                            Callee {
                                                addr: 0x00283818,
                                                f: ee::s07::ScriptMovie_Stop,
                                            }
                                            .call(c, 0x00277030)?;
                                            break 'b_277030;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[S2][0]);
                                    break 'b_277034;
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[S2][0] as i64) < 0;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_277048;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00277800,
                                    f: ee::s07::ScriptList_TryAdvance,
                                }
                                .call(c, 0x00277044)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_277048;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_27704c;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `GameCamera* GameCamera_ctor(GameCamera* self, int a1)` (0x15dc70): construct the
/// camera base, then allocate and construct the manual (`+0xa4`), chase (`+0xa8`)
/// and script (`+0xac`) controllers and clear the camera state; returns `self`.
#[allow(unreachable_code)]
pub fn game_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        Callee {
            addr: 0x0015d698,
            f: ee::s01::CameraBase_ctor,
        }
        .call(c, 0x0015dc8c)?;
        c.g[V1][0] = 0x460000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4778));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x90), c.g[V1][0]);
        call_ptr(c, 0x00176d68, 0x0015dca0)?;
        c.g[A1][0] = 0x460000;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48b8));
        c.g[A0][0] = 0x20;
        call_ptr(c, 0x00317d70, 0x0015dcb8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0015cd80,
            f: ee::s01::ManualCamController_ctor,
        }
        .call(c, 0x0015dcc4)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48a0));
        c.g[A0][0] = 0x44;
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0015dce0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x001619a8,
            f: ee::s01::ChaseCamController_ctor,
        }
        .call(c, 0x0015dcf0)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4888));
        c.g[A0][0] = 0x20;
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0015dd0c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00168fe8, 0x0015dd1c)?;
        c.g[V1][0] = 0xffffffffffffffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xac), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb0), c.g[V1][0]);
        c.g[A0][0] = 0x500000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), 0u64);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xea0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x454), 0u64);
        c.g[A1][0] = 0x500000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x458), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xed0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x45c), 0u64);
        c.g[A2][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x450), 0u64);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x460), 0u64);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(3, (c.g[A0][0] as u32));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x470));
        c.lq(4, (c.g[A1][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4a4), c.g[A2][0]);
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0x490));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4a0), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `AI* AI_ctor(AI* self)` (0x1286a0): store the AI tuning constants, allocate
/// the 0x8e0-byte rider table (first word cleared, five entries reset) into
/// `+0xa4`, then allocate and construct the `DangerMan` (`+0xa8`); returns `self`.
#[allow(unreachable_code)]
pub fn ai_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[A2][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = 0x460000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7a60));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ab0));
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 0x460000;
        c.g[T0][0] = 0x460000;
        c.g[T1][0] = 0x460000;
        c.g[T2][0] = 0x460000;
        c.g[T3][0] = 0x460000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7a38));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7a88));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b78));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7ad8));
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7b00));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7b28));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb0), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), c.g[A3][0]);
        c.g[A1][0] = 0x450000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xac), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f00));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[T0][0]);
        c.g[A0][0] = 0x8e0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[T1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[T2][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[T3][0]);
        call_ptr(c, 0x00317d70, 0x00128730)?;
        c.g[T0][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffffffffff;
        c.sw((c.g[T0][0] as u32), 0u64);
        c.g[V0][0] = 0x5;
        'l_128740: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A1][0] = 0x450000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00128740);
                }
                continue 'l_128740;
            } else {
                c.cycles += 22;
                c.sw((c.g[S1][0] as u32), c.g[T0][0]);
                c.g[V1][0] = 0x1;
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
                c.g[A2][0] = 0u64;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xd4), c.g[V1][0]);
                c.g[A3][0] = 0u64;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xd8), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f10));
                c.g[A0][0] = 0xb60;
                call_ptr(c, 0x00317d70, 0x00128788)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x00101310,
                    f: ee::s00::DangerMan_ctor,
                }
                .call(c, 0x00128794)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
                c.g[V0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0u64);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void ScriptEngine_ListStop(ScriptEngine* self, int i)` (0x2790a0): stop NIS list
/// `i` (`ScriptList_Stop(lists + i * 0xcc, 1)`), and leave script mode when it was
/// the playing list.
#[allow(unreachable_code)]
pub fn script_engine_list_stop(c: &mut Cpu) {
    guest(c, |c| {
        'b_2790e8: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = 0xcc;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.mult(0, 2, 16, 2);
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[A1][0] = 0x1;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            Callee {
                addr: 0x00276868,
                f: ee::s07::ScriptList_Stop,
            }
            .call(c, 0x002790d0)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x550));
            let t = c.g[V0][0] != c.g[S0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_2790e8;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0027bf90,
                    f: ee::s07::ScriptEngine_ExitScriptMode,
                }
                .call(c, 0x002790e4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_2790e8;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `WScriptMan* WScriptMan_ctor(WScriptMan* self)` (0x3087d0): construct the command
/// queue, the three process lists and the collect sets (the inner 0x40-word tables
/// filled with -1), register `self` in the global, clear the 200-byte block at
/// `+0x3cc` and `WScriptMan_Reset(self)`; returns `self`.
#[allow(unreachable_code)]
pub fn w_script_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = 0xffffffffffff0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[S0][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0xffffffffffff0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3cc));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S4][0] = 0xffffffffffffffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
        Callee {
            addr: 0x0030c738,
            f: ee::s10::WScriptMan_CmdQueueCtor,
        }
        .call(c, 0x00308818)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6400));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x494), c.g[V0][0]);
        Callee {
            addr: 0x0030b058,
            f: ee::s10::WScriptProcessList_ctor,
        }
        .call(c, 0x0030882c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
        Callee {
            addr: 0x0030b058,
            f: ee::s10::WScriptProcessList_ctor,
        }
        .call(c, 0x00308834)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
        Callee {
            addr: 0x0030b058,
            f: ee::s10::WScriptProcessList_ctor,
        }
        .call(c, 0x0030883c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c0));
        c.g[V0][0] = 0u64;
        'l_308848: loop {
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10c));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x3f;
            'l_308858: loop {
                c.cycles += 7;
                c.sw((c.g[V0][0] as u32), c.g[S3][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = c.g[V1][0] != c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00308858);
                    }
                    continue 'l_308858;
                } else {
                    c.cycles += 5;
                    Callee {
                        addr: 0x0030c390,
                        f: ee::s10::WScriptMan_ResetCollectSet,
                    }
                    .call(c, 0x0030887c)?;
                    c.g[V0][0] = c.g[S2][0];
                    let t = c.g[V0][0] != c.g[S4][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00308848);
                        }
                        continue 'l_308848;
                    } else {
                        c.cycles += 19;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                        c.g[A0][0] = c.g[S5][0];
                        c.sw((c.g[GP][0] as u32).wrapping_add(0xce8), c.g[S0][0]);
                        c.g[A1][0] = 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x28c), c.g[V0][0]);
                        c.g[A2][0] = 0xc8;
                        call_ptr(c, 0x003e6448, 0x003088a4)?;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00308988,
                            f: ee::s10::WScriptMan_Reset,
                        }
                        .call(c, 0x003088ac)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}

/// `void ScriptEngine_LoadBankSync(ScriptEngine* self, int bank, const char* name,
/// int memFlags, int key, char* mem, int memSize)` (0x27b268): load a script bank
/// synchronously (from `mem` with `FILE_loadatz`, or `FILE_loadz` plus
/// `BXFILE_size`) and attach the data to the bank slot; a slot that is in use is
/// left alone, a failed load clears the slot.
#[allow(unreachable_code)]
pub fn script_engine_load_bank_sync(c: &mut Cpu) {
    guest(c, |c| {
        'b_27b348: {
            'b_27b344: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S6][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 4);
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x628));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A1][0] = c.g[A3][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = c.g[T1][0];
                if t {
                    break 'b_27b344;
                } else {
                    'b_27b314: {
                        'b_27b304: {
                            'b_27b2fc: {
                                'b_27b2dc: {
                                    c.cycles += 3;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                                    let t = c.g[S3][0] == 0u64;
                                    c.g[S5][0] = 0u64;
                                    if t {
                                        break 'b_27b2dc;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] = 0u64;
                                        call_ptr(c, 0x003e1bb8, 0x0027b2d4)?;
                                        let t = true;
                                        if c.g[V0][0] != 0 {
                                            c.g[S0][0] = c.g[S3][0];
                                        }
                                        if t {
                                            break 'b_27b2fc;
                                        } else {
                                            break 'b_27b2dc;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x003e1908, 0x0027b2e4)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;

                                if t {
                                    break 'b_27b304;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x00317f98, 0x0027b2f8)?;
                                    c.g[S5][0] = c.g[V0][0];
                                    break 'b_27b2fc;
                                }
                            }
                            c.cycles += 2;
                            if c.g[S0][0] != 0u64 {
                                c.sw((c.g[S4][0] as u32).wrapping_add(0xa48), c.g[S6][0]);
                                break 'b_27b314;
                            } else {
                                break 'b_27b304;
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00277f08,
                            f: ee::s07::ScriptBankSlot_Clear,
                        }
                        .call(c, 0x0027b30c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        if t {
                            break 'b_27b348;
                        } else {
                            break 'b_27b314;
                        }
                    }
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x51c));
                    Callee {
                        addr: 0x00275920,
                        f: ee::s07::DataInstanceMan_AddData,
                    }
                    .call(c, 0x0027b328)?;
                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0xa;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xa48), c.g[V0][0]);
                    let t = c.g[S3][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    if t {
                        break 'b_27b344;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                        break 'b_27b344;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_27b348;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `cPS2ChunkResolve* cPS2ChunkResolve_ctor(cPS2ChunkResolve* self)` (0x3aa2f0): set
/// the vtable and the shared static table (cleared once, 24 entries of
/// `{-1, 1000.0, 0.5}`); returns `self`.
#[allow(unreachable_code)]
pub fn c_ps2_chunk_resolve_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_3aa33c: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = 0x490000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5150));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = 0x510000;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2978));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            let t = c.g[V1][0] != 0u64;
            c.sw((c.g[S1][0] as u32), 0u64);
            if t {
                break 'b_3aa33c;
            } else {
                c.cycles += 6;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x70b0));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A0][0] = 0x1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2978), c.g[A0][0]);
                break 'b_3aa33c;
            }
        }
        c.cycles += 21;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x70b0));
        c.g[A1][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x130;
        call_ptr(c, 0x003e6448, 0x003aa354)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = 0x510000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A0][0] = 0xffffffffffffffff;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x70a8), c.g[V0][0]);
        c.g[A1][0] = 0xffffffffffffffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
        c.g[V1][0] = c.g[S0][0];
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 1);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x120));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.sw((c.g[S2][0] as u32).wrapping_sub(0x70b0), 0u64);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x10));
        'l_3aa390: loop {
            c.cycles += 7;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
            c.sh((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
            c.sh((c.g[V1][0] as u32).wrapping_add(0x1a), c.g[A1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x10));
                if event_due(c) {
                    deliver_event_here(c, 0x003aa390);
                }
                continue 'l_3aa390;
            } else {
                c.cycles += 16;
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x70b0));
                c.g[A0][0] = 0xffffffffffff0000;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A0][0] | 0xfffb;
                c.g[V0][0] = c.g[S1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.g[V1][0] | 0x1;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.g[V1][0] | 0x2;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void MultiParticle_CreateEmitter(MultiParticle* self)` (0x358020): allocate the
/// index table (`count` words, all -1) at `+0xc`, the 0x190-byte emitter block and
/// its object (`+0x10`), clear the table again through the new object and register
/// the emitter with the particle system.
#[allow(unreachable_code)]
pub fn multi_particle_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        'b_35808c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A1][0] = 0x4a0000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4030));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A2][0] = 0x20000000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32));
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x00358058)?;
            c.g[A0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_35808c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                'l_358070: loop {
                    c.cycles += 7;
                    c.sw((c.g[A0][0] as u32), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A2][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00358070);
                        }
                        continue 'l_358070;
                    } else {
                        break 'b_35808c;
                    }
                }
            }
        }
        'b_3580f0: {
            c.cycles += 13;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = 0x190;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4038));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
            call_ptr(c, 0x00317d70, 0x003580a8)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00370018, 0x003580b0)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            c.g[A0][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if t {
                break 'b_3580f0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                'l_3580d0: loop {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x003580d0);
                        }
                        continue 'l_3580d0;
                    } else {
                        break 'b_3580f0;
                    }
                }
            }
        }
        c.cycles += 11;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x500000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
        call_ptr(c, 0x003705e0, 0x00358104)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP, SP};
    use std::ptr::addr_of_mut;

    /// Fixture controls (above the fakes' log, `DATA + 0x200000`).
    const OC433_CTRL: u32 = DATA + 0x20_0000;
    const OC433_MAGIC: u32 = 0x0c43_3c43;
    /// Forced results of fake callees: one 0x40-byte slot per callee address.
    const OC433_FORCE: u32 = OC433_CTRL + 0x100;
    /// Fixture objects live here: above the zeroed region the fakes' pointer
    /// results fall in (`DATA..DATA + 0x40000`).
    const OC433_FIX: u32 = DATA + 0x4_0000;

    /// A 32-bit word of test data.
    fn oc433_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A word at `gp + off`.
    fn oc433_gp(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32((GAME_GP as i32 + off) as u32, v);
    }

    fn oc433_slot(addr: u32) -> u32 {
        OC433_FORCE + ((addr >> 2) % 0x3fd) * 0x40
    }

    /// Force the results (`v0`, sign-extended) of the successive calls of fake
    /// callee `addr` (the last value repeats).
    fn oc433_force(c: &mut Cpu, addr: u32, vals: &[i32]) {
        let s = oc433_slot(addr);
        c.bus.write32(s, vals.len() as u32);
        c.bus.write32(s + 4, 0);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write32(s + 8 + 4 * i as u32, *v as u32);
        }
    }

    /// A fake callee: log (`fakes::fake`), then the forced result if any.
    fn oc433_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let s = oc433_slot(addr);
        let n = c.bus.read32(s);
        if n != 0 {
            let i = c.bus.read32(s + 4);
            c.bus.write32(s + 4, i + 1);
            let v = c.bus.read32(s + 8 + 4 * i.min(n - 1));
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc433_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC433_CTRL) != OC433_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc433_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            fn oc433_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc433_fakes! {
        (oc433_fake_00101310, 0x0010_1310, s00, OVERRIDE_00101310, f_00101310__generated),
        (oc433_fake_001033b0, 0x0010_33b0, s00, OVERRIDE_001033B0, f_001033b0__generated),
        (oc433_fake_00113b10, 0x0011_3b10, s00, OVERRIDE_00113B10, f_00113b10__generated),
        (oc433_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc433_fake_00145dd0, 0x0014_5dd0, s01, OVERRIDE_00145DD0, f_00145dd0__generated),
        (oc433_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc433_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc433_fake_0014de68, 0x0014_de68, s01, OVERRIDE_0014DE68, f_0014de68__generated),
        (oc433_fake_0015cd80, 0x0015_cd80, s01, OVERRIDE_0015CD80, f_0015cd80__generated),
        (oc433_fake_0015cf80, 0x0015_cf80, s01, OVERRIDE_0015CF80, f_0015cf80__generated),
        (oc433_fake_0015d698, 0x0015_d698, s01, OVERRIDE_0015D698, f_0015d698__generated),
        (oc433_fake_0015e030, 0x0015_e030, s01, OVERRIDE_0015E030, f_0015e030__generated),
        (oc433_fake_001619a8, 0x0016_19a8, s01, OVERRIDE_001619A8, f_001619a8__generated),
        (oc433_fake_00161a60, 0x0016_1a60, s01, OVERRIDE_00161A60, f_00161a60__generated),
        (oc433_fake_00162060, 0x0016_2060, s01, OVERRIDE_00162060, f_00162060__generated),
        (oc433_fake_00162310, 0x0016_2310, s01, OVERRIDE_00162310, f_00162310__generated),
        (oc433_fake_00168fe8, 0x0016_8fe8, s02, OVERRIDE_00168FE8, f_00168fe8__generated),
        (oc433_fake_00176d68, 0x0017_6d68, s02, OVERRIDE_00176D68, f_00176d68__generated),
        (oc433_fake_002300f0, 0x0023_00f0, s06, OVERRIDE_002300F0, f_002300f0__generated),
        (oc433_fake_00233aa0, 0x0023_3aa0, s06, OVERRIDE_00233AA0, f_00233aa0__generated),
        (oc433_fake_00234be8, 0x0023_4be8, s06, OVERRIDE_00234BE8, f_00234be8__generated),
        (oc433_fake_00238510, 0x0023_8510, s06, OVERRIDE_00238510, f_00238510__generated),
        (oc433_fake_0026d5e8, 0x0026_d5e8, s07, OVERRIDE_0026D5E8, f_0026d5e8__generated),
        (oc433_fake_0026f228, 0x0026_f228, s07, OVERRIDE_0026F228, f_0026f228__generated),
        (oc433_fake_0026f7b8, 0x0026_f7b8, s07, OVERRIDE_0026F7B8, f_0026f7b8__generated),
        (oc433_fake_00272cc0, 0x0027_2cc0, s07, OVERRIDE_00272CC0, f_00272cc0__generated),
        (oc433_fake_00275920, 0x0027_5920, s07, OVERRIDE_00275920, f_00275920__generated),
        (oc433_fake_00276868, 0x0027_6868, s07, OVERRIDE_00276868, f_00276868__generated),
        (oc433_fake_00277450, 0x0027_7450, s07, OVERRIDE_00277450, f_00277450__generated),
        (oc433_fake_00277598, 0x0027_7598, s07, OVERRIDE_00277598, f_00277598__generated),
        (oc433_fake_002776e0, 0x0027_76e0, s07, OVERRIDE_002776E0, f_002776e0__generated),
        (oc433_fake_00277778, 0x0027_7778, s07, OVERRIDE_00277778, f_00277778__generated),
        (oc433_fake_00277dd8, 0x0027_7dd8, s07, OVERRIDE_00277DD8, f_00277dd8__generated),
        (oc433_fake_00277f08, 0x0027_7f08, s07, OVERRIDE_00277F08, f_00277f08__generated),
        (oc433_fake_00279370, 0x0027_9370, s07, OVERRIDE_00279370, f_00279370__generated),
        (oc433_fake_0027bf90, 0x0027_bf90, s07, OVERRIDE_0027BF90, f_0027bf90__generated),
        (oc433_fake_0027cc28, 0x0027_cc28, s07, OVERRIDE_0027CC28, f_0027cc28__generated),
        (oc433_fake_0027d0c0, 0x0027_d0c0, s07, OVERRIDE_0027D0C0, f_0027d0c0__generated),
        (oc433_fake_0027d2e8, 0x0027_d2e8, s07, OVERRIDE_0027D2E8, f_0027d2e8__generated),
        (oc433_fake_00281a50, 0x0028_1a50, s07, OVERRIDE_00281A50, f_00281a50__generated),
        (oc433_fake_00282540, 0x0028_2540, s07, OVERRIDE_00282540, f_00282540__generated),
        (oc433_fake_00282798, 0x0028_2798, s07, OVERRIDE_00282798, f_00282798__generated),
        (oc433_fake_00283430, 0x0028_3430, s07, OVERRIDE_00283430, f_00283430__generated),
        (oc433_fake_00283440, 0x0028_3440, s07, OVERRIDE_00283440, f_00283440__generated),
        (oc433_fake_00283818, 0x0028_3818, s07, OVERRIDE_00283818, f_00283818__generated),
        (oc433_fake_00283d28, 0x0028_3d28, s07, OVERRIDE_00283D28, f_00283d28__generated),
        (oc433_fake_00283d70, 0x0028_3d70, s07, OVERRIDE_00283D70, f_00283d70__generated),
        (oc433_fake_00288ae0, 0x0028_8ae0, s07, OVERRIDE_00288AE0, f_00288ae0__generated),
        (oc433_fake_0028b180, 0x0028_b180, s07, OVERRIDE_0028B180, f_0028b180__generated),
        (oc433_fake_002ad5f0, 0x002a_d5f0, s08, OVERRIDE_002AD5F0, f_002ad5f0__generated),
        (oc433_fake_00308988, 0x0030_8988, s10, OVERRIDE_00308988, f_00308988__generated),
        (oc433_fake_00308c60, 0x0030_8c60, s10, OVERRIDE_00308C60, f_00308c60__generated),
        (oc433_fake_00308f38, 0x0030_8f38, s10, OVERRIDE_00308F38, f_00308f38__generated),
        (oc433_fake_00309e50, 0x0030_9e50, s10, OVERRIDE_00309E50, f_00309e50__generated),
        (oc433_fake_0030b058, 0x0030_b058, s10, OVERRIDE_0030B058, f_0030b058__generated),
        (oc433_fake_0030b928, 0x0030_b928, s10, OVERRIDE_0030B928, f_0030b928__generated),
        (oc433_fake_0030c390, 0x0030_c390, s10, OVERRIDE_0030C390, f_0030c390__generated),
        (oc433_fake_0030c738, 0x0030_c738, s10, OVERRIDE_0030C738, f_0030c738__generated),
        (oc433_fake_00316898, 0x0031_6898, s10, OVERRIDE_00316898, f_00316898__generated),
        (oc433_fake_00317670, 0x0031_7670, s10, OVERRIDE_00317670, f_00317670__generated),
        (oc433_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc433_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc433_fake_00326c60, 0x0032_6c60, s10, OVERRIDE_00326C60, f_00326c60__generated),
        (oc433_fake_00370018, 0x0037_0018, s12, OVERRIDE_00370018, f_00370018__generated),
        (oc433_fake_003705e0, 0x0037_05e0, s12, OVERRIDE_003705E0, f_003705e0__generated),
        (oc433_fake_0039f840, 0x0039_f840, s13, OVERRIDE_0039F840, f_0039f840__generated),
        (oc433_fake_003a8528, 0x003a_8528, s13, OVERRIDE_003A8528, f_003a8528__generated),
        (oc433_fake_003a9800, 0x003a_9800, s13, OVERRIDE_003A9800, f_003a9800__generated),
        (oc433_fake_003a9858, 0x003a_9858, s13, OVERRIDE_003A9858, f_003a9858__generated),
        (oc433_fake_003e6448, 0x003e_6448, s14, OVERRIDE_003E6448, f_003e6448__generated),
        (oc433_fake_003e6574, 0x003e_6574, s14, OVERRIDE_003E6574, f_003e6574__generated),
        (oc433_fake_00417828, 0x0041_7828, s15, OVERRIDE_00417828, f_00417828__generated),
    }

    /// Run `hand` against the generated body (`generated`, entered at `entry`)
    /// over `seeds` fixtures: scrambled registers, the gated fakes on, the
    /// BELibrary dummy set, and `setup` for the function's own data. `calls`
    /// is the most fake calls a path makes (the unwind index is drawn below it).
    fn oc433_run(
        generated: GuestFn,
        hand: GuestFn,
        entry: u32,
        calls: u32,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, &mut Rng, u32),
    ) {
        oc433_install_fakes();
        fakes::covered(generated, hand, seeds, &|c, seed| {
            let mut r = machine(c, seed, entry as u64, calls);
            c.bus.write32(OC433_CTRL, OC433_MAGIC);
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x8000);
            c.pc = entry;
            setup(c, &mut r, seed);
        });
    }

    /// `GameCamera_Init` against the generated body: rider index 0, 1 and 2
    /// (the switch has cases 0 and 1 only), random view parameters; the
    /// controllers are fakes.
    #[test]
    fn game_camera_init_matches() {
        oc433_run(
            ee::s01::f_0015de60__generated,
            game_camera_init,
            0x0015_de60,
            6,
            90,
            &|c, r, seed| {
                let (app, world, riders, this) = (
                    OC433_FIX + 0x1000,
                    OC433_FIX + 0x1200,
                    OC433_FIX + 0x1400,
                    OC433_FIX + 0x2000,
                );
                oc433_gp(c, -0x848, app);
                oc433_w(c, app + 0x84, world);
                oc433_w(c, world + 0xc, riders);
                fill(c, r, app + 0xb0, 4);
                fill(c, r, riders + 0x28, 4);
                oc433_w(c, this + 0xa4, OC433_FIX + 0x2800);
                oc433_w(c, this + 0xa8, OC433_FIX + 0x2a00);
                oc433_w(c, this + 0xac, OC433_FIX + 0x2c00);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3) as u64;
                for k in 12..15 {
                    c.f[k] = r.edge_f32();
                }
            },
        );
    }

    /// `SSXApp_FlushTicks` against the generated body: the pad-queue global
    /// set (the flush call, sometimes unwinding) or clear.
    #[test]
    fn ssx_app_flush_ticks_matches() {
        oc433_run(
            ee::s05::f_00227e68__generated,
            ssx_app_flush_ticks,
            0x0022_7e68,
            1,
            40,
            &|c, _r, seed| {
                oc433_gp(c, -0x850, if seed % 3 == 0 { 0 } else { OC433_FIX });
            },
        );
    }

    /// `World_ResetSections` against the generated body: 0..3 sections (some
    /// resident), each with 0..3 sub-entries pointing at table entries that
    /// are sometimes missing. Dispatcher-shaped: entered at its start.
    #[test]
    fn world_reset_sections_matches() {
        oc433_run(
            ee::s13::f_003a6800__generated,
            world_reset_sections,
            0x003a_6800,
            7,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, a, b, tbl, secs) =
                    (f + 0x1000, f + 0x1100, f + 0x1200, f + 0x1300, f + 0x2000);
                oc433_w(c, this, a);
                oc433_w(c, a, b);
                oc433_w(c, a + 8, tbl);
                oc433_w(c, b + 0x14, secs);
                let count = seed % 4;
                let mut resident = Vec::new();
                for i in 0..3u32 {
                    let sub = (r.next_u32() % 4) as u16;
                    c.bus.write16(secs + 0x58 * i + 0x26, sub);
                    resident.push(((seed >> (2 + i)) & 1) as i32 | (i == 0) as i32);
                    let p = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        f + 0x3000 + 0x100 * i
                    };
                    oc433_w(c, tbl + 4 * i, p);
                    oc433_w(c, f + 0x3000 + 0x100 * i + 0x1c, f + 0x4000 + 0x100 * i);
                    for s in 0..3u32 {
                        let ptr = f + 0x5000 + 0x40 * (i * 3 + s);
                        let x = if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            (ptr >> 2) << 8
                        };
                        oc433_w(c, f + 0x4000 + 0x100 * i + 4 * s, x);
                        oc433_w(c, ptr + 8, r.next_u32());
                    }
                }
                oc433_force(c, 0x003a_9800, &[count as i32]);
                oc433_force(c, 0x003a_9858, &resident);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `WScriptFn_DefineCollectSet` against the generated body: first use or
    /// not, 0..3 (or a negative count of) entries of the three kinds (matching
    /// value, plain store, float store), the first slot -1 or not.
    #[test]
    fn w_script_fn_define_collect_set_matches() {
        oc433_run(
            ee::s10::f_00300e28__generated,
            w_script_fn_define_collect_set,
            0x0030_0e28,
            2,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (out, entries) = (f + 0x1000, f + 0x1100);
                oc433_gp(c, 0x25bc, seed % 2);
                oc433_gp(c, 0xce8, f + 0x1400);
                let table = [0u32, 2, 3];
                for (i, t) in table.iter().enumerate() {
                    oc433_gp(c, 0xb70 + 4 * i as i32, *t);
                }
                let n = [-1i32, 0, 1, 2, 3, 3][(seed / 2 % 6) as usize];
                for e in 0..3u32 {
                    let idx = if e == 0 {
                        (seed / 12) % 3
                    } else {
                        r.next_u32() % 3
                    };
                    let at = entries + 0x10 * e;
                    oc433_w(c, at, idx);
                    let value = if seed % 5 == 0 && e == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 50
                    };
                    oc433_w(c, at + 4, value);
                    let t = table[idx as usize];
                    oc433_w(c, at + 0xc, if r.next_u32() % 2 == 0 { t } else { t + 1 });
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `ScriptAudioObj_Deactivate` against the generated body: 0..3 audio
    /// events, the audio flag, four handle triples (some negative), and the
    /// audio-fields reader's flag word.
    #[test]
    fn script_audio_obj_deactivate_matches() {
        oc433_run(
            ee::s07::f_002807b0__generated,
            script_audio_obj_deactivate,
            0x0028_07b0,
            14,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let this = f + 0x1000;
                fill(c, r, this + 0x1c, 8);
                for k in 0..12u32 {
                    let v = if k % 3 == 0 && r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 9
                    };
                    oc433_w(c, this + 0xbc + 4 * k, v);
                }
                oc433_w(c, this + 0x8, f + 0x1800);
                oc433_w(
                    c,
                    this + 0xec,
                    [-1i32, 0, 1, 2, 3, 3][(seed % 6) as usize] as u32,
                );
                oc433_w(c, this + 0xf0, (seed / 6) % 2);
                oc433_gp(c, 0xce8, f + 0x1900);
                oc433_w(c, SP - 0xa0 + 0x24, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `GameStateReset_Exit` against the generated body: mode byte 0, 5, 6, 7,
    /// the "already reset" byte set or not, and the single-player check (the
    /// first rider's word cleared only when the world's slot holds 1).
    #[test]
    fn game_state_reset_exit_matches() {
        oc433_run(
            ee::s06::f_00235868__generated,
            game_state_reset_exit,
            0x0023_5868,
            12,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (app, x, a3, a0, a1, a0b, v) = (
                    f + 0x1000,
                    f + 0x1100,
                    f + 0x1300,
                    f + 0x1400,
                    f + 0x1500,
                    f + 0x1600,
                    f + 0x1700,
                );
                oc433_gp(c, -0x848, app);
                oc433_gp(c, -0x488, x);
                oc433_gp(c, 0xce8, f + 0x1800);
                fill(c, r, x + 0x28, 1);
                oc433_w(c, app + 0xc0, f + 0x1900);
                oc433_w(c, app + 0x84, a3);
                oc433_w(c, a3 + 0x84, a0);
                oc433_w(c, a3 + 0x10, a0b);
                oc433_w(c, a0 + 0x10, [1u32, 1, 0][(seed / 4 % 3) as usize]);
                oc433_w(c, a0 + 4, a1);
                oc433_w(c, a1 + 0xb0, (seed / 12) % 3);
                oc433_w(c, a0b, v);
                c.sb(0x0053_5bc8 + 0x49, (seed % 2) as u64);
                c.sb(0x0053_5bc8 + 0x48, [0u64, 5, 6, 7][(seed / 2 % 4) as usize]);
                c.g[A0][0] = sx(f + 0x2000);
            },
        );
    }

    /// `ScriptViewObj_Activate` against the generated body: the sequence
    /// playing or not (forced result), unwinds in every call.
    #[test]
    fn script_view_obj_activate_matches() {
        oc433_run(
            ee::s07::f_00281370__generated,
            script_view_obj_activate,
            0x0028_1370,
            7,
            60,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, engine, app, world, rider_tab) =
                    (f + 0x1000, f + 0x1100, f + 0x1200, f + 0x1300, f + 0x1400);
                oc433_gp(c, -0x84c, engine);
                oc433_gp(c, -0x848, app);
                oc433_w(c, app + 0x84, world);
                oc433_w(c, world + 0xc, rider_tab);
                fill(c, r, rider_tab + 0xa4, 1);
                oc433_w(c, this + 4, r.next_u32());
                oc433_force(c, 0x0027_9370, &[(seed % 2) as i32]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `cWorldCache_FlushHullPages` against the generated body: 0..3 slots
    /// with states 0..5, in-use flags and ages around the threshold.
    #[test]
    fn c_world_cache_flush_hull_pages_matches() {
        oc433_run(
            ee::s13::f_003a88a8__generated,
            c_world_cache_flush_hull_pages,
            0x003a_88a8,
            3,
            200,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (cache, this) = (f + 0x1000, f + 0x2000);
                oc433_gp(c, 0x2a74, cache);
                oc433_w(c, cache + 0x1c, r.next_u32() % 6);
                let n = seed % 4;
                oc433_w(c, this + 0x3ec, n);
                for i in 0..3u32 {
                    let e = this + 0x3f0 + 0x18 * i;
                    oc433_w(c, e, r.next_u32() % 6);
                    oc433_w(c, e + 4, r.next_u32() % 2);
                    oc433_w(c, e + 0x10, r.next_u32() % 6);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `ScriptList_Prepare` against the generated body: list state 0, 1 or 2;
    /// empty or filled queue; the first and second entry preparation failing
    /// or not; movie and script entries (loading or ready).
    #[test]
    fn script_list_prepare_matches() {
        oc433_run(
            ee::s07::f_00276270__generated,
            script_list_prepare,
            0x0027_6270,
            9,
            240,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, engine, entry) = (f + 0x1000, f + 0x1100, f + 0x1800);
                oc433_w(c, this, engine);
                oc433_w(c, this + 0xa4, [0u32, 0, 0, 0, 1, 2][(seed % 6) as usize]);
                oc433_w(
                    c,
                    entry + 0xc,
                    if (seed / 6) % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 9
                    },
                );
                oc433_force(c, 0x0028_3d28, &[((seed / 6) % 5 != 1) as i32]);
                oc433_force(
                    c,
                    0x0027_7450,
                    &[
                        if (seed / 30) % 4 == 0 { -1 } else { 3 },
                        if (seed / 7) % 2 == 0 { -1 } else { 4 },
                    ],
                );
                oc433_force(c, 0x0028_3d70, &[entry as i32]);
                oc433_force(c, 0x0027_7dd8, &[(seed / 2 % 2) as i32]);
                oc433_force(c, 0x0028_2798, &[(seed / 3 % 2) as i32]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `GameStateCountdown_Enter` against the generated body: the race-start
    /// notification object set or not, the scripts-reset flag, and the mode
    /// byte (4..6: go to the next state; others: the countdown audio and
    /// script event, with the replay check forced to "replaying").
    #[test]
    fn game_state_countdown_enter_matches() {
        oc433_run(
            ee::s06::f_00234ad0__generated,
            game_state_countdown_enter,
            0x0023_4ad0,
            17,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (x, y, z, comm, app) =
                    (f + 0x1000, f + 0x1100, f + 0x1200, f + 0x1300, f + 0x1400);
                oc433_gp(c, -0x488, x);
                oc433_w(c, x + 0xc, y);
                fill(c, r, x + 0x28, 1);
                oc433_w(c, x + 0x48, z);
                oc433_gp(c, -0x204, if seed % 2 == 0 { comm } else { 0 });
                oc433_w(c, comm + 0x70, r.next_u32() % 30);
                oc433_gp(c, -0x848, app);
                fill(c, r, app + 0x78, 1);
                oc433_w(c, 0x0053_4b30, (seed / 2) % 2);
                c.sb(0x0053_5c10, [0u64, 4, 5, 6, 7][(seed / 4 % 5) as usize]);
                oc433_force(c, 0x0028_8ae0, &[1]);
                c.g[A0][0] = sx(f + 0x2000);
            },
        );
    }

    /// `cWorldCache_ReleaseAllHulls` against the generated body (entered at
    /// its start, the resume entries go to the generated `__entries`): 0..3
    /// slots with states 0..5 (the jump table over 0..4, 5 is out of range).
    #[test]
    fn c_world_cache_release_all_hulls_matches() {
        oc433_run(
            ee::s13::f_003a8330__generated,
            c_world_cache_release_all_hulls,
            0x003a_8330,
            6,
            200,
            &|c, r, seed| {
                elf_image(c);
                let this = OC433_FIX + 0x2000;
                oc433_w(c, this + 0x3ec, seed % 4);
                oc433_w(c, this + 0x390, 0);
                for i in 0..3u32 {
                    let e = this + 0x3f0 + 0x18 * i;
                    oc433_w(c, e, r.next_u32() % 6);
                    fill(c, r, e + 4, 5);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `ScriptList_SkipCurrent` against the generated body: list state 3 or
    /// not, skip already requested or not, the playing entry skippable or not,
    /// no / same / different pending entry, movie and script pending entries.
    #[test]
    fn script_list_skip_current_matches() {
        oc433_run(
            ee::s07::f_00276f48__generated,
            script_list_skip_current,
            0x0027_6f48,
            6,
            240,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, engine, e1, e2) = (f + 0x1000, f + 0x1100, f + 0x1800, f + 0x1900);
                oc433_w(c, this, engine);
                oc433_w(c, this + 0xa4, if seed % 11 == 0 { 2 } else { 3 });
                oc433_w(c, this + 0xc8, (seed % 13 == 0) as u32);
                oc433_w(c, this + 0xa8, 1);
                let pending = [u32::MAX, 0, 2, 5][(seed / 3 % 4) as usize];
                oc433_w(c, this + 0xbc, pending);
                oc433_w(
                    c,
                    e1 + 8,
                    if seed % 7 == 0 {
                        0
                    } else {
                        1 | (r.next_u32() & 0xe)
                    },
                );
                oc433_w(
                    c,
                    e2 + 0xc,
                    if (seed / 5) % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 9
                    },
                );
                oc433_force(c, 0x0028_3d70, &[e1 as i32, e2 as i32]);
                let prepared = match (seed / 12) % 4 {
                    0 => pending as i32,
                    1 => -1,
                    _ => 4,
                };
                oc433_force(c, 0x0027_7450, &[prepared]);
                oc433_force(c, 0x0027_7dd8, &[(seed / 2 % 2) as i32]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `GameCamera_ctor` against the generated body (constructors and
    /// allocator are fakes).
    #[test]
    fn game_camera_ctor_matches() {
        oc433_run(
            ee::s01::f_0015dc70__generated,
            game_camera_ctor,
            0x0015_dc70,
            8,
            60,
            &|c, r, _seed| {
                c.g[A0][0] = sx(OC433_FIX + 0x2000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `AI_ctor` against the generated body (allocator and `DangerMan_ctor`
    /// are fakes).
    #[test]
    fn ai_ctor_matches() {
        oc433_run(
            ee::s00::f_001286a0__generated,
            ai_ctor,
            0x0012_86a0,
            3,
            60,
            &|c, _r, _seed| {
                c.g[A0][0] = sx(OC433_FIX + 0x2000);
            },
        );
    }

    /// `ScriptEngine_ListStop` against the generated body: the stopped list is
    /// the playing one (script mode left) or not.
    #[test]
    fn script_engine_list_stop_matches() {
        oc433_run(
            ee::s07::f_002790a0__generated,
            script_engine_list_stop,
            0x0027_90a0,
            2,
            60,
            &|c, r, seed| {
                let this = OC433_FIX + 0x1000;
                let i = seed % 4;
                oc433_w(c, this + 0x54c, OC433_FIX + 0x2000);
                oc433_w(
                    c,
                    this + 0x550,
                    if seed % 3 == 0 { r.next_u32() } else { i },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
            },
        );
    }

    /// `WScriptMan_ctor` against the generated body (the sub-object
    /// constructors, memset and reset are fakes).
    #[test]
    fn w_script_man_ctor_matches() {
        oc433_run(
            ee::s10::f_003087d0__generated,
            w_script_man_ctor,
            0x0030_87d0,
            7,
            60,
            &|c, r, _seed| {
                oc433_gp(c, 0x16c8, r.next_u32());
                c.g[A0][0] = sx(OC433_FIX + 0x1000);
            },
        );
    }

    /// `ScriptEngine_LoadBankSync` against the generated body, for a bank
    /// slot that is already in use (the early exit). The loading paths call
    /// `FILE_loadz` / `FILE_loadatz` (the real EA file layer, which reaches the
    /// disc and cannot be bounded on fixture data), so they are not covered by
    /// this test.
    #[test]
    fn script_engine_load_bank_sync_matches() {
        check(
            ee::s07::f_0027b268__generated,
            script_engine_load_bank_sync,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27b268, 0);
                let this = OC433_FIX + 0x1000;
                let bank = seed % 4;
                oc433_w(c, this + (bank << 4) + 0x628, 1 + (r.next_u32() & 0xffff));
                c.pc = 0x0027_b268;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = bank as u64;
            },
        );
    }

    /// `cPS2ChunkResolve_ctor` against the generated body: first construction
    /// (the static table is built) or a later one.
    #[test]
    fn c_ps2_chunk_resolve_ctor_matches() {
        oc433_run(
            ee::s13::f_003aa2f0__generated,
            c_ps2_chunk_resolve_ctor,
            0x003a_a2f0,
            1,
            60,
            &|c, _r, seed| {
                oc433_gp(c, 0x2978, seed % 2);
                c.g[A0][0] = sx(OC433_FIX + 0x1000);
            },
        );
    }

    /// `MultiParticle_CreateEmitter` against the generated body: 0..4
    /// emitters (an empty table allocates nothing to fill).
    #[test]
    fn multi_particle_create_emitter_matches() {
        oc433_run(
            ee::s11::f_00358020__generated,
            multi_particle_create_emitter,
            0x0035_8020,
            4,
            100,
            &|c, r, seed| {
                let this = OC433_FIX + 0x1000;
                oc433_w(c, this, seed % 5);
                fill(c, r, this + 8, 1);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
