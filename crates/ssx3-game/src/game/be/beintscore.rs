//! `src/be/beintscore.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::be::beintmission::callee::BE_GET_RIDER_CHARACTER;
use crate::game::be::beintmission::callee::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_49a8, be_score_int_ctor, "BEScoreInt_ctor"),
    (0x0015_49e8, be_score_int_v02, "BEScoreInt_v02"),
    (
        0x0015_4a58,
        be_score_int_update_best_stats,
        "BEScoreInt_UpdateBestStats",
    ),
    (
        0x0015_50e8,
        be_score_int_get_high_score_for,
        "BEScoreInt_GetHighScoreFor",
    ),
    (
        0x0015_5130,
        be_score_int_get_high_score,
        "BEScoreInt_GetHighScore",
    ),
    (
        0x0015_5190,
        be_score_int_get_high_score_owner,
        "BEScoreInt_GetHighScoreOwner",
    ),
    (
        0x0015_5208,
        be_score_int_get_high_score_name,
        "BEScoreInt_GetHighScoreName",
    ),
    (
        0x0015_5270,
        be_score_int_get_player_result,
        "BEScoreInt_GetPlayerResult",
    ),
    (
        0x0015_5288,
        be_score_int_get_best_records,
        "BEScoreInt_GetBestRecords",
    ),
    (
        0x0015_5328,
        be_score_int_place_to_medal,
        "BEScoreInt_PlaceToMedal",
    ),
    (
        0x0015_5390,
        bescore_int_update_record_level,
        "BEScoreInt_UpdateRecordLevel",
    ),
    (
        0x0015_57e0,
        be_score_int_can_submit_score,
        "BEScoreInt_CanSubmitScore",
    ),
    (
        0x0015_58f8,
        bescore_int_get_event_best_place,
        "BEScoreInt_GetEventBestPlace",
    ),
    (
        0x0015_4ab8,
        be_score_int_submit_race_result,
        "BEScoreInt_SubmitRaceResult",
    ),
    (
        0x0015_4ee8,
        be_score_int_submit_placement,
        "BEScoreInt_SubmitPlacement",
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
        0x0015_5420,
        be_score_int_update_records,
        "BEScoreInt_UpdateRecords",
    ),
    (
        0x0015_49e0,
        be_score_int_commit_001549e0,
        "BEScoreInt_Commit",
    ),
    (
        0x0015_5380,
        be_score_int_save_state_00155380,
        "BEScoreInt_SaveState",
    ),
    (
        0x0015_5388,
        be_score_int_load_state_00155388,
        "BEScoreInt_LoadState",
    ),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstringctor::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstringctor::callee::BE_GET_RIDER_PROFILE;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
}

// ---------------------------------------------------------------------------
// Front-end scores.

