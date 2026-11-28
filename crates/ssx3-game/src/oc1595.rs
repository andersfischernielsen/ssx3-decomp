//! Staging module oc1595 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1595`.

#![allow(unused_imports)]

use crate::call::{guest, Callee, Frame};
use crate::game::lib::kernel::syscall_wrapper;
use ssx3_ee as ee;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_3c40, i_change_thread_priority, "iChangeThreadPriority"),
    (0x0042_3cb0, i_refer_thread_status, "iReferThreadStatus"),
    (0x0042_3d80, end_of_heap, "EndOfHeap"),
    (0x0042_3e40, set_osd_config_param, "SetOsdConfigParam"),
    (0x0042_4050, i_flush_cache, "iFlushCache"),
    (0x0042_4110, set_vsync_flag, "SetVSyncFlag"),
    (0x0042_4a20, i_enable_intc_wrapper, "iEnableIntc"),
    (0x0042_4a40, i_disable_intc_wrapper, "iDisableIntc"),
];

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

/// `_iEnableIntc` (0x423b40): the callee of `iEnableIntc`.
const I_ENABLE_INTC: Callee = Callee {
    addr: 0x0042_3b40,
    f: ee::s15::f_00423b40,
};

/// `_iDisableIntc` (0x423b50): the callee of `iDisableIntc`.
const I_DISABLE_INTC: Callee = Callee {
    addr: 0x0042_3b50,
    f: ee::s15::f_00423b50,
};

/// The shared shape of `iEnableIntc` and `iDisableIntc`: 8 cycles, a 16-byte
/// frame holding `ra`, one call to the `_i*Intc` wrapper returning to `ret`,
/// then the frame is popped and `ra` returned to. The callee unwinding leaves
/// the frame as it is.
fn forward_to_intc(c: &mut Cpu, callee: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void iEnableIntc(void)` (0x424a20): forwards to `_iEnableIntc`, returning
/// to 0x424a30.
pub fn i_enable_intc_wrapper(c: &mut Cpu) {
    forward_to_intc(c, I_ENABLE_INTC, 0x0042_4a30);
}

/// `void iDisableIntc(void)` (0x424a40): forwards to `_iDisableIntc`,
/// returning to 0x424a50.
pub fn i_disable_intc_wrapper(c: &mut Cpu) {
    forward_to_intc(c, I_DISABLE_INTC, 0x0042_4a50);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine};

    /// A syscall wrapper: one trap, which the fake answers (and may unwind).
    fn syscall_matches(gen: GuestFn, hand: GuestFn, salt: u64) {
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

    /// The forwarding wrappers: the callee's trap is the one fake call; it
    /// may unwind, in which case the wrapper returns without popping its frame.
    #[test]
    fn i_enable_intc_wrapper_matches() {
        covered(
            ee::s15::f_00424a20__generated,
            i_enable_intc_wrapper,
            200,
            &|c: &mut Cpu, seed: u32| {
                machine(c, seed, 7, 1);
            },
        );
    }

    #[test]
    fn i_disable_intc_wrapper_matches() {
        covered(
            ee::s15::f_00424a40__generated,
            i_disable_intc_wrapper,
            200,
            &|c: &mut Cpu, seed: u32| {
                machine(c, seed, 8, 1);
            },
        );
    }
}
