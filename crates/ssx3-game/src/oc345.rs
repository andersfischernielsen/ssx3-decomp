//! Staging module oc345 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc345`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::ai::computer::{
    hit_rider_index, HIT_ADD_AGGRESSION, HIT_AGGRESSION_INSTANCE, HIT_AUDIO_INSTANCE,
    HIT_PLAY_COLLISION_SOUND, HIT_SPEECH_COLLISION,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0010_c450,
        computer_actor_rider_get_aggression_mode,
        "ComputerActorRider_GetAggressionMode",
    ),
    (
        0x0010_c758,
        computer_actor_rider_init_difficulty,
        "ComputerActorRider_InitDifficulty",
    ),
    (
        0x0010_c9a8,
        computer_actor_rider_set_attribs_for_peak,
        "ComputerActorRider_SetAttribsForPeak",
    ),
    (
        0x0010_e2e8,
        rider_report_rider_hit1,
        "Rider_ReportRiderHit1",
    ),
    (
        0x0010_e3a8,
        rider_report_rider_hit2,
        "Rider_ReportRiderHit2",
    ),
    (
        0x0010_e468,
        rider_report_rider_hit3,
        "Rider_ReportRiderHit3",
    ),
    (0x0010_e770, rider_v13, "Rider_v13"),
    (0x0010_e7d0, rider_v14, "Rider_v14"),
    (
        0x0010_f1c0,
        rider_dispatch_event_table,
        "Rider_DispatchEventTable",
    ),
];

// ---------------------------------------------------------------------------
// Computer rider difficulty.

/// The aggression-mode jump table (`.rodata`, one entry per level 1..5).
pub(crate) const AGGRESSION_MODE_TABLE: u32 = 0x0045_6a70;

/// `short ComputerActorRider_GetAggressionMode(self, int level, int mode)`
/// (0x10c450): how aggressively a computer rider of difficulty `level`
/// (1..5) plays in race `mode`. Levels 1 and 2: 1 for modes 1 and 2, else 2;
/// level 3: 0 for mode 1, else 1; level 4: 1 for modes other than 1 and 2;
/// level 5 and out-of-range levels: 0.
pub fn computer_actor_rider_get_aggression_mode(c: &mut Cpu) {
    c.cycles += 4;
    let level_m1 = r32(c, A1).wrapping_sub(1);
    c.g[V1][0] = sx(level_m1);
    if level_m1 >= 5 {
        c.cycles += 3;
        c.g[V0][0] = 0;
        return ret(c);
    }
    // The jump through the table: v1 = &table[level - 1], a0 = the target.
    c.cycles += 6;
    let entry = (level_m1 << 2).wrapping_add(AGGRESSION_MODE_TABLE);
    c.g[V1][0] = sx(entry);
    c.g[A0][0] = Addr(entry).lw(c);
    let mode = c.g[A2][0];
    match level_m1 {
        0 | 1 => {
            c.cycles += 3;
            c.g[A0][0] = 1;
            c.g[V1][0] = mode ^ 2;
            if mode == 1 {
                c.cycles += 2;
                c.g[V0][0] = 1;
            } else {
                // Level 1 takes the branch (1 cycle), level 2 falls through
                // to a `b` (2 cycles).
                c.cycles += if level_m1 == 0 { 3 } else { 4 };
                c.g[V0][0] = if mode == 2 { 1 } else { 2 };
            }
        }
        2 => {
            c.cycles += 5;
            c.g[V1][0] = 1;
            c.g[V0][0] = (mode != 1) as u64;
        }
        3 => {
            c.cycles += 3;
            c.g[A0][0] = 1;
            c.g[V0][0] = 0;
            if mode != 1 {
                c.cycles += 4;
                c.g[V1][0] = mode ^ 2;
                c.g[V0][0] = (mode != 2) as u64;
            } else {
                c.cycles += 2;
            }
        }
        _ => {
            c.cycles += 3;
            c.g[V1][0] = 1;
            c.g[V0][0] = 0;
        }
    }
    ret(c)
}

/// `cBE_getBE` / `BELibrary_GetInstance`.
pub(crate) const INIT_DIFF_BE_GET_BE: u32 = 0x0014_dc80;
/// `cBE_getInterface` / `BELibrary_GetInterface(be, index)`.
pub(crate) const INIT_DIFF_BE_GET_INTERFACE: u32 = 0x0014_dd58;
/// `BENewPlayerInt_GetDifficulty(iface, isRace)`.
pub(crate) const INIT_DIFF_GET_DIFFICULTY: u32 = 0x0014_7cb8;
/// `BENewRaceInt_GetTrack(iface)`.
pub(crate) const INIT_DIFF_GET_TRACK: u32 = 0x0014_4be0;
/// The race settings block (`D_00535BC8`): +0x48 race kind, +0x4a override.
pub(crate) const INIT_DIFF_RACE_SETTINGS: u32 = 0x0053_5bc8;
/// The cheat flags word (`D_005308D0`); bit 2 maxes the computer riders.
pub(crate) const INIT_DIFF_CHEATS: u32 = 0x0053_08d0;

