//! `src/seg/seg_156860.cpp`: hand-written replacements (moved from the
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
    (0x0025_5a10, c_comm_system_get_version_tag, "cCommSystem_GetVersionTag"),
    (0x0025_5d38, c_comm_system_has_channel, "cCommSystem_HasChannel"),
];


/// `const char *cCommSystem_GetVersionTag(void)` (0x255a10): returns the
/// version-tag string at 0x481370.
pub fn c_comm_system_get_version_tag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x48_0000u32.wrapping_add(0x1370));
    abi::ret(c);
}


/// `bool cCommSystem_HasChannel(cCommSystem *self, int index)` (0x255d38):
/// whether `self->channels[index]` is non-null.
pub fn c_comm_system_has_channel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
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
    fn c_comm_system_get_version_tag_matches() {
        check(ssx3_ee::s06::f_00255a10, c_comm_system_get_version_tag, 30, &|c, seed| {
            let mut r = Rng(0x255a_1000 ^ seed as u64);
            scramble(c, &mut r);
        });
    }


    #[test]
    fn c_comm_system_has_channel_matches() {
        check(ssx3_ee::s06::f_00255d38, c_comm_system_has_channel, 200, &|c, seed| {
            let mut r = Rng(0x255d_3800 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 4) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                idx as u64
            } else {
                0x1_0000_0000u64 | idx as u64
            };
            c.bus.write32(
                DATA + idx * 4,
                match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000,
                    3 => 0x7fff_ffff,
                    _ => r.next_u32(),
                },
            );
        });
    }
}
