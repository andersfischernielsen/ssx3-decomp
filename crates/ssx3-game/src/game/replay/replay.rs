//! `src/replay/replay.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::replay::playbackman::oc653_off;
pub(crate) use crate::game::replay::playbackman::oc683_direct;
pub(crate) use crate::game::replay::playbackman::oc683_off;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_ddc0,
        world_state_read_objects,
        "WorldState_ReadObjects",
    ),
    (
        0x0026_de58,
        world_state_notify_objects_restored,
        "WorldState_NotifyObjectsRestored",
    ),
    (
        0x0026_e468,
        replay_find_next_snapshot,
        "Replay_FindNextSnapshot",
    ),
    (
        0x0026_e490,
        replay_find_prev_snapshot,
        "Replay_FindPrevSnapshot",
    ),
    (
        0x0026_e4b8,
        replay_snap_to_snapshot,
        "Replay_SnapToSnapshot",
    ),
    (
        0x0026_e528,
        replay_seek_next_snapshot,
        "Replay_SeekNextSnapshot",
    ),
    (
        0x0026_e568,
        replay_seek_prev_snapshot,
        "Replay_SeekPrevSnapshot",
    ),
    (0x0026_e5a8, snapshot_list_ctor, "SnapshotList_ctor"),
    (0x0026_e5c0, snapshot_list_clear, "SnapshotList_Clear"),
    (0x0026_e608, snapshot_list_remove, "SnapshotList_Remove"),
    (
        0x0026_e670,
        snapshot_list_pop_front,
        "SnapshotList_PopFront",
    ),
    (
        0x0026_e800,
        replay_highlight_slot_clear,
        "Replay_HighlightSlotClear",
    ),
    (
        0x0026_e838,
        replay_highlight_slot_score,
        "Replay_HighlightSlotScore",
    ),
    (0x0026_e8e0, replay_stream_ctor, "ReplayStream_ctor"),
    (
        0x0026_e6e0,
        replay_highlight_slot_begin,
        "Replay_HighlightSlotBegin",
    ),
    (
        0x0026_e7f8,
        replay_highlight_slot_detach,
        "Replay_HighlightSlotDetach",
    ),
    (
        0x0026_ded8,
        world_state_create_object,
        "WorldState_CreateObject",
    ),
    (
        0x0026_e340,
        world_state_read_dead_objects,
        "WorldState_ReadDeadObjects",
    ),
    (
        0x0026_e6a0,
        snapshot_list_find_by_frame,
        "SnapshotList_FindByFrame",
    ),
    (0x0026_dbf0, world_state_read_2, "WorldState_Read"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;

    pub(crate) use crate::game::bx::bxstringctor::callee::SNAPSHOT_LIST_REMOVE;

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::REPLAY_FIND_PREV_SNAPSHOT;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::REPLAY_SNAP_TO_SNAPSHOT;
}

/// `SnapshotList *SnapshotList_ctor(SnapshotList *list)` (0x26e5a8): zero the
/// head/tail/count words and return the list.
pub fn snapshot_list_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    list.off(0x4).sw(c, 0);
    list.sw(c, 0);
    list.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void SnapshotList_Clear(SnapshotList *list)` (0x26e5c0): zero the
/// head/tail/count words.
pub fn snapshot_list_clear(c: &mut Cpu) {
    c.cycles += 4;
    let list = Addr(abi::arg(c, 0));
    list.off(0x8).sw(c, 0);
    list.off(0x4).sw(c, 0);
    list.sw(c, 0);
    abi::ret(c);
}

/// `void SnapshotList_Remove(SnapshotList *list, Snapshot *node)` (0x26e608):
/// unlink `node` from the doubly linked list and decrement the count.
pub fn snapshot_list_remove(c: &mut Cpu) {
    const HEAD: u32 = 0x0;
    const TAIL: u32 = 0x4;
    const COUNT: u32 = 0x8;
    const NEXT: u32 = 0x14;
    const PREV: u32 = 0x18;

    let list = Addr(abi::arg(c, 0));
    let node = c.g[A1][0];
    let node_at = Addr(node as u32);

    c.cycles += 3;
    let head = list.off(HEAD).lw(c);
    c.g[V0][0] = head;
    let tail;
    if node != head {
        tail = list.off(TAIL).lw(c);
        c.g[V0][0] = tail;
    } else {
        c.cycles += 3;
        let next = node_at.off(NEXT).lw(c);
        c.g[V0][0] = next;
        list.off(HEAD).sw(c, next);
        tail = list.off(TAIL).lw(c);
        c.g[V0][0] = tail;
    }

    c.cycles += 2;
    let mut next;
    if node != tail {
        next = node_at.off(NEXT).lw(c);
        c.g[V1][0] = next;
    } else {
        c.cycles += 3;
        let prev = node_at.off(PREV).lw(c);
        c.g[V0][0] = prev;
        list.off(TAIL).sw(c, prev);
        next = node_at.off(NEXT).lw(c);
        c.g[V1][0] = next;
    }

    c.cycles += 2;
    let mut prev;
    if next == 0 {
        prev = node_at.off(PREV).lw(c);
        c.g[V1][0] = prev;
    } else {
        c.cycles += 3;
        prev = node_at.off(PREV).lw(c);
        c.g[V0][0] = prev;
        Addr(next as u32).off(PREV).sw(c, prev);
        prev = node_at.off(PREV).lw(c);
        c.g[V1][0] = prev;
    }

    c.cycles += 2;
    if prev == 0 {
        node_at.off(PREV).sw(c, 0);
    } else {
        c.cycles += 3;
        next = node_at.off(NEXT).lw(c);
        c.g[V0][0] = next;
        Addr(prev as u32).off(NEXT).sw(c, next);
        node_at.off(PREV).sw(c, 0);
    }

    c.cycles += 5;
    node_at.off(NEXT).sw(c, 0);
    let count = list.off(COUNT).lw(c);
    let result = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = result;
    list.off(COUNT).sw(c, result);
    abi::ret(c);
}

/// `void Replay_HighlightSlotClear(Replay *replay)` (0x26e800): clear the
/// highlight bit for the currently selected slot in its slot byte, then drop
/// the slot pointer.
pub fn replay_highlight_slot_clear(c: &mut Cpu) {
    const SLOT_INDEX: u32 = 0x0;
    const SLOT_PTR: u32 = 0xb0;
    const SLOT_FLAGS: u32 = 0x1e;

    c.cycles += 4;
    let replay = Addr(abi::arg(c, 0));
    c.g[A2][0] = c.g[A0][0];
    let slot = replay.off(SLOT_PTR).lw(c);
    c.g[A1][0] = slot;
    c.g[V1][0] = 1;
    if slot != 0 {
        c.cycles += 7;
        let index = replay.off(SLOT_INDEX).lw(c);
        c.g[V0][0] = index;
        let flags = Addr(slot as u32).off(SLOT_FLAGS).lbu(c);
        c.g[A0][0] = flags;
        c.g[V1][0] = sx(1u32 << (index as u32 & 31));
        c.g[V1][0] = !(0u64 | c.g[V1][0]);
        c.g[A0][0] = flags & c.g[V1][0];
        Addr(slot as u32).off(SLOT_FLAGS).sb(c, c.g[A0][0]);
        replay.off(SLOT_PTR).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `ReplayStream *ReplayStream_ctor(ReplayStream *stream)` (0x26e8e0):
/// install the vtable and zero the buffer fields; returns the stream.
pub fn replay_stream_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 + 0x1898;

    c.cycles += 9;
    let stream = Addr(abi::arg(c, 0));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    stream.off(0x4).sw(c, 0);
    stream.sw(c, vtable);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    stream.off(0x10).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay snapshot stepping.

/// `Snapshot *Replay_FindNextSnapshot(Replay *r, int frame)` (0x26e468): the
/// snapshot found for `frame`, or its `+0x14` successor when it has one.
pub fn replay_find_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e478)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

/// `Snapshot *Replay_FindPrevSnapshot(Replay *r, int frame)` (0x26e490): the
/// snapshot found for `frame`, or its `+0x18` predecessor when it has one.
pub fn replay_find_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e4a0)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

/// `SnapshotList_PopFront(list)` 0x26e670: unlink and return the list's first
/// snapshot node.
pub fn snapshot_list_pop_front(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let list = abi::arg(c, 0);
        let node = c.lw(list);
        c.g[S0][0] = node;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SNAPSHOT_LIST_REMOVE.call(c, 0x0026_e688)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekNextSnapshot(Replay* self)` (0x26E528): find the snapshot
