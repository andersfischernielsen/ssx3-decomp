//! Staging module oc639 (wave g19.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc639`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0017_a830,
    debug_menu_add_world_if_missing,
    "DebugMenu_AddWorldIfMissing",
)];

/// `DebugMenu_AddWorldIfMissing` (0x17a830): `int f(self, name, tag)`. The
/// reference calls it `cTrackFileMan_checkLocation`. Returns 0 when a row of
/// the location table already has file `name` and big file `tag` (rows are
/// 0x8c bytes: `File` at +4, `BigFile` at +0x44). Otherwise it opens the
/// world's big file (the BigFile ctor on `worlds/<tag>`), asks
/// `World_SdbHasSections` whether a section `name` exists and returns the
/// `bFound` flag it sets (0 when that gives no map).
pub fn debug_menu_add_world_if_missing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = 0;
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A2][0];
        let this = c.g[S3][0] as u32;
        c.g[V0][0] = Addr(this).lw(c);
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = c.g[A1][0];

        let mut listed = false;
        if !no_rows {
            c.cycles += 2;
            c.g[S1][0] = sx(this.wrapping_add(0x44));
            c.g[S0][0] = sx(this.wrapping_add(0x4));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_aa88, 0x0017_a87c)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(this).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_aa88, 0x0017_a890)?;
                    if c.g[V0][0] == 0 {
                        // Both names match: the location is listed already.
                        c.cycles += 2;
                        listed = true;
                        break;
                    }
                    c.g[V0][0] = Addr(this).lw(c);
                }
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_a870);
                }
            }
        }

        if listed {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 40;
            c.g[S1][0] = sx(frame.at(c, 0x10));
            c.g[S0][0] = sx(0x0046_0000u32.wrapping_sub(0x31e0));
            // workStr = BXString(tag) ; base = "data/worlds" + workStr
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_82d8, 0x0017_a8cc)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0031_86d0, 0x0017_a8dc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a8e8)?;
            // workStr = base + "big" ; BigFile(pbf) opened on workStr.
            c.g[S2][0] = sx(frame.at(c, 0x20));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = sx(0x004a_1298);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_8630, 0x0017_a900)?;
            let work_str = Addr(frame.at(c, 0x10)).lw(c);
            c.g[A1][0] = work_str;
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_68b0, 0x0017_a910)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a91c)?;
            // nm.sectionName = name; bFound = isValidMap(base, 1, &nm)
            let base_str = Addr(sp(c)).lw(c);
            c.g[A0][0] = base_str;
            c.g[A2][0] = sx(frame.at(c, 0x30));
            let name = c.g[S5][0];
            Addr(frame.at(c, 0x30)).sw(c, name);
            c.g[A1][0] = 1;
            call_ptr(c, 0x003a_6948, 0x0017_a930)?;
            let found = Addr(frame.at(c, 0x34)).lw(c);
            c.g[S0][0] = found;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 2;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = 0;
            }
            call_ptr(c, 0x0031_68e8, 0x0017_a944)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a950)?;
            c.g[V0][0] = c.g[S0][0];
        }

        c.cycles += 9;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes (`DATA+0x1f000`, clear of the fixture): the
    /// magic enables them, +4 is the `bFound` the map reports, +8 makes the
    /// map come back missing (`isValidMap` returns 0).
    const OC639_CTRL: u32 = DATA + 0x1_f000;
    const OC639_MAGIC: u32 = 0x6f63_3633;

    /// A gated fake for a callee the body calls through the table: with the
    /// gate on it logs the call and returns (`fakes::fake`), with it off it
    /// runs what the slot held before the hook.
    macro_rules! oc639_gated_fake {
        ($fake:ident, $prev:ident, $target:literal, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC639_CTRL) != OC639_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc639_gated_fake!(
        oc639_fake_3182d8,
        OC639_PREV_3182D8,
        0x0031_82d8,
        ee::s10::f_003182d8__generated
    );
    oc639_gated_fake!(
        oc639_fake_3186d0,
        OC639_PREV_3186D0,
        0x0031_86d0,
        ee::s10::f_003186d0__generated
    );
    oc639_gated_fake!(
        oc639_fake_318260,
        OC639_PREV_318260,
        0x0031_8260,
        ee::s10::f_00318260__generated
    );
    oc639_gated_fake!(
        oc639_fake_318630,
        OC639_PREV_318630,
        0x0031_8630,
        ee::s10::f_00318630__generated
    );
    oc639_gated_fake!(
        oc639_fake_3168b0,
        OC639_PREV_3168B0,
        0x0031_68b0,
        ee::s10::f_003168b0__generated
    );
    oc639_gated_fake!(
        oc639_fake_3168e8,
        OC639_PREV_3168E8,
        0x0031_68e8,
        ee::s10::f_003168e8__generated
    );

    static OC639_PREV_3A6948: AtomicUsize = AtomicUsize::new(0);

    /// `World_SdbHasSections(name, flags, nm)`: with the gate on, it sets the
    /// `bFound` word of `nm` (a2+4) from the fixture and may come back 0.
    fn oc639_fake_3a6948(c: &mut Cpu) {
        if c.bus.read32(OC639_CTRL) != OC639_MAGIC {
            return fakes::fall_back(c, &OC639_PREV_3A6948, ee::s13::f_003a6948__generated);
        }
        let nm = c.g[A2][0] as u32;
        let found = c.bus.read32(OC639_CTRL + 4);
        c.bus.write32(nm + 4, found);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003a_6948, ra);
        if c.pc == ra && c.bus.read32(OC639_CTRL + 8) != 0 {
            c.g[V0][0] = 0;
        }
    }

    fn oc639_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc639_fake_3182d8,
            &OC639_PREV_3182D8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc639_fake_3186d0,
            &OC639_PREV_3186D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc639_fake_318260,
            &OC639_PREV_318260,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc639_fake_318630,
            &OC639_PREV_318630,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003168B0),
            oc639_fake_3168b0,
            &OC639_PREV_3168B0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003168E8),
            oc639_fake_3168e8,
            &OC639_PREV_3168E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6948),
            oc639_fake_3a6948,
            &OC639_PREV_3A6948,
        );
    }

    /// A NUL-terminated string in RAM.
    fn put_cstr(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// `DebugMenu_AddWorldIfMissing` against the generated body: zero to
    /// three location rows whose names match the wanted file and big file
    /// in different ways (case-insensitively for some), so the scan lists
    /// the location on some seeds and runs off the end on the others. The
    /// big file, the section lookup (bFound 0/1, and no map on every fifth
    /// seed) and every callee run as fakes; strcasecmp runs for real.
    #[test]
    fn debug_menu_add_world_if_missing_matches() {
        const SELF: u32 = DATA;
        const NAME: u32 = DATA + 0x2000;
        const TAG: u32 = DATA + 0x2100;
        oc639_install_fakes();
        covered(
            ee::s02::f_0017a830__generated,
            debug_menu_add_world_if_missing,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17a830, 9);
                c.bus.write32(OC639_CTRL, OC639_MAGIC);
                c.bus.write32(OC639_CTRL + 4, r.next_u32() & 1);
                c.bus.write32(OC639_CTRL + 8, (seed % 5 == 0) as u32);
                let rows = seed % 4;
                c.bus.write32(SELF, rows);
                put_cstr(c, NAME, "snow");
                put_cstr(c, TAG, "big");
                for i in 0..rows {
                    let (file, big) = match (seed / 3 + i) % 4 {
                        0 => ("SNOW", "big"),
                        1 => ("snow", "Other"),
                        2 => ("other", "BIG"),
                        _ => ("other", "other"),
                    };
                    put_cstr(c, SELF + 4 + 0x8c * i, file);
                    put_cstr(c, SELF + 0x44 + 0x8c * i, big);
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(NAME);
                c.g[A2][0] = sx(TAG);
                c.pc = 0x0017_a830;
            },
        );
    }
}
