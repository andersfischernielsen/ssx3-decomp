//! Staging wave 201/202 (oc215): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use crate::{abi, guest::Addr};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0032_52f8,
        input_vm_read_pad_channel,
        "InputVM_ReadPadChannel",
    ),
    (0x0032_79d0, ai_fwd_diff_cache_get, "AIFwdDiffCache_Get"),
    (
        0x0012_1aa0,
        rider_update_orientation,
        "Rider_UpdateOrientation",
    ),
    (
        0x002f_5400,
        world_light_man_get_instance_lighting,
        "WorldLightMan_GetInstanceLighting",
    ),
    (
        0x0024_29b0,
        occlusion_man_build_occluder_planes,
        "OcclusionMan_BuildOccluderPlanes",
    ),
    (0x0011_39a0, air_predictor_step, "AirPredictor_Step"),
];

// ---------------------------------------------------------------------------
// InputVM_ReadPadChannel (0x003252f8)
// ---------------------------------------------------------------------------

/// The `switch (kind)` jump table of `InputVM_ReadPadChannel` in `.rodata`
/// and its case labels (kinds 0..=5).
const PAD_CHANNEL_TABLE: u32 = 0x0048_e310;
const PAD_CHANNEL_CASES: [u32; 6] = [
    0x0032_5320,
    0x0032_5344,
    0x0032_539c,
    0x0032_53d4,
    0x0032_53f8,
    0x0032_541c,
];

/// `InputVM.defKind`: the kind used for kind 5 ("default").
const PADVM_DEF_KIND: u32 = 0x0;
/// `InputVM.list`: the pad channel list (`{ int count; Entry e[]; }`).
const PADVM_LIST: u32 = 0x4;
/// One channel entry is 0x1c bytes; the list's entries start at +4.
const PAD_ENTRY_SIZE: u32 = 0x1c;
/// Entry fields (relative to `list + idx * 0x1c`): the float value (+4) and
/// the unsigned words read by kinds 1..=4.
const PAD_ENTRY_VALUE: u32 = 0x4;
const PAD_ENTRY_KIND_WORD: [u32; 5] = [0, 0x10, 0x8, 0xc, 0x14];

/// `float InputVM_ReadPadChannel(InputVM* self, int idx, int kind)`
/// (0x003252f8): the value of channel `idx` of the input VM's pad channel
/// list, as a float. Kind 0 reads the entry's float, kinds 1..=4 one of its
/// unsigned words (converted unsigned to float), kind 5 retries with the
/// VM's default kind; an index past the count gives 0 (kinds 0) or the
/// conversion of 0, an unknown kind 0.0.
pub fn input_vm_read_pad_channel(c: &mut Cpu) {
    c.cycles += 1;
    c.g[A3][0] = c.g[A0][0];
    let this = r32(c, A3);
    let idx = c.g[A1][0];

    // 0x003252fc: the switch head (kind 5 comes back here).
    loop {
        c.cycles += 3;
        let kind = c.g[A2][0];
        if kind >= 6 {
            c.g[V0][0] = 0x49_0000;
            break;
        }
        c.cycles += 6;
        let entry = PAD_CHANNEL_TABLE.wrapping_add((kind as u32) << 2);
        c.g[V1][0] = sx(entry);
        c.g[V0][0] = sx(PAD_CHANNEL_TABLE);
        c.g[A0][0] = sx(PAD_CHANNEL_CASES[kind as usize]);

        if kind == 5 {
            // 0x0032541c: kind = self->defKind, back to the head.
            c.cycles += 2;
            c.g[A2][0] = Addr(this.wrapping_add(PADVM_DEF_KIND)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0032_52fc);
            }
            continue;
        }

        // Every case: `idx < list->count` (64-bit signed compare).
        c.cycles += if kind == 1 { 3 } else { 5 };
        let list = Addr(this.wrapping_add(PADVM_LIST)).lw(c);
        c.g[A0][0] = list;
        let count = Addr(list as u32).lw(c);
        let in_range = (idx as i64) < (count as i64);
        if kind == 1 {
            c.cycles += 2;
        }
        c.g[V0][0] = PAD_ENTRY_SIZE as u64;

        if kind == 0 {
            if !in_range {
                break;
            }
            // 0x00325334: f0 = entry.value.
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            c.f[0] = Addr(at.wrapping_add(PAD_ENTRY_VALUE)).read_u32(c);
            abi::ret(c);
            return;
        }

        // Kinds 1..=4: the entry's word, or 0 past the count.
        let word = if in_range {
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            let w = Addr(at.wrapping_add(PAD_ENTRY_KIND_WORD[kind as usize])).lw(c);
            if kind == 3 || kind == 4 {
                c.g[V0][0] = w;
                if event_due(c) {
                    deliver_event_here(c, 0x0032_53c4);
                }
            }
            w
        } else {
            // (Kinds 3 and 4 jump back to 0x003253c0.)
            if kind >= 3 && event_due(c) {
                deliver_event_here(c, 0x0032_53c0);
            }
            c.cycles += 1;
            0
        };
        c.g[V0][0] = word;

        // 0x0032536c / 0x003253c4: unsigned int to float.
        c.cycles += 2;
        let half = sx((word as u32) >> 1);
        c.g[V1][0] = half;
        if (word as i64) >= 0 {
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5374);
            }
            // 0x00325374
            c.cycles += 4;
            c.f[0] = FpuF32::from_i32(word as i32).0;
        } else {
            // (x >> 1 | x & 1) converted and doubled.
            c.cycles += if kind == 1 { 1 } else { 2 };
            c.g[V0][0] = word & 1;
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5388);
            }
            // 0x00325388
            c.cycles += 5;
            let v = c.g[V0][0] | half;
            c.g[V0][0] = v;
            let f = FpuF32::from_i32(v as i32);
            c.f[0] = (f + f).0;
        }
        abi::ret(c);
        return;
    }

    // 0x00325424: no value.
    c.cycles += 3;
    c.f[0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AIFwdDiffCache_Get (0x003279d0)
// ---------------------------------------------------------------------------

/// `AIFwdDiffCache` fields: the least recently used node, the hash bucket
/// array and the bucket count.
const FDC_LRU: u32 = 0x4;
const FDC_BUCKETS: u32 = 0x8;
const FDC_COUNT: u32 = 0xc;
/// Cache node fields: the key, the LRU ring's next/prev, the bucket chain.
const FDN_KEY: u32 = 0x0;
const FDN_LRU_NEXT: u32 = 0x4;
const FDN_HASH_NEXT: u32 = 0xc;
/// The source spline's cache key.
const FD_SRC_KEY: u32 = 0x150;

const FDC_TOUCH: Callee = Callee {
    addr: 0x0032_7ac0,
    f: ee::s10::f_00327ac0,
};
const FDC_HASH_REMOVE: Callee = Callee {
    addr: 0x0032_7b30,
    f: ee::s10::f_00327b30,
};
const FDC_HASH_INSERT: Callee = Callee {
    addr: 0x0032_7bb0,
    f: ee::s10::f_00327bb0,
};
const FDC_BUILD_ENTRY: Callee = Callee {
    addr: 0x0032_7c00,
    f: ee::s10::f_00327c00,
};

/// `void AIFwdDiffCache_Get(AIFwdDiffCache* self, const void* src,
/// Node** out)` (0x003279d0): the forward-difference table for the AI
/// spline `src`, cached by its key (`src+0x150`). The key's bucket chain
/// (sorted ascending) is searched; a hit is moved to the front of the LRU
/// ring, a miss recycles the least recently used node (rehashed under the
/// new key, the ring advanced, the table rebuilt from `src`). `*out` is the
/// node.
pub fn ai_fwd_diff_cache_get(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = r32(c, S2);
        c.g[V0][0] = Addr(this.wrapping_add(FDC_COUNT)).lw(c);
        c.g[S1][0] = Addr(r32(c, S4).wrapping_add(FD_SRC_KEY)).lw(c);

        // 0x00327a08: the key's bucket.
        c.cycles += 7;
        let (_, bucket) = abi::divu(c, r32(c, S1), r32(c, V0));
        c.g[A0][0] = Addr(this.wrapping_add(FDC_BUCKETS)).lw(c);
        c.g[V1][0] = sx((bucket << 2).wrapping_add(r32(c, A0)));
        c.g[S0][0] = Addr(r32(c, V1)).lw(c);

        // 0x00327a44: walk the chain while node->key <= key.
        loop {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.g[S0][0] = Addr(this.wrapping_add(FDC_LRU)).lw(c);
                break;
            }
            c.cycles += 5;
            let node_key = Addr(r32(c, S0).wrapping_add(FDN_KEY)).lw(c);
            c.g[V1][0] = node_key;
            c.g[V0][0] = (c.g[S1][0] < node_key) as u64;
            if c.g[V0][0] != 0 {
                // 0x00327a60: past the key, a miss.
                c.cycles += 1;
                c.g[S0][0] = Addr(this.wrapping_add(FDC_LRU)).lw(c);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_7a28);
            }
            // 0x00327a28
            c.cycles += 2;
            if c.g[V1][0] != c.g[S1][0] {
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(FDN_HASH_NEXT)).lw(c);
                continue;
            }
            // 0x00327a30: a hit.
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            FDC_TOUCH.call(c, 0x0032_7a3c)?;
            Addr(r32(c, S3)).sw(c, c.g[S0][0]);
            epilogue(c, frame);
            return Ok(());
        }

        // 0x00327a64: recycle the least recently used node.
        c.cycles += 14;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        FDC_HASH_REMOVE.call(c, 0x0032_7a70)?;
        let node = r32(c, S0);
        Addr(node.wrapping_add(FDN_KEY)).sw(c, c.g[S1][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        FDC_HASH_INSERT.call(c, 0x0032_7a80)?;
        c.g[V0][0] = Addr(node.wrapping_add(FDN_LRU_NEXT)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        Addr(this.wrapping_add(FDC_LRU)).sw(c, c.g[V0][0]);
        FDC_BUILD_ENTRY.call(c, 0x0032_7a98)?;
        Addr(r32(c, S3)).sw(c, c.g[S0][0]);
        epilogue(c, frame);
        Ok(())
    });

    /// 0x00327a9c
    fn epilogue(c: &mut Cpu, frame: Frame) {
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
    }
}

