//! `src/fe/festatebonusmaterial.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::fe_ctor_with_vtable;
pub(crate) use crate::game::fe::feasyncfile::fe_dtor_with_vtable;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_54d8,
        c_fe_state_bonus_material_init,
        "cFEStateBonusMaterial_Init",
    ),
    (
        0x0019_5540,
        c_fe_state_bonus_material_on_widget_created,
        "cFEStateBonusMaterial_OnWidgetCreated",
    ),
    (
        0x0019_55b8,
        fe_state_bonus_material_on_widget_notify,
        "cFEStateBonusMaterial_OnWidgetNotify",
    ),
    (
        0x001d_22f0,
        c_fe_state_video_player_ctor,
        "cFEStateVideoPlayer_ctor",
    ),
    (
        0x001d_2598,
        cfe_state_video_player_set_movie,
        "cFEStateVideoPlayer_SetMovie",
    ),
    (
        0x001d_e750,
        c_fe_state_bonus_material_ctor_2,
        "cFEStateBonusMaterial_ctor_2",
    ),
    (
        0x001d_e788,
        c_fe_state_bonus_material_dtor,
        "cFEStateBonusMaterial_dtor",
    ),
    (
        0x0019_55e0,
        bonus_material_on_activate,
        "cFEStateBonusMaterial_OnActivate",
    ),
    (
        0x0019_5600,
        bonus_material_on_widget_action,
        "cFEStateBonusMaterial_OnWidgetAction",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::STRCPY_2 as STRCPY;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;

    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
}

// ---------------------------------------------------------------------------
// Video player.

/// `void cFEStateVideoPlayer_SetMovie(s, const char* name, const char* movie)`
/// (0x1d2598): copy `name` into +0x48 and, when non-null, `movie` into +0x148
/// (else clear the first byte there). 10 (+4) + 2 (+4) cycles plus the copies.
pub fn cfe_state_video_player_set_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(self_, 0x48));
        call_ptr(c, callee::STRCPY, 0x001d_25b8)?;
        let movie = c.g[S1][0];
        c.g[A0][0] = sx(p(self_, 0x148));
        if movie == 0 {
            c.cycles += 2;
            Addr(p(self_, 0x148)).sb(c, 0);
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 4;
            c.g[A1][0] = movie;
            call_ptr(c, callee::STRCPY, 0x001d_25c8)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The front-end widget-notify results.
pub(crate) const WIDGET_IGNORED: u64 = 0x101;

pub(crate) const WIDGET_ACTIVATED: u64 = 0x100;

/// The two-key widget predicates (0x1F3CC8, 0x1F43D8, 0x1955B8): ids at or
/// above the second threshold are ignored (0x101); below the first threshold
/// also 0x101; in between answers 0x100.
pub(crate) fn widget_range_notify(c: &mut Cpu, low: u64) {
    c.cycles += 3;
    c.g[V1][0] = (c.g[A2][0] < 0xa) as u64;
    let above = c.g[V1][0] == 0;
    c.g[V0][0] = WIDGET_IGNORED;
    if !above {
        c.cycles += 4;
        c.g[A0][0] = (c.g[A2][0] < low) as u64;
        c.g[V1][0] = WIDGET_IGNORED;
        c.g[V0][0] = WIDGET_ACTIVATED;
        if c.g[A0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateBonusMaterial_OnWidgetNotify(...)` (0x1955B8): ids 8..9 answer
/// 0x100.
pub fn fe_state_bonus_material_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}

/// `cFEStateBonusMaterial_Init(this)` 0x1954d8: add the bonus-material screen
/// and play its first frame.
pub fn c_fe_state_bonus_material_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_0168);
        c.g[S1][0] = lw_at(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x0019_54fc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_5510)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_5528)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBonusMaterial_OnWidgetCreated(self, menu)` (0x195540): map
/// the created menu's name hash to the bonus-material sub-page index.
pub fn c_fe_state_bonus_material_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[A0][0] = 0x4a_0000;
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x14f8));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = lw_r(c, S1, 0x38);
        call_ptr(c, callee::STR_HASH, 0x0019_5564)?;
        if c.g[S0][0] == c.g[V0][0] {
            sw_r(c, S1, 0x18, 0);
        }
        c.cycles += 6;
        c.g[A0][0] = 0x4a_0000;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1500));
        call_ptr(c, callee::STR_HASH, 0x0019_557c)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a_0000;
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            sw_r(c, S1, 0x18, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1508));
        call_ptr(c, callee::STR_HASH, 0x0019_5598)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.lq(16, sp_2(c).wrapping_add(0x20));
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            sw_r(c, S1, 0x18, c.g[V0][0]);
        }
        c.cycles += 4;
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void* cFEStateVideoPlayer_ctor(self, engine, owner, signed char player)`
/// (0x1d22f0): build the video player state's UI object and two strings.
pub fn c_fe_state_video_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let engine = abi::arg64(c, 1);
        let owner = abi::arg64(c, 2);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(self_);
        c.g[S1][0] = sx(r32(c, A3) << 24);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((r32(c, S1) as i32) >> 24) as u32);
        c.g[S2][0] = sx(p(self_, 0x48));
        c.g[A1][0] = engine;
        c.g[A2][0] = owner;
        call_ptr(c, callee::UI_OBJECT_CTOR_2, 0x001d_2318)?;
        c.g[V0][0] = 0x0047_0000;
        c.sb(at(c, S0, 0x44), c.g[S1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x66d8));
        c.g[A1][0] = 6;
        c.g[V1][0] = 1;
        c.sb(at(c, S0, 0x48), 0);
        c.sb(at(c, S0, 0x148), 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x24c));
        c.sw(at(c, S0, 0x274), 0);
        c.g[S1][0] = sx(p(r32(c, S0), 0x148));
        c.sw(at(c, S0, 0x278), 0);
        c.sw(at(c, S0, 0x27c), 0);
        c.sw(at(c, S0, 0x248), 0);
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.sw(at(c, S0, 0xc), c.g[A1][0]);
        c.sw(at(c, S0, 0x270), c.g[V1][0]);
        call_ptr(c, callee::UI_STRING_CTOR, 0x001d_235c)?;
        c.sw(at(c, S0, 0x24c), c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(at(c, S0, 0x250), c.g[S1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIState_ctor` (0x39e2a0), reached through the function table.
