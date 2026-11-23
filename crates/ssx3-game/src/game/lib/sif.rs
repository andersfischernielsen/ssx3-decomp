//! `src/lib/sif.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_4140, sce_sif_dma_stat, "sceSifDmaStat"),
    (0x0042_4160, sce_sif_set_dma, "sceSifSetDma"),
    (0x0042_4190, isce_sif_set_dchain, "isceSifSetDChain"),
    (
        0x0042_6230,
        sce_sif_cmd_intr_handler,
        "sceSifCmdIntrHandler",
    ),
    (
        0x0042_ac10,
        sce_sif_alloc_sys_memory,
        "sceSifAllocSysMemory",
    ),
    (0x0042_ac90, sce_sif_free_iop_heap, "sceSifFreeIopHeap"),
    (0x0042_ad08, sce_sif_free_sys_memory, "sceSifFreeSysMemory"),
    (0x0042_ad28, sce_sif_load_iop_heap, "sceSifLoadIopHeap"),
    (0x0042_ae18, sce_sif_query_mem_size, "sceSifQueryMemSize"),
    (
        0x0042_ae88,
        sce_sif_query_max_free_mem_size,
        "sceSifQueryMaxFreeMemSize",
    ),
    (
        0x0042_aef8,
        sce_sif_query_total_free_mem_size,
        "sceSifQueryTotalFreeMemSize",
    ),
    (
        0x0042_af68,
        sce_sif_query_block_top_address,
        "sceSifQueryBlockTopAddress",
    ),
    (
        0x0042_afe8,
        sce_sif_query_block_size,
        "sceSifQueryBlockSize",
    ),
    (0x0042_b068, sce_sif_load_file_init, "sceSifLoadFileInit"),
    (
        0x0042_b168,
        libkernl_loadfile_needs_rebind,
        "libkernl_LoadfileNeedsRebind",
    ),
    (0x0042_b1f8, sce_sif_load_file_reset, "sceSifLoadFileReset"),
    (
        0x0042_b230,
        sce_sif_load_module_buffer,
        "sceSifLoadModuleBuffer",
    ),
    (0x0042_b438, sce_sif_stop_module, "sceSifStopModule"),
    (0x0042_b640, sce_sif_unload_module, "sceSifUnloadModule"),
    (
        0x0042_b6d0,
        sce_sif_search_module_by_name,
        "sceSifSearchModuleByName",
    ),
    (
        0x0042_b770,
        sce_sif_search_module_by_address,
        "sceSifSearchModuleByAddress",
    ),
    (
        0x0042_b800,
        sce_sif_load_module_buffer_2,
        "sceSifLoadModuleBuffer_",
    ),
    (
        0x0042_b820,
        sce_sif_load_start_module_buffer,
        "sceSifLoadStartModuleBuffer",
    ),
    (0x0042_ba68, sce_sif_load_module, "sceSifLoadModule"),
    (
        0x0042_ba88,
        sce_sif_load_start_module,
        "sceSifLoadStartModule",
    ),
    (0x0042_bbb0, sce_sif_load_elf_part, "sceSifLoadElfPart"),
    (0x0042_bbf8, sce_sif_iop_get_val, "sceSifIopGetVal"),
    (0x0042_bce8, sce_sif_iop_set_val, "sceSifIopSetVal"),
    (0x0042_bdc8, sce_sif_reset_iop, "sceSifResetIop"),
    (0x0042_bf08, sce_sif_is_alive_iop, "sceSifIsAliveIop"),
    (0x0042_bf30, sce_sif_sync_iop, "sceSifSyncIop"),
    (0x0042_bf68, sce_sif_reboot_iop, "sceSifRebootIop"),
    (0x0042_b840, _sce_sif_load_module, "_sceSifLoadModule"),
    (0x0042_baa8, _sce_sif_load_elf_part, "_sceSifLoadElfPart"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// Memory compare. (`0x0041_5fc8`).
    pub(crate) const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::f_00415fc8,
    };

    pub(super) use crate::game::lib::libc::callee::MEMCPY;

    /// Memory set. (`0x0041_6210`).
    pub(crate) const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::f_00416210,
    };

    /// Bounded string copy. (`0x0041_6ce0`).
    pub(crate) const STRNCPY: Callee = Callee {
        addr: 0x0041_6ce0,
        f: ee::s15::f_00416ce0,
    };

    /// Stop SIF DMA. (`0x0042_4070`).
    pub(crate) const STOP_DMA: Callee = Callee {
        addr: 0x0042_4070,
        f: ee::s15::sceSifStopDma,
    };

    /// Start SIF DMA. (`0x0042_4160`).
    pub(crate) const SET_DMA: Callee = Callee {
        addr: 0x0042_4160,
        f: ee::s15::sceSifSetDma,
    };

    /// Set the DMA chain. (`0x0042_4190`).
    pub(crate) const SET_DCHAIN: Callee = Callee {
        addr: 0x0042_4190,
        f: ee::s15::f_00424190,
    };

    /// Write an SIF register. (`0x0042_41a0`).
    pub(crate) const SET_REG: Callee = Callee {
        addr: 0x0042_41a0,
        f: ee::s15::sceSifSetReg,
    };

    /// Read an SIF register. (`0x0042_41b0`).
    pub(crate) const GET_REG: Callee = Callee {
        addr: 0x0042_41b0,
        f: ee::s15::sceSifGetReg,
    };

    /// Reset the TTY. (`0x0042_4220`).
    pub(crate) const RESET_TTY: Callee = Callee {
        addr: 0x0042_4220,
        f: ee::s15::libkernl_ResetTtyInit,
    };

    /// Formatted print. (`0x0042_5c60`).
    pub(crate) const PRINTF: Callee = Callee {
        addr: 0x0042_5c60,
        f: ee::s15::scePrintf,
    };

    /// Write back the data cache. (`0x0042_6358`).
    pub(crate) const WRITEBACK: Callee = Callee {
        addr: 0x0042_6358,
        f: ee::s15::sceSifWriteBackDCache,
    };

    /// Initialise SIF RPC. (`0x0042_6408`).
    pub(crate) const INIT_RPC: Callee = Callee {
        addr: 0x0042_6408,
        f: ee::s15::sceSifInitRpc,
    };

    /// Shut down SIF RPC. (`0x0042_65a8`).
    pub(crate) const EXIT_RPC: Callee = Callee {
        addr: 0x0042_65a8,
        f: ee::s15::sceSifExitRpc,
    };

    /// Bind an SIF RPC. (`0x0042_6b48`).
    pub(crate) const BIND_RPC: Callee = Callee {
        addr: 0x0042_6b48,
        f: ee::s15::sceSifBindRpc,
    };

    /// Issue an SIF RPC call. (`0x0042_6d18`).
    pub(crate) const CALL_RPC: Callee = Callee {
        addr: 0x0042_6d18,
        f: ee::s15::sceSifCallRpc,
    };

    /// Free an IOP heap block (port entry). (`0x0042_ac90`).
    pub(crate) const FREE_IOP_HEAP: Callee = Callee {
        addr: 0x0042_ac90,
        f: ee::s15::f_0042ac90,
    };

    /// Initialise the loader (bind on demand). (`0x0042_b068`).
    pub(crate) const LOAD_FILE_INIT: Callee = Callee {
        addr: 0x0042_b068,
        f: ee::s15::sceSifLoadFileInit,
    };

    /// Rebind check for the loader. (`0x0042_b168`).
    pub(crate) const NEEDS_REBIND: Callee = Callee {
        addr: 0x0042_b168,
        f: ee::s15::f_0042b168,
    };

    /// Shared module-buffer load worker. (`0x0042_b230`).
    pub(crate) const LOAD_BUFFER_WORKER: Callee = Callee {
        addr: 0x0042_b230,
        f: ee::s15::f_0042b230,
    };

    /// Shared module load/start worker. (`0x0042_b840`).
    pub(crate) const LOAD_MODULE_WORKER: Callee = Callee {
        addr: 0x0042_b840,
        f: ee::s15::f_0042b840,
    };

    /// Shared ELF-part load worker. (`0x0042_baa8`).
    pub(crate) const LOAD_ELF_WORKER: Callee = Callee {
        addr: 0x0042_baa8,
        f: ee::s15::f_0042baa8,
    };

    /// Reset the IOP (port entry). (`0x0042_bdc8`).
    pub(crate) const RESET_IOP: Callee = Callee {
        addr: 0x0042_bdc8,
        f: ee::s15::f_0042bdc8,
    };

    /// Enable interrupts. (`0x0042_c0c0`).
    pub(crate) const EINTR: Callee = Callee {
        addr: 0x0042_c0c0,
        f: ee::s15::f_0042c0c0,
    };
}

