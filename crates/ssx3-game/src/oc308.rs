//! Staging wave 212 (oc308): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_c540, rail_modifier_v02, "RailModifier_v02"),
    (
        0x0034_5538,
        spline_modifier_save_path,
        "SplineModifier_SavePath",
    ),
    (0x0027_0130, replay_alloc_snapshot, "Replay_AllocSnapshot"),
    (0x0034_2718, anim_teeter_save, "AnimTeeter_Save"),
    (0x0034_1cf0, anim_object_dtor, "AnimObject_dtor"),
    (
        0x0031_2bd0,
        rider_anim_clear_flags_and_play_anim,
        "RiderAnim_ClearFlagsAndPlayAnim",
    ),
    (
        0x0034_c600,
        c_flag_manager_remove_flag,
        "cFlagManager_RemoveFlag",
    ),
    (0x0032_0330, mem_pool32_alloc, "MEM_Pool32Alloc"),
    (0x0034_aea0, flag_dtor, "Flag_dtor"),
    (0x0031_e2d8, dlheap_sys_alloc, "DLHeap_SysAlloc"),
];

/// Rail modifier: the saved block (0x30 bytes at +0x10) and the instance
/// pointer (+0x40) written to the stream.
pub(crate) const RAIL_MODIFIER_BLOCK: u32 = 0x10;
pub(crate) const RAIL_MODIFIER_INSTANCE: u32 = 0x40;
/// Stream vtable slots (`{i16 delta, fn}`): write bytes (+0x8) and write a
/// pointer (+0x28).
pub(crate) const STREAM_WRITE_BYTES: u32 = 0x8;
pub(crate) const STREAM_WRITE_PTR: u32 = 0x28;

/// A `jalr` through the stream's vtable slot `slot` (`{i16 delta, fn}`):
/// `a0 = stream + delta`, target the slot's function, return to `ret`.
/// Leaves v1 = vtable, v0 = target, as the compiled sequence does.
fn oc308_stream_call(c: &mut Cpu, stream_reg: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let stream = r32(c, stream_reg);
    c.g[V1][0] = c.lw(stream);
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V0][0] = c.lw(vt.wrapping_add(slot + 4));
    c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
    let target = c.g[V0][0] as u32;
    call_ptr(c, target, ret)
}

