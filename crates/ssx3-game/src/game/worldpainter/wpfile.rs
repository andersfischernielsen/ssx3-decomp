//! `src/worldpainter/wpfile.cpp`: hand-written replacements (moved from the
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
    (0x002b_b370, wpfile_ctor, "WPFile_ctor"),
];


/// `WPFile* WPFile_ctor(WPFile* f)` (0x2bb370): clear the two half-word fields
/// at +0/+2, set +4 to -1 and +8 to 0.
pub fn wpfile_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let f = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    f.off(0x8).sw(c, 0);
    f.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    f.sh(c, 0);
    f.off(0x2).sh(c, 0);
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
    fn wpfile_ctor_matches() {
        check(ssx3_ee::s08::f_002bb370, wpfile_ctor, 50, &|c, seed| {
            let mut r = Rng(0xb370 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }
}
