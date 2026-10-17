//! Work package D8: memory, containers, replay & script
//! (`docs/investigation/phase2-candidates-2.md`).
//!
//! - **VRAM heap** (`VramHeap_*`, the texture manager's GS memory allocator):
//!   a fixed array of 0x123 block nodes ([`VramNode`]) linked twice: in
//!   address order (`prev`/`next`, a doubly linked list over the whole VRAM
//!   range) and into one of 17 size-class lists per state (free / used),
//!   whose heads are pseudo-nodes inside the heap struct ([`VramHeap`]).
//!   Size-list links are either a node index (>= 0) or a tagged reference to
//!   a list head (`0x9000_0000 | class` for the used lists, `0xa000_0000 |
//!   class` for the free lists); `-1` ends a list. Unused nodes form the
//!   spare chain through `next`.
//! - **Main memory** (`EA_MemFree`, `MEM_*`): the allocator entry points
//!   around the small-block pools and the dlmalloc arena at 0x4ff250, under
//!   the allocator's recursive [`EAMutex`].
//! - **Replay** and **world script** leaves: the replay stop/cancel flags,
//!   the replay stream write cursor and the script event ring.
//!
//! Replacements follow the call protocol of [`crate::call`] and the loop
//! rules of [`crate::events`]; each charges the EE cycles block by block as
//! the generated code does and leaves the registers (including LO/HI after
//! the node-index `mult`s) the original leaves.

use crate::abi::{self, addu};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{Resume, back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_fa50, replay_stop_recording, "Replay_StopRecording"),
    (0x0027_0628, replay_cancel_instant_replay, "Replay_CancelInstantReplay"),
    (0x0026_d740, replay_add_snapshot, "Replay_AddSnapshot"),
    (0x0026_e9d0, replay_stream_write, "ReplayStream_Write"),
    (0x0030_c820, wscript_event_queue_pop, "WScriptMan_EventQueuePop"),
    (0x0030_c870, wscript_event_queue_peek, "WScriptMan_EventQueuePeek"),
    (0x003e_6220, ea_mem_zero, "EA_MemZero"),
    (0x0025_23a8, mem_free, "MEM_Free"),
    (0x0031_7e98, ea_mem_free, "EA_MemFree"),
    (0x0031_9b48, mem_free_internal, "MEM_FreeInternal"),
    (0x0031_9a90, mem_alloc_internal, "MEM_AllocInternal"),
    (0x003e_5700, ea_mutex_lock, "EAMutex_Lock"),
    (0x003e_5760, ea_mutex_unlock, "EAMutex_Unlock"),
    (0x0036_5fe8, vram_heap_pop_spare_node, "VramHeap_PopSpareNode"),
    (0x0036_6008, vram_heap_remove_node, "VramHeap_RemoveNode"),
    (0x0036_6238, vram_heap_unlink_size_list, "VramHeap_UnlinkSizeList"),
    (0x0036_60a8, vram_heap_link_free_block, "VramHeap_LinkFreeBlock"),
    (0x0036_67f8, vram_heap_find_free_block, "VramHeap_FindFreeBlock"),
    (0x0036_62d0, vram_heap_split_block, "VramHeap_SplitBlock"),
    (0x0036_63d8, vram_heap_free_block, "VramHeap_FreeBlock"),
    (0x0036_68f8, vram_heap_evict_cheapest_block, "VramHeap_EvictCheapestBlock"),
    (0x0036_6ce0, vram_heap_alloc, "VramHeap_Alloc"),
];

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `sw` through the CPU store path.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v)
}

/// The low 32 bits of a register.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// Direct-call targets (the generated function, through its override slot).
mod callee {
    use super::{ee, Callee};

    pub const SNAPSHOT_LIST_PUSH_BACK: Callee = Callee { addr: 0x0026_e5d0, f: ee::s07::SnapshotList_PushBack };
    pub const EA_MEM_SET: Callee = Callee { addr: 0x003e_6448, f: ee::s14::EA_MemSet };
    pub const MEM_FREE_INTERNAL: Callee = Callee { addr: 0x0031_9b48, f: ee::s10::MEM_FreeInternal };
    pub const MEM_IS_POOL_POINTER: Callee = Callee { addr: 0x0031_a088, f: ee::s10::MEM_IsPoolPointer };
    pub const MEM_POOL_FREE: Callee = Callee { addr: 0x0031_9f68, f: ee::s10::MEM_PoolFree };
    pub const MEM_HEAP_FREE: Callee = Callee { addr: 0x0032_00c0, f: ee::s10::MEM_HeapFree };
    pub const MEM_POOL_ALLOC: Callee = Callee { addr: 0x0031_9e48, f: ee::s10::MEM_PoolAlloc };
    pub const MEM_HEAP_MEMALIGN_OFFSET: Callee =
        Callee { addr: 0x0032_0078, f: ee::s10::MEM_HeapMemalignOffset };
    pub const MEM_HEAP_MEMALIGN: Callee = Callee { addr: 0x0032_0058, f: ee::s10::MEM_HeapMemalign };
}

/// Cross-shard direct calls the generator emitted as `call_indirect` (they
/// go through the function table like the original).
mod table_callee {
    pub const EA_MEM_COPY: u32 = 0x003e_6574;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const EA_MEM_ZERO: u32 = 0x003e_6220;
    pub const EA_MUTEX_LOCK: u32 = 0x003e_5700;
    pub const EA_MUTEX_UNLOCK: u32 = 0x003e_5760;
    /// Kernel call stubs.
    pub const GET_THREAD_ID: u32 = 0x0042_3c90;
    pub const WAIT_SEMA: u32 = 0x0042_3de0;
    pub const SIGNAL_SEMA: u32 = 0x0042_3dc0;
    pub const I_SIGNAL_SEMA: u32 = 0x0042_3dd0;
}

// ---------------------------------------------------------------------------
// Replay.

/// `Replay` fields used here.
pub mod replay {
    /// Replay state (0xe, 0xf: playing back).
    pub const STATE: u32 = 0x0;
    /// The free/used snapshot list (`state::SnapshotList`) at +0x3b0.
    pub const SNAPSHOTS: u32 = 0x3b0;
    /// Non-zero while an instant replay is pending.
    pub const INSTANT_REPLAY: u32 = 0x3d4;
    /// Set to 1 to stop recording.
    pub const STOP_RECORDING: u32 = 0x60c;
    /// gp offset of the game module pointer.
    pub const GAME_MODULE_GP: u32 = 0x848;
}

