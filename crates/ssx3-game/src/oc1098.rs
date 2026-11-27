//! Staging module oc1098 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1098`.

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
    (0x003b_8290, snd_fx_flags_to_mask, "SND_FxFlagsToMask"),
    (0x003b_82b8, snd_set_fx, "SND_SetFx"),
    (0x003b_83a0, snd_set_fx_send, "SND_SetFxSend"),
    (0x003b_84e0, snd_set_fx_volume, "SND_SetFxVolume"),
    (0x003b_8700, snd_set_fx_enable, "SND_SetFxEnable"),
    (0x003b_8a38, snd_stop_all, "SND_StopAll"),
    (0x003b_c968, snd_dma_to_spr, "SNDENC_DmaToSpr"),
    (0x003b_ca28, snd_dma_from_spr, "SNDENC_DmaFromSpr"),
];

/// `u32 SND_FxFlagsToMask(u32 flags)` (0x3b8290): the mask bits of the
/// requested bus kinds: 0x10 -> 0x4 (with 0x8 when 0x40 is set), 0x20 -> 0x100.
/// Returns in v0; a0, a1, a2, v1 keep their scratch values.
pub fn snd_fx_flags_to_mask(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = c.g[A0][0] >> 3;
    c.g[A2][0] = c.g[A0][0] & 0x10;
    c.g[V0][0] = c.g[V0][0] & 0x8;
    c.g[A0][0] = c.g[A0][0] & 0x20;
    c.g[V1][0] = c.g[V0][0] | 0x4;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A1][0] = c.g[V0][0] | 0x100;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    ret(c);
}

