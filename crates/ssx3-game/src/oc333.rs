//! Staging module oc333 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc333`.

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
    (0x003d_6d50, spch_find_bank_by_id, "SPCH_FindBankById"),
    (
        0x003c_0be8,
        snd_iop_release_channel_callback,
        "SND_IopReleaseChannelCallback",
    ),
    (
        0x0034_fcc0,
        world_obj_run_kill_script,
        "WorldObj_RunKillScript",
    ),
    (0x003c_93f8, snd_dec_utk_free, "SND_DecUtk_Free"),
    (
        0x0034_fd00,
        world_obj_kill_restorable,
        "WorldObj_KillRestorable",
    ),
    (
        0x0029_7438,
        ssx_audio_system_stop_wipeout_loop_2,
        "SSXAudioSystem_StopWipeoutLoop2",
    ),
    (
        0x0029_6e20,
        ssx_audio_system_stop_wipeout_loop,
        "SSXAudioSystem_StopWipeoutLoop",
    ),
    (0x003c_ddb8, snd_utk_reset, "SND_UtkReset"),
    (
        0x0034_51c0,
        spline_modifier_init_path,
        "SplineModifier_InitPath",
    ),
    (
        0x0035_3398,
        modifier_block_take_particle_list,
        "ModifierBlock_TakeParticleList",
    ),
];

// ---------------------------------------------------------------------------
// SPCH_FindBankById (0x3d6d50).

/// `D_004A4824` (`gp + 0x1734`): the speech bank pointer table, 8 slots.
pub(crate) const SPCH_FIND_BANK_TABLE: u32 = 0x1734;

/// `D_004A4828` (`gp + 0x1738`): the number of used table slots.
pub(crate) const SPCH_FIND_BANK_COUNT: u32 = 0x1738;

