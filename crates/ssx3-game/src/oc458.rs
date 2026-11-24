//! Staging module oc458 (wave wave225): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc458`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_f170, be_char_get_age, "BE_GetRiderCharDBField30"),
    (0x0014_f1b0, be_char_get_height, "BE_GetRiderCharDBField34"),
    (0x0014_f1e0, be_char_get_nationality, "BE_GetRiderCharDBField44"),
    (0x0014_f210, be_char_get_voice_id, "BE_GetRiderCharDBField54"),
    (
        0x0015_2368,
        be_test_char_profile_bolt_flags,
        "BE_TestCharProfileBoltFlags",
    ),
    (0x0015_2430, be_set_char_profile_flag, "BE_SetCharProfileFlag"),
    (
        0x0015_33f8,
        be_mission_int_get_peak_collected_count,
        "BEMissionInt_GetPeakCollectedCount",
    ),
];

/// `BE_GetRiderCharacter(int player)` (0x14a080).
const CHAR_GETTER_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// `BE_GetRiderProfile(int player)` (0x14a0e0).
const PEAK_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::BE_GetRiderProfile,
};

/// The character database: 0x88-byte records (`cBECharacterInterface`).
const CHAR_GETTER_DB: u32 = 0x0053_0970;

/// Size of one character record.
const CHAR_GETTER_RECORD: u32 = 0x88;

/// The shared body of `getAge` and `getVoiceID` (0x14f170, 0x14f210): the
/// word at `field` of the rider's character record. `BE_GetRiderCharacter(a1)`
/// selects the record; its pointer is kept in s0 across the call (0x20-byte
/// frame, s0 saved at +0x10, ra at +0).
fn char_getter_record_word(c: &mut Cpu, call_ret: u32, field: u32) -> Flow {
    c.cycles += 15;
    let frame = Frame::open(c, 0x20);
    c.g[A0][0] = c.g[A1][0];
    frame.save(c, S0, 0x10);
    c.g[S0][0] = sx(CHAR_GETTER_DB);
    frame.save_ra(c, 0);
    CHAR_GETTER_RIDER_CHARACTER.call(c, call_ret)?;
    c.g[V1][0] = CHAR_GETTER_RECORD as u64;
    frame.restore_ra(c, 0);
    let character = c.g[V0][0] as u32;
    let off = abi::mult(c, character, CHAR_GETTER_RECORD);
    c.g[V0][0] = sx(off);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(field).lw(c);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// The shared body of `getHeight` and `getNationality` (0x14f1b0, 0x14f1e0):
/// the address `0x530000 + table + character * 0x88` (a pointer, no memory
/// read). The frame holds only ra.
fn char_getter_table_address(c: &mut Cpu, call_ret: u32, table: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    CHAR_GETTER_RIDER_CHARACTER.call(c, call_ret)?;
    c.g[A0][0] = CHAR_GETTER_RECORD as u64;
    let entry = 0x0053_0000u32.wrapping_add(table);
    c.g[V1][0] = sx(entry);
    let character = c.g[V0][0] as u32;
    let off = abi::mult(c, character, CHAR_GETTER_RECORD);
    c.g[V0][0] = sx(off);
    frame.restore_ra(c, 0);
    c.g[V0][0] = sx(off.wrapping_add(entry));
    frame.close(c);
    Ok(())
}

/// `cBECharacterInterface::getAge` (0x14f170): the rider character's age word
/// (record +0x60).
pub fn be_char_get_age(c: &mut Cpu) {
    guest(c, |c| char_getter_record_word(c, 0x0014_f18c, 0x60));
}

/// `cBECharacterInterface::getHeight` (0x14f1b0): the address of the rider
/// character's height entry in the table at 0x5309d4.
pub fn be_char_get_height(c: &mut Cpu) {
    guest(c, |c| char_getter_table_address(c, 0x0014_f1c0, 0x9d4));
}

/// `cBECharacterInterface::getNationality` (0x14f1e0): the address of the rider
/// character's nationality entry in the table at 0x5309e4.
pub fn be_char_get_nationality(c: &mut Cpu) {
    guest(c, |c| char_getter_table_address(c, 0x0014_f1f0, 0x9e4));
}

/// `cBECharacterInterface::getVoiceID` (0x14f210): the rider character's voice
/// id word (record +0x84).
pub fn be_char_get_voice_id(c: &mut Cpu) {
    guest(c, |c| char_getter_record_word(c, 0x0014_f22c, 0x84));
}

/// `int BE_TestCharProfileBoltFlags(self, id, mask)` (0x152368): whether every
/// bit of `mask` is set in the 16-bit flags of bolt map entry `id`. The map
/// (at self+0x288) gives a halfword index per id, the flags sit at
/// `self + 4 * index + 0x292`. Returns 1 when `(~flags & mask) == 0`.
pub fn be_test_char_profile_bolt_flags(c: &mut Cpu) {
    c.cycles += 11;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(this).off(0x288).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lh(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x292)).lhu(c);
    c.g[V0][0] = !c.g[V0][0];
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `void BE_SetCharProfileFlag(self, bit, on)` (0x152430): sets (`on` != 0)
/// or clears bit `bit & 31` of the word at self+0xacc.
pub fn be_set_char_profile_flag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 1;
    c.g[V1][0] = sx(1u32 << ((c.g[A1][0] as u32) & 31));
    let word = Addr(c.g[A0][0] as u32).off(0xacc);
    if c.g[A2][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = word.lw(c);
        c.g[V1][0] = !c.g[V1][0];
        c.g[V0][0] &= c.g[V1][0];
        let value = c.g[V0][0];
        word.sw(c, value);
    } else {
        c.cycles += 4;
        c.g[V0][0] = word.lw(c);
        c.g[V0][0] |= c.g[V1][0];
        let value = c.g[V0][0];
        word.sw(c, value);
    }
    abi::ret(c);
}

