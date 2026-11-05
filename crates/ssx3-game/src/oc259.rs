//! Staging wave 207 (oc259): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

// ---------------------------------------------------------------------------
// RaceStateRace_Update (0x113db0)

/// AI race-state object: the frame counter of the current state (+0xc).
const RACE_STATE_FRAMES: u32 = 0xc;
/// The world (gp-0x848) and its game module (+0x84).
const RACE_WORLD_GP: u32 = 0x848;
const RACE_WORLD_GAME_MODULE: u32 = 0x84;
/// The AI state "race finished" and the game module state it requests.
const RACE_AI_STATE_FINISHED: u64 = 0x6;
const RACE_MODULE_STATE_END_RACE: u64 = 0x5;

/// `AI_AllPlayersFinished` (0x12a250).
const AI_ALL_PLAYERS_FINISHED: Callee = Callee {
    addr: 0x0012_a250,
    f: ee::s00::AI_AllPlayersFinished,
};
/// `AI_SetRaceState` (0x113b10).
const AI_SET_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b10,
    f: ee::s00::f_00113b10,
};
/// `GameModule_RequestState` (0x231250, through the function table).
const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;

/// `void RaceStateRace_Update(void *state, AI *ai /*a1*/)` (0x113db0): the
/// per-frame update of the AI's "race" state. Counts the frame
/// (`ai->frames++`); once every player has finished, switches the AI to the
/// finished state (6) and asks the game module for the end-of-race state
/// (`GameModule_RequestState(world->module, 5, 0, 0)`). Frame 0x20 (ra at
/// 0, s0 at 0x10).
pub fn race_state_race_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        let ai = Addr(r32(c, S0));
        let frames = sx((ai.off(RACE_STATE_FRAMES).lw(c) as u32).wrapping_add(1));
        c.g[V0][0] = frames;
        ai.off(RACE_STATE_FRAMES).sw(c, frames);
        AI_ALL_PLAYERS_FINISHED.call(c, 0x0011_3dd4)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = RACE_AI_STATE_FINISHED;
            AI_SET_RACE_STATE.call(c, 0x0011_3dec)?;
            let world = Addr(r32(c, GP).wrapping_sub(RACE_WORLD_GP)).lw(c);
            c.g[V0][0] = world;
            c.g[A1][0] = RACE_MODULE_STATE_END_RACE;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = Addr(world as u32).off(RACE_WORLD_GAME_MODULE).lw(c);
            call_ptr(c, GAME_MODULE_REQUEST_STATE, 0x0011_3e04)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DLHeap_Free (0x31ed60)

/// dlmalloc state (`*heap`): `max_fast` with the flag bits (bit 0: state
/// initialised, bit 1: fast chunks present), the fast bins (from +0), the
/// top chunk (+0x30), the unsorted bin header (+0x38, its `fd` at +0x40),
/// the trim threshold (+0x344) and the top pad (+0x348).
const DLF_TOP: u32 = 0x30;
const DLF_UNSORTED: u32 = 0x38;
const DLF_UNSORTED_FD: u32 = 0x40;
const DLF_TRIM_THRESHOLD: u32 = 0x344;
const DLF_TOP_PAD: u32 = 0x348;
/// Chunk fields: previous size (+0), size with flags (+4), `fd` (+8), `bk`
/// (+0xc); size flags: previous in use (bit 0), mmapped (bit 1).
const DLF_CHUNK_SIZE: u32 = 0x4;
const DLF_CHUNK_FD: u32 = 0x8;
const DLF_CHUNK_BK: u32 = 0xc;
/// `FASTBIN_CONSOLIDATION_THRESHOLD`.
const DLF_CONSOLIDATE_THRESHOLD: u64 = 0x4000;

/// `DLHeap_Consolidate` (0x31eee8).
const DLHEAP_CONSOLIDATE: Callee = Callee {
    addr: 0x0031_eee8,
    f: ee::s10::f_0031eee8,
};
/// `DLHeap_SysTrim` (0x31e6d8).
const DLHEAP_SYS_TRIM: Callee = Callee {
    addr: 0x0031_e6d8,
    f: ee::s10::f_0031e6d8,
};

