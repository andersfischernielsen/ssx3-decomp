//! Staging module oc457 (wave wave225): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc457`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_ada8,
        bolt_int_get_bolt_state,
        "BEBoltInt_GetBoltState",
    ),
    (
        0x0014_add8,
        bolt_int_get_rider_bolt_state,
        "BEBoltInt_GetRiderBoltState",
    ),
    (
        0x0014_afb0,
        bolt_int_set_bolt_equipped,
        "BEBoltInt_SetBoltEquipped",
    ),
    (
        0x0014_b1b0,
        bolt_int_char_record_call_52368,
        "BEBoltInt_CharRecordCall52368",
    ),
    (
        0x0014_b2f8,
        bolt_int_count_available_bolts,
        "BEBoltInt_CountAvailableBolts",
    ),
    (
        0x0014_b378,
        bolt_int_get_available_bolts,
        "BEBoltInt_GetAvailableBolts",
    ),
    (
        0x0014_b700,
        bolt_int_dispatch_category,
        "BEBoltInt_DispatchCategory",
    ),
    (
        0x0014_b920,
        bolt_int_collect_rider_equipped_texture_names,
        "BEBoltInt_CollectRiderEquippedTextureNames",
    ),
    (
        0x0014_bba0,
        bolt_int_get_rider_bolt_texture_name,
        "BEBoltInt_GetRiderBoltTextureName",
    ),
    (
        0x0014_dee8,
        be_call_empty_stub_dee8,
        "BE_CallEmptyStub_dee8",
    ),
    (
        0x0014_e2c0,
        be_network_int_get_sync_data_size,
        "BENetworkInt_GetSyncDataSize",
    ),
    (
        0x0014_ef00,
        be_get_rider_char_db_entry,
        "BE_GetRiderCharDBEntry",
    ),
    (
        0x0014_f108,
        be_get_rider_char_db_field_1c,
        "BE_GetRiderCharDBField1C",
    ),
    (
        0x0014_f138,
        be_get_rider_char_db_flag_2c,
        "BE_GetRiderCharDBFlag2C",
    ),
];

/// The bolt database (`Bolts_GetDB`), the profile library (the character
/// records of every rider profile), and their strides.
const BOLT_DB_BASE: u32 = 0x004a_6750;
const BOLT_PROFILE_LIB: u32 = 0x004a_6ca8;
/// Bytes per profile and per character record.
const BOLT_PROFILE_STRIDE: u32 = 0x9b50;
const BOLT_CHARACTER_STRIDE: u32 = 0x0f88;
/// Character record fields: the bolt lookup pointer, the bolt table.
const BOLT_LOOKUP_PTR: u32 = 0x288;
const BOLT_TABLE: u32 = 0x290;
/// The 525-entry bolt table, 4 bytes each.
const BOLT_TABLE_BYTES: u32 = 0x834;

/// `BEBoltInt_GetRiderProfileAndChar` (0x14ac70).
const BOLT_CALL_RIDER_PROFILE_AND_CHAR: Callee = Callee {
    addr: 0x0014_ac70,
    f: ee::s01::BEBoltInt_GetRiderProfileAndChar,
};

/// `BEBoltInt_GetBoltState` (0x14ada8).
const BOLT_CALL_GET_BOLT_STATE: Callee = Callee {
    addr: 0x0014_ada8,
    f: ee::s01::BEBoltInt_GetBoltState,
};

/// `BE_SetCharProfileBoltEquipped` (0x151c90).
const BOLT_CALL_SET_CHAR_EQUIPPED: Callee = Callee {
    addr: 0x0015_1c90,
    f: ee::s01::BE_SetCharProfileBoltEquipped,
};

/// `BE_EnsureDefaultBoltsEquipped` (0x1521f0).
const BOLT_CALL_ENSURE_DEFAULTS: Callee = Callee {
    addr: 0x0015_21f0,
    f: ee::s01::BE_EnsureDefaultBoltsEquipped,
};

/// `BE_ValidateExclusiveBolts` (0x1520e8).
const BOLT_CALL_VALIDATE_EXCLUSIVE: Callee = Callee {
    addr: 0x0015_20e8,
    f: ee::s01::BE_ValidateExclusiveBolts,
};

/// `BE_TestCharProfileBoltFlags` (0x152368).
const BOLT_CALL_TEST_FLAGS: Callee = Callee {
    addr: 0x0015_2368,
    f: ee::s01::BE_TestCharProfileBoltFlags,
};

