//! `src/worldpainter/worldpainterman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_b130, cworld_painter_man_ctor, "cWorldPainterMan_ctor"),
];


// ---------------------------------------------------------------------------
// World painter files.

/// `cWorldPainterMan* cWorldPainterMan_ctor(cWorldPainterMan* self)`
/// (0x2bb130): clear +4, return self.
pub fn cworld_painter_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x4).sw(c, 0);
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
    fn cworld_painter_man_ctor_matches() {
        check(ssx3_ee::s08::f_002bb130, cworld_painter_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0xb130 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }
}