/// `int SND_SetFx(flags, busLevel, c, d, e)` (0x3b82b8): when no bus kind
/// bit is given, all of 0x1f8 is taken. Each kind bit of the mask gets its
/// record from `SND_GetFxParams`, which takes the three words; then
/// `SND_SetFxSend` sets the send level. Returns 0.
pub fn snd_set_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 18, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 20, 0x40);
        c.g[V0][0] = c.g[S2][0] & 0x1f8;
        frame.save(c, 21, 0x30);
        c.g[S2][0] = c.g[S2][0] | 0x1f8;
        frame.save(c, 22, 0x20);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A0][0];
        }
        frame.save(c, 23, 0x10);
        frame.save(c, 16, 0x80);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, 17, 0x70);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, 19, 0x50);
        c.g[S6][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = 0x4;
        Callee {
            addr: 0x003b_8290,
            f: ee::s13::f_003b8290,
        }
        .call(c, 0x003b_830c)?;
        c.g[V1][0] = 0xffff_ffff_ffff_fe07;
        c.g[S0][0] = c.g[V0][0];
        c.g[S3][0] = c.g[S2][0] & c.g[V1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.g[S1][0] & c.g[S0][0];
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if !t {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x003b_8218,
                        f: ee::s13::f_003b8218,
                    }
                    .call(c, 0x003b_8334)?;
                    c.g[A0][0] = c.g[S3][0];
                    let rec = c.g[V0][0] as u32;
                    let v = c.g[S5][0];
                    Addr(rec.wrapping_add(0x8)).sw(c, v);
                    c.g[A1][0] = c.g[S1][0];
                    let v = c.g[S4][0];
                    Addr(rec).sh(c, v);
                    let v = c.g[S6][0];
                    Addr(rec.wrapping_add(0x4)).sw(c, v);
                    Callee {
                        addr: 0x003c_36a8,
                        f: ee::s13::f_003c36a8,
                    }
                    .call(c, 0x003b_834c)?;
                    c.g[V1][0] = !(c.g[S1][0]);
                    c.g[S0][0] = c.g[S0][0] & c.g[V1][0];
                }
                c.cycles += 2;
                let t = c.g[S0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_8320);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        if c.g[S4][0] != 0 {
            c.g[A1][0] = c.g[S7][0];
        }
        Callee {
            addr: 0x003b_83a0,
            f: ee::s13::f_003b83a0,
        }
        .call(c, 0x003b_836c)?;
        frame.restore(c, 16, 0x80);
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SetFxSend(flags, busLevel, c, d)` (0x3b83a0): the bus kinds
/// named in the mask (0x10, 0x20, 0x40) write the level byte into the
/// sound globals' record (`sndgs` at 0x510000-0x5718, stride 0x14); the 0x40
/// kind also pushes the level through `SND_SetFxVolume` (panned pair
/// `busLevel * (1/127)`). Under the lock, each voice gets `SND_SetAuxSend`.
pub fn snd_set_fx_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = 0xffff_ffff_ffff_fe07;
        frame.save(c, 18, 0x30);
        c.g[S2][0] = c.g[A2][0] & c.g[V0][0];
        c.g[V1][0] = c.g[A2][0] & 0x1f8;
        c.g[A2][0] = c.g[A2][0] | 0x1f8;
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        if c.g[V1][0] != 0 {
            c.g[A2][0] = c.g[A0][0];
        }
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.g[A2][0] & 0x10;
        let t = c.g[V0][0] == 0;
        c.g[S3][0] = 0x0051_0000;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = 0x14;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
            c.mult(0, 2, 18, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let v = c.g[S0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x1fa)).sb(c, v);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0] & 0x20;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 0x14;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
            c.mult(0, 2, 18, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let v = c.g[S0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x20e)).sb(c, v);
        }
        c.cycles += 3;
        c.g[S1][0] = c.g[A2][0] & 0x40;
        let t = c.g[S1][0] == 0;
        c.g[V0][0] = 0x14;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
            c.mult(0, 2, 18, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let v = c.g[S0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x222)).sb(c, v);
        }
        c.cycles += 4;
        Callee {
            addr: 0x003b_58a0,
            f: ee::s13::f_003b58a0,
        }
        .call(c, 0x003b_8434)?;
        let t = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !t {
            c.cycles += 12;
            c.mtc1(16, 0);
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0x3c01_0000;
            c.g[AT][0] = c.g[AT][0] | 0x204;
            c.mtc1(1, 1);
            c.g[A1][0] = 0x8;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.mul_s(0, 0, 1);
            let s = sp(c);
            c.swc1(0, s.wrapping_add(0x4));
            let s = sp(c);
            c.swc1(0, s);
            Callee {
                addr: 0x003b_84e0,
                f: ee::s13::f_003b84e0,
            }
            .call(c, 0x003b_846c)?;
        }
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x17e)).lh(c);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[S3][0] = c.g[V1][0];
            c.g[S1][0] = 0;
            c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1f0)).lw(c);
            loop {
                c.cycles += 12;
                c.g[A1][0] = c.g[S2][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x61)).lb(c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                Callee {
                    addr: 0x003b_88c8,
                    f: ee::s13::f_003b88c8,
                }
                .call(c, 0x003b_84a8)?;
                c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x17e)).lh(c);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1f0)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_8488);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 10;
        Callee {
            addr: 0x003b_58d8,
            f: ee::s13::f_003b58d8,
        }
        .call(c, 0x003b_84c0)?;
        frame.restore(c, 16, 0x50);
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SetFxVolume(busIndex, flags, c, panPair*)` (0x3b84e0): with
/// flags bit 0x8, the 8-byte pan pair is copied into the bus record
/// (`D_0050AB14`, stride 0x14). Then `SND_SetSpuFxVolume` runs with the same
/// arguments.
pub fn snd_set_fx_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0] & 0x8;
        let t = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !t {
            c.cycles += 9;
            c.g[V1][0] = 0x14;
            c.g[V0][0] = 0x0051_0000;
            c.mult(0, 3, 4, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x54ec));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[A3][0] as u32);
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[V1][0] as u32);
        }
        c.cycles += 6;
        Callee {
            addr: 0x003c_36c8,
            f: ee::s13::f_003c36c8,
        }
        .call(c, 0x003b_851c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetFxEnable(handle, enable)` (0x3b8700): every channel of
/// `SND_HandleToChannel(handle)` gets `enable` at its byte +0x5f and
/// `SND_ChannelApplyFxEnable`; the channels come from `SND_NextHandleChannel`
/// one at a time. Returns the handle's channel (negative if none).
///
/// The loop re-enters at its head after an unwound event, so the entry is
/// the only `pc` the replacement runs; any resume runs the generated body.
pub fn snd_set_fx_enable(c: &mut Cpu) {
    if c.pc != 0x003b_8700 {
        return ee::s13::f_003b8700__generated(c);
    }
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        Callee {
            addr: 0x003b_a6e8,
            f: ee::s13::f_003ba6e8,
        }
        .call(c, 0x003b_8720)?;
        c.g[S0][0] = c.g[V0][0];
        let negative = (c.g[S0][0] as i64) < 0;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        if !negative {
            c.cycles += 5;
            c.g[V0][0] = 0x0051_0000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
            let v = c.g[V1][0];
            Addr(sp(c)).sw(c, v);
            c.g[S2][0] = 0x8c;
            // Enters at the next-channel call (0x3b8758), then the body at
            // 0x3b8740 and back again while a channel is left.
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003b_8764);
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x003b_aa00,
                    f: ee::s13::f_003baa00,
                }
                .call(c, 0x003b_8764)?;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(sp(c)).lw(c);
                if !t {
                    break;
                }
                if crate::events::back_edge_unwinds(c, 0x003b_8740) {
                    return Ok(());
                }
                c.cycles += 6;
                c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1f0)).lw(c);
                c.mult(0, 2, 2, 18);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let v = c.g[S1][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x5f)).sb(c, v);
                c.g[A0][0] = Addr(sp(c)).lw(c);
                Callee {
                    addr: 0x003c_2ec8,
                    f: ee::s13::f_003c2ec8,
                }
                .call(c, 0x003b_8758)?;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x40);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_StopAll(void)` (0x3b8a38): under the lock, `SND_Stop` for each
/// voice of the sound globals (`voicestotal`, the channel table).
pub fn snd_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        Callee {
            addr: 0x003b_58a0,
            f: ee::s13::f_003b58a0,
        }
        .call(c, 0x003b_8a54)?;
        c.g[V0][0] = 0x0051_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x17e)).lh(c);
        let t = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = c.g[V0][0];
        if !t {
            c.cycles += 2;
            c.g[S1][0] = 0;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x1f0)).lw(c);
            loop {
                c.cycles += 9;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                Callee {
                    addr: 0x003b_89e0,
                    f: ee::s13::f_003b89e0,
                }
                .call(c, 0x003b_8a84)?;
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x17e)).lh(c);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V1][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x1f0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003b_8a70);
                }
            }
        }
        c.cycles += 8;
        Callee {
            addr: 0x003b_58d8,
            f: ee::s13::f_003b58d8,
        }
        .call(c, 0x003b_8a9c)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SNDENC_DmaToSpr(memAddr, sprAddr, qwordCount, flags)` (0x3bc968):
/// waits for the to-SPR channel (0x1000d400) to go idle, starts it with the
/// three addresses and 0x100, and, with flags bit 0x10000, waits again.
pub fn snd_dma_to_spr(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x1000_0000;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] | 0xd400;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = c.g[V1][0] & 0x100;
    let t = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !t {
        c.cycles += 1;
        c.g[V1][0] = c.g[V1][0] | 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_c988);
            }
        }
    }
    c.cycles += 17;
    c.g[V0][0] = 0x1000_0000;
    c.g[A0][0] = 0x1000_0000;
    c.g[V0][0] = c.g[V0][0] | 0xd410;
    c.g[A0][0] = c.g[A0][0] | 0xd480;
    let v = c.g[T0][0];
    Addr(c.g[V0][0] as u32).sw(c, v);
    c.g[V1][0] = 0x1000_0000;
    let v = c.g[A1][0];
    Addr(c.g[A0][0] as u32).sw(c, v);
    c.g[V1][0] = c.g[V1][0] | 0xd420;
    let v = c.g[A2][0];
    Addr(c.g[V1][0] as u32).sw(c, v);
    c.g[A0][0] = 0x1000_0000;
    c.g[A0][0] = c.g[A0][0] | 0xd400;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x10000;
    let v = c.g[V1][0];
    Addr(c.g[A0][0] as u32).sw(c, v);
    c.g[V0][0] = c.g[A3][0] & c.g[V0][0];
    let t = c.g[V0][0] == 0;
    if !t {
        c.cycles += 5;
        c.g[V0][0] = 0x1001_0000;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x2c00)).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x100;
        let t = c.g[V0][0] == 0;
        c.g[V1][0] = 0x1000_0000;
        if !t {
            c.cycles += 1;
            c.g[V1][0] = c.g[V1][0] | 0xd400;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_ca00);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void SNDENC_DmaFromSpr(sprAddr, memAddr, qwordCount, flags)` (0x3bca28):