/// `Bolts_GetCharBoltCount(db, kind)` (0x14d988).
const BOLT_CALL_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `Bolts_GetCharBolts(db, kind)` (0x14d998).
const BOLT_CALL_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `BEBoltInt_GetRiderProfile` (0x14ac50).
const BOLT_CALL_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_ac50,
    f: ee::s01::BEBoltInt_GetRiderProfile,
};

/// `BEBoltInt_GetRiderCharacter` (0x14ac30).
const BOLT_CALL_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_ac30,
    f: ee::s01::BEBoltInt_GetRiderCharacter,
};

/// `BEBoltInt_CollectEquippedTextureNames` (0x14b7a8).
const BOLT_CALL_COLLECT_TEXTURES: Callee = Callee {
    addr: 0x0014_b7a8,
    f: ee::s01::BEBoltInt_CollectEquippedTextureNames,
};

/// `BEBoltInt_GetBoltTextureName` (0x14b988).
const BOLT_CALL_TEXTURE_NAME: Callee = Callee {
    addr: 0x0014_b988,
    f: ee::s01::BEBoltInt_GetBoltTextureName,
};

/// `BE_GetRiderCharacter` (0x14a080).
const BOLT_CALL_GET_RIDER_CHARACTER_DB: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// `BE_EmptyStub_a078` (0x14a078).
const BOLT_CALL_EMPTY_STUB: Callee = Callee {
    addr: 0x0014_a078,
    f: ee::s01::BE_EmptyStub_a078,
};

/// `memcpy` (0x3e6574), reached through the function table.
const BOLT_MEMCPY: u32 = 0x003e_6574;

/// `BEBoltInt_GetBoltState(self, profile, character, slot)` (0x14ada8): the
/// address of a slot's word in a character record,
/// `profile lib + profile * 0x9b50 + character * 0xf88 + 0x290 + slot * 4`.
/// `self` is unused.
pub fn bolt_int_get_bolt_state(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    let profile = c.g[A1][0] as u32;
    c.g[A1][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
    let character = c.g[A2][0] as u32;
    c.g[A2][0] = sx(abi::mult(c, character, BOLT_CHARACTER_STRIDE));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V1][0] = sx(BOLT_PROFILE_LIB);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_TABLE));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BEBoltInt_GetRiderBoltState(self, rider, slot)` (0x14add8): the bolt
/// state address of a rider's current profile and character.
pub fn bolt_int_get_rider_bolt_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[SP][0];
        Addr(sp(c)).sw(c, 0);
        BOLT_CALL_RIDER_PROFILE_AND_CHAR.call(c, 0x0014_adfc)?;
        c.g[A2][0] = Addr(sp(c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        BOLT_CALL_GET_BOLT_STATE.call(c, 0x0014_ae10)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_SetBoltEquipped(self, profile, character, slot, on)`
