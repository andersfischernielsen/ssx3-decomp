//! Staging wave 206 (oc250): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{self, call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_8948, vfx_avalanche_update, "VfxAvalanche_Update"),
    (0x003d_5290, path_find_track, "PATH_FindTrack"),
    (0x0011_4db8, rider_try_air_reverse, "Rider_TryAirReverse"),
    (0x0031_ad20, ssx_cd_tray_callback, "ssx_CdTrayCallback"),
    (0x0028_3af8, script_movie_toggle_pause, "ScriptMovie_TogglePause"),
    (0x002e_e448, world_painter_get_weather08, "WorldPainter_GetWeather08"),
    (0x0034_4138, crowd_man3d_render, "CrowdMan3d_Render"),
    (0x0029_d610, ssx_audio_system_update_loop_sfx, "SSXAudioSystem_UpdateLoopSfx"),
    (0x0032_c0f8, collide_mesh_leaf_triangles, "Collide_MeshLeafTriangles"),
];

/// `sw reg, addr`.
#[inline]
fn sw_reg(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sw(c, v);
}

/// `gp`-relative global address.
#[inline]
fn gp_at(c: &Cpu, off: i32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_add(off as u32))
}

// ---------------------------------------------------------------------------
// VfxAvalanche_Update (0x2d8948)

/// Avalanche manager globals (gp-relative).
mod avg {
    /// Avalanches enabled.
    pub const ENABLED: i32 = 0x940;
    /// Bake requested (re-bake every history at the next update).
    pub const BAKE_PENDING: i32 = 0x944;
    /// Start requested (`< 0`: one-shot, cleared after the start).
    pub const START_PENDING: i32 = 0x96c;
    /// Update active instances (`< 0`: one-shot, cleared after the pass).
    pub const UPDATE_PENDING: i32 = 0x970;
    /// The avalanche to start.
    pub const START_ID: i32 = 0x98c;
    /// Has data (avalanche histories loaded).
    pub const HAVE_DATA: i32 = 0x9cc;
    /// Cleared together with the bake request.
    pub const BAKE_CLEAR: i32 = 0x9d4;
    /// The 16 active avalanches (0x1c bytes each) at 0x538938.
    pub const ACTIVE: u32 = 0x0054_0000 - 0x76c8;
    pub const ACTIVE_SIZE: u32 = 0x1c;
    pub const ACTIVE_COUNT: u32 = 16;
}

mod av_callee {
    use super::{ee, Callee};
    pub const BAKE_ALL: Callee = Callee { addr: 0x002d_96e0, f: ee::s09::f_002d96e0 };
    pub const APPLY_HISTORY_DATA: Callee = Callee { addr: 0x002d_a028, f: ee::s09::f_002da028 };
    pub const START: Callee = Callee { addr: 0x002d_97a8, f: ee::s09::f_002d97a8 };
    pub const UPDATE_ACTIVE: Callee = Callee { addr: 0x002d_7ef8, f: ee::s09::f_002d7ef8 };
}

/// `void VfxAvalanche_Update(void)` (0x2d8948): the avalanche manager's per
/// frame update. When enabled: runs a pending bake (`Avalanche_BakeAll`),
/// re-applies the history data, starts a requested avalanche, and updates
/// the 16 active avalanches when histories are loaded and an update is
/// requested. Negative (one-shot) requests are cleared after use.
pub fn vfx_avalanche_update(c: &mut Cpu) {
    use avg::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = gp_at(c, ENABLED).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            update_enabled(c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The enabled part of `VfxAvalanche_Update` (up to the epilogue).
fn update_enabled(c: &mut Cpu) -> call::Flow {
    use avg::*;
    c.cycles += 3;
    c.g[V0][0] = gp_at(c, BAKE_PENDING).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        av_callee::BAKE_ALL.call(c, 0x002d_8974)?;
        gp_at(c, BAKE_PENDING).sw(c, 0);
        gp_at(c, BAKE_CLEAR).sw(c, 0);
    }
    c.cycles += 5;
    av_callee::APPLY_HISTORY_DATA.call(c, 0x002d_8984)?;
    let start = gp_at(c, START_PENDING).lw(c);
    c.g[V0][0] = gp_at(c, ENABLED).lw(c);
    if start != 0 {
        c.cycles += 5;
        c.g[A0][0] = gp_at(c, START_ID).lw(c);
        av_callee::START.call(c, 0x002d_8998)?;
        c.g[V1][0] = gp_at(c, START_PENDING).lw(c);
        if (c.g[V1][0] as i64) < 0 {
            gp_at(c, START_PENDING).sw(c, 0);
        }
        c.cycles += 1;
        c.g[V0][0] = gp_at(c, ENABLED).lw(c);
    }
    // Still enabled, histories loaded, update requested.
    c.cycles += 2;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = gp_at(c, HAVE_DATA).lw(c);
    if !enabled {
        return Ok(());
    }
    c.cycles += 2;
    let have_data = c.g[V0][0] != 0;
    c.g[V0][0] = gp_at(c, UPDATE_PENDING).lw(c);
    if !have_data {
        return Ok(());
    }
    c.cycles += 2;
    let update = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0054_0000;
    if !update {
        return Ok(());
    }
    c.cycles += 2;
    c.g[S0][0] = sx(ACTIVE);
    c.g[S1][0] = sx(ACTIVE + ACTIVE_COUNT * ACTIVE_SIZE);
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        av_callee::UPDATE_ACTIVE.call(c, 0x002d_89d0)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(ACTIVE_SIZE));
        let more = (c.g[S0][0] as i64) < (c.g[S1][0] as i64);
        c.g[V0][0] = gp_at(c, UPDATE_PENDING).lw(c);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002d_89c8);
        }
    }
    c.cycles += 2;
    if (c.g[V0][0] as i64) < 0 {
        gp_at(c, UPDATE_PENDING).sw(c, 0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// PATH_FindTrack (0x3d5290)

/// The PATH module's four loaded track files (0x928 bytes each) at 0x517610.
mod path {
    pub const SLOTS: u32 = 0x0051_7610;
    pub const SLOT_SIZE: u32 = 0x928;
    pub const SLOT_COUNT: u64 = 4;
    /// Which id classes the file holds (top nibble: kind, next nibble: group).
    pub const MASK: u32 = 0x30;
    /// Non-zero when the slot is loaded.
    pub const LOADED: u32 = 0x34;
    /// The 24 tracks, one per bit of the id's low 24 bits.
    pub const TRACKS: u32 = 0x58;
    pub const TRACK_BITS: u64 = 24;
}

/// `PATHTrack *PATH_FindTrack(u32 id)` (0x3d5290): the track named by `id`:
/// the first loaded slot whose class mask shares the id's top nibble and its
/// group nibble, then the track of the lowest set bit among the id's low 24
/// bits. 0 when no slot has it.
pub fn path_find_track(c: &mut Cpu) {
    use path::*;
    c.cycles += 14;
    let id = c.g[A0][0];
    c.g[T3][0] = sx(SLOTS + TRACKS);
    c.g[T2][0] = id & 0xffff_ffff_f000_0000;
    c.g[T1][0] = id & 0x0f00_0000;
    c.g[A2][0] = 0x00ff_ffff;
    c.g[T0][0] = 0;
    c.g[A0][0] = id & 0x00ff_ffff;
    c.g[A1][0] = sx(SLOTS + LOADED);
    c.g[A3][0] = 0;
    loop {
        // a1: this slot's LOADED word; a3: the slot's offset.
        c.cycles += 3;
        let loaded = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = loaded;
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(LOADED - MASK)).lw(c);
        let mut skip_cycle = true;
        if loaded != 0 {
            c.cycles += 3;
            let mask = c.g[V1][0];
            let kind = mask & c.g[T2][0];
            c.g[V0][0] = mask & c.g[T1][0];
            if kind != 0 {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    skip_cycle = false;
                } else {
                    // The track of the id's lowest set bit.
                    c.cycles += 2;
                    c.g[A2][0] = 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    loop {
                        c.cycles += 4;
                        let bit = ((c.g[A0][0] as u32) >> ((c.g[A2][0] as u32) & 31)) & 1;
                        c.g[V0][0] = bit as u64;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        if bit != 0 {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            ret(c);
                            return;
                        }
                        c.cycles += 3;
                        let more = c.g[A2][0] < TRACK_BITS;
                        c.g[V0][0] = more as u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_52f0);
                        }
                    }
                }
            }
        }
        if skip_cycle {
            c.cycles += 1;
        }
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SLOT_SIZE));
        let more = c.g[T0][0] < SLOT_COUNT;
        c.g[V0][0] = more as u64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(SLOT_SIZE));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_52c8);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// Rider_TryAirReverse (0x114db8)

