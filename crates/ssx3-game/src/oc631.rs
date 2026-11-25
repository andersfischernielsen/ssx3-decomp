//! Staging module oc631 (wave g11.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc631`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::game::be::bedbprofile::callee::BE_GET_EVENT_RECORD_INDEX;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_2528,
        be_record_char_profile_event_place_medals,
        "BE_RecordCharProfileEventPlace",
    ),
    (0x0015_39a8, be_mission_int_set_collected_mask, "BEMissionInt_SetCollectedMask"),
];

/// Per-event stat records in the character profile: `profile + 0xad0 + 8 *
/// event_type`. Byte 0 is `hasPlayed`, byte 1 `BestMedal` (-1 = no place yet),
/// bytes 2..3 `iNumGoldOrPlat`.
pub(crate) const OC631_STAT_RECORDS: u32 = 0xad0;
/// The medal counters: `profile + 0xba0 + 8 * category`, with `a` (race mode)
/// at +0 and `b` (other modes) at +4.
pub(crate) const OC631_MEDALS: u32 = 0xba0;
/// `cBELibrary_locationDB`: 100-byte event records, the medal category at +0x54.
pub(crate) const OC631_LOCATION_DB: u32 = 0x0043_d950;
/// `BE_GetEventRecordIndex` returns this for an event type it does not know.
pub(crate) const OC631_UNKNOWN_TYPE: u64 = 0x1a;

/// `void cBECharProfileDB_updateMedalStats(profile, event, mode, place)`
/// (0x152528, `BE_RecordCharProfileEventPlace`): records a finishing `place`
/// (-1 records nothing) for `event` in the profile's stat record for that
/// event's type. Places 0 and 1 count a gold/platinum. The stat keeps the best
/// (smallest) place. The first place of an event also counts its medal
/// category: `a` for race mode 0, `b` for modes 1..3. Returns 1 when the place
/// became the new best, else 0.
pub fn be_record_char_profile_event_place_medals(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A2][0]; // mode
        c.g[S1][0] = c.g[A0][0]; // profile
        c.g[S2][0] = c.g[A3][0]; // place
        c.g[S3][0] = c.g[A1][0]; // event
        c.g[S4][0] = u64::MAX;
        let no_place = c.g[S2][0] == c.g[S4][0];
        frame.save_ra(c, 0);
        if no_place {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            be_record_event_place_body(c)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The part of [`be_record_char_profile_event_place_medals`] after the `-1`
/// check. Leaves the return value in v0.
fn be_record_event_place_body(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    BE_GET_EVENT_RECORD_INDEX.call(c, 0x0015_2568)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = OC631_UNKNOWN_TYPE;
    let unknown = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
    if unknown {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }

    // The stat record of this event's type: hasPlayed = 1, and a place below
    // two also counts a gold/platinum.
    c.cycles += 6;
    c.g[A0][0] = 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(OC631_STAT_RECORDS));
    c.g[V1][0] = (c.g[S2][0] < 2) as u64;
    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let stat = c.g[A2][0] as u32;
    let counts_gold = c.g[V1][0] != 0;
    let played = c.g[A0][0];
    Addr(stat).sb(c, played);
    if counts_gold {
        c.cycles += 3;
        c.g[V0][0] = Addr(stat).off(2).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let gold = c.g[V0][0];
        Addr(stat).off(2).sh(c, gold);
    }

    // BestMedal: -1 means no place yet.
    c.cycles += 3;
    c.g[V0][0] = Addr(stat).off(1).lb(c);
    let unset = c.g[V0][0] == c.g[S4][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if !unset {
        c.cycles += 2;
        let better = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !better {
            // A place at or after the best one: nothing changes.
            return Ok(());
        }
        c.cycles += 2;
        let place = c.g[S2][0];
        Addr(stat).off(1).sb(c, place);
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(());
    }

    // The event's first place: count its medal category.
    c.cycles += 7;
    c.g[V1][0] = 0x64;
    c.g[V0][0] = 0x0044_0000;
    c.mult(0, V1, S3, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let race_mode = c.g[S0][0] == 0;
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
    if race_mode {
        c.cycles += 3;
        c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(OC631_MEDALS));
        count_medal(c);
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 3) as u64;
        let in_b_modes = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
        if in_b_modes {
            // Modes 1..3 count `b`; other modes count nothing.
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(OC631_MEDALS + 4));
            count_medal(c);
        }
    }

    // Record the place as the event's best.
    c.cycles += 1;
    let place = c.g[S2][0];
    Addr(stat).off(1).sb(c, place);
    c.cycles += 2;
    c.g[V0][0] = 1;
    Ok(())
}

/// The medal counter at `V1 + A0`: its word goes up by one (the generated
/// `addu`, `lw`, `addiu`, `sw` block).
fn count_medal(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let medal = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(medal).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let count = c.g[V0][0];
    Addr(medal).sw(c, count);
}

/// `BE_GetRiderProfile` (0x14a0e0): the rider's profile index.
const OC631_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::f_0014a0e0,
};

/// `BE_GetRiderCharacter` (0x14a080): the rider's character index.
const OC631_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::f_0014a080,
};

