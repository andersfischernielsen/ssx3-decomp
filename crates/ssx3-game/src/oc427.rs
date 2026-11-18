//! Staging module oc427 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc427`.

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
        0x0027_7450,
        script_list_prepare_entry,
        "ScriptList_PrepareEntry",
    ),
    (
        0x0011_72e0,
        rider_metrix_init_hudfx,
        "RiderMetrix_InitHUDFX",
    ),
    (0x0027_4918, in_seq_instance_stop, "InSeq_InstanceStop"),
    (
        0x0027_bb08,
        script_engine_get_locator_pose,
        "ScriptEngine_GetLocatorPose",
    ),
    (0x0027_9a70, script_engine_v10, "ScriptEngine_v10"),
    (
        0x0015_e050,
        game_camera_snap_to_default_chase_view,
        "GameCamera_SnapToDefaultChaseView",
    ),
    (
        0x0035_a328,
        multi_spline_modifier_alloc_arrays,
        "MultiSplineModifier_AllocArrays",
    ),
    (
        0x0030_db70,
        rider_model_add_model_ref,
        "RiderModel_AddModelRef",
    ),
    (
        0x0030_c4a8,
        wscript_man_collect_set_add_items,
        "WScriptMan_CollectSetAddItems",
    ),
    (
        0x0028_1c68,
        script_view_obj_cut_to_shot,
        "ScriptViewObj_CutToShot",
    ),
    (
        0x0032_4398,
        input_parser_parse_function1,
        "InputParser_ParseFunction1",
    ),
    (
        0x0015_3708,
        be_mission_int_count_collected_bits_on_peak,
        "BEMissionInt_CountCollectedBitsOnPeak",
    ),
    (0x0023_a108, gm_handler1_reset, "GMHandler1_Reset"),
];

/// `int ScriptList::PrepareEntry(this, int start, bool skip_flagged)` (0x277450,
/// `tNISList::SetupNextScript`): find the first entry of the script list's command
/// queue (`this + 0xc`) from index `start` that is ready to play and return its
/// index, or -1. Entries with flag 2 (+0x8) are skipped when `skip_flagged`; an
/// entry whose script id (+0xc) is already non-negative is ready. A script
/// entry requests its script from the engine (`*this`) by index (+0x4) or, with
/// a negative index, by category (+0x0), with the entry's parameters
/// (+0x10, +0x14, +0x18); the id it gets back is stored in +0xc. A movie entry
/// opens its video on the engine's movie player (+0x500) when that one is idle,
/// clearing the id when it opened.
pub fn script_list_prepare_entry(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00277450);
        'b_277578: {
            'b_277574: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S1][0] = c.g[S0][0];
                Callee {
                    addr: 0x00283d28,
                    f: ee::s07::ScriptCmdQueue_Count,
                }
                .call(c, 0x00277484)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_277574;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                    'l_277498: loop {
                        'b_277564: {
                            'b_2774b8: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00283d70,
                                    f: ee::s07::ScriptCmdQueue_At,
                                }
                                .call(c, 0x002774a0)?;
                                let t = c.g[S4][0] == 0u64;
                                c.g[S0][0] = c.g[V0][0];
                                if t {
                                    break 'b_2774b8;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.g[V0][0] & 0x2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        break 'b_277564;
                                    } else {
                                        break 'b_2774b8;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.g[V0][0] = c.g[S1][0];
                            if t {
                                break 'b_277574;
                            } else {
                                'b_277560: {
                                    'b_277528: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00277dd8,
                                            f: ee::s07::ScriptList_IsMovieEntry,
                                        }
                                        .call(c, 0x002774d0)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                            break 'b_277528;
                                        } else {
                                            'b_277514: {
                                                'b_2774fc: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    let t = (c.g[A1][0] as i64) < 0;
                                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                    if t {
                                                        break 'b_2774fc;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        Callee {
                                                            addr: 0x002795d0,
                                                            f: ee::s07::ScriptEngine_SpawnScript,
                                                        }
                                                        .call(c, 0x002774f4)?;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xc),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_277514;
                                                        } else {
                                                            break 'b_2774fc;
                                                        }
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                                c.g[A2][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[T0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                Callee {
                                                    addr: 0x00279528,
                                                    f: ee::s07::ScriptEngine_SpawnChosenScript,
                                                }
                                                .call(c, 0x00277510)?;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc),
                                                    c.g[V0][0],
                                                );
                                                break 'b_277514;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = c.g[S1][0];
                                            if t {
                                                break 'b_277560;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_277578;
                                                } else {
                                                    break 'b_277528;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x500));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_277574;
                                    } else {
                                        'b_277558: {
                                            c.cycles += 9;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00277de8,
                                                f: ee::s07::ScriptList_GetMovieInfo,
                                            }
                                            .call(c, 0x00277540)?;
                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                            Callee {
                                                addr: 0x002838e8,
                                                f: ee::s07::ScriptMovie_Open,
                                            }
                                            .call(c, 0x00277550)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                                break 'b_277558;
                                            } else {
                                                break 'b_277558;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_277574;
                                        } else {
                                            break 'b_277560;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                break 'b_277564;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00277498);
                            }
                            continue 'l_277498;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0xffffffffffffffff;
                            break 'b_277574;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_277578;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void RiderMetrix::InitHUDFX(this, Rider* rider)` (0x1172e0): remember the
/// rider (+0x1ac); when its HUD slot (+0x870) is not negative allocate 44
/// `cHUDFX` entries of 0x9c bytes (0x1ad0, pool tag at 0x4a1078, +0x1b0 holds
/// the array, +0x1b4 the count), construct each and number them
/// (`index` at +0x98).
pub fn rider_metrix_init_hudfx(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x001172e0);
        'b_117390: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1ac), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x870));
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0x2c;
            if t {
                break 'b_117390;
            } else {
                c.cycles += 13;
                c.g[A1][0] = 0x4a0000;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1078));
                c.g[A0][0] = 0x1ad0;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b0));
                call_ptr(c, 0x00317e30, 0x00117334)?;
                c.g[S2][0] = 0x2b;
                c.g[S3][0] = c.g[V0][0];
                c.g[S4][0] = 0xffffffffffffffff;
                c.g[S0][0] = c.g[S3][0];
                'l_117348: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00116fa8,
                        f: ee::s00::cHUDFX_ctor,
                    }
                    .call(c, 0x00117354)?;
                    let t = c.g[S2][0] != c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00117348);
                        }
                        continue 'l_117348;
                    } else {
                        c.cycles += 4;
                        c.sw((c.g[S5][0] as u32), c.g[S3][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b4));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_117390;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = 0u64;
                            'l_117370: loop {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b0));
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[V1][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b4));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00117370);
                                    }
                                    continue 'l_117370;
                                } else {
                                    break 'b_117390;
                                }
                            }
                        }
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
        c.pc = t;
        return Ok(());
    })
}

/// `void InSeq::InstanceStop(this)` (0x274918, replay cache script player): does
/// nothing when the play state (+0x20) is 0; when an entity update is running
/// (+0x3c) only flags a deferred stop (+0x40 = 1); otherwise stops each
/// entity player (the header's byte +1 entries of 0x3c bytes at +0x18), clears
/// the play state, frame, pause and step counters (+0x20..+0x38, the
/// script length and jump frame to -1) and tells the in-sequence listener
/// (`GetListener`) through its vtable slot +0x20/+0x24.
pub fn in_seq_instance_stop(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00274918);
        'b_2749d4: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_2749d4;
            } else {
                'b_2749d0: {
                    'b_274950: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_274950;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                            if t {
                                break 'b_2749d0;
                            } else {
                                break 'b_274950;
                            }
                        }
                    }
                    'b_27498c: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_27498c;
                        } else {
                            c.cycles += 2;
                            c.g[S2][0] = 0u64;
                            'l_274968: loop {
                                c.cycles += 9;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                Callee {
                                    addr: 0x00273dc0,
                                    f: ee::s07::InSeq_ChannelStop,
                                }
                                .call(c, 0x00274978)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00274968);
                                    }
                                    continue 'l_274968;
                                } else {
                                    break 'b_27498c;
                                }
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    Callee {
                        addr: 0x00274d70,
                        f: ee::s07::InSeq_InstanceGetListener,
                    }
                    .call(c, 0x002749b0)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        break 'b_2749d0;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002749d0)?;
                        break 'b_2749d0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_2749d4;
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
    })
}

