//! `src/bx/seg_1ba100.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x002b_9100, audio_debug_printf, "Audio_DebugPrintf")];

// ---------------------------------------------------------------------------
// Audio.

/// `void Audio_DebugPrintf(int unused, ...)` (0x2b9100): spill the register
/// arguments a1..a3/t0..t3 and float arguments f12/f14/f16/f18 into the caller's
/// frame (a debug printf stub with an empty body).
pub fn audio_debug_printf(c: &mut Cpu) {
    c.cycles += 14;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x80);
    c.g[SP][0] = sx(sp);
    let s = Addr(sp);
    s.off(0x48).sd(c, c.g[A1][0]);
    s.off(0x50).sd(c, c.g[A2][0]);
    s.off(0x58).sd(c, c.g[A3][0]);
    s.off(0x60).sd(c, c.g[T0][0]);
    s.off(0x68).sd(c, c.g[T1][0]);
    s.off(0x70).sd(c, c.g[T2][0]);
    s.off(0x78).sd(c, c.g[T3][0]);
    s.off(0x38).write_fpu(c, FpuF32(c.f[12]));
    s.off(0x3c).write_fpu(c, FpuF32(c.f[14]));
    s.off(0x40).write_fpu(c, FpuF32(c.f[16]));
    s.off(0x44).write_fpu(c, FpuF32(c.f[18]));
    c.g[SP][0] = sx(sp.wrapping_add(0x80));
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
    fn audio_debug_printf_matches() {
        check(
            ssx3_ee::s08::f_002b9100,
            audio_debug_printf,
            100,
            &|c, seed| {
                let mut r = Rng(0x9100 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }
}
