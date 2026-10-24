//! `src/render/shps.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003b_3d00, shps_get_entry_name, "SHPS_GetEntryName")];

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

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
}