/// (0x14afb0): set or clear the equipped flag (0x10) of a bolt in a
/// character record, validate the bolt set, and restore the saved bolt table
/// when the change is refused. Returns 1 when the change stuck.
pub fn bolt_int_set_bolt_equipped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        let character = c.g[A2][0] as u32;
        c.g[A2][0] = sx(abi::mult(c, character, BOLT_CHARACTER_STRIDE));
        let profile = c.g[A1][0] as u32;
        c.g[A1][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
        let frame = Frame::open(c, 0x8a0);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
        frame.save(c, S1, 0x880);
        frame.save(c, S0, 0x890);
        c.g[S1][0] = c.g[A3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S2, 0x870);
        frame.save(c, S3, 0x860);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0x850);
        c.g[V1][0] = c.g[SP][0];
        c.g[A3][0] = 0x20c;
        c.g[A0][0] = u64::MAX;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = u64::MAX;
        // The local table of 525 (id, flags) pairs starts as all unused.
        loop {
            c.cycles += 7;
            let entry = c.g[V1][0] as u32;
            let unused_id = c.g[A0][0];
            Addr(entry).sh(c, unused_id);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            Addr(entry.wrapping_add(2)).sh(c, 0);
            let more = c.g[A3][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_b000);
            }
        }
        // Save the character's bolt table, apply the change, validate.
        let applied: bool = 'applied: {
            c.cycles += 11;
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(BOLT_TABLE));
            c.g[A0][0] = c.g[SP][0];
            c.g[S3][0] = c.g[A1][0];
            c.g[A2][0] = BOLT_TABLE_BYTES as u64;
            call_ptr(c, BOLT_MEMCPY, 0x0014_b030)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            BOLT_CALL_SET_CHAR_EQUIPPED.call(c, 0x0014_b040)?;
            let set_ok = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
            if !set_ok {
                break 'applied false;
            }
            // The table slot of the bolt: the lookup gives its index (or -1).
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(BOLT_LOOKUP_PTR)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
            let unknown = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if unknown {
                c.cycles += 1;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_TABLE));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            // Set or clear the equipped flag (0x10) of the entry.
            c.cycles += 2;
            let clear = c.g[S2][0] == 0;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(2)).lhu(c);
            if clear {
                c.cycles += 1;
                c.g[V0][0] &= 0xffef;
            } else {
                c.cycles += 2;
                c.g[V0][0] |= 0x10;
            }
            c.cycles += 5;
            let flags = c.g[V0][0];
            Addr((c.g[A1][0] as u32).wrapping_add(2)).sh(c, flags);
            c.g[A0][0] = c.g[S0][0];
            BOLT_CALL_ENSURE_DEFAULTS.call(c, 0x0014_b08c)?;
            let ensured = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !ensured {
                break 'applied false;
            }
            c.cycles += 5;
            Addr(sp(c).wrapping_add(0x840)).sw(c, 0);
            c.g[A1][0] = sx(sp(c).wrapping_add(0x840));
            BOLT_CALL_VALIDATE_EXCLUSIVE.call(c, 0x0014_b0a0)?;
            let validated = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(sp(c).wrapping_add(0x840)).lw(c);
            if !validated {
                break 'applied false;
            }
            c.cycles += 2;
            let no_count = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if no_count {
                break 'applied true;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            BOLT_CALL_ENSURE_DEFAULTS.call(c, 0x0014_b0b8)?;
            let ensured_again = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            ensured_again
        };
        if applied {
            c.g[V0][0] = 1;
        } else {
            // Refused: put the saved bolt table back and report 0.
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = BOLT_TABLE_BYTES as u64;
            call_ptr(c, BOLT_MEMCPY, 0x0014_b0d0)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x880);
        frame.restore(c, S0, 0x890);
        frame.restore(c, S2, 0x870);
        frame.restore(c, S3, 0x860);
        frame.restore_ra(c, 0x850);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_CharRecordCall52368(self, rider, a2, a3)` (0x14b1b0): the rider's
/// character record, tested for bolt flags (`BE_TestCharProfileBoltFlags`).
pub fn bolt_int_char_record_call_52368(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        Addr(sp(c)).sw(c, 0);
        BOLT_CALL_RIDER_PROFILE_AND_CHAR.call(c, 0x0014_b1d4)?;
        c.g[A0][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.g[V1][0] = 0x004a_0000;
        let character = c.g[A0][0] as u32;
        c.g[A0][0] = sx(abi::mult(c, character, 0xf88));
        let profile = c.g[V0][0] as u32;
        c.g[V0][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        BOLT_CALL_TEST_FLAGS.call(c, 0x0014_b204)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_CountAvailableBolts(kind)` (0x14b2f8): how many of a
/// character's bolts (of `kind`) are not yet owned (byte +1 of the 0x38-byte
/// record is zero).
pub fn bolt_int_count_available_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = BOLT_DB_BASE as u64;
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLT_COUNT.call(c, 0x0014_b320)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        BOLT_CALL_CHAR_BOLTS.call(c, 0x0014_b330)?;
        c.g[A2][0] = c.g[V0][0];
        let count_positive = (c.g[S2][0] as i64) > 0;
        c.g[A0][0] = 0;
        if count_positive {
            c.cycles += 1;
            c.g[A1][0] = c.g[S2][0];
            loop {
                c.cycles += 7;
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x38));
                let more = c.g[A1][0] != 0;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b340);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[A0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy one 0x38-byte bolt record (56 bytes, seven doublewords) from `src`
/// to `dst`, in the generated order. Returns the last doubleword of each
/// lane (lane 1: +0x18, lane 2: +0x20, +0x28, +0x30), which the generated
/// code leaves in `v0`, `a2`, `a3` and `t0`.
fn bolt_copy_record(c: &mut Cpu, src: u32, dst: u32) -> [u64; 4] {
    let first = [
        Addr(src).read_u64_unaligned(c),
        Addr(src.wrapping_add(8)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x10)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x18)).read_u64_unaligned(c),
    ];
    Addr(dst).write_u64_unaligned(c, first[0]);
    Addr(dst.wrapping_add(8)).write_u64_unaligned(c, first[1]);
    Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, first[2]);
    Addr(dst.wrapping_add(0x18)).write_u64_unaligned(c, first[3]);
    let second = [
        Addr(src.wrapping_add(0x20)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x28)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x30)).read_u64_unaligned(c),
    ];
    Addr(dst.wrapping_add(0x20)).write_u64_unaligned(c, second[0]);
    Addr(dst.wrapping_add(0x28)).write_u64_unaligned(c, second[1]);
    Addr(dst.wrapping_add(0x30)).write_u64_unaligned(c, second[2]);
    [second[0], second[1], second[2], first[3]]
}

