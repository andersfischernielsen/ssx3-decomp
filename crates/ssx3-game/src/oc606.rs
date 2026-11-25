//! Staging module oc606 (wave 233, list s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc606`.

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
    &[(0x003a_6948, world_sdb_has_sections, "World_SdbHasSections")];

/// `int World_SdbHasSections(const char* path, int n, sSdbName* names)`
/// (0x3a6948, `isValidMap` in the reference): builds `path` + `str_bar_e` +
/// `str_sdb` (the map's `.sdb` file), loads it with `FILE_loadz` and checks
/// its magic (0x3800, a half-word at +0). For each of the `n` names
/// (`{ char* sectionName; int bFound; }`, 8 bytes) it upper-cases a copy and
/// scans the file's 0x58-byte entries (count at +8, entries from +0x50) for
/// an equal string, setting `bFound`. Returns 1 if any name matched. The
/// file buffer is freed at the end (`EA_MemFree`, when it loaded).
pub fn world_sdb_has_sections(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S7, 0x40);
        frame.save(c, S0, 0xb0);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S2, 0x90);
        c.g[A1][0] = c.g[A0][0];
        c.g[S2][0] = sx(frame.at(c, 0x20));
        frame.save(c, S1, 0xa0);
        frame.save(c, S5, 0x60);
        c.g[S0][0] = 0x4a0000;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S6, 0x50);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x47c0));
        frame.save_ra(c, 0x30);

        // cBXString str(path); prefixed = str_bar_e + str; full = prefixed + str_sdb
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_82d8, 0x003a_6994)?;
        c.g[S4][0] = 0;
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_86d0, 0x003a_69a8)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x47c8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0031_8630, 0x003a_69bc)?;
        c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
        c.g[A1][0] = 0x0300_0000;
        call_ptr(c, 0x003e_1908, 0x003a_69c8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_69d8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_69e4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_69f0)?;

        // ret = data && data->magic == 0x3800
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S3][0] as u32).lhu(c);
            c.g[V0][0] = c.g[V0][0] ^ 0x3800;
            c.g[S4][0] = (c.g[V0][0] < 0x1) as u64;
        }

        c.cycles += 2;
        let valid = c.g[S4][0] != 0;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x50));
        if valid {
            c.cycles += 4;
            c.g[S2][0] = 0;
            let entries = c.g[V0][0];
            Addr(c.g[S3][0] as u32).off(0x14).sw(c, entries);
            let names_left = (c.g[S7][0] as i64) > 0;
            c.g[S4][0] = 0;
            if names_left {
                c.cycles += 3;
                c.g[S6][0] = 0x58;
                c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                // One pass per name: i = s2, names[i] = s5 + i * 8.
                'names: loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[S1][0] = 0;
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    call_ptr(c, 0x0031_82d8, 0x003a_6a40)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_8d28, 0x003a_6a48)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));

                    // Scan the entries for the upper-cased name (j = s1).
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                        c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                        let in_range = c.g[V0][0] != 0;
                        c.mult(0, V0, S1, S6);
                        if !in_range {
                            break;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = Addr(c.g[S3][0] as u32).off(0x14).lw(c);
                        c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0041_65a8, 0x003a_6a70)?;
                        if c.g[V0][0] != 0 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                            if event_due(c) {
                                deliver_event_here(c, 0x003a_6a50);
                            }
                            continue;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[S4][0] = 1;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
                        break;
                    }

                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x003a_6a90)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_6a28);
                        }
                        continue 'names;
                    }
                    break 'names;
                }
            }
        }

        // if (data) cMemMan_free(data); return ret
        c.cycles += 2;
        let has_file = c.g[S3][0] != 0;
        c.g[V0][0] = c.g[S4][0];
        if has_file {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7e98, 0x003a_6aac)?;
            c.g[V0][0] = c.g[S4][0];
        }

        c.cycles += 11;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine};
    use crate::difftest::{Rng, DATA, RA as FIXTURE_RA, SP as STACK};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture: the gate word (+4: what FILE_loadz returns, 0 = no file).
    const SDB_CTRL: u32 = DATA + 0x2000;
    const SDB_MAGIC: u32 = 0x5db0_0d1e;
    /// The `.sdb` image: magic (u16) at +0, count at +8, entries from +0x50.
    const SDB_FILE: u32 = DATA + 0x3000;
    /// `{ char* sectionName; int bFound; }` records, 8 bytes each.
    const SDB_NAMES: u32 = DATA + 0x5000;
    /// The name strings, 16 bytes apart.
    const SDB_STRS: u32 = DATA + 0x5100;
    const SDB_PATH: u32 = DATA + 0x7000;

    static OC606_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_PREFIX: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_FULL: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_UPPER: AtomicUsize = AtomicUsize::new(0);
    static OC606_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// cBXString copy constructor (0x3182d8): the string is the source
    /// pointer, so the name compares against the entries as written.
    fn oc606_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_CTOR, ee::s10::f_003182d8__generated);
        }
        let ra = c.g[RA][0] as u32;
        let (obj, src) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        fake(c, 0x0031_82d8, ra);
        c.bus.write32(obj, src);
    }

    /// Prefix builder (0x3186d0): the result string is its source.
    fn oc606_fake_prefix(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_PREFIX, ee::s10::f_003186d0__generated);
        }
        let ra = c.g[RA][0] as u32;
        let (obj, src) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        fake(c, 0x0031_86d0, ra);
        c.bus.write32(obj, src);
    }

    /// Full-name builder (0x318630): the result is the names area.
    fn oc606_fake_full(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_FULL, ee::s10::f_00318630__generated);
        }
        let ra = c.g[RA][0] as u32;
        let obj = c.g[A0][0] as u32;
        fake(c, 0x0031_8630, ra);
        c.bus.write32(obj, SDB_STRS);
    }

    /// `FILE_loadz` (0x3e1908): the fixture picks NULL or the image.
    fn oc606_fake_load(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_LOAD, ee::s14::f_003e1908__generated);
        }
        let ra = c.g[RA][0] as u32;
        let sel = c.bus.read32(SDB_CTRL + 4);
        fake(c, 0x003e_1908, ra);
        c.g[V0][0] = if sel == 0 { 0 } else { SDB_FILE as u64 };
    }

    /// cBXString destructor (0x318260): nothing to free in the fixture.
    fn oc606_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_DTOR, ee::s10::f_00318260__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0031_8260, ra);
    }

    /// Upper-casing (0x318d28): the fixture's names are already upper case.
    fn oc606_fake_upper(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_UPPER, ee::s10::f_00318d28__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0031_8d28, ra);
    }

    /// `EA_MemFree` (0x317e98).
    fn oc606_fake_free(c: &mut Cpu) {
        if c.bus.read32(SDB_CTRL) != SDB_MAGIC {
            return fall_back(c, &OC606_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0031_7e98, ra);
    }

    fn oc606_install_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc606_fake_ctor,
            &OC606_PREV_CTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc606_fake_prefix,
            &OC606_PREV_PREFIX,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc606_fake_full,
            &OC606_PREV_FULL,
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1908),
            oc606_fake_load,
            &OC606_PREV_LOAD,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc606_fake_dtor,
            &OC606_PREV_DTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318D28),
            oc606_fake_upper,
            &OC606_PREV_UPPER,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc606_fake_free,
            &OC606_PREV_FREE,
        );
    }

    fn oc606_put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Every branch: no file (NULL), a bad magic, a good file with 0..3
    /// entries, 0..3 names (matching, missing, and early-found ones).
    #[test]
    fn world_sdb_has_sections_matches() {
        oc606_install_fakes();
        covered(
            ee::s13::f_003a6948__generated,
            world_sdb_has_sections,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3a_6948, 8);
                c.g[RA][0] = FIXTURE_RA as u64;
                c.g[SP][0] = STACK as u64;
                c.g[A0][0] = SDB_PATH as u64;
                let n = r.next_u32() % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = SDB_NAMES as u64;
                c.pc = 0x003a_6948;

                c.bus.write32(SDB_CTRL, SDB_MAGIC);
                let loads = if seed % 5 == 0 { 0 } else { 1 };
                c.bus.write32(SDB_CTRL + 4, loads);

                let magic: u16 = if seed % 4 == 1 { 0x1234 } else { 0x3800 };
                c.bus.write16(SDB_FILE, magic);
                let count = r.next_u32() % 4;
                c.bus.write32(SDB_FILE + 8, count);
                let entries: [&[u8]; 4] = [b"SEC0", b"SEC1", b"SEC2", b"SEC3"];
                for (j, e) in entries.iter().enumerate() {
                    oc606_put_cstr(c, SDB_FILE + 0x50 + 0x58 * j as u32, e);
                }

                let texts: [&[u8]; 5] = [b"SEC0", b"SEC1", b"SEC2", b"SEC3", b"NOPE"];
                for i in 0..n {
                    let s = SDB_STRS + 16 * i;
                    let pick = (r.next_u32() % 5) as usize;
                    oc606_put_cstr(c, s, texts[pick]);
                    c.bus.write32(SDB_NAMES + 8 * i, s);
                    c.bus.write32(SDB_NAMES + 8 * i + 4, r.next_u32());
                }
            },
        );
    }
}