/// `int Replay_StopRecording(Replay* r)` (0x26fa50): unless the replay is
/// playing back (state 0xe or 0xf), sets the stop flag (+0x60c). Returns 1.
/// 6 (+1 when stopping) + 2 cycles.
pub fn replay_stop_recording(c: &mut Cpu) {
    let rp = r(c, A0);
    c.cycles += 6;
    let state = sx((lw(c, rp + replay::STATE) as u32).wrapping_sub(0xe));
    let playing = state < 2;
    c.g[V0][0] = 1;
    if !playing {
        c.cycles += 1;
        sw(c, rp.wrapping_add(replay::STOP_RECORDING), 1);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_CancelInstantReplay(Replay* r)` (0x270628): a pending
/// instant replay (+0x3d4) is cancelled when the game module's world says
/// so (`gameModule->+0x84->+0xc->+0x7c == 1`). 4 (+6) + 2 cycles.
pub fn replay_cancel_instant_replay(c: &mut Cpu) {
    let rp = r(c, A0);
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    let pending = lw(c, rp.wrapping_add(replay::INSTANT_REPLAY));
    c.g[A2][0] = pending;
    let module = lw(c, r(c, GP).wrapping_sub(replay::GAME_MODULE_GP));
    c.g[A0][0] = module;
    if pending != 0 {
        c.cycles += 6;
        c.g[A1][0] = 1;
        let a = lw(c, (module as u32).wrapping_add(0x84));
        c.g[V0][0] = a;
        let b = lw(c, (a as u32).wrapping_add(0xc));
        c.g[V1][0] = b;
        let flag = lw(c, (b as u32).wrapping_add(0x7c));
        c.g[A0][0] = flag;
        if flag == 1 {
            sw(c, rp.wrapping_add(replay::INSTANT_REPLAY), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_AddSnapshot(Replay* r, Snapshot* s)` (0x26d740): appends `s`
/// to the snapshot list at +0x3b0. 7 cycles plus the callee.
pub fn replay_add_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = addu(c.g[A0][0], replay::SNAPSHOTS as u64);
        callee::SNAPSHOT_LIST_PUSH_BACK.call(c, 0x0026_d750)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ReplayStream_Write(ReplayStream* s, const void* src, int n)`
/// (0x26e9d0): copies `n` bytes to the write cursor (+0x8) and advances it
/// and the byte count (+0x10). 19 cycles plus the copy.
pub fn replay_stream_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let s = c.g[A0][0];
        c.g[S0][0] = s;
        frame.save_ra(c, 0);
        let n = c.g[A2][0];
        c.g[S1][0] = n;
        let s = s as u32;
        c.g[A0][0] = lw(c, s + 0x8);
        call_ptr(c, table_callee::EA_MEM_COPY, 0x0026_e9f0)?;
        let cursor = addu(lw(c, s + 0x8), n);
        c.g[V0][0] = cursor;
        let count = addu(lw(c, s + 0x10), n);
        frame.restore_ra(c, 0);
        c.g[V1][0] = count;
        sw(c, s + 0x8, cursor);
        sw(c, s + 0x10, count);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World script event ring.

/// The world script manager's event ring: `{head +0, tail +4, events[16]
/// +8}` of 12-byte events. Empty when `head == tail`.
#[derive(Clone, Copy, Debug)]
pub struct EventRing(pub Addr);

impl EventRing {
    pub const HEAD: u32 = 0x0;
    pub const TAIL: u32 = 0x4;
    pub const EVENTS: u32 = 0x8;
    pub const EVENT_SIZE: u32 = 0xc;
    pub const CAPACITY: u32 = 16;
}

/// `Event* WScriptMan_EventQueuePop(EventRing* q)` (0x30c820): the oldest
/// event (head advanced modulo 16), or null when empty. 5 + 2 or 5 + 13
/// cycles; the `mult` leaves LO/HI.
pub fn wscript_event_queue_pop(c: &mut Cpu) {
    let q = r(c, A0);
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    let head = lw(c, q + EventRing::HEAD);
    c.g[V1][0] = head;
    let tail = lw(c, q + EventRing::TAIL);
    c.g[V0][0] = EventRing::EVENT_SIZE as u64;
    if head == tail {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 13;
    let next = sx((head as u32).wrapping_add(1));
    c.g[A1][0] = next;
    let off = abi::mult(c, head as u32, EventRing::EVENT_SIZE);
    // `(head + 1) % 16` as gcc computes a signed remainder.
    let non_negative = (next as i64) > -1;
    c.g[A0][0] = non_negative as u64;
    let biased = if non_negative { next } else { sx((head as u32).wrapping_add(0x10)) };
    let floor = sx((((biased as u32) as i32 >> 4) as u32) << 4);
    c.g[V1][0] = floor;
    let wrapped = sx((next as u32).wrapping_sub(floor as u32));
    c.g[A1][0] = wrapped;
    c.g[V0][0] = sx(q.wrapping_add(off.wrapping_add(EventRing::EVENTS)));
    sw(c, q + EventRing::HEAD, wrapped);
    abi::ret(c);
}

/// `Event* WScriptMan_EventQueuePeek(EventRing* q)` (0x30c870): the oldest
/// event, or null when empty. 4 + 2 or 4 + 4 cycles.
pub fn wscript_event_queue_peek(c: &mut Cpu) {
    let q = r(c, A0);
    c.cycles += 4;
    let head = lw(c, q + EventRing::HEAD);
    c.g[V1][0] = head;
    let tail = lw(c, q + EventRing::TAIL);
    c.g[V0][0] = EventRing::EVENT_SIZE as u64;
    if head == tail {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 4;
        let off = abi::mult(c, head as u32, EventRing::EVENT_SIZE);
        c.g[V0][0] = sx(q.wrapping_add(off.wrapping_add(EventRing::EVENTS)));
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Main memory.

/// The allocator's mutex.
const MEM_MUTEX: u32 = 0x0053_8b00;
/// The dlmalloc arena of the main heap.
const MEM_HEAP: u32 = 0x004f_f250;

/// `void EA_MemZero(void* p, int n)` (0x3e6220): `EA_MemSet(p, 0, n)`.
/// 8 cycles plus the callee.
pub fn ea_mem_zero(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        callee::EA_MEM_SET.call(c, 0x003e_6234)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MEM_Free(void* p)` (0x2523a8): `EA_MemFree(p)` unless null; returns
/// 1. 3 (+2) + 4 cycles.
pub fn mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table_callee::EA_MEM_FREE, 0x0025_23bc)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void EA_MemFree(void* p)` (0x317e98): `MEM_FreeInternal(p)` under the
/// allocator mutex. 18 cycles plus the callees.
pub fn ea_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = MEM_MUTEX as u64;
        let block = c.g[A0][0];
        c.g[S1][0] = block;
        frame.save_ra(c, 0);
        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_LOCK, 0x0031_7ebc)?;
        c.g[A0][0] = block;
        callee::MEM_FREE_INTERNAL.call(c, 0x0031_7ec4)?;
        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_UNLOCK, 0x0031_7ecc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_FreeInternal(void* p)` (0x319b48): null is ignored; pool
/// blocks go back to their pool (`MEM_PoolFree`), others to the heap
/// (`MEM_HeapFree(&heap, p)`).
pub fn mem_free_internal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let p = c.g[A0][0];
        c.g[S0][0] = p;
        frame.save_ra(c, 0);
        if p != 0 {
            c.cycles += 4;
            callee::MEM_IS_POOL_POINTER.call(c, 0x0031_9b64)?;
            let pooled = c.g[V0][0] != 0;
            c.g[A0][0] = 0x0050_0000;
            if pooled {
                c.cycles += 4;
                c.g[A0][0] = p;
                callee::MEM_POOL_FREE.call(c, 0x0031_9b74)?;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[A1][0] = p;
            c.g[A0][0] = MEM_HEAP as u64;
            callee::MEM_HEAP_FREE.call(c, 0x0031_9b88)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MEM_AllocInternal(u32 size, u32 align, int flags, u32 offset)`
/// (0x319a90): small blocks (`0 < size <= 0x200`, no offset) come from the
/// pools; others from the heap with at least 16-byte alignment
/// (`MEM_HeapMemalignOffset` when an offset is given). A negative `flags`
/// asks for zeroed memory (`EA_MemZero`). Returns the block.
pub fn mem_alloc_internal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let size = c.g[A0][0];
        c.g[S1][0] = size;
        frame.save(c, S0, 0x30);
        let flags = c.g[A2][0];
        c.g[S2][0] = flags;
        frame.save_ra(c, 0);
        let offset = c.g[A3][0];

        let mut pooled = false;
        if offset == 0 {
            c.cycles += 2;
            c.g[V0][0] = (size < 0x201) as u64;
            if size != 0 {
                c.cycles += 2;
                let small = size < 0x201;
                c.g[V0][0] = 0xf;
                if small {
                    c.cycles += 4;
                    callee::MEM_POOL_ALLOC.call(c, 0x0031_9ac8)?;
                    pooled = true;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0xf;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0xf;
        }
        if !pooled {
            // align = max(align, 16)
            c.cycles += 4;
            c.g[V1][0] = 0x10;
            let above = 0xf < c.g[A1][0];
            c.g[V0][0] = above as u64;
            if !above {
                c.g[A1][0] = 0x10;
            }
            c.g[A0][0] = MEM_HEAP as u64;
            c.g[A2][0] = size;
            if offset != 0 {
                c.cycles += 6;
                callee::MEM_HEAP_MEMALIGN_OFFSET.call(c, 0x0031_9af4)?;
            } else {
                c.cycles += 5;
                callee::MEM_HEAP_MEMALIGN.call(c, 0x0031_9b0c)?;
            }
        }
        let block = c.g[V0][0];
        c.g[S0][0] = block;

        c.cycles += 2;
        if block != 0 && (flags as i64) >= 0 {
            c.cycles += 2;
            frame.restore(c, S1, 0x20);
        } else {
            if block != 0 {
                c.cycles += 2 + 4;
                c.g[A1][0] = size;
                c.g[A0][0] = block;
                call_ptr(c, table_callee::EA_MEM_ZERO, 0x0031_9b2c)?;
            }
            c.g[V0][0] = block;
            c.cycles += 1;
            frame.restore(c, S1, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAMutex.

/// A recursive mutex over a kernel semaphore: `{+4 owner thread, +8 depth,
/// +0xc semaphore id}`.
#[derive(Clone, Copy, Debug)]
pub struct EAMutex(pub Addr);

impl EAMutex {
    pub const OWNER: u32 = 0x4;
    pub const DEPTH: u32 = 0x8;
    pub const SEMA: u32 = 0xc;
}

/// The kernel's "in interrupt handler" flag (`iSignalSema` must be used).
const IN_INTERRUPT: u32 = 0x0045_0de4;

/// `void EAMutex_Lock(EAMutex* m)` (0x3e5700): re-entry by the owner bumps
/// the depth; otherwise waits on the semaphore, then takes ownership.
pub fn ea_mutex_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let m = c.g[A0][0];
        c.g[S0][0] = m;
        let m = m as u32;
        call_ptr(c, table_callee::GET_THREAD_ID, 0x003e_5714)?;
        let owner = lw(c, m + EAMutex::OWNER);
        c.g[V1][0] = owner;
        if owner == c.g[V0][0] {
            c.cycles += 4;
            let depth = sx((lw(c, m + EAMutex::DEPTH) as u32).wrapping_add(1));
            c.g[V0][0] = depth;
            sw(c, m + EAMutex::DEPTH, depth);
        } else {
            c.cycles += 8;
            c.g[A0][0] = lw(c, m + EAMutex::SEMA);
            call_ptr(c, table_callee::WAIT_SEMA, 0x003e_5738)?;
            call_ptr(c, table_callee::GET_THREAD_ID, 0x003e_5740)?;
            let depth = lw(c, m + EAMutex::DEPTH);
            sw(c, m + EAMutex::OWNER, c.g[V0][0]);
            let depth = sx((depth as u32).wrapping_add(1));
            c.g[V1][0] = depth;
            sw(c, m + EAMutex::DEPTH, depth);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAMutex_Unlock(EAMutex* m)` (0x3e5760): the owner drops one level;
/// at depth 0 (or when called by another thread) the owner is cleared /
/// left and the semaphore signalled (`iSignalSema` inside an interrupt
/// handler).
pub fn ea_mutex_unlock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let m = c.g[A0][0];
        c.g[S0][0] = m;
        let m = m as u32;
        call_ptr(c, table_callee::GET_THREAD_ID, 0x003e_5774)?;
        let owner = lw(c, m + EAMutex::OWNER);
        c.g[V1][0] = owner;
        let mine = owner == c.g[V0][0];
        c.g[V0][0] = 0x0045_0000;
        let mut signal = true;
        if mine {
            c.cycles += 4;
            let depth = sx((lw(c, m + EAMutex::DEPTH) as u32).wrapping_sub(1));
            c.g[V0][0] = depth;
            sw(c, m + EAMutex::DEPTH, depth);
            if depth != 0 {
                signal = false;
            } else {
                c.cycles += 2;
                sw(c, m + EAMutex::OWNER, 0);
                c.g[V0][0] = 0x0045_0000;
            }
        }
        if signal {
            c.cycles += 3;
            let in_irq = lw(c, IN_INTERRUPT);
            c.g[V1][0] = in_irq;
            if in_irq != 0 {
                c.cycles += 4;
                c.g[A0][0] = lw(c, m + EAMutex::SEMA);
                call_ptr(c, table_callee::I_SIGNAL_SEMA, 0x003e_57ac)?;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = lw(c, m + EAMutex::SEMA);
            call_ptr(c, table_callee::SIGNAL_SEMA, 0x003e_57bc)?;
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
// VRAM heap.

/// The VRAM heap (`TexMan`'s GS memory allocator). Offsets of its fields.
#[derive(Clone, Copy, Debug)]
pub struct VramHeap(pub Addr);

impl VramHeap {
    /// `VramNode*` node array (0x123 nodes).
    pub const NODES: u32 = 0x1ff0;
    /// 17 used-block size-class list heads (pseudo-nodes, 0x1c bytes each;
    /// only their `size_next` is used). Sorted by address.
    pub const USED_LISTS: u32 = 0x1ff8;
    /// 17 free-block size-class list heads. Sorted by size.
    pub const FREE_LISTS: u32 = 0x21d4;
    /// Head of the spare node chain (through `next`).
    pub const SPARE: u32 = 0x23dc;
    /// `OwnerRecord**` per-owner records (texture slots) whose cached VRAM
    /// addresses are reset when a block is freed.
    pub const OWNERS: u32 = 0x23e8;
    /// Number of size classes.
    pub const CLASSES: u64 = 17;
}

/// A VRAM block node (0x1c bytes).
pub struct VramNode;

impl VramNode {
    pub const SIZE_OF: u32 = 0x1c;
    /// Block address | flags: bit 0 used, bit 1 locked (never evicted),
    /// bit 2 which pair of owner-record fields caches it.
    pub const WORD: u32 = 0x0;
    /// Block size.
    pub const SIZE: u32 = 0x4;
    /// Owner id (0x7fffffff: none; -1 for free blocks).
    pub const OWNER: u32 = 0x8;
    /// Address-order links (node indices, -1 at the ends).
    pub const PREV: u32 = 0xc;
    pub const NEXT: u32 = 0x10;
    /// Size-class list links (node index, tagged list head, or -1).
    pub const SIZE_PREV: u32 = 0x14;
    pub const SIZE_NEXT: u32 = 0x18;
}

/// Tag of a size-list link that names a list head instead of a node.
const TAG_MASK: u64 = 0x7000_0000;
const TAG_USED: u64 = 0x1000_0000;
/// The tagged head references the lists store (sign-extended).
const TAG_USED_REF: u64 = 0xffff_ffff_9000_0000;
const TAG_FREE_REF: u64 = 0xffff_ffff_a000_0000;
/// The class index part of a tagged reference.
const TAG_INDEX: u64 = 0x0fff_ffff;
/// Per-owner frame/weight table for eviction: `{last_used_frame, weight}`.
const VRAM_OWNER_STATS: u32 = 0x0053_ab80;
/// `VramNode::OWNER` of a block without owner.
const NO_OWNER: u64 = 0x7fff_ffff;

/// The size class of a block size as the inlined `plzcw` sequence computes
/// it (`floor(log2(size)) - 6` for positive sizes), sign-extended.
fn size_class(size: u64) -> u64 {
    let x = size as u32;
    let y = if (x as i32) < 0 { !x } else { x };
    let redundant_sign_bits = y.leading_zeros().saturating_sub(1).min(31);
    sx((redundant_sign_bits ^ 0x1f).wrapping_sub(7))
}

/// `sx((class ^ 0x1f) - 1)`: the intermediate the sequence leaves in v0
/// when the class itself goes to another register.
fn size_class_v0(size: u64) -> u64 {
    sx((size_class(size) as u32).wrapping_add(6))
}

/// `mult rd, rs, 0x1c` (node index → byte offset): LO/HI and `rd`.
fn scale(c: &mut Cpu, rd: usize, idx: u64) -> u64 {
    let v = sx(abi::mult(c, idx as u32, VramNode::SIZE_OF));
    c.g[rd][0] = v;
    v
}

/// Resolves a size/address list link to the address of what it names: a
/// node (`scaled` = link * 0x1c, already in a register) or a list head
/// (tagged link). Leaves v0/v1/LO/HI as the inlined sequence does.
fn link_target(c: &mut Cpu, heap: u32, link: u64, nodes: u64, scaled: u64) -> u32 {
    if (link as i64) >= 0 {
        c.cycles += 1;
        let a = addu(scaled, nodes);
        c.g[V1][0] = a;
        return a as u32;
    }
    c.cycles += 5;
    c.g[V1][0] = TAG_USED;
    let used = link & TAG_MASK == TAG_USED;
    c.cycles += 6;
    let scaled = scale(c, V0, link & TAG_INDEX);
    let base = if used { VramHeap::USED_LISTS } else { VramHeap::FREE_LISTS };
    let off = addu(scaled, base as u64);
    c.g[V0][0] = off;
    let a = addu(heap as u64, off);
    c.g[V1][0] = a;
    a as u32
}

/// `int VramHeap_PopSpareNode(VramHeap* h)` (0x365fe8): takes the first
/// node of the spare chain. 8 cycles.
pub fn vram_heap_pop_spare_node(c: &mut Cpu) {
    let h = r(c, A0);
    c.cycles += 8;
    let idx = lw(c, h + VramHeap::SPARE);
    c.g[V0][0] = idx;
    let nodes = lw(c, h + VramHeap::NODES);
    c.g[A2][0] = nodes;
    let scaled = scale(c, V1, idx);
    let node = addu(scaled, nodes);
    c.g[V1][0] = node;
    let next = lw(c, node as u32 + VramNode::NEXT);
    c.g[A1][0] = next;
    sw(c, h + VramHeap::SPARE, next);
    abi::ret(c);
}

/// `void VramHeap_RemoveNode(VramHeap* h, int idx)` (0x366008): unlinks a
/// node from the address-order list and puts it on the spare chain.
pub fn vram_heap_remove_node(c: &mut Cpu) {
    let idx = c.g[A1][0];
    c.cycles += 8;
    c.g[T0][0] = 0x1c;
    let h = r(c, A0);
    c.g[A2][0] = c.g[A0][0];
    scale(c, V0, idx);
    let nodes = lw(c, h + VramHeap::NODES);
    c.g[V1][0] = nodes;
    let node = addu(nodes, c.g[V0][0]) as u32;
    c.g[A3][0] = sx(node);
    let prev = lw(c, node + VramNode::PREV);
    c.g[A0][0] = prev;
    let scaled = scale(c, V0, prev);
    let before = link_target(c, h, prev, nodes, scaled);

    c.cycles += 5;
    let next = lw(c, node + VramNode::NEXT);
    c.g[V0][0] = next;
    sw(c, before + VramNode::NEXT, next);
    let next = lw(c, node + VramNode::NEXT);
    c.g[V1][0] = next;
    c.g[V0][0] = 0x1c;
    if (next as i64) >= 0 {
        c.cycles += 5;
        let nodes = lw(c, h + VramHeap::NODES);
        c.g[A0][0] = nodes;
        let scaled = scale(c, V0, next);
        let prev = lw(c, node + VramNode::PREV);
        c.g[V1][0] = prev;
        let after = addu(scaled, nodes);
        c.g[V0][0] = after;
        sw(c, after as u32 + VramNode::PREV, prev);
    }
    c.cycles += 4;
    let spare = lw(c, h + VramHeap::SPARE);
    c.g[V0][0] = spare;
    sw(c, node + VramNode::NEXT, spare);
    sw(c, h + VramHeap::SPARE, idx);
    abi::ret(c);
}

/// `void VramHeap_UnlinkSizeList(VramHeap* h, int idx)` (0x366238): unlinks
/// a node from its size-class list.
pub fn vram_heap_unlink_size_list(c: &mut Cpu) {
    let h = r(c, A0);
    c.cycles += 7;
    c.g[A3][0] = 0x1c;
    let nodes = lw(c, h + VramHeap::NODES);
    c.g[V0][0] = nodes;
    let scaled = scale(c, A1, c.g[A1][0]);
    let node = addu(nodes, scaled) as u32;
    c.g[A2][0] = sx(node);
    let prev = lw(c, node + VramNode::SIZE_PREV);
    c.g[A1][0] = prev;
    let scaled = scale(c, T0, prev);
    let before = link_target(c, h, prev, nodes, scaled);

    c.cycles += 5;
    let next = lw(c, node + VramNode::SIZE_NEXT);
    c.g[V0][0] = next;
    sw(c, before + VramNode::SIZE_NEXT, next);
    let next = lw(c, node + VramNode::SIZE_NEXT);
    c.g[V1][0] = next;
    c.g[V0][0] = 0x1c;
    if (next as i64) >= 0 {
        c.cycles += 5;
        let nodes = lw(c, h + VramHeap::NODES);
        c.g[A0][0] = nodes;
        let scaled = scale(c, V0, next);
        let prev = lw(c, node + VramNode::SIZE_PREV);
        c.g[V1][0] = prev;
        let after = addu(scaled, nodes);
        c.g[V0][0] = after;
        sw(c, after as u32 + VramNode::SIZE_PREV, prev);
    }
    c.cycles += 2;
    abi::ret(c);
}

const LINK_FREE_BLOCK: u32 = 0x0036_60a8;
const LINK_FREE_BLOCK_END: u32 = 0x0036_6234;
/// Resume points of `VramHeap_LinkFreeBlock` (event checks).
const LINK_APPEND: u32 = 0x0036_6130;
const LINK_SCAN: u32 = 0x0036_6168;

#[derive(Clone, Copy, PartialEq, Debug)]
enum LinkStage {
    /// Walk the class list from the node scaled in v1 (head 0x366168).
    Scan,
    /// Append after the last node (0x366130).
    Append,
    /// Insert before the current node; v1 holds its `size_prev` link.
    InsertBefore,
    /// Restore s0 and return.
    Exit,
}

/// `void VramHeap_LinkFreeBlock(VramHeap* h, int idx)` (0x3660a8): links a
/// node into the size-class list of its state (used lists sorted by
/// address, free lists by size).
///
/// A `match pc` dispatcher in the original: the list walk checks events at
/// 0x366168 (next node) and 0x366130 (append), which are entry points too.
/// Registers: t0 heap, t1 node, a1 idx, a3 current index, v1 its offset,
/// t4 the class's table (`USED_LISTS`/`FREE_LISTS` base), t7 the size.
pub fn vram_heap_link_free_block(c: &mut Cpu) {
    let mut stage = match c.pc {
        LINK_FREE_BLOCK => match link_entry(c) {
            Some(s) => s,
            None => return,
        },
        LINK_SCAN => LinkStage::Scan,
        LINK_APPEND => LinkStage::Append,
        pc => {
            c.jump_out(pc, LINK_FREE_BLOCK, LINK_FREE_BLOCK_END);
            return;
        }
    };
    loop {
        let flow = match stage {
            LinkStage::Scan => link_scan(c),
            LinkStage::Append => {
                c.cycles += 3;
                let node = r(c, T1);
                sw(c, node + VramNode::SIZE_NEXT, c.g[V0][0]);
                sw(c, node + VramNode::SIZE_PREV, c.g[A3][0]);
                Resume::Go(LinkStage::Exit)
            }
            LinkStage::InsertBefore => link_insert_before(c),
            LinkStage::Exit => {
                c.cycles += 3;
                c.lq(S0, r(c, SP));
                c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
                abi::ret(c);
                Resume::Done
            }
        };
        match flow {
            Resume::Go(s) => stage = s,
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// The entry block: picks the list (by the used bit) and the class (by
/// size); empty lists take the node at once, else sets up the walk.
fn link_entry(c: &mut Cpu) -> Option<LinkStage> {
    c.cycles += 11;
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
    c.g[V1][0] = 0x1c;
    c.sq(S0, r(c, SP));
    let scaled = scale(c, V1, c.g[A1][0]);
    let h = r(c, A0);
    c.g[T0][0] = c.g[A0][0];
    let nodes = lw(c, h + VramHeap::NODES);
    c.g[A0][0] = nodes;
    let node = addu(nodes, scaled);
    c.g[T1][0] = node;
    let used = lw(c, node as u32 + VramNode::WORD) & 1;
    c.g[V0][0] = used;
    c.g[T4][0] = sx(h.wrapping_add(VramHeap::USED_LISTS));
    c.cycles += 2;
    if used != 0 {
        c.g[A3][0] = TAG_USED_REF;
    } else {
        c.g[T4][0] = sx(h.wrapping_add(VramHeap::FREE_LISTS));
        c.g[A3][0] = TAG_FREE_REF;
    }

    c.cycles += 14;
    let size = lw(c, node as u32 + VramNode::SIZE);
    c.g[T2][0] = size;
    let class = size_class(size);
    c.g[V0][0] = class;
    c.g[V1][0] = 0x1c;
    c.g[A0][0] = u64::MAX;
    let off = scale(c, T3, class);
    let head = addu(off, c.g[T4][0]) as u32;
    c.g[A2][0] = sx(head);
    let first = lw(c, head + VramNode::SIZE_NEXT);
    c.g[V1][0] = first;
    if first == u64::MAX {
        // Empty list: the node is its only entry.
        c.cycles += 5;
        sw(c, head + VramNode::SIZE_NEXT, c.g[A1][0]);
        let tagged = class | c.g[A3][0];
        c.g[V0][0] = tagged;
        sw(c, node as u32 + VramNode::SIZE_NEXT, first);
        sw(c, node as u32 + VramNode::SIZE_PREV, tagged);
        return Some(LinkStage::Exit);
    }
    c.g[A3][0] = first;
    c.cycles += 2;
    c.g[S0][0] = TAG_USED;
    if (first as i64) < 0 {
        return Some(LinkStage::Exit);
    }
    c.cycles += 9;
    c.g[T3][0] = TAG_INDEX;
    c.g[T7][0] = size;
    c.g[A0][0] = 0x1c;
    c.g[T8][0] = u64::MAX;
    c.g[T6][0] = sx(h.wrapping_add(VramHeap::USED_LISTS));
    c.g[T5][0] = sx(h.wrapping_add(VramHeap::FREE_LISTS));
    c.g[T9][0] = TAG_MASK;
    scale(c, V1, first);
    Some(LinkStage::Scan)
}

/// The list walk (head 0x366168): finds the node to insert before, or the
/// end of the list. Loop state lives in the registers.
fn link_scan(c: &mut Cpu) -> Resume<LinkStage> {
    loop {
        c.cycles += 6;
        let h = r(c, T0);
        let nodes = lw(c, h + VramHeap::NODES);
        c.g[V0][0] = nodes;
        c.g[T2][0] = nodes;
        let cur = addu(nodes, c.g[V1][0]) as u32;
        c.g[A2][0] = sx(cur);
        let next = lw(c, cur + VramNode::SIZE_NEXT);
        c.g[V0][0] = next;
        if next == c.g[T8][0] {
            sw(c, cur + VramNode::SIZE_NEXT, c.g[A1][0]);
            if back_edge_unwinds(c, LINK_APPEND) {
                return Resume::Unwind;
            }
            return Resume::Go(LinkStage::Append);
        }
        c.cycles += 2;
        let node = r(c, T1);
        if c.g[T4][0] == c.g[T6][0] {
            // Used lists: by address.
            c.cycles += 5;
            let cur_word = lw(c, cur + VramNode::WORD);
            c.g[V1][0] = cur_word;
            let word = lw(c, node + VramNode::WORD);
            let before = (word as i64) < (cur_word as i64);
            c.g[V0][0] = before as u64;
            if before {
                c.g[V1][0] = lw(c, cur + VramNode::SIZE_PREV);
                return Resume::Go(LinkStage::InsertBefore);
            }
        }
        c.cycles += 2;
        let scaled = scale(c, V1, c.g[A3][0]);
        let cur_addr = if c.g[T4][0] != c.g[T5][0] {
            c.cycles += 1;
            let a = addu(scaled, c.g[T2][0]);
            c.g[V0][0] = a;
            a
        } else {
            // Free lists: by size.
            c.cycles += 4;
            let cur_size = lw(c, cur + VramNode::SIZE);
            let smaller = (cur_size as i64) < (c.g[T7][0] as i64);
            let a = addu(scaled, c.g[T2][0]);
            c.g[V0][0] = a;
            if !smaller {
                c.cycles += 1;
                c.g[V1][0] = lw(c, cur + VramNode::SIZE_PREV);
                return Resume::Go(LinkStage::InsertBefore);
            }
            a
        };
        // Advance (0x36621c).
        c.cycles += 3;
        let next = lw(c, cur_addr as u32 + VramNode::SIZE_NEXT);
        c.g[A3][0] = next;
        scale(c, V1, next);
        if (next as i64) < 0 {
            return Resume::Go(LinkStage::Exit);
        }
        if back_edge_unwinds(c, LINK_SCAN) {
            return Resume::Unwind;
        }
    }
}

/// Insert the node before the current one (a3); v1 holds the current
/// node's `size_prev` link (a node or a list head).
fn link_insert_before(c: &mut Cpu) -> Resume<LinkStage> {
    c.cycles += 3;
    let node = r(c, T1);
    let h = r(c, T0);
    let prev = c.g[V1][0];
    sw(c, node + VramNode::SIZE_NEXT, c.g[A3][0]);
    sw(c, node + VramNode::SIZE_PREV, prev);
    let before = if (prev as i64) >= 0 {
        c.cycles += 3;
        let scaled = scale(c, V1, prev);
        let a = addu(lw(c, h + VramHeap::NODES), scaled);
        c.g[V0][0] = a;
        a
    } else {
        c.cycles += 3;
        let used = prev & c.g[T9][0] == c.g[S0][0];
        c.g[V0][0] = prev & c.g[T3][0];
        c.cycles += 4;
        let scaled = scale(c, V0, c.g[V0][0]);
        let base = if used { VramHeap::USED_LISTS } else { VramHeap::FREE_LISTS };
        let off = addu(scaled, base as u64);
        let a = addu(h as u64, off);
        c.g[V0][0] = a;
        a
    };
    c.cycles += 7;
    let idx = c.g[A1][0];
    sw(c, before as u32 + VramNode::SIZE_NEXT, idx);
    let next = lw(c, node + VramNode::SIZE_NEXT);
    let nodes = lw(c, h + VramHeap::NODES);
    c.g[V1][0] = nodes;
    let scaled = scale(c, V0, next);
    let after = addu(scaled, nodes);
    c.g[V0][0] = after;
    sw(c, after as u32 + VramNode::SIZE_PREV, idx);
    Resume::Go(LinkStage::Exit)
}

/// `int VramHeap_FindFreeBlock(VramHeap* h, int size)` (0x3667f8): the
/// first free block of at least `size` bytes, searching the free lists from
/// the size's class upwards; -1 if none.
pub fn vram_heap_find_free_block(c: &mut Cpu) {
    const CLASS_LOOP: u32 = 0x0036_6830;
    const NODE_LOOP: u32 = 0x0036_6848;
    c.cycles += 10;
    let size = c.g[A1][0];
    let class = size_class(size);
    c.g[V0][0] = size_class_v0(size);
    c.g[A3][0] = class;
    let in_range = (class as i64) < VramHeap::CLASSES as i64;
    c.g[V1][0] = in_range as u64;
    c.g[V0][0] = 0x1c;
    if in_range {
        c.cycles += 4;
        let off = scale(c, V0, class);
        let off = addu(off, (VramHeap::FREE_LISTS + VramNode::SIZE_NEXT) as u64);
        c.g[V0][0] = off;
        c.g[T0][0] = addu(off, c.g[A0][0]);
        loop {
            // One class list (head 0x366830).
            c.cycles += 3;
            let first = lw(c, r(c, T0));
            c.g[V1][0] = first;
            if (first as i64) < 0 {
                c.g[A3][0] = addu(c.g[A3][0], 1);
            } else {
                c.cycles += 3;
                c.g[T2][0] = lw(c, r(c, A0) + VramHeap::NODES);
                c.g[T1][0] = 0x1c;
                scale(c, V0, first);
                loop {
                    // One node (head 0x366848).
                    c.cycles += 5;
                    let node = addu(c.g[V0][0], c.g[T2][0]) as u32;
                    c.g[A2][0] = sx(node);
                    let node_size = lw(c, node + VramNode::SIZE);
                    let small = (node_size as i64) < (c.g[A1][0] as i64);
                    c.g[V0][0] = small as u64;
                    if !small {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        abi::ret(c);
                        return;
                    }
                    let next = lw(c, node + VramNode::SIZE_NEXT);
                    c.g[V1][0] = next;
                    c.cycles += 2;
                    scale(c, V0, next);
                    if (next as i64) < 0 {
                        c.cycles += 1;
                        c.g[A3][0] = addu(c.g[A3][0], 1);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, NODE_LOOP);
                    }
                }
            }
            c.cycles += 3;
            let more = (c.g[A3][0] as i64) < VramHeap::CLASSES as i64;
            c.g[V0][0] = more as u64;
            c.g[T0][0] = addu(c.g[T0][0], VramNode::SIZE_OF as u64);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, CLASS_LOOP);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    abi::ret(c);
}

/// Direct calls between the VRAM heap functions (shard 12).
mod vram_callee {
    use super::{ee, Callee};

    pub const UNLINK_SIZE_LIST: Callee = Callee { addr: 0x0036_6238, f: ee::s12::VramHeap_UnlinkSizeList };
    pub const POP_SPARE_NODE: Callee = Callee { addr: 0x0036_5fe8, f: ee::s12::VramHeap_PopSpareNode };
    pub const LINK_FREE_BLOCK: Callee = Callee { addr: 0x0036_60a8, f: ee::s12::VramHeap_LinkFreeBlock };
    pub const REMOVE_NODE: Callee = Callee { addr: 0x0036_6008, f: ee::s12::VramHeap_RemoveNode };
    pub const FREE_BLOCK: Callee = Callee { addr: 0x0036_63d8, f: ee::s12::VramHeap_FreeBlock };
    pub const FIND_FREE_BLOCK: Callee = Callee { addr: 0x0036_67f8, f: ee::s12::VramHeap_FindFreeBlock };
    pub const EVICT_CHEAPEST_BLOCK: Callee =
        Callee { addr: 0x0036_68f8, f: ee::s12::VramHeap_EvictCheapestBlock };
    pub const EVICT_RANGE: Callee = Callee { addr: 0x0036_69f0, f: ee::s12::VramHeap_EvictRange };
    pub const SPLIT_BLOCK: Callee = Callee { addr: 0x0036_62d0, f: ee::s12::VramHeap_SplitBlock };
}

/// `void VramHeap_SplitBlock(VramHeap* h, int idx, int size)` (0x3662d0):
/// takes the free block `idx` for an allocation of `size`: the remainder
/// (if any, and if a spare node is left) becomes a new free block after it;
/// the block is marked used and moved to the used lists.
pub fn vram_heap_split_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        let idx = c.g[A1][0];
        c.g[S3][0] = idx;
        c.g[S4][0] = 0x1c;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        let scaled = scale(c, V1, idx);
        frame.save(c, S2, 0x40);
        let h = c.g[A0][0];
        c.g[S1][0] = h;
        frame.save(c, S5, 0x10);
        let size = c.g[A2][0];
        c.g[S2][0] = size;
        frame.save_ra(c, 0);
        let h = h as u32;
        let nodes = lw(c, h + VramHeap::NODES);
        c.g[V0][0] = nodes;
        let node = addu(nodes, scaled);
        c.g[S0][0] = node;
        let node = node as u32;
        vram_callee::UNLINK_SIZE_LIST.call(c, 0x0036_6310)?;

        let block_size = lw(c, node + VramNode::SIZE);
        c.g[V0][0] = block_size;
        c.g[S5][0] = u64::MAX;
        let mut split = false;
        if size != block_size {
            c.cycles += 3;
            let spare = lw(c, h + VramHeap::SPARE);
            c.g[V0][0] = spare;
            split = spare != u64::MAX;
        }
        let word = if !split {
            c.cycles += 2;
            lw(c, node + VramNode::WORD)
        } else {
            // The remainder becomes a new free block after this one.
            c.cycles += 17;
            c.g[A0][0] = h as u64;
            vram_callee::POP_SPARE_NODE.call(c, 0x0036_6338)?;
            let rest_idx = c.g[V0][0];
            c.g[A2][0] = rest_idx;
            let nodes = lw(c, h + VramHeap::NODES);
            let scaled = scale(c, A0, rest_idx);
            let addr = addu(lw(c, node + VramNode::WORD), size);
            c.g[V0][0] = addr;
            let rest = addu(nodes, scaled);
            c.g[V1][0] = rest;
            let rest = rest as u32;
            sw(c, rest + VramNode::WORD, addr);
            let block_size = lw(c, node + VramNode::SIZE);
            sw(c, rest + VramNode::OWNER, u64::MAX);
            let rest_size = sx((block_size as u32).wrapping_sub(size as u32));
            c.g[A0][0] = rest_size;
            sw(c, rest + VramNode::PREV, idx);
            sw(c, rest + VramNode::SIZE, rest_size);
            let next = lw(c, node + VramNode::NEXT);
            c.g[V0][0] = next;
            sw(c, rest + VramNode::NEXT, next);
            if (next as i64) >= 0 {
                c.cycles += 4;
                let scaled = scale(c, V0, next);
                let nodes = lw(c, h + VramHeap::NODES);
                c.g[V1][0] = nodes;
                let after = addu(scaled, nodes);
                c.g[V0][0] = after;
                sw(c, after as u32 + VramNode::PREV, rest_idx);
            }
            c.cycles += 6;
            c.g[A1][0] = rest_idx;
            sw(c, node + VramNode::NEXT, rest_idx);
            c.g[A0][0] = h as u64;
            vram_callee::LINK_FREE_BLOCK.call(c, 0x0036_6394)?;
            let word = lw(c, node + VramNode::WORD);
            sw(c, node + VramNode::SIZE, size);
            word
        };
        // Mark used and link into the used lists.
        c.cycles += 14;
        let word = word | 1;
        c.g[V0][0] = word;
        sw(c, node + VramNode::WORD, word);
        c.g[A0][0] = h as u64;
        c.g[A1][0] = idx;
        vram_callee::LINK_FREE_BLOCK.call(c, 0x0036_63b0)?;
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

/// `void VramHeap_FreeBlock(VramHeap* h, int idx)` (0x3663d8): frees a
/// block: resets its owner's cached address fields, clears the flags,
/// moves it to the free lists and merges it with free neighbours in
/// address order (the merged-away nodes go back to the spare chain).
pub fn vram_heap_free_block(c: &mut Cpu) {
    const MERGE_LOOP: u32 = 0x0036_6488;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x1c;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = 0x7fff_0000;
        let idx = c.g[A1][0];
        c.g[S2][0] = idx;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let scaled = scale(c, V1, idx);
        frame.save(c, S3, 0x10);
        let h = c.g[A0][0];
        c.g[S1][0] = h;
        frame.save_ra(c, 0);
        c.g[V0][0] = NO_OWNER;
        let h = h as u32;
        let nodes = lw(c, h + VramHeap::NODES);
        c.g[A0][0] = nodes;
        let node = addu(nodes, scaled);
        c.g[S0][0] = node;
        let node = node as u32;
        let owner = lw(c, node + VramNode::OWNER);
        let slot = sx((owner as u32) << 2);
        c.g[V1][0] = slot;
        if owner != NO_OWNER {
            // Forget the owner's cached VRAM address.
            c.cycles += 7;
            let owners = lw(c, h + VramHeap::OWNERS);
            c.g[A0][0] = owners;
            let word = lw(c, node + VramNode::WORD);
            let second = sx(((word as u32) as i32 >> 2) as u32) & 1;
            c.g[V0][0] = second;
            let record = lw(c, addu(slot, owners) as u32) as u32;
            c.g[V1][0] = sx(record);
            c.g[V0][0] = u64::MAX;
            if second != 0 {
                c.cycles += 4;
                sw(c, record + 0x34, u64::MAX);
                sw(c, record + 0x2c, u64::MAX);
            } else {
                c.cycles += 3;
                sw(c, record + 0x30, u64::MAX);
                sw(c, record + 0x28, u64::MAX);
            }
        }
        c.cycles += 13;
        let word = lw(c, node + VramNode::WORD) & !0x3f;
        c.g[V1][0] = !0x3f;
        c.g[A0][0] = h as u64;
        c.g[A1][0] = idx;
        c.g[V0][0] = word;
        sw(c, node + VramNode::WORD, word);
        c.g[S3][0] = 0x1c;
        vram_callee::UNLINK_SIZE_LIST.call(c, 0x0036_6474)?;
        c.g[A0][0] = h as u64;
        c.g[A1][0] = idx;
        vram_callee::LINK_FREE_BLOCK.call(c, 0x0036_6480)?;
        c.g[A1][0] = lw(c, node + VramNode::PREV);

        // Merge with free neighbours. s0/s2: the block (node, index).
        loop {
            c.cycles += 2;
            let prev = c.g[A1][0];
            let prev_off = scale(c, A0, prev);
            let block = r(c, S0);
            let mut merge_prev = false;
            let next;
            if (prev as i64) < 0 {
                c.cycles += 1;
                next = lw(c, block + VramNode::NEXT);
            } else {
                c.cycles += 6;
                let nodes = lw(c, h + VramHeap::NODES);
                c.g[V1][0] = nodes;
                let prev_node = addu(prev_off, nodes);
                c.g[A0][0] = prev_node;
                let used = lw(c, prev_node as u32 + VramNode::WORD) & 1;
                c.g[V0][0] = used;
                if used != 0 {
                    next = lw(c, block + VramNode::NEXT);
                } else {
                    // The previous block absorbs this one.
                    c.cycles += 3;
                    c.g[S2][0] = prev;
                    c.g[S0][0] = prev_node;
                    merge_prev = true;
                    next = 0;
                }
            }
            if !merge_prev {
                c.g[V1][0] = next;
                c.cycles += 2;
                let next_off = scale(c, V1, next);
                if (next as i64) < 0 {
                    c.cycles += 1;
                    break;
                }
                c.cycles += 6;
                let nodes = lw(c, h + VramHeap::NODES);
                c.g[A0][0] = nodes;
                let next_node = addu(next_off, nodes);
                c.g[V1][0] = next_node;
                let used = lw(c, next_node as u32 + VramNode::WORD) & 1;
                c.g[V0][0] = used;
                if used != 0 {
                    break;
                }
            }
            // Merge s0 with its address-order successor.
            c.cycles += 21;
            let block = r(c, S0);
            c.g[A0][0] = h as u64;
            c.g[A1][0] = c.g[S2][0];
            vram_callee::UNLINK_SIZE_LIST.call(c, 0x0036_64e4)?;
            c.g[A1][0] = lw(c, block + VramNode::NEXT);
            c.g[A0][0] = h as u64;
            vram_callee::UNLINK_SIZE_LIST.call(c, 0x0036_64f0)?;
            let next = lw(c, block + VramNode::NEXT);
            c.g[A1][0] = next;
            c.g[A0][0] = h as u64;
            let nodes = lw(c, h + VramHeap::NODES);
            c.g[A3][0] = nodes;
            let next_off = scale(c, V1, next);
            let size = lw(c, block + VramNode::SIZE);
            let next_node = addu(next_off, nodes);
            c.g[V1][0] = next_node;
            let next_size = lw(c, next_node as u32 + VramNode::SIZE);
            c.g[V0][0] = next_size;
            let merged = addu(size, next_size);
            c.g[A2][0] = merged;
            sw(c, block + VramNode::SIZE, merged);
            vram_callee::REMOVE_NODE.call(c, 0x0036_6518)?;
            c.g[A0][0] = h as u64;
            c.g[A1][0] = c.g[S2][0];
            vram_callee::LINK_FREE_BLOCK.call(c, 0x0036_6524)?;
            c.g[A1][0] = lw(c, r(c, S0) + VramNode::PREV);
            if event_due(c) {
                deliver_event_here(c, MERGE_LOOP);
            }
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

/// `bool VramHeap_EvictCheapestBlock(VramHeap* h, int size, int keep_a,
/// int keep_b, int frame)` (0x3668f8): among the used blocks of the size's
/// class that are big enough, not locked and not owned by `keep_a`/`keep_b`,
/// frees the one whose owner is cheapest to reload (`size * weight`, or 0
/// when the owner was last used at or before `frame`). Returns whether a
/// block was freed.
pub fn vram_heap_evict_cheapest_block(c: &mut Cpu) {
    const SCAN_LOOP: u32 = 0x0036_6950;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let size = c.g[A1][0];
        let class = size_class(size);
        c.g[V1][0] = 0x1c;
        c.g[T3][0] = 0x7fff_0000;
        let off = scale(c, V0, class);
        c.g[T6][0] = u64::MAX;
        let h = c.g[A0][0];
        let head = addu(h, off);
        c.g[V0][0] = head;
        let first = lw(c, head as u32 + VramHeap::USED_LISTS + VramNode::SIZE_NEXT);
        c.g[T1][0] = first;
        c.g[T3][0] = NO_OWNER;
        if (first as i64) >= 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0054_0000;
            c.g[T4][0] = lw(c, h as u32 + VramHeap::NODES);
            c.g[T7][0] = VRAM_OWNER_STATS as u64;
            c.g[T5][0] = 0x1c;
            scale(c, V0, first);
            loop {
                // One candidate (head 0x366950); t1 its index.
                c.cycles += 5;
                let node = addu(c.g[V0][0], c.g[T4][0]) as u32;
                c.g[V1][0] = sx(node);
                let node_size = lw(c, node + VramNode::SIZE);
                c.g[T2][0] = node_size;
                let small = (node_size as i64) < (c.g[A1][0] as i64);
                c.g[V0][0] = small as u64;
                if !small {
                    c.cycles += 5;
                    let locked = sx(((lw(c, node + VramNode::WORD) as u32) as i32 >> 1) as u32) & 1;
                    c.g[V0][0] = locked;
                    if locked == 0 {
                        c.cycles += 3;
                        let owner = lw(c, node + VramNode::OWNER);
                        c.g[V0][0] = owner;
                        if owner != c.g[A2][0] {
                            c.cycles += 2;
                            let stats = sx((owner as u32) << 3);
                            c.g[V1][0] = stats;
                            if owner == c.g[A3][0] {
                                c.cycles += 1;
                            } else {
                                c.cycles += 5;
                                let stats = addu(stats, c.g[T7][0]) as u32;
                                c.g[V1][0] = sx(stats);
                                let last_used = lw(c, stats);
                                let recent = (c.g[T0][0] as i64) < (last_used as i64);
                                c.g[V0][0] = recent as u64;
                                if !recent {
                                    c.g[V1][0] = 0;
                                } else {
                                    c.cycles += 2;
                                    let weight = lw(c, stats + 4);
                                    c.g[V0][0] = weight;
                                    let cost = sx(abi::mult(c, node_size as u32, weight as u32));
                                    c.g[V1][0] = cost;
                                }
                                c.cycles += 3;
                                let cost = c.g[V1][0];
                                let cheaper = (cost as i64) < (c.g[T3][0] as i64);
                                c.g[V0][0] = cheaper as u64;
                                if cheaper {
                                    c.cycles += 2;
                                    c.g[T3][0] = cost;
                                    c.g[T6][0] = c.g[T1][0];
                                    c.cycles += 1;
                                }
                            }
                        }
                    }
                }
                scale(c, V1, c.g[T1][0]);
                // Next candidate.
                c.cycles += 4;
                let node = addu(c.g[V1][0], c.g[T4][0]);
                c.g[V0][0] = node;
                let next = lw(c, node as u32 + VramNode::SIZE_NEXT);
                c.g[T1][0] = next;
                scale(c, V0, next);
                if (next as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SCAN_LOOP);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        let best = c.g[T6][0];
        if (best as i64) >= 0 {
            c.cycles += 3;
            c.g[A1][0] = best;
            vram_callee::FREE_BLOCK.call(c, 0x0036_69e0)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int VramHeap_Alloc(VramHeap* h, int size, int owner, int keep, int
/// frame, bool page_align)` (0x366ce0): allocates at least 0x100 bytes
/// (rounded up to 8 KiB pages if asked), evicting the cheapest block or a
/// cheapest range until a free block fits. Returns the node index; the
/// block's owner is `owner`.
pub fn vram_heap_alloc(c: &mut Cpu) {
    const RETRY: u32 = 0x0036_6d40;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        if c.g[T1][0] != 0 {
            // Round up to a whole 8 KiB page.
            c.cycles += 3;
            let pad = sx(0u32.wrapping_sub(r(c, S1))) & 0x1fff;
            c.g[V0][0] = pad;
            c.g[S1][0] = addu(c.g[S1][0], pad);
        }
        c.cycles += 6;
        c.g[V1][0] = 0x100;
        let big = 0xff < c.g[S1][0] as i64;
        c.g[V0][0] = big as u64;
        c.g[S6][0] = 0x1c;
        if !big {
            c.g[S1][0] = 0x100;
        }
        c.g[A0][0] = c.g[S2][0];
        let (h, size, owner, keep, now) = (c.g[S2][0], c.g[S1][0], c.g[S3][0], c.g[S5][0], c.g[S4][0]);
        let idx = loop {
            c.cycles += 5;
            c.g[A1][0] = size;
            vram_callee::FIND_FREE_BLOCK.call(c, 0x0036_6d48)?;
            let idx = c.g[V0][0];
            c.g[S0][0] = idx;
            c.g[A2][0] = size;
            if (idx as i64) >= 0 {
                break idx;
            }
            c.cycles += 8;
            c.g[A0][0] = h;
            c.g[A1][0] = size;
            c.g[A2][0] = owner;
            c.g[A3][0] = keep;
            c.g[T0][0] = now;
            vram_callee::EVICT_CHEAPEST_BLOCK.call(c, 0x0036_6d6c)?;
            let evicted = c.g[V0][0] != 0;
            c.g[A0][0] = h;
            if !evicted {
                c.cycles += 7;
                c.g[A1][0] = size;
                c.g[A2][0] = owner;
                c.g[A3][0] = keep;
                c.g[T0][0] = now;
                vram_callee::EVICT_RANGE.call(c, 0x0036_6d88)?;
                c.g[A0][0] = h;
            }
            if event_due(c) {
                deliver_event_here(c, RETRY);
            }
        };
        c.cycles += 18;
        c.g[A0][0] = h;
        c.g[A1][0] = idx;
        vram_callee::SPLIT_BLOCK.call(c, 0x0036_6d9c)?;
        let off = scale(c, V1, idx);
        let nodes = lw(c, h as u32 + VramHeap::NODES);
        c.g[A0][0] = nodes;
        c.g[V0][0] = idx;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S2, 0x50);
        let node = addu(off, nodes);
        c.g[V1][0] = node;
        frame.restore(c, S4, 0x30);
        sw(c, node as u32 + VramNode::OWNER, owner);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::*;
    use crate::difftest::{check, check_lohi, check_paths, check_paths_lohi, fill, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 300;

    // ------------------------------------------------------------ replay

    #[test]
    fn replay_stop_recording_matches() {
        check_paths(ssx3_ee::s07::f_0026fa50__generated, replay_stop_recording, SEEDS, &[8, 9], &|c, seed| {
            let mut r = machine(c, seed, 0x801, 0);
            let rp = ptr(&mut r);
            c.g[A0][0] = rp as u64;
            let state = match r.next_u32() % 3 {
                0 => 0xd + r.next_u32() % 4,
                1 => r.next_u32() % 0x20,
                _ => r.next_u32(),
            };
            c.bus.write32(rp, state);
        });
    }

    #[test]
    fn replay_cancel_instant_replay_matches() {
        check_paths(ssx3_ee::s07::f_00270628__generated, replay_cancel_instant_replay, SEEDS, &[6, 12], &|c, seed| {
            let mut r = machine(c, seed, 0x802, 0);
            let rp = ptr(&mut r);
            c.g[A0][0] = rp as u64;
            let pending = if r.next_u32() % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(rp + 0x3d4, pending);
            let (m, a, b) = (ptr(&mut r), ptr(&mut r), ptr(&mut r));
            c.bus.write32(GAME_GP - 0x848, m);
            c.bus.write32(m + 0x84, a);
            c.bus.write32(a + 0xc, b);
            let flag = match r.next_u32() % 3 {
                0 => 1,
                1 => r.next_u32() % 3,
                _ => r.next_u32(),
            };
            c.bus.write32(b + 0x7c, flag);
        });
    }

    #[test]
    fn replay_add_snapshot_matches() {
        check(ssx3_ee::s07::f_0026d740__generated, replay_add_snapshot, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x803, 0);
            let rp = ptr(&mut r);
            let node = ptr(&mut r);
            c.g[A0][0] = rp as u64;
            c.g[A1][0] = node as u64;
            let list = rp + 0x3b0;
            let tail = if r.next_u32() % 3 == 0 { 0 } else { ptr(&mut r) };
            c.bus.write32(list + 4, tail);
            c.bus.write32(list, if r.next_u32() % 3 == 0 { 0 } else { ptr(&mut r) });
            c.bus.write32(list + 8, r.next_u32() % 50);
        });
    }

    #[test]
    fn replay_stream_write_matches() {
        check(ssx3_ee::s07::f_0026e9d0__generated, replay_stream_write, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x804, 0);
            fill(c, &mut r, DATA, 0x800);
            let s = DATA + 0x4000 + 0x10 * (r.next_u32() % 16);
            c.g[A0][0] = s as u64;
            c.bus.write32(s + 8, DATA + 0x6000 + r.next_u32() % 64);
            c.bus.write32(s + 0x10, r.next_u32());
            c.g[A1][0] = (DATA + r.next_u32() % 0x100) as u64;
            let n = match r.next_u32() % 4 {
                0 => 0,
                1 => r.next_u32() % 16,
                2 => r.next_u32() % 0x400,
                _ => 0u32.wrapping_sub(r.next_u32() % 4),
            };
            c.g[A2][0] = sx(n);
        });
    }

    // ------------------------------------------------------------ event ring

    fn ring_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let q = ptr(&mut r);
        c.g[A0][0] = q as u64;
        fill(c, &mut r, q, 0x60);
        let head = match r.next_u32() % 4 {
            0 => r.next_u32() % 16,
            1 => 15,
            2 => 0u32.wrapping_sub(r.next_u32() % 20),
            _ => r.next_u32(),
        };
        let tail = if r.next_u32() % 3 == 0 { head } else { r.next_u32() % 16 };
        c.bus.write32(q, head);
        c.bus.write32(q + 4, tail);
    }

    #[test]
    fn wscript_event_queue_pop_matches() {
        check_paths_lohi(ssx3_ee::s10::f_0030c820__generated, wscript_event_queue_pop, SEEDS, &[7, 18], &|c, seed| {
            ring_setup(c, seed, 0x805)
        });
    }

    #[test]
    fn wscript_event_queue_peek_matches() {
        check_paths_lohi(ssx3_ee::s10::f_0030c870__generated, wscript_event_queue_peek, SEEDS, &[6, 8], &|c, seed| {
            ring_setup(c, seed, 0x806)
        });
    }

    // ------------------------------------------------------------ memory

    /// Runs a generated function to completion as a setup step.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, crate::difftest::RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, with a few
    /// live blocks; returns them.
    fn mem_heap(c: &mut Cpu, r: &mut Rng) -> Vec<u32> {
        c.g[GP][0] = GAME_GP as u64;
        run(c, 0x0031_ff60, ssx3_ee::s10::DLHeap_InitArena, &[super::MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000]);
        let mut blocks = Vec::new();
        for _ in 0..r.next_u32() % 6 {
            let size = 8 + r.next_u32() % 0x2000;
            let b = run(c, 0x0032_0058, ssx3_ee::s10::MEM_HeapMemalign, &[super::MEM_HEAP as u64, 16, size as u64]);
            blocks.push(b as u32);
        }
        blocks
    }

    #[test]
    fn ea_mem_zero_matches() {
        check(ssx3_ee::s14::f_003e6220__generated, ea_mem_zero, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x807, 0);
            fill(c, &mut r, DATA, 0x400);
            c.g[A0][0] = (DATA + r.next_u32() % 0x100) as u64;
            c.g[A1][0] = match r.next_u32() % 3 {
                0 => 0,
                1 => (r.next_u32() % 0x300) as u64,
                _ => sx(0u32.wrapping_sub(r.next_u32() % 4)),
            };
        });
    }

    /// The allocator mutex: sometimes held by "this thread" (the first fake
    /// syscall, `GetThreadId`, returns 0x42), at some depth.
    fn mutex_state(c: &mut Cpu, r: &mut Rng, m: u32) {
        match r.next_u32() % 3 {
            0 => {
                c.bus.write32(m + 4, 0x42);
                c.bus.write32(LOG_RESULTS, 0x42);
                c.bus.write32(m + 8, r.next_u32() % 4);
            }
            1 => {
                c.bus.write32(m + 4, r.next_u32() % 4);
                c.bus.write32(m + 8, r.next_u32() % 3);
            }
            _ => {
                c.bus.write32(m + 4, 0x42);
                c.bus.write32(LOG_RESULTS, 0x42);
                c.bus.write32(m + 8, 1);
            }
        }
        c.bus.write32(m + 0xc, r.next_u32() % 64);
        c.bus.write32(super::IN_INTERRUPT, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() });
    }

    #[test]
    fn ea_mutex_lock_matches() {
        covered(ssx3_ee::s14::f_003e5700__generated, ea_mutex_lock, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x808, 3);
            let m = ptr(&mut r);
            c.g[A0][0] = m as u64;
            mutex_state(c, &mut r, m);
        });
    }

    #[test]
    fn ea_mutex_unlock_matches() {
        covered(ssx3_ee::s14::f_003e5760__generated, ea_mutex_unlock, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x809, 2);
            let m = ptr(&mut r);
            c.g[A0][0] = m as u64;
            mutex_state(c, &mut r, m);
        });
    }

    /// A pointer to free: null, a pool pointer (the pools' bases are 0 in
    /// the test machine, so small addresses count as pooled), or a live
    /// heap block.
    fn free_target(c: &mut Cpu, r: &mut Rng) -> u64 {
        let blocks = mem_heap(c, r);
        match r.next_u32() % 4 {
            0 => 0,
            1 => (0x10 + 8 * (r.next_u32() % 0x80)) as u64,
            _ if !blocks.is_empty() => blocks[(r.next_u32() as usize) % blocks.len()] as u64,
            _ => 0,
        }
    }

    #[test]
    fn mem_free_internal_matches() {
        check(ssx3_ee::s10::f_00319b48__generated, mem_free_internal, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x80a, 0);
            let p = free_target(c, &mut r);
            c.g[A0][0] = p;
        });
    }

    #[test]
    fn ea_mem_free_and_mem_free_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s10::f_00317e98__generated as fn(&mut Cpu), ea_mem_free as fn(&mut Cpu), 0x80b),
            (ssx3_ee::s06::f_002523a8__generated, mem_free, 0x80c),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 4);
                mutex_state(c, &mut r, super::MEM_MUTEX);
                let p = free_target(c, &mut r);
                c.g[A0][0] = p;
            });
        }
    }

    #[test]
    fn mem_alloc_internal_matches() {
        check(ssx3_ee::s10::f_00319a90__generated, mem_alloc_internal, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x80d, 0);
            mem_heap(c, &mut r);
            c.g[A0][0] = match r.next_u32() % 5 {
                0 => 0,
                1 => (r.next_u32() % 0x210) as u64,
                2 => 0x200 + (r.next_u32() % 3) as u64,
                3 => (r.next_u32() % 0x8000) as u64,
                _ => (r.next_u32() % 0x80000) as u64,
            };
            c.g[A1][0] = [0, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
            c.g[A2][0] = if r.next_u32() % 2 == 0 { sx(0x8000_0000 | r.next_u32()) } else { (r.next_u32() >> 1) as u64 };
            c.g[A3][0] = if r.next_u32() % 2 == 0 { 0 } else { (r.next_u32() % 64) as u64 };
        });
    }

    // ------------------------------------------------------------ VRAM heap

    use crate::difftest::events::{check_events, driven, schedule, Events};

    const HEAP: u32 = DATA + 0x10000;
    const NODES: u32 = DATA + 0x20000;
    const OWNER_TABLE: u32 = DATA + 0x30000;
    const HEAP_SIZE: u32 = 0x20_0000;

    /// The node array as `(index, word, size, owner)` for the nodes in
    /// address order.
    fn blocks(c: &mut Cpu) -> Vec<(u32, u32, u32, u32)> {
        let mut out = Vec::new();
        let mut i = c.bus.read32(HEAP + 0x23c0);
        while (i as i32) >= 0 && out.len() < 0x123 {
            let n = NODES + i * 0x1c;
            out.push((i, c.bus.read32(n), c.bus.read32(n + 4), c.bus.read32(n + 8)));
            i = c.bus.read32(n + 0x10);
        }
        out
    }

    /// A VRAM heap built by the game's own code: `VramHeap_Init`, then a
    /// random sequence of allocations and frees (owners 0..6, some
    /// without owner); some used blocks get the locked / second-field bits.
    fn vram_heap(c: &mut Cpu, r: &mut Rng, full: bool) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(HEAP + 0x1ff0, NODES);
        for i in 0..16 {
            c.bus.write32(OWNER_TABLE + 4 * i, OWNER_TABLE + 0x100 + 0x40 * i);
        }
        for i in 0..8 {
            c.bus.write32(super::VRAM_OWNER_STATS + 8 * i, r.next_u32() % 100);
            c.bus.write32(super::VRAM_OWNER_STATS + 8 * i + 4, r.next_u32() % 8);
        }
        let base = 0x1000 * (r.next_u32() % 4);
        run(c, 0x0036_5e40, ssx3_ee::s12::VramHeap_Init, &[HEAP as u64, base as u64, HEAP_SIZE as u64, OWNER_TABLE as u64]);
        let mut used = Vec::new();
        let mut total = 0u32;
        for _ in 0..r.next_u32() % if full { 800 } else { 60 } {
            if !used.is_empty() && r.next_u32() % 3 == 0 {
                let (i, s) = used.swap_remove((r.next_u32() as usize) % used.len());
                // (Skip blocks an allocation already evicted.)
                let n = NODES + i as u32 * 0x1c;
                if c.bus.read32(n) & 1 != 0 && c.bus.read32(n + 8) != u32::MAX {
                    run(c, 0x0036_63d8, ssx3_ee::s12::VramHeap_FreeBlock, &[HEAP as u64, i]);
                }
                total -= s;
                continue;
            }
            let size = 0x100 * (1 + r.next_u32() % [4, 32, 256][(r.next_u32() % 3) as usize]);
            let limit = if full { HEAP_SIZE - HEAP_SIZE / 16 } else { HEAP_SIZE / 2 };
            if total + size > limit {
                continue;
            }
            let owner = if r.next_u32() % 5 == 0 { 0x7fff_ffff } else { r.next_u32() % 7 };
            let i = run(c, 0x0036_6ce0, ssx3_ee::s12::VramHeap_Alloc, &[HEAP as u64, size as u64, owner as u64, 7, 50, 0]);
            let s = c.bus.read32(NODES + i as u32 * 0x1c + 4);
            used.push((i, s));
            total += s;
        }
        for (i, _) in used {
            let n = NODES + i as u32 * 0x1c;
            let w = c.bus.read32(n);
            let flags = match r.next_u32() % 6 {
                0 if !full => 2,
                1 => 4,
                _ => 0,
            };
            c.bus.write32(n, w | flags);
        }
    }

    /// Indices of free / used nodes.
    fn pick(c: &mut Cpu, r: &mut Rng, used: bool) -> Option<u32> {
        let b: Vec<u32> = blocks(c).into_iter().filter(|b| (b.1 & 1 != 0) == used).map(|b| b.0).collect();
        if b.is_empty() {
            None
        } else {
            Some(b[(r.next_u32() as usize) % b.len()])
        }
    }

    /// `check_events` plus LO/HI (`check_lohi`) on the same setups.
    fn check_vram(gen: fn(&mut Cpu), hand: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32) -> Events) {
        check_events(gen, hand, seeds, setup);
        check_lohi(gen, hand, seeds, &|c, s| {
            setup(c, s);
        });
    }

    fn vram_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        vram_machine_full(c, seed, salt, false)
    }

    /// A machine with a VRAM heap; `full`: filled to 15/16, no locked
    /// blocks (so allocations must evict, and eviction always succeeds).
    fn vram_machine_full(c: &mut Cpu, seed: u32, salt: u64, full: bool) -> Rng {
        let mut r = Rng(0x7672_616d ^ (seed as u64) << 12 ^ salt);
        vram_heap(c, &mut r, full);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = HEAP as u64;
        r
    }

    #[test]
    fn vram_heap_pop_spare_node_matches() {
        check_lohi(ssx3_ee::s12::f_00365fe8__generated, vram_heap_pop_spare_node, SEEDS, &|c, seed| {
            vram_machine(c, seed, 1);
        });
    }

    #[test]
    fn vram_heap_remove_node_matches() {
        check_lohi(ssx3_ee::s12::f_00366008__generated, vram_heap_remove_node, SEEDS, &|c, seed| {
            let mut r = vram_machine(c, seed, 2);
            let b = blocks(c);
            let i = b[(r.next_u32() as usize) % b.len()].0;
            c.g[A1][0] = i as u64;
            if r.next_u32() % 4 == 0 {
                // A tagged head reference as `prev` (both tables).
                let tag = if r.next_u32() % 2 == 0 { 0x9000_0000u32 } else { 0xa000_0000 };
                c.bus.write32(NODES + i * 0x1c + 0xc, tag | (r.next_u32() % 17));
            }
        });
    }

    #[test]
    fn vram_heap_unlink_size_list_matches() {
        check_lohi(ssx3_ee::s12::f_00366238__generated, vram_heap_unlink_size_list, SEEDS, &|c, seed| {
            let mut r = vram_machine(c, seed, 3);
            let b = blocks(c);
            c.g[A1][0] = b[(r.next_u32() as usize) % b.len()].0 as u64;
        });
    }

    driven!(gen_link, ssx3_ee::s12::f_003660a8__generated);
    driven!(hand_link, vram_heap_link_free_block);

    #[test]
    fn vram_heap_link_free_block_matches() {
        check_vram(gen_link, hand_link, 600, &|c, seed| {
            let mut r = vram_machine(c, seed, 4);
            let b = blocks(c);
            let i = b[(r.next_u32() as usize) % b.len()].0;
            run(c, 0x0036_6238, ssx3_ee::s12::VramHeap_UnlinkSizeList, &[HEAP as u64, i as u64]);
            let n = NODES + i * 0x1c;
            if r.next_u32() % 3 == 0 {
                let w = c.bus.read32(n);
                c.bus.write32(n, w ^ 1);
            }
            if r.next_u32() % 4 == 0 {
                // Sizes between the existing ones (and negative ones).
                let s = match r.next_u32() % 3 {
                    0 => 0x100 * (1 + r.next_u32() % 64),
                    1 => r.next_u32() % 0x8000,
                    _ => 0x40 << (r.next_u32() % 16),
                };
                c.bus.write32(n + 4, s);
            }
            c.g[A1][0] = i as u64;
            c.g[A0][0] = HEAP as u64;
            schedule(c, &mut r, seed, 0x0036_60a8)
        });
    }

    #[test]
    fn vram_heap_find_free_block_matches() {
        check_vram(ssx3_ee::s12::f_003667f8__generated, vram_heap_find_free_block, 400, &|c, seed| {
            let mut r = vram_machine(c, seed, 5);
            c.g[A1][0] = match r.next_u32() % 5 {
                0 => (0x100 * (1 + r.next_u32() % 64)) as u64,
                1 => (r.next_u32() % 0x40_0000) as u64,
                2 => (0x40u32 << (r.next_u32() % 26)) as u64,
                3 => HEAP_SIZE as u64,
                _ => r.next_u32() as u64 | (r.next_u32() as u64) << 32,
            };
            schedule(c, &mut r, seed, 0x0036_67f8)
        });
    }

    #[test]
    fn vram_heap_split_block_matches() {
        check_vram(ssx3_ee::s12::f_003662d0__generated, vram_heap_split_block, 400, &|c, seed| {
            let mut r = vram_machine(c, seed, 6);
            let i = pick(c, &mut r, false).unwrap_or(0);
            let size = c.bus.read32(NODES + i * 0x1c + 4);
            c.g[A1][0] = i as u64;
            c.g[A2][0] = match r.next_u32() % 3 {
                0 => size as u64,
                _ => (0x100 * (1 + r.next_u32() % (size / 0x100).max(1))).min(size) as u64,
            };
            if r.next_u32() % 5 == 0 {
                c.bus.write32(HEAP + 0x23dc, u32::MAX);
            }
            schedule(c, &mut r, seed, 0x0036_62d0)
        });
    }

    #[test]
    fn vram_heap_free_block_matches() {
        check_vram(ssx3_ee::s12::f_003663d8__generated, vram_heap_free_block, 400, &|c, seed| {
            let mut r = vram_machine(c, seed, 7);
            let i = pick(c, &mut r, true).unwrap_or(0);
            c.g[A1][0] = i as u64;
            schedule(c, &mut r, seed, 0x0036_63d8)
        });
    }

    #[test]
    fn vram_heap_evict_cheapest_block_matches() {
        check_vram(ssx3_ee::s12::f_003668f8__generated, vram_heap_evict_cheapest_block, 400, &|c, seed| {
            let mut r = vram_machine(c, seed, 8);
            c.g[A1][0] = match r.next_u32() % 3 {
                0 => (0x100 * (1 + r.next_u32() % 64)) as u64,
                1 => (0x100 * (1 + r.next_u32() % 4)) as u64,
                _ => (0x100 << (r.next_u32() % 12)) as u64,
            };
            c.g[A2][0] = (r.next_u32() % 8) as u64;
            c.g[A3][0] = if r.next_u32() % 2 == 0 { 0x7fff_ffff } else { (r.next_u32() % 8) as u64 };
            c.g[T0][0] = (r.next_u32() % 100) as u64;
            schedule(c, &mut r, seed, 0x0036_68f8)
        });
    }

    #[test]
    fn vram_heap_alloc_matches() {
        check_vram(ssx3_ee::s12::f_00366ce0__generated, vram_heap_alloc, 400, &|c, seed| {
            let mut r = vram_machine_full(c, seed, 9, seed % 4 < 3);
            c.g[A1][0] = match r.next_u32() % 4 {
                0 => (r.next_u32() % 0x300) as u64,
                1 => (0x100 * (1 + r.next_u32() % 64)) as u64,
                2 => (HEAP_SIZE / 16 + r.next_u32() % (HEAP_SIZE / 4)) as u64,
                _ => (r.next_u32() % 0x8_0000) as u64,
            };
            c.g[A2][0] = (r.next_u32() % 8) as u64;
            c.g[A3][0] = 7;
            c.g[T0][0] = (r.next_u32() % 100) as u64;
            c.g[T1][0] = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() as u64 };
            schedule(c, &mut r, seed, 0x0036_6ce0)
        });
    }
}
