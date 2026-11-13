//! Staging module oc328 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc328`.

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
    (0x0034_2c08, debounce_ctor_2, "Debounce_ctor_2"),
    (0x0031_d700, c_csi_cubic_spline_init, "cCSICubicSpline_Init"),
    (
        0x0037_0cf8,
        dynamic_emitter_free_data,
        "DynamicEmitter_FreeData",
    ),
    (
        0x0029_3dc0,
        ssx_audio_system_calc_edge_loop_group1,
        "SSXAudioSystem_CalcEdgeLoopGroup1",
    ),
    (0x0035_13d0, intersect_info_clear, "IntersectInfo_Clear"),
    (
        0x0015_0540,
        beb_agt_int_get_trick_points_2,
        "BEBAGTInt_GetTrickPoints2",
    ),
    (
        0x0015_0528,
        beb_agt_int_get_trick_points,
        "BEBAGTInt_GetTrickPoints",
    ),
    (
        0x0029_3b70,
        ssx_audio_system_calc_surface_loop_remote_group1,
        "SSXAudioSystem_CalcSurfaceLoopRemoteGroup1",
    ),
    (0x0040_f840, isnan, "isnan"),
    (0x003d_d5a0, ea_file_queue_close, "EAFile_QueueClose"),
];

// ---------------------------------------------------------------------------
// Debounce_ctor_2 (0x342c08)

/// The object's debounce time, stored as the raw bits of an `int` (+0x2c).
pub(crate) const DEBOUNCE_TIME_WORD: u32 = 0x2c;

/// The object's mode word (+0x30).
pub(crate) const DEBOUNCE_MODE_WORD: u32 = 0x30;

/// The object's linked move node (`self->node`, +0x18).
pub(crate) const DEBOUNCE_NODE: u32 = 0x18;

/// A node's flags word (+8).
pub(crate) const DEBOUNCE_NODE_FLAGS: u32 = 0x8;

/// The definition passed to the constructor: `time` (`float`, +4), `f08` (+8)
/// and `mode` (+0xc).
pub(crate) const DEBOUNCE_DEF_TIME: u32 = 0x4;

pub(crate) const DEBOUNCE_DEF_F08: u32 = 0x8;

pub(crate) const DEBOUNCE_DEF_MODE: u32 = 0xc;

/// `D_004A5B64` (gp+0x2a74), whose `+0x10` word is the tick-rate scale.
#[cfg(test)]
pub(crate) const DEBOUNCE_RATE_SCALE: u32 = 0x004a_5b64;

