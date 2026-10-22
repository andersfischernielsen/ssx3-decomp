//! `src/dirtysock/dnasauth.cpp`: hand-written replacements (moved from the
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
    (0x0025_6228, dnasauth_init, "DNASAuth_Init"),
];


/// `void DNASAuth_Init(DNASAuth *self)` (0x256228): clear the auth state and
/// set the handle fields to -1.
pub fn dnasauth_init(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.sw(c, 0);
    this.off(0x18).sw(c, c.g[V0][0]);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
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
    fn dnasauth_init_matches() {
        check(ssx3_ee::s06::f_00256228, dnasauth_init, 100, &|c, seed| {
            let mut r = Rng(0x2562_2800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }
}
