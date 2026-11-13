//! `src/bx/cubicspline.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::intersect::aifwddiff::oc230_add;
pub(crate) use crate::game::intersect::aifwddiff::oc230_sll;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_e1d8, dlheap_large_bin_index, "DLHeap_LargeBinIndex"),
    (0x0031_fbb8, dlheap_usable_size, "DLHeap_UsableSize"),
    (0x0031_ff60, dl_heap_init_arena, "DLHeap_InitArena"),
    (0x0032_0488, mem_pool8_free, "MEM_Pool8Free"),
    (0x0032_0498, mem_pool16_free, "MEM_Pool16Free"),
    (0x0032_04a8, mem_pool32_free, "MEM_Pool32Free"),
    (0x0032_04b8, mem_pool64_free, "MEM_Pool64Free"),
    (0x0032_04d0, mem_pool128_free, "MEM_Pool128Free"),
    (0x0032_04e8, mem_pool256_free, "MEM_Pool256Free"),
    (0x0032_0500, mem_pool512_free, "MEM_Pool512Free"),
    (
        0x0031_dee0,
        c_csi_cubic_spline_evaluate,
        "cCSICubicSpline_Evaluate",
    ),
    (0x0031_9e48, mem_pool_alloc, "MEM_PoolAlloc"),
    (0x0031_f2c8, dlheap_memalign, "DLHeap_Memalign"),
    (0x0031_ed60, dlheap_free, "DLHeap_Free"),
    (
        0x0031_d7e0,
        c_csi_cubic_spline_add_point,
        "cCSICubicSpline_AddPoint",
    ),
    (0x0032_0368, mem_pool64_alloc, "MEM_Pool64Alloc"),
    (0x0032_0408, mem_pool256_alloc, "MEM_Pool256Alloc"),
    (
        0x0031_d8b0,
        c_csi_cubic_spline_solve,
        "cCSICubicSpline_Solve",
    ),
    (0x0031_eee8, dlheap_consolidate, "DLHeap_Consolidate"),
    (0x0032_02f8, mem_pool16_alloc, "MEM_Pool16Alloc"),
    (0x0032_03b8, mem_pool128_alloc, "MEM_Pool128Alloc"),
    (
        0x0031_d660,
        c_csi_cubic_spline_alloc,
        "cCSICubicSpline_Alloc",
    ),
    (0x0031_e2d8, dlheap_sys_alloc, "DLHeap_SysAlloc"),
    (0x0032_0330, mem_pool32_alloc, "MEM_Pool32Alloc"),
];

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
}

/// `int DLHeap_LargeBinIndex(uint size)` (0x31e1d8): the free-list bin for a
/// large allocation of `size` bytes (0x5f when the size index is too big).
pub fn dlheap_large_bin_index(c: &mut Cpu) {
    c.cycles += 6;
    let size = abi::arg(c, 0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = 0xffff;
    c.g[A0][0] = sx(size >> 8);
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    let too_big = c.g[V0][0] != 0;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x100));
    if too_big {
        c.cycles += 2;
        c.g[V0][0] = 0x5f;
        abi::ret(c);
        return;
    }
    c.cycles += 26;
    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 16);
    c.g[V1][0] = c.g[V1][0] & 0x8;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x4;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x2;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 15);
    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 14);
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd));
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
    c.g[V0][0] = c.g[V0][0] & 0x3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `uint DLHeap_UsableSize(void *block)` (0x31fbb8): usable payload size of a
/// heap block, from its header word at `block-4`.
pub fn dlheap_usable_size(c: &mut Cpu) {
    let a1 = abi::arg(c, 1);
    'b_31fc18: {
        c.cycles += 2;
        if a1 == 0 {
            break 'b_31fc18;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(a1.wrapping_sub(4)).lw(c);
        c.g[V0][0] = c.g[A0][0] & 0x2;
        let small = c.g[V0][0] == 0;
        c.g[A1][0] = sx(a1.wrapping_sub(8));
        if small {
            c.cycles += 8;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffe;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x1;
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V0][0] | 0xfffc;
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                abi::ret(c);
                return;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffc;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void MEM_Pool8Free(MEM_Pool8 *pool, void *ptr)` (0x320488): push `ptr` on
/// the pool's free list at +0x800.
pub fn mem_pool8_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x800).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x800).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void MEM_Pool16Free(MEM_Pool16 *pool, void *ptr)` (0x320498): push `ptr`
/// on the pool's free list at +0x2000.
pub fn mem_pool16_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x2000).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x2000).sw(c, c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-pool free lists: push a block onto the singly linked free list stored
// at a fixed offset inside the pool's base (the new head is written into the
// block's `next` word and into the pool head).

/// `void MEM_Pool32Free(void *pool, void *block)` (0x3204a8): push `block` on
/// the 32-byte pool's free list whose head lives at `pool + 0x6000`.
pub fn mem_pool32_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = Addr(abi::arg(c, 0));
    let block = c.g[A1][0];
    let head = pool.off(0x6000).lw(c);
    Addr(block as u32).sw(c, head);
    pool.off(0x6000).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool64Free(void *pool, void *block)` (0x3204b8): the 64-byte pool
/// is based at `pool + 0x30000`, head at `+0x2000`.
pub fn mem_pool64_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x30000));
    c.g[V1][0] = 0x30000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).off(0x2000).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).off(0x2000).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool128Free(void *pool, void *block)` (0x3204d0): 128-byte pool
/// based at `pool + 0x20000`, head at `+0x5800`.
pub fn mem_pool128_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).off(0x5800).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).off(0x5800).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool256Free(void *pool, void *block)` (0x3204e8): 256-byte pool
/// based at `pool + 0x20000`, head at its very start.
pub fn mem_pool256_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool512Free(void *pool, void *block)` (0x320500): same layout as
/// the 256-byte pool.
pub fn mem_pool512_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `DLHeap_InitArena` (0x31FF60): carve a 16-byte aligned block out of an arena
/// and initialise the heap header in it.
pub fn dl_heap_init_arena(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = low(c, A1).wrapping_add(0xf);
        c.g[V1][0] |= 0xfff0;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[V0][0] &= c.g[V1][0];
        c.sq(16, sp(c).wrapping_add(0x10));
        c.g[A1][0] = low(c, V0).wrapping_sub(low(c, A1));
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = low(c, A2).wrapping_sub(low(c, A1));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[V0][0];
        sw_at(c, S0, 0xc, c.g[A2][0]);
        c.g[A1][0] = 0;
        sw_at(c, S0, 0x4, c.g[V0][0]);
        c.g[A2][0] = 0x378;
        sw_at(c, S0, 0, c.g[V0][0]);
        call_ptr(c, addr::MEMSET, 0x0031_ffa8)?;
        c.g[V1][0] = lw_at(c, S0, 0x4);
        c.g[V0][0] = lw_at(c, S0, 0xc);
        c.g[V1][0] = low(c, V1).wrapping_add(0x380);
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.g[V0][0] = low(c, V0).wrapping_sub(0x380);
        sw_at(c, S0, 0x8, c.g[V1][0]);
        sw_at(c, S0, 0xc, c.g[V0][0]);
        sw_at(c, S0, 0x4, c.g[V1][0]);
        c.lq(16, sp(c).wrapping_add(0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

pub(crate) mod oc230_spline {

    pub const END_TIME: u32 = 0x4;

    pub const SEGMENTS: u32 = 0x8;

    pub const SEG_SIZE: u32 = 0x14;

    pub const SEG_START: u32 = 0x10;

    /// Word 0 with the current-segment field cleared.
    pub const CUR_CLEAR: u64 = 0xffff_ffff_f807_ffff;

    pub const EVAL_SEGMENT: crate::call::Callee = crate::call::Callee {
        addr: 0x0031_d5a8,
        f: ssx3_ee::s10::cCSICubicSpline_EvalSegment,
    };
}

/// `float cCSICubicSpline_Evaluate(cCSICubicSpline* s, float t)`
/// (0x31dee0): evaluates the segment holding t (`EvalSegment`, result in
/// f0). The current segment is tried first (start <= t <= next start, or
/// the end time for the last); else t before it: segment 0 when t is before
/// segment 1; t after it: the last segment when the current is the one
/// before it, the next one when t is before the one after; otherwise a
/// binary search over the start times. The found segment becomes current.
pub fn c_csi_cubic_spline_evaluate(c: &mut Cpu) {
    use oc230_spline as sp_;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let spline = r32(c, A2);
        c.g[V0][0] = c.lw(spline);
        c.g[V1][0] = sx(r32(c, V0) >> 11) & 0xff; // count
        c.g[V0][0] = sx(r32(c, V0) >> 19);
        c.g[A0][0] = c.g[V0][0] & 0xff; // current
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(1)); // last
        let is_last = c.g[A3][0] == c.g[A0][0];
        c.g[V0][0] = sp_::SEG_SIZE as u64;
        // a1 = the current segment's upper bound (the next start or the
        // end time), t2 = the segments.
        if !is_last {
            c.cycles += 5;
            c.g[V1][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
            c.mult(0, V0, A0, V0);
            c.g[T2][0] = c.g[V1][0];
            c.g[V0][0] = oc230_add(c, V0, V1);
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
        } else {
            c.cycles += 3;
            c.g[A1][0] = sx(spline.wrapping_add(sp_::END_TIME));
            c.g[T2][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
        }
        let segs = r32(c, T2);

        // The current segment?
        c.cycles += 11;
        c.g[V0][0] = c.lw(spline);
        c.g[V1][0] = sp_::SEG_SIZE as u64;
        c.g[T1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V0) >> 19) & 0xff;
        c.mult(0, T0, V0, V1);
        c.g[A0][0] = oc230_add(c, T0, T2);
        let cur = r32(c, A0);
        c.lwc1(0, cur.wrapping_add(sp_::SEG_START));
        c.c_le_s(0, 12);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, r32(c, A1));
            c.c_le_s(12, 0);
            if c.fcc() {
                c.cycles += 4;
                sp_::EVAL_SEGMENT.call(c, 0x0031_df74)?;
                return oc230_spline_exit(c, frame);
            }
            c.lwc1(0, cur.wrapping_add(sp_::SEG_START));
        }

        c.cycles += 4;
        c.c_lt_s(12, 0);
        let before = c.fcc();
        c.g[V0][0] = sx(r32(c, T1) >> 19);
        let mut search = false;
        if before {
            // Before the current one: segment 0 when t < segment 1's start.
            c.cycles += 5;
            c.lwc1(0, segs.wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
            c.c_lt_s(12, 0);
            c.g[A0][0] = c.g[T2][0];
            if c.fcc() {
                c.cycles += 7;
                c.g[V0][0] = c.g[T1][0] & sp_::CUR_CLEAR;
                let v = c.g[V0][0];
                c.sw(spline, v);
                sp_::EVAL_SEGMENT.call(c, 0x0031_dfb4)?;
                return oc230_spline_exit(c, frame);
            }
            search = true;
        }
        if !search {
            // After it: the last segment when the current is the one before.
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[A0][0] = c.g[V0][0] & 0xff;
            let next_is_last = c.g[A0][0] == c.g[V1][0];
            c.g[A1][0] = sp_::SEG_SIZE as u64;
            if next_is_last {
                c.cycles += 14;
                c.g[A0][0] = sp_::SEG_SIZE as u64;
                c.g[V0][0] = 0xffff_ffff_f807_0000;
                c.mult(0, A0, A3, A0);
                c.g[V1][0] = c.g[A3][0] & 0xff;
                c.g[V0][0] |= 0xffff;
                c.g[V1][0] = oc230_sll(c, V1, 19);
                c.g[V0][0] &= c.g[T1][0];
                c.g[A1][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
                c.g[V0][0] |= c.g[V1][0];
                c.g[A0][0] = oc230_add(c, A1, A0);
                let v = c.g[V0][0];
                c.sw(spline, v);
                sp_::EVAL_SEGMENT.call(c, 0x0031_dffc)?;
                return oc230_spline_exit(c, frame);
            }
            // The next one when t is before the start of the one after it.
            c.cycles += 7;
            c.mult(0, V1, A0, A1);
            c.g[V0][0] = oc230_add(c, V1, T2);
            c.lwc1(
                0,
                r32(c, V0).wrapping_add(2 * sp_::SEG_SIZE + sp_::SEG_START),
            );
            c.c_lt_s(12, 0);
            let in_next = c.fcc();
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            if in_next {
                c.cycles += 14;
                c.g[V1][0] = 0xffff_ffff_f807_0000;
                c.g[V0][0] &= 0xff;
                c.g[V1][0] |= 0xffff;
                c.g[V0][0] = oc230_sll(c, V0, 19);
                c.g[V1][0] &= c.g[T1][0];
                c.g[V1][0] |= c.g[V0][0];
                c.g[A0][0] = sx(r32(c, V1) >> 19);
                let v = c.g[V1][0];
                c.sw(spline, v);
                c.g[A0][0] &= 0xff;
                c.mult(0, V0, A0, A1);
                c.g[A0][0] = oc230_add(c, V0, T2);
                sp_::EVAL_SEGMENT.call(c, 0x0031_e050)?;
                return oc230_spline_exit(c, frame);
            }
        }

        // Binary search: lo (t0) .. hi (a1) over the segment starts.
        c.cycles += 2;
        c.g[A1][0] = c.g[A3][0];
        c.g[T0][0] = 0;
        loop {
            c.cycles += 10;
            c.g[V1][0] = oc230_add(c, A1, T0);
            c.g[A0][0] = sp_::SEG_SIZE as u64;
            c.g[V0][0] = sx(r32(c, V1) >> 31);
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[V1][0] = sx(((r32(c, V1) as i32) >> 1) as u32); // mid
            c.mult(0, V0, V1, A0);
            c.g[A0][0] = oc230_add(c, V0, T2);
            let seg = r32(c, A0);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.lwc1(1, seg.wrapping_add(sp_::SEG_START));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.lwc1(0, seg.wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
            } else {
                c.cycles += 1;
                c.lwc1(0, spline.wrapping_add(sp_::END_TIME));
            }
            c.cycles += 4;
            c.c_le_s(1, 12);
            if !c.fcc() {
                // t before mid: hi = mid.
                c.g[A1][0] = c.g[V1][0];
            } else {
                c.cycles += 4;
                c.c_le_s(12, 0);
                let inside = c.fcc();
                c.g[T0][0] = sx(r32(c, V1).wrapping_add(1));
                if inside {
                    c.cycles += 9;
                    c.g[V0][0] = 0xffff_ffff_f807_0000;
                    c.g[V1][0] &= 0xff;
                    c.g[V0][0] |= 0xffff;
                    c.g[V1][0] = oc230_sll(c, V1, 19);
                    c.g[V0][0] &= c.g[T1][0];
                    c.g[V0][0] |= c.g[V1][0];
                    let v = c.g[V0][0];
                    c.sw(spline, v);
                    sp_::EVAL_SEGMENT.call(c, 0x0031_e0d4)?;
                    return oc230_spline_exit(c, frame);
                }
                // t after mid's end: lo = mid + 1.
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_e060);
            }
        }
    })
}

