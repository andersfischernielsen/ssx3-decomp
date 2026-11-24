//! Staging module oc459 (wave wave225): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc459`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::game::be::beintmission::callee::{BE_GET_RIDER_CHARACTER, BE_GET_RIDER_PROFILE};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_3b00,
        be_mission_int_set_collected,
        "BEMissionInt_SetCollected",
    ),
    (
        0x0015_3f68,
        be_mission_int_get_mission_value,
        "BEMissionInt_GetMissionValue",
    ),
    (
        0x0015_47d8,
        be_mission_int_empty_stub_47d8,
        "BEMissionInt_EmptyStub_47d8",
    ),
    (
        0x0015_47e0,
        be_mission_int_empty_stub_47e0,
        "BEMissionInt_EmptyStub_47e0",
    ),
    (
        0x0015_52b0,
        be_score_int_get_record_level,
        "BEScoreInt_GetRecordLevel",
    ),
    (
        0x0015_5848,
        be_score_int_get_event_podium_count,
        "BEScoreInt_GetEventPodiumCount",
    ),
    (
        0x0015_59a8,
        be_score_int_record_event_result,
        "BEScoreInt_RecordEventResult",
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
        0x0015_7350,
        be_reward_int_get_audio_system_field504,
        "BERewardInt_GetAudioSystemField504",
    ),
];

/// `void BEMissionInt_SetCollected(BEMissionInt *self, int rider, int bit,
/// int idx)` (0x153b00): set bit `bit` of the collected mask of mission `idx`
/// in `rider`'s profile/character record, then count the set bits of that
/// mission's 64-bit mask into the character's collectible total for the
/// mission (`cTotalCollected`, at mission slot `idx + track`).
pub fn be_mission_int_set_collected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_3b30)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_3b3c)?;
        c.g[T1][0] = 0x530000;
        c.g[V1][0] = 0x9b50;
        c.g[S4][0] = c.g[T1][0];
        c.g[T7][0] = c.g[V0][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b0));
        c.mult(0, V1, S2, V1);
        c.g[T0][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = 0xf88;
        c.mult(0, A0, T7, A0);
        c.g[T2][0] = 0xc;
        c.mult(0, T0, T0, T2);
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.mult(0, A1, S3, T2);
        c.g[T5][0] = c.g[V1][0];
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[S1][0] as i64)) as u64;
        c.g[A2][0] = 0x4a0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6cac));
        c.g[T4][0] = c.g[A1][0];
        if c.g[A3][0] != 0 {
            c.g[V0][0] = c.g[S1][0];
        }
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[T6][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S1][0] as u32) & 31));
        c.g[T9][0] = 0xffff_ffff_ffff_ffff;
        c.g[T8][0] = 0x1;
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        let word = c.g[A0][0];
        Addr(c.g[A1][0] as u32).sw(c, word);
        c.g[T3][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x40)).lw(c);
        c.mult(0, T0, V0, T2);
        loop {
            c.cycles += 18;
            c.g[V1][0] = ((c.g[T9][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A3][0];
            }
            c.g[A2][0] = sx((c.g[T8][0] as u32) << ((c.g[A3][0] as u32) & 31));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.g[A1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[A0][0] = ((c.g[A3][0] as i64) < 64i64) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] &= c.g[A2][0];
            let again = c.g[A0][0] != 0;
            if c.g[V1][0] != 0 {
                c.g[T3][0] = c.g[A1][0];
            }
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_3be0);
                }
                continue;
            }
            break;
        }
        c.cycles += 23;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0xc;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = 0xf88;
        c.mult(0, A1, T7, A1);
        c.g[A0][0] = 0x9b50;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.mult(0, A0, S2, A0);
        c.mult(0, V0, V0, A2);
        c.g[V1][0] = 0x4a0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore_ra(c, 0);
        let count = c.g[T3][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sb(c, count);
        frame.close(c);
        Ok(())
    })
}

