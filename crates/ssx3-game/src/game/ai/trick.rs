//! `src/ai/trick.cpp`: hand-written replacements (moved from the
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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_6938, trick_get_grab_name, "Trick_GetGrabName")];

/// `char *Trick_GetGrabName(int grab)` (0x116938): look up the grab-name
/// pointer in the 4-byte-entry table based at 0x43d160 (0x440000 - 0x2ea0).
pub fn trick_get_grab_name(c: &mut Cpu) {
    c.cycles += 6;
    let index4 = abi::arg(c, 0).wrapping_shl(2);
    c.g[V0][0] = sx(0x44_0000u32.wrapping_sub(0x2ea0)); // table base
    let slot = Addr(index4.wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx(slot.0);
    c.g[V0][0] = slot.lw(c);
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
    fn trick_get_grab_name_matches() {
        check(
            ssx3_ee::s00::f_00116938,
            trick_get_grab_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x3344_5566 ^ seed as u64);
                scramble(c, &mut r);
                // Low 32 bits index the table; the upper half is junk (ignored).
                let idx = (seed % 64) as u32;
                c.g[A0][0] = idx as u64 | ((r.next_u32() as u64) << 32);
                let base = 0x43d160u32;
                for i in 0..64 {
                    c.bus.write32(base + i * 4, r.next_u32());
                }
            },
        );
    }
}
