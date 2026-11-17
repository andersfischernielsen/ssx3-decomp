//! `src/be/beintcharacter.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_ef30,
        be_get_rider_char_db_weight,
        "BE_GetRiderCharDBWeight",
    ),
    (0x0014_efa8, be_get_rider_size_scale, "BE_GetRiderSizeScale"),
];

// ---------------------------------------------------------------------------
// BE_GetRiderCharDBWeight (0x14ef30)

/// `int8_t BE_GetRiderCharacter(int player)` (0x14a080).
pub(crate) const BE_GET_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// The character database: 0x88-byte records (`cBECharacterInterface`).
pub(crate) const CHAR_DB: u32 = 0x0053_0970;

pub(crate) const CHAR_DB_RECORD: u32 = 0x88;

/// Character record: the rider's weight class.
pub(crate) const CHAR_WEIGHT: u32 = 0x40;

/// `int BE_GetRiderCharDBWeight(void*, int riderIndex)` (0x14ef30,
/// `cBECharacterInterface::getWeight`): the weight word of the rider's
/// character-DB record. The record base is kept in s0 across the call.
pub fn be_get_rider_char_db_weight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(CHAR_DB);
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ef4c)?;
        c.g[V1][0] = CHAR_DB_RECORD as u64;
        frame.restore_ra(c, 0);
        let character = c.g[V0][0] as u32;
        let off = abi::mult(c, character, CHAR_DB_RECORD);
        c.g[V0][0] = sx(off);
        let record = CHAR_DB.wrapping_add(off);
        c.g[S0][0] = sx(record);
        c.g[V0][0] = Addr(record).off(CHAR_WEIGHT).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE_GetRiderSizeScale (0x14efa8)

pub(crate) const SIZE_SCALE_GET_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

pub(crate) const SIZE_SCALE_GET_ALT_CHARACTER: Callee = Callee {
    addr: 0x0014_a0b0,
    f: ee::s01::BE_GetRiderAltCharacter,
};

pub(crate) const SIZE_SCALE_GET_BE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};

pub(crate) const SIZE_SCALE_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};

pub(crate) const SIZE_SCALE_IS_SHARING: Callee = Callee {
    addr: 0x0014_77e8,
    f: ee::s01::BENewPlayerInt_IsSharingCharRecord,
};

