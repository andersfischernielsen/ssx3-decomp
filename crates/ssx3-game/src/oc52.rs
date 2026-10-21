//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! The functions here are transcribed from the generated bodies (the
//! authority for registers, cycles and memory): the FPU/COP2 operations the
//! originals issue are re-issued on the same registers, so the values they
//! leave in `f0`/`f1`, the condition bit and VU0 are exact.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_0c60, vfx_screen_tint_update, "VfxScreenTint_Update"),
    (0x002f_00a0, ps2_visual_effects_man_apply_world_painter, "PS2VisualEffectsMan_ApplyWorldPainter"),
    (0x0023_1840, game_module_check_pause_request, "GameModule_CheckPauseRequest"),
    (0x0016_62a0, chase_camera_algorithm_smooth_pitch, "ChaseCameraAlgorithm_SmoothPitch"),
];

/// `gp + off` (a gp-relative global) as a guest address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `rd = rd + k` on a 32-bit register value (`addiu`).
#[inline(always)]
fn addiu(c: &mut Cpu, rd: usize, k: u32) {
    let v = (c.g[rd][0] as u32).wrapping_add(k);
    c.g[rd][0] = sx(v);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateFade (0x287c48).

/// The fade state of an `sSaMix7C48` (offsets from the object).
mod fade {
    /// Non-zero while a fade is running.
    pub const FADING: u32 = 0x63c0;
    /// Bits: which channels the fade drives (bit 2 = `chan[4]`/the mix).
    pub const MASK: u32 = 0x63c4;
    /// Per-frame step applied to the current value.
    pub const RATE: u32 = 0x63c8;
    /// Where the ramp stops.
    pub const TARGET: u32 = 0x63cc;
    /// The current fade value.
    pub const CUR: u32 = 0x63d0;
    /// `chan[4].out`, scaled by the fade for the mix levels.
    pub const CHAN4: u32 = 0x6300;
    /// Level written with the fade value before the mix levels.
    pub const MIX_LEVEL: u32 = 0x63f4;
    /// The ten output mix levels (`mix[0..10]`).
    pub const MIX: u32 = 0x636c;
}

/// One faded channel pair: `fadeCur` is stored at `out`, `gain * fadeCur` at
/// `src + 4` and `gain * fadeCur * tail` at `src + 8`; the mask is left in v0
/// for the next test (as the original does, block by block).
///
/// `out` is the level the channel's fade value is written to; `src` the pair
/// of gains (also where the two scaled values go); `tail` the second factor.
fn scale_channel(c: &mut Cpu, base: u32, out: u32, src: u32, tail: u32) {
    c.cycles += 9;
    c.lwc1(0, base.wrapping_add(fade::CUR));
    c.lwc1(1, base.wrapping_add(src));
    c.lwc1(2, base.wrapping_add(tail));
    c.mul_s(1, 1, 0);
    c.swc1(0, base.wrapping_add(out));
    c.mul_s(2, 1, 2);
    c.swc1(1, base.wrapping_add(src).wrapping_add(4));
    c.swc1(2, base.wrapping_add(src).wrapping_add(8));
    c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
}

/// Test mask bit `bit` (+`cost` cycles) and, when set, scale that channel.
/// Every path leaves the mask in v0 (the original reloads it either way).
fn mask_bit(c: &mut Cpu, base: u32, bit: u64, cost: u32, out: u32, src: u32, tail: u32) {
    c.cycles += cost as u64;
    let m = c.lw(base.wrapping_add(fade::MASK));
    if m & bit != 0 {
        scale_channel(c, base, out, src, tail);
    } else {
        c.g[V0][0] = m;
    }
}

/// `void func_00287C48(sSaMix *s)` (0x287c48): advance the audio fade ramp.
///
/// With `fading` set, `cur += rate` is stored and, once the ramp passes
/// `target` (below it for a negative rate, above it for a positive one), the
/// value is clamped to the target and `fading` cleared. Every channel bit of
/// the mask then scales the current value into its gain pair, and bit 2 also
/// rebuilds the ten output mix levels from `chan[2].out` (+0x6300) times the
/// constants at `gp+0x43c..0x460` (`D_004A352C..0x3550`).
pub fn ssx_audio_system_update_fade(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    let fading = c.lw(base.wrapping_add(fade::FADING));
    c.g[V0][0] = fading;
    if fading != 0 {
        // `cur += rate`
        c.cycles += 8;
        c.lwc1(0, base.wrapping_add(fade::CUR));
        c.lwc1(1, base.wrapping_add(fade::RATE));
        c.mtc1(0, 2); // f2 = 0.0
        c.add_s(0, 0, 1);
        c.c_lt_s(1, 2); // rate < 0
        let falling = c.fcc();
        c.swc1(0, base.wrapping_add(fade::CUR));
        if falling {
            // a falling ramp stops when cur < target
            c.cycles += 4;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(0, 1);
        } else {
            // a rising ramp stops when target < cur
            c.cycles += 3;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(1, 0);
        }
        c.cycles += 2;
        if c.fcc() {
            c.cycles += 3;
            c.swc1(1, base.wrapping_add(fade::CUR));
            c.sw(base.wrapping_add(fade::FADING), 0);
        }
        // Channels 0 and 1 only test their bit (their `set` was inlined away).
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));

        // Channel 4: the ten mix levels.
        c.cycles += 3;
        c.g[V0][0] &= 4;
        c.lwc1(1, gp(c, 0x43c));
        if c.g[V0][0] != 0 {
            c.cycles += 37;
            c.lwc1(12, base.wrapping_add(fade::CUR));
            c.lwc1(11, base.wrapping_add(fade::CHAN4));
            c.lwc1(0, base.wrapping_add(fade::MIX_LEVEL));
            c.mul_s(11, 11, 12);
            c.lwc1(2, gp(c, 0x440));
            c.lwc1(4, gp(c, 0x444));
            c.mul_s(0, 11, 0);
            c.lwc1(5, gp(c, 0x448));
            c.lwc1(7, gp(c, 0x44c));
            c.lwc1(9, gp(c, 0x450));
            c.lwc1(10, gp(c, 0x454));
            c.mul_s(1, 0, 1);
            c.lwc1(8, gp(c, 0x458));
            c.mul_s(2, 0, 2);
            c.lwc1(6, gp(c, 0x45c));
            c.mul_s(4, 0, 4);
            c.lwc1(3, gp(c, 0x460));
            c.mul_s(5, 0, 5);
            c.mul_s(7, 0, 7);
            c.swc1(1, base.wrapping_add(fade::MIX));
            c.mul_s(3, 0, 3);
            c.swc1(2, base.wrapping_add(fade::MIX).wrapping_add(4));
            c.mul_s(9, 0, 9);
            c.swc1(4, base.wrapping_add(fade::MIX).wrapping_add(8));
            c.mul_s(10, 0, 10);
            c.swc1(5, base.wrapping_add(fade::MIX).wrapping_add(0xc));
            c.mul_s(8, 0, 8);
            c.swc1(7, base.wrapping_add(fade::MIX).wrapping_add(0x10));
            c.mul_s(6, 0, 6);
            c.swc1(9, base.wrapping_add(fade::MIX).wrapping_add(0x14));
            c.swc1(10, base.wrapping_add(fade::MIX).wrapping_add(0x18));
            c.swc1(8, base.wrapping_add(fade::MIX).wrapping_add(0x1c));
            c.swc1(6, base.wrapping_add(fade::MIX).wrapping_add(0x20));
            c.swc1(3, base.wrapping_add(fade::MIX).wrapping_add(0x24));
            c.swc1(12, base.wrapping_add(0x63a4));
            c.swc1(11, base.wrapping_add(fade::CHAN4).wrapping_add(4));
            c.swc1(0, base.wrapping_add(fade::CHAN4).wrapping_add(8));
        }

        // Channel 2 (bit 8, one cycle more: it reloads the mask first).
        mask_bit(c, base, 8, 4, 0x6398, 0x62d0, 0x63e8);
        // Channel 3, 6, 7, 8, 9, 10.
        mask_bit(c, base, 0x80, 3, 0x63a8, 0x6310, 0x63f8);
        mask_bit(c, base, 0x20, 3, 0x63ac, 0x6320, 0x63fc);
        mask_bit(c, base, 0x40, 3, 0x63b0, 0x6330, 0x6400);
        mask_bit(c, base, 0x10, 3, 0x63b4, 0x6340, 0x6404);
        mask_bit(c, base, 0x100, 3, 0x63b8, 0x6350, 0x6408);

        // Channel 11 (bit 0x200): no mask reload, one cycle less.
        c.cycles += 3;
        c.g[V0][0] &= 0x200;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.lwc1(0, base.wrapping_add(fade::CUR));
            c.lwc1(1, base.wrapping_add(0x6360));
            c.lwc1(2, base.wrapping_add(0x640c));
            c.mul_s(1, 1, 0);
            c.swc1(0, base.wrapping_add(0x63bc));
            c.mul_s(2, 1, 2);
            c.swc1(1, base.wrapping_add(0x6364));
            c.swc1(2, base.wrapping_add(0x6368));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// VfxScreenTint_Update (0x390c60).

/// The screen-tint animation state (`gp` globals, `D_004A46xx`).
mod tint {
    /// Master enable (`D_004A460C`).
    pub const ENABLED: i32 = 0x151c;
    /// Countdown of the frames left before another flash may start
    /// (`D_004A46B4`).
    pub const COOLDOWN: i32 = 0x15c4;
    /// Flash timer (`D_004A4698`): -1 while idle, then counts up.
    pub const TIMER: i32 = 0x15a8;
    /// Trigger threshold (`D_004A4610`).
    pub const TRIGGER: i32 = 0x1520;
    /// Flash probability factor (`D_004A4614`).
    pub const CHANCE: i32 = 0x1524;
    /// Idle delay (`D_004A4618`).
    pub const D0: i32 = 0x1528;
    /// Second segment (`D_004A461C`).
    pub const D1: i32 = 0x152c;
    /// Third segment (`D_004A4620`).
    pub const D2: i32 = 0x1530;
    /// Fourth segment (`D_004A4624`).
    pub const D3: i32 = 0x1534;
}

/// The frame after the prologue (the part skipped when `ENABLED` is 0).
/// Returns true when it already reloaded `s0` (the original's early return
/// from the flash path, which skips the tail's `lq s0`).
fn tint_body(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    use tint::*;
    // Per-frame cooldown.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp(c, COOLDOWN));
    let cooled = (c.g[V0][0] as i64) <= 0;
    addiu(c, V0, 0xffff_ffff);
    if !cooled {
        c.cycles += 1;
        c.sw(gp(c, COOLDOWN), c.g[V0][0]);
    }

    // A running flash: pick the next one at random, else leave the timer
    // counting up to the trigger.
    c.cycles += 3;
    c.g[A3][0] = c.lw(gp(c, TIMER));
    let idle = (c.g[A3][0] as i64) >= 0;
    c.g[A2][0] = c.lw(gp(c, TRIGGER));
    if !idle {
        // `RandF() < chance^2` decides whether a flash fires now.
        c.cycles += 16;
        c.g[S0][0] = 0x007f_0000;
        call_ptr(c, 0x0031_77f0, 0x0039_0ca0)?;
        c.g[S0][0] |= 0xffff;
        c.g[S1][0] = 0x3f80_0000;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(0, gp(c, CHANCE));
        c.mtc1(2, 2);
        c.mul_s(0, 0, 0);
        c.sub_s(1, 2, 20);
        c.c_lt_s(1, 0);
        if !c.fcc() {
            frame.restore(c, S0, 0x20);
            return Ok(true);
        }
        // Flash: duration `(rand - 1) * K0 + K1` scaled by K2.
        c.cycles += 15;
        call_ptr(c, 0x0031_77f0, 0x0039_0ce0)?;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.lwc1(0, gp(c, -0x26a4));
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(2, gp(c, -0x26a0));
        c.mtc1(2, 3);
        c.lwc1(1, gp(c, -0x269c));
        c.sub_s(12, 3, 20);
        c.mul_s(12, 12, 0);
        c.add_s(12, 12, 2);
        c.mul_s(12, 12, 1);
        Callee { addr: 0x0039_0ec8, f: ssx3_ee::s12::VfxScreenTint_TriggerFlash }.call(c, 0x0039_0d0c)?;
        frame.restore(c, S0, 0x20);
        return Ok(true);
    }

    // The flash timer: count up to the trigger or clamp to it.
    c.cycles += 2;
    let trig_neg = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = c.lw(gp(c, D0));
    let advance;
    if trig_neg {
        advance = true;
    } else {
        c.cycles += 9;
        c.g[V1][0] = c.lw(gp(c, D1));
        c.g[A1][0] = c.lw(gp(c, D2));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw(gp(c, D3));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.sw(gp(c, TIMER), c.g[A2][0]);
            advance = false;
        } else {
            advance = true;
        }
    }
    if advance {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.sw(gp(c, TIMER), c.g[V0][0]);
    }

    // The tint level and colours for the current segment of the flash.
    c.cycles += 5;
    c.g[V1][0] = c.lw(gp(c, TIMER));
    c.g[A1][0] = c.lw(gp(c, D0));
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let past_first = c.g[V0][0] != 0;
    c.g[T0][0] = c.lw(gp(c, D1));
    if !past_first {
        // Ramp up over the first segment: level = (timer / d0) * K.
        c.cycles += 17;
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(5, gp(c, 0x1538));
        c.mtc1(5, 2);
        c.cvt_s_w(2, 2);
        c.lwc1(3, gp(c, 0x1544));
        c.lwc1(4, gp(c, 0x1548));
        c.g[V0][0] = 5;
        c.lwc1(1, gp(c, 0x154c));
        c.div_s(0, 0, 2);
        c.swc1(3, gp(c, 0x15b0));
        c.swc1(4, gp(c, 0x15b4));
        c.swc1(1, gp(c, 0x15b8));
        c.sw(gp(c, 0x15bc), c.g[V0][0]);
        c.mul_s(0, 0, 5);
        c.swc1(0, gp(c, 0x15ac));
        return Ok(false);
    } else {
        // Second segment: its peak colours.
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let third = c.g[V0][0] != 0;
        c.g[A2][0] = c.lw(gp(c, D2));
        if !third {
            c.cycles += 6;
            c.lwc1(3, gp(c, 0x1538));
            c.g[V0][0] = 5;
            c.lwc1(0, gp(c, 0x1544));
            c.lwc1(1, gp(c, 0x1548));
            c.lwc1(2, gp(c, 0x154c));
        } else {
            // Third segment.
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let tail = c.g[V0][0] != 0;
            c.g[A3][0] = c.lw(gp(c, D3));
            if !tail {
                // Third segment: its peak colours.
                c.cycles += 5;
                c.lwc1(3, gp(c, 0x153c));
                c.g[V0][0] = 5;
                c.lwc1(0, gp(c, 0x1550));
                c.lwc1(1, gp(c, 0x1554));
                c.lwc1(2, gp(c, 0x1558));
            } else {
                // Past the third segment: the fourth, or back to idle.
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let past_end = c.g[V0][0] != 0;
                c.lwc1(2, gp(c, 0x14f4));
                if !past_end {
                    // Last segment: level ramps down from 1.
                    c.cycles += 29;
                    c.mtc1(5, 2);
                    c.cvt_s_w(2, 2);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.lwc1(8, gp(c, 0x1540));
                    c.mtc1(8, 6);
                    c.cvt_s_w(6, 6);
                    c.lwc1(4, gp(c, 0x155c));
                    c.mtc1(6, 7);
                    c.cvt_s_w(7, 7);
                    c.lwc1(5, gp(c, 0x1560));
                    c.sub_s(0, 0, 2);
                    c.lwc1(3, gp(c, 0x1564));
                    c.mtc1(7, 2);
                    c.cvt_s_w(2, 2);
                    c.g[V0][0] = 5;
                    c.swc1(4, gp(c, 0x15b0));
                    c.sub_s(0, 0, 6);
                    c.swc1(5, gp(c, 0x15b4));
                    c.swc1(3, gp(c, 0x15b8));
                    c.sw(gp(c, 0x15bc), c.g[V0][0]);
                    c.sub_s(0, 0, 7);
                    c.div_s(0, 0, 2);
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 8);
                    c.swc1(1, gp(c, 0x15ac));
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    c.lwc1(1, gp(c, 0x14f8));
                    c.g[V1][0] = 5;
                    c.lwc1(0, gp(c, 0x14fc));
                    c.sw(gp(c, TIMER), c.g[V0][0]);
                    c.swc1(2, gp(c, 0x15b0));
                    c.swc1(1, gp(c, 0x15b4));
                    c.swc1(0, gp(c, 0x15b8));
                    c.sw(gp(c, 0x15bc), c.g[V1][0]);
                    c.sw(gp(c, 0x15ac), 0);
                }
                return Ok(false);
            }
        }
    }

    // Segment colours written for the second, third and fourth segments.
    c.cycles += 6;
    c.swc1(3, gp(c, 0x15ac));
    c.swc1(0, gp(c, 0x15b0));
    c.swc1(1, gp(c, 0x15b4));
    c.swc1(2, gp(c, 0x15b8));
    c.sw(gp(c, 0x15bc), c.g[V0][0]);
    Ok(false)
}

