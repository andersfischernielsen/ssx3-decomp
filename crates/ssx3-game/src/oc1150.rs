//! Staging module oc1150 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1150`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_5b30, snd_set_config_percent_4a, "SND_SetConfigPercent4A"),
    (0x003c_5b80, snd_set_config_42, "SND_SetConfig42"),
];

/// `SND_GetConfig(SndOptions* out)` (0x3b4dd8), called first by both setters.
pub(crate) const SND_GET_CONFIG_CALLEE: Callee = Callee {
    addr: 0x003b_4dd8,
    f: ee::s13::f_003b4dd8,
};

/// `SND_SetConfig(const SndOptions* cfg)` (0x3b4fc0), applies the copy.
pub(crate) const SND_SET_CONFIG_CALLEE: Callee = Callee {
    addr: 0x003b_4fc0,
    f: ee::s13::f_003b4fc0,
};

/// `void SND_SetConfigPercent4A(float percent)` (0x3c5b30): stores
/// `(int)(percent * 100)` as the byte at +0x4a of a copy of the SND options
/// (fetched with `SND_GetConfig` into the frame) and applies the copy with
/// `SND_SetConfig`. Returns 0. The reference calls it `System_SetDtsQuality`.
pub fn snd_set_config_percent_4a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x100);
        frame.save_fpr(c, 20, 0xf0);
        // The frame's `ra` is saved before the callee overwrites it.
        frame.save_ra(c, 0xe0);
        let percent = abi::farg(c, 0);
        abi::set_fpr(c, 20, percent);
        c.g[A0][0] = c.g[SP][0];
        SND_GET_CONFIG_CALLEE.call(c, 0x003c_5b48)?;
        // `mtc1 AT, f0`: the constant 100.0f is left in AT as well.
        c.g[AT][0] = 0x42c8_0000;
        let hundred = FpuF32(c.g[AT][0] as u32);
        abi::set_fpr(c, 0, hundred);
        c.g[A0][0] = c.g[SP][0];
        // `mul.s f20, f20, f0` then `cvt.w.s f0, f20` (truncating).
        let scaled = FpuF32(c.f[20]) * hundred;
        abi::set_fpr(c, 20, scaled);
        let fixed = FpuF32(scaled.to_i32() as u32);
        abi::set_fpr(c, 0, fixed);
        c.g[V0][0] = abi::mfc1(fixed);
        let v0 = c.g[V0][0];
        Addr(frame.at(c, 0x4a)).sb(c, v0);
        SND_SET_CONFIG_CALLEE.call(c, 0x003c_5b68)?;
        frame.restore_ra(c, 0xe0);
        c.g[V0][0] = 0;
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SetConfig42(int value)` (0x3c5b80): stores the low byte of the
/// argument at +0x42 of a copy of the SND options (fetched with
/// `SND_GetConfig` into the frame) and applies the copy with `SND_SetConfig`.
/// Returns 0. The reference calls it `System_SetRouteMainFxToIop`.
pub fn snd_set_config_42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        SND_GET_CONFIG_CALLEE.call(c, 0x003c_5b98)?;
        let value = c.g[S0][0];
        Addr(frame.at(c, 0x42)).sb(c, value);
        c.g[A0][0] = c.g[SP][0];
        SND_SET_CONFIG_CALLEE.call(c, 0x003c_5ba4)?;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and mode word of the test fakes, in the machine's own RAM
    /// (`DATA + 0x20_1150`): the fakes answer only when the fixture wrote
    /// [`OC1150_MAGIC`] there. Mode 1 makes `SND_GetConfig` unwind, mode 2
    /// makes `SND_SetConfig` unwind, mode 0 returns normally.
    const OC1150_GATE: u32 = DATA + 0x20_1150;
    const OC1150_MODE: u32 = OC1150_GATE + 4;
    const OC1150_MAGIC: u32 = 0x1150_c0de;

    static OC1150_PREV_GET: AtomicUsize = AtomicUsize::new(0);
    static OC1150_PREV_SET: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `SND_GetConfig`: returns (or unwinds in mode 1).
    fn oc1150_fake_get_config(c: &mut Cpu) {
        if c.bus.read32(OC1150_GATE) != OC1150_MAGIC {
            return fakes::fall_back(c, &OC1150_PREV_GET, ee::s13::f_003b4dd8__generated);
        }
        c.g[V0][0] = 0x1150;
        c.pc = if c.bus.read32(OC1150_MODE) == 1 {
            0xdead_0010
        } else {
            c.g[RA][0] as u32
        };
    }

    /// Gated fake for `SND_SetConfig`: returns 0 (or unwinds in mode 2).
    fn oc1150_fake_set_config(c: &mut Cpu) {
        if c.bus.read32(OC1150_GATE) != OC1150_MAGIC {
            return fakes::fall_back(c, &OC1150_PREV_SET, ee::s13::f_003b4fc0__generated);
        }
        c.g[V0][0] = 0;
        c.pc = if c.bus.read32(OC1150_MODE) == 2 {
            0xdead_0020
        } else {
            c.g[RA][0] as u32
        };
    }

    /// Both setters call the two fakes: hook them into the override slots
    /// (once per process) and write the gate in this test's machine.
    fn oc1150_install(c: &mut Cpu, seed: u32) {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4DD8),
            oc1150_fake_get_config,
            &OC1150_PREV_GET,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4FC0),
            oc1150_fake_set_config,
            &OC1150_PREV_SET,
        );
        c.bus.write32(OC1150_GATE, OC1150_MAGIC);
        c.bus.write32(OC1150_MODE, seed % 3);
    }

    /// `SND_SetConfigPercent4A`: edge and ordinary percentages, both
    /// callees returning or unwinding.
    #[test]
    fn snd_set_config_percent_4a_matches() {
        check(
            ee::s13::f_003c5b30__generated,
            snd_set_config_percent_4a,
            200,
            &|c, seed| {
                let mut r = Rng(0x003c_5b30 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                oc1150_install(c, seed);
                c.f[12] = match seed % 4 {
                    0 => r.edge_f32(),
                    1 => r.f32(1.0e9),
                    _ => r.f32(2.0),
                };
            },
        );
    }

    /// `SND_SetConfig42`: the argument's low byte stored with the callees
    /// returning or unwinding.
    #[test]
    fn snd_set_config_42_matches() {
        check(
            ee::s13::f_003c5b80__generated,
            snd_set_config_42,
            200,
            &|c, seed| {
                let mut r = Rng(0x003c_5b80 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                oc1150_install(c, seed);
            },
        );
    }
}
