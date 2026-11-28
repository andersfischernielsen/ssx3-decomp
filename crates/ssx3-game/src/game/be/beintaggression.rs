//! `src/be/beintaggression.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::be::beintmission::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_m2;
use crate::game::be::beintmission::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_m2;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_5ab0,
        be_aggression_int_get_rival_state,
        "BEAggressionInt_GetRivalState",
    ),
    (
        0x0015_c7e0,
        be_aggression_int_destroy_instance,
        "BEAggressionInt_DestroyInstance",
    ),
    (
        0x0015_5bf0,
        be_aggression_int_add_aggression,
        "BEAggressionInt_AddAggression",
    ),
    (
        0x0015_60a0,
        be_aggression_int_save_state,
        "BEAggressionInt_SaveState",
    ),
    (
        0x0015_5e58,
        be_aggression_int_decay_all,
        "BEAggressionInt_DecayAll",
    ),
    (
        0x0015_61b0,
        be_aggression_int_load_state,
        "BEAggressionInt_LoadState",
    ),
    (
        0x0015_62b8,
        be_library_switch_phase,
        "BELibrary_SwitchPhase",
    ),
    (
        0x0015_5aa0,
        be_aggression_int_commit_00155aa0,
        "BEAggressionInt_Commit",
    ),
    (
        0x0015_5aa8,
        be_aggression_int_reset_00155aa8,
        "BEAggressionInt_Reset",
    ),
];

/// `void BEAggressionInt_DestroyInstance(void)` (0x15c7e0).
pub fn be_aggression_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ea4));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c7f4)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ea4), 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const NEW_RACE_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_48d8,
    f: ssx3_ee::s01::f_001448d8,
};

pub(crate) const F_00145750: Callee = Callee {
    addr: 0x0014_5750,
    f: ssx3_ee::s01::f_00145750,
};

/// `int BEAggressionInt_GetRivalState(BEAggressionInt *self, int rider,
/// int rival)` (0x155ab0): compare the rival's current race position with the
/// rider's and return 2/… from the aggression table, or 2 when they are equal.
pub fn be_aggression_int_get_rival_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_5ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5ae0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5aec)?;
        c.g[S1][0] = c.g[V0][0];
        NEW_RACE_GET_INSTANCE.call(c, 0x0015_5af4)?;
        c.g[A0][0] = c.g[V0][0];
        F_00145750.call(c, 0x0015_5afc)?;
        let t = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = RIDER_STRIDE as u64;
        if t {
            c.cycles += 1;
            c.g[V0][0] = 0x2;
        } else {
            c.cycles += 12;
            c.g[A1][0] = 0x9b50;
            c.mult(0, A0, S1, A0);
            c.mult(0, A1, S2, A1);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
            c.g[V1][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbc1));
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

