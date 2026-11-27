//! Staging module oc1109 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1109`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Layout of one of the three track fade updaters (volume, aux send, fade3).
pub(crate) struct Oc1109FadeLayout {
    /// Fade start time word (`start`), `cur` byte, `tgt` byte at `cur + 2`.
    start: u32,
    cur: u32,
    /// Config word / halfword in the track table (duration = halfword).
    cfg_w: u32,
    cfg_h: u32,
    /// Platform setter and its return addresses (fade done / mid-fade).
    setter: u32,
    ret_done: u32,
    ret_set: u32,
    /// Volume fade only: extra byte at +0x34 holding the final value.
    volume: bool,
}

/// Shared body of the fade updaters: interpolate a byte from `cur` to `tgt`
/// over `duration` ticks along a curve (linear, square, inverse), call the
/// platform setter, and reset the fade when finished.
fn oc1109_fade(c: &mut Cpu, p: &Oc1109FadeLayout) -> crate::call::Flow {
    let tgt = p.cur + 2;
    c.cycles += 29;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0x450000;
    frame.save(c, S0, 0x10);
    c.g[A3][0] &= 0xffff;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(0x450000u32.wrapping_sub(0xbe0));
    c.g[A2][0] = 0xffff_ffff_ffff_ff80;
    let s0 = c.g[S0][0] as u32;
    c.g[V0][0] = c.lb(s0.wrapping_add(tgt));
    c.g[T0][0] = 1;
    c.g[V1][0] = c.lb(s0.wrapping_add(p.cur));
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 3);
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    let a0 = c.g[A0][0] as u32;
    c.g[V0][0] = c.lw(a0.wrapping_add(p.cfg_w));
    c.g[V1][0] = c.lhu(a0.wrapping_add(p.cfg_h));
    c.mtc1(0, 1);
    c.g[V0][0] &= 0x7f;
    c.sub_s(2, 2, 0);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[A3][0] |= c.g[V1][0];
    c.g[A3][0] &= c.g[A2][0];
    c.c_lt_s(2, 1);
    let ge = !c.fcc();
    c.g[A3][0] |= c.g[V0][0];
    if !ge {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    // elapsed = now - start, as an unsigned 32-bit value in f1
    c.cycles += 9;
    c.g[V0][0] = 0x450000;
    c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
    c.g[A1][0] = c.lw(0x450000u32.wrapping_sub(0xbc8));
    c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
    c.g[V0][0] = c.lw(s0.wrapping_add(p.start));
    c.g[A3][0] &= c.g[A0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[A3][0] |= c.g[V1][0];
    if !neg {
        c.cycles += 4;
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
    } else {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
    }
    // duration (unsigned 16-bit) in f0
    c.cycles += 2;
    if (c.g[V0][0] as i64) < 0 {
        c.g[V0][0] &= 1;
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[A3][0] as u32) >> 17);
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
    } else {
        c.cycles += 4;
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
    }
    c.cycles += 7;
    c.div_s(0, 1, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.c_le_s(1, 0);
    let running = !c.fcc();
    if p.volume {
        c.g[V0][0] = c.g[A3][0] & 0x80;
    } else {
        c.g[A1][0] = 0;
    }
    let s0 = c.g[S0][0] as u32;
    if !running {
        // fade finished: set the final value, clear the fade state
        if p.volume {
            c.cycles += 11;
            c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
            c.g[A1][0] = c.lb(s0.wrapping_add(0x34));
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
            c.g[A2][0] = c.lb(s0.wrapping_add(tgt));
        }
        call_ptr(c, p.setter, p.ret_done)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = u64::MAX;
        let s0 = c.g[S0][0] as u32;
        let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
        if p.volume {
            c.sb(s0.wrapping_add(0x34), v1);
            c.sb(s0.wrapping_add(p.cur), v0);
            c.sb(s0.wrapping_add(tgt), v1);
            c.sb(s0.wrapping_add(p.cur + 1), v1);
            c.sw(s0.wrapping_add(p.start), 0);
        } else {
            c.sb(s0.wrapping_add(p.cur), v0);
            c.sb(s0.wrapping_add(p.cur + 1), v1);
            c.sw(s0.wrapping_add(p.start), 0);
            c.sb(s0.wrapping_add(tgt), v1);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        return oc1109_fade_exit(c, frame);
    }
    if p.volume {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[A3][0] & 0x80;
    }
    if c.g[V0][0] != 0 {
        c.sub_s(0, 1, 0);
    }
    c.cycles += 4;
    c.g[V1][0] = c.g[A3][0] & 0x7f;
    c.g[V0][0] = 2;
    let is2 = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[V1][0] < 3) as u64;
    if is2 {
        c.cycles += 3;
        c.mul_s(0, 0, 0);
        c.sub_s(3, 1, 0);
        c.cycles += 1;
        c.g[V0][0] = c.lb(s0.wrapping_add(p.cur));
    } else {
        c.cycles += 2;
        let ge3 = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !ge3 {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.sub_s(3, 1, 0);
                c.cycles += 1;
                c.g[V0][0] = c.lb(s0.wrapping_add(p.cur));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lb(s0.wrapping_add(p.cur));
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                c.div_s(1, 1, 0);
                c.g[AT][0] = 0x41c8_0000;
                c.mtc1(AT, 0);
                c.div_s(3, 1, 0);
                c.cycles += 1;
                c.g[V0][0] = c.lb(s0.wrapping_add(p.cur));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lb(s0.wrapping_add(p.cur));
            }
        }
    }
    // f1 = min(cur, tgt), f2 = |tgt - cur|
    c.cycles += 6;
    c.g[V1][0] = c.lb(s0.wrapping_add(tgt));
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.mtc1(V1, 1);
        c.cvt_s_w(1, 1);
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(2, 0);
    if c.fcc() {
        c.neg_s(2, 2);
    }
    c.cycles += 4;
    c.c_le_s(0, 3);
    if !c.fcc() {
        frame.restore(c, S0, 0x10);
        return oc1109_fade_exit(c, frame);
    }
    c.cycles += 6;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.c_le_s(3, 0);
    if !c.fcc() {
        frame.restore(c, S0, 0x10);
        return oc1109_fade_exit(c, frame);
    }
    if p.volume {
        c.mul_s(0, 2, 3);
        c.add_s(3, 1, 0);
        c.cvt_w_s(0, 3);
        c.mfc1(A1, 0);
        c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
    } else {
        c.cycles += 1;
        c.mul_s(0, 2, 3);
        c.g[A0][0] = c.lw(s0.wrapping_add(0x54));
        c.add_s(3, 1, 0);
        c.cvt_w_s(0, 3);
        c.mfc1(A2, 0);
        c.g[A1][0] = 0;
    }
    call_ptr(c, p.setter, p.ret_set)?;
    c.cycles += 1;
    frame.restore(c, S0, 0x10);
    oc1109_fade_exit(c, frame)
}