/// `int BEMissionInt_GetMissionValue(BEMissionInt *self, int id)` (0x153f68):
/// the `f14` word of the mission record found for `id` (the mission table
/// at 0x43ee10, 0x24 bytes per entry, read through `FindMission`).
pub fn be_mission_int_get_mission_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        crate::game::be::beintmission::callee::FIND_MISSION.call(c, 0x0015_3f78)?;
        c.g[A0][0] = 0x24;
        c.g[V1][0] = 0x440000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        frame.close(c);
        Ok(())
    })
}

/// `void cBEMissionInterface_addStreamData(void*)` (0x1547d8): an empty stub.
pub fn be_mission_int_empty_stub_47d8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        ret(c);
        Ok(())
    })
}

/// `void cBEMissionInterface_remStreamData(void*)` (0x1547e0): an empty stub.
pub fn be_mission_int_empty_stub_47e0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        ret(c);
        Ok(())
    })
}

/// `int BEScoreInt_GetRecordLevel(void *self, int rider)` (0x1552b0): the
/// `level` byte (+0xbb8) of the rider's character record, indexed by the
/// rider's character (the record stride is 0xf88; 0x4a6ca8 is the table).
pub fn be_score_int_get_record_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_52c8)?;
        c.g[A0][0] = 0xf88;
        c.g[V1][0] = 0x4a0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xbb8)).lb(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BE_GetCharProfileEventRecord` (0x152398).
const CALLEE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
    addr: 0x0015_2398,
    f: ee::s01::f_00152398,
};

/// `BE_RecordCharProfileEventPlace` (0x152528).
const CALLEE_RECORD_CHAR_PROFILE_EVENT_PLACE: Callee = Callee {
    addr: 0x0015_2528,
    f: ee::s01::f_00152528,
};

/// `int BEScoreInt_GetEventPodiumCount(void *self, int rider, int event,
/// int place)` (0x155848): 0 while the rider's record flag (+0x5f9) is set
/// or the rider's profile index is not below 3; otherwise the record count
/// (`lhu` at +2) of the character's event record for `event`/`place`.
pub fn be_score_int_get_event_podium_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5f9)).lb(c);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        let flag_set = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        let mut lookup = false;
        if !flag_set {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            BE_GET_RIDER_PROFILE.call(c, 0x0015_5880)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[S0][0] < 0x3) as u64;
            if c.g[V0][0] != 0 {
                lookup = true;
            }
        }
        if flag_set {
            c.cycles += 1;
        }
        if lookup {
            c.cycles += 15;
            c.g[A0][0] = c.g[S1][0];
            BE_GET_RIDER_CHARACTER.call(c, 0x0015_5898)?;
            c.g[V1][0] = 0xf88;
            c.g[A0][0] = 0x9b50;
            c.mult(0, V0, V0, V1);
            c.mult(0, A0, S0, A0);
            c.g[V1][0] = 0x4a0000;
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[A2][0] = c.g[S3][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            CALLEE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_58c4)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2)).lhu(c);
            }
        } else {
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEScoreInt_RecordEventResult(void *self, int rider, int event,
/// int place, int extra)` (0x1559a8): record a placement for the rider's
/// character unless the record flag (+0x5f9) is set or the profile index is
/// not below 3. Mirrors `BEScoreInt_GetEventPodiumCount`'s lookup.
pub fn be_score_int_record_event_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x530000;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5f9)).lb(c);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S0, 0x50);
        let flag_set = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        let mut tail = true;
        if !flag_set {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            BE_GET_RIDER_PROFILE.call(c, 0x0015_59e8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[S0][0] < 0x3) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 14;
                c.g[A0][0] = c.g[S1][0];
                BE_GET_RIDER_CHARACTER.call(c, 0x0015_5a00)?;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, V0, V0, V1);
                c.mult(0, A0, S0, A0);
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                CALLEE_RECORD_CHAR_PROFILE_EVENT_PLACE.call(c, 0x0015_5a30)?;
            } else {
                tail = false;
            }
        }
        if tail {
            c.cycles += 1;
        }
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
                BE_GET_RIDER_PROFILE.call(c, 0x0015_6228)?;
                c.g[S2][0] = 0xbc1;
                c.mult(0, S4, V0, FP);
                c.g[S0][0] = 0x9;
                c.g[A0][0] = c.g[S1][0];
                'inner: loop {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = c.g[S1][0];
                    BE_GET_RIDER_CHARACTER.call(c, 0x0015_6240)?;
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

