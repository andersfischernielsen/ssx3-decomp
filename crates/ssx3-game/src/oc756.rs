//! Staging module oc756 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc756`.
//!
//! Not ported (resumable dispatcher bodies, left to the orchestrator):
//! `cFEStateCharSelect_OnWidgetAction` (0x1817e8), `PATH_PostEvent` (0x3d16f0),
//! `FEUISound_v02` (0x1a2e58).

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
    (0x003a_0c50, ui_text_set_string, "UIText_SetString"),
    (
        0x0029_8138,
        ssx_audio_system_play_rider_bump_sound,
        "SSXAudioSystem_PlayRiderBumpSound",
    ),
    (
        0x002e_6320,
        c_speed_boost_streamer_fx_ctor,
        "cSpeedBoostStreamerFX_ctor",
    ),
];

/// `void UIText_SetString(UIText* text, const char* s)` (0x3a0c50): clears the
/// text's cached-string word at +0xb0, then `UIText_SetAsciiString(text, s)`.
pub fn ui_text_set_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(UI_TEXT_SET_STRING_CACHE)).sw(c, 0);
        Callee {
            addr: 0x003a_0c70,
            f: ee::s13::f_003a0c70,
        }
        .call(c, 0x003a_0c64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The text's cached-string word cleared by `UIText_SetString`.
pub(crate) const UI_TEXT_SET_STRING_CACHE: u32 = 0xb0;

/// `void SSXAudioSystem_PlayRiderBumpSound(SSXAudioSystem* snd, Rider* a,
/// Rider* b)` (0x298138, `cSSXAudio_Punch`): when rider `a` (or else `b`) is
/// active (+0x874 set and +0x87c > 0) and sound 5 is not already tracked, adds
/// a voice record to the player's voice table (the 24-byte descriptor at
/// player+0x1dc is copied into slot n, the slot's parameter arrays are
/// reset, 100/50/127/1 defaults and 0x5a/0x32 are stored), asks the channel
/// gain pointer, the rider listener index and the pushed sound, and tracks
/// sound 5 with the player's clock + 0x32.
///
/// The voice-record part keeps the generated op order: the register dance
/// (v0/v1/a0..a3/t0..t2 left over) is the authority.
pub fn ssx_audio_system_play_rider_bump_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = 0;
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        // v1 = rider `a` is active (+0x874 set, +0x87c > 0 as unsigned).
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(RIDER_BUMP_ACTIVE));
        let a0_is_b = c.g[A2][0];
        c.g[A0][0] = a0_is_b;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(RIDER_BUMP_COUNT));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        // Play when `a` is active, else when `b` is.
        let mut play = false;
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S3][0];
            play = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RIDER_BUMP_ACTIVE));
            c.g[A2][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RIDER_BUMP_COUNT));
                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[A2][0] != 0 {
                play = true;
            }
        }
        if play {
            c.cycles += 5;
            c.g[A1][0] = 5;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0029_5028, 0x0029_81a4)?; // IsTrackedSoundPlaying(snd, 5, 0)
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                return bump_epilogue(c, frame);
            }
            c.cycles += 175;
            c.g[S0][0] = c.lw(c.g[S3][0] as u32);
            c.g[T1][0] = 0x18;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[S2][0] = 0x5a;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 9);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[V1][0] as u32);
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 9);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x00416210, 0x00298284)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A3][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[T1][0] = 0x7f;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T2][0] = 0x1;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[A3][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[T1][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[T2][0]);
            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S2][0]);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            call_ptr(c, 0x00287968, 0x002983dc)?;
            c.g[A3][0] = c.lw(c.g[S0][0] as u32);
            c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x110));
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[T0][0]);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            call_ptr(c, 0x00288a20, 0x00298420)?;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x23c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x002906b8, 0x00298440)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x5;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0u64;
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x260));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x32));
            call_ptr(c, 0x00295628, 0x00298468)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        bump_epilogue(c, frame)
    })
}

/// Rider fields read by `SSXAudioSystem_PlayRiderBumpSound`.
pub(crate) const RIDER_BUMP_ACTIVE: u32 = 0x874;
pub(crate) const RIDER_BUMP_COUNT: u32 = 0x87c;