/// after the current one (+0x484 index), keep it at +0x488, advance the index
/// to its successor and snap the replay there.
pub fn replay_seek_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r_2(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_NEXT_SNAPSHOT.call(c, 0x0026_e540)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r_2(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e554)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekPrevSnapshot(Replay* self)` (0x26E568): the same one
/// snapshot backwards (index at +0x484, snapshot at +0x488).
pub fn replay_seek_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r_2(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_PREV_SNAPSHOT.call(c, 0x0026_e580)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r_2(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e594)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SnapToSnapshot(Replay* r)` (0x26e4b8): seek both replay
/// streams to the snapshot nearest the current frame.
pub fn replay_snap_to_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48c));
        c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
        call::call_ptr(c, 0x0026_e448, 0x0026_e4e0)?;
        Addr(p(r32(c, S0), 0x488)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lw(c);
        Addr(p(r32(c, S0), 0x484)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
                call::call_ptr(c, 0x0026_d4d8, 0x0026_e508)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_e4f0);
                }
                continue;
            }
            c.cycles += 6;
            lqsp(c, 16, 0x30);
            lqsp(c, 17, 0x20);
            lqsp(c, 18, 0x10);
            frame.restore_ra(c, 0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = ra;
            return Ok(());
        }
    });
}

/// `void WorldState_NotifyObjectsRestored(WorldState*, void* arg)`
/// (0x0026de58): walk the process list at `gp+0x2898`, virtual-call each
/// process with `arg`, then hand the process back to the list iterator.
pub fn world_state_notify_objects_restored(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A0][0] = sx(gpa(c, 0x2898));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_4ea8, 0x0026_de7c)?; // ProcessList_First
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x0026_dea0)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0035_4ef8, 0x0026_deb0)?; // ProcessList_Next
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.lq(S0, sp(c).wrapping_add(0x30));
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de90);
                }
            }
        } else {
            c.lq(S0, sp(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `WorldState_ReadObjects` (0x26ddc0): read the object count and create each
/// world object, then finalize the list.
pub fn world_state_read_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let this = r(c, A0);
        c.g[S3][0] = sx(this);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = Addr(r(c, S1)).lw(c);
        c.g[A0][0] = Addr(p_2(r(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p_2(r(c, V0), 0x14)).lw(c);
        let target = r(c, V1);
        c.g[RA][0] = sx(0x0026_de04);
        c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0026_de04)?;
        c.g[V0][0] = Addr(r(c, SP)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S3][0];
        if !empty {
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0026_de20);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                callee::WORLD_STATE_CREATE_OBJECT.call(c, 0x0026_de20)?;
                c.g[V0][0] = Addr(r(c, SP)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de10);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(r(c, GP).wrapping_add(0x2898u32));
        c.g[RA][0] = sx(0x0026_de3c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::WORLD_STATE_FINALIZE, 0x0026_de3c)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay highlight.

/// `int Replay_HighlightSlotScore(self, replay, mode)` (0x26e838): compare a
/// replay slot's distance (or time) against the current highlight and set it
/// when the slot is better.
pub fn replay_highlight_slot_score(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        'end: {
            'mid: {
                'obj: {
                    'inner: {
                        'dist: {
                            'cmp: {
                                c.cycles += 3;
                                let t = c.g[A2][0] == 0;
                                frame.save_ra(c, 0);
                                if t {
                                    break 'cmp;
                                }
                                c.cycles += 5;
                                c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0x9c));
                                c.g[V0][0] = lw_2(c, at(c.g[A0][0], 0xa0));
                                c.g[A2][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                let t = c.g[A2][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'end;
                                }
                                'gate: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 1000i64) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.lwc1(1, at(c.g[A1][0], 0x44));
                                        break 'gate;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                                    break 'inner;
                                }
                                c.cycles += 6;
                                c.g[AT][0] = 0x40a0_0000;
                                c.mtc1(1, 0);
                                c.c_le_s(0, 1);
                                if !c.fcc() {
                                    c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0xa4));
                                    break 'dist;
                                }
                                c.cycles += 8;
                                c.g[AT][0] = 0x447a_0000;
                                c.mtc1(1, 0);
                                c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                                c.mul_s(0, 1, 0);
                                c.cvt_w_s(1, 0);
                                c.mfc1(6, 1);
                                c.g[A0][0] = lw_2(c, at(c.g[A0][0], 0xb0));
                                break 'obj;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0xa4));
                            break 'dist;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = lw_2(c, at(c.g[A0][0], 0xa8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        let t = c.g[V1][0] == 0;
                        c.g[A2][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                        if t {
                            break 'mid;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                        break 'inner;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = lw_2(c, at(c.g[A0][0], 0xb0));
                    break 'obj;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0026_e8c8);
                call_ptr(c, 0x0026_d730, 0x0026_e8c8)?;
                c.g[V0][0] = 1;
                break 'end;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'end;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_HighlightSlotBegin (0x26e6e0)

/// `Replay_SnapshotSetHighlightValue(rider, index, 0)` (0x26d730).
pub(crate) const REPLAY_HIGHLIGHT_SET_VALUE: u64 = 0x0026_d730;

/// `sReplayObj` fields: the slot index, its data block, the attached rider.
pub(crate) const REPLAY_HIGHLIGHT_INDEX: u32 = 0x00;

pub(crate) const REPLAY_HIGHLIGHT_DATA: u32 = 0x04;

pub(crate) const REPLAY_HIGHLIGHT_RIDER: u32 = 0xb0;

/// The data block: 0xa0 bytes copied in 0x20-byte chunks, then 0xc more.
pub(crate) const REPLAY_HIGHLIGHT_CHUNK: u32 = 0x20;

pub(crate) const REPLAY_HIGHLIGHT_BULK: u32 = 0xa0;

/// The rider's highlight flag byte (bit per slot).
pub(crate) const REPLAY_HIGHLIGHT_FLAGS: u32 = 0x1e;

/// `void Replay_HighlightSlotBegin(sReplayObj* self, char* rider, sReplayData* src)`
/// (0x26e6e0): mark the rider's per-slot highlight bit, tell the snapshot
/// system, attach the rider to the slot and copy its 0xac-byte data block.
/// Frame 0x40.
pub fn replay_highlight_slot_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // rider
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0]; // src
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_INDEX));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_FLAGS));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.sb(
            (c.g[S0][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_FLAGS),
            c.g[V1][0],
        );
        c.g[RA][0] = sx(0x0026_e728);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_INDEX));
        call_ptr(c, REPLAY_HIGHLIGHT_SET_VALUE as u32, 0x0026_e728)?;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_DATA));
        c.g[V0][0] = c.g[S2][0] | c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.sw(
            (c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_RIDER),
            c.g[S0][0],
        );
        if aligned {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_BULK));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[S2][0] as u32);
                c.g[A1][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                let more = c.g[S2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_e798);
                    }
                    continue;
                }
                break;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_BULK));
            loop {
                c.cycles += 19;
                c.ldl(4, (c.g[S2][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[S2][0] as u32);
                c.ldl(5, (c.g[S2][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.ldl(6, (c.g[S2][0] as u32).wrapping_add(0x17));
                c.ldr(6, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.ldl(7, (c.g[S2][0] as u32).wrapping_add(0x1f));
                c.ldr(7, (c.g[S2][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[V1][0] as u32);
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                let more = c.g[S2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_e740);
                    }
                    continue;
                }
                c.cycles += 2;
                break;
            }
        }
        // The tail: the data block's last 0xc bytes.
        c.cycles += 12;
        c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[S2][0] as u32);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ Replay_HighlightSlotDetach

/// The replay highlight slot's attached object (`+0xb0`).
pub(crate) const REPLAY_HIGHLIGHT_SLOT_OBJECT: u32 = 0xb0;

/// `void Replay_HighlightSlotDetach(ReplayHighlightSlot* slot)` (0x26e7f8):
/// forget the slot's attached object (`slot->object = NULL`).
pub fn replay_highlight_slot_detach(c: &mut Cpu) {
    c.cycles += 2;
    let slot = Addr(abi::arg(c, 0));
    slot.off(REPLAY_HIGHLIGHT_SLOT_OBJECT).sw(c, 0);
    ret(c);
}

/// `cReplay_restoreDeadBucket` (0x26e340): reads the `{count, pad}` header of
/// a dead-object bucket from `stream` through its virtual `read`, then
/// `count` 4-byte refs. Each ref is resolved to a node (`ref.get()`, 0 when
/// the table holds none) and a 0x1c-byte `cDeadNode` is allocated
/// (`str_Dead_Object`) and constructed on it.
pub fn world_state_read_dead_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[A2][0] = 8;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = 0;
        let stream = c.g[S1][0] as u32;
        c.g[A3][0] = c.lw(stream);
        let ref_slot = frame.at(c, 0x10);
        let minus_one = c.g[V0][0];
        c.sw(ref_slot, minus_one);
        // stream->read(&hdr, 8): `this` moves by the delta at vtable+0x10.
        let vt = c.g[A3][0] as u32;
        c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V1][0] = c.lw(vt.wrapping_add(0x14));
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        let read = c.g[V1][0] as u32;
        call_ptr(c, read, 0x0026_e384)?;
        let hdr = sp(c);
        c.g[V0][0] = c.lw(hdr);
        let count = c.g[V0][0] as i64;
        c.g[S2][0] = 0x48_0000;
        if count > 0 {
            c.cycles += 2;
            c.g[V1][0] = c.lw(stream);
            loop {
                c.cycles += 22;
                let ref_addr = sp(c).wrapping_add(0x10);
                c.g[A1][0] = sx(ref_addr);
                c.g[A2][0] = 4;
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
                c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
                let read = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, read, 0x0026_e3b0)?;
                // cMemMan_alloc(0x1c, str_Dead_Object, 0x20000000, 0)
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1720));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0026_e3c4)?;
                c.g[A2][0] = c.g[V0][0];
                let ref_word = c.lw(ref_addr);
                c.g[A1][0] = ref_word;
                // ref.get(): table gp+0x16c8 -> ctx -> [+8] selects the entry by the low byte.
                let ctx_slot = (c.g[GP][0] as u32).wrapping_add(0x16c8);
                c.g[V0][0] = c.lw(ctx_slot);
                c.g[V1][0] = c.g[A1][0] & 0xff;
                let ctx = c.g[V0][0] as u32;
                c.g[A0][0] = c.lw(ctx);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                let table = c.g[A0][0] as u32;
                c.g[V0][0] = c.lw(table.wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let entry_slot = c.g[V1][0] as u32;
                c.g[V1][0] = c.lw(entry_slot);
                let entry_null = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                if entry_null {
                    c.cycles += 1;
                    c.g[A1][0] = 0;
                } else {
                    c.cycles += 7;
                    let entry = c.g[V1][0] as u32;
                    c.g[A0][0] = c.lw(entry.wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    let base = c.g[A0][0] as u32;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(base));
                    let node_slot = c.g[V0][0] as u32;
                    c.g[V1][0] = c.lw(node_slot);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    if !found {
                        c.cycles += 1;
                        c.g[A1][0] = 0;
                    }
                }
                // cDeadNode_cDeadNode(mem, node)
                c.cycles += 7;
                c.g[A0][0] = c.g[A2][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0035_06d8, 0x0026_e41c)?;
                let hdr = sp(c);
                c.g[V0][0] = c.lw(hdr);
                let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V1][0] = c.lw(stream);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_e398);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cReplayFrameList_findFrame` (0x26e6a0): the frame of a list (`a0`, its
/// head at +0) whose time is `a1`, or the frame that brackets it. Walks the
/// `next` links (+0x14) and stops at the last frame, at a frame whose time
/// is `a1`, or at the one just before a time greater than `a1`. Frame times
/// sit at +0x30.
pub fn snapshot_list_find_by_frame(c: &mut Cpu) {
    c.cycles += 2;
    let list = c.g[A0][0] as u32;
    c.g[A0][0] = c.lw(list);
    loop {
        c.cycles += 3;
        let frame = c.g[A0][0] as u32;
        c.g[V1][0] = c.lw(frame.wrapping_add(0x14));
        if c.g[V1][0] == 0 {
            break;
        }
        c.cycles += 3;
        let time = c.lw(frame.wrapping_add(0x30));
        c.g[V0][0] = time;
        let target = c.g[A1][0];
        let same = time == target;
        c.g[V0][0] = ((time as i64) < (target as i64)) as u64;
        if same {
            break;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.lw(frame.wrapping_add(0x14));
            if event_due(c) {
                deliver_event_here(c, 0x0026_e6a8);
            }
            continue;
        }
        c.cycles += 4;
        let next = c.g[V1][0] as u32;
        let next_time = c.lw(next.wrapping_add(0x30));
        c.g[V0][0] = next_time;
        c.g[V0][0] = ((target as i64) < (next_time as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.lw(frame.wrapping_add(0x14));
            if event_due(c) {
                deliver_event_here(c, 0x0026_e6a8);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `WorldState_CreateObject` (0x0026ded8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn world_state_create_object(c: &mut Cpu) {
    if c.pc != 0x0026ded8 {
        return ee::s07::f_0026ded8__generated(c);
    }
    guest(c, |c| {
        'b_26e32c: {
            'b_26e328: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A2][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A1][0];
                c.g[RA][0] = sx(0x0026def8);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0026ea68,
                    f: ee::s07::ReplayStream_PeekInt,
                }
                .call(c, 0x0026def8)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x17u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    break 'b_26e328;
                } else {
                    'b_26e2e0: {
                        'b_26e2b0: {
                            'b_26e280: {
                                'b_26e250: {
                                    'b_26e220: {
                                        'b_26e1f0: {
                                            'b_26e1c0: {
                                                'b_26e190: {
                                                    'b_26e160: {
                                                        'b_26e130: {
                                                            'b_26e100: {
                                                                'b_26e0d0: {
                                                                    'b_26e0a0: {
                                                                        'b_26e070: {
                                                                            'b_26e040: {
                                                                                'b_26e010: {
                                                                                    'b_26dfe0: {
                                                                                        'b_26dfb0: {
                                                                                            'b_26df80: {
                                                                                                'b_26df50: {
                                                                                                    c.cycles += 6;
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16c0));
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                    let t = c.g[A0][0] as u32;
                                                                                                    match t {
0x0026df20 => {
c.cycles += 12;
c.g[A1][0] = 0x480000;
c.g[A0][0] = 0x34;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15f0));
c.g[A2][0] = 0x20000000;
c.g[RA][0] = sx(0x0026df38);
c.g[A3][0] = 0u64;
call_ptr(c, 0x00317d70, 0x0026df38)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0026df48);
c.g[A2][0] = c.g[S1][0];
call_ptr(c, 0x00342d10, 0x0026df48)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_26e32c;
} else {
break 'b_26df50;
}
}
0x0026df50 => {
break 'b_26df50;
}
0x0026df80 => {
break 'b_26df80;
}
0x0026dfb0 => {
break 'b_26dfb0;
}
0x0026dfe0 => {
break 'b_26dfe0;
}
0x0026e010 => {
break 'b_26e010;
}
0x0026e040 => {
break 'b_26e040;
}
0x0026e070 => {
break 'b_26e070;
}
0x0026e0a0 => {
break 'b_26e0a0;
}
0x0026e0d0 => {
break 'b_26e0d0;
}
0x0026e100 => {
break 'b_26e100;
}
0x0026e130 => {
break 'b_26e130;
}
0x0026e160 => {
break 'b_26e160;
}
0x0026e190 => {
break 'b_26e190;
}
0x0026e1c0 => {
break 'b_26e1c0;
}
0x0026e1f0 => {
break 'b_26e1f0;
}
0x0026e220 => {
break 'b_26e220;
}
0x0026e250 => {
break 'b_26e250;
}
0x0026e280 => {
break 'b_26e280;
}
0x0026e2b0 => {
break 'b_26e2b0;
}
0x0026e2e0 => {
break 'b_26e2e0;
}
0x0026e328 => {
break 'b_26e328;
}
_ => { c.jump_out(t, 0x0026ded8, 0x0026e33c); return Ok(()); }
}
                                                                                                }
                                                                                                c.cycles += 12;
                                                                                                c.g[A1][0] = 0x480000;
                                                                                                c.g[A0][0] = 0x6c;
                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1600));
                                                                                                c.g[A2][0] = 0x20000000;
                                                                                                c.g[RA][0] = sx(0x0026df68);
                                                                                                c.g[A3][0] = 0u64;
                                                                                                call_ptr(c, 0x00317d70, 0x0026df68)?;
                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                c.g[A1][0] = c.g[S0][0];
                                                                                                c.g[RA][0] = sx(0x0026df78);
                                                                                                c.g[A2][0] = c.g[S1][0];
                                                                                                call_ptr(c, 0x00341c80, 0x0026df78)?;
                                                                                                let t = true;
                                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_26e32c;
                                                                                                } else {
                                                                                                    break 'b_26df80;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 12;
                                                                                            c.g[A1][0] = 0x480000;
                                                                                            c.g[A0][0] = 0x70;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1610));
                                                                                            c.g[A2][0] = 0x20000000;
                                                                                            c.g[RA][0] = sx(0x0026df98);
                                                                                            c.g[A3][0] = 0u64;
                                                                                            call_ptr(c, 0x00317d70, 0x0026df98)?;
                                                                                            c.g[A0][0] = c.g[V0][0];
                                                                                            c.g[A1][0] = c.g[S0][0];
                                                                                            c.g[RA][0] = sx(0x0026dfa8);
                                                                                            c.g[A2][0] = c.g[S1][0];
                                                                                            call_ptr(c, 0x00359270, 0x0026dfa8)?;
                                                                                            let t =
                                                                                                true;
                                                                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                            if t {
                                                                                                break 'b_26e32c;
                                                                                            } else {
                                                                                                break 'b_26dfb0;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            12;
                                                                                        c.g[A1][0] = 0x480000;
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0x88;
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1620));
                                                                                        c.g[A2][0] = 0x20000000;
                                                                                        c.g[RA][0] = sx(0x0026dfc8);
                                                                                        c.g[A3]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        call_ptr(c, 0x00317d70, 0x0026dfc8)?;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0];
                                                                                        c.g[A1]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x0026dfd8);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00358db0, 0x0026dfd8)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        if t {
                                                                                            break 'b_26e32c;
                                                                                        } else {
                                                                                            break 'b_26dfe0;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 12;
                                                                                    c.g[A1][0] =
                                                                                        0x480000;
                                                                                    c.g[A0][0] =
                                                                                        0x80;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1630));
                                                                                    c.g[A2][0] =
                                                                                        0x20000000;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0026dff8,
                                                                                    );
                                                                                    c.g[A3][0] =
                                                                                        0u64;
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00317d70,
                                                                                        0x0026dff8,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A1][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0026e008,
                                                                                    );
                                                                                    c.g[A2][0] =
                                                                                        c.g[S1][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003422e8,
                                                                                        0x0026e008,
                                                                                    )?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                    if t {
                                                                                        break 'b_26e32c;
                                                                                    } else {
                                                                                        break 'b_26e010;
                                                                                    }
                                                                                }
                                                                                c.cycles += 12;
                                                                                c.g[A1][0] =
                                                                                    0x480000;
                                                                                c.g[A0][0] = 0xb0;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1640));
                                                                                c.g[A2][0] =
                                                                                    0x20000000;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0026e028);
                                                                                c.g[A3][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x00317d70,
                                                                                    0x0026e028,
                                                                                )?;
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[A1][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x0026e038);
                                                                                c.g[A2][0] =
                                                                                    c.g[S1][0];
                                                                                call_ptr(
                                                                                    c, 0x003464e0,
                                                                                    0x0026e038,
                                                                                )?;
                                                                                let t = true;
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                if t {
                                                                                    break 'b_26e32c;
                                                                                } else {
                                                                                    break 'b_26e040;
                                                                                }
                                                                            }
                                                                            c.cycles += 12;
                                                                            c.g[A1][0] = 0x4a0000;
                                                                            c.g[A0][0] = 0x24;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x3440,
                                                                                    ));
                                                                            c.g[A2][0] = 0x20000000;
                                                                            c.g[RA][0] =
                                                                                sx(0x0026e058);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x0026e058,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x0026e068);
                                                                            c.g[A2][0] = c.g[S1][0];
                                                                            call_ptr(
                                                                                c, 0x003437c0,
                                                                                0x0026e068,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_26e32c;
                                                                            } else {
                                                                                break 'b_26e070;
                                                                            }
                                                                        }
                                                                        c.cycles += 12;
                                                                        c.g[A1][0] = 0x4a0000;
                                                                        c.g[A0][0] = 0x50;
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x3448,
                                                                                ));
                                                                        c.g[A2][0] = 0x20000000;
                                                                        c.g[RA][0] = sx(0x0026e088);
                                                                        c.g[A3][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00317d70,
                                                                            0x0026e088,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.g[RA][0] = sx(0x0026e098);
                                                                        c.g[A2][0] = c.g[S1][0];
                                                                        call_ptr(
                                                                            c, 0x00341548,
                                                                            0x0026e098,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        if t {
                                                                            break 'b_26e32c;
                                                                        } else {
                                                                            break 'b_26e0a0;
                                                                        }
                                                                    }
                                                                    c.cycles += 12;
                                                                    c.g[A1][0] = 0x4a0000;
                                                                    c.g[A0][0] = 0x10c;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x3450));
                                                                    c.g[A2][0] = 0x20000000;
                                                                    c.g[RA][0] = sx(0x0026e0b8);
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x0026e0b8,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026e0c8);
                                                                    c.g[A2][0] = c.g[S1][0];
                                                                    call_ptr(
                                                                        c, 0x0034cbe8, 0x0026e0c8,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    if t {
                                                                        break 'b_26e32c;
                                                                    } else {
                                                                        break 'b_26e0d0;
                                                                    }
                                                                }
                                                                c.cycles += 12;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A0][0] = 0x1c;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x3458));
                                                                c.g[A2][0] = 0x20000000;
                                                                c.g[RA][0] = sx(0x0026e0e8);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x0026e0e8,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x0026e0f8);
                                                                c.g[A2][0] = c.g[S1][0];
                                                                call_ptr(
                                                                    c, 0x0034ae68, 0x0026e0f8,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_26e32c;
                                                                } else {
                                                                    break 'b_26e100;
                                                                }
                                                            }
                                                            c.cycles += 12;
                                                            c.g[A1][0] = 0x480000;
                                                            c.g[A0][0] = 0x6c0;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1650));
                                                            c.g[A2][0] = 0x20000000;
                                                            c.g[RA][0] = sx(0x0026e118);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x0026e118)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x0026e128);
                                                            c.g[A2][0] = c.g[S1][0];
                                                            call_ptr(c, 0x00352168, 0x0026e128)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_26e32c;
                                                            } else {
                                                                break 'b_26e130;
                                                            }
                                                        }
                                                        c.cycles += 12;
                                                        c.g[A1][0] = 0x480000;
                                                        c.g[A0][0] = 0x2c;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1660));
                                                        c.g[A2][0] = 0x20000000;
                                                        c.g[RA][0] = sx(0x0026e148);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x0026e148)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026e158);
                                                        c.g[A2][0] = c.g[S1][0];
                                                        call_ptr(c, 0x00357918, 0x0026e158)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_26e32c;
                                                        } else {
                                                            break 'b_26e160;
                                                        }
                                                    }
                                                    c.cycles += 12;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = 0x218;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3460));
                                                    c.g[A2][0] = 0x20000000;
                                                    c.g[RA][0] = sx(0x0026e178);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x0026e178)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0026e188);
                                                    c.g[A2][0] = c.g[S1][0];
                                                    call_ptr(c, 0x00358588, 0x0026e188)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_26e32c;
                                                    } else {
                                                        break 'b_26e190;
                                                    }
                                                }
                                                c.cycles += 12;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = 0x1c;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x3468));
                                                c.g[A2][0] = 0x20000000;
                                                c.g[RA][0] = sx(0x0026e1a8);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x0026e1a8)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026e1b8);
                                                c.g[A2][0] = c.g[S1][0];
                                                call_ptr(c, 0x00350fd8, 0x0026e1b8)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_26e32c;
                                                } else {
                                                    break 'b_26e1c0;
                                                }
                                            }
                                            c.cycles += 12;
                                            c.g[A1][0] = 0x480000;
                                            c.g[A0][0] = 0x7c;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1670));
                                            c.g[A2][0] = 0x20000000;
                                            c.g[RA][0] = sx(0x0026e1d8);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x0026e1d8)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0026e1e8);
                                            c.g[A2][0] = c.g[S1][0];
                                            call_ptr(c, 0x0034ed88, 0x0026e1e8)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_26e32c;
                                            } else {
                                                break 'b_26e1f0;
                                            }
                                        }
                                        c.cycles += 12;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A0][0] = 0x48;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1680));
                                        c.g[A2][0] = 0x20000000;
                                        c.g[RA][0] = sx(0x0026e208);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x0026e208)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0026e218);
                                        c.g[A2][0] = c.g[S1][0];
                                        call_ptr(c, 0x003430d0, 0x0026e218)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_26e32c;
                                        } else {
                                            break 'b_26e220;
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = 0x1c;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3470));
                                    c.g[A2][0] = 0x20000000;
                                    c.g[RA][0] = sx(0x0026e238);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x0026e238)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026e248);
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x00350f08, 0x0026e248)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_26e32c;
                                    } else {
                                        break 'b_26e250;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = 0x4a0000;
                                c.g[A0][0] = 0x2c;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3478));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x0026e268);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x0026e268)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0026e278);
                                c.g[A2][0] = c.g[S1][0];
                                call_ptr(c, 0x00356e60, 0x0026e278)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_26e32c;
                                } else {
                                    break 'b_26e280;
                                }
                            }
                            c.cycles += 12;
                            c.g[A1][0] = 0x480000;
                            c.g[A0][0] = 0x80;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1690));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0026e298);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0026e298)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0026e2a8);
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00348fa0, 0x0026e2a8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_26e32c;
                            } else {
                                break 'b_26e2b0;
                            }
                        }
                        c.cycles += 12;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x80;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16a0));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x0026e2c8);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x0026e2c8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0026e2d8);
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x00349db0, 0x0026e2d8)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_26e32c;
                        } else {
                            break 'b_26e2e0;
                        }
                    }
                    c.cycles += 13;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x3c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16b0));
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x0026e2f8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0026e2f8)?;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0026e308);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00350b98, 0x0026e308)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_26e328;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0026e328);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0026e328)?;
                        break 'b_26e328;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_26e32c;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void WorldState_Read(WorldState *self, Stream *s)` (0x26dbf0): restores
