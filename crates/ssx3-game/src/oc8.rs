//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and LO/HI) results exactly, with a
//! differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_4c60, be_new_race_int_get_track_name_by_index, "BENewRaceInt_GetTrackNameByIndex"),
    (0x0014_4d70, be_new_race_int_set_rider_slot, "BENewRaceInt_SetRiderSlot"),
    (0x0014_4d98, be_new_race_int_set_num_ai_riders, "BENewRaceInt_SetNumAIRiders"),
    (0x0014_5340, be_new_race_int_get_event_name, "BENewRaceInt_GetEventName"),
    (0x0014_5378, be_new_race_int_get_game_mode_name, "BENewRaceInt_GetGameModeName"),
    (0x0014_5398, be_new_race_int_get_event_type_name, "BENewRaceInt_GetEventTypeName"),
    (0x0014_53b8, be_new_race_int_get_event_type_name_by_index, "BENewRaceInt_GetEventTypeNameByIndex"),
    (0x0014_72c8, be_new_player_int_set_pad_index_by_slot, "BENewPlayerInt_SetPadIndexBySlot"),
    (0x0014_7318, be_new_player_int_get_pad_index_by_slot, "BENewPlayerInt_GetPadIndexBySlot"),
    (0x0014_7448, be_new_player_int_set_character_by_slot, "BENewPlayerInt_SetCharacterBySlot"),
    (0x0014_7488, be_new_player_int_get_character_by_slot, "BENewPlayerInt_GetCharacterBySlot"),
];

// ---------------------------------------------------------------------------
// New race tables.