/// `void* Debounce_ctor_2(self, list, node, def, src)` (0x342c08): the
/// `ModObj` constructor with this object's vtable (`0x4906f0`), its debounce
/// time (`def->time * *(int*)(D_004A5B64 + 0x10)`, truncated) at +0x2c and
/// `def->f08` at +0x30, then takes over `src`. Finally the node flags follow
/// `def->mode`: 0 clears bits 0-3, 1 sets bit 0 and bit 2 (clearing 1), and
/// any other mode sets bit 2 (clearing 1) only when bits 0 and 1 are both set.
/// Returns `self`. Frame 0x40 (s0 = def, s1 = src, s2 = self).
pub fn debounce_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A3][0]; // def
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[A2][0]; // ctor node = our a2
        c.g[S1][0] = c.g[T0][0]; // our a4 (the source node) = ctor list
        frame.save_ra(c, 0);
        c.g[A2][0] = 0; // ctor type
        Callee {
            addr: 0x0035_5280,
            f: ee::s11::ModObj_ctor,
        }
        .call(c, 0x0034_2c34)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0x2a74)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x6f0));
        c.g[A1][0] = c.g[S1][0];
        Addr(r32(c, S2).wrapping_add(0xc)).sw(c, c.g[V1][0]); // self->vt
        c.g[A0][0] = c.g[S2][0];
        // self->+0x2c = (int)(def->time * (float)*(int*)(D_004A5B64 + 0x10)).
        c.lwc1(1, r32(c, V0).wrapping_add(0x10));
        c.cvt_s_w(1, 1);
        c.lwc1(0, r32(c, S0).wrapping_add(DEBOUNCE_DEF_TIME));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, S2).wrapping_add(DEBOUNCE_TIME_WORD));
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(DEBOUNCE_DEF_F08)).lw(c);
        Addr(r32(c, S2).wrapping_add(DEBOUNCE_MODE_WORD)).sw(c, c.g[V0][0]);
        Callee {
            addr: 0x0035_5f10,
            f: ee::s11::ModObj_TakeOverObject,
        }
        .call(c, 0x0034_2c70)?;

        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(DEBOUNCE_DEF_MODE)).lw(c);
        c.g[V0][0] = 1;
        // The `+1` tail block runs for every path but the plain `mode >= 2`
        // one (which leaves the loop directly).
        let mut tail = true;
        if c.g[V1][0] != 0 {
            // mode != 0
            c.cycles += 2;
            c.g[A1][0] = Addr(r32(c, S2).wrapping_add(DEBOUNCE_NODE)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                // mode != 1: touch the flags only when both low bits are set.
                c.cycles += 5;
                c.g[V1][0] = 3;
                c.g[A0][0] = Addr(r32(c, A1).wrapping_add(DEBOUNCE_NODE_FLAGS)).lw(c);
                c.g[V0][0] = c.g[A0][0] & 3;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = c.g[S2][0];
                    tail = false;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffd; // clear bit 1
                    c.g[V0][0] &= c.g[A0][0];
                    c.g[V0][0] |= 0x4;
                    Addr(r32(c, A1).wrapping_add(DEBOUNCE_NODE_FLAGS)).sw(c, c.g[V0][0]);
                }
            } else {
                // mode == 1: flags |= 1, then flags = (flags & ~2) | 4.
                c.cycles += 11;
                c.g[V1][0] = 0xffff_ffff_ffff_fffd;
                c.g[V0][0] = Addr(r32(c, A1).wrapping_add(DEBOUNCE_NODE_FLAGS)).lw(c);
                c.g[V0][0] |= 0x1;
                Addr(r32(c, A1).wrapping_add(DEBOUNCE_NODE_FLAGS)).sw(c, c.g[V0][0]);
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(DEBOUNCE_NODE)).lw(c);
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(DEBOUNCE_NODE_FLAGS)).lw(c);
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= 0x4;
                Addr(r32(c, A0).wrapping_add(DEBOUNCE_NODE_FLAGS)).sw(c, c.g[V0][0]);
            }
        } else {
            // mode == 0: clear bits 0-3.
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(DEBOUNCE_NODE)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_fff0;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(DEBOUNCE_NODE_FLAGS)).lw(c);
            c.g[V0][0] &= c.g[V1][0];
            Addr(r32(c, A0).wrapping_add(DEBOUNCE_NODE_FLAGS)).sw(c, c.g[V0][0]);
        }
        if tail {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
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
// cCSICubicSpline_Init (0x31d700)

/// `void cCSICubicSpline_Init(cCSICubicSpline* self)` (0x31d700): allocate the
/// spline's data, then clear bit 0 of its flags word (the low bits of the
/// first word hold the point count, bit 0 is this "initialised" flag).
/// Frame 0x20 (s0 = self).
pub fn c_csi_cubic_spline_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0031_d660,
            f: ee::s10::cCSICubicSpline_Alloc,
        }
        .call(c, 0x0031_d714)?;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.restore_ra(c, 0);
        c.g[V0][0] &= c.g[V1][0];
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DynamicEmitter_FreeData (0x370cf8)

/// Emitter fields: the element count (+0x170), the last index (+0x178), the
/// particle array (+0x1a0) and the state array (+0x1a4).
pub(crate) const EMITTER_FREE_COUNT: u32 = 0x170;

pub(crate) const EMITTER_FREE_LAST: u32 = 0x178;

pub(crate) const EMITTER_FREE_PARTICLES: u32 = 0x1a0;

pub(crate) const EMITTER_FREE_STATES: u32 = 0x1a4;