/// one replay frame from the stream `s` (a vtable read) into the world: the
/// buckets are merged and purged, the map reset, each subsystem's
/// `readFromReplayFrame` runs in turn, then the objects are notified and
/// the AI and game view updated. Frame 0x70: saves ra and s0-s2.
pub fn world_state_read_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = sx(oc683_off(c.g[GP][0], 0x2898));
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);

        // Buckets: merge and purge the bucket manager, reset the world map.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0026_dc1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_5118, 0x0026_dc28)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_5118, 0x0026_dc34)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0x16c8));
        call_ptr(c, 0x003a_6800, 0x0026_dc3c)?;
        c.g[A0][0] = c.g[S1][0];
        oc683_direct(0x0026_e9b0).call(c, 0x0026_dc44)?;

        // Stream read (vtable slot 0x10): the frame header into sp.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc60)?;

        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0031_7908, 0x0026_dc68)?;

        // Stream read again (vtable slot 0x10) into sp+0x20.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx(oc683_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 4;
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc88)?;

        // The same slot again, through A3, with the word at sp+0x20 stored
        // to the global at gp+0x2a88.
        c.g[A3][0] = c.lw(c.g[S1][0] as u32);
        c.g[A2][0] = 4;
        c.g[V0][0] = c.lw(oc683_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lh(oc683_off(c.g[A3][0], 0x10));
        c.g[V1][0] = c.lw(oc683_off(c.g[A3][0], 0x14));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let t = c.g[V1][0] as u32;
        let gp2a88 = oc683_off(c.g[GP][0], 0x2a88);
        c.sw(gp2a88, c.g[V0][0]);
        call_ptr(c, t, 0x0026_dcac)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[SP][0], 0x20));
        let gp2a8c = oc683_off(c.g[GP][0], 0x2a8c);
        c.sw(gp2a8c, c.g[V0][0]);
        call_ptr(c, 0x002c_03e8, 0x0026_dcb8)?;

        // cBE_readFromReplayFrame(cBE_getBE(), s).
        call_ptr(c, 0x0014_dc80, 0x0026_dcc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0014_dfa8, 0x0026_dccc)?;

        // Vtable call on world->view (+0x84), slot 0x48, with A0 = view + delta.
        c.g[V1][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(oc683_off(c.g[V1][0], 0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh(oc683_off(c.g[V0][0], 0x48));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x4c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dcec)?;

        // Vtable call on view->+0xc, slot 0x40 (the field at +0xcc).
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc683_off(c.g[A0][0], 0x84));
        c.g[A2][0] = c.lw(oc683_off(c.g[V0][0], 0xc));
        c.g[V1][0] = c.lw(oc683_off(c.g[A2][0], 0xcc));
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x40));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x44));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dd10)?;

        // Object and stream subsystems: each reads from the replay frame.
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002d_9d68, 0x0026_dd18)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xce8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_bd20, 0x0026_dd24)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x6f0));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9e58, 0x0026_dd30)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xf00));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_4240, 0x0026_dd3c)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xf38));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7d28, 0x0026_dd48)?;

        // WorldState_ReadObjects(self, 1, s); WorldState_ReadDeadObjects(self, s).
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_ddc0).call(c, 0x0026_dd58)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        oc683_direct(0x0026_e340).call(c, 0x0026_dd64)?;

        // Field of the view's object (+0x84 -> +0xc -> +0xa4): the 0x103578 call.
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc683_off(c.g[A0][0], 0x84));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0xc));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0xa4));
        call_ptr(c, 0x0010_3578, 0x0026_dd7c)?;

        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_de58).call(c, 0x0026_dd88)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0xc));
        call_ptr(c, 0x0012_b788, 0x0026_dd98)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0x84));
        call_ptr(c, 0x0022_e840, 0x0026_dda8)?;

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ai::ai::tests::wf_2 as wf;
    pub(crate) use crate::game::bx::bxstringctor::tests::game;

    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    use ssx3_ee as ee;

    #[test]
    fn snapshot_list_ctor_matches() {
        check(
            ssx3_ee::s07::f_0026e5a8,
            snapshot_list_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snapshot_list_clear_matches() {
        check(
            ssx3_ee::s07::f_0026e5c0,
            snapshot_list_clear,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snapshot_list_remove_matches() {
        check(
            ssx3_ee::s07::f_0026e608,
            snapshot_list_remove,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_000f ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                let a = DATA + 0x100;
                let b = DATA + 0x120;
                let cc = DATA + 0x140;
                // a <-> b <-> cc
                c.bus.write32(list, a);
                c.bus.write32(list + 4, cc);
                c.bus.write32(list + 8, 3);
                c.bus.write32(a + 0x14, b);
                c.bus.write32(a + 0x18, 0);
                c.bus.write32(b + 0x14, cc);
                c.bus.write32(b + 0x18, a);
                c.bus.write32(cc + 0x14, 0);
                c.bus.write32(cc + 0x18, b);
                let node = match seed % 4 {
                    0 => a,            // head (prev == 0)
                    1 => cc,           // tail (next == 0)
                    2 => b,            // middle (both non-zero)
                    _ => DATA + 0x200, // not in the list
                };
                if seed % 4 == 3 {
                    c.bus.write32(DATA + 0x200 + 0x14, DATA + 0x300);
                    c.bus.write32(DATA + 0x200 + 0x18, DATA + 0x320);
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                    c.bus.write32(DATA + 0x320 + i * 4, r.next_u32());
                }
                c.g[A0][0] = list as u64;
                c.g[A1][0] = node as u64;
            },
        );
    }

    #[test]
    fn replay_highlight_slot_clear_matches() {
        check(
            ssx3_ee::s07::f_0026e800,
            replay_highlight_slot_clear,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let slot = DATA + 0x100;
                // Cover both the empty-slot and clear-bit paths.
                c.bus
                    .write32(DATA + 0xb0, if seed % 2 == 0 { 0 } else { slot });
                c.bus.write32(
                    DATA,
                    match seed % 3 {
                        0 => (seed % 31) as u32,
                        1 => 31,
                        _ => r.next_u32(),
                    },
                );
                c.bus.write8(slot + 0x1e, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn replay_stream_ctor_matches() {
        check(
            ssx3_ee::s07::f_0026e8e0,
            replay_stream_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..5 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------ replay snapshots

    /// A two-snapshot list at `+0x3b0` of the replay object; the frame searched
    /// for picks whether the found snapshot is the last one or not.
    pub(crate) fn snapshot_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 1);
        let replay = SCRATCH;
        c.g[A0][0] = replay as u64;
        c.g[A1][0] = (if seed % 2 == 0 { 100u32 } else { 250u32 }) as u64;
        let a = SCRATCH + 0x200;
        let b = SCRATCH + 0x300;
        c.bus.write32(replay + 0x3b0, a);
        // a: frame 100, next/prev b; b: frame 200, end of the list.
        c.bus.write32(a + 0x14, b);
        c.bus.write32(a + 0x18, b);
        c.bus.write32(b + 0x14, 0);
        c.bus.write32(b + 0x18, 0);
        c.bus.write32(a + 0x30, 100);
        c.bus.write32(b + 0x30, 200u32 + r.next_u32() % 4);
        c.bus.write32(a + 0x34, r.next_u32() % 16);
    }

    #[test]
    fn replay_find_next_snapshot_matches() {
        check(
            ee::s07::f_0026e468__generated,
            replay_find_next_snapshot,
            100,
            &|c, seed| {
                snapshot_setup(c, seed, 0x26e468);
            },
        );
    }

    #[test]
    fn replay_find_prev_snapshot_matches() {
        check(
            ee::s07::f_0026e490__generated,
            replay_find_prev_snapshot,
            100,
            &|c, seed| {
                snapshot_setup(c, seed, 0x26e490);
            },
        );
    }

    #[test]
    fn snapshot_list_pop_front_matches() {
        check(
            ssx3_ee::s07::f_0026e670__generated,
            snapshot_list_pop_front,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc8, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Empty list (null head) or one node with no neighbours.
                let head = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA, head);
                c.bus.write32(DATA + 0x100 + 0x14, 0);
                c.bus.write32(DATA + 0x100 + 0x18, 0);
            },
        );
    }

    #[test]
    fn replay_seek_next_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e528__generated,
            replay_seek_next_snapshot,
            60,
            &|c, seed| {
                machine(c, seed, 0x26e5_28, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x484, seed % 4);
            },
        );
    }

    #[test]
    fn replay_seek_prev_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e568__generated,
            replay_seek_prev_snapshot,
            60,
            &|c, seed| {
                machine(c, seed, 0x26e5_68, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x484, seed % 4);
            },
        );
    }

    #[test]
    fn replay_snap_to_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e4b8__generated,
            replay_snap_to_snapshot,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26e4b8, 0);
                let replay = DATA + 0x1000;
                c.g[A0][0] = replay as u64;
                w32(c, replay + 0x484, r.next_u32());
                // Empty snapshot list and streams.
                w32(c, replay + 0x3b0, 0);
                w32(c, replay + 0x48c, 0);
                w32(c, replay + 0x490, 0);
            },
        );
    }

    #[test]
    fn world_state_notify_objects_restored_matches() {
        check(
            ee::s07::f_0026de58__generated,
            world_state_notify_objects_restored,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x26de, 1);
                let list = GAME_GP.wrapping_add(0x2898);
                let arg = DATA + 0x300;
                let proc = DATA + 0x400;
                let proc2 = DATA + 0x600;
                // `ProcessList_First(list, arg)`: `A1 = arg + lw(list+4)*0x44`,
                // `V0 = lw(A1+8)`; it is 0 when that equals `A1+0x14`.
                let n = (seed % 3) as u32; // 0: empty list, 1: one process
                w(c, list + 4, n);
                if n == 0 {
                    w(c, arg + 8, arg.wrapping_add(0x14));
                } else {
                    w(c, arg + 0x44 + 8, proc);
                    w(c, proc + 0xc, DATA + 0x500);
                    w(c, DATA + 0x500 + 0x18, 0);
                    w(c, DATA + 0x500 + 0x1c, FAKE);
                    // `ProcessList_Next` returns `lw(proc+4)` unless it equals
                    // `arg + n*0x44 + 0x14`; 0 ends the walk.
                    w(c, proc + 4, if seed % 2 == 0 { 0 } else { proc2 });
                    w(c, proc2 + 0xc, DATA + 0x520);
                    w(c, DATA + 0x520 + 0x18, 0);
                    w(c, DATA + 0x520 + 0x1c, FAKE);
                }
                w(c, arg, 1);
                c.g[A1][0] = sx(arg);
            },
        );
    }

    #[test]
    fn world_state_read_objects_matches() {
        check(
            ee::s07::f_0026ddc0__generated,
            world_state_read_objects,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = DATA + 0x900;
                w(c, obj, vt);
                slot_2(c, vt, 0x10, FAKE);
                w(
                    c,
                    0x01f0_0000,
                    if seed & 1 == 0 { 0 } else { 1 + (seed % 2) },
                );
                w(c, this, 0);
                let game = DATA + 0xa00;
                w(c, this + 0x84, game);
                let _ = r;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0xb00) as u64;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    // ------------------------------------------------------- replay highlight

    #[test]
    fn replay_highlight_slot_score_matches() {
        check(
            ee::s07::f_0026e838__generated,
            replay_highlight_slot_score,
            SEEDS_2,
            &|c, seed| {
                let _ = base_3(c, seed, 0x26e8_38, 4);
                let obj = SCRATCH_2;
                let rep = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = rep as u64;
                c.g[A2][0] = (seed % 3) as u64;
                let d = (seed % 5) as i32 * 400;
                w(c, rep + 0x9c, (1000 + d) as u32);
                w(c, obj + 0xa0, 1000);
                wf(c, rep + 0x44, if seed % 2 == 0 { 4.0 } else { 6.0 });
                w(c, rep + 0xa4, seed.wrapping_mul(11));
                w(c, obj + 0xa8, (seed % 4) * 3);
                w(c, rep + 0xb0, rep + 0x200);
                w(c, obj, rep);
                w(c, obj + 0xb0, rep + 0x300);
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake;

    use crate::difftest::fill;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::oc318_gated;

    use std::sync::atomic::AtomicUsize;

    pub(crate) const CTRL_HIGHLIGHT: u32 = DATA + 0x6_0020;

    pub(crate) const MAGIC_HIGHLIGHT: u32 = 0x26e6_e001;

    // --------------------------------------- Replay_HighlightSlotBegin

    oc318_gated!(
        oc318_fake_highlight_set_value,
        OC318_PREV_HIGHLIGHT,
        CTRL_HIGHLIGHT,
        MAGIC_HIGHLIGHT,
        0x0026_d730,
        ee::s07::f_0026d730__generated
    );

    pub(crate) fn replay_highlight_install() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026D730),
            oc318_fake_highlight_set_value,
            &OC318_PREV_HIGHLIGHT,
        );
    }

    /// A slot object, a rider and a data block: aligned and unaligned
    /// blocks, so both copy loops run.
    pub(crate) fn replay_highlight_setup(c: &mut Cpu, seed: u32) {
        replay_highlight_install();
        let mut r = machine(c, seed, 0x0026_e6e0, 1);
        c.pc = 0x0026_e6e0;
        c.bus.write32(CTRL_HIGHLIGHT, MAGIC_HIGHLIGHT);
        // `self + 4` must be 8-aligned for the aligned path.
        let this = if seed % 2 == 0 {
            DATA + 0x404
        } else {
            DATA + 0x400
        };
        let rider = DATA + 0x800;
        let src = DATA + 0x1000 + 4 * (seed % 2);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = rider as u64;
        c.g[A2][0] = src as u64;
        c.bus.write32(this + REPLAY_HIGHLIGHT_INDEX, seed % 8);
        fill(c, &mut r, rider, 8); // 0x20 bytes
        fill(c, &mut r, this + REPLAY_HIGHLIGHT_DATA, 0xac / 4);
        fill(c, &mut r, src, 0xac / 4);
    }

    /// `Replay_HighlightSlotBegin`: both copy loops (aligned and not) and the
    /// snapshot call (a logging fake).
    #[test]
    fn replay_highlight_slot_begin_matches() {
        covered(
            ee::s07::f_0026e6e0__generated,
            replay_highlight_slot_begin,
            200,
            &replay_highlight_setup,
        );
    }

    /// The copy loops' back-edge event checks, with events delivered and
    /// rescheduled mid-copy.
    #[test]
    fn replay_highlight_slot_begin_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s07::f_0026e6e0__generated,
            replay_highlight_slot_begin,
            60,
            &|c, seed| {
                replay_highlight_setup(c, seed);
                let mut r = Rng(0x0026_e601 ^ seed as u64);
                schedule(c, &mut r, seed, 0x0026_e6e0)
            },
        );
    }

    /// `Replay_HighlightSlotDetach` on a slot with a random object.
    #[test]
    fn replay_highlight_slot_detach_matches() {
        check(
            ee::s07::f_0026e7f8__generated,
            replay_highlight_slot_detach,
            64,
            &|c, seed| {
                let mut r = Rng(0x0026_e7f8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0026_e7f8;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus
                    .write32(DATA + 0x100 + REPLAY_HIGHLIGHT_SLOT_OBJECT, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::replay::frameptr::tests::RDO_STREAM;

    pub(crate) use crate::game::replay::frameptr::tests::RDO_SCRIPT;

    pub(crate) use crate::game::replay::frameptr::tests::dead_stream_fixture;
    pub(crate) use crate::game::replay::frameptr::tests::dead_table_fixture;

    /// The read fake: serves the scripted word into the buffer (`a1`), then
    /// logs the call as the stock fake does.
    pub(crate) fn rdo_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let buf = c.g[A1][0] as u32;
        let word = c.bus.read32(RDO_SCRIPT + 4 * n);
        c.bus.write32(buf, word);
        if n == 0 {
            c.bus.write32(buf + 4, 0);
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn rdo_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_e340, 0);
        c.hooks.miss = rdo_miss;
        let count = seed % 5;
        c.bus.write32(RDO_SCRIPT, count);
        dead_stream_fixture(c, &mut r);
        dead_table_fixture(c, &mut r);
        for n in 1..=count {
            let rf = r.next_u32() & 0x3fff;
            c.bus.write32(RDO_SCRIPT + 4 * n, rf);
        }
        crate::game::ai::computer::tests::rbc_test_allocator(c);
        c.g[GP][0] = GAME_GP as u64;
        c.g[SP][0] = crate::difftest::SP as u64;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.g[A1][0] = RDO_STREAM as u64;
    }

    #[test]
    fn world_state_read_dead_objects_matches() {
        check(
            ee::s07::f_0026e340__generated,
            world_state_read_dead_objects,
            200,
            &rdo_setup,
        );
    }

    // A frame list of 1-5 frames (times -5, 5, 15 ..) and a target time, one
    // third of the seeds exactly on a frame.
    pub(crate) fn snapshot_list_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_e6a0, 0);
        let n = 1 + seed % 5;
        let list = DATA + 0x20_8000;
        let frame = |i: u32| DATA + 0x20_8100 + 0x40 * i;
        c.bus.write32(list, frame(0));
        for i in 0..n {
            let next = if i + 1 < n { frame(i + 1) } else { 0 };
            c.bus.write32(frame(i) + 0x14, next);
            c.bus.write32(frame(i) + 0x30, (i * 10).wrapping_sub(5));
        }
        let target = if seed % 3 == 0 {
            (seed % n).wrapping_mul(10).wrapping_sub(5)
        } else {
            (r.next_u32() % 60).wrapping_sub(10)
        };
        c.g[A0][0] = list as u64;
        c.g[A1][0] = sx(target);
    }

    #[test]
    fn snapshot_list_find_by_frame_matches() {
        check(
            ee::s07::f_0026e6a0__generated,
            snapshot_list_find_by_frame,
            200,
            &snapshot_list_setup,
        );
    }

    #[test]
    fn world_state_create_object_matches() {
        check(
            ee::s07::f_0026ded8__generated,
            world_state_create_object,
            200,
            &|c, seed| {
                machine(c, seed, 0xded8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0026ded8;
            },
        );
    }

    pub(crate) use crate::game::replay::playbackman::tests::oc683_in_child;
    pub(crate) use crate::game::replay::playbackman::tests::oc683_machine;

    pub(crate) use crate::game::replay::playbackman::tests::OC683_STREAM;

    pub(crate) use crate::game::replay::playbackman::tests::oc683_world;
    pub(crate) use crate::game::replay::playbackman::tests::OC683_STATE;

    /// `WorldState_Read(self, stream)`: one replay frame read back into the
    /// world, 30 callees, each a logged fake (unwinds at random calls).
    #[test]
    fn world_state_read_matches() {
        oc683_in_child("world_state_read_matches", &|| {
            check(
                ee::s07::f_0026dbf0__generated,
                world_state_read_2,
                120,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_dbf0, 34);
                    oc683_world(c);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.g[A1][0] = sx(OC683_STREAM);
                    c.pc = 0x0026_dbf0;
                },
            );
        });
    }
}
