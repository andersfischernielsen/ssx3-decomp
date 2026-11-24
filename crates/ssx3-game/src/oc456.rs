//! Staging module oc456 (wave wave225): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc456`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::be::beintmission::{BE_GET_RIDER_CHARACTER, BE_GET_RIDER_PROFILE};
use crate::game::be::beintplayer::GET_SLOT;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_6320, be_new_player_int_set_peak_flag, "BENewPlayerInt_SetPeakFlag"),
    (0x0014_66a0, be_new_player_int_get_current_peak_flag_b, "BENewPlayerInt_GetCurrentPeakFlagB"),
    (0x0014_68b0, be_new_player_int_set_peak_flag_b, "BENewPlayerInt_SetPeakFlagB"),
    (0x0014_6c10, be_new_player_int_set_event_type_flag, "BENewPlayerInt_SetEventTypeFlag"),
    (0x0014_7698, be_new_player_int_get_slot_field4, "BENewPlayerInt_GetSlotField4"),
    (0x0014_76d8, be_new_player_int_set_slot_field4, "BENewPlayerInt_SetSlotField4"),
    (0x0014_7718, be_new_player_int_get_slot_field8, "BENewPlayerInt_GetSlotField8"),
    (0x0014_7758, be_new_player_int_set_slot_field8, "BENewPlayerInt_SetSlotField8"),
    (0x0014_7a30, be_new_player_int_set_message_flag, "BENewPlayerInt_SetMessageFlag"),
    (0x0014_7b20, be_new_player_int_get_char_pair_a, "BENewPlayerInt_GetCharPairA"),
    (0x0014_7bb0, be_new_player_int_get_char_pair_b, "BENewPlayerInt_GetCharPairB"),
    (0x0014_7c40, be_new_player_int_set_difficulty, "BENewPlayerInt_SetDifficulty"),
    (0x0014_7f10, be_new_player_int_get_difficulty_counter, "BENewPlayerInt_GetDifficultyCounter"),
    (0x0014_8328, be_char_int_get_char_data, "BECharInt_GetCharData"),
    (0x0014_a078, be_empty_stub_a078, "BE_EmptyStub_a078"),
    (0x0014_acf8, be_init_rider_char_profile_bolt_map, "BE_InitRiderCharProfileBoltMap"),
];

/// Offset of the 64-bit rival-challenge word in a character record.
const CHALLENGE_WORD: u32 = 0x278;

/// The rival-challenge bit store every kind branch ends in: read the 64-bit
/// challenge word of `profileLib[profile][char]`, clear `mask`'s zero bit
/// and or in bit `shift` of `value`. `mul_reg` holds the character index.
fn rival_challenge_store(c: &mut Cpu, cycles: u64, mul_reg: usize, shift: u32, mask: u64) {
    c.cycles += cycles;
    c.g[V1][0] = 0xf88;
    c.g[V0][0] = 0x9b50;
    c.mult(0, V1, mul_reg, V1);
    c.mult(0, V0, S3, V0);
    c.g[A0][0] = 0x4a0000;
    c.g[A2][0] = mask;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
    c.g[A1][0] = c.g[S4][0] & 0x1;
    c.g[A1][0] = c.g[A1][0] << shift;
    // Tail: the record address, then the read-modify-write.
    c.cycles += 7;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let word = Addr((c.g[V1][0] as u32).wrapping_add(CHALLENGE_WORD));
    c.g[V0][0] = word.ld(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    let updated = c.g[V0][0];
    word.sd(c, updated);
}

/// `BENewPlayerInt_SetPeakFlag` (0x146320; the C++ name is
/// `cBENewPlayerInterface_setRivalChallengePlayed`): set one bit of a rider's
/// rival-challenge word in its profile record. `kind` 0..=2 selects the peak;
/// a non-zero `flag` picks the time challenge (bits 3..=5), zero the points
/// challenge (bits 0..=2). The bit takes the low bit of `value`. Other kinds
/// store nothing. Saves s0..s4 in a 0x60 frame.
pub fn be_new_player_int_set_peak_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S4][0] = c.g[T0][0];

        c.g[A0][0] = c.g[S0][0];
        crate::game::be::beintmission::BE_GET_RIDER_PROFILE.call(c, 0x0014_6354)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        crate::game::be::beintplayer::GET_SLOT.call(c, 0x0014_6360)?;

        // Record address: 0x534fe0 + slot * 0x1c.
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let record = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(record).off(0x11).lb(c);
        c.g[A1][0] = c.g[A0][0];

        let time = c.g[S1][0] != 0;
        c.cycles += 2;
        c.g[V0][0] = 1;
        if c.g[S2][0] == 0 {
            if time {
                rival_challenge_store(c, 10, A0, 3, 0xffff_ffff_ffff_fff7);
            } else {
                rival_challenge_store(c, 9, A0, 0, 0xffff_ffff_ffff_fffe);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 2;
            if c.g[S2][0] == 1 {
                if time {
                    rival_challenge_store(c, 10, A0, 4, 0xffff_ffff_ffff_ffef);
                } else {
                    rival_challenge_store(c, 10, A0, 1, 0xffff_ffff_ffff_fffd);
                }
            } else {
                c.cycles += 2;
                if c.g[S2][0] == 2 {
                    if time {
                        rival_challenge_store(c, 10, A0, 5, 0xffff_ffff_ffff_ffdf);
                    } else {
                        rival_challenge_store(c, 9, A1, 2, 0xffff_ffff_ffff_fffb);
                    }
                }
            }
        }

        // Epilogue: every path restores s0 here (the generated code does so
        // before it reaches the epilogue).
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The 64-bit challenge-word read of `profileLib[profile][char]` that both
/// `peak_flag_b_by_kind` branches end in: `V1` holds `char * 0xf88` (set by
/// the caller), `V0` gets bit `bit` of the word, `A0` the profile's offset.
fn peak_flag_b_read(c: &mut Cpu, cycles: u64, bit: u32) {
    c.cycles += cycles;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V1, T0, V1);
    c.mult(0, A0, S1, A0);
    c.g[V0][0] = 0x4a_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = Addr((c.g[V1][0] as u32).wrapping_add(CHALLENGE_WORD)).ld(c);
    c.g[V0][0] = (word >> bit) & 1;
}

