//! Staging module oc792 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc792`.

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
    (0x003db790, spch_seed_random, "SPCH_SeedRandom"),
    (
        0x003d3058,
        path_calc_track_mem_size,
        "PATH_CalcTrackMemSize",
    ),
    (0x002a7718, crowd_audio_stop_all, "CrowdAudio_StopAll"),
    (0x003d6778, spch_calc_buffer_size, "SPCH_CalcBufferSize"),
    (
        0x001a0420,
        fe_module_destroy_rider_previews,
        "FEModule_DestroyRiderPreviews",
    ),
    (0x003b7eb0, sndstrm_fade_volume, "SNDSTRM_FadeVolume"),
];

/// `void SPCH_SeedRandom(int seed)` (0x3db790): fills the 32 u16 pairs at
/// 0x450b88 with 0xffff, then derives the five random-generator state words
/// at 0x450b70..0x450b84 from `seed` (fixed constant sums) and clears the
/// word at `gp+0x1768`.
pub fn spch_seed_random(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x450000;
    c.g[A2][0] = 0x450000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb88));
    c.g[A1][0] = 0xffff;
    c.g[V1][0] = 0x1f;
    loop {
        c.cycles += 7;
        c.sh(c.g[V0][0] as u32, c.g[A1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sh((c.g[V0][0] as u32).wrapping_add(0x2), c.g[A1][0]);
        let t = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x003db7a8);
            }
            continue;
        }
        break;
    }
    c.cycles += 29;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 16);
    c.g[V0][0] = 0xfffffffff22d0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] |= 0xe56;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = 0xffffffff96040000;
    c.sw((c.g[A2][0] as u32).wrapping_add(0xb70), c.g[V0][0]);
    c.g[V1][0] |= 0x1893;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb70));
    c.g[V1][0] = 0x3df30000;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] |= 0xb646;
    c.g[A0][0] = 0x40dd0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] |= 0xe76d;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.g[A1][0] = 0xffffffff97320000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A1][0] |= 0x7ae1;
    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
    c.g[V1][0] = 0xffffffffd1a90000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] |= 0xfbe7;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1768), 0u64);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x14), c.g[V0][0]);
    c.pc = t;
}

/// `int PATH_CalcTrackMemSize(int n)` (0x3d3058): `PATH_PlatCalcTrackMemSize()`
/// (0x2ae478) plus `0x110 + 16 * n` (a1 = base, rounded up to 16 as a signed
/// value) plus 0x10.
pub fn path_calc_track_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x003d306c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002ae478, 0x003d306c)?;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x110));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// `void CrowdAudio_StopAll(cCrowdSys* self)` (0x2a7718): removes all sources of
/// each of the `self+0x14` crowd instances (0x190 bytes each, from `self+0x18`),
/// then calls the stop callback in the sound manager's (`*self+0xab0`) vtable-like
/// entry (`+0x1c`, `this` adjusted by the s16 at `+0x18`) on `*self+0x1d8`.
pub fn crowd_audio_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !t {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.mtc1(0, 12);
                c.g[RA][0] = sx(0x002a7754);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                Callee {
                    addr: 0x002a6430,
                    f: ee::s08::f_002a6430,
                }
                .call(c, 0x002a7754)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x190));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002a7740);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = 0x4a0000;
        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x3707));
        c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3700));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xab0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x002a778c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, t, 0x002a778c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

/// `int SPCH_CalcBufferSize(int count, int unit, int mode)` (0x3d6778): with
/// `n = count * unit` (a0 = a0 * a1) and `v1 = (n + 7 if n <= -1 else n) >> 3`
/// (n rounded towards zero over 8): mode 1 returns `v1 / 10`, mode 2
/// `(2 * v1) / 7`, every other mode `v1`.
pub fn spch_calc_buffer_size(c: &mut Cpu) {
    c.cycles += 8;
    c.mult(0, 4, 4, 5);
    c.g[V1][0] = 0xffffffffffffffff;
    c.g[A1][0] = 0x1;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    let mode_is_one = c.g[A2][0] == c.g[A1][0];
    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
    let mut done = false;
    if !mode_is_one {
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[V0][0] = 0x2;
        let mut to_e8 = false;
        if t {
            to_e8 = true;
        } else {
            c.cycles += 2;
            let t = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if t {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
                c.g[V1][0] = 0x7;
                c.div(0, 2, 3);
                c.cycles += 2;
                c.g[V0][0] = c.lo[0];
                c.g[V1][0] = c.g[V0][0];
                to_e8 = true;
            } else {
                c.cycles += 2;
                done = true;
            }
        }
        if to_e8 {
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
            done = true;
        }
        if done {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.pc = t;
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = 0xa;
    c.div(0, 3, 2);
    c.cycles += 3;
    c.g[V1][0] = c.lo[0];
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[V1][0];
    c.pc = t;
}

/// `void FEModule_DestroyRiderPreviews(FEModule* self, int flags)` (0x1a0420):
/// unloads the rider previews, runs the destructor on the two previews at
/// `self+0x19c0-0xce0` and `self+0xce0`/`self+0` (0xce0 bytes each, last to
/// first; nothing for a null `self`), and deletes `self` when `flags & 1`.
pub fn fe_module_destroy_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001a0440);
        c.g[S2][0] = c.g[A1][0];
        Callee {
            addr: 0x001a04c8,
            f: ee::s03::f_001a04c8,
        }
        .call(c, 0x001a0440)?;
        let null = c.g[S1][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c0));
        // Skipping straight to the flag test (no `+1` block charge) happens
        // only when `self + 0x19c0 == self`; it cannot, but keep the shape.
        let mut skip_charge = false;
        if !null {
            c.cycles += 2;
            let same = c.g[S1][0] == c.g[S0][0];
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if same {
                skip_charge = true;
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xce0));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[RA][0] = sx(0x001a0464);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0019e498,
                        f: ee::s03::f_0019e498,
                    }
                    .call(c, 0x001a0464)?;
                    let t = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xce0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001a0458);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
        if !skip_charge {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a0480);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00317e50, 0x001a0480)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