/// `void DynamicEmitter_FreeData(DynamicEmitter* self, int reset)` (0x370cf8):
/// free the particle and state arrays (`EA_MemFree`, through the function
/// table) and clear their slots; with `reset`, also clear the element count
/// and set the last index to -1. Frame 0x30 (s0 = self, s1 = reset).
pub fn dynamic_emitter_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(EMITTER_FREE_PARTICLES)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0037_0d20)?;
            Addr(r32(c, S0).wrapping_add(EMITTER_FREE_PARTICLES)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(EMITTER_FREE_STATES)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0037_0d38)?;
            Addr(r32(c, S0).wrapping_add(EMITTER_FREE_STATES)).sw(c, 0);
        }
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            Addr(r32(c, S0).wrapping_add(EMITTER_FREE_COUNT)).sw(c, 0);
            Addr(r32(c, S0).wrapping_add(EMITTER_FREE_LAST)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_CalcEdgeLoopGroup1 (0x293dc0)

/// Per-rider audio level slots (`self + 0x5990`).
pub(crate) const EDGE_LOOP_LEVELS: u32 = 0x5990;

/// The volume curve descriptor (`D_00482CB8`, 40 bytes).
pub(crate) const EDGE_LOOP_VOLUME_CURVE: u32 = 0x0048_2cb8;

/// The pitch curve descriptor (`D_00482CE0`, 40 bytes).
pub(crate) const EDGE_LOOP_PITCH_CURVE: u32 = 0x0048_2ce0;

/// `0.1f` (`gp-0x45b0`): below it the level is scaled by the speed.
pub(crate) const EDGE_LOOP_THRESHOLD: u32 = 0x45b0;

/// Rider audio fields: the level components (+0x750, +0x754), the pressure
/// (+0x758) and the speed (+0x75c); the physics object at +0x6c0.
pub(crate) const EDGE_LOOP_RIDER_SPEED: u32 = 0x75c;

pub(crate) const EDGE_LOOP_RIDER_PRESSURE: u32 = 0x758;

pub(crate) const EDGE_LOOP_RIDER_BODY: u32 = 0x6c0;

/// Copy the 8 bytes at `src` to `dst` with the unaligned load/store pair.
pub(crate) fn edge_loop_copy_dword(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.ldl(reg, src.wrapping_add(7));
    c.ldr(reg, src);
    c.sdl(reg, dst.wrapping_add(7));
    c.sdr(reg, dst);
}

/// `void SSXAudioSystem_CalcEdgeLoopGroup1(SSXAudioSystem* self, Rider* rider,
/// int* volume, int* pitch)` (0x293dc0): the board-edge loop of group 1. The
/// level is the rider's two level factors (+0x750 + +0x754), scaled by
/// `speed * 10` when the speed (+0x75c) is under 0.1. The edge index comes
/// from the rider's virtual (`+0x6c0`, slot 0x38); a jump of more than 0.1
/// from the previous level moves it a half (falling) or an eighth (rising)
/// of the way. The level is stored back and the volume curve is evaluated at
/// it (`*volume`), the pitch curve at the pressure (`*pitch`, times 4096).
/// Frame 0xc0 (s0 = self, s1 = volume, s2 = pitch, f20..f22 saved).
pub fn ssx_audio_system_calc_edge_loop_group1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = c.g[A1][0]; // rider
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0]; // audio system
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0]; // volume out
        frame.save_fpr(c, 21, 0xa8);
        c.g[S2][0] = c.g[A3][0]; // pitch out
        frame.save_fpr(c, 20, 0xa0);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 22, 0xb0);
        let rider = r32(c, V0);
        c.lwc1(21, r32(c, GP).wrapping_sub(EDGE_LOOP_THRESHOLD));
        c.lwc1(12, rider.wrapping_add(EDGE_LOOP_RIDER_SPEED));
        c.lwc1(1, rider.wrapping_add(0x754));
        c.lwc1(0, rider.wrapping_add(0x750));
        c.c_lt_s(12, 21);
        let quiet = c.fcc();
        c.add_s(20, 1, 0); // level = +0x754 + +0x750
        if quiet {
            c.cycles += 4;
            c.mul_s(1, 20, 12);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0); // 10.0
            c.mul_s(20, 1, 0);
        }

        // The edge index from the rider's virtual.
        c.cycles += 19;
        c.g[V1][0] = Addr(rider.wrapping_add(EDGE_LOOP_RIDER_BODY)).lw(c);
        c.g[A1][0] = sx(rider.wrapping_add(EDGE_LOOP_RIDER_BODY));
        c.lwc1(22, rider.wrapping_add(EDGE_LOOP_RIDER_PRESSURE));
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0029_3e3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(EDGE_LOOP_LEVELS));
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.lwc1(2, r32(c, V0)); // previous level
        c.sub_s(1, 2, 20);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let jump = c.fcc(); // 0.1 < |prev - level|
        c.g[V1][0] = 0x0048_0000;
        if jump {
            c.cycles += 3;
            c.c_lt_s(20, 2);
            if c.fcc() {
                // Falling: half of the difference.
                c.cycles += 5;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 0);
                c.mul_s(0, 1, 0);
                c.sub_s(20, 2, 0);
            } else {
                c.cycles += 4;
                c.c_lt_s(2, 20);
                c.g[V1][0] = 0x0048_0000;
                if c.fcc() {
                    // Rising: an eighth of the difference.
                    c.cycles += 5;
                    c.sub_s(0, 20, 2);
                    c.g[AT][0] = 0x3e00_0000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 0, 1);
                    c.add_s(20, 2, 0);
                }
            }
        }

        // Store the level, copy both curves to the stack, evaluate them.
        c.cycles += 69;
        c.g[V0][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[T0][0] = sx(r32(c, V1).wrapping_add(0x2cb8));
        let base = sp(c);
        for (i, reg) in [4usize, 6, 7, 4, 6].into_iter().enumerate() {
            let o = 8 * i as u32;
            edge_loop_copy_dword(c, reg, EDGE_LOOP_VOLUME_CURVE + o, base + o);
        }
        c.swc1(20, r32(c, V0)); // levels[idx] = level
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = sx(base.wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = sx(EDGE_LOOP_PITCH_CURVE);
        for (i, reg) in [2usize, 3, 6, 2, 3].into_iter().enumerate() {
            let o = 8 * i as u32;
            edge_loop_copy_dword(c, reg, EDGE_LOOP_PITCH_CURVE + o, base + 0x30 + o);
        }
        c.mov_s(12, 20);
        Callee {
            addr: 0x0029_0b58,
            f: ee::s07::Audio_EvalCurve5,
        }
        .call(c, 0x0029_3f78)?;
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, S1)); // *volume
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 22);
        Callee {
            addr: 0x0029_0b58,
            f: ee::s07::Audio_EvalCurve5,
        }
        .call(c, 0x0029_3f8c)?;
        c.g[AT][0] = 0x4580_0000;
        c.mtc1(1, 1); // 4096.0
        frame.restore(c, S0, 0x90);
        c.mul_s(0, 0, 1);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 22, 0xb0);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, S2)); // *pitch
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore(c, S2, 0x70);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IntersectInfo_Clear (0x3513d0)

