//! Staging module oc423 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc423`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_9e50, script_engine_v04, "ScriptEngine_v04"),
    (
        0x0010_f3b8,
        ai_reset_rider_relations,
        "AI_ResetRiderRelations",
    ),
    (0x0027_3530, in_seq_ramp_track_start, "InSeq_RampTrackStart"),
    (
        0x0013_adc0,
        rider_railslide_state_compute_entry_angle,
        "RiderRailslideState_ComputeEntryAngle",
    ),
    (0x0031_4a18, mdf_file_set_data, "MdfFile_SetData"),
    (
        0x0011_5b08,
        rider_get_start_pose_angle,
        "Rider_GetStartPoseAngle",
    ),
    (
        0x0027_b750,
        script_engine_get_rider_path_pose,
        "ScriptEngine_GetRiderPathPose",
    ),
    (0x0027_24e8, in_seq_heap_delete, "InSeq_HeapDelete"),
    (
        0x0027_d330,
        script_engine_add_binding,
        "ScriptEngine_AddBinding",
    ),
    (
        0x0027_98c0,
        script_engine_on_data_added,
        "ScriptEngine_OnDataAdded",
    ),
    (0x0026_f228, replay_reset, "Replay_Reset"),
    (
        0x0014_6a70,
        be_new_player_int_get_event_type_flag,
        "BENewPlayerInt_GetEventTypeFlag",
    ),
    (
        0x0017_1d10,
        camera_trigger_read_point_bound_obj,
        "CameraTrigger_ReadPointBoundObj",
    ),
    (
        0x0022_e550,
        game_view_man_update_split_screen,
        "GameViewMan_UpdateSplitScreen",
    ),
    (
        0x0035_9f88,
        multi_spline_modifier_ctor,
        "MultiSplineModifier_ctor",
    ),
    (0x0027_9d00, script_engine_v02, "ScriptEngine_v02"),
    (
        0x0035_a5f8,
        multi_spline_modifier_insert_node_instances,
        "MultiSplineModifier_InsertNodeInstances",
    ),
    (
        0x0022_d278,
        section_man_is_event_loaded,
        "SectionMan_IsEventLoaded",
    ),
    (
        0x0015_2398,
        be_get_char_profile_event_record,
        "BE_GetCharProfileEventRecord",
    ),
];

/// Call targets of this module's ports.
const OC423_GET_SCRIPT_OBJ: Callee = Callee {
    addr: 0x0027_d320,
    f: ee::s07::ScriptEngine_GetScriptObj,
};
const OC423_CAM_IS_BOUND_TO_PROCESS: Callee = Callee {
    addr: 0x0027_d790,
    f: ee::s07::ScriptObjCam_IsBoundToProcess,
};
const OC423_IS_OBJ_BOUND_TO_PROCESS: Callee = Callee {
    addr: 0x0027_d210,
    f: ee::s07::ScriptEngine_IsObjBoundToProcess,
};
const OC423_CAM_HAS_BINDING: Callee = Callee {
    addr: 0x0028_2ef0,
    f: ee::s07::ScriptObjCam_HasBinding,
};

/// `void cSSXScriptEngine_IsObjInUseByPlayer(ScriptEngine *engine, int index,
/// int val)` (0x279e50, "ScriptEngine_v04"): looks up the script object at
/// `engine + index*4` and forwards `val` by object type: type 1 (a camera
/// object) to `ScriptObjCam_IsBoundToProcess(obj, val)`, type 2 to
/// `ScriptObjCam_HasBinding(obj, val)`, anything else to
/// `ScriptEngine_IsObjBoundToProcess(engine, index, val)`.
pub fn script_engine_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        OC423_GET_SCRIPT_OBJ.call(c, 0x0027_9e74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = 2;
        if c.g[V1][0] == 1 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            OC423_CAM_IS_BOUND_TO_PROCESS.call(c, 0x0027_9e90)?;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                OC423_CAM_HAS_BINDING.call(c, 0x0027_9ebc)?;
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S0][0];
                OC423_IS_OBJ_BOUND_TO_PROCESS.call(c, 0x0027_9eac)?;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const OC423_RIDER_IS_LOADED: Callee = Callee {
    addr: 0x0011_d640,
    f: ee::s00::Rider_IsLoaded,
};
const OC423_RIDER_GET_DIST_TO_FINISH: Callee = Callee {
    addr: 0x0011_3128,
    f: ee::s00::Rider_GetDistToFinish,
};

/// `void cAI_RefreshGroupAI(cAI *ai)` (0x10f3b8, "AI_ResetRiderRelations"):
/// for every rider i below `ai->rider_count` (+0x78) clears the rider's
/// relation table to the other riders: six entries of 0x24 bytes at the start
/// of the rider (`+0` loaded flag, `+4` value from the rider's vtable
/// method +0x40, `+8` default float from `gp-0x7cb0`, `+0xc..+0x20` zero),
/// then stores the distance to the finish (+0x4d8) and `-1` (+0xe8).
pub fn ai_reset_rider_relations(c: &mut Cpu) {
    /// Number of riders (word at `ai + 0x78`).
    const RIDER_COUNT: u32 = 0x78;
    /// Rider pointer array (`ai + 0x28`).
    const RIDERS: u32 = 0x28;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S0, 0xa0);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(RIDER_COUNT));
        c.g[S5][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add(RIDERS));
            c.g[S7][0] = sx((c.g[S5][0] as u32) << 2);
            'riders: loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.sw(frame.at(c, 4), c.g[V0][0]);
                c.sw(frame.at(c, 0), c.g[S0][0]);
                c.g[S3][0] = 0;
                c.g[S1][0] = 0;
                c.g[S4][0] = c.g[FP][0];
                'others: loop {
                    // Entry j of rider i: "is rider j loaded" (0 for j == i).
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = 0;
                    let same = c.g[S5][0] == c.g[S3][0];
                    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if same {
                        c.cycles += 1;
                        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(RIDER_COUNT));
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                        } else {
                            c.cycles += 3;
                            OC423_RIDER_IS_LOADED.call(c, 0x0010_f448)?;
                            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                            c.cycles += 1;
                            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                        }
                    }
                    // Entry j value: rider j's vtable method +0x40.
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(RIDER_COUNT));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    } else {
                        c.cycles += 12;
                        c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, target, 0x0010_f47c)?;
                        c.g[A0][0] = c.lw(frame.at(c, 0));
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[V0][0]);
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    }
                    c.cycles += 26;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7cb0));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A0][0] = ((c.g[S3][0] as i64) < 6) as u64;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(8));
                    // Zero fields +0xc..+0x20, alternating v0/v1 as the compiler did.
                    for (n, off) in [0xcu32, 0x10, 0x14, 0x18, 0x1c, 0x20]
                        .into_iter()
                        .enumerate()
                    {
                        let r = if n % 2 == 0 { V0 } else { V1 };
                        c.g[r][0] = c.lw(c.g[S0][0] as u32);
                        c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.sw((c.g[r][0] as u32).wrapping_add(off), 0);
                    }
                    let more = c.g[A0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
                    if !more {
                        break 'others;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_f420);
                    }
                }
                c.cycles += 13;
                c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[S5][0] = c.lw(frame.at(c, 4));
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                OC423_RIDER_GET_DIST_TO_FINISH.call(c, 0x0010_f508)?;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4d8));
                c.g[V0][0] = u64::MAX;
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(RIDER_COUNT));
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S7][0] = sx((c.g[S5][0] as u32) << 2);
                if c.g[V0][0] == 0 {
                    break 'riders;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_f400);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

const OC423_INSEQ_HEAP_ALLOC: Callee = Callee {
    addr: 0x0027_2308,
    f: ee::s07::InSeq_HeapAlloc,
};
const OC423_RAMP_TRACK_ACTIVATE_KEY: Callee = Callee {
    addr: 0x0027_3860,
    f: ee::s07::InSeq_RampTrackActivateKey,
};
const OC423_RAMP_TRACK_NEXT_KEY: Callee = Callee {
    addr: 0x0027_38f8,
    f: ee::s07::InSeq_RampTrackNextKey,
};

/// `bool InSeq_RampTrackStart(RampTrack *track, RampKeys *keys, Seq *seq,
/// uint *flags)` (0x273530, "cScriptPlayerChannelObject_Play"): starts a ramp
/// track. Stores `keys` (+0x10, also +8), `seq` (+0xc), allocates
/// `keys->count * 0x24` bytes of ramp records from the sequence heap
/// (`*(*(seq + 8) + 8) + 8`) into +0x1c, resets the key index (+0x18) and sets
/// the cursor (+0x14) to the first key (`keys + 4`). Then, as long as the
/// first key has no flags, it sets bit 0 of `*flags`, activates the key under
/// the cursor and advances to the next key until a key with flags (+4) or the
/// end. Returns whether the key list is non-empty.
pub fn in_seq_ramp_track_start(c: &mut Cpu) {
    /// Key list header: `u16 key_count` (+0), `u16 record_count` (+2); keys
    /// follow at +4.
    const KEYS_HEADER: u32 = 4;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
        c.g[S1][0] = c.g[A3][0];
        // 'done_load: reload keys into v1 and return; 'epilogue: return with v1.
        'epilogue: {
            'reload: {
                if c.g[V0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 13;
                c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(2));
                c.g[A1][0] = 0x24;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.mult(0, 0, V1, A1);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lo[0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                OC423_INSEQ_HEAP_ALLOC.call(c, 0x0027_357c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                } else {
                    // Empty countdown over the record count.
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    loop {
                        c.cycles += 7;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_3598);
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                }
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(KEYS_HEADER));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                if c.g[V0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'epilogue;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] |= 1;
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    OC423_RAMP_TRACK_ACTIVATE_KEY.call(c, 0x0027_35ec)?;
                    c.g[A0][0] = c.g[S0][0];
                    OC423_RAMP_TRACK_NEXT_KEY.call(c, 0x0027_35f4)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'epilogue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(4));
                    if c.g[V0][0] != 0 {
                        break 'reload;
                    }
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_35d8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

const OC423_RAILSLIDE_COMPUTE_RAIL_FRAME: Callee = Callee {
    addr: 0x0013_bd80,
    f: ee::s01::RiderRailslideState_ComputeRailFrame,
};

/// `void cRailslideMotion_setCrookedAngle(RiderRailslideState *state, const
/// Vec4 *rail_normal)` (0x13adc0, "RiderRailslideState_ComputeEntryAngle"):
/// the angle between the rider's facing (`rider + 0x1b0`) and the rail,
/// stored clamped to `+-gp[-0x7170]` (about pi/4) at `state + 0x1c`.
///
/// `side = up x n` (`up` from `ComputeRailFrame(state, n)`), angle =
/// `Atan2(facing . side, facing . n)` where the atan2 is the original's
/// `Math_Atan(y/x)` with the quadrant fixed by the signs (`+-pi` from
/// `gp-0x7178/-0x7174`, and `+-pi/2` for a zero denominator).
pub fn rider_railslide_state_compute_entry_angle(c: &mut Cpu) {
    /// The rider pointer in the state (`state + 0x50`).
    const RIDER: u32 = 0x50;
    /// The rider's facing vector.
    const RIDER_FACING: u32 = 0x1b0;
    /// The stored (clamped) angle.
    const CROOKED_ANGLE: u32 = 0x1c;
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.swc1(22, frame.at(c, 0x80));
        c.swc1(21, frame.at(c, 0x78));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(20, frame.at(c, 0x70));
        c.g[A1][0] = c.g[S1][0];
        frame.save_ra(c, 0x40);
        c.g[A2][0] = c.g[S0][0];
        OC423_RAILSLIDE_COMPUTE_RAIL_FRAME.call(c, 0x0013_adf4)?;
        c.mtc1(0, 22);
        // sp[0x20] = up x n (the side axis); vf4 = up, vf5 = n.
        c.lqc2(4, sp(c));
        c.lqc2(5, c.g[S0][0] as u32);
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        c.sqc2(6, frame.at(c, 0x20));
        c.lq(3, frame.at(c, 0x20));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RIDER));
        c.sq(3, frame.at(c, 0x10));
        c.lq(2, (c.g[A0][0] as u32).wrapping_add(RIDER_FACING));
        c.sq(2, frame.at(c, 0x20));
        // f21 = facing . side
        c.lqc2(3, frame.at(c, 0x20));
        c.lqc2(5, frame.at(c, 0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 21);
        // f20 = facing . n
        c.lq(2, (c.g[A0][0] as u32).wrapping_add(RIDER_FACING));
        c.sq(2, frame.at(c, 0x30));
        c.lqc2(3, frame.at(c, 0x30));
        c.lqc2(5, c.g[S0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 20);
        c.c_eq_s(20, 22);
        let gp = c.g[GP][0] as u32;
        if c.fcc() {
            // Zero denominator: 0 for a zero numerator, else -+pi/2.
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let numerator_zero = c.fcc();
            c.mov_s(12, 21);
            if !numerator_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp.wrapping_sub(0x7180));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(12, gp.wrapping_sub(0x717c));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0013_aeb8)?; // Math_Atan
            c.c_lt_s(20, 22);
            let denominator_positive = !c.fcc();
            c.mov_s(12, 0);
            if !denominator_positive {
                // Negative denominator: shift the quadrant by +-pi.
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp.wrapping_sub(0x7178));
                    c.add_s(12, 12, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(0x7174));
                    c.sub_s(12, 12, 0);
                }
            }
        }
        // Clamp to [-limit, limit] and store.
        c.cycles += 6;
        c.lwc1(1, gp.wrapping_sub(0x7170));
        c.neg_s(0, 1);
        c.c_le_s(0, 12);
        let above_lower = c.fcc();
        c.swc1(12, (c.g[S1][0] as u32).wrapping_add(CROOKED_ANGLE));
        if above_lower {
            c.cycles += 1;
            c.min_s(0, 12, 1);
        }
        c.cycles += 9;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(CROOKED_ANGLE));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