/// The shared epilogue of `SSXAudioSystem_PlayRiderBumpSound`.
fn bump_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cSpeedBoostStreamerFX* cSpeedBoostStreamerFX_ctor(cSpeedBoostStreamerFX* self,
/// int inCharge)` (0x2e6320): with `inCharge`, constructs the embedded rider
/// at self+0x3b0 (stored at self+0); points the rider's script-object, rider
/// and state vtables at the streamer's tables (without `inCharge` it copies
/// the two tables onto the stack and adds the virtual-base offset to their
/// second entries); runs the 30-element trail constructor loop; allocates the
/// 20-point stream buffer (`operator_new_named(0x280, "cSpeedBoostStreamerFX",
/// 0, 0)`, stored at self+0x3ac) and calls `cSpeedBoostStreamerFX_Reset`.
/// Returns `self`.
pub fn c_speed_boost_streamer_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        if c.g[S1][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(SBS_RIDER_OFFSET));
            c.g[A0][0] = c.g[V0][0];
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x0011_b698, 0x002e_6350)?; // Rider_ctor(self + 0x3b0)
        }
        c.cycles += 17;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ab0)); // 0x488550
        c.g[A3][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
        c.g[V0][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470)); // 0x459b90
        c.g[V1][0] = 0x490000;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        c.g[T3][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7a68)); // 0x488598
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(SBS_STREAM_BUF));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
        if c.g[S1][0] == 0 {
            // Not in charge: private copies of the two vtables on the stack
            // with the virtual-base delta added to entry 1.
            c.cycles += 3;
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let again = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_63a0);
                }
            }
            c.cycles += 14;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ab0));
            c.g[A3][0] = c.ld(c.g[A2][0] as u32);
            c.sd(c.g[V1][0] as u32, c.g[A3][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
            c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7a68));
            c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SBS_RIDER_OFFSET));
            c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            loop {
                c.cycles += 11;
                c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let again = c.g[A3][0] != c.g[T2][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_6404);
                }
            }
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7a68));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
            c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
        }
        // The 30-element trail constructor loop (no effect but its cycles).
        c.cycles += 2;
        c.g[V0][0] = 0x1d;
        c.g[V1][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let again = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x280;
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_6460);
            }
        }
        c.cycles += 17;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7b08)); // "cSpeedBoostStreamerFX"
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002e_648c)?; // operator_new_named(0x280, name, 0, 0)
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 0x13;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002e_6640, 0x002e_64a4)?; // cSpeedBoostStreamerFX_Reset(self)
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x120);
        frame.restore(c, S0, 0x130);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// Offset of the embedded rider in `cSpeedBoostStreamerFX`.
