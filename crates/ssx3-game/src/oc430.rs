//! Staging module oc430 (wave wave220): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc430`.

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
    (
        0x0035_5b30,
        mod_obj_add_multi_spline_modifier,
        "ModObj_AddMultiSplineModifier",
    ),
    (
        0x0016_2458,
        chase_camera_algorithm_dtor,
        "ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0031_1110,
        anim_data_base_free_chan_caches,
        "AnimDataBase_FreeChanCaches",
    ),
    (
        0x0027_46f8,
        in_seq_instance_release,
        "InSeq_InstanceRelease",
    ),
    (0x0026_c898, playback_man_free_data, "PlaybackMan_FreeData"),
    (
        0x0028_2dd0,
        c_transition_man_start_transition,
        "cTransitionMan_StartTransition",
    ),
    (
        0x0015_2de0,
        be_save_int_load_profile_save,
        "BESaveInt_LoadProfileSave",
    ),
    (
        0x0015_5e58,
        be_aggression_int_decay_all,
        "BEAggressionInt_DecayAll",
    ),
    (
        0x0027_8358,
        script_engine_load_main_scripts,
        "ScriptEngine_LoadMainScripts",
    ),
    (
        0x0032_dc10,
        world_init_sphere_tree_cache,
        "World_InitSphereTreeCache",
    ),
    (
        0x0012_b498,
        ai_apply_player_rider_settings,
        "AI_ApplyPlayerRiderSettings",
    ),
    (0x0023_2e20, c_game_load_screen_load, "cGameLoadScreen_Load"),
    (0x0022_e968, game_module_ctor, "GameModule_ctor"),
    (0x0012_5c70, player_ctor, "Player_ctor"),
    (
        0x0011_2d58,
        rider_snap_to_nearest_ai_path,
        "Rider_SnapToNearestAIPath",
    ),
];

// Ported but not registered (no bounded differential fixture; see each doc):
// 0x00325f48 InputPreproc_Run (input_preproc_run), 0x002355c0
// GameStateReset_Enter (game_state_reset_enter), 0x00233048
// cGameLoadScreen_LoadPictures (c_game_load_screen_load_pictures), 0x0022d6c8
// SectionMan_OnEventTrigger (section_man_on_event_trigger), 0x00226900
// SSXApp_Init (ssx_app_init).

/// `void ModObj_AddMultiSplineModifier(cMoveNode* self, void* desc)` (0x355b30):
/// allocates an 0x58-byte `MultiSplineModifier` (tag string at 0x48e9e8, flags
/// 0x20000000), constructs it from `desc` and the node's instance (`self+0x18`),
/// and installs it as the node's transform modifier.
pub fn mod_obj_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x58;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1618));
        call_ptr(c, 0x0031_7d70, 0x0035_5b60)?; // cMemMan_alloc
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_9f88, 0x0035_5b70)?; // MultiSplineModifier_ctor
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_54b0, 0x0035_5b7c)?; // ModObj_SetTransformModifier
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCameraAlgorithm_dtor(ChaseCameraAlgorithm* self, int flags)`
/// (0x162458): resets the vtable to the base-class one while destroying the five
/// `cCSICubicSpline` members at `+0x334 .. +0x304` (stride 0xc, flag 2), sets
/// the final vtable (0x45bde0) and frees `self` (`delete`) when bit 0 of
/// `flags` is set.
pub fn chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4440));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        // (member offset, return address) of the five spline destructor calls
        for (off, ret) in [
            (0x334u32, 0x0016_2488u32),
            (0x328, 0x0016_2494),
            (0x31c, 0x0016_24a0),
            (0x310, 0x0016_24ac),
            (0x304, 0x0016_24b8),
        ] {
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_d618, ret)?; // cCSICubicSpline_dtor
        }
        c.g[V0][0] = 0x46_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4220));
        let keep = c.g[S0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0016_24d4)?; // operator_delete
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimDataBase_FreeChanCaches(void*)` (0x311110): releases the 48
/// channel caches. Entry `i` of the table at 0x4fe7a0 (`cache`) holds a
/// pointer to 20 records of 0x24 bytes; for each record the byte at `+5` of
/// the record's first pointer is set to 0xff and the block at `+4` is freed;
/// then the table block itself is freed and the entry and its twin in the table
/// at 0x4fe860 are cleared.
pub fn anim_data_base_free_chan_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S6, 0x30);
        c.g[V1][0] = 0x50_0000;
        frame.save(c, S7, 0x20);
        c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860));
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[S6][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x17a0));
        frame.save(c, S1, 0x80);
        c.g[A0][0] = 0;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        loop {
            c.cycles += 5;
            c.g[S3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let empty = c.g[V0][0] == 0;
            c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[S4][0] = 0xff;
                c.g[S1][0] = 0;
                c.g[S2][0] = 0x13;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    let none = c.g[V0][0] == 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    if !none {
                        c.cycles += 1;
                        c.sb((c.g[V0][0] as u32).wrapping_add(5), c.g[S4][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
                    let no_block = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if !no_block {
                        c.cycles += 2;
                        call_ptr(c, 0x0031_7e98, 0x0031_11b4)?; // EA_MemFree
                    }
                    c.cycles += 2;
                    if (c.g[S2][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_1180);
                        }
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    let no_table = c.g[A0][0] == 0;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if !no_table {
                        c.cycles += 3;
                        call_ptr(c, 0x0031_7e98, 0x0031_11d4)?; // EA_MemFree
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    }
                    c.cycles += 2;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.sw(c.g[V0][0] as u32, 0);
                    break;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 48) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1158);
                }
                continue;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InSeq_InstanceRelease(InSeqInstance* self)` (0x2746f8): if the
/// instance is live (`+4`), stops it, releases every channel (`+0x18` array of
/// 0x3c-byte `InSeq_Channel`s, count in the byte at `(+0x10)+1`), destroys the
/// channel array backwards and deletes it, then resets the instance to its
/// idle state (`-1` ids, `1.0f` rate, everything else 0).
pub fn in_seq_instance_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let idle = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !idle {
            c.cycles += 6;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0027_4918, 0x0027_4724)?; // InSeq_InstanceStop
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(1));
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            } else {
                c.cycles += 1;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    call_ptr(c, 0x0027_3a00, 0x0027_4748)?; // InSeq_ChannelRelease
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4738);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break;
                }
            }
            c.cycles += 2;
            let no_array = c.g[A1][0] == 0;
            c.g[V0][0] = 0x3c;
            if !no_array {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
                c.mult(0, A0, V1, V0);
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if c.g[A1][0] == c.g[S0][0] {
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_3970, 0x0027_478c)?; // InSeq_Channel_dtor
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        let more = c.g[V0][0] != c.g[S0][0];
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_4780);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x0027_2510, 0x0027_47a4)?; // InSeq_HeapDeleteArray
            }
            c.cycles += 19;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let this = c.g[S1][0] as u32;
            c.sw(this.wrapping_add(0x34), c.g[V0][0]);
            c.swc1(0, this.wrapping_add(0x24));
            c.sw(this.wrapping_add(0x38), 0);
            c.sw(this, c.g[V0][0]);
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28, 0x2c] {
                c.sw(this.wrapping_add(off), 0);
            }
            c.sw(this.wrapping_add(0x30), c.g[V0][0]);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PlaybackMan_FreeData(PlaybackMan* self)` (0x26c898): cancels the
/// pending async read (`+0x78`, `-1` = none), frees the per-entry buffers
/// (`+0x58`, `+0x50` entries of 12 bytes: words at `+4` freed and cleared,
/// `+8` cleared), the entry table, the block at `+0x4c`, and the data block
/// at `+4` (`delete`); clears the sizes and pointers.
pub fn playback_man_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if c.g[A0][0] == c.g[S1][0] {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
        } else {
            c.cycles += 4;
            call_ptr(c, 0x003d_faf0, 0x0026_c8c8)?; // AsyncFile_Cancel
            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[S1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if !none {
                c.cycles += 1;
                c.g[S1][0] = 0;
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    } else {
                        c.cycles += 6;
                        call_ptr(c, 0x0031_7e98, 0x0026_c904)?; // EA_MemFree
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    }
                    c.cycles += 7;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(8), 0);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_c8e8);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            if c.g[A0][0] == 0 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0026_c944)?; // EA_MemFree
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0);
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0026_c95c)?; // EA_MemFree
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        }
        c.cycles += 2;
        let no_data = c.g[A0][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
        if !no_data {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0026_c974)?; // operator_delete
            c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        }
        c.cycles += 7;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cTransitionMan_StartTransition(cTransitionMan* self, int a1, int kind,
/// int a3, float t, float x, float y, float z)` (0x282dd0): ends the running
/// transition (a virtual call through `self+0xc`, slot `0x28`/`0x2c`) unless
/// the state at `+0x18` is 9 (idle). Then either resets to idle (`kind == 9`
/// or `x + y + z <= t`), or records the new transition (`+0x14 = a1`,
/// `+0x18 = kind`, `+0x28 = a3`, the vector scaled by the constant at
/// `gp - 0x4778` into `+0x1c..+0x24`) and starts it (virtual slot `0x20`/`0x24`).
pub fn c_transition_man_start_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 9;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0x78);
        c.g[S4][0] = c.g[A3][0];
        frame.save_fpr(c, 22, 0x70);
        c.mov_s(23, 12);
        frame.save_fpr(c, 21, 0x68);
        c.mov_s(22, 15);
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(21, 14);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        let idle = c.g[V0][0] == c.g[S1][0];
        c.mov_s(20, 13);
        if !idle {
            // end the running transition
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_2e40)?;
        }
        c.cycles += 2;
        let to_idle = c.g[S2][0] == c.g[S1][0];
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let mut start = false;
        if !to_idle {
            c.cycles += 6;
            c.add_s(0, 21, 22);
            c.add_s(0, 0, 20);
            c.c_le_s(0, 23);
            start = !c.fcc();
            c.g[A1][0] = c.g[S3][0];
        }
        if start {
            c.cycles += 16;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4778));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
            c.mul_s(2, 20, 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S4][0]);
            c.mul_s(1, 21, 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
            c.mul_s(0, 22, 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x24));
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_2ebc)?;
        } else {
            c.cycles += 7;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x78);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `char* InputPreproc_Run(InputPreproc* self, const char* path, int* size)`