/// `void BEAggressionInt::SaveState(cStream* s)` (0x1560a0): writes the
/// rider count (0x5305e0) through a stack copy, then for each rider the ten
/// 3-byte aggression entries of its profile and character in the table at
/// 0x4a6ca8 (`[profile * 0x9b50 + character * 0xf88 + 0xbc1 + 3 * j]`), all
/// with `s->Write` (vtable +8).
pub fn be_aggression_int_save_state(c: &mut Cpu) {
    const GET_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };
    const GET_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::BE_GetRiderCharacter,
    };
    const RIDER_COUNT: u32 = 0x0053_05e0;
    const TABLE: u32 = 0x004a_6ca8;
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S1, 0x90);
        c.g[A2][0] = 0x4;
        frame.save(c, S3, 0x70);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        // n = rider count; s->Write(&n, 4)
        let n = c.lw(RIDER_COUNT);
        c.g[T0][0] = n;
        let stream = c.g[S3][0] as u32;
        let vt = c.lw(stream);
        c.g[A3][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.sw(frame.at(c, 0), n);
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0015_60fc)?;
        let count = c.lw(frame.at(c, 0));
        c.g[V0][0] = 0x004a_0000;
        if count as i64 > 0 {
            c.cycles += 4;
            c.g[FP][0] = 0x9b50;
            c.g[S7][0] = sx(TABLE);
            c.g[S6][0] = 0xf88;
            c.g[A0][0] = c.g[S1][0];
            loop {
                // profile row of rider s1
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                GET_PROFILE.call(c, 0x0015_6120)?;
                c.g[S2][0] = 0xbc1;
                c.mult(0, S4, V0, FP);
                c.g[S0][0] = 0x9;
                c.g[A0][0] = c.g[S1][0];
                loop {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    GET_CHARACTER.call(c, 0x0015_6138)?;
                    c.mult(0, V1, V0, S6);
                    c.g[A2][0] = 0x3;
                    let row = (c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32);
                    let stream = c.g[S3][0] as u32;
                    let vt = c.lw(stream);
                    c.g[V1][0] = vt;
                    let row = (c.g[S4][0] as u32).wrapping_add(row);
                    c.g[V0][0] = sx(row);
                    c.g[A1][0] = sx(row.wrapping_add(c.g[S2][0] as u32));
                    let delta = c.lh((vt as u32).wrapping_add(0x8));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(3));
                    let write = c.lw((vt as u32).wrapping_add(0xc));
                    c.g[A3][0] = write;
                    c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
                    call_ptr(c, write as u32, 0x0015_6164)?;
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    c.g[A0][0] = c.g[S1][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_6130);
                    }
                }
                c.cycles += 5;
                let count = c.lw(frame.at(c, 0));
                c.g[S1][0] = c.g[S5][0];
                let more = ((c.g[S1][0] as i64) < (count as i64)) as u64;
                c.g[V0][0] = more;
                if more == 0 {
                    break;
                }
                c.g[A0][0] = c.g[S1][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0015_6118);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The aggression records: `[profile][character]` blocks of 0xf88 bytes
/// (0x9b50 per profile), with a 3-byte `{state, level, points}` entry per
/// rival character at +0xbc1.
pub(crate) const AGGRESSION_RECORDS: u32 = 0x004a_6ca8;

pub(crate) const AGGRESSION_PROFILE_SIZE: u64 = 0x9b50;

pub(crate) const AGGRESSION_CHAR_SIZE: u64 = 0xf88;

pub(crate) const AGGRESSION_STATE: u32 = 0xbc1;

pub(crate) const AGGRESSION_LEVEL: u32 = 0xbc2;

pub(crate) const AGGRESSION_POINTS: u32 = 0xbc3;

/// Set while aggression messages are suppressed.
pub(crate) const AGGRESSION_MESSAGES_OFF: u32 = 0x0053_05f9;

/// Where [`be_aggression_int_add_aggression`]'s level-up rules continue.
pub(crate) enum AggressionExit {
    /// Reset to level 1 / 5 points (`0x155dcc`), then as `Clamped`.
    Floor,
    /// A clamped level (`0x155de8`): reload a0, then as `Kept`.
    Clamped,
    /// `0x155dec`: reload a1.
    Kept,
    /// Store (`0x155df0`).
    Store,
}