/// `void DLHeap_Free(DLHeap *heap, void *mem)` (0x31ed60): dlmalloc's
/// `free`. Null is ignored; a chunk no bigger than `max_fast` goes on the
/// front of its fast bin (and marks the heap as having fast chunks); an
/// mmapped chunk is left alone; any other chunk is merged with a free
/// previous chunk, then with the top chunk (becoming the new top) or a free
/// next chunk, and put on the unsorted bin. When the merged chunk reaches
/// 16 KiB, pending fast chunks are consolidated and the top chunk is
/// trimmed once it passes the trim threshold. Frame 0x30 (ra at 0, s1 at
/// 0x10, s0 at 0x20).
pub fn dlheap_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        let mem = c.g[A1][0];
        let state = Addr(r32(c, S1)).lw(c);
        c.g[S0][0] = state;
        let av = Addr(state as u32);
        'out: {
            if mem == 0 {
                c.cycles += 1;
                break 'out;
            }
            let mem = mem as u32;
            c.cycles += 8;
            let head = Addr(mem.wrapping_sub(4)).lw(c);
            c.g[T0][0] = head;
            let size_mask: u64 = 0xffff_ffff_ffff_fffc;
            c.g[A0][0] = size_mask;
            let max_fast = av.lw(c);
            c.g[V1][0] = max_fast;
            let mut size = head & size_mask;
            c.g[A2][0] = size;
            c.g[V0][0] = (max_fast < size) as u64;
            let mut p = sx(mem.wrapping_sub(8));
            c.g[A3][0] = p;
            if max_fast >= size {
                // Fast bin: push on the front of bin (size >> 3) - 1.
                c.cycles += 10;
                av.sw(c, max_fast | 0x3);
                c.g[V1][0] = max_fast | 0x3;
                let bin = sx(((size as u32) >> 3) << 2);
                let bin = sx((bin as u32).wrapping_sub(4));
                let bin = sx(av.0.wrapping_add(bin as u32));
                c.g[V0][0] = bin;
                let old = Addr(bin as u32).lw(c);
                c.g[V1][0] = old;
                Addr(mem).sw(c, old);
                Addr(bin as u32).sw(c, p);
                c.cycles += 1;
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = head & 0x2;
            if c.g[V0][0] != 0 {
                // Mmapped chunk: not handled by this heap.
                break 'out;
            }
            c.cycles += 9;
            c.g[V0][0] = max_fast | 0x1;
            let next = sx((p as u32).wrapping_add(size as u32));
            c.g[T0][0] = next;
            av.sw(c, max_fast | 0x1);
            let prev_inuse = Addr(mem.wrapping_sub(4)).lw(c);
            let next_head = Addr(next as u32).off(DLF_CHUNK_SIZE).lw(c);
            c.g[V0][0] = next_head;
            c.g[V1][0] = (prev_inuse ^ 0x1) & 0x1;
            let next_size = next_head & size_mask;
            c.g[T1][0] = next_size;
            if c.g[V1][0] != 0 {
                // Merge with the free previous chunk (unlink it).
                c.cycles += 7;
                let prev_size = Addr(mem.wrapping_sub(8)).lw(c);
                c.g[V0][0] = prev_size;
                p = sx((p as u32).wrapping_sub(prev_size as u32));
                c.g[A3][0] = p;
                size = sx((size as u32).wrapping_add(prev_size as u32));
                c.g[A2][0] = size;
                let fd = Addr(p as u32).off(DLF_CHUNK_FD).lw(c);
                c.g[A0][0] = fd;
                let bk = Addr(p as u32).off(DLF_CHUNK_BK).lw(c);
                c.g[V0][0] = bk;
                Addr(fd as u32).off(DLF_CHUNK_BK).sw(c, bk);
                Addr(bk as u32).off(DLF_CHUNK_FD).sw(c, fd);
            }
            c.cycles += 3;
            let top = av.off(DLF_TOP).lw(c);
            c.g[V0][0] = top;
            let after_next = sx((next as u32).wrapping_add(next_size as u32));
            c.g[V1][0] = after_next;
            if next == top {
                // Merge into the top chunk.
                c.cycles += 4;
                size = sx((size as u32).wrapping_add(next_size as u32));
                c.g[A2][0] = size;
                c.g[V0][0] = size | 0x1;
                Addr(p as u32).off(DLF_CHUNK_SIZE).sw(c, size | 0x1);
                av.off(DLF_TOP).sw(c, p);
            } else {
                c.cycles += 4;
                let next_inuse = Addr(after_next as u32).off(DLF_CHUNK_SIZE).lw(c) & 0x1;
                c.g[V0][0] = next_inuse;
                // Clear the next chunk's "previous in use".
                Addr(next as u32).off(DLF_CHUNK_SIZE).sw(c, next_size);
                if next_inuse == 0 {
                    // Merge with the free next chunk (unlink it).
                    c.cycles += 5;
                    let bk = Addr(next as u32).off(DLF_CHUNK_BK).lw(c);
                    c.g[V0][0] = bk;
                    size = sx((size as u32).wrapping_add(next_size as u32));
                    c.g[A2][0] = size;
                    let fd = Addr(next as u32).off(DLF_CHUNK_FD).lw(c);
                    c.g[A0][0] = fd;
                    Addr(fd as u32).off(DLF_CHUNK_BK).sw(c, bk);
                    Addr(bk as u32).off(DLF_CHUNK_FD).sw(c, fd);
                }
                // Put the chunk on the front of the unsorted bin.
                c.cycles += 11;
                let first = av.off(DLF_UNSORTED_FD).lw(c);
                c.g[A0][0] = first;
                let bin = sx(av.0.wrapping_add(DLF_UNSORTED));
                Addr(p as u32).off(DLF_CHUNK_BK).sw(c, bin);
                c.g[V1][0] = size | 0x1;
                Addr(p as u32).off(DLF_CHUNK_FD).sw(c, first);
                let foot = sx((p as u32).wrapping_add(size as u32));
                c.g[V0][0] = foot;
                av.off(DLF_UNSORTED_FD).sw(c, p);
                Addr(first as u32).off(DLF_CHUNK_BK).sw(c, p);
                Addr(p as u32).off(DLF_CHUNK_SIZE).sw(c, size | 0x1);
                Addr(foot as u32).sw(c, size);
            }
            c.cycles += 3;
            c.g[V0][0] = (size < DLF_CONSOLIDATE_THRESHOLD) as u64;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            let flags = Addr(r32(c, S0)).lw(c) & 0x2;
            c.g[V0][0] = flags;
            c.g[A0][0] = c.g[S1][0];
            if flags != 0 {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                DLHEAP_CONSOLIDATE.call(c, 0x0031_eea0)?;
            }
            c.cycles += 9;
            let av = Addr(r32(c, S0));
            let top = av.off(DLF_TOP).lw(c);
            c.g[A1][0] = top;
            c.g[V1][0] = size_mask;
            let threshold = av.off(DLF_TRIM_THRESHOLD).lw(c);
            c.g[A0][0] = threshold;
            let top_size = Addr(top as u32).off(DLF_CHUNK_SIZE).lw(c) & size_mask;
            c.g[V0][0] = (top_size < threshold) as u64;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A1][0] = av.off(DLF_TOP_PAD).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            DLHEAP_SYS_TRIM.call(c, 0x0031_eed4)?;
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
// ScriptMovie_Update (0x283aa0)

/// Script movie: the player (+0), "started" (+4), "updated" (+8)
/// and "finished" (+0xc).
const SCRIPT_MOVIE_PLAYER: u32 = 0x0;
const SCRIPT_MOVIE_STARTED: u32 = 0x4;
const SCRIPT_MOVIE_UPDATED: u32 = 0x8;
const SCRIPT_MOVIE_DONE: u32 = 0xc;
/// `VidEnginePlayer_Update` (0x253938, through the function table).
const VID_ENGINE_PLAYER_UPDATE: u32 = 0x0025_3938;

/// `void ScriptMovie_Update(ScriptMovie *movie)` (0x283aa0): while a
/// script-driven movie has a player, has started and is not done, updates
/// its video player and sets +8. Frame 0x20 (ra at 0, s0 at 0x10).
pub fn script_movie_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = Addr(r32(c, S0));
        c.g[A0][0] = movie.off(SCRIPT_MOVIE_PLAYER).lw(c);
        'out: {
            if c.g[A0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = movie.off(SCRIPT_MOVIE_STARTED).lw(c);
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = movie.off(SCRIPT_MOVIE_DONE).lw(c);
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 5;
            call_ptr(c, VID_ENGINE_PLAYER_UPDATE, 0x0028_3adc)?;
            c.g[V0][0] = 1;
            Addr(r32(c, S0)).off(SCRIPT_MOVIE_UPDATED).sw(c, 1);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3db0, race_state_race_update, "RaceStateRace_Update"),
    (0x0031_ed60, dlheap_free, "DLHeap_Free"),
    (0x0028_3aa0, script_movie_update, "ScriptMovie_Update"),
    (0x0032_84b8, spatial_hash_remove, "SpatialHash_Remove"),
    (0x002e_edb0, world_painter_get_light_glow08, "WorldPainter_GetLightGlow08"),
    (0x002e_e660, world_painter_get_weather50, "WorldPainter_GetWeather50"),
    (0x0012_e528, rider_wipeout_sub_update_anim_speed, "RiderWipeoutSub_UpdateAnimSpeed"),
    (0x0027_28f8, in_seq_update_all, "InSeq_UpdateAll"),
    (0x003d_fc48, stream_file_consume_bytes, "StreamFile_ConsumeBytes"),
];

// ---------------------------------------------------------------------------
// SpatialHash_Remove (0x3284b8)

