//! Staging module oc797 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc797`.

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
        0x0020_1720,
        event_select_reset_flags,
        "EventSelect_ResetFlags",
    ),
    (0x0039_e288, ui_sound_base_ctor, "UISound_BaseCtor"),
    (
        0x002b_1758,
        speech_system_cancel_pending_instance,
        "SpeechSystem_CancelPendingInstance",
    ),
    (0x0022_1df0, ov_screen_race_pre_dtor, "OVScreenRacePre_dtor"),
    (
        0x001d_f418,
        c_fe_state_rule_select_dtor,
        "cFEStateRuleSelect_dtor",
    ),
];

/// `EventSelect_ResetFlags(EventSelect* s)` (0x201720): clears +0x1c, sets
/// +0x10 and +0x14 to 1, and sets +0x18 to 1 when the gp word at -0xb5c is
/// below 2 (unsigned), else 0.
pub fn event_select_reset_flags(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(EVENT_SELECT_MODE_GP));
    c.g[V1][0] = 0x1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), 0u64);
    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
    let flag_zero = c.g[V0][0] == 0u64;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
    c.cycles += 2;
    if flag_zero {
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
    } else {
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    }
    ret(c);
}

/// The gp-relative word (gp - this) `EventSelect_ResetFlags` tests.
pub(crate) const EVENT_SELECT_MODE_GP: u32 = 0xb5c;

/// `UISound* UISound_BaseCtor(UISound* this)` (0x39e288): stores the base
/// vtable (0x46dd60) at +0 and returns `this`.
pub fn ui_sound_base_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x470000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x22a0));
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    ret(c);
}

/// `cCrowdRender2D* cCrowdRender2D_ctor(cCrowdRender2D* this)` (0x2db6d8):
/// runs `cCrowdRender2D_LoadTextures(this)` and returns `this`.
///
/// Not installed: `LoadTextures` runs the texture loader, which has no
/// bounded short path on test data, so there is no differential test.
#[allow(dead_code)]
pub fn c_crowd_render_2d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002d_b748,
            f: ee::s09::f_002db748,
        }
        .call(c, 0x002d_b6ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_CancelPendingInstance(SpeechSystem* s, int i)`
/// (0x2b1758): `SpeechInstance_CancelPending(s->instances[i])`.
pub fn speech_system_cancel_pending_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x002a_fd10,
            f: ee::s08::f_002afd10,
        }
        .call(c, 0x002b_1774)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenRacePre_dtor(this)` (0x221df0): sets the vtable word at +8 to the
/// UIState vtable (0x474e08), then calls `UIState_dtor(this, a1)`.
pub fn ov_screen_race_pre_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1e0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRuleSelect_dtor(this)` (0x1df418): sets the vtable word at +8
/// (0x46d0d0), then calls `UIState_dtor(this, a1)`.
pub fn c_fe_state_rule_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x001d_f434)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, ptr};
    use crate::difftest::{check, DATA, GAME_GP, RA as DT_RA};
    use std::sync::atomic::AtomicUsize;

    /// Arms oc797's gated callee fakes in a machine.
    const OC797_CTRL: u32 = DATA + 0x30_0000;
    const OC797_MAGIC: u32 = 0x0797_f4ce;
    /// Word the speech fake's forced result is read from (0: the fake's own).
    const OC797_MAIL: u32 = DATA + 0x30_0010;

    fn oc797_arm(c: &mut Cpu) {
        c.bus.write32(OC797_CTRL, OC797_MAGIC);
    }

    /// The fake callee: logs the call, then returns the forced `v0` of the
    /// mailbox (stored +1) unless it unwound.
    fn oc797_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC797_MAIL);
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc797_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC797_CTRL) != OC797_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc797_fake_call(c, $addr);
                }
            )*
            fn oc797_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc797_fakes!(
        (
            oc797_fk_39e390,
            OC797_PREV_39E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039_e390
        ),
        (
            oc797_fk_3d7ec8,
            OC797_PREV_3D7EC8,
            ee::s14::OVERRIDE_003D7EC8,
            ee::s14::f_003d7ec8__generated,
            0x003d_7ec8
        ),
    );

    /// `EventSelect_ResetFlags`: the mode word below and at/above 2.
    #[test]
    fn event_select_reset_flags_matches() {
        check(
            ee::s05::f_00201720__generated,
            event_select_reset_flags,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797a, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
                let mode = [0u32, 1, 2, 3, 0xffff_ffff, 7][seed as usize % 6];
                c.bus.write32(GAME_GP - EVENT_SELECT_MODE_GP, mode);
            },
        );
    }

    /// `UISound_BaseCtor`.
    #[test]
    fn ui_sound_base_ctor_matches() {
        check(
            ee::s13::f_0039e288__generated,
            ui_sound_base_ctor,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797b, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `SpeechSystem_CancelPendingInstance`: the instance table lookup, the
    /// forced non-zero/zero `SPCH_DequeueChannel` result and unwinds.
    #[test]
    fn speech_system_cancel_pending_instance_matches() {
        oc797_install_fakes();
        fakes::covered(
            ee::s08::f_002b1758__generated,
            speech_system_cancel_pending_instance,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797c, 1);
                oc797_arm(c);
                let sys = DATA + 0x1000;
                let table = DATA + 0x2000;
                let inst = DATA + 0x4000;
                c.g[A0][0] = sys as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write32(sys + 4, table);
                for i in 0..8 {
                    c.bus.write32(table + 4 * i, inst + 0x1000 * i);
                }
                c.bus
                    .write32(inst + 0x900 + 0x1000 * (seed % 8), r.next_u32());
                c.bus
                    .write32(OC797_MAIL, if seed % 3 == 0 { 1 } else { 2 + (seed % 2) });
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `OVScreenRacePre_dtor`: vtable store and the `UIState_dtor` call.
    #[test]
    fn ov_screen_race_pre_dtor_matches() {
        oc797_install_fakes();
        fakes::covered(
            ee::s05::f_00221df0__generated,
            ov_screen_race_pre_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797d, 1);
                oc797_arm(c);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `cFEStateRuleSelect_dtor`: same shape.
    #[test]
    fn c_fe_state_rule_select_dtor_matches() {
        oc797_install_fakes();
        fakes::covered(
            ee::s04::f_001df418__generated,
            c_fe_state_rule_select_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797e, 1);
                oc797_arm(c);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }
}
