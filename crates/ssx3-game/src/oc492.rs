//! Staging module oc492 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc492`.

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
    (0x0016_7f10, manual_cam_algo_dtor, "ManualCamAlgo_dtor"),
    (0x0016_7f40, editor_cam_algo_dtor, "EditorCamAlgo_dtor"),
    (
        0x0016_7f70,
        rider_camera_algorithm_dtor,
        "RiderCameraAlgorithm_dtor",
    ),
    (0x0016_8030, camera_base_dtor, "CameraBase_dtor"),
    (
        0x0016_8298,
        chase_camera_algorithm_global_ctors,
        "ChaseCameraAlgorithm_GlobalCtors",
    ),
    (
        0x0016_82f8,
        lazy_chase_camera_algorithm_dtor,
        "LazyChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_8690,
        look_back_chase_camera_algorithm_dtor,
        "LookBackChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_8a08,
        mid_right_chase_camera_algorithm_dtor,
        "MidRightChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_8d18,
        mid_left_chase_camera_algorithm_dtor,
        "MidLeftChaseCameraAlgorithm_dtor",
    ),
    (
        0x0016_bee8,
        camera_trigger_list_ctor,
        "CameraTriggerList_ctor",
    ),
    (0x0016_c5a8, camera_trigger_dtor, "CameraTrigger_dtor"),
    (
        0x0016_cdc8,
        camera_trigger_man_load,
        "CameraTriggerMan_Load",
    ),
];

/// `operator delete` (`__builtin_delete`, 0x317e50).
const OPERATOR_DELETE: u32 = 0x0031_7e50;
/// `ChaseCameraAlgorithm_dtor`, the base destructor the chase variants chain to.
const CHASE_CAMERA_BASE_DTOR: u32 = 0x0016_2458;
/// `ChaseCameraAlgorithm_StaticInit` (direct call from the global ctor).
const CHASE_CAMERA_STATIC_INIT: u32 = 0x0016_73f8;
/// `CameraTriggerMan_LoadFile`.
const CAMERA_TRIGGER_MAN_LOAD_FILE: u32 = 0x0016_c170;
/// The vtable word of the camera classes: `0x460000` minus the offset of the
/// class's vtable inside the image.
const VTABLE_BASE: u32 = 0x0046_0000;

/// The deleting tail of a destructor: `a1 &= 1; if (a1) operator
/// delete(this)`, the call returning to `ret`. Two cycles are charged only
/// when it frees.
fn delete_when_flagged(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A1][0] = c.g[A1][0] & 0x1;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE, ret)?;
    }
    Ok(())
}

/// A deleting destructor of a camera algorithm: stores `vtable` at
/// `this + field`, then frees `this` when the delete flag is set.
fn vtable_deleting_dtor(c: &mut Cpu, vtable: u32, field: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    let vt = c.g[V0][0];
    Addr(r32(c, A0).wrapping_add(field)).sw(c, vt);
    delete_when_flagged(c, ret)?;
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A chase-camera destructor that chains to the base destructor
/// (`ChaseCameraAlgorithm_dtor`) after storing its own vtable at `this + 0x10`.
fn chained_chase_dtor(c: &mut Cpu, vtable: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    let vt = c.g[V0][0];
    Addr(r32(c, V1).wrapping_add(0x10)).sw(c, vt);
    call_ptr(c, CHASE_CAMERA_BASE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ManualCamAlgo::~ManualCamAlgo` (0x167f10): vtable at +0x10, then the
/// deleting tail.
pub fn manual_cam_algo_dtor(c: &mut Cpu) {
    guest(c, |c| {
        vtable_deleting_dtor(c, VTABLE_BASE - 0x4220, 0x10, 0x0016_7f34)
    })
}

/// `EditorCamAlgo::~EditorCamAlgo` (0x167f40): same shape as
/// [`manual_cam_algo_dtor`].
pub fn editor_cam_algo_dtor(c: &mut Cpu) {
    guest(c, |c| {
        vtable_deleting_dtor(c, VTABLE_BASE - 0x4220, 0x10, 0x0016_7f64)
    })
}

/// `RiderCameraAlgorithm::~RiderCameraAlgorithm` (0x167f70): same shape as
/// [`manual_cam_algo_dtor`].
pub fn rider_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        vtable_deleting_dtor(c, VTABLE_BASE - 0x4220, 0x10, 0x0016_7f94)
    })
}

/// `CameraBase::~CameraBase` (0x168030): the vtable goes to +0x90 (the
/// base's own slot), then the deleting tail.
pub fn camera_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        vtable_deleting_dtor(c, VTABLE_BASE - 0x4750, 0x90, 0x0016_8054)
    })
}

/// `ChaseCameraAlgorithm_GlobalCtors` (0x168298): one-time static setup with
/// `a0 = 1`, `a1 = 0xffff`, through `ChaseCameraAlgorithm_StaticInit`.
pub fn chase_camera_algorithm_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: CHASE_CAMERA_STATIC_INIT,
            f: ee::s02::f_001673f8,
        }
        .call(c, 0x0016_82ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LazyChaseCameraAlgorithm::~LazyChaseCameraAlgorithm` (0x1682f8): vtable
/// at +0x10, then the chase base destructor.
pub fn lazy_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        chained_chase_dtor(c, VTABLE_BASE - 0x39d8, 0x0016_8314)
    })
}

