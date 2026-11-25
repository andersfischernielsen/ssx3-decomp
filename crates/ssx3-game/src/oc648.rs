//! Staging module oc648 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc648`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_ac88, gm_handler8_reset_port, "GMHandler8_Reset"),
    (0x0023_b6c0, gm_handler5_reset_port, "GMHandler5_Reset"),
    (
        0x0023_b8c8,
        gm_handler5_on_rider_finished_port,
        "GMHandler5_OnRiderFinished",
    ),
    (
        0x0023_bdb8,
        gm_handler6_on_rider_finished_port,
        "GMHandler6_OnRiderFinished",
    ),
];

// Callees of the game-mode handlers (generated addresses).
/// `cBE_getBE()`.
const BE_GET_BE: u32 = 0x0014_dc80;
/// `cBE_getInterface(be, n)`: 0 race, 1 player, 8 score.
const BE_GET_INTERFACE: u32 = 0x0014_dd58;
/// `cBENewPlayerInterface_getHumanRider(player, n)`.
const BE_GET_HUMAN_RIDER: u32 = 0x0014_6e98;
/// `cBENewRaceInterface_getRival(race, n)`.
const BE_GET_RIVAL: u32 = 0x0014_5750;
/// Player character id (`GET_CHAR_ID`): `(player, alt)`.
const BE_GET_CHAR_ID: u32 = 0x0014_7488;
/// Player alternate character id (`GET_ALT_CHAR`): `(player, alt)`.
const BE_GET_ALT_CHAR: u32 = 0x0014_74c8;
/// `cBENewPlayerInterface_getRiderCheatCharID(player, n)`.
const BE_GET_RIDER_CHEAT_ID: u32 = 0x0014_7410;
/// The player's rider-slot call with `(player, n)` (generated `0x147398`).
const BE_PLAYER_SLOT: u32 = 0x0014_7398;
/// `cAI_isRaceFinished(ai)`.
const AI_IS_RACE_FINISHED: u32 = 0x0012_a250;
/// `cRider_estimateFinishTick(rider)`.
const RIDER_ESTIMATE_TICK: u32 = 0x0012_2d78;
/// `cRider_estimateFinishScore(rider)`.
const RIDER_ESTIMATE_SCORE: u32 = 0x0012_2e50;
/// `GameModeMan_SortByTime(handler, n)`.
const SORT_BY_TIME: u32 = 0x0023_8bf8;
/// `GameModeMan_SortByScore(handler, n)`.
const SORT_BY_SCORE: u32 = 0x0023_8b70;
/// `GMHandler8_PickOpponents(self)`.
const PICK_OPPONENTS: u32 = 0x0023_b0a8;
/// `memset(p, c, n)`.
const MEMSET: u32 = 0x0041_6210;

/// `gp-0x480`: the game-mode handler (the current GM object).
#[cfg(test)]
const GP_GM: u32 = 0x480;

