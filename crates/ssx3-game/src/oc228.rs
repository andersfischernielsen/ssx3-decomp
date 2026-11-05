//! Staging wave 203 (oc228): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, addu, r32, reg, ret, set_reg};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003c_1298,
        snd_release_finished_voices,
        "SND_ReleaseFinishedVoices",
    ),
    (0x0038_ee78, vfx_plants_update, "VfxPlants_Update"),
    (0x0022_00c0, hud_draw_speedometer, "HUD_DrawSpeedometer"),
    (0x003e_12e0, stream_file_get_chunk, "StreamFile_GetChunk"),
    (
        0x0037_dbe8,
        ps2_graphics_man_clip_code,
        "PS2GraphicsMan_v91",
    ),
];

/// `slt`: signed 64-bit compare of two registers.
#[inline]
fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// `addiu`-style 32-bit add of an immediate, sign-extended.
#[inline]
fn addiu(a: u64, k: i32) -> u64 {
    sx((a as u32).wrapping_add(k as u32))
}

// ---------------------------------------------------------------------------
// SND_ReleaseFinishedVoices (0x3c1298)

/// The SND driver state and the voice status block the IOP reports into.
mod release {
    /// `lui 0x51`: the high half both blocks are addressed from.
    pub const HI: u64 = 0x0051_0000;
    /// The driver state (`HI - 0x5718`).
    pub const STATE: u32 = 0x0050_a8e8;
    /// The voice status block (`HI - 0x5300`).
    pub const VOICES: u32 = 0x0050_ad00;
    // State fields.
    /// Channel table pointer.
    pub const CHANNELS: u32 = 0x1f0;
    /// Number of hardware-voice channels (u8): the first group.
    pub const COUNT_A: u32 = 0x30;
    /// Number of channels of the second group (u8), after the first.
    pub const COUNT_B: u32 = 0x2f;
    /// Output mode (u8); 2 releases a voice's mirror (`+0x18`) as well.
    pub const MODE: u32 = 0x49;
    // Channel fields (0x8c bytes each).
    pub const CHANNEL_SIZE: u32 = 0x8c;
    /// Voice indices (s16 each).
    pub const CH_VOICES: u32 = 0x04;
    /// Number of voices (u8).
    pub const CH_VOICE_COUNT: u32 = 0x23;
    /// Timed-stop state (s16); -1 = no timed stop pending.
    pub const CH_STOP: u32 = 0x28;
    // Voice status block fields.
    /// Pointer to the status table the IOP fills in.
    pub const STATUS: u32 = 0xcb0;
    /// Last key seen per first-group voice (8-byte slots: key, remaining).
    pub const SEEN_A: u32 = 0x83c;
    /// Last key seen per second-group voice (8-byte slots: key, remaining).
    pub const SEEN_B: u32 = 0x9bc;
    // Status table entries.
    /// First group: key word, 8-byte stride.
    pub const KEY_A: u32 = 0x20;
    /// Second group: key word, 16-byte stride.
    pub const KEY_B: u32 = 0x1a0;
    /// Mode 2: the mirror voice of voice `h` is `h + 0x18`.
    pub const MIRROR: u32 = 0x18;
    /// Return addresses of the two `SND_ChannelRelease` calls.
    pub const RET_A: u32 = 0x003c_13b0;
    pub const RET_B: u32 = 0x003c_1504;
    /// Loop heads (event checks).
    pub const HEAD_A: u32 = 0x003c_12e8;
    pub const HEAD_A_VOICES: u32 = 0x003c_1388;
    pub const HEAD_B: u32 = 0x003c_1428;
    pub const HEAD_B_VOICES: u32 = 0x003c_14c8;

    pub const SND_CHANNEL_RELEASE: super::Callee = super::Callee {
        addr: 0x003b_a550,
        f: super::ee::s13::SND_ChannelRelease,
    };
}