// ---------------------------------------------------------------------------
// Rider_UpdateOrientation (0x00121aa0)
// ---------------------------------------------------------------------------

/// FPU register `r` as a value.
#[inline(always)]
fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// Set FPU register `r`.
#[inline(always)]
fn set_fr(c: &mut Cpu, r: usize, v: FpuF32) {
    c.f[r] = v.0;
}

/// `c.eq.s a, b` (FCC set), returning the condition.
#[inline(always)]
fn fcc_eq(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let t = a.eq_s(b);
    abi::set_fcc(c, t);
    t
}

/// `c.lt.s a, b` (FCC set), returning the condition.
#[inline(always)]
fn fcc_lt(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let t = a.lt(b);
    abi::set_fcc(c, t);
    t
}

/// `c.le.s a, b` (FCC set), returning the condition.
#[inline(always)]
fn fcc_le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let t = a.le(b);
    abi::set_fcc(c, t);
    t
}

/// `Rider.orient`: the rider's orientation quaternion (x, y, z, w).
const RIDER_QUAT: u32 = 0x120;
/// The static zero vec4 and the reference axis (both in `.bss`, written at
/// boot).
const ZERO_VEC4: u32 = 0x004f_f120;
const REF_AXIS: u32 = 0x004f_f140;
/// `.lit4` constants (gp-relative, subtracted): -pi/2, pi/2, pi, pi, pi/2,
/// 1e-5, pi/2, 1e-5 and the seconds per tick (1/60).
const GP_NEG_HALF_PI: u32 = 0x78e8;
const GP_HALF_PI_A: u32 = 0x78e4;
const GP_PI_A: u32 = 0x78e0;
const GP_PI_B: u32 = 0x78dc;
const GP_HALF_PI_B: u32 = 0x78d8;
const GP_EPS_A: u32 = 0x78d4;
const GP_HALF_PI_C: u32 = 0x78d0;
const GP_EPS_B: u32 = 0x78cc;
const GP_TICK_STEP: u32 = 0x78c8;

const QUAT_FROM_TWO_VECTORS: u32 = 0x0031_bb30;
const MATH_ATAN: u32 = 0x0031_c228;
const MATH_SIN_COS: u32 = 0x0031_be50;
const MATH_ASIN: u32 = 0x0031_c128;
const MATH_SIN: u32 = 0x0031_bf60;
const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::f_0011e098,
};

/// `vf8 = vf4 * vf5` (quaternion product, VU0 macro ops).
fn quat_mul_vu0(c: &mut Cpu) {
    let v = c.vu0();
    v.mul(0b1111, 7, 4, Src::Vec(5));
    v.opmula(4, 5);
    v.opmsub(6, 5, 4);
    v.mula(0b1110, 4, Src::Bc(5, W));
    v.madda(0b1110, 5, Src::Bc(4, W));
    v.suba(0b0001, 7, Src::Bc(7, X));
    v.msuba(0b0001, 0, Src::Bc(7, Y));
    v.msub(0b0001, 8, 0, Src::Bc(7, Z));
    v.madd(0b1110, 8, 6, Src::Bc(0, W));
}

/// A gp-relative `.lit4` float (`lwc1 f, -off(gp)`).
fn gp_lit(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(r32(c, GP).wrapping_sub(off)).read_fpu(c)
}

