//! `src/camera/cameracontroller.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_cba0, cam_controller_ctor, "CamController_ctor"),
    (0x0015_cc10, cam_controller_dtor, "CamController_dtor"),
    (0x0015_ccf0, cam_controller_reset, "CamController_Reset"),
    (
        0x0015_cd80,
        manual_cam_controller_ctor,
        "ManualCamController_ctor",
    ),
    (
        0x0015_ce10,
        manual_cam_controller_dtor,
        "ManualCamController_dtor",
    ),
    (
        0x0015_ce98,
        manual_cam_controller_reset_for_rider,
        "ManualCamController_ResetForRider",
    ),
    (
        0x0015_cee8,
        manual_cam_controller_reset,
        "ManualCamController_Reset",
    ),
    (
        0x0015_cf28,
        cam_controller_set_algorithm,
        "CamController_SetAlgorithm",
    ),
    (
        0x0015_d050,
        chase_cam_controller_set_algorithm_immediate,
        "ChaseCamController_SetAlgorithmImmediate",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;

    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;

    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;

    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;

    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;

    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;

    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;

    pub const CAM_CONTROLLER_RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0015_cc70,
        f: ee::s01::f_0015cc70,
    };
}

// ---------------------------------------------------------------------------
// Camera algorithms.

/// `ChaseCamController_SetAlgorithmImmediate(Ctrl* c, int algo, void* arg)`
/// (0x15d050): `SetAlgorithm(c, algo, arg, 1.0f)`.
pub fn chase_cam_controller_set_algorithm_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        frame.save_ra(c, 0);
        callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0015_d068)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera controller / algorithm constructors.

/// `void ManualCamController_Reset(this)` (0x15cee8): reset the controller and
/// then each algorithm in its list via the list's vtable slot at `+0x24`.
pub fn manual_cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = this;
        callee::CAM_CONTROLLER_RESET.call(c, 0x0015_cefc)?;
        c.g[A1][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_cf14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ManualCamController_ResetForRider(cam, rider)` (0x15ce98): reset the
/// rider's camera controller and tell its algorithm list (slot 0x18/0x1c) to
/// follow the rider.
pub fn manual_cam_controller_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        CAM_CONTROLLER_RESET_FOR_RIDER.call(c, 0x0015_ceb4)?;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
        let cam = c.g[A2][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = Addr(cam).off(0x10).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x18).lh(c);
        c.g[V1][0] = Addr(vt).off(0x1c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0015_ced0);
        c.g[A0][0] = sx(cam.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_ced0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_SetAlgorithm(CamController* self, void* algo,
/// void* arg)` (0x15cf28): replace the controller's algorithm list with one
/// algorithm.
pub fn cam_controller_set_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, A0);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::CAM_CONTROLLER_CLEAR_ALGORITHMS.call(c, 0x0015_cf4c)?;
        c.g[A0][0] = Addr(p(this, 8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S2][0];
        callee::ALGORITHM_LIST_PUSH.call(c, 0x0015_cf64)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_dtor(CamController* self, int flags)` (0x15cc10):
/// destroy the algorithm list and optionally free the object.
pub fn cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b938);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).off(0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        let no_list = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_list {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0015_cc44);
            c.g[A1][0] = 3;
            callee::ALGORITHM_LIST_DTOR.call(c, 0x0015_cc44)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_cc58);
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0015_cc58)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_Reset(CamController* self)` (0x15ccf0): reset every
/// camera node's override state and the current camera's chase/splash
/// managers.
pub fn cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S1, 8)).lw(c);
        c.g[S0][0] = Addr(at(c, V0, 0)).lw(c);
        let none = c.g[S0][0] == 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(at(c, A1, 0x10)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x20)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x24)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0015_cd2c)?;
                c.g[S0][0] = Addr(at(c, S0, 0x14)).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_cd18);
                }
            }
        }
        c.cycles += 9;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0015_e030, 0x0015_cd40)?;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x002f_41a8, 0x0015_cd48)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cCameraController* cCameraController_ctor(cCameraController* c, int a)`
/// (0x15cba0).
pub fn cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0045_b938;
        sqsp(c, 17, 0x10);
        sqsp(c, 16, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x0045_b220;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        Addr(p_2(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 8;
        call::call_ptr(c, 0x0031_7d70, 0x0015_cbdc)?;
        c.g[A0][0] = c.g[V0][0];
        call::call_ptr(c, 0x0015_c930, 0x0015_cbe4)?;
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 0x10)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0xc)).sw(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p_2(r32(c, S0), 4)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// The `sDeleteTarget` sequence of the UI destructors: `v0 = lw(a2+slot)`,
/// `delta = lh(v0+8)`, `fn = lw(v0+0xc)`, `a0 = a2 + delta`, `jalr fn`. `a1`
/// (the delete kind, 3) must already be set.
pub(crate) fn delete_owner(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(p(r32(c, A2), slot)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
    c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
    let target = r32(c, V1);
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
    call_ptr(c, target, ret)
}

// ---------------------------------------------------------------------------
// Camera controllers.

/// `void ManualCamController_dtor(ManualCamController* self, int flags)`
/// (0x15ce10): release the two camera controllers and run the base
/// destructor.
pub fn manual_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b8d8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 0x10, 0x0015_ce54)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 0x10, 0x0015_ce74)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0015_ce80);
        c.g[A1][0] = c.g[S1][0];
        callee::CAM_CONTROLLER_DTOR.call(c, 0x0015_ce80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* ManualCamController_ctor(self, void* cam)` (0x15cd80): build the
