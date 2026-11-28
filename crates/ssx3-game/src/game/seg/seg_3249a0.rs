//! `src/seg/seg_3249a0.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::game::lib::kernel::syscall_wrapper;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0042_3c50,
        rotate_thread_ready_queue,
        "RotateThreadReadyQueue",
    ),
    (0x0042_3ca0, refer_thread_status, "ReferThreadStatus"),
    (0x0042_39a0, set_gs_crt, "SetGsCrt"),
    (0x0042_3a50, set_vtlb_refill_handler, "SetVTLBRefillHandler"),
    (0x0042_3b20, set_alarm, "SetAlarm"),
    (0x0042_3b80, i_set_alarm, "iSetAlarm"),
    (0x0042_3bf0, terminate_thread, "TerminateThread"),
    (0x0042_3c00, i_terminate_thread, "iTerminateThread"),
    (
        0x0042_3c40,
        i_change_thread_priority,
        "iChangeThreadPriority",
    ),
    (0x0042_3cb0, i_refer_thread_status, "iReferThreadStatus"),
    (0x0042_3d80, end_of_heap, "EndOfHeap"),
    (0x0042_3e40, set_osd_config_param, "SetOsdConfigParam"),
    (0x0042_4050, i_flush_cache, "iFlushCache"),
    (0x0042_4110, set_vsync_flag, "SetVSyncFlag"),
];

/// `int ReferThreadStatus(int thid, ThreadParam* info)` (0x423ca0): syscall
/// 0x30.
pub fn refer_thread_status(c: &mut Cpu) {
    crate::game::lib::kernel::syscall_wrapper(c, 0x30, 0x0042_3ca4, 0x0042_3ca8);
}

/// `int RotateThreadReadyQueue(int prio)` (0x423c50): syscall 0x2b.
pub fn rotate_thread_ready_queue(c: &mut Cpu) {
    crate::game::lib::kernel::syscall_wrapper(c, 0x2b, 0x0042_3c54, 0x0042_3c58);
}

/// `void SetGsCrt(int interlace, int mode, int field)` (0x4239a0): syscall 2.
pub fn set_gs_crt(c: &mut Cpu) {
    syscall_wrapper(c, 0x2, 0x0042_39a4, 0x0042_39a8);
}

/// `void SetVTLBRefillHandler(int which, void (*handler)(void))` (0x423a50):
/// syscall 0xd.
pub fn set_vtlb_refill_handler(c: &mut Cpu) {
    syscall_wrapper(c, 0xd, 0x0042_3a54, 0x0042_3a58);
}

/// `void SetAlarm(usec ticks, handler, arg)` (0x423b20): syscall 0xfc.
pub fn set_alarm(c: &mut Cpu) {
    syscall_wrapper(c, 0xfc, 0x0042_3b24, 0x0042_3b28);
}

/// `void iSetAlarm(int ticks, handler, arg)` (0x423b80): interrupt-disabled
/// syscall -254.
pub fn i_set_alarm(c: &mut Cpu) {
    syscall_wrapper(c, (-254i32) as u32, 0x0042_3b84, 0x0042_3b88);
}

/// `void TerminateThread(int thid)` (0x423bf0): syscall 0x25.
pub fn terminate_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x25, 0x0042_3bf4, 0x0042_3bf8);
}

/// `void iTerminateThread(int thid)` (0x423c00): interrupt-disabled syscall
/// -38.
pub fn i_terminate_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-38i32) as u32, 0x0042_3c04, 0x0042_3c08);
}

/// `iChangeThreadPriority` (0x423c40): the interrupt-level kernel call with
/// code -42 (`v1`). The trap sits at 0x423c44, the return is 0x423c48.
pub fn i_change_thread_priority(c: &mut Cpu) {
    syscall_wrapper(c, (-42i32) as u32, 0x0042_3c44, 0x0042_3c48);
}

/// `iReferThreadStatus` (0x423cb0): the interrupt-level kernel call with
/// code -49. The trap sits at 0x423cb4, the return is 0x423cb8.
pub fn i_refer_thread_status(c: &mut Cpu) {
    syscall_wrapper(c, (-49i32) as u32, 0x0042_3cb4, 0x0042_3cb8);
}

