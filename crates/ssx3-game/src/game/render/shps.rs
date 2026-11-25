//! `src/render/shps.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_3d00, shps_get_entry_name, "SHPS_GetEntryName"),
    (0x003b_3900, shps_find_entry_by_name, "SHPS_FindEntryByName"),
    (0x003b_39b8, shps_find, "SHPS_Find"),
];

/// `char* SHPS_GetEntryName(SHPSheet* sheet, int index, char* out)`
/// (0x3b3d00): copy the entry's name pointer (or "" when out of range) into
/// `out` (an unaligned 4-byte sequence via `swr`/`swl`).
pub fn shps_get_entry_name(c: &mut Cpu) {
    c.cycles += 5;
    let sheet = abi::arg(c, 0);
    let out = c.g[A2][0];
    c.g[T1][0] = Addr(sheet).off(0x8).lw(c);
    c.g[T3][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[AT][0] = (c.g[A1][0] < c.g[T1][0]) as u64;
    let out_of_range = c.g[AT][0] == 0;
    c.g[T3][0] = sx(sheet.wrapping_add(c.g[T3][0] as u32));
    if out_of_range {
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.swr(V0, out as u32);
        c.swl(V0, (out as u32).wrapping_add(3));
        c.g[V0][0] = c.g[A2][0];
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x10).lw(c);
    c.swr(V0, out as u32);
    c.swl(V0, (out as u32).wrapping_add(3));
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

pub(crate) mod callee {

    pub(crate) use crate::game::lib::libc::callee::STRCMP;

    pub(crate) use crate::game::world::wscriptcache::callee::SHPS_FIND_ENTRY_BY_NAME;
    pub(crate) use crate::game::world::wscriptcache::callee::SHPS_GET_ENTRY_NAME;
    pub(crate) use crate::game::world::wscriptcache::callee::SHP_FIND_LONG_NAME;
}

/// A shape sheet: the entry count at +8, then 8-byte entries from +0x10
/// (a 4-character name, then the image's offset from the sheet).
pub(crate) const SHPS_COUNT: u32 = 0x8;

pub(crate) const SHPS_ENTRIES: u32 = 0x10;

pub(crate) const SHPS_ENTRY_OFFSET: u32 = 0x4;

pub(crate) const SHPS_ENTRY_SIZE: u32 = 0x8;

/// The loop head of `SHPS_FindEntryByName`.
pub(crate) const FIND_ENTRY_LOOP: u32 = 0x003b_3938;

/// `SHPHeader* SHPS_FindEntryByName(SHPSheet* sheet, const char* name)`
/// (0x3b3900): the first entry whose name equals `name`: its long name
/// (`SHP_FindLongName`) if the image has one, else its 4-character entry
/// name (copied NUL-terminated to the stack). Null when none matches.
pub fn shps_find_entry_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        // s2 the sheet, s1 the index, s3 the count, s4 the name, s0 the
        // entry's offset field.
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x10);
        frame.save(c, S3, 0x30);
        let sheet = r32(c, S2);
        c.g[S3][0] = Addr(sheet).off(SHPS_COUNT).lw(c);
        let empty = (c.g[S3][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        'exit: {
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(sheet.wrapping_add(SHPS_ENTRIES + SHPS_ENTRY_OFFSET));
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    callee::SHP_FIND_LONG_NAME.call(c, 0x003b_3944)?;
                    let long = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[V0][0];
                    if !long {
                        c.cycles += 7;
                        let buf = frame.at(c, 0);
                        Addr(buf).off(4).sb(c, 0);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        callee::SHPS_GET_ENTRY_NAME.call(c, 0x003b_3960)?;
                        c.g[V0][0] = c.g[SP][0];
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    callee::STRCMP.call(c, 0x003b_3970)?;
                    let differs = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    if !differs {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                        break 'exit;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(SHPS_ENTRY_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, FIND_ENTRY_LOOP);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SHPHeader* SHPS_Find(SHPSheet* sheet, const char* name)` (0x3b39b8):
/// `SHPS_FindEntryByName`.
pub fn shps_find(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SHPS_FIND_ENTRY_BY_NAME.call(c, 0x003b_39c8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use ssx3_ee as ee;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn shps_get_entry_name_matches() {
        check(
            ssx3_ee::s13::f_003b3d00,
            shps_get_entry_name,
            200,
            &|c, seed| {
                let mut r = Rng(0xf006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                c.bus.write32(DATA + 8, count);
                c.bus.write32(DATA + index * 8 + 0x10, r.next_u32());
                let out = DATA + 0x300 + (seed % 4);
                c.g[A2][0] = out as u64;
                c.bus.write32(out & !3, r.next_u32());
            },
        );
    }

    use crate::difftest::check_paths;

    pub(crate) use crate::game::world::wscriptcache::tests::shp_unwind_seed;
    pub(crate) use crate::game::world::wscriptcache::tests::shp_unwind_setup;

    pub(crate) use crate::game::world::wscriptcache::tests::shps_find_call;

    /// Totals with the strcmp fake: no entries 21; one "p" entry matching
    /// 54, not 55; one entry without a long name matching 78, not 79.
    #[test]
    fn shps_find_entry_by_name_matches() {
        check_paths(
            ee::s13::f_003b3900__generated,
            shps_find_entry_by_name,
            302,
            &[21, 54, 55, 78, 79],
            &|c, seed| {
                // Seeds 300, 301: one entry without a long name, unwinding
                // at SHPS_GetEntryName, then at strcmp.
                let (seed, unwind) = shp_unwind_seed(seed, 300, &[5]);
                shps_find_call(c, seed, 0x003b_3900);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// `SHPS_FindEntryByName` plus 7: no entries 28; one "p" entry 61 / 62;
    /// one without a long name 85 / 86.
    #[test]
    fn shps_find_matches() {
        check_paths(
            ee::s13::f_003b39b8__generated,
            shps_find,
            302,
            &[28, 61, 62, 85, 86],
            &|c, seed| {
                // Seeds 300, 301: one entry without a long name, unwinding
                // at SHPS_GetEntryName, then at strcmp.
                let (seed, unwind) = shp_unwind_seed(seed, 300, &[5]);
                shps_find_call(c, seed, 0x003b_39b8);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    pub(crate) use crate::game::world::wscriptcache::tests::shps_put_sheet;

    driven!(gen_shps_find_entry_by_name, ee::s13::f_003b3900__generated);

    driven!(hand_shps_find_entry_by_name, shps_find_entry_by_name);

    /// 20..31 entries (nothing matches, strcmp faked) with live events at
    /// the loop's back edge.
    #[test]
    fn shps_find_entry_by_name_events() {
        check_events(
            gen_shps_find_entry_by_name,
            hand_shps_find_entry_by_name,
            150,
            &|c, seed| {
                // strcmp faked (its real body unwinds mid-call).
                let mut r = shps_find_call(c, seed / 3 * 20 + 11, 0x003b_3900);
                let count = 20 + seed % 12;
                shps_put_sheet(c, &mut r, count, b"nope");
                schedule(c, &mut r, seed, 0x003b_3900)
            },
        );
    }
}