const OC423_MDF_PARSE_HEADER: Callee = Callee {
    addr: 0x0031_4aa0,
    f: ee::s10::MdfFile_ParseHeader,
};

/// `void MdfFile_SetData(MdfFile *file, void *data)` (0x314a18): stores the
/// data pointer at `file + 0` and parses its header (`MdfFile_ParseHeader`).
pub fn mdf_file_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[A1][0];
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        OC423_MDF_PARSE_HEADER.call(c, 0x0031_4a30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float Rider_GetStartPoseAngle(Rider *rider)` (0x115b08): the character
/// scale of the rider's character (`rider + 0x86c`) from the BE library's
/// interface 2, as an angle: `scale * 90.0 - 90.0` degrees.
pub fn rider_get_start_pose_angle(c: &mut Cpu) {
    /// The rider's character id.
    const CHARACTER: u32 = 0x86c;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0011_5b1c)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0014_dd58, 0x0011_5b28)?; // BELibrary_GetInterface
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHARACTER));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_efa8, 0x0011_5b34)?; // BECharacterInterface_GetScale
        c.g[AT][0] = 0x42b4_0000; // 90.0f
        c.mtc1(1, 1);
        frame.restore(c, S0, 0x10);
        c.mul_s(0, 0, 1);
        frame.restore_ra(c, 0);
        c.sub_s(0, 0, 1);
        frame.close(c);
        Ok(())
    })
}

const OC423_PATH_DATA_FIND_MARKER: Callee = Callee {
    addr: 0x0026_b5e0,
    f: ee::s07::PathData_FindMarker,
};

/// The original's open-coded `atan2(y = f21, x = f20)` (the caller has done
/// `c_eq_s(20, 22)` against zero in f22): for `x != 0`, `Math_Atan(y / x)`
/// shifted by `+-pi` (`gp - k[2]` / `gp - k[3]`) when `x < 0`; for `x == 0`
/// `y` itself when zero, else `gp - k[0]` (`y < 0`) or `gp - k[1]`. The result
/// is stored at `*dest` (a callee-saved register holding the pointer).
fn oc423_atan2_store(c: &mut Cpu, dest: usize, k: [u32; 4], ret: u32) -> Flow {
    let gp = c.g[GP][0] as u32;
    if !c.fcc() {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, 0x0031_c228, ret)?; // Math_Atan
        c.c_lt_s(20, 22);
        let negative_x = c.fcc();
        c.mov_s(12, 0);
        if negative_x {
            c.cycles += 3;
            c.c_lt_s(22, 21);
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp.wrapping_sub(k[2]));
                c.add_s(12, 12, 0);
            } else {
                c.cycles += 2;
                c.lwc1(0, gp.wrapping_sub(k[3]));
                c.sub_s(12, 12, 0);
            }
        }
        c.cycles += 1;
        c.mov_s(0, 12);
        c.cycles += 1;
        c.swc1(0, c.g[dest][0] as u32);
    } else {
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let y_zero = c.fcc();
        c.mov_s(0, 21);
        if y_zero {
            c.cycles += 1;
            c.swc1(0, c.g[dest][0] as u32);
        } else {
            c.cycles += 4;
            c.c_le_s(22, 21);
            c.lwc1(0, gp.wrapping_sub(k[0]));
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp.wrapping_sub(k[1]));
            }
            c.swc1(0, c.g[dest][0] as u32);
        }
    }
    Ok(())
}

/// `void ScriptEngine_GetRiderPathPose(ScriptEngine*, Rider *rider, Vec4
/// *pos, float *roll, float *pitch, float *yaw)` (0x27b750): the start point
/// of the rider's path (marker 1 of the path set at `0x4d33a0`, for the path id
/// from the rider's sub-object method +0x38/+0x3c): `*pos` = its position
/// (w = 1.0), `*yaw = atan2(dy, dx)`, `*pitch = atan2(dz, sqrt(dx*dx +
/// dy*dy))`, `*roll = 0`. Arguments a2 = `pos`, a3 = `roll`, t0 = `pitch`,
/// t1 = `yaw`.
pub fn script_engine_get_rider_path_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0x4d_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x33a0));
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        c.swc1(22, frame.at(c, 0x80));
        c.g[S4][0] = c.g[A3][0];
        c.swc1(21, frame.at(c, 0x78));
        c.g[S2][0] = c.g[T1][0];
        c.swc1(20, frame.at(c, 0x70));
        frame.save_ra(c, 0x10);
        c.mtc1(0, 22);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_b7ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        OC423_PATH_DATA_FIND_MARKER.call(c, 0x0027_b7bc)?;
        c.g[S0][0] = c.g[V0][0];
        // *pos = (px, py, pz, 1.0)
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        let marker = c.g[S0][0] as u32;
        c.lwc1(0, marker.wrapping_add(0x8));
        c.lwc1(1, marker.wrapping_add(0xc));
        c.lwc1(2, marker.wrapping_add(0x10));
        c.swc1(0, frame.at(c, 0));
        c.swc1(1, frame.at(c, 4));
        c.swc1(2, frame.at(c, 8));
        c.swc1(3, frame.at(c, 0xc));
        c.lq(2, sp(c));
        c.sq(2, c.g[S1][0] as u32);
        // yaw = atan2(dy, dx)
        c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.c_eq_s(20, 22);
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x18));
        oc423_atan2_store(c, S2, [0x47ec, 0x47e8, 0x47e4, 0x47e0], 0x0027_b838)?;
        // pitch = atan2(dz, sqrt(dx*dx + dy*dy))
        c.cycles += 13;
        c.mtc1(0, 22);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.swc1(0, frame.at(c, 0));
        c.mul_s(0, 0, 0);
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
        c.mul_s(2, 1, 1);
        c.swc1(1, frame.at(c, 4));
        c.add_s(0, 0, 2);
        c.sqrt_s(20, 0);
        c.c_eq_s(20, 22);
        oc423_atan2_store(c, S3, [0x47dc, 0x47d8, 0x47d4, 0x47d0], 0x0027_b8dc)?;
        // roll = 0
        c.cycles += 12;
        c.sw(c.g[S4][0] as u32, 0);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

const OC423_INSEQ_HEAP_FREE: Callee = Callee {
    addr: 0x0027_23a8,
    f: ee::s07::InSeq_HeapFree,
};

