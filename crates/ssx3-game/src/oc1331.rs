//! Staging module oc1331 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1331`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::game::bx::cubicspline::{
    MEM_BLK_FLAGS, MEM_BLK_NEXT, MEM_BLK_PREV, MEM_BLK_SIZE, MEM_CLS_ALIGN,
    MEM_CLS_LOCKED, MEM_CLS_MUTEX, MEM_CLS_NEXT, MEM_CLS_OVERHEAD, MEM_CLS_PREV,
    MEM_CLS_QUANTUM, MEM_FREE_NEXT, MEM_FREE_PREV,
};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_2a10, mem_heap_alloc_aligned, "MEM_HeapAllocAligned"),
    (0x0031_7ee0, ea_mem_sync_lock, "EA_MemSyncLock"),
    (0x0031_7d48, ea_mem_alloc_tagged, "EA_MemAllocTagged"),
    (0x0031_7d20, ea_mem_alloc_default, "EA_MemAllocDefault"),
    (
        0x0031_7f18,
        ea_mem_report_stats_wrapper,
        "EA_MemReportStats_Wrapper",
    ),
];

/// Callee slots shared with the MEM heap unit (re-exported so there is one
/// name per address) plus the one this module adds.
pub(crate) mod callee {
    use super::ee;
    use crate::call::Callee;
    pub(crate) use crate::game::bx::cubicspline::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::cubicspline::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::bx::cubicspline::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::bx::cubicspline::callee::MEM_GET_HEAP_BLOCK_OVERHEAD;
    pub(crate) use crate::game::bx::cubicspline::callee::MEM_INIT_BLOCK_HEADER;
    /// `MEM_ReportStatsLocked` (0x319cc8): pool-statistics reporter under
    /// the memory-manager lock (no crate callee name exists yet).
    pub(crate) const MEM_REPORT_STATS_LOCKED: Callee = Callee {
        addr: 0x0031_9cc8,
        f: ee::s10::f_00319cc8,
    };
}

/// Resume heads of [`mem_heap_alloc_aligned`]: the entry and every loop
/// back edge the generated body can unwind at. Post-call addresses go
/// through `jump_out` for the trampoline to re-drive, just like the
/// generated body (no callee of this function can unwind in a test: the
/// header/overhead callees are straight-line and the mutex/callback
/// targets are test fakes that never unwind).
mod heap_alloc {
    pub const ENTRY: u32 = 0x0025_2a10;
    /// Outer retry after the heap changed under us (0x252f20's check).
    pub const RETRY: u32 = 0x0025_2a88;
    /// Size-walk reload (0x252b48) and scan (0x252b54).
    pub const WALK1_RELOAD: u32 = 0x0025_2b48;
    pub const WALK1_SCAN: u32 = 0x0025_2b54;
    /// Backward size-walk reload (0x252b90) and scan (0x252b9c).
    pub const WALK2_RELOAD: u32 = 0x0025_2b90;
    pub const WALK2_SCAN: u32 = 0x0025_2b9c;
    /// Plain scan (0x252be0) and its forward-pointer variant (0x252c0c).
    pub const WALK3_SCAN: u32 = 0x0025_2be0;
    pub const WALK3_FP_SCAN: u32 = 0x0025_2c0c;
    /// Split inserts, forward (0x252d48) and backward (0x252d74) halves of
    /// the first, forward (0x252e38) and backward (0x252e64) of the second.
    pub const INS1_FWD: u32 = 0x0025_2d48;
    pub const INS1_BWD: u32 = 0x0025_2d74;
    pub const INS2_FWD: u32 = 0x0025_2e38;
    pub const INS2_BWD: u32 = 0x0025_2e64;
    /// End of the generated range (for `jump_out`).
    pub const END: u32 = 0x0025_2f60;
}

/// Stages of [`mem_heap_alloc_aligned`] between its resume heads: one
/// stage per generated block group, driven by the loop in the entry
/// function. Loop state lives in the registers (reloaded from them after
/// every event check), so resuming at a head just re-enters its stage.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Heap {
    Entry,
    LockCheck,
    /// `0x252a88`: lock-state dispatch both the entry and the retry
    /// path share. The retry path (`0x252f20`) enters here directly:
    /// it already reloaded the lock word into V0, skipping LockCheck.
    LockCheckJoin,
    RoundSize,
    Walk1First,
    Walk1Reload,
    Walk1Scan,
    Walk1Check,
    Walk2First,
    Walk2Reload,
    Walk2Scan,
    Walk2Check,
    Walk3First,
    Walk3Scan,
    Walk3FpScan,
    Walk3FpCheck,
    Walk3Check,
    Walk3Join,
    Found,
    Unlink,
    FitFwd,
    FitPlain,
    FitJoin,
    NoSplit,
    Split,
    SplitInsCall,
    SplitInsFwdPrime,
    SplitInsFwd,
    SplitInsBwdPrime,
    SplitInsBwd,
    SplitInsLink,
    SplitPlainCall,
    Ins2FwdPrime,
    Ins2Fwd,
    Ins2BwdPrime,
    Ins2Bwd,
    Ins2Link,
    Common,
    Fail,
    /// `0x252f08`: fail-path lock-word reload (+1). The success path
    /// skips this block: Common's `+10` tail (0x252ec4) already
    /// reloaded V0 for free.
    UnlockCheck,
    /// `0x252f0c`: lock-state dispatch both exits share.
    UnlockJoin,
    RetryCheck,
    /// Normal exit via `0x252f2c`: the `V0 = S5` move (and its +1), then
    /// the shared `0x252f30` tail.
    Epilogue,
    /// `0x252f30` tail: the frame restore both exits share. The
    /// negative-size early exit enters here straight from the entry
    /// (skipping the `V0 = S5` move); `V0` keeps the entry delay-slot 0.
    EpilogueTail,
}

