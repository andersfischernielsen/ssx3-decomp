//! Staging wave 212 (oc305): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

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
    (0x0027_0870, replay_on_rider_land, "Replay_OnRiderLand"),
    (0x0027_0538, replay_prune_snapshots, "Replay_PruneSnapshots"),
    (0x0035_3150, modifier_block_free_tex_flip, "ModifierBlock_FreeTexFlip"),
    (0x0030_b228, wscript_process_list_count, "WScriptProcessList_Count"),
    (
        0x0028_8c08,
        ssx_audio_system_is_any_view_in_range,
        "SSXAudioSystem_IsAnyViewInRange",
    ),
    (0x002f_c420, wscript_fn_make_dead_node, "WScriptFn_MakeDeadNode"),
    (0x0012_3128, rider_get_character, "Rider_GetCharacter"),
    (0x0035_7000, position_modifier_save, "PositionModifier_Save"),
];

// ---------------------------------------------------------------------------
// Replay_OnRiderLand (0x270870)

/// `Replay` flags that block highlight bookkeeping on landing: `+0x61c`
/// (playing back), `+0x620` (instant replay), then the replay state at `+0`.
pub(crate) const OC305_REPLAY_LAND_BUSY_FLAGS: [u32; 3] = [0x61c, 0x620, 0x0];

/// `Replay_FindRiderIndex` (0x270730).
pub(crate) const OC305_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};

/// `Replay_EndHighlight` (0x270ce8).
pub(crate) const OC305_REPLAY_END_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0ce8,
    f: ee::s07::Replay_EndHighlight,
};

/// `Replay_CheckInstantReplay` (0x2705b0).
pub(crate) const OC305_REPLAY_CHECK_INSTANT_REPLAY: Callee = Callee {
    addr: 0x0027_05b0,
    f: ee::s07::Replay_CheckInstantReplay,
};

/// `void Replay_OnRiderLand(Replay* self, int riderId, void* data)`
/// (0x270870): when the replay is idle (none of `+0x61c`, `+0x620`, `+0`
/// set), find the rider's index and, when found, close its highlight
/// (`Replay_EndHighlight(self, index, data, 1)`) and check whether the
/// landing earns an instant replay (`Replay_CheckInstantReplay(self, data,
/// 1)`).
pub fn replay_on_rider_land(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC305_REPLAY_LAND_BUSY_FLAGS[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set (or the full path): one more cycle at the tail.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC305_REPLAY_LAND_BUSY_FLAGS[1..].iter().copied() {
                c.cycles += 3;
                c.g[V0][0] = Addr(this).off(off).lw(c);
                if c.g[V0][0] != 0 {
                    busy = true;
                    break;
                }
            }
            if !busy {
                tail = true;
                c.cycles += 4;
                OC305_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_08b4)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[V0][0];
                if found {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 1;
                    OC305_REPLAY_END_HIGHLIGHT.call(c, 0x0027_08cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 1;
                    OC305_REPLAY_CHECK_INSTANT_REPLAY.call(c, 0x0027_08dc)?;
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_PruneSnapshots (0x270538)

/// `Replay` live snapshot list head (`+0x3b0`).
pub(crate) const OC305_REPLAY_LIVE_SNAPSHOTS: u32 = 0x3b0;
/// Snapshot node: next (`+0x14`), and the keep markers: a halfword at
/// `+0x1c` and bytes at `+0x1f` and `+0x1e`.
pub(crate) const OC305_SNAPSHOT_NEXT: u32 = 0x14;
pub(crate) const OC305_SNAPSHOT_KEEP_HALF: u32 = 0x1c;
pub(crate) const OC305_SNAPSHOT_KEEP_B: u32 = 0x1f;
pub(crate) const OC305_SNAPSHOT_KEEP_A: u32 = 0x1e;

/// `Replay_FreeSnapshot` (0x270238).
pub(crate) const OC305_REPLAY_FREE_SNAPSHOT: Callee = Callee {
    addr: 0x0027_0238,
    f: ee::s07::Replay_FreeSnapshot,
};

/// `void Replay_PruneSnapshots(Replay* self)` (0x270538): walk the live
/// snapshot list and free (`Replay_FreeSnapshot(self, node)`) every snapshot
/// nothing keeps (its halfword at `+0x1c` and bytes at `+0x1f`/`+0x1e` all
/// zero). The next pointer is read before the free unlinks the node.
pub fn replay_prune_snapshots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S1)).off(OC305_REPLAY_LIVE_SNAPSHOTS).lw(c);
        frame.restore(c, S0, 0x20);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(OC305_SNAPSHOT_KEEP_HALF).lhu(c);
            loop {
                // The node and its keep halfword live in a1/v0 (an event at
                // the loop head sees them there).
                let node = Addr(r32(c, A1));
                c.cycles += 2;
                c.g[S0][0] = node.off(OC305_SNAPSHOT_NEXT).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = node.off(OC305_SNAPSHOT_KEEP_B).lbu(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = node.off(OC305_SNAPSHOT_KEEP_A).lbu(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S1][0];
                            OC305_REPLAY_FREE_SNAPSHOT.call(c, 0x0027_0588)?;
                            c.cycles += 1;
                        }
                    }
                }
                c.g[A1][0] = c.g[S0][0];
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).off(OC305_SNAPSHOT_KEEP_HALF).lhu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0560);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeTexFlip (0x353150)