/// `int BEMissionInt_GetPeakCollectedCount(self, rider, idx)` (0x1533f8): the
/// sum of `cTotalCollected` (record +4, 12-byte records) over 17 collectible
/// entries from `idx` of the rider's profile and character, in the profile
/// library at 0x4a6ca8 (0x9b50 per profile, 0xf88 per character).
pub fn be_mission_int_get_peak_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        PEAK_RIDER_PROFILE.call(c, 0x0015_341c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        CHAR_GETTER_RIDER_CHARACTER.call(c, 0x0015_3428)?;
        c.g[V1][0] = 0xc;
        c.g[A0][0] = 0xf88;
        let idx = c.g[S0][0] as u32;
        let idx_off = sx(abi::mult(c, idx, 0xc));
        c.g[S0][0] = idx_off;
        let character = c.g[V0][0] as u32;
        let char_off = sx(abi::mult(c, character, 0xf88));
        c.g[V0][0] = char_off;
        c.g[A1][0] = 0x9b50;
        c.g[V1][0] = 0x4a_0000;
        let profile = c.g[S2][0] as u32;
        let profile_off = sx(abi::mult(c, profile, 0x9b50));
        c.g[S2][0] = profile_off;
        c.g[V1][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
        c.g[A1][0] = 0;
        c.g[A0][0] = 0x10;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lb(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_3460);
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA};
    use crate::state::race_setup::{
        PLAYER_SLOTS, RIDER_RECORDS, RIDER_RECORD_SIZE, REC_CHARACTER, REC_FLAGS, REC_PROFILE_ID,
    };

    /// Race setup for `rider` (a player index): its slot, the slot's record
    /// (profile word, flags, character). Returns the profile index
    /// `BE_GetRiderProfile` gives and the character.
    fn rider_fixture(c: &mut Cpu, r: &mut Rng, rider: u32) -> (u32, u32) {
        let slot = r.next_u32() % 16;
        let profile_word = if r.next_u32() % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 2
        };
        let flags = r.next_u32();
        let ch = r.next_u32() % 16;
        c.bus.write32(PLAYER_SLOTS.0 + 4 * rider, slot);
        let rec = RIDER_RECORDS.0 + slot * RIDER_RECORD_SIZE;
        c.bus.write32(rec + REC_PROFILE_ID, profile_word);
        c.bus.write32(rec + REC_FLAGS, flags);
        c.bus.write8(rec + REC_CHARACTER, ch as u8);
        let profile = if profile_word == u32::MAX {
            2
        } else {
            flags & 1
        };
        (profile, ch)
    }

    /// The `cBECharacterInterface` getters' setup: a rider's character (through
    /// its race record) and random words in every character record.
    fn character_getter_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        let rider = seed % 4;
        let _ = rider_fixture(c, &mut r, rider);
        c.g[A1][0] = rider as u64;
        fill(c, &mut r, CHAR_GETTER_DB, 16 * CHAR_GETTER_RECORD / 4);
    }

    #[test]
    fn be_char_get_age_matches() {
        check(ee::s01::f_0014f170__generated, be_char_get_age, 200, &|c, seed| {
            character_getter_setup(c, seed, 0x0014_f170);
        });
    }

    #[test]
    fn be_char_get_height_matches() {
        check(ee::s01::f_0014f1b0__generated, be_char_get_height, 200, &|c, seed| {
            character_getter_setup(c, seed, 0x0014_f1b0);
        });
    }

    #[test]
    fn be_char_get_nationality_matches() {
        check(ee::s01::f_0014f1e0__generated, be_char_get_nationality, 200, &|c, seed| {
            character_getter_setup(c, seed, 0x0014_f1e0);
        });
    }

    #[test]
    fn be_char_get_voice_id_matches() {
        check(ee::s01::f_0014f210__generated, be_char_get_voice_id, 200, &|c, seed| {
            character_getter_setup(c, seed, 0x0014_f210);
        });
    }

    /// The map entry of a random id, its flags, and a mask that is either
    /// empty (true) or random (mostly false).
    #[test]
    fn be_test_char_profile_bolt_flags_matches() {
        check(
            ee::s01::f_00152368__generated,
            be_test_char_profile_bolt_flags,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2368, 0);
                c.pc = 0x0015_2368;
                let this = DATA;
                let map = DATA + 0x400;
                let id = r.next_u32() % 32;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.bus.write32(this + 0x288, map);
                c.bus.write16(map + 2 * id, (r.next_u32() % 64) as u16);
                for k in 0..64 {
                    c.bus.write16(this + 4 * k + 0x292, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn be_set_char_profile_flag_matches() {
        check(
            ee::s01::f_00152430__generated,
            be_set_char_profile_flag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2430, 0);
                c.pc = 0x0015_2430;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.bus.write32(DATA + 0xacc, r.next_u32());
            },
        );
    }

    /// A rider's profile and character, and the 17 collectible records from
    /// `idx` in its profile-library entry (random).
    #[test]
    fn be_mission_int_get_peak_collected_count_matches() {
        check(
            ee::s01::f_001533f8__generated,
            be_mission_int_get_peak_collected_count,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_33f8, 0);
                c.pc = 0x0015_33f8;
                let rider = seed % 8;
                let (profile, ch) = rider_fixture(c, &mut r, rider);
                let idx = r.next_u32() % 40;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = idx as u64;
                let base = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88 + idx * 0xc;
                fill(c, &mut r, base, 52);
            },
        );
    }
}
