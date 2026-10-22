//! `src/md5.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_d368, md5_init, "MD5_Init"),
];


/// `void MD5_Init(MD5_CTX *ctx)` (0x31d368): load the standard MD5 constants.
pub fn md5_init(c: &mut Cpu) {
    c.cycles += 15;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x6745_0000;
    c.g[A1][0] = 0xffff_ffff_efcd_0000;
    c.g[A2][0] = 0xffff_ffff_98ba_0000;
    c.g[V0][0] = 0x1032_0000;
    c.g[V1][0] = c.g[V1][0] | 0x2301;
    c.g[A1][0] = c.g[A1][0] | 0xab89;
    c.g[A2][0] = c.g[A2][0] | 0xdcfe;
    c.g[V0][0] = c.g[V0][0] | 0x5476;
    Addr(this).off(0x8).sw(c, c.g[V1][0]);
    Addr(this).off(0x14).sw(c, c.g[V0][0]);
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    Addr(this).off(0x10).sw(c, c.g[A2][0]);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
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
    fn md5_init_matches() {
        check(ssx3_ee::s10::f_0031d368, md5_init, 100, &|c, seed| {
            let mut r = Rng(0x2019 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }
}
