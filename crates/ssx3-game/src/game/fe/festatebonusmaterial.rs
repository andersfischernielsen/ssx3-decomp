//! `src/fe/festatebonusmaterial.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::at;
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
}