/// `int SNDSTRM_FadeVolume(int stream, int fadeTime, int targetVol)` (0x3b7eb0):
/// looks the stream up; no stream returns -8; else stores the target volume
/// byte at `stream+0xec`, starts a fade of the stream's voice (`+4`) over
/// `fadeTime / 10` and returns 0.
pub fn sndstrm_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x003b7ecc);
        c.g[S1][0] = c.g[A2][0];
        Callee {
            addr: 0x003b6a38,
            f: ee::s13::f_003b6a38,
        }
        .call(c, 0x003b7ecc)?;
        c.g[V1][0] = c.g[V0][0];
        let t = c.g[V1][0] == 0;
        c.g[V0][0] = 0xa;
        if t {
            c.cycles += 1;
            c.g[V0][0] = 0xfffffffffffffff8;
        } else {
            c.cycles += 4;
            c.sb((c.g[V1][0] as u32).wrapping_add(0xec), c.g[S1][0]);
            c.div(0, 16, 2);
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = c.lo[0];
            c.g[RA][0] = sx(0x003b7ef8);
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x003b8530,
                f: ee::s13::f_003b8530,
            }
            .call(c, 0x003b7ef8)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc792's callee fakes in a machine's RAM.
    pub(crate) const OC792_CTRL: u32 = DATA + 0x34_0000;
    pub(crate) const OC792_MAGIC: u32 = 0x0792_f4ce;
    /// The object the tests point their first argument at.
    pub(crate) const OC792_OBJ: u32 = DATA + 0x1_0000;
    pub(crate) const OC792_AUX: u32 = DATA + 0x2_0000;

    /// The mailbox word of the fake for `addr` (0: the fake's own result).
    pub(crate) fn oc792_mail(addr: u32) -> u32 {
        DATA + 0x34_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc792_arm(c: &mut Cpu) {
        c.bus.write32(OC792_CTRL, OC792_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc792_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc792_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc792_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc792_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    macro_rules! oc792_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC792_CTRL) != OC792_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc792_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc792_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc792_fakes!(
        (
            oc792_fk_2a6430,
            OC792_PREV_2A6430,
            ee::s08::OVERRIDE_002A6430,
            ee::s08::f_002a6430__generated,
            0x002a_6430
        ),
        (
            oc792_fk_2ae478,
            OC792_PREV_2AE478,
            ee::s08::OVERRIDE_002AE478,
            ee::s08::f_002ae478__generated,
            0x002a_e478
        ),
        (
            oc792_fk_1a04c8,
            OC792_PREV_1A04C8,
            ee::s03::OVERRIDE_001A04C8,
            ee::s03::f_001a04c8__generated,
            0x001a_04c8
        ),
        (
            oc792_fk_19e498,
            OC792_PREV_19E498,
            ee::s03::OVERRIDE_0019E498,
            ee::s03::f_0019e498__generated,
            0x0019_e498
        ),
        (
            oc792_fk_317e50,
            OC792_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x0031_7e50
        ),
        (
            oc792_fk_3b6a38,
            OC792_PREV_3B6A38,
            ee::s13::OVERRIDE_003B6A38,
            ee::s13::f_003b6a38__generated,
            0x003b_6a38
        ),
        (
            oc792_fk_3b8530,
            OC792_PREV_3B8530,
            ee::s13::OVERRIDE_003B8530,
            ee::s13::f_003b8530__generated,
            0x003b_8530
        ),
    );

    /// `SPCH_SeedRandom`: any seed (positive, negative, zero), random junk in
    /// the other registers; the table and the state words are compared.
    #[test]
    fn spch_seed_random_matches() {
        check(
            ee::s14::f_003db790__generated,
            spch_seed_random,
            200,
            &|c, seed| {
                let mut r = Rng(0x7a92_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003d_b790;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => sx(r.next_u32() & 0xffff),
                    _ => sx(r.next_u32()),
                };
            },
        );
    }

    /// `PATH_CalcTrackMemSize`: the platform size (fake, random results incl.
    /// negative) plus random track counts, with and without an unwind.
    #[test]
    fn path_calc_track_mem_size_matches() {
        oc792_install_fakes();
        fakes::covered(
            ee::s14::f_003d3058__generated,
            path_calc_track_mem_size,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x792a, 1);
                oc792_arm(c);
                c.pc = 0x003d_3058;
                c.g[A0][0] = sx(match seed % 3 {
                    0 => r.next_u32() & 0xff,
                    1 => r.next_u32(),
                    _ => r.next_u32() | 0xf000_0000,
                });
                if seed % 5 != 0 {
                    let v = match seed % 4 {
                        0 => r.next_u32() & 0xfff,
                        1 => r.next_u32(),
                        2 => 0xffff_fff0,
                        _ => 0x7fff_fff0,
                    };
                    oc792_force(c, 0x002a_e478, v);
                }
            },
        );
    }

    /// `CrowdAudio_StopAll`: 0 to 4 instances (the empty and negative counts
    /// too), the final callback is a fake method (`FAKE`).
    #[test]
    fn crowd_audio_stop_all_matches() {
        oc792_install_fakes();
        fakes::covered(
            ee::s08::f_002a7718__generated,
            crowd_audio_stop_all,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x792b, 6);
                oc792_arm(c);
                c.pc = 0x002a_7718;
                c.g[A0][0] = sx(OC792_OBJ);
                let count = [0u32, 1, 2, 4, 3, 0xffff_ffff][(seed % 6) as usize];
                c.bus.write32(OC792_OBJ, OC792_AUX);
                c.bus.write32(OC792_OBJ + 0x14, count);
                c.bus.write32(OC792_OBJ + 0x18, OC792_OBJ + 0x100);
                let mgr = OC792_AUX + 0x1000;
                c.bus.write32(OC792_AUX + 0xab0, mgr);
                c.bus.write16(mgr + 0x18, r.next_u32() as u16);
                c.bus.write32(mgr + 0x1c, fakes::FAKE);
                fill(c, &mut r, 0x004a_3700, 8);
            },
        );
    }

    /// `SPCH_CalcBufferSize`: all modes (0, 1, 2, other), positive, zero and
    /// negative sizes.
    #[test]
    fn spch_calc_buffer_size_matches() {
        check(
            ee::s14::f_003d6778__generated,
            spch_calc_buffer_size,
            400,
            &|c, seed| {
                let mut r = Rng(0x7a92_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003d_6778;
                c.g[A0][0] = sx(match seed % 4 {
                    0 => r.next_u32() & 0xff,
                    1 => (r.next_u32() & 0xff).wrapping_neg(),
                    2 => r.next_u32() & 0xffff,
                    _ => r.next_u32(),
                });
                c.g[A1][0] = sx(match seed % 3 {
                    0 => r.next_u32() & 0xf,
                    1 => r.next_u32() & 0xff,
                    _ => r.next_u32(),
                });
                c.g[A2][0] =
                    sx([0u32, 1, 2, 3, r.next_u32(), 0xffff_ffff][(seed / 4 % 6) as usize]);
            },
        );
    }

    /// `FEModule_DestroyRiderPreviews`: null and real `self`, flag bit 0 on and
    /// off; the unload, two destructors and the delete are fakes that may unwind.
    #[test]
    fn fe_module_destroy_rider_previews_matches() {
        oc792_install_fakes();
        fakes::covered(
            ee::s03::f_001a0420__generated,
            fe_module_destroy_rider_previews,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x792c, 4);
                oc792_arm(c);
                c.pc = 0x001a_0420;
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(OC792_OBJ) };
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `SNDSTRM_FadeVolume`: no stream (fake returns 0) and a stream record at
    /// `OC792_AUX` with random fade time and volume.
    #[test]
    fn sndstrm_fade_volume_matches() {
        oc792_install_fakes();
        fakes::covered(
            ee::s13::f_003b7eb0__generated,
            sndstrm_fade_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x792d, 2);
                oc792_arm(c);
                c.pc = 0x003b_7eb0;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(match seed % 3 {
                    0 => r.next_u32() & 0xfff,
                    1 => r.next_u32(),
                    _ => 0,
                });
                c.g[A2][0] = sx(r.next_u32());
                oc792_force(c, 0x003b_6a38, if seed % 4 == 0 { 0 } else { OC792_AUX });
                c.bus.write32(OC792_AUX + 4, r.next_u32());
            },
        );
    }
}