/// `void InSeq_HeapDelete(void *p)` (0x2724e8): frees an object allocated by
/// `InSeq_HeapNew`: the heap pointer sits in the word `gp[0x398]` bytes before
/// `p`; `InSeq_HeapFree(*header, header)`.
pub fn in_seq_heap_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x398));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        OC423_INSEQ_HEAP_FREE.call(c, 0x0027_2500)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_AddBinding(ScriptEngine *engine, int key, Binding
/// *binding)` (0x27d330): puts the binding into the first free of the 30
/// binding slots (`engine + 0x50 + i*0x14`: binding pointer, key at +4),
/// writes the slot index into `binding + 4` and returns it (30 when full).
/// Slot 0 is checked first and, when free, also keeps the key at +0x54.
pub fn script_engine_add_binding(c: &mut Cpu) {
    /// First binding slot.
    const SLOTS: u32 = 0x50;
    /// Number of slots.
    const SLOT_COUNT: i64 = 30;
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(SLOTS));
    c.g[A3][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[A1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(4), 0);
    } else {
        c.cycles += 1;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < SLOT_COUNT) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = 0x14;
            if full {
                break;
            }
            c.cycles += 6;
            c.mult(0, V0, A3, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SLOTS));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[V1][0] != 0 {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0027_d354);
                }
                continue;
            }
            c.cycles += 3;
            c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
            c.sw((c.g[A2][0] as u32).wrapping_add(4), c.g[A3][0]);
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

const OC423_STRNCMP: u32 = 0x0041_6b18;
const OC423_GET_BANK_LIST: Callee = Callee {
    addr: 0x0027_c078,
    f: ee::s07::ScriptData_GetBankList,
};
const OC423_REGISTER_SCRIPT: Callee = Callee {
    addr: 0x0028_2150,
    f: ee::s07::ScriptEngine_RegisterScript,
};
const OC423_ANIM_BANK_ADD: Callee = Callee {
    addr: 0x0027_5500,
    f: ee::s07::ScriptAnimBankMan_AddBank,
};
const OC423_SOUND_BANK_ADD: Callee = Callee {
    addr: 0x0028_3580,
    f: ee::s07::ScriptSoundBankMan_AddBank,
};

/// `void ScriptEngine_OnDataAdded(ScriptEngine *engine, const char *ext,
/// void *data, uint size, int *kind, int *bank)` (0x2798c0): a data file was
/// loaded; by its extension (3 characters compared with the strings at
/// `0x4a34c8` "scr", `0x4a34d0` "anm", `0x4a34d8` "snd"):
/// - script: the bank list's `+0x1c` gets the engine's current attaching bank
///   (+0xa48), `*kind = 0`, and the script is registered
///   (`ScriptEngine_RegisterScript(engine, data, list->+0x18)`);
/// - animation bank: `*kind = 1`, `*bank = engine->+0xa48`, linked into the
///   anim bank manager (`engine + 0xa28`);
/// - sound bank: `*kind = 2`, `*bank = ...`, linked into the sound bank
///   manager (`engine + 0xa38`, with `size`).
/// Arguments: a0 engine, a1 ext, a2 data, a3 size, t0 kind, t1 bank.
pub fn script_engine_on_data_added(c: &mut Cpu) {
    /// The current attaching bank number.
    const CURRENT_BANK: u32 = 0xa48;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x20);
        c.g[A0][0] = 0x4a_0000;
        frame.save(c, S5, 0x10);
        c.g[S3][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34c8));
        c.g[A2][0] = 3;
        call_ptr(c, OC423_STRNCMP, 0x0027_9908)?;
        c.g[A0][0] = 0x4a_0000;
        'done: {
            if c.g[V0][0] == 0 {
                // Script file.
                c.cycles += 11;
                c.g[A0][0] = c.g[S2][0];
                OC423_GET_BANK_LIST.call(c, 0x0027_9918)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CURRENT_BANK));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.sw(c.g[S3][0] as u32, 0);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                OC423_REGISTER_SCRIPT.call(c, 0x0027_9934)?;
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34d0));
            c.g[A2][0] = 3;
            call_ptr(c, OC423_STRNCMP, 0x0027_994c)?;
            let not_anim = c.g[V0][0] != 0;
            c.g[A0][0] = 0x4a_0000;
            if !not_anim {
                // Animation bank.
                c.cycles += 10;
                c.g[V0][0] = 1;
                c.g[A2][0] = c.g[S2][0];
                c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CURRENT_BANK));
                c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CURRENT_BANK));
                OC423_ANIM_BANK_ADD.call(c, 0x0027_9974)?;
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34d8));
            c.g[A2][0] = 3;
            call_ptr(c, OC423_STRNCMP, 0x0027_998c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            // Sound bank.
            c.cycles += 10;
            c.g[V0][0] = 2;
            c.g[A2][0] = c.g[S2][0];
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            c.g[A3][0] = c.g[S5][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa38));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CURRENT_BANK));
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CURRENT_BANK));
            OC423_SOUND_BANK_ADD.call(c, 0x0027_99b8)?;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
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

const OC423_REPLAY_EMPTY_RESET_HOOK: Callee = Callee {
    addr: 0x0026_f498,
    f: ee::s07::Replay_EmptyResetHook,
};
const OC423_REPLAY_SET_STATE: Callee = Callee {
    addr: 0x0026_f4a0,
    f: ee::s07::Replay_SetState,
};
const OC423_REPLAY_CACHE_RESET: Callee = Callee {
    addr: 0x0026_d168,
    f: ee::s07::ReplayCache_Reset,
};
const OC423_SNAPSHOT_LIST_CLEAR: Callee = Callee {
    addr: 0x0026_e5c0,
    f: ee::s07::SnapshotList_Clear,
};
const OC423_SNAPSHOT_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0026_e5d0,
    f: ee::s07::SnapshotList_PushBack,
};

/// `void Replay_Reset(Replay *replay)` (0x26f228): resets the replay
/// recorder/player unless it is in state 14 or 15 (`SaveState` 15 mid-reset):
/// sets state 15 and the replay state to 10, clears all playback and write
/// bookkeeping fields, releases and re-initialises the two replay caches (their
/// highlight slots at `+0x494`, stride 0xb4, each with 0xac bytes of data at
/// `+0x498`), empties the frame list (`+0x3b0`) and the free list (`+0x3bc`),
/// refills the free list with the `MaxFrames` (+8) frames at `+0x2c` (stride
/// 0x38) and sets the highlight/chapter limits (3 and 3 when the AI object's
/// field +0x7c is 1, else 1 and 0).
pub fn replay_reset(c: &mut Cpu) {
    /// Fields cleared to zero, in the compiler's store order.
    const CLEARED: [u32; 32] = [
        0x28, 0x608, 0x484, 0x3c8, 0x3cc, 0x488, 0x3d4, 0x3ac, 0x3d0, 0x1c, 0x20, 0x618, 0x61c,
        0x620, 0x624, 0x63c, 0x650, 0x640, 0x644, 0x648, 0x638, 0x64c, 0x654, 0x658, 0x65c, 0x660,
        0x664, 0x66c, 0x674, 0x678, 0x67c, 0x668,
    ];
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            // States 14 and 15: nothing to reset.
            frame.restore(c, S0, 0x80);
        } else {
            c.cycles += 51;
            c.g[S2][0] = 0;
            OC423_REPLAY_EMPTY_RESET_HOOK.call(c, 0x0026_f274)?;
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3b0));
            c.g[V0][0] = 0xf;
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0xa;
            OC423_REPLAY_SET_STATE.call(c, 0x0026_f28c)?;
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3bc));
            c.g[S7][0] = u64::MAX;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x494));
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x498));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48c));
            let this = c.g[S0][0] as u32;
            for off in [0x60c, 0x610, 0x614, 0x628, 0x62c] {
                c.sw(this.wrapping_add(off), 0);
            }
            for off in CLEARED {
                c.sw(this.wrapping_add(off), 0);
            }
            // The two highlight slots: release the cache, clear the slot.
            loop {
                c.cycles += 3;
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    OC423_REPLAY_CACHE_RESET.call(c, 0x0026_f34c)?;
                }
                c.sw((c.g[S1][0] as u32).wrapping_add(0xb0), 0);
                c.cycles += 12;
                c.g[A0][0] = c.g[S4][0];
                c.sw(c.g[S1][0] as u32, c.g[S7][0]);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0xac;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb4));
                call_ptr(c, 0x003e_6448, 0x0026_f368)?; // EA_MemSet
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f338);
                }
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S6][0];
            c.g[S2][0] = 0;
            OC423_SNAPSHOT_LIST_CLEAR.call(c, 0x0026_f38c)?;
            c.g[A0][0] = c.g[S5][0];
            OC423_SNAPSHOT_LIST_CLEAR.call(c, 0x0026_f394)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if (c.g[V0][0] as i64) > 0 {
                // Refill the free list with the frames.
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S5][0];
                    OC423_SNAPSHOT_LIST_PUSH_BACK.call(c, 0x0026_f3ac)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f3a0);
                    }
                }
            }
            c.cycles += 9;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x630), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x634), 0);
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7c));
            c.g[A2][0] = 3;
            if c.g[V0][0] != c.g[A1][0] {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
            } else {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const OC423_BE_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::BE_GetRiderProfile,
};

#[cfg(not(feature = "ee-unwind"))]
fn event_type_flag_resume(c: &mut Cpu) {
    ee::s01::f_00146a70__entries(c)
}
#[cfg(feature = "ee-unwind")]
fn event_type_flag_resume(c: &mut Cpu) {
    ee::s01::f_00146a70__generated(c)
}