/// `void BERewardInt_LoadRewardsFile(void)` (0x156a10): load the rewards
/// file (`RewardsFile_Load(0x4c3e98, 1)`). Not installed: the real loader
/// runs away on test data, so there is no differential test.
#[allow(dead_code)]
pub fn be_reward_int_load_rewards_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4c0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        crate::game::be::beintmission::callee::REWARDS_FILE_LOAD.call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_GetAudioSystemField504(void)` (0x157350): the word at
/// +0x504 of the object the table function at 0x28b180 returns.
pub fn be_reward_int_get_audio_system_field504(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0028_b180, 0x0015_7360)?;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x504)).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};

    /// The BELibrary dummy at gp-0x1f04 and the mission table at 0x43ee10.
    fn be_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3100, 0x200);
        r
    }

    #[test]
    fn be_mission_int_set_collected_matches() {
        check(
            ee::s01::f_00153b00__generated,
            be_mission_int_set_collected,
            120,
            &|c, seed| {
                let mut r = be_setup(c, seed, 0x3b00);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() % 64) as u64;
                c.g[A3][0] = (seed % 16) as u64;
                fill(c, &mut r, 0x004a_6cac, 0x2000);
            },
        );
    }

    #[test]
    fn be_mission_int_get_mission_value_matches() {
        check(
            ee::s01::f_00153f68__generated,
            be_mission_int_get_mission_value,
            60,
            &|c, seed| {
                let mut r = be_setup(c, seed, 0x3f68);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 4 == 0 {
                    0x9999
                } else {
                    (0x1000 + seed % 88) as u64
                };
                fill(c, &mut r, 0x0043_ee10, 88 * 9);
                for k in 0..88u32 {
                    let e = 0x0043_ee10 + 0x24 * k;
                    c.bus.write32(e, 0x1000 + k);
                }
            },
        );
    }

    #[test]
    fn be_mission_int_empty_stubs_match() {
        check(
            ee::s01::f_001547d8__generated,
            be_mission_int_empty_stub_47d8,
            8,
            &|c, seed| {
                machine(c, seed, 0x47d8, 0);
            },
        );
        check(
            ee::s01::f_001547e0__generated,
            be_mission_int_empty_stub_47e0,
            8,
            &|c, seed| {
                machine(c, seed, 0x47e0, 0);
            },
        );
    }

    #[test]
    fn be_score_int_get_record_level_matches() {
        check(
            ee::s01::f_001552b0__generated,
            be_score_int_get_record_level,
            60,
            &|c, seed| {
                let mut r = be_setup(c, seed, 0x52b0);
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, 0x004a_6ca8, 0x1000);
            },
        );
    }

    #[test]
    fn be_score_int_get_event_podium_count_matches() {
        check(
            ee::s01::f_00155848__generated,
            be_score_int_get_event_podium_count,
            60,
            &|c, seed| {
                let mut r = be_setup(c, seed, 0x5848);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                if seed % 5 == 0 {
                    c.bus.write8(0x0053_05f9, 1);
                }
                fill(c, &mut r, 0x004a_6ca8, 0x800);
            },
        );
    }

    #[test]
    fn be_score_int_record_event_result_matches() {
        check(
            ee::s01::f_001559a8__generated,
            be_score_int_record_event_result,
            60,
            &|c, seed| {
                let mut r = be_setup(c, seed, 0x59a8);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                c.g[T0][0] = r.next_u32() as u64;
                if seed % 5 == 0 {
                    c.bus.write8(0x0053_05f9, 1);
                }
                fill(c, &mut r, 0x004a_6ca8, 0x800);
            },
        );
    }

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
    fn be_reward_int_get_audio_system_field504_matches() {
        check(
            ee::s01::f_00157350__generated,
            be_reward_int_get_audio_system_field504,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7350, 0);
                fill(c, &mut r, DATA + 0x3000, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
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
}
