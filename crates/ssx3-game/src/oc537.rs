//! Staging module oc537 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc537`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0027_b948,
        script_engine_get_rider_pose,
        "ScriptEngine_GetRiderPose",
    ),
    (
        0x0027_c0a0,
        script_cam_manual_get_key_block,
        "ScriptCam_ManualGetKeyBlock",
    ),
    (
        0x0027_c0c0,
        script_cam_manual_get_track_block,
        "ScriptCam_ManualGetTrackBlock",
    ),
    (
        0x0027_c2b0,
        script_cam_target_get_key_block,
        "ScriptCam_TargetGetKeyBlock",
    ),
    (
        0x0027_c2d0,
        script_cam_target_get_track_block,
        "ScriptCam_TargetGetTrackBlock",
    ),
    (
        0x0027_c4b8,
        script_cam_get_subject_params,
        "ScriptCam_GetSubjectParams",
    ),
    (
        0x0027_c4c0,
        script_cam_subject_get_key_block,
        "ScriptCam_SubjectGetKeyBlock",
    ),
    (
        0x0027_c4e0,
        script_cam_subject_get_track_block,
        "ScriptCam_SubjectGetTrackBlock",
    ),
    (
        0x0027_c6a0,
        script_cam_get_algorithmic_params,
        "ScriptCam_GetAlgorithmicParams",
    ),
    (
        0x0027_c6a8,
        script_cam_algorithmic_get_key_block,
        "ScriptCam_AlgorithmicGetKeyBlock",
    ),
    (
        0x0027_c6c8,
        script_cam_algorithmic_get_track_block,
        "ScriptCam_AlgorithmicGetTrackBlock",
    ),
    (
        0x0027_c890,
        script_view_get_key_block,
        "ScriptView_GetKeyBlock",
    ),
    (
        0x0027_c8b0,
        script_view_get_track_block,
        "ScriptView_GetTrackBlock",
    ),
    (
        0x0027_c960,
        script_rider_get_key_block,
        "ScriptRider_GetKeyBlock",
    ),
    (
        0x0027_c980,
        script_rider_get_track_block,
        "ScriptRider_GetTrackBlock",
    ),
    (
        0x0027_d788,
        script_obj_cam_set_weight,
        "ScriptObjCam_SetWeight",
    ),
];

/// `Math_Atan` (0x31c228), reached through the function table.
const SCRIPT_POSE_ATAN: u32 = 0x0031_c228;

/// `InSeq_ChannelGetBlockPayload` (0x2742e0): overridden elsewhere.
const SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD: Callee = Callee {
    addr: 0x0027_42e0,
    f: ee::s07::f_002742e0,
};
/// `InSeq_FindChannelData` (0x274d90): generated.
const SCRIPT_CHANNEL_FIND_DATA: Callee = Callee {
    addr: 0x0027_4d90,
    f: ee::s07::f_00274d90,
};
/// `InSeq_GetBlockPayload` (0x274318): generated.
const SCRIPT_BLOCK_GET_PAYLOAD: Callee = Callee {
    addr: 0x0027_4318,
    f: ee::s07::f_00274318,
};

