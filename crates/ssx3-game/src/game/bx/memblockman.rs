//! `src/bx/memblockman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::gp_at;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
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
use ssx3_ee_rt::vu0::X;
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
    (0x0025_6188, ssx_mount_slus_ovf_big, "ssx_MountSlusOvfBig"),
    (0x0031_9660, ssx_get_modules_dir, "ssx_GetModulesDir"),
    (0x0031_9690, ssx_build_module_path, "ssx_BuildModulePath"),
    (0x0031_9718, ssx_load_iop_module, "ssx_LoadIopModule"),
    (0x0031_97d0, ssx_load_msifrpc, "ssx_LoadMsifrpc"),
    (
        0x0031_9930,
        ssx_install_power_off_handler,
        "ssx_InstallPowerOffHandler",
    ),
    (0x0031_a6d8, ps2_exec_man_init, "PS2ExecMan_Init"),
    (
        0x0031_7478,
        ps2_exec_man_set_callback,
        "PS2ExecMan_SetCallback",
    ),
    (0x0031_a920, ps2_exec_man_v03, "PS2ExecMan_v03"),
    (0x0031_aaf0, ps2_exec_man_wait_sema, "PS2ExecMan_WaitSema"),
    (
        0x0032_09c0,
        ps2_exec_man_forward_slot05,
        "PS2ExecMan_ForwardSlot05",
    ),
    (
        0x0032_09f0,
        ps2_exec_man_forward_slot04,
        "PS2ExecMan_ForwardSlot04",
    ),
    (
        0x0032_0a20,
        ps2_exec_man_forward_slot06,
        "PS2ExecMan_ForwardSlot06",
    ),
    (
        0x0032_0a60,
        ps2_exec_man_forward_slot11,
        "PS2ExecMan_ForwardSlot11",
    ),
    (
        0x0032_0a90,
        ps2_exec_man_forward_slot12,
        "PS2ExecMan_ForwardSlot12",
    ),
    (0x0032_0b08, ps2_exec_man_v06, "PS2ExecMan_v06"),
    (0x0025_6088, ssx_load_net_gui_elf, "ssx_LoadNetGuiElf"),
    (0x0031_9610, ssx_iop_init, "ssx_IopInit"),
    (0x0031_9790, ssx_unload_iop_module, "ssx_UnloadIopModule"),
    (0x0031_9800, ssx_exit_msif_rpc, "ssx_ExitMsifRpc"),
    (0x0031_9820, ssx_iop_reboot, "ssx_IopReboot"),
    (0x0031_98a0, ssx_power_off_thread, "ssx_PowerOffThread"),
    (
        0x0031_9910,
        ssx_power_off_signal_sema,
        "ssx_PowerOffSignalSema",
    ),
    (0x0031_a308, vu0_save_context, "VU0_SaveContext"),
    (0x0031_a3c0, vu0_restore_context, "VU0_RestoreContext"),
    (
        0x0031_ab30,
        ps2exec_run_callback_sync,
        "PS2ExecMan_RunCallbackSync",
    ),
    (0x0031_abc0, ps2_exec_man_v07_0031abc0, "PS2ExecMan_v07"),
    (
        0x0031_abc8,
        ps2_exec_man_get_frame_rate_0031abc8,
        "PS2ExecMan_GetFrameRate",
    ),
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
                c.cycles += 2;
                c.g[A0][0] = Addr(p(c, S0, 0x4034)).lw(c);
                if call_ptr(c, table::WAIT_SEMA, 0x0031_ac30).is_err() {
                    return;
                }
                c.pc = 0x0031_ac30;
            }
            // After the wait returns: also the resume address of a native thread
            // (see `ssx3_ps2::kernel::register_native_entry`).
            0x0031_ac30 => {
                c.cycles += 3;
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
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0031_aca4);
                    c.g[A0][0] = lw_r(c, S0, 0x4044);
                    call_ptr(c, 0x0042_3de0, 0x0031_aca4)?;
                    pc = 0x0031_aca4;
                }
                // After the job wait: also the resume address of a native thread.
                0x0031_aca4 => {
                    c.cycles += 3;
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
///
/// Dispatcher-shaped: the main thread unwinds at the frame wait (return address
/// 0x31aa8c) and re-enters here (see `ssx3_ps2::kernel::register_native_entry`);
/// the other call return addresses are accepted too.
pub fn ps2_exec_man_wait_frame(c: &mut Cpu) {
    const WAIT_RA: u32 = 0x0031_aa8c;
    const DRAIN_LOOP: u32 = 0x0031_aa90;
    const POST_WAIT: u32 = 0x0031_aaa0;
    loop {
        match c.pc {
            0x0031_aa58 => {
                c.cycles += 14;
                let frame = Frame::open(c, 0x20);
                c.g[V0][0] = 1;
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                let this = r32(c, S0);
                Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 1);
                if virtual_call(c, EXEC_PRE_WAIT_SLOT, 0x0031_aa84).is_err() {
                    return;
                }
                c.pc = 0x0031_aa84;
            }
            // After the pre-wait hook.
            0x0031_aa84 => {
                let this = r32(c, S0);
                c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
                if call_ptr(c, WAIT_SEMA, WAIT_RA).is_err() {
                    return;
                }
                c.pc = WAIT_RA;
            }
            // After the frame wait (also the main thread's resume address).
            WAIT_RA => {
                let this = r32(c, S0);
                Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 0);
                c.pc = DRAIN_LOOP;
            }
            DRAIN_LOOP => {
                c.cycles += 5;
                let this = r32(c, S0);
                c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
                if call_ptr(c, POLL_SEMA, 0x0031_aa98).is_err() {
                    return;
                }
                c.pc = 0x0031_aa98;
            }
            // After a `PollSema`: stop draining once it no longer returns the id.
            0x0031_aa98 => {
                let this = r32(c, S0);
                c.g[V1][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
                if c.g[V0][0] != c.g[V1][0] {
                    c.pc = POST_WAIT;
                } else {
                    if event_due(c) {
                        deliver_event_here(c, DRAIN_LOOP);
                    }
                    c.pc = DRAIN_LOOP;
                }
            }
            POST_WAIT => {
                c.cycles += 9;
                // (`v0` holds the vtable here, `v1` the method.)
                let this = r32(c, S0);
                c.g[V0][0] = Addr(this.wrapping_add(EXEC_VT)).lw(c);
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT)).lh(c);
                c.g[V1][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT + 4)).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
                if call_ptr(c, method, 0x0031_aab8).is_err() {
                    return;
                }
                c.pc = 0x0031_aab8;
            }
            0x0031_aab8 => {
                let frame = Frame { size: 0x20 };
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0031_aa58, 0x0031_aac8);
                return;
            }
        }
    }
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

