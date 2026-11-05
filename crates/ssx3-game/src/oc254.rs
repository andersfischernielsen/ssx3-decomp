//! Staging wave 207 (oc254): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{self, call_ptr, guest, sp, Callee, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_6c98, sndstrm_get_buffered_bytes, "SNDSTRM_GetBufferedBytes"),
    (0x002a_6d18, crowd_audio_update, "CrowdAudio_Update"),
    (0x003e_13e8, stream_file_release_chunk, "StreamFile_ReleaseChunk"),
    (0x0021_e7e0, hud_draw_stretched_panel, "HUD_DrawStretchedPanel"),
    (
        0x0029_3b50,
        ssx_audio_system_calc_surface_loop_remote_group0,
        "SSXAudioSystem_CalcSurfaceLoopRemoteGroup0",
    ),
    (0x002e_ef60, world_painter_get_light_glow38, "WorldPainter_GetLightGlow38"),
    (0x002e_e8e8, world_painter_get_weather88, "WorldPainter_GetWeather88"),
    (0x002e_b8f0, transition_effect_update_screen_rect, "TransitionEffect_UpdateScreenRect"),
    (0x0011_7908, rider_metrix_get_trick_bonus, "RiderMetrix_GetTrickBonus"),
    (0x0031_7810, rand_next_game, "Rand_NextGame"),
];

// ---------------------------------------------------------------------------
// SNDSTRM_GetBufferedBytes (0x3c6c98)

/// The sound streamer's per-slot stream table (`D_005157B8 + 0x304`).
const SNDSTRM_SLOT_TABLE: u32 = 0x0051_57b8 + 0x304;
/// Stream fields: bytes buffered in the two halves of the ring.
const SNDSTRM_BUFFERED_A: u32 = 0x48;
const SNDSTRM_BUFFERED_B: u32 = 0x4c;

/// `int SNDSTRM_GetBufferedBytes(int slot)` (0x3c6c98): how many bytes the
/// stream in `slot` has buffered (the sum of its two buffer counts).
pub fn sndstrm_get_buffered_bytes(c: &mut Cpu) {
    c.cycles += 9;
    let index = sx((c.g[A0][0] as u32) << 2);
    c.g[A0][0] = index;
    let stream = Addr(SNDSTRM_SLOT_TABLE.wrapping_add(index as u32)).lw(c);
    c.g[V1][0] = stream;
    let a = Addr((stream as u32).wrapping_add(SNDSTRM_BUFFERED_A)).lw(c);
    let b = Addr((stream as u32).wrapping_add(SNDSTRM_BUFFERED_B)).lw(c);
    c.g[A0][0] = a;
    c.g[V0][0] = sx((a as u32).wrapping_add(b as u32));
    ret(c);
}

// ---------------------------------------------------------------------------
// CrowdAudio_Update (0x2a6d18)

/// CrowdAudio fields: number of crowd instances, their array (0x190 bytes each).
const CROWD_AUDIO_COUNT: u32 = 0x14;
const CROWD_AUDIO_INSTANCES: u32 = 0x18;
const CROWD_AUDIO_INSTANCE_SIZE: u32 = 0x190;

const CROWD_AUDIO_UPDATE_INSTANCE: Callee =
    Callee { addr: 0x002a_4e88, f: ee::s08::f_002a4e88 };

/// `void CrowdAudio_Update(CrowdAudio *self)` (0x2a6d18): updates every crowd
/// sound instance (`CrowdAudio_UpdateInstance`), re-reading the count after
/// each one.
pub fn crowd_audio_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_COUNT)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            // s0: index, s2: byte offset of the instance.
            c.g[S2][0] = 0;
            loop {
                c.cycles += 8;
                let array = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_INSTANCES)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[A0][0] = sx((array as u32).wrapping_add(r32(c, S2)));
                CROWD_AUDIO_UPDATE_INSTANCE.call(c, 0x002a_6d50)?;
                let count = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(CROWD_AUDIO_INSTANCE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_6d40);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ReleaseChunk (0x3e13e8)

/// Stream file header fields.
mod sfile {
    /// The header's mutex (an `EAMutex`).
    pub const MUTEX: u32 = 0x4;
    /// Start and end of the chunk buffer.
    pub const BUFFER_START: u32 = 0x30;
    pub const BUFFER_END: u32 = 0x34;
    /// Read state: 2 = stalled waiting for space (set to 1 = reading again).
    pub const READ_STATE: u32 = 0x38;
    pub const STALLED: u64 = 2;
    /// Read sizes for the next request: normal, and after a low-water hit.
    pub const READ_SIZE: u32 = 0x3c;
    pub const READ_SIZE_LOW: u32 = 0x40;
    /// Set when the buffered bytes fell under the low-water mark.
    pub const LOW_WATER_HIT: u32 = 0x48;
    /// Chunk fields: status (-2 = released) and size in bytes.
    pub const CHUNK_STATUS: u32 = 0x0;
    pub const CHUNK_BYTES: u32 = 0x4;
    pub const RELEASED: u64 = 0xffff_ffff_ffff_fffe;
}

