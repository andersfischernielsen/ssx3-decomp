//! `src/replay/replay.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
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
use crate::guest::Addr;
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
    use std::sync::atomic::Ordering;
    use std::sync::Once;

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
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC318_PREV_HIGHLIGHT.store(ee::s07::OVERRIDE_0026D730 as usize, Ordering::Relaxed);
            ee::s07::OVERRIDE_0026D730 = oc318_fake_highlight_set_value;
        });
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
}