/// `void ssx_MountSlusOvfBig(void)` (0x00256188): mounts the
/// `data/config/slusovf.big` archive (`EAFile_MountBig(name, 0, 100,
/// &D_004A2ED4)`).
pub fn ssx_mount_slus_ovf_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3d0));
        c.g[A3][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x21c));
        c.g[A2][0] = 0x64;
        call_ptr(c, 0x003d_ed50, 0x0025_61a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_GetModulesDir(void* self, char* out)` (0x00319660): writes the
/// IOP modules directory (`sprintf(out, "%sDATA/MODULES...", "cdrom0:...")`).
pub fn ssx_get_modules_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        c.g[A2][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2628));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2638));
        call_ptr(c, 0x0041_7828, 0x0031_9680)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_BuildModulePath(void* self, char* out, const char* name)`
/// (0x00319690): a name that already has a device (contains `:`) is copied
/// as is, otherwise it is `modules dir + name + suffix`
/// (`sprintf(out, fmt, dir, name, suffix)`).
pub fn ssx_build_module_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x140);
        frame.save(c, 16, 0x130);
        frame.save(c, 17, 0x120);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 18, 0x110);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[A1][0] = 0x3a;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6404, 0x0031_96bc)?;
        let has_device = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if has_device {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x0031_96cc)?;
            frame.restore(c, 16, 0x130);
        } else {
            c.cycles += 12;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0031_9660,
                f: ee::s10::f_00319660,
            }
            .call(c, 0x0031_96e0)?;
            c.g[A1][0] = 0x4a_0000;
            c.g[T0][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3e98));
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3ea0));
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0041_7828, 0x0031_9700)?;
            frame.restore(c, 16, 0x130);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x120);
        frame.restore(c, 18, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `int ssx_LoadIopModule(void* self, const char* module, int retry, const char* args, int arg_len)`
/// (0x00319718): builds the module path and calls `sceSifLoadModule(path,
/// args, arg_len)`. A negative result is retried when `retry` is set, or
/// while it is -204 (`0xffffff34`); any other result is returned.
pub fn ssx_load_iop_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x150);
        frame.save(c, 18, 0x120);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 16, 0x140);
        frame.save(c, 17, 0x130);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 19, 0x110);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[T0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[S3][0] = 0xffff_ffff_ffff_ff34;
        Callee {
            addr: 0x0031_9690,
            f: ee::s10::f_00319690,
        }
        .call(c, 0x0031_974c)?;
        c.g[A0][0] = c.g[SP][0];
        'retry: loop {
            c.cycles += 1;
            c.g[A1][0] = c.g[S1][0];
            'again: loop {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0042_ba68, 0x0031_975c)?;
                if (c.g[V0][0] as i64) >= 0 {
                    frame.restore(c, 16, 0x140);
                    break 'retry;
                }
                c.cycles += 2;
                let retry = c.g[S2][0] != 0;
                c.g[A0][0] = c.g[SP][0];
                if retry {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9750);
                    }
                    continue 'retry;
                }
                c.cycles += 2;
                let busy = c.g[V0][0] == c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9754);
                    }
                    continue 'again;
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x140);
                break 'retry;
            }
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x130);
        frame.restore(c, 18, 0x120);
        frame.restore(c, 19, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_LoadMsifrpc(void* self)` (0x003197d0): loads the `MSIFRPC.IRX`
/// IOP module (`ssx_LoadIopModule(D_004A3E94, "MSIFRPC.IRX", 1, 0, 0)`).
pub fn ssx_load_msifrpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda4));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2618));
        frame.save_ra(c, 0);
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        Callee {
            addr: 0x0031_9718,
            f: ee::s10::f_00319718,
        }
        .call(c, 0x0031_97f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_InstallPowerOffHandler(void)` (0x00319930): when the power-off
/// module is present (`sceSifSearchModuleByName` >= 0), creates a semaphore
/// (initial count 0, max 1), starts a thread (stack 0x2000 bytes, priority 1)
/// and registers the power-off callback with `sceCdPOffCallback`; the
/// semaphore id is kept in `gp + 0x2a64`, the thread id in `gp + 0x2a80`.
pub fn ssx_install_power_off_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x70);
        c.g[A0][0] = 0x4a_0000;
        frame.save(c, 16, 0x60);
        frame.save_ra(c, 0x50);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3eb0));
        call_ptr(c, 0x0042_b6d0, 0x0031_9948)?;
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = 1;
        if !missing {
            c.cycles += 28;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x30));
            c.sw(frame.at(c, 0x38), 0);
            c.sw(frame.at(c, 0x34), c.g[S0][0]);
            c.sw(frame.at(c, 0x44), 0);
            call_ptr(c, 0x0042_3da0, 0x0031_9964)?;
            c.g[A1][0] = 0x32_0000;
            c.g[A2][0] = 0x54_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6760));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x74f0));
            c.g[V1][0] = 0x2000;
            c.g[A3][0] = 0x4a_0000;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30f0));
            c.g[A0][0] = c.g[SP][0];
            c.sw(frame.at(c, 0x4), c.g[A1][0]);
            c.sw(frame.at(c, 0xc), c.g[V1][0]);
            c.sw(frame.at(c, 0x10), c.g[A3][0]);
            c.sw(frame.at(c, 0x8), c.g[A2][0]);
            c.sw(frame.at(c, 0x14), c.g[S0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2a64), c.g[V0][0]);
            call_ptr(c, 0x0042_3ba0, 0x0031_99a0)?;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a64));
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2a80), c.g[V0][0]);
            call_ptr(c, 0x0042_3bc0, 0x0031_99b0)?;
            c.g[A0][0] = 0x32_0000;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a64));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x66f0));
            call_ptr(c, 0x0040_0d58, 0x0031_99c0)?;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}

/// `void PS2ExecMan_Init(PS2ExecMan* self)` (0x0031a6d8): saves the VU0 context
/// into the manager (`self + 0x14080`), records the main thread id and
/// priority, creates the semaphores (`+0xc`, `+0x18`, `+0x4034`, `+0x4044`), starts
/// the execution thread (`+0x4040`: entry 0x31ac60, a 0x10000-byte stack at
/// `self + 0x4040`) and a second thread (`+0x4030`: entry 0x31ac08, a
/// 0x4000-byte stack at `self + 0x30`), then installs INTC handler 3
/// (`+0x4290`). Converted from
/// the generated body (every register, cycle and call kept).
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_exec_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 145;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[A0][0] = 0x10000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A0][0] = c.g[A0][0] | 0x4080;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = 0x10000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S5][0] = 0x400;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.g[S6][0] = 0x10;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
        c.g[S7][0] = 0x4a0000;
        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x30f0));
        c.g[RA][0] = sx(0x0031a734);
        Callee {
            addr: 0x0031a308,
            f: ee::s10::VU0_SaveContext,
        }
        .call(c, 0x0031a734)?;
        c.g[RA][0] = sx(0x0031a73c);
        call_ptr(c, 0x00423c90, 0x0031a73c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x30;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[RA][0] = sx(0x0031a750);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00416210, 0x0031a750)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0031a75c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00423ca0, 0x0031a75c)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[RA][0] = sx(0x0031a778);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416210, 0x0031a778)?;
        c.g[RA][0] = sx(0x0031a780);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a780)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0031a798);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a798)?;
        c.g[RA][0] = sx(0x0031a7a0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a7a0)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0031a7bc);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a7bc)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[S5][0]);
        c.g[RA][0] = sx(0x0031a7c8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a7c8)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4048), 0u64);
        c.g[A2][0] = 0x30;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0031a7e4);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4044), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a7e4)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4040));
        c.g[V0][0] = 0x320000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53a0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[S4][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
        c.g[RA][0] = sx(0x0031a824);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), 0u64);
        call_ptr(c, 0x00423ba0, 0x0031a824)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4040), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0031a838);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xddc), c.g[V0][0]);
        call_ptr(c, 0x00423bc0, 0x0031a838)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4038), 0u64);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[RA][0] = sx(0x0031a84c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416210, 0x0031a84c)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[S5][0]);
        c.g[RA][0] = sx(0x0031a858);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a858)?;
        c.g[A2][0] = 0x30;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0031a86c);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4034), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a86c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = 0x320000;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0x4000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53f8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
        c.g[RA][0] = sx(0x0031a8b0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), 0u64);
        call_ptr(c, 0x00423ba0, 0x0031a8b0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0031a8c0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4030), c.g[V0][0]);
        call_ptr(c, 0x00423bc0, 0x0031a8c0)?;
        c.g[RA][0] = sx(0x0031a8c8);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x00424880, 0x0031a8c8)?;
        c.g[A1][0] = 0x320000;
        c.g[A3][0] = 0x320000;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x5430));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5b70));
        c.g[A0][0] = 0x3;
        c.g[RA][0] = sx(0x0031a8e4);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00423a90, 0x0031a8e4)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4290), c.g[V0][0]);
        c.g[RA][0] = sx(0x0031a8f0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x004248e8, 0x0031a8f0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2ExecMan_v03(PS2ExecMan *man)` (0x31a920): install the default
/// exception/signal handlers and semaphores on `man`.
pub fn ps2_exec_man_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_4880, 0x0031_a93c)?;
        c.g[S0][0] = 0x10000;
        c.g[A0][0] = 0x3;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4290));
        call_ptr(c, 0x0042_3aa0, 0x0031_a950)?;
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_48e8, 0x0031_a958)?;
        c.g[V1][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4044));
        call_ptr(c, 0x0042_3dc0, 0x0031_a968)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4034));
        call_ptr(c, 0x0042_3dc0, 0x0031_a970)?;
        call_ptr(c, 0x0042_3cc0, 0x0031_a978)?;
        call_ptr(c, 0x0042_3cc0, 0x0031_a980)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4030));
        call_ptr(c, 0x0042_3bf0, 0x0031_a988)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4030));
        call_ptr(c, 0x0042_3bb0, 0x0031_a990)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4040));
        call_ptr(c, 0x0042_3bf0, 0x0031_a998)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4040));
        call_ptr(c, 0x0042_3bb0, 0x0031_a9a0)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4034));
        call_ptr(c, 0x0042_3db0, 0x0031_a9a8)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4044));
        call_ptr(c, 0x0042_3db0, 0x0031_a9b0)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0042_3db0, 0x0031_a9b8)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        call_ptr(c, 0x0042_3db0, 0x0031_a9c0)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_ForwardSlot06(PS2ExecMan *man)` (0x320a20): tail-call the
/// slot-6 handler through the object's slot table (-1 when `man` has no
/// table).
pub fn ps2_exec_man_forward_slot06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        if c.g[A1][0] != 0u64 {
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.cycles += 4;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0032_0a50)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_SetCallback(PS2ExecMan *man, ...)` (0x317478): store the
/// callback arguments at `man`, then tail-call the slot handler through the
/// object's slot table.
pub fn ps2_exec_man_set_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, t, 0x0031_749c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_ForwardSlot05(PS2ExecMan *man)` (0x3209c0): tail-call the
/// slot-5 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot05(c: &mut Cpu) {
    forward_slot(c, 0x28, 0x2c, 0x0032_09e0);
}