/// Spatial hash (octree) node: eight child pointers (+0..+0x1c), then the
/// heads of the item lists (+0x20 + 4 * slot, three slots).
const SH_NODE_ITEMS: u32 = 0x20;
const SH_ITEM_SLOTS: i64 = 3;
/// Item link: next (+0), previous (+4).
const SH_LINK_NEXT: u32 = 0x0;
const SH_LINK_PREV: u32 = 0x4;
/// Keys are `{level, x, y, z}` words.
const SH_KEY_X: u32 = 0x4;
const SH_KEY_Y: u32 = 0x8;
const SH_KEY_Z: u32 = 0xc;
/// Resume points of the emptiness scan (event checks at its back edges).
const SH_REMOVE_ENTRY: u32 = 0x0032_84b8;
const SH_REMOVE_NOT_EMPTY: u32 = 0x0032_8614;
const SH_REMOVE_SCAN_ITEMS: u32 = 0x0032_8620;
const SH_REMOVE_END: u32 = 0x0032_8660;
/// `SpatialHash_Remove` itself (the recursion) and `__builtin_delete`.
const SPATIAL_HASH_REMOVE: Callee = Callee {
    addr: SH_REMOVE_ENTRY,
    f: ee::s10::SpatialHash_Remove,
};
const SH_BUILTIN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::f_00317e50,
};

/// `void SpatialHash_Remove(Node *node /*a0*/, int slot /*a1*/, Link *link
/// /*a2*/, Key *item /*a3*/, Key *at /*t0*/)` (0x3284b8): removes an item's
/// link from the octree. At the item's level the link is unlinked from its
/// list (the node's list head for `slot` when it was first); above it the
/// function recurses into the child containing the item (child key on the
/// stack: one level down, coordinates doubled plus the item's bit), and
/// deletes that child once it has no children and no items left. Frame 0x30
/// (child key at 0, ra at 0x10, s0 at 0x20). Dispatcher-shaped: the scan's
/// event checks may unwind, and it is re-entered at 0x328614 / 0x328620.
pub fn spatial_hash_remove(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| match entry {
        SH_REMOVE_ENTRY => spatial_hash_remove_body(c),
        SH_REMOVE_NOT_EMPTY | SH_REMOVE_SCAN_ITEMS => spatial_hash_remove_tail(c, entry),
        pc => {
            c.jump_out(pc, SH_REMOVE_ENTRY, SH_REMOVE_END);
            Ok(())
        }
    })
}

fn spatial_hash_remove_body(c: &mut Cpu) -> Flow {
    let frame = Frame { size: 0x30 };
    c.cycles += 11;
    Frame::open(c, 0x30);
    c.g[T3][0] = c.g[A3][0];
    frame.save(c, S0, 0x20);
    c.g[V1][0] = c.g[T0][0];
    frame.save_ra(c, 0x10);
    c.g[T4][0] = c.g[A0][0];
    c.g[T5][0] = c.g[A1][0];
    let item_level = Addr(r32(c, T3)).lw(c);
    c.g[V0][0] = item_level;
    let level = Addr(r32(c, V1)).lw(c);
    c.g[T1][0] = level;
    c.g[T2][0] = c.g[A2][0];
    let link = Addr(r32(c, T2));

    if item_level == level {
        // At the item's level: unlink it.
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, T5) << 2);
        let next = link.off(SH_LINK_NEXT).lw(c);
        c.g[V1][0] = next;
        let head = sx(r32(c, V0).wrapping_add(r32(c, T4)).wrapping_add(SH_NODE_ITEMS));
        c.g[A0][0] = head;
        if next != 0 {
            c.cycles += 2;
            let prev = link.off(SH_LINK_PREV).lw(c);
            c.g[V0][0] = prev;
            Addr(next as u32).off(SH_LINK_PREV).sw(c, prev);
        }
        c.cycles += 3;
        let prev = link.off(SH_LINK_PREV).lw(c);
        c.g[V1][0] = prev;
        let next = link.off(SH_LINK_NEXT).lw(c);
        c.g[V0][0] = next;
        c.cycles += 2;
        if prev != 0 {
            Addr(prev as u32).off(SH_LINK_NEXT).sw(c, next);
        } else {
            Addr(head as u32).sw(c, next);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        spatial_hash_remove_epilogue(c, frame);
        return Ok(());
    }

    // Above it: the child key (one level down) and the child slot.
    c.cycles += 41;
    let at = Addr(r32(c, V1));
    let item = Addr(r32(c, T3));
    let shift = (level as u32).wrapping_sub(item_level as u32).wrapping_sub(1);
    c.g[A0][0] = sx(shift);
    let sh = shift & 31;
    let x2 = sx((at.off(SH_KEY_X).lw(c) as u32) << 1);
    let z2 = sx((at.off(SH_KEY_Z).lw(c) as u32) << 1);
    let y2 = sx((at.off(SH_KEY_Y).lw(c) as u32) << 1);
    let ix = sx(((item.off(SH_KEY_X).lw(c) as u32) as i32 >> sh) as u32);
    let iy = sx(((item.off(SH_KEY_Y).lw(c) as u32) as i32 >> sh) as u32);
    let iz = sx(((item.off(SH_KEY_Z).lw(c) as u32) as i32 >> sh) as u32);
    let bx = sx((ix as u32).wrapping_sub(x2 as u32));
    let by = sx((iy as u32).wrapping_sub(y2 as u32));
    let bz = sx((iz as u32).wrapping_sub(z2 as u32));
    let cy = sx((y2 as u32).wrapping_add(by as u32));
    let cx = sx((x2 as u32).wrapping_add(bx as u32));
    let cz = sx((z2 as u32).wrapping_add(bz as u32));
    let off_y = sx((by as u32) << 3);
    let off_z = sx((bz as u32) << 2);
    let slot = sx(r32(c, T4).wrapping_add((bx as u32) << 4).wrapping_add(off_y as u32));
    let child_level = sx((level as u32).wrapping_sub(1));
    c.g[T1][0] = child_level;
    let key = Addr(sp(c));
    key.off(SH_KEY_X).sw(c, cx);
    key.off(SH_KEY_Y).sw(c, cy);
    key.off(SH_KEY_Z).sw(c, cz);
    let child_slot = sx((slot as u32).wrapping_add(off_z as u32));
    c.g[S0][0] = child_slot;
    key.sw(c, child_level);
    c.g[V0][0] = off_y;
    c.g[V1][0] = off_z;
    c.g[A1][0] = c.g[T5][0];
    c.g[A2][0] = c.g[T2][0];
    c.g[A3][0] = c.g[T3][0];
    c.g[A0][0] = Addr(child_slot as u32).lw(c);
    c.g[T0][0] = c.g[SP][0];
    SPATIAL_HASH_REMOVE.call(c, 0x0032_85b0)?;

    // Is the child empty now? First its eight child pointers.
    let child = Addr(r32(c, S0)).lw(c);
    c.g[V1][0] = child;
    for i in 0..8u32 {
        if i > 0 {
            c.cycles += 3;
        }
        let v = Addr(child as u32).off(4 * i).lw(c);
        c.g[V0][0] = v;
        if v != 0 {
            if i == 7 {
                return spatial_hash_remove_tail(c, SH_REMOVE_NOT_EMPTY);
            }
            c.g[V0][0] = 0;
            return spatial_hash_remove_tail(c, 0x0032_863c);
        }
        if i < 7 {
            c.g[V0][0] = 0;
        }
    }
    c.g[V1][0] = sx((child as u32).wrapping_add(SH_NODE_ITEMS));
    c.cycles += 1;
    c.g[A0][0] = 0;
    spatial_hash_remove_tail(c, SH_REMOVE_SCAN_ITEMS)
}

