//! Staging module oc1563 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1563`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x002d_40c0, visual_effects_static_init, "VisualEffects_StaticInit")];

/// A value of one colour word: a `.lit4` constant read from the global
/// pointer's pool (`gp + offset`) or a literal bit pattern.
#[derive(Clone, Copy)]
pub(crate) enum VisualEffectsCv {
    Lit(i32),
    Bits(u32),
}
use VisualEffectsCv as Cv;

/// The 56 `sColor_40C0` (r, g, b, a) globals at 0x4d53f0..=0x4d5760 (step 0x10)
/// and the four floats each one is set to.
#[rustfmt::skip]
pub(crate) const VISUAL_EFFECTS_COLORS: [(u32, [Cv; 4]); 56] = [
    (0x004d53f0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f000000)]),
    (0x004d5400, [Cv::Lit(-0x3e4c), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0)]),
    (0x004d5410, [Cv::Lit(-0x3e48), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0)]),
    (0x004d5420, [Cv::Lit(-0x3e44), Cv::Lit(-0x3e24), Cv::Lit(-0x3e20), Cv::Lit(-0x3e40)]),
    (0x004d5430, [Cv::Lit(-0x3e3c), Cv::Lit(-0x3e1c), Cv::Lit(-0x3e18), Cv::Bits(0)]),
    (0x004d5440, [Cv::Lit(-0x3e38), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0)]),
    (0x004d5450, [Cv::Lit(-0x3e34), Cv::Lit(-0x3e14), Cv::Lit(-0x3e10), Cv::Lit(-0x3e30)]),
    (0x004d5460, [Cv::Lit(-0x3e2c), Cv::Lit(-0x3e0c), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d5470, [Cv::Lit(-0x3e28), Cv::Lit(-0x3e08), Cv::Lit(-0x3df8), Cv::Bits(0)]),
    (0x004d5480, [Cv::Lit(-0x3e04), Cv::Lit(-0x3df4), Cv::Lit(-0x3df0), Cv::Lit(-0x3e00)]),
    (0x004d5490, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d54a0, [Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d54b0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3dfc), Cv::Lit(-0x3dec), Cv::Lit(-0x3dfc)]),
    (0x004d54c0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d54d0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d54e0, [Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d54f0, [Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d5500, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d5510, [Cv::Bits(0x3f800000), Cv::Bits(0x3f000000), Cv::Bits(0x3f800000), Cv::Bits(0x3f000000)]),
    (0x004d5520, [Cv::Bits(0x3f800000), Cv::Bits(0x3f000000), Cv::Bits(0x3f800000), Cv::Bits(0x3f000000)]),
    (0x004d5530, [Cv::Bits(0x3f800000), Cv::Bits(0x3f000000), Cv::Bits(0x3f800000), Cv::Bits(0x3f000000)]),
    (0x004d5540, [Cv::Bits(0x3f800000), Cv::Lit(-0x3de0), Cv::Lit(-0x3ddc), Cv::Lit(-0x3de8)]),
    (0x004d5550, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Lit(-0x3dd8), Cv::Bits(0)]),
    (0x004d5560, [Cv::Lit(-0x3de4), Cv::Lit(-0x3dd4), Cv::Lit(-0x3dd0), Cv::Bits(0)]),
    (0x004d5570, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d5580, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Lit(-0x3dc4), Cv::Bits(0)]),
    (0x004d5590, [Cv::Bits(0x3f800000), Cv::Lit(-0x3dc0), Cv::Lit(-0x3dbc), Cv::Bits(0)]),
    (0x004d55a0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3db8), Cv::Lit(-0x3db4), Cv::Lit(-0x3dcc)]),
    (0x004d55b0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3da0), Cv::Lit(-0x3db0), Cv::Bits(0x3f800000)]),
    (0x004d55c0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3dac), Cv::Bits(0x3f800000), Cv::Lit(-0x3dc8)]),
    (0x004d55d0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d55e0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d55f0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3da8), Cv::Lit(-0x3da4), Cv::Bits(0)]),
    (0x004d5600, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d5610, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d8c), Cv::Bits(0x3f800000), Cv::Bits(0)]),
    (0x004d5620, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d5630, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d5640, [Cv::Lit(-0x3d9c), Cv::Bits(0), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d5650, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d88), Cv::Lit(-0x3d84), Cv::Lit(-0x3d98)]),
    (0x004d5660, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d80), Cv::Bits(0x3f800000), Cv::Lit(-0x3d94)]),
    (0x004d5670, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Lit(-0x3d7c), Cv::Bits(0)]),
    (0x004d5680, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d78), Cv::Bits(0x3f800000), Cv::Lit(-0x3d90)]),
    (0x004d5690, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0x3f800000)]),
    (0x004d56a0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3da0), Cv::Bits(0x3f800000), Cv::Lit(-0x3da0)]),
    (0x004d56b0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d56c0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d56d0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0)]),
    (0x004d56e0, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d6c), Cv::Bits(0x3f800000), Cv::Lit(-0x3d74)]),
    (0x004d56f0, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Lit(-0x3d68), Cv::Lit(-0x3d70)]),
    (0x004d5700, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Lit(-0x3d64), Cv::Bits(0)]),
    (0x004d5710, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d60), Cv::Lit(-0x3d5c), Cv::Bits(0)]),
    (0x004d5720, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d50), Cv::Lit(-0x3d4c), Cv::Bits(0)]),
    (0x004d5730, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d48), Cv::Lit(-0x3d44), Cv::Lit(-0x3d58)]),
    (0x004d5740, [Cv::Bits(0x3f800000), Cv::Lit(-0x3d40), Cv::Lit(-0x3d3c), Cv::Lit(-0x3d54)]),
    (0x004d5750, [Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0x3f800000), Cv::Bits(0)]),
    (0x004d5760, [Cv::Bits(0x3f800000), Cv::Bits(0x3f800000), Cv::Bits(0), Cv::Bits(0)]),
];