/// `void SND_ReleaseFinishedVoices(void)` (0x3c1298): walks both channel
/// groups of the SND driver and releases the voices of every channel whose
/// hardware voice has finished. For each channel `i` with a non-zero key in
/// the IOP status table and no timed stop pending (`stop == -1`):
///
/// - first group: when the key changed since last time, the remaining
///   count is reloaded from the key's low 23 bits; when it is zero, each of
///   the channel's voices gets its status key cleared and
///   `SND_ChannelRelease(voice)` (in mode 2 the mirror voice `+0x18` too);
/// - second group: when the key changed, the remaining word is cleared;
///   when it is -1, each voice's status key is cleared and
///   `SND_ChannelRelease(voice)`.
pub fn snd_release_finished_voices(c: &mut Cpu) {
    use release::*;
    guest(c, |c| {
        let st = |o: u32| Addr(STATE + o);
        let vo = |o: u32| Addr(VOICES + o);
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        set_reg(c, A0, HI);
        frame.save(c, S2, 0x70);
        set_reg(c, V0, sx(STATE));
        frame.save(c, FP, 0x10);
        set_reg(c, S2, 0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0);
        let count_a = st(COUNT_A).lbu(c);
        set_reg(c, V1, count_a);
        set_reg(c, FP, HI);

        // First group.
        if count_a != 0 {
            c.cycles += 3;
            set_reg(c, V0, HI);
            set_reg(c, S7, sx(VOICES));
            let mut i: u32 = 0;
            loop {
                c.cycles += 6;
                let status = vo(STATUS).lw(c);
                let slot = sx(i << 3);
                set_reg(c, A2, slot);
                let entry = addu(status, slot);
                set_reg(c, V0, entry);
                let key = Addr((entry as u32).wrapping_add(KEY_A)).lw(c);
                set_reg(c, A1, key);
                set_reg(c, S6, sx(i.wrapping_add(1)));
                // true: through the one-cycle block that reloads v1.
                let short = 'ch: {
                    if key == 0 {
                        break 'ch true;
                    }
                    c.cycles += 9;
                    set_reg(c, V0, CHANNEL_SIZE as u64);
                    set_reg(c, T0, sx(STATE));
                    let off = sx(abi::mult(c, i, CHANNEL_SIZE));
                    set_reg(c, A3, off);
                    let chans = st(CHANNELS).lw(c);
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let stop = Addr((ch as u32).wrapping_add(CH_STOP)).lh(c);
                    set_reg(c, V0, u64::MAX);
                    set_reg(c, V1, sx(STATE));
                    if stop != u64::MAX {
                        break 'ch false;
                    }
                    c.cycles += 5;
                    let seen_key = sx(VOICES + SEEN_A);
                    set_reg(c, T1, seen_key);
                    let at = addu(slot, seen_key);
                    set_reg(c, V0, at);
                    let seen = Addr(at as u32).lw(c);
                    set_reg(c, V1, seen);
                    let seen_left = sx(VOICES + SEEN_A + 4);
                    set_reg(c, A0, seen_left);
                    if seen != key {
                        c.cycles += 5;
                        let at = addu(slot, seen_left);
                        set_reg(c, V1, at);
                        let left = key & 0x7f_ffff;
                        set_reg(c, V0, left);
                        Addr(at as u32).sw(c, left);
                    }
                    c.cycles += 4;
                    let at = addu(slot, seen_left);
                    set_reg(c, V0, at);
                    let left = Addr(at as u32).lw(c);
                    set_reg(c, V1, sx(STATE));
                    if left != 0 {
                        break 'ch false;
                    }
                    c.cycles += 5;
                    let chans = st(CHANNELS).lw(c);
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                    set_reg(c, V1, n);
                    set_reg(c, S1, 0);
                    if n == 0 {
                        break 'ch true;
                    }
                    c.cycles += 6;
                    set_reg(c, S3, sx(STATE));
                    set_reg(c, S2, off);
                    set_reg(c, S4, sx(VOICES));
                    set_reg(c, S5, 2);
                    let mut chans = st(CHANNELS).lw(c);
                    set_reg(c, V0, chans);
                    let mut j: u32 = 0;
                    loop {
                        c.cycles += 13;
                        let half = sx(j << 1);
                        set_reg(c, V1, half);
                        let status = vo(STATUS).lw(c);
                        let at = addu(addu(off, chans), half);
                        set_reg(c, V0, at);
                        let voice = Addr((at as u32).wrapping_add(CH_VOICES)).lh(c);
                        set_reg(c, S0, voice);
                        let v8 = sx((voice as u32) << 3);
                        set_reg(c, V1, v8);
                        set_reg(c, A0, voice);
                        let e = addu(status, v8);
                        set_reg(c, A1, e);
                        Addr((e as u32).wrapping_add(KEY_A)).sw(c, 0);
                        SND_CHANNEL_RELEASE.call(c, RET_A)?;
                        let mode = st(MODE).lbu(c);
                        set_reg(c, V0, mode);
                        if mode == 2 {
                            c.cycles += 6;
                            let status = vo(STATUS).lw(c);
                            let m8 = sx((voice as u32).wrapping_add(MIRROR) << 3);
                            set_reg(c, V0, m8);
                            let e = addu(status, m8);
                            set_reg(c, V1, e);
                            Addr((e as u32).wrapping_add(KEY_A)).sw(c, 0);
                        }
                        let chans_now = st(CHANNELS).lw(c);
                        c.cycles += 6;
                        j = j.wrapping_add(1);
                        set_reg(c, S1, sx(j));
                        let ch = addu(off, chans_now);
                        set_reg(c, V1, ch);
                        let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                        let more = slt(sx(j), n);
                        set_reg(c, V0, more);
                        if more == 0 {
                            break 'ch true;
                        }
                        chans = st(CHANNELS).lw(c);
                        set_reg(c, V0, chans);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_A_VOICES);
                        }
                    }
                };
                if short {
                    c.cycles += 1;
                    set_reg(c, V1, sx(STATE));
                }
                c.cycles += 5;
                i = i.wrapping_add(1);
                set_reg(c, S2, sx(i));
                let count = st(COUNT_A).lbu(c);
                let more = slt(sx(i), count);
                set_reg(c, V0, more);
                set_reg(c, A0, HI);
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD_A);
                }
            }
        }

        // Second group: channels COUNT_A.. (status slots from 0).
        c.cycles += 5;
        set_reg(c, A0, HI);
        set_reg(c, V0, sx(STATE));
        let count_b = st(COUNT_B).lbu(c);
        set_reg(c, V1, count_b);
        set_reg(c, S2, 0);
        if count_b != 0 {
            c.cycles += 4;
            set_reg(c, V0, HI);
            set_reg(c, S7, sx(VOICES));
            let mut status = vo(STATUS).lw(c);
            set_reg(c, V0, status);
            let mut i: u32 = 0;
            loop {
                c.cycles += 5;
                let slot16 = sx(i << 4);
                set_reg(c, V1, slot16);
                let entry = addu(status, slot16);
                set_reg(c, V0, entry);
                let key = Addr((entry as u32).wrapping_add(KEY_B)).lw(c);
                set_reg(c, A3, key);
                set_reg(c, S6, sx(i.wrapping_add(1)));
                let short = 'ch: {
                    if key == 0 {
                        break 'ch true;
                    }
                    c.cycles += 11;
                    set_reg(c, T0, sx(STATE));
                    set_reg(c, T2, CHANNEL_SIZE as u64);
                    let first = st(COUNT_A).lbu(c);
                    let chans = st(CHANNELS).lw(c);
                    set_reg(c, V1, chans);
                    let idx = addu(sx(i), first);
                    let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let stop = Addr((ch as u32).wrapping_add(CH_STOP)).lh(c);
                    set_reg(c, A2, stop);
                    set_reg(c, V0, u64::MAX);
                    set_reg(c, A0, HI);
                    if stop != u64::MAX {
                        break 'ch false;
                    }
                    c.cycles += 6;
                    let slot = sx(i << 3);
                    set_reg(c, A1, slot);
                    let seen_key = sx(VOICES + SEEN_B);
                    set_reg(c, T1, seen_key);
                    let at = addu(slot, seen_key);
                    set_reg(c, V1, at);
                    let seen = Addr(at as u32).lw(c);
                    set_reg(c, V0, seen);
                    let seen_left = sx(VOICES + SEEN_B + 4);
                    set_reg(c, A0, seen_left);
                    if seen != key {
                        c.cycles += 2;
                        let at = addu(slot, seen_left);
                        set_reg(c, V0, at);
                        Addr(at as u32).sw(c, 0);
                    }
                    c.cycles += 4;
                    let at = addu(slot, seen_left);
                    set_reg(c, V0, at);
                    let left = Addr(at as u32).lw(c);
                    set_reg(c, V1, left);
                    set_reg(c, A0, HI);
                    if left != stop {
                        break 'ch false;
                    }
                    c.cycles += 8;
                    let first = st(COUNT_A).lbu(c);
                    let chans = st(CHANNELS).lw(c);
                    set_reg(c, A0, chans);
                    let idx = addu(sx(i), first);
                    let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                    set_reg(c, V1, n);
                    set_reg(c, S1, 0);
                    if n == 0 {
                        break 'ch true;
                    }
                    c.cycles += 4;
                    set_reg(c, S3, sx(STATE));
                    set_reg(c, S5, sx(VOICES));
                    set_reg(c, S4, CHANNEL_SIZE as u64);
                    let mut first = st(COUNT_A).lbu(c);
                    set_reg(c, V1, first);
                    let mut j: u32 = 0;
                    loop {
                        c.cycles += 24;
                        let half = sx(j << 1);
                        set_reg(c, A2, half);
                        let chans = st(CHANNELS).lw(c);
                        set_reg(c, A0, chans);
                        j = j.wrapping_add(1);
                        set_reg(c, S1, sx(j));
                        let status = vo(STATUS).lw(c);
                        let idx = addu(sx(i), first);
                        let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                        let at = addu(addu(off, chans), half);
                        set_reg(c, V0, at);
                        let voice = Addr((at as u32).wrapping_add(CH_VOICES)).lh(c);
                        set_reg(c, S0, voice);
                        let v16 = sx((voice as u32).wrapping_sub(first as u32) << 4);
                        set_reg(c, V1, v16);
                        set_reg(c, A0, voice);
                        let e = addu(status, v16);
                        set_reg(c, A1, e);
                        Addr((e as u32).wrapping_add(KEY_B)).sw(c, 0);
                        SND_CHANNEL_RELEASE.call(c, RET_B)?;
                        let first_now = st(COUNT_A).lbu(c);
                        let chans = st(CHANNELS).lw(c);
                        set_reg(c, A0, chans);
                        let idx = addu(sx(i), first_now);
                        let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                        let ch = addu(off, chans);
                        set_reg(c, V0, ch);
                        let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                        let more = slt(sx(j), n);
                        set_reg(c, V1, more);
                        if more == 0 {
                            break 'ch true;
                        }
                        first = st(COUNT_A).lbu(c);
                        set_reg(c, V1, first);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_B_VOICES);
                        }
                    }
                };
                if short {
                    c.cycles += 1;
                    set_reg(c, A0, HI);
                }
                c.cycles += 6;
                i = i.wrapping_add(1);
                set_reg(c, S2, sx(i));
                set_reg(c, V1, sx(STATE));
                let count = st(COUNT_B).lbu(c);
                let more = slt(sx(i), count);
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                status = vo(STATUS).lw(c);
                set_reg(c, V0, status);
                if event_due(c) {
                    deliver_event_here(c, HEAD_B);
                }
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VfxPlants_Update (0x38ee78)