/// VU0 macro sequences as the compiler emitted them (same registers).
mod vu {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    use ssx3_ee_rt::Cpu;

    /// `a · b` (xyz) into GPR `g` and FPR `f`: vf3 = a, vf5 = b, w lane of vf4.
    pub fn dot(c: &mut Cpu, a: u32, b: u32, g: usize, f: usize) {
        c.lqc2(3, a);
        c.lqc2(5, b);
        dot_vf3_vf5(c, g, f);
    }

    /// The dot product of vf3 and vf5 (already loaded).
    pub fn dot_vf3_vf5(c: &mut Cpu, g: usize, f: usize) {
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(g, 4);
        c.mtc1(g, f);
    }

    /// `dst = src / |src|` (vf3 = src, Q = rsqrt).
    pub fn normalize(c: &mut Cpu, src: u32, dst: u32) {
        c.lqc2(3, src);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, dst);
    }

    /// `dst = src * f` (through GPR `g`: mfc1, qmtc2 into vf3).
    pub fn scale(c: &mut Cpu, f: usize, src: u32, dst: u32, g: usize) {
        c.mfc1(g, f);
        c.lqc2(4, src);
        c.qmtc2(g, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, dst);
    }

    /// `dst = a - b` (vf3 = a, vf4 = b).
    pub fn sub(c: &mut Cpu, a: u32, b: u32, dst: u32) {
        c.lqc2(3, a);
        c.lqc2(4, b);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, dst);
    }

    /// `dst = a + b` (vf3 = a, vf4 = b).
    pub fn add(c: &mut Cpu, a: u32, b: u32, dst: u32) {
        c.lqc2(3, a);
        c.lqc2(4, b);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, dst);
    }

    /// A 128-bit copy through GPR `g` (`lq`, `sq`).
    pub fn copy(c: &mut Cpu, src: u32, dst: u32, g: usize) {
        c.lq(g, src);
        c.sq(g, dst);
    }
}

/// Rider fields used by `Rider_TryAirReverse`.
mod air_reverse {
    /// Position (vec4).
    pub const POS: u32 = 0x110;
    /// Orientation quaternion.
    pub const ROT: u32 = 0x120;
    /// Velocity.
    pub const VEL: u32 = 0x170;
    /// Up axis.
    pub const UP: u32 = 0x180;
    /// Ground point.
    pub const GROUND: u32 = 0x1e0;
    /// The character (board-relative bone table).
    pub const CHARACTER: u32 = 0x780;
    /// Index of the reverse pivot bone.
    pub const PIVOT_BONE: u32 = 0x89c;
    /// The air predictor.
    pub const PREDICTOR: u32 = 0x788;
    /// Predictor: state (1, 3: landing ahead), surface, landing normal and point.
    pub const PRED_STATE: u32 = 0xac;
    pub const PRED_SURFACE: u32 = 0x90;
    pub const PRED_NORMAL: u32 = 0x10;
    pub const PRED_POINT: u32 = 0x20;
    /// World: game at +0x84, its surface table (0xb0 per surface) at +0x44.
    pub const SURFACE_SIZE: u32 = 0xb0;
    /// The surface type whose landing plane is the rider's own.
    pub const SURFACE_OWN_PLANE: u64 = 0x12;
    /// Literals: dot limits (gp-0x7b94, gp-0x7b90), the prime time
    /// (gp-0x7b8c), the reverse limit (0x4a0ea8), the origin (0x4ff130).
    pub const GP_VEL_LIMIT: u32 = 0x7b94;
    pub const GP_HEIGHT_LIMIT: u32 = 0x7b90;
    pub const GP_PRIME_TIME: u32 = 0x7b8c;
    pub const REVERSE_LIMIT: u32 = 0x004a_0ea8;
    pub const ORIGIN: u32 = 0x004f_f130;

    use super::{ee, Callee};
    pub const BUILD_WORLD_MATRIX: Callee = Callee { addr: 0x0011_e098, f: ee::s00::f_0011e098 };
    pub const TOGGLE_STANCE: Callee = Callee { addr: 0x0011_5168, f: ee::s00::f_00115168 };
    pub const AIR_PREDICTOR_PRIME: Callee = Callee { addr: 0x0011_35b8, f: ee::s00::f_001135b8 };
}

