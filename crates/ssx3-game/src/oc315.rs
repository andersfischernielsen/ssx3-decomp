//! Staging wave 213 (oc315): hand-written replacements for functions with no
//! C++ in the reference, ported with Ghidra's decompilation as the readable
//! aid. Each reproduces the generated body's registers, cycles, memory and
//! call order.

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
    (0x0035_1010, restore_node_save, "RestoreNode_Save"),
    (0x003d_b838, spch_decode_sample_entry, "SPCH_DecodeSampleEntry"),
];

// ---------------------------------------------------------------------------
// RestoreNode_Save (0x351010).

/// `WorldObj_Save(self, stream)`.
pub(crate) const RESTORE_NODE_WORLD_OBJ_SAVE: Callee = Callee {
    addr: 0x0034_fe90,
    f: ee::s11::WorldObj_Save,
};

/// `void RestoreNode_Save(RestoreNode* self, Stream* stream)` (0x351010): a
/// restore node saves only its world-object part: `WorldObj_Save(self,
/// stream)`. Frame 0x10 (ra +0).
pub fn restore_node_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(RESTORE_NODE_WORLD_OBJ_SAVE),
        0x0035_1020,
    );
}

// ---------------------------------------------------------------------------
// SPCH_DecodeSampleEntry (0x3db838).

/// Speech bank header: low nibble = extra bytes per entry.
pub(crate) const SPCH_DECODE_FLAGS: u32 = 0x4;
/// Speech bank header: number of entries.
pub(crate) const SPCH_DECODE_COUNT: u32 = 0x5;
/// Speech bank header: block size in 256-byte units, minus one.
pub(crate) const SPCH_DECODE_BLOCK: u32 = 0x7;
/// Speech bank header: (u16) end block of the last entry.
pub(crate) const SPCH_DECODE_END: u32 = 0x8;
/// Speech bank: first entry. Each is `2 + extra` bytes: a big-endian u16
/// start block, then the extra bytes; bit 7 of the first byte marks an
/// alias whose second byte is the index of the entry it shares.
pub(crate) const SPCH_DECODE_ENTRIES: u32 = 0xc;
/// Output record: length (word).
pub(crate) const SPCH_DECODE_OUT_LEN: u32 = 0x0;
/// Output record: start offset (word).
pub(crate) const SPCH_DECODE_OUT_START: u32 = 0x4;
/// Output record: number of extra bytes (word).
pub(crate) const SPCH_DECODE_OUT_EXTRA_N: u32 = 0x8;
/// Output record: the extra bytes (4 reset to 0xff, up to 15 copied).
pub(crate) const SPCH_DECODE_OUT_EXTRA: u32 = 0xc;