/// `int PS2ExecMan_ForwardSlot04(PS2ExecMan *man)` (0x3209f0): tail-call the
/// slot-4 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot04(c: &mut Cpu) {
    forward_slot(c, 0x20, 0x24, 0x0032_0a10);
}

/// `int PS2ExecMan_ForwardSlot11(PS2ExecMan *man)` (0x320a60): tail-call the
/// slot-11 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot11(c: &mut Cpu) {
    forward_slot(c, 0x58, 0x5c, 0x0032_0a80);
}

/// Shared body of the `ForwardSlot` wrappers: load the handler and the
/// signed offset from the object's slot table and tail-call it.
pub(crate) fn forward_slot(c: &mut Cpu, half: u32, word: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(half));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(word));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_ForwardSlot12(PS2ExecMan *man)` (0x320a90): tail-call the
/// slot-12 handler through the object's slot table (the target lives at
/// `*(*(man + 8) + 8) + 0x64`; the argument is `*(man + 8)` plus the signed
/// half at `+0x60`).
pub fn ps2_exec_man_forward_slot12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0032_0ab0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_WaitSema(PS2ExecMan *man)` (0x31aaf0): wait on the
/// semaphore id at `man + 0xc`, through the kernel's `WaitSema`.
pub fn ps2_exec_man_wait_sema(c: &mut Cpu) {
    tail_wrapper(c, Some(0xc), Target::Table(0x0042_3de0), 0x0031_ab00);
}

/// `int PS2ExecMan_v06(void)` (0x320b08): return the calling thread's id
/// through the kernel's `GetThreadId`.
pub fn ps2_exec_man_v06(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0042_3c90), 0x0032_0b18);
}

/// `VU0_RestoreContext(ctx)` (0x0031a3c0): reloads VU0 from a context record
/// (the mirror of [`vu0_save_context`]). vi22 and vi18 come from +0x208 and
/// +0x20c, vf1..vf6 from +0x1f0 (through ACC) and +0x00..+0x50, then
/// `VU0_SetSignedUnitVector(ctx->+0x200)` runs, then vf7..vf31 from +0x60 on,
/// vi16 from +0x204.
pub fn vu0_restore_context(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[T0][0] = c.g[S0][0];
        let ctx = c.g[S0][0] as u32;
        let slot = |vf: usize| ctx.wrapping_add(0x10 * (vf as u32 - 1));
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x208));
        c.ctc2(9, 22);
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x20c));
        c.ctc2(9, 18);
        c.lqc2(1, ctx.wrapping_add(0x1f0));
        c.vu0().adda(0b1111, 1, Src::Bc(0, X));
        for vf in 1..=6 {
            c.lqc2(vf, slot(vf));
        }
        c.g[A0][0] = c.lw(ctx.wrapping_add(0x200));
        Callee {
            addr: 0x0031_a268,
            f: ee::s10::f_0031a268,
        }
        .call(c, 0x0031_a40c)?;
        c.g[T0][0] = c.g[S0][0];
        for vf in 7..=12 {
            c.lqc2(vf, slot(vf));
        }
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x204));
        for vf in 13..=14 {
            c.lqc2(vf, slot(vf));
        }
        c.ctc2(9, 16);
        for vf in 15..=31 {
            c.lqc2(vf, slot(vf));
        }
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VU0_SaveContext(ctx)` (0x0031a308): stores VU0 into a context record: the
/// control words vi17 (+0x200), vi16 (+0x204), vi22 (+0x208) and vi18 (+0x20c),
/// and vf1..vf31 at +0x00..+0x1e0 (vf1 again at +0x1f0, after the final madd).
/// Leaves `t0 = a0`, and `t1` holds the last control word read, as the
/// generated body does.
pub fn vu0_save_context(c: &mut Cpu) {
    c.cycles += 45;
    let ctx = c.g[A0][0] as u32;
    c.g[T0][0] = c.g[A0][0];
    c.cfc2(9, 17);
    c.sqc2(1, ctx);
    c.sqc2(2, ctx.wrapping_add(0x10));
    c.sw(ctx.wrapping_add(0x200), c.g[T1][0]);
    c.sqc2(3, ctx.wrapping_add(0x20));
    c.sqc2(4, ctx.wrapping_add(0x30));
    c.cfc2(9, 16);
    c.sqc2(5, ctx.wrapping_add(0x40));
    c.sqc2(6, ctx.wrapping_add(0x50));
    c.sw(ctx.wrapping_add(0x204), c.g[T1][0]);
    c.vu0().waitq();
    c.cfc2(9, 22);
    c.sw(ctx.wrapping_add(0x208), c.g[T1][0]);
    c.cfc2(9, 18);
    c.sw(ctx.wrapping_add(0x20c), c.g[T1][0]);
    c.sqc2(7, ctx.wrapping_add(0x60));
    c.sqc2(8, ctx.wrapping_add(0x70));
    c.vu0().madd(0b1111, 1, 0, Src::Bc(0, X));
    for vf in 9..=31 {
        c.sqc2(vf, ctx.wrapping_add(0x10 * (vf as u32 - 1)));
    }
    c.sqc2(1, ctx.wrapping_add(0x1f0));
    ret(c);
}

/// `PS2ExecMan_v07` (0x0031_abc0): straight-line leaf. Mechanical port of the generated body.
pub fn ps2_exec_man_v07_0031abc0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PS2ExecMan_GetFrameRate` (0x0031_abc8): straight-line leaf. Mechanical port of the generated body.
pub fn ps2_exec_man_get_frame_rate_0031abc8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3c;
    ret(c);
}

