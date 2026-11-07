//! Staging wave 211 (oc290): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_ecd8,
    rider_model_sample_part_transform,
    "RiderModel_SamplePartTransform",
)];

// ------------------------------------------- RiderModel_SamplePartTransform

/// Layouts read by `RiderModel_SamplePartTransform`.
pub(crate) mod sample_part_xf {
    use crate::call::Callee;

    /// `gp + 0xd08`: the registered animation databases, indexed by the low
    /// byte of an anim ref's first word.
    pub const ANIM_DBS_GP: u32 = 0xd08;
    /// The identity transform (position, rotation) at `0x500000 - 0xdd0`.
    pub const IDENTITY_XF: u32 = 0x0050_0000 - 0xdd0;
    /// The origin the root position is taken relative to (`0x500000 - 0xed0`).
    pub const ROOT_ORIGIN: u32 = 0x0050_0000 - 0xed0;

    // AnimDataBase: +8 clip table (0x14-byte clips), +0xc chunk descriptors.
    pub const DB_CLIPS: u32 = 0x8;
    pub const DB_CHUNKS: u32 = 0xc;
    pub const CLIP_SIZE: u32 = 0x14;
    // Clip: +4 first chunk descriptor, +8 channel count, +0xa chunks per
    // channel. Chunk descriptor (8 bytes): +4 the part id of its channel.
    pub const CLIP_FIRST_CHUNK: u32 = 0x4;
    pub const CLIP_CHANNELS: u32 = 0x8;
    pub const CLIP_CHUNKS: u32 = 0xa;
    pub const DESC_PART: u32 = 0x4;

    // Anim state (the anim ref): +0 db index | clip << 8, +0x20 root
    // position, +0x30 root rotation (quaternion), +0x40 use-alternate-curves
    // and scale flag.
    pub const ST_ROOT_POS: u32 = 0x20;
    pub const ST_ROOT_ROT: u32 = 0x30;
    pub const ST_ALT: u32 = 0x40;

    // RiderModel: +0xc bones (0x58 bytes), +0x1c bone index per part, +0x20
    // first curve per curve slot.
    pub const MODEL_BONES: u32 = 0xc;
    pub const MODEL_PART_BONE: u32 = 0x1c;
    pub const MODEL_CURVE_BASE: u32 = 0x20;
    pub const BONE_SIZE: u32 = 0x58;
    // Bone: +4 curve slot, +0x38 bone info.
    pub const BONE_SLOT: u32 = 0x4;
    pub const BONE_INFO: u32 = 0x38;
    // Bone info: +0x10 parent (-1: root), +0x16 alternate curve slot offset,
    // +0x18..0x1b rotation axis selectors (0..3 component, 4..7 negated),
    // +0x40 position scale.
    pub const INFO_PARENT: u32 = 0x10;
    pub const INFO_ALT_SLOT: u32 = 0x16;
    pub const INFO_SCALE: u32 = 0x40;

    pub const SAMPLE_CHANNEL: Callee = Callee {
        addr: 0x0031_1318,
        f: ssx3_ee::s10::AnimDataBase_SampleChannel,
    };
}

