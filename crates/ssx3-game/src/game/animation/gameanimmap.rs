//! `src/animation/gameanimmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
#[cfg(test)]
use ssx3_ee as ee;
#[cfg(test)]
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0031_50b0, anim_data_empty_hook, "AnimData_EmptyHook")];

/// `void AnimData_EmptyHook(void)` (0x3150b0): does nothing.
pub fn anim_data_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Leaf setters and getters: random registers, argument pointing at the
    /// data area, struct contents random.
    pub(crate) fn oc438_leaf(gen: GuestFn, hand: GuestFn, entry: u32) {
        check(gen, hand, 200, &|c, seed| {
            let mut r = Rng(0x1eaf ^ entry as u64 ^ (seed as u64) << 32);
            scramble(c, &mut r);
            c.pc = entry;
            let p = DATA + (r.next_u32() & 0xf_ff00 & !3);
            c.g[A0][0] = p as u64;
            for k in 0..0x40 {
                c.bus.write32(p + 4 * k, r.next_u32());
            }
            // Flag bytes and ids: zero, small, negative, any.
            let word = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.bus.write32(p + 8, word);
            c.bus.write32(p, word ^ 0xe5);
        });
    }

    #[test]
    fn anim_data_empty_hook_matches() {
        oc438_leaf(
            ee::s10::f_003150b0__generated,
            anim_data_empty_hook,
            0x0031_50b0,
        );
    }
}