/// (0x325f48, `cInputPreProcessor::loadFileSize`): resets the preprocessor
/// (size 0, capacity 0x800, a 0x800-byte output buffer), splits `path` at its
/// last `/` into the directory (kept at `sp+0`, empty when there is no `/`)
/// and the file name (`sp+0x80`), runs `InputPreproc_ProcessFile` over it, copies
/// the output to a fresh buffer of exactly its length, frees the work buffer,
/// stores the length in `*size` and returns the copy.
///
/// Resume entries: when registering, add the `c.pc != entry` fallback to the
/// generated twin (`__entries`, `__generated` in the unwind form); the generated
/// body has resume points, this port does not handle them.
///
/// NOT registered: `InputPreproc_ProcessFile` (not in the override list) runs the
/// whole file reader, so no bounded differential fixture exists.
pub fn input_preproc_run(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x800;
        frame.save(c, S3, 0xd0);
        c.g[A3][0] = 0;
        frame.save(c, S4, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0xe0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0xb0);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x100);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0xf0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1c48));
        frame.save_ra(c, 0xa0);
        c.g[A0][0] = 0x800;
        c.sw(c.g[S3][0] as u32, 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S3][0] as u32).wrapping_add(4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_5f98)?; // operator_new_named
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x2f;
        call_ptr(c, 0x0041_acc0, 0x0032_5fa8)?; // strrchr
        c.g[S2][0] = c.g[V0][0];
        let no_slash = c.g[S2][0] == 0;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        if !no_slash {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S4][0] as u32));
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6ce0, 0x0032_5fc8)?; // strncpy (directory)
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sb(c.g[V1][0] as u32, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5fe0)?; // strcpy (file name)
            c.g[A2][0] = c.g[S1][0];
        } else {
            c.cycles += 6;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.sb(c.g[SP][0] as u32, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5ffc)?; // strcpy (file name)
            c.g[A2][0] = c.g[S0][0];
        }
        c.cycles += 16;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0032_6078, 0x0032_600c)?; // InputPreproc_ProcessFile
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0032_6020)?; // operator_new_named
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_605c, 0x0032_6034)?; // memcpy
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0032_6048)?; // EA_MemFree
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x100);
        c.sw(c.g[S5][0] as u32, c.g[V1][0]);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// One `BE*Int_GetInstance()->onLoad()` of `BESaveInt_LoadProfileSave`: the
/// object returned by the call at `getter` has a vtable at `+0xc` whose
/// entry at `+0x10`/`+0x14` (`{delta, fn}`) is called with `this + delta`.
fn be_save_int_on_load(c: &mut Cpu, getter: u32, ret_getter: u32, ret_call: u32) -> Flow {
    call_ptr(c, getter, ret_getter)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let target = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_call)
}

/// `int BESaveInt_LoadProfileSave(BESaveInt* self, char* buf, int idx)`
/// (0x152de0): loads profile `idx` from a save buffer. The CRC16 (seed 0xfbea)
/// of the first `GetProfileSaveSize() - 4` bytes must equal the word stored
/// after them, else returns 0. Otherwise: remembers the ten per-sub-profile
/// `keep` words of the profile slot (stride 0xf88 from `0x4a6f30 + idx *
/// 0x9b50`), copies the 12-byte name (`strncpy` into `0x535b20 + idx * 0x1c`)
/// and the 0x9b40-byte profile block (an aligned `ld/sd` copy or an unaligned
/// `ldl/ldr` one, by the `(src | dst) & 7` test) from the buffer into
/// `0x4a6ca8 + idx * 0x9b50`, stores the flag byte at `0x535b31 + idx * 0x1c`,
/// calls the three interfaces' `onLoad`, and restores each sub-profile's `keep`
/// word and its equip flags (`BE_RestoreCharProfileEquipFlags`). Returns 1.
pub fn be_save_int_load_profile_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0015_2ba8, 0x0015_2e04)?; // BESaveInt_GetProfileSaveSize
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0xfbea;
        call_ptr(c, 0x003e_62d0, 0x0015_2e18)?; // crc16
        let sp0 = frame.at(c, 0x30);
        c.sw(sp0, 0);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.lwl(3, (c.g[S0][0] as u32).wrapping_add(3));
        c.lwr(3, c.g[S0][0] as u32);
        c.swl(3, sp0.wrapping_add(3));
        c.swr(3, sp0);
        c.g[V1][0] = c.lw(sp0);
        let mismatch = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.g[S0][0] = 0x4a_0000;
            c.mult(0, V1, S2, V1);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x288));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 9;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            // keep[i] = profile[idx].sub[i].keep, i = 9..0 (into sp[0..9])
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf88));
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                let more = (c.g[A1][0] as i64) >= 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_2e60);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 17;
            c.g[V0][0] = 0x1c;
            c.g[A0][0] = 0x53_0000;
            c.mult(0, V0, S2, V0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b20));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xc;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0041_6ce0, 0x0015_2ea0)?; // strncpy (name)
            c.g[V1][0] = 0x9b50;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6ca8));
            c.mult(0, V1, S2, V1);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = c.g[S1][0] | c.g[A0][0];
            c.g[V0][0] &= 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !aligned {
                c.cycles += 2;
                c.g[V0][0] = 0x9b40;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                loop {
                    c.cycles += 19;
                    let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
                    c.ldl(7, a1.wrapping_add(7));
                    c.ldr(7, a1);
                    c.ldl(8, a1.wrapping_add(0xf));
                    c.ldr(8, a1.wrapping_add(8));
                    c.ldl(3, a1.wrapping_add(0x17));
                    c.ldr(3, a1.wrapping_add(0x10));
                    c.ldl(6, a1.wrapping_add(0x1f));
                    c.ldr(6, a1.wrapping_add(0x18));
                    c.sdl(7, a0.wrapping_add(7));
                    c.sdr(7, a0);
                    c.sdl(8, a0.wrapping_add(0xf));
                    c.sdr(8, a0.wrapping_add(8));
                    c.sdl(3, a0.wrapping_add(0x17));
                    c.sdr(3, a0.wrapping_add(0x10));
                    c.sdl(6, a0.wrapping_add(0x1f));
                    c.sdr(6, a0.wrapping_add(0x18));
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_2ec8);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x1c;
                    break;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x9b40;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                loop {
                    c.cycles += 11;
                    let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
                    c.g[A3][0] = c.ld(a1);
                    c.g[T0][0] = c.ld(a1.wrapping_add(8));
                    c.g[V1][0] = c.ld(a1.wrapping_add(0x10));
                    c.g[A2][0] = c.ld(a1.wrapping_add(0x18));
                    c.sd(a0, c.g[A3][0]);
                    c.sd(a0.wrapping_add(8), c.g[T0][0]);
                    c.sd(a0.wrapping_add(0x10), c.g[V1][0]);
                    c.sd(a0.wrapping_add(0x18), c.g[A2][0]);
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_2f24);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0x1c;
                    break;
                }
            }
            // the last 16 bytes, then the flag byte
            c.cycles += 45;
            let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
            c.ldl(7, a1.wrapping_add(7));
            c.ldr(7, a1);
            c.ldl(8, a1.wrapping_add(0xf));
            c.ldr(8, a1.wrapping_add(8));
            c.sdl(7, a0.wrapping_add(7));
            c.sdr(7, a0);
            c.sdl(8, a0.wrapping_add(0xf));
            c.mult(0, V1, S2, V1);
            c.sdr(8, a0.wrapping_add(8));
            c.g[S0][0] = 0x9b50;
            c.g[V0][0] = 0x53_0000;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b31));
            c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
            c.g[S3][0] = 9;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            be_save_int_on_load(c, 0x0014_5970, 0x0015_2f9c, 0x0015_2fb0)?; // BENewPlayerInt
            be_save_int_on_load(c, 0x0014_7f78, 0x0015_2fb8, 0x0015_2fcc)?; // BECharInt
            be_save_int_on_load(c, 0x0014_f900, 0x0015_2fd4, 0x0015_2fe8)?; // BEBAGTInt
            c.mult(0, S0, S2, S0);
            c.g[V0][0] = 0x4a_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[S2][0] = c.g[SP][0];
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x288));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            loop {
                c.cycles += 9;
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf88));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0015_15b8, 0x0015_3024)?; // BE_RestoreCharProfileEquipFlags
                let more = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf88));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_3008);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The aggression state machine of `BEAggressionInt_DecayAll`: given the
/// state `st` (v[0]), the level `lvl` (points / 5) and the points `pts` of one
/// aggression slot, returns `(cycles of the compiled compare chain, st, lvl,
/// pts)`. State 0 caps the level at 2 (10 points), 1 sends level >= 4 to state 3
/// (level 2, 10 points), 2 raises level 0 to 1 and caps level >= 4 at 3 (15
/// points), 3 turns level 0 into state 1 (level 3, 15 points) and caps level >= 5
/// at 4 (20 points); other states stay as they are. The cycle counts are the
/// generated block charges of the compiled if-chain.
pub(crate) fn be_aggression_step(st: i64, lvl: i64, pts: u64) -> (u64, i64, i64, u64) {
    match st {
        1 if lvl >= 4 => (11, 3, 2, 10),
        1 => (7, st, lvl, pts),
        0 if lvl < 3 => (10, st, lvl, pts),
        0 => (15, st, 2, 10),
        2 if lvl <= 0 => (13, st, 1, pts),
        2 if lvl < 4 => (12, st, lvl, pts),
        2 => (17, st, 3, 15),
        3 if lvl == 0 => (17, 1, 3, 15),
        3 if lvl < 5 => (14, st, lvl, pts),
        3 => (17, st, 4, 20),
        _ if st < 0 => (9, st, lvl, pts),
        _ => (11, st, lvl, pts),
    }
}

/// `void BEAggressionInt_DecayAll(BEAggressionInt* self)` (0x155e58): for every
/// player of the race (count at `0x5305e0 + 0x30`) and each of the ten
/// aggression slots of the rider's character profile (3 bytes per slot at
/// `0x4a7869 + profile * 0x9b50 + char * 0xf88`: state, level, points), reduces
/// the points by 3 (not below 0), tells the HUD (`func_001E2A08(slot, points)`,
/// unless the race's byte at `+0x49` is set), then applies
/// [`be_aggression_step`] and writes state, points and level back.
pub fn be_aggression_int_decay_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = 0x53_0000;
        frame.save(c, S4, 0x60);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.g[S4][0];
            'players: loop {
                c.cycles += 8;
                c.g[S6][0] = 0;
                call_ptr(c, 0x0014_a0e0, 0x0015_5ea8)?; // BE_GetRiderProfile
                c.g[FP][0] = 0;
                c.g[A0][0] = 0x9b50;
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.mult(0, S5, V0, A0);
                let slot = frame.at(c, 0);
                c.sw(slot, c.g[V1][0]);
                c.g[S7][0] = 0;
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0014_a080, 0x0015_5ec8)?; // BE_GetRiderCharacter
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = 0xf88;
                    c.mult(0, V1, S3, V1);
                    c.g[A0][0] = 5;
                    c.cycles += 22;
                    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[A0][0] = 0x4a_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x786b));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.g[V1][0] = 0x53_0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                    c.g[A2][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = 0x4a_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7869));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lb(c.g[V1][0] as u32);
                    c.g[V1][0] = 5;
                    c.g[S2][0] = c.lb(c.g[V0][0] as u32);
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_sub(3));
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[A1][0] == 0 {
                        c.g[S1][0] = 0;
                    }
                    c.div(0, S1, V1);
                    c.g[V0][0] = c.lo[0];
                    let hud_off = c.g[A2][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if !hud_off {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x001e_2a08, 0x0015_5f44)?; // HUD aggression update
                    }
                    let (cyc, st, lvl, pts) =
                        be_aggression_step(c.g[S2][0] as i64, c.g[S0][0] as i64, c.g[S1][0]);
                    c.cycles += cyc;
                    c.g[S2][0] = st as u64;
                    c.g[S0][0] = lvl as u64;
                    c.g[S1][0] = pts;
                    c.g[V1][0] = 0xf88;
                    c.g[A0][0] = 0x4a_0000;
                    c.cycles += 19;
                    c.mult(0, V1, S3, V1);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7869));
                    c.g[A2][0] = 0x4a_0000;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x786b));
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(3));
                    c.g[A1][0] = ((c.g[S6][0] as i64) < 10) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(3));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sb(c.g[V1][0] as u32, c.g[S2][0]);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = 0x4a_0000;
                    c.sb(c.g[A0][0] as u32, c.g[S1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x786a));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let more_slots = c.g[A1][0] != 0;
                    c.sb(c.g[V0][0] as u32, c.g[S0][0]);
                    if more_slots {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_5ec0);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 7;
                c.g[A0][0] = 0x53_0000;
                let slot = frame.at(c, 0);
                c.g[S4][0] = c.lw(slot);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b0));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more_players = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S4][0];
                if more_players {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_5ea0);
                    }
                    continue 'players;
                }
                break;
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

