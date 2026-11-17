//! `src/seg/seg_3249a0.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

#[cfg(test)]
use ssx3_ee as ee;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

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
}