/// `cSSXScriptEngine::FillOrientationFromRiderOrientationInfo(self, obj, pos,
/// roll, pitch, yaw)` (0x27b948). `obj` holds a sub-object at +0x6c0 whose
/// vtable method at +0x2c (`this` adjusted by the delta at +0x28) returns the
/// rider position, copied to `pos`. The heading is row 1 of its matrix (+0x1b0,
/// copied to the stack as `dir`): roll is 0, yaw = atan2(dir.y, dir.x), pitch =
/// atan2(dir.z, length(dir.x, dir.y)), each with the quadrant corrections of
/// the original. FPU code; the branches follow the generated blocks.
pub fn script_engine_get_rider_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save_fpr(c, 22, 0x90);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x20);

        let obj = c.g[A1][0];
        c.g[S0][0] = obj;
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[T0][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T1][0];
        let sub = r32(c, S0).wrapping_add(0x6c0);
        c.g[A1][0] = sx(sub);
        c.f[22] = 0;

        // Position: the sub-object's virtual method at +0x2c.
        let vt = Addr(sub).lw(c);
        c.g[V1][0] = vt;
        let slot = VSlot::read(c, vt, 0x28);
        c.g[A0][0] = slot.this(c.g[A1][0]);
        c.g[V0][0] = slot.func;
        call_ptr(c, slot.func as u32, 0x0027_b9a0)?;

        // *pos = the returned vector; dir = row 1 of the matrix, on the stack.
        let got = r32(c, V0);
        c.lq(V1, got);
        let pos = r32(c, S1);
        c.sq(V1, pos);
        let row = r32(c, S0).wrapping_add(0x1b0);
        c.lq(V0, row);
        let sp0 = sp(c);
        c.sq(V0, sp0);

        let gp = r32(c, GP);
        c.f[20] = Addr(sp0).read_u32(c);
        let zero = FpuF32(c.f[22]);
        let dx_zero = FpuF32(c.f[20]).eq_s(zero);
        crate::abi::set_fcc(c, dx_zero);
        let dx_nonzero = !dx_zero;
        c.f[21] = Addr(sp0.wrapping_add(4)).read_u32(c);

        'axis: {
            'yaw_atan: {
                'vec_x: {
                    if dx_nonzero {
                        break 'vec_x;
                    }
                    // dir.x == 0: yaw is +-pi/2 (or 0 when dir.y is 0).
                    c.cycles += 4;
                    let dy_zero = FpuF32(c.f[21]).eq_s(zero);
                    crate::abi::set_fcc(c, dy_zero);
                    c.f[12] = c.f[21];
                    if dy_zero {
                        break 'yaw_atan;
                    }
                    c.cycles += 4;
                    let dy_nonneg = zero.le(FpuF32(c.f[21]));
                    crate::abi::set_fcc(c, dy_nonneg);
                    c.f[12] = Addr(gp.wrapping_sub(0x47cc)).read_u32(c);
                    c.f[0] = Addr(sp0).read_u32(c);
                    if !dy_nonneg {
                        break 'yaw_atan;
                    }
                    c.cycles += 3;
                    c.f[12] = Addr(gp.wrapping_sub(0x47c8)).read_u32(c);
                    c.f[1] = Addr(sp0.wrapping_add(4)).read_u32(c);
                    break 'axis;
                }

                // dir.x != 0: yaw = atan(dir.y / dir.x) with the quadrant fix.
                c.cycles += 7;
                c.f[12] = FpuF32(c.f[21]).div(FpuF32(c.f[20])).0;
                call_ptr(c, SCRIPT_POSE_ATAN, 0x0027_b9fc)?;
                // The callee may leave f22 changed: re-read the zero the
                // original compares against.
                let zero = FpuF32(c.f[22]);
                let dx_negative = FpuF32(c.f[20]).lt(zero);
                crate::abi::set_fcc(c, dx_negative);
                c.f[12] = c.f[0];
                if !dx_negative {
                    break 'yaw_atan;
                }
                'yaw_sign: {
                    c.cycles += 3;
                    let dy_positive = zero.lt(FpuF32(c.f[21]));
                    crate::abi::set_fcc(c, dy_positive);
                    if !dy_positive {
                        break 'yaw_sign;
                    }
                    c.cycles += 3;
                    c.f[0] = Addr(gp.wrapping_sub(0x47c4)).read_u32(c);
                    c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[0])).0;
                    break 'yaw_atan;
                }
                c.cycles += 2;
                c.f[0] = Addr(gp.wrapping_sub(0x47c0)).read_u32(c);
                c.f[12] = (FpuF32(c.f[12]) - FpuF32(c.f[0])).0;
            }
            // Shared exit of the yaw paths: dir.x, dir.y to f0, f1.
            c.cycles += 2;
            c.f[0] = Addr(sp0).read_u32(c);
            c.f[1] = Addr(sp0.wrapping_add(4)).read_u32(c);
            break 'axis;
        }

        'done: {
            'pitch: {
                'horiz_zero: {
                    // Horizontal length of dir.x, dir.y; *yaw is in f12.
                    c.cycles += 12;
                    let fx = FpuF32(c.f[0]);
                    c.f[2] = (fx * fx).0;
                    let yaw = c.f[12];
                    Addr(r32(c, S2)).write_u32(c, yaw);
                    let fy = FpuF32(c.f[1]);
                    c.f[3] = (fy * fy).0;
                    let f0_saved = c.f[0];
                    Addr(sp0.wrapping_add(0x10)).write_u32(c, f0_saved);
                    c.f[22] = 0;
                    let f1_saved = c.f[1];
                    Addr(sp0.wrapping_add(0x14)).write_u32(c, f1_saved);
                    let horiz_sq = FpuF32(c.f[2]) + FpuF32(c.f[3]);
                    c.f[2] = horiz_sq.0;
                    c.f[21] = Addr(sp0.wrapping_add(8)).read_u32(c);
                    let horiz = horiz_sq.sqrt();
                    c.f[20] = horiz.0;
                    let len_zero = horiz.eq_s(zero);
                    crate::abi::set_fcc(c, len_zero);
                    if !len_zero {
                        break 'horiz_zero;
                    }
                    // Zero horizontal length: pitch is dir.z or +-pi/2.
                    c.cycles += 4;
                    let dz_zero = FpuF32(c.f[21]).eq_s(zero);
                    crate::abi::set_fcc(c, dz_zero);
                    c.f[0] = c.f[21];
                    if dz_zero {
                        break 'pitch;
                    }
                    c.cycles += 4;
                    let dz_nonneg = zero.le(FpuF32(c.f[21]));
                    crate::abi::set_fcc(c, dz_nonneg);
                    c.f[0] = Addr(gp.wrapping_sub(0x47bc)).read_u32(c);
                    if !dz_nonneg {
                        let v = c.f[0];
                        Addr(r32(c, S3)).write_u32(c, v);
                        break 'done;
                    }
                    c.cycles += 3;
                    c.f[0] = Addr(gp.wrapping_sub(0x47b8)).read_u32(c);
                    let v = c.f[0];
                    Addr(r32(c, S3)).write_u32(c, v);
                    break 'done;
                }

                // pitch = atan(dir.z / length) with the quadrant fix.
                'pitch_atan: {
                    c.cycles += 7;
                    c.f[12] = FpuF32(c.f[21]).div(FpuF32(c.f[20])).0;
                    call_ptr(c, SCRIPT_POSE_ATAN, 0x0027_ba9c)?;
                    let zero = FpuF32(c.f[22]);
                    let len_negative = FpuF32(c.f[20]).lt(zero);
                    crate::abi::set_fcc(c, len_negative);
                    c.f[12] = c.f[0];
                    if !len_negative {
                        break 'pitch_atan;
                    }
                    'pitch_sign: {
                        c.cycles += 3;
                        let dz_positive = zero.lt(FpuF32(c.f[21]));
                        crate::abi::set_fcc(c, dz_positive);
                        if !dz_positive {
                            break 'pitch_sign;
                        }
                        c.cycles += 3;
                        c.f[0] = Addr(gp.wrapping_sub(0x47b4)).read_u32(c);
                        c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[0])).0;
                        break 'pitch_atan;
                    }
                    c.cycles += 2;
                    c.f[0] = Addr(gp.wrapping_sub(0x47b0)).read_u32(c);
                    c.f[12] = (FpuF32(c.f[12]) - FpuF32(c.f[0])).0;
                }
                c.cycles += 1;
                c.f[0] = c.f[12];
            }
            c.cycles += 1;
            let pitch = c.f[0];
            Addr(r32(c, S3)).write_u32(c, pitch);
        }

        // roll = 0.
        c.cycles += 12;
        Addr(r32(c, S4)).sw(c, 0);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 22, 0x90);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptCam_GetSubjectParams(self)` (0x27c4b8): the word at self+0x20.
pub fn script_cam_get_subject_params(c: &mut Cpu) {
    c.cycles += 2;
    let addr = r32(c, A0).wrapping_add(0x20);
    c.g[V0][0] = Addr(addr).lw(c);
    ret(c);
}

/// `ScriptCam_GetAlgorithmicParams(self)` (0x27c6a0): the word at self+0x20.
pub fn script_cam_get_algorithmic_params(c: &mut Cpu) {
    c.cycles += 2;
    let addr = r32(c, A0).wrapping_add(0x20);
    c.g[V0][0] = Addr(addr).lw(c);
    ret(c);
}

/// `ScriptObjCam_SetWeight(self, weight)` (0x27d788): stores the float
/// argument f12 at self+0x18.
pub fn script_obj_cam_set_weight(c: &mut Cpu) {
    c.cycles += 2;
    let weight = crate::abi::farg(c, 0);
    let addr = r32(c, A0).wrapping_add(0x18);
    Addr(addr).write_fpu(c, weight);
    ret(c);
}

/// `ScriptCam_ManualGetKeyBlock(self)` (0x27c0a0): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_cam_manual_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c0b0,
    )
}

/// `ScriptCam_TargetGetKeyBlock(self)` (0x27c2b0): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_cam_target_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c2c0,
    )
}

/// `ScriptCam_SubjectGetKeyBlock(self)` (0x27c4c0): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_cam_subject_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c4d0,
    )
}

/// `ScriptCam_AlgorithmicGetKeyBlock(self)` (0x27c6a8): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_cam_algorithmic_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c6b8,
    )
}

/// `ScriptView_GetKeyBlock(self)` (0x27c890): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_view_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c8a0,
    )
}

/// `ScriptRider_GetKeyBlock(self)` (0x27c960): `InSeq_ChannelGetBlockPayload(self)`.
pub fn script_rider_get_key_block(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(SCRIPT_CHANNEL_GET_BLOCK_PAYLOAD),
        0x0027_c970,
    )
}

/// The track-block wrappers' shape: `FindChannelData(self, entity)` gives
/// the channel, then `GetBlockPayload(channel, variation)` with the variation
/// (a2) kept in s0 across the first call.
fn script_track_block_wrapper(c: &mut Cpu, find_ret: u32, payload_ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        SCRIPT_CHANNEL_FIND_DATA.call(c, find_ret)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        SCRIPT_BLOCK_GET_PAYLOAD.call(c, payload_ret)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptCam_ManualGetTrackBlock(self, entity, variation)` (0x27c0c0).
pub fn script_cam_manual_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c0d4, 0x0027_c0e0)
}