/// `void *MEM_HeapAllocAligned(const char *name, size_t size, int align,
/// int a3, void *heap_or_null, int ignored)` (0x252a10): the heap
/// allocator core. Picks the heap class from `heap_or_null` (or the
/// default heap word at `GP-0x278` when null), rounds the request, walks
/// the free list for a fitting block, unlinks and optionally splits it,
/// and returns the aligned data pointer (`S5`), or null when no block
/// fits (invoking the out-of-memory callback at `GP-0x274` first, and
/// retrying when it reports the heap changed). `name` and the last flag
/// pass through untouched: the core never reads them. Re-entered at its
/// loop heads after an unwind.
pub fn mem_heap_alloc_aligned(c: &mut Cpu) {
    guest(c, |c| {
        let mut st = match c.pc {
            heap_alloc::ENTRY => Heap::Entry,
            heap_alloc::RETRY => Heap::LockCheckJoin,
            heap_alloc::WALK1_RELOAD => Heap::Walk1Reload,
            heap_alloc::WALK1_SCAN => Heap::Walk1Scan,
            heap_alloc::WALK2_RELOAD => Heap::Walk2Reload,
            heap_alloc::WALK2_SCAN => Heap::Walk2Scan,
            heap_alloc::WALK3_SCAN => Heap::Walk3Scan,
            heap_alloc::WALK3_FP_SCAN => Heap::Walk3FpScan,
            heap_alloc::INS1_FWD => Heap::SplitInsFwd,
            heap_alloc::INS1_BWD => Heap::SplitInsBwd,
            heap_alloc::INS2_FWD => Heap::Ins2Fwd,
            heap_alloc::INS2_BWD => Heap::Ins2Bwd,
            pc => {
                c.jump_out(pc, heap_alloc::ENTRY, heap_alloc::END);
                return Ok(());
            }
        };
        // The frame is opened at the entry stage; resume stages reuse it.
        let frame = Frame { size: 0xd0 };
        loop {
            st = match st {
                Heap::Entry => {
                    c.cycles += 25;
                    Frame::open(c, 0xd0);
                    c.g[V1][0] = 0x4d0000;
                    frame.save(c, S6, 0x60);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2438));
                    c.g[S6][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x278));
                    frame.save(c, S5, 0x70);
                    if c.g[T0][0] != 0 {
                        c.g[S6][0] = c.g[T0][0];
                    }
                    frame.save(c, S0, 0xc0);
                    c.g[V0][0] = c.g[S6][0] & 0x3f;
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S3, 0x90);
                    frame.save(c, S4, 0x80);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    frame.save(c, S2, 0xa0);
                    c.g[V0][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    frame.save(c, S7, 0x50);
                    c.g[S5][0] = 0u64;
                    frame.save(c, FP, 0x40);
                    frame.save_ra(c, 0x30);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[S2][0] = c.lw(c.g[V0][0] as u32);
                    let t = (c.g[A1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        Heap::EpilogueTail
                    } else {
                        Heap::LockCheck
                    }
                }
                Heap::LockCheck => {
                    // 0x252a74: the heap lock, taken when the class
                    // word says so. The RETRY resume skips this block
                    // and enters LockCheckJoin directly (see below).
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S6][0] & 0x200;
                    c.g[FP][0] = c.g[S6][0] & 0x100;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] =
                        c.lw((c.g[S2][0] as u32).wrapping_add(MEM_CLS_LOCKED));
                    Heap::LockCheckJoin
                }
                Heap::LockCheckJoin => {
                    // 0x252a88: lock-state dispatch both entries share.
                    // The retry path (0x252f20) enters here with V0
                    // already reloaded, skipping the block above.
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                    if t {
                        Heap::RoundSize
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0025_2a98);
                        c.g[A0][0] = sx(
                            (c.g[S2][0] as u32).wrapping_add(MEM_CLS_MUTEX),
                        );
                        call_ptr(c, callee::EA_MUTEX_LOCK, 0x0025_2a98)?;
                        Heap::RoundSize
                    }
                }
                Heap::RoundSize => {
                    // Quantum/alignment mask (S3), overhead, and the
                    // rounded request (A2).
                    c.cycles += 4;
                    c.g[V0][0] =
                        c.lw((c.g[S2][0] as u32).wrapping_add(MEM_CLS_QUANTUM));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[S6][0] & 0x400;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S3][0] = c.g[S4][0];
                        if !t {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(
                                (c.g[S2][0] as u32).wrapping_add(MEM_CLS_ALIGN),
                            );
                            c.g[S3][0] =
                                sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        }
                    }
                    c.cycles += 15;
                    c.g[A0][0] = 0u64;
                    c.g[RA][0] = sx(0x0025_2ad4);
                    c.g[A1][0] = c.g[S6][0];
                    callee::MEM_GET_HEAP_BLOCK_OVERHEAD.call(c, 0x0025_2ad4)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.g[A0][0];
                    c.g[A3][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = 0x7;
                    c.g[V1][0] = 0x8;
                    c.g[V0][0] =
                        ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = sx(
                        (c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32),
                    );
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    if t {
                        Heap::Walk3First
                    } else {
                        Heap::Walk1First
                    }
                }
                Heap::Walk1First => {
                    // Bisection setup: forward (size) walk or backward
                    // walk by the FP bit; the plain scan when the size
                    // bit is clear is chosen above.
                    c.cycles += 5;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V0][0] =
                        ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[T1][0] = 0u64;
                    let t = c.g[FP][0] != 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = 0u64;
                    }
                    if t {
                        Heap::Walk2First
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(
                            (c.g[S2][0] as u32).wrapping_add(MEM_CLS_NEXT),
                        );
                        c.g[V0][0] =
                            c.lw((c.g[V1][0] as u32).wrapping_add(MEM_BLK_SIZE));
                        c.g[V0][0] =
                            sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                        c.g[V0][0] = ((c.g[A0][0] as i64)
                            < (c.g[V0][0] as i64))
                            as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw(
                                (c.g[V1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                            );
                            Heap::Walk1Scan
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
                            c.g[A0][0] = 0x4253;
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[S0][0] = c.g[T1][0];
                            if t {
                                Heap::Found
                            } else {
                                c.cycles += 2;
                                c.g[T1][0] = c.g[V1][0];
                                Heap::Walk1Reload
                            }
                        }
                    }
                }
                Heap::Walk1Reload => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(
                        (c.g[T1][0] as u32).wrapping_add(MEM_BLK_SIZE),
                    );
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[V1][0] = c.lw(
                        (c.g[V1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    Heap::Walk1Scan
                }
                Heap::Walk1Scan => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(
                            (c.g[V1][0] as u32).wrapping_add(MEM_BLK_SIZE),
                        );
                        c.g[V0][0] =
                            sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                        c.g[V0][0] = ((c.g[A0][0] as i64)
                            < (c.g[V0][0] as i64))
                            as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw(
                                (c.g[V1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                            );
                            if back_edge_unwinds(c, heap_alloc::WALK1_SCAN) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    Heap::Walk1Check
                }
                Heap::Walk1Check => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
                    c.g[A0][0] = 0x4253;
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[T1][0] = c.g[V1][0];
                        if back_edge_unwinds(c, heap_alloc::WALK1_RELOAD) {
                            return Ok(());
                        }
                        Heap::Walk1Reload
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[T1][0];
                        Heap::Found
                    }
                }
                Heap::Walk2First => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_PREV),
                    );
                    Heap::Walk2Scan
                }
                Heap::Walk2Reload => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(
                        (c.g[T1][0] as u32).wrapping_add(MEM_BLK_SIZE),
                    );
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[V1][0] = c.lw(
                        (c.g[V1][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    Heap::Walk2Scan
                }
                Heap::Walk2Scan => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(
                            (c.g[V1][0] as u32).wrapping_add(MEM_BLK_SIZE),
                        );
                        c.g[V0][0] =
                            sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                        c.g[V0][0] = ((c.g[A0][0] as i64)
                            < (c.g[V0][0] as i64))
                            as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw(
                                (c.g[V1][0] as u32).wrapping_add(MEM_FREE_PREV),
                            );
                            if back_edge_unwinds(c, heap_alloc::WALK2_SCAN) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    Heap::Walk2Check
                }
                Heap::Walk2Check => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
                    c.g[A0][0] = 0x4253;
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[T1][0] = c.g[V1][0];
                        if back_edge_unwinds(c, heap_alloc::WALK2_RELOAD) {
                            return Ok(());
                        }
                        Heap::Walk2Reload
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[T1][0];
                        Heap::Found
                    }
                }
                Heap::Walk3First => {
                    // Plain scan without the size bit: the forward walk,
                    // or its forward-pointer variant with the FP bit.
                    c.cycles += 2;
                    let t = c.g[FP][0] != 0u64;
                    c.g[T1][0] = c.g[S7][0];
                    if t {
                        c.cycles += 1;
                        c.g[T1][0] = c.lw(
                            (c.g[T1][0] as u32).wrapping_add(MEM_FREE_PREV),
                        );
                        Heap::Walk3FpScan
                    } else {
                        c.cycles += 2;
                        c.g[T1][0] = c.lw(
                            (c.g[T1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        );
                        Heap::Walk3Scan
                    }
                }
                Heap::Walk3Scan => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(
                            (c.g[T1][0] as u32).wrapping_add(MEM_BLK_SIZE),
                        );
                        c.g[V0][0] = ((c.g[V0][0] as i64)
                            < (c.g[A2][0] as i64))
                            as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[T1][0] = c.lw(
                                (c.g[T1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                            );
                            if back_edge_unwinds(c, heap_alloc::WALK3_SCAN) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    Heap::Walk3Check
                }
                Heap::Walk3FpScan => {
                    // Forward-pointer variant: walks the +0x14 chain while
                    // the node sits below the caller's S1 (the entry S1:
                    // nothing writes it on this path).
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[A3][0] < c.g[S1][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A3][0] = c.lw(
                                (c.g[A3][0] as u32).wrapping_add(MEM_FREE_PREV),
                            );
                            if back_edge_unwinds(c, heap_alloc::WALK3_FP_SCAN)
                            {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    Heap::Walk3FpCheck
                }
                Heap::Walk3FpCheck => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu(c.g[T1][0] as u32);
                    Heap::Walk3Join
                }
                Heap::Walk3Check => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lhu(c.g[T1][0] as u32);
                    Heap::Walk3Join
                }
                Heap::Walk3Join => {
                    c.cycles += 3;
                    c.g[S0][0] = 0u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x4253;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[T1][0];
                    }
                    Heap::Found
                }
                Heap::Found => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = 0x424f;
                    if t {
                        Heap::Fail
                    } else {
                        Heap::Unlink
                    }
                }
                Heap::Unlink => {
                    // Unhook the found block and retag it allocated.
                    c.cycles += 15;
                    c.g[V1][0] = c.lw(
                        (c.g[S0][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    c.g[V0][0] = c.lw(
                        (c.g[S0][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    c.g[S1][0] = 0u64;
                    c.g[A1][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_SIZE));
                    c.sw(
                        (c.g[V1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        c.g[V0][0],
                    );
                    c.sw(
                        (c.g[V0][0] as u32).wrapping_add(MEM_FREE_PREV),
                        c.g[V1][0],
                    );
                    c.sh(c.g[S0][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lhu(
                        (c.g[S0][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                    );
                    c.g[V1][0] = c.lw(
                        (c.g[S0][0] as u32).wrapping_add(MEM_BLK_NEXT),
                    );
                    c.g[V0][0] = c.g[V0][0] & 0xbfff;
                    c.sh(
                        (c.g[S0][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                        c.g[V0][0],
                    );
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_PREV));
                    let t = c.g[FP][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        Heap::FitPlain
                    } else {
                        Heap::FitFwd
                    }
                }
                Heap::FitFwd => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx(
                        (c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32),
                    );
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32),
                    );
                    c.g[V1][0] = !(0u64 | c.g[S3][0]);
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32),
                    );
                    c.g[S1][0] = c.g[S0][0];
                    Heap::FitJoin
                }
                Heap::FitPlain => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx(
                        (c.g[S0][0] as u32).wrapping_add(c.g[T2][0] as u32),
                    );
                    c.g[V1][0] = !(0u64 | c.g[S3][0]);
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32),
                    );
                    Heap::FitJoin
                }
                Heap::FitJoin => {
                    // Aligned data address (S5) and the split decision.
                    c.cycles += 12;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[S5][0] = sx(
                        (c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32),
                    );
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx(
                        (c.g[A1][0] as u32).wrapping_sub(c.g[A2][0] as u32),
                    );
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x10));
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0x8), c.g[V0][0]);
                    c.g[S3][0] = c.g[S0][0];
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64))
                        as u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0x4), c.g[V0][0]);
                    if t {
                        Heap::NoSplit
                    } else {
                        Heap::Split
                    }
                }
                Heap::NoSplit => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(
                        (c.g[S0][0] as u32).wrapping_add(MEM_BLK_NEXT),
                    );
                    c.sw(
                        (c.g[V0][0] as u32).wrapping_add(MEM_BLK_PREV),
                        c.g[S0][0],
                    );
                    c.g[V1][0] = c.lw(
                        (c.g[S0][0] as u32).wrapping_add(MEM_BLK_PREV),
                    );
                    c.sw(
                        (c.g[V1][0] as u32).wrapping_add(MEM_BLK_NEXT),
                        c.g[S0][0],
                    );
                    Heap::Common
                }
                Heap::Split => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        Heap::SplitPlainCall
                    } else {
                        Heap::SplitInsCall
                    }
                }
                Heap::SplitInsCall => {
                    // Forward-pool split: header the new block, bisect
                    // the free list for its slot, and link it in.
                    c.cycles += 25;
                    c.g[V0][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.sw(
                        (c.g[V0][0] as u32).wrapping_add(MEM_BLK_PREV),
                        c.g[S0][0],
                    );
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T2][0] = c.g[S0][0];
                    c.g[V0][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0x4), c.g[S1][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0x8), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0025_2d10);
                    c.g[T1][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_PREV));
                    callee::MEM_INIT_BLOCK_HEADER.call(c, 0x0025_2d10)?;
                    c.g[A0][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_NEXT),
                    );
                    c.g[A3][0] = c.g[S7][0];
                    c.g[V0][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_PREV),
                    );
                    c.g[A2][0] = c.g[A3][0];
                    c.g[A1][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32),
                    );
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32),
                    );
                    c.g[V0][0] =
                        sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                    c.g[A0][0] = sx(
                        (c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32),
                    );
                    c.g[A0][0] = (c.g[A0][0] < c.g[S1][0]) as u64;
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = sx(
                        (c.g[A1][0] as u32).wrapping_sub(c.g[S1][0] as u32),
                    );
                    if t {
                        Heap::SplitInsBwdPrime
                    } else {
                        Heap::SplitInsFwdPrime
                    }
                }
                Heap::SplitInsFwdPrime => {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    Heap::SplitInsFwd
                }
                Heap::SplitInsFwd => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[S1][0] < c.g[A2][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A2][0] = c.lw(
                                (c.g[A2][0] as u32).wrapping_add(MEM_FREE_PREV),
                            );
                            if back_edge_unwinds(c, heap_alloc::INS1_FWD) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    c.cycles += 2;
                    c.g[A3][0] = c.lw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    Heap::SplitInsLink
                }
                Heap::SplitInsBwdPrime => {
                    c.cycles += 1;
                    c.g[A3][0] = c.lw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    Heap::SplitInsBwd
                }
                Heap::SplitInsBwd => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[A3][0] < c.g[S1][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A3][0] = c.lw(
                                (c.g[A3][0] as u32).wrapping_add(MEM_FREE_NEXT),
                            );
                            if back_edge_unwinds(c, heap_alloc::INS1_BWD) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    c.cycles += 2;
                    c.g[A2][0] = c.lw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    Heap::SplitInsLink
                }
                Heap::SplitInsLink => {
                    c.cycles += 11;
                    c.sw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_SIZE), c.g[A1][0]);
                    c.g[V1][0] = 0x4246;
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        c.g[A3][0],
                    );
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(MEM_FREE_PREV),
                        c.g[A2][0],
                    );
                    c.sw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        c.g[S1][0],
                    );
                    c.sw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_PREV),
                        c.g[S1][0],
                    );
                    c.sh(c.g[S1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lhu(
                        (c.g[S1][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                    );
                    c.g[V0][0] = c.g[V0][0] | 0x4000;
                    c.sh(
                        (c.g[S1][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                        c.g[V0][0],
                    );
                    Heap::Common
                }
                Heap::SplitPlainCall => {
                    // Plain split: header the tail, bisect, and link it.
                    c.cycles += 29;
                    c.g[V0][0] = sx(
                        (c.g[S5][0] as u32).wrapping_add((c.g[A3][0] as u32)),
                    );
                    c.g[A0][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.g[V1][0] = !(0u64 | c.g[S4][0]);
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32),
                    );
                    c.g[S1][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[A0][0] as u32).wrapping_add(MEM_BLK_PREV), c.g[S1][0]);
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_PREV));
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = c.g[S0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(MEM_BLK_NEXT), c.g[S0][0]);
                    c.g[T2][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x8));
                    c.g[RA][0] = sx(0x0025_2e00);
                    callee::MEM_INIT_BLOCK_HEADER.call(c, 0x0025_2e00)?;
                    c.g[A0][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_NEXT),
                    );
                    c.g[A3][0] = c.g[S7][0];
                    c.g[V0][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_PREV),
                    );
                    c.g[A2][0] = c.g[A3][0];
                    c.g[A1][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32),
                    );
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
                    c.g[V0][0] = sx(
                        (c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32),
                    );
                    c.g[V0][0] =
                        sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                    c.g[A0][0] = sx(
                        (c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32),
                    );
                    c.g[A0][0] = (c.g[A0][0] < c.g[S1][0]) as u64;
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = sx(
                        (c.g[A1][0] as u32).wrapping_sub(c.g[S1][0] as u32),
                    );
                    if t {
                        Heap::Ins2BwdPrime
                    } else {
                        Heap::Ins2FwdPrime
                    }
                }
                Heap::Ins2FwdPrime => {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    Heap::Ins2Fwd
                }
                Heap::Ins2Fwd => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[S1][0] < c.g[A2][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A2][0] = c.lw(
                                (c.g[A2][0] as u32).wrapping_add(MEM_FREE_PREV),
                            );
                            if back_edge_unwinds(c, heap_alloc::INS2_FWD) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    c.cycles += 2;
                    c.g[A3][0] = c.lw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    Heap::Ins2Link
                }
                Heap::Ins2BwdPrime => {
                    c.cycles += 1;
                    c.g[A3][0] = c.lw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_NEXT),
                    );
                    Heap::Ins2Bwd
                }
                Heap::Ins2Bwd => {
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[A3][0] < c.g[S1][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A3][0] = c.lw(
                                (c.g[A3][0] as u32).wrapping_add(MEM_FREE_NEXT),
                            );
                            if back_edge_unwinds(c, heap_alloc::INS2_BWD) {
                                return Ok(());
                            }
                        } else {
                            break;
                        }
                    }
                    // 0x252e80: backward-tail single load (+1); the forward
                    // tail (0x252e54) is +2.
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_PREV),
                    );
                    Heap::Ins2Link
                }
                Heap::Ins2Link => {
                    c.cycles += 12;
                    c.sw((c.g[S1][0] as u32).wrapping_add(MEM_BLK_SIZE), c.g[A1][0]);
                    c.g[V1][0] = 0x4246;
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        c.g[A3][0],
                    );
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(MEM_FREE_PREV),
                        c.g[A2][0],
                    );
                    c.sw(
                        (c.g[A2][0] as u32).wrapping_add(MEM_FREE_NEXT),
                        c.g[S1][0],
                    );
                    c.sw(
                        (c.g[A3][0] as u32).wrapping_add(MEM_FREE_PREV),
                        c.g[S1][0],
                    );
                    c.sh(c.g[S1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lhu(
                        (c.g[S1][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                    );
                    c.g[V0][0] = c.g[V0][0] | 0x4000;
                    c.sh(
                        (c.g[S1][0] as u32).wrapping_add(MEM_BLK_FLAGS),
                        c.g[V0][0],
                    );
                    c.sw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_NEXT), c.g[S1][0]);
                    Heap::Common
                }
                Heap::Common => {
                    // Header the allocated block, then the unlock check.
                    c.cycles += 10;
                    c.g[T2][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_NEXT));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A3][0] = c.lw(
                        (c.g[S2][0] as u32).wrapping_add(MEM_CLS_OVERHEAD),
                    );
                    c.g[A1][0] = 0u64;
                    c.g[T1][0] =
                        c.lw((c.g[S0][0] as u32).wrapping_add(MEM_BLK_PREV));
                    c.g[T0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0025_2ee4);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    callee::MEM_INIT_BLOCK_HEADER.call(c, 0x0025_2ee4)?;
                    // 0x252ec4's tail: the lock-word reload rides in the
                    // +10 (no separate charge on this edge).
                    c.g[V0][0] =
                        c.lw((c.g[S2][0] as u32).wrapping_add(MEM_CLS_LOCKED));
                    Heap::UnlockJoin
                }
                Heap::Fail => {
                    // No block fit: run the out-of-memory callback when
                    // one is installed, then fall through to unlock.
                    c.cycles += 3;
                    c.g[V0][0] =
                        c.lw((c.g[GP][0] as u32).wrapping_sub(0x274));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        Heap::UnlockCheck
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 0u64;
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0025_2f04);
                        c.g[A2][0] = c.g[S6][0];
                        call_ptr(c, t, 0x0025_2f04)?;
                        c.sw(
                            (c.g[SP][0] as u32).wrapping_add(0x1c),
                            c.g[V0][0],
                        );
                        Heap::UnlockCheck
                    }
                }
                Heap::UnlockCheck => {
                    // 0x252f08: fail-path entry only (Common bypasses it).
                    c.cycles += 1;
                    c.g[V0][0] =
                        c.lw((c.g[S2][0] as u32).wrapping_add(MEM_CLS_LOCKED));
                    Heap::UnlockJoin
                }
                Heap::UnlockJoin => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                    if t {
                        Heap::RetryCheck
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0025_2f1c);
                        c.g[A0][0] = sx(
                            (c.g[S2][0] as u32).wrapping_add(MEM_CLS_MUTEX),
                        );
                        call_ptr(c, callee::EA_MUTEX_UNLOCK, 0x0025_2f1c)?;
                        c.g[V1][0] =
                            c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        Heap::RetryCheck
                    }
                }
                Heap::RetryCheck => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(
                            (c.g[S2][0] as u32).wrapping_add(MEM_CLS_LOCKED),
                        );
                        if back_edge_unwinds(c, heap_alloc::RETRY) {
                            return Ok(());
                        }
                        Heap::LockCheckJoin
                    } else {
                        Heap::Epilogue
                    }
                }
                Heap::Epilogue => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S5][0];
                    Heap::EpilogueTail
                }
                Heap::EpilogueTail => {
                    c.cycles += 12;
                    frame.restore(c, S0, 0xc0);
                    frame.restore(c, S1, 0xb0);
                    frame.restore(c, S2, 0xa0);
                    frame.restore(c, S3, 0x90);
                    frame.restore(c, S4, 0x80);
                    frame.restore(c, S5, 0x70);
                    frame.restore(c, S6, 0x60);
                    frame.restore(c, S7, 0x50);
                    frame.restore(c, FP, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
            };
        }
    })
}