/// `bool SPCH_DecodeSampleEntry(SpchBank* bank, int index, SpchEntry* out)`
/// (0x3db838), a leaf: fills `out` with entry `index`'s start offset (start
/// block * block size), its length (up to the next entry that is not an
/// alias, else the bank's end) and its extra bytes. Returns 0 (with `out`
/// reset to len 0 / start -1 / extras 0xff) when `index` is not below the
/// entry count, else 1. The multiplies leave LO/HI as the original's; the
/// body works in the original's registers (t2 bank, a1 index, a3 entry,
/// t5 entry size, t0/t1 cursor/limit, t4 end, t6 found, a0 result).
pub fn spch_decode_sample_entry(c: &mut Cpu) {
    c.cycles += 16;
    let out = c.g[A2][0] as u32;
    c.g[V1][0] = 0xff;
    c.g[V0][0] = u64::MAX;
    Addr(out).off(0xf).sb(c, 0xff);
    c.g[T2][0] = c.g[A0][0];
    Addr(out).off(SPCH_DECODE_OUT_START).sw(c, u64::MAX);
    c.g[A0][0] = 0;
    Addr(out).off(SPCH_DECODE_OUT_LEN).sw(c, 0);
    c.g[T6][0] = 0;
    Addr(out).off(SPCH_DECODE_OUT_EXTRA_N).sw(c, 0);
    for i in 0..3 {
        Addr(out).off(SPCH_DECODE_OUT_EXTRA + i).sb(c, 0xff);
    }
    let count = Addr(c.g[T2][0] as u32).off(SPCH_DECODE_COUNT).lbu(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (count as i64)) as u64;
    c.g[T4][0] = 0;
    if c.g[V0][0] != 0 {
        spch_decode_sample_entry_found(c, out);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// The in-range part of [`spch_decode_sample_entry`].
fn spch_decode_sample_entry_found(c: &mut Cpu, out: u32) {
    c.cycles += 10;
    let bank = c.g[T2][0] as u32;
    c.g[V0][0] = Addr(bank).off(SPCH_DECODE_FLAGS).lbu(c);
    c.g[A3][0] = sx(bank.wrapping_add(SPCH_DECODE_ENTRIES));
    c.g[T7][0] = c.g[A3][0];
    c.g[T0][0] = 0;
    c.g[T1][0] = c.g[V0][0] & 0xf;
    c.g[T5][0] = sx((c.g[T1][0] as u32).wrapping_add(2));
    Addr(out).off(SPCH_DECODE_OUT_EXTRA_N).sw(c, c.g[T1][0]);
    c.mult(0, V0, A1, T5); // index * entry size
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if c.g[T1][0] != 0 {
        // Copy the extra bytes.
        c.cycles += 2;
        c.g[T3][0] = sx(out.wrapping_add(SPCH_DECODE_OUT_EXTRA));
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_b8a8);
            }
        }
    }
    // An alias: the start block comes from the entry it names.
    c.cycles += 4;
    c.g[V0][0] = Addr(c.g[A3][0] as u32).lbu(c) & 0x80;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(1)).lbu(c);
        c.mult(0, V0, A1, T5);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T7][0] as u32));
    }
    c.g[V1][0] = Addr(c.g[A3][0] as u32).lbu(c);
    // start = (hi << 8 | lo) * ((block + 1) << 8).
    c.cycles += 13;
    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[V0][0] = Addr(bank).off(SPCH_DECODE_BLOCK).lbu(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(1)).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A3][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.mult(0, A0, A0, A3);
    Addr(out).off(SPCH_DECODE_OUT_START).sw(c, c.g[A0][0]);
    c.g[T1][0] = Addr(bank).off(SPCH_DECODE_COUNT).lbu(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T1][0] as i64)) as u64;
    // The end: the next non-alias entry's start, else the bank's end.
    loop {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(bank).off(SPCH_DECODE_END).lhu(c);
            c.g[T6][0] = 1;
            c.mult(0, T4, V0, A3);
        } else {
            c.g[V0][0] = Addr(c.g[T0][0] as u32).lbu(c);
            c.cycles += 3;
            c.g[V0][0] &= 0x80;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            } else {
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lbu(c);
                c.cycles += 5;
                c.g[T6][0] = 1;
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(1)).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.mult(0, T4, T4, A3);
            }
        }
        c.cycles += 2;
        if c.g[T6][0] != 0 {
            break;
        }
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T1][0] as i64)) as u64;
        if event_due(c) {
            deliver_event_here(c, 0x003d_b918);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    Addr(out).off(SPCH_DECODE_OUT_LEN).sw(c, c.g[V0][0]);
    c.g[A0][0] = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, DATA};

    /// `RestoreNode_Save` through the real `WorldObj_Save` (and
    /// `SyncProcess_Save`), the stream's writers fakes (some unwind).
    #[test]
    fn restore_node_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(ee::s11::f_00351010__generated, restore_node_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_1010, 3);
            c.pc = 0x0035_1010;
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
        });
    }

    /// A speech bank at `DATA` with random entries (some aliases), an index
    /// in range, past the end, or -1, and the output record at `DATA+0x800`.
    fn spch_decode_sample_entry_setup(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let bank = DATA;
        let extra = if seed % 5 == 0 { 0 } else { r.next_u32() % 16 };
        let count = 1 + r.next_u32() % 12;
        c.bus.write8(bank + SPCH_DECODE_FLAGS, ((r.next_u32() & 0xf0) | extra) as u8);
        c.bus.write8(bank + SPCH_DECODE_COUNT, count as u8);
        c.bus.write8(bank + SPCH_DECODE_BLOCK, r.next_u32() as u8);
        c.bus.write16(bank + SPCH_DECODE_END, r.next_u32() as u16);
        let size = extra + 2;
        for i in 0..count + 2 {
            let e = bank + SPCH_DECODE_ENTRIES + i * size;
            for k in 0..size {
                c.bus.write8(e + k, r.next_u32() as u8);
            }
            if r.next_u32() % 3 == 0 {
                // An alias of another entry.
                c.bus.write8(e, (0x80 | r.next_u32()) as u8);
                c.bus.write8(e + 1, (r.next_u32() % (count + 1)) as u8);
            } else {
                c.bus.write8(e, (r.next_u32() & 0x7f) as u8);
            }
        }
        let index = match seed % 9 {
            0 => count as u64 + (r.next_u32() % 3) as u64,
            1 => u64::MAX,
            _ => (r.next_u32() % count) as u64,
        };
        c.g[A0][0] = sx(bank);
        c.g[A1][0] = index;
        c.g[A2][0] = sx(DATA + 0x800);
        c.pc = 0x003d_b838;
    }

    #[test]
    fn spch_decode_sample_entry_matches() {
        use crate::difftest::{scramble, Rng};
        check_lohi(ee::s14::f_003db838__generated, spch_decode_sample_entry, 300, &|c, seed| {
            let mut r = Rng(0x3db8_3800 ^ seed as u64);
            scramble(c, &mut r);
            spch_decode_sample_entry_setup(c, &mut r, seed);
        });
    }

    /// Events delivered (and unwinds requested) at both loops' back edges.
    #[test]
    fn spch_decode_sample_entry_events_match() {
        use crate::difftest::events::{check_events, schedule};
        use crate::difftest::{scramble, Rng};
        check_events(ee::s14::f_003db838__generated, spch_decode_sample_entry, 300, &|c, seed| {
            let mut r = Rng(0x3db8_3801 ^ seed as u64);
            scramble(c, &mut r);
            spch_decode_sample_entry_setup(c, &mut r, seed);
            schedule(c, &mut r, seed, 0x003d_b838)
        });
    }
}