const INIT_DIFF_GET_AGGRESSION_MODE: Callee = Callee {
    addr: 0x0010_c450,
    f: ee::s00::ComputerActorRider_GetAggressionMode,
};
const INIT_DIFF_APPLY_DIFFICULTY: Callee = Callee {
    addr: 0x0010_c4f8,
    f: ee::s00::ComputerActorRider_ApplyDifficulty,
};

/// Computer rider fields: aggression mode (short), "max level in mode 3"
/// flag, CPU-controlled-race flag, skill A and B.
pub(crate) const CAR_AGGRESSION: u32 = 0xe00;
pub(crate) const CAR_TOP_LEVEL: u32 = 0xe04;
pub(crate) const CAR_RACE_FLAG: u32 = 0xe08;
pub(crate) const CAR_SKILL_A: u32 = 0xdf8;
pub(crate) const CAR_SKILL_B: u32 = 0xdfc;

/// `void ComputerActorRider_InitDifficulty(cComputer *self)` (0x10c758,
/// `cComputer_updateRiderDifficulty`): read the rider's difficulty level
/// (its controller's virtual at +0x38, -1 meaning level 1), set the
/// aggression mode, flags and individual difficulty, then apply the race's
/// overrides: the max-AI cheat (skills 100/1.0, tune -100/100/2) or a
/// course boost of skill B (+25% on track 4, +10% on tracks 2 and 3,
/// clamped to 1.0).
pub fn computer_actor_rider_init_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 1;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = 1;
        frame.save_ra(c, 0);
        // level = rider->controller.GetDifficultyLevel(); mode = world mode.
        let rider = Addr(r32(c, S1)).off(0x18).lw(c);
        c.g[A2][0] = Addr(r32(c, V0)).off(0xc0).lw(c);
        let ctl = (rider as u32).wrapping_add(0x6c0);
        let vt = Addr(ctl).lw(c);
        c.g[A1][0] = vt;
        c.g[S2][0] = Addr(r32(c, A2)).lw(c);
        c.g[V1][0] = Addr(vt as u32).off(0x38).lh(c);
        c.g[V0][0] = Addr(vt as u32).off(0x3c).lw(c);
        c.g[A0][0] = sx(ctl.wrapping_add(r32(c, V1)));
        let method = r32(c, V0);
        call_ptr(c, method, 0x0010_c7a4)?;
        let got = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] = (-1i64 < got as i64) as u64;
        c.g[A2][0] = c.g[S2][0];
        if c.g[V1][0] != 0 {
            c.g[S3][0] = got;
        }
        c.g[A1][0] = c.g[S3][0];
        INIT_DIFF_GET_AGGRESSION_MODE.call(c, 0x0010_c7c0)?;
        let this = Addr(r32(c, S1));
        let aggression = c.g[V0][0];
        this.off(CAR_AGGRESSION).sh(c, aggression);
        if c.g[S3][0] != c.g[S0][0] {
            c.cycles += 1;
            this.off(CAR_TOP_LEVEL).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if c.g[S2][0] != 3 {
                this.off(CAR_TOP_LEVEL).sw(c, 0);
            } else {
                c.cycles += 2;
                let level = c.g[S3][0];
                this.off(CAR_TOP_LEVEL).sw(c, level);
            }
        }

        // Race kinds 1, 5 and 6 set the race flag.
        c.cycles += 12;
        call_ptr(c, INIT_DIFF_BE_GET_BE, 0x0010_c7e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, INIT_DIFF_BE_GET_INTERFACE, 0x0010_c7f4)?;
        c.g[S0][0] = 0x53_0000;
        let kind = Addr(INIT_DIFF_RACE_SETTINGS).off(0x48).lb(c);
        c.g[V1][0] = kind;
        let flagged = if kind == 1 {
            true
        } else {
            c.cycles += 2;
            if kind == 6 {
                true
            } else {
                c.cycles += 2;
                kind == 5
            }
        };
        let this = Addr(r32(c, S1));
        if flagged {
            c.cycles += 2;
            c.g[V0][0] = 1;
            this.off(CAR_RACE_FLAG).sw(c, 1);
        } else {
            c.g[V0][0] = 5;
            this.off(CAR_RACE_FLAG).sw(c, 0);
        }

        // Individual difficulty from the player interface.
        c.cycles += 23;
        c.g[S0][0] = sx(INIT_DIFF_RACE_SETTINGS);
        call_ptr(c, INIT_DIFF_BE_GET_BE, 0x0010_c830)?;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, INIT_DIFF_BE_GET_INTERFACE, 0x0010_c83c)?;
        let kind = Addr(r32(c, S0)).off(0x48).lb(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = (kind < 1) as u64;
        call_ptr(c, INIT_DIFF_GET_DIFFICULTY, 0x0010_c84c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        INIT_DIFF_APPLY_DIFFICULTY.call(c, 0x0010_c85c)?;
        call_ptr(c, INIT_DIFF_BE_GET_BE, 0x0010_c864)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, INIT_DIFF_BE_GET_INTERFACE, 0x0010_c870)?;
        c.g[S2][0] = c.g[V0][0];
        // Aggression override: 4 -> 0, 5 -> 2.
        let over = Addr(r32(c, S0)).off(0x4a).lb(c);
        c.g[V1][0] = over;
        c.g[V0][0] = 5;
        let this = Addr(r32(c, S1));
        c.cycles += 2;
        if over == 4 {
            this.off(CAR_AGGRESSION).sh(c, 0);
            c.cycles += 1;
        } else if over == 5 {
            c.cycles += 3;
            c.g[V0][0] = 2;
            this.off(CAR_AGGRESSION).sh(c, 2);
        }
        c.g[V1][0] = 0x53_0000;

        c.cycles += 5;
        let cheats = Addr(INIT_DIFF_CHEATS).lw(c);
        c.g[V0][0] = ((cheats as u32) >> 2) as u64 & 1;
        c.g[A2][0] = 1;
        if c.g[V0][0] != 0 {
            // Max-AI cheat.
            c.cycles += 19;
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(1, 2);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = sx(r32(c, S3).wrapping_sub(1));
            let this = r32(c, S1);
            c.g[V1][0] = Addr(this).off(0x18).lw(c);
            c.g[A1][0] = 2;
            c.g[AT][0] = 0xffff_ffff_c2c8_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = (c.g[A0][0] < 2) as u64;
            c.swc1(1, this.wrapping_add(CAR_SKILL_B));
            c.swc1(2, this.wrapping_add(CAR_SKILL_A));
            Addr(this).off(CAR_TOP_LEVEL).sw(c, 1);
            c.swc1(0, r32(c, V1).wrapping_add(0xdc));
            c.g[V0][0] = Addr(this).off(0x18).lw(c);
            c.swc1(2, r32(c, V0).wrapping_add(0xe0));
            c.g[V1][0] = Addr(this).off(0x18).lw(c);
            Addr(r32(c, V1)).off(0xe4).sw(c, 2);
            c.cycles += 2;
            let mode = if c.g[A0][0] != 0 { 2 } else { 1 };
            Addr(this).off(CAR_AGGRESSION).sh(c, mode);
        } else {
            // Course boost of skill B.
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, INIT_DIFF_GET_TRACK, 0x0010_c918)?;
            c.g[V1][0] = 4;
            let scale = if c.g[V0][0] == 4 {
                c.cycles += 4;
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(1, 0);
                let this = r32(c, S1);
                c.lwc1(1, this.wrapping_add(CAR_SKILL_B));
                true
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, INIT_DIFF_GET_TRACK, 0x0010_c93c)?;
                c.g[V1][0] = 2;
                let tenth = if c.g[V0][0] == 2 {
                    let this = r32(c, S1);
                    c.lwc1(1, this.wrapping_add(CAR_SKILL_B));
                    true
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, INIT_DIFF_GET_TRACK, 0x0010_c950)?;
                    c.g[V1][0] = 3;
                    let this = r32(c, S1);
                    if c.g[V0][0] != 3 {
                        c.lwc1(0, this.wrapping_add(CAR_SKILL_B));
                        false
                    } else {
                        c.cycles += 1;
                        c.lwc1(1, this.wrapping_add(CAR_SKILL_B));
                        true
                    }
                };
                if tenth {
                    c.cycles += 1;
                    let lit = r32(c, GP).wrapping_sub(0x7d1c);
                    c.lwc1(0, lit);
                }
                tenth
            };
            let this = r32(c, S1);
            if scale {
                c.cycles += 4;
                c.mul_s(0, 1, 0);
                c.add_s(1, 1, 0);
                c.swc1(1, this.wrapping_add(CAR_SKILL_B));
                c.lwc1(0, this.wrapping_add(CAR_SKILL_B));
            }
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, this.wrapping_add(CAR_SKILL_B));
            }
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