/// `cCSICubicSpline_Evaluate`'s exit after the segment call.
pub(crate) fn oc230_spline_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

pub(crate) mod pool_alloc {
    use super::ee;
    use super::Callee;

    /// `gp+0xdd8`: the size-class pools are set up.
    pub const GP_POOLS_READY: u32 = 0xdd8;

    pub(crate) const fn pool(addr: u32, f: super::GuestFn) -> Callee {
        Callee { addr, f }
    }

    /// The 8-byte class (`MEM_Pool8Alloc(pool)`, gp slot, return address).
    pub const POOL8: (u32, Callee, u32) =
        (0x2a70, pool(0x0032_02c0, ee::s10::f_003202c0), 0x0031_9e88);

    /// The larger classes in order: (upper size limit, gp slot of the
    /// pool, `MEM_PoolNAlloc(pool, align)`, return address).
    pub const CLASSES: [(u64, u32, Callee, u32); 6] = [
        (
            0x10,
            0x2a60,
            pool(0x0032_02f8, ee::s10::f_003202f8),
            0x0031_9ea4,
        ),
        (
            0x20,
            0x2a6c,
            pool(0x0032_0330, ee::s10::f_00320330),
            0x0031_9ec0,
        ),
        (
            0x40,
            0x2a7c,
            pool(0x0032_0368, ee::s10::f_00320368),
            0x0031_9edc,
        ),
        (
            0x80,
            0x2a84,
            pool(0x0032_03b8, ee::s10::f_003203b8),
            0x0031_9ef8,
        ),
        (
            0x100,
            0x2a78,
            pool(0x0032_0408, ee::s10::f_00320408),
            0x0031_9f14,
        ),
        (
            0x200,
            0x2a68,
            pool(0x0032_0448, ee::s10::f_00320448),
            0x0031_9f30,
        ),
    ];

    /// `MEM_AllocInternal(size, align, 0, 0)`.
    pub const ALLOC_INTERNAL: Callee = pool(0x0031_9a90, ee::s10::f_00319a90);
}

/// `void* MEM_PoolAlloc(uint size, uint align)` (0x319e48): allocates a
/// small block. `size = max(size, align)`; with the pools ready it is
/// rounded up to 4 and served by the smallest size-class pool (8 ... 512
/// bytes) that fits; anything larger, or every request before the pools
/// exist (as size 0x201), goes to `MEM_AllocInternal(size, max(align, 8),
/// 0, 0)`. Frame 0x10 (ra only).
pub fn mem_pool_alloc(c: &mut Cpu) {
    use pool_alloc::*;
    guest(c, |c| {
        c.cycles += 7;
        let align = c.g[A1][0];
        c.g[A2][0] = align;
        let frame = Frame::open(c, 0x10);
        let bigger = align < c.g[A0][0];
        c.g[V0][0] = bigger as u64;
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_POOLS_READY));
        c.g[V1][0] = ready;
        frame.save_ra(c, 0);
        if !bigger {
            c.g[A0][0] = align;
        }

        'internal: {
            if ready == 0 {
                c.cycles += 2;
                c.g[A0][0] = 0x201;
                c.g[V0][0] = 7;
                break 'internal;
            }
            // Round up to a multiple of 4.
            c.cycles += 7;
            let pad = (0u32.wrapping_sub((c.g[A0][0] & 3) as u32) & 3) as u64;
            c.g[V0][0] = pad;
            let size = sx((c.g[A0][0] as u32).wrapping_add(pad as u32));
            c.g[A0][0] = size;
            c.g[V1][0] = (size < 9) as u64;
            c.g[V0][0] = (size < 0x11) as u64;
            if size < 9 {
                c.cycles += 4;
                let (slot, alloc, ret) = POOL8;
                c.g[A0][0] = c.lw(gp.wrapping_add(slot));
                alloc.call(c, ret)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            for (i, &(limit, slot, alloc, ret)) in CLASSES.iter().enumerate() {
                c.cycles += 2;
                c.g[V0][0] = match CLASSES.get(i + 1) {
                    Some(&(next, ..)) => (size < next + 1) as u64,
                    None => 7,
                };
                if size < limit + 1 {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(gp.wrapping_add(slot));
                    c.g[A1][0] = align;
                    alloc.call(c, ret)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
            }
        }

        // MEM_AllocInternal(size, max(align, 8), 0, 0); v0 holds 7.
        c.cycles += 7;
        c.g[A1][0] = 8;
        let wider = c.g[V0][0] < align;
        c.g[V0][0] = wider as u64;
        c.g[A3][0] = 0;
        if wider {
            c.g[A1][0] = align;
        }
        c.g[A2][0] = 0;
        ALLOC_INTERNAL.call(c, 0x0031_9f58)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DLHeap_Memalign (0x31f2c8)

pub(crate) const DLHEAP_MALLOC: Callee = Callee {
    addr: 0x0031_e818,
    f: ee::s10::DLHeap_Malloc,
};

pub(crate) const DLHEAP_FREE: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};

/// `int* __errno()` (through the function table).
pub(crate) const ERRNO: u32 = 0x0041_5fb8;

pub(crate) const ENOMEM: u64 = 0xc;

/// dlmalloc chunk header bits: previous chunk in use, chunk is mmapped.
pub(crate) const PREV_INUSE: u64 = 0x1;

pub(crate) const IS_MMAPPED: u64 = 0x2;

/// `void* DLHeap_Memalign(DLHeap* heap /*a0*/, uint align /*a1*/,
/// uint size /*a2*/)` (0x31f2c8): dlmalloc's `memalign`. An alignment of 16
/// or less is a plain `DLHeap_Malloc`; otherwise the alignment is raised to
/// at least 16 and to a power of two, the request padded to a chunk size
/// (`ENOMEM` and null when it is too large), and `size + align + 16` bytes
/// are allocated. A misaligned result gives up its leading part (as a free
/// chunk, or by moving the offset of an mmapped chunk), a large enough tail
/// is split off and freed. Frame 0x50 (ra at 0, s3 / s2 / s1 / s0 at
/// 0x10 .. 0x40).
pub fn dlheap_memalign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A1][0] = c.g[A2][0];
        c.g[V0][0] = (c.g[S0][0] < 0x11) as u64;
        frame.save_ra(c, 0);
        'tail: {
            if c.g[V0][0] != 0 {
                // Small alignment: plain malloc (a0 heap, a1 size).
                c.cycles += 4;
                DLHEAP_MALLOC.call(c, 0x0031_f2fc)?;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            // s0 = the alignment, at least 16 and a power of two.
            c.cycles += 8;
            c.g[V0][0] = 0xf;
            c.g[A0][0] = 0x10;
            c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[V1][0] &= c.g[S0][0];
            let pow2 = c.g[V1][0] == 0;
            c.g[V1][0] = 0x20;
            if !pow2 {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[V1][0];
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = sx(r32(c, V1) << 1);
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            c.g[S0][0] = c.g[V1][0];
                            break;
                        }
                        c.g[V1][0] = sx(r32(c, V1) << 1);
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_f338);
                        }
                    }
                }
            }
            // The request as a chunk size (s2), or ENOMEM.
            c.cycles += 5;
            c.g[V0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x13));
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                call_ptr(c, ERRNO, 0x0031_f374)?;
                c.g[V1][0] = ENOMEM;
                c.sw(r32(c, V0), c.g[V1][0]);
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
            c.g[S2][0] = 0x10;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
            }
            // Over-allocate: size + align + 16.
            c.cycles += 7;
            c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, S0)));
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            DLHEAP_MALLOC.call(c, 0x0031_f3ac)?;
            c.g[A3][0] = c.g[V0][0];
            if c.g[A3][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            c.divu(0, A3, S0); // HI = misalignment
            c.cycles += 2;
            c.cycles += 3;
            c.g[V0][0] = c.hi[0];
            let misaligned = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, A3).wrapping_sub(8)); // the chunk
            if misaligned {
                // s0: the aligned chunk, t0: the leading size.
                c.cycles += 18;
                let mem = r32(c, A3);
                let align = r32(c, S0);
                c.g[V0][0] = sx(mem.wrapping_add(align));
                c.g[V1][0] = sx(0u32.wrapping_sub(align));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[A1][0] = 0xf;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A2][0] = c.lw(mem.wrapping_sub(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                c.g[V1][0] = 0xffff_ffff_ffff_0000;
                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S1)));
                c.g[S0][0] = sx(r32(c, V0).wrapping_add(align));
                c.g[A1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                c.g[V1][0] |= 0xfffc;
                if c.g[A1][0] != 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
                c.g[V1][0] = c.g[A2][0] & c.g[V1][0];
                c.g[T0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S1)));
                c.g[A2][0] &= IS_MMAPPED;
                let mmapped = c.g[A2][0] != 0;
                c.g[A0][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
                let chunk = r32(c, S0);
                if mmapped {
                    // Move the mmapped chunk's offset forward.
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(mem.wrapping_sub(8));
                    c.g[V0][0] = c.g[A0][0] | IS_MMAPPED;
                    c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T0)));
                    c.g[V0][0] = sx(chunk.wrapping_add(8));
                    c.sw(chunk, c.g[V1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break 'tail;
                }
                // Give the leading part back.
                c.cycles += 14;
                c.g[V0][0] = c.g[A0][0] | PREV_INUSE;
                c.g[A2][0] = sx(chunk.wrapping_add(r32(c, A0)));
                c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                c.g[A1][0] = c.g[A3][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S0][0];
                let next = r32(c, A2);
                c.g[V0][0] = c.lw(next.wrapping_add(4));
                c.g[V0][0] |= PREV_INUSE;
                c.sw(next.wrapping_add(4), c.g[V0][0]);
                c.g[V1][0] = c.lw(mem.wrapping_sub(4));
                c.g[V1][0] &= PREV_INUSE;
                c.g[V1][0] |= c.g[T0][0];
                c.sw(mem.wrapping_sub(4), c.g[V1][0]);
                DLHEAP_FREE.call(c, 0x0031_f470)?;
            }
            // s1: the aligned chunk.
            c.cycles += 4;
            let chunk = r32(c, S1);
            c.g[A0][0] = c.lw(chunk.wrapping_add(4));
            c.g[V0][0] = c.g[A0][0] & IS_MMAPPED;
            let mmapped = c.g[V0][0] != 0;
            c.g[V0][0] = sx(chunk.wrapping_add(8));
            if !mmapped {
                // Split off and free a large enough tail.
                c.cycles += 7;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x10));
                c.g[V0][0] |= 0xfffc;
                c.g[V0][0] &= c.g[A0][0];
                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                let split = c.g[V1][0] != 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S2)));
                if split {
                    c.cycles += 10;
                    let need = r32(c, S2);
                    c.g[A1][0] = sx(chunk.wrapping_add(need));
                    c.g[V0][0] |= PREV_INUSE;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(r32(c, A1).wrapping_add(4), c.g[V0][0]);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(8));
                    c.g[V0][0] = c.lw(chunk.wrapping_add(4));
                    c.g[V0][0] &= PREV_INUSE;
                    c.g[V0][0] |= c.g[S2][0];
                    c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                    DLHEAP_FREE.call(c, 0x0031_f4c4)?;
                }
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DLHeap_Free (0x31ed60)

