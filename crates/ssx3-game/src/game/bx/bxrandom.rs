//! `src/bx/bxrandom.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_77f0, rand_next_main, "Rand_NextMain"),
    (0x0031_7810, rand_next_game, "Rand_NextGame"),
    (0x0031_78e0, rand_save_game_state, "Rand_SaveGameState"),
    (0x0031_7830, rand_float_range, "Rand_FloatRange"),
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

// ---------------------------------------------------------------------------
// Rand_SaveGameState (0x3178e0)

/// The game RNG's 0x18-byte state block.
pub(crate) const RAND_STATE: u32 = 0x004f_f030;

pub(crate) const RAND_STATE_BYTES: u64 = 0x18;

/// `EA_MemCopy(dst, src, n)` (0x3e6574, cross-shard).
pub(crate) const RAND_MEM_COPY: u32 = 0x003e_6574;

/// `void Rand_SaveGameState(void* dst)` (0x3178e0): copy the game RNG's
/// 0x18-byte state (`0x4ff030`) into `dst`. Frame 0x10.
pub fn rand_save_game_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        // `a1 = 0x500000 - 0xfd0`.
        c.g[A1][0] = RAND_STATE as u64;
        frame.save_ra(c, 0);
        c.g[A2][0] = RAND_STATE_BYTES;
        call_ptr(c, RAND_MEM_COPY, 0x0031_78f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_FloatRange (0x317830)

/// `Rand_NextGame` (0x317810).
pub const OC331_RAND_NEXT_GAME: Callee = Callee {
    addr: 0x0031_7810,
    f: ee::s10::Rand_NextGame,
};

/// `float Rand_FloatRange(float min, float max)` (0x317830): a uniform float
/// in `[min, max)`, from the game RNG's mantissa bits (`1.0 + m/2^23`).
pub fn rand_float_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        c.mov_s(21, 12); // f21 = min
        frame.save_ra(c, 0);
        c.mov_s(20, 13); // f20 = max
        OC331_RAND_NEXT_GAME.call(c, 0x0031_784c)?;
        c.g[V1][0] = 0x7f_0000;
        c.g[A0][0] = 0x3f80_0000;
        c.g[V1][0] |= 0xffff;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0); // f0 = 1.0f
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= c.g[A0][0]; // a float in [1, 2)
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.mtc1(2, 1); // f1 = that float
        let unit = FpuF32(c.f[1]) - FpuF32(c.f[0]); // x - 1.0 in [0, 1)
        c.f[20] = (FpuF32(c.f[20]) - FpuF32(c.f[21])).0; // max - min
        c.f[0] = unit.0;
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[21]) + FpuF32(c.f[20])).0;
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
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

    #[test]
    fn rand_save_game_state_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ssx3_ee::s10::f_003178e0__generated,
            rand_save_game_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_78e0, 0);
                // The RNG state and a junk destination (sometimes unaligned,
                // which takes EA_MemCopy's byte head).
                fill(c, &mut r, RAND_STATE, 6);
                let dst = DATA + 0x100 + (seed as u32 % 4);
                fill(c, &mut r, dst & !3, 8);
                c.g[A0][0] = dst as u64;
            },
        );
    }

    /// `Rand_FloatRange`: the real `Rand_NextGame` on random generator
    /// states, an ordinary and an edge-case range.
    #[test]
    fn rand_float_range_matches() {
        use crate::game::bx::bxrandom::RAND_GAME_STATE;
        check(
            ee::s10::f_00317830__generated,
            rand_float_range,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x317830, 0);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x0031_7830;
                let (lo, hi) = match seed % 4 {
                    0 => (r.f32(10.0), r.f32(10.0)),
                    1 => (r.edge_f32(), r.edge_f32()),
                    2 => (0x3f80_0000, 0x4000_0000), // 1.0 .. 2.0
                    _ => (r.f32(1e6), r.f32(1e6)),
                };
                c.f[12] = lo;
                c.f[13] = hi;
                fill(c, &mut r, RAND_GAME_STATE, 8);
            },
        );
    }
}