/// The `syscall` trap: `pc = ret`, invoke the hook at `at`; `false` when the
/// hook unwound (a thread switch mid-syscall).
#[inline]
pub(crate) fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The final due-event check of a syscall wrapper (result ignored, as in the
/// compiled code).
#[inline]
pub(crate) fn event_if_due(c: &mut Cpu) {
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap at `at`, return to
/// `ret`, then a due-event check and `jr ra`.
#[inline]
pub(crate) fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    event_if_due(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Kernel syscall wrappers.

/// `int sceSifSetDma(SifDmaTransfer* t, int len)` (0x424160): syscall 0x77.
pub fn sce_sif_set_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0x77, 0x0042_4164, 0x0042_4168);
}

/// `int isceSifSetDChain(void)` (0x424190): syscall -0x78 (0xffffff88).
pub fn isce_sif_set_dchain(c: &mut Cpu) {
    syscall_wrapper(c, (-120i32) as u32, 0x0042_4194, 0x0042_4198);
}

/// `int sceSifDmaStat(unsigned int id)` (0x424140): syscall 0x76.
pub fn sce_sif_dma_stat(c: &mut Cpu) {
    syscall_wrapper(c, 0x76, 0x0042_4144, 0x0042_4148);
}

pub(crate) mod sif_cmd {

    pub const DATA: u32 = 0x0052_bcd8;

    /// Pointer to the receive buffer (first byte: packet size).
    pub const RECV: u32 = 0x0;

    /// System command handlers (`{func, data}` pairs) and their count.
    pub const SYS_TABLE: u32 = 0xc;

    pub const SYS_COUNT: u32 = 0x10;

    /// User command handlers and their count.
    pub const USR_TABLE: u32 = 0x14;

    pub const USR_COUNT: u32 = 0x18;

    /// Packet header: the command id (bit 31: system command).
    pub const PKT_CID: u32 = 0x8;
}

/// `int sceSifCmdIntrHandler(void)` (0x426230, libkernel): the SIF DMA
/// interrupt handler. Re-enables interrupts, and if a packet arrived (its
/// size byte non-zero) copies it to the stack (rounded up to quadwords),
/// clears the size byte, re-arms the receive chain (`isceSifSetDChain`),
/// and dispatches the command id to the system (bit 31 set) or user handler
/// table (`func(packet, data)`) when it is in range and set; then `ei`.
/// Returns 0.
pub fn sce_sif_cmd_intr_handler(c: &mut Cpu) {
    use sif_cmd::*;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        let at = frame.at(c, 0x70);
        c.sd(at, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        callee::EINTR.call(c, 0x0042_6244)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A3][0] = Addr(DATA + RECV).lw(c);
        c.g[S0][0] = sx(DATA);
        let recv = r32(c, A3);
        c.g[V0][0] = Addr(recv).lbu(c);
        c.g[A1][0] = c.g[V0][0] & 0xff;
        let size = c.g[A1][0];
        c.g[V0][0] = 0;
        if size != 0 {
            // Copy the packet (size rounded up to 16) to the stack.
            c.cycles += 10;
            c.g[V0][0] = sx((size as u32).wrapping_add(0xf));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx((size as u32).wrapping_add(0x1e));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 4) as u32);
            Addr(recv).sb(c, 0);
            let quads = c.g[A1][0] as i64;
            c.g[A0][0] = c.g[A1][0];
            if quads > 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[SP][0];
                loop {
                    c.cycles += 7;
                    c.lq(V0, r32(c, A2));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sq(V0, r32(c, V1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_6290);
                    }
                }
            }

            c.cycles += 5;
            callee::SET_DCHAIN.call(c, 0x0042_62b4)?;
            let pkt = frame.at(c, 0);
            c.g[V1][0] = Addr(pkt + PKT_CID).lw(c);
            let system = (c.g[V1][0] as i64) < 0;
            // (table, count, return address) of the handler table used.
            let handler = if system {
                c.cycles += 8;
                c.g[V0][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V1][0] = 0x7fff_ffff;
                c.g[A0][0] = Addr(DATA + SYS_COUNT).lw(c);
                c.g[A1][0] = c.g[V0][0] & c.g[V1][0];
                (SYS_TABLE, 0x0042_6300u32, 5u64)
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V0][0] = Addr(DATA + USR_COUNT).lw(c);
                (USR_TABLE, 0x0042_633c, 3)
            };
            let (table, ret_to, call_cycles) = handler;
            // The count is in a0 (system) or v0 (user); the compare result
            // replaces it.
            let count_reg = if system { A0 } else { V0 };
            let in_range = (c.g[A1][0] as i64) < (c.g[count_reg][0] as i64);
            c.g[count_reg][0] = in_range as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if in_range {
                c.cycles += 5;
                c.g[V1][0] = Addr(DATA + table).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let entry = r32(c, V0);
                c.g[A2][0] = Addr(entry).lw(c);
                if c.g[A2][0] != 0 {
                    c.cycles += call_cycles;
                    c.g[A1][0] = Addr(entry + 4).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    let func = r32(c, A2);
                    call_ptr(c, func, ret_to)?;
                }
            }
            c.cycles += 3;
            c.ei();
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x80);
        let at = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(at);
        frame.close(c);
        Ok(())
    })
}

/// sceSifRebootIop (0x42bf68): reboots the IOP: measures the reset image
/// path; if it fits, runs `InitRpc` then `ExitRpc`, copies the path onto
/// the stack and runs `sceSifResetIop`; otherwise warns and returns 0.
pub fn sce_sif_reboot_iop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x4a0000;
        c.sd(frame.at(c, 0x60), c.g[S1][0]);
        c.sd(frame.at(c, 0x50), c.g[S0][0]);
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lb(c.g[S0][0] as u32);
        let empty = c.g[V1][0] == 0u64;
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x52a0));
        if !empty {
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb));
            loop {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                let more = c.g[V0][0] != 0u64;
                if !more {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0042_bf90);
                }
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 0x51u64) as u64;
        let short = c.g[V0][0] != 0u64;
        c.g[A0][0] = 0x4a0000;
        if short {
            c.cycles += 8;
            c.g[A0][0] = 0u64;
            callee::INIT_RPC.call(c, 0x0042_bfe4)?;
            callee::EXIT_RPC.call(c, 0x0042_bfec)?;
            c.g[V0][0] = c.lb(c.g[S1][0] as u32);
            c.g[V1][0] = c.g[SP][0];
            let done = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
            if done {
                c.cycles += 2;
                c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                c.g[A0][0] = c.g[A1][0];
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                loop {
                    c.cycles += 7;
                    c.sb(c.g[V1][0] as u32, c.g[A0][0]);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                    let more = c.g[V0][0] != 0u64;
                    if !more {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A1][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_c000);
                    }
                }
            }
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.sb(c.g[V1][0] as u32, 0u64);
            } else {
                c.cycles += 1;
                loop {
                    c.cycles += 7;
                    c.sb(c.g[V1][0] as u32, c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    let more = c.g[V0][0] != 0u64;
                    if !more {
                        c.cycles += 1;
                        c.sb(c.g[V1][0] as u32, 0u64);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_c038);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0u64;
            callee::RESET_IOP.call(c, 0x0042_c064)?;
        } else {
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5290));
            callee::PRINTF.call(c, 0x0042_bfd4)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        c.g[S1][0] = c.ld(frame.at(c, 0x60));
        c.g[S0][0] = c.ld(frame.at(c, 0x50));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifSyncIop (0x42bf30): polls the IOP reset-done register once,
/// resetting the TTY so a later `printf` does not hang.
pub fn sce_sif_sync_iop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[A0][0] = 0x4;
        callee::GET_REG.call(c, 0x0042_bf40)?;
        c.g[V1][0] = 0x40000;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        let settled = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0u64;
        if !settled {
            c.cycles += 3;
            callee::RESET_TTY.call(c, 0x0042_bf58)?;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifIsAliveIop (0x42bf08): reports whether the IOP is alive (its status
/// register's alive bit).
pub fn sce_sif_is_alive_iop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[A0][0] = 0x4;
        callee::GET_REG.call(c, 0x0042_bf18)?;
        c.g[V1][0] = 0x10000;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifResetIop (0x42bdc8): resets the IOP: stops SIF DMA, snapshots the
/// reset image path into the reset packet, flushes the cache and kicks the
/// reset DMA off with the right status bits.
pub fn sce_sif_reset_iop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::STOP_DMA.call(c, 0x0042_bde4)?;
        c.g[A0][0] = 0xffffffff80000000;
        callee::GET_REG.call(c, 0x0042_bdf0)?;
        c.g[T2][0] = 0x530000;
        c.g[T3][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(0xd40));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.lb(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0u64;
        c.g[T1][0] = 0u64;
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = c.g[S1][0];
            c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
            loop {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(0xd40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.sb((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                let more = c.g[V1][0] != 0u64;
                if !more {
                    break;
                }
                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_be18);
                }
            }
        }
        c.cycles += 31;
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0xd40));
        c.g[V1][0] = 0xffffffff80000000;
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), 0u64);
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A0][0] = c.g[A0][0] << 32;
        c.g[A0][0] = c.g[A0][0] | 0xff;
        c.g[V1][0] = c.g[V1][0] | 0x3;
        c.g[A1][0] = 0x68;
        c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_sub(0xd40));
        c.g[A3][0] = 0x68;
        c.sw((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.g[T0][0] = 0x44;
        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.sd((c.g[T2][0] as u32).wrapping_sub(0xd40), c.g[V0][0]);
        c.g[A0][0] = c.g[A2][0];
        c.sb((c.g[T2][0] as u32).wrapping_sub(0xd40), c.g[A1][0]);
        c.g[A1][0] = 0x68;
        c.sw(sp(c).wrapping_add(0x4), c.g[T3][0]);
        c.sw(sp(c).wrapping_add(0x8), c.g[A3][0]);
        c.sw(sp(c).wrapping_add(0xc), c.g[T0][0]);
        c.sw(sp(c), c.g[A2][0]);
        callee::WRITEBACK.call(c, 0x0042_be94)?;
        c.g[A0][0] = 0x4;
        c.g[A1][0] = 0x40000;
        callee::SET_REG.call(c, 0x0042_bea0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x1;
        callee::SET_DMA.call(c, 0x0042_beac)?;
        let kicked = c.g[V0][0] == 0u64;
        c.g[A0][0] = 0x4;
        if !kicked {
            c.cycles += 14;
            c.g[A1][0] = 0x10000;
            callee::SET_REG.call(c, 0x0042_bebc)?;
            c.g[A0][0] = 0x4;
            c.g[A1][0] = 0x20000;
            callee::SET_REG.call(c, 0x0042_bec8)?;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[A0][0] | 0x2;
            callee::SET_REG.call(c, 0x0042_bed8)?;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = 0u64;
            callee::SET_REG.call(c, 0x0042_bee4)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifIopSetVal (0x42bce8): sets an IOP module value: initialises the
/// loader with `LoadFileInit` (binding on demand), fills the call packet
/// with the byte/halfword/word value, then calls RPC (function 2). Returns
/// 0 on success, `0xffffffffffff0000` when the loader is down,
/// `0xfffffffffffeffff` when the call fails, and `0xfffffffffffefffe` for
/// a width other than 0/1/2.
pub fn sce_sif_iop_set_val(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S0][0] = c.g[A2][0];
        callee::LOAD_FILE_INIT.call(c, 0x0042_bd0c)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.g[A3][0] = 0x530000;
        if !ok {
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffffff0000;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf80));
            c.sw((c.g[A3][0] as u32).wrapping_sub(0xf80), c.g[S2][0]);
            let wide = c.g[S0][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
            if !wide {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let half = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = 0x2;
                if !half {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                    c.sh((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        c.cycles += 1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        c.g[V0][0] = c.g[V0][0] | 0xfffe;
                        c.cycles += 6;
                        c.g[RA][0] = c.ld(frame.at(c, 0x40));
                        c.g[S2][0] = c.ld(frame.at(c, 0x30));
                        c.g[S1][0] = c.ld(frame.at(c, 0x20));
                        c.g[S0][0] = c.ld(frame.at(c, 0x10));
                        let ret = c.g[RA][0] as u32;
                        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
                        c.pc = ret;
                        return Ok(());
                    }
                }
            }
            c.cycles += 17;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf80));
            c.g[A0][0] = 0x530000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
            c.sw(sp(c), 0u64);
            c.g[A1][0] = 0x2;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = 0x20;
            c.g[T1][0] = c.g[A3][0];
            c.g[T2][0] = 0x10;
            c.g[T3][0] = 0u64;
            callee::CALL_RPC.call(c, 0x0042_bd94)?;
            c.g[A0][0] = 0xfffffffffffe0000;
            c.g[V1][0] = 0xffffffffffffffff;
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.g[V0][0] = c.g[A0][0];
            if c.g[V1][0] != 0u64 {
                c.g[V0][0] = 0u64;
            }
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifIopGetVal (0x42bbf8): reads an IOP module value: validates the
/// loader, calls RPC, then stores the byte/halfword/word result.
pub fn sce_sif_iop_get_val(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'b_42bcc8: {
            'b_42bc30: {
                c.cycles += 12;

                c.sd(frame.at(c, 0x40), c.g[S3][0]);
                c.sd(frame.at(c, 0x30), c.g[S2][0]);
                c.g[S3][0] = c.g[A0][0];
                c.sd(frame.at(c, 0x10), c.g[S0][0]);
                c.g[S2][0] = c.g[A1][0];
                c.sd(frame.at(c, 0x50), c.g[RA][0]);
                c.g[S0][0] = c.g[A2][0];
                c.sd(frame.at(c, 0x20), c.g[S1][0]);
                callee::LOAD_FILE_INIT.call(c, 0x0042_bc20)?;
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
                if t {
                    break 'b_42bc30;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffff0000;
                    if t {
                        break 'b_42bcc8;
                    } else {
                        break 'b_42bc30;
                    }
                }
            }
            'b_42bcc4: {
                'b_42bcc0: {
                    'b_42bcb4: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = 0x530000;
                        if t {
                            break 'b_42bcb4;
                        } else {
                            'b_42bc84: {
                                c.cycles += 16;
                                c.g[A0][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0xf80), c.g[S3][0]);
                                c.g[A3][0] = c.g[V0][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
                                c.g[A1][0] = 0x3;
                                c.sw(sp(c), 0u64);
                                c.g[A2][0] = 0u64;
                                c.g[T0][0] = 0x20;
                                c.g[T1][0] = c.g[A3][0];
                                c.g[T2][0] = 0x20;
                                c.g[T3][0] = 0u64;
                                callee::CALL_RPC.call(c, 0x0042_bc70)?;
                                let t = (c.g[V0][0] as i64) >= 0;
                                if t {
                                    break 'b_42bc84;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xfffffffffffe0000;
                                    let t = true;
                                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                                    if t {
                                        break 'b_42bcc8;
                                    } else {
                                        break 'b_42bc84;
                                    }
                                }
                            }
                            'b_42bc98: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_42bc98;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                    let t = true;
                                    c.sb(c.g[S2][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_42bcc4;
                                    } else {
                                        break 'b_42bc98;
                                    }
                                }
                            }
                            'b_42bcac: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_42bcac;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                    let t = true;
                                    c.sh(c.g[S2][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_42bcc4;
                                    } else {
                                        break 'b_42bcac;
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[S0][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                break 'b_42bcc0;
                            } else {
                                break 'b_42bcb4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0xfffe;
                    if t {
                        break 'b_42bcc8;
                    } else {
                        break 'b_42bcc0;
                    }
                }
                c.cycles += 1;
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                break 'b_42bcc4;
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_42bcc8;
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadElfPart (0x42bbb0): loads an ELF part: forwards to the shared
/// `_sceSifLoadElfPart` worker with RPC function number 1.
pub fn sce_sif_load_elf_part(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[A3][0] = 0x1;
        callee::LOAD_ELF_WORKER.call(c, 0x0042_bbc0)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadStartModule (0x42ba88): starts a module by path: forwards to the
/// shared `_sceSifLoadModule` worker with RPC function number 0.
pub fn sce_sif_load_start_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[T0][0] = 0u64;
        callee::LOAD_MODULE_WORKER.call(c, 0x0042_ba98)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadModule (0x42ba68): loads a module by path: forwards to the shared
/// `_sceSifLoadModule` worker with RPC function number 0, passing the caller's stack.
pub fn sce_sif_load_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = 0u64;
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[SP][0];
        callee::LOAD_MODULE_WORKER.call(c, 0x0042_ba7c)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadStartModuleBuffer (0x42b820): starts a buffered module: forwards
/// to the shared `sceSifLoadModuleBuffer` worker.
pub fn sce_sif_load_start_module_buffer(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::LOAD_BUFFER_WORKER),
        0x0042_b830,
    );
}

/// Shared SIF heap-query shape: check the loader word at `0x455224`, stash
/// the caller's `a0` as the block id when `input` is set, issue CALL_RPC
/// with the function id, and return the reply word (0 when that word is
/// negative). Covers the five `Query*`
/// ports: function id, return address, and whether they take a block id.
fn query_heap(c: &mut Cpu, func: u32, ret: u32, input: bool) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_done: {
            'b_check: {
                if input {
                    c.cycles += 7;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5224));
                    c.g[A1][0] = c.g[A0][0];
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5224));
                }
                c.sd(frame.at(c, 0x20), c.g[RA][0]);
                let t = (if input { c.g[V0][0] } else { c.g[V1][0] } as i64) >= 0;
                c.sd(frame.at(c, 0x10), c.g[S0][0]);
                if t {
                    break 'b_check;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_done;
                }
            }
            if input {
                c.cycles += 16;
                c.g[A3][0] = 0x530000;
                c.g[A0][0] = 0x530000;
                c.sw((c.g[A3][0] as u32).wrapping_sub(0x10c0), c.g[A1][0]);
            } else {
                c.cycles += 14;
                c.g[A0][0] = 0x530000;
            }
            c.g[S0][0] = 0x530000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1140));
            if input {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10c0));
            }
            c.sw(sp(c), 0u64);
            c.g[A1][0] = func as u64;
            c.g[A2][0] = 0u64;
            if !input {
                c.g[A3][0] = 0u64;
            }
            c.g[T0][0] = if input { 0x4 } else { 0u64 };
            c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1100));
            c.g[T2][0] = 0x4;
            c.g[T3][0] = 0u64;
            callee::CALL_RPC.call(c, ret)?;
            if (c.g[V0][0] as i64) >= 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1100));
                break 'b_done;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                break 'b_done;
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadModuleBuffer_ (0x42b800): loads a buffered module: forwards to
/// the shared `sceSifLoadModuleBuffer` worker with the caller's stack.
pub fn sce_sif_load_module_buffer_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[SP][0];
        callee::LOAD_BUFFER_WORKER.call(c, 0x0042_b810)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifSearchModuleByAddress (0x42b770): finds a loaded module by address:
