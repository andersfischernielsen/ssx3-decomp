//! Staging module oc490 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc490`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_c738,
        be_score_int_destroy_instance,
        "BEScoreInt_DestroyInstance",
    ),
    (
        0x0015_dae0,
        game_camera_reset_all_chase_algorithms,
        "GameCamera_ResetAllChaseAlgorithms",
    ),
    (
        0x0016_0478,
        manual_chase_camera_algorithm_dtor,
        "ManualChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_07a0,
        matrix_chase_camera_algorithm_dtor,
        "MatrixChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_0bb8,
        relative_chase_camera_algorithm_dtor,
        "RelativeChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_16e8,
        idle_chase_camera_algorithm_dtor,
        "IdleChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_2290,
        chase_cam_controller_restore_default_algorithm,
        "ChaseCamController_RestoreDefaultAlgorithm",
    ),
    (
        0x0016_1f50,
        chase_cam_controller_end_script_camera,
        "ChaseCamController_EndScriptCamera",
    ),
    (0x0016_7de8, camera_algorithm_dtor, "CameraAlgorithm_dtor"),
    (
        0x0016_7ea0,
        manual_cam_algo_base_dtor,
        "ManualCamAlgoBase_dtor",
    ),
];

// ---------------------------------------------------------------------------
// `BEScoreInt_DestroyInstance` (0x15c738), `cBEScoreInterface_purgeInterface`.
//
// `void BEScoreInt_DestroyInstance(void)`: `delete` the score interface
// singleton kept at `gp - 0x1ea8`, then clear that slot.

pub fn be_score_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let slot = (c.g[GP][0] as u32).wrapping_sub(0x1ea8);
        c.g[A0][0] = Addr(slot).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0015_c74c)?;
        frame.restore_ra(c, 0);
        Addr(slot).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameCamera_ResetAllChaseAlgorithms` (0x15dae0), `cCamera_snapChaseControllers`.
//
// `void GameCamera_ResetAllChaseAlgorithms()`: snap the chase controllers of
// the race manager's two camera slots. Slot 0 always gets its algorithm's
// snap call; slot 1 only when the rider count is at least 2.

pub fn game_camera_reset_all_chase_algorithms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = world;
        let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
        c.g[V1][0] = cameras;
        c.g[S1][0] = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xa8)).lw(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_db1c)?;
        // Read the callee-saved registers after the call, as the body does.
        c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        let only_slot0 = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if !only_slot0 {
            c.cycles += 8;
            let riders = c.g[S1][0] as u32;
            c.g[V0][0] = Addr(riders.wrapping_add(0x8)).lw(c);
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0015_db44)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The chase camera algorithm destructors (0x160478, 0x1607a0, 0x160bb8,
// 0x1616e8). Each stores its own vtable at `+0x10` of `self`, then runs the
// base `ChaseCameraAlgorithm_dtor(self, flags)` with `flags` passed through.

/// Shared body of the four chase-subclass destructors.
fn oc490_chase_subclass_dtor(c: &mut Cpu, vtable: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(vtable);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_2458, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ManualChaseCameraAlgorithm_dtor(self, flags)` (0x160478).
pub fn manual_chase_camera_algorithm_dtor(c: &mut Cpu) {
    oc490_chase_subclass_dtor(c, 0x0046_0000u32.wrapping_sub(0x4530), 0x0016_0494)
}

/// `MatrixChaseCameraAlgorithm_dtor(self, flags)` (0x1607a0).
pub fn matrix_chase_camera_algorithm_dtor(c: &mut Cpu) {
    oc490_chase_subclass_dtor(c, 0x0046_0000u32.wrapping_sub(0x45a8), 0x0016_07bc)
}

/// `RelativeChaseCameraAlgorithm_dtor(self, flags)` (0x160bb8).
pub fn relative_chase_camera_algorithm_dtor(c: &mut Cpu) {
    oc490_chase_subclass_dtor(c, 0x0046_0000u32.wrapping_sub(0x4620), 0x0016_0bd4)
}

/// `IdleChaseCameraAlgorithm_dtor(self, flags)` (0x1616e8).
pub fn idle_chase_camera_algorithm_dtor(c: &mut Cpu) {
    oc490_chase_subclass_dtor(c, 0x0046_0000u32.wrapping_sub(0x44b8), 0x0016_1704)
}

