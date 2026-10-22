//! `src/render/morton.cpp`: hand-written replacements (moved from the
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
    (0x0036_a3b0, morton_spread8, "Morton_Spread8"),
];


/// `uint Morton_Spread8(uint x)` (0x36a3b0): spread the low 8 bits of `x` into
/// every other bit.
pub fn morton_spread8(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = c.g[A0][0] & 0xff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0xf0f;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x3333;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x5555;
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
    fn morton_spread8_matches() {
        check(ssx3_ee::s12::f_0036a3b0, morton_spread8, 200, &|c, seed| {
            let mut r = Rng(0x36a3_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
        });
    }
}
