//! `src/seg/seg_21e5a8.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_d5e8, c_csi_cubic_spline_ctor, "cCSICubicSpline_ctor"),
];


/// `cCSICubicSpline *cCSICubicSpline_ctor(cCSICubicSpline *this)`
/// (0x31d5e8): mask the flags word at +0 and clear the count at +8.
pub fn c_csi_cubic_spline_ctor(c: &mut Cpu) {
    c.cycles += 12;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[A2][0] = 0xffff_ffff_ffff_fffd;
    c.g[A3][0] = 0xffff_ffff_ffff_fffb;
    c.g[A1][0] = 0xffff_ffff_fff8_0000;
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[A1][0] = c.g[A1][0] | 0x7ff;
    c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
    Addr(this).off(0x8).sw(c, 0);
    Addr(this).sw(c, c.g[V1][0]);
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
    fn c_csi_cubic_spline_ctor_matches() {
        check(ssx3_ee::s10::f_0031d5e8, c_csi_cubic_spline_ctor, 200, &|c, seed| {
            let mut r = Rng(0x201a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }
}
