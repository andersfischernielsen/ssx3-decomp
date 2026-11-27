//! Staging module oc1004 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1004`.

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
    (0x0022_1110, ov_screen_bc_fail_dtor, "OVScreenBCFail_dtor"),
    (
        0x0022_1138,
        ov_screen_top_times_dtor,
        "OVScreenTopTimes_dtor",
    ),
    (
        0x0022_1160,
        ov_screen_top_times_ctor,
        "OVScreenTopTimes_ctor",
    ),
    (0x0022_11c8, ov_screen_map_ctor, "OVScreenMap_ctor"),
    (0x0022_1228, ov_screen_map_ctor_2, "OVScreenMap_ctor_2"),
    (0x0022_1298, ov_screen_map_dtor, "OVScreenMap_dtor"),
    (0x0022_12c8, ov_screen_session_ctor, "OVScreenSession_ctor"),
    (
        0x0022_1350,
        ov_screen_session_ctor_2,
        "OVScreenSession_ctor_2",
    ),
];

/// `cUIState_cUIState` (0x39e2a0): the base constructor every overlay state runs first.
const OV_STATE_BASE_CTOR: u32 = 0x0039_e2a0;
/// `cUIState__cUIState` (0x39e390): the base destructor the overlay destructors run.
const OV_STATE_BASE_DTOR: u32 = 0x0039_e390;
/// `EventSelect_ctor` (0x2006b8): the event-select object the map overlay builds.
const OV_EVENT_SELECT_CTOR: u32 = 0x0020_06b8;

/// The address `lui 0x470000; addiu off` forms: a vtable or static in the
/// 0x47xxxx data of the image (the overlay state vtables).
fn ov_image_sym(off: u32) -> u64 {
    let base: u32 = 0x0047_0000;
    sx(base.wrapping_add(off))
}