/// `void IntersectInfo_Clear(IntersectInfo* self)` (0x3513d0): zero the two
/// 0x40-byte blocks (at +0 and +0x44) with `MEM_fill` and the word between
/// them (+0x40). Frame 0x20 (s0 = self).
pub fn intersect_info_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x40;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6448, 0x0035_13ec)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x44));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        call_ptr(c, 0x003e_6448, 0x0035_13fc)?;
        Addr(r32(c, S0).wrapping_add(0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BEBAGTInt_GetTrickPoints / GetTrickPoints2 (0x150528 / 0x150540)

/// The trick-bag point table (`0x530600`, 8 bytes per entry: points and an
/// extra word).
#[cfg(test)]
pub(crate) const TRICK_POINTS_TABLE: u32 = 0x0053_0600;

/// `int BEBAGTInt_GetTrickPoints(BEBAGTInt* self, int trick)` (0x150528): the
/// trick's point value from the global table (`0x530600 + trick * 8`).
pub fn beb_agt_int_get_trick_points(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0053_0000;
    c.g[A1][0] = sx(r32(c, A1) << 3);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x600));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    ret(c);
}

/// `int BEBAGTInt_GetTrickPoints2(BEBAGTInt* self, int trick)` (0x150540):
/// the word after the trick's point value in the global table.
pub fn beb_agt_int_get_trick_points_2(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x0053_0000;
    c.g[A1][0] = sx(r32(c, A1) << 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x600));
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x4)).lw(c);
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_CalcSurfaceLoopRemoteGroup1 (0x293b70)