/// `BENewRaceInt_GetTrackPeak(iface)`.
pub(crate) const PEAK_ATTRIBS_GET_TRACK_PEAK: u32 = 0x0014_4c98;
/// `BECharInt_SetAllAttribs(iface, rider, level)`.
pub(crate) const PEAK_ATTRIBS_SET_ALL: u32 = 0x0014_8aa8;

/// One call of the rider controller's virtual at +0x38 (the rider index)
/// for `ComputerActorRider_SetAttribsForPeak`: `a0` holds the rider.
fn peak_attribs_rider_index(c: &mut Cpu, ret_to: u32) -> crate::call::Flow {
    let ctl = r32(c, A0).wrapping_add(0x6c0);
    let vt = Addr(ctl).lw(c);
    c.g[V1][0] = vt;
    let delta = Addr(vt as u32).off(0x38).lh(c);
    c.g[A1][0] = delta;
    c.g[V0][0] = Addr(vt as u32).off(0x3c).lw(c);
    c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
    let method = r32(c, V0);
    call_ptr(c, method, ret_to)
}

/// `void ComputerActorRider_SetAttribsForPeak(cComputer *self)` (0x10c9a8):
/// set all of the rider's character attributes for the current track's
/// peak: level 7 on peak 2, 4 on peak 1, else 1.
pub fn computer_actor_rider_set_attribs_for_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, INIT_DIFF_BE_GET_BE, 0x0010_c9c4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, INIT_DIFF_BE_GET_INTERFACE, 0x0010_c9d0)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, INIT_DIFF_BE_GET_BE, 0x0010_c9d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, INIT_DIFF_BE_GET_INTERFACE, 0x0010_c9e4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, PEAK_ATTRIBS_GET_TRACK_PEAK, 0x0010_c9f0)?;
        c.g[V1][0] = 2;
        let (level, rets) = if c.g[V0][0] == 2 {
            c.cycles += 13;
            c.g[A0][0] = Addr(r32(c, S1)).off(0x18).lw(c);
            (7, (0x0010_ca18, 0x0010_ca28))
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, PEAK_ATTRIBS_GET_TRACK_PEAK, 0x0010_ca38)?;
            c.g[V1][0] = 1;
            c.g[A0][0] = Addr(r32(c, S1)).off(0x18).lw(c);
            if c.g[V0][0] == 1 {
                c.cycles += 12;
                (4, (0x0010_ca5c, 0x0010_ca6c))
            } else {
                c.cycles += 11;
                (1, (0x0010_ca8c, 0x0010_ca9c))
            }
        };
        peak_attribs_rider_index(c, rets.0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = level;
        call_ptr(c, PEAK_ATTRIBS_SET_ALL, rets.1)?;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider-vs-rider hits.

/// `SSXAudioSystem_PlayRiderBumpSound(audio, a, b)`.
pub(crate) const HIT_PLAY_BUMP_SOUND: u32 = 0x0029_8138;
/// `SSXAudioSystem_PlayHitBoostSound(audio, rider)`.
pub(crate) const HIT_PLAY_HIT_BOOST_SOUND: u32 = 0x0029_8d00;

/// The shape of `Rider_ReportRiderHit0..2` (entry `base`): rider `a` (a0)
/// hit rider `b` (a1): play `sound(audio, a, b)`, the collision speech
/// `(audio, b, a, speech_kind, speech_variant)`, and add aggression of
/// `event` from `b`'s index towards `a`'s.
/// `sound_args`: the registers the sound call's `a1`/`a2` come from (Hit1
/// passes `(a, b)`, Hit2 `(b, a)`).
fn report_rider_hit(
    c: &mut Cpu,
    base: u32,
    sound: u32,
    sound_args: (usize, usize),
    speech: (u64, u64),
    event: u64,
) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, HIT_AUDIO_INSTANCE, base + 0x20)?;
        c.g[A1][0] = c.g[sound_args.0][0];
        c.g[A2][0] = c.g[sound_args.1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, sound, base + 0x30)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, base + 0x38)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = speech.1;
        c.g[A3][0] = speech.0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_SPEECH_COLLISION, base + 0x50)?;
        call_ptr(c, HIT_AGGRESSION_INSTANCE, base + 0x58)?;
        c.g[S2][0] = c.g[V0][0];
        hit_rider_index(c, S0, base + 0x74)?;
        c.g[S0][0] = c.g[V0][0];
        hit_rider_index(c, S1, base + 0x90)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = event;
        call_ptr(c, HIT_ADD_AGGRESSION, base + 0xa4)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_ReportRiderHit1(Rider *a, Rider *b)` (0x10e2e8): rider `a`
/// hit rider `b`: collision sound, collision speech (kind 1, variant 1),
/// aggression event 1.
pub fn rider_report_rider_hit1(c: &mut Cpu) {
    report_rider_hit(
        c,
        0x0010_e2e8,
        HIT_PLAY_COLLISION_SOUND,
        (S1, S0),
        (1, 1),
        1,
    );
}

/// `void Rider_ReportRiderHit2(Rider *a, Rider *b)` (0x10e3a8): a bump:
/// bump sound, collision speech (kind 0, variant 0), aggression event 2.
pub fn rider_report_rider_hit2(c: &mut Cpu) {
    report_rider_hit(c, 0x0010_e3a8, HIT_PLAY_BUMP_SOUND, (S0, S1), (0, 0), 2);
}

const HIT_KNOCKDOWN_OPPONENT: Callee = Callee {
    addr: 0x0011_9400,
    f: ee::s00::RiderMetrix_OnKnockdownOpponent,
};
const HIT_ADD_BOOST: Callee = Callee {
    addr: 0x0010_e098,
    f: ee::s00::Rider_AddBoost,
};

/// `void Rider_ReportRiderHit3(Rider *a, Rider *b)` (0x10e468): rider `b`
/// knocked rider `a` down: bump sound, collision speech (kind 0, variant
/// 1), hit-boost sound for `b`, aggression event 3 from `b` towards `a`,
/// then `b`'s knockdown metric and the boost it earns (kind 2).
pub fn rider_report_rider_hit3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e48c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_PLAY_BUMP_SOUND, 0x0010_e49c)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e4a4)?;
        c.g[T0][0] = 1;
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_SPEECH_COLLISION, 0x0010_e4bc)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e4c4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_PLAY_HIT_BOOST_SOUND, 0x0010_e4d0)?;
        call_ptr(c, HIT_AGGRESSION_INSTANCE, 0x0010_e4d8)?;
        c.g[S3][0] = c.g[V0][0];
        // b's index (the controller pointer goes to a1, s0 keeps b).
        c.g[A1][0] = c.g[S0][0];
        hit_rider_index(c, A1, 0x0010_e4f4)?;
        c.g[S2][0] = c.g[V0][0];
        hit_rider_index(c, S1, 0x0010_e510)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 3;
        call_ptr(c, HIT_ADD_AGGRESSION, 0x0010_e524)?;
        c.g[A0][0] = Addr(r32(c, S0)).off(0x790).lw(c);
        HIT_KNOCKDOWN_OPPONENT.call(c, 0x0010_e52c)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 0);
        c.g[A1][0] = 2;
        HIT_ADD_BOOST.call(c, 0x0010_e53c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider sound-event virtuals.

/// `SSXAudioSystem_PlayRiderSfx(audio, kind, rider, volume)`.
pub(crate) const RIDER_SFX_PLAY: u32 = 0x0029_ced8;
/// `SSXAudioSystem_ArcadeSay(audio, rider, line)`.
pub(crate) const RIDER_SFX_ARCADE_SAY: u32 = 0x002a_3b18;

/// The shape of `Rider_v13`/`Rider_v14` (entry `base`): add `amount` (f12)
/// to the float at `self + field`, play rider sound `kind` at volume 1.0
/// and the arcade speech line `line`.
fn rider_sfx_event(c: &mut Cpu, base: u32, field: u32, kind: u64, line: u64) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let at = r32(c, S0).wrapping_add(field);
        c.lwc1(0, at);
        c.add_s(0, 0, 12);
        c.swc1(0, at);
        call_ptr(c, HIT_AUDIO_INSTANCE, base + 0x20)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = kind;
        call_ptr(c, RIDER_SFX_PLAY, base + 0x38)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, base + 0x40)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = line;
        call_ptr(c, RIDER_SFX_ARCADE_SAY, base + 0x50)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_v13(Rider *self, float amount)` (0x10e770): add `amount` to