/// `void BEAggressionInt_AddAggression(BEAggressionInt* self, int rider,
/// int rival, int event)` (0x155bf0): add the event's aggression points
/// (events 0..3: 1, 2, 4, 6) to `rider`'s record of `rival`; the level is
/// points / 5. On a level-up the player (rider 0) gets the aggression
/// message unless messages are off, and the record's state clamps the
/// level: state 0 caps at level 2 (10 points), state 1 turns into state 3
/// at level 4 (level 2, 10 points), state 2 stays within 1..3 and state 3
/// within 1..4 (5 points per level). Frame 0xa0: ra at 0, fp at 0x10,
/// s7..s0 at 0x20..0x90.
pub fn be_aggression_int_add_aggression(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S4][0];
        Callee {
            addr: 0x0014_a0e0,
            f: ee::s01::BE_GetRiderProfile,
        }
        .call(c, 0x0015_5c30)?;
        c.g[S7][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        Callee {
            addr: 0x0014_a080,
            f: ee::s01::BE_GetRiderCharacter,
        }
        .call(c, 0x0015_5c3c)?;
        c.g[S6][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0014_a080,
            f: ee::s01::BE_GetRiderCharacter,
        }
        .call(c, 0x0015_5c48)?;

        // The record: profile s7, character s6, rival character s5.
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        c.mult(0, A0, S6, A0);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, A1, S7, A1);
        let rival3 = (c.g[S5][0] as u32).wrapping_mul(3);
        c.g[FP][0] = 0x4a_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(rival3
            .wrapping_add(c.g[A0][0] as u32)
            .wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = sx(AGGRESSION_RECORDS);
        c.g[A2][0] = 1;
        let rec = AGGRESSION_RECORDS.wrapping_add(c.g[V0][0] as u32);
        c.g[A1][0] = sx(rec);
        c.g[V1][0] = sx(rec);
        let rec = Addr(rec);
        c.g[A0][0] = rec.off(AGGRESSION_LEVEL).lb(c);
        c.g[S1][0] = rec.off(AGGRESSION_POINTS).lb(c);
        c.g[S3][0] = rec.off(AGGRESSION_STATE).lb(c);

        // Points for the event.
        let event = c.g[S2][0] as i64;
        let (add, cycles) = match event {
            1 => (2, 4),
            0 => (1, 9),
            2 => (4, 9),
            3 => (6, 10),
            e if e < 0 => (0, 7),
            _ => (0, 9),
        };
        c.cycles += cycles;
        c.g[A3][0] = add;
        c.g[V0][0] = 5;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(add as u32));

        c.cycles += 3;
        c.div(0, S1, V0);
        c.cycles += 5;
        c.g[S0][0] = c.lo[0];
        c.g[S2][0] = c.g[S0][0];
        let level_up = (c.g[A0][0] as i64) < (c.g[S2][0] as i64);
        c.g[V0][0] = level_up as u64;
        let exit = if !level_up {
            c.g[A0][0] = AGGRESSION_CHAR_SIZE;
            AggressionExit::Kept
        } else {
            aggression_level_up(c)?
        };
        if let AggressionExit::Floor = exit {
            c.cycles += 3;
            c.g[S2][0] = 1;
            c.g[S1][0] = 5;
        }
        if matches!(exit, AggressionExit::Floor | AggressionExit::Clamped) {
            c.cycles += 1;
            c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        }
        if !matches!(exit, AggressionExit::Store) {
            c.cycles += 1;
            c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        }

        // Store the record.
        c.cycles += 25;
        c.mult(0, A0, S6, A0);
        c.mult(0, A1, S7, A1);
        let rival3 = (c.g[S5][0] as u32).wrapping_mul(3);
        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6ca8));
        let off = rival3
            .wrapping_add(c.g[A0][0] as u32)
            .wrapping_add(c.g[A1][0] as u32);
        c.g[V0][0] = sx(off);
        let rec = (c.g[V1][0] as u32).wrapping_add(off);
        c.g[A0][0] = sx(rec);
        c.g[A1][0] = sx(rec);
        c.g[V1][0] = sx(rec);
        let rec = Addr(rec);
        let (points, level, state) = (c.g[S1][0], c.g[S2][0], c.g[S3][0]);
        rec.off(AGGRESSION_POINTS).sb(c, points);
        rec.off(AGGRESSION_LEVEL).sb(c, level);
        rec.off(AGGRESSION_STATE).sb(c, state);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The level-up part of [`be_aggression_int_add_aggression`]: the message