/// `void Rider_UpdateOrientation(Rider* rider, const Vec4* up,
/// const Vec4* heading, float rate, float maxTurn)` (0x00121aa0): turns
/// the rider's orientation toward a target: the quaternion that brings the
/// rider's local Z axis onto `up`, then (unless `heading` is the zero
/// vector) spun about `up` to face `heading` (atan of the heading in the
/// target frame). The rotation from the current to the target orientation
/// is limited to `min(maxTurn, angle * rate) / 60` per tick about its
/// axis (the reference axis when it is tiny), applied to the rider's
/// quaternion, and the world matrix rebuilt.
pub fn rider_update_orientation(c: &mut Cpu) {
    guest(c, |c| {
        // 0x00121aa0: frame, the rider's local Z axis.
        c.cycles += 52;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x3f80_0000;
        set_fr(c, 7, FpuF32::ONE);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 24, 0xd0);
        c.g[S2][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0xc8);
        c.f[24] = c.f[12];
        frame.save_ra(c, 0x70);
        c.f[23] = c.f[13];
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        let s = sp(c);
        let rider = r32(c, S1);
        let up = r32(c, S2);
        let heading = r32(c, S0);
        let quat = rider.wrapping_add(RIDER_QUAT);
        c.g[A0][0] = sx(s);
        c.g[A1][0] = sx(s + 0x10);
        c.g[A2][0] = c.g[S2][0];
        let x = Addr(quat).read_fpu(c);
        let y = Addr(quat + 4).read_fpu(c);
        let z = Addr(quat + 8).read_fpu(c);
        let w = Addr(quat + 0xc).read_fpu(c);
        set_fr(c, 2, x);
        set_fr(c, 3, y);
        let xx = x * x;
        let yy = y * y;
        let zy = z * y;
        c.lq(V0, quat);
        let wx = w * x;
        set_fr(c, 8, wx);
        Addr(s + 0x1c).sw(c, 0);
        set_fr(c, 6, yy);
        c.sq(V0, s);
        let zx = z * x;
        let wy = w * y;
        set_fr(c, 5, wy);
        let sq2 = (xx + yy) + (xx + yy);
        set_fr(c, 4, sq2);
        let ay = zy - wx;
        let ay = ay + ay;
        let az = FpuF32::ONE - sq2;
        let ax = zx + wy;
        let ax = ax + ax;
        set_fr(c, 1, ay);
        set_fr(c, 7, az);
        set_fr(c, 0, ax);
        Addr(s + 0x14).write_fpu(c, ay);
        Addr(s + 0x18).write_fpu(c, az);
        Addr(s + 0x10).write_fpu(c, ax);
        // target (sp+0) = the rotation of that axis onto `up`.
        call_ptr(c, QUAT_FROM_TWO_VECTORS, 0x0012_1b50)?;

        // 0x00121b50: is `heading` the zero vector?
        c.g[V0][0] = 0x50_0000;
        c.g[V1][0] = sx(ZERO_VEC4);
        let lane = |c: &mut Cpu, i: u32, swap: bool| {
            let h = Addr(heading + 4 * i).read_fpu(c);
            let k = Addr(ZERO_VEC4 + 4 * i).read_fpu(c);
            if swap {
                set_fr(c, 1, k);
                set_fr(c, 0, h);
                fcc_eq(c, h, k)
            } else {
                set_fr(c, 1, h);
                set_fr(c, 0, k);
                fcc_eq(c, h, k)
            }
        };
        let eq0 = lane(c, 0, false);
        c.g[V0][0] = 0;
        let differs = if !eq0 {
            c.cycles += 1;
            true
        } else {
            c.cycles += 6;
            if !lane(c, 1, false) {
                true
            } else {
                c.cycles += 6;
                if !lane(c, 2, false) {
                    true
                } else {
                    c.cycles += 5;
                    if lane(c, 3, true) {
                        false
                    } else {
                        c.cycles += 1;
                        true
                    }
                }
            }
        };
        if differs {
            c.g[V0][0] = 1;
        }

        // 0x00121bb8
        c.cycles += 2;
        if differs {
            // The heading in the target frame (vf8 = conj-rotated heading).
            c.cycles += 21;
            c.lqc2(4, s);
            c.lqc2(5, heading);
            {
                let v = c.vu0();
                v.sub(0b0001, 8, 8, Src::Vec(8));
                v.opmula(4, 5);
                v.opmsub(6, 5, 4);
                v.opmula(4, 6);
                v.opmsub(7, 6, 4);
                v.mula(0b1110, 5, Src::Bc(0, W));
                v.msuba(0b1110, 6, Src::Bc(4, W));
                v.msuba(0b1110, 6, Src::Bc(4, W));
                v.madda(0b1110, 7, Src::Bc(0, W));
                v.madd(0b1110, 8, 7, Src::Bc(0, W));
            }
            c.sqc2(8, s + 0x20);
            set_fr(c, 22, FpuF32::ZERO);
            c.lq(V0, s + 0x20);
            c.sq(V0, s + 0x10);
            let hy = Addr(s + 0x14).read_fpu(c);
            set_fr(c, 20, hy);
            let y_zero = fcc_eq(c, hy, FpuF32::ZERO);
            let hx = Addr(s + 0x10).read_fpu(c);
            set_fr(c, 21, hx);
            // f0 = -yaw
            if !y_zero {
                // 0x00121c40: atan(hx / hy), moved into the right quadrant.
                c.cycles += 7;
                set_fr(c, 12, hx.div(hy));
                call_ptr(c, MATH_ATAN, 0x0012_1c4c)?;
                let negative = fcc_lt(c, hy, FpuF32::ZERO);
                c.f[12] = c.f[0];
                if negative {
                    c.cycles += 3;
                    if fcc_lt(c, FpuF32::ZERO, hx) {
                        c.cycles += 3;
                        let pi = gp_lit(c, GP_PI_A);
                        set_fr(c, 0, pi);
                        set_fr(c, 12, fr(c, 12) + pi);
                    } else {
                        c.cycles += 2;
                        let pi = gp_lit(c, GP_PI_B);
                        set_fr(c, 0, pi);
                        set_fr(c, 12, fr(c, 12) - pi);
                    }
                }
                c.cycles += 1;
                c.f[0] = c.f[12];
                c.cycles += 1;
                set_fr(c, 0, -fr(c, 0));
            } else {
                c.cycles += 4;
                let x_zero = fcc_eq(c, hx, FpuF32::ZERO);
                set_fr(c, 0, hx);
                if x_zero {
                    c.cycles += 1;
                    set_fr(c, 0, -hx);
                } else {
                    c.cycles += 4;
                    let positive = fcc_le(c, FpuF32::ZERO, hx);
                    let k = gp_lit(c, GP_NEG_HALF_PI);
                    if !positive {
                        set_fr(c, 0, -k);
                    } else {
                        c.cycles += 3;
                        let k = gp_lit(c, GP_HALF_PI_A);
                        set_fr(c, 0, -k);
                    }
                }
            }

            // 0x00121c84: spin the target about `up` by the yaw.
            c.cycles += 34;
            c.g[AT][0] = 0x3f00_0000;
            c.g[A0][0] = sx(s + 0x60);
            c.g[A1][0] = sx(s + 0x64);
            set_fr(c, 12, fr(c, 0) * FpuF32(0x3f00_0000));
            call_ptr(c, MATH_SIN_COS, 0x0012_1c9c)?;
            let ux = Addr(up).read_fpu(c);
            let uy = Addr(up + 4).read_fpu(c);
            let uz = Addr(up + 8).read_fpu(c);
            let sin = Addr(s + 0x60).read_fpu(c);
            let cos = Addr(s + 0x64).read_fpu(c);
            set_fr(c, 4, uy);
            set_fr(c, 2, cos);
            let (qx, qy, qz) = (sin * ux, sin * uy, sin * uz);
            set_fr(c, 3, qz);
            set_fr(c, 1, qx);
            set_fr(c, 0, qy);
            Addr(s + 0x4c).write_fpu(c, cos);
            Addr(s + 0x48).write_fpu(c, qz);
            Addr(s + 0x40).write_fpu(c, qx);
            Addr(s + 0x44).write_fpu(c, qy);
            c.lqc2(4, s + 0x40);
            c.lqc2(5, s);
            quat_mul_vu0(c);
            c.sqc2(8, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s);
            c.sq(V0, s + 0x30);
        }
        c.f[0] = Addr(quat).read_u32(c);

        // 0x00121d0c: delta = target * conj(current) (sp+0x10).
        c.cycles += 31;
        let qx = fr(c, 0);
        let qy = Addr(quat + 4).read_fpu(c);
        let qz = Addr(quat + 8).read_fpu(c);
        let qw = Addr(quat + 0xc).read_fpu(c);
        set_fr(c, 0, -qx);
        set_fr(c, 1, -qy);
        set_fr(c, 3, qw);
        set_fr(c, 2, -qz);
        Addr(s + 0x30).write_fpu(c, -qx);
        Addr(s + 0x3c).write_fpu(c, qw);
        Addr(s + 0x34).write_fpu(c, -qy);
        Addr(s + 0x38).write_fpu(c, -qz);
        set_fr(c, 21, FpuF32::ZERO);
        c.lq(V0, s + 0x30);
        c.sq(V0, s + 0x20);
        c.lqc2(4, s);
        c.lqc2(5, s + 0x20);
        quat_mul_vu0(c);
        c.sqc2(8, s + 0x30);
        c.lq(V0, s + 0x30);
        c.sq(V0, s + 0x10);
        let dw = Addr(s + 0x1c).read_fpu(c);
        set_fr(c, 0, dw);

        // Its angle (f20 = 2 acos |w|) and half-angle sign (f12 = +-0.5).
        let has_axis = if fcc_lt(c, FpuF32::ZERO, dw) {
            c.cycles += 12;
            c.g[AT][0] = 0x3f80_0000;
            set_fr(c, 12, dw.min(FpuF32::ONE));
            call_ptr(c, MATH_ASIN, 0x0012_1d98)?;
            let half_pi = gp_lit(c, GP_HALF_PI_B);
            let eps = gp_lit(c, GP_EPS_A);
            let d = half_pi - fr(c, 0);
            set_fr(c, 1, d);
            let angle = d + d;
            set_fr(c, 20, angle);
            let big = fcc_le(c, eps, angle);
            c.f[2] = Addr(s + 0x10).read_u32(c);
            if big {
                c.cycles += 4;
                c.g[AT][0] = 0x3f00_0000;
                set_fr(c, 12, FpuF32(0x3f00_0000));
                c.f[0] = Addr(s + 0x14).read_u32(c);
            }
            big
        } else {
            c.cycles += 13;
            c.g[AT][0] = sx(0xbf80_0000);
            set_fr(c, 12, -dw.max(FpuF32(0xbf80_0000)));
            call_ptr(c, MATH_ASIN, 0x0012_1ddc)?;
            let half_pi = gp_lit(c, GP_HALF_PI_C);
            let eps = gp_lit(c, GP_EPS_B);
            let d = half_pi - fr(c, 0);
            set_fr(c, 1, d);
            let angle = d + d;
            set_fr(c, 20, angle);
            let big = fcc_le(c, eps, angle);
            c.f[2] = Addr(s + 0x10).read_u32(c);
            if big {
                c.cycles += 3;
                c.g[AT][0] = sx(0xbf00_0000);
                set_fr(c, 12, FpuF32(0xbf00_0000));
                c.f[0] = Addr(s + 0x14).read_u32(c);
            }
            big
        };
        if has_axis {
            // 0x00121e08: axis = delta.xyz / sin(+-angle/2) (sp+0x20).
            c.cycles += 18;
            set_fr(c, 12, fr(c, 20) * fr(c, 12));
            c.f[1] = Addr(s + 0x18).read_u32(c);
            Addr(s + 0x40).write_fpu(c, fr(c, 2));
            Addr(s + 0x44).write_fpu(c, fr(c, 0));
            Addr(s + 0x48).write_fpu(c, fr(c, 1));
            Addr(s + 0x4c).write_fpu(c, fr(c, 21));
            call_ptr(c, MATH_SIN, 0x0012_1e24)?;
            c.mfc1(V1, 0);
            c.qmtc2(V1, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, s + 0x40);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s + 0x20);
            c.sq(V0, s + 0x30);
        } else {
            // 0x00121e50: no rotation to speak of, the reference axis.
            c.cycles += 4;
            c.g[V0][0] = sx(REF_AXIS);
            c.lq(V1, REF_AXIS);
            c.sq(V1, s + 0x20);
        }

        // 0x00121e60: turn by min(maxTurn, angle * rate) per second.
        c.cycles += 52;
        let step = gp_lit(c, GP_TICK_STEP);
        set_fr(c, 1, step);
        c.g[AT][0] = 0x3f00_0000;
        c.g[A0][0] = sx(s + 0x68);
        c.g[A1][0] = sx(s + 0x6c);
        let turn = fr(c, 23).min(fr(c, 20) * fr(c, 24));
        set_fr(c, 0, turn);
        let angle = turn * step;
        set_fr(c, 20, angle);
        set_fr(c, 12, angle * FpuF32(0x3f00_0000));
        call_ptr(c, MATH_SIN_COS, 0x0012_1e88)?;
        let sin = Addr(s + 0x68).read_fpu(c);
        c.g[A0][0] = c.g[S1][0];
        let ax = Addr(s + 0x20).read_fpu(c);
        let ay = Addr(s + 0x24).read_fpu(c);
        let az = Addr(s + 0x28).read_fpu(c);
        set_fr(c, 4, az);
        let (rx, ry, rz) = (sin * ax, sin * ay, sin * az);
        set_fr(c, 1, rx);
        set_fr(c, 2, ry);
        let cos = Addr(s + 0x6c).read_fpu(c);
        set_fr(c, 3, cos);
        set_fr(c, 0, rz);
        Addr(s + 0x3c).write_fpu(c, cos);
        Addr(s + 0x30).write_fpu(c, rx);
        Addr(s + 0x34).write_fpu(c, ry);
        Addr(s + 0x38).write_fpu(c, rz);
        c.lq(V0, s + 0x30);
        c.sq(V0, s + 0x10);
        // orient = step * orient
        c.lqc2(4, s + 0x10);
        c.lqc2(5, quat);
        quat_mul_vu0(c);
        c.sqc2(8, s + 0x40);
        c.lq(V0, s + 0x40);
        c.sq(V0, quat);
        c.sq(V0, s + 0x30);
        RIDER_BUILD_WORLD_MATRIX.call(c, 0x0012_1f04)?;

        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 24, 0xd0);
        frame.restore_fpr(c, 23, 0xc8);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WorldLightMan_GetInstanceLighting (0x002f5400)
