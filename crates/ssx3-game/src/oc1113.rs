//! Staging module oc1113 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1113`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_67f8, spch_shutdown, "SPCH_Shutdown"),
    (
        0x003d_69a8,
        spch_reset_channel_events,
        "SPCH_ResetChannelEvents",
    ),
    (0x003d_6bc0, spch_set_variation, "SPCH_SetVariation"),
    (0x003d_6ca0, spch_remove_bank, "SPCH_RemoveBank"),
    (0x003d_6cf8, spch_bank_has_sample, "SPCH_BankHasSample"),
    (0x003d_6d30, spch_clear_bank_table, "SPCH_ClearBankTable"),
    (0x003d_6db8, spch_find_bank, "SPCH_FindBank"),
    (
        0x003d_6f80,
        spch_bank_mark_variation_bits,
        "SPCH_BankMarkVariationBits",
    ),
];

/// Speech driver globals, gp-relative: `D_004A4820` (the init marker, 0x01789A34
/// while the driver is up), `D_004A4824` (the speech table list), `D_004A4828`
/// (its slot count) and `D_004A4834` (the random-subset seed).
const SPCH_GP_INIT: u32 = 0x1730;
const SPCH_GP_TABLES: u32 = 0x1734;
const SPCH_GP_TABLE_COUNT: u32 = 0x1738;
const SPCH_GP_SEED: u32 = 0x1744;

/// The callback block shutdown clears (`D_0044FF90`: three words).
const SPCH_CALLBACK_BLOCK: u32 = 0x0044_ff90;

/// The generated speech helpers that the ports call (through their dispatchers,
/// so a replaced callee is reached too).
const CALLEE_CLEAR_BANK_TABLE: Callee = Callee {
    addr: 0x003d_6d30,
    f: ee::s14::f_003d6d30,
};
const CALLEE_CLEAR_EVENT_TABLES: Callee = Callee {
    addr: 0x003d_8278,
    f: ee::s14::f_003d8278,
};
const CALLEE_BANK_CLEAR_USAGE_BITS: Callee = Callee {
    addr: 0x003d_7038,
    f: ee::s14::f_003d7038,
};
const CALLEE_BANK_MARK_VARIATION_BITS: Callee = Callee {
    addr: 0x003d_6f80,
    f: ee::s14::f_003d6f80,
};

/// A speech driver global: the word at `gp + off`.
fn spch_bank_global(c: &Cpu, off: u32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_add(off))
}

/// `SPCH_Shutdown()` (0x3d67f8): when the driver is up (`D_004A4820 ==
/// 0x01789A34`), clears the marker and the callback block `D_0044FF90`, then
/// forgets the speech table list (`SPCH_ClearBankTable`) and clears the channel
/// events (`SPCH_ClearEventTables`).
pub fn spch_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0178_0000;
        c.g[V1][0] = spch_bank_global(c, SPCH_GP_INIT).lw(c);
        c.g[V0][0] |= 0x9a34;
        let initialised = c.g[V1][0] == c.g[V0][0];
        frame.save(c, RA, 0);
        if initialised {
            c.cycles += 9;
            c.g[V0][0] = 0x0045_0000;
            spch_bank_global(c, SPCH_GP_INIT).sw(c, 0);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70));
            Addr(SPCH_CALLBACK_BLOCK).sw(c, 0);
            Addr(SPCH_CALLBACK_BLOCK + 0x8).sw(c, 0);
            Addr(SPCH_CALLBACK_BLOCK + 0x4).sw(c, 0);
            CALLEE_CLEAR_BANK_TABLE.call(c, 0x003d_682c)?;
            CALLEE_CLEAR_EVENT_TABLES.call(c, 0x003d_6834)?;
        }
        c.cycles += 3;
        frame.restore(c, RA, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SPCH_ResetChannelEvents()` (0x3d69a8): the 8 channel entries of
/// `D_00450660` (a halfword id and a word each) get id 0xFFFF and word 0.
pub fn spch_reset_channel_events(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0045_0000;
    c.g[A1][0] = 0;
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x660));
    c.g[T0][0] = 0xffff;
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
    loop {
        c.cycles += 10;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[A1][0] = c.g[A1][0].wrapping_add(1);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A0][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        let id = c.g[T0][0];
        Addr(c.g[V1][0] as u32).sh(c, id);
        let more = c.g[A0][0] != 0;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_69c0);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `SPCH_SetVariation(seed)` (0x3d6bc0): when the driver is up, stores the seed
/// (`D_004A4834`) and re-draws every speech table's random subset. Tables with
/// flag 0x80 clear their subset (`SPCH_BankClearUsageBits`) and mark the new
/// one (`SPCH_BankMarkVariationBits`).
pub fn spch_set_variation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = spch_bank_global(c, SPCH_GP_INIT).lw(c);
        frame.save(c, RA, 0x20);
        frame.save(c, S1, 0x10);
        let initialised = c.g[V0][0] != 0;
        frame.save(c, S0, 0);
        if initialised {
            c.cycles += 3;
            c.g[V0][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
            let has_list = c.g[V0][0] != 0;
            let seed = c.g[A0][0];
            spch_bank_global(c, SPCH_GP_SEED).sw(c, seed);
            if has_list {
                c.cycles += 3;
                c.g[V0][0] = spch_bank_global(c, SPCH_GP_TABLE_COUNT).lw(c);
                let counted = (c.g[V0][0] as i64) > 0;
                c.g[S1][0] = 0;
                if counted {
                    c.cycles += 2;
                    c.g[V1][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
                    loop {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
                        if c.g[S0][0] != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lbu(c) & 0x80;
                            if c.g[V0][0] != 0 {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                CALLEE_BANK_CLEAR_USAGE_BITS.call(c, 0x003d_6c24)?;
                                c.g[A0][0] = c.g[S0][0];
                                CALLEE_BANK_MARK_VARIATION_BITS.call(c, 0x003d_6c2c)?;
                            }
                        }
                        c.g[V0][0] = spch_bank_global(c, SPCH_GP_TABLE_COUNT).lw(c);
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_6bf8);
                        }
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, RA, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SPCH_RemoveBank(slot)` (0x3d6ca0): clears slot `slot` of the speech table
/// list (no range check); nothing when there is no list.
pub fn spch_remove_bank(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
    let has_list = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
    if has_list {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V0][0] as u32).sw(c, 0);
    }
    c.cycles += 2;
    ret(c);
}

