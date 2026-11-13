//! `src/bx/memblockman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::gp_at;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::intersect::riderspheretree::sw_reg;
pub(crate) use crate::game::movie::movieplayer::gp;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::wscript::wscriptman::p as p_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_2960,
        mem_get_heap_block_overhead,
        "MEM_GetHeapBlockOverhead",
    ),
    (0x0031_9a30, mem_init, "MEM_Init"),
    (0x0031_9b98, mem_get_block_size, "MEM_GetBlockSize"),
    (
        0x0031_9cc8,
        mem_report_stats_locked,
        "MEM_ReportStatsLocked",
    ),
    (0x0031_9f68, mem_pool_free, "MEM_PoolFree"),
    (0x0031_a088, mem_is_pool_pointer, "MEM_IsPoolPointer"),
    (0x0031_a130, mem_get_pool_block_size, "MEM_GetPoolBlockSize"),
    (0x0031_a200, mem_report_pool_stats, "MEM_ReportPoolStats"),
    (
        0x0031_a268,
        vu0_set_signed_unit_vector,
        "VU0_SetSignedUnitVector",
    ),
    (
        0x0031_a6b8,
        ssx_app_set_main_thread_priority,
        "SSXApp_SetMainThreadPriority",
    ),
    (0x0031_a9d8, ps2_exec_man_v11, "PS2ExecMan_v11"),
    (0x0031_aa18, ps2_exec_man_v12, "PS2ExecMan_v12"),
    (0x0031_aac8, ps2_exec_man_v05, "PS2ExecMan_v05"),
    (0x0031_ab10, ps2_exec_man_v14, "PS2ExecMan_v14"),
    (
        0x0031_ac08,
        ps2_exec_man_vsync_thread,
        "PS2ExecMan_VSyncThread",
    ),
    (
        0x0031_ac60,
        ps2_exec_man_worker_thread,
        "PS2ExecMan_WorkerThread",
    ),
    (0x0031_acd8, ps2_exec_man_v09, "PS2ExecMan_v09"),
    (0x0031_ad00, ps2_exec_man_v10, "PS2ExecMan_v10"),
    (0x0031_ad18, ssx_clear_tray_opened, "ssx_ClearTrayOpened"),
    (
        0x0031_a490,
        ps2_exec_man_vblank_end_intc,
        "PS2ExecMan_VBlankEndIntc",
    ),
    (0x0031_aa58, ps2_exec_man_wait_frame, "PS2ExecMan_WaitFrame"),
    (0x0031_ad20, ssx_cd_tray_callback, "ssx_CdTrayCallback"),
    (
        0x0031_abd0,
        ps2_exec_man_vblank_signal,
        "PS2ExecMan_VBlankSignal",
    ),
    (0x0032_0058, mem_heap_memalign, "MEM_HeapMemalign"),
    (0x0032_00c0, mem_heap_free, "MEM_HeapFree"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::VU0_RESTORE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::VU0_SAVE_CONTEXT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_REPORT_POOL_STAT6;

    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::DL_HEAP_USABLE_SIZE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::EA_MUTEX_UNLOCK;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEM_GET_POOL_BLOCK_SIZE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEM_REPORT_POOL_STATS;

    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
}

// ---------------------------------------------------------------------------
// Pool allocator.

/// `gp`-relative offsets of the pool free-list heads, smallest first.
pub(crate) mod pool {

    pub const P8: i32 = 0x2a70;

    pub const P16: i32 = 0x2a60;

    pub const P32: i32 = 0x2a6c;

    pub const P64: i32 = 0x2a7c;

    pub const P128: i32 = 0x2a84;

    pub const P256: i32 = 0x2a78;

    pub const P512: i32 = 0x2a68;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {

    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;

    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
}

/// `int MEM_GetHeapBlockOverhead(MEMHeap *heap, uint bucket)` (0x252960):
/// index the 64-entry table at 0x4d2438 and return the overhead at +0x30 of
/// the record.
pub fn mem_get_heap_block_overhead(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A1][0] = c.g[A1][0] & 0x3f;
    c.g[V0][0] = 0x4d_0000;
    c.g[V0][0] = sx(0x4d_0000u32.wrapping_add(0x2438));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory manager.

/// `int MEM_IsPoolPointer(void *ptr)` (0x31a088): true when `ptr` falls in
/// one of the fixed-size pool ranges described by the globals at gp+0x2a60..
pub fn mem_is_pool_pointer(c: &mut Cpu) {
    let a0 = abi::arg(c, 0);
    let gp = c.g[GP][0] as u32;
    'b_31a0f4: {
        c.cycles += 5;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a70)).lw(c);
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x800u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a60)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x2000u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a6c)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x6000u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2a7c)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x3_0000;
        c.g[V0][0] = c.g[V0][0] | 0x1fff;
        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2a84)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x2_0000;
        c.g[V0][0] = c.g[V0][0] | 0x57ff;
        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a78)).lw(c);
        if t {
            c.cycles += 7;
            c.g[A1][0] = 0x1_0000;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp.wrapping_add(0x2a68)).lw(c);
            if t {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                abi::ret(c);
                return;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            abi::ret(c);
            return;
        }
        break 'b_31a0f4;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    abi::ret(c);
}