fn oc1109_fade_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void PATH_TrackUpdateVolumeFade(Track* t)` (0x3d5a98): fade the track
/// volume byte (`+0xc` to `+0xe`, start time `+8`, final `+0x34`).
#[allow(dead_code)]
pub fn path_track_update_volume_fade(c: &mut Cpu) {
    guest(c, |c| {
        oc1109_fade(
            c,
            &Oc1109FadeLayout {
                start: 0x8,
                cur: 0xc,
                cfg_w: 0x18,
                cfg_h: 0x14,
                setter: 0x002a_f6c0,
                ret_done: 0x003d_5bb4,
                ret_set: 0x003d_5cac,
                volume: true,
            },
        )
    })
}

/// `void PATH_TrackUpdateAuxSendFade(Track* t)` (0x3d5cc0): aux send fade
/// (`+0x14` to `+0x16`, start `+0x10`).
#[allow(dead_code)]
pub fn path_track_update_aux_send_fade(c: &mut Cpu) {
    guest(c, |c| {
        oc1109_fade(
            c,
            &Oc1109FadeLayout {
                start: 0x10,
                cur: 0x14,
                cfg_w: 0x20,
                cfg_h: 0x1c,
                setter: 0x002a_f4e0,
                ret_done: 0x003d_5ddc,
                ret_set: 0x003d_5ed8,
                volume: false,
            },
        )
    })
}

/// `void PATH_TrackUpdateFade3(Track* t)` (0x3d5ee8): third fade
/// (`+0x1c` to `+0x1e`, start `+0x18`).
#[allow(dead_code)]
pub fn path_track_update_fade3(c: &mut Cpu) {
    guest(c, |c| {
        oc1109_fade(
            c,
            &Oc1109FadeLayout {
                start: 0x18,
                cur: 0x1c,
                cfg_w: 0x28,
                cfg_h: 0x24,
                setter: 0x002a_f580,
                ret_done: 0x003d_6004,
                ret_set: 0x003d_60f8,
                volume: false,
            },
        )
    })
}

/// `fd = (float)(uint32)V0` the way gcc emits it: `V1 = V0 >> 1` always, then
/// the plain convert, or (top bit set) halve with the low bit kept, convert
/// and double. `pos`/`neg` are the cycles of the two arms.
fn oc1109_u2f(c: &mut Cpu, fd: usize, pos: u64, neg: u64) {
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if !negative {
        c.cycles += pos;
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
    } else {
        c.cycles += neg;
        c.g[V0][0] &= 1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
        c.add_s(fd, fd, fd);
    }
}

/// `int PATH_TrackGetBeatPos(Track* t, BeatPos* out)` (0x3d32b8): look up
/// the track's pattern entry, read the platform play position, compute the
/// beat index (returned) and, if `out` is non-null, fill it with beat
/// length, remaining ticks etc.
pub fn path_track_get_beat_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x450000;
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        let a = frame.at(c, 0x68);
        c.swc1(21, a);
        let a = frame.at(c, 0x60);
        c.swc1(20, a);
        let s1 = c.g[S1][0] as u32;
        let spp = sp(c);
        c.g[A0][0] = c.lw(0x450000u32.wrapping_sub(0xbe0));
        c.g[V1][0] = c.lh(s1.wrapping_add(0x36));
        let p = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(p.wrapping_add(0x38));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[A1][0] = c.lw(p.wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = c.lw(p.wrapping_add(0x50));
        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let s2 = c.g[S2][0] as u32;
        c.g[V1][0] = c.lh(s2);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let v1 = c.g[V1][0] as u32;
        c.ldl(2, v1.wrapping_sub(1));
        c.ldr(2, v1.wrapping_sub(8));
        c.sdl(2, spp.wrapping_add(7));
        c.sdr(2, spp);
        // beat count (sp+4) -> f20
        c.g[V0][0] = c.lw(spp.wrapping_add(4));
        oc1109_u2f(c, 20, 4, 6);
        c.g[V0][0] = c.lw(s2.wrapping_add(4));
        // f0 = beats * subdivisions, f20 /= f0 when the time-signature bits are set
        c.cycles += 5;
        c.g[V1][0] = 0x0f00_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] &= c.g[V1][0];
        let none = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 24);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[A0][0] as u32) >> 20);
            c.g[V1][0] &= 0xf;
            c.g[V0][0] &= 0xf;
            c.mult(0, 2, 3, 2);
            oc1109_u2f(c, 0, 4, 5);
            c.cycles += 1;
            c.div_s(20, 20, 0);
        }
        // f1 = float(sp+4)
        c.cycles += 3;
        c.g[V0][0] = c.lw(spp.wrapping_add(4));
        oc1109_u2f(c, 1, 4, 6);
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 20);
        // f0 = float((word >> 20) & 0xf)
        c.cycles += 3;
        c.g[V1][0] = c.g[V0][0] & 0xf;
        let t = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] &= 1;
        if !t {
            c.cycles += 4;
            c.mtc1(V1, 0);
            c.cvt_s_w(0, 0);
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
        }
        c.g[A0][0] = c.lw(s1.wrapping_add(0x54));
        c.cycles += 9;
        c.div_s(21, 1, 0);
        c.g[A1][0] = sx(spp.wrapping_add(0x10));
        c.g[A2][0] = sx(spp.wrapping_add(0x14));
        call_ptr(c, 0x002a_ec58, 0x003d_3424)?;
        c.g[V1][0] = c.lw(s1.wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 23);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 27) as u32);
        let t = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = c.lw(spp.wrapping_add(0x14));
        let mut looped = false;
        if !t {
            c.cycles += 4;
            c.g[V0][0] = c.lw(s2.wrapping_add(0xc));
            c.g[V0][0] &= 2;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(spp.wrapping_add(0x14));
            looped = !t;
        }
        if looped {
            c.cycles += 2;
            oc1109_u2f(c, 2, 4, 6);
            c.g[V0][0] = c.lw(s1.wrapping_add(0x40));
            c.cycles += 2;
            oc1109_u2f(c, 1, 4, 6);
            c.g[V0][0] = c.lw(s2.wrapping_add(4));
            c.cycles += 12;
            c.g[A0][0] = c.lw(s1.wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 20);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 24);
            c.g[V1][0] &= 0xf;
            c.g[V0][0] &= 0xf;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 23);
            c.mult(0, 2, 2, 3);
            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 27) as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            oc1109_u2f(c, 0, 4, 6);
            c.mul_s(0, 1, 0);
            c.cycles += 3;
            c.g[V0][0] = c.lw(spp.wrapping_add(4));
            c.add_s(2, 2, 0);
            oc1109_u2f(c, 1, 4, 8);
            c.sub_s(1, 1, 2);
        } else {
            c.cycles += 2;
            oc1109_u2f(c, 2, 4, 6);
            c.g[V0][0] = c.lw(spp.wrapping_add(0x10));
            c.cycles += 2;
            oc1109_u2f(c, 1, 4, 5);
        }
        c.cycles += 11;
        c.div_s(1, 1, 20);
        c.cvt_w_s(0, 1);
        c.mfc1(A3, 0);
        c.mul_s(1, 1, 20);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(3, 21, 1);
        c.mul_s(0, 0, 20);
        let out_null = c.g[S0][0] == 0;
        c.sub_s(0, 0, 1);
        if !out_null {
            c.cycles += 27;
            let s0 = c.g[S0][0] as u32;
            c.g[V1][0] = c.lbu(s2.wrapping_add(7));
            c.g[A0][0] = 0xffff_ffff_ffff_fff0;
            c.g[V0][0] = c.lw(s0);
            c.g[A1][0] = 0xffff_ffff_ffff_ff0f;
            c.g[V1][0] &= 0xf;
            c.g[A2][0] = c.lw(spp.wrapping_add(4));
            c.g[V0][0] &= c.g[A0][0];
            c.g[V0][0] |= c.g[V1][0];
            c.sw(s0, c.g[V0][0]);
            c.g[V0][0] &= c.g[A1][0];
            c.g[V1][0] = c.lbu(s2.wrapping_add(8));
            c.cvt_w_s(1, 2);
            c.swc1(1, s0.wrapping_add(0x18));
            c.sw(s0.wrapping_add(0xc), c.g[A2][0]);
            c.g[V1][0] >>= 4;
            c.cvt_w_s(1, 20);
            c.swc1(1, s0.wrapping_add(4));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.cvt_w_s(1, 21);
            c.swc1(1, s0.wrapping_add(8));
            c.g[V0][0] |= c.g[V1][0];
            c.sw(s0, c.g[V0][0]);
            c.cvt_w_s(1, 3);
            c.swc1(1, s0.wrapping_add(0x14));
            c.cvt_w_s(1, 0);
            c.swc1(1, s0.wrapping_add(0x10));
            c.sb(s0.wrapping_add(1), c.g[A3][0]);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = c.g[A3][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x68);
        c.lwc1(21, a);
        let a = frame.at(c, 0x60);
        c.lwc1(20, a);
        frame.close(c);
        Ok(())
    })
}

/// `SPCH_BuildSentencePart(Sentence* st, SentenceDesc* d)` (0x3d8e58): clear
/// the sentence's per-sample bitmasks and flags, then for each element decode
/// its sample entries and mark which are present (the normal entry; every
/// register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn spch_build_sentence_part(c: &mut Cpu) {
    if c.pc != 0x003d8e58 {
        return ee::s14::f_003d8e58__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003d8e58;
        loop {
            match pc {
                0x003d8e58 => {
                    c.cycles += 26;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                    c.g[T4][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[T5][0] = 0x1;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(31, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[S4][0] = c.g[A1][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 0u64;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                    c.g[A0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = c.g[V0][0] & 0x7c;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        pc = 0x003d8ee4;
                        continue;
                    }
                    pc = 0x003d8ec0;
                    continue;
                }
                0x003d8ec0 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.g[S5][0];
                    pc = 0x003d8ec8;
                    continue;
                }
                0x003d8ec8 => {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x003d8ec8) {
                            return Ok(());
                        }
                        pc = 0x003d8ec8;
                        continue;
                    }
                    pc = 0x003d8ee4;
                    continue;
                }
                0x003d8ee4 => {
                    c.cycles += 3;
                    c.g[S3][0] = 0xb;
                    c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5f));
                    pc = 0x003d8ef0;
                    continue;
                }
                0x003d8ef0 => {
                    c.cycles += 7;
                    c.sb((c.g[V0][0] as u32), 0u64);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x003d8ef0) {
                            return Ok(());
                        }
                        pc = 0x003d8ef0;
                        continue;
                    }
                    pc = 0x003d8f0c;
                    continue;
                }
                0x003d8f0c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[S7][0] = sx((c.g[V0][0] as u32) >> 2);
                    c.g[S6][0] = 0x1;
                    pc = 0x003d8f18;
                    continue;
                }
                0x003d8f18 => {
                    c.cycles += 2;
                    let t = c.g[S7][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x003d9048;
                        continue;
                    }
                    pc = 0x003d8f20;
                    continue;
                }
                0x003d8f20 => {
                    c.cycles += 2;
                    c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[S1][0] = c.g[T4][0];
                    pc = 0x003d8f28;
                    continue;
                }
                0x003d8f28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x7));

                    if c.g[V0][0] != 0u64 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x003d903c;
                        continue;
                    }
                    pc = 0x003d8f34;
                    continue;
                }
                0x003d8f34 => {
                    c.cycles += 22;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32));
                    c.g[V1][0] = c.lbu((c.g[A0][0] as u32));
                    c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[S2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x62));
                    c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x003d8f80);
                    c.sq(13, (c.g[SP][0] as u32).wrapping_add(0x20));
                    Callee {
                        addr: 0x003db838,
                        f: ee::s14::SPCH_DecodeSampleEntry,
                    }
                    .call(c, 0x003d8f80)?;
                    c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(13, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x003d9000;
                        continue;
                    }
                    pc = 0x003d8f8c;
                    continue;
                }
                0x003d8f8c => {
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) <= 0;
                    c.g[A3][0] = 0u64;
                    if t {
                        pc = 0x003d9038;
                        continue;
                    }
                    pc = 0x003d8f94;
                    continue;
                }
                0x003d8f94 => {
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = 0xff;
                    c.g[T1][0] = c.g[S1][0];
                    c.g[T3][0] = 0xfe;
                    c.g[T2][0] = 0x1;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
                    pc = 0x003d8fb0;
                    continue;
                }
                0x003d8fb0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x8));
                    let t = c.g[A0][0] == c.g[T0][0];
                    c.g[V0][0] = c.g[A0][0] & 0x80;
                    if t {
                        pc = 0x003d9024;
                        continue;
                    }
                    pc = 0x003d8fbc;
                    continue;
                }
                0x003d8fbc => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        pc = 0x003d9028;
                        continue;
                    }
                    pc = 0x003d8fc4;
                    continue;
                }
                0x003d8fc4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32));
                    let t = c.g[V0][0] != c.g[T0][0];
                    c.g[A0][0] = c.g[A0][0] & 0x7f;
                    if t {
                        pc = 0x003d9008;
                        continue;
                    }
                    pc = 0x003d8fd0;
                    continue;
                }
                0x003d8fd0 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lbu((c.g[A1][0] as u32));
                        pc = 0x003d8ff0;
                        continue;
                    }
                    pc = 0x003d8fe4;
                    continue;
                }
                0x003d8fe4 => {
                    c.cycles += 3;
                    c.sb((c.g[T1][0] as u32).wrapping_add(0x7), 0u64);
                    let t = true;
                    c.g[S6][0] = 0u64;
                    if t {
                        pc = 0x003d9024;
                        continue;
                    }
                    pc = 0x003d8ff0;
                    continue;
                }
                0x003d8ff0 => {
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[V0][0] as u32) & 31)) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;

                    if c.g[V0][0] != 0u64 {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        pc = 0x003d9028;
                        continue;
                    }
                    pc = 0x003d9000;
                    continue;
                }
                0x003d9000 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[T5][0] = 0u64;
                    if t {
                        pc = 0x003d9050;
                        continue;
                    }
                    pc = 0x003d9008;
                    continue;
                }
                0x003d9008 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[T3][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        pc = 0x003d9028;
                        continue;
                    }
                    pc = 0x003d9010;
                    continue;
                }
                0x003d9010 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    c.g[V1][0] = sx((c.g[T2][0] as u32) << ((c.g[V1][0] as u32) & 31));
                    c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                    pc = 0x003d9024;
                    continue;
                }
                0x003d9024 => {
                    c.cycles += 1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    pc = 0x003d9028;
                    continue;
                }
                0x003d9028 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003d8fb0) {
                            return Ok(());
                        }
                        pc = 0x003d8fb0;
                        continue;
                    }
                    pc = 0x003d9038;
                    continue;
                }
                0x003d9038 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x003d903c;
                    continue;
                }
                0x003d903c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x003d8f28) {
                            return Ok(());
                        }
                        pc = 0x003d8f28;
                        continue;
                    }
                    pc = 0x003d9048;
                    continue;
                }
                0x003d9048 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[S6][0] = 0x1;
                    if t {
                        if back_edge_unwinds(c, 0x003d8f18) {
                            return Ok(());
                        }
                        pc = 0x003d8f18;
                        continue;
                    }
                    pc = 0x003d9050;
                    continue;
                }
                0x003d9050 => {
                    c.cycles += 13;
                    c.lq(31, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.g[T5][0];
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d8e58, 0x003d9084);
                    return Ok(());
                }
            }
        }
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_8e58,
        spch_build_sentence_part,
        "SPCH_BuildSentencePart",
    ),
    (0x003d_32b8, path_track_get_beat_pos, "PATH_TrackGetBeatPos"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA};

    /// Fixture for the three fade updaters: a track at DATA, the track
    /// table at DATA+0x1000, the clock at 0x44f438.
    fn fade_setup(c: &mut Cpu, seed: u32, salt: u64, l: &Oc1109FadeLayout) {
        let mut r = fakes::machine(c, seed, salt, 1);
        let table = DATA + 0x1000;
        c.bus.write32(0x44f420, table);
        let mode = seed % 5;
        let dur = [0u32, 1, 100, 60000, 7][(seed / 5 % 5) as usize];
        c.bus.write32(table + l.cfg_w, mode | (r.next_u32() & 0x80));
        c.bus.write16(table + l.cfg_h, dur as u16);
        let now = r.next_u32() >> (seed % 3 * 8);
        c.bus.write32(0x44f438, now);
        let start = match seed % 4 {
            0 => now.wrapping_sub(r.next_u32() % (dur + 1)),
            1 => now.wrapping_sub(dur + 5),
            2 => now.wrapping_add(r.next_u32() % 100 + 1), // clock before start: huge elapsed
            _ => now.wrapping_sub(r.next_u32() % 200),
        };
        c.bus.write32(DATA + l.start, start);
        let cur = (r.next_u32() % 200) as u8;
        let tgt = if seed % 7 == 0 {
            cur
        } else {
            (r.next_u32() % 200) as u8
        };
        c.bus.write8(DATA + l.cur, cur);
        c.bus.write8(DATA + l.cur + 2, tgt);
        c.bus.write8(DATA + 0x34, (r.next_u32() & 0xff) as u8);
        c.bus.write32(DATA + 0x54, r.next_u32());
        c.g[A0][0] = DATA as u64;
    }

    const OC1109_VOL: Oc1109FadeLayout = Oc1109FadeLayout {
        start: 0x8,
        cur: 0xc,
        cfg_w: 0x18,
        cfg_h: 0x14,
        setter: 0,
        ret_done: 0,
        ret_set: 0,
        volume: true,
    };
    const OC1109_AUX: Oc1109FadeLayout = Oc1109FadeLayout {
        start: 0x10,
        cur: 0x14,
        cfg_w: 0x20,
        cfg_h: 0x1c,
        setter: 0,
        ret_done: 0,
        ret_set: 0,
        volume: false,
    };
    const OC1109_F3: Oc1109FadeLayout = Oc1109FadeLayout {
        start: 0x18,
        cur: 0x1c,
        cfg_w: 0x28,
        cfg_h: 0x24,
        setter: 0,
        ret_done: 0,
        ret_set: 0,
        volume: false,
    };

    /// Beat position: pattern table lookup, loop-adjusted and plain arms, null and
    /// non-null `out`.
    #[test]
    fn path_track_get_beat_pos_matches() {
        check(
            ee::s14::f_003d32b8__generated,
            path_track_get_beat_pos,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d32b8, 1);
                let table = DATA + 0x1000;
                c.bus.write32(0x44f420, table);
                // +0x38: u16 index table, +0x34: entries (4 bytes), +0x50: u64 table
                c.bus.write32(table + 0x38, DATA + 0x2000);
                c.bus.write32(table + 0x34, DATA + 0x2100);
                c.bus.write32(table + 0x50, DATA + 0x2200);
                c.bus.write16(DATA + 0x36, 1);
                c.bus.write16(DATA + 0x2000 + 2, 3); // index -> entry 3
                c.bus.write16(DATA + 0x2100 + 12, 2); // entry -> u64 slot 2
                let beats = match seed % 4 {
                    0 => 0,
                    1 => 7,
                    2 => 0x8000_0001,
                    _ => r.next_u32(),
                };
                c.bus.write64(
                    DATA + 0x2200 + 8,
                    (beats as u64) << 32 | r.next_u32() as u64,
                );
                let sig = [0u32, 0x0400_0000, 0x0440_0000, 0x0f10_0000, 0x0220_0000]
                    [(seed / 4 % 5) as usize];
                c.bus
                    .write32(DATA + 0x2100 + 12 + 4, sig | (r.next_u32() & 0xf0f0_00ff));
                c.bus
                    .write32(DATA + 0x2100 + 12 + 12, if seed % 3 == 0 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x20, [0u32, 3 << 4, 1 << 8][(seed % 3) as usize]);
                let spp = c.g[SP][0] as u32 - 0x70;
                c.bus.write32(
                    spp + 0x10,
                    if seed % 7 == 0 {
                        0x8000_0005
                    } else {
                        r.next_u32() % 90
                    },
                );
                c.bus.write32(
                    spp + 0x14,
                    if seed % 5 == 0 {
                        0x8000_0009
                    } else {
                        r.next_u32() % 90
                    },
                );
                c.bus.write32(
                    DATA + 0x40,
                    if seed % 5 == 0 {
                        0x8000_0003
                    } else {
                        r.next_u32() % 50
                    },
                );
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 6 == 0 {
                    0
                } else {
                    (DATA + 0x3000) as u64
                };
            },
        );
    }

    /// Sentence builder: a sentence of up to 4 elements over two random banks;
    /// the real (ported) SPCH_DecodeSampleEntry runs as callee.
    #[test]
    fn spch_build_sentence_part_matches() {
        check(
            ee::s14::f_003d8e58__generated,
            spch_build_sentence_part,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d8e58, 0);
                let st = DATA + 0x100;
                let d = DATA + 0x400;
                for i in 0..0x200 {
                    c.bus.write8(st + i, r.next_u32() as u8);
                }
                for i in 0..0x800 {
                    c.bus.write8(d + i, r.next_u32() as u8);
                }
                for b in 0..2u32 {
                    let bank = DATA + 0x1000 + b * 0x1400;
                    for i in 0..0x1200 {
                        c.bus.write8(bank + i, r.next_u32() as u8);
                    }
                    c.bus.write8(bank + 5, 1 + (r.next_u32() % 12) as u8);
                }
                let tbl = DATA + 0xf00;
                c.bus.write32(tbl, DATA + 0x1000);
                c.bus.write32(tbl + 4, DATA + 0x2400);
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_add(0x1734), tbl);
                let n = 1 + seed % 4;
                c.bus.write8(d + 4, ((n << 2) | (r.next_u32() & 3)) as u8);
                c.bus.write8(d + 5, (seed / 4 % 4) as u8);
                c.bus.write8(d + 6, (r.next_u32() % 8) as u8);
                for e in 0..n {
                    let el = st + 8 * e;
                    c.bus.write16(el, (r.next_u32() % 2) as u16);
                    c.bus.write8(el + 6, (r.next_u32() % 6) as u8);
                    c.bus
                        .write8(el + 7, if seed % 5 == 0 && e == 0 { 1 } else { 0 });
                    c.bus.write8(d + 8 + e, (r.next_u32() % 8) as u8);
                }
                for i in 0..6 {
                    c.bus.write8(st + 0x62 + i, (r.next_u32() % 12) as u8);
                }
                c.pc = 0x003d_8e58;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = d as u64;
            },
        );
    }
}