/// `void ScriptEngine_LoadMainScripts(ScriptEngine* self)` (0x278358): sets up
/// the script engine's pools. Allocates the 200-byte link pool (`+0x544`: ten
/// 20-byte links, four `-1` ids and a `next` pointer to the following link),
/// keeps the first link as the free link (`+0x548`), loads script bank 0 synchronously
/// (`ScriptEngine_LoadBankSync`), registers the four script objects (`+0x6d0`,
/// two of 0x120 bytes, `+0x91c`, `+0x938`), allocates the two-element
/// `ScriptList` arrays of 0x1a8 bytes (stored at `+0x54c` and `+0x624`, each
/// element pointing back to `self`; the second one with its index).
pub fn script_engine_load_main_scripts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S1, 0x70);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bd8));
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = 0;
        c.g[A0][0] = 0xc8;
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0027_839c)?; // operator_new_named
        c.sw((c.g[S1][0] as u32).wrapping_add(0x544), c.g[V0][0]);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A3][0] = 0x14;
        c.g[A0][0] = 0;
        c.g[A2][0] = 9;
        // the ten links
        loop {
            c.cycles += 20;
            let pool = (c.g[S1][0] as u32).wrapping_add(0x544);
            c.g[V1][0] = c.lw(pool);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw(pool);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
            c.g[V1][0] = c.lw(pool);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.g[V0][0] = c.lw(pool);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
            c.g[V1][0] = c.lw(pool);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_83b0);
                }
                continue;
            }
            break;
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x544));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x3ac));
        c.g[A1][0] = 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 0;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x544));
        c.g[T2][0] = 0;
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x91c));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x938));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x548), c.g[V0][0]);
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x54c));
        call_ptr(c, 0x0027_b268, 0x0027_843c)?; // ScriptEngine_LoadBankSync
        c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x624));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
        c.g[S2][0] = 1;
        // the two rig script objects, then the two single ones
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0028_3298, 0x0027_8454)?; // ScriptObj_Register
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8448);
                }
                continue;
            }
            break;
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S3][0];
        c.g[S2][0] = 1;
        call_ptr(c, 0x0028_3298, 0x0027_8468)?; // ScriptObj_Register
        c.g[S5][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0028_3298, 0x0027_8474)?; // ScriptObj_Register
                                                // objsA: two ScriptLists
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0x1a8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1be8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0027_848c)?; // operator_new_named
        c.g[V1][0] = 2;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[S0][0] = c.g[S3][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0027_5d10, 0x0027_84ac)?; // ScriptList_ctor
            let more = c.g[S2][0] != c.g[S5][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_84a0);
                }
                continue;
            }
            break;
        }
        c.cycles += 3;
        c.sw(c.g[S6][0] as u32, c.g[S3][0]);
        c.g[V1][0] = 0;
        c.g[S0][0] = 1;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54c));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xcc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_84c0);
                }
                continue;
            }
            break;
        }
        // objsB: two ScriptLists
        c.cycles += 13;
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0x1a8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bf8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[S0][0] = 2;
        call_ptr(c, 0x0031_7e30, 0x0027_84f8)?; // operator_new_named
        c.g[S2][0] = 1;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[S0][0] = c.g[S3][0];
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0027_5d10, 0x0027_851c)?; // ScriptList_ctor
            let more = c.g[S2][0] != c.g[S4][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8510);
                }
                continue;
            }
            break;
        }
        c.cycles += 3;
        c.sw(c.g[S7][0] as u32, c.g[S3][0]);
        c.g[S0][0] = 0;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 13;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x624));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x624));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x624));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(8), c.g[S0][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xcc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8530);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
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

/// `void GameStateReset_Enter(GameStateReset* self)` (0x2355c0): enters the
/// "reset" game state. Records whether the script engine is in a transport
/// sequence (`+0x1c`), clears the user awards, reads the race setup
/// (`0x535bc8`: byte `+0x49` game path, byte `+0x48` game mode), picks the
/// state (`+0x0`, `+0x4`: path set: 1/0; no path and the section id (`+0x1bc`
/// of the section manager) in 0xe..0x10: 1/1; otherwise 4/0), switches the AI
/// to state 3 in mode 4, resets the BE state, re-initialises the missions and
/// runs the global scripts, enters the hub when the location type is 1, hides
/// the load indicator and starts the NIS lists (path set) or the idle list
/// (`+0x14`), then re-enables the audio engine.
///
/// NOT registered: `0x158e30` (reward bookkeeping) and `0x309f18` (global
/// script reset) are real, unslotted callees with their own loops, so no bounded
/// differential fixture exists.
pub fn game_state_reset_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        call_ptr(c, 0x0027_aa80, 0x0023_55f0)?; // IsInTransportSequence
        let not_transport = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !not_transport {
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        }
        c.cycles += 17;
        call_ptr(c, 0x0014_dc80, 0x0023_5604)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5610)?; // cBE_getInterface(0)
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_5618)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0023_5624)?; // cBE_getInterface(13)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_8e30, 0x0023_562c)?; // clear user awards
        c.g[V0][0] = 0x53_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
        let has_path = c.g[S3][0] != 0;
        c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
        let mut in_range = false;
        if !has_path {
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        if has_path || in_range {
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 1) as u64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V1][0]);
            c.g[V0][0] = 4;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 4;
            c.sw((c.g[S1][0] as u32).wrapping_add(4), 0);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 2;
        if c.g[S4][0] != c.g[V0][0] {
            c.g[A0][0] = 0;
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 3;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0011_3b10, 0x0023_569c)?; // cAI_setAIState
            c.g[A0][0] = 0;
        }
        c.cycles += 13;
        c.g[S5][0] = 1;
        call_ptr(c, 0x0014_de68, 0x0023_56a8)?; // cBE_setState(0)
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030_9030, 0x0023_56b0)?; // WScriptMan_initMissions
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030_9f18, 0x0023_56b8)?; // WScriptMan_runGlobalScripts
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4be0, 0x0023_56c0)?; // getLocationID
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4cc0, 0x0023_56cc)?; // getLocationType
        let not_hub = c.g[V0][0] != c.g[S5][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        if !not_hub {
            c.cycles += 18;
            call_ptr(c, 0x0014_dc80, 0x0023_56dc)?; // cBE_getBE
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0023_56e8)?; // cBE_getInterface(1)
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_4be0, 0x0023_56f4)?; // getLocationID
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e10, 0x0023_5704)?; // setLastHub
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_5718)?; // player->onHub()
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        }
        c.cycles += 11;
        c.g[V1][0] = c.g[S4][0] ^ 4;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V1][0] = (c.g[V1][0] != 0) as u64;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
        call_ptr(c, 0x001f_36d8, 0x0023_5740)?; // cBXLoadIndicator_Hide
        if !has_path {
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.cycles += 2;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0027_8f68, 0x0023_57b0)?; // PlayNISList(1, 1, 0)
            }
        } else {
            c.cycles += 7;
            c.sw(c.g[S1][0] as u32, c.g[S5][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(4), 0);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
            c.g[V0][0] ^= 2;
            let not_mode2 = c.g[V0][0] != 0;
            c.g[A1][0] = 1;
            if !not_mode2 {
                c.cycles += 8;
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                call_ptr(c, 0x0027_8f68, 0x0023_5770)?; // PlayNISList(1, 0, 1)
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                c.g[V0][0] ^= 2;
                let not_mode2 = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !not_mode2 {
                    c.cycles += 5;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 1;
                    call_ptr(c, 0x0027_8f68, 0x0023_5790)?; // PlayNISList(0, 0, 1)
                }
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028_b180, 0x0023_57b8)?; // cAudioInterface_bxAudio
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b_3a98, 0x0023_57c0)?; // audio func
        call_ptr(c, 0x0028_b180, 0x0023_57c8)?; // cAudioInterface_bxAudio
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a_4b68, 0x0023_57d0)?; // audio func
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

