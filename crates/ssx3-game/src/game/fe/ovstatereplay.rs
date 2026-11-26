//! `src/fe/ovstatereplay.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Frame;
use crate::call::Target;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p;
pub(crate) use crate::game::ui::uilistbox::oc711_lw;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_de30,
        ov_screen_replay_menu_on_widget_created,
        "OVScreenReplayMenu_OnWidgetCreated",
    ),
    (
        0x0020_e818,
        ov_screen_replay_menu_update_progress_bar,
        "OVScreenReplayMenu_UpdateProgressBar",
    ),
    (0x0026_f980, replay_end_playback, "Replay_EndPlayback"),
    (
        0x0020_e8e0,
        cov_state_overlay_render,
        "cOVStateOverlay_Render",
    ),
    (
        0x0020_e900,
        ov_state_overlay_update,
        "cOVStateOverlay_Update",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `OVScreenReplayMenu_UpdateProgressBar(this)` (0x20e818): move the replay
/// progress handle to `170 + 315 * played/total` (0 when there is no replay).
pub fn ov_screen_replay_menu_update_progress_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x1cc8));
        callee::STR_HASH.call(c, 0x0020_e834)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0020_e840)?;
        c.g[A0][0] = c.g[V0][0];
        let absent = c.g[A0][0] == 0;
        c.lq(16, frame.at(c, 0x20));
        if !absent {
            c.cycles += 12;
            c.ldl(2, abi::r32(c, A0).wrapping_add(0x4b));
            c.ldr(2, abi::r32(c, A0).wrapping_add(0x44));
            c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0x4c).lw(c);
            c.sdl(2, sp(c).wrapping_add(7));
            c.sdr(2, sp(c));
            Addr(sp(c)).off(8).sw(c, c.g[A1][0]);
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3cc).lw(c);
            let no_total = (c.g[V1][0] as i64) <= 0;
            if !no_total {
                c.cycles += 7;
                c.lwc1(1, abi::r32(c, V0).wrapping_add(0x484));
                c.cvt_s_w(1, 1);
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.div_s(2, 1, 0);
            } else {
                c.cycles += 1;
                c.mtc1(0, 2);
            }
            c.cycles += 13;
            c.lwc1(0, gp(c, -0x54f8));
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(1, 1);
            c.mul_s(0, 2, 0);
            c.add_s(0, 0, 1);
            c.swc1(0, sp(c));
            c.ldl(2, sp(c).wrapping_add(7));
            c.ldr(2, sp(c));
            c.g[V1][0] = Addr(sp(c)).off(8).lw(c);
            c.sdl(2, abi::r32(c, A0).wrapping_add(0x4b));
            c.sdr(2, abi::r32(c, A0).wrapping_add(0x44));
            Addr(c.g[A0][0] as u32).off(0x4c).sw(c, c.g[V1][0]);
            c.lq(16, frame.at(c, 0x20));
        }

        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_EndPlayback(Replay *self)` (0x26f980): cancel instant replay,
/// clear the replay state and restore the appropriate camera.
pub fn replay_end_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        call_ptr(c, 0x0027_0628, 0x0026_f9b0)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x61c)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x620)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
            } else {
                c.cycles += 5;
                call_ptr(c, 0x0028_b180, 0x0026_f9d0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_f2c0, 0x0026_f9d8)?;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
            }
        }
        c.cycles += 4;
        Addr(p(c.g[S0][0], 0x61c)).sw(c, 0);
        Addr(p(c.g[S0][0], 0x620)).sw(c, 0);
        let some = c.g[V0][0] != 0;
        Addr(p(c.g[S0][0], 0x624)).sw(c, 0);
        if some {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0026_f850, 0x0026_f9f4)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x620)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_ef80, 0x0026_fa08)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x60c)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_eea0, 0x0026_fa18)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x60c)).lw(c);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0026_f4a0, 0x0026_fa3c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0026_f4a0, 0x0026_fa2c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// OVScreenReplayMenu_OnWidgetCreated (0x000020de30).

/// `OVScreenReplayMenu_OnWidgetCreated`: wire the replay menu widgets by name.
pub fn ov_screen_replay_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_20df00: {
            'b_20defc: {
                'b_20dec4: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A0][0] = 0x470000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bf0));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x0020de54)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_20dec4;
                    } else {
                        c.cycles += 7;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x608));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_20defc;
                        } else {
                            c.cycles += 19;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x0020de8c);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x0020de8c {
                                return Err(crate::call::Unwound);
                            }
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x0020dea4);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x0020dea4 {
                                return Err(crate::call::Unwound);
                            }
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x0020debc);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x0020debc {
                                return Err(crate::call::Unwound);
                            }
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_20df00;
                            } else {
                                break 'b_20dec4;
                            }
                        }
                    }
                }
                'b_20def8: {
                    'b_20dee0: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c08));
                        call_ptr(c, 0x00317670, 0x0020ded0)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x470000;
                            break 'b_20dee0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_20def8;
                            } else {
                                break 'b_20dee0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c20));
                    call_ptr(c, 0x00317670, 0x0020deec)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_20df00;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x2;
                        break 'b_20def8;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_20defc;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_20df00;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `void cOVState::onUpdate(self)` (0x20e900, `cOVStateOverlay_Update`):
/// pauses the game (`cGame_activatePauseMenu(SSXApp->Game, 1)`), then runs
/// the state's `cUIState::onUpdate(self)`.
pub fn ov_state_overlay_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x84);
        call_ptr(c, 0x0023_1840, 0x0020_e920)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x0020_e928)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) const UI_STATE_RENDER: u32 = 0x0039_e6b8;

// ---------------------------------------------------------------------------
// cOVStateOverlay_Render

/// `void cOVStateOverlay_Render(void)` (0x20e8e0): `UIState_Render()`, a
/// forwarding wrapper (7 cycles, 0x10 frame, tail call through the table).
pub fn cov_state_overlay_render(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(UI_STATE_RENDER), 0x0020_e8f0)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_6 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::str_hash_2 as str_hash;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;

    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD_2;

    use ssx3_ee as ee;

    #[test]
    fn ov_screen_replay_menu_update_progress_bar_matches() {
        check(
            ee::s05::f_0020e818__generated,
            ov_screen_replay_menu_update_progress_bar,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20e8, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let obj = DATA + 0x2000;
                c.bus.write32(this + 0x40, obj);
                for i in 0..3 {
                    c.bus.write32(obj + 0x44 + 4 * i, r.next_u32());
                }
                let replay = DATA + 0x2400;
                c.bus.write32(RMAN + 0x28, replay);
                c.bus.write32(
                    replay + 0x3cc,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 100) as u32
                    },
                );
                c.bus.write32(replay + 0x484, seed % 100);
                c.bus.write32(GAME_GP - 0x54f8, 315.0f32.to_bits());
            },
        );
    }

    #[test]
    fn replay_end_playback_matches() {
        check(
            ee::s07::f_0026f980__generated,
            replay_end_playback,
            40,
            &|c, seed| {
                let _r = base(c, seed, 0x26f9);
                c.g[A0][0] = DATA as u64;
                // 0xE/0xF return immediately; other states run the teardown.
                c.bus.write32(DATA, if seed % 2 == 0 { 0xe } else { 0x3 });
                c.bus.write32(DATA + 0x608, (seed % 2) as u32);
                c.bus.write32(DATA + 0x60c, (seed % 3) as u32);
                c.bus.write32(DATA + 0x61c, 0);
                c.bus.write32(DATA + 0x620, 0);
            },
        );
    }

    #[test]
    fn ov_screen_replay_menu_on_widget_created_matches() {
        diff(
            ee::s05::f_0020de30__generated,
            ov_screen_replay_menu_on_widget_created,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x20_de30);
                let widget = DATA + 0x2000;
                put_str(c, 0x0047_1bf0, b"replay");
                let h = str_hash(c, 0x0047_1bf0);
                w(
                    c,
                    widget + 0x38,
                    if seed % 3 == 0 { r.next_u32() } else { h },
                );
                w(c, widget + 8, 0);
                w(c, widget + 0x18, r.next_u32());
                let game = WORLD_2 + 0x1000;
                w(c, game + 0x28, DATA + 0x3000);
                w(
                    c,
                    DATA + 0x3000 + 0x608,
                    if seed % 2 == 0 { 0 } else { DATA + 0x3100 },
                );
                w(c, DATA + 0x3100, r.next_u32());
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    pub(crate) use crate::game::ui::uilistbox::tests::put;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The pause overlay: the game pointer through `gp-0x848`, then the
    /// two calls (both return to their `ra`).
    /// RAM word that arms the two fakes below (set only by this test's
    /// fixture, so other tests' machines still run the real callees).
    pub(crate) const OC711_FX_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC711_FX_MAGIC: u32 = 0x4f43_3711;

    /// Fake for `cGame_activatePauseMenu` (0x231840): logged when armed,
    /// otherwise the generated body.
    pub(crate) fn oc711_fake_pause(c: &mut Cpu) {
        if c.bus.read32(OC711_FX_CTRL) != OC711_FX_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc711_fake_pause),
                ee::s06::f_00231840__generated,
            );
        }
        fakes::fake_call(c, 0x0023_1840);
    }

    /// Fake for `cUIState_onUpdate` (0x39e510), same gate.
    pub(crate) fn oc711_fake_state_update(c: &mut Cpu) {
        if c.bus.read32(OC711_FX_CTRL) != OC711_FX_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc711_fake_state_update),
                ee::s13::f_0039e510__generated,
            );
        }
        fakes::fake_call(c, 0x0039_e510);
    }

    #[test]
    fn ov_state_overlay_update_matches() {
        // Both callees reach the real game code, which runs away on this
        // fixture; the armed fakes log the calls instead (the diff still
        // compares the arguments and the register dance).
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00231840),
            oc711_fake_pause,
            fakes::prev_cell(oc711_fake_pause),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039E510),
            oc711_fake_state_update,
            fakes::prev_cell(oc711_fake_state_update),
        );
        check(
            ee::s05::f_0020e900__generated,
            ov_state_overlay_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x0020_e900 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC711_FX_CTRL, OC711_FX_MAGIC);
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA + 0x500;
                let game = DATA + 0x600;
                put(c, GAME_GP.wrapping_sub(0x848), world);
                put(c, world + 0x84, game);
                put(c, game + 4, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 2) * 0x40);
            },
        );
    }

    /// Gate for the fakes below: they run only when the machine holds this
    /// word, so other tests reaching the same slots run the real bodies.
    pub(crate) const OC715_GATE: u32 = DATA + 0x200000;

    pub(crate) const OC715_MAGIC: u32 = 0x715c_0de5;

    pub(crate) static OC715_PREV_RENDER: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc715_fake_ui_state_render(c: &mut Cpu) {
        if c.bus.read32(OC715_GATE) != OC715_MAGIC {
            return fall_back(c, &OC715_PREV_RENDER, ee::s13::f_0039e6b8__generated);
        }
        fake_call(c, UI_STATE_RENDER);
    }

    #[test]
    fn cov_state_overlay_render_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E6B8),
            oc715_fake_ui_state_render,
            &OC715_PREV_RENDER,
        );
        check(
            ee::s05::f_0020e8e0__generated,
            cov_state_overlay_render,
            50,
            &|c, seed| {
                let mut r = Rng(0x0020_e8e0 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC715_GATE, OC715_MAGIC);
            },
        );
    }
}