mod sfile_callee {
    use super::{ee, Callee};
    pub const VALIDATE: Callee = Callee { addr: 0x003d_fbd0, f: ee::s14::f_003dfbd0 };
    pub const CONSUME_BYTES: Callee = Callee { addr: 0x003d_fc48, f: ee::s14::f_003dfc48 };
    pub const MUTEX_LOCK: Callee = Callee { addr: 0x003e_5700, f: ee::s14::f_003e5700 };
    pub const MUTEX_UNLOCK: Callee = Callee { addr: 0x003e_5760, f: ee::s14::f_003e5760 };
    pub const ISSUE_READ: Callee = Callee { addr: 0x003e_03d0, f: ee::s14::f_003e03d0 };
}

/// `void StreamFile_ReleaseChunk(StreamFile *file, Chunk *chunk)` (0x3e13e8):
/// gives a consumed chunk's bytes back to the stream's ring buffer. Valid
/// file, chunk inside the buffer and not released yet: mark it released,
/// consume its bytes, and if the reader was stalled for space set it reading
/// again (under the file's mutex) and issue the next read.
pub fn stream_file_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        sfile_callee::VALIDATE.call(c, 0x003e_1408)?;
        release_chunk_validated(c, frame)?;
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The body of `StreamFile_ReleaseChunk` after `StreamFile_Validate` (s0 =
/// chunk), up to the reload of s0.
fn release_chunk_validated(c: &mut Cpu, frame: Frame) -> Flow {
    use sfile::*;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    // The chunk must lie inside [start, end - 8].
    c.cycles += 5;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = header;
    let start = Addr((header as u32).wrapping_add(BUFFER_START)).lw(c);
    c.g[V0][0] = (c.g[S0][0] < start) as u64;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 5;
    let end = Addr((header as u32).wrapping_add(BUFFER_END)).lw(c);
    c.g[V0][0] = (sx((end as u32).wrapping_sub(8)) < c.g[S0][0]) as u64;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(CHUNK_STATUS)).lw(c);
    c.g[V1][0] = RELEASED;
    c.g[S1][0] = STALLED;
    if c.g[V0][0] == RELEASED {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 10;
    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(CHUNK_BYTES)).lw(c);
    Addr(r32(c, S0).wrapping_add(CHUNK_STATUS)).sw(c, RELEASED);
    sfile_callee::CONSUME_BYTES.call(c, 0x003e_1454)?;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = sx((header as u32).wrapping_add(MUTEX));
    sfile_callee::MUTEX_LOCK.call(c, 0x003e_1460)?;
    // s0 = the read state before the change (stalled -> reading).
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[V1][0] = header;
    c.g[S0][0] = Addr((header as u32).wrapping_add(READ_STATE)).lw(c);
    if c.g[S0][0] == c.g[S1][0] {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr((header as u32).wrapping_add(READ_STATE)).sw(c, 1);
    }
    let header = Addr(frame.at(c, 0)).lw(c);
    c.cycles += 4;
    c.g[A0][0] = sx((header as u32).wrapping_add(MUTEX));
    sfile_callee::MUTEX_UNLOCK.call(c, 0x003e_1484)?;
    let was_stalled = c.g[S0][0] == c.g[S1][0];
    // The caller's s0 comes back before the read is issued (IssueRead's
    // prologue saves that value); the caller's own reload repeats it.
    frame.restore(c, S0, 0x30);
    if !was_stalled {
        return Ok(());
    }
    // Was stalled: issue the next read.
    c.cycles += 4;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = header;
    c.g[V0][0] = Addr((header as u32).wrapping_add(LOW_WATER_HIT)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = Addr((header as u32).wrapping_add(READ_SIZE)).lw(c);
        sfile_callee::ISSUE_READ.call(c, 0x003e_14b4)?;
        c.cycles += 1;
    } else {
        c.cycles += 4;
        c.g[A1][0] = Addr((header as u32).wrapping_add(READ_SIZE_LOW)).lw(c);
        sfile_callee::ISSUE_READ.call(c, 0x003e_14a4)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// HUD_DrawStretchedPanel (0x21e7e0)

/// HUD fields read by the stretched panel.
mod hud_panel {
    /// Texture id stored into the draw state before drawing.
    pub const TEXTURE_ID: u32 = 0x474;
    /// Panel texture coordinates: v0, u0, u1, v1.
    pub const TEX_V0: u32 = 0x480;
    pub const TEX_U0: u32 = 0x484;
    pub const TEX_U1: u32 = 0x488;
    pub const TEX_V1: u32 = 0x48c;
    /// World (gp-0x854) -> draw state (+0xe84), whose texture slot is at +0x10.
    pub const WORLD: i32 = -0x854;
    pub const WORLD_DRAW_STATE: u32 = 0xe84;
    pub const DRAW_STATE_TEXTURE: u32 = 0x10;
    /// Width of each fixed end cap of a wide panel.
    pub const CAP: u32 = 0x4210_0000; // 36.0
    pub const WIDE: u32 = 0x4290_0000; // 72.0
}

const HUD_DRAW_PANEL_SEGMENT: Callee = Callee { addr: 0x0021_ea00, f: ee::s05::f_0021ea00 };

/// FPU register `n` as a float.
#[inline]
fn hud_panel_fpr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `lwc1` from `base + off`.
#[inline]
fn hud_panel_lwc1(c: &mut Cpu, base: u64, off: u32) -> FpuF32 {
    Addr((base as u32).wrapping_add(off)).read_fpu(c)
}

/// `void HUD_DrawStretchedPanel(HUD *self, const Color *colour, float pos[2],
/// float size[2], float t, float scale_x, float scale_y, float alpha_min,
/// float alpha_max)` (0x21e7e0): draws a HUD panel scaled about its centre
/// (pos and size are updated in place), with its colour's first component
/// scaled by a triangle wave of `t` between `alpha_min` and `alpha_max`.
/// Panels wider than 72 are drawn as two 36-wide end caps and a stretched
/// middle (`HUD_DrawPanelSegment` three times), others as one segment.
pub fn hud_draw_stretched_panel(c: &mut Cpu) {
    use hud_panel::*;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        let one = FpuF32(0x3f80_0000);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = one.0;
        frame.save(c, S0, 0x50);
        let range = hud_panel_fpr(c, 16) - hud_panel_fpr(c, 15);
        c.f[16] = range.0;
        frame.save(c, S1, 0x40);
        let t = hud_panel_fpr(c, 12);
        let past_peak = abi::c_lt_s(c, one, t);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_fpr(c, 22, 0x70);
        c.g[S3][0] = c.g[SP][0];
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        // A local copy of the colour (16 bytes, possibly unaligned source).
        let colour = r32(c, A1);
        c.g[V0][0] = Addr(colour).read_u64_unaligned(c);
        c.g[V1][0] = Addr(colour.wrapping_add(8)).read_u64_unaligned(c);
        let (lo, hi) = (c.g[V0][0], c.g[V1][0]);
        Addr(frame.at(c, 0)).write_u64_unaligned(c, lo);
        Addr(frame.at(c, 8)).write_u64_unaligned(c, hi);
        let red = Addr(frame.at(c, 0)).read_fpu(c);
        c.f[1] = red.0;
        // Triangle wave: t for t <= 1, else 2 - t.
        let wave = if past_peak {
            c.cycles += 5;
            c.g[AT][0] = 0x4000_0000;
            let fall = FpuF32(0x4000_0000) - t;
            range * fall
        } else {
            c.cycles += 1;
            range * t
        };
        c.cycles += 38;
        let alpha = hud_panel_fpr(c, 15) + wave;
        let red = red * alpha;
        c.f[12] = red.0;
        c.f[0] = one.0;
        let s0 = c.g[S0][0];
        let s2 = c.g[S2][0];
        let width = hud_panel_lwc1(c, s0, 0);
        let height = hud_panel_lwc1(c, s0, 4);
        let half = FpuF32(0x3f00_0000);
        c.f[6] = half.0;
        let x = hud_panel_lwc1(c, s2, 0);
        let y = hud_panel_lwc1(c, s2, 4);
        let grow_h = height * (hud_panel_fpr(c, 14) - one);
        let grow_w = width * (hud_panel_fpr(c, 13) - one);
        c.f[4] = grow_h.0;
        c.f[5] = grow_w.0;
        Addr(frame.at(c, 0)).write_fpu(c, red);
        c.g[AT][0] = sx(WIDE);
        c.f[22] = WIDE;
        let half_h = grow_h * half;
        let half_w = grow_w * half;
        c.f[2] = half_w.0;
        let y = y - half_h;
        let x = x - half_w;
        c.f[3] = x.0;
        Addr((s2 as u32).wrapping_add(4)).write_fpu(c, y);
        Addr(s2 as u32).write_fpu(c, x);
        let width = hud_panel_lwc1(c, s0, 0) + grow_w;
        let height = hud_panel_lwc1(c, s0, 4) + grow_h;
        c.f[0] = width.0;
        c.f[1] = height.0;
        Addr(s0 as u32).write_fpu(c, width);
        Addr((s0 as u32).wrapping_add(4)).write_fpu(c, height);
        // The draw state's texture.
        let world = Addr((c.g[GP][0] as u32).wrapping_add(WORLD as u32)).lw(c);
        c.g[V1][0] = world;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(TEXTURE_ID)).lw(c);
        c.g[V0][0] = Addr((world as u32).wrapping_add(WORLD_DRAW_STATE)).lw(c);
        let texture = c.g[A0][0];
        Addr(r32(c, V0).wrapping_add(DRAW_STATE_TEXTURE)).sh(c, texture);
        let width = hud_panel_lwc1(c, s0, 0);
        c.f[12] = width.0;
        let wide = abi::c_lt_s(c, FpuF32(WIDE), width);
        c.g[A0][0] = c.g[S1][0];
        if wide {
            draw_wide_panel(c)?;
        } else {
            c.cycles += 9;
            let this = c.g[A0][0];
            c.f[13] = hud_panel_lwc1(c, s0, 4).0;
            c.f[14] = hud_panel_lwc1(c, this, TEX_U0).0;
            c.g[A1][0] = c.g[S3][0];
            c.f[15] = hud_panel_lwc1(c, this, TEX_V0).0;
            c.g[A2][0] = c.g[S2][0];
            c.f[16] = hud_panel_lwc1(c, this, TEX_U1).0;
            c.f[17] = hud_panel_lwc1(c, this, TEX_V1).0;
            HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e9d4)?;
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 9;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The wide case of `HUD_DrawStretchedPanel`: left cap (u0..mid), middle
/// (mid..mid), right cap (mid..u1), with `pos[0]` advanced after each.
fn draw_wide_panel(c: &mut Cpu) -> Flow {
    use hud_panel::*;
    c.cycles += 46;
    let this = c.g[S1][0];
    let u0 = hud_panel_lwc1(c, this, TEX_U0);
    let u1 = hud_panel_lwc1(c, this, TEX_U1);
    c.f[0] = u1.0;
    c.g[A1][0] = c.g[S3][0];
    c.f[14] = u0.0;
    c.g[AT][0] = sx(CAP);
    c.f[21] = CAP;
    let size = c.g[S0][0];
    c.f[13] = hud_panel_lwc1(c, size, 4).0;
    c.f[15] = hud_panel_lwc1(c, this, TEX_V0).0;
    c.f[12] = CAP;
    c.f[17] = hud_panel_lwc1(c, this, TEX_V1).0;
    c.g[A2][0] = c.g[S2][0];
    let mid = (u0 + u1) * hud_panel_fpr(c, 6);
    c.f[20] = mid.0;
    c.f[16] = mid.0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e938)?;

    // Middle: pos[0] += 36, width - 72.
    let (s0, s1, s2) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    c.g[A0][0] = s1;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = s2;
    let x = hud_panel_lwc1(c, s2, 0) + hud_panel_fpr(c, 21);
    c.f[0] = x.0;
    c.f[14] = c.f[20];
    c.f[16] = c.f[20];
    Addr(s2 as u32).write_fpu(c, x);
    let width = hud_panel_lwc1(c, s0, 0);
    c.f[13] = hud_panel_lwc1(c, s0, 4).0;
    c.f[12] = (width - hud_panel_fpr(c, 22)).0;
    c.f[15] = hud_panel_lwc1(c, s1, TEX_V0).0;
    c.f[17] = hud_panel_lwc1(c, s1, TEX_V1).0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e970)?;

    // Right cap: pos[0] += width - 72.
    let (s0, s1, s2) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    let middle = hud_panel_lwc1(c, s0, 0) - hud_panel_fpr(c, 22);
    c.f[1] = middle.0;
    c.g[A0][0] = s1;
    let x = hud_panel_lwc1(c, s2, 0) + middle;
    c.f[0] = x.0;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = s2;
    c.f[12] = c.f[21];
    c.f[14] = c.f[20];
    Addr(s2 as u32).write_fpu(c, x);
    c.f[13] = hud_panel_lwc1(c, s0, 4).0;
    c.f[15] = hud_panel_lwc1(c, s1, TEX_V0).0;
    c.f[16] = hud_panel_lwc1(c, s1, TEX_U1).0;
    c.f[17] = hud_panel_lwc1(c, s1, TEX_V1).0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e9ac)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_CalcSurfaceLoopRemoteGroup0 (0x293b50)