/// `void cGameLoadScreen_LoadPictures(cGameLoadScreen* self, int)` (0x233048):
/// loads the load screen's textures from a BigFile. Rider race (the byte at
/// `0x535c11` is 2): the two characters' pictures (`sprintf` of the character
/// ids of both players into the name, `FILE_loadpackz`, then a
/// texture load through the graphics manager's vtable `+0x170` into
/// `self + 0x10 + 4 * i`). Otherwise: the course picture of the location id
/// (0 when above 16) into `self + 0x10`. The BigFile is closed with flag 2.
///
/// NOT registered: `cBigFile` open and `FILE_loadpackz` are real, unslotted
/// callees that read the disc, so no bounded differential fixture exists.
pub fn c_game_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S2, 0xe0);
        frame.save(c, S0, 0x100);
        frame.save(c, S3, 0xd0);
        frame.save(c, S1, 0xf0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3078)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_3084)?; // cBE_getInterface(1): players
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_308c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3098)?; // cBE_getInterface(0): race
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 2;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
        let course = c.g[A0][0] != c.g[V1][0];
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        if !course {
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = 0x48_0000;
            c.g[S7][0] = c.g[A0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
            c.g[A2][0] = 0x100;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_68b0, 0x0023_30cc)?; // cBigFile open
            c.g[S1][0] = 0;
            c.g[V0][0] = 0x44_0000;
            c.g[S5][0] = 0x48_0000;
            c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
            c.g[S4][0] = 0x48_0000;
            loop {
                c.cycles += 29;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = (0 < c.g[S1][0]) as u64;
                call_ptr(c, 0x0014_7488, 0x0023_30ec)?; // getPlayerCharID
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                call_ptr(c, 0x0041_7828, 0x0023_3108)?; // sprintf
                c.g[A1][0] = 0x300_0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] |= 0x100;
                call_ptr(c, 0x003e_2190, 0x0023_3118)?; // FILE_loadpackz
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A3][0] = 0;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[T0][0] = 1;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_314c)?; // loadTex
                let no_data = c.g[S0][0] == 0;
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                if !no_data {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_7e98, 0x0023_315c)?; // EA_MemFree
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_30e0);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_68e8, 0x0023_3178)?; // cBigFile close
            frame.restore(c, S0, 0x100);
        } else {
            c.cycles += 40;
            c.g[A1][0] = 0x48_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_68b0, 0x0023_3194)?; // cBigFile open
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_4be0, 0x0023_319c)?; // getLocationID
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[A2][0] = 0x44_0000;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
            c.g[V1][0] = ((c.g[V0][0] as i64) < 17) as u64;
            c.g[A1][0] = 0x4a_0000;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0;
            }
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            call_ptr(c, 0x0041_7828, 0x0023_31d0)?; // sprintf
            c.g[A1][0] = 0x300_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] |= 0x100;
            call_ptr(c, 0x003e_2190, 0x0023_31e0)?; // FILE_loadpackz
            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = 0x48_0000;
            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
            c.g[T1][0] = 0xffff_ffff_ffff_ffff;
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_3218)?; // loadTex
            let no_data = c.g[S0][0] == 0;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if !no_data {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0023_3228)?; // EA_MemFree
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_68e8, 0x0023_3234)?; // cBigFile close
            frame.restore(c, S0, 0x100);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void cGameLoadScreen_Load(cGameLoadScreen* self)` (0x232e20): builds the
/// load screen UI ("data/ui/gl"). Asks the graphics manager (vtable `+0x160`,
/// argument 0x100) to prepare, allocates the 0x70-byte UI engine (`+0xc`) and
/// loads the file into it, hands it the world's fonts and locale (`+0x88`,
/// `+0x8c`), gives it a fresh 0x1c-byte input node, and pushes the first load
/// state: the lodge state (0x48 bytes) when the lodge flag (`gp - 0x1718`) is
/// set, or the race has no path and the mode is 5 or 6; the conquer state
/// (0x48 bytes, clears `gp - 0x1714`) when there is no path and the mode is
/// not 5/6; otherwise the 0x4c-byte picture state, which loads the pictures
/// (`cGameLoadScreen_LoadPictures`).
pub fn c_game_load_screen_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        frame.save(c, S2, 0x10);
        c.g[A1][0] = 0x100;
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_2e54)?; // graphics manager vtable call
        call_ptr(c, 0x0014_dc80, 0x0023_2e5c)?; // cBE_getBE
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_2e68)?; // cBE_getInterface(0)
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4968));
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0023_2e80)?; // cMemMan_alloc (UIEngine)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7b08, 0x0023_2e88)?; // UIEngine ctor
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42d8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x0039_7c40, 0x0023_2e9c)?; // UIEngine_loadFile("data/ui/gl")
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x48_0000;
        c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = 0x1c;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x88));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4610));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.sw((c.g[T1][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        call_ptr(c, 0x0031_7d70, 0x0023_2ed0)?; // cMemMan_alloc (input node)
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = 0x47_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e30));
        c.sb((c.g[V0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(8), c.g[V1][0]);
        let node = c.g[V0][0] as u32;
        for off in [0x18u32, 0xc, 0x10, 0x14] {
            c.sw(node.wrapping_add(off), 0);
        }
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
        let lodge_flag = c.g[V1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        // 0: lodge state, 1: conquer state, 2: picture state
        let state;
        if lodge_flag {
            state = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = 0x53_0000;
            c.g[A0][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                state = 3;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = 5;
                let mode5 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 6;
                if mode5 {
                    state = 0;
                } else {
                    c.cycles += 2;
                    let not6 = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                    state = if not6 { 3 } else { 0 };
                }
            }
        }
        let mut state = state;
        if state == 3 {
            // other: conquer state unless a path is set or the mode is 5/6
            c.cycles += 3;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let path = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if !path {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.g[V1][0] ^ 5;
                let mode5 = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] ^ 6;
                if !mode5 {
                    c.cycles += 1;
                    c.g[A0][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            c.g[A1][0] = 0x48_0000;
            state = if c.g[A0][0] != 0 { 1 } else { 2 };
        }
        match state {
            0 => {
                c.cycles += 16;
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42c8));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2f48)?; // cMemMan_alloc (lodge state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_2f5c)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3788));
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
            }
            1 => {
                c.cycles += 15;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42b0));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2fac)?; // cMemMan_alloc (conquer state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_2fc0)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1714), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3858));
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
            }
            _ => {
                c.cycles += 19;
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4298));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2fec)?; // cMemMan_alloc (picture state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_3000)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x36b8));
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
                c.g[A1][0] = 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3048, 0x0023_3020)?; // cGameLoadScreen_LoadPictures
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0023_3030)?; // UIStateStack_push
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Section event 2..6 of `SectionMan_OnEventTrigger`: the (game mode, game
/// event) pair the race setup is switched to, and the generated block's cycle
/// charge. Other events change nothing.
pub(crate) fn section_man_event_modes(event: u32) -> Option<(u32, u32, u64)> {
    match event {
        2 => Some((0, 0, 8)),
        3 => Some((1, 1, 8)),
        4 => Some((2, 3, 8)),
        5 => Some((3, 2, 8)),
        6 => Some((5, 4, 6)),
        _ => None,
    }
}

/// `void SectionMan_OnEventTrigger(SectionMan* self, int event)` (0x22d6c8):
/// when an event section is triggered in a race (no game path, game mode 4,
/// the location not in 14..16 and `event != 1`), resets the BE state; if the
/// game is in state 4 it resets the AI rider totals, clears the race manager's
/// `+0xc` word, and switches the race setup to the (mode, event) of the section
/// event (location 14..16: mode 4, event 12), runs the race manager's virtual
/// slot at `+0xc`, notifies the HUD with the game event and starts the section
/// transition of the UI at `+0x1b0`.
///
/// Resume entries: when registering, add the `c.pc != entry` fallback to the
/// generated twin (`__entries`, `__generated` in the unwind form); the generated
/// body has resume points, this port does not handle them.
///
/// NOT registered: the final steps run `0x238160` (the game mode switch:
/// its own jump tables and a virtual call), which has no bounded fixture.
pub fn section_man_on_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0022_d6ec)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_d6f8)?; // cBE_getInterface(0): race
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0x53_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
        let has_path = c.g[V1][0] != 0;
        c.g[A0][0] = 0;
        // skip: a game path is set, the mode is not 4, the section is the
        // finale (14..16) or the event is 1
        if has_path {
            c.cycles += 1;
            c.g[A0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] ^= 4;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 1;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    if c.g[S1][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[A0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0;
            call_ptr(c, 0x0014_de68, 0x0022_d758)?; // cBE_setState(0)
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = 4;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
            if c.g[V1][0] == c.g[A0][0] {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b180, 0x0022_d778)?; // cAI_resetRidersStatTotals
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                let finale = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if finale {
                    c.cycles += 7;
                    c.g[A1][0] = 4;
                    call_ptr(c, 0x0014_4df0, 0x0022_d7ac)?; // setGameMode(4)
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x0014_51e8, 0x0022_d7b8)?; // setGameEvent(12)
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 7) as u64;
                    let in_table = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    let mut modes = None;
                    if in_table {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4bb0));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        modes = section_man_event_modes(c.g[S1][0] as u32);
                    }
                    match modes {
                        Some((mode, event, cyc)) => {
                            let (rets, last) = match mode {
                                0 => ((0x0022_d7f0, 0x0022_d7fc), false),
                                1 => ((0x0022_d810, 0x0022_d81c), false),
                                2 => ((0x0022_d830, 0x0022_d83c), false),
                                3 => ((0x0022_d850, 0x0022_d85c), false),
                                _ => ((0x0022_d870, 0x0022_d87c), true),
                            };
                            c.cycles += cyc;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = mode as u64;
                            call_ptr(c, 0x0014_4df0, rets.0)?; // setGameMode
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = event as u64;
                            call_ptr(c, 0x0014_51e8, rets.1)?; // setGameEvent
                            if last {
                                c.cycles += 1;
                            }
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        }
                        None => {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        }
                    }
                }
                c.cycles += 15;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0022_d890)?; // race object's virtual slot
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
                call_ptr(c, 0x0023_8160, 0x0022_d8a4)?; // set game mode
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0023_1250, 0x0022_d8b8)?; // section transition
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One of the allocation calls of `World_InitSphereTreeCache`:
/// `operator_new_named(size, tag, 0, 0)` returning to `ret`.
fn world_sphere_tree_new(c: &mut Cpu, size: u64, tag: u64, ret: u32) -> Flow {
    c.g[A0][0] = size;
    c.g[A1][0] = tag;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, ret)
}

/// `void World_InitSphereTreeCache(int nSmall, int smallDepth, int nBig, int
/// bigDepth)` (0x32dc10): sets up the sphere-tree buffer cache. A tree of depth
/// `d` has `1 + 8 + ... + 8^d` nodes; the cache gets `nSmall` buffers of the
/// `smallDepth` tree size and `nBig` of the `bigDepth` size (`bigDepth` is also
/// kept at `gp + 0xef8`), each with an id slot set to -1, in the pointer/id
/// arrays at `gp + 0xed8/0xedc` (small) and `gp + 0xee8/0xeec` (big). Counts
/// go to `gp + 0xee0/0xef0`, the cursors (`+0xee4`, `+0xef4`) to 0.
pub fn world_init_sphere_tree_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.sw(gp.wrapping_add(0xef8), c.g[A3][0]);
        frame.save(c, S3, 0x60);
        c.g[V0][0] = 1;
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, FP, 0x10);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = 0;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        let negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        // smallSize = 1 + 8 + ... (smallDepth + 1 terms), in s5
        if !negative {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                let more = c.g[S2][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dc60);
                    }
                    continue;
                }
                break;
            }
        }
        // bigSize, in fp
        c.cycles += 3;
        c.g[V1][0] = c.lw(gp.wrapping_add(0xef8));
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !negative {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                let more = c.g[S2][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dc90);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 5;
        for off in [0xed8u32, 0xedc, 0xee8] {
            c.sw(gp.wrapping_add(off), 0);
        }
        let no_small = c.g[S4][0] == 0;
        c.sw(gp.wrapping_add(0xeec), 0);
        if !no_small {
            c.cycles += 15;
            c.g[S0][0] = 0x49_0000;
            c.g[S1][0] = sx((c.g[S4][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1ac0));
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dce0)?; // pointers
            c.sw(gp.wrapping_add(0xed8), c.g[V0][0]);
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dcf8)?; // ids
            c.sw(gp.wrapping_add(0xedc), c.g[V0][0]);
        }
        c.cycles += 2;
        let no_big = c.g[S3][0] == 0;
        c.g[S0][0] = 0x49_0000;
        if !no_big {
            c.cycles += 14;
            c.g[S1][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1ac0));
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dd20)?; // pointers
            c.sw(gp.wrapping_add(0xee8), c.g[V0][0]);
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dd38)?; // ids
            c.sw(gp.wrapping_add(0xeec), c.g[V0][0]);
        }
        c.cycles += 2;
        let none = (c.g[S4][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S7][0] = 0x49_0000;
            c.g[S6][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 15;
                c.g[S0][0] = c.lw(gp.wrapping_add(0xed8));
                c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let size = c.g[S5][0];
                let tag = sx((c.g[S7][0] as u32).wrapping_sub(0x1ac0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                world_sphere_tree_new(c, size, tag, 0x0032_dd74)?; // small buffer
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                c.g[V0][0] = c.lw(gp.wrapping_add(0xedc));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let more = c.g[V1][0] != 0;
                c.sw(c.g[S1][0] as u32, c.g[S6][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dd50);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 2;
        let none = (c.g[S3][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S6][0] = 0x49_0000;
            c.g[S5][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 15;
                c.g[S0][0] = c.lw(gp.wrapping_add(0xee8));
                c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let size = c.g[FP][0];
                let tag = sx((c.g[S6][0] as u32).wrapping_sub(0x1ac0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                world_sphere_tree_new(c, size, tag, 0x0032_ddc4)?; // big buffer
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.g[V0][0] = c.lw(gp.wrapping_add(0xeec));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let more = c.g[V1][0] != 0;
                c.sw(c.g[S1][0] as u32, c.g[S5][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dda0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.sw(gp.wrapping_add(0xee0), c.g[S4][0]);
        c.sw(gp.wrapping_add(0xef0), c.g[S3][0]);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        c.sw(gp.wrapping_add(0xee4), 0);
        c.sw(gp.wrapping_add(0xef4), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_ApplyPlayerRiderSettings(AI* self, AIPlayer* obj, int i)`
/// (0x12b498): applies the race options to the player rider (`obj + 0x18`).
/// The option word at `0x5308b8 + 0x18` and the player's entry `i` (12 bytes at
/// `0x5308b8`: flags `+0`, initial score `+4`) pick the boost mode (`+0x304`:
/// option bit 3: 3, bit 0: 0, entry bit 5: 2, else 1; `Rider_ResetBoostMeter`),
/// a full boost meter (entry bit 3: 1.0f at `+0xb24`), the boost level (option
/// bit 4: 2 at `+0xb28`), uber permission (entry bit 4: off, option bit 1:
/// on), entry bit 0 (2 at `+0xb20`), `+0xb30` from `func_0014F7A8(i)`, the
/// ranked-network value at `+0xb34` (8; else 1 by entry bit 1), entry bit 2
/// (0.5f at the rider metrix `+0x1c4`), and the rider metrix's base score.
pub fn ai_apply_player_rider_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x53_0000;
        frame.save(c, S0, 0x40);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] &= 1;
        let bit3 = c.g[V0][0] != 0;
        c.g[A1][0] = 1;
        // boost mode (a1)
        if bit3 {
            c.cycles += 2;
            c.g[A1][0] = 3;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            let bit0 = c.g[V0][0] != 0;
            c.g[V0][0] = 0xc;
            if bit0 {
                c.cycles += 2;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 2;
                c.mult(0, V1, S2, V0);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 5);
                c.g[V1][0] &= 1;
                if c.g[V1][0] != 0 {
                    c.g[A1][0] = c.g[A0][0];
                }
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x304), c.g[A1][0]);
        call_ptr(c, 0x0012_0d58, 0x0012_b51c)?; // Rider_ResetBoostMeter
        c.g[V0][0] = 0xc;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8b8));
        c.mult(0, V0, S2, V0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
        c.g[V1][0] &= 1;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb24));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        }
        // option bit 4: boost level 2
        c.cycles += 4;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[V0][0] &= 1;
        let bit4 = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if bit4 {
            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xb28), c.g[V0][0]);
        }
        // uber permission: entry bit 4 off, else option bit 1 on
        c.cycles += 5;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[A1][0] = 0;
            call_ptr(c, 0x0012_5958, 0x0012_b58c)?; // Rider_SetBoostEnabled(rider, 0)
            c.g[V1][0] = 0xc;
        } else {
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] &= 1;
            let bit1 = c.g[V0][0] != 0;
            c.g[V1][0] = 0xc;
            if bit1 {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = 1;
                call_ptr(c, 0x0012_5958, 0x0012_b5b0)?; // Rider_SetBoostEnabled(rider, 1)
                c.g[V1][0] = 0xc;
            }
        }
        // entry bit 0: +0xb20 = 2
        c.cycles += 7;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8b8));
        c.mult(0, V1, S2, V1);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] &= 1;
        let bit0 = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if bit0 {
            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xb20), c.g[V0][0]);
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_f7a8, 0x0012_b5e0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 7;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xb30), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014_dd58, 0x0012_b5f8)?; // cBE_getInterface(7): network
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let local = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
        // network game: ranked sets 8; local game: entry bit 1 sets 1
        let mut store = false;
        if !local {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
            let unranked = c.g[V0][0] == 0;
            c.g[V0][0] = 8;
            if !unranked {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                store = true;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 1;
            let bit1 = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if bit1 {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                store = true;
            }
        }
        if store {
            c.cycles += 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xb34), c.g[V0][0]);
        }
        // entry bit 2: rider metrix 0.5f; then the base score
        c.cycles += 11;
        c.g[V1][0] = 0xc;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.mult(0, A2, S2, V1);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8b8));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
        c.g[V0][0] &= 1;
        let bit2 = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        if bit2 {
            c.cycles += 3;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1c4));
        }
        c.cycles += 9;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        call_ptr(c, 0x0011_7520, 0x0012_b678)?; // RiderMetrix_SetBaseScore
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameModule* GameModule_ctor(cGameModule* self)` (0x22e968): constructs the
/// game module. Runs the base constructor (`0x231cd0`), points the vtable at
/// 0x47d110, clears `+0xc/+0x24/+0x48/+0x7c/+0x84`, runs the setup calls
/// (`0x244240(0)`, the sub-object constructors at `+0xb0`, `+0x100`, `+0x158`,
/// `+0x178`, `+0x1b8`), fills the embedded state-table entries (`+0xd0..+0x228`
/// with their vtables 0x47d... and ids 3/4/5/8/0xa/0xb/7/1) and clears the
/// eleven words from `+0x68` upward. Returns `self`.
///
/// This port keeps the generated register sequence (the callees see every
/// register) and only converts the call protocol (`call_ptr`, frame, events).
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn game_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 136;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x3;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = 0xa;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S3][0] = 0x4;
        call_ptr(c, 0x00231cd0, 0x0022e998)?;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2ef0));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S4][0] as u32), c.g[V0][0]);
        c.g[A0][0] = 0u64;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x48), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x7c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), 0u64);
        call_ptr(c, 0x00244240, 0x0022e9c0)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x84), 0u64);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x00234188, 0x0022e9cc)?;
        c.g[S0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd0), 0u64);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2918));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xdc), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2d90));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b28));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xec), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe0), 0u64);
        c.g[A1][0] = 0x5;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd4), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2b80));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd0), c.g[S1][0]);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x100));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xec), c.g[V1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xfc), c.g[S0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe0), c.g[S3][0]);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        call_ptr(c, 0x00233c10, 0x0022ea40)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x124), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x118), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2970));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[A0][0] = 0x8;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x124), c.g[V0][0]);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x134), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2de8));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x128), 0u64);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x118), c.g[A0][0]);
        c.g[A2][0] = 0xb;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x11c), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x29c8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x134), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2a78));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x128), c.g[A0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x144), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x158));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x138), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x12c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x138), c.g[A2][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x154), c.g[S0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x148), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x144), c.g[V0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x13c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x14c), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x148), c.g[S2][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x154), c.g[A1][0]);
        call_ptr(c, 0x00234ef8, 0x0022ead0)?;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x178));
        call_ptr(c, 0x00236868, 0x0022ead8)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x198), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x18c), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2e40));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x7;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x198), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c30));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a8), c.g[S0][0]);
        c.g[V0][0] = 0x1;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x19c), 0u64);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1b8));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x190), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a0), c.g[S1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1b0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1b4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a8), c.g[A1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
        call_ptr(c, 0x00236208, 0x0022eb30)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e8), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1dc), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c88));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f8), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e98));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e8), c.g[V0][0]);
        c.g[A0][0] = 0xffffffffffffffff;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1ec), 0u64);
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x68));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
        c.g[A1][0] = 0x1;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e0), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f8), c.g[V1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1ec), c.g[S3][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x228), c.g[A0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x224), 0u64);
        'l_22eb88: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0xc), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[V0][0] as u32), 0u64);
            let t = (c.g[A1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0022eb88);
                }
                continue 'l_22eb88;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S4][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `cPlayer* Player_ctor(cPlayer* self, int embed_rider)` (0x125c70): constructs a