/// validates the loader, then calls RPC and returns the module id.
pub fn sce_sif_search_module_by_address(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'b_42b7ec: {
            c.cycles += 8;

            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            c.sd(frame.at(c, 0x30), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_b788)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42b7ec;
            } else {
                'b_42b7ac: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b798)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0x530000;
                    if t {
                        break 'b_42b7ac;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42b7ec;
                        } else {
                            break 'b_42b7ac;
                        }
                    }
                }
                c.cycles += 14;
                c.g[A0][0] = 0x530000;
                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xf80));
                c.sw((c.g[S0][0] as u32).wrapping_sub(0xf80), c.g[S1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
                c.sw(sp(c), 0u64);
                c.g[A1][0] = 0xa;
                c.g[A2][0] = 0u64;
                c.g[T0][0] = 0x4;
                c.g[T1][0] = c.g[A3][0];
                c.g[T2][0] = 0x4;
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0042_b7dc)?;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0xf80));
                    break 'b_42b7ec;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    break 'b_42b7ec;
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifSearchModuleByName (0x42b6d0): finds a loaded module by name:
/// validates the loader, copies the name in, and calls RPC.
pub fn sce_sif_search_module_by_name(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_42b760: {
            c.cycles += 7;

            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            c.sd(frame.at(c, 0x20), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            callee::LOAD_FILE_INIT.call(c, 0x0042_b6e4)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42b760;
            } else {
                'b_42b708: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b6f4)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x530000;
                        break 'b_42b708;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42b760;
                        } else {
                            break 'b_42b708;
                        }
                    }
                }
                c.cycles += 20;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf78));
                c.g[A2][0] = 0xfc;
                c.g[A0][0] = c.g[S0][0];
                callee::STRNCPY.call(c, 0x0042_b71c)?;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = 0x530000;
                c.g[A3][0] = c.g[V1][0];
                c.sb((c.g[V1][0] as u32).wrapping_add(0x103), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
                c.g[A1][0] = 0x9;
                c.sw(sp(c), 0u64);
                c.g[A2][0] = 0u64;
                c.g[T0][0] = 0x200;
                c.g[T1][0] = c.g[A3][0];
                c.g[T2][0] = 0x4;
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0042_b750)?;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
                    break 'b_42b760;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    break 'b_42b760;
                }
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifUnloadModule (0x42b640): unloads a module by id: validates the
/// loader, then calls RPC and returns the result.
pub fn sce_sif_unload_module(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'b_42b6bc: {
            c.cycles += 8;

            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            c.sd(frame.at(c, 0x30), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_b658)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42b6bc;
            } else {
                'b_42b67c: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b668)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0x530000;
                    if t {
                        break 'b_42b67c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42b6bc;
                        } else {
                            break 'b_42b67c;
                        }
                    }
                }
                c.cycles += 14;
                c.g[A0][0] = 0x530000;
                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xf80));
                c.sw((c.g[S0][0] as u32).wrapping_sub(0xf80), c.g[S1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
                c.sw(sp(c), 0u64);
                c.g[A1][0] = 0x8;
                c.g[A2][0] = 0u64;
                c.g[T0][0] = 0x4;
                c.g[T1][0] = c.g[A3][0];
                c.g[T2][0] = 0x4;
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0042_b6ac)?;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0xf80));
                    break 'b_42b6bc;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    break 'b_42b6bc;
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifStopModule (0x42b438): stops a module: validates the loader, copies
/// any arguments in, calls RPC, and stores the result.
pub fn sce_sif_stop_module(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'b_42b618: {
            c.cycles += 16;

            c.sd(frame.at(c, 0x70), c.g[S6][0]);
            c.sd(frame.at(c, 0x40), c.g[S3][0]);
            c.g[S6][0] = c.g[A3][0];
            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            c.g[S3][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            c.g[S1][0] = c.g[A1][0];
            c.sd(frame.at(c, 0x80), c.g[RA][0]);
            c.g[S0][0] = c.g[A2][0];
            c.sd(frame.at(c, 0x60), c.g[S5][0]);
            c.sd(frame.at(c, 0x50), c.g[S4][0]);
            c.sd(frame.at(c, 0x30), c.g[S2][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_b470)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42b618;
            } else {
                'b_42b494: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b480)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = 0x530000;
                    if t {
                        break 'b_42b494;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42b618;
                        } else {
                            break 'b_42b494;
                        }
                    }
                }
                'b_42b5d0: {
                    'b_42b5cc: {
                        'b_42b5c8: {
                            c.cycles += 4;
                            c.g[S5][0] = c.g[S4][0];
                            c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xf80));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_sub(0xf80), c.g[S3][0]);
                            if t {
                                break 'b_42b5c8;
                            } else {
                                'b_42b5b4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 253i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        break 'b_42b5b4;
                                    } else {
                                        'b_42b564: {
                                            'b_42b528: {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x104));
                                                c.g[V0][0] = c.g[S0][0] | c.g[A0][0];
                                                c.g[V0][0] = c.g[V0][0] & 0x7;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A3][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_42b528;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                                    c.g[A1][0] = 0x530000;
                                                    'l_42b4cc: loop {
                                                        c.cycles += 21;
                                                        c.ldl(
                                                            3,
                                                            (c.g[A3][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(3, c.g[A3][0] as u32);
                                                        c.ldl(
                                                            6,
                                                            (c.g[A3][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            6,
                                                            (c.g[A3][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            8,
                                                            (c.g[A3][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            8,
                                                            (c.g[A3][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            9,
                                                            (c.g[A3][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            9,
                                                            (c.g[A3][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            3,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(3, c.g[A0][0] as u32);
                                                        c.sdl(
                                                            6,
                                                            (c.g[A0][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            6,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A3][0] != c.g[V0][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0042_b4cc);
                                                            }
                                                            continue 'l_42b4cc;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            if t {
                                                                break 'b_42b564;
                                                            } else {
                                                                break 'b_42b528;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                            c.g[A1][0] = 0x530000;
                                            'l_42b530: loop {
                                                c.cycles += 13;
                                                c.g[T2][0] = c.ld(c.g[A3][0] as u32);
                                                c.g[V1][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A2][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                                                c.g[T0][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A0][0] as u32, c.g[T2][0]);
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    c.g[A2][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    c.g[T0][0],
                                                );
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A3][0] != c.g[V0][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0042_b530);
                                                    }
                                                    continue 'l_42b530;
                                                } else {
                                                    break 'b_42b564;
                                                }
                                            }
                                        }
                                        c.cycles += 20;
                                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x7));
                                        c.ldr(9, c.g[A3][0] as u32);
                                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0xf));
                                        c.ldr(10, (c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.ldl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                        c.ldr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                        c.lwl(8, (c.g[A3][0] as u32).wrapping_add(0x1b));
                                        c.lwr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                        c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(9, c.g[A0][0] as u32);
                                        c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.swl(8, (c.g[A0][0] as u32).wrapping_add(0x1b));
                                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xf80));
                                        c.g[V0][0] = 0xfc;
                                        c.swr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        let t = true;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        if t {
                                            break 'b_42b5d0;
                                        } else {
                                            break 'b_42b5b4;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x104));
                                c.g[A2][0] = c.g[S1][0];
                                callee::MEMCPY.call(c, 0x0042_b5c0)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                                if t {
                                    break 'b_42b5cc;
                                } else {
                                    break 'b_42b5c8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                        break 'b_42b5cc;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = 0x530000;
                    break 'b_42b5d0;
                }
                'b_42b610: {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xf80));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd80));
                    c.sw(sp(c), 0u64);
                    c.g[A1][0] = 0x7;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x200;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x8;
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0042_b5fc)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_42b610;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        if t {
                            break 'b_42b618;
                        } else {
                            break 'b_42b610;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0xf80));
                c.sw(c.g[S6][0] as u32, c.g[V1][0]);
                break 'b_42b618;
            }
        }
        c.cycles += 10;
        c.g[RA][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadModuleBuffer (0x42b230): loads a buffered module: validates the
/// loader, copies any arguments in, calls RPC, and stores the result.
pub fn sce_sif_load_module_buffer(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'b_42b410: {
            c.cycles += 16;

            c.sd(frame.at(c, 0x70), c.g[S6][0]);
            c.sd(frame.at(c, 0x40), c.g[S3][0]);
            c.g[S6][0] = c.g[A3][0];
            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            c.g[S3][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            c.g[S1][0] = c.g[A1][0];
            c.sd(frame.at(c, 0x80), c.g[RA][0]);
            c.g[S0][0] = c.g[A2][0];
            c.sd(frame.at(c, 0x60), c.g[S5][0]);
            c.sd(frame.at(c, 0x50), c.g[S4][0]);
            c.sd(frame.at(c, 0x30), c.g[S2][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_b268)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42b410;
            } else {
                'b_42b28c: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b278)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = 0x530000;
                    if t {
                        break 'b_42b28c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42b410;
                        } else {
                            break 'b_42b28c;
                        }
                    }
                }
                'b_42b3c8: {
                    'b_42b3c4: {
                        'b_42b3c0: {
                            c.cycles += 4;
                            c.g[S5][0] = c.g[S4][0];
                            c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xf80));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_sub(0xf80), c.g[S3][0]);
                            if t {
                                break 'b_42b3c0;
                            } else {
                                'b_42b3ac: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 253i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        break 'b_42b3ac;
                                    } else {
                                        'b_42b35c: {
                                            'b_42b320: {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x104));
                                                c.g[V0][0] = c.g[S0][0] | c.g[A0][0];
                                                c.g[V0][0] = c.g[V0][0] & 0x7;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A3][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_42b320;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                                    c.g[A1][0] = 0x530000;
                                                    'l_42b2c4: loop {
                                                        c.cycles += 21;
                                                        c.ldl(
                                                            3,
                                                            (c.g[A3][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(3, c.g[A3][0] as u32);
                                                        c.ldl(
                                                            6,
                                                            (c.g[A3][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            6,
                                                            (c.g[A3][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            8,
                                                            (c.g[A3][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            8,
                                                            (c.g[A3][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            9,
                                                            (c.g[A3][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            9,
                                                            (c.g[A3][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            3,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(3, c.g[A0][0] as u32);
                                                        c.sdl(
                                                            6,
                                                            (c.g[A0][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            6,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A3][0] != c.g[V0][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0042_b2c4);
                                                            }
                                                            continue 'l_42b2c4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            if t {
                                                                break 'b_42b35c;
                                                            } else {
                                                                break 'b_42b320;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                            c.g[A1][0] = 0x530000;
                                            'l_42b328: loop {
                                                c.cycles += 13;
                                                c.g[T2][0] = c.ld(c.g[A3][0] as u32);
                                                c.g[V1][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A2][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                                                c.g[T0][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A0][0] as u32, c.g[T2][0]);
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    c.g[A2][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    c.g[T0][0],
                                                );
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A3][0] != c.g[V0][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0042_b328);
                                                    }
                                                    continue 'l_42b328;
                                                } else {
                                                    break 'b_42b35c;
                                                }
                                            }
                                        }
                                        c.cycles += 20;
                                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x7));
                                        c.ldr(9, c.g[A3][0] as u32);
                                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0xf));
                                        c.ldr(10, (c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.ldl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                        c.ldr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                        c.lwl(8, (c.g[A3][0] as u32).wrapping_add(0x1b));
                                        c.lwr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                        c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(9, c.g[A0][0] as u32);
                                        c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.swl(8, (c.g[A0][0] as u32).wrapping_add(0x1b));
                                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xf80));
                                        c.g[V0][0] = 0xfc;
                                        c.swr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        let t = true;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        if t {
                                            break 'b_42b3c8;
                                        } else {
                                            break 'b_42b3ac;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x104));
                                c.g[A2][0] = c.g[S1][0];
                                callee::MEMCPY.call(c, 0x0042_b3b8)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                                if t {
                                    break 'b_42b3c4;
                                } else {
                                    break 'b_42b3c0;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                        break 'b_42b3c4;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = 0x530000;
                    break 'b_42b3c8;
                }
                'b_42b408: {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xf80));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd80));
                    c.sw(sp(c), 0u64);
                    c.g[A1][0] = 0x6;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x200;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x8;
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0042_b3f4)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_42b408;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        if t {
                            break 'b_42b410;
                        } else {
                            break 'b_42b408;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0xf80));
                c.sw(c.g[S6][0] as u32, c.g[V1][0]);
                break 'b_42b410;
            }
        }
        c.cycles += 10;
        c.g[RA][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
        c.pc = ret;
        Ok(())
    })
}

/// libkernl_LoadfileNeedsRebind (0x42b168): reports whether the loader needs
/// to rebind: compares the in-RAM path stamp with the two known stamps.
pub fn libkernl_loadfile_needs_rebind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        'b_42b1d4: {
            c.g[V0][0] = 0x450000;
            c.sd(frame.at(c, 0x30), c.g[S3][0]);
            c.g[V1][0] = 0x530000;
            c.sd(frame.at(c, 0x10), c.g[S1][0]);
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5164));
            c.sd(frame.at(c, 0x20), c.g[S2][0]);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd58));
            c.sd(frame.at(c, 0x40), c.g[RA][0]);
            c.g[S2][0] = 0u64;
            c.sd(frame.at(c, 0), c.g[S0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = 0x4;
            callee::MEMCMP.call(c, 0x0042_b1a4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = 0x450000;
            if t {
                break 'b_42b1d4;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x522c));
                c.g[A2][0] = 0x4;
                callee::MEMCMP.call(c, 0x0042_b1bc)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x522c));
                if t {
                    break 'b_42b1d4;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = 0x4;
                    callee::MEMCMP.call(c, 0x0042_b1d0)?;
                    c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_42b1d4;
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadFileReset (0x42b1f8): resets the loader state: writes -1 to
/// `0x455228` and zeroes the load-info block, returning 0.
pub fn sce_sif_load_file_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A0][0] = 0x530000;
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd58));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5228), c.g[V1][0]);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x4;
        callee::MEMSET.call(c, 0x0042_b220)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = 0u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifLoadFileInit (0x42b068): ensures the loader is bound: returns 0
/// if already bound, otherwise binds the RPC and returns 0 on success
/// (-1 when the IOP side is unreachable).
pub fn sce_sif_load_file_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_42b150: {
            'b_42b14c: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                c.g[S2][0] = 0x450000;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5228));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                let t = (c.g[V0][0] as i64) >= 0;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                if t {
                    break 'b_42b14c;
                } else {
                    c.cycles += 2;
                    c.g[S1][0] = 0x530000;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xd80));
                    'l_42b090: loop {
                        'b_42b0b4: {
                            c.cycles += 7;
                            c.g[A1][0] = 0xffffffff80000000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[A1][0] | 0x6;
                            c.g[A2][0] = 0u64;
                            callee::BIND_RPC.call(c, 0x0042_b0a4)?;
                            if (c.g[V0][0] as i64) >= 0 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                break 'b_42b0b4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_42b150;
                                } else {
                                    break 'b_42b0b4;
                                }
                            }
                        }
                        'b_42b11c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_42b11c;
                            } else {
                                'b_42b0fc: {
                                    c.cycles += 13;
                                    c.g[S1][0] = 0x530000;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5228), 0u64);
                                    c.sw(c.g[SP][0] as u32, 0u64);
                                    c.g[A1][0] = 0xff;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0u64;
                                    c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                    c.g[T2][0] = 0x4;
                                    c.g[T3][0] = 0u64;
                                    callee::CALL_RPC.call(c, 0x0042_b0e8)?;
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.g[V1][0] = 0x530000;
                                    if t {
                                        break 'b_42b0fc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xfffffffffffe0000;
                                        let t = true;
                                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                                        if t {
                                            break 'b_42b150;
                                        } else {
                                            break 'b_42b0fc;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xf80));
                                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd58));
                                c.lwl(4, (c.g[A3][0] as u32).wrapping_add(0x3));
                                c.lwr(4, c.g[A3][0] as u32);
                                c.swl(4, (c.g[A2][0] as u32).wrapping_add(0x3));
                                c.swr(4, c.g[A2][0] as u32);
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_42b150;
                                } else {
                                    break 'b_42b11c;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x100000;
                        c.g[V1][0] = 0xffffffffffffffff;
                        'l_42b128: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V0][0] != c.g[V1][0];

                            if t {
                                if c.cycles >= c.next_event {
                                    c.pc = 0x0042b128;
                                    c.event();
                                }
                                continue 'l_42b128;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xd80));
                                if t {
                                    if c.cycles >= c.next_event {
                                        c.pc = 0x0042b090;
                                        c.event();
                                    }
                                    continue 'l_42b090;
                                } else {
                                    break 'b_42b14c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_42b150;
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifQueryBlockSize (0x42afe8): queries an IOP heap block size: reads the
/// loader state, then calls RPC with the block id and returns the size.
pub fn sce_sif_query_block_size(c: &mut Cpu) {
    query_heap(c, 0x9, 0x0042_b044, true);
}

/// sceSifQueryBlockTopAddress (0x42af68): queries an IOP heap block's top
/// address: same shape as `QueryBlockSize` with function id 8.
pub fn sce_sif_query_block_top_address(c: &mut Cpu) {
    query_heap(c, 0x8, 0x0042_afc4, true);
}

/// sceSifQueryTotalFreeMemSize (0x42aef8): queries the IOP heap's total free
/// size: same shape with function id 7 and no input word.
pub fn sce_sif_query_total_free_mem_size(c: &mut Cpu) {
    query_heap(c, 0x7, 0x0042_af48, false);
}

/// sceSifQueryMaxFreeMemSize (0x42ae88): queries the IOP heap's largest free
/// block: function id 6.
pub fn sce_sif_query_max_free_mem_size(c: &mut Cpu) {
    query_heap(c, 0x6, 0x0042_aed8, false);
}

/// sceSifQueryMemSize (0x42ae18): queries the IOP heap size: function id 5.
pub fn sce_sif_query_mem_size(c: &mut Cpu) {
    query_heap(c, 0x5, 0x0042_ae68, false);
}

/// sceSifLoadIopHeap (0x42ad28): loads the IOP heap description: copies the
/// heap name in (up to 252 bytes) and calls RPC for the heap block.
pub fn sce_sif_load_iop_heap(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_42ae08: {
            'b_42ad4c: {
                c.cycles += 7;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5224));
                c.g[A2][0] = c.g[A0][0];
                c.sd(frame.at(c, 0x20), c.g[RA][0]);
                let t = (c.g[V1][0] as i64) >= 0;
                c.sd(frame.at(c, 0x10), c.g[S0][0]);
                if t {
                    break 'b_42ad4c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_42ae08;
                }
            }
            'b_42adb4: {
                'b_42adb0: {
                    'b_42ada8: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                        c.g[A3][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1080));
                        c.sb((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let t = c.g[V0][0] == 0u64;
                        c.g[T0][0] = 0u64;
                        if t {
                            break 'b_42ada8;
                        } else {
                            c.cycles += 4;
                            c.g[T1][0] = 0x530000;
                            c.g[S0][0] = 0x530000;
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                            'l_42ad78: loop {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T0][0] as i64) < 252i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                if t {
                                    break 'b_42adb0;
                                } else {
                                    c.cycles += 7;
                                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1080));
                                    c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.sb((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                                    if c.g[A0][0] != 0u64 {
                                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0042_ad78);
                                        }
                                        continue 'l_42ad78;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0xfc;
                                        break 'b_42adb4;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[T1][0] = 0x530000;
                    c.g[S0][0] = 0x530000;
                    break 'b_42adb0;
                }
                c.cycles += 1;
                c.g[V0][0] = 0xfc;
                break 'b_42adb4;
            }
            'b_42adcc: {
                c.cycles += 2;
                if c.g[T0][0] != c.g[V0][0] {
                    c.sw((c.g[A3][0] as u32).wrapping_sub(0x1080), c.g[A1][0]);
                    break 'b_42adcc;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1080));
                    c.g[T0][0] = 0xfb;
                    c.sb((c.g[V0][0] as u32).wrapping_add(0xff), 0u64);
                    c.sw((c.g[A3][0] as u32).wrapping_sub(0x1080), c.g[A1][0]);
                    break 'b_42adcc;
                }
            }
            c.cycles += 14;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1080));
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1140));
            c.sb((c.g[V0][0] as u32).wrapping_add(0xff), 0u64);
            c.g[A3][0] = c.g[V0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5));
            c.sw(sp(c), 0u64);
            c.g[A1][0] = 0x3;
            c.g[A2][0] = 0u64;
            c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1100));
            c.g[T2][0] = 0x4;
            c.g[T3][0] = 0u64;
            callee::CALL_RPC.call(c, 0x0042_adfc)?;
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1100));
            if t {
                break 'b_42ae08;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_42ae08;
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifFreeSysMemory (0x42ad08): frees system memory: forwards to the
/// shared `sceSifFreeIopHeap` worker.
pub fn sce_sif_free_sys_memory(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::FREE_IOP_HEAP), 0x0042_ad18);
}

