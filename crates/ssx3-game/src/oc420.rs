//! Staging module oc420 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc420`.

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
        0x0027_c2a8,
        script_cam_get_target_params,
        "ScriptCam_GetTargetParams",
    ),
    (0x0027_2488, in_seq_heap_new, "InSeq_HeapNew"),
    (
        0x003a_6c98,
        world_get_section_missions,
        "World_GetSectionMissions",
    ),
    (
        0x003a_6c68,
        world_get_section_mission_count,
        "World_GetSectionMissionCount",
    ),
    (
        0x003a_d230,
        world_chunk_resolve_hook,
        "World_ChunkResolveHook",
    ),
    (0x0027_3aa8, in_seq_channel_start, "InSeq_ChannelStart"),
    (0x0022_4e50, cluno_table_alloc, "cLunoTable_Alloc"),
    (
        0x0034_0970,
        ai_fwd_diff_cache_entry_ctor,
        "AIFwdDiffCacheEntry_ctor",
    ),
    (
        0x0012_a340,
        ai_set_track_paths_buffer,
        "AI_SetTrackPathsBuffer",
    ),
    (
        0x0027_d210,
        script_engine_is_obj_bound_to_process,
        "ScriptEngine_IsObjBoundToProcess",
    ),
    (
        0x0032_7da8,
        cps2_chunk_resolve_parse_header,
        "cPS2ChunkResolve_ParseHeader",
    ),
    (0x0030_3598, wscript_fn_rand_range, "WScriptFn_RandRange"),
    (
        0x0030_0260,
        wscript_fn_stop_object_loop,
        "WScriptFn_StopObjectLoop",
    ),
    (
        0x002f_dc60,
        wscript_fn_add_parent_modifier_rev,
        "WScriptFn_AddParentModifierRev",
    ),
    (
        0x0012_3b48,
        rider_detach_from_script,
        "Rider_DetachFromScript",
    ),
    (
        0x0011_be88,
        rider_add_bolt_files_to_load_list,
        "Rider_AddBoltFilesToLoadList",
    ),
    (0x0011_b698, rider_ctor, "Rider_ctor"),
    (
        0x003a_ccd8,
        t_wscript_cache_relocate,
        "tWScriptCache_Relocate",
    ),
    (
        0x0027_9f18,
        script_engine_get_target_object,
        "ScriptEngine_GetTargetObject",
    ),
];

// ---------------------------------------------------------------------------
// Small leaves and wrappers.

/// `ScriptCam` target-params accessor (0x27c2a8): returns the word at
/// `self + 0x20`.
pub fn script_cam_get_target_params(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
    ret(c);
}

/// `InSeq_HeapAlloc` (0x272308).
const OC420_INSEQ_HEAP_ALLOC: Callee = Callee {
    addr: 0x0027_2308,
    f: ee::s07::f_00272308,
};

/// `void* InSeq_HeapNew(uint base, Heap* heap)` (0x272488): allocates
/// `base + gp[0x398]` bytes from `heap` (`InSeq_HeapAlloc(heap_arg, size)`
/// with the arguments in that order), stores `heap` at the block's first word, and
/// returns it advanced by `gp[0x398]`.
pub fn in_seq_heap_new(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x398)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        OC420_INSEQ_HEAP_ALLOC.call(c, 0x0027_24a8)?;
        Addr(c.g[V0][0] as u32).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x398)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `tWScriptCache_GetMissions` (0x3add70).
const OC420_GET_MISSIONS: Callee = Callee {
    addr: 0x003a_dd70,
    f: ee::s13::f_003add70,
};

/// `tWScriptCache_GetMissionCount` (0x3add40).
const OC420_GET_MISSION_COUNT: Callee = Callee {
    addr: 0x003a_dd40,
    f: ee::s13::f_003add40,
};

/// Shared body of the two section-mission accessors: the world's script
/// cache table (`*(*(world + 4) + 4)`) holds 12-byte entries; call `callee`
/// on entry `i` (`a1 = i * 12`, `v0 = 12` stay behind as `mult` leaves them).
fn oc420_section_entry(c: &mut Cpu, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* World_GetSectionMissions(World* w, int section)` (0x3a6c98): the
/// mission list of one script-cache entry.
pub fn world_get_section_missions(c: &mut Cpu) {
    oc420_section_entry(c, OC420_GET_MISSIONS, 0x003a_6cb8);
}

/// `int World_GetSectionMissionCount(World* w, int section)` (0x3a6c68):
/// the mission count of one script-cache entry.
pub fn world_get_section_mission_count(c: &mut Cpu) {
    oc420_section_entry(c, OC420_GET_MISSION_COUNT, 0x003a_6c88);
}

/// `void World_ChunkResolveHook(void)` (0x3ad230): forwards to
/// `LensFX_GetFlareSizeForRes` (0x2e27e8) through the function table.
pub fn world_chunk_resolve_hook(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x002e_27e8),
        0x003a_d240,
    );
}

// ---------------------------------------------------------------------------
// InSeq_ChannelStart.

/// Fields of the in-sequence player instance used by `InSeq_ChannelStart`.
pub(crate) const OC420_INST_OWNER: u32 = 0x08;
pub(crate) const OC420_INST_HDR: u32 = 0x1c;
pub(crate) const OC420_INST_CHANNELS: u32 = 0x24;
pub(crate) const OC420_INST_READY: u32 = 0x28;
pub(crate) const OC420_INST_PLAYER: u32 = 0x2c;
pub(crate) const OC420_INST_DONE: u32 = 0x30;
pub(crate) const OC420_INST_FLAGS: u32 = 0x34;
pub(crate) const OC420_INST_STARTED: u32 = 0x38;

const OC420_INSEQ_HEAP_NEW: Callee = Callee {
    addr: 0x0027_2488,
    f: ee::s07::f_00272488,
};

/// `bool InSeq_ChannelStart(Instance* self)` (0x273aa8): builds the
/// instance's channel objects (type 0 curve, 1 ramp, 2 span, 3 event track,
/// from the header's per-channel records, each allocated with
/// `InSeq_HeapNew`) and starts each through its virtual at vtable+4
/// (`done`/`flags` accumulate), then marks the instance ready, asks the
/// owner's listener for a player and starts it. Returns `self->ready`.
pub fn in_seq_channel_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[S1][0] as u32;
        Addr(this + OC420_INST_DONE).sw(c, 0);
        c.g[A0][0] = Addr(this + OC420_INST_HDR).lw(c);
        Addr(this + OC420_INST_FLAGS).sw(c, 0);
        // Header byte +2: channel count.
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(2)).lbu(c);
        c.g[A1][0] = c.g[V0][0];
        'channels: {
            if c.g[V0][0] == 0 {
                break 'channels;
            }
            c.cycles += 13;
            c.g[V1][0] = Addr(this + OC420_INST_OWNER).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[S2][0] = 0;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.g[S5][0];
            OC420_INSEQ_HEAP_ALLOC.call(c, 0x0027_3b08)?;
            Addr(this + OC420_INST_CHANNELS).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if c.g[A0][0] == 0 {
                break 'channels;
            }
            c.cycles += 2;
            c.g[S7][0] = 0;
            loop {
                // Channel record: type (u16) and size (u16) at s3.
                c.cycles += 5;
                c.g[S6][0] = c.g[S3][0];
                c.g[V0][0] = 1;
                c.g[V1][0] = Addr(c.g[S6][0] as u32).lhu(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                let kind = c.g[V1][0];
                // (ctor, HeapNew size, HeapNew return, ctor return) per type.
                let mut ctor: Option<(Callee, u64, u32, u32)> = None;
                let mut tail = true;
                if kind == 1 {
                    c.cycles += 10;
                    c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                    ctor = Some((
                        Callee {
                            addr: 0x0027_34a8,
                            f: ee::s07::f_002734a8,
                        },
                        0x28,
                        0x0027_3bac,
                        0x0027_3bb4,
                    ));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    c.g[V0][0] = 2;
                    if kind < 2 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                        if kind == 0 {
                            c.cycles += 9;
                            ctor = Some((
                                Callee {
                                    addr: 0x0027_25b8,
                                    f: ee::s07::f_002725b8,
                                },
                                0x28,
                                0x0027_3b84,
                                0x0027_3b8c,
                            ));
                        } else {
                            c.cycles += 2;
                            tail = false;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                        if kind == 2 {
                            c.cycles += 10;
                            c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                            ctor = Some((
                                Callee {
                                    addr: 0x0027_2d68,
                                    f: ee::s07::f_00272d68,
                                },
                                0x2c,
                                0x0027_3bd4,
                                0x0027_3bdc,
                            ));
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                            if kind == 3 {
                                c.cycles += 8;
                                ctor = Some((
                                    Callee {
                                        addr: 0x0027_3180,
                                        f: ee::s07::f_00273180,
                                    },
                                    0x28,
                                    0x0027_3bf8,
                                    0x0027_3c00,
                                ));
                            } else {
                                c.cycles += 2;
                                tail = false;
                            }
                        }
                    }
                }
                if let Some((ctor, size, heap_ret, ctor_ret)) = ctor {
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A0][0] = size;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    OC420_INSEQ_HEAP_NEW.call(c, heap_ret)?;
                    c.g[A0][0] = c.g[V0][0];
                    ctor.call(c, ctor_ret)?;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                }
                if tail {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                }
                // Start the channel object: vtable slot 1.
                c.g[S0][0] = sx(this + OC420_INST_DONE);
                c.cycles += 14;
                Addr(sp(c)).sw(c, 0);
                c.g[S4][0] = 0;
                c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = c.g[S3][0];
                c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[SP][0];
                c.g[V1][0] = Addr(c.g[T0][0] as u32).off(4).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_3c3c)?;
                let started = if c.g[V0][0] != 0 {
                    c.g[S4][0] = 1;
                    c.cycles += 1;
                    1
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this + OC420_INST_DONE).lw(c);
                    if c.g[V0][0] == 0 {
                        0
                    } else {
                        // This `+1` and the shared one of the started path.
                        c.cycles += 2;
                        c.g[S4][0] = 1;
                        1
                    }
                };
                Addr(c.g[S0][0] as u32).sw(c, started);
                c.cycles += 12;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(4));
                c.g[V1][0] = Addr(sp(c)).lw(c);
                c.g[V0][0] = Addr(this + OC420_INST_FLAGS).lw(c);
                c.g[A0][0] = Addr(this + OC420_INST_HDR).lw(c);
                c.g[V0][0] |= c.g[V1][0];
                Addr(this + OC420_INST_FLAGS).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(c.g[S6][0] as u32).off(2).lhu(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(2).lbu(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if c.g[V0][0] == 0 {
                    break 'channels;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3b28);
                }
            }
        }
        // Mark ready, ask the owner's listener for a player and start it.
        c.cycles += 7;
        c.g[S0][0] = 1;
        c.g[A0][0] = Addr(this + OC420_INST_OWNER).lw(c);
        Addr(this + OC420_INST_READY).sw(c, c.g[S0][0]);
        Callee {
            addr: 0x0027_4d70,
            f: ee::s07::f_00274d70,
        }
        .call(c, 0x0027_3c98)?;
        c.g[T0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        if c.g[T0][0] != 0 {
            c.cycles += 8;
            c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
            c.g[A3][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            c.g[A2][0] = Addr(c.g[A3][0] as u32).off(3).lbu(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_3cc0)?;
            Addr(this + OC420_INST_PLAYER).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[T0][0] = Addr(this + OC420_INST_PLAYER).lw(c);
        c.g[A1][0] = c.g[S1][0];
        if c.g[T0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
            c.g[V1][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(3).lbu(c);
            let target = c.g[A3][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_3cec)?;
        }
        c.cycles += 13;
        Addr(this + OC420_INST_STARTED).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(this + OC420_INST_READY).lw(c);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cLunoTable_Alloc.

/// `void cLunoTable_Alloc(cLunoTableRef* ref, int n)` (0x224e50): allocates
/// a table object (0x14 bytes, "LunoTable": nodes, refCount, modulus) into
/// `ref->table`, then an `n`-entry node array (32 bytes each, preceded by a
/// count word) whose nodes are cleared and given their vtable word at +0x1c;
/// sets `modulus = n - 1` and `refCount = 1`.
pub fn cluno_table_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x14;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x67a0));
        // operator new (EA_MemAllocNamed).
        call_ptr(c, 0x0031_7d70, 0x0022_4e84)?;
        let table = c.g[V0][0] as u32;
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 5);
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] |= 0x10;
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        c.g[S2][0] = sx(table.wrapping_add(0x10));
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x6790));
        Addr(table.wrapping_add(0x10)).sw(c, 0);
        Addr(table.wrapping_add(8)).sw(c, 0);
        c.g[A3][0] = 0;
        Addr(table.wrapping_add(0xc)).sw(c, 0);
        // operator new[] (operator_new_named).
        call_ptr(c, 0x0031_7e30, 0x0022_4eb4)?;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        Addr(c.g[V0][0] as u32).sw(c, c.g[S0][0]);
        c.g[V1][0] = c.g[A2][0];
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_0000;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x6770));
            loop {
                c.cycles += 9;
                let node = c.g[V1][0] as u32;
                Addr(node.wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                Addr(node.wrapping_add(8)).sw(c, 0);
                Addr(node).sw(c, 0);
                Addr(node.wrapping_add(0x14)).sw(c, 0);
                Addr(node.wrapping_add(0xc)).sw(c, 0);
                Addr(node.wrapping_add(0x18)).sw(c, 0);
                let more = c.g[A0][0] != c.g[A1][0];
                c.g[V1][0] = sx(node.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_4ed8);
                }
            }
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        Addr(c.g[S2][0] as u32).sw(c, c.g[A2][0]);
        c.g[A0][0] = 1;
        Addr((c.g[A1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, c.g[A0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AIFwdDiffCacheEntry_ctor.

/// One empty element-constructor loop of `AIFwdDiffCacheEntry_ctor`
/// (`for (i = n - 1; i != -1; i--) {}` for an array of `n`): `n + 1`
/// iterations; leaves `v1 = -1`, `a1 = -2`, `a2 = -1`.
fn oc420_empty_ctor_loop(c: &mut Cpu, n: u32, head: u32) {
    c.cycles += 4;
    c.g[V1][0] = n as u64;
    c.g[A2][0] = u64::MAX;
    c.g[A1][0] = sx(n.wrapping_sub(1));
    loop {
        c.cycles += 8;
        c.g[V1][0] = n as u64;
        c.g[V1][0] = c.g[A1][0];
        let more = c.g[V1][0] != c.g[A2][0];
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `AIFwdDiffCacheEntry* AIFwdDiffCacheEntry_ctor(AIFwdDiffCacheEntry* self)`
/// (0x340970): returns `self`; the member arrays (9, 8 and 2 elements) have
/// trivial constructors, so only their empty loops remain.
pub fn ai_fwd_diff_cache_entry_ctor(c: &mut Cpu) {
    c.g[V0][0] = c.g[A0][0];
    oc420_empty_ctor_loop(c, 9, 0x0034_0980);
    oc420_empty_ctor_loop(c, 8, 0x0034_09b0);
    oc420_empty_ctor_loop(c, 2, 0x0034_09e0);
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// AI_SetTrackPathsBuffer.

/// `Rider_InitTrackPaths` (0x112180).
const OC420_RIDER_INIT_TRACK_PATHS: Callee = Callee {
    addr: 0x0011_2180,
    f: ee::s00::f_00112180,
};

/// Where `AI_SetTrackPathsBuffer` continues after its mode checks.
#[derive(Clone, Copy, PartialEq)]
enum Oc420PathsNext {
    /// Straight to the epilogue (the `s0` restore already done).
    Epilogue,
    /// Return with `v0` as it is (`+1`, restore `s0`, epilogue).
    Return,
    /// Mode accepted: load the path buffer.
    Load,
    /// Plain mode check (modes 1 and 2 are rejected).
    ModeCheck,
}

/// `bool AI_SetTrackPathsBuffer(AI* self, int, int mode, void* data)`
/// (0x12a340): installs the AI path buffer `data` for the current section.
/// The level id (`SectionMan` lookup) picks which modes are valid: on levels
/// 17..21 the new-race interface selects mode 1 (flag bytes +0x48 == 5 or
/// +0x4a == 11) or mode 2 (+0x48 == 6); on the other levels modes 1 and 2 are
/// refused. On levels below 22 the global path data is purged, filled from
/// `data`, and (when the AI interface exists) every allocated rider
/// re-initialises its track paths. Returns whether the buffer was taken.
pub fn ai_set_track_paths_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        call_ptr(c, 0x0022_e0e0, 0x0012_a374)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0012_a37c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0012_a388)?;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x11));
        c.g[V1][0] = (c.g[V1][0] < 5) as u64;
        c.g[V0][0] = 0x0053_0000;
        let mut next;
        if c.g[V1][0] == 0 {
            // Level outside 17..21: only the mode check below applies.
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            next = Oc420PathsNext::ModeCheck;
        } else {
            // The new-race interface (0x535bc8): flag bytes +0x48 and +0x4a.
            c.cycles += 5;
            c.g[V1][0] = 5;
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lb(c);
            // `mode1`: the interface asks for mode 1 (else mode 2 is checked).
            let mut mode1 = false;
            if c.g[A0][0] == c.g[V1][0] {
                c.g[V0][0] = 1;
                mode1 = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4a)).lb(c);
                c.g[V0][0] = 0xb;
                let other = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 6;
                if !other {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    mode1 = true;
                }
            }
            if mode1 {
                c.cycles += 2;
                let ok = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if ok {
                    next = Oc420PathsNext::Load;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x40);
                    next = Oc420PathsNext::Epilogue;
                }
            } else {
                c.cycles += 2;
                let not6 = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                if not6 {
                    next = Oc420PathsNext::ModeCheck;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    let ok = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0;
                    if ok {
                        next = Oc420PathsNext::Load;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x40);
                        next = Oc420PathsNext::Epilogue;
                    }
                }
            }
        }
        if next == Oc420PathsNext::ModeCheck {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let rejected = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            next = if rejected {
                Oc420PathsNext::Return
            } else {
                Oc420PathsNext::Load
            };
        }
        if next == Oc420PathsNext::Load {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 22) as u64;
            let low_level = c.g[V0][0] != 0;
            c.g[S0][0] = 0x004d_0000;
            next = Oc420PathsNext::Return;
            if low_level {
                c.cycles += 11;
                c.g[S0][0] = sx(0x004d_0000u32.wrapping_add(0x33a0));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_af00, 0x0012_a414)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0026_ad70, 0x0012_a420)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                let no_ai = c.g[A0][0] == 0;
                c.g[V0][0] = 1;
                if !no_ai {
                    // Re-initialise each allocated rider's track paths.
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x78)).lw(c);
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = 0;
                    if !none {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        loop {
                            c.cycles += 8;
                            c.g[A1][0] = 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                            OC420_RIDER_INIT_TRACK_PATHS.call(c, 0x0012_a458)?;
                            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x78)).lw(c);
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0012_a448);
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        if next == Oc420PathsNext::Return {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptEngine_IsObjBoundToProcess.

/// `ScriptEngine_BindingHasProcess` (0x27d2c8).
const OC420_BINDING_HAS_PROCESS: Callee = Callee {
    addr: 0x0027_d2c8,
    f: ee::s07::f_0027d2c8,
};

/// `bool ScriptEngine_IsObjBoundToProcess(ScriptEngine* self, int idx,
/// int process)` (0x27d210): asks the object at `self[idx]` (virtual at
/// vtable+0x30: how many bindings it has) and checks each binding
/// (virtual at vtable+0x38 -> `binding[1]`) with
/// `ScriptEngine_BindingHasProcess`; true when one is bound to `process`.
pub fn script_engine_is_obj_bound_to_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S3, 0x20);
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.save(c, S4, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        c.g[S1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x34)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_d258)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        if (c.g[S2][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x38)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x3c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_d27c)?;
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S4][0];
                OC420_BINDING_HAS_PROCESS.call(c, 0x0027_d28c)?;
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if found {
                    break;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_d268);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_ParseHeader (bxSphereTree_Load).