/// dlmalloc state (`*heap`): `max_fast` with the flag bits (bit 0: state
/// initialised, bit 1: fast chunks present), the fast bins (from +0), the
/// top chunk (+0x30), the unsorted bin header (+0x38, its `fd` at +0x40),
/// the trim threshold (+0x344) and the top pad (+0x348).
pub(crate) const DLF_TOP: u32 = 0x30;

pub(crate) const DLF_UNSORTED: u32 = 0x38;

pub(crate) const DLF_UNSORTED_FD: u32 = 0x40;

pub(crate) const DLF_TRIM_THRESHOLD: u32 = 0x344;

pub(crate) const DLF_TOP_PAD: u32 = 0x348;

/// Chunk fields: previous size (+0), size with flags (+4), `fd` (+8), `bk`
/// (+0xc); size flags: previous in use (bit 0), mmapped (bit 1).
pub(crate) const DLF_CHUNK_SIZE: u32 = 0x4;

pub(crate) const DLF_CHUNK_FD: u32 = 0x8;

pub(crate) const DLF_CHUNK_BK: u32 = 0xc;

/// `FASTBIN_CONSOLIDATION_THRESHOLD`.
pub(crate) const DLF_CONSOLIDATE_THRESHOLD: u64 = 0x4000;

/// `DLHeap_Consolidate` (0x31eee8).
pub(crate) const DLHEAP_CONSOLIDATE: Callee = Callee {
    addr: 0x0031_eee8,
    f: ee::s10::f_0031eee8,
};

/// `DLHeap_SysTrim` (0x31e6d8).
pub(crate) const DLHEAP_SYS_TRIM: Callee = Callee {
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
// MEM_Pool256Alloc (0x320408, cubicspline.cpp)

/// The 256-byte pool's free-list head (offset in the pool object).
pub(crate) const OC264_POOL256_FREE_HEAD: u32 = 0x2_0000;

pub(crate) const OC264_MEM_POOL_ALLOC: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::MEM_PoolAlloc,
};

/// `void* MEM_Pool256Alloc(Pool256* pool)` (0x320408): pops a block off the
/// 256-byte pool's free list; when the list is empty falls back to
/// `MEM_PoolAlloc(0x104, align)` (the caller's a1 is passed through).
pub fn mem_pool256_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x2_0000;
        frame.save_ra(c, 0);
        let head = r32(c, A0).wrapping_add(OC264_POOL256_FREE_HEAD);
        c.g[A0][0] = sx(head);
        let block = Addr(head).lw(c);
        c.g[V0][0] = block;
        if block != 0 {
            c.cycles += 3;
            let next = Addr(block as u32).lw(c);
            c.g[V1][0] = next;
            Addr(head).sw(c, next);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x104;
            OC264_MEM_POOL_ALLOC.call(c, 0x0032_042c)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCSICubicSpline_AddPoint (0x31d7e0)

/// `cCSICubicSpline_Solve(spline)`: compute the second derivatives.
pub(crate) const SPLINE_ADD_SOLVE: Callee = Callee {
    addr: 0x0031_d8b0,
    f: ee::s10::cCSICubicSpline_Solve,
};

/// cCSICubicSpline: flags word at +0 (bit 2 solved, bits 3..10 points
/// added, bits 11..18 capacity), the keys at +8.
pub(crate) const SPLINE_ADD_FLAGS: u32 = 0x0;

pub(crate) const SPLINE_ADD_KEYS: u32 = 0x8;

pub(crate) const SPLINE_ADD_SOLVED: u64 = 0x4;

/// A key: 0x14 bytes, `{x, y, end_x, end_y, d2}`.
pub(crate) const SPLINE_ADD_KEY_SIZE: u32 = 0x14;

/// `void cCSICubicSpline_AddPoint(cCSICubicSpline* self, float x, float y)`
/// (0x31d7e0): append the key `(x, y)`; when the spline is already full,
/// store `(x, y)` as the last key's end tangent instead, solve the spline
/// and mark it solved.
pub fn c_csi_cubic_spline_add_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = r32(c, S0);
        let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
        c.g[V1][0] = sx(flags >> 11) & 0xff;
        c.g[V0][0] = sx(flags >> 3);
        c.g[A2][0] = c.g[V0][0] & 0xff;
        let added = r32(c, A2);
        let x = FpuF32(c.f[12]);
        let y = FpuF32(c.f[13]);
        if c.g[A2][0] == c.g[V1][0] {
            // Full: the last key's end point, then solve.
            c.cycles += 17;
            c.g[A1][0] = SPLINE_ADD_KEY_SIZE as u64;
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let end = abi::mult(c, added, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V0));
            c.g[V1][0] = sx(end);
            Addr(end.wrapping_sub(0xc)).write_fpu(c, x);
            let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let n = (flags >> 3) & 0xff;
            let end = abi::mult(c, n, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(end);
            Addr(end.wrapping_sub(0x8)).write_fpu(c, y);
            SPLINE_ADD_SOLVE.call(c, 0x0031_d840)?;
            let me = r32(c, S0);
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) | SPLINE_ADD_SOLVED;
            let v = c.g[V0][0];
            Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).sw(c, v);
        } else {
            // Append key `added`.
            c.cycles += 20;
            c.g[A0][0] = SPLINE_ADD_KEY_SIZE as u64;
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let key = abi::mult(c, added, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V0));
            c.g[A1][0] = 0xffff_ffff_ffff_f807;
            c.g[V1][0] = sx(key);
            Addr(key).write_fpu(c, x);
            let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let n = (flags >> 3) & 0xff;
            let key = abi::mult(c, n, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(key);
            Addr(key.wrapping_add(4)).write_fpu(c, y);
            // added += 1 (8 bits at bit 3).
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c);
            let next = (sx(r32(c, V1) >> 3) as u32).wrapping_add(1) & 0xff;
            c.g[V0][0] = sx(next << 3);
            c.g[V1][0] = (c.g[V1][0] & c.g[A1][0]) | c.g[V0][0];
            let v = c.g[V1][0];
            Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).sw(c, v);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MEM_Pool64Alloc (0x320368)

/// Free-list head of the 64-byte pool.
pub(crate) const MEM_POOL64_FREE: u32 = 0x3_2000;

pub(crate) const MEM_POOL64_FALLBACK: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::f_00319e48,
};

/// `void* MEM_Pool64Alloc(Pool64* pool)` (0x320368): pop a block off the
/// 64-byte pool's free list (`pool + 0x32000`); when it is empty fall back
/// to `MEM_PoolAlloc(0x44, a1)` (a1 is whatever the caller left there).
pub fn mem_pool64_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let pool = r32(c, A0);
        c.g[V0][0] = sx(pool.wrapping_add(0x3_0000));
        let head = pool.wrapping_add(MEM_POOL64_FREE);
        let free = Addr(head).lw(c);
        c.g[V1][0] = 0x3_0000;
        if free == 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x44;
            MEM_POOL64_FALLBACK.call(c, 0x0032_038c)?;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(head);
            c.g[V0][0] = free;
            c.g[A0][0] = Addr(free as u32).lw(c);
            Addr(head).sw(c, c.g[A0][0]);
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCSICubicSpline_Solve (0x31d8b0, cubicspline.cpp)

/// Spline header word: knot count in bits 11..18, bit 0 = clamped ends (the
/// start slope in knot 0's +0x10, the end slope at +4). Knots at +8, 20 bytes
/// each: on input `{x, y, ..}` (the last knot's `{x, y}` at +8/+0xc), on
/// output `{d, c, b, y, x}`.
pub(crate) const CSP_KNOTS: u32 = 0x8;

pub(crate) const CSP_END_SLOPE: u32 = 0x4;

pub(crate) const CSP_KNOT_SIZE: u32 = 0x14;

pub(crate) const CSP_Y: u32 = 0x400;

pub(crate) const CSP_H: u32 = 0x800;

pub(crate) const CSP_ALPHA: u32 = 0xc00;

pub(crate) const CSP_L: u32 = 0x1000;

pub(crate) const CSP_MU: u32 = 0x1400;

pub(crate) const CSP_Z: u32 = 0x1800;

pub(crate) const CSP_C: u32 = 0x2000;

/// `.lit4` constants (1/3) for the natural and clamped back-substitution.
pub(crate) const CSP_THIRD_NATURAL: u32 = 0x2e48;

pub(crate) const CSP_THIRD_CLAMPED: u32 = 0x2e44;

pub(crate) const CSP_THREE: u32 = 0x4040_0000;

#[inline]
pub(crate) fn csp_fset(c: &mut Cpu, regs: &[(usize, FpuF32)]) {
    for &(r, v) in regs {
        c.f[r] = v.0;
    }
}

#[inline]
pub(crate) fn csp_add(c: &mut Cpu, r: usize, k: u32) {
    c.g[r][0] = sx(r32(c, r).wrapping_add(k));
}

#[inline]
pub(crate) fn csp_lt(c: &Cpu, a: usize, b: usize) -> u64 {
    ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
}