/// `bool Rider_TryAirReverse(Rider *rider)` (0x114db8): in the air with a
/// landing predicted (predictor state 1 or 3), check the rider is above the
/// landing plane, moving toward it and high enough; then flip the rider
/// around the pivot bone (a half turn so it lands facing the other way):
/// move the position by the rotated pivot offset, rebuild the world matrix,
/// switch stance and re-prime the air predictor. Returns whether it did.
pub fn rider_try_air_reverse(c: &mut Cpu) {
    use air_reverse::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0xb0);
        frame.save(c, S0, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        frame.save_ra(c, 0x90);
        let rider = r32(c, S1);
        let pred = Addr(rider + PREDICTOR).lw(c) as u32;
        c.g[V1][0] = Addr(pred + PRED_STATE).lw(c);
        c.g[A0][0] = 0;
        let landing = if c.g[V1][0] == 1 {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            c.g[V1][0] == 3
        };
        if landing {
            c.cycles += 1;
            c.g[A0][0] = 1;
        }
        c.cycles += 2;
        c.g[V1][0] = SURFACE_SIZE as u64;
        if !landing {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            try_reverse(c)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `c.lt.s fs, ft` and the condition bit.
#[inline]
fn lt(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    c.c_lt_s(fs, ft);
    c.fcc()
}

/// `Rider_TryAirReverse` past the predictor-state test: leaves the result in
/// v0.
fn try_reverse(c: &mut Cpu) -> call::Flow {
    use air_reverse::*;
    let rider = r32(c, S1);
    let s = sp(c);
    // Two axes of the landing plane (sp, sp+0x10) and the rider's up at
    // sp+0x20: the surface table says whether they are the rider's own (up,
    // ground) or the predictor's landing pair.
    c.cycles += 11;
    c.g[T0][0] = Addr(rider + PREDICTOR).lw(c);
    let pred = c.g[T0][0] as u32;
    c.g[V0][0] = gp_at(c, -0x848).lw(c);
    let world = c.g[V0][0] as u32;
    c.g[A2][0] = SURFACE_OWN_PLANE;
    c.g[A1][0] = Addr(pred + PRED_SURFACE).lw(c);
    c.g[A0][0] = Addr(world + 0x84).lw(c);
    c.mult(0, V1, A1, V1);
    c.lq(A3, rider + UP);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x44)).lw(c);
    c.sq(A3, s + 0x20);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let mut own_plane = c.g[A1][0] == c.g[A2][0];
    if !own_plane {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
        if c.g[V0][0] == 0 {
            // The predicted landing plane.
            c.lq(V0, pred + PRED_POINT);
            c.cycles += 3;
            c.sq(V0, s);
            vu::copy(c, pred + PRED_NORMAL, s + 0x10, V1);
        } else {
            own_plane = true;
        }
    }
    if own_plane {
        c.cycles += 4;
        c.lq(V0, rider + GROUND);
        c.sq(A3, s);
        c.sq(V0, s + 0x10);
    }

    // The rider must be above the plane (up · plane >= 0) ...
    c.cycles += 14;
    c.mtc1(0, 6);
    vu::dot(c, s + 0x20, s, V0, 0);
    let below = lt(c, 0, 6);
    c.g[V0][0] = 0;
    if below {
        return Ok(());
    }
    // ... moving into it (vel · ground under the limit) ...
    c.cycles += 17;
    c.lq(V0, rider + VEL);
    c.g[S2][0] = sx(rider + GROUND);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(GP_VEL_LIMIT));
    c.sq(V0, s + 0x30);
    vu::dot(c, s + 0x30, rider + GROUND, V1, 1);
    let too_fast = lt(c, 0, 1);
    c.g[V0][0] = 0;
    if too_fast {
        return Ok(());
    }
    // ... and along the plane's own direction.
    c.cycles += 14;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(GP_HEIGHT_LIMIT));
    vu::dot(c, s + 0x30, s, V0, 1);
    let away = lt(c, 0, 1);
    c.g[V0][0] = 0;
    if away {
        return Ok(());
    }

    // The velocity and the plane's second axis without their components
    // along the plane, normalized; their dot product must stay within the
    // reverse limit.
    c.cycles += 75;
    vu::scale(c, 1, s, s + 0x70, V0);
    c.g[V1][0] = 0x004a_0000;
    vu::copy(c, s + 0x70, s + 0x60, V0);
    vu::sub(c, s + 0x30, s + 0x60, s + 0x70);
    vu::copy(c, s + 0x70, s + 0x50, V0);
    vu::normalize(c, s + 0x50, s + 0x70);
    vu::copy(c, s + 0x70, s + 0x40, V0);
    vu::dot(c, s + 0x10, s, A0, 0);
    vu::scale(c, 0, s, s + 0x80, A1);
    vu::copy(c, s + 0x80, s + 0x70, V0);
    vu::sub(c, s + 0x10, s + 0x70, s + 0x80);
    vu::copy(c, s + 0x80, s + 0x60, V0);
    vu::normalize(c, s + 0x60, s + 0x80);
    c.lq(V0, s + 0x80);
    c.lwc1(1, REVERSE_LIMIT);
    c.sq(V0, s + 0x10);
    c.sq(V0, s + 0x50);
    vu::dot(c, s + 0x40, s + 0x10, V1, 0);
    if lt(c, 1, 0) {
        c.g[V0][0] = 0;
        return Ok(());
    }

    // The flip: the pivot bone scaled by the character's scale, relative to
    // the origin, doubled; rotated by the orientation and added to the
    // position.
    c.cycles += 71;
    let character = Addr(rider + CHARACTER).lw(c);
    c.g[V1][0] = character;
    let ch = character as u32;
    c.g[A0][0] = c.g[S1][0];
    c.g[V0][0] = Addr(rider + PIVOT_BONE).lw(c);
    c.g[S0][0] = sx(rider + POS);
    c.g[A1][0] = Addr(ch + 0x24).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.lwc1(1, ch + 0x14c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let bone = c.g[V0][0] as u32;
    c.lwc1(4, ch + 0x140);
    c.lwc1(3, bone + 0xc);
    c.lwc1(0, bone);
    c.mul_s(3, 3, 1);
    c.lwc1(5, ch + 0x144);
    c.mul_s(0, 0, 4);
    c.lwc1(2, bone + 4);
    c.lwc1(4, ch + 0x148);
    c.lwc1(1, bone + 8);
    c.mul_s(2, 2, 5);
    c.g[V0][0] = 0x0050_0000;
    c.swc1(0, s + 0x60);
    c.mul_s(1, 1, 4);
    c.swc1(3, s + 0x6c);
    c.swc1(2, s + 0x64);
    c.g[V0][0] = sx(ORIGIN);
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 0);
    c.swc1(1, s + 0x68);
    vu::sub(c, s + 0x60, ORIGIN, s + 0x70);
    vu::copy(c, s + 0x70, s + 0x50, V0);
    c.swc1(6, s + 0x58);
    vu::scale(c, 0, s + 0x50, s + 0x80, V1);
    vu::copy(c, s + 0x80, s + 0x70, V0);
    // Rotate by the orientation quaternion (VU0 macro sequence).
    {
        use ssx3_ee_rt::vu0::{Src, W};
        c.lqc2(4, rider + ROT);
        c.lqc2(5, s + 0x70);
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, s + 0x80);
    }
    vu::copy(c, s + 0x80, s + 0x60, V0);
    vu::add(c, rider + POS, s + 0x60, rider + POS);
    BUILD_WORLD_MATRIX.call(c, 0x0011_5128)?;
    c.g[A0][0] = c.g[S1][0];
    TOGGLE_STANCE.call(c, 0x0011_5130)?;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(PREDICTOR)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(GP_PRIME_TIME));
    c.g[A2][0] = c.g[S2][0];
    AIR_PREDICTOR_PRIME.call(c, 0x0011_5144)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// ssx_CdTrayCallback (0x31ad20)

mod cd_tray {
    /// gp+0xde4: a tray request is pending (close the tray first).
    pub const GP_TRAY_PENDING: i32 = 0xde4;
    /// gp+0xde0: the tray was opened (disc changed).
    pub const GP_TRAY_OPENED: i32 = 0xde0;
    /// The CD status bits (0x519c40): bits 1|2 = the disc must be re-mounted.
    pub const CD_STATUS: u32 = 0x0052_0000 - 0x63c0;
    pub const REMOUNT: u64 = 0x6;
    /// `sceCdTrayReq(mode, &traycnt)` (mode 2: check).
    pub const SCE_CD_TRAY_REQ: u32 = 0x0040_2618;
    pub const TRAY_CHECK: u64 = 2;
    /// `EAFile_CdMountDisc()`.
    pub const EAFILE_CD_MOUNT_DISC: u32 = 0x003e_3d78;
}