/// `char *BENewRaceInt_GetTrackNameByIndex(BENewRaceInt *self, int index)`
/// (0x144c60): `trackNames[index]`, records of 100 bytes starting at 0x43d954.
pub fn be_new_race_int_get_track_name_by_index(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 1);
    c.g[V1][0] = 0x64; // 100
    let low = abi::mult(c, index, 0x64);
    c.g[A1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x26ac); // 0x43d954
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `void BENewRaceInt_SetRiderSlot(BENewRaceInt *self, int slot, int rider)`
/// (0x144d70): `riders[slot] = rider` and `riderSlots[rider] = slot`.
pub fn be_new_race_int_set_rider_slot(c: &mut Cpu) {
    c.cycles += 9;
    let slot = c.g[A1][0] as u32;
    let rider = c.g[A2][0] as u32;
    let base = 0x53_0000u32.wrapping_add(0x5bc8); // 0x535bc8
    c.g[A0][0] = sx(rider << 2);
    c.g[V0][0] = sx(base.wrapping_add(slot << 2));
    c.g[V1][0] = sx(base.wrapping_add(rider << 2));
    Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
    Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `bool BENewRaceInt_SetNumAIRiders(BENewRaceInt *self, int count)` (0x144d98):
/// bump a per-race running total by `count`, store it, and report whether the
/// selected entry is now past the limit `count` (1 when it is not).
pub fn be_new_race_int_set_num_ai_riders(c: &mut Cpu) {
    c.cycles += 21;
    let race = 0x53_0000u32.wrapping_add(0x5bc8); // 0x535bc8
    let num = c.g[A1][0];
    let entry = Addr(race).off(0x49).lb(c);
    let scaled = abi::mult(c, entry as u32, 0x1c);
    c.g[A2][0] = sx(scaled);
    let mut total = Addr(race).off(0x34).lw(c) as u32;
    let base = Addr(race).off(0x38).lw(c) as u32;
    let mut offset = (Addr(race).off(0x48).lb(c) as u32) << 2;
    total = total.wrapping_add(num as u32);
    Addr(race).off(0x3c).sw(c, num);
    total = total.wrapping_add(base);
    offset = offset.wrapping_add(scaled);
    Addr(race).off(0x30).sw(c, sx(total));
    let table = 0x44_0000u32.wrapping_sub(0x2708); // 0x43d8f8
    offset = offset.wrapping_add(table);
    c.g[V1][0] = sx(offset);
    let loaded = Addr(offset).lw(c);
    c.g[V0][0] = ((loaded < c.g[A1][0]) as u64) ^ 1;
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventName(BENewRaceInt *self)` (0x145340):
/// `eventNames[eventType]`, records of 84 bytes starting at 0x43e97c.
pub fn be_new_race_int_get_event_name(c: &mut Cpu) {
    c.cycles += 8;
    let event = Addr(0x53_0000u32.wrapping_add(0x5c12)).lb(c) as u32; // 0x535c12
    let low = abi::mult(c, event, 0x54);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1684); // 0x43e97c
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetGameModeName(BENewRaceInt *self)` (0x145378):
/// `gameModeNames[gameMode]`, records of 67 bytes starting at 0x43e707.
pub fn be_new_race_int_get_game_mode_name(c: &mut Cpu) {
    c.cycles += 8;
    let mode = Addr(0x53_0000u32.wrapping_add(0x5c11)).lb(c) as u32; // 0x535c11
    let low = abi::mult(c, mode, 0x43);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x18f9); // 0x43e707
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventTypeName(BENewRaceInt *self)` (0x145398):
/// `eventTypeNames[eventType]`, records of 60 bytes starting at 0x43e7d0.
pub fn be_new_race_int_get_event_type_name(c: &mut Cpu) {
    c.cycles += 8;
    let kind = Addr(0x53_0000u32.wrapping_add(0x5c10)).lb(c) as u32; // 0x535c10
    let low = abi::mult(c, kind, 0x3c);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1830); // 0x43e7d0
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventTypeNameByIndex(BENewRaceInt *self, int index)`
/// (0x1453b8): `eventTypeNames[index]`, records of 60 bytes at 0x43e7d0.
pub fn be_new_race_int_get_event_type_name_by_index(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 1);
    c.g[V1][0] = 0x3c; // 60
    let low = abi::mult(c, index, 0x3c);
    c.g[A1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1830); // 0x43e7d0
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Player slots. `BENewPlayerInt_GetSlot` (0x147798) computes
// `0x534fe0 + slot*28`; these functions inline that arithmetic.

/// `void BENewPlayerInt_SetPadIndexBySlot(BENewPlayerInt *self, int pad,
/// int value)` (0x1472c8): store `value` in the player's pad-index word and,
/// when it is non-negative, force bit 0 of the flag word at +0x10 to `pad & 1`
/// (clearing bits 0 and 2 first).
pub fn be_new_player_int_set_pad_index_by_slot(c: &mut Cpu) {
    c.cycles += 8;
    let pad = c.g[A1][0];
    let value = c.g[A2][0];
    c.g[A3][0] = pad;
    let scaled = abi::mult(c, pad as u32, 0x1c);
    c.g[A0][0] = sx(scaled);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    c.g[V1][0] = sx(base);
    let player = base.wrapping_add(scaled);
    c.g[V0][0] = sx(player);
    Addr(player).off(0xc).sw(c, value);
    if (value as i64) >= 0 {
        c.cycles += 9;
        c.g[A2][0] = sx(player);
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        let flags = Addr(player).off(0x10).lw(c);
        let bit = c.g[A3][0] & 1;
        c.g[A0][0] = 0xffff_ffff_ffff_fffb;
        let updated = ((flags & 0xffff_ffff_ffff_fffe) | bit) & 0xffff_ffff_ffff_fffb;
        c.g[V1][0] = bit;
        c.g[V0][0] = updated;
        Addr(player).off(0x10).sw(c, updated);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int BENewPlayerInt_GetPadIndexBySlot(BENewPlayerInt *self, int pad)`
/// (0x147318): the player's pad-index word (slot `pad`, record size 28).
pub fn be_new_player_int_get_pad_index_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    let pad = abi::arg(c, 1);
    let scaled = abi::mult(c, pad, 0x1c);
    c.g[A1][0] = sx(scaled);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    let player = base.wrapping_add(scaled);
    c.g[V1][0] = sx(player);
    c.g[V0][0] = Addr(player).off(0xc).lw(c);
    abi::ret(c);
}

/// `void BENewPlayerInt_SetCharacterBySlot(BENewPlayerInt *self, int pad,
/// int character)` (0x147448): if the player's character slot is unlocked
/// (bit 2 of the word at +0x10 clear) store `character` at +0x11, otherwise
/// copy the character already at the base record's +0x11.
pub fn be_new_player_int_set_character_by_slot(c: &mut Cpu) {
    c.cycles += 11;
    let pad = abi::arg(c, 1);
    let character = c.g[A2][0];
    let scaled = abi::mult(c, pad, 0x1c);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    c.g[A0][0] = sx(base);
    c.g[V1][0] = 0x1;
    let player = base.wrapping_add(scaled);
    c.g[A1][0] = sx(player);
    let flags = Addr(player).off(0x10).lw(c);
    let locked = ((flags as u32) >> 2) & 1;
    c.g[V0][0] = locked as u64;
    if locked != 1 {
        c.cycles += 2;
        Addr(player).off(0x11).sb(c, character);
    } else {
        c.cycles += 3;
        let copied = Addr(base).off(0x11).lbu(c);
        c.g[V0][0] = copied;
        Addr(player).off(0x11).sb(c, copied);
    }
    abi::ret(c);
}

/// `int BENewPlayerInt_GetCharacterBySlot(BENewPlayerInt *self, int pad)`
/// (0x147488): the player's character byte at +0x11.
pub fn be_new_player_int_get_character_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    let pad = abi::arg(c, 1);
    let scaled = abi::mult(c, pad, 0x1c);
    c.g[V1][0] = 0x1c;
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    let player = base.wrapping_add(scaled);
    c.g[V0][0] = sx(base);
    c.g[A1][0] = sx(player);
    c.g[V0][0] = Addr(player).off(0x11).lb(c);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, scramble, Rng};

    /// The `BENewRaceInt` singleton and the player-record base the generated
    /// `GetSlot` computes (`0x534fe0`).
    const RACE: u32 = 0x53_5bc8;
    const PLAYERS: u32 = 0x53_4fe0;

    #[test]
    fn be_new_race_int_get_track_name_by_index_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144c60,
            be_new_race_int_get_track_name_by_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                let index = match seed % 3 {
                    0 => seed % 8,
                    1 => 0xffff_ffff, // -1
                    _ => r.next_u32(),
                };
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_set_rider_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144d70,
            be_new_race_int_set_rider_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_set_num_ai_riders_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144d98,
            be_new_race_int_set_num_ai_riders,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0003 ^ seed as u64);
                scramble(c, &mut r);
                let entry = (seed % 8) as u8;
                let count = (seed % 4) as u8;
                c.bus.write8(RACE + 0x48, entry);
                c.bus.write8(RACE + 0x49, count);
                c.bus.write32(RACE + 0x34, r.next_u32());
                c.bus.write32(RACE + 0x38, r.next_u32());
                c.g[A1][0] = match seed % 4 {
                    0 => r.next_u32() as u64,
                    1 => 0xffff_ffff_8000_0000,
                    2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => 0x1_0000_0000u64 | r.next_u32() as u64,
                };
                let at = 0x43_d8f8u32
                    .wrapping_add((entry as u32) << 2)
                    .wrapping_add((count as u32) * 0x1c);
                c.bus.write32(at, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145340,
            be_new_race_int_get_event_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0005 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c12, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_game_mode_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145378,
            be_new_race_int_get_game_mode_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0006 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c11, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_type_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145398,
            be_new_race_int_get_event_type_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0007 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c10, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_type_name_by_index_matches() {
        check_lohi(
            ssx3_ee::s01::f_001453b8,
            be_new_race_int_get_event_type_name_by_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0008 ^ seed as u64);
                scramble(c, &mut r);
                let index = match seed % 3 {
                    0 => seed % 8,
                    1 => 0xffff_ffff, // -1
                    _ => r.next_u32(),
                };
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn be_new_player_int_set_pad_index_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_001472c8,
            be_new_player_int_set_pad_index_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0009 ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                let mut value = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                if seed % 2 == 0 {
                    value |= 0x8000_0000_0000_0000; // negative: skip the flag update
                } else {
                    value &= 0x7fff_ffff_ffff_ffff;
                }
                c.g[A2][0] = value;
                c.bus.write32(PLAYERS + slot * 28 + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_pad_index_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147318,
            be_new_player_int_get_pad_index_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000a ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.bus.write32(PLAYERS + slot * 28 + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_set_character_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147448,
            be_new_player_int_set_character_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                let player = PLAYERS + slot * 28;
                // Cover both: bit 2 set (locked -> copy) and clear (store).
                let mut flags = r.next_u32();
                flags = (flags & !0x4) | if seed % 2 == 0 { 0x4 } else { 0 };
                c.bus.write32(player + 0x10, flags);
                c.bus.write8(PLAYERS + 0x11, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn be_new_player_int_get_character_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147488,
            be_new_player_int_get_character_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000c ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.bus.write8(PLAYERS + slot * 28 + 0x11, r.next_u32() as u8);
            },
        );
    }
}
