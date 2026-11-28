//! `src/lib/kernel.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::lib::gcc::frame_ld;
use crate::game::lib::gcc::frame_sd;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_39c0, exit, "_Exit"),
    (0x0042_39e0, load_exec_ps2, "_LoadExecPS2"),
    (0x0042_3a80, add_intc_handler, "AddIntcHandler"),
    (0x0042_3a90, add_intc_handler_2, "AddIntcHandler"),
    (0x0042_3ab0, add_dmac_handler, "AddDmacHandler"),
    (0x0042_3ac0, add_dmac_handler_2, "AddDmacHandler"),
    (0x0042_3ae0, enable_intc, "_EnableIntc"),
    (0x0042_3af0, disable_intc, "_DisableIntc"),
    (0x0042_3b00, enable_dmac, "_EnableDmac"),
    (0x0042_3b10, disable_dmac, "_DisableDmac"),
    (0x0042_3b40, i_enable_intc, "_iEnableIntc"),
    (0x0042_3b50, i_disable_intc, "_iDisableIntc"),
    (0x0042_3b60, i_enable_dmac, "_iEnableDmac"),
    (0x0042_3b70, i_disable_dmac, "_iDisableDmac"),
    (0x0042_3bc0, start_thread, "_StartThread"),
    (0x0042_3c90, get_thread_id, "GetThreadId"),
    (0x0042_3ce0, i_wakeup_thread, "_iWakeupThread"),
    (0x0042_3d20, i_suspend_thread, "_iSuspendThread"),
    (0x0042_3dc0, signal_sema, "SignalSema"),
    (0x0042_3dd0, i_signal_sema, "iSignalSema"),
    (0x0042_3de0, wait_sema, "WaitSema"),
    (0x0042_3df0, poll_sema, "PollSema"),
    (0x0042_4020, flush_cache, "FlushCache"),
    (0x0042_40e0, gs_put_imr, "GsPutIMR"),
    (0x0042_41c0, exec_osd, "_ExecOSD"),
    (0x0042_4210, init_tlb, "_InitTLB"),
    (0x0042_45f0, sync_dcache, "_SyncDCache"),
    (0x0042_4730, invalid_dcache, "_InvalidDCache"),
    (0x0042_47d8, invalid_dcache_2, "InvalidDCache"),
    (0x0042_4698, sync_dcache_2, "SyncDCache"),
    (0x0010_01c8, crt_exit, "_exit"),
    (0x0042_c628, init_tlb_2, "InitTLB"),
    (0x0042_c6a0, load_exec_ps2_2, "LoadExecPS2"),
    (0x0042_c6e8, exit_2, "Exit"),
    (0x0042_c710, exec_osd_2, "ExecOSD"),
    (0x0042_cd58, init_tlb_3, "InitTLB"),
    (
        0x0042_c0d8,
        libkernl_init_malloc_sema,
        "libkernl_InitMallocSema",
    ),
    (0x0042_c120, kcopy, "kCopy"),
    (0x0042_c1a8, find_address, "FindAddress"),
    (0x0042_c1b8, get_syscall_handler, "GetSyscallHandler"),
    (
        0x0042_c1f0,
        libkernl_init_syscall_table,
        "libkernl_InitSyscallTable",
    ),
    (0x0042_c2f0, rfu116_set_syscall_5, "RFU116_SetSyscall"),
    (0x0042_c340, rfu116_set_syscall_4, "RFU116_SetSyscall"),
    (0x0042_c350, kcopy_2, "kCopy_2"),
    (0x0042_c398, rfu091_3, "RFU091"),
    (
        0x0042_c3a8,
        libkernl_is_old_osd_config,
        "libkernl_IsOldOsdConfig",
    ),
    (0x0042_c410, init_osd, "InitOsd"),
    (0x0042_c4c0, kcopy_3, "kCopy_3"),
    (0x0042_c500, rfu116_set_syscall_3, "RFU116_SetSyscall"),
    (
        0x0042_c510,
        libkernl_copy_args_to_kernel,
        "libkernl_CopyArgsToKernel",
    ),
    (0x0042_c760, rfu116_set_syscall_2, "RFU116_SetSyscall"),
    (0x0042_c770, kcopy_4, "kCopy_4"),
    (0x0042_c7b8, rfu091_2, "RFU091"),
    (0x0042_c7c8, init_alarm, "InitAlarm"),
    (
        0x0042_ca80,
        libkernl_install_tlb_refill_handlers,
        "libkernl_InstallTLBRefillHandlers",
    ),
    (0x0042_cb68, kcopy_5, "kCopy_5"),
    (0x0042_cbb0, rfu091, "RFU091"),
    (0x0042_cbc0, rfu116_set_syscall, "RFU116_SetSyscall"),
    (0x0042_cbd0, init_exec_ps2, "InitExecPS2"),
    (0x0042_ccb8, rfu086_wait_evnet_flag, "RFU086_WaitEvnetFlag"),
    (0x0042_ccc8, set_tlb_entry, "SetTLBEntry"),
    (0x0042_cd98, libkernl_setup_tlb, "libkernl_SetupTLB"),
    (0x0042_c300, init_sys, "_InitSys"),
    (
        0x0042_e508,
        rcmp_av_ms_timer_update,
        "RCMP_AV_MS_TIMER_Update",
    ),
    (0x0041_1310, exit_3, "exit"),
    (
        0x0042_4220,
        libkernl_reset_tty_init,
        "libkernl_ResetTtyInit",
    ),
    (0x0042_4e00, libkernl_tty_ring_init, "libkernl_TtyRingInit"),
    (0x0042_4e28, libkernl_tty_ring_push, "libkernl_TtyRingPush"),
    (0x0042_4e68, libkernl_tty_ring_pop, "libkernl_TtyRingPop"),
    (0x0042_5320, libkernl_sio_putc, "libkernl_SioPutc"),
    (0x0042_5408, libkernl_sio_putc_crlf, "libkernl_SioPutcCRLF"),
    (0x0042_5c28, libkernl_printf, "libkernl_printf"),
    (0x0042_3cd0, wakeup_thread, "WakeupThread"),
    (0x0042_3d10, suspend_thread, "SuspendThread"),
    (
        0x0042_4230,
        libkernl_wait_vblank_start,
        "libkernl_WaitVBlankStart",
    ),
    (
        0x0042_42c0,
        libkernl_wait_vblank_or_flag,
        "libkernl_WaitVBlankOrFlag",
    ),
    (0x0042_4880, disable_intc_guarded, "DisableIntc"),
    (0x0042_48e8, enable_intc_guarded, "EnableIntc"),
    (
        0x0042_4b78,
        libkernl_init_thread_helper,
        "libkernl_InitThreadHelper",
    ),
    (
        0x0042_4ea8,
        libkernl_tty_deci2_handler,
        "libkernl_TtyDeci2Handler",
    ),
    (0x0042_5040, libkernl_tty_write, "libkernl_TtyWrite"),
    (0x0042_5190, libkernl_tty_read, "libkernl_TtyRead"),
    (0x0042_5260, libkernl_tty_init, "libkernl_TtyInit"),
    (
        0x0042_5358,
        libkernl_kputc_buffered,
        "libkernl_KputcBuffered",
    ),
    (0x0042_5440, libkernl_double_to_int, "libkernl_DoubleToInt"),
    (0x0042_54d0, libkernl_print_double, "libkernl_PrintDouble"),
    (0x0042_5638, libkernl_vprintf, "libkernl_vprintf"),
    (
        0x0042_cfc0,
        libkernl_tlb_exception_handler,
        "libkernl_TLBExceptionHandler",
    ),
    (
        0x0042_d100,
        libkernl_exception_return_handler,
        "libkernl_ExceptionReturnHandler",
    ),
    (
        0x0042_4aa0,
        libkernl_intr_helper_thread_2,
        "libkernl_IntrHelperThread",
    ),
    (0x0042_c4d0, kcopy_3_impl, "kCopy_3_Impl"),
    (0x0042_c780, kcopy_4_impl, "kCopy_4_Impl"),
    (0x0042_cb78, kcopy_5_impl, "kCopy_5_Impl"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(super) use crate::game::lib::libc::callee::STRLEN;
    /// `EIntr` (`0x0042_c0c0`; the callee form of [`super::EINTR`]).
    pub(super) use crate::game::lib::sif::callee::EINTR;

    /// `libkernl_vprintf` (`0x00425638`).
    pub(crate) const VPRINTF: Callee = Callee {
        addr: 0x0042_5638,
        f: ee::s15::f_00425638,
    };

    /// Direct-call target: f_00423a50 (0x0042_3a50).
    pub(crate) const SET_HANDLER: Callee = Callee {
        addr: 0x0042_3a50,
        f: ee::s15::f_00423a50,
    };

    /// Direct-call target: CreateSema (0x0042_3da0).
    pub(crate) const CREATE_SEMA: Callee = Callee {
        addr: 0x0042_3da0,
        f: ee::s15::CreateSema,
    };

    /// Direct-call target: f_00423e40 (0x0042_3e40).
    pub(crate) const SET_PARAM: Callee = Callee {
        addr: 0x0042_3e40,
        f: ee::s15::f_00423e40,
    };

    /// Direct-call target: f_00423e50 (0x0042_3e50).
    pub(crate) const GET_PARAM: Callee = Callee {
        addr: 0x0042_3e50,
        f: ee::s15::f_00423e50,
    };

    /// Direct-call target: f_00424020 (0x0042_4020).
    pub(crate) const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::f_00424020,
    };

    /// Direct-call target: f_00424200 (0x0042_4200).
    pub(crate) const GET_MEM_SIZE: Callee = Callee {
        addr: 0x0042_4200,
        f: ee::s15::f_00424200,
    };

    /// Direct-call target: f_00424210 (0x0042_4210).
    pub(crate) const WRITE_TLB: Callee = Callee {
        addr: 0x0042_4210,
        f: ee::s15::f_00424210,
    };

    /// Direct-call target: f_004245f0 (0x0042_45f0).
    pub(crate) const SYNC_DCACHE_LINES: Callee = Callee {
        addr: 0x0042_45f0,
        f: ee::s15::f_004245f0,
    };

    /// Direct-call target: f_00424b78 (0x0042_4b78).
    pub(crate) const INIT_THREAD_HELPER: Callee = Callee {
        addr: 0x0042_4b78,
        f: ee::s15::f_00424b78,
    };

    /// Direct-call target: f_00425c28 (0x0042_5c28).
    pub(crate) const PRINTF: Callee = Callee {
        addr: 0x0042_5c28,
        f: ee::s15::f_00425c28,
    };

    /// Direct-call target: f_0042c0d8 (0x0042_c0d8).
    pub(crate) const INIT_MALLOC_SEMA: Callee = Callee {
        addr: 0x0042_c0d8,
        f: ee::s15::f_0042c0d8,
    };

    /// Direct-call target: kCopy (0x0042_c120).
    pub(crate) const KCOPY: Callee = Callee {
        addr: 0x0042_c120,
        f: ee::s15::kCopy,
    };

    /// Direct-call target: f_0042c1a8 (0x0042_c1a8).
    pub(crate) const FIND_ADDRESS: Callee = Callee {
        addr: 0x0042_c1a8,
        f: ee::s15::f_0042c1a8,
    };

    /// Direct-call target: libkernl_InitSyscallTable (0x0042_c1f0).
    pub(crate) const INIT_SYSCALL_TABLE: Callee = Callee {
        addr: 0x0042_c1f0,
        f: ee::s15::libkernl_InitSyscallTable,
    };

    /// RFU116 set-syscall stub. (`0x0042_c2f0`).
    pub(crate) const RFU116_SET_SYSCALL_C2F0: Callee = Callee {
        addr: 0x0042_c2f0,
        f: ee::s15::f_0042c2f0,
    };

    /// RFU116 set-syscall stub. (`0x0042_c340`).
    pub(crate) const RFU116_SET_SYSCALL_C340: Callee = Callee {
        addr: 0x0042_c340,
        f: ee::s15::f_0042c340,
    };

    /// Direct-call target: kCopy_2 (0x0042_c350).
    pub(crate) const KCOPY_2: Callee = Callee {
        addr: 0x0042_c350,
        f: ee::s15::kCopy_2,
    };

    /// RFU091 wait stub. (`0x0042_c398`).
    pub(crate) const RFU091_3: Callee = Callee {
        addr: 0x0042_c398,
        f: ee::s15::f_0042c398,
    };

    /// Direct-call target: libkernl_IsOldOsdConfig (0x0042_c3a8).
    pub(crate) const IS_OLD: Callee = Callee {
        addr: 0x0042_c3a8,
        f: ee::s15::libkernl_IsOldOsdConfig,
    };

    /// Direct-call target: f_0042c410 (0x0042_c410).
    pub(crate) const INIT_OSD: Callee = Callee {
        addr: 0x0042_c410,
        f: ee::s15::f_0042c410,
    };

    /// Direct-call target: kCopy_3 (0x0042_c4c0).
    pub(crate) const KCOPY_3: Callee = Callee {
        addr: 0x0042_c4c0,
        f: ee::s15::kCopy_3,
    };

    /// RFU116 set-syscall stub. (`0x0042_c500`).
    pub(crate) const RFU116_SET_SYSCALL_C500: Callee = Callee {
        addr: 0x0042_c500,
        f: ee::s15::f_0042c500,
    };

    /// Direct-call target: libkernl_CopyArgsToKernel (0x0042_c510).
    pub(crate) const COPY_ARGS: Callee = Callee {
        addr: 0x0042_c510,
        f: ee::s15::libkernl_CopyArgsToKernel,
    };

    /// Direct-call target: f_0042c628 (0x0042_c628).
    pub(crate) const INIT_TLB_FWD: Callee = Callee {
        addr: 0x0042_c628,
        f: ee::s15::f_0042c628,
    };

    /// Direct-call target: f_0042c6e8 (0x0042_c6e8).
    pub(crate) const EXIT_NOW: Callee = Callee {
        addr: 0x0042_c6e8,
        f: ee::s15::f_0042c6e8,
    };

    /// RFU116 set-syscall stub. (`0x0042_c760`).
    pub(crate) const RFU116_SET_SYSCALL_C760: Callee = Callee {
        addr: 0x0042_c760,
        f: ee::s15::f_0042c760,
    };

    /// Direct-call target: kCopy_4 (0x0042_c770).
    pub(crate) const KCOPY_4: Callee = Callee {
        addr: 0x0042_c770,
        f: ee::s15::kCopy_4,
    };

    /// RFU091 wait stub. (`0x0042_c7b8`).
    pub(crate) const RFU091_2: Callee = Callee {
        addr: 0x0042_c7b8,
        f: ee::s15::f_0042c7b8,
    };

    /// Direct-call target: f_0042c7c8 (0x0042_c7c8).
    pub(crate) const INIT_ALARM: Callee = Callee {
        addr: 0x0042_c7c8,
        f: ee::s15::f_0042c7c8,
    };

    /// Direct-call target: kCopy_5 (0x0042_cb68).
    pub(crate) const KCOPY_5: Callee = Callee {
        addr: 0x0042_cb68,
        f: ee::s15::kCopy_5,
    };

    /// RFU091 wait stub. (`0x0042_cbb0`).
    pub(crate) const RFU091: Callee = Callee {
        addr: 0x0042_cbb0,
        f: ee::s15::f_0042cbb0,
    };

    /// RFU116 set-syscall stub. (`0x0042_cbc0`).
    pub(crate) const RFU116_SET_SYSCALL_CBC0: Callee = Callee {
        addr: 0x0042_cbc0,
        f: ee::s15::f_0042cbc0,
    };

    /// Direct-call target: f_0042cbd0 (0x0042_cbd0).
    pub(crate) const INIT_EXEC: Callee = Callee {
        addr: 0x0042_cbd0,
        f: ee::s15::f_0042cbd0,
    };

    /// Wait for the TLB event flag (RFU086). (`0x0042_ccb8`).
    pub(crate) const WAIT_FLAG: Callee = Callee {
        addr: 0x0042_ccb8,
        f: ee::s15::RFU086_WaitEvnetFlag,
    };

    /// Direct-call target: libkernl_SetupTLB (0x0042_cd98).
    pub(crate) const SETUP_TLB: Callee = Callee {
        addr: 0x0042_cd98,
        f: ee::s15::libkernl_SetupTLB,
    };

    /// `libkernl_TtyRingPush` (`0x00424e28`).
    pub(crate) const TTY_RING_PUSH: Callee = Callee {
        addr: 0x0042_4e28,
        f: ee::s15::f_00424e28,
    };

    /// `libkernl_TtyRingPop` (`0x00424e68`).
    pub(crate) const TTY_RING_POP: Callee = Callee {
        addr: 0x0042_4e68,
        f: ee::s15::f_00424e68,
    };

    /// `libkernl_TtyRingInit` (`0x00424e00`).
    pub(crate) const TTY_RING_INIT: Callee = Callee {
        addr: 0x0042_4e00,
        f: ee::s15::f_00424e00,
    };

    /// `sceDeci2ExRecv` (`0x0042c968`).
    pub(crate) const DECI2_EX_RECV: Callee = Callee {
        addr: 0x0042_c968,
        f: ee::s15::f_0042c968,
    };

    /// `sceDeci2ExSend` (`0x0042c9a0`).
    pub(crate) const DECI2_EX_SEND: Callee = Callee {
        addr: 0x0042_c9a0,
        f: ee::s15::f_0042c9a0,
    };

    /// `sceDeci2ReqSend` (`0x0042c910`).
    pub(crate) const DECI2_REQ_SEND: Callee = Callee {
        addr: 0x0042_c910,
        f: ee::s15::f_0042c910,
    };

    /// `sceDeci2Poll` (`0x0042c940`).
    pub(crate) const DECI2_POLL: Callee = Callee {
        addr: 0x0042_c940,
        f: ee::s15::f_0042c940,
    };

    /// `sceDeci2Open` (`0x0042c8a0`).
    pub(crate) const DECI2_OPEN: Callee = Callee {
        addr: 0x0042_c8a0,
        f: ee::s15::f_0042c8a0,
    };

    /// `Deci2Kputs` (`0x0042ca58`).
    pub(crate) const DECI2_KPUTS: Callee = Callee {
        addr: 0x0042_ca58,
        f: ee::s15::f_0042ca58,
    };

    pub(crate) use crate::game::dirtysock::buddyapi::callee::{
        CREATE_THREAD, DELETE_SEMA, GET_THREAD_ID, START_THREAD, WAIT_SEMA, WAKEUP_THREAD,
    };

    /// `RotateThreadReadyQueue` (`0x00423c50`).
    pub(crate) const ROTATE_QUEUE: Callee = Callee {
        addr: 0x0042_3c50,
        f: ee::s15::f_00423c50,
    };

    /// `SuspendThread` (`0x00423d10`).
    pub(crate) const SUSPEND_THREAD: Callee = Callee {
        addr: 0x0042_3d10,
        f: ee::s15::f_00423d10,
    };

    pub(crate) use crate::game::lib::cdvd::callee::CHANGE_THREAD_PRIORITY;

    /// `SetVSyncFlag` (`0x00424110`).
    pub(crate) const SET_VSYNC_FLAG: Callee = Callee {
        addr: 0x0042_4110,
        f: ee::s15::f_00424110,
    };

    /// `_DisableIntc` (`0x00423af0`).
    pub(crate) const DISABLE_INTC_FN: Callee = Callee {
        addr: 0x0042_3af0,
        f: ee::s15::f_00423af0,
    };

    /// `_EnableIntc` (`0x00423ae0`).
    pub(crate) const ENABLE_INTC_FN: Callee = Callee {
        addr: 0x0042_3ae0,
        f: ee::s15::f_00423ae0,
    };
    pub(crate) use crate::game::lib::libc::callee::CLEANUP;
    pub(crate) use crate::game::lib::libc::callee::DO_GLOBAL_DTORS;
}

// ---------------------------------------------------------------------------
// Kernel syscall wrappers. Each sets the syscall code in `v1`, traps (the
// return address is `ret`, the trap itself at `ret - 4`), delivers any due
// event and returns whatever the trap left in `v0`.

/// The `syscall` trap: `pc = ret`, invoke the syscall hook at `at`; `false`
/// when the hook unwound (a thread switch mid-syscall).
#[inline]
pub(crate) fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The event check the syscall wrappers make just before returning (the
/// result of `event` is ignored, as compiled code ignores it).
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

/// `void _Exit(int status)` (0x4239c0): syscall 4.
pub fn exit(c: &mut Cpu) {
    syscall_wrapper(c, 4, 0x0042_39c4, 0x0042_39c8);
}

/// `void _LoadExecPS2(const char *path, int argc, char **argv)` (0x4239e0):
/// syscall 6.
pub fn load_exec_ps2(c: &mut Cpu) {
    syscall_wrapper(c, 6, 0x0042_39e4, 0x0042_39e8);
}

/// `int _EnableIntc(int intr)` (0x423ae0): syscall 0x14.
pub fn enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x14, 0x0042_3ae4, 0x0042_3ae8);
}

/// `int _DisableIntc(int intr)` (0x423af0): syscall 0x15.
pub fn disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x15, 0x0042_3af4, 0x0042_3af8);
}

/// `int _EnableDmac(int channel)` (0x423b00): syscall 0x16.
pub fn enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x16, 0x0042_3b04, 0x0042_3b08);
}

/// `int _DisableDmac(int channel)` (0x423b10): syscall 0x17.
pub fn disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x17, 0x0042_3b14, 0x0042_3b18);
}

/// `int _iEnableIntc(int intr)` (0x423b40): interrupt-disabled syscall -26.
pub fn i_enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-26i32) as u32, 0x0042_3b44, 0x0042_3b48);
}

/// `int _iDisableIntc(int intr)` (0x423b50): syscall -27.
pub fn i_disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-27i32) as u32, 0x0042_3b54, 0x0042_3b58);
}

/// `int _iEnableDmac(int channel)` (0x423b60): syscall -28.
pub fn i_enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-28i32) as u32, 0x0042_3b64, 0x0042_3b68);
}

/// `int _iDisableDmac(int channel)` (0x423b70): syscall -29.
pub fn i_disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-29i32) as u32, 0x0042_3b74, 0x0042_3b78);
}

/// `int _StartThread(struct thread *th, void *arg)` (0x423bc0): syscall 0x22.
pub fn start_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x22, 0x0042_3bc4, 0x0042_3bc8);
}

/// `int _iWakeupThread(int thid)` (0x423ce0): syscall -52.
pub fn i_wakeup_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-52i32) as u32, 0x0042_3ce4, 0x0042_3ce8);
}

/// `int _iSuspendThread(int thid)` (0x423d20): syscall -56.
pub fn i_suspend_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-56i32) as u32, 0x0042_3d24, 0x0042_3d28);
}

/// `u64 GsPutIMR(u64 imr)` (0x4240e0): syscall 0x71 (set the GS interrupt
/// mask).
pub fn gs_put_imr(c: &mut Cpu) {
    syscall_wrapper(c, 0x71, 0x0042_40e4, 0x0042_40e8);
}

/// `void _ExecOSD(int argc, char **argv)` (0x4241c0): syscall 0x7b.
pub fn exec_osd(c: &mut Cpu) {
    syscall_wrapper(c, 0x7b, 0x0042_41c4, 0x0042_41c8);
}

/// `void _InitTLB(void)` (0x424210): syscall 0x82.
pub fn init_tlb(c: &mut Cpu) {
    syscall_wrapper(c, 0x82, 0x0042_4214, 0x0042_4218);
}

/// One pass of the cache-range loop: compute `address` from the CP0 tag
/// register (`mfc0 $2, $28`), masked to a page and offset by the index, and
/// classify both sub-blocks (cycle count only; the branches always fall
/// through). Returns the cycles charged.
#[inline]
pub(crate) fn cache_pass(c: &mut Cpu) {
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
}

/// The shared body of `_SyncDCache`/`_InvalidDCache`: walk 4 KiB in 0x40
/// steps; `head` is the loop head for event delivery.
pub(crate) fn cache_range(c: &mut Cpu, head: u32) {
    c.cycles += 4;
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] |= 0xf000;
    loop {
        cache_pass(c);
        c.cycles += 5;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4096) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            crate::events::deliver_event_here(c, head);
        }
    }
}

/// `void _SyncDCache(void *start, void *end)` (0x4245f0): write back the
/// cache lines covering `[start, end)`.
pub fn sync_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4600);
}

/// `void _InvalidDCache(void *start, void *end)` (0x424730): invalidate the
/// cache lines covering `[start, end)`.
pub fn invalid_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4740);
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap, a due-event check and
/// `jr ra`.
#[inline]
pub(crate) fn syscall_wrapper_2(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
    abi::ret(c);
}

/// `int AddIntcHandler(int intr, int (*handler)(), int arg, int flag)`
/// (0x423a80): syscall 0x10.
pub fn add_intc_handler(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x10, 0x0042_3a84, 0x0042_3a88);
}

/// `int AddIntcHandler(...)` (0x423a90): identical syscall-0x10 wrapper.
pub fn add_intc_handler_2(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x10, 0x0042_3a94, 0x0042_3a98);
}

/// `int AddDmacHandler(int channel, int (*handler)(), int arg, int flag)`
/// (0x423ab0): syscall 0x12.
pub fn add_dmac_handler(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x12, 0x0042_3ab4, 0x0042_3ab8);
}

/// `int AddDmacHandler(...)` (0x423ac0): identical syscall-0x12 wrapper.
pub fn add_dmac_handler_2(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x12, 0x0042_3ac4, 0x0042_3ac8);
}

