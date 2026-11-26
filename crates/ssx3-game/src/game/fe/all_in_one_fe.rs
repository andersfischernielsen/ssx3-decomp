//! `src/fe/all_in_one_fe.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
#[cfg(test)]
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0021_ccc8, ui_icon_flags_ctor, "UIIconFlags_ctor")];

/// `UIIconFlags::ctor` (0x21ccc8): keeps the high nibble of the flags byte
/// at `*a0` and clears the low one. Returns the flags pointer in v0.
pub fn ui_icon_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let flags = c.g[A0][0];
    let masked = Addr(flags as u32).lbu(c) & 0xf0;
    c.g[V1][0] = masked;
    c.g[V0][0] = flags;
    c.g[V1][0] = masked;
    Addr(flags as u32).sb(c, masked);
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn ui_icon_flags_ctor_matches() {
        check(
            ee::s05::f_0021ccc8__generated,
            ui_icon_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0021_ccc8 ^ seed as u64);
                scramble(c, &mut r);
                let p = DATA + 0x100 + 0x10 * (seed % 16);
                c.g[A0][0] = sx(p);
                c.bus.write8(p, r.next_u32() as u8);
            },
        );
    }
}