/// `ModifierBlock`: the owned texture-flip modifier (`+0x8`).
pub(crate) const OC305_MODIFIER_BLOCK_TEX_FLIP: u32 = 0x8;
/// `__builtin_delete` (0x317e50).
pub(crate) const OC305_BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `void ModifierBlock_FreeTexFlip(ModifierBlock* self)` (0x353150): delete
/// the block's texture-flip modifier (`+0x8`), if any, and clear the field.
pub fn modifier_block_free_tex_flip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(OC305_MODIFIER_BLOCK_TEX_FLIP).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, OC305_BUILTIN_DELETE, 0x0035_3174)?;
            Addr(r32(c, S0)).off(OC305_MODIFIER_BLOCK_TEX_FLIP).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptProcessList_Count (0x30b228)

/// `cWScriptListNode`: the next process (`+0x18`).
pub(crate) const OC305_WSCRIPT_PROCESS_NEXT: u32 = 0x18;

/// `int WScriptProcessList_Count(WScriptProcessList* self)` (0x30b228): the
/// number of processes in the list (head at `+0`, linked through `+0x18`).
pub fn wscript_process_list_count(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        loop {
            // The node and count live in a0/v0 (an event at the loop head
            // sees them there).
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, A0)).off(OC305_WSCRIPT_PROCESS_NEXT).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_b238);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsAnyViewInRange (0x288c08)

/// Entry of `SSXAudioSystem_IsAnyViewInRange`.
pub(crate) const OC305_VIEW_IN_RANGE_ENTRY: u32 = 0x0028_8c08;
/// Loop head (the per-view body), where the function resumes after an
/// unwind at the back-edge event check.
pub(crate) const OC305_VIEW_IN_RANGE_HEAD: u32 = 0x0028_8c48;
/// End of the function (for `jump_out`).
pub(crate) const OC305_VIEW_IN_RANGE_END: u32 = 0x0028_8ce8;
/// Metres to world units (100.0f).
pub(crate) const OC305_VIEW_IN_RANGE_UNITS: u32 = 0x42c8_0000;

/// `SSXAudioSystem_GetViewPos` (0x288ac0).
pub(crate) const OC305_SSX_AUDIO_GET_VIEW_POS: Callee = Callee {
    addr: 0x0028_8ac0,
    f: ee::s07::SSXAudioSystem_GetViewPos,
};

/// `SSXAudioSystem_GetListenerCount` (0x288ce8).
pub(crate) const OC305_SSX_AUDIO_GET_LISTENER_COUNT: Callee = Callee {
    addr: 0x0028_8ce8,
    f: ee::s07::SSXAudioSystem_GetListenerCount,
};