/// `ssx_LoadNetGuiElf()` (0x256088): `0x31b008("cdrom0:\NETGUI\NTGUI.ELF", 1,
/// &D_004A2EC0)`, the NetGui ELF load.
pub fn ssx_load_net_gui_elf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x230));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x378));
        call_ptr(c, 0x0031_b008, 0x0025_60a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ssx_IopInit (0x319610)

/// `void ssx_IopInit(void* self)` (0x319610): starts the IOP side of the
/// system: the SIF RPC and IOP heap, the reboot of the IOP (`ssx_IopReboot`
/// with `self`), then the CD and file system setup. Frame 0x20; s0 = self.
pub fn ssx_iop_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0042_6408,
            f: ee::s15::f_00426408,
        }
        .call(c, 0x0031_9628)?;
        Callee {
            addr: 0x0042_ab18,
            f: ee::s15::f_0042ab18,
        }
        .call(c, 0x0031_9630)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0031_9820,
            f: ee::s10::f_00319820,
        }
        .call(c, 0x0031_9638)?;
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0040_1718,
            f: ee::s15::f_00401718,
        }
        .call(c, 0x0031_9640)?;
        c.g[A0][0] = 2;
        Callee {
            addr: 0x0040_2450,
            f: ee::s15::f_00402450,
        }
        .call(c, 0x0031_9648)?;
        Callee {
            addr: 0x0042_7d28,
            f: ee::s15::f_00427d28,
        }
        .call(c, 0x0031_9650)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_UnloadIopModule (0x319790)

/// `void ssx_UnloadIopModule(void* self, ...)` (0x319790): stops the IOP
/// module (`sceSifStopModule`, with a2 = 0 and a3 = the frame's scratch
/// word), then unloads it (`sceSifUnloadModule` with `self`). Frame 0x30;
/// s0 = a1.
pub fn ssx_unload_iop_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        Callee {
            addr: 0x0042_b438,
            f: ee::s15::f_0042b438,
        }
        .call(c, 0x0031_97b4)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0042_b640,
            f: ee::s15::f_0042b640,
        }
        .call(c, 0x0031_97bc)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_ExitMsifRpc (0x319800)

/// `void ssx_ExitMsifRpc(void* self)` (0x319800): `sceSifMExitRpc`, with a0
/// passed through. Frame 0x10.
pub fn ssx_exit_msif_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0040_b1f8,
            f: ee::s15::f_0040b1f8,
        }
        .call(c, 0x0031_9810)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_IopReboot (0x319820)

/// `void ssx_IopReboot(void* self)` (0x319820): resets the CD and the SIF
/// RPC, builds the IOP module path (`ssx_BuildModulePath` into the frame's
/// buffer, module name at `0x48d9f8`), then reboots the IOP. The reboot is
/// retried while it answers 0; after a non-zero answer the IOP is synced
/// (retried while 0), and the RPC is re-initialised. Frame 0x120; s0 = self.
/// The loops check events at their back edges.
pub fn ssx_iop_reboot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0042_6408,
            f: ee::s15::f_00426408,
        }
        .call(c, 0x0031_9838)?;
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0040_1718,
            f: ee::s15::f_00401718,
        }
        .call(c, 0x0031_9840)?;
        c.g[A0][0] = 2;
        Callee {
            addr: 0x0040_2450,
            f: ee::s15::f_00402450,
        }
        .call(c, 0x0031_9848)?;
        c.g[A2][0] = 0x0049_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2608));
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0031_9690,
            f: ee::s10::f_00319690,
        }
        .call(c, 0x0031_985c)?;
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            Callee {
                addr: 0x0042_bf68,
                f: ee::s15::f_0042bf68,
            }
            .call(c, 0x0031_9868)?;
            if c.g[V0][0] == 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_9860);
                }
                continue;
            }
            loop {
                c.cycles += 4;
                Callee {
                    addr: 0x0042_bf30,
                    f: ee::s15::f_0042bf30,
                }
                .call(c, 0x0031_9878)?;
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9870);
                    }
                    continue;
                }
                c.cycles += 8;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x0042_6408,
                    f: ee::s15::f_00426408,
                }
                .call(c, 0x0031_9888)?;
                Callee {
                    addr: 0x0042_ab18,
                    f: ee::s15::f_0042ab18,
                }
                .call(c, 0x0031_9890)?;
                frame.restore(c, S0, 0x110);
                frame.restore_ra(c, 0x100);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// 0x003198a0 ssx_PowerOffThread
// ---------------------------------------------------------------------------

/// `void ssx_PowerOffThread(int sema)` (0x3198a0): the power-off thread.
/// Waits on `sema`, then polls `sceDevctl` (disc tray) until it answers
/// non-negative, clears the status word at `sp` and calls `sceCdPowerOff`
/// until that word stays zero, then starts over. The generated tail that
/// signals the power-off semaphore is unreachable (its condition is always
/// true), so it is not ported. Never returns.
pub fn ssx_power_off_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = 0x4a0000;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        'disc: loop {
            // WaitSema(sema)
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0042_3de0, 0x0031_98c0)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3ea8));
            'devctl: loop {
                // sceDevctl(...)
                c.cycles += 8;
                c.g[A1][0] = 0x4402;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                call_ptr(c, 0x0042_a538, 0x0031_98e0)?;
                let busy = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3ea8));
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_98c8);
                    }
                    continue 'devctl;
                }
                c.cycles += 2;
                let status = sp(c);
                c.sw(status, 0);
                'poweroff: loop {
                    // sceCdPowerOff(sp)
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0040_2398, 0x0031_98f8)?;
                    let old_zero = c.g[V0][0] == 0;
                    let status = sp(c);
                    c.g[V0][0] = c.lw(status);
                    if old_zero {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_98f0);
                        }
                        continue 'poweroff;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_98f0);
                        }
                        continue 'poweroff;
                    }
                    c.cycles += 2;
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_98b8);
                    }
                    continue 'disc;
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// 0x00319910 ssx_PowerOffSignalSema
// ---------------------------------------------------------------------------

/// `void ssx_PowerOffSignalSema(int sema)` (0x319910): forwards to
/// `SignalSema` (0x423dd0) with the same argument and returns its result.
pub fn ssx_power_off_signal_sema(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0042_3dd0), 0x0031_9920)
}

// ---------------------------------------------------------------------------
// 0x0031ab30 PS2ExecMan_RunCallbackSync
// ---------------------------------------------------------------------------