/// The end of the emptiness scan from `at`: the item slots (from
/// 0x328620, v1 = the slot, a0 = slots seen), "not empty" (0x328614) or the
/// decision (0x32863c, v0 = empty): delete the child and clear its pointer.
fn spatial_hash_remove_tail(c: &mut Cpu, mut at: u32) -> Flow {
    let frame = Frame { size: 0x30 };
    loop {
        match at {
            SH_REMOVE_NOT_EMPTY => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                at = 0x0032_863c;
            }
            SH_REMOVE_SCAN_ITEMS => {
                c.cycles += 3;
                let v = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = v;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if v != 0 {
                    if back_edge_unwinds(c, SH_REMOVE_NOT_EMPTY) {
                        return Err(crate::call::Unwound);
                    }
                    at = SH_REMOVE_NOT_EMPTY;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < SH_ITEM_SLOTS) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, SH_REMOVE_SCAN_ITEMS) {
                        return Err(crate::call::Unwound);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                at = 0x0032_863c;
            }
            _ => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    SH_BUILTIN_DELETE.call(c, 0x0032_864c)?;
                    Addr(r32(c, S0)).sw(c, 0);
                    c.cycles += 1;
                }
                frame.restore(c, S0, 0x20);
                spatial_hash_remove_epilogue(c, frame);
                return Ok(());
            }
        }
    }
}

fn spatial_hash_remove_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow08 (0x2eedb0), WorldPainter_GetWeather50 (0x2ee660)

/// The world painter view table `D_004FA370` (built as `0x500000 - 0x5c90`),
/// 0xf0 bytes per view; each view holds pointers to interface holders
/// (`holder -> object`, the object's vtable at +4).
const WP_GET_VIEWS: u32 = 0x0050_0000;
const WP_GET_VIEWS_NEG: u32 = 0x5c90;
const WP_GET_VIEW_SIZE: u64 = 0xf0;
/// View fields: the light-glow interface holder (+0x8), the weather one
/// (+0x20).
const WP_VIEW_LIGHT_GLOW: u32 = 0x8;
const WP_VIEW_WEATHER: u32 = 0x20;
/// Object vtable pointer offset.
const WP_OBJ_VTABLE: u32 = 0x4;