/// `BEScoreResult *BEScoreInt_GetPlayerResult(BEScoreInt *self, int player, ...)`
/// (0x155270): `self + player*0xb0 + 0x20`.
pub fn be_score_int_get_player_result(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0xb0;
    let slot = abi::mult(c, abi::arg(c, 1), 0xb0);
    c.g[V0][0] = sx(slot.wrapping_add(0x20));
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BEScoreRecord *BEScoreInt_GetBestRecords(BEScoreInt *self, int rider, int character)`
/// (0x155288): `rider*0x9b50 + character*0xf88 + 0x4a6ca8 + 0xde4`.
pub fn be_score_int_get_best_records(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[V0][0] = sx(rider_off);
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[A2][0] = sx(char_off);
    c.g[V1][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xde4));
    abi::ret(c);
}

/// `int BEScoreInt_PlaceToMedal(int place)` (0x155328): medal grade for a finish
/// place — 1st -> 2, 2nd -> 3, 3rd -> 1, anything else -1.
pub fn be_score_int_place_to_medal(c: &mut Cpu) {
    c.cycles += 3;
    let place = abi::arg64(c, 1);
    if place == 1 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    if (place as i64) >= 2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 2;
        if place == 2 {
            c.cycles += 2;
            c.g[V0][0] = 3;
            abi::ret(c);
            return;
        }
        c.g[V0][0] = u64::MAX;
        c.cycles += 2;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // place < 2 and not 1: either 0, or a negative sign-extended value.
    c.cycles += 2;
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    if place == 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Score singleton.

/// `BEScoreInt_ctor(self)` (0x1549a8): set the name at +0xc and reset.
pub fn be_score_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5358));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(Addr(c.g[S0][0] as u32 + 0xc).0, c.g[V0][0]);
        c.pc = 0x0015_49e8;
        BE_SCORE_INT_V02.call(c, 0x0015_49c8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BEScoreInt_v02(self)` (0x1549e8): zero 0x160 bytes from +0x20, set
/// +0x10/+0x14 to 0 and +0x18/+0x1c to -1, then set two stride-0xb0 records
/// to -1.
pub fn be_score_int_v02(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(Addr(c.g[S0][0] as u32 + 0x10).0, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw(Addr(c.g[S0][0] as u32 + 0x14).0, 0);
        c.g[A2][0] = 0x160;
        c.sw(Addr(c.g[S0][0] as u32 + 0x1c).0, c.g[V0][0]);
        c.g[RA][0] = 0x0015_4a1c;
        c.sw(Addr(c.g[S0][0] as u32 + 0x18).0, c.g[V0][0]);
        call_ptr(c, 0x003e_6448, 0x0015_4a1c)?;
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[V0][0] = 1;
        loop {
            c.cycles += 7;
            c.sw(Addr(c.g[S0][0] as u32).0, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb0));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_4a28);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

pub(crate) const BE_SCORE_INT_V02: Callee = Callee {
    addr: 0x0015_49e8,
    f: ssx3_ee::s01::f_001549e8,
};

/// `BEScoreInt_UpdateBestStats(self, rider)` (0x154a58): ask the rider's
/// profile whether stats should update, then `BEScoreInt_UpdateRecords`.
pub fn be_score_int_update_best_stats(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_4a74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = Addr(0x0053_05f9).lb(c);
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
            let update = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if update {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = u64::MAX;
                c.g[A3][0] = u64::MAX;
                call_ptr(c, 0x0015_5420, 0x0015_4aa4)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEScoreInt_GetHighScoreFor(score, int i, int a, int b)` (0x1550e8):
/// entry `i` (0x14 bytes) of the high-score table for `(a, b)`, or 0.
pub fn be_score_int_get_high_score_for(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE.call(c, 0x0015_5104)?;
        let table = c.g[V0][0];
        c.g[V1][0] = 0x14;
        if table != 0 {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V0][0] = c.lw(r32(c, V1));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEScoreInt_GetHighScore(int index)` (0x155130): the high-score entry
/// at `index` for the current player, or 0 when no table exists.
pub fn be_score_int_get_high_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let index = c.g[A1][0];
        c.g[S0][0] = index;
        call_ptr(c, 0x0014_dc80, 0x0015_5144)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0015_5150)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = Addr(r32_2(c, V1)).off(0x5f0).lw(c);
        c.g[A1][0] = Addr(r32_2(c, V0)).off(0x5c12).lb(c);
        call_ptr(c, 0x0014_ab20, 0x0015_5164)?;
        let table = c.g[V0][0] != 0;
        c.g[V1][0] = 0x14;
        if table {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(r32_2(c, V1)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lb(c)
}

/// `char* BEScoreInt_GetHighScoreName(int idx)` (0x155208): resolve the BE
/// interface and the score table, then return entry `idx`'s name (20-byte
/// records, name at +8) or the default name at `gp-0x1eac`.
pub fn be_score_int_get_high_score_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let idx = c.g[A1][0];
        c.g[S0][0] = idx;
        call_ptr(c, 0x0014_dc80, 0x0015_521c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0015_5228)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = 0x0053_0000;
        c.g[A1][0] = lb(c, 0x0053_5c12);
        c.g[A0][0] = lw(c, 0x0053_05f0);
        call_ptr(c, 0x0014_ab20, 0x0015_523c)?;
        c.g[A1][0] = c.g[V0][0];
        let have = c.g[A1][0] != 0;
        c.g[V0][0] = 0x14;
        if have {
            c.cycles += 3;
            c.mult(0, V1, S0, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, gp(c, -0x1eac));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BEScoreInt_CanSubmitScore(BEScoreInt* self, int riders)` (0x1557e0):
/// no handicap is set, the BE interface is up, and fewer than two profiles
/// are in career mode.
pub fn be_score_int_can_submit_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        open(c, 0x10);
        c.g[A1][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
        let few = c.g[A1][0] == 0;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if !few {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0015_57f8);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_57f8)?;
            let no_handicap = c.g[V0][0] == 0;
            if !no_handicap {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                c.cycles += 3;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x10);
                return Ok(());
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.cycles += 3;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            close(c, 0x10);
            return Ok(());
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x0015_5810);
        call_ptr(c, 0x0014_dc80, 0x0015_5810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_581c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014_dd58, 0x0015_581c)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5bfc));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
        let career = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if !career {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_5838);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_5838)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x10);
        Ok(())
    })
}

/// `func_00155190` (0x155190): `BEScoreInt_GetHighScoreOwner`. Looks up the
/// high-score table for entry `a1` (`BEScoreInt_GetHighScoreTable(`D_005305f0`,
/// `D_00535c12`)`); when it exists returns `table[a1].+4 + 0x530990`, else the
/// current player name (`gp-0x1eac`). Signature: `char* owner(int)`, where
/// `a0` is a `this` the original never reads.
pub fn be_score_int_get_high_score_owner(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0015_51a4);
        c.g[S0][0] = c.g[A1][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_51a4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_51b0);
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0015_51b0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = Addr(low(c, V1).wrapping_add(0x5f0)).lw(c);
        c.g[RA][0] = sx(0x0015_51c4);
        c.g[A1][0] = Addr(low(c, V0).wrapping_add(0x5c12)).lb(c);
        callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE.call(c, 0x0015_51c4)?;
        let no_table = c.g[V0][0] == 0;
        c.g[A0][0] = 0x14;
        if no_table {
            c.cycles += 1;
            c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x1eac));
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x88;
            c.mult(0, V1, S0, A0);
            c.g[A0][0] = sx(low(c, V1).wrapping_add(low(c, V0)));
            c.g[V0][0] = lw_r(c, A0, 0x4);
            c.g[V1][0] = 0x53_0000;
            c.g[V1][0] = sx(low(c, V1).wrapping_add(0x990));
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, V1)));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE scores.

/// `void BEScoreInt_UpdateRecordLevel(BEScoreInt* self, int idx, int score)`
/// (0x155390): raise the stored record level for `idx` when `score` beats it.
pub fn bescore_int_update_record_level(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = 6;
    c.g[V0][0] = 0x0044_0000;
    c.mult(0, V1, A1, V1); // idx * 6
    c.g[T3][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xed0));
    c.g[T1][0] = c.g[A0][0];
    c.g[A0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    c.g[T0][0] = 0;
    c.g[T2][0] = Addr(p(r32(c, A0), 0xbb8)).lb(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[V0][0] = Addr(p(r32(c, V1), 4)).lh(c);
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let below = c.g[V0][0] != 0;
    c.g[A3][0] = 2;
    if !below {
        c.cycles += 2;
        c.g[T0][0] = 3;
    } else {
        c.cycles += 1;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        loop {
            c.cycles += 2;
            let negative = (c.g[A3][0] as i64) < 0;
            c.g[V0][0] = 6;
            if negative {
                break;
            }
            c.cycles += 9;
            c.g[V1][0] = sx(r32(c, A3) << 1);
            c.mult(0, V0, A1, V0); // idx * 6
            c.g[A0][0] = sx(r32(c, T3).wrapping_add(0xed0));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, V1)).lh(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[T0][0] = sx(r32(c, A3).wrapping_add(1));
                break;
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            if event_due(c) {
                deliver_event_here(c, 0x0015_53d4);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
    let raise = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    if raise {
        c.cycles += 1;
        Addr(p(r32(c, V0), 0xbb8)).sb(c, c.g[T0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `BEScoreInt_GetEventBestPlace` (0x1558f8): return the event record's best
// place for a rider, or -1 when the profile is unavailable or out of range.
pub fn bescore_int_get_event_best_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        let blocked = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if blocked {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::BE_GET_RIDER_PROFILE.call(c, 0x0015_5930)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
            let bad = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if bad {
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                c.cycles += 15;
                c.g[A0][0] = c.g[S1][0];
                callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_5948)?;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 2, 2, 3);
                c.mult(0, 4, 16, 4);
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                callee::BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_5974)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                }
            }
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEScoreInt_SubmitRaceResult` (0x00154ab8): Records a finished race for one human rider: copies the 0xa0-byte race stats into the score object, updates the best stats and the per-event high score table (insert sorted, shift lower entries down, bump the other human's rank), then the records and the character/event profile score. `(self, rider, stats, score)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn be_score_int_submit_race_result(c: &mut Cpu) {
    guest(c, |c| {
        'b_154eb8: {
            'b_154eb4: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S4][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S3][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S5][0] = c.g[A2][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0014a0e0, 0x00154afc)?; // BE_GetRiderProfile
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0xb0;
                if t {
                    break 'b_154eb4;
                } else {
                    'b_154bec: {
                        'b_154b9c: {
                            c.cycles += 7;
                            c.mult(0, 2, 18, 3);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.g[S5][0] | c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_154b9c;
                            } else {
                                c.cycles += 8;
                                c.g[A2][0] = sx((c.g[S2][0] as u32) << 2);
                                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xa0));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xcc));
                                c.g[S6][0] = 0x440000;
                                c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                                'l_154b48: loop {
                                    c.cycles += 19;
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(8, c.g[A0][0] as u32);
                                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(8, c.g[V1][0] as u32);
                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00154b48);
                                        }
                                        continue 'l_154b48;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0xb0;
                                        if t {
                                            break 'b_154bec;
                                        } else {
                                            break 'b_154b9c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[T0][0] = sx((c.g[S2][0] as u32) << 2);
                        c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T0][0]);
                        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xa0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xcc));
                        c.g[S6][0] = 0x440000;
                        c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                        'l_154bbc: loop {
                            c.cycles += 11;
                            c.g[A2][0] = c.ld(c.g[A0][0] as u32);
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00154bbc);
                                }
                                continue 'l_154bbc;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0xb0;
                                break 'b_154bec;
                            }
                        }
                    }
                    'b_154d14: {
                        'b_154ccc: {
                            c.cycles += 53;
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.mult(0, 6, 18, 2);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.ldr(7, c.g[A0][0] as u32);
                            c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.sdr(7, c.g[V1][0] as u32);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S5][0];
                            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                            call_ptr(c, 0x00154a58, 0x00154c24)?; // BEScoreInt_UpdateBestStats
                            call_ptr(c, 0x0014dc80, 0x00154c2c)?; // BELibrary_GetInstance
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dd58, 0x00154c38)?; // BELibrary_GetInterface
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0014a080, 0x00154c40)?; // BE_GetRiderCharacter
                            c.g[A2][0] = 0x530000;
                            c.g[S7][0] = c.g[V0][0];
                            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5b0));
                            c.g[A3][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = 0x64;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5bc8));
                            c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x26b0));
                            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                            c.mult(0, 4, 4, 3);
                            c.g[V1][0] = 0xf88;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                            c.g[A1][0] = 0x9b50;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.mult(0, 3, 23, 3);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.mult(0, 5, 18, 5);
                            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[T0][0] = 0x4a0000;
                            c.g[S6][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x6ca8));
                            c.sw(c.g[A2][0] as u32, 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw(c.g[A3][0] as u32, c.g[S0][0]);
                            c.g[V0][0] = c.g[S6][0] ^ 0x16;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x48));
                            let t = c.g[V1][0] == 0u64;
                            if c.g[V0][0] == 0 {
                                c.g[S6][0] = 0u64;
                            }
                            if t {
                                break 'b_154ccc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_154d14;
                                } else {
                                    break 'b_154ccc;
                                }
                            }
                        }
                        'b_154d00: {
                            'b_154ce8: {
                                c.cycles += 3;
                                c.g[T1][0] = 0x1;
                                let t = (c.g[S3][0] as i64) < 0;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                if t {
                                    break 'b_154ce8;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(19, 1);
                                    c.cvt_s_w(1, 1);
                                    let t = true;

                                    if t {
                                        break 'b_154d00;
                                    } else {
                                        break 'b_154ce8;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                            c.g[V0][0] = c.g[S3][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.mtc1(2, 1);
                            c.cvt_s_w(1, 1);
                            c.add_s(1, 1, 1);
                            break 'b_154d00;
                        }
                        c.cycles += 5;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6dec));
                        c.mul_s(0, 1, 0);
                        c.cvt_w_s(1, 0);
                        c.mfc1(19, 1);
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_154d14;
                    }
                    'b_154e48: {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x001557e0, 0x00154d1c)?; // BEScoreInt_CanSubmitScore
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                        if t {
                            break 'b_154e48;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x0014ab20, 0x00154d2c)?; // BEScoreInt_GetHighScoreTable
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_154e48;
                            } else {
                                c.cycles += 8;
                                c.g[S0][0] = 0u64;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                                c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[A1][0] = c.g[A2][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                c.g[S2][0] =
                                    sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                'l_154d58: loop {
                                    'b_154d70: {
                                        'b_154d6c: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                            if t {
                                                break 'b_154d6c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = (c.g[V0][0] < c.g[S3][0]) as u64;
                                                if t {
                                                    break 'b_154d70;
                                                } else {
                                                    break 'b_154d6c;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                                        break 'b_154d70;
                                    }
                                    'b_154e38: {
                                        c.cycles += 2;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            break 'b_154e38;
                                        } else {
                                            'b_154dc0: {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x4;
                                                if t {
                                                    break 'b_154dc0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                                    'l_154d88: loop {
                                                        c.cycles += 14;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.ldl(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_sub(0xd),
                                                        );
                                                        c.ldr(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x14),
                                                        );
                                                        c.ldl(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x5),
                                                        );
                                                        c.ldr(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_sub(0xc),
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x4));
                                                        c.sdl(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(6, c.g[V1][0] as u32);
                                                        c.sdl(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[T0][0],
                                                        );
                                                        c.g[V0][0] = ((c.g[S0][0] as i64)
                                                            < (c.g[A0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x14));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00154d88);
                                                            }
                                                            continue 'l_154d88;
                                                        } else {
                                                            break 'b_154dc0;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 19;
                                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S7][0]);
                                            c.sw(c.g[A1][0] as u32, c.g[S3][0]);
                                            call_ptr(c, 0x0014dc80, 0x00154dcc)?; // BELibrary_GetInstance
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x0014dd58, 0x00154dd8)?; // BELibrary_GetInterface
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            call_ptr(c, 0x00147170, 0x00154de4)?; // BENewPlayerInt_GetName
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x004166f4, 0x00154df0)?; // call_indirect
                                            c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            if t {
                                                break 'b_154e48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A1][0] = sx((c.g[FP][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = ((c.g[V1][0] as i64)
                                                    < (c.g[S0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_154e48;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] =
                                                        ((c.g[V0][0] as i64) < 5i64) as u64;
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                    if t {
                                                        break 'b_154e48;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A2][0] = 0xffffffffffffffff;
                                                        let t = true;
                                                        c.sw(c.g[A1][0] as u32, c.g[A2][0]);
                                                        if t {
                                                            break 'b_154e48;
                                                        } else {
                                                            break 'b_154e38;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00154d58);
                                        }
                                        continue 'l_154d58;
                                    } else {
                                        break 'b_154e48;
                                    }
                                }
                            }
                        }
                    }
                    'b_154e80: {
                        'b_154e6c: {
                            c.cycles += 3;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            let t = c.g[A3][0] == 0u64;
                            c.g[A0][0] = c.g[S4][0];
                            if t {
                                break 'b_154e6c;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A3][0] = c.g[S3][0];
                                call_ptr(c, 0x00155420, 0x00154e64)?; // BEScoreInt_UpdateRecords
                                let t = true;
                                c.g[T0][0] = 0x530000;
                                if t {
                                    break 'b_154e80;
                                } else {
                                    break 'b_154e6c;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x00155420, 0x00154e7c)?; // BEScoreInt_UpdateRecords
                        c.g[T0][0] = 0x530000;
                        break 'b_154e80;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5b0));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        break 'b_154eb8;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                        c.g[A3][0] = c.g[S3][0];
                        call_ptr(c, 0x00152460, 0x00154ea4)?; // BE_RecordCharProfileEventScore
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        break 'b_154eb4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_154eb8;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

/// `BEScoreInt_SubmitPlacement` (0x00154ee8): Records a race placement for one rider: stores the result in the score object, converts it to a medal, notifies the event-result hooks and, unless the race is a replay or a practice run, reports the (medal, best time/score) to the reward interface. `(self, rider, result, score)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn be_score_int_submit_placement(c: &mut Cpu) {
    guest(c, |c| {
        'b_1550bc: {
            'b_1550b8: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S6][0] = c.g[A3][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0014a0e0, 0x00154f2c)?; // BE_GetRiderProfile
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S5][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0xb0;
                if t {
                    break 'b_1550b8;
                } else {
                    'b_154fc4: {
                        c.cycles += 31;
                        c.g[V1][0] = 0x530000;
                        c.mult(0, 4, 21, 2);
                        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[S2][0]);
                        call_ptr(c, 0x0014dc80, 0x00154f54)?; // BELibrary_GetInstance
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00154f60)?; // BELibrary_GetInterface
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S7][0] = c.g[V0][0];
                        call_ptr(c, 0x0014a080, 0x00154f6c)?; // BE_GetRiderCharacter
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[A0][0] = 0x64;
                        c.g[FP][0] = c.g[V0][0];
                        c.g[A1][0] = 0x530000;
                        c.mult(0, 3, 3, 4);
                        c.g[V0][0] = 0x440000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S1][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x5c12));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S3][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[S3][0] ^ 0x16;
                        if c.g[V0][0] == 0 {
                            c.g[S3][0] = 0u64;
                        }
                        call_ptr(c, 0x00155328, 0x00154fa8)?; // BEScoreInt_PlaceToMedal
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                        let t = c.g[V1][0] == 0u64;
                        c.g[S0][0] = c.g[V0][0];
                        if t {
                            break 'b_154fc4;
                        } else {
                            c.cycles += 3;
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.g[V0][0] = 0x1;
                            if c.g[S2][0] == 0 {
                                c.g[S0][0] = c.g[V0][0];
                            }
                            break 'b_154fc4;
                        }
                    }
                    'b_154fe4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0xffffffffffffffff;
                        if t {
                            break 'b_154fe4;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = c.g[S2][0];
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x001e25d8, 0x00154fe0)?; // call_indirect
                            c.g[V0][0] = 0xffffffffffffffff;
                            break 'b_154fe4;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        break 'b_1550bc;
                    } else {
                        'b_155024: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_155024;
                            } else {
                                c.cycles += 5;
                                call_ptr(c, 0x001e1eb8, 0x00155000)?; // call_indirect
                                c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_155024;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_155024;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_155024;
                                        } else {
                                            c.cycles += 2;
                                            call_ptr(c, 0x001e1550, 0x00155024)?; // call_indirect
                                            break 'b_155024;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 11;
                        call_ptr(c, 0x0014dc80, 0x0015502c)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0xd;
                        call_ptr(c, 0x0014dd58, 0x00155038)?; // BELibrary_GetInterface
                        c.g[A0][0] = 0x530000;
                        c.g[S4][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b0));
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            break 'b_1550bc;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[S2][0] = 0x4;
                            let t = c.g[V0][0] == c.g[S2][0];
                            c.g[A0][0] = c.g[S7][0];
                            if t {
                                break 'b_1550b8;
                            } else {
                                'b_1550a0: {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x001456a0, 0x0015506c)?; // BENewRaceInt_GetEventRecordTarget
                                    c.g[A3][0] = c.g[V0][0];
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    let t = c.g[A3][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_1550a0;
                                    } else {
                                        'b_155098: {
                                            'b_155090: {
                                                c.cycles += 2;
                                                if c.g[S1][0] == 0u64 {
                                                    c.g[V0][0] = 0x3c;
                                                    break 'b_155090;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[S1][0] != c.g[S2][0] {
                                                        c.g[V0][0] = ((c.g[S6][0] as i64)
                                                            < (c.g[A3][0] as i64))
                                                            as u64;
                                                        break 'b_155098;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x3c;
                                                        break 'b_155090;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.mult(0, 2, 7, 2);
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                                            break 'b_155098;
                                        }
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0 {
                                            c.g[S0][0] = 0u64;
                                        }
                                        c.g[A0][0] = c.g[S4][0];
                                        break 'b_1550a0;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = c.g[FP][0];
                                c.g[A3][0] = c.g[S1][0];
                                c.g[T0][0] = c.g[S3][0];
                                c.g[T1][0] = c.g[S0][0];
                                call_ptr(c, 0x001591e8, 0x001550b8)?; // BERewardInt_OnEventResult
                                break 'b_1550b8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_1550bc;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
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
pub(crate) const CALLEE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
    addr: 0x0015_2398,
    f: ee::s01::f_00152398,
};

/// `BE_RecordCharProfileEventPlace` (0x152528).
pub(crate) const CALLEE_RECORD_CHAR_PROFILE_EVENT_PLACE: Callee = Callee {
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

/// `BEScoreInt_UpdateRecords` (0x00155420): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_score_int_update_records(c: &mut Cpu) {
    if c.pc != 0x00155420 {
        return be_score_int_update_records_resume(c);
    }
    guest(c, |c| {
        'b_1557b0: {
            'b_1557ac: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0x530000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[S5][0] = c.g[A3][0];
                if t {
                    break 'b_1557ac;
                } else {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00155474);
                    Callee {
                        addr: 0x0014a0e0,
                        f: ee::s01::BE_GetRiderProfile,
                    }
                    .call(c, 0x00155474)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[S3][0] < 0x3u64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        break 'b_1557b0;
                    } else {
                        'b_1554f8: {
                            'b_1554f0: {
                                c.cycles += 24;
                                c.g[RA][0] = sx(0x0015548c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x0014a080,
                                    f: ee::s01::BE_GetRiderCharacter,
                                }
                                .call(c, 0x0015548c)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[A3][0] = 0x64;
                                c.g[V1][0] = 0xf88;
                                c.g[A2][0] = 0x9b50;
                                c.mult(0, 5, 5, 7);
                                c.mult(0, 2, 2, 3);
                                c.g[A0][0] = 0x440000;
                                c.g[V1][0] = 0x4a0000;
                                c.mult(0, 6, 19, 6);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26b0));
                                c.g[S6][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                                c.g[FP][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                                c.g[S1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A3][0] = 0u64;
                                c.g[V0][0] = c.g[S0][0] ^ 0x16;
                                c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0xde4));
                                let t = c.g[S2][0] == 0u64;
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = 0u64;
                                }
                                if t {
                                    break 'b_1554f0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xc;
                                    let t = c.g[S6][0] != c.g[V0][0];
                                    c.g[S3][0] = c.g[A3][0];
                                    if t {
                                        break 'b_1554f8;
                                    } else {
                                        break 'b_1554f0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[A3][0] = 0x8;
                            c.g[S3][0] = c.g[A3][0];
                            break 'b_1554f8;
                        }
                        'b_155790: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
                            if t {
                                break 'b_155790;
                            } else {
                                c.cycles += 1;
                                'l_155508: loop {
                                    'b_155784: {
                                        'b_155780: {
                                            c.cycles += 3;
                                            c.g[V1][0] = (c.g[S3][0] < 0xau64) as u64;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A1][0] =
                                                sx((c.g[S7][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if t {
                                                break 'b_155780;
                                            } else {
                                                'b_155728: {
                                                    'b_1556f8: {
                                                        'b_1556b0: {
                                                            'b_155668: {
                                                                'b_155620: {
                                                                    'b_1555ec: {
                                                                        'b_1555b8: {
                                                                            'b_155584: {
                                                                                'b_155564: {
                                                                                    c.cycles += 7;
                                                                                    c.g[V0][0] =
                                                                                        0x460000;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[S3][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5790));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                    c.g[A0][0] = c
                                                                                        .lw(
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    let t = c.g[A0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    match t {
0x00155530 => {
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = c.lw(c.g[A1][0] as u32);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
break 'b_155784;
} else {
c.cycles += 8;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A2][0] = c.g[V1][0];
c.sw(c.g[A1][0] as u32, c.g[V1][0]);
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x0015555c);
c.g[A1][0] = 0x3;
Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x0015555c)?;
let t = true;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
if t {
break 'b_155784;
} else {
break 'b_155564;
}
}
}
0x00155564 => {
break 'b_155564;
}
0x00155584 => {
break 'b_155584;
}
0x001555b8 => {
break 'b_1555b8;
}
0x001555ec => {
break 'b_1555ec;
}
0x00155620 => {
break 'b_155620;
}
0x00155668 => {
break 'b_155668;
}
0x001556b0 => {
break 'b_1556b0;
}
0x001556f8 => {
break 'b_1556f8;
}
0x00155728 => {
break 'b_155728;
}
_ => { c.jump_out(t, 0x00155420, 0x001557dc); return Ok(()); }
}
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                                                c.g[V0][0] = c
                                                                                    .lw(c.g[A1][0]
                                                                                        as u32);
                                                                                c.g[V0][0] = ((c.g
                                                                                    [V1][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                if c.g[V0][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                                                    break 'b_155784;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.sw(
                                                                                        c.g[A1][0]
                                                                                            as u32,
                                                                                        c.g[V1][0],
                                                                                    );
                                                                                    let t = true;
                                                                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                                                                                    if t {
                                                                                        break 'b_155780;
                                                                                    } else {
                                                                                        break 'b_155584;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 5;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c,
                                                                                ));
                                                                            c.g[V0][0] = c
                                                                                .lw(c.g[A1][0]
                                                                                    as u32);
                                                                            c.g[V0][0] = ((c.g[V1]
                                                                                [0]
                                                                                as i64)
                                                                                < (c.g[V0][0]
                                                                                    as i64))
                                                                                as u64;
                                                                            if c.g[V0][0] != 0u64 {
                                                                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                                                break 'b_155784;
                                                                            } else {
                                                                                c.cycles += 8;
                                                                                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                                                                                c.g[A2][0] =
                                                                                    c.g[V1][0];
                                                                                c.sw(
                                                                                    c.g[A1][0]
                                                                                        as u32,
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x001555b0);
                                                                                c.g[A1][0] = 0x5;
                                                                                Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001555b0)?;
                                                                                let t = true;
                                                                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                                                if t {
                                                                                    break 'b_155784;
                                                                                } else {
                                                                                    break 'b_1555b8;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V1][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        c.g[V0][0] =
                                                                            c.lw(c.g[A1][0] as u32);
                                                                        c.g[V0][0] = ((c.g[V1][0]
                                                                            as i64)
                                                                            < (c.g[V0][0] as i64))
                                                                            as u64;
                                                                        if c.g[V0][0] != 0u64 {
                                                                            c.g[S3][0] =
                                                                                sx((c.g[S3][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            break 'b_155784;
                                                                        } else {
                                                                            c.cycles += 8;
                                                                            c.sw(
                                                                                (c.g[A1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                                c.g[S0][0],
                                                                            );
                                                                            c.g[A2][0] = c.g[V1][0];
                                                                            c.sw(
                                                                                c.g[A1][0] as u32,
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001555e4);
                                                                            c.g[A1][0] = 0x4;
                                                                            Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001555e4)?;
                                                                            let t = true;
                                                                            c.g[S3][0] =
                                                                                sx((c.g[S3][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_155784;
                                                                            } else {
                                                                                break 'b_1555ec;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x88));
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A1][0] as u32);
                                                                    c.g[V0][0] = ((c.g[V1][0]
                                                                        as i64)
                                                                        < (c.g[V0][0] as i64))
                                                                        as u64;
                                                                    if c.g[V0][0] != 0u64 {
                                                                        c.g[S3][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_155784;
                                                                    } else {
                                                                        c.cycles += 8;
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                            c.g[S0][0],
                                                                        );
                                                                        c.g[A2][0] = c.g[V1][0];
                                                                        c.sw(
                                                                            c.g[A1][0] as u32,
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        c.g[RA][0] = sx(0x00155618);
                                                                        c.g[A1][0] = 0x7;
                                                                        Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155618)?;
                                                                        let t = true;
                                                                        c.g[S3][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        if t {
                                                                            break 'b_155784;
                                                                        } else {
                                                                            break 'b_155620;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 10;
                                                                c.g[AT][0] = 0x3f000000;
                                                                c.mtc1(1, 1);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x5c),
                                                                );
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[A1][0] as u32);
                                                                c.add_s(0, 0, 1);
                                                                c.cvt_w_s(1, 0);
                                                                c.mfc1(3, 1);
                                                                c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                    < (c.g[V0][0] as i64))
                                                                    as u64;
                                                                if c.g[V0][0] != 0u64 {
                                                                    c.g[S3][0] = sx((c.g[S3][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    break 'b_155784;
                                                                } else {
                                                                    c.cycles += 8;
                                                                    c.sw(
                                                                        (c.g[A1][0] as u32)
                                                                            .wrapping_add(0x4),
                                                                        c.g[S0][0],
                                                                    );
                                                                    c.g[A2][0] = c.g[V1][0];
                                                                    c.sw(
                                                                        c.g[A1][0] as u32,
                                                                        c.g[V1][0],
                                                                    );
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[RA][0] = sx(0x00155660);
                                                                    c.g[A1][0] = 0x1;
                                                                    Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155660)?;
                                                                    let t = true;
                                                                    c.g[S3][0] = sx((c.g[S3][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    if t {
                                                                        break 'b_155784;
                                                                    } else {
                                                                        break 'b_155668;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 1);
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x58),
                                                            );
                                                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                            c.add_s(0, 0, 1);
                                                            c.cvt_w_s(1, 0);
                                                            c.mfc1(3, 1);
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.g[S3][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_155784;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.sw(
                                                                    (c.g[A1][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    c.g[S0][0],
                                                                );
                                                                c.g[A2][0] = c.g[V1][0];
                                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[RA][0] = sx(0x001556a8);
                                                                c.g[A1][0] = 0u64;
                                                                Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001556a8)?;
                                                                let t = true;
                                                                c.g[S3][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                if t {
                                                                    break 'b_155784;
                                                                } else {
                                                                    break 'b_1556b0;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 10;
                                                        c.g[AT][0] = 0x3f000000;
                                                        c.mtc1(1, 1);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S2][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                        c.add_s(0, 0, 1);
                                                        c.cvt_w_s(1, 0);
                                                        c.mfc1(3, 1);
                                                        c.g[V0][0] = ((c.g[V1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_155784;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.sw(
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[S0][0],
                                                            );
                                                            c.g[A2][0] = c.g[V1][0];
                                                            c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x001556f0);
                                                            c.g[A1][0] = 0x2;
                                                            Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x001556f0)?;
                                                            let t = true;
                                                            c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_155784;
                                                            } else {
                                                                break 'b_1556f8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    if (c.g[S5][0] as i64) <= 0 {
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_155784;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[S6][0] != 0u64 {
                                                            c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_155784;
                                                        } else {
                                                            'b_15571c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[A1][0] as u32);
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = ((c.g[V0][0] as i64)
                                                                    < (c.g[S5][0] as i64))
                                                                    as u64;
                                                                if t {
                                                                    break 'b_15571c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] != 0u64 {
                                                                        c.g[S3][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_155784;
                                                                    } else {
                                                                        break 'b_15571c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.sw(
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[S0][0],
                                                            );
                                                            let t = true;
                                                            c.sw(c.g[A1][0] as u32, c.g[S5][0]);
                                                            if t {
                                                                break 'b_155780;
                                                            } else {
                                                                break 'b_155728;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S4][0] as i64) <= 0;
                                                c.g[V0][0] =
                                                    sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                                                if t {
                                                    break 'b_155780;
                                                } else {
                                                    'b_15574c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                            break 'b_15574c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x6;
                                                            if c.g[FP][0] != c.g[V0][0] {
                                                                c.g[S3][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_155784;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[A1][0] as u32);
                                                                break 'b_15574c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = ((c.g[S4][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_155784;
                                                    } else {
                                                        'b_155770: {
                                                            c.cycles += 6;
                                                            c.g[V0][0] = 0x64;
                                                            c.sw(
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[S0][0],
                                                            );
                                                            c.div(0, 20, 2);
                                                            c.sw(c.g[A1][0] as u32, c.g[S4][0]);
                                                            if c.g[V0][0] == 0u64 {
                                                                break 'b_155770;
                                                            } else {
                                                                break 'b_155770;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.lo[0];
                                                        c.g[RA][0] = sx(0x00155780);
                                                        c.g[A1][0] = 0x6;
                                                        Callee { addr: 0x00155390, f: ee::s01::BEScoreInt_UpdateRecordLevel }.call(c, 0x00155780)?;
                                                        break 'b_155780;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'b_155784;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00155508);
                                        }
                                        continue 'l_155508;
                                    } else {
                                        break 'b_155790;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        if t {
                            break 'b_1557b0;
                        } else {
                            c.cycles += 5;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88));
                            c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x50));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S7][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                                break 'b_1557b0;
                            } else {
                                break 'b_1557ac;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_1557b0;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `be_score_int_update_records` (converted from the `entries` body): any entry
/// other than 0x00155420, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn be_score_int_update_records_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00155420 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V0][0] = 0x530000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S4][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = c.g[A3][0];
                    if t {
                        pc = 0x001557ac;
                        continue;
                    }
                    pc = 0x0015546c;
                    continue;
                }
                0x0015546c => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00155474);
                    Callee {
                        addr: 0x0014a0e0,
                        f: ee::s01::BE_GetRiderProfile,
                    }
                    .call(c, 0x00155474)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[S3][0] < 0x3u64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001557b0;
                        continue;
                    }
                    pc = 0x00155484;
                    continue;
                }
                0x00155484 => {
                    c.cycles += 24;
                    c.g[RA][0] = sx(0x0015548c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0014a080,
                        f: ee::s01::BE_GetRiderCharacter,
                    }
                    .call(c, 0x0015548c)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A3][0] = 0x64;
                    c.g[V1][0] = 0xf88;
                    c.g[A2][0] = 0x9b50;
                    c.mult(0, 5, 5, 7);
                    c.mult(0, 2, 2, 3);
                    c.g[A0][0] = 0x440000;
                    c.g[V1][0] = 0x4a0000;
                    c.mult(0, 6, 19, 6);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26b0));
                    c.g[S6][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                    c.g[FP][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] = c.g[S0][0] ^ 0x16;
                    c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0xde4));
                    let t = c.g[S2][0] == 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = 0u64;
                    }
                    if t {
                        pc = 0x001554f0;
                        continue;
                    }
                    pc = 0x001554e4;
                    continue;
                }
                0x001554e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xc;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[S3][0] = c.g[A3][0];
                    if t {
                        pc = 0x001554f8;
                        continue;
                    }
                    pc = 0x001554f0;
                    continue;
                }
                0x001554f0 => {
                    c.cycles += 2;
                    c.g[A3][0] = 0x8;
                    c.g[S3][0] = c.g[A3][0];
                    pc = 0x001554f8;
                    continue;
                }
                0x001554f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
                    if t {
                        pc = 0x00155790;
                        continue;
                    }
                    pc = 0x00155504;
                    continue;
                }
                0x00155504 => {
                    c.cycles += 1;
                    pc = 0x00155508;
                    continue;
                }
                0x00155508 => {
                    c.cycles += 3;
                    c.g[V1][0] = (c.g[S3][0] < 0xau64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x00155780;
                        continue;
                    }
                    pc = 0x00155514;
                    continue;
                }
                0x00155514 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5790));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00155530 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155544;
                    continue;
                }
                0x00155544 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0015555c);
                    c.g[A1][0] = 0x3;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x0015555c)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155564;
                    continue;
                }
                0x00155564 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155578;
                    continue;
                }
                0x00155578 => {
                    c.cycles += 3;
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    if t {
                        pc = 0x00155780;
                        continue;
                    }
                    pc = 0x00155584;
                    continue;
                }
                0x00155584 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155598;
                    continue;
                }
                0x00155598 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001555b0);
                    c.g[A1][0] = 0x5;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x001555b0)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001555b8;
                    continue;
                }
                0x001555b8 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001555cc;
                    continue;
                }
                0x001555cc => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001555e4);
                    c.g[A1][0] = 0x4;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x001555e4)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001555ec;
                    continue;
                }
                0x001555ec => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155600;
                    continue;
                }
                0x00155600 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00155618);
                    c.g[A1][0] = 0x7;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x00155618)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155620;
                    continue;
                }
                0x00155620 => {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(1, 0);
                    c.mfc1(3, 1);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155648;
                    continue;
                }
                0x00155648 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00155660);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x00155660)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155668;
                    continue;
                }
                0x00155668 => {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(1, 0);
                    c.mfc1(3, 1);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155690;
                    continue;
                }
                0x00155690 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001556a8);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x001556a8)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001556b0;
                    continue;
                }
                0x001556b0 => {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(1, 0);
                    c.mfc1(3, 1);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001556d8;
                    continue;
                }
                0x001556d8 => {
                    c.cycles += 8;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001556f0);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x001556f0)?;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x001556f8;
                    continue;
                }
                0x001556f8 => {
                    c.cycles += 2;

                    if (c.g[S5][0] as i64) <= 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155700;
                    continue;
                }
                0x00155700 => {
                    c.cycles += 2;

                    if c.g[S6][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155708;
                    continue;
                }
                0x00155708 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    if t {
                        pc = 0x0015571c;
                        continue;
                    }
                    pc = 0x00155714;
                    continue;
                }
                0x00155714 => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x0015571c;
                    continue;
                }
                0x0015571c => {
                    c.cycles += 3;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    let t = true;
                    c.sw(c.g[A1][0] as u32, c.g[S5][0]);
                    if t {
                        pc = 0x00155780;
                        continue;
                    }
                    pc = 0x00155728;
                    continue;
                }
                0x00155728 => {
                    c.cycles += 2;
                    let t = (c.g[S4][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x00155780;
                        continue;
                    }
                    pc = 0x00155730;
                    continue;
                }
                0x00155730 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        pc = 0x0015574c;
                        continue;
                    }
                    pc = 0x0015573c;
                    continue;
                }
                0x0015573c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;

                    if c.g[FP][0] != c.g[V0][0] {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155748;
                    continue;
                }
                0x00155748 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    pc = 0x0015574c;
                    continue;
                }
                0x0015574c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x00155784;
                        continue;
                    }
                    pc = 0x00155758;
                    continue;
                }
                0x00155758 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x64;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.div(0, 20, 2);
                    c.sw(c.g[A1][0] as u32, c.g[S4][0]);

                    if c.g[V0][0] == 0u64 {
                        pc = 0x00155770;
                        continue;
                    }
                    pc = 0x00155770;
                    continue;
                }
                0x00155770 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lo[0];
                    c.g[RA][0] = sx(0x00155780);
                    c.g[A1][0] = 0x6;
                    Callee {
                        addr: 0x00155390,
                        f: ee::s01::BEScoreInt_UpdateRecordLevel,
                    }
                    .call(c, 0x00155780)?;
                    pc = 0x00155780;
                    continue;
                }
                0x00155780 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x00155784;
                    continue;
                }
                0x00155784 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 3);
                    if t {
                        if back_edge_unwinds(c, 0x00155508) {
                            return Ok(());
                        }
                        pc = 0x00155508;
                        continue;
                    }
                    pc = 0x00155790;
                    continue;
                }
                0x00155790 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001557b0;
                        continue;
                    }
                    pc = 0x00155798;
                    continue;
                }
                0x00155798 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S7][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                        pc = 0x001557b0;
                        continue;
                    }
                    pc = 0x001557ac;
                    continue;
                }
                0x001557ac => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x001557b0;
                    continue;
                }
                0x001557b0 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00155420, 0x001557dc);
                    return Ok(());
                }
            }
        }
    });
}