/// `int SPCH_FindBankById(u16 id)` (0x3d6d50): walk the bank table and
/// return the index of the first bank whose id word (+0) is `id`, else -1.
/// The slot offset counts in 16.16 fixed point (`a2 += 0x10000`, the index
/// being the high half) so the loop's exit test reads the old index.
pub fn spch_find_bank_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, GP).wrapping_add(SPCH_FIND_BANK_TABLE)).lw(c);
    let no_table = c.g[V1][0] == 0;
    c.g[A0][0] &= 0xffff;
    if no_table {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_FIND_BANK_COUNT)).lw(c);
    let no_banks = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = 0;
    if !no_banks {
        c.cycles += 4;
        c.g[A3][0] = c.g[V0][0];
        c.g[A2][0] = 0x1_0000;
        c.g[T0][0] = 0x1_0000;
        'scan: loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).lhu(c);
                let hit = c.g[V0][0] == c.g[A0][0];
                c.g[V0][0] = c.g[A2][0];
                if hit {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A1][0];
                    ret(c);
                    return;
                }
            } else {
                c.g[V0][0] = c.g[A2][0];
            }
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            c.g[A1][0] = sx(((r32(c, V0) as i32) >> 16) as u32);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, T0)));
            if !more {
                break 'scan;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_6d78);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_IopReleaseChannelCallback (0x3c0be8).

/// The IOP-resident sound state block (`0x510000 - 0x5718 = 0x50a8e8`,
/// built by the original as that sum; the test's fixture writes it).
#[allow(dead_code)]
pub(crate) const SND_RELEASE_STATE: u32 = 0x0050_a8e8;

/// The state block's channel-count byte, added to the callback's index.
pub(crate) const SND_RELEASE_STATE_B: u32 = 0x30;

/// The state block's free-channel byte, added to the callback's index.
pub(crate) const SND_RELEASE_STATE_A: u32 = 0x2f;

/// One voice channel's stride (in the channel table at state + 0x1f0).
pub(crate) const SND_RELEASE_CHANNEL_SIZE: u64 = 0x8c;

/// `void SND_IopReleaseChannelCallback(int index)` (0x3c0be8): release the
/// channel at `index` shifted by the two IOP state counters (the RPC that
/// the IOP side calls back with a slot number).
pub fn snd_iop_release_channel_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x51_0000;
        frame.save_ra(c, 0);
        c.g[T2][0] = c.g[A0][0];
        c.g[V1][0] = SND_RELEASE_CHANNEL_SIZE;
        c.g[T3][0] = c.g[V0][0];
        c.mult(0, V1, T2, V1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5718));
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SND_RELEASE_STATE_B)).lbu(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(SND_RELEASE_STATE_A)).lbu(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[RA][0] = sx(0x003c_0c0c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        SND_CHANNEL_RELEASE.call(c, 0x003c_0c0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObj_RunKillScript (0x34fcc0).

/// `void *WorldObj_RunKillScript(WorldObj *self)` (0x34fcc0): run the kill
/// script for the object's instance node (`*(self + 0x18)`), returning
/// whether one ran.
pub fn world_obj_run_kill_script(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        Some(0x18),
        crate::call::Target::Table(0x002d_19e8),
        0x0034_fcd0,
    );
}

// ---------------------------------------------------------------------------
// SND_DecUtk_Free (0x3c93f8).

/// `void SND_DecUtk_Free(UtkDecoder *self)` (0x3c93f8): free the sample
/// buffer (`self + 0x28`) when there is one. The frame's saved `ra` is
/// reloaded before the branch and again after the free, so both paths
/// return through the same `jr`.
pub fn snd_dec_utk_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x28)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            SND_UTK_FREE.call(c, 0x003c_9414)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObj_KillRestorable (0x34fd00).

/// The object's instance node.
pub(crate) const WORLDOBJ_KR_NODE: u32 = 0x18;

/// The instance node's flags word.
pub(crate) const WORLDOBJ_KR_FLAGS: u32 = 0x8;

/// The object's vtable.
pub(crate) const WORLDOBJ_KR_VTABLE: u32 = 0xc;

/// `void WorldObj_KillRestorable(WorldObj *self, bool runscript)`
/// (0x34fd00): optionally run the kill script (and keep the object when one
/// ran); otherwise in the instance node's flags word keep only bits 8-9
/// (`&= 0xffff0300`), fold the high half into the low one, set the dead bit
/// and ask the vtable slot +8 (state 3) to expire the object.
pub fn world_obj_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let run_script = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if run_script {
            c.cycles += 4;
            WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0034_fd1c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 16;
        let me = r32(c, S0);
        c.g[A1][0] = Addr(me.wrapping_add(WORLDOBJ_KR_NODE)).lw(c);
        let mask = 0xffff_ffff_ffff_0300u64;
        c.g[V1][0] = mask;
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(WORLDOBJ_KR_FLAGS)).lw(c);
        c.g[V0][0] &= c.g[V1][0];
        c.sw(r32(c, A1).wrapping_add(WORLDOBJ_KR_FLAGS), c.g[V0][0]);
        c.g[A0][0] = Addr(me.wrapping_add(WORLDOBJ_KR_NODE)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(WORLDOBJ_KR_FLAGS)).lw(c);
        c.g[V1][0] = sx(((r32(c, V0) as i32) >> 16) as u32);
        c.g[V0][0] |= c.g[V1][0];
        c.sw(r32(c, A0).wrapping_add(WORLDOBJ_KR_FLAGS), c.g[V0][0]);
        c.g[V1][0] = Addr(me.wrapping_add(WORLDOBJ_KR_NODE)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WORLDOBJ_KR_FLAGS)).lw(c);
        c.g[V0][0] |= 0x2;
        let null = c.g[S0][0] == 0;
        c.sw(r32(c, V1).wrapping_add(WORLDOBJ_KR_FLAGS), c.g[V0][0]);
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(me.wrapping_add(WORLDOBJ_KR_VTABLE)).lw(c);
            c.g[A1][0] = 0x3;
            let slot = crate::call::VSlot::read(c, c.g[V0][0], 0x8);
            c.g[A0][0] = sx(me.wrapping_add(slot.delta as u32));
            c.g[V1][0] = slot.func;
            call_ptr(c, slot.func as u32, 0x0034_fd7c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StopWipeoutLoop2 / _StopWipeoutLoop (0x297438 / 0x296e20).

/// `SSXAudioSystem_GetInstance()` (`gp + 0x410`).
pub(crate) const SSX_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

/// The sound system pointer inside the audio system.
pub(crate) const SSX_AUDIO_SOUND: u32 = 0x118;

/// The bank monitor inside the sound system.
pub(crate) const SSX_AUDIO_BANK_MONITOR: u32 = 0x1d8;

/// The second wipeout loop's bank-monitor handle in the rider.
pub(crate) const WIPEOUT_LOOP_2_HANDLE: u32 = 0x778;

/// The first wipeout loop's bank-monitor handle in the rider.
pub(crate) const WIPEOUT_LOOP_HANDLE: u32 = 0x774;

/// `void SSXAudioSystem_StopWipeoutLoop2(unused, rider)` (0x297438): when the
/// rider still has a second wipeout-loop handle (`+0x778`, negative for
/// none), stop it on the bank monitor with a 0.75 s fade and mark it unused.
pub fn ssx_audio_system_stop_wipeout_loop_2(c: &mut Cpu) {
    stop_wipeout_loop(c, WIPEOUT_LOOP_2_HANDLE, 0x0029_745c, 0x0029_747c);
}

/// `void SSXAudioSystem_StopWipeoutLoop(unused, rider)` (0x296e20): the same
/// for the first loop's handle (`+0x774`).
pub fn ssx_audio_system_stop_wipeout_loop(c: &mut Cpu) {
    stop_wipeout_loop(c, WIPEOUT_LOOP_HANDLE, 0x0029_6e44, 0x0029_6e64);
}

/// The shared body of the two: frame 0x20 (s0 = rider).
fn stop_wipeout_loop(c: &mut Cpu, handle_off: u32, ret_get: u32, ret_stop: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(handle_off)).lw(c);
        if (c.g[V0][0] as i64) < 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 13;
            call_ptr(c, SSX_AUDIO_GET_INSTANCE, ret_get)?;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SSX_AUDIO_SOUND)).lw(c);
            c.g[A2][0] = 0x1;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(handle_off)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            c.g[AT][0] = 0x3f40_0000;
            c.mtc1(1, 12); // f12 = 0.75 (fade seconds)
            c.g[RA][0] = sx(ret_stop);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(SSX_AUDIO_BANK_MONITOR));
            BANK_MONITOR_STOP_HANDLE.call(c, ret_stop)?;
            c.g[V0][0] = u64::MAX;
            c.sw(r32(c, S0).wrapping_add(handle_off), c.g[V0][0]);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkReset (0x3cddb8).