/// `int GetThreadId(void)` (0x423c90): syscall 0x2f, then one event check
/// (the generated code tests for a due event before returning).
pub fn get_thread_id(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x2f;
    c.pc = 0x0042_3c98;
    c.syscall(0x0042_3c94);
    if c.pc != 0x0042_3c98 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int SignalSema(int sema)` (0x423dc0): syscall 0x42.
pub fn signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x42;
    c.pc = 0x0042_3dc8;
    c.syscall(0x0042_3dc4);
    if c.pc != 0x0042_3dc8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int iSignalSema(int sema)` (0x423dd0): syscall 0x43 (the interrupt-level
/// variant; the generated code leaves `v1` sign-extended from 0xffffffbd,
/// i.e. the 64-bit syscall word -0x43).
pub fn i_signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xffff_ffff_ffff_ffbd;
    c.pc = 0x0042_3dd8;
    c.syscall(0x0042_3dd4);
    if c.pc != 0x0042_3dd8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int WaitSema(int sema)` (0x423de0): syscall 0x44.
pub fn wait_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x44;
    c.pc = 0x0042_3de8;
    c.syscall(0x0042_3de4);
    if c.pc != 0x0042_3de8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `void FlushCache(int mode)` (0x424020): syscall 0x64.
pub fn flush_cache(c: &mut Cpu) {
    syscall_wrapper(c, 0x64, 0x0042_4024, 0x0042_4028);
}

/// `int PollSema(int sema)` (0x423df0): syscall 0x45.
pub fn poll_sema(c: &mut Cpu) {
    syscall_wrapper(c, 0x45, 0x0042_3df4, 0x0042_3df8);
}

// ---------------------------------------------------------------------------
// InvalidDCache (0x4247d8)

/// `DIntr()` (0x42c078): disable interrupts, return whether they were on.
pub(crate) const DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};

/// `libkernl_SioPutc` (`0x0042_5320`).
pub(crate) const SIO_PUTC: Callee = Callee {
    addr: 0x0042_5320,
    f: ee::s15::libkernl_SioPutc,
};

/// `_InvalidDCache(start, end)` (0x424730): the cache-line loop.
pub(crate) const INVALID_DCACHE_LINES: Callee = Callee {
    addr: 0x0042_4730,
    f: ee::s15::f_00424730,
};

/// `EIntr()` (0x42c0c0).
pub(crate) const EINTR: u32 = 0x0042_c0c0;

/// `void InvalidDCache(void* start, void* end)` (0x4247d8, libkernel):
/// invalidate the data cache lines covering [start, end] (both rounded down
/// to 64 bytes) with interrupts disabled; re-enables them (tail call to
/// `EIntr`) if they were enabled on entry (Status.EIE, COP0 r12 bit 16).
pub fn invalid_dcache_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let at = frame.at(c, 0x20);
        c.sd(at, c.g[S2][0]);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] &= 0x10000;
        let enabled = c.g[S0][0] != 0;
        if enabled {
            c.cycles += 2;
            DINTR.call(c, 0x0042_4810)?;
        }
        c.cycles += 7;
        const LINE_MASK: u64 = 0xffff_ffff_ffff_ffc0;
        c.g[A1][0] = c.g[S1][0] & LINE_MASK;
        c.g[A0][0] = c.g[S2][0] & LINE_MASK;
        INVALID_DCACHE_LINES.call(c, 0x0042_4824)?;
        frame.restore_ra(c, 0x30);
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if enabled {
            // Tail call: EIntr returns to our caller.
            c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
            c.pc = EINTR;
            ee::s15::f_0042c0c0(c);
        } else {
            frame.close(c);
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SyncDCache (0x424698)

/// `void SyncDCache(void* start, void* end)` (0x424698, libkernel): write
/// back the data cache lines covering [start, end] (both rounded down to 64
/// bytes, `_SyncDCache` at 0x4245f0) with interrupts disabled; re-enables
/// them (tail call to `EIntr`) if they were enabled on entry (Status.EIE,
/// COP0 r12 bit 16). Same shape as `InvalidDCache`.
pub fn sync_dcache_2(c: &mut Cpu) {
    use crate::game::lib::kernel::{DINTR, EINTR};

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let at = frame.at(c, 0x20);
        c.sd(at, c.g[S2][0]);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] &= 0x10000;
        let enabled = c.g[S0][0] != 0;
        if enabled {
            c.cycles += 2;
            DINTR.call(c, 0x0042_46d0)?;
        }
        c.cycles += 7;
        const LINE_MASK: u64 = 0xffff_ffff_ffff_ffc0;
        c.g[A0][0] = LINE_MASK;
        c.g[A1][0] = c.g[S1][0] & LINE_MASK;
        c.g[A0][0] = c.g[S2][0] & LINE_MASK;
        callee::SYNC_DCACHE_LINES.call(c, 0x0042_46e4)?;
        frame.restore_ra(c, 0x30);
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if enabled {
            // Tail call: EIntr returns to our caller.
            c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
            c.pc = EINTR;
            ee::s15::f_0042c0c0(c);
        } else {
            frame.close(c);
        }
        Ok(())
    })
}

// --------------------------------------------------------------------- _exit

/// `Exit` (0x42c6e8): kernel exit (`InitTLB`, then the `_Exit` syscall).
pub(crate) const CRT_EXIT_KERNEL: u32 = 0x0042_c6e8;

/// `void _exit(int)` (0x1001c8): the crt0 exit stub, `Exit(0)` (a tail
/// call; the status argument is ignored).
pub fn crt_exit(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = 0;
    c.pc = CRT_EXIT_KERNEL;
    ssx3_ee_rt::funcs::call_indirect(c, CRT_EXIT_KERNEL);
}

/// InitTLB (0x42cd58): when the machine has 32 MB (`GetMemorySize`), set up
/// the kernel TLB tables; otherwise write the fallback TLB and return.
pub fn init_tlb_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::GET_MEM_SIZE.call(c, 0x0042_cd68)?;
        c.g[V1][0] = 0x2000000;
        let big = c.g[V0][0] != c.g[V1][0];
        if !big {
            c.cycles += 4;
            callee::SETUP_TLB.call(c, 0x0042_cd7c)?;
            c.cycles += 2;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        callee::WRITE_TLB.call(c, 0x0042_cd8c)?;
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// ExecOSD (0x42c710): copies the OSD arguments into kernel RAM, runs the
/// `InitTLB` forwarder, then tail-calls the already-ported `_ExecOSD`.
pub fn exec_osd_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 0x4a0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5270));
        callee::COPY_ARGS.call(c, 0x0042_c734)?;
        callee::INIT_TLB_FWD.call(c, 0x0042_c73c)?;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6538));
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = 0x0042_41c0;
        ee::s15::f_004241c0(c);
        Ok(())
    })
}

/// Exit (0x42c6e8): runs the `InitTLB` forwarder, then tail-calls the
/// already-ported `_Exit` with the saved `a0`.
pub fn exit_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        callee::INIT_TLB_FWD.call(c, 0x0042_c6fc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = 0x0042_39c0;
        ee::s15::f_004239c0(c);
        Ok(())
    })
}

/// LoadExecPS2 (0x42c6a0): copies the boot arguments into kernel RAM, runs the
/// `InitTLB` forwarder, then tail-calls the already-ported `_LoadExecPS2`
/// with the executable path and the relocated argument vector.
pub fn load_exec_ps2_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        callee::COPY_ARGS.call(c, 0x0042_c6b8)?;
        c.g[S0][0] = c.g[V0][0];
        callee::INIT_TLB_FWD.call(c, 0x0042_c6c0)?;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6538));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = 0x0042_39e0;
        ee::s15::f_004239e0(c);
        Ok(())
    })
}

/// InitTLB (0x42c628): 8-byte forwarder that tail-calls the real `InitTLB`
/// (0x42cd58, which gates on `GetMemorySize`).
pub fn init_tlb_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        c.pc = 0x0042_cd58;
        ee::s15::f_0042cd58(c);
        Ok(())
    })
}

/// libkernl_InitMallocSema (0x42c0d8): creates the kernel heap semaphore and
/// the malloc semaphore, publishing both semaphore ids at `0x455248` and `0x45524c`.
pub fn libkernl_init_malloc_sema(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x1;
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        c.sw(sp(c).wrapping_add(0x28), c.g[V0][0]);
        c.sw(sp(c).wrapping_add(0x4), c.g[V0][0]);
        c.sw(sp(c).wrapping_add(0x8), c.g[V0][0]);
        c.sw(sp(c).wrapping_add(0x24), c.g[V0][0]);
        callee::CREATE_SEMA.call(c, 0x0042_c0fc)?;
        c.g[V1][0] = 0x450000;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x5248), c.g[V0][0]);
        callee::CREATE_SEMA.call(c, 0x0042_c10c)?;
        c.g[V1][0] = 0x450000;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x524c), c.g[V0][0]);
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ret;
        Ok(())
    })
}

/// Kernel copy syscall 0x5a (`kCopy`, 0x42c120): sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn kcopy(c: &mut Cpu) {
    syscall_wrapper(c, 0x5a, 0x0042_c124, 0x0042_c128);
}

/// FindAddress (0x42c1a8): reserved syscall 0x83: sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn find_address(c: &mut Cpu) {
    syscall_wrapper(c, 0x83, 0x0042_c1ac, 0x0042_c1b0);
}

/// GetSyscallHandler (0x42c1b8): reads one word through the pointer at
/// `0x455230` (plus `a0 * 4`) with `kCopy` and returns it.
pub fn get_syscall_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5230));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::KCOPY.call(c, 0x0042_c1dc)?;
        c.g[V0][0] = c.lw(sp(c));
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// libkernl_InitSyscallTable (0x42c1f0): installs two handlers from the
/// table at `0x455238`, then alternates two `FindAddress` searches until
/// they agree and stores the result.
pub fn libkernl_init_syscall_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x50), c.g[S5][0]);
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[S5][0] = 0x430000;
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S4][0] = 0x430000;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5238));
        c.sd(frame.at(c, 0x60), c.g[S6][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5238));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::RFU116_SET_SYSCALL_C2F0.call(c, 0x0042_c230)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        callee::RFU116_SET_SYSCALL_C2F0.call(c, 0x0042_c23c)?;
        c.g[A0][0] = 0xffffffff80000000;
        c.g[A1][0] = 0xffffffff80080000;
        c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x3e98));
        callee::FIND_ADDRESS.call(c, 0x0042_c24c)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = 0xffffffff80000000;
        c.g[A1][0] = 0xffffffff80080000;
        c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x3ed0));
        callee::FIND_ADDRESS.call(c, 0x0042_c260)?;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x20c));
        c.g[S2][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x168));
        let converged = c.g[S1][0] == c.g[S0][0];
        c.g[S6][0] = 0x450000;
        if converged {
            c.cycles += 1;
            c.sw((c.g[S6][0] as u32).wrapping_add(0x5230), c.g[S1][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = (c.g[S1][0] < c.g[S0][0]) as u64;
            loop {
                c.cycles += 2;
                let at_end = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                if !at_end {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffffffff80080000;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x3e98));
                    callee::FIND_ADDRESS.call(c, 0x0042_c28c)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x20c));
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0xffffffff80080000;
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x3ed0));
                    callee::FIND_ADDRESS.call(c, 0x0042_c2a8)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x168));
                }
                c.cycles += 2;
                let done = c.g[S1][0] != c.g[S0][0];
                c.g[V0][0] = (c.g[S1][0] < c.g[S0][0]) as u64;
                if !done {
                    c.cycles += 2;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x5230), c.g[S1][0]);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0042_c278);
                }
            }
        }
        c.cycles += 10;
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        c.g[S6][0] = c.ld(frame.at(c, 0x60));
        c.g[S5][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// Reserved syscall 0x74 (`RFU116_SetSyscall`, 0x42c2f0): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu116_set_syscall_5(c: &mut Cpu) {
    syscall_wrapper(c, 0x74, 0x0042_c2f4, 0x0042_c2f8);
}

/// Reserved syscall 0x74 (`RFU116_SetSyscall`, 0x42c340): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu116_set_syscall_4(c: &mut Cpu) {
    syscall_wrapper(c, 0x74, 0x0042_c344, 0x0042_c348);
}

/// Reserved syscall 0x74 (`RFU116_SetSyscall`, 0x42c500): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu116_set_syscall_3(c: &mut Cpu) {
    syscall_wrapper(c, 0x74, 0x0042_c504, 0x0042_c508);
}

/// Kernel copy syscall 0x5a (`kCopy_2`, 0x42c350): sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn kcopy_2(c: &mut Cpu) {
    syscall_wrapper(c, 0x5a, 0x0042_c354, 0x0042_c358);
}

/// Reserved syscall 0x5b (`RFU091`, 0x42c398): sets `v1`, traps, delivers a
/// due event and returns through `ra`.
pub fn rfu091_3(c: &mut Cpu) {
    syscall_wrapper(c, 0x5b, 0x0042_c39c, 0x0042_c3a0);
}

/// libkernl_IsOldOsdConfig (0x42c3a8): probes the OSD config version by
/// writing a test pattern through `SetOsdConfigParam` / `GetOsdConfigParam`;
/// returns whether the stored version predates the current one.
pub fn libkernl_is_old_osd_config(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[SP][0];
        callee::GET_PARAM.call(c, 0x0042_c3bc)?;
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V0][0] = c.g[V0][0] | 0x1fff;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x4));
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.g[V1][0] | 0x2000;
        c.sw(sp(c).wrapping_add(0x4), c.g[V1][0]);
        callee::SET_PARAM.call(c, 0x0042_c3e0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::GET_PARAM.call(c, 0x0042_c3e8)?;
        c.g[A0][0] = c.g[SP][0];
        callee::SET_PARAM.call(c, 0x0042_c3f0)?;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x4));
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        c.g[V0][0] = c.g[V0][0] & 0x7;
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ret;
        Ok(())
    })
}

/// InitOsd (0x42c410): if the OSD config is old, installs the OSD boot
/// handler (a `kCopy` of the handler template, cache flushes, then
/// `RFU116`/`RFU091` syscalls) and returns through the entry epilogue.
pub fn init_osd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save_ra(c, 0x30);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        callee::IS_OLD.call(c, 0x0042_c428)?;
        let fresh = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x450000;
        if !fresh {
            c.cycles += 20;
            c.g[S2][0] = 0x2;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x59f8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x59f8));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            callee::RFU116_SET_SYSCALL_C340.call(c, 0x0042_c448)?;
            c.g[A1][0] = 0x450000;
            c.g[A0][0] = 0xffffffff80070000;
            c.g[A2][0] = 0x7a8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5250));
            c.g[A0][0] = c.g[A0][0] | 0x4000;
            callee::KCOPY_2.call(c, 0x0042_c460)?;
            c.g[A0][0] = 0u64;
            callee::FLUSH_CACHE.call(c, 0x0042_c468)?;
            c.g[A0][0] = 0x2;
            callee::FLUSH_CACHE.call(c, 0x0042_c470)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            callee::RFU116_SET_SYSCALL_C340.call(c, 0x0042_c47c)?;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            loop {
                c.cycles += 9;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                callee::RFU091_3.call(c, 0x0042_c488)?;
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A1][0] = c.g[V0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                callee::RFU116_SET_SYSCALL_C340.call(c, 0x0042_c498)?;
                c.g[V0][0] = (c.g[S2][0] < 0x3u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_c480);
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// Kernel copy syscall 0x5a (`kCopy_3`, 0x42c4c0): sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn kcopy_3(c: &mut Cpu) {
    syscall_wrapper(c, 0x5a, 0x0042_c4c4, 0x0042_c4c8);
}

/// libkernl_CopyArgsToKernel (0x42c510): copies the boot argument strings
/// into the kernel argument area (at most 15 arguments besides the program
/// name), measuring each with `strlen` and copying with the `kCopy` syscall.
/// Returns `s5`, set to the area base + 0x40 before any copy; the area is
/// where the word at `0x456538` points.
pub fn libkernl_copy_args_to_kernel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[V1][0] = 0x450000;
        c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6538));
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5a10));
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S5][0] = c.g[V0][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0x70);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5a10));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.sw(sp(c), c.g[V0][0]);
        callee::RFU116_SET_SYSCALL_C500.call(c, 0x0042_c564)?;
        c.g[V1][0] = ((c.g[S2][0] as i64) < 16i64) as u64;
        c.g[V0][0] = 0xf;
        c.g[A0][0] = c.g[S3][0];
        if c.g[V1][0] == 0 {
            c.g[S2][0] = c.g[V0][0];
        }
        c.g[A1][0] = sx(sp(c));
        c.g[A2][0] = 0x4;
        callee::KCOPY_3.call(c, 0x0042_c580)?;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S0][0];
        callee::STRLEN.call(c, 0x0042_c58c)?;
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = c.lw(sp(c));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        callee::KCOPY_3.call(c, 0x0042_c5a0)?;
        c.g[V1][0] = c.lw(sp(c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        let done = (c.g[S2][0] as i64) <= 0;
        c.sw(sp(c), c.g[V1][0]);
        if done {
            // Epilogue: no more arguments.
        } else {
            c.cycles += 2;
            c.g[S0][0] = c.g[S4][0];
            loop {
                c.cycles += 18;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx(sp(c));
                c.g[A2][0] = 0x4;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                callee::KCOPY_3.call(c, 0x0042_c5cc)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                callee::STRLEN.call(c, 0x0042_c5d8)?;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lw(sp(c));
                c.g[A2][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                callee::KCOPY_3.call(c, 0x0042_c5f0)?;
                c.g[V1][0] = c.lw(sp(c));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                let more = c.g[S2][0] != 0u64;
                c.sw(sp(c), c.g[V1][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0042_c5b8);
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S5][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// Reserved syscall 0x74 (`RFU116_SetSyscall`, 0x42c760): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu116_set_syscall_2(c: &mut Cpu) {
    syscall_wrapper(c, 0x74, 0x0042_c764, 0x0042_c768);
}

/// Kernel copy syscall 0x5a (`kCopy_4`, 0x42c770): sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn kcopy_4(c: &mut Cpu) {
    syscall_wrapper(c, 0x5a, 0x0042_c774, 0x0042_c778);
}

/// Reserved syscall 0x5b (`RFU091`, 0x42c7b8): sets `v1`, traps, delivers a
/// due event and returns through `ra`.
pub fn rfu091_2(c: &mut Cpu) {
    syscall_wrapper(c, 0x5b, 0x0042_c7bc, 0x0042_c7c0);
}

/// InitAlarm (0x42c7c8): when Timer 3 mode (bit 8 of `0x10001810`) is clear,
/// installs the kernel alarm handler (one `RFU116`, two `kCopy`s of the
/// handler template, cache flushes, then `RFU091`/`RFU116` rounds) and
/// returns through the entry epilogue.
pub fn init_alarm(c: &mut Cpu) {
    const ALARM_FLAG: u32 = 0x1000_1810;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x10000000;
        frame.save_ra(c, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0x1810;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V1][0] = c.lw(ALARM_FLAG);
        c.g[V1][0] = c.g[V1][0] & 0x100;
        let armed = c.g[V1][0] != 0u64;
        frame.restore_ra(c, 0x30);
        if !armed {
            c.cycles += 27;
            c.g[V0][0] = 0x450000;
            c.g[S2][0] = 0x2;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6180));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6180));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            callee::RFU116_SET_SYSCALL_C760.call(c, 0x0042_c810)?;
            c.g[A1][0] = 0x450000;
            c.g[A0][0] = 0xffffffff80070000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a18));
            c.g[A0][0] = c.g[A0][0] | 0x6000;
            c.g[A2][0] = 0x740;
            callee::KCOPY_4.call(c, 0x0042_c828)?;
            c.g[A1][0] = 0x450000;
            c.g[A0][0] = 0x80000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6158));
            c.g[A0][0] = c.g[A0][0] | 0x2000;
            c.g[A2][0] = 0x28;
            callee::KCOPY_4.call(c, 0x0042_c840)?;
            c.g[A0][0] = 0u64;
            callee::FLUSH_CACHE.call(c, 0x0042_c848)?;
            c.g[A0][0] = 0x2;
            callee::FLUSH_CACHE.call(c, 0x0042_c850)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            callee::RFU116_SET_SYSCALL_C760.call(c, 0x0042_c85c)?;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            loop {
                c.cycles += 9;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                callee::RFU091_2.call(c, 0x0042_c868)?;
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A1][0] = c.g[V0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                callee::RFU116_SET_SYSCALL_C760.call(c, 0x0042_c878)?;
                c.g[V0][0] = (c.g[S2][0] < 0x8u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.cycles += 1;
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_c860);
                }
            }
        }
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// libkernl_InstallTLBRefillHandlers (0x42ca80): stores the caller's
/// handler address (`a0`) at `0x4564f0` and installs the fixed handler
/// `0x42cfc0` for exceptions 1..3. Returns the stored address.
pub fn libkernl_install_tlb_refill_handlers(c: &mut Cpu) {
    const HANDLER_SLOT: u32 = 0x0045_64f0;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S0][0] = 0x430000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3040));
        c.g[S1][0] = c.g[A0][0];
        c.sw(HANDLER_SLOT, c.g[S1][0]);
        c.g[A1][0] = c.g[S0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x1;
        callee::SET_HANDLER.call(c, 0x0042_cab0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = 0x2;
        callee::SET_HANDLER.call(c, 0x0042_cabc)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = 0x3;
        callee::SET_HANDLER.call(c, 0x0042_cac8)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// Kernel copy syscall 0x5a (`kCopy_5`, 0x42cb68): sets `v1`, traps, delivers
/// a due event and returns through `ra`.
pub fn kcopy_5(c: &mut Cpu) {
    syscall_wrapper(c, 0x5a, 0x0042_cb6c, 0x0042_cb70);
}

/// Reserved syscall 0x5b (`RFU091`, 0x42cbb0): sets `v1`, traps, delivers a
/// due event and returns through `ra`.
pub fn rfu091(c: &mut Cpu) {
    syscall_wrapper(c, 0x5b, 0x0042_cbb4, 0x0042_cbb8);
}

/// Reserved syscall 0x74 (`RFU116_SetSyscall`, 0x42cbc0): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu116_set_syscall(c: &mut Cpu) {
    syscall_wrapper(c, 0x74, 0x0042_cbc4, 0x0042_cbc8);
}

/// InitExecPS2 (0x42cbd0): copies a 0x330-byte kernel template into RAM,
/// flushes the data cache, then installs the boot syscall handlers (three
/// `RFU116` calls, then five trips of one `RFU091` plus one `RFU116` each)
/// for the loaded executable.
pub fn init_exec_ps2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = 0x3;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6540));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6540));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::RFU116_SET_SYSCALL_CBC0.call(c, 0x0042_cc00)?;
        c.g[A1][0] = 0x450000;
        c.g[A0][0] = 0xffffffff80070000;
        c.g[A2][0] = 0x330;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x61c0));
        c.g[A0][0] = c.g[A0][0] | 0x5000;
        callee::KCOPY_5.call(c, 0x0042_cc18)?;
        c.g[A0][0] = 0u64;
        callee::FLUSH_CACHE.call(c, 0x0042_cc20)?;
        c.g[A0][0] = 0x2;
        callee::FLUSH_CACHE.call(c, 0x0042_cc28)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        callee::RFU116_SET_SYSCALL_CBC0.call(c, 0x0042_cc34)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        callee::RFU116_SET_SYSCALL_CBC0.call(c, 0x0042_cc40)?;
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        loop {
            c.cycles += 9;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            callee::RFU091.call(c, 0x0042_cc50)?;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = c.g[V0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
            callee::RFU116_SET_SYSCALL_CBC0.call(c, 0x0042_cc60)?;
            c.g[V0][0] = (c.g[S2][0] < 0x8u64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_cc48);
                }
            } else {
                c.cycles += 10;
                c.g[A0][0] = 0x3;
                callee::RFU091.call(c, 0x0042_cc74)?;
                c.g[V1][0] = 0x450000;
                c.g[RA][0] = c.ld(frame.at(c, 0x30));
                c.g[S2][0] = c.ld(frame.at(c, 0x20));
                c.g[S1][0] = c.ld(frame.at(c, 0x10));
                c.g[S0][0] = c.ld(frame.at(c, 0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x6538), c.g[V0][0]);
                let ret = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
                c.pc = ret;
                return Ok(());
            }
        }
    })
}

/// Reserved syscall 0x56 (`RFU086_WaitEvnetFlag`, 0x42ccb8): sets `v1`, traps,
/// delivers a due event and returns through `ra`.
pub fn rfu086_wait_evnet_flag(c: &mut Cpu) {
    syscall_wrapper(c, 0x56, 0x0042_ccbc, 0x0042_ccc0);
}

/// SetTLBEntry (0x42ccc8): with the entry index (`a0 - 13`) below 35, wait
/// for the TLB event flag; otherwise report -1. Returns the flag result, or
/// -1 when the index is out of range.
pub fn set_tlb_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd));
        c.g[V0][0] = (c.g[V0][0] < 0x23u64) as u64;
        let in_range = c.g[V0][0] != 0u64;
        frame.save_ra(c, 0);
        if !in_range {
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffffffffff;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        callee::WAIT_FLAG.call(c, 0x0042_ccec)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// libkernl_SetupTLB (0x42cd98): builds the kernel TLB tables from the config
/// at `0x4567f0` (entry count, per-entry `{key, ...}` words), one
/// `RFU086` syscall per entry word, then enables the tables (two `mtc0`
/// writes to CP0 register 6, Wired). Prints and `Exit(1)`s when the table
/// overflows 48 entries.
pub fn libkernl_setup_tlb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5268));
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = 0x450000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x67f0));
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x67f0));
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[A1][0];
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[T0][0] = c.g[A3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
        c.g[RA][0] = sx(0x0042cde8);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
        c.pc = callee::PRINTF.addr;
        (callee::PRINTF.f)(c);
        if c.pc != 0x0042cde8 {
            return Err(Unwound);
        }
        c.mtc0(0, 6);
        c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x67f0));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 49i64) as u64;
        let fits = c.g[V0][0] != 0u64;
        c.g[T9][0] = 0u64;
        if !fits {
            c.cycles += 5;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x0042ce0c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5230));
            c.pc = callee::PRINTF.addr;
            (callee::PRINTF.f)(c);
            if c.pc != 0x0042ce0c {
                return Err(Unwound);
            }
            c.g[RA][0] = sx(0x0042ce14);
            c.g[A0][0] = 0x1;
            c.pc = callee::EXIT_NOW.addr;
            (callee::EXIT_NOW.f)(c);
            if c.pc != 0x0042ce14 {
                return Err(Unwound);
            }
        }
        // First entry list: words at `[s0 + 0x10]`, `t9` entries.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
        let run_first = c.g[V0][0] == 0u64;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        if !run_first {
            c.cycles += 2;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[T9][0];
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                callee::WAIT_FLAG.call(c, 0x0042_ce40)?;
                c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_ce28);
                }
            }
        }
        // Recompute the bound from the second list head; same walk.
        c.cycles += 6;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x67f0));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = sx((c.g[T9][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[S1][0] as i64) < 49i64) as u64;
        let ok = c.g[V1][0] != 0u64;
        c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
        if !ok {
            c.cycles += 6;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x0042ce74);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5218));
            c.pc = callee::PRINTF.addr;
            (callee::PRINTF.f)(c);
            if c.pc != 0x0042ce74 {
                return Err(Unwound);
            }
            c.g[RA][0] = sx(0x0042ce7c);
            c.g[A0][0] = 0x1;
            c.pc = callee::EXIT_NOW.addr;
            (callee::EXIT_NOW.f)(c);
            if c.pc != 0x0042ce7c {
                return Err(Unwound);
            }
            c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
        }
        c.cycles += 2;
        let run_second = c.g[V0][0] == 0u64;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        if !run_second {
            c.cycles += 2;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[T9][0];
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                callee::WAIT_FLAG.call(c, 0x0042_cea8)?;
                c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_ce90);
                }
            }
        }
        // Enable the tables and walk the third list.
        c.cycles += 7;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x67f0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[T9][0]);
        c.mtc0(25, 6);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        if (c.g[V0][0] as i64) <= 0 {
            c.g[S0][0] = c.g[T9][0];
        } else {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[T9][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 49i64) as u64;
            let ok3 = c.g[V0][0] != 0u64;
            c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if !ok3 {
                c.cycles += 6;
                c.g[A0][0] = 0x4a0000;
                c.g[RA][0] = sx(0x0042cef0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5200));
                c.pc = callee::PRINTF.addr;
                (callee::PRINTF.f)(c);
                if c.pc != 0x0042cef0 {
                    return Err(Unwound);
                }
                c.g[RA][0] = sx(0x0042cef8);
                c.g[A0][0] = 0x1;
                c.pc = callee::EXIT_NOW.addr;
                (callee::EXIT_NOW.f)(c);
                if c.pc != 0x0042cef8 {
                    return Err(Unwound);
                }
                c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
            }
            c.cycles += 2;
            let run_third = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            if !run_third {
                c.cycles += 1;
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[T9][0];
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    callee::WAIT_FLAG.call(c, 0x0042_cf20)?;
                    c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        break;
                    }
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_cf08);
                    }
                }
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[T9][0];
        }
        // Final table: one syscall per remaining entry up to 48.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 48i64) as u64;
        let run_last = c.g[V0][0] == 0u64;
        c.g[T9][0] = sx((c.g[T9][0] as u32) << 13);
        if !run_last {
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffe0000000;
            c.g[S1][0] = sx((c.g[T9][0] as u32).wrapping_add(c.g[V0][0] as u32));
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                callee::WAIT_FLAG.call(c, 0x0042_cf64)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 48i64) as u64;
                let more = c.g[V0][0] != 0u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2000));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0042_cf48);
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[V0][0] = c.g[T9][0];
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// _InitSys (0x42c300): kernel start-up: heap-semaphore, syscall table, alarm,
/// thread helper and OSD init, then tail-calls `InitExecPS2` to boot.
pub fn init_sys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::INIT_MALLOC_SEMA.call(c, 0x0042_c310)?;
        callee::INIT_SYSCALL_TABLE.call(c, 0x0042_c318)?;
        callee::INIT_ALARM.call(c, 0x0042_c320)?;
        callee::INIT_THREAD_HELPER.call(c, 0x0042_c328)?;
        callee::INIT_OSD.call(c, 0x0042_c330)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        c.pc = callee::INIT_EXEC.addr;
        (callee::INIT_EXEC.f)(c);
        Ok(())
    })
}

/// RCMP_AV_MS_TIMER_Update (0x42e508): calls `EATimer_GetTicks` (`0x3e4e98`)
/// with `a0`, then folds the tick delta into the accumulator at `a0 + 8`
/// (a 64-bit `multu` of the delta by the word at `a0 + 0x10`, shifted `>> 12`
/// when that word is not `0x1000`) and stores the tick count at `[a0]`.
pub fn rcmp_av_ms_timer_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        const TIMER_CB: u32 = 0x003e_4e98;
        call_ptr(c, TIMER_CB, 0x0042_e51c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x1000;
        let use_mult = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        if use_mult {
            c.cycles += 10;
            c.g[V1][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.multu(0, 0, 4, 2);
            c.g[A2][0] = c.lo[0];
            c.g[V0][0] = c.hi[0];
            c.g[A2][0] = c.g[A2][0] << 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[A2][0] = c.g[A2][0] >> 32;
            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] >> 12;
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 32;
        }
        c.cycles += 7;
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
        c.sd((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// exit

/// The reset vector `exit` jumps to after running the destructors.
pub(crate) const EXIT_RESET: u32 = 0x0010_01c8;

/// `void exit(int status)` (0x411310): run the global destructors and the
/// stdio cleanup, then reset (a fatal-error path; never returns).
pub fn exit_3(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        callee::DO_GLOBAL_DTORS.call(c, 0x0041_1324)?;
        callee::CLEANUP.call(c, 0x0041_132c)?;
        c.g[A0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        c.pc = EXIT_RESET;
        ssx3_ee_rt::funcs::call_indirect(c, EXIT_RESET);
        Ok(())
    })
}

/// TTY ring-buffer header words (see `TtyRingPush`/`TtyRingPop`).
pub(crate) mod tty_ring {
    /// Live-byte count: decremented by pop, incremented by push.
    pub(crate) const COUNT_OFF: u32 = 0x4;
    /// Head offset: advanced by pop, wrapped at the end.
    pub(crate) const HEAD_OFF: u32 = 0x8;
    /// Tail offset: advanced by push, wrapped at the end.
    pub(crate) const TAIL_OFF: u32 = 0xc;
    /// Header size in bytes: head and tail start at `ring + 0x10`.
    pub(crate) const HDR_SIZE: u32 = 0x10;
}

/// SIO registers.
pub(crate) mod sio {
    /// Status (`0x1000_f130`): bit 15 set while the port is busy.
    pub(crate) const STAT: u32 = 0x1000_f130;
    /// Transmit holding (`0x1000_f180`).
    pub(crate) const TX: u32 = 0x1000_f180;
    /// Busy bit of the status word.
    pub(crate) const BUSY: u64 = 0x8000;
}

/// `void libkernl_ResetTtyInit(void)` (0x424220): clear the TTY-init flag word.
pub fn libkernl_reset_tty_init(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x450000;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5168), 0u64);
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void libkernl_TtyRingInit(unsigned int capacity)` (0x00424e00): set up
/// the fixed TTY ring header: store the capacity word and point head and
/// tail past the header.
pub fn libkernl_tty_ring_init(c: &mut Cpu) {
    use tty_ring::*;
    c.cycles += 9;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4840));
    c.sw((c.g[V0][0] as u32).wrapping_sub(0x4840), c.g[A0][0]);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(HDR_SIZE));
    c.g[V0][0] = c.g[V1][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(HEAD_OFF), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(COUNT_OFF), 0u64);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V1][0] as u32).wrapping_add(TAIL_OFF), c.g[A0][0]);
    c.pc = t;
}