/// `void cCSICubicSpline_Solve(sCubicSpline* self)` (0x31d8b0): computes the
/// cubic spline coefficients through the knots (the classic tridiagonal
/// solve: h, alpha, l/mu/z forward sweep, c/b/d back substitution), natural
/// or clamped, in ten 256-float arrays on a 0x2830-byte stack frame, and
/// writes `{d, c, b, y, x}` back into each knot; the last knot's x goes to
/// +4 and the header's bits 19..26 are cleared. A leaf (s0..s2 saved at
/// 0x2800..0x2820).
pub fn c_csi_cubic_spline_solve(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x2830);
    c.g[T3][0] = c.g[A0][0];
    frame.save(c, S0, 0x2820);
    frame.save(c, S1, 0x2810);
    frame.save(c, S2, 0x2800);
    let this = r32(c, T3);
    c.g[V0][0] = c.lw(this);
    c.g[T2][0] = sx(r32(c, V0) >> 11) & 0xff;
    let base = sp(c);
    let ys = base.wrapping_add(CSP_Y);
    c.g[T7][0] = sx(ys);
    let n = r32(c, T2);
    // Copy the knots' x and y into the frame.
    if n != 0 {
        c.cycles += 4;
        c.g[V0][0] = c.lw(this.wrapping_add(CSP_KNOTS));
        c.g[A0][0] = sx(ys);
        c.g[V1][0] = sx(base);
        c.g[T0][0] = n as u64;
        loop {
            c.cycles += 9;
            let knot = r32(c, V0);
            let x = Addr(knot).read_fpu(c);
            csp_add(c, T0, u32::MAX);
            Addr(r32(c, V1)).write_fpu(c, x);
            csp_add(c, V1, 4);
            let y = Addr(knot.wrapping_add(4)).read_fpu(c);
            c.g[V0][0] = sx(knot.wrapping_add(CSP_KNOT_SIZE));
            Addr(r32(c, A0)).write_fpu(c, y);
            c.f[0] = y.0;
            let more = c.g[T0][0] != 0;
            csp_add(c, A0, 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d8e8);
            }
        }
    }
    // 0x31d8f0: the last knot's {x, y}, then h[i] = x[i+1] - x[i].
    c.cycles += 14;
    c.g[V1][0] = CSP_KNOT_SIZE as u64;
    c.g[A1][0] = c.lw(this.wrapping_add(CSP_KNOTS));
    c.mult(0, V1, T2, V1);
    let n4 = n << 2;
    c.g[V0][0] = sx(n4);
    c.g[A0][0] = sx(base.wrapping_add(n4));
    c.g[T6][0] = sx(n4);
    c.g[V0][0] = sx(ys.wrapping_add(n4));
    c.g[T0][0] = 0;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    let last = r32(c, V1);
    let x = Addr(last.wrapping_sub(0xc)).read_fpu(c);
    Addr(r32(c, A0)).write_fpu(c, x);
    let y = Addr(last.wrapping_sub(0x8)).read_fpu(c);
    Addr(r32(c, V0)).write_fpu(c, y);
    csp_fset(c, &[(0, x), (1, y)]);
    if n != 0 {
        c.cycles += 1;
        c.g[V1][0] = sx(base);
        loop {
            c.cycles += 8;
            let p = r32(c, V1);
            let x1 = Addr(p.wrapping_add(4)).read_fpu(c);
            csp_add(c, T0, 1);
            let x0 = Addr(p).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let h = x1 - x0;
            Addr(p.wrapping_add(CSP_H)).write_fpu(c, h);
            c.g[V1][0] = sx(p.wrapping_add(4));
            csp_fset(c, &[(0, h), (1, x0)]);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d948);
            }
        }
    }
    // 0x31d968: natural or clamped ends.
    c.cycles += 4;
    c.g[V0][0] = c.lw(this);
    c.g[V1][0] = c.g[V0][0] & 1;
    let clamped = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if clamped {
        c.cycles += 2;
        c.f[0] = Addr(ys).read_fpu(c).0;
        csp_solve_clamped(c, base);
    } else {
        csp_solve_natural(c, base);
    }
    // 0x31de30: write {d, c, b, y, x} back into the knots.
    c.cycles += 2;
    c.g[A0][0] = sx(base);
    if c.g[T2][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[T7][0];
        c.g[T0][0] = c.g[T2][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 25;
            let this = r32(c, T3);
            let p = r32(c, A0);
            let off = r32(c, A1);
            csp_add(c, T0, u32::MAX);
            // d, c, b into the knot's +0, +4, +8 (v1, v0, v1 hold the knot).
            for (k, src, reg) in [(0u32, 0x2400u32, V1), (4, CSP_C, V0), (8, 0x1c00, V1)] {
                let knots = c.lw(this.wrapping_add(CSP_KNOTS));
                let val = Addr(p.wrapping_add(src)).read_fpu(c);
                c.g[reg][0] = sx(off.wrapping_add(knots as u32));
                Addr(r32(c, reg).wrapping_add(k)).write_fpu(c, val);
            }
            let knots = c.lw(this.wrapping_add(CSP_KNOTS));
            let y = Addr(r32(c, A2)).read_fpu(c);
            c.g[V0][0] = sx(off.wrapping_add(knots as u32));
            csp_add(c, A2, 4);
            Addr(r32(c, V0).wrapping_add(0xc)).write_fpu(c, y);
            let knots = c.lw(this.wrapping_add(CSP_KNOTS));
            let x = Addr(p).read_fpu(c);
            c.g[V1][0] = sx(off.wrapping_add(knots as u32));
            c.g[A0][0] = sx(p.wrapping_add(4));
            Addr(r32(c, V1).wrapping_add(0x10)).write_fpu(c, x);
            c.f[0] = x.0;
            let more = c.g[T0][0] != 0;
            c.g[A1][0] = sx(off.wrapping_add(CSP_KNOT_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_de48);
            }
        }
    }
    // 0x31deac
    c.cycles += 13;
    let this = r32(c, T3);
    c.g[V1][0] = c.lw(this);
    c.g[A0][0] = sx(base.wrapping_add(r32(c, T6)));
    c.g[V0][0] = 0xffff_ffff_f807_ffff;
    let xn = Addr(r32(c, A0)).read_fpu(c);
    c.f[0] = xn.0;
    frame.restore(c, S0, 0x2820);
    c.g[V1][0] &= c.g[V0][0];
    frame.restore(c, S1, 0x2810);
    frame.restore(c, S2, 0x2800);
    Addr(this.wrapping_add(CSP_END_SLOPE)).write_fpu(c, xn);
    c.sw(this, c.g[V1][0]);
    frame.close(c);
}

/// alpha[i] = 3/h[i] (y[i+1]-y[i]) - 3/h[i-1] (y[i]-y[i-1]) for i in
/// 1..n, from `A0` (alpha) / `V1` (y) / `A1`-or-`A0` cursor registers
/// (the natural and clamped loops at 0x31d9b8 / 0x31dc18 differ only in
/// their register use).
pub(crate) fn csp_alpha_step(c: &mut Cpu, out: u32, hp: u32, yp: u32) -> [FpuF32; 6] {
    let h1 = Addr(hp).read_fpu(c);
    csp_add(c, T0, 1);
    let h0 = Addr(hp.wrapping_sub(4)).read_fpu(c);
    c.g[V0][0] = csp_lt(c, T0, T2);
    let y1 = Addr(yp).read_fpu(c);
    let k = FpuF32(c.f[5]);
    let r1 = k.div(h1);
    let y2 = Addr(yp.wrapping_add(4)).read_fpu(c);
    let r0 = k.div(h0);
    let y0 = Addr(yp.wrapping_sub(4)).read_fpu(c);
    let dy1 = y2 - y1;
    let dy0 = y1 - y0;
    let p1 = r1 * dy1;
    let p0 = r0 * dy0;
    let alpha = p1 - p0;
    Addr(out).write_fpu(c, alpha);
    [dy1, alpha, dy0, p0, y0, p1]
}

/// Natural ends (0x31d988..0x31db60): alpha, the forward sweep with
/// l[0] = 1, mu[0] = z[0] = 0, then c/b/d back substitution.
pub(crate) fn csp_solve_natural(c: &mut Cpu, base: u32) {
    let ys = base.wrapping_add(CSP_Y);
    c.cycles += 7;
    c.g[T0][0] = 1;
    c.g[T8][0] = sx(base.wrapping_add(CSP_L));
    c.g[V0][0] = csp_lt(c, T0, T2);
    c.g[T4][0] = sx(base.wrapping_add(CSP_Z));
    c.g[T9][0] = sx(base.wrapping_add(CSP_C));
    c.g[S0][0] = sx(r32(c, T2).wrapping_sub(1));
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[AT][0] = CSP_THREE as u64;
        c.f[5] = CSP_THREE;
        c.g[A0][0] = sx(base.wrapping_add(CSP_ALPHA + 4));
        c.g[V1][0] = sx(ys.wrapping_add(4));
        loop {
            c.cycles += 18;
            let a = r32(c, A0);
            let v = r32(c, V1);
            let [dy1, alpha, dy0, p0, y0, _] = csp_alpha_step(c, a, a.wrapping_sub(0x400), v);
            c.g[V1][0] = sx(v.wrapping_add(4));
            csp_fset(c, &[(0, dy1), (1, alpha), (2, dy0), (3, p0), (4, y0)]);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(a.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d9b8);
            }
        }
    }
    // 0x31da00: forward sweep.
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = 0x3f80_0000;
    c.g[T0][0] = 1;
    c.sw(base.wrapping_add(CSP_MU), 0);
    c.g[V0][0] = csp_lt(c, T0, T2);
    Addr(base.wrapping_add(CSP_L)).write_fpu(c, FpuF32::ONE);
    c.sw(base.wrapping_add(CSP_Z), 0);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = sx(r32(c, T4).wrapping_add(4));
        c.g[V1][0] = sx(base.wrapping_add(CSP_H));
        c.g[A0][0] = sx(r32(c, T8).wrapping_add(4));
        loop {
            c.cycles += 26;
            let v = r32(c, V1);
            let a0 = r32(c, A0);
            let a1 = r32(c, A1);
            let x0 = Addr(v.wrapping_sub(0x800)).read_fpu(c);
            csp_add(c, T0, 1);
            let x2 = Addr(v.wrapping_sub(0x7f8)).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let mu0 = Addr(v.wrapping_add(0xc00)).read_fpu(c);
            let dx = x2 - x0;
            let h0 = Addr(v).read_fpu(c);
            let hm = h0 * mu0;
            let l = (dx + dx) - hm;
            Addr(a0).write_fpu(c, l);
            let h1 = Addr(v.wrapping_add(4)).read_fpu(c);
            let h0b = Addr(v).read_fpu(c);
            let mu = h1.div(l);
            let alpha = Addr(v.wrapping_add(0x404)).read_fpu(c);
            Addr(v.wrapping_add(0xc04)).write_fpu(c, mu);
            c.g[V1][0] = sx(v.wrapping_add(4));
            let z0 = Addr(a1.wrapping_sub(4)).read_fpu(c);
            let lr = Addr(a0).read_fpu(c);
            let hz = h0b * z0;
            c.g[A0][0] = sx(a0.wrapping_add(4));
            let z = (alpha - hz).div(lr);
            Addr(a1).write_fpu(c, z);
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx(a1.wrapping_add(4));
            csp_fset(c, &[(0, z0), (1, lr), (2, z), (3, hz)]);
            if !more {
                c.cycles += 2;
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = 0x3f80_0000;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_da30);
            }
        }
    }
    // 0x31daa0: l[n] = 1, z[n] = c[n] = 0; back substitution.
    c.cycles += 8;
    let t6 = r32(c, T6);
    c.g[V0][0] = sx(r32(c, T8).wrapping_add(t6));
    c.g[V1][0] = sx(r32(c, T4).wrapping_add(t6));
    c.g[A0][0] = sx(r32(c, T9).wrapping_add(t6));
    let f0 = FpuF32(c.f[0]);
    Addr(r32(c, V0)).write_fpu(c, f0);
    c.g[T0][0] = c.g[S0][0];
    c.sw(r32(c, V1), 0);
    c.sw(r32(c, A0), 0);
    if (c.g[T0][0] as i64) < 0 {
        return;
    }
    c.cycles += 10;
    let j4 = r32(c, T0) << 2;
    c.g[V0][0] = sx(j4);
    c.g[A0][0] = sx(base.wrapping_add(CSP_H).wrapping_add(j4));
    c.g[A1][0] = sx(j4.wrapping_add(r32(c, T4)));
    c.lwc1(6, r32(c, GP).wrapping_sub(CSP_THIRD_NATURAL));
    c.g[AT][0] = CSP_THREE as u64;
    c.f[5] = CSP_THREE;
    c.g[V1][0] = sx(j4.wrapping_add(4).wrapping_add(r32(c, T9)));
    c.g[V0][0] = sx(j4.wrapping_add(r32(c, T7)));
    loop {
        c.cycles += 29;
        let v1 = r32(c, V1);
        let a0 = r32(c, A0);
        let a1 = r32(c, A1);
        let v0 = r32(c, V0);
        let c_next = Addr(v1).read_fpu(c);
        csp_add(c, T0, u32::MAX);
        let mu = Addr(a0.wrapping_add(0xc00)).read_fpu(c);
        let z = Addr(a1).read_fpu(c);
        c.g[A1][0] = sx(a1.wrapping_sub(4));
        let cj = z - mu * c_next;
        Addr(v1.wrapping_sub(4)).write_fpu(c, cj);
        let y0 = Addr(v0).read_fpu(c);
        let y1 = Addr(v0.wrapping_add(4)).read_fpu(c);
        let s = c_next + (cj + cj);
        let h = Addr(a0).read_fpu(c);
        c.g[V0][0] = sx(v0.wrapping_sub(4));
        let h3 = h * FpuF32(c.f[5]);
        let hs6 = (h * s) * FpuF32(c.f[6]);
        let b = (y1 - y0).div(h) - hs6;
        Addr(a0.wrapping_add(0x1400)).write_fpu(c, b);
        let cn2 = Addr(v1).read_fpu(c);
        let cj2 = Addr(v1.wrapping_sub(4)).read_fpu(c);
        c.g[V1][0] = sx(v1.wrapping_sub(4));
        let d = (cn2 - cj2).div(h3);
        Addr(a0.wrapping_add(0x1c00)).write_fpu(c, d);
        let more = (c.g[T0][0] as i64) >= 0;
        c.g[A0][0] = sx(a0.wrapping_sub(4));
        csp_fset(c, &[(0, d), (1, cj2), (2, h), (3, hs6), (4, h3)]);
        if !more {
            c.cycles += 2;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_dae8);
        }
    }
}