const SSX_AUDIO_CALC_SURFACE_LOOP_GROUP0: Callee =
    Callee { addr: 0x0029_3538, f: ee::s07::f_00293538 };

/// `void SSXAudioSystem_CalcSurfaceLoopRemoteGroup0(...)` (0x293b50): the
/// remote (other riders') variant of the surface loop group 0 calculation;
/// forwards its arguments to `SSXAudioSystem_CalcSurfaceLoopGroup0`.
pub fn ssx_audio_system_calc_surface_loop_remote_group0(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        call::Target::Direct(SSX_AUDIO_CALC_SURFACE_LOOP_GROUP0),
        0x0029_3b60,
    );
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow38 (0x2eef60), WorldPainter_GetWeather88 (0x2ee8e8)

/// The world painter's per-view entries (0xf0 bytes each) at 0x4fa370.
mod painter_views {
    pub const ENTRIES: u32 = 0x0050_0000 - 0x5c90;
    pub const ENTRY_SIZE: u64 = 0xf0;
    /// Handles of the light-glow and weather settings objects.
    pub const LIGHT_GLOW: u32 = 0x08;
    pub const WEATHER: u32 = 0x20;
    /// Vtable slots of the float getters used here.
    pub const VT_GLOW38: u32 = 0x98;
    pub const VT_WEATHER88: u32 = 0x1d8;
}