/// `void BEMissionInt_SetCollectedMask(rider, idx, src)` (0x1539a8): copies the
/// 8-byte collectible mask at `src` into the rider's profile slot for location
/// `idx`, then counts its set bits into `profileLib[profile][character]
/// .mCollectibles[idx + iLocationIdx].cTotalCollected`.
pub fn be_mission_int_set_collected_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        c.g[A0][0] = c.g[S0][0];
        OC631_RIDER_PROFILE.call(c, 0x0015_39d4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        OC631_RIDER_CHARACTER.call(c, 0x0015_39e0)?;
        c.g[T5][0] = c.g[V0][0];

        // Destination: profileLib-sized slot of the profile and character,
        // then the location's 12-byte block; A1 = the mask table base.
        c.g[V1][0] = 0x9b50;
        c.g[V0][0] = 0x0053_0000;
        c.mult(0, V1, S2, V1);
        c.g[T8][0] = c.g[V0][0];
        c.g[A0][0] = 0xf88;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
        c.mult(0, A0, T5, A0);
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = 0x004a_0000;
        c.g[V0][0] = 0xc;
        c.g[T3][0] = c.g[V1][0];
        c.mult(0, A2, A2, V0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6cb0));
        c.mult(0, V0, S3, V0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T4][0] = c.g[A0][0];
        c.g[T6][0] = c.g[A1][0];
        c.g[S0][0] = u64::MAX;
        c.g[T7][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[T2][0] = c.g[A2][0];
        c.g[T0][0] = c.g[V0][0];
        c.g[T1][0] = 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));

        // Copy the 8-byte mask (unaligned pair, as the generated code does).
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[S1][0] as u32);
        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[V0][0] as u32);

        // Count the set bits, one mask bit per iteration.
        loop {
            c.cycles += 18;
            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A3][0];
            }
            c.g[A2][0] = sx((c.g[T7][0] as u32) << ((c.g[A3][0] as u32) & 31));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[A0][0] = ((c.g[A3][0] as i64) < 64) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
            let more = c.g[A0][0] != 0;
            if c.g[V1][0] != 0 {
                c.g[T1][0] = c.g[A1][0];
            }
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_3a60);
                }
                continue;
            }
            break;
        }

        c.cycles += 22;
        c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0xc;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = 0xf88;
        c.mult(0, A1, T5, A1);
        c.g[A0][0] = 0x9b50;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.mult(0, A0, S2, A0);
        c.mult(0, V0, V0, A2);
        c.g[V1][0] = 0x004a_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let count = c.g[T1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sb(c, count);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::DATA;

    use crate::game::fe::feasyncfile::tests::base_17 as base;

    use ssx3_ee as ee;

    /// The event-type table `BE_GetEventRecordIndex` searches for events 0..16:
    /// two `(key, value)` pairs of 4 bytes each, 16 bytes per event.
    const OC631_TYPE_TABLE: u32 = 0x0045_a2f8;

    #[test]
    fn be_record_char_profile_event_place_medals_matches() {
        check(
            ee::s01::f_00152528__generated,
            be_record_char_profile_event_place_medals,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x1525);
                scramble(c, &mut r);
                let ev = seed % 20;
                let mode = seed % 6;
                let place: u64 = if seed % 9 == 0 {
                    u64::MAX
                } else {
                    (seed % 6) as u64
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ev as u64;
                c.g[A2][0] = mode as u64;
                c.g[A3][0] = place;

                // Event lookup: the medal category of the event's location.
                c.bus.write32(
                    OC631_LOCATION_DB + 100 * ev + 0x54,
                    (seed as u32 + 3) % 8,
                );
                // Event-type table: found for some modes, not for others
                // (then the lookup returns the unknown type).
                let base_entry = OC631_TYPE_TABLE + 16 * ev;
                c.bus.write32(base_entry, if seed % 2 == 0 { mode as u32 } else { 7 });
                c.bus.write32(base_entry + 4, seed as u32 % 4);
                c.bus.write32(base_entry + 8, if seed % 3 == 0 { mode as u32 } else { 99 });
                c.bus.write32(base_entry + 12, 4 + seed as u32 % 20);

                // Stat records: unset (-1) or with a best place, some with
                // gold counts.
                for t in 0..27u32 {
                    let rec = DATA + OC631_STAT_RECORDS + 8 * t;
                    let best = if seed % 3 == 0 { 0xff } else { seed as u32 % 5 };
                    let gold = seed as u32 % 4;
                    let played = seed as u32 % 2;
                    c.bus.write32(rec, played | (best << 8) | (gold << 16));
                }
            },
        );
    }

    #[test]
    fn be_mission_int_set_collected_mask_matches() {
        check(
            ee::s01::f_001539a8__generated,
            be_mission_int_set_collected_mask,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x1539);
                scramble(c, &mut r);
                let src = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64; // rider
                c.g[A2][0] = (seed % 8) as u64; // location index
                c.g[A3][0] = src as u64; // the 8-byte mask
                c.bus.write32(src, r.next_u32());
                c.bus.write32(src + 4, r.next_u32());

                // The rider's record slot, then the record fields the two
                // rider callees read (profile word at +0xc, character byte
                // at +0x11).
                for rider in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * rider, (seed + rider) % 3);
                }
                for k in 0..3u32 {
                    let rec = 0x0053_5b20 + 0x1c * k;
                    let profile = if (seed + k) % 5 == 0 {
                        u32::MAX
                    } else {
                        (seed + k) % 7
                    };
                    c.bus.write32(rec + 0xc, profile);
                    c.bus.write32(rec + 0x10, ((seed + k) % 4) << 8);
                }
                c.bus.write32(0x0053_05f0, seed % 5); // raceDB.iLocationIdx
            },
        );
    }
}
