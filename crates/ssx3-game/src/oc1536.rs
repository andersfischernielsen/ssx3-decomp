//! Staging module oc1536 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1536`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_c5d8, bolts_set_bolt_field_0c, "Bolts_SetBoltField0C"),
    (
        0x0014_c6a0,
        crate::game::be::belibrary::bolts_load_db,
        "Bolts_LoadDB",
    ),
    (
        0x0014_daa0,
        bolts_find_table2_entries_wrapper,
        "Bolts_FindTable2Entries_Wrapper",
    ),
    (0x0014_dac0, bolts_find_table2_entries, "Bolts_FindTable2Entries"),
    (0x0015_0ea8, state_thunk_00150ea8, "lbl_00150EA8"),
    (0x0015_0ed0, state_thunk_00150ed0, "lbl_00150ED0"),
    (0x0015_0ef8, state_thunk_00150ef8, "lbl_00150EF8"),
    (0x0015_0f20, state_thunk_00150f20, "lbl_00150F20"),
];

/// `void Bolts_SetBoltField0C(BoltsDB *db, int group, int key, short value)`
/// (0x14c5d8): find the bolt's index with `Bolts_FindBoltIndex` and store
/// `value` as a halfword at +0x0c of that bolt's 0x38-byte entry (the
/// entry table is at `db + 4`).
pub fn bolts_set_bolt_field_0c(c: &mut Cpu) {
    const FIND_BOLT_INDEX: Callee = Callee {
        addr: 0x0014_d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };

    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        FIND_BOLT_INDEX.call(c, 0x0014_c5f4)?;
        c.g[V1][0] = 0x38;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        c.mult(0, 2, 2, 3);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let value = c.g[S1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sh(c, value);
        frame.restore(c, 17, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_FindTable2Entries_Wrapper` (0x14daa0): forwards to
/// `Bolts_FindTable2Entries` with `a3` set to its own frame slot (the
/// out-count the callee increments).
pub fn bolts_find_table2_entries_wrapper(c: &mut Cpu) {
    const FIND_TABLE2_ENTRIES: Callee = Callee {
        addr: 0x0014_dac0,
        f: ee::s01::Bolts_FindTable2Entries,
    };

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[SP][0];
        FIND_TABLE2_ENTRIES.call(c, 0x0014_dab0)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_FindTable2Entries(BoltsDB *db, int group, int key, int *count)`
/// (0x14dac0, leaf): scan the group's entries (12 bytes each, from the
/// index `first` for `count` entries). Entries whose halfword at +2 equals
/// `key` are counted in `*count`; the first of them is returned. The scan
/// stops at the first entry above `key`.
pub fn bolts_find_table2_entries(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let count_out = c.g[A3][0] as u32;
    Addr(count_out).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20c)).lw(c);
    if (c.g[T0][0] as i64) > 0 {
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x284)).lw(c);
        'b_14db38: {
            c.cycles += 4;
            c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
            let empty = c.g[V0][0] == 0;
            c.g[T1][0] = 0;
            if empty {
                break 'b_14db38;
            }
            c.cycles += 2;
            c.g[V0][0] = 0xc;
            c.mult(0, 5, 3, 2);
            'l_14daf8: loop {
                'b_14db28: {
                    'b_14db20: {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(2)).lh(c);
                        let differs = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if differs {
                            break 'b_14db20;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = Addr(count_out).lw(c);
                        if c.g[T1][0] == 0 {
                            c.g[T1][0] = c.g[T0][0];
                        }
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        let count = c.g[V0][0];
                        Addr(count_out).sw(c, count);
                        break 'b_14db28;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        break 'b_14db38;
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                if !more {
                    break 'b_14db38;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_daf8);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[T1][0];
        ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
    }
}

// `Bolts_LoadDB` (0x14c6a0) is already ported in `game::be::belibrary`
// (`bolts_load_db`); it is registered in this module's table with its test.

/// Shared body of the four 40-byte thunks at 0x150ea8..0x150f20: compute the
/// address of this (T3, T4) pair's state byte, `0x4a6ca8 + 0x9b50*T3 +
/// 0xf88*T4 + off`, and tail-call the state dispatcher at 0x150fbc, which
/// bumps that byte (its `ra` is the thunk's caller).
fn state_thunk_tail(c: &mut Cpu, off: u32) {
    c.cycles += 10;
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, 3, 12, 3);
    c.mult(0, 4, 11, 4);
    c.g[V0][0] = sx(0x004a_6ca8);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
    c.pc = 0x0015_0fbc;
    call_indirect(c, 0x0015_0fbc);
}

/// `lbl_00150EA8` (0x150ea8): [`state_thunk_tail`] with offset 0xbdf.
pub fn state_thunk_00150ea8(c: &mut Cpu) {
    state_thunk_tail(c, 0xbdf);
}

