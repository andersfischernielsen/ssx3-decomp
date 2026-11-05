//! `src/be/beintcharacter.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0014_ef30,
    be_get_rider_char_db_weight,
    "BE_GetRiderCharDBWeight",
)];

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

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
}
