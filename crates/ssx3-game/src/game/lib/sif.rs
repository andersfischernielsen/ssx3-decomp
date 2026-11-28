//! `src/lib/sif.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::lib::kernel::callee::FLUSH_CACHE;
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
    (0x0040_b130, sce_sif_minit_rpc, "sceSifMInitRpc"),
    (0x0040_b1f8, sce_sif_mexit_rpc, "sceSifMExitRpc"),
    (
        0x0040_b3d8,
        sce_sif_mbind_rpc_default,
        "sceSifMBindRpcDefault",
    ),
    (0x0040_b400, sce_sif_mbind_rpc, "sceSifMBindRpc"),
    (0x0040_b5b0, sce_sif_mterm_rpc, "sceSifMTermRpc"),
    (0x0040_b6d0, sce_sif_mcall_rpc, "sceSifMCallRpc"),
    (0x0040_b208, libmrpc_get_packet, "libmrpc_GetPacket"),
    (0x0040_b2b0, libmrpc_free_packet, "libmrpc_FreePacket"),
    (0x0040_b2d0, libmrpc_request_end, "libmrpc_RequestEnd"),
    (0x0042_4070, sce_sif_stop_dma, "sceSifStopDma"),
    (0x0042_4150, isce_sif_dma_stat, "isceSifDmaStat"),
    (0x0042_4170, isce_sif_set_dma, "isceSifSetDma"),
    (0x0042_4180, sce_sif_set_dchain, "sceSifSetDChain"),
    (0x0042_41a0, sce_sif_set_reg, "sceSifSetReg"),
    (0x0042_41b0, sce_sif_get_reg, "sceSifGetReg"),
    (
        0x0042_5ce0,
        sce_sif_cmd_change_addr_handler,
        "sceSifCmd_ChangeAddrHandler",
    ),
    (0x0042_5fb8, sce_sif_exit_cmd, "sceSifExitCmd"),
    (
        0x0042_6050,
        sce_sif_remove_cmd_handler,
        "sceSifRemoveCmdHandler",
    ),
    (0x0042_61f0, isce_sif_send_cmd, "isceSifSendCmd"),
    (0x0042_6408, sce_sif_init_rpc, "sceSifInitRpc"),
    (0x0042_65a8, sce_sif_exit_rpc, "sceSifExitRpc"),
    (0x0042_6698, sce_sif_rpc_get_fpacket, "sceSifRpc_GetFPacket"),
    (
        0x0042_66c8,
        sce_sif_rpc_get_fpacket2,
        "sceSifRpc_GetFPacket2",
    ),
    (0x0042_73f8, sce_sif_rpc_loop, "sceSifRpcLoop"),
    (0x0042_6078, _sce_sif_send_cmd, "_sceSifSendCmd"),
    (0x0042_41d0, deci2_call, "Deci2Call"),
    (0x0042_5d38, sce_sif_init_cmd, "sceSifInitCmd"),
    (
        0x0042_67c0,
        sce_sif_rpc_retry_send_alarm,
        "sceSifRpc_RetrySendAlarm",
    ),
    (
        0x0042_6820,
        sce_sif_rpc_request_rdata,
        "sceSifRpc_RequestRData",
    ),
    (0x0042_68f0, sce_sif_get_other_data, "sceSifGetOtherData"),
    (
        0x0042_6a48,
        sce_sif_rpc_search_svdata,
        "sceSifRpc_SearchSvdata",
    ),
    (
        0x0042_6a98,
        sce_sif_rpc_request_bind,
        "sceSifRpc_RequestBind",
    ),
    (
        0x0042_6c88,
        sce_sif_rpc_request_call,
        "sceSifRpc_RequestCall",
    ),
    (0x0042_6f48, sce_sif_set_rpc_queue, "sceSifSetRpcQueue"),
    (0x0042_70b0, sce_sif_remove_rpc, "sceSifRemoveRpc"),
    (
        0x0042_7148,
        sce_sif_remove_rpc_queue,
        "sceSifRemoveRpcQueue",
    ),
    (
        0x0042_71d8,
        sce_sif_get_next_request,
        "sceSifGetNextRequest",
    ),
    (0x0042_7230, sce_sif_exec_request, "sceSifExecRequest"),
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

    /// `isceSifSetDma` (`0x00424170`).
    pub(crate) const ISET_DMA: Callee = Callee {
        addr: 0x0042_4170,
        f: ee::s15::f_00424170,
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
    /// `libmrpc_GetPacket` (`0x0040_b208`).
    pub(crate) const MRPC_GET_PACKET: Callee = Callee {
        addr: 0x0040_b208,
        f: ee::s15::libmrpc_GetPacket,
    };

    /// `libmrpc_FreePacket` (`0x0040_b2b0`).
    pub(crate) const MRPC_FREE_PACKET: Callee = Callee {
        addr: 0x0040_b2b0,
        f: ee::s15::libmrpc_FreePacket,
    };

    /// `sceSifMBindRpc` (`0x0040_b400`).
    pub(crate) const MBIND_RPC: Callee = Callee {
        addr: 0x0040_b400,
        f: ee::s15::sceSifMBindRpc,
    };

    pub(super) use crate::game::lib::kernel::callee::CREATE_SEMA;

    /// `DeleteSema` (`0x0042_3db0`).
    pub(crate) const DELETE_SEMA: Callee = Callee {
        addr: 0x0042_3db0,
        f: ee::s15::DeleteSema,
    };

    pub(super) use crate::game::lib::cdvd::callee::ISIGNAL_SEMA;

    pub(super) use crate::game::lib::usbkb::callee::WAIT_SEMA;

    /// `sceSifGetSreg` (`0x0042_5cf0`).
    pub(crate) const GET_SREG: Callee = Callee {
        addr: 0x0042_5cf0,
        f: ee::s15::sceSifGetSreg,
    };

    pub(super) use crate::game::lib::cdvd::callee::ADD_CMD_HANDLER;

    /// `sceSifSendCmd` (`0x0042_61b0`).
    pub(crate) const SEND_CMD: Callee = Callee {
        addr: 0x0042_61b0,
        f: ee::s15::sceSifSendCmd,
    };

    /// `sceSifInitCmd` (`0x0042_5d38`; shared with `buddyapi`).
    pub(crate) use crate::game::dirtysock::buddyapi::callee::SCE_SIF_INIT_CMD as INIT_CMD;

    /// `sceSifExitCmd` (`0x0042_5fb8`).
    pub(crate) const EXIT_CMD: Callee = Callee {
        addr: 0x0042_5fb8,
        f: ee::s15::f_00425fb8,
    };

    /// Inner `_sceSifSendCmd` (`0x0042_6078`; `SEND_CMD` is its wrapper).
    pub(crate) const SEND_CMD_INNER: Callee = Callee {
        addr: 0x0042_6078,
        f: ee::s15::f_00426078,
    };

    /// `DisableDmac` (`0x0042_4950`).
    pub(crate) const DISABLE_DMAC: Callee = Callee {
        addr: 0x0042_4950,
        f: ee::s15::f_00424950,
    };

    /// `RemoveDmacHandler` (`0x0042_3ad0`).
    pub(crate) const REMOVE_DMAC_HANDLER: Callee = Callee {
        addr: 0x0042_3ad0,
        f: ee::s15::f_00423ad0,
    };

    /// `sceSifRpc_GetFPacket` (`0x0042_6698`).
    pub(crate) const GET_FPACKET: Callee = Callee {
        addr: 0x0042_6698,
        f: ee::s15::f_00426698,
    };

    /// `sceSifExecRequest` (`0x0042_7230`).
    pub(crate) const EXEC_REQUEST: Callee = Callee {
        addr: 0x0042_7230,
        f: ee::s15::f_00427230,
    };

    /// `sceSifGetNextRequest` (`0x0042_71d8`).
    pub(crate) const GET_NEXT_REQUEST: Callee = Callee {
        addr: 0x0042_71d8,
        f: ee::s15::f_004271d8,
    };

    /// `isceSifSendCmd` (`0x004261f0`).
    pub(crate) const ISCE_SEND_CMD: Callee = Callee {
        addr: 0x0042_61f0,
        f: ee::s15::f_004261f0,
    };

    /// `sceSifRpc_GetFPacket2` (`0x004266c8`).
    pub(crate) const GET_FPACKET2: Callee = Callee {
        addr: 0x0042_66c8,
        f: ee::s15::f_004266c8,
    };

    pub(crate) use crate::game::seg::seg_325220::oc698_callee::{
        OC698_SCE_SIF_RPC_FREE_PACKET, OC698_SCE_SIF_RPC_GET_PACKET,
    };

    /// `sceSifRpc_SearchSvdata` (`0x00426a48`; same-module call).
    pub(crate) const SEARCH_SVDATA: Callee = Callee {
        addr: 0x0042_6a48,
        f: ee::s15::f_00426a48,
    };

    /// `sceFs_FdSemaInit` (`0x00427438`).
    pub(crate) const FD_SEMA_INIT: Callee = Callee {
        addr: 0x0042_7438,
        f: ee::s15::f_00427438,
    };

    /// `AddDmacHandler` (`0x00423ab0`).
    pub(crate) const ADD_DMAC_HANDLER: Callee = Callee {
        addr: 0x0042_3ab0,
        f: ee::s15::f_00423ab0,
    };

    /// `sceSifSetDChain` (`0x00424180`).
    pub(crate) const SCE_SET_DCHAIN: Callee = Callee {
        addr: 0x0042_4180,
        f: ee::s15::f_00424180,
    };

    /// `EnableDmac` (`0x004249b8`, the interrupt-saving wrapper).
    pub(crate) const ENABLE_DMAC: Callee = Callee {
        addr: 0x0042_49b8,
        f: ee::s15::f_004249b8,
    };

    /// `SleepThread` (`0x0042_3cc0`; shared with `buddyapi`).
    pub(crate) use crate::game::dirtysock::buddyapi::callee::SLEEP_THREAD;

    pub(super) use crate::game::lib::cdvd::callee::DINTR;
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
        if !t {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x522c));
            c.g[A2][0] = 0x4;
            callee::MEMCMP.call(c, 0x0042_b1bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x522c));
            if !t {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = 0x4;
                callee::MEMCMP.call(c, 0x0042_b1d0)?;
                c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
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

// ---------------------------------------------------------------------------
// Shared SIF-RPC / memory-card RPC layout.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// `int sceSifMCallRpc(client, sub, mode, send, send_size, recv, recv_size,
// tid, end_cb, end_param)` (0x40b6d0): nonzero mode prints and returns
// -1; misaligned send/recv sizes print and return -1; no free packet
// returns -1. Mode&1 sends fire-and-forget (0, or -2 when the send
// fails); mode 0 waits on a fresh sema after a successful send.
pub fn sce_sif_mcall_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        c.sd(frame.at(c, 0xa0), c.g[FP][0]);
        c.sd(frame.at(c, 0x90), c.g[S7][0]);
        c.g[FP][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x80), c.g[S6][0]);
        c.g[S7][0] = c.g[T3][0];
        c.sd(frame.at(c, 0x70), c.g[S5][0]);
        c.g[S6][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x60), c.g[S4][0]);
        c.g[S5][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x50), c.g[S3][0]);
        c.g[S4][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x40), c.g[S2][0]);
        c.g[S3][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0xb0), c.g[RA][0]);
        c.g[S1][0] = c.g[T2][0];
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        if c.g[FP][0] != 0 {
            // Nonzero mode: print and fail.
            c.cycles += 5;
            c.g[A0][0] = sx(0x0049_8178);
            c.g[RA][0] = sx(0x0040_b72c);
            callee::PRINTF.call(c, 0x0040_b72c)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            // Mode 0: wait on the client sema first, then check sizes.
            c.cycles += 5;
            c.g[RA][0] = sx(0x0040_b73c);
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
            callee::WAIT_SEMA.call(c, 0x0040_b73c)?;
            c.g[V1][0] = c.g[S4][0] & 0x3f;
            c.g[A0][0] = 0x4a_0000;
            if c.g[V1][0] != 0 {
                // Misaligned send size: print and fail.
                c.cycles += 5;
                c.g[A1][0] = sx(0x0049_81d8);
                c.g[A0][0] = sx(0x0049_81a0);
                c.g[A2][0] = c.g[S4][0];
                c.cycles += 4;
                c.g[RA][0] = sx(0x0040_b780);
                callee::PRINTF.call(c, 0x0040_b780)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else if {
                c.cycles += 3;
                c.g[V0][0] = c.g[S5][0] & 0x3f;
                c.g[A1][0] = 0x4a0000;
                c.g[V0][0] != 0
            } {
                // Misaligned receive size: print and fail.
                c.cycles += 4;
                c.g[A0][0] = sx(0x0049_81e8);
                c.g[A1][0] = sx(0x0049_81d8);
                c.g[A2][0] = c.g[S5][0];
                c.cycles += 4;
                c.g[RA][0] = sx(0x0040_b780);
                callee::PRINTF.call(c, 0x0040_b780)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                // Aligned: take a packet and fill it in.
                c.cycles += 6;
                c.g[A0][0] = sx(MRPC_POOL);
                c.g[RA][0] = sx(0x0040_b794);
                callee::MRPC_GET_PACKET.call(c, 0x0040_b794)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(frame.at(c, 0xc0));
                if c.g[S0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[S7][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S6][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S3][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S5][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    if c.g[S4][0] != c.g[S5][0] {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S4][0];
                        if (c.g[S3][0] as i64) > 0 {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0040_b80c);
                            c.g[A1][0] = c.g[S3][0];
                            callee::WRITEBACK.call(c, 0x0040_b80c)?;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S5][0];
                        if (c.g[S1][0] as i64) > 0 {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0040_b81c);
                            c.g[A1][0] = c.g[S1][0];
                            callee::WRITEBACK.call(c, 0x0040_b81c)?;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[FP][0] & 0x1;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x0040_b7f4);
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = c.g[S3][0];
                        }
                        callee::WRITEBACK.call(c, 0x0040_b7f4)?;
                        c.g[V0][0] = c.g[FP][0] & 0x1;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        // Synchronous completion: wait on a fresh sema.
                        c.g[S5][0] = 0x1;
                        c.cycles += 8;
                        c.sw(frame.at(c, 0x8), 0u64);
                        c.sw(frame.at(c, 0x4), c.g[S5][0]);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0040_b890);
                        c.sw(frame.at(c, 0x14), 0u64);
                        callee::CREATE_SEMA.call(c, 0x0040_b890)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                        if (c.g[S1][0] as i64) >= 0 {
                            c.cycles += 11;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[S5][0]);
                            c.g[A0][0] = 0xffff_ffff_8000_001a;
                            c.g[A3][0] = c.g[S4][0];
                            c.g[T1][0] = c.g[S3][0];
                            c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0040_b8d0);
                            c.g[A2][0] = 0x40;
                            callee::SEND_CMD.call(c, 0x0040_b8d0)?;
                            if c.g[V0][0] == 0 {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0040_b8e0);
                                c.g[A0][0] = c.g[S0][0];
                                callee::MRPC_FREE_PACKET.call(c, 0x0040_b8e0)?;
                                c.g[RA][0] = sx(0x0040_b8e8);
                                c.g[A0][0] = c.g[S1][0];
                                callee::DELETE_SEMA.call(c, 0x0040_b8e8)?;
                                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                            } else {
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x0040_b8f8);
                                c.g[A0][0] = c.g[S1][0];
                                callee::WAIT_SEMA.call(c, 0x0040_b8f8)?;
                                c.g[RA][0] = sx(0x0040_b900);
                                c.g[A0][0] = c.g[S1][0];
                                callee::DELETE_SEMA.call(c, 0x0040_b900)?;
                                c.g[V0][0] = 0u64;
                            }
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0040_b8a4);
                            c.g[A0][0] = c.g[S0][0];
                            callee::MRPC_FREE_PACKET.call(c, 0x0040_b8a4)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                        }
                    } else {
                        // Callback completion: flag the packet and send.
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        if c.g[S7][0] == 0 {
                            c.cycles += 2;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                        }
                        c.cycles += 12;
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        c.g[A0][0] = 0xffff_ffff_8000_001a;
                        c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                        c.g[A3][0] = c.g[S4][0];
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[T1][0] = c.g[S3][0];
                        c.g[A0][0] = c.g[A0][0] | 0x1a;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0040_b864);
                        c.g[A2][0] = 0x40;
                        callee::SEND_CMD.call(c, 0x0040_b864)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = 0u64;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0040_b874);
                            c.g[A0][0] = c.g[S0][0];
                            callee::MRPC_FREE_PACKET.call(c, 0x0040_b874)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        frame.restore_ra(c, 0xb0);
        c.g[FP][0] = c.ld(frame.at(c, 0xa0));
        c.g[S7][0] = c.ld(frame.at(c, 0x90));
        c.g[S6][0] = c.ld(frame.at(c, 0x80));
        c.g[S5][0] = c.ld(frame.at(c, 0x70));
        c.g[S4][0] = c.ld(frame.at(c, 0x60));
        c.g[S3][0] = c.ld(frame.at(c, 0x50));
        c.g[S2][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `int sceSifMTermRpc(client, mode)` (0x40b5b0): nonzero mode prints and
// returns -1. Otherwise waits on the client sema, takes a packet (none:
// -1), and sends the 0x1d terminate command. A failed send frees the
// packet, deletes the sema and returns -2; a sent command waits, deletes
// both semas and returns the client's return word.
pub fn sce_sif_mterm_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        if c.g[A1][0] == 0 {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0040_b5e8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            callee::WAIT_SEMA.call(c, 0x0040_b5e8)?;
            c.g[A0][0] = sx(MRPC_POOL);
            c.g[RA][0] = sx(0x0040_b5f4);
            callee::MRPC_GET_PACKET.call(c, 0x0040_b5f4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if c.g[S0][0] != 0 {
                c.cycles += 18;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = 0x1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sp(c) as u64;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw(frame.at(c, 0x4), c.g[A1][0]);
                c.sw(frame.at(c, 0x8), 0u64);
                c.g[RA][0] = sx(0x0040_b63c);
                c.sw(frame.at(c, 0x14), 0u64);
                callee::CREATE_SEMA.call(c, 0x0040_b63c)?;
                c.g[S2][0] = c.g[V0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                if (c.g[S2][0] as i64) >= 0 {
                    c.cycles += 10;
                    c.g[A0][0] = 0xffff_ffff_8000_001d;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[A0][0] | 0x1d;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[RA][0] = sx(0x0040_b678);
                    c.g[T1][0] = 0u64;
                    callee::SEND_CMD.call(c, 0x0040_b678)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0040_b688);
                        c.g[A0][0] = c.g[S0][0];
                        callee::MRPC_FREE_PACKET.call(c, 0x0040_b688)?;
                        c.g[RA][0] = sx(0x0040_b690);
                        c.g[A0][0] = c.g[S2][0];
                        callee::DELETE_SEMA.call(c, 0x0040_b690)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x0040_b6a0);
                        c.g[A0][0] = c.g[S2][0];
                        callee::WAIT_SEMA.call(c, 0x0040_b6a0)?;
                        c.g[RA][0] = sx(0x0040_b6a8);
                        c.g[A0][0] = c.g[S2][0];
                        callee::DELETE_SEMA.call(c, 0x0040_b6a8)?;
                        c.g[RA][0] = sx(0x0040_b6b0);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        callee::DELETE_SEMA.call(c, 0x0040_b6b0)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                    }
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_b650);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MRPC_FREE_PACKET.call(c, 0x0040_b650)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                }
            }
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0049_8178);
            c.g[RA][0] = sx(0x0040_b5d8);
            callee::PRINTF.call(c, 0x0040_b5d8)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x50);
        c.g[S2][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `int sceSifMBindRpc(client, rpc_id, mode, arg1, arg2, arg3)` (0x40b400):
// nonzero mode prints and returns -1. Otherwise creates the bind sema
// (-3 when exhausted), waits on it, takes a packet (none: deletes the
// sema, -1), creates the completion sema (-3 on exhaustion, freeing the
// packet and deleting the bind sema), and sends the 0x19 bind command.
// A failed send frees the packet, deletes both semas and returns -2; a
// sent command waits, deletes the completion sema, and deletes the bind
// sema too when the bind failed.
pub fn sce_sif_mbind_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.sd(frame.at(c, 0x80), c.g[S6][0]);
        c.sd(frame.at(c, 0x70), c.g[S5][0]);
        c.g[S6][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x60), c.g[S4][0]);
        c.g[S5][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x50), c.g[S3][0]);
        c.g[S4][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x90), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        if c.g[A2][0] == 0 {
            c.cycles += 8;
            c.g[S2][0] = 0x1;
            c.sw(frame.at(c, 0x14), 0u64);
            c.sw(frame.at(c, 0x8), c.g[S2][0]);
            c.g[A0][0] = sp(c) as u64;
            c.g[RA][0] = sx(0x0040_b468);
            c.sw(frame.at(c, 0x4), c.g[S2][0]);
            callee::CREATE_SEMA.call(c, 0x0040_b468)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0040_b480);
                c.g[A0][0] = c.g[V0][0];
                callee::WAIT_SEMA.call(c, 0x0040_b480)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                c.g[A0][0] = sx(MRPC_POOL);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[S4][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[S5][0]);
                c.g[RA][0] = sx(0x0040_b4a0);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[S6][0]);
                callee::MRPC_GET_PACKET.call(c, 0x0040_b4a0)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.cycles += 16;
                    c.g[A0][0] = sp(c) as u64;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S3][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S5][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S6][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
                    c.sw(frame.at(c, 0x4), c.g[S2][0]);
                    c.sw(frame.at(c, 0x8), 0u64);
                    c.g[RA][0] = sx(0x0040_b4f0);
                    c.sw(frame.at(c, 0x14), 0u64);
                    callee::CREATE_SEMA.call(c, 0x0040_b4f0)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                    if (c.g[S2][0] as i64) >= 0 {
                        c.cycles += 10;
                        c.g[A0][0] = 0xffff_ffff_8000_0019;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[A0][0] | 0x19;
                        c.g[A2][0] = 0x40;
                        c.g[A3][0] = 0u64;
                        c.g[T0][0] = 0u64;
                        c.g[RA][0] = sx(0x0040_b534);
                        c.g[T1][0] = 0u64;
                        callee::SEND_CMD.call(c, 0x0040_b534)?;
                        if c.g[V0][0] == 0 {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x0040_b544);
                            c.g[A0][0] = c.g[S0][0];
                            callee::MRPC_FREE_PACKET.call(c, 0x0040_b544)?;
                            c.g[RA][0] = sx(0x0040_b54c);
                            c.g[A0][0] = c.g[S2][0];
                            callee::DELETE_SEMA.call(c, 0x0040_b54c)?;
                            c.g[RA][0] = sx(0x0040_b554);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                            callee::DELETE_SEMA.call(c, 0x0040_b554)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0040_b564);
                            c.g[A0][0] = c.g[S2][0];
                            callee::WAIT_SEMA.call(c, 0x0040_b564)?;
                            c.g[RA][0] = sx(0x0040_b56c);
                            c.g[A0][0] = c.g[S2][0];
                            callee::DELETE_SEMA.call(c, 0x0040_b56c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[V0][0] = 0u64;
                            if c.g[V1][0] == 0 {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0040_b580);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                callee::DELETE_SEMA.call(c, 0x0040_b580)?;
                                c.g[V0][0] = 0u64;
                            }
                        }
                    } else {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0040_b504);
                        c.g[A0][0] = c.g[S0][0];
                        callee::MRPC_FREE_PACKET.call(c, 0x0040_b504)?;
                        c.g[RA][0] = sx(0x0040_b50c);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        callee::DELETE_SEMA.call(c, 0x0040_b50c)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                    }
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_b4b4);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    callee::DELETE_SEMA.call(c, 0x0040_b4b4)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
            }
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0049_8178);
            c.g[RA][0] = sx(0x0040_b448);
            callee::PRINTF.call(c, 0x0040_b448)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 10;
        frame.restore_ra(c, 0x90);
        c.g[S6][0] = c.ld(frame.at(c, 0x80));
        c.g[S5][0] = c.ld(frame.at(c, 0x70));
        c.g[S4][0] = c.ld(frame.at(c, 0x60));
        c.g[S3][0] = c.ld(frame.at(c, 0x50));
        c.g[S2][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `int sceSifMBindRpcDefault(client, rpc_id, mode)` (0x40b3d8): bind with
// the default arguments (arg1 0x800, arg2 0x2000, arg3 0x20).
pub fn sce_sif_mbind_rpc_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0x800;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x2000;
        c.g[RA][0] = sx(0x0040_b3f0);
        c.g[T1][0] = 0x20;
        callee::MBIND_RPC.call(c, 0x0040_b3f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `void sceSifMExitRpc(void)` (0x40b1f8): shut the SIF RPC manager down.
pub fn sce_sif_mexit_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = 0x450000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x33e0), 0u64);
        ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `void sceSifMInitRpc(int mode)` (0x40b130): start the SIF RPC manager