/// `bool SSXAudioSystem_IsAnyViewInRange(SSXAudioSystem* self, const Vec4*
/// pos, float radius)` (0x288c08): whether any listener view lies within
/// `radius` metres (`radius * 100` units, compared squared) of `pos` with
/// the view in front of it (`(view - pos).z > 0`). Frame 0x70 (ra at 0x20,
/// s0..s2 at 0x50/0x40/0x30, f20/f21 at 0x60/0x68; the difference vector
/// spilled at 0x10 and 0x0). Loop state: s0 = view index, s1 = self, s2 =
/// pos, f20 = radius squared, f21 = 0. Dispatcher-shaped: re-entered at
/// [`OC305_VIEW_IN_RANGE_HEAD`] after an unwind at the loop's event check.
pub fn ssx_audio_system_is_any_view_in_range(c: &mut Cpu) {
    use crate::abi::{c_lt_s, set_fpr};
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        // Entering at the head resumes inside the loop body.
        let mut at_head = match c.pc {
            OC305_VIEW_IN_RANGE_ENTRY => {
                c.cycles += 16;
                let frame = Frame::open(c, 0x70);
                c.g[AT][0] = sx(OC305_VIEW_IN_RANGE_UNITS);
                c.f[0] = OC305_VIEW_IN_RANGE_UNITS;
                frame.save_fpr(c, 20, 0x60);
                let r = crate::abi::farg(c, 0) * FpuF32(c.f[0]);
                set_fpr(c, 20, r);
                frame.save_fpr(c, 21, 0x68);
                frame.save(c, S0, 0x50);
                frame.save(c, S1, 0x40);
                c.g[S0][0] = 0;
                set_fpr(c, 20, r * r);
                frame.save(c, S2, 0x30);
                c.f[21] = 0;
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0x20);
                c.g[S2][0] = c.g[A1][0];
                false
            }
            OC305_VIEW_IN_RANGE_HEAD => true,
            pc => {
                c.jump_out(pc, OC305_VIEW_IN_RANGE_ENTRY, OC305_VIEW_IN_RANGE_END);
                return Ok(());
            }
        };
        let mut found = false;
        loop {
            if !at_head {
                // 0x288cb0: more views?
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                OC305_SSX_AUDIO_GET_LISTENER_COUNT.call(c, 0x0028_8cb8)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.g[S1][0];
                if event_due(c) && crate::events::back_edge_unwinds(c, OC305_VIEW_IN_RANGE_HEAD) {
                    return Ok(());
                }
            }
            at_head = false;
            // 0x288c48: d = view position - pos; |d|^2 through VU0.
            c.cycles += 21;
            c.g[A1][0] = c.g[S0][0];
            OC305_SSX_AUDIO_GET_VIEW_POS.call(c, 0x0028_8c50)?;
            let view = r32(c, V0);
            let pos = r32(c, S2);
            let spill_hi = frame.at(c, 0x10);
            let spill = frame.at(c, 0);
            c.lqc2(3, view);
            c.lqc2(4, pos);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, spill_hi);
            c.lq(V0, spill_hi);
            c.sq(V0, spill);
            c.lqc2(3, spill);
            c.lqc2(5, spill);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V1, 4);
            c.f[0] = c.g[V1][0] as u32;
            let near = c_lt_s(c, FpuF32(c.f[0]), FpuF32(c.f[20]));
            c.lwc1(0, spill + 8);
            if near {
                // 0x288c9c: in front of the listener?
                c.cycles += 4;
                let ahead = c_lt_s(c, FpuF32(c.f[21]), FpuF32(c.f[0]));
                c.g[V0][0] = 1;
                if ahead {
                    found = true;
                    break;
                }
            }
            // 0x288cac
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        }
        if !found {
            // 0x288cc4
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // 0x288cc8
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_MakeDeadNode (0x2fc420)

/// gp-relative guard of the static default parameter block.
pub(crate) const OC305_DEAD_NODE_DEFAULTS_READY_GP: u32 = 0x24f4;
/// The static default parameter block `{id = -1, 0, tag}` (12 bytes;
/// `0x50_0000 - 0x4b00`).
pub(crate) const OC305_DEAD_NODE_DEFAULTS: u32 = 0x004f_b500;
/// gp-relative: parameter kinds by slot (`D_004A3C00`); kind 2 is a float.
pub(crate) const OC305_DEAD_NODE_KINDS_GP: u32 = 0xb10;
pub(crate) const OC305_DEAD_NODE_KIND_FLOAT: u64 = 2;
/// gp-relative: the world-object directory and the world (its default
/// object at `+0x290`).
pub(crate) const OC305_DEAD_NODE_DIRECTORY_GP: u32 = 0x16c8;
pub(crate) const OC305_DEAD_NODE_WORLD_GP: u32 = 0xce8;
pub(crate) const OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT: u32 = 0x290;
/// Object: its active node (`+0xc`).
pub(crate) const OC305_DEAD_NODE_OBJECT_NODE: u32 = 0xc;
/// The factory's result value (12 bytes, returned by copy; `0x4d_0000 -
/// 0x6f68`).
pub(crate) const OC305_DEAD_NODE_RESULT: u32 = 0x004c_9098;
/// `EA_MemCopy` (0x3e6574).
pub(crate) const OC305_EA_MEM_COPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38).
pub(crate) const OC305_WOF_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `WorldObjFactory_DeadNode` (0x2fc2c0).
pub(crate) const OC305_WOF_DEAD_NODE: Callee = Callee {
    addr: 0x002f_c2c0,
    f: ee::s10::WorldObjFactory_DeadNode,
};