/// `bool BENewPlayerInt_GetEventTypeFlag(this, int rider, int character, int
/// kind)` (0x146a70): the profile's event flag `kind` (6..11) of `character`:
/// bit `kind + 8` of the 64-bit word at `+0x278` of the profile record
/// (`0x4a6ca8 + profile * 0x9b50 + character * 0xf88`, `profile` from
/// `BE_GetRiderProfile(rider)`); other kinds give 0. The kind goes through
/// the original's jump table at `0x45a1f0` (one shift amount per case).
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn be_new_player_int_get_event_type_flag(c: &mut Cpu) {
    if c.pc != 0x0014_6a70 {
        return event_type_flag_resume(c);
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        OC423_BE_GET_RIDER_PROFILE.call(c, 0x0014_6a90)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(6));
        c.g[A1][0] = c.g[V0][0];
        let in_range = c.g[V1][0] < 6;
        c.g[V0][0] = 0x46_0000;
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5e10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // Each jump-table target is the same code with another shift.
            let shift = match target {
                0x0014_6abc => 18,
                0x0014_6af0 => 17,
                0x0014_6b24 => 16,
                0x0014_6b58 => 15,
                0x0014_6b8c => 14,
                0x0014_6bc0 => 13,
                _ => {
                    c.jump_out(target, 0x0014_6a70, 0x0014_6c0c);
                    return Ok(());
                }
            };
            c.cycles += 13;
            c.g[V1][0] = 0xf88;
            c.g[A0][0] = 0x9b50;
            c.mult(0, V1, S1, V1);
            c.mult(0, A0, A1, A0);
            c.g[V0][0] = 0x4a_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x278));
            c.g[V0][0] <<= shift;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[V0][0] &= 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTrigger_ReadPointBoundObj(Serializer *s, BoundObjPoint *obj)`
/// (0x171d10): marks the bound object as a point (`obj->+0x28 = 3`), asks its
/// owner (`obj->+0x24`, vtable slot +8/+0xc) for the vector to fill, and reads
/// three floats into it (`Serialize_ReadVec3(s, vec)`).
pub fn camera_trigger_read_point_bound_obj(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0017_1d3c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d_25a0, 0x0017_1d48)?; // Serialize_ReadVec3
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const OC423_BE_GET_EVENT_RECORD_INDEX: Callee = Callee {
    addr: 0x0014_ab68,
    f: ee::s01::BE_GetEventRecordIndex,
};

/// `EventRecord *BE_GetCharProfileEventRecord(CharProfile *profile, int a, int
/// b)` (0x152398): the profile's record for the event `BE_GetEventRecordIndex(a,
/// b)` returns: `profile + 0xad0 + index * 8`, or null for index 0x1a (not
/// found).
pub fn be_get_char_profile_event_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        OC423_BE_GET_EVENT_RECORD_INDEX.call(c, 0x0015_23b4)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1a;
        let not_found = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
        if not_found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xad0));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameViewMan_UpdateSplitScreen(GameViewMan *views)` (0x22e550): when
/// the game's view mode (`AI + 0x214`, 0..9) changes (and the game is not in
/// the 1..9 sized-session case read from `AI + 0x28`), stores it at `+0xc` and
/// lays out the views: the screen width and height come from the display
/// object (`gp - 0x854`, +0x10d8, methods +0x50/+0x54 and +0x58/+0x5c; the
/// unsigned int results are converted to float). Modes 0..2 and 6.. use one
/// full-screen view (far clip 30000), modes 3..5 two side-by-side views of half
/// the width (far clip 20000). Then the world's per-view slots (`+0x250`,
/// `+0x2a0` stride 0x50) of the unused views are cleared.
pub fn game_view_man_update_split_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, frame.at(c, 0x20));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
        'done: {
            if c.g[V1][0] == c.g[A0][0] {
                frame.restore(c, S0, 0x10);
                break 'done;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'done;
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 1;
            let first_layout = c.g[V1][0] == c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
            if !first_layout {
                // Screen width -> f20 (unsigned int to float).
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0022_e5c0)?;
                let high_bit = (c.g[V0][0] as i64) < 0;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if high_bit {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[V1][0];
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    c.add_s(20, 20, 20);
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                }
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                // Screen height -> f2.
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0022_e604)?;
                let high_bit = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if high_bit {
                    c.cycles += 6;
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[V1][0];
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    c.add_s(2, 2, 2);
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                }
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                // Lay out the views for the new mode (v1).
                c.cycles += 3;
                let below_6 = ((c.g[V1][0] as i64) < 6) as u64;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
                if below_6 != 0 {
                    c.cycles += 2;
                    let below_3 = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if !below_3 {
                        // Modes 3..5: two views side by side.
                        c.cycles += 20;
                        c.g[AT][0] = 0x3f00_0000; // 0.5
                        c.mtc1(1, 0);
                        c.g[V0][0] = 2;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                        c.mul_s(0, 20, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x51c8));
                        let this = c.g[S0][0] as u32;
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                        c.swc1(2, this.wrapping_add(0x34));
                        c.swc1(0, this.wrapping_add(0x30));
                        c.sw(this.wrapping_add(0x14), 0);
                        c.sw(this.wrapping_add(0x18), 0);
                        c.sw(this.wrapping_add(0x1c), 0);
                        c.swc1(0, this.wrapping_add(0x20));
                        c.swc1(2, this.wrapping_add(0x24));
                        c.swc1(0, this.wrapping_add(0x28));
                        c.sw(this.wrapping_add(0x2c), 0);
                        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.lw(this.wrapping_add(8));
                        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                    } else {
                        oc423_split_single_view(c);
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    oc423_split_single_view(c);
                }
                // Clear the unused views' world slots.
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 1;
                let two_views = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if two_views {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x250), 0);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x2a0), 0);
                } else {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[A2][0] = 0x50;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.mult(0, A0, A0, A2);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x250), 0);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The one-view layout of `GameViewMan_UpdateSplitScreen` (v0 = 1 on entry):
/// view count 1, the full width and height, far clip `gp - 0x51c4`.
fn oc423_split_single_view(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x51c4));
    let this = c.g[S0][0] as u32;
    c.sw(this.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(20, this.wrapping_add(0x20));
    c.swc1(2, this.wrapping_add(0x24));
    c.sw(this.wrapping_add(0x14), 0);
    c.sw(this.wrapping_add(0x18), 0);
    c.sw(this.wrapping_add(0x1c), 0);
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
}

const OC423_SPLINE_INIT_PATH: Callee = Callee {
    addr: 0x0034_51c0,
    f: ee::s11::SplineModifier_InitPath,
};
const OC423_MULTI_SPLINE_SET_SPEED: Callee = Callee {
    addr: 0x0035_a550,
    f: ee::s11::MultiSplineModifier_SetSpeed,
};
const OC423_MULTI_SPLINE_ALLOC_ARRAYS: Callee = Callee {
    addr: 0x0035_a328,
    f: ee::s11::MultiSplineModifier_AllocArrays,
};
const OC423_MULTI_SPLINE_ALLOC_NODE_INSTANCES: Callee = Callee {
    addr: 0x0035_a3f0,
    f: ee::s11::MultiSplineModifier_AllocNodeInstances,
};
const OC423_MULTI_SPLINE_CLONE_INSTANCES: Callee = Callee {
    addr: 0x0035_a458,
    f: ee::s11::MultiSplineModifier_CloneInstances,
};
const OC423_MULTI_SPLINE_UPDATE_NODES: Callee = Callee {
    addr: 0x0035_ac20,
    f: ee::s11::MultiSplineModifier_UpdateNodes,
};
const OC423_MULTI_SPLINE_INSERT_NODE_INSTANCES: Callee = Callee {
    addr: 0x0035_a5f8,
    f: ee::s11::MultiSplineModifier_InsertNodeInstances,
};

/// `MultiSplineModifier *MultiSplineModifier_ctor(MultiSplineModifier *self,
/// const Desc *desc, Instance *instance)` (0x359f88): builds a spline
/// modifier that moves several node instances along a path. Copies the
/// description (node count `desc+0x2c` -> +4, `desc+8` -> +8, speed scale
/// `desc+0x10 * gp[-0x2a2c]` -> +0xc, draw-path flag `desc+0x28 > 0` -> +0x1c,
/// the colour quad `desc+0x18..0x24` -> +0x20), initialises the path
/// (`SplineModifier_InitPath(&self->spline, desc+4)`), sets the speed
/// (`desc+0xc`), picks the start offset (+0x10: `desc+0x14` when non-zero, kept
/// in `[0, path length]`), allocates and fills the node arrays and instances
/// and sets the focus count (+0x34) from instance flag 0x100. Returns `self`.
pub fn multi_spline_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe98));
        c.g[V1][0] |= 0xffff;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x40), c.g[A2][0]);
        c.sw(this, c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sw(this.wrapping_add(0x48), c.g[V1][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        c.sw(this.wrapping_add(0x50), 0);
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.sw(this.wrapping_add(0x18), 0);
        let desc = c.g[S1][0] as u32;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a2c));
        c.g[V0][0] = c.lw(desc.wrapping_add(0x2c));
        c.sw(this.wrapping_add(4), c.g[V0][0]);
        c.g[V1][0] = c.lw(desc.wrapping_add(8));
        c.sw(this.wrapping_add(8), c.g[V1][0]);
        c.lwc1(0, desc.wrapping_add(0x10));
        c.mul_s(0, 0, 1);
        c.swc1(0, this.wrapping_add(0xc));
        c.g[A1][0] = c.lw(desc.wrapping_add(4));
        OC423_SPLINE_INIT_PATH.call(c, 0x0035_9ff8)?;
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_SET_SPEED.call(c, 0x0035_a004)?;
        // Start offset: 0.0 when the speed is not positive, else the +0x54 value.
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
        c.c_le_s(1, 0);
        if !c.fcc() {
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
            c.cycles += 1;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 2;
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 25;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        c.mtc1(0, 4);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        // The colour quad, copied through the stack.
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x1c));
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x24));
        c.swc1(0, sp(c));
        c.swc1(1, frame.at(c, 4));
        c.swc1(2, frame.at(c, 8));
        c.swc1(3, frame.at(c, 0xc));
        c.ldl(2, frame.at(c, 7));
        c.ldr(2, sp(c));
        c.ldl(3, frame.at(c, 0xf));
        c.ldr(3, frame.at(c, 8));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x27));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x2f));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x28));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
        c.c_eq_s(0, 4);
        if !c.fcc() {
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
        }
        // Keep the offset within [0, path length].
        c.cycles += 5;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.c_lt_s(1, 4);
        if c.fcc() {
            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 5;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_ALLOC_ARRAYS.call(c, 0x0035_a0bc)?;
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_ALLOC_NODE_INSTANCES.call(c, 0x0035_a0c4)?;
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_CLONE_INSTANCES.call(c, 0x0035_a0cc)?;
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_UPDATE_NODES.call(c, 0x0035_a0d4)?;
        c.g[A0][0] = c.g[S0][0];
        OC423_MULTI_SPLINE_INSERT_NODE_INSTANCES.call(c, 0x0035_a0dc)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
        c.g[V0][0] &= 0x100;
        let no_focus = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if no_focus {
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x34), 0);
        } else {
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

const OC423_ADD_SCRIPT_OBJ: Callee = Callee {
    addr: 0x0027_cea8,
    f: ee::s07::ScriptEngine_AddScriptObj,
};
const OC423_VIEW_OBJ_ADD_CAM: Callee = Callee {
    addr: 0x0028_1190,
    f: ee::s07::ScriptViewObj_AddCam,
};
const OC423_VIEW_OBJ_SET_TRANSITION: Callee = Callee {
    addr: 0x0028_12e0,
    f: ee::s07::ScriptViewObj_SetTransition,
};

/// `int ScriptEngine_v02(ScriptEngine *engine, ScriptObj *obj)` (0x279d00):
/// adds a script object to the engine (`ScriptEngine_AddScriptObj`) and hooks it
/// up by type: a type-1 object (camera/view) is added to its view's camera
/// list (`engine + 0x6d0 + obj->+0x7c * 0x120`, `ScriptViewObj_AddCam`), a
/// type-2 object asks its owner (vtable +0x48/+0x4c) for a slot and has its
/// transition set (`ScriptViewObj_SetTransition(engine + 0x6d0 + slot * 0x120,
/// obj)`). Returns the new object's index.
pub fn script_engine_v02(c: &mut Cpu) {
    /// First view object (`engine + 0x6d0`, stride 0x120).
    const VIEWS: u32 = 0x6d0;
    const VIEW_SIZE: u64 = 0x120;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        OC423_ADD_SCRIPT_OBJ.call(c, 0x0027_9d20)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 2;
        'done: {
            if c.g[V1][0] == 1 {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = VIEW_SIZE;
                c.g[A1][0] = c.g[S0][0];
                c.mult(0, A0, A0, V0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(VIEWS));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                OC423_VIEW_OBJ_ADD_CAM.call(c, 0x0027_9d50)?;
                c.g[V0][0] = c.g[S2][0];
                break 'done;
            }
            c.cycles += 2;
            let other = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S2][0];
            if other {
                break 'done;
            }
            c.cycles += 12;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0027_9d74)?;
            c.g[V1][0] = VIEW_SIZE;
            c.g[A1][0] = c.g[S0][0];
            c.mult(0, V0, V0, V1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(VIEWS));
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            OC423_VIEW_OBJ_SET_TRANSITION.call(c, 0x0027_9d8c)?;
            c.g[V0][0] = c.g[S2][0];
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

