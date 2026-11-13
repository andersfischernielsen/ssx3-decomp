//! `src/fe/festatecredits.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::dirtysock::tagsunk::const_reg;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::sound::bankmonitor::leave;
use crate::guest::Addr;
use crate::tick;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_63b8,
        cfe_options_credits_update,
        "cFEOptionsCredits_Update",
    ),
    (
        0x0018_63e8,
        c_fe_options_credits_handle_input,
        "cFEOptionsCredits_HandleInput",
    ),
    (
        0x0018_6478,
        fe_options_credits_on_widget_notify,
        "cFEOptionsCredits_OnWidgetNotify",
    ),
    (
        0x0018_64b0,
        cfe_options_credits_on_widget_action,
        "cFEOptionsCredits_OnWidgetAction",
    ),
    (
        0x0018_65a8,
        cfe_state_on_deactivate,
        "cFEState_OnDeactivate",
    ),
    (
        0x0018_6610,
        fe_state_event_select_ctor,
        "cFEStateEventSelect_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;

    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// `void cFEOptionsCredits_Update(cFEOptionsCredits* self)` (0x1863b8):
/// advance the credits scroll (`+0x60 += +0x64`), then the base update.
///
/// The scroll is authored as a per-60 Hz-frame step: `+0x64` is a speed in
/// pixels per authored frame (it ramps between -8 and 8), so the position
/// advances once per 60 Hz boundary (`tick::ui_frame_due`) rather than once
/// per tick. Scaling the *step* with `* 60 / rate` is not usable here: a
/// 1 px/frame speed would truncate to 0 and the credits would stall while the
/// speed ramps up.
pub fn cfe_options_credits_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(self_, 0x60)).lw(c);
        c.g[A1][0] = Addr(at(self_, 0x64)).lw(c);
        // One authored frame per 60 Hz boundary; at 60 Hz that is every
        // update, so the original add runs unchanged.
        let step = if tick::ui_frame_due(c) {
            c.g[A1][0] as u32
        } else {
            0
        };
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(step));
        let y = c.g[V1][0];
        Addr(at(self_, 0x60)).sw(c, y);
        call_ptr(c, callee::UI_STATE_V13, 0x0018_63d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateEventSelect* cFEStateEventSelect_ctor(self)` (0x186610): UI state
/// constructor, then install the vtable/state and construct the embedded
/// event-select sub-object at +0x48.
pub fn fe_state_event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0018_6624)?;
        let this = r32(c, S0);
        const_reg(c, V1, 0x47_0000, -0x42d8);
        c.g[V0][0] = 0x13;
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        call_ptr(c, 0x0020_06b8, 0x0018_6640)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `int cFEOptionsCredits_OnWidgetNotify(...)` (0x186478): `Ok`/`Next`/`Prev`
/// (6, 8, 9) answer 0x100, everything else 0x101.
pub fn fe_options_credits_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if !is_six {
        c.cycles += 2;
        let below_six = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
        if !below_six {
            c.cycles += 2;
            let at_least_ten = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 8) as u64;
            if at_least_ten {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let below_eight = c.g[V0][0] != 0;
            if below_eight {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            // 8 <= id < 10
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_ACTIVATED;
    abi::ret(c);
}

/// `void cFEOptionsCredits_OnWidgetAction(self, item, int msg)` (0x1864b0):
/// for message 6, ask the engine object's slot 5 for the credits screen
/// (hash name 0x24) and push it if found.
pub fn cfe_options_credits_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let no_item = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if no_item {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            if c.g[A2][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x24;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0018_64f4);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_64f4)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0018_6508);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0018_6508)?;
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void cFEState_OnDeactivate(cFEState* self)` (0x1865a8): find the current
/// screen's credits object and tell the FE module it was left.
pub fn cfe_state_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_65c4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        call_ptr(c, 0x0039_7840, 0x0018_65c4)?;
        c.g[S0][0] = c.g[V0][0];
        let no_screen = c.g[S0][0] == 0;
        c.g[A0][0] = 0x4a0000;
        if no_screen {
            // The empty-screen path runs the shared tail (its +1 and lq16).
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0018_65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_65d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0018_65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_65e4)?;
            let not_found = c.g[V0][0] == 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if !not_found {
                c.cycles += 4;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[RA][0] = sx(0x0018_65fc);
                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                call_ptr(c, 0x001a_0708, 0x0018_65fc)?;
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `int cFEOptionsCredits_HandleInput(self, Pad* pad)` (0x1863e8): map the
/// pad's left/right presses to a credits scroll step (or a page turn).
pub fn c_fe_options_credits_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let pad = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = pad;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(self_);
        c.g[V1][0] = c.lw(at_2(c, S0, 8));
        c.g[A0][0] = c.lh(at_2(c, V1, 0x88));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x8c));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0018_6414)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
            c.g[V0][0] = ((c.g[V1][0] as i64) < -7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 1;
                c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.g[V1][0] = c.lw(at_2(c, S0, 8));
            c.cycles += 6;
            c.g[A0][0] = c.lh(at_2(c, V1, 0x90));
            c.g[V0][0] = c.lw(at_2(c, V1, 0x94));
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0018_6444)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let too_far = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                if too_far {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;

    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::fereal::tests::fake_slot;
    pub(crate) use crate::game::fe::fereal::tests::str_hash;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    use ssx3_ee as ee;

    use ssx3_ee::s02;

    #[test]
    fn cfe_options_credits_update_matches() {
        check(
            s02::f_001863b8__generated,
            cfe_options_credits_update,
            120,
            &|c, seed| {
                machine(c, seed, 0x1863b8, 1);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                c.bus.write32(OBJ + 0x60, 0x1000 + seed as u32);
                c.bus.write32(OBJ + 0x64, seed as u32 & 0x3f);
            },
        );
    }

    // --------------------------------------------------- rate: credits scroll

    /// A callee outside the translated code: answer null and return at once.
    /// The rate test below only looks at the scroll position.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// The credits position after `updates` updates at rate `hz` with speed
    /// `speed`, the VSync counter running from 1 (so a faster tick sees its
    /// 60 Hz boundaries).
    fn credits_position(hz: u32, updates: u32, speed: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let _ = machine(&mut c, 0, 0x1863b8, 0);
        globals(&mut c);
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        clear(&mut c, OBJ, 0x100);
        c.bus.write32(OBJ + 0x64, speed);
        for i in 0..updates {
            c.bus.write32(app + 0x18, 1 + i);
            c.g[A0][0] = sx(OBJ);
            cfe_options_credits_update(&mut c);
            assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        }
        c.bus.read32(OBJ + 0x60)
    }

    /// The credits scroll advances one authored frame per 60 Hz boundary: four
    /// such frames of speed 3 move 12 px whether that is 4 updates at 60 Hz, 8
    /// at 120 or 16 at 240, and a 1 px/frame speed is not truncated away.
    #[test]
    fn cfe_options_credits_scroll_follows_the_rate() {
        assert_eq!(credits_position(60, 4, 3), 12, "60 Hz, speed 3");
        assert_eq!(credits_position(120, 8, 3), 12, "120 Hz, speed 3");
        assert_eq!(credits_position(240, 16, 3), 12, "240 Hz, speed 3");
        assert_eq!(credits_position(60, 4, 1), 4, "60 Hz, speed 1");
        assert_eq!(credits_position(120, 8, 1), 4, "120 Hz, speed 1");
        assert_eq!(credits_position(240, 16, 1), 4, "240 Hz, speed 1");
    }

    #[test]
    fn fe_state_event_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00186610__generated,
            fe_state_event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x186610, 0);
                be_library(c);
                // Below 2 the event-select constructor skips the BE interface.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 5 },
                );
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_options_credits_on_widget_notify_matches() {
        check(
            ee::s02::f_00186478__generated,
            fe_options_credits_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0018_6478, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn cfe_options_credits_on_widget_action_matches() {
        check(
            ee::s02::f_001864b0__generated,
            cfe_options_credits_on_widget_action,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18_64b0, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
                c.g[A2][0] = if seed % 2 == 0 { 6 } else { (seed % 9) as u64 };
                // self+0x10 -> engine; engine+0 -> object; object+4 -> vtable.
                c.bus.write32(DATA + 0x10, DATA + 0x200);
                c.bus.write32(DATA + 0x200, DATA + 0x210);
                fake_slot(c, &mut r, DATA + 0x210, 0x4, 0x28);
            },
        );
    }

    #[test]
    fn cfe_state_on_deactivate_matches() {
        check(
            ee::s02::f_001865a8__generated,
            cfe_state_on_deactivate,
            40,
            &|c, seed| {
                base(c, seed, 0x18_65a8);
                let hash = str_hash(c, 0x004a_1398);
                c.g[A0][0] = sx(DATA);
                // The list at self+0x24: head at +0x28, sentinel check on +4.
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x28, 0);
                } else {
                    let screen = DATA + 0x300;
                    c.bus.write32(DATA + 0x28, screen);
                    c.bus.write32(screen + 0x4, DATA + 0x310);
                    if seed % 3 == 1 {
                        // FindObject walks a null array: returns null.
                        c.bus.write32(screen + 0x3c, 0);
                    } else {
                        let count = DATA + 0x600;
                        let array = DATA + 0x610;
                        let obj = DATA + 0x700;
                        c.bus.write32(screen + 0x38, count);
                        c.bus.write32(count, 4);
                        c.bus.write32(screen + 0x3c, array);
                        c.bus.write32(array, obj);
                        c.bus.write32(obj + 0x38, hash as u32);
                        c.bus.write8(obj + 0x95, 0x2a);
                    }
                }
                c.bus.write8(DATA + 0xc, 0x11);
            },
        );
    }

    /// `SSXAudioSystem_GetInstance`: returns `*(gp+0x410)`, 0 in tests.
    pub(crate) const RET_ZERO: u32 = 0x0028_b180;

    #[test]
    fn c_fe_options_credits_handle_input_matches() {
        check(
            ee::s02::f_001863e8__generated,
            c_fe_options_credits_handle_input,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1863, 0);
                env(c);
                let this = DATA + 0x400;
                let pad = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0x88, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                slot(c, vt, 0x90, if seed % 3 == 0 { RET_ZERO } else { EMPTY });
                w32(c, pad + 8, vt);
                w32(
                    c,
                    this + 0x64,
                    [0i32, 1, -7, -8, 7, 8, 9][(seed % 7) as usize] as u32,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
            },
        );
    }
}
