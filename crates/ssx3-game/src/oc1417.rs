//! Staging module oc1417 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1417`.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `MD5_HashString` (`0x00317710`).
    pub(crate) const MD5_HASH_STRING: Callee = Callee {
        addr: 0x0031_7710,
        f: ee::s10::f_00317710,
    };
}

/// `int PS2ExecMan_SetCallback(PS2ExecMan *man, ...)` (0x317478): store the
/// callback arguments at `man`, then tail-call the slot handler through the
/// object's slot table.
pub fn ps2_exec_man_set_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, t, 0x0031_749c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *MD5_HashStringDiscard(const char *s, ...)` (0x317798): run the
/// MD5 string hash over `s` into stack scratch, returning `s` (the digest
/// is discarded; the hash overwrites the string's first 16 bytes).
pub fn md5_hash_string_discard(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        MD5_HASH_STRING.call(c, 0x0031_77b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, 16, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_ForwardSlot05(PS2ExecMan *man)` (0x3209c0): tail-call the
/// slot-5 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot05(c: &mut Cpu) {
    forward_slot(c, 0x28, 0x2c, 0x0032_09e0);
}

/// `int PS2ExecMan_ForwardSlot04(PS2ExecMan *man)` (0x3209f0): tail-call the
/// slot-4 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot04(c: &mut Cpu) {
    forward_slot(c, 0x20, 0x24, 0x0032_0a10);
}

/// `int PS2ExecMan_ForwardSlot11(PS2ExecMan *man)` (0x320a60): tail-call the
/// slot-11 handler through the object's slot table.
pub fn ps2_exec_man_forward_slot11(c: &mut Cpu) {
    forward_slot(c, 0x58, 0x5c, 0x0032_0a80);
}

/// Shared body of the `ForwardSlot` wrappers: load the handler and the
/// signed offset from the object's slot table and tail-call it.
fn forward_slot(c: &mut Cpu, half: u32, word: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(half));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(word));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0031_7478,
        ps2_exec_man_set_callback,
        "PS2ExecMan_SetCallback",
    ),
    (
        0x0031_7798,
        md5_hash_string_discard,
        "MD5_HashStringDiscard",
    ),
    (
        0x0032_09c0,
        ps2_exec_man_forward_slot05,
        "PS2ExecMan_ForwardSlot05",
    ),
    (
        0x0032_09f0,
        ps2_exec_man_forward_slot04,
        "PS2ExecMan_ForwardSlot04",
    ),
    (
        0x0032_0a60,
        ps2_exec_man_forward_slot11,
        "PS2ExecMan_ForwardSlot11",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{require_paths, DATA};

    /// The `miss` hook with a fixed 4-cycle cost: the fake's hash-based cost
    /// would otherwise make single-path wrappers' totals seed-varying.
    fn norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// `PS2ExecMan_SetCallback`: the arguments land at the object before
    /// the fake slot handler runs with the adjusted object.
    #[test]
    fn ps2_exec_man_set_callback_matches() {
        check(
            ee::s10::f_00317478__generated,
            ps2_exec_man_set_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7478, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x100;
                const TAB: u32 = DATA + 0x200;
                c.bus.write32(OBJ + 8, TAB);
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x400) as u16,
                    _ => (0x10000 - r.next_u32() % 0x400) as u16,
                };
                c.bus.write16(TAB + 0x68, off);
                c.bus.write32(TAB + 0x6c, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = 0xc0de_0000 + seed as u64;
                c.g[A2][0] = 0xf00d_0000 + seed as u64 * 3;
                c.pc = 0x0031_7478;
            },
        );
        require_paths(ee::s10::f_00317478__generated, 300, &[16], &|c, seed| {
            let mut r = machine(c, seed, 0x0031_7478, 0);
            c.hooks.miss = norm_miss;
            const OBJ: u32 = DATA + 0x100;
            const TAB: u32 = DATA + 0x200;
            c.bus.write32(OBJ + 8, TAB);
            let off = match seed % 3 {
                0 => 0,
                1 => (r.next_u32() % 0x400) as u16,
                _ => (0x10000 - r.next_u32() % 0x400) as u16,
            };
            c.bus.write16(TAB + 0x68, off);
            c.bus.write32(TAB + 0x6c, FAKE);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = 0xc0de_0000 + seed as u64;
            c.g[A2][0] = 0xf00d_0000 + seed as u64 * 3;
            c.pc = 0x0031_7478;
        });
    }

    /// `MD5_HashStringDiscard`: strings across the padding boundary hash
    /// into the string's first 16 bytes on both sides, returning the string.
    #[test]
    fn md5_hash_string_discard_matches() {
        check(
            ee::s10::f_00317798__generated,
            md5_hash_string_discard,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7798, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let s = DATA + 0x100;
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                for i in 0..0x10u32 {
                    c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
                }
                c.g[A0][0] = s as u64;
                c.g[A1][0] = s as u64;
                c.g[A2][0] = 0xdddd_0000 + seed as u64;
                c.pc = 0x0031_7798;
            },
        );
    }

    /// The three remaining `ForwardSlot` wrappers: the slot table yields
    /// the fake handler with a seed-steered offset. The tables are
    /// prefilled with garbage, so a wrong offset reads a visibly wrong
    /// target into the call log instead of the fake.
    #[test]
    fn ps2_exec_man_forward_slots_match() {
        for (gen, hand, addr, half, word) in [
            (
                ee::s10::f_003209c0__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot05 as fn(&mut Cpu),
                0x0032_09c0,
                0x28,
                0x2c,
            ),
            (
                ee::s10::f_003209f0__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot04 as fn(&mut Cpu),
                0x0032_09f0,
                0x20,
                0x24,
            ),
            (
                ee::s10::f_00320a60__generated as fn(&mut Cpu),
                ps2_exec_man_forward_slot11 as fn(&mut Cpu),
                0x0032_0a60,
                0x58,
                0x5c,
            ),
        ] {
            check(gen, hand, 100, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                for i in 0..0x20u32 {
                    c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                }
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + half, off);
                c.bus.write32(TAB2 + word, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = addr;
            });
            require_paths(gen, 100, &[15], &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                for i in 0..0x20u32 {
                    c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                }
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + half, off);
                c.bus.write32(TAB2 + word, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = addr;
            });
        }
    }
}
