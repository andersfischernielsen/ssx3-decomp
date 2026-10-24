//! `src/dirtysock/gamecomm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0025_8330,
    c_game_comm_update_max_latency,
    "cGameComm_UpdateMaxLatency",
)];

// ---------------------------------------------------------------------------
// Game comm.

/// `void cGameComm_UpdateMaxLatency(cGameComm *self)` (0x258330): keep the
/// running maximum of the two latency samples, clamped to the world's target
/// latency.
pub fn c_game_comm_update_max_latency(c: &mut Cpu) {
    c.cycles += 10;
    let this = Addr(abi::arg(c, 0));
    let a = this.off(0x114).read_fpu(c);
    let b = this.off(0x118).read_fpu(c);
    c.f[0] = a.0;
    c.f[1] = b.0;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let max = b.max(a);
    c.f[1] = max.0;
    this.off(0x11c).write_fpu(c, max);
    let limit = Addr(world as u32).off(0x14).read_fpu(c);
    c.f[0] = limit.0;
    if abi::c_lt_s(c, max, limit) {
        this.off(0x11c).write_fpu(c, limit);
    }
    c.cycles += 2;
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
    fn c_game_comm_update_max_latency_matches() {
        check(
            ssx3_ee::s06::f_00258330,
            c_game_comm_update_max_latency,
            300,
            &|c, seed| {
                let mut r = Rng(0x2583_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let world = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                // Seeds 0 and 1 force the FPU compare both ways; the rest fuzz.
                let (a, b, limit) = match seed % 3 {
                    0 => (1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()),
                    1 => (5.0f32.to_bits(), 4.0f32.to_bits(), 1.0f32.to_bits()),
                    _ => (r.edge_f32(), r.edge_f32(), r.edge_f32()),
                };
                c.bus.write32(DATA + 0x114, a);
                c.bus.write32(DATA + 0x118, b);
                c.bus.write32(world + 0x14, limit);
                c.bus.write32(DATA + 0x11c, r.next_u32());
            },
        );
    }
}