/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst` (a 64-byte matrix copy).
#[inline(always)]
fn copy_mat(c: &mut Cpu, src: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, src.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.sqc2(1 + k as usize, dst.wrapping_add(0x10 * k));
    }
}

/// `dst = a * b` with the VU0 macro ops (vf4-7 = a, vf8-11 = b, vf12-15 =
/// the result columns).
#[inline(always)]
fn mul_mat(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(4 + k as usize, a.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.lqc2(8 + k as usize, b.wrapping_add(0x10 * k));
    }
    for k in 0..4u8 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + k, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + k, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + k, Z));
        c.vu0().madd(0b1111, 12 + k, 7, Src::Bc(8 + k, W));
    }
    for k in 0..4u32 {
        c.sqc2(12 + k as usize, dst.wrapping_add(0x10 * k));
    }
}

/// Globals of the plant sway effect (gp-relative unless noted).
mod plants {
    /// The light/effects manager whose vtable slot gives the clock.
    pub const GP_MANAGER: u32 = 0x2a90;
    /// Its vtable pointer and the clock method's slot.
    pub const MANAGER_VTABLE: u32 = 0x10d8;
    pub const SLOT_CLOCK: u32 = 0x390;
    /// `1/600`: clock ticks to cycles (`gp - 0x26ac`).
    pub const GP_NEG_TIME_SCALE: u32 = 0x26ac;
    /// Degrees to radians (`gp - 0x26a8`).
    pub const GP_NEG_DEG2RAD: u32 = 0x26a8;
    /// Tweakables (`D_004A4448..`): sway amplitude scale/offset, wobble
    /// amplitude, twist amplitude, the four speeds (ints), the sway and
    /// wobble directions (degrees).
    pub const GP_SWAY_AMP: u32 = 0x1358;
    pub const GP_SWAY_BASE: u32 = 0x135c;
    pub const GP_WOBBLE_AMP: u32 = 0x1360;
    pub const GP_TWIST_AMP: u32 = 0x1364;
    pub const GP_SWAY_SPEED: u32 = 0x136c;
    pub const GP_TWIST_SPEED: u32 = 0x1370;
    pub const GP_WOBBLE_SPEED: u32 = 0x1374;
    pub const GP_WOBBLE_PHASE: u32 = 0x1378;
    pub const GP_SWAY_DIR: u32 = 0x137c;
    pub const GP_WOBBLE_DIR: u32 = 0x1380;
    /// The twist axis (`D_004FF160`) and the base matrix (`D_004FF1A0`).
    pub const AXIS: u32 = 0x004f_f160;
    pub const BASE: u32 = 0x004f_f1a0;
    /// The eight output matrices (`D_00504D80`, 64 bytes each).
    pub const CELLS: u32 = 0x0050_4d80;
    pub const SIN: u32 = 0x0031_bf60;
    pub const SIN_COS: u32 = 0x0031_be50;
    pub const COS: u32 = 0x0031_c040;
    pub const HEAD: u32 = 0x0038_ef20;
}

/// `void VfxPlants_Update(void)` (0x38ee78): builds the eight plant sway
/// matrices for this frame from the manager's clock. Per phase `i/8`: a sway
/// amplitude, a wobble amplitude and a twist angle from `sinf` of the
/// scaled clock; `m0 = base * rot(axis, twist)`; the cell matrix is the base
/// with its third row pushed along the sway direction, `m0`'s third column
/// along the wobble direction, and finally `cell = cell * m0`.
pub fn vfx_plants_update(c: &mut Cpu) {
    use plants::*;
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x1b0);
        let gp = r32(c, GP);
        let manager = Addr(gp.wrapping_add(GP_MANAGER)).lw(c);
        set_reg(c, A1, manager);
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        set_reg(c, S1, 0);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save(c, S7, 0xf0);
        set_reg(c, S6, 0x50_0000);
        frame.save(c, FP, 0xe0);
        set_reg(c, S5, sx(AXIS));
        frame.save_fpr(c, 26, 0x1a0);
        frame.save_fpr(c, 25, 0x198);
        frame.save_fpr(c, 24, 0x190);
        frame.save_fpr(c, 23, 0x188);
        frame.save(c, S0, 0x160);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 22, 0x180);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        c.lwc1(25, gp.wrapping_sub(GP_NEG_TIME_SCALE));
        let vt = Addr((manager as u32).wrapping_add(MANAGER_VTABLE)).lw(c);
        set_reg(c, V1, vt);
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 26); // 1.0
        let slot = VSlot::read(c, vt, SLOT_CLOCK);
        set_reg(c, A0, slot.delta);
        set_reg(c, V0, slot.func);
        c.lwc1(24, gp.wrapping_sub(GP_NEG_DEG2RAD));
        set_reg(c, A0, slot.this(manager));
        call_ptr(c, slot.func as u32, 0x0038_eef4)?;
        // f23 = (float)clock.
        c.mtc1(V0, 23);
        c.cvt_s_w(23, 23);
        let sp0 = sp(c);
        let (sin_at, cos_at) = (sp0 + 0xc0, sp0 + 0xc4);
        set_reg(c, FP, sx(cos_at));
        set_reg(c, V0, sx(sin_at));
        set_reg(c, S7, sx(sp0));
        Addr(sp0 + 0xc8).sw(c, sx(sin_at));
        let rot = sp0 + 0x40;
        set_reg(c, S2, sx(rot));
        set_reg(c, V0, 0x50_0000);
        let tmp = sp0 + 0x80;
        set_reg(c, S3, sx(tmp));
        set_reg(c, S4, sx(BASE));
        let m0 = sp0;
        let mut i: u32 = 0;
        loop {
            c.cycles += 207;
            // sway = SWAY_AMP * sin(i/8 + clock * SWAY_SPEED * scale) + SWAY_BASE
            c.lwc1(12, gp.wrapping_add(GP_SWAY_SPEED));
            c.cvt_s_w(12, 12);
            set_reg(c, AT, 0x3e00_0000);
            c.mtc1(AT, 0); // 0.125
            c.mtc1(S1, 20);
            c.cvt_s_w(20, 20);
            c.mul_s(12, 23, 12);
            c.mul_s(20, 20, 0); // f20 = i / 8
            c.mul_s(12, 12, 25);
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_ef4c)?;
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_SPEED));
            c.lwc1(21, gp.wrapping_add(GP_SWAY_AMP));
            c.mul_s(12, 23, 12);
            c.lwc1(1, gp.wrapping_add(GP_SWAY_BASE));
            c.mul_s(21, 21, 0);
            c.mul_s(12, 12, 25);
            c.add_s(21, 21, 1); // f21 = sway
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_ef70)?;
            // wobble = WOBBLE_AMP * sin(i/8 * WOBBLE_PHASE * scale + b)
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_PHASE));
            c.mul_s(12, 20, 12);
            c.mul_s(12, 12, 25);
            c.add_s(12, 12, 0);
            call_ptr(c, SIN, 0x0038_ef84)?;
            c.lwc1(12, gp.wrapping_add(GP_TWIST_SPEED));
            c.lwc1(22, gp.wrapping_add(GP_WOBBLE_AMP));
            c.mul_s(12, 23, 12);
            c.mul_s(22, 22, 0); // f22 = wobble
            c.mul_s(12, 12, 25);
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_efa0)?;
            // angle = TWIST_AMP * sin(...); m0 = base; sincos(&s, &c, angle).
            c.lwc1(12, gp.wrapping_add(GP_TWIST_AMP));
            c.mul_s(12, 12, 0);
            copy_mat(c, BASE, m0);
            let v = Addr(sp0 + 0xc8).lw(c);
            set_reg(c, A0, v);
            set_reg(c, A1, sx(cos_at));
            call_ptr(c, SIN_COS, 0x0038_efd4)?;
            // rot = rotation about the axis by the angle (Rodrigues).
            c.lwc1(11, cos_at);
            c.lwc1(4, AXIS + 8);
            c.sub_s(1, 26, 11); // 1 - cos
            c.lwc1(0, AXIS);
            c.lwc1(6, AXIS + 4);
            c.lwc1(8, sin_at);
            c.mul_s(2, 1, 4);
            Addr(rot + 0x30).sw(c, 0);
            c.mul_s(5, 1, 0);
            Addr(rot + 0x34).sw(c, 0);
            c.mul_s(1, 1, 6);
            Addr(rot + 0x38).sw(c, 0);
            c.mul_s(13, 8, 4);
            Addr(rot + 0x0c).sw(c, 0);
            c.mul_s(12, 8, 0);
            Addr(rot + 0x1c).sw(c, 0);
            c.mul_s(9, 2, 4);
            Addr(rot + 0x2c).sw(c, 0);
            c.mul_s(10, 5, 4);
            c.swc1(26, rot + 0x3c);
            c.mul_s(3, 2, 0);
            c.mul_s(7, 5, 0);
            c.mul_s(4, 1, 4);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(5, 5, 6);
            c.mul_s(1, 1, 6);
            c.add_s(3, 3, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(3, rot + 0x08);
            c.add_s(7, 7, 11);
            c.swc1(0, rot + 0x04);
            c.add_s(5, 5, 13);
            c.swc1(2, rot + 0x18);
            c.sub_s(10, 10, 8);
            c.swc1(9, rot + 0x28);
            c.add_s(1, 1, 11);
            c.swc1(7, rot);
            c.add_s(4, 4, 12);
            c.swc1(5, rot + 0x10);
            c.swc1(10, rot + 0x20);
            c.swc1(1, rot + 0x14);
            c.swc1(4, rot + 0x24);
            // m0 = m0 * rot; cell = base.
            mul_mat(c, m0, rot, m0);
            set_reg(c, V1, 0x50_0000);
            let cell = CELLS.wrapping_add(i << 6);
            set_reg(c, V0, sx(CELLS));
            set_reg(c, S0, sx(cell));
            copy_mat(c, BASE, cell);
            // cell row 2 += (sin, cos)(SWAY_DIR) * sway
            c.lwc1(12, gp.wrapping_add(GP_SWAY_DIR));
            c.mul_s(12, 12, 24);
            call_ptr(c, SIN, 0x0038_f138)?;
            c.mul_s(0, 0, 21);
            c.lwc1(1, cell + 0x20);
            c.lwc1(12, gp.wrapping_add(GP_SWAY_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, cell + 0x20);
            call_ptr(c, COS, 0x0038_f154)?;
            c.mul_s(0, 0, 21);
            c.lwc1(1, cell + 0x24);
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, cell + 0x24);
            // m0 column 2 += (sin, cos)(WOBBLE_DIR) * wobble
            call_ptr(c, SIN, 0x0038_f170)?;
            c.mul_s(0, 22, 0);
            c.lwc1(1, m0 + 0x08);
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, m0 + 0x08);
            call_ptr(c, COS, 0x0038_f18c)?;
            c.mul_s(22, 22, 0);
            c.lwc1(0, m0 + 0x18);
            c.add_s(0, 0, 22);
            c.swc1(0, m0 + 0x18);
            // cell = cell * m0 (through two stack copies).
            mul_mat(c, cell, m0, tmp);
            copy_mat(c, tmp, rot);
            copy_mat(c, rot, cell);
            i += 1;
            set_reg(c, S1, sx(i));
            let more = (i as i32) < 8;
            set_reg(c, V0, more as u64);
            frame.restore(c, S0, 0x160);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 18;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 26, 0x1a0);
        frame.restore_fpr(c, 25, 0x198);
        frame.restore_fpr(c, 24, 0x190);
        frame.restore_fpr(c, 23, 0x188);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_DrawSpeedometer (0x2200c0)

mod speedo {
    /// The HUD settings block copied to the stack (0x288 bytes, 8-aligned).
    pub const SETTINGS: u32 = 0x0053_5610;
    pub const SETTINGS_SIZE: u32 = 0x280;
    /// Settings word 0: bit 19 clear = the converted units (factor and
    /// second label).
    pub const UNITS_FLAG: u64 = 0x8_0000;
    /// The rider's velocity provider (an object with a vtable).
    pub const RIDER_VELOCITY: u32 = 0x6c0;
    pub const SLOT_GET_VELOCITY: u32 = 0x10;
    /// The speedometer's HUD element (element 25 of 0x24 bytes).
    pub const ELEM: u32 = 0x384;
    pub const ELEM_ALIGN_X: u32 = 0x3a4;
    pub const ELEM_ALIGN_Y: u32 = 0x3a5;
    /// The HUD's text object.
    pub const HUD_TEXT: u32 = 0x42c;
    /// Text object fields: colour (8 bytes at 0x40), base scale, scale.
    pub const TEXT_COLOR: u32 = 0x40;
    pub const TEXT_BASE_SCALE: u32 = 0x30;
    pub const TEXT_SCALE: u32 = 0x38;
    pub const TEXT_LINE: u32 = 0x14;
    /// The two unit labels (flag set / clear) and the number format.
    pub const LABEL_A: u32 = 0x004a_2308;
    pub const LABEL_B: u32 = 0x004a_2310;
    pub const NUMBER_FORMAT: u32 = 0x004a_2138;
    /// `gp -` constants: speed scale, units conversion factor, the two digit-scale
    /// factors (fewer than three digits or not), the label offset.
    pub const GP_NEG_SPEED_SCALE: u32 = 0x5320;
    pub const GP_NEG_UNITS_FACTOR: u32 = 0x531c;
    pub const GP_NEG_SCALE_SHORT: u32 = 0x5318;
    pub const GP_NEG_SCALE_LONG: u32 = 0x5314;
    pub const GP_NEG_LABEL_OFFSET: u32 = 0x5310;
    // Callees (all called through the function table).
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const HUD_GET_ELEM_POS: u32 = 0x001e_91f8;
    pub const HUD_GET_ELEM_SCALE: u32 = 0x001e_9290;
    pub const HUD_GET_ELEM_COLOR: u32 = 0x001e_91a8;
    pub const FONT_MEASURE_TEXT: u32 = 0x0039_1fb0;
    pub const HUD_ALIGN_ELEM_POS: u32 = 0x001f_10f8;
    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const FONT_DRAW_TEXT_CENTERED: u32 = 0x0039_2908;
    pub const COPY_HEAD: u32 = 0x0022_01e0;
}

/// `void HUD_DrawSpeedometer(Hud* hud, Elem* elems, Rider* rider, int
/// arg)` (0x2200c0): the speed readout. Copies the HUD settings to the
/// stack, takes the length of the rider's velocity (VU0 `sqrt`) times the
/// speed scale (and the units factor when settings bit 19 is clear),
/// draws the unit label at the speedometer element (measured, aligned with
/// `HUD_AlignElemPos`), then the number (`sprintf("%d", speed + 0.5)`)
/// centred above it, scaled down for three or more digits.
///
/// The settings block is at a constant 8-aligned address, so only the
/// compiled code's aligned copy loop is reachable (its `ldl/ldr` twin is
/// dead code).
pub fn hud_draw_speedometer(c: &mut Cpu) {
    use speedo::*;
    guest(c, |c| {
        let p = |a: u32, o: u32| a.wrapping_add(o);
        c.cycles += 25;
        let frame = Frame::open(c, 0x3d0);
        frame.save(c, S1, 0x3a0);
        frame.save(c, S2, 0x390);
        let rider = r32(c, A2);
        set_reg(c, S1, reg(c, A2));
        frame.save(c, S4, 0x370);
        let hud = r32(c, A0);
        set_reg(c, S2, reg(c, A0));
        let sp0 = sp(c);
        Addr(sp0 + 0x310).sw(c, reg(c, A3));
        let elems = r32(c, A1);
        set_reg(c, S4, reg(c, A1));
        frame.save(c, S0, 0x3b0);
        frame.save(c, S3, 0x380);
        frame.save(c, S5, 0x360);
        frame.save(c, S6, 0x350);
        frame.save(c, S7, 0x340);
        frame.save(c, FP, 0x330);
        frame.save_ra(c, 0x320);
        frame.save_fpr(c, 20, 0x3c0);
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0022_0104)?;
        set_reg(c, A0, reg(c, V0));
        set_reg(c, A1, 4);
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0022_0110)?;

        // Copy the settings to sp+0 (32 bytes per step).
        set_reg(c, V1, 0x53_0000);
        set_reg(c, A3, sx(SETTINGS));
        set_reg(c, V0, 0); // SETTINGS & 7
        set_reg(c, A0, sx(sp0));
        c.cycles += 13;
        let (label_pos, label_size) = (sp0 + 0x2d0, sp0 + 0x2e0);
        set_reg(c, T2, sx(label_pos));
        set_reg(c, T3, sx(label_size));
        Addr(sp0 + 0x314).sw(c, sx(label_pos));
        let end = SETTINGS + SETTINGS_SIZE;
        set_reg(c, V0, sx(end));
        Addr(sp0 + 0x318).sw(c, sx(label_size));
        set_reg(c, A1, 0x4a_0000);
        set_reg(c, T0, 0x4a_0000);
        let velocity = p(rider, RIDER_VELOCITY);
        set_reg(c, A2, sx(velocity));
        let elem = p(elems, ELEM);
        set_reg(c, S0, sx(elem));
        let (pos, scale, color, number) = (sp0 + 0x290, sp0 + 0x2a0, sp0 + 0x2b0, sp0 + 0x2f0);
        set_reg(c, S5, sx(pos));
        set_reg(c, S6, sx(scale));
        set_reg(c, S7, sx(color));
        set_reg(c, S3, sx(number));
        let mut from = SETTINGS;
        let mut to = sp0;
        loop {
            c.cycles += 11;
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                let v = c.ld(from + 8 * k as u32);
                set_reg(c, r, v);
            }
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                c.sd(to + 8 * k as u32, reg(c, r));
            }
            from += 0x20;
            to += 0x20;
            set_reg(c, A3, sx(from));
            set_reg(c, A0, sx(to));
            if from == end {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, COPY_HEAD);
            }
        }
        c.cycles += 1;
        let vt = Addr(velocity).lw(c);
        set_reg(c, V1, vt);

        // The tail doubleword; then the velocity vector.
        c.cycles += 26;
        set_reg(c, FP, sx(LABEL_B));
        c.ldl(V0, from + 7);
        c.ldr(V0, from);
        c.sdl(V0, to + 7);
        set_reg(c, S1, sx(LABEL_A));
        c.sdr(V0, to);
        let slot = VSlot::read(c, vt, SLOT_GET_VELOCITY);
        set_reg(c, A0, slot.delta);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(velocity)));
        call_ptr(c, slot.func as u32, 0x0022_0238)?;
        // |v| with VU0 (vf6.w = 1), into f0, times the speed scale.
        c.lqc2(3, r32(c, V0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(GP_NEG_SPEED_SCALE));
        let units = Addr(sp0).lw(c) & UNITS_FLAG;
        set_reg(c, V1, UNITS_FLAG);
        set_reg(c, V0, units);
        c.mul_s(20, 0, 1);
        let mut label = LABEL_A;
        if units == 0 {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_UNITS_FACTOR));
            set_reg(c, S1, sx(LABEL_B));
            label = LABEL_B;
            c.mul_s(20, 20, 0);
        }

        // The label: element position, scale and colour; measure; align; draw.
        c.cycles += 71;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(pos));
        call_ptr(c, HUD_GET_ELEM_POS, 0x0022_0290)?;
        set_reg(c, A1, sx(scale));
        set_reg(c, A0, sx(elem));
        call_ptr(c, HUD_GET_ELEM_SCALE, 0x0022_029c)?;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(color));
        call_ptr(c, HUD_GET_ELEM_COLOR, 0x0022_02a8)?;
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V1, sx(text));
        set_reg(c, A2, sx(sp0 + 0x2c0));
        set_reg(c, A1, sx(label));
        set_reg(c, A3, 0);
        c.ldl(T3, color + 7);
        c.ldr(T3, color);
        c.ldl(V0, color + 0xf);
        c.ldr(V0, color + 8);
        c.sdl(T3, p(text, TEXT_COLOR + 7));
        c.sdr(T3, p(text, TEXT_COLOR));
        c.sdl(V0, p(text, TEXT_COLOR + 0xf));
        c.sdr(V0, p(text, TEXT_COLOR + 8));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(1, scale);
        c.lwc1(0, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(text, TEXT_SCALE));
        c.lwc1(1, scale + 4);
        c.mul_s(2, 2, 1);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, A0, sx(text));
        c.lwc1(12, p(text, TEXT_SCALE));
        c.lwc1(13, p(text, TEXT_SCALE + 4));
        call_ptr(c, FONT_MEASURE_TEXT, 0x0022_030c)?;
        c.lwc1(1, sp0 + 0x2c8);
        set_reg(c, T0, sx(number));
        c.lwc1(2, sp0 + 0x2cc);
        set_reg(c, A2, sx(pos));
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        set_reg(c, A0, sx(hud));
        let ay = Addr(p(elems, ELEM_ALIGN_Y)).lb(c);
        set_reg(c, T2, ay);
        let ax = Addr(p(elems, ELEM_ALIGN_X)).lb(c);
        set_reg(c, T1, ax);
        let v = Addr(sp0 + 0x314).lw(c);
        set_reg(c, A1, v);
        let v = Addr(sp0 + 0x310).lw(c);
        set_reg(c, T3, v);
        let v = Addr(sp0 + 0x318).lw(c);
        set_reg(c, A3, v);
        c.swc1(1, label_size);
        c.swc1(0, number + 4);
        c.swc1(0, number);
        c.swc1(2, label_size + 4);
        call_ptr(c, HUD_ALIGN_ELEM_POS, 0x0022_0350)?;
        c.lwc1(12, label_pos);
        set_reg(c, A1, sx(label));
        c.lwc1(13, label_pos + 4);
        let v = Addr(p(hud, HUD_TEXT)).lw(c);
        set_reg(c, A0, v);
        call_ptr(c, FONT_DRAW_TEXT, 0x0022_0364)?;

        // The number: sprintf("%d", (int)(speed + 0.5)).
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 0);
        set_reg(c, T1, 0x4a_0000);
        set_reg(c, A1, sx(NUMBER_FORMAT));
        c.add_s(0, 20, 0);
        c.cvt_w_s(1, 0);
        c.mfc1(A2, 1);
        set_reg(c, A0, sx(number));
        call_ptr(c, SPRINTF, 0x0022_0388)?;
        set_reg(c, A0, sx(number));
        call_ptr(c, STRLEN, 0x0022_0390)?;
        let short = reg(c, V0) < 3;
        set_reg(c, V0, short as u64);
        c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_SHORT));
        c.lwc1(1, scale);
        if !short {
            c.cycles += 1;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_LONG));
        }
        c.cycles += 62;
        c.lwc1(2, scale + 4);
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 0);
        c.swc1(1, scale);
        c.swc1(2, scale + 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        set_reg(c, A1, sx(number));
        c.lwc1(0, scale);
        c.lwc1(1, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(1, 1, 0);
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 4);
        c.lwc1(5, gp.wrapping_sub(GP_NEG_LABEL_OFFSET));
        c.swc1(1, p(text, TEXT_SCALE));
        c.lwc1(0, scale + 4);
        c.mul_s(2, 2, 0);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        // x = label x + width / 2; y = label y - (line * scale.y * sy / 2 + off).
        c.lwc1(0, label_size);
        c.lwc1(12, label_pos);
        c.mul_s(0, 0, 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(3, scale + 4);
        c.lwc1(2, label_pos + 4);
        set_reg(c, A0, sx(text));
        c.add_s(12, 12, 0);
        c.swc1(12, label_pos);
        c.lwc1(0, p(text, TEXT_LINE));
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.mul_s(0, 0, 3);
        c.mul_s(0, 0, 4);
        c.add_s(0, 0, 5);
        c.sub_s(2, 2, 0);
        c.mov_s(13, 2);
        c.swc1(2, label_pos + 4);
        call_ptr(c, FONT_DRAW_TEXT_CENTERED, 0x0022_043c)?;
        // Restore the text scale (base * 1).
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        let unit = sp0 + 0x300;
        c.swc1(0, unit + 4);
        c.swc1(0, unit);
        c.lwc1(2, p(text, TEXT_BASE_SCALE));
        c.lwc1(0, p(text, TEXT_BASE_SCALE + 4));
        c.swc1(2, p(text, TEXT_SCALE));
        c.lwc1(1, unit + 4);
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x3b0);
        frame.restore(c, S1, 0x3a0);
        frame.restore(c, S2, 0x390);
        frame.restore(c, S3, 0x380);
        frame.restore(c, S4, 0x370);
        frame.restore(c, S5, 0x360);
        frame.restore(c, S6, 0x350);
        frame.restore(c, S7, 0x340);
        frame.restore(c, FP, 0x330);
        frame.restore_ra(c, 0x320);
        frame.restore_fpr(c, 20, 0x3c0);
        c.swc1(0, p(text, TEXT_SCALE + 4));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_GetChunk (0x3e12e0)

mod chunk {
    use super::{ee, Callee};
    /// Stream manager (`*handle`): the ring start (`+0x30`) and its mutex
    /// (`+4`).
    pub const MAN_MUTEX: u32 = 0x4;
    pub const MAN_RING: u32 = 0x30;
    /// Stream handle: its tag (`+4`), the bytes still queued (`+8`), the
    /// next chunk (`+0xc`).
    pub const H_TAG: u32 = 0x4;
    pub const H_QUEUED: u32 = 0x8;
    pub const H_NEXT: u32 = 0xc;
    /// Chunk header: `+0` -1 = wrap to the ring start; `+4` tag (top byte)
    /// and size (low 24 bits).
    pub const CH_WRAP: u32 = 0x0;
    pub const CH_TAG_SIZE: u32 = 0x4;
    pub const SIZE_MASK: u64 = 0xff_ffff;
    pub const TAG_MASK: u64 = 0xffff_ffff_ff00_0000;
    pub const HEAD: u32 = 0x003e_13a0;

    pub const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };
    pub const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };
    pub const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
}

