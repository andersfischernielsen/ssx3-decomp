//! Staging module oc633 (wave g13.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc633`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_a3b8, be_reward_int_find_requirement_tier, "BERewardInt_FindRequirementTier"),
];

/// `gMedalsInfo[3][4][5]`: 2-byte `{event, location}` entries, the
/// location stride 2, the row stride 0xa and the table stride 0x28.
pub(crate) const BE_MEDALS_INFO: u32 = 0x0045_aad8;

/// `int cBERewardInterface_getTrackTrophyID(cBERewardInterface* self, int eventID, int locationID)`
/// (0x0015a3b8): the index `j` of the first medal entry whose event is
/// `eventID` and whose location is `locationID` (events 6..=11 match on the
/// event alone), searched over `i` (3), `j` (4) and `k` (5 entries, stopping
/// after 5 misses in a row of entries). `-1` when nothing matches.
///
/// Arguments: `a1` = eventID, `a2` = locationID. Scratch: `a0` (row
/// pointer), `a3` (misses in the current row), `t0` = j, `t1` = i, `v1`
/// (entry pointer). `t2..t8` hold the special event ids and the table base.
pub fn be_reward_int_find_requirement_tier(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x0046_0000;
    c.g[T1][0] = 0;
    c.g[T8][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
    c.g[T7][0] = 0xa;
    c.g[T6][0] = 0x6;
    c.g[T5][0] = 0x7;
    c.g[T4][0] = 0x8;
    c.g[T3][0] = 0x9;
    c.g[T2][0] = 0xb;
    c.g[V0][0] = 0x28;
    loop {
        // i: the table of three.
        c.cycles += 2;
        c.g[T0][0] = 0;
        c.mult(0, A0, T1, V0);
        'cols: loop {
            // j: the four rows of the table.
            c.cycles += 2;
            c.g[A3][0] = 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T8][0] as u32));
            loop {
                // k: the five entries of the row.
                if medal_entry_matches(c) {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T0][0];
                    ret(c);
                    return;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 5i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a3f0);
                }
            }
            // Next j.
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa));
            if !more {
                break 'cols;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a3e8);
            }
        }
        // Next i.
        c.cycles += 4;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x28;
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_a3e0);
        }
    }
}

/// One entry of the k loop (the `'b_15a440` block): true when the entry at
/// `v1` is the one asked for. A mismatch on the event counts a miss in `a3`.
fn medal_entry_matches(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    if c.g[V0][0] != c.g[A1][0] {
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        return false;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1)).lb(c);
    if c.g[V0][0] == c.g[A2][0] {
        return true;
    }
    for reg in [T6, T5, T4, T3, T7] {
        c.cycles += 2;
        if c.g[A1][0] == c.g[reg][0] {
            return true;
        }
    }
    c.cycles += 2;
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
    c.g[A1][0] == c.g[T2][0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng};

    #[test]
    fn be_reward_int_find_requirement_tier_matches() {
        check(
            ee::s01::f_0015a3b8__generated,
            be_reward_int_find_requirement_tier,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f5a_3b80 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_a3b8;
                // A table of small event ids (6..=11 and the others) and
                // location ids, so every branch is reached.
                for n in 0..60u32 {
                    let ev = (r.next_u32() % 14) as u8;
                    let loc = (r.next_u32() % 5) as u8;
                    c.bus.write8(BE_MEDALS_INFO + 2 * n, ev);
                    c.bus.write8(BE_MEDALS_INFO + 2 * n + 1, loc);
                }
                c.g[A1][0] = sx(r.next_u32() % 14);
                c.g[A2][0] = sx(r.next_u32() % 5);
            },
        );
    }
}