/// `int libkernl_TtyRingPush(void *ring, ...)` (0x00424e28): append to the
/// TTY ring; wraps the tail back to the buffer start when it runs past
/// the end.
pub fn libkernl_tty_ring_push(c: &mut Cpu) {
    use tty_ring::*;
    c.cycles += 11;
    c.g[A1][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF));
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(HDR_SIZE));
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let wrapped = c.g[A0][0] == c.g[V1][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF), c.g[A0][0]);
    if wrapped {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(HDR_SIZE));
        c.sw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF), c.g[V0][0]);
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `int libkernl_TtyRingPop(void *ring)` (0x00424e68): take a byte off the
/// TTY ring; wraps the head back to the buffer start when it runs past
/// the end.
pub fn libkernl_tty_ring_pop(c: &mut Cpu) {
    use tty_ring::*;
    c.cycles += 11;
    c.g[A1][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF));
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(HDR_SIZE));
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let wrapped = c.g[A0][0] == c.g[V1][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF), c.g[A0][0]);
    if wrapped {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(HDR_SIZE));
        c.sw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF), c.g[V0][0]);
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void libkernl_SioPutc(int ch)` (0x00425320): wait for the SIO port to
/// go idle, then send the byte.
pub fn libkernl_sio_putc(c: &mut Cpu) {
    use sio::*;
    guest(c, |c| {
        c.cycles += 2;
        // The port builds the register address in two steps; both words
        // are named in `sio` above.
        c.g[V1][0] = STAT as u64;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] & BUSY;
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x00425328);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = TX as u64;
        let t = c.g[RA][0] as u32;
        c.sb(c.g[V1][0] as u32, c.g[A0][0]);
        c.pc = t;
        return Ok(());
    });
}

/// `void libkernl_SioPutcCRLF(int ch)` (0x00425408): send `\r` first when
/// the byte is `\n`, then send the byte itself.
pub fn libkernl_sio_putc_crlf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0xa;
        let newline = c.g[A0][0] == c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        // A plain byte tail-calls the putc below (the delay-slot `RA`
        // save runs on both paths).
        if !newline {
            c.cycles += 3;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.pc = 0x00425320;
            (ee::s15::libkernl_SioPutc)(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[A0][0] = 0xd;
        SIO_PUTC.call(c, 0x00425420)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[A0][0] = 0xa;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = 0x00425320;
        (ee::s15::libkernl_SioPutc)(c);
        return Ok(());
    });
}

/// `int libkernl_printf(const char *fmt, ...)` (0x00425c28): format into the
/// caller's stack buffer through `libkernl_vprintf`.
pub fn libkernl_printf(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x58));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T2][0]);
        c.g[RA][0] = sx(0x00425c54);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T3][0]);
        VPRINTF.call(c, 0x00425c54)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `int WakeupThread(int id, ...)` (0x423cd0): syscall 0x33.
pub fn wakeup_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x33, 0x0042_3cd4, 0x0042_3cd8);
}

/// `int SuspendThread(int id)` (0x423d10): syscall 0x37.
pub fn suspend_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x37, 0x0042_3d14, 0x0042_3d18);
}

/// `int libkernl_TtyWrite(const char *src, int count)` (0x425040): with
/// interrupts off, copy up to `count` bytes of `src` into the Deci2 tty
/// packet (a newline becomes CR LF; the copy stops when the packet's 256
/// bytes are full), send it with `sceDeci2ReqSend` and poll `sceDeci2Poll`
/// until the handler clears the busy word. Returns the source bytes taken;
/// -1 when the busy word is already set on entry (nothing touched) or the
/// send fails (the busy word is cleared again).
pub fn libkernl_tty_write(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_42516c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
            c.g[S5][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
            c.g[S4][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4730));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.g[S2][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[S0][0]);
            c.g[S1][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_42516c;
            } else {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0042508c);
                DINTR.call(c, 0x0042508c)?;
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = 0x20000000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4700));
                c.g[A0][0] = 0x1;
                c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.g[T0][0] = 0xffffffffffffffff;
                c.g[A3][0] = 0xa;
                c.g[A1][0] = 0xd;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                'l_4250b8: loop {
                    'b_425108: {
                        c.cycles += 3;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        if c.g[S2][0] == c.g[T0][0] {
                            c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4730));
                            break 'b_425108;
                        } else {
                            'b_425104: {
                                'b_4250e8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                                    let t = c.g[V0][0] != c.g[A3][0];
                                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                                    if t {
                                        break 'b_4250e8;
                                    } else {
                                        c.cycles += 5;
                                        c.sb(c.g[A0][0] as u32, c.g[A1][0]);
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = ((c.g[S1][0] as i64) < 256i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_425104;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                                            break 'b_4250e8;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 256i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x004250b8);
                                    }
                                    continue 'l_4250b8;
                                } else {
                                    break 'b_425104;
                                }
                            }
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4730));
                            break 'b_425108;
                        }
                    }
                    'b_42513c: {
                        c.cycles += 9;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x7));
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x4730));
                        c.g[RA][0] = sx(0x00425124);
                        c.sh(c.g[A2][0] as u32, c.g[V1][0]);
                        DECI2_REQ_SEND.call(c, 0x00425124)?;
                        let t = (c.g[V0][0] as i64) >= 0;
                        if t {
                            break 'b_42513c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00425134);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            callee::EINTR.call(c, 0x00425134)?;
                            let t = true;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_42516c;
                            } else {
                                break 'b_42513c;
                            }
                        }
                    }
                    'b_425160: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = c.g[S5][0];
                        if t {
                            break 'b_425160;
                        } else {
                            'l_425148: loop {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x4730));
                                c.g[RA][0] = sx(0x00425154);
                                DECI2_POLL.call(c, 0x00425154)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V0][0] != 0u64;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00425148);
                                    }
                                    continue 'l_425148;
                                } else {
                                    break 'b_425160;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00425168);
                    callee::EINTR.call(c, 0x00425168)?;
                    c.g[V0][0] = c.g[S4][0];
                    break 'b_42516c;
                }
            }
        }
        c.cycles += 9;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `int libkernl_TtyRead(char *dst, int count)` (0x425190): pop up to
/// `count` bytes from the Deci2 tty ring into `dst`, stopping early at a
/// newline or CR; returns the bytes stored. An empty count returns 0.
pub fn libkernl_tty_read(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_425240: {
            'b_42523c: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[V1][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.g[S4][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                let t = (c.g[S2][0] as i64) <= 0;
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                if t {
                    break 'b_42523c;
                } else {
                    c.cycles += 3;
                    c.g[S3][0] = 0x530000;
                    c.g[V0][0] = 0x530000;
                    'l_4251c8: loop {
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x4718));
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        'l_4251d8: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            let t = c.g[V0][0] == 0u64;
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x004251d8);
                                }
                                continue 'l_4251d8;
                            } else {
                                'b_425230: {
                                    'b_425228: {
                                        c.cycles += 11;
                                        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4730));
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                                        c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                        c.g[RA][0] = sx(0x00425210);
                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                        TTY_RING_POP.call(c, 0x00425210)?;
                                        c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                                        c.g[V0][0] = 0xa;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0xd;
                                        if t {
                                            break 'b_425228;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V1][0] = c.g[S1][0];
                                            if t {
                                                break 'b_425230;
                                            } else {
                                                break 'b_425228;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = c.g[S1][0];
                                    break 'b_425240;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x530000;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x004251c8);
                                    }
                                    continue 'l_4251c8;
                                } else {
                                    break 'b_42523c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
            break 'b_425240;
        }
        c.cycles += 8;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `int libkernl_TtyInit(void)` (0x425260): open the Deci2 tty channel
/// and set up its packet and ring; returns 1, or 0 when the open fails.
pub fn libkernl_tty_init(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_425308: {
            'b_4252ac: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[A0][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[RA][0] = sx(0x0042527c);
                c.g[S0][0] = 0x530000;
                FLUSH_CACHE.call(c, 0x0042527c)?;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4730));
                c.g[A2][0] = 0x420000;
                c.g[A0][0] = 0x210;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4ea8));
                c.g[RA][0] = sx(0x00425294);
                c.g[A1][0] = c.g[S1][0];
                DECI2_OPEN.call(c, 0x00425294)?;
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x4730), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x4730));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[A0][0] = 0x530000;
                if t {
                    break 'b_4252ac;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_425308;
                    } else {
                        break 'b_4252ac;
                    }
                }
            }
            c.cycles += 23;
            c.g[V0][0] = 0x530000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V1][0] = 0x20000000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45c0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4700));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.g[A2][0] = 0x210;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[A1][0] = 0x45;
            c.g[V1][0] = 0x48;
            c.g[A0][0] = 0x100;
            c.sh((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x6), c.g[A1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x7), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.g[RA][0] = sx(0x00425300);
            c.sh((c.g[V0][0] as u32).wrapping_add(0x2), 0u64);
            TTY_RING_INIT.call(c, 0x00425300)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.g[V0][0] = 0x1;
            break 'b_425308;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void libkernl_KputcBuffered(int ch)` (0x425358): buffer one char for
/// the Deci2 console, flushing a full buffer first and a newline straight
/// through `Deci2Kputs`.
pub fn libkernl_kputc_buffered(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_4253a4: {
            'b_4253a0: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.g[S1][0] = 0x450000;
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5174));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = ((c.g[A1][0] as i64) < 126i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                if t {
                    break 'b_4253a0;
                } else {
                    c.cycles += 8;
                    c.g[S2][0] = 0x530000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x5174), 0u64);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4480));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00425398);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x7f), 0u64);
                    DECI2_KPUTS.call(c, 0x00425398)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5174));
                    break 'b_4253a4;
                }
            }
            c.cycles += 1;
            c.g[S2][0] = 0x530000;
            break 'b_4253a4;
        }
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4480));
        if t {
            c.cycles += 10;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x5174), c.g[V1][0]);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sb(c.g[V0][0] as u32, c.g[S0][0]);
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = t;
            Ok(())
        } else {
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4480));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x5174), 0u64);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            c.sb(c.g[V0][0] as u32, c.g[S0][0]);
            c.g[V1][0] = c.g[V0][0];
            c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            c.sb((c.g[V1][0] as u32).wrapping_add(0x1), 0u64);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = 0x0042_ca58;
            (ee::s15::f_0042ca58)(c);
            Ok(())
        }
    })
}

/// `int libkernl_DoubleToInt(double x)` (0x425440): the magnitude of the
/// double bits in `a0` as an int (the sign bit is ignored), rounded up when
/// the fraction is at least 0.75; 0 below 0.5 (zero and denormals
/// included), 9999 from 2^65 up, and otherwise the low 32 bits of the
/// result, sign-extended.
pub fn libkernl_double_to_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = c.g[A1][0] << 1;
        c.g[A2][0] = c.g[V0][0] >> 53;
        c.g[A2][0] = c.g[A2][0].wrapping_sub(0x433);
        c.g[V0][0] = ((c.g[A2][0] as i64) < -53i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 13i64) as u64;
        if t {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.g[A1][0] << 12;
            if t {
                'b_4254c4: {
                    'b_4254b8: {
                        c.cycles += 5;
                        c.g[V1][0] = 0x8000;
                        c.g[V1][0] = c.g[V1][0] << 37;
                        c.g[A1][0] = c.g[V0][0] >> 12;
                        let t = (c.g[A2][0] as i64) >= 0;
                        c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
                        if t {
                            break 'b_4254b8;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = 0u64.wrapping_sub(c.g[A2][0]);
                            c.g[V0][0] = 0x3;
                            c.g[V1][0] = c.g[A2][0].wrapping_sub(0x2);
                            c.g[V1][0] = c.g[V1][0] << 32;
                            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                            c.g[A1][0] = c.g[A1][0] >> (c.g[V1][0] & 63);
                            c.g[A0][0] = c.g[A1][0] & 0x3;
                            if c.g[A0][0] != c.g[V0][0] {
                                c.g[A1][0] = c.g[A1][0] >> 2;
                                break 'b_4254c4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A1][0] >> 2;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0].wrapping_add(0x1);
                                if t {
                                    break 'b_4254c4;
                                } else {
                                    break 'b_4254b8;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A2][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[A1][0] = c.g[A1][0] << (c.g[V0][0] & 63);
                    break 'b_4254c4;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] << 32;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.pc = t;
                Ok(())
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x270f;
                c.pc = t;
                Ok(())
            }
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0u64;
            c.pc = t;
            Ok(())
        }
    })
}

/// `void libkernl_TtyDeci2Handler(int event, int arg, TtyState *tty)`
/// (0x424ea8): the tty protocol's Deci2 callback. Event 3 sends the
/// pending transmit bytes through `sceDeci2ExSend` and advances the
/// transmit pointer and count (a failure prints and clears the busy word).
/// Events 1 and 2 with `arg` bytes waiting receive them into the receive
/// buffer through `sceDeci2ExRecv` (printing past 0x140 bytes); with `arg`
/// 0 they push the received bytes, from offset 12 up to the packet's
/// length, to the tty ring and empty the buffer. Event 4 prints if bytes
/// are still pending and clears the busy word. Other events return.
/// The normal entry is converted from the generated body (every register,
/// cycle and call kept); any other `pc` (a resume at the push loop's head
/// after an unwind) runs the generated body.
pub fn libkernl_tty_deci2_handler(c: &mut Cpu) {
    if c.pc != 0x00424ea8 {
        return libkernl_tty_deci2_handler_resume(c);
    }
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = 0x00424ea8;
        loop {
            match pc {
                0x00424ea8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0x3;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S1][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S0][0] = c.g[A1][0];
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    if t {
                        pc = 0x00424fb4;
                        continue;
                    }
                    pc = 0x00424ecc;
                    continue;
                }
                0x00424ecc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x00424ee8;
                        continue;
                    }
                    pc = 0x00424ed8;
                    continue;
                }
                0x00424ed8 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425004;
                        continue;
                    }
                    pc = 0x00424ee0;
                    continue;
                }
                0x00424ee0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x0042502c;
                        continue;
                    }
                    pc = 0x00424ee8;
                    continue;
                }
                0x00424ee8 => {
                    c.cycles += 2;
                    let t = (c.g[A0][0] as i64) <= 0;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424ef0;
                    continue;
                }
                0x00424ef0 => {
                    c.cycles += 2;

                    if c.g[S0][0] == 0u64 {
                        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        pc = 0x00424f58;
                        continue;
                    }
                    pc = 0x00424ef8;
                    continue;
                }
                0x00424ef8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < 0x141u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x00424f14;
                        continue;
                    }
                    pc = 0x00424f0c;
                    continue;
                }
                0x00424f0c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00424f14);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54f8));
                    PRINTF.call(c, 0x00424f14)?;
                    pc = 0x00424f14;
                    continue;
                }
                0x00424f14 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[S0][0] & 0xffff;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x00424f2c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    DECI2_EX_RECV.call(c, 0x00424f2c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = (c.g[S0][0] as i64) >= 0;

                    if t {
                        pc = 0x00424f44;
                        continue;
                    }
                    pc = 0x00424f38;
                    continue;
                }
                0x00424f38 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00424f44);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54d0));
                    PRINTF.call(c, 0x00424f44)?;
                    pc = 0x00424f44;
                    continue;
                }
                0x00424f44 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424f58;
                    continue;
                }
                0x00424f58 => {
                    c.cycles += 5;
                    c.g[S0][0] = 0xc;
                    c.g[V0][0] = c.lhu(c.g[S2][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.g[S2][0];
                    if t {
                        pc = 0x00424fa8;
                        continue;
                    }
                    pc = 0x00424f6c;
                    continue;
                }
                0x00424f6c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x00424f7c;
                        continue;
                    }
                    pc = 0x00424f74;
                    continue;
                }
                0x00424f74 => {
                    c.cycles += 1;
                    pc = 0x00424f78;
                    continue;
                }
                0x00424f78 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    pc = 0x00424f7c;
                    continue;
                }
                0x00424f7c => {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[RA][0] = sx(0x00424f98);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    TTY_RING_PUSH.call(c, 0x00424f98)?;
                    c.g[V0][0] = c.lhu(c.g[S2][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x00424f78) {
                            return Ok(());
                        }
                        pc = 0x00424f78;
                        continue;
                    }
                    pc = 0x00424fa8;
                    continue;
                }
                0x00424fa8 => {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424fb4;
                    continue;
                }
                0x00424fb4 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00424fc8);
                    c.g[A2][0] = c.g[A2][0] & 0xffff;
                    DECI2_EX_SEND.call(c, 0x00424fc8)?;
                    c.g[A1][0] = c.g[V0][0];

                    if (c.g[A1][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        pc = 0x00424fe8;
                        continue;
                    }
                    pc = 0x00424fd4;
                    continue;
                }
                0x00424fd4 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00424fe0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54b8));
                    PRINTF.call(c, 0x00424fe0)?;
                    let t = true;

                    if t {
                        pc = 0x00425020;
                        continue;
                    }
                    pc = 0x00424fe8;
                    continue;
                }
                0x00424fe8 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00425004;
                    continue;
                }
                0x00425004 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00425020;
                        continue;
                    }
                    pc = 0x00425010;
                    continue;
                }
                0x00425010 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x00425020);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54a0));
                    PRINTF.call(c, 0x00425020)?;
                    pc = 0x00425020;
                    continue;
                }
                0x00425020 => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x00425028;
                    continue;
                }
                0x00425028 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x0042502c;
                    continue;
                }
                0x0042502c => {
                    c.cycles += 4;
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00424ea8, 0x0042503c);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `libkernl_tty_deci2_handler` (converted from the `generated` body): any entry
/// other than 0x00424ea8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn libkernl_tty_deci2_handler_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00424ea8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0x3;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S1][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S0][0] = c.g[A1][0];
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    if t {
                        pc = 0x00424fb4;
                        continue;
                    }
                    pc = 0x00424ecc;
                    continue;
                }
                0x00424ecc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x00424ee8;
                        continue;
                    }
                    pc = 0x00424ed8;
                    continue;
                }
                0x00424ed8 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425004;
                        continue;
                    }
                    pc = 0x00424ee0;
                    continue;
                }
                0x00424ee0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x0042502c;
                        continue;
                    }
                    pc = 0x00424ee8;
                    continue;
                }
                0x00424ee8 => {
                    c.cycles += 2;
                    let t = (c.g[A0][0] as i64) <= 0;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424ef0;
                    continue;
                }
                0x00424ef0 => {
                    c.cycles += 2;

                    if c.g[S0][0] == 0u64 {
                        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        pc = 0x00424f58;
                        continue;
                    }
                    pc = 0x00424ef8;
                    continue;
                }
                0x00424ef8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < 0x141u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x00424f14;
                        continue;
                    }
                    pc = 0x00424f0c;
                    continue;
                }
                0x00424f0c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00424f14);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54f8));
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x00424f14)?;
                    pc = 0x00424f14;
                    continue;
                }
                0x00424f14 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[S0][0] & 0xffff;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x00424f2c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    Callee {
                        addr: 0x0042c968,
                        f: ee::s15::sceDeci2ExRecv,
                    }
                    .call(c, 0x00424f2c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = (c.g[S0][0] as i64) >= 0;

                    if t {
                        pc = 0x00424f44;
                        continue;
                    }
                    pc = 0x00424f38;
                    continue;
                }
                0x00424f38 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00424f44);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54d0));
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x00424f44)?;
                    pc = 0x00424f44;
                    continue;
                }
                0x00424f44 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424f58;
                    continue;
                }
                0x00424f58 => {
                    c.cycles += 5;
                    c.g[S0][0] = 0xc;
                    c.g[V0][0] = c.lhu(c.g[S2][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.g[S2][0];
                    if t {
                        pc = 0x00424fa8;
                        continue;
                    }
                    pc = 0x00424f6c;
                    continue;
                }
                0x00424f6c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x00424f7c;
                        continue;
                    }
                    pc = 0x00424f74;
                    continue;
                }
                0x00424f74 => {
                    c.cycles += 1;
                    pc = 0x00424f78;
                    continue;
                }
                0x00424f78 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    pc = 0x00424f7c;
                    continue;
                }
                0x00424f7c => {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[RA][0] = sx(0x00424f98);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x00424e28,
                        f: ee::s15::libkernl_TtyRingPush,
                    }
                    .call(c, 0x00424f98)?;
                    c.g[V0][0] = c.lhu(c.g[S2][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x00424f78) {
                            return Ok(());
                        }
                        pc = 0x00424f78;
                        continue;
                    }
                    pc = 0x00424fa8;
                    continue;
                }
                0x00424fa8 => {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00424fb4;
                    continue;
                }
                0x00424fb4 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00424fc8);
                    c.g[A2][0] = c.g[A2][0] & 0xffff;
                    Callee {
                        addr: 0x0042c9a0,
                        f: ee::s15::sceDeci2ExSend,
                    }
                    .call(c, 0x00424fc8)?;
                    c.g[A1][0] = c.g[V0][0];

                    if (c.g[A1][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        pc = 0x00424fe8;
                        continue;
                    }
                    pc = 0x00424fd4;
                    continue;
                }
                0x00424fd4 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00424fe0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54b8));
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x00424fe0)?;
                    let t = true;

                    if t {
                        pc = 0x00425020;
                        continue;
                    }
                    pc = 0x00424fe8;
                    continue;
                }
                0x00424fe8 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00425028;
                        continue;
                    }
                    pc = 0x00425004;
                    continue;
                }
                0x00425004 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00425020;
                        continue;
                    }
                    pc = 0x00425010;
                    continue;
                }
                0x00425010 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x00425020);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54a0));
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x00425020)?;
                    pc = 0x00425020;
                    continue;
                }
                0x00425020 => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x00425028;
                    continue;
                }
                0x00425028 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x0042502c;
                    continue;
                }
                0x0042502c => {
                    c.cycles += 4;
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00424ea8, 0x0042503c);
                    return Ok(());
                }
            }
        }
    });
}

/// `libkernl_PrintDouble` (0x004254d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libkernl_print_double(c: &mut Cpu) {
    if c.pc != 0x004254d0 {
        return libkernl_print_double_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x004254d0;
        loop {
            match pc {
                0x004254d0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S2][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S1][0] = 0u64;
                    c.g[RA][0] = sx(0x004254f8);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x004254f8)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x0042551c;
                        continue;
                    }
                    pc = 0x00425500;
                    continue;
                }
                0x00425500 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00425508);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00411f70,
                        f: ee::s15::__subdf3,
                    }
                    .call(c, 0x00425508)?;
                    c.g[V1][0] = 0x450000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0042551c);
                    c.g[A0][0] = 0x2d;
                    call_ptr(c, t, 0x0042551c)?;
                    pc = 0x0042551c;
                    continue;
                }
                0x0042551c => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5468));
                    c.g[RA][0] = sx(0x0042552c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x0042552c)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[S2][0] = 0x4a0000;
                    if t {
                        pc = 0x00425574;
                        continue;
                    }
                    pc = 0x00425534;
                    continue;
                }
                0x00425534 => {
                    c.cycles += 2;
                    pc = 0x00425554;
                    continue;
                }
                0x0042553c => {
                    c.cycles += 1;
                    pc = 0x00425540;
                    continue;
                }
                0x00425540 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x8048;
                    c.g[A1][0] = c.g[A1][0] << 47;
                    c.g[RA][0] = sx(0x00425550);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00411fd8,
                        f: ee::s15::__muldf3,
                    }
                    .call(c, 0x00425550)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x00425554;
                    continue;
                }
                0x00425554 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5460));
                    c.g[RA][0] = sx(0x00425564);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x00425564)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425540) {
                            return Ok(());
                        }
                        pc = 0x00425540;
                        continue;
                    }
                    pc = 0x0042556c;
                    continue;
                }
                0x0042556c => {
                    c.cycles += 2;
                    pc = 0x004255c4;
                    continue;
                }
                0x00425574 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffc0;
                    c.g[A1][0] = c.g[A1][0] << 46;
                    c.g[RA][0] = sx(0x00425584);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x00425584)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S2][0] = 0x4a0000;
                    if t {
                        pc = 0x004255c4;
                        continue;
                    }
                    pc = 0x0042558c;
                    continue;
                }
                0x0042558c => {
                    c.cycles += 2;
                    pc = 0x004255ac;
                    continue;
                }
                0x00425594 => {
                    c.cycles += 1;
                    pc = 0x00425598;
                    continue;
                }
                0x00425598 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x8048;
                    c.g[A1][0] = c.g[A1][0] << 47;
                    c.g[RA][0] = sx(0x004255a8);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00412280,
                        f: ee::s15::__divdf3,
                    }
                    .call(c, 0x004255a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x004255ac;
                    continue;
                }
                0x004255ac => {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffc0;
                    c.g[A1][0] = c.g[A1][0] << 46;
                    c.g[RA][0] = sx(0x004255bc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x004255bc)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425598) {
                            return Ok(());
                        }
                        pc = 0x00425598;
                        continue;
                    }
                    pc = 0x004255c4;
                    continue;
                }
                0x004255c4 => {
                    c.cycles += 13;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5458));
                    c.g[RA][0] = sx(0x004255d4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00411fd8,
                        f: ee::s15::__muldf3,
                    }
                    .call(c, 0x004255d4)?;
                    c.g[RA][0] = sx(0x004255dc);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0042d380,
                        f: ee::s15::__fixunsdfdi,
                    }
                    .call(c, 0x004255dc)?;
                    c.g[RA][0] = sx(0x004255e4);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00425440,
                        f: ee::s15::libkernl_DoubleToInt,
                    }
                    .call(c, 0x004255e4)?;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5480));
                    c.g[RA][0] = sx(0x004255f0);
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x004255f0)?;
                    let t = (c.g[S1][0] as i64) < 0;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x00425618;
                        continue;
                    }
                    pc = 0x004255f8;
                    continue;
                }
                0x004255f8 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5478));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00425c28;
                    ee::s15::libkernl_printf(c);
                    return Ok(());
                }
                0x00425618 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5470));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00425c28;
                    ee::s15::libkernl_printf(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x004254d0, 0x00425638);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `libkernl_print_double` (converted from the `generated` body): any entry
