//! Staging module oc357 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc357`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_e818, dlheap_malloc, "DLHeap_Malloc")];

// ---------------------------------------------------------------------------
// DLHeap_Malloc (0x31e818)

/// dlmalloc state fields `DLHeap_Malloc` uses (after `max_fast` with its
/// flags at +0 and the fast bins from +4): the top chunk (+0x30), the last
/// remainder (+0x34), the unsorted bin header (+0x38, `fd` +0x40, `bk`
/// +0x44; the small and large bins follow, 8 bytes each, bin i's header at
/// +0x30 + 8i) and the bin map (+0x338, three words).
pub(crate) const DLM_TOP: u32 = 0x30;

pub(crate) const DLM_LAST_REMAINDER: u32 = 0x34;

pub(crate) const DLM_UNSORTED: u32 = 0x38;

pub(crate) const DLM_UNSORTED_FD: u32 = 0x40;

pub(crate) const DLM_UNSORTED_BK: u32 = 0x44;

pub(crate) const DLM_BINMAP: u32 = 0x338;

/// Chunk fields: size with flags (+4, bit 0 previous in use), `fd` (+8),
/// `bk` (+0xc).
pub(crate) const DLM_CHUNK_SIZE: u32 = 0x4;

pub(crate) const DLM_CHUNK_FD: u32 = 0x8;

pub(crate) const DLM_CHUNK_BK: u32 = 0xc;

/// `__getreent` (`int* __errno()`), through the function table.
pub(crate) const DLM_ERRNO: u32 = 0x0041_5fb8;

/// `DLHeap_Consolidate` (0x31eee8).
pub(crate) const DLM_CONSOLIDATE: Callee = Callee {
    addr: 0x0031_eee8,
    f: ee::s10::DLHeap_Consolidate,
};

/// `DLHeap_LargeBinIndex` (0x31e1d8).
pub(crate) const DLM_LARGE_BIN_INDEX: Callee = Callee {
    addr: 0x0031_e1d8,
    f: ee::s10::DLHeap_LargeBinIndex,
};

/// `DLHeap_SysAlloc` (0x31e2d8).
pub(crate) const DLM_SYS_ALLOC: Callee = Callee {
    addr: 0x0031_e2d8,
    f: ee::s10::DLHeap_SysAlloc,
};