/// `void SSXAudioSystem_CalcSurfaceLoopRemoteGroup1(SSXAudioSystem* self,
/// Rider* rider, int* pitch, int* volume)` (0x293b70): the remote-racer twin
/// of `SSXAudioSystem_CalcSurfaceLoopGroup1`, called with the same arguments.
/// Frame 0x10 (just `ra`).
pub fn ssx_audio_system_calc_surface_loop_remote_group1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0029_3740,
            f: ee::s07::SSXAudioSystem_CalcSurfaceLoopGroup1,
        }
        .call(c, 0x0029_3b80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// isnan (0x40f840)

/// `int isnan(double x)` (0x40f840): the libc test, `(hx & 0x7fffffff) |
/// (lx | -lx) >> 31 > 0x7ff00000`, with `hx`/`lx` the two halves of `a0`
/// (the double is passed in one 64-bit GPR).
pub fn isnan(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = c.g[A0][0] << 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64; // lx
    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64; // hx
    c.g[V1][0] = 0x7fff_0000;
    c.g[A1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] |= 0xffff;
    c.g[V0][0] |= c.g[A1][0];
    c.g[A0][0] &= c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
    c.g[V1][0] = 0x7ff0_0000;
    c.g[A0][0] |= c.g[V0][0];
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 31);
    ret(c);
}

// ---------------------------------------------------------------------------
// EAFile_QueueClose (0x3dd5a0)

/// The EAFile request pool (`0x519ab0`) and its request-list head (`D_00519AE4`,
/// reached as `0x520000 - 0x651c`).
#[cfg(test)]
pub(crate) const EAFILE_QC_LIST_HEAD: u32 = 0x0051_9ae4;

/// A request's words: the flags/state (+0), the queue link (+0xc), the
/// priority (+0x10), the second argument (+0x14) and the node id (+0x24).
pub(crate) const EAFILE_QC_LINK: u32 = 0xc;

pub(crate) const EAFILE_QC_PRIORITY: u32 = 0x10;

pub(crate) const EAFILE_QC_ARG2: u32 = 0x14;

pub(crate) const EAFILE_QC_ID: u32 = 0x24;

/// A request-list node: its id (+0xc) and next link (+0x14).
pub(crate) const EAFILE_QC_LIST_ID: u32 = 0xc;

pub(crate) const EAFILE_QC_LIST_NEXT: u32 = 0x14;