// ---------------------------------------------------------------------------
// The camera algorithm base destructors (0x167de8 `CameraAlgorithm_dtor`,
// 0x167ea0 `ManualCamAlgoBase_dtor`). Store the base vtable at `+0x10`, then
// `delete self` (operator_delete) when bit 0 of `flags` is set.

/// Shared body of the two base-class destructors; `ret` is the return
/// address of the `operator_delete` call in each.
fn oc490_camera_base_dtor(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x4220));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        Addr((c.g[A0][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraAlgorithm_dtor(CameraAlgorithm* self, int flags)` (0x167de8).
pub fn camera_algorithm_dtor(c: &mut Cpu) {
    oc490_camera_base_dtor(c, 0x0016_7e0c)
}

/// `void ManualCamAlgoBase_dtor(ManualCamAlgoBase* self, int flags)` (0x167ea0).
pub fn manual_cam_algo_base_dtor(c: &mut Cpu) {
    oc490_camera_base_dtor(c, 0x0016_7ec4)
}

// ---------------------------------------------------------------------------
// `ChaseCamController_RestoreDefaultAlgorithm` (0x162290).
//
// `void ChaseCamController_RestoreDefaultAlgorithm(Ctrl* c)`: set the
// controller's algorithm back to the one stored at `+0x24`, immediately.

pub fn chase_cam_controller_restore_default_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24)).lw(c);
        call_ptr(c, 0x0015_d050, 0x0016_22a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ChaseCamController_EndScriptCamera` (0x161f50).
//
// `void ChaseCamController_EndScriptCamera(Ctrl* c)`: clear the script-camera
// bits (0 and 1) of the flag word at `+0x30`; unless bit 2 was set, switch to
// the algorithm at `+0x24`, otherwise to the fixed one (0x42). The new
// algorithm's argument comes from `+0x34`.

/// The flag word at `+0x30` of the controller.
pub(crate) const END_SCRIPT_CAM_FLAGS: u32 = 0x30;

pub fn chase_cam_controller_end_script_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xffff_ffff_ffff_fffd;
        c.g[A0][0] = 0xffff_ffff_ffff_fffe;
        c.g[A1][0] = 0x42;
        let ctrl = c.g[A2][0] as u32;
        c.g[V1][0] = Addr(ctrl.wrapping_add(END_SCRIPT_CAM_FLAGS)).lw(c);
        let flags = c.g[V1][0];
        let cleared = flags & c.g[V0][0];
        c.g[V0][0] = cleared & c.g[A0][0];
        c.g[V1][0] = flags & 4;
        let scripted = c.g[V1][0] != 0;
        Addr(ctrl.wrapping_add(END_SCRIPT_CAM_FLAGS)).sw(c, c.g[V0][0]);
        if !scripted {
            c.cycles += 1;
            c.g[A1][0] = Addr(ctrl.wrapping_add(0x24)).lw(c);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[A2][0];
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0015_d050, 0x0016_1f90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{DATA, GAME_GP};
    use crate::game::intersect::riderspheretree::tests::{
        oc430_install_fakes, OC430_CTRL, OC430_MAGIC,
    };
    use std::ptr::addr_of_mut;

    /// Gate word for the fake of `ChaseCamController_SetAlgorithmImmediate`
    /// (0x15d050), in the machine's own RAM.
    const OC490_CTRL: u32 = DATA + 0x30_0000;
    const OC490_MAGIC: u32 = 0x4f43_4f34;

    /// Gated fake of `ChaseCamController_SetAlgorithmImmediate`: logs the call
    /// while the gate is set, runs the previous body otherwise.
    fn oc490_fake_set_algo_immediate(c: &mut Cpu) {
        if c.bus.read32(OC490_CTRL) != OC490_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc490_fake_set_algo_immediate),
                ee::s01::f_0015d050__generated,
            );
        }
        fakes::fake_call(c, 0x0015_d050);
    }

    fn oc490_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015D050),
            oc490_fake_set_algo_immediate,
            fakes::prev_cell(oc490_fake_set_algo_immediate),
        );
    }

    /// The chase/camera destructors: gate the spline and `operator_delete`
    /// fakes on, `self` at `DATA + 0x1000`, random `flags`.
    fn oc490_dtor_check(generated: fn(&mut Cpu), hand: fn(&mut Cpu), entry: u32, salt: u64) {
        oc430_install_fakes();
        covered(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, salt, 6);
            c.bus.write32(OC430_CTRL, OC430_MAGIC);
            c.pc = entry;
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    #[test]
    fn be_score_int_destroy_instance_matches() {
        oc430_install_fakes();
        check(
            ee::s01::f_0015c738__generated,
            be_score_int_destroy_instance,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_c738, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_c738;
                let obj = if seed % 2 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(GAME_GP.wrapping_sub(0x1ea8), obj);
                c.bus.write32(DATA + 0x1000, r.next_u32());
            },
        );
    }

    #[test]
    fn game_camera_reset_all_chase_algorithms_matches() {
        covered(
            ee::s01::f_0015dae0__generated,
            game_camera_reset_all_chase_algorithms,
            200,
            &|c, seed| {
                machine(c, seed, 0x0015_dae0, 8);
                c.pc = 0x0015_dae0;
                let w = DATA + 0x1000;
                let x = DATA + 0x2000;
                let rm = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, w);
                c.bus.write32(w + 0x84, x);
                c.bus.write32(x + 0x84, rm);
                c.bus.write32(rm, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write32(rm + 4, DATA + 0x4000);
                c.bus.write32(rm + 8, DATA + 0x5000);
                for cam in [DATA + 0x4000, DATA + 0x5000] {
                    c.bus.write32(cam + 0xa8, DATA + 0x6000);
                    c.bus.write32(DATA + 0x6000 + 0x14, DATA + 0x7000);
                    c.bus.write16(DATA + 0x7000 + 0x20, 0);
                    c.bus.write32(DATA + 0x7000 + 0x24, FAKE);
                }
            },
        );
    }

    #[test]
    fn manual_chase_camera_algorithm_dtor_matches() {
        oc490_dtor_check(
            ee::s01::f_00160478__generated,
            manual_chase_camera_algorithm_dtor,
            0x0016_0478,
            0x0016_0478,
        );
    }

    #[test]
    fn matrix_chase_camera_algorithm_dtor_matches() {
        oc490_dtor_check(
            ee::s01::f_001607a0__generated,
            matrix_chase_camera_algorithm_dtor,
            0x0016_07a0,
            0x0016_07a0,
        );
    }

    #[test]
    fn relative_chase_camera_algorithm_dtor_matches() {
        oc490_dtor_check(
            ee::s01::f_00160bb8__generated,
            relative_chase_camera_algorithm_dtor,
            0x0016_0bb8,
            0x0016_0bb8,
        );
    }

    #[test]
    fn idle_chase_camera_algorithm_dtor_matches() {
        oc490_dtor_check(
            ee::s01::f_001616e8__generated,
            idle_chase_camera_algorithm_dtor,
            0x0016_16e8,
            0x0016_16e8,
        );
    }

    #[test]
    fn camera_algorithm_dtor_matches() {
        oc490_dtor_check(
            ee::s02::f_00167de8__generated,
            camera_algorithm_dtor,
            0x0016_7de8,
            0x0016_7de8,
        );
    }

    #[test]
    fn manual_cam_algo_base_dtor_matches() {
        oc490_dtor_check(
            ee::s02::f_00167ea0__generated,
            manual_cam_algo_base_dtor,
            0x0016_7ea0,
            0x0016_7ea0,
        );
    }

    #[test]
    fn chase_cam_controller_restore_default_algorithm_matches() {
        oc490_install_fakes();
        check(
            ee::s01::f_00162290__generated,
            chase_cam_controller_restore_default_algorithm,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0016_2290, 0);
                c.bus.write32(OC490_CTRL, OC490_MAGIC);
                c.pc = 0x0016_2290;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x24, r.next_u32());
            },
        );
    }

    #[test]
    fn chase_cam_controller_end_script_camera_matches() {
        oc490_install_fakes();
        check(
            ee::s01::f_00161f50__generated,
            chase_cam_controller_end_script_camera,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0016_1f50, 0);
                c.bus.write32(OC490_CTRL, OC490_MAGIC);
                c.pc = 0x0016_1f50;
                let ctrl = DATA + 0x1000;
                c.g[A0][0] = sx(ctrl);
                let flags = r.next_u32();
                let flags = if seed % 2 == 0 { flags | 4 } else { flags & !4 };
                c.bus.write32(ctrl + END_SCRIPT_CAM_FLAGS, flags);
                c.bus.write32(ctrl + 0x24, r.next_u32());
                c.bus.write32(ctrl + 0x34, r.next_u32());
            },
        );
    }
}