/// player. With a non-zero second argument the embedded rider (`self + 0xe10`)
/// is constructed first (`Rider_ctor`, after the 30 member slots that point at
/// it have been set). Then `RiderBase_ctor(self, 0)` runs and the rider's vtable
/// pointers are set; when the second argument is zero the vtable tables are
/// copied by two block-copy loops (0x125d74, 0x125dd8) with the offsets fixed
/// up. Finally the rider flags are set and the object created through
/// `0x270070`. Returns `self`.
///
/// This port keeps the generated register sequence (the callees see every
/// register) and only converts the call protocol (`call_ptr`, events).
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_125d18: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S1][0] = c.g[A1][0];
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            if t {
                break 'b_125d18;
            } else {
                c.cycles += 34;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe10));
                c.g[A0][0] = c.g[V0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0xd20), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc70), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xb40), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xb00), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xaf0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xad0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x610), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x520), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x470), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3b0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3a0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x398), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x384), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x364), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x358), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x340), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2b8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2a4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x288), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x224), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, 0x0011b698, 0x00125d18)?;
                break 'b_125d18;
            }
        }
        'b_125e68: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0010fcd8, 0x00125d24)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = 0x460000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ca0));
            c.g[A0][0] = 0x460000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x460000;
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[A3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
            c.g[T6][0] = 0x460000;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_125e68;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_125d74: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld((c.g[A1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[V1][0] as u32), c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00125d74);
                        }
                        continue 'l_125d74;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7ca0));
                        c.g[T7][0] = c.ld((c.g[A1][0] as u32));
                        c.g[T4][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7c58));
                        c.sd((c.g[V1][0] as u32), c.g[T7][0]);
                        c.g[T3][0] = c.g[T5][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_add(0xa0));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe10));
                        c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_125dd8: loop {
                            c.cycles += 11;
                            c.g[T0][0] = c.ld((c.g[T4][0] as u32));
                            c.g[T1][0] = c.ld((c.g[T4][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = c.ld((c.g[T4][0] as u32).wrapping_add(0x10));
                            c.g[S1][0] = c.ld((c.g[T4][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[T3][0] as u32), c.g[T0][0]);
                            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                            c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                            c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x20));
                            let t = c.g[T4][0] != c.g[A1][0];
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00125dd8);
                                }
                                continue 'l_125dd8;
                            } else {
                                c.cycles += 25;
                                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7c58));
                                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xa0));
                                c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A3][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x80));
                                c.g[T0][0] =
                                    sx((c.g[T0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[A2][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x90));
                                c.g[A3][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.g[S2][0] = c.ld((c.g[T4][0] as u32));
                                c.g[V0][0] = c.ld((c.g[T4][0] as u32).wrapping_add(0x8));
                                c.sd((c.g[T3][0] as u32), c.g[S2][0]);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                c.sw((c.g[T1][0] as u32).wrapping_add(0x6c0), c.g[T5][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A2][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[A3][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A0][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[T0][0]);
                                break 'b_125e68;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[V0][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x7cc8));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
        c.g[V1][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdf4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdfc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe00), 0u64);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x874), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x00270070, 0x00125e98)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdf8), c.g[V0][0]);
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