/// `void EA_MemSyncLock(void)` (0x317ee0): lock then unlock the global EA
/// memory mutex (`0x538b00`, the `MEM_MUTEX` word): a full barrier that
/// serializes this thread against concurrent allocators. Returns nothing.
pub fn ea_mem_sync_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = 0x540000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7500));
        c.g[RA][0] = sx(0x0031_7efc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::EA_MUTEX_LOCK, 0x0031_7efc)?;
        c.g[RA][0] = sx(0x0031_7f04);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::EA_MUTEX_UNLOCK, 0x0031_7f04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *EA_MemAllocTagged(size_t size, ...)` (0x317d48): allocate
/// `size` bytes with the default tag: the tag word from `GP+0xd94`, the
/// fixed name at `0x4a3e88`, flags 0. Its body is identical to
/// [`ea_mem_alloc_default`]'s apart from the return address; transcribed
/// as generated.
pub fn ea_mem_alloc_tagged(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd94));
        c.g[A1][0] = 0x4a0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3e88));
        c.g[RA][0] = sx(0x0031_7d64);
        c.g[A3][0] = 0u64;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0031_7d64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *EA_MemAllocDefault(size_t size)` (0x317d20): allocate `size`
/// bytes with the default tag: the tag word from `GP+0xd94`, the fixed
/// name at `0x4a3e88`, flags 0. Identical body to [`ea_mem_alloc_tagged`];
/// transcribed as generated.
pub fn ea_mem_alloc_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd94));
        c.g[A1][0] = 0x4a0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3e88));
        c.g[RA][0] = sx(0x0031_7d3c);
        c.g[A3][0] = 0u64;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0031_7d3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EA_MemReportStats_Wrapper(void)` (0x317f18): report the pool
/// statistics under the memory-manager lock (tail call into
/// `MEM_ReportStatsLocked`).
pub fn ea_mem_report_stats_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0031_7f28);
        callee::MEM_REPORT_STATS_LOCKED.call(c, 0x0031_7f28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{machine, LOG_RESULTS};
    use crate::difftest::{check, check_paths, fresh, run_generated, DATA, GAME_GP};
    use crate::difftest::fakes::FAKE;
    use crate::game::bx::cubicspline::tests::{
        heap_alloc_machine, heap_slot, write_filled,
    };
    use crate::game::fe::festatetrophyroom::tests::belib;

    /// Heap class under test (below the arena/Log regions; every word has
    /// a nonzero low byte).
    pub(crate) const HEAP_ALLOC_CLS: u32 = DATA + 0x2010;
    /// Free-list nodes N1..N4 (ends 0x40/0x50/size/0x100) and the mutex.
    pub(crate) const HEAP_ALLOC_N1: u32 = DATA + 0x3010;
    pub(crate) const HEAP_ALLOC_N2: u32 = DATA + 0x3110;
    pub(crate) const HEAP_ALLOC_N3: u32 = DATA + 0x3210;
    pub(crate) const HEAP_ALLOC_N4: u32 = DATA + 0x3310;
    pub(crate) const HEAP_ALLOC_MUTEX: u32 = DATA + 0x2410;
    /// Long insert-chain nodes above the split point (events test only).
    pub(crate) const HEAP_ALLOC_H1: u32 = DATA + 0x9010;
    pub(crate) const HEAP_ALLOC_H2: u32 = DATA + 0x9110;
    /// High found block for the forward-long insert variant.
    pub(crate) const HEAP_ALLOC_N3H: u32 = DATA + 0x8010;
    /// Heap table slot used by every seed.
    pub(crate) const HEAP_ALLOC_SLOT: u32 = 2;

    /// Modes of [`mem_heap_alloc_aligned_build`]: negative size, failed
    /// walk, forward-pointer plain scan, no-split walk1 (explicit and
    /// quantum mask), split walk3/walk1/walk2, split walk1 (class mask).
    pub(crate) const HEAP_ALLOC_MODES: u32 = 9;

    /// Build the heap fixture for `mode` (0..8). `t0src` selects the heap
    /// word source (T0 vs `GP-0x278`); `locked` sets the class lock bit
    /// (through the real mutexes). Returns S6.
    pub(crate) fn mem_heap_alloc_aligned_build(
        c: &mut Cpu,
        mode: u32,
        t0src: bool,
        locked: bool,
    ) -> u32 {
        // Hard pre-fill: every word the port reads or writes is nonzero.
        for w in 0..0x40u32 {
            write_filled(c, HEAP_ALLOC_N1 + 4 * w, 0x1000 + w);
            write_filled(c, HEAP_ALLOC_N2 + 4 * w, 0x2000 + w);
            write_filled(c, HEAP_ALLOC_N3 + 4 * w, 0x3000 + w);
            write_filled(c, HEAP_ALLOC_N4 + 4 * w, 0x4000 + w);
            write_filled(c, HEAP_ALLOC_CLS + 4 * w, 0x5000 + w);
        }
        let n3size = match mode {
            3 | 4 => 0x80,
            8 => 0xd0,
            _ => 0xc0,
        };
        // Node tags and ends (the walk compares +4 against the rounded
        // size and stops at the 0x4253 sentinel).
        c.bus.write16(HEAP_ALLOC_N1, 0x4246);
        c.bus.write32(HEAP_ALLOC_N1 + 0x4, 0x40);
        c.bus.write32(HEAP_ALLOC_N1 + 0x10, HEAP_ALLOC_N2);
        c.bus.write32(HEAP_ALLOC_N1 + 0x14, 0);
        c.bus.write16(HEAP_ALLOC_N2, 0x4246);
        c.bus.write32(HEAP_ALLOC_N2 + 0x4, 0x50);
        c.bus.write32(HEAP_ALLOC_N2 + 0x10, HEAP_ALLOC_N3);
        c.bus.write32(HEAP_ALLOC_N2 + 0x14, HEAP_ALLOC_N1);
        c.bus.write16(HEAP_ALLOC_N3, 0x4246);
        c.bus.write32(HEAP_ALLOC_N3 + 0x4, n3size);
        c.bus.write32(HEAP_ALLOC_N3 + 0x10, HEAP_ALLOC_N4);
        c.bus.write32(HEAP_ALLOC_N3 + 0x14, HEAP_ALLOC_N2);
        c.bus.write16(HEAP_ALLOC_N4, 0x4253);
        c.bus.write32(HEAP_ALLOC_N4 + 0x4, 0x100);
        c.bus.write32(HEAP_ALLOC_N4 + 0x10, 0);
        c.bus.write32(HEAP_ALLOC_N4 + 0x14, HEAP_ALLOC_N3);
        // Class words: heads, quantum, align, overhead, lock, mutex.
        c.bus.write32(
            HEAP_ALLOC_CLS + 0x20,
            if mode == 1 { HEAP_ALLOC_N4 } else { HEAP_ALLOC_N1 },
        );
        c.bus.write32(
            HEAP_ALLOC_CLS + 0x24,
            if mode == 2 || mode == 7 {
                // Backward-walk tail: starting from the head runs the
                // generated walk off the list (prev 0) into a spin.
                HEAP_ALLOC_N4
            } else {
                HEAP_ALLOC_N1
            },
        );
        c.bus.write32(HEAP_ALLOC_CLS + 0x28, 0x10);
        c.bus.write32(HEAP_ALLOC_CLS + 0x2c, 0x20);
        c.bus.write32(HEAP_ALLOC_CLS + 0x30, 0x10);
        c.bus.write32(HEAP_ALLOC_CLS + 0x38, locked as u32);
        c.bus.write32(HEAP_ALLOC_CLS + 0x3c, HEAP_ALLOC_MUTEX);
        c.bus.write32(heap_slot(HEAP_ALLOC_SLOT), HEAP_ALLOC_CLS);
        let flags = match mode {
            0 | 1 | 3 | 4 | 6 => 0x200,
            2 => 0x100,
            5 => 0x000,
            7 => 0x300,
            _ => 0x600,
        };
        let s6 = HEAP_ALLOC_SLOT | flags;
        c.bus.write32(GAME_GP - 0x278, s6);
        c.bus.write32(GAME_GP - 0x274, 0);
        c.g[T0][0] = if t0src { s6 as u64 } else { 0 };
        // S1 is entry garbage the plain-scan FP variant compares against:
        // 0 ends its walk at once (a longer walk would chase garbage
        // links, so that arm stays uncovered).
        c.g[S1][0] = 0;
        // The entry name is never read; the entry A3 offsets the aligned
        // address and must keep the block stores inside the span.
        c.g[A0][0] = (DATA + 0x910) as u64;
        c.g[A1][0] = if mode == 0 { 0xffff_ffff_ffff_ffff } else { 0x40 };
        c.g[A2][0] = if mode == 4 || mode == 8 { 0 } else { 8 };
        c.g[A3][0] = 0x20;
        s6
    }

    /// Matches-test setup: `seed % 9` the mode, `seed / 9 % 2` the heap
    /// word source, `seed / 18 % 2` the lock bit. All four walk
    /// directions, the FP-plain arm, split/no-split, the empty walk and
    /// the negative size.
    pub(crate) fn mem_heap_alloc_aligned_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x252a_10, 0);
        mem_heap_alloc_aligned_build(
            c,
            seed % HEAP_ALLOC_MODES,
            seed / 9 % 2 == 1,
            seed / 18 % 2 == 1,
        );
        c.pc = 0x0025_2a10;
    }

    /// Totals (generated costs, all 72 seeds terminate): mode 0 negative
    /// size 37; mode 1 no-split walk1 100 (194 locked); mode 2 FP-plain
    /// 99 (200 locked); mode 3 split walk1 179 (273 locked); mode 4
    /// split walk2 180 (281 locked); mode 5 split walk3 221 (314
    /// locked); mode 6 split walk1 236 (333 locked); mode 7 backward
    /// walk from the tail, found-tag empty, fail path 105 (205 locked);
    /// mode 8 split walk1 with the class mask 240 (336 locked). Locked
    /// seeds take the real mutexes; their costs are seed-hash dependent
    /// but deterministic, so all are pinned.
    #[test]
    fn mem_heap_alloc_aligned_matches() {
        check_paths(
            ee::s06::f_00252a10__generated,
            mem_heap_alloc_aligned,
            72,
            &[
                37, 99, 100, 105, 179, 180, 194, 200, 205, 221, 236, 240,
                273, 281, 314, 333, 336,
            ],
            &mem_heap_alloc_aligned_setup,
        );
    }

    driven!(gen_mem_heap_alloc_aligned, ee::s06::f_00252a10__generated);

    driven!(hand_mem_heap_alloc_aligned, mem_heap_alloc_aligned);

    /// Events setup: `seed % 8` the shape (walk1, walk2, walk3, walk1
    /// with a long forward insert, walk3 with a long backward insert,
    /// empty walk, no-split walk1, empty walk with a forced callback).
    /// `seed / 8 % 2` the lock bit (timed targets only run unlocked, so
    /// the unknown mutex costs cannot shift them), `seed / 16 % 2` the
    /// heap word source, `seed / 64 % 3` the targeted head. The first
    /// executed check fires on its own; later heads are timed by prefix
    /// cycles (scan 79/75/72, reload 82/90, second scan 75/82, insert
    /// 186/169, retry 87): earlier checks see `cycles < next_event` and
    /// stay quiet, so a mistimed constant only moves the resume, never
    /// breaks agreement.
    pub(crate) fn mem_heap_alloc_aligned_events_setup(
        c: &mut Cpu,
        seed: u32,
    ) -> Events {
        let mut r = machine(c, seed, 0x252a_10, 0);
        let shape = seed % 8;
        let locked = seed / 8 % 2 == 1 && shape != 7;
        let (mode, retry) = match shape {
            0 => (6, false),
            1 => (7, false),
            2 => (5, false),
            3 => (6, false),
            4 => (5, false),
            5 => (1, false),
            6 => (3, false),
            _ => (1, true),
        };
        mem_heap_alloc_aligned_build(c, mode, seed / 16 % 2 == 1, locked);
        if shape == 3 || shape == 4 {
            // Long insert chain above the split point: H1 -> H2 -> N1.
            for w in 0..0x40u32 {
                write_filled(c, HEAP_ALLOC_H1 + 4 * w, 0x6000 + w);
                write_filled(c, HEAP_ALLOC_H2 + 4 * w, 0x7000 + w);
            }
            c.bus.write32(HEAP_ALLOC_H1 + 0x4, 0x40);
            c.bus.write32(HEAP_ALLOC_H1 + 0x14, HEAP_ALLOC_H2);
            c.bus.write32(HEAP_ALLOC_H2 + 0x4, 0x40);
            c.bus.write32(HEAP_ALLOC_H2 + 0x14, HEAP_ALLOC_N1);
            c.bus.write32(HEAP_ALLOC_CLS + 0x24, HEAP_ALLOC_H1);
        }
        if shape == 3 {
            // High found block: the insert midpoint then falls below the
            // split point and the forward insert walks twice.
            for w in 0..0x40u32 {
                write_filled(c, HEAP_ALLOC_N3H + 4 * w, 0x8000 + w);
            }
            c.bus.write16(HEAP_ALLOC_N3H, 0x4246);
            c.bus.write32(HEAP_ALLOC_N3H + 0x4, 0xc0);
            c.bus.write32(HEAP_ALLOC_N3H + 0x10, HEAP_ALLOC_N4);
            c.bus.write32(HEAP_ALLOC_N3H + 0x14, HEAP_ALLOC_N2);
            c.bus.write32(HEAP_ALLOC_N2 + 0x10, HEAP_ALLOC_N3H);
            c.bus.write32(HEAP_ALLOC_N4 + 0x14, HEAP_ALLOC_N3H);
        }
        if retry {
            // Force the first callback to report a changed heap (the
            // retry arm), the second to use its hash (exit).
            c.bus.write32(GAME_GP - 0x274, FAKE);
            c.bus.write32(LOG_RESULTS, 1);
        }
        c.pc = 0x0025_2a10;
        let m = schedule(c, &mut r, seed, 0x0025_2a10);
        if m != Events::Off && !locked {
            let base = c.cycles;
            let at = match (shape, seed / 64 % 3) {
                // Walk1 reload (scan step at +79 stays quiet).
                (0 | 6, 1) => base + 82,
                // Walk2 second scan / reload.
                (1, 1) => base + 90,
                (1, 2) => base + 76,
                // Walk3 second scan.
                (2, 1) => base + 75,
                // Long inserts (walk checks below +100 stay quiet).
                (3 | 4, 1) => base + 100,
                // Walk1 reload on the long shapes.
                (3, 2) => base + 82,
                // Retry check (no earlier check exists on this shape).
                (7, _) => base + 87,
                _ => return m,
            };
            c.next_event = at;
        }
        m
    }

    /// Every loop head under both driven sides: first-edge scans fire on
    /// their own; reloads, second scans, long inserts and the retry arm
    /// are timed above. Expected resumes: scan heads on every walk
    /// shape, reload heads on walk1/walk2, insert heads on the long
    /// shapes, the retry head on shape 7. The FP-plain scan head
    /// (0x252c0c) never fires (S1 = 0 exits its walk at once) and
    /// post-call addresses never resume in tests (no callee unwinds).
    #[test]
    fn mem_heap_alloc_aligned_events() {
        check_events(
            gen_mem_heap_alloc_aligned,
            hand_mem_heap_alloc_aligned,
            96,
            &mem_heap_alloc_aligned_events_setup,
        );
    }

    /// Sync lock: straight-line lock/unlock of the global mutex through
    /// the real mutex code; single arm.
    #[test]
    fn ea_mem_sync_lock_matches() {
        check(
            ee::s10::f_00317ee0__generated,
            ea_mem_sync_lock,
            40,
            &|c, seed| {
                machine(c, seed, 0x317e_e0, 0);
                c.pc = 0x0031_7ee0;
            },
        );
    }

    /// Tagged alloc: the wrapper passes the tag word, fixed name and zero
    /// flags into the real named allocator (real DLHeap on the heap
    /// fixture, so path totals vary and are not pinned).
    #[test]
    fn ea_mem_alloc_tagged_matches() {
        check(
            ee::s10::f_00317d48__generated,
            ea_mem_alloc_tagged,
            64,
            &|c, seed| {
                heap_alloc_machine(c, seed, 0x317d_48);
                c.bus.write32(
                    GAME_GP + 0xd94,
                    [0x0000_0001, 0x0000_0401, 0x0100_0001, 0x1000_0001]
                        [(seed / 8 % 4) as usize],
                );
                c.g[A0][0] =
                    [0x10, 0x11, 0x40, 0x100, 0x1000, 0x8000, 0x20000, 0x100000]
                        [(seed % 8) as usize];
                c.g[A1][0] = (DATA + 0x901) as u64;
                c.g[A3][0] = seed as u64;
                c.pc = 0x0031_7d48;
            },
        );
    }

    /// Default alloc: identical body to the tagged wrapper (apart from
    /// the return address); same fixture shape.
    #[test]
    fn ea_mem_alloc_default_matches() {
        check(
            ee::s10::f_00317d20__generated,
            ea_mem_alloc_default,
            64,
            &|c, seed| {
                heap_alloc_machine(c, seed, 0x317d_20);
                c.bus.write32(
                    GAME_GP + 0xd94,
                    [0x0000_0001, 0x0000_0401, 0x0100_0001, 0x1000_0001]
                        [(seed / 8 % 4) as usize],
                );
                c.g[A0][0] =
                    [0x10, 0x11, 0x40, 0x100, 0x1000, 0x8000, 0x20000, 0x100000]
                        [(seed % 8) as usize];
                c.g[A1][0] = (DATA + 0x901) as u64;
                c.g[A3][0] = seed as u64;
                c.pc = 0x0031_7d20;
            },
        );
    }

    /// Report-stats wrapper: straight-line call into the real locked
    /// reporter; the pool flag steers its empty/non-empty arms ( pool
    /// heads stay zero, as in the reporter's own test).
    #[test]
    fn ea_mem_report_stats_wrapper_matches() {
        check(
            ee::s10::f_00317f18__generated,
            ea_mem_report_stats_wrapper,
            48,
            &|c, seed| {
                machine(c, seed, 0x317f_18, 0);
                belib(c);
                c.bus.write32(GAME_GP + 0xdd8, seed % 2);
                for off in
                    [0x2a60u32, 0x2a68, 0x2a6c, 0x2a70, 0x2a78, 0x2a7c, 0x2a84]
                {
                    c.bus.write32(GAME_GP + off, 0);
                }
                c.pc = 0x0031_7f18;
            },
        );
    }
}
