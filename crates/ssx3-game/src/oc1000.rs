//! Staging module oc1000 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1000`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_ca88,
        ov_screen_pda_template_dtor,
        "OVScreenPDATemplate_dtor",
    ),
    (
        0x0021_caa8,
        ov_screen_pda_template_ctor,
        "OVScreenPDATemplate_ctor",
    ),
    (
        0x0021_cae0,
        ov_screen_message_center_dtor,
        "OVScreenMessageCenter_dtor",
    ),
    (
        0x0021_cb40,
        ov_state_view_popup_dtor,
        "cOVState_ViewPopup_dtor",
    ),
    (
        0x0021_cb70,
        ov_state_static_init_record,
        "OVStateStatic_InitRecord",
    ),
    (0x0021_cc40, ov_state_hud1p_dtor, "cOVStateHUD1P_dtor"),
    (
        0x0021_cce0,
        ov_screen_freestyle_standings_dtor,
        "OVScreenFreestyleStandings_dtor",
    ),
    (
        0x0021_cd08,
        ov_screen_freestyle_standings_ctor,
        "OVScreenFreestyleStandings_ctor",
    ),
];

// The front-end screens and states below derive from the UI state base:
// `UIState` constructor at 0x39e2a0 and destructor at 0x39e390, both reached
// through the function table (`call_ptr`). Their vtables sit in the game
// image at 0x47xxxx (the `+8` word of each object).

/// `void OVScreenPDATemplate_dtor(OVScreenPDATemplate *self)` (0x21ca88): runs
/// the UIState destructor on `self`.
pub fn ov_screen_pda_template_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e390, 0x0021_ca98)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPDATemplate *OVScreenPDATemplate_ctor(OVScreenPDATemplate *self)`
/// (0x21caa8): runs the UIState constructor, then sets the screen's vtable
/// (0x4743f0) at `+8`. Returns `self` in `v0`.
pub fn ov_screen_pda_template_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_cabc)?;
        c.g[V1][0] = sx(0x0047_43f0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_dtor(OVScreenMessageCenter *self)` (0x21cae0):
/// sets the message-center vtable (0x474e08) at `+8` and runs the UIState
/// destructor.
pub fn ov_screen_message_center_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_4e08);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0021_cafc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVState_ViewPopup_dtor(cOVState_ViewPopup *self)` (0x21cb40): sets
/// the view-popup state vtable (0x474248) at `+8` and runs the UIState
/// destructor.
pub fn ov_state_view_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_4248);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0021_cb5c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVStateStatic_InitRecord(OVStateStatic *rec)` (0x21cb70): a leaf that
/// clears the record: word 0 = 1 (v0 keeps the 1), words +4, +8, +0xc, +0x10,
/// +0x14 = 0.
pub fn ov_state_static_init_record(c: &mut Cpu) {
    c.cycles += 8;
    let rec = c.g[A0][0] as u32;
    c.g[V0][0] = 0x1;
    c.sw(rec.wrapping_add(0x14), 0);
    c.sw(rec, c.g[V0][0]);
    c.sw(rec.wrapping_add(0x4), 0);
    c.sw(rec.wrapping_add(0x8), 0);
    c.sw(rec.wrapping_add(0xc), 0);
    c.sw(rec.wrapping_add(0x10), 0);
    ret(c);
}