/// Address of the `sSlot_40C0` global (`gp + 0x23d0`, constructed with -1).
pub(crate) const VISUAL_EFFECTS_SLOT_GP_OFF: u32 = 0x23d0;

/// `void VisualEffects_StaticInit(int initialize, int priority)` (0x2d40c0):
/// the static-init body of the visual-effects unit. When `priority == 0xffff`
/// and `initialize != 0` it constructs the 56 colour globals (four floats
/// each, read from the gp literal pool or constants 0/1/0.5) and the slot
/// global at `gp + 0x23d0` to -1.
pub fn visual_effects_static_init(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    let sp = c.g[SP][0] as u32;
    for (i, reg) in [16usize, 17, 18, 19, 20, 21, 22, 23].into_iter().enumerate() {
        c.sq(reg, sp.wrapping_add(0x80 - 0x10 * i as u32));
    }
    let skip = c.g[A1][0] != c.g[V0][0];
    c.sq(30, sp);
    if !skip {
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        c.g[T5][0] = 0x4d0000;
        if !none {
            c.cycles += 491;
            let gp = c.g[GP][0] as u32;
            for (base, words) in VISUAL_EFFECTS_COLORS {
                for (i, w) in words.into_iter().enumerate() {
                    let v = match w {
                        Cv::Lit(off) => Addr(gp.wrapping_add(off as u32)).lwu(c) as u32,
                        Cv::Bits(b) => b,
                    };
                    Addr(base + 4 * i as u32).sw(c, v as u64);
                }
            }
            // sSlot_40C0(-1): a byte store, then the word store.
            Addr(gp.wrapping_add(VISUAL_EFFECTS_SLOT_GP_OFF)).sb(c, 0xff);
            Addr(gp.wrapping_add(VISUAL_EFFECTS_SLOT_GP_OFF)).sw(c, 0xffff_ffff);
            // Scratch registers the generated body leaves behind.
            c.g[A0][0] = 0x4d0000;
            c.g[A1][0] = 0x4d0000;
            c.g[A2][0] = 0x4d0000;
            c.g[A3][0] = 0x4d0000;
            c.g[T0][0] = 0x4d55c0;
            c.g[T1][0] = 0x4d55d0;
            c.g[T2][0] = 0x4d5750;
            c.g[T3][0] = 0x4d5570;
            c.g[T4][0] = 0x4d5670;
            c.g[T6][0] = 0x4d5710;
            c.g[T7][0] = 0x4d5730;
            c.g[T8][0] = 0x4d0000;
            c.g[T9][0] = 0x4d0000;
            c.g[AT][0] = 0x3f000000;
            c.g[T5][0] = 0x4d0000;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = u64::MAX;
            c.f[0] = 0x3f800000;
            c.f[1] = c.bus.read32(gp.wrapping_sub(0x3d54));
            c.f[2] = c.bus.read32(gp.wrapping_sub(0x3d58));
            c.f[3] = c.bus.read32(gp.wrapping_sub(0x3d4c));
            c.f[4] = c.bus.read32(gp.wrapping_sub(0x3d50));
            c.f[5] = c.bus.read32(gp.wrapping_sub(0x3d44));
            c.f[6] = c.bus.read32(gp.wrapping_sub(0x3d40));
            c.f[7] = c.bus.read32(gp.wrapping_sub(0x3d3c));
            c.f[8] = c.bus.read32(gp.wrapping_sub(0x3d48));
            c.f[9] = c.bus.read32(gp.wrapping_sub(0x3d7c));
            c.f[10] = c.bus.read32(gp.wrapping_sub(0x3d78));
            c.f[11] = c.bus.read32(gp.wrapping_sub(0x3da4));
            c.f[12] = c.bus.read32(gp.wrapping_sub(0x3da0));
            c.f[13] = c.bus.read32(gp.wrapping_sub(0x3e1c));
            c.f[14] = c.bus.read32(gp.wrapping_sub(0x3e18));
            c.f[15] = c.bus.read32(gp.wrapping_sub(0x3e14));
            c.f[16] = c.bus.read32(gp.wrapping_sub(0x3e10));
            c.f[17] = c.bus.read32(gp.wrapping_sub(0x3e24));
            c.f[18] = c.bus.read32(gp.wrapping_sub(0x3e20));
            c.f[19] = c.bus.read32(gp.wrapping_sub(0x3e08));
        }
    }
    c.cycles += 11;
    c.lq(16, sp.wrapping_add(0x80));
    c.lq(17, sp.wrapping_add(0x70));
    c.lq(18, sp.wrapping_add(0x60));
    c.lq(19, sp.wrapping_add(0x50));
    c.lq(20, sp.wrapping_add(0x40));
    c.lq(21, sp.wrapping_add(0x30));
    c.lq(22, sp.wrapping_add(0x20));
    c.lq(23, sp.wrapping_add(0x10));
    c.lq(30, sp);
    c.g[SP][0] = sx(sp.wrapping_add(0x90));
    abi::ret(c);
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check, scramble, Rng, GAME_GP};

    /// Static init of the visual-effects colour table: runs the guard paths
    /// (priority != 0xffff, initialize == 0) and the full construction, with
    /// random literal pool words.
    #[test]
    fn visual_effects_static_init_matches() {
        check(
            ssx3_ee::s09::f_002d40c0__generated,
            visual_effects_static_init,
            300,
            &|c, seed| {
                let mut r = Rng(0x2d40_c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                let gp = GAME_GP;
                let mut off = 0x3e4cu32;
                while off >= 0x3d3c {
                    let v = r.next_u32();
                    c.bus.write32(gp.wrapping_sub(off), v);
                    off -= 4;
                }
                c.bus.write32(gp.wrapping_add(VISUAL_EFFECTS_SLOT_GP_OFF), r.next_u32());
                c.g[A0][0] = if seed % 5 == 1 { 0 } else { sx(1 + seed) };
                c.g[A1][0] = if seed % 7 == 2 { sx(r.next_u32()) } else { 0xffff };
                            },
        );
    }
}