/// The kind dispatch of `BENewPlayerInt_GetCurrentPeakFlagB`: kind 0, 1 and 2
/// return bits `base`, `base + 1` and `base + 2` of the challenge word (the
/// last one costs `kind2_cycles`); any other kind returns 0. `V1` holds the
/// kind on entry.
fn peak_flag_b_by_kind(c: &mut Cpu, base: u32, kind2_cycles: u64) {
    let kind = c.g[V1][0];
    c.cycles += 3;
    c.g[V0][0] = 1;
    if kind == 1 {
        c.g[V1][0] = 0xf88;
        peak_flag_b_read(c, 12, base + 1);
        return;
    }
    c.cycles += 3;
    if (kind as i64) < 2 {
        c.g[V0][0] = 1;
        c.cycles += 2;
        c.g[V1][0] = 0xf88;
        if kind == 0 {
            peak_flag_b_read(c, 12, base);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
    } else {
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.g[V1][0] = 0xf88;
        if kind == 2 {
            peak_flag_b_read(c, kind2_cycles, base + 2);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
    }
}

/// `int BENewPlayerInt_GetCurrentPeakFlagB(BENewPlayerInt *self, int index)`
/// (0x1466a0; the C++ reference calls it `cBENewPlayerInterface_isRivalEventLocked1`):
/// one bit of a rider's challenge word in its profile record, chosen by the
/// kind of the current location (`locationDB[raceDB[16]].kind`, 0..=2). Bits
/// 6..=8 when `raceDB[18] == 5`, bits 9..=11 otherwise; other kinds give 0.
pub fn be_new_player_int_get_current_peak_flag_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];

        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_66bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        GET_SLOT.call(c, 0x0014_66c8)?;

        // Race-DB words at 0x5305f0 (location index) and 0x5305f8 (flag byte).
        c.g[A1][0] = 0x53_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b0));
        c.g[A3][0] = 0x64;
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x40)).lw(c);
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 0x44_0000;
        c.mult(0, A2, A2, A3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26b0));
        c.g[A3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x48)).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = 5;
        c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11)).lb(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let not_five = c.g[A3][0] != c.g[A1][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x54)).lw(c);

        if not_five {
            peak_flag_b_by_kind(c, 9, 11);
        } else {
            peak_flag_b_by_kind(c, 6, 12);
        }

        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The store block of `BENewPlayerInt_SetPeakFlagB`: `profileLib[profile][char]`
