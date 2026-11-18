//! Staging module oc426 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc426`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0015_a818, rewards_file_parse, "RewardsFile_Parse")];

/// `RewardsFile` field offsets (`self`).
pub(crate) const REWARDS_BASE: u32 = 0x4;
pub(crate) const REWARDS_STRINGS: u32 = 0x2c;
/// Jump table of the section parser: one handler address per section type.
pub(crate) const REWARDS_JUMP_TABLE: u32 = 0x0045_ab50;

#[cfg(not(feature = "ee-unwind"))]
fn rewards_parse_resume(c: &mut Cpu) {
    ee::s01::f_0015a818__entries(c)
}
#[cfg(feature = "ee-unwind")]
fn rewards_parse_resume(c: &mut Cpu) {
    ee::s01::f_0015a818__generated(c)
}

/// One string-offset fixup: the word at `a0 + off`, when non-zero, becomes
/// `strings + word - 1`. `pre` is the cycle charge of the block head.
fn rewards_fixup_word(c: &mut Cpu, off: u32, pre: u64) {
    c.cycles += pre;
    let at = (c.g[A0][0] as u32).wrapping_add(off);
    c.g[V1][0] = c.lw(at);
    let zero = c.g[V1][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    if !zero {
        c.cycles += 3;
        let strings = c.lw((c.g[A3][0] as u32).wrapping_add(REWARDS_STRINGS));
        c.g[V0][0] = strings;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(at, c.g[V0][0]);
    }
}

/// Fixup loop over the 12-byte entries of the first two tables (`a2` is the
/// byte offset, the table pointer is reloaded from `self + tbl` each time).
/// The count is in `v0` and `a1 = 0` on entry (generated block head).
fn rewards_fix_stride12(c: &mut Cpu, tbl: u32, cnt: u32, head: u32) {
    if (c.g[V0][0] as i64) <= 0 {
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = 0;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        rewards_fixup_word(c, 0, 0);
        rewards_fixup_word(c, 4, 3);
        rewards_fixup_word(c, 8, 3);
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Fixup loop over the 8- or 16-byte entries of the other tables
/// (`v1` holds the table pointer, `shift` the entry size, `words` how many
/// string words each entry has). Same entry state as [`rewards_fix_stride12`].
fn rewards_fix_shifted(c: &mut Cpu, tbl: u32, cnt: u32, shift: u32, words: u32, head: u32) {
    if (c.g[V0][0] as i64) <= 0 {
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << shift);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        rewards_fixup_word(c, 0, 0);
        rewards_fixup_word(c, 4, 3);
        if words == 3 {
            rewards_fixup_word(c, 8, 3);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Header of a fixup loop: `cycles += pre`, load the count at `self + cnt`
/// into `v0`, `a1 = 0`.
fn rewards_fix_head(c: &mut Cpu, pre: u64, cnt: u32) {
    c.cycles += pre;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
    c.g[A1][0] = 0;
}

/// `void RewardsFile_Parse(RewardsFile* self, int* data)` (0x15a818): parse a
/// rewards file in place. The data starts with nine `(type, count)` headers,
/// each followed by `count` table entries (12 bytes for types 0 and 1, 8 for
/// type 2, 16 for the rest); the jump table at 0x45ab50 sends a type to its
/// handler, which stores the table pointer and entry count in `self`. The
/// string pool follows (after one more word); every non-zero string offset in
/// the entries then becomes a pointer (`strings + offset - 1`).
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn rewards_file_parse(c: &mut Cpu) {
    if c.pc != 0x0015_a818 {
        return rewards_parse_resume(c);
    }
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[T0][0] = 0;
    c.sw((c.g[A3][0] as u32).wrapping_add(REWARDS_BASE), c.g[A1][0]);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    loop {
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[A2][0] = c.lw(c.g[A1][0] as u32);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 9) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x46_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x54b0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                // (table pointer offset, count offset, entry shift)
                let shifted = |off: u32, cnt: u32, shift: u32, c: &mut Cpu| {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << shift);
                    c.sw((c.g[A3][0] as u32).wrapping_add(off), c.g[A1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[A3][0] as u32).wrapping_add(cnt), c.g[A2][0]);
                };
                // Types 0 and 1: 12-byte entries, `v1 = count * 12`.
                let stride12 = |off: u32, cnt: u32, c: &mut Cpu| {
                    c.cycles += 6;
                    c.g[V0][0] = 0xc;
                    c.sw((c.g[A3][0] as u32).wrapping_add(off), c.g[A1][0]);
                    c.mult(0, 3, 6, 2);
                    c.sw((c.g[A3][0] as u32).wrapping_add(cnt), c.g[A2][0]);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                };
                match target {
                    0x0015_a85c => stride12(0x8, 0x30, c),
                    0x0015_a874 => stride12(0xc, 0x34, c),
                    0x0015_a88c => shifted(0x10, 0x38, 3, c),
                    0x0015_a8a0 => shifted(0x14, 0x3c, 4, c),
                    0x0015_a8b4 => shifted(0x18, 0x40, 4, c),
                    0x0015_a8c8 => shifted(0x1c, 0x44, 4, c),
                    0x0015_a8dc => shifted(0x24, 0x4c, 4, c),
                    0x0015_a8f0 => shifted(0x20, 0x48, 4, c),
                    0x0015_a904 => {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x50), c.g[A2][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    _ => {
                        c.jump_out(target, 0x0015_a818, 0x0015_ad54);
                        return;
                    }
                }
            }
        }
        c.cycles += 4;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[T0][0] as i64) < 9) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        if event_due(c) {
            deliver_event_here(c, 0x0015_a828);
        }
    }
    // String pool, then the fixups of each table.
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x30));
    c.sw((c.g[A3][0] as u32).wrapping_add(REWARDS_STRINGS), c.g[A1][0]);
    c.g[A1][0] = 0;
    rewards_fix_stride12(c, 0x8, 0x30, 0x0015_a940);
    rewards_fix_head(c, 3, 0x34);
    rewards_fix_stride12(c, 0xc, 0x34, 0x0015_a9b8);
    rewards_fix_head(c, 3, 0x38);
    rewards_fix_shifted(c, 0x10, 0x38, 3, 2, 0x0015_aa30);
    rewards_fix_head(c, 3, 0x3c);
    rewards_fix_shifted(c, 0x14, 0x3c, 4, 3, 0x0015_aa90);
    rewards_fix_head(c, 3, 0x40);
    rewards_fix_shifted(c, 0x18, 0x40, 4, 3, 0x0015_ab08);
    rewards_fix_head(c, 3, 0x44);
    rewards_fix_shifted(c, 0x1c, 0x44, 4, 3, 0x0015_ab80);
    rewards_fix_head(c, 3, 0x4c);
    rewards_fix_shifted(c, 0x24, 0x4c, 4, 3, 0x0015_abf8);
    rewards_fix_head(c, 3, 0x48);
    rewards_fix_shifted(c, 0x20, 0x48, 4, 3, 0x0015_ac70);
    rewards_fix_head(c, 3, 0x50);
    rewards_fix_shifted(c, 0x28, 0x50, 4, 3, 0x0015_ace8);
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check_lohi, scramble, Rng, DATA, RA as SENTINEL};

    /// Fixture: a rewards file at `DATA + 0x1000` (nine headers in random
    /// type order, some empty or out of range, 0..3 entries each with
    /// random zero / non-zero string offsets), `self` at `DATA`.
    fn rewards_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x15a8_1800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        let handlers = [
            0x15a85cu32, 0x15a874, 0x15a88c, 0x15a8a0, 0x15a8b4, 0x15a8c8, 0x15a8dc, 0x15a8f0,
            0x15a904,
        ];
        for (k, h) in handlers.iter().enumerate() {
            c.bus.write32(REWARDS_JUMP_TABLE + 4 * k as u32, *h);
        }
        let mut order: Vec<u32> = (0..9).collect();
        for i in (1..9).rev() {
            order.swap(i, r.next_u32() as usize % (i + 1));
        }
        let data = DATA + 0x1000;
        let mut p = data;
        for n in 0..9 {
            let mut ty = order[n];
            if seed % 4 == 1 && r.next_u32() % 3 == 0 {
                ty = 9 + r.next_u32() % 3;
            }
            let count = match r.next_u32() % 4 {
                0 => 0,
                k => k,
            };
            c.bus.write32(p, ty);
            c.bus.write32(p + 4, count);
            p += 8;
            let stride = match ty {
                0 | 1 => 12,
                2 => 8,
                _ => 16,
            };
            if count != 0 && ty < 9 {
                for w in 0..count * stride / 4 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 0x100
                    };
                    c.bus.write32(p + 4 * w, v);
                }
                p += count * stride;
            }
        }
        // the word before the string pool
        c.bus.write32(p, 0x1234);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(data);
    }

    /// `RewardsFile_Parse`: random section orders and counts, out-of-range
    /// types, zero and non-zero string offsets (LO/HI too).
    #[test]
    fn rewards_file_parse_matches() {
        check_lohi(ee::s01::f_0015a818__generated, rewards_file_parse, 300, &|c, seed| {
            rewards_fixture(c, seed);
            c.pc = 0x0015_a818;
        });
    }

    /// Event delivery at the parse loop and the nine fixup loop heads.
    #[test]
    fn rewards_file_parse_events_match() {
        check_events(ee::s01::f_0015a818__generated, rewards_file_parse, 240, &|c, seed| {
            rewards_fixture(c, seed);
            let mut r = Rng(0x15a8_1801 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0015_a818);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[RA][0] = SENTINEL as u64;
            mode
        });
    }
}
