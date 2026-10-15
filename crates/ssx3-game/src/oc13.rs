//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements for the buddy/replay/path data helpers. Each
//! copies the generated body's logic, cycle charging and observable register
//! (and FPU condition) results exactly, with a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_2938, buddy_man_get_message_count, "BuddyMan_GetMessageCount"),
    (0x0026_3308, buddy_man_get_challenger, "BuddyMan_GetChallenger"),
    (0x0026_3630, buddy_man_get_message, "BuddyMan_GetMessage"),
    (0x0026_6d00, c_voip_man_ctor, "cVoipMan_ctor"),
    (0x0026_a9b0, path_bounds_distance, "Path_BoundsDistance"),
    (0x0026_af70, path_data_get_track_path, "PathData_GetTrackPath"),
    (0x0026_af88, path_data_get_ai_path, "PathData_GetAIPath"),
    (0x0026_d168, replay_cache_reset, "ReplayCache_Reset"),
    (0x0026_d228, replay_cache_push_frame, "ReplayCache_PushFrame"),
    (0x0026_d730, replay_snapshot_set_highlight_value, "Replay_SnapshotSetHighlightValue"),
    (0x0026_e5a8, snapshot_list_ctor, "SnapshotList_ctor"),
    (0x0026_e5c0, snapshot_list_clear, "SnapshotList_Clear"),
    (0x0026_e608, snapshot_list_remove, "SnapshotList_Remove"),
    (0x0026_e800, replay_highlight_slot_clear, "Replay_HighlightSlotClear"),
    (0x0026_e8e0, replay_stream_ctor, "ReplayStream_ctor"),
    (0x0026_e950, replay_stream_set_buffer, "ReplayStream_SetBuffer"),
    (0x0026_e9b0, replay_stream_rewind_read, "ReplayStream_RewindRead"),
    (0x0027_0658, replay_request_instant_replay, "Replay_RequestInstantReplay"),
];

