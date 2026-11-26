//! Staging module oc793 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc793`.

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
    (0x001a_1ce8, fe_module_ctor, "FEModule_ctor"),
    (
        0x001a_0358,
        fe_module_rider_preview_play_pending_speech,
        "FEModule_RiderPreviewPlayPendingSpeech",
    ),
    (0x0018_0060, feui_input_is_down, "FEUIInput_IsDown"),
    (0x0018_0040, feui_input_is_up, "FEUIInput_IsUp"),
    (0x0038_ae60, ps2_bezier_man_ctor, "PS2BezierMan_ctor"),
    (0x002d_b980, ccrowd_anim2d_create, "cCrowdAnim2D_Create"),
];

/// Offset of the first `FEModule` member constructed after the `cFE` base
/// (0xb5ab0), the sub-object at 0xb5abc, and the three words cleared last.
pub(crate) const FE_MODULE_CLEAR_BASE: u32 = 0xb_0000;
pub(crate) const FE_MODULE_PENDING_SPEECH_A: u32 = 0xc1c;
pub(crate) const FE_MODULE_PENDING_SPEECH_B: u32 = 0xc20;

/// `FEModule* FEModule_ctor(FEModule* this)` (0x1a1ce8).
///
/// Runs the `cFE` base constructor, installs the vtable (0x46d7c0) at
/// `this`, constructs the sub-objects at `this+0xb5ab0` (0x263660) and
/// `this+0xb5abc` (0x263998), clears the words at `this+0xb5ad4/d8/dc` and
/// returns `this`.
pub fn fe_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001956f8, 0x001a_1cfc)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2840));
        c.g[A0][0] |= 0x5ab0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        call_ptr(c, 0x0026_3660, 0x001a_1d18)?;
        c.g[A0][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[A0][0] |= 0x5abc;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0026_3998, 0x001a_1d28)?;
        c.g[V1][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[V0][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore_ra(c, 0);
        let this_end = c.g[S0][0] as u32;
        c.sw(this_end.wrapping_add(0x5adc), 0);
        c.sw(this_end.wrapping_add(0x5ad4), 0);
        c.sw(this_end.wrapping_add(0x5ad8), 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RiderPreviewPlayPendingSpeech(FEModule* this)` (0x1a0358).
///
/// Two pending speech handles (`this+0xc1c`, `this+0xc20`): the first one,
/// if set, is passed with `*this` to 0x2a1778 and cleared; otherwise the
/// second one, if set, is passed with `*this` to 0x2a16b0 and cleared. Both
/// start with a 0x28b180 call that yields the sound system.
pub fn fe_module_rider_preview_play_pending_speech(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_A));
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001a_037c)?;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002a_1778, 0x001a_0388)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_A),
                0,
            );
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_B));
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                call_ptr(c, 0x0028_b180, 0x001a_03a0)?;
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a_16b0, 0x001a_03ac)?;
                c.sw(
                    (c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_B),
                    0,
                );
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offset of the input binding in a `FEUIInput` (`+0x18`).
pub(crate) const FEUI_INPUT_BINDING: u32 = 0x18;

/// `bool FEUIInput_IsDown(FEUIInput* this)` (0x180060): evaluates the
/// binding's input 0x73 (`InputBinding_EvalBool`, 0x320c48).
///
/// The generated body also has a resume entry at 0x180070 (after the call's
/// delay slot); resumes go to the generated `__entries` twin.
pub fn feui_input_is_down(c: &mut Cpu) {
    if c.pc != 0x0018_0060 {
        return ee::s02::f_00180060__entries(c);
    }
    feui_input_eval(c, 0x73, 0x0018_0074);
}

/// `bool FEUIInput_IsUp(FEUIInput* this)` (0x180040): evaluates the
/// binding's input 0x72 (`InputBinding_EvalBool`, 0x320c48).
pub fn feui_input_is_up(c: &mut Cpu) {
    feui_input_eval(c, 0x72, 0x0018_0054);
}

/// The shared body of `FEUIInput_IsUp/IsDown`: `InputBinding_EvalBool(
/// this->binding, input)`; `ret` is the call's return address.
fn feui_input_eval(c: &mut Cpu, input: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = input as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(FEUI_INPUT_BINDING));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PS2BezierMan* PS2BezierMan_ctor(PS2BezierMan* this)` (0x38ae60).
///
/// Runs the `BezierManBase` constructor, installs the vtable (0x493760 at
/// `this+4`), constructs the patch cache at `this+0x10`, runs
/// `PS2BezierMan_Init` and returns `this`.
pub fn ps2_bezier_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0037_4c90,
            f: ee::s12::f_00374c90,
        }
        .call(c, 0x0038_ae74)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3760));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        Callee {
            addr: 0x0037_2b30,
            f: ee::s12::f_00372b30,
        }
        .call(c, 0x0038_ae88)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0038_af30,
            f: ee::s12::f_0038af30,
        }
        .call(c, 0x0038_ae98)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cCrowdAnim2D* cCrowdAnim2D_Create(void* arg)` (0x2db980): allocates 0x20
/// bytes with `operator_new_named(0x20, name 0x4875d8, flags 0x20000000, 0)`
/// and runs `cCrowdAnim2D_ctor(mem, arg)`; returns the constructor's result.
pub fn ccrowd_anim2d_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75d8));
        c.g[A0][0] = 0x20;
        call_ptr(c, 0x0031_7d70, 0x002d_b9a8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x002d_b9c8,
            f: ee::s09::f_002db9c8,
        }
        .call(c, 0x002d_b9b4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    const OC793_CTRL: u32 = DATA + 0x20_0000;
    const OC793_MAGIC: u32 = 0x4f43_7933;

    macro_rules! oc793_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC793_CTRL) != OC793_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc793_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc793_fakes!(
        (
            oc793_fake_320c48,
            OC793_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc793_fake_263660,
            OC793_PREV_263660,
            ee::s07::OVERRIDE_00263660,
            ee::s07::f_00263660__generated,
            0x0026_3660
        ),
        (
            oc793_fake_263998,
            OC793_PREV_263998,
            ee::s07::OVERRIDE_00263998,
            ee::s07::f_00263998__generated,
            0x0026_3998
        ),
        (
            oc793_fake_28b180,
            OC793_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc793_fake_2a1778,
            OC793_PREV_2A1778,
            ee::s08::OVERRIDE_002A1778,
            ee::s08::f_002a1778__generated,
            0x002a_1778
        ),
        (
            oc793_fake_2a16b0,
            OC793_PREV_2A16B0,
            ee::s08::OVERRIDE_002A16B0,
            ee::s08::f_002a16b0__generated,
            0x002a_16b0
        ),
        (
            oc793_fake_374c90,
            OC793_PREV_374C90,
            ee::s12::OVERRIDE_00374C90,
            ee::s12::f_00374c90__generated,
            0x0037_4c90
        ),
        (
            oc793_fake_372b30,
            OC793_PREV_372B30,
            ee::s12::OVERRIDE_00372B30,
            ee::s12::f_00372b30__generated,
            0x0037_2b30
        ),
        (
            oc793_fake_317d70,
            OC793_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc793_fake_2db9c8,
            OC793_PREV_2DB9C8,
            ee::s09::OVERRIDE_002DB9C8,
            ee::s09::f_002db9c8__generated,
            0x002d_b9c8
        ),
        // Callees of the real `cFE_cFE` (0x1956f8) and `cFE_Init` helper
        // (0x2dbd10) that `FEModule_ctor` runs.
        (
            oc793_fake_231cd0,
            OC793_PREV_231CD0,
            ee::s06::OVERRIDE_00231CD0,
            ee::s06::f_00231cd0__generated,
            0x0023_1cd0
        ),
        (
            oc793_fake_15d698,
            OC793_PREV_15D698,
            ee::s01::OVERRIDE_0015D698,
            ee::s01::f_0015d698__generated,
            0x0015_d698
        ),
        (
            oc793_fake_15d700,
            OC793_PREV_15D700,
            ee::s01::OVERRIDE_0015D700,
            ee::s01::f_0015d700__generated,
            0x0015_d700
        ),
        (
            oc793_fake_19cdf0,
            OC793_PREV_19CDF0,
            ee::s03::OVERRIDE_0019CDF0,
            ee::s03::f_0019cdf0__generated,
            0x0019_cdf0
        ),
        (
            oc793_fake_1a03c0,
            OC793_PREV_1A03C0,
            ee::s03::OVERRIDE_001A03C0,
            ee::s03::f_001a03c0__generated,
            0x001a_03c0
        ),
        (
            oc793_fake_397b08,
            OC793_PREV_397B08,
            ee::s13::OVERRIDE_00397B08,
            ee::s13::f_00397b08__generated,
            0x0039_7b08
        ),
        (
            oc793_fake_3e18c8,
            OC793_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
    );

    /// A scrambled machine with the gated fakes on, `pc` at `entry`.
    fn oc793_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        oc793_install_fakes();
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC793_CTRL, OC793_MAGIC);
        r
    }

    /// `FEModule_ctor`: the real `cFE_cFE` and its helper run with logging
    /// fakes as callees; the two sub-object constructors are fakes; unwinds
    /// at random calls.
    #[test]
    fn fe_module_ctor_matches() {
        covered(
            ee::s03::f_001a1ce8__generated,
            fe_module_ctor,
            120,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x001a_1ce8, 14);
                c.g[A0][0] = sx(DATA + 0x4_0000);
            },
        );
    }

    /// `FEModule_RiderPreviewPlayPendingSpeech`: first handle set, only the
    /// second set, none set.
    #[test]
    fn fe_module_rider_preview_play_pending_speech_matches() {
        covered(
            ee::s03::f_001a0358__generated,
            fe_module_rider_preview_play_pending_speech,
            120,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x001a_0358, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let (a, b) = match seed % 4 {
                    0 => (r.next_u32() | 1, r.next_u32() | 1),
                    1 => (0, r.next_u32() | 1),
                    2 => (0, 0),
                    _ => (r.next_u32() | 1, 0),
                };
                c.bus.write32(this + FE_MODULE_PENDING_SPEECH_A, a);
                c.bus.write32(this + FE_MODULE_PENDING_SPEECH_B, b);
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    /// `FEUIInput_IsDown`: one forwarded call, answered or unwound.
    #[test]
    fn feui_input_is_down_matches() {
        covered(
            ee::s02::f_00180060__generated,
            feui_input_is_down,
            60,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x0018_0060, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + FEUI_INPUT_BINDING, r.next_u32());
            },
        );
    }

    /// `FEUIInput_IsUp`: one forwarded call, answered or unwound.
    #[test]
    fn feui_input_is_up_matches() {
        covered(
            ee::s02::f_00180040__generated,
            feui_input_is_up,
            60,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x0018_0040, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + FEUI_INPUT_BINDING, r.next_u32());
            },
        );
    }

    /// `PS2BezierMan_ctor`: the base and patch cache constructors are fakes.
    /// `PS2BezierMan_Init` (0x38af30, no override slot: a big real body)
    /// cannot be bounded on test data, so every seed unwinds in one of the
    /// first two calls and the path through `Init` is not covered (it is
    /// `v0 = this` plus the epilogue).
    #[test]
    fn ps2_bezier_man_ctor_matches() {
        covered(
            ee::s12::f_0038ae60__generated,
            ps2_bezier_man_ctor,
            60,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x0038_ae60, 2);
                c.bus.write32(fakes::LOG_UNWIND_AT, seed % 2);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    /// `cCrowdAnim2D_Create`: the allocation and the constructor are fakes.
    #[test]
    fn ccrowd_anim2d_create_matches() {
        covered(
            ee::s09::f_002db980__generated,
            ccrowd_anim2d_create,
            80,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x002d_b980, 2);
            },
        );
    }
}