/// for the player, then the state's level rules (level s0 = s2, points s1,
/// state s3).
pub(crate) fn aggression_level_up(c: &mut Cpu) -> Result<AggressionExit, crate::call::Unwound> {
    use AggressionExit::*;
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S4][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = Addr(AGGRESSION_MESSAGES_OFF).lb(c);
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S1][0];
            // MessageCenter_CheckAggressionMessage(rival, points)
            call_ptr(c, 0x001e_2a08, 0x0015_5d38)?;
            c.g[V0][0] = 1;
        }
    }
    let state = c.g[S3][0] as i64;
    let level = c.g[S0][0] as i64;
    c.cycles += 2;
    c.g[V0][0] = (state < 2) as u64;
    if state == 1 {
        c.cycles += 3;
        c.g[V0][0] = (level < 4) as u64;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 4 {
            c.cycles += 4;
            c.g[S2][0] = 2;
            c.g[S1][0] = 10;
            c.g[S3][0] = 3;
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[V0][0] = 2;
    if state < 2 {
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if state == 0 {
            c.cycles += 3;
            c.g[V0][0] = (level < 3) as u64;
            c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
            if level < 3 {
                return Ok(Store);
            }
            c.cycles += 3;
            c.g[S2][0] = 2;
            c.g[S1][0] = 10;
            return Ok(Clamped);
        }
        c.cycles += 2;
        c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        return Ok(Store);
    }
    c.cycles += 2;
    c.g[V0][0] = 3;
    if state == 2 {
        c.cycles += 2;
        c.g[V0][0] = (level < 4) as u64;
        if level <= 0 {
            return Ok(Floor);
        }
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 4 {
            c.cycles += 3;
            c.g[S2][0] = 3;
            c.g[S1][0] = 15;
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[A0][0] = AGGRESSION_CHAR_SIZE;
    if state == 3 {
        c.cycles += 2;
        c.g[V0][0] = (level < 5) as u64;
        if level <= 0 {
            return Ok(Floor);
        }
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 5 {
            c.cycles += 2;
            c.g[S2][0] = 4;
            c.g[S1][0] = 20;
            return Ok(Clamped);
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
    Ok(Store)
}

/// The aggression state machine of `BEAggressionInt_DecayAll`: given the
/// state `st` (v[0]), the level `lvl` (points / 5) and the points `pts` of one
/// aggression slot, returns `(cycles of the compiled compare chain, st, lvl,
/// pts)`. State 0 caps the level at 2 (10 points), 1 sends level >= 4 to state 3
/// (level 2, 10 points), 2 raises level 0 to 1 and caps level >= 4 at 3 (15
/// points), 3 turns level 0 into state 1 (level 3, 15 points) and caps level >= 5
/// at 4 (20 points); other states stay as they are. The cycle counts are the
/// generated block charges of the compiled if-chain.
pub(crate) fn be_aggression_step(st: i64, lvl: i64, pts: u64) -> (u64, i64, i64, u64) {
    match st {
        1 if lvl >= 4 => (11, 3, 2, 10),
        1 => (7, st, lvl, pts),
        0 if lvl < 3 => (10, st, lvl, pts),
        0 => (15, st, 2, 10),
        2 if lvl <= 0 => (13, st, 1, pts),
        2 if lvl < 4 => (12, st, lvl, pts),
        2 => (17, st, 3, 15),
        3 if lvl == 0 => (17, 1, 3, 15),
        3 if lvl < 5 => (14, st, lvl, pts),
        3 => (17, st, 4, 20),
        _ if st < 0 => (9, st, lvl, pts),
        _ => (11, st, lvl, pts),
    }
}

/// `void BEAggressionInt_DecayAll(BEAggressionInt* self)` (0x155e58): for every
/// player of the race (count at `0x5305e0 + 0x30`) and each of the ten
/// aggression slots of the rider's character profile (3 bytes per slot at
/// `0x4a7869 + profile * 0x9b50 + char * 0xf88`: state, level, points), reduces
/// the points by 3 (not below 0), tells the HUD (`func_001E2A08(slot, points)`,
/// unless the race's byte at `+0x49` is set), then applies
/// [`be_aggression_step`] and writes state, points and level back.
pub fn be_aggression_int_decay_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = 0x53_0000;
        frame.save(c, S4, 0x60);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.g[S4][0];
            'players: loop {
                c.cycles += 8;
                c.g[S6][0] = 0;
                call_ptr(c, 0x0014_a0e0, 0x0015_5ea8)?; // BE_GetRiderProfile
                c.g[FP][0] = 0;
                c.g[A0][0] = 0x9b50;
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.mult(0, S5, V0, A0);
                let slot = frame.at(c, 0);
                c.sw(slot, c.g[V1][0]);
                c.g[S7][0] = 0;
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0014_a080, 0x0015_5ec8)?; // BE_GetRiderCharacter
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = 0xf88;
                    c.mult(0, V1, S3, V1);
                    c.g[A0][0] = 5;
                    c.cycles += 22;
                    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[A0][0] = 0x4a_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x786b));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.g[V1][0] = 0x53_0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                    c.g[A2][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = 0x4a_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7869));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lb(c.g[V1][0] as u32);
                    c.g[V1][0] = 5;
                    c.g[S2][0] = c.lb(c.g[V0][0] as u32);
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_sub(3));
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[A1][0] == 0 {
                        c.g[S1][0] = 0;
                    }
                    c.div(0, S1, V1);
                    c.g[V0][0] = c.lo[0];
                    let hud_off = c.g[A2][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if !hud_off {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x001e_2a08, 0x0015_5f44)?; // HUD aggression update
                    }
                    let (cyc, st, lvl, pts) =
                        be_aggression_step(c.g[S2][0] as i64, c.g[S0][0] as i64, c.g[S1][0]);
                    c.cycles += cyc;
                    c.g[S2][0] = st as u64;
                    c.g[S0][0] = lvl as u64;
                    c.g[S1][0] = pts;
                    c.g[V1][0] = 0xf88;
                    c.g[A0][0] = 0x4a_0000;
                    c.cycles += 19;
                    c.mult(0, V1, S3, V1);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7869));
                    c.g[A2][0] = 0x4a_0000;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x786b));
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(3));
                    c.g[A1][0] = ((c.g[S6][0] as i64) < 10) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(3));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sb(c.g[V1][0] as u32, c.g[S2][0]);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = 0x4a_0000;
                    c.sb(c.g[A0][0] as u32, c.g[S1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x786a));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let more_slots = c.g[A1][0] != 0;
                    c.sb(c.g[V0][0] as u32, c.g[S0][0]);
                    if more_slots {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_5ec0);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 7;
                c.g[A0][0] = 0x53_0000;
                let slot = frame.at(c, 0);
                c.g[S4][0] = c.lw(slot);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b0));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more_players = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S4][0];
                if more_players {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_5ea0);
                    }
                    continue 'players;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BEAggressionInt_LoadState(void *self, void *stream)` (0x1561b0):
