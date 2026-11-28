//! Staging module oc1496 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1496`.

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
    (0x0040_a000, fixup_copy_bytes, "func_0040A000"),
    (0x002b_e1f8, tw_ingame_lighting_set_defaults, "tWPInGameData_Lighting_setDefaults"),
    (0x002b_b5f0, wp_file_relocate, "cWorldPainterSection_resolve"),
    (0x0019_59a8, crate::game::util::locale::fe_locale_load_table, "FELocale_LoadTable"),
];

/// `void func_0040A000(void* buf)` (0x40a000): `buf` is taken through the
/// uncached alias (`| 0x20000000`). Copies `lw(+0)` bytes from `+0x10` to
/// the address at `+8`, then `lw(+4)` bytes from `+0x50` to the address at
/// `+0xc`; each copy runs only for a positive count. Returns `lw(+4)`.
pub fn fixup_copy_bytes(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x2000_0000;
    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(base).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A2][0] = Addr(base.wrapping_add(8)).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[A1][0] = 0;
        if positive {
            c.cycles += 2;
            c.g[A3][0] = sx(base.wrapping_add(0x10));
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let byte = c.g[V1][0];
                Addr(c.g[A2][0] as u32).sb(c, byte);
                c.g[V0][0] = Addr(base).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_a028);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(base.wrapping_add(4)).lw(c);
    } else {
        c.g[V0][0] = Addr(base.wrapping_add(4)).lw(c);
    }
    fixup_copy_bytes_tail(c, base);
    c.cycles += 2;
    ret(c);
}

/// The second copy of `fixup_copy_bytes`: entered with `V0 = lw(base+4)`.
fn fixup_copy_bytes_tail(c: &mut Cpu, base: u32) {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A2][0] = Addr(base.wrapping_add(0xc)).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[A1][0] = 0;
        if positive {
            c.cycles += 2;
            c.g[A3][0] = sx(base.wrapping_add(0x50));
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let byte = c.g[V1][0];
                Addr(c.g[A2][0] as u32).sb(c, byte);
                c.g[V0][0] = Addr(base.wrapping_add(4)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_a068);
                }
            }
        }
    }
}