/// `MEM_copy(dst, src, n)` (0x3e6574) through the function table.
fn oc420_mem_copy(c: &mut Cpu, dst: u32, src: u32, n: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = sx(dst);
    c.g[A1][0] = sx(src);
    c.g[A2][0] = n as u64;
    call_ptr(c, 0x003e_6574, ret)
}

/// `int cPS2ChunkResolve_ParseHeader(Tree* self, char* buf, int treeId)`
/// (0x327da8; the reference calls it `bxSphereTree_Load`): reads the sphere
/// tree header at `buf` into `self->blk` (the block at `self + 0xc0`, whose
/// address is also kept at `self + 0x98`): copies the counts, the bounds
/// and the two 36-byte tables, points the node and data arrays behind the
/// header, caches the centre as a quad (w = 1.0) at `self + 0x80`. Returns
/// the number of bytes consumed (0 when `buf` is null).
pub fn cps2_chunk_resolve_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = 0;
        if c.g[S1][0] != 0 {
            c.cycles += 61;
            let this = c.g[S0][0] as u32;
            let buf = c.g[S1][0] as u32;
            c.g[V0][0] = sx(this.wrapping_add(0xc0));
            c.g[V1][0] = 1;
            Addr(this.wrapping_add(0xc0)).sw(c, c.g[A2][0]);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            Addr(this.wrapping_add(0xc4)).sw(c, 0);
            c.g[A1][0] = c.g[S1][0];
            Addr(this.wrapping_add(0x98)).sw(c, c.g[V0][0]);
            c.g[A2][0] = 4;
            Addr(this.wrapping_add(0xd0)).sw(c, c.g[V1][0]);
            call_ptr(c, 0x003e_6574, 0x0032_7df0)?;
            // len0 -> sp+0x10, len1 -> sp+0x14.
            oc420_mem_copy(
                c,
                sp(c).wrapping_add(0x14),
                buf.wrapping_add(4),
                4,
                0x0032_7e00,
            )?;
            let mut blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(c, blk.wrapping_add(8), buf.wrapping_add(8), 4, 0x0032_7e14)?;
            blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(
                c,
                blk.wrapping_add(0xc),
                buf.wrapping_add(0xc),
                4,
                0x0032_7e28,
            )?;
            blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(
                c,
                blk.wrapping_add(0x14),
                buf.wrapping_add(0x10),
                0xc,
                0x0032_7e3c,
            )?;
            blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(
                c,
                blk.wrapping_add(0x2c),
                buf.wrapping_add(0x1c),
                0xc,
                0x0032_7e50,
            )?;
            blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(
                c,
                blk.wrapping_add(0x38),
                buf.wrapping_add(0x28),
                0x24,
                0x0032_7e64,
            )?;
            blk = Addr(this.wrapping_add(0x98)).lw(c) as u32;
            oc420_mem_copy(
                c,
                blk.wrapping_add(0x5c),
                buf.wrapping_add(0x4c),
                0x24,
                0x0032_7e78,
            )?;
            c.g[V0][0] = Addr(this.wrapping_add(0x98)).lw(c);
            c.g[A1][0] = sx(buf.wrapping_add(0x70));
            c.g[A0][0] = 0xc;
            Addr((c.g[V0][0] as u32).wrapping_add(0x20)).sw(c, c.g[A1][0]);
            c.g[V1][0] = Addr(this.wrapping_add(0x98)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.mult(0, A2, V0, A0);
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            Addr((c.g[V1][0] as u32).wrapping_add(0x24)).sw(c, c.g[A1][0]);
            c.g[A2][0] = Addr(this.wrapping_add(0x98)).lw(c);
            c.g[V0][0] = Addr(sp(c).wrapping_add(0x14)).lw(c);
            c.g[A0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            // Leaf tree (+8 == 0): the data pointer at +0x28 is the node
            // pointer at +0x24; otherwise null.
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x24)).lw(c);
                Addr((c.g[A2][0] as u32).wrapping_add(0x28)).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 2;
                Addr((c.g[A2][0] as u32).wrapping_add(0x28)).sw(c, 0);
            }
            c.cycles += 13;
            c.g[V1][0] = Addr(this.wrapping_add(0x98)).lw(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            let blk = c.g[V1][0] as u32;
            c.lwc1(0, blk.wrapping_add(0x14));
            c.swc1(0, sp(c));
            c.lwc1(1, blk.wrapping_add(0x18));
            c.swc1(1, sp(c).wrapping_add(4));
            c.lwc1(0, blk.wrapping_add(0x1c));
            c.swc1(2, sp(c).wrapping_add(0xc));
            c.swc1(0, sp(c).wrapping_add(8));
            c.lq(3, sp(c));
            c.sq(3, this.wrapping_add(0x80));
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared pieces of the WScriptFn_* argument decoders.

/// The argument-copy loop of the `WScriptFn_*` decoders: `a1` entries of 16
/// bytes `{index, value, _, kind}` at `a2` are written into the word buffer
/// at `buf_reg` (`buf[index] = value`, converted to float when the entry's
/// kind differs from the kind table's (`t0`) and that one is 2). `a3` is 2.
fn oc420_param_loop(c: &mut Cpu, buf_reg: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[buf_reg][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let mut convert = false;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
            } else {
                convert = true;
            }
        }
        if convert {
            c.cycles += 4;
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
            c.cvt_s_w(0, 0);
            c.swc1(0, c.g[A0][0] as u32);
        } else {
            c.cycles += 1;
            Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The tail the `WScriptFn_*` decoders share: copy the 12-byte default
/// result block at `0x4c9098` into the caller's return slot (`s3`, unaligned
/// `ldl/ldr/sdl/sdr` plus a word) and return it in `v0`. `v1` is 0x4d0000.
fn oc420_copy_result_block(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(8)).lw(c);
    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[S3][0] as u32);
    Addr((c.g[S3][0] as u32).wrapping_add(8)).sw(c, c.g[A1][0]);
}

// ---------------------------------------------------------------------------
// WScriptFn_RandRange.

/// `float Rand_FloatRange(float min, float max)` (0x317830).
const OC420_RAND_FLOAT_RANGE: Callee = Callee {
    addr: 0x0031_7830,
    f: ee::s10::f_00317830,
};

/// `void* WScriptFn_RandRange(Ctx* self, int n, Arg* args)` (0x303598): a
/// script function filling a 3-float parameter block (`{min, max, ..}`,
/// from a lazily initialised static at 0x4fbc60: 0, 1.0, flag byte) with
/// `n` 16-byte `{index, value, _, kind}` arguments (the value is converted
/// to float when the argument kind differs from the parameter's table kind
/// and that kind is 2), then stores a random float in `[min, max)` into the
/// result register of `self` (`self[2] = 2`, `LunoVMRegister_AsFloat`).
/// Returns `self`.
pub fn wscript_fn_rand_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_fpr(c, 20, 0x70);
        if !initialised {
            // First use: `static Params p = {0, 1.0f, flag}` (the guard
            // and the trailing byte copy included).
            c.cycles += 10;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x43a0));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lbu(c);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x43a0)).sw(c, 0);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sb(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).sw(c, c.g[A0][0]);
            Addr(sp(c)).sb(c, c.g[V1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x43a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3608)?;
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S2][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc30));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_3620);
        }
        c.cycles += 18;
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(4));
        c.lwc1(12, c.g[S0][0] as u32);
        OC420_RAND_FLOAT_RANGE.call(c, 0x0030_3674)?;
        c.g[V0][0] = 2;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_6618, 0x0030_3688)?;
        c.swc1(20, c.g[V0][0] as u32);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_StopObjectLoop.

/// `SSXAudioSystem_GetInstance` (0x28b180) and
/// `SSXAudioSystem_StopObjectLoop` (0x297eb8).
const OC420_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
const OC420_AUDIO_STOP_OBJECT_LOOP: u32 = 0x0029_7eb8;