/// `void *SND_UtkReset(Utk *self)` (0x3cddb8): clear the decoder's queue
/// state and mark it running; returns `self`.
pub fn snd_utk_reset(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x1;
    c.sw(r32(c, A0).wrapping_add(0xd44), 0);
    c.sw(r32(c, A0).wrapping_add(0xd64), c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    c.sw(r32(c, A0).wrapping_add(0xd54), 0);
    c.sw(r32(c, A0).wrapping_add(0xd58), 0);
    c.sh(r32(c, A0).wrapping_add(0xd50), 0);
    c.sh(r32(c, A0).wrapping_add(0xd52), 0);
    c.sw(r32(c, A0).wrapping_add(0xd5c), 0);
    let ra = c.g[RA][0] as u32;
    c.sw(r32(c, A0).wrapping_add(0xd60), 0);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// SplineModifier_InitPath (0x3451c0).

/// `World_GetInstance()` (`gp + 0x16c8`, the instance cell); the test's
/// fixture sets it.
#[allow(dead_code)]
pub(crate) const SPLINE_INIT_GP_INSTANCE: u32 = 0x16c8;

/// The instance's array of model sets.
pub(crate) const SPLINE_INIT_SETS: u32 = 0x8;

/// A model set's reference array.
pub(crate) const SPLINE_INIT_REFS: u32 = 0x44;

/// A track node's value word.
pub(crate) const SPLINE_INIT_NODE_VALUE: u32 = 0x24;

/// A track reference's id (`self + 0`), index (`+4`), node (`+8`) and the
/// cached path length (`+0xc`).
pub(crate) const SPLINE_INIT_INDEX: u32 = 0x4;

pub(crate) const SPLINE_INIT_NODE: u32 = 0x8;

pub(crate) const SPLINE_INIT_LENGTH: u32 = 0xc;

/// `cSpline_calcLength` for a `cTrack`: the spline object is at node + 0x68,
/// its key count at +0x20 and its current key at +0x24 (the test's fixture
/// builds that node).
#[allow(dead_code)]
pub(crate) const SPLINE_INIT_SPLINE: u32 = 0x68;

/// `void SplineModifier_InitPath(SplineFollow *self, u32 id)` (0x3451c0):
/// store the track id, look the track node up in the instance's model sets
/// (`sets[id & 0xff]`, then `refs[id >> 8]`, then `ref << 2`), cache the
/// node's value at +8, clear the key index and store the path length.
pub fn spline_modifier_init_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_51d8);
        c.sw(r32(c, S0), c.g[A1][0]);
        call_ptr(c, 0x002d_1bd8, 0x0034_51d8)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c); // the world
        c.g[A1][0] = Addr(r32(c, S0)).lw(c); // the id
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(SPLINE_INIT_SETS)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c); // the model set
        let no_set = c.g[V1][0] == 0;
        c.g[V0][0] = sx(r32(c, A1) >> 8);
        if no_set {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(SPLINE_INIT_REFS)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let no_ref = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            if no_ref {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 10;
        c.g[V0][0] = Addr(r32(c, V0).wrapping_add(SPLINE_INIT_NODE_VALUE)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(SPLINE_INIT_INDEX), 0);
        c.g[RA][0] = sx(0x0034_5230);
        c.sw(r32(c, S0).wrapping_add(SPLINE_INIT_NODE), c.g[V0][0]);
        SPLINE_GET_PATH_LENGTH.call(c, 0x0034_5230)?;
        c.swc1(0, r32(c, S0).wrapping_add(SPLINE_INIT_LENGTH));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_TakeParticleList (0x353398).

/// `void ModifierBlock_TakeParticleList(ModifierBlock *self, List *list)`
/// (0x353398): for each particle node in `list`, call its vtable slot +0x18
/// method on `node + delta`, push the node onto the block's list
/// (`self + 0x10`), then clear the source list's head and tail.
pub fn modifier_block_take_particle_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(r32(c, S2)).lw(c);
        let empty = c.g[S1][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
            'nodes: loop {
                c.cycles += 11;
                c.g[S0][0] = Addr(r32(c, S1)).lw(c);
                let slot = crate::call::VSlot::read(c, c.g[V0][0], 0x18);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(slot.delta as u32));
                c.g[V1][0] = slot.func;
                call_ptr(c, slot.func as u32, 0x0035_33dc)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0035_33e8);
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x10));
                MODIFIER_LIST_PUSH_BACK.call(c, 0x0035_33e8)?;
                c.g[S1][0] = c.g[S0][0];
                if c.g[S1][0] == 0 {
                    break 'nodes;
                }
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_33c8);
                }
            }
        }
        c.cycles += 9;
        c.sw(r32(c, S2).wrapping_add(0x4), 0);
        c.sw(r32(c, S2), 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Callees.

/// `SND_ChannelRelease(int index)` 0x3ba550.
pub(crate) const SND_CHANNEL_RELEASE: Callee = Callee {
    addr: 0x003b_a550,
    f: ee::s13::SND_ChannelRelease,
};

/// `SND_UtkFree(void *buffer)` 0x3cd1d0.
pub(crate) const SND_UTK_FREE: Callee = Callee {
    addr: 0x003c_d1d0,
    f: ee::s14::f_003cd1d0,
};

/// `WorldObj_RunKillScript(WorldObj *self)` 0x34fcc0.
pub(crate) const WORLD_OBJ_RUN_KILL_SCRIPT: Callee = Callee {
    addr: 0x0034_fcc0,
    f: ee::s11::WorldObj_RunKillScript,
};

/// `BankMonitor_StopHandle(monitor, handle, kind, fade)` 0x2ad5f0.
pub(crate) const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// `SplineModifier_GetPathLength(cSpline *self)` 0x3454e8.
pub(crate) const SPLINE_GET_PATH_LENGTH: Callee = Callee {
    addr: 0x0034_54e8,
    f: ee::s11::SplineModifier_GetPathLength,
};

/// `ModifierList_PushBack(List *list, void *node)` 0x345720.
pub(crate) const MODIFIER_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0034_5720,
    f: ee::s11::f_00345720,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// The `WScriptMan` fakes are process-global slots, so this module's two
    /// tests that install them run one at a time (a concurrent `Drop` would
    /// put the generated body back under the other's feet).
    static WSCRIPT_SLOT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn lock_wscript_slots() -> std::sync::MutexGuard<'static, ()> {
        WSCRIPT_SLOT_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// `SPCH_FindBankById`: no table, an empty table, and 1-6 slots with
    /// null banks and bank ids that match the request or not. The request's
    /// high half is junk (the function masks it).
    #[test]
    fn spch_find_bank_by_id_matches() {
        const BANKS: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x200;
        check(
            ee::s14::f_003d6d50__generated,
            spch_find_bank_by_id,
            240,
            &|c, seed| {
                let mut r = Rng(0x3d6d_5000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_6d50;
                c.g[GP][0] = GAME_GP as u64;
                let count = if seed % 5 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 6) as i32
                };
                c.bus.write32(
                    GAME_GP + SPCH_FIND_BANK_TABLE,
                    if seed % 7 == 0 { 0 } else { TABLE },
                );
                c.bus.write32(GAME_GP + SPCH_FIND_BANK_COUNT, count as u32);
                let id = r.next_u32() as u16;
                let sel = (r.next_u32() % 6) as u32;
                for i in 0..6u32 {
                    let bank = BANKS + 0x40 * i;
                    c.bus
                        .write32(TABLE + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { bank });
                    c.bus.write16(
                        bank,
                        if i == sel && seed % 3 != 0 {
                            id
                        } else {
                            r.next_u32() as u16
                        },
                    );
                }
                c.g[A0][0] = sx((r.next_u32() & 0xffff_0000) | id as u32);
            },
        );
    }

    /// `SND_IopReleaseChannelCallback`: the IOP state block's two counters
    /// (0..3) and a small callback index, so every channel the release
    /// touches is a free one in a zeroed channel table.
    #[test]
    fn snd_iop_release_channel_callback_matches() {
        use crate::difftest::fakes::machine;
        const TABLE: u32 = DATA + 0x1000;
        const CHANNELS: u32 = 0x1f0;
        check(
            ee::s13::f_003c0be8__generated,
            snd_iop_release_channel_callback,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c0be8, 0);
                c.pc = 0x003c_0be8;
                c.bus.write32(SND_RELEASE_STATE + CHANNELS, TABLE);
                c.bus.write8(
                    SND_RELEASE_STATE + SND_RELEASE_STATE_B,
                    (r.next_u32() % 4) as u8,
                );
                c.bus.write8(
                    SND_RELEASE_STATE + SND_RELEASE_STATE_A,
                    (r.next_u32() % 4) as u8,
                );
                c.g[A0][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// `WorldObj_RunKillScript`: the instance node's kill script (the faked
    /// `WScriptMan` looks up the id it is handed).
    #[test]
    fn world_obj_run_kill_script_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::game::lib::snd::tests::{WscriptFakeSlots, FAKE_CTRL, FAKE_MAGIC};
        use crate::game::util::menu::WSCRIPT_MANAGER_GP;
        let _guard = lock_wscript_slots();
        let _slots = WscriptFakeSlots::install();
        check(
            ee::s11::f_0034fcc0__generated,
            world_obj_run_kill_script,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fcc0, 1);
                c.pc = 0x0034_fcc0;
                c.bus.write32(FAKE_CTRL, FAKE_MAGIC);
                let manager = DATA + 0x100;
                fill(c, &mut r, manager + 0x280, 8);
                c.bus.write32(GAME_GP + WSCRIPT_MANAGER_GP, manager);
                c.bus.write32(manager + 0x28c, DATA + 0x200);
                c.bus.write32(LOG_RESULTS, (seed % 2) as u32);
                c.bus.write32(DATA + 0x400 + 0x18, DATA + 0x500);
                c.g[A0][0] = sx(DATA + 0x400);
            },
        );
    }

    /// `SND_DecUtk_Free`: a decoder whose buffer word (+0x28) is null or a
    /// pointer; the free goes through `SND_UtkFree`, whose one indirect call
    /// is a fake.
    #[test]
    fn snd_dec_utk_free_matches() {
        use crate::difftest::fakes::machine;
        check(ee::s14::f_003c93f8__generated, snd_dec_utk_free, 160, &|c, seed| {
            let _ = machine(c, seed, 0x3c93f8, 1);
            c.pc = 0x003c_93f8;
            let dec = DATA + 0x100;
            c.bus
                .write32(dec + 0x28, if seed % 3 == 0 { 0 } else { DATA + 0x400 });
            c.g[A0][0] = sx(dec);
        });
    }

    /// `WorldObj_KillRestorable`: an object with an instance (random flags
    /// word) and a vtable whose slot +8 is a fake; `a1` asks for the kill
    /// script (which is faked to find one or not), and on 1 in 8 seeds the
    /// object is null.
    #[test]
    fn world_obj_kill_restorable_matches() {
        use crate::difftest::fakes::covered;
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::vtable;
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::game::lib::snd::tests::{WscriptFakeSlots, FAKE_CTRL, FAKE_MAGIC};
        use crate::game::util::menu::WSCRIPT_MANAGER_GP;
        const ME: u32 = DATA + 0x400;
        const NODE: u32 = DATA + 0x800;
        let _guard = lock_wscript_slots();
        let _slots = WscriptFakeSlots::install();
        covered(
            ee::s11::f_0034fd00__generated,
            world_obj_kill_restorable,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fd00, 2);
                c.pc = 0x0034_fd00;
                c.bus.write32(FAKE_CTRL, FAKE_MAGIC);
                let manager = DATA + 0x100;
                fill(c, &mut r, manager + 0x280, 8);
                c.bus.write32(GAME_GP + WSCRIPT_MANAGER_GP, manager);
                c.bus.write32(manager + 0x28c, DATA + 0x200);
                c.bus.write32(LOG_RESULTS, (seed % 2) as u32);
                c.bus.write32(ME + 0x18, NODE);
                c.bus.write32(NODE + 8, r.next_u32());
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(ME + 0xc, vt);
                c.g[A0][0] = if seed % 8 == 0 { 0u64 } else { sx(ME) };
                c.g[A1][0] = ((seed / 2) % 2) as u64;
            },
        );
    }

    /// A rider with the wipeout-loop handle (`off`) at -1 or a small id,
    /// its audio system (`gp + 0x410` -> sound -> bank monitor) and 64 free
    /// bank-monitor instances (so the stop's instance search walks them).
    fn wipeout_loop_setup(c: &mut Cpu, seed: u32, off: u32) {
        use crate::difftest::fakes::machine;
        const AUDIO: u32 = DATA + 0x100;
        const PTR_CELL: u32 = DATA + 0x200;
        const SND: u32 = DATA + 0x300;
        const MONITOR: u32 = DATA + 0x1000;
        const INSTANCES: u32 = DATA + 0x4000;
        let mut r = machine(c, seed, 0x297438, 0);
        c.bus.write32(GAME_GP + 0x410, AUDIO);
        c.bus.write32(AUDIO + SSX_AUDIO_SOUND, PTR_CELL);
        c.bus.write32(PTR_CELL, SND);
        c.bus.write32(SND + SSX_AUDIO_BANK_MONITOR, MONITOR);
        c.bus.write32(MONITOR + 0x8ec, INSTANCES);
        let handle = if seed % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() & 0x7fff_ffff
        };
        c.bus.write32(DATA + 0x800 + off, handle);
        c.g[A0][0] = sx(DATA + 0x600);
        c.g[A1][0] = sx(DATA + 0x800);
    }

    #[test]
    fn ssx_audio_system_stop_wipeout_loop_2_matches() {
        check(
            ee::s08::f_00297438__generated,
            ssx_audio_system_stop_wipeout_loop_2,
            160,
            &|c, seed| {
                wipeout_loop_setup(c, seed, WIPEOUT_LOOP_2_HANDLE);
                c.pc = 0x0029_7438;
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_wipeout_loop_matches() {
        check(
            ee::s08::f_00296e20__generated,
            ssx_audio_system_stop_wipeout_loop,
            160,
            &|c, seed| {
                wipeout_loop_setup(c, seed, WIPEOUT_LOOP_HANDLE);
                c.pc = 0x0029_6e20;
            },
        );
    }

    /// `SND_UtkReset`: the eight queue words after an object at a random
    /// (16-byte aligned) address.
    #[test]
    fn snd_utk_reset_matches() {
        check(ee::s14::f_003cddb8__generated, snd_utk_reset, 100, &|c, seed| {
            let mut r = Rng(0x3cdd_b800 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003c_ddb8;
            c.g[A0][0] = sx(DATA + (r.next_u32() & 0x3_fff0));
        });
    }

    /// `SplineModifier_InitPath`: a self with a track id (its low byte
    /// indexing the world's model sets, its high byte the set's refs) whose
    /// lookup finds a node, no set, or a ref with no pointer; `SND`-style
    /// spline data behind the node keeps `GetPathLength` short.
    #[test]
    fn spline_modifier_init_path_matches() {
        use crate::difftest::fakes::machine;
        const SELF: u32 = DATA;
        const INSTANCE: u32 = DATA + 0x100;
        const WORLD: u32 = DATA + 0x200;
        const SETS: u32 = DATA + 0x300;
        const SET: u32 = DATA + 0x400;
        const REFS: u32 = DATA + 0x500;
        const NODE: u32 = DATA + 0x800;
        const SPLINE: u32 = DATA + 0x1000;
        const LIST: u32 = DATA + 0x1100;
        check(
            ee::s11::f_003451c0__generated,
            spline_modifier_init_path,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3451c0, 0);
                c.pc = 0x0034_51c0;
                // World_GetInstance() -> a cell holding the world pointer.
                c.bus.write32(GAME_GP + SPLINE_INIT_GP_INSTANCE, INSTANCE);
                c.bus.write32(INSTANCE, WORLD);
                c.bus.write32(WORLD + SPLINE_INIT_SETS, SETS);
                let id = ((r.next_u32() % 3) << 8) | (r.next_u32() % 3);
                let no_set = seed % 5 == 0;
                c.bus
                    .write32(SETS + 4 * (id & 0xff), if no_set { 0 } else { SET });
                c.bus.write32(SET + SPLINE_INIT_REFS, REFS);
                // refToPtr: (refs[idx] >> 8) << 2 = the node address.
                let refs = match seed % 3 {
                    0 => 0,
                    _ => (NODE >> 2) << 8,
                };
                c.bus.write32(REFS + 4 * (id >> 8), refs);
                c.bus.write32(NODE + SPLINE_INIT_NODE_VALUE, r.next_u32());
                c.bus.write32(NODE + SPLINE_INIT_SPLINE, SPLINE);
                c.bus.write32(SPLINE + 0x20, r.next_u32() % 2);
                c.bus.write32(SPLINE + 0x24, LIST);
                for k in 0..0x40 {
                    c.bus.write32(LIST + 4 * k, r.f32(4.0));
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(id);
            },
        );
    }

    /// `ModifierBlock_TakeParticleList`: an empty list, or 1-3 particle
    /// nodes with vtable slot +0x18 methods (fakes); the block's own list
    /// starts empty or holds an entry.
    #[test]
    fn modifier_block_take_particle_list_matches() {
        use crate::difftest::fakes::covered;
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::vtable;
        const BLOCK: u32 = DATA + 0x100;
        const LIST: u32 = DATA + 0x200;
        const BLOCK_LIST: u32 = DATA + 0x300;
        const NODES: u32 = DATA + 0x1000;
        covered(
            ee::s11::f_00353398__generated,
            modifier_block_take_particle_list,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353398, 4);
                c.pc = 0x0035_3398;
                c.bus.write32(BLOCK + 0x10, BLOCK_LIST);
                let n = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                let mut prev = 0u32;
                for k in 0..n {
                    let node = NODES + 0x100 * k;
                    if k == 0 {
                        c.bus.write32(LIST, node);
                    } else {
                        c.bus.write32(prev, node);
                    }
                    c.bus.write32(node, 0);
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(node + 8, vt);
                    prev = node;
                }
                c.bus.write32(LIST + 4, if n == 0 { 0 } else { NODES });
                c.g[A0][0] = sx(BLOCK);
                c.g[A1][0] = sx(LIST);
            },
        );
    }
}