/// `void cOVStateHUD1P_dtor(cOVStateHUD1P *self, int flags)` (0x21cc40): sets
/// the HUD state vtable (0x474160) at `+8`, destroys the HUD radar meters of
/// the rider array (`+0x48`, `0x8c` bytes each, back from `+0x160`) with
/// `HUDMeter_dtor(meter + 8, 2)`, then runs the UIState destructor with
/// `flags`.
pub fn ov_state_hud1p_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x0047_4160);
        frame.save(c, 17, 0x30);
        frame.save(c, 19, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        frame.save(c, 18, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        // The rider meter array pointer (`self + 0x48`) is not null.
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x160));
            let no_meters = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !no_meters {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8c));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    Callee {
                        addr: 0x0020_eb08,
                        f: ee::s05::f_0020eb08,
                    }
                    .call(c, 0x0021_cc94)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8c));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_cc88);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0039_e390, 0x0021_cca8)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenFreestyleStandings_dtor(OVScreenFreestyleStandings *self)`
/// (0x21cce0): the same body as the message-center destructor (vtable
/// 0x474e08 at `+8`, then the UIState destructor).
pub fn ov_screen_freestyle_standings_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_4e08);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0021_ccfc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenFreestyleStandings *OVScreenFreestyleStandings_ctor(
/// OVScreenFreestyleStandings *self, int a1, int a2)` (0x21cd08): runs the
/// UIState constructor, stores `a2` at `+0x98`, clears the words at `+0x48`,
/// `+0x4c` and `+0x54`, sets the standings vtable (0x474088) at `+8` and the
/// halfword at `+0x58` from the table at 0x4a2088. Returns `self` in `v0`.
pub fn ov_screen_freestyle_standings_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_cd24)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(0x0047_4e08));
        c.g[A1][0] = 0x4a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(0x0047_4088);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1000 fakes (a fixture address of this machine only).
    const GATE: u32 = DATA + 0x0020_1000;
    const MAGIC: u32 = 0x0c10_0d00;
    const SEEDS: u32 = 200;

    static OC1000_PREV_39E2A0: AtomicUsize = AtomicUsize::new(0);
    static OC1000_PREV_39E390: AtomicUsize = AtomicUsize::new(0);
    static OC1000_PREV_20EB08: AtomicUsize = AtomicUsize::new(0);

    /// The gated fake for a shared callee: logs the call like `fake_call`
    /// while the gate word is set, else runs the previous body.
    fn oc1000_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: fn(&mut Cpu)) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    fn oc1000_fake_39e2a0(c: &mut Cpu) {
        oc1000_gated(
            c,
            0x0039_e2a0,
            &OC1000_PREV_39E2A0,
            ee::s13::f_0039e2a0__generated,
        )
    }

    fn oc1000_fake_39e390(c: &mut Cpu) {
        oc1000_gated(
            c,
            0x0039_e390,
            &OC1000_PREV_39E390,
            ee::s13::f_0039e390__generated,
        )
    }

    fn oc1000_fake_20eb08(c: &mut Cpu) {
        oc1000_gated(
            c,
            0x0020_eb08,
            &OC1000_PREV_20EB08,
            ee::s05::f_0020eb08__generated,
        )
    }

    fn oc1000_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1000_fake_39e2a0,
            &OC1000_PREV_39E2A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1000_fake_39e390,
            &OC1000_PREV_39E390,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020EB08),
            oc1000_fake_20eb08,
            &OC1000_PREV_20EB08,
        );
    }

    /// Turn the shared-callee fakes on for this machine.
    fn oc1000_arm(c: &mut Cpu) {
        c.bus.write32(GATE, MAGIC);
    }

    /// Scramble the machine (the `calls` fake unwinds at a random call) and
    /// arm the shared-callee fakes; the caller sets `self` in `a0`.
    fn oc1000_object(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        oc1000_arm(c);
    }

    #[test]
    fn ov_screen_pda_template_dtor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021ca88__generated,
            ov_screen_pda_template_dtor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21ca88, 1);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn ov_screen_pda_template_ctor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021caa8__generated,
            ov_screen_pda_template_ctor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21caa8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_message_center_dtor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021cae0__generated,
            ov_screen_message_center_dtor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21cae0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_state_view_popup_dtor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021cb40__generated,
            ov_state_view_popup_dtor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21cb40, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_state_static_init_record_matches() {
        check(
            ee::s05::f_0021cb70__generated,
            ov_state_static_init_record,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x21cb_70 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x100, 0x10);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 3) * 0x40);
            },
        );
    }

    #[test]
    fn ov_state_hud1p_dtor_matches() {
        oc1000_install_fakes();
        check_events(
            ee::s05::f_0021cc40__generated,
            ov_state_hud1p_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21cc40, 0);
                oc1000_arm(c);
                // Seed 0 of 4: `self + 0x48` wraps to zero, a null meter
                // array, so the function skips the whole meter loop.
                let this = if seed % 4 == 0 {
                    0xffff_ffb8
                } else {
                    DATA + 0x100
                };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed & 1) as u64;
                fill(c, &mut r, DATA + 0x100, 0x80);
                schedule(c, &mut r, seed, 0x0021_cc40)
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_standings_dtor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021cce0__generated,
            ov_screen_freestyle_standings_dtor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21cce0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_standings_ctor_matches() {
        oc1000_install_fakes();
        covered(
            ee::s05::f_0021cd08__generated,
            ov_screen_freestyle_standings_ctor,
            SEEDS,
            &|c, seed| {
                oc1000_object(c, seed, 0x21cd08, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64).wrapping_mul(0x9e37) ^ 0x5a5a;
                c.bus.write16(0x4a_2088, seed as u16);
            },
        );
    }
}