// (idempotent: returns at once when already up). Registers the 0x18
// command handler, announces the manager with a 0x1 broadcast, then
// spins until SREG 1 comes up (preset in tests so the loop exits).
pub fn sce_sif_minit_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x33e0));
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 33;
            c.g[S1][0] = 0x1;
            c.g[RA][0] = sx(0x0040_b158);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x33e0), c.g[S1][0]);
            callee::DINTR.call(c, 0x0040_b158)?;
            c.g[A3][0] = 0x530000;
            c.g[A2][0] = 0x20;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7040));
            c.g[S0][0] = 0x530000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7840));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.g[V0][0] = 0x20000000;
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x7040), c.g[S1][0]);
            c.g[V0][0] = c.g[S0][0] | c.g[V0][0];
            c.g[A1][0] = sx(0x0040_b2d0);
            c.g[A0][0] = 0xffff_ffff_8000_0000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A2][0] = c.g[V1][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A0][0] = c.g[A0][0] | 0x18;
            c.g[RA][0] = sx(0x0040_b1a0);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
            callee::ADD_CMD_HANDLER.call(c, 0x0040_b1a0)?;
            c.g[RA][0] = sx(0x0040_b1a8);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
            callee::EINTR.call(c, 0x0040_b1a8)?;
            c.g[A0][0] = 0xffff_ffff_8000_0000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
            c.g[A1][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.g[A0][0] = c.g[A0][0] | 0x1;
            c.g[A2][0] = 0x18;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[RA][0] = sx(0x0040_b1d0);
            c.g[T1][0] = 0u64;
            callee::SEND_CMD.call(c, 0x0040_b1d0)?;
            'l_wait: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0040_b1d8);
                c.g[A0][0] = 0x1;
                callee::GET_SREG.call(c, 0x0040_b1d8)?;
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_b1d0);
                    }
                    continue 'l_wait;
                } else {
                    break 'l_wait;
                }
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// The RPC manager's packet pool (`libmrpc_GetPacket`): a counter, the
/// packet array and its length.
pub(crate) const MRPC_POOL: u32 = 0x0052_8fc0;

pub fn libmrpc_free_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[V0][0] | 0xfffe;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        ret(c);
        Ok(())
    })
}

pub fn libmrpc_get_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_b220);
        c.g[S1][0] = c.g[A0][0];
        callee::DINTR.call(c, 0x0040_b220)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = 0u64;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'b_found: {
            if (c.g[A0][0] as i64) > 0 {
                c.cycles += 2;
                c.g[A1][0] = 0x1;
                'l_scan: loop {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10)) & 0x1;
                    if c.g[V0][0] == 0 {
                        // Free slot: mark it, bump the pool total, done.
                        c.cycles += 7;
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 16);
                        c.g[V0][0] = c.g[V0][0] | 0x5;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let wrapped = c.g[V1][0] == c.g[A1][0];
                        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                        if wrapped {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.g[V1][0] = 0x1;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        }
                        c.cycles += 5;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                        c.g[RA][0] = sx(0x0040_b27c);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        callee::EINTR.call(c, 0x0040_b27c)?;
                        c.g[V0][0] = c.g[S0][0];
                        break 'b_found;
                    }
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_b238);
                        }
                        continue 'l_scan;
                    } else {
                        break;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0040_b298);
            callee::EINTR.call(c, 0x0040_b298)?;
            c.g[V0][0] = 0u64;
            break 'b_found;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void libmrpc_RequestEnd(packet *reply)` (0x40b2d0): the SIF command
/// handler for the RPC manager's replies. By the reply's command id: a call
/// completion runs the client's end callback (with its argument) and wakes
/// the caller's sema, a bind completion copies the server's reply words
/// into the client, a terminate completion stores the result word; then the
/// owner's sema is signalled (when valid) and the packet freed.
pub fn libmrpc_request_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_8000_001a;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        let call = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        // The command id (v1) picks the completion: each known one copies
        // what its waiter needs and wakes it, then the owner (packet's
        // client +8) is signalled and the packet freed.
        'owner: {
            'reload: {
                'terminate: {
                    'bind: {
                        'call: {
                            if call {
                                break 'call;
                            }
                            'above: {
                                c.cycles += 2;
                                if c.g[V0][0] != 0 {
                                    break 'above;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = 0xffff_ffff_8000_0019;
                                let bind = c.g[V1][0] == c.g[V0][0];
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                if bind {
                                    break 'bind;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'owner;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0xffff_ffff_8000_001c;
                            let signal_only = c.g[V1][0] == c.g[V0][0];
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            if signal_only {
                                break 'reload;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0xffff_ffff_8000_001d;
                            if c.g[V1][0] == c.g[V0][0] {
                                break 'terminate;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'owner;
                        }
                        // Call completion: the end callback, then the waiter.
                        c.cycles += 4;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        if c.g[V0][0] != 0 {
                            c.cycles += 2;
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                            call_ptr(c, target, 0x0040_b358)?;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                        callee::ISIGNAL_SEMA.call(c, 0x0040_b360)?;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        break 'reload;
                    }
                    // Bind completion: the server's reply words, then the waiter.
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    callee::ISIGNAL_SEMA.call(c, 0x0040_b388)?;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    break 'reload;
                }
                // Terminate completion: the result word, then the waiter.
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                callee::ISIGNAL_SEMA.call(c, 0x0040_b3a0)?;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        }
        c.cycles += 2;
        if (c.g[A0][0] as i64) >= 0 {
            c.cycles += 2;
            callee::ISIGNAL_SEMA.call(c, 0x0040_b3b8)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        callee::MRPC_FREE_PACKET.call(c, 0x0040_b3c0)?;
        c.sw(c.g[S0][0] as u32, 0u64);
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod sif_cmd_table {

    /// Table pointer used for system commands (`cid < 0`, `0x0052_bce4`).
    pub(crate) const SYS_TABLE: u32 = 0x0052_bce4;

    /// Table pointer used for user commands (`cid >= 0`, `0x0052_bcec`).
    pub(crate) const USER_TABLE: u32 = 0x0052_bcec;
}

/// `int sceSifInitRpc(void)` (0x00426408): bring up SIF RPC. The first call
/// disables interrupts, runs `sceSifInitCmd`, registers the four command
/// handlers and re-enables interrupts; then it waits for the IOP side
/// (spins on SREG 0 until the peer answers) and records the SIF register.
/// Later calls return at once through the client-word early exit.
pub fn sce_sif_init_rpc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[RA][0] = sx(0x00426420);
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        DINTR.call(c, 0x00426420)?;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5180));
        c.g[S1][0] = 0x1;
        // The client word is set after the first bring-up: later calls
        // take the early exit (the `S1 = 1` runs on both paths).
        if c.g[V0][0] == 0u64 {
            c.cycles += 60;
            c.g[RA][0] = sx(0x00426450);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5180), c.g[S1][0]);
            EINTR.call(c, 0x00426450)?;
            c.g[RA][0] = sx(0x00426458);
            INIT_CMD.call(c, 0x00426458)?;
            c.g[RA][0] = sx(0x00426460);
            DINTR.call(c, 0x00426460)?;
            c.g[V1][0] = 0x530000;
            c.g[T0][0] = 0x530000;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4180));
            c.g[A2][0] = 0x530000;
            c.g[A3][0] = 0x530000;
            c.g[S0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x2980));
            c.g[V1][0] = 0x20;
            c.g[V0][0] = 0x20000000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3980));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3180));
            c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
            c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.g[V0][0] = c.g[S2][0] | c.g[V0][0];
            c.sw((c.g[T0][0] as u32).wrapping_sub(0x2980), c.g[S1][0]);
            c.g[A1][0] = 0x420000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
            c.g[A0][0] = 0xffffffff80000000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6708));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
            c.g[A0][0] = c.g[A0][0] | 0x8;
            c.g[A2][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[RA][0] = sx(0x004264d4);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
            ADD_CMD_HANDLER.call(c, 0x004264d4)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6a98));
            c.g[A0][0] = c.g[A0][0] | 0x9;
            c.g[RA][0] = sx(0x004264ec);
            c.g[A2][0] = c.g[S0][0];
            ADD_CMD_HANDLER.call(c, 0x004264ec)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c88));
            c.g[A0][0] = c.g[A0][0] | 0xa;
            c.g[RA][0] = sx(0x00426504);
            c.g[A2][0] = c.g[S0][0];
            ADD_CMD_HANDLER.call(c, 0x00426504)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6820));
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0042651c);
            c.g[A0][0] = c.g[A0][0] | 0xc;
            ADD_CMD_HANDLER.call(c, 0x0042651c)?;
            c.g[RA][0] = sx(0x00426524);
            EINTR.call(c, 0x00426524)?;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[RA][0] = sx(0x00426530);
            c.g[A0][0] = c.g[A0][0] | 0x2;
            GET_REG.call(c, 0x00426530)?;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            // A live SIF register means the peer answered: restore and
            // return (the delay-slot `RA` restore runs on both paths).
            if c.g[V0][0] != 0u64 {
                c.cycles += 5;
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
            // Otherwise send the init packet and wait for the peer.
            c.cycles += 10;
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = 0xffffffff80000000;
            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[S1][0]);
            c.g[A0][0] = c.g[A0][0] | 0x2;
            c.g[A2][0] = 0x10;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[RA][0] = sx(0x0042655c);
            c.g[T1][0] = 0u64;
            SEND_CMD.call(c, 0x0042655c)?;
            loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00426568);
                c.g[A0][0] = 0u64;
                GET_SREG.call(c, 0x00426568)?;
                let waiting = c.g[V0][0] == 0u64;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                if waiting {
                    if event_due(c) {
                        deliver_event_here(c, 0x00426560);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 8;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0x1;
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[A0][0] | 0x2;
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            // Tail call: SetReg returns straight to our caller.
            c.pc = 0x004241a0;
            (ee::s15::sceSifSetReg)(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        // Tail call: EIntr returns straight to our caller.
        c.pc = 0x0042c0c0;
        (ee::s15::EIntr)(c);
        return Ok(());
    });
}

/// `int sceSifStopDma(unsigned int id)` (0x424070): syscall 0x6b.
pub fn sce_sif_stop_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0x6b, 0x0042_4074, 0x0042_4078);
}

/// `int isceSifDmaStat(unsigned int id)` (0x424150): syscall -0x76 (0xffffff8a).
pub fn isce_sif_dma_stat(c: &mut Cpu) {
    syscall_wrapper(c, 0xffffff8a, 0x0042_4154, 0x0042_4158);
}

/// `int isceSifSetDma(SifDmaTransfer* t, int len)` (0x424170): syscall -0x77 (0xffffff89).
pub fn isce_sif_set_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0xffffff89, 0x0042_4174, 0x0042_4178);
}

/// `int sceSifSetDChain(void)` (0x424180): syscall 0x78.
pub fn sce_sif_set_dchain(c: &mut Cpu) {
    syscall_wrapper(c, 0x78, 0x0042_4184, 0x0042_4188);
}

/// `int sceSifSetReg(int reg, unsigned int val)` (0x4241a0): syscall 0x79.
pub fn sce_sif_set_reg(c: &mut Cpu) {
    syscall_wrapper(c, 0x79, 0x0042_41a4, 0x0042_41a8);
}

/// `unsigned int sceSifGetReg(int reg)` (0x4241b0): syscall 0x7a.
pub fn sce_sif_get_reg(c: &mut Cpu) {
    syscall_wrapper(c, 0x7a, 0x0042_41b4, 0x0042_41b8);
}

/// `int Deci2Call(...)` (0x4241d0): syscall 0x7c.
pub fn deci2_call(c: &mut Cpu) {
    syscall_wrapper(c, 0x7c, 0x0042_41d4, 0x0042_41d8);
}