/// The shared body of the world painter's float getters: `view`'s settings
/// handle at `field`, its object's virtual getter at vtable offset `slot`
/// (returning a `float *`), the float it points at in f0.
fn world_painter_view_float(c: &mut Cpu, field: u32, slot: u32, ret_at: u32) {
    use painter_views::*;
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = ENTRY_SIZE;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(ENTRIES);
        let entry = r32(c, A0).wrapping_add(ENTRIES);
        c.g[A0][0] = sx(entry);
        c.g[V1][0] = Addr(entry.wrapping_add(field)).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        let obj = r32(c, A2);
        c.g[A1][0] = Addr(obj.wrapping_add(4)).lw(c);
        let vt = r32(c, A1);
        let delta = Addr(vt.wrapping_add(slot)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
        let getter = r32(c, V0);
        c.g[A0][0] = sx(obj.wrapping_add(delta as u32));
        call_ptr(c, getter, ret_at)?;
        let value = Addr(r32(c, V0)).read_fpu(c);
        abi::set_fret(c, value);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLightGlow38(int view)` (0x2eef60): a light-glow
/// setting of `view` (virtual getter at vtable +0x98).
pub fn world_painter_get_light_glow38(c: &mut Cpu) {
    use painter_views::*;
    world_painter_view_float(c, LIGHT_GLOW, VT_GLOW38, 0x002e_ef98);
}

/// `float WorldPainter_GetWeather88(int view)` (0x2ee8e8): a weather setting
/// of `view` (virtual getter at vtable +0x1d8).
pub fn world_painter_get_weather88(c: &mut Cpu) {
    use painter_views::*;
    world_painter_view_float(c, WEATHER, VT_WEATHER88, 0x002e_e920);
}

// ---------------------------------------------------------------------------
// TransitionEffect_UpdateScreenRect (0x2eb8f0)

/// Globals of the transition effect (gp-relative).
mod transition_rect {
    /// The world object (gp+0x2a90); its vtable pointer at +0x10d8, the
    /// screen-rect getter at vtable +0x150.
    pub const GP_WORLD: u32 = 0x2a90;
    pub const WORLD_VTABLE: u32 = 0x10d8;
    pub const VT_SCREEN_RECT: u32 = 0x150;
    /// The rect fields read: left (+4) and width (+8).
    pub const RECT_LEFT: u32 = 0x4;
    pub const RECT_WIDTH: u32 = 0x8;
    /// Cached left, right (left + width) and width (right - left).
    pub const GP_LEFT: u32 = 0xa4c;
    pub const GP_RIGHT: u32 = 0xa50;
    pub const GP_WIDTH: u32 = 0xa54;
}

/// `void TransitionEffect_UpdateScreenRect(void)` (0x2eb8f0): caches the
/// world's screen rect for the transition effect: left, right = left + width
/// and the width recomputed as right - left.
pub fn transition_effect_update_screen_rect(c: &mut Cpu) {
    use transition_rect::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = Addr(gp.wrapping_add(GP_WORLD)).lw(c);
        frame.save_ra(c, 0);
        let world = r32(c, A1);
        c.g[V1][0] = Addr(world.wrapping_add(WORLD_VTABLE)).lw(c);
        let vt = r32(c, V1);
        let delta = Addr(vt.wrapping_add(VT_SCREEN_RECT)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(VT_SCREEN_RECT + 4)).lw(c);
        let getter = r32(c, V0);
        c.g[A0][0] = sx(world.wrapping_add(delta as u32));
        call_ptr(c, getter, 0x002e_b910)?;
        let rect = r32(c, V0);
        let left = Addr(rect.wrapping_add(RECT_LEFT)).read_fpu(c);
        let width = Addr(rect.wrapping_add(RECT_WIDTH)).read_fpu(c);
        let gp = c.g[GP][0] as u32;
        Addr(gp.wrapping_add(GP_LEFT)).write_fpu(c, left);
        let right = width + left;
        c.f[1] = right.0;
        frame.restore_ra(c, 0);
        let span = right - left;
        c.f[0] = span.0;
        Addr(gp.wrapping_add(GP_RIGHT)).write_fpu(c, right);
        Addr(gp.wrapping_add(GP_WIDTH)).write_fpu(c, span);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_GetTrickBonus (0x117908)

/// The metrix's trick bonus factor (float).
const RIDER_METRIX_TRICK_BONUS: u32 = 0x1c;
/// The scale applied to it (gp-0x7b0c, 10000.0 in the game).
const RIDER_METRIX_BONUS_SCALE_GP: u32 = 0x7b0c;

/// `int RiderMetrix_GetTrickBonus(RiderMetrix *self)` (0x117908): the trick
/// bonus factor scaled to points, `(int)(factor * scale + 5)` rounded down
/// to a multiple of 10 (toward zero).
pub fn rider_metrix_get_trick_bonus(c: &mut Cpu) {
    c.cycles += 7;
    let factor = Addr(r32(c, A0).wrapping_add(RIDER_METRIX_TRICK_BONUS)).read_fpu(c);
    let gp = c.g[GP][0] as u32;
    let scale = Addr(gp.wrapping_sub(RIDER_METRIX_BONUS_SCALE_GP)).read_fpu(c);
    c.f[2] = scale.0;
    c.g[A0][0] = 0xa;
    c.g[AT][0] = 0x40a0_0000;
    let five = FpuF32(0x40a0_0000);
    c.cycles += 8;
    let rounded = factor * scale + five;
    c.f[0] = rounded.0;
    let points = rounded.to_i32();
    c.f[1] = points as u32;
    c.g[V0][0] = sx(points as u32);
    let (_, rem) = abi::div(c, points as u32, 10);
    c.g[V1][0] = sx(rem);
    c.g[V0][0] = sx((points as u32).wrapping_sub(rem));
    ret(c);
}

// ---------------------------------------------------------------------------
// Rand_NextGame (0x317810)

/// The game's random generator state (0x4ff030).
const RAND_GAME_STATE: u32 = 0x0050_0000 - 0xfd0;
const RAND_NEXT: Callee = Callee { addr: 0x0031_7a08, f: ee::s10::f_00317a08 };

/// `u32 Rand_NextGame(void)` (0x317810): the next number of the game's
/// random generator (`Rand_Next(&g_GameRand)`).
pub fn rand_next_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(RAND_GAME_STATE);
        RAND_NEXT.call(c, 0x0031_7824)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_paths, fill, put_words, scramble, Rng, DATA, GAME_GP};

    /// Slot table entries point at streams in DATA with random buffer counts.
    #[test]
    fn sndstrm_get_buffered_bytes_matches() {
        check(ee::s13::f_003c6c98__generated, sndstrm_get_buffered_bytes, 300, &|c, seed| {
            let mut r = Rng(0x3c6c_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x003c_6c98;
            let slot = r.next_u32() % 8;
            // Upper bits of a0 are ignored by the `sll`.
            c.g[A0][0] = (r.next_u32() as u64) << 32 | slot as u64;
            let stream = DATA + 0x100 * (slot + 1);
            c.bus.write32(SNDSTRM_SLOT_TABLE + 4 * slot, stream);
            c.bus.write32(stream + SNDSTRM_BUFFERED_A, r.next_u32());
            c.bus.write32(stream + SNDSTRM_BUFFERED_B, r.next_u32());
        });
    }

    /// 0..3 instances (sometimes a negative count); the instances are
    /// inactive (+0xd4 = 0), so the real `CrowdAudio_UpdateInstance` takes
    /// its short path.
    #[test]
    fn crowd_audio_update_matches() {
        check(ee::s08::f_002a6d18__generated, crowd_audio_update, 200, &|c, seed| {
            let mut r = Rng(0x2a6d_1800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x002a_6d18;
            c.g[GP][0] = GAME_GP as u64;
            let count: i32 = match seed % 5 {
                0 => 0,
                1 => -(r.next_u32() as i32 & 0xff) - 1,
                n => n as i32 - 1,
            };
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + CROWD_AUDIO_COUNT, count as u32);
            let array = DATA + 0x1000;
            c.bus.write32(DATA + CROWD_AUDIO_INSTANCES, array);
            for i in 0..4 {
                c.bus.write32(array + i * CROWD_AUDIO_INSTANCE_SIZE + 0xd4, 0);
            }
        });
    }

    /// Stream file fixture: handle at DATA -> header at DATA+0x1000 (starting
    /// with the magic 'MRTS'), buffer DATA+0x2000..DATA+0x3000, chunk inside or outside it.
    /// The mutex syscalls go to the fakes (some unwind). The real
    /// `StreamFile_IssueRead` takes its short path: no pending request, the
    /// ring full enough that it only marks the reader stalled again.
    #[test]
    fn stream_file_release_chunk_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(ee::s14::f_003e13e8__generated, stream_file_release_chunk, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3e13e8, 10);
            c.pc = 0x003e_13e8;
            let handle = DATA;
            let header = DATA + 0x1000;
            let start = DATA + 0x2000;
            let end = DATA + 0x3000;
            let case = seed % 9;
            c.g[A0][0] = if case == 0 { 0 } else { handle as u64 };
            c.bus.write32(handle, header);
            let magic = 0x4d52_5453;
            c.bus.write32(header, if case == 1 { magic ^ (1 + r.next_u32() % 0xff) } else { magic });
            c.bus.write32(header + sfile::BUFFER_START, start);
            c.bus.write32(header + sfile::BUFFER_END, end);
            let chunk = match case {
                2 => start - 4 * (1 + r.next_u32() % 4),
                3 => end - 8 + 4 * (1 + r.next_u32() % 4),
                4 => end - 8,
                _ => start + 0x100,
            };
            c.g[A1][0] = chunk as u64;
            let status = if case == 5 { 0xffff_fffe } else { r.next_u32() & 0xff };
            c.bus.write32(chunk + sfile::CHUNK_STATUS, status);
            c.bus.write32(chunk + sfile::CHUNK_BYTES, r.next_u32() & 0xfff);
            // Read state: mostly stalled (2), sometimes reading (1) or idle.
            let state = match r.next_u32() % 4 {
                0 => r.next_u32() % 2,
                _ => 2,
            };
            c.bus.write32(header + sfile::READ_STATE, state);
            c.bus.write32(header + sfile::READ_SIZE, r.next_u32() & 0xffff);
            c.bus.write32(header + sfile::READ_SIZE_LOW, r.next_u32() & 0xffff);
            c.bus.write32(header + sfile::LOW_WATER_HIT, r.next_u32() & 1);
            // StreamFile_ConsumeBytes: buffered count and low-water mark (kept
            // above the count in state 1, so it never reprioritises a request).
            c.bus.write32(header + 0x4c, r.next_u32() & 0xfffff);
            let low = if state == 1 { 0x7fff_ffff } else { r.next_u32() & 0xfffff };
            c.bus.write32(header + 0x44, low);
            // Mutex: owner, depth, semaphore (owner never the fake thread id).
            c.bus.write32(header + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(header + 0xc, r.next_u32() & 3);
            c.bus.write32(header + 0x10, r.next_u32() & 0xff);
            // StreamFile_IssueRead short path.
            c.bus.write32(header + 0x50, DATA + 0x2200);
            c.bus.write32(header + 0x54, DATA + 0x2200);
            c.bus.write32(header + 0x58, DATA + 0x2100);
            c.bus.write32(header + 0x5c, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000 + 0xc, 0);
            c.bus.write32(header + 0x17c, 0x1000);
        });
    }

    /// A HUD at DATA with random texture coordinates, a (sometimes
    /// unaligned) colour, pos/size in DATA; widths below and above 72 (the
    /// three-segment path), t on both sides of the triangle-wave peak. The
    /// world's draw callback (`HUD_DrawPanelSegment`'s vtable call) is a fake
    /// that sometimes unwinds.
    #[test]
    fn hud_draw_stretched_panel_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(ee::s05::f_0021e7e0__generated, hud_draw_stretched_panel, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x21e7e0, 3);
            c.pc = 0x0021_e7e0;
            let hud = DATA;
            let colour = DATA + 0x100 + (seed % 4) * 4 + (seed % 3);
            let pos = DATA + 0x200;
            let size = DATA + 0x210;
            let world = DATA + 0x1000;
            let draw_state = DATA + 0x3000;
            let vtable = DATA + 0x5000;
            c.g[A0][0] = hud as u64;
            c.g[A1][0] = colour as u64;
            c.g[A2][0] = pos as u64;
            c.g[A3][0] = size as u64;
            c.bus.write32(hud + hud_panel::TEXTURE_ID, r.next_u32());
            for off in [hud_panel::TEX_V0, hud_panel::TEX_U0, hud_panel::TEX_U1, hud_panel::TEX_V1] {
                c.bus.write32(hud + off, ((r.next_u32() % 1000) as f32 / 1000.0).to_bits());
            }
            // Byte by byte: the colour may be unaligned.
            for i in 0..4 {
                let bits = ((r.next_u32() % 256) as f32).to_bits();
                for (k, b) in bits.to_le_bytes().into_iter().enumerate() {
                    c.bus.write8(colour + 4 * i + k as u32, b);
                }
            }
            c.bus.write32(pos, r.f32(400.0));
            c.bus.write32(pos + 4, r.f32(300.0));
            let width = if seed % 2 == 0 { 20 + r.next_u32() % 50 } else { 75 + r.next_u32() % 300 };
            c.bus.write32(size, (width as f32).to_bits());
            c.bus.write32(size + 4, ((10 + r.next_u32() % 120) as f32).to_bits());
            // t in [0, 2), scales around 1 (rarely exactly 1), alpha range.
            c.f[12] = ((r.next_u32() % 2000) as f32 / 1000.0).to_bits();
            c.f[13] = (0.8 + (r.next_u32() % 400) as f32 / 1000.0).to_bits();
            c.f[14] = (0.8 + (r.next_u32() % 400) as f32 / 1000.0).to_bits();
            if seed % 7 == 0 {
                c.f[13] = 1.0f32.to_bits();
            }
            c.f[15] = ((r.next_u32() % 500) as f32 / 1000.0).to_bits();
            c.f[16] = (0.5 + (r.next_u32() % 500) as f32 / 1000.0).to_bits();
            c.bus.write32(GAME_GP - 0x854, world);
            c.bus.write32(world + hud_panel::WORLD_DRAW_STATE, draw_state);
            c.bus.write32(world + 0x10d8, vtable);
            c.bus.write16(vtable + 0x238, (r.next_u32() & 0x3c) as u16);
            c.bus.write32(vtable + 0x23c, FAKE);
        });
    }

    /// The audio system at DATA, the rider at DATA+0x10000 with its view
    /// object's vtable call faked (forced to a small surface index, sometimes
    /// unwinding); the curve tables and constants come from the ELF image.
    #[test]
    fn ssx_audio_system_calc_surface_loop_remote_group0_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        covered(
            ee::s07::f_00293b50__generated,
            ssx_audio_system_calc_surface_loop_remote_group0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x293b50, 1);
                elf_image(c);
                c.pc = 0x0029_3b50;
                let system = DATA;
                let rider = DATA + 0x10000;
                let vt = DATA + 0x20000;
                let out = DATA + 0x30000;
                c.g[A0][0] = system as u64;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = (out + 4) as u64;
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x3c, FAKE);
                for off in [0x750, 0x754, 0x758, 0x75c] {
                    c.bus.write32(rider + off, ((r.next_u32() % 2000) as f32 / 1000.0).to_bits());
                }
                c.bus.write32(LOG_RESULTS, 1 + seed % 4);
                for i in 0..8 {
                    c.bus.write32(system + 0x59b8 + 4 * i, ((r.next_u32() % 2000) as f32 / 1000.0).to_bits());
                }
            },
        );
    }

    /// Shared fixture of the world painter getters: entry `view` points at a
    /// handle -> object -> vtable whose getter is a fake returning a pointer
    /// to a float (sometimes unwinding).
    fn world_painter_view_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32, field: u32, slot: u32) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use painter_views::*;
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        let view = r.next_u32() % 8;
        let handle = DATA + 0x100;
        let obj = DATA + 0x200;
        let vt = DATA + 0x400;
        let value = DATA + 0x800;
        c.bus.write32(ENTRIES + view * ENTRY_SIZE as u32 + field, handle);
        c.bus.write32(handle, obj);
        c.bus.write32(obj + 4, vt);
        c.bus.write16(vt + slot, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(vt + slot + 4, FAKE);
        c.bus.write32(value, r.f32(100.0));
        c.bus.write32(LOG_RESULTS, value);
        c.g[A0][0] = (r.next_u32() as u64) << 32 | view as u64;
    }

    #[test]
    fn world_painter_get_light_glow38_matches() {
        use painter_views::*;
        crate::difftest::fakes::covered(
            ee::s09::f_002eef60__generated,
            world_painter_get_light_glow38,
            200,
            &|c, seed| world_painter_view_fixture(c, seed, 0x2eef60, 0x002e_ef60, LIGHT_GLOW, VT_GLOW38),
        );
    }

    #[test]
    fn world_painter_get_weather88_matches() {
        use painter_views::*;
        crate::difftest::fakes::covered(
            ee::s09::f_002ee8e8__generated,
            world_painter_get_weather88,
            200,
            &|c, seed| world_painter_view_fixture(c, seed, 0x2ee8e8, 0x002e_e8e8, WEATHER, VT_WEATHER88),
        );
    }

    /// The world (gp+0x2a90) at DATA with its vtable's screen-rect getter a
    /// fake returning a pointer to a random rect (sometimes unwinding).
    #[test]
    fn transition_effect_update_screen_rect_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use transition_rect::*;
        covered(
            ee::s09::f_002eb8f0__generated,
            transition_effect_update_screen_rect,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eb8f0, 1);
                c.pc = 0x002e_b8f0;
                let world = DATA;
                let vt = DATA + 0x2000;
                let rect = DATA + 0x3000;
                c.bus.write32(GAME_GP + GP_WORLD, world);
                c.bus.write32(world + WORLD_VTABLE, vt);
                c.bus.write16(vt + VT_SCREEN_RECT, ((r.next_u32() as i16) & !3) as u16);
                c.bus.write32(vt + VT_SCREEN_RECT + 4, FAKE);
                c.bus.write32(rect + RECT_LEFT, if seed % 4 == 0 { r.edge_f32() } else { r.f32(640.0) });
                c.bus.write32(rect + RECT_WIDTH, if seed % 4 == 1 { r.edge_f32() } else { r.f32(640.0) });
                c.bus.write32(LOG_RESULTS, rect);
            },
        );
    }

    /// Random and edge-case bonus factors (both signs, huge values that
    /// saturate the conversion), the game's scale and random ones.
    #[test]
    fn rider_metrix_get_trick_bonus_matches() {
        crate::difftest::check_lohi(ee::s00::f_00117908__generated, rider_metrix_get_trick_bonus, 300, &|c, seed| {
            let mut r = Rng(0x0011_7908 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.pc = 0x0011_7908;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let factor = match seed % 4 {
                0 => r.edge_f32(),
                1 => r.f32(1.0),
                _ => r.f32(10.0),
            };
            c.bus.write32(DATA + RIDER_METRIX_TRICK_BONUS, factor);
            let scale = if seed % 5 == 0 { r.edge_f32() } else { 10000.0f32.to_bits() };
            c.bus.write32(GAME_GP - RIDER_METRIX_BONUS_SCALE_GP, scale);
        });
    }

    /// Random generator state (all six words random, so both carry paths
    /// of `Rand_Next` are taken).
    #[test]
    fn rand_next_game_matches() {
        check(ee::s10::f_00317810__generated, rand_next_game, 200, &|c, seed| {
            let mut r = Rng(0x0031_7810 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0031_7810;
            fill(c, &mut r, RAND_GAME_STATE, 8);
        });
    }
}