/// `void func_00390C60(void)` (0x390c60): animate the light manager's screen
/// tint. With `D_004A460C` set, tick the cooldown, and every frame either
/// roll for a new flash (`RandF() < D_004A4614^2`, with a random duration and
/// colour) or step the running flash through its four segments, writing the
/// level (+0x15ac) and colours (+0x15b0..+0x15bc).
///
/// Frame 0x40: s0 at +0x20, s1 at +0x10, ra at 0, f20 at +0x30.
pub fn vfx_screen_tint_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, tint::ENABLED));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.swc1(20, frame.at(c, 0x30));
        let reloaded = if c.g[V0][0] != 0 { tint_body(c, frame)? } else { false };
        if !reloaded {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` on a guest pointer (the `lw`/`sw` offsets).
#[inline(always)]
fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// PS2VisualEffectsMan_ApplyWorldPainter (0x2f00a0).

/// `gp` globals the world-painter copy reads (`D_0050xxxx` and friends).
mod painter {
    /// Painter descriptor array base; accessors index it with a0.
    pub const DESCS: u32 = 0x004f_a370;
    /// Graphics manager (`*(gp + 0x2a90)`).
    pub const GFX: i32 = 0x2a90;
    /// "Applied" flag pair.
    pub const READY: i32 = 0x1238;
    pub const HAVE_TINT: i32 = 0x14ec;
}

/// One `WorldPainter_Get*` accessor: the painter index goes in a0, the value
/// comes back in f0.
fn painter_get(c: &mut Cpu, addr: u32, ret: u32, f: GuestFn, index: u64) -> Flow {
    c.g[A0][0] = index;
    Callee { addr, f }.call(c, ret)
}

/// Store the previous accessor's result at `cache + slot + off` and call the
/// next accessor (the interleaved store/call pattern of the cache loop).
fn painter_step(
    c: &mut Cpu,
    index: u64,
    cache: u64,
    slot: u64,
    off: u32,
    addr: u32,
    ret: u32,
    f: GuestFn,
) -> Flow {
    c.g[V0][0] = sx((cache as u32).wrapping_add(slot as u32));
    c.g[A0][0] = index;
    let dst = (c.g[V0][0] as u32).wrapping_add(off);
    c.swc1(0, dst);
    Callee { addr, f }.call(c, ret)
}

/// `void PS2VisualEffectsMan_ApplyWorldPainter(cache, int index)` (0x2f00a0):
/// copy the world painter's parameters for `index` into the effects
/// manager's per-index record.
///
/// The first time (`D_004A5D38 == 0`) the fog, screen-tint and weather values
/// are latched into the gp globals and a 4-float record (`+0x4720`), and the
/// tint-enable flag is set. Then the light-glow values are latched (unless
/// `D_004A63CC` says otherwise) and the record at `cache + index*4` gets the
/// surface, camera, fog, tint and fill values at `+0x8c..+0x24c`.
///
/// Frame 0x60: s1 at +0x30, s2 at +0x20, s0 at +0x40, ra at +0x10, f21 at
/// +0x58, f20 at +0x50.
pub fn ps2_visual_effects_man_apply_world_painter(c: &mut Cpu) {
    use ssx3_ee::s09 as wp;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.lw(gp(c, painter::READY));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        c.swc1(21, frame.at(c, 0x58));
        c.swc1(20, frame.at(c, 0x50));
        let applied = c.g[V0][0] != 0;
        c.g[S2][0] = c.lw(gp(c, painter::GFX));
        let index = c.g[S1][0];
        if !applied {
            // First use: latch the fog / tint / weather parameters.
            c.cycles += 35;
            c.g[S0][0] = 0x0050_0000;
            painter_get(c, 0x002e_e9c0, 0x002f_00d8, wp::WorldPainter_GetFog08, index)?;
            c.g[V0][0] = 1;
            c.sw(gp(c, 0x1244), c.g[V0][0]);
            c.swc1(0, gp(c, 0x1248));
            painter_get(c, 0x002e_ea08, 0x002f_00ec, wp::WorldPainter_GetFog10, index)?;
            c.swc1(0, gp(c, 0x123c));
            painter_get(c, 0x002e_ea50, 0x002f_00f8, wp::WorldPainter_GetFog18, index)?;
            c.swc1(0, gp(c, 0x1240));
            painter_get(c, 0x002e_ea98, 0x002f_0104, wp::WorldPainter_GetFog20, index)?;
            c.mov_s(21, 0);
            painter_get(c, 0x002e_eae0, 0x002f_0110, wp::WorldPainter_GetFog28, index)?;
            c.mov_s(20, 0);
            painter_get(c, 0x002e_eb28, 0x002f_011c, wp::WorldPainter_GetFog30, index)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.swc1(21, frame.at(c, 4));
            c.swc1(1, frame.at(c, 0));
            c.swc1(20, frame.at(c, 8));
            c.swc1(0, frame.at(c, 0xc));
            c.g[A1][0] = sx(0x0050_0000u32.wrapping_add(0x4720));
            c.ldl(2, frame.at(c, 7));
            c.ldr(2, frame.at(c, 0));
            c.ldl(3, frame.at(c, 0xf));
            c.ldr(3, frame.at(c, 8));
            let dst = c.g[A1][0] as u32;
            c.sdl(2, dst.wrapping_add(7));
            c.sdr(2, dst);
            c.sdl(3, dst.wrapping_add(0xf));
            c.sdr(3, dst.wrapping_add(8));
        }

        // Screen tint / fill / weather (skipped once latched).
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp(c, painter::HAVE_TINT));
        let have_tint = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp(c, 0x12dc));
        if !have_tint {
            c.cycles += 29;
            c.g[S0][0] = 0;
            painter_get(c, 0x002e_eb70, 0x002f_0170, wp::WorldPainter_GetScreenTintRed, index)?;
            c.swc1(0, gp(c, 0x14f4));
            painter_get(c, 0x002e_ebd0, 0x002f_017c, wp::WorldPainter_GetScreenTintGreen, index)?;
            c.swc1(0, gp(c, 0x14f8));
            painter_get(c, 0x002e_ec30, 0x002f_0188, wp::WorldPainter_GetScreenTintBlue, index)?;
            c.swc1(0, gp(c, 0x14fc));
            painter_get(c, 0x002e_ec90, 0x002f_0194, wp::WorldPainter_GetScreenFillRed, index)?;
            c.swc1(0, gp(c, 0x1500));
            painter_get(c, 0x002e_ecf0, 0x002f_01a0, wp::WorldPainter_GetScreenFillGreen, index)?;
            c.swc1(0, gp(c, 0x1504));
            painter_get(c, 0x002e_ed50, 0x002f_01ac, wp::WorldPainter_GetScreenFillBlue, index)?;
            c.swc1(0, gp(c, 0x1508));
            painter_get(c, 0x002e_e738, 0x002f_01b8, wp::WorldPainter_GetWeather58, index)?;
            c.swc1(0, gp(c, 0x1524));
            c.g[A0][0] = 1;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c.g[V0][0], 0x84));
            c.g[V1][0] = c.lw(at(c.g[V1][0], 0x28));
            c.g[V0][0] = c.lw(at(c.g[V1][0], 0));
            let mut store_again = true;
            if c.g[V0][0] == c.g[A0][0] {
                c.g[S0][0] = 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V1][0];
                Callee { addr: 0x002f_7be0, f: wp::PS2VisualEffectsMan_IsHandleNull }
                    .call(c, 0x002f_01e0)?;
                if c.g[V0][0] == 0 {
                    c.sw(gp(c, 0x151c), c.g[S0][0]);
                    store_again = false;
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = 1;
                }
            }
            if store_again {
                c.cycles += 1;
                c.sw(gp(c, 0x151c), c.g[S0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(gp(c, 0x12dc));
        }

        // Light glows (skipped when the weather flag is set).
        c.cycles += 2;
        let have_glow = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        if !have_glow {
            c.cycles += 22;
            painter_get(c, 0x002e_edb0, 0x002f_0204, wp::WorldPainter_GetLightGlow08, index)?;
            c.swc1(0, gp(c, 0x12e4));
            painter_get(c, 0x002e_edf8, 0x002f_0210, wp::WorldPainter_GetLightGlow10, index)?;
            c.swc1(0, gp(c, 0x12e8));
            painter_get(c, 0x002e_ee40, 0x002f_021c, wp::WorldPainter_GetLightGlow18, index)?;
            c.swc1(0, gp(c, 0x12ec));
            painter_get(c, 0x002e_ee88, 0x002f_0228, wp::WorldPainter_GetLightGlow20, index)?;
            c.swc1(0, gp(c, 0x12f0));
            painter_get(c, 0x002e_eed0, 0x002f_0234, wp::WorldPainter_GetLightGlow28, index)?;
            c.swc1(0, gp(c, 0x12f4));
            painter_get(c, 0x002e_ef18, 0x002f_0240, wp::WorldPainter_GetLightGlow30, index)?;
            c.swc1(0, gp(c, 0x1300));
            painter_get(c, 0x002e_ef60, 0x002f_024c, wp::WorldPainter_GetLightGlow38, index)?;
            c.swc1(0, gp(c, 0x1304));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        }

        // The per-index record: each step stores the previous accessor's
        // value and starts the next one.
        let cache = c.g[S2][0];
        let slot = c.g[S0][0];
        c.cycles += 67;
        c.g[V1][0] = c.lw(gp(c, 0xa6c));
        c.g[V0][0] = sx((cache as u32).wrapping_add(slot as u32));
        c.g[A0][0] = index;
        let dst = at(c.g[V0][0], 0x8c);
        c.sw(dst, c.g[V1][0]);
        Callee { addr: 0x002e_f038, f: wp::WorldPainter_GetSurface08 }.call(c, 0x002f_0268)?;
        painter_step(c, index, cache, slot, 0xac, 0x002e_e3b8, 0x002f_0278, wp::WorldPainter_GetCamera08)?;
        painter_step(c, index, cache, slot, 0xcc, 0x002e_e9c0, 0x002f_0288, wp::WorldPainter_GetFog08)?;
        painter_step(c, index, cache, slot, 0xec, 0x002e_ea98, 0x002f_0298, wp::WorldPainter_GetFog20)?;
        painter_step(c, index, cache, slot, 0x10c, 0x002e_eae0, 0x002f_02a8, wp::WorldPainter_GetFog28)?;
        painter_step(c, index, cache, slot, 0x12c, 0x002e_eb28, 0x002f_02b8, wp::WorldPainter_GetFog30)?;
        painter_step(c, index, cache, slot, 0x14c, 0x002e_ea08, 0x002f_02c8, wp::WorldPainter_GetFog10)?;
        painter_step(c, index, cache, slot, 0x16c, 0x002e_ea50, 0x002f_02d8, wp::WorldPainter_GetFog18)?;
        painter_step(c, index, cache, slot, 0x18c, 0x002e_eb70, 0x002f_02e8, wp::WorldPainter_GetScreenTintRed)?;
        painter_step(c, index, cache, slot, 0x1ac, 0x002e_ebd0, 0x002f_02f8, wp::WorldPainter_GetScreenTintGreen)?;
        painter_step(c, index, cache, slot, 0x1cc, 0x002e_ec30, 0x002f_0308, wp::WorldPainter_GetScreenTintBlue)?;
        painter_step(c, index, cache, slot, 0x1ec, 0x002e_ec90, 0x002f_0318, wp::WorldPainter_GetScreenFillRed)?;
        painter_step(c, index, cache, slot, 0x20c, 0x002e_ecf0, 0x002f_0328, wp::WorldPainter_GetScreenFillGreen)?;
        painter_step(c, index, cache, slot, 0x22c, 0x002e_ed50, 0x002f_0338, wp::WorldPainter_GetScreenFillBlue)?;

        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        frame.restore(c, S1, 0x30);
        let dst = at(c.g[S0][0], 0x24c);
        c.swc1(0, dst);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameModule_CheckPauseRequest (0x231840).

/// `gp` globals of the pause check.
mod pause {
    /// `D_004A2EEC`: the active "loading" screen (its `+8` freezes input).
    pub const LOADING: i32 = -0x204;
    /// `D_004A2E9C`: disables the select-confirm input evaluation.
    pub const NO_CONFIRM: i32 = -0x254;
    /// `D_00534B30`: a pending restart flag.
    pub const RESTART: u32 = 0x0053_4b30;
}

/// The body after the early out: pick the player the pause menu should
/// select and store the ready mask (+0x224) and selection (+0x228).
fn pause_request_body(c: &mut Cpu) -> Result<(), Unwound> {
    // `cBE_getBE()` / `cBE_getInterface(be, 1)`.
    c.cycles += 10;
    c.g[S4][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x0023_1884)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0014_dd58, 0x0023_1890)?;
    c.g[V1][0] = c.lw(at(c.g[S2][0], 0x28));
    c.g[S6][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x608));
    let single = c.g[V0][0] != 0;
    c.g[S3][0] = 0xffff_ffff_ffff_ffff;
    if single {
        // One active player (`game+0x28`): mask him if not ready, else let
        // the confirm input select him.
        c.cycles += 8;
        c.g[S0][0] = c.lw(at(c.g[V1][0], 0x28));
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
        c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
        c.g[A1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
            c.g[A1][0] = (0 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0x3c;
                call_ptr(c, 0x0032_0c48, 0x0023_18f0)?;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = c.g[S0][0];
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.g[S4][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
        }
    } else {
        // Several riders: mask every active one that is not ready.
        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
        c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
        let no_riders = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_riders {
            c.cycles += 2;
            c.g[S5][0] = 1;
            c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = c.lw(at(c.g[V0][0], 0x40));
                c.g[V1][0] = c.lw(at(c.g[A0][0], 0x18));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x87c));
                let inactive = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S6][0];
                if !inactive {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_7290, 0x0023_1934)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
                    let not_ready = c.g[V0][0] == 0;
                    c.g[A1][0] = 0;
                    if !not_ready {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                        c.g[A1][0] = c.g[S5][0];
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = 0;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S5][0] as u32) << ((c.g[S0][0] as u32) & 31));
                    if c.g[A1][0] != 0 {
                        // Ready: select it unless one is selected already.
                        c.cycles += 2;
                        if (c.g[S3][0] as i64) >= 0 {
                            c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x3c;
                                call_ptr(c, 0x0032_0c48, 0x0023_198c)?;
                                if c.g[V0][0] != 0 {
                                    c.g[S3][0] = c.g[S0][0];
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            }
                        }
                    } else {
                        c.cycles += 2;
                        let bit = c.g[V0][0];
                        c.g[S4][0] |= bit;
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                }
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_1910);
                    }
                    continue;
                }
                break;
            }
        }
    }

    // The ready mask and the selection.
    c.cycles += 2;
    let no_mask = c.g[S4][0] == 0;
    let s2 = c.g[S2][0];
    let mask = c.g[S4][0];
    c.sw(at(s2, 0x224), mask);
    if !no_mask {
        c.cycles += 2;
        if (c.g[S3][0] as i64) >= 0 {
            c.sw(at(s2, 0x228), c.g[S3][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.lw(at(s2, 0xc));
            c.g[S0][0] = 0;
            c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = 0;
            if no_riders {
                c.cycles += 1;
                c.sw(at(s2, 0x228), c.g[S3][0]);
            } else {
                // Search for the first ready rider to select.
                c.cycles += 3;
                c.g[S1][0] = 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw(at(c.g[V0][0], 0x40));
                    c.g[A0][0] = c.lw(at(c.g[V1][0], 0x18));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0x87c));
                    let inactive = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S6][0];
                    if !inactive {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0014_7290, 0x0023_19f8)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw(gp(c, -0x848));
                        c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xb0));
                        c.g[V0][0] = c.lw(at(c.g[V1][0], 0));
                        let not_ready = c.g[V0][0] == 0;
                        c.g[A1][0] = 0;
                        if !not_ready {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = 0;
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != 0 {
                            c.g[S3][0] = c.g[A2][0];
                            c.cycles += 1;
                            c.sw(at(s2, 0x228), c.g[S3][0]);
                            break;
                        }
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(at(s2, 0xc));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_19d8);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.sw(at(s2, 0x228), c.g[S3][0]);
                    break;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }

    // No one is ready: only a pending restart lets the pause be left.
    c.cycles += 2;
    if (c.g[S3][0] as i64) < 0 {
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 9;
    call_ptr(c, 0x0014_dc80, 0x0023_1a5c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 7;
    call_ptr(c, 0x0014_dd58, 0x0023_1a68)?;
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
    if c.g[V0][0] != 0 {
        c.sw(at(s2, 0x228), c.g[S3][0]);
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    Callee { addr: 0x0023_1ab8, f: ssx3_ee::s06::GameModule_CanRestartFromPause }
        .call(c, 0x0023_1a80)?;
    let restart = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !restart {
        return Ok(());
    }
    c.sw(at(s2, 0x228), c.g[S3][0]);
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// `int GameModule_CheckPauseRequest(GameModule *g)` (0x231840): decides
/// which player the pause menu selects. Returns 1 when the pause can be
/// confirmed (a ready player is selected, or a restart is permitted), 0
/// otherwise; the ready mask goes to `g+0x224` and the selection to `g+0x228`.
///
/// Frame 0x80: s2 at +0x50, s0 at +0x70, s1 at +0x60, s3 at +0x40, s4 at
/// +0x30, s5 at +0x20, s6 at +0x10, ra at 0.
pub fn game_module_check_pause_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(gp(c, pause::LOADING));
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        let loading = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        let mut stopped = false;
        if loading {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 8));
            stopped = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = 0;
        }
        if !stopped {
            pause_request_body(c)?;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_SmoothPitch (0x1662a0).
//
// (The VU0 pipeline uses `Src`/`X`/`Y`/`Z`/`W`.)

/// One dot product through the VU0 macro pipeline: `vf4 = dot(*a, *b)`, the
/// result read back into `gpr` (`lqc2 vf3/vf5`, the four-op multiply-add
/// chain, `qmfc2`). Leaves vf3-vf6 and acc as the original does.
fn vu0_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// `void ChaseCameraAlgorithm_SmoothPitch(self, float x, float y, float z,
/// float w)` (0x1662a0): blend the camera's pitch (`+0x54`) and its smoothed
/// pitch (`+0x1d8`) towards the target angles, limited by the angle between
/// the camera-to-target direction and the camera's forward vector, then
/// re-aim the camera (`RotateByPitch`), update the look-at offset and let
/// `CheckDistanceReset` correct the distance.
///
/// Frame 0xb0: s0 at +0x80, ra at +0x70, f20 at +0x90, f21 at +0x98, f22 at
/// +0xa0, f23 at +0xa8. Scratch vectors at +0x0..+0x60.
pub fn chase_camera_algorithm_smooth_pitch(c: &mut Cpu) {
    guest(c, |c| {
        use ssx3_ee::s02 as cam;
        c.cycles += 14;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(23, frame.at(c, 0xa8));
        c.swc1(22, frame.at(c, 0xa0));
        c.swc1(21, frame.at(c, 0x98));
        c.swc1(20, frame.at(c, 0x90));
        c.lwc1(1, at(c.g[S0][0], 0x1d8));
        c.lwc1(0, at(c.g[S0][0], 0x54));
        c.c_lt_s(1, 0);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // Up or down: pick the blend factor and target angle from the
        // argument pair (w,y) or (z,x).
        c.cycles += 2;
        let up = c.g[V0][0] != 0;
        c.mov_s(20, 15);
        if up {
            c.cycles += 2;
            c.mov_s(22, 13);
        } else {
            c.cycles += 2;
            c.mov_s(20, 14);
            c.mov_s(22, 12);
        }

        // Blend the current pitch and re-aim the camera to get the actual
        // pitch offset.
        c.cycles += 42;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(23, at(c.g[S0][0], 0x54));
        c.g[A1][0] = sx(frame.at(c, 0x10));
        c.sub_s(1, 21, 20);
        c.lwc1(0, at(c.g[S0][0], 0x1d8));
        c.mul_s(0, 20, 0);
        c.mul_s(1, 1, 23);
        c.add_s(0, 0, 1);
        c.swc1(0, at(c.g[S0][0], 0x54));
        c.lqc2(3, at(c.g[S0][0], 0x20));
        c.lqc2(4, at(c.g[S0][0], 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x10));
        c.lq(2, frame.at(c, 0x10));
        c.g[RA][0] = sx(0x0016_633c);
        c.sq(2, frame.at(c, 0));
        Callee { addr: 0x0016_68b8, f: cam::ChaseCameraAlgorithm_RotateByPitch }
            .call(c, 0x0016_633c)?;
        vu0_dot(c, frame.at(c, 0), frame.at(c, 0), V0);
        c.mtc1(2, 0);
        vu0_dot(c, frame.at(c, 0x10), frame.at(c, 0x10), V1);
        c.mtc1(3, 1);
        c.mul_s(0, 0, 1);
        c.lwc1(2, gp(c, -0x68c0));
        c.sqrt_s(3, 0);
        c.c_lt_s(3, 2);
        let beyond = !c.fcc();
        if beyond {
            // The aimed direction is long enough: the pitch is the angle
            // between it and the target offset.
            c.cycles += 16;
            vu0_dot(c, frame.at(c, 0), frame.at(c, 0x10), V0);
            c.mtc1(2, 0);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 1);
            c.div_s(0, 0, 3);
            c.c_le_s(1, 0);
            let clamp = !c.fcc();
            c.mov_s(12, 1);
            if !clamp {
                c.cycles += 1;
                c.min_s(12, 0, 21);
            }
            c.cycles += 4;
            call_ptr(c, 0x0031_c128, 0x0016_63f4)?;
            c.lwc1(1, gp(c, -0x68bc));
            c.sub_s(1, 1, 0);
        } else {
            c.cycles += 3;
            c.mtc1(0, 1);
        }

        // Turn rate: `ang * k` (or a linear step when the angle is tiny).
        c.cycles += 6;
        c.lwc1(0, gp(c, -0x68b8));
        c.mul_s(0, 22, 0);
        c.c_lt_s(0, 1);
        let small = !c.fcc();
        c.mov_s(14, 20);
        if !small {
            c.cycles += 2;
            c.div_s(0, 1, 0);
            c.div_s(14, 20, 0);
        }

        // Blend the smoothed pitch and re-aim, then re-aim the look-at
        // offset and let the distance reset check run.
        c.cycles += 68;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A1][0] = sx(frame.at(c, 0x20));
        c.lwc1(0, at(c.g[S0][0], 0x1d8));
        c.g[A0][0] = c.g[S0][0];
        c.sub_s(1, 1, 14);
        c.mul_s(0, 14, 0);
        c.mul_s(1, 1, 23);
        c.add_s(0, 0, 1);
        c.swc1(0, at(c.g[S0][0], 0x1d8));
        c.g[RA][0] = sx(0x0016_644c);
        c.swc1(0, at(c.g[S0][0], 0x54));
        Callee { addr: 0x0016_68b8, f: cam::ChaseCameraAlgorithm_RotateByPitch }
            .call(c, 0x0016_644c)?;
        c.g[A2][0] = c.lw(at(c.g[S0][0], 0x30));
        c.g[A0][0] = sx(frame.at(c, 0x40));
        c.g[V0][0] = c.lw(at(c.g[A2][0], 0));
        c.g[A1][0] = c.lh(at(c.g[V0][0], 8));
        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xc));
        c.g[RA][0] = sx(0x0016_6468);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        let target = c.g[V1][0] as u32;
        call_ptr(c, target, 0x0016_6468)?;
        c.lqc2(3, frame.at(c, 0x40));
        c.lqc2(4, at(c.g[S0][0], 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x30));
        vu0_dot(c, frame.at(c, 0x20), frame.at(c, 0x20), V0);
        c.mtc1(2, 1);
        vu0_dot(c, frame.at(c, 0x30), frame.at(c, 0x20), V1);
        c.mtc1(3, 0);
        c.div_s(0, 0, 1);
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x50));
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x40));
        c.lqc2(3, at(c.g[S0][0], 0x40));
        c.lqc2(4, frame.at(c, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x60));
        c.lq(2, frame.at(c, 0x60));
        c.sq(2, at(c.g[S0][0], 0x20));
        c.g[RA][0] = sx(0x0016_650c);
        c.sq(2, frame.at(c, 0x50));
        Callee { addr: 0x0016_8150, f: cam::ChaseCameraAlgorithm_CheckDistanceReset }
            .call(c, 0x0016_650c)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 23, 0xa8);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    

    #[test]
    fn vfx_screen_tint_update_matches() {
        check(
            ssx3_ee::s12::f_00390c60__generated,
            vfx_screen_tint_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3912_0c60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A quiet PRNG state so `Rand_Next` returns immediately.
                for i in 0..0x24u32 {
                    c.bus.write32(0x004f_f018 + i * 4, 0);
                }
                c.bus.write32(g + 0x151c, if seed % 6 == 0 { 0 } else { 1 });
                c.bus.write32(g + 0x15c4, match seed % 3 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => (seed % 20) as u32,
                });
                // Timer: idle (-1) on some seeds, otherwise inside or past the
                // segment lengths.
                let (d0, d1, d2, d3) = (4u32, 3, 2, 5);
                let timer = match seed % 8 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 2,
                    3 => 5,
                    4 => 8,
                    5 => 11,
                    6 => 13,
                    _ => (r.next_u32() % 18) as u32,
                };
                c.bus.write32(g + 0x15a8, timer);
                c.bus.write32(g + 0x1520, match seed % 5 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 20,
                    _ => r.next_u32() & 0x3f,
                });
                c.bus.write32(g + 0x1524, r.edge_f32());
                c.bus.write32(g + 0x1528, d0);
                c.bus.write32(g + 0x152c, d1);
                c.bus.write32(g + 0x1530, d2);
                c.bus.write32(g + 0x1534, d3);
                for off in [0x1538u32, 0x153c, 0x1540, 0x1544, 0x1548, 0x154c,
                            0x1550, 0x1554, 0x1558, 0x155c, 0x1560, 0x1564,
                            0x14f4, 0x14f8, 0x14fc] {
                    c.bus.write32(g + off, r.edge_f32());
                }
                c.bus.write32(g + 0x15ac, r.next_u32());
                c.bus.write32(g + 0x15b0, r.next_u32());
                c.bus.write32(g + 0x15b4, r.next_u32());
                c.bus.write32(g + 0x15b8, r.next_u32());
                c.bus.write32(g + 0x15bc, r.next_u32());
            },
        );
    }

    #[test]
    fn check_pause_request_matches() {
        check(
            ssx3_ee::s06::f_00231840__generated,
            game_module_check_pause_request,
            300,
            &|c, seed| {
                let mut r = Rng(0x2318_4067 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A loading screen, sometimes already frozen.
                let loading = DATA + 0x500;
                c.bus.write32(g - 0x204, match seed % 4 {
                    0 | 1 => 0,
                    _ => loading,
                });
                c.bus.write32(loading + 8, match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xffff_ffff,
                });
                // The confirm input evaluation runs the input VM; most seeds
                // skip it, some reach it with a stream that stops at once.
                c.bus.write32(g - 0x254, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(g - 0x84c, 0x002c_c098);
                let map = DATA + 0x6300;
                c.bus.write32(map + 4, DATA + 0x6400);
                c.bus.write32(map + 0x14, DATA + 0x6500);
                c.bus.write32(DATA + 0x6400 + 0xf0, 0);
                c.bus.write32(DATA + 0x6500, 0x20); // the VM's stop opcode
                // The cBE singleton and the interface slot it indexes.
                c.bus.write32(g - 0x1f04, 0x0080_9000);
                c.bus.write32(g - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x1004, DATA + 0x1200);
                c.bus.write16(DATA + 0x1208, 0);
                c.bus.write32(DATA + 0x120c, 0x002c_c098);
                // The game module and its player/rider managers.
                let game = DATA + 0x200;
                c.g[A0][0] = game as u64;
                c.bus.write32(game + 0x28, DATA + 0x3000);
                c.bus.write32(game + 0xc, DATA + 0x4000);
                c.bus.write32(game + 0x210, 0); // CanRestartFromPause: 0
                c.bus.write32(game + 0x224, r.next_u32());
                c.bus.write32(game + 0x228, r.next_u32());
                // One active player, or a rider list.
                let one = seed % 3 != 0;
                c.bus.write32(DATA + 0x3000 + 0x608, one as u32);
                c.bus.write32(DATA + 0x3000 + 0x28, (seed % 4) as u32);
                c.bus.write32(DATA + 0x4000 + 0x7c, (seed % 4) as u32);
                for i in 0..4u32 {
                    let rider = DATA + 0x4100 + i * 0x20;
                    c.bus.write32(DATA + 0x4000 + 0x40 + i * 4, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x4200 + i * 0x20);
                    c.bus.write32(DATA + 0x4200 + i * 0x20 + 0x87c, ((seed + i) % 2) as u32);
                    // BENewPlayerInt slot table: slot i, player i.
                    c.bus.write32(0x0053_05b0 + i * 4, i);
                    c.bus.write32(0x0053_4fe0 + i * 0x1c + 0xc, i);
                }
                c.bus.write32(g - 0x848, DATA + 0x6000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x6000 + i * 4 + 0xb0, DATA + 0x6100 + i * 0x20);
                    let ready = (seed + i * 7) % 2 == 0;
                    c.bus.write32(DATA + 0x6100 + i * 0x20, if ready { DATA + 0x6200 } else { 0 });
                    c.bus.write32(DATA + 0x6100 + i * 0x20 + 4, map);
                    c.bus.write32(DATA + 0x6200, 1);
                }
                c.bus.write32(pause::RESTART, match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32(),
                });
            },
        );
    }

    #[test]
    fn apply_world_painter_matches() {
        check(
            ssx3_ee::s09::f_002f00a0__generated,
            ps2_visual_effects_man_apply_world_painter,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f00_a0c5 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                let index = (seed % 4) as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index;
                // world -> app (+0x84) -> game module (+0x28) -> handle.
                c.bus.write32(g - 0x848, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200, match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() & 1,
                });
                c.bus.write32(g + 0x2a90, DATA + 0x4000);
                // `gp + 0xa70 == 0`: the tint accessors take the 1.0f path.
                c.bus.write32(g + 0xa70, 0);
                // The painter descriptors all lead to one object whose vtable
                // methods are MenuSys_IsActive (it returns 0 for a zero word).
                for off in (0..0x40u32).step_by(4) {
                    c.bus.write32(0x004f_a370 + off, DATA + 0x1000);
                }
                c.bus.write32(0x004f_a370 + 0x24, r.edge_f32());
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, DATA + 0x1200);
                c.bus.write32(DATA + 0x1100, 0);
                for slot in (0x30..0x1b8u32).step_by(8) {
                    c.bus.write16(DATA + 0x1200 + slot, 0);
                    c.bus.write32(DATA + 0x1200 + slot + 4, 0x002c_c098);
                }
                c.bus.write32(g + 0x1238, (seed % 3 == 0) as u32);
                c.bus.write32(g + 0x14ec, (seed % 5 == 0) as u32);
                c.bus.write32(g + 0x12dc, (seed % 4 == 1) as u32);
                c.bus.write32(g + 0xa6c, r.next_u32());
                for off in [0x1244u32, 0x1248, 0x123c, 0x1240, 0x151c, 0x1524,
                            0x12e4, 0x12e8, 0x12ec, 0x12f0, 0x12f4, 0x1300,
                            0x1304, 0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508] {
                    c.bus.write32(g + off, r.next_u32());
                }
                for off in (0..0x260u32).step_by(4) {
                    c.bus.write32(DATA + 0x4000 + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn smooth_pitch_matches() {
        check(
            ssx3_ee::s02::f_001662a0__generated,
            chase_camera_algorithm_smooth_pitch,
            200,
            &|c, seed| {
                let mut r = Rng(0x1662_a0f3 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let s = DATA;
                c.g[A0][0] = s as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[15] = r.edge_f32();
                c.bus.write32(s + 0x54, r.edge_f32());
                c.bus.write32(s + 0x1d8, r.edge_f32());
                put_vec(c, s + 0x20, [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)]);
                put_vec(c, s + 0x40, [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)]);
                // `self+0x30` is the object whose method updates the look-at
                // target; its vtable slot 8/0xc is MenuSys_IsActive here.
                c.bus.write32(s + 0x30, s + 0x100);
                c.bus.write32(s + 0x100, s + 0x200);
                c.bus.write16(s + 0x208, 0);
                c.bus.write32(s + 0x20c, 0x002c_c098);
                c.bus.write32(s + 0x104, s + 0x120);
                c.bus.write32(s + 0x120, 0);
                for off in [0x200u32, 0x204, 0x210, 0x214, 0x218, 0x21c, 0x220,
                            0x224, 0x228, 0x22c, 0x230, 0x234, 0x238, 0x23c,
                            0x2c0, 0x2c4, 0x2c8, 0x2cc, 0x2d0, 0x2d4, 0x2d8,
                            0x2e0, 0x2e4, 0x2e8, 0x2ec, 0x1b0, 0x1b4, 0x1b8,
                            0x1bc, 0x1c0] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x68c0), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x68bc), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x68b8), r.edge_f32());
            },
        );
    }
}
