//! Staging module oc1324 (sceio-1 wave, printf + pad-device stubs):
//! hand-written replacements, each with its differential test against the
//! generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1324`.

use crate::call::{guest, Callee};
use crate::difftest::fakes::{covered, machine};
use crate::difftest::{check_paths, DATA};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_5c28, libkernl_printf, "libkernl_printf"),
    (
        0x0032_7800,
        pad_device_get_num_actuators,
        "PadDevice_GetNumActuators",
    ),
    (
        0x0032_7808,
        pad_device_set_actuator,
        "PadDevice_SetActuator",
    ),
];

pub(crate) mod oc1324_callee {
    use super::ee;
    use super::Callee;

    /// `libkernl_vprintf` (`0x00425638`, dropped from this lane: never runs).
    pub(crate) const OC1324_VPRINTF: Callee = Callee {
        addr: 0x0042_5638,
        f: ee::s15::f_00425638,
    };
}

/// `int libkernl_printf(const char *fmt, ...)` (0x00425c28): format into the
/// caller's stack buffer through `libkernl_vprintf`.
pub fn libkernl_printf(c: &mut Cpu) {
    use oc1324_callee::*;
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x58));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T2][0]);
        c.g[RA][0] = sx(0x00425c54);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T3][0]);
        OC1324_VPRINTF.call(c, 0x00425c54)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `int PadDevice_GetNumActuators(...)` (0x00327800): no actuators.
pub fn pad_device_get_num_actuators(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
}

/// `int PadDevice_SetActuator(...)` (0x00327808): ignored.
pub fn pad_device_set_actuator(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const SEEDS: u32 = 200;

    /// `libkernl_printf`: a small integer format and a plain string, with
    /// the varargs spilled to the stack frame like the caller would.
    #[test]
    fn libkernl_printf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_324_001, 2);
            c.pc = 0x0042_5c28;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Format digit: an integer conversion and a plain string.
            let fmt: &[u8] = if seed % 2 == 0 {
                b"v=%d\n\0"
            } else {
                b"hi\n\0"
            };
            for (i, b) in fmt.iter().enumerate() {
                c.bus.write8(DATA + 0x100 + i as u32, *b);
            }
            c.g[A1][0] = (seed.wrapping_mul(0x9e37_79b9) & 0xffff) as u64;
            c.g[A2][0] = (seed.wrapping_mul(0x5bd1_e995) | 0x0101_0100) as u64;
            c.g[A3][0] = (seed.wrapping_mul(0x1b87_3b09) | 0x0101_0100) as u64;
            c.g[T0][0] = 0x0101_0100;
            c.g[T1][0] = 0x0101_0101;
            c.g[T2][0] = 0x0101_0102;
            c.g[T3][0] = 0x0101_0103;
        };
        covered(ee::s15::f_00425c28, libkernl_printf, SEEDS, &setup);
    }

    /// `PadDevice_GetNumActuators`: always 0.
    #[test]
    fn pad_device_get_num_actuators_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_324_002, 0);
            c.pc = 0x0032_7800;
        };
        check_paths(
            ee::s10::f_00327800,
            pad_device_get_num_actuators,
            SEEDS,
            &[2],
            &setup,
        );
    }

    /// `PadDevice_SetActuator`: ignored.
    #[test]
    fn pad_device_set_actuator_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_324_003, 0);
            c.pc = 0x0032_7808;
            c.g[A0][0] = (seed.wrapping_mul(0x9e37_79b9) | 1) as u64;
            c.g[A1][0] = (seed.wrapping_mul(0x5bd1_e995) | 1) as u64;
        };
        check_paths(
            ee::s10::f_00327808,
            pad_device_set_actuator,
            SEEDS,
            &[2],
            &setup,
        );
    }
}