/// `int ssx_CdTrayCallback(void)` (0x31ad20): the CD tray poll: answer a
/// pending tray request (`sceCdTrayReq(2, &cnt)`), and when the status asks
/// for a re-mount, check the tray again (a non-zero count marks the tray as
/// opened), clear the re-mount bits and mount the disc. Returns 0.
pub fn ssx_cd_tray_callback(c: &mut Cpu) {
    use cd_tray::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = gp_at(c, GP_TRAY_PENDING).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = 0x0052_0000;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = Addr(CD_STATUS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            gp_at(c, GP_TRAY_PENDING).sw(c, 0);
            c.g[A0][0] = TRAY_CHECK;
            let s = sp(c);
            Addr(s).sw(c, 0);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, SCE_CD_TRAY_REQ, 0x0031_ad54)?;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0] & REMOUNT;
        c.g[V0][0] = REMOUNT;
        let remount = c.g[V1][0] == REMOUNT;
        frame.restore(c, S0, 0x30);
        if remount {
            c.cycles += 7;
            let s = sp(c);
            Addr(s + 4).sw(c, 0);
            c.g[A0][0] = TRAY_CHECK;
            c.g[A1][0] = c.g[SP][0] | 4;
            call_ptr(c, SCE_CD_TRAY_REQ, 0x0031_ad74)?;
            let s = sp(c);
            c.g[V1][0] = Addr(s + 4).lw(c);
            c.g[V0][0] = 1;
            if c.g[V1][0] != 0 {
                c.cycles += 1;
                gp_at(c, GP_TRAY_OPENED).sw(c, 1);
            }
            c.cycles += 6;
            c.g[V1][0] = 0xffff_ffff_ffff_fff9;
            c.g[V0][0] = Addr(CD_STATUS).lw(c) & c.g[V1][0];
            sw_reg(c, CD_STATUS, V0);
            call_ptr(c, EAFILE_CD_MOUNT_DISC, 0x0031_ad98)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptMovie_TogglePause (0x283af8)

mod script_movie {
    /// ScriptMovie: the player, playing flag, pause request.
    pub const PLAYER: u32 = 0x0;
    pub const PLAYING: u32 = 0x4;
    pub const PAUSE_REQ: u32 = 0x8;
    /// VidEnginePlayer: paused flag.
    pub const PLAYER_PAUSED: u32 = 0x8;
    /// `VidEnginePlayer_Resume`, `_Pause`, `_Render(player, paused)`.
    pub const RESUME: u32 = 0x0025_3a40;
    pub const PAUSE: u32 = 0x0025_39e0;
    pub const RENDER: u32 = 0x0025_3890;
}

/// `void ScriptMovie_TogglePause(ScriptMovie *self)` (0x283af8): for a
/// playing movie, apply the pause request: resume a paused player when one is
/// pending, else pause a running one; then render the player with the
/// request's opposite and clear the request.
pub fn script_movie_toggle_pause(c: &mut Cpu) {
    use script_movie::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = r32(c, S0);
        c.g[A0][0] = Addr(movie + PLAYER).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(movie + PLAYING).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                let request = Addr(movie + PAUSE_REQ).lw(c) != 0;
                let player = r32(c, A0);
                c.g[V0][0] = Addr(player + PLAYER_PAUSED).lw(c);
                c.cycles += 2;
                let reload = if request {
                    if c.g[V0][0] == 0 {
                        false
                    } else {
                        c.cycles += 4;
                        call_ptr(c, RESUME, 0x0028_3b3c)?;
                        true
                    }
                } else {
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        call_ptr(c, PAUSE, 0x0028_3b54)?;
                    }
                    true
                };
                let movie = r32(c, S0);
                c.g[A1][0] = Addr(movie + PAUSE_REQ).lw(c);
                if reload {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(movie + PLAYER).lw(c);
                }
                c.cycles += 4;
                c.g[A1][0] ^= 1;
                call_ptr(c, RENDER, 0x0028_3b64)?;
                Addr(r32(c, S0) + PAUSE_REQ).sw(c, 0);
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_GetWeather08 (0x2ee448)

mod painter {
    /// gp+0xa70: weather effects disabled.
    pub const GP_DISABLED: i32 = 0xa70;
    /// The world painter's per-view slots (0xf0 bytes each) at 0x4fa370.
    pub const SLOTS: u32 = 0x0050_0000 - 0x5c90;
    pub const SLOT_SIZE: u64 = 0xf0;
    /// Slot: the weather object handle (`**obj`), the fade (1 = gone).
    pub const SLOT_OBJ: u32 = 0x20;
    pub const SLOT_FADE: u32 = 0x24;
    /// The object's virtual `float *GetWeather08()` (slot +0x158/+0x15c).
    pub const VT_GET: u32 = 0x158;
}

/// `float WorldPainter_GetWeather08(int view)` (0x2ee448): the weather
/// object's value 0x08 for a view (its virtual getter), scaled by
/// `1 - fade`; 0 when weather is disabled.
pub fn world_painter_get_weather08(c: &mut Cpu) {
    use painter::*;
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = gp_at(c, GP_DISABLED).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.mtc1(0, 0);
        } else {
            c.cycles += 19;
            c.g[S0][0] = SLOT_SIZE;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, S0, A0, S0);
            c.g[V0][0] = sx(SLOTS);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(SLOTS));
            let slot = r32(c, S0);
            c.g[V1][0] = Addr(slot + SLOT_OBJ).lw(c);
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
            let obj = r32(c, A2);
            c.g[A1][0] = Addr(obj + 4).lw(c);
            let vt = r32(c, A1);
            c.g[A0][0] = Addr(vt + VT_GET).lh(c);
            c.g[V0][0] = Addr(vt + VT_GET + 4).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x002e_e498)?;
            let slot = r32(c, S0);
            c.lwc1(2, slot + SLOT_FADE);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.lwc1(1, r32(c, V0));
            c.sub_s(0, 0, 2);
            c.mul_s(0, 1, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan3d_Render (0x344138)

mod crowd3d {
    /// The five crowd groups (0x1f4 bytes each).
    pub const GROUP_SIZE: u32 = 0x1f4;
    pub const GROUP_COUNT: u64 = 5;
    /// Group: its model id (-1: unused).
    pub const GROUP_MODEL: u32 = 0x8;

    use super::{ee, Callee};
    pub const GROUP_RENDER: Callee = Callee { addr: 0x0034_4e18, f: ee::s11::f_00344e18 };
}

/// `void CrowdMan3d_Render(CrowdGroup groups[5])` (0x344138): render every
/// crowd group in use (`CrowdMan3d_GroupRender`).
pub fn crowd_man3d_render(c: &mut Cpu) {
    use crowd3d::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S2][0] = u64::MAX;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(GROUP_MODEL));
        c.g[S3][0] = GROUP_COUNT - 1;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            if c.g[V0][0] != c.g[S2][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                GROUP_RENDER.call(c, 0x0034_417c)?;
            }
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(GROUP_SIZE));
            c.cycles += 3;
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(GROUP_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_4168);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateLoopSfx (0x29d610)