/// `isceSifSendCmd` (0x4261f0): repack the SIF send registers around
/// `_sceSifSendCmd` (moves `a1` into the command slot, `a2`/`a3` down one).
pub fn isce_sif_send_cmd(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = c.g[A2][0];
        c.g[V1][0] = c.g[A3][0];
        c.g[T3][0] = c.g[T0][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[T2][0] = c.g[T1][0];
        c.g[A2][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[V1][0];
        c.g[T1][0] = c.g[T3][0];
        c.g[RA][0] = sx(0x00426220);
        c.g[A1][0] = 0x1;
        SEND_CMD_INNER.call(c, 0x00426220)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `_sceSifSendCmd` (0x00426078): build the SIF send packet at the caller's
/// buffer and kick the DMA off. Sizes in `0x10..=0x70` go inline (below
/// `0x10` and above `0x70` take the early exit with 0); the length word
/// decides between the packet fill, the empty store and the early exit; the
/// `S3` flag (argument `a1`) bit 0 picks the DMA start, bit 2 the write-back.
pub fn _sce_sif_send_cmd(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
        c.g[S1][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x10));
        c.g[S3][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[T0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x61u64) as u64;
        c.g[A1][0] = c.g[T2][0];
        // Small sizes go inline; anything bigger leaves through the early
        // exit with 0 (the delay-slot `A1 = T2` runs on both paths).
        if c.g[V0][0] == 0u64 {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            // Join `break 'b_42618c`: the shared restore below (one `RA`
            // load, matching the generated join).
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            c.cycles += 1;
        } else {
            if (c.g[A1][0] as i64) <= 0 {
                // Empty packet: clear the header words.
                c.cycles += 2 + 3;
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                // Generated `+1` block.
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
            } else {
                // Fill the packet header.
                c.cycles += 2 + 12;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                c.g[S2][0] = 0x1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.g[S3][0] & 0x4;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                // (the `A1 = S2 << 4` below runs unless the write-back arm
                // takes it first; the delay-slot store runs on both).
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                if c.g[V1][0] != 0u64 {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00426100);
                    WRITEBACK.call(c, 0x00426100)?;
                    // The write-back arm rejoins past the `+1` block.
                    c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                } else {
                    // Generated `+1` block.
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                }
            }
            // Commit the packet to the send area and kick the transfer.
            c.cycles += 23;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x4320));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[S1][0]);
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S4][0]);
            c.sb(c.g[S0][0] as u32, c.g[S1][0]);
            c.g[V0][0] = 0x44;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00426168);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            WRITEBACK.call(c, 0x00426168)?;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V0][0] == 0u64 {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0042618c);
                c.g[A0][0] = c.g[SP][0];
                SET_DMA.call(c, 0x0042618c)?;
                // Join `break 'b_42618c`: the shared restore below (one
                // `RA` load, matching the generated join).
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0042617c);
                c.g[A0][0] = c.g[SP][0];
                ISET_DMA.call(c, 0x0042617c)?;
                // Joins the epilogue directly (`break 'b_426190`).
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            }
        }
        c.cycles += 7;
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceSifExitRpc(void)` (0x4265a8): run `sceSifExitCmd`, then clear the
/// SIF RPC client word.
pub fn sce_sif_exit_rpc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x004265b8);
        EXIT_CMD.call(c, 0x004265b8)?;
        c.g[V0][0] = 0x450000;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5180), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceSifRemoveCmdHandler(int cid)` (0x00426050): clear the handler
/// slot for `cid` in the user (`cid >= 0`) or system table.
pub fn sce_sif_remove_cmd_handler(c: &mut Cpu) {
    use sif_cmd_table::*;
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
    if (c.g[A0][0] as i64) >= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        c.g[A0][0] = c.lw(USER_TABLE);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x530000;
        c.g[A0][0] = c.lw(SYS_TABLE);
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[V1][0] as u32, 0u64);
    c.pc = t;
}

/// `void sceSifCmd_ChangeAddrHandler(...)` (0x00425ce0): copy the address
/// word at `a0 + 0x10` to `a1 + 8`.
pub fn sce_sif_cmd_change_addr_handler(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.pc = t;
}

/// `int sceSifExitCmd(void)` (0x00425fb8): stop DMA channel 5, drop its
/// handler, and clear the SIF command state word.
pub fn sce_sif_exit_cmd(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x00425fc8);
        c.g[A0][0] = 0x5;
        DISABLE_DMAC.call(c, 0x00425fc8)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = 0x5;
        c.g[RA][0] = sx(0x00425fd8);
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x432c));
        REMOVE_DMAC_HANDLER.call(c, 0x00425fd8)?;
        c.g[V1][0] = 0x450000;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x517c), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void *sceSifRpc_GetFPacket(...)` (0x00426698): packet slot for the next
/// free packet: the remainder of the pool words + 1, scaled to a slot.
pub fn sce_sif_rpc_get_fpacket(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.div(0, 5, 3);
    // (both arms of the generated zero test fall through identically).
    c.cycles += 7;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[V0][0] = c.hi[0];
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.pc = t;
}

/// `void *sceSifRpc_GetFPacket2(...)` (0x004266c8): the packet at slot
/// `a1`, or the next free packet when `a1` is negative or past the count.
pub fn sce_sif_rpc_get_fpacket2(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 3;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        let negative = (c.g[A1][0] as i64) < 0;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let direct = if negative {
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                false
            } else {
                true
            }
        };
        if direct {
            c.cycles += 4;
            c.g[RA][0] = sx(0x004266ec);
            GET_FPACKET.call(c, 0x004266ec)?;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 6);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `sceSifRpcLoop` (0x004273f8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated twin.
pub fn sce_sif_rpc_loop(c: &mut Cpu) {
    if c.pc != 0x004273f8 {
        return ee::s15::f_004273f8__generated(c);
    }
    guest(c, |c| {
        use callee::*;

        let mut pc: u32 = 0x004273f8;
        loop {
            match pc {
                0x004273f8 => {
                    c.cycles += 5;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    let t = true;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        pc = 0x00427418;
                        continue;
                    }
                    pc = 0x0042740c;
                    continue;
                }
                0x0042740c => {
                    c.cycles += 1;
                    pc = 0x00427410;
                    continue;
                }
                0x00427410 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00427418);
                    EXEC_REQUEST.call(c, 0x00427418)?;
                    pc = 0x00427418;
                    continue;
                }
                0x00427418 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427420);
                    c.g[A0][0] = c.g[S0][0];
                    GET_NEXT_REQUEST.call(c, 0x00427420)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00427410) {
                            return Ok(());
                        }
                        pc = 0x00427410;
                        continue;
                    }
                    pc = 0x00427428;
                    continue;
                }
                0x00427428 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427430);
                    SLEEP_THREAD.call(c, 0x00427430)?;
                    let t = true;

                    if t {
                        if back_edge_unwinds(c, 0x00427418) {
                            return Ok(());
                        }
                        pc = 0x00427418;
                        continue;
                    }
                    c.pc = 0x00427438;
                    ee::s15::sceFs_FdSemaInit(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x004273f8, 0x00427438);
                    return Ok(());
                }
            }
        }
    });
}

/// `sceSifRpc_RetrySendAlarm(int id, u16 time, void *pkt)` (0x4267c0): the
/// alarm handler that re-sends the RPC packet `pkt` (in `a2`) through
/// `isceSifSendCmd` (command 0x80000008, 0x40 bytes, the destination words
/// at `pkt + 0x24..0x2c`), returning the send's result. When the send fails
/// it re-arms itself for 10 ticks with `iSetAlarm` (a tail call).
pub fn sce_sif_rpc_retry_send_alarm(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A0][0] = 0xffffffff80000000;
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        c.g[A0][0] = c.g[A0][0] | 0x8;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x40;
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
        ISCE_SEND_CMD.call(c, 0x004267f0)?;
        let t = c.g[V0][0] != 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
            c.cycles += 3;
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = t;
            return Ok(());
        }
        c.cycles += 7;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x420000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67c0));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[A0][0] = 0xa;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        // Tail call: `iSetAlarm` returns straight to our caller.
        c.pc = 0x00423b80;
        (ee::s15::f_00423b80)(c);
        Ok(())
    })
}

/// `int sceSifRpc_RequestRData(...)` (0x426820): build the reply packet for
/// an RPC request. Flag word bit 2 set: allocate from the free pool
/// (`GetFPacket2`, index `word >> 16`); clear: from the request's own
/// packet (`GetFPacket`). Then send through `isceSifSendCmd`; on failure
/// arm alarm 10 instead of returning the packet.
pub fn sce_sif_rpc_request_rdata(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_426864: {
            'b_426858: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[A0][0] = c.g[A1][0];
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[A1][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                if t {
                    break 'b_426858;
                }
                c.cycles += 4;
                c.g[A1][0] = sx(c.g[A1][0] as u32 >> 16);
                GET_FPACKET2.call(c, 0x00426850)?;
                let t = true;
                c.g[S0][0] = c.g[V0][0];
                if t {
                    break 'b_426864;
                } else {
                    break 'b_426858;
                }
            }
            c.cycles += 3;
            GET_FPACKET.call(c, 0x00426860)?;
            c.g[S0][0] = c.g[V0][0];
            break 'b_426864;
        }
        c.cycles += 23;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = 0xffffffff80000000;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.g[V0][0] | 0xc;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A0][0] = 0xffffffff80000000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A1][0]);
        c.g[A0][0] = c.g[A0][0] | 0x8;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x40;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
        c.g[T1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        ISCE_SEND_CMD.call(c, 0x004268b8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
            c.cycles += 4;
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            return Ok(());
        }
        c.cycles += 8;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x420000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67c0));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = 0xa;
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        // Tail call: `iSetAlarm` returns straight to our caller.
        c.pc = 0x00423b80;
        (ee::s15::f_00423b80)(c);
        Ok(())
    })
}

/// `int sceSifGetOtherData(...)` (0x4268f0): fetch an RPC reply packet
/// (`GetPacket` from the fixed SIF pool), fill in the caller's words and
/// send the request (`SendCmd`). With no flag bit 0, wait on a fresh
/// semaphore (`CreateSema`/`WaitSema`/`DeleteSema`); a failed send frees
/// the packet and reports -2, a failed create -3, no packet -1.
pub fn sce_sif_get_other_data(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_426a24: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
            c.g[A0][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.g[S4][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S5][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
            c.g[S2][0] = c.g[T0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2980));
            OC698_SCE_SIF_RPC_GET_PACKET.call(c, 0x00426930)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_426a24;
            }
            'b_4269e8: {
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.g[S2][0] & 0x1;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S3][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S4][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S5][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
                if t {
                    break 'b_4269e8;
                }
                'b_426990: {
                    c.cycles += 7;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.g[SP][0];
                    CREATE_SEMA.call(c, 0x00426978)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        break 'b_426990;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426988)?;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffffd;
                    if t {
                        break 'b_426a24;
                    } else {
                        break 'b_426990;
                    }
                }
                'b_4269d0: {
                    c.cycles += 10;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[A0][0] | 0xc;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    SEND_CMD.call(c, 0x004269b0)?;
                    let t = c.g[V0][0] != 0u64;
                    if t {
                        break 'b_4269d0;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x004269c0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    DELETE_SEMA.call(c, 0x004269c8)?;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if t {
                        break 'b_426a24;
                    } else {
                        break 'b_4269d0;
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                WAIT_SEMA.call(c, 0x004269d8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                DELETE_SEMA.call(c, 0x004269e0)?;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_426a24;
                } else {
                    break 'b_4269e8;
                }
            }
            c.cycles += 12;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[A0][0] = 0xffffffff80000000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[A0][0] = c.g[A0][0] | 0xc;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x40;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0u64;
            SEND_CMD.call(c, 0x00426a10)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_426a24;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426a20)?;
            c.g[V0][0] = 0xfffffffffffffffe;
            break 'b_426a24;
        }
        c.cycles += 9;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `void *sceSifRpc_SearchSvdata(...)` (0x426a48): scan the server-data
/// buckets for the entry whose id matches `a0`; null when no bucket or no
/// entry matches. Both list walks check for due events on their back edge.
pub fn sce_sif_rpc_search_svdata(c: &mut Cpu) {
    guest(c, |c| {
        'b_426a8c: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28));
            let t = c.g[A1][0] == 0u64;
            if t {
                break 'b_426a8c;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            'l_426a58: loop {
                'b_426a84: {
                    c.cycles += 2;
                    if c.g[V1][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                        break 'b_426a84;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    'l_426a68: loop {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A0][0] {
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.cycles += 2;
                            if c.g[V1][0] != 0u64 {
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x00426a68);
                                }
                                continue 'l_426a68;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                            break 'b_426a84;
                        }
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = c.g[V1][0];
                        c.pc = t;
                        return Ok(());
                    }
                }
                c.cycles += 2;
                if c.g[A1][0] != 0u64 {
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if event_due(c) {
                        deliver_event_here(c, 0x00426a58);
                    }
                    continue 'l_426a58;
                }
                break 'b_426a8c;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0u64;
        c.pc = t;
        Ok(())
    })
}

/// `int sceSifRpc_RequestBind(...)` (0x426a98): allocate a bind packet
/// (`GetFPacket`), look the server data up (`SearchSvdata`) and, when
/// found, copy its words into the packet and send it (`isceSifSendCmd`,
/// tail call). Without a match, clear the server words instead.
pub fn sce_sif_rpc_request_bind(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_426b14: {
            'b_426b04: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[S1][0] = c.g[A1][0];
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[A0][0] = c.g[S1][0];
                GET_FPACKET.call(c, 0x00426abc)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = 0xffffffff80000000;
                c.g[V0][0] = c.g[V0][0] | 0x9;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                SEARCH_SVDATA.call(c, 0x00426ae8)?;
                c.g[V1][0] = c.g[V0][0];
                if c.g[V1][0] != 0u64 {
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    break 'b_426b04;
                }
                c.cycles += 4;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x24), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x28), 0u64);
                let t = true;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), 0u64);
                if t {
                    break 'b_426b14;
                } else {
                    break 'b_426b04;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
            break 'b_426b14;
        }
        c.cycles += 13;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = 0xffffffff80000000;
        c.g[A0][0] = c.g[A0][0] | 0x8;
        c.g[A2][0] = 0x40;
        c.g[A3][0] = 0u64;
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        // Tail call: `isceSifSendCmd` returns straight to our caller.
        c.pc = 0x004261f0;
        (ee::s15::f_004261f0)(c);
        Ok(())
    })
}

/// `sceSifRpc_RequestCall(void *pkt)` (0x426c88): append the server data
/// `sd = [pkt + 0x34]` to its queue `q = [sd + 0x40]` (head `q + 0xc` when
/// empty, else linked after the tail `q + 0x10`, which it becomes), copy
/// the call's words from the packet into `sd`, then wake the queue's
/// thread (`iWakeupThread`, a tail call) unless its id (`q + 0`) is
/// negative or its wake word (`q + 4`) is already set.
pub fn sce_sif_rpc_request_call(c: &mut Cpu) {
    guest(c, |c| {
        'b_426ca8: {
            'b_426ca4: {
                c.cycles += 5;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                    break 'b_426ca4;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                if t {
                    break 'b_426ca8;
                } else {
                    break 'b_426ca4;
                }
            }
            c.cycles += 1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
            break 'b_426ca8;
        }
        'b_426d0c: {
            c.cycles += 20;
            c.sw((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2c));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x34), c.g[V1][0]);
            c.g[A0][0] = c.lw(c.g[A2][0] as u32);
            let t = (c.g[A0][0] as i64) < 0;
            if t {
                break 'b_426d0c;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] != 0u64;
            if t {
                break 'b_426d0c;
            }
            c.cycles += 2;
            // Tail return through `iWakeupThread` (no return check: the
            // function ends wherever the wakeup leaves `pc`).
            c.pc = 0x00424c50;
            (ee::s15::f_00424c50)(c);
            return Ok(());
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `int sceSifGetNextRequest(...)` (0x4271d8): under `DIntr`, take the
/// next queued request off the server entry (marking it taken), or report
/// the entry empty. The interrupt state is restored (`EIntr`) either way.
pub fn sce_sif_get_next_request(c: &mut Cpu) {
    guest(c, |c| {
        'b_427210: {
            'b_427204: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                callee::DINTR.call(c, 0x004271f0)?;
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if c.g[S1][0] != 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                    break 'b_427204;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                if t {
                    break 'b_427210;
                } else {
                    break 'b_427204;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            break 'b_427210;
        }
        c.cycles += 8;
        callee::EINTR.call(c, 0x00427218)?;
        c.g[V0][0] = c.g[S1][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `sceSifExecRequest(void *sd)` (0x427230): run an RPC request: call the
/// server function (`sd + 4`) with the caches written back around it, take
/// a reply packet (`GetFPacket2` when the flag word's bit 2 is set, else
/// `GetFPacket`), then answer: with a reply count (`sd + 0x30`) through
/// `sceSifSendCmd`, retried until it succeeds; without one by
/// `sceSifSetDma` of the reply descriptors, waiting out a 0x100000-step
/// countdown and retrying while the start fails. Both retry loops check
/// for due events on their back edges.
pub fn sce_sif_exec_request(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_427274: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[S4][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = c.g[V0][0] as u32;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            call_ptr(c, t, 0x00427268)?;
            c.g[S2][0] = c.g[V0][0];
            if c.g[S2][0] != 0u64 {
                c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                break 'b_427274;
            } else {
                break 'b_427274;
            }
        }
        'b_427288: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            let t = (c.g[A1][0] as i64) <= 0;
            if t {
                break 'b_427288;
            }
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            WRITEBACK.call(c, 0x00427288)?;
            break 'b_427288;
        }
        'b_427298: {
            c.cycles += 2;
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[A0][0] = c.g[S2][0];
            if t {
                break 'b_427298;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S4][0];
            WRITEBACK.call(c, 0x00427298)?;
            break 'b_427298;
        }
        'b_4272d4: {
            'b_4272c4: {
                c.cycles += 6;
                DINTR.call(c, 0x004272a0)?;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[A1][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = sx(c.g[A1][0] as u32 >> 16);
                if t {
                    break 'b_4272c4;
                }
                c.cycles += 5;
                c.g[A0][0] = 0x530000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2980));
                GET_FPACKET2.call(c, 0x004272bc)?;
                let t = true;
                c.g[S0][0] = c.g[V0][0];
                if t {
                    break 'b_4272d4;
                } else {
                    break 'b_4272c4;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = 0x530000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2980));
            GET_FPACKET.call(c, 0x004272d0)?;
            c.g[S0][0] = c.g[V0][0];
            break 'b_4272d4;
        }
        'b_4273dc: {
            'b_427330: {
                c.cycles += 10;
                EINTR.call(c, 0x004272dc)?;
                c.g[V1][0] = 0xffffffff80000000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[V1][0] = c.g[V1][0] | 0xa;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                if c.g[V0][0] == 0u64 {
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_427330;
                }
                c.cycles += 1;
                'l_427300: loop {
                    c.cycles += 10;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    c.g[A0][0] = c.g[A0][0] | 0x8;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S4][0];
                    SEND_CMD.call(c, 0x00427320)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00427300);
                        }
                        continue 'l_427300;
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_4273dc;
                    } else {
                        break 'b_427330;
                    }
                }
            }
            'b_427354: {
                c.cycles += 3;
                c.g[S3][0] = 0u64;
                let t = (c.g[S4][0] as i64) <= 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                if t {
                    break 'b_427354;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                c.g[S3][0] = 0x1;
                c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S4][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                break 'b_427354;
            }
            c.cycles += 17;
            c.g[A1][0] = sx((c.g[S3][0] as u32) << 4);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = 0x40;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = c.g[SP][0];
            'l_427398: loop {
                'b_4273d8: {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S3][0];
                    SET_DMA.call(c, 0x004273a0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    if t {
                        break 'b_4273d8;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x100000;
                    c.g[A0][0] = 0xffffffffffffffff;
                    'l_4273b0: loop {
                        c.cycles += 7;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        let t = c.g[V1][0] != c.g[A0][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x004273b0);
                            }
                            continue 'l_4273b0;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.g[SP][0];
                            if event_due(c) {
                                deliver_event_here(c, 0x00427398);
                            }
                            continue 'l_427398;
                        }
                        c.cycles += 1;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                        break 'b_4273d8;
                    }
                }
                c.cycles += 1;
                c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_4273dc;
            }
        }
        c.cycles += 6;
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    })
}

/// `int sceSifSetRpcQueue(...)` (0x426f48): under `DIntr`, park the queue
/// `a0` in the SIF pool (fixed head word): empty head takes it directly,
/// otherwise append after the last queued entry (the walk checks for due
/// events). The interrupt state is restored by the `EIntr` tail call.
pub fn sce_sif_set_rpc_queue(c: &mut Cpu) {
    guest(c, |c| {
        'b_426fc8: {
            'b_426f94: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[S0][0] = c.g[A1][0];
                callee::DINTR.call(c, 0x00426f64)?;
                c.g[V1][0] = 0x530000;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2980));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
                let t = c.g[V0][0] != 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
                if t {
                    break 'b_426f94;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[S1][0]);
                if t {
                    break 'b_426fc8;
                } else {
                    break 'b_426f94;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
            if c.g[V1][0] == 0u64 {
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                break 'b_426fc8;
            }
            c.cycles += 1;
            'l_426fa8: loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V0][0] != 0u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00426fa8);
                    }
                    continue 'l_426fa8;
                }
                c.cycles += 1;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                break 'b_426fc8;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        // Tail call: `EIntr` returns straight to our caller.
        c.pc = 0x0042c0c0;
        (ee::s15::f_0042c0c0)(c);
        Ok(())
    })
}

/// `int sceSifRemoveRpc(...)` (0x4270b0): under `DIntr`, unlink the RPC
/// entry `a0` from its server's list: a head match drops it directly,
/// otherwise the list is walked (due events checked) for the entry whose
/// successor is `a0`, and the successor is skipped over.
pub fn sce_sif_remove_rpc(c: &mut Cpu) {
    guest(c, |c| {
        'b_427120: {
            'b_4270e8: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.g[S2][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                callee::DINTR.call(c, 0x004270d0)?;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = c.g[S0][0] != c.g[S1][0];
                if t {
                    break 'b_4270e8;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                let t = true;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                if t {
                    break 'b_427120;
                } else {
                    break 'b_4270e8;
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            if t {
                break 'b_427120;
            }
            'b_42711c: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                if c.g[V1][0] == c.g[S1][0] {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    break 'b_42711c;
                }
                c.cycles += 1;
                'l_427100: loop {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let t = c.g[S0][0] == 0u64;
                    if t {
                        break 'b_427120;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[V1][0] = c.g[V0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00427100);
                        }
                        continue 'l_427100;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    break 'b_42711c;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
            break 'b_427120;
        }
        c.cycles += 9;
        callee::EINTR.call(c, 0x00427128)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int sceSifRemoveRpcQueue(...)` (0x427148): under `DIntr`, drop the