/// `int BuddyMan_MapPresenceState(BuddyMan *man, int slot)` (0x2628c0): maps a
/// slot number to a presence state (0..3) when the list at `man + 0xc8` is
/// present, `-1` for an absent list or a slot outside 0..3.
pub fn buddy_man_map_presence_state(c: &mut Cpu) {
    const PRESENCE: u32 = 0xc8;
    const NONE: u64 = u64::MAX; // -1

    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let slot = c.g[A1][0];
    let present = man.off(PRESENCE).lw(c) != 0;
    c.g[V0][0] = 1;
    if !present {
        c.cycles += 2;
        c.g[V0][0] = NONE;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if slot == 1 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_two = (slot as i64) < 2;
    c.g[V0][0] = 2;
    if below_two {
        c.cycles += 2;
        if slot == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = NONE;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 3;
    if slot == 2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = NONE;
    if slot == 3 {
        c.cycles += 2;
        c.g[V0][0] = 3;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int BuddyMan_GetMessageCount(BuddyMan *man)` (0x262938): number of
/// messages, `(end - begin) / 4` with the two pointers at +0xac / +0xb0.
pub fn buddy_man_get_message_count(c: &mut Cpu) {
    const MESSAGES_BEGIN: u32 = 0xac;
    const MESSAGES_END: u32 = 0xb0;

    c.cycles += 5;
    let man = Addr(abi::arg(c, 0));
    let begin = man.off(MESSAGES_BEGIN).lw(c);
    c.g[V1][0] = begin;
    let end = man.off(MESSAGES_END).lw(c);
    c.g[V0][0] = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    abi::ret(c);
}

/// `BuddyMan_Message *BuddyMan_GetChallenger(BuddyMan *man, int index)`
/// (0x263308): `challengers[index]`, records being 4 bytes after the begin
/// pointer at +0xbc (end at +0xc0), or NULL when out of range.
pub fn buddy_man_get_challenger(c: &mut Cpu) {
    const CHALLENGERS_BEGIN: u32 = 0xbc;
    const CHALLENGERS_END: u32 = 0xc0;

    c.cycles += 7;
    let man = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    let begin = man.off(CHALLENGERS_BEGIN).lw(c);
    c.g[A2][0] = begin;
    let end = man.off(CHALLENGERS_END).lw(c);
    let count = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    c.g[V0][0] = count;
    c.g[V1][0] = sx((index as u32) << 2);
    if index < count {
        c.cycles += 3;
        let slot = sx((begin as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = slot;
        c.g[V0][0] = Addr(slot as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `BuddyMan_Message *BuddyMan_GetMessage(BuddyMan *man, int index)`
/// (0x263630): `messages[index]`, records being 4 bytes after the begin
/// pointer at +0xac (end at +0xb0), or NULL when out of range.
pub fn buddy_man_get_message(c: &mut Cpu) {
    const MESSAGES_BEGIN: u32 = 0xac;
    const MESSAGES_END: u32 = 0xb0;

    c.cycles += 7;
    let man = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    let begin = man.off(MESSAGES_BEGIN).lw(c);
    c.g[A2][0] = begin;
    let end = man.off(MESSAGES_END).lw(c);
    let count = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    c.g[V0][0] = count;
    c.g[V1][0] = sx((index as u32) << 2);
    if index < count {
        c.cycles += 3;
        let slot = sx((begin as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = slot;
        c.g[V0][0] = Addr(slot as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `cVoipMan *cVoipMan_ctor(cVoipMan *this)` (0x266d00): zero the two words of
/// a fresh voip manager and return it unchanged in v0.
pub fn c_voip_man_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `float Path_BoundsDistance(const PathBounds *box, const Vec3 *p)`
/// (0x26a9b0): signed distance from `p` to the AABB whose X/Y/Z minima are at
/// box+0x1c/0x20/0x24 and maxima at box+0x28/0x2c/0x30. Returns the largest
/// per-axis penetration in f0 (with the scratch FPU registers the original
/// leaves behind).
pub fn path_bounds_distance(c: &mut Cpu) {
    const BOX_MIN_X: u32 = 0x1c;
    const BOX_MIN_Y: u32 = 0x20;
    const BOX_MIN_Z: u32 = 0x24;
    const BOX_MAX_X: u32 = 0x28;
    const BOX_MAX_Y: u32 = 0x2c;
    const BOX_MAX_Z: u32 = 0x30;

    let box_addr = Addr(abi::arg(c, 0)); // a0
    let p = Addr(abi::arg(c, 1)); // a1

    // X axis.
    c.cycles += 7;
    let mut f1 = p.read_fpu(c);
    let mut f2 = box_addr.off(BOX_MIN_X).read_fpu(c);
    let mut f3 = FpuF32(0); // mtc1 r0 -> f3
    let mut f4 = f3; // mov_s 4, 3
    let mut f0 = f3; // mov_s 0, 3
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f3 = f2 - f1;
    } else {
        c.cycles += 5;
        f2 = box_addr.off(BOX_MAX_X).read_fpu(c);
        if abi::c_lt_s(c, f2, f1) {
            f3 = f1 - f2;
        }
    }

    // Y axis.
    c.cycles += 6;
    f1 = p.off(0x4).read_fpu(c);
    f2 = box_addr.off(BOX_MIN_Y).read_fpu(c);
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f4 = f2 - f1;
    } else {
        f2 = box_addr.off(BOX_MAX_Y).read_fpu(c);
        c.cycles += 4;
        if abi::c_lt_s(c, f2, f1) {
            f4 = f1 - f2;
        }
    }

    // Z axis.
    c.cycles += 6;
    f1 = p.off(0x8).read_fpu(c);
    f2 = box_addr.off(BOX_MIN_Z).read_fpu(c);
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f0 = f2 - f1;
    } else {
        f2 = box_addr.off(BOX_MAX_Z).read_fpu(c);
        c.cycles += 4;
        if abi::c_lt_s(c, f2, f1) {
            f0 = f1 - f2;
        }
    }

    // Combine the three penetration distances into f0.
    c.cycles += 3;
    if abi::c_lt_s(c, f4, f3) {
        c.cycles += 4;
        if !abi::c_lt_s(c, f3, f0) {
            f0 = f3;
        }
    } else {
        c.cycles += 3;
        if !abi::c_lt_s(c, f4, f0) {
            c.cycles += 2;
            f0 = f4;
            abi::set_fpr(c, 0, f0);
            abi::set_fpr(c, 1, f1);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::set_fpr(c, 0, f0);
    abi::set_fpr(c, 1, f1);
    abi::ret(c);
}

/// `PathTrack *PathData_GetTrackPath(PathData *self, int index)` (0x26af70):
/// `self->track_paths[index]`, records of 60 bytes after the pointer at +0x14.
pub fn path_data_get_track_path(c: &mut Cpu) {
    const TRACK_PATHS: u32 = 0x14;
    const TRACK_RECORD_SIZE: u32 = 0x3c;

    c.cycles += 5;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V1][0] = TRACK_RECORD_SIZE as u64;
    let base = self_.off(TRACK_PATHS).lw(c);
    let scaled = abi::mult(c, abi::arg(c, 1), TRACK_RECORD_SIZE);
    c.g[A1][0] = sx(scaled);
    c.g[V0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `PathAI *PathData_GetAIPath(PathData *self, int index)` (0x26af88):
/// `self->ai_paths[index]`, records of 64 bytes after the pointer at +0xc.
pub fn path_data_get_ai_path(c: &mut Cpu) {
    const AI_PATHS: u32 = 0xc;

    c.cycles += 4;
    let self_ = Addr(abi::arg(c, 0));
    let base = self_.off(AI_PATHS).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 6);
    c.g[V0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `void ReplayCache_Reset(ReplayCache *cache)` (0x26d168): zero the count and
/// the two list pointers.
pub fn replay_cache_reset(c: &mut Cpu) {
    c.cycles += 4;
    let cache = Addr(abi::arg(c, 0));
    cache.off(0x8).sw(c, 0);
    cache.sw(c, 0);
    cache.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void ReplayCache_PushFrame(ReplayCache *cache, const void *frame)`
/// (0x26d228): append an 8-byte frame to the ring buffer (base at +0xc, count
/// at +0), tagging the header's low 12 bits, or flag the world when full.
pub fn replay_cache_push_frame(c: &mut Cpu) {
    const FRAME_BUFFER: u32 = 0xc;
    const FRAME_SIZE: u32 = 8;
    const HEADER_MASK: u64 = 0xffff_ffff_ffff_f000;
    const MAX_FRAMES: i64 = 4096;
    const WORLD_VIEWS: u32 = 0x84;
    const VIEW_FLAG: u32 = 0x610;
    const VIEW_FLAG_VALUE: u32 = 0xb4;

    c.cycles += 5;
    let cache = Addr(abi::arg(c, 0));
    let frame = abi::arg(c, 1);
    c.g[A2][0] = c.g[A0][0];
    let count = cache.lw(c);
    c.g[V0][0] = ((count as i64) < MAX_FRAMES) as u64;
    let room = c.g[V0][0] != 0;
    c.g[A3][0] = c.g[A1][0];
    if room {
        c.cycles += 20;
        let base = cache.off(FRAME_BUFFER).lw(c);
        c.g[V1][0] = base;
        let slot = sx(((count as u32) << 3).wrapping_add(base as u32));
        c.g[V0][0] = slot;
        c.g[A1][0] = HEADER_MASK;
        let data = Addr(frame).read_u64_unaligned(c);
        Addr(slot as u32).write_u64_unaligned(c, data);
        let count2 = cache.lw(c);
        let base2 = cache.off(FRAME_BUFFER).lw(c);
        let slot2 = sx(((count2 as u32) << 3).wrapping_add(base2 as u32));
        c.g[A0][0] = slot2;
        let header = Addr(slot2 as u32).lw(c);
        let tagged = (header & c.g[A1][0]) | 1;
        c.g[V0][0] = tagged;
        Addr(slot2 as u32).sw(c, tagged);
        c.g[V1][0] = sx((cache.lw(c) as u32).wrapping_add(1));
        cache.sw(c, c.g[V1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = world;
    c.g[A2][0] = 1;
    c.g[A1][0] = VIEW_FLAG_VALUE as u64;
    let views = Addr(world as u32).off(WORLD_VIEWS).lw(c);
    c.g[V0][0] = views;
    let view = Addr(views as u32).off(0x28).lw(c);
    c.g[V1][0] = view;
    Addr(view as u32).off(VIEW_FLAG).off(0x4).sw(c, c.g[A1][0]);
    Addr(view as u32).off(VIEW_FLAG).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `void WorldStateStream_Reset(WorldStateStream *stream)` (0x26d628):
/// reinitialise the stream fields (a scratch 32-bit zero on the stack is used
/// to store the first list word unaligned).
pub fn world_state_stream_reset(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    let sp = Addr((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[SP][0] = sx(sp.0);
    c.g[V0][0] = u64::MAX; // -1
    sp.sh(c, 0);
    sp.off(2).sb(c, 0);
    sp.off(3).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, c.g[V0][0]);
    c.lwl(AT, sp.off(3).0);
    c.lwr(AT, sp.0);
    c.swl(AT, stream.off(0x1f).0);
    c.swr(AT, stream.off(0x1c).0);
    stream.off(0x2c).sw(c, c.g[V0][0]);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, c.g[V0][0]);
    stream.off(0x24).sw(c, c.g[V0][0]);
    stream.off(0x28).sw(c, c.g[V0][0]);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx(sp.0.wrapping_add(0x10));
    abi::ret(c);
}

/// `void Replay_SnapshotSetHighlightValue(Replay *replay, int slot, uint value)`
/// (0x26d730): `replay->snapshots[slot].highlight = value`.
pub fn replay_snapshot_set_highlight_value(c: &mut Cpu) {
    const HIGHLIGHT: u32 = 0x20;

    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(HIGHLIGHT).sw(c, c.g[A2][0]);
    abi::ret(c);
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

/// `void ReplayStream_SetBuffer(ReplayStream *stream, void *buf, uint len)`
/// (0x26e950): set begin/end/cursor and the length.
pub fn replay_stream_set_buffer(c: &mut Cpu) {
    c.cycles += 6;
    let stream = Addr(abi::arg(c, 0));
    let buf = abi::arg(c, 1);
    let len = abi::arg(c, 2);
    c.g[V0][0] = sx(buf.wrapping_add(len));
    stream.off(0x10).sw(c, c.g[A2][0]);
    stream.off(0x8).sw(c, c.g[V0][0]);
    stream.off(0xc).sw(c, c.g[A1][0]);
    stream.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void ReplayStream_RewindRead(ReplayStream *stream)` (0x26e9b0): reset the
/// read cursor to the buffer begin.
pub fn replay_stream_rewind_read(c: &mut Cpu) {
    c.cycles += 3;
    let stream = Addr(abi::arg(c, 0));
    let begin = stream.off(0x4).lw(c);
    c.g[V0][0] = begin;
    stream.off(0xc).sw(c, begin);
    abi::ret(c);
}

/// `void Replay_RequestInstantReplay(Replay *replay)` (0x270658): request an
/// instant replay only when one is not already active.
pub fn replay_request_instant_replay(c: &mut Cpu) {
    const ACTIVE: u32 = 0x620;
    const REQUESTED: u32 = 0x624;

    c.cycles += 3;
    let replay = Addr(abi::arg(c, 0));
    let active = replay.off(ACTIVE).lw(c);
    c.g[V0][0] = 1;
    if active == 0 {
        c.cycles += 1;
        replay.off(REQUESTED).sw(c, 1);
    }
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn buddy_man_map_presence_state_matches() {
        check(ssx3_ee::s07::f_002628c0, buddy_man_map_presence_state, 300, &|c, seed| {
            let mut r = Rng(0x0c13_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => u64::MAX, // negative
                _ => 5,        // out of range, high
            };
            // Cover present (non-zero) and absent.
            c.bus.write32(DATA + 0xc8, if seed % 7 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }

    #[test]
    fn buddy_man_get_message_count_matches() {
        check(ssx3_ee::s07::f_00262938, buddy_man_get_message_count, 200, &|c, seed| {
            let mut r = Rng(0x0c13_0002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let begin = DATA + 0x200;
            let end = match seed % 4 {
                0 => begin.wrapping_sub(16), // negative count
                1 => begin,                 // zero
                _ => begin + 16,
            };
            c.bus.write32(DATA + 0xac, begin);
            c.bus.write32(DATA + 0xb0, end);
        });
    }

    #[test]
    fn buddy_man_get_challenger_matches() {
        check(ssx3_ee::s07::f_00263308, buddy_man_get_challenger, 300, &|c, seed| {
            let mut r = Rng(0x0c13_0003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let begin = DATA + 0x200;
            let end = if seed % 5 == 0 { begin.wrapping_sub(8) } else { begin + 16 };
            c.bus.write32(DATA + 0xbc, begin);
            c.bus.write32(DATA + 0xc0, end);
            c.g[A1][0] = match seed % 8 {
                0..=3 => (seed % 4) as u64, // in range when count > 0
                4 => 4,                     // first out-of-range slot
                5 => 0xffff_ffff,           // wraps the base by -4
                6 => 0,
                _ => u64::MAX,
            };
            for i in 0..8 {
                c.bus.write32(begin.wrapping_sub(8) + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn buddy_man_get_message_matches() {
        check(ssx3_ee::s07::f_00263630, buddy_man_get_message, 300, &|c, seed| {
            let mut r = Rng(0x0c13_0004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let begin = DATA + 0x200;
            let end = if seed % 5 == 0 { begin.wrapping_sub(8) } else { begin + 16 };
            c.bus.write32(DATA + 0xac, begin);
            c.bus.write32(DATA + 0xb0, end);
            c.g[A1][0] = match seed % 8 {
                0..=3 => (seed % 4) as u64,
                4 => 4,
                5 => 0xffff_ffff,
                6 => 0,
                _ => u64::MAX,
            };
            for i in 0..8 {
                c.bus.write32(begin.wrapping_sub(8) + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn c_voip_man_ctor_matches() {
        check(ssx3_ee::s07::f_00266d00, c_voip_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn path_bounds_distance_matches() {
        check(ssx3_ee::s07::f_0026a9b0, path_bounds_distance, 300, &|c, seed| {
            let mut r = Rng(0x0c13_0006 ^ seed as u64);
            scramble(c, &mut r);
            let point = DATA;
            let bounds = DATA + 0x100;
            c.g[A0][0] = bounds as u64; // box
            c.g[A1][0] = point as u64; // point
            if seed % 3 == 0 {
                // Degenerate box at 0 so the per-axis distance equals |d|;
                // the table picks each of the four combine branches.
                let d: [f32; 3] = match (seed / 3) % 8 {
                    0 => [2.0, 1.0, 3.0],
                    1 => [2.0, 1.0, 1.0],
                    2 => [1.0, 2.0, 5.0],
                    3 => [1.0, 3.0, 2.0],
                    4 => [-2.0, -1.0, -3.0],
                    5 => [-2.0, -1.0, -1.0],
                    6 => [1.0, -2.0, 5.0],
                    _ => [0.0, 3.0, 0.0],
                };
                for i in 0..3 {
                    c.bus.write32(point + i * 4, d[i as usize].to_bits());
                }
                for off in [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30] {
                    c.bus.write32(bounds + off, 0);
                }
            } else {
                for off in [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30] {
                    c.bus.write32(bounds + off, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(point + i * 4, r.edge_f32());
                }
            }
        });
    }

    #[test]
    fn path_data_get_track_path_matches() {
        check_lohi(ssx3_ee::s07::f_0026af70, path_data_get_track_path, 200, &|c, seed| {
            let mut r = Rng(0x0c13_0007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x14, r.next_u32());
        });
    }

    #[test]
    fn path_data_get_ai_path_matches() {
        check(ssx3_ee::s07::f_0026af88, path_data_get_ai_path, 200, &|c, seed| {
            let mut r = Rng(0x0c13_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }

    #[test]
    fn replay_cache_reset_matches() {
        check(ssx3_ee::s07::f_0026d168, replay_cache_reset, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn replay_cache_push_frame_matches() {
        check(ssx3_ee::s07::f_0026d228, replay_cache_push_frame, 300, &|c, seed| {
            let mut r = Rng(0x0c13_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            if seed % 2 == 0 {
                // Room left in the ring buffer.
                let count = r.next_u32() % 8;
                let base = DATA + 0x200 + (seed % 3);
                c.bus.write32(DATA, count);
                c.bus.write32(DATA + 0xc, base);
                for i in 0..2 {
                    c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                }
                for i in 0..2 {
                    c.bus.write32(base + count * 8 + i * 4, r.next_u32());
                }
            } else {
                // Full: falls back to flagging the world view.
                c.bus.write32(DATA, 4096 + (r.next_u32() % 8));
                let world = DATA + 0x400;
                let views = DATA + 0x500;
                let view = DATA + 0x600;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, views);
                c.bus.write32(views + 0x28, view);
                c.bus.write32(view + 0x610, r.next_u32());
                c.bus.write32(view + 0x614, r.next_u32());
            }
        });
    }

    #[test]
    fn world_state_stream_reset_matches() {
        check(ssx3_ee::s07::f_0026d628, world_state_stream_reset, 100, &|c, seed| {
            let mut r = Rng(0x0c13_000b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn replay_snapshot_set_highlight_value_matches() {
        check(ssx3_ee::s07::f_0026d730, replay_snapshot_set_highlight_value, 200, &|c, seed| {
            let mut r = Rng(0x0c13_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 16) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for i in 0..0x30 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn snapshot_list_ctor_matches() {
        check(ssx3_ee::s07::f_0026e5a8, snapshot_list_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c13_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn snapshot_list_clear_matches() {
        check(ssx3_ee::s07::f_0026e5c0, snapshot_list_clear, 50, &|c, seed| {
            let mut r = Rng(0x0c13_000e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn snapshot_list_remove_matches() {
        check(ssx3_ee::s07::f_0026e608, snapshot_list_remove, 300, &|c, seed| {
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
                0 => a, // head (prev == 0)
                1 => cc, // tail (next == 0)
                2 => b, // middle (both non-zero)
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
        });
    }

    #[test]
    fn replay_highlight_slot_clear_matches() {
        check(ssx3_ee::s07::f_0026e800, replay_highlight_slot_clear, 200, &|c, seed| {
            let mut r = Rng(0x0c13_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let slot = DATA + 0x100;
            // Cover both the empty-slot and clear-bit paths.
            c.bus.write32(DATA + 0xb0, if seed % 2 == 0 { 0 } else { slot });
            c.bus.write32(
                DATA,
                match seed % 3 {
                    0 => (seed % 31) as u32,
                    1 => 31,
                    _ => r.next_u32(),
                },
            );
            c.bus.write8(slot + 0x1e, r.next_u32() as u8);
        });
    }

    #[test]
    fn replay_stream_ctor_matches() {
        check(ssx3_ee::s07::f_0026e8e0, replay_stream_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..5 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn replay_stream_set_buffer_matches() {
        check(ssx3_ee::s07::f_0026e950, replay_stream_set_buffer, 100, &|c, seed| {
            let mut r = Rng(0x0c13_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for i in 0..5 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn replay_stream_rewind_read_matches() {
        check(ssx3_ee::s07::f_0026e9b0, replay_stream_rewind_read, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, r.next_u32());
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }

    #[test]
    fn replay_request_instant_replay_matches() {
        check(ssx3_ee::s07::f_00270658, replay_request_instant_replay, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x620, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 0x624, r.next_u32());
        });
    }
}
