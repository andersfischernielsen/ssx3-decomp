//! Staging module oc532 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc532`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
    (
        0x0026_ec08,
        replay_stream_read_node_ref,
        "ReplayStream_ReadNodeRef",
    ),
    (
        0x0026_ed90,
        replay_snapshot_empty_hook_b,
        "Replay_SnapshotEmptyHookB",
    ),
    (
        0x0026_f898,
        replay_set_controller_index,
        "Replay_SetControllerIndex",
    ),
    (0x0026_f088, replay_dtor, "Replay_dtor"),
    (0x0027_0ed8, replay_end_save, "Replay_EndSave"),
    (0x0027_0ef8, replay_reset_on_load, "Replay_ResetOnLoad"),
    (0x0027_0f18, replay_empty_load_hook, "Replay_EmptyLoadHook"),
];

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

/// `ReplayStream_RewindRead` (0x26e9b0): `self->+0xc = self->+0x4`.
const OC532_REWIND_READ: Callee = Callee {
    addr: 0x0026_e9b0,
    f: ee::s07::f_0026e9b0,
};

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

/// `cReplayFramePtr_loadBlock` (0x26eab8), listed as `ReplayStream_Decompress`:
/// reads a block tag from `src` and its size from `*pp`, advances `*pp` past
/// the 4-byte size, Huffman-decodes `size` bytes into the stream's block
/// (`self+4`), moves `*pp` past them, sets `self+8` (WriteData) to the end of
/// the decoded bytes and rewinds the stream. Returns 1.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        // MEM_copy(&hdr.tag, src, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        let pp = c.g[S0][0] as u32;
        c.g[A1][0] = c.lw(pp);
        c.g[A0][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 4;
        // MEM_copy(&hdr.size, *pp, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = c.lw(pp);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        let advanced = c.g[A0][0];
        c.sw(pp, advanced);
        // HUFF_decode(*pp, 0, self->Block)
        let self_ptr = c.g[S1][0] as u32;
        c.g[A2][0] = c.lw(self_ptr.wrapping_add(0x4));
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        let size_slot = sp(c).wrapping_add(0x4);
        c.g[V1][0] = c.lw(size_slot);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.lw(pp);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let moved = c.g[A1][0];
        c.sw(pp, moved);
        let self_ptr = c.g[S1][0] as u32;
        c.g[V1][0] = c.lw(self_ptr.wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let write_data = c.g[V1][0];
        c.sw(self_ptr.wrapping_add(0x8), write_data);
        Callee::call(OC532_REWIND_READ, c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `sNode_EC08* ReplayStream_ReadNodeRef(stream, owner)` (0x26ec08): reads an
/// 8-byte `{id, type}` record through the stream's virtual `read`. An id of
/// -1 (`~id == 0`) gives no node and the result is 0. Otherwise the id is
/// resolved through the ref table behind `gp+0x16c8` (the same lookup as
/// `world_state_read_dead_objects`): the node's owner (+0xc) becomes `owner`
/// and its flags (+0x8) keep bits 0xFFFF0300 and take `type & 0xFCFF`.
pub fn replay_stream_read_node_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A2][0] = 8;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = 0;
        let stream = c.g[A0][0] as u32;
        c.g[A3][0] = c.lw(stream);
        let rec = sp(c);
        c.sw(rec, u64::MAX);
        // stream->read(&rec, 8)
        let vt = c.g[A3][0] as u32;
        c.g[V1][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let read = c.g[V0][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, read, 0x0026_ec48)?;
        let rec = sp(c);
        let id = c.lw(rec);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let none = c.g[V0][0] == 0;
        let ctx_slot = (c.g[GP][0] as u32).wrapping_add(0x16c8);
        c.g[A1][0] = c.lw(ctx_slot);
        if !none {
            c.cycles += 8;
            c.g[V0][0] = sx(((c.g[A2][0] & 0xff) as u32) << 2);
            let ctx = c.g[A1][0] as u32;
            c.g[V1][0] = c.lw(ctx);
            let table = c.g[V1][0] as u32;
            c.g[A0][0] = c.lw(table.wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let entry_slot = c.g[V0][0] as u32;
            c.g[V1][0] = c.lw(entry_slot);
            let entry_null = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            // Node pointer in v0 (0 when the table holds none).
            let mut has_node = false;
            if !entry_null {
                c.cycles += 7;
                let entry = c.g[V1][0] as u32;
                c.g[A0][0] = c.lw(entry.wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let node_slot = c.g[V0][0] as u32;
                c.g[V1][0] = c.lw(node_slot);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                has_node = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !has_node {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 10;
            c.g[S0][0] = c.g[V0][0];
            let node = c.g[S0][0] as u32;
            let owner = c.g[S1][0];
            c.sw(node.wrapping_add(0xc), owner);
            c.g[A0][0] = 0xffff_ffff_ffff_0300;
            c.g[V1][0] = c.lw(node.wrapping_add(0x8));
            let type_slot = sp(c).wrapping_add(0x4);
            c.g[V0][0] = c.lhu(type_slot);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V0][0] &= 0xfcff;
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            c.sw(node.wrapping_add(0x8), flags);
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

/// `cReplay_restoreReplayFrame` (0x26db88), a direct call (no override slot).
const OC532_RESTORE_REPLAY_FRAME: Callee = Callee {
    addr: 0x0026_db88,
    f: ee::s07::f_0026db88,
};

/// `cReplay_freeReplayFrame` (0x270238).
const OC532_FREE_REPLAY_FRAME: Callee = Callee {
    addr: 0x0027_0238,
    f: ee::s07::f_00270238,
};

/// `Replay_SnapshotEmptyHookB` (0x26ed90): an empty hook.
pub fn replay_snapshot_empty_hook_b(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Replay_SetControllerIndex` (0x26f898): `self->controllerIndex (+0x28) = a1`.
pub fn replay_set_controller_index(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    let index = c.g[A1][0];
    c.sw(this.wrapping_add(0x28), index);
    ret(c);
}

/// `cReplay_popReplaySaveFrame` (0x26f850), listed as
/// `Replay_RestoreResumeSnapshot`: when the save frame (`self+0x3d0`) is set,
/// restores it, frees it and clears the slot.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let save_frame = c.lw(this.wrapping_add(0x3d0));
        c.g[A1][0] = save_frame;
        if save_frame != 0 {
            c.cycles += 7;
            OC532_RESTORE_REPLAY_FRAME.call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(this.wrapping_add(0x3d0));
            c.g[A0][0] = c.g[S0][0];
            OC532_FREE_REPLAY_FRAME.call(c, 0x0026_f880)?;
            c.sw(this.wrapping_add(0x3d0), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_EndPlayback` (0x26f980): the replay's stop call.
const OC532_END_PLAYBACK: Callee = Callee {
    addr: 0x0026_f980,
    f: ee::s07::f_0026f980,
};

/// `Replay_EmptyResetHook` (0x26f498).
const OC532_EMPTY_RESET_HOOK: Callee = Callee {
    addr: 0x0026_f498,
    f: ee::s07::f_0026f498,
};

/// `Replay_FreeBuffers` (0x26f428), the `cReplay_purge` of the reference.
const OC532_FREE_BUFFERS: Callee = Callee {
    addr: 0x0026_f428,
    f: ee::s07::f_0026f428,
};

/// `cReplay__cReplay` (0x26f088), listed as `Replay_dtor`: stops a running
/// replay (`*self` in 1..9), syncs and purges it, destroys the highlight
/// array at `+0x494` (stride 0xb4) and the frame array at `+0x2c` (stride
/// 0x38, the virtual destructor in slot 0x38/0x3c of each frame's vtable),
/// then frees `self` when `flags & 1` is set.
pub fn replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = c.lw(this);
        let state_zero = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !state_zero {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        // Stop the replay if it was running.
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            OC532_END_PLAYBACK.call(c, 0x0026_f0c8)?;
        }
        c.g[A0][0] = c.g[S1][0];
        // Sync and purge, then the EmptyResetHook/FreeBuffers pair.
        c.cycles += 8;
        c.g[S3][0] = c.g[S0][0] & 0x1;
        OC532_EMPTY_RESET_HOOK.call(c, 0x0026_f0d4)?;
        c.g[S2][0] = sx(this.wrapping_add(0x2c));
        c.g[A0][0] = c.g[S1][0];
        OC532_FREE_BUFFERS.call(c, 0x0026_f0e0)?;
        // Highlights at +0x494: destroy from the end, stride 0xb4.
        c.g[V1][0] = sx(this.wrapping_add(0x494));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(this.wrapping_add(0x5fc));
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                loop {
                    c.cycles += 7;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_f100);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
        // Frames at +0x2c: destroy from the end, stride 0x38.
        c.cycles += 2;
        let frames_zero = c.g[S2][0] == 0;
        c.g[S0][0] = sx(this.wrapping_add(0x3ac));
        if !frames_zero {
            c.cycles += 2;
            let last = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
            if !last {
                c.cycles += 1;
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    let slot = c.g[S0][0] as u32;
                    c.g[V0][0] = c.lw(slot);
                    let obj = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lh(obj.wrapping_add(0x38));
                    c.g[V1][0] = c.lw(obj.wrapping_add(0x3c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(slot.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0026_f148)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f130);
                    }
                }
            }
        }
        // The self-delete when flags & 1.
        c.cycles += 2;
        let no_delete = c.g[S3][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_f160)?;
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

/// `Replay_BeginPlayback` (0x26f8a0): starts playing back the frame `a1`
/// (an instant replay when `a2` is set, an auto replay when `a3` is). Unless
/// the replay is stopping (state 14 or 15), it pushes the resume snapshot
/// when none is loaded, sets state 7, clears the command and camera fields,
/// seeks to the frame, steps one frame and sets up the instant or the normal
/// replay cameras. A normal, non-auto replay also starts the audio system.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        let state = c.lw(this);
        c.g[V0][0] = sx((state as u32).wrapping_sub(0xe));
        c.g[V0][0] = ((c.g[V0][0] < 2) as u64) ^ 1;
        let ok = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        // Which epilogue tail the path ends in: x1 (lq s0) and x2 (lq s1).
        let mut run_x1 = true;
        let mut run_x2 = true;
        if ok {
            c.cycles += 3;
            let loaded = c.lw(this.wrapping_add(0x608));
            c.g[V0][0] = loaded;
            if loaded == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?;
            }
            c.g[A0][0] = c.g[S0][0];
            // Normal and instant replays alike: state 7, then seek and step.
            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?;
            c.sw(this.wrapping_add(0x618), 0);
            c.g[A1][0] = c.g[S2][0];
            c.sw(this.wrapping_add(0x61c), 0);
            c.g[A0][0] = c.g[S0][0];
            let instant = c.g[S1][0];
            c.sw(this.wrapping_add(0x620), instant);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?;
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?;
            }
            // Instant: x2 only. Auto: none. Otherwise the audio start, then x1 and x2.
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                run_x1 = false;
            } else {
                c.cycles += 2;
                if c.g[S3][0] != 0 {
                    run_x1 = false;
                    run_x2 = false;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?;
                }
            }
        }
        if run_x1 {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        if run_x2 {
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_UpdateCameraInput` (0x26fa78): per frame of a replay in state 3,
/// the camera input. When the input of the replay's player (`self+0x28`)
/// reports button 108, the game view manager updates. In camera subset 11
/// (`self+0x630`), when any of the four stick inputs (49, 48, 50, 51) is
/// non-zero, it updates the view manager too.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_update_camera_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        let this = c.g[S0][0] as u32;
        let state = c.lw(this);
        c.g[V0][0] = state ^ 3;
        if c.g[V0][0] == 0 {
            // 'b_26fad4: the input of the replay's player, button 108.
            c.cycles += 10;
            c.g[V1][0] = c.lw(this.wrapping_add(0x28));
            c.g[A1][0] = 0x6c;
            let app = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(app);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let input_slot = (c.g[V0][0] as u32).wrapping_add(0xb0);
            c.g[S1][0] = c.lw(input_slot);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0032_0c48, 0x0026_fac0)?;
            let no_state = c.g[V0][0] == 0;
            let app = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(app);
            if !no_state {
                c.cycles += 3;
                let view_slot = (c.g[V0][0] as u32).wrapping_add(0x84);
                c.g[V1][0] = c.lw(view_slot);
                let view = (c.g[V1][0] as u32).wrapping_add(0x84);
                c.g[A0][0] = c.lw(view);
                call_ptr(c, 0x0022_e840, 0x0026_fad4)?;
            }
            // Camera subset 11 of the replay's `self+0x630`.
            c.cycles += 9;
            c.g[V0][0] = c.lw(this.wrapping_add(0x630));
            c.g[V1][0] = 0x44_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5438));
            c.g[A1][0] = 0xb;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let subset = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = subset;
            if subset == c.g[A1][0] {
                // 'b_26fb60: the four stick inputs. Any non-zero one updates.
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                c.f[20] = 0;
                let codes = [0x31u64, 0x30, 0x32, 0x33];
                let rets = [0x0026_fb08u32, 0x0026_fb20, 0x0026_fb38, 0x0026_fb50];
                let mut update = false;
                for (i, (&code, &ret)) in codes.iter().zip(rets.iter()).enumerate() {
                    if i > 0 {
                        c.cycles += 6;
                    }
                    c.g[A1][0] = code;
                    call_ptr(c, 0x0032_0bf0, ret)?;
                    let eq = crate::ps2f::FpuF32(c.f[0]).eq_s(crate::ps2f::FpuF32(c.f[20]));
                    crate::abi::set_fcc(c, eq);
                    c.g[A0][0] = c.g[S1][0];
                    if i < 3 {
                        if !eq {
                            update = true;
                            break;
                        }
                    } else {
                        update = !eq;
                    }
                }
                if update {
                    // The view manager's update (the tail of the branch).
                    c.cycles += 5;
                    let app = (c.g[GP][0] as u32).wrapping_sub(0x848);
                    c.g[V0][0] = c.lw(app);
                    let view_slot = (c.g[V0][0] as u32).wrapping_add(0x84);
                    c.g[V1][0] = c.lw(view_slot);
                    let view = (c.g[V1][0] as u32).wrapping_add(0x84);
                    c.g[A0][0] = c.lw(view);
                    call_ptr(c, 0x0022_e840, 0x0026_fb70)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_EndSave` (0x270ed8): frees the save buffers.
pub fn replay_end_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0027_0f78, 0x0027_0ee8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_ResetOnLoad` (0x270ef8): resets the replay state on a load.
pub fn replay_reset_on_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_f228, 0x0027_0f08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_EmptyLoadHook` (0x270f18): an empty hook.
pub fn replay_empty_load_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};

    // Fixture of `world_state_read_dead_objects`: the stream object and its
    // vtable (`read` at slot 0x14, the `this` delta at 0x10), the table chain
    // behind `gp+0x16c8`, and the script the read fake serves (fake call n
    // returns script word n: the header count, then the refs).
    const RDO_STREAM: u32 = DATA + 0x20_0000;
    const RDO_VT: u32 = DATA + 0x20_0100;
    const RDO_CTX: u32 = DATA + 0x20_0200;
    const RDO_TA: u32 = DATA + 0x20_0300;
    const RDO_TB: u32 = DATA + 0x20_0400;
    const RDO_TC: u32 = DATA + 0x20_0800;
    const RDO_ENTRY: u32 = DATA + 0x20_0c00;
    const RDO_SCRIPT: u32 = DATA + 0x20_1000;
    const RDO_NODES: u32 = DATA + 0x20_3000;

    /// The read fake: serves the scripted word into the buffer (`a1`), then
    /// logs the call as the stock fake does.
    fn rdo_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let buf = c.g[A1][0] as u32;
        let word = c.bus.read32(RDO_SCRIPT + 4 * n);
        c.bus.write32(buf, word);
        if n == 0 {
            c.bus.write32(buf + 4, 0);
        }
        fakes::fake_call(c, target);
    }

    /// The stream object and its vtable: `read` at slot 0x14, the `this`
    /// delta at 0x10 (the read fakes write the buffer, not `this`).
    fn dead_stream_fixture(c: &mut Cpu, r: &mut Rng) {
        c.bus.write16(RDO_VT + 0x10, (r.next_u32() as u16) & !3);
        c.bus.write32(RDO_VT + 0x14, fakes::FAKE);
        c.bus.write32(RDO_STREAM, RDO_VT);
    }

    /// The ref table behind `gp+0x16c8`: ctx -> table A, [+8] -> table B (by
    /// the low byte of a ref), entry +0x1c -> table C (by `ref >> 8`). Refs
    /// below 0x4000 reach the nodes at `RDO_NODES`, zero for some entries.
    fn dead_table_fixture(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP + 0x16c8, RDO_CTX);
        c.bus.write32(RDO_CTX, RDO_TA);
        c.bus.write32(RDO_TA + 8, RDO_TB);
        c.bus.write32(RDO_ENTRY + 0x1c, RDO_TC);
        for lo in 0..256u32 {
            let entry = if lo % 3 == 0 { 0 } else { RDO_ENTRY };
            c.bus.write32(RDO_TB + 4 * lo, entry);
        }
        for j in 0..64u32 {
            let word = if j % 4 == 0 {
                0
            } else {
                // `(word >> 8) << 2` is the node address RDO_NODES + 4 * j.
                (((RDO_NODES >> 2) + j) << 8) | (r.next_u32() & 0xff)
            };
            c.bus.write32(RDO_TC + 4 * j, word);
        }
    }

    fn rdo_setup(c: &mut Cpu, seed: u32) {
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

    /// The read fake of `replay_stream_read_node_ref`: serves the scripted id
    /// and type (`{id, type}`), then logs the call.
    fn rnr_miss(c: &mut Cpu, target: u32) {
        let buf = c.g[A1][0] as u32;
        let id = c.bus.read32(RDO_SCRIPT);
        let ty = c.bus.read32(RDO_SCRIPT + 4);
        c.bus.write32(buf, id);
        c.bus.write32(buf + 4, ty);
        fakes::fake_call(c, target);
    }

    fn rnr_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_ec08, 0);
        c.hooks.miss = rnr_miss;
        let id = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() & 0x3fff
        };
        c.bus.write32(RDO_SCRIPT, id);
        c.bus.write32(RDO_SCRIPT + 4, r.next_u32());
        dead_stream_fixture(c, &mut r);
        dead_table_fixture(c, &mut r);
        c.g[A0][0] = RDO_STREAM as u64;
        c.g[A1][0] = (RDO_NODES + 0x1000) as u64;
    }

    #[test]
    fn replay_stream_read_node_ref_matches() {
        check(
            ee::s07::f_0026ec08__generated,
            replay_stream_read_node_ref,
            200,
            &rnr_setup,
        );
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
    fn snapshot_list_setup(c: &mut Cpu, seed: u32) {
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

    // Fixture of `replay_stream_decompress`: the stream (`Block` at +4,
    // `WriteData` at +8), the `char**` cursor and the source block: a tag at
    // `src`, then the size and the coded bytes at `*pp`.
    const RS_SELF: u32 = DATA + 0x20_8800;
    const RS_CURSOR: u32 = DATA + 0x20_8900;
    const RS_SRC: u32 = DATA + 0x20_8a00;
    const RS_BLOCK: u32 = DATA + 0x20_9000;
    const RS_CODED: u32 = DATA + 0x20_9800;

    fn replay_stream_decompress_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_eab8, 0);
        c.bus.write32(RS_SELF + 0x4, RS_BLOCK);
        c.bus.write32(RS_SELF + 0x8, 0);
        c.bus.write32(RS_CURSOR, RS_CODED);
        c.bus.write32(RS_SRC, r.next_u32());
        c.bus.write32(RS_CODED, 4 + seed % 8);
        for i in 0..16 {
            c.bus.write32(RS_CODED + 4 + 4 * i, r.next_u32());
        }
        c.g[A0][0] = RS_SELF as u64;
        c.g[A1][0] = RS_CURSOR as u64;
        c.g[A2][0] = RS_SRC as u64;
    }

    #[test]
    fn replay_snapshot_empty_hook_b_matches() {
        check(
            ee::s07::f_0026ed90__generated,
            replay_snapshot_empty_hook_b,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x26_ed90, 0);
            },
        );
    }

    #[test]
    fn replay_set_controller_index_matches() {
        check(
            ee::s07::f_0026f898__generated,
            replay_set_controller_index,
            50,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x26_f898, 0);
                let this = DATA + 0x20_a300;
                c.bus.write32(this + 0x28, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // The save frame of a replay sits at `self+0x3d0`; a non-zero one is
    // restored and freed (real callees on a zeroed frame).

    // The replay's stop, reset hook and buffer purge are gated fakes (the
    // gate word in RAM); the frame destructors are a vtable whose slot
    // points at the miss fake. Flags are even (no self delete here).
    const DTOR_THIS: u32 = DATA + 0x20_b000;
    const DTOR_GATE: u32 = DATA + 0x20_b800;
    const DTOR_MAGIC: u32 = 0x4454_5230;
    const DTOR_VT: u32 = DATA + 0x20_bc00;
    const DTOR_OBJS: u32 = DATA + 0x20_c000;

    static OC532_DTOR_PREV_END: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC532_DTOR_PREV_RESET: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC532_DTOR_PREV_PURGE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn dtor_gated(
        c: &mut Cpu,
        prev: &std::sync::atomic::AtomicUsize,
        generated: GuestFn,
        target: u32,
    ) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    fn dtor_fake_end(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_END,
            ee::s07::f_0026f980__generated,
            0x0026_f980,
        )
    }

    fn dtor_fake_reset(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_RESET,
            ee::s07::f_0026f498__generated,
            0x0026_f498,
        )
    }

    fn dtor_fake_purge(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_PURGE,
            ee::s07::f_0026f428__generated,
            0x0026_f428,
        )
    }

    fn dtor_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F980),
            dtor_fake_end,
            &OC532_DTOR_PREV_END,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F498),
            dtor_fake_reset,
            &OC532_DTOR_PREV_RESET,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F428),
            dtor_fake_purge,
            &OC532_DTOR_PREV_PURGE,
        );
    }

    fn replay_dtor_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_f088, 0);
        dtor_install();
        c.bus.write32(DTOR_GATE, DTOR_MAGIC);
        c.bus.write16(DTOR_VT + 0x38, (r.next_u32() as u16) & !3);
        c.bus.write32(DTOR_VT + 0x3c, fakes::FAKE);
        // State 0 (idle), 3 (running: stopped) or 12 (finished).
        c.bus.write32(DTOR_THIS, [0, 3, 12][(seed % 3) as usize]);
        for k in 0..14 {
            let obj = DTOR_OBJS + 0x40 * k;
            c.bus.write32(obj, DTOR_VT);
            c.bus.write32(DTOR_THIS + 0x2c + 0x38 * k, obj);
        }
        c.g[A0][0] = DTOR_THIS as u64;
        c.g[A1][0] = (r.next_u32() & !1) as u64;
    }

    #[test]
    fn replay_dtor_matches() {
        check(ee::s07::f_0026f088__generated, replay_dtor, 60, &replay_dtor_setup);
    }

    // Begin-playback's callees are log-only gated fakes (the gate word in
    // RAM): with the gate set, a call is logged and returns to its caller;
    // otherwise the body the slot held runs. `StepFrame` has no slot and runs
    // as generated.
    const BP_SELF: u32 = DATA + 0x20_d000;
    const BP_GATE: u32 = DATA + 0x20_d800;
    const BP_MAGIC: u32 = 0x4250_4c59;
    static OC532_BP_PREV_26F7F8: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_26f7f8(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26F7F8,
            ee::s07::f_0026f7f8__generated,
            0x0026f7f8,
        )
    }
    static OC532_BP_PREV_26F4A0: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_26f4a0(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26F4A0,
            ee::s07::f_0026f4a0__generated,
            0x0026f4a0,
        )
    }
    static OC532_BP_PREV_2702F8: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_2702f8(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_2702F8,
            ee::s07::f_002702f8__generated,
            0x002702f8,
        )
    }
    static OC532_BP_PREV_26EF30: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_26ef30(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26EF30,
            ee::s07::f_0026ef30__generated,
            0x0026ef30,
        )
    }
    static OC532_BP_PREV_26EE68: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_26ee68(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26EE68,
            ee::s07::f_0026ee68__generated,
            0x0026ee68,
        )
    }
    static OC532_BP_PREV_28B180: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_28b180(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_28B180,
            ee::s07::f_0028b180__generated,
            0x0028b180,
        )
    }
    static OC532_BP_PREV_28F200: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    fn bp_fake_28f200(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_28F200,
            ee::s07::f_0028f200__generated,
            0x0028f200,
        )
    }
    fn bp_gated(
        c: &mut Cpu,
        prev: &std::sync::atomic::AtomicUsize,
        generated: GuestFn,
        target: u32,
    ) {
        if c.bus.read32(BP_GATE) != BP_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    fn bp_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
            bp_fake_26f7f8,
            &OC532_BP_PREV_26F7F8,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            bp_fake_26f4a0,
            &OC532_BP_PREV_26F4A0,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            bp_fake_2702f8,
            &OC532_BP_PREV_2702F8,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
            bp_fake_26ef30,
            &OC532_BP_PREV_26EF30,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
            bp_fake_26ee68,
            &OC532_BP_PREV_26EE68,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            bp_fake_28b180,
            &OC532_BP_PREV_28B180,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028F200),
            bp_fake_28f200,
            &OC532_BP_PREV_28F200,
        );
    }

    fn replay_begin_playback_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_f8a0, 0);
        bp_install();
        c.bus.write32(BP_GATE, BP_MAGIC);
        // States 3, 7 (running) and 14, 15 (stopping: nothing starts).
        c.bus.write32(BP_SELF, [3, 14, 7, 15][(seed % 4) as usize]);
        c.bus
            .write32(BP_SELF + 0x608, if seed % 3 == 0 { 0 } else { 1 });
        c.g[A0][0] = BP_SELF as u64;
        c.g[A1][0] = (DATA + 0x20_d400 + (r.next_u32() & 0x3f0)) as u64;
        c.g[A2][0] = (seed % 2) as u64;
        c.g[A3][0] = (seed % 5 == 0) as u64;
    }

    // `Replay_UpdateCameraInput`: the input and view-manager callees are
    // gated fakes (the gate word in RAM, else the body the slot held). The
    // stick inputs answer from a script: `getValue(code)` gives the word at
    // `RUCI_VALUES + 4 * (code & 3)` as f0.
    const RUCI_SELF: u32 = DATA + 0x20_d900;
    const RUCI_APP: u32 = DATA + 0x20_da00;
    const RUCI_INPUT: u32 = DATA + 0x20_db00;
    const RUCI_VM: u32 = DATA + 0x20_dc00;
    const RUCI_VALUES: u32 = DATA + 0x20_dd00;
    const RUCI_GATE: u32 = DATA + 0x20_dd80;
    const RUCI_MAGIC: u32 = 0x5255_4349;
    const RUCI_SUBSET: u32 = 0x0044_5438;

    static OC532_RUCI_PREV_GETSTATE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC532_RUCI_PREV_GETVALUE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC532_RUCI_PREV_UPDATE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn ruci_fake_getstate(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_GETSTATE, ee::s10::f_00320c48__generated);
        }
        fakes::fake_call(c, 0x0032_0c48);
    }

    fn ruci_fake_getvalue(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_GETVALUE, ee::s10::f_00320bf0__generated);
        }
        let value = c.bus.read32(RUCI_VALUES + 4 * (c.g[A1][0] as u32 & 3));
        fakes::fake_call(c, 0x0032_0bf0);
        c.f[0] = value;
    }

    fn ruci_fake_update(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_UPDATE, ee::s05::f_0022e840__generated);
        }
        fakes::fake_call(c, 0x0022_e840);
    }

    fn ruci_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320C48),
            ruci_fake_getstate,
            &OC532_RUCI_PREV_GETSTATE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            ruci_fake_getvalue,
            &OC532_RUCI_PREV_GETVALUE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_0022E840),
            ruci_fake_update,
            &OC532_RUCI_PREV_UPDATE,
        );
    }

    fn replay_update_camera_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_fa78, 0);
        ruci_install();
        c.bus.write32(RUCI_GATE, RUCI_MAGIC);
        // State 3 runs the body; 7 leaves at once.
        c.bus.write32(RUCI_SELF, [3, 7, 3, 3][(seed % 4) as usize]);
        let idx = seed % 2;
        c.bus.write32(RUCI_SELF + 0x28, idx);
        c.bus.write32(RUCI_SELF + 0x630, idx);
        // Camera subset 11 (or another) for that index.
        c.bus
            .write32(RUCI_SUBSET + 4 * idx, if seed % 3 == 0 { 5 } else { 11 });
        c.bus.write32(GAME_GP - 0x848, RUCI_APP);
        c.bus.write32(RUCI_APP + 0xb0 + 4 * idx, RUCI_INPUT);
        c.bus.write32(RUCI_APP + 0x84, RUCI_VM);
        c.bus.write32(RUCI_VM + 0x84, r.next_u32());
        for k in 0..4 {
            c.bus.write32(RUCI_VALUES + 4 * k, 0);
        }
        if seed % 2 == 0 {
            c.bus
                .write32(RUCI_VALUES + 4 * ((seed / 2) % 4), 0x3f80_0000);
        }
        c.g[A0][0] = RUCI_SELF as u64;
    }

    // The save-buffer free and the replay reset are gated log-only fakes
    // (the gate word in RAM), as in `Replay_BeginPlayback`.
    const LOAD_GATE: u32 = DATA + 0x20_de00;
    const LOAD_MAGIC: u32 = 0x4c4f_4144;

    static OC532_LOAD_PREV_FREESAVE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static OC532_LOAD_PREV_RESET: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn load_fake_freesave(c: &mut Cpu) {
        if c.bus.read32(LOAD_GATE) != LOAD_MAGIC {
            return fakes::fall_back(c, &OC532_LOAD_PREV_FREESAVE, ee::s07::f_00270f78__generated);
        }
        fakes::fake_call(c, 0x0027_0f78);
    }

    fn load_fake_reset(c: &mut Cpu) {
        if c.bus.read32(LOAD_GATE) != LOAD_MAGIC {
            return fakes::fall_back(c, &OC532_LOAD_PREV_RESET, ee::s07::f_0026f228__generated);
        }
        fakes::fake_call(c, 0x0026_f228);
    }

    fn load_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00270F78),
            load_fake_freesave,
            &OC532_LOAD_PREV_FREESAVE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F228),
            load_fake_reset,
            &OC532_LOAD_PREV_RESET,
        );
    }

    fn load_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x27_0ed8, 0);
        load_install();
        c.bus.write32(LOAD_GATE, LOAD_MAGIC);
    }

    #[test]
    fn replay_end_save_matches() {
        check(ee::s07::f_00270ed8__generated, replay_end_save, 20, &load_setup);
    }

    #[test]
    fn replay_reset_on_load_matches() {
        check(ee::s07::f_00270ef8__generated, replay_reset_on_load, 20, &load_setup);
    }

    #[test]
    fn replay_empty_load_hook_matches() {
        check(
            ee::s07::f_00270f18__generated,
            replay_empty_load_hook,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x27_0f18, 0);
            },
        );
    }
}