/// (`A0` the profile offset, `S2` the character) gets bit `shift` of its
/// challenge word set to `value & 1` (`S3`), under `mask`. `cycles` is the
/// block's own cost, the common tail adds 8.
fn peak_flag_b_store(c: &mut Cpu, cycles: u64, shift: u32, mask: u64) {
    c.cycles += cycles;
    c.g[V1][0] = 0xf88;
    c.g[V0][0] = 0x9b50;
    c.mult(0, V0, A0, V0);
    c.mult(0, V1, S2, V1);
    c.g[A0][0] = 0x4a_0000;
    c.g[A1][0] = c.g[S3][0] & 0x1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
    c.g[A2][0] = mask;
    c.g[A1][0] = c.g[A1][0] << shift;
    c.cycles += 8;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let word = Addr((c.g[V1][0] as u32).wrapping_add(CHALLENGE_WORD));
    c.g[V0][0] = word.ld(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    let updated = c.g[V0][0];
    word.sd(c, updated);
}

/// Kinds 0..=2 with `flag != 0` (the time challenge): bits 6..=8.
fn peak_flag_b_time_group(c: &mut Cpu) {
    let kind = c.g[S1][0];
    c.cycles += 3;
    c.g[V0][0] = ((kind as i64) < 2) as u64;
    if kind == 1 {
        peak_flag_b_store(c, 10, 7, 0xffff_ffff_ffff_ff7f);
        return;
    }
    c.cycles += 2;
    if (kind as i64) < 2 {
        c.cycles += 2;
        if kind == 0 {
            peak_flag_b_store(c, 10, 6, 0xffff_ffff_ffff_ffbf);
            return;
        }
        c.cycles += 2;
        return;
    }
    c.g[V0][0] = 2;
    c.cycles += 2;
    if kind == 2 {
        peak_flag_b_store(c, 10, 8, 0xffff_ffff_ffff_feff);
        return;
    }
    c.cycles += 2;
}

/// Kinds 0..=2 with `flag == 0` (the points challenge): bits 9..=11.
fn peak_flag_b_free_group(c: &mut Cpu) {
    let kind = c.g[S1][0];
    c.cycles += 3;
    c.g[V0][0] = ((kind as i64) < 2) as u64;
    if kind == 1 {
        peak_flag_b_store(c, 10, 10, 0xffff_ffff_ffff_fbff);
        return;
    }
    c.cycles += 2;
    if (kind as i64) < 2 {
        c.cycles += 2;
        if kind == 0 {
            peak_flag_b_store(c, 10, 9, 0xffff_ffff_ffff_fdff);
            return;
        }
        c.cycles += 2;
        return;
    }
    c.g[V0][0] = 2;
    c.cycles += 2;
    if kind == 2 {
        peak_flag_b_store(c, 9, 11, 0xffff_ffff_ffff_f7ff);
        return;
    }
    c.cycles += 2;
}

/// `void BENewPlayerInt_SetPeakFlagB(BENewPlayerInt *self, int index,
/// int character, int kind, int flag, int value)` (0x1468b0; C++
/// `cBENewPlayerInterface_setRivalEventLocked`): set one bit of a rider's
/// challenge word to `value & 1`. Kinds 0..=2 are bits 6..=8 when `flag` is
/// non-zero, bits 9..=11 when it is zero; other kinds change nothing.
pub fn be_new_player_int_set_peak_flag_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_68e0)?;

        let flag_zero = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if flag_zero {
            peak_flag_b_free_group(c);
        } else {
            peak_flag_b_time_group(c);
        }

        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One bit store of `BENewPlayerInt_SetEventTypeFlag`: `profileLib[profile]
/// [char]` (`A1` the profile offset, `S1` the character) gets bit `bit` set to
/// `value & 1` (`S2`), under `mask`. `cycles` is the case's own cost; the
/// common tail adds 6.
fn event_type_flag_store(c: &mut Cpu, cycles: u64, bit: u32, mask: u64) {
    c.cycles += cycles;
    c.g[V1][0] = 0xf88;
    c.g[V0][0] = 0x9b50;
    c.mult(0, V0, A1, V0);
    c.mult(0, V1, S1, V1);
    c.g[A0][0] = 0x4a_0000;
    c.g[A1][0] = c.g[S2][0] & 0x1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
    c.g[A2][0] = mask;
    c.g[A1][0] = c.g[A1][0] << bit;
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let word = Addr((c.g[V1][0] as u32).wrapping_add(CHALLENGE_WORD));
    c.g[V0][0] = word.ld(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    let updated = c.g[V0][0];
    word.sd(c, updated);
}

/// `void BENewPlayerInt_SetEventTypeFlag(BENewPlayerInt *self, int index,
/// int character, int event, int value)` (0x146c10): for event types 6..=11
/// set one bit of the rider's challenge word (bits 14..=19) to `value & 1`.
/// The bit comes from a jump table in the game data at `0x45a210`, indexed by
/// `event - 6`, whose targets name the case; other events change nothing.
/// Resumes at any point other than the entry through the generated body.
pub fn be_new_player_int_set_event_type_flag(c: &mut Cpu) {
    if c.pc != 0x0014_6c10 {
        return ssx3_ee::s01::f_00146c10__entries(c);
    }
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_6c38)?;

        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x6) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5df0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (cycles, bit, mask) = match target {
                0x0014_6c64 => (10, 14, 0xffff_ffff_ffff_bfff),
                0x0014_6c8c => (11, 15, 0xffff_ffff_ffff_7fff),
                0x0014_6cb8 => (11, 16, 0xffff_ffff_fffe_ffff),
                0x0014_6ce4 => (11, 17, 0xffff_ffff_fffd_ffff),
                0x0014_6d10 => (11, 18, 0xffff_ffff_fffb_ffff),
                0x0014_6d3c => (10, 19, 0xffff_ffff_fff7_ffff),
                _ => {
                    c.jump_out(target, 0x0014_6c10, 0x0014_6d94);
                    return Ok(());
                }
            };
            event_type_flag_store(c, cycles, bit, mask);
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_GetCharProfileData` (0x14ad28), called by `BE_InitRiderCharProfileBoltMap`.
const INIT_BOLT_MAP_PROFILE_DATA: Callee = Callee {
    addr: 0x0014_ad28,
    f: ssx3_ee::s01::f_0014ad28,
};

/// `BE_InitCharProfileBoltMap` (0x151a88).
const INIT_BOLT_MAP_INIT: Callee = Callee {
    addr: 0x0015_1a88,
    f: ssx3_ee::s01::f_00151a88,
};

/// `BENewPlayerInt_GetSlotField4` / `..Field8` (0x147698, 0x147718): read the
/// word at `offset` of the rider's slot record (`0x534fe0 + slot * 0x1c`).
fn oc456_slot_field_get(c: &mut Cpu, ret: u32, offset: u32) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = 0x53_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4fe0));
        GET_SLOT.call(c, ret)?;
        c.g[V1][0] = 0x1c;
        frame.restore_ra(c, 0);
        c.mult(0, V0, V0, V1);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(offset)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetSlotField4` / `..Field8` (0x1476d8, 0x147758): store