/// `void GMHandler8_Reset(GMHandler8* self)` (0x23ac88): reset the GM
/// handler's screen fields, clear the 5*8-byte table at `self+4`, run its
/// vtable's slot 5, copy the player's character ids and alt-character ids
/// into the handler, and pick opponents when the mode word was 1.
pub fn gm_handler8_reset_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;

        // player = cBE_getInterface(cBE_getBE(), 1)
        call_ptr(c, BE_GET_BE, 0x0023_aca8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, BE_GET_INTERFACE, 0x0023_acb4)?;
        c.g[S0][0] = c.g[V0][0];

        c.g[V1][0] = 4;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = 3;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = 7;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A3][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        call_ptr(c, BE_GET_INTERFACE, 0x0023_ad0c)?;

        // A profile word in the BE library's static data selects the copy.
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let no_profile = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        if !no_profile {
            c.cycles += 10;
            call_ptr(c, BE_GET_BE, 0x0023_ad24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_INTERFACE, 0x0023_ad30)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c04));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        }

        // memset(self + 4, 0, 0x28), then zero the handler's timers.
        c.cycles += 36;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = 9;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
        call_ptr(c, MEMSET, 0x0023_ad68)?;

        // self's vtable slot 5: {int16 delta; fn}, called with self + delta.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let slot = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot, 0x0023_ad7c)?;

        // The character ids (+0x18, +0x1c) and alt ids (+0x40, +0x44).
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_GET_CHAR_ID, 0x0023_ad88)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, BE_GET_CHAR_ID, 0x0023_ad9c)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, BE_GET_ALT_CHAR, 0x0023_adb0)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        call_ptr(c, BE_GET_ALT_CHAR, 0x0023_adc4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let mode_not_one = c.g[A0][0] != c.g[S2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        if !mode_not_one {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: PICK_OPPONENTS,
                f: ee::s06::GMHandler8_PickOpponents,
            }
            .call(c, 0x0023_addc)?;
        }

        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GMHandler5_Reset(GMHandler5* self)` (0x23b6c0): the rival time
/// handler's reset. The byte at `0x535c11` picks the branch: when it is 2 the
/// player's per-rider tables are filled from the riders and the function
/// returns; otherwise the rival tables are cleared and filled from the race.
pub fn gm_handler5_reset_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_23b8a8: {
            'b_23b814: {
                c.cycles += 48;
                let frame = Frame::open(c, 0x60);
                frame.save(c, 16, 0x50);
                frame.save(c, 17, 0x40);
                frame.save(c, 20, 0x10);
                frame.save(c, 18, 0x30);
                frame.save(c, 19, 0x20);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                call_ptr(c, BE_GET_BE, 0x0023_b6e4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                call_ptr(c, BE_GET_INTERFACE, 0x0023_b6f0)?;
                c.g[S4][0] = c.g[V0][0];
                call_ptr(c, BE_GET_BE, 0x0023_b6f8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, BE_GET_INTERFACE, 0x0023_b704)?;
                c.g[S1][0] = c.g[V0][0];

                c.g[V1][0] = 1;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = u64::MAX;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x94] {
                    c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
                }

                // self's vtable slot 5 (delta + fn), called with self + delta.
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let slot = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, slot, 0x0023_b76c)?;

                c.g[V1][0] = 0x0053_0000;
                c.g[A0][0] = 2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                let flag_not_two = c.g[V0][0] != c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                if flag_not_two {
                    break 'b_23b814;
                }

                // Flag == 2: one table per rider from the player's riders.
                c.cycles += 37;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x0023_b794);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0);
                call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b794)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 1;
                c.g[S2][0] = sx((c.g[S0][0] as u32) << 2);
                call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b7a8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[S3][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, BE_PLAYER_SLOT, 0x0023_b7bc)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, BE_GET_RIDER_CHEAT_ID, 0x0023_b7d4)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                call_ptr(c, BE_PLAYER_SLOT, 0x0023_b7ec)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, BE_GET_RIDER_CHEAT_ID, 0x0023_b804)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                break 'b_23b8a8;
            }

            // Flag != 2: the rival tables.
            c.cycles += 37;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, MEMSET, 0x0023_b824)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b830)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b840)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, BE_PLAYER_SLOT, 0x0023_b84c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b868)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BE_GET_RIVAL, 0x0023_b874)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b894)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x40), 0);
        }

        c.cycles += 8;
        frame_epilogue_b6c0(c);
        Ok(())
    })
}

/// The epilogue of `GMHandler5_Reset` (0x23b6c0): reload s0.. and `ra`
/// from its frame, pop it and return.
fn frame_epilogue_b6c0(c: &mut Cpu) {
    let frame = Frame { size: 0x60 };
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void GMHandler5_OnRiderFinished(GMHandler5* self, int idx)` (0x23b8c8):
/// the rival time handler's rider-finish step. When the race is finished and
/// the rider `idx` has finished (its state at +0x6c0), it stores the rider's
/// finish tick in the rival table and sorts the times; the player's own
/// entry is recorded at the end.
pub fn gm_handler5_on_rider_finished_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        'b_23b9b0: {
            'b_23b934: {
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                frame.save(c, 16, 0x30);
                frame.save(c, 17, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, 18, 0x10);
                frame.save_ra(c, 0);
                frame.save_fpr(c, 20, 0x40);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_b8f4)?;
                let not_finished = c.g[V0][0] == 0;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if not_finished {
                    break 'b_23b934;
                }
                c.cycles += 14;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S2][0] = c.g[A0][0];
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                let vfn = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, vfn, 0x0023_b92c)?;
                let entry_done = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if entry_done {
                    break 'b_23b9b0;
                }
                break 'b_23b934;
            }

            'b_23b9ac: {
                'b_23b988: {
                    c.cycles += 10;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x480));
                    let no_rider_block = c.g[V1][0] == 0;
                    c.g[V1][0] = 0x0053_0000;
                    if no_rider_block {
                        break 'b_23b988;
                    }
                    // The rider has a block: store its finish flag in the
                    // rival table and the point table.
                    c.cycles += 11;
                    c.g[A0][0] = 0x0053_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = 0x0005_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66a8));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] |= 0x7e40;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = 1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    break 'b_23b9ac;
                }
                c.cycles += 9;
                c.g[V0][0] = 0x0053_0000;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x478));
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                c.sw(c.g[V1][0] as u32, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        }

        // 'b_23bb7c: the player's own time, once the race is finished.
        'b_23bb7c: {
            'b_23bb78: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_b9bc)?;
                let not_finished = c.g[V0][0] == 0;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if not_finished {
                    break 'b_23bb78;
                }
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let vfn = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, vfn, 0x0023_b9ec)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, 16, 0x30);
                    break 'b_23bb7c;
                }

                'b_23bb68: {
                    'b_23ba5c: {
                        c.cycles += 10;
                        call_ptr(c, BE_GET_BE, 0x0023_b9fc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_ba08)?;
                        c.g[V1][0] = 0x0053_0000;
                        c.g[A0][0] = 2;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let flag_not_two = c.g[V0][0] != c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        if flag_not_two {
                            break 'b_23ba5c;
                        }
                        c.cycles += 14;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A1][0] = 0xa;
                        call_ptr(c, SORT_BY_TIME, 0x0023_ba28)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A0][0] = 1;
                        c.g[A1][0] = 0x0053_0000;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A0][0]);
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6640));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = 0;
                        }
                        let is_player = c.g[S0][0] == c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if is_player {
                            break 'b_23bb78;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        break 'b_23bb68;
                    }

                    'b_23bad8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = 0;
                        if none {
                            break 'b_23bad8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.mtc1(0, 20);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                        'l_23ba78: loop {
                            'b_23baa8: {
                                c.cycles += 11;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                c.c_le_s(20, 0);
                                let finish_le = c.fcc();
                                c.g[A2][0] = 1;
                                if finish_le {
                                    break 'b_23baa8;
                                }
                                c.cycles += 1;
                                c.g[A2][0] = 0;
                            }
                            'b_23bac0: {
                                c.cycles += 2;
                                let not_done = c.g[A2][0] != 0;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                if not_done {
                                    break 'b_23bac0;
                                }
                                c.cycles += 4;
                                call_ptr(c, RIDER_ESTIMATE_TICK, 0x0023_bab8)?;
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            }
                            c.cycles += 6;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0023_ba78);
                                }
                                continue 'l_23ba78;
                            }
                            break 'b_23bad8;
                        }
                    }

                    'b_23bb64: {
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x0023_bae0);
                        c.g[S1][0] = 1;
                        call_ptr(c, BE_GET_BE, 0x0023_bae0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_baec)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_baf8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0xa;
                        c.g[A0][0] = c.g[V1][0];
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                        call_ptr(c, SORT_BY_TIME, 0x0023_bb10)?;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        let slot_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if slot_taken {
                            break 'b_23bb64;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                        let rider_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if rider_taken {
                            break 'b_23bb64;
                        }
                        c.cycles += 7;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[S1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0);
                        break 'b_23bb78;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
                break 'b_23bb78;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
            break 'b_23bb7c;
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void GMHandler6_OnRiderFinished(GMHandler6* self, int idx)` (0x23bdb8):
/// the point-score handler's rider-finish step. Copies a 0xa0-byte block of
/// the rider's data into the frame (`ldl`/`ldr` when the source is unaligned),
/// then the rider's point entry is recorded; when the race is finished the
/// scores are sorted and the player's own entry is recorded.
pub fn gm_handler6_on_rider_finished_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 16;
        let frame = Frame::open(c, 0x100);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, 16, 0xe0);
        frame.save(c, 17, 0xd0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0xc0);
        c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
        frame.save_ra(c, 0xb0);
        frame.save_fpr(c, 20, 0xf0);
        'b_23bf08: {
            'b_23be20: {
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                let has_block = c.g[V1][0] == 0;
                c.g[V1][0] = 0x0053_0000;
                if has_block {
                    break 'b_23be20;
                }
                c.cycles += 10;
                c.g[A1][0] = 0x0053_0000;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = 1;
                c.sw(c.g[V0][0] as u32, 0);
                c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                c.g[S1][0] = c.g[A1][0];
                break 'b_23bf08;
            }

            // The copy of the 32-byte result block into the stack frame.
            'b_23bed4: {
                'b_23be98: {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    if aligned {
                        break 'b_23be98;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                    c.g[S2][0] = c.g[A2][0];
                    c.g[S1][0] = 0x0053_0000;
                    c.g[V0][0] = 0x0053_0000;
                    'l_23be44: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[V1][0] as u32);
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let more = c.g[V1][0] != c.g[A0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0023_be44);
                            }
                            continue 'l_23be44;
                        }
                        c.cycles += 2;
                        break 'b_23bed4;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                c.g[S2][0] = c.g[A2][0];
                c.g[S1][0] = 0x0053_0000;
                c.g[V0][0] = 0x0053_0000;
                'l_23bea8: loop {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let more = c.g[V1][0] != c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_bea8);
                        }
                        continue 'l_23bea8;
                    }
                    break 'b_23bed4;
                }
            }

            c.cycles += 13;
            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(4, c.g[V1][0] as u32);
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[A1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66a8));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6640));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0);
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            break 'b_23bf08;
        }

        // 'b_23c0b4: the player's score once the race is finished.
        'b_23c0b4: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_bf18)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0xe0);
                break 'b_23c0b4;
            }

            'b_23c0b0: {
                'b_23c0a8: {
                    'b_23bf90: {
                        c.cycles += 10;
                        call_ptr(c, BE_GET_BE, 0x0023_bf28)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_bf34)?;
                        c.g[V1][0] = 0x0053_0000;
                        c.g[A0][0] = 2;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let flag_not_two = c.g[V0][0] != c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        if flag_not_two {
                            break 'b_23bf90;
                        }
                        c.cycles += 13;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A1][0] = 2;
                        call_ptr(c, SORT_BY_SCORE, 0x0023_bf54)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = 1;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6640));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6640));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = 0;
                        }
                        let is_player = c.g[S0][0] == c.g[V1][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if is_player {
                            break 'b_23c0b0;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        break 'b_23c0a8;
                    }

                    // The rider slot is free: the score table (+0x1..) walk.
                    'b_23c010: {
                        c.cycles += 5;
                        c.g[S2][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = 0;
                        if none {
                            break 'b_23c010;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.mtc1(0, 20);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                        'l_23bfb0: loop {
                            'b_23bfe0: {
                                c.cycles += 11;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                c.c_le_s(20, 0);
                                let finish_le = c.fcc();
                                c.g[A2][0] = 1;
                                if finish_le {
                                    break 'b_23bfe0;
                                }
                                c.cycles += 1;
                                c.g[A2][0] = 0;
                            }
                            'b_23bff8: {
                                c.cycles += 2;
                                let not_done = c.g[A2][0] != 0;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                if not_done {
                                    break 'b_23bff8;
                                }
                                c.cycles += 4;
                                call_ptr(c, RIDER_ESTIMATE_SCORE, 0x0023_bff0)?;
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            }
                            c.cycles += 6;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0023_bfb0);
                                }
                                continue 'l_23bfb0;
                            }
                            break 'b_23c010;
                        }
                    }

                    // 'b_23c09c: the race finished: sort the scores, record.
                    'b_23c09c: {
                        c.cycles += 21;
                        c.g[S1][0] = 1;
                        call_ptr(c, BE_GET_BE, 0x0023_c018)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_c024)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_c030)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 2;
                        c.g[A0][0] = c.g[V1][0];
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                        call_ptr(c, SORT_BY_SCORE, 0x0023_c048)?;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        let slot_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if slot_taken {
                            break 'b_23c09c;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                        let rider_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if rider_taken {
                            break 'b_23c09c;
                        }
                        c.cycles += 7;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[S1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0);
                        break 'b_23c0b0;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    break 'b_23c0a8;
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
            }
            c.cycles += 1;
            frame.restore(c, 16, 0xe0);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0xd0);
        frame.restore(c, 18, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA, GAME_GP};
    use crate::game::scripter::ssxscriptengine::tests::{base_14, check3, LEAF, WORLD_4};

    const SEEDS: u32 = 40;
    /// The game-mode handler object (`gp-0x480`).
    const GM: u32 = DATA + 0x2_2000;
    /// The `self` of the reset functions and its vtable (slot 0x28/0x2c).
    const SELF: u32 = DATA + 0x2_3000;
    const SELF_VT: u32 = DATA + 0x2_3800;
    /// Four riders, each 0x800 bytes apart; the state pointer at +0x6c0.
    const RIDERS: u32 = DATA + 0x2_8000;
    /// The copy source of a rider's result block (aligned or not, by seed).
    const SRC: u32 = DATA + 0x3_0000;
    /// A rider block that the point and time tables read.
    const BLOCK: u32 = DATA + 0x2_f000;
    /// The AI's rider table is at `ai + 0x28`.
    const AI: u32 = WORLD_4 + 0x2000;

    /// The BE library's interfaces: the real `cBE_getInterface` lands in a
    /// fake that returns `IFACE`, as in the gamemode tests.
    fn oc648_be(c: &mut Cpu) {
        const IFACE: u32 = DATA + 0x6_0000;
        const BE_STATE: u32 = DATA + 0x6_1000;
        const BE_VT: u32 = DATA + 0x6_1100;
        c.hooks.miss = fakes::fake_call;
        c.hooks.jump_out = fakes::fake_jump_out;
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        for n in 0..8u32 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, IFACE);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_STATE);
        c.bus.write32(BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        c.bus.write32(BE_VT + 0xc, fakes::FAKE);
    }

    /// Scrambled machine with the world chain (world -> game -> AI) and the
    /// game-mode handler at `gp-0x480`.
    fn oc648_world(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = base_14(c, seed, salt);
        oc648_be(c);
        let game = WORLD_4 + 0x1000;
        c.bus.write32(WORLD_4 + 0x84, game);
        c.bus.write32(game + 0xc, AI);
        c.bus.write32(AI + 0x8, r.next_u32() % 4000);
        c.bus.write32(GAME_GP - GP_GM, GM);
        for i in 0..0x40u32 {
            c.bus.write32(GM + 4 * i, 0);
        }
        c.bus.write32(GM + 0x84, (seed % 2) as u32);
        r
    }

    /// The four riders of the AI's table, with their state objects (a leaf
    /// method) and the copy sources and blocks the handlers read.
    fn oc648_riders(c: &mut Cpu, seed: u32) {
        for i in 0..4u32 {
            let rider = RIDERS + 0x800 * i;
            let state = rider + 0x400;
            c.bus.write32(AI + 0x28 + 4 * i, rider);
            c.bus.write32(rider + 0x6c0, state);
            c.bus.write32(state + 0x4c, LEAF);
            c.bus.write16(state + 0x48, 0);
            let aligned = (seed + i) % 2 == 0;
            c.bus.write32(rider + 0x790, if aligned { SRC } else { SRC + 4 });
            c.bus.write32(rider + 0x480, if (seed + i) % 3 == 0 { 0 } else { BLOCK });
        }
    }

    /// `self` for the reset functions: a vtable whose slot 5 is the leaf.
    fn oc648_self(c: &mut Cpu) {
        c.bus.write32(SELF, SELF_VT);
        c.bus.write16(SELF_VT + 0x28, 0);
        c.bus.write32(SELF_VT + 0x2c, LEAF);
    }

    #[test]
    fn gm_handler8_reset_port_matches() {
        check3(
            ee::s06::f_0023ac88__generated,
            gm_handler8_reset_port,
            SEEDS,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23ac88);
                oc648_self(c);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(SELF);
                c.pc = 0x0023_ac88;
            },
        );
    }

    #[test]
    fn gm_handler5_reset_port_matches() {
        check3(
            ee::s06::f_0023b6c0__generated,
            gm_handler5_reset_port,
            SEEDS,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23b6c0);
                oc648_self(c);
                oc648_riders(c, seed);
                // The flag byte at 0x535c11 picks the early-out (2) or the
                // rival tables.
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                c.g[A0][0] = sx(SELF);
                c.pc = 0x0023_b6c0;
            },
        );
    }

    #[test]
    fn gm_handler5_on_rider_finished_port_matches() {
        check3(
            ee::s06::f_0023b8c8__generated,
            gm_handler5_on_rider_finished_port,
            SEEDS,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23b8c8);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0023_b8c8;
            },
        );
    }

    #[test]
    fn gm_handler6_on_rider_finished_port_matches() {
        check3(
            ee::s06::f_0023bdb8__generated,
            gm_handler6_on_rider_finished_port,
            SEEDS,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23bdb8);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0023_bdb8;
            },
        );
    }
}