/// `void RailModifier_v02(RailModifier* self, cStream* stream)` (0x35c540):
/// save the rail modifier: its 0x30-byte block at +0x10 through
/// `stream->WriteBytes`, then its instance pointer (+0x40) through
/// `stream->WritePtr`.
pub fn rail_modifier_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x30;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(RAIL_MODIFIER_BLOCK));
        oc308_stream_call(c, S0, STREAM_WRITE_BYTES, 0x0035_c574)?;
        // Registers at the call: v1 = vtable, a1 = instance, a0 = stream + delta.
        let vt = c.lw(r32(c, S0));
        c.g[V1][0] = vt;
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(RAIL_MODIFIER_INSTANCE));
        c.g[A0][0] = c.lh((vt as u32).wrapping_add(STREAM_WRITE_PTR));
        c.g[V0][0] = c.lw((vt as u32).wrapping_add(STREAM_WRITE_PTR + 4));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0035_c58c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SplineModifier_SavePath(SplineModifier* self, cStream* stream)`
/// (0x345538): write the path's first 8 bytes (the object itself) through
/// `stream->WriteBytes(self, 8)`.
pub fn spline_modifier_save_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0x8;
        oc308_stream_call(c, A3, STREAM_WRITE_BYTES, 0x0034_5560)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replay: the free-snapshot list (head +0x3bc, tail +0x3c0, count +0x3c4).
pub(crate) const REPLAY_FREE_LIST: u32 = 0x3bc;
pub(crate) const REPLAY_FREE_COUNT: u32 = 0x3c4;
/// Snapshot fields reset on allocation.
pub(crate) const SNAPSHOT_DESC: u32 = 0x1c;
pub(crate) const SNAPSHOT_FRAME_A: u32 = 0x20;
pub(crate) const SNAPSHOT_FRAME_B: u32 = 0x24;
pub(crate) const SNAPSHOT_USED: u32 = 0x30;
pub(crate) const SNAPSHOT_INDEX: u32 = 0x34;

const REPLAY_RECYCLE_OLDEST_SNAPSHOT: Callee = Callee {
    addr: 0x0027_01a0,
    f: ee::s07::Replay_RecycleOldestSnapshot,
};
const SNAPSHOT_LIST_POP_FRONT: Callee = Callee {
    addr: 0x0026_e670,
    f: ee::s07::SnapshotList_PopFront,
};

/// `Snapshot* Replay_AllocSnapshot(Replay* self)` (0x270130): take a
/// snapshot from the free list (recycling the oldest one first when the list
/// is empty) and reset it: the 4-byte descriptor at +0x1c cleared, the frame
/// links (+0x20, +0x24) and index (+0x34) set to -1, +0x30 cleared.
pub fn replay_alloc_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let replay = r32(c, A0);
        c.g[V0][0] = c.lw(replay.wrapping_add(REPLAY_FREE_COUNT));
        let have_free = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = sx(replay.wrapping_add(REPLAY_FREE_LIST));
        if !have_free {
            c.cycles += 2;
            REPLAY_RECYCLE_OLDEST_SNAPSHOT.call(c, 0x0027_0150)?;
        }
        c.cycles += 19;
        c.g[A0][0] = c.g[S0][0];
        SNAPSHOT_LIST_POP_FRONT.call(c, 0x0027_0158)?;
        c.g[V1][0] = c.g[V0][0];
        let snapshot = r32(c, V1);
        // The zeroed 4-byte descriptor is built on the stack and copied
        // with an unaligned word move (lwl/lwr, swl/swr), as compiled.
        let tmp = sp(c);
        c.sh(tmp, 0);
        c.sb(tmp.wrapping_add(2), 0);
        c.g[A0][0] = u64::MAX;
        c.sb(tmp.wrapping_add(3), 0);
        c.lwl(A1, tmp.wrapping_add(3));
        c.lwr(A1, tmp);
        c.swl(A1, snapshot.wrapping_add(SNAPSHOT_DESC + 3));
        c.swr(A1, snapshot.wrapping_add(SNAPSHOT_DESC));
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.sw(snapshot.wrapping_add(SNAPSHOT_INDEX), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_FRAME_A), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_FRAME_B), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_USED), 0);
        frame.close(c);
        Ok(())
    })
}

/// AnimTeeter: the embedded AnimObject at +0x30; the teeter's own 0x30
/// bytes at +0 are written raw.
pub(crate) const ANIM_TEETER_ANIM_OBJECT: u32 = 0x30;
pub(crate) const ANIM_TEETER_SAVED_BYTES: u32 = 0x30;

const ANIM_OBJECT_BASE_SAVE: Callee = Callee {
    addr: 0x0034_e3d8,
    f: ee::s11::AnimObject_BaseSave,
};

/// `void AnimTeeter_Save(AnimTeeter* self, cStream* stream)` (0x342718):
/// save the embedded AnimObject (+0x30), then the teeter's first 0x30 bytes
/// through `stream->WriteBytes`.
pub fn anim_teeter_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(ANIM_TEETER_ANIM_OBJECT));
        ANIM_OBJECT_BASE_SAVE.call(c, 0x0034_2738)?;
        // Registers at the call: v1 = vtable, a1 = self, a2 = 0x30.
        let stream = r32(c, S0);
        c.g[V1][0] = c.lw(stream);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = ANIM_TEETER_SAVED_BYTES as u64;
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(STREAM_WRITE_BYTES));
        c.g[V0][0] = c.lw(vt.wrapping_add(STREAM_WRITE_BYTES + 4));
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0034_2754)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// AnimObject (as a derived object): its vtable word at +0x3c and the base
/// part at +0x1c.
pub(crate) const ANIM_OBJECT_DTOR_VPTR: u32 = 0x3c;
pub(crate) const ANIM_OBJECT_DTOR_BASE: u32 = 0x1c;
/// The vtable the destructor installs (0x490b10).
pub(crate) const ANIM_OBJECT_DTOR_VTABLE: u32 = 0x0049_0b10;

const ANIM_OBJECT_BASE_DTOR: Callee = Callee {
    addr: 0x0034_dba8,
    f: ee::s11::AnimObject_BaseDtor,
};
/// `__builtin_delete` (operator delete; another shard: through the table).
const OC308_BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `void AnimObject_dtor(AnimObject* self, int flags)` (0x341cf0): reset
/// the vtable (+0x3c), destroy the base part at +0x1c (flags 0), and free
/// the object when `flags & 1`.
pub fn anim_object_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(ANIM_OBJECT_DTOR_VTABLE);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S1);
        c.sw(this.wrapping_add(ANIM_OBJECT_DTOR_VPTR), c.g[V0][0]);
        c.g[A0][0] = sx(this.wrapping_add(ANIM_OBJECT_DTOR_BASE));
        c.g[A1][0] = 0;
        ANIM_OBJECT_BASE_DTOR.call(c, 0x0034_1d20)?;
        c.g[S0][0] &= 1;
        let free = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC308_BUILTIN_DELETE, 0x0034_1d34)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Rider: the 64-bit flag pair at +0xb0 and the flag moved to the second
/// word (0x3f).
pub(crate) const RIDER_ANIM_FLAGS: u32 = 0xb0;
pub(crate) const RIDER_ANIM_FLAG_BIT: u64 = 0x3f;

/// `Flags64_MoveToSecond` (another shard: through the table).
const OC308_FLAGS64_MOVE_TO_SECOND: u32 = 0x0014_4670;
const ANIM_CONTROLLER_PLAY_ANIM: Callee = Callee {
    addr: 0x0031_28e8,
    f: ee::s10::AnimController_PlayAnim,
};

/// `void RiderAnim_ClearFlagsAndPlayAnim(AnimController* ctrl, _, Rider*
/// rider, int anim)` (0x312bd0): move flag 63 of the rider's flags (+0xb0)
/// to the second word, then `AnimController_PlayAnim(ctrl, anim, 0, -1.0)`
/// (blend -1: the anim table's default).
pub fn rider_anim_clear_flags_and_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = RIDER_ANIM_FLAG_BIT;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(RIDER_ANIM_FLAGS));
        call_ptr(c, OC308_FLAGS64_MOVE_TO_SECOND, 0x0031_2bf4)?;
        // blend = -1.0f (0xbf800000 through at).
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        ANIM_CONTROLLER_PLAY_ANIM.call(c, 0x0031_2c0c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Flag manager: 15 flag groups of 0x188 bytes from +0x20; a group's
/// member count at +0x5c (0: unused).
pub(crate) const FLAG_MANAGER_GROUPS: u32 = 0x20;
pub(crate) const FLAG_MANAGER_GROUP_COUNT: u32 = 15;
pub(crate) const FLAG_GROUP_SIZE: u32 = 0x188;
pub(crate) const FLAG_GROUP_MEMBERS: u32 = 0x5c;

const C_FLAG_MANAGER_GROUP_REMOVE_FLAG: Callee = Callee {
    addr: 0x0034_b168,
    f: ee::s11::cFlagManager_GroupRemoveFlag,
};

/// `void cFlagManager_RemoveFlag(cFlagManager* self, Flag* flag)`
/// (0x34c600): offer the flag to each used group in turn
/// (`cFlagManager_GroupRemoveFlag`) until one removes it.
pub fn c_flag_manager_remove_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(FLAG_MANAGER_GROUPS));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 3;
            let group = r32(c, S0);
            c.g[V0][0] = c.lw(group.wrapping_add(FLAG_GROUP_MEMBERS));
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                C_FLAG_MANAGER_GROUP_REMOVE_FLAG.call(c, 0x0034_c634)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break;
                }
            }
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            let more = (c.g[S1][0] as i64) < FLAG_MANAGER_GROUP_COUNT as i64;
            c.g[V0][0] = more as u64;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLAG_GROUP_SIZE));
            if !more {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c620);
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