/// `BEBoltInt_GetAvailableBolts(kind, out)` (0x14b378): copy the unowned
/// bolt records of `kind` to `out` (0x38 bytes each). Returns how many were
/// copied. The records are walked from the last one back to the first.
pub fn bolt_int_get_available_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = BOLT_DB_BASE as u64;
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLT_COUNT.call(c, 0x0014_b3a8)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLTS.call(c, 0x0014_b3b8)?;
        c.g[V1][0] = c.g[V0][0];
        let count_positive = (c.g[S3][0] as i64) > 0;
        c.g[A1][0] = 0;
        if count_positive {
            c.cycles += 1;
            c.g[A0][0] = c.g[S3][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(1)).lb(c);
                let unowned = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                if unowned {
                    c.cycles += 30;
                    let src = c.g[V1][0] as u32;
                    let dst = c.g[S2][0] as u32;
                    let regs = bolt_copy_record(c, src, dst);
                    c.g[V0][0] = regs[0];
                    c.g[A2][0] = regs[1];
                    c.g[A3][0] = regs[2];
                    c.g[T0][0] = regs[3];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x38));
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b3c8);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_DispatchCategory(category)` (0x14b700): the name of a bolt
/// category (0..9), from a jump table at 0x45a5c0 of handler addresses.
/// Categories of 10 or more return 0. An unknown handler leaves through
/// `jump_out`, as the generated code does.
pub fn bolt_int_dispatch_category(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0xau64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5a40));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let handler = c.g[A0][0] as u32;
    let name_offset: u32 = match handler {
        0x0014_b724 => 0x5be8,
        0x0014_b730 => 0x5bc0,
        0x0014_b73c => 0x5b98,
        0x0014_b748 => 0x5b68,
        0x0014_b754 => 0x5b40,
        0x0014_b760 => 0x5b18,
        0x0014_b76c => 0x5ae8,
        0x0014_b778 => 0x5ac0,
        0x0014_b784 => 0x5a98,
        0x0014_b790 => 0x5a70,
        _ => {
            c.jump_out(handler, 0x0014_b700, 0x0014_b7a4);
            return;
        }
    };
    c.cycles += 3;
    c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(name_offset));
    abi::ret(c);
}

/// `BEBoltInt_CollectRiderEquippedTextureNames(self, rider, a2, a3)`
/// (0x14b920): the equipped bolt textures of a rider's character, through
/// `BEBoltInt_CollectEquippedTextureNames`.
pub fn bolt_int_collect_rider_equipped_texture_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_RIDER_PROFILE.call(c, 0x0014_b94c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_RIDER_CHARACTER.call(c, 0x0014_b958)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        BOLT_CALL_COLLECT_TEXTURES.call(c, 0x0014_b96c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_GetRiderBoltTextureName(self, rider, a2, a3, t0)` (0x14bba0):
/// the texture name of a rider's bolt, through `BEBoltInt_GetBoltTextureName`.
pub fn bolt_int_get_rider_bolt_texture_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BOLT_CALL_RIDER_PROFILE.call(c, 0x0014_bbd4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        BOLT_CALL_RIDER_CHARACTER.call(c, 0x0014_bbe0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[V0][0];
        BOLT_CALL_TEXTURE_NAME.call(c, 0x0014_bbf8)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BE_CallEmptyStub_dee8` (0x14dee8): calls the empty stub with the word at
/// `gp-0x1ef8`.
pub fn be_call_empty_stub_dee8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ef8)).lw(c);
        frame.save_ra(c, 0);
        BOLT_CALL_EMPTY_STUB.call(c, 0x0014_defc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BENetworkInt_GetSyncDataSize` (0x14e2c0): the size of the network sync
/// block, 0xc44 bytes.
pub fn be_network_int_get_sync_data_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xc44;
    abi::ret(c);
}

/// `BE_GetRiderCharDBEntry(rider)` (0x14ef00): the rider's character record
/// in the character table at 0x5309a0.
pub fn be_get_rider_char_db_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BOLT_CALL_GET_RIDER_CHARACTER_DB.call(c, 0x0014_ef10)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x0053_0000;
        let character = c.g[V0][0] as u32;
        c.g[V0][0] = sx(abi::mult(c, character, 0x88));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x9a0));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    });
}