const OC423_WORLD_OBJ_GET_BOUND_BOX: Callee = Callee {
    addr: 0x0035_0570,
    f: ee::s11::WorldObj_GetBoundBox,
};

/// `void MultiSplineModifier_InsertNodeInstances(MultiSplineModifier *self)`
/// (0x35a5f8; the reference's `setupOverlapSystem`): gets the bound box of the
/// instance (`self->+0x40 -> +0xc`) into the stack buffer and, for every node
/// `i` from 1 to `self->+4`, builds the node's box around the instance
/// position (`instance[i]->+0x40`) with `+-radius` (`self->+0x18`) in the node
/// table (`self->+0x3c`, stride 0x60: min at +0x30, max at +0x40) and inserts
/// the instance into the object interface's spatial structure
/// (`getNash()` method calls at `0x328f28` and `0x328c20`).
pub fn multi_spline_modifier_insert_node_instances(c: &mut Cpu) {
    /// Node count (`self + 4`) and the node table (`+0x3c`), node
    /// instances (`+0x44`).
    const COUNT: u32 = 4;
    const NODES: u32 = 0x3c;
    const NODE_INSTANCES: u32 = 0x44;
    /// Node table entry size.
    const NODE_SIZE: u64 = 0x60;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0xa0);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0x70);
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x18));
        frame.save(c, S0, 0xd0);
        c.g[S4][0] = 1;
        frame.save(c, S1, 0xc0);
        c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(COUNT));
        frame.save(c, S2, 0xb0);
        frame.save(c, S5, 0x80);
        frame.save_ra(c, 0x60);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        OC423_WORLD_OBJ_GET_BOUND_BOX.call(c, 0x0035_a63c)?;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(COUNT));
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let has_nodes = c.g[V0][0] != 0;
        c.g[S5][0] = u64::MAX;
        if has_nodes {
            c.cycles += 1;
            c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
            loop {
                c.cycles += 66;
                c.g[A2][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODE_INSTANCES));
                c.g[S0][0] = NODE_SIZE;
                // The extent vector (r, r, r, 0) on the stack at +0x20.
                c.swc1(0, frame.at(c, 0x28));
                c.mult(0, S0, S4, S0);
                c.swc1(0, frame.at(c, 0x20));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.swc1(0, frame.at(c, 0x24));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.sw(frame.at(c, 0x2c), 0);
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODES));
                // box.min = position - extent
                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x30));
                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                // box.max = position + extent
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODE_INSTANCES));
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODES));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x40));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x30));
                c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
                // Insert the node instance (getNash()->insert(item, &box.min)).
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODE_INSTANCES));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S2][0] = c.lw(c.g[A2][0] as u32);
                call_ptr(c, 0x002d_1be0, 0x0035_a6f0)?; // cObjectInterface_getNash
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(NODES));
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = sx(frame.at(c, 0x30));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                call_ptr(c, 0x0032_8f28, 0x0035_a708)?;
                c.g[V0][0] = c.lw(frame.at(c, 0x34));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.lw(frame.at(c, 0x38));
                c.g[A2][0] = c.g[S2][0];
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = c.lw(frame.at(c, 0x3c));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S1][0];
                }
                c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx(frame.at(c, 0x30));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = 0;
                if c.g[T0][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                call_ptr(c, 0x0032_8c20, 0x0035_a748)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(COUNT));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a650);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

const OC423_SECTION_IS_LOADED: Callee = Callee {
    addr: 0x0022_e078,
    f: ee::s05::SectionMan_IsSectionLoaded,
};