/// `ScriptCam_TargetGetTrackBlock(self, entity, variation)` (0x27c2d0).
pub fn script_cam_target_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c2e4, 0x0027_c2f0)
}

/// `ScriptCam_SubjectGetTrackBlock(self, entity, variation)` (0x27c4e0).
pub fn script_cam_subject_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c4f4, 0x0027_c500)
}

/// `ScriptCam_AlgorithmicGetTrackBlock(self, entity, variation)` (0x27c6c8).
pub fn script_cam_algorithmic_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c6dc, 0x0027_c6e8)
}

/// `ScriptView_GetTrackBlock(self, entity, variation)` (0x27c8b0).
pub fn script_view_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c8c4, 0x0027_c8d0)
}

/// `ScriptRider_GetTrackBlock(self, entity, variation)` (0x27c980).
pub fn script_rider_get_track_block(c: &mut Cpu) {
    script_track_block_wrapper(c, 0x0027_c994, 0x0027_c9a0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// Gate word for the channel-payload fake (test machines only).
    const OC537_GATE: u32 = DATA + 0x200000;
    const OC537_MAGIC: u32 = 0x0537_0001;
    static OC537_PREV_CHANNEL_PAYLOAD: AtomicUsize = AtomicUsize::new(0);

    /// Records the `InSeq_ChannelGetBlockPayload` calls while the gate is set.
    fn oc537_fake_channel_payload(c: &mut Cpu) {
        if c.bus.read32(OC537_GATE) != OC537_MAGIC {
            return fakes::fall_back(
                c,
                &OC537_PREV_CHANNEL_PAYLOAD,
                ee::s07::f_002742e0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_42e0, ra);
    }

    fn oc537_hook_channel_payload() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002742E0),
            oc537_fake_channel_payload,
            &OC537_PREV_CHANNEL_PAYLOAD,
        );
    }

    /// The one-call wrappers: gate on, random arguments.
    fn oc537_key_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(OC537_GATE, OC537_MAGIC);
        c.g[A0][0] = sx(fakes::ptr(&mut r));
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
    }

    /// The track-block wrappers: a small channel count and variation, and a
    /// self block of random words (the lookups only read it).
    fn oc537_track_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let me = DATA + 0x40000;
        fill(c, &mut r, me, 0x40);
        c.g[A0][0] = sx(me);
        c.g[A1][0] = sx(seed % 4);
        c.g[A2][0] = sx(r.next_u32() % 4);
    }

    #[test]
    fn script_engine_get_rider_pose_matches() {
        covered(
            ee::s07::f_0027b948__generated,
            script_engine_get_rider_pose,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27b948, 1);
                let obj = DATA + 0x10000;
                let vt = DATA + 0x30000;
                c.bus.write16(vt + 0x28, (r.next_u32() as u16) & !3);
                c.bus.write32(vt + 0x2c, FAKE);
                c.bus.write32(obj + 0x6c0, vt);
                // Row 1 of the matrix: zero, positive, negative or random.
                let comp = |r: &mut Rng, pick: u32| match pick {
                    0 => 0,
                    1 => 1.25f32.to_bits(),
                    2 => (-0.75f32).to_bits(),
                    _ => r.f32(2.0),
                };
                let x = comp(&mut r, seed % 4);
                let y = comp(&mut r, seed / 4 % 4);
                let z = comp(&mut r, seed / 16 % 4);
                c.bus.write32(obj + 0x1b0, x);
                c.bus.write32(obj + 0x1b4, y);
                c.bus.write32(obj + 0x1b8, z);
                c.bus.write32(obj + 0x1bc, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x20000);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
                c.g[T0][0] = sx(DATA + 0x2200);
                c.g[T1][0] = sx(DATA + 0x2300);
            },
        );
    }

    #[test]
    fn script_cam_manual_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c0a0__generated,
            script_cam_manual_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c0a0),
        );
    }

    #[test]
    fn script_cam_target_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c2b0__generated,
            script_cam_target_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c2b0),
        );
    }

    #[test]
    fn script_cam_subject_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c4c0__generated,
            script_cam_subject_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c4c0),
        );
    }

    #[test]
    fn script_cam_algorithmic_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c6a8__generated,
            script_cam_algorithmic_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c6a8),
        );
    }

    #[test]
    fn script_view_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c890__generated,
            script_view_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c890),
        );
    }

    #[test]
    fn script_rider_get_key_block_matches() {
        oc537_hook_channel_payload();
        covered(
            ee::s07::f_0027c960__generated,
            script_rider_get_key_block,
            SEEDS,
            &|c, seed| oc537_key_setup(c, seed, 0x0c960),
        );
    }

    #[test]
    fn script_cam_manual_get_track_block_matches() {
        check(
            ee::s07::f_0027c0c0__generated,
            script_cam_manual_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c0c0),
        );
    }

    #[test]
    fn script_cam_target_get_track_block_matches() {
        check(
            ee::s07::f_0027c2d0__generated,
            script_cam_target_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c2d0),
        );
    }

    #[test]
    fn script_cam_subject_get_track_block_matches() {
        check(
            ee::s07::f_0027c4e0__generated,
            script_cam_subject_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c4e0),
        );
    }

    #[test]
    fn script_cam_algorithmic_get_track_block_matches() {
        check(
            ee::s07::f_0027c6c8__generated,
            script_cam_algorithmic_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c6c8),
        );
    }

    #[test]
    fn script_view_get_track_block_matches() {
        check(
            ee::s07::f_0027c8b0__generated,
            script_view_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c8b0),
        );
    }

    #[test]
    fn script_rider_get_track_block_matches() {
        check(
            ee::s07::f_0027c980__generated,
            script_rider_get_track_block,
            SEEDS,
            &|c, seed| oc537_track_setup(c, seed, 0x0c980),
        );
    }

    #[test]
    fn script_cam_get_subject_params_matches() {
        check(
            ee::s07::f_0027c4b8__generated,
            script_cam_get_subject_params,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0c4b8, 0);
                fill(c, &mut r, DATA + 0x50000, 0x10);
                c.g[A0][0] = sx(DATA + 0x50000);
            },
        );
    }

    #[test]
    fn script_cam_get_algorithmic_params_matches() {
        check(
            ee::s07::f_0027c6a0__generated,
            script_cam_get_algorithmic_params,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0c6a0, 0);
                fill(c, &mut r, DATA + 0x50000, 0x10);
                c.g[A0][0] = sx(DATA + 0x50000);
            },
        );
    }

    #[test]
    fn script_obj_cam_set_weight_matches() {
        check(
            ee::s07::f_0027d788__generated,
            script_obj_cam_set_weight,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x0d788, 0);
                c.g[A0][0] = sx(DATA + 0x60000);
            },
        );
    }
}