pub(crate) const FE_UI_STATE_CTOR: u32 = 0x0039_e2a0;

/// `UIState_dtor` (0x39e390), reached through the function table.
pub(crate) const FE_UI_STATE_DTOR: u32 = 0x0039_e390;

/// `cFEStateBonusMaterial` constructor (0x1de750): `UIState_ctor(this, 0)`,
/// vtable at `+8`.
pub fn c_fe_state_bonus_material_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e768,
            0x46e8,
        )
    });
}

/// `cFEStateBonusMaterial` destructor (0x1de788): vtable at `+8`, then
/// `UIState_dtor(this)`.
pub fn c_fe_state_bonus_material_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(c, 10, Target::Table(FE_UI_STATE_DTOR), 0x2f30, 0x001d_e7a4)
    });
}

/// `void cFEStateBonusMaterial_onGainFocus(self)` (0x1955e0): forwards to
/// `UIState_OnActivate` (0x39e4c0).
pub fn bonus_material_on_activate(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x0019_55f0);
}

/// `void cFEStateBonusMaterial_onWidgetAction(self, item, int event)`
/// (0x195600): with an item, event 5 opens the video player popup for the
/// item's clip (`cFEStateVideoPlayer` ctor 0x1d22f0, source files 0x1d2598,
/// push popup 0x39f290); event 6 asks the engine's custom interface for the
/// next state and pushes it (0x39f400).
pub fn bonus_material_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let no_item = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        // 0: leave through the epilogue after `lq s0`, 1: with the extra cycle.
        let mut extra_cycle = false;
        if no_item {
            extra_cycle = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let is_5 = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A2][0] < 6) as u64;
            if is_5 {
                oc851_open_video_popup(c)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    frame.restore(c, 16, 0x20);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 6;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
                        c.g[A3][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A3).wrapping_add(4)).lw(c);
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
                        let t = r32(c, V0);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        call_ptr(c, t, 0x0019_56d0)?;
                        let none = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[V0][0];
                        if !none {
                            c.cycles += 3;
                            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f400, 0x0019_56e4)?;
                        }
                        extra_cycle = true;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 16, 0x20);
                    }
                }
            }
        }
        if extra_cycle {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Event 5 of `bonus_material_on_widget_action`: build a `cFEStateVideoPlayer`
/// popup for the clip table entry of the item (`0x440000 + 0x1128`, indexed by
/// the item's +0x18) and push it. Ends with `lq s0` (the caller's epilogue).
pub(crate) fn oc851_open_video_popup(c: &mut Cpu) -> Flow {
    c.cycles += 27;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = 0x280;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x0019_565c)?;
    c.g[A3][0] = Addr(r32(c, S1).wrapping_add(0x44)).lb(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x001d_22f0, 0x0019_5670)?;
    c.g[A3][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
    c.g[V1][0] = 0x0044_0000;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1128));
    c.g[S0][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(r32(c, A3)).lw(c);
    call_ptr(c, 0x001d_2598, 0x0019_5698)?;
    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f290, 0x0019_56a8)?;
    let frame = Frame { size: 0x30 };
    frame.restore(c, 16, 0x20);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::init_screen_test;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2;
    use ssx3_ee as ee;

    // -------------------------------------------------------- video player

    #[test]
    fn cfe_state_video_player_set_movie_matches() {
        check(
            ee::s04::f_001d2598__generated,
            cfe_state_video_player_set_movie,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d25);
                let s = DATA;
                let name = DATA + 0x200;
                let movie = DATA + 0x300;
                for (i, base_at) in [name, movie].into_iter().enumerate() {
                    let n = (r.next_u32() % 16) as usize;
                    for k in 0..n {
                        c.bus.write8(
                            base_at + k as u32,
                            b'a' + ((seed + i as u32 + k as u32) % 26) as u8,
                        );
                    }
                    c.bus.write8(base_at + n as u32, 0);
                }
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(name);
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(movie) };
            },
        );
    }

    #[test]
    fn fe_state_bonus_material_on_widget_notify_matches() {
        check(
            ee::s02::f_001955b8__generated,
            fe_state_bonus_material_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0019_55b8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_bonus_material_init_matches() {
        init_screen_test(
            ee::s02::f_001954d8__generated,
            c_fe_state_bonus_material_init,
            0x1954d8,
        );
    }

    /// Run a guest helper during setup; returns its `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `StrHash` (0x317670).
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    #[test]
    fn c_fe_state_bonus_material_on_widget_created_matches() {
        check(
            ee::s02::f_00195540__generated,
            c_fe_state_bonus_material_on_widget_created,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x195540, 3);
                be_library(c);
                let h0 = str_hash(c, 0x004a_14f8);
                let h1 = str_hash(c, 0x004a_1500);
                let h2 = str_hash(c, 0x004a_1508);
                let menu = DATA + 0x800;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = menu as u64;
                let hash = match seed % 4 {
                    0 => h0,
                    1 => h1,
                    2 => h2,
                    _ => r.next_u32(),
                };
                w(c, menu + 0x38, hash);
                w(c, menu + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_ctor_matches() {
        check(
            ee::s04::f_001d22f0__generated,
            c_fe_state_video_player_ctor,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x1d22, 0);
                env(c);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = (-(seed as i32 % 100) as u64) & 0xff;
            },
        );
    }

    /// A UI-state constructor or destructor: the base helpers are real ports
    /// (`UIState_ctor`, `UIState_dtor`), so the plain machine is enough.
    pub(crate) fn ui_state_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn c_fe_state_bonus_material_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de750__generated,
            c_fe_state_bonus_material_ctor_2,
            0x001d_e750,
        );
    }

    #[test]
    fn c_fe_state_bonus_material_dtor_matches() {
        ui_state_case(
            ee::s04::f_001de788__generated,
            c_fe_state_bonus_material_dtor,
            0x001d_e788,
        );
    }

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::fe::festatebigradio::tests::oc851_machine;

    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_IFACE;

    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_OBJ2;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_SELF;

    pub(crate) const OC851_IFACE_VT: u32 = DATA + 0x8_6800;

    /// `leaf` returning 0 (a real function with no callees).
    pub(crate) const OC851_ZERO_LEAF: u32 = 0x0014_06c0;

    /// `cFEStateBonusMaterial_OnActivate`: forwards to `UIState_OnActivate`.
    #[test]
    fn bonus_material_on_activate_matches() {
        check(
            ee::s02::f_001955e0__generated,
            bonus_material_on_activate,
            40,
            &|c, seed| {
                let _ = oc851_machine(c, seed, 0x1955e0, 1);
                c.g[A0][0] = sx(OC851_SELF);
                c.pc = 0x0019_55e0;
            },
        );
    }

    /// `cFEStateBonusMaterial_OnWidgetAction`: no item, events 4 and 7 (nothing),
    /// 5 (video popup), 6 (next state, with and without an answer).
    #[test]
    fn bonus_material_on_widget_action_matches() {
        check(
            ee::s02::f_00195600__generated,
            bonus_material_on_widget_action,
            240,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x195600, 4);
                c.bus.write32(OC851_SELF + 0x10, OC851_IFACE);
                c.bus.write8(OC851_SELF + 0x44, r.next_u32() as u8);
                // The engine's custom interface: slot 0x28/0x2c of the table at +4.
                c.bus.write32(OC851_IFACE, OC851_IFACE);
                c.bus.write32(OC851_IFACE + 4, OC851_IFACE_VT);
                c.bus.write16(OC851_IFACE_VT + 0x28, 0);
                let target = if seed % 3 == 0 { OC851_ZERO_LEAF } else { FAKE };
                c.bus.write32(OC851_IFACE_VT + 0x2c, target);
                c.bus.write32(OC851_OBJ2 + 0x18, r.next_u32() % 4);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { OC851_OBJ2 as u64 };
                c.g[A2][0] = [5u64, 6, 6, 4, 7, 0x10, 5][seed as usize % 7];
                c.pc = 0x0019_5600;
            },
        );
    }
}