/// sceSifFreeIopHeap (0x42ac90): frees an IOP heap block: reads the loader
/// word at `0x455224` (returns 0 when negative), then calls RPC (function 2)
/// and returns the reply word (`-1` when the call fails).
pub fn sce_sif_free_iop_heap(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_42acf8: {
            'b_42acb4: {
                c.cycles += 7;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5224));
                c.g[A1][0] = c.g[A0][0];
                c.sd(frame.at(c, 0x20), c.g[RA][0]);
                let t = (c.g[V0][0] as i64) >= 0;
                c.sd(frame.at(c, 0x10), c.g[S0][0]);
                if t {
                    break 'b_42acb4;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_42acf8;
                }
            }
            c.cycles += 16;
            c.g[A3][0] = 0x530000;
            c.g[A0][0] = 0x530000;
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x10c0), c.g[A1][0]);
            c.g[S0][0] = 0x530000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1140));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10c0));
            c.sw(sp(c), 0u64);
            c.g[A1][0] = 0x2;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = 0x4;
            c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1100));
            c.g[T2][0] = 0x4;
            c.g[T3][0] = 0u64;
            callee::CALL_RPC.call(c, 0x0042_acec)?;
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1100));
            if t {
                break 'b_42acf8;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_42acf8;
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// sceSifAllocSysMemory (0x42ac10): allocates system memory: reads the loader
/// word at `0x455224` (returns 0 when negative), then calls RPC (function 4)
/// with the size and mode and returns the reply address.
pub fn sce_sif_alloc_sys_memory(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_42ac80: {
            c.cycles += 8;
            c.g[V0][0] = 0x450000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5224));
            c.g[A3][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x20), c.g[RA][0]);
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            let t = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_42ac80;
            } else {
                c.cycles += 19;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = 0x530000;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10c0));
                c.sw((c.g[V0][0] as u32).wrapping_sub(0x10c0), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                c.g[S0][0] = 0x530000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1140));
                c.g[A3][0] = c.g[V1][0];
                c.g[A1][0] = 0x4;
                c.sw(sp(c), 0u64);
                c.g[A2][0] = 0u64;
                c.g[T0][0] = 0xc;
                c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1100));
                c.g[T2][0] = 0x4;
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0042_ac74)?;
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1100));
                if t {
                    break 'b_42ac80;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_42ac80;
                }
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// _sceSifLoadModule (0x42b840): the shared module load/start worker:
/// validates the loader, copies the module name in, and calls RPC.
pub fn _sce_sif_load_module(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        'b_42ba40: {
            c.cycles += 16;

            c.sd(frame.at(c, 0x50), c.g[S4][0]);
            c.sd(frame.at(c, 0x40), c.g[S3][0]);
            c.g[S4][0] = c.g[A3][0];
            c.sd(frame.at(c, 0x30), c.g[S2][0]);
            c.g[S3][0] = c.g[T0][0];
            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            c.g[S2][0] = c.g[A1][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x70), c.g[RA][0]);
            c.g[S0][0] = c.g[A2][0];
            c.sd(frame.at(c, 0x60), c.g[S5][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_b878)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42ba40;
            } else {
                'b_42b89c: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_b888)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x530000;
                        break 'b_42b89c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42ba40;
                        } else {
                            break 'b_42b89c;
                        }
                    }
                }
                'b_42b9f8: {
                    'b_42b9f4: {
                        'b_42b9e8: {
                            c.cycles += 8;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf78));
                            c.g[A2][0] = 0xfc;
                            c.g[A0][0] = c.g[S1][0];
                            callee::STRNCPY.call(c, 0x0042_b8b0)?;
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x8));
                            let t = c.g[S0][0] == 0u64;
                            c.sb((c.g[V0][0] as u32).wrapping_add(0x103), 0u64);
                            if t {
                                break 'b_42b9e8;
                            } else {
                                'b_42b9d0: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < 253i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        break 'b_42b9d0;
                                    } else {
                                        'b_42b984: {
                                            'b_42b944: {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                                                c.g[V0][0] = c.g[S0][0] | c.g[A0][0];
                                                c.g[V0][0] = c.g[V0][0] & 0x7;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_42b944;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                                    c.g[S5][0] = 0x530000;
                                                    c.g[A1][0] = 0x530000;
                                                    'l_42b8e8: loop {
                                                        c.cycles += 21;
                                                        c.ldl(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(6, c.g[V1][0] as u32);
                                                        c.ldl(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            8,
                                                            (c.g[V1][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            8,
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            9,
                                                            (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            9,
                                                            (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            6,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(6, c.g[A0][0] as u32);
                                                        c.sdl(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            8,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0042_b8e8);
                                                            }
                                                            continue 'l_42b8e8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            if t {
                                                                break 'b_42b984;
                                                            } else {
                                                                break 'b_42b944;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                            c.g[S5][0] = 0x530000;
                                            c.g[A1][0] = 0x530000;
                                            'l_42b950: loop {
                                                c.cycles += 13;
                                                c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                                                c.g[A3][0] =
                                                    c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                                c.g[T0][0] =
                                                    c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[T1][0] =
                                                    c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A0][0] as u32, c.g[A2][0]);
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[A3][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    c.g[T0][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    c.g[T1][0],
                                                );
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0042_b950);
                                                    }
                                                    continue 'l_42b950;
                                                } else {
                                                    break 'b_42b984;
                                                }
                                            }
                                        }
                                        c.cycles += 19;
                                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.ldr(6, c.g[V1][0] as u32);
                                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.lwl(9, (c.g[V1][0] as u32).wrapping_add(0x1b));
                                        c.lwr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(6, c.g[A0][0] as u32);
                                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.swl(9, (c.g[A0][0] as u32).wrapping_add(0x1b));
                                        c.g[V0][0] = 0xfc;
                                        c.swr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        let t = true;
                                        c.sw((c.g[S5][0] as u32).wrapping_sub(0xf80), c.g[V0][0]);
                                        if t {
                                            break 'b_42b9f8;
                                        } else {
                                            break 'b_42b9d0;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                                c.g[A2][0] = c.g[S2][0];
                                c.g[S5][0] = 0x530000;
                                callee::MEMCPY.call(c, 0x0042_b9e0)?;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x8), c.g[S2][0]);
                                if t {
                                    break 'b_42b9f4;
                                } else {
                                    break 'b_42b9e8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.sb((c.g[V0][0] as u32).wrapping_add(0x104), 0u64);
                        c.g[S5][0] = 0x530000;
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x8), 0u64);
                        break 'b_42b9f4;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = 0x530000;
                    break 'b_42b9f8;
                }
                'b_42ba38: {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xf80));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd80));
                    c.g[A1][0] = c.g[S3][0];
                    c.sw(sp(c), 0u64);
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x200;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x8;
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0042_ba24)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_42ba38;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        if t {
                            break 'b_42ba40;
                        } else {
                            break 'b_42ba38;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0xf80));
                c.sw(c.g[S4][0] as u32, c.g[V1][0]);
                break 'b_42ba40;
            }
        }
        c.cycles += 9;
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
        c.pc = ret;
        Ok(())
    })
}