/// `int MEM_GetPoolBlockSize(void *ptr)` (0x31a130): the block size (8, 16,
/// 32, 64, 128, 256 or 512) of the pool containing `ptr`, or 0.
pub fn mem_get_pool_block_size(c: &mut Cpu) {
    c.cycles += 5;
    let a0 = abi::arg(c, 0);
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x2a70)).lw(c);
    c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < 0x800u64) as u64;
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(gp.wrapping_add(0x2a60)).lw(c);
    if t {
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x2000u64) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a6c)).lw(c);
        if t {
            c.cycles += 4;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[V0][0] < 0x6000u64) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V1][0] = Addr(gp.wrapping_add(0x2a7c)).lw(c);
            if t {
                c.cycles += 7;
                c.g[V0][0] = 0x3_0000;
                c.g[V0][0] = c.g[V0][0] | 0x1fff;
                c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = Addr(gp.wrapping_add(0x2a84)).lw(c);
                if t {
                    c.cycles += 7;
                    c.g[V0][0] = 0x2_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x57ff;
                    c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(gp.wrapping_add(0x2a78)).lw(c);
                    if t {
                        c.cycles += 7;
                        c.g[A1][0] = 0x1_0000;
                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0;
                        c.g[V1][0] = Addr(gp.wrapping_add(0x2a68)).lw(c);
                        if t {
                            c.cycles += 2;
                            c.g[V0][0] = 0x100;
                            abi::ret(c);
                            return;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = 0x200;
                        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                        c.g[V1][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = 0;
                        }
                        abi::ret(c);
                        return;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x80;
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x40;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x20;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x8;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// VU0 context.

/// `void PS2ExecMan_v12(MemBlockMan* self)` (0x31aa18): save the VU0 context
/// into the block manager's area at `self+0x14080` when its flag at
/// `self+0x14048` is set.
pub fn ps2_exec_man_v12(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x4048));
        let empty = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = 0x1_0000;
            c.g[A0][0] |= 0x4080;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            callee::VU0_SAVE_CONTEXT.call(c, 0x0031_aa48)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void PS2ExecMan_v11(MemBlockMan* self)` (0x31a9d8): restore the VU0
/// context from `self+0x14080` under the same flag.
pub fn ps2_exec_man_v11(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x4048));
        let empty = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = 0x1_0000;
            c.g[A0][0] |= 0x4080;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            callee::VU0_RESTORE_CONTEXT.call(c, 0x0031_aa08)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_PoolFree(void* block)` (0x319f68): find the pool whose base lies
/// below `block` within the size class's address range and push the block
/// onto that pool's free list. Blocks outside every range are ignored.
pub fn mem_pool_free(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = c.lw(gp(c, pool::P8));
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < 0x800) as u64;
    let outside = c.g[V0][0] == 0;
    c.sd(sp(c), c.g[RA][0]);
    if !outside {
        c.cycles += 4;
        c.g[A0][0] = c.g[V1][0];
        if callee::POOL8_FREE.call(c, 0x0031_9f8c).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    // 16-byte class.
    c.cycles += 5;
    c.g[A0][0] = c.lw(gp(c, pool::P16));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < 0x2000) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL16_FREE.call(c, 0x0031_9fb0).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P32));
    // 32-byte class.
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < 0x6000) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL32_FREE.call(c, 0x0031_9fd0).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P64));
    // 64-byte class: `block - base <= 0x31fff`.
    c.cycles += 7;
    c.g[V0][0] = 0x3_0000;
    c.g[V0][0] = c.g[V0][0] | 0x1fff;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL64_FREE.call(c, 0x0031_9ffc).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P128));
    // 128-byte class: `block - base <= 0x257ff`.
    c.cycles += 7;
    c.g[V0][0] = 0x2_0000;
    c.g[V0][0] = c.g[V0][0] | 0x57ff;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL128_FREE.call(c, 0x0031_a028).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P256));
    // 256-byte class: `block - base <= 0x1ffff`.
    c.cycles += 7;
    c.g[V1][0] = 0x1_0000;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL256_FREE.call(c, 0x0031_a054).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P512));
    // 512-byte class (and the fall-through when the block is below every
    // pool base, where the unsigned difference reads as huge).
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    let outside = c.g[V0][0] == 0;
    c.g[RA][0] = c.ld(sp(c));
    if !outside {
        c.cycles += 3;
        if callee::POOL512_FREE.call(c, 0x0031_a078).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
    }
    mem_pool_free_done(c);
}