mod loop_sfx {
    /// SSXAudioSystem: a looping effect is playing, and its SND handle.
    pub const LOOP_ACTIVE: u32 = 0x5fc4;
    pub const LOOP_HANDLE: u32 = 0x5fc8;
    /// `SND_Lock`, `SND_IsHandleDone(handle)`, `SND_Unlock`.
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_IS_HANDLE_DONE: u32 = 0x003b_8ab8;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    use super::{ee, Callee};
    pub const STOP_LOOP_SFX: Callee = Callee { addr: 0x0029_d678, f: ee::s08::f_0029d678 };
    pub const START_LOOP_SFX: Callee = Callee { addr: 0x0029_d370, f: ee::s08::f_0029d370 };
}

/// `void SSXAudioSystem_UpdateLoopSfx(SSXAudioSystem *self)` (0x29d610):
/// while a looping effect plays, check its SND handle (under the SND lock);
/// when the sample has ended, stop it (fade 0) and start the next one.
pub fn ssx_audio_system_update_loop_sfx(c: &mut Cpu) {
    use loop_sfx::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1) + LOOP_ACTIVE).lw(c);
        frame.restore(c, S0, 0x20);
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            call_ptr(c, SND_LOCK, 0x0029_d638)?;
            c.g[A0][0] = Addr(r32(c, S1) + LOOP_HANDLE).lw(c);
            call_ptr(c, SND_IS_HANDLE_DONE, 0x0029_d640)?;
            c.g[S0][0] = (c.g[V0][0] > 0) as u64;
            call_ptr(c, SND_UNLOCK, 0x0029_d648)?;
            let done = c.g[S0][0] != 0;
            frame.restore(c, S0, 0x20);
            if done {
                c.cycles += 6;
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S1][0];
                STOP_LOOP_SFX.call(c, 0x0029_d65c)?;
                c.g[A0][0] = c.g[S1][0];
                START_LOOP_SFX.call(c, 0x0029_d664)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Collide_MeshLeafTriangles (0x32c0f8)

mod leaf {
    /// Collider (`self`, s2): skip back-facing triangles, the sweep direction,
    /// the vtable.
    pub const USE_NORMAL: u32 = 0x8;
    pub const DIR: u32 = 0x10;
    pub const VTABLE: u32 = 0x50;
    /// Virtual `bool TestTri(p0, p1, p2, nrm, &pos, &hitNrm, &t)` (+0x20/+0x24),
    /// its swept form (+0x28/+0x2c), `bool TestBox(lo, hi)` (+0x48/+0x4c).
    pub const VT_TRI: u32 = 0x20;
    pub const VT_TRI_SWEPT: u32 = 0x28;
    pub const VT_BOX: u32 = 0x48;
    /// Leaf mesh (s3): triangle count (u16), indices (3 bytes per triangle),
    /// boxes (0x18 each: lo +0, hi +0xc, ten triangles per box), vertices,
    /// face normals (16 bytes each).
    pub const NUM_TRIS: u32 = 0x0;
    pub const INDICES: u32 = 0x4;
    pub const BOXES: u32 = 0x8;
    pub const VERTS: u32 = 0xc;
    pub const NORMALS: u32 = 0x10;
    pub const BOX_SIZE: u64 = 0x18;
    pub const TRIS_PER_BOX: u64 = 10;
    /// Hit records (0x80 bytes each).
    pub const HIT_SIZE: u32 = 0x80;
    /// The zero vector (0x4ff120).
    pub const ZERO_VEC: u32 = 0x0050_0000 - 0xee0;
    /// The back-face limits (gp-0x2d88: small meshes, gp-0x2d84: boxed).
    pub const GP_FACING_SMALL: u32 = 0x2d88;
    pub const GP_FACING_BOXED: u32 = 0x2d84;
    /// Stack: the swept flag, the output, the hit count, the box count; the
    /// test's position (sp), normal (sp+0x10) and fraction (sp+0x20).
    pub const SP_SWEPT: u32 = 0x24;
    pub const SP_OUT: u32 = 0x28;
    pub const SP_HITS: u32 = 0x2c;
    pub const SP_BOXES: u32 = 0x30;
}

/// The back-face test of triangle s1 (`useNormal && nrm · dir > limit`):
/// true when it is skipped. Leaves t0 = the normal's address.
fn leaf_faces_away(c: &mut Cpu, limit: u32) -> bool {
    use leaf::*;
    c.cycles += 5;
    let mesh = r32(c, S3);
    let this = r32(c, S2);
    c.g[V0][0] = Addr(mesh + NORMALS).lw(c);
    c.g[V1][0] = sx(r32(c, S1) << 4);
    c.g[A0][0] = Addr(this + USE_NORMAL).lw(c);
    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    if c.g[A0][0] == 0 {
        return false;
    }
    c.cycles += 14;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(limit));
    let normal = r32(c, T0);
    vu::dot(c, normal, this + DIR, V0, 1);
    lt(c, 0, 1)
}