/// other than 0x004254d0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn libkernl_print_double_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x004254d0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S2][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S1][0] = 0u64;
                    c.g[RA][0] = sx(0x004254f8);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x004254f8)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x0042551c;
                        continue;
                    }
                    pc = 0x00425500;
                    continue;
                }
                0x00425500 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00425508);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00411f70,
                        f: ee::s15::__subdf3,
                    }
                    .call(c, 0x00425508)?;
                    c.g[V1][0] = 0x450000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0042551c);
                    c.g[A0][0] = 0x2d;
                    call_ptr(c, t, 0x0042551c)?;
                    pc = 0x0042551c;
                    continue;
                }
                0x0042551c => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5468));
                    c.g[RA][0] = sx(0x0042552c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x0042552c)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[S2][0] = 0x4a0000;
                    if t {
                        pc = 0x00425574;
                        continue;
                    }
                    pc = 0x00425534;
                    continue;
                }
                0x00425534 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00425554;
                        continue;
                    }
                    pc = 0x0042553c;
                    continue;
                }
                0x0042553c => {
                    c.cycles += 1;
                    pc = 0x00425540;
                    continue;
                }
                0x00425540 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x8048;
                    c.g[A1][0] = c.g[A1][0] << 47;
                    c.g[RA][0] = sx(0x00425550);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00411fd8,
                        f: ee::s15::__muldf3,
                    }
                    .call(c, 0x00425550)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x00425554;
                    continue;
                }
                0x00425554 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5460));
                    c.g[RA][0] = sx(0x00425564);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x00425564)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425540) {
                            return Ok(());
                        }
                        pc = 0x00425540;
                        continue;
                    }
                    pc = 0x0042556c;
                    continue;
                }
                0x0042556c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x004255c4;
                        continue;
                    }
                    pc = 0x00425574;
                    continue;
                }
                0x00425574 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffc0;
                    c.g[A1][0] = c.g[A1][0] << 46;
                    c.g[RA][0] = sx(0x00425584);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x00425584)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S2][0] = 0x4a0000;
                    if t {
                        pc = 0x004255c4;
                        continue;
                    }
                    pc = 0x0042558c;
                    continue;
                }
                0x0042558c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x004255ac;
                        continue;
                    }
                    pc = 0x00425594;
                    continue;
                }
                0x00425594 => {
                    c.cycles += 1;
                    pc = 0x00425598;
                    continue;
                }
                0x00425598 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x8048;
                    c.g[A1][0] = c.g[A1][0] << 47;
                    c.g[RA][0] = sx(0x004255a8);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00412280,
                        f: ee::s15::__divdf3,
                    }
                    .call(c, 0x004255a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x004255ac;
                    continue;
                }
                0x004255ac => {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffc0;
                    c.g[A1][0] = c.g[A1][0] << 46;
                    c.g[RA][0] = sx(0x004255bc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00412500,
                        f: ee::s15::__cmpdf2,
                    }
                    .call(c, 0x004255bc)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425598) {
                            return Ok(());
                        }
                        pc = 0x00425598;
                        continue;
                    }
                    pc = 0x004255c4;
                    continue;
                }
                0x004255c4 => {
                    c.cycles += 13;
                    c.g[AT][0] = 0x4a0000;
                    c.g[A1][0] = c.ld((c.g[AT][0] as u32).wrapping_sub(0x5458));
                    c.g[RA][0] = sx(0x004255d4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00411fd8,
                        f: ee::s15::__muldf3,
                    }
                    .call(c, 0x004255d4)?;
                    c.g[RA][0] = sx(0x004255dc);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0042d380,
                        f: ee::s15::__fixunsdfdi,
                    }
                    .call(c, 0x004255dc)?;
                    c.g[RA][0] = sx(0x004255e4);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00425440,
                        f: ee::s15::libkernl_DoubleToInt,
                    }
                    .call(c, 0x004255e4)?;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5480));
                    c.g[RA][0] = sx(0x004255f0);
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00425c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x004255f0)?;
                    let t = (c.g[S1][0] as i64) < 0;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x00425618;
                        continue;
                    }
                    pc = 0x004255f8;
                    continue;
                }
                0x004255f8 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5478));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00425c28;
                    ee::s15::libkernl_printf(c);
                    return Ok(());
                }
                0x00425618 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5470));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00425c28;
                    ee::s15::libkernl_printf(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x004254d0, 0x00425638);
                    return Ok(());
                }
            }
        }
    });
}

/// `libkernl_vprintf` (0x00425638): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libkernl_vprintf(c: &mut Cpu) {
    if c.pc != 0x00425638 {
        return libkernl_vprintf_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00425638;
        loop {
            match pc {
                0x00425638 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
                    c.g[S3][0] = c.g[A1][0];
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
                    c.g[RA][0] = sx(0x00425668);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                    Callee {
                        addr: 0x0042c078,
                        f: ee::s15::DIntr,
                    }
                    .call(c, 0x00425668)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bec;
                        continue;
                    }
                    pc = 0x00425678;
                    continue;
                }
                0x00425678 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[S4][0] = 0u64;
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = 0x25;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A3][0] = 0u64;
                    if t {
                        pc = 0x00425bb8;
                        continue;
                    }
                    pc = 0x00425690;
                    continue;
                }
                0x00425690 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x00425694;
                    continue;
                }
                0x00425694 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S2][0];
                    pc = 0x00425698;
                    continue;
                }
                0x00425698 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = (c.g[A0][0] < 0x49u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00425bd0;
                        continue;
                    }
                    pc = 0x004256b4;
                    continue;
                }
                0x004256b4 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5450));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x004256cc => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x2));
                    if t {
                        pc = 0x00425754;
                        continue;
                    }
                    pc = 0x004256e4;
                    continue;
                }
                0x004256e4 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0xa;
                    if t {
                        pc = 0x00425714;
                        continue;
                    }
                    pc = 0x004256f4;
                    continue;
                }
                0x004256f4 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x1f;
                    c.mult(0, 3, 5, 3);
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
                    let t = true;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A0][0];
                    }
                    if t {
                        pc = 0x00425718;
                        continue;
                    }
                    pc = 0x00425714;
                    continue;
                }
                0x00425714 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425718;
                    continue;
                }
                0x00425718 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = (c.g[A1][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x00425690) {
                            return Ok(());
                        }
                        pc = 0x00425690;
                        continue;
                    }
                    pc = 0x00425724;
                    continue;
                }
                0x00425724 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x00425728;
                    continue;
                }
                0x00425728 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x1f;
                    c.g[A0][0] = 0x30;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sb(c.g[V1][0] as u32, c.g[A0][0]);
                    let t = (c.g[A1][0] as i64) > 0;

                    if t {
                        if back_edge_unwinds(c, 0x00425728) {
                            return Ok(());
                        }
                        pc = 0x00425728;
                        continue;
                    }
                    pc = 0x00425748;
                    continue;
                }
                0x00425748 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425698) {
                            return Ok(());
                        }
                        pc = 0x00425698;
                        continue;
                    }
                    pc = 0x00425750;
                    continue;
                }
                0x00425750 => {
                    c.cycles += 1;
                    c.g[A3][0] = 0x6c;
                    pc = 0x00425754;
                    continue;
                }
                0x00425754 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x00425694) {
                            return Ok(());
                        }
                        pc = 0x00425694;
                        continue;
                    }
                    pc = 0x0042575c;
                    continue;
                }
                0x0042575c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A3][0] = 0x68;
                    if t {
                        if back_edge_unwinds(c, 0x00425754) {
                            return Ok(());
                        }
                        pc = 0x00425754;
                        continue;
                    }
                    pc = 0x00425764;
                    continue;
                }
                0x00425764 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x0042577c;
                        continue;
                    }
                    pc = 0x00425770;
                    continue;
                }
                0x00425770 => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x00425790;
                        continue;
                    }
                    pc = 0x0042577c;
                    continue;
                }
                0x0042577c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0042578c;
                        continue;
                    }
                    pc = 0x00425784;
                    continue;
                }
                0x00425784 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x00425790;
                        continue;
                    }
                    pc = 0x0042578c;
                    continue;
                }
                0x0042578c => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x00425790;
                    continue;
                }
                0x00425790 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x004257b0;
                        continue;
                    }
                    pc = 0x0042579c;
                    continue;
                }
                0x0042579c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x004257d8;
                        continue;
                    }
                    pc = 0x004257b0;
                    continue;
                }
                0x004257b0 => {
                    c.cycles += 2;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x004257b8;
                    continue;
                }
                0x004257b8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[S1][0] & 0x7;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[S1][0] = c.g[S1][0] >> 3;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    let t = c.g[S1][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x004257b8) {
                            return Ok(());
                        }
                        pc = 0x004257b8;
                        continue;
                    }
                    pc = 0x004257d8;
                    continue;
                }
                0x004257d8 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x004257e4;
                        continue;
                    }
                    pc = 0x004257e0;
                    continue;
                }
                0x004257e0 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x004257e4;
                    continue;
                }
                0x004257e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004257f0;
                    continue;
                }
                0x004257f0 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004257f8;
                    continue;
                }
                0x004257f8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0042580c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x0042580c)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004257f8) {
                            return Ok(());
                        }
                        pc = 0x004257f8;
                        continue;
                    }
                    pc = 0x00425818;
                    continue;
                }
                0x00425818 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425820;
                    continue;
                }
                0x00425820 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x00425838;
                        continue;
                    }
                    pc = 0x0042582c;
                    continue;
                }
                0x0042582c => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042584c;
                        continue;
                    }
                    pc = 0x00425838;
                    continue;
                }
                0x00425838 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00425848;
                        continue;
                    }
                    pc = 0x00425840;
                    continue;
                }
                0x00425840 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042584c;
                        continue;
                    }
                    pc = 0x00425848;
                    continue;
                }
                0x00425848 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0042584c;
                    continue;
                }
                0x0042584c => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x0042586c;
                        continue;
                    }
                    pc = 0x00425858;
                    continue;
                }
                0x00425858 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425898;
                        continue;
                    }
                    pc = 0x0042586c;
                    continue;
                }
                0x0042586c => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425870;
                    continue;
                }
                0x00425870 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S1][0] & 0xf;
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V1][0].wrapping_add(0x30);
                    if t {
                        pc = 0x00425884;
                        continue;
                    }
                    pc = 0x00425880;
                    continue;
                }
                0x00425880 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[V1][0].wrapping_add(0x57);
                    pc = 0x00425884;
                    continue;
                }
                0x00425884 => {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[S1][0] = c.g[S1][0] >> 4;
                    let t = c.g[S1][0] != 0u64;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    if t {
                        if back_edge_unwinds(c, 0x00425870) {
                            return Ok(());
                        }
                        pc = 0x00425870;
                        continue;
                    }
                    pc = 0x00425898;
                    continue;
                }
                0x00425898 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x004258a4;
                        continue;
                    }
                    pc = 0x004258a0;
                    continue;
                }
                0x004258a0 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x004258a4;
                    continue;
                }
                0x004258a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004258b0;
                    continue;
                }
                0x004258b0 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004258b8;
                    continue;
                }
                0x004258b8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x004258cc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x004258cc)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004258b8) {
                            return Ok(());
                        }
                        pc = 0x004258b8;
                        continue;
                    }
                    pc = 0x004258d8;
                    continue;
                }
                0x004258d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x004258e0;
                    continue;
                }
                0x004258e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x004258f8;
                        continue;
                    }
                    pc = 0x004258ec;
                    continue;
                }
                0x004258ec => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042590c;
                        continue;
                    }
                    pc = 0x004258f8;
                    continue;
                }
                0x004258f8 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00425908;
                        continue;
                    }
                    pc = 0x00425900;
                    continue;
                }
                0x00425900 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lh((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042590c;
                        continue;
                    }
                    pc = 0x00425908;
                    continue;
                }
                0x00425908 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0042590c;
                    continue;
                }
                0x0042590c => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x00425928;
                        continue;
                    }
                    pc = 0x00425918;
                    continue;
                }
                0x00425918 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    let t = true;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    if t {
                        pc = 0x00425988;
                        continue;
                    }
                    pc = 0x00425928;
                    continue;
                }
                0x00425928 => {
                    c.cycles += 2;
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S5][0] = 0x450000;
                    if t {
                        pc = 0x00425940;
                        continue;
                    }
                    pc = 0x00425930;
                    continue;
                }
                0x00425930 => {
                    c.cycles += 4;
                    c.g[S1][0] = 0u64.wrapping_sub(c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425940);
                    c.g[A0][0] = 0x2d;
                    call_ptr(c, t, 0x00425940)?;
                    pc = 0x00425940;
                    continue;
                }
                0x00425940 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0042598c;
                        continue;
                    }
                    pc = 0x00425948;
                    continue;
                }
                0x00425948 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00425950;
                    continue;
                }
                0x00425950 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00425958);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0042d470,
                        f: ee::s15::__moddi3,
                    }
                    .call(c, 0x00425958)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00425974);
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    Callee {
                        addr: 0x0040fcb0,
                        f: ee::s15::__divdi3,
                    }
                    .call(c, 0x00425974)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425950) {
                            return Ok(());
                        }
                        pc = 0x00425950;
                        continue;
                    }
                    pc = 0x00425980;
                    continue;
                }
                0x00425980 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0042598c;
                        continue;
                    }
                    pc = 0x00425988;
                    continue;
                }
                0x00425988 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x0042598c;
                    continue;
                }
                0x0042598c => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x00425998;
                        continue;
                    }
                    pc = 0x00425994;
                    continue;
                }
                0x00425994 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x00425998;
                    continue;
                }
                0x00425998 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004259a4;
                    continue;
                }
                0x004259a4 => {
                    c.cycles += 1;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004259a8;
                    continue;
                }
                0x004259a8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x004259bc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x004259bc)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004259a8) {
                            return Ok(());
                        }
                        pc = 0x004259a8;
                        continue;
                    }
                    pc = 0x004259c8;
                    continue;
                }
                0x004259c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x004259d0;
                    continue;
                }
                0x004259d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x004259e8;
                        continue;
                    }
                    pc = 0x004259dc;
                    continue;
                }
                0x004259dc => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x004259fc;
                        continue;
                    }
                    pc = 0x004259e8;
                    continue;
                }
                0x004259e8 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x004259f8;
                        continue;
                    }
                    pc = 0x004259f0;
                    continue;
                }
                0x004259f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x004259fc;
                        continue;
                    }
                    pc = 0x004259f8;
                    continue;
                }
                0x004259f8 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x004259fc;
                    continue;
                }
                0x004259fc => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x00425a1c;
                        continue;
                    }
                    pc = 0x00425a08;
                    continue;
                }
                0x00425a08 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425a58;
                        continue;
                    }
                    pc = 0x00425a1c;
                    continue;
                }
                0x00425a1c => {
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00425a28;
                    continue;
                }
                0x00425a28 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00425a30);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0042dad8,
                        f: ee::s15::__umoddi3,
                    }
                    .call(c, 0x00425a30)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00425a4c);
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    Callee {
                        addr: 0x00411538,
                        f: ee::s15::__udivdi3,
                    }
                    .call(c, 0x00425a4c)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425a28) {
                            return Ok(());
                        }
                        pc = 0x00425a28;
                        continue;
                    }
                    pc = 0x00425a58;
                    continue;
                }
                0x00425a58 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x00425a64;
                        continue;
                    }
                    pc = 0x00425a60;
                    continue;
                }
                0x00425a60 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x00425a64;
                    continue;
                }
                0x00425a64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x00425a70;
                    continue;
                }
                0x00425a70 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x00425a78;
                    continue;
                }
                0x00425a78 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425a8c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425a8c)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00425a78) {
                            return Ok(());
                        }
                        pc = 0x00425a78;
                        continue;
                    }
                    pc = 0x00425a98;
                    continue;
                }
                0x00425a98 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425aa0;
                    continue;
                }
                0x00425aa0 => {
                    c.cycles += 7;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.mtc1(0, 0);
                    c.lwc1(12, (c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.c_eq_s(12, 0);
                    let t = !c.fcc();
                    c.g[S5][0] = 0x450000;
                    if t {
                        pc = 0x00425ad4;
                        continue;
                    }
                    pc = 0x00425abc;
                    continue;
                }
                0x00425abc => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x30;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425acc);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425acc)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425ad4;
                    continue;
                }
                0x00425ad4 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00425adc);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00413af8,
                        f: ee::s15::__extendsfdf2,
                    }
                    .call(c, 0x00425adc)?;
                    c.g[RA][0] = sx(0x00425ae4);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x004254d0,
                        f: ee::s15::libkernl_PrintDouble,
                    }
                    .call(c, 0x00425ae4)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425aec;
                    continue;
                }
                0x00425aec => {
                    c.cycles += 5;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    if t {
                        pc = 0x00425b58;
                        continue;
                    }
                    pc = 0x00425b00;
                    continue;
                }
                0x00425b00 => {
                    c.cycles += 22;
                    c.g[S5][0] = 0x450000;
                    c.g[A0][0] = 0x28;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b14);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425b14)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b20);
                    c.g[A0][0] = 0x6e;
                    call_ptr(c, t, 0x00425b20)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b2c);
                    c.g[A0][0] = 0x75;
                    call_ptr(c, t, 0x00425b2c)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b38);
                    c.g[A0][0] = 0x6c;
                    call_ptr(c, t, 0x00425b38)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b44);
                    c.g[A0][0] = 0x6c;
                    call_ptr(c, t, 0x00425b44)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b50);
                    c.g[A0][0] = 0x29;
                    call_ptr(c, t, 0x00425b50)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425b58;
                    continue;
                }
                0x00425b58 => {
                    c.cycles += 4;
                    c.g[S0][0] = c.g[V1][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S5][0] = 0x450000;
                    pc = 0x00425b68;
                    continue;
                }
                0x00425b68 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b7c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425b7c)?;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425b68) {
                            return Ok(());
                        }
                        pc = 0x00425b68;
                        continue;
                    }
                    pc = 0x00425b88;
                    continue;
                }
                0x00425b88 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425b90;
                    continue;
                }
                0x00425b90 => {
                    c.cycles += 10;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = 0x450000;
                    c.g[S1][0] = c.lb((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = c.g[S1][0] << 32;
                    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425bb0);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425bb0)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425bb8;
                    continue;
                }
                0x00425bb8 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425bc8);
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425bc8)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425bd0;
                    continue;
                }
                0x00425bd0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x00425bd8;
                    continue;
                }
                0x00425bd8 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425bdc;
                    continue;
                }
                0x00425bdc => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S2][0];
                    pc = 0x00425be0;
                    continue;
                }
                0x00425be0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00425678) {
                            return Ok(());
                        }
                        pc = 0x00425678;
                        continue;
                    }
                    pc = 0x00425bec;
                    continue;
                }
                0x00425bec => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x00425c00;
                        continue;
                    }
                    pc = 0x00425bf4;
                    continue;
                }
                0x00425bf4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00425bfc);
                    Callee {
                        addr: 0x0042c0c0,
                        f: ee::s15::EIntr,
                    }
                    .call(c, 0x00425bfc)?;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x00425c00;
                    continue;
                }
                0x00425c00 => {
                    c.cycles += 9;
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00425638, 0x00425c24);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `libkernl_vprintf` (converted from the `generated` body): any entry
/// other than 0x00425638, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn libkernl_vprintf_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00425638 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
                    c.g[S3][0] = c.g[A1][0];
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
                    c.g[RA][0] = sx(0x00425668);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                    Callee {
                        addr: 0x0042c078,
                        f: ee::s15::DIntr,
                    }
                    .call(c, 0x00425668)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bec;
                        continue;
                    }
                    pc = 0x00425678;
                    continue;
                }
                0x00425678 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[S4][0] = 0u64;
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = 0x25;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A3][0] = 0u64;
                    if t {
                        pc = 0x00425bb8;
                        continue;
                    }
                    pc = 0x00425690;
                    continue;
                }
                0x00425690 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x00425694;
                    continue;
                }
                0x00425694 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S2][0];
                    pc = 0x00425698;
                    continue;
                }
                0x00425698 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = (c.g[A0][0] < 0x49u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00425bd0;
                        continue;
                    }
                    pc = 0x004256b4;
                    continue;
                }
                0x004256b4 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5450));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x004256cc => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x2));
                    if t {
                        pc = 0x00425754;
                        continue;
                    }
                    pc = 0x004256e4;
                    continue;
                }
                0x004256e4 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0xa;
                    if t {
                        pc = 0x00425714;
                        continue;
                    }
                    pc = 0x004256f4;
                    continue;
                }
                0x004256f4 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x1f;
                    c.mult(0, 3, 5, 3);
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
                    let t = true;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A0][0];
                    }
                    if t {
                        pc = 0x00425718;
                        continue;
                    }
                    pc = 0x00425714;
                    continue;
                }
                0x00425714 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425718;
                    continue;
                }
                0x00425718 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = (c.g[A1][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x00425690) {
                            return Ok(());
                        }
                        pc = 0x00425690;
                        continue;
                    }
                    pc = 0x00425724;
                    continue;
                }
                0x00425724 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x00425728;
                    continue;
                }
                0x00425728 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x1f;
                    c.g[A0][0] = 0x30;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sb(c.g[V1][0] as u32, c.g[A0][0]);
                    let t = (c.g[A1][0] as i64) > 0;

                    if t {
                        if back_edge_unwinds(c, 0x00425728) {
                            return Ok(());
                        }
                        pc = 0x00425728;
                        continue;
                    }
                    pc = 0x00425748;
                    continue;
                }
                0x00425748 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425698) {
                            return Ok(());
                        }
                        pc = 0x00425698;
                        continue;
                    }
                    pc = 0x00425750;
                    continue;
                }
                0x00425750 => {
                    c.cycles += 1;
                    c.g[A3][0] = 0x6c;
                    pc = 0x00425754;
                    continue;
                }
                0x00425754 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x00425694) {
                            return Ok(());
                        }
                        pc = 0x00425694;
                        continue;
                    }
                    pc = 0x0042575c;
                    continue;
                }
                0x0042575c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A3][0] = 0x68;
                    if t {
                        if back_edge_unwinds(c, 0x00425754) {
                            return Ok(());
                        }
                        pc = 0x00425754;
                        continue;
                    }
                    pc = 0x00425764;
                    continue;
                }
                0x00425764 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x0042577c;
                        continue;
                    }
                    pc = 0x00425770;
                    continue;
                }
                0x00425770 => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x00425790;
                        continue;
                    }
                    pc = 0x0042577c;
                    continue;
                }
                0x0042577c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0042578c;
                        continue;
                    }
                    pc = 0x00425784;
                    continue;
                }
                0x00425784 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x00425790;
                        continue;
                    }
                    pc = 0x0042578c;
                    continue;
                }
                0x0042578c => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x00425790;
                    continue;
                }
                0x00425790 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x004257b0;
                        continue;
                    }
                    pc = 0x0042579c;
                    continue;
                }
                0x0042579c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x004257d8;
                        continue;
                    }
                    pc = 0x004257b0;
                    continue;
                }
                0x004257b0 => {
                    c.cycles += 2;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x004257b8;
                    continue;
                }
                0x004257b8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[S1][0] & 0x7;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[S1][0] = c.g[S1][0] >> 3;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    let t = c.g[S1][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x004257b8) {
                            return Ok(());
                        }
                        pc = 0x004257b8;
                        continue;
                    }
                    pc = 0x004257d8;
                    continue;
                }
                0x004257d8 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x004257e4;
                        continue;
                    }
                    pc = 0x004257e0;
                    continue;
                }
                0x004257e0 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x004257e4;
                    continue;
                }
                0x004257e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004257f0;
                    continue;
                }
                0x004257f0 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004257f8;
                    continue;
                }
                0x004257f8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0042580c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x0042580c)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004257f8) {
                            return Ok(());
                        }
                        pc = 0x004257f8;
                        continue;
                    }
                    pc = 0x00425818;
                    continue;
                }
                0x00425818 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425820;
                    continue;
                }
                0x00425820 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x00425838;
                        continue;
                    }
                    pc = 0x0042582c;
                    continue;
                }
                0x0042582c => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042584c;
                        continue;
                    }
                    pc = 0x00425838;
                    continue;
                }
                0x00425838 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00425848;
                        continue;
                    }
                    pc = 0x00425840;
                    continue;
                }
                0x00425840 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042584c;
                        continue;
                    }
                    pc = 0x00425848;
                    continue;
                }
                0x00425848 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0042584c;
                    continue;
                }
                0x0042584c => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x0042586c;
                        continue;
                    }
                    pc = 0x00425858;
                    continue;
                }
                0x00425858 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425898;
                        continue;
                    }
                    pc = 0x0042586c;
                    continue;
                }
                0x0042586c => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425870;
                    continue;
                }
                0x00425870 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S1][0] & 0xf;
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V1][0].wrapping_add(0x30);
                    if t {
                        pc = 0x00425884;
                        continue;
                    }
                    pc = 0x00425880;
                    continue;
                }
                0x00425880 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[V1][0].wrapping_add(0x57);
                    pc = 0x00425884;
                    continue;
                }
                0x00425884 => {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[S1][0] = c.g[S1][0] >> 4;
                    let t = c.g[S1][0] != 0u64;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    if t {
                        if back_edge_unwinds(c, 0x00425870) {
                            return Ok(());
                        }
                        pc = 0x00425870;
                        continue;
                    }
                    pc = 0x00425898;
                    continue;
                }
                0x00425898 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x004258a4;
                        continue;
                    }
                    pc = 0x004258a0;
                    continue;
                }
                0x004258a0 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x004258a4;
                    continue;
                }
                0x004258a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004258b0;
                    continue;
                }
                0x004258b0 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004258b8;
                    continue;
                }
                0x004258b8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x004258cc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x004258cc)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004258b8) {
                            return Ok(());
                        }
                        pc = 0x004258b8;
                        continue;
                    }
                    pc = 0x004258d8;
                    continue;
                }
                0x004258d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x004258e0;
                    continue;
                }
                0x004258e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x004258f8;
                        continue;
                    }
                    pc = 0x004258ec;
                    continue;
                }
                0x004258ec => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042590c;
                        continue;
                    }
                    pc = 0x004258f8;
                    continue;
                }
                0x004258f8 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00425908;
                        continue;
                    }
                    pc = 0x00425900;
                    continue;
                }
                0x00425900 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lh((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x0042590c;
                        continue;
                    }
                    pc = 0x00425908;
                    continue;
                }
                0x00425908 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0042590c;
                    continue;
                }
                0x0042590c => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x00425928;
                        continue;
                    }
                    pc = 0x00425918;
                    continue;
                }
                0x00425918 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    let t = true;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    if t {
                        pc = 0x00425988;
                        continue;
                    }
                    pc = 0x00425928;
                    continue;
                }
                0x00425928 => {
                    c.cycles += 2;
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S5][0] = 0x450000;
                    if t {
                        pc = 0x00425940;
                        continue;
                    }
                    pc = 0x00425930;
                    continue;
                }
                0x00425930 => {
                    c.cycles += 4;
                    c.g[S1][0] = 0u64.wrapping_sub(c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425940);
                    c.g[A0][0] = 0x2d;
                    call_ptr(c, t, 0x00425940)?;
                    pc = 0x00425940;
                    continue;
                }
                0x00425940 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0042598c;
                        continue;
                    }
                    pc = 0x00425948;
                    continue;
                }
                0x00425948 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00425950;
                    continue;
                }
                0x00425950 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00425958);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0042d470,
                        f: ee::s15::__moddi3,
                    }
                    .call(c, 0x00425958)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00425974);
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    Callee {
                        addr: 0x0040fcb0,
                        f: ee::s15::__divdi3,
                    }
                    .call(c, 0x00425974)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425950) {
                            return Ok(());
                        }
                        pc = 0x00425950;
                        continue;
                    }
                    pc = 0x00425980;
                    continue;
                }
                0x00425980 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0042598c;
                        continue;
                    }
                    pc = 0x00425988;
                    continue;
                }
                0x00425988 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x0042598c;
                    continue;
                }
                0x0042598c => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x00425998;
                        continue;
                    }
                    pc = 0x00425994;
                    continue;
                }
                0x00425994 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x00425998;
                    continue;
                }
                0x00425998 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x004259a4;
                    continue;
                }
                0x004259a4 => {
                    c.cycles += 1;
                    c.g[S5][0] = 0x450000;
                    pc = 0x004259a8;
                    continue;
                }
                0x004259a8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x004259bc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x004259bc)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x004259a8) {
                            return Ok(());
                        }
                        pc = 0x004259a8;
                        continue;
                    }
                    pc = 0x004259c8;
                    continue;
                }
                0x004259c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x004259d0;
                    continue;
                }
                0x004259d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6c;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x68;
                    if t {
                        pc = 0x004259e8;
                        continue;
                    }
                    pc = 0x004259dc;
                    continue;
                }
                0x004259dc => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[S1][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x004259fc;
                        continue;
                    }
                    pc = 0x004259e8;
                    continue;
                }
                0x004259e8 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x004259f8;
                        continue;
                    }
                    pc = 0x004259f0;
                    continue;
                }
                0x004259f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t {
                        pc = 0x004259fc;
                        continue;
                    }
                    pc = 0x004259f8;
                    continue;
                }
                0x004259f8 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x004259fc;
                    continue;
                }
                0x004259fc => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                    let t = c.g[S1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1f), 0u64);
                    if t {
                        pc = 0x00425a1c;
                        continue;
                    }
                    pc = 0x00425a08;
                    continue;
                }
                0x00425a08 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x30;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1e), c.g[V0][0]);
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425a58;
                        continue;
                    }
                    pc = 0x00425a1c;
                    continue;
                }
                0x00425a1c => {
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00425a28;
                    continue;
                }
                0x00425a28 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00425a30);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0042dad8,
                        f: ee::s15::__umoddi3,
                    }
                    .call(c, 0x00425a30)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00425a4c);
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    Callee {
                        addr: 0x00411538,
                        f: ee::s15::__udivdi3,
                    }
                    .call(c, 0x00425a4c)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425a28) {
                            return Ok(());
                        }
                        pc = 0x00425a28;
                        continue;
                    }
                    pc = 0x00425a58;
                    continue;
                }
                0x00425a58 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = (c.g[S4][0] < c.g[S0][0]) as u64;
                    if t {
                        pc = 0x00425a64;
                        continue;
                    }
                    pc = 0x00425a60;
                    continue;
                }
                0x00425a60 => {
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S4][0];
                    }
                    pc = 0x00425a64;
                    continue;
                }
                0x00425a64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x00425a70;
                    continue;
                }
                0x00425a70 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x450000;
                    pc = 0x00425a78;
                    continue;
                }
                0x00425a78 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425a8c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425a8c)?;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00425a78) {
                            return Ok(());
                        }
                        pc = 0x00425a78;
                        continue;
                    }
                    pc = 0x00425a98;
                    continue;
                }
                0x00425a98 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425aa0;
                    continue;
                }
                0x00425aa0 => {
                    c.cycles += 7;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.mtc1(0, 0);
                    c.lwc1(12, (c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.c_eq_s(12, 0);
                    let t = !c.fcc();
                    c.g[S5][0] = 0x450000;
                    if t {
                        pc = 0x00425ad4;
                        continue;
                    }
                    pc = 0x00425abc;
                    continue;
                }
                0x00425abc => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x30;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425acc);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425acc)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425ad4;
                    continue;
                }
                0x00425ad4 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00425adc);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00413af8,
                        f: ee::s15::__extendsfdf2,
                    }
                    .call(c, 0x00425adc)?;
                    c.g[RA][0] = sx(0x00425ae4);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x004254d0,
                        f: ee::s15::libkernl_PrintDouble,
                    }
                    .call(c, 0x00425ae4)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425aec;
                    continue;
                }
                0x00425aec => {
                    c.cycles += 5;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    if t {
                        pc = 0x00425b58;
                        continue;
                    }
                    pc = 0x00425b00;
                    continue;
                }
                0x00425b00 => {
                    c.cycles += 22;
                    c.g[S5][0] = 0x450000;
                    c.g[A0][0] = 0x28;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b14);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425b14)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b20);
                    c.g[A0][0] = 0x6e;
                    call_ptr(c, t, 0x00425b20)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b2c);
                    c.g[A0][0] = 0x75;
                    call_ptr(c, t, 0x00425b2c)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b38);
                    c.g[A0][0] = 0x6c;
                    call_ptr(c, t, 0x00425b38)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425b44);
                    c.g[A0][0] = 0x6c;
                    call_ptr(c, t, 0x00425b44)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b50);
                    c.g[A0][0] = 0x29;
                    call_ptr(c, t, 0x00425b50)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425b58;
                    continue;
                }
                0x00425b58 => {
                    c.cycles += 4;
                    c.g[S0][0] = c.g[V1][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S5][0] = 0x450000;
                    pc = 0x00425b68;
                    continue;
                }
                0x00425b68 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425b7c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425b7c)?;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x00425b68) {
                            return Ok(());
                        }
                        pc = 0x00425b68;
                        continue;
                    }
                    pc = 0x00425b88;
                    continue;
                }
                0x00425b88 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425b90;
                    continue;
                }
                0x00425b90 => {
                    c.cycles += 10;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = 0x450000;
                    c.g[S1][0] = c.lb((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5178));
                    c.g[A0][0] = c.g[S1][0] << 32;
                    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00425bb0);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425bb0)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425bb8;
                    continue;
                }
                0x00425bb8 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5178));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00425bc8);
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x00425bc8)?;
                    let t = true;
                    c.g[S0][0] = c.g[S2][0];
                    if t {
                        pc = 0x00425be0;
                        continue;
                    }
                    pc = 0x00425bd0;
                    continue;
                }
                0x00425bd0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00425bdc;
                        continue;
                    }
                    pc = 0x00425bd8;
                    continue;
                }
                0x00425bd8 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00425bdc;
                    continue;
                }
                0x00425bdc => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S2][0];
                    pc = 0x00425be0;
                    continue;
                }
                0x00425be0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00425678) {
                            return Ok(());
                        }
                        pc = 0x00425678;
                        continue;
                    }
                    pc = 0x00425bec;
                    continue;
                }
                0x00425bec => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x00425c00;
                        continue;
                    }
                    pc = 0x00425bf4;
                    continue;
                }
                0x00425bf4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00425bfc);
                    Callee {
                        addr: 0x0042c0c0,
                        f: ee::s15::EIntr,
                    }
                    .call(c, 0x00425bfc)?;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x00425c00;
                    continue;
                }
                0x00425c00 => {
                    c.cycles += 9;
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00425638, 0x00425c24);
                    return Ok(());
                }
            }
        }
    });
}