// ---------------------------------------------------------------------------

/// `WorldLightMan` fields: the world, the instance nodes (32 of 0x180
/// bytes), the 32 hash heads and the node count.
const WLM_WORLD: u32 = 0x4;
const WLM_NODES: u32 = 0x210;
const WLM_NODE_SIZE: u32 = 0x180;
const WLM_MAX_NODES: u64 = 0x20;
const WLM_HEADS: u32 = 0x3210;
const WLM_COUNT: u32 = 0x3290;
/// Instance node fields: id, priority, current lighting (0xa0 bytes),
/// position, overlap region, target lighting (0xa0 bytes), hash links.
const WLN_ID: u32 = 0x0;
const WLN_PRIORITY: u32 = 0x4;
const WLN_CUR: u32 = 0x10;
const WLN_POS: u32 = 0xb0;
const WLN_BOX: u32 = 0xc0;
const WLN_TARGET: u32 = 0xd0;
const WLN_NEXT: u32 = 0x170;
const WLN_PREV: u32 = 0x174;
/// Overlap region fields: id, priority, bounds (min, max), cell.
const WLB_ID: u32 = 0x0;
const WLB_PRIORITY: u32 = 0x4;
const WLB_MIN: u32 = 0x10;
const WLB_MAX: u32 = 0x20;
const WLB_CELL: u32 = 0x30;
/// Instance fields: the model (0 for a plain instance), the position, the
/// lighting key.
const INST_MODEL: u32 = 0xc;
const INST_POS: u32 = 0x40;
const INST_KEY: u32 = 0x78;
/// gp-relative globals: the frame state (`+0x18` the priority stamp), the
/// irradiance override flag, the default irradiance index; `.lit4` -99999.
const GP_FRAME_STATE: u32 = 0x2a74;
const GP_IRRADIANCE_OVERRIDE: u32 = 0xa74;
const GP_DEFAULT_IRRADIANCE: u32 = 0x12d4;
const GP_NEG_99999: u32 = 0x38b4;
/// The irradiance table and the "resolved" tag of an irradiance reference.
const IRRADIANCE_TABLE: u32 = 0x0050_47f8;
const IRRADIANCE_RESOLVED: u64 = 0x12_3400;

const WLM_EVICT_OLDEST: Callee = Callee {
    addr: 0x002f_59d0,
    f: ee::s09::f_002f59d0,
};
const WLM_GET_OVERLAP_REGION: Callee = Callee {
    addr: 0x002f_5a70,
    f: ee::s09::f_002f5a70,
};
const IRRADIANCE_GET_OVERRIDE: Callee = Callee {
    addr: 0x002e_e2f8,
    f: ee::s09::f_002ee2f8,
};
const WLM_ADD_POINT_LIGHTS: Callee = Callee {
    addr: 0x002f_5af0,
    f: ee::s09::f_002f5af0,
};
const INSTANCE_GET_NODE_MATRIX: u32 = 0x0034_fed8;
const OVERLAP_LIST_GATHER_TYPE2: u32 = 0x0033_1450;
const IRRADIANCE_FIND_INDEX: u32 = 0x0038_ac50;
const IRRADIANCE_GET_ENTRY: u32 = 0x0038_abf8;