/// Shared body of the overlay state destructors (0x221110, 0x221138, 0x221298):
/// store the overlay vtable at `this+8` (`this` = `a0`), then run the base
/// destructor with the same `a0`. `ret` is the return address of that call.
fn ov_state_dtor_body(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = ov_image_sym(0x4e08);
    let vtable = c.g[V0][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
    call_ptr(c, OV_STATE_BASE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cOVState_BIGCHALLENGE_FAIL::~` (0x221110): overlay vtable at `this+8`, then the base dtor.
pub fn ov_screen_bc_fail_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_112c));
}

/// `cOVState_TOPTIMES::~` (0x221138): the same destructor shape as the BC fail screen.
pub fn ov_screen_top_times_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_1154));
}

/// `cOVState_MAP::~` (0x221298): the same destructor shape again.
pub fn ov_screen_map_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_12b4));
}

/// `cOVState_TOPTIMES::cOVState_TOPTIMES(this, engine, overlayId)` (0x221160):
/// the base constructor with `overlayId` (a2), then the top-times vtables and
/// fields (overlay id at +0x98, zeroed lists and counters, a halfword from the
/// game image's table at `0x4a2088` into +0x58). Returns `this` in v0.
pub fn ov_screen_top_times_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_117c)?;
        let this = Addr(c.g[S0][0] as u32);
        let overlay_id = c.g[S1][0];
        this.off(0x98).sw(c, overlay_id);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        c.g[V1][0] = vtable;
        this.off(0x8).sw(c, vtable);
        c.g[A1][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        c.g[V1][0] = ov_image_sym(0x34d0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let halfword = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = halfword;
        let v1 = c.g[V1][0];
        this.off(0x8).sw(c, v1);
        this.off(0x58).sh(c, halfword);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cOVState_MAP::cOVState_MAP` (0x2211c8, the one-argument form): the base
/// constructor with engine 0, then the map vtables and fields. Returns `this`.
pub fn ov_screen_map_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_11e0)?;
        let this = Addr(c.g[S0][0] as u32);
        this.off(0x98).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x48).sw(c, 0);
        c.g[V1][0] = vtable;
        this.off(0x8).sw(c, vtable);
        c.g[A1][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        c.g[V1][0] = ov_image_sym(0x33f8);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let halfword = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = halfword;
        let v1 = c.g[V1][0];
        this.off(0x8).sw(c, v1);
        this.off(0x58).sh(c, halfword);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cOVState_MAP::cOVState_MAP(this, engine, overlayId)` (0x221228): the base
/// constructor with the caller's arguments (overlay id saved from a2), the map
/// vtables and fields, then `EventSelect_ctor` on `this+0x9c`. Returns `this`.
pub fn ov_screen_map_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_1244)?;
        let this = Addr(c.g[S0][0] as u32);
        let overlay_id = c.g[S1][0];
        this.off(0x98).sw(c, overlay_id);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x48).sw(c, 0);
        this.off(0x8).sw(c, vtable);
        c.g[A1][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        let map_vtable = ov_image_sym(0x33f8);
        c.g[V0][0] = map_vtable;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
        let halfword = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = halfword;
        this.off(0x8).sw(c, map_vtable);
        this.off(0x58).sh(c, halfword);
        Callee {
            addr: OV_EVENT_SELECT_CTOR,
            f: ee::s05::f_002006b8,
        }
        .call(c, 0x0022_1280)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_002212C8` (0x2212c8), the session overlay's one-argument constructor:
/// the base constructor with engine 0, the session vtables and fields, and a
/// countdown loop of eight steps (`a1` from 7 down to -1) with a back-edge
/// event check. Returns `this`.
pub fn ov_screen_session_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_12e0)?;
        let this = Addr(c.g[S0][0] as u32);
        this.off(0x98).sw(c, 0);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x8).sw(c, vtable);
        c.g[A0][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        let session_vtable = ov_image_sym(0x3320);
        c.g[V0][0] = session_vtable;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 0x7;
        let halfword = Addr((c.g[A0][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = halfword;
        this.off(0x8).sw(c, session_vtable);
        this.off(0x58).sh(c, halfword);
        loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let more = c.g[A1][0] != c.g[A2][0];
            c.g[V0][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0022_1320);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// `cOVState_MAP::cOVState_MAP(this, engine, overlayId)` (0x221350), the
/// session overlay's three-argument constructor: the same shape as the
/// one-argument form with the overlay id from a2 and `this` kept in s1.
pub fn ov_screen_session_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_136c)?;
        let this = Addr(c.g[S1][0] as u32);
        let overlay_id = c.g[S0][0];
        this.off(0x98).sw(c, overlay_id);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x8).sw(c, vtable);
        c.g[A0][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        let session_vtable = ov_image_sym(0x3320);
        c.g[V0][0] = session_vtable;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 0x7;
        let halfword = Addr((c.g[A0][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = halfword;
        this.off(0x8).sw(c, session_vtable);
        this.off(0x58).sh(c, halfword);
        loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let more = c.g[A1][0] != c.g[A2][0];
            c.g[V0][0] = c.g[S1][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0022_13a8);
                }
                continue;
            }
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the test machine's RAM: the callee fakes answer only when it
    /// holds [`OC1004_MAGIC`] (otherwise they run the generated bodies).
    const OC1004_GATE: u32 = DATA + 0x000f_0000;
    const OC1004_MAGIC: u32 = 0x1004_c0de;
    /// The overlay object every test passes as `this`.
    const OC1004_THIS: u32 = DATA + 0x1000;

    static OC1004_PREV_BASE_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1004_PREV_BASE_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1004_PREV_EVENT_SELECT: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of the base constructor: logs the call and returns to `ra`.
    fn oc1004_fake_base_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_BASE_CTOR, ee::s13::f_0039e2a0__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_STATE_BASE_CTOR, ret);
    }

    /// Gated fake of the base destructor.
    fn oc1004_fake_base_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_BASE_DTOR, ee::s13::f_0039e390__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_STATE_BASE_DTOR, ret);
    }

    /// Gated fake of `EventSelect_ctor`.
    fn oc1004_fake_event_select(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_EVENT_SELECT, ee::s05::f_002006b8__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_EVENT_SELECT_CTOR, ret);
    }

    /// Hook the three fakes into their override slots (once per process).
    fn oc1004_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1004_fake_base_ctor,
            &OC1004_PREV_BASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1004_fake_base_dtor,
            &OC1004_PREV_BASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002006B8),
            oc1004_fake_event_select,
            &OC1004_PREV_EVENT_SELECT,
        );
    }

    /// Scrambled registers, the callee fakes armed, the overlay object filled
    /// with random words and passed as `a0`. Every other seed makes an event
    /// due at the first loop back edge.
    fn oc1004_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1004_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC1004_GATE, OC1004_MAGIC);
        // The halfword the constructors copy from the image table at 0x4a2088.
        c.bus.write32(0x004a_2088, r.next_u32());
        fill(c, &mut r, OC1004_THIS, 0x40);
        c.g[A0][0] = sx(OC1004_THIS);
        if seed % 2 == 0 {
            c.next_event = c.cycles;
        }
        r
    }

    #[test]
    fn ov_screen_bc_fail_dtor_matches() {
        covered(
            ee::s05::f_00221110__generated,
            ov_screen_bc_fail_dtor,
            240,
            &|c, seed| {
                oc1004_setup(c, seed, 0x0022_1110, 1);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_dtor_matches() {
        covered(
            ee::s05::f_00221138__generated,
            ov_screen_top_times_dtor,
            240,
            &|c, seed| {
                oc1004_setup(c, seed, 0x0022_1138, 1);
            },
        );
    }

    #[test]
    fn ov_screen_map_dtor_matches() {
        covered(
            ee::s05::f_00221298__generated,
            ov_screen_map_dtor,
            240,
            &|c, seed| {
                oc1004_setup(c, seed, 0x0022_1298, 1);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_ctor_matches() {
        covered(
            ee::s05::f_00221160__generated,
            ov_screen_top_times_ctor,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_1160, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_map_ctor_matches() {
        covered(
            ee::s05::f_002211c8__generated,
            ov_screen_map_ctor,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_11c8, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_map_ctor_2_matches() {
        covered(
            ee::s05::f_00221228__generated,
            ov_screen_map_ctor_2,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_1228, 2);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_session_ctor_matches() {
        covered(
            ee::s05::f_002212c8__generated,
            ov_screen_session_ctor,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_12c8, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_session_ctor_2_matches() {
        covered(
            ee::s05::f_00221350__generated,
            ov_screen_session_ctor_2,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_1350, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }
}