/// The shared `addiu sp, sp, 0x10; jr ra` of [`mem_pool_free`].
#[inline(always)]
pub(crate) fn mem_pool_free_done(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

/// A `gp_2`-relative global.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `void ssx_ClearTrayOpened(void)` (0x31ad18): clears the disc-tray-opened
/// latch at `D_004A3ED0`.
pub fn ssx_clear_tray_opened(c: &mut Cpu) {
    c.cycles += 2;
    let slot = gp_2(c, 0xde0);
    c.sw(slot, 0);
    abi::ret(c);
}

/// The `gp`-relative game global holding the main thread handle
/// (`D_004A3ECC`).
pub(crate) const APP_MAIN_THREAD_GP: u32 = 0x0ddc;

// ---------------------------------------------------------------------------
// SSXApp_SetMainThreadPriority / PS2ExecMan_v14.

/// `void SSXApp_SetMainThreadPriority(void)` (0x31a6b8): set the app thread's
/// priority to 0x65 (`ChangeThreadPriority(handle, 0x65)`).
pub fn ssx_app_set_main_thread_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(APP_MAIN_THREAD_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x65;
        call_ptr(c, 0x0042_3c30, 0x0031_a6cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2ExecMan_v14(void)` (0x31ab10): hand the semaphore at `self + 0xc`
/// over (`SignalSema`), then sleep forever (`SleepThread`) — the idle routine
/// of a thread. The `0x31ab30` tail call the compiler left after the loop is
/// unreachable (it emitted `li t0, 1` before the branch), so the loop below is
/// the whole body; a thread switch requested by the event hook unwinds it.
pub fn ps2_exec_man_v14(c: &mut Cpu) {
    guest(c, |c: &mut Cpu| -> Flow {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0042_3dc0, 0x0031_ab20)?;
        loop {
            c.cycles += 4;
            call_ptr(c, 0x0042_3cc0, 0x0031_ab28)?;
            if event_due(c) {
                deliver_event_here(c, 0x0031_ab20);
            }
        }
    })
}

/// `void PS2ExecMan_v05(PS2ExecMan* self)` (0x31aac8): release the exec
/// manager's semaphore (`+0x18`) if it holds one (`+0x1c`).
pub fn ps2_exec_man_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(abi::arg(c, 0), 0x1c)).lw(c);
        let held = c.g[V0][0];
        frame.restore_ra(c, 0);
        if held != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(at_2(abi::arg(c, 0), 0x18)).lw(c);
            call_ptr(c, callee::I_SIGNAL_SEMA, 0x0031_aae4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void PS2ExecMan_v09(PS2ExecMan* self, int value)` (0x31acd8): store
/// `value` in the manager's +0x14048 word and release the semaphore at_2
/// +0x14044.
pub fn ps2_exec_man_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x1_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(abi::arg(c, 0).wrapping_add(0x1_0000));
        let base = c.g[V1][0] as u32;
        let value = c.g[A1][0];
        Addr(at_2(base, 0x4048)).sw(c, value);
        c.g[A0][0] = Addr(at_2(base, 0x4044)).lw(c);
        call_ptr(c, callee::I_SIGNAL_SEMA, 0x0031_acf4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MEM_GetBlockSize(void* p)` (0x319B98): the pool class of `p` when it
/// lies in one of the six memory pools, else the heap's usable size.
pub fn mem_get_block_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        callee::MEM_GET_POOL_BLOCK_SIZE.call(c, 0x0031_9bac)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0050_0000u32.wrapping_sub(0xdb0));
            c.g[A1][0] = c.g[S0][0];
            callee::DL_HEAP_USABLE_SIZE.call(c, 0x0031_9bc4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_ReportStatsLocked(void)` (0x319CC8): report the pool statistics
/// under the memory-manager lock (`D_00538B00`).
pub fn mem_report_stats_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(0x0054_0000u32.wrapping_sub(0x7500));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MUTEX_LOCK.call(c, 0x0031_9ce4)?;
        callee::MEM_REPORT_POOL_STATS.call(c, 0x0031_9cec)?;
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MUTEX_UNLOCK.call(c, 0x0031_9cf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_ReportPoolStats(void)` (0x31a200): ask the seven size-class
/// pools to report themselves, when the pool allocator is initialized.
pub fn mem_report_pool_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, gp(c, 0xdd8));
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 14;
            c.g[RA][0] = sx(0x0031_a218);
            c.g[A0][0] = lw(c, gp(c, 0x2a70));
            callee::MEM_REPORT_POOL_STAT0.call(c, 0x0031_a218)?;
            c.g[RA][0] = sx(0x0031_a220);
            c.g[A0][0] = lw(c, gp(c, 0x2a60));
            callee::MEM_REPORT_POOL_STAT1.call(c, 0x0031_a220)?;
            c.g[RA][0] = sx(0x0031_a228);
            c.g[A0][0] = lw(c, gp(c, 0x2a6c));
            callee::MEM_REPORT_POOL_STAT2.call(c, 0x0031_a228)?;
            c.g[RA][0] = sx(0x0031_a230);
            c.g[A0][0] = lw(c, gp(c, 0x2a7c));
            callee::MEM_REPORT_POOL_STAT3.call(c, 0x0031_a230)?;
            c.g[RA][0] = sx(0x0031_a238);
            c.g[A0][0] = lw(c, gp(c, 0x2a84));
            callee::MEM_REPORT_POOL_STAT4.call(c, 0x0031_a238)?;
            c.g[RA][0] = sx(0x0031_a240);
            c.g[A0][0] = lw(c, gp(c, 0x2a78));
            callee::MEM_REPORT_POOL_STAT5.call(c, 0x0031_a240)?;
            c.g[RA][0] = sx(0x0031_a248);
            c.g[A0][0] = lw(c, gp(c, 0x2a68));
            callee::MEM_REPORT_POOL_STAT6.call(c, 0x0031_a248)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Engine and input.

/// `void PS2ExecMan_VSyncThread(PS2ExecMan* m)` (0x31ac08): the vsync thread:
/// wait on the manager's semaphore, tick the app's vsync callback, and loop
/// while the shutdown flag at `+0x20` is still clear. The event check at the
/// loop back edge re-enters the function at 0x31ac20, so this port accepts
/// that resume address too.
pub fn ps2_exec_man_vsync_thread(c: &mut Cpu) {
    loop {
        match c.pc {
            0x0031_ac08 => {
                // Entry: the prologue falls straight through to the
                // `WaitSema` block (the generator's `t` is the constant true,
                // so 0x31ac1c/0x31ac20 are *not* run before the first wait).
                c.cycles += 5;
                let frame = Frame::open(c, 0x20);
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.pc = 0x0031_ac28;
            }
            // Dead fall-through block of the original (`t` is always true);
            // only the trampoline could resume here, and it never does.
            0x0031_ac1c => {
                c.cycles += 1;
                c.pc = 0x0031_ac20;
            }
            0x0031_ac20 => {
                // Registers at the call: a0 = this, ra = 0x31ac28.
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if callee::APP_VSYNC_CALLBACK.call(c, 0x0031_ac28).is_err() {
                    return;
                }
                c.pc = 0x0031_ac28;
            }
            0x0031_ac28 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c, S0, 0x4034)).lw(c);
                if call_ptr(c, table::WAIT_SEMA, 0x0031_ac30).is_err() {
                    return;
                }
                c.g[V1][0] = Addr(p(c, S0, 0x20)).lw(c);
                if c.g[V1][0] == 0 {
                    if back_edge_unwinds(c, 0x0031_ac20) {
                        return;
                    }
                    c.pc = 0x0031_ac20;
                } else {
                    c.pc = 0x0031_ac3c;
                }
            }
            0x0031_ac3c => {
                c.cycles += 8;
                c.g[A0][0] = Addr(p(c, S0, 0x10)).lw(c);
                if call_ptr(c, table::WAKEUP_THREAD, 0x0031_ac44).is_err() {
                    return;
                }
                if call_ptr(c, table::SLEEP_THREAD, 0x0031_ac4c).is_err() {
                    return;
                }
                let frame = Frame { size: 0x20 };
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0031_ac08, 0x0031_ac5c);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Misc small leaves.

/// `int PS2ExecMan_v10(PS2ExecMan* m)` (0x0031ad00): true when the slot at
/// `+0x14048` is idle.
pub fn ps2_exec_man_v10(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0).wrapping_add(0x1_0000);
    c.g[V0][0] = 0x1_0000;
    c.g[A0][0] = sx(base);
    c.g[V0][0] = c.lw(base.wrapping_add(0x4048));
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `void MEM_Init(void* arena, size_t size)` (0x319a30): initialise the main
/// DLHeap and the small-block pools.
pub fn mem_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let arena = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x53_8b00;
        let size = c.g[A1][0];
        c.g[S0][0] = arena;
        c.g[S1][0] = size;
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_5698, 0x0031_9a54)?;
        c.g[A0][0] = 0x4f_f250;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        Addr(gp(c, 0xdcc)).sw(c, c.g[S0][0]);
        Addr(gp(c, 0xdd0)).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0031_ff60, 0x0031_9a70)?;
        c.g[V0][0] = 0x1;
        Addr(gp(c, 0xdc8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_9d20, 0x0031_9a7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_0031AC60` (0x31ac60): the PS/2 execution manager's worker thread.
/// Sets up its frame at `this + 0x10000`, then loops: run the pending job at
/// `+0x4048` through its vtable (`vtable[0x10]`) and clear it, post the
/// semaphore at `+0x4044` (`0x423de0`), and while `this+0x20` is zero go round
/// again (back-edge event check); when it is set, signal (`0x423cd0`) and free
/// (`0x423cc0`) and return. Dispatcher-shaped: the trampoline may re-enter at
/// the loop head after an unwind.
pub fn ps2_exec_man_worker_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0031_ac60 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x30);
                    c.g[V0][0] = 0x1_0000;
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S0][0] = sx(low(c, S1).wrapping_add(low(c, V0)));
                    pc = 0x0031_ac9c;
                }
                0x0031_ac80 => {
                    c.cycles += 7;
                    c.g[A1][0] = lw_r(c, S0, 0x4048);
                    c.g[V0][0] = Addr(low(c, A1)).lw(c);
                    c.g[A0][0] = lh_r(c, V0, 0x10);
                    c.g[V1][0] = lw_r(c, V0, 0x14);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0031_ac98);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0031_ac98)?;
                    sw_r(c, S0, 0x4048, 0);
                    pc = 0x0031_ac9c;
                }
                0x0031_ac9c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0031_aca4);
                    c.g[A0][0] = lw_r(c, S0, 0x4044);
                    call_ptr(c, 0x0042_3de0, 0x0031_aca4)?;
                    c.g[V1][0] = lw_r(c, S1, 0x20);
                    if c.g[V1][0] == 0 {
                        if back_edge_unwinds(c, 0x0031_ac80) {
                            return Ok(());
                        }
                        pc = 0x0031_ac80;
                    } else {
                        pc = 0x0031_acb0;
                    }
                }
                0x0031_acb0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0031_acb8);
                    c.g[A0][0] = lw_r(c, S1, 0x10);
                    call_ptr(c, 0x0042_3cd0, 0x0031_acb8)?;
                    c.g[RA][0] = sx(0x0031_acc0);
                    call_ptr(c, 0x0042_3cc0, 0x0031_acc0)?;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_ac60, 0x0031_acd4);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// VU0 helper.