/// The shared body of the world painter's float getters: call the virtual
/// method whose `{this delta, target}` pair is at `vslot` in the vtable of
/// the object held at `holder_off` in view `i` (a0), and return the float
/// at the pointer it returns. Frame 0x10 (ra at 0).
fn wp_view_get_float(c: &mut Cpu, holder_off: u32, vslot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_GET_VIEW_SIZE;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = sx(WP_GET_VIEWS);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_GET_VIEWS.wrapping_sub(WP_GET_VIEWS_NEG));
        let view = Addr(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[A0][0] = sx(view.0);
        let holder = view.off(holder_off).lw(c);
        c.g[V1][0] = holder;
        let obj = Addr(holder as u32).lw(c);
        c.g[A2][0] = obj;
        let vtable = Addr(obj as u32).off(WP_OBJ_VTABLE).lw(c);
        c.g[A1][0] = vtable;
        let delta = Addr(vtable as u32).off(vslot).lh(c);
        let target = Addr(vtable as u32).off(vslot + 4).lw(c);
        c.g[V0][0] = target;
        c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
        call_ptr(c, target as u32, ret)?;
        let v = Addr(r32(c, V0)).read_fpu(c);
        c.f[0] = v.0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLightGlow08(int view)` (0x2eedb0): the light
/// glow interface's float from virtual slot 0x68 of view `view`.
pub fn world_painter_get_light_glow08(c: &mut Cpu) {
    wp_view_get_float(c, WP_VIEW_LIGHT_GLOW, 0x68, 0x002e_ede8);
}

/// `float WorldPainter_GetWeather50(int view)` (0x2ee660): the weather
/// interface's float from virtual slot 0x1a0 of view `view`.
pub fn world_painter_get_weather50(c: &mut Cpu) {
    wp_view_get_float(c, WP_VIEW_WEATHER, 0x1a0, 0x002e_e698);
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdateAnimSpeed (0x12e528)

/// Wipeout sub-state fields: the tumble timer (+0x70) and the rider (+0x80).
const WIPEOUT_TIMER: u32 = 0x70;
const WIPEOUT_RIDER: u32 = 0x80;
/// Rider fields: progress along the course (+0x470), the motion state
/// (+0x77c, its velocity at +0x60) and the animation controller (+0x784).
const WIPEOUT_RIDER_PROGRESS: u32 = 0x470;
const WIPEOUT_RIDER_MOTION: u32 = 0x77c;
const WIPEOUT_MOTION_VELOCITY: u32 = 0x60;
const WIPEOUT_RIDER_ANIM: u32 = 0x784;
/// Animation sequence: playback speed (+0x10) and speed scale (+0x90).
const WIPEOUT_SEQ_SPEED: u32 = 0x10;
const WIPEOUT_SEQ_SCALE: u32 = 0x90;
/// The animation layer the wipeout plays on, and the two animation ids
/// that always play at normal speed.
const WIPEOUT_LAYER: u64 = 2;
const WIPEOUT_ANIM_FIXED_A: u64 = 0x1d;
const WIPEOUT_ANIM_FIXED_B: u64 = 0x19;
/// `1 / (2 pi)` (gp-0x77a4).
const WIPEOUT_INV_TWO_PI_GP: u32 = 0x77a4;
const WIPEOUT_HALF: u32 = 0x3f00_0000;
const WIPEOUT_ONE: u32 = 0x3f80_0000;
const WIPEOUT_TWO: u32 = 0x4000_0000;
/// `AnimController_GetLayerSequence` (0x311b20) and
/// `AnimController_GetLayerAnimEntry` (0x311ae8), through the table.
const WIPEOUT_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;
const WIPEOUT_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

/// `float RiderWipeoutSub_UpdateAnimSpeed(RiderWipeoutSub *self)`
/// (0x12e528): scales the tumble animation (layer 2) with the rider's
/// speed: `|velocity| * sequence speed / (2 pi)` clamped to [0.5, 2] (a
/// NaN gives 0.5). Animations 0x1d and 0x19 play at 1; after the tumble
/// timer reaches 1 or while the rider is not behind the start (progress
/// >= 0) the scale is at least 2; otherwise the clamped value. Returns the
/// clamped value. Frame 0x40 (ra at 0, s1 at 0x10, s0 at 0x20, f20 at
/// 0x30).
pub fn rider_wipeout_sub_update_anim_speed(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = WIPEOUT_LAYER;
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
        c.g[V0][0] = rider;
        let rider = Addr(rider as u32);
        c.g[A0][0] = rider.off(WIPEOUT_RIDER_ANIM).lw(c);
        c.g[S0][0] = rider.off(WIPEOUT_RIDER_MOTION).lw(c);
        call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e554)?;

        // speed = |velocity| (VU0 macro mode, as the original).
        let seq_speed = Addr(r32(c, V0)).off(WIPEOUT_SEQ_SPEED).read_fpu(c);
        c.f[1] = seq_speed.0;
        let vel = r32(c, S0).wrapping_add(WIPEOUT_MOTION_VELOCITY);
        c.lqc2(3, vel);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        let speed = FpuF32(c.g[V1][0] as u32);
        let inv_two_pi = Addr(gp.wrapping_sub(WIPEOUT_INV_TWO_PI_GP)).read_fpu(c);
        c.f[3] = inv_two_pi.0;
        let half = FpuF32(WIPEOUT_HALF);
        let two = FpuF32(WIPEOUT_TWO);
        c.f[2] = half.0;
        c.f[4] = two.0;
        c.g[AT][0] = WIPEOUT_TWO as u64;
        let rate = speed * seq_speed * inv_two_pi;
        c.f[0] = rate.0;
        let above = half.le(rate);
        abi::set_fcc(c, above);
        let clamped = if above {
            c.cycles += 1;
            rate.min(two)
        } else {
            half
        };
        c.f[20] = clamped.0;

        // The animation playing on the layer decides the scale.
        c.cycles += 7;
        let anim = |c: &mut Cpu| {
            let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
            c.g[V0][0] = rider;
            c.g[A0][0] = Addr(rider as u32).off(WIPEOUT_RIDER_ANIM).lw(c);
        };
        c.g[A1][0] = WIPEOUT_LAYER;
        anim(c);
        call_ptr(c, WIPEOUT_GET_LAYER_ANIM_ENTRY, 0x0012_e5c0)?;
        c.g[V1][0] = WIPEOUT_ANIM_FIXED_A;
        c.g[A1][0] = WIPEOUT_LAYER;
        let mut fixed = c.g[V0][0] == c.g[V1][0];
        if !fixed {
            c.cycles += 6;
            anim(c);
            call_ptr(c, WIPEOUT_GET_LAYER_ANIM_ENTRY, 0x0012_e5d8)?;
            c.g[V1][0] = WIPEOUT_ANIM_FIXED_B;
            fixed = c.g[V0][0] == c.g[V1][0];
        }
        let one = FpuF32(WIPEOUT_ONE);
        if fixed {
            c.cycles += 8;
            c.g[A1][0] = WIPEOUT_LAYER;
            anim(c);
            call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e5f4)?;
            c.g[AT][0] = WIPEOUT_ONE as u64;
            c.f[0] = one.0;
            Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, one);
        } else {
            let timer = Addr(r32(c, S1)).off(WIPEOUT_TIMER).read_fpu(c);
            c.f[1] = timer.0;
            c.cycles += 6;
            c.g[AT][0] = WIPEOUT_ONE as u64;
            c.f[0] = one.0;
            let timer_done = one.le(timer);
            abi::set_fcc(c, timer_done);
            let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
            c.g[A0][0] = rider;
            let mut at_least_two = timer_done;
            if !timer_done {
                c.cycles += 7;
                c.g[V0][0] = rider;
                c.f[1] = 0;
                let progress = Addr(rider as u32).off(WIPEOUT_RIDER_PROGRESS).read_fpu(c);
                c.f[0] = progress.0;
                let ahead = FpuF32(0).le(progress);
                abi::set_fcc(c, ahead);
                c.g[V1][0] = 1;
                if !ahead {
                    c.cycles += 1;
                    c.g[V1][0] = 0;
                }
                c.cycles += 2;
                at_least_two = ahead;
            }
            if at_least_two {
                c.cycles += 8;
                c.g[A0][0] = Addr(r32(c, A0)).off(WIPEOUT_RIDER_ANIM).lw(c);
                c.g[A1][0] = WIPEOUT_LAYER;
                call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e650)?;
                c.g[AT][0] = WIPEOUT_TWO as u64;
                let scale = FpuF32(c.f[20]).max(two);
                c.f[0] = scale.0;
                Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, scale);
            } else {
                c.g[A0][0] = Addr(r32(c, V0)).off(WIPEOUT_RIDER_ANIM).lw(c);
                c.cycles += 3;
                c.g[A1][0] = WIPEOUT_LAYER;
                call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e66c)?;
                let scale = FpuF32(c.f[20]);
                Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, scale);
            }
        }
        c.cycles += 7;
        c.f[0] = c.f[20];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_UpdateAll (0x2728f8)

/// Input sequencer: the first sequence node (+0x1c). Node: the instance
/// (+8) and the next node (+0x50).
const INSEQ_FIRST: u32 = 0x1c;
const INSEQ_NODE_INSTANCE: u32 = 0x8;
const INSEQ_NODE_NEXT: u32 = 0x50;
/// `InSeq_InstanceUpdate` (0x274a30).
const INSEQ_INSTANCE_UPDATE: Callee = Callee {
    addr: 0x0027_4a30,
    f: ee::s07::InSeq_InstanceUpdate,
};

/// `void InSeq_UpdateAll(InSeq *seq)` (0x2728f8): updates the instance of
/// every node in the sequencer's list. Frame 0x20 (ra at 0, s0 at 0x10).
pub fn in_seq_update_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr(r32(c, A0)).off(INSEQ_FIRST).lw(c);
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(INSEQ_NODE_INSTANCE));
                INSEQ_INSTANCE_UPDATE.call(c, 0x0027_2918)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(INSEQ_NODE_NEXT).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2910);
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ConsumeBytes (0x3dfc48)

/// Stream file fields: its mutex (+4), the read mode (+0x38), the refill
/// priority (+0x40), the low-water mark (+0x44), the "needs refill" flag
/// (+0x48), the buffered byte count (+0x4c) and the file handle (+0x174).
const STREAM_FILE_MUTEX: u32 = 0x4;
const STREAM_FILE_MODE: u32 = 0x38;
const STREAM_FILE_PRIORITY: u32 = 0x40;
const STREAM_FILE_LOW_WATER: u32 = 0x44;
const STREAM_FILE_NEEDS_REFILL: u32 = 0x48;
const STREAM_FILE_BUFFERED: u32 = 0x4c;
const STREAM_FILE_HANDLE: u32 = 0x174;
/// The read mode whose refills are reprioritised.
const STREAM_FILE_MODE_STREAMING: u64 = 1;
const STREAM_FILE_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const STREAM_FILE_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};
const STREAM_FILE_SET_REQUEST_PRIORITY: Callee = Callee {
    addr: 0x003d_cfc0,
    f: ee::s14::EAFile_SetRequestPriority,
};

/// `void StreamFile_ConsumeBytes(StreamFile *st, int n)` (0x3dfc48): takes
/// `n` bytes off the buffered count (under the stream's mutex). When that
/// crosses the low-water mark, flags the stream for a refill and, for a
/// streaming read, raises its file request to the refill priority. Frame
/// 0x50 (ra at 0, s3 / s2 / s1 / s0 at 0x10 .. 0x40).
pub fn stream_file_consume_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(r32(c, S3).wrapping_add(STREAM_FILE_MUTEX));
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        STREAM_FILE_MUTEX_LOCK.call(c, 0x003d_fc74)?;
        let st = Addr(r32(c, S3));
        let old = st.off(STREAM_FILE_BUFFERED).lw(c);
        c.g[S0][0] = old;
        c.g[A0][0] = c.g[S1][0];
        let new = sx((old as u32).wrapping_sub(r32(c, S2)));
        c.g[S1][0] = new;
        st.off(STREAM_FILE_BUFFERED).sw(c, new);
        STREAM_FILE_MUTEX_UNLOCK.call(c, 0x003d_fc88)?;
        let st = Addr(r32(c, S3));
        let low = st.off(STREAM_FILE_LOW_WATER).lw(c);
        c.g[V0][0] = low;
        c.g[S0][0] = ((old as i64) < (low as i64)) as u64;
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            let crossed = (new as i64) < (low as i64);
            c.g[V0][0] = 1;
            if crossed {
                c.cycles += 3;
                let mode = st.off(STREAM_FILE_MODE).lw(c);
                c.g[V1][0] = mode;
                st.off(STREAM_FILE_NEEDS_REFILL).sw(c, 1);
                if mode == STREAM_FILE_MODE_STREAMING {
                    c.cycles += 3;
                    c.g[A1][0] = st.off(STREAM_FILE_PRIORITY).lw(c);
                    c.g[A0][0] = st.off(STREAM_FILE_HANDLE).lw(c);
                    STREAM_FILE_SET_REQUEST_PRIORITY.call(c, 0x003d_fcbc)?;
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::machine;
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};

    /// Runs a guest function to completion as a setup step (returns v0).
    fn oc259_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// RaceStateRace_Update: no riders (all finished), riders all finished
    /// or some still racing (`rider->physics->+0x470` progress below 0);
    /// the AI already in the finished state or not (the real
    /// AI_SelectRaceState runs through its jump table); the real
    /// GameModule_RequestState stores the request.
    #[test]
    fn race_state_race_update_matches() {
        check(ee::s00::f_00113db0__generated, race_state_race_update, 300, &|c, seed| {
            let mut r = Rng(0x0011_3db0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0011_3db0;
            // AI_SelectRaceState's jump table (states 1..7).
            put_words(
                c,
                0x0045_6ca0,
                &[0x0011_3b74, 0x0011_3b94, 0x0011_3ba8, 0x0011_3bbc, 0x0011_3bd0, 0x0011_3be4, 0x0011_3bf8],
            );
            let ai = DATA;
            let state = if seed % 3 == 0 { 6 } else { r.next_u32() % 9 };
            c.bus.write32(ai, state);
            c.bus.write32(ai + RACE_STATE_FRAMES, r.next_u32());
            c.bus.write32(ai + 0x98, r.next_u32());
            let riders = if seed % 4 == 0 { 0 } else { 1 + r.next_u32() % 4 };
            c.bus.write32(ai + 0x7c, riders);
            let all_done = seed % 2 == 0;
            for i in 0..riders {
                let rider = DATA + 0x1000 + 0x100 * i;
                let phys = DATA + 0x2000 + 0x1000 * i;
                c.bus.write32(ai + 0x40 + 4 * i, rider);
                c.bus.write32(rider + 0x18, phys);
                let racing = !all_done && r.next_u32() % 2 == 0;
                let v = (r.next_u32() % 100) as f32 + 0.5;
                let v = if racing { -v } else if r.next_u32() % 4 == 0 { 0.0 } else { v };
                c.bus.write32(phys + 0x470, v.to_bits());
            }
            let world = DATA + 0x8000;
            c.bus.write32(GAME_GP - RACE_WORLD_GP, world);
            c.bus.write32(world + RACE_WORLD_GAME_MODULE, DATA + 0x9000);
            c.g[A1][0] = sx(ai);
        });
    }

    /// DLHeap_Free on a real dlmalloc heap (`DLHeap_InitArena` on a 256 KiB
    /// arena, 1..8 mallocs of fast, small and large sizes, some of them
    /// freed again first so the freed chunk has free neighbours): null,
    /// fast chunks, merges with the previous / next free chunk or the top
    /// chunk, unsorted-bin insertion, 16 KiB merges (with fast chunks
    /// pending: consolidation; a big top: the real SysTrim), and a fake
    /// mmapped chunk (left alone).
    #[test]
    fn dlheap_free_matches() {
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        check(ee::s10::f_0031ed60__generated, dlheap_free, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_ed60, 0);
            let ra = c.g[RA][0];
            oc259_run(
                c,
                0x0031_ff60,
                ee::s10::DLHeap_InitArena,
                &[HEAP as u64, ARENA as u64, 0x4_0000],
            );
            let n = 1 + r.next_u32() % 8;
            let mut ptrs = Vec::new();
            for _ in 0..n {
                let size = match r.next_u32() % 4 {
                    0 => r.next_u32() % 0x40,
                    1 => 0x40 + r.next_u32() % 0x800,
                    2 => 0x1000 + r.next_u32() % 0x3000,
                    _ => 0x3000 + r.next_u32() % 0x6000,
                };
                let p = oc259_run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, size as u64]);
                ptrs.push(p);
            }
            let target = (r.next_u32() % n) as usize;
            for (i, &p) in ptrs.iter().enumerate() {
                if i != target && p != 0 && r.next_u32() % 3 != 0 {
                    oc259_run(c, 0x0031_ed60, ee::s10::f_0031ed60__generated, &[HEAP as u64, p]);
                }
            }
            c.g[RA][0] = ra;
            c.pc = 0x0031_ed60;
            c.g[A0][0] = sx(HEAP);
            c.g[A1][0] = match seed % 12 {
                0 => 0,
                1 => {
                    // A fake mmapped chunk (size 0x100, mmapped bit).
                    c.bus.write32(DATA + 0x1004, 0x102);
                    sx(DATA + 0x1008)
                }
                _ => ptrs[target],
            };
        });
    }

    /// ScriptMovie_Update: no player, not started, done, or running (the
    /// real VidEnginePlayer_Update takes its short paths: already playing,
    /// then done or a pending request).
    #[test]
    fn script_movie_update_matches() {
        check(ee::s07::f_00283aa0__generated, script_movie_update, 200, &|c, seed| {
            let mut r = Rng(0x0028_3aa0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0028_3aa0;
            let movie = DATA;
            let player = DATA + 0x1000;
            let pick = |r: &mut Rng, zero: bool| if zero { 0 } else { r.next_u32() | 1 };
            c.bus.write32(movie + SCRIPT_MOVIE_PLAYER, if seed % 5 == 0 { 0 } else { player });
            let v = pick(&mut r, seed % 7 == 1);
            c.bus.write32(movie + SCRIPT_MOVIE_STARTED, v);
            let v = r.next_u32();
            c.bus.write32(movie + SCRIPT_MOVIE_UPDATED, v);
            let v = pick(&mut r, seed % 3 != 2);
            c.bus.write32(movie + SCRIPT_MOVIE_DONE, v);
            // The video player: already playing (+0x40), then finished
            // (+0xc) or with a pending request (+0).
            c.bus.write32(player + 0x40, 1);
            let done = seed % 2 == 0;
            c.bus.write32(player + 0xc, if done { 1 } else { 0 });
            c.bus.write32(player, if done { 0 } else { r.next_u32() | 1 });
            c.g[A0][0] = sx(movie);
        });
    }

    /// SpatialHash_Remove on a real octree path: the root key one to four
    /// levels above the item (or at it), nodes from the real main heap so
    /// the real `__builtin_delete` frees emptied ones; every node on the
    /// path randomly kept alive by another child or item; the link first,
    /// last, middle or alone in its list. Some seeds resume at the scan's
    /// event-check heads (0x328614 / 0x328620) inside a frame.
    #[test]
    fn spatial_hash_remove_matches() {
        const MAIN_HEAP: u32 = 0x004f_f250;
        const ROOT_KEY: u32 = DATA + 0x100;
        const ITEM_KEY: u32 = DATA + 0x110;
        const LINK: u32 = DATA + 0x200;
        const ROOT: u32 = DATA + 0x1000;
        check(ee::s10::f_003284b8__generated, spatial_hash_remove, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_84b8, 0);
            let ra = c.g[RA][0];
            // __builtin_delete -> MEM_FreeInternal: no pools, the main heap.
            c.bus.write32(GAME_GP + 0xdd8, 0);
            for slot in [0x2a60u32, 0x2a68, 0x2a6c, 0x2a70, 0x2a78, 0x2a7c, 0x2a84] {
                c.bus.write32(GAME_GP + slot, 0);
            }
            oc259_run(
                c,
                0x0031_ff60,
                ee::s10::DLHeap_InitArena,
                &[MAIN_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
            );
            let new_node = |c: &mut Cpu, r: &mut Rng| -> u32 {
                let n = oc259_run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[MAIN_HEAP as u64, 0x2c]) as u32;
                for i in 0..11 {
                    c.bus.write32(n + 4 * i, 0);
                }
                match r.next_u32() % 3 {
                    0 => c.bus.write32(n + 4 * (r.next_u32() % 8), r.next_u32() | 1),
                    1 => c.bus.write32(n + SH_NODE_ITEMS + 4 * (r.next_u32() % 3), r.next_u32() | 1),
                    _ => {}
                }
                n
            };
            c.g[RA][0] = ra;

            if seed % 10 == 9 {
                // Resume inside a frame at a scan head.
                let child = new_node(c, &mut r);
                let frame_sp = crate::difftest::SP - 0x30;
                c.g[SP][0] = sx(frame_sp);
                c.bus.write64(frame_sp + 0x10, crate::difftest::RA as u64);
                let parent_slot = ROOT + 4 * (r.next_u32() % 8);
                c.bus.write32(parent_slot, child);
                c.g[S0][0] = sx(parent_slot);
                let j = r.next_u32() % 3;
                c.g[A0][0] = j as u64;
                c.g[V1][0] = sx(child + SH_NODE_ITEMS + 4 * j);
                c.pc = if seed % 20 == 9 { SH_REMOVE_SCAN_ITEMS } else { SH_REMOVE_NOT_EMPTY };
                return;
            }

            let top = 1 + r.next_u32() % 4;
            let item_level = r.next_u32() % (top + 1);
            let span = 1u32 << (top - item_level);
            let xyz = [r.next_u32() % span, r.next_u32() % span, r.next_u32() % span];
            put_words(c, ITEM_KEY, &[item_level, xyz[0], xyz[1], xyz[2]]);
            put_words(c, ROOT_KEY, &[top, 0, 0, 0]);
            for i in 0..11 {
                c.bus.write32(ROOT + 4 * i, 0);
            }
            // The path from the root down to the item's level.
            let mut node = ROOT;
            let mut k = [0u32; 3];
            for lev in (item_level + 1..=top).rev() {
                let shift = lev - item_level - 1;
                let b: Vec<u32> = (0..3).map(|i| (xyz[i] >> shift) - 2 * k[i]).collect();
                let child = new_node(c, &mut r);
                c.bus.write32(node + (b[0] << 4) + (b[1] << 3) + (b[2] << 2), child);
                for i in 0..3 {
                    k[i] = 2 * k[i] + b[i];
                }
                node = child;
            }
            let slot = r.next_u32() % 3;
            let next = if r.next_u32() % 2 == 0 { DATA + 0x210 } else { 0 };
            let prev = if r.next_u32() % 2 == 0 { DATA + 0x220 } else { 0 };
            put_words(c, LINK, &[next, prev]);
            put_words(c, DATA + 0x210, &[r.next_u32(), r.next_u32()]);
            put_words(c, DATA + 0x220, &[r.next_u32(), r.next_u32()]);
            if prev == 0 {
                c.bus.write32(node + SH_NODE_ITEMS + 4 * slot, LINK);
            }
            c.pc = SH_REMOVE_ENTRY;
            c.g[A0][0] = sx(ROOT);
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = sx(LINK);
            c.g[A3][0] = sx(ITEM_KEY);
            c.g[T0][0] = sx(ROOT_KEY);
        });
    }

    /// A world painter view `i` (0..3) whose interface holder at
    /// `holder_off` points at an object with a fake virtual method at
    /// `vslot` (it returns a pointer to the float; some calls unwind).
    fn wp_view_fixture(c: &mut Cpu, seed: u32, salt: u64, holder_off: u32, vslot: u32) {
        let mut r = machine(c, seed, salt, 1);
        let i = r.next_u32() % 3;
        let view = 0x004f_a370 + i * 0xf0;
        let holder = DATA + 0x400 + i * 0x100;
        let obj = DATA + 0x800 + i * 0x100;
        let vt = crate::difftest::fakes::vtable(c, &mut r, &[vslot]);
        c.bus.write32(obj + WP_OBJ_VTABLE, vt);
        c.bus.write32(holder, obj);
        c.bus.write32(view + holder_off, holder);
        c.g[A0][0] = sx(i);
    }

    /// WorldPainter_GetLightGlow08: views 0..2 through a fake method.
    #[test]
    fn world_painter_get_light_glow08_matches() {
        crate::difftest::fakes::covered(ee::s09::f_002eedb0__generated, world_painter_get_light_glow08, 120, &|c, seed| {
            wp_view_fixture(c, seed, 0x2e_edb0, WP_VIEW_LIGHT_GLOW, 0x68);
            c.pc = 0x002e_edb0;
        });
    }

    /// WorldPainter_GetWeather50: views 0..2 through a fake method.
    #[test]
    fn world_painter_get_weather50_matches() {
        crate::difftest::fakes::covered(ee::s09::f_002ee660__generated, world_painter_get_weather50, 120, &|c, seed| {
            wp_view_fixture(c, seed, 0x2e_e660, WP_VIEW_WEATHER, 0x1a0);
            c.pc = 0x002e_e660;
        });
    }

    /// RiderWipeoutSub_UpdateAnimSpeed: random rider velocities and
    /// sequence speeds (rates below 0.5, inside, above 2, edge floats);
    /// the layer playing animation 0x1d, 0x19, another id or the "none"
    /// id 0x1b6 (the real AnimController getters and the anim table); the
    /// tumble timer below or at least 1; progress negative or not.
    #[test]
    fn rider_wipeout_sub_update_anim_speed_matches() {
        check(ee::s00::f_0012e528__generated, rider_wipeout_sub_update_anim_speed, 300, &|c, seed| {
            let mut r = Rng(0x0012_e528 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0012_e528;
            let (this, rider, ctrl, motion) = (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
            let (lists, seq) = (DATA + 0x4000, DATA + 0x5000);
            c.bus.write32(GAME_GP - WIPEOUT_INV_TWO_PI_GP, 0.159_154_94f32.to_bits());
            c.bus.write32(this + WIPEOUT_RIDER, rider);
            let timer = if seed % 3 == 0 { 1.0 + (r.next_u32() % 4) as f32 } else { (r.next_u32() % 100) as f32 / 100.0 };
            c.bus.write32(this + WIPEOUT_TIMER, timer.to_bits());
            c.bus.write32(rider + WIPEOUT_RIDER_ANIM, ctrl);
            c.bus.write32(rider + WIPEOUT_RIDER_MOTION, motion);
            let progress = r.f32(500.0);
            c.bus.write32(rider + WIPEOUT_RIDER_PROGRESS, progress);
            for i in 0..4 {
                let v = if seed % 11 == 5 { r.edge_f32() } else { r.f32(40.0) };
                c.bus.write32(motion + WIPEOUT_MOTION_VELOCITY + 4 * i, v);
            }
            // Layer 2's sequence list (+0x50 + 2 * 8) and its first sequence.
            c.bus.write32(ctrl + 0x50, lists);
            c.bus.write32(lists + 0x10 + 4, seq);
            let speed = if seed % 7 == 3 { r.edge_f32() } else { r.f32(1.0) };
            c.bus.write32(seq + WIPEOUT_SEQ_SPEED, speed);
            c.bus.write32(seq + WIPEOUT_SEQ_SCALE, r.next_u32());
            // Layer 2's animation index and its anim table entry.
            let index = if seed % 9 == 8 { 0x1b6 } else { 1 + r.next_u32() % 0x100 };
            c.bus.write32(ctrl + 8, index);
            let id = [0x1d, 0x19, 0x1c, r.next_u32() % 0x40][(seed % 4) as usize];
            c.bus.write32(0x0044_6990 + index * 0x1c, id);
            c.g[A0][0] = sx(this);
        });
    }

    /// InSeq_UpdateAll: an empty list or one to five nodes whose instances
    /// take the real InSeq_InstanceUpdate's short path (no restart pending,
    /// no queued ticks) into InSeq_InstanceTick's early returns (idle, or
    /// waiting on a delay).
    #[test]
    fn in_seq_update_all_matches() {
        check(ee::s07::f_002728f8__generated, in_seq_update_all, 200, &|c, seed| {
            let mut r = Rng(0x0027_28f8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0027_28f8;
            let seq = DATA;
            let n = if seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 5 };
            let mut next = 0u32;
            for i in (0..n).rev() {
                let node = DATA + 0x1000 + 0x100 * i;
                let inst = node + INSEQ_NODE_INSTANCE;
                c.bus.write32(node + INSEQ_NODE_NEXT, next);
                c.bus.write32(inst + 0x34, 0xffff_ffff);
                c.bus.write32(inst + 0x38, 0);
                let idle = r.next_u32() % 2 == 0;
                c.bus.write32(inst + 0x20, if idle { 0 } else { 1 + r.next_u32() % 4 });
                c.bus.write32(inst + 0x2c, 1 + r.next_u32() % 100);
                next = node;
            }
            c.bus.write32(seq + INSEQ_FIRST, next);
            c.g[A0][0] = sx(seq);
        });
    }

    /// StreamFile_ConsumeBytes: the count already below the low-water mark,
    /// staying above it, or crossing it (streaming mode or not: the real
    /// EAFile_SetRequestPriority with a null or unknown handle); the real
    /// EAMutex lock / unlock (syscalls faked), owned by the caller or not.
    #[test]
    fn stream_file_consume_bytes_matches() {
        check(ee::s14::f_003dfc48__generated, stream_file_consume_bytes, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x003d_fc48, 0);
            c.pc = 0x003d_fc48;
            let st = DATA;
            let low = r.next_u32() % 0x10000;
            let old = match seed % 3 {
                0 => low.wrapping_sub(1 + r.next_u32() % 0x100),
                _ => low + r.next_u32() % 0x1000,
            };
            let n = match seed % 4 {
                0 => r.next_u32() % 0x10,
                1 => r.next_u32() % 0x2000,
                2 => old.wrapping_sub(low) + 1 + r.next_u32() % 4,
                _ => r.next_u32(),
            };
            c.bus.write32(st + STREAM_FILE_BUFFERED, old);
            c.bus.write32(st + STREAM_FILE_LOW_WATER, low);
            c.bus.write32(st + STREAM_FILE_NEEDS_REFILL, r.next_u32());
            c.bus.write32(st + STREAM_FILE_MODE, if seed % 2 == 0 { 1 } else { r.next_u32() % 3 });
            c.bus.write32(st + STREAM_FILE_PRIORITY, r.next_u32() % 8);
            c.bus.write32(st + STREAM_FILE_HANDLE, if seed % 5 == 0 { 0 } else { r.next_u32() });
            // The mutex: owner thread id (+4), lock count (+8), semaphore (+0xc).
            let m = st + STREAM_FILE_MUTEX;
            put_words(c, m + 4, &[r.next_u32() % 4, r.next_u32() % 3, r.next_u32() % 16]);
            c.g[A0][0] = sx(st);
            c.g[A1][0] = sx(n);
        });
    }
}
