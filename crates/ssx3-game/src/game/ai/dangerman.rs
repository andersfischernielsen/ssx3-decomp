//! `src/ai/dangerman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_1a10, danger_man_get_rider_danger, "DangerMan_GetRiderDanger"),
];


// ---------------------------------------------------------------------------
// Danger manager.

/// `int DangerMan_GetRiderDanger(DangerMan *man, int rider)` (0x101a10):
/// `man->riders[rider].danger` at `man + rider*12 + 0x710`.
pub fn danger_man_get_rider_danger(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    let scaled = abi::mult(c, abi::arg(c, 1), 0xc);
    c.g[V1][0] = sx(scaled);
    let slot = abi::arg(c, 0).wrapping_add(scaled);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(0x710).lw(c);
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;


    #[test]
    fn danger_man_get_rider_danger_matches() {
        check(ssx3_ee::s00::f_00101a10__generated, danger_man_get_rider_danger, 200, &|c, seed| {
            let mut r = Rng(0x2718_2818 ^ seed as u64);
            scramble(c, &mut r);
            let rider = seed % 5;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = rider as u64;
            c.bus.write32(DATA + rider * 12 + 0x710, r.next_u32());
        });
    }
}