/// `BEScoreInt_Commit` (0x0015_49e0): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_commit_001549e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEScoreInt_SaveState` (0x0015_5380): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_save_state_00155380(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEScoreInt_LoadState` (0x0015_5388): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_load_state_00155388(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beintnetwork::tests::alloc_setup;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_2;

    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    use ssx3_ee as ee;

    #[test]
    fn be_score_int_get_player_result_matches() {
        check_lohi(
            ssx3_ee::s01::f_00155270,
            be_score_int_get_player_result,
            200,
            &|c, seed| {
                let mut r = Rng(0xa000_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn be_score_int_get_best_records_matches() {
        check_lohi(
            ssx3_ee::s01::f_00155288,
            be_score_int_get_best_records,
            200,
            &|c, seed| {
                let mut r = Rng(0xb000_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((seed / 8) % 8) as u64;
            },
        );
    }

    #[test]
    fn be_score_int_place_to_medal_matches() {
        check(
            ssx3_ee::s01::f_00155328,
            be_score_int_place_to_medal,
            400,
            &|c, seed| {
                let mut r = Rng(0xc000_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = match seed % 5 {
                    0 => 1,
                    1 => 0,
                    2 => 2,
                    3 => 3 + (seed % 8) as u64,
                    _ => 0xffff_ffff_ffff_ff00u64 | (seed as u64 & 0xff),
                };
            },
        );
    }

    #[test]
    fn be_score_int_update_best_stats_match() {
        check(
            ssx3_ee::s01::f_00154a58,
            be_score_int_update_best_stats,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x4a58, 1);
                c.g[A0][0] = 0;
                c.g[A1][0] = DATA as u64;
                c.bus.write32(0x0053_5b0, 0);
                c.bus.write32(
                    0x0053_5b20 + 0xc,
                    if seed % 3 == 0 { 0 } else { 0xffff_ffff },
                );
                c.bus.write32(0x0053_5b20 + 0x10, (seed % 2) as u32);
                c.bus.write8(0x0053_05f9, (seed % 5 == 0) as u8);
            },
        );
    }

    #[test]
    fn be_score_int_match() {
        check(
            ssx3_ee::s01::f_001549a8,
            be_score_int_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 61, 6);
                alloc_setup(c, &mut r);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                for i in 0..0x80 {
                    c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_001549e8,
            be_score_int_v02,
            60,
            &|c, seed| {
                let mut r = Rng(0x49e8_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                for i in 0..0x80 {
                    c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_score_int_get_high_score_for_matches() {
        check(
            ee::s01::f_001550e8__generated,
            be_score_int_get_high_score_for,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_50e8, 0);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (r.next_u32() % 0x20) as u64;
                c.g[A2][0] = (seed % 0x1a) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn be_score_int_get_high_score_matches() {
        check(
            ssx3_ee::s01::f_00155130__generated,
            be_score_int_get_high_score,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x155130, 0);
                // A created BE library singleton.
                w(c, GAME_GP - 0x1f04, OBJ3);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, 0x0053_08a8, 0);
                w(c, 0x0053_05f0, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.bus.write8(0x0053_0000 + 0x5c12, (seed % 8) as u8);
                for i in 0..8u32 {
                    w(c, OBJ2 + 20 * i, seed + i);
                }
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // ------------------------------------------------------------- score names

    #[test]
    fn be_score_int_get_high_score_name_matches() {
        check(
            ssx3_ee::s01::f_00155208__generated,
            be_score_int_get_high_score_name,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x155208);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write8(0x0053_5c12, (seed % 3) as u8);
                c.bus.write32(0x0053_05f0, DATA + 0x600);
                c.bus.write32(0x004a_1244, DATA + 0x700);
            },
        );
    }

    /// The lazily built BE singleton (`gp-0x1f04`) pointed at a zeroed dummy,
    /// and the current interface index (`gp-0x1efc`) at zero.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }

    #[test]
    fn be_score_int_can_submit_score_matches() {
        check(
            ee::s01::f_001557e0__generated,
            be_score_int_can_submit_score,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_57e0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(0x0053_5bfc, (seed % 3) as u32);
                c.bus
                    .write32(0x0053_08b8, if seed % 3 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn be_score_int_get_high_score_owner_matches() {
        check(
            ee::s01::f_00155190__generated,
            be_score_int_get_high_score_owner,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x155190, 3);
                world(c);
                be_library_2(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                b(c, 0x0053_5c12, (seed % 6) as u8);
                if seed % 2 == 0 {
                    w_2(c, 0x0053_05f0, 0);
                } else {
                    let table = DATA + 0x600;
                    w_2(c, 0x0053_05f0, table);
                    let idx = (seed % 4) as u32;
                    w_2(c, table + idx * 0x14 + 4, seed % 7);
                }
            },
        );
    }

    #[test]
    fn bescore_int_update_record_level_matches() {
        check(
            ee::s01::f_00155390__generated,
            bescore_int_update_record_level,
            150,
            &|c, seed| {
                let _ = base(c, seed, 0x15_5390);
                globals(c);
                // Three short thresholds per record.
                for i in 0..3u32 {
                    c.bus.write16(0x0044_0ed0 + 6 * i, 100u16 + i as u16 * 100);
                    c.bus
                        .write16(0x0044_0ed0 + 6 * i + 2, 300u16 + i as u16 * 100);
                    c.bus
                        .write16(0x0044_0ed0 + 6 * i + 4, 600u16 + i as u16 * 100);
                }
                let idx = (seed % 3) as u32;
                c.bus.write8(DATA + idx + 0xbb8, (seed % 4) as u8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(idx);
                c.g[A2][0] = [0u64, 150, 350, 650, 80][seed as usize % 5];
            },
        );
    }

    #[test]
    fn bescore_int_get_event_best_place_matches() {
        check(
            ee::s01::f_001558f8__generated,
            bescore_int_get_event_best_place,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1558);
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed / 5 % 4) as u64;
                c.g[A3][0] = (seed / 20 % 4) as u64;
                c.bus.write8(0x0053_05f9, (seed % 3 == 0) as u8);
                fill(c, &mut r, 0x004a_6ca8, 0x1000);
            },
        );
    }

    pub(crate) use crate::game::fe::festateloadscreen::tests::oc444_run;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ2;

    /// A BE rider index (the profile tables map it to profile 0, 1 or none).
    pub(crate) fn oc444_rider(r: &mut Rng) -> u32 {
        r.next_u32() % 4
    }

    // ------------------------------------------------- BEScoreInt_Submit*.

    /// `BEScoreInt_SubmitRaceResult(self, rider, stats, score)`: copies the
    /// stats (aligned and unaligned source), updates the best-stats, the
    /// high-score table and the records for profile 0 and 1.
    #[test]
    fn be_score_int_submit_race_result_matches() {
        oc444_run(
            ee::s01::f_00154ab8__generated,
            be_score_int_submit_race_result,
            0x0015_4ab8,
            0x15_4ab8,
            120,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = oc444_rider(r) as u64;
                c.g[A2][0] = (OC444_OBJ2 + 4 * (seed % 2)) as u64;
                let scores = [0u32, 1, 59, 600, u32::MAX, r.next_u32() % 100_000];
                c.g[A3][0] = sx(scores[seed as usize % 6]);
            },
        );
    }

    /// `BEScoreInt_SubmitPlacement(self, rider, result, score)`.
    #[test]
    fn be_score_int_submit_placement_matches() {
        oc444_run(
            ee::s01::f_00154ee8__generated,
            be_score_int_submit_placement,
            0x0015_4ee8,
            0x15_4ee8,
            120,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = oc444_rider(r) as u64;
                let results = [0u32, 1, 2, 3, u32::MAX, 5];
                c.g[A2][0] = sx(results[seed as usize % 6]);
                c.g[A3][0] = [0u32, 100, 6000, 7200][r.next_u32() as usize % 4] as u64;
            },
        );
    }

    pub(crate) use crate::game::be::beintmission::tests::be_setup;

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
    fn be_score_int_update_records_matches() {
        check(
            ee::s01::f_00155420__generated,
            be_score_int_update_records,
            200,
            &|c, seed| {
                machine(c, seed, 0x5420, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00155420;
            },
        );
    }

    /// `BEScoreInt_Commit` (0x0015_49e0) against the generated body.
    #[test]
    fn be_score_int_commit_001549e0_matches() {
        check(
            ee::s01::f_001549e0__generated,
            be_score_int_commit_001549e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1549e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEScoreInt_SaveState` (0x0015_5380) against the generated body.
    #[test]
    fn be_score_int_save_state_00155380_matches() {
        check(
            ee::s01::f_00155380__generated,
            be_score_int_save_state_00155380,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155380, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEScoreInt_LoadState` (0x0015_5388) against the generated body.
    #[test]
    fn be_score_int_load_state_00155388_matches() {
        check(
            ee::s01::f_00155388__generated,
            be_score_int_load_state_00155388,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155388, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