/// `void VU0_SetSignedUnitVector(int flags, float* out)` (0x31a268): write four
/// components of ±1 or a table value to `out[0..3]`, choosing each component's
/// value from `flags` bits 0xb/0x3 (0x8800), bit 3 (± from the tables at
/// `gp-0x2ee0`/`gp-0x2edc`) and bit 7 (negate). Then re-pack the four floats in
/// VU0 VF1/VF2 and multiply them (`mul.xyzw`), restoring the VI registers.
pub fn vu0_set_signed_unit_vector(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[A2][0] = 0;
    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.g[SP][0];
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x8800;
        let hi_bits = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[A0][0] & 0x8;
        let bit3 = c.g[V0][0] != 0;
        if hi_bits {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x2ee0));
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            if !bit3 {
                c.cycles += 1;
                c.lwc1(0, gp(c, -0x2edc));
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0x80;
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            if bit3 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0] & 0x80;
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.g[A0][0] & 0x80;
            }
        }
        c.cycles += 2;
        let positive = c.g[V0][0] == 0;
        c.swc1(0, c.g[A1][0] as u32);
        if !positive {
            c.cycles += 1;
            c.neg_s(0, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0031_a278);
            }
            continue;
        }
        break;
    }
    c.cycles += 10;
    c.qmfc2(8, 1);
    c.qmfc2(9, 2);
    c.lqc2(1, c.g[SP][0] as u32);
    c.lqc2(2, p_2(c.g[SP][0] as u32, 0x10));
    c.vu0().mul(0b1111, 1, 1, Src::Vec(2));
    c.qmtc2(8, 1);
    c.qmtc2(9, 2);
    c.vu0().vnop();
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void PS2ExecMan_v03(self)` (0x31a920): shut down the PS2 exec manager's
/// interrupts, threads and semaphores.
pub fn ps2_exec_man_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_4880, 0x0031_a93c)?;
        c.g[S0][0] = 0x10000;
        c.g[A0][0] = 0x3;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = Addr(p_3(c.g[S0][0] as u32, 0x4290)).lw(c);
        call_ptr(c, 0x0042_3aa0, 0x0031_a950)?;
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_48e8, 0x0031_a958)?;
        c.g[V1][0] = 0x1;
        c.sw(p_3(c.g[S1][0] as u32, 0x20), c.g[V1][0]);
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x4044)).lw(c);
        call_ptr(c, 0x0042_3dc0, 0x0031_a968)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0x4034)).lw(c);
        call_ptr(c, 0x0042_3dc0, 0x0031_a970)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0x4030)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0031_a988)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0x4030)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0031_a990)?;
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x4040)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0031_a998)?;
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x4040)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0031_a9a0)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0x4034)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9a8)?;
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x4044)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9b0)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0x18)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9b8)?;
        c.g[A0][0] = Addr(p_3(c.g[S1][0] as u32, 0xc)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0031_a9c0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2ExecMan_VBlankEndIntc (0x31a490)

/// `int PS2ExecMan_VBlankEndIntc(void *arg, int (*handler)(void *))`
/// (0x31a490): the VBlank-end interrupt trampoline. Saves all 32 FPU
/// registers on the stack (through v0), calls `handler(arg)`, restores
/// them, re-enables interrupts (`ei`) and returns 1.
pub fn ps2_exec_man_vblank_end_intc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 138;
        let frame = Frame::open(c, 0x90);
        frame.save_ra(c, 0x80);
        for f in 0..32usize {
            c.mfc1(V0, f);
            Addr(frame.at(c, 4 * f as u32)).sw(c, c.g[V0][0]);
        }
        let handler = abi::arg(c, 1);
        call_ptr(c, handler, 0x0031_a5a0)?;
        for f in 0..32usize {
            c.g[V0][0] = Addr(frame.at(c, 4 * f as u32)).lw(c);
            c.mtc1(V0, f);
        }
        c.ei();
        frame.restore_ra(c, 0x80);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod cd_tray {

    /// gp+0xde4: a tray request is pending (close the tray first).
    pub const GP_TRAY_PENDING: i32 = 0xde4;

    /// gp+0xde0: the tray was opened (disc changed).
    pub const GP_TRAY_OPENED: i32 = 0xde0;

    /// The CD status bits (0x519c40): bits 1|2 = the disc must be re-mounted.
    pub const CD_STATUS: u32 = 0x0052_0000 - 0x63c0;

    pub const REMOUNT: u64 = 0x6;

    /// `sceCdTrayReq(mode, &traycnt)` (mode 2: check).
    pub const SCE_CD_TRAY_REQ: u32 = 0x0040_2618;

    pub const TRAY_CHECK: u64 = 2;

    /// `EAFile_CdMountDisc()`.
    pub const EAFILE_CD_MOUNT_DISC: u32 = 0x003e_3d78;
}

/// `int ssx_CdTrayCallback(void)` (0x31ad20): the CD tray poll: answer a
/// pending tray request (`sceCdTrayReq(2, &cnt)`), and when the status asks
/// for a re-mount, check the tray again (a non-zero count marks the tray as
/// opened), clear the re-mount bits and mount the disc. Returns 0.
pub fn ssx_cd_tray_callback(c: &mut Cpu) {
    use cd_tray::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = gp_at(c, GP_TRAY_PENDING).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = 0x0052_0000;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = Addr(CD_STATUS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            gp_at(c, GP_TRAY_PENDING).sw(c, 0);
            c.g[A0][0] = TRAY_CHECK;
            let s = sp(c);
            Addr(s).sw(c, 0);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, SCE_CD_TRAY_REQ, 0x0031_ad54)?;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0] & REMOUNT;
        c.g[V0][0] = REMOUNT;
        let remount = c.g[V1][0] == REMOUNT;
        frame.restore(c, S0, 0x30);
        if remount {
            c.cycles += 7;
            let s = sp(c);
            Addr(s + 4).sw(c, 0);
            c.g[A0][0] = TRAY_CHECK;
            c.g[A1][0] = c.g[SP][0] | 4;
            call_ptr(c, SCE_CD_TRAY_REQ, 0x0031_ad74)?;
            let s = sp(c);
            c.g[V1][0] = Addr(s + 4).lw(c);
            c.g[V0][0] = 1;
            if c.g[V1][0] != 0 {
                c.cycles += 1;
                gp_at(c, GP_TRAY_OPENED).sw(c, 1);
            }
            c.cycles += 6;
            c.g[V1][0] = 0xffff_ffff_ffff_fff9;
            c.g[V0][0] = Addr(CD_STATUS).lw(c) & c.g[V1][0];
            sw_reg(c, CD_STATUS, V0);
            call_ptr(c, EAFILE_CD_MOUNT_DISC, 0x0031_ad98)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2ExecMan_WaitFrame (0x31aa58)

/// PS2ExecMan: vtable, the frame semaphore id, the "waiting" flag.
pub(crate) const EXEC_VT: u32 = 0x8;

pub(crate) const EXEC_FRAME_SEMA: u32 = 0x18;

pub(crate) const EXEC_WAITING: u32 = 0x1c;

/// Virtual slots called before and after the wait.
pub(crate) const EXEC_PRE_WAIT_SLOT: u32 = 0x58;

pub(crate) const EXEC_POST_WAIT_SLOT: u32 = 0x60;

/// `WaitSema` and `PollSema` (EE kernel wrappers, through the table).
pub(crate) const WAIT_SEMA: u32 = 0x0042_3de0;

pub(crate) const POLL_SEMA: u32 = 0x0042_3df0;

/// `void PS2ExecMan_WaitFrame(PS2ExecMan* self)` (0x31aa58): mark the manager
/// waiting, call its pre-wait hook (virtual slot 0x58), block on the frame
/// semaphore, clear the flag, drain the semaphore (`PollSema` until it no
/// longer returns the id), then call the post-wait hook (slot 0x60).
pub fn ps2_exec_man_wait_frame(c: &mut Cpu) {
    const DRAIN_LOOP: u32 = 0x0031_aa90;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 1);
        virtual_call(c, EXEC_PRE_WAIT_SLOT, 0x0031_aa84)?;
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
        call_ptr(c, WAIT_SEMA, 0x0031_aa8c)?;
        let this = r32(c, S0);
        Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 0);
        loop {
            c.cycles += 5;
            let this = r32(c, S0);
            c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
            call_ptr(c, POLL_SEMA, 0x0031_aa98)?;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DRAIN_LOOP);
            }
        }
        c.cycles += 9;
        // (`v0` holds the vtable here, `v1` the method.)
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(EXEC_VT)).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT + 4)).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0031_aab8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The pre-wait virtual call: `v1` holds the vtable, `v0` the method.
pub(crate) fn virtual_call(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let this = r32(c, S0);
    c.g[V1][0] = Addr(this.wrapping_add(EXEC_VT)).lw(c);
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let method = r32(c, V0);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

// -- 0x0031abd0 PS2ExecMan_VBlankSignal --------------------------------------

/// Exec manager: "VSync thread busy" flag (+0x4038) and its VBlank
/// semaphore id (+0x4034).
pub(crate) const EXEC_VSYNC_BUSY: u32 = 0x4038;

pub(crate) const EXEC_VSYNC_SEMA: u32 = 0x4034;

/// `void PS2ExecMan_VBlankSignal(void)` (0x31abd0), the VBlank handler: when
/// the exec manager's VSync thread is idle (+0x4038 == 0), wake it with
/// `iSignalSema(sema)` (+0x4034). Frame 0x10 (ra).
pub fn ps2_exec_man_vblank_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // App_GetExecMan()
        Callee {
            addr: 0x0031_7520,
            f: ee::s10::f_00317520,
        }
        .call(c, 0x0031_abe0)?;
        let exec = c.g[V0][0];
        c.g[V1][0] = exec;
        let busy = Addr((exec as u32).wrapping_add(EXEC_VSYNC_BUSY)).lw(c);
        c.g[V0][0] = busy;
        frame.restore_ra(c, 0);
        if busy == 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((exec as u32).wrapping_add(EXEC_VSYNC_SEMA)).lw(c);
            // iSignalSema(sema)
            call_ptr(c, 0x0042_3dd0, 0x0031_abf8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `DLHeap_Free` (0x31ed60).
pub(crate) const DLHEAP_FREE: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};

/// `void MEM_HeapFree(DLHeap* heap, void* mem)` (0x3200c0): forwards to
/// `DLHeap_Free` (a 7-cycle `ra`-saving wrapper).
pub fn mem_heap_free(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(DLHEAP_FREE),
        0x0032_00d0,
    );
}

/// `void* MEM_HeapMemalign(DLHeap* heap, uint align, uint size)` (0x320058):
/// forwards to `DLHeap_Memalign` (0x31f2c8) in a 0x10 frame holding `ra`.
pub fn mem_heap_memalign(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0031_f2c8,
            f: ee::s10::DLHeap_Memalign,
        }),
        0x0032_0068,
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fakes::LOG_UNWIND_AT;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;

    use ssx3_ee as ee;

    use ssx3_ee::s10;

    #[test]
    fn mem_get_heap_block_overhead_matches() {
        check(
            ssx3_ee::s06::f_00252960,
            mem_get_heap_block_overhead,
            200,
            &|c, seed| {
                let mut r = Rng(0x2529_6000 ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 0x40;
                c.g[A1][0] = if seed % 2 == 0 {
                    idx as u64
                } else {
                    0xffff_ffff_0000_0000u64 | idx as u64
                };
                let node = DATA + 0x40;
                c.bus.write32(0x4d2438 + (idx << 2), node);
                c.bus.write32(node + 0x30, r.next_u32());
            },
        );
    }

    /// The seven pool bases/boundaries `MEM_*` read through `gp`.
    pub(crate) fn write_pool_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0x2a70, 0x0040_0000);
        c.bus.write32(GAME_GP + 0x2a60, 0x0050_0000);
        c.bus.write32(GAME_GP + 0x2a6c, 0x0060_0000);
        c.bus.write32(GAME_GP + 0x2a7c, 0x0080_0000);
        c.bus.write32(GAME_GP + 0x2a84, 0x00c0_0000);
        c.bus.write32(GAME_GP + 0x2a78, 0x0100_0000);
        c.bus.write32(GAME_GP + 0x2a68, 0x0180_0000);
    }

    pub(crate) fn pool_candidates() -> [u32; 12] {
        [
            0x0040_0004,
            0x0050_0004,
            0x0060_0004,
            0x0080_0004,
            0x0090_0000,
            0x00c0_0004,
            0x00d0_0000,
            0x0100_0004,
            0x0101_0000,
            0x0180_0004,
            0x0181_0000,
            0x0000_0200,
        ]
    }

    #[test]
    fn mem_is_pool_pointer_matches() {
        check(
            ssx3_ee::s10::f_0031a088,
            mem_is_pool_pointer,
            300,
            &|c, seed| {
                let mut r = Rng(0x2017 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                write_pool_globals(c);
                let cands = pool_candidates();
                c.g[A0][0] = cands[(seed as usize) % cands.len()] as u64;
            },
        );
    }

    #[test]
    fn mem_get_pool_block_size_matches() {
        check(
            ssx3_ee::s10::f_0031a130,
            mem_get_pool_block_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x2018 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                write_pool_globals(c);
                let cands = pool_candidates();
                c.g[A0][0] = cands[(seed as usize) % cands.len()] as u64;
            },
        );
    }

    #[test]
    fn ps2_exec_man_v12_matches() {
        check(
            ssx3_ee::s10::f_0031aa18__generated,
            ps2_exec_man_v12,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 61, 0);
                c.bus
                    .write32(DATA + 0x14048, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x14080, r.next_u32());
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ps2_exec_man_v11_matches() {
        check(
            ssx3_ee::s10::f_0031a9d8__generated,
            ps2_exec_man_v11,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 67, 0);
                c.bus
                    .write32(DATA + 0x14048, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x14080, r.next_u32());
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn mem_pool_free_matches() {
        const POOLS: [i32; 7] = [0x2a70, 0x2a60, 0x2a6c, 0x2a7c, 0x2a84, 0x2a78, 0x2a68];
        check(
            ee::s10::f_00319f68__generated,
            mem_pool_free,
            140,
            &|c, seed| {
                let mut r = Rng(0x319f_68 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let block = DATA + 0x2000;
                c.g[A0][0] = sx(block);
                // Every pool base starts far above the block; the seed picks one
                // that is just below it so that size class is chosen.
                let far = DATA + 0x40_0000;
                for (k, off) in POOLS.iter().enumerate() {
                    let base = if k as u32 == seed % 8 {
                        block - 0x100
                    } else {
                        far
                    };
                    c.bus.write32((GAME_GP as i32 + *off) as u32, base);
                }
                c.bus.write32(block, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_clear_tray_opened_matches() {
        check(
            ee::s10::f_0031ad18__generated,
            ssx_clear_tray_opened,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0031_ad18, seed);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus
                    .write32(0x004a_3ed0, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn ssx_app_set_main_thread_priority_matches() {
        crate::difftest::check(
            ee::s10::f_0031a6b8__generated,
            ssx_app_set_main_thread_priority,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31a6_b8, 2);
                // The app singleton's main thread handle (null covers the
                // "not created yet" path through ChangeThreadPriority).
                c.bus.write32(
                    GAME_GP + APP_MAIN_THREAD_GP,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ps2_exec_man_v14_matches() {
        crate::difftest::check(
            ee::s10::f_0031ab10__generated,
            ps2_exec_man_v14,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31ab_10, 1);
                // The body sleeps forever; the fake syscall unwinds after a few
                // iterations, which is how the original leaves the loop too.
                c.bus.write32(LOG_UNWIND_AT, seed % 4);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x10c, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_exec_man_v05_matches() {
        check(
            s10::f_0031aac8__generated,
            ps2_exec_man_v05,
            120,
            &|c, seed| {
                machine(c, seed, 0x31aac8, 1);
                clear(c, OBJ, 0x40);
                c.g[A0][0] = sx(OBJ);
                // A held semaphore / none.
                c.bus.write32(OBJ + 0x1c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(OBJ + 0x18, seed as u32 & 3);
            },
        );
    }

    #[test]
    fn ps2_exec_man_v09_matches() {
        check(
            s10::f_0031acd8__generated,
            ps2_exec_man_v09,
            120,
            &|c, seed| {
                machine(c, seed, 0x31acd8, 1);
                clear(c, OBJ + 0x1_4000, 0x20);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32 & 0xff);
                c.bus.write32(OBJ + 0x1_4044, seed as u32 & 3);
            },
        );
    }

    #[test]
    fn mem_get_block_size_matches() {
        check(
            ssx3_ee::s10::f_00319b98__generated,
            mem_get_block_size,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x319b_98, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    w(c, GAME_GP + 0x2a70, DATA);
                }
            },
        );
    }

    #[test]
    fn mem_report_stats_locked_matches() {
        check(
            ssx3_ee::s10::f_00319cc8__generated,
            mem_report_stats_locked,
            60,
            &|c, seed| {
                machine(c, seed, 0x319c_c8, 3);
                belib(c);
            },
        );
    }

    #[test]
    fn mem_report_pool_stats_matches() {
        check(
            ee::s10::f_0031a200__generated,
            mem_report_pool_stats,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x31a200, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.bus
                    .write32(gp_at(0xdd8), if seed % 3 == 0 { 0 } else { 1 });
                for off in [0x2a60i32, 0x2a68, 0x2a6c, 0x2a70, 0x2a78, 0x2a7c, 0x2a84] {
                    c.bus.write32(gp_at(off), 0);
                }
            },
        );
    }

    // ------------------------------------------------------------- engine

    #[test]
    fn ps2_exec_man_vsync_thread_matches() {
        check(
            ssx3_ee::s10::f_0031ac08__generated,
            ps2_exec_man_vsync_thread,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xac08, 0);
                let m = DATA;
                // The app's vsync clock. The tick loop must run exactly once:
                // (int)v24 - (int)v28 - 1 == -1, so v24 = 1.0 and v28 = 0.
                let clock = DATA + 0x100;
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32() % 1000);
                c.bus.write32(clock + 0x24, 0x3f80_0000);
                c.bus.write32(clock + 0x28, 0);
                let ticks = vtable(c, &mut r, &[0x38]);
                c.bus.write32(clock + 0x5c, ticks);
                let owner = DATA + 0x200;
                let handler = vtable(c, &mut r, &[0x28]);
                c.bus.write32(clock + 8, owner);
                c.bus.write32(owner + 8, handler);
                c.bus.write32(m + 0x4034, 0x1234);
                c.bus.write32(m + 0x10, 0x5678);
                // Three shapes of the thread body:
                //  0: shutdown flag set -> wait, wake the app thread, sleep,
                //     return (the whole exit path).
                //  1: flag clear, but the *first* wait unwinds: proves no
                //     vsync callback runs before the first `WaitSema`.
                //  2: flag clear and the wait succeeds, so one full iteration
                //     runs (wait -> flag -> App_VSyncCallback) and the
                //     callback's own vtable call unwinds inside it.
                let unwind = match seed % 3 {
                    0 => {
                        c.bus.write32(m + 0x20, 1);
                        u32::MAX
                    }
                    1 => {
                        c.bus.write32(m + 0x20, 0);
                        0
                    }
                    _ => {
                        c.bus.write32(m + 0x20, 0);
                        1
                    }
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                c.g[A0][0] = m as u64;
                // The thread body is entered at its own address.
                c.pc = 0x0031_ac08;
            },
        );
    }

    #[test]
    fn ps2_exec_man_v10_matches() {
        check(
            ssx3_ee::s10::f_0031ad00__generated,
            ps2_exec_man_v10,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31ad00, 0);
                c.g[A0][0] = sx(DATA);
                word(
                    c,
                    DATA + 0x14048,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    // ---------------------------------------------------------- memory / UI

    #[test]
    fn mem_init_matches() {
        check(
            ssx3_ee::s10::f_00319a30__generated,
            mem_init,
            12,
            &|c, seed| {
                fakes::machine(c, seed, 0x319a30, 0);
                let arena = DATA + 0x40000;
                c.g[A0][0] = arena as u64;
                c.g[A1][0] = 0x40000 - (seed as u64 % 0x40) * 16;
            },
        );
    }

    /// A vtable dispatch target: object `obj` with vtable `vt` whose slot `off`
    /// is a fake callee.
    pub(crate) fn fake_slot(c: &mut Cpu, obj: u32, vt: u32, off: u32) {
        w(c, obj, vt);
        slot(c, vt, off, 0, FAKE);
    }

    #[test]
    fn ps2_exec_man_worker_thread_matches() {
        check(
            ee::s10::f_0031ac60__generated,
            ps2_exec_man_worker_thread,
            24,
            &|c, seed| {
                machine(c, seed, 0x31ac60, 4);
                be_library(c);
                world(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.pc = 0x0031_ac60;
                // the loop terminates when the "done" flag at +0x20 is set
                w(c, this + 0x20, 1);
                w(c, this + 0x10, DATA + 0x400);
                if seed % 2 == 0 {
                    let job = DATA + 0x800;
                    w(c, this + 0x1_0000 + 0x4048, job);
                    fake_slot(c, job, DATA + 0x2000, 0x10);
                }
            },
        );
    }

    // ------------------------------------------------------------- VU0 unit

    #[test]
    fn vu0_set_signed_unit_vector_matches() {
        check(
            ssx3_ee::s10::f_0031a268__generated,
            vu0_set_signed_unit_vector,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x0031_a268);
                let t0 = gp(c, -0x2ee0);
                c.bus.write32(t0, r.edge_f32());
                let t1 = gp(c, -0x2edc);
                c.bus.write32(t1, r.edge_f32());
                c.g[A0][0] = if seed % 4 == 0 {
                    (r.next_u32() & 0xffff_0000) as u64
                } else {
                    r.next_u32() as u64
                };
                c.g[A1][0] = sx(DATA + 0x4000);
            },
        );
    }

    /// PS2ExecMan_VBlankEndIntc: random FPU registers and status; the
    /// handler is a fake that clobbers f0 and sometimes unwinds.
    #[test]
    fn ps2_exec_man_vblank_end_intc_matches() {
        covered(
            ee::s10::f_0031a490__generated,
            ps2_exec_man_vblank_end_intc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31a490, 1);
                c.pc = 0x0031_a490;
                c.cop0[12] = r.next_u32();
                c.g[A1][0] = FAKE as u64;
            },
        );
    }

    /// Write a gp-relative global.
    pub(crate) fn put_gp(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// The tray poll with and without a pending request, and status words
    /// without the re-mount bits (both set means `EAFile_CdMountDisc`, which
    /// mounts a disc and cannot be bounded in a test: that branch is not
    /// reached here). The real `sceCdTrayReq` takes its "busy" exit (the
    /// fake syscalls never grant the semaphore).
    #[test]
    fn ssx_cd_tray_callback_matches() {
        use cd_tray::*;
        check(
            ssx3_ee::s10::f_0031ad20__generated,
            ssx_cd_tray_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31ad20, 0);
                c.pc = 0x0031_ad20;
                put_gp(
                    c,
                    GP_TRAY_PENDING,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                put_gp(c, GP_TRAY_OPENED, r.next_u32());
                let status = r.next_u32();
                let status = match seed / 2 % 3 {
                    0 => status & !6,
                    1 => status & !4 | 2,
                    _ => status & !2 | 4,
                };
                c.bus.write32(CD_STATUS, status);
            },
        );
    }

    /// `PS2ExecMan_WaitFrame`: fake pre/post hooks (v-table), the real
    /// `WaitSema`/`PollSema` wrappers on the fake syscall; `PollSema`
    /// answers the semaphore id 0..3 times before something else; any call
    /// may unwind.
    #[test]
    fn ps2_exec_man_wait_frame_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        const SEMA: u32 = 0x17;
        check(
            ee::s10::f_0031aa58__generated,
            ps2_exec_man_wait_frame,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_aa58, 6);
                c.pc = 0x0031_aa58;
                let this = DATA;
                let vt = vtable(c, &mut r, &[EXEC_PRE_WAIT_SLOT, EXEC_POST_WAIT_SLOT]);
                c.bus.write32(this + EXEC_VT, vt);
                c.bus.write32(this + EXEC_FRAME_SEMA, SEMA);
                c.bus.write32(this + EXEC_WAITING, r.next_u32());
                // Calls: 0 pre hook, 1 WaitSema, 2.. PollSema, then the post hook.
                let polls = seed % 4;
                for k in 0..polls {
                    c.bus.write32(LOG_RESULTS + 4 * (2 + k), SEMA);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // -- 0x0031abd0 PS2ExecMan_VBlankSignal ------------------------------------

    /// `PS2ExecMan_VBlankSignal` against the generated body: the VSync thread
    /// busy (no signal) or idle (the real `iSignalSema`, whose syscall is a
    /// fake: some unwind).
    #[test]
    fn ps2_exec_man_vblank_signal_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        const APP: u32 = DATA + 0x1_0000;
        const EXEC: u32 = DATA + 0x2_0000;
        covered(
            ee::s10::f_0031abd0__generated,
            ps2_exec_man_vblank_signal,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_abd0, 1);
                c.pc = 0x0031_abd0;
                c.bus.write32(GAME_GP + 0x2a74, APP);
                c.bus.write32(APP + 8, EXEC);
                let busy = if r.next_u32() % 3 == 0 {
                    r.next_u32() | 1
                } else {
                    0
                };
                c.bus.write32(EXEC + EXEC_VSYNC_BUSY, busy);
                c.bus.write32(EXEC + EXEC_VSYNC_SEMA, r.next_u32() % 64);
            },
        );
    }

    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::bx::cubicspline::tests::oc281_run;

    /// `MEM_HeapFree` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, a few mallocs, some freed first): null, small and
    /// large chunks, as in `dlheap_free_matches`.
    #[test]
    fn mem_heap_free_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        check(
            ee::s10::f_003200c0__generated,
            mem_heap_free,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_00c0, 0);
                let ra = c.g[RA][0];
                oc281_run(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[HEAP as u64, ARENA as u64, 0x4_0000],
                );
                let n = 1 + r.next_u32() % 6;
                let mut ptrs = Vec::new();
                for _ in 0..n {
                    let size = match r.next_u32() % 3 {
                        0 => r.next_u32() % 0x40,
                        1 => 0x40 + r.next_u32() % 0x800,
                        _ => 0x1000 + r.next_u32() % 0x3000,
                    };
                    ptrs.push(oc281_run(
                        c,
                        0x0031_e818,
                        ee::s10::DLHeap_Malloc,
                        &[HEAP as u64, size as u64],
                    ));
                }
                let target = (r.next_u32() % n) as usize;
                for (i, &p) in ptrs.iter().enumerate() {
                    if i != target && p != 0 && r.next_u32() % 3 == 0 {
                        oc281_run(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[HEAP as u64, p]);
                    }
                }
                c.g[RA][0] = ra;
                c.pc = 0x0032_00c0;
                c.g[A0][0] = sx(HEAP);
                c.g[A1][0] = if seed % 10 == 0 { 0 } else { ptrs[target] };
            },
        );
    }

    /// `MEM_HeapMemalign` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, a few mallocs first), with small, power-of-two and odd
    /// alignments and sizes from zero to too large.
    #[test]
    fn mem_heap_memalign_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        const BACK: u32 = 0x0000_7770;
        fn heap_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) {
            for (i, a) in args.iter().enumerate() {
                c.g[[A0, A1, A2][i]][0] = *a;
            }
            c.g[RA][0] = BACK as u64;
            c.pc = addr;
            f(c);
            assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        }
        check(
            ee::s10::f_00320058__generated,
            mem_heap_memalign,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0058, 0);
                heap_call(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[HEAP as u64, ARENA as u64, 0x4_0000],
                );
                for _ in 0..r.next_u32() % 4 {
                    let n = (r.next_u32() % 0x300) as u64;
                    heap_call(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, n]);
                }
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0032_0058;
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
}