/// `void tWPInGameData::Lighting::setDefaults(Lighting* l)` (0x2be1f8):
/// writes the default lighting block at `l`: four unaligned 8-byte copies of
/// a constant pair from the GP area (the last one after the float constants),
/// the floats 1.0 at `+0x48` and 0.75 at `+0x50`, and a zero word at `+0`.
pub fn tw_ingame_lighting_set_defaults(c: &mut Cpu) {
    c.cycles += 24;
    let pair_hi = (c.g[GP][0] as u32).wrapping_add(0x22e7);
    let pair_lo = (c.g[GP][0] as u32).wrapping_add(0x22e0);
    let l = c.g[A0][0] as u32;
    for (hi, lo) in [(0x0fu32, 0x08u32), (0x1f, 0x18), (0x2f, 0x28)] {
        c.ldl(2, pair_hi);
        c.ldr(2, pair_lo);
        c.sdl(2, l.wrapping_add(hi));
        c.sdr(2, l.wrapping_add(lo));
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x3f40_0000;
    c.mtc1(1, 0);
    c.ldl(2, pair_hi);
    c.ldr(2, pair_lo);
    c.sdl(2, l.wrapping_add(0x3f));
    c.sdr(2, l.wrapping_add(0x38));
    c.swc1(1, l.wrapping_add(0x48));
    c.swc1(0, l.wrapping_add(0x50));
    Addr(l).sw(c, 0);
    ret(c);
}

/// `void cWorldPainterSection::resolve(self)` (0x2bb5f0), the `WPFile_Relocate`
/// name: when `self->+8` is non-null and its first word is 0x10 (or the
/// `+2` halfword of `self` is non-zero, else it calls 0x317e98 first), loops
/// `count` (`*(p+4)`) entries of `p+8`: each -1 entry is cleared, others are
/// rebased to `p + entry` and passed to `WPData_Relocate`. Ends by storing 3
/// as a halfword at `self`.
pub fn wp_file_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        c.g[V0][0] = 0x10;
        'end: {
            if c.g[A0][0] == 0 {
                break 'end;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(2)).lh(c);
                if c.g[V0][0] != 0 {
                    Addr(r32(c, S1).wrapping_add(8)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x002b_b638)?;
                    Addr(r32(c, S1).wrapping_add(8)).sw(c, 0);
                }
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            }
            c.cycles += 2;
            let null_entry = c.g[V1][0] == 0;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
            if null_entry {
                break 'end;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if none {
                break 'end;
            }
            c.cycles += 1;
            c.g[S3][0] = 0xffff_ffff_ffff_ffff;
            loop {
                'after_relocate: {
                    'entry_done: {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[V1][0] != c.g[S3][0] {
                            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                            break 'entry_done;
                        }
                        c.cycles += 2;
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'after_relocate;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.g[V0][0];
                    let rebased = c.g[V0][0];
                    Addr(r32(c, S0)).sw(c, rebased);
                    call_ptr(c, 0x002b_af08, 0x002b_b67c)?;
                }
                c.cycles += 6;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break 'end;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_b658);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 3;
        let three = c.g[V0][0];
        Addr(r32(c, S1)).sh(c, three);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine, machine_rt};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn fixup_copy_bytes_matches() {
        check(
            ee::s15::f_0040a000__generated,
            fixup_copy_bytes,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0040_a000, 0);
                let p = DATA + 0x100;
                c.g[A0][0] = sx(p);
                let count1 = [0u32, 1, 4, 0xffff_ffff][seed as usize % 4];
                let count2 = [2u32, 0, 3, 0xffff_fffe][(seed / 4) as usize % 4];
                c.bus.write32(p, count1);
                c.bus.write32(p + 4, count2);
                c.bus.write32(p + 8, DATA + 0x2000);
                c.bus.write32(p + 0xc, DATA + 0x3000);
                fill(c, &mut r, p + 0x10, 0x20);
                fill(c, &mut r, p + 0x50, 0x20);
            },
        );
    }

    #[test]
    fn tw_ingame_lighting_set_defaults_matches() {
        check(
            ee::s08::f_002be1f8__generated,
            tw_ingame_lighting_set_defaults,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x002b_e1f8, 0);
                c.g[A0][0] = sx(DATA + 0x100 + 0x40 * (seed % 3));
            },
        );
    }

    /// Gate word in the machine's RAM: the test fakes act only when it is set.
    const OC1496_GATE: u32 = DATA + 0x2_0000;
    const OC1496_MAGIC: u32 = 0x1496_1496;
    static OC1496_PREV_WP_DATA_RELOCATE: AtomicUsize = AtomicUsize::new(0);
    static OC1496_PREV_317E98: AtomicUsize = AtomicUsize::new(0);

    /// `WPData_Relocate` replaced by a return (the relocation itself is not
    /// under test here).
    fn oc1496_fake_wp_data_relocate(c: &mut Cpu) {
        if c.bus.read32(OC1496_GATE) != OC1496_MAGIC {
            return fall_back(
                c,
                &OC1496_PREV_WP_DATA_RELOCATE,
                ee::s08::f_002baf08__generated,
            );
        }
        ret(c);
    }

    /// The `0x317e98` callee replaced by a return.
    fn oc1496_fake_317e98(c: &mut Cpu) {
        if c.bus.read32(OC1496_GATE) != OC1496_MAGIC {
            return fall_back(c, &OC1496_PREV_317E98, ee::s10::f_00317e98__generated);
        }
        ret(c);
    }

    #[test]
    fn wp_file_relocate_matches() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BAF08),
            oc1496_fake_wp_data_relocate,
            &OC1496_PREV_WP_DATA_RELOCATE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1496_fake_317e98,
            &OC1496_PREV_317E98,
        );
        check(
            ee::s08::f_002bb5f0__generated,
            wp_file_relocate,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x002b_b5f0, 0);
                c.bus.write32(OC1496_GATE, OC1496_MAGIC);
                let s = DATA + 0x1000;
                let obj = DATA + 0x1100;
                c.g[A0][0] = sx(s);
                // seed % 6: 0 = null pointer; 1, 4, 5 = header 0x10; 2 = header
                // other, halfword set; 3 = header other, halfword zero (calls 0x317e98).
                let case = seed % 6;
                let kind: u32 = if case == 2 || case == 3 { 0x11 } else { 0x10 };
                c.bus.write32(s, if case == 2 { 0x0001_0000 } else { 0 });
                c.bus.write32(s + 8, if case == 0 { 0 } else { obj });
                c.bus.write32(obj, kind);
                let count: u32 = (seed / 6) % 4;
                c.bus.write32(obj + 4, count);
                for i in 0u32..4 {
                    let e = if (seed / 6 + i) % 2 == 0 {
                        0xffff_ffff
                    } else {
                        0x40 * (i + 1)
                    };
                    c.bus.write32(obj + 8 + 4 * i, e);
                }
            },
        );
    }

    #[test]
    fn fe_locale_load_table_matches() {
        check(
            ee::s02::f_001959a8__generated,
            crate::game::util::locale::fe_locale_load_table,
            80,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x0019_59a8, 0);
                let this = DATA + 0x100;
                // 0x80 is a negative language index (the early return).
                let lang: u32 = [0u32, 1, 2, 3, 0x80][seed as usize % 5];
                let slot = this + 8 + 4 * lang;
                c.bus.write32(slot, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = lang as u64;
            },
        );
    }
}
