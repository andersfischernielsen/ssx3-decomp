//! Staging module oc347 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc347`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_8ef8, libc_qsort, "qsort")];

// ---------------------------------------------------------------------------
// qsort (0x418ef8)
// ---------------------------------------------------------------------------

// Frame slots of `qsort` (offsets from `sp`, frame 0xf0).
/// `n`: the element count of the current sub-array.
const QSORT_SLOT_N: u32 = 0x00;
/// `swaptype`: 0 = one dword per element, 1 = dwords, 2 = bytes.
const QSORT_SLOT_SWAPTYPE: u32 = 0x04;
/// `swap_cnt`: the partition swapped or met an equal element.
const QSORT_SLOT_SWAPPED: u32 = 0x08;
/// End of the array for the small-array insertion sort.
const QSORT_SLOT_SMALL_END: u32 = 0x0c;
/// `swaptype < 2` (dword swaps).
const QSORT_SLOT_DWORD_SWAP: u32 = 0x10;
/// `(es >> 3) << 32`: the dword count of an element, in the upper half.
const QSORT_SLOT_DWORDS: u32 = 0x18;
/// `es << 32`: the byte count of an element, in the upper half.
const QSORT_SLOT_BYTES: u32 = 0x20;
/// End of the array for the no-swap insertion sort.
const QSORT_SLOT_PART_END: u32 = 0x28;
/// `pa = a + es`.
const QSORT_SLOT_PA: u32 = 0x2c;
/// `n - 1`.
const QSORT_SLOT_N_MINUS_1: u32 = 0x30;
/// `t1` kept across the right scan's compare.
const QSORT_SLOT_T1_SAVE: u32 = 0x40;

pub(crate) const QSORT_ENTRY: u32 = 0x0041_8ef8;
const QSORT_FRAME: u32 = 0xf0;

#[inline(always)]
fn qsort_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

#[inline(always)]
fn qsort_subu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_sub(b as u32))
}

#[inline(always)]
fn qsort_lw_sp(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    c.lw(a)
}

#[inline(always)]
fn qsort_sw_reg(c: &mut Cpu, off: u32, reg: usize) {
    let v = c.g[reg][0];
    qsort_sw_sp(c, off, v);
}

#[inline(always)]
fn qsort_sw_sp(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    c.sw(a, v);
}

/// `cmp(a0, a1)` through the comparator in s7.
#[inline(always)]
fn qsort_cmp(c: &mut Cpu, ret: u32) -> Flow {
    let f = c.g[S7][0] as u32;
    call_ptr(c, f, ret)
}