/// the float at +0x2e8, play rider sound 0 and arcade line 1.
pub fn rider_v13(c: &mut Cpu) {
    rider_sfx_event(c, 0x0010_e770, 0x2e8, 0, 1);
}

/// `void Rider_v14(Rider *self, float amount)` (0x10e7d0): add `amount` to
/// the float at +0x2ec, play rider sound 1 and arcade line 2.
pub fn rider_v14(c: &mut Cpu) {
    rider_sfx_event(c, 0x0010_e7d0, 0x2ec, 1, 2);
}

// ---------------------------------------------------------------------------
// Rider event dispatch.

/// The event jump table (`.rodata`, kinds 0..7).
pub(crate) const RIDER_EVENT_TABLE: u32 = 0x0045_6af0;

const RIDER_EVENT_AWARD_BONUS_BOOST: Callee = Callee {
    addr: 0x0010_e8b8,
    f: ee::s00::Rider_AwardBonusBoost,
};
const RIDER_EVENT_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};

/// `void Rider_DispatchEventTable(Rider *self, int kind, float amount)`
/// (0x10f1c0): run a course event on the rider. Kinds 1..3 call the rider
/// controller's virtuals +0x68/+0x70/+0x78 (`Rider_v13..v15`), 4 adds
/// boost (kind 8), 5 resets the rider to the path (reason 4), 6 awards a
/// bonus boost of `(int)amount`; 0, 7 and out-of-range kinds do nothing.
pub fn rider_dispatch_event_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        let kind = c.g[A1][0];
        c.g[V0][0] = (kind < 8) as u64;
        frame.save_ra(c, 0);
        if kind < 8 {
            c.cycles += 7;
            let entry = ((kind as u32) << 2).wrapping_add(RIDER_EVENT_TABLE);
            c.g[V0][0] = sx(RIDER_EVENT_TABLE);
            c.g[V1][0] = sx(entry);
            c.g[A0][0] = Addr(entry).lw(c);
            let this = r32(c, A2);
            match kind {
                1..=3 => {
                    // The controller's virtual at 0x68 + 8 * (kind - 1).
                    c.cycles += if kind == 3 { 4 } else { 5 };
                    let slot = 0x68 + 8 * (kind as u32 - 1);
                    let ctl = this.wrapping_add(0x6c0);
                    let vt = Addr(ctl).lw(c);
                    c.g[V1][0] = vt;
                    c.g[A1][0] = sx(ctl);
                    let delta = Addr(vt as u32).off(slot).lh(c);
                    c.g[V0][0] = Addr(vt as u32).off(slot + 4).lw(c);
                    c.cycles += 4;
                    c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
                    let method = r32(c, V0);
                    call_ptr(c, method, 0x0010_f230)?;
                }
                4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[A1][0] = 8;
                    HIT_ADD_BOOST.call(c, 0x0010_f244)?;
                }
                6 => {
                    c.cycles += 6;
                    c.cvt_w_s(0, 12);
                    c.mfc1(A1, 0);
                    c.g[A0][0] = c.g[A2][0];
                    RIDER_EVENT_AWARD_BONUS_BOOST.call(c, 0x0010_f25c)?;
                }
                5 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 4;
                    RIDER_EVENT_CHECK_RESET_PATH.call(c, 0x0010_f274)?;
                    c.cycles += 1;
                }
                _ => {
                    c.cycles += 1;
                }
            }
        } else {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, machine, vtable, LOG_RESULTS};
    use crate::difftest::{check, elf_image, put_words, Rng, DATA, GAME_GP};
    use crate::game::ai::computer::HIT_INDEX_SLOT;
    use std::sync::atomic::AtomicUsize;

    /// `GetAggressionMode`'s jump table as the ELF holds it.
    const OC345_AGGRESSION_TABLE: [u32; 5] = [
        0x0010_c478,
        0x0010_c498,
        0x0010_c4ac,
        0x0010_c4c0,
        0x0010_c4dc,
    ];

    /// `Rider_DispatchEventTable`'s jump table as the ELF holds it.
    const OC345_EVENT_TABLE: [u32; 8] = [
        0x0010_f274,
        0x0010_f1f0,
        0x0010_f204,
        0x0010_f218,
        0x0010_f238,
        0x0010_f264,
        0x0010_f24c,
        0x0010_f274,
    ];

    /// Every level (in and out of 1..5) against modes 0..3.
    #[test]
    fn computer_actor_rider_get_aggression_mode_matches() {
        check(
            ee::s00::f_0010c450__generated,
            computer_actor_rider_get_aggression_mode,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c450, 0);
                c.pc = 0x0010_c450;
                put_words(c, AGGRESSION_MODE_TABLE, &OC345_AGGRESSION_TABLE);
                let levels: [u64; 9] = [0, 1, 2, 3, 4, 5, 6, u64::MAX, 0x1_0000_0002];
                c.g[A1][0] = levels[seed as usize % levels.len()];
                let modes: [u64; 6] = [0, 1, 2, 3, u64::MAX, 0x1_0000_0001];
                c.g[A2][0] = modes[(seed as usize / 9 + r.next_u32() as usize) % modes.len()];
            },
        );
    }

    /// The BE library singleton and its interface table: every
    /// `GetInterface` is a fake virtual call (slot 8).
    fn oc345_belibrary(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    /// A rider whose controller's virtual +0x38 is the real leaf
    /// `SSXAudioSystem_GetInstance` (0x28b180), so it returns the word at
    /// gp+0x410 (`value`, negative values too).
    fn oc345_rider_returning(c: &mut Cpu, self_at: u32, rider: u32, value: u32) {
        let vt = DATA + 0x5000;
        c.bus.write32(self_at + 0x18, rider);
        c.bus.write32(rider + 0x6c0, vt);
        c.bus.write16(vt + 0x38, 0);
        c.bus.write32(vt + 0x3c, 0x0028_b180);
        c.bus.write32(GAME_GP + 0x410, value);
    }

    /// InitDifficulty with the real callees (the ELF image supplies the
    /// jump tables and literals): levels -5..6, modes 0..3, race kinds,
    /// overrides, the max-AI cheat and tracks 2/3/4/other. The three
    /// `GetInterface` calls are fakes (some unwind).
    #[test]
    fn computer_actor_rider_init_difficulty_matches() {
        covered(
            ee::s00::f_0010c758__generated,
            computer_actor_rider_init_difficulty,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c758, 3);
                elf_image(c);
                oc345_belibrary(c, &mut r);
                let me = DATA + 0x2000;
                let rider = DATA + 0x4000;
                let levels = [-5i32, -1, 0, 1, 1, 2, 3, 4, 5, 6];
                let level = levels[r.next_u32() as usize % levels.len()];
                oc345_rider_returning(c, me, rider, level as u32);
                // The world's race mode.
                let world = DATA + 0x6000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0xc0, world + 0x100);
                c.bus.write32(world + 0x100, r.next_u32() % 4);
                // GetDifficulty: character 0's difficulties.
                c.bus
                    .write16(0x004a_6ca8 + 0x280, (r.next_u32() % 3) as u16);
                c.bus
                    .write16(0x004a_6ca8 + 0x284, (r.next_u32() % 3) as u16);
                let kinds = [0u8, 1, 2, 5, 6];
                c.bus.write8(
                    INIT_DIFF_RACE_SETTINGS + 0x48,
                    kinds[r.next_u32() as usize % kinds.len()],
                );
                let overs = [0u8, 4, 5];
                c.bus.write8(
                    INIT_DIFF_RACE_SETTINGS + 0x4a,
                    overs[r.next_u32() as usize % overs.len()],
                );
                c.bus.write32(
                    INIT_DIFF_CHEATS,
                    if seed % 4 == 0 {
                        0x4
                    } else {
                        r.next_u32() & !4
                    },
                );
                c.bus
                    .write32(0x0053_5c08, [0u32, 2, 3, 4][(seed as usize / 4) % 4]);
                c.bus.write32(me + CAR_SKILL_B, r.f32(150.0));
                c.g[A0][0] = me as u64;
            },
        );
    }

    /// SetAttribsForPeak on peaks 0..3 (track 0's peak word) with rider
    /// indices 0..7; the `GetInterface` calls are fakes (some unwind) and
    /// `SetAllAttribs` runs for real.
    #[test]
    fn computer_actor_rider_set_attribs_for_peak_matches() {
        covered(
            ee::s00::f_0010c9a8__generated,
            computer_actor_rider_set_attribs_for_peak,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c9a8, 2);
                oc345_belibrary(c, &mut r);
                let me = DATA + 0x2000;
                oc345_rider_returning(c, me, DATA + 0x4000, r.next_u32() % 8);
                c.bus.write32(0x0053_5c08, 0);
                c.bus.write32(0x0043_d950 + 0x54, seed % 4);
                c.g[A0][0] = me as u64;
            },
        );
    }

    /// The Rider_ReportRiderHit0 fixture (see its test): no sound views,
    /// speech volume 0, fake GetRiderIndex methods, real AddAggression on
    /// filled records. `first_index` is the fake-call number of the first
    /// GetRiderIndex.
    fn oc345_hit_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32, first_index: u32) {
        use crate::game::be::beintaggression::{AGGRESSION_MESSAGES_OFF, AGGRESSION_RECORDS};
        let mut r = machine(c, seed, salt, calls);
        let gp = GAME_GP;
        let world = DATA + 0x5_0000;
        let audio = DATA + 0x1_0000;
        let agg = DATA + 0x6_0000;
        let (a, b) = (DATA, DATA + 0x1000);
        c.bus.write32(gp + 0x410, audio);
        c.bus.write32(audio + 0x62fc, 0);
        c.bus.write32(gp - 0x848, world);
        c.bus.write32(world + 0x84, world + 0x100);
        c.bus.write32(world + 0x100 + 0x84, world + 0x300);
        c.bus.write32(world + 0x300 + 0x10, 0);
        c.bus.write32(world + 0x100 + 0x28, world + 0x200);
        c.bus.write32(world + 0x200, 1 + r.next_u32() % 9);
        c.bus.write32(gp - 0x1ea4, agg);
        for rider in [a, b] {
            let vt = vtable(c, &mut r, &[0x28, HIT_INDEX_SLOT]);
            c.bus.write32(rider + 0x6c0, vt);
            // No sfx table: the bump and hit-boost sounds stop early.
            c.bus.write32(rider + 0x874, 0);
        }
        let rider = 1 + r.next_u32() % 2;
        let rival = 1 + r.next_u32() % 3;
        c.bus.write32(LOG_RESULTS + 4 * first_index, rider);
        c.bus.write32(LOG_RESULTS + 4 * (first_index + 1), rival);
        let pick = |r: &mut Rng, v: &[i32]| v[(r.next_u32() as usize) % v.len()];
        for idx in 0..4u32 {
            c.bus.write32(0x0053_05b0 + 4 * idx, idx);
            let rec = 0x0053_5b20 + 0x1c * idx;
            let flag = pick(&mut r, &[-1, 0, 1]) as u32;
            c.bus.write32(rec + 0xc, flag);
            c.bus.write32(rec + 0x10, r.next_u32() % 4);
            c.bus.write8(rec + 0x11, (r.next_u32() % 5) as u8);
        }
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
        c.g[A0][0] = a as u64;
        c.g[A1][0] = b as u64;
    }

    /// Rider_ReportRiderHit1: as Hit0 (the collision sound's position
    /// method is fake call 0).
    #[test]
    fn rider_report_rider_hit1_matches() {
        covered(
            ee::s00::f_0010e2e8__generated,
            rider_report_rider_hit1,
            200,
            &|c, seed| oc345_hit_fixture(c, seed, 0x10_e2e8, 3, 1),
        );
    }

    /// Rider_ReportRiderHit2: the bump sound stops early (no sfx table), so
    /// the GetRiderIndex fakes are calls 0 and 1.
    #[test]
    fn rider_report_rider_hit2_matches() {
        covered(
            ee::s00::f_0010e3a8__generated,
            rider_report_rider_hit2,
            200,
            &|c, seed| oc345_hit_fixture(c, seed, 0x10_e3a8, 2, 0),
        );
    }

    /// Rider_ReportRiderHit3: as Hit2, plus the real knockdown metric (no
    /// HUD: +0x1b0 = 0) and AddBoost (boost state 3 returns at once).
    #[test]
    fn rider_report_rider_hit3_matches() {
        covered(
            ee::s00::f_0010e468__generated,
            rider_report_rider_hit3,
            200,
            &|c, seed| {
                oc345_hit_fixture(c, seed, 0x10_e468, 2, 0);
                let b = DATA + 0x1000;
                c.bus.write32(b + 0x790, DATA + 0x3000);
                c.bus.write32(b + 0x304, 3);
            },
        );
    }

    /// Rider_v13/v14 with the real sound callees on their short paths: the
    /// rider has no sfx table, arcade speech is off or the rider's +0x470
    /// is not negative.
    fn oc345_sfx_fixture(c: &mut Cpu, seed: u32, salt: u64, field: u32) {
        let mut r = machine(c, seed, salt, 0);
        let audio = DATA + 0x1_0000;
        let rider = DATA + 0x2000;
        c.bus.write32(GAME_GP + 0x410, audio);
        let speech_on = if seed % 2 == 0 { 0 } else { 0x3f80_0000 };
        c.bus.write32(audio + 0x62bc + 0xa0, speech_on);
        c.bus.write32(rider + 0x874, 0);
        c.bus.write32(rider + 0x470, r.f32(10.0) & 0x7fff_ffff);
        c.bus.write32(rider + field, r.edge_f32());
        c.f[12] = r.edge_f32();
        c.g[A0][0] = rider as u64;
    }

    #[test]
    fn rider_v13_matches() {
        check(ee::s00::f_0010e770__generated, rider_v13, 60, &|c, seed| {
            oc345_sfx_fixture(c, seed, 0x10_e770, 0x2e8)
        });
    }

    #[test]
    fn rider_v14_matches() {
        check(ee::s00::f_0010e7d0__generated, rider_v14, 60, &|c, seed| {
            oc345_sfx_fixture(c, seed, 0x10_e7d0, 0x2ec)
        });
    }

    /// Gate for the DispatchEventTable fakes (in each test machine's RAM).
    pub(crate) const OC345_CTRL: u32 = DATA + 0x0f_8000;
    pub(crate) const OC345_MAGIC: u32 = 0x4f43_3435;

    static OC345_PREV_AWARD: AtomicUsize = AtomicUsize::new(0);
    static OC345_PREV_RESET: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `Rider_AwardBonusBoost` (0x10e8b8).
    fn oc345_fake_award_bonus_boost(c: &mut Cpu) {
        if c.bus.read32(OC345_CTRL) != OC345_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC345_PREV_AWARD,
                ee::s00::f_0010e8b8__generated,
            );
        }
        fake_call(c, 0x0010_e8b8);
    }

    /// Gated fake for `Rider_CheckResetPath` (0x116120).
    fn oc345_fake_check_reset_path(c: &mut Cpu) {
        if c.bus.read32(OC345_CTRL) != OC345_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC345_PREV_RESET,
                ee::s00::f_00116120__generated,
            );
        }
        fake_call(c, 0x0011_6120);
    }

    /// Every event kind 0..9 (and a huge one): the controller virtuals are
    /// fakes, AwardBonusBoost and CheckResetPath gated fakes, AddBoost real
    /// (boost state 3 returns at once).
    #[test]
    fn rider_dispatch_event_table_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_0010E8B8),
            oc345_fake_award_bonus_boost,
            &OC345_PREV_AWARD,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00116120),
            oc345_fake_check_reset_path,
            &OC345_PREV_RESET,
        );
        covered(
            ee::s00::f_0010f1c0__generated,
            rider_dispatch_event_table,
            220,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_f1c0, 1);
                c.pc = 0x0010_f1c0;
                c.bus.write32(OC345_CTRL, OC345_MAGIC);
                put_words(c, RIDER_EVENT_TABLE, &OC345_EVENT_TABLE);
                let rider = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x68, 0x70, 0x78]);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(rider + 0x304, 3);
                let kinds: [u64; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, u64::MAX];
                c.g[A0][0] = rider as u64;
                c.g[A1][0] = kinds[seed as usize % kinds.len()];
                c.f[12] = r.f32(100.0);
            },
        );
    }
}