/// Clamped ends (0x31db64..0x31de2c; f0 = y[0]): alpha with the end slopes,
/// the forward sweep with l[0] = 2 h[0], mu[0] = 1/2, then back substitution.
pub(crate) fn csp_solve_clamped(c: &mut Cpu, base: u32) {
    let ys = base.wrapping_add(CSP_Y);
    let this = r32(c, T3);
    let t6 = r32(c, T6);
    let three = FpuF32(CSP_THREE);
    c.cycles += 40;
    c.g[A1][0] = sx(r32(c, T2).wrapping_sub(1));
    let y_first = FpuF32(c.f[0]);
    let y1 = Addr(base.wrapping_add(CSP_Y + 4)).read_fpu(c);
    let m4 = r32(c, A1) << 2;
    c.g[A0][0] = sx(m4);
    c.g[AT][0] = CSP_THREE as u64;
    c.g[A2][0] = sx(ys.wrapping_add(m4));
    let h0 = Addr(base.wrapping_add(CSP_H)).read_fpu(c);
    c.g[V0][0] = c.lw(this.wrapping_add(CSP_KNOTS));
    c.g[A3][0] = sx(ys.wrapping_add(t6));
    let end_slope = Addr(this.wrapping_add(CSP_END_SLOPE)).read_fpu(c);
    c.g[V1][0] = sx(base.wrapping_add(CSP_H));
    let start_slope = Addr(r32(c, V0).wrapping_add(0x10)).read_fpu(c);
    c.g[S1][0] = c.g[V1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(m4));
    let alpha0 = ((y1 - y_first) * three).div(h0) - start_slope * three;
    c.g[S2][0] = sx(base.wrapping_add(CSP_ALPHA));
    c.g[T0][0] = 1;
    c.g[V0][0] = sx(base.wrapping_add(CSP_ALPHA).wrapping_add(t6));
    c.g[S0][0] = c.g[A1][0];
    c.g[A0][0] = csp_lt(c, T0, T2);
    c.g[T8][0] = sx(base.wrapping_add(CSP_L));
    c.g[T5][0] = sx(base.wrapping_add(CSP_MU));
    c.g[T4][0] = sx(base.wrapping_add(CSP_Z));
    c.g[T9][0] = sx(base.wrapping_add(CSP_C));
    Addr(base.wrapping_add(CSP_ALPHA)).write_fpu(c, alpha0);
    let yn = Addr(r32(c, A3)).read_fpu(c);
    let yp = Addr(r32(c, A2)).read_fpu(c);
    let hl = Addr(r32(c, V1)).read_fpu(c);
    let q = ((yn - yp) * three).div(hl);
    let alpha_n = end_slope * three - q;
    Addr(r32(c, V0)).write_fpu(c, alpha_n);
    csp_fset(c, &[(0, q), (1, yp), (2, hl), (3, alpha_n), (4, three)]);
    if c.g[A0][0] != 0 {
        c.cycles += 5;
        c.f[5] = c.f[4];
        c.g[A1][0] = sx(base.wrapping_add(CSP_ALPHA + 4));
        c.g[V1][0] = sx(ys.wrapping_add(4));
        c.g[A0][0] = sx(base.wrapping_add(CSP_H + 4));
        loop {
            c.cycles += 19;
            let a0 = r32(c, A0);
            let v = r32(c, V1);
            let a1 = r32(c, A1);
            let [dy1, alpha, dy0, p0, y0, _] = csp_alpha_step(c, a1, a0, v);
            c.g[A0][0] = sx(a0.wrapping_add(4));
            c.g[V1][0] = sx(v.wrapping_add(4));
            csp_fset(c, &[(0, alpha), (1, dy1), (2, dy0), (3, p0), (4, y0)]);
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx(a1.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_dc18);
            }
        }
    }
    // 0x31dc64: forward sweep.
    c.cycles += 12;
    let h0 = Addr(base.wrapping_add(CSP_H)).read_fpu(c);
    c.g[T0][0] = 1;
    let a0v = Addr(base.wrapping_add(CSP_ALPHA)).read_fpu(c);
    c.g[V0][0] = csp_lt(c, T0, T2);
    let l0 = h0 + h0;
    c.g[AT][0] = 0x3f00_0000;
    let half = FpuF32(0x3f00_0000);
    Addr(base.wrapping_add(CSP_MU)).write_fpu(c, half);
    let z0 = a0v.div(l0);
    Addr(base.wrapping_add(CSP_L)).write_fpu(c, l0);
    Addr(base.wrapping_add(CSP_Z)).write_fpu(c, z0);
    csp_fset(c, &[(0, half), (1, l0), (2, z0)]);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[A3][0] = sx(r32(c, T4).wrapping_add(4));
        c.g[T1][0] = sx(r32(c, S2).wrapping_add(4));
        c.g[A2][0] = sx(r32(c, T5).wrapping_add(4));
        c.g[V1][0] = c.g[S1][0];
        c.g[A1][0] = sx(base);
        c.g[A0][0] = sx(r32(c, T8).wrapping_add(4));
        loop {
            c.cycles += 29;
            let a1 = r32(c, A1);
            let v1 = r32(c, V1);
            let a2 = r32(c, A2);
            let a3 = r32(c, A3);
            let t1 = r32(c, T1);
            let a0 = r32(c, A0);
            let x0 = Addr(a1).read_fpu(c);
            csp_add(c, T0, 1);
            let x2 = Addr(a1.wrapping_add(8)).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let hm1 = Addr(v1).read_fpu(c);
            c.g[A1][0] = sx(a1.wrapping_add(4));
            let dx = x2 - x0;
            let mu0 = Addr(a2.wrapping_sub(4)).read_fpu(c);
            let hm = hm1 * mu0;
            let l = (dx + dx) - hm;
            Addr(a0).write_fpu(c, l);
            let h = Addr(v1.wrapping_add(4)).read_fpu(c);
            let mu = h.div(l);
            Addr(a2).write_fpu(c, mu);
            c.g[A2][0] = sx(a2.wrapping_add(4));
            let hm1b = Addr(v1).read_fpu(c);
            let zp = Addr(a3.wrapping_sub(4)).read_fpu(c);
            c.g[V1][0] = sx(v1.wrapping_add(4));
            let alpha = Addr(t1).read_fpu(c);
            let hz = hm1b * zp;
            let lr = Addr(a0).read_fpu(c);
            c.g[T1][0] = sx(t1.wrapping_add(4));
            c.g[A0][0] = sx(a0.wrapping_add(4));
            let z = (alpha - hz).div(lr);
            Addr(a3).write_fpu(c, z);
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx(a3.wrapping_add(4));
            csp_fset(c, &[(0, z), (1, hz), (2, zp), (3, lr)]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_dcb0);
            }
        }
    }
    // 0x31dd24: l[n], z[n] = c[n]; back substitution.
    c.cycles += 25;
    let m4 = r32(c, S0) << 2;
    c.g[A2][0] = sx(m4);
    c.g[AT][0] = 0x4000_0000;
    let two = FpuF32(0x4000_0000);
    c.g[T5][0] = sx(r32(c, T5).wrapping_add(m4));
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(m4));
    let mu = Addr(r32(c, T5)).read_fpu(c);
    c.g[V0][0] = sx(r32(c, T8).wrapping_add(t6));
    let h = Addr(r32(c, A3)).read_fpu(c);
    c.g[T1][0] = sx(r32(c, T4).wrapping_add(m4));
    c.g[V1][0] = sx(r32(c, S2).wrapping_add(t6));
    c.g[A0][0] = sx(r32(c, T4).wrapping_add(t6));
    c.g[A1][0] = sx(r32(c, T9).wrapping_add(t6));
    c.g[T0][0] = c.g[S0][0];
    let ln = h * (two - mu);
    Addr(r32(c, V0)).write_fpu(c, ln);
    let hb = Addr(r32(c, A3)).read_fpu(c);
    let zp = Addr(r32(c, T1)).read_fpu(c);
    let alpha = Addr(r32(c, V1)).read_fpu(c);
    let hz = hb * zp;
    let zn = (alpha - hz).div(ln);
    Addr(r32(c, A0)).write_fpu(c, zn);
    Addr(r32(c, A1)).write_fpu(c, zn);
    csp_fset(c, &[(0, zn), (1, hz), (2, zp), (3, ln)]);
    if (c.g[T0][0] as i64) < 0 {
        return;
    }
    c.cycles += 10;
    c.g[V0][0] = sx(m4.wrapping_add(4));
    c.g[A1][0] = sx(base.wrapping_add(0x2400).wrapping_add(m4));
    c.lwc1(6, r32(c, GP).wrapping_sub(CSP_THIRD_CLAMPED));
    c.g[V1][0] = c.g[A3][0];
    c.g[AT][0] = CSP_THREE as u64;
    c.f[5] = CSP_THREE;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T9)));
    c.g[A3][0] = c.g[T5][0];
    c.g[A0][0] = sx(m4.wrapping_add(r32(c, T7)));
    loop {
        c.cycles += 32;
        let a3 = r32(c, A3);
        let v0 = r32(c, V0);
        let t1 = r32(c, T1);
        let a0 = r32(c, A0);
        let v1 = r32(c, V1);
        let a1 = r32(c, A1);
        let mu = Addr(a3).read_fpu(c);
        csp_add(c, T0, u32::MAX);
        let c_next = Addr(v0).read_fpu(c);
        c.g[A3][0] = sx(a3.wrapping_sub(4));
        let z = Addr(t1).read_fpu(c);
        c.g[T1][0] = sx(t1.wrapping_sub(4));
        let cj = z - mu * c_next;
        Addr(v0.wrapping_sub(4)).write_fpu(c, cj);
        let y1 = Addr(a0.wrapping_add(4)).read_fpu(c);
        let y0 = Addr(a0).read_fpu(c);
        let s = c_next + (cj + cj);
        let h = Addr(v1).read_fpu(c);
        c.g[A0][0] = sx(a0.wrapping_sub(4));
        let hs6 = (h * s) * FpuF32(c.f[6]);
        let b = (y1 - y0).div(h) - hs6;
        Addr(a1.wrapping_sub(0x800)).write_fpu(c, b);
        let cn2 = Addr(v0).read_fpu(c);
        let cj2 = Addr(v0.wrapping_sub(4)).read_fpu(c);
        let hh = Addr(v1).read_fpu(c);
        c.g[V0][0] = sx(v0.wrapping_sub(4));
        c.g[V1][0] = sx(v1.wrapping_sub(4));
        let h3 = hh * FpuF32(c.f[5]);
        let d = (cn2 - cj2).div(h3);
        Addr(a1).write_fpu(c, d);
        let more = (c.g[T0][0] as i64) >= 0;
        c.g[A1][0] = sx(a1.wrapping_sub(4));
        csp_fset(c, &[(0, h3), (1, cj2), (2, d), (3, h), (4, y0)]);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_ddb0);
        }
    }
}