/// `Result WScriptFn_StopObjectLoop(Result* out, int n, Arg* args)`
/// (0x300260): decodes `n` arguments into `{object id, sound, loop}`
/// (static defaults at 0x4fbb28: `-1` objects, lazily initialised), resolves
/// the object (id `-1` is the current object `*(gp[0xce8] + 0x290)`;
/// otherwise through the world cache) and, if there is one, stops the
/// sound's loop on it. Always copies the 12-byte default result into `*out`.
pub fn wscript_fn_stop_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).lw(c);
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, -1, flag}`.
            c.cycles += 12;
            c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44d8));
            c.g[V0][0] = u64::MAX;
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lbu(c);
            Addr(0x0050_0000u32.wrapping_sub(0x44d8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
            Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[A0][0]);
            Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).sw(c, c.g[A1][0]);
            Addr(sp(c)).sb(c, c.g[A2][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x10;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x44d8));
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_02e0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x6560));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_02f8);
        }
        // The object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S2][0] = c.g[A1][0];
        }
        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[S2][0] != 0 {
            c.cycles += 9;
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
            c.g[S0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            call_ptr(c, OC420_AUDIO_GET_INSTANCE, 0x0030_03c0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, OC420_AUDIO_STOP_OBJECT_LOOP, 0x0030_03d4)?;
            c.g[V1][0] = 0x004d_0000;
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddParentModifierRev.

/// `cMoveNode_addParent` (0x355978).
const OC420_MOVE_NODE_ADD_PARENT: u32 = 0x0035_5978;

/// `Result WScriptFn_AddParentModifierRev(Result* out, int n, Arg* args)`
/// (0x2fdc60): decodes `n` arguments into `{child id, parent id, flags,
/// x, y, z}` (static defaults at 0x4fb758), resolves both objects (`-1` child
/// = the current object), and when the child's move node is not already of
/// kind 6 (`vtable+0x50`, `v10(6)`) and the parent's move node answers
/// `vtable+0x80`, adds the parent to its move node with the offset vector.
/// Copies the default result block into `*out`.
pub fn wscript_fn_add_parent_modifier_rev(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).lw(c);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, 0, 0.0, 0.0, 0.0, flag}`.
            c.cycles += 15;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x48a8));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lbu(c);
            c.mtc1(0, 0);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x48a8)).sw(c, c.g[V1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(0x18)).sb(c, c.g[A1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
            Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).sw(c, c.g[A0][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
            Addr(sp(c)).sb(c, c.g[A1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x48a8));
        c.g[A2][0] = 0x1c;
        Addr(sp(c).wrapping_add(0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x002f_dcf0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x61c0));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S1, 0x002f_dd08);
        }
        // The child object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let record = c.g[V1][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if record != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The parent: only when buf[1] is not -1.
        c.cycles += 4;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
        c.g[V0][0] = !c.g[V0][0];
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0x004d_0000;
        } else {
            'done: {
                c.cycles += 3;
                c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 6;
                if c.g[S2][0] != 0 {
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dde0)?;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[V0][0] != 0 {
                        break 'done;
                    }
                    // Parent object through the world cache.
                    c.cycles += 10;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let record = c.g[V1][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    if record != 0 {
                        c.cycles += 7;
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let present = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !present {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[S0][0] == 0 {
                        break 'done;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_de58)?;
                    let answered = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if answered {
                        // Add the parent with the offset vector (x, y, z, 0).
                        c.cycles += 11;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A3][0] = sx(sp(c).wrapping_add(0x30));
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x18)).lw(c);
                        c.swc1(2, sp(c).wrapping_add(0x30));
                        c.swc1(0, sp(c).wrapping_add(0x34));
                        c.swc1(1, sp(c).wrapping_add(0x38));
                        Addr(sp(c).wrapping_add(0x3c)).sw(c, 0);
                        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
                        call_ptr(c, OC420_MOVE_NODE_ADD_PARENT, 0x002f_de8c)?;
                    }
                }
                // (also the exit when the child has no move node, or the
                // parent has none / does not answer)
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_DetachFromScript.

/// `Rider_Reset` (0x11d390) and `Rider_GetCharacter` (0x123128).
const OC420_RIDER_RESET: Callee = Callee {
    addr: 0x0011_d390,
    f: ee::s00::f_0011d390,
};
const OC420_RIDER_GET_CHARACTER: Callee = Callee {
    addr: 0x0012_3128,
    f: ee::s00::f_00123128,
};

/// `bool Rider_DetachFromScript(Rider* self)` (0x123b48): the script lets
/// go of the rider. Deactivates the script object (+0x6d0), resets the
/// rider, tells the audio system to drop its handle when `+0xac8` is set,
/// clears the script bookkeeping (+0xac4..+0xb04), shows the standard rider
/// model parts again (hide parts 5, 6, 8, 9, 11 and show 4, 7; the bolt parts
/// of the character's bolt list are shown through the profile when the script
/// object's kind byte (+6) is set), and finally stops the four sound handles
/// at +0xb08 (set to -1). Returns 1 (`v0` from the first stage).
pub fn rider_detach_from_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(0x6d0));
        call_ptr(c, 0x0028_03f0, 0x0012_3b74)?;
        c.g[A0][0] = c.g[S2][0];
        OC420_RIDER_RESET.call(c, 0x0012_3b7c)?;
        c.g[V0][0] = Addr(this.wrapping_add(0xac8)).lw(c);
        let sound = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if sound {
            c.cycles += 5;
            call_ptr(c, 0x0028_b180, 0x0012_3b90)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002a_1bd0, 0x0012_3b98)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 39;
        c.g[A0][0] = Addr(this.wrapping_add(0x780)).lw(c);
        Addr(this.wrapping_add(0xafc)).sw(c, 0);
        c.g[A1][0] = 5;
        Addr(this.wrapping_add(0xb04)).sw(c, 0);
        c.g[A2][0] = 0;
        Addr(this.wrapping_add(0xac4)).sw(c, 0);
        c.g[S5][0] = sx(this.wrapping_add(0xb08));
        Addr(this.wrapping_add(0xad0)).sw(c, 0);
        Addr(this.wrapping_add(0xb00)).sw(c, c.g[V0][0]);
        // Hide the parts 5, 6, 8, 9, 11; show 4 and 7.
        call_ptr(c, 0x0030_ea80, 0x0012_3bc4)?;
        for (part, ret) in [
            (6u64, 0x0012_3bd4u32),
            (8, 0x0012_3be4),
            (9, 0x0012_3bf4),
            (0xb, 0x0012_3c04),
        ] {
            c.g[A0][0] = Addr(this.wrapping_add(0x780)).lw(c);
            c.g[A1][0] = part;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0030_ea80, ret)?;
        }
        for (part, ret) in [(4u64, 0x0012_3c14u32), (7, 0x0012_3c24)] {
            c.g[A0][0] = Addr(this.wrapping_add(0x780)).lw(c);
            c.g[A1][0] = part;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0030_e9e0, ret)?;
        }
        c.g[A0][0] = Addr(this.wrapping_add(0x6d8)).lw(c);
        call_ptr(c, 0x0027_c958, 0x0012_3c2c)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(6)).lb(c);
        c.g[S0][0] = c.g[S5][0];
        if c.g[V1][0] != 0 {
            // Show the bolt parts the character's profile has unlocked.
            c.cycles += 24;
            call_ptr(c, 0x0014_dc80, 0x0012_3c40)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0014_dd58, 0x0012_3c4c)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_bdb8, 0x0012_3c54)?;
            c.g[A1][0] = Addr(this.wrapping_add(0x86c)).lw(c);
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_ad50, 0x0012_3c64)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[S4][0] = c.g[V0][0];
            OC420_RIDER_GET_CHARACTER.call(c, 0x0012_3c70)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0014_d988, 0x0012_3c80)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0014_d998, 0x0012_3c90)?;
            c.g[S1][0] = c.g[V0][0];
            if (c.g[S3][0] as i64) > 0 {
                loop {
                    c.cycles += 3;
                    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lb(c);
                    if (c.g[S0][0] as i64) < 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x34)).lw(c);
                        c.g[V0][0] &= 0x40;
                        if c.g[V0][0] == 0 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                        } else {
                            // The bolt's record in the profile: short table
                            // at +0x288 maps the bolt's slot to a record at
                            // +0x290 (-1: none).
                            c.cycles += 7;
                            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lh(c);
                            c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x288)).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
                            let missing = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if missing {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[V0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            }
                            c.cycles += 4;
                            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lhu(c);
                            c.g[V0][0] &= 0x10;
                            c.g[A1][0] = c.g[S0][0];
                            if c.g[V0][0] != 0 {
                                c.cycles += 15;
                                for (mode, ret) in [
                                    (0u64, 0x0012_3cfcu32),
                                    (1, 0x0012_3d0c),
                                    (2, 0x0012_3d1c),
                                    (3, 0x0012_3d2c),
                                ] {
                                    c.g[A0][0] = Addr(this.wrapping_add(0x780)).lw(c);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A2][0] = mode;
                                    call_ptr(c, 0x0030_e9e0, ret)?;
                                }
                            }
                            c.cycles += 1;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                        }
                    }
                    c.cycles += 2;
                    let more = c.g[S3][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_3c98);
                    }
                }
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[S5][0];
        }
        // Stop the four sound handles at +0xb08.
        c.cycles += 3;
        c.g[S2][0] = u64::MAX;
        c.g[S1][0] = 3;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let unused = (c.g[V0][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            if !unused {
                c.cycles += 9;
                call_ptr(c, 0x0028_b180, 0x0012_3d5c)?;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x118)).lw(c);
                c.g[A2][0] = 1;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.mtc1(0, 12);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                call_ptr(c, 0x002a_d5f0, 0x0012_3d78)?;
            }
            c.cycles += 3;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S2][0]);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_3d48);
            }
        }
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
// Rider_AddBoltFilesToLoadList.

/// Adds `name` (in `name_reg`) to the rider's sorted-later name table at
/// `table_reg` (`+0x2940` start, `+0x2944` count; 0x108-byte entries: 0x100
/// bytes of name, then two words) unless `strcmp` finds it among the current
/// entries. Returns whether it was already there. `a0_from_s6`: the first
/// loop keeps `a0 = s6` (the next index) after each compare.
fn oc420_bolt_name_add(
    c: &mut Cpu,
    name_reg: usize,
    table_reg: usize,
    cmp_ret: u32,
    cpy_ret: u32,
    head: u32,
    a0_from_s6: bool,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, table_reg).wrapping_add(0x2940)).lw(c);
    c.g[A0][0] = Addr(r32(c, table_reg).wrapping_add(0x2944)).lw(c);
    c.g[S1][0] = c.g[V0][0];
    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
    let empty = c.g[V1][0] == 0;
    c.g[V0][0] = 0x108;
    if !empty {
        c.cycles += 2;
        c.mult(0, V0, S1, V0);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, table_reg)));
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[name_reg][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0041_65a8, cmp_ret)?;
            let found = c.g[V0][0] == 0;
            if a0_from_s6 {
                c.g[A0][0] = c.g[S6][0];
            }
            if found {
                return Ok(true);
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let again = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x108));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    // Append: copy the name into the entry after the last one.
    c.cycles += 12;
    c.g[V0][0] = 0x108;
    c.g[A1][0] = c.g[name_reg][0];
    c.mult(0, V0, S2, V0);
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, table_reg)));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0041_66f4, cpy_ret)?;
    c.g[T0][0] = u64::MAX;
    Addr((c.g[S0][0] as u32).wrapping_add(0x104)).sw(c, 0);
    Addr((c.g[S0][0] as u32).wrapping_add(0x100)).sw(c, c.g[T0][0]);
    c.g[V0][0] = Addr(r32(c, table_reg).wrapping_add(0x2944)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(r32(c, table_reg).wrapping_add(0x2944)).sw(c, c.g[V0][0]);
    Ok(false)
}

/// `void Rider_AddBoltFilesToLoadList(Rider* self, int* list, int n)`
/// (0x11be88): appends the bolt animation file names of `list[0..n]` to the
/// rider's load list (`self+0x894`, 0x108-byte name entries, no duplicates)
/// and sorts the new part with `qsort`/`Rider_LoadListCompare`. For a cheat
/// character the names come from each entry's `+0x2c` field; otherwise from
/// `BEBoltInt_GetBoltTextureName` into a stack buffer.
pub fn rider_add_bolt_files_to_load_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x1b0);
        frame.save(c, S5, 0x150);
        frame.save(c, S7, 0x130);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, FP, 0x120);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, S0, 0x1a0);
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S6, 0x140);
        frame.save_ra(c, 0x110);
        let this = c.g[S5][0] as u32;
        c.g[FP][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0011_bec4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, 0x0014_dd58, 0x0011_bed0)?;
        Addr(sp(c).wrapping_add(0x100)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this.wrapping_add(0x86c)).lw(c);
        call_ptr(c, 0x0014_ac30, 0x0011_bedc)?;
        Addr(sp(c).wrapping_add(0x104)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this.wrapping_add(0x86c)).lw(c);
        call_ptr(c, 0x0014_a0b0, 0x0011_bee8)?;
        let cheat = c.g[V0][0] != 0;
        // Which exit the loops lead to: 0 = the `+2` one (cheat loop ended),
        // 1 = the shared `+1` one.
        let mut plain_exit = true;
        if cheat {
            c.cycles += 2;
            let none = (c.g[S7][0] as i64) <= 0;
            c.g[A0][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                loop {
                    c.cycles += 6;
                    c.g[S6][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
                    c.g[S4][0] = Addr(this.wrapping_add(0x894)).lw(c);
                    let mut tail_plus_one = true;
                    if c.g[S3][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S3][0] as u32).lb(c);
                        c.g[A0][0] = c.g[S6][0];
                        if c.g[V0][0] == 0 {
                            tail_plus_one = false;
                        } else if oc420_bolt_name_add(
                            c,
                            S3,
                            S4,
                            0x0011_bf54,
                            0x0011_bf84,
                            0x0011_bf48,
                            true,
                        )? {
                            tail_plus_one = false;
                        }
                    }
                    if tail_plus_one {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S6][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    if !more {
                        c.cycles += 2;
                        c.g[A0][0] = Addr(this.wrapping_add(0x894)).lw(c);
                        plain_exit = false;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_bf00);
                    }
                }
            }
        } else {
            c.g[A0][0] = Addr(this.wrapping_add(0x86c)).lw(c);
            c.cycles += 4;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0014_ac50, 0x0011_bfbc)?;
            let none = (c.g[S7][0] as i64) <= 0;
            Addr(sp(c).wrapping_add(0x108)).sw(c, c.g[V0][0]);
            if !none {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                loop {
                    c.cycles += 10;
                    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[A1][0] = Addr(sp(c).wrapping_add(0x108)).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[T0][0] = c.g[SP][0];
                    c.g[A2][0] = Addr(sp(c).wrapping_add(0x104)).lw(c);
                    c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_b988, 0x0011_bfe8)?;
                    let got = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp(c)).lb(c);
                    let mut tail_plus_one = true;
                    if got {
                        c.cycles += 3;
                        c.g[S4][0] = c.g[SP][0];
                        let empty = c.g[V0][0] == 0;
                        c.g[S3][0] = Addr(this.wrapping_add(0x894)).lw(c);
                        if !empty {
                            if oc420_bolt_name_add(
                                c,
                                S4,
                                S3,
                                0x0011_c02c,
                                0x0011_c05c,
                                0x0011_c020,
                                false,
                            )? {
                                tail_plus_one = false;
                            }
                        }
                    }
                    if tail_plus_one {
                        c.cycles += 1;
                    }
                    c.g[S0][0] = c.g[S6][0];
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_bfc8);
                    }
                }
            }
        }
        if plain_exit {
            c.cycles += 1;
            c.g[A0][0] = Addr(this.wrapping_add(0x894)).lw(c);
        }
        // Sort the entries added by this call.
        c.cycles += 21;
        c.g[V0][0] = 0x0012_0000;
        c.g[A3][0] = sx(0x0012_0000u32.wrapping_sub(0x4988));
        c.g[V1][0] = 0x108;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2940)).lw(c);
        c.g[A2][0] = 0x108;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2944)).lw(c);
        c.mult(0, T0, V0, V1);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0041_8ef8, 0x0011_c0ac)?;
        frame.restore(c, S0, 0x1a0);
        frame.restore(c, S1, 0x190);
        frame.restore(c, S2, 0x180);
        frame.restore(c, S3, 0x170);
        frame.restore(c, S4, 0x160);
        frame.restore(c, S5, 0x150);
        frame.restore(c, S6, 0x140);
        frame.restore(c, S7, 0x130);
        frame.restore(c, FP, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_ctor.

/// `sw` of the low word of `v` at `addr`.
fn oc420_sw(c: &mut Cpu, addr: u32, v: u64) {
    Addr(addr).sw(c, v);
}

/// `Rider* Rider_ctor(Rider* self)` (0x11b698): constructs a rider object:
/// the rider state base, the script-object vtables, the default indices
/// (-1) and flags, the named allocations (track/AI path hints, overlap list,
/// attached emitter FX, bolt-on FX, line/wall hints, board shadow), the
/// cleared script/boost fields and the boost defaults (mode 1, source mask
/// 31, max speed 4000.0, cheat steer 0.18, speed lerp 0.015). Returns `self`.
pub fn rider_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 130;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        c.g[S2][0] = c.g[A0][0];
        let this = c.g[S2][0] as u32;
        let at = |off: u32| this.wrapping_add(off);
        // cRiderState::cRiderState
        call_ptr(c, 0x0014_0738, 0x0011_b6b8)?;
        c.g[S0][0] |= 0xffff;
        c.g[V0][0] = 0x0046_0000;
        c.g[A0][0] = sx(at(0x6d0));
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x6338));
        c.g[S1][0] = 1;
        oc420_sw(c, at(0x6c0), c.g[V0][0]);
        // cScriptObjRider::cScriptObjRider (the script-object part at +0x6d0)
        call_ptr(c, 0x0027_f968, 0x0011_b6d4)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 0x0046_0000;
        c.g[A0][0] = 0x0046_0000;
        c.g[T0][0] = u64::MAX;
        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x6430));
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x6470));
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x63e8));
        c.g[A2][0] = 0x31;
        c.g[T1][0] = 0x004a_0000;
        oc420_sw(c, at(0x6e8), c.g[V1][0]); // script object vtable
        oc420_sw(c, at(0x888), c.g[T0][0]); // safe render tick
        c.g[A1][0] = 0x0045_0000;
        oc420_sw(c, at(0x86c), c.g[T0][0]); // AI index
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x78a8));
        oc420_sw(c, at(0x870), c.g[T0][0]); // view index
        c.g[A3][0] = 0;
        oc420_sw(c, at(0x6c0), c.g[A0][0]); // rider vtable
        oc420_sw(c, at(0x434), c.g[A2][0]); // ground material
        c.g[A0][0] = 0x20;
        oc420_sw(c, at(0x6d0), c.g[V0][0]); // script-object-rider vtable
        oc420_sw(c, at(0x880), 0);
        oc420_sw(c, at(0x884), c.g[S1][0]);
        oc420_sw(c, at(0x874), 0);
        oc420_sw(c, at(0x878), 0);
        oc420_sw(c, at(0x87c), c.g[S1][0]);
        oc420_sw(c, at(0x784), 0);
        oc420_sw(c, at(0x780), 0);
        oc420_sw(c, at(0xaa0), 0);
        oc420_sw(c, at(0x3fc), 0);
        oc420_sw(c, at(0x430), c.g[S0][0]); // ground poly id
                                            // The 8-byte key at 0x4a5960 -> self + 0xaac.
        c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5960));
        c.ldl(2, (c.g[T2][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[T2][0] as u32);
        c.sdl(2, at(0xab3));
        c.sdr(2, at(0xaac));
        c.g[A2][0] = 0;
        // "Track path intersect info"
        call_ptr(c, 0x0031_7d70, 0x0011_b768)?;
        // "AI path intersect info"
        c.g[A1][0] = 0x0045_0000;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x78c8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        oc420_sw(c, at(0xac0), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0011_b784)?;
        // "OverlapList" (0xa14 bytes), the Nash grid of the game in its first word.
        c.g[A1][0] = 0x0045_0000;
        c.g[A0][0] = 0xa14;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x78e0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        oc420_sw(c, at(0xabc), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0011_b7a0)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 0x0045_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x78f0));
        c.g[A0][0] = 0xa8;
        c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x20)).lw(c);
        let list = c.g[V0][0] as u32;
        oc420_sw(c, list.wrapping_add(0x210), c.g[S0][0]);
        oc420_sw(c, list, c.g[V1][0]);
        oc420_sw(c, at(0x860), c.g[V0][0]);
        oc420_sw(c, list.wrapping_add(4), 0);
        oc420_sw(c, list.wrapping_add(8), c.g[S0][0]);
        oc420_sw(c, list.wrapping_add(0x10c), c.g[S0][0]);
        // "AttachedEmitterFX" (168 bytes) and its constructor.
        call_ptr(c, 0x0031_7d70, 0x0011_b7dc)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002f_1850, 0x0011_b7e8)?;
        // "cBoltOnFXInterface" (200 bytes).
        c.g[A1][0] = 0x0045_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x7908));
        c.g[A0][0] = 0xc8;
        c.g[A2][0] = 0;
        oc420_sw(c, at(0x88c), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0011_b804)?;
        c.g[A2][0] = Addr(at(0x88c)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_c9e0, 0x0011_b814)?;
        // "Line Intersect Hint" and "Wall Intersect Hint" (0x10 bytes each).
        c.g[A1][0] = 0x0045_0000;
        c.g[A0][0] = 0x10;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x7920));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        oc420_sw(c, at(0x890), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0011_b830)?;
        c.g[A1][0] = 0x0045_0000;
        c.g[A0][0] = 0x10;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x7938));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        oc420_sw(c, at(0x864), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0011_b84c)?;
        oc420_sw(c, at(0x868), c.g[V0][0]);
        // MEM_fill(self + 0x794, 0, 0x20)
        c.g[A0][0] = sx(at(0x794));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20;
        call_ptr(c, 0x003e_6448, 0x0011_b860)?;
        c.por(2, 0, 0);
        oc420_sw(c, at(0xb00), c.g[S1][0]);
        c.sq(2, at(0xae0));
        c.g[A0][0] = u64::MAX;
        oc420_sw(c, at(0x78c), 0);
        c.g[V1][0] = 3;
        oc420_sw(c, at(0x5b0), 0);
        c.g[V0][0] = sx(at(0xb14));
        oc420_sw(c, at(0xac4), 0);
        oc420_sw(c, at(0xac8), 0);
        oc420_sw(c, at(0xad0), 0);
        oc420_sw(c, at(0xaf0), 0);
        oc420_sw(c, at(0xaf4), 0);
        oc420_sw(c, at(0xaf8), 0);
        oc420_sw(c, at(0xafc), 0);
        oc420_sw(c, at(0xb04), 0);
        // The four sound handles at +0xb08 = -1.
        loop {
            c.cycles += 7;
            oc420_sw(c, c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_b8a0);
            }
        }
        c.cycles += 47;
        // "BoaderShadow" (0x1a0 bytes) and its constructor.
        c.g[A1][0] = 0x0045_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0045_0000u32.wrapping_add(0x7950));
        c.g[A0][0] = 0x1a0;
        c.g[S0][0] = u64::MAX;
        call_ptr(c, 0x0031_7d70, 0x0011_b8d8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0037_58f8, 0x0011_b8e0)?;
        oc420_sw(c, at(0x450), c.g[S0][0]);
        c.g[A0][0] = sx(at(0x6dc));
        oc420_sw(c, at(0xaa4), 0);
        oc420_sw(c, at(0xb18), 0);
        oc420_sw(c, at(0xb1c), 0);
        oc420_sw(c, at(0xaa8), c.g[V0][0]);
        // ScriptObject_InitScriptObject(self + 0x6dc)
        call_ptr(c, 0x0028_3298, 0x0011_b8fc)?;
        oc420_sw(c, at(0x778), c.g[S0][0]);
        oc420_sw(c, at(0x774), c.g[S0][0]);
        // cSSXAudio_RiderCreated(audio instance, self)
        call_ptr(c, 0x0028_b180, 0x0011_b908)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0028_9af0, 0x0011_b914)?;
        c.g[AT][0] = 0x457a_0000;
        c.mtc1(1, 2);
        c.g[V0][0] = 0x1f;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x79bc));
        c.g[A0][0] = 1;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x79b8));
        c.g[V1][0] = 3;
        oc420_sw(c, at(0xb28), c.g[V0][0]); // boost source mask
        c.g[V0][0] = c.g[S2][0];
        oc420_sw(c, at(0xb2c), c.g[V1][0]); // uber flags
        oc420_sw(c, at(0x308), c.g[A0][0]);
        c.swc1(2, at(0x30c)); // boost max speed 4000.0
        c.swc1(1, at(0x310)); // cheat steer align
        c.swc1(0, at(0x314)); // cheat speed lerp
        oc420_sw(c, at(0xb24), 0);
        oc420_sw(c, at(0x304), c.g[A0][0]);
        oc420_sw(c, at(0xb20), c.g[A0][0]);
        oc420_sw(c, at(0xb30), 0);
        oc420_sw(c, at(0xb34), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// tWScriptCache_Relocate.

/// `tWScriptCache_Relocate`'s inline record lookup (the "resolve" the C++
/// reference calls): the id in `t0` is `slot | index << 8`; `*(s1)` is the
/// world cache whose slot table (+8) holds records; the record's table at
/// `table_off` maps `index` to `value << 8`; the result (`value << 2`, 0 when
/// absent) lands in `a2`. `head` is the cycle charge of the first block;
/// `load_cache` is whether that block also loads the cache pointer into `a2`.
/// `a3` is the running index.
fn oc420_cache_record(c: &mut Cpu, head: u64, table_off: u32, load_cache: bool) {
    c.cycles += head;
    if load_cache {
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    }
    c.g[A1][0] = c.g[T0][0] & 0xff;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 8);
    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let present = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
    if !present {
        c.cycles += 2;
        c.g[A2][0] = 0;
    } else {
        c.cycles += 8;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(table_off)).lw(c);
        c.g[V1][0] = sx((c.g[T0][0] as u32) >> 8);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[V0][0];
    }
}

/// `*(reg + off) += reg` where `reg` (`v1` or `a0`) holds the script data
/// base (reloaded from `entry + 8` first when `reload`): turns an offset
/// into a pointer.
fn oc420_rebase(c: &mut Cpu, entry: u32, reg: usize, off: u32, reload: bool) {
    if reload {
        c.g[reg][0] = Addr(entry.wrapping_add(8)).lw(c);
    }
    let base = c.g[reg][0] as u32;
    c.g[V0][0] = Addr(base.wrapping_add(off)).lw(c);
    c.g[V0][0] = sx(base.wrapping_add(c.g[V0][0] as u32));
    Addr(base.wrapping_add(off)).sw(c, c.g[V0][0]);
}

/// `void tWScriptCache_Relocate(Entry* e, World** wp, int index)`
/// (0x3accd8): turns a freshly loaded world script block (`e + 8`, header
/// `0x1000` and the world id at +0xc) from file offsets into pointers: the
/// ten table pointers, the entries of the +0x3c and +0x50 tables, then
/// records the script data refers to are resolved in the world cache (the
/// track records at +0x88/+0x8 for the +0x44 count, the type 1/3 table entries
/// at +0xc for the +0x4c count, the spline records at +0x28/+0x1c for the +0x54
/// count) and the two tables at +0x24/+0x20 and +0x2c/+0x28 are relocated.
/// A block with the wrong header or world is dropped (freed when the entry
/// owns it); the entry's state word becomes 3 either way.
pub fn t_wscript_cache_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let entry = c.g[S0][0] as u32;
        c.g[A3][0] = Addr(entry.wrapping_add(8)).lw(c);
        c.g[S2][0] = c.g[A2][0];
        'finish: {
            'done: {
                if c.g[A3][0] == 0 {
                    break 'done;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A3][0] as u32).lhu(c);
                c.g[V0][0] = 0x1000;
                let mut keep = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                    if c.g[A1][0] == c.g[V1][0] {
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        keep = true;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(entry.wrapping_add(2)).lh(c);
                    }
                } else {
                    c.g[V0][0] = Addr(entry.wrapping_add(2)).lh(c);
                }
                if !keep {
                    // Wrong block: free it when the entry owns it, and forget it.
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        Addr(entry.wrapping_add(8)).sw(c, 0);
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A3][0];
                        call_ptr(c, 0x0031_7e98, 0x003a_cd3c)?;
                        Addr(entry.wrapping_add(8)).sw(c, 0);
                    }
                    c.cycles += 1;
                    c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                }
                c.cycles += 2;
                let none = c.g[V1][0] == 0;
                c.g[V0][0] = 3;
                if none {
                    break 'finish;
                }
                // The ten table pointers.
                c.cycles += 44;
                c.g[A1][0] = 0;
                for (reg, off, reload) in [
                    (V1, 0x14u32, false),
                    (V1, 0x1c, true),
                    (A0, 0x24, true),
                    (V1, 0x2c, true),
                    (A0, 0x34, true),
                    (V1, 0x3c, true),
                    (A0, 0x40, true),
                    (V1, 0x48, true),
                    (A0, 0x50, true),
                    (V1, 0x58, true),
                ] {
                    oc420_rebase(c, entry, reg, off, reload);
                }
                c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x38)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x3c)).lw(c);
                if !empty {
                    // Entries of the +0x3c table (count at +0x38).
                    c.cycles += 1;
                    loop {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x38)).lw(c);
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                        if !more {
                            c.cycles += 1;
                            c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce00);
                        }
                    }
                }
                // Entries of the +0x50 table (count at +0x4c) get the +0x40 base.
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4c)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A2][0] = 0;
                if !empty {
                    c.cycles += 1;
                    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
                    loop {
                        c.cycles += 12;
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x40)).lw(c);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).sw(c, c.g[A0][0]);
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4c)).lw(c);
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce38);
                        }
                    }
                }
                // Records of the world cache for the +0x44 track entries.
                c.cycles += 7;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[T0][0] = 0xffff_ffff_ffff_0000;
                c.g[A3][0] = 0;
                c.g[T0][0] |= 0xff00;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[T0][0] |= c.g[S2][0];
                if !empty {
                    c.cycles += 3;
                    c.g[T1][0] = 0xffff_ffff_ffff_0000;
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                    loop {
                        oc420_cache_record(c, 8, 0x1c, false);
                        c.cycles += 22;
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32) << 1);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lw(c);
                        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        Addr((c.g[A2][0] as u32).wrapping_add(0x88)).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
                        c.g[V0][0] &= c.g[T1][0];
                        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V0][0] |= c.g[V1][0];
                        c.g[V0][0] |= 2;
                        Addr((c.g[A2][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x44)).lw(c);
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce90);
                        }
                    }
                }
                // Table entries of type 1 and 3 point at records (+0xc), others at 0.
                c.cycles += 4;
                c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                if !empty {
                    c.cycles += 4;
                    c.g[T2][0] = 1;
                    c.g[T1][0] = 3;
                    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lw(c);
                    loop {
                        c.cycles += 6;
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                        let mut resolve = true;
                        if c.g[V0][0] == c.g[T2][0] {
                            c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T1][0] {
                                Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, 0);
                                resolve = false;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                            }
                        }
                        if resolve {
                            c.cycles += 8;
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            c.g[V1][0] = c.g[A1][0] & 0xff;
                            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            let present = c.g[V1][0] != 0;
                            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                            let mut found = false;
                            if present {
                                c.cycles += 7;
                                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64)).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                found = c.g[V0][0] != 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            }
                            if !found {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 1;
                            Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 6;
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_cf58);
                        }
                    }
                }
                // Spline records (+0x54 of them): two halfwords each from +0x58.
                c.cycles += 4;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[T1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
                let empty = (c.g[T1][0] as i64) <= 0;
                c.g[A3][0] = 0;
                if !empty {
                    c.cycles += 2;
                    c.g[T2][0] = 0xffff_ffff_ffff_0000;
                    loop {
                        oc420_cache_record(c, 9, 0x44, true);
                        c.cycles += 21;
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
                        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.lwl(3, (c.g[V0][0] as u32).wrapping_add(3));
                        c.lwr(3, c.g[V0][0] as u32);
                        c.swl(3, sp(c).wrapping_add(3));
                        c.swr(3, sp(c));
                        c.g[V1][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                        Addr((c.g[A2][0] as u32).wrapping_add(0x28)).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr(sp(c)).lh(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 16);
                        c.g[V0][0] &= c.g[T2][0];
                        c.g[V0][0] |= c.g[V1][0];
                        let more = c.g[A1][0] != 0;
                        Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                        if !more {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_cff0);
                        }
                    }
                }
                // Relocate `+0x28` records (0x40 bytes each, field +8 at +0x2c).
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
                if !empty {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V1][0];
                    loop {
                        c.cycles += 7;
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        let more = c.g[A1][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_d0b0);
                        }
                    }
                }
                // Relocate the `+0x20` records (0x54 bytes each, field +8 at +0x24).
                c.cycles += 4;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                if !empty {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V1][0];
                    loop {
                        c.cycles += 7;
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        let more = c.g[A1][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_d0e0);
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 3;
        }
        c.cycles += 7;
        Addr(entry).sh(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptEngine_GetTargetObject.

/// `ScriptEngine_GetInstanceScript` (0x27a668).
const OC420_GET_INSTANCE_SCRIPT: Callee = Callee {
    addr: 0x0027_a668,
    f: ee::s07::f_0027a668,
};

/// Where `ScriptEngine_GetTargetObject` continues (the blocks of the
/// generated body, reached from the jump table or from one another).
#[derive(Clone, Copy)]
enum Oc420Target {
    /// Slot 0: the player's first rider index.
    Slot0,
    /// Slot 1 and 2: rider index 1 or 2 of the instance script.
    Slot1,
    Slot2,
    /// Rider index in `a1` (>= 0 checked): the rider object.
    RiderIndex,
    /// Slots 3 and 4.
    Slot3,
    /// Slots 5..9 and 10..14.
    Slot5,
    Slot10,
    /// The +0x48 table: `v0` is the byte offset of the entry.
    Table48,
    /// Object in `v0`: its +0x18 word, default 0 (`v1`).
    ObjectField,
    /// Slots 15..20.
    Slot15,
    /// Return 0.
    Null,
    /// Return `v0`.
    Return,
}

/// `Object* ScriptEngine_GetTargetObject(ScriptEngine* self, int slot,
/// int player)` (0x279f18): the object the script's target `slot` (0..20)
/// names: slots 0-2 are the player's rider indices from the instance script
/// (`GetInstanceScript` words 0..2), 3-4 and 5-14 index the level's object
/// tables (+0x40; +0x48/+0x5c, split at the counts at +0x80/+0x84) and
/// 15-20 its rider list (+0x28). The slot's code address comes from a jump
/// table at 0x481ca0; an address the port does not know goes to
/// `jump_out`. The result is the object's word at +0x18 (or the rider
/// pointer), 0 when out of range.
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn script_engine_get_target_object(c: &mut Cpu) {
    if c.pc != 0x0027_9f18 {
        return oc420_get_target_object_resume(c);
    }
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[A1][0] < 0x15) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut at = if out_of_range {
            Oc420Target::Null
        } else {
            c.cycles += 7;
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ca0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x0027_9f48 => Oc420Target::Slot0,
                0x0027_9f68 => Oc420Target::Slot1,
                0x0027_9f7c => Oc420Target::Slot2,
                0x0027_9fac => Oc420Target::Slot3,
                0x0027_9fe8 => Oc420Target::Slot5,
                0x0027_a034 => Oc420Target::Slot10,
                0x0027_a098 => Oc420Target::Slot15,
                other => {
                    c.jump_out(other, 0x0027_9f18, 0x0027_a0d4);
                    return Ok(());
                }
            }
        };
        let world = |c: &mut Cpu| Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        loop {
            at = match at {
                Oc420Target::Slot0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[A2][0];
                    OC420_GET_INSTANCE_SCRIPT.call(c, 0x0027_9f54)?;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let negative = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = world(c);
                    if negative {
                        Oc420Target::Null
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                        Oc420Target::RiderIndex
                    }
                }
                Oc420Target::Slot1 | Oc420Target::Slot2 => {
                    let (cycles, ret, field) = if matches!(at, Oc420Target::Slot1) {
                        (5, 0x0027_9f74, 4)
                    } else {
                        (4, 0x0027_9f88, 8)
                    };
                    c.cycles += cycles;
                    c.g[A1][0] = c.g[A2][0];
                    c.g[A0][0] = c.g[A3][0];
                    OC420_GET_INSTANCE_SCRIPT.call(c, ret)?;
                    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(field)).lw(c);
                    c.cycles += 2;
                    let negative = (c.g[A1][0] as i64) < 0;
                    c.g[A0][0] = world(c);
                    if negative {
                        Oc420Target::Null
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                        Oc420Target::RiderIndex
                    }
                }
                Oc420Target::RiderIndex => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
                    Oc420Target::Return
                }
                Oc420Target::Slot3 => {
                    c.cycles += 8;
                    c.g[V0][0] = world(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(3));
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    if none {
                        Oc420Target::Null
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
                        c.g[A0][0] = 0;
                        if c.g[V0][0] != 0 {
                            c.cycles += 1;
                            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
                        }
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A0][0];
                        Oc420Target::Return
                    }
                }
                Oc420Target::Slot5 => {
                    c.cycles += 10;
                    c.g[V1][0] = world(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(5));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x80)).lw(c);
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if none {
                        Oc420Target::Null
                    } else {
                        // j < n80: the +0x48 table; else the +0x5c table at j - n80.
                        c.cycles += 2;
                        let inside = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        if inside {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                            Oc420Target::Table48
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = 0;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5c)).lw(c);
                            Oc420Target::ObjectField
                        }
                    }
                }
                Oc420Target::Slot10 => {
                    c.cycles += 10;
                    c.g[V1][0] = world(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(10));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x80)).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if none {
                        Oc420Target::Null
                    } else {
                        // j < n84: the +0x5c table; else the +0x48 table at j - n84.
                        c.cycles += 2;
                        let inside = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                        if !inside {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            Oc420Target::Table48
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = 0;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5c)).lw(c);
                            Oc420Target::ObjectField
                        }
                    }
                }
                Oc420Target::Table48 => {
                    c.cycles += 3;
                    c.g[V1][0] = 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lw(c);
                    Oc420Target::ObjectField
                }
                Oc420Target::ObjectField => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    Oc420Target::Return
                }
                Oc420Target::Slot15 => {
                    c.cycles += 8;
                    c.g[V0][0] = world(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf));
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    if none {
                        Oc420Target::Null
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
                        Oc420Target::Return
                    }
                }
                Oc420Target::Null => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    Oc420Target::Return
                }
                Oc420Target::Return => break,
            };
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(not(feature = "ee-unwind"))]
fn oc420_get_target_object_resume(c: &mut Cpu) {
    ee::s07::f_00279f18__entries(c)
}
#[cfg(feature = "ee-unwind")]
fn oc420_get_target_object_resume(c: &mut Cpu) {
    ee::s07::f_00279f18__generated(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, machine, FAKE};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP, SP as SP_TOP};
    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------------------ fake callees

    /// Control word of this module's gated fakes (in the machine's RAM, so it
    /// never leaks into another test even when tests share a process).
    const OC420_CTRL: u32 = DATA + 0x3f_0000;
    const OC420_MAGIC: u32 = 0x0420_f00d;
    /// One forced result word per fake (`ret_idx`).
    const OC420_RET: u32 = DATA + 0x3f_0010;

    /// A logging fake while the control word is set (its result is the word
    /// the fixture stored for it, sign-extended), otherwise whatever was
    /// installed before (chained through `prev`).
    fn oc420_fake_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC420_CTRL) == OC420_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(OC420_RET + 4 * ret_idx));
            }
            return;
        }
        crate::difftest::fakes::fall_back(c, prev, fallback)
    }

    /// Stamps out a gated fake for the override slot of `$addr` and the
    /// function `$install` that hooks it (once per process).
    macro_rules! oc420_fake {
        ($fake:ident, $prev:ident, $install:ident, $addr:expr, $ret:expr, $shard:ident, $slot:ident, $gen:ident) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc420_fake_gated(c, $addr, $ret, &$prev, ee::$shard::$gen);
            }
            fn $install() {
                crate::difftest::fakes::hook(
                    std::ptr::addr_of_mut!(ee::$shard::$slot),
                    $fake,
                    &$prev,
                );
            }
        };
    }

    oc420_fake!(
        oc420_fake_section,
        OC420_PREV_SECTION,
        oc420_hook_section,
        0x0022_e0e0,
        0,
        s05,
        OVERRIDE_0022E0E0,
        f_0022e0e0__generated
    );
    oc420_fake!(
        oc420_fake_get_be,
        OC420_PREV_GET_BE,
        oc420_hook_get_be,
        0x0014_dc80,
        1,
        s01,
        OVERRIDE_0014DC80,
        f_0014dc80__generated
    );
    oc420_fake!(
        oc420_fake_get_iface,
        OC420_PREV_GET_IFACE,
        oc420_hook_get_iface,
        0x0014_dd58,
        2,
        s01,
        OVERRIDE_0014DD58,
        f_0014dd58__generated
    );
    oc420_fake!(
        oc420_fake_path_free,
        OC420_PREV_PATH_FREE,
        oc420_hook_path_free,
        0x0026_af00,
        3,
        s07,
        OVERRIDE_0026AF00,
        f_0026af00__generated
    );
    oc420_fake!(
        oc420_fake_path_set,
        OC420_PREV_PATH_SET,
        oc420_hook_path_set,
        0x0026_ad70,
        4,
        s07,
        OVERRIDE_0026AD70,
        f_0026ad70__generated
    );
    oc420_fake!(
        oc420_fake_init_paths,
        OC420_PREV_INIT_PATHS,
        oc420_hook_init_paths,
        0x0011_2180,
        5,
        s00,
        OVERRIDE_00112180,
        f_00112180__generated
    );
    oc420_fake!(
        oc420_fake_rand_range,
        OC420_PREV_RAND_RANGE,
        oc420_hook_rand_range,
        0x0031_7830,
        6,
        s10,
        OVERRIDE_00317830,
        f_00317830__generated
    );
    oc420_fake!(
        oc420_fake_luno_as_float,
        OC420_PREV_LUNO_AS_FLOAT,
        oc420_hook_luno_as_float,
        0x0022_6618,
        7,
        s05,
        OVERRIDE_00226618,
        f_00226618__generated
    );
    oc420_fake!(
        oc420_fake_audio_instance,
        OC420_PREV_AUDIO_INSTANCE,
        oc420_hook_audio_instance,
        0x0028_b180,
        8,
        s07,
        OVERRIDE_0028B180,
        f_0028b180__generated
    );
    oc420_fake!(
        oc420_fake_audio_stop_loop,
        OC420_PREV_AUDIO_STOP_LOOP,
        oc420_hook_audio_stop_loop,
        0x0029_7eb8,
        9,
        s08,
        OVERRIDE_00297EB8,
        f_00297eb8__generated
    );
    oc420_fake!(
        oc420_fake_add_parent,
        OC420_PREV_ADD_PARENT,
        oc420_hook_add_parent,
        0x0035_5978,
        10,
        s11,
        OVERRIDE_00355978,
        f_00355978__generated
    );
    oc420_fake!(
        oc420_fake_rider_script_off,
        OC420_PREV_SCRIPT_OFF,
        oc420_hook_script_off,
        0x0028_03f0,
        11,
        s07,
        OVERRIDE_002803F0,
        f_002803f0__generated
    );
    oc420_fake!(
        oc420_fake_rider_reset,
        OC420_PREV_RIDER_RESET,
        oc420_hook_rider_reset,
        0x0011_d390,
        12,
        s00,
        OVERRIDE_0011D390,
        f_0011d390__generated
    );
    oc420_fake!(
        oc420_fake_hide_part,
        OC420_PREV_HIDE_PART,
        oc420_hook_hide_part,
        0x0030_ea80,
        13,
        s10,
        OVERRIDE_0030EA80,
        f_0030ea80__generated
    );
    oc420_fake!(
        oc420_fake_show_part,
        OC420_PREV_SHOW_PART,
        oc420_hook_show_part,
        0x0030_e9e0,
        14,
        s10,
        OVERRIDE_0030E9E0,
        f_0030e9e0__generated
    );
    oc420_fake!(
        oc420_fake_bolts_db,
        OC420_PREV_BOLTS_DB,
        oc420_hook_bolts_db,
        0x0014_bdb8,
        15,
        s01,
        OVERRIDE_0014BDB8,
        f_0014bdb8__generated
    );
    oc420_fake!(
        oc420_fake_profile,
        OC420_PREV_PROFILE,
        oc420_hook_profile,
        0x0014_ad50,
        16,
        s01,
        OVERRIDE_0014AD50,
        f_0014ad50__generated
    );
    oc420_fake!(
        oc420_fake_get_character,
        OC420_PREV_GET_CHARACTER,
        oc420_hook_get_character,
        0x0012_3128,
        17,
        s00,
        OVERRIDE_00123128,
        f_00123128__generated
    );
    oc420_fake!(
        oc420_fake_bolt_count,
        OC420_PREV_BOLT_COUNT,
        oc420_hook_bolt_count,
        0x0014_d988,
        18,
        s01,
        OVERRIDE_0014D988,
        f_0014d988__generated
    );
    oc420_fake!(
        oc420_fake_bolt_list,
        OC420_PREV_BOLT_LIST,
        oc420_hook_bolt_list,
        0x0014_d998,
        19,
        s01,
        OVERRIDE_0014D998,
        f_0014d998__generated
    );
    oc420_fake!(
        oc420_fake_stop_handle,
        OC420_PREV_STOP_HANDLE,
        oc420_hook_stop_handle,
        0x002a_d5f0,
        20,
        s08,
        OVERRIDE_002AD5F0,
        f_002ad5f0__generated
    );
    oc420_fake!(
        oc420_fake_rider_char,
        OC420_PREV_RIDER_CHAR,
        oc420_hook_rider_char,
        0x0014_ac30,
        21,
        s01,
        OVERRIDE_0014AC30,
        f_0014ac30__generated
    );
    oc420_fake!(
        oc420_fake_alt_char,
        OC420_PREV_ALT_CHAR,
        oc420_hook_alt_char,
        0x0014_a0b0,
        22,
        s01,
        OVERRIDE_0014A0B0,
        f_0014a0b0__generated
    );
    oc420_fake!(
        oc420_fake_rider_profile,
        OC420_PREV_RIDER_PROFILE,
        oc420_hook_rider_profile,
        0x0014_ac50,
        23,
        s01,
        OVERRIDE_0014AC50,
        f_0014ac50__generated
    );
    oc420_fake!(
        oc420_fake_bolt_texture,
        OC420_PREV_BOLT_TEXTURE,
        oc420_hook_bolt_texture,
        0x0014_b988,
        24,
        s01,
        OVERRIDE_0014B988,
        f_0014b988__generated
    );
    oc420_fake!(
        oc420_fake_rider_init,
        OC420_PREV_RIDER_INIT,
        oc420_hook_rider_init,
        0x0014_0738,
        25,
        s01,
        OVERRIDE_00140738,
        f_00140738__generated
    );
    oc420_fake!(
        oc420_fake_script_obj_rider,
        OC420_PREV_SCRIPT_OBJ_RIDER,
        oc420_hook_script_obj_rider,
        0x0027_f968,
        26,
        s07,
        OVERRIDE_0027F968,
        f_0027f968__generated
    );
    oc420_fake!(
        oc420_fake_mem_alloc,
        OC420_PREV_MEM_ALLOC,
        oc420_hook_mem_alloc,
        0x0031_7d70,
        27,
        s10,
        OVERRIDE_00317D70,
        f_00317d70__generated
    );
    oc420_fake!(
        oc420_fake_emitter_fx,
        OC420_PREV_EMITTER_FX,
        oc420_hook_emitter_fx,
        0x002f_1850,
        28,
        s09,
        OVERRIDE_002F1850,
        f_002f1850__generated
    );
    oc420_fake!(
        oc420_fake_bolt_on_fx,
        OC420_PREV_BOLT_ON_FX,
        oc420_hook_bolt_on_fx,
        0x002e_c9e0,
        29,
        s09,
        OVERRIDE_002EC9E0,
        f_002ec9e0__generated
    );
    oc420_fake!(
        oc420_fake_shadow_ctor,
        OC420_PREV_SHADOW_CTOR,
        oc420_hook_shadow_ctor,
        0x0037_58f8,
        30,
        s12,
        OVERRIDE_003758F8,
        f_003758f8__generated
    );
    oc420_fake!(
        oc420_fake_script_register,
        OC420_PREV_SCRIPT_REGISTER,
        oc420_hook_script_register,
        0x0028_3298,
        31,
        s07,
        OVERRIDE_00283298,
        f_00283298__generated
    );
    oc420_fake!(
        oc420_fake_mem_free,
        OC420_PREV_MEM_FREE,
        oc420_hook_mem_free,
        0x0031_7e98,
        32,
        s10,
        OVERRIDE_00317E98,
        f_00317e98__generated
    );

    /// The accessor returns the word at +0x20.
    #[test]
    fn script_cam_get_target_params_matches() {
        check(
            ee::s07::f_0027c2a8__generated,
            script_cam_get_target_params,
            50,
            &|c, seed| {
                let mut r = Rng(0x27c2_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0027_c2a8;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x120, r.next_u32());
            },
        );
    }

    /// Heap allocator wrapper against free lists of 0..3 blocks (fits,
    /// splits, none), with a random `gp[0x398]` bias.
    #[test]
    fn in_seq_heap_new_matches() {
        check_lohi(ee::s07::f_00272488__generated, in_seq_heap_new, 160, &|c, seed| {
            let mut r = Rng(0x2724_8800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0027_2488;
            c.g[GP][0] = GAME_GP as u64;
            c.bus
                .write32(GAME_GP + 0x398, [0u32, 4, 8, 12][(seed / 4 % 4) as usize]);
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
            c.g[A0][0] = sx(want);
            c.g[A1][0] = sx(heap);
        });
    }

    /// Section accessors: entry `i` of the 12-byte table, read back through
    /// the real script-cache getters.
    fn oc420_section_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x3a6c_0000 ^ seed as u64);
        scramble(c, &mut r);
        let (world, mid, table) = (DATA, DATA + 0x100, DATA + 0x200);
        c.pc = entry;
        c.g[A0][0] = sx(world);
        c.g[A1][0] = [0u64, 1, 2, 5, 0xffff_ffff_0000_0003][(seed % 5) as usize];
        c.bus.write32(world + 4, mid);
        c.bus.write32(mid + 4, table);
        for i in 0..8 {
            c.bus.write32(table + 12 * i + 0x20, r.next_u32());
            c.bus.write32(table + 12 * i + 0x24, r.next_u32());
        }
    }

    #[test]
    fn world_get_section_missions_matches() {
        check_lohi(
            ee::s13::f_003a6c98__generated,
            world_get_section_missions,
            40,
            &|c, seed| oc420_section_fixture(c, seed, 0x003a_6c98),
        );
    }

    #[test]
    fn world_get_section_mission_count_matches() {
        check_lohi(
            ee::s13::f_003a6c68__generated,
            world_get_section_mission_count,
            40,
            &|c, seed| oc420_section_fixture(c, seed, 0x003a_6c68),
        );
    }

    /// The hook forwards to the flare-size lookup for assorted resolutions.
    #[test]
    fn world_chunk_resolve_hook_matches() {
        check(
            ee::s13::f_003ad230__generated,
            world_chunk_resolve_hook,
            70,
            &|c, seed| {
                machine(c, seed, 0x3a_d230, 0);
                c.g[A0][0] = [0x20u64, 0x10, 0x40, 0x30, 33, 0, u64::MAX][(seed % 7) as usize];
                c.pc = 0x003a_d230;
            },
        );
    }

    // ------------------------------------------------------- InSeq_ChannelStart

    const OC420_THIS: u32 = DATA + 0x1000;
    const OC420_OWNER: u32 = DATA + 0x1100;
    const OC420_HEAP: u32 = DATA + 0x1200;
    const OC420_HDR: u32 = DATA + 0x1400;
    const OC420_LISTENER: u32 = DATA + 0x1800;
    const OC420_LISTENER_VT: u32 = DATA + 0x1900;
    const OC420_PLAYER: u32 = DATA + 0x1a00;
    const OC420_PLAYER_VT: u32 = DATA + 0x1b00;
    const OC420_DUMMY: u32 = DATA + 0x1c00;
    const OC420_DUMMY_VT: u32 = DATA + 0x1d00;
    const OC420_BLOCK: u32 = DATA + 0x4000;

    /// An instance with 0..4 channel records (types 0-3 build a track
    /// object with the real constructor; type 4 builds nothing), channel
    /// virtuals that are fakes (non-zero) or the zero leaf, an optional
    /// listener that hands out a player, and an optional stale player.
    fn oc420_channel_start_setup(c: &mut Cpu, seed: u32, min_channels: u32) -> Rng {
        let mut r = machine(c, seed, 0x27_3aa8, 4);
        c.pc = 0x0027_3aa8;
        c.g[A0][0] = OC420_THIS as u64;
        c.bus
            .write32(GAME_GP + 0x398, [0u32, 4][(seed % 2) as usize]);
        c.bus.write32(OC420_THIS + 8, OC420_OWNER);
        c.bus.write32(OC420_OWNER + 8, OC420_HEAP);
        c.bus.write32(OC420_HEAP + 0x10, OC420_BLOCK);
        c.bus.write32(OC420_BLOCK, 0x800);
        c.bus.write32(OC420_BLOCK + 4, 0);
        for k in 0..4 {
            c.bus.write32(OC420_BLOCK + 8 + 4 * k, OC420_DUMMY);
        }
        c.bus.write32(OC420_DUMMY + 4, OC420_DUMMY_VT);
        c.bus.write16(OC420_DUMMY_VT + 8, 0);
        c.bus.write32(OC420_DUMMY_VT + 0xc, FAKE + 0xf0);
        // The four track vtables the constructors point objects at.
        let mut fakes_used = 0;
        let mut vt_fn = [0u32; 4];
        for (i, vt) in [0x0048_1a10u32, 0x0048_18f0, 0x0048_19b0, 0x0048_1950]
            .into_iter()
            .enumerate()
        {
            let f = if r.next_u32() % 4 == 0 {
                0x0014_06c0
            } else {
                FAKE + 0x10 * (i as u32 + 1)
            };
            vt_fn[i] = f;
            c.bus
                .write16(vt + 8, [0u16, 0x10][(r.next_u32() % 2) as usize]);
            c.bus.write32(vt + 0xc, f);
        }
        let n = min_channels + (seed / 2) % (5 - min_channels);
        c.bus.write32(OC420_THIS + 0x1c, OC420_HDR);
        c.bus.write16(OC420_HDR, 0x10);
        c.bus.write8(OC420_HDR + 2, n as u8);
        c.bus.write8(OC420_HDR + 3, r.next_u32() as u8);
        let mut rec = OC420_HDR + 0x10;
        for _ in 0..n {
            let kind = r.next_u32() % 5;
            let size = 4 * (r.next_u32() % 3);
            c.bus.write16(rec, kind as u16);
            c.bus.write16(rec + 2, 4 + size as u16);
            // Fake channels: the track kinds 0 (curve), 1 (ramp), 2 (span), 3 (event).
            let used = match kind {
                0 => vt_fn[0],
                1 => vt_fn[1],
                2 => vt_fn[2],
                3 => vt_fn[3],
                _ => FAKE + 0xf0,
            };
            if used >= FAKE && used < FAKE + 0x100 {
                fakes_used += 1;
            }
            rec += 4 + 4 + size;
        }
        // Listener (cached at +0x44 or through the owner) and player.
        let listener = [0u32, OC420_LISTENER][(r.next_u32() % 4 != 0) as usize];
        if r.next_u32() % 2 == 0 {
            c.bus.write32(OC420_THIS + 0x44, listener);
        } else {
            c.bus.write32(OC420_THIS + 0x44, 0);
            c.bus.write32(OC420_OWNER, listener);
        }
        c.bus.write32(OC420_LISTENER, OC420_LISTENER_VT);
        c.bus.write16(
            OC420_LISTENER_VT + 0x30,
            [0u16, 8][(r.next_u32() % 2) as usize],
        );
        c.bus.write32(OC420_LISTENER_VT + 0x34, FAKE + 0xe0);
        c.bus.write32(OC420_PLAYER, OC420_PLAYER_VT);
        c.bus
            .write16(OC420_PLAYER_VT + 8, [0u16, 4][(r.next_u32() % 2) as usize]);
        c.bus.write32(OC420_PLAYER_VT + 0xc, FAKE + 0xd0);
        c.bus.write32(
            OC420_THIS + 0x2c,
            [0u32, OC420_PLAYER][(r.next_u32() % 2) as usize],
        );
        // The listener's call is the next fake after the channel starts
        // (only the fake-backed ones count); it hands out the player.
        c.bus.write32(
            crate::difftest::fakes::LOG_RESULTS + 4 * fakes_used,
            OC420_PLAYER,
        );
        c.bus.write32(OC420_THIS + 0x24, 0);
        r
    }

    #[test]
    fn in_seq_channel_start_matches() {
        use crate::difftest::events::{check_events, schedule};
        covered(
            ee::s07::f_00273aa8__generated,
            in_seq_channel_start,
            300,
            &|c, seed| {
                oc420_channel_start_setup(c, seed, 0);
            },
        );
        check_events(
            ee::s07::f_00273aa8__generated,
            in_seq_channel_start,
            150,
            &|c, seed| {
                let mut r = oc420_channel_start_setup(c, seed, 2);
                schedule(c, &mut r, seed, 0x0027_3aa8)
            },
        );
    }

    // ------------------------------------------------------- cLunoTable_Alloc

    /// `cLunoTable_Alloc` against the real allocator (pool + arena): zero
    /// to six nodes.
    #[test]
    fn cluno_table_alloc_matches() {
        use crate::difftest::elf_image;
        check(ee::s05::f_00224e50__generated, cluno_table_alloc, 14, &|c, seed| {
            machine(c, seed, 0x22_4e50, 0);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9_0000);
            c.bus.write32(GAME_GP - 0x848, DATA + 0xa_0000);
            crate::game::ai::computer::tests::rbc_test_allocator(c);
            c.pc = 0x0022_4e50;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = [0u64, 1, 2, 3, 6][(seed % 5) as usize];
        });
    }

    // ------------------------------------------------ AIFwdDiffCacheEntry_ctor

    #[test]
    fn ai_fwd_diff_cache_entry_ctor_matches() {
        use crate::difftest::events::{check_events, schedule};
        check(
            ee::s11::f_00340970__generated,
            ai_fwd_diff_cache_entry_ctor,
            20,
            &|c, seed| {
                let mut r = Rng(0x3409_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0034_0970;
            },
        );
        check_events(
            ee::s11::f_00340970__generated,
            ai_fwd_diff_cache_entry_ctor,
            90,
            &|c, seed| {
                let mut r = Rng(0x3409_7100 ^ seed as u64);
                scramble(c, &mut r);
                schedule(c, &mut r, seed, 0x0034_0970)
            },
        );
    }

    // ------------------------------------------------- AI_SetTrackPathsBuffer

    /// Level ids around the 17..21 window, the new-race flag bytes, modes
    /// 0-3 (and one with junk in the high word), with and without an AI
    /// interface and 0-3 riders. The `SectionMan`/`cBE`/path callees are fakes.
    #[test]
    fn ai_set_track_paths_buffer_matches() {
        oc420_hook_section();
        oc420_hook_get_be();
        oc420_hook_get_iface();
        oc420_hook_path_free();
        oc420_hook_path_set();
        oc420_hook_init_paths();
        covered(
            ee::s00::f_0012a340__generated,
            ai_set_track_paths_buffer,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_a340, 8);
                c.pc = 0x0012_a340;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                let lvl = [3u32, 17, 18, 19, 20, 21, 22, 30, 16][(seed % 9) as usize];
                c.bus.write32(OC420_RET, lvl);
                let (world, man, this) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x2000);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0x78, DATA + 0x1200);
                c.bus.write32(
                    man + 0xc,
                    [0u32, DATA + 0x1300][(r.next_u32() % 3 != 0) as usize],
                );
                // The new-race interface flags.
                let flags = [5u8, 6, 11, 0, 7];
                c.bus
                    .write8(0x0053_5bc8 + 0x48, flags[(r.next_u32() % 5) as usize]);
                c.bus
                    .write8(0x0053_5bc8 + 0x4a, flags[(r.next_u32() % 5) as usize]);
                let riders = (r.next_u32() % 4) as u32;
                c.bus
                    .write32(this + 0x78, if r.next_u32() % 5 == 0 { 0 } else { riders });
                for i in 0..4 {
                    c.bus
                        .write32(this + 0x28 + 4 * i, DATA + 0x3000 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A2][0] = match (seed / 9) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => 0x1_0000_0001,
                };
                c.g[A3][0] = sx(DATA + 0x4000);
            },
        );
    }

    // --------------------------------------- ScriptEngine_IsObjBoundToProcess

    /// The object's binding count and binding getter are fakes (virtuals at
    /// +0x30 and +0x38 of its vtable); the binding check is the real
    /// `ScriptEngine_BindingHasProcess` over a tiny process table.
    #[test]
    fn script_engine_is_obj_bound_to_process_matches() {
        covered(
            ee::s07::f_0027d210__generated,
            script_engine_is_obj_bound_to_process,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_d210, 6);
                c.pc = 0x0027_d210;
                let (engine, obj, vt) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let idx = seed % 3;
                c.bus.write32(engine + 4 * idx, obj);
                c.bus.write32(obj + 0xc, vt);
                c.bus
                    .write16(vt + 0x30, [0u16, 4][(r.next_u32() % 2) as usize]);
                // Binding count: the zero leaf (no bindings) or a fake forced to 1..3.
                if seed % 5 == 0 {
                    c.bus.write32(vt + 0x34, 0x0014_06c0);
                } else {
                    c.bus.write32(vt + 0x34, FAKE + 0x10);
                    c.bus
                        .write32(crate::difftest::fakes::LOG_RESULTS, 1 + seed % 3);
                }
                c.bus.write32(engine + 0x58, (r.next_u32() % 4) as u32);
                c.bus
                    .write16(vt + 0x38, [0u16, 8][(r.next_u32() % 2) as usize]);
                c.bus.write32(vt + 0x3c, FAKE + 0x20);
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = if seed % 4 == 0 {
                    0xffff_ffff_0000_0000 | idx as u64
                } else {
                    idx as u64
                };
                c.g[A2][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    // ------------------------------------------- cPS2ChunkResolve_ParseHeader

    /// A sphere-tree header (random words; node count 0..4, leaf or inner
    /// flag) copied through the real `MEM_copy`, and a null buffer.
    #[test]
    fn cps2_chunk_resolve_parse_header_matches() {
        check(
            ee::s10::f_00327da8__generated,
            cps2_chunk_resolve_parse_header,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_7da8, 0);
                c.pc = 0x0032_7da8;
                let (this, buf) = (DATA + 0x100, DATA + 0x1000);
                for i in 0..0x40 {
                    c.bus.write32(buf + 4 * i, r.next_u32());
                }
                c.bus.write32(buf, r.next_u32() % 0x100);
                c.bus.write32(buf + 4, r.next_u32() % 0x100);
                c.bus.write32(buf + 8, [0u32, 1, 7][(seed % 3) as usize]);
                c.bus.write32(buf + 0xc, r.next_u32() % 5);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { buf as u64 };
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------ WScriptFn_RandRange

    /// With and without the first-use static init, 0..3 arguments of
    /// matching and differing kinds (the int->float conversion included);
    /// the random call and the register accessor are fakes, `MEM_copy` is real.
    #[test]
    fn wscript_fn_rand_range_matches() {
        oc420_hook_rand_range();
        oc420_hook_luno_as_float();
        covered(
            ee::s10::f_00303598__generated,
            wscript_fn_rand_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_3598, 3);
                c.pc = 0x0030_3598;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 7, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x26c8, (seed % 2) as u32);
                // The static parameter block (already initialised half the time).
                c.bus.write32(0x004f_bc60, r.next_u32());
                c.bus.write32(0x004f_bc64, 0x3f80_0000);
                c.bus.write8(0x004f_bc68, r.next_u32() as u8);
                let kinds = [0u32, 1, 2];
                for k in 0..3 {
                    c.bus
                        .write32(GAME_GP + 0xc30 + 4 * k, kinds[(r.next_u32() % 3) as usize]);
                }
                let args = DATA + 0x2000;
                let n = (seed / 2) % 4;
                for i in 0..n {
                    let e = args + 0x10 * i;
                    let k = r.next_u32() % 3;
                    c.bus.write32(e, k);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 0xc, (r.next_u32() % 3) as u32);
                }
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = if seed % 9 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
            },
        );
    }

    // ----------------------------------------- the WScriptFn_* decoder fixture

    /// The object ids a decoder resolves: `slot | index << 8` for the slots
    /// and indices of `oc420_world_objects`, and -1 (the current object).
    const OC420_IDS: [u32; 8] = [0xffff_ffff, 0x000, 0x100, 0x001, 0x101, 0x302, 0x005, 0x200];

    /// World cache, the object records and their move nodes. The cache is
    /// reached through `gp + 0x16c8`; slots 0..3 hold records (slot 2 empty),
    /// whose tables hold `value << 8` with `value << 2` an object address
    /// (zero for some entries); the current object is `OBJ0`. Each object's
    /// word at +0xc is its move node; node vtable slots +0x50/+0x54 and
    /// +0x80/+0x84 are fakes or the zero leaf.
    fn oc420_world_objects(c: &mut Cpu, r: &mut Rng) {
        let (cache_ptr, cache, slots) = (DATA + 0x1100, DATA + 0x1200, DATA + 0x1300);
        c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
        c.bus.write32(cache_ptr, cache);
        c.bus.write32(cache + 8, slots);
        let vt = DATA + 0x2800;
        for s in 0..4u32 {
            let rec = DATA + 0x1400 + 0x40 * s;
            c.bus.write32(slots + 4 * s, if s == 2 { 0 } else { rec });
            let table = DATA + 0x1500 + 0x20 * s;
            c.bus.write32(rec + 0x1c, table);
            for k in 0..4u32 {
                let obj = DATA + 0x2000 + 0x100 * ((s + k) % 4);
                let entry = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    (obj >> 2) << 8
                };
                c.bus.write32(table + 4 * k, entry);
            }
        }
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x1600);
        c.bus.write32(DATA + 0x1600 + 0x290, DATA + 0x2000);
        for j in 0..4u32 {
            let obj = DATA + 0x2000 + 0x100 * j;
            let node = DATA + 0x2400 + 0x100 * j;
            c.bus
                .write32(obj + 0xc, if r.next_u32() % 5 == 0 { 0 } else { node });
            c.bus.write32(node + 0xc, vt);
            c.bus.write32(node + 0x18, r.next_u32());
        }
        c.bus
            .write16(vt + 0x50, [0u16, 8][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x54,
            [FAKE + 0x10, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
        c.bus
            .write16(vt + 0x80, [0u16, 4][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x84,
            [FAKE + 0x20, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
    }

    /// `n` argument entries `{index, value, _, kind}` at `args`; the first
    /// one often sets word 0 (the object id) to one of `OC420_IDS`.
    fn oc420_args(c: &mut Cpu, r: &mut Rng, args: u32, n: u32, words: u32, table: u32) {
        for k in 0..words {
            c.bus
                .write32(table + 4 * k, [0u32, 1, 2][(r.next_u32() % 3) as usize]);
        }
        for i in 0..n {
            let e = args + 0x10 * i;
            let idx = if i == 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() % words
            };
            c.bus.write32(e, idx);
            let val = if idx == 0 {
                OC420_IDS[(r.next_u32() % 8) as usize]
            } else {
                r.next_u32() % 1000
            };
            c.bus.write32(e + 4, val);
            let tv = c.bus.read32(table + 4 * idx);
            c.bus.write32(e + 0xc, tv);
            if r.next_u32() % 3 == 0 {
                c.bus.write32(e + 0xc, r.next_u32() % 3);
            }
        }
    }

    // ------------------------------------------------ WScriptFn_StopObjectLoop

    #[test]
    fn wscript_fn_stop_object_loop_matches() {
        oc420_hook_audio_instance();
        oc420_hook_audio_stop_loop();
        covered(
            ee::s10::f_00300260__generated,
            wscript_fn_stop_object_loop,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_0260, 3);
                c.pc = 0x0030_0260;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 8, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x257c, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, -1, flag}.
                let d = 0x004f_bb28;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, r.next_u32());
                c.bus.write8(d + 0xc, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 4, 0x0044_6560);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 11 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    // -------------------------------------------- WScriptFn_AddParentModifierRev

    #[test]
    fn wscript_fn_add_parent_modifier_rev_matches() {
        oc420_hook_add_parent();
        covered(
            ee::s10::f_002fdc60__generated,
            wscript_fn_add_parent_modifier_rev,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_dc60, 5);
                c.pc = 0x002f_dc60;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 10, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x2520, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, 0, 0.0 x3, flag}.
                let d = 0x004f_b758;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 8, r.next_u32());
                for k in 3..6 {
                    c.bus.write32(d + 4 * k, r.f32(10.0));
                }
                c.bus.write8(d + 0x18, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 6, 0x0044_61c0);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 13 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    // ------------------------------------------------ Rider_DetachFromScript

    /// With and without the sound handle, the bolt parts (0..3 bolt records:
    /// negative id, flag clear, profile slot missing or present with the
    /// show bit set or clear) and handles at +0xb08 that are in use or -1.
    /// Every callee but the two trivial leaves (`0x2a1bd0`, `0x27c958`) is a fake.
    #[test]
    fn rider_detach_from_script_matches() {
        oc420_hook_script_off();
        oc420_hook_rider_reset();
        oc420_hook_audio_instance();
        oc420_hook_hide_part();
        oc420_hook_show_part();
        oc420_hook_get_be();
        oc420_hook_get_iface();
        oc420_hook_bolts_db();
        oc420_hook_profile();
        oc420_hook_get_character();
        oc420_hook_bolt_count();
        oc420_hook_bolt_list();
        oc420_hook_stop_handle();
        covered(
            ee::s00::f_00123b48__generated,
            rider_detach_from_script,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_3b48, 30);
                c.pc = 0x0012_3b48;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                let (this, obj, ptr) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x2100);
                let (prof, shorts, recs, audio, list) = (
                    DATA + 0x4000,
                    DATA + 0x4400,
                    DATA + 0x4800,
                    DATA + 0x4c00,
                    DATA + 0x5000,
                );
                c.bus.write32(this + 0xac8, [0u32, 5][(seed % 2) as usize]);
                c.bus.write32(this + 0x780, DATA + 0x1800);
                c.bus.write32(this + 0x6d8, obj);
                c.bus.write32(obj + 0x20, ptr);
                c.bus.write8(ptr + 6, [0u8, 1][((seed / 2) % 2) as usize]);
                c.bus.write32(this + 0x86c, r.next_u32());
                for k in 0..4 {
                    let h = if r.next_u32() % 2 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 100
                    };
                    c.bus.write32(this + 0xb08 + 4 * k, h);
                }
                // The audio instance and what it points to.
                c.bus.write32(OC420_RET + 4 * 8, audio);
                c.bus.write32(audio + 0x118, list);
                c.bus.write32(list, DATA + 0x5100);
                // Bolt profile: slot table, records.
                c.bus.write32(OC420_RET + 4 * 16, prof);
                c.bus.write32(prof + 0x288, shorts);
                let count = (seed / 4) % 4;
                c.bus.write32(OC420_RET + 4 * 18, count);
                c.bus.write32(OC420_RET + 4 * 19, recs);
                for i in 0..4u32 {
                    let rec = recs + 0x38 * i;
                    c.bus
                        .write8(rec + 0x10, [0xffu8, 1, 2, 3][(r.next_u32() % 4) as usize]);
                    c.bus.write32(
                        rec + 0x34,
                        [0u32, 0x40, 0x40, 0xffff_ffff][(r.next_u32() % 4) as usize],
                    );
                    c.bus.write16(rec + 4, (r.next_u32() % 4) as u16);
                    let v = [0xffffu16, 0, 1, 2][(r.next_u32() % 4) as usize];
                    c.bus.write16(shorts + 2 * i, v);
                    c.bus.write16(
                        prof + 0x290 + 4 * (v as u32 & 3) + 2,
                        [0u16, 0x10, 0x10, 0xffff][(r.next_u32() % 4) as usize],
                    );
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------ Rider_AddBoltFilesToLoadList

    /// Cheat and plain characters, 0..3 bolt entries with null, empty,
    /// repeated and new names, a load-list table that already holds 0..2
    /// names; strcmp/strcpy/qsort are the real ones, the bolt database
    /// queries are fakes (the plain path's name buffer is pre-written on the
    /// stack).
    #[test]
    fn rider_add_bolt_files_to_load_list_matches() {
        oc420_hook_get_be();
        oc420_hook_get_iface();
        oc420_hook_rider_char();
        oc420_hook_alt_char();
        oc420_hook_rider_profile();
        oc420_hook_bolt_texture();
        covered(
            ee::s00::f_0011be88__generated,
            rider_add_bolt_files_to_load_list,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_be88, 20);
                c.pc = 0x0011_be88;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                let (this, table, list) = (DATA + 0x1000, DATA + 0x8000, DATA + 0x6000);
                c.bus.write32(this + 0x86c, r.next_u32());
                c.bus.write32(this + 0x894, table);
                let cheat = seed % 2;
                c.bus.write32(OC420_RET + 4 * 22, cheat);
                c.bus.write32(
                    OC420_RET + 4 * 24,
                    [0u32, 1, 1, 1][((seed / 2) % 4) as usize],
                );
                let names: [&[u8]; 4] = [b"bolt_a\0", b"bolt_b\0", b"\0", b"bolt_a\0"];
                let start = (r.next_u32() % 3) as u32;
                let count = (r.next_u32() % 3) as u32;
                c.bus.write32(table + 0x2940, start);
                c.bus.write32(table + 0x2944, count);
                for j in 0..(start + count) {
                    let name = names[(r.next_u32() % 2) as usize];
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(table + 0x108 * j + k as u32, *b);
                    }
                }
                let n = (seed / 8) % 4;
                for i in 0..n {
                    let obj = list + 0x100 * i;
                    c.bus.write32(list + 0x800 + 4 * i, obj);
                    let s = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        DATA + 0x7000 + 0x20 * i
                    };
                    c.bus.write32(obj + 0x2c, s);
                    if s != 0 {
                        for (k, b) in names[(r.next_u32() % 4) as usize].iter().enumerate() {
                            c.bus.write8(s + k as u32, *b);
                        }
                    }
                }
                // The plain path's name buffer (stack frame is 0x1b0 deep).
                for (k, b) in names[(r.next_u32() % 3) as usize].iter().enumerate() {
                    c.bus.write8(SP_TOP - 0x1b0 + k as u32, *b);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(list + 0x800);
                c.g[A2][0] = if seed % 17 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
            },
        );
    }

    // ---------------------------------------------------------------- Rider_ctor

    /// The whole constructor with every sub-constructor and the allocator
    /// faked (the allocator returns one scratch block; `EA_MemSet` and the
    /// audio-created hook are the real ones).
    #[test]
    fn rider_ctor_matches() {
        oc420_hook_rider_init();
        oc420_hook_script_obj_rider();
        oc420_hook_mem_alloc();
        oc420_hook_emitter_fx();
        oc420_hook_bolt_on_fx();
        oc420_hook_shadow_ctor();
        oc420_hook_script_register();
        oc420_hook_audio_instance();
        covered(ee::s00::f_0011b698__generated, rider_ctor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_b698, 12);
            c.pc = 0x0011_b698;
            c.bus.write32(OC420_CTRL, OC420_MAGIC);
            c.bus.write32(OC420_RET + 4 * 27, DATA + 0x6000);
            c.bus.write32(OC420_RET + 4 * 8, DATA + 0x6800);
            c.bus.write32(GAME_GP - 0x848, DATA + 0x500);
            c.bus.write32(DATA + 0x500 + 0x84, DATA + 0x540);
            c.bus.write32(DATA + 0x540 + 0x20, r.next_u32());
            c.bus.write32(GAME_GP - 0x79bc, r.next_u32());
            c.bus.write32(GAME_GP - 0x79b8, r.next_u32());
            c.bus.write32(0x004a_5960, r.next_u32());
            c.bus.write32(0x004a_5964, r.next_u32());
            c.g[A0][0] = sx(DATA + 0x2_0000);
        });
    }

    // ------------------------------------------------ tWScriptCache_Relocate

    /// A script block at `DATA + 0x1_0000`: header `0x1000` and world id,
    /// ten table offsets, 0..3 entries in each table (see the layout below),
    /// the world cache with three slot records (one empty) and objects that
    /// the records resolve to.
    fn oc420_relocate_setup(c: &mut Cpu, r: &mut Rng, seed: u32, kind: u32) {
        const D: u32 = DATA + 0x1_0000;
        let (entry, wp, cache, ymeta, slots) = (
            DATA + 0x1000,
            DATA + 0x1100,
            DATA + 0x1200,
            DATA + 0x1210,
            DATA + 0x1300,
        );
        let wid = 0x1234_5678u32;
        c.bus
            .write32(entry + 8, if seed % 16 == 15 && kind == 3 { 0 } else { D });
        c.bus.write16(entry + 2, if seed % 8 < 6 { 0 } else { 1 });
        c.bus.write32(wp, cache);
        c.bus.write32(cache, ymeta);
        c.bus.write32(ymeta + 4, wid);
        c.bus.write32(cache + 8, slots);
        c.bus.write16(D, if kind == 0 { 0x1001 } else { 0x1000 });
        c.bus
            .write32(D + 0xc, if kind == 1 { wid ^ 1 } else { wid });
        // Table offsets.
        for (off, v) in [
            (0x14u32, 0x400u32),
            (0x1c, 0x500),
            (0x24, 0x600),
            (0x2c, 0x800),
            (0x34, 0x900),
            (0x3c, 0xa00),
            (0x40, 0xb00),
            (0x48, 0xc00),
            (0x50, 0xd00),
            (0x58, 0xe00),
        ] {
            c.bus.write32(D + off, v);
        }
        let n4c = r.next_u32() % 4;
        let n44 = r.next_u32() % 4;
        c.bus.write32(D + 0x20, r.next_u32() % 3);
        c.bus.write32(D + 0x28, r.next_u32() % 3);
        c.bus.write32(D + 0x38, r.next_u32() % 4);
        c.bus.write32(D + 0x44, n44);
        c.bus.write32(D + 0x4c, n4c);
        c.bus.write32(D + 0x54, r.next_u32() % 3);
        // +0x3c: word offsets; +0x600/+0x800: records (field +8 an offset).
        for i in 0..4u32 {
            c.bus.write32(D + 0xa00 + 4 * i, 0x20 * i);
            c.bus
                .write16(D + 0xc00 + 2 * i, (r.next_u32() % n4c.max(1)) as u16);
            // +0x50 table: offsets from the +0x40 base to the entries.
            c.bus.write32(D + 0xd00 + 4 * i, 0x100 * i);
            let e = D + 0xb00 + 0x100 * i;
            c.bus
                .write32(e, [0u32, 1, 3, 2][(r.next_u32() % 4) as usize]);
            let slot = r.next_u32() % 4;
            c.bus.write32(e + 0xc, slot | (r.next_u32() % 4) << 8);
            c.bus
                .write32(D + 0x600 + 0x54 * i + 8, r.next_u32() % 0x100);
            c.bus
                .write32(D + 0x800 + 0x40 * i + 8, r.next_u32() % 0x100);
            c.bus.write32(e + 4, r.next_u32());
            // +0x58: two halfwords per spline.
            c.bus.write32(D + 0xe00 + 4 * i, r.next_u32());
        }
        // World cache records.
        for s in 0..3u32 {
            let rec = DATA + 0x1400 + 0x80 * s;
            c.bus.write32(slots + 4 * s, if s == 2 { 0 } else { rec });
            for (off, base) in [(0x1cu32, 0x1800u32), (0x44, 0x1900), (0x64, 0x1a00)] {
                let table = DATA + base + 0x20 * s;
                c.bus.write32(rec + off, table);
                for k in 0..4u32 {
                    let obj = DATA + 0x2000 + 0x100 * ((s + k) % 4);
                    let word = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        (obj >> 2) << 8
                    };
                    c.bus.write32(table + 4 * k, word);
                }
            }
        }
        c.g[A0][0] = sx(entry);
        c.g[A1][0] = sx(wp);
        c.g[A2][0] = [1u64, 0, 2, 5][(r.next_u32() % 4) as usize];
    }

    #[test]
    fn t_wscript_cache_relocate_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc420_hook_mem_free();
        covered(
            ee::s13::f_003accd8__generated,
            t_wscript_cache_relocate,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_ccd8, 1);
                c.pc = 0x003a_ccd8;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                oc420_relocate_setup(c, &mut r, seed, seed % 4);
            },
        );
        check_events(
            ee::s13::f_003accd8__generated,
            t_wscript_cache_relocate,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_ccd9, 0);
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                oc420_relocate_setup(c, &mut r, seed, 2);
                schedule(c, &mut r, seed, 0x003a_ccd8)
            },
        );
    }

    // ------------------------------------------ ScriptEngine_GetTargetObject

    /// Every slot 0..22 (21 and 22 out of range), with the jump table at
    /// 0x481ca0 in RAM (one entry sometimes an unknown address, which both
    /// sides send to `jump_out`), the level counts and tables, and the
    /// instance-script indices (negative, valid).
    #[test]
    fn script_engine_get_target_object_matches() {
        check(
            ee::s07::f_00279f18__generated,
            script_engine_get_target_object,
            460,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_9f18, 2);
                c.pc = 0x0027_9f18;
                let slot = seed % 23;
                let labels: [u32; 21] = [
                    0x0027_9f48,
                    0x0027_9f68,
                    0x0027_9f7c,
                    0x0027_9fac,
                    0x0027_9fac,
                    0x0027_9fe8,
                    0x0027_9fe8,
                    0x0027_9fe8,
                    0x0027_9fe8,
                    0x0027_9fe8,
                    0x0027_a034,
                    0x0027_a034,
                    0x0027_a034,
                    0x0027_a034,
                    0x0027_a034,
                    0x0027_a098,
                    0x0027_a098,
                    0x0027_a098,
                    0x0027_a098,
                    0x0027_a098,
                    0x0027_a098,
                ];
                for (i, l) in labels.iter().enumerate() {
                    c.bus.write32(0x0048_1ca0 + 4 * i as u32, *l);
                }
                if seed % 29 == 0 {
                    c.bus.write32(0x0048_1ca0 + 4 * slot.min(20), 0x0027_9f10);
                }
                let (world, man, lvl) = (DATA + 0x500, DATA + 0x540, DATA + 0x600);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0xc, lvl);
                for (off, hi) in [(0x78u32, 6u32), (0x7c, 3), (0x80, 4), (0x84, 4)] {
                    c.bus.write32(lvl + off, r.next_u32() % hi);
                }
                for j in 0..8u32 {
                    let obj = DATA + 0x800 + 0x40 * j;
                    c.bus.write32(obj + 0x18, r.next_u32());
                    for off in [0x28u32, 0x40, 0x48, 0x5c] {
                        let v = if r.next_u32() % 4 == 0 { 0 } else { obj };
                        c.bus.write32(lvl + off + 4 * j, v);
                    }
                }
                // The instance script (player -> +0xc -> +0xc).
                let (player, mid, script) = (DATA + 0x700, DATA + 0x740, DATA + 0x780);
                c.bus.write32(player + 0xc, mid);
                c.bus.write32(mid + 0xc, script);
                for k in 0..3u32 {
                    let idx = [0xffff_ffffu32, 0, 1, 2, 5, 7][(r.next_u32() % 6) as usize];
                    c.bus.write32(script + 4 * k, idx);
                }
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(player);
            },
        );
    }
}