/// queue `a0` from the SIF pool's queue list (head or walked-to entry,
/// with due-event checks); the interrupt state is restored after.
pub fn sce_sif_remove_rpc_queue(c: &mut Cpu) {
    guest(c, |c| {
        'b_4271b8: {
            'b_427180: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                callee::DINTR.call(c, 0x00427160)?;
                c.g[V1][0] = 0x530000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2980));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                let t = c.g[S0][0] != c.g[S1][0];
                if t {
                    break 'b_427180;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                let t = true;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                if t {
                    break 'b_4271b8;
                } else {
                    break 'b_427180;
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            if t {
                break 'b_4271b8;
            }
            'b_4271b4: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                if c.g[V1][0] == c.g[S1][0] {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_4271b4;
                }
                c.cycles += 1;
                'l_427198: loop {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let t = c.g[S0][0] == 0u64;
                    if t {
                        break 'b_4271b8;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[V1][0] = c.g[V0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00427198);
                        }
                        continue 'l_427198;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_4271b4;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            break 'b_4271b8;
        }
        c.cycles += 8;
        callee::EINTR.call(c, 0x004271c0)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int sceSifInitCmd(...)` (0x425d38): bring up the SIF command path.
/// When the init flag is clear, zero the command tables, enable
/// interrupts, flush the cache, install the DMA chain and handlers, and
/// send the init packet (polling `GetReg` until the peer answers); when
/// set, just restore the interrupt state. The table/poll loops check for
/// due events on their back edges.
pub fn sce_sif_init_cmd(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        DINTR.call(c, 0x00425d58)?;
        c.g[T2][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x517c));
        let t = c.g[V0][0] == 0u64;
        c.g[S3][0] = 0x530000;
        if t {
            c.cycles += 26;
            c.g[A1][0] = 0x530000;
            c.g[V0][0] = 0x20000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4380));
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4400));
            c.g[S2][0] = 0x530000;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
            c.g[V1][0] = 0x1;
            c.g[T1][0] = 0x530000;
            c.g[A0][0] = 0x530000;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4328));
            c.sw((c.g[T2][0] as u32).wrapping_add(0x517c), c.g[V1][0]);
            c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4300));
            c.sw((c.g[S2][0] as u32).wrapping_sub(0x4328), c.g[A2][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4200));
            c.g[A3][0] = 0x20;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.g[V1][0] = c.g[T0][0];
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.g[S4][0] = 0x530000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.g[S0][0] = 0x1f;
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
            'l_425df0: loop {
                c.cycles += 7;
                c.sw(c.g[V1][0] as u32, 0u64);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.sw(c.g[V1][0] as u32 + 0x4, 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                let t = (c.g[S0][0] as i64) >= 0;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00425df0);
                    }
                    continue 'l_425df0;
                }
                break 'l_425df0;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x530000;
            c.g[S0][0] = 0x1f;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4200));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7c));
            'l_425e20: loop {
                c.cycles += 7;
                c.sw(c.g[V0][0] as u32, 0u64);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                let t = (c.g[S0][0] as i64) >= 0;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00425e20);
                    }
                    continue 'l_425e20;
                }
                break 'l_425e20;
            }
            'b_425e98: {
                c.cycles += 20;
                c.g[V0][0] = 0x420000;
                c.g[V1][0] = 0x420000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ce0));
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4300));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5cc0));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4328));
                c.sw((c.g[T1][0] as u32).wrapping_sub(0x4300), c.g[V0][0]);
                c.g[S0][0] = 0x20;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[S1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                EINTR.call(c, 0x00425e6c)?;
                c.g[A0][0] = 0u64;
                FLUSH_CACHE.call(c, 0x00425e74)?;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0xe010;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x20;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_425e98;
                }
                c.cycles += 3;
                c.g[AT][0] = 0x10010000;
                c.sw((c.g[AT][0] as u32).wrapping_sub(0x1ff0), c.g[S0][0]);
                c.g[V0][0] = 0x10000000;
                break 'b_425e98;
            }
            'b_425eb8: {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] | 0xc000;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x100;
                let t = c.g[V1][0] != 0u64;
                c.g[A1][0] = 0x420000;
                if t {
                    break 'b_425eb8;
                }
                c.cycles += 3;
                SCE_SET_DCHAIN.call(c, 0x00425eb4)?;
                c.g[A1][0] = 0x420000;
                break 'b_425eb8;
            }
            c.cycles += 13;
            c.g[A0][0] = 0x5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6230));
            c.g[A2][0] = 0u64;
            ADD_DMAC_HANDLER.call(c, 0x00425ec8)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x5;
            c.sw((c.g[V1][0] as u32).wrapping_sub(0x432c), c.g[V0][0]);
            ENABLE_DMAC.call(c, 0x00425ed8)?;
            c.g[A0][0] = 0xffffffff80000000;
            GET_REG.call(c, 0x00425ee4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            if t {
                c.cycles += 1;
                c.g[S0][0] = 0x20000;
                'l_425f30: loop {
                    c.cycles += 5;
                    c.g[A0][0] = 0x4;
                    GET_REG.call(c, 0x00425f38)?;
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x2;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00425f30);
                        }
                        continue 'l_425f30;
                    }
                    break 'l_425f30;
                }
                c.cycles += 29;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4328));
                GET_REG.call(c, 0x00425f4c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = 0xffffffff80000000;
                c.g[A1][0] = c.g[V0][0];
                SET_REG.call(c, 0x00425f5c)?;
                c.g[A0][0] = 0xffffffff80000000;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[A0][0] | 0x1;
                SET_REG.call(c, 0x00425f6c)?;
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4340));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4400));
                c.g[A0][0] = 0xffffffff80000000;
                c.g[A1][0] = c.g[V1][0];
                c.g[A2][0] = 0x14;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[A0][0] | 0x2;
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                // Tail call: `SendCmd` returns straight to our caller.
                c.pc = 0x004261b0;
                (ee::s15::f_004261b0)(c);
                return Ok(());
            }
            c.cycles += 16;
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4340));
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4400));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0xffffffff80000000;
            c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = 0x14;
            c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[T0][0] = 0u64;
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[T1][0] = 0u64;
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
            // Tail call: `SendCmd` returns straight to our caller.
            c.pc = 0x004261b0;
            (ee::s15::f_004261b0)(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        // Tail call: `EIntr` returns straight to our caller.
        c.pc = 0x0042c0c0;
        (ee::s15::f_0042c0c0)(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::driven;

    /// `sceSifGetSreg` result address for SREG 1 (`sceSifMInitRpc` spins on
    /// it): `sceSifGetSreg(n)` returns `[0x52be00 + 4*n]`, so SREG 1 is
    /// `[0x52be04]` (not `0x52bec4`, which is SREG 49 and never polled here).
    pub(crate) const SREG1: u32 = 0x0052_be04;

    /// The SIF RPC packet pool `sceSifCallRpc` and `sceSifBindRpc` take packets
    /// from (`sceSifRpc_GetPacket`): a counter, the packet array and its length.
    pub(crate) const SIF_RPC_POOL: u32 = 0x0052_d680;

    /// [`SIF_RPC_POOL`]'s length (0 in a test machine: the calls fail at once).
    pub(crate) const SIF_RPC_POOL_COUNT: u32 = 0x0052_d688;

    /// SIF RPC manager state word: nonzero while `sceSifMInitRpc` is active
    /// (cleared by `sceSifMExitRpc`).
    pub(crate) const SIF_INIT: u32 = 0x0045_33e0;

    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

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

    /// Query with no free SIF packet: the real CallRpc finds no packet and
    /// returns -1 normally, so query_heap takes the RPC-failure arm
    /// (`V0 = 0xffff0000 | 0xffff`, i.e. -1). Pins that arm differentially;
    /// the `| with ^` mutant there is value-identical (disjoint consts).
    #[test]
    fn sce_sif_query_mem_size_rpc_failure_matches() {
        check(
            ee::s15::f_0042ae18__generated,
            sce_sif_query_mem_size,
            SEEDS,
            &|c, seed| {
                // calls=0: no fake unwinds anywhere on this path.
                machine(c, seed, 121408, 0);
                c.pc = 0x0042_ae18;
                // Gate word clear: even-seed path takes the query (CALL_RPC)
                // branch rather than the early V0=0 exit.
                c.bus.write32(0x0045_5224, 0);
                c.bus.write32(0x0052_ef00, 0x7a00_7a00);
                // Packet table present but with zero packets: GetPacket
                // claims nothing, CallRpc returns -1 without unwinding.
                c.bus.write32(0x0052_d680, 0);
                c.bus.write32(0x0052_d684, DATA + 0x700);
                c.bus.write32(0x0052_d688, 0);
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
    use super::ee;
    use crate::difftest::SP;

    /// Client struct for the SIF manager tests (`sceSifMCallRpc` et al.).
    pub(crate) const SIF_CLIENT: u32 = DATA + 0x100;

    /// MCallRpc: nonzero mode (-1); mode 0 with a misaligned send or
    /// receive buffer (-1), or aligned buffers (equal or not, sizes zero or
    /// not) with an empty pool (-1) or a free packet, which is filled in and
    /// sent synchronously: the completion wait blocks (no IOP answers), or,
    /// on the guest thread, returns at once (the IOP answered, 0) or the
    /// send fails (DMA queue full: -2, the packet freed), or its `CreateSema`
    /// fails (on the guest thread: -3, the packet freed). The callback arm
    /// (mode bit 0) cannot follow the mode-0 check and is not reached.
    #[test]
    fn sce_sif_mcall_rpc_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b6d0);
            c.pc = 0x0040_b6d0;
            for i in 0..16 {
                c.bus.write32(SIF_CLIENT + i * 4, 0);
            }
            c.bus.write32(SIF_CLIENT + 0x28, 0x1234);
            c.bus.write32(SIF_CLIENT + 0x14, (DATA + 0x80) as u32);
            c.g[A0][0] = SIF_CLIENT as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            // Ninth stack argument (end_param) above the 0xc0 frame.
            c.bus.write32(SP, (DATA + 0x300) as u32);
            c.g[A2][0] = (seed % 2 * 3) as u64;
            let (send, recv) = [
                (DATA + 0x401, DATA + 0x500),
                (DATA + 0x400, DATA + 0x501),
                (DATA + 0x400, DATA + 0x500),
                (DATA + 0x440, DATA + 0x440),
            ][(seed / 2 % 4) as usize];
            c.g[A3][0] = send as u64;
            c.g[T1][0] = recv as u64;
            c.g[T0][0] = (seed / 16 % 2 * 0x80) as u64;
            c.g[T2][0] = (seed / 32 % 2 * 0x40) as u64;
            c.g[T3][0] = (seed / 64 % 2 * 0x0041_2345) as u64;
            if seed / 8 % 2 == 1 {
                // A free packet: the fill, writeback and send path.
                c.bus.write32(MRPC_POOL, seed / 64 % 2);
                c.bus.write32(MRPC_POOL + 4, SIF_PACKET);
                c.bus.write32(MRPC_POOL + 8, 1);
                packet_slots(c, SIF_PACKET, 1, &|_| false);
                match seed / 128 {
                    1 => answered_on_thread(c, DMA_GATE),
                    2 => answered_on_thread(c, WAIT_GATE),
                    3 => create_sema_fails_after(c, 0),
                    _ => {}
                }
            }
        };
        check(ee::s15::f_0040b6d0__generated, sce_sif_mcall_rpc, 512, call);
        require_paths(
            ee::s15::f_0040b6d0__generated,
            512,
            &[
                84, 148, 151, 156, 159, 174, 177, 181, 184, 207, 210, 236, 239, 244, 247, 259, 262,
                263, 265, 266, 267, 270, 271, 274, 278, 281, 285, 288, 289, 292, 301, 304, 305,
                307, 308, 327, 330, 331, 334, 1359, 1375, 2717, 2733, 2836, 2852,
            ],
            call,
        );
    }

    /// MTermRpc: nonzero mode (-1); empty pool after the wait (-1); a
    /// free packet: the terminate command is sent and the completion wait
    /// blocks (no IOP answers), or, on the guest thread, the wait returns
    /// at once (the IOP answered: both semas deleted, the client's return
    /// word) or the send fails (DMA queue full: -2, the packet freed). On
    /// seeds 120 and up, on the guest thread, its `CreateSema` fails (-3,
    /// the packet freed).
    #[test]
    fn sce_sif_mterm_rpc_matches() {
        let sif_mterm_rpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b5b0);
            c.pc = 0x0040_b5b0;
            for i in 0..16 {
                c.bus.write32(SIF_CLIENT + i * 4, 0);
            }
            c.bus.write32(SIF_CLIENT + 0x28, 0x1234);
            c.g[A0][0] = SIF_CLIENT as u64;
            c.g[A1][0] = (seed % 3 == 0) as u64;
            if seed % 3 == 1 {
                c.bus.write32(MRPC_POOL, 0);
                c.bus.write32(MRPC_POOL + 4, SIF_PACKET);
                c.bus.write32(MRPC_POOL + 8, 1);
                packet_slots(c, SIF_PACKET, 1, &|_| false);
                match seed / 3 % 3 {
                    1 => answered_on_thread(c, WAIT_GATE),
                    2 => answered_on_thread(c, DMA_GATE),
                    _ => {}
                }
            }
            if seed >= 120 {
                create_sema_fails_after(c, 0);
            }
        };
        check(
            ee::s15::f_0040b5b0__generated,
            sce_sif_mterm_rpc,
            240,
            sif_mterm_rpc_call,
        );
        require_paths(
            ee::s15::f_0040b5b0__generated,
            240,
            &[57, 64, 115, 210, 229, 231, 1340, 1356],
            sif_mterm_rpc_call,
        );
    }

    /// MBindRpc: nonzero mode (-1); sema creation is real; empty pool (on
    /// the guest thread: -1 after deleting the bind sema); a free packet:
    /// the bind command is sent and the completion wait blocks (no IOP
    /// answers), or, on the guest thread, the wait returns at once (the IOP
    /// answered: 0, both semas deleted) or the send fails (DMA queue full:
    /// -2, the packet freed, both semas deleted). On seeds 120 and up, on
    /// the guest thread, the bind sema's or the completion sema's
    /// `CreateSema` fails (-3; the second frees the packet and deletes the
    /// bind sema).
    #[test]
    fn sce_sif_mbind_rpc_matches() {
        let sif_mbind_rpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b400);
            c.pc = 0x0040_b400;
            for i in 0..16 {
                c.bus.write32(SIF_CLIENT + i * 4, 0);
            }
            c.g[A0][0] = SIF_CLIENT as u64;
            c.g[A1][0] = 0x2222;
            c.g[A2][0] = (seed % 3 == 0) as u64;
            c.g[A3][0] = 0x11;
            c.g[T0][0] = 0x22;
            c.g[T1][0] = 0x33;
            if seed % 3 == 1 {
                c.bus.write32(MRPC_POOL, 0);
                c.bus.write32(MRPC_POOL + 4, SIF_PACKET);
                c.bus.write32(MRPC_POOL + 8, 1);
                packet_slots(c, SIF_PACKET, 1, &|_| false);
                match seed / 3 % 3 {
                    1 => answered_on_thread(c, WAIT_GATE),
                    2 => answered_on_thread(c, DMA_GATE),
                    _ => {}
                }
            } else if seed % 3 == 2 {
                on_thread(c);
            }
            if seed >= 120 {
                create_sema_fails_after(c, seed / 3 % 2);
            }
        };
        check(
            ee::s15::f_0040b400__generated,
            sce_sif_mbind_rpc,
            240,
            sif_mbind_rpc_call,
        );
        require_paths(
            ee::s15::f_0040b400__generated,
            240,
            &[39, 101, 148, 233, 261, 262, 1352, 1368],
            sif_mbind_rpc_call,
        );
    }

    /// Gate words for [`fake_wait_sema`] and [`fake_set_dma`] (in the
    /// machine's RAM).
    const WAIT_GATE: u32 = DATA + 0x6200;
    const DMA_GATE: u32 = DATA + 0x6204;
    const GATE_MAGIC: u32 = 0x4d42_4e44;

    /// `WaitSema` with its gate set: the sema is already signalled (the
    /// IOP answered at once), so the wait returns its id without blocking.
    fn fake_wait_sema(c: &mut Cpu) {
        if c.bus.read32(WAIT_GATE) != GATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_wait_sema),
                ee::s15::f_00423de0__generated,
            );
        }
        c.g[V0][0] = sx(c.g[A0][0] as u32);
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `sceSifSetDma` with its gate set: the DMA queue is full, so it
    /// returns 0 and nothing is sent.
    fn fake_set_dma(c: &mut Cpu) {
        if c.bus.read32(DMA_GATE) != GATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_set_dma),
                ee::s15::f_00424160__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// Run on the main guest thread (a test machine has none), so the kernel
    /// calls that reschedule (`DeleteSema`) come back to the caller instead
    /// of the idle loop.
    fn on_thread(c: &mut Cpu) {
        let saved = (c.pc, c.g[A0], c.g[A1], c.g[RA]);
        ssx3_ps2::kernel::start_main_thread(c, saved.0, 0, 0);
        (c.pc, c.g[A0], c.g[A1], c.g[RA]) = saved;
    }

    /// Hook both fakes, open the gate at `gate`, and run on the main guest
    /// thread.
    fn answered_on_thread(c: &mut Cpu, gate: u32) {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            fake_wait_sema,
            crate::difftest::fakes::prev_cell(fake_wait_sema),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00424160),
            fake_set_dma,
            crate::difftest::fakes::prev_cell(fake_set_dma),
        );
        c.bus.write32(gate, GATE_MAGIC);
        on_thread(c);
    }

    /// Gate words for [`create_sema_fails`].
    const SEMA_FAIL_GATE: u32 = DATA + 0x6208;
    const SEMA_FAIL_SKIP: u32 = DATA + 0x620c;

    /// The syscall hook with its gate set: after `SEMA_FAIL_SKIP` more
    /// `CreateSema` calls (syscall 0x40), one fails with -1 (the kernel is
    /// out of semas); every other syscall goes to the machine's hook.
    fn create_sema_fails(c: &mut Cpu, at: u32) {
        if c.bus.read32(SEMA_FAIL_GATE) == GATE_MAGIC && c.g[V1][0] as u32 == 0x40 {
            let skip = c.bus.read32(SEMA_FAIL_SKIP);
            c.bus.write32(SEMA_FAIL_SKIP, skip.wrapping_sub(1));
            if skip == 0 {
                c.g[V0][0] = u64::MAX;
                return;
            }
        }
        (ssx3_ps2::platform::HOOKS.syscall)(c, at)
    }

    /// Make the `CreateSema` after `skip` successful ones fail, on the main
    /// guest thread.
    fn create_sema_fails_after(c: &mut Cpu, skip: u32) {
        c.hooks.syscall = create_sema_fails;
        c.bus.write32(SEMA_FAIL_GATE, GATE_MAGIC);
        c.bus.write32(SEMA_FAIL_SKIP, skip);
        on_thread(c);
    }

    /// MBindRpcDefault: forwards to the real `sceSifMBindRpc` with a
    /// nonzero mode, which prints and returns -1. The bind path itself
    /// is that function's test.
    #[test]
    fn sce_sif_mbind_rpc_default_matches() {
        let sif_mbind_rpc_default_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b3d8);
            c.pc = 0x0040_b3d8;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = 0x45;
            c.g[A2][0] = 1 + (seed % 3) as u64;
        };
        check(
            ee::s15::f_0040b3d8__generated,
            sce_sif_mbind_rpc_default,
            60,
            sif_mbind_rpc_default_call,
        );
        require_paths(
            ee::s15::f_0040b3d8__generated,
            60,
            &[1361],
            sif_mbind_rpc_default_call,
        );
    }

    /// MExitRpc: clears the init word and returns its old address. Fully
    /// covered (leaf).
    #[test]
    fn sce_sif_mexit_rpc_matches() {
        let sif_mexit_rpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b1f8);
            c.pc = 0x0040_b1f8;
            c.bus.write32(SIF_INIT, (seed % 2) as u32);
        };
        check(
            ee::s15::f_0040b1f8__generated,
            sce_sif_mexit_rpc,
            60,
            sif_mexit_rpc_call,
        );
        require_paths(ee::s15::f_0040b1f8__generated, 60, &[3], sif_mexit_rpc_call);
    }

    /// MInitRpc: already-up early return, and the full bring-up (SREG 1
    /// preset so the wait loop exits at once). Fully covered.
    #[test]
    fn sce_sif_minit_rpc_matches() {
        let sif_minit_rpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b130);
            c.pc = 0x0040_b130;
            if seed % 2 == 0 {
                c.bus.write32(SIF_INIT, 0x77);
            } else {
                c.bus.write32(SIF_INIT, 0);
                c.bus.write32(SREG1, 1);
            }
        };
        check(
            ee::s15::f_0040b130__generated,
            sce_sif_minit_rpc,
            60,
            sif_minit_rpc_call,
        );
        require_paths(
            ee::s15::f_0040b130__generated,
            60,
            &[12, 176],
            sif_minit_rpc_call,
        );
    }

    /// Request-owner struct for the `libmrpc_RequestEnd` test.
    pub(crate) const RPC_OWNER: u32 = DATA + 0x300;

    /// Freed packet for the `libmrpc_RequestEnd` tail.
    pub(crate) const RPC_PACKET: u32 = DATA + 0x500;

    /// Request struct for the `libmrpc_RequestEnd` test.
    pub(crate) const RPC_REQUEST: u32 = DATA + 0x200;

    /// Scratch packet used to back crafted `libmrpc_GetPacket` pools.
    pub(crate) const SIF_PACKET: u32 = DATA + 0x400;

    /// Write `len` bytes of packet slots at `base` (stride 0x40), with
    /// the used/free flag of slot `i` from `used(i)`.
    pub(crate) fn packet_slots(c: &mut Cpu, base: u32, len: u32, used: &dyn Fn(u32) -> bool) {
        for i in 0..len {
            for w in 0..16 {
                c.bus.write32(base + i * 0x40 + w * 4, 0);
            }
            if used(i) {
                c.bus.write32(base + i * 0x40 + 0x10, 1);
            }
            c.bus.write32(base + i * 0x40 + 0x18, 0x1000 + i);
        }
    }

    /// Common fixture: ELF image (format strings, tables), scrambled
    /// registers, game-like gp, the SIF RPC pool drained (the real
    /// `sceSifCallRpc` fails fast with -1), the MC client down, and the
    /// SIF packet pool zeroed. Each test raises what its seeds need.
    pub(crate) fn mrpc_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x0040_b6d0 ^ (seed as u64) << 12 ^ salt);
        elf_image(c);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(SIF_RPC_POOL_COUNT, 0);
        c.bus.write32(crate::game::lib::mc::LIBMC_CLIENT + 0x24, 0);
        c.bus.write32(MRPC_POOL, 0);
        c.bus.write32(MRPC_POOL + 4, 0);
        c.bus.write32(MRPC_POOL + 8, 0);
        r
    }

    /// FreePacket: random flag/owner words; clears the owner word and
    /// masks the flags word down to bit 0. Fully covered (leaf).
    #[test]
    fn libmrpc_free_packet_matches() {
        let mrpc_free_packet_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = mrpc_setup(c, seed, 0x0040_b2b0);
            c.pc = 0x0040_b2b0;
            c.g[A0][0] = RPC_PACKET as u64;
            c.bus.write32(RPC_PACKET + 0x10, r.next_u32());
            c.bus.write32(RPC_PACKET + 0x18, r.next_u32());
        };
        check(
            ee::s15::f_0040b2b0__generated,
            libmrpc_free_packet,
            200,
            mrpc_free_packet_call,
        );
        require_paths(
            ee::s15::f_0040b2b0__generated,
            200,
            &[7],
            mrpc_free_packet_call,
        );
    }

    /// GetPacket: empty pool, all-used pool (both return 0), and free
    /// slots at several positions with both pool-total arms. Fully
    /// covered (the event hook never fires in tests).
    #[test]
    fn libmrpc_get_packet_matches() {
        let mrpc_get_packet_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b208);
            c.pc = 0x0040_b208;
            let (total, base, count, used) = match seed % 6 {
                0 => (0u32, 0u32, 0u32, 0u32),
                1 => (0, SIF_PACKET, 1, 1),
                2 => (5, SIF_PACKET, 2, 2),
                3 => (0, SIF_PACKET, 3, 0),
                4 => (5, SIF_PACKET, 3, 1),
                _ => (0, SIF_PACKET, 3, 2),
            };
            c.bus.write32(DATA + 0x200, total);
            c.bus.write32(DATA + 0x204, base);
            c.bus.write32(DATA + 0x208, count);
            // `used` leading slots busy, the rest free.
            packet_slots(c, SIF_PACKET, 3, &|i| i < used);
            c.g[A0][0] = (DATA + 0x200) as u64;
        };
        check(
            ee::s15::f_0040b208__generated,
            libmrpc_get_packet,
            200,
            mrpc_get_packet_call,
        );
        require_paths(
            ee::s15::f_0040b208__generated,
            200,
            &[32, 41, 48, 50, 54, 64],
            mrpc_get_packet_call,
        );
    }

    /// RequestEnd: every command id (0x1a with and without the
    /// completion callback, 0x19, 0x1c, 0x1d, lower and higher
    /// leftovers), with the tail sema negative (skipped) and live-bogus
    /// (signalled, fails fast). Fully covered.
    #[test]
    fn libmrpc_request_end_matches() {
        let mrpc_request_end_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mrpc_setup(c, seed, 0x0040_b2d0);
            c.pc = 0x0040_b2d0;
            let cmd = [
                0x8000_001au32,
                0x8000_001a,
                0x8000_0019,
                0x8000_001c,
                0x8000_001d,
                0x8000_0010,
                0x8000_0020,
                0x5,
            ][seed as usize % 8];
            c.bus.write32(RPC_REQUEST + 0x20, cmd);
            c.bus.write32(RPC_REQUEST + 0x1c, RPC_OWNER);
            c.bus.write32(RPC_REQUEST + 0x24, 0xaaaa_0001);
            c.bus.write32(RPC_REQUEST + 0x28, 0xbbbb_0002);
            c.bus.write32(RPC_REQUEST + 0x2c, 0xcccc_0003);
            for w in [0x0u32, 0x8, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                if w != 0x1c || seed % 2 == 0 {
                    c.bus.write32(RPC_OWNER + w, 0);
                }
            }
            c.bus.write32(RPC_OWNER, RPC_PACKET);
            c.bus.write32(
                RPC_OWNER + 8,
                if seed % 3 == 2 { 0xffff_ffff } else { 0x1234 },
            );
            c.bus.write32(
                RPC_OWNER + 0x1c,
                if seed % 4 == 1 { 0x0014_06c0 } else { 0 },
            );
            c.bus.write32(RPC_OWNER + 0x20, (DATA + 0x600) as u32);
            c.bus.write32(RPC_OWNER + 0x28, 0x1234);
            for w in [0x10u32, 0x18] {
                c.bus.write32(RPC_PACKET + w, 0);
            }
            c.g[A0][0] = RPC_REQUEST as u64;
        };
        check(
            ee::s15::f_0040b2d0__generated,
            libmrpc_request_end,
            200,
            mrpc_request_end_call,
        );
        require_paths(
            ee::s15::f_0040b2d0__generated,
            200,
            &[34, 35, 39, 40, 41, 44, 45, 46, 47, 48, 50, 53, 54],
            mrpc_request_end_call,
        );
    }

    driven!(
        libmrpc_get_packet_gen_driven,
        ee::s15::f_0040b208__generated
    );
    driven!(libmrpc_get_packet_hand_driven, libmrpc_get_packet);

    /// Header `{total, base, count}` of the long packet pool.
    const LONG_POOL: u32 = DATA + 0x1204;

    /// Slots (stride 0x40) of the long packet pool.
    const LONG_POOL_SLOTS: u32 = DATA + 0x1440;

    /// Taking a SIF RPC packet from a pool whose leading slots are all
    /// busy: the scan steps over them with a live event, then takes the
    /// first free slot (last or second to last) or, none free, returns 0.
    #[test]
    fn libmrpc_get_packet_events() {
        check_events(
            libmrpc_get_packet_gen_driven,
            libmrpc_get_packet_hand_driven,
            144,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0040_b238);
                c.pc = 0x0040_b208;
                let count = 8 + 4 * (seed / 3 % 4);
                // At least six busy slots lead: the first back edge comes
                // about 34 cycles in, the sixth about 69.
                let busy = count - seed / 3 / 4 % 3;
                // Pool total zero (the skip-zero arm) or not.
                let total = if seed / 3 / 12 % 2 == 0 {
                    0
                } else {
                    0x0011_0005
                };
                c.bus.write32(LONG_POOL, total);
                c.bus.write32(LONG_POOL + 4, LONG_POOL_SLOTS);
                c.bus.write32(LONG_POOL + 8, count);
                for i in 0..count {
                    let slot = LONG_POOL_SLOTS + i * 0x40;
                    for w in 0..16u32 {
                        c.bus.write32(slot + w * 4, r.next_u32() | 0x0101_0101);
                    }
                    let flags = c.bus.read32(slot + 0x10);
                    c.bus.write32(
                        slot + 0x10,
                        if i < busy { flags | 1 } else { flags & !1 | 2 },
                    );
                }
                c.g[A0][0] = LONG_POOL as u64;
                schedule(c, &mut r, seed, 0x0040_b208)
            },
        );
    }

    // sceSifMInitRpc (0x40b130): no events test; the SREG 1 wait at
    // 0x40b1d0 cannot loop and still end in the harness (see the table).
    use super::sif_cmd_table::SYS_TABLE;
    use super::sif_cmd_table::USER_TABLE;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;

    /// Client word (`0x0045_5180`, RAM: the early exit is testable) and the
    /// SREG polled words (`0x0052_be00`, RAM but zeroed by `sceSifInitCmd`
    /// during bring-up, so the poll loop always spins in-harness).
    pub(crate) const SIF_CLIENT_WORD: u32 = 0x0045_5180;

    /// Common bring-up fixture: game-like machine, client word steered by
    /// `seed % 2` (nonzero: the early exit; zero: the full bring-up, which
    /// leaves with the peer live since the register read comes back
    /// nonzero). The send path needs the register fake (see
    /// `init_rpc_send_setup`).
    pub(crate) fn init_rpc_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = machine(c, seed, 0x0042_6408, 4);
        c.pc = 0x0042_6408;
        c.bus
            .write32(SIF_CLIENT_WORD, if seed % 2 == 0 { 0x77 } else { 0 });
        r
    }

    /// `sceSifInitRpc`: the early exit and the live-peer bring-up.
    #[test]
    fn sce_sif_init_rpc_matches() {
        covered(
            ee::s15::f_00426408__generated,
            sce_sif_init_rpc,
            SEEDS,
            &|c, seed| {
                init_rpc_setup(c, seed);
            },
        );
    }

    driven!(sce_sif_init_rpc_send_driven, sce_sif_init_rpc);
    driven!(
        sce_sif_init_rpc_send_generated_driven,
        ee::s15::f_00426408__generated
    );

    /// `sceSifInitRpc` send path with events due: deliveries in the
    /// bring-up and the SREG poll unwind and re-enter identically on both
    /// sides, and the loop still exits when the peer answers.
    #[test]
    fn sce_sif_init_rpc_send_events() {
        check_events(
            sce_sif_init_rpc_send_generated_driven,
            sce_sif_init_rpc_send_driven,
            SEEDS,
            &|c, seed| {
                let mut r = init_rpc_send_setup(c, seed);
                c.bus
                    .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
                schedule(c, &mut r, seed, 0x0042_6408)
            },
        );
    }

    /// Gate + countdown for the send-path fakes (per-machine words; other
    /// tests' machines read 0 here, so their runs fall back to the slot body).
    pub(crate) const SEND_GATE: u32 = DATA + 0x6230;
    pub(crate) const SEND_SREG_N: u32 = DATA + 0x6234;

    /// `sceSifGetReg` with the gate set and `ra` at the bring-up read
    /// (`0x0042_6530`): the peer has not answered yet, so the init packet
    /// is sent. Anywhere else (or with the gate clear): the slot body.
    fn fake_get_reg_send(c: &mut Cpu) {
        if c.bus.read32(SEND_GATE) != GATE_MAGIC || c.g[RA][0] as u32 != 0x0042_6530 {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_get_reg_send),
                ee::s15::f_004241b0__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `sceSifGetSreg` with the gate set: 0 while polls remain, then
    /// nonzero (the peer answers). The count lives in the machine's RAM,
    /// so both sides poll the same number of times.
    fn fake_get_sreg_answer(c: &mut Cpu) {
        if c.bus.read32(SEND_GATE) != GATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_get_sreg_answer),
                ee::s15::f_00425cf0__generated,
            );
        }
        let n = c.bus.read32(SEND_SREG_N);
        if n > 0 {
            c.bus.write32(SEND_SREG_N, n - 1);
            c.g[V0][0] = 0;
        } else {
            c.g[V0][0] = 1;
        }
        c.cycles += 6;
        crate::abi::ret(c);
    }

    /// Send-path bring-up: like [`init_rpc_setup`] but the client word is
    /// always clear and the register/sreg fakes steer into the send, poll
    /// the SREG loop `1 + seed % 3` times, then let the peer answer.
    pub(crate) fn init_rpc_send_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = init_rpc_setup(c, seed);
        c.bus.write32(SIF_CLIENT_WORD, 0);
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004241B0),
            fake_get_reg_send,
            crate::difftest::fakes::prev_cell(fake_get_reg_send),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00425CF0),
            fake_get_sreg_answer,
            crate::difftest::fakes::prev_cell(fake_get_sreg_answer),
        );
        c.bus.write32(SEND_GATE, GATE_MAGIC);
        c.bus.write32(SEND_SREG_N, 1 + seed % 3);
        r
    }

    /// `sceSifInitRpc` through the send path: the init packet goes out,
    /// the SREG loop spins until the peer answers, and the `SetReg` tail
    /// call returns to the caller.
    #[test]
    fn sce_sif_init_rpc_send_matches() {
        check(
            ee::s15::f_00426408__generated,
            sce_sif_init_rpc,
            SEEDS,
            &|c, seed| {
                init_rpc_send_setup(c, seed);
            },
        );
        require_paths(
            ee::s15::f_00426408__generated,
            SEEDS,
            // Three SREG poll counts (1-3 zero polls) crossed with the
            // syscall fake's per-seed charge in the real bring-up callees.
            &[624, 642, 665, 706, 1063, 1074, 1090],
            &|c, seed| {
                init_rpc_send_setup(c, seed);
            },
        );
    }

    #[test]
    fn sce_sif_stop_dma_matches() {
        syscall_test(ee::s15::f_00424070__generated, sce_sif_stop_dma, 1_321_001);
    }

    #[test]
    fn isce_sif_dma_stat_matches() {
        syscall_test(ee::s15::f_00424150__generated, isce_sif_dma_stat, 1_321_002);
    }

    #[test]
    fn isce_sif_set_dma_matches() {
        syscall_test(ee::s15::f_00424170__generated, isce_sif_set_dma, 1_321_003);
    }

    #[test]
    fn sce_sif_set_dchain_matches() {
        syscall_test(
            ee::s15::f_00424180__generated,
            sce_sif_set_dchain,
            1_321_004,
        );
    }

    #[test]
    fn sce_sif_set_reg_matches() {
        syscall_test(ee::s15::f_004241a0__generated, sce_sif_set_reg, 1_321_005);
    }

    #[test]
    fn sce_sif_get_reg_matches() {
        syscall_test(ee::s15::f_004241b0__generated, sce_sif_get_reg, 1_321_006);
    }

    #[test]
    fn deci2_call_matches() {
        syscall_test(ee::s15::f_004241d0__generated, deci2_call, 1_321_007);
    }

    /// `isceSifSendCmd`: repack the registers, run `_sceSifSendCmd`, restore.
    /// The wrapper moves `a1` into the command slot and `a2`/`a3` down one,
    /// and copies `t1` into the length slot (`t2`); the callee's length is
    /// the steered `t1`, while `t2` is scratch overwritten before the call.
    /// `A0` and `A3` name different buffers (nonzero low bytes), so an
    /// `A0`/`A3` mix-up in the repack shows. Arms: size below/equal/above
    /// the 0x71 limit; length positive, zero (empty arm) and a huge length
    /// (packet arm). The wrapper forces `a1 = 1`, so the write-back arm
    /// stays unreachable here (covered by the direct test).
    #[test]
    fn isce_sif_send_cmd_matches() {
        covered(
            ee::s15::f_004261f0__generated,
            isce_sif_send_cmd,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_321_009, 2);
                c.pc = 0x0042_61f0;
                c.g[A0][0] = (DATA + 0x310) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                // Size digit: 0x10 and 0x70 take the packet body, 0x71 and
                // 0x1000 take the early exit.
                c.g[A2][0] = [0x10, 0x70, 0x71, 0x1000][seed as usize % 4] as u64;
                c.g[A3][0] = (DATA + 0x320) as u64;
                c.g[T0][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
                // Length digit (what the callee sees): 12 and 8 build a
                // packet, 0 takes the empty arm, 0x00000000fffffffc is a
                // huge positive length (packet arm, not -1: only a properly
                // sign-extended -1 would be negative).
                c.g[T1][0] = [12, 8, 0, 0xffff_fffcu64][seed as usize / 4 % 4];
                // Scratch length slot: overwritten before the call, so a
                // missed copy would show as a different length.
                c.g[T2][0] = (seed.wrapping_mul(0x5bd1_e995) | 0x0202_0200) as u64;
                for i in 0..32 {
                    c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
                }
            },
        );
    }

    /// `_sceSifSendCmd`: size classes around the 0x71 inline limit; length
    /// classes (empty via 0 and a sign-extended -4, plain, write-back
    /// length); the
    /// `S3` flag (argument `a1`) bit 0 picks the DMA start, bit 2 the
    /// write-back. Every steering value from an independent digit: `A0`
    /// and `T0` name different buffers, and the flag (`A1`) can equal the
    /// length (`T2`) (both may be zero). (A zero-extended
    /// `0x00000000fffffffc` would be a huge positive length and explode
    /// the write-back range; only a properly sign-extended -4 takes the
    /// empty arm.)
    #[test]
    fn _sce_sif_send_cmd_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_320_001, 4);
            c.pc = 0x0042_6078;
            // Size digit: packet body (0x10, 0x70), early exit (0x71,
            // 0x1000). Length digit on `T2`: empty (0 and sign-extended
            // -1), plain (8), write-back length (12). Flag digit on `A1`:
            // bit 0 (DMA start) crossed with bit 2 (write-back).
            c.g[A3][0] = [0x10, 0x70, 0x71, 0x1000][seed as usize % 4] as u64;
            c.g[T2][0] = [0, 0xffff_ffff_fffffffcu64, 8, 12][seed as usize / 4 % 4];
            c.g[A1][0] = [0, 1, 4, 5][seed as usize / 16 % 4] as u64;
            // Packet pointer and data: real buffers, nonzero fill.
            c.g[A2][0] = (DATA + 0x100) as u64;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = (DATA + 0x340) as u64;
            c.g[T1][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
            for i in 0..64 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
            }
        };
        covered(
            ee::s15::f_00426078__generated,
            _sce_sif_send_cmd,
            SEEDS,
            &setup,
        );
        // Path totals (generated cycles): 26 early exit; 85 empty via the
        // sign-extended -4; 88/93 empty with/without the DMA-start flag;
        // 96/97 packet with/without it; 89 write-back length; 124/127 the
        // write-back flag without/with DMA start; 99/106/107 the 0x70 size
        // class (empty, DMA start, plain).
        require_paths(
            ee::s15::f_00426078__generated,
            SEEDS,
            &[26, 85, 88, 89, 93, 96, 97, 99, 106, 107, 124, 127],
            &setup,
        );
    }

    /// `_sceSifSendCmd`: the `S3` flag (argument `a1`) bit 0 picks the DMA
    /// start and bit 2 the write-back, on seeds where the packet path runs.
    #[test]
    fn _sce_sif_send_cmd_s1_flag_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_320_002, 4);
            c.pc = 0x0042_6078;
            c.g[A3][0] = 0x20;
            // Flag digit: bit 0 (syscall vs inline DMA start) crossed with
            // bit 2 (write-back off vs on); the length stays a plain 8.
            c.g[A1][0] = [0, 1, 4, 5][seed as usize % 4] as u64;
            c.g[T2][0] = 8;
            c.g[A2][0] = (DATA + 0x100) as u64;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = (DATA + 0x340) as u64;
            c.g[T1][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
            for i in 0..64 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
            }
        };
        covered(
            ee::s15::f_00426078__generated,
            _sce_sif_send_cmd,
            SEEDS,
            &setup,
        );
        // Path totals (generated cycles), one per flag setting on the
        // packet path: 97/96 bit 0 (DMA start) off/on, 124/127 bit 2
        // (write-back) on with bit 0 off/on.
        require_paths(
            ee::s15::f_00426078__generated,
            SEEDS,
            &[96, 97, 124, 127],
            &setup,
        );
    }

    /// `sceSifExitRpc`: run `sceSifExitCmd`, clear the RPC client word.
    #[test]
    fn sce_sif_exit_rpc_matches() {
        covered(
            ee::s15::f_004265a8__generated,
            sce_sif_exit_rpc,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_321_010, 2);
                c.pc = 0x0042_65a8;
                c.bus
                    .write32(SIF_CLIENT_WORD, seed.wrapping_mul(0x9e37_79b9) | 1);
            },
        );
    }

    /// `sceSifRemoveCmdHandler`: negative (system table) and non-negative
    /// (user table) cids, crossing the table words.
    #[test]
    fn sce_sif_remove_cmd_handler_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_001, 0);
            c.pc = 0x0042_6050;
            // cid digit: negative, zero, small, large (index words differ).
            c.g[A0][0] = [-4, 0, 3, 0x1_0000i64][seed as usize % 4] as u64;
            c.bus.write32(SYS_TABLE, 0x0101_0100 + seed);
            c.bus.write32(USER_TABLE, 0x0202_0200 + seed);
            c.bus.write32(SYS_TABLE + 0x18, 0x0303_0300 + seed);
            c.bus.write32(USER_TABLE + 0x18, 0x0404_0400 + seed);
        };
        check_paths(
            ee::s15::f_00426050__generated,
            sce_sif_remove_cmd_handler,
            SEEDS,
            &[7, 8],
            &setup,
        );
    }

    /// `sceSifCmd_ChangeAddrHandler`: copy `a0 + 0x10` to `a1 + 8`.
    #[test]
    fn sce_sif_cmd_change_addr_handler_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_002, 0);
            c.pc = 0x0042_5ce0;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus
                .write32(DATA + 0x110, seed.wrapping_mul(0x9e37_79b9) | 1);
            c.bus
                .write32(DATA + 0x208, seed.wrapping_mul(0x5bd1_e995) | 1);
        };
        check_paths(
            ee::s15::f_00425ce0__generated,
            sce_sif_cmd_change_addr_handler,
            SEEDS,
            &[3],
            &setup,
        );
    }

    /// `sceSifExitCmd`: stop DMA channel 5, drop its handler, clear state.
    /// Single path (the totals vary with the syscall fake's charge in
    /// `RemoveDmacHandler` and the scrambled cop0 state in `DisableDmac`).
    #[test]
    fn sce_sif_exit_cmd_matches() {
        covered(
            ee::s15::f_00425fb8__generated,
            sce_sif_exit_cmd,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_322_003, 4);
                c.pc = 0x0042_5fb8;
                c.bus
                    .write32(0x0052_bcd4, seed.wrapping_mul(0x9e37_79b9) | 1);
                c.bus
                    .write32(0x0045_517c, seed.wrapping_mul(0x5bd1_e995) | 1);
            },
        );
    }

    /// `sceSifRpc_GetFPacket`: divisor/dividend classes, including 0, with
    /// LO/HI compared (the `div` writes both).
    #[test]
    fn sce_sif_rpc_get_fpacket_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_004, 0);
            c.pc = 0x0042_6698;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Divisor digit: 0, 1, 7. Dividend digit: 0, 0x40, -5.
            c.bus.write32(DATA + 0x118, [0, 1, 7][seed as usize % 3]);
            c.bus
                .write32(DATA + 0x124, [0, 0x40, 0xffff_fffb][seed as usize / 3 % 3]);
            c.bus
                .write32(DATA + 0x114, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_lohi(
            ee::s15::f_00426698__generated,
            sce_sif_rpc_get_fpacket,
            SEEDS,
            &setup,
        );
        require_paths(ee::s15::f_00426698__generated, SEEDS, &[12], &setup);
    }

    /// `sceSifRpc_GetFPacket2`: negative, in-range, and at/past-the-count
    /// indexes (index == count takes the free-packet arm); the struct
    /// words vary independently.
    #[test]
    fn sce_sif_rpc_get_fpacket2_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_005, 0);
            c.pc = 0x0042_66c8;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Index digit: negative (direct), 0 and 2 in range of the
            // count word, 3 equal to it (direct), 9 past it (direct).
            c.g[A1][0] = [-1, 0, 2, 3, 9i64][seed as usize % 5] as u64;
            c.bus.write32(DATA + 0x120, 3);
            c.bus
                .write32(DATA + 0x11c, seed.wrapping_mul(0x9e37_79b9) | 1);
            c.bus
                .write32(DATA + 0x114, seed.wrapping_mul(0x5bd1_e995) | 1);
            c.bus.write32(DATA + 0x118, (seed & 0x7f) + 1);
            c.bus.write32(DATA + 0x124, seed | 1);
        };
        check_paths(
            ee::s15::f_004266c8__generated,
            sce_sif_rpc_get_fpacket2,
            SEEDS,
            &[12, 21, 25],
            &setup,
        );
    }

    /// RPC-loop fixture: the game-like machine plus the queue below.
    /// Every seed unwinds at a fake call (`LOG_UNWIND_AT` set explicitly —
    /// the loop itself never returns), so both sides are compared on the
    /// same prefix.
    pub(crate) fn rpcloop_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 1_325_001, 4);
        rpcloop_queue(c, seed);
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, seed % 2);
    }

    /// The rpc-loop queue fixture (no machine): the next-request word
    /// (queue + 0xc) alternates: 0 takes the sleep path (unwound), a
    /// scratch request address (DATA+0x200, which GetNextRequest returns)
    /// takes the re-request arm once. ExecRequest reads the executed
    /// function at its argument + 4; the first round passes the queue
    /// itself, whose word at +4 names a real zero-returning leaf
    /// (`0x1406c0`, so no packet work happens). The rest is pool scratch
    /// the packet code reads.
    pub(crate) fn rpcloop_queue(c: &mut Cpu, seed: u32) {
        c.pc = 0x0042_73f8;
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.bus
            .write32(DATA + 0x10c, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
        c.bus.write32(DATA + 0x104, 0x0014_06c0);
        c.bus.write32(DATA + 0x108, seed);
        c.bus.write32(DATA + 0x124, seed);
        c.bus.write32(DATA + 0x118, seed);
        c.bus.write32(DATA + 0x114, seed | 1);
        c.bus.write32(DATA + 0x120, 3);
        c.bus.write32(DATA + 0x128, seed);
        c.bus.write32(DATA + 0x130, 0);
        c.bus.write32(DATA + 0x134, 0);
        c.bus.write32(DATA + 0x11c, seed | 1);
        c.bus.write32(DATA + 0x120 + 0x20, seed);
        c.bus.write32(DATA + 0x200 + 0x3c, 0);
        for i in 0..16 {
            c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
        }
    }

    /// `sceSifRpcLoop`: one `ExecRequest` + `GetNextRequest` round (empty
    /// queue sleeps, non-empty re-requests once), unwound at a fake call.
    #[test]
    fn sce_sif_rpc_loop_matches() {
        covered(
            ee::s15::f_004273f8__generated,
            sce_sif_rpc_loop,
            SEEDS,
            &|c, seed| {
                rpcloop_setup(c, seed);
            },
        );
    }

    /// Gate for the rpc-loop `EIntr` fake (per-machine; other tests'
    /// machines read 0 here, so their runs fall back to the slot body).
    pub(crate) const RPCLOOP_GATE: u32 = DATA + 0x6260;
    /// `EIntr`-at-`0x427218` calls so far: the first skips the reschedule
    /// (the re-request arm runs), later ones reschedule (the test unwinds).
    pub(crate) const RPCLOOP_EINTR_N: u32 = DATA + 0x6264;
    pub(crate) const RPCLOOP_MAGIC: u32 = 0x5250_4352;

    /// `EIntr` at the end of `sceSifGetNextRequest` (`ra == 0x0042_7218`)
    /// with the gate set: enable interrupts like the real one. The first
    /// call skips the `ei()` reschedule, so the back edge after a nonzero
    /// request runs the re-request arm instead of unwinding; later calls
    /// reschedule, so the second back edge unwinds and the test ends (the
    /// IOP never answers headless, and the loop itself never returns).
    /// Anywhere else (or with the gate clear): the slot body.
    fn fake_rpcloop_eintr(c: &mut Cpu) {
        if c.bus.read32(RPCLOOP_GATE) != RPCLOOP_MAGIC || c.g[RA][0] as u32 != 0x0042_7218 {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_rpcloop_eintr),
                ee::s15::f_0042c0c0__generated,
            );
        }
        let n = c.bus.read32(RPCLOOP_EINTR_N);
        c.bus.write32(RPCLOOP_EINTR_N, n + 1);
        if n > 0 {
            c.next_event = c.cycles;
        }
        c.g[V0][0] = ((c.cop0[12] >> 16) & 1) as u64;
        c.cop0[12] |= 1 << 16;
        c.cycles += 6;
        crate::abi::ret(c);
    }

    /// Hook the rpc-loop `EIntr` fake into its override slot.
    pub(crate) fn rpcloop_hook_fakes() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C0C0),
            fake_rpcloop_eintr,
            crate::difftest::fakes::prev_cell(fake_rpcloop_eintr),
        );
    }

    /// Re-request setup: the queue fixture with a request pending on every
    /// seed, no fake-call unwind (the loop must unwind at the back edge),
    /// and the `EIntr` fake keeping the first back edge quiet so a nonzero
    /// request re-requests once, then unwinds.
    pub(crate) fn rpcloop_rerequest_setup(c: &mut Cpu, seed: u32) {
        rpcloop_setup(c, seed);
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        rpcloop_hook_fakes();
        c.bus.write32(RPCLOOP_GATE, RPCLOOP_MAGIC);
        c.bus.write32(RPCLOOP_EINTR_N, 0);
        c.bus.write32(DATA + 0x10c, DATA + 0x200);
    }

    /// `sceSifRpcLoop` through the re-request arm: a nonzero request runs
    /// `ExecRequest` again instead of unwinding at the back edge.
    #[test]
    fn sce_sif_rpc_loop_rerequest_matches() {
        check(
            ee::s15::f_004273f8__generated,
            sce_sif_rpc_loop,
            SEEDS,
            &|c, seed| {
                rpcloop_rerequest_setup(c, seed);
            },
        );
        require_paths(
            ee::s15::f_004273f8__generated,
            SEEDS,
            // One re-request then the second back edge unwinds; the spread
            // is the syscall fake's per-seed charge in the real `ExecRequest`.
            &[
                1316217, 1316221, 1316222, 1316224, 1316226, 1316230, 1316231, 1316234, 1316239,
                1316246, 1316248, 1316250, 1316252, 1316254, 1316258, 1316260, 1316262, 1316263,
                1316264, 1316270, 1316272, 1316279, 1316287, 1316289, 1316293, 1316297,
            ],
            &|c, seed| {
                rpcloop_rerequest_setup(c, seed);
            },
        );
    }

    /// `sceSifRpcLoop` with events due: the back edges unwind identically
    /// on both sides. Not driven: the loop never returns to its caller, so
    /// a trampoline re-entry would restart it forever; the plain comparison
    /// still verifies the unwound states and schedules. `Deliver` seeds run
    /// as `Unwind` (no VBlank is ever due headless on a `Deliver` schedule,
    /// so the loop would neither unwind nor exit).
    #[test]
    fn sce_sif_rpc_loop_events() {
        check_events(
            ee::s15::f_004273f8__generated,
            sce_sif_rpc_loop,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_325_002, 0);
                rpcloop_queue(c, seed);
                rpcloop_hook_fakes();
                c.bus.write32(RPCLOOP_GATE, RPCLOOP_MAGIC);
                c.bus.write32(RPCLOOP_EINTR_N, 0);
                c.bus.write32(DATA + 0x10c, DATA + 0x200);
                let mode = schedule(c, &mut r, seed, 0x0042_73f8);
                if mode == crate::difftest::events::Events::Deliver {
                    ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                        .sched
                        .next_vblank = 0;
                    crate::difftest::events::Events::Unwind
                } else {
                    mode
                }
            },
        );
    }

    /// `sceSifRpcLoop` entered at a resume head: every non-entry `pc` runs
    /// the generated body (the guard), so both sides run the same partial
    /// pass; flipping the guard would restart the hand arms from the entry.
    #[test]
    fn sce_sif_rpc_loop_resume_matches() {
        check(
            ee::s15::f_004273f8__generated,
            sce_sif_rpc_loop,
            SEEDS,
            &|c, seed| {
                rpcloop_setup(c, seed);
                // A resume frame: the entry's saves with a plausible
                // return, then the head under test.
                c.g[S0][0] = c.g[A0][0];
                c.g[ssx3_ee_rt::cpu::reg::SP][0] =
                    sx((c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32).wrapping_sub(0x20));
                c.sd(c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32, c.g[S0][0]);
                c.sd(
                    (c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32).wrapping_add(0x10),
                    c.g[RA][0],
                );
                c.pc = [0x0042_7410, 0x0042_7418][seed as usize % 2];
            },
        );
    }
    use crate::difftest::fakes;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;

    /// Gate + forced result for the send-command fakes below (per-machine
    /// words; other tests' machines read 0 here, so their runs fall back
    /// to the slot bodies).
    pub(crate) const SEND_CMD_GATE: u32 = DATA + 0x6400;

    pub(crate) const SEND_RESULT: u32 = DATA + 0x6404;

    /// `isceSifSendCmd`/`sceSifSendCmd` with the gate set: report the
    /// forced result without sending. Anywhere else (or with the gate
    /// clear): the slot body.
    pub(crate) fn fake_send_zero(c: &mut Cpu) {
        if c.bus.read32(SEND_CMD_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_send_zero),
                ee::s15::f_004261f0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(SEND_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `sceSifSendCmd` with the gate set: fail the next `SEND_N` sends
    /// (`u32::MAX` fails them all), then the slot body. Anywhere else
    /// (or with the gate clear): the slot body.
    pub(crate) const SEND_N: u32 = DATA + 0x641c;

    pub(crate) fn fake_send_cmd_zero(c: &mut Cpu) {
        if c.bus.read32(SEND_CMD_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_send_cmd_zero),
                ee::s15::f_004261b0__generated,
            );
        }
        let n = c.bus.read32(SEND_N);
        if n == 0 {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_send_cmd_zero),
                ee::s15::f_004261b0__generated,
            );
        }
        if n != u32::MAX {
            c.bus.write32(SEND_N, n - 1);
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_send_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004261F0),
            fake_send_zero,
            fakes::prev_cell(fake_send_zero),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004261B0),
            fake_send_cmd_zero,
            fakes::prev_cell(fake_send_cmd_zero),
        );
    }

    /// Packet words the SIF sends read: nonzero with nonzero low bytes.
    pub(crate) fn send_words(c: &mut Cpu, base: u32, seed: u32) {
        for (i, off) in [0x14u32, 0x1c, 0x20, 0x24, 0x28].iter().enumerate() {
            c.bus.write32(
                base + off,
                0x0101_0100
                    + seed
                        .wrapping_mul(0x9e37_79b9)
                        .wrapping_add(i as u32 * 0x110),
            );
        }
    }

    /// `sceSifRpc_RetrySendAlarm`: the retry packet goes out through the
    /// real send (length 8..64: packet fill, always succeeding here).
    #[test]
    fn retry_send_matches() {
        covered(
            ee::s15::f_004267c0__generated,
            sce_sif_rpc_retry_send_alarm,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_001, 1);
                c.pc = 0x0042_67c0;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.bus.write32(DATA + 0x324, 0x0202_0200 + seed);
                c.bus.write32(DATA + 0x328, 0x0303_0300 + seed);
                // Length digit: small packet fills, all succeeding.
                c.bus.write32(DATA + 0x32c, [8, 12, 64][seed as usize % 3]);
            },
        );
    }

    /// `sceSifRpc_RetrySendAlarm` with the send forced to fail: the alarm
    /// arm runs the real `iSetAlarm`.
    #[test]
    fn retry_send_fail_matches() {
        hook_send_fakes();
        covered(
            ee::s15::f_004267c0__generated,
            sce_sif_rpc_retry_send_alarm,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_002, 1);
                c.pc = 0x0042_67c0;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.bus.write32(DATA + 0x324, 0x0202_0200 + seed);
                c.bus.write32(DATA + 0x328, 0x0303_0300 + seed);
                c.bus.write32(DATA + 0x32c, 12);
                c.bus.write32(SEND_CMD_GATE, GATE_MAGIC);
                c.bus.write32(SEND_RESULT, 0);
            },
        );
    }

    /// Pool struct shared by the `GetFPacket`/`GetFPacket2` fixtures: count
    /// 3, all words nonzero with nonzero low bytes.
    pub(crate) fn sif_pool(c: &mut Cpu, base: u32, seed: u32) {
        for i in 0..10 {
            c.bus.write32(
                base + 4 * i,
                0x0101_0100 + seed.wrapping_mul(0x5bd1_e995).wrapping_add(i * 0x111),
            );
        }
        c.bus.write32(base + 0x20, 3);
    }

    /// Real packets for the packet pool at `pool`: free packets at `DATA +
    /// 0x1000` (four of 0x40 bytes; the next one from `seed`), fixed
    /// packets at `DATA + 0x2000` (three), every packet word nonzero so the
    /// stores into them show.
    pub(crate) fn real_pool(c: &mut Cpu, pool: u32, seed: u32) {
        c.bus.write32(pool + 0x14, DATA + 0x1000);
        c.bus.write32(pool + 0x18, 4);
        c.bus.write32(pool + 0x1c, DATA + 0x2000);
        c.bus.write32(pool + 0x20, 3);
        c.bus.write32(pool + 0x24, seed % 7);
        for i in 0..0x40u32 {
            c.bus.write32(DATA + 0x1000 + 4 * i, 0x5a5a_0000 + 4 * i);
        }
        for i in 0..0x30u32 {
            c.bus.write32(DATA + 0x2000 + 4 * i, 0x6b6b_0000 + 4 * i);
        }
    }

    /// `sceSifRpc_RequestRData`: flag bit 2 clear takes `GetFPacket`,
    /// set takes `GetFPacket2` (index `word >> 16`: 0 and 2 in range of
    /// the count, 9 past it); the reply then goes out through the real
    /// send.
    #[test]
    fn request_rdata_matches() {
        covered(
            ee::s15::f_00426820__generated,
            sce_sif_rpc_request_rdata,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_003, 1);
                c.pc = 0x0042_6820;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                sif_pool(c, DATA + 0x100, seed);
                real_pool(c, DATA + 0x100, seed);
                // Bit digit crossed with the pool-word index digit.
                let bit = [0, 4, 4][seed as usize % 3];
                let idx = [0, 2, 9][seed as usize / 3 % 3];
                c.bus
                    .write32(DATA + 0x310, ((idx << 16) | bit | 0x0101) as u32);
                // Send-setup words: inline size, small lengths.
                send_words(c, DATA + 0x300, seed);
                c.bus.write32(DATA + 0x320, 0x40);
                c.bus
                    .write32(DATA + 0x328, [8, 12, 64][seed as usize / 9 % 3]);
            },
        );
        // The pool's own splits (index == count takes the free-packet
        // arm) are covered by the `GetFPacket2` tests; the request arms
        // here end in the real send, whose fake call charges vary.
    }

    /// `sceSifRpc_RequestRData` with the send forced to fail: the alarm
    /// arm runs the real `iSetAlarm`.
    #[test]
    fn request_rdata_fail_matches() {
        hook_send_fakes();
        covered(
            ee::s15::f_00426820__generated,
            sce_sif_rpc_request_rdata,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_004, 1);
                c.pc = 0x0042_6820;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                sif_pool(c, DATA + 0x100, seed);
                real_pool(c, DATA + 0x100, seed);
                let bit = [0, 4][seed as usize % 2];
                c.bus
                    .write32(DATA + 0x310, ((2 << 16) | bit | 0x0101) as u32);
                send_words(c, DATA + 0x300, seed);
                c.bus.write32(DATA + 0x320, 0x40);
                c.bus.write32(DATA + 0x328, 12);
                c.bus.write32(SEND_CMD_GATE, GATE_MAGIC);
                c.bus.write32(SEND_RESULT, 0);
            },
        );
    }

    /// Gate + reply packet for the `GetPacket` fake below.
    pub(crate) const PACKET_GATE: u32 = DATA + 0x6408;

    pub(crate) const PACKET_ADDR: u32 = DATA + 0x640c;

    /// `sceSifRpc_GetPacket` with the gate set: hand out the test packet.
    /// Anywhere else (or with the gate clear): the slot body (the zeroed
    /// fixed pool then takes the null arm).
    pub(crate) fn fake_get_packet(c: &mut Cpu) {
        if c.bus.read32(PACKET_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_get_packet),
                ee::s15::f_004265d0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(PACKET_ADDR) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_packet_fake() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004265D0),
            fake_get_packet,
            fakes::prev_cell(fake_get_packet),
        );
    }

    /// Reply packet for `GetOtherData`: all words nonzero with nonzero
    /// low bytes.
    pub(crate) fn other_packet(c: &mut Cpu, base: u32, seed: u32) {
        for i in 0..10 {
            c.bus.write32(
                base + 4 * i,
                0x0404_0400 + seed.wrapping_mul(0x1f2b_3c4d).wrapping_add(i * 0x101),
            );
        }
    }

    /// `sceSifGetOtherData`: no packet from the pool reports -1; a packet
    /// with flag bit 0 set sends straight through, clear waits on a fresh
    /// semaphore; both end in the real `SendCmd`.
    #[test]
    fn get_other_data_matches() {
        hook_packet_fake();
        hook_send_fakes();
        covered(
            ee::s15::f_004268f0__generated,
            sce_sif_get_other_data,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_005, 1);
                c.pc = 0x0042_68f0;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = 0x1111_1100 + seed as u64;
                c.g[A2][0] = 0x2222_2200 + seed as u64;
                c.g[A3][0] = 0x3333_3300 + seed as u64;
                // Flag digit: direct send vs semaphore wait.
                c.g[T0][0] = [1, 0, 0][seed as usize % 3] as u64;
                // Length the sends use.
                c.g[T2][0] = 12;
                // Packet digit: null pool arm vs handed-out packet.
                if seed / 3 % 2 == 0 {
                    other_packet(c, DATA + 0x400, seed);
                    c.bus.write32(PACKET_GATE, GATE_MAGIC);
                    c.bus.write32(PACKET_ADDR, DATA + 0x400);
                } else {
                    c.bus.write32(PACKET_GATE, 0);
                }
            },
        );
        // Unreached: the `CreateSema` failure arm (V0 = -3). The
        // harness's fake syscalls always return non-negative ids, so
        // the failure check never fires here; in the game a drained
        // semaphore table takes it.
    }

    /// `sceSifGetOtherData` with the send forced to fail: both the
    /// semaphore and the direct path free the packet and report -2.
    #[test]
    fn get_other_data_send_fail_matches() {
        hook_packet_fake();
        hook_send_fakes();
        covered(
            ee::s15::f_004268f0__generated,
            sce_sif_get_other_data,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_006, 1);
                c.pc = 0x0042_68f0;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = 0x1111_1100 + seed as u64;
                c.g[A2][0] = 0x2222_2200 + seed as u64;
                c.g[A3][0] = 0x3333_3300 + seed as u64;
                c.g[T0][0] = (seed % 2) as u64;
                c.g[T2][0] = 12;
                other_packet(c, DATA + 0x400, seed);
                c.bus.write32(PACKET_GATE, GATE_MAGIC);
                c.bus.write32(PACKET_ADDR, DATA + 0x400);
                c.bus.write32(SEND_CMD_GATE, GATE_MAGIC);
                c.bus.write32(SEND_N, u32::MAX);
            },
        );
    }

    /// Bucket list for `SearchSvdata`: two buckets, three entries then
    /// two; every id word nonzero with a nonzero low byte.
    pub(crate) fn search_buckets(
        c: &mut Cpu,
        seed: u32,
        null_list: bool,
        absent: bool,
        target_slot: u32,
    ) -> u32 {
        let ids = [
            0x0b0b_0b01 + seed,
            0x0b0b_0b02 + seed,
            0x0b0b_0b03 + seed,
            0x0c0c_0c01 + seed,
            0x0c0c_0c02 + seed,
        ];
        if null_list {
            c.bus.write32(DATA + 0x328, 0);
            return 0x0d0d_0d00 + seed;
        }
        c.bus.write32(DATA + 0x328, DATA + 0x400);
        // Bucket 0 (entries 0-2) -> bucket 1 (entries 3-4) -> null.
        c.bus.write32(DATA + 0x400, 0);
        c.bus.write32(DATA + 0x408, DATA + 0x500);
        c.bus.write32(DATA + 0x414, DATA + 0x480);
        c.bus.write32(DATA + 0x480, 0);
        c.bus.write32(DATA + 0x488, DATA + 0x5c0);
        c.bus.write32(DATA + 0x494, 0);
        // Entries: ids then null-terminated chains.
        for (i, base) in [0x500u32, 0x540, 0x580, 0x5c0, 0x600].iter().enumerate() {
            c.bus.write32(DATA + *base, ids[i]);
            c.bus.write32(
                DATA + *base + 0x38,
                if i == 2 || i == 4 {
                    0
                } else {
                    DATA + base + 0x40
                },
            );
        }
        if absent {
            0x0e0e_0e00 + seed
        } else {
            ids[target_slot as usize % ids.len()]
        }
    }

    /// `sceSifRpc_SearchSvdata`: empty list, id found in the first entry,
    /// id found in the last entry, id absent. No calls on any path, so
    /// the four totals pin every arm.
    #[test]
    fn search_svdata_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_361_007, 0);
            c.pc = 0x0042_6a48;
            // Mode digit: null list / found first / found last / absent.
            let mode = seed % 4;
            let target = search_buckets(c, seed, mode == 0, mode == 3, 4);
            c.g[A0][0] = (if mode == 1 {
                0x0b0b_0b01 + seed
            } else {
                target
            }) as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
        };
        check(
            ee::s15::f_00426a48__generated,
            sce_sif_rpc_search_svdata,
            SEEDS,
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00426a48__generated);
            seen.insert(c.cycles - before);
        }
        // Four modes, four totals: one arm per total.
        assert_eq!(seen.len(), 4, "paths seen: {seen:?}");
    }

    driven!(search_svdata_driven, sce_sif_rpc_search_svdata);

    driven!(
        search_svdata_generated_driven,
        ee::s15::f_00426a48__generated
    );

    /// `sceSifRpc_SearchSvdata` with events due: deliveries on both list
    /// walks deliver inline on both sides; the loop still finds the id.
    /// Two buckets: the first one's six misses run the inner head
    /// (0x426a68), the hop to the second (entries at `DATA + 0x700`, apart
    /// from the first's) the outer head (0x426a58).
    #[test]
    fn search_svdata_events() {
        check_events(
            search_svdata_generated_driven,
            search_svdata_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_361_008, 0);
                c.bus.write32(DATA + 0x328, DATA + 0x400);
                c.bus.write32(DATA + 0x408, DATA + 0x500);
                c.bus.write32(DATA + 0x414, DATA + 0x480);
                c.bus.write32(DATA + 0x488, DATA + 0x700);
                c.bus.write32(DATA + 0x494, 0);
                // Bucket 1: six misses; bucket 2: miss then the target.
                for i in 0..6u32 {
                    c.bus
                        .write32(DATA + 0x500 + 0x40 * i, 0x0b0b_0c00 + seed + i);
                    c.bus.write32(
                        DATA + 0x500 + 0x40 * i + 0x38,
                        if i == 5 {
                            0
                        } else {
                            DATA + 0x500 + 0x40 * (i + 1)
                        },
                    );
                }
                c.bus.write32(DATA + 0x700, 0x0b0b_0d00 + seed);
                c.bus.write32(DATA + 0x738, DATA + 0x740);
                c.bus.write32(DATA + 0x740, 0x0b0b_0b00 + seed);
                c.bus.write32(DATA + 0x778, 0);
                c.g[A0][0] = (0x0b0b_0b00 + seed) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                schedule(c, &mut r, seed, 0x0042_6a48)
            },
        );
    }

    /// Bucket list like [`search_buckets`] but rooted at `holder` with
    /// buckets/entries at `base`: for callees whose holder is not the
    /// DATA+0x300 request struct.
    pub(crate) fn search_buckets_at(
        c: &mut Cpu,
        seed: u32,
        holder: u32,
        base: u32,
        absent: bool,
    ) -> u32 {
        let ids = [
            0x0b0b_0b01 + seed,
            0x0b0b_0b02 + seed,
            0x0b0b_0b03 + seed,
            0x0c0c_0c01 + seed,
            0x0c0c_0c02 + seed,
        ];
        c.bus.write32(holder + 0x28, base);
        c.bus.write32(base, 0);
        c.bus.write32(base + 0x8, base + 0x100);
        c.bus.write32(base + 0x14, base + 0x80);
        c.bus.write32(base + 0x80, 0);
        c.bus.write32(base + 0x88, base + 0x1c0);
        c.bus.write32(base + 0x94, 0);
        for (i, off) in [0x100u32, 0x140, 0x180, 0x1c0, 0x200].iter().enumerate() {
            c.bus.write32(base + off, ids[i]);
            c.bus.write32(
                base + off + 0x38,
                if i == 2 || i == 4 {
                    0
                } else {
                    base + off + 0x40
                },
            );
        }
        if absent {
            0x0e0e_0e00 + seed
        } else {
            ids[4]
        }
    }

    /// `sceSifRpc_RequestBind`: with the id present in the server data
    /// the packet takes the found words, otherwise zeros; either way it
    /// goes out through the real `isceSifSendCmd`.
    #[test]
    fn request_bind_matches() {
        check(
            ee::s15::f_00426a98__generated,
            sce_sif_rpc_request_bind,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_009, 1);
                c.pc = 0x0042_6a98;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                // GetFPacket inputs: the packet at `DATA + 0x700` (index
                // 0 of the pool's free packets), its words nonzero.
                c.bus.write32(DATA + 0x400, 0x2323_2300 + seed);
                c.bus.write32(DATA + 0x414, DATA + 0x700);
                c.bus.write32(DATA + 0x418, 0x2424_2400 + seed);
                c.bus.write32(DATA + 0x424, 0);
                for i in 0..0x30u32 {
                    c.bus.write32(DATA + 0x700 + 4 * i, 0x7c7c_0000 + 4 * i);
                }
                // Found vs missing server entry; the found one (at `DATA +
                // 0x640`) has nonzero words for the packet (`+0x8`, `+0x14`).
                let absent = seed % 2 == 0;
                let target = search_buckets_at(c, seed, DATA + 0x400, DATA + 0x440, absent);
                c.bus.write32(DATA + 0x648, 0x2626_2600 + seed);
                c.bus.write32(DATA + 0x654, 0x2727_2700 + seed);
                c.bus.write32(DATA + 0x31c, target);
                c.bus.write32(DATA + 0x314, 0x2525_2500 + seed);
                c.bus.write32(DATA + 0x320, target);
            },
        );
    }

    /// `sceSifRpc_RequestCall`: the request packet starts the server
    /// entry's queue (empty) or is appended at its tail, the call words are
    /// copied into the entry, and the server thread is woken unless its id
    /// is negative or its wake word is already set (real `iWakeupThread`).
    /// The four totals without the wakeup (which makes syscalls) pin the
    /// empty/append and the two no-wake arms.
    #[test]
    fn request_call_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_361_010, 0);
            c.pc = 0x0042_6c88;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.bus.write32(DATA + 0x334, DATA + 0x400);
            c.bus.write32(DATA + 0x440, DATA + 0x500);
            for (i, off) in [0x14u32, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x10]
                .iter()
                .enumerate()
            {
                c.bus.write32(
                    DATA + 0x300 + off,
                    0x3131_3100
                        + seed
                            .wrapping_mul(0xabcd_1234)
                            .wrapping_add(i as u32 * 0x101),
                );
            }
            // The entry words the copies overwrite start nonzero.
            for i in 0..12u32 {
                c.bus.write32(DATA + 0x40c + 4 * i, 0x3838_0000 + 4 * i);
            }
            // Queue digit: empty, or a queued packet whose tail (at `DATA
            // + 0x640`) takes the append.
            let queued = seed % 2 == 1;
            c.bus
                .write32(DATA + 0x50c, if queued { DATA + 0x600 } else { 0 });
            c.bus.write32(
                DATA + 0x510,
                if queued {
                    DATA + 0x640
                } else {
                    0x4343_4300 + seed
                },
            );
            c.bus.write32(DATA + 0x67c, 0x4444_4400 + seed);
            // Thread digit: id negative, wake word set, or a wakeup.
            let mode = seed / 2 % 3;
            c.bus.write32(
                DATA + 0x500,
                if mode == 0 {
                    0xffff_ffff
                } else {
                    0x4545_4500 + seed
                },
            );
            c.bus
                .write32(DATA + 0x504, if mode == 2 { 0 } else { 0x4646_4600 + seed });
        };
        check_paths(
            ee::s15::f_00426c88__generated,
            sce_sif_rpc_request_call,
            SEEDS,
            &[28, 29, 31, 32],
            setup,
        );
    }

    /// `sceSifGetNextRequest`: a queued request is taken off the server
    /// entry (marked taken), otherwise the entry is reported empty; the
    /// interrupt state is restored either way.
    #[test]
    fn get_next_request_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_361_011, 0);
            c.pc = 0x0042_71d8;
            c.g[A0][0] = (DATA + 0x300) as u64;
            // The taken word both arms store, nonzero before.
            c.bus.write32(DATA + 0x304, 0x5252_5200 + seed);
            // Queue digit: empty, a request with another behind it, the
            // last request.
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x30c, 0),
                1 => {
                    c.bus.write32(DATA + 0x30c, DATA + 0x400);
                    c.bus.write32(DATA + 0x43c, 0x5151_5100 + seed);
                }
                _ => {
                    c.bus.write32(DATA + 0x30c, DATA + 0x400);
                    c.bus.write32(DATA + 0x43c, 0);
                }
            }
        };
        check(
            ee::s15::f_004271d8__generated,
            sce_sif_get_next_request,
            SEEDS,
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_004271d8__generated);
            seen.insert(c.cycles - before);
        }
        assert_eq!(seen.len(), 2, "paths seen: {seen:?}");
    }

    /// Gate + countdown for the `SetDma` fake below.
    pub(crate) const SET_DMA_GATE: u32 = DATA + 0x6414;

    pub(crate) const DMA_FAIL_N: u32 = DATA + 0x6418;

    /// `sceSifSetDma` with the gate set: fail the next `DMA_FAIL_N` starts
    /// (the wait loop spins), then fall back to the slot body. Anywhere
    /// else (or with the gate clear): the slot body.
    pub(crate) fn fake_set_dma_zero(c: &mut Cpu) {
        if c.bus.read32(SET_DMA_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_set_dma_zero),
                ee::s15::f_00424160__generated,
            );
        }
        let n = c.bus.read32(DMA_FAIL_N);
        if n > 0 {
            c.bus.write32(DMA_FAIL_N, n - 1);
            c.g[V0][0] = 0;
            c.cycles += 4;
            crate::abi::ret(c);
            return;
        }
        fakes::fall_back(
            c,
            fakes::prev_cell(fake_set_dma_zero),
            ee::s15::f_00424160__generated,
        );
    }

    pub(crate) fn hook_dma_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00424160),
            fake_set_dma_zero,
            fakes::prev_cell(fake_set_dma_zero),
        );
    }

    /// Request struct for `ExecRequest`: every steering word from an
    /// independent digit (server address/kind, cache sizes, pool flag,
    /// reply count), and real packets in the SIF pool at `0x52d680`.
    pub(crate) fn exec_setup(c: &mut Cpu, seed: u32) {
        c.pc = 0x0042_7230;
        real_pool(c, 0x0052_d680, seed);
        c.g[A0][0] = (DATA + 0x300) as u64;
        // Server digit: the real zero leaf (no reply) vs an unmapped
        // address (the miss-hook fake answers nonzero).
        c.bus.write32(
            DATA + 0x304,
            if seed % 2 == 0 {
                0x0014_06c0
            } else {
                DATA + 0x700
            },
        );
        for (i, off) in [0x8u32, 0x24, 0x1c, 0x20, 0x28].iter().enumerate() {
            c.bus.write32(
                DATA + 0x300 + off,
                0x6161_6100
                    + seed
                        .wrapping_mul(0x789a_bcde)
                        .wrapping_add(i as u32 * 0x111),
            );
        }
        // Cache-size digits: writeback or skip, each.
        c.bus.write32(
            DATA + 0x30c,
            if seed / 2 % 2 == 0 { 0xffff_fff0 } else { 0x10 },
        );
        c.bus.write32(
            DATA + 0x32c,
            if seed / 4 % 2 == 0 { 0xffff_fff0 } else { 0x10 },
        );
        // Pool flag digit: plain packet vs free-pool packet; in-range index.
        c.bus.write32(
            DATA + 0x334,
            (((seed / 8 % 2) * 2) << 16) | ((seed / 8 % 2) * 4) | 0x0101,
        );
        // Reply-count digit: skip the send loop or run it once.
        c.bus
            .write32(DATA + 0x330, if seed / 16 % 2 == 0 { 0 } else { 5 });
    }

    /// `sceSifExecRequest`: the server answer picks the reply shape, the
    /// cache sizes pick the writebacks, the pool flag picks the packet
    /// and the reply count runs or skips the send loop; the real `SendCmd`
    /// succeeds first try.
    #[test]
    fn exec_request_matches() {
        covered(
            ee::s15::f_00427230__generated,
            sce_sif_exec_request,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_361_012, 1);
                exec_setup(c, seed);
            },
        );
    }

    /// `sceSifExecRequest` on the DMA path (no reply count) with the first
    /// DMA start failing: the countdown (0x4273b0) and the retry
    /// (0x427398) run once, then the real start goes through. Few seeds:
    /// each run spins about 7.3 M cycles.
    #[test]
    fn exec_request_dma_wait_matches() {
        hook_dma_fakes();
        check_paths(
            ee::s15::f_00427230__generated,
            sce_sif_exec_request,
            8,
            &[7340220, 7340222, 7340227, 7340228, 7340230],
            &|c, seed| {
                machine(c, seed, 1_361_013, 0);
                exec_setup(c, seed * 16 + 15);
                // Nonsense server answer, no reply count (the DMA path).
                c.bus.write32(DATA + 0x304, DATA + 0x700);
                c.bus.write32(DATA + 0x330, 0);
                c.bus.write32(SET_DMA_GATE, GATE_MAGIC);
                c.bus.write32(DMA_FAIL_N, 1);
            },
        );
    }

    driven!(exec_request_driven, sce_sif_exec_request);

    driven!(
        exec_request_generated_driven,
        ee::s15::f_00427230__generated
    );

    /// `sceSifExecRequest` with events due and the first send failing
    /// once: the send loop delivers inline on both sides, then the
    /// real send breaks it out.
    #[test]
    fn exec_request_events() {
        hook_dma_fakes();
        hook_send_fakes();
        check_events(
            exec_request_generated_driven,
            exec_request_driven,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 1_361_014, 0);
                exec_setup(c, seed * 16 + 15);
                c.bus.write32(DATA + 0x304, DATA + 0x700);
                c.bus.write32(DATA + 0x330, 5);
                c.bus.write32(SEND_CMD_GATE, GATE_MAGIC);
                c.bus.write32(SEND_N, 1);
                schedule(c, &mut r, seed, 0x0042_7230)
            },
        );
    }

    /// Queue links in the fixed SIF pool head (`0x52d680 + 0x28`): a chain
    /// of `len` entries at `base`, linked through `+0x14`.
    pub(crate) fn pool_chain(c: &mut Cpu, len: u32, base: u32) {
        c.bus
            .write32(0x0052_d680 + 0x28, if len == 0 { 0 } else { base });
        for i in 0..len {
            c.bus.write32(
                base + 0x40 * i + 0x14,
                if i + 1 == len {
                    0
                } else {
                    base + 0x40 * (i + 1)
                },
            );
            c.bus.write32(base + 0x40 * i, 0x7171_7100 + i);
        }
    }

    /// `sceSifSetRpcQueue`: empty head, one queued entry, three queued
    /// entries. No traps on any path, so the totals pin every arm.
    #[test]
    fn set_rpc_queue_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_362_001, 0);
            c.pc = 0x0042_6f48;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[A1][0] = 0x7272_7200 + seed as u64;
            // The queue struct's words, which the setup zeroes, start
            // nonzero.
            for i in 0..7u32 {
                c.bus.write32(DATA + 0x300 + 4 * i, 0x7373_7300 + i);
            }
            pool_chain(c, [0, 1, 3][seed as usize % 3], 0x0052_d000);
        };
        check(
            ee::s15::f_00426f48__generated,
            sce_sif_set_rpc_queue,
            SEEDS,
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00426f48__generated);
            seen.insert(c.cycles - before);
        }
        assert_eq!(seen.len(), 3, "paths seen: {seen:?}");
    }

    driven!(set_rpc_queue_driven, sce_sif_set_rpc_queue);

    driven!(
        set_rpc_queue_generated_driven,
        ee::s15::f_00426f48__generated
    );

    /// `sceSifSetRpcQueue` with events due: the append walk unwinds and
    /// re-enters identically.
    #[test]
    fn set_rpc_queue_events() {
        check_events(
            set_rpc_queue_generated_driven,
            set_rpc_queue_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_362_002, 0);
                c.pc = 0x0042_6f48;
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = 0x7272_7200 + seed as u64;
                pool_chain(c, 8, 0x0052_d000);
                schedule(c, &mut r, seed, 0x0042_6f48)
            },
        );
    }

    /// Server entry (`+0x8` head) with a chain of `len` RPC entries at
    /// `base`, linked through `+0x38`; returns the entry addresses.
    pub(crate) fn rpc_chain(c: &mut Cpu, server: u32, base: u32, len: u32) -> Vec<u32> {
        c.bus.write32(server + 0x8, if len == 0 { 0 } else { base });
        let mut out = Vec::new();
        for i in 0..len {
            let e = base + 0x40 * i;
            out.push(e);
            c.bus.write32(e, 0x8181_8100 + i);
            c.bus.write32(
                e + 0x38,
                if i + 1 == len {
                    0
                } else {
                    base + 0x40 * (i + 1)
                },
            );
        }
        out
    }

    /// `sceSifRemoveRpc`: the head drops directly, the second entry and a
    /// later one unlink by skipping, an empty list and an absent entry
    /// report null. No traps on any path, so the five totals are the five
    /// exits.
    #[test]
    fn remove_rpc_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_362_003, 0);
            c.pc = 0x0042_70b0;
            // Mode digit: head match, empty list, second-entry match, walk
            // match, absent.
            let mode = seed % 5;
            let entries = rpc_chain(c, DATA + 0x400, DATA + 0x500, if mode == 1 { 0 } else { 4 });
            c.g[A0][0] = match mode {
                0 => entries[0],
                1 => DATA + 0x500,
                2 => entries[1],
                3 => entries[2],
                _ => DATA + 0x700,
            } as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
        };
        check(
            ee::s15::f_004270b0__generated,
            sce_sif_remove_rpc,
            SEEDS,
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_004270b0__generated);
            seen.insert(c.cycles - before);
        }
        assert_eq!(seen.len(), 5, "paths seen: {seen:?}");
    }

    driven!(remove_rpc_driven, sce_sif_remove_rpc);

    driven!(remove_rpc_generated_driven, ee::s15::f_004270b0__generated);

    /// `sceSifRemoveRpc` with events due: the unlink walk unwinds and
    /// re-enters identically.
    #[test]
    fn remove_rpc_events() {
        check_events(
            remove_rpc_generated_driven,
            remove_rpc_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_362_004, 0);
                let entries = rpc_chain(c, DATA + 0x400, DATA + 0x500, 8);
                c.g[A0][0] = entries[7] as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                schedule(c, &mut r, seed, 0x0042_70b0)
            },
        );
    }

    /// `sceSifRemoveRpcQueue`: the head drops directly, the second queue
    /// and a later one unlink by skipping, an empty list and an absent
    /// queue report null. No traps on any path, so the five totals are
    /// the five exits.
    #[test]
    fn remove_rpc_queue_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_362_005, 0);
            c.pc = 0x0042_7148;
            // Mode digit: head match, empty list, second-queue match, walk
            // match, absent.
            let mode = seed % 5;
            pool_chain(c, if mode == 1 { 0 } else { 4 }, DATA + 0x500);
            c.g[A0][0] = [
                DATA + 0x500,
                DATA + 0x500,
                DATA + 0x540,
                DATA + 0x580,
                DATA + 0x700,
            ][mode as usize] as u64;
        };
        check(
            ee::s15::f_00427148__generated,
            sce_sif_remove_rpc_queue,
            SEEDS,
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00427148__generated);
            seen.insert(c.cycles - before);
        }
        assert_eq!(seen.len(), 5, "paths seen: {seen:?}");
    }

    driven!(remove_rpc_queue_driven, sce_sif_remove_rpc_queue);

    driven!(
        remove_rpc_queue_generated_driven,
        ee::s15::f_00427148__generated
    );

    /// `sceSifRemoveRpcQueue` with events due: the unlink walk unwinds
    /// and re-enters identically.
    #[test]
    fn remove_rpc_queue_events() {
        check_events(
            remove_rpc_queue_generated_driven,
            remove_rpc_queue_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_362_006, 0);
                pool_chain(c, 8, DATA + 0x500);
                c.g[A0][0] = (DATA + 0x500 + 0x40 * 7) as u64;
                schedule(c, &mut r, seed, 0x0042_7148)
            },
        );
    }

    /// Gate + poll countdown for the `GetReg` fake below (per-machine
    /// words; other tests' machines read 0 here, so their runs fall back
    /// to the slot body).
    pub(crate) const INIT_GATE: u32 = DATA + 0x6500;

    pub(crate) const INIT_POLL_N: u32 = DATA + 0x6504;

    /// `sceSifGetReg` with the gate set: the bring-up read reports no
    /// peer (enter the poll), the poll answers 0 `INIT_POLL_N` times then
    /// sets the ready bit, the post-poll read reports a fixed value.
    /// Anywhere else (or with the gate clear): the slot body.
    pub(crate) fn fake_init_get_reg(c: &mut Cpu) {
        if c.bus.read32(INIT_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_init_get_reg),
                ee::s15::f_004241b0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        if ra == 0x0042_5ee4 {
            c.g[V0][0] = 0;
        } else if ra == 0x0042_5f38 {
            let n = c.bus.read32(INIT_POLL_N);
            if n > 0 {
                c.bus.write32(INIT_POLL_N, n - 1);
                c.g[V0][0] = 0;
            } else {
                c.g[V0][0] = 0x20000;
            }
        } else if ra == 0x0042_5f4c {
            c.g[V0][0] = 0x37;
        } else {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_init_get_reg),
                ee::s15::f_004241b0__generated,
            );
        }
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_init_fake() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004241B0),
            fake_init_get_reg,
            fakes::prev_cell(fake_init_get_reg),
        );
    }

    /// `sceSifInitCmd` with the init flag set: the short exit restores
    /// the interrupt state. No traps on the path, so the total pins it.
    #[test]
    fn init_cmd_short_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_362_010, 0);
            c.pc = 0x0042_5d38;
            c.bus.write32(0x0045_0000 + 0x517c, 0x0101_0100 + seed);
        };
        check_paths(
            ee::s15::f_00425d38__generated,
            sce_sif_init_cmd,
            SEEDS,
            &[34],
            setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00425d38__generated);
            seen.insert(c.cycles - before);
        }
        assert_eq!(seen.len(), 1, "paths seen: {seen:?}");
    }

    /// `sceSifInitCmd` with the flag clear: the full bring-up runs over
    /// the game's SIF state (nonzero before, so the table clears show),
    /// ending in the real `SendCmd`. The `GetReg` fake steers the peer
    /// poll; with it off (every fourth seed) the syscall answers nonzero
    /// and the peer-already-up arm (0x425eec) runs.
    #[test]
    fn init_cmd_full_matches() {
        hook_init_fake();
        let setup = &|c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_362_011, 1);
            c.pc = 0x0042_5d38;
            c.bus.write32(0x0045_0000 + 0x517c, 0);
            for i in 0..0x70u32 {
                c.bus.write32(0x0052_bcc0 + 4 * i, 0x5d5d_0000 ^ (4 * i));
            }
            if seed % 4 != 3 {
                c.bus.write32(INIT_GATE, GATE_MAGIC);
            }
            c.bus.write32(INIT_POLL_N, 1 + seed % 3);
        };
        check_paths(
            ee::s15::f_00425d38__generated,
            sce_sif_init_cmd,
            SEEDS,
            &[536, 766, 823, 824, 828],
            setup,
        );
    }

    driven!(init_cmd_driven, sce_sif_init_cmd);

    driven!(init_cmd_generated_driven, ee::s15::f_00425d38__generated);

    /// `sceSifInitCmd` with events due: deliveries in the table loops
    /// and the peer poll deliver inline on both sides.
    #[test]
    fn init_cmd_events() {
        hook_init_fake();
        check_events(
            init_cmd_generated_driven,
            init_cmd_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_362_012, 0);
                c.pc = 0x0042_5d38;
                c.bus.write32(0x0045_0000 + 0x517c, 0);
                c.bus.write32(INIT_GATE, GATE_MAGIC);
                c.bus.write32(INIT_POLL_N, 2);
                c.g[T2][0] = 12;
                schedule(c, &mut r, seed, 0x0042_5d38)
            },
        );
    }
}