/// `void *EndOfHeap(void)` (0x423d80): the kernel call with code 0x3e. The
/// trap sits at 0x423d84, the return is 0x423d88.
pub fn end_of_heap(c: &mut Cpu) {
    syscall_wrapper(c, 0x3e, 0x0042_3d84, 0x0042_3d88);
}

/// `void SetOsdConfigParam(unsigned *p)` (0x423e40): the kernel call with
/// code 0x4a. The trap sits at 0x423e44, the return is 0x423e48.
pub fn set_osd_config_param(c: &mut Cpu) {
    syscall_wrapper(c, 0x4a, 0x0042_3e44, 0x0042_3e48);
}

/// `void iFlushCache(int)` (0x424050): the interrupt-level kernel call with
/// code -104. The trap sits at 0x424054, the return is 0x424058.
pub fn i_flush_cache(c: &mut Cpu) {
    syscall_wrapper(c, (-104i32) as u32, 0x0042_4054, 0x0042_4058);
}

/// `void SetVSyncFlag(...)` (0x424110): the kernel call with code 0x73. The
/// trap sits at 0x424114, the return is 0x424118.
pub fn set_vsync_flag(c: &mut Cpu) {
    syscall_wrapper(c, 0x73, 0x0042_4114, 0x0042_4118);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ssx3_ee as ee;

    /// ReferThreadStatus: fake syscalls (which may unwind).
    #[test]
    fn refer_thread_status_matches() {
        crate::game::lib::kernel::tests::syscall_test(
            ee::s15::f_00423ca0__generated,
            refer_thread_status,
            0x423ca0,
        );
    }

    /// RotateThreadReadyQueue: fake syscalls (which may unwind).
    #[test]
    fn rotate_thread_ready_queue_matches() {
        crate::game::lib::kernel::tests::syscall_test(
            ee::s15::f_00423c50__generated,
            rotate_thread_ready_queue,
            0x423c50,
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::game::lib::kernel::tests::syscall_test;

    #[test]
    fn set_gs_crt_matches() {
        syscall_test(ee::s15::f_004239a0__generated, set_gs_crt, 0x1594_01);
    }

    #[test]
    fn set_vtlb_refill_handler_matches() {
        syscall_test(
            ee::s15::f_00423a50__generated,
            set_vtlb_refill_handler,
            0x1594_02,
        );
    }

    #[test]
    fn set_alarm_matches() {
        syscall_test(ee::s15::f_00423b20__generated, set_alarm, 0x1594_03);
    }

    #[test]
    fn i_set_alarm_matches() {
        syscall_test(ee::s15::f_00423b80__generated, i_set_alarm, 0x1594_04);
    }

    #[test]
    fn terminate_thread_matches() {
        syscall_test(ee::s15::f_00423bf0__generated, terminate_thread, 0x1594_05);
    }

    #[test]
    fn i_terminate_thread_matches() {
        syscall_test(
            ee::s15::f_00423c00__generated,
            i_terminate_thread,
            0x1594_06,
        );
    }

    /// A syscall wrapper: one trap, which the fake answers (and may unwind).
    pub(crate) fn syscall_matches(gen: GuestFn, hand: GuestFn, salt: u64) {
        covered(gen, hand, 200, &|c: &mut Cpu, seed: u32| {
            machine(c, seed, salt, 1);
        });
    }

    #[test]
    fn i_change_thread_priority_matches() {
        syscall_matches(ee::s15::f_00423c40__generated, i_change_thread_priority, 1);
    }

    #[test]
    fn i_refer_thread_status_matches() {
        syscall_matches(ee::s15::f_00423cb0__generated, i_refer_thread_status, 2);
    }

    #[test]
    fn end_of_heap_matches() {
        syscall_matches(ee::s15::f_00423d80__generated, end_of_heap, 3);
    }

    #[test]
    fn set_osd_config_param_matches() {
        syscall_matches(ee::s15::f_00423e40__generated, set_osd_config_param, 4);
    }

    #[test]
    fn i_flush_cache_matches() {
        syscall_matches(ee::s15::f_00424050__generated, i_flush_cache, 5);
    }

    #[test]
    fn set_vsync_flag_matches() {
        syscall_matches(ee::s15::f_00424110__generated, set_vsync_flag, 6);
    }
}