/// `Result WScriptFn_MakeDeadNode(Param* params, int count)` (0x2fc420; a0
/// = the hidden result slot, a1 = count, a2 = params): fill a 3-word
/// parameter block from the static defaults and the `{slot, value, -, kind}`
/// parameters (an int given for a float slot is converted), look the object
/// id up (-1: the world's default object), and unless the object's active
/// node already takes a kind-6 modifier for these parameters, make it a
/// dead node (`WorldObjFactory_DeadNode(object, block)`). Returns the
/// factory's result value. Frame 0x80 (ra at 0x20, s4..s0 at
/// 0x30..0x70, the block at 0x10).
pub fn wscript_fn_make_dead_node(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        let gp = c.g[GP][0] as u32;
        let ready = Addr(gp).off(OC305_DEAD_NODE_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x50_0000;
        frame.save_ra(c, 0x20);
        let defaults = Addr(OC305_DEAD_NODE_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 10;
            c.g[V1][0] = sx(defaults.0);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(8).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(8).sb(c, tag);
            Addr(gp).off(OC305_DEAD_NODE_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(4).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(defaults.0);
        c.g[A2][0] = 0xc;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, OC305_EA_MEM_COPY, 0x002f_c4a0)?;
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(OC305_DEAD_NODE_KINDS_GP));
            c.g[A3][0] = OC305_DEAD_NODE_KIND_FLOAT;
            loop {
                // Param cursor a2, count a1 (an event at the head sees them).
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S1)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c4b8);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let gp = c.g[GP][0] as u32;
        let id = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(gp).off(OC305_DEAD_NODE_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(gp).off(OC305_DEAD_NODE_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32).off(OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            let sections = Addr(directory as u32).lw(c);
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 8;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                0
            };
            c.g[A0][0] = object;
            c.cycles += 1;
            c.g[S0][0] = object;
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let node = Addr(r32(c, S0)).off(OC305_DEAD_NODE_OBJECT_NODE).lw(c);
            c.g[A0][0] = node;
            c.g[A1][0] = 6;
            let mut kill = true;
            if node != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                OC305_WOF_ATTACH_MODIFIER.call(c, 0x002f_c584)?;
                kill = c.g[V0][0] == 0;
            }
            if kill {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                OC305_WOF_DEAD_NODE.call(c, 0x002f_c598)?;
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr(r32(c, S4).wrapping_sub(0x4d_0000 - OC305_DEAD_NODE_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        let tail = result.off(8).lw(c);
        c.g[V1][0] = tail;
        let out = Addr(r32(c, S2));
        out.write_u64_unaligned(c, head);
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_GetCharacter (0x123128)

/// `Rider`: its AI / player index (`mAIIndex`, `+0x86c`).
pub(crate) const OC305_RIDER_AI_INDEX: u32 = 0x86c;
/// `BELibrary_GetInstance` (0x14dc80), `BELibrary_GetInterface` (0x14dd58)
/// and `BENewPlayerInt_GetRiderCharID` (0x147398).
pub(crate) const OC305_BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
pub(crate) const OC305_BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
pub(crate) const OC305_BE_NEW_PLAYER_GET_RIDER_CHAR_ID: u32 = 0x0014_7398;

/// `int Rider_GetCharacter(Rider* self)` (0x123128): the rider's character
/// id from the new-player back-end interface
/// (`BELibrary_GetInstance()->GetInterface(1)->GetRiderCharID(self->mAIIndex)`).
pub fn rider_get_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OC305_BE_LIBRARY_GET_INSTANCE, 0x0012_313c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, OC305_BE_LIBRARY_GET_INTERFACE, 0x0012_3148)?;
        c.g[A1][0] = Addr(r32(c, S0)).off(OC305_RIDER_AI_INDEX).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, OC305_BE_NEW_PLAYER_GET_RIDER_CHAR_ID, 0x0012_3154)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PositionModifier_Save (0x357000)

/// `PositionModifier`: the saved transform block (`+0x10`, 0x40 bytes).
pub(crate) const OC305_POSITION_MODIFIER_BLOCK: u32 = 0x10;
pub(crate) const OC305_POSITION_MODIFIER_BLOCK_SIZE: u64 = 0x40;
/// The stream's write method (vtable `{delta +0x8, fn +0xc}`).
pub(crate) const OC305_STREAM_WRITE_SLOT: u32 = 0x8;

/// `void PositionModifier_Save(PositionModifier* self, Stream* out)`
/// (0x357000): write the modifier's 0x40-byte transform block (`+0x10`)
/// through the stream's first virtual method (`out->Write(block, 0x40)`).
pub fn position_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A0).wrapping_add(OC305_POSITION_MODIFIER_BLOCK));
        c.g[A2][0] = OC305_POSITION_MODIFIER_BLOCK_SIZE;
        let stream = r32(c, A3);
        let vt = Addr(stream).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr(vt as u32).off(OC305_STREAM_WRITE_SLOT).lh(c);
        let method = Addr(vt as u32).off(OC305_STREAM_WRITE_SLOT + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0035_7028)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Replay_OnRiderLand`: the find-rider world of `Replay_FindRiderIndex`'s
    /// test (found, not found, no riders, fakes unwinding), each busy flag
    /// set or all clear, the rider's highlight slot free (the real
    /// `Replay_EndHighlight` returns early), and the real
    /// `Replay_CheckInstantReplay` with instant replays armed or not, one or
    /// more riders, the landing time close to or past the last one.
    #[test]
    fn replay_on_rider_land_matches() {
        use crate::game::replay::replay2::tests::replay_find_rider_fixture;
        crate::difftest::check(ee::s07::f_00270870__generated, replay_on_rider_land, 300, &|c, seed| {
            replay_find_rider_fixture(c, seed);
            let id = c.g[A0][0];
            let mut r = crate::difftest::Rng(0x2708_7000 ^ seed as u64);
            let this = crate::difftest::DATA + 0x3_0000;
            let data = crate::difftest::DATA + 0x3_9000;
            let busy = seed / 2 % 5;
            for (i, off) in OC305_REPLAY_LAND_BUSY_FLAGS.into_iter().enumerate() {
                let set = busy as usize == i + 1;
                c.bus.write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
            }
            // Highlight slots: all free.
            for k in 0..5u32 {
                c.bus.write32(this + 0x494 + 0xb4 * k + 0xb0, 0);
            }
            // Instant replay armed (`+0x3d4`), last times `+0x474/+0x47c`.
            c.bus.write32(this + 0x3d4, (seed % 3 != 0) as u32);
            let last = r.next_u32() % 5000;
            c.bus.write32(this + 0x474, last);
            c.bus.write32(this + 0x47c, last);
            let now = last.wrapping_add(if seed % 4 < 2 { 500 } else { 1500 });
            c.bus.write32(data + 0x9c, now);
            c.bus.write32(data + 0xa4, now);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = id;
            c.g[A2][0] = sx(data);
            c.pc = 0x0027_0870;
        });
    }

    /// `Replay_PruneSnapshots`: a live list of 0..6 snapshots (doubly
    /// linked, with count) and a free list, each node kept by its halfword,
    /// either byte, or nothing (freed through the real `Replay_FreeSnapshot`:
    /// unlink, stream reset, push onto the free list); events off, delivered,
    /// or unwinding at the loop head.
    #[test]
    fn replay_prune_snapshots_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(ee::s07::f_00270538__generated, replay_prune_snapshots, 240, &|c, seed| {
            let mut r = crate::difftest::Rng(0x2705_3800 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            let this = crate::difftest::DATA;
            let n = seed / 3 % 7;
            let node_at = |i: u32| crate::difftest::DATA + 0x1000 + 0x100 * i;
            for i in 0..n {
                let node = node_at(i);
                crate::difftest::fill(c, &mut r, node, 0x40);
                let next = if i + 1 < n { node_at(i + 1) } else { 0 };
                let prev = if i > 0 { node_at(i - 1) } else { 0 };
                c.bus.write32(node + OC305_SNAPSHOT_NEXT, next);
                c.bus.write32(node + 0x18, prev);
                let keep = r.next_u32() % 4;
                let v = 1 + r.next_u32() % 0xff;
                c.bus.write16(node + OC305_SNAPSHOT_KEEP_HALF, if keep == 1 { v as u16 } else { 0 });
                c.bus.write8(node + OC305_SNAPSHOT_KEEP_B, if keep == 2 { v as u8 } else { 0 });
                c.bus.write8(node + OC305_SNAPSHOT_KEEP_A, if keep == 3 { v as u8 } else { 0 });
            }
            let live = this + OC305_REPLAY_LIVE_SNAPSHOTS;
            c.bus.write32(live, if n > 0 { node_at(0) } else { 0 });
            c.bus.write32(live + 4, if n > 0 { node_at(n - 1) } else { 0 });
            c.bus.write32(live + 8, n);
            // Free list at +0x3bc: empty or one node.
            let spare = crate::difftest::DATA + 0x2000;
            let has_spare = seed % 2 == 0;
            c.bus.write32(this + 0x3bc, if has_spare { spare } else { 0 });
            c.bus.write32(this + 0x3c0, if has_spare { spare } else { 0 });
            c.bus.write32(this + 0x3c4, has_spare as u32);
            c.bus.write32(spare + OC305_SNAPSHOT_NEXT, 0);
            c.bus.write32(spare + 0x18, 0);
            c.g[A0][0] = sx(this);
            let mode = schedule(c, &mut r, seed, 0x0027_0538);
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 20) as u64;
            }
            mode
        });
    }

    /// `ModifierBlock_FreeTexFlip`: no modifier, or one allocated from a real
    /// dlmalloc arena and freed by the real `__builtin_delete`.
    #[test]
    fn modifier_block_free_tex_flip_matches() {
        use crate::game::object::animnode::tests::heap_block;
        use crate::game::wscript::wscriptman::tests::alloc_ready;
        crate::difftest::check(
            ee::s11::f_00353150__generated,
            modifier_block_free_tex_flip,
            200,
            &|c, seed| {
                let _ = crate::difftest::fakes::machine(c, seed, 0x3531_50, 0);
                alloc_ready(c);
                let this = crate::difftest::DATA + 0x100;
                for i in 0..4 {
                    c.bus.write32(this + 4 * i, 0x1111_0000 + i);
                }
                let owned = if seed % 2 == 0 { 0 } else { heap_block(c, 0x20 + 0x10 * (seed % 4)) };
                c.bus.write32(this + OC305_MODIFIER_BLOCK_TEX_FLIP, owned);
                c.g[A0][0] = sx(this);
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0035_3150;
            },
        );
    }

    /// `WScriptProcessList_Count`: empty lists and lists of up to 9
    /// processes; events off, delivered, or unwinding at the loop head.
    #[test]
    fn wscript_process_list_count_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(ee::s10::f_0030b228__generated, wscript_process_list_count, 240, &|c, seed| {
            let mut r = crate::difftest::Rng(0x30b2_2800 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            let list = crate::difftest::DATA;
            let n = seed / 3 % 10;
            let node_at = |i: u32| crate::difftest::DATA + 0x1000 + 0x40 * i;
            for i in 0..n {
                crate::difftest::fill(c, &mut r, node_at(i), 0x20);
                let next = if i + 1 < n { node_at(i + 1) } else { 0 };
                c.bus.write32(node_at(i) + OC305_WSCRIPT_PROCESS_NEXT, next);
            }
            c.bus.write32(list, if n > 0 { node_at(0) } else { 0 });
            c.g[A0][0] = sx(list);
            let mode = schedule(c, &mut r, seed, 0x0030_b228);
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 20) as u64;
            }
            mode
        });
    }

    /// The audio world for `SSXAudioSystem_IsAnyViewInRange`: the world at
    /// `gp-0x848` (`+0x84` the game module, or none), the module's view list
    /// (`+0x84`: count at `+0x10`, views from `+0x4`), each view's position
    /// at `+0x20` near `pos` or far, in front of it or behind; a radius
    /// argument in f12.
    pub(crate) fn oc305_view_in_range_fixture(c: &mut Cpu, seed: u32, min_views: u32) {
        use crate::difftest::{fill, scramble, Rng, DATA, GAME_GP};
        let mut r = Rng(0x288c_0800 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA;
        let module = DATA + 0x100;
        let views = DATA + 0x200;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, if seed % 9 == 0 { 0 } else { module });
        c.bus.write32(module + 0x84, views);
        // At most 3: the view pointers sit at +0x4..+0xc, right below the
        // count at +0x10 (a fourth pointer would overwrite the count).
        let count = min_views + seed / 2 % (4 - min_views);
        c.bus.write32(views + 0x10, count);
        let pos = DATA + 0x1000;
        let mut coord = || f32::from_bits(r.f32(5000.0));
        let base = [coord(), coord(), coord()];
        for (k, v) in base.iter().enumerate() {
            c.bus.write32(pos + 4 * k as u32, v.to_bits());
        }
        c.bus.write32(pos + 12, r.next_u32());
        let radius = 0.5 + f32::from_bits(r.f32(10.0)).abs();
        c.f[12] = radius.to_bits();
        for i in 0..count {
            let view = DATA + 0x2000 + 0x100 * i;
            fill(c, &mut r, view, 0x40);
            c.bus.write32(views + 4 + 4 * i, view);
            let near = r.next_u32() % 2 == 0;
            let reach = radius * 100.0 * if near { 0.5 } else { 1.5 };
            let dz = if r.next_u32() % 3 == 0 { -reach * 0.5 } else { reach * 0.5 };
            let d = [reach * 0.5, -reach * 0.5, dz];
            for k in 0..3 {
                c.bus.write32(view + 0x20 + 4 * k as u32, (base[k] + d[k]).to_bits());
            }
        }
        c.g[A0][0] = sx(DATA + 0x3000);
        c.g[A1][0] = sx(pos);
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = OC305_VIEW_IN_RANGE_ENTRY;
    }

    crate::difftest::events::driven!(oc305_gen_view_in_range, ee::s07::f_00288c08__generated);
    crate::difftest::events::driven!(oc305_hand_view_in_range, ssx_audio_system_is_any_view_in_range);

    /// `SSXAudioSystem_IsAnyViewInRange`: 0..3 views, near or far, in
    /// front or behind, no game module (one view); then events off,
    /// delivered, or unwinding at the loop's back edge and re-entering at
    /// the head.
    #[test]
    fn ssx_audio_system_is_any_view_in_range_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        crate::difftest::check(
            ee::s07::f_00288c08__generated,
            ssx_audio_system_is_any_view_in_range,
            240,
            &|c, seed| oc305_view_in_range_fixture(c, seed, 0),
        );
        check_events(oc305_gen_view_in_range, oc305_hand_view_in_range, 150, &|c, seed| {
            // At least two views so the back edge is taken.
            oc305_view_in_range_fixture(c, seed, 2);
            let mut r = crate::difftest::Rng(0x288c_0801 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, OC305_VIEW_IN_RANGE_ENTRY);
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 40) as u64;
            }
            mode
        });
    }

    /// `WScriptFn_MakeDeadNode`: the static defaults initialised or not;
    /// -1..3 parameters (the first always sets slot 1, the dead-node mode;
    /// others random slots/kinds, ints into float slots converted; slot 0 is
    /// the object id); the id is -1 (the world's default object, sometimes
    /// none) or a section/object pair whose section or entry may be empty.
    /// Found objects have no node (the real `WorldObjFactory_DeadNode` runs
    /// with a mode that sets the object's dead flags or returns at once) or
    /// a node whose modifier check is a fake returning non-zero (nothing
    /// done; sometimes unwinding). Not reached: a node whose modifier check
    /// fails (needs a zero result through the real AttachModifier chain),
    /// and the dead-node modes 0/3 that allocate.
    #[test]
    fn wscript_fn_make_dead_node_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const OBJS: u32 = DIR + 0x2000;
        crate::difftest::check(ee::s10::f_002fc420__generated, wscript_fn_make_dead_node, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2f_c420, 1);
            c.pc = 0x002f_c420;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Dead-node modes that return or only set flags.
            const MODES: [u32; 5] = [1, 2, 5, u32::MAX, 0x7fff];
            let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 4 };
            let ready = seed % 4 != 0 || (count as i32) <= 0;
            w(c, GAME_GP + OC305_DEAD_NODE_DEFAULTS_READY_GP, ready as u32);
            fill(c, &mut r, OC305_DEAD_NODE_DEFAULTS, 3);
            if ready {
                let id = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    (r.next_u32() % 4) | (r.next_u32() % 4) << 8
                };
                w(c, OC305_DEAD_NODE_DEFAULTS, id);
                w(c, OC305_DEAD_NODE_DEFAULTS + 4, MODES[(r.next_u32() % 5) as usize]);
            }
            let kinds = GAME_GP + OC305_DEAD_NODE_KINDS_GP;
            w(c, kinds, 0);
            w(c, kinds + 4, r.next_u32() % 4);
            w(c, kinds + 8, r.next_u32() % 4);
            // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
            w(c, GAME_GP + OC305_DEAD_NODE_DIRECTORY_GP, DIR);
            w(c, DIR, SECTIONS);
            w(c, SECTIONS + 8, TABLE);
            for i in 0..4 {
                w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                let obj = OBJS + 0x100 * i;
                let entry = if i == 3 {
                    r.next_u32() & 0xff
                } else {
                    (obj >> 2) << 8 | (r.next_u32() & 0xff)
                };
                w(c, ENTRIES + 4 * i, entry);
                fill(c, &mut r, obj, 0x10);
                let node = if (seed / 4) % 3 == 0 { NODE } else { 0 };
                w(c, obj + OC305_DEAD_NODE_OBJECT_NODE, node);
            }
            w(c, SECTION + 0x1c, ENTRIES);
            w(c, GAME_GP + OC305_DEAD_NODE_WORLD_GP, WORLD);
            w(
                c,
                WORLD + OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT,
                if seed % 7 == 0 { 0 } else { OBJS + 0x100 },
            );
            // The node's modifier check: a fake (non-zero result).
            w(c, NODE + 0xc, VT);
            c.bus.write16(VT + 0x50, 0);
            w(c, VT + 0x54, FAKE);
            // The result value.
            fill(c, &mut r, OC305_DEAD_NODE_RESULT, 3);
            // Parameters: {slot, value, -, kind}.
            for i in 0..3 {
                let p = PARAMS + 0x10 * i;
                let slot = if i == 0 { 1 } else { r.next_u32() % 3 };
                let value = match slot {
                    0 => match r.next_u32() % 4 {
                        0 => u32::MAX,
                        _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                    },
                    1 => MODES[(r.next_u32() % 5) as usize],
                    _ => r.next_u32(),
                };
                w(c, p, slot);
                w(c, p + 4, value);
                w(c, p + 8, r.next_u32());
                w(c, p + 0xc, if slot == 0 { 0 } else { r.next_u32() % 4 });
            }
            c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
            c.g[A1][0] = sx(count);
            c.g[A2][0] = sx(PARAMS);
        });
    }

    /// `Rider_GetCharacter`: a BELibrary dummy instance, the phase's
    /// interface table entry with a fake `GetInterface` method (sometimes
    /// unwinding), and the real `GetRiderCharID` over the player slot table
    /// (`0x5305b0`) and the character records (`0x534fe0`, 0x1c bytes, id
    /// byte at +0x11).
    #[test]
    fn rider_get_character_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const IFACE: u32 = DATA + 0x2000;
        const VT: u32 = DATA + 0x2100;
        covered(ee::s00::f_00123128__generated, rider_get_character, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x12_3128, 1);
            c.pc = 0x0012_3128;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x4000);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, IFACE);
            c.bus.write32(IFACE + 4, VT);
            c.bus.write16(VT + 8, (4 * (r.next_u32() % 4)) as u16);
            c.bus.write32(VT + 0xc, FAKE);
            for i in 0..8 {
                c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
            }
            fill(c, &mut r, 0x0053_4fe0, 8 * 0x1c / 4);
            let rider = DATA;
            c.bus.write32(rider + OC305_RIDER_AI_INDEX, r.next_u32() % 8);
            c.g[A0][0] = sx(rider);
        });
    }

    /// `PositionModifier_Save`: a stream whose write method is a fake
    /// (logging the block pointer and size; sometimes unwinding), with
    /// `this` adjustments of either sign.
    #[test]
    fn position_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::DATA;
        covered(ee::s11::f_00357000__generated, position_modifier_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_7000, 1);
            c.pc = 0x0035_7000;
            let stream = DATA + 0x1000;
            let vt = DATA + 0x2000;
            c.bus.write32(stream, vt);
            let delta = (r.next_u32() % 0x40) as i32 - 0x20;
            c.bus.write16(vt + OC305_STREAM_WRITE_SLOT, delta as u16);
            c.bus.write32(vt + OC305_STREAM_WRITE_SLOT + 4, FAKE + 0x10 * (seed % 3));
            c.g[A0][0] = sx(DATA + 4 * (r.next_u32() % 4));
            c.g[A1][0] = sx(stream);
        });
    }
}