/// Test triangle s1 (index offset s5) with the collider's virtual triangle
/// test (swept or not); v0 = hit.
fn leaf_test_tri(c: &mut Cpu, ret_swept: u32, ret_plain: u32) -> call::Flow {
    use leaf::*;
    c.cycles += 14;
    let mesh = r32(c, S3);
    c.g[V0][0] = Addr(mesh + INDICES).lw(c);
    c.g[A2][0] = Addr(mesh + VERTS).lw(c);
    c.g[V0][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
    let idx = r32(c, V0);
    c.g[V1][0] = Addr(idx + 2).lbu(c);
    c.g[A0][0] = Addr(idx).lbu(c);
    c.g[A1][0] = Addr(idx + 1).lbu(c);
    c.g[V1][0] = sx(r32(c, V1) << 4);
    c.g[A3][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    c.g[A0][0] = sx(r32(c, A0) << 4);
    c.g[A1][0] = sx(r32(c, A1) << 4);
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    let s = sp(c);
    c.g[V1][0] = Addr(s + SP_SWEPT).lw(c);
    let swept = c.g[V1][0] != 0;
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
    let (slot, ret) = if swept {
        c.cycles += 11;
        (VT_TRI_SWEPT, ret_swept)
    } else {
        c.cycles += 9;
        (VT_TRI, ret_plain)
    };
    let this = r32(c, S2);
    c.g[V1][0] = Addr(this + VTABLE).lw(c);
    let vt = r32(c, V1);
    c.g[A1][0] = c.g[V0][0];
    c.g[T1][0] = c.g[SP][0];
    c.g[T2][0] = sx(s + 0x10);
    c.g[A0][0] = Addr(vt + slot).lh(c);
    c.g[T3][0] = sx(s + 0x20);
    c.g[V0][0] = Addr(vt + slot + 4).lw(c);
    let f = r32(c, V0);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// Count a hit in the stack's hit counter (a0 = the new count).
fn leaf_count_hit(c: &mut Cpu) {
    use leaf::*;
    let s = sp(c);
    c.g[A0][0] = Addr(s + SP_HITS).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    sw_reg(c, s + SP_HITS, A0);
}

/// `int Collide_MeshLeafTriangles(Collider *self, LeafMesh *mesh, int swept,
/// TriHit *out)` (0x32c0f8): test the leaf's triangles against the collider
/// (its virtual triangle test, swept or not), skipping back-facing ones when
/// asked, and write a hit record (type 2, position, normal, fraction,
/// triangle index) per hit. Meshes of 11 triangles or more are tested in
/// boxes of ten, each box first against the collider's virtual box test.
/// Returns the number of hits.
pub fn collide_mesh_leaf_triangles(c: &mut Cpu) {
    use leaf::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0xc0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        let mesh = r32(c, S3);
        c.g[V0][0] = Addr(mesh + NUM_TRIS).lhu(c);
        let s = sp(c);
        sw_reg(c, s + SP_SWEPT, A2);
        sw_reg(c, s + SP_OUT, A3);
        c.g[V0][0] = (c.g[V0][0] < 11) as u64;
        Addr(s + SP_HITS).sw(c, 0);
        if c.g[V0][0] != 0 {
            leaf_small(c)?;
        } else {
            leaf_boxed(c)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Fewer than 11 triangles: test each (v0 = the hit count at the end).
fn leaf_small(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.g[S1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = 0x0050_0000;
    c.g[S0][0] = c.g[A3][0];
    c.g[S6][0] = sx(ZERO_VEC);
    c.g[S4][0] = u64::MAX;
    c.g[S5][0] = 0;
    loop {
        if leaf_faces_away(c, GP_FACING_SMALL) {
            c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
        } else {
            leaf_test_tri(c, 0x0032_c210, 0x0032_c23c)?;
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            if hit {
                c.cycles += 27;
                let out = r32(c, S0);
                let none = c.g[S4][0];
                Addr(out + 0x54).sw(c, 0);
                sw_reg(c, out + 0x44, V0);
                leaf_count_hit(c);
                Addr(out + 0x60).sw(c, none);
                Addr(out + 0x50).sw(c, 0);
                Addr(out + 0x58).sw(c, 0);
                Addr(out + 0x5c).sw(c, none);
                Addr(out + 0x64).sw(c, none);
                Addr(out + 0x4c).sw(c, none);
                let s = sp(c);
                let zero = r32(c, S6);
                c.lq(V0, zero);
                c.lq(V1, s);
                c.lq(A0, s + 0x10);
                c.sq(V0, out + 0x20);
                c.lwc1(0, s + 0x20);
                c.lq(V0, zero);
                c.sq(V1, out);
                c.sq(V0, out + 0x30);
                c.sq(A0, out + 0x10);
                c.swc1(0, out + 0x40);
                Addr(out + 0x68).sw(c, 0);
                Addr(out + 0x6c).sw(c, 0);
                Addr(out + 0x70).sw(c, 0);
                sw_reg(c, out + 0x60, S1);
                Addr(out + 0x54).sw(c, 0);
                c.g[S0][0] = sx(out.wrapping_add(HIT_SIZE));
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
        }
        c.cycles += 4;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
        c.g[V0][0] = more as u64;
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c168);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
    Ok(())
}

/// 11 triangles or more: per box of ten that passes the box test, test its
/// triangles (v0 = the hit count at the end).
fn leaf_boxed(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.g[V1][0] = TRIS_PER_BOX;
    c.cycles += 6;
    c.g[S0][0] = 0;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(9));
    c.div(0, V0, V1);
    c.g[A0][0] = c.lo[0];
    let s = sp(c);
    sw_reg(c, s + SP_BOXES, A0);
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = BOX_SIZE;
        loop {
            // s0: the box index (fp: the next one).
            c.cycles += 12;
            let this = r32(c, S2);
            c.g[A2][0] = Addr(this + VTABLE).lw(c);
            c.mult(0, V1, S0, V1);
            c.g[A1][0] = Addr(r32(c, S3) + BOXES).lw(c);
            let vt = r32(c, A2);
            c.g[A0][0] = Addr(vt + VT_BOX).lh(c);
            c.g[FP][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = Addr(vt + VT_BOX + 4).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
            let f = r32(c, V0);
            c.g[A2][0] = sx(r32(c, A1).wrapping_add(0xc));
            call_ptr(c, f, 0x0032_c320)?;
            let inside = c.g[V0][0] != 0;
            c.g[V0][0] = TRIS_PER_BOX;
            if inside {
                leaf_box_tris(c)?;
            }
            c.cycles += 5;
            let s = sp(c);
            c.g[V1][0] = Addr(s + SP_BOXES).lw(c);
            c.g[S0][0] = c.g[FP][0];
            let more = (c.g[S0][0] as i64) < (c.g[V1][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = BOX_SIZE;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_c2f8);
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
    Ok(())
}

/// The triangles of box s0 (from s0*10 to min(fp*10, count)).
fn leaf_box_tris(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 8;
    c.g[S4][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.mult(0, A0, FP, V0);
    c.mult(0, S1, S0, V0);
    if (c.g[A0][0] as i64) < (c.g[S4][0] as i64) {
        c.g[V0][0] = 1;
        c.g[S4][0] = c.g[A0][0];
    } else {
        c.g[V0][0] = 0;
    }
    let any = (c.g[S1][0] as i64) < (c.g[S4][0] as i64);
    c.g[V1][0] = any as u64;
    c.g[V0][0] = 0x0050_0000;
    if !any {
        return Ok(());
    }
    c.cycles += 8;
    c.g[A0][0] = sx(r32(c, S1) << 1);
    c.g[S7][0] = sx(ZERO_VEC);
    c.g[S5][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
    let s = sp(c);
    c.g[V0][0] = Addr(s + SP_HITS).lw(c);
    c.g[S6][0] = u64::MAX;
    c.g[A0][0] = Addr(s + SP_OUT).lw(c);
    c.g[V1][0] = sx(r32(c, V0) << 7);
    c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    loop {
        if leaf_faces_away(c, GP_FACING_BOXED) {
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        } else {
            leaf_test_tri(c, 0x0032_c410, 0x0032_c43c)?;
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            if hit {
                c.cycles += 27;
                let out = r32(c, S0);
                let none = c.g[S6][0];
                let zero = r32(c, S7);
                Addr(out + 0x54).sw(c, 0);
                sw_reg(c, out + 0x44, V0);
                leaf_count_hit(c);
                Addr(out + 0x60).sw(c, none);
                Addr(out + 0x50).sw(c, 0);
                Addr(out + 0x58).sw(c, 0);
                Addr(out + 0x5c).sw(c, none);
                Addr(out + 0x64).sw(c, none);
                Addr(out + 0x4c).sw(c, none);
                c.lq(V0, zero);
                c.sq(V0, out + 0x20);
                c.lq(V1, zero);
                Addr(out + 0x68).sw(c, 0);
                c.sq(V1, out + 0x30);
                Addr(out + 0x6c).sw(c, 0);
                Addr(out + 0x70).sw(c, 0);
                let s = sp(c);
                c.lq(V0, s + 0x10);
                c.lq(V1, s);
                c.sq(V0, out + 0x10);
                c.sq(V1, out);
                c.lwc1(0, s + 0x20);
                sw_reg(c, out + 0x60, S1);
                c.swc1(0, out + 0x40);
                Addr(out + 0x54).sw(c, 0);
                c.g[S0][0] = sx(out.wrapping_add(HIT_SIZE));
            }
            c.cycles += 1;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        }
        c.cycles += 3;
        let more = (c.g[S1][0] as i64) < (c.g[S4][0] as i64);
        c.g[V0][0] = more as u64;
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c368);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, put_vec, Rng, DATA, GAME_GP};

    /// Write a gp-relative global.
    fn put_gp(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// Every combination of the manager's flags except a bake request (the
    /// bake branch, `Avalanche_BakeAll`, reads history files from disc and is
    /// not reached here); no histories loaded
    /// (gp+0x9c0 = 0) and empty active slots, so the real callees take their
    /// short paths.
    #[test]
    fn vfx_avalanche_update_matches() {
        use avg::*;
        check(ssx3_ee::s09::f_002d8948__generated, vfx_avalanche_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2d8948, 0);
            let flag = |r: &mut Rng, k: u32| match (seed / 3u32.pow(k)) % 3 {
                0 => 0,
                1 => 1 + r.next_u32() % 0x7fff,
                _ => 0x8000_0000 | r.next_u32(),
            };
            let enabled = if seed % 7 == 0 { 0 } else { 1 };
            // No bake request: Avalanche_BakeAll always exports and reloads
            // the history files (disc access, unbounded on test data).
            let _ = flag(&mut r, 0);
            let bake = 0;
            let start = flag(&mut r, 1);
            let data = flag(&mut r, 2);
            let update = flag(&mut r, 3);
            put_gp(c, ENABLED, enabled);
            put_gp(c, BAKE_PENDING, bake);
            put_gp(c, START_PENDING, start);
            put_gp(c, UPDATE_PENDING, update);
            put_gp(c, HAVE_DATA, data);
            put_gp(c, BAKE_CLEAR, r.next_u32());
            put_gp(c, START_ID, DATA + (r.next_u32() & 0xfff0));
            put_gp(c, 0x9c0, 0);
            // The BELibrary singleton (gp-0x1f04), so nothing builds one.
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.pc = 0x002d_8948;
        });
    }

    /// Four track slots with random loaded flags and class masks (often
    /// matching the id's nibbles), random track pointers, ids with and
    /// without low bits.
    #[test]
    fn path_find_track_matches() {
        use path::*;
        check(ssx3_ee::s14::f_003d5290__generated, path_find_track, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d5290, 0);
            let kind = (1 + r.next_u32() % 15) << 28;
            let group = (r.next_u32() % 16) << 24;
            let low = match seed % 4 {
                0 => 0,
                1 => 1 << (r.next_u32() % 24),
                _ => r.next_u32() & 0xff_ffff,
            };
            let mut id = (kind | group | low) as u64;
            if seed % 5 == 0 {
                id |= (r.next_u32() as u64) << 32;
            }
            c.g[A0][0] = id;
            for i in 0..SLOT_COUNT as u32 {
                let slot = SLOTS + i * SLOT_SIZE;
                c.bus.write32(slot + LOADED, (r.next_u32() % 3 != 0) as u32);
                let mask = match r.next_u32() % 4 {
                    0 => r.next_u32(),
                    1 => kind,
                    _ => kind | group | (r.next_u32() & 0x00ff_ffff),
                };
                c.bus.write32(slot + MASK, mask);
                for k in 0..24 {
                    c.bus.write32(slot + TRACKS + 4 * k, r.next_u32());
                }
            }
            c.pc = 0x003d_5290;
        });
    }

    /// A rider in the air over random geometry: predictor state 1, 3 or
    /// neither; the rider's own plane (surface 0x12 or a flagged surface) or
    /// the predictor's; random up/velocity/axes so every limit test fails on
    /// some seeds and some seeds flip (real BuildWorldMatrix, ToggleStance,
    /// AirPredictor_Prime).
    fn try_air_reverse_setup(c: &mut Cpu, seed: u32) {
        use air_reverse::*;
        let mut r = machine(c, seed, 0x114db8, 0);
        crate::difftest::elf_image(c);
        c.pc = 0x0011_4db8;
        let rider = DATA;
        let pred = DATA + 0x1000;
        let character = DATA + 0x2000;
        let bones = DATA + 0x3000;
        let world = DATA + 0x4000;
        let game = DATA + 0x4100;
        let table = DATA + 0x5000;
        let owner = DATA + 0x2_0000 + (r.next_u32() & 0xfff0);
        for off in (0..0x900).step_by(4) {
            c.bus.write32(rider + off, r.f32(1.5));
        }
        for off in (0..0x100).step_by(4) {
            c.bus.write32(pred + off, r.f32(1.5));
            c.bus.write32(bones + off, r.f32(3.0));
        }
        for off in (0x140..0x150).step_by(4) {
            c.bus.write32(character + off, r.f32(2.0));
        }
        c.bus.write32(character + 0x24, bones);
        c.bus.write32(rider + PIVOT_BONE, r.next_u32() % 8);
        c.bus.write32(rider + CHARACTER, character);
        c.bus.write32(rider + PREDICTOR, pred);
        c.bus.write32(rider + 0x77c, owner);
        c.bus.write32(rider + 0x784, owner);
        let state = match seed % 5 {
            0 | 3 => 1,
            1 | 4 => 3,
            _ => r.next_u32() % 3 * 2,
        };
        c.bus.write32(pred + PRED_STATE, state);
        let surface = if seed / 5 % 3 == 0 { 0x12 } else { r.next_u32() % 8 };
        c.bus.write32(pred + PRED_SURFACE, surface);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, game);
        c.bus.write32(game + 0x44, table);
        for k in 0..0x13 {
            c.bus.write32(table + k * SURFACE_SIZE + 0x44, r.next_u32() % 2);
        }
        c.g[A0][0] = rider as u64;
    }

    #[test]
    fn rider_try_air_reverse_matches() {
        check(ssx3_ee::s00::f_00114db8__generated, rider_try_air_reverse, 300, &try_air_reverse_setup);
        // Some seeds really flip (the whole body runs).
        let mut flipped = 0;
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            try_air_reverse_setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ssx3_ee::s00::f_00114db8__generated);
            flipped += (c.g[V0][0] == 1) as u32;
        }
        assert!(flipped >= 5, "flipped {flipped}");
    }

    /// The tray poll with and without a pending request, and status words
    /// without the re-mount bits (both set means `EAFile_CdMountDisc`, which
    /// mounts a disc and cannot be bounded in a test: that branch is not
    /// reached here). The real `sceCdTrayReq` takes its "busy" exit (the
    /// fake syscalls never grant the semaphore).
    #[test]
    fn ssx_cd_tray_callback_matches() {
        use cd_tray::*;
        check(ssx3_ee::s10::f_0031ad20__generated, ssx_cd_tray_callback, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x31ad20, 0);
            c.pc = 0x0031_ad20;
            put_gp(c, GP_TRAY_PENDING, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            put_gp(c, GP_TRAY_OPENED, r.next_u32());
            let status = r.next_u32();
            let status = match seed / 2 % 3 {
                0 => status & !6,
                1 => status & !4 | 2,
                _ => status & !2 | 4,
            };
            c.bus.write32(CD_STATUS, status);
        });
    }

    /// Movies with and without a player, playing or not, every combination
    /// of the pause request and the player's paused flag. The player is
    /// mostly not set up (its +0x38 is 0, so the real Resume/Pause/Render
    /// return at once), sometimes with only its stream (+0x38, no +0x3c).
    #[test]
    fn script_movie_toggle_pause_matches() {
        use script_movie::*;
        check(ssx3_ee::s07::f_00283af8__generated, script_movie_toggle_pause, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x283af8, 0);
            c.pc = 0x0028_3af8;
            let movie = DATA;
            let player = DATA + 0x100;
            let flag = |r: &mut Rng, k: u32| if (seed >> k) & 1 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(movie + PLAYER, if seed % 9 == 8 { 0 } else { player });
            c.bus.write32(movie + PLAYING, if seed % 7 == 6 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(movie + PAUSE_REQ, flag(&mut r, 0));
            c.bus.write32(player + PLAYER_PAUSED, flag(&mut r, 1));
            c.bus.write32(player + 0x38, flag(&mut r, 2) & (seed / 4 % 2).wrapping_neg());
            c.bus.write32(player + 0x3c, 0);
            c.g[A0][0] = movie as u64;
        });
    }

    /// Disabled (0) or a view's slot whose weather object's getter is a fake
    /// (returning a pointer to a random float; some seeds unwind there).
    #[test]
    fn world_painter_get_weather08_matches() {
        use painter::*;
        crate::difftest::fakes::covered(
            ssx3_ee::s09::f_002ee448__generated,
            world_painter_get_weather08,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ee448, 1);
                c.pc = 0x002e_e448;
                put_gp(c, GP_DISABLED, if seed % 5 == 0 { r.next_u32() | 1 } else { 0 });
                let view = r.next_u32() % 8;
                let slot = SLOTS + view * SLOT_SIZE as u32;
                let handle = DATA + 0x100;
                let obj = DATA + 0x200;
                let vt = DATA + 0x400;
                let value = DATA + 0x800;
                c.bus.write32(slot + SLOT_OBJ, handle);
                c.bus.write32(slot + SLOT_FADE, r.f32(1.0));
                c.bus.write32(handle, obj);
                c.bus.write32(obj + 4, vt);
                c.bus.write16(vt + VT_GET, ((r.next_u32() as i16) & !3) as u16);
                c.bus.write32(vt + VT_GET + 4, crate::difftest::fakes::FAKE);
                c.bus.write32(value, r.f32(100.0));
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, value);
                c.g[A0][0] = view as u64;
            },
        );
    }

    /// Five groups, each unused (-1) or in use with an empty instance list
    /// (every slot -1, so the real `CrowdMan3d_GroupRender` only walks it).
    #[test]
    fn crowd_man3d_render_matches() {
        use crowd3d::*;
        check(ssx3_ee::s11::f_00344138__generated, crowd_man3d_render, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x344138, 0);
            c.pc = 0x0034_4138;
            let groups = DATA;
            for g in 0..GROUP_COUNT as u32 {
                let grp = groups + g * GROUP_SIZE;
                let used = (seed >> g) & 1 == 1;
                c.bus.write32(grp + GROUP_MODEL, if used { r.next_u32() >> 1 } else { u32::MAX });
                for k in 0..32 {
                    c.bus.write32(grp + 0xc + 8 * k, u32::MAX);
                }
            }
            c.g[A0][0] = groups as u64;
        });
    }

    /// A looping effect off, or on with a handle that is live in the SND
    /// channel table or negative (done). SND_Lock/SND_Unlock go
    /// through the lock hooks, which are fakes (some seeds unwind there); the
    /// next effect's StartLoopSfx returns at once (no loop list, +0x6c74 = 0).
    #[test]
    fn ssx_audio_system_update_loop_sfx_matches() {
        use crate::difftest::fakes::{covered, FAKE, LOG_RESULTS};
        use loop_sfx::*;
        /// SND state (0x50a8e8): channel count (+0x17e), channel table (+0x1f0).
        const SNDS: u32 = 0x0051_0000 - 0x5718;
        const LOCK_HOOK: u32 = 0x0045_0000 - 0x3b94;
        covered(ssx3_ee::s08::f_0029d610__generated, ssx_audio_system_update_loop_sfx, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x29d610, 2);
            c.pc = 0x0029_d610;
            for i in 0..2 {
                c.bus.write32(LOG_RESULTS + 4 * i, 0);
            }
            c.bus.write32(LOCK_HOOK, FAKE + 0x100);
            c.bus.write32(LOCK_HOOK + 4, FAKE + 0x110);
            let channels = DATA + 0x8000;
            c.bus.write16(SNDS + 0x17e, 8);
            c.bus.write32(SNDS + 0x1f0, channels);
            c.bus.write8(SNDS + 0x176, (r.next_u32() % 4) as u8);
            let sys = DATA;
            let handle = r.next_u32() & 0x7fff_ff00 | r.next_u32() % 8;
            // The handle's channel is live (a stale non-negative handle
            // would make StopLoopSfx stop it through a bank monitor).
            let ch = channels + 0x8c * (handle & 0xff);
            c.bus.write8(ch + 0x69, 1);
            c.bus.write32(ch, handle);
            c.bus.write32(sys + LOOP_ACTIVE, if seed % 6 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(
                sys + LOOP_HANDLE,
                if seed % 5 == 0 { 0x8000_0000 | r.next_u32() } else { handle },
            );
            c.bus.write32(sys + 0x6c74, 0);
            c.g[A0][0] = sys as u64;
        });
    }

    /// A collider and a leaf mesh of 0..25 triangles (small meshes and boxed
    /// ones, with a partial last box). The triangle and box tests are fakes
    /// (hits; some seeds unwind) or a real leaf returning 0 (misses), chosen
    /// per slot; back-face skipping on or off with random normals.
    #[test]
    fn collide_mesh_leaf_triangles_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use leaf::*;
        const MISS: u32 = 0x0014_06c0;
        covered(ssx3_ee::s10::f_0032c0f8__generated, collide_mesh_leaf_triangles, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x32c0f8, 30);
            c.pc = 0x0032_c0f8;
            let this = DATA;
            let vt = DATA + 0x100;
            let mesh = DATA + 0x200;
            let indices = DATA + 0x400;
            let boxes = DATA + 0x600;
            let verts = DATA + 0x800;
            let normals = DATA + 0xa00;
            let out = DATA + 0x1000;
            c.bus.write32(this + USE_NORMAL, (seed % 3 != 0) as u32);
            put_vec(c, this + DIR, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            c.bus.write32(this + VTABLE, vt);
            for (i, slot) in [VT_TRI, VT_TRI_SWEPT, VT_BOX].into_iter().enumerate() {
                c.bus.write16(vt + slot, ((r.next_u32() as i16) & !3) as u16);
                let f = if r.next_u32() % 4 == 0 { MISS } else { FAKE + 0x10 * i as u32 };
                c.bus.write32(vt + slot + 4, f);
            }
            let tris = match seed % 4 {
                0 => r.next_u32() % 11,
                1 => 11 + r.next_u32() % 15,
                2 => 20,
                _ => r.next_u32() % 26,
            };
            c.bus.write16(mesh + NUM_TRIS, tris as u16);
            c.bus.write32(mesh + INDICES, indices);
            c.bus.write32(mesh + BOXES, boxes);
            c.bus.write32(mesh + VERTS, verts);
            c.bus.write32(mesh + NORMALS, normals);
            for k in 0..3 * 25 {
                c.bus.write8(indices + k, (r.next_u32() % 16) as u8);
            }
            for k in 0..25 {
                put_vec(c, normals + 16 * k, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            }
            c.bus.write32(GAME_GP - GP_FACING_SMALL, r.f32(1.0));
            c.bus.write32(GAME_GP - GP_FACING_BOXED, r.f32(1.0));
            c.g[A0][0] = this as u64;
            c.g[A1][0] = mesh as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 };
            c.g[A3][0] = out as u64;
        });
    }
}