/// _sceSifLoadElfPart (0x42baa8): the shared ELF-part loader: initialises
/// the loader with `LoadFileInit` (binding on demand), checks the rebind
/// stamp, copies the section names in, and calls RPC for the load.
pub fn _sce_sif_load_elf_part(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'b_42bb8c: {
            c.cycles += 14;

            c.sd(frame.at(c, 0x50), c.g[S4][0]);
            c.sd(frame.at(c, 0x40), c.g[S3][0]);
            c.g[S4][0] = c.g[A2][0];
            c.sd(frame.at(c, 0x30), c.g[S2][0]);
            c.g[S3][0] = c.g[A3][0];
            c.sd(frame.at(c, 0x10), c.g[S0][0]);
            c.g[S2][0] = c.g[A1][0];
            c.sd(frame.at(c, 0x60), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x20), c.g[S1][0]);
            callee::LOAD_FILE_INIT.call(c, 0x0042_bad8)?;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffff0000;
            if t {
                break 'b_42bb8c;
            } else {
                'b_42bafc: {
                    c.cycles += 4;
                    callee::NEEDS_REBIND.call(c, 0x0042_bae8)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x530000;
                        break 'b_42bafc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffc;
                        if t {
                            break 'b_42bb8c;
                        } else {
                            break 'b_42bafc;
                        }
                    }
                }
                'b_42bb6c: {
                    c.cycles += 25;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf78));
                    c.g[A2][0] = 0xfc;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x8));
                    callee::STRNCPY.call(c, 0x0042_bb14)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x103), 0u64);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                    c.g[A2][0] = 0xfc;
                    callee::STRNCPY.call(c, 0x0042_bb28)?;
                    c.g[A0][0] = 0x530000;
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x1ff), 0u64);
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd80));
                    c.sw(sp(c), 0u64);
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x200;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x10;
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0042_bb58)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x8));
                        break 'b_42bb6c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        if t {
                            break 'b_42bb8c;
                        } else {
                            break 'b_42bb6c;
                        }
                    }
                }
                'b_42bb80: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                        break 'b_42bb80;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xfffffffffffe0000;
                        let t = true;
                        c.g[V0][0] = c.g[V0][0] | 0xfffd;
                        if t {
                            break 'b_42bb8c;
                        } else {
                            break 'b_42bb80;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                break 'b_42bb8c;
            }
        }
        c.cycles += 8;
        c.g[RA][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
        c.pc = ret;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::put_words;

    use crate::difftest::DATA;

    pub(crate) const SEEDS: u32 = 200;

    pub(crate) fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    // ------------------------------------------------------------ syscalls

    #[test]
    fn sce_sif_set_dma_matches() {
        syscall_test(ssx3_ee::s15::f_00424160__generated, sce_sif_set_dma, 34);
    }

    #[test]
    fn isce_sif_set_dchain_matches() {
        syscall_test(ssx3_ee::s15::f_00424190__generated, isce_sif_set_dchain, 35);
    }

    #[test]
    fn sce_sif_dma_stat_matches() {
        syscall_test(ssx3_ee::s15::f_00424140__generated, sce_sif_dma_stat, 38);
    }

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;

    /// sceSifCmdIntrHandler: no packet (size 0) or a packet of 1..0x70
    /// bytes; system or user command ids in range or not, handlers set or
    /// null. The SetDChain syscall and the handler are fakes (sometimes
    /// unwinding); EIntr runs for real.
    #[test]
    fn sce_sif_cmd_intr_handler_matches() {
        covered(
            ee::s15::f_00426230__generated,
            sce_sif_cmd_intr_handler,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x426230, 2);
                c.pc = 0x0042_6230;
                c.cop0[12] = r.next_u32();
                let recv = DATA;
                let (sys, usr) = (DATA + 0x200, DATA + 0x300);
                fill(c, &mut r, recv, 0x20);
                let size = if seed % 6 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x70
                };
                c.bus.write8(recv, size as u8);
                let n = r.next_u32() % 8;
                let cid = match r.next_u32() % 4 {
                    0 => 0x8000_0000 | n,
                    1 => n,
                    2 => 0x8000_0000 | (5 + r.next_u32() % 8),
                    _ => 5 + r.next_u32() % 8,
                };
                c.bus.write32(recv + sif_cmd::PKT_CID, cid);
                put_words(c, sif_cmd::DATA, &[recv]);
                c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_TABLE, sys);
                c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_COUNT, 6);
                c.bus.write32(sif_cmd::DATA + sif_cmd::USR_TABLE, usr);
                c.bus.write32(sif_cmd::DATA + sif_cmd::USR_COUNT, 6);
                for k in 0..8 {
                    let f = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * k
                    };
                    c.bus.write32(sys + 8 * k, f);
                    c.bus.write32(sys + 8 * k + 4, r.next_u32());
                    let f = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * k
                    };
                    c.bus.write32(usr + 8 * k, f);
                    c.bus.write32(usr + 8 * k + 4, r.next_u32());
                }
            },
        );
    }
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::require_paths;

    /// Write a NUL-terminated string to guest RAM.
    pub(crate) fn put_cstr(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// Free SIF RPC packet table at 0x52d680 so BIND_RPC and CALL_RPC can
    /// succeed headless: GetPacket reads the packet count at +8 and the
    /// first packet at +4, then claims a packet whose [pkt+0x10] bit0 is
    /// clear. Four free packets cover one BIND plus one CALL per side.
    /// Gated on `seed % 9 == 8`: other seeds keep count 0 (bind/call fail,
    /// old paths preserved). A free packet gates success the way the
    /// hardware does; exhausted/absent tables fail exactly as before.
    pub(crate) fn sif_packet_setup(c: &mut Cpu, seed: u32) {
        if seed % 9 == 8 {
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x700 + 0x40 * i + 0x10, 0);
            }
            c.bus.write32(0x0052_d680, 0);
            c.bus.write32(0x0052_d684, DATA + 0x700);
            c.bus.write32(0x0052_d688, 4);
        } else {
            c.bus.write32(0x0052_d688, 0);
        }
    }

    /// Steer `LoadFileInit` by the init word: even seeds see `0x455228 == 0`
    /// (already initialized, early exit), odd seeds `-1` (full bind path).
    pub(crate) fn loadfile_setup(c: &mut Cpu, seed: u32) {
        c.bus
            .write32(0x0045_5228, if seed % 2 == 0 { 0 } else { 0xFFFF_FFFF });
        sif_packet_setup(c, seed);
    }

    #[test]
    fn sce_sif_reboot_iop_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121303, 1);
            c.pc = 0x0042_bf68;
            c.g[A0][0] = (DATA + 0x104) as u64;
            match seed % 4 {
                0 => put_cstr(c, DATA + 0x104, b""),
                1 => put_cstr(c, DATA + 0x104, b"abcd"),
                2 => {
                    for i in 0..0x60 {
                        c.bus.write8(DATA + 0x104 + i, b'x');
                    }
                    c.bus.write8(DATA + 0x104 + 0x60, 0);
                }
                // Boundary length: V0 counts len+0xb, so a 70-char string makes
                // V0==0x51 exactly at the check (long arm like the 0x60 case).
                _ => {
                    for i in 0..70 {
                        c.bus.write8(DATA + 0x104 + i, b'y');
                    }
                    c.bus.write8(DATA + 0x104 + 70, 0);
                }
            }
            // Set the S1 done byte nonzero so the post-`InitRpc` copy loop
            // runs instead of being skipped (S1 = 0x4a0000-0x52a0).
            if seed % 5 == 4 {
                c.bus.write8(0x0049_ad60, 0x41);
                c.bus.write8(0x0049_ad61, 0x42);
            }
            let _ = r;
        };
        check(
            ee::s15::f_0042bf68__generated,
            sce_sif_reboot_iop,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042bf68__generated,
            SEEDS,
            // Empty (647, 1233 unwind), short (1338, 681 unwind) and long
            // (756) image paths: all three strlen classes, both unwind sides.
            &[647, 681, 756, 1233, 1338],
            setup,
        );
    }

    #[test]
    fn sce_sif_reboot_iop_events() {
        check_events(
            ee::s15::f_0042bf68__generated,
            sce_sif_reboot_iop,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121303, 0);
                c.pc = 0x0042_bf68;
                c.g[A0][0] = (DATA + 0x104) as u64;
                put_cstr(c, DATA + 0x104, if seed % 2 == 0 { b"abcd" } else { b"" });
                schedule(c, &mut r, seed, 0x0042_bf68)
            },
        );
    }

    #[test]
    fn sce_sif_sync_iop_matches() {
        check(
            ee::s15::f_0042bf30__generated,
            sce_sif_sync_iop,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121304, 1);
                c.pc = 0x0042_bf30;
                // Force the GetReg result (first fake call) to have the
                // reset-done bit set on some seeds, so the done arm runs
                // (the harness fake never sets bit 0x40000 on its own).
                if seed % 9 == 8 {
                    c.bus.write32(crate::difftest::fakes::LOG_RESULTS, 0x40000);
                }
            },
        );
    }

    #[test]
    fn sce_sif_is_alive_iop_matches() {
        check(
            ee::s15::f_0042bf08__generated,
            sce_sif_is_alive_iop,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121305, 1);
                c.pc = 0x0042_bf08;
            },
        );
    }

    #[test]
    fn sce_sif_reset_iop_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121306, 1);
            c.pc = 0x0042_bdc8;
            c.g[A0][0] = (DATA + 0x104) as u64;
            c.g[A1][0] = (DATA + 0x204) as u64;
            put_cstr(c, DATA + 0x104, if seed % 2 == 0 { b"abcd" } else { b"" });
            // Nonzero descriptor word (T2-0xd40 = 0x52f2c0) so the ld&mask
            // result is observable in compared RAM: the surviving low byte
            // and high half must both be nonzero.
            if seed % 5 == 4 {
                c.bus.write32(0x0052_f2c0, 0x7a007a41);
                c.bus.write32(0x0052_f2c4, 0x00ff00ff);
            }
            let _ = r;
        };
        check(
            ee::s15::f_0042bdc8__generated,
            sce_sif_reset_iop,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042bdc8__generated,
            SEEDS,
            // "abcd" (226) vs "" (172, 29 unwind).
            &[29, 172, 226],
            setup,
        );
    }

    #[test]
    fn sce_sif_reset_iop_events() {
        check_events(
            ee::s15::f_0042bdc8__generated,
            sce_sif_reset_iop,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121306, 0);
                c.pc = 0x0042_bdc8;
                c.g[A0][0] = (DATA + 0x104) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                put_cstr(c, DATA + 0x104, b"abcd");
                schedule(c, &mut r, seed, 0x0042_bdc8)
            },
        );
    }

    #[test]
    fn sce_sif_iop_set_val_matches() {
        check(
            ee::s15::f_0042bce8__generated,
            sce_sif_iop_set_val,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121307, 1);
                c.pc = 0x0042_bce8;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x84) as u64;
                c.g[A1][0] = (DATA + 0x104) as u64;
                // S0==3 with init-ok (even seed): reaches the 0xFFFEFFFE early-return arm.
                c.g[A2][0] = (if seed % 8 == 2 { 3 } else { seed % 4 }) as u64;
                // Nonzero S1 word so halfword/word widths diverge (lhu vs lw).
                c.bus
                    .write32(DATA + 0x104, 0x7a00_7a00u32.wrapping_add(seed));
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_iop_get_val_matches() {
        check(
            ee::s15::f_0042bbf8__generated,
            sce_sif_iop_get_val,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121308, 1);
                c.pc = 0x0042_bbf8;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x84) as u64;
                c.g[A1][0] = (DATA + 0x104) as u64;
                // S0==3 on an init-ok (even) seed: the S0<3 value is only live
                // when init succeeds (init-fail overwrites V0 before any read),
                // and init-ok needs an even seed (loadfile_setup parity word).
                c.g[A2][0] = (if seed % 4 == 0 { 3 } else { seed % 4 }) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_load_elf_part_matches() {
        check(
            ee::s15::f_0042bbb0__generated,
            sce_sif_load_elf_part,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121309, 1);
                c.pc = 0x0042_bbb0;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                put_cstr(c, DATA + 0x104, b"mod.irx");
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_load_start_module_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121311, 1);
            c.pc = 0x0042_ba88;
            loadfile_setup(c, seed);
            c.g[A0][0] = (DATA + 0x104) as u64;
            c.g[A1][0] = (DATA + 0x204) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                0
            } else {
                (DATA + 0x304) as u64
            };
            put_cstr(c, DATA + 0x104, b"mod.irx");
            let _ = r;
        };
        check(
            ee::s15::f_0042ba88__generated,
            sce_sif_load_start_module,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042ba88__generated,
            SEEDS,
            // Small args (2005) vs buffered args (113, 2180 unwind).
            &[113, 2005, 2180],
            setup,
        );
    }

    #[test]
    fn sce_sif_load_module_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121312, 1);
            c.pc = 0x0042_ba68;
            loadfile_setup(c, seed);
            c.g[A0][0] = (DATA + 0x104) as u64;
            c.g[A1][0] = if seed % 3 == 0 {
                (seed % 32) as u64
            } else {
                (DATA + 0x204) as u64
            };
            c.g[A2][0] = if seed % 3 == 0 {
                0
            } else {
                (DATA + 0x304) as u64
            };
            put_cstr(c, DATA + 0x104, b"mod.irx");
            let _ = r;
        };
        check(
            ee::s15::f_0042ba68__generated,
            sce_sif_load_module,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042ba68__generated,
            SEEDS,
            // Small args (2006) vs buffered args (114, 2181 unwind).
            &[114, 2006, 2181],
            setup,
        );
    }

    #[test]
    fn sce_sif_load_start_module_buffer_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121314, 1);
            c.pc = 0x0042_b820;
            loadfile_setup(c, seed);
            c.g[A0][0] = (DATA + 0x104) as u64;
            c.g[A1][0] = (DATA + 0x204) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                0
            } else {
                (DATA + 0x304) as u64
            };
            let _ = r;
        };
        check(
            ee::s15::f_0042b820__generated,
            sce_sif_load_start_module_buffer,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042b820__generated,
            SEEDS,
            // Small args (208) vs buffered args (114, 385 unwind).
            &[114, 208, 385],
            setup,
        );
    }

    #[test]
    fn sce_sif_load_module_buffer_2_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121315, 1);
            c.pc = 0x0042_b800;
            loadfile_setup(c, seed);
            c.g[A0][0] = (DATA + 0x104) as u64;
            c.g[A1][0] = (DATA + 0x204) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                0
            } else {
                (DATA + 0x304) as u64
            };
            let _ = r;
        };
        check(
            ee::s15::f_0042b800__generated,
            sce_sif_load_module_buffer_2,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042b800__generated,
            SEEDS,
            // Small args (208) vs buffered args (114, 385 unwind).
            &[114, 208, 385],
            setup,
        );
    }

    #[test]
    fn sce_sif_search_module_by_address_matches() {
        check(
            ee::s15::f_0042b770__generated,
            sce_sif_search_module_by_address,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121316, 1);
                c.pc = 0x0042_b770;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                if seed % 5 == 4 {
                    // Force NEEDS_REBIND nonzero (fail arm): distinct nonzero
                    // rebind words so the memcmp chain resolves nonzero.
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_search_module_by_name_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let r = machine(c, seed, 121317, 1);
            c.pc = 0x0042_b6d0;
            loadfile_setup(c, seed);
            c.g[A0][0] = (DATA + 0x104) as u64;
            put_cstr(c, DATA + 0x104, b"mod.irx");
            if seed % 5 == 4 {
                // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                // distinct nonzero words resolve the whole chain nonzero.
                c.bus.write32(0x0052_f2a8, 0x7a007a00);
                c.bus.write32(0x0045_5164, 0x00ff00ff);
            }
            let _ = r;
        };
        check(
            ee::s15::f_0042b6d0__generated,
            sce_sif_search_module_by_name,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042b6d0__generated,
            SEEDS,
            // Even seeds (1978, full bind+RPC path) vs odd (92, early exit).
            &[92, 1978],
            setup,
        );
    }

    #[test]
    fn sce_sif_unload_module_matches() {
        check(
            ee::s15::f_0042b640__generated,
            sce_sif_unload_module,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121318, 1);
                c.pc = 0x0042_b640;
                loadfile_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                if seed % 5 == 4 {
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_stop_module_matches() {
        check(
            ee::s15::f_0042b438__generated,
            sce_sif_stop_module,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121319, 1);
                c.pc = 0x0042_b438;
                loadfile_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                // S1 small WITH S0 nonzero (decoupled from the S0==0 skip):
                // seed%3==0 gives small A1, seed%5==4 forces A2 nonzero so the
                // S1<253 arm (and S1==253 on seed%11==10) is actually reached.
                c.g[A1][0] = if seed % 11 == 10 {
                    253
                } else if seed % 3 == 0 {
                    (seed % 32) as u64
                } else {
                    (DATA + 0x204) as u64
                };
                c.g[A2][0] = if seed % 5 == 4 {
                    // Low 3 bits clear so (S0&A0)&7==0 differs from (S0|A0)&7.
                    0x800300
                } else if seed % 3 == 0 && seed % 11 != 10 {
                    0
                } else {
                    (DATA + 0x304) as u64
                };
                c.g[A3][0] = (DATA + 0x404) as u64;
                // Fill the copy source (S0 = A2, 0xe0 bytes) with distinct
                // nonzero bytes so misdirected reads/writes diverge.
                // A2==0 seeds keep zero buffers.
                if c.g[A2][0] != 0 {
                    let base = c.g[A2][0] as u32;
                    for i in 0..0xe0u32 {
                        let b = ((i * 7 + 3) & 0xff) as u8;
                        c.bus.write8(base + i, if b == 0 { 1 } else { b });
                    }
                }
                if seed % 7 == 6 {
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_stop_module_events() {
        check_events(
            ee::s15::f_0042b438__generated,
            sce_sif_stop_module,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121319, 0);
                c.pc = 0x0042_b438;
                loadfile_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                c.g[A2][0] = (DATA + 0x304) as u64;
                c.g[A3][0] = (DATA + 0x404) as u64;
                schedule(c, &mut r, seed, 0x0042_b438)
            },
        );
    }

    #[test]
    fn sce_sif_load_module_buffer_matches() {
        check(
            ee::s15::f_0042b230__generated,
            sce_sif_load_module_buffer,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121320, 1);
                c.pc = 0x0042_b230;
                loadfile_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                c.g[A1][0] = if seed % 11 == 10 {
                    253
                } else if seed % 3 == 0 {
                    (seed % 32) as u64
                } else {
                    (DATA + 0x204) as u64
                };
                c.g[A2][0] = if seed % 5 == 4 {
                    0x800300
                } else if seed % 3 == 0 && seed % 11 != 10 {
                    0
                } else {
                    (DATA + 0x304) as u64
                };
                c.g[A3][0] = (DATA + 0x404) as u64;
                // Fill the copy source (S0 = A2, 0xe0 bytes) with distinct
                // nonzero bytes so misdirected reads/writes diverge.
                // A2==0 seeds keep zero buffers.
                if c.g[A2][0] != 0 {
                    let base = c.g[A2][0] as u32;
                    for i in 0..0xe0u32 {
                        let b = ((i * 7 + 3) & 0xff) as u8;
                        c.bus.write8(base + i, if b == 0 { 1 } else { b });
                    }
                }
                if seed % 7 == 6 {
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_load_module_buffer_events() {
        check_events(
            ee::s15::f_0042b230__generated,
            sce_sif_load_module_buffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121320, 0);
                c.pc = 0x0042_b230;
                loadfile_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                c.g[A2][0] = (DATA + 0x304) as u64;
                c.g[A3][0] = (DATA + 0x404) as u64;
                schedule(c, &mut r, seed, 0x0042_b230)
            },
        );
    }

    /// Steer the Query/Heap/Alloc gating word: even seeds take the RPC
    /// path (`0x455224 == 0`), odd seeds return 0 early (`-1`).
    /// Also serves the success reply word at `0x52ef00` (`0x530000 - 0x1100`)
    /// so the success return's load changes `V0`.
    pub(crate) fn query_setup(c: &mut Cpu, seed: u32) {
        c.bus
            .write32(0x0045_5224, if seed % 2 == 0 { 0 } else { 0xFFFF_FFFF });
        c.bus.write32(0x0052_ef00, 0x7a00_7a00);
        sif_packet_setup(c, seed);
    }

    #[test]
    fn libkernl_loadfile_needs_rebind_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 121402, 1);
            c.pc = 0x0042_b168;
            // The three compared stamps: `[S1]` at 0x52F2A8, `[S3]`
            // at 0x455164, and the bound stamp at 0x45522C.
            match seed % 4 {
                0 => {}
                1 => {
                    put_words(c, 0x0052_f2a8, &[0x0101_0101]);
                }
                2 => {
                    put_words(c, 0x0052_f2a8, &[0x0101_0101]);
                    put_words(c, 0x0045_522c, &[0x0202_0202]);
                    put_words(c, 0x0045_5164, &[0xFFFF_FFFF]);
                }
                _ => {
                    put_words(c, 0x0052_f2a8, &[0x0101_0101]);
                    put_words(c, 0x0045_522c, &[0xFFFF_FFFF]);
                }
            }
        };
        check(
            ee::s15::f_0042b168__generated,
            libkernl_loadfile_needs_rebind,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042b168__generated,
            SEEDS,
            // Stamps untouched (65), S1 pinned + bound (83),
            // S1 pinned (107, classes 1+3).
            &[65, 83, 107],
            setup,
        );
    }

    #[test]
    fn sce_sif_load_file_reset_matches() {
        check(
            ee::s15::f_0042b1f8__generated,
            sce_sif_load_file_reset,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121401, 1);
                c.pc = 0x0042_b1f8;
            },
        );
    }

    #[test]
    fn sce_sif_load_file_init_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 121403, 1);
            c.pc = 0x0042_b068;
            loadfile_setup(c, seed);
        };
        check(
            ee::s15::f_0042b068__generated,
            sce_sif_load_file_init,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042b068__generated,
            SEEDS,
            // Init word clear (15, early exit) vs set (81, full bind path).
            // Packet seeds (%9==8) run the post-bind delay (29361355).
            &[15, 81, 29361355],
            setup,
        );
    }

    #[test]
    fn sce_sif_load_file_init_events() {
        check_events(
            ee::s15::f_0042b068__generated,
            sce_sif_load_file_init,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121403, 0);
                c.pc = 0x0042_b068;
                loadfile_setup(c, seed);
                schedule(c, &mut r, seed, 0x0042_b068)
            },
        );
    }

    #[test]
    fn sce_sif_query_block_size_matches() {
        check(
            ee::s15::f_0042afe8__generated,
            sce_sif_query_block_size,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121404, 1);
                c.pc = 0x0042_afe8;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_query_block_top_address_matches() {
        check(
            ee::s15::f_0042af68__generated,
            sce_sif_query_block_top_address,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121405, 1);
                c.pc = 0x0042_af68;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_query_total_free_mem_size_matches() {
        check(
            ee::s15::f_0042aef8__generated,
            sce_sif_query_total_free_mem_size,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121406, 1);
                c.pc = 0x0042_aef8;
                query_setup(c, seed);
            },
        );
    }

    #[test]
    fn sce_sif_query_max_free_mem_size_matches() {
        check(
            ee::s15::f_0042ae88__generated,
            sce_sif_query_max_free_mem_size,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121407, 1);
                c.pc = 0x0042_ae88;
                query_setup(c, seed);
            },
        );
    }

    #[test]
    fn sce_sif_query_mem_size_matches() {
        check(
            ee::s15::f_0042ae18__generated,
            sce_sif_query_mem_size,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121408, 1);
                c.pc = 0x0042_ae18;
                query_setup(c, seed);
            },
        );
    }

    #[test]
    fn sce_sif_load_iop_heap_matches() {
        check(
            ee::s15::f_0042ad28__generated,
            sce_sif_load_iop_heap,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121409, 1);
                c.pc = 0x0042_ad28;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                if seed % 7 == 6 {
                    // 252-char heap name: runs the copy loop to its T0==252 bound.
                    for i in 0..252u32 {
                        c.bus.write8(DATA + 0x104 + i, b'h');
                    }
                    c.bus.write8(DATA + 0x104 + 252, 0);
                } else if seed % 2 == 0 && seed % 11 != 10 {
                    put_cstr(c, DATA + 0x104, b"heap0");
                } else {
                    // Empty name (also on some even seeds, where the gate
                    // word is 0, so the NUL-first copy arm actually runs).
                    put_cstr(c, DATA + 0x104, b"");
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_load_iop_heap_events() {
        check_events(
            ee::s15::f_0042ad28__generated,
            sce_sif_load_iop_heap,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121409, 0);
                c.pc = 0x0042_ad28;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                put_cstr(c, DATA + 0x104, b"heap0");
                schedule(c, &mut r, seed, 0x0042_ad28)
            },
        );
    }

    #[test]
    fn sce_sif_free_sys_memory_matches() {
        check(
            ee::s15::f_0042ad08__generated,
            sce_sif_free_sys_memory,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121410, 1);
                c.pc = 0x0042_ad08;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_free_iop_heap_matches() {
        check(
            ee::s15::f_0042ac90__generated,
            sce_sif_free_iop_heap,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121411, 1);
                c.pc = 0x0042_ac90;
                query_setup(c, seed);
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn sce_sif_alloc_sys_memory_matches() {
        check(
            ee::s15::f_0042ac10__generated,
            sce_sif_alloc_sys_memory,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121412, 1);
                c.pc = 0x0042_ac10;
                query_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
                c.g[A1][0] = (seed % 16) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn _sce_sif_load_module_matches() {
        check(
            ee::s15::f_0042b840__generated,
            _sce_sif_load_module,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121313, 1);
                c.pc = 0x0042_b840;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                c.g[A1][0] = if seed % 11 == 10 {
                    253
                } else if seed % 3 == 0 {
                    (seed % 32) as u64
                } else {
                    (DATA + 0x204) as u64
                };
                c.g[A2][0] = if seed % 5 == 4 {
                    0x800300
                } else if seed % 3 == 0 && seed % 11 != 10 {
                    0
                } else {
                    (DATA + 0x304) as u64
                };
                c.g[T0][0] = (seed % 2) as u64;
                put_cstr(c, DATA + 0x104, b"mod.irx");
                // Fill the copy source (S0 = A2, 0xe0 bytes) with distinct
                // nonzero bytes so misdirected reads/writes diverge.
                // A2==0 seeds keep zero buffers.
                if c.g[A2][0] != 0 {
                    let base = c.g[A2][0] as u32;
                    for i in 0..0xe0u32 {
                        let b = ((i * 7 + 3) & 0xff) as u8;
                        c.bus.write8(base + i, if b == 0 { 1 } else { b });
                    }
                }
                if seed % 7 == 6 {
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn _sce_sif_load_module_events() {
        check_events(
            ee::s15::f_0042b840__generated,
            _sce_sif_load_module,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121313, 0);
                c.pc = 0x0042_b840;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                c.g[A2][0] = (DATA + 0x304) as u64;
                c.g[T0][0] = 0;
                put_cstr(c, DATA + 0x104, b"mod.irx");
                schedule(c, &mut r, seed, 0x0042_b840)
            },
        );
    }

    #[test]
    fn _sce_sif_load_elf_part_matches() {
        check(
            ee::s15::f_0042baa8__generated,
            _sce_sif_load_elf_part,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121310, 1);
                c.pc = 0x0042_baa8;
                loadfile_setup(c, seed);
                c.g[A0][0] = (DATA + 0x104) as u64;
                c.g[A1][0] = (DATA + 0x204) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 2) as u64;
                put_cstr(c, DATA + 0x104, b"mod.irx");
                put_cstr(c, DATA + 0x204, b"args");
                if seed % 5 == 4 {
                    // NEEDS_REBIND reads [0x52f2a8] vs [0x455164] (memcmp#1):
                    // distinct nonzero words resolve the whole chain nonzero.
                    c.bus.write32(0x0052_f2a8, 0x7a007a00);
                    c.bus.write32(0x0045_5164, 0x00ff00ff);
                }
                let _ = r;
            },
        );
    }
}