pub(crate) const SBS_RIDER_OFFSET: u32 = 0x3b0;
/// Offset of the stream buffer pointer in `cSpeedBoostStreamerFX`.
pub(crate) const SBS_STREAM_BUF: u32 = 0x3ac;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check_lohi, elf_image, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes (in each machine's own RAM).
    const OC756_CTRL: u32 = DATA + 0x2f_0000;
    const OC756_MAGIC: u32 = 0x0756_0756;
    /// Base of the fixtures' data (clear of the fakes' call log at DATA+0x100000).
    const OC756_BASE: u32 = DATA + 0x21_0000;

    /// The fake callee: logs the call like `fakes::fake_call`, then, unless it
    /// unwound, returns `mail - 1` when the fixture put `mail` (non-zero) in
    /// the mailbox of this fake (so a fake can return 0).
    fn oc756_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC756_CTRL + 0x10 + 4 * idx);
        fakes::fake(c, addr, ra);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc756_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC756_CTRL) != OC756_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc756_fake_call(c, $addr, $idx);
                }
            )*
            fn oc756_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc756_fakes!(
        (
            oc756_fake_set_ascii,
            OC756_PREV_SET_ASCII,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70,
            0
        ),
        (
            oc756_fake_is_tracked,
            OC756_PREV_IS_TRACKED,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028,
            0
        ),
        (
            oc756_fake_memset,
            OC756_PREV_MEMSET,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210,
            1
        ),
        (
            oc756_fake_gain_ptr,
            OC756_PREV_GAIN_PTR,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968,
            2
        ),
        (
            oc756_fake_listener,
            OC756_PREV_LISTENER,
            ee::s07::OVERRIDE_00288A20,
            ee::s07::f_00288a20__generated,
            0x0028_8a20,
            3
        ),
        (
            oc756_fake_pushed,
            OC756_PREV_PUSHED,
            ee::s07::OVERRIDE_002906B8,
            ee::s07::f_002906b8__generated,
            0x0029_06b8,
            4
        ),
        (
            oc756_fake_track,
            OC756_PREV_TRACK,
            ee::s08::OVERRIDE_00295628,
            ee::s08::f_00295628__generated,
            0x0029_5628,
            5
        ),
        (
            oc756_fake_rider_ctor,
            OC756_PREV_RIDER_CTOR,
            ee::s00::OVERRIDE_0011B698,
            ee::s00::f_0011b698__generated,
            0x0011_b698,
            6
        ),
        (
            oc756_fake_new_named,
            OC756_PREV_NEW_NAMED,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30,
            7
        ),
        (
            oc756_fake_reset,
            OC756_PREV_RESET,
            ee::s09::OVERRIDE_002E6640,
            ee::s09::f_002e6640__generated,
            0x002e_6640,
            8
        ),
    );

    /// `UIText_SetString`: clears the +0xb0 word of the text and calls the
    /// (faked, sometimes unwinding) `UIText_SetAsciiString`.
    #[test]
    fn ui_text_set_string_matches() {
        oc756_install_fakes();
        covered(
            ee::s13::f_003a0c50__generated,
            ui_text_set_string,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a0c50, 1);
                c.pc = 0x003a_0c50;
                c.bus.write32(OC756_CTRL, OC756_MAGIC);
                let text = OC756_BASE + 0x1000;
                c.bus.write32(text + UI_TEXT_SET_STRING_CACHE, r.next_u32());
                c.g[A0][0] = text as u64;
            },
        );
    }

    /// `SSXAudioSystem_PlayRiderBumpSound`: rider `a` or `b` active or not
    /// (+0x874 and +0x87c as zero / positive / negative), sound 5 already
    /// tracked or not (the faked `IsTrackedSoundPlaying` returns 1 or 0), the
    /// voice record built in the player (descriptor copied to the slot at
    /// `(n + base) * 24`, the parameter arrays reset) and the five faked
    /// callees (which may unwind).
    #[test]
    fn ssx_audio_system_play_rider_bump_sound_matches() {
        oc756_install_fakes();
        check_lohi(
            ee::s08::f_00298138__generated,
            ssx_audio_system_play_rider_bump_sound,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x298138, 6);
                c.pc = 0x0029_8138;
                c.bus.write32(OC756_CTRL, OC756_MAGIC);
                let base = OC756_BASE;
                let (snd, player, rider_a, rider_b) =
                    (base, base + 0x1000, base + 0x4000, base + 0x6000);
                let holder = base + 0x400;
                // A rider: +0x874 / +0x87c by kind.
                let put_rider = |c: &mut Cpu, rider: u32, kind: u32| {
                    let (active, count): (u32, u32) = match kind {
                        0 => (0, 5),
                        1 => (1, 0),
                        2 => (1, 7),
                        _ => (1, 0x8000_0010),
                    };
                    c.bus.write32(rider + RIDER_BUMP_ACTIVE, active);
                    c.bus.write32(rider + RIDER_BUMP_COUNT, count);
                };
                put_rider(c, rider_a, seed % 4);
                put_rider(c, rider_b, (seed / 4) % 4);
                // IsTrackedSoundPlaying: 0 (mail 1) or 1 (mail 2).
                c.bus.write32(OC756_CTRL + 0x10, 1 + (seed / 16) % 2);
                // The audio system.
                c.bus.write32(snd, player);
                c.bus.write32(snd + 4, base + 0x8000);
                c.bus.write32(snd + 8, base + 0x8100);
                c.bus.write32(snd + 0x118, holder);
                c.bus.write32(holder, player);
                // The player: descriptor, voice count n (+0x1f4), the slot
                // base (+0x1f8: the voice-table address is (n+1+base)*24),
                // the parameter-array pointers (+0x20c.. = Q+0x34..).
                for i in 0..6 {
                    c.bus.write32(player + 0x1dc + 4 * i, r.next_u32());
                }
                let n0 = seed % 3;
                let k: u32 = 436_907; // 24 * k = DATA + 0x200008
                c.bus.write32(player + 0x1f4, n0);
                c.bus.write32(player + 0x1f8, k - (n0 + 1));
                for (i, off) in [
                    0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
                ]
                .iter()
                .enumerate()
                {
                    c.bus
                        .write32(player + 0x1d8 + off, base + 0x9000 + 0x100 * i as u32);
                }
                // The clock the tracking time is taken from.
                c.bus.write32(player + 0x260, r.next_u32() & 0xffff);
            },
        );
    }

    /// `cSpeedBoostStreamerFX_ctor`: in charge (the rider constructed at
    /// +0x3b0) or not (the vtable copies on the stack), with the faked
    /// `Rider_ctor`, `operator_new_named` and `cSpeedBoostStreamerFX_Reset`
    /// (which may unwind); the vtables come from the ELF image.
    #[test]
    fn c_speed_boost_streamer_fx_ctor_matches() {
        oc756_install_fakes();
        covered(
            ee::s09::f_002e6320__generated,
            c_speed_boost_streamer_fx_ctor,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x2e6320, 3);
                elf_image(c);
                c.pc = 0x002e_6320;
                c.bus.write32(OC756_CTRL, OC756_MAGIC);
                let this = OC756_BASE + 0x1000;
                let rider = OC756_BASE + 0x4000;
                // Not in charge: the rider already exists (self+0 points at it,
                // with its virtual-base offset making the delta non-trivial).
                c.bus.write32(this, rider);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    1 + (seed % 5) as u64
                };
            },
        );
    }
}