/// Copy a lighting set (0xa0 bytes, 0x20 per step) from `v0` to `v1` up
/// to `end` (in `a0` or `s7`), with `lq`/`sq` through `t0`/`t1` (GPR
/// numbers `lo`, `lo + 1`); back-edge event checks at `head`.
fn copy_lighting(c: &mut Cpu, end_reg: usize, lo: usize, head: u32) {
    loop {
        c.cycles += 7;
        let src = r32(c, V0);
        let dst = r32(c, V1);
        c.lq(lo, src);
        c.lq(lo + 1, src + 0x10);
        c.sq(lo, dst);
        c.sq(lo + 1, dst + 0x10);
        c.g[V0][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V0][0] != c.g[end_reg][0];
        c.g[V1][0] = sx(dst.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The instance's position into `sp+0` (and `sp+0x10` / `sp+0x50`): the
/// model node matrix's translation (`v0`, the model, non-zero; `a0` set by
/// the caller), or the plain instance position.
fn instance_position(
    c: &mut Cpu,
    s: u32,
    matrix_cycles: u64,
    plain_cycles: u64,
    ret: u32,
) -> crate::call::Flow {
    let obj = r32(c, S0);
    if c.g[V0][0] != 0 {
        c.cycles += matrix_cycles;
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(s + 0x10);
        call_ptr(c, INSTANCE_GET_NODE_MATRIX, ret)?;
        c.lq(V0, s + 0x40);
        c.sq(V0, s);
        c.sq(V0, s + 0x50);
    } else {
        c.cycles += plain_cycles;
        c.lq(V0, obj + INST_POS);
        c.sq(V0, s);
        c.sq(V0, s + 0x10);
    }
    Ok(())
}

/// Resolve the irradiance reference in `s0` (tagging it resolved), fetch
/// its entry and copy it into the node's target (`s4`).
fn resolve_irradiance(c: &mut Cpu, ret_find: u32, ret_get: u32, head: u32) -> crate::call::Flow {
    c.cycles += 4;
    let r = r32(c, S0);
    c.g[V0][0] = Addr(r + 4).lw(c);
    c.g[S1][0] |= 0x3400;
    c.g[S5][0] = 0x50_0000;
    if c.g[V0][0] != c.g[S1][0] {
        c.cycles += 5;
        c.g[A0][0] = sx(IRRADIANCE_TABLE);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, IRRADIANCE_FIND_INDEX, ret_find)?;
        Addr(r).sw(c, c.g[V0][0]);
        Addr(r + 4).sw(c, c.g[S1][0]);
    }
    c.cycles += 5;
    c.g[A1][0] = Addr(r).lw(c);
    c.g[A0][0] = sx(IRRADIANCE_TABLE);
    call_ptr(c, IRRADIANCE_GET_ENTRY, ret_get)?;
    c.g[V1][0] = c.g[S4][0];
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
    copy_lighting(c, A0, A1, head);
    c.cycles += 2;
    c.g[V0][0] = c.g[S4][0];
    Ok(())
}

/// `Lighting* WorldLightMan_GetInstanceLighting(WorldLightMan* self,
/// Instance* inst, int force)` (0x002f5400): the lighting set for a world
/// instance, cached per instance key in 32 hashed nodes. A new instance
/// takes a free node (or evicts the oldest) and is lit from scratch; a
/// cached one is re-lit only when `force` is set and it moved more than 10
/// units (its irradiance reloaded past 500). Re-lighting refreshes the
/// node's overlap region (a fresh one, or rebuilt as a 1000-unit box when
/// the position left it), reloads the irradiance (override, the world's
/// two irradiance references, or the default entry), and adds the point
/// lights to the copy. Returns `&node->cur`.
pub fn world_light_man_get_instance_lighting(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xe0);
        c.g[S1][0] = 0;
        frame.save(c, S5, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0xb0);
        frame.save(c, S2, 0xf0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S4, 0xd0);
        frame.save(c, S7, 0xa0);
        frame.save(c, FP, 0x90);
        frame.save_ra(c, 0x80);
        let s = sp(c);
        let obj = r32(c, S0);
        let this = r32(c, S3);
        let gp = r32(c, GP);

        // The instance's hash chain.
        let key = Addr(obj + INST_KEY).lw(c);
        c.g[A0][0] = key;
        c.g[V1][0] = sx((key as u32) >> 8);
        c.g[S5][0] = c.g[V1][0] & 0x1f;
        let slot = r32(c, S5);
        c.g[V0][0] = sx(this.wrapping_add(slot << 2));
        c.g[S2][0] = Addr(r32(c, V0) + WLM_HEADS).lw(c);
        c.g[A1][0] = Addr(gp + GP_FRAME_STATE).lw(c);
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2) + WLN_ID).lw(c);
            if c.g[V0][0] == c.g[A0][0] {
                c.g[S1][0] = 1;
            } else {
                c.cycles += 2;
                c.g[S2][0] = Addr(r32(c, S2) + WLN_NEXT).lw(c);
                // 0x002f5470
                loop {
                    c.cycles += 2;
                    if c.g[S2][0] == 0 {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, S2) + WLN_ID).lw(c);
                    c.g[V0][0] = Addr(obj + INST_KEY).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S2][0] = Addr(r32(c, S2) + WLN_NEXT).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_5470);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = 1;
                    break;
                }
            }
        }

        // 0x002f548c: a new node for a new instance.
        c.cycles += 2;
        c.g[S4][0] = Addr(r32(c, A1) + 0x18).lw(c);
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(this + WLM_COUNT).lw(c);
            let fits = c.g[A0][0] < WLM_MAX_NODES;
            c.g[V0][0] = WLM_NODE_SIZE as u64;
            if fits {
                c.cycles += 6;
                let count = r32(c, A0);
                c.g[V1][0] = sx(count.wrapping_add(1));
                let off = abi::mult(c, count, WLM_NODE_SIZE);
                c.g[V0][0] = sx(off);
                Addr(this + WLM_COUNT).sw(c, c.g[V1][0]);
                let off = off.wrapping_add(WLM_NODES);
                c.g[V0][0] = sx(off);
                c.g[S2][0] = sx(this.wrapping_add(off));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S4][0];
                WLM_EVICT_OLDEST.call(c, 0x002f_54c8)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // 0x002f54cc: link it at the head of its chain.
            c.cycles += 9;
            let node = r32(c, S2);
            c.g[A0][0] = Addr(obj + INST_KEY).lw(c);
            c.g[V1][0] = sx(slot << 2);
            Addr(node + WLN_ID).sw(c, c.g[A0][0]);
            let head = this.wrapping_add(WLM_HEADS).wrapping_add(slot << 2);
            c.g[V0][0] = sx(head);
            c.g[A0][0] = Addr(head).lw(c);
            Addr(node + WLN_PREV).sw(c, 0);
            Addr(node + WLN_NEXT).sw(c, c.g[A0][0]);
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                Addr(r32(c, A0) + WLN_PREV).sw(c, c.g[S2][0]);
            }
            c.cycles += 1;
            Addr(head).sw(c, c.g[S2][0]);
        }

        // 0x002f54f8: stamp it; moved (s4) / reload (s5).
        c.cycles += 4;
        let node = r32(c, S2);
        Addr(node + WLN_PRIORITY).sw(c, c.g[S4][0]);
        c.g[S5][0] = 0;
        c.g[S4][0] = 0;
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(obj + INST_MODEL).lw(c);
            c.g[S5][0] = 1;
            c.g[S4][0] = 1;
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
            }
            instance_position(c, s, 0, 4, 0x002f_5528)?;
        } else {
            c.cycles += 2;
            if c.g[S6][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(obj + INST_MODEL).lw(c);
                c.g[A0][0] = c.g[S0][0];
                instance_position(c, s, 7, 3, 0x002f_5568)?;
                // 0x002f5584: how far it moved since it was lit.
                c.cycles += 22;
                c.lqc2(3, s);
                c.lqc2(4, node + WLN_POS);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x20);
                c.g[AT][0] = 0x4120_0000;
                set_fr(c, 0, FpuF32(0x4120_0000));
                c.lq(V0, s + 0x20);
                c.sq(V0, s + 0x10);
                c.lqc2(3, s + 0x10);
                {
                    let v = c.vu0();
                    v.add(0b1000, 6, 0, Src::Bc(0, W));
                    v.mul(0b1111, 4, 3, Src::Vec(3));
                    v.adda(0b1000, 4, Src::Bc(4, Y));
                    v.madda(0b1000, 6, Src::Bc(4, Z));
                    v.madd(0b1000, 4, 6, Src::Bc(4, W));
                    v.sqrt(4, 0);
                    v.waitq();
                }
                c.cfc2(V1, 22);
                c.mtc1(V1, 1);
                let dist = fr(c, 1);
                if fcc_lt(c, FpuF32(0x4120_0000), dist) {
                    c.g[S4][0] = 1;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x43fa_0000;
                set_fr(c, 0, FpuF32(0x43fa_0000));
                if fcc_lt(c, FpuF32(0x43fa_0000), dist) {
                    c.g[S5][0] = 1;
                }
            }
        }

        // 0x002f55f4
        c.cycles += 2;
        c.g[S6][0] = sx(node + WLN_CUR);
        if c.g[S4][0] != 0 {
            // The node's overlap region: its own, or a fresh one.
            c.cycles += 5;
            c.lq(V0, s);
            c.g[A0][0] = 0;
            c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
            c.sq(V0, node + WLN_POS);
            let mut own = false;
            if c.g[V1][0] != 0 {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, V1) + WLB_ID).lw(c);
                c.g[V0][0] = Addr(node + WLN_ID).lw(c);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
                    own = true;
                }
            }
            if !own {
                c.cycles += 8;
                c.g[A1][0] = Addr(node + WLN_PRIORITY).lw(c);
                c.g[A0][0] = c.g[S3][0];
                WLM_GET_OVERLAP_REGION.call(c, 0x002f_562c)?;
                c.g[V1][0] = Addr(node + WLN_ID).lw(c);
                c.g[A0][0] = 1;
                Addr(node + WLN_BOX).sw(c, c.g[V0][0]);
                Addr(r32(c, V0) + WLB_ID).sw(c, c.g[V1][0]);
                c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
            }
            // 0x002f5640
            c.cycles += 3;
            c.g[V0][0] = Addr(node + WLN_PRIORITY).lw(c);
            Addr(r32(c, V1) + WLB_PRIORITY).sw(c, c.g[V0][0]);
            let mut rebuild = c.g[A0][0] != 0;
            if !rebuild {
                // 0x002f5658: is the position strictly inside the region's
                // bounds (v1)? Failing x-min or z-max, or passing, goes
                // through 0x002f56d4; v0 ends 1 either way.
                let bx = Addr(node + WLN_BOX).lw(c);
                c.g[V0][0] = bx;
                let bx = bx as u32;
                c.g[V1][0] = 0;
                let via_d4 = 'test: {
                    for i in 0..3u32 {
                        c.cycles += if i == 0 { 7 } else { 6 };
                        let lo = Addr(bx + WLB_MIN + 4 * i).read_fpu(c);
                        let pos = Addr(node + WLN_POS + 4 * i).read_fpu(c);
                        set_fr(c, 0, lo);
                        set_fr(c, 1, pos);
                        if !fcc_lt(c, lo, pos) {
                            if i == 0 {
                                break 'test true;
                            }
                            c.g[V0][0] = 1;
                            break 'test false;
                        }
                        c.cycles += 5;
                        let hi = Addr(bx + WLB_MAX + 4 * i).read_fpu(c);
                        set_fr(c, 0, hi);
                        if !fcc_lt(c, pos, hi) {
                            if i == 2 {
                                break 'test true;
                            }
                            c.g[V0][0] = 1;
                            break 'test false;
                        }
                    }
                    c.g[V1][0] = 1;
                    true
                };
                if via_d4 {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
                // 0x002f56d8
                c.cycles += 3;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                c.g[S4][0] = sx(node + WLN_TARGET);
                rebuild = c.g[A0][0] != 0;
            }
            if rebuild {
                // 0x002f56e4: a 1000-unit box around the position, its
                // overlap list gathered again.
                c.cycles += 34;
                c.g[AT][0] = 0x447a_0000;
                let k = FpuF32(0x447a_0000);
                set_fr(c, 0, k);
                c.g[A0][0] = Addr(node + WLN_BOX).lw(c);
                Addr(s + 0x40).write_fpu(c, k);
                Addr(s + 0x44).write_fpu(c, k);
                Addr(s + 0x48).write_fpu(c, k);
                Addr(s + 0x4c).sw(c, 0);
                c.lqc2(3, node + WLN_POS);
                c.lqc2(4, s + 0x40);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x50);
                c.lq(V0, s + 0x50);
                Addr(s + 0x68).write_fpu(c, k);
                c.sq(V0, s + 0x30);
                Addr(s + 0x60).write_fpu(c, k);
                Addr(s + 0x64).write_fpu(c, k);
                Addr(s + 0x6c).sw(c, 0);
                c.lqc2(3, node + WLN_POS);
                c.lqc2(4, s + 0x60);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x70);
                c.lq(V0, s + 0x70);
                c.lq(V1, s + 0x30);
                c.sq(V0, s + 0x20);
                c.sq(V0, s + 0x50);
                c.sq(V1, s + 0x10);
                let bx = r32(c, A0);
                c.sq(V1, bx + WLB_MIN);
                c.lq(V0, s + 0x20);
                c.sq(V0, bx + WLB_MAX);
                let bx = Addr(node + WLN_BOX).lw(c) as u32;
                c.g[A1][0] = sx(bx);
                c.g[A0][0] = Addr(bx + WLB_CELL).lw(c);
                c.g[A1][0] = sx(bx + WLB_MIN);
                call_ptr(c, OVERLAP_LIST_GATHER_TYPE2, 0x002f_5768)?;
                c.g[S4][0] = sx(node + WLN_TARGET);
            }

            // 0x002f576c: reload the target irradiance.
            c.cycles += 4;
            c.g[S6][0] = sx(node + WLN_CUR);
            c.g[S7][0] = sx(node + WLN_NEXT);
            c.g[FP][0] = sx(node + WLN_POS);
            if c.g[S5][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S4][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp + GP_IRRADIANCE_OVERRIDE).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    IRRADIANCE_GET_OVERRIDE.call(c, 0x002f_5790)?;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
                    copy_lighting(c, A0, A1, 0x002f_5798);
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S4][0];
                } else {
                    c.g[A1][0] = Addr(this + WLM_WORLD).lw(c);
                    // 0x002f57bc: the world samples the ground light under
                    // the position, then its irradiance references.
                    c.cycles += 19;
                    let world = r32(c, A1);
                    c.f[12] = Addr(node + WLN_POS).read_u32(c);
                    c.g[V1][0] = Addr(world + 4).lw(c);
                    let vt = r32(c, V1);
                    c.f[13] = Addr(node + WLN_POS + 4).read_u32(c);
                    c.g[A0][0] = Addr(vt + 0x10).lh(c);
                    c.g[V0][0] = Addr(vt + 0x14).lw(c);
                    c.f[14] = gp_lit(c, GP_NEG_99999).0;
                    c.g[A0][0] = sx(world.wrapping_add(r32(c, A0)));
                    call_ptr(c, r32(c, V0), 0x002f_57dc)?;
                    c.g[V1][0] = Addr(this + WLM_WORLD).lw(c);
                    c.g[A2][0] = Addr(r32(c, V1)).lw(c);
                    c.g[A1][0] = Addr(r32(c, A2) + 4).lw(c);
                    c.g[A0][0] = Addr(r32(c, A1) + 0x140).lh(c);
                    c.g[V0][0] = Addr(r32(c, A1) + 0x144).lw(c);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    call_ptr(c, r32(c, V0), 0x002f_57f8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    c.g[S1][0] = 0x12_0000;
                    if c.g[V0][0] != 0 {
                        resolve_irradiance(c, 0x002f_5824, 0x002f_5838, 0x002f_5840)?;
                    } else {
                        // 0x002f5864: the second reference.
                        c.cycles += 11;
                        c.g[V0][0] = Addr(this + WLM_WORLD).lw(c);
                        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A1) + 4).lw(c);
                        c.g[A0][0] = Addr(r32(c, V1) + 0x128).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1) + 0x12c).lw(c);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, r32(c, V0), 0x002f_5880)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        c.g[S1][0] = 0x12_0000;
                        if c.g[V0][0] != 0 {
                            resolve_irradiance(c, 0x002f_58ac, 0x002f_58c0, 0x002f_58c8)?;
                        } else {
                            // 0x002f58ec: the default entry.
                            c.cycles += 6;
                            c.g[A0][0] = 0x50_0000;
                            c.g[A1][0] = Addr(gp + GP_DEFAULT_IRRADIANCE).lw(c);
                            c.g[A0][0] = sx(IRRADIANCE_TABLE);
                            call_ptr(c, IRRADIANCE_GET_ENTRY, 0x002f_58fc)?;
                            c.g[V1][0] = c.g[S4][0];
                            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
                            copy_lighting(c, A0, A1, 0x002f_5904);
                            c.cycles += 1;
                            c.g[V0][0] = c.g[S4][0];
                        }
                    }
                }
            }

            // 0x002f5924: cur = target, plus the point lights.
            c.cycles += 1;
            c.g[V1][0] = c.g[S6][0];
            copy_lighting(c, S7, A0, 0x002f_5928);
            c.cycles += 7;
            c.g[V0][0] = Addr(node + WLN_BOX).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[FP][0];
            c.g[A3][0] = c.g[S6][0];
            c.g[A2][0] = Addr(r32(c, V0) + WLB_CELL).lw(c);
            c.g[T0][0] = 4;
            WLM_ADD_POINT_LIGHTS.call(c, 0x002f_5960)?;
        }

        // 0x002f5960
        c.cycles += 13;
        c.g[V0][0] = c.g[S6][0];
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// OcclusionMan_BuildOccluderPlanes (0x002429b0)
// ---------------------------------------------------------------------------