/// `u8* StreamFile_GetChunk(StreamFile* handle)` (0x3e12e0): takes the
/// handle's next queued chunk. Null for an invalid handle or an empty
/// queue. Otherwise clears the chunk's tag byte (keeping its size),
/// subtracts the size from the queued count under the manager's mutex and,
/// when data remains, walks the ring (chunks by size, a -1 word wraps to the
/// ring start) to the next chunk carrying this handle's tag; returns the
/// chunk taken.
pub fn stream_file_get_chunk(c: &mut Cpu) {
    use chunk::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        let sp0 = sp(c);
        set_reg(c, A1, sx(sp0));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        set_reg(c, A2, sx(sp0) | 4);
        STREAM_FILE_VALIDATE.call(c, 0x003e_1300)?;
        let invalid = reg(c, V0) != 0;
        set_reg(c, V0, 0);
        'out: {
            if invalid {
                break 'out;
            }
            c.cycles += 4;
            let h = Addr(sp0 + 4).lw(c) as u32;
            set_reg(c, V1, sx(h));
            let queued = Addr(h + H_QUEUED).lw(c);
            set_reg(c, V0, queued);
            if queued == 0 {
                c.cycles += 2;
                set_reg(c, V0, 0);
                break 'out;
            }
            let chunk = Addr(h + H_NEXT).lw(c) as u32;
            set_reg(c, S2, sx(chunk));
            c.cycles += 17;
            set_reg(c, V0, SIZE_MASK);
            let ts = Addr(chunk + CH_TAG_SIZE).lw(c);
            set_reg(c, V1, ts);
            let size = ts & SIZE_MASK;
            set_reg(c, S1, size);
            Addr(chunk + CH_TAG_SIZE).sw(c, size);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_LOCK.call(c, 0x003e_1340)?;
            let h = Addr(sp0 + 4).lw(c);
            set_reg(c, V0, h);
            let h = h as u32;
            let left = sx((Addr(h + H_QUEUED).lw(c) as u32).wrapping_sub(size as u32));
            set_reg(c, S0, left);
            Addr(h + H_QUEUED).sw(c, left);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_UNLOCK.call(c, 0x003e_135c)?;
            let mut at = sx(chunk.wrapping_add(size as u32));
            set_reg(c, A2, at);
            if (left as i64) > 0 {
                c.cycles += 8;
                let h = Addr(sp0 + 4).lw(c);
                set_reg(c, A1, h);
                let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                set_reg(c, V0, tag_word);
                set_reg(c, A0, TAG_MASK);
                let tag = sx((Addr((h as u32).wrapping_add(H_TAG)).lw(c) as u32) << 24);
                set_reg(c, V1, tag);
                set_reg(c, T1, h);
                if tag_word != tag {
                    c.cycles += 7;
                    set_reg(c, A0, 0xff_0000);
                    let man = Addr(sp0).lw(c);
                    set_reg(c, A1, man);
                    set_reg(c, T0, u64::MAX);
                    set_reg(c, A0, SIZE_MASK);
                    set_reg(c, A3, TAG_MASK);
                    let mut first = Addr(at as u32 + CH_WRAP).lw(c);
                    set_reg(c, V0, first);
                    loop {
                        c.cycles += 2;
                        if first != u64::MAX {
                            let ts = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c);
                            set_reg(c, V0, ts);
                            c.cycles += 2;
                            let size = ts & SIZE_MASK;
                            set_reg(c, S1, size);
                            at = addu(at, size);
                        } else {
                            c.cycles += 2;
                            at = Addr((man as u32).wrapping_add(MAN_RING)).lw(c);
                        }
                        set_reg(c, A2, at);
                        c.cycles += 4;
                        let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                        set_reg(c, V0, tag_word);
                        if tag_word == tag {
                            break;
                        }
                        first = Addr(at as u32 + CH_WRAP).lw(c);
                        set_reg(c, V0, first);
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                }
                c.cycles += 1;
                Addr((h as u32).wrapping_add(H_NEXT)).sw(c, at);
            }
            c.cycles += 1;
            set_reg(c, V0, sx(chunk));
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_v91 (0x37dbe8): the clip code of a point

mod clip {
    use super::{ee, Callee};
    /// `PS2GraphicsMan` fields: the world-to-clip matrix, the clip bounds
    /// (min x, min y, max z, ..., max x, max y, min z) and the "view matrices
    /// valid" flag.
    pub const CLIP_MATRIX: u32 = 0x6b30;
    pub const MIN_X: u32 = 0x6b70;
    pub const MIN_Y: u32 = 0x6b74;
    pub const MAX_Z: u32 = 0x6b78;
    pub const MAX_X: u32 = 0x6b80;
    pub const MAX_Y: u32 = 0x6b84;
    pub const MIN_Z: u32 = 0x6b88;
    pub const VIEW_VALID: u32 = 0x6b90;

    pub const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };
}