/// The 32-byte pool's free list head (`pool + 0x6000`).
pub(crate) const MEM_POOL32_FREE: u32 = 0x6000;
const MEM_POOL32_FALLBACK: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::MEM_PoolAlloc,
};

/// `void* MEM_Pool32Alloc(Pool32* pool)` (0x320330): pop a block off the
/// 32-byte pool's free list (`pool + 0x6000`); when it is empty fall back
/// to `MEM_PoolAlloc(0x24, a1)` (a1 is whatever the caller left there).
pub fn mem_pool32_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let head = r32(c, A0).wrapping_add(MEM_POOL32_FREE);
        c.g[V0][0] = c.lw(head);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.lw(r32(c, V0)); // next free block
            c.cycles += 2;
            c.sw(head, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x24;
            MEM_POOL32_FALLBACK.call(c, 0x0032_034c)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Flag: the vtable word at +0xc and the flag's instance at +0x18 (what the
/// flag manager tracks).
pub(crate) const FLAG_DTOR_VPTR: u32 = 0xc;
pub(crate) const FLAG_DTOR_INSTANCE: u32 = 0x18;
/// The vtable the destructor installs (0x48fc10).
pub(crate) const FLAG_DTOR_VTABLE: u32 = 0x0048_fc10;

/// `World_GetFlagManager` (another shard: through the table).
const OC308_WORLD_GET_FLAG_MANAGER: u32 = 0x002d_1cb0;
const C_FLAG_MANAGER_REMOVE_FLAG: Callee = Callee {
    addr: 0x0034_c600,
    f: ee::s11::cFlagManager_RemoveFlag,
};
const OC308_WORLD_OBJ_DTOR: Callee = Callee {
    addr: 0x0034_fbf0,
    f: ee::s11::WorldObj_dtor,
};

/// `void Flag_dtor(Flag* self, int flags)` (0x34aea0): reset the vtable
/// (+0xc), take the flag's instance (+0x18) out of the world's flag manager,
/// then destroy the WorldObj base (which frees on `flags & 1`).
pub fn flag_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(FLAG_DTOR_VTABLE);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.sw(this.wrapping_add(FLAG_DTOR_VPTR), c.g[V0][0]);
        call_ptr(c, OC308_WORLD_GET_FLAG_MANAGER, 0x0034_aec8)?;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(FLAG_DTOR_INSTANCE));
        c.g[A0][0] = c.g[V0][0];
        C_FLAG_MANAGER_REMOVE_FLAG.call(c, 0x0034_aed4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        OC308_WORLD_OBJ_DTOR.call(c, 0x0034_aee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// dlmalloc `malloc_state` fields DLHeap_SysAlloc uses.
pub(crate) mod oc308_av {
    /// `max_fast`; bit 1 (2): fast chunks present.
    pub const MAX_FAST: u32 = 0x0;
    pub const TOP: u32 = 0x30;
    pub const TRIM_THRESHOLD: u32 = 0x344;
    pub const TOP_PAD: u32 = 0x348;
    pub const PAGESIZE: u32 = 0x35c;
    /// Bit 0: MORECORE_CONTIGUOUS.
    pub const MORECORE_PROPERTIES: u32 = 0x360;
    pub const MMAPPED_MEM: u32 = 0x364;
    pub const SBRKED_MEM: u32 = 0x368;
    pub const MAX_SBRKED_MEM: u32 = 0x36c;
    pub const MAX_TOTAL_MEM: u32 = 0x374;
}

const DLHEAP_CONSOLIDATE: Callee = Callee {
    addr: 0x0031_eee8,
    f: ee::s10::DLHeap_Consolidate,
};
const DLHEAP_MALLOC: Callee = Callee {
    addr: 0x0031_e818,
    f: ee::s10::DLHeap_Malloc,
};
const DLHEAP_MORE_CORE: Callee = Callee {
    addr: 0x0031_ffd8,
    f: ee::s10::DLHeap_MoreCore,
};
const DLHEAP_FREE: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};
/// `__errno` (another shard: through the table).
const OC308_ERRNO: u32 = 0x0041_5fb8;

/// `void* DLHeap_SysAlloc(mspace* ms, size_t nb, malloc_state* av)`
/// (0x31e2d8): dlmalloc's `sYSMALLOc` without mmap. With fast chunks
/// pending, consolidate and retry `DLHeap_Malloc(nb - 15)`. Otherwise grow
/// the heap through `DLHeap_MoreCore` (page-rounded `nb + top_pad + 16`,
/// less the old top when contiguous): extend the top chunk in place, or
/// (foreign or non-contiguous break) align the new space, fetch the
/// correction with a second MoreCore call, make it the new top and fence
/// off and free the old top. Update the sbrk statistics and split `nb`
/// off the top; on failure `errno = ENOMEM` and return 0.
///
/// Stack locals: +0 nb, +4 old top, +8 size, +0xc ~pagemask.
pub fn dlheap_sys_alloc(c: &mut Cpu) {
    use oc308_av::*;
    const ALL_ONES: u64 = u64::MAX;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S6, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, FP, 0x20);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        let av = r32(c, S1);
        let local = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
        c.g[V1][0] = c.lw(av.wrapping_add(PAGESIZE));
        let nb_slot = local(c, 0);
        c.sw(nb_slot, c.g[A1][0]);
        c.g[V0][0] = c.lw(av.wrapping_add(MAX_FAST)) & 2;
        // fp = pagemask
        c.g[FP][0] = sx(r32(c, V1).wrapping_sub(1));
        'done: {
            if c.g[V0][0] != 0 {
                // Fast chunks pending: consolidate, then malloc again.
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                DLHEAP_CONSOLIDATE.call(c, 0x0031_e32c)?;
                c.g[V0][0] = c.lw(local(c, 0));
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0xf));
                DLHEAP_MALLOC.call(c, 0x0031_e33c)?;
                frame.restore(c, S0, 0xa0);
                break 'done;
            }

            // size = nb + top_pad + 16 (less old_size when contiguous).
            c.cycles += 22;
            c.g[V1][0] = c.lw(av.wrapping_add(TOP));
            c.g[S2][0] = ALL_ONES; // snd_brk = MORECORE_FAILURE
            c.g[A2][0] = c.lw(local(c, 0));
            c.g[S3][0] = ALL_ONES; // brk = MORECORE_FAILURE
            let old_top_slot = local(c, 4);
            c.sw(old_top_slot, c.g[V1][0]);
            c.g[V1][0] = 0xffff_ffff_ffff_fffc;
            c.g[A0][0] = c.lw(local(c, 4));
            c.g[V0][0] = c.lw(av.wrapping_add(TOP_PAD));
            c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(av.wrapping_add(MORECORE_PROPERTIES));
            c.g[S4][0] = c.g[A1][0] & c.g[V1][0]; // old_size
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
            c.g[V1][0] = c.lw(local(c, 4));
            c.g[V0][0] <<= 32;
            c.g[S0][0] = c.g[V0][0] >> 32; // size
            c.g[A0][0] &= 1;
            c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, S4))); // old_end
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = r32(c, S4) as u64;
                c.g[S0][0] = c.g[S0][0].wrapping_sub(c.g[V0][0]);
            }

            // Round to pages; brk = MoreCore(size) when size > 0.
            c.cycles += 10;
            c.g[V0][0] = !c.g[FP][0];
            c.g[V1][0] = c.g[FP][0] << 32;
            let notmask_slot = local(c, 0xc);
            c.sw(notmask_slot, c.g[V0][0]);
            c.g[V1][0] >>= 32;
            c.g[V0][0] <<= 32;
            c.g[V1][0] = c.g[S0][0].wrapping_add(c.g[V1][0]);
            c.g[V0][0] >>= 32;
            c.g[S0][0] = c.g[V1][0] & c.g[V0][0];
            let positive = (c.g[S0][0] as i64) > 0;
            c.g[A0][0] = c.g[S6][0];
            if positive {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                DLHEAP_MORE_CORE.call(c, 0x0031_e3d8)?;
                c.g[S3][0] = c.g[V0][0];
            }

            'split: {
                c.cycles += 2;
                'fail: {
                    if c.g[S3][0] == c.g[S2][0] {
                        break 'fail;
                    }
                    'stats: {
                        'foreign: {
                            // sbrked_mem += size; extend the top in place?
                            c.cycles += 7;
                            c.g[V0][0] = c.lw(av.wrapping_add(SBRKED_MEM));
                            c.g[V1][0] = sx(r32(c, S0));
                            let size_slot = local(c, 8);
                            c.sw(size_slot, c.g[V1][0]);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                            let extends = c.g[S3][0] == c.g[S5][0];
                            c.sw(av.wrapping_add(SBRKED_MEM), c.g[V0][0]);
                            if !extends {
                                break 'foreign;
                            }
                            c.cycles += 10;
                            c.g[V0][0] = r32(c, S4) as u64;
                            c.g[V1][0] = 1;
                            c.g[A0][0] = c.lw(local(c, 4));
                            c.g[V0][0] = c.g[S0][0].wrapping_add(c.g[V0][0]);
                            c.g[V0][0] |= c.g[V1][0];
                            c.g[V0][0] = sx(r32(c, V0));
                            c.sw(r32(c, A0).wrapping_add(4), c.g[V0][0]);
                            break 'stats;
                        }
                        'new_top: {
                            'got_snd: {
                                'non_contiguous: {
                                    // s0 = correction, s7 = aligned_brk.
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lw(av.wrapping_add(MORECORE_PROPERTIES));
                                    c.g[S0][0] = 0;
                                    c.g[V0][0] = c.g[V1][0] & 1;
                                    c.g[S7][0] = c.g[S3][0];
                                    if c.g[V0][0] == 0 {
                                        break 'non_contiguous;
                                    }
                                    // A break below the old end: not contiguous after all.
                                    'reload: {
                                        'clear: {
                                            c.cycles += 2;
                                            let no_old = c.g[S4][0] == 0;
                                            c.g[V0][0] = (c.g[S7][0] < c.g[S5][0]) as u64;
                                            if no_old {
                                                break 'clear;
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0 {
                                                c.g[V0][0] =
                                                    c.lw(av.wrapping_add(MORECORE_PROPERTIES));
                                                break 'reload;
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                                            c.g[V0][0] &= c.g[V1][0];
                                            c.sw(av.wrapping_add(MORECORE_PROPERTIES), c.g[V0][0]);
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw(av.wrapping_add(MORECORE_PROPERTIES));
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] &= 1;
                                    if c.g[V0][0] == 0 {
                                        break 'non_contiguous;
                                    }
                                    // Contiguous: count the foreign gap.
                                    c.cycles += 2;
                                    c.g[V1][0] = sx(r32(c, S3).wrapping_sub(r32(c, S5)));
                                    if c.g[S4][0] != 0 {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(av.wrapping_add(SBRKED_MEM));
                                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                        c.sw(av.wrapping_add(SBRKED_MEM), c.g[V0][0]);
                                    }
                                    // Front misalignment.
                                    c.cycles += 4;
                                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(8));
                                    c.g[V1][0] = c.g[V0][0] & 0xf;
                                    c.g[V0][0] = 0x10;
                                    if c.g[V1][0] != 0 {
                                        c.cycles += 6;
                                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                                        c.g[V0][0] <<= 32;
                                        c.g[S0][0] = c.g[V0][0] >> 32;
                                        c.g[V1][0] = sx(r32(c, S0));
                                        c.g[S7][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                                    }
                                    // correction += old_size + end page rounding;
                                    // snd_brk = MoreCore(correction).
                                    c.cycles += 23;
                                    c.g[V0][0] = r32(c, S4) as u64;
                                    c.g[A1][0] = c.lw(local(c, 8));
                                    c.g[A2][0] = c.lw(local(c, 0xc));
                                    c.g[S0][0] = c.g[S0][0].wrapping_add(c.g[V0][0]);
                                    c.g[S5][0] = sx(r32(c, S3).wrapping_add(r32(c, A1)));
                                    c.g[V1][0] = sx(r32(c, S0));
                                    c.g[A0][0] = c.g[S6][0];
                                    c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, FP)));
                                    c.g[V0][0] &= c.g[A2][0];
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                                    c.g[V0][0] = r32(c, V0) as u64;
                                    c.g[S0][0] = c.g[S0][0].wrapping_add(c.g[V0][0]);
                                    c.g[A1][0] = c.g[S0][0];
                                    DLHEAP_MORE_CORE.call(c, 0x0031_e4f0)?;
                                    c.g[V1][0] = ALL_ONES;
                                    c.g[S2][0] = c.g[V0][0];
                                    let failed = c.g[S2][0] == c.g[V1][0];
                                    c.g[V0][0] = (c.g[S2][0] < c.g[S3][0]) as u64;
                                    if failed {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[A1][0] = 0;
                                        DLHEAP_MORE_CORE.call(c, 0x0031_e510)?;
                                        c.g[S0][0] = 0;
                                        c.g[S2][0] = c.g[V0][0];
                                        break 'got_snd;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0 {
                                        // snd_brk below brk: give up contiguity.
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw(av.wrapping_add(MORECORE_PROPERTIES));
                                        c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                                        c.g[S2][0] = c.g[S5][0];
                                        c.g[V1][0] &= c.g[V0][0];
                                        c.g[S0][0] = 0;
                                        c.sw(av.wrapping_add(MORECORE_PROPERTIES), c.g[V1][0]);
                                    }
                                    break 'got_snd;
                                }
                                // Not contiguous: snd_brk = MoreCore(0).
                                c.cycles += 4;
                                c.g[V0][0] = ALL_ONES;
                                let have_snd = c.g[S2][0] != c.g[V0][0];
                                c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S7)));
                                if have_snd {
                                    break 'new_top;
                                }
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = 0;
                                DLHEAP_MORE_CORE.call(c, 0x0031_e560)?;
                                c.g[A0][0] = c.lw(local(c, 8));
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V1][0] = c.lw(av.wrapping_add(SBRKED_MEM));
                                c.g[V0][0] = sx(r32(c, S2).wrapping_sub(r32(c, S3)));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
                                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                c.sw(av.wrapping_add(SBRKED_MEM), c.g[V1][0]);
                            }
                            c.cycles += 4;
                            c.g[V0][0] = ALL_ONES;
                            let no_snd = c.g[S2][0] == c.g[V0][0];
                            c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S7)));
                            if no_snd {
                                break 'stats;
                            }
                        }
                        // top = aligned_brk, size (snd_brk - aligned_brk + correction) | 1.
                        c.cycles += 13;
                        c.g[V0][0] = 1;
                        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[S0][0]);
                        c.sw(av.wrapping_add(TOP), c.g[S7][0]);
                        c.g[V1][0] |= c.g[V0][0];
                        c.g[A0][0] = sx(r32(c, S0));
                        c.g[V1][0] = sx(r32(c, V1));
                        c.sw(r32(c, S7).wrapping_add(4), c.g[V1][0]);
                        c.g[V0][0] = c.lw(av.wrapping_add(SBRKED_MEM));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        let no_old = c.g[S4][0] == 0;
                        c.sw(av.wrapping_add(SBRKED_MEM), c.g[V0][0]);
                        if no_old {
                            break 'stats;
                        }
                        // Shrink the old top, fence it off (two size-4|1 words).
                        c.cycles += 13;
                        c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                        c.g[A0][0] = sx(r32(c, S4).wrapping_sub(0xc));
                        c.g[A2][0] = c.lw(local(c, 4));
                        c.g[S4][0] = c.g[A0][0] & c.g[V0][0];
                        c.g[A1][0] = 5;
                        c.g[V1][0] = c.g[S4][0] | 1;
                        c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, S4)));
                        c.sw(r32(c, A2).wrapping_add(4), c.g[V1][0]);
                        c.g[A0][0] = (c.g[S4][0] < 0x10) as u64;
                        c.sw(r32(c, V0).wrapping_add(8), c.g[A1][0]);
                        let small = c.g[A0][0] != 0;
                        c.sw(r32(c, V0).wrapping_add(4), c.g[A1][0]);
                        if small {
                            break 'stats;
                        }
                        // Free the old top with trimming off.
                        c.cycles += 8;
                        c.g[V0][0] = ALL_ONES;
                        c.g[S0][0] = c.lw(av.wrapping_add(TRIM_THRESHOLD));
                        c.g[A0][0] = c.g[S6][0];
                        c.sw(av.wrapping_add(TRIM_THRESHOLD), c.g[V0][0]);
                        c.g[A1][0] = sx(r32(c, A2).wrapping_add(8));
                        DLHEAP_FREE.call(c, 0x0031_e610)?;
                        c.sw(av.wrapping_add(TRIM_THRESHOLD), c.g[S0][0]);
                    }
                    // Statistics.
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(av.wrapping_add(SBRKED_MEM));
                    c.g[V0][0] = c.lw(av.wrapping_add(MAX_SBRKED_MEM));
                    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.sw(av.wrapping_add(MAX_SBRKED_MEM), c.g[A0][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(av.wrapping_add(MMAPPED_MEM));
                    c.g[V0][0] = c.lw(av.wrapping_add(MAX_TOTAL_MEM));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.sw(av.wrapping_add(MAX_TOTAL_MEM), c.g[A0][0]);
                    }
                    // Split nb off the top if it is big enough.
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(local(c, 0));
                    c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                    c.g[A1][0] = c.lw(av.wrapping_add(TOP));
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x10));
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(4));
                    c.g[V1][0] &= c.g[V0][0];
                    c.g[V1][0] = sx(r32(c, V1));
                    c.g[A0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    let too_small = c.g[A0][0] != 0;
                    c.g[A2][0] = c.lw(local(c, 0));
                    if too_small {
                        break 'fail;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = c.g[A2][0] | 1;
                    c.sw(av.wrapping_add(TOP), c.g[A0][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A2)));
                    c.sw(r32(c, A1).wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] |= 1;
                    c.sw(r32(c, A0).wrapping_add(4), c.g[V1][0]);
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                    break 'split;
                }
                // errno = ENOMEM; return 0.
                c.cycles += 5;
                call_ptr(c, OC308_ERRNO, 0x0031_e69c)?;
                c.g[V1][0] = 0xc;
                c.sw(r32(c, V0), c.g[V1][0]);
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::DATA;

    /// A stream object at `stream` whose vtable (at `vt`) has fake methods
    /// in the slots `slots` with random `this` deltas.
    fn oc308_fake_stream(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        stream: u32,
        vt: u32,
        slots: &[u32],
    ) {
        c.bus.write32(stream, vt);
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// Both stream calls are made (fakes log their arguments), and either may
    /// unwind.
    #[test]
    fn rail_modifier_v02_matches() {
        covered(ee::s11::f_0035c540__generated, rail_modifier_v02, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_c540, 2);
            let self_ = DATA;
            let stream = DATA + 0x1000;
            oc308_fake_stream(
                c,
                &mut r,
                stream,
                DATA + 0x2000,
                &[STREAM_WRITE_BYTES, STREAM_WRITE_PTR],
            );
            c.bus.write32(self_ + RAIL_MODIFIER_INSTANCE, r.next_u32());
            c.g[A0][0] = sx(self_);
            c.g[A1][0] = sx(stream);
            c.pc = 0x0035_c540;
        });
    }

    /// The one stream write is made (fake logs its arguments) and may unwind.
    #[test]
    fn spline_modifier_save_path_matches() {
        covered(
            ee::s11::f_00345538__generated,
            spline_modifier_save_path,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_5538, 1);
                let stream = DATA + 0x1000;
                oc308_fake_stream(c, &mut r, stream, DATA + 0x2000, &[STREAM_WRITE_BYTES]);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xff0));
                c.g[A1][0] = sx(stream);
                c.pc = 0x0034_5538;
            },
        );
    }

    /// Free list with one snapshot (count > 0: popped directly) or a
    /// non-positive count (the recycle pass runs first; its ring is empty, so
    /// it only walks twice and returns).
    #[test]
    fn replay_alloc_snapshot_matches() {
        crate::difftest::check(
            ee::s07::f_00270130__generated,
            replay_alloc_snapshot,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_0130, 0);
                let replay = DATA;
                let node = DATA + 0x1000;
                let count = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 5) + 1,
                    _ => (r.next_u32() | 0x8000_0000),
                };
                c.bus.write32(replay + REPLAY_FREE_COUNT, count);
                c.bus.write32(replay + REPLAY_FREE_LIST, node);
                c.bus.write32(replay + REPLAY_FREE_LIST + 4, node);
                // Recycle ring: current (+0x3ac) and start (+0x3b0) empty.
                c.bus.write32(replay + 0x3ac, 0);
                c.bus.write32(replay + 0x3b0, 0);
                crate::difftest::fill(c, &mut r, node, 0x10);
                c.bus.write32(node + 0x14, 0);
                c.bus.write32(node + 0x18, 0);
                c.g[A0][0] = sx(replay);
                c.pc = 0x0027_0130;
            },
        );
    }

    /// A stream whose vtable has a fake in every slot (AnimObject_BaseSave,
    /// ModObj_Save and WorldObj_Save write through several of them).
    fn oc308_all_fake_stream(c: &mut Cpu, r: &mut crate::difftest::Rng, stream: u32, vt: u32) {
        c.bus.write32(stream, vt);
        for off in (0..0x340).step_by(8) {
            let delta = ((r.next_u32() as i16 as i32 % 64) & !3) as u16;
            c.bus.write16(vt + off, delta);
            c.bus.write32(vt + off + 4, FAKE + 2 * off);
        }
    }

    /// The base save runs for real (its stream writes are fakes; no modifier
    /// block to save), then the teeter's own write; any call may unwind.
    #[test]
    fn anim_teeter_save_matches() {
        covered(ee::s11::f_00342718__generated, anim_teeter_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_2718, 8);
            let self_ = DATA;
            crate::difftest::fill(c, &mut r, self_, 0x40);
            let base = self_ + ANIM_TEETER_ANIM_OBJECT;
            // ModObj (base + 0x14): no modifier block; WorldObj fields zero.
            c.bus.write32(base + 0x30, 0);
            c.bus.write32(base + 0x4c, 0);
            let stream = DATA + 0x1000;
            oc308_all_fake_stream(c, &mut r, stream, DATA + 0x2000);
            c.g[A0][0] = sx(self_);
            c.g[A1][0] = sx(stream);
            c.pc = 0x0034_2718;
        });
    }

    /// The base destructor runs for real on a heap object (empty ModObj, an
    /// optional cache block, no nodes); `flags & 1` frees the object.
    #[test]
    fn anim_object_dtor_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(ee::s11::f_00341cf0__generated, anim_object_dtor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_1cf0, 0);
            // The game heap over test memory, pools off.
            c.bus.write32(GAME_GP + 0xdd8, 0);
            oc238_run(
                c,
                0x0031_ff60,
                ee::s10::f_0031ff60,
                &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
            );
            let alloc = |c: &mut Cpu, size: u64| -> u32 {
                oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[size, 0]) as u32
            };
            let me = alloc(c, 0x80);
            for i in 0..0x20 {
                c.bus.write32(me + 4 * i, 0);
            }
            crate::difftest::fill(c, &mut r, me, 0x7);
            let base = me + ANIM_OBJECT_DTOR_BASE;
            c.bus.write16(base + 0x14 + 0x10, (r.next_u32() % 8) as u16);
            let cache = if seed % 3 == 0 { 0 } else { alloc(c, 0x20) };
            c.bus.write32(base + 0x44, cache);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = (r.next_u32() & 0xff) as u64;
            c.pc = 0x0034_1cf0;
        });
    }

    /// Flags64_MoveToSecond runs for real; PlayAnim runs for real with its
    /// layer lookup a fake: 0x207 (no layer: returns) or a layer, where the
    /// next fake (inside PlayAnimImpl) unwinds.
    #[test]
    fn rider_anim_clear_flags_and_play_anim_matches() {
        use crate::difftest::fakes::{LOG_RESULTS, LOG_UNWIND_AT};
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s10::f_00312bd0__generated,
            rider_anim_clear_flags_and_play_anim,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_2bd0, 1);
                let ctrl = DATA + 0x4000;
                let rider = DATA + 0x8000;
                let anim = r.next_u32() % 64;
                crate::difftest::fill(c, &mut r, rider + RIDER_ANIM_FLAGS, 4);
                let vt = DATA + 0xc000;
                c.bus
                    .write16(vt + 0x20, ((r.next_u32() % 0x40) & !3) as u16);
                c.bus.write32(vt + 0x24, FAKE);
                c.bus.write32(ctrl + 0x58, vt);
                let layer = if seed % 3 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0x207
                };
                c.bus.write32(LOG_RESULTS, layer);
                if layer != 0x207 {
                    c.bus.write32(LOG_UNWIND_AT, 1);
                }
                let tables = DATA + 0x10000;
                c.bus.write32(GAME_GP + 0xd8c, tables);
                c.bus.write32(tables + 4 * layer + 0x1030, DATA + 0x14000);
                let entry = 0x0044_6990 + 0x1c * anim;
                c.bus.write32(entry + 0xc, r.next_u32() % 4);
                c.bus.write32(entry + 0x10, r.f32(2.0));
                c.bus.write32(ctrl + 0x50, DATA + 0x18000);
                c.g[A0][0] = sx(ctrl);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(rider);
                c.g[A3][0] = anim as u64;
                c.pc = 0x0031_2bd0;
            },
        );
    }

    /// A flag manager at `mgr` with its 15 groups: most unused, some with
    /// 1..4 members on the flag's track or another one; `flag` (its id at
    /// +0x78, its track cell at +0x80) is a member of some. Groups have no
    /// mesh or buffer, so a group emptied frees nothing.
    fn oc308_flag_manager(c: &mut Cpu, r: &mut crate::difftest::Rng, mgr: u32, flag: u32) {
        let track_cell = DATA + 0x2_0000;
        let members = DATA + 0x2_1000;
        let track = r.next_u32();
        c.bus.write32(flag + 0x80, track_cell);
        c.bus.write32(track_cell, track);
        c.bus.write32(flag + 0x78, r.next_u32() % 4);
        for i in 0..FLAG_MANAGER_GROUP_COUNT {
            let g = mgr + FLAG_MANAGER_GROUPS + FLAG_GROUP_SIZE * i;
            let count = if r.next_u32() % 3 == 0 {
                1 + r.next_u32() % 4
            } else {
                0
            };
            c.bus.write32(g + FLAG_GROUP_MEMBERS, count);
            c.bus.write32(
                g + 0x58,
                if r.next_u32() % 3 == 0 {
                    track ^ 1
                } else {
                    track
                },
            );
            c.bus.write32(g + 0x60, 0);
            c.bus.write32(g + 0x74, 0);
            for k in 0..6u32 {
                let m = members + 0x100 * k;
                c.bus.write32(g + 0x94 + 4 * k, m);
                c.bus.write32(m + 0x78, r.next_u32() % 4);
            }
        }
    }

    /// Groups used or not, the flag found in one of them (stops) or none.
    #[test]
    fn c_flag_manager_remove_flag_matches() {
        crate::difftest::check(
            ee::s11::f_0034c600__generated,
            c_flag_manager_remove_flag,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c600, 0);
                let mgr = DATA;
                let flag = DATA + 0x1_0000;
                oc308_flag_manager(c, &mut r, mgr, flag);
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = sx(flag);
                c.pc = 0x0034_c600;
            },
        );
    }

    /// Events due inside the group loop (all groups unused: the loop runs
    /// its 15 iterations).
    #[test]
    fn c_flag_manager_remove_flag_events() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s11::f_0034c600__generated,
            c_flag_manager_remove_flag,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c6e0, 0);
                let mgr = DATA;
                let flag = DATA + 0x1_0000;
                oc308_flag_manager(c, &mut r, mgr, flag);
                if seed % 2 == 0 {
                    for i in 0..FLAG_MANAGER_GROUP_COUNT {
                        c.bus.write32(
                            mgr + FLAG_MANAGER_GROUPS + FLAG_GROUP_SIZE * i + FLAG_GROUP_MEMBERS,
                            0,
                        );
                    }
                }
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = sx(flag);
                schedule(c, &mut r, seed, 0x0034_c600)
            },
        );
    }

    /// A block on the free list (popped) or an empty list: MEM_PoolAlloc
    /// over a test heap, with the size-class pools on or off.
    #[test]
    fn mem_pool32_alloc_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(ee::s10::f_00320330__generated, mem_pool32_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_0330, 0);
            let pool32 = DATA + 0x20_0000;
            if seed % 3 != 0 {
                let block = DATA + 0x30_0000 + 0x20 * (r.next_u32() % 16);
                c.bus.write32(pool32 + MEM_POOL32_FREE, block);
                c.bus
                    .write32(block, if seed % 2 == 0 { 0 } else { block + 0x20 });
            } else {
                c.bus.write32(pool32 + MEM_POOL32_FREE, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + 0xdd8, 1);
                    const POOLS: [(u32, u32, u32); 7] = [
                        (0x2a70, 0x800, 0x40),
                        (0x2a60, 0x2000, 0x40),
                        (0x2a6c, 0x6000, 0x40),
                        (0x2a7c, 0x32000, 0x40),
                        (0x2a84, 0x25800, 0x80),
                        (0x2a78, 0x20000, 0x100),
                        (0x2a68, 0x20000, 0x200),
                    ];
                    let mut pool = DATA + 0xc_0000;
                    let mut base = DATA + 0x8_0000;
                    for (slot, head, stride) in POOLS {
                        for n in 0..64u32 {
                            let node = base + stride * n;
                            c.bus.write32(node, if n == 63 { 0 } else { node + stride });
                        }
                        c.bus.write32(pool + head, base);
                        c.bus.write32(GAME_GP + slot, pool);
                        pool += 0x100;
                        base += 0x8000;
                    }
                } else {
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                }
            }
            c.g[A0][0] = sx(pool32);
            c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
            c.pc = 0x0032_0330;
        });
    }

    /// The world's flag manager (World at gp-0x848, +0x84, +0x70) with the
    /// flag's instance in some groups; the WorldObj base destructor runs for
    /// real (no free: `flags & 1` clear).
    #[test]
    fn flag_dtor_matches() {
        use crate::difftest::GAME_GP;
        crate::difftest::check(ee::s11::f_0034aea0__generated, flag_dtor, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x34_aea0, 0);
            let obj = DATA;
            let inst = DATA + 0x100;
            let world = DATA + 0x3_0000;
            let ai = DATA + 0x3_1000;
            let mgr = DATA + 0x4_0000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, ai);
            c.bus.write32(ai + 0x70, mgr);
            oc308_flag_manager(c, &mut r, mgr, inst);
            // WorldObj part (as in WorldObj_dtor's test).
            crate::difftest::fill(c, &mut r, obj, 8);
            c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
            if seed % 3 == 0 {
                c.bus.write32(obj + 4, 0);
            } else {
                c.bus.write32(obj + 4, DATA + 0x200);
                c.bus
                    .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x300 });
            }
            c.bus.write32(obj + FLAG_DTOR_INSTANCE, inst);
            c.bus.write32(inst + 0x8, r.next_u32());
            c.bus
                .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (r.next_u32() & !1) as u64;
            c.pc = 0x0034_aea0;
        });
    }

    /// DLHeap_SysAlloc on the game's heap (mspace 0x4ff250) brought up over
    /// test memory by DLHeap_InitArena and two real allocations. Per seed:
    /// the plain contiguous extension of the top; fast chunks pending
    /// (consolidate + malloc); a foreign break (alignment correction, second
    /// MoreCore, old top fenced and freed); not contiguous (MoreCore(0)); a
    /// break below the old end (contiguity dropped); a request beyond the
    /// arena (errno); a limit that fails only the second MoreCore; an old top
    /// of size 16 or 0 (no free / no fence).
    #[test]
    fn dlheap_sys_alloc_matches() {
        use super::oc308_av::*;
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        const MS: u32 = 0x004f_f250;
        crate::difftest::check(ee::s10::f_0031e2d8__generated, dlheap_sys_alloc, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x31_e2d8, 0);
            c.bus.write32(GAME_GP + 0xdd8, 0);
            oc238_run(
                c,
                0x0031_ff60,
                ee::s10::f_0031ff60,
                &[MS as u64, (DATA + 0x40000) as u64, 0x40000],
            );
            for size in [0x40u64, 0x100 + (r.next_u32() % 0x400) as u64] {
                oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[size, 0]);
            }
            let av = c.bus.read32(MS);
            let mut nb = 0x10 * (1 + r.next_u32() % 0x200);
            let shift_brk = |c: &mut Cpu, delta: i32| {
                let brk = c.bus.read32(MS + 8);
                c.bus.write32(MS + 8, brk.wrapping_add(delta as u32));
            };
            let foreign = 4 * (1 + r.next_u32() % 0x40) as i32;
            match seed % 9 {
                0 => {}
                1 => {
                    let mf = c.bus.read32(av + MAX_FAST);
                    c.bus.write32(av + MAX_FAST, mf | 2);
                }
                2 => shift_brk(c, foreign),
                3 => {
                    let p = c.bus.read32(av + MORECORE_PROPERTIES);
                    c.bus.write32(av + MORECORE_PROPERTIES, p & !1);
                    if seed % 2 == 0 {
                        shift_brk(c, foreign);
                    }
                }
                4 => shift_brk(c, -0x100),
                5 => nb = 0x8_0000,
                6 => {
                    // The first MoreCore succeeds exactly, the second fails.
                    shift_brk(c, foreign);
                    let top = c.bus.read32(av + TOP);
                    let old_size = c.bus.read32(top + 4) & !3;
                    let mask = c.bus.read32(av + PAGESIZE).wrapping_sub(1);
                    let size = nb
                        .wrapping_add(c.bus.read32(av + TOP_PAD))
                        .wrapping_add(16)
                        .wrapping_sub(old_size);
                    let size = size.wrapping_add(mask) & !mask;
                    let brk = c.bus.read32(MS + 8);
                    let base = c.bus.read32(MS + 4);
                    c.bus.write32(
                        MS + 0xc,
                        brk.wrapping_add(size).wrapping_add(4).wrapping_sub(base),
                    );
                }
                7 => {
                    shift_brk(c, foreign);
                    let top = c.bus.read32(av + TOP);
                    c.bus.write32(top + 4, if seed % 2 == 0 { 0x11 } else { 1 });
                }
                _ => nb = 0x10 * (1 + r.next_u32() % 4),
            }
            c.g[A0][0] = sx(MS);
            c.g[A1][0] = nb as u64;
            c.g[A2][0] = sx(av);
            c.pc = 0x0031_e2d8;
        });
    }
}