/// Occluder fields: the four corners (+0x10..+0x40) and the four side
/// planes (+0x60..+0x90, `(n, -eye.n)`).
const OCC_CORNERS: u32 = 0x10;
const OCC_PLANES: u32 = 0x60;

/// `sp + off = v - eye` through VU0 (`sqc2` to `sp+0x40`, copied by `lq`/`sq`
/// to `sp+0x30` and `sp+off`).
fn occ_edge(c: &mut Cpu, s: u32, eye: u32, v: u32, off: u32) {
    c.lqc2(3, v);
    c.lqc2(4, eye);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x40);
    c.lq(V0, s + 0x40);
    c.sq(V0, s + 0x30);
    c.sq(V0, s + off);
}

/// One side plane through the eye and the corners `a`, `b`:
/// `n = normalize((a - eye) x (b - eye))`, `d = -(eye . n)`, stored at
/// `out`. The first plane keeps its values in a different register order.
fn occ_plane(c: &mut Cpu, s: u32, eye: u32, a: u32, b: u32, out: u32, first: bool) {
    occ_edge(c, s, eye, a, 0);
    occ_edge(c, s, eye, b, 0x10);
    c.lqc2(4, s);
    c.lqc2(5, s + 0x10);
    {
        let v = c.vu0();
        v.opmula(4, 5);
        v.opmsub(6, 5, 4);
        v.sub(0b0001, 6, 6, Src::Vec(6));
    }
    c.sqc2(6, s + 0x50);
    c.lq(V0, s + 0x50);
    c.sq(V0, s + 0x40);
    c.lqc2(3, s + 0x40);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(3));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
        v.rsqrt(0, 3, 4, 0);
        v.waitq();
        v.mul(0b1111, 5, 3, Src::Q);
    }
    c.sqc2(5, s + 0x50);
    c.lq(V0, s + 0x50);
    c.sq(V0, s + 0x20);
    c.sq(V0, s + 0x30);
    // eye . n
    c.lqc2(3, eye);
    c.lqc2(5, s + 0x20);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(5));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
    }
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    let d = -fr(c, 0);
    set_fr(c, 0, d);
    Addr(s + 0x2c).write_fpu(c, d);
    let nx = Addr(s + 0x20).read_u32(c);
    let ny = Addr(s + 0x24).read_u32(c);
    let nz = Addr(s + 0x28).read_u32(c);
    c.f[1] = if first { nz } else { nx };
    c.f[2] = ny;
    if !first {
        c.f[3] = nz;
    }
    Addr(out).write_u32(c, nx);
    Addr(out + 4).write_u32(c, ny);
    Addr(out + 8).write_u32(c, nz);
    Addr(out + 0xc).write_fpu(c, d);
}