/// `void ScriptEngine::GetLocatorPose(this, int cat, int slot, Vec4* pos, float* yaw, float* pitch, int* flag, Vec4* off)`
/// (0x27bb08): look up the bone a script locator names. The table of locator
/// codes for category `cat` is at `this + 0x558 + cat * 4` (category 50 maps
/// through the race location, `0x482660[location]`); a missing table or an
/// empty (-1) code leaves the outputs alone. Otherwise the bone of the world
/// model `code & 0xff` gives the position (w = 1), the yaw (`atan2`) and
/// pitch of its axes; `*flag` is cleared and, with an offset, the offset
/// rotated by the bone's matrix is added to the position.
pub fn script_engine_get_locator_pose(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0027bb08);
        'b_27bb90: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.g[V0][0] = 0x32;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S3][0] = c.g[T3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S4][0] = c.g[T1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S5][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[S6][0] = c.g[A3][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
            let t = c.g[T2][0] != c.g[V0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
            if t {
                break 'b_27bb90;
            } else {
                c.cycles += 13;
                call_ptr(c, 0x0014dc80, 0x0027bb64)?;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0027bb70)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00144bc0, 0x0027bb78)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = 0x480000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2660));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[T2][0] = c.lw((c.g[V1][0] as u32));
                break 'b_27bb90;
            }
        }
        'b_27bd44: {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x558));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            if t {
                break 'b_27bd44;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[T2][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = !(0u64 | c.g[T2][0]);
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_27bd44;
                } else {
                    'b_27bc04: {
                        'b_27bbe0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.g[T2][0] & 0xff;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                            let t = c.g[A0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[T2][0] as u32) >> 8);
                            if t {
                                break 'b_27bbe0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[S0][0] = 0u64;
                                if t {
                                    break 'b_27bc04;
                                } else {
                                    break 'b_27bbe0;
                                }
                            }
                        }
                        'b_27bc00: {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_27bc00;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_27bc00;
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[V0][0];
                        break 'b_27bc04;
                    }
                    'b_27bcb8: {
                        'b_27bcb4: {
                            'b_27bc74: {
                                c.cycles += 17;
                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x40));
                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x44));
                                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 3);
                                c.swc1(0, (c.g[SP][0] as u32));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                c.mtc1(0, 22);
                                c.lq(2, (c.g[SP][0] as u32));
                                c.sq(2, (c.g[S2][0] as u32));
                                c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x10));
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_27bc74;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(0, 21);
                                    if t {
                                        break 'b_27bcb4;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47ac));
                                        if !c.fcc() {
                                            c.swc1(0, (c.g[S4][0] as u32));
                                            break 'b_27bcb8;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47a8));
                                            let t = true;
                                            c.swc1(0, (c.g[S4][0] as u32));
                                            if t {
                                                break 'b_27bcb8;
                                            } else {
                                                break 'b_27bc74;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_27bcb0: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                call_ptr(c, 0x0031c228, 0x0027bc80)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_27bcb0;
                                } else {
                                    'b_27bca8: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_27bca8;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47a4));
                                            let t = true;
                                            c.add_s(12, 12, 0);
                                            if t {
                                                break 'b_27bcb0;
                                            } else {
                                                break 'b_27bca8;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47a0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_27bcb0;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(0, 12);
                            break 'b_27bcb4;
                        }
                        c.cycles += 1;
                        c.swc1(0, (c.g[S4][0] as u32));
                        break 'b_27bcb8;
                    }
                    c.cycles += 5;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0031c128, 0x0027bcc0)?;
                    c.swc1(0, (c.g[S5][0] as u32));
                    let t = c.g[S3][0] == 0u64;
                    c.sw((c.g[S6][0] as u32), 0u64);
                    if t {
                        break 'b_27bd44;
                    } else {
                        c.cycles += 30;
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(1, (c.g[V1][0] as u32));
                        c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                        c.sqc2(1, (c.g[A0][0] as u32));
                        c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                        c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.lq(2, (c.g[S3][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                        c.lqc2(8, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[A0][0] as u32));
                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                        c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lqc2(3, (c.g[S2][0] as u32));
                        c.lqc2(4, (c.g[SP][0] as u32));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[S2][0] as u32));
                        break 'b_27bd44;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    })
}

/// `void ScriptEngine::v10(this, Script* script, int idx, int* players)` (0x279a70):
/// choose the variation of every entity player of a script before it starts.
/// Builds the 21 player flag words (`func_0027BDB8`), then for each entity:
/// type-5 entities pick randomly among the options (the candidates) whose slot
/// mask matches the flags (the slot maps through `0x482608`, a used slot's
/// flags are consumed), other entities with at least two variations pick one
/// at random (`BXrand`). Finally counts the script in the master table (`this + 0x530`, 12 bytes per script, count at +0).
pub fn script_engine_v10(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00279a70);
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S5][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A0][0]);
        c.g[FP][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.g[A1][0] = c.g[SP][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
        c.g[A2][0] = c.lw((c.g[A3][0] as u32));
        c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        Callee {
            addr: 0x0027bdb8,
            f: ee::s07::ScriptEngine_EvalChoiceParams,
        }
        .call(c, 0x00279ac0)?;
        c.g[V1][0] = c.g[SP][0];
        c.g[A0][0] = 0x14;
        'l_279ac8: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[S5][0] = c.g[S5][0] | c.g[V0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00279ac8);
                }
                continue 'l_279ac8;
            } else {
                'b_279c8c: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_279c8c;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[FP][0];
                        'l_279af8: loop {
                            'b_279c78: {
                                'b_279c74: {
                                    'b_279c3c: {
                                        c.cycles += 8;
                                        c.g[A1][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00274c20,
                                            f: ee::s07::InSeq_InstanceGetChannel,
                                        }
                                        .call(c, 0x00279b00)?;
                                        c.g[S3][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S3][0];
                                        Callee {
                                            addr: 0x002743c8,
                                            f: ee::s07::InSeq_ChannelGetId,
                                        }
                                        .call(c, 0x00279b0c)?;
                                        c.g[V1][0] = 0x5;
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[A0][0] = c.g[S3][0];
                                        if t {
                                            break 'b_279c3c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            Callee {
                                                addr: 0x00274388,
                                                f: ee::s07::InSeq_ChannelGetHeaderField4,
                                            }
                                            .call(c, 0x00279b20)?;
                                            c.g[S4][0] = c.g[V0][0];
                                            c.g[V0][0] = ((c.g[S4][0] as i64) < 2i64) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                                                break 'b_279c78;
                                            } else {
                                                'b_279bc8: {
                                                    c.cycles += 3;
                                                    c.g[S2][0] = 0u64;
                                                    let t = (c.g[S4][0] as i64) <= 0;
                                                    c.g[S1][0] = 0u64;
                                                    if t {
                                                        break 'b_279bc8;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[V0][0] = 0x480000;
                                                        c.g[S0][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x60));
                                                        c.g[S6][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x2608));
                                                        c.g[A0][0] = c.g[S3][0];
                                                        'l_279b50: loop {
                                                            'b_279bbc: {
                                                                'b_279bb0: {
                                                                    'b_279bac: {
                                                                        'b_279b9c: {
                                                                            c.cycles += 9;
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            Callee { addr: 0x0027c960, f: ee::s07::ScriptRider_GetKeyBlock }.call(c, 0x00279b58)?;
                                                                            c.g[V1][0] = c.g[V0][0];
                                                                            c.g[V0][0] = c
                                                                                .lb((c.g[V1][0]
                                                                                    as u32));
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[S6][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[V0][0] = c
                                                                                .lw((c.g[V0][0]
                                                                                    as u32));
                                                                            if (c.g[V0][0] as i64)
                                                                                < 0
                                                                            {
                                                                                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
                                                                                break 'b_279b9c;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
                                                                                let t = c.g[A1][0]
                                                                                    == 0u64;
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                if t {
                                                                                    break 'b_279bac;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    c.g[V1][0] = c
                                                                                        .lw(
                                                                                        (c.g[A0][0]
                                                                                            as u32),
                                                                                    );
                                                                                    c.g[V1][0] = c
                                                                                        .g[A1][0]
                                                                                        & c.g[V1]
                                                                                            [0];
                                                                                    if c.g[V1][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                        break 'b_279bbc;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.sw((c.g[S0][0] as u32), c.g[S2][0]);
                                                                                        if t {
                                                                                            break 'b_279bb0;
                                                                                        } else {
                                                                                            break 'b_279b9c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] & c.g[S5][0];
                                                                        if t {
                                                                            break 'b_279bac;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0] == 0u64 {
                                                                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                break 'b_279bbc;
                                                                            } else {
                                                                                break 'b_279bac;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.sw(
                                                                        (c.g[S0][0] as u32),
                                                                        c.g[S2][0],
                                                                    );
                                                                    break 'b_279bb0;
                                                                }
                                                                c.cycles += 3;
                                                                c.g[S1][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4));
                                                                c.g[S2][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_279bbc;
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[S2][0] as i64)
                                                                < (c.g[S4][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A0][0] = c.g[S3][0];
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00279b50,
                                                                    );
                                                                }
                                                                continue 'l_279b50;
                                                            } else {
                                                                break 'b_279bc8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                if (c.g[S1][0] as i64) <= 0 {
                                                    c.g[V1][0] = c
                                                        .lw((c.g[FP][0] as u32).wrapping_add(0x10));
                                                    break 'b_279c78;
                                                } else {
                                                    'b_279be4: {
                                                        c.cycles += 5;
                                                        call_ptr(c, 0x003177f0, 0x00279bd8)?;
                                                        c.divu(0, 2, 17);
                                                        if c.g[S1][0] == 0u64 {
                                                            break 'b_279be4;
                                                        } else {
                                                            break 'b_279be4;
                                                        }
                                                    }
                                                    c.cycles += 16;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[V1][0] = c.hi[0];
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[SP][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[A1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x60));
                                                    Callee {
                                                        addr: 0x00273a60,
                                                        f: ee::s07::InSeq_ChannelSelectBlock,
                                                    }
                                                    .call(c, 0x00279bfc)?;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    Callee {
                                                        addr: 0x0027c958,
                                                        f: ee::s07::ScriptRider_GetParams,
                                                    }
                                                    .call(c, 0x00279c04)?;
                                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                                                    c.g[V0][0] = 0x480000;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x2608));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    if t {
                                                        break 'b_279c74;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = sx((c.g[SP][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                        c.sw((c.g[A0][0] as u32), 0u64);
                                                        c.g[V1][0] = !(0u64 | c.g[V1][0]);
                                                        let t = true;
                                                        c.g[S5][0] = c.g[S5][0] & c.g[V1][0];
                                                        if t {
                                                            break 'b_279c74;
                                                        } else {
                                                            break 'b_279c3c;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    Callee {
                                        addr: 0x00274388,
                                        f: ee::s07::InSeq_ChannelGetHeaderField4,
                                    }
                                    .call(c, 0x00279c44)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                                        break 'b_279c78;
                                    } else {
                                        'b_279c68: {
                                            c.cycles += 5;
                                            call_ptr(c, 0x003177f0, 0x00279c5c)?;
                                            c.divu(0, 2, 16);
                                            if c.g[S0][0] == 0u64 {
                                                break 'b_279c68;
                                            } else {
                                                break 'b_279c68;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A1][0] = c.hi[0];
                                        c.g[A0][0] = c.g[S3][0];
                                        Callee {
                                            addr: 0x00273a60,
                                            f: ee::s07::InSeq_ChannelSelectBlock,
                                        }
                                        .call(c, 0x00279c74)?;
                                        break 'b_279c74;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                                break 'b_279c78;
                            }
                            c.cycles += 5;
                            c.g[S0][0] = c.g[S7][0];
                            c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[FP][0];
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00279af8);
                                }
                                continue 'l_279af8;
                            } else {
                                break 'b_279c8c;
                            }
                        }
                    }
                }
                c.cycles += 21;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[V1][0] = 0xc;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x530));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.mult(0, 3, 2, 3);
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void BELibrary::LoadCharDB()` (0x149bb8, `cBELibrary::loadData`): reset
/// the back-end library: the reward DB, the race DB (6 ids), the six rider
/// slots, load `data/be/charDB.dbl` into the character table, build the ten
/// rows of the equipment value table (+5 of the base table), initialise the
/// three profile libraries and fill the grab-point score table with its
/// constants.
/// NOT installed (no `OVERRIDES` entry, no test): its real callees
/// (`RewardsFile_Load`, `EAFile_LoadFileEx`) go through the file queue and
/// `BoltFile_Acquire`, which have no override slot to fake, so a differential
/// fixture cannot be bounded.
pub fn belibrary_load_char_db(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00149bb8);
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S3][0] = 0x530000;
        Callee {
            addr: 0x0014c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x00149be0)?;
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x00149bf0)?;
        c.g[S0][0] = 0x530000;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0x4c;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00416210, 0x00149c08)?;
        c.g[T4][0] = 0u64;
        'l_149c10: loop {
            c.cycles += 7;
            c.sw((c.g[S0][0] as u32), c.g[T4][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T4][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00149c10);
                }
                continue 'l_149c10;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = 0xa8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416210, 0x00149c44)?;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                'l_149c50: loop {
                    c.cycles += 8;
                    c.sb((c.g[S0][0] as u32), 0u64);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x00149c50);
                        }
                        continue 'l_149c50;
                    } else {
                        'b_149cac: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x460000;
                            c.g[A1][0] = 0x100;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d20));
                            call_ptr(c, 0x003e18c8, 0x00149c80)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = 0x530000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x550;
                            call_ptr(c, 0x0041605c, 0x00149c98)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_149cac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x00149ca8)?;
                                c.g[V0][0] = 0x530000;
                                break 'b_149cac;
                            }
                        }
                        c.cycles += 7;
                        c.g[V1][0] = 0x440000;
                        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d8));
                        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf00));
                        c.g[T4][0] = 0u64;
                        c.g[T5][0] = 0xb;
                        c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd));
                        'l_149cc8: loop {
                            c.cycles += 35;
                            c.g[A0][0] = c.lbu((c.g[T2][0] as u32));
                            c.g[A2][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x1));
                            c.g[A3][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5));
                            c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5));
                            c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x6));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5));
                            c.sb((c.g[T1][0] as u32), c.g[T4][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x3), c.g[A3][0]);
                            c.g[V0][0] = ((c.g[T4][0] as i64) < 10i64) as u64;
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x5), c.g[A1][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x4), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x2), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x5), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x1), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_add(0x1), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32), c.g[T5][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0xa), c.g[T5][0]);
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xf));
                            let t = c.g[V0][0] != 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xf));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00149cc8);
                                }
                                continue 'l_149cc8;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                                c.g[A2][0] = 0x288;
                                call_ptr(c, 0x00416210, 0x00149d68)?;
                                c.g[S2][0] = 0x9b50;
                                c.g[V1][0] = 0x4a0000;
                                c.g[V0][0] = 0x10000;
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] = c.g[V0][0] | 0xd1f0;
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.g[S0][0];
                                'l_149d88: loop {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001567b8,
                                        f: ee::s01::BE_ResetProfileCharacters,
                                    }
                                    .call(c, 0x00149d90)?;
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00149d88);
                                        }
                                        continue 'l_149d88;
                                    } else {
                                        c.cycles += 182;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x600));
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x600), 0u64);
                                        c.g[A1][0] = 0xc8;
                                        c.g[V1][0] = 0x190;
                                        c.g[A3][0] = 0x12c;
                                        c.g[T1][0] = 0xfa;
                                        c.g[T0][0] = 0x15e;
                                        c.g[A2][0] = 0x1f4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                                        c.g[A0][0] = 0xbb8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[T1][0]);
                                        c.g[T2][0] = 0x3e8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                        c.g[T0][0] = 0x7d0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
                                        c.g[T3][0] = 0x9c4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                        c.g[T4][0] = 0xfa0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                        c.g[T5][0] = 0x1388;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x60), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x108), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xac), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x104), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x148), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x158), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x168), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x150), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x154), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x160), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x164), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x170), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x174), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x178), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x188), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x198), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x19c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x180), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x190), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x194), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x118), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x128), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x138), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x120), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x130), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x140), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x208), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x218), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x21c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x228), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x22c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x214), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x224), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x230), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x234), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x204), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x238), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), c.g[T4][0]);
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a4), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x280), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x284), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x240), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x244), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x248), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x254), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x258), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x260), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x264), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x268), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x270), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x274), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x278), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x288), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x290), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x294), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x298), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a0), c.g[T5][0]);
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
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
    })
}

/// `void GameCamera::SnapToDefaultChaseView(this, Vec4* a1, Vec4* a2)` (0x15e050):
/// build a default chase algorithm on the stack, point it at `a1` (+0x40) and
/// `a2` (+0x20) with w = 1, run its two set-up steps and copy the resulting
/// pose into the camera (+0x20..); then rebuild the camera matrix at +0x40
/// (look matrix, quaternion rotation, negated translation) and store the
/// matrix' z axis in +0x80, and destroy the algorithm.
pub fn game_camera_snap_to_default_chase_view(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0015e050);
        c.cycles += 149;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x450));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x00176d68, 0x0015e07c)?;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.lq(3, (c.g[S1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.lq(2, (c.g[S2][0] as u32));
        c.g[A1][0] = 0x1;
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4c));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
        call_ptr(c, 0x00166640, 0x0015e0a8)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x00166f90, 0x0015e0b0)?;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x390));
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x001673a0, 0x0015e0bc)?;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.g[V1][0] = 0x4c0000;
        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x53a0));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        c.lqc2(1, (c.g[V1][0] as u32));
        c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
        c.sqc2(1, (c.g[A2][0] as u32));
        c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.lqc2(4, (c.g[A1][0] as u32));
        c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
        c.vu0().add(0b1110, 5, 4, Src::Vec(4));
        c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
        c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
        c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
        c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
        c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
        c.vu0().opmula(5, 4);
        c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
        c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
        c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
        c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
        c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
        c.vu0().suba(0b0010, 1, Src::Bc(6, X));
        c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
        c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
        c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
        c.vu0().adda(0b0010, 0, Src::Bc(8, X));
        c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
        c.vu0().adda(0b0100, 0, Src::Bc(8, X));
        c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
        c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
        c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
        c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
        c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
        c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
        c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
        c.sqc2(0, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sqc2(10, (c.g[V0][0] as u32));
        c.sqc2(11, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.sqc2(12, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[A2][0] as u32));
        c.lqc2(5, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[V0][0] as u32));
        c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, (c.g[A2][0] as u32));
        c.sqc2(13, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sqc2(14, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.sqc2(15, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x24));
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x28));
        c.neg_s(4, 0);
        c.neg_s(5, 2);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x39c));
        c.neg_s(1, 3);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x3f0));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x394));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x398));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x3f4));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3f8));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x3a4));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x3a8));
        c.lqc2(4, (c.g[A2][0] as u32));
        c.lqc2(5, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x48));
        c.g[A0][0] = c.g[SP][0];
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = 0x2;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x68));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x394));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x398));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x39c), 0u64);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x80));
        call_ptr(c, 0x00176da8, 0x0015e288)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x440));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x450));
        c.pc = t;
        return Ok(());
    })
}

/// `void MultiSplineModifier::AllocArrays(this)` (0x35a328): allocate the
/// modifier's arrays with `operator new`: `MultiAnimMats` (+0x38, 0x40 bytes per
/// node, the count at `*(*(this+0x40)+0x80)+4`, pool 0x20000000), `MultiNodes`
/// (+0x3c, 0x60 bytes per model, `NumModels` at +4, pool 0xa0000000) and
/// `NodeInsts` (+0x44, 4 bytes per model). A zero count turns into the -1
/// array-size sentinel of the compiler's `new[]`.
pub fn multi_spline_modifier_alloc_arrays(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0035a328);
        'b_35a378: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A1][0] = 0x490000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15d0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A2][0] = 0x20000000;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A3][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
            call_ptr(c, 0x00317e30, 0x0035a368)?;
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_35a378;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0xffffffffffffffff;
                break 'b_35a378;
            }
        }
        'b_35a3b0: {
            c.cycles += 13;
            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
            c.g[A0][0] = 0x60;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0xffffffffa0000000;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15c0));
            c.g[A3][0] = 0u64;
            c.mult(0, 0, 16, 4);
            c.g[A0][0] = c.lo[0];
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x00317e30, 0x0035a3a4)?;
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_35a3b0;
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0xffffffffffffffff;
                break 'b_35a3b0;
            }
        }
        c.cycles += 15;
        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
        c.g[A1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15b0));
        c.g[A2][0] = 0x20000000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        call_ptr(c, 0x00317e30, 0x0035a3d0)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `void RiderModel::AddModelRef(this, int ref)` (0x30db70): append `ref` to the
/// model reference list (`count` at +0x60, entries at +0x64).
pub fn rider_model_add_model_ref(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030db70);
    c.cycles += 7;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x60), c.g[V1][0]);
    c.pc = t;
    return;
}

/// `void WScriptMan::CollectSetAddItems(pool, int* ids, int n)` (0x30c4a8): append
/// each valid (not -1) instance id of `ids` to the entry pool (`count` at +0,
/// ids from +0xc) and, depending on the race path (new race DB flag), either
/// respawn the instance straight away or only when `func_001538E8` says it
/// is not already collected.
pub fn wscript_man_collect_set_add_items(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0030c4a8);
        'b_30c6a0: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            call_ptr(c, 0x0014dc80, 0x0030c4dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x0030c4e8)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[S5][0] = c.g[V0][0];
            if t {
                break 'b_30c6a0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x530000;
                c.g[S3][0] = c.g[S0][0];
                c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                c.g[S4][0] = 0x490000;
                'l_30c500: loop {
                    'b_30c698: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                        c.g[V0][0] = !(0u64 | c.g[V1][0]);
                        if c.g[V0][0] == 0u64 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                            break 'b_30c698;
                        } else {
                            'b_30c68c: {
                                'b_30c688: {
                                    'b_30c5f4: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                        call_ptr(c, 0x0014dc80, 0x0030c524)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0014dd58, 0x0030c530)?;
                                        c.g[V1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x49));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                                        if t {
                                            break 'b_30c5f4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A2][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A3][0] = c.lw((c.g[S1][0] as u32));
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x001538e8, 0x0030c550)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                                            if t {
                                                break 'b_30c688;
                                            } else {
                                                'b_30c5a8: {
                                                    'b_30c584: {
                                                        c.cycles += 9;
                                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        c.g[V0][0] = c.g[A1][0] & 0xff;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_30c584;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[S0][0] = 0u64;
                                                            if t {
                                                                break 'b_30c5a8;
                                                            } else {
                                                                break 'b_30c584;
                                                            }
                                                        }
                                                    }
                                                    'b_30c5a4: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = 0u64;
                                                        if t {
                                                            break 'b_30c5a4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[A1][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            break 'b_30c5a4;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[S0][0] = c.g[A1][0];
                                                    break 'b_30c5a8;
                                                }
                                                'b_30c5d0: {
                                                    c.cycles += 3;
                                                    c.g[A2][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[A2][0] == 0u64;
                                                    c.g[A0][0] = 0x1c;
                                                    if t {
                                                        break 'b_30c5d0;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A1][0] = 0x3;
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x0030c5cc)?;
                                                        c.g[A0][0] = 0x1c;
                                                        break 'b_30c5d0;
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    sx((c.g[S4][0] as u32).wrapping_sub(0x6948));
                                                c.g[A2][0] = 0x20000000;
                                                c.g[A3][0] = 0u64;
                                                Callee {
                                                    addr: 0x00317d70,
                                                    f: ee::s10::EA_MemAllocNamed,
                                                }
                                                .call(c, 0x0030c5e0)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S0][0];
                                                call_ptr(c, 0x003506d8, 0x0030c5ec)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_30c68c;
                                                } else {
                                                    break 'b_30c5f4;
                                                }
                                            }
                                        }
                                    }
                                    'b_30c644: {
                                        'b_30c620: {
                                            c.cycles += 9;
                                            c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[V0][0] = c.g[A1][0] & 0xff;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                            if c.g[V1][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                                break 'b_30c620;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S0][0] = 0u64;
                                                if t {
                                                    break 'b_30c644;
                                                } else {
                                                    break 'b_30c620;
                                                }
                                            }
                                        }
                                        'b_30c640: {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_30c640;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                                                break 'b_30c640;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = c.g[A1][0];
                                        break 'b_30c644;
                                    }
                                    'b_30c66c: {
                                        c.cycles += 3;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if c.g[A2][0] == 0u64 {
                                            c.g[A0][0] = 0x1c;
                                            break 'b_30c66c;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[A1][0] = 0x3;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x0030c668)?;
                                            c.g[A0][0] = 0x1c;
                                            break 'b_30c66c;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6948));
                                    c.g[A2][0] = 0x20000000;
                                    c.g[A3][0] = 0u64;
                                    Callee {
                                        addr: 0x00317d70,
                                        f: ee::s10::EA_MemAllocNamed,
                                    }
                                    .call(c, 0x0030c67c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x003506d8, 0x0030c688)?;
                                    break 'b_30c688;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_30c68c;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                            break 'b_30c698;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S3][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030c500);
                        }
                        continue 'l_30c500;
                    } else {
                        break 'b_30c6a0;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
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
    })
}

/// `void ScriptViewObj::CutToShot(this, void* info, float frame)` (0x281c68): switch
/// the script view to the shot node `info[1]`: fade out the other view of the
/// pair (a kind >= 7 node asks for mode 9, otherwise the other camera is
/// reset to 0.0), set the view of the node's bit to the node's mode and,
/// when the node plays for a length, start its camera/effect through the
/// shot's function (+0xd8): kind >= 7 uses the short form, kinds 1..6 the
/// long form with the node's parameters. +0xe8 tells whether it started.
pub fn script_view_obj_cut_to_shot(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00281c68);
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.mov_s(20, 12);
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        'l_281c98: loop {
            'b_281ce8: {
                c.cycles += 2;
                if c.g[S1][0] == c.g[S3][0] {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_281ce8;
                } else {
                    'b_281cc8: {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_281cc8;
                        } else {
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0x9;
                            Callee {
                                addr: 0x00281ac8,
                                f: ee::s07::ScriptViewObj_BindCamSlot,
                            }
                            .call(c, 0x00281cc0)?;
                            let t = true;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_281ce8;
                            } else {
                                break 'b_281cc8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00281c40,
                        f: ee::s07::ScriptViewObj_GetSlotCam,
                    }
                    .call(c, 0x00281cd0)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_281ce8;
                    } else {
                        c.cycles += 4;
                        c.mtc1(0, 12);
                        c.g[A0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x0027d788,
                            f: ee::s07::ScriptObjCam_SetWeight,
                        }
                        .call(c, 0x00281ce4)?;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_281ce8;
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00281c98);
                }
                continue 'l_281c98;
            } else {
                'b_281e60: {
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00281ac8,
                        f: ee::s07::ScriptViewObj_BindCamSlot,
                    }
                    .call(c, 0x00281d00)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xe8), 0u64);
                    c.g[V0][0] = 0x8;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.lb((c.g[V1][0] as u32));
                    if c.g[A0][0] == c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_281e60;
                    } else {
                        'b_281e5c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_281e5c;
                            } else {
                                'b_281dd8: {
                                    'b_281dd0: {
                                        'b_281dbc: {
                                            c.cycles += 6;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xe8),
                                                c.g[V0][0],
                                            );
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                            c.g[V1][0] = c.lb((c.g[V1][0] as u32));
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                                            let t = c.g[V0][0] != 0u64;

                                            if t {
                                                break 'b_281dbc;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[S1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xd8));
                                                if t {
                                                    break 'b_281dd0;
                                                } else {
                                                    'b_281da4: {
                                                        'b_281d8c: {
                                                            c.cycles += 13;
                                                            Callee { addr: 0x0027d2e8, f: ee::s07::ScriptEngine_GetInstance }.call(c, 0x00281d50)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[A1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            Callee { addr: 0x0027d2f0, f: ee::s07::ScriptEngine_GetBindingProcess }.call(c, 0x00281d5c)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.cvt_s_w(0, 0);
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.lb((c.g[V0][0] as u32));
                                                            let t = (c.g[A0][0] as i64) < 0;
                                                            c.sub_s(12, 20, 0);
                                                            if t {
                                                                break 'b_281d8c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.mtc1(4, 13);
                                                                c.cvt_s_w(13, 13);
                                                                let t = true;

                                                                if t {
                                                                    break 'b_281da4;
                                                                } else {
                                                                    break 'b_281d8c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                                        c.g[V0][0] = c.g[A0][0] & 0x1;
                                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                        c.mtc1(2, 13);
                                                        c.cvt_s_w(13, 13);
                                                        c.add_s(13, 13, 13);
                                                        break 'b_281da4;
                                                    }
                                                    c.cycles += 6;
                                                    c.mtc1(0, 14);
                                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                    c.mov_s(15, 14);
                                                    Callee {
                                                        addr: 0x00282db0,
                                                        f: ee::s07::cTransitionMan_StopTransition,
                                                    }
                                                    .call(c, 0x00281db4)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_281e60;
                                                    } else {
                                                        break 'b_281dbc;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if (c.g[V1][0] as i64) <= 0 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            break 'b_281e60;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd8));
                                            if t {
                                                break 'b_281dd8;
                                            } else {
                                                break 'b_281dd0;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xe8), 0u64);
                                    if t {
                                        break 'b_281e5c;
                                    } else {
                                        break 'b_281dd8;
                                    }
                                }
                                'b_281e4c: {
                                    'b_281e2c: {
                                        c.cycles += 17;
                                        Callee {
                                            addr: 0x0027d2e8,
                                            f: ee::s07::ScriptEngine_GetInstance,
                                        }
                                        .call(c, 0x00281de0)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                        Callee {
                                            addr: 0x0027d2f0,
                                            f: ee::s07::ScriptEngine_GetBindingProcess,
                                        }
                                        .call(c, 0x00281dec)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.cvt_s_w(0, 0);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x4));
                                        c.sub_s(12, 20, 0);
                                        c.g[A2][0] = c.lb((c.g[V0][0] as u32));
                                        c.mtc1(3, 13);
                                        c.cvt_s_w(13, 13);
                                        let t = (c.g[A0][0] as i64) < 0;
                                        c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_281e2c;
                                        } else {
                                            c.cycles += 4;
                                            c.mtc1(4, 14);
                                            c.cvt_s_w(14, 14);
                                            let t = true;
                                            c.g[V1][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x2));
                                            if t {
                                                break 'b_281e4c;
                                            } else {
                                                break 'b_281e2c;
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                    c.g[V0][0] = c.g[A0][0] & 0x1;
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.mtc1(2, 14);
                                    c.cvt_s_w(14, 14);
                                    c.add_s(14, 14, 14);
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x2));
                                    break 'b_281e4c;
                                }
                                c.cycles += 4;
                                c.mtc1(3, 15);
                                c.cvt_s_w(15, 15);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                Callee {
                                    addr: 0x00282dd0,
                                    f: ee::s07::cTransitionMan_StartTransition,
                                }
                                .call(c, 0x00281e5c)?;
                                break 'b_281e5c;
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_281e60;
                    }
                }
                c.cycles += 7;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `Node* InputParser::ParseFunction1(Parser* self, int op)` (0x324398): parse the
/// `( expr )` argument of a one-argument expression function; a constant
/// operand is folded in place (ops 0x18..0x21: atan, sin, cos, two helper
/// calls, floor, ceil, abs, sqrt, wrap to a period), otherwise a unary node
/// {op, operand} is built. A missing `(` or `)` reports the error and returns
/// 0 (freeing the operand). Kept as the generated block dispatcher (`pc` is the
/// block address), so resume entries work.
pub fn input_parser_parse_function1(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00324398 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0x9;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        pc = 0x003243d4;
                        continue;
                    }
                    pc = 0x003243c0;
                    continue;
                }
                0x003243c0 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x490000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e68));
                    Callee {
                        addr: 0x00321500,
                        f: ee::s10::InputParser_Error,
                    }
                    .call(c, 0x003243cc)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x003243d4;
                    continue;
                }
                0x003243d4 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00321af0,
                        f: ee::s10::InputParser_NextToken,
                    }
                    .call(c, 0x003243dc)?;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00323268,
                        f: ee::s10::InputParser_ParseConditional,
                    }
                    .call(c, 0x003243e4)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x98));
                        pc = 0x003243f8;
                        continue;
                    }
                    pc = 0x003243f0;
                    continue;
                }
                0x003243f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x003243f8;
                    continue;
                }
                0x003243f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x00324424;
                        continue;
                    }
                    pc = 0x00324404;
                    continue;
                }
                0x00324404 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f48));
                    Callee {
                        addr: 0x00321500,
                        f: ee::s10::InputParser_Error,
                    }
                    .call(c, 0x00324410)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00321638,
                        f: ee::s10::InputParser_FreeNode,
                    }
                    .call(c, 0x0032441c)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x00324424;
                    continue;
                }
                0x00324424 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00321af0,
                        f: ee::s10::InputParser_NextToken,
                    }
                    .call(c, 0x0032442c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x18));
                    if t {
                        pc = 0x0032457c;
                        continue;
                    }
                    pc = 0x0032443c;
                    continue;
                }
                0x0032443c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324448;
                    continue;
                }
                0x00324448 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1e20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00324460 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031c228,
                        f: ee::s10::Math_Atan,
                    }
                    .call(c, 0x00324468)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324470;
                    continue;
                }
                0x00324470 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031bf60,
                        f: ee::s10::Math_Sin,
                    }
                    .call(c, 0x00324478)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324480;
                    continue;
                }
                0x00324480 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x0031c040,
                        f: ee::s10::Math_Cos,
                    }
                    .call(c, 0x00324488)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x00324490;
                    continue;
                }
                0x00324490 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0040d758, 0x00324498)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244a0;
                    continue;
                }
                0x003244a0 => {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0040da10, 0x003244a8)?;
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244b0;
                    continue;
                }
                0x003244b0 => {
                    c.cycles += 7;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);

                    if !c.fcc() {
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244cc;
                    continue;
                }
                0x003244cc => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    let t = true;
                    c.sub_s(1, 1, 0);
                    if t {
                        pc = 0x00324504;
                        continue;
                    }
                    pc = 0x003244dc;
                    continue;
                }
                0x003244dc => {
                    c.cycles += 7;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(1, 0);

                    if !c.fcc() {
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x003244f8;
                    continue;
                }
                0x003244f8 => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.add_s(1, 1, 0);
                    pc = 0x00324504;
                    continue;
                }
                0x00324504 => {
                    c.cycles += 2;
                    let t = true;
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032450c;
                    continue;
                }
                0x0032450c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.abs_s(0, 0);
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032451c;
                    continue;
                }
                0x0032451c => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.sqrt_s(0, 0);
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00324574;
                        continue;
                    }
                    pc = 0x0032452c;
                    continue;
                }
                0x0032452c => {
                    c.cycles += 11;
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e2c));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 2, 0);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();

                    if t {
                        pc = 0x00324564;
                        continue;
                    }
                    pc = 0x00324558;
                    continue;
                }
                0x00324558 => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.sub_s(1, 1, 0);
                    pc = 0x00324564;
                    continue;
                }
                0x00324564 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2e28));
                    c.mul_s(0, 1, 0);
                    c.sub_s(0, 2, 0);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    pc = 0x00324574;
                    continue;
                }
                0x00324574 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00324590;
                        continue;
                    }
                    pc = 0x0032457c;
                    continue;
                }
                0x0032457c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x003215f8,
                        f: ee::s10::InputParser_AllocNode,
                    }
                    .call(c, 0x00324584)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.sw((c.g[V1][0] as u32), c.g[S2][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    pc = 0x00324590;
                    continue;
                }
                0x00324590 => {
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
                    c.jump_out(pc, 0x00324398, 0x003245a8);
                    return Ok(());
                }
            }
        }
    })
}

/// `int BEMissionInt::CountCollectedBitsOnPeak(this, int profile, int character, int owner)`
/// (0x153708): when a game is running (`gp - 0x848` world, field +0x84 non-zero) ask
/// `CountCollectedOnPeak` (rider found by `BE_FindRiderByProfile`)
/// for the rider's index; otherwise add up the set collect bits (two words of
/// 32 bits) of every collectible entry of the profile and character whose
/// owner peak matches.
pub fn be_mission_int_count_collected_bits_on_peak(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00153708);
        'b_153840: {
            'b_15375c: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[T0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[T1][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A3][0];
                if t {
                    break 'b_15375c;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[T0][0];
                    Callee {
                        addr: 0x0014a130,
                        f: ee::s01::BE_FindRiderByProfile,
                    }
                    .call(c, 0x00153744)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001535c0,
                        f: ee::s01::BEMissionInt_CountCollectedOnPeak,
                    }
                    .call(c, 0x00153754)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_153840;
                    } else {
                        break 'b_15375c;
                    }
                }
            }
            c.cycles += 15;
            c.g[A1][0] = 0x9b50;
            c.g[A0][0] = 0xf88;
            c.mult(0, 5, 8, 5);
            c.mult(0, 4, 9, 4);
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = 0x4a0000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x265c));
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6cac));
            c.g[T6][0] = 0u64;
            c.g[V0][0] = 0u64;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[T9][0] = 0x64;
            c.g[T8][0] = 0xc;
            c.mult(0, 13, 2, 24);
            'l_153798: loop {
                c.cycles += 4;
                c.g[A0][0] = 0u64;
                c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.mult(0, 2, 4, 25);
                'l_1537a8: loop {
                    'b_15381c: {
                        c.cycles += 4;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        let t = c.g[V0][0] != c.g[S1][0];
                        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_15381c;
                        } else {
                            c.cycles += 10;
                            c.mult(0, 3, 4, 24);
                            c.g[T0][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[T4][0] = 0xffffffffffffffff;
                            c.g[T3][0] = 0x1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                            'l_1537e0: loop {
                                c.cycles += 14;
                                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f));
                                if c.g[V0][0] != 0 {
                                    c.g[V1][0] = c.g[A3][0];
                                }
                                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                                c.g[A2][0] = sx((c.g[T3][0] as u32) << ((c.g[A3][0] as u32) & 31));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                c.g[A0][0] = ((c.g[A3][0] as i64) < 64i64) as u64;
                                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                let t = c.g[A0][0] != 0u64;
                                if c.g[V0][0] != 0 {
                                    c.g[T0][0] = c.g[A1][0];
                                }
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001537e0);
                                    }
                                    continue 'l_1537e0;
                                } else {
                                    c.cycles += 1;
                                    c.g[T6][0] =
                                        sx((c.g[T6][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                    break 'b_15381c;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[T2][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 22i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 2, 4, 25);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001537a8);
                        }
                        continue 'l_1537a8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[T7][0];
                        if (c.g[V0][0] as i64) <= 0 {
                            c.mult(0, 13, 2, 24);
                            if event_due(c) {
                                deliver_event_here(c, 0x00153798);
                            }
                            continue 'l_153798;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.g[T6][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            break 'b_153840;
                        }
                    }
                }
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
    })
}

/// `void GMHandler1::Reset(this)` (0x23a108): reset the single-player game mode
/// handler for the heat in `iNextHeat`: unlock the results, set the heat number
/// (with the quick-start flag forcing heat 3) and per heat (1: first heat,
/// 2: second, 3: final) set the AI counts, clear or rebuild the rider tables
/// (`func_0023A4F0`, `func_0023A668`, memsets of the heat tables to 9 or 0),
/// store the next location/heat and clear the event flags.
pub fn gm_handler1_reset(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0023a108);
        'b_23a198: {
            'b_23a194: {
                c.cycles += 29;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S0][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                call_ptr(c, 0x0014dc80, 0x0023a148)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023a154)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023a15c)?;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0023a168)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A1][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                if t {
                    break 'b_23a194;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = 0x3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    if c.g[V0][0] == c.g[A0][0] {
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        break 'b_23a198;
                    } else {
                        c.cycles += 2;
                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        break 'b_23a194;
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            break 'b_23a198;
        }
        'b_23a4c8: {
            'b_23a4c4: {
                'b_23a4c0: {
                    'b_23a360: {
                        'b_23a254: {
                            c.cycles += 5;
                            c.g[V0][0] = 0x2;
                            c.g[S1][0] = c.lw((c.g[A0][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0u64);
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), 0u64);
                            if t {
                                break 'b_23a254;
                            } else {
                                'b_23a1c8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_23a1c8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_23a360;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            if t {
                                                break 'b_23a4c8;
                                            } else {
                                                break 'b_23a1c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = (c.g[S1][0] as i64) < 0;
                                c.g[S1][0] = 0x1;
                                if t {
                                    break 'b_23a4c4;
                                } else {
                                    'b_23a1fc: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x5;
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x98), 0u64);
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                        c.sw((c.g[A0][0] as u32), c.g[S1][0]);
                                        let t = c.g[V1][0] != c.g[S1][0];
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                                        if t {
                                            break 'b_23a1fc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0023a4f0,
                                                f: ee::s06::GMHandler1_PickOpponents,
                                            }
                                            .call(c, 0x0023a1f4)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0023a668,
                                                f: ee::s06::GMHandler1_SetupRiderSlots,
                                            }
                                            .call(c, 0x0023a1fc)?;
                                            break 'b_23a1fc;
                                        }
                                    }
                                    c.cycles += 22;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                    c.g[A1][0] = 0x9;
                                    c.g[A2][0] = 0x78;
                                    call_ptr(c, 0x00416210, 0x0023a20c)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[A1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0023a220)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x90), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0u64);
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                    if t {
                                        break 'b_23a4c0;
                                    } else {
                                        break 'b_23a254;
                                    }
                                }
                            }
                        }
                        'b_23a2ec: {
                            c.cycles += 9;
                            c.g[V0][0] = 0x5;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S6][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x98), 0u64);
                            c.g[S7][0] = 0x530000;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x80));
                            if t {
                                break 'b_23a2ec;
                            } else {
                                c.cycles += 12;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00146e98, 0x0023a284)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x00147398, 0x0023a294)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
                                c.g[V0][0] = 0x530000;
                                c.g[A2][0] = 0x2;
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
                                'l_23a2a8: loop {
                                    'b_23a2d8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                        let t = c.g[V0][0] == c.g[S1][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a2d8;
                                        } else {
                                            'b_23a2d4: {
                                                c.cycles += 5;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                    break 'b_23a2d4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                                                    break 'b_23a2d4;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            break 'b_23a2d8;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[A2][0] as i64) >= 0;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0023a2a8);
                                        }
                                        continue 'l_23a2a8;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a2ec)?;
                                        break 'b_23a2ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 29;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A3][0] = 0x2;
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6640));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                        c.g[A1][0] = 0x9;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A3][0]);
                        c.g[A2][0] = 0x28;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A3][0]);
                        call_ptr(c, 0x00416210, 0x0023a314)?;
                        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6730));
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x28;
                        call_ptr(c, 0x00416210, 0x0023a324)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x9;
                        c.g[A2][0] = 0x28;
                        call_ptr(c, 0x00416210, 0x0023a334)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x88), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        if t {
                            break 'b_23a4c4;
                        } else {
                            break 'b_23a360;
                        }
                    }
                    'b_23a458: {
                        'b_23a454: {
                            c.cycles += 9;
                            c.g[S3][0] = 0x1;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S6][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[S3][0]);
                            c.g[S7][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                            if t {
                                break 'b_23a454;
                            } else {
                                'b_23a3d8: {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_23a3d8;
                                    } else {
                                        c.cycles += 19;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a4f0,
                                            f: ee::s06::GMHandler1_PickOpponents,
                                        }
                                        .call(c, 0x0023a394)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[V0][0] as u32), c.g[S3][0]);
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a3a4)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[A1][0] = 0x9;
                                        c.g[A2][0] = 0x78;
                                        c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                                        call_ptr(c, 0x00416210, 0x0023a3bc)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x0023a3d0)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a458;
                                        } else {
                                            break 'b_23a3d8;
                                        }
                                    }
                                }
                                c.cycles += 14;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00146e98, 0x0023a3e0)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x00147398, 0x0023a3f0)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x00145750, 0x0023a3fc)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[V0][0] = 0x530000;
                                c.g[A2][0] = 0x2;
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
                                'l_23a410: loop {
                                    'b_23a440: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                        let t = c.g[V0][0] == c.g[S1][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a440;
                                        } else {
                                            'b_23a43c: {
                                                c.cycles += 5;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                    break 'b_23a43c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                                                    break 'b_23a43c;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            break 'b_23a440;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[A2][0] as i64) >= 0;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0023a410);
                                        }
                                        continue 'l_23a410;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a454)?;
                                        break 'b_23a454;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        break 'b_23a458;
                    }
                    c.cycles += 26;
                    c.g[A3][0] = 0x3;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6640));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                    c.g[A1][0] = 0x9;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A3][0]);
                    c.g[A2][0] = 0x28;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A3][0]);
                    call_ptr(c, 0x00416210, 0x0023a47c)?;
                    c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6730));
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x0023a48c)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x0023a49c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x88), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    break 'b_23a4c0;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_23a4c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_23a4c8;
        }
        c.cycles += 10;
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, put_words, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (written by `oc427_base`).
    const OC427_CTRL: u32 = DATA + 0x20_0000;
    const OC427_MAGIC: u32 = 0x0c42_7a11;
    /// Result queues of the faked callees: `[head, len, values: u64 x 30]`,
    /// 0x100 bytes per callee, in the order of `OC427_CALLEES`.
    const OC427_QUEUES: u32 = OC427_CTRL + 0x100;

    /// The callees the gated fakes stand in for (all have override slots).
    const OC427_CALLEES: &[u32] = &[
        0x0028_3d28, // ScriptCmdQueue_Count
        0x0028_3d70, // ScriptCmdQueue_At
        0x0027_7dd8, // ScriptList_IsMovieEntry
        0x0027_95d0, // ScriptEngine_SpawnScript
        0x0027_9528, // ScriptEngine_SpawnChosenScript
        0x0028_38e8, // ScriptMovie_Open
        0x0027_3dc0, // InSeq_ChannelStop
        0x0027_4d70, // InSeq_InstanceGetListener
        0x0027_4c20, // InSeq_InstanceGetChannel
        0x0027_43c8, // InSeq_ChannelGetId
        0x0027_3a60, // InSeq_ChannelSelectBlock
        0x0028_1c40, // ScriptViewObj_GetSlotCam
        0x0027_d2e8, // ScriptEngine_GetInstance
        0x0027_d2f0, // ScriptEngine_GetBindingProcess
        0x0011_6fa8, // cHUDFX_ctor
        0x0014_dc80, // BELibrary_GetInstance
        0x0014_dd58, // BELibrary_GetInterface
        0x0014_4bc0, // BENewRaceInt_GetTrackInfo
        0x0014_c6a0, // Bolts_LoadDB
        0x0015_67b8, // BE_ResetProfileCharacters
        0x0015_38e8, // BEMissionInt_IsCollected
        0x0014_6e98, // BENewPlayerInt_GetHumanRider
        0x0014_7398, // BENewPlayerInt_GetCharacter
        0x0014_5750, // BENewRaceInt_GetPeakRivalCharacter
        0x0014_a130, // BE_FindRiderByProfile
        0x0015_35c0, // BEMissionInt_CountCollectedOnPeak
        0x0017_6d68, // Default_3ChaseCameraAlgorithm_ctor
        0x0016_6640, // ChaseCameraAlgorithm_ComputeAngles
        0x0016_6f90, // ChaseCameraAlgorithm_BuildOutputView
        0x0016_73a0, // ChaseCameraAlgorithm_GetView
        0x0031_7e30, // operator_new_named
        0x0031_c228, // Math_Atan
        0x0031_c128, // Math_Asin
        0x0031_7e98, // EA_MemFree
        0x0031_77f0, // Rand_NextMain
        0x0031_7d70, // EA_MemAllocNamed
        0x0032_1500, // InputParser_Error
        0x0032_1af0, // InputParser_NextToken
        0x0032_3268, // InputParser_ParseConditional
        0x0031_bf60, // Math_Sin
        0x0031_c040, // Math_Cos
        0x0032_15f8, // InputParser_AllocNode
        0x0035_06d8, // DeadNode_ctor
        0x0041_6210, // memset
        0x0041_605c, // memcpy
        0x0040_d758, // expf
        0x0040_da10, // logf
        0x0012_3128, // Rider_GetCharacter
        0x0027_42e0, // GetBlockPayload
        0x0031_d790, // cCSICubicSpline_Free
        0x0031_7e50, // __builtin_delete
    ];

    fn oc427_queue_base(addr: u32) -> u32 {
        let i = OC427_CALLEES
            .iter()
            .position(|&a| a == addr)
            .expect("callee without a fake");
        OC427_QUEUES + 0x100 * i as u32
    }

    /// The faked callee `addr` returns `values` from its next calls (then its
    /// default hashed pointer).
    fn oc427_queue(c: &mut Cpu, addr: u32, values: &[u64]) {
        let q = oc427_queue_base(addr);
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            c.bus.write64(q + 8 + 8 * i as u32, *v);
        }
    }

    /// Pop the next queued value of callee `addr`.
    fn oc427_pop(c: &mut Cpu, addr: u32) -> Option<u64> {
        let q = oc427_queue_base(addr);
        let head = c.bus.read32(q);
        if head < c.bus.read32(q + 4) {
            c.bus.write32(q, head + 1);
            Some(c.bus.read64(q + 8 + 8 * head))
        } else {
            None
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake`, then returns the
    /// next queued value; `NextToken` stores it as the parser's token
    /// instead, `GetView` fills the pose it was asked for.
    fn oc427_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0032_1af0 => {
                if let Some(token) = oc427_pop(c, addr) {
                    c.bus.write32(a0 + 0x98, token as u32);
                }
            }
            0x0016_73a0 => {
                for k in 0..8u32 {
                    c.bus
                        .write32(a0 + 4 * k, fakes::mix(n as u64 * 8 + k as u64) as u32);
                }
            }
            _ => {
                if let Some(v) = oc427_pop(c, addr) {
                    c.g[V0][0] = v;
                }
            }
        }
    }

    macro_rules! oc427_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC427_CTRL) != OC427_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc427_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once per
            /// process; with the gate off they run what the slots held).
            fn oc427_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc427_fakes! {
        (oc427_fake_00283d28, 0x0028_3d28, s07, OVERRIDE_00283D28, f_00283d28__generated),
        (oc427_fake_00283d70, 0x0028_3d70, s07, OVERRIDE_00283D70, f_00283d70__generated),
        (oc427_fake_00277dd8, 0x0027_7dd8, s07, OVERRIDE_00277DD8, f_00277dd8__generated),
        (oc427_fake_002795d0, 0x0027_95d0, s07, OVERRIDE_002795D0, f_002795d0__generated),
        (oc427_fake_00279528, 0x0027_9528, s07, OVERRIDE_00279528, f_00279528__generated),
        (oc427_fake_002838e8, 0x0028_38e8, s07, OVERRIDE_002838E8, f_002838e8__generated),
        (oc427_fake_00273dc0, 0x0027_3dc0, s07, OVERRIDE_00273DC0, f_00273dc0__generated),
        (oc427_fake_00274d70, 0x0027_4d70, s07, OVERRIDE_00274D70, f_00274d70__generated),
        (oc427_fake_00274c20, 0x0027_4c20, s07, OVERRIDE_00274C20, f_00274c20__generated),
        (oc427_fake_002743c8, 0x0027_43c8, s07, OVERRIDE_002743C8, f_002743c8__generated),
        (oc427_fake_00273a60, 0x0027_3a60, s07, OVERRIDE_00273A60, f_00273a60__generated),
        (oc427_fake_00281c40, 0x0028_1c40, s07, OVERRIDE_00281C40, f_00281c40__generated),
        (oc427_fake_0027d2e8, 0x0027_d2e8, s07, OVERRIDE_0027D2E8, f_0027d2e8__generated),
        (oc427_fake_0027d2f0, 0x0027_d2f0, s07, OVERRIDE_0027D2F0, f_0027d2f0__generated),
        (oc427_fake_00116fa8, 0x0011_6fa8, s00, OVERRIDE_00116FA8, f_00116fa8__generated),
        (oc427_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc427_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc427_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc427_fake_0014c6a0, 0x0014_c6a0, s01, OVERRIDE_0014C6A0, f_0014c6a0__generated),
        (oc427_fake_001567b8, 0x0015_67b8, s01, OVERRIDE_001567B8, f_001567b8__generated),
        (oc427_fake_001538e8, 0x0015_38e8, s01, OVERRIDE_001538E8, f_001538e8__generated),
        (oc427_fake_00146e98, 0x0014_6e98, s01, OVERRIDE_00146E98, f_00146e98__generated),
        (oc427_fake_00147398, 0x0014_7398, s01, OVERRIDE_00147398, f_00147398__generated),
        (oc427_fake_00145750, 0x0014_5750, s01, OVERRIDE_00145750, f_00145750__generated),
        (oc427_fake_0014a130, 0x0014_a130, s01, OVERRIDE_0014A130, f_0014a130__generated),
        (oc427_fake_001535c0, 0x0015_35c0, s01, OVERRIDE_001535C0, f_001535c0__generated),
        (oc427_fake_00176d68, 0x0017_6d68, s02, OVERRIDE_00176D68, f_00176d68__generated),
        (oc427_fake_00166640, 0x0016_6640, s02, OVERRIDE_00166640, f_00166640__generated),
        (oc427_fake_00166f90, 0x0016_6f90, s02, OVERRIDE_00166F90, f_00166f90__generated),
        (oc427_fake_001673a0, 0x0016_73a0, s02, OVERRIDE_001673A0, f_001673a0__generated),
        (oc427_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc427_fake_0031c228, 0x0031_c228, s10, OVERRIDE_0031C228, f_0031c228__generated),
        (oc427_fake_0031c128, 0x0031_c128, s10, OVERRIDE_0031C128, f_0031c128__generated),
        (oc427_fake_00317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc427_fake_003177f0, 0x0031_77f0, s10, OVERRIDE_003177F0, f_003177f0__generated),
        (oc427_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc427_fake_00321500, 0x0032_1500, s10, OVERRIDE_00321500, f_00321500__generated),
        (oc427_fake_00321af0, 0x0032_1af0, s10, OVERRIDE_00321AF0, f_00321af0__generated),
        (oc427_fake_00323268, 0x0032_3268, s10, OVERRIDE_00323268, f_00323268__generated),
        (oc427_fake_0031bf60, 0x0031_bf60, s10, OVERRIDE_0031BF60, f_0031bf60__generated),
        (oc427_fake_0031c040, 0x0031_c040, s10, OVERRIDE_0031C040, f_0031c040__generated),
        (oc427_fake_003215f8, 0x0032_15f8, s10, OVERRIDE_003215F8, f_003215f8__generated),
        (oc427_fake_003506d8, 0x0035_06d8, s11, OVERRIDE_003506D8, f_003506d8__generated),
        (oc427_fake_00416210, 0x0041_6210, s15, OVERRIDE_00416210, f_00416210__generated),
        (oc427_fake_0041605c, 0x0041_605c, s15, OVERRIDE_0041605C, f_0041605c__generated),
        (oc427_fake_0040d758, 0x0040_d758, s15, OVERRIDE_0040D758, f_0040d758__generated),
        (oc427_fake_0040da10, 0x0040_da10, s15, OVERRIDE_0040DA10, f_0040da10__generated),
        (oc427_fake_00123128, 0x0012_3128, s00, OVERRIDE_00123128, f_00123128__generated),
        (oc427_fake_002742e0, 0x0027_42e0, s07, OVERRIDE_002742E0, f_002742e0__generated),
        (oc427_fake_0031d790, 0x0031_d790, s10, OVERRIDE_0031D790, f_0031d790__generated),
        (oc427_fake_00317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
    }

    /// Scrambled registers, the gate, `pc` at the entry, the fakes. `calls`
    /// bounds the call index a fake may unwind at.
    fn oc427_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc427_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC427_CTRL, OC427_MAGIC);
        r
    }

    fn oc427_gp(off: i32) -> u32 {
        (GAME_GP as i32 + off) as u32
    }

    /// A vtable at `vt` with `{delta, FAKE}` at `slot`.
    fn oc427_vtable(c: &mut Cpu, vt: u32, slot: u32, delta: u16) {
        c.bus.write16(vt + slot, delta);
        c.bus.write32(vt + slot + 4, fakes::FAKE);
    }

    /// The world model cache the locator code walks (`gp + 0x16c8`): a
    /// cache object whose model table (+8) holds up to three models (or a
    /// null one), each with a bone table (+0x1c) of three bones, a bone
    /// being the word `(address >> 2) << 8`. Bone `b` of any model lives at
    /// `bones + 0x100 * b`.
    fn oc427_world_cache(c: &mut Cpu, r: &mut Rng, bones: u32) {
        let wc = DATA + 0x6000;
        let cache = DATA + 0x6100;
        let models = DATA + 0x6200;
        c.bus.write32(oc427_gp(0x16c8), wc);
        c.bus.write32(wc, cache);
        c.bus.write32(cache + 8, models);
        for m in 0..3u32 {
            let model = DATA + 0x6300 + 0x100 * m;
            let table = DATA + 0x6400 + 0x100 * m;
            let null = r.next_u32() % 5 == 0;
            c.bus.write32(models + 4 * m, if null { 0 } else { model });
            c.bus.write32(model + 0x1c, table);
            for b in 0..3u32 {
                let bone = bones + 0x100 * b;
                let word = if r.next_u32() % 7 == 0 {
                    r.next_u32() & 0xff
                } else {
                    ((bone >> 2) << 8) | (r.next_u32() & 0xff)
                };
                c.bus.write32(table + 4 * b, word);
            }
        }
    }

    /// A locator code: bone `b` of model `m`, or -1.
    fn oc427_locator_code(r: &mut Rng) -> u32 {
        if r.next_u32() % 6 == 0 {
            0xffff_ffff
        } else {
            ((r.next_u32() % 3) << 8) | (r.next_u32() % 3)
        }
    }

    /// `ScriptList_PrepareEntry`: random queues of entries (categories and
    /// indices, flag 2, ready/unready ids), random movie entries and spawn
    /// results (negative or valid ids), an idle or busy movie player, and
    /// callees that unwind.
    #[test]
    fn script_list_prepare_entry_matches() {
        covered(
            ee::s07::f_00277450__generated,
            script_list_prepare_entry,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x277450, 24, 0x0027_7450);
                let this = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let entries = DATA + 0x3000;
                c.bus.write32(this, engine);
                // the movie player (+0x500) is idle in two of three seeds
                c.bus
                    .write32(engine + 0x500, if r.next_u32() % 3 == 0 { 1 } else { 0 });
                for k in 0..4u32 {
                    let e = entries + 0x20 * k;
                    c.bus.write32(e, r.next_u32() % 4);
                    c.bus
                        .write32(e + 4, [0xffff_ffff, 2, 5][r.next_u32() as usize % 3]);
                    c.bus
                        .write32(e + 8, if r.next_u32() % 3 == 0 { 2 } else { 0 });
                    c.bus.write32(
                        e + 0xc,
                        [0xffff_ffff, 0xffff_ffff, 7][r.next_u32() as usize % 3],
                    );
                    for w in 0..3 {
                        c.bus.write32(e + 0x10 + 4 * w, r.next_u32());
                    }
                }
                let mut at = [0u64; 12];
                let mut movie = [0u64; 12];
                let mut spawn = [0u64; 12];
                let mut open = [0u64; 12];
                for k in 0..12 {
                    at[k] = (entries + 0x20 * (r.next_u32() % 4)) as u64;
                    movie[k] = (r.next_u32() % 3 == 0) as u64;
                    spawn[k] = [u64::MAX, 0, 3, 9][r.next_u32() as usize % 4];
                    open[k] = (r.next_u32() % 2) as u64;
                }
                oc427_queue(c, 0x0028_3d28, &[1 + (r.next_u32() % 4) as u64]);
                oc427_queue(c, 0x0028_3d70, &at);
                oc427_queue(c, 0x0027_7dd8, &movie);
                oc427_queue(c, 0x0027_95d0, &spawn);
                oc427_queue(c, 0x0027_9528, &spawn);
                oc427_queue(c, 0x0028_38e8, &open);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
            },
        );
    }

    /// `RiderMetrix_InitHUDFX`: a rider with a negative or valid HUD slot;
    /// the allocation returns a clean block, the 44 constructors are faked
    /// (one of the 46 calls may unwind).
    #[test]
    fn rider_metrix_init_hudfx_matches() {
        covered(
            ee::s00::f_001172e0__generated,
            rider_metrix_init_hudfx,
            100,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x1172e0, 46, 0x0011_72e0);
                let this = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let slot = if seed % 3 == 0 {
                    0xffff_fff0 | (r.next_u32() % 8)
                } else {
                    r.next_u32() % 100
                };
                c.bus.write32(rider + 0x870, slot);
                oc427_queue(c, 0x0031_7e30, &[(DATA + 0x1_0000) as u64]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rider);
            },
        );
    }

    /// `InSeq_InstanceStop`: stopped, update-in-progress and running
    /// instances with 0..3 entity players; the listener is absent or has a
    /// vtable entry (a fake method) at +0x20.
    #[test]
    fn in_seq_instance_stop_matches() {
        covered(
            ee::s07::f_00274918__generated,
            in_seq_instance_stop,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x274918, 6, 0x0027_4918);
                let this = DATA + 0x1000;
                let header = DATA + 0x2000;
                let listener = DATA + 0x3000;
                let vt = DATA + 0x3100;
                c.bus.write32(this + 0x10, header);
                c.bus.write32(this + 0x18, DATA + 0x2400);
                c.bus
                    .write32(this + 0x20, [0, 1, 2][r.next_u32() as usize % 3]);
                c.bus.write32(this + 0x3c, (r.next_u32() % 3 == 0) as u32);
                c.bus.write8(header + 1, (r.next_u32() % 4) as u8);
                c.bus.write32(listener, vt);
                oc427_vtable(c, vt, 0x20, (r.next_u32() as u16) & !3);
                oc427_queue(
                    c,
                    0x0027_4d70,
                    &[if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        listener as u64
                    }],
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `ScriptEngine_GetLocatorPose`: plain and race-location (50) categories,
    /// missing tables, empty codes, missing models, bones with zero and
    /// signed axes (atan2 and the pitch asin are faked), with and without an
    /// offset to rotate.
    #[test]
    fn script_engine_get_locator_pose_matches() {
        covered(
            ee::s07::f_0027bb08__generated,
            script_engine_get_locator_pose,
            200,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x27bb08, 6, 0x0027_bb08);
                let this = DATA + 0x1000;
                let table = DATA + 0x1800;
                let bones = DATA + 0x7000;
                let cat = if seed % 4 == 0 { 50 } else { r.next_u32() % 10 };
                let slot = r.next_u32() % 4;
                // the category's code table (or none)
                let real_cat = if cat == 50 { r.next_u32() % 10 } else { cat };
                let tbl = if r.next_u32() % 6 == 0 { 0 } else { table };
                c.bus.write32(this + 0x558 + 4 * real_cat, tbl);
                if cat == 50 {
                    // the race location (GetTrackInfo -> word) maps to a category
                    c.bus.write32(DATA + 0x1900, r.next_u32() % 4);
                    oc427_queue(c, 0x0014_4bc0, &[(DATA + 0x1900) as u64]);
                    for k in 0..4 {
                        c.bus.write32(0x0048_2660 + 4 * k, r.next_u32() % 10);
                    }
                    let k = c.bus.read32(DATA + 0x1900);
                    let mapped = c.bus.read32(0x0048_2660 + 4 * k);
                    // the code table of the mapped category
                    c.bus.write32(this + 0x558 + 4 * real_cat, 0);
                    c.bus.write32(this + 0x558 + 4 * mapped, tbl);
                }
                for s in 0..4 {
                    c.bus.write32(table + 4 * s, oc427_locator_code(&mut r));
                }
                oc427_world_cache(c, &mut r, bones);
                for b in 0..3u32 {
                    let bone = bones + 0x100 * b;
                    for w in 0..0x20 {
                        c.bus.write32(bone + 4 * w, r.f32(100.0));
                    }
                    for off in [0x10, 0x14] {
                        let v = [0, 0x3fc0_0000, 0xc000_0000][r.next_u32() as usize % 3];
                        c.bus.write32(bone + off, v);
                    }
                }
                for k in 0..4 {
                    c.bus.write32(oc427_gp(-0x47ac + 4 * k), r.f32(10.0));
                }
                let off = DATA + 0x8200;
                for w in 0..4 {
                    c.bus.write32(off + 4 * w, r.f32(10.0));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(slot);
                c.g[A2][0] = sx(DATA + 0x8000); // position out
                c.g[A3][0] = sx(DATA + 0x8120); // flag out
                c.g[T0][0] = sx(DATA + 0x8110); // pitch out
                c.g[T1][0] = sx(DATA + 0x8100); // yaw out
                c.g[T2][0] = sx(cat);
                c.g[T3][0] = if r.next_u32() % 3 == 0 { 0 } else { sx(off) };
            },
        );
    }

    /// `ScriptEngine_v10`: scripts of 0..3 entity players (a channel each, of
    /// type 5 or not, with 0..5 variations); the 21 player flags come from the
    /// real `EvalChoiceParams` over a jump table of one handler and the
    /// (faked) rider characters; the options' slots map through `0x482608`.
    #[test]
    fn script_engine_v10_matches() {
        covered(ee::s07::f_00279a70__generated, script_engine_v10, 120, &|c, seed| {
            let mut r = oc427_base(c, seed, 0x279a70, 60, 0x0027_9a70);
            let engine = DATA + 0x1000;
            let script = DATA + 0x2000;
            let header = DATA + 0x2100;
            let players = DATA + 0x1f00;
            let world = DATA + 0x3000;
            let brain = DATA + 0x3100;
            let state = DATA + 0x3200;
            let master = DATA + 0x5800;
            let opts = DATA + 0x5000;
            // EvalChoiceParams: world -> +0x84 -> +0xc -> state; handler
            // 0x27be3c for every flag, rider characters from the queue
            c.bus.write32(oc427_gp(-0x848), world);
            c.bus.write32(world + 0x84, brain);
            c.bus.write32(brain + 0xc, state);
            for j in 0..21 {
                c.bus.write32(0x0048_1ec0 + 4 * j, 0x0027_be3c);
            }
            let mut chars = [0u64; 21];
            for v in chars.iter_mut() {
                *v = (r.next_u32() % 8) as u64;
            }
            oc427_queue(c, 0x0012_3128, &chars);
            for k in 0..8 {
                c.bus.write32(0x0048_1bb0 + 4 * k, r.next_u32() & 0xff);
            }
            // players: the first must be valid for the handler
            c.bus.write32(players, r.next_u32() % 3);
            c.bus.write32(players + 4, r.next_u32() % 3);
            c.bus.write32(players + 8, r.next_u32() % 3);
            // the script
            c.bus.write32(script + 0x10, header);
            let events = r.next_u32() % 4;
            c.bus.write8(header + 1, events as u8);
            c.bus.write32(engine + 0x530, master);
            for k in 0..8 {
                c.bus.write32(0x0048_2608 + 4 * k, r.next_u32() % 6);
            }
            let mut ids = [0u64; 4];
            let mut channels = [0u64; 4];
            let mut keyblocks = [0u64; 30];
            let mut rands = [0u64; 12];
            for e in 0..4u32 {
                let ch = DATA + 0x4000 + 0x100 * e;
                channels[e as usize] = ch as u64;
                ids[e as usize] = if r.next_u32() % 3 == 0 { 3 } else { 5 };
                c.bus.write32(ch + 0x10, ch + 0x40);
                c.bus.write32(ch + 0x44, r.next_u32() % 6);
                c.bus.write32(ch + 0x20, opts + 8 * (r.next_u32() % 8));
            }
            for k in 0..8u32 {
                // option records: signed slot byte, mask half at +2
                c.bus.write8(opts + 8 * k, (r.next_u32() % 8) as u8);
                let mask = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write16(opts + 8 * k + 2, mask as u16);
            }
            for v in keyblocks.iter_mut() {
                *v = (opts + 8 * (r.next_u32() % 8)) as u64;
            }
            for v in rands.iter_mut() {
                *v = r.next_u32() as u64;
            }
            oc427_queue(c, 0x0027_4c20, &channels);
            oc427_queue(c, 0x0027_43c8, &ids);
            oc427_queue(c, 0x0027_42e0, &keyblocks);
            oc427_queue(c, 0x0031_77f0, &rands);
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = sx(script);
            c.g[A2][0] = (r.next_u32() % 3) as u64;
            c.g[A3][0] = sx(players);
        });
    }

    /// `MultiSplineModifier_AllocArrays`: the three allocations (faked
    /// `operator new`), with zero and non-zero element counts.
    #[test]
    fn multi_spline_modifier_alloc_arrays_matches() {
        covered(
            ee::s11::f_0035a328__generated,
            multi_spline_modifier_alloc_arrays,
            100,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x35a328, 3, 0x0035_a328);
                let this = DATA + 0x1000;
                let instance = DATA + 0x2000;
                let model = DATA + 0x2100;
                c.bus.write32(this + 4, r.next_u32() % 5 * (seed % 2));
                c.bus.write32(this + 0x40, instance);
                c.bus.write32(instance + 0x80, model);
                c.bus
                    .write32(model + 4, r.next_u32() % 6 * ((seed / 2) % 2));
                oc427_queue(
                    c,
                    0x0031_7e30,
                    &[
                        (DATA + 0x1_0000) as u64,
                        (DATA + 0x2_0000) as u64,
                        (DATA + 0x3_0000) as u64,
                    ],
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `RiderModel_AddModelRef` (a leaf): appends to the reference list.
    #[test]
    fn rider_model_add_model_ref_matches() {
        check(
            ee::s10::f_0030db70__generated,
            rider_model_add_model_ref,
            50,
            &|c, seed| {
                let mut r = Rng(0x30db_7000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0030_db70;
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x60, r.next_u32() % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `WScriptMan_CollectSetAddItems`: ids that are -1, valid or point at
    /// missing models, both race paths (the byte at `0x535c11`), the
    /// `IsCollected` answers, bones with and without an object whose vtable
    /// method (a fake) is called before the object is rebuilt.
    #[test]
    fn wscript_man_collect_set_add_items_matches() {
        covered(
            ee::s10::f_0030c4a8__generated,
            wscript_man_collect_set_add_items,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x30c4a8, 20, 0x0030_c4a8);
                let pool = DATA + 0x1000;
                let ids = DATA + 0x2000;
                let bones = DATA + 0x7000;
                let size = r.next_u32() % 4;
                c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
                c.bus.write32(pool + 8, r.next_u32() % 4);
                for k in 0..4 {
                    c.bus.write32(ids + 4 * k, oc427_locator_code(&mut r));
                }
                oc427_world_cache(c, &mut r, bones);
                // bones: a +0xc object (or none) with a vtable at +0xc of it
                for b in 0..3u32 {
                    let bone = bones + 0x100 * b;
                    let obj = DATA + 0x7800 + 0x100 * b;
                    let vt = DATA + 0x7c00 + 0x100 * b;
                    c.bus
                        .write32(bone + 0xc, if r.next_u32() % 3 == 0 { 0 } else { obj });
                    c.bus.write32(obj + 0xc, vt);
                    oc427_vtable(c, vt, 8, (r.next_u32() as u16) & !3);
                }
                let mut collected = [0u64; 8];
                for v in collected.iter_mut() {
                    *v = (r.next_u32() % 2) as u64;
                }
                oc427_queue(c, 0x0015_38e8, &collected);
                oc427_queue(
                    c,
                    0x0031_7d70,
                    &[
                        (DATA + 0x1_0000) as u64,
                        (DATA + 0x1_1000) as u64,
                        (DATA + 0x1_2000) as u64,
                        (DATA + 0x1_3000) as u64,
                    ],
                );
                c.g[A0][0] = sx(pool);
                c.g[A1][0] = sx(ids);
                c.g[A2][0] = sx(size);
            },
        );
    }

    /// `BEMissionInt_CountCollectedBitsOnPeak`: with a running game (the
    /// +0x84 field of the world) it forwards to the faked counter, otherwise
    /// it counts the collect bits of every entry whose owner matches
    /// (random bit words over a region covering the profiles and characters).
    #[test]
    fn be_mission_int_count_collected_bits_on_peak_matches() {
        covered(
            ee::s01::f_00153708__generated,
            be_mission_int_count_collected_bits_on_peak,
            120,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x153708, 3, 0x0015_3708);
                let world = DATA + 0x3000;
                c.bus.write32(oc427_gp(-0x848), world);
                c.bus
                    .write32(world + 0x84, if seed % 3 == 0 { 1 } else { 0 });
                for j in 0..22 {
                    c.bus.write32(0x0043_d9a4 + 100 * j, r.next_u32() % 3);
                }
                fill(c, &mut r, 0x004a_6cac, 0x8000);
                for k in 0..0x8000u32 {
                    // sparse bit words: every third word zero
                    if k % 3 == 0 {
                        c.bus.write32(0x004a_6cac + 4 * k, 0);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 3) as u64;
            },
        );
    }

    /// `ScriptViewObj_CutToShot`: shot nodes of every kind (-1, 1..8, >= 7
    /// takes the short form), bits 0 and 1, zero and non-zero lengths
    /// (including a negative one, the unsigned float conversion), a missing
    /// shot function or one without work; the slot binding, weight and
    /// transition callees are the real ones over mostly empty objects.
    #[test]
    fn script_view_obj_cut_to_shot_matches() {
        covered(
            ee::s07::f_00281c68__generated,
            script_view_obj_cut_to_shot,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x281c68, 6, 0x0028_1c68);
                let this = DATA + 0x1000;
                let info = DATA + 0x2000;
                let node = DATA + 0x2100;
                let kind = DATA + 0x2200;
                c.bus.write32(info + 4, node);
                c.bus.write32(node, r.next_u32() % 4);
                c.bus.write32(node + 4, r.next_u32() % 2);
                c.bus.write32(node + 8, r.next_u32() % 100);
                let len = [0, 0, 30, 0xffff_ff00, 7][r.next_u32() as usize % 5];
                c.bus.write32(node + 0x10, len);
                c.bus.write32(node + 0x14, kind);
                let a = [-1i32, 1, 3, 6, 7, 8, 9][r.next_u32() as usize % 7];
                c.bus.write8(kind, a as u8);
                c.bus.write8(kind + 1, (r.next_u32() % 4) as u8);
                c.bus.write16(kind + 2, r.next_u32() as u16);
                c.bus.write16(kind + 4, r.next_u32() as u16);
                for s in 0..2 {
                    c.bus.write32(
                        this + 0xdc + 4 * s,
                        [0xffff_ffff, 0, 1][r.next_u32() as usize % 3],
                    );
                    c.bus.write32(this + 0xd0 + 4 * s, 0);
                }
                // the shot function / transition manager object (+0xd8), all zero
                c.bus.write32(
                    this + 0xd8,
                    if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        DATA + 0x1600
                    },
                );
                let cam = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    (DATA + 0x4000) as u64
                };
                oc427_queue(c, 0x0028_1c40, &[cam, cam]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(info);
                c.f[12] = r.f32(100.0);
            },
        );
    }

    /// `InputParser_ParseFunction1`: `(` expected, the operand parse failing,
    /// `)` expected (the error frees the operand), and the folding of
    /// constants for every function op (0x18..0x21, floor/ceil/wrap over
    /// random floats) against other ops and operand types (a unary node is
    /// built). Entered at the function start only.
    #[test]
    fn input_parser_parse_function1_matches() {
        covered(
            ee::s10::f_00324398__generated,
            input_parser_parse_function1,
            240,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x324398, 6, 0x0032_4398);
                let this = DATA + 0x1000;
                let operand = DATA + 0x2000;
                let node = DATA + 0x3000;
                let handlers = [
                    0x0032_4460u32,
                    0x0032_4470,
                    0x0032_4480,
                    0x0032_4490,
                    0x0032_44a0,
                    0x0032_44b0,
                    0x0032_44dc,
                    0x0032_450c,
                    0x0032_451c,
                    0x0032_452c,
                ];
                for (k, h) in handlers.iter().enumerate() {
                    c.bus.write32(0x0048_e1e0 + 4 * k as u32, *h);
                }
                c.bus.write32(oc427_gp(-0x2e2c), 0x3e22_f983);
                c.bus.write32(oc427_gp(-0x2e28), 0x40c9_0fdb);
                let first = if seed % 7 == 0 { 5 } else { 9 };
                c.bus.write32(this + 0x98, first);
                // after `(`: a token, the operand (or a failed parse), then `)`
                let second = if seed % 5 == 0 { 3 } else { 10 };
                oc427_queue(c, 0x0032_1af0, &[second, 1]);
                let fold = second == 10 && seed % 4 != 0;
                let ty = if fold { 4 } else { 0x40 };
                c.bus.write32(operand, ty);
                c.bus.write32(operand + 4, r.edge_f32());
                oc427_queue(
                    c,
                    0x0032_3268,
                    &[if seed % 11 == 0 { 0 } else { operand as u64 }],
                );
                oc427_queue(c, 0x0032_15f8, &[node as u64]);
                let op = if seed % 6 == 0 {
                    0x40 + r.next_u32() % 4
                } else {
                    0x18 + r.next_u32() % 10
                };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(op);
            },
        );
    }

    /// `GMHandler1_Reset`: every heat (-1..4; heats 0 and 1 are the first
    /// heat), the quick-start flag only with heat 3, the rebuild paths of
    /// `PickOpponents`/`SetupRiderSlots` skipped by their guards (they are
    /// real, unbounded callees): the +8/+0xc latches non-zero and the event
    /// flag at +0x84 not 1. The first-heat vtable call is a fake method.
    #[test]
    fn gm_handler1_reset_matches() {
        covered(ee::s06::f_0023a108__generated, gm_handler1_reset, 200, &|c, seed| {
            let mut r = oc427_base(c, seed, 0x23a108, 14, 0x0023_a108);
            let this = DATA + 0x1000;
            let vt = DATA + 0x1400;
            let gm = DATA + 0x3000;
            let heat = [0xffff_ffffu32, 0, 1, 2, 3, 4][r.next_u32() as usize % 6];
            c.bus.write32(oc427_gp(-0x480), gm);
            c.bus.write32(gm + 0x70, heat);
            c.bus.write32(gm, r.next_u32() % 5);
            c.bus
                .write32(gm + 0x84, [0, 2, 3][r.next_u32() as usize % 3]);
            c.bus.write8(
                0x0053_5c11,
                if heat == 3 {
                    (r.next_u32() % 2) as u8
                } else {
                    0
                },
            );
            c.bus.write32(this, vt);
            oc427_vtable(c, vt, 0x28, (r.next_u32() as u16) & !3);
            c.bus.write32(this + 8, 1);
            c.bus.write32(this + 0xc, 1);
            // character table lookups (faked interfaces; ids 0..2)
            oc427_queue(c, 0x0014_6e98, &[(r.next_u32() % 3) as u64]);
            oc427_queue(c, 0x0014_7398, &[(r.next_u32() % 4) as u64]);
            oc427_queue(c, 0x0014_5750, &[(r.next_u32() % 4) as u64]);
            put_words(c, 0x0053_6708, &[0, 1, 2]);
            c.g[A0][0] = sx(this);
        });
    }

    /// `GameCamera_SnapToDefaultChaseView`: the chase algorithm is built and
    /// destroyed on the stack (constructor, angle and view steps faked, the
    /// view filled with random words; the destructors are the real ones over
    /// a mostly empty object), the camera matrix is rebuilt through VU0.
    #[test]
    fn game_camera_snap_to_default_chase_view_matches() {
        covered(
            ee::s01::f_0015e050__generated,
            game_camera_snap_to_default_chase_view,
            120,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x15e050, 5, 0x0015_e050);
                let this = DATA + 0x1000;
                for w in 0..0x40 {
                    c.bus.write32(this + 4 * w, r.f32(10.0));
                }
                for w in 0..8 {
                    c.bus.write32(DATA + 0x2000 + 4 * w, r.f32(10.0));
                }
                for w in 0..16 {
                    c.bus.write32(0x004c_53a0 + 4 * w, r.f32(2.0));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2010);
            },
        );
    }
}
