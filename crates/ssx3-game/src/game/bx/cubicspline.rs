//! `src/bx/cubicspline.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
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
use ssx3_ee_rt::float;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Flow;

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
    (0x0031_d7e0, c_csi_cubic_spline_add_point, "cCSICubicSpline_AddPoint"),
    (0x0032_0368, mem_pool64_alloc, "MEM_Pool64Alloc"),
    (0x0032_0408, mem_pool256_alloc, "MEM_Pool256Alloc"),
];

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
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
pub(crate) const SPLINE_ADD_SOLVE: Callee = Callee { addr: 0x0031_d8b0, f: ee::s10::cCSICubicSpline_Solve };


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


pub(crate) const MEM_POOL64_FALLBACK: Callee = Callee { addr: 0x0031_9e48, f: ee::s10::f_00319e48 };



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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
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
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::{fill, put_words};

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
    use super::*;

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
    use super::*;


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
        check(ee::s10::f_0031f2c8__generated, dlheap_memalign, 300, &|c, seed| {
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
        });
    }
    use super::*;
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
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::{elf_image, put_vec};


    /// MEM_Pool256Alloc: a non-empty free list (pop), or an empty one with
    /// the real MEM_PoolAlloc behind it: pools ready (served by the 512-byte
    /// pool) or not yet (dlmalloc on the arena the setup initialises).
    #[test]
    fn mem_pool256_alloc_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(ee::s10::f_00320408__generated, mem_pool256_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_0408, 0);
            let pool = DATA + 0x1000;
            let blocks = DATA + 0x2_8000;
            match seed % 3 {
                0 => {
                    // Free list of a few 0x100-byte blocks.
                    let n = 1 + r.next_u32() % 4;
                    for i in 0..n {
                        let node = blocks + 0x100 * i;
                        c.bus.write32(node, if i + 1 == n { 0 } else { node + 0x100 });
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
        });
    }


    /// cCSICubicSpline_AddPoint: room left (append) or full (end point and
    /// the real Solve over a small sorted spline); random flag bits around
    /// the counts.
    #[test]
    fn c_csi_cubic_spline_add_point_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(ee::s10::f_0031d7e0__generated, c_csi_cubic_spline_add_point, 300, &|c, seed| {
            let mut r = Rng(0x0031_d7e0 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0031_d7e0;
            let spline = DATA + 0x100;
            let keys = DATA + 0x400;
            let cap = 2 + r.next_u32() % 6;
            let added = if seed % 3 == 0 { cap } else { r.next_u32() % cap };
            let other = r.next_u32() & !(0xffff << 3);
            c.bus.write32(spline + SPLINE_ADD_FLAGS, other | added << 3 | cap << 11);
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
            c.f[13] = if seed % 5 == 0 { r.edge_f32() } else { r.f32(10.0) };
        });
    }


    /// MEM_Pool64Alloc: a pool with free blocks (pop) or none (the real
    /// MEM_PoolAlloc(0x44, a1) on the fixture of its own test: size-class
    /// pools ready or not, a real dlmalloc heap behind MEM_AllocInternal;
    /// a1 an alignment 0..128).
    #[test]
    fn mem_pool64_alloc_matches() {
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(ee::s10::f_00320368__generated, mem_pool64_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_0368, 0);
            let pool64 = DATA + 0x20_0000;
            let free = seed % 3 != 0;
            if free {
                let block = DATA + 0x30_0000 + 0x40 * (r.next_u32() % 16);
                c.bus.write32(pool64 + MEM_POOL64_FREE, block);
                c.bus.write32(block, if seed % 2 == 0 { 0 } else { block + 0x40 });
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
        });
    }
}