/// One of the original's element swap loops (`swapfunc`, inlined 20 times):
/// exchange `n` dwords (`wide`) or bytes between `*p` and `*q`, advancing
/// both. Each pass leaves the old `*q` in v1 and the old `*p` in v0; the back
/// edge checks events at `head`.
fn qsort_swap_loop(c: &mut Cpu, head: u32, wide: bool, p: usize, q: usize, n: usize) {
    loop {
        c.cycles += 8;
        let pa = c.g[p][0] as u32;
        let qa = c.g[q][0] as u32;
        let step = if wide {
            c.g[V1][0] = c.ld(qa);
            c.g[n][0] = c.g[n][0].wrapping_sub(1);
            c.g[V0][0] = c.ld(pa);
            c.sd(pa, c.g[V1][0]);
            c.sd(qa, c.g[V0][0]);
            8
        } else {
            c.g[V1][0] = c.lbu(qa);
            c.g[n][0] = c.g[n][0].wrapping_sub(1);
            c.g[V0][0] = c.lb(pa);
            c.sb(pa, c.g[V1][0]);
            c.sb(qa, c.g[V0][0]);
            1
        };
        c.g[p][0] = sx(pa.wrapping_add(step));
        c.g[q][0] = sx(qa.wrapping_add(step));
        if (c.g[n][0] as i64) <= 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `med3(a, b, cc)` after its first compare's set-up (`a0 = a`, `a1 = b`
/// and the block's cycles): the median of the three elements in a0. `ret`
/// is the first compare's return address; the other four follow at
/// +0x10, +0x20, +0x3c and +0x50.
fn qsort_med3(c: &mut Cpu, a: usize, b: usize, cc: usize, ret: u32) -> Flow {
    qsort_cmp(c, ret)?;
    let a_ge_b = (c.g[V0][0] as i64) >= 0;
    c.g[A0][0] = c.g[b][0];
    if !a_ge_b {
        // a < b: cmp(b, cc) < 0 ? b : (cmp(a, cc) < 0 ? cc : a)
        c.cycles += 4;
        c.g[A1][0] = c.g[cc][0];
        qsort_cmp(c, ret + 0x10)?;
        let b_lt_c = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[a][0];
        if b_lt_c {
            c.cycles += 1;
            c.g[A0][0] = c.g[b][0];
        } else {
            c.cycles += 7;
            c.g[A1][0] = c.g[cc][0];
            qsort_cmp(c, ret + 0x20)?;
            let a_ge_c = (-1i64) < (c.g[V0][0] as i64);
            c.g[V1][0] = a_ge_c as u64;
            c.g[A0][0] = if a_ge_c { c.g[a][0] } else { c.g[cc][0] };
        }
    } else {
        // a >= b: cmp(b, cc) > 0 ? b : (cmp(a, cc) < 0 ? a : cc)
        c.cycles += 4;
        c.g[A1][0] = c.g[cc][0];
        qsort_cmp(c, ret + 0x3c)?;
        if (c.g[V0][0] as i64) > 0 {
            c.g[A0][0] = c.g[b][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[a][0];
            c.g[A1][0] = c.g[cc][0];
            qsort_cmp(c, ret + 0x50)?;
            let a_ge_c = (-1i64) < (c.g[V0][0] as i64);
            c.g[V1][0] = a_ge_c as u64;
            c.g[A0][0] = if a_ge_c { c.g[cc][0] } else { c.g[a][0] };
        }
    }
    Ok(())
}

/// Where one of the two insertion sorts lives in the original.
struct QsortInsertion {
    /// Cycles of the set-up block.
    setup: u64,
    /// The frame slot holding the end of the array.
    end_slot: u32,
    /// Outer loop head (`pm += es`), the swap head after a compare, the
    /// compare's return address and the two swap loop heads.
    outer: u32,
    swap: u32,
    cmp_ret: u32,
    dword_loop: u32,
    byte_loop: u32,
}

const QSORT_SMALL: QsortInsertion = QsortInsertion {
    setup: 6,
    end_slot: QSORT_SLOT_SMALL_END,
    outer: 0x0041_8f98,
    swap: 0x0041_8fa0,
    cmp_ret: 0x0041_903c,
    dword_loop: 0x0041_8fd0,
    byte_loop: 0x0041_9000,
};

const QSORT_NO_SWAP: QsortInsertion = QsortInsertion {
    setup: 7,
    end_slot: QSORT_SLOT_PART_END,
    outer: 0x0041_9650,
    swap: 0x0041_9658,
    cmp_ret: 0x0041_96f4,
    dword_loop: 0x0041_9688,
    byte_loop: 0x0041_96b8,
};

/// The insertion sort `for (pm = a + es; pm < end; pm += es) for (pl = pm;
/// pl > a && cmp(pl - es, pl) > 0; pl -= es) swap(pl, pl - es);`, entered
/// with s0 = a + es, v1 = end and v0 = es >> 3. Ends with s0 reloaded.
fn qsort_insertion(c: &mut Cpu, at: &QsortInsertion) -> Flow {
    c.cycles += at.setup;
    qsort_sw_reg(c, at.end_slot, V1);
    c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
    c.g[FP][0] = c.g[V0][0] << 32;
    c.g[S4][0] = c.g[S2][0] << 32;
    c.g[S5][0] = ((c.g[V1][0] as i64) < 2) as u64;
    c.g[S1][0] = c.g[S0][0];
    loop {
        // pm: s3 = the next pm, s1 = pl.
        c.cycles += 2;
        c.g[S3][0] = qsort_addu(c.g[S0][0], c.g[S2][0]);
        loop {
            c.cycles += 3;
            let above_base = c.g[S6][0] < c.g[S1][0];
            c.g[V0][0] = above_base as u64;
            c.g[S0][0] = qsort_subu(c.g[S1][0], c.g[S2][0]);
            if !above_base {
                break;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            qsort_cmp(c, at.cmp_ret)?;
            if (c.g[V0][0] as i64) <= 0 {
                break;
            }
            c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
            if event_due(c) {
                deliver_event_here(c, at.swap);
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.cycles += 5;
                let (lo, hi) = (c.g[S0][0] as u32, c.g[S1][0] as u32);
                c.g[V0][0] = c.ld(lo);
                c.g[V1][0] = c.ld(hi);
                c.sd(hi, c.g[V0][0]);
                c.sd(lo, c.g[V1][0]);
                c.cycles += 1;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                if c.g[S5][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[FP][0] >> 32;
                    c.g[A2][0] = c.g[S0][0];
                    qsort_swap_loop(c, at.dword_loop, true, A0, A2, A1);
                    c.cycles += 2;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S4][0] >> 32;
                    c.g[A2][0] = c.g[S0][0];
                    qsort_swap_loop(c, at.byte_loop, false, A0, A2, A1);
                    c.cycles += 1;
                }
            }
            c.g[S1][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.g[V1][0] = qsort_lw_sp(c, at.end_slot);
        c.g[S0][0] = c.g[S3][0];
        let more = c.g[S0][0] < c.g[V1][0];
        c.g[V0][0] = more as u64;
        c.g[S1][0] = c.g[S0][0];
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, at.outer);
        }
    }
    c.cycles += 2;
    let a = sp(c).wrapping_add(0xe0);
    c.lq(S0, a);
    Ok(())
}

/// `void qsort(void* a, size_t n, size_t es, int (*cmp)(const void*, const
/// void*))` (0x418ef8): the BSD 4.4 libc quicksort. Arrays under 7 elements
/// are insertion sorted; otherwise the pivot is the median of three (of
/// three medians of three above 40 elements), the partition gathers the
/// elements equal to the pivot at both ends and moves them to the middle,
/// the left part is sorted recursively and the right part iteratively. A
/// partition that swapped nothing falls back to insertion sort. Elements are
/// swapped one dword at a time when the array and `es` are 8-aligned
/// (`es == 8` swaps inline), else byte by byte.
pub fn libc_qsort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, QSORT_FRAME);
        frame.save(c, S2, 0xc0);
        frame.save(c, S6, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S7, 0x70);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S1, 0xd0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        qsort_sw_reg(c, QSORT_SLOT_N, A1);
        // s6 = a, s2 = es, s7 = cmp.
        loop {
            // SWAPINIT
            c.cycles += 3;
            c.g[V0][0] = c.g[S6][0] & 7;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[S2][0] & 7;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[S2][0] ^ 8) != 0) as u64;
                }
            }
            c.cycles += 5;
            qsort_sw_reg(c, QSORT_SLOT_SWAPTYPE, V0);
            c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_N);
            let small = c.g[V1][0] < 7;
            c.g[V0][0] = small as u64;
            qsort_sw_sp(c, QSORT_SLOT_SWAPPED, 0);
            if small {
                c.cycles += 6;
                c.mult(0, V0, V1, S2);
                c.g[S0][0] = qsort_addu(c.g[S6][0], c.g[S2][0]);
                c.g[V1][0] = qsort_addu(c.g[V0][0], c.g[S6][0]);
                let any = c.g[S0][0] < c.g[V1][0];
                c.g[V0][0] = sx((c.g[S2][0] as u32) >> 3);
                if any {
                    qsort_insertion(c, &QSORT_SMALL)?;
                } else {
                    c.cycles += 1;
                    let a = frame.at(c, 0xe0);
                    c.lq(S0, a);
                }
                break;
            }

            // pm = a + (n / 2) * es (s0), pn = a + (n - 1) * es (s5)
            c.cycles += 8;
            c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_N);
            c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
            let under_8 = c.g[A0][0] < 8;
            c.g[V1][0] = under_8 as u64;
            c.g[A0][0] = qsort_subu(c.g[A0][0], 1);
            qsort_sw_reg(c, QSORT_SLOT_N_MINUS_1, A0);
            c.mult(0, A0, V0, S2);
            c.g[S0][0] = qsort_addu(c.g[A0][0], c.g[S6][0]);
            if !under_8 {
                // pl = a (s1)
                c.cycles += 7;
                c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_N);
                c.g[S1][0] = c.g[S6][0];
                let under_41 = c.g[V1][0] < 0x29;
                c.g[V0][0] = under_41 as u64;
                c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_N_MINUS_1);
                c.mult(0, V1, V1, S2);
                c.g[S5][0] = qsort_addu(c.g[V1][0], c.g[S6][0]);
                if !under_41 {
                    // d = (n / 8) * es (fp): pseudo-median of nine.
                    c.cycles += 11;
                    c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_N);
                    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 3);
                    c.mult(0, FP, V0, S2);
                    c.g[A0][0] = c.g[S6][0];
                    c.g[S1][0] = qsort_addu(c.g[S6][0], c.g[FP][0]);
                    c.g[V0][0] = sx((c.g[FP][0] as u32) << 1);
                    c.g[S3][0] = qsort_addu(c.g[V0][0], c.g[S6][0]);
                    c.g[A1][0] = c.g[S1][0];
                    // pl = med3(pl, pl + d, pl + 2d)
                    qsort_med3(c, S6, S1, S3, 0x0041_90c0)?;
                    c.cycles += 8;
                    c.g[S4][0] = qsort_subu(c.g[S0][0], c.g[FP][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[S3][0] = qsort_addu(c.g[S0][0], c.g[FP][0]);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S0][0];
                    // pm = med3(pm - d, pm, pm + d)
                    qsort_med3(c, S4, S0, S3, 0x0041_9140)?;
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[FP][0] as u32) << 1);
                    c.g[S3][0] = qsort_subu(c.g[S5][0], c.g[FP][0]);
                    c.g[S4][0] = qsort_subu(c.g[S5][0], c.g[V0][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S3][0];
                    // pn = med3(pn - 2d, pn - d, pn)
                    qsort_med3(c, S4, S3, S5, 0x0041_91c4)?;
                    c.cycles += 1;
                    c.g[S5][0] = c.g[A0][0];
                }
                // pm = med3(pl, pm, pn)
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                qsort_med3(c, S1, S0, S5, 0x0041_923c)?;
                c.cycles += 1;
                c.g[S0][0] = c.g[A0][0];
            }

            // swap(a, pm); pa = pb = a + es
            c.cycles += 3;
            c.g[V0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
            c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                let (pm, a) = (c.g[S0][0] as u32, c.g[S6][0] as u32);
                c.g[V1][0] = c.ld(pm);
                c.g[A0][0] = qsort_addu(c.g[S6][0], c.g[S2][0]);
                qsort_sw_reg(c, QSORT_SLOT_PA, A0);
                c.g[V0][0] = c.ld(a);
                c.sd(a, c.g[V1][0]);
                c.sd(pm, c.g[V0][0]);
                c.cycles += 1;
            } else {
                c.cycles += 3;
                let dwords = (c.g[V1][0] as i64) < 2;
                c.g[V0][0] = dwords as u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32) >> 3);
                if dwords {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] <<= 32;
                    c.g[A2][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    c.g[V0][0] = qsort_addu(c.g[S6][0], c.g[S2][0]);
                    qsort_sw_reg(c, QSORT_SLOT_PA, V0);
                    qsort_swap_loop(c, 0x0041_92f8, true, A2, A0, A1);
                    c.cycles += 2;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = qsort_addu(c.g[S6][0], c.g[S2][0]);
                    c.g[V0][0] = c.g[S2][0] << 32;
                    qsort_sw_reg(c, QSORT_SLOT_PA, V1);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    c.g[A2][0] = c.g[S6][0];
                    qsort_swap_loop(c, 0x0041_9338, false, A2, A0, A1);
                    c.cycles += 1;
                }
            }
            c.g[V0][0] = qsort_lw_sp(c, QSORT_SLOT_N_MINUS_1);

            // pc = pd = a + (n - 1) * es. s3 = pa, s1 = pb, s0 = pc, s4 = pd.
            c.cycles += 14;
            c.g[A0][0] = c.g[S2][0] << 32;
            c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
            c.mult(0, V0, V0, S2);
            c.g[S1][0] = qsort_lw_sp(c, QSORT_SLOT_PA);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let a = frame.at(c, QSORT_SLOT_BYTES);
            c.sd(a, c.g[A0][0]);
            qsort_sw_reg(c, QSORT_SLOT_DWORD_SWAP, V1);
            c.g[S3][0] = c.g[S1][0];
            c.g[S4][0] = qsort_addu(c.g[V0][0], c.g[S6][0]);
            c.g[V0][0] = sx((c.g[S2][0] as u32) >> 3);
            c.g[S0][0] = c.g[S4][0];
            c.g[V0][0] <<= 32;
            let a = frame.at(c, QSORT_SLOT_DWORDS);
            c.sd(a, c.g[V0][0]);

            'partition: loop {
                // while (pb <= pc && (r = cmp(pb, a)) <= 0)
                loop {
                    c.cycles += 3;
                    let crossed = c.g[S0][0] < c.g[S1][0];
                    c.g[V0][0] = crossed as u64;
                    c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                    if crossed {
                        break;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A0][0] = c.g[S1][0];
                    qsort_cmp(c, 0x0041_9468)?;
                    c.g[A1][0] = c.g[V0][0];
                    let le = (c.g[A1][0] as i64) <= 0;
                    c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                    if !le {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_9398);
                    }
                    c.cycles += 2;
                    c.g[T1][0] = qsort_addu(c.g[S1][0], c.g[S2][0]);
                    if c.g[A1][0] == 0 {
                        // swap_cnt = 1; swap(pa, pb); pa += es
                        c.cycles += 4;
                        c.g[V0][0] = 1;
                        c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                        qsort_sw_reg(c, QSORT_SLOT_SWAPPED, V0);
                        if c.g[V1][0] == 0 {
                            c.cycles += 6;
                            let (pb, pa) = (c.g[S1][0] as u32, c.g[S3][0] as u32);
                            c.g[V1][0] = c.ld(pb);
                            c.g[A3][0] = qsort_addu(c.g[S3][0], c.g[S2][0]);
                            c.g[V0][0] = c.ld(pa);
                            c.sd(pa, c.g[V1][0]);
                            c.sd(pb, c.g[V0][0]);
                            c.cycles += 1;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_DWORD_SWAP);
                            let a = frame.at(c, QSORT_SLOT_DWORDS);
                            c.g[V0][0] = c.ld(a);
                            if c.g[A0][0] != 0 {
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A3][0] = qsort_addu(c.g[A1][0], c.g[S2][0]);
                                c.g[A2][0] = c.g[V0][0] >> 32;
                                qsort_swap_loop(c, 0x0041_93e8, true, A1, A0, A2);
                                c.cycles += 2;
                            } else {
                                c.cycles += 6;
                                let a = frame.at(c, QSORT_SLOT_BYTES);
                                c.g[V1][0] = c.ld(a);
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A3][0] = qsort_addu(c.g[A1][0], c.g[S2][0]);
                                c.g[A2][0] = c.g[V1][0] >> 32;
                                qsort_swap_loop(c, 0x0041_9428, false, A1, A0, A2);
                                c.cycles += 1;
                            }
                        }
                        c.g[S3][0] = c.g[A3][0];
                    }
                    c.cycles += 1;
                    c.g[S1][0] = c.g[T1][0];
                }

                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S2][0] as u32) >> 3);
                c.g[FP][0] = c.g[V0][0] << 32;
                c.g[S5][0] = c.g[S2][0] << 32;
                c.g[T1][0] = ((c.g[A0][0] as i64) < 2) as u64;

                // while (pb <= pc && (r = cmp(pc, a)) >= 0)
                loop {
                    c.cycles += 3;
                    let crossed = c.g[S0][0] < c.g[S1][0];
                    c.g[V0][0] = crossed as u64;
                    c.g[A1][0] = c.g[S6][0];
                    if crossed {
                        break 'partition;
                    }
                    c.cycles += 6;
                    let a = frame.at(c, QSORT_SLOT_T1_SAVE);
                    c.sq(T1, a);
                    c.g[A0][0] = c.g[S0][0];
                    qsort_cmp(c, 0x0041_9548)?;
                    c.g[A1][0] = c.g[V0][0];
                    let ge = (c.g[A1][0] as i64) >= 0;
                    let a = frame.at(c, QSORT_SLOT_T1_SAVE);
                    c.lq(T1, a);
                    if !ge {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_9488);
                    }
                    c.cycles += 2;
                    c.g[T0][0] = qsort_subu(c.g[S0][0], c.g[S2][0]);
                    if c.g[A1][0] == 0 {
                        // swap_cnt = 1; swap(pc, pd); pd -= es
                        c.cycles += 4;
                        c.g[V0][0] = 1;
                        c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                        qsort_sw_reg(c, QSORT_SLOT_SWAPPED, V0);
                        if c.g[V1][0] == 0 {
                            c.cycles += 6;
                            let (pd, pc) = (c.g[S4][0] as u32, c.g[S0][0] as u32);
                            c.g[V1][0] = c.ld(pd);
                            c.g[A3][0] = qsort_subu(c.g[S4][0], c.g[S2][0]);
                            c.g[V0][0] = c.ld(pc);
                            c.sd(pc, c.g[V1][0]);
                            c.sd(pd, c.g[V0][0]);
                            c.cycles += 1;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            if c.g[T1][0] != 0 {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[A2][0] = c.g[FP][0] >> 32;
                                c.g[A3][0] = qsort_subu(c.g[A1][0], c.g[S2][0]);
                                qsort_swap_loop(c, 0x0041_94d0, true, A0, A1, A2);
                                c.cycles += 2;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[A2][0] = c.g[S5][0] >> 32;
                                c.g[A3][0] = qsort_subu(c.g[A1][0], c.g[S2][0]);
                                qsort_swap_loop(c, 0x0041_9508, false, A0, A1, A2);
                                c.cycles += 1;
                            }
                        }
                        c.g[S4][0] = c.g[A3][0];
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[T0][0];
                }

                // swap(pb, pc); swap_cnt = 1; pb += es; pc -= es
                c.cycles += 3;
                c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                c.g[V0][0] = qsort_lw_sp(c, QSORT_SLOT_DWORD_SWAP);
                if c.g[A0][0] == 0 {
                    c.cycles += 7;
                    let (pc, pb) = (c.g[S0][0] as u32, c.g[S1][0] as u32);
                    c.g[V1][0] = c.ld(pc);
                    c.g[T1][0] = qsort_addu(c.g[S1][0], c.g[S2][0]);
                    c.g[V0][0] = c.ld(pb);
                    c.g[T0][0] = qsort_subu(c.g[S0][0], c.g[S2][0]);
                    c.sd(pb, c.g[V1][0]);
                    c.sd(pc, c.g[V0][0]);
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    let a = frame.at(c, QSORT_SLOT_DWORDS);
                    c.g[V1][0] = c.ld(a);
                    if c.g[V0][0] != 0 {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T1][0] = qsort_addu(c.g[A1][0], c.g[S2][0]);
                        c.g[A2][0] = c.g[V1][0] >> 32;
                        c.g[T0][0] = qsort_subu(c.g[A0][0], c.g[S2][0]);
                        qsort_swap_loop(c, 0x0041_9598, true, A1, A0, A2);
                        c.cycles += 2;
                    } else {
                        c.cycles += 6;
                        let a = frame.at(c, QSORT_SLOT_BYTES);
                        c.g[V0][0] = c.ld(a);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T1][0] = qsort_addu(c.g[A1][0], c.g[S2][0]);
                        c.g[A2][0] = c.g[V0][0] >> 32;
                        c.g[T0][0] = qsort_subu(c.g[A0][0], c.g[S2][0]);
                        qsort_swap_loop(c, 0x0041_95d8, false, A1, A0, A2);
                        c.cycles += 1;
                    }
                }
                c.g[V1][0] = 1;
                c.cycles += 4;
                c.g[S1][0] = c.g[T1][0];
                c.g[S0][0] = c.g[T0][0];
                qsort_sw_reg(c, QSORT_SLOT_SWAPPED, V1);
                if event_due(c) {
                    deliver_event_here(c, 0x0041_9450);
                }
            }

            c.cycles += 3;
            c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPPED);
            c.g[A1][0] = qsort_subu(c.g[S1][0], c.g[S3][0]);
            if c.g[A0][0] == 0 {
                // Nothing swapped: insertion sort the whole sub-array.
                c.cycles += 7;
                c.g[V0][0] = qsort_lw_sp(c, QSORT_SLOT_N);
                c.g[S0][0] = qsort_lw_sp(c, QSORT_SLOT_PA);
                c.mult(0, V0, V0, S2);
                c.g[V1][0] = qsort_addu(c.g[V0][0], c.g[S6][0]);
                let any = c.g[S0][0] < c.g[V1][0];
                c.g[V0][0] = sx((c.g[S2][0] as u32) >> 3);
                if any {
                    qsort_insertion(c, &QSORT_NO_SWAP)?;
                } else {
                    c.cycles += 1;
                    let a = frame.at(c, 0xe0);
                    c.lq(S0, a);
                }
                break;
            }

            // r = min(pa - a, pb - pa); vecswap(a, pb - r, r)
            c.cycles += 9;
            c.g[V1][0] = qsort_subu(c.g[S3][0], c.g[S6][0]);
            let lower = (c.g[V1][0] as i64) < (c.g[A1][0] as i64);
            c.g[V0][0] = lower as u64;
            c.g[A3][0] = c.g[A1][0];
            if lower {
                c.g[A1][0] = c.g[V1][0];
            }
            c.g[S0][0] = qsort_subu(c.g[S4][0], c.g[S0][0]);
            c.g[V0][0] = qsort_lw_sp(c, QSORT_SLOT_N);
            c.mult(0, V0, V0, S2);
            c.g[S5][0] = qsort_addu(c.g[V0][0], c.g[S6][0]);
            if (c.g[A1][0] as i64) <= 0 {
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
                let dwords = (c.g[A0][0] as i64) < 2;
                c.g[V0][0] = dwords as u64;
                c.g[V1][0] = qsort_subu(c.g[S1][0], c.g[A1][0]);
                if dwords {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 3);
                    c.g[A2][0] = c.g[V1][0];
                    c.g[V0][0] <<= 32;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    qsort_swap_loop(c, 0x0041_9760, true, A0, A2, A1);
                    c.cycles += 2;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[A1][0] << 32;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    c.g[A0][0] = c.g[S6][0];
                    qsort_swap_loop(c, 0x0041_9798, false, A0, A2, A1);
                    c.cycles += 1;
                }
            }
            c.g[V0][0] = qsort_subu(c.g[S5][0], c.g[S4][0]);

            // r = min(pd - pc, pn - pd - es); vecswap(pb, pn - r, r)
            c.cycles += 5;
            c.g[A1][0] = qsort_subu(c.g[V0][0], c.g[S2][0]);
            let lower = c.g[S0][0] < c.g[A1][0];
            c.g[V1][0] = lower as u64;
            if lower {
                c.g[A1][0] = c.g[S0][0];
            }
            let none = (c.g[A1][0] as i64) <= 0;
            c.g[V1][0] = qsort_lw_sp(c, QSORT_SLOT_SWAPTYPE);
            if none {
                c.cycles += 1;
            } else {
                c.cycles += 3;
                let dwords = (c.g[V1][0] as i64) < 2;
                c.g[V0][0] = dwords as u64;
                c.g[V1][0] = qsort_subu(c.g[S5][0], c.g[A1][0]);
                if dwords {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 3);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] <<= 32;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    qsort_swap_loop(c, 0x0041_97f0, true, A0, A2, A1);
                    c.cycles += 2;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[A1][0] << 32;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0] >> 32;
                    c.g[A2][0] = c.g[V1][0];
                    qsort_swap_loop(c, 0x0041_9828, false, A0, A2, A1);
                    c.cycles += 1;
                }
            }
            c.g[A1][0] = c.g[A3][0];

            // if ((r = pb - pa) > es) qsort(a, r / es, es, cmp)
            c.cycles += 3;
            let left = c.g[S2][0] < c.g[A1][0];
            c.g[V0][0] = left as u64;
            if !left {
                c.g[A1][0] = c.g[S0][0];
            } else {
                c.cycles += 3;
                c.divu(0, A1, S2);
                c.cycles += 6;
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.lo[0];
                c.g[A3][0] = c.g[S7][0];
                Callee {
                    addr: QSORT_ENTRY,
                    f: ee::s15::qsort,
                }
                .call(c, 0x0041_9878)?;
                c.g[A1][0] = c.g[S0][0];
            }

            // if ((r = pd - pc) > es) { a = pn - r; n = r / es; goto loop; }
            c.cycles += 3;
            let right = c.g[S2][0] < c.g[A1][0];
            c.g[V0][0] = right as u64;
            let a = frame.at(c, 0xe0);
            c.lq(S0, a);
            if !right {
                break;
            }
            c.cycles += 3;
            c.divu(0, A1, S2);
            c.cycles += 4;
            c.g[S6][0] = qsort_subu(c.g[S5][0], c.g[A1][0]);
            c.g[V0][0] = c.lo[0];
            qsort_sw_reg(c, QSORT_SLOT_N, V0);
            if event_due(c) {
                deliver_event_here(c, 0x0041_8f34);
            }
        }

        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `GameModeMan_CompareByTime` (0x245e78): compares `TABLE[*a]` with
    /// `TABLE[*b]` (unsigned), returning -1, 0 or 1. The comparator the
    /// fixture sorts with.
    const QSORT_TEST_CMP: u32 = 0x0024_5e78;
    const QSORT_TEST_TABLE: u32 = 0x0053_6640;

    /// Every element size and alignment (dword `es == 8`, dwords, bytes for
    /// an odd size or a misaligned array), counts across the insertion,
    /// median-of-3 and median-of-9 thresholds, and orders that hit the
    /// equal-key, the no-swap fallback, the recursion and the loop back.
    #[test]
    fn libc_qsort_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        // (es, offset of the array from DATA)
        const SIZES: [(u32, u32); 6] = [(8, 0), (16, 0), (24, 0), (4, 0), (8, 4), (12, 0)];
        const COUNTS: [u32; 14] = [0, 1, 2, 3, 5, 6, 7, 8, 12, 30, 40, 41, 60, 100];
        check_lohi(
            ee::s15::f_00418ef8__generated,
            libc_qsort,
            280,
            &|c, seed| {
                let mut r = Rng(0x5157_0a7e_0000_1234 ^ seed as u64);
                scramble(c, &mut r);
                let (es, off) = SIZES[(seed % 6) as usize];
                let n = COUNTS[((seed / 6) % 14) as usize];
                let base = DATA + off;
                // Strictly increasing keys: equal keys come from equal indices.
                for i in 0..128 {
                    c.bus
                        .write32(QSORT_TEST_TABLE + 4 * i, i * 3 + r.next_u32() % 3);
                }
                for i in 0..n {
                    let key = match (seed / 7) % 5 {
                        0 => r.next_u32() % 8,
                        1 => r.next_u32() % 128,
                        2 => i,
                        3 => n - 1 - i,
                        _ => 5,
                    };
                    let e = base + i * es;
                    c.bus.write32(e, key);
                    for w in 1..es / 4 {
                        c.bus.write32(e + 4 * w, r.next_u32());
                    }
                }
                c.g[A0][0] = sx(base);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = es as u64;
                c.g[A3][0] = sx(QSORT_TEST_CMP);
                c.pc = QSORT_ENTRY;
            },
        );
    }
}
