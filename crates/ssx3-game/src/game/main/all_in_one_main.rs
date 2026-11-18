//! `src/main/all_in_one_main.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001d_d580, ui_screen_flags_ctor, "UIScreenFlags_ctor")];

/// `UIScreenFlags* UIScreenFlags_ctor(UIScreenFlags* self)` (0x001dd580):
/// sets bit 0 and clears bit 1 of the flag halfword at +0 and returns `self`.
pub fn ui_screen_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] |= 0x1;
    c.g[V1][0] &= 0xfffd;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = ra;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// Flag halfword with every combination of bits 0 and 1 and random
    /// upper bits.
    #[test]
    fn ui_screen_flags_ctor_matches() {
        check(
            ee::s04::f_001dd580__generated,
            ui_screen_flags_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd580, 0);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(OC431_BASE + 0x100, r.next_u32());
                c.pc = 0x001d_d580;
            },
        );
    }
}