/// `void OcclusionMan_BuildOccluderPlanes(OcclusionMan* self,
/// const Vec4* eye, Occluder* occ)` (0x002429b0): the occluder's four side
/// planes from the eye through each pair of adjacent corners (normalised
/// on VU0), each `(n, -eye.n)`.
pub fn occlusion_man_build_occluder_planes(c: &mut Cpu) {
    c.cycles += 215;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    let s = sp(c);
    let eye = r32(c, A1);
    let occ = r32(c, A2);
    let corner = |i: u32| occ + OCC_CORNERS + 0x10 * (i % 4);
    for i in 0..4u32 {
        let (a, b) = (corner(i + 1), corner(i));
        occ_plane(c, s, eye, a, b, occ + OCC_PLANES + 0x10 * i, i == 0);
    }
    c.g[SP][0] = sx(s.wrapping_add(0x60));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AirPredictor_Step (0x001139a0)
// ---------------------------------------------------------------------------

/// `.lit4` constants (gp-relative, subtracted): seconds per tick (1/60),
/// the per-tick horizontal drag (-0.2/60), the gravity step falling
/// (-1900/60) and rising (-850/60).
const GP_AIR_STEP: u32 = 0x7c50;
const GP_AIR_DRAG: u32 = 0x7c4c;
const GP_AIR_GRAVITY_FALLING: u32 = 0x7c48;
const GP_AIR_GRAVITY_RISING: u32 = 0x7c44;
/// `AirPredictor.maxSpeed`.
const AIRP_MAX_SPEED: u32 = 0xa8;

/// `vf5 = vf4 * f` with `f` passed through `v0` (`mfc1`/`qmtc2` into vf3).
fn air_scale_vu0(c: &mut Cpu, v: u32, out: u32) {
    c.mfc1(V0, 0);
    c.lqc2(4, v);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// `float AirPredictor_Step(AirPredictor* self, const Vec4* pos,
/// const Vec4* vel, Vec4* outPos, Vec4* outVel)` (0x001139a0): one tick of
/// the airborne path prediction: `outPos = pos + vel / 60`; `outVel = vel`
/// with horizontal drag and gravity (weaker while rising), clamped to the
/// predictor's maximum speed. Returns the new speed (f0).
pub fn air_predictor_step(c: &mut Cpu) {
    c.cycles += 27;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    let s = sp(c);
    let this = r32(c, A0);
    let pos = r32(c, A1);
    let vel = r32(c, A2);
    let out_pos = r32(c, A3);
    let out_vel = r32(c, T0);
    let step = gp_lit(c, GP_AIR_STEP);
    set_fr(c, 0, step);
    air_scale_vu0(c, vel, s + 0x20);
    set_fr(c, 5, FpuF32::ZERO);
    c.lq(V0, s + 0x20);
    let drag = gp_lit(c, GP_AIR_DRAG);
    set_fr(c, 3, drag);
    c.sq(V0, s + 0x10);
    // outPos = pos + vel * step
    c.lqc2(3, pos);
    c.lqc2(4, s + 0x10);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x20);
    let g = gp_lit(c, GP_AIR_GRAVITY_FALLING);
    set_fr(c, 4, g);
    c.lq(V0, s + 0x20);
    c.sq(V0, out_pos);
    c.sq(V0, s);
    let vx = Addr(vel).read_fpu(c);
    let vz = Addr(vel + 8).read_fpu(c);
    let vy = Addr(vel + 4).read_fpu(c);
    set_fr(c, 0, vz);
    set_fr(c, 2, vx * drag);
    let rising = fcc_lt(c, FpuF32::ZERO, vz);
    set_fr(c, 1, vy * drag);
    if rising {
        c.cycles += 1;
        let g = gp_lit(c, GP_AIR_GRAVITY_RISING);
        set_fr(c, 4, g);
    }

    // 0x00113a10: outVel = vel + (drag x, drag y, gravity, 0).
    c.cycles += 25;
    Addr(s + 0x10).write_fpu(c, fr(c, 2));
    Addr(s + 0x14).write_fpu(c, fr(c, 1));
    Addr(s + 0x18).write_fpu(c, fr(c, 4));
    Addr(s + 0x1c).write_fpu(c, fr(c, 5));
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x10);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x20);
    c.lq(V0, s + 0x20);
    c.sq(V0, out_vel);
    c.sq(V0, s);
    // |outVel| (xyzw) on VU0.
    c.lqc2(3, out_vel);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(3));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
        v.sqrt(4, 0);
        v.waitq();
    }
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    let speed = fr(c, 0);
    let max = Addr(this + AIRP_MAX_SPEED).read_fpu(c);
    set_fr(c, 1, max);
    if fcc_lt(c, max, speed) {
        c.cycles += 7;
        set_fr(c, 0, max.div(speed));
        air_scale_vu0(c, out_vel, out_vel);
        set_fr(c, 0, max);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(s.wrapping_add(0x30));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{
        check, check_lohi, fill, put_vec, put_words, scramble, Rng, DATA, GAME_GP,
    };

    /// `InputVM_ReadPadChannel`: every kind (0..=5 and out of range), the
    /// index inside and past the count, words with and without bit 31 (the
    /// unsigned conversion's two paths); kind 5 retries with the default
    /// kind (never 5 again).
    #[test]
    fn input_vm_read_pad_channel_matches() {
        check_lohi(
            ee::s10::f_003252f8__generated,
            input_vm_read_pad_channel,
            300,
            &|c, seed| {
                let mut r = Rng(0x3252_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_52f8;
                put_words(c, PAD_CHANNEL_TABLE, &PAD_CHANNEL_CASES);
                let this = DATA;
                let list = DATA + 0x100;
                let count = 1 + r.next_u32() % 6;
                let def = [0u32, 1, 2, 3, 4, 6, 9][(seed / 7 % 7) as usize];
                c.bus.write32(this + PADVM_DEF_KIND, def);
                c.bus.write32(this + PADVM_LIST, list);
                c.bus.write32(list, count);
                for i in 0..(count * PAD_ENTRY_SIZE / 4 + 8) {
                    let w = match r.next_u32() % 3 {
                        0 => r.next_u32() | 0x8000_0000,
                        1 => r.next_u32() & 0x7fff_ffff,
                        _ => r.next_u32() % 300,
                    };
                    c.bus.write32(list + 4 + i * 4, w);
                }
                c.g[A0][0] = sx(this);
                let idx = match seed % 4 {
                    0 => count,
                    // A negative index passes the signed compare.
                    1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
                    _ => r.next_u32() % count,
                };
                c.g[A1][0] = sx(idx);
                c.g[A2][0] = match seed % 7 {
                    6 => [6u64, 0xffff_ffff_ffff_ffff, 1 << 40][(seed / 7 % 3) as usize],
                    k => k as u64,
                };
            },
        );
    }

    /// `AIFwdDiffCache_Get`: a cache of three nodes on an LRU ring, hashed
    /// into sorted bucket chains (some unused, key -1); the source key is a
    /// cached one (hit: Touch) or a new one (miss: rehash, rebuild through
    /// the VU0 helpers, whose micro program is a stub that ends at once).
    #[test]
    fn ai_fwd_diff_cache_get_matches() {
        use crate::difftest::elf_image;
        check_lohi(
            ee::s10::f_003279d0__generated,
            ai_fwd_diff_cache_get,
            200,
            &|c, seed| {
                let mut r = Rng(0x3279_d000 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0032_79d0;
                // VU0 micro program at 0: NOP / NOP[E], NOP / NOP.
                put_words(
                    c,
                    0x1100_0000,
                    &[0x8000_033c, 0x4000_02ff, 0x8000_033c, 0x0000_02ff],
                );
                let this = DATA;
                let src = DATA + 0x100;
                let out = DATA + 0x300;
                let buckets = DATA + 0x400;
                let nodes = [DATA + 0x1000, DATA + 0x3000, DATA + 0x5000];
                let count = 1 + r.next_u32() % 4;
                c.bus.write32(this + FDC_LRU, nodes[(seed % 3) as usize]);
                c.bus.write32(this + FDC_BUCKETS, buckets);
                c.bus.write32(this + FDC_COUNT, count);
                fill(c, &mut r, src, 0x60);
                // Distinct keys (some unused), the LRU ring 0 -> 1 -> 2 -> 0.
                let mut keys = [0u32; 3];
                for i in 0..3 {
                    keys[i] = if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        (i as u32) * 7 + r.next_u32() % 7
                    };
                    c.bus.write32(nodes[i] + FDN_KEY, keys[i]);
                    c.bus.write32(nodes[i] + FDN_LRU_NEXT, nodes[(i + 1) % 3]);
                    c.bus.write32(nodes[i] + 0x8, nodes[(i + 2) % 3]);
                }
                // Bucket chains, ascending by key.
                for b in 0..count {
                    let mut chain: Vec<usize> = (0..3)
                        .filter(|&i| keys[i] != u32::MAX && keys[i] % count == b)
                        .collect();
                    chain.sort_by_key(|&i| keys[i]);
                    let mut link = buckets + 4 * b;
                    for &i in &chain {
                        c.bus.write32(link, nodes[i]);
                        link = nodes[i] + FDN_HASH_NEXT;
                    }
                    c.bus.write32(link, 0);
                }
                let used: Vec<u32> = keys.iter().copied().filter(|&k| k != u32::MAX).collect();
                let key = if seed % 2 == 0 && !used.is_empty() {
                    used[(r.next_u32() as usize) % used.len()]
                } else {
                    r.next_u32() % 24
                };
                c.bus.write32(src + FD_SRC_KEY, key);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// `Rider_UpdateOrientation`: random unit orientations, up vectors and
    /// headings (zero in each lane combination, so every compare of the
    /// zero-vector test is reached), and a set of "aligned" seeds (identity
    /// orientation, up = +Z, heading on the X axis or zero) that reach the
    /// exact-zero heading paths and the tiny-rotation fallback axis.
    #[test]
    fn rider_update_orientation_matches() {
        use crate::difftest::elf_image;
        fn unit(r: &mut Rng, n: usize) -> [u32; 4] {
            let mut v = [0f32; 4];
            for x in v.iter_mut().take(n) {
                *x = f32::from_bits(r.f32(1.0));
            }
            let len = v.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-3);
            v.map(|x| (x / len).to_bits())
        }
        check_lohi(
            ee::s00::f_00121aa0__generated,
            rider_update_orientation,
            300,
            &|c, seed| {
                let mut r = Rng(0x0121_aa00 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0012_1aa0;
                let rider = DATA;
                let up = DATA + 0x400;
                let heading = DATA + 0x500;
                put_vec(c, ZERO_VEC4, [0; 4]);
                put_vec(c, REF_AXIS, unit(&mut r, 3));
                let aligned = seed % 5 == 0;
                if aligned {
                    put_vec(c, rider + RIDER_QUAT, [0, 0, 0, 0x3f80_0000]);
                    put_vec(c, up, [0, 0, 0x3f80_0000, 0]);
                    let hx = [0u32, 0x3f80_0000, 0xbf80_0000, 0][(seed / 5 % 4) as usize];
                    let hz = if seed / 5 % 4 == 3 { 0 } else { 0x3f00_0000 };
                    put_vec(c, heading, [hx, 0, hz, 0]);
                } else {
                    let q = unit(&mut r, 4);
                    put_vec(c, rider + RIDER_QUAT, q);
                    put_vec(c, up, unit(&mut r, 3));
                    let mut h = unit(&mut r, 4);
                    // Zero lanes: all four (no heading), or a prefix.
                    match seed % 7 {
                        0 => h = [0; 4],
                        1 => h[0] = 0,
                        2 => (h[0], h[1]) = (0, 0),
                        3 => (h[0], h[1], h[2]) = (0, 0, 0),
                        _ => {}
                    }
                    put_vec(c, heading, h);
                }
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(up);
                c.g[A2][0] = sx(heading);
                c.f[12] = r.f32(20.0) & 0x7fff_ffff;
                c.f[13] = r.f32(10.0) & 0x7fff_ffff;
            },
        );
    }

    /// `WorldLightMan_GetInstanceLighting`: a light manager with 0..=32
    /// cached nodes in consistent hash chains, overlap regions sharing one
    /// empty cell (so the gather and the point lights take their short
    /// paths), and the world's lookups as fakes (their irradiance
    /// references forced to fixture records). Seeds cover a cached / new
    /// instance (free node or eviction), a plain or modelled instance
    /// (node matrix through fakes), `force` with small / medium / large
    /// moves, the node's own region inside / outside its bounds or a fresh
    /// region, and every irradiance source (override, first or second
    /// reference resolved or by name, default); some fake calls unwind.
    #[test]
    fn world_light_man_get_instance_lighting_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        const THIS: u32 = 0x0085_0000;
        const CELL: u32 = 0x0087_0000;
        const TREE: u32 = 0x0087_1000;
        const REF1: u32 = 0x0088_0000;
        const REF2: u32 = 0x0088_0100;
        const ENTRIES: u32 = 0x0089_0000;
        const NAMES: u32 = 0x0089_1000;
        const WORLD: u32 = 0x008a_0000;
        const WORLD_VT: u32 = 0x008a_1000;
        const INNER: u32 = 0x008a_2000;
        const INNER_VT: u32 = 0x008a_3000;
        const FRAME_STATE: u32 = 0x008a_4000;
        const OBJ: u32 = 0x008b_0000;
        const MODEL: u32 = 0x008b_1000;
        const MODEL_VT: u32 = 0x008b_2000;
        check(
            ee::s09::f_002f5400__generated,
            world_light_man_get_instance_lighting,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f54, 5);
                elf_image(c);
                c.pc = 0x002f_5400;
                let gp = GAME_GP;
                let fl = |x: f32| x.to_bits();
                let prio = 100 + seed % 50;
                put_words(c, FRAME_STATE + 0x18, &[prio]);
                put_words(c, gp + GP_FRAME_STATE, &[FRAME_STATE]);

                // World lookups: all fakes (delta 0).
                put_words(c, THIS + WLM_WORLD, &[WORLD]);
                put_words(c, WORLD, &[INNER, WORLD_VT]);
                put_words(c, WORLD_VT + 0x10, &[0, FAKE]);
                put_words(c, INNER + 4, &[INNER_VT]);
                put_words(c, INNER_VT + 0x140, &[0, FAKE + 0x10]);
                put_words(c, INNER_VT + 0x128, &[0, FAKE + 0x20]);

                // Irradiance: table of 2 named entries, references.
                put_words(c, IRRADIANCE_TABLE, &[0, 2, ENTRIES, NAMES]);
                fill(c, &mut r, ENTRIES - 0xa0, 3 * 0x28);
                put_words(c, NAMES, &[0x0063_6261, 0, 0x007a_7978, 0]);
                let reference = |c: &mut Cpu, r: &mut Rng, at: u32, k: u32| match k {
                    0 => put_words(c, at, &[0, 0]),
                    1 => put_words(c, at, &[r.next_u32() % 3, IRRADIANCE_RESOLVED as u32]),
                    2 => put_words(c, at, &[0x007a_7978, 0]),
                    _ => put_words(c, at, &[0x0071_7171, 0]),
                };
                let k1 = r.next_u32() % 4;
                let k2 = r.next_u32() % 4;
                reference(c, &mut r, REF1, k1);
                reference(c, &mut r, REF2, k2);
                put_words(
                    c,
                    gp + 0x24c0,
                    &[r.next_u32() % 3, IRRADIANCE_RESOLVED as u32],
                );
                put_words(c, gp + GP_IRRADIANCE_OVERRIDE, &[(seed % 5 == 0) as u32]);
                put_words(c, gp + GP_DEFAULT_IRRADIANCE, &[r.next_u32() % 4]);
                put_words(c, gp - GP_NEG_99999, &[fl(-99999.0)]);

                // The instance.
                let pos = [r.f32(800.0), r.f32(800.0), r.f32(800.0), fl(1.0)];
                let modelled = seed % 3 == 0;
                put_words(c, OBJ + INST_MODEL, &[if modelled { MODEL } else { 0 }]);
                put_words(c, MODEL + 0xc, &[MODEL_VT]);
                put_words(c, MODEL_VT + 0xd0, &[0, FAKE + 0x30]);
                put_words(c, MODEL_VT + 0xe8, &[0, FAKE + 0x40]);
                put_vec(c, OBJ + INST_POS, pos);

                // Overlap regions (some free) sharing the empty cell.
                put_words(c, CELL, &[TREE]);
                for i in 0..8 {
                    let b = THIS + 0x10 + i * 0x40;
                    let id = if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    };
                    put_words(c, b, &[id, r.next_u32() % prio]);
                    put_words(c, b + WLB_CELL, &[CELL]);
                }

                // Cached nodes in consistent chains.
                let count = match seed % 4 {
                    0 => 32,
                    _ => r.next_u32() % 33,
                };
                put_words(c, THIS + WLM_COUNT, &[count]);
                let mut ids = Vec::new();
                for i in 0..count {
                    let n = THIS + WLM_NODES + i * WLM_NODE_SIZE;
                    let id = (r.next_u32() & !0x1f00) | (r.next_u32() % 4) << 8;
                    ids.push(id);
                    put_words(c, n, &[id, r.next_u32() % (prio + 1)]);
                    fill(c, &mut r, n + WLN_CUR, 0x28);
                    fill(c, &mut r, n + WLN_TARGET, 0x28);
                    // Its position: on the instance, moved a little, a lot.
                    let d = [0.0f32, 30.0, 700.0][(r.next_u32() % 3) as usize];
                    let p = [fl(f32::from_bits(pos[0]) + d), pos[1], pos[2], fl(1.0)];
                    put_vec(c, n + WLN_POS, p);
                    // Its region: none, its own (bounds around the instance,
                    // sometimes not containing it), or someone else's.
                    let b = THIS + 0x10 + (r.next_u32() % 8) * 0x40;
                    match r.next_u32() % 4 {
                        0 => put_words(c, n + WLN_BOX, &[0]),
                        1 => {
                            put_words(c, b, &[!id]);
                            put_words(c, n + WLN_BOX, &[b]);
                        }
                        _ => {
                            put_words(c, b, &[id]);
                            put_words(c, n + WLN_BOX, &[b]);
                            let mut lo = [0u32; 4];
                            let mut hi = [0u32; 4];
                            for a in 0..3 {
                                let v = f32::from_bits(pos[a]);
                                let out = r.next_u32() % 8;
                                lo[a] = fl(if out == 0 { v + 5.0 } else { v - 900.0 });
                                hi[a] = fl(if out == 1 { v - 5.0 } else { v + 900.0 });
                            }
                            put_vec(c, b + WLB_MIN, lo);
                            put_vec(c, b + WLB_MAX, hi);
                        }
                    }
                    // Link at the head of its chain.
                    let head = THIS + WLM_HEADS + ((id >> 8) & 0x1f) * 4;
                    let old = c.bus.read32(head);
                    put_words(c, n + WLN_NEXT, &[old, 0]);
                    if old != 0 {
                        put_words(c, old + WLN_PREV, &[n]);
                    }
                    put_words(c, head, &[n]);
                }
                let key = if !ids.is_empty() && seed % 2 == 0 {
                    ids[(r.next_u32() as usize) % ids.len()]
                } else {
                    (r.next_u32() & !0x1f00) | (r.next_u32() % 4) << 8
                };
                put_words(c, OBJ + INST_KEY, &[key]);

                // Fake call results: the references (index after the world
                // query, and the node matrix's two calls when modelled).
                let base = if modelled { 3 } else { 1 };
                put_words(c, LOG_RESULTS + 4 * base, &[REF1, REF2]);

                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = (seed % 7 != 0) as u64;
            },
        );
    }

    /// `OcclusionMan_BuildOccluderPlanes`: random eyes and corners (some
    /// degenerate: a corner on the eye, two corners equal).
    #[test]
    fn occlusion_man_build_occluder_planes_matches() {
        check(
            ee::s06::f_002429b0__generated,
            occlusion_man_build_occluder_planes,
            300,
            &|c, seed| {
                let mut r = Rng(0x0024_29b0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0024_29b0;
                let eye = DATA;
                let occ = DATA + 0x100;
                let v = |r: &mut Rng| [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.edge_f32()];
                let e = v(&mut r);
                put_vec(c, eye, e);
                for i in 0..4 {
                    let mut k = v(&mut r);
                    if seed % 11 == i {
                        k = e;
                    }
                    put_vec(c, occ + OCC_CORNERS + 0x10 * i, k);
                }
                if seed % 13 == 0 {
                    let k = c.bus.read128(occ + OCC_CORNERS);
                    c.bus.write128(occ + OCC_CORNERS + 0x10, k);
                }
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(eye);
                c.g[A2][0] = sx(occ);
            },
        );
    }

    /// `AirPredictor_Step`: random positions and velocities (rising and
    /// falling, edge floats), maximum speeds above and below the result.
    #[test]
    fn air_predictor_step_matches() {
        use crate::difftest::elf_image;
        check(ee::s00::f_001139a0__generated, air_predictor_step, 300, &|c, seed| {
            let mut r = Rng(0x0011_39a0 ^ seed as u64);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x0011_39a0;
            let this = DATA;
            let pos = DATA + 0x100;
            let vel = DATA + 0x110;
            let out_pos = DATA + 0x200;
            let out_vel = DATA + 0x210;
            put_vec(
                c,
                pos,
                [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.f32(2.0)],
            );
            let z = if seed % 9 == 0 {
                r.edge_f32()
            } else {
                r.f32(60.0)
            };
            put_vec(c, vel, [r.f32(60.0), r.f32(60.0), z, r.f32(1.0)]);
            let max = if seed % 2 == 0 {
                r.f32(30.0) & 0x7fff_ffff
            } else {
                0x4479_c000
            };
            put_words(c, this + AIRP_MAX_SPEED, &[max]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(pos);
            c.g[A2][0] = sx(vel);
            c.g[A3][0] = sx(out_pos);
            c.g[T0][0] = sx(out_vel);
        });
    }
}