/// `int PS2GraphicsMan_v91(PS2GraphicsMan* self, const Vec4* p)` (0x37dbe8):
/// the clip outcode of a point. Rebuilds the view matrices if stale,
/// transforms `p` by the world-to-clip matrix (VU0), divides x and y by w
/// and sets bits for x below/above (2/1), y below/above (8/4) the bounds;
/// behind the eye (`1/w < 0`) flips the code with `^ 0x2f`, otherwise z
/// above/below adds 0x20/0x10.
pub fn ps2_graphics_man_clip_code(c: &mut Cpu) {
    use clip::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let gfx = r32(c, A0);
        set_reg(c, S0, reg(c, A0));
        frame.save_ra(c, 0x20);
        let valid = Addr(gfx + VIEW_VALID).lw(c);
        set_reg(c, V0, valid);
        let p = r32(c, A1);
        set_reg(c, S1, reg(c, A1));
        if valid == 0 {
            c.cycles += 2;
            UPDATE_VIEW_MATRICES.call(c, 0x0037_dc10)?;
        }
        c.cycles += 28;
        let m = gfx + CLIP_MATRIX;
        set_reg(c, V0, sx(m));
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 3);
        c.lqc2(8, p);
        for k in 0..4u32 {
            c.lqc2(4 + k as usize, m + 0x10 * k);
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        let sp0 = sp(c);
        c.sqc2(12, sp0 + 0x10);
        let mut code: u64 = 0;
        set_reg(c, V1, 0);
        c.lq(V0, sp0 + 0x10);
        c.lwc1(4, gfx + MIN_X);
        c.sq(V0, sp0);
        // v.w = 1/w; v.x *= v.w; v.y *= v.w.
        c.lwc1(0, sp0 + 0xc);
        c.lwc1(1, sp0);
        c.div_s(3, 3, 0);
        c.lwc1(2, sp0 + 4);
        c.mul_s(1, 1, 3);
        c.swc1(3, sp0 + 0xc);
        c.mul_s(2, 2, 3);
        c.c_lt_s(1, 4);
        c.swc1(1, sp0);
        let below_x = c.fcc();
        c.swc1(2, sp0 + 4);
        if below_x {
            c.cycles += 1;
            code = 2;
        }
        c.cycles += 5;
        c.lwc1(0, gfx + MAX_X);
        c.c_lt_s(0, 1);
        if c.fcc() {
            code |= 1;
        }
        c.cycles += 5;
        c.lwc1(0, gfx + MIN_Y);
        c.c_lt_s(2, 0);
        if c.fcc() {
            code |= 8;
        }
        c.cycles += 5;
        c.lwc1(0, gfx + MAX_Y);
        c.c_lt_s(0, 2);
        if c.fcc() {
            code |= 4;
        }
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(3, 0);
        let behind = c.fcc();
        c.lwc1(1, sp0 + 8);
        if behind {
            c.cycles += 2;
            code ^= 0x2f;
        } else {
            c.cycles += 5;
            c.lwc1(0, gfx + MAX_Z);
            c.c_lt_s(0, 1);
            if c.fcc() {
                code |= 0x20;
            }
            c.cycles += 5;
            c.lwc1(0, gfx + MIN_Z);
            c.c_lt_s(1, 0);
            if c.fcc() {
                code |= 0x10;
            }
        }
        c.cycles += 6;
        set_reg(c, V1, code);
        frame.restore(c, S0, 0x40);
        set_reg(c, V0, code);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check_lohi, elf_image, fill, put_words, Rng, DATA, GAME_GP};

    /// SND_ReleaseFinishedVoices over random channel groups: keys zero or
    /// not, timed stops pending or not, seen keys equal or changed (with a
    /// key whose low 23 bits are zero), remaining 0 / -1 or not, 0-3 voices
    /// per channel, mode 2 or not; the real `SND_ChannelRelease` runs on
    /// the same channel table.
    #[test]
    fn snd_release_finished_voices_matches() {
        use super::release::*;
        check_lohi(
            ee::s13::f_003c1298__generated,
            snd_release_finished_voices,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3c_1298, 0);
                c.pc = 0x003c_1298;
                let count_a = r.next_u32() % 5;
                let count_b = r.next_u32() % 4;
                let total = (count_a + count_b).max(1);
                let chans = DATA;
                let status = DATA + 0x4000;
                c.bus.write32(STATE + CHANNELS, chans);
                c.bus.write8(STATE + COUNT_A, count_a as u8);
                c.bus.write8(STATE + COUNT_B, count_b as u8);
                let mode = if r.next_u32() % 2 == 0 {
                    2
                } else {
                    r.next_u32() as u8
                };
                c.bus.write8(STATE + MODE, mode);
                c.bus
                    .write16(crate::sound::snd::CHANNEL_COUNT.0, total as u16);
                c.bus.write32(VOICES + STATUS, status);
                fill(c, &mut r, status - 0x100, 0x500 / 4);
                for i in 0..total {
                    let ch = chans + i * CHANNEL_SIZE;
                    fill(c, &mut r, ch, CHANNEL_SIZE / 4);
                    let stop = if r.next_u32() % 4 != 0 {
                        0xffff
                    } else {
                        r.next_u32() as u16
                    };
                    c.bus.write16(ch + CH_STOP, stop);
                    let n = r.next_u32() % 4;
                    c.bus.write8(ch + CH_VOICE_COUNT, n as u8);
                    for j in 0..4 {
                        c.bus
                            .write16(ch + CH_VOICES + 2 * j, (r.next_u32() % total) as u16);
                    }
                    // SND_ChannelRelease's fields: group, allocated state.
                    c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
                    c.bus.write8(ch + 0x69, (r.next_u32() % 3) as u8);
                }
                let key = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    1 => 0x0080_0000 * (r.next_u32() % 4),
                    _ => r.next_u32(),
                };
                for i in 0..count_a {
                    let k = key(&mut r);
                    c.bus.write32(status + KEY_A + 8 * i, k);
                    let seen = VOICES + SEEN_A + 8 * i;
                    c.bus.write32(
                        seen,
                        if r.next_u32() % 2 == 0 {
                            k
                        } else {
                            r.next_u32()
                        },
                    );
                    c.bus.write32(
                        seen + 4,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32()
                        },
                    );
                }
                for i in 0..count_b {
                    let k = key(&mut r);
                    c.bus.write32(status + KEY_B + 16 * i, k);
                    let seen = VOICES + SEEN_B + 8 * i;
                    c.bus.write32(
                        seen,
                        if r.next_u32() % 2 == 0 {
                            k
                        } else {
                            r.next_u32()
                        },
                    );
                    c.bus.write32(
                        seen + 4,
                        if r.next_u32() % 2 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32()
                        },
                    );
                }
            },
        );
    }

    /// VfxPlants_Update with the game image loaded (real sinf/cosf/sincos),
    /// random tweakables, axis and base matrix, the manager's clock method a
    /// fake (forced small clock; it unwinds for some seeds).
    #[test]
    fn vfx_plants_update_matches() {
        use super::plants::*;
        covered(ee::s12::f_0038ee78__generated, vfx_plants_update, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x38_ee78, 1);
            elf_image(c);
            c.pc = 0x0038_ee78;
            let gp = GAME_GP;
            let manager = ptr(&mut r);
            c.bus.write32(gp + GP_MANAGER, manager);
            let vt = vtable(c, &mut r, &[SLOT_CLOCK]);
            c.bus.write32(manager + MANAGER_VTABLE, vt);
            c.bus.write32(LOG_RESULTS, r.next_u32() % 100_000);
            for off in [
                GP_SWAY_AMP,
                GP_SWAY_BASE,
                GP_WOBBLE_AMP,
                GP_TWIST_AMP,
                GP_SWAY_DIR,
                GP_WOBBLE_DIR,
            ] {
                let v = r.f32(90.0);
                c.bus.write32(gp + off, v);
            }
            for off in [
                GP_SWAY_SPEED,
                GP_TWIST_SPEED,
                GP_WOBBLE_SPEED,
                GP_WOBBLE_PHASE,
            ] {
                let v = r.next_u32() % 2000;
                c.bus.write32(gp + off, v);
            }
            for k in 0..3 {
                let v = r.f32(1.0);
                c.bus.write32(AXIS + 4 * k, v);
            }
            for k in 0..16 {
                let v = r.f32(10.0);
                c.bus.write32(BASE + 4 * k, v);
            }
        });
    }

    /// A float in [lo, hi) as its bit pattern.
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// HUD_DrawSpeedometer: the BELibrary dummy and interface (a fake
    /// method), the rider's velocity method a fake returning a random vector,
    /// random settings (units bit both ways), HUD elements and text object as
    /// in the position-panel test, speeds giving one to three digits; real
    /// HUD/font/sprintf/strlen callees.
    #[test]
    fn hud_draw_speedometer_matches() {
        use super::speedo::*;
        covered(ee::s05::f_002200c0__generated, hud_draw_speedometer, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x22_00c0, 2);
            c.pc = 0x0022_00c0;
            let gp = GAME_GP;
            let (hud, elems, rider, text) = (DATA, DATA + 0x1000, DATA + 0x1c00, DATA + 0x800);
            // BELibrary singleton and interface slot 0 with a fake method.
            c.bus.write32(gp - 0x1f04, DATA + 0x30_0000);
            c.bus.write32(gp - 0x1efc, 0);
            let lib_if = DATA + 0x3000;
            c.bus.write32(0x0053_08a8, lib_if);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(lib_if + 4, vt);
            let vt2 = vtable(c, &mut r, &[SLOT_GET_VELOCITY]);
            c.bus.write32(rider + RIDER_VELOCITY, vt2);
            let vel = DATA + 0x2000;
            c.bus.write32(LOG_RESULTS + 4, vel);
            for k in 0..4 {
                let v = uf(&mut r, -70.0, 70.0);
                c.bus.write32(vel + 4 * k, v);
            }
            // The settings block (word 0: the units flag).
            fill(c, &mut r, SETTINGS, 0x288 / 4);
            let w0 = r.next_u32() & !(UNITS_FLAG as u32);
            c.bus.write32(
                SETTINGS,
                w0 | if seed % 2 == 0 { UNITS_FLAG as u32 } else { 0 },
            );
            put_cstr(c, NUMBER_FORMAT, b"%d");
            put_cstr(c, LABEL_A, b"MPH");
            put_cstr(c, LABEL_B, b"KPH");
            for off in [
                GP_NEG_SPEED_SCALE,
                GP_NEG_UNITS_FACTOR,
                GP_NEG_SCALE_SHORT,
                GP_NEG_SCALE_LONG,
                GP_NEG_LABEL_OFFSET,
            ] {
                let v = uf(&mut r, 0.2, 3.0);
                c.bus.write32(gp - off, v);
            }
            // The HUD, its text object and elements.
            c.bus.write32(hud + HUD_TEXT, text);
            c.bus.write32(text + TEXT_BASE_SCALE, uf(&mut r, 1.0, 20.0));
            c.bus
                .write32(text + TEXT_BASE_SCALE + 4, uf(&mut r, 1.0, 20.0));
            c.bus.write32(text + TEXT_LINE, r.next_u32() % 32);
            for i in 0..48u32 {
                let e = elems + 0x24 * i;
                c.bus.write16(e, (r.next_u32() % 640) as u16);
                c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                c.bus.write16(e + 4, (r.next_u32() % 64) as u16);
                c.bus.write16(e + 6, (r.next_u32() % 64) as u16);
                for k in 0..4 {
                    let v = uf(&mut r, 0.0, 1.0);
                    c.bus.write32(e + 0x10 + 4 * k, v);
                }
                c.bus.write8(e + 0x20, (r.next_u32() % 3) as u8);
                c.bus.write8(e + 0x21, (r.next_u32() % 3) as u8);
                c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
            }
            // Font_DrawText's globals (as in the position-panel test).
            c.bus.write32(gp - 0xf28, DATA + 0x1800);
            fill(c, &mut r, DATA + 0x1800, 4);
            c.bus.write32(gp - 0x854, DATA + 0x1900);
            c.bus.write32(DATA + 0x1900 + 0xe84, DATA + 0x1a00);
            c.bus.write32(DATA + 0x1a00 + 8, r.next_u32());
            c.g[A0][0] = sx(hud);
            c.g[A1][0] = sx(elems);
            c.g[A2][0] = sx(rider);
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    /// StreamFile_GetChunk: null / bad-magic / valid handles, empty or
    /// non-empty queues, a ring of 2-6 chunks plus a wrap marker with this
    /// handle's tag somewhere other than the chunk taken; the mutex's kernel
    /// calls are fakes (some unwind).
    #[test]
    fn stream_file_get_chunk_matches() {
        use super::chunk::*;
        covered(
            ee::s14::f_003e12e0__generated,
            stream_file_get_chunk,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_12e0, 4);
                c.pc = 0x003e_12e0;
                let (h, man, ring) = (DATA, DATA + 0x100, DATA + 0x1000);
                c.g[A0][0] = if seed % 13 == 0 { 0 } else { sx(h) };
                c.bus.write32(h, man);
                c.bus.write32(
                    man,
                    if seed % 11 == 0 {
                        r.next_u32()
                    } else {
                        0x4d52_5453
                    },
                );
                // The mutex at man+4: owner, depth, semaphore.
                c.bus.write32(man + 8, r.next_u32() % 4);
                c.bus.write32(man + 0xc, r.next_u32() % 3);
                c.bus.write32(man + 0x10, r.next_u32() % 64);
                c.bus.write32(man + MAN_RING, ring);
                let tag = 1 + r.next_u32() % 255;
                c.bus.write32(h + H_TAG, tag);
                let k = 2 + r.next_u32() % 5;
                let take = r.next_u32() % k;
                let mut hit = (take + 1 + r.next_u32() % (k - 1)) % k;
                if hit == take {
                    hit = (take + 1) % k;
                }
                let mut at = ring;
                let mut take_at = ring;
                let mut take_size = 0;
                for i in 0..k {
                    let size = 0x10 * (1 + r.next_u32() % 8);
                    let t = if i == hit || (i != take && r.next_u32() % 3 == 0) {
                        tag
                    } else {
                        (tag + 1 + r.next_u32() % 254) % 256
                    };
                    let w0 = r.next_u32() & 0x7fff_ffff;
                    c.bus.write32(at, w0);
                    c.bus.write32(at + CH_TAG_SIZE, t << 24 | size);
                    if i == take {
                        take_at = at;
                        take_size = size;
                    }
                    at += size;
                }
                c.bus.write32(at, u32::MAX);
                c.bus.write32(
                    at + CH_TAG_SIZE,
                    r.next_u32() & 0x00ff_ffff | 0x0100_0000 * ((tag + 7) % 256),
                );
                c.bus.write32(h + H_NEXT, take_at);
                let queued = match seed % 4 {
                    0 => 0,
                    1 => take_size,
                    2 => take_size + 0x10 * (1 + r.next_u32() % 16),
                    _ => r.next_u32() % 0x100,
                };
                c.bus.write32(h + H_QUEUED, queued);
            },
        );
    }

    /// PS2GraphicsMan_v91: random clip matrix, point and bounds (points
    /// inside and outside each bound, w of both signs); the stale-matrices
    /// path runs the real `PS2GraphicsMan_UpdateViewMatrices` on some seeds.
    #[test]
    fn ps2_graphics_man_clip_code_matches() {
        use super::clip::*;
        check(
            ee::s12::f_0037dbe8__generated,
            ps2_graphics_man_clip_code,
            300,
            &|c, seed| {
                let mut r = Rng(0x37_dbe8 ^ (seed as u64) << 12);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0037_dbe8;
                let gfx = DATA;
                let p = DATA + 0x8000;
                c.g[A0][0] = sx(gfx);
                c.g[A1][0] = sx(p);
                c.bus.write32(
                    gfx + VIEW_VALID,
                    if seed % 8 == 0 { 0 } else { r.next_u32() | 1 },
                );
                for k in 0..16 {
                    let v = if k % 5 == 0 {
                        uf(&mut r, 0.5, 2.0)
                    } else {
                        uf(&mut r, -1.0, 1.0)
                    };
                    c.bus.write32(gfx + CLIP_MATRIX + 4 * k, v);
                }
                for k in 0..3 {
                    let v = uf(&mut r, -2.0, 2.0);
                    c.bus.write32(p + 4 * k, v);
                }
                let w = if seed % 3 == 0 {
                    uf(&mut r, -2.0, -0.1)
                } else {
                    uf(&mut r, 0.1, 2.0)
                };
                c.bus.write32(p + 12, w);
                for off in [MIN_X, MIN_Y, MIN_Z] {
                    let v = uf(&mut r, -1.5, 0.0);
                    c.bus.write32(gfx + off, v);
                }
                for off in [MAX_X, MAX_Y, MAX_Z] {
                    let v = uf(&mut r, 0.0, 1.5);
                    c.bus.write32(gfx + off, v);
                }
            },
        );
    }
}