/// `LookBackChaseCameraAlgorithm::~LookBackChaseCameraAlgorithm` (0x168690).
pub fn look_back_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        chained_chase_dtor(c, VTABLE_BASE - 0x3960, 0x0016_86ac)
    })
}

/// `MidRightChaseCameraAlgorithm::~MidRightChaseCameraAlgorithm` (0x168a08).
pub fn mid_right_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        chained_chase_dtor(c, VTABLE_BASE - 0x3ac8, 0x0016_8a24)
    })
}

/// `MidLeftChaseCameraAlgorithm::~MidLeftChaseCameraAlgorithm` (0x168d18).
pub fn mid_left_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        chained_chase_dtor(c, VTABLE_BASE - 0x3a50, 0x0016_8d34)
    })
}

/// `CameraTriggerList::CameraTriggerList(this)` (0x16bee8): clears the
/// list's first word and returns `this` in v0.
pub fn camera_trigger_list_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(r32(c, A0)).sw(c, 0);
    ret(c);
}

/// `CameraTrigger::~CameraTrigger` (0x16c5a8): the flag-gated delete with no
/// vtable store.
pub fn camera_trigger_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        delete_when_flagged(c, 0x0016_c5c0)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan::Load(this)` (0x16cdc8): loads the trigger file; when
/// that returns 0, clears the `+0x20` word of each trigger region in the
/// list at `this + 0` (`count` at `this + 4`), and returns 0.
pub fn camera_trigger_man_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: CAMERA_TRIGGER_MAN_LOAD_FILE,
            f: ee::s02::f_0016c170,
        }
        .call(c, 0x0016_cddc)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                loop {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    Addr(r32(c, V1).wrapping_add(0x20)).sw(c, 0);
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_cdf8);
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x10);
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
    use crate::difftest::fakes;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the fakes below answer only when this is in the machine's
    /// RAM, so other tests in the process see the real callees.
    const OC492_CTRL: u32 = DATA + 0x20_0000;
    const OC492_MAGIC: u32 = 0x0492_0492;

    static OC492_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC492_PREV_BASE: AtomicUsize = AtomicUsize::new(0);
    static OC492_PREV_LOAD_FILE: AtomicUsize = AtomicUsize::new(0);

    /// `operator delete` with the gate on: return at once, nothing freed.
    fn oc492_free_fake(c: &mut Cpu) {
        if c.bus.read32(OC492_CTRL) != OC492_MAGIC {
            return fakes::fall_back(c, &OC492_PREV_FREE, ee::s10::f_00317e50__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The chase base destructor with the gate on: return at once.
    fn oc492_base_fake(c: &mut Cpu) {
        if c.bus.read32(OC492_CTRL) != OC492_MAGIC {
            return fakes::fall_back(c, &OC492_PREV_BASE, ee::s01::f_00162458__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// `CameraTriggerMan_LoadFile` with the gate on: returns the result the
    /// setup wrote at `OC492_CTRL + 4`.
    fn oc492_load_file_fake(c: &mut Cpu) {
        if c.bus.read32(OC492_CTRL) != OC492_MAGIC {
            return fakes::fall_back(c, &OC492_PREV_LOAD_FILE, ee::s02::f_0016c170__generated);
        }
        c.g[V0][0] = c.bus.read32(OC492_CTRL + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    fn oc492_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc492_free_fake,
            &OC492_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162458),
            oc492_base_fake,
            &OC492_PREV_BASE,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C170),
            oc492_load_file_fake,
            &OC492_PREV_LOAD_FILE,
        );
    }

    /// Destructor fixture: `this` at DATA, the delete flag from the seed's
    /// low bit (the other bits random), the gate on.
    fn oc492_dtor_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x492_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.bus.write32(OC492_CTRL, OC492_MAGIC);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = (r.next_u32() as u64 & !1) | (seed as u64 & 1);
    }

    #[test]
    fn manual_cam_algo_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00167f10__generated,
            manual_cam_algo_dtor,
            200,
            &|c, seed| oc492_dtor_setup(c, seed, 0x0016_7f10),
        );
    }

    #[test]
    fn editor_cam_algo_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00167f40__generated,
            editor_cam_algo_dtor,
            200,
            &|c, seed| oc492_dtor_setup(c, seed, 0x0016_7f40),
        );
    }

    #[test]
    fn rider_camera_algorithm_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00167f70__generated,
            rider_camera_algorithm_dtor,
            200,
            &|c, seed| oc492_dtor_setup(c, seed, 0x0016_7f70),
        );
    }

    #[test]
    fn camera_base_dtor_matches() {
        oc492_hooks();
        check(ee::s02::f_00168030__generated, camera_base_dtor, 200, &|c, seed| {
            oc492_dtor_setup(c, seed, 0x0016_8030)
        });
    }

    #[test]
    fn chase_camera_algorithm_global_ctors_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00168298__generated,
            chase_camera_algorithm_global_ctors,
            20,
            &|c, seed| {
                let mut r = Rng(0x492_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_8298;
                c.g[GP][0] = sx(GAME_GP);
            },
        );
    }

    #[test]
    fn lazy_chase_camera_algorithm_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_001682f8__generated,
            lazy_chase_camera_algorithm_dtor,
            200,
            &|c, seed| {
                oc492_dtor_setup(c, seed, 0x0016_82f8);
            },
        );
    }

    #[test]
    fn look_back_chase_camera_algorithm_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00168690__generated,
            look_back_chase_camera_algorithm_dtor,
            200,
            &|c, seed| {
                oc492_dtor_setup(c, seed, 0x0016_8690);
            },
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00168a08__generated,
            mid_right_chase_camera_algorithm_dtor,
            200,
            &|c, seed| {
                oc492_dtor_setup(c, seed, 0x0016_8a08);
            },
        );
    }

    #[test]
    fn mid_left_chase_camera_algorithm_dtor_matches() {
        oc492_hooks();
        check(
            ee::s02::f_00168d18__generated,
            mid_left_chase_camera_algorithm_dtor,
            200,
            &|c, seed| {
                oc492_dtor_setup(c, seed, 0x0016_8d18);
            },
        );
    }

    #[test]
    fn camera_trigger_list_ctor_matches() {
        check(
            ee::s02::f_0016bee8__generated,
            camera_trigger_list_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x492_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_bee8;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, r.next_u32());
            },
        );
    }

    #[test]
    fn camera_trigger_dtor_matches() {
        oc492_hooks();
        check(ee::s02::f_0016c5a8__generated, camera_trigger_dtor, 200, &|c, seed| {
            oc492_dtor_setup(c, seed, 0x0016_c5a8);
        });
    }

    #[test]
    fn camera_trigger_man_load_matches() {
        oc492_hooks();
        check(
            ee::s02::f_0016cdc8__generated,
            camera_trigger_man_load,
            200,
            &|c, seed| {
                let mut r = Rng(0x492_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_cdc8;
                c.bus.write32(OC492_CTRL, OC492_MAGIC);
                // Result of CameraTriggerMan_LoadFile: nonzero on a third of the seeds.
                c.bus
                    .write32(OC492_CTRL + 4, if seed % 3 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(DATA + 0x1000);
                // The list: pointer array at +0, count at +4 (negative on some seeds).
                let count = match seed % 6 {
                    5 => u32::MAX,
                    k => k,
                };
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write32(DATA + 0x1004, count);
                for i in 0..4 {
                    c.bus
                        .write32(DATA + 0x2000 + 4 * i, DATA + 0x3000 + 0x40 * i);
                    c.bus.write32(DATA + 0x3000 + 0x40 * i + 0x20, r.next_u32());
                }
            },
        );
    }
}