/// for each rider, read the callee-reported counts (`func(stream, 4, &count)`
/// through the stream vtable) and accumulate the per-character records.
pub fn be_aggression_int_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xb0);
        'b: {
            c.g[A2][0] = 0x4;
            frame.save(c, S1, 0x90);
            frame.save(c, S3, 0x70);
            c.g[S1][0] = 0;
            frame.save(c, S0, 0xa0);
            c.g[S3][0] = c.g[A1][0];
            frame.save(c, S2, 0x80);
            c.g[A1][0] = c.g[SP][0];
            frame.save(c, S4, 0x60);
            frame.save(c, S5, 0x50);
            frame.save(c, S6, 0x40);
            frame.save(c, S7, 0x30);
            frame.save(c, FP, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = Addr(c.g[S3][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0015_6200)?;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = 0x4a0000;
            if none {
                break 'b;
            }
            c.cycles += 5;
            c.g[FP][0] = 0x9b50;
            c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[S6][0] = 0xf88;
            c.g[A0][0] = c.g[S1][0];
            'outer: loop {
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.g[S1][0];
                BE_GET_RIDER_PROFILE_m2.call(c, 0x0015_6228)?;
                c.g[S2][0] = 0xbc1;
                c.mult(0, S4, V0, FP);
                c.g[S0][0] = 0x9;
                c.g[A0][0] = c.g[S1][0];
                'inner: loop {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = c.g[S1][0];
                    BE_GET_RIDER_CHARACTER_m2.call(c, 0x0015_6240)?;
                    c.mult(0, V1, V0, S6);
                    c.g[A2][0] = 0x3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.g[V1][0] = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3));
                    c.g[A3][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
                    let target = c.g[A3][0] as u32;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0015_626c)?;
                    if (c.g[S0][0] as i64) >= 0 {
                        c.g[A0][0] = c.g[S1][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_6238);
                        }
                        continue 'inner;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
                    c.g[S1][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S1][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_6220);
                        }
                        continue 'outer;
                    }
                    break 'b;
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BELibrary_SwitchPhase(int phase)` (0x1562b8): run the outgoing
/// phase's exit hook (vtable slot +0x18/+0x1c through the table at
/// 0x5308a8), store the new phase in the global at gp-0x1efc, then run the
/// incoming phase's enter hook (slot +0x20/+0x24).
pub fn be_library_switch_phase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1efc)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = 0x530000;
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8a8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        let exit_hook = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, exit_hook, 0x0015_62f8)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        let phase = c.g[V1][0];
        Addr((c.g[GP][0] as u32).wrapping_sub(0x1efc)).sw(c, phase);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        let enter_hook = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, enter_hook, 0x0015_6320)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEAggressionInt_Commit` (0x0015_5aa0): straight-line leaf. Mechanical port of the generated body.