/// `value` (`A2`) at `offset` of the rider's slot record.
fn oc456_slot_field_set(c: &mut Cpu, ret: u32, offset: u32) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, ret)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let value = c.g[S0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(offset)).sw(c, value);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetSlotField4(BENewPlayerInt *self, int index)`
/// (0x147698): the word at +0x14 of the rider's slot record.
pub fn be_new_player_int_get_slot_field4(c: &mut Cpu) {
    oc456_slot_field_get(c, 0x0014_76b4, 0x14);
}

/// `void BENewPlayerInt_SetSlotField4(BENewPlayerInt *self, int index,
/// int value)` (0x1476d8): store `value` at +0x14 of the rider's slot record.
pub fn be_new_player_int_set_slot_field4(c: &mut Cpu) {
    oc456_slot_field_set(c, 0x0014_76f0, 0x14);
}

/// `int BENewPlayerInt_GetSlotField8(BENewPlayerInt *self, int index)`
/// (0x147718): the word at +0x18 of the rider's slot record.
pub fn be_new_player_int_get_slot_field8(c: &mut Cpu) {
    oc456_slot_field_get(c, 0x0014_7734, 0x18);
}

/// `void BENewPlayerInt_SetSlotField8(BENewPlayerInt *self, int index,
/// int value)` (0x147758): store `value` at +0x18 of the rider's slot record.
pub fn be_new_player_int_set_slot_field8(c: &mut Cpu) {
    oc456_slot_field_set(c, 0x0014_7770, 0x18);
}

/// `void BECharInt_GetCharData(void *dst, ..., int rider)` (0x148328): copy
/// the 15-byte record of the rider's character (`0x5308d8 + 15 * id`) to
/// `dst` (the caller's `a0`), with unaligned moves, and return `dst`.
pub fn be_char_int_get_char_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        crate::game::be::beintmission::BE_GET_RIDER_CHARACTER.call(c, 0x0014_8340)?;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V1][0] = 0x53_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8d8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.g[S0][0];
        let src = c.g[A0][0] as u32;
        let dst = c.g[S0][0] as u32;
        c.ldl(3, src.wrapping_add(7));
        c.ldr(3, src);
        c.lwl(5, src.wrapping_add(0xb));
        c.lwr(5, src.wrapping_add(8));
        c.g[A2][0] = Addr(src.wrapping_add(0xc)).lb(c);
        c.g[A3][0] = Addr(src.wrapping_add(0xd)).lb(c);
        c.sdl(3, dst.wrapping_add(7));
        c.sdr(3, dst);
        c.swl(5, dst.wrapping_add(0xb));
        c.swr(5, dst.wrapping_add(8));
        let v = c.g[A2][0];
        Addr(dst.wrapping_add(0xc)).sb(c, v);
        let v = c.g[A3][0];
        Addr(dst.wrapping_add(0xd)).sb(c, v);
        c.g[V1][0] = Addr(src.wrapping_add(0xe)).lb(c);
        let v = c.g[V1][0];
        Addr(dst.wrapping_add(0xe)).sb(c, v);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_EmptyStub_a078` (0x14a078): does nothing, returns.
pub fn be_empty_stub_a078(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void BE_InitRiderCharProfileBoltMap(a0, a1, int char)` (0x14acf8): get
/// the character's profile data (the caller's arguments pass through) and
/// initialise its bolt map with the character in `s0`.
pub fn be_init_rider_char_profile_bolt_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        INIT_BOLT_MAP_PROFILE_DATA.call(c, 0x0014_ad0c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        INIT_BOLT_MAP_INIT.call(c, 0x0014_ad18)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `V1 = profileLib base + char * 0xf88` for the character in `A0` (the
/// `cBELibrary_profileLib[0][char]` record the setters address). Returns it.
fn oc456_char_record_addr(c: &mut Cpu) -> u32 {
    c.g[V1][0] = 0xf88;
    c.g[V0][0] = 0x4a_0000;
    c.mult(0, V1, A0, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] as u32
}

/// `void BENewPlayerInt_SetMessageFlag(BENewPlayerInt *self, int index,
/// int bit, int set)` (0x147a30): set (`set != 0`) or clear bit `bit` of the
/// rider's 32-bit message-flag words (`+0xf08` of its profile record), the
/// word chosen as `bit / 32` (C division) and the bit as `bit & 31`.
pub fn be_new_player_int_set_message_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_7a5c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        GET_SLOT.call(c, 0x0014_7a68)?;

        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1f));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11)).lb(c);
        let word_index = ((c.g[A0][0] as u32) as i32) >> 5;
        c.g[A0][0] = sx(word_index as u32);
        c.g[V0][0] = 0x4a_0000;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 5);
        c.mult(0, A1, A1, A2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let flag_zero = c.g[S3][0] == 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf08));

        if flag_zero {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S2][0] as u32) & 31));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V1][0] = !c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        } else {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S2][0] as u32) & 31));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        }

        c.cycles += 8;
        let word = c.g[A0][0] as u32;
        let value = c.g[V0][0];
        Addr(word).sw(c, value);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `BENewPlayerInt_GetCharPairA` / `GetCharPairB`: the
