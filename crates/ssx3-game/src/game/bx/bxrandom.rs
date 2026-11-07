//! `src/bx/bxrandom.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::set_reg;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_77f0, rand_next_main, "Rand_NextMain"),
    (0x0031_7810, rand_next_game, "Rand_NextGame"),
];

// ---------------------------------------------------------------------------
// Rand_NextMain (0x3177f0)

/// The game's main random generator state (`0x500000 - 0xfe8`).
pub(crate) const RAND_MAIN: u32 = 0x004f_f018;

pub(crate) const RAND_NEXT: Callee = Callee {
    addr: 0x0031_7a08,
    f: ee::s10::Rand_Next,
};

/// `unsigned BXrand(void)` (0x3177f0): the next number of the main random
/// generator (`Rand_Next(&main_rand)`).
pub fn rand_next_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        set_reg(c, A0, 0x50_0000);
        frame.save_ra(c, 0);
        set_reg(c, A0, sx(RAND_MAIN));
        RAND_NEXT.call(c, 0x0031_7804)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_NextGame (0x317810)

/// The game's random generator state (0x4ff030).
pub(crate) const RAND_GAME_STATE: u32 = 0x0050_0000 - 0xfd0;

pub(crate) const RAND_NEXT_2: Callee = Callee {
    addr: 0x0031_7a08,
    f: ee::s10::f_00317a08,
};

/// `u32 Rand_NextGame(void)` (0x317810): the next number of the game's
/// random generator (`Rand_Next(&g_GameRand)`).
pub fn rand_next_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(RAND_GAME_STATE);
        RAND_NEXT_2.call(c, 0x0031_7824)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::Rng;

    /// `Rand_NextMain`: random generator state words (indices and table
    /// entries); the real `Rand_Next` runs.
    #[test]
    fn rand_next_main_matches() {
        crate::difftest::check(
            ee::s10::f_003177f0__generated,
            rand_next_main,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3177f0, 0);
                c.pc = 0x0031_77f0;
                fill(c, &mut r, RAND_MAIN, 0x10);
            },
        );
    }

    use crate::difftest::{check, scramble};

    /// Random generator state (all six words random, so both carry paths
    /// of `Rand_Next` are taken).
    #[test]
    fn rand_next_game_matches() {
        check(
            ee::s10::f_00317810__generated,
            rand_next_game,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_7810 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0031_7810;
                fill(c, &mut r, RAND_GAME_STATE, 8);
            },
        );
    }
}