/// `void libkernl_WaitVBlankStart(void)` (0x424230): wait for the next
/// VSYNC with interrupts managed around the acknowledge.
pub fn libkernl_wait_vblank_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_424268: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00424240);
            DINTR.call(c, 0x00424240)?;
            c.g[V1][0] = 0x10000000;
            c.g[A0][0] = 0x4;
            c.g[V1][0] = c.g[V1][0] | 0xf000;
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0x10000000;
            if t {
                break 'b_424268;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00424264);
                callee::EINTR.call(c, 0x00424264)?;
                c.g[V1][0] = 0x10000000;
                break 'b_424268;
            }
        }
        c.cycles += 2;
        c.g[V1][0] = c.g[V1][0] | 0xf000;
        'l_424270: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] & 0x4;
            let t = c.g[V0][0] == 0u64;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00424270);
                }
                continue 'l_424270;
            } else {
                c.cycles += 9;
                c.g[RA][0] = sx(0x00424294);
                DINTR.call(c, 0x00424294)?;
                c.g[V1][0] = 0x10000000;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = c.g[V1][0] | 0xf000;
                c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                } else {
                    c.cycles += 2;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = 0x0042_c0c0;
                    (ee::s15::f_0042c0c0)(c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void libkernl_WaitVBlankOrFlag(void)` (0x4242c0): wait for VSYNC or a
/// flag set by the vsync callback, with interrupts managed around the
/// acknowledge.
pub fn libkernl_wait_vblank_or_flag(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_424308: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A0][0] = c.g[SP][0];
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[RA][0] = sx(0x004242d8);
            c.g[A1][0] = c.g[SP][0] | 0x8;
            SET_VSYNC_FLAG.call(c, 0x004242d8)?;
            c.g[RA][0] = sx(0x004242e0);
            DINTR.call(c, 0x004242e0)?;
            c.g[V1][0] = 0x10000000;
            c.g[A0][0] = 0x4;
            c.g[V1][0] = c.g[V1][0] | 0xf000;
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0x10000000;
            if t {
                break 'b_424308;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00424304);
                callee::EINTR.call(c, 0x00424304)?;
                c.g[V1][0] = 0x10000000;
                break 'b_424308;
            }
        }
        c.cycles += 2;
        c.g[V1][0] = c.g[V1][0] | 0xf000;
        'l_424310: loop {
            'b_42432c: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 0x4;
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_42432c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00424310);
                        }
                        continue 'l_424310;
                    } else {
                        break 'b_42432c;
                    }
                }
            }
            'b_424354: {
                c.cycles += 8;
                c.g[RA][0] = sx(0x00424334);
                DINTR.call(c, 0x00424334)?;
                c.g[V1][0] = 0x4;
                c.g[AT][0] = 0x10010000;
                c.sw((c.g[AT][0] as u32).wrapping_sub(0x1000), c.g[V1][0]);
                let t = c.g[V0][0] == 0u64;
                if t {
                    break 'b_424354;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00424354);
                    callee::EINTR.call(c, 0x00424354)?;
                    break 'b_424354;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = t;
            return Ok(());
        }
    })
}

/// `int DisableIntc(int line)` (0x424880): mask an interrupt line,
/// returning the previous enable state.
pub fn disable_intc_guarded(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_4248b0: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[S0][0]);
            c.mfc0(16, 12);
            c.g[V0][0] = 0x10000;
            c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            if t {
                break 'b_4248b0;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x004248b0);
                DINTR.call(c, 0x004248b0)?;
                break 'b_4248b0;
            }
        }
        'b_4248d4: {
            c.cycles += 6;
            c.g[RA][0] = sx(0x004248b8);
            c.g[A0][0] = c.g[S1][0];
            DISABLE_INTC_FN.call(c, 0x004248b8)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0];
            if t {
                break 'b_4248d4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x004248d0);
                callee::EINTR.call(c, 0x004248d0)?;
                c.g[V0][0] = c.g[S1][0];
                break 'b_4248d4;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int EnableIntc(int line)` (0x4248e8): unmask an interrupt line,
/// returning the previous enable state.
pub fn enable_intc_guarded(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_424918: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[S0][0]);
            c.mfc0(16, 12);
            c.g[V0][0] = 0x10000;
            c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            if t {
                break 'b_424918;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00424918);
                DINTR.call(c, 0x00424918)?;
                break 'b_424918;
            }
        }
        'b_42493c: {
            c.cycles += 6;
            c.g[RA][0] = sx(0x00424920);
            c.g[A0][0] = c.g[S1][0];
            ENABLE_INTC_FN.call(c, 0x00424920)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0];
            if t {
                break 'b_42493c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00424938);
                callee::EINTR.call(c, 0x00424938)?;
                c.g[V0][0] = c.g[S1][0];
                break 'b_42493c;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void libkernl_IntrHelperThread(void *queue)` (0x424aa0): helper-thread
/// main dispatching queued interrupt events (Wakeup/Rotate/Suspend/Printf
/// by queue byte). It never returns by design; no `OVERRIDES` entry (see
/// `intr_helper_thread_prefix_matches`, which checks the prologue and the
/// first `WaitSema` via a forced unwind).
#[allow(dead_code)]
pub fn libkernl_intr_helper_thread(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S5][0]);
        c.g[S6][0] = 0x530000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
        c.g[S5][0] = 0x4a0000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
        c.g[S4][0] = 0x1;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[S3][0] = 0x2;
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9));
        'l_424ae0: loop {
            'b_424b68: {
                'b_424b58: {
                    'b_424b48: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x00424ae8);
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x4c60));
                        WAIT_SEMA.call(c, 0x00424ae8)?;
                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[V1][0] = c.g[V1][0] & 0x1ff;
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        let t = c.g[V0][0] == c.g[S4][0];
                        c.g[A1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                        if t {
                            break 'b_424b48;
                        } else {
                            'b_424b38: {
                                'b_424b28: {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5520));
                                    if t {
                                        break 'b_424b28;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = c.g[A2][0];
                                        if t {
                                            break 'b_424b38;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            if t {
                                                break 'b_424b68;
                                            } else {
                                                break 'b_424b28;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S3][0];
                                c.g[V1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                if t {
                                    break 'b_424b58;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    if t {
                                        break 'b_424b68;
                                    } else {
                                        break 'b_424b38;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00424b40);
                            c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                            WAKEUP_THREAD.call(c, 0x00424b40)?;
                            let t = true;
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00424ae0);
                                }
                                continue 'l_424ae0;
                            } else {
                                break 'b_424b48;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00424b50);
                    c.g[A0][0] = c.lbu(c.g[A2][0] as u32);
                    ROTATE_QUEUE.call(c, 0x00424b50)?;
                    let t = true;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00424ae0);
                        }
                        continue 'l_424ae0;
                    } else {
                        break 'b_424b58;
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x00424b60);
                c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                SUSPEND_THREAD.call(c, 0x00424b60)?;
                let t = true;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00424ae0);
                    }
                    continue 'l_424ae0;
                } else {
                    break 'b_424b68;
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x00424b70);
            PRINTF.call(c, 0x00424b70)?;
            let t = true;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00424ae0);
                }
                continue 'l_424ae0;
            } else {
                c.pc = 0x0042_4b78;
                (ee::s15::f_00424b78__generated)(c);
                return Ok(());
            }
        }
    })
}

/// `int libkernl_InitThreadHelper(void)` (0x424b78): create the interrupt
/// helper's sema and thread, start the thread and drop the caller to
/// priority 1. Returns the thread id; -1 when a thread id is already set or
/// a create fails (a failed thread create deletes the sema again).
pub fn libkernl_init_thread_helper(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_424c38: {
            'b_424c08: {
                'b_424c00: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                    c.g[S0][0] = 0x450000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5170));
                    let t = (c.g[V0][0] as i64) > 0;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                    if t {
                        break 'b_424c00;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = 0xff;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                        c.g[RA][0] = sx(0x00424ba8);
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        CREATE_SEMA.call(c, 0x00424ba8)?;
                        c.g[S1][0] = 0x530000;
                        let t = (c.g[V0][0] as i64) < 0;
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x4c60), c.g[V0][0]);
                        if t {
                            break 'b_424c00;
                        } else {
                            c.cycles += 17;
                            c.g[V0][0] = 0x420000;
                            c.g[V1][0] = 0x530000;
                            c.g[A1][0] = 0x4a0000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5060));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30f0));
                            c.g[A2][0] = 0x400;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.g[A0][0] = c.g[SP][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.g[RA][0] = sx(0x00424bec);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                            CREATE_THREAD.call(c, 0x00424bec)?;
                            c.g[A0][0] = c.g[V0][0];
                            let t = (c.g[A0][0] as i64) >= 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5170), c.g[A0][0]);
                            if t {
                                break 'b_424c08;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x00424c00);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x4c60));
                                DELETE_SEMA.call(c, 0x00424c00)?;
                                break 'b_424c00;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_424c38;
                } else {
                    break 'b_424c08;
                }
            }
            c.cycles += 12;
            c.g[V0][0] = 0x530000;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c58));
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x4c58), 0u64);
            c.g[A1][0] = c.g[V1][0];
            c.g[RA][0] = sx(0x00424c20);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            START_THREAD.call(c, 0x00424c20)?;
            c.g[RA][0] = sx(0x00424c28);
            GET_THREAD_ID.call(c, 0x00424c28)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00424c34);
            c.g[A1][0] = 0x1;
            CHANGE_THREAD_PRIORITY.call(c, 0x00424c34)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5170));
            break 'b_424c38;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    })
}

/// `void libkernl_TLBExceptionHandler(void)` (0x42cfc0): save the thread
/// context (GPRs, HI/LO, SA, COP0) on a TLB refill miss.
pub fn libkernl_tlb_exception_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 64;
        c.g[K0][0] = 0x530000;
        c.g[K0][0] = sx((c.g[K0][0] as u32).wrapping_add(0x380));
        c.sq(1, (c.g[K0][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[K0][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[K0][0] as u32).wrapping_add(0x30));
        c.sq(4, (c.g[K0][0] as u32).wrapping_add(0x40));
        c.sq(5, (c.g[K0][0] as u32).wrapping_add(0x50));
        c.sq(6, (c.g[K0][0] as u32).wrapping_add(0x60));
        c.sq(7, (c.g[K0][0] as u32).wrapping_add(0x70));
        c.sq(8, (c.g[K0][0] as u32).wrapping_add(0x80));
        c.sq(9, (c.g[K0][0] as u32).wrapping_add(0x90));
        c.sq(10, (c.g[K0][0] as u32).wrapping_add(0xa0));
        c.sq(11, (c.g[K0][0] as u32).wrapping_add(0xb0));
        c.sq(12, (c.g[K0][0] as u32).wrapping_add(0xc0));
        c.sq(13, (c.g[K0][0] as u32).wrapping_add(0xd0));
        c.sq(14, (c.g[K0][0] as u32).wrapping_add(0xe0));
        c.sq(15, (c.g[K0][0] as u32).wrapping_add(0xf0));
        c.sq(16, (c.g[K0][0] as u32).wrapping_add(0x100));
        c.sq(17, (c.g[K0][0] as u32).wrapping_add(0x110));
        c.sq(18, (c.g[K0][0] as u32).wrapping_add(0x120));
        c.sq(19, (c.g[K0][0] as u32).wrapping_add(0x130));
        c.sq(20, (c.g[K0][0] as u32).wrapping_add(0x140));
        c.sq(21, (c.g[K0][0] as u32).wrapping_add(0x150));
        c.sq(22, (c.g[K0][0] as u32).wrapping_add(0x160));
        c.sq(23, (c.g[K0][0] as u32).wrapping_add(0x170));
        c.sq(24, (c.g[K0][0] as u32).wrapping_add(0x180));
        c.sq(25, (c.g[K0][0] as u32).wrapping_add(0x190));
        c.sq(28, (c.g[K0][0] as u32).wrapping_add(0x1c0));
        c.sq(29, (c.g[K0][0] as u32).wrapping_add(0x1d0));
        c.sq(30, (c.g[K0][0] as u32).wrapping_add(0x1e0));
        c.sq(31, (c.g[K0][0] as u32).wrapping_add(0x1f0));
        c.g[V0][0] = c.hi[0];
        c.g[AT][0] = 0x530000;
        c.sd((c.g[AT][0] as u32).wrapping_add(0x580), c.g[V0][0]);
        c.g[V0][0] = c.hi[1];
        c.g[AT][0] = 0x530000;
        c.sd((c.g[AT][0] as u32).wrapping_add(0x588), c.g[V0][0]);
        c.g[V0][0] = c.lo[0];
        c.g[AT][0] = 0x530000;
        c.sd((c.g[AT][0] as u32).wrapping_add(0x590), c.g[V0][0]);
        c.g[V0][0] = c.lo[1];
        c.g[AT][0] = 0x530000;
        c.sd((c.g[AT][0] as u32).wrapping_add(0x598), c.g[V0][0]);
        c.g[V0][0] = c.sa as u64;
        c.g[AT][0] = 0x530000;
        c.sd((c.g[AT][0] as u32).wrapping_add(0x5a0), c.g[V0][0]);
        c.mfc0(4, 12);
        c.mfc0(5, 13);
        c.mfc0(6, 14);
        c.mfc0(7, 8);
        c.g[T0][0] = 0x530000;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x380));
        c.g[AT][0] = 0x530000;
        c.sw((c.g[AT][0] as u32).wrapping_add(0x5a8), c.g[A2][0]);
        c.g[AT][0] = 0x430000;
        c.g[AT][0] = sx((c.g[AT][0] as u32).wrapping_sub(0x2f40));
        c.mtc0(1, 14);
        c.mfc0(1, 12);
        c.g[V0][0] = 0xfffffffffffffffe;
        c.g[AT][0] = c.g[AT][0] & c.g[V0][0];
        c.mtc0(1, 12);
        c.pc = c.cop0[14];
        Ok(())
    })
}

/// `void libkernl_ExceptionReturnHandler(void)` (0x42d100): restore the
/// thread context saved by the TLB handler and return to EPC.
pub fn libkernl_exception_return_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        c.mfc0(1, 12);
        c.g[K0][0] = 0xffffffffffffffe4;
        c.g[AT][0] = c.g[AT][0] & c.g[K0][0];
        c.mtc0(1, 12);
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5a8));
        c.mtc0(2, 14);
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x580));
        c.hi[0] = c.g[V0][0];
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x588));
        c.hi[1] = c.g[V0][0];
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x590));
        c.lo[0] = c.g[V0][0];
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x598));
        c.lo[1] = c.g[V0][0];
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x5a0));
        c.sa = c.g[V0][0] as u32;
        c.g[K0][0] = 0x530000;
        c.g[K0][0] = sx((c.g[K0][0] as u32).wrapping_add(0x380));
        c.lq(1, (c.g[K0][0] as u32).wrapping_add(0x10));
        c.lq(2, (c.g[K0][0] as u32).wrapping_add(0x20));
        c.lq(3, (c.g[K0][0] as u32).wrapping_add(0x30));
        c.lq(4, (c.g[K0][0] as u32).wrapping_add(0x40));
        c.lq(5, (c.g[K0][0] as u32).wrapping_add(0x50));
        c.lq(6, (c.g[K0][0] as u32).wrapping_add(0x60));
        c.lq(7, (c.g[K0][0] as u32).wrapping_add(0x70));
        c.lq(8, (c.g[K0][0] as u32).wrapping_add(0x80));
        c.lq(9, (c.g[K0][0] as u32).wrapping_add(0x90));
        c.lq(10, (c.g[K0][0] as u32).wrapping_add(0xa0));
        c.lq(11, (c.g[K0][0] as u32).wrapping_add(0xb0));
        c.lq(12, (c.g[K0][0] as u32).wrapping_add(0xc0));
        c.lq(13, (c.g[K0][0] as u32).wrapping_add(0xd0));
        c.lq(14, (c.g[K0][0] as u32).wrapping_add(0xe0));
        c.lq(15, (c.g[K0][0] as u32).wrapping_add(0xf0));
        c.lq(16, (c.g[K0][0] as u32).wrapping_add(0x100));
        c.lq(17, (c.g[K0][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[K0][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[K0][0] as u32).wrapping_add(0x130));
        c.lq(20, (c.g[K0][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[K0][0] as u32).wrapping_add(0x150));
        c.lq(22, (c.g[K0][0] as u32).wrapping_add(0x160));
        c.lq(23, (c.g[K0][0] as u32).wrapping_add(0x170));
        c.lq(24, (c.g[K0][0] as u32).wrapping_add(0x180));
        c.lq(25, (c.g[K0][0] as u32).wrapping_add(0x190));
        c.lq(28, (c.g[K0][0] as u32).wrapping_add(0x1c0));
        c.lq(29, (c.g[K0][0] as u32).wrapping_add(0x1d0));
        c.lq(30, (c.g[K0][0] as u32).wrapping_add(0x1e0));
        c.lq(31, (c.g[K0][0] as u32).wrapping_add(0x1f0));
        c.mfc0(26, 12);
        c.g[K0][0] = c.g[K0][0] | 0x13;
        c.mtc0(26, 12);
        c.pc = c.cop0[14];
        Ok(())
    })
}