pub fn be_aggression_int_commit_00155aa0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEAggressionInt_Reset` (0x0015_5aa8): straight-line leaf. Mechanical port of the generated body.
pub fn be_aggression_int_reset_00155aa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;

    #[test]
    fn be_aggression_int_get_rival_state_matches() {
        check(
            ssx3_ee::s01::f_00155ab0,
            be_aggression_int_get_rival_state,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5ab0, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x400);
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }

    use crate::difftest::fakes::{covered, FAKE};
    use crate::difftest::RA as SENTINEL;

    /// `BEAggressionInt_SaveState`: rider count -1..4 at 0x5305e0, the
    /// rider index table (0x5305b0) pointing at rider records (0x535b20,
    /// 0x1c each) with profile -1 / random flags and a character byte;
    /// the stream's `Write` (vtable +8) is a fake (logs the registers,
    /// sometimes unwinds). BE_GetRiderProfile/Character run for real.
    #[test]
    fn be_aggression_int_save_state_matches() {
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s01::f_001560a0__generated,
            be_aggression_int_save_state,
            200,
            &|c, seed| {
                let n = (seed % 6) as i32 - 1;
                let calls = 1 + 10 * n.max(0) as u32;
                let mut r = machine(c, seed, 0x1560a0, calls);
                c.pc = 0x0015_60a0;
                c.bus.write32(0x0053_05e0, n as u32);
                for i in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for k in 0..8u32 {
                    let rec = 0x0053_5b20 + 0x1c * k;
                    c.bus.write32(
                        rec + 0xc,
                        if r.next_u32() % 3 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32()
                        },
                    );
                    c.bus.write32(rec + 0x10, r.next_u32());
                }
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// `BEAggressionInt_AddAggression`: riders 0..2 (rider 0 is the player:
    /// the aggression message, suppressed or not, with the message centre's
    /// short path), profiles 0..2 (one by the "-1" flag), events -1..4,
    /// records in every state with levels and points around each clamp.
    #[test]
    fn be_aggression_int_add_aggression_matches() {
        const SLOTS: u32 = 0x0053_05b0;
        const CHARS: u32 = 0x0053_5b20;
        const WORLD: u32 = DATA + 0x5_0000;
        check(
            ee::s01::f_00155bf0__generated,
            be_aggression_int_add_aggression,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_5bf0, 0);
                let pick = |r: &mut Rng, v: &[i32]| v[(r.next_u32() as usize) % v.len()];
                // Rider slots and character records.
                for idx in 0..4u32 {
                    c.bus.write32(SLOTS + 4 * idx, idx);
                    let rec = CHARS + 0x1c * idx;
                    let flag = pick(&mut r, &[-1, 0, 1]) as u32;
                    c.bus.write32(rec + 0xc, flag);
                    c.bus.write32(rec + 0x10, r.next_u32() % 4);
                    c.bus.write8(rec + 0x11, (r.next_u32() % 5) as u8);
                }
                let rider = if seed % 3 == 0 { 0 } else { r.next_u32() % 3 };
                let rival = 1 + r.next_u32() % 3;
                // Fill the records this pair can reach (every profile).
                for profile in 0..3u32 {
                    for ch in 0..5u32 {
                        for rv in 0..5u32 {
                            let e = AGGRESSION_RECORDS + profile * 0x9b50 + ch * 0xf88 + rv * 3;
                            c.bus
                                .write8(e + 0xbc1, pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u8);
                            c.bus
                                .write8(e + 0xbc2, pick(&mut r, &[-1, 0, 1, 2, 3, 4, 5]) as u8);
                            c.bus.write8(
                                e + 0xbc3,
                                pick(&mut r, &[-3, 0, 4, 9, 14, 15, 19, 24, 30]) as u8,
                            );
                        }
                    }
                }
                c.bus
                    .write8(AGGRESSION_MESSAGES_OFF, (r.next_u32() % 2) as u8);
                // MessageCenter_CheckAggressionMessage's short path.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD + 0x100);
                c.bus.write32(WORLD + 0x100 + 0x28, WORLD + 0x200);
                c.bus.write32(WORLD + 0x200, 1 + r.next_u32() % 9);
                c.g[A0][0] = sx(DATA + 0x6_0000);
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = rival as u64;
                c.g[A3][0] = sx(pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u32);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0015_5bf0;
            },
        );
    }

    use crate::difftest::check_lohi;

    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;

    /// Fixture for `BEAggressionInt_DecayAll`: 1..3 players, profile and
    /// character lookups return small indices (see `oc430_result`), every
    /// aggression cell (state in -1..4, points 0..40) random, the HUD call
    /// on or off (the real HUD function returns early on the fixture world).
    pub(crate) fn oc430_aggression_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(0x0053_05e0 + 0x30, 1 + seed % 3);
        c.bus.write8(0x0053_05b0 + 0x49, (seed % 4 == 0) as u8);
        for p in 0..3u32 {
            for ca in 0..10u32 {
                for j in 0..10u32 {
                    let cell = 0x004a_7869 + p * 0x9b50 + ca * 0xf88 + 3 * j;
                    c.bus
                        .write8(cell, ((r.next_u32() % 6) as u8).wrapping_sub(1));
                    c.bus.write8(cell + 1, r.next_u32() as u8);
                    c.bus.write8(cell + 2, (r.next_u32() % 41) as u8);
                }
            }
        }
        let world = DATA + 0x5000;
        c.bus.write32(c.g[GP][0] as u32 - 0x848, world);
        c.bus.write32(world + 0x84, DATA + 0x5100);
        c.bus.write32(DATA + 0x5100 + 0x28, DATA + 0x5200);
        c.bus.write32(DATA + 0x5200, 5);
    }

    /// `BEAggressionInt_DecayAll`: every state/level combination, with and
    /// without the HUD call (LO/HI of the index multiplies too).
    #[test]
    fn be_aggression_int_decay_all_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s01::f_00155e58__generated,
            be_aggression_int_decay_all,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_5e58, 20);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_5e58;
                oc430_aggression_fixture(c, &mut r, seed);
            },
        );
    }

    use crate::difftest::fakes::vtable;

    #[test]
    fn be_aggression_int_load_state_matches() {
        check(
            ee::s01::f_001561b0__generated,
            be_aggression_int_load_state,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x61b0, 0);
                let obj = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                c.bus.write32(obj, vt);
                c.g[A1][0] = obj as u64;
                fill(c, &mut r, DATA + 0x3000, 0x100);
            },
        );
    }

    #[test]
    fn be_library_switch_phase_matches() {
        check(
            ee::s01::f_001562b8__generated,
            be_library_switch_phase,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x62b8, 0);
                let vt = vtable(c, &mut r, &[0x18, 0x1c, 0x20, 0x24]);
                for k in 0..4u32 {
                    let obj = DATA + 0x2000 + 0x40 * k;
                    c.bus.write32(obj + 4, vt);
                    c.bus.write32(0x0053_08a8 + 4 * k, obj);
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), seed % 4);
                c.g[A0][0] = DATA as u64 + 0x3000 + (seed % 4) as u64 * 4;
                c.bus.write32(DATA + 0x3000 + (seed % 4) * 4, seed % 4);
            },
        );
    }

    /// `BEAggressionInt_Commit` (0x0015_5aa0) against the generated body.
    #[test]
    fn be_aggression_int_commit_00155aa0_matches() {
        check(
            ee::s01::f_00155aa0__generated,
            be_aggression_int_commit_00155aa0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155aa0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEAggressionInt_Reset` (0x0015_5aa8) against the generated body.
    #[test]
    fn be_aggression_int_reset_00155aa8_matches() {
        check(
            ee::s01::f_00155aa8__generated,
            be_aggression_int_reset_00155aa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155aa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