/// camera controller and its two algorithms, then select the manual one.
pub fn manual_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let cam = abi::arg64(c, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::CAM_CONTROLLER_CTOR, 0x0015_cd94)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4728));
        c.g[A1][0] = 0x0046_0000;
        c.sw(at_2(c, S0, 0x14), c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dd0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.sw(r32(c, S0), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        let _ = cam;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cdc0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::MANUAL_CAM_ALGO_CTOR, 0x0015_cdc8)?;
        c.g[A1][0] = 0x0046_0000;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dc0));
        c.sw(at_2(c, S0, 0x18), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cde4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::EDITOR_CAM_ALGO_CTOR, 0x0015_cdec)?;
        c.sw(at_2(c, S0, 0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x51;
        call_ptr(c, callee::MANUAL_CAM_SELECT_ALGORITHM, 0x0015_cdfc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;
    pub(crate) use crate::game::be::bedbreward::tests::call_setup;
    pub(crate) use crate::game::be::beintmission::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_6;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn chase_cam_controller_set_algorithm_immediate_matches() {
        crate::difftest::check(
            ee::s01::f_0015d050__generated,
            chase_cam_controller_set_algorithm_immediate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15d0_50, 2);
                be_library(c);
                // Algorithm indices below 0x62 go through the (unloaded, all
                // zero) jump table, which leaves the function at once.
                c.g[A1][0] = (r.next_u32() % 0x62) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Camera controller / algorithm constructors.

    #[test]
    fn manual_cam_controller_reset_matches() {
        check(
            ssx3_ee::s01::f_0015cee8__generated,
            manual_cam_controller_reset,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce_e8, 0);
                let this = DATA;
                let list = DATA + 0x100;
                let vt = DATA + 0x200;
                c.g[A0][0] = this as u64;
                // `CamController_Reset` walks `*(this + 8)` (null: nothing to do)
                // and resets the camera at `+0x10` (null: the splash manager's
                // rider chain is zeroed too).
                c.bus.write32(this + 0x8, list);
                c.bus.write32(this + 0x10, 0);
                c.bus.write32(this + 0x18, DATA + 0x300);
                c.bus.write32(DATA + 0x300 + 0x10, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 7) as u16);
                c.bus.write32(vt + 0x24, FAKE);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x15ce98 ManualCamController_ResetForRider.

    #[test]
    fn manual_cam_controller_reset_for_rider_matches() {
        check(
            ee::s01::f_0015ce98__generated,
            manual_cam_controller_reset_for_rider,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce_98, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.bus.write32(this + 8, 0); // empty algorithm list
                c.bus.write32(this + 0x10, DATA + 0x200);
                c.bus.write32(this + 0x18, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(DATA + 0x300 + 0x10, vt);
            },
        );
    }

    #[test]
    fn cam_controller_set_algorithm_matches() {
        check(
            ee::s01::f_0015cf28__generated,
            cam_controller_set_algorithm,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15cf_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let list = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, list, 0x100);
                c.bus.write32(this + 8, list);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        call_setup(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn cam_controller_dtor_matches() {
        check(
            ee::s01::f_0015cc10__generated,
            cam_controller_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x15cc10, 2);
                alloc_ready_2(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// A v-table at `at` with `{delta = 0, fake}` at each slot.
    pub(crate) fn put_vtable(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 0);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn cam_controller_reset_matches() {
        check(
            ee::s01::f_0015ccf0__generated,
            cam_controller_reset,
            50,
            &|c, seed| {
                machine(c, seed, 0x15cc_f0, 0);
                c.g[A0][0] = sx(DATA);
                let n0 = DATA + 0x1000;
                let n1 = if seed % 2 == 0 { 0 } else { DATA + 0x1200 };
                c.bus.write32(DATA + 8, n0);
                c.bus
                    .write32(n0, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(n0 + 0x14, n1);
                if n1 != 0 {
                    c.bus.write32(n1, DATA + 0x2200);
                    c.bus.write32(n1 + 0x14, 0);
                }
                for p in [DATA + 0x2000u32, DATA + 0x2200] {
                    c.bus.write32(p + 0x10, p + 0x100);
                    put_vtable(c, p + 0x100, &[0x20]);
                }
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    /// The engine's size-class pool allocator, as in `oc106`: the "pools
    /// ready" flag plus eight free nodes per size class.
    pub(crate) fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w32(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w32(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w32(c, pool + head, base);
            w32(c, GAME_GP + slot, pool);
        }
    }

    #[test]
    fn cam_controller_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015cba0__generated,
            cam_controller_ctor,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x15cba0, 0);
                pool(c);
                let cam = DATA + 0x1000;
                c.g[A0][0] = cam as u64;
                c.g[A1][0] = (seed % 5) as u64;
            },
        );
    }

    /// A child object `obj+field` whose vtable (at `child+vt_field`) dispatches
    /// slot_2 8 to a fake.
    pub(crate) fn fake_child(c: &mut Cpu, obj: u32, field: u32, vt_field: u32, n: u32) {
        let child = DATA + 0x400 + 0x40 * n;
        let vt = DATA + 0x800 + 0x40 * n;
        w(c, obj + field, child);
        w(c, child + vt_field, vt);
        slot_2(c, vt, 8, 0, FAKE + 0x10 * n);
    }

    #[test]
    fn manual_cam_controller_dtor_matches() {
        check(
            ssx3_ee::s01::f_0015ce10__generated,
            manual_cam_controller_dtor,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15ce10, 3);
                be_library_2(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 != 0 {
                    fake_child(c, obj, 0x18, 0x10, 0);
                }
                if seed % 4 != 0 {
                    fake_child(c, obj, 0x1c, 0x10, 1);
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn manual_cam_controller_ctor_matches() {
        check(
            ee::s01::f_0015cd80__generated,
            manual_cam_controller_ctor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x15cd, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x10, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x800);
            },
        );
    }
}
