//! Staging module oc1149 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1149`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_5788, snd_get_sample_rate, "SND_GetSampleRate"),
    (0x003c_57f0, snd_get_config_core_byte, "SND_GetConfigCoreByte"),
    (0x003c_5880, snd_set_config_core_param, "SND_SetConfigCoreParam"),
    (0x003c_5908, snd_get_config_core_param, "SND_GetConfigCoreParam"),
    (0x003c_5998, snd_set_config_percent_pair, "SND_SetConfigPercentPair"),
    (0x003c_5a28, snd_get_config_percent_pair, "SND_GetConfigPercentPair"),
    (0x003c_5ac0, snd_set_config_3f, "SND_SetConfig3F"),
    (0x003c_5af8, snd_set_config_6c, "SND_SetConfig6C"),
];

/// `int SND_GetSampleRate(int index, int* out)` (0x3c5788): `*out` gets the
/// output sample rate (index 0) or the IOP output rate (index 1) of the sound
/// configuration, read through `SND_GetConfig` into a 0xe0-byte record (the
/// rates are its halfwords at +0x2a and +0x2c). Any other index returns -5;
/// success returns 0.
pub fn snd_get_sample_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, sp(c).wrapping_add(0x100));
        c.sq(17, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0u64);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_57ac)?;
        c.g[V0][0] = 0x1;
        let mut ok = false;
        if c.g[S1][0] == 0u64 {
            c.cycles += 3;
            c.g[V0][0] = c.lhu(sp(c).wrapping_add(0x2a));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            ok = true;
        } else {
            c.cycles += 2;
            let one = c.g[S1][0] == c.g[V0][0];
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            if one {
                c.cycles += 2;
                c.g[V0][0] = c.lhu(sp(c).wrapping_add(0x2c));
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                ok = true;
            }
        }
        c.cycles += 2;
        if ok {
            c.g[V0][0] = 0u64;
        }
        c.lq(16, sp(c).wrapping_add(0x100));
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0xf0));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_GetConfigCoreByte(int index, u8* out)` (0x3c57f0): `*out` gets
/// the byte at +6 (index 0), +7 (index 1) or +8 (index 2) of the sound
/// configuration record read through `SND_GetConfig`. Any other index returns
/// -5 and leaves `*out` at 0; success returns 0.
pub fn snd_get_config_core_byte(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(17, sp(c).wrapping_add(0xf0));
        c.sq(16, sp(c).wrapping_add(0x100));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S1][0] as u32, 0u64);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_5814)?;
        c.g[V1][0] = 0x1;
        let index = c.g[S0][0];
        c.g[V0][0] = ((index as i64) < 2) as u64;
        let mut ok = false;
        if index == c.g[V1][0] {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x7));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            ok = true;
        } else {
            c.cycles += 2;
            let big = c.g[V0][0] == 0;
            c.g[V0][0] = 0x2;
            if big {
                c.cycles += 2;
                let two = index == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if two {
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x8));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    ok = true;
                }
            } else {
                c.cycles += 2;
                let zero = index == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if zero {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x6));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    ok = true;
                }
            }
        }
        c.cycles += 2;
        if ok {
            c.g[V0][0] = 0u64;
        }
        c.lq(16, sp(c).wrapping_add(0x100));
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0xf0));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_SetConfigCoreParam(int index, u8 value)` (0x3c5880): reads the
/// configuration through `SND_GetConfig`, stores `value` in the byte at +0x2e
/// (index 0), +0x2f (index 1) or +0x30 (index 2), and writes the record back
/// through `SND_SetConfig`. Any other index returns -5 without writing;
/// success returns 0.
pub fn snd_set_config_core_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, sp(c).wrapping_add(0x100));
        c.sq(17, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_58a0)?;
        c.g[V1][0] = 0x1;
        let index = c.g[S0][0];
        c.g[V0][0] = ((index as i64) < 2) as u64;
        let mut ok = false;
        if index == c.g[V1][0] {
            c.cycles += 2;
            c.sb(sp(c).wrapping_add(0x2f), c.g[S1][0]);
            ok = true;
        } else {
            c.cycles += 2;
            let big = c.g[V0][0] == 0;
            c.g[V0][0] = 0x2;
            if big {
                c.cycles += 2;
                let two = index == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if two {
                    c.cycles += 1;
                    c.sb(sp(c).wrapping_add(0x30), c.g[S1][0]);
                    ok = true;
                } else {
                    c.cycles += 2;
                    c.lq(16, sp(c).wrapping_add(0x100));
                }
            } else {
                c.cycles += 2;
                let zero = index == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if zero {
                    c.cycles += 2;
                    c.sb(sp(c).wrapping_add(0x2e), c.g[S1][0]);
                    ok = true;
                } else {
                    c.cycles += 2;
                    c.lq(16, sp(c).wrapping_add(0x100));
                }
            }
        }
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x003b_4fc0, 0x003c_58f0)?;
            c.g[V0][0] = 0u64;
            c.lq(16, sp(c).wrapping_add(0x100));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0xf0));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_GetConfigCoreParam(int index, u8* out)` (0x3c5908): `*out` gets
/// the byte at +0x2e (index 0), +0x2f (index 1) or +0x30 (index 2) of the
/// sound configuration record read through `SND_GetConfig`. Any other index
/// returns -5; success returns 0.
pub fn snd_get_config_core_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, sp(c).wrapping_add(0x100));
        c.sq(17, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_5928)?;
        c.g[V1][0] = 0x1;
        let index = c.g[S0][0];
        c.g[V0][0] = ((index as i64) < 2) as u64;
        let mut ok = false;
        if index == c.g[V1][0] {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x2f));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            ok = true;
        } else {
            c.cycles += 2;
            let big = c.g[V0][0] == 0;
            c.g[V0][0] = 0x2;
            if big {
                c.cycles += 2;
                let two = index == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if two {
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x30));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    ok = true;
                }
            } else {
                c.cycles += 2;
                let zero = index == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if zero {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x2e));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    ok = true;
                }
            }
        }
        c.cycles += 2;
        if ok {
            c.g[V0][0] = 0u64;
        }
        c.lq(16, sp(c).wrapping_add(0x100));
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0xf0));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_SetConfigPercentPair(int index, float percent)` (0x3c5998):
/// reads the configuration through `SND_GetConfig`, stores
/// `(int)(percent * 100)` as a byte at +0x40 (index 0) or +0x41 (index 1),
/// and writes the record back through `SND_SetConfig`. Any other index
/// returns -5 without writing; success returns 0. `f20` (the percent) is
/// saved in the frame and restored on the way out.
pub fn snd_set_config_percent_pair(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0x100));
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.mov_s(20, 12);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_59b8)?;
        let index = c.g[S0][0];
        c.g[V0][0] = 0x1;
        let mut ok = false;
        if index == 0 {
            c.cycles += 7;
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(1, 0);
            c.mul_s(0, 20, 0);
            c.cvt_w_s(1, 0);
            c.mfc1(2, 1);
            c.sb(sp(c).wrapping_add(0x40), c.g[V0][0]);
            ok = true;
        } else {
            c.cycles += 2;
            let one = index == c.g[V0][0];
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            if one {
                c.cycles += 6;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.mul_s(0, 20, 0);
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                c.sb(sp(c).wrapping_add(0x41), c.g[V0][0]);
                ok = true;
            } else {
                c.cycles += 2;
                c.lq(16, sp(c).wrapping_add(0xf0));
            }
        }
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x003b_4fc0, 0x003c_5a0c)?;
            c.g[V0][0] = 0u64;
            c.lq(16, sp(c).wrapping_add(0xf0));
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        c.lwc1(20, sp(c).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `float SND_GetConfigPercentPair(int index, float* out)` (0x3c5a28): reads
/// the configuration through `SND_GetConfig`, then `*out` gets the byte at
/// +0x40 (index 0) or +0x41 (index 1) as a fraction (`byte * 0.01f`). Any
/// other index returns -5 and leaves `*out` alone; success returns 0.
pub fn snd_get_config_percent_pair(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, sp(c).wrapping_add(0x100));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, sp(c).wrapping_add(0xf0));
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_5a48)?;
        let index = c.g[S0][0];
        c.g[V0][0] = 0x1;
        let mut ok = false;
        if index == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x40));
            ok = true;
        } else {
            c.cycles += 2;
            let one = index == c.g[V0][0];
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            if one {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(sp(c).wrapping_add(0x41));
                ok = true;
            } else {
                c.cycles += 2;
                c.lq(16, sp(c).wrapping_add(0x100));
            }
        }
        if ok {
            c.cycles += 9;
            c.g[AT][0] = 0x3c23_0000;
            c.g[AT][0] = c.g[AT][0] | 0xd70a;
            c.mtc1(1, 1);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
            c.swc1(0, (c.g[S1][0] as u32));
            c.g[V0][0] = 0u64;
            c.lq(16, sp(c).wrapping_add(0x100));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0xf0));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_SetConfig3F(u8 value)` (0x3c5ac0): reads the configuration
/// through `SND_GetConfig`, stores `value` at +0x3f and writes the record
/// back through `SND_SetConfig`. Returns 0.
pub fn snd_set_config_3f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.sq(16, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_5ad8)?;
        c.sb(sp(c).wrapping_add(0x3f), c.g[S0][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4fc0, 0x003c_5ae4)?;
        c.lq(16, sp(c).wrapping_add(0xf0));
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    });
}

/// `int SND_SetConfig6C(int value)` (0x3c5af8): reads the configuration
/// through `SND_GetConfig`, stores the word `value` at +0x6c and writes the
/// record back through `SND_SetConfig`. Returns 0.
pub fn snd_set_config_6c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.sq(16, sp(c).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4dd8, 0x003c_5b10)?;
        c.sw(sp(c).wrapping_add(0x6c), c.g[S0][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x003b_4fc0, 0x003c_5b1c)?;
        c.lq(16, sp(c).wrapping_add(0xf0));
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, machine, mix};
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word, the seed word and the magic that arms the fakes: RAM in
    /// each test machine, not statics, so parallel tests cannot see each other.
    pub(crate) const OC1149_CTRL: u32 = DATA + 0x20_1140;
    pub(crate) const OC1149_SEED: u32 = DATA + 0x20_1144;
    pub(crate) const OC1149_MAGIC: u32 = 0x1149_c0de;

    /// The override slots the fakes were hooked over (their gate-off path).
    pub(crate) static OC1149_PREV_GET_CONFIG: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1149_PREV_SET_CONFIG: AtomicUsize = AtomicUsize::new(0);

    /// The index / which arguments the setups cover: both defined indices,
    /// the first undefined one, and a negative one.
    pub(crate) const OC1149_WHICH: [u32; 5] = [0, 1, 2, 3, 0xffff_ffff];

    /// `SND_GetConfig` (0x3b4dd8) while the gate is on: fills the 0xe0-byte
    /// record the caller passed in `a0` with seed-derived bytes, then returns
    /// through `fake` (logged, registers clobbered).
    pub(crate) fn oc1149_fake_get_config(c: &mut Cpu) {
        if c.bus.read32(OC1149_CTRL) != OC1149_MAGIC {
            return fall_back(c, &OC1149_PREV_GET_CONFIG, ee::s13::f_003b4dd8__generated);
        }
        let record = c.g[A0][0] as u32;
        let seed = c.bus.read32(OC1149_SEED) as u64;
        for i in 0..0xe0u32 {
            let byte = mix(seed.wrapping_mul(0x101) ^ i as u64) as u8;
            c.bus.write8(record.wrapping_add(i), byte);
        }
        let back = c.g[RA][0] as u32;
        fake(c, 0x003b_4dd8, back);
    }

    /// `SND_SetConfig` (0x3b4fc0) while the gate is on: logged and returned.
    pub(crate) fn oc1149_fake_set_config(c: &mut Cpu) {
        if c.bus.read32(OC1149_CTRL) != OC1149_MAGIC {
            return fall_back(c, &OC1149_PREV_SET_CONFIG, ee::s13::f_003b4fc0__generated);
        }
        let back = c.g[RA][0] as u32;
        fake(c, 0x003b_4fc0, back);
    }

    /// Hooks the two fakes on their override slots (once per process).
    pub(crate) fn oc1149_install() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4DD8),
            oc1149_fake_get_config,
            &OC1149_PREV_GET_CONFIG,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4FC0),
            oc1149_fake_set_config,
            &OC1149_PREV_SET_CONFIG,
        );
    }

    /// A scrambled machine with the fakes armed for this seed.
    pub(crate) fn oc1149_machine(c: &mut Cpu, seed: u32, salt: u64) {
        oc1149_install();
        machine(c, seed, salt, 0);
        c.bus.write32(OC1149_CTRL, OC1149_MAGIC);
        c.bus.write32(OC1149_SEED, seed);
    }

    #[test]
    fn snd_get_sample_rate_matches() {
        check(ee::s13::f_003c5788__generated, snd_get_sample_rate, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5788);
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.g[A1][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn snd_get_config_core_byte_matches() {
        check(ee::s13::f_003c57f0__generated, snd_get_config_core_byte, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_57f0);
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.g[A1][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn snd_set_config_core_param_matches() {
        check(ee::s13::f_003c5880__generated, snd_set_config_core_param, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5880);
            let mut r = Rng(0x003c_5880 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    #[test]
    fn snd_get_config_core_param_matches() {
        check(ee::s13::f_003c5908__generated, snd_get_config_core_param, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5908);
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.g[A1][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn snd_set_config_percent_pair_matches() {
        check(ee::s13::f_003c5998__generated, snd_set_config_percent_pair, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5998);
            let mut r = Rng(0x003c_5998 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.f[12] = if seed % 2 == 0 { r.f32(1.5) } else { r.edge_f32() };
        });
    }

    #[test]
    fn snd_get_config_percent_pair_matches() {
        check(ee::s13::f_003c5a28__generated, snd_get_config_percent_pair, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5a28);
            c.g[A0][0] = sx(OC1149_WHICH[(seed % 5) as usize]);
            c.g[A1][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn snd_set_config_3f_matches() {
        check(ee::s13::f_003c5ac0__generated, snd_set_config_3f, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5ac0);
            let mut r = Rng(0x003c_5ac0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            c.g[A0][0] = sx(r.next_u32());
        });
    }

    #[test]
    fn snd_set_config_6c_matches() {
        check(ee::s13::f_003c5af8__generated, snd_set_config_6c, 200, &|c, seed| {
            oc1149_machine(c, seed, 0x003c_5af8);
            let mut r = Rng(0x003c_5af8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            c.g[A0][0] = sx(r.next_u32());
        });
    }
}
