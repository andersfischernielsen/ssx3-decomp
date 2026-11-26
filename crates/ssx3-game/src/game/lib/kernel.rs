//! `src/lib/kernel.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
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
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(super) use crate::game::lib::libc::callee::STRLEN;

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
}