/// profile and slot calls (returning to `ret_profile` and `ret_slot`), then
/// the word at `offset` of the character record selected by the rider's
/// character and the second argument (`S2 << 3`).
fn oc456_char_pair_get(c: &mut Cpu, ret_profile: u32, ret_slot: u32, offset: u32) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, ret_profile)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        GET_SLOT.call(c, ret_slot)?;

        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S0, S0, V1);
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[S2][0] = sx((c.g[S2][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S1, 0x20);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11)).lb(c);
        frame.restore_ra(c, 0);
        c.mult(0, V1, V1, A2);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(offset)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetCharPairA(BENewPlayerInt *self, int index, int pair)`
/// (0x147b20): the pair-A word of the rider's character record.
pub fn be_new_player_int_get_char_pair_a(c: &mut Cpu) {
    oc456_char_pair_get(c, 0x0014_7b44, 0x0014_7b50, 0xba0);
}

/// `int BENewPlayerInt_GetCharPairB(BENewPlayerInt *self, int index, int pair)`
/// (0x147bb0): the pair-B word of the rider's character record.
pub fn be_new_player_int_get_char_pair_b(c: &mut Cpu) {
    oc456_char_pair_get(c, 0x0014_7bd4, 0x0014_7be0, 0xba4);
}

/// `void BENewPlayerInt_SetDifficulty(BENewPlayerInt *self, int value,
/// int flag)` (0x147c40): store `value` in the first player's character record
/// at +0x280 (`flag != 0`) or +0x284 (`flag == 0`), and zero the halfword
/// after it.
pub fn be_new_player_int_set_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        crate::game::be::beintmission::BE_GET_RIDER_CHARACTER.call(c, 0x0014_7c60)?;
        let flag_zero = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if flag_zero {
            c.cycles += 7;
            let record = oc456_char_record_addr(c);
            let value = c.g[S1][0];
            Addr(record.wrapping_add(0x284)).sh(c, value);
            Addr(record.wrapping_add(0x286)).sh(c, 0);
        } else {
            c.cycles += 8;
            let record = oc456_char_record_addr(c);
            let value = c.g[S1][0];
            Addr(record.wrapping_add(0x280)).sh(c, value);
            Addr(record.wrapping_add(0x282)).sh(c, 0);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetDifficultyCounter(BENewPlayerInt *self, int kind)`
/// (0x147f10): the halfword counter at +0x282 of the first player's character
/// record for a non-zero `kind`, at +0x286 otherwise.
pub fn be_new_player_int_get_difficulty_counter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        crate::game::be::beintmission::BE_GET_RIDER_CHARACTER.call(c, 0x0014_7f28)?;
        let nonzero = c.g[S0][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        if nonzero {
            c.cycles += 6;
            let record = oc456_char_record_addr(c);
            c.g[V0][0] = Addr(record.wrapping_add(0x282)).lh(c);
        } else {
            c.cycles += 7;
            let record = oc456_char_record_addr(c);
            c.g[V0][0] = Addr(record.wrapping_add(0x286)).lh(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// `cBELibrary_profileLib` as the store addresses it (`0x4a0000 + 0x6ca8`).
    const PROFILE_LIB: u32 = 0x004a_6ca8;

    /// Byte stride of one profile record in `profileLib`.
    const PROFILE_STRIDE: u32 = 0x9b50;

    /// Byte stride of one character record inside a profile.
    const CHAR_STRIDE: u32 = 0xf88;

    /// Writes the rider `index` slot word (`0x5305b0 + 4 * index`), the
    /// profile words `BE_GetRiderProfile` reads for `slot`, and the slot's
    /// character byte. Returns (profile, character).
    fn oc456_rider_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        index: u32,
        slot: u32,
        profile_two: bool,
    ) -> (u32, i8) {
        c.bus.write32(0x0053_05b0 + 4 * index, slot);
        let q = 0x0053_5b20 + slot * 0x1c;
        let w_c = if profile_two { 0xffff_ffff } else { r.next_u32() };
        let w_10 = r.next_u32();
        c.bus.write32(q + 0xc, w_c);
        c.bus.write32(q + 0x10, w_10);
        let profile = if w_c == 0xffff_ffff { 2 } else { w_10 & 1 };
        let ch = r.next_u32() as u8;
        c.bus.write8(0x0053_4fe0 + slot * 0x1c + 0x11, ch);
        (profile, ch as i8)
    }

    /// Address of the 64-bit challenge word of `profileLib[profile][ch]`.
    fn oc456_challenge_word(profile: u32, ch: i8) -> u32 {
        PROFILE_LIB
            .wrapping_add(CHALLENGE_WORD)
            .wrapping_add(profile.wrapping_mul(PROFILE_STRIDE))
            .wrapping_add((ch as i32 as u32).wrapping_mul(CHAR_STRIDE))
    }

    #[test]
    fn be_new_player_int_set_peak_flag_matches() {
        check(
            ssx3_ee::s01::f_00146320__generated,
            be_new_player_int_set_peak_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x1463_2000 ^ seed as u64);
                scramble(c, &mut r);
                // Rider index (the slot word at 0x5305b0 + 4 * index), the slot
                // it names, and the profile record that slot's callees read.
                let index = (seed / 3) % 4;
                let slot = seed % 3;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = if seed % 7 == 0 {
                    r.next_u32() as u64
                } else {
                    (seed % 5) as u64
                };
                c.g[A3][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(0x0053_05b0 + 4 * index, slot);
                let q = 0x0053_5b20 + slot * 0x1c;
                let w_c = if seed % 4 == 0 { 0xffff_ffff } else { r.next_u32() };
                let w_10 = r.next_u32();
                c.bus.write32(q + 0xc, w_c);
                c.bus.write32(q + 0x10, w_10);
                let profile = if w_c == 0xffff_ffff { 2 } else { w_10 & 1 };
                let ch = r.next_u32() as u8;
                c.bus.write8(0x0053_4fe0 + slot * 0x1c + 0x11, ch);
                let word = PROFILE_LIB
                    .wrapping_add(CHALLENGE_WORD)
                    .wrapping_add(profile.wrapping_mul(PROFILE_STRIDE))
                    .wrapping_add((ch as i8 as i32 as u32).wrapping_mul(CHAR_STRIDE));
                c.bus.write32(word, r.next_u32());
                c.bus.write32(word + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_current_peak_flag_b_matches() {
        check(
            ssx3_ee::s01::f_001466a0__generated,
            be_new_player_int_get_current_peak_flag_b,
            300,
            &|c, seed| {
                let mut r = Rng(0x1466_a000 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed / 3) % 4;
                let slot = seed % 3;
                c.g[A1][0] = index as u64;
                let (profile, ch) = oc456_rider_fixture(c, &mut r, index, slot, seed % 4 == 0);
                // Location (raceDB[16]) and its kind, and the flag byte
                // raceDB[18] (5 on half the seeds).
                let loc = (seed / 2) % 3;
                c.bus.write32(0x0053_05f0, loc);
                c.bus.write8(0x0053_05f8, if seed % 2 == 0 { 5 } else { r.next_u32() as u8 });
                let kind = [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize];
                c.bus.write32(0x0043_d950 + loc * 100 + 0x54, kind);
                c.bus.write32(oc456_challenge_word(profile, ch), r.next_u32());
                c.bus.write32(oc456_challenge_word(profile, ch) + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_set_peak_flag_b_matches() {
        check(
            ssx3_ee::s01::f_001468b0__generated,
            be_new_player_int_set_peak_flag_b,
            300,
            &|c, seed| {
                let mut r = Rng(0x1468_b000 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed / 3) % 4;
                let slot = seed % 3;
                let (profile, _) = oc456_rider_fixture(c, &mut r, index, slot, seed % 4 == 0);
                let ch = (seed / 5) % 4;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = [0u64, 1, 2, 3, u64::MAX][(seed % 5) as usize];
                c.g[T0][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.g[T1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let word = oc456_challenge_word(profile, ch as i8);
                c.bus.write32(word, r.next_u32());
                c.bus.write32(word + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_set_event_type_flag_matches() {
        check(
            ssx3_ee::s01::f_00146c10__generated,
            be_new_player_int_set_event_type_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x146c_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0014_6c10;
                let index = (seed / 3) % 4;
                let slot = seed % 3;
                let (profile, _) = oc456_rider_fixture(c, &mut r, index, slot, seed % 4 == 0);
                let ch = (seed / 5) % 4;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = ch as u64;
                // Event types 6..=13: 12 and 13 fall outside the table.
                c.g[A3][0] = (6 + seed % 8) as u64;
                c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                // The jump table at 0x45a210 holds the six case addresses.
                let cases = [
                    0x0014_6c64u32,
                    0x0014_6c8c,
                    0x0014_6cb8,
                    0x0014_6ce4,
                    0x0014_6d10,
                    0x0014_6d3c,
                ];
                for (i, case) in cases.iter().enumerate() {
                    c.bus.write32(0x0045_a210 + 4 * i as u32, *case);
                }
                let word = oc456_challenge_word(profile, ch as i8);
                c.bus.write32(word, r.next_u32());
                c.bus.write32(word + 4, r.next_u32());
            },
        );
    }

    /// The rider's slot word and record, for the slot-field tests.
    fn oc456_slot_setup(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u64, u32) {
        let index = seed % 4;
        let slot = (seed / 4) % 3;
        c.g[A1][0] = index as u64;
        c.bus.write32(0x0053_05b0 + 4 * index, slot);
        c.bus.write32(0x0053_4fe0 + slot * 0x1c + 0x14, r.next_u32());
        c.bus.write32(0x0053_4fe0 + slot * 0x1c + 0x18, r.next_u32());
        (index as u64, slot)
    }

    #[test]
    fn be_new_player_int_set_message_flag_matches() {
        check(
            ssx3_ee::s01::f_00147a30__generated,
            be_new_player_int_set_message_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x147a_3000 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed / 3) % 4;
                let slot = seed % 3;
                let (profile, ch) = oc456_rider_fixture(c, &mut r, index, slot, seed % 4 == 0);
                c.g[A1][0] = index as u64;
                // Bits -35..=34 cover both signs of the word index.
                let bit = (seed % 70) as i32 - 35;
                c.g[A2][0] = bit as i64 as u64;
                c.g[A3][0] = (seed % 2) as u64;
                let w = bit / 32;
                let addr = PROFILE_LIB
                    .wrapping_add(profile.wrapping_mul(PROFILE_STRIDE))
                    .wrapping_add((ch as i32 as u32).wrapping_mul(CHAR_STRIDE))
                    .wrapping_add(0xf08)
                    .wrapping_add((w * 4) as u32);
                c.bus.write32(addr, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_char_pair_a_matches() {
        check_char_pair(
            ssx3_ee::s01::f_00147b20__generated,
            be_new_player_int_get_char_pair_a,
            0xba0,
            0x1200,
        );
    }

    #[test]
    fn be_new_player_int_get_char_pair_b_matches() {
        check_char_pair(
            ssx3_ee::s01::f_00147bb0__generated,
            be_new_player_int_get_char_pair_b,
            0xba4,
            0x1300,
        );
    }

    /// Both pair getters: the record word at `offset` plus the pair index.
    fn check_char_pair(generated: GuestFn, hand: GuestFn, offset: u32, salt: u64) {
        check(generated, hand, 300, &|c, seed| {
            let mut r = Rng(salt ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed / 3) % 4;
            let slot = seed % 3;
            let (profile, ch) = oc456_rider_fixture(c, &mut r, index, slot, seed % 4 == 0);
            c.g[A1][0] = index as u64;
            let pair = r.next_u32() % 16;
            c.g[A2][0] = pair as u64;
            let addr = PROFILE_LIB
                .wrapping_add(profile.wrapping_mul(PROFILE_STRIDE))
                .wrapping_add((ch as i32 as u32).wrapping_mul(CHAR_STRIDE))
                .wrapping_add(pair << 3)
                .wrapping_add(offset);
            c.bus.write32(addr, r.next_u32());
        });
    }

    #[test]
    fn be_new_player_int_set_difficulty_matches() {
        check(
            ssx3_ee::s01::f_00147c40__generated,
            be_new_player_int_set_difficulty,
            200,
            &|c, seed| {
                let mut r = Rng(0x147c_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn be_new_player_int_get_difficulty_counter_matches() {
        check(
            ssx3_ee::s01::f_00147f10__generated,
            be_new_player_int_get_difficulty_counter,
            200,
            &|c, seed| {
                let mut r = Rng(0x147f_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(0x0045_0000 + seed * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_slot_field4_matches() {
        check(
            ssx3_ee::s01::f_00147698__generated,
            be_new_player_int_get_slot_field4,
            200,
            &|c, seed| {
                let mut r = Rng(0x1476_9800 ^ seed as u64);
                scramble(c, &mut r);
                oc456_slot_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn be_new_player_int_get_slot_field8_matches() {
        check(
            ssx3_ee::s01::f_00147718__generated,
            be_new_player_int_get_slot_field8,
            200,
            &|c, seed| {
                let mut r = Rng(0x1477_1800 ^ seed as u64);
                scramble(c, &mut r);
                oc456_slot_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn be_new_player_int_set_slot_field4_matches() {
        check(
            ssx3_ee::s01::f_001476d8__generated,
            be_new_player_int_set_slot_field4,
            200,
            &|c, seed| {
                let mut r = Rng(0x1476_d800 ^ seed as u64);
                scramble(c, &mut r);
                oc456_slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn be_new_player_int_set_slot_field8_matches() {
        check(
            ssx3_ee::s01::f_00147758__generated,
            be_new_player_int_set_slot_field8,
            200,
            &|c, seed| {
                let mut r = Rng(0x1477_5800 ^ seed as u64);
                scramble(c, &mut r);
                oc456_slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn be_char_int_get_char_data_matches() {
        check(
            ssx3_ee::s01::f_00148328__generated,
            be_char_int_get_char_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x1483_2800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // Character ids and the 15-byte records they index.
                c.bus.write32(0x0053_05b0 + 4 * (seed % 4), r.next_u32() % 8);
                for i in 0..16 {
                    c.bus.write8(0x0053_08d8 + i, r.next_u32() as u8);
                }
                c.bus.write32(0x0053_08d8 + 15 * 2 + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn be_empty_stub_a078_matches() {
        check(
            ssx3_ee::s01::f_0014a078__generated,
            be_empty_stub_a078,
            10,
            &|c, seed| {
                let mut r = Rng(0x14a0_7800 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn be_init_rider_char_profile_bolt_map_matches() {
        check(
            ssx3_ee::s01::f_0014acf8__generated,
            be_init_rider_char_profile_bolt_map,
            100,
            &|c, seed| {
                let mut r = Rng(0x14ac_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = (seed % 4) as u64;
            },
        );
    }
}