/// `void SSXApp_Init(cSSXApp* self, int argc, char** argv)` (0x226900): the
/// application's start-up: console config, the pad cache (0x2ef0-byte player
/// manager), the graphics manager and its vtable setup, light and bezier
/// managers, the two input pads, the command line, the load indicator, the
/// pre-start screen module and the FE async manager.
///
/// NOT registered: `SSXApp_ParseCommandLine` and the console/app-manager
/// constructors it calls are real, unslotted functions that read the
/// disc/config, so no bounded differential fixture exists. The port keeps the
/// generated register sequence and only converts the call protocol.
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn ssx_app_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x1;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[S5][0] = 0x480000;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x848), c.g[S2][0]);
        call_ptr(c, 0x002c7440, 0x00226940)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65c0));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x2efc;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022695c)?;
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        call_ptr(c, 0x00326a28, 0x0022696c)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x850), c.g[V0][0]);
        call_ptr(c, 0x00375a08, 0x00226978)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x854), c.g[V0][0]);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00226990)?;
        call_ptr(c, 0x0038dbd0, 0x00226998)?;
        call_ptr(c, 0x0038aef8, 0x002269a0)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65b0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x002269b8)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x844), c.g[V0][0]);
        'l_2269c0: loop {
            c.cycles += 10;
            c.g[A0][0] = 0x2a4;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x65a0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x002269d4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            c.sw((c.g[V0][0] as u32), 0u64);
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002269c0);
                }
                continue 'l_2269c0;
            } else {
                c.cycles += 94;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), 0u64);
                c.g[A2][0] = c.g[S3][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), 0u64);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x00227bf0, 0x00226a04)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A1][0] = 0x100;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00226a20)?;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0x480000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54c0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28b0));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x003b39b8, 0x00226a44)?;
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[A2][0] = 0x480000;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6590));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[T2][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x00226a68)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                call_ptr(c, 0x00369ff8, 0x00226a78)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6580));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x00226a90)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x480000;
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[A2][0] = 0x100;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6570));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x13));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = 0xc;
                call_ptr(c, 0x00317d70, 0x00226af0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00232328, 0x00226af8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00316ef8, 0x00226b04)?;
                call_ptr(c, 0x00237cf8, 0x00226b0c)?;
                c.g[A1][0] = 0x480000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6560));
                c.g[A3][0] = 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                c.g[A0][0] = 0x590;
                call_ptr(c, 0x00317d70, 0x00226b28)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3590, 0x00226b30)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3648, 0x00226b3c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void Rider_SnapToNearestAIPath(Rider* self)` (0x112d58): finds the reset
/// path nearest to the rider (`cPathSys_getNearResetPaths`: up to six paths
/// around `self + 0x110`) by the sum of the squared distances to the nearest
/// point and to the point 796 units ahead, skipping paths the rider's being
/// (`+0x6c0`, vtable slot 8) refuses when the path is nearly finished (length -
/// 200 < nearest parameter). The nearest path replaces the rider's current one
/// (`+0xab8`; unchanged when it is already the nearest) and the rider's AI
/// aim data is refreshed (`+0xabc`, `+0x490`, `+0x4a0`, `+0x4c0`, `+0x4c4`).
///
/// This port keeps the generated register sequence (VU0 and FPU ops included)
/// and only converts the call protocol (`call_ptr`, events).
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn rider_snap_to_nearest_ai_path(c: &mut Cpu) {
    guest(c, |c| {
        'b_112eec: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
            c.g[A3][0] = 0x6;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[A2][0] = c.g[SP][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = 0x4d0000;
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xe8));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
            call_ptr(c, 0x0026af98, 0x00112da8)?;
            c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x7c78));
            if (c.g[V0][0] as i64) <= 0 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                break 'b_112eec;
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0x43480000;
                c.mtc1(1, 24);
                c.g[S3][0] = c.g[V0][0];
                c.g[AT][0] = 0x44470000;
                c.mtc1(1, 23);
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
                c.g[S0][0] = c.g[SP][0];
                'l_112dd0: loop {
                    'b_112ee0: {
                        'b_112e2c: {
                            c.cycles += 17;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[T0][0] = 0x1;
                            call_ptr(c, 0x0026a428, 0x00112de8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            c.mov_s(21, 0);
                            call_ptr(c, 0x0026ac48, 0x00112df4)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c0));
                            c.mov_s(20, 0);
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00112e0c)?;
                            if c.g[V0][0] != 0u64 {
                                c.add_s(0, 21, 23);
                                break 'b_112e2c;
                            } else {
                                c.cycles += 5;
                                c.sub_s(0, 20, 24);
                                c.c_lt_s(0, 21);
                                if c.fcc() {
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                    break 'b_112ee0;
                                } else {
                                    c.cycles += 1;
                                    c.add_s(0, 21, 23);
                                    break 'b_112e2c;
                                }
                            }
                        }
                        c.cycles += 40;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x64));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                        call_ptr(c, 0x00269f18, 0x00112e40)?;
                        c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 1);
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.add_s(1, 1, 0);
                        c.c_lt_s(1, 22);
                        let t = !c.fcc();
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_112ee0;
                        } else {
                            c.cycles += 5;
                            c.g[S2][0] = c.lw((c.g[S0][0] as u32));
                            c.mov_s(22, 1);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                            c.g[V0][0] = c.g[S2][0] ^ c.g[V0][0];
                            if c.g[V0][0] == 0 {
                                c.g[S2][0] = 0u64;
                            }
                            break 'b_112ee0;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00112dd0);
                        }
                        continue 'l_112dd0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                        break 'b_112eec;
                    }
                }
            }
        }
        'b_112f7c: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            if t {
                break 'b_112f7c;
            } else {
                'b_112efc: {
                    c.cycles += 2;
                    if c.g[S2][0] != 0u64 {
                        c.sw((c.g[S1][0] as u32).wrapping_add(0xab8), c.g[S2][0]);
                        break 'b_112efc;
                    } else {
                        break 'b_112efc;
                    }
                }
                c.cycles += 32;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xabc));
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c8));
                c.g[T1][0] = 0x1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x490));
                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xabc));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                call_ptr(c, 0x0026a638, 0x00112f24)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c4));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c4));
                call_ptr(c, 0x00269f18, 0x00112f3c)?;
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c4));
                c.g[A0][0] = c.g[S0][0];
                c.g[AT][0] = 0x44470000;
                c.mtc1(1, 1);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x68));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.add_s(0, 0, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xab8));
                c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x490));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x68));
                call_ptr(c, 0x00269f18, 0x00112f68)?;
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c4));
                c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x4a0));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c0));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                break 'b_112f7c;
            }
        }
        c.cycles += 12;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xe8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, check_lohi, fill, Rng, DATA, RA as SENTINEL};
    use std::ptr::addr_of_mut;

    /// Fakes are live only in machines that hold `OC430_MAGIC` at `OC430_CTRL`.
    pub(crate) const OC430_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC430_MAGIC: u32 = 0x4f43_4330;

    /// Result a fake callee returns instead of the hashed pointer, where the
    /// function under test needs a usable value (call index `n`, `a0` as set by
    /// the caller).
    fn oc430_result(addr: u32, a0: u64, n: u32) -> Option<u64> {
        match addr {
            0x0031_7d70 => Some((DATA + 0x1_0000 + 0x1000 * (n & 15)) as u64),
            0x0031_7e30 => Some((DATA + 0x6000 + 0x800 * (n & 7)) as u64),
            0x0039_7b08 => Some(a0),
            0x0014_a0e0 => Some(a0 % 3),
            0x0014_a080 => Some((a0 * 3 + 1) % 10),
            _ => None,
        }
    }

    /// One logged fake call (`fakes::fake`), with `oc430_result` applied.
    fn oc430_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0];
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            if let Some(v) = oc430_result(addr, a0, n) {
                c.g[V0][0] = v;
            }
        }
    }

    macro_rules! oc430_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC430_CTRL) != OC430_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    oc430_fake_call(c, $addr);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once per
            /// process; with the gate off they run what the slots held).
            pub(crate) fn oc430_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc430_fakes! {
        (oc430_fake_317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc430_fake_3554b0, 0x0035_54b0, s11, OVERRIDE_003554B0, f_003554b0__generated),
        (oc430_fake_359f88, 0x0035_9f88, s11, OVERRIDE_00359F88, f_00359f88__generated),
        (oc430_fake_31d790, 0x0031_d790, s10, OVERRIDE_0031D790, f_0031d790__generated),
        (oc430_fake_317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
        (oc430_fake_317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc430_fake_274918, 0x0027_4918, s07, OVERRIDE_00274918, f_00274918__generated),
        (oc430_fake_273a00, 0x0027_3a00, s07, OVERRIDE_00273A00, f_00273a00__generated),
        (oc430_fake_273970, 0x0027_3970, s07, OVERRIDE_00273970, f_00273970__generated),
        (oc430_fake_2724e8, 0x0027_24e8, s07, OVERRIDE_002724E8, f_002724e8__generated),
        (oc430_fake_152ba8, 0x0015_2ba8, s01, OVERRIDE_00152BA8, f_00152ba8__generated),
        (oc430_fake_416ce0, 0x0041_6ce0, s15, OVERRIDE_00416CE0, f_00416ce0__generated),
        (oc430_fake_145970, 0x0014_5970, s01, OVERRIDE_00145970, f_00145970__generated),
        (oc430_fake_147f78, 0x0014_7f78, s01, OVERRIDE_00147F78, f_00147f78__generated),
        (oc430_fake_14f900, 0x0014_f900, s01, OVERRIDE_0014F900, f_0014f900__generated),
        (oc430_fake_1515b8, 0x0015_15b8, s01, OVERRIDE_001515B8, f_001515b8__generated),
        (oc430_fake_14a0e0, 0x0014_a0e0, s01, OVERRIDE_0014A0E0, f_0014a0e0__generated),
        (oc430_fake_14a080, 0x0014_a080, s01, OVERRIDE_0014A080, f_0014a080__generated),
        (oc430_fake_317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc430_fake_283298, 0x0028_3298, s07, OVERRIDE_00283298, f_00283298__generated),
        (oc430_fake_275d10, 0x0027_5d10, s07, OVERRIDE_00275D10, f_00275d10__generated),
        (oc430_fake_120d58, 0x0012_0d58, s00, OVERRIDE_00120D58, f_00120d58__generated),
        (oc430_fake_125958, 0x0012_5958, s00, OVERRIDE_00125958, f_00125958__generated),
        (oc430_fake_14f7a8, 0x0014_f7a8, s01, OVERRIDE_0014F7A8, f_0014f7a8__generated),
        (oc430_fake_14dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc430_fake_117520, 0x0011_7520, s00, OVERRIDE_00117520, f_00117520__generated),
        (oc430_fake_397b08, 0x0039_7b08, s13, OVERRIDE_00397B08, f_00397b08__generated),
        (oc430_fake_397c40, 0x0039_7c40, s13, OVERRIDE_00397C40, f_00397c40__generated),
        (oc430_fake_39e2a0, 0x0039_e2a0, s13, OVERRIDE_0039E2A0, f_0039e2a0__generated),
        (oc430_fake_39f400, 0x0039_f400, s13, OVERRIDE_0039F400, f_0039f400__generated),
        (oc430_fake_14dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc430_fake_231cd0, 0x0023_1cd0, s06, OVERRIDE_00231CD0, f_00231cd0__generated),
        (oc430_fake_233c10, 0x0023_3c10, s06, OVERRIDE_00233C10, f_00233c10__generated),
        (oc430_fake_234188, 0x0023_4188, s06, OVERRIDE_00234188, f_00234188__generated),
        (oc430_fake_234ef8, 0x0023_4ef8, s06, OVERRIDE_00234EF8, f_00234ef8__generated),
        (oc430_fake_236208, 0x0023_6208, s06, OVERRIDE_00236208, f_00236208__generated),
        (oc430_fake_236868, 0x0023_6868, s06, OVERRIDE_00236868, f_00236868__generated),
        (oc430_fake_244240, 0x0024_4240, s06, OVERRIDE_00244240, f_00244240__generated),
        (oc430_fake_10fcd8, 0x0010_fcd8, s00, OVERRIDE_0010FCD8, f_0010fcd8__generated),
        (oc430_fake_11b698, 0x0011_b698, s00, OVERRIDE_0011B698, f_0011b698__generated),
        (oc430_fake_270070, 0x0027_0070, s07, OVERRIDE_00270070, f_00270070__generated),
        (oc430_fake_26afb8, 0x0026_afb8, s07, OVERRIDE_0026AFB8, f_0026afb8__generated),
        (oc430_fake_26a428, 0x0026_a428, s07, OVERRIDE_0026A428, f_0026a428__generated),
        (oc430_fake_26ac48, 0x0026_ac48, s07, OVERRIDE_0026AC48, f_0026ac48__generated),
        (oc430_fake_269f18, 0x0026_9f18, s07, OVERRIDE_00269F18, f_00269f18__generated),
        (oc430_fake_26a638, 0x0026_a638, s07, OVERRIDE_0026A638, f_0026a638__generated),
    }

    /// `ModObj_AddMultiSplineModifier` against the generated body; the
    /// allocator, the constructor and the setter are gated fakes (every call's
    /// registers are logged and compared, some calls unwind).
    #[test]
    fn mod_obj_add_multi_spline_modifier_matches() {
        oc430_install_fakes();
        covered(
            ee::s11::f_00355b30__generated,
            mod_obj_add_multi_spline_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_5b30, 3);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0035_5b30;
                let node = DATA + 0x1000;
                c.bus.write32(node + 0x18, r.next_u32());
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x2000 + (r.next_u32() & 0xff0));
            },
        );
    }

    /// `ChaseCameraAlgorithm_dtor`: the five member destructors and the
    /// optional `delete` (bit 0 of the flags) are gated fakes.
    #[test]
    fn chase_camera_algorithm_dtor_matches() {
        oc430_install_fakes();
        covered(
            ee::s01::f_00162458__generated,
            chase_camera_algorithm_dtor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0016_2458, 6);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0016_2458;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// Channel-cache tables (0x4fe7a0 / 0x4fe860) for `AnimDataBase_FreeChanCaches`:
    /// a few of the 48 entries point at 20 records of 0x24 bytes whose object
    /// and block pointers are zero or non-zero.
    fn oc430_chan_caches(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for i in 0..48u32 {
            let live = if seed % 4 == 0 {
                i < 2
            } else {
                r.next_u32() % 12 == 0
            };
            let base = DATA + 0x1000 + i * 0x300;
            c.bus
                .write32(0x004f_e7a0 + 4 * i, if live { base } else { 0 });
            c.bus.write32(0x004f_e860 + 4 * i, r.next_u32());
            if live {
                for j in 0..20u32 {
                    let rec = base + 0x24 * j;
                    let obj = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x20000 + 0x40 * (i * 20 + j)
                    };
                    c.bus.write32(rec, obj);
                    let blk = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(rec + 4, blk);
                }
            }
        }
    }

    /// `AnimDataBase_FreeChanCaches`: empty and live entries, records with
    /// and without object / block pointers; the frees are gated fakes.
    #[test]
    fn anim_data_base_free_chan_caches_matches() {
        oc430_install_fakes();
        covered(
            ee::s10::f_00311110__generated,
            anim_data_base_free_chan_caches,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_1110, 30);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0031_1110;
                oc430_chan_caches(c, &mut r, seed);
            },
        );
    }

    /// The same with event delivery at the loop back edges.
    #[test]
    fn anim_data_base_free_chan_caches_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s10::f_00311110__generated,
            anim_data_base_free_chan_caches,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_1111, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_chan_caches(c, &mut r, seed);
                let mode = schedule(c, &mut r, seed, 0x0031_1110);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// Fixture for `InSeq_InstanceRelease`: the instance at `DATA + 0x1000`
    /// (live or idle, 0..3 channels, an array with a count word before it or
    /// none).
    fn oc430_inseq_instance(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let this = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(this + 4 * k, r.next_u32());
        }
        c.bus
            .write32(this + 4, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
        let desc = DATA + 0x2000;
        c.bus.write32(this + 0x10, desc);
        // the channel count is the byte at `desc + 1`
        c.bus
            .write32(desc, (r.next_u32() % 4) << 8 | (r.next_u32() & 0xff));
        let array = DATA + 0x4010;
        c.bus
            .write32(this + 0x18, if seed % 7 == 3 { 0 } else { array });
        c.bus.write32(array - 0x10, r.next_u32() % 4);
        c.g[A0][0] = sx(this);
    }

    /// `InSeq_InstanceRelease`: idle and live instances, no / some channels,
    /// no / empty / non-empty channel array (LO/HI of the count multiply too).
    #[test]
    fn in_seq_instance_release_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s07::f_002746f8__generated,
            in_seq_instance_release,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_46f8, 8);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0027_46f8;
                oc430_inseq_instance(c, &mut r, seed);
            },
        );
    }

    /// The same with event delivery at the two loop back edges.
    #[test]
    fn in_seq_instance_release_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s07::f_002746f8__generated,
            in_seq_instance_release,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_46f9, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_inseq_instance(c, &mut r, seed | 1);
                let mode = schedule(c, &mut r, seed, 0x0027_46f8);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// Fixture for `PlaybackMan_FreeData`: the manager at `DATA + 0x1000` with
    /// 0..3 entries of 12 bytes (buffer words zero or not), an entry table,
    /// a `+0x4c` block and a data block, each present or not. The async
    /// handle at `+0x78` is `-1` unless `cancel`.
    fn oc430_playback_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, cancel: bool) {
        let this = DATA + 0x1000;
        for k in 0..0x30 {
            c.bus.write32(this + 4 * k, r.next_u32());
        }
        c.bus.write32(
            this + 0x78,
            if cancel {
                r.next_u32() & 0xffff
            } else {
                u32::MAX
            },
        );
        let n = (r.next_u32() % 4) as i32 - if seed % 5 == 0 { 2 } else { 0 };
        c.bus.write32(this + 0x50, n as u32);
        let table = DATA + 0x2000;
        c.bus
            .write32(this + 0x58, if seed % 4 == 0 { 0 } else { table });
        for k in 0..4 {
            let blk = if r.next_u32() % 3 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(table + 12 * k + 4, blk);
            c.bus.write32(table + 12 * k + 8, r.next_u32());
        }
        c.bus
            .write32(this + 0x4c, if seed % 3 == 0 { 0 } else { r.next_u32() });
        c.bus
            .write32(this + 4, if seed % 7 == 0 { 0 } else { r.next_u32() });
        c.g[A0][0] = sx(this);
    }

    /// `PlaybackMan_FreeData` without a pending async read: entry tables of
    /// 0 / 1..3 entries (or a negative count), the three optional blocks.
    #[test]
    fn playback_man_free_data_matches() {
        oc430_install_fakes();
        covered(
            ee::s07::f_0026c898__generated,
            playback_man_free_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_c898, 8);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0026_c898;
                oc430_playback_fixture(c, &mut r, seed, false);
            },
        );
    }

    /// The same with event delivery at the entry loop's back edge.
    #[test]
    fn playback_man_free_data_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s07::f_0026c898__generated,
            playback_man_free_data,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_c899, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_playback_fixture(c, &mut r, seed | 1, false);
                let mode = schedule(c, &mut r, seed, 0x0026_c898);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// With a pending async read the real `AsyncFile_Cancel` runs (no slot to
    /// fake): its handle lookup finds nothing in the zeroed tables.
    #[test]
    fn playback_man_free_data_cancel_matches() {
        oc430_install_fakes();
        check(
            ee::s07::f_0026c898__generated,
            playback_man_free_data,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_c89a, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0026_c898;
                oc430_playback_fixture(c, &mut r, seed, true);
            },
        );
    }

    /// `cTransitionMan_StartTransition`: idle / running transition (end call),
    /// `kind == 9` and the `x + y + z <= t` reset, and the start path; both
    /// virtual calls go to fake methods (some unwind).
    #[test]
    fn c_transition_man_start_transition_matches() {
        covered(
            ee::s07::f_00282dd0__generated,
            c_transition_man_start_transition,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_2dd0, 2);
                c.pc = 0x0028_2dd0;
                let this = DATA + 0x1000;
                for k in 0..0x10 {
                    c.bus.write32(this + 4 * k, r.next_u32());
                }
                let vt = fakes::vtable(c, &mut r, &[0x28, 0x20]);
                c.bus.write32(this + 0xc, vt);
                c.bus.write32(
                    this + 0x18,
                    if seed % 3 == 0 { 9 } else { r.next_u32() % 9 },
                );
                c.bus.write32(c.g[GP][0] as u32 - 0x4778, r.f32(2.0));
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(if seed % 4 == 1 { 9 } else { r.next_u32() % 12 });
                c.g[A3][0] = sx(r.next_u32());
                for f in 12..16 {
                    c.f[f] = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(4.0)
                    };
                }
            },
        );
    }

    /// Fixture for `BESaveInt_LoadProfileSave`: a save buffer of 0x9c10 bytes
    /// at `DATA + 0x1000` (+4 on odd seeds: the unaligned copy loop), the CRC
    /// table zeroed (so the CRC is 0 and a zero stored word matches), forced
    /// fake results (call 0: the save size, calls 2/4/6: the interface
    /// objects, whose vtable slot `+0x10` points at fake methods).
    fn oc430_profile_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let buf = DATA + 0x1000 + if seed & 1 == 1 { 4 } else { 0 };
        fill(c, r, buf, 0x2700);
        for k in 0..0x80 {
            c.bus.write32(0x0045_1018 + 4 * k, 0);
        }
        c.bus.write32(
            buf + 0x9c00,
            if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(fakes::LOG_RESULTS, 0x9c04);
        for (k, n) in [2u32, 4, 6].into_iter().enumerate() {
            let obj = DATA + 0x2_0000 + 0x100 * k as u32;
            let vt = DATA + 0x3_0000 + 0x40 * k as u32;
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k as u32);
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, obj);
        }
        c.g[A1][0] = sx(buf);
        c.g[A2][0] = sx(r.next_u32() % 4);
    }

    /// `BESaveInt_LoadProfileSave`: matching and mismatching CRC words, four
    /// profile slots, aligned and unaligned block copies (LO/HI too); the
    /// real CRC runs, everything else is fake.
    #[test]
    fn be_save_int_load_profile_save_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s01::f_00152de0__generated,
            be_save_int_load_profile_save,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2de0, 14);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_2de0;
                oc430_profile_fixture(c, &mut r, seed);
            },
        );
    }

    /// The aggression state machine against a plain restatement of the
    /// C++ switch (state / level / points transitions).
    #[test]
    fn be_aggression_step_matches_switch() {
        for st in -2..6i64 {
            for lvl in 0..8i64 {
                let (_, s, l, p) = be_aggression_step(st, lvl, 7);
                let (mut es, mut el, mut ep) = (st, lvl, 7u64);
                match st {
                    0 if lvl >= 3 => (el, ep) = (2, 10),
                    1 if lvl >= 4 => (el, ep, es) = (2, 10, 3),
                    2 if lvl <= 0 => el = 1,
                    2 if lvl >= 4 => (el, ep) = (3, 15),
                    3 if lvl == 0 => (es, el, ep) = (1, 3, 15),
                    3 if lvl >= 5 => (el, ep) = (4, 20),
                    _ => {}
                }
                assert_eq!((s, l, p), (es, el, ep), "st {st} lvl {lvl}");
            }
        }
    }

    /// Fixture for `BEAggressionInt_DecayAll`: 1..3 players, profile and
    /// character lookups return small indices (see `oc430_result`), every
    /// aggression cell (state in -1..4, points 0..40) random, the HUD call
    /// on or off (the real HUD function returns early on the fixture world).
    fn oc430_aggression_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(0x0053_05e0 + 0x30, 1 + seed % 3);
        c.bus.write8(0x0053_05b0 + 0x49, (seed % 4 == 0) as u8);
        for p in 0..3u32 {
            for ca in 0..10u32 {
                for j in 0..10u32 {
                    let cell = 0x004a_7869 + p * 0x9b50 + ca * 0xf88 + 3 * j;
                    c.bus
                        .write8(cell, ((r.next_u32() % 6) as u8).wrapping_sub(1));
                    c.bus.write8(cell + 1, r.next_u32() as u8);
                    c.bus.write8(cell + 2, (r.next_u32() % 41) as u8);
                }
            }
        }
        let world = DATA + 0x5000;
        c.bus.write32(c.g[GP][0] as u32 - 0x848, world);
        c.bus.write32(world + 0x84, DATA + 0x5100);
        c.bus.write32(DATA + 0x5100 + 0x28, DATA + 0x5200);
        c.bus.write32(DATA + 0x5200, 5);
    }

    /// `BEAggressionInt_DecayAll`: every state/level combination, with and
    /// without the HUD call (LO/HI of the index multiplies too).
    #[test]
    fn be_aggression_int_decay_all_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s01::f_00155e58__generated,
            be_aggression_int_decay_all,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_5e58, 20);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_5e58;
                oc430_aggression_fixture(c, &mut r, seed);
            },
        );
    }

    // The events test for `BEAggressionInt_DecayAll` was dropped: its loop never reaches a scheduled event on any bounded fixture.

    /// `ScriptEngine_LoadMainScripts`: the engine at `DATA + 0x1000` with bank 0
    /// already loaded (so the real `LoadBankSync` returns at once); the
    /// allocator hands out distinct pool blocks (see `oc430_result`).
    fn oc430_script_engine_fixture(c: &mut Cpu, r: &mut Rng) {
        let this = DATA + 0x1000;
        c.bus.write32(this + 0x628, r.next_u32() | 1);
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn script_engine_load_main_scripts_matches() {
        oc430_install_fakes();
        covered(
            ee::s07::f_00278358__generated,
            script_engine_load_main_scripts,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_8358, 11);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0027_8358;
                oc430_script_engine_fixture(c, &mut r);
            },
        );
    }

    /// The same with event delivery at the six loops' back edges.
    #[test]
    fn script_engine_load_main_scripts_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s07::f_00278358__generated,
            script_engine_load_main_scripts,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_8359, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_script_engine_fixture(c, &mut r);
                let mode = schedule(c, &mut r, seed, 0x0027_8358);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// Fixture for `cGameLoadScreen_Load`: the graphics manager (`gp - 0x854`)
    /// with a vtable whose slot `+0x160` is a fake, the world (`gp - 0x848`),
    /// the lodge flag (`gp - 0x1718`) and the race setup (no path, mode 0..7).
    /// The picture state (which runs the real `LoadPictures`) is never chosen:
    /// that needs a path set without the lodge flag.
    fn oc430_load_screen_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let gp = c.g[GP][0] as u32;
        let (grman, vt, world) = (DATA + 0x1000, DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(gp - 0x854, grman);
        c.bus.write32(grman + 0x10d8, vt);
        c.bus.write16(vt + 0x160, (r.next_u32() as i16 & !3) as u16);
        c.bus.write32(vt + 0x164, fakes::FAKE);
        c.bus.write32(gp - 0x848, world);
        c.bus.write32(world + 0x88, r.next_u32());
        c.bus.write32(world + 0x8c, r.next_u32());
        c.bus
            .write32(gp - 0x1718, if seed % 3 == 0 { 1 } else { 0 });
        c.bus.write8(0x0053_5bc8 + 0x49, 0);
        c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 8) as u8);
        c.g[A0][0] = sx(DATA + 0x5000);
    }

    /// `cGameLoadScreen_Load`: the lodge state (flag, or mode 5/6) and the
    /// conquer state (other modes); the allocator, constructors, the file load,
    /// the UI push and the graphics manager's method are fakes.
    #[test]
    fn c_game_load_screen_load_matches() {
        oc430_install_fakes();
        covered(
            ee::s06::f_00232e20__generated,
            c_game_load_screen_load,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0023_2e20, 10);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0023_2e20;
                oc430_load_screen_fixture(c, &mut r, seed);
            },
        );
    }

    /// `World_InitSphereTreeCache`: 0..3 small and big buffers, depths -1..3
    /// (a negative depth leaves the size 0); the allocator is a gated fake.
    #[test]
    fn world_init_sphere_tree_cache_matches() {
        oc430_install_fakes();
        covered(
            ee::s11::f_0032dc10__generated,
            world_init_sphere_tree_cache,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_dc10, 10);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0032_dc10;
                c.g[A0][0] = sx(r.next_u32() % 4);
                c.g[A1][0] = sx((r.next_u32() % 5).wrapping_sub(1));
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[A3][0] = sx((r.next_u32() % 5).wrapping_sub(1));
            },
        );
    }

    /// The same with event delivery at the four loop back edges.
    #[test]
    fn world_init_sphere_tree_cache_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s11::f_0032dc10__generated,
            world_init_sphere_tree_cache,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_dc11, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                let args = [
                    r.next_u32() % 4,
                    (r.next_u32() % 5).wrapping_sub(1),
                    r.next_u32() % 4,
                    (r.next_u32() % 5).wrapping_sub(1),
                ];
                let mode = schedule(c, &mut r, seed, 0x0032_dc10);
                for (i, a) in args.into_iter().enumerate() {
                    c.g[A0 + i][0] = sx(a);
                }
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// `AI_ApplyPlayerRiderSettings`: random option word and per-player flag
    /// words (every boost mode, uber, level and network combination); the six
    /// callees are fakes (some unwind).
    #[test]
    fn ai_apply_player_rider_settings_matches() {
        oc430_install_fakes();
        covered(
            ee::s00::f_0012b498__generated,
            ai_apply_player_rider_settings,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_b498, 6);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0012_b498;
                for k in 0..12 {
                    c.bus.write32(0x0053_08b8 + 4 * k, r.next_u32());
                }
                c.bus.write32(
                    0x0053_4b30,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus
                    .write32(0x0053_4b34, if seed % 4 == 0 { 0 } else { r.next_u32() });
                let (obj, rider, metrix, world) =
                    (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(obj + 0x18, rider);
                c.bus.write32(rider + 0x790, metrix);
                c.bus.write32(world + 0x78, r.next_u32());
                c.bus.write32(c.g[GP][0] as u32 - 0x848, world);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = sx(r.next_u32() % 4);
            },
        );
    }

    /// `GameModule_ctor`: all seven constructor / setup callees are gated
    /// fakes (some unwind); the state-table stores and the final clearing loop
    /// run for real.
    #[test]
    fn game_module_ctor_matches() {
        oc430_install_fakes();
        covered(ee::s05::f_0022e968__generated, game_module_ctor, 200, &|c, seed| {
            let _ = machine(c, seed, 0x0022_e968, 7);
            c.bus.write32(OC430_CTRL, OC430_MAGIC);
            c.pc = 0x0022_e968;
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }

    /// The same with event delivery at the clearing loop's back edge.
    #[test]
    fn game_module_ctor_events_match() {
        oc430_install_fakes();
        check_events(ee::s05::f_0022e968__generated, game_module_ctor, 90, &|c, seed| {
            let mut r = machine(c, seed, 0x0022_e969, 0);
            c.bus.write32(OC430_CTRL, OC430_MAGIC);
            let mode = schedule(c, &mut r, seed, 0x0022_e968);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[RA][0] = SENTINEL as u64;
            mode
        });
    }

    /// Fixture for `Player_ctor`: the player at `DATA + 0x1000` with a rider
    /// pointer at `+0x18`, the world's app object chain for the last call.
    fn oc430_player_fixture(c: &mut Cpu, seed: u32) {
        let this = DATA + 0x1000;
        c.bus.write32(this + 0x18, DATA + 0x3000);
        let world = DATA + 0x4000;
        c.bus.write32(c.g[GP][0] as u32 - 0x848, world);
        c.bus.write32(world + 0x84, DATA + 0x4100);
        c.bus.write32(DATA + 0x4100 + 0x28, DATA + 0x4200);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
    }

    /// `Player_ctor`: with and without the embedded-rider construction (and
    /// so without / with the vtable copy loops); the three callees are fakes.
    #[test]
    fn player_ctor_matches() {
        oc430_install_fakes();
        covered(ee::s00::f_00125c70__generated, player_ctor, 200, &|c, seed| {
            let _ = machine(c, seed, 0x0012_5c70, 3);
            c.bus.write32(OC430_CTRL, OC430_MAGIC);
            c.pc = 0x0012_5c70;
            oc430_player_fixture(c, seed);
        });
    }

    /// The same with event delivery at the copy loops' back edges.
    #[test]
    fn player_ctor_events_match() {
        oc430_install_fakes();
        check_events(ee::s00::f_00125c70__generated, player_ctor, 90, &|c, seed| {
            let mut r = machine(c, seed, 0x0012_5c71, 0);
            c.bus.write32(OC430_CTRL, OC430_MAGIC);
            let mode = schedule(c, &mut r, seed, 0x0012_5c70);
            oc430_player_fixture(c, seed);
            c.g[RA][0] = SENTINEL as u64;
            mode
        });
    }

    /// The `miss` hook of the snap test: a fake call, except that the being's
    /// virtual method (at `FAKE`) answers 0 or non-zero by a hash.
    fn oc430_snap_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0];
        fakes::fake(c, target, ra);
        if c.pc == ra && target == fakes::FAKE && fakes::mix(a0 ^ c.cycles) & 1 == 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `Rider_SnapToNearestAIPath`: 1..6 candidate paths (forced count from
    /// the near-path query, random path pointers and fake distances), the being
    /// method answering both ways, with and without a current path (`+0xab8`).
    #[test]
    fn rider_snap_to_nearest_ai_path_matches() {
        oc430_install_fakes();
        covered(
            ee::s00::f_00112d58__generated,
            rider_snap_to_nearest_ai_path,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_2d58, 20);
                c.hooks.miss = oc430_snap_miss;
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0011_2d58;
                let this = DATA + 0x1000;
                for k in 0..0x40 {
                    c.bus.write32(this + 0x100 + 4 * k, r.next_u32());
                }
                let being = DATA + 0x6000;
                c.bus
                    .write16(being + 0x40, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(being + 0x44, fakes::FAKE);
                c.bus.write32(this + 0x6c0, being);
                c.bus
                    .write32(this + 0xab8, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(this + 0xabc, DATA + 0x3000);
                c.bus.write32(fakes::LOG_RESULTS, 1 + r.next_u32() % 6);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