/// `int EAFile_QueueClose(int id, int arg1, int arg2)` (0x3dd5a0): take a
/// request from the pool, set its state to 3 ("close"), store the arguments
/// and the node id, and mark it signalled when a request-list node with that
/// id exists; then hand it to the service queue and return its state word.
/// Frame 0x50 (s0 = arg1, s1 = arg2, s2 = request, s3 = id).
pub fn ea_file_queue_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        Callee {
            addr: 0x003d_e670,
            f: ee::s14::EAFile_AllocRequest,
        }
        .call(c, 0x003d_d5c8)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0052_0000;
        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ff0f_ffff; // clear the state bits
        c.g[A0][0] = 0x30_0000; // state 3
        c.g[V1][0] &= c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, A1).wrapping_sub(0x651c)).lw(c);
        c.g[V1][0] |= c.g[A0][0];
        c.g[A2][0] = 3;
        Addr(r32(c, S2)).sw(c, c.g[V1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_PRIORITY)).sw(c, c.g[S0][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ARG2)).sw(c, c.g[S1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ID)).sw(c, c.g[S3][0]);
        loop {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_ID)).lw(c);
                if c.g[V0][0] != c.g[S3][0] {
                    c.g[A1][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_NEXT)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_d600);
                    }
                    continue;
                }
                c.cycles += 3;
                Addr(r32(c, S2).wrapping_add(EAFILE_QC_LINK)).sw(c, c.g[A2][0]);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x003d_e668,
                    f: ee::s14::EAFile_QueueSignalStub,
                }
                .call(c, 0x003d_d620)?;
            }
            break;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x003d_dfa8,
            f: ee::s14::EAFile_ServiceQueue,
        }
        .call(c, 0x003d_d628)?;
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Debounce_ctor_2` runs the real `ModObj` constructor chain (its
    /// `WorldObj` and sync-process fixtures mirror `mod_obj_ctor_matches`):
    /// definitions with mode 0, 1 and >=2, node flags with the focus bit and
    /// with or without both low bits set, and an empty `a4` source for the
    /// take-over.
    #[test]
    fn debounce_ctor_2_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const DEF: u32 = DATA + 0x300;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        const SCALE: u32 = DATA + 0x4000;
        check(
            ee::s11::f_00342c08__generated,
            debounce_ctor_2,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_2c08, 0);
                c.pc = 0x0034_2c08;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                fill(c, &mut r, SELF, 0x10);
                fill(c, &mut r, NODE, 0x10);
                let flags = (r.next_u32() & !0xf)
                    | if seed % 4 == 0 { 0x100 } else { 0 }
                    | if seed % 2 == 0 { 3 } else { (seed / 8) % 3 };
                w(c, NODE + DEBOUNCE_NODE_FLAGS, flags);
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                for i in 0..4 {
                    w(c, PROCS + 0x44 * i, r.next_u32());
                    w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                    w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
                }
                fill(c, &mut r, 0x0044_aff0, 0x20);
                // World -> game -> race -> focus manager.
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, WORLD + 0x100);
                w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
                w(c, WORLD + 0x200 + 0xa4, FOCUS);
                w(
                    c,
                    FOCUS + 0xd4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, FOCUS + 0xd8, r.next_u32() % 8);
                // The definition: a time, the +8 word and the mode.
                w(c, DEF + DEBOUNCE_DEF_TIME, r.f32(8.0));
                w(c, DEF + DEBOUNCE_DEF_F08, r.next_u32());
                w(c, DEF + DEBOUNCE_DEF_MODE, ((seed / 4) % 3) as u32);
                // The tick-rate scale pointer and its value.
                w(c, DEBOUNCE_RATE_SCALE, SCALE);
                w(
                    c,
                    SCALE + 0x10,
                    [0, 1, 2, 60, u32::MAX][(seed % 5) as usize],
                );
                // The process-list index, as in `mod_obj_ctor_matches`: it
                // reaches `ProcessList_Add` as a byte offset into PROCS.
                let list = (seed % 4) as u64;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = list;
                c.g[A2][0] = sx(NODE);
                c.g[A3][0] = sx(DEF);
                c.g[T0][0] = 0; // no source node: the take-over takes its empty path
            },
        );
    }

    /// `cCSICubicSpline_Init` on a spline block with a heap arena (the real
    /// `cCSICubicSpline_Alloc` allocates the point array) and an old data
    /// pointer to free, counts 0, 1 and many.
    #[test]
    fn c_csi_cubic_spline_init_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        use crate::game::be::beintreward::tests::world;
        use crate::game::bx::bxstring::tests::{alloc, heap};
        use crate::game::bx::bxstringctor::tests::be_library;
        check(
            ee::s10::f_0031d700__generated,
            c_csi_cubic_spline_init,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_d700, 0);
                be_library(c);
                world(c);
                heap(c);
                let old = if seed % 3 == 0 {
                    0
                } else {
                    alloc(c, 0x14 * (1 + r.next_u32() % 16))
                };
                let spline = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, spline, 8);
                c.bus.write32(
                    spline + crate::game::bx::cubicspline::SPLINE_ALLOC_DATA,
                    old,
                );
                c.g[A0][0] = sx(spline);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    _ => (r.next_u32() % 300) as u64,
                };
                c.pc = 0x0031_d700;
            },
        );
    }

    /// `DynamicEmitter_FreeData`: arrays present or empty, and `reset` set or
    /// clear; the heap is up (sizes over the pool range) so `EA_MemFree`
    /// really returns the blocks.
    #[test]
    fn dynamic_emitter_free_data_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        use crate::game::be::beintreward::tests::world;
        use crate::game::bx::bxstring::tests::{alloc, heap};
        use crate::game::bx::bxstringctor::tests::be_library;
        check(
            ee::s12::f_00370cf8__generated,
            dynamic_emitter_free_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_0cf8, 0);
                be_library(c);
                world(c);
                heap(c);
                let emitter = DATA + 0x100;
                fill(c, &mut r, emitter + 0x160, 0x50);
                let particles = if seed % 3 == 0 {
                    0
                } else {
                    alloc(c, 0x200 + r.next_u32() % 0x100)
                };
                let states = if seed % 4 == 0 {
                    0
                } else {
                    alloc(c, 0x200 + r.next_u32() % 0x100)
                };
                c.bus.write32(emitter + EMITTER_FREE_PARTICLES, particles);
                c.bus.write32(emitter + EMITTER_FREE_STATES, states);
                c.g[A0][0] = sx(emitter);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// `SSXAudioSystem_CalcEdgeLoopGroup1`: speed under and over the 0.1
    /// threshold (the level scaled by `speed * 10` or not), an edge getter
    /// that is a fake virtual (forced index 1..8; some seeds unwind there),
    /// previous levels below, above and near the new level (both adjustment
    /// branches), and the real `Audio_EvalCurve5` on sorted five-point curves.
    #[test]
    fn ssx_audio_system_calc_edge_loop_group1_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
        use crate::difftest::{check, Rng, DATA, GAME_GP};
        const AUDIO: u32 = DATA;
        const RIDER: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0xa000;
        check(
            ee::s07::f_00293dc0__generated,
            ssx_audio_system_calc_edge_loop_group1,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_3dc0, 1);
                c.pc = 0x0029_3dc0;
                c.bus
                    .write32(GAME_GP - EDGE_LOOP_THRESHOLD, 0.1f32.to_bits());
                // Curves: x ascending 0..1, then y values.
                for (base, k) in [
                    (EDGE_LOOP_VOLUME_CURVE, 0.5f32),
                    (EDGE_LOOP_PITCH_CURVE, 1.0f32),
                ] {
                    for i in 0..5u32 {
                        c.bus.write32(base + 4 * i, (i as f32 * 0.25).to_bits());
                        c.bus
                            .write32(base + 0x14 + 4 * i, (k * (i * i) as f32 + 0.1).to_bits());
                    }
                }
                let f = |r: &mut Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo)
                };
                c.bus.write32(RIDER + 0x750, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x754, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x758, f(&mut r, 0.0, 1.0).to_bits());
                c.bus
                    .write32(RIDER + EDGE_LOOP_RIDER_SPEED, f(&mut r, 0.0, 0.6).to_bits());
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(RIDER + EDGE_LOOP_RIDER_BODY, vt);
                let index = r.next_u32() % 8;
                // The fake's forced result (zero would mean "a pointer", so the
                // index is 1..8).
                c.bus.write32(LOG_RESULTS, index + 1);
                for i in 0..10u32 {
                    let prev = match (seed + i) % 3 {
                        0 => f(&mut r, -2.0, -1.0),
                        1 => f(&mut r, 1.5, 3.0),
                        _ => f(&mut r, 0.0, 1.0),
                    };
                    c.bus
                        .write32(AUDIO + EDGE_LOOP_LEVELS + 4 * i, prev.to_bits());
                }
                c.g[A0][0] = sx(AUDIO);
                c.g[A1][0] = sx(RIDER);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(OUT + 4);
            },
        );
    }

    /// `IntersectInfo_Clear` zeroes the two blocks and the word between them
    /// (the real `MEM_fill`).
    #[test]
    fn intersect_info_clear_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s11::f_003513d0__generated,
            intersect_info_clear,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_13d0, 0);
                c.pc = 0x0035_13d0;
                let info = DATA + 0x100;
                fill(c, &mut r, info, 0x30);
                c.g[A0][0] = sx(info);
            },
        );
    }

    /// The trick-bag point getters read the global table at
    /// `0x530600 + trick * 8`; tricks 0..0x40 cover the low entries.
    fn trick_points_case(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        use crate::difftest::{check, put_words, scramble, Rng};
        check(gen, hand, 200, &|c, seed| {
            let mut r = Rng(0x1234_5678 ^ (seed as u64) << 8 ^ salt);
            scramble(c, &mut r);
            let mut words = [0u32; 0x80];
            for w in words.iter_mut() {
                *w = r.next_u32();
            }
            put_words(c, TRICK_POINTS_TABLE, &words);
            c.g[A1][0] = (r.next_u32() % 0x40) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn beb_agt_int_get_trick_points_matches() {
        trick_points_case(
            ee::s01::f_00150528__generated,
            beb_agt_int_get_trick_points,
            0x0015_0528,
            0x15_0528,
        );
    }

    #[test]
    fn beb_agt_int_get_trick_points_2_matches() {
        trick_points_case(
            ee::s01::f_00150540__generated,
            beb_agt_int_get_trick_points_2,
            0x0015_0540,
            0x15_0540,
        );
    }

    /// `SSXAudioSystem_CalcSurfaceLoopRemoteGroup1` just calls
    /// `SSXAudioSystem_CalcSurfaceLoopGroup1`; the fixture is that function's
    /// (rider factors, a fake surface getter, previous levels, real curves).
    #[test]
    fn ssx_audio_system_calc_surface_loop_remote_group1_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
        use crate::difftest::{check, Rng, DATA, GAME_GP};
        use crate::game::sound::soundsys::{
            SURFACE_LOOP1_LEVELS, SURFACE_LOOP1_PITCH_CURVE, SURFACE_LOOP1_VOLUME_CURVE,
        };
        const AUDIO: u32 = DATA;
        const RIDER: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0xa000;
        check(
            ee::s07::f_00293b70__generated,
            ssx_audio_system_calc_surface_loop_remote_group1,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_3b70, 1);
                c.pc = 0x0029_3b70;
                c.bus.write32(GAME_GP - 0x45c0, 0.3f32.to_bits());
                for (base, k) in [
                    (SURFACE_LOOP1_PITCH_CURVE, 1.0f32),
                    (SURFACE_LOOP1_VOLUME_CURVE, 0.5),
                ] {
                    for i in 0..5u32 {
                        c.bus.write32(base + 4 * i, (i as f32 * 0.25).to_bits());
                        c.bus
                            .write32(base + 0x14 + 4 * i, (k * (i * i) as f32 + 0.1).to_bits());
                    }
                }
                let f = |r: &mut Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo)
                };
                c.bus.write32(RIDER + 0x750, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x754, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x758, f(&mut r, 0.0, 1.0).to_bits());
                c.bus.write32(RIDER + 0x75c, f(&mut r, 0.0, 0.6).to_bits());
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(RIDER + 0x6c0, vt);
                let index = r.next_u32() % 8;
                c.bus.write32(LOG_RESULTS, index + 1);
                for i in 0..10u32 {
                    let prev = match (seed + i) % 3 {
                        0 => f(&mut r, -2.0, -1.0),
                        1 => f(&mut r, 1.5, 3.0),
                        _ => f(&mut r, 0.0, 1.0),
                    };
                    c.bus
                        .write32(AUDIO + SURFACE_LOOP1_LEVELS + 4 * i, prev.to_bits());
                }
                c.g[A0][0] = sx(AUDIO);
                c.g[A1][0] = sx(RIDER);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(OUT + 4);
            },
        );
    }

    /// `isnan` on zeroes, normal doubles, floats bit patterns and the
    /// NaN/signalling-NaN encodings.
    #[test]
    fn isnan_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s15::f_0040f840__generated, isnan, 200, &|c, seed| {
            let mut r = Rng(0x5eed_0bad ^ seed as u64);
            scramble(c, &mut r);
            let a0 = match seed % 6 {
                0 => 0,
                1 => 0x7ff0_0000_0000_0000, // +inf
                2 => 0xfff0_0000_0000_0001, // -inf, low word set
                3 => 0x7ff8_0000_0000_0000, // NaN
                4 => 0x7ff0_0000_7ff0_0000, // inf high, junk low
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
            c.g[A0][0] = a0;
            c.pc = 0x0040_f840;
        });
    }

    /// `EAFile_QueueClose`: a real request pool (two free slots), a
    /// request-list with no matching, a matching or a second matching node,
    /// and priorities that keep the real service queue on its short path.
    #[test]
    fn ea_file_queue_close_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const POOL: u32 = 0x0051_9ab0;
        const SLOTS: u32 = DATA + 0x1_0000;
        const NODE: u32 = DATA + 0x2_0000;
        const NODE2: u32 = DATA + 0x2_0100;
        const ID: u32 = 0x1234;
        check(
            ee::s14::f_003dd5a0__generated,
            ea_file_queue_close,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d5a0, 0);
                c.pc = 0x003d_d5a0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The request pool: two free slots, a zeroed mutex and a zeroed
                // request queue (the real service queue then takes its short
                // path).
                for i in 0..0x10u32 {
                    w(c, POOL + 4 * i, 0);
                }
                w(c, POOL, 2);
                w(c, POOL + 0x28, SLOTS);
                for i in 0..0x20u32 {
                    w(c, SLOTS + 4 * i, 0);
                }
                // The request list.
                let head = if seed % 3 == 0 { 0 } else { NODE };
                w(c, EAFILE_QC_LIST_HEAD, head);
                fill(c, &mut r, NODE, 0x20);
                fill(c, &mut r, NODE2, 0x20);
                w(
                    c,
                    NODE + EAFILE_QC_LIST_ID,
                    if seed % 2 == 0 { ID } else { ID + 1 },
                );
                w(
                    c,
                    NODE + EAFILE_QC_LIST_NEXT,
                    if seed % 4 == 0 { NODE2 } else { 0 },
                );
                w(c, NODE2 + EAFILE_QC_LIST_ID, ID);
                w(c, NODE2 + EAFILE_QC_LIST_NEXT, 0);
                c.g[A0][0] = ID as u64;
                c.g[A1][0] = 1 + (r.next_u32() % 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }
}