/// `Xform* RiderModel_SamplePartTransform(Xform* out, RiderModel* model,
/// AnimState* st, int part)` (0x30ecd8): find the channel of the state's
/// clip that animates `part` (the first one whose first chunk descriptor
/// carries that part id) and sample its bone's position (3 curves, w = 1,
/// optionally scaled by the bone's scale) and rotation (3 curves, a unit
/// quaternion with the axes remapped). A root bone (parent -1) is moved
/// into the state's root frame (rotated by the root rotation, offset by the
/// root position). With no matching channel the result is the identity
/// transform. Returns `out` (position at +0, rotation at +0x10).
pub fn rider_model_sample_part_transform(c: &mut Cpu) {
    use sample_part_xf as k;
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x110);
        c.g[T0][0] = 0x14;
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 20, 0x100);
        // s2 = state, s3 = channel index, s4 = out
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = 0;
        c.g[S4][0] = c.g[A0][0];
        let st = Addr(c.g[S2][0] as u32);
        // t2 = the database table, t1 = the clip's channel count, t0 = model
        let dbs = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let word = st.lw(c) as u32;
        c.g[T2][0] = dbs;
        c.g[V0][0] = sx((word & 0xff) << 2);
        c.g[V1][0] = sx(word >> 8);
        c.g[A2][0] = abi::addu(dbs, c.g[V0][0]);
        c.mult(0, V1, V1, T0);
        c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(k::DB_CLIPS).lw(c);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[T1][0] = Addr(c.g[V1][0] as u32).off(k::CLIP_CHANNELS).lhu(c);
        c.g[T0][0] = c.g[A1][0];
        let mut found = false;
        if c.g[T1][0] != 0 {
            c.cycles += 7;
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[20] = FpuF32::ONE.0;
            c.g[S7][0] = 0x50_0000;
            c.g[S5][0] = sx(sp(c).wrapping_add(0x10));
            c.g[S6][0] = u64::MAX;
            c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
            loop {
                if channel_part_matches(c) {
                    sample_matched_part(c)?;
                    found = true;
                    break;
                }
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_ed68);
                }
            }
        }
        if !found {
            // no channel animates the part: the identity transform
            c.cycles += 6;
            c.g[V0][0] = k::IDENTITY_XF as u64;
            c.lq(3, k::IDENTITY_XF + 0x10);
            c.lq(4, k::IDENTITY_XF);
            let out = c.g[S4][0] as u32;
            c.sq(3, out.wrapping_add(0x10));
            c.sq(4, out);
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    });

    /// One step of the channel search (a1 = the state's first word, s3 =
    /// channel): the part id of the channel's first chunk descriptor (in
    /// v1), true when it is `part` (a3).
    fn channel_part_matches(c: &mut Cpu) -> bool {
        c.cycles += 19;
        let word = c.g[A1][0] as u32;
        c.g[V1][0] = k::CLIP_SIZE as u64;
        c.g[V0][0] = sx((word & 0xff) << 2);
        c.g[A1][0] = sx(word >> 8);
        c.mult(0, A1, A1, V1);
        c.g[V0][0] = abi::addu(c.g[T2][0], c.g[V0][0]);
        let db = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = db;
        c.g[A0][0] = Addr(db as u32).off(k::DB_CLIPS).lw(c);
        c.g[A2][0] = Addr(db as u32).off(k::DB_CHUNKS).lw(c);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        let clip = Addr(c.g[A0][0] as u32);
        c.g[V1][0] = clip.off(k::CLIP_CHUNKS).lhu(c);
        c.g[V0][0] = clip.off(k::CLIP_FIRST_CHUNK).lw(c);
        c.mult(0, V1, S3, V1);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[A2][0]);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(k::DESC_PART).lbu(c);
        c.g[V1][0] == c.g[A3][0]
    }

    /// The matching channel's bone: position, rotation, root frame, then
    /// the result into `out`.
    fn sample_matched_part(c: &mut Cpu) -> Flow {
        let base_addr = curve_base_slot(c);
        sample_position(c, base_addr)?;
        if is_root(c) {
            position_to_root_frame(c);
        } else {
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        }
        sample_rotation(c)?;
        c.cycles += 3;
        let parent = Addr(c.g[S0][0] as u32).off(k::INFO_PARENT).lh(c);
        c.g[V0][0] = parent;
        let s = sp(c);
        c.lq(2, s);
        if parent == c.g[S6][0] {
            rotation_to_root_frame(c);
        }
        c.cycles += 4;
        let s = sp(c);
        c.lq(3, s.wrapping_add(0x10));
        let out = c.g[S4][0] as u32;
        c.sq(2, out);
        c.sq(3, out.wrapping_add(0x10));
        Ok(())
    }

    /// The bone of `part` (a3) and its curve slots: s0 = bone info, a3 =
    /// curve slot; returns the address of the curve-base entry to sample
    /// from (the alternate slot `+info.alt` when the state's flag is set),
    /// with a0/a1 = the base table and the slot's byte offset.
    fn curve_base_slot(c: &mut Cpu) -> u32 {
        c.cycles += 12;
        let model = Addr(c.g[T0][0] as u32);
        c.g[V0][0] = model.off(k::MODEL_PART_BONE).lw(c);
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[A1][0] = k::BONE_SIZE as u64;
        c.g[A0][0] = model.off(k::MODEL_BONES).lw(c);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.mult(0, V0, V0, A1);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        let bone = Addr(c.g[A0][0] as u32);
        c.g[S0][0] = bone.off(k::BONE_INFO).lw(c);
        c.g[A3][0] = bone.off(k::BONE_SLOT).lw(c);
        let slot = c.g[A3][0] as u32;
        if c.g[A2][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(k::INFO_ALT_SLOT).lh(c);
            c.g[A1][0] = sx(slot << 2);
            c.g[V1][0] = model.off(k::MODEL_CURVE_BASE).lw(c);
            c.g[V0][0] = abi::addu(c.g[V0][0], slot as u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = c.g[V1][0];
            c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = model.off(k::MODEL_CURVE_BASE).lw(c);
            c.g[V0][0] = sx(slot << 2);
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[V0][0] as u32
    }

    /// `AnimDataBase_SampleChannel(db, out, st, channel, first, 3)` for the
    /// state's database (v0/v1 = its table index and entry, as left by the
    /// original).
    fn sample_curves(c: &mut Cpu, out: u32, first: u64, ret: u32) -> Flow {
        let st = c.g[S2][0] as u32;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A1][0] = sx(out);
        c.g[A2][0] = sx(st);
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = first;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[T1][0] = 3;
        k::SAMPLE_CHANNEL.call(c, ret)
    }

    /// The position: curves `s1 = base[slot'] - base[slot]` onward into
    /// sp+0x20, w = 1 (f20), copied to sp+0 (through sp+0x30), multiplied by
    /// the bone's scale when the state's flag is set.
    fn sample_position(c: &mut Cpu, base_addr: u32) -> Flow {
        c.cycles += 26;
        c.g[S1][0] = Addr(base_addr).lw(c);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        c.g[V1][0] = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let slot_base = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A3][0] = slot_base;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(slot_base as u32));
        let s = sp(c);
        let first = c.g[S1][0];
        sample_curves(c, s.wrapping_add(0x20), first, 0x0030_ee54)?;
        let s = sp(c);
        for i in 0..3 {
            let v = Addr(s + 0x20 + 4 * i).read_fpu(c);
            c.f[i as usize] = v.0;
        }
        for i in 0..3 {
            let v = FpuF32(c.f[i as usize]);
            Addr(s + 0x30 + 4 * i).write_fpu(c, v);
        }
        let w = FpuF32(c.f[20]);
        Addr(s + 0x3c).write_fpu(c, w);
        c.lq(2, s + 0x30);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        c.sq(2, s);
        if c.g[V1][0] != 0 {
            c.cycles += 18;
            let scale = Addr(c.g[S0][0] as u32).off(k::INFO_SCALE);
            let mut sc = [FpuF32::ZERO; 4];
            for i in 0..4 {
                sc[i] = scale.off(4 * i as u32).read_fpu(c);
                c.f[4 + i] = sc[i].0;
            }
            let mut p = [FpuF32::ZERO; 4];
            for i in 0..4 {
                p[i] = Addr(s + 4 * i as u32).read_fpu(c) * sc[i];
            }
            // f1..f3 = x, y, z; f0 = w
            c.f[1] = p[0].0;
            c.f[2] = p[1].0;
            c.f[3] = p[2].0;
            c.f[0] = p[3].0;
            for i in 0..4 {
                Addr(s + 0x30 + 4 * i as u32).write_fpu(c, p[i]);
            }
            c.lq(2, s + 0x30);
            c.sq(2, s);
        }
        Ok(())
    }

    /// `info.parent == -1` (v0 = the parent).
    fn is_root(c: &mut Cpu) -> bool {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(k::INFO_PARENT).lh(c);
        c.g[V0][0] == c.g[S6][0]
    }

    /// vf8.xyz = the vector `vec` rotated by the quaternion `quat`
    /// (q v q* through two cross products), vf8.x zeroed first.
    fn rotate_by_quat(c: &mut Cpu, quat: u32, vec: u32) {
        c.lqc2(4, quat);
        c.lqc2(5, vec);
        let vu = c.vu0();
        vu.sub(0b0001, 8, 8, Src::Vec(8));
        vu.opmula(4, 5);
        vu.opmsub(6, 5, 4);
        vu.opmula(4, 6);
        vu.opmsub(7, 6, 4);
        vu.mula(0b1110, 5, Src::Bc(0, W));
        vu.madda(0b1110, 6, Src::Bc(4, W));
        vu.madda(0b1110, 6, Src::Bc(4, W));
        vu.madda(0b1110, 7, Src::Bc(0, W));
        vu.madd(0b1110, 8, 7, Src::Bc(0, W));
    }

    /// A root bone's position into the state's frame: a direction (w = 0)
    /// is only rotated; a point is taken relative to the origin, rotated
    /// and offset by the root position. The result goes to sp+0 (via
    /// sp+0x30); v0 = the state's db index.
    fn position_to_root_frame(c: &mut Cpu) {
        c.cycles += 5;
        let s = sp(c);
        let w = Addr(s + 0xc).read_fpu(c);
        c.f[1] = w.0;
        c.f[0] = 0;
        let direction = w.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, direction);
        let st = c.g[S2][0] as u32;
        if direction {
            c.cycles += 15;
            rotate_by_quat(c, st + k::ST_ROOT_ROT, s);
            c.sqc2(8, s + 0x40);
            c.lq(2, s + 0x40);
        } else {
            c.cycles += 26;
            c.lqc2(3, s);
            let origin = (c.g[S7][0] as u32).wrapping_sub(0xed0);
            c.lqc2(4, origin);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x60);
            c.lq(2, s + 0x60);
            c.sq(2, s + 0x50);
            rotate_by_quat(c, st + k::ST_ROOT_ROT, s + 0x50);
            c.sqc2(8, s + 0x60);
            c.lq(2, s + 0x60);
            c.sq(2, s + 0x40);
            c.lqc2(3, s + 0x40);
            c.lqc2(4, st + k::ST_ROOT_POS);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x60);
            c.lq(2, s + 0x60);
        }
        c.cycles += 4;
        c.sq(2, s + 0x30);
        c.lq(2, s + 0x30);
        c.sq(2, s);
        c.g[V0][0] = Addr(st).lbu(c);
    }

    /// The rotation: curves `s1 + 3` onward into sp+0x30 (v0 = the state's
    /// db index), completed to a unit quaternion at sp+0x10, its axes
    /// remapped when the state's flag is set.
    fn sample_rotation(c: &mut Cpu) -> Flow {
        c.cycles += 92;
        let first = sx((c.g[S1][0] as u32).wrapping_add(3));
        c.g[V1][0] = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let s = sp(c);
        sample_curves(c, s.wrapping_add(0x30), first, 0x0030_efc4)?;
        quat_from_vector_part(c);
        if c.g[A2][0] != 0 {
            remap_axis(c, 0x1b, 3, true);
            remap_axis(c, 0x18, 2, false);
            remap_axis(c, 0x19, 1, false);
            remap_axis(c, 0x1a, 0, false);
            c.cycles += 6;
            let s = sp(c);
            for (f, off) in [(3usize, 0x4cu32), (2, 0x40), (1, 0x44), (0, 0x48)] {
                let v = FpuF32(c.f[f]);
                Addr(s + off).write_fpu(c, v);
            }
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x10);
        }
        Ok(())
    }

    /// The quaternion from its sampled vector part (sp+0x30..0x38): the
    /// MMI/VU0 sequence the original issues to get w = sqrt(1 - |v|^2)
    /// with the signs restored, written to sp+0x10 as (x, y, z, w) products;
    /// a2 = the state's flag.
    fn quat_from_vector_part(c: &mut Cpu) {
        let s = sp(c);
        // (x, y, z, 0) to sp+0x40 through f0..f2
        for (f, src) in [(1usize, 0x34u32), (2, 0x38), (0, 0x30)] {
            let v = Addr(s + src).read_fpu(c);
            c.f[f] = v.0;
        }
        for (f, dst) in [(1usize, 0x44u32), (2, 0x48), (0, 0x40)] {
            let v = FpuF32(c.f[f]);
            Addr(s + dst).write_fpu(c, v);
        }
        Addr(s + 0x4c).sw(c, 0);
        c.lqc2(3, s + 0x40);
        c.vu0().add(0b1110, 4, 0, Src::Bc(0, W));
        c.vu0().abs(0b1110, 6, 3);
        c.vu0().ftoi(0, 0b1111, 8, 3);
        c.vu0().add(0b1110, 7, 3, Src::Vec(6));
        c.qmfc2(4, 8);
        c.vu0().ftoi(0, 0b1110, 6, 4);
        c.qmfc2(5, 7);
        c.pceqw(5, 5, 0);
        c.paddw(4, 4, 5);
        c.psraw(5, 4, 1);
        c.qmtc2(5, 7);
        c.qmfc2(2, 6);
        c.paddw(2, 2, 4);
        c.vu0().itof(0, 0b1111, 7, 7);
        c.psllw(4, 4, 15);
        c.psllw(4, 4, 15);
        c.pceqw(5, 4, 0);
        c.vu0().suba(0b1110, 3, Src::Vec(7));
        c.vu0().msub(0b1110, 3, 7, Src::Bc(0, W));
        c.pcgtw(4, 4, 0);
        c.por(4, 4, 5);
        c.vu0().suba(0b1110, 3, Src::Bc(0, W));
        c.vu0().msub(0b1110, 5, 4, Src::Bc(0, W));
        c.psllw(2, 2, 15);
        c.psllw(2, 2, 15);
        c.vu0().mul(0b1110, 5, 5, Src::Vec(3));
        c.vu0().sub(0b1110, 9, 0, Src::Vec(5));
        c.vu0().mula(0b1110, 9, Src::Vec(5));
        c.vu0().madd(0b1110, 10, 4, Src::Bc(0, W));
        c.qmfc2(5, 9);
        c.pand(5, 5, 4);
        c.qmfc2(3, 5);
        c.pnor(4, 4, 0);
        c.pand(4, 4, 3);
        c.por(5, 5, 4);
        c.sq(5, s + 0x50);
        c.sqc2(10, s + 0x60);
        for i in 0..3 {
            let v = Addr(s + 0x60 + 4 * i).read_fpu(c).sqrt();
            c.f[0] = v.0;
            Addr(s + 0x60 + 4 * i).write_fpu(c, v);
        }
        c.lqc2(3, s + 0x60);
        c.pceqw(5, 2, 0);
        c.pcgtw(2, 2, 0);
        c.por(2, 2, 5);
        c.vu0().sub(0b1110, 4, 0, Src::Vec(3));
        c.qmfc2(5, 3);
        c.pand(5, 5, 2);
        c.pnor(2, 2, 0);
        c.qmfc2(3, 4);
        c.pand(2, 2, 3);
        c.por(5, 5, 2);
        c.sq(5, s + 0x60);
        let rd = |c: &mut Cpu, off: u32| Addr(s + off).read_fpu(c);
        let f4 = rd(c, 0x50);
        let f0 = rd(c, 0x54);
        let f1 = rd(c, 0x68);
        let f0 = f4 * f0;
        let f5 = rd(c, 0x58);
        let f2 = rd(c, 0x64);
        let f3 = rd(c, 0x60);
        let f1 = f0 * f1;
        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        let f4 = f4 * f2;
        let f0 = f0 * f5;
        c.f[0] = f0.0;
        c.f[1] = f1.0;
        c.f[2] = f2.0;
        c.f[3] = f3.0;
        c.f[4] = f4.0;
        c.f[5] = f5.0;
        Addr(s + 0x10).write_fpu(c, f3);
        Addr(s + 0x14).write_fpu(c, f1);
        Addr(s + 0x18).write_fpu(c, f4);
        Addr(s + 0x1c).write_fpu(c, f0);
    }

    /// One rotation component of the axis remap into `f[dest]`: component
    /// `sel` of sp+0x10 for selector `sel < 4`, else component `sel - 4`
    /// negated (through f0). The first one also sets v1 = sp+0x10 (and
    /// reads through s5, the same address).
    fn remap_axis(c: &mut Cpu, field: u32, dest: usize, first: bool) {
        let info = Addr(c.g[S0][0] as u32);
        c.cycles += 4;
        let sel = info.off(field).lbu(c) as u32;
        let direct = sel < 4;
        let base = if first {
            c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
            c.cycles += if direct { 5 } else { 6 };
            c.g[S5][0] as u32
        } else {
            c.cycles += if direct { 4 } else { 5 };
            c.g[V1][0] as u32
        };
        let index = if direct { sel } else { sel - 4 };
        let at = base.wrapping_add(index << 2);
        c.g[V0][0] = sx(at);
        let v = Addr(at).read_fpu(c);
        if direct {
            c.f[dest] = v.0;
        } else {
            c.f[0] = v.0;
            c.f[dest] = (-v).0;
        }
    }

    /// A root bone's rotation into the state's frame: sp+0x10 = root
    /// rotation * rotation (quaternion product), also at sp+0x40; v0's
    /// quadword ($2) is reloaded from sp+0.
    fn rotation_to_root_frame(c: &mut Cpu) {
        c.cycles += 16;
        let s = sp(c);
        let st = c.g[S2][0] as u32;
        c.lqc2(4, st + k::ST_ROOT_ROT);
        c.lqc2(5, s + 0x10);
        let vu = c.vu0();
        vu.mul(0b1111, 7, 4, Src::Vec(5));
        vu.opmula(4, 5);
        vu.opmsub(6, 5, 4);
        vu.mula(0b1110, 4, Src::Bc(5, W));
        vu.madda(0b1110, 5, Src::Bc(4, W));
        vu.suba(0b0001, 7, Src::Bc(7, X));
        vu.msuba(0b0001, 0, Src::Bc(7, Y));
        vu.msub(0b0001, 8, 0, Src::Bc(7, Z));
        vu.madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, s + 0x50);
        c.lq(2, s + 0x50);
        c.sq(2, s + 0x10);
        c.sq(2, s + 0x40);
        c.lq(2, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, Rng, DATA, SP};
    use crate::game::animation::animmodel::tests::sample_setup;

    /// `RiderModel_SamplePartTransform` on top of the
    /// `AnimDataBase_SampleChannel` fixture (database at DATA, clip 1 with
    /// three chunks per channel, the anim state at DATA+0x80): 0..2
    /// channels, the part on channel 0, channel 1 or none; root and child
    /// bones, the alternate-slot/scale flag on and off, a zero position w
    /// (direction path), all axis selectors.
    #[test]
    fn rider_model_sample_part_transform_matches() {
        use sample_part_xf as k;
        const STATE: u32 = DATA + 0x80;
        const TABLE: u32 = DATA + 0x200; // chunk descriptors
        const DBS: u32 = DATA + 0x2_0000;
        const MODEL: u32 = DATA + 0x2_1000;
        const PART_BONE: u32 = DATA + 0x2_1100;
        const BONES: u32 = DATA + 0x2_1200;
        const CURVE_BASE: u32 = DATA + 0x2_1400;
        const INFOS: u32 = DATA + 0x2_1800;
        const OUT: u32 = DATA + 0x2_2000;
        check(
            ee::s10::f_0030ecd8__generated,
            rider_model_sample_part_transform,
            250,
            &|c, seed| {
                sample_setup(c, seed);
                let mut r = Rng(0x30ec_d8a1 ^ (seed as u64) << 20);
                let fv = |r: &mut Rng| {
                    if r.next_u32() % 6 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    }
                };
                // stack below sp: what the sampler leaves unwritten
                for i in 0..0x80 {
                    let v = fv(&mut r);
                    c.bus.write32(SP - 0x200 + 4 * i, v);
                }
                // the identity transform and the root origin
                for i in 0..8 {
                    let v = fv(&mut r);
                    c.bus.write32(k::IDENTITY_XF + 4 * i, v);
                }
                for i in 0..4 {
                    let v = fv(&mut r);
                    c.bus.write32(k::ROOT_ORIGIN + 4 * i, v);
                }
                // the database table: the state's index -> the database
                let gp = c.g[GP][0] as u32;
                c.bus.write32(gp + k::ANIM_DBS_GP, DBS);
                let db_index = c.bus.read32(STATE) & 0xff;
                c.bus.write32(DBS + 4 * db_index, DATA);
                // channels of clip 1; channel 0's part is the bank or 7
                let channels = seed % 3;
                c.bus.write16(DATA + 0x100 + 0x14 + k::CLIP_CHANNELS, channels as u16);
                let bank = c.bus.read32(TABLE + 4) & 0xff;
                let first_desc = TABLE + 8; // clip 1 starts at descriptor 1
                if (seed / 3) % 2 == 1 {
                    c.bus.write8(first_desc + k::DESC_PART, 7);
                }
                let part = if seed % 7 == 0 { 9 } else { bank };
                // the state: root position and rotation, the flag
                put_vec(c, STATE + k::ST_ROOT_POS, [fv(&mut r), fv(&mut r), fv(&mut r), fv(&mut r)]);
                put_vec(c, STATE + k::ST_ROOT_ROT, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
                let flag = if seed % 4 < 2 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(STATE + k::ST_ALT, flag);
                // the model: part -> bone, bones, curve bases
                c.bus.write32(MODEL + k::MODEL_PART_BONE, PART_BONE);
                c.bus.write32(MODEL + k::MODEL_BONES, BONES);
                c.bus.write32(MODEL + k::MODEL_CURVE_BASE, CURVE_BASE);
                for p in 0..2 {
                    c.bus.write32(PART_BONE + 4 * p, r.next_u32() % 3);
                }
                let mut base = r.next_u32() % 3;
                for i in 0..8 {
                    c.bus.write32(CURVE_BASE + 4 * i, base);
                    base += r.next_u32() % 2;
                }
                for b in 0..3 {
                    let bone = BONES + k::BONE_SIZE * b;
                    let info = INFOS + 0x80 * b;
                    c.bus.write32(bone + k::BONE_INFO, info);
                    c.bus.write32(bone + k::BONE_SLOT, r.next_u32() % 4);
                    let parent = if (seed + b) % 3 == 0 { 0xffff } else { r.next_u32() % 5 };
                    c.bus.write16(info + k::INFO_PARENT, parent as u16);
                    c.bus.write16(info + k::INFO_ALT_SLOT, (r.next_u32() % 4) as u16);
                    for a in 0..4 {
                        c.bus.write8(info + 0x18 + a, (r.next_u32() % 8) as u8);
                    }
                    for i in 0..4 {
                        let v = fv(&mut r);
                        c.bus.write32(info + k::INFO_SCALE + 4 * i, v);
                    }
                    if seed % 5 == 0 {
                        // zero w: the root position is a direction
                        c.bus.write32(info + k::INFO_SCALE + 0xc, 0);
                    }
                }
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(MODEL);
                c.g[A2][0] = sx(STATE);
                c.g[A3][0] = part as u64;
                c.pc = 0x0030_ecd8;
            },
        );
    }
}