pub(crate) mod dl_consolidate_f {

    /// `max_fast` (low bits are flags: bit 1 = have fast chunks).
    pub const MAX_FAST: u32 = 0x0;

    /// `fastbins[]`.
    pub const FASTBINS: u32 = 0x4;

    /// `top`.
    pub const TOP: u32 = 0x30;

    /// The unsorted bin (`bins[0]` as a fake chunk).
    pub const UNSORTED: u32 = 0x38;

    /// Chunk: `prev_size`, `size` (bit 0 = previous in use), `fd`, `bk`.
    pub const PREV_SIZE: u32 = 0x0;

    pub const SIZE: u32 = 0x4;

    pub const FD: u32 = 0x8;

    pub const BK: u32 = 0xc;
}

/// `MEM_PoolAlloc` (0x319e48).
pub(crate) const MEM_POOL_ALLOC_281: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::MEM_PoolAlloc,
};

/// 128-byte pool: the free-list head (offset from the pool).
pub(crate) const POOL128_FREE_HEAD: u32 = 0x2_5800;

/// `void* MEM_Pool128Alloc(Pool128* pool)` (0x3203b8): pop a 128-byte block
/// off the pool's free list; when the list is empty, fall back to
/// `MEM_PoolAlloc(0x84, align)` (the next size class, or dlmalloc).
pub fn mem_pool128_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let pool = r32(c, A0);
        c.g[V0][0] = sx(pool.wrapping_add(0x2_0000));
        frame.save_ra(c, 0);
        let head = Addr(pool.wrapping_add(POOL128_FREE_HEAD)).lw(c);
        c.g[V1][0] = 0x2_0000;
        if head == 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x84;
            MEM_POOL_ALLOC_281.call(c, 0x0032_03dc)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            let head_at = pool.wrapping_add(POOL128_FREE_HEAD);
            c.g[V1][0] = sx(head_at);
            c.g[V0][0] = Addr(head_at).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            let next = c.g[A0][0];
            Addr(head_at).sw(c, next);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `DLHeap_InitState` (0x31e260).
pub(crate) const DLHEAP_INIT_STATE: Callee = Callee {
    addr: 0x0031_e260,
    f: ee::s10::DLHeap_InitState,
};

/// Unlink a free chunk from its bin (`FD->bk = BK; BK->fd = FD`), with the
/// original's register use (`v0` = fd, `v1` = bk).
pub(crate) fn dl_consolidate_unlink(c: &mut Cpu, chunk: u32) {
    use dl_consolidate_f::*;
    c.g[V0][0] = Addr(chunk.wrapping_add(FD)).lw(c);
    c.g[V1][0] = Addr(chunk.wrapping_add(BK)).lw(c);
    let (fd, bk) = (c.g[V0][0], c.g[V1][0]);
    Addr((fd as u32).wrapping_add(BK)).sw(c, bk);
    Addr((bk as u32).wrapping_add(FD)).sw(c, fd);
}

/// `void DLHeap_Consolidate(DLHeap* ms, mstate av)` (0x31eee8): dlmalloc's
/// `malloc_consolidate`: with the state initialised (`max_fast != 0`),
/// clear the fast-chunks flag and free every chunk in every fast bin
/// properly: merge it with a free previous / next neighbour and put it in
/// the unsorted bin, or into the top chunk when it borders it. An
/// uninitialised state is set up with `DLHeap_InitState(av)` instead.
pub fn dlheap_consolidate(c: &mut Cpu) {
    use dl_consolidate_f::*;
    let frame = Frame { size: 0x10 };
    guest(c, |c| {
        c.cycles += 4;
        Frame::open(c, 0x10);
        let av = r32(c, A1);
        c.g[V0][0] = Addr(av.wrapping_add(MAX_FAST)).lw(c);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[A1][0];
            DLHEAP_INIT_STATE.call(c, 0x0031_f00c)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 10;
        c.g[T5][0] = sx(av.wrapping_add(UNSORTED));
        c.g[T4][0] = sx(av.wrapping_add(FASTBINS));
        c.g[V1][0] = c.g[V0][0] & 0xffff_ffff_ffff_fffd; // clear_fastchunks
        let max_fast = c.g[V1][0];
        Addr(av.wrapping_add(MAX_FAST)).sw(c, max_fast);
        c.g[V0][0] = sx(((r32(c, V1) >> 3) << 2).wrapping_sub(4));
        c.g[T7][0] = sx(av.wrapping_add(r32(c, V0))); // last fast bin
        loop {
            // One fast bin (t4).
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, T4)).lw(c);
            c.g[T6][0] = sx(r32(c, T4).wrapping_add(4));
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                c.g[T3][0] = 0xffff_ffff_ffff_fffe; // ~PREV_INUSE
                c.g[T2][0] = 0xffff_ffff_ffff_fffc; // ~SIZE_BITS
                Addr(r32(c, T4)).sw(c, 0);
                loop {
                    // One chunk (a2); t1 = the next in the bin.
                    c.cycles += 9;
                    let chunk = r32(c, A2);
                    c.g[V1][0] = Addr(chunk.wrapping_add(SIZE)).lw(c);
                    c.g[T1][0] = Addr(chunk.wrapping_add(FD)).lw(c);
                    c.g[A3][0] = c.g[V1][0] & c.g[T3][0]; // size
                    c.g[A0][0] = sx(chunk.wrapping_add(r32(c, A3))); // next chunk
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SIZE)).lw(c);
                    c.g[V1][0] = (c.g[V1][0] ^ 1) & 1;
                    c.g[T0][0] = c.g[V0][0] & c.g[T2][0]; // next size
                    if c.g[V1][0] != 0 {
                        // The previous chunk is free: merge backwards.
                        c.cycles += 7;
                        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(PREV_SIZE)).lw(c);
                        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
                        let prev = r32(c, A2);
                        dl_consolidate_unlink(c, prev);
                    }
                    c.cycles += 3;
                    let top = Addr(av.wrapping_add(TOP)).lw(c);
                    let next = r32(c, A0);
                    c.g[V0][0] = sx(next.wrapping_add(r32(c, T0)));
                    if c.g[A0][0] != top {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SIZE)).lw(c) & 1;
                        let next_size = c.g[T0][0];
                        Addr(next.wrapping_add(SIZE)).sw(c, next_size); // clear inuse
                        if c.g[V1][0] == 0 {
                            // The next chunk is free too: merge forwards.
                            c.cycles += 5;
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, T0)));
                            dl_consolidate_unlink(c, next);
                        }
                        // Into the unsorted bin.
                        c.cycles += 10;
                        let bin = r32(c, T5);
                        let chunk = r32(c, A2);
                        let size = c.g[A3][0];
                        c.g[V0][0] = Addr(bin.wrapping_add(FD)).lw(c);
                        let first = c.g[V0][0];
                        c.g[V1][0] = size | 1;
                        let head = c.g[V1][0];
                        Addr(bin.wrapping_add(FD)).sw(c, chunk as u64);
                        c.g[A0][0] = sx(chunk.wrapping_add(size as u32));
                        Addr((first as u32).wrapping_add(BK)).sw(c, chunk as u64);
                        Addr(chunk.wrapping_add(SIZE)).sw(c, head);
                        Addr(chunk.wrapping_add(FD)).sw(c, first);
                        Addr(chunk.wrapping_add(BK)).sw(c, bin as u64);
                        Addr(r32(c, A0)).sw(c, size); // foot
                    } else {
                        // Borders the top chunk: becomes part of it.
                        c.cycles += 4;
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, T0)));
                        c.g[V0][0] = c.g[A3][0] | 1;
                        let (chunk, head) = (r32(c, A2), c.g[V0][0]);
                        Addr(chunk.wrapping_add(SIZE)).sw(c, head);
                        Addr(av.wrapping_add(TOP)).sw(c, chunk as u64);
                    }
                    c.cycles += 2;
                    c.g[A2][0] = c.g[T1][0];
                    if c.g[T1][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_ef40);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[T4][0];
            let more = c.g[V0][0] != c.g[T7][0];
            c.g[T4][0] = c.g[T6][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_ef20);
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MEM_Pool16Alloc (0x3202f8)

/// The 16-byte pool's free-list head (`pool + 0x2000`).
pub(crate) const MEM_POOL16_FREE: u32 = 0x2000;

pub(crate) const MEM_POOL16_FALLBACK: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::f_00319e48,
};

/// `void* MEM_Pool16Alloc(Pool16* pool)` (0x3202f8): pop a block off the
/// 16-byte pool's free list (`pool + 0x2000`); when it is empty fall back
/// to `MEM_PoolAlloc(0x14, a1)` (a1 is whatever the caller left there).
pub fn mem_pool16_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let head = r32(c, A0).wrapping_add(MEM_POOL16_FREE);
        c.g[V0][0] = c.lw(head);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.lw(r32(c, V0)); // next free block
            c.cycles += 2;
            c.sw(head, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x14;
            MEM_POOL16_FALLBACK.call(c, 0x0032_0314)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

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

/// The 32-byte pool's free list head (`pool + 0x6000`).
pub(crate) const MEM_POOL32_FREE: u32 = 0x6000;

pub(crate) const MEM_POOL32_FALLBACK: Callee = Callee {
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

pub(crate) const DLHEAP_CONSOLIDATE_2: Callee = Callee {
    addr: 0x0031_eee8,
    f: ee::s10::DLHeap_Consolidate,
};

pub(crate) const DLHEAP_MALLOC_2: Callee = Callee {
    addr: 0x0031_e818,
    f: ee::s10::DLHeap_Malloc,
};

pub(crate) const DLHEAP_MORE_CORE: Callee = Callee {
    addr: 0x0031_ffd8,
    f: ee::s10::DLHeap_MoreCore,
};

pub(crate) const DLHEAP_FREE_2: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};

/// `__errno` (another shard: through the table).
pub(crate) const OC308_ERRNO: u32 = 0x0041_5fb8;

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
                DLHEAP_CONSOLIDATE_2.call(c, 0x0031_e32c)?;
                c.g[V0][0] = c.lw(local(c, 0));
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0xf));
                DLHEAP_MALLOC_2.call(c, 0x0031_e33c)?;
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
                        DLHEAP_FREE_2.call(c, 0x0031_e610)?;
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

// ---------------------------------------------------------------------------
// cCSICubicSpline_Alloc (0x31d660)

/// `cCSICubicSpline`: the flags word at +0 (bits 11..18: segment count,
/// bit 1: allocated, bits 3..10 cleared on alloc), the segment array at +8.
pub(crate) const SPLINE_ALLOC_DATA: u32 = 0x8;

