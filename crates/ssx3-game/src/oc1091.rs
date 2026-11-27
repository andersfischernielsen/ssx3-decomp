//! Staging module oc1091 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1091`.

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
    (0x003b_7e38, sndstrm_open_ex, "SNDSTRM_OpenEx"),
    (0x003b_8010, sndstrm_set_fx_enable, "SNDSTRM_SetFxEnable"),
];

/// Offset of the channel handle (`shandle`) in a stream object.
const OC1091_STREAM_SHANDLE: u32 = 0x4;
/// Offset of the stream's dry level byte (`po.drylevel`).
const OC1091_STREAM_DRYLEVEL: u32 = 0xf0;

/// `SNDSTRM_OpenEx(playOpts, maxRequests, maxChunks, mem, memSize, handle)`
/// (0x3b7e38): `SNDSTRM_Open(a1, a2, a3, t0, t1, a0, 1)`, the caller's `a0`
/// moving to the last argument; returns what `SNDSTRM_Open` returned.
pub fn sndstrm_open_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[T2][0] = 0x1;
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[T1][0] = c.g[V0][0];
        Callee {
            addr: 0x003b_7450,
            f: ee::s13::f_003b7450,
        }
        .call(c, 0x003b_7e64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SNDSTRM_SetFxEnable(h, enable)` (0x3b8010): -8 if stream `h` does not
/// exist; else `enable` becomes the stream's dry level (byte +0xf0),
/// `SND_SetFxEnable(shandle, enable)` switches its effect, and the result is 0.
pub fn sndstrm_set_fx_enable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Callee {
            addr: 0x003b_6a38,
            f: ee::s13::f_003b6a38,
        }
        .call(c, 0x003b_8024)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        } else {
            c.cycles += 5;
            let stream = c.g[V0][0] as u32;
            let shandle = Addr(stream.wrapping_add(OC1091_STREAM_SHANDLE)).lw(c);
            c.g[A0][0] = shandle;
            let enable = c.g[S0][0];
            c.g[A1][0] = enable;
            Addr(stream.wrapping_add(OC1091_STREAM_DRYLEVEL)).sb(c, enable);
            Callee {
                addr: 0x003b_8700,
                f: ee::s13::f_003b8700,
            }
            .call(c, 0x003b_803c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use crate::sound::snd;

    const SEEDS: u32 = 300;
    /// Written at the gate words of the two fakes to turn them on.
    const OC1091_FAKE_MAGIC: u32 = 0x1091_c0de;
    /// Gate word of the `SNDSTRM_Open` fake, and its argument log.
    const OC1091_OPEN_GATE: u32 = DATA + 0x20_1000;
    const OC1091_OPEN_LOG: u32 = DATA + 0x20_1100;
    /// Gate word of the `SND_SetFxEnable` fake, and its argument log.
    const OC1091_FX_GATE: u32 = DATA + 0x20_1200;
    const OC1091_FX_LOG: u32 = DATA + 0x20_1300;
    /// Stream objects for the `SNDSTRM_GetStream` table (64 of them).
    const OC1091_OBJ: u32 = DATA + 0x2000;

    /// `SNDSTRM_Open` stand-in: logs its seven arguments and returns a value
    /// derived from `t1`; the generated body runs when the gate is clear.
    fn oc1091_fake_snd_open(c: &mut Cpu) {
        if c.bus.read32(OC1091_OPEN_GATE) != OC1091_FAKE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1091_fake_snd_open),
                ssx3_ee::s13::f_003b7450__generated,
            );
        }
        for (i, reg) in [A0, A1, A2, A3, T0, T1, T2].into_iter().enumerate() {
            let arg = c.g[reg][0] as u32;
            c.bus.write32(OC1091_OPEN_LOG + 4 * i as u32, arg);
        }
        let t1 = c.g[T1][0] as u32;
        c.g[V0][0] = sx(0x0b00_0000 ^ t1);
        ret(c);
    }

    /// `SND_SetFxEnable` stand-in: logs its two arguments.
    fn oc1091_fake_set_fx_enable(c: &mut Cpu) {
        if c.bus.read32(OC1091_FX_GATE) != OC1091_FAKE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1091_fake_set_fx_enable),
                ssx3_ee::s13::f_003b8700__generated,
            );
        }
        let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        c.bus.write32(OC1091_FX_LOG, a0);
        c.bus.write32(OC1091_FX_LOG + 4, a1);
        ret(c);
    }

    /// Hooks both fakes into their override slots (once per process; each
    /// runs its generated body unless its own gate word is set).
    fn oc1091_install_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ssx3_ee::s13::OVERRIDE_003B7450),
            oc1091_fake_snd_open,
            fakes::prev_cell(oc1091_fake_snd_open),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ssx3_ee::s13::OVERRIDE_003B8700),
            oc1091_fake_set_fx_enable,
            fakes::prev_cell(oc1091_fake_set_fx_enable),
        );
    }

    #[test]
    fn sndstrm_open_ex_matches() {
        oc1091_install_fakes();
        check(
            ssx3_ee::s13::f_003b7e38__generated,
            sndstrm_open_ex,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x1091_0001 ^ ((seed as u64) << 20));
                scramble(c, &mut r);
                c.bus.write32(OC1091_OPEN_GATE, OC1091_FAKE_MAGIC);
            },
        );
    }

    #[test]
    fn sndstrm_set_fx_enable_matches() {
        oc1091_install_fakes();
        check(
            ssx3_ee::s13::f_003b8010__generated,
            sndstrm_set_fx_enable,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x1091_0002 ^ ((seed as u64) << 20));
                scramble(c, &mut r);
                c.bus.write32(OC1091_FX_GATE, OC1091_FAKE_MAGIC);
                // Zero streams on every 7th seed: the -8 path.
                let count = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x3f
                };
                c.bus.write8(snd::STREAM_COUNT.0, count as u8);
                for k in 0..0x40u32 {
                    let obj = OC1091_OBJ + k * 0x100;
                    c.bus.write32(snd::STREAMS.0 + 4 * k, obj);
                    c.bus.write32(obj + OC1091_STREAM_SHANDLE, r.next_u32());
                }
                let h = match seed % 4 {
                    0 => r.next_u32(),
                    1 => 0u32.wrapping_sub(r.next_u32() % 4),
                    _ => r.next_u32() % count.max(1),
                };
                c.g[A0][0] = sx(h);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