/// `lbl_00150ED0` (0x150ed0): [`state_thunk_tail`] with offset 0xbe0.
pub fn state_thunk_00150ed0(c: &mut Cpu) {
    state_thunk_tail(c, 0xbe0);
}

/// `lbl_00150EF8` (0x150ef8): [`state_thunk_tail`] with offset 0xbe1.
pub fn state_thunk_00150ef8(c: &mut Cpu) {
    state_thunk_tail(c, 0xbe1);
}

/// `lbl_00150F20` (0x150f20): [`state_thunk_tail`] with offset 0xbe2.
pub fn state_thunk_00150f20(c: &mut Cpu) {
    state_thunk_tail(c, 0xbe2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fakes, DATA, RA as DIFF_RA};
    use crate::game::be::belibrary::bolts_load_db;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the callee fakes below answer only
    /// when it holds [`OC1536_MAGIC`] (otherwise they fall back to the body
    /// they replaced).
    const OC1536_CTRL: u32 = DATA + 0x20_0000;
    const OC1536_MAGIC: u32 = 0x1536_c0de;
    /// The forced `Bolts_FindBoltIndex` result, read by its fake.
    const OC1536_FIND_RESULT: u32 = OC1536_CTRL + 0x20;

    static OC1536_PREV_0014D448: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014D240: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014BE70: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014C2B0: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014C320: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014C3C8: AtomicUsize = AtomicUsize::new(0);
    static OC1536_PREV_0014C488: AtomicUsize = AtomicUsize::new(0);

    /// Log a call the gate lets through (`fakes::fake`; the fixtures use no
    /// call budget, so it never unwinds). With the gate off, run the body the
    /// fake replaced and return false.
    fn oc1536_gated(c: &mut Cpu, addr: u32, prev: &AtomicUsize, generated: fn(&mut Cpu)) -> bool {
        if c.bus.read32(OC1536_CTRL) != OC1536_MAGIC {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        true
    }

    fn oc1536_fake_find_bolt_index(c: &mut Cpu) {
        if oc1536_gated(
            c,
            0x0014_d448,
            &OC1536_PREV_0014D448,
            ee::s01::f_0014d448__generated,
        ) {
            let index = c.bus.read32(OC1536_FIND_RESULT);
            c.g[V0][0] = sx(index);
        }
    }

    fn oc1536_fake_free_db(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_d240,
            &OC1536_PREV_0014D240,
            ee::s01::f_0014d240__generated,
        );
    }

    fn oc1536_fake_read_file(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_be70,
            &OC1536_PREV_0014BE70,
            ee::s01::f_0014be70__generated,
        );
    }

    fn oc1536_fake_index_bolts(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_c2b0,
            &OC1536_PREV_0014C2B0,
            ee::s01::f_0014c2b0__generated,
        );
    }

    fn oc1536_fake_index_table2(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_c320,
            &OC1536_PREV_0014C320,
            ee::s01::f_0014c320__generated,
        );
    }

    fn oc1536_fake_index_table3(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_c3c8,
            &OC1536_PREV_0014C3C8,
            ee::s01::f_0014c3c8__generated,
        );
    }

    fn oc1536_fake_index_table4(c: &mut Cpu) {
        oc1536_gated(
            c,
            0x0014_c488,
            &OC1536_PREV_0014C488,
            ee::s01::f_0014c488__generated,
        );
    }

    /// Hook the callee fakes (once per process; each falls back when the
    /// gate is off).
    fn oc1536_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014D448),
            oc1536_fake_find_bolt_index,
            &OC1536_PREV_0014D448,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014D240),
            oc1536_fake_free_db,
            &OC1536_PREV_0014D240,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014BE70),
            oc1536_fake_read_file,
            &OC1536_PREV_0014BE70,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C2B0),
            oc1536_fake_index_bolts,
            &OC1536_PREV_0014C2B0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C320),
            oc1536_fake_index_table2,
            &OC1536_PREV_0014C320,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C3C8),
            oc1536_fake_index_table3,
            &OC1536_PREV_0014C3C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C488),
            oc1536_fake_index_table4,
            &OC1536_PREV_0014C488,
        );
    }

    /// The `Bolts_FindTable2Entries` fixture: one database whose group table
    /// holds a few 12-byte entries, halfword +2 = level.
    fn oc1536_table2_setup(c: &mut Cpu, seed: u32) {
        let _ = fakes::machine(c, seed, 0x14dac0, 0);
        let db = DATA + 0x1000;
        let entries = DATA + 0x3000;
        let group = seed % 4;
        let count = (seed / 4) % 4;
        let first = (seed / 16) % 3;
        let key = (seed / 3) % 3;
        c.g[A0][0] = sx(db);
        c.g[A1][0] = sx(group);
        c.g[A2][0] = sx(key);
        c.g[A3][0] = sx(DATA + 0x2000);
        c.bus.write32(db + 8, entries);
        c.bus.write32(db + 0x20c + 4 * group, count);
        c.bus.write32(db + 0x284 + 4 * group, first);
        c.bus.write32(DATA + 0x2000, seed % 3);
        for i in 0..first + count + 2 {
            c.bus.write16(entries + 12 * i + 2, ((i + seed) % 4) as u16);
        }
    }

    /// The `Bolts_LoadDB` fixture: a loaded database (the early return), or
    /// the full reload with the callees faked and the four index loops fed
    /// a few entries.
    fn oc1536_load_db_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x14c6a0, 0);
        c.bus.write32(OC1536_CTRL, OC1536_MAGIC);
        let this = DATA + 0x1000;
        c.g[A0][0] = sx(this);
        if seed % 3 == 0 {
            c.bus.write32(this + 0x10, DATA + 0x3000);
            return;
        }
        c.bus.write32(this + 0x10, 0);
        // 0 skips the table loops (the count is below one).
        c.bus.write32(this + 4, (seed / 3) % 2);
        // Loop counts at the global table for (outer k, inner j).
        for (k, j, n) in [(0u32, 0u32, 2u32), (1, 3, 1), (2, 9, 3)] {
            let base = 0x004a_6ca8 + k * 0x9b50 + j * 0xf88;
            c.bus.write32(base + 0x28c, n);
            for i in 0..n {
                c.bus.write32(base + 0x290 + 4 * i, r.next_u32());
            }
        }
    }

    /// A state thunk fixture: the (T3, T4) pair's state byte, 5 (done) or
    /// an active value; the caller's `ra` is the test return address.
    fn oc1536_thunk_setup(c: &mut Cpu, seed: u32, off: u32) {
        let _ = fakes::machine(c, seed, 0x150ea8, 0);
        c.g[RA][0] = sx(DIFF_RA);
        let t3 = seed % 5;
        let t4 = (seed / 5) % 4;
        c.g[T3][0] = t3 as u64;
        c.g[T4][0] = t4 as u64;
        let state = if seed % 2 == 0 {
            5
        } else {
            ((seed / 2) % 4) as u8
        };
        let t1 = 0x004a_6ca8 + t3 * 0x9b50 + t4 * 0xf88 + off;
        c.bus.write8(t1, state);
    }

    #[test]
    fn bolts_set_bolt_field_0c_matches() {
        oc1536_install();
        check_lohi(
            ee::s01::f_0014c5d8__generated,
            bolts_set_bolt_field_0c,
            200,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x14c5d8, 0);
                c.bus.write32(OC1536_CTRL, OC1536_MAGIC);
                c.bus.write32(OC1536_FIND_RESULT, seed % 8);
                let this = DATA + 0x1000;
                c.bus.write32(this + 4, DATA + 0x4000);
                c.g[A0][0] = sx(this);
                c.g[A3][0] = sx(0x1234 + seed * 7);
            },
        );
    }

    #[test]
    fn bolts_load_db_matches() {
        oc1536_install();
        check_lohi(
            ee::s01::f_0014c6a0__generated,
            bolts_load_db,
            200,
            &|c, seed| oc1536_load_db_setup(c, seed),
        );
    }

    #[test]
    fn bolts_find_table2_entries_matches() {
        check_lohi(
            ee::s01::f_0014dac0__generated,
            bolts_find_table2_entries,
            200,
            &|c, seed| oc1536_table2_setup(c, seed),
        );
    }

    #[test]
    fn bolts_find_table2_entries_wrapper_matches() {
        check_lohi(
            ee::s01::f_0014daa0__generated,
            bolts_find_table2_entries_wrapper,
            200,
            &|c, seed| oc1536_table2_setup(c, seed),
        );
    }

    #[test]
    fn state_thunk_00150ea8_matches() {
        check_lohi(
            ee::s01::f_00150ea8__generated,
            state_thunk_00150ea8,
            200,
            &|c, seed| oc1536_thunk_setup(c, seed, 0xbdf),
        );
    }

    #[test]
    fn state_thunk_00150ed0_matches() {
        check_lohi(
            ee::s01::f_00150ed0__generated,
            state_thunk_00150ed0,
            200,
            &|c, seed| oc1536_thunk_setup(c, seed, 0xbe0),
        );
    }

    #[test]
    fn state_thunk_00150ef8_matches() {
        check_lohi(
            ee::s01::f_00150ef8__generated,
            state_thunk_00150ef8,
            200,
            &|c, seed| oc1536_thunk_setup(c, seed, 0xbe1),
        );
    }

    #[test]
    fn state_thunk_00150f20_matches() {
        check_lohi(
            ee::s01::f_00150f20__generated,
            state_thunk_00150f20,
            200,
            &|c, seed| oc1536_thunk_setup(c, seed, 0xbe2),
        );
    }
}