/// `SPCH_BankHasSample(slot, entryIndex)` (0x3d6cf8): 1 when `entryIndex` is
/// below the entry count (halfword +0xA) of the table in `slot`; 0 when the count
/// is 0xFFFF, the slot is negative or there is no list.
pub fn spch_bank_has_sample(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
    let has_list = c.g[V1][0] != 0;
    c.g[A2][0] = 0;
    if has_list {
        c.cycles += 2;
        let slot_negative = (c.g[A0][0] as i64) < 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        if !slot_negative {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 0xffff;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xa).lhu(c);
            let not_full = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = c.g[V1][0];
            if not_full {
                c.cycles += 1;
                c.g[A2][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// `SPCH_ClearBankTable()` (0x3d6d30): forgets the speech table list (its
/// pointer and its slot count).
pub fn spch_clear_bank_table(c: &mut Cpu) {
    c.cycles += 3;
    spch_bank_global(c, SPCH_GP_TABLES).sw(c, 0);
    spch_bank_global(c, SPCH_GP_TABLE_COUNT).sw(c, 0);
    ret(c);
}

/// `SPCH_FindBank(tableId, subId)` (0x3d6db8): the slot of the first table whose
/// halfwords 0 and 2 (id, sub id) match, or -1.
pub fn spch_find_bank(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = spch_bank_global(c, SPCH_GP_TABLES).lw(c);
    c.g[A0][0] &= 0xffff;
    let has_list = c.g[V1][0] != 0;
    c.g[A1][0] &= 0xffff;
    if has_list {
        c.cycles += 3;
        c.g[V0][0] = spch_bank_global(c, SPCH_GP_TABLE_COUNT).lw(c);
        let counted = (c.g[V0][0] as i64) > 0;
        c.g[A3][0] = 0;
        if counted {
            c.cycles += 5;
            c.g[T1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[V1][0];
            c.g[T0][0] = 0x10000;
            c.g[T2][0] = 0x10000;
            loop {
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                let has_table = c.g[V1][0] != 0;
                c.g[V0][0] = c.g[T0][0];
                if has_table {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                    let id_match = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = c.g[T0][0];
                    if id_match {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2).lhu(c);
                        let sub_match = c.g[V0][0] == c.g[A1][0];
                        c.g[V0][0] = c.g[T0][0];
                        if sub_match {
                            c.cycles += 2;
                            c.g[V0][0] = c.g[A3][0];
                            ret(c);
                            return;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_6de8);
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    ret(c);
}

/// `SPCH_BankMarkVariationBits(table)` (0x3d6f80): marks the current subset of a
/// random-subset speech table in its row bitmask. With `c` the subset count (the
/// byte after the rows), subset `seed % c` covers rows `r*w/c` up to
/// `(r+1)*w/c` (`w` the row count, byte +5), whose bits are set. Nothing when
/// `c` is 0.
pub fn spch_bank_mark_variation_bits(c: &mut Cpu) {
    let table = c.g[A0][0] as u32;
    c.cycles += 13;
    c.g[V1][0] = Addr(table).off(0x4).lbu(c);
    c.g[A1][0] = Addr(table).off(0x5).lbu(c);
    c.g[V1][0] &= 0xf;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
    let rows = c.g[A1][0] as u32;
    let cols_plus_two = c.g[V0][0] as u32;
    let product = crate::abi::mult(c, rows, cols_plus_two);
    c.g[V0][0] = sx(product);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
    c.g[V0][0] &= 0x3ffc;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = Addr(c.g[A3][0] as u32).lbu(c);
    let no_subsets = (c.g[A0][0] as i64) <= 0;
    c.g[V1][0] = spch_bank_global(c, SPCH_GP_SEED).lw(c);
    if !no_subsets {
        c.cycles += 16;
        let seed = c.g[V1][0] as u32;
        let count = c.g[A0][0] as u32;
        crate::abi::divu(c, seed, count);
        c.g[V0][0] = c.hi[0];
        let subset = c.g[V0][0] as u32;
        let subset_rows = crate::abi::mult(c, subset, rows);
        c.g[V1][0] = sx(subset_rows);
        c.g[A2][0] = sx(subset_rows);
        crate::abi::divu(c, subset_rows, count);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = c.lo[0];
        let end_numerator = c.g[V0][0] as u32;
        crate::abi::divu(c, end_numerator, count);
        c.g[A1][0] = c.g[V1][0] & 0x7;
        c.g[A0][0] = sx((c.g[V1][0] as u32) >> 3);
        c.g[V0][0] = c.lo[0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let empty = (c.g[A2][0] as i64) <= 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[T0][0] = 1;
            c.g[A3][0] = 8;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
            loop {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[T0][0] as u32) << ((c.g[A1][0] as u32) & 31));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] |= c.g[V1][0];
                let more_bits = c.g[A1][0] != c.g[A3][0];
                let byte = c.g[V0][0];
                Addr(c.g[A0][0] as u32).sb(c, byte);
                if !more_bits {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                if c.g[A2][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7008);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    /// The test speech table list: slot words at `SPCH_TEST_LIST`.
    const SPCH_TEST_LIST: u32 = DATA + 0x8000;
    /// The test speech tables: 0x400 bytes each from `SPCH_TEST_TABLES`.
    const SPCH_TEST_TABLES: u32 = DATA + 0x9000;
    const SPCH_TEST_TABLE_SIZE: u32 = 0x400;

    /// A scrambled register file, the game's gp, and the test's random source.
    fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Write a driver global (gp-relative offset).
    fn set_global(c: &mut Cpu, off: u32, v: u32) {
        c.bus.write32(GAME_GP + off, v);
    }

    /// A speech table at `node`: the flags (columns `cols`, the random-subset
    /// bit when `subset`), `rows` rows, and `subsets` in its subset count byte
    /// with random bitmask bytes after it.
    fn write_table(
        c: &mut Cpu,
        r: &mut Rng,
        node: u32,
        cols: u32,
        rows: u32,
        subset: bool,
        subsets: u32,
    ) {
        let flags: u8 = ((cols & 0xf) as u8) | (if subset { 0x80 } else { 0 });
        c.bus.write8(node + 4, flags);
        c.bus.write8(node + 5, rows as u8);
        let area = node + ((rows * (cols + 2) + 0xf) & 0x3ffc) + (cols & 0xf) * 4;
        c.bus.write8(area, subsets as u8);
        for i in 1..=(rows / 8 + 2) {
            let b = r.next_u32() as u8;
            c.bus.write8(area + i, b);
        }
    }

    #[test]
    fn spch_shutdown_matches() {
        check(
            ee::s14::f_003d67f8__generated,
            spch_shutdown,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_67f8);
                let init = if seed % 3 == 0 { 0 } else { 0x0178_9a34 };
                set_global(c, SPCH_GP_INIT, init);
                set_global(c, SPCH_GP_TABLES, SPCH_TEST_LIST);
                set_global(c, SPCH_GP_TABLE_COUNT, 1 + r.next_u32() % 4);
                for i in 0..3 {
                    c.bus.write32(SPCH_CALLBACK_BLOCK + 4 * i, r.next_u32());
                }
                for i in 0..16 {
                    c.bus.write32(0x0044_ff9c + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn spch_reset_channel_events_matches() {
        check(
            ee::s14::f_003d69a8__generated,
            spch_reset_channel_events,
            200,
            &|c: &mut Cpu, seed: u32| {
                start(c, seed, 0x1113_69a8);
                for i in 0..16 {
                    c.bus.write32(0x0045_0660 + 4 * i, 0x1234_5678);
                }
            },
        );
    }

    #[test]
    fn spch_set_variation_matches() {
        check(
            ee::s14::f_003d6bc0__generated,
            spch_set_variation,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_6bc0);
                let init = if seed % 6 == 0 { 0 } else { 0x0178_9a34 };
                set_global(c, SPCH_GP_INIT, init);
                let list = if seed % 7 == 1 { 0 } else { SPCH_TEST_LIST };
                set_global(c, SPCH_GP_TABLES, list);
                let count = match seed % 9 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    k => k % 4 + 1,
                };
                set_global(c, SPCH_GP_TABLE_COUNT, count);
                for k in 0..4u32 {
                    let node = SPCH_TEST_TABLES + k * SPCH_TEST_TABLE_SIZE;
                    let slot = if k == 2 { 0 } else { node };
                    c.bus.write32(SPCH_TEST_LIST + 4 * k, slot);
                    let cols = 1 + r.next_u32() % 4;
                    let rows = 1 + r.next_u32() % 12;
                    let subsets = r.next_u32() % 5;
                    let subset = (seed + k) % 2 == 0;
                    write_table(c, &mut r, node, cols, rows, subset, subsets);
                }
            },
        );
    }

    #[test]
    fn spch_remove_bank_matches() {
        check(
            ee::s14::f_003d6ca0__generated,
            spch_remove_bank,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_6ca0);
                let list = if seed % 4 == 0 { 0 } else { SPCH_TEST_LIST };
                set_global(c, SPCH_GP_TABLES, list);
                for k in 0..8 {
                    c.bus.write32(SPCH_TEST_LIST + 4 * k, r.next_u32());
                }
                c.g[A0][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn spch_bank_has_sample_matches() {
        check(
            ee::s14::f_003d6cf8__generated,
            spch_bank_has_sample,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_6cf8);
                let list = if seed % 5 == 0 { 0 } else { SPCH_TEST_LIST };
                set_global(c, SPCH_GP_TABLES, list);
                for k in 0..6u32 {
                    let node = SPCH_TEST_TABLES + k * SPCH_TEST_TABLE_SIZE;
                    c.bus.write32(SPCH_TEST_LIST + 4 * k, node);
                    let entries = if (seed + k) % 7 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write16(node + 0xa, entries as u16);
                }
                c.g[A0][0] = sx(((seed % 6) as u32).wrapping_sub(1));
                c.g[A1][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    #[test]
    fn spch_clear_bank_table_matches() {
        check(
            ee::s14::f_003d6d30__generated,
            spch_clear_bank_table,
            100,
            &|c: &mut Cpu, seed: u32| {
                start(c, seed, 0x1113_6d30);
                set_global(c, SPCH_GP_TABLES, SPCH_TEST_LIST + seed);
                set_global(c, SPCH_GP_TABLE_COUNT, 1 + seed);
            },
        );
    }

    #[test]
    fn spch_find_bank_matches() {
        check(
            ee::s14::f_003d6db8__generated,
            spch_find_bank,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_6db8);
                let list = if seed % 5 == 0 { 0 } else { SPCH_TEST_LIST };
                set_global(c, SPCH_GP_TABLES, list);
                let count = if seed % 6 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                set_global(c, SPCH_GP_TABLE_COUNT, count);
                for k in 0..4u32 {
                    let node = SPCH_TEST_TABLES + k * SPCH_TEST_TABLE_SIZE;
                    let slot = if (seed + k) % 4 == 3 { 0 } else { node };
                    c.bus.write32(SPCH_TEST_LIST + 4 * k, slot);
                    let id = (r.next_u32() % 3) as u16;
                    let sub = (r.next_u32() % 3) as u16;
                    c.bus.write16(node, id);
                    c.bus.write16(node + 2, sub);
                }
                c.g[A0][0] = (r.next_u32() % 3) as u64;
                c.g[A1][0] = (r.next_u32() % 3) as u64;
            },
        );
    }

    #[test]
    fn spch_bank_mark_variation_bits_matches() {
        check_lohi(
            ee::s14::f_003d6f80__generated,
            spch_bank_mark_variation_bits,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = start(c, seed, 0x1113_6f80);
                set_global(c, SPCH_GP_SEED, r.next_u32());
                let node = SPCH_TEST_TABLES + (seed % 4) * SPCH_TEST_TABLE_SIZE;
                let cols = 1 + r.next_u32() % 4;
                let rows = 1 + r.next_u32() % 40;
                let subsets = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                write_table(c, &mut r, node, cols, rows, true, subsets);
                c.g[A0][0] = node as u64;
            },
        );
    }
}