/// `int SectionMan_IsEventLoaded(SectionMan *sections, int event)` (0x22d278):
/// whether every section the event needs is loaded. Event `event` is the
/// 0x28-byte record at `0x442488 + event*0x28`: `count` (+4) section ids at
/// `+0x10` (each checked with `SectionMan_IsSectionLoaded`, stopping at the
/// first miss) and two optional extra section-map indexes at `+8` and `+0xc`
/// (negative: none; else the map entry at `0x442168 + index*0x10` is only
/// checked when its field `+4` is non-negative).
pub fn section_man_is_event_loaded(c: &mut Cpu) {
    /// The event records (`0x440000 + 0x2488`) and the section map
    /// (`0x440000 + 0x2168`).
    const EVENT_BASE: u32 = 0x2488;
    const SECTION_MAP: u32 = 0x2168;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x30);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S6][0] = 0x44_0000;
        // The event's own sections.
        loop {
            c.cycles += 8;
            c.g[V0][0] = 0x28;
            c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(EVENT_BASE));
            c.mult(0, S0, S4, V0);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            if !more {
                break;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d2e4)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_d2b0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[S3][0] = 0;
            break;
        }
        // The first extra section (+8).
        'first: {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
            let none = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0x44_0000;
            if none {
                c.cycles += 1;
                c.g[V0][0] = 0x28;
                break 'first;
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SECTION_MAP));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            let unmapped = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x28;
            if unmapped {
                break 'first;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d320)?;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x28;
        }
        // The second extra section (+0xc); the result is s3.
        'second: {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(EVENT_BASE));
            c.mult(0, V0, S4, V0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let none = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0x44_0000;
            if none {
                c.cycles += 1;
                c.g[V0][0] = c.g[S3][0];
                break 'second;
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SECTION_MAP));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            let unmapped = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = c.g[S3][0];
            if unmapped {
                break 'second;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d360)?;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fake, fall_back, hook, machine, FAKE, LOG_COUNT};
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    fn w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Gate and scripted results of this module's fakes (by call index & 7).
    const OC423_CTRL: u32 = DATA + 0xf_e000;
    const OC423_MAGIC: u32 = 0x0423_a11e;
    const OC423_RET: u32 = OC423_CTRL + 0x10;

    /// Log the call like every fake, then return the scripted result for this
    /// call index (sign-extended in v0, raw bits in f0).
    fn oc423_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc == ra {
            let word = c.bus.read32(OC423_RET + 4 * (n & 7));
            c.g[V0][0] = sx(word);
            c.f[0] = word;
        }
    }

    macro_rules! oc423_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC423_CTRL) != OC423_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                oc423_fake_call(c, $addr);
            }
        };
    }
    #[test]
    fn script_engine_v04_matches() {
        check(ee::s07::f_00279e50__generated, script_engine_v04, 150, &|c, seed| {
            machine(c, seed, 0x0027_9e50, 0);
            let engine = DATA;
            let index = seed % 4;
            let obj = DATA + 0x400;
            w(c, engine + 4 * index, obj);
            // Object type: 1 (camera), 2 (binding id), else generic.
            w(c, obj, [1, 2, 0, 5][(seed % 4) as usize]);
            // Camera object unbound: ScriptObjCam_IsBoundToProcess returns early.
            w(c, obj + 0x14, 0);
            // Generic path: IsObjBoundToProcess asks the object's vtable for a
            // count; the real `return 0` leaf 0x1406c0 gives zero entries.
            w(c, obj + 0xc, DATA + 0x500);
            c.bus.write16(DATA + 0x500 + 0x30, 0);
            w(c, DATA + 0x500 + 0x34, 0x0014_06c0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.g[A2][0] = (seed % 3) as u64 * 0x100;
        });
    }

    #[test]
    fn ai_reset_rider_relations_matches() {
        fakes::covered(
            ee::s00::f_0010f3b8__generated,
            ai_reset_rider_relations,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_f3b8, 6);
                let ai = DATA;
                let count = seed % 5; // 0..=4 riders
                w(c, ai + 0x78, count);
                let vt = DATA + 0x8000;
                for k in 0..6u32 {
                    let rider = DATA + 0x1000 * (k + 1);
                    w(c, ai + 0x28 + 4 * k, rider);
                    for i in 0..0x240u32 {
                        w(c, rider + 4 * i, r.next_u32());
                    }
                    // Load step 7 means loaded.
                    w(c, rider + 0x880, if (seed + k) % 2 == 0 { 7 } else { k });
                    w(c, rider + 0x6c0, vt);
                    w(c, rider + 0x4d0, r.next_u32());
                }
                c.bus.write16(vt + 0x40, 0x10);
                w(c, vt + 0x44, fakes::FAKE);
                c.g[A0][0] = ai as u64;
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_start_matches() {
        check(
            ee::s07::f_00273530__generated,
            in_seq_ramp_track_start,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_3530, 0);
                let track = DATA;
                let keys = DATA + 0x1000;
                let seq = DATA + 0x200;
                let flags = DATA + 0x500;
                w(c, flags, r.next_u32());
                // seq -> +8 -> heap object -> +8 -> heap head; free block at +0x418.
                w(c, seq + 8, DATA + 0x300);
                w(c, DATA + 0x308, DATA + 0x400);
                let block = DATA + 0x2000;
                w(c, DATA + 0x418, block);
                w(c, block, 0x400);
                w(c, block + 4, 0);
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write16(keys, count as u16);
                c.bus.write16(keys + 2, count.max(1) as u16);
                for i in 0..4u32 {
                    let key = keys + 4 + 0x18 * i;
                    c.bus.write16(key, 0x18);
                    c.bus.write16(key + 2, (i % count.max(1)) as u16);
                    let stop = match (seed / 5) % 4 {
                        0 => i == 0 && seed % 7 == 0,
                        1 => i == 1,
                        _ => false,
                    };
                    c.bus.write16(key + 4, stop as u16);
                    w(c, key + 8, r.next_u32());
                    w(c, key + 0xc, r.next_u32());
                    w(c, key + 0x10, r.next_u32());
                    w(c, key + 0x14, r.next_u32());
                }
                c.g[A0][0] = track as u64;
                c.g[A1][0] = keys as u64;
                c.g[A2][0] = seq as u64;
                c.g[A3][0] = flags as u64;
            },
        );
    }

    /// The rider's facing against the rail normal in all quadrants, with
    /// zero dot products (facing zero, or orthogonal to the rail normal).
    #[test]
    fn rider_railslide_state_compute_entry_angle_matches() {
        check(
            ee::s01::f_0013adc0__generated,
            rider_railslide_state_compute_entry_angle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_adc0, 0);
                let state = DATA + 0x100;
                let rider = DATA + 0x400;
                let n = DATA + 0x200;
                w(c, state + 0x10, r.f32(3.0));
                w(c, state + 0x50, rider);
                w(c, GAME_GP - 0x7118, 0.92f32.to_bits());
                put_words(c, 0x004f_f160, &[0, 0, 0x3f80_0000, 0]);
                // Math constants: -pi/2, pi/2 ... and the clamp limit pi/4.
                w(c, GAME_GP - 0x7180, (-1.5707964f32).to_bits());
                w(c, GAME_GP - 0x717c, 1.5707964f32.to_bits());
                w(c, GAME_GP - 0x7178, 3.1415927f32.to_bits());
                w(c, GAME_GP - 0x7174, 3.1415927f32.to_bits());
                w(c, GAME_GP - 0x7170, 0.7853982f32.to_bits());
                // Rail normal: mostly up (z) or arbitrary.
                let nz = if seed % 2 == 0 {
                    0.5f32.to_bits()
                } else {
                    r.f32(1.0)
                };
                put_words(c, n, &[r.f32(0.5), r.f32(0.5), nz, 0]);
                // Facing vector (+0x1b0, and +0x1c0 for ComputeRailFrame).
                let facing = match seed % 6 {
                    0 => [0, 0, 0, 0],
                    // orthogonal to n when n = (0,0,1): z = 0
                    1 => {
                        put_words(c, n, &[0, 0, 0x3f80_0000, 0]);
                        [r.f32(1.0), r.f32(1.0), 0, 0]
                    }
                    _ => [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                };
                put_words(c, rider + 0x1b0, &facing);
                put_words(c, rider + 0x1c0, &[r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(n);
            },
        );
    }

    #[test]
    fn mdf_file_set_data_matches() {
        check(ee::s10::f_00314a18__generated, mdf_file_set_data, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_4a18, 0);
            let file = DATA;
            let data = DATA + 0x100;
            for i in 0..8u32 {
                w(c, data + 4 * i, r.next_u32());
            }
            c.bus.write16(data + 6, (seed % 0x40) as u16);
            c.g[A0][0] = file as u64;
            c.g[A1][0] = data as u64;
        });
    }

    oc423_fake!(
        oc423_fake_be_get_instance,
        OC423_PREV_BE_GET_INSTANCE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc423_fake!(
        oc423_fake_be_get_interface,
        OC423_PREV_BE_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc423_fake!(
        oc423_fake_be_get_scale,
        OC423_PREV_BE_GET_SCALE,
        0x0014_efa8,
        ee::s01::f_0014efa8__generated
    );

    /// The BE library chain faked (instance, interface 2, scale); the scale
    /// comes back scripted as a float.
    #[test]
    fn rider_get_start_pose_angle_matches() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc423_fake_be_get_instance,
            &OC423_PREV_BE_GET_INSTANCE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc423_fake_be_get_interface,
            &OC423_PREV_BE_GET_INTERFACE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EFA8),
            oc423_fake_be_get_scale,
            &OC423_PREV_BE_GET_SCALE,
        );
        fakes::covered(
            ee::s00::f_00115b08__generated,
            rider_get_start_pose_angle,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_5b08, 3);
                w(c, OC423_CTRL, OC423_MAGIC);
                w(c, OC423_RET + 8, r.f32(3.0));
                let rider = DATA;
                w(c, rider + 0x86c, r.next_u32());
                c.g[A0][0] = rider as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_find_marker,
        OC423_PREV_FIND_MARKER,
        0x0026_b5e0,
        ee::s07::f_0026b5e0__generated
    );

    /// The path start marker (position, direction) from a faked
    /// `PathData_FindMarker`; direction components zero, positive or negative
    /// so every quadrant and zero-denominator branch of both atan2s is taken.
    #[test]
    fn script_engine_get_rider_path_pose_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026B5E0),
            oc423_fake_find_marker,
            &OC423_PREV_FIND_MARKER,
        );
        fakes::covered(
            ee::s07::f_0027b750__generated,
            script_engine_get_rider_path_pose,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_b750, 2);
                w(c, OC423_CTRL, OC423_MAGIC);
                let marker = DATA + 0x800;
                // Call 0 is the rider's method (plain fake), call 1 FindMarker.
                w(c, OC423_RET + 4, marker);
                let pick = |k: u32| [0.0f32, 1.0, -1.0, 0.5, -2.0][(k % 5) as usize].to_bits();
                put_words(c, marker + 8, &[r.f32(5.0), r.f32(5.0), r.f32(5.0)]);
                put_words(
                    c,
                    marker + 0x14,
                    &[pick(seed), pick(seed / 5), pick(seed / 25 + seed / 3)],
                );
                for (i, k) in [
                    0x47ec, 0x47e8, 0x47e4, 0x47e0, 0x47dc, 0x47d8, 0x47d4, 0x47d0,
                ]
                .into_iter()
                .enumerate()
                {
                    let v = [-1.5707964f32, 1.5707964, 3.1415927, 3.1415927][i % 4];
                    w(c, GAME_GP - k, v.to_bits());
                }
                let rider = DATA + 0x1000;
                let vt = DATA + 0x2000;
                w(c, rider + 0x6c0, vt);
                c.bus.write16(vt + 0x38, 0x20);
                w(c, vt + 0x3c, FAKE);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = (DATA + 0x340) as u64;
                c.g[T0][0] = (DATA + 0x320) as u64;
                c.g[T1][0] = (DATA + 0x310) as u64;
            },
        );
    }

    #[test]
    fn in_seq_heap_delete_matches() {
        check(ee::s07::f_002724e8__generated, in_seq_heap_delete, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x0027_24e8, 0);
            let header_size = 0x10u32;
            w(c, GAME_GP + 0x398, header_size);
            let heap = DATA;
            // Empty free list; the block's size header at header - 8.
            w(c, heap + 8, 0);
            w(c, heap, r.next_u32());
            let header = DATA + 0x1010;
            w(c, header, heap);
            w(c, header - 8, 0x30);
            w(c, header - 4, 0);
            // Every third call frees a null header.
            c.g[A0][0] = if seed % 3 == 0 {
                header_size as u64
            } else {
                (header + header_size) as u64
            };
        });
    }

    #[test]
    fn script_engine_add_binding_matches() {
        check(
            ee::s07::f_0027d330__generated,
            script_engine_add_binding,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_d330, 0);
                let engine = DATA;
                let binding = DATA + 0x1000;
                // Slots 0..first_free are taken; seed % 32 > 30 fills all 30.
                let first_free = seed % 32;
                for i in 0..30u32 {
                    let taken = if i < first_free {
                        1 + r.next_u32() % 100
                    } else {
                        0
                    };
                    w(c, engine + 0x50 + 0x14 * i, taken);
                    w(c, engine + 0x50 + 0x14 * i + 4, r.next_u32());
                }
                w(c, binding + 4, r.next_u32());
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = binding as u64;
            },
        );
    }

    // The real ScriptData_GetBankList forwards to InSeq_GetHeaderExtra.
    oc423_fake!(
        oc423_fake_header_extra,
        OC423_PREV_HEADER_EXTRA,
        0x0027_4c38,
        ee::s07::f_00274c38__generated
    );
    oc423_fake!(
        oc423_fake_register_script,
        OC423_PREV_REGISTER_SCRIPT,
        0x0028_2150,
        ee::s07::f_00282150__generated
    );
    // The real ScriptAnimBankMan_AddBank takes its short path (bank found).
    oc423_fake!(
        oc423_fake_anim_find_bank,
        OC423_PREV_ANIM_FIND_BANK,
        0x0027_5718,
        ee::s07::f_00275718__generated
    );
    oc423_fake!(
        oc423_fake_sound_bank_add,
        OC423_PREV_SOUND_BANK_ADD,
        0x0028_3580,
        ee::s07::f_00283580__generated
    );

    /// Script, animation and sound extensions and an unknown one; the bank
    /// lookups and registrations faked.
    #[test]
    fn script_engine_on_data_added_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274C38),
            oc423_fake_header_extra,
            &OC423_PREV_HEADER_EXTRA,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282150),
            oc423_fake_register_script,
            &OC423_PREV_REGISTER_SCRIPT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00275718),
            oc423_fake_anim_find_bank,
            &OC423_PREV_ANIM_FIND_BANK,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283580),
            oc423_fake_sound_bank_add,
            &OC423_PREV_SOUND_BANK_ADD,
        );
        fakes::covered(
            ee::s07::f_002798c0__generated,
            script_engine_on_data_added,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_98c0, 2);
                w(c, OC423_CTRL, OC423_MAGIC);
                let bank_list = DATA + 0x700;
                w(c, OC423_RET, bank_list);
                w(c, bank_list + 0x18, r.next_u32());
                // The extension strings in the game's rodata.
                w(c, 0x004a_34c8, 0x0072_6373); // "scr"
                w(c, 0x004a_34d0, 0x006d_6e61); // "anm"
                w(c, 0x004a_34d8, 0x0064_6e73); // "snd"
                let ext = DATA + 0x600;
                w(
                    c,
                    ext,
                    [0x0072_6373, 0x006d_6e61, 0x0064_6e73, 0x007a_7978][(seed % 4) as usize],
                );
                let engine = DATA;
                w(c, engine + 0xa48, r.next_u32());
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = ext as u64;
                c.g[A2][0] = (DATA + 0x800) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = (DATA + 0x900) as u64;
                c.g[T1][0] = (DATA + 0x910) as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_replay_set_state,
        OC423_PREV_REPLAY_SET_STATE,
        0x0026_f4a0,
        ee::s07::f_0026f4a0__generated
    );
    oc423_fake!(
        oc423_fake_replay_cache_reset,
        OC423_PREV_REPLAY_CACHE_RESET,
        0x0026_d168,
        ee::s07::f_0026d168__generated
    );
    oc423_fake!(
        oc423_fake_snapshot_clear,
        OC423_PREV_SNAPSHOT_CLEAR,
        0x0026_e5c0,
        ee::s07::f_0026e5c0__generated
    );
    oc423_fake!(
        oc423_fake_snapshot_push_back,
        OC423_PREV_SNAPSHOT_PUSH_BACK,
        0x0026_e5d0,
        ee::s07::f_0026e5d0__generated
    );

    /// Resettable and non-resettable states (13/14/15/16), 0..3 frames, the
    /// two cache slots set or not, both AI player-count variants.
    #[test]
    fn replay_reset_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc423_fake_replay_set_state,
            &OC423_PREV_REPLAY_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D168),
            oc423_fake_replay_cache_reset,
            &OC423_PREV_REPLAY_CACHE_RESET,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5C0),
            oc423_fake_snapshot_clear,
            &OC423_PREV_SNAPSHOT_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5D0),
            oc423_fake_snapshot_push_back,
            &OC423_PREV_SNAPSHOT_PUSH_BACK,
        );
        fakes::covered(ee::s07::f_0026f228__generated, replay_reset, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x0026_f228, 8);
            w(c, OC423_CTRL, OC423_MAGIC);
            let replay = DATA;
            // Dirty every field the reset clears.
            for i in 0..0x700u32 {
                w(c, replay + 4 * i, r.next_u32() | 1);
            }
            w(
                c,
                replay,
                [13, 14, 15, 16, 0, 2, 13, 13][(seed % 8) as usize],
            );
            w(c, replay + 8, seed / 8 % 4);
            for slot in 0..2u32 {
                let cache = if (seed / 32 + slot) % 2 == 0 {
                    0
                } else {
                    DATA + 0x3000
                };
                w(c, replay + 0x48c + 4 * slot, cache);
            }
            let world = DATA + 0x4000;
            let manager = DATA + 0x4100;
            let ai = DATA + 0x4200;
            w(c, GAME_GP - 0x848, world);
            w(c, world + 0x84, manager);
            w(c, manager + 0xc, ai);
            w(c, ai + 0x7c, if seed % 3 == 0 { 1 } else { 2 });
            c.g[A0][0] = replay as u64;
        });
    }

    oc423_fake!(
        oc423_fake_be_get_rider_profile,
        OC423_PREV_BE_GET_RIDER_PROFILE,
        0x0014_a0e0,
        ee::s01::f_0014a0e0__generated
    );

    /// Every kind 4..=13 (6..11 select the flag bit), three characters and
    /// profiles, random 64-bit flag words; the profile lookup faked.
    #[test]
    fn be_new_player_int_get_event_type_flag_matches() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014A0E0),
            oc423_fake_be_get_rider_profile,
            &OC423_PREV_BE_GET_RIDER_PROFILE,
        );
        fakes::covered(
            ee::s01::f_00146a70__generated,
            be_new_player_int_get_event_type_flag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_6a70, 1);
                c.pc = 0x0014_6a70;
                w(c, OC423_CTRL, OC423_MAGIC);
                let profile = seed % 3;
                let character = (seed / 3) % 4;
                let kind = 4 + seed % 10;
                w(c, OC423_RET, profile);
                // The original's jump table at 0x45a1f0.
                put_words(
                    c,
                    0x0045_a1f0,
                    &[
                        0x0014_6abc,
                        0x0014_6af0,
                        0x0014_6b24,
                        0x0014_6b58,
                        0x0014_6b8c,
                        0x0014_6bc0,
                    ],
                );
                let record = 0x004a_6ca8 + profile * 0x9b50 + character * 0xf88;
                c.bus.write64(
                    record + 0x278,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = character as u64;
                c.g[A3][0] = kind as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_serialize_read_u32,
        OC423_PREV_SERIALIZE_READ_U32,
        0x002d_2510,
        ee::s09::f_002d2510__generated
    );

    fn hook_serialize_read_u32() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2510),
            oc423_fake_serialize_read_u32,
            &OC423_PREV_SERIALIZE_READ_U32,
        );
    }

    /// The owner's vector method is a plain fake; the real `Serialize_ReadVec3`
    /// reads three words through the faked `Serialize_ReadU32`.
    #[test]
    fn camera_trigger_read_point_bound_obj_matches() {
        hook_serialize_read_u32();
        fakes::covered(
            ee::s02::f_00171d10__generated,
            camera_trigger_read_point_bound_obj,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0017_1d10, 5);
                w(c, OC423_CTRL, OC423_MAGIC);
                for i in 1..4 {
                    w(c, OC423_RET + 4 * i, r.next_u32() % 0x1000);
                }
                let obj = DATA + 0x5_0000;
                let vt = DATA + 0x5_1000;
                w(c, obj + 0x24, vt);
                c.bus.write16(vt + 8, 0x10);
                w(c, vt + 0xc, FAKE);
                w(c, obj + 0x28, r.next_u32());
                c.g[A0][0] = (DATA + 0x5_2000) as u64;
                c.g[A1][0] = obj as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_be_event_record_index,
        OC423_PREV_BE_EVENT_RECORD_INDEX,
        0x0014_ab68,
        ee::s01::f_0014ab68__generated
    );

    #[test]
    fn be_get_char_profile_event_record_matches() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AB68),
            oc423_fake_be_event_record_index,
            &OC423_PREV_BE_EVENT_RECORD_INDEX,
        );
        fakes::covered(
            ee::s01::f_00152398__generated,
            be_get_char_profile_event_record,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2398, 1);
                w(c, OC423_CTRL, OC423_MAGIC);
                // Index 0x1a is "not found"; the rest are record slots.
                w(c, OC423_RET, if seed % 4 == 0 { 0x1a } else { seed % 26 });
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// New view modes 0..8 (one view, two views, one view again), the width and
    /// height methods answered by the faked `Serialize_ReadU32` (so the
    /// unsigned-to-float high-bit branch is reachable through its scripted
    /// negative results).
    #[test]
    fn game_view_man_update_split_screen_matches() {
        hook_serialize_read_u32();
        fakes::covered(
            ee::s05::f_0022e550__generated,
            game_view_man_update_split_screen,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_e550, 2);
                w(c, OC423_CTRL, OC423_MAGIC);
                let sizes = [0x500u32, 0x2d0, 0xffff_ffff, 0x8000_0001, 0x280];
                w(c, OC423_RET, sizes[(seed / 5 % 5) as usize]);
                w(c, OC423_RET + 4, sizes[(seed / 3 % 5) as usize]);
                let views = DATA;
                let display = DATA + 0x2000;
                let display_vt = DATA + 0x4000;
                let world = DATA + 0x5000;
                let game = DATA + 0x5100;
                let slots_ptr = DATA + 0x5300;
                let table = DATA + 0x6000;
                let cam1 = DATA + 0x7000;
                w(c, GAME_GP - 0x848, world);
                w(c, GAME_GP - 0x854, display);
                w(c, GAME_GP - 0x51c8, 20000.0f32.to_bits());
                w(c, GAME_GP - 0x51c4, 30000.0f32.to_bits());
                w(c, world + 0x84, game);
                let mode = seed % 9;
                w(c, game + 0x214, mode);
                w(c, game + 0x28, DATA + 0x5200);
                w(
                    c,
                    DATA + 0x5200,
                    [0, 0, 20, 30, 5, 10, 0, 1][(seed / 7 % 8) as usize],
                );
                w(c, game + 0x10, slots_ptr);
                w(c, slots_ptr, table);
                for i in 0..0x30u32 {
                    w(c, table + 0x250 + 4 * i, r.next_u32() | 1);
                }
                w(c, display + 0x10d8, display_vt);
                c.bus.write16(display_vt + 0x50, 0x10);
                w(c, display_vt + 0x54, 0x002d_2510);
                c.bus.write16(display_vt + 0x58, 0x20);
                w(c, display_vt + 0x5c, 0x002d_2510);
                w(c, views, if seed % 4 == 0 { 1 } else { 0 });
                w(c, views + 4, DATA + 0x7800);
                w(c, views + 8, cam1);
                w(c, cam1 + 0xb0, seed / 2 % 2);
                w(
                    c,
                    views + 0xc,
                    if seed % 5 == 0 {
                        mode
                    } else {
                        r.next_u32() % 9
                    },
                );
                w(c, views + 0x10, r.next_u32());
                c.g[A0][0] = views as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_spline_init_path,
        OC423_PREV_SPLINE_INIT_PATH,
        0x0034_51c0,
        ee::s11::f_003451c0__generated
    );
    oc423_fake!(
        oc423_fake_multi_spline_set_speed,
        OC423_PREV_MULTI_SPLINE_SET_SPEED,
        0x0035_a550,
        ee::s11::f_0035a550__generated
    );
    oc423_fake!(
        oc423_fake_multi_spline_alloc_node_instances,
        OC423_PREV_MULTI_SPLINE_ALLOC_NODE_INSTANCES,
        0x0035_a3f0,
        ee::s11::f_0035a3f0__generated
    );
    oc423_fake!(
        oc423_fake_multi_spline_update_nodes,
        OC423_PREV_MULTI_SPLINE_UPDATE_NODES,
        0x0035_ac20,
        ee::s11::f_0035ac20__generated
    );
    oc423_fake!(
        oc423_fake_world_obj_get_bound_box,
        OC423_PREV_WORLD_OBJ_GET_BOUND_BOX,
        0x0035_0570,
        ee::s11::f_00350570__generated
    );
    oc423_fake!(
        oc423_fake_ea_alloc,
        OC423_PREV_EA_ALLOC,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc423_fake!(
        oc423_fake_ea_mem_copy,
        OC423_PREV_EA_MEM_COPY,
        0x003e_6574,
        ee::s14::f_003e6574__generated
    );

    /// Callees shared by the multi-spline modifier tests: the allocator and
    /// the path/instance helpers are faked; every fake returns the `POOL`
    /// block (all words pointing at itself).
    const OC423_POOL: u32 = DATA + 0x2_0000;

    fn hook_multi_spline_callees() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003451C0),
            oc423_fake_spline_init_path,
            &OC423_PREV_SPLINE_INIT_PATH,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0035A550),
            oc423_fake_multi_spline_set_speed,
            &OC423_PREV_MULTI_SPLINE_SET_SPEED,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0035A3F0),
            oc423_fake_multi_spline_alloc_node_instances,
            &OC423_PREV_MULTI_SPLINE_ALLOC_NODE_INSTANCES,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0035AC20),
            oc423_fake_multi_spline_update_nodes,
            &OC423_PREV_MULTI_SPLINE_UPDATE_NODES,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00350570),
            oc423_fake_world_obj_get_bound_box,
            &OC423_PREV_WORLD_OBJ_GET_BOUND_BOX,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc423_fake_ea_alloc,
            &OC423_PREV_EA_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc423_fake_ea_mem_copy,
            &OC423_PREV_EA_MEM_COPY,
        );
    }

    fn multi_spline_pool(c: &mut Cpu) {
        w(c, OC423_CTRL, OC423_MAGIC);
        for i in 0..8u32 {
            w(c, OC423_RET + 4 * i, OC423_POOL);
        }
        for i in 0..0x100u32 {
            w(c, OC423_POOL + 4 * i, OC423_POOL);
        }
    }

    /// One node (so the real AllocArrays/CloneInstances/InsertNodeInstances
    /// loops end at once), speeds of both signs, offsets inside and outside
    /// the path, draw-path and focus flags on and off.
    #[test]
    fn multi_spline_modifier_ctor_matches() {
        hook_multi_spline_callees();
        fakes::covered(
            ee::s11::f_00359f88__generated,
            multi_spline_modifier_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_9f88, 8);
                multi_spline_pool(c);
                let this = DATA;
                let desc = DATA + 0x1000;
                let instance = DATA + 0x2000;
                let pick = |k: u32| [0.0f32, 1.0, -1.0, 0.5, 3.0][(k % 5) as usize].to_bits();
                w(c, GAME_GP - 0x2a2c, 1.0f32.to_bits());
                w(c, desc + 0x2c, seed % 2);
                w(c, desc + 4, r.next_u32());
                w(c, desc + 8, r.next_u32());
                w(c, desc + 0xc, pick(seed));
                w(c, desc + 0x10, pick(seed / 5));
                w(c, desc + 0x14, pick(seed / 3));
                for i in 0..4u32 {
                    w(c, desc + 0x18 + 4 * i, r.f32(1.0));
                }
                w(c, desc + 0x28, [0, 7, 0xffff_ffff][(seed / 2 % 3) as usize]);
                w(
                    c,
                    instance + 8,
                    if seed % 3 == 0 { 0x100 } else { 0 } | r.next_u32() & 0xff,
                );
                // Path length (+0x54) and offset (+0x10) of the half-built modifier.
                w(c, this + 0x54, pick(seed / 7));
                w(c, this + 0x10, r.f32(2.0));
                c.g[A0][0] = this as u64;
                c.g[A1][0] = desc as u64;
                c.g[A2][0] = instance as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_add_script_obj,
        OC423_PREV_ADD_SCRIPT_OBJ,
        0x0027_cea8,
        ee::s07::f_0027cea8__generated
    );
    oc423_fake!(
        oc423_fake_view_obj_add_cam,
        OC423_PREV_VIEW_OBJ_ADD_CAM,
        0x0028_1190,
        ee::s07::f_00281190__generated
    );
    oc423_fake!(
        oc423_fake_view_obj_set_transition,
        OC423_PREV_VIEW_OBJ_SET_TRANSITION,
        0x0028_12e0,
        ee::s07::f_002812e0__generated
    );

    /// Camera objects (type 1), objects with an owner method (type 2) and
    /// others; the engine's list/transition helpers faked.
    #[test]
    fn script_engine_v02_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027CEA8),
            oc423_fake_add_script_obj,
            &OC423_PREV_ADD_SCRIPT_OBJ,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00281190),
            oc423_fake_view_obj_add_cam,
            &OC423_PREV_VIEW_OBJ_ADD_CAM,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002812E0),
            oc423_fake_view_obj_set_transition,
            &OC423_PREV_VIEW_OBJ_SET_TRANSITION,
        );
        fakes::covered(ee::s07::f_00279d00__generated, script_engine_v02, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x0027_9d00, 3);
            w(c, OC423_CTRL, OC423_MAGIC);
            w(c, OC423_RET, r.next_u32() % 40);
            let obj = DATA + 0x100;
            let vt = DATA + 0x300;
            w(c, obj, [1, 2, 0, 3][(seed % 4) as usize]);
            w(c, obj + 0x7c, seed % 3);
            w(c, obj + 0xc, vt);
            c.bus.write16(vt + 0x48, 0x10);
            w(c, vt + 0x4c, FAKE);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = obj as u64;
        });
    }

    oc423_fake!(
        oc423_fake_get_nash,
        OC423_PREV_GET_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated
    );
    oc423_fake!(
        oc423_fake_nash_insert,
        OC423_PREV_NASH_INSERT,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    oc423_fake!(
        oc423_fake_nash_insert_box,
        OC423_PREV_NASH_INSERT_BOX,
        0x0032_8c20,
        ee::s10::f_00328c20__generated
    );

    /// One to four nodes (1 to 3 loop iterations) around a position with
    /// random floats; the spatial-structure calls faked (their float/int
    /// stack-buffer words select the conditional moves).
    #[test]
    fn multi_spline_modifier_insert_node_instances_matches() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00350570),
            oc423_fake_world_obj_get_bound_box,
            &OC423_PREV_WORLD_OBJ_GET_BOUND_BOX,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc423_fake_get_nash,
            &OC423_PREV_GET_NASH,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc423_fake_nash_insert,
            &OC423_PREV_NASH_INSERT,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328C20),
            oc423_fake_nash_insert_box,
            &OC423_PREV_NASH_INSERT_BOX,
        );
        fakes::covered(
            ee::s11::f_0035a5f8__generated,
            multi_spline_modifier_insert_node_instances,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_a5f8, 10);
                w(c, OC423_CTRL, OC423_MAGIC);
                for i in 0..8u32 {
                    w(c, OC423_RET + 4 * i, DATA + 0x1_0000 + 0x100 * i);
                }
                let this = DATA;
                let instance = DATA + 0x1000;
                let nodes = DATA + 0x3000;
                let node_instances = DATA + 0x4000;
                w(c, this + 4, 1 + seed % 4);
                w(c, this + 0x18, r.f32(4.0));
                w(c, this + 0x3c, nodes);
                w(c, this + 0x40, instance);
                w(c, this + 0x44, node_instances);
                w(c, instance + 0xc, DATA + 0x1100);
                for i in 0..4u32 {
                    let item = DATA + 0x5000 + 0x100 * i;
                    w(c, node_instances + 4 * i, item);
                    for k in 0..4u32 {
                        w(c, item + 0x40 + 4 * k, r.f32(10.0));
                    }
                }
                for i in 0..0x200u32 {
                    w(c, nodes + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    oc423_fake!(
        oc423_fake_section_is_loaded,
        OC423_PREV_SECTION_IS_LOADED,
        0x0022_e078,
        ee::s05::f_0022e078__generated
    );

    /// Events with 0..3 own sections and optional extra sections (unmapped,
    /// mapped); the section-loaded check scripted per call (loaded or not).
    #[test]
    fn section_man_is_event_loaded_matches() {
        hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E078),
            oc423_fake_section_is_loaded,
            &OC423_PREV_SECTION_IS_LOADED,
        );
        fakes::covered(
            ee::s05::f_0022d278__generated,
            section_man_is_event_loaded,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_d278, 6);
                w(c, OC423_CTRL, OC423_MAGIC);
                for i in 0..8u32 {
                    // Mostly loaded, sometimes not.
                    w(
                        c,
                        OC423_RET + 4 * i,
                        if r.next_u32() % 4 == 0 { 0 } else { 1 },
                    );
                }
                let event = seed % 3;
                let record = 0x0044_2488 + 0x28 * event;
                w(c, record + 4, seed / 3 % 4);
                for i in 0..4u32 {
                    w(c, record + 0x10 + 4 * i, r.next_u32() % 8);
                }
                let extra = |k: u32| match k % 3 {
                    0 => 0xffff_ffffu32,
                    1 => 1,
                    _ => 2,
                };
                w(c, record + 8, extra(seed / 12));
                w(c, record + 0xc, extra(seed / 36));
                for i in 0..3u32 {
                    let entry = 0x0044_2168 + 0x10 * i;
                    w(
                        c,
                        entry + 4,
                        if (seed / 7 + i) % 2 == 0 {
                            0xffff_ffff
                        } else {
                            i
                        },
                    );
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = event as u64;
            },
        );
    }
}