/// `void PS2ExecMan_RunCallbackSync(PS2ExecMan* m)` (0x31ab30). Sets the
/// "callback pending" flag, waits for the current callback slot to
/// reach the callback's id (`0x423c90`/`0x423d10` poll the callback
/// semaphore), then either invokes the registered callback directly (the
/// slot is the one we are waiting for) or calls the table callback at
/// `*(m + 0x10000 ...)` and waits on the callback semaphore.
pub fn ps2exec_run_callback_sync(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        let pending = (c.g[S0][0] as u32).wrapping_add(0x4038);
        let one = c.g[V0][0];
        c.sw(pending, one);
        call_ptr(c, 0x0042_3c90, 0x0031_ab54)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10000));
        let entry_addr = (c.g[V0][0] as u32).wrapping_add(0x4040);
        let entry = c.lw(entry_addr);
        c.g[A0][0] = entry;
        if c.g[S1][0] != c.g[A0][0] {
            c.cycles += 3;
            call_ptr(c, 0x0042_3d10, 0x0031_ab74)?;
        }
        let word = (c.g[S0][0] as u32).wrapping_add(0x10);
        c.g[V0][0] = c.lw(word);
        // Both branches above end with V0 = *(s0 + 0x10).
        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0031_74a8, 0x0031_ab8c)?; // PS2ExecMan_InvokeCallback(m, 0)
        } else {
            let table = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = table;
            c.cycles += 7;
            let fn_off_addr = (c.g[V0][0] as u32).wrapping_add(0x28);
            c.g[A0][0] = c.lh(fn_off_addr);
            let fn_addr = (c.g[V0][0] as u32).wrapping_add(0x2c);
            c.g[V1][0] = c.lw(fn_addr);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0031_aba4)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_aaf0, 0x0031_abac)?; // PS2ExecMan_WaitSema(m)
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC689_MAGIC` (otherwise the body they replaced runs).
    pub(crate) const OC689_CTRL: u32 = DATA + 0x000e_4000;

    pub(crate) const OC689_MAGIC: u32 = 0x0689_0a11;

    /// Results (sign-extended words) of the fakes, indexed by fake call n % 16.
    pub(crate) const OC689_RESULTS: u32 = DATA + 0x000e_4100;

    /// The graphics manager and its tables in the graphics tests.
    pub(crate) const OC689_GFX: u32 = DATA + 0x0001_0000;

    pub(crate) const OC689_SEEDS: u32 = 120;

    pub(crate) static OC689_PREV_MOUNT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC689_PREV_SEARCH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC689_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake of an SCE/EA callee: logs the call like any fake and
    /// answers the seeded (sign-extended) result.
    pub(crate) fn oc689_fake(
        c: &mut Cpu,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
        target: u32,
    ) {
        if c.bus.read32(OC689_CTRL) != OC689_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC689_RESULTS + 4 * (n % 16)) as i32 as i64 as u64;
        fakes::fake_call(c, target);
        c.g[V0][0] = result;
    }

    /// `EAFile_MountBig`.
    pub(crate) fn oc689_fake_mount(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_MOUNT,
            ee::s14::f_003ded50__generated,
            0x003d_ed50,
        )
    }

    /// `sceSifSearchModuleByName`.
    pub(crate) fn oc689_fake_search(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_SEARCH,
            ee::s15::f_0042b6d0__generated,
            0x0042_b6d0,
        )
    }

    /// `sceSifLoadModule`.
    pub(crate) fn oc689_fake_load(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_LOAD,
            ee::s15::f_0042ba68__generated,
            0x0042_ba68,
        )
    }

    pub(crate) fn oc689_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DED50),
            oc689_fake_mount,
            &OC689_PREV_MOUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B6D0),
            oc689_fake_search,
            &OC689_PREV_SEARCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BA68),
            oc689_fake_load,
            &OC689_PREV_LOAD,
        );
    }

    pub(crate) fn oc689_put_cstr(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The strings `sprintf` formats in the module-path functions: the
    /// modules directory (`0x48d9d8` with the device `0x48d9c8`), the module
    /// name `MSIFRPC.IRX` (`0x48d9e8`) and the path format and suffix.
    pub(crate) fn oc689_put_module_strings(c: &mut Cpu) {
        oc689_put_cstr(c, 0x0048_d9c8, b"cdrom0:");
        oc689_put_cstr(c, 0x0048_d9d8, b"%sDATA\\MODULES\\");
        oc689_put_cstr(c, 0x0048_d9e8, b"MSIFRPC.IRX");
        oc689_put_cstr(c, 0x004a_3e98, b"%s%s%s");
        oc689_put_cstr(c, 0x004a_3ea0, b";1");
    }

    /// Fill the result table of the fakes: the first `bad` calls get
    /// `first` (a failure), later ones a positive result.
    pub(crate) fn oc689_results(c: &mut Cpu, r: &mut Rng, bad: u32, first: &[i32]) {
        for n in 0..16u32 {
            let v: i32 = if n < bad {
                first[r.next_u32() as usize % first.len()]
            } else {
                1 + (r.next_u32() % 9) as i32
            };
            c.bus.write32(OC689_RESULTS + 4 * n, v as u32);
        }
    }

    /// `ssx_MountSlusOvfBig`: one `EAFile_MountBig` call with fixed arguments.
    #[test]
    fn ssx_mount_slus_ovf_big_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s06::f_00256188__generated,
            ssx_mount_slus_ovf_big,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x0689_0001, 1);
                c.pc = 0x0025_6188;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
            },
        );
    }

    /// `ssx_GetModulesDir`: the real `sprintf` formats the modules path.
    #[test]
    fn ssx_get_modules_dir_matches() {
        check(
            ee::s10::f_00319660__generated,
            ssx_get_modules_dir,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0002, 0);
                c.pc = 0x0031_9660;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x1000 + (r.next_u32() & 0xf0)) as u64;
                oc689_put_module_strings(c);
                if seed % 4 == 3 {
                    oc689_put_cstr(c, 0x0048_d9c8, b"host0:");
                }
            },
        );
    }

    /// `ssx_BuildModulePath`: a name with a device is copied (`strcpy`), a
    /// bare one gets the modules directory and the suffix.
    #[test]
    fn ssx_build_module_path_matches() {
        check(
            ee::s10::f_00319690__generated,
            ssx_build_module_path,
            90,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0003, 0);
                c.pc = 0x0031_9690;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x3000 + (r.next_u32() & 0xf0)) as u64;
                c.g[A2][0] = (DATA + 0x1000) as u64;
                oc689_put_module_strings(c);
                let name: &[u8] = match seed % 3 {
                    0 => b"mod.irx",
                    1 => b"host0:mod.irx",
                    _ => b"",
                };
                oc689_put_cstr(c, DATA + 0x1000, name);
            },
        );
    }

    /// The fixture of `ssx_LoadIopModule`: `long` makes the first eight
    /// loads fail (busy, -204, or any failure) so the loops iterate.
    pub(crate) fn oc689_load_iop_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_0004, 6);
        c.pc = 0x0031_9718;
        c.bus.write32(OC689_CTRL, OC689_MAGIC);
        oc689_put_module_strings(c);
        let retry = seed % 2;
        let first: &[i32] = if retry == 1 {
            &[-204, -1, -5]
        } else {
            &[-204, -204, -1]
        };
        oc689_results(c, &mut r, if long { 8 } else { 3 }, first);
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = retry as u64;
        c.g[A3][0] = (DATA + 0x2000) as u64;
        c.g[T0][0] = (r.next_u32() % 0x40) as u64;
        let name: &[u8] = if seed % 3 == 0 {
            b"host0:a.irx"
        } else {
            b"MSIFRPC.IRX"
        };
        oc689_put_cstr(c, DATA + 0x1000, name);
        r
    }

    /// `ssx_LoadIopModule`: success, a failure that is not retried, and the
    /// retry loops (any failure with `retry`, -204 without).
    #[test]
    fn ssx_load_iop_module_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_00319718__generated,
            ssx_load_iop_module,
            OC689_SEEDS,
            &|c, seed| {
                oc689_load_iop_setup(c, seed, false);
            },
        );
        check_events(
            ee::s10::f_00319718__generated,
            ssx_load_iop_module,
            OC689_SEEDS,
            &|c, seed| {
                let mut r = oc689_load_iop_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x0031_9718)
            },
        );
    }

    /// `ssx_LoadMsifrpc`: the fixed `MSIFRPC.IRX` load with retry.
    #[test]
    fn ssx_load_msifrpc_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_003197d0__generated,
            ssx_load_msifrpc,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0005, 6);
                c.pc = 0x0031_97d0;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
                oc689_put_module_strings(c);
                oc689_results(c, &mut r, 3, &[-204, -1, -5]);
                c.bus.write32(GAME_GP_IO, DATA + 0x100);
            },
        );
    }

    /// `gp + 0xda4` (`D_004A3E94`): the IOP loader object `ssx_LoadMsifrpc` passes on.
    pub(crate) const GAME_GP_IO: u32 = crate::difftest::GAME_GP + 0xda4;

    /// `ssx_InstallPowerOffHandler`: with and without the power-off module.
    #[test]
    fn ssx_install_power_off_handler_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_00319930__generated,
            ssx_install_power_off_handler,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0006, 8);
                c.pc = 0x0031_9930;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
                let miss = if seed % 3 == 0 {
                    -1
                } else {
                    5 + (r.next_u32() % 5) as i32
                };
                for n in 0..16u32 {
                    c.bus.write32(OC689_RESULTS + 4 * n, miss as u32);
                }
            },
        );
    }

    /// `PS2ExecMan_Init`: the manager object lives at DATA; every syscall is a fake.
    #[test]
    fn ps2_exec_man_init_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_0031a6d8__generated,
            ps2_exec_man_init,
            OC689_SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x0689_0007, 24);
                c.pc = 0x0031_a6d8;
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// The fixture of `PS2GraphicsMan_BuildDisplayModeTable`: the built-in
    /// table at `0x44b440` (8 modes) with sizes and depth fields.
    #[allow(dead_code)]
    pub(crate) fn oc689_build_table_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_000b, 16);
        c.pc = 0x0037_bb10;
        c.g[A0][0] = OC689_GFX as u64;
        oc689_put_cstr(c, 0x0049_28e8, b"%d %d %d");
        for i in 0..8u32 {
            let m = 0x0044_b440 + 0xa4 * i;
            c.bus
                .write32(m + 0x88, [640, 512, 320][(r.next_u32() % 3) as usize]);
            c.bus
                .write32(m + 0x8c, [448, 480, 240][(r.next_u32() % 3) as usize]);
            let big = r.next_u32() % 4 != 0;
            c.bus.write32(
                m + 0x90,
                if big {
                    0x3b + r.next_u32() % 8
                } else {
                    r.next_u32() % 4
                },
            );
            c.bus.write32(
                m + 0x94,
                if big {
                    0x3b + r.next_u32() % 8
                } else {
                    r.next_u32() % 3
                },
            );
            c.bus.write32(m + 0x98, r.next_u32() % 0x40);
        }
        r
    }

    use crate::difftest::check_paths;

    use crate::difftest::require_paths;

    pub(crate) use crate::game::mem::memstd::tests::norm_miss;

    /// `PS2ExecMan_v03`: syscalls run against the test machine on both
    /// sides.
    #[test]
    fn ps2_exec_man_v03_matches() {
        check(
            ee::s10::f_0031a920__generated,
            ps2_exec_man_v03,
            30,
            &|c, seed| {
                machine(c, seed, 0x0031_a920, 0);
                c.hooks.miss = norm_miss;
                const MAN: u32 = DATA + 0x100;
                const MEM: u32 = DATA + 0x10000;
                for i in 0..0x10u32 {
                    c.bus.write32(MAN + 4 * i, 0xaaaa_0000 + i);
                }
                for i in 0..0x1100u32 {
                    c.bus.write32(MEM + 4 * i, 0xbbbb_0000 + i + seed);
                }
                c.g[A0][0] = MAN as u64;
                c.pc = 0x0031_a920;
            },
        );
        require_paths(
            ee::s10::f_0031a920__generated,
            30,
            &[
                224, 232, 233, 234, 235, 236, 239, 240, 242, 243, 244, 245, 246, 248, 249, 251,
                252, 256,
            ],
            &|c, seed| {
                machine(c, seed, 0x0031_a920, 0);
                c.hooks.miss = norm_miss;
                const MAN: u32 = DATA + 0x100;
                const MEM: u32 = DATA + 0x10000;
                for i in 0..0x10u32 {
                    c.bus.write32(MAN + 4 * i, 0xaaaa_0000 + i);
                }
                for i in 0..0x1100u32 {
                    c.bus.write32(MEM + 4 * i, 0xbbbb_0000 + i + seed);
                }
                c.g[A0][0] = MAN as u64;
                c.pc = 0x0031_a920;
            },
        );
    }

    /// `PS2ExecMan_ForwardSlot06`: no table reads -1; a table runs the fake
    /// handler with a steered offset.
    #[test]
    fn ps2_exec_man_forward_slot06_matches() {
        check(
            ee::s10::f_00320a20__generated,
            ps2_exec_man_forward_slot06,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a20, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                if seed % 2 == 0 {
                    c.bus.write32(OBJ + 8, 0);
                } else {
                    c.bus.write32(OBJ + 8, TAB1);
                    c.bus.write32(TAB1 + 8, TAB2);
                    for i in 0..0x20u32 {
                        c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                    }
                    let off = match seed % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 0x4000) as u16,
                        _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                    };
                    c.bus.write16(TAB2 + 0x30, off);
                    c.bus.write32(TAB2 + 0x34, FAKE);
                }
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a20;
            },
        );
        require_paths(
            ee::s10::f_00320a20__generated,
            100,
            &[11, 16],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a20, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                if seed % 2 == 0 {
                    c.bus.write32(OBJ + 8, 0);
                } else {
                    c.bus.write32(OBJ + 8, TAB1);
                    c.bus.write32(TAB1 + 8, TAB2);
                    for i in 0..0x20u32 {
                        c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                    }
                    let off = match seed % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 0x4000) as u16,
                        _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                    };
                    c.bus.write16(TAB2 + 0x30, off);
                    c.bus.write32(TAB2 + 0x34, FAKE);
                }
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a20;
            },
        );
    }

    /// `PS2ExecMan_SetCallback`: the arguments land at the object before
    /// the fake slot handler runs with the adjusted object.
    #[test]
    fn ps2_exec_man_set_callback_matches() {
        check(
            ee::s10::f_00317478__generated,
            ps2_exec_man_set_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7478, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x100;
                const TAB: u32 = DATA + 0x200;
                c.bus.write32(OBJ + 8, TAB);
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x400) as u16,
                    _ => (0x10000 - r.next_u32() % 0x400) as u16,
                };
                c.bus.write16(TAB + 0x68, off);
                c.bus.write32(TAB + 0x6c, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = 0xc0de_0000 + seed as u64;
                c.g[A2][0] = 0xf00d_0000 + seed as u64 * 3;
                c.pc = 0x0031_7478;
            },
        );
        require_paths(ee::s10::f_00317478__generated, 300, &[16], &|c, seed| {
            let mut r = machine(c, seed, 0x0031_7478, 0);
            c.hooks.miss = norm_miss;
            const OBJ: u32 = DATA + 0x100;
            const TAB: u32 = DATA + 0x200;
            c.bus.write32(OBJ + 8, TAB);
            let off = match seed % 3 {
                0 => 0,
                1 => (r.next_u32() % 0x400) as u16,
                _ => (0x10000 - r.next_u32() % 0x400) as u16,
            };
            c.bus.write16(TAB + 0x68, off);
            c.bus.write32(TAB + 0x6c, FAKE);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = 0xc0de_0000 + seed as u64;
            c.g[A2][0] = 0xf00d_0000 + seed as u64 * 3;
            c.pc = 0x0031_7478;
        });
    }

    /// The three remaining `ForwardSlot` wrappers: the slot table yields
    /// the fake handler with a seed-steered offset. The tables are
    /// prefilled with garbage, so a wrong offset reads a visibly wrong
    /// target into the call log instead of the fake.
    #[test]
    fn ps2_exec_man_forward_slots_match() {
        for (gen, hand, addr, half, word) in [
            (
                ee::s10::f_003209c0__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot05 as fn(&mut Cpu),
                0x0032_09c0,
                0x28,
                0x2c,
            ),
            (
                ee::s10::f_003209f0__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot04 as fn(&mut Cpu),
                0x0032_09f0,
                0x20,
                0x24,
            ),
            (
                ee::s10::f_00320a60__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot11 as fn(&mut Cpu),
                0x0032_0a60,
                0x58,
                0x5c,
            ),
        ] {
            check(gen, hand, 100, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                for i in 0..0x20u32 {
                    c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                }
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + half, off);
                c.bus.write32(TAB2 + word, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = addr;
            });
            require_paths(gen, 100, &[15], &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                for i in 0..0x20u32 {
                    c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                }
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + half, off);
                c.bus.write32(TAB2 + word, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = addr;
            });
        }
    }

    /// `PS2ExecMan_ForwardSlot12`: the slot table yields the fake handler
    /// with a seed-steered signed offset, so the forwarded argument varies
    /// on both sides.
    #[test]
    fn ps2_exec_man_forward_slot12_matches() {
        check(
            ee::s10::f_00320a90__generated,
            ps2_exec_man_forward_slot12,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a90, 0);
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + 0x60, off);
                c.bus.write32(TAB2 + 0x64, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a90;
            },
        );
    }

    /// Gate for the `WaitSema` / `GetThreadId` fakes below: the syscalls run
    /// for real unless a test plants this magic word.
    pub(crate) const CTRL: u32 = DATA + 0x1f00;

    pub(crate) const MAGIC: u32 = 0x5711_5e4d;

    pub(crate) static PREV_WAIT_SEMA: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_GET_THREAD_ID: AtomicUsize = AtomicUsize::new(0);

    /// The fake kernel call: log the call like `fakes::fake_call` (so the
    /// strict-args check sees the wrapper's call arguments), then pin the
    /// cost to the real syscall's 4 cycles — the fake's hash-based cost
    /// would otherwise make the wrapper's cycle total seed-varying.
    pub(crate) fn fake_kernel_call(c: &mut Cpu, addr: u32) {
        let before = c.cycles;
        fakes::fake_call(c, addr);
        c.cycles = before + 4;
    }

    pub(crate) fn fake_wait_sema(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_WAIT_SEMA, ee::s15::f_00423de0__generated);
        }
        fake_kernel_call(c, 0x0042_3de0);
    }

    pub(crate) fn fake_get_thread_id(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_GET_THREAD_ID, ee::s15::f_00423c90__generated);
        }
        fake_kernel_call(c, 0x0042_3c90);
    }

    pub(crate) fn install_kernel_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            fake_wait_sema,
            &PREV_WAIT_SEMA,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423C90),
            fake_get_thread_id,
            &PREV_GET_THREAD_ID,
        );
    }

    /// `PS2ExecMan_WaitSema`: the sema id comes from `man + 0xc` (varied
    /// per seed); the fake `WaitSema` returns a hash `V0`, so the wrapper's
    /// return path runs with a distinctive value on both sides.
    #[test]
    fn ps2_exec_man_wait_sema_matches() {
        check_paths(
            ee::s10::f_0031aaf0__generated,
            ps2_exec_man_wait_sema,
            300,
            &[11],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_aaf0, 0);
                install_kernel_fakes();
                c.bus.write32(CTRL, MAGIC);
                let man = DATA + 0x100 + (r.next_u32() & 0xf0);
                c.bus.write32(man + 0xc, 0x1000 + seed);
                c.g[A0][0] = man as u64;
                c.pc = 0x0031_aaf0;
            },
        );
    }

    /// `PS2ExecMan_v06`: the fake `GetThreadId` returns a hash `V0`.
    #[test]
    fn ps2_exec_man_v06_matches() {
        check_paths(
            ee::s10::f_00320b08__generated,
            ps2_exec_man_v06,
            300,
            &[11],
            &|c, seed| {
                machine(c, seed, 0x0032_0b08, 0);
                install_kernel_fakes();
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x0032_0b08;
            },
        );
    }

    use crate::difftest::fill;

    pub(crate) use crate::game::bx::ps2main::tests::oc1579_install;
    pub(crate) use crate::game::bx::ps2main::tests::oc1579_machine;
    pub(crate) use crate::game::bx::ps2main::tests::BUSY;

    pub(crate) use crate::game::bx::ps2main::tests::POWER_RET;
    pub(crate) use crate::game::bx::ps2main::tests::POWER_STATUS;
    pub(crate) use crate::game::dirtysock::dnasauth::tests::install_all;

    pub(crate) use crate::game::dirtysock::dnasauth::tests::open_gates;

    pub(crate) use crate::game::dirtysock::dnasauth::tests::G_LOAD_ELF;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1486_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC1486_MAGIC: u32 = 0x1486_f00d;

    macro_rules! oc1486_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1486_CTRL) != OC1486_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1486_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1486_fakes! {
        oc1486_fake_service_queue, 0x003d_dfa8, ee::s14::OVERRIDE_003DDFA8, ee::s14::f_003ddfa8__generated;
    }

    #[test]
    fn vu0_save_context_matches() {
        check(
            ee::s10::f_0031a308__generated,
            vu0_save_context,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1486_3a08, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = sx(crate::difftest::RA);
                fill(c, &mut r, DATA + 0x1000, 0x20);
            },
        );
    }

    #[test]
    fn vu0_restore_context_matches() {
        check(
            ee::s10::f_0031a3c0__generated,
            vu0_restore_context,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1486_3a3c, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = sx(crate::difftest::RA);
                fill(c, &mut r, DATA + 0x1000, 0x90);
                // The unit-vector helper's argument: a small mask.
                c.bus.write32(DATA + 0x1200, r.next_u32() & 0xffff);
            },
        );
    }

    /// `PS2ExecMan_v07` (0x0031_abc0) against the generated body.
    #[test]
    fn ps2_exec_man_v07_0031abc0_matches() {
        check(
            ee::s10::f_0031abc0__generated,
            ps2_exec_man_v07_0031abc0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x31abc0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2ExecMan_GetFrameRate` (0x0031_abc8) against the generated body.
    #[test]
    fn ps2_exec_man_get_frame_rate_0031abc8_matches() {
        check(
            ee::s10::f_0031abc8__generated,
            ps2_exec_man_get_frame_rate_0031abc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x31abc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn ssx_load_net_gui_elf_matches() {
        install_all();
        check(
            ee::s06::f_00256088__generated,
            ssx_load_net_gui_elf,
            100,
            &|c, seed| {
                fakes::machine(c, seed, 0x2560_88, 0);
                open_gates(c, &[G_LOAD_ELF]);
            },
        );
    }

    /// The gated IOP fakes. Each answers through `fakes::fake` (logged, the
    /// call index may unwind), and falls back to the body it replaced when the
    /// gate word in the machine's RAM is not set.
    pub(crate) const IOP_GATE: u32 = DATA + 0x2_f000;

    pub(crate) const IOP_MAGIC: u32 = 0x1578_0001;

    /// Calls to `sceSifRebootIop` / `sceSifSyncIop` still answering 0.
    pub(crate) const IOP_REBOOT_LEFT: u32 = DATA + 0x2_f008;

    pub(crate) const IOP_SYNC_LEFT: u32 = DATA + 0x2_f00c;

    pub(crate) fn oc1578_no_prev(_c: &mut Cpu) {
        unreachable!("oc1578 fake called before it was hooked");
    }

    macro_rules! oc1578_fake {
        ($name:ident, $target:literal) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(IOP_GATE) != IOP_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), oc1578_no_prev);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc1578_fake!(oc1578_fake_426408, 0x0042_6408);
    // sceSifInitRpc
    oc1578_fake!(oc1578_fake_42ab18, 0x0042_ab18);
    // sceSifInitIopHeap
    oc1578_fake!(oc1578_fake_401718, 0x0040_1718);
    // sceCdInit
    oc1578_fake!(oc1578_fake_402450, 0x0040_2450);
    // sceCdMmode
    oc1578_fake!(oc1578_fake_427d28, 0x0042_7d28);
    // sceFsReset
    oc1578_fake!(oc1578_fake_40b1f8, 0x0040_b1f8);
    // sceSifMExitRpc
    oc1578_fake!(oc1578_fake_42b438, 0x0042_b438);
    // sceSifStopModule
    oc1578_fake!(oc1578_fake_42b640, 0x0042_b640);
    // sceSifUnloadModule
    oc1578_fake!(oc1578_fake_319690, 0x0031_9690);
    // ssx_BuildModulePath

    /// Returns 0 (not ready) while the countdown at `cell` runs, then 1.
    pub(crate) fn oc1578_countdown(c: &mut Cpu, cell: u32) -> u64 {
        let left = c.bus.read32(cell);
        if left > 0 {
            c.bus.write32(cell, left - 1);
            0
        } else {
            1
        }
    }

    pub(crate) fn oc1578_fake_reboot(c: &mut Cpu) {
        if c.bus.read32(IOP_GATE) != IOP_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1578_fake_reboot), oc1578_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0042_bf68, ra); // sceSifRebootIop
        c.g[V0][0] = oc1578_countdown(c, IOP_REBOOT_LEFT);
    }

    pub(crate) fn oc1578_fake_sync(c: &mut Cpu) {
        if c.bus.read32(IOP_GATE) != IOP_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1578_fake_sync), oc1578_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0042_bf30, ra); // sceSifSyncIop
        c.g[V0][0] = oc1578_countdown(c, IOP_SYNC_LEFT);
    }

    /// Hooks every gated fake into its callee's override slot (once per process).
    pub(crate) fn oc1578_hook() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00426408),
            oc1578_fake_426408,
            fakes::prev_cell(oc1578_fake_426408),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042AB18),
            oc1578_fake_42ab18,
            fakes::prev_cell(oc1578_fake_42ab18),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00401718),
            oc1578_fake_401718,
            fakes::prev_cell(oc1578_fake_401718),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402450),
            oc1578_fake_402450,
            fakes::prev_cell(oc1578_fake_402450),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00427D28),
            oc1578_fake_427d28,
            fakes::prev_cell(oc1578_fake_427d28),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040B1F8),
            oc1578_fake_40b1f8,
            fakes::prev_cell(oc1578_fake_40b1f8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B438),
            oc1578_fake_42b438,
            fakes::prev_cell(oc1578_fake_42b438),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B640),
            oc1578_fake_42b640,
            fakes::prev_cell(oc1578_fake_42b640),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00319690),
            oc1578_fake_319690,
            fakes::prev_cell(oc1578_fake_319690),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BF68),
            oc1578_fake_reboot,
            fakes::prev_cell(oc1578_fake_reboot),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BF30),
            oc1578_fake_sync,
            fakes::prev_cell(oc1578_fake_sync),
        );
    }

    /// A machine with the gate set and the RPC countdowns: `reboot` calls
    /// answer 0 first, then the sync calls.
    pub(crate) fn oc1578_iop_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        reboot: u32,
        sync: u32,
    ) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(IOP_GATE, IOP_MAGIC);
        c.bus.write32(IOP_REBOOT_LEFT, reboot);
        c.bus.write32(IOP_SYNC_LEFT, sync);
        r
    }

    #[test]
    fn ssx_iop_init_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319610__generated,
            ssx_iop_init,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319610, 6, seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9610;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn ssx_unload_iop_module_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319790__generated,
            ssx_unload_iop_module,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319790, 6, 0, 0);
                c.pc = 0x0031_9790;
                c.g[A1][0] = (DATA + 0x200 + 0x10 * (seed % 4)) as u64;
            },
        );
    }

    #[test]
    fn ssx_exit_msif_rpc_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319800__generated,
            ssx_exit_msif_rpc,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319800, 6, 0, 0);
                c.pc = 0x0031_9800;
                c.g[A0][0] = (DATA + 0x300 + 0x10 * (seed % 4)) as u64;
            },
        );
    }

    /// The reboot loops: the reboot answers 0 for `seed % 3` calls, the sync
    /// for `(seed / 3) % 3`, with unwinding fakes at random call indices.
    #[test]
    fn ssx_iop_reboot_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319820__generated,
            ssx_iop_reboot,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319820, 6, seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9820;
                c.g[A0][0] = (DATA + 0x400) as u64;
            },
        );
    }

    /// Events delivered at the reboot loops' back edges (every seed retries the
    /// reboot at least once, so the event checks run).
    #[test]
    fn ssx_iop_reboot_events() {
        oc1578_hook();
        check_events(
            ee::s10::f_00319820__generated,
            ssx_iop_reboot,
            120,
            &|c, seed| {
                let mut r = oc1578_iop_machine(c, seed, 0x319820, 0, 1 + seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9820;
                c.g[A0][0] = (DATA + 0x400) as u64;
                schedule(c, &mut r, seed, 0x0031_9820)
            },
        );
    }

    #[test]
    fn ssx_power_off_thread_matches() {
        oc1579_install();
        check(
            ee::s10::f_003198a0__generated,
            ssx_power_off_thread,
            200,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0001, 8);
                c.g[A0][0] = sx(DATA + 0x100);
                // Unwind within the first calls: the poll loops never end otherwise.
                c.bus.write32(fakes::LOG_UNWIND_AT, seed % 8);
                c.bus.write32(BUSY, (seed / 8 % 3 == 0) as u32);
                c.bus.write32(POWER_STATUS, seed / 3 % 2);
                c.bus.write32(POWER_RET, seed / 5 % 2);
            },
        );
    }

    #[test]
    fn ssx_power_off_signal_sema_matches() {
        oc1579_install();
        check(
            ee::s10::f_00319910__generated,
            ssx_power_off_signal_sema,
            100,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0002, 2);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn ps2exec_run_callback_sync_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031ab30__generated,
            ps2exec_run_callback_sync,
            200,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0003, 0);
                let m = DATA + 0x1000;
                c.g[A0][0] = sx(m);
                let v = 0x0080_1000;
                // The first call (0x423c90) returns v.
                c.bus.write32(fakes::LOG_RESULTS, v);
                c.bus
                    .write32(m + 0x1_4040, if seed % 2 == 0 { v } else { v + 0x1000 });
                c.bus.write32(m + 0x10, if seed % 3 == 0 { v } else { 0 });
                // The table entry the tail path calls through (not in the table: a miss).
                c.bus.write32(m + 8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2028, 0x40);
                c.bus.write32(DATA + 0x202c, fakes::FAKE + 0x80);
            },
        );
    }
}
