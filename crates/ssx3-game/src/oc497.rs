//! Staging module oc497 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc497`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_cea8,
        camera_trigger_man_remove_trigger_regions_wrapper,
        "CameraTriggerMan_RemoveTriggerRegions_Wrapper",
    ),
    (
        0x0016_cf40,
        camera_trigger_man_empty_stub,
        "CameraTriggerMan_EmptyStub",
    ),
    (
        0x0016_d1d8,
        camera_trigger_man_update_triggers_mask1,
        "CameraTriggerMan_UpdateTriggersMask1",
    ),
    (
        0x0017_32b8,
        bounded_camera_algorithm_base_dtor,
        "BoundedCameraAlgorithmBase_dtor",
    ),
];

/// `void CameraTriggerMan_RemoveTriggerRegions_Wrapper(CameraTriggerMan* man)`
/// (0x16cea8): calls `CameraTriggerMan_RemoveTriggerRegions(man)` (0x16ca18)
/// and returns to its own caller.
pub fn camera_trigger_man_remove_trigger_regions_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0016_ca18,
            f: ee::s02::f_0016ca18,
        }
        .call(c, 0x0016_ceb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_EmptyStub()` (0x16cf40): does nothing.
pub fn camera_trigger_man_empty_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void CameraTriggerMan_UpdateTriggersMask1(CameraTriggerMan* man)`
/// (0x16d1d8): `CameraTriggerMan_UpdateTriggers(man, 1)`.
pub fn camera_trigger_man_update_triggers_mask1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x0016_cf48,
            f: ee::s02::CameraTriggerMan_UpdateTriggers,
        }
        .call(c, 0x0016_d1e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoundedCameraAlgorithmBase::~BoundedCameraAlgorithmBase(this, flags)`
/// (0x1732b8): stores the base vtable word 0x45bde0 at `this+0x10`, then when
/// bit 0 of `flags` is set frees `this` through `operator delete`
/// (0x317e50, with `this` still in `a0`). Leaves `v0` = 0x45bde0 and
/// `a1` = `flags & 1`, as the generated body does.
pub fn bounded_camera_algorithm_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0045_bde0);
        c.g[A1][0] &= 0x1;
        let this = Addr(c.g[A0][0] as u32);
        let base = c.g[V0][0];
        this.off(0x10).sw(c, base);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::f_00317e50,
            }
            .call(c, 0x0017_32dc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's `operator delete` fake (in the machine's RAM).
    const OC497_CTRL: u32 = DATA + 0xf_fd00;

    const OC497_MAGIC: u32 = 0x4f43_4497;

    static OC497_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    /// `operator delete` (0x317e50): logged, not run, while the gate is set.
    fn oc497_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC497_CTRL) == OC497_MAGIC {
            let ra = c.g[RA][0] as u32;
            return fake(c, 0x0031_7e50, ra);
        }
        fall_back(c, &OC497_PREV_DELETE, ee::s10::f_00317e50__generated)
    }

    /// `CameraTriggerMan_RemoveTriggerRegions_Wrapper` against the generated body.
    /// The count at +4 is zero, so the trigger loop is skipped and only the
    /// tail (stack and list resets) runs.
    #[test]
    fn camera_trigger_man_remove_trigger_regions_wrapper_matches() {
        check(
            ee::s02::f_0016cea8__generated,
            camera_trigger_man_remove_trigger_regions_wrapper,
            50,
            &|c, seed| {
                let mut r = Rng(0x497c_ea8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 4, 0);
                c.pc = 0x0016_cea8;
            },
        );
    }

    /// `CameraTriggerMan_EmptyStub`: only the return.
    #[test]
    fn camera_trigger_man_empty_stub_matches() {
        check(
            ee::s02::f_0016cf40__generated,
            camera_trigger_man_empty_stub,
            10,
            &|c, seed| {
                let mut r = Rng(0x497c_f40 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_cf40;
            },
        );
    }

    /// `CameraTriggerMan_UpdateTriggersMask1`: `UpdateTriggers(man, 1)`. The
    /// callee is the CameraTriggerMan fake from `cameratriggerman`'s tests,
    /// gated on that test module's machine magic (`oc438_machine`).
    #[test]
    fn camera_trigger_man_update_triggers_mask1_matches() {
        crate::game::camera::trigger::cameratriggerman::tests::oc438_install_fakes();
        crate::difftest::fakes::covered(
            ee::s02::f_0016d1d8__generated,
            camera_trigger_man_update_triggers_mask1,
            200,
            &|c, seed| {
                let mut r = crate::game::camera::trigger::cameratriggerman::tests::oc438_machine(
                    c,
                    seed,
                    0x0016_d1d8,
                    1,
                );
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    /// `BoundedCameraAlgorithmBase_dtor` with both flag values; the delete
    /// call goes to the gated fake above.
    #[test]
    fn bounded_camera_algorithm_base_dtor_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc497_fake_delete,
            &OC497_PREV_DELETE,
        );
        check(
            ee::s02::f_001732b8__generated,
            bounded_camera_algorithm_base_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x497_1732 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(OC497_CTRL, OC497_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x0017_32b8;
            },
        );
    }
}