/// `BE_GetRiderCharDBField1C(rider)` (0x14f108): the character table's
/// field at +0x1c of a rider's character, 0x5309bc.
pub fn be_get_rider_char_db_field_1c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BOLT_CALL_GET_RIDER_CHARACTER_DB.call(c, 0x0014_f118)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x0053_0000;
        let character = c.g[V0][0] as u32;
        c.g[V0][0] = sx(abi::mult(c, character, 0x88));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x9bc));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    });
}

/// `BE_GetRiderCharDBFlag2C(rider)` (0x14f138): whether a rider's character
/// has the flag at +0x5c of its 0x970 entry set (`0 < word`).
pub fn be_get_rider_char_db_flag_2c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BOLT_CALL_GET_RIDER_CHARACTER_DB.call(c, 0x0014_f148)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x0053_0000;
        let character = c.g[V0][0] as u32;
        c.g[V0][0] = sx(abi::mult(c, character, 0x88));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x970));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    /// Gate for the fake of `BE_SetCharProfileBoltEquipped` (the installed
    /// slot): the fake runs only while this word holds `SET_MAGIC`; the next
    /// word is its result (bit 0).
    const SET_GATE: u32 = DATA + 0x20_0000;
    const SET_MAGIC: u32 = 0x5e7b_0457;

    /// The fake of `BE_SetCharProfileBoltEquipped`: a forced result, gate off
    /// runs the body the slot held.
    fn oc457_set_equipped_fake(c: &mut Cpu) {
        if c.bus.read32(SET_GATE) != SET_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(oc457_set_equipped_fake),
                ee::s01::f_00151c90__generated,
            );
        }
        c.g[V0][0] = (c.bus.read32(SET_GATE + 4) & 1) as u64;
        c.cycles += 9;
        abi::ret(c);
    }

    /// Hooks the fake into the installed slot (once per process).
    fn oc457_install_fakes() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00151C90),
            oc457_set_equipped_fake,
            crate::difftest::fakes::prev_cell(oc457_set_equipped_fake),
        );
    }

    /// The character record of (profile, character) in the profile library.
    fn oc457_record(profile: u32, character: u32) -> u32 {
        BOLT_PROFILE_LIB + profile * BOLT_PROFILE_STRIDE + character * BOLT_CHARACTER_STRIDE
    }

    /// A bolt database entry for `kind` with `count` records at a fixed list
    /// (the list's flag bytes at +1 are random).
    fn oc457_bolt_db(c: &mut Cpu, r: &mut Rng, kind: u32, count: u32) {
        let entry = BOLT_DB_BASE + kind * 4;
        let list = DATA + 0x3000;
        c.bus.write32(BOLT_DB_BASE + 4, list);
        c.bus.write32(entry + 0x2c, count);
        c.bus.write32(entry + 0xa4, 0);
        for i in 0..count {
            c.bus.write8(list + 0x38 * i + 1, (r.next_u32() & 1) as u8);
        }
    }

    #[test]
    fn bolt_int_get_bolt_state_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014ada8__generated,
            bolt_int_get_bolt_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x1ada_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64
                } else {
                    (seed % 4) as u64
                };
                c.g[A2][0] = if seed % 4 == 0 {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                } else {
                    (seed % 3) as u64
                };
                c.g[A3][0] = (seed % 9) as u64;
            },
        );
    }

    #[test]
    fn bolt_int_get_rider_bolt_state_matches() {
        check(
            ssx3_ee::s01::f_0014add8__generated,
            bolt_int_get_rider_bolt_state,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0xadd8, 0);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 9) as u64;
            },
        );
    }

    #[test]
    fn bolt_int_set_bolt_equipped_matches() {
        oc457_install_fakes();
        check_lohi(
            ssx3_ee::s01::f_0014afb0__generated,
            bolt_int_set_bolt_equipped,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xafb0, 0);
                let profile = seed % 2;
                let character = seed % 3;
                let slot = seed % 5;
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = character as u64;
                c.g[A3][0] = slot as u64;
                c.g[T0][0] = (seed % 2) as u64;
                c.bus.write32(SET_GATE, SET_MAGIC);
                c.bus
                    .write32(SET_GATE + 4, if seed % 4 == 0 { 0 } else { 1 });
                let rec = oc457_record(profile, character);
                c.bus.write32(rec + BOLT_LOOKUP_PTR, DATA + 0x2000);
                let index: i16 = if seed % 7 == 0 { -1 } else { (seed % 4) as i16 };
                c.bus.write16(DATA + 0x2000 + 2 * slot, index as u16);
                fill(c, &mut r, rec + BOLT_TABLE, 0x40);
            },
        );
    }

    #[test]
    fn bolt_int_char_record_call_52368_matches() {
        check(
            ssx3_ee::s01::f_0014b1b0__generated,
            bolt_int_char_record_call_52368,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb1b0, 0);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bolt_int_count_available_bolts_matches() {
        check(
            ssx3_ee::s01::f_0014b2f8__generated,
            bolt_int_count_available_bolts,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb2f8, 0);
                let kind = seed % 3;
                c.g[A1][0] = kind as u64;
                fill(c, &mut r, DATA + 0x3000, 0x40);
                oc457_bolt_db(c, &mut r, kind, seed % 5);
            },
        );
    }

    #[test]
    fn bolt_int_get_available_bolts_matches() {
        check(
            ssx3_ee::s01::f_0014b378__generated,
            bolt_int_get_available_bolts,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb378, 0);
                let kind = seed % 3;
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = (DATA + 0x5000) as u64;
                fill(c, &mut r, DATA + 0x3000, 0x40);
                fill(c, &mut r, DATA + 0x5000, 0x80);
                oc457_bolt_db(c, &mut r, kind, seed % 5);
            },
        );
    }

    #[test]
    fn bolt_int_dispatch_category_matches() {
        const HANDLERS: [u32; 10] = [
            0x0014_b724,
            0x0014_b730,
            0x0014_b73c,
            0x0014_b748,
            0x0014_b754,
            0x0014_b760,
            0x0014_b76c,
            0x0014_b778,
            0x0014_b784,
            0x0014_b790,
        ];
        check(
            ssx3_ee::s01::f_0014b700__generated,
            bolt_int_dispatch_category,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0xb700, 0);
                // The generated body checks `pc` for its resume entries.
                c.pc = 0x0014_b700;
                c.g[A1][0] = if seed % 13 == 0 {
                    u64::MAX
                } else {
                    (seed % 12) as u64
                };
                for (i, h) in HANDLERS.iter().enumerate() {
                    c.bus.write32(0x0045_a5c0 + 4 * i as u32, *h);
                }
                if seed % 5 == 0 {
                    c.bus.write32(0x0045_a5c0 + 4 * (seed % 10), 0x0014_b7a4);
                }
            },
        );
    }

    #[test]
    fn bolt_int_collect_rider_equipped_texture_names_matches() {
        check(
            ssx3_ee::s01::f_0014b920__generated,
            bolt_int_collect_rider_equipped_texture_names,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb920, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bolt_int_get_rider_bolt_texture_name_matches() {
        check(
            ssx3_ee::s01::f_0014bba0__generated,
            bolt_int_get_rider_bolt_texture_name,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0xbba0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn be_call_empty_stub_dee8_matches() {
        check(
            ssx3_ee::s01::f_0014dee8__generated,
            be_call_empty_stub_dee8,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0xdee8, 0);
                c.bus.write32(GAME_GP - 0x1ef8, r.next_u32());
            },
        );
    }

    #[test]
    fn be_network_int_get_sync_data_size_matches() {
        check(
            ssx3_ee::s01::f_0014e2c0__generated,
            be_network_int_get_sync_data_size,
            4,
            &|c, seed| {
                let mut r = Rng(0xe2c0_0000 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn be_get_rider_char_db_getters_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_0014ef00__generated as fn(&mut Cpu),
                be_get_rider_char_db_entry as fn(&mut Cpu),
                0xef00,
            ),
            (
                ssx3_ee::s01::f_0014f108__generated,
                be_get_rider_char_db_field_1c,
                0xf108,
            ),
            (
                ssx3_ee::s01::f_0014f138__generated,
                be_get_rider_char_db_flag_2c,
                0xf138,
            ),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, 0x0053_0000, 0x800);
            });
        }
    }
}