/// The allocation name (`0x490000 - 0x24c8`).
pub(crate) const SPLINE_ALLOC_NAME: u64 = 0x0048_db38;

/// `void cCSICubicSpline_Alloc(cCSICubicSpline* self, int points)`
/// (0x31d660): frees the old segment array (`cCSICubicSpline_Free`), stores
/// `points - 1` as the 8-bit segment count in the flags, allocates
/// `count * 0x14` bytes (`operator_new_named`) and marks the spline
/// allocated (bit 1 set, bits 3..10 cleared). Frame 0x30 (s0 = self, s1 =
/// points).
pub fn c_csi_cubic_spline_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SPLINE_ALLOC_DATA)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            Callee {
                addr: 0x0031_d790,
                f: ee::s10::cCSICubicSpline_Free,
            }
            .call(c, 0x0031_d688)?;
        }
        c.cycles += 30;
        let spline = r32(c, S0);
        let flags = Addr(spline).lw(c);
        let count = sx(r32(c, S1).wrapping_sub(1)) & 0xff;
        c.g[V0][0] = 0xffff_ffff_fff8_07ff;
        let count_bits = sx((count as u32) << 11);
        c.g[V1][0] = count_bits;
        let flags = (flags & 0xffff_ffff_fff8_07ff) | count_bits;
        c.g[A2][0] = flags;
        c.g[A3][0] = 0x14;
        Addr(spline).sw(c, flags);
        c.g[A0][0] = sx((flags as u32) >> 11) & 0xff;
        c.g[A1][0] = SPLINE_ALLOC_NAME;
        c.mult(0, 0, A0, A3);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        Callee {
            addr: 0x0031_7e30,
            f: ee::s10::operator_new_named,
        }
        .call(c, 0x0031_d6d4)?;
        let spline = r32(c, S0);
        c.g[A0][0] = 0xffff_ffff_ffff_f807;
        let data = c.g[V0][0];
        let flags = (Addr(spline).lw(c) | 2) & 0xffff_ffff_ffff_f807;
        Addr(spline.wrapping_add(SPLINE_ALLOC_DATA)).sw(c, data);
        c.g[V1][0] = flags;
        Addr(spline).sw(c, flags);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS;

    #[test]
    fn dlheap_large_bin_index_matches() {
        check(
            ssx3_ee::s10::f_0031e1d8,
            dlheap_large_bin_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x201b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    2 => 0x0100_0000,
                    3 => 0x00ff_ffff,
                    _ => r.next_u32(),
                } as u64;
            },
        );
    }

    #[test]
    fn dlheap_usable_size_matches() {
        check(
            ssx3_ee::s10::f_0031fbb8,
            dlheap_usable_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x201c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 8 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                c.bus.write32(DATA + 0xfc, r.next_u32());
                for i in 0..16 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mem_pool8_free_matches() {
        check(ssx3_ee::s10::f_00320488, mem_pool8_free, 100, &|c, seed| {
            let mut r = Rng(0x201d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x800, r.next_u32());
        });
    }

    #[test]
    fn mem_pool16_free_matches() {
        check(
            ssx3_ee::s10::f_00320498,
            mem_pool16_free,
            100,
            &|c, seed| {
                let mut r = Rng(0x201e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x2000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool32_free_matches() {
        check(
            ssx3_ee::s10::f_003204a8,
            mem_pool32_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x6000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool64_free_matches() {
        check(
            ssx3_ee::s10::f_003204b8,
            mem_pool64_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x30000 + 0x2000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool128_free_matches() {
        check(
            ssx3_ee::s10::f_003204d0,
            mem_pool128_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000 + 0x5800, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool256_free_matches() {
        check(
            ssx3_ee::s10::f_003204e8,
            mem_pool256_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool512_free_matches() {
        check(
            ssx3_ee::s10::f_00320500,
            mem_pool512_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1005 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000, r.next_u32());
            },
        );
    }

    #[test]
    fn dl_heap_init_arena_matches() {
        check(
            ee::s10::f_0031ff60__generated,
            dl_heap_init_arena,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x31_ff60, 1);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x3000 + 0x1000 * (seed % 4) + seed % 8) as u64;
                c.g[A2][0] = (0x4000 + 0x1000 * (seed % 3)) as u64;
                c.pc = 0x0031_ff60;
            },
        );
    }

    use crate::difftest::fill;

    /// cCSICubicSpline_Evaluate: 1..8 segments with increasing start
    /// times, a random current segment, t at a segment start, before the
    /// first, at the end, inside the current one or anywhere in range
    /// (the real EvalSegment runs).
    #[test]
    fn c_csi_cubic_spline_evaluate_matches() {
        check(
            ee::s10::f_0031dee0__generated,
            c_csi_cubic_spline_evaluate,
            300,
            &|c, seed| {
                let mut r = Rng(0x0031_dee0 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0031_dee0;
                let f = |x: f32| x.to_bits();
                let (spline, segs) = (DATA, DATA + 0x100);
                let n = 1 + r.next_u32() % 8;
                let cur = r.next_u32() % n;
                let junk = r.next_u32() & 0xf800_07ff;
                c.bus.write32(spline, junk | n << 11 | cur << 19);
                let mut starts = Vec::new();
                for i in 0..n {
                    let t0 = i as f32 + (r.next_u32() % 50) as f32 * 0.01;
                    starts.push(t0);
                    let seg = segs + 0x14 * i;
                    for k in 0..4 {
                        let v = r.f32(4.0);
                        c.bus.write32(seg + 4 * k, v);
                    }
                    c.bus.write32(seg + 0x10, f(t0));
                }
                let end = n as f32 + 0.25;
                c.bus.write32(spline + 4, f(end));
                c.bus.write32(segs + 0x14 * n + 0x10, f(end + 5.0));
                c.bus.write32(segs + 0x14 * (n + 1) + 0x10, f(end + 6.0));
                c.bus.write32(spline + 8, segs);
                let t = match seed % 6 {
                    0 => starts[(r.next_u32() % n) as usize],
                    1 => starts[0] - 0.5,
                    2 => end,
                    3 => starts[cur as usize] + 0.3,
                    _ => starts[0] + (end - starts[0]) * (r.next_u32() % 1000) as f32 / 1000.0,
                };
                c.f[12] = f(t);
                c.g[A0][0] = sx(spline);
            },
        );
    }

    /// Runs a generated function to completion as a setup step.
    pub(crate) fn oc238_run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// MEM_PoolAlloc: pools ready (64 free nodes per size class) or not;
    /// sizes in every class, at the class limits (rounding up by 4 crosses
    /// them), above 0x200 and huge; alignments 0..128 (bigger than the size
    /// or not); the main dlmalloc heap behind MEM_AllocInternal.
    #[test]
    fn mem_pool_alloc_matches() {
        check(
            ee::s10::f_00319e48__generated,
            mem_pool_alloc,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_9e48, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                if seed % 4 != 0 {
                    c.bus.write32(GAME_GP + 0xdd8, 1);
                    // (gp slot, free-list head offset in the pool, block stride)
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
                let limits = [8u32, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200];
                let size = match seed % 6 {
                    0 => r.next_u32() % 0x220,
                    1 => limits[(r.next_u32() % 7) as usize] - (r.next_u32() % 4),
                    2 => limits[(r.next_u32() % 7) as usize] + 1 + (r.next_u32() % 3),
                    3 => r.next_u32() % 0x40,
                    4 => r.next_u32() % 0x2000,
                    _ => r.next_u32() % 0x210,
                };
                c.g[A0][0] = size as u64;
                c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128, 0x100][(r.next_u32() % 8) as usize];
                c.pc = 0x0031_9e48;
            },
        );
    }

    // --------------------------------------------------------- DLHeap_Memalign

    /// `DLHeap_Memalign` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, then a few random mallocs so results start at
    /// different alignments): small alignments (plain malloc), powers of two
    /// and other values up to 4 KiB (rounded up), sizes from 0 to beyond the
    /// arena (malloc fails), and sizes above the limit (`ENOMEM` through
    /// `__errno`). Not reached: the mmapped-chunk paths (the heap never
    /// maps).
    #[test]
    fn dlheap_memalign_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        const BACK: u32 = 0x0000_7770;
        fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
            for (i, a) in args.iter().enumerate() {
                c.g[[A0, A1, A2][i]][0] = *a;
            }
            c.g[RA][0] = BACK as u64;
            c.pc = addr;
            f(c);
            assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
            c.g[V0][0]
        }
        check(
            ee::s10::f_0031f2c8__generated,
            dlheap_memalign,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_f2c8, 0);
                let ra = c.g[RA][0];
                run(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[HEAP as u64, ARENA as u64, 0x4_0000],
                );
                for _ in 0..r.next_u32() % 4 {
                    let n = (r.next_u32() % 0x300) as u64;
                    run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, n]);
                }
                c.g[RA][0] = ra;
                c.pc = 0x0031_f2c8;
                c.g[A0][0] = sx(HEAP);
                c.g[A1][0] = match seed % 6 {
                    0 => (r.next_u32() % 17) as u64,
                    1 => 1u64 << (5 + r.next_u32() % 8),
                    _ => (17 + r.next_u32() % 4096) as u64,
                };
                c.g[A2][0] = match seed % 11 {
                    0 => 0,
                    1 => 0x10_0000 + (r.next_u32() % 0x1000) as u64,
                    2 => sx(0xffff_ffe0 + r.next_u32() % 0x20),
                    _ => (r.next_u32() % 0x2000) as u64,
                };
            },
        );
    }

    pub(crate) use crate::game::object::splinemodifier::tests::oc259_run;

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
        check(
            ee::s10::f_0031ed60__generated,
            dlheap_free,
            300,
            &|c, seed| {
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
                    let p = oc259_run(
                        c,
                        0x0031_e818,
                        ee::s10::DLHeap_Malloc,
                        &[HEAP as u64, size as u64],
                    );
                    ptrs.push(p);
                }
                let target = (r.next_u32() % n) as usize;
                for (i, &p) in ptrs.iter().enumerate() {
                    if i != target && p != 0 && r.next_u32() % 3 != 0 {
                        oc259_run(
                            c,
                            0x0031_ed60,
                            ee::s10::f_0031ed60__generated,
                            &[HEAP as u64, p],
                        );
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
            },
        );
    }

    /// MEM_Pool256Alloc: a non-empty free list (pop), or an empty one with
    /// the real MEM_PoolAlloc behind it: pools ready (served by the 512-byte
    /// pool) or not yet (dlmalloc on the arena the setup initialises).
    #[test]
    fn mem_pool256_alloc_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s10::f_00320408__generated,
            mem_pool256_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_0408, 0);
                let pool = DATA + 0x1000;
                let blocks = DATA + 0x2_8000;
                match seed % 3 {
                    0 => {
                        // Free list of a few 0x100-byte blocks.
                        let n = 1 + r.next_u32() % 4;
                        for i in 0..n {
                            let node = blocks + 0x100 * i;
                            c.bus
                                .write32(node, if i + 1 == n { 0 } else { node + 0x100 });
                        }
                        c.bus.write32(pool + OC264_POOL256_FREE_HEAD, blocks);
                    }
                    1 => {
                        // Empty: MEM_PoolAlloc takes the ready 512-byte pool.
                        c.bus.write32(pool + OC264_POOL256_FREE_HEAD, 0);
                        c.bus.write32(GAME_GP + 0xdd8, 1);
                        let p512 = DATA + 0x4_0000;
                        let base = DATA + 0x5_0000;
                        for n in 0..8u32 {
                            let node = base + 0x200 * n;
                            c.bus.write32(node, if n == 7 { 0 } else { node + 0x200 });
                        }
                        c.bus.write32(p512 + 0x2_0000, base);
                        c.bus.write32(GAME_GP + 0x2a68, p512);
                    }
                    _ => {
                        // Empty, pools not ready: the dlmalloc heap.
                        oc238_run(
                            c,
                            0x0031_ff60,
                            ee::s10::f_0031ff60,
                            &[0x004f_f250, (DATA + 0x4_0000) as u64, 0x4_0000],
                        );
                        c.bus.write32(pool + OC264_POOL256_FREE_HEAD, 0);
                        c.bus.write32(GAME_GP + 0xdd8, 0);
                    }
                }
                c.pc = 0x0032_0408;
                c.g[A0][0] = sx(pool);
                c.g[A1][0] = [0u64, 4, 8, 16, 64][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// cCSICubicSpline_AddPoint: room left (append) or full (end point and
    /// the real Solve over a small sorted spline); random flag bits around
    /// the counts.
    #[test]
    fn c_csi_cubic_spline_add_point_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(
            ee::s10::f_0031d7e0__generated,
            c_csi_cubic_spline_add_point,
            300,
            &|c, seed| {
                let mut r = Rng(0x0031_d7e0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0031_d7e0;
                let spline = DATA + 0x100;
                let keys = DATA + 0x400;
                let cap = 2 + r.next_u32() % 6;
                let added = if seed % 3 == 0 {
                    cap
                } else {
                    r.next_u32() % cap
                };
                let other = r.next_u32() & !(0xffff << 3);
                c.bus
                    .write32(spline + SPLINE_ADD_FLAGS, other | added << 3 | cap << 11);
                c.bus.write32(spline + SPLINE_ADD_KEYS, keys);
                let mut x = 0.0f32;
                for k in 0..cap {
                    x += 0.25 + (r.next_u32() % 100) as f32 / 50.0;
                    let at = keys + SPLINE_ADD_KEY_SIZE * k;
                    c.bus.write32(at, x.to_bits());
                    c.bus.write32(at + 4, r.f32(10.0));
                    c.bus.write32(at + 8, r.f32(10.0));
                    c.bus.write32(at + 0xc, r.f32(10.0));
                    c.bus.write32(at + 0x10, r.f32(10.0));
                }
                c.g[A0][0] = sx(spline);
                c.f[12] = r.f32(10.0);
                c.f[13] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(10.0)
                };
            },
        );
    }

    /// MEM_Pool64Alloc: a pool with free blocks (pop) or none (the real
    /// MEM_PoolAlloc(0x44, a1) on the fixture of its own test: size-class
    /// pools ready or not, a real dlmalloc heap behind MEM_AllocInternal;
    /// a1 an alignment 0..128).
    #[test]
    fn mem_pool64_alloc_matches() {
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(
            ee::s10::f_00320368__generated,
            mem_pool64_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_0368, 0);
                let pool64 = DATA + 0x20_0000;
                let free = seed % 3 != 0;
                if free {
                    let block = DATA + 0x30_0000 + 0x40 * (r.next_u32() % 16);
                    c.bus.write32(pool64 + MEM_POOL64_FREE, block);
                    c.bus
                        .write32(block, if seed % 2 == 0 { 0 } else { block + 0x40 });
                } else {
                    c.bus.write32(pool64 + MEM_POOL64_FREE, 0);
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
                c.g[A0][0] = sx(pool64);
                c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
                c.pc = 0x0032_0368;
            },
        );
    }

    /// cCSICubicSpline_Solve: 0-12 knots (sometimes 40) with increasing x
    /// and random y, natural or clamped ends (random slopes), random other
    /// header bits; the two 1/3 `.lit4` entries set. A leaf.
    #[test]
    fn c_csi_cubic_spline_solve_matches() {
        use crate::difftest::{check, scramble, Rng, GAME_GP};
        check(
            ee::s10::f_0031d8b0__generated,
            c_csi_cubic_spline_solve,
            300,
            &|c, seed| {
                let mut r = Rng(0x31d8_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_d8b0;
                c.g[GP][0] = GAME_GP as u64;
                c.bus
                    .write32(GAME_GP - CSP_THIRD_NATURAL, (1.0f32 / 3.0).to_bits());
                c.bus
                    .write32(GAME_GP - CSP_THIRD_CLAMPED, (1.0f32 / 3.0).to_bits());
                let spline = DATA;
                let knots = DATA + 0x100;
                let n = if seed % 11 == 0 { 40 } else { seed % 13 };
                let header = (r.next_u32() & !(0xff << 11) & !1) | n << 11 | (seed / 2) % 2;
                c.bus.write32(spline, header);
                c.bus.write32(spline + CSP_END_SLOPE, r.f32(4.0));
                c.bus.write32(spline + CSP_KNOTS, knots);
                let mut x = (r.next_u32() % 100) as f32 * 0.1;
                for k in 0..=n {
                    let e = knots + CSP_KNOT_SIZE * k;
                    let y = (r.next_u32() % 2000) as f32 * 0.01 - 10.0;
                    // Knot k's {x, y} at +0/+4; the last knot's also at +8/+0xc.
                    c.bus.write32(e, x.to_bits());
                    c.bus.write32(e + 4, y.to_bits());
                    if k + 1 == n {
                        c.bus.write32(e + 8, (x + 1.5).to_bits());
                        c.bus.write32(e + 0xc, y.to_bits());
                    }
                    c.bus.write32(e + 0x10, r.f32(2.0));
                    x += 0.25 + (r.next_u32() % 300) as f32 * 0.01;
                }
                c.g[A0][0] = spline as u64;
            },
        );
    }

    /// Run a guest function from a test setup until it returns.
    pub(crate) fn oc281_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
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

    /// `MEM_Pool128Alloc`: a non-empty free list (pop), or an empty one with
    /// the real `MEM_PoolAlloc` behind it: pools ready (the 256-byte pool
    /// serves it from its free list) or not yet (dlmalloc on an arena the
    /// setup initialises).
    #[test]
    fn mem_pool128_alloc_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        check(
            ee::s10::f_003203b8__generated,
            mem_pool128_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_03b8, 0);
                let pool = DATA + 0x1000;
                let blocks = DATA + 0x3_0000;
                match seed % 3 {
                    0 => {
                        // Free list of a few 0x80-byte blocks.
                        let n = 1 + r.next_u32() % 4;
                        for i in 0..n {
                            let node = blocks + 0x80 * i;
                            c.bus
                                .write32(node, if i + 1 == n { 0 } else { node + 0x80 });
                        }
                        c.bus.write32(pool + POOL128_FREE_HEAD, blocks);
                    }
                    1 => {
                        // Empty: MEM_PoolAlloc takes the ready 256-byte pool.
                        c.bus.write32(pool + POOL128_FREE_HEAD, 0);
                        c.bus.write32(GAME_GP + 0xdd8, 1);
                        let p256 = DATA + 0x4_0000;
                        let base = DATA + 0x6_8000;
                        for n in 0..8u32 {
                            let node = base + 0x100 * n;
                            c.bus.write32(node, if n == 7 { 0 } else { node + 0x100 });
                        }
                        c.bus.write32(p256 + 0x2_0000, base);
                        c.bus.write32(GAME_GP + 0x2a78, p256);
                    }
                    _ => {
                        // Empty, pools not ready: the dlmalloc heap.
                        let ra = c.g[RA][0];
                        oc281_run(
                            c,
                            0x0031_ff60,
                            ee::s10::DLHeap_InitArena,
                            &[0x004f_f250, (DATA + 0x8_0000) as u64, 0x4_0000],
                        );
                        c.g[RA][0] = ra;
                        c.bus.write32(pool + POOL128_FREE_HEAD, 0);
                        c.bus.write32(GAME_GP + 0xdd8, 0);
                    }
                }
                c.pc = 0x0032_03b8;
                c.g[A0][0] = sx(pool);
                c.g[A1][0] = [0u64, 4, 8, 16, 64][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// `DLHeap_Consolidate` on a real dlmalloc heap (`DLHeap_InitArena` on
    /// a 256 KiB arena): 2..12 mallocs of fast and larger sizes, some fast
    /// ones freed (into the fast bins), some larger ones freed (free
    /// neighbours: backward / forward merges), the last one bordering top
    /// (merge into top); and an uninitialised state (`DLHeap_InitState`).
    #[test]
    fn dlheap_consolidate_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        check(
            ee::s10::f_0031eee8__generated,
            dlheap_consolidate,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_eee8, 0);
                let ra = c.g[RA][0];
                oc281_run(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[HEAP as u64, ARENA as u64, 0x4_0000],
                );
                let n = 2 + r.next_u32() % 11;
                let mut ptrs = Vec::new();
                for _ in 0..n {
                    let fast = r.next_u32() % 3 != 0;
                    let size = if fast {
                        r.next_u32() % 0x39
                    } else {
                        0x48 + r.next_u32() % 0x200
                    };
                    let p = oc281_run(
                        c,
                        0x0031_e818,
                        ee::s10::DLHeap_Malloc,
                        &[HEAP as u64, size as u64],
                    );
                    ptrs.push((p, fast));
                }
                for &(p, fast) in ptrs.iter() {
                    let free = if fast {
                        r.next_u32() % 4 != 0
                    } else {
                        r.next_u32() % 2 == 0
                    };
                    if p != 0 && free {
                        oc281_run(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[HEAP as u64, p]);
                    }
                }
                let av = c.bus.read32(HEAP);
                if seed % 10 == 0 {
                    c.bus.write32(av, 0);
                }
                c.g[RA][0] = ra;
                c.pc = 0x0031_eee8;
                c.g[A0][0] = sx(HEAP);
                c.g[A1][0] = sx(av);
            },
        );
    }

    /// MEM_Pool16Alloc: a free block (last or not) is popped; an empty list
    /// falls back to the real MEM_PoolAlloc with pools ready or not (the
    /// fixture of `mem_pool64_alloc_matches`).
    #[test]
    fn mem_pool16_alloc_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s10::f_003202f8__generated,
            mem_pool16_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_02f8, 0);
                let pool16 = DATA + 0x20_0000;
                if seed % 3 != 0 {
                    let block = DATA + 0x30_0000 + 0x10 * (r.next_u32() % 16);
                    c.bus.write32(pool16 + MEM_POOL16_FREE, block);
                    c.bus
                        .write32(block, if seed % 2 == 0 { 0 } else { block + 0x10 });
                } else {
                    c.bus.write32(pool16 + MEM_POOL16_FREE, 0);
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
                c.g[A0][0] = sx(pool16);
                c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
                c.pc = 0x0032_02f8;
            },
        );
    }

    /// A block on the free list (popped) or an empty list: MEM_PoolAlloc
    /// over a test heap, with the size-class pools on or off.
    #[test]
    fn mem_pool32_alloc_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(
            ee::s10::f_00320330__generated,
            mem_pool32_alloc,
            200,
            &|c, seed| {
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
            },
        );
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
        crate::difftest::check(
            ee::s10::f_0031e2d8__generated,
            dlheap_sys_alloc,
            300,
            &|c, seed| {
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
            },
        );
    }

    /// `cCSICubicSpline_Alloc`: no old array, or an old array from the real
    /// heap (freed by the real `cCSICubicSpline_Free`); point counts 0..300
    /// (the 8-bit count wraps), random flags; the real `operator_new_named`
    /// over a test arena.
    #[test]
    fn c_csi_cubic_spline_alloc_matches() {
        use crate::game::be::beintreward::tests::world;
        use crate::game::bx::bxstring::tests::{alloc, heap};
        use crate::game::bx::bxstringctor::tests::be_library;
        crate::difftest::check_lohi(
            ee::s10::f_0031d660__generated,
            c_csi_cubic_spline_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_d660, 0);
                be_library(c);
                world(c);
                heap(c);
                let old = if seed % 3 == 0 {
                    0
                } else {
                    alloc(c, 0x14 * (1 + r.next_u32() % 16))
                };
                let spline = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, spline, 8);
                c.bus.write32(spline + SPLINE_ALLOC_DATA, old);
                c.g[A0][0] = sx(spline);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    _ => (r.next_u32() % 300) as u64,
                };
                c.pc = 0x0031_d660;
            },
        );
    }
}
