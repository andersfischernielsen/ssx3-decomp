//! Staging module oc1013 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1013`.

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
        0x0022_1d58,
        ov_screen_in_game_error_dtor,
        "OVScreenInGameError_dtor",
    ),
    (
        0x0022_1d88,
        ov_screen_profile_save_pda_dtor,
        "OVScreenProfileSavePDA_dtor",
    ),
    (
        0x0022_1db0,
        ov_screen_profile_save_pda_ctor_2,
        "OVScreenProfileSavePDA_ctor_2",
    ),
    (0x0022_1e18, ov_screen_race_pre_ctor, "OVScreenRacePre_ctor"),
    (
        0x0022_1e80,
        ov_screen_freestyle_pre_dtor,
        "OVScreenFreestylePre_dtor",
    ),
    (
        0x0022_1ea8,
        ov_screen_freestyle_pre_ctor,
        "OVScreenFreestylePre_ctor",
    ),
    (
        0x0022_1f10,
        ov_screen_multi_freestyle_pre_dtor,
        "OVScreenMultiFreestylePre_dtor",
    ),
    (
        0x0022_1f38,
        ov_screen_multi_freestyle_pre_ctor,
        "OVScreenMultiFreestylePre_ctor",
    ),
];

/// The destructor shape shared by the `OVScreen*` classes: a 0x10 frame, the
/// vtable word `0x470000 + vt_off` stored at `this + 8`, then a tail call of
/// the base destructor `base_dtor` returning to `ret`. The frame closes after it.
fn oc1013_screen_dtor(c: &mut Cpu, vt_off: u32, base_dtor: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(vt_off));
        let vt = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vt);
        call_ptr(c, base_dtor, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The constructor shape of the three `OVScreen*Pre` classes: a 0x30 frame
/// keeping `this` in s0 and the third argument in s1, the `UIState` base
/// constructor (0x39e2a0) returning to `call_ret`, then the vtable words
/// (`0x470000 + 0x4e08` at +8, `0x470000 + vt_off` after the reload) and the
/// zeroed fields. `this + 0x58` gets the halfword at `0x4a0000 + 0x2088`.
fn oc1013_screen_ctor(c: &mut Cpu, call_ret: u32, vt_off: u32) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, call_ret)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        let s1 = c.g[S1][0];
        Addr(this.wrapping_add(0x98)).sw(c, s1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        c.g[A1][0] = 0x004a_0000;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V1][0] = 0x0047_0000;
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vt_off));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        let vt2 = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt2);
        let a0 = c.g[A0][0];
        Addr(this.wrapping_add(0x58)).sh(c, a0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError_dtor` (0x221d58): vtable `0x474e08`, then the
/// `UIState` base destructor (0x39e390).
pub fn ov_screen_in_game_error_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1d74)
}

/// `OVScreenProfileSavePDA_dtor` (0x221d88): vtable `0x472928`, then the
/// `MemCardStateBase` destructor (0x1d5428).
pub fn ov_screen_profile_save_pda_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x2928, 0x001d_5428, 0x0022_1da4)
}

/// `OVScreenProfileSavePDA_ctor_2` (0x221db0): the `MemCardStateBase`
/// constructor (0x1d53b0) with a0 = this and a1 = a2 = 0, then the vtable
/// `0x472928` at this + 8. v0 ends as `this`.
pub fn ov_screen_profile_save_pda_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_53b0, 0x0022_1dcc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2928));
        frame.restore_ra(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenRacePre_ctor` (0x221e18): the `UIState` constructor, then the
/// race-pre screen fields and vtable `0x472850`.
pub fn ov_screen_race_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1e34, 0x2850)
}

/// `OVScreenFreestylePre_dtor` (0x221e80): vtable `0x474e08`, then the
/// `UIState` base destructor.
pub fn ov_screen_freestyle_pre_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1e9c)
}

/// `OVScreenFreestylePre_ctor` (0x221ea8): as the race-pre constructor, with
/// vtable `0x472778`.
pub fn ov_screen_freestyle_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1ec4, 0x2778)
}

/// `OVScreenMultiFreestylePre_dtor` (0x221f10): vtable `0x474e08`, then the
/// `UIState` base destructor.
pub fn ov_screen_multi_freestyle_pre_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1f2c)
}

/// `OVScreenMultiFreestylePre_ctor` (0x221f38): as the race-pre constructor,
/// with vtable `0x4726a0`.
pub fn ov_screen_multi_freestyle_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1f54, 0x26a0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC1013_MAGIC` (otherwise the body they replaced runs).
    const OC1013_CTRL: u32 = DATA + 0x0040_0000;
    const OC1013_MAGIC: u32 = 0x1013_c0de;
    /// The fixture screen object (`this`).
    const OC1013_O: u32 = DATA + 0x0050_0000;

    static OC1013_PREV_39E390: AtomicUsize = AtomicUsize::new(0);
    static OC1013_PREV_39E2A0: AtomicUsize = AtomicUsize::new(0);
    static OC1013_PREV_1D5428: AtomicUsize = AtomicUsize::new(0);
    static OC1013_PREV_1D53B0: AtomicUsize = AtomicUsize::new(0);

    fn oc1013_fake_39e390(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_39E390, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    fn oc1013_fake_39e2a0(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_39E2A0, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    fn oc1013_fake_1d5428(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_1D5428, ee::s04::f_001d5428__generated);
        }
        fakes::fake_call(c, 0x001d_5428);
    }

    fn oc1013_fake_1d53b0(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_1D53B0, ee::s04::f_001d53b0__generated);
        }
        fakes::fake_call(c, 0x001d_53b0);
    }

    /// Hook the four base-class bodies once per process.
    fn oc1013_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1013_fake_39e390,
            &OC1013_PREV_39E390,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1013_fake_39e2a0,
            &OC1013_PREV_39E2A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D5428),
            oc1013_fake_1d5428,
            &OC1013_PREV_1D5428,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
            oc1013_fake_1d53b0,
            &OC1013_PREV_1D53B0,
        );
    }

    /// Scrambled machine with one logged callee call (it may unwind) and the
    /// gate on.
    fn oc1013_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 1);
        c.bus.write32(OC1013_CTRL, OC1013_MAGIC);
        r
    }

    #[test]
    fn ov_screen_in_game_error_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221d58__generated,
            ov_screen_in_game_error_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0001);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221d88__generated,
            ov_screen_profile_save_pda_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0002);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_ctor_2_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221db0__generated,
            ov_screen_profile_save_pda_ctor_2,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0003);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_race_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221e18__generated,
            ov_screen_race_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0004);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_pre_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221e80__generated,
            ov_screen_freestyle_pre_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0005);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221ea8__generated,
            ov_screen_freestyle_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0006);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221f10__generated,
            ov_screen_multi_freestyle_pre_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0007);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221f38__generated,
            ov_screen_multi_freestyle_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0008);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }
}