/// 0x31ed30: restore s1..s7, fp and ra, close the 0xa0 frame and return.
fn dlheap_malloc_leave_restored(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore(c, FP, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// 0x31ed2c: restore s0, then the common exit.
fn dlheap_malloc_leave(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x90);
    dlheap_malloc_leave_restored(c, frame);
}

/// 0x31ecfc: exact fit; set "previous in use" on the chunk after the
/// victim (a0 = that chunk).
fn dlheap_malloc_set_next_inuse(c: &mut Cpu) {
    c.cycles += 4;
    let next = r32(c, A0);
    c.g[V1][0] = c.lw(next.wrapping_add(DLM_CHUNK_SIZE)) | 1;
    c.sw(next.wrapping_add(DLM_CHUNK_SIZE), c.g[V1][0]);
}

/// 0x31ecd4: the split remainder (a2) gets its size word (a0 = size | 1)
/// and its footer (at v1, the remainder size t1).
fn dlheap_malloc_remainder_tail(c: &mut Cpu) {
    c.cycles += 3;
    c.sw(r32(c, A2).wrapping_add(DLM_CHUNK_SIZE), c.g[A0][0]);
    c.sw(r32(c, V1), c.g[T1][0]);
}

/// `void* DLHeap_Malloc(DLHeap* heap /*a0*/, uint bytes /*a1*/)`
/// (0x31e818): dlmalloc 2.7's `malloc` on the heap's state (`*heap`, kept
/// in s2). Requests of 0xffffffe0 bytes or more set `ENOMEM` and return
/// null. The request is padded to a chunk size `nb` (s3, at least 16).
/// A heap that has never freed a chunk (state word bit 0 clear) is served
/// from the top chunk (a zero state word is initialised by
/// `DLHeap_Consolidate` first). Otherwise, in dlmalloc's order: the fast
/// bin's first chunk; for small sizes the small bin's last chunk; (large
/// sizes consolidate pending fast chunks first); then the unsorted chunks
/// are taken one by one: the last remainder is split for a small request
/// when it is the only one, an exact fit is returned, and every other chunk
/// is sorted into its small or (size-ordered) large bin, setting its bin
/// map bit. Large requests then take the smallest fitting chunk of their
/// own bin; then the bin map is scanned upwards for the first non-empty
/// bin (clearing bits of bins found empty), whose last chunk is split (a
/// remainder of 16 bytes or more goes to the unsorted bin and, for small
/// requests, becomes the last remainder). Last the top chunk is split, or
/// `DLHeap_SysAlloc` grows the heap. Returns the chunk + 8. Frame 0xa0 (ra
/// at 0, fp at 0x10, s7 .. s0 at 0x20 .. 0x90).
pub fn dlheap_malloc(c: &mut Cpu) {
    guest(c, |c| {
        const SIZE_MASK: u64 = 0xffff_ffff_ffff_fffc;
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, FP, 0x10);
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        c.g[V0][0] = (0xffff_ffff_ffff_ffdf < c.g[A1][0]) as u64;
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.lw(r32(c, FP));
        if c.g[V0][0] != 0 {
            // Too large: errno = ENOMEM, null.
            c.cycles += 6;
            call_ptr(c, DLM_ERRNO, 0x0031_e864)?;
            c.g[V1][0] = 0xc;
            c.sw(r32(c, V0), c.g[V1][0]);
            c.g[V0][0] = 0;
            dlheap_malloc_leave(c, frame);
            return Ok(());
        }
        // nb (s3): bytes + 4 rounded up to 16, at least 16.
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x13));
        c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
        c.g[S3][0] = 0x10;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_fff0;
            c.g[S3][0] = c.g[V1][0] & c.g[V0][0];
        }
        c.cycles += 5;
        let max_fast = c.lw(r32(c, S2));
        c.g[V1][0] = max_fast;
        c.g[V0][0] = (max_fast < c.g[S3][0]) as u64;
        'use_top: {
            if (max_fast ^ 1) & 1 != 0 {
                // Nothing ever freed: straight to the top chunk.
                c.cycles += 2;
                if max_fast == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    DLM_CONSOLIDATE.call(c, 0x0031_e8b8)?;
                }
                c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(DLM_TOP));
                break 'use_top;
            }
            c.cycles += 2;
            let not_fast = c.g[V0][0] != 0;
            c.g[S7][0] = (c.g[S3][0] < 0x100) as u64;
            if !not_fast {
                // Fast bin (nb >> 3) - 2: pop its first chunk.
                c.cycles += 6;
                c.g[V0][0] = sx(r32(c, S3) >> 1);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                c.g[S1][0] = c.lw(r32(c, A0));
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
                if c.g[S1][0] != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_FD));
                    c.sw(r32(c, A0), c.g[V1][0]);
                    dlheap_malloc_leave(c, frame);
                    return Ok(());
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x38));
            if c.g[S7][0] != 0 {
                // Small bin nb >> 3 (s6): take its last chunk.
                c.cycles += 5;
                c.g[A2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                c.g[S1][0] = c.lw(r32(c, A2).wrapping_add(4));
                c.g[T0][0] = sx(r32(c, A2).wrapping_sub(8));
                c.g[S6][0] = sx(r32(c, S3) >> 3);
                if c.g[S1][0] != c.g[T0][0] {
                    c.cycles += 9;
                    let victim = r32(c, S1);
                    c.g[A0][0] = sx(victim.wrapping_add(r32(c, S3)));
                    c.g[A3][0] = c.lw(victim.wrapping_add(DLM_CHUNK_BK));
                    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(DLM_CHUNK_SIZE));
                    c.g[V0][0] = sx(victim.wrapping_add(8));
                    c.g[V1][0] |= 1;
                    c.sw(r32(c, A0).wrapping_add(DLM_CHUNK_SIZE), c.g[V1][0]);
                    c.sw(r32(c, A2).wrapping_add(4), c.g[A3][0]);
                    c.sw(r32(c, A3).wrapping_add(DLM_CHUNK_FD), c.g[T0][0]);
                    dlheap_malloc_leave(c, frame);
                    return Ok(());
                }
            } else {
                // Large: its bin index (s6); consolidate pending fast chunks.
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                DLM_LARGE_BIN_INDEX.call(c, 0x0031_e934)?;
                c.g[V1][0] = c.lw(r32(c, S2)) & 0x2;
                c.g[S6][0] = c.g[V0][0];
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    DLM_CONSOLIDATE.call(c, 0x0031_e950)?;
                }
            }

            // The unsorted chunks (s1, from the back; s5 = the bin header).
            c.cycles += 4;
            c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(DLM_UNSORTED_BK));
            c.g[S5][0] = sx(r32(c, S2).wrapping_add(DLM_UNSORTED));
            if c.g[S1][0] != c.g[S5][0] {
                c.cycles += 2;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = SIZE_MASK;
                    c.g[A3][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_BK));
                    c.g[S0][0] = c.g[V1][0] & c.g[V0][0];
                    'unlink: {
                        if c.g[S7][0] != 0 {
                            c.cycles += 2;
                            if c.g[A3][0] != c.g[S5][0] {
                                c.sw(r32(c, S2).wrapping_add(DLM_UNSORTED_BK), c.g[A3][0]);
                                break 'unlink;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(DLM_LAST_REMAINDER));
                            if c.g[S1][0] != c.g[V0][0] {
                                c.sw(r32(c, S2).wrapping_add(DLM_UNSORTED_BK), c.g[A3][0]);
                                break 'unlink;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x10));
                            c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                            c.g[A2][0] = sx(r32(c, S1).wrapping_add(r32(c, S3)));
                            if c.g[V0][0] != 0 {
                                // Split the lone last remainder.
                                c.cycles += 11;
                                let rem = r32(c, A2);
                                let base = r32(c, S2);
                                c.g[T1][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3)));
                                c.sw(base.wrapping_add(DLM_LAST_REMAINDER), c.g[A2][0]);
                                c.g[A1][0] = c.g[S3][0] | 1;
                                c.sw(base.wrapping_add(DLM_UNSORTED_FD), c.g[A2][0]);
                                c.g[A0][0] = c.g[T1][0] | 1;
                                c.sw(base.wrapping_add(DLM_UNSORTED_BK), c.g[A2][0]);
                                c.g[V1][0] = sx(rem.wrapping_add(r32(c, T1)));
                                c.sw(rem.wrapping_add(DLM_CHUNK_BK), c.g[A3][0]);
                                c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
                                c.sw(rem.wrapping_add(DLM_CHUNK_FD), c.g[A3][0]);
                                c.sw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE), c.g[A1][0]);
                                dlheap_malloc_remainder_tail(c);
                                dlheap_malloc_leave(c, frame);
                                return Ok(());
                            }
                        }
                        c.cycles += 1;
                        c.sw(r32(c, S2).wrapping_add(DLM_UNSORTED_BK), c.g[A3][0]);
                    }
                    c.cycles += 2;
                    let exact = c.g[S0][0] == c.g[S3][0];
                    c.sw(r32(c, A3).wrapping_add(DLM_CHUNK_FD), c.g[S5][0]);
                    if exact {
                        c.cycles += 2;
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, S3)));
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
                        dlheap_malloc_set_next_inuse(c);
                        dlheap_malloc_leave(c, frame);
                        return Ok(());
                    }
                    // Sort the chunk into its bin: a1 = index, a3 = bck,
                    // a2 = fwd, t0 = the bin map.
                    c.cycles += 3;
                    c.g[S4][0] = (c.g[S0][0] < 0x100) as u64;
                    c.g[A1][0] = sx(r32(c, S0) >> 3);
                    'placed: {
                        if c.g[S4][0] != 0 {
                            c.cycles += 7;
                            c.g[T0][0] = sx(r32(c, S2).wrapping_add(DLM_BINMAP));
                            c.g[V0][0] = sx(r32(c, A1) << 3);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x38));
                            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                            c.g[A2][0] = c.lw(r32(c, V0));
                            c.g[A3][0] = sx(r32(c, V0).wrapping_sub(8));
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S0][0];
                            DLM_LARGE_BIN_INDEX.call(c, 0x0031_e9dc)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = sx(r32(c, A1) << 3);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x38));
                            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                            c.g[A2][0] = c.lw(r32(c, A0));
                            c.g[A3][0] = sx(r32(c, A0).wrapping_sub(8));
                            c.g[T0][0] = sx(r32(c, S2).wrapping_add(DLM_BINMAP));
                            if c.g[A2][0] != c.g[A3][0] {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(DLM_CHUNK_BK));
                                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(DLM_CHUNK_SIZE));
                                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                if c.g[V0][0] != 0 {
                                    // Smaller than the smallest: at the back.
                                    c.cycles += 3;
                                    c.g[A2][0] = c.g[A3][0];
                                    c.g[A3][0] = c.lw(r32(c, A0).wrapping_add(4));
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = sx(((r32(c, A1) as i32) >> 5) as u32);
                                    if c.g[S4][0] != 0 {
                                        break 'placed;
                                    }
                                    // Walk forward to the first chunk no
                                    // bigger than this one.
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_SIZE));
                                    c.g[S0][0] |= 1;
                                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                    if c.g[V0][0] == 0 {
                                        c.g[A3][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_BK));
                                        break 'placed;
                                    }
                                    c.cycles += 2;
                                    c.g[A2][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_FD));
                                    loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_SIZE));
                                        c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                        if c.g[V0][0] == 0 {
                                            break;
                                        }
                                        c.g[A2][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_FD));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0031_ea40);
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A3][0] = c.lw(r32(c, A2).wrapping_add(DLM_CHUNK_BK));
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = sx(((r32(c, A1) as i32) >> 5) as u32);
                    }
                    // Mark the bin in the map and link the chunk in.
                    c.cycles += 15;
                    c.g[V1][0] = 1;
                    c.g[A0][0] = sx(r32(c, A0) << 2);
                    c.g[A1][0] &= 0x1f;
                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = sx(r32(c, V1) << (r32(c, A1) & 31));
                    c.g[V0][0] = c.lw(r32(c, A0));
                    c.g[V0][0] |= c.g[V1][0];
                    c.sw(r32(c, A0), c.g[V0][0]);
                    let victim = r32(c, S1);
                    c.sw(victim.wrapping_add(DLM_CHUNK_BK), c.g[A3][0]);
                    c.sw(victim.wrapping_add(DLM_CHUNK_FD), c.g[A2][0]);
                    c.sw(r32(c, A2).wrapping_add(DLM_CHUNK_BK), c.g[S1][0]);
                    c.sw(r32(c, A3).wrapping_add(DLM_CHUNK_FD), c.g[S1][0]);
                    c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(DLM_UNSORTED_BK));
                    if c.g[S1][0] == c.g[S5][0] {
                        break;
                    }
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_e968);
                    }
                }
            }

            c.cycles += 2;
            if c.g[S7][0] == 0 {
                // Large request: the smallest fitting chunk of its own bin
                // (s1 from the back; t0 = the bin header).
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, S6) << 3);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x38));
                c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                c.g[S1][0] = c.lw(r32(c, V0).wrapping_add(4));
                c.g[T0][0] = sx(r32(c, V0).wrapping_sub(8));
                c.g[T2][0] = c.g[S5][0];
                if c.g[S1][0] != c.g[T0][0] {
                    c.cycles += 5;
                    c.g[A0][0] = SIZE_MASK;
                    c.g[T3][0] = c.g[S3][0] | 1;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
                    loop {
                        c.cycles += 4;
                        c.g[S0][0] = c.g[V0][0] & c.g[A0][0];
                        c.g[V1][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
                        if c.g[V1][0] == 0 {
                            // Fits: unlink it.
                            c.cycles += 8;
                            let victim = r32(c, S1);
                            c.g[A2][0] = c.lw(victim.wrapping_add(DLM_CHUNK_FD));
                            c.g[T1][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3)));
                            c.g[A3][0] = c.lw(victim.wrapping_add(DLM_CHUNK_BK));
                            c.g[A1][0] = sx(victim.wrapping_add(8));
                            c.g[V0][0] = (c.g[T1][0] < 0x10) as u64;
                            c.sw(r32(c, A2).wrapping_add(DLM_CHUNK_BK), c.g[A3][0]);
                            c.sw(r32(c, A3).wrapping_add(DLM_CHUNK_FD), c.g[A2][0]);
                            if c.g[V0][0] != 0 {
                                // Remainder too small: hand it all out.
                                c.cycles += 3;
                                c.g[A0][0] = sx(victim.wrapping_add(r32(c, S0)));
                                c.g[V0][0] = c.g[A1][0];
                                dlheap_malloc_set_next_inuse(c);
                                dlheap_malloc_leave(c, frame);
                                return Ok(());
                            }
                            // Split: the remainder goes to the unsorted bin.
                            c.cycles += 10;
                            c.g[A2][0] = sx(victim.wrapping_add(r32(c, S3)));
                            let rem = r32(c, A2);
                            let base = r32(c, S2);
                            c.g[A0][0] = c.g[T1][0] | 1;
                            c.sw(base.wrapping_add(DLM_UNSORTED_BK), c.g[A2][0]);
                            c.g[V1][0] = sx(rem.wrapping_add(r32(c, T1)));
                            c.sw(base.wrapping_add(DLM_UNSORTED_FD), c.g[A2][0]);
                            c.g[V0][0] = c.g[A1][0];
                            c.sw(rem.wrapping_add(DLM_CHUNK_BK), c.g[T2][0]);
                            c.sw(rem.wrapping_add(DLM_CHUNK_FD), c.g[T2][0]);
                            c.sw(victim.wrapping_add(DLM_CHUNK_SIZE), c.g[T3][0]);
                            dlheap_malloc_remainder_tail(c);
                            dlheap_malloc_leave(c, frame);
                            return Ok(());
                        }
                        c.g[S1][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_BK));
                        c.cycles += 2;
                        if c.g[S1][0] == c.g[T0][0] {
                            break;
                        }
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_ead8);
                        }
                    }
                }
                c.cycles += 1;
            }
            c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));

            // Scan the bin map from bin s6 up: a0 = the map word's address,
            // a3 = the map word, a2 = the bit, t0 = the bin, t1 = the block.
            c.cycles += 21;
            let idx = r32(c, S6);
            let base = r32(c, S2);
            c.g[A0][0] = sx(base.wrapping_add(DLM_BINMAP));
            c.g[T1][0] = sx(idx >> 5);
            c.g[T5][0] = c.g[A0][0];
            c.g[V0][0] = sx(r32(c, T1) << 2);
            c.g[V1][0] = sx(idx << 3);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[A1][0] = sx(r32(c, T1) << 8);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(base));
            c.g[A2][0] = c.g[S6][0] & 0x1f;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x38));
            c.g[V0][0] = 1;
            c.g[T2][0] = 0xffff_ffff_ffff_0000;
            c.g[A3][0] = c.lw(r32(c, A0));
            c.g[A2][0] = sx(r32(c, V0) << (r32(c, A2) & 31));
            c.g[T0][0] = sx(r32(c, V1).wrapping_add(0x30));
            c.g[T3][0] = c.g[S5][0];
            c.g[T6][0] = c.g[S7][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(base));
            c.g[T2][0] |= 0xfffc;
            c.g[T4][0] = c.g[S3][0] | 1;
            loop {
                c.cycles += 3;
                let past = c.g[A3][0] < c.g[A2][0];
                c.g[V0][0] = sx(r32(c, T1) << 2);
                let mut next_block = past;
                if !past {
                    c.cycles += 2;
                    let bit_left = c.g[A2][0] != 0;
                    c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                    if !bit_left {
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, T1) << 2);
                        next_block = true;
                    }
                }
                if next_block {
                    // The next non-empty map word, or the top chunk.
                    c.cycles += 2;
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, T5)));
                    c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                    loop {
                        c.cycles += 5;
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                        c.g[V0][0] = (c.g[T1][0] < 3) as u64;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x100));
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(DLM_TOP));
                            break 'use_top;
                        }
                        c.cycles += 3;
                        c.g[A3][0] = c.lw(r32(c, V1));
                        if c.g[A3][0] != 0 {
                            break;
                        }
                        c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_ebb0);
                        }
                    }
                    c.cycles += 3;
                    c.g[T0][0] = sx(r32(c, A1).wrapping_sub(8));
                    c.g[A2][0] = 1;
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                }
                // Advance to the first set bit.
                loop {
                    c.cycles += 5;
                    if c.g[V0][0] != 0 {
                        break;
                    }
                    c.g[A2][0] = sx(r32(c, A2) << 1);
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_ebe0);
                    }
                    c.cycles += 1;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                }
                c.cycles += 3;
                c.g[S1][0] = c.lw(r32(c, T0).wrapping_add(DLM_CHUNK_BK));
                if c.g[S1][0] != c.g[T0][0] {
                    // The bin's last chunk: unlink and split.
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
                    c.cycles += 7;
                    let victim = r32(c, S1);
                    let bin = r32(c, T0);
                    c.g[A3][0] = c.lw(victim.wrapping_add(DLM_CHUNK_BK));
                    c.g[S0][0] = c.g[V0][0] & c.g[T2][0];
                    c.g[T1][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3)));
                    c.sw(bin.wrapping_add(DLM_CHUNK_BK), c.g[A3][0]);
                    c.g[V0][0] = (c.g[T1][0] < 0x10) as u64;
                    c.sw(r32(c, A3).wrapping_add(DLM_CHUNK_FD), c.g[T0][0]);
                    if c.g[V0][0] != 0 {
                        // Remainder too small: hand it all out.
                        c.cycles += 1;
                        c.g[A0][0] = sx(victim.wrapping_add(r32(c, S0)));
                        c.cycles += 1;
                        c.g[V0][0] = sx(victim.wrapping_add(8));
                        dlheap_malloc_set_next_inuse(c);
                        dlheap_malloc_leave(c, frame);
                        return Ok(());
                    }
                    c.cycles += 6;
                    c.g[A2][0] = sx(victim.wrapping_add(r32(c, S3)));
                    let rem = r32(c, A2);
                    let base = r32(c, S2);
                    c.sw(base.wrapping_add(DLM_UNSORTED_FD), c.g[A2][0]);
                    c.sw(base.wrapping_add(DLM_UNSORTED_BK), c.g[A2][0]);
                    c.sw(rem.wrapping_add(DLM_CHUNK_BK), c.g[T3][0]);
                    c.sw(rem.wrapping_add(DLM_CHUNK_FD), c.g[T3][0]);
                    if c.g[T6][0] != 0 {
                        c.cycles += 1;
                        c.sw(base.wrapping_add(DLM_LAST_REMAINDER), c.g[A2][0]);
                    }
                    c.cycles += 6;
                    c.sw(victim.wrapping_add(DLM_CHUNK_SIZE), c.g[T4][0]);
                    c.g[V0][0] = c.g[T1][0] | 1;
                    c.sw(rem.wrapping_add(DLM_CHUNK_SIZE), c.g[V0][0]);
                    c.g[V1][0] = sx(rem.wrapping_add(r32(c, T1)));
                    c.sw(r32(c, V1), c.g[T1][0]);
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
                    dlheap_malloc_leave(c, frame);
                    return Ok(());
                }
                // Empty bin: clear its bit, go on with the next one.
                c.cycles += 6;
                c.g[V0][0] = !c.g[A2][0];
                c.g[T0][0] = sx(r32(c, S1).wrapping_add(8));
                c.g[A3][0] &= c.g[V0][0];
                c.g[A2][0] = sx(r32(c, A2) << 1);
                c.sw(r32(c, A0), c.g[A3][0]);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_eb90);
                }
            }
        }

        // The top chunk (s1): split it, or grow the heap.
        c.cycles += 8;
        c.g[V0][0] = SIZE_MASK;
        c.g[V1][0] = sx(r32(c, S3).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(DLM_CHUNK_SIZE));
        c.g[S0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[V1][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
        c.g[A0][0] = c.g[FP][0];
        if c.g[V1][0] != 0 {
            c.cycles += 5;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            DLM_SYS_ALLOC.call(c, 0x0031_eca0)?;
            frame.restore(c, S0, 0x90);
            dlheap_malloc_leave_restored(c, frame);
            return Ok(());
        }
        c.cycles += 7;
        let top = r32(c, S1);
        c.g[A2][0] = sx(top.wrapping_add(r32(c, S3)));
        c.g[V0][0] = c.g[S3][0] | 1;
        c.sw(r32(c, S2).wrapping_add(DLM_TOP), c.g[A2][0]);
        c.g[T1][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3)));
        c.sw(top.wrapping_add(DLM_CHUNK_SIZE), c.g[V0][0]);
        c.g[V1][0] = c.g[T1][0] | 1;
        c.sw(r32(c, A2).wrapping_add(DLM_CHUNK_SIZE), c.g[V1][0]);
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
        dlheap_malloc_leave(c, frame);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DLHeap_Malloc on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, then up to 24 random mallocs and frees of fast, small,
    /// medium and large sizes, so the fast bins, the unsorted bin, the small
    /// and large bins and the bin map hold chunks): no earlier calls (the
    /// zero state word: consolidate first), mallocs only (top chunk), sizes
    /// equal or close to freed ones (fast and small bin hits, exact unsorted
    /// fits, the last-remainder split, large-bin best fit and the bin-map
    /// scan with stale bits), sizes beyond the arena (`DLHeap_SysAlloc`) and
    /// sizes above the limit (`ENOMEM` through `__getreent`).
    #[test]
    fn dlheap_malloc_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        fn dlm_setup_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
            const BACK: u32 = 0x0000_7770;
            for (i, a) in args.iter().enumerate() {
                c.g[[A0, A1, A2][i]][0] = *a;
            }
            c.g[RA][0] = BACK as u64;
            c.pc = addr;
            f(c);
            assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
            c.g[V0][0]
        }
        fn dlm_pick_size(r: &mut crate::difftest::Rng) -> u32 {
            match r.next_u32() % 8 {
                0 | 1 => r.next_u32() % 0x48,
                2 | 3 => 0x48 + r.next_u32() % 0xb0,
                4 | 5 => 0xf8 + r.next_u32() % 0x700,
                6 => 0x800 + r.next_u32() % 0x2000,
                _ => 0x2800 + r.next_u32() % 0x6000,
            }
        }
        check(
            ee::s10::f_0031e818__generated,
            dlheap_malloc,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_e818, 0);
                let ra = c.g[RA][0];
                dlm_setup_call(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[HEAP as u64, ARENA as u64, 0x4_0000],
                );
                let ops = match seed % 10 {
                    0 => 0,
                    1 => 1 + r.next_u32() % 4,
                    _ => 2 + r.next_u32() % 23,
                };
                let only_mallocs = seed % 10 == 1;
                let mut live: Vec<(u64, u32)> = Vec::new();
                let mut freed: Vec<u32> = Vec::new();
                for _ in 0..ops {
                    if !only_mallocs && !live.is_empty() && r.next_u32() % 5 < 2 {
                        let i = (r.next_u32() as usize) % live.len();
                        let (p, size) = live.swap_remove(i);
                        freed.push(size);
                        dlm_setup_call(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[HEAP as u64, p]);
                    } else {
                        let size = dlm_pick_size(&mut r);
                        let p = dlm_setup_call(
                            c,
                            0x0031_e818,
                            ee::s10::DLHeap_Malloc,
                            &[HEAP as u64, size as u64],
                        );
                        if p != 0 {
                            live.push((p, size));
                        }
                    }
                }
                c.g[RA][0] = ra;
                c.pc = 0x0031_e818;
                c.g[A0][0] = sx(HEAP);
                c.g[A1][0] = match seed % 13 {
                    0 => sx(0xffff_ffe0 + r.next_u32() % 0x20),
                    1 => 0x4_0000 + (r.next_u32() % 0x1_0000) as u64,
                    2 | 3 | 4 | 5 if !freed.is_empty() => {
                        let s = freed[(r.next_u32() as usize) % freed.len()];
                        (s.saturating_sub(r.next_u32() % 0x20)) as u64
                    }
                    6 | 7 if !freed.is_empty() => {
                        let s = freed[(r.next_u32() as usize) % freed.len()];
                        (s / 2 + r.next_u32() % (s / 2 + 1)) as u64
                    }
                    _ => dlm_pick_size(&mut r) as u64,
                };
            },
        );
    }
}
