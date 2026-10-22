//! `src/visualfx/hudfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_6fa8, chudfx_ctor, "cHUDFX_ctor"),
];


/// `cHUDFX *cHUDFX_ctor(cHUDFX *this)` (0x116fa8): set the state word at +0 to
/// 0x34 and return `this`.
pub fn chudfx_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.0 as u64;
    c.g[V1][0] = 0x34;
    Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
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
    fn chudfx_ctor_matches() {
        check(ssx3_ee::s00::f_00116fa8, chudfx_ctor, 50, &|c, seed| {
            let mut r = Rng(0x4455_6677 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }
}