/// `float BE_GetRiderSizeScale(void* self, int rider)` (0x14efa8): the
/// rider model's scale. A cheat character (the alt character, when the cheat
/// flag at 0x535c11 is set or for riders past the first) has a fixed scale
/// from the jump table at 0x45a660 (0.98, 0.85, 2, 0.7, 1.5, 1.2, 1.3, 0.75,
/// 1, else 0.9); a rider sharing a character record 0.8; otherwise the
/// character's size percentage (+0x48 of its 0x88-byte record at 0x530970)
/// * 0.01.
pub fn be_get_rider_size_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        SIZE_SCALE_GET_CHARACTER.call(c, 0x0014_efc8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        SIZE_SCALE_GET_ALT_CHARACTER.call(c, 0x0014_efd4)?;
        c.g[S1][0] = c.g[V0][0];
        SIZE_SCALE_GET_BE.call(c, 0x0014_efdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        SIZE_SCALE_GET_INTERFACE.call(c, 0x0014_efe8)?;
        let cheat = c.g[S1][0] != 0;
        c.g[V0][0] = 0x53_0000;
        let gp = c.g[GP][0] as u32;
        'exit: {
            if cheat {
                c.cycles += 3;
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let flag = c.g[V1][0] != 0;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xa));
                let applies = flag || {
                    c.cycles += 2;
                    c.g[S0][0] != 0
                };
                if applies {
                    // switch (cheat - 10) through the jump table.
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x14) as u64;
                    let in_table = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x46_0000;
                    let mut target = 0x0014_f0a4;
                    if in_table {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x59a0));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        target = c.g[A0][0] as u32;
                    }
                    // (cycles, scale: a .lit4 at gp-off, or an immediate via `at`)
                    let lit = |c: &mut Cpu, off: u32| {
                        c.cycles += 3;
                        c.lwc1(0, gp.wrapping_sub(off));
                    };
                    let imm = |c: &mut Cpu, bits: u64| {
                        c.cycles += 4;
                        c.g[AT][0] = bits;
                        c.mtc1(1, 0);
                    };
                    match target {
                        0x0014_f028 => lit(c, 0x6e0c),
                        0x0014_f034 => lit(c, 0x6e08),
                        0x0014_f040 => imm(c, 0x4000_0000),
                        0x0014_f050 => lit(c, 0x6e04),
                        0x0014_f05c => imm(c, 0x3fc0_0000),
                        0x0014_f06c => lit(c, 0x6e00),
                        0x0014_f078 => lit(c, 0x6dfc),
                        0x0014_f084 => imm(c, 0x3f40_0000),
                        0x0014_f094 => imm(c, 0x3f80_0000),
                        0x0014_f0a4 => lit(c, 0x6df8),
                        t => {
                            c.jump_out(t, 0x0014_efa8, 0x0014_f100);
                            return Ok(());
                        }
                    }
                    frame.restore(c, S0, 0x30);
                    break 'exit;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            SIZE_SCALE_IS_SHARING.call(c, 0x0014_f0b8)?;
            let sharing = c.g[V0][0] != 0;
            c.g[V1][0] = 0x88;
            if sharing {
                c.cycles += 1;
                c.lwc1(0, gp.wrapping_sub(0x6df0));
            } else {
                c.cycles += 9;
                c.g[V0][0] = 0x53_0000;
                c.mult(0, V1, S2, V1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x970));
                c.lwc1(1, gp.wrapping_sub(0x6df4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
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

    /// The race-slot table and the per-slot rider records
    /// `BE_GetRiderCharacter` reads.
    pub(crate) const PLAYER_SLOTS: u32 = 0x0053_05b0;

    pub(crate) const RIDER_RECORDS: u32 = 0x0053_5b20;

    /// BE_GetRiderCharDBWeight: a player's slot, its record's character
    /// byte, and the weight word of that character's DB record.
    #[test]
    fn be_get_rider_char_db_weight_matches() {
        check(
            ee::s01::f_0014ef30__generated,
            be_get_rider_char_db_weight,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_ef30, 0);
                c.pc = 0x0014_ef30;
                let player = seed % 4;
                let slot = r.next_u32() % 8;
                let ch = r.next_u32() % 16;
                c.g[A1][0] = player as u64;
                c.bus.write32(PLAYER_SLOTS + 4 * player, slot);
                c.bus.write8(RIDER_RECORDS + slot * 0x1c + 0x11, ch as u8);
                c.bus
                    .write32(CHAR_DB + ch * CHAR_DB_RECORD + CHAR_WEIGHT, r.next_u32());
            },
        );
    }

    use crate::difftest::put_words;

    use crate::difftest::fakes::covered;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::OC396_CTRL;
    pub(crate) use crate::game::ai::rider::tests::OC396_MAGIC;
    pub(crate) use crate::game::ai::rider::tests::OC396_RET;

    pub(crate) use crate::game::ai::ai::tests::oc396_install_be_fakes;

    /// Every cheat character (and ids past the table), with and without the
    /// cheat flag, for the first and later riders; shared records; the size
    /// table.
    #[test]
    fn be_get_rider_size_scale_matches() {
        oc396_install_be_fakes();
        covered(
            ee::s01::f_0014efa8__generated,
            be_get_rider_size_scale,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_efa8, 5);
                c.pc = 0x0014_efa8;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let ch = r.next_u32() % 12;
                let cheat = if seed % 4 == 0 { 0 } else { 8 + seed / 4 % 26 };
                put_words(
                    c,
                    OC396_RET,
                    &[ch, cheat, DATA + 0x500, 0, r.next_u32() % 2],
                );
                put_words(
                    c,
                    0x0045_a660,
                    &[
                        0x14f028, 0x14f034, 0x14f0a4, 0x14f094, 0x14f034, 0x14f084, 0x14f0a4,
                        0x14f084, 0x14f094, 0x14f094, 0x14f06c, 0x14f094, 0x14f078, 0x14f078,
                        0x14f0a4, 0x14f0a4, 0x14f05c, 0x14f0a4, 0x14f050, 0x14f040,
                    ],
                );
                for (k, v) in [0.98f32, 0.85, 0.7, 1.2, 1.3, 0.9, 0.01, 0.8]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write32(GAME_GP - 0x6e0c + 4 * k as u32, v.to_bits());
                }
                c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
                c.bus
                    .write32(0x0053_0970 + 0x88 * ch + 0x48, 80 + r.next_u32() % 40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
            },
        );
    }
}
