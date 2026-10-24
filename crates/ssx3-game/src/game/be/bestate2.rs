//! `src/be/bestate2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintmission::event_check;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
pub(crate) use crate::game::be::belibrary::EA_MEM_COPY;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_64b0, be_post_race_exit, "BEPostRace_Exit"),
    (0x0015_6510, be_post_race_enter, "BEPostRace_Enter"),
    (0x0015_66e8, be_pre_race_exit, "BEPreRace_Exit"),
    (0x0015_6750, be_pre_race_enter, "BEPreRace_Enter"),
    (
        0x0015_67b8,
        be_reset_profile_characters,
        "BE_ResetProfileCharacters",
    ),
    (
        0x0015_6810,
        be_reset_profile_rivalries,
        "BE_ResetProfileRivalries",
    ),
    (0x0015_6950, be_backup_character, "BE_BackupCharacter"),
    (0x0015_6988, be_restore_character, "BE_RestoreCharacter"),
];

pub(crate) const F_00151600: Callee = Callee {
    addr: 0x0015_1600,
    f: ssx3_ee::s01::f_00151600,
};

pub(crate) const F_001519E0: Callee = Callee {
    addr: 0x0015_19e0,
    f: ssx3_ee::s01::f_001519e0,
};

/// `void BE_ResetProfileCharacters(void *self, int arg)` (0x1567b8): reset the
/// ten characters (`f_00151600(self + i*0xf88, i, arg)`).
pub fn be_reset_profile_characters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            F_00151600.call(c, 0x0015_67ec)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_67d8);
                continue;
            } else {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_ResetProfileRivalries(void *self)` (0x156810): reset the ten
/// rivalries (`f_001519e0(self + i*0xf88, i)`).
pub fn be_reset_profile_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            F_001519E0.call(c, 0x0015_6834)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_6828);
                continue;
            } else {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_BackupCharacter(int self, int rider)` (0x156950): copy the rider's
/// 0xf88-byte character record to 0x4c4090 and remember the rider index.
pub fn be_backup_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        c.sb((c.g[GP][0] as u32).wrapping_sub(0x1e94), c.g[A1][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        call_ptr(c, EA_MEM_COPY, 0x0015_697c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_RestoreCharacter(int self)` (0x156988): copy 0x4c4090 back to the
/// remembered rider's character record.
pub fn be_restore_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_sub(0x1e94));
        c.g[V1][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x4c0000;
        c.mult(0, A3, V0, V1);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[A1][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, EA_MEM_COPY, 0x0015_69b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `base + off` as an effective address (`addu` of the low 32 bits).
#[inline(always)]
pub(crate) fn ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp_rel(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw rt, gp+off`.
#[inline(always)]
pub(crate) fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    c.lw(gp_rel(c, off))
}

// ---------------------------------------------------------------------------
// Pre/post-race visibility flags.

/// `void BEPostRace_Exit()` (0x1564B0) and friends: clear (or set) the
/// `visible` word at `+8` of a run of `BE*` singletons. The loads are
/// software-pipelined, one register ahead of the stores.
pub(crate) fn be_race_flags(c: &mut Cpu, value: u64, loads: &[(usize, i32)]) {
    c.g[loads[0].0][0] = gp_lw(c, loads[0].1);
    for k in 1..loads.len() {
        c.g[loads[k].0][0] = gp_lw(c, loads[k].1);
        let prev = loads[k - 1].0;
        c.sw(ea(c.g[prev][0], 8), value);
    }
    let last = loads[loads.len() - 1].0;
    c.sw(ea(c.g[last][0], 8), value);
}

pub fn be_post_race_exit(c: &mut Cpu) {
    c.cycles += 23;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f30),
            (V1, -0x1ee8),
            (A0, -0x1f2c),
            (V0, -0x1ee4),
            (V1, -0x1ed4),
            (A0, -0x1eec),
            (V0, -0x1f28),
            (V1, -0x1ee0),
            (A0, -0x1edc),
            (A1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_post_race_enter(c: &mut Cpu) {
    c.cycles += 24;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (A1, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (A1, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (A2, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_exit(c: &mut Cpu) {
    c.cycles += 25;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f3c),
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (V0, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (V0, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (V1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_enter(c: &mut Cpu) {
    c.cycles += 26;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f3c),
            (A0, -0x1f30),
            (A1, -0x1ee8),
            (V1, -0x1f2c),
            (A0, -0x1ee4),
            (A1, -0x1ed4),
            (V1, -0x1eec),
            (A0, -0x1f28),
            (A1, -0x1ee0),
            (A2, -0x1edc),
            (A0, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn be_reset_profile_matches() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_001567b8 as fn(&mut Cpu),
                be_reset_profile_characters as fn(&mut Cpu),
                0x67b8,
            ),
            (ssx3_ee::s01::f_00156810, be_reset_profile_rivalries, 0x6810),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x8000);
            });
        }
    }

    #[test]
    fn be_backup_restore_character_matches() {
        check(
            ssx3_ee::s01::f_00156950,
            be_backup_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6950, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, DATA, 0x4000);
            },
        );
        check(
            ssx3_ee::s01::f_00156988,
            be_restore_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6988, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write8(GAME_GP.wrapping_sub(0x1e94), (seed % 8) as u8);
                fill(c, &mut r, DATA, 0x4000);
            },
        );
    }

    pub(crate) const SEEDS: u32 = 40;

    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary singleton (`gp - 0x1F04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to build the whole BE
    /// singleton graph (and allocate from a zeroed heap).
    pub(crate) fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    /// A run of distinct `BE*` singletons for the visibility-flag functions.
    pub(crate) fn race_singletons(c: &mut Cpu) {
        const OFFSETS: [i32; 12] = [
            -0x1f3c, -0x1f30, -0x1f2c, -0x1ee8, -0x1ee4, -0x1ed4, -0x1eec, -0x1f28, -0x1ee0,
            -0x1edc, -0x1ea4, -0x1e8c,
        ];
        for (i, off) in OFFSETS.into_iter().enumerate() {
            w(
                c,
                (GAME_GP as i32 + off) as u32,
                DATA + 0x100 + 0x40 * i as u32,
            );
        }
    }

    // ------------------------------------------------------- race flow flags

    #[test]
    fn be_post_race_exit_matches() {
        check(
            ee::s01::f_001564b0__generated,
            be_post_race_exit,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_64b0, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_post_race_enter_matches() {
        check(
            ee::s01::f_00156510__generated,
            be_post_race_enter,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_6510, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_pre_race_exit_matches() {
        check(
            ee::s01::f_001566e8__generated,
            be_pre_race_exit,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_66e8, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_pre_race_enter_matches() {
        check(
            ee::s01::f_00156750__generated,
            be_pre_race_enter,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_6750, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }
}