/// the from-SPR channel (0x1000d000) counterpart of [`snd_dma_to_spr`].
pub fn snd_dma_from_spr(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x1000_0000;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = c.g[V1][0] & 0x100;
    let t = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !t {
        c.cycles += 1;
        c.g[V1][0] = c.g[V1][0] | 0xd000;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_ca48);
            }
        }
    }
    c.cycles += 17;
    c.g[V0][0] = 0x1000_0000;
    c.g[A0][0] = 0x1000_0000;
    c.g[V0][0] = c.g[V0][0] | 0xd010;
    c.g[A0][0] = c.g[A0][0] | 0xd080;
    let v = c.g[A1][0];
    Addr(c.g[V0][0] as u32).sw(c, v);
    c.g[V1][0] = 0x1000_0000;
    let v = c.g[T0][0];
    Addr(c.g[A0][0] as u32).sw(c, v);
    c.g[V1][0] = c.g[V1][0] | 0xd020;
    let v = c.g[A2][0];
    Addr(c.g[V1][0] as u32).sw(c, v);
    c.g[A0][0] = 0x1000_0000;
    c.g[A0][0] = c.g[A0][0] | 0xd000;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x10000;
    let v = c.g[V1][0];
    Addr(c.g[A0][0] as u32).sw(c, v);
    c.g[V0][0] = c.g[A3][0] & c.g[V0][0];
    let t = c.g[V0][0] == 0;
    if !t {
        c.cycles += 5;
        c.g[V0][0] = 0x1001_0000;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x3000)).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x100;
        let t = c.g[V0][0] == 0;
        c.g[V1][0] = 0x1000_0000;
        if !t {
            c.cycles += 1;
            c.g[V1][0] = c.g[V1][0] | 0xd000;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_cac0);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA};

    const SEEDS: u32 = 200;
    const SALT: u64 = 0x1098_0000;
    /// Sound globals (`sndgs`) at 0x510000-0x5718.
    const OC1098_SNDGS: u32 = 0x0051_0000 - 0x5718;
    /// Written at the gate word to turn the stand-ins on.
    const OC1098_MAGIC: u32 = 0x1098_c0de;
    /// Gate word; mailbox words (slot i at `+4*i`, slot 8: NextHandle count).
    const OC1098_GATE: u32 = DATA + 0x2e_0000;
    const OC1098_BOX: u32 = DATA + 0x2e_0010;
    /// Channel records (stride 0x8c), the GetFxParams record, 8 pan bytes.
    const OC1098_CHANS: u32 = DATA + 0x2e_1000;
    const OC1098_OBJ: u32 = DATA + 0x2e_3000;
    const OC1098_PAN: u32 = DATA + 0x2e_4000;
    /// The DMA memory buffer (to and from the SPR).
    const OC1098_DMA_MEM: u32 = DATA + 0x2e_5000;

    /// Logs a call like the machine's miss hook; the result (when the gate
    /// is on) is the sign-extended mailbox word `slot`.
    macro_rules! oc1098_stand_in {
        ($fake:ident, $target:expr, $slot:expr, $generated:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1098_GATE) != OC1098_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
                if c.pc == ra {
                    c.g[V0][0] = sx(c.bus.read32(OC1098_BOX + 4 * $slot));
                }
            }
        };
    }

    oc1098_stand_in!(oc1098_lock, 0x003b_58a0, 15, ee::s13::f_003b58a0__generated);
    oc1098_stand_in!(
        oc1098_unlock,
        0x003b_58d8,
        15,
        ee::s13::f_003b58d8__generated
    );
    oc1098_stand_in!(oc1098_stop, 0x003b_89e0, 15, ee::s13::f_003b89e0__generated);
    oc1098_stand_in!(
        oc1098_aux_send,
        0x003b_88c8,
        15,
        ee::s13::f_003b88c8__generated
    );
    oc1098_stand_in!(
        oc1098_spu_volume,
        0x003c_36c8,
        15,
        ee::s13::f_003c36c8__generated
    );
    oc1098_stand_in!(
        oc1098_fx_params,
        0x003b_8218,
        0,
        ee::s13::f_003b8218__generated
    );
    oc1098_stand_in!(
        oc1098_group_effect,
        0x003c_36a8,
        15,
        ee::s13::f_003c36a8__generated
    );
    oc1098_stand_in!(
        oc1098_handle_to_channel,
        0x003b_a6e8,
        1,
        ee::s13::f_003ba6e8__generated
    );
    oc1098_stand_in!(
        oc1098_apply_fx_enable,
        0x003c_2ec8,
        15,
        ee::s13::f_003c2ec8__generated
    );

    /// `SND_NextHandleChannel` stand-in: returns 1 while the count at slot 8
    /// is non-zero (one channel per count), then 0.
    fn oc1098_next_handle(c: &mut Cpu) {
        if c.bus.read32(OC1098_GATE) != OC1098_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1098_next_handle),
                ee::s13::f_003baa00__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003b_aa00, ra);
        if c.pc == ra {
            let left = c.bus.read32(OC1098_BOX + 8);
            c.bus.write32(OC1098_BOX + 8, left.saturating_sub(1));
            c.g[V0][0] = (left > 0) as u64;
        }
    }

    /// Install the stand-ins in their override slots (once per process).
    fn oc1098_hook() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1098_lock,
            fakes::prev_cell(oc1098_lock),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1098_unlock,
            fakes::prev_cell(oc1098_unlock),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B89E0),
            oc1098_stop,
            fakes::prev_cell(oc1098_stop),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B88C8),
            oc1098_aux_send,
            fakes::prev_cell(oc1098_aux_send),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C36C8),
            oc1098_spu_volume,
            fakes::prev_cell(oc1098_spu_volume),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8218),
            oc1098_fx_params,
            fakes::prev_cell(oc1098_fx_params),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C36A8),
            oc1098_group_effect,
            fakes::prev_cell(oc1098_group_effect),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
            oc1098_handle_to_channel,
            fakes::prev_cell(oc1098_handle_to_channel),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C2EC8),
            oc1098_apply_fx_enable,
            fakes::prev_cell(oc1098_apply_fx_enable),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BAA00),
            oc1098_next_handle,
            fakes::prev_cell(oc1098_next_handle),
        );
    }

    /// Gate on, GetFxParams returns its record, HandleToChannel returns a
    /// channel (negative on every fourth seed), NextHandle yields `seed % 3`
    /// channels, and `voices` voices with their handles in the table.
    fn oc1098_world(c: &mut Cpu, seed: u32, r: &mut Rng, voices: u16) {
        c.bus.write32(OC1098_GATE, OC1098_MAGIC);
        c.bus.write32(OC1098_BOX, OC1098_OBJ);
        let handle = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 4
        };
        c.bus.write32(OC1098_BOX + 4, handle);
        c.bus.write32(OC1098_BOX + 8, seed % 3);
        c.bus.write16(OC1098_SNDGS + 0x17e, voices);
        c.bus.write32(OC1098_SNDGS + 0x1f0, OC1098_CHANS);
        for i in 0..4 {
            c.bus.write32(OC1098_CHANS + 0x8c * i, r.next_u32());
        }
    }

    /// A busIndex, flags and FX mask small enough to address the bus records.
    fn oc1098_fx_args(c: &mut Cpu, seed: u32, r: &mut Rng) {
        // Bits 0x1f8 pick the bus kinds; the low 3 bits keep the index small.
        let kinds = if seed % 5 == 0 {
            0
        } else {
            r.next_u32() & 0x1f8
        };
        c.g[A0][0] = (kinds | (seed % 8)) as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = r.next_u32() as u64;
    }

    #[test]
    fn snd_fx_flags_to_mask_matches() {
        check(
            ee::s13::f_003b8290__generated,
            snd_fx_flags_to_mask,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT, 0);
                c.g[A0][0] = (r.next_u32() & !0x1f8 | (seed & 0x3f) << 3) as u64;
            },
        );
    }

    #[test]
    fn snd_set_fx_matches() {
        oc1098_hook();
        check(
            ee::s13::f_003b82b8__generated,
            snd_set_fx,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 1, 0);
                oc1098_world(c, seed, &mut r, (seed % 4) as u16);
                oc1098_fx_args(c, seed, &mut r);
                c.pc = 0x003b_82b8;
            },
        );
    }

    #[test]
    fn snd_set_fx_send_matches() {
        oc1098_hook();
        check(
            ee::s13::f_003b83a0__generated,
            snd_set_fx_send,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 2, 0);
                oc1098_world(c, seed, &mut r, (seed % 4) as u16);
                oc1098_fx_args(c, seed, &mut r);
                c.pc = 0x003b_83a0;
            },
        );
    }

    #[test]
    fn snd_set_fx_volume_matches() {
        oc1098_hook();
        check(
            ee::s13::f_003b84e0__generated,
            snd_set_fx_volume,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 3, 0);
                c.bus.write32(OC1098_PAN, r.next_u32());
                c.bus.write32(OC1098_PAN + 4, r.next_u32());
                c.g[A0][0] = (seed % 8) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = OC1098_PAN as u64;
                c.pc = 0x003b_84e0;
            },
        );
    }

    #[test]
    fn snd_set_fx_enable_matches() {
        oc1098_hook();
        check(
            ee::s13::f_003b8700__generated,
            snd_set_fx_enable,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 4, 0);
                oc1098_world(c, seed, &mut r, 2);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (seed & 1) as u64;
                c.pc = 0x003b_8700;
            },
        );
    }

    #[test]
    fn snd_stop_all_matches() {
        oc1098_hook();
        check(
            ee::s13::f_003b8a38__generated,
            snd_stop_all,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 5, 0);
                oc1098_world(c, seed, &mut r, (seed % 4) as u16);
                c.pc = 0x003b_8a38;
            },
        );
    }

    /// DMA setup: the channel idle, the buffer in RAM, a small transfer and
    /// flags with or without the wait-after bit (0x10000). The hardware
    /// model finishes the transfer when the start bit is written, so the
    /// waits end.
    fn oc1098_dma_args(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.bus.write32(0x1000_d400, 0);
        c.bus.write32(0x1000_d000, 0);
        c.g[A0][0] = OC1098_DMA_MEM as u64;
        c.g[A1][0] = ((seed * 16) % 0x3000) as u64;
        c.g[A2][0] = (seed % 4) as u64;
        c.g[A3][0] =
            (r.next_u32() & !0x0001_0000) as u64 | if seed % 2 == 0 { 0x0001_0000 } else { 0 };
    }

    #[test]
    fn snd_dma_to_spr_matches() {
        check(
            ee::s13::f_003bc968__generated,
            snd_dma_to_spr,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 6, 0);
                oc1098_dma_args(c, seed, &mut r);
                c.pc = 0x003b_c968;
            },
        );
    }

    #[test]
    fn snd_dma_from_spr_matches() {
        check(
            ee::s13::f_003bca28__generated,
            snd_dma_from_spr,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, SALT + 7, 0);
                oc1098_dma_args(c, seed, &mut r);
                c.g[A0][0] = ((seed * 16) % 0x3000) as u64;
                c.g[A1][0] = OC1098_DMA_MEM as u64;
                c.pc = 0x003b_ca28;
            },
        );
    }
}