/// `kCopy_3_Impl(dst, src, count)` (0x42c4d0): copies `count` bytes from
/// `src` to `dst` one at a time, then returns 0. The loop checks events at
/// its back edge (head 0x42c4d8).
pub fn kcopy_3_impl(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A3][0] = 0u64;
    if c.g[A2][0] != 0u64 {
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            c.sb(c.g[A0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0042_c4d8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// The word copy shared by `kCopy_4_Impl` and `kCopy_5_Impl`: `a2` is a byte
/// count, copied as `a2 >> 2` words from `a1` to `a0`, then returns 0. The
/// back edge checks events at `head`.
pub(crate) fn kcopy_words(c: &mut Cpu, head: u32) {
    c.cycles += 3;
    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
    let empty = c.g[A2][0] == 0u64;
    c.g[A3][0] = 0u64;
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `kCopy_4_Impl(dst, src, bytes)` (0x42c780): word copy, see [`kcopy_words`].
pub fn kcopy_4_impl(c: &mut Cpu) {
    kcopy_words(c, 0x0042_c790);
}

/// `kCopy_5_Impl(dst, src, bytes)` (0x42cb78): the same word copy as
/// [`kcopy_4_impl`], with its own loop head (0x42cb88).
pub fn kcopy_5_impl(c: &mut Cpu) {
    kcopy_words(c, 0x0042_cb88);
}

/// Internal arms of [`libkernl_intr_helper_thread`] (not guest addresses: they
/// never reach `c.pc`).
pub(crate) const ARM_WAKEUP: u32 = 0xfff0_0001;

pub(crate) const ARM_ROTATE: u32 = 0xfff0_0002;

pub(crate) const ARM_SUSPEND: u32 = 0xfff0_0003;

pub(crate) const ARM_PRINTF: u32 = 0xfff0_0004;

/// `void libkernl_IntrHelperThread(void *queue)` (0x00424aa0): the interrupt
/// helper thread's main. Forever: `WaitSema(sema)`, pop the next 2-byte record
/// of the queue (`queue[0] & 0x1ff` is the read index; type at `+8 + 2 * idx`,
/// argument at `+9 + 2 * idx`) and dispatch on its type: 0 `WakeupThread(arg)`,
/// 1 `RotateThreadReadyQueue(arg)`, 2 `SuspendThread(arg)`, anything else
/// `printf(fmt)`. It never returns. A `match pc` body so a thread that
/// unwound inside a callee (the parked `WaitSema`) resumes in the middle:
/// every return address of the generated body and the loop head are arms. The
/// generated body charges the whole receive block (12 cycles) before the
/// `WaitSema` call, so the resume arm after it charges nothing.
pub fn libkernl_intr_helper_thread_2(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0042_4aa0 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
                    c.sd(sp(c).wrapping_add(0x10), c.g[S1][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(sp(c).wrapping_add(0x60), c.g[S6][0]);
                    c.sd(sp(c).wrapping_add(0x50), c.g[S5][0]);
                    c.g[S6][0] = 0x530000;
                    c.sd(sp(c).wrapping_add(0x40), c.g[S4][0]);
                    c.g[S5][0] = 0x4a0000;
                    c.sd(sp(c).wrapping_add(0x30), c.g[S3][0]);
                    c.g[S4][0] = 0x1;
                    c.sd(sp(c).wrapping_add(0x20), c.g[S2][0]);
                    c.g[S3][0] = 0x2;
                    c.sd(sp(c), c.g[S0][0]);
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sd(sp(c).wrapping_add(0x70), c.g[RA][0]);
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9));
                    pc = 0x0042_4ae0;
                }
                // loop head: wait for the semaphore
                0x0042_4ae0 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x4c60));
                    Callee {
                        addr: 0x0042_3de0,
                        f: ee::s15::WaitSema,
                    }
                    .call(c, 0x0042_4ae8)?;
                    pc = 0x0042_4ae8;
                }
                // after WaitSema: pop the record and dispatch on its type
                0x0042_4ae8 => {
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] &= 0x1ff;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                    let is_rotate = c.g[V0][0] == c.g[S4][0];
                    c.g[A1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                    if is_rotate {
                        pc = ARM_ROTATE;
                        continue;
                    }
                    c.cycles += 2;
                    let not_small = c.g[A1][0] == 0;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5520));
                    if not_small {
                        c.cycles += 2;
                        let is_suspend = c.g[V0][0] == c.g[S3][0];
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        if is_suspend {
                            pc = ARM_SUSPEND;
                        } else {
                            c.cycles += 2;
                            pc = ARM_PRINTF;
                        }
                    } else {
                        c.cycles += 2;
                        let is_wakeup = c.g[V0][0] == 0;
                        c.g[V1][0] = c.g[A2][0];
                        if is_wakeup {
                            pc = ARM_WAKEUP;
                        } else {
                            c.cycles += 2;
                            pc = ARM_PRINTF;
                        }
                    }
                }
                // type 0: WakeupThread(arg)
                ARM_WAKEUP => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0042_3cd0,
                        f: ee::s15::WakeupThread,
                    }
                    .call(c, 0x0042_4b40)?;
                    pc = 0x0042_4b40;
                }
                // type 1: RotateThreadReadyQueue(arg)
                ARM_ROTATE => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[A2][0] as u32);
                    Callee {
                        addr: 0x0042_3c50,
                        f: ee::s15::RotateThreadReadyQueue,
                    }
                    .call(c, 0x0042_4b50)?;
                    pc = 0x0042_4b50;
                }
                // type 2: SuspendThread(arg)
                ARM_SUSPEND => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0042_3d10,
                        f: ee::s15::SuspendThread,
                    }
                    .call(c, 0x0042_4b60)?;
                    pc = 0x0042_4b60;
                }
                // other types: printf(fmt)
                ARM_PRINTF => {
                    c.cycles += 4;
                    Callee {
                        addr: 0x0042_5c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x0042_4b70)?;
                    pc = 0x0042_4b70;
                }
                // back edge after any handler
                0x0042_4b40 | 0x0042_4b50 | 0x0042_4b60 | 0x0042_4b70 => {
                    if back_edge_unwinds(c, 0x0042_4ae0) {
                        return Ok(());
                    }
                    pc = 0x0042_4ae0;
                }
                _ => {
                    c.jump_out(pc, 0x0042_4aa0, 0x0042_4b78);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::require_paths;

    /// Bounded TLB config: entry count 48-52 (49+ takes the overflow-`Exit`
    /// path), small second/third bounds, and the three entry lists in
    /// scratch RAM, so every walk terminates (used only by the setup_tlb
    /// tests below.)
    pub(crate) fn setup_tlb_tables(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(0x0045_67f0, 48 + (seed % 5));
        c.bus.write32(0x0045_67f4, 3);
        c.bus.write32(0x0045_67f8, 2);
        c.bus.write32(0x0045_6800, DATA + 0x204);
        c.bus.write32(0x0045_6804, DATA + 0x404);
        c.bus.write32(0x0045_6808, DATA + 0x604);
        crate::difftest::put_words(
            c,
            DATA + 0x204,
            &[
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
                DATA + 0x244,
                r.next_u32(),
            ],
        );
    }

    /// Craft an argv table: `argc` NUL-terminated strings after the pointer
    /// array, all in scratch RAM (used only by the copy_args tests below.)
    pub(crate) fn craft_argv(c: &mut Cpu, r: &mut Rng, argc: u32) {
        let base = DATA + 0x104;
        for i in 0..argc.min(8) {
            let s = base + 0x100 + i * 0x20;
            c.bus.write32(base + 4 * i, s);
            let len = 1 + r.next_u32() % 12;
            for j in 0..len {
                c.bus.write8(s + j, b'x' + ((i + j) % 20) as u8);
            }
            c.bus.write8(s + len, 0);
        }
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::lib::sif::tests::syscall_test as syscall_test_2;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS as SEEDS_2;

    pub(crate) const SEEDS: u32 = 200;

    // ------------------------------------------------------------ syscalls

    /// Every syscall wrapper: fake syscalls (which may unwind), all argument
    /// registers scrambled.
    pub(crate) fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            // A few varied pointers in the argument registers.
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn exit_matches() {
        syscall_test(ssx3_ee::s15::f_004239c0, exit, 1);
    }

    #[test]
    fn load_exec_ps2_matches() {
        syscall_test(ssx3_ee::s15::f_004239e0, load_exec_ps2, 2);
    }

    #[test]
    fn enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423ae0, enable_intc, 3);
    }

    #[test]
    fn disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423af0, disable_intc, 4);
    }

    #[test]
    fn enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b00, enable_dmac, 5);
    }

    #[test]
    fn disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b10, disable_dmac, 6);
    }

    #[test]
    fn i_enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b40, i_enable_intc, 7);
    }

    #[test]
    fn i_disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b50, i_disable_intc, 8);
    }

    #[test]
    fn i_enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b60, i_enable_dmac, 9);
    }

    #[test]
    fn i_disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b70, i_disable_dmac, 10);
    }

    #[test]
    fn start_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423bc0, start_thread, 11);
    }

    #[test]
    fn i_wakeup_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423ce0, i_wakeup_thread, 12);
    }

    #[test]
    fn i_suspend_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423d20, i_suspend_thread, 13);
    }

    /// `GsPutIMR`: the kernel wrappers' test (fake syscalls that may
    /// unwind).
    #[test]
    fn gs_put_imr_matches() {
        syscall_test(ee::s15::f_004240e0__generated, gs_put_imr, 0x71);
    }

    /// `GsPutIMR` with an event due right after the syscall (a fake that
    /// returns): `next_event` 0..7 cycles past entry (`seed / 3 % 8`), the
    /// wrapper charges 4 and the fake syscall at least 3, so the closing
    /// event check delivers on every event seed (`seed % 3` the mode).
    #[test]
    fn gs_put_imr_events() {
        crate::difftest::events::driven!(gen_put_imr, ee::s15::f_004240e0__generated);
        crate::difftest::events::driven!(hand_put_imr, gs_put_imr);
        check_events(gen_put_imr, hand_put_imr, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x0042_40e0, 0);
            c.g[A0][0] = r.next_u32() as u64;
            let events = schedule(c, &mut r, seed, 0x0042_40e0);
            if events != Events::Off {
                c.next_event = c.cycles + (seed / 3 % 8) as u64;
            }
            events
        });
    }

    #[test]
    fn exec_osd_matches() {
        syscall_test(ssx3_ee::s15::f_004241c0, exec_osd, 14);
    }

    #[test]
    fn init_tlb_matches() {
        syscall_test(ssx3_ee::s15::f_00424210, init_tlb, 15);
    }

    pub(crate) fn cache_test(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, salt, 0);
            c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
            c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
            schedule(c, &mut r, seed, entry)
        };
        check_events(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn sync_dcache_matches() {
        cache_test(ssx3_ee::s15::f_004245f0, sync_dcache, 0x0042_45f0, 70);
    }

    #[test]
    fn invalid_dcache_matches() {
        cache_test(ssx3_ee::s15::f_00424730, invalid_dcache, 0x0042_4730, 71);
    }

    // ------------------------------------------------------- syscall wrappers

    #[test]
    fn add_intc_and_dmac_handler_match() {
        for (gen, hand) in [
            (
                ee::s15::f_00423a80 as fn(&mut Cpu),
                add_intc_handler as fn(&mut Cpu),
            ),
            (ee::s15::f_00423a90, add_intc_handler_2),
            (ee::s15::f_00423ab0, add_dmac_handler),
            (ee::s15::f_00423ac0, add_dmac_handler_2),
        ] {
            covered(gen, hand, SEEDS_2, &|c, seed| {
                let _ = machine(c, seed, 0x423a80, 1);
            });
        }
    }

    #[test]
    fn get_thread_id_matches() {
        check(
            ssx3_ee::s15::f_00423c90__generated,
            get_thread_id,
            40,
            &|c, seed| {
                machine(c, seed, 0x423c_90, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn signal_sema_matches() {
        check(
            ssx3_ee::s15::f_00423dc0__generated,
            signal_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_c0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn i_signal_sema_matches() {
        check(
            ssx3_ee::s15::f_00423dd0__generated,
            i_signal_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_d0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn wait_sema_matches() {
        check(
            ssx3_ee::s15::f_00423de0__generated,
            wait_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_e0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn flush_cache_matches() {
        syscall_test_2(ssx3_ee::s15::f_00424020__generated, flush_cache, 36);
    }

    #[test]
    fn poll_sema_matches() {
        syscall_test_2(ssx3_ee::s15::f_00423df0__generated, poll_sema, 37);
    }

    /// Data-cache invalidate: interrupts enabled or not on entry (Status
    /// bit 16, with other Status bits random), random ranges.
    #[test]
    fn invalid_dcache_matches_2() {
        check(
            ee::s15::f_004247d8__generated,
            invalid_dcache_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x47d8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0042_47d8;
                let status = r.next_u32() & !0x1_0000;
                c.cop0[12] = if seed % 2 == 0 {
                    status | 0x1_0000
                } else {
                    status
                };
                c.cop0[28] = r.next_u32();
                let start = r.next_u32() % 0x0200_0000;
                let end = start.wrapping_add(r.next_u32() % 0x2000);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | start as u64;
                c.g[A1][0] = sx(end);
            },
        );
    }

    /// SyncDCache: interrupts enabled or not on entry (Status bit 16, other
    /// Status bits random), random ranges; the real DIntr/_SyncDCache/EIntr.
    #[test]
    fn sync_dcache_matches_2() {
        check(
            ee::s15::f_00424698__generated,
            sync_dcache_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x4698 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0042_4698;
                let status = r.next_u32() & !0x1_0000;
                c.cop0[12] = if seed % 2 == 0 {
                    status | 0x1_0000
                } else {
                    status
                };
                c.cop0[28] = r.next_u32();
                let start = r.next_u32() % 0x0200_0000;
                let end = start.wrapping_add(r.next_u32() % 0x2000);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | start as u64;
                c.g[A1][0] = sx(end);
            },
        );
    }

    /// `_exit` through the real `Exit` / `InitTLB` chain with fake syscalls:
    /// `GetMemorySize` answers a hash (never 32 MB), so `_InitTLB` runs and
    /// `_Exit` returns to the caller; some seeds unwind in a syscall.
    #[test]
    fn crt_exit_matches() {
        check(ee::s00::f_001001c8__generated, crt_exit, 64, &|c, seed| {
            machine(c, seed, 0x0010_01c8, 3);
            c.pc = 0x0010_01c8;
        });
    }
    use crate::difftest::check_lohi;
    use crate::difftest::put_words;

    #[test]
    fn init_tlb_3_matches() {
        check(
            ee::s15::f_0042cd58__generated,
            init_tlb_3,
            SEEDS,
            &|c, seed| {
                use crate::difftest::fakes::LOG_RESULTS;
                machine(c, seed, 121003, 1);
                c.pc = 0x0042_cd58;
                // Force `GetMemorySize` to answer 32 MB on even seeds so the
                // `SetupTLB` arm runs; odd seeds take the fallback-TLB arm.
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 0x2000000);
                }
            },
        );
    }

    #[test]
    fn exec_osd_2_matches() {
        check(
            ee::s15::f_0042c710__generated,
            exec_osd_2,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121204, 1);
                c.pc = 0x0042_c710;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                c.g[A1][0] = (DATA + 0x104 + (r.next_u32() & 0xf0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn exit_2_matches() {
        check(ee::s15::f_0042c6e8__generated, exit_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 121205, 1);
            c.pc = 0x0042_c6e8;
            c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            let _ = r;
        });
    }

    #[test]
    fn load_exec_ps2_2_matches() {
        check(
            ee::s15::f_0042c6a0__generated,
            load_exec_ps2_2,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121206, 1);
                c.pc = 0x0042_c6a0;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                c.g[A1][0] = (DATA + 0x104 + (r.next_u32() & 0xf0)) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn init_tlb_2_matches() {
        check(
            ee::s15::f_0042c628__generated,
            init_tlb_2,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121207, 1);
                c.pc = 0x0042_c628;
            },
        );
    }

    #[test]
    fn libkernl_init_malloc_sema_matches() {
        check(
            ee::s15::f_0042c0d8__generated,
            libkernl_init_malloc_sema,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121302, 1);
                c.pc = 0x0042_c0d8;
            },
        );
    }

    #[test]
    fn kcopy_matches() {
        syscall_test(ee::s15::f_0042c120__generated, kcopy, 121221);
    }

    #[test]
    fn find_address_matches() {
        syscall_test(ee::s15::f_0042c1a8__generated, find_address, 121220);
    }

    #[test]
    fn get_syscall_handler_matches() {
        check(
            ee::s15::f_0042c1b8__generated,
            get_syscall_handler,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121219, 1);
                c.pc = 0x0042_c1b8;
                c.g[A0][0] = (r.next_u32() % 64) as u64;
                let _ = r;
            },
        );
    }

    #[test]
    fn libkernl_init_syscall_table_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            use crate::difftest::fakes::LOG_RESULTS;
            machine(c, seed, 121218, 1);
            c.pc = 0x0042_c1f0;
            // Force the two `FindAddress` results to agree, so the walk
            // converges immediately instead of wandering on fake hashes.
            put_words(c, LOG_RESULTS + 8, &[0x1000, 0xf5c]);
        };
        check(
            ee::s15::f_0042c1f0__generated,
            libkernl_init_syscall_table,
            SEEDS,
            setup,
        );
        require_paths(
            ee::s15::f_0042c1f0__generated,
            SEEDS,
            // Converged walk (90) + unwind (43).
            &[43, 90],
            setup,
        );
    }

    #[test]
    fn libkernl_init_syscall_table_events() {
        check_events(
            ee::s15::f_0042c1f0__generated,
            libkernl_init_syscall_table,
            300,
            &|c, seed| {
                use crate::difftest::fakes::LOG_RESULTS;
                let mut r = machine(c, seed, 121218, 0);
                c.pc = 0x0042_c1f0;
                // First miss, then agree after one trip round the loop
                // head, so the walk stays far under the call-trace cap:
                // even seeds exit through the first arm, odd seeds take
                // one trip through each arm and exit through the second.
                // n=0,1 (the two `RFU116` setups) read unwritten words and
                // return hashes, which the walk discards.
                if seed % 2 == 0 {
                    put_words(c, LOG_RESULTS + 8, &[0x1000, 0x1000, 0x10a4]);
                } else {
                    put_words(c, LOG_RESULTS + 8, &[0x1000, 0x1000, 0x900000, 0x8fff5c]);
                }
                schedule(c, &mut r, seed, 0x0042_c1f0)
            },
        );
    }

    #[test]
    fn rfu116_set_syscall_5_matches() {
        syscall_test(ee::s15::f_0042c2f0__generated, rfu116_set_syscall_5, 121217);
    }

    #[test]
    fn rfu116_set_syscall_4_matches() {
        syscall_test(ee::s15::f_0042c340__generated, rfu116_set_syscall_4, 121215);
    }

    #[test]
    fn kcopy_2_matches() {
        syscall_test(ee::s15::f_0042c350__generated, kcopy_2, 121214);
    }

    #[test]
    fn rfu091_3_matches() {
        syscall_test(ee::s15::f_0042c398__generated, rfu091_3, 121213);
    }

    #[test]
    fn libkernl_is_old_osd_config_matches() {
        check(
            ee::s15::f_0042c3a8__generated,
            libkernl_is_old_osd_config,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121212, 1);
                c.pc = 0x0042_c3a8;
            },
        );
    }

    #[test]
    fn init_osd_matches() {
        check(
            ee::s15::f_0042c410__generated,
            init_osd,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121211, 1);
                c.pc = 0x0042_c410;
            },
        );
    }

    #[test]
    fn init_osd_events() {
        check_events(ee::s15::f_0042c410__generated, init_osd, 300, &|c, seed| {
            let mut r = machine(c, seed, 121211, 0);
            c.pc = 0x0042_c410;
            schedule(c, &mut r, seed, 0x0042_c410)
        });
    }

    #[test]
    fn kcopy_3_matches() {
        syscall_test(ee::s15::f_0042c4c0__generated, kcopy_3, 121210);
    }

    #[test]
    fn libkernl_copy_args_to_kernel_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 121208, 1);
            c.pc = 0x0042_c510;
            // argc==16 exercises the S2<16 boundary (clamps to 0xf).
            let argc = if seed % 11 == 10 {
                16
            } else if seed % 2 == 0 {
                0
            } else {
                1 + seed % 3
            };
            c.g[A0][0] = (DATA + 0x84) as u64;
            c.g[A1][0] = argc as u64;
            c.g[A2][0] = (DATA + 0x104) as u64;
            put_words(c, DATA + 0x84, &[0x7a00_7a00, 0x7a00_7a00, 0]);
            craft_argv(c, &mut r, argc);
            // craft_argv caps the table at 8 entries; materialise the rest so
            // large argc values read valid pointers, not zero RAM.
            for i in 8..argc {
                let s = DATA + 0x104 + 0x100 + i * 0x20;
                c.bus.write32(DATA + 0x104 + 4 * i, s);
                c.bus.write32(s, 0x00787878);
            }
        };
        check(
            ee::s15::f_0042c510__generated,
            libkernl_copy_args_to_kernel,
            SEEDS,
            setup,
        );
    }

    #[test]
    fn libkernl_copy_args_to_kernel_events() {
        check_events(
            ee::s15::f_0042c510__generated,
            libkernl_copy_args_to_kernel,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121208, 0);
                c.pc = 0x0042_c510;
                c.g[A0][0] = (DATA + 0x84) as u64;
                c.g[A1][0] = 3;
                c.g[A2][0] = (DATA + 0x104) as u64;
                put_words(c, DATA + 0x84, &[0x7a00_7a00, 0x7a00_7a00, 0]);
                craft_argv(c, &mut r, 3);
                schedule(c, &mut r, seed, 0x0042_c510)
            },
        );
    }

    #[test]
    fn rfu116_set_syscall_2_matches() {
        syscall_test(ee::s15::f_0042c760__generated, rfu116_set_syscall_2, 121203);
    }

    #[test]
    fn kcopy_4_matches() {
        syscall_test(ee::s15::f_0042c770__generated, kcopy_4, 121202);
    }

    #[test]
    fn rfu091_2_matches() {
        syscall_test(ee::s15::f_0042c7b8__generated, rfu091_2, 121201);
    }

    #[test]
    fn init_alarm_matches() {
        check(
            ee::s15::f_0042c7c8__generated,
            init_alarm,
            SEEDS,
            &|c, seed| {
                let r = machine(c, seed, 121020, 1);
                c.pc = 0x0042_c7c8;
                // Set Timer 3 mode on some seeds so the armed/skip arm runs:
                // the skip arm returns the 0x10001810 address it built.
                if seed % 9 == 8 {
                    c.bus.write32(0x1000_1810, 0x100);
                }
                let _ = r;
            },
        );
    }

    #[test]
    fn init_alarm_events() {
        check_events(
            ee::s15::f_0042c7c8__generated,
            init_alarm,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121020, 0);
                // Clear the alarm flag on even seeds so the install path
                // (and its loop head) runs.
                if seed % 2 == 0 {
                    c.bus.write32(0x1000_1810, 0);
                }
                schedule(c, &mut r, seed, 0x0042_c7c8)
            },
        );
    }

    #[test]
    fn libkernl_install_tlb_refill_handlers_matches() {
        check(
            ee::s15::f_0042ca80__generated,
            libkernl_install_tlb_refill_handlers,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121010, 0);
                c.pc = 0x0042_ca80;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }

    #[test]
    fn kcopy_5_matches() {
        syscall_test(ee::s15::f_0042cb68__generated, kcopy_5, 121009);
    }

    #[test]
    fn rfu091_matches() {
        syscall_test(ee::s15::f_0042cbb0__generated, rfu091, 121008);
    }

    #[test]
    fn rfu116_set_syscall_matches() {
        syscall_test(ee::s15::f_0042cbc0__generated, rfu116_set_syscall, 121007);
    }

    #[test]
    fn init_exec_ps2_matches() {
        check(
            ee::s15::f_0042cbd0__generated,
            init_exec_ps2,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 121006, 1);
                c.pc = 0x0042_cbd0;
            },
        );
    }

    #[test]
    fn init_exec_ps2_events() {
        check_events(
            ee::s15::f_0042cbd0__generated,
            init_exec_ps2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121006, 0);
                schedule(c, &mut r, seed, 0x0042_cbd0)
            },
        );
    }

    #[test]
    fn rfu086_wait_evnet_flag_matches() {
        syscall_test(
            ee::s15::f_0042ccb8__generated,
            rfu086_wait_evnet_flag,
            121005,
        );
    }

    #[test]
    fn set_tlb_entry_matches() {
        check(
            ee::s15::f_0042ccc8__generated,
            set_tlb_entry,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121004, 0);
                c.pc = 0x0042_ccc8;
                // Steer the index in and out of range on alternating seeds.
                c.g[A0][0] = if seed % 9 == 8 {
                    // Boundary index: (0x30-0xd)==0x23, exercises the == edge of the <0x23 range check.
                    0x30
                } else if seed % 2 == 0 {
                    13 + (r.next_u32() % 35) as u64
                } else {
                    0x100 + (r.next_u32() % 0x100) as u64
                };
            },
        );
    }

    #[test]
    fn libkernl_setup_tlb_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 121002, 1);
            c.pc = 0x0042_cd98;
            setup_tlb_tables(c, &mut r, seed);
            if seed % 11 == 10 {
                // All-zero config: every list empty so T9 stays 0 into the
                // final table, exercising the S0<48 enter arm.
                for w in 0..16u32 {
                    c.bus.write32(0x0045_67f0 + 4 * w, 0);
                }
            } else if seed % 13 == 12 {
                // Count-48 config: walk exactly 48 entries so T9==48 at the
                // final pre-check (the `< 49` bound sees exact equality),
                // with zeroed per-list bounds so lists 2+ don't advance T9
                // further.
                c.bus.write32(0x0045_67f0, 48);
                c.bus.write32(0x0045_67f4, 0);
                c.bus.write32(0x0045_67f8, 0);
            } else if seed % 7 == 6 {
                // Count-0 config: S1==0 with the other bounds intact, so the
                // first-list (T9<S1) boundary check sees exact equality.
                c.bus.write32(0x0045_67f0, 0);
            }
        };
        check(
            ee::s15::f_0042cd98__generated,
            libkernl_setup_tlb,
            SEEDS,
            setup,
        );
    }

    #[test]
    fn libkernl_setup_tlb_events() {
        check_events(
            ee::s15::f_0042cd98__generated,
            libkernl_setup_tlb,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 121002, 0);
                c.pc = 0x0042_cd98;
                // Drive the table through all four entry-list walks.
                setup_tlb_tables(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0042_cd98)
            },
        );
    }

    /// Syscall hook modelling real kernel register discipline for the
    /// counter-driven walks below: the default fake clobbers every
    /// caller-saved register (including the T9 loop counter), so exact
    /// loop bounds never bind in-harness although they do in-game (the
    /// kernel preserves everything but V0/V1 across a syscall). Preserve
    /// AT/T0-T9, keep the fake's V0/V1/A-reg/cycle/unwind behaviour.
    fn syscall_preserve_t(c: &mut Cpu, at: u32) {
        let t = [
            c.g[1][0], c.g[8][0], c.g[9][0], c.g[10][0], c.g[11][0], c.g[12][0], c.g[13][0],
            c.g[14][0], c.g[15][0], c.g[24][0], c.g[25][0],
        ];
        crate::difftest::fakes::fake_syscall(c, at);
        c.g[1][0] = t[0];
        for (i, r) in [8usize, 9, 10, 11, 12, 13, 14, 15, 24, 25]
            .iter()
            .enumerate()
        {
            c.g[*r][0] = t[i + 1];
        }
    }

    /// Seed-classed walks with preserved T-regs: every entry-list loop
    /// completes, so the exact-boundary mutants bind. Classes (seed%8):
    /// 40/46 with default bounds (3,2) kill the loop-check rows (1592,
    /// 1644, 1667-==, 1702) and the S1==49 second bound (1607);
    /// count 49 + V0_2=0 makes T9==S1==49 in the overflow arm (1627);
    /// count 46 + V0_2=1 makes the third-list S1==49 (1665);
    /// count 46 + V0_2=1 + V0_3=1 lands S0==48 into the final table (1717).
    #[test]
    fn libkernl_setup_tlb_exact_bounds_matches() {
        check(
            ee::s15::f_0042cd98__generated,
            libkernl_setup_tlb,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 121002, 0);
                c.hooks.syscall = syscall_preserve_t;
                c.pc = 0x0042_cd98;
                setup_tlb_tables(c, &mut r, seed);
                let (count, v2, v3) = match seed % 8 {
                    2 => (49, 0, 2),
                    3 => (46, 1, 2),
                    4 | 7 => (46, 1, 1),
                    _ => (if seed % 2 == 0 { 40 } else { 46 }, 3, 2),
                };
                c.bus.write32(0x0045_67f0, count);
                c.bus.write32(0x0045_67f4, v2);
                c.bus.write32(0x0045_67f8, v3);
                let _ = r;
            },
        );
    }

    #[test]
    fn init_sys_matches() {
        let setup = &|c: &mut Cpu, seed: u32| {
            use crate::difftest::fakes::LOG_RESULTS;
            machine(c, seed, 121216, 1);
            c.pc = 0x0042_c300;
            // Bound the nested `InitSyscallTable` walk (its `FIND`s are
            // the 5th/6th syscalls: two `CreateSema`s + two setups first).
            put_words(c, LOG_RESULTS + 16, &[0x1000, 0xf5c]);
        };
        check(ee::s15::f_0042c300__generated, init_sys, SEEDS, setup);
        require_paths(
            ee::s15::f_0042c300__generated,
            SEEDS,
            // Bounded walk termination: full path (862, 864 config
            // variants), short unwind/config paths (40-47).
            &[40, 41, 42, 43, 44, 45, 46, 47, 862, 864],
            setup,
        );
    }

    #[test]
    fn rcmp_av_ms_timer_update_matches() {
        check_lohi(
            ee::s15::f_0042e508__generated,
            rcmp_av_ms_timer_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121001, 1);
                c.pc = 0x0042_e508;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                let base = c.g[A0][0] as u32;
                c.bus.write32(base, DATA + 0x204);
                c.bus.write32(base + 8, (r.next_u32() & 0xffffff) as u32);
                // Third word value (5) takes the use_mult arm with a nonzero
                // product, so the 64-bit combine and `>> 12` shift run on
                // nonzero values (0x1000/0 seeds take the skip arm).
                c.bus.write32(
                    base + 0x10,
                    if seed % 3 == 2 {
                        5
                    } else if seed % 2 == 0 {
                        0x1000
                    } else {
                        0x0
                    },
                );
            },
        );
    }

    #[test]
    fn rfu116_set_syscall_3_matches() {
        syscall_test(ee::s15::f_0042c500__generated, rfu116_set_syscall_3, 121209);
    }
    use crate::difftest::check_paths;

    /// `libkernl_ResetTtyInit`: the single store of 0 to the TTY-init flag.
    #[test]
    fn libkernl_reset_tty_init_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_321_008, 0);
            c.pc = 0x0042_4220;
            c.bus
                .write32(0x0045_5168, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_paths(
            ee::s15::f_00424220__generated,
            libkernl_reset_tty_init,
            SEEDS,
            &[3],
            &setup,
        );
    }

    /// `libkernl_TtyRingInit`: the ring header points at the TTY buffer.
    #[test]
    fn libkernl_tty_ring_init_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_005, 0);
            c.pc = 0x0042_4e00;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus
                .write32(0x0052_b7c0, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_paths(
            ee::s15::f_00424e00__generated,
            libkernl_tty_ring_init,
            SEEDS,
            &[9],
            &setup,
        );
    }

    /// `libkernl_TtyRingPush`/`Pop`: plain advance, and the exact wrap
    /// value (the wrap threshold is `ring + base + 0x10`, so only that
    /// tail/head value wraps: the normal end-of-buffer case, since head
    /// and tail are absolute pointers starting at `ring + 0x10`).
    #[test]
    fn libkernl_tty_ring_push_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_006, 0);
            c.pc = 0x0042_4e28;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus.write32(DATA + 0x104, seed);
            // Tail digit: mid-buffer (advance) or exactly the wrap
            // threshold minus one (wrap).
            let wrap = (DATA + 0x100 + DATA + 0x200 + 0x10).wrapping_sub(1);
            c.bus
                .write32(DATA + 0x10c, [DATA + 0x205, wrap][seed as usize % 2]);
            for i in 0..16 {
                c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
            }
        };
        check_paths(
            ee::s15::f_00424e28__generated,
            libkernl_tty_ring_push,
            SEEDS,
            &[13, 15],
            &setup,
        );
    }

    #[test]
    fn libkernl_tty_ring_pop_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_007, 0);
            c.pc = 0x0042_4e68;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus.write32(DATA + 0x104, seed);
            // Head digit: mid-buffer (advance) or exactly the wrap
            // threshold minus one (wrap).
            let wrap = (DATA + 0x100 + DATA + 0x200 + 0x10).wrapping_sub(1);
            c.bus
                .write32(DATA + 0x108, [DATA + 0x205, wrap][seed as usize % 2]);
            for i in 0..16 {
                c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
            }
        };
        check_paths(
            ee::s15::f_00424e68__generated,
            libkernl_tty_ring_pop,
            SEEDS,
            &[13, 15],
            &setup,
        );
    }

    /// `libkernl_SioPutc`: the idle port sends at once. (The poll loop's
    /// back edge is unreachable in the harness: SIO_STAT reads 0, so the
    /// busy arm never runs.)
    #[test]
    fn libkernl_sio_putc_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_008, 0);
            c.pc = 0x0042_5320;
            c.g[A0][0] = (seed & 0x7f) as u64 | 0x20;
        };
        check_paths(
            ee::s15::f_00425320__generated,
            libkernl_sio_putc,
            SEEDS,
            &[14],
            &setup,
        );
    }

    /// `libkernl_SioPutcCRLF`: plain bytes and `\n` (sent as `\r\n`). The
    /// putc underneath touches only MMIO, so no fake ever fires: plain
    /// `check` compares exactly.
    #[test]
    fn libkernl_sio_putc_crlf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_009, 0);
            c.pc = 0x0042_5408;
            // Byte digit: `\n` (two sends) and plain bytes (one send).
            c.g[A0][0] = [0x0a, 0x41, 0x0d, 0x00][seed as usize % 4] as u64;
        };
        check(
            ee::s15::f_00425408__generated,
            libkernl_sio_putc_crlf,
            SEEDS,
            &setup,
        );
    }

    /// `libkernl_printf`: a small integer format and a plain string, with
    /// the varargs spilled to the stack frame like the caller would.
    #[test]
    fn libkernl_printf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_324_001, 2);
            c.pc = 0x0042_5c28;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Format digit: an integer conversion and a plain string.
            let fmt: &[u8] = if seed % 2 == 0 {
                b"v=%d\n\0"
            } else {
                b"hi\n\0"
            };
            for (i, b) in fmt.iter().enumerate() {
                c.bus.write8(DATA + 0x100 + i as u32, *b);
            }
            c.g[A1][0] = (seed.wrapping_mul(0x9e37_79b9) & 0xffff) as u64;
            c.g[A2][0] = (seed.wrapping_mul(0x5bd1_e995) | 0x0101_0100) as u64;
            c.g[A3][0] = (seed.wrapping_mul(0x1b87_3b09) | 0x0101_0100) as u64;
            c.g[T0][0] = 0x0101_0100;
            c.g[T1][0] = 0x0101_0101;
            c.g[T2][0] = 0x0101_0102;
            c.g[T3][0] = 0x0101_0103;
        };
        covered(
            ee::s15::f_00425c28__generated,
            libkernl_printf,
            SEEDS,
            &setup,
        );
    }
    use crate::difftest::compare;
    use crate::difftest::fakes;

    #[test]
    fn wakeup_thread_matches() {
        syscall_test(ee::s15::f_00423cd0__generated, wakeup_thread, 1_360_010);
    }

    #[test]
    fn suspend_thread_matches() {
        syscall_test(ee::s15::f_00423d10__generated, suspend_thread, 1_360_011);
    }

    pub(crate) const GATE_MAGIC: u32 = 0x4d42_4e44;

    /// Gate + forced result for the `sceDeci2ExRecv` fake below.
    pub(crate) const RECV_GATE: u32 = DATA + 0x6600;

    pub(crate) const RECV_RESULT: u32 = DATA + 0x6604;

    /// `sceDeci2ExRecv` with the gate set: report the forced byte count
    /// without touching hardware. Anywhere else (or with the gate
    /// clear): the slot body.
    pub(crate) fn fake_ex_recv(c: &mut Cpu) {
        if c.bus.read32(RECV_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_ex_recv),
                ee::s15::f_0042c968__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(RECV_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_ex_recv() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C968),
            fake_ex_recv,
            fakes::prev_cell(fake_ex_recv),
        );
    }

    /// Gate + forced result for the `sceDeci2ExSend` fake below.
    pub(crate) const SEND_GATE: u32 = DATA + 0x6610;

    pub(crate) const SEND_RESULT: u32 = DATA + 0x6614;

    /// `sceDeci2ExSend` with the gate set: report the forced byte count
    /// without touching hardware. Anywhere else (or with the gate
    /// clear): the slot body.
    pub(crate) fn fake_ex_send(c: &mut Cpu) {
        if c.bus.read32(SEND_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_ex_send),
                ee::s15::f_0042c9a0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(SEND_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_ex_send() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C9A0),
            fake_ex_send,
            fakes::prev_cell(fake_ex_send),
        );
    }

    /// Gate + forced result for the `sceDeci2ReqSend` fake below.
    pub(crate) const REQ_GATE: u32 = DATA + 0x6620;

    pub(crate) const REQ_RESULT: u32 = DATA + 0x6624;

    /// `sceDeci2ReqSend` with the gate set: report the forced result
    /// without touching hardware. Anywhere else (or with the gate
    /// clear): the slot body.
    pub(crate) fn fake_req_send(c: &mut Cpu) {
        if c.bus.read32(REQ_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_req_send),
                ee::s15::f_0042c910__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(REQ_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_req_send() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C910),
            fake_req_send,
            fakes::prev_cell(fake_req_send),
        );
    }

    /// Gate for the `sceDeci2Poll` fake below.
    pub(crate) const POLL_GATE: u32 = DATA + 0x6628;

    /// `sceDeci2Poll` with the gate set: report completion by clearing
    /// the packet flag word (`S0 + 0xc`, the packet base is in `S0` at
    /// the call). Anywhere else (or with the gate clear): the slot body.
    pub(crate) fn fake_poll(c: &mut Cpu) {
        if c.bus.read32(POLL_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_poll),
                ee::s15::f_0042c940__generated,
            );
        }
        c.bus.write32((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_poll() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C940),
            fake_poll,
            fakes::prev_cell(fake_poll),
        );
    }

    /// Gate + forced result for the `sceDeci2Open` fake below.
    pub(crate) const OPEN_GATE: u32 = DATA + 0x6630;

    pub(crate) const OPEN_RESULT: u32 = DATA + 0x6634;

    /// `sceDeci2Open` with the gate set: report the forced result
    /// (sign-extended, so -1 steers the failure arm) without touching
    /// hardware. Anywhere else (or with the gate clear): the slot body.
    pub(crate) fn fake_open(c: &mut Cpu) {
        if c.bus.read32(OPEN_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_open),
                ee::s15::f_0042c8a0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(OPEN_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_open() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042C8A0),
            fake_open,
            fakes::prev_cell(fake_open),
        );
    }

    /// Gate for the `Deci2Kputs` fake below.
    pub(crate) const KPUTS_GATE: u32 = DATA + 0x6638;

    /// `Deci2Kputs` with the gate set: swallow the string without touching
    /// hardware. Anywhere else (or with the gate clear): the slot body.
    pub(crate) fn fake_kputs(c: &mut Cpu) {
        if c.bus.read32(KPUTS_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_kputs),
                ee::s15::f_0042ca58__generated,
            );
        }
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_kputs() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0042CA58),
            fake_kputs,
            fakes::prev_cell(fake_kputs),
        );
    }

    /// Plausible tty state at `DATA + 0x1000`: buffer pointers and small
    /// positions; the count word steers the send path.
    pub(crate) fn tty_state(c: &mut Cpu, count: u32) {
        c.bus.write32(DATA + 0x1000, (DATA + 0x2000) as u32);
        c.bus.write32(DATA + 0x1004, count);
        c.bus.write32(DATA + 0x1008, 0);
        c.bus.write32(DATA + 0x1010, 0);
        c.bus.write32(DATA + 0x1014, (DATA + 0x3000) as u32);
        c.bus.write32(DATA + 0x1018, (DATA + 0x4000) as u32);
        c.bus.write32(DATA + 0x100c, 0x0101_0100);
    }

    /// `libkernl_TtyDeci2Handler` over the mode digit: the Deci2 send
    /// path (3), the ring arm (1, with an empty ring: the push loop runs
    /// in the unwind and events tests), the clear-busy arm (4) and the
    /// early exits (5, 0); mode 2 is not seeded. The faked receive/send
    /// report success or failure.
    #[test]
    fn deci2_handler_matches() {
        hook_ex_recv();
        hook_ex_send();
        check(
            ee::s15::f_00424ea8__generated,
            libkernl_tty_deci2_handler,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_001, 1);
                c.pc = 0x0042_4ea8;
                // Mode digit: send path (3), ring arm (1), clear-busy
                // arm (4), early exits (5, 0).
                c.g[A0][0] = [3, 1, 4, 5, 0][seed as usize % 5] as u64;
                c.g[A1][0] = [0, 8][seed as usize / 5 % 2] as u64;
                c.g[A2][0] = (DATA + 0x1000) as u64;
                tty_state(c, [0, 0x20][seed as usize / 10 % 2]);
                // Boundary arms the sweep left uncovered (mode 1 only,
                // so the other modes' paths do not move): S0=8 with the
                // state word at 0x139 sums to exactly 0x141, taking the
                // 0x424f0c PRINTF arm (`<0x141` becomes `<=`) instead of
                // 0x424f14; S0=0 with the ring count at 12 takes the
                // 0x424f6c side of the `12<V0` check instead of 0x424fa8
                // (same cycles either way; the mutant takes the other
                // side in both cases).
                if seed as usize % 5 == 1 && seed as usize / 5 % 2 == 1 {
                    c.bus.write32(DATA + 0x1008, 0x139);
                }
                if seed as usize % 5 == 1 && seed as usize / 5 % 2 == 0 {
                    c.bus.write16(DATA + 0x3000, 12);
                }
                c.bus.write32(RECV_GATE, GATE_MAGIC);
                c.bus.write32(SEND_GATE, GATE_MAGIC);
                c.bus
                    .write32(RECV_RESULT, [4, 0xffff_ffff][seed as usize / 20 % 2]);
                c.bus
                    .write32(SEND_RESULT, [4, 0xffff_ffff][seed as usize / 40 % 2]);
            },
        );
    }

    crate::difftest::events::driven!(deci2_driven, libkernl_tty_deci2_handler);
    crate::difftest::events::driven!(deci2_generated_driven, ee::s15::f_00424ea8__generated);

    /// `libkernl_TtyDeci2Handler` with events due in the ring-push loop
    /// (the receive ring holds 0x80 bytes): an unwinding event at the back
    /// edge returns at `0x424f78`, and `drive` re-enters there through the
    /// entry guard; a delivered one continues inline.
    #[test]
    fn deci2_handler_events() {
        hook_ex_recv();
        hook_ex_send();
        check_events(deci2_generated_driven, deci2_driven, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 1_366_017, 0);
            machine(c, seed, 1_366_001, 1);
            c.pc = 0x0042_4ea8;
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[A2][0] = (DATA + 0x1000) as u64;
            tty_state(c, 0);
            // The receive ring holds 0x80 distinct bytes and the push
            // target's write pointer is real RAM, so each push lands.
            deci2_ring(c, 0x80);
            c.bus.write32(RECV_GATE, GATE_MAGIC);
            c.bus.write32(SEND_GATE, GATE_MAGIC);
            c.bus.write32(RECV_RESULT, 4);
            c.bus.write32(SEND_RESULT, 4);
            schedule(c, &mut r, seed, 0x0042_4ea8)
        });
    }

    /// The ring-path state the push loop reads: the receive ring at
    /// `DATA + 0x3000` holding `count` bytes, and the push target at
    /// `DATA + 0x4000` whose write pointer (`+0xc`) is real RAM.
    pub(crate) fn deci2_ring(c: &mut Cpu, count: u16) {
        for i in 0..0x100u32 {
            c.bus.write8(DATA + 0x3000 + i, 0x30 + (i % 0x40) as u8);
        }
        c.bus.write16(DATA + 0x3000, count);
        c.bus.write32(DATA + 0x400c, DATA + 0x5000);
    }

    /// `libkernl_TtyDeci2Handler` called once (not driven) with a VBlank
    /// due in the push loop: the back edge at `0x424f78` unwinds, and the
    /// handler returns there with the unwind bit set instead of finishing.
    #[test]
    fn deci2_handler_unwind_matches() {
        hook_ex_recv();
        hook_ex_send();
        check(
            ee::s15::f_00424ea8__generated,
            libkernl_tty_deci2_handler,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_366_031, 0);
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = (DATA + 0x1000) as u64;
                tty_state(c, 0);
                deci2_ring(c, 0x80);
                c.bus.write32(RECV_GATE, GATE_MAGIC);
                c.bus.write32(SEND_GATE, GATE_MAGIC);
                // Every seed in the unwinding mode (`schedule` picks it
                // for `seed % 3 == 1`).
                schedule(c, &mut r, seed * 3 + 1, 0x0042_4ea8);
            },
        );
    }

    /// `libkernl_TtyDeci2Handler` re-entered at its loop head `0x424f78`
    /// (where the back-edge unwind resumes), mid-loop: frame saved, ring
    /// pointer in `s2`/`v1`, index in `s0` below the ring count.
    #[test]
    fn deci2_handler_resume_matches() {
        check(
            ee::s15::f_00424ea8__generated,
            libkernl_tty_deci2_handler,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_032, 0);
                tty_state(c, 0);
                deci2_ring(c, 0x20 + (seed % 4) as u16);
                let sp = crate::difftest::SP - 0x40;
                c.g[SP][0] = sp as u64;
                c.bus.write64(sp, 0x5050_5050_0000_0000 + seed as u64);
                c.bus
                    .write64(sp + 0x10, 0x5151_5151_0000_0000 + seed as u64);
                c.bus
                    .write64(sp + 0x20, 0x5252_5252_0000_0000 + seed as u64);
                c.bus.write64(sp + 0x30, crate::difftest::RA as u64);
                c.g[S1][0] = (DATA + 0x1000) as u64;
                c.g[S2][0] = (DATA + 0x3000) as u64;
                c.g[V1][0] = (DATA + 0x3000) as u64;
                c.g[S0][0] = (0xd + seed % 0x10) as u64;
                c.pc = 0x0042_4f78;
            },
        );
    }

    /// Source bytes at `DATA + 0x2000`: `len` bytes, every 8th a newline
    /// so both the CRLF expansion and the plain store run; distinctive
    /// nonzero content (zeroed RAM would prove nothing).
    pub(crate) fn tty_src(c: &mut Cpu, len: usize) {
        for i in 0..len.min(0x300) {
            let b = if i % 8 == 7 {
                0x0a
            } else {
                0x41 + (i % 26) as u8
            };
            c.bus.write8(DATA + 0x2000 + i as u32, b);
        }
    }

    /// `libkernl_TtyWrite` over the lock word, the count (empty, short,
    /// overflow past 256 with CRLF expansion), newline/plain bytes and
    /// the ReqSend success/failure arms; the poll fake completes the send.
    #[test]
    fn tty_write_matches() {
        hook_req_send();
        hook_poll();
        check(
            ee::s15::f_00425040__generated,
            libkernl_tty_write,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_002, 1);
                c.pc = 0x0042_5040;
                let lock = [0, 1][seed as usize % 2];
                c.bus.write32(0x0052_b8dc, lock);
                c.bus.write32(0x0052_b8e0, 0);
                let count = [0, 3, 0x20, 0x150][seed as usize / 2 % 4];
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = count as u64;
                tty_src(c, 0x300);
                c.bus.write32(REQ_GATE, GATE_MAGIC);
                c.bus.write32(POLL_GATE, GATE_MAGIC);
                c.bus
                    .write32(REQ_RESULT, [0, 0xffff_ffff][seed as usize / 8 % 2]);
            },
        );
    }

    crate::difftest::events::driven!(tty_write_driven, libkernl_tty_write);
    crate::difftest::events::driven!(tty_write_generated_driven, ee::s15::f_00425040__generated);

    /// `libkernl_TtyWrite` with events due: deliveries in the copy
    /// loop deliver inline on both sides.
    #[test]
    fn tty_write_events() {
        hook_req_send();
        hook_poll();
        check_events(
            tty_write_generated_driven,
            tty_write_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_366_015, 0);
                machine(c, seed, 1_366_002, 1);
                c.pc = 0x0042_5040;
                let lock = [0, 1][seed as usize % 2];
                c.bus.write32(0x0052_b8dc, lock);
                c.bus.write32(0x0052_b8e0, 0);
                let count = [0, 3, 0x20, 0x150][seed as usize / 2 % 4];
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = count as u64;
                tty_src(c, 0x300);
                c.bus.write32(REQ_GATE, GATE_MAGIC);
                c.bus.write32(POLL_GATE, GATE_MAGIC);
                c.bus
                    .write32(REQ_RESULT, [0, 0xffff_ffff][seed as usize / 8 % 2]);
                schedule(c, &mut r, seed, 0x0042_5040)
            },
        );
    }

    /// `libkernl_TtyRead` over the count (empty, single, multi-byte) and
    /// the first bytes (newline, CR, plain then newline, plain to
    /// exhaustion); the ring always has data ready (the not-ready spin
    /// waits on async Deci2 input, unreachable in the harness).
    #[test]
    fn tty_read_matches() {
        check(
            ee::s15::f_00425190__generated,
            libkernl_tty_read,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_003, 1);
                c.pc = 0x0042_5190;
                // Ring struct doubles as the Deci2 handle: the state word
                // at `0x52b8e8` names it, its +4 word is the ready count.
                c.bus.write32(0x0052_b8e8, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                c.bus.write32(DATA + 0x3104, 8);
                c.bus.write32(DATA + 0x3108, DATA + 0x3200);
                c.bus.write32(DATA + 0x310c, 0);
                // Byte 0 steers immediate newline / CR / plain; byte 2
                // steers early newline vs. exhaustion; all nonzero.
                let b0 = [0x0a, 0x0d, 0x41][seed as usize % 3];
                let b2 = [0x0a, 0x44][seed as usize / 3 % 2];
                let bytes = [b0, 0x42, b2, 0x43, 0x44, 0x45, 0x46, 0x47];
                for (i, &b) in bytes.iter().enumerate() {
                    c.bus.write8(DATA + 0x3200 + i as u32, b);
                }
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = [0, 1, 4, 6][seed as usize / 6 % 4] as u64;
            },
        );
    }

    crate::difftest::events::driven!(tty_read_driven, libkernl_tty_read);
    crate::difftest::events::driven!(tty_read_generated_driven, ee::s15::f_00425190__generated);

    /// `libkernl_TtyRead` with events due: deliveries in the scan loop
    /// deliver inline on both sides. The empty
    /// read returns before any event check, and short reads can finish
    /// before the first due event, so the events fixture uses long
    /// reads (the arm shapes are covered by the entry test).
    #[test]
    fn tty_read_events() {
        check_events(
            tty_read_generated_driven,
            tty_read_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_366_016, 0);
                machine(c, seed, 1_366_003, 1);
                c.pc = 0x0042_5190;
                c.bus.write32(0x0052_b8e8, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                c.bus.write32(DATA + 0x3104, 0x20);
                c.bus.write32(DATA + 0x3108, DATA + 0x3200);
                c.bus.write32(DATA + 0x310c, 0);
                for i in 0..0x20 {
                    c.bus.write8(DATA + 0x3200 + i, 0x41 + (i % 26) as u8);
                }
                c.bus.write8(DATA + 0x3200 + 0x0a, 0x0a);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = [0x08, 0x10, 0x20, 0x10][seed as usize % 4] as u64;
                schedule(c, &mut r, seed, 0x0042_5190)
            },
        );
    }

    /// `libkernl_TtyInit` over the `sceDeci2Open` success/failure arms;
    /// the faked open reports a channel or -1 without touching hardware.
    /// The state block (`0x52b8d0`) and the packet header (`0x52b900`,
    /// stored through the uncached mirror) start non-zero, so each of the
    /// success arm's zero and header stores changes RAM.
    #[test]
    fn tty_init_matches() {
        hook_open();
        check(
            ee::s15::f_00425260__generated,
            libkernl_tty_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_004, 1);
                c.pc = 0x0042_5260;
                for i in 0..8 {
                    c.bus.write32(0x0052_b8d0 + 4 * i, 0x5a5a_5a00 + i + seed);
                    c.bus.write32(0x0052_b900 + 4 * i, 0x6b6b_6b00 + i + seed);
                }
                c.bus.write32(OPEN_GATE, GATE_MAGIC);
                c.bus
                    .write32(OPEN_RESULT, [0, 0xffff_ffff][seed as usize % 2]);
            },
        );
    }

    /// `libkernl_KputcBuffered` over the buffer count (room, full) and the
    /// char (newline flush vs. plain store); the faked `Deci2Kputs`
    /// swallows flushes without touching hardware.
    #[test]
    fn kputc_buffered_matches() {
        hook_kputs();
        check(
            ee::s15::f_00425358__generated,
            libkernl_kputc_buffered,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_005, 1);
                c.pc = 0x0042_5358;
                c.g[A0][0] = [0x0a, 0x41][seed as usize % 2] as u64;
                c.bus
                    .write32(0x0045_5174, [0, 5, 125, 126, 200][seed as usize / 2 % 5]);
                // Distinctive nonzero buffer content under the store.
                for i in 0..0x80 {
                    c.bus.write8(0x0052_bb80 + i, 0x61 + (i % 26) as u8);
                }
                c.bus.write32(KPUTS_GATE, GATE_MAGIC);
            },
        );
    }

    /// `libkernl_DoubleToInt` over magnitudes: zero, fractions, integers,
    /// large and tiny values (saturating and zero arms).
    #[test]
    fn double_to_int_matches() {
        check(
            ee::s15::f_00425440__generated,
            libkernl_double_to_int,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_366_006, 1);
                c.pc = 0x0042_5440;
                c.g[A0][0] = [
                    0x0000_0000_0000_0000, // 0.0
                    0x3ff0_0000_0000_0000, // 1.0
                    0xc004_0000_0000_0000, // -2.5
                    0x4059_0000_0000_0000, // 100.0
                    0x3ffb_9999_9999_999a, // 1.725
                    0x4047_4567_89ab_cdef, // 46.54
                    0x41f0_0000_0000_0000, // 2^32
                    0x3eb0_0000_0000_0000, // 2^-20
                    0x4320_0000_0000_0000, // 2^51
                    0x4330_0000_0000_0000, // 2^52
                    0x4340_0000_0000_0000, // 2^53
                    0x4530_0000_0000_0000, // 2^84 (saturates)
                    0x3a90_0000_0000_0000, // 2^-86 (zero)
                    0xbeb0_0000_0000_0000, // -2^-20
                    0x400c_0000_0000_0000, // 3.5
                    0x4006_6666_6666_6666, // 2.8
                    // Boundary exponents the sweep left uncovered: exp
                    // 0x3fe (A2==-53; 0.75 computes 1, `<=` returns 0),
                    // exp 0x440 (A2==13; 9999 vs computed 0), and a
                    // nonzero mantissa low with A2==1 (2^53+2: shift
                    // 1 vs 0, ret 2 vs 1/0).
                    0x3fe8_0000_0000_0000, // 0.75
                    0x4400_0000_0000_0000, // 2^65
                    0x4340_0000_0000_0001, // 2^53+2
                ][seed as usize % 19];
            },
        );
    }

    /// Fixture for [`print_double_matches`]: the game's image (the 0.1
    /// thresholds, the 1e6 scale and the three format strings at
    /// `0x49ab80`), the double in `A0`, and the putc pointer aimed at the
    /// real buffered putc (flushes swallowed by the faked `Deci2Kputs`).
    /// Zero is left out: with the real thresholds it never leaves the
    /// multiply loop, in the game too.
    pub(crate) fn print_double_fixture(c: &mut Cpu, seed: u32) {
        machine(c, seed, 1_366_008, 1);
        crate::difftest::elf_image(c);
        c.pc = 0x0042_54d0;
        c.g[A0][0] = [
            0x3ff0_0000_0000_0000, // 1.0: divide loop once, "e+%d"
            0xc004_0000_0000_0000, // -2.5: negation, divide loop
            0x3fa9_9999_9999_999a, // 0.05: multiply loop once, "e%d"
            0x4059_0000_0000_0000, // 100.0: divide loop three times
            0x3eb0_c6f7_a0b5_ed8d, // 1e-6: multiply loop six times
            0x4009_21fb_5444_2d18, // pi
            0x462e_6466_296d_0c95, // 1e30: long divide loop
            0x3fe0_0000_0000_0000, // 0.5: neither loop, "e+%d"
            0xbfa9_9999_9999_999a, // -0.05: negation, multiply loop
            0x3fb9_9999_9999_999a, // 0.1: the entry threshold itself
        ][seed as usize % 10];
        c.bus.write32(0x0045_5178, 0x0042_5358);
        c.bus.write32(0x0045_5174, 0);
        c.bus.write32(KPUTS_GATE, GATE_MAGIC);
    }

    /// `libkernl_PrintDouble` over signs and magnitudes: the negation, the
    /// multiply loop (below 0.1) and the divide loop (1.0 and up), and both
    /// exponent tails ("e%d" after the multiply loop, "e+%d" otherwise).
    /// The mantissa prints as 0 for every input, in the game too
    /// (`libkernl_DoubleToInt` reads the scaled integer as double bits), so
    /// the sign and the exponent are what the printed text shows.
    #[test]
    fn print_double_matches() {
        hook_kputs();
        check(
            ee::s15::f_004254d0__generated,
            libkernl_print_double,
            SEEDS,
            &|c, seed| {
                print_double_fixture(c, seed);
            },
        );
    }

    /// Resume heads of `libkernl_PrintDouble`: every soft-float call
    /// return plus the two normalization loop back edges.
    const PRINT_DOUBLE_RESUME_HEADS: [u32; 15] = [
        0x0042_54f8,
        0x0042_5508,
        0x0042_551c,
        0x0042_552c,
        0x0042_5550,
        0x0042_5564,
        0x0042_5584,
        0x0042_55a8,
        0x0042_55bc,
        0x0042_55d4,
        0x0042_55dc,
        0x0042_55e4,
        0x0042_55f0,
        0x0042_5540,
        0x0042_5598,
    ];

    /// Frame + prologue state for a resume at `head`: the entry
    /// fixture lays down the putc gate, then the prologue is replayed
    /// verbatim. `A0` is forced to a positive magnitude: the
    /// multiply-normalization loop only terminates for non-negative
    /// values (a real resume there always holds the absolute value).
    /// `V0` carries a plausible callee result (`S0` for value
    /// helpers, 0 for the compare helper).
    fn print_double_resume_fixture(c: &mut Cpu, seed: u32) {
        use ssx3_ee_rt::cpu::reg::*;
        use ssx3_ee_rt::cpu::sx;
        print_double_fixture(c, seed);
        let mags = [
            0x3ff0_0000_0000_0000, // 1.0
            0x4059_0000_0000_0000, // 100.0
            0x3eb0_0000_0000_0000, // 2^-20
            0x4009_21fb_5444_2d18, // pi
            0x462e_6466_296d_0c95, // 1e30
            0x3ff3_3333_3333_3333, // 1.2
            0x40c8_0000_0000_0000, // 12345.0
        ];
        c.g[A0][0] = mags[seed as usize % mags.len()];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        c.g[S2][0] = 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S1][0] = 0u64;
        let head = PRINT_DOUBLE_RESUME_HEADS[seed as usize % PRINT_DOUBLE_RESUME_HEADS.len()];
        // Compare-helper returns: 0 takes the not-less arm; value
        // helpers return their result into the working value.
        c.g[V0][0] = match head {
            0x0042_54f8 | 0x0042_5508 | 0x0042_552c | 0x0042_5564 | 0x0042_5584 | 0x0042_55bc => 0,
            _ => c.g[S0][0],
        };
        c.g[A0][0] = c.g[S0][0];
        c.pc = head;
    }

    /// Direct-entry resume test for `libkernl_PrintDouble`.
    #[test]
    fn print_double_resume_matches() {
        use crate::difftest::{compare, strict_ok};
        use std::collections::BTreeSet;
        hook_kputs();
        let mut seen = BTreeSet::new();
        for seed in 0..SEEDS {
            let head = PRINT_DOUBLE_RESUME_HEADS[seed as usize % PRINT_DOUBLE_RESUME_HEADS.len()];
            seen.insert(head);
            let r = compare(
                ee::s15::f_004254d0__generated,
                libkernl_print_double,
                &|c| print_double_resume_fixture(c, seed),
            );
            assert!(
                r.hard.is_empty(),
                "seed {seed} head {head:#x}: {:#?}",
                r.hard
            );
            assert!(
                strict_ok(&r.soft),
                "seed {seed} head {head:#x} (scratch): {:#?}",
                r.soft
            );
        }
        assert!(
            seen.len() == PRINT_DOUBLE_RESUME_HEADS.len(),
            "resume heads not covered: {seen:?}",
        );
    }

    crate::difftest::events::driven!(print_double_driven, libkernl_print_double);
    crate::difftest::events::driven!(
        print_double_generated_driven,
        ee::s15::f_004254d0__generated
    );

    /// Gate for the `libkernl_printf` fake below.
    pub(crate) const PRINTF_GATE: u32 = DATA + 0x66b0;

    /// `libkernl_printf` with the gate set: print nothing. Anywhere else (or
    /// with the gate clear): the slot body.
    pub(crate) fn fake_printf(c: &mut Cpu) {
        if c.bus.read32(PRINTF_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_printf),
                ee::s15::f_00425c28__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_printf() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00425C28),
            fake_printf,
            fakes::prev_cell(fake_printf),
        );
    }

    /// `libkernl_PrintDouble` with events due in its two loops (heads
    /// `0x425540`, multiply, and `0x425598`, divide): unwinds re-enter
    /// through the entry guard identically. The printf calls are faked:
    /// an unwind inside the real `vprintf` would leave a pc `drive` cannot
    /// re-enter through this function (the entry test runs them for real).
    #[test]
    fn print_double_events() {
        hook_kputs();
        hook_printf();
        check_events(
            print_double_generated_driven,
            print_double_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_366_013, 0);
                print_double_fixture(c, seed);
                c.bus.write32(PRINTF_GATE, GATE_MAGIC);
                schedule(c, &mut r, seed, 0x0042_54d0)
            },
        );
    }

    /// The vprintf rows: a format and its 8-byte vararg slots. The game's
    /// spec table (`0x49abb0`, indexed from '0') has arms for `0 c d e f h
    /// l o s u x` only; every other character in '1'..'x' takes the skip
    /// arm 0x425bd8, and anything outside it (`%`, `-`, `+`, space, `y`)
    /// the bound-fail arm 0x425bd0.
    const VPRINTF_ROWS: [(&str, &[u64]); 13] = [
        // Zero (0x425918), negative (0x425930) and positive `%d`.
        ("%d %d %d", &[0, 0xffff_ffd6, 1234]),
        // Zero and nonzero `%o %x %u` (0x42579c, 0x425858, 0x425a08), hex
        // digits a-f (0x425880), an unsigned value past `i32::MAX`.
        ("%o %o %x %x %u %u", &[0, 0x29c, 0, 0x1a2b, 0, 0xee6b_2800]),
        // `l` and `h` on every conversion (64-bit and 16-bit reads).
        (
            "%lo %ho %lx %hx %lu %hu %ld %hd",
            &[
                0x0001_0000_0000_0007,
                0xffff_0000_0001_0009,
                0x0000_0001_0000_00ab,
                0x1234_5678_0000_abcd,
                0x0000_0001_0000_0000,
                0xffff_0000_0000_fffe,
                0xffff_ffff_ffff_fffb,
                0x0000_0000_0000_ff9c,
            ],
        ),
        // Zero-padded widths on each conversion; `%00d` (width 0) takes
        // the back edge to 0x425690, `%050d` the clamp to 31.
        (
            "[%05d][%05x][%05o][%05u][%00d][%012d][%050d][%0d]",
            &[0xffff_ffd6, 0x1f, 8, 7, 5, 123, 9, 6],
        ),
        // `%c`, a string and an empty string ("(null)").
        (
            "%c%s|%s",
            &[0x41, (DATA + 0x6100) as u64, (DATA + 0x6200) as u64],
        ),
        // `%f`/`%e` read the slot's low word as a float: nonzero ones run
        // `PrintDouble` (0x425ad4), 0.0 prints '0' (0x425abc).
        ("%f %e %f %f", &[0x3f80_0000, 0x3d4c_cccd, 0, 0xc020_0000]),
        // Skip arms: '%', '-', 'y' fail the bound (0x425bd0); '5', 'E',
        // 'i' have no arm (0x425bd8). No slot is read.
        ("%%|%5d|%-d|%y|%E|%i.", &[]),
        // Empty format.
        ("", &[]),
        // A newline flushes the buffered putc through `Deci2Kputs`.
        ("a=%d\nb", &[7]),
        // Literal text only.
        ("lit-only-no-specs-at-all-0123456789", &[]),
        // Boundary formats the sweep left uncovered (appended so seeds
        // 0-9 keep their rows): `%0:` feeds ':' (0x3a) as the first width
        // byte, taking the `<=0xa` side of the first digit check; `%05:`
        // feeds a digit then ':' for the second; `%032d` computes width
        // 32, taking the unclamped side of the `<32` check (31 vs 32).
        ("%0:d", &[1]),
        ("%05:d", &[2]),
        ("%032d", &[3]),
    ];

    /// Shared vprintf setup: the game's image (spec table, `__clz_tab`,
    /// float constants), row `seed % 10` with its format at `DATA + 0x500`
    /// and slots at `DATA + 0x6000`, the strings at `DATA + 0x6100`, the
    /// putc pointer on the real buffered putc, and the interrupt-enable
    /// bit (the exit's `EIntr` arm 0x425bf4) from the next seed digit.
    fn vprintf_setup(c: &mut Cpu, seed: u32) {
        crate::difftest::elf_image(c);
        c.pc = 0x0042_5638;
        let (fmt, slots) = VPRINTF_ROWS[seed as usize % VPRINTF_ROWS.len()];
        for (i, &b) in fmt.as_bytes().iter().enumerate() {
            c.bus.write8(DATA + 0x500 + i as u32, b);
        }
        c.bus.write8(DATA + 0x500 + fmt.len() as u32, 0);
        for (i, &v) in slots.iter().enumerate() {
            c.bus.write64(DATA + 0x6000 + 8 * i as u32, v);
        }
        for (i, &b) in b"hi\0".iter().enumerate() {
            c.bus.write8(DATA + 0x6100 + i as u32, b);
        }
        c.bus.write32(DATA + 0x6200, 0);
        eie(c, seed / VPRINTF_ROWS.len() as u32);
        c.g[A0][0] = (DATA + 0x500) as u64;
        c.g[A1][0] = (DATA + 0x6000) as u64;
        c.bus.write32(0x0045_5178, 0x0042_5358);
        c.bus.write32(0x0045_5174, 0);
        c.bus.write32(KPUTS_GATE, GATE_MAGIC);
    }

    /// Fixture for [`vprintf_matches`]: [`vprintf_setup`] on a machine
    /// whose first fake call may unwind.
    pub(crate) fn vprintf_fixture(c: &mut Cpu, seed: u32) {
        machine(c, seed, 1_366_009, 1);
        vprintf_setup(c, seed);
    }

    /// `libkernl_vprintf` over the rows of [`VPRINTF_ROWS`]: every
    /// conversion arm of the game's spec table with zero, negative and
    /// wide values, the `l`/`h` lengths, zero-padded widths, both skip
    /// arms and the newline flush; the real float helpers and
    /// `PrintDouble` run, and flushes go through the faked `Deci2Kputs`.
    #[test]
    fn vprintf_matches() {
        hook_kputs();
        check(
            ee::s15::f_00425638__generated,
            libkernl_vprintf,
            SEEDS,
            &|c, seed| {
                vprintf_fixture(c, seed);
            },
        );
    }

    /// Resume heads of `libkernl_vprintf`: the DIntr return, every
    /// putc-table call return, and every loop back edge.
    const VPRINTF_RESUME_HEADS: [u32; 38] = [
        0x0042_5668,
        0x0042_580c,
        0x0042_58cc,
        0x0042_5940,
        0x0042_5958,
        0x0042_5974,
        0x0042_59bc,
        0x0042_5a30,
        0x0042_5a4c,
        0x0042_5a8c,
        0x0042_5acc,
        0x0042_5adc,
        0x0042_5ae4,
        0x0042_5b14,
        0x0042_5b20,
        0x0042_5b2c,
        0x0042_5b38,
        0x0042_5b44,
        0x0042_5b50,
        0x0042_5b7c,
        0x0042_5bb0,
        0x0042_5bc8,
        0x0042_5bfc,
        0x0042_5678,
        0x0042_5690,
        0x0042_5694,
        0x0042_5698,
        0x0042_5728,
        0x0042_5754,
        0x0042_57b8,
        0x0042_57f8,
        0x0042_5870,
        0x0042_58b8,
        0x0042_5950,
        0x0042_59a8,
        0x0042_5a28,
        0x0042_5a78,
        0x0042_5b68,
    ];

    /// Frame + prologue state for a resume at `head`: the entry
    /// fixture lays down the format, varargs and putc gate, then the
    /// prologue is replayed verbatim. Counters start small (`A1`,
    /// accumulators zeroed) so every loop back edge terminates
    /// quickly; pointers stay on the fixture words. `V0` needs no
    /// preset: every resume path reloads it before branching on it.
    fn vprintf_resume_fixture(c: &mut Cpu, seed: u32) {
        use ssx3_ee_rt::cpu::reg::*;
        use ssx3_ee_rt::cpu::sx;
        vprintf_fixture(c, seed);
        // The digit loops call the generated `__moddi3`/`__divdi3`,
        // which normalize through the game's `__clz_tab` at
        // `0x4a0000 - 0x51e8` (test RAM starts zeroed): without it the
        // quotients are garbage and the loop never converges.
        crate::game::lib::gcc::tests::clz_table(c, 0x0049_ae18);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.g[A0][0] = (DATA + 0x500) as u64;
        c.g[A1][0] = 3;
        c.g[V0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = 0;
        c.g[S4][0] = 0;
        c.g[S5][0] = 0x450000;
        c.g[S6][0] = 0;
        c.g[S7][0] = 0;
        let head = VPRINTF_RESUME_HEADS[seed as usize % VPRINTF_RESUME_HEADS.len()];
        match head {
            // Digit-emission loops read the value from `A0` first (the
            // previous arm moves `S1` there) and build the digits at
            // `S0`: a real unwind leaves `A0 == S1 == value` with `S0`
            // in the digit buffer, not the format pointer. Stage a
            // scratch buffer with digits and a NUL like the stack
            // buffer would hold, so the resume runs the loop and the
            // emit tail to completion.
            0x0042_5950 | 0x0042_5a28 => {
                // A real unwind leaves the value in `A0`/`S1` and the
                // stack digit buffer in `S0` (NUL at `SP + 0x1f`, digits
                // built downward); the conversion tail then runs to
                // completion and the outer scan exits at its NUL.
                c.g[A0][0] = 0x1234;
                c.g[S1][0] = 0x1234;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f));
                c.bus.write8((c.g[SP][0] as u32).wrapping_add(0x1f), 0);
            }
            _ => {}
        }
        c.pc = head;
    }

    /// Direct-entry resume test for `libkernl_vprintf`.
    #[test]
    fn vprintf_resume_matches() {
        use crate::difftest::{compare, strict_ok};
        use std::collections::BTreeSet;
        hook_kputs();
        let mut seen = BTreeSet::new();
        for seed in 0..SEEDS {
            let head = VPRINTF_RESUME_HEADS[seed as usize % VPRINTF_RESUME_HEADS.len()];
            seen.insert(head);
            let r = compare(ee::s15::f_00425638__generated, libkernl_vprintf, &|c| {
                vprintf_resume_fixture(c, seed)
            });
            assert!(
                r.hard.is_empty(),
                "seed {seed} head {head:#x}: {:#?}",
                r.hard
            );
            assert!(
                strict_ok(&r.soft),
                "seed {seed} head {head:#x} (scratch): {:#?}",
                r.soft
            );
        }
        assert!(
            seen.len() == VPRINTF_RESUME_HEADS.len(),
            "resume heads not covered: {seen:?}",
        );
    }

    crate::difftest::events::driven!(vprintf_driven, libkernl_vprintf);
    crate::difftest::events::driven!(vprintf_generated_driven, ee::s15::f_00425638__generated);

    /// Events-test fixture: [`vprintf_setup`] with no fake-unwind lottery
    /// (`drive` requires the UNWIND bit, which the lottery never sets), and
    /// the `%f` row swapped for the first: an unwind inside the nested
    /// `PrintDouble` would leave a pc `drive` cannot re-enter through
    /// `vprintf` (the entry test runs that row).
    pub(crate) fn vprintf_events_fixture(c: &mut Cpu, seed: u32) {
        machine(c, seed, 1_366_014, 0);
        let row = seed as usize % VPRINTF_ROWS.len();
        vprintf_setup(c, if row == 5 { seed - 5 } else { seed });
    }

    /// `libkernl_vprintf` with events due: deliveries in the format
    /// scan and conversion loops unwind and re-enter identically.
    #[test]
    fn vprintf_events() {
        hook_kputs();
        check_events(
            vprintf_generated_driven,
            vprintf_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1_366_014, 0);
                vprintf_events_fixture(c, seed);
                schedule(c, &mut r, seed, 0x0042_5638)
            },
        );
    }

    /// Status bit 16 (EIE) set or clear, keeping the other scrambled bits.
    pub(crate) fn eie(c: &mut Cpu, seed: u32) {
        if seed % 2 == 0 {
            c.cop0[12] |= 0x1_0000;
        } else {
            c.cop0[12] &= !0x1_0000;
        }
    }

    /// Set the VSYNC bit in I_STAT so the VBlank polls exit at once.
    pub(crate) fn vsync(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.i_stat |= 4;
    }

    /// `libkernl_WaitVBlankStart` with interrupts enabled/disabled on entry;
    /// the VSYNC bit is set so the poll exits (the spin itself waits on
    /// hardware, unreachable in the harness).
    #[test]
    fn wait_vblank_start_matches() {
        check(
            ee::s15::f_00424230__generated,
            libkernl_wait_vblank_start,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_001, 1);
                c.pc = 0x0042_4230;
                eie(c, seed);
                vsync(c);
            },
        );
    }

    /// `libkernl_WaitVBlankOrFlag` with interrupts enabled/disabled; the
    /// VSYNC bit is set so the poll exits via hardware (the stack-flag arm
    /// waits on an async callback, unreachable in the harness).
    #[test]
    fn wait_vblank_or_flag_matches() {
        check(
            ee::s15::f_004242c0__generated,
            libkernl_wait_vblank_or_flag,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_002, 1);
                c.pc = 0x0042_42c0;
                eie(c, seed);
                vsync(c);
            },
        );
    }

    /// `DisableIntc` with interrupts enabled/disabled on entry and distinct
    /// handler ids; the real `_DisableIntc` runs on both sides.
    #[test]
    fn disable_intc_guarded_matches() {
        check(
            ee::s15::f_00424880__generated,
            disable_intc_guarded,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_003, 1);
                c.pc = 0x0042_4880;
                eie(c, seed);
                c.g[A0][0] = [1, 2, 4][seed as usize / 2 % 3] as u64;
            },
        );
    }

    /// `EnableIntc` with interrupts enabled/disabled on entry and distinct
    /// handler ids; the real `_EnableIntc` runs on both sides.
    #[test]
    fn enable_intc_guarded_matches() {
        check(
            ee::s15::f_004248e8__generated,
            enable_intc_guarded,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_004, 1);
                c.pc = 0x0042_48e8;
                eie(c, seed);
                c.g[A0][0] = [1, 2, 4][seed as usize / 2 % 3] as u64;
            },
        );
    }

    /// Prefix of `libkernl_IntrHelperThread` up to the first `WaitSema`
    /// (forced unwind): the helper never returns by design (thread main),
    /// so only the prologue and the first call are comparable here.
    #[test]
    fn intr_helper_thread_prefix_matches() {
        for seed in 0..8 {
            let setup = &|c: &mut Cpu| {
                machine(c, seed, 1_367_005, 1);
                c.pc = 0x0042_4aa0;
                c.g[A0][0] = (DATA + 0x4000) as u64;
                c.bus.write32(DATA + 0x4000, 1);
                for i in 0..16u32 {
                    c.bus.write8(DATA + 0x4008 + i, 0x41 + (i % 26) as u8);
                    c.bus.write8(DATA + 0x4009 + i, 0x42 + (i % 26) as u8);
                }
                c.bus.write32(fakes::LOG_UNWIND_AT, 0);
            };
            let r = compare(ee::s15::f_00424aa0, libkernl_intr_helper_thread, setup);
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
        }
    }

    /// `libkernl_InitThreadHelper` over the already-initialized word and the
    /// `CreateSema`/`CreateThread` success/failure arms (forced syscall
    /// results).
    #[test]
    fn init_thread_helper_matches() {
        hook_create_sema();
        hook_create_thread();
        check_reaching(
            ee::s15::f_00424b78__generated,
            libkernl_init_thread_helper,
            SEEDS,
            // The create-thread failure arm's DeleteSema, and the success
            // tail's ChangeThreadPriority.
            &[0x0042_4c00, 0x0042_4c34],
            &|c, seed| {
                machine(c, seed, 1_367_006, 1);
                c.pc = 0x0042_4b78;
                // Thread id word: already set (return -1) or clear.
                c.bus.write32(0x0045_5170, [1, 0][seed as usize % 2]);
                c.bus.write32(CREATE_SEMA_GATE, GATE_MAGIC);
                c.bus.write32(CREATE_THREAD_GATE, GATE_MAGIC);
                // CreateSema and CreateThread succeed or fail (-1),
                // independently; sema id 0 takes the success side of the
                // `<0` check (the sweep left V0==0 uncovered).
                c.bus.write32(
                    CREATE_SEMA_RESULT,
                    [5, 0xffff_ffff, 0][seed as usize / 2 % 3],
                );
                c.bus.write32(
                    CREATE_THREAD_RESULT,
                    [7, 0xffff_ffff][seed as usize / 4 % 2],
                );
            },
        );
    }

    /// Gate + forced result for the `CreateSema` fake below.
    pub(crate) const CREATE_SEMA_GATE: u32 = DATA + 0x66a0;

    pub(crate) const CREATE_SEMA_RESULT: u32 = DATA + 0x66a4;

    /// `CreateSema` with the gate set: report the forced id (sign-extended,
    /// so -1 steers the failure arms). Anywhere else (or with the gate
    /// clear): the slot body.
    pub(crate) fn fake_create_sema(c: &mut Cpu) {
        if c.bus.read32(CREATE_SEMA_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_create_sema),
                ee::s15::f_00423da0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(CREATE_SEMA_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_create_sema() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423DA0),
            fake_create_sema,
            fakes::prev_cell(fake_create_sema),
        );
    }

    /// Gate + forced result for the `CreateThread` fake below.
    pub(crate) const CREATE_THREAD_GATE: u32 = DATA + 0x66a8;

    pub(crate) const CREATE_THREAD_RESULT: u32 = DATA + 0x66ac;

    /// `CreateThread` with the gate set: report the forced id
    /// (sign-extended). Anywhere else (or with the gate clear): the slot
    /// body.
    pub(crate) fn fake_create_thread(c: &mut Cpu) {
        if c.bus.read32(CREATE_THREAD_GATE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(fake_create_thread),
                ee::s15::f_00423ba0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(CREATE_THREAD_RESULT) as i32 as i64 as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn hook_create_thread() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423BA0),
            fake_create_thread,
            fakes::prev_cell(fake_create_thread),
        );
    }

    /// `libkernl_TLBExceptionHandler`: leaf saving the thread state; all
    /// inputs distinctive.
    #[test]
    fn tlb_exception_handler_matches() {
        check(
            ee::s15::f_0042cfc0__generated,
            libkernl_tlb_exception_handler,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_007, 1);
                c.pc = 0x0042_cfc0;
                c.cop0[12] = 0x1010_0000 + seed;
                c.cop0[13] = 0x2020_0000 + seed;
                c.cop0[14] = 0x0030_0000 + seed;
                c.cop0[8] = 0x4040_0000 + seed;
                c.hi[0] = 0x1111_0000 + seed as u64;
                c.hi[1] = 0x2222_0000 + seed as u64;
                c.lo[0] = 0x3333_0000 + seed as u64;
                c.lo[1] = 0x4444_0000 + seed as u64;
                c.sa = 0x5555_0000 + seed as u32;
                c.g[A2][0] = 0x6666_0000 + seed as u64;
            },
        );
    }

    /// `libkernl_ExceptionReturnHandler`: leaf restoring the thread state;
    /// the saved words and COP0 are distinctive, EPC names the return.
    #[test]
    fn exception_return_handler_matches() {
        check(
            ee::s15::f_0042d100__generated,
            libkernl_exception_return_handler,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_367_008, 1);
                c.pc = 0x0042_d100;
                c.cop0[12] = 0x1010_0000 + seed;
                c.bus.write32(0x0053_0580, 0x1111_0000 + seed);
                c.bus.write32(0x0053_0588, 0x2222_0000 + seed);
                c.bus.write32(0x0053_0590, 0x3333_0000 + seed);
                c.bus.write32(0x0053_0598, 0x4444_0000 + seed);
                c.bus.write32(0x0053_05a0, 0x5555_0000 + seed);
                c.bus.write32(0x0053_05a8, 0x0040_0000 + seed);
                for i in 0..0x1f0u32 {
                    c.bus.write8(0x0053_0390 + i, 0x61 + (i % 26) as u8);
                }
            },
        );
    }
    pub(crate) use crate::game::lib::gcc::tests::exit_arm_fakes;
    pub(crate) use crate::game::lib::libc::tests::glue_machine;
    pub(crate) use crate::game::lib::libc::tests::SEEDS_4 as SEEDS_3;

    #[test]
    fn exit_matches_2() {
        check(
            ee::s15::f_00411310__generated,
            exit_3,
            SEEDS_3,
            &|c, seed| {
                glue_machine(c, seed, 0);
                let mut r = Rng(0x1310 ^ seed as u64);
                c.g[A0][0] = (r.next_u32() % 256) as u64;
                exit_arm_fakes(c);
            },
        );
    }

    use crate::difftest::fill;
    use crate::difftest::strict_ok;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc1601_install;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc1601_machine;

    /// The copy fixture: the event schedule for `seed`, then a2 = `count`,
    /// a0 = dst and a1 = src in DATA, both filled with random words.
    pub(crate) fn oc1597_copy_setup(c: &mut Cpu, seed: u32, entry: u32, count: u32) -> Events {
        let mut r = machine(c, seed, entry as u64, 0);
        let mode = schedule(c, &mut r, seed, entry);
        fill(c, &mut r, DATA + 0x1000, 32);
        fill(c, &mut r, DATA + 0x2000, 32);
        c.g[A0][0] = (DATA + 0x1000) as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = count as u64;
        mode
    }

    #[test]
    fn kcopy_3_impl_matches() {
        check_events(
            ee::s15::f_0042c4d0__generated,
            kcopy_3_impl,
            200,
            &|c, seed| {
                let n = if seed % 7 == 0 { 0 } else { 1 + seed % 20 };
                oc1597_copy_setup(c, seed, 0x0042_c4d0, n)
            },
        );
    }

    #[test]
    fn kcopy_4_impl_matches() {
        check_events(
            ee::s15::f_0042c780__generated,
            kcopy_4_impl,
            200,
            &|c, seed| {
                // a2 = words * 4 + (0..3): the shift drops the low bits.
                let words = if seed % 7 == 0 { 0 } else { 1 + seed % 9 };
                oc1597_copy_setup(c, seed, 0x0042_c780, words * 4 + seed % 4)
            },
        );
    }

    #[test]
    fn kcopy_5_impl_matches() {
        check_events(
            ee::s15::f_0042cb78__generated,
            kcopy_5_impl,
            200,
            &|c, seed| {
                let words = if seed % 7 == 0 { 0 } else { 1 + seed % 9 };
                oc1597_copy_setup(c, seed, 0x0042_cb78, words * 4 + seed % 4)
            },
        );
    }

    /// Target number of fake calls of the helper-thread resume test.
    pub(crate) const OC1601_TOTAL: u32 = DATA + 0x21_6020;

    /// The generated helper thread, driven until the fake call number
    /// `OC1601_TOTAL` unwinds.
    pub(crate) fn oc1601_helper_generated(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, total);
        ee::s15::f_00424aa0__generated(c);
    }

    /// The port, driven until the fake call number `OC1601_TOTAL` unwinds.
    pub(crate) fn oc1601_helper_port(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, total);
        libkernl_intr_helper_thread_2(c);
    }

    /// The port driven the same way as a native thread would be: every fake
    /// call unwinds, and the thread is re-entered at the call's return address
    /// (the fake's results are the same as for a normal return), until call
    /// number `OC1601_TOTAL` unwinds.
    pub(crate) fn oc1601_helper_resumed(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, 0);
        libkernl_intr_helper_thread_2(c);
        while c.pc & 0xffff_0000 == 0xdead_0000 {
            let n = (c.pc & 0xffff) >> 4;
            if n >= total {
                break;
            }
            c.bus.write32(fakes::LOG_UNWIND_AT, n + 1);
            let ret = fakes::logged_returns(c)[n as usize];
            c.pc = ret;
            libkernl_intr_helper_thread_2(c);
        }
    }

    /// Queue and registers of the helper thread: the read index, 64 records of
    /// every type (wake, rotate, suspend, two printf kinds).
    pub(crate) fn oc1601_helper_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1601_machine(c, seed, 0x0042_4aa0, 0);
        let queue = DATA + 0x4000;
        c.bus.write32(queue, r.next_u32() % 20);
        for i in 0..64u32 {
            c.bus.write8(queue + 8 + 2 * i, (r.next_u32() % 5) as u8);
            c.bus.write8(queue + 9 + 2 * i, r.next_u32() as u8);
        }
        c.bus
            .write32(0x0053_0000u32.wrapping_sub(0x4c60), r.next_u32());
        c.bus.write32(OC1601_TOTAL, 1 + seed % 11);
        c.g[A0][0] = queue as u64;
        c.pc = 0x0042_4aa0;
    }

    /// `libkernl_IntrHelperThread` over many dispatched records: wake, rotate,
    /// suspend and printf arms, the loop back edge, ending at a chosen fake
    /// call's unwind (the thread never returns).
    #[test]
    fn libkernl_intr_helper_thread_matches() {
        oc1601_install();
        for seed in 0..60 {
            let r = compare(oc1601_helper_generated, oc1601_helper_port, &|c| {
                oc1601_helper_setup(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(strict_ok(&r.soft), "seed {seed} (scratch): {:#?}", r.soft);
        }
    }

    /// The resume shape: the port unwinding at every call and re-entered at the
    /// call's return address ends in exactly the state of the generated
    /// (uninterrupted) body.
    #[test]
    fn libkernl_intr_helper_thread_resumes() {
        oc1601_install();
        for seed in 0..60 {
            let r = compare(oc1601_helper_generated, oc1601_helper_resumed, &|c| {
                oc1601_helper_setup(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(strict_ok(&r.soft), "seed {seed} (scratch): {:#?}", r.soft);
        }
    }
}
