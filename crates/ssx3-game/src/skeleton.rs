//! Work package D2 skeleton, animation & VU0 math
//! (docs/investigation/phase2-candidates-2.md, "D2"; layouts from
//! docs/investigation/structs/rider.md).
//!
//! The rider's skeleton lives in a `RiderModel`: per bone a local
//! translation and rotation (the bind pose), the animated bone transform
//! (`{pos, quat}`), a local 4x4 matrix and a scaled skinning matrix. Every
//! frame `RiderModel_ApplyPose` runs forward kinematics over the bone
//! segments (parent transform ⊗ local transform) and
//! `RiderModel_BuildBoneMatrices` turns each bone transform into its
//! matrices. The other functions are small vector/quaternion/matrix helpers
//! and the animation-sequence list walkers around them.
//!
//! Every replacement reproduces the original's observable state exactly:
//! memory writes (including dead stack scratch), the registers a caller or
//! a later spill could see (v0/v1 with their upper 64 bits after `lq`,
//! f0/f1, the FPU condition bit, temporaries at event checks), the VU0
//! registers it leaves behind (VF, ACC, Q), and the EE cycles of every
//! executed block. VU0 arithmetic follows the original operation order with
//! [`VuF32`] (MADD/MSUB round once), FPU arithmetic uses [`FpuF32`].

use crate::abi::{self, r32, set_word};
use crate::call::{guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_0120, build_bone_matrix, "RiderModel_BuildBoneMatrix"),
    (
        0x0031_03f0,
        build_bone_matrices,
        "RiderModel_BuildBoneMatrices",
    ),
    (0x0032_9910, rider_sphere_tree_ctor, "RiderSphereTree_ctor"),
    (
        0x0032_9ae0,
        rider_sphere_tree_update_from_bones,
        "RiderSphereTree_UpdateFromBones",
    ),
    (0x0032_c590, sphere_tree_get_radius, "SphereTree_GetRadius"),
    (0x0032_c630, sphere_tree_translate, "SphereTree_Translate"),
    (0x0031_b088, det2x2, "Math_Det2x2"),
    (0x0026_ac48, path_get_length, "Path_GetLength"),
    (0x0031_0530, translate, "RiderModel_Translate"),
    (0x0031_0200, apply_pose, "RiderModel_ApplyPose"),
    (0x0031_4760, anim_seq_list_get_at, "AnimSeqList_GetAt"),
    (0x0031_45f8, anim_seq_list_remove, "AnimSeqList_Remove"),
    (
        0x0031_2598,
        anim_controller_evaluate_pose,
        "AnimController_EvaluatePose",
    ),
    (0x0031_23c0, anim_controller_update, "AnimController_Update"),
    (0x0037_dee0, project_point, "PS2GraphicsMan_ProjectPoint"),
    (
        0x0011_dfe0,
        rider_rotate_around_axis,
        "Rider_RotateAroundAxis",
    ),
    (0x0031_b6c8, mat4_inverse, "Mat4_Inverse"),
    (0x0031_b748, mat4_to_pos_quat, "Mat4_ToPosQuat"),
    (0x0036_d400, vu0_rand_next, "Vu0_RandNext"),
    (
        0x0032_c5e0,
        sphere_tree_set_position,
        "SphereTree_SetPosition",
    ),
    (0x0031_b098, det3x3, "Math_Det3x3"),
    (0x0031_b7a8, quat_from_matrix, "Quat_FromMatrix"),
    (0x0031_bb30, quat_from_two_vectors, "Quat_FromTwoVectors"),
];

// ------------------------------------------------------------------ views

/// `RiderModel` (0x160 bytes, alloc "RiderModel"): the rider's skeleton.
#[derive(Clone, Copy, Debug)]
pub struct RiderModel(pub Addr);

impl RiderModel {
    /// `+0x08` number of bone segments.
    pub fn segment_count(self) -> Addr {
        self.0.off(0x08)
    }
    /// `+0x0c` pointer to the segments ([`Segment`], 0x58 bytes each).
    pub fn segments(self) -> Addr {
        self.0.off(0x0c)
    }
    /// `+0x10` number of bones.
    pub fn bone_count(self) -> Addr {
        self.0.off(0x10)
    }
    /// `+0x24` pointer to the local (bind) translations, 16 bytes per bone.
    pub fn local_translations(self) -> Addr {
        self.0.off(0x24)
    }
    /// `+0x28` pointer to the local (bind) rotations, 16 bytes per bone.
    pub fn local_rotations(self) -> Addr {
        self.0.off(0x28)
    }
    /// `+0x2c` pointer to the bone transforms ([`BoneXform`], 0x20 bytes each).
    pub fn bones(self) -> Addr {
        self.0.off(0x2c)
    }
    /// `+0x30` pointer to the local bone matrices (0x40 bytes each).
    pub fn local_matrices(self) -> Addr {
        self.0.off(0x30)
    }
    /// `+0x34` pointer to the scaled (skinning) bone matrices (0x40 each).
    pub fn skin_matrices(self) -> Addr {
        self.0.off(0x34)
    }
    /// `+0x140` scale vector: per row in the skinning matrices, per
    /// component for the local translations.
    pub fn scale(self) -> Addr {
        self.0.off(0x140)
    }
}

/// A bone transform: position at +0 (vec4), rotation quaternion at +0x10.
#[derive(Clone, Copy, Debug)]
pub struct BoneXform(pub Addr);

impl BoneXform {
    pub fn pos(self) -> Addr {
        self.0
    }
    pub fn rot(self) -> Addr {
        self.0.off(0x10)
    }
}

/// `RiderSphereTree`: collision spheres that follow bones.
#[derive(Clone, Copy, Debug)]
pub struct RiderSphereTree(pub Addr);

impl RiderSphereTree {
    /// `+0x00` the bone transforms followed (`RiderModel::bones`).
    pub fn bones(self) -> Addr {
        self.0
    }
    /// `+0x10` root position (copy of the root bone's).
    pub fn root_pos(self) -> Addr {
        self.0.off(0x10)
    }
    /// `+0x24` root bone index (-1: none).
    pub fn root_bone(self) -> Addr {
        self.0.off(0x24)
    }
    /// `+0x28` second bone index (-1: none).
    pub fn bone_28(self) -> Addr {
        self.0.off(0x28)
    }
    /// `+0x2c` number of spheres.
    pub fn sphere_count(self) -> Addr {
        self.0.off(0x2c)
    }
    /// Sphere `i` (0x20 bytes at +0x30: position at +0, bone index at +0x14).
    pub fn sphere(self, i: u32) -> Addr {
        self.0.off(0x30 + i * 0x20)
    }
}

/// `SphereTree` (collision bounds of a world object).
#[derive(Clone, Copy, Debug)]
pub struct SphereTree(pub Addr);

impl SphereTree {
    /// `+0x80` position (vec4).
    pub fn pos(self) -> Addr {
        self.0.off(0x80)
    }
    /// `+0x90` radius scale.
    pub fn scale(self) -> Addr {
        self.0.off(0x90)
    }
    /// `+0x98` pointer to the definition (`+0x14` offset, `+0x20` radius ptr).
    pub fn def(self) -> Addr {
        self.0.off(0x98)
    }
}

// ---------------------------------------------------------------- helpers

/// `lw` of a pointer field.
fn ptr(c: &mut Cpu, field: Addr) -> Addr {
    Addr(field.lw(c) as u32)
}

/// Lane-wise `vmul` of a vector by a scalar (broadcast field).
fn scale4(v: Vec4, s: VuF32) -> Vec4 {
    Vec4 {
        x: v.x * s,
        y: v.y * s,
        z: v.z * s,
        w: v.w * s,
    }
}

/// Lane-wise `vadd`.
fn add4(a: Vec4, b: Vec4) -> Vec4 {
    Vec4 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
}

/// A quaternion as a 3x3 rotation matrix plus the VU0 intermediates.
struct QuatRows {
    /// Rows 0..2 (w lanes 0).
    rows: [Vec4; 3],
    /// Doubled cross terms `(2yz, 2zx, 2xy)` (vf8.xyz).
    cross: [VuF32; 3],
}

/// The game's quaternion → rotation rows (shared VU0 sequence of
/// `Rider_BuildWorldMatrix` and `RiderModel_BuildBoneMatrix`; row-vector
/// convention):
///
/// ```text
/// row0 = (1 - 2y² - 2z²,  2xy + 2zw,      2zx - 2yw,     0)
/// row1 = (2xy - 2zw,      1 - 2z² - 2x²,  2yz + 2xw,     0)
/// row2 = (2zx + 2yw,      2yz - 2xw,      1 - 2x² - 2y², 0)
/// ```
///
/// Operation order: `d = q + q`, squares `d*q`, w products `d*q.w`, cross
/// products by OPMULA then `MADD + 0*0`; diagonal `(1 - a) - 1*b` (SUBA,
/// MSUB), off-diagonals `(0 + c) ± 1*e` (ADDA, MADD/MSUB). Each MADD/MSUB
/// rounds once.
fn quat_rows(q: Vec4) -> QuatRows {
    let (one, zero) = (VuF32::ONE, VuF32::ZERO);
    let (x2, y2, z2) = (q.x + q.x, q.y + q.y, q.z + q.z);
    let (sx, sy, sz) = (x2 * q.x, y2 * q.y, z2 * q.z);
    let (xw, yw, zw) = (x2 * q.w, y2 * q.w, z2 * q.w);
    let yz = VuF32::madd(y2 * q.z, zero, zero);
    let zx = VuF32::madd(z2 * q.x, zero, zero);
    let xy = VuF32::madd(x2 * q.y, zero, zero);
    QuatRows {
        rows: [
            Vec4 {
                x: VuF32::msub(one - sy, one, sz),
                y: VuF32::madd(zero + xy, one, zw),
                z: VuF32::msub(zero + zx, one, yw),
                w: zero,
            },
            Vec4 {
                x: VuF32::msub(zero + xy, one, zw),
                y: VuF32::msub(one - sz, one, sx),
                z: VuF32::madd(zero + yz, one, xw),
                w: zero,
            },
            Vec4 {
                x: VuF32::madd(zero + zx, one, yw),
                y: VuF32::msub(zero + yz, one, xw),
                z: VuF32::msub(one - sx, one, sy),
                w: zero,
            },
        ],
        cross: [yz, zx, xy],
    }
}

/// Read a 4x4 matrix (four quadword rows).
fn read_mat(c: &mut Cpu, a: Addr) -> [Vec4; 4] {
    std::array::from_fn(|i| a.off(0x10 * i as u32).read_vec4(c))
}

// ------------------------------------------------------------ replacements

/// `RiderModel_BuildBoneMatrix(model, bone, xform)` (0x310120): build the
/// bone's local matrix from its transform and the scaled skinning matrix.
///
/// ```text
/// L[bone] = | R(xform.rot) |      S[bone] = | L.row0 * scale.x |
///           | xform.pos    |                | L.row1 * scale.y |
///                                           | L.row2 * scale.z |
///                                           | L.row3 * scale.w |
/// ```
///
/// `R` is [`quat_rows`] of the raw (not renormalised) quaternion; `L`'s
/// translation row keeps `pos.w`. VU0 macro code, 55 cycles, no calls; the
/// skinning pass reloads `L` from memory (and re-reads the matrix pointer),
/// as the original does.
pub fn build_bone_matrix(c: &mut Cpu) {
    c.cycles += 55;
    let model = RiderModel(Addr(abi::arg(c, 0)));
    let offset = abi::arg(c, 1) << 6;
    let xform = BoneXform(Addr(abi::arg(c, 2)));

    let local = Addr(ptr(c, model.local_matrices()).0.wrapping_add(offset));
    let q = xform.rot().read_vec4(c);
    let pos = xform.pos().read_vec4(c);
    let r = quat_rows(q);
    local.off(0x30).write_vec4(c, pos);
    for (i, row) in r.rows.iter().enumerate() {
        local.off(0x10 * i as u32).write_vec4(c, *row);
    }

    // Skinning matrix: each row of L times one lane of the scale.
    let local = Addr(ptr(c, model.local_matrices()).0.wrapping_add(offset));
    let skin_base = ptr(c, model.skin_matrices());
    let skin = skin_base.off(offset);
    let s = model.scale().read_vec4(c);
    let l = read_mat(c, local);
    let scaled = [
        scale4(l[0], s.x),
        scale4(l[1], s.y),
        scale4(l[2], s.z),
        scale4(l[3], s.w),
    ];
    for (i, row) in scaled.iter().enumerate() {
        skin.off(0x10 * i as u32).write_vec4(c, *row);
    }

    // Registers and VU0 state as the original leaves them.
    set_word(c, V0, skin_base.0);
    set_word(c, V1, local.0);
    set_word(c, A1, skin.0);
    vu0_scratch::set_vf(c, 1, 0b1111, s);
    vu0_scratch::set_vf(c, 3, 0b1111, pos);
    for i in 0..4 {
        vu0_scratch::set_vf(c, 4 + i, 0b1111, l[i]);
        vu0_scratch::set_vf(c, 8 + i, 0b1111, scaled[i]);
    }
    vu0_scratch::set_vf(c, 12, 0b1111, r.rows[2]);
    // The last ADDAs of the row code: x = 0 + zx, y = 0 + yz, z = 0 + zx.
    let [yz, zx, _] = r.cross;
    let zero = VuF32::ZERO;
    let acc = Vec4 {
        x: zero + zx,
        y: zero + yz,
        z: zero + zx,
        w: zero,
    };
    vu0_scratch::set_acc(c, 0b1110, acc);
    abi::ret(c);
}

const BUILD_BONE_MATRIX: Callee = Callee {
    addr: 0x0031_0120,
    f: ee::s10::RiderModel_BuildBoneMatrix,
};

/// `RiderModel_BuildBoneMatrices(model)` (0x3103f0): for every bone,
/// `RiderModel_BuildBoneMatrix(model, i, &model->bones[i])`. The bone count
/// and the bones pointer are re-read every iteration.
///
/// Frame 0x30 (s0 at +0x20, s1 at +0x10, ra at +0); structured loop at
/// 0x310418 with an event check on each back edge.
pub fn build_bone_matrices(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_0418;
    const RET: u32 = 0x0031_0430;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let model = RiderModel(Addr(r32(c, S1)));
        let count = model.bone_count().lw(c);
        c.g[V0][0] = count;
        c.g[S0][0] = 0;
        if count as i64 > 0 {
            c.cycles += 2;
            c.g[V0][0] = model.bones().lw(c);
            loop {
                c.cycles += 10;
                // Registers at the call (s0 already incremented).
                let i = r32(c, S0);
                let bones = r32(c, V0);
                set_word(c, A2, bones.wrapping_add(i << 5));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                set_word(c, S0, i.wrapping_add(1));
                BUILD_BONE_MATRIX.call(c, RET)?;
                let more = (c.g[S0][0] as i64) < (model.bone_count().lw(c) as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V0][0] = model.bones().lw(c);
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderSphereTree_ctor(t)` (0x329910): no root bone yet (`+0x24 = -1`,
/// `+0x28 = -1`). Returns `t`. 9 cycles; v1 is left at -1.
pub fn rider_sphere_tree_ctor(c: &mut Cpu) {
    c.cycles += 9;
    let t = RiderSphereTree(Addr(abi::arg(c, 0)));
    t.root_bone().sw(c, u64::MAX);
    t.bone_28().sw(c, u64::MAX);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `RiderSphereTree_UpdateFromBones(t)` (0x329ae0): each sphere takes the
/// position of its bone (`sphere[i].pos = bones[sphere[i].bone].pos`), then
/// the root position is copied from the root bone. Plain 16-byte GPR copies
/// (`lq`/`sq`, no float arithmetic); count and bones pointer are re-read
/// every iteration. Structured loop at 0x329af8.
///
/// Leaves the root qword in v1 (all 128 bits), the root bone address in v0
/// (upper half: the last sphere's qword), a0 = bones, a1 past the last
/// sphere, a3 = count.
pub fn rider_sphere_tree_update_from_bones(c: &mut Cpu) {
    const HEAD: u32 = 0x0032_9af8;
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    let t = RiderSphereTree(Addr(r32(c, A2)));
    let count = t.sphere_count().lw(c);
    c.g[V0][0] = count;
    c.g[A3][0] = 0;
    if count as i64 > 0 {
        c.cycles += 2;
        c.g[A1][0] = c.g[A2][0];
        loop {
            c.cycles += 11;
            let sphere = Addr(r32(c, A1));
            let bone = sphere.off(0x44).lw(c) as u32;
            let i = r32(c, A3).wrapping_add(1);
            set_word(c, A3, i);
            c.g[A0][0] = t.bones().lw(c);
            let src = Addr(bone << 5).off(r32(c, A0));
            set_word(c, V1, src.0);
            // Sphere i's position (sphere = t + i*0x20, position at +0x30).
            c.lq(V0, src.0);
            c.sq(V0, sphere.off(0x30).0);
            let more = (i as i32 as i64) < (t.sphere_count().lw(c) as i64);
            c.g[V0][0] = more as u64;
            set_word(c, A1, sphere.off(0x20).0);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 7;
    let root = t.root_bone().lw(c) as u32;
    c.g[A0][0] = t.bones().lw(c);
    let src = Addr(root << 5).off(r32(c, A0));
    set_word(c, V0, src.0);
    c.lq(V1, src.0);
    c.sq(V1, t.root_pos().0);
    abi::ret(c);
}

/// `SphereTree_GetRadius(t)` (0x32c590): `*t->def->radius * t->scale`
/// (FPU), in f0. Leaves f1 = scale, v1 = def, v0 = radius pointer.
pub fn sphere_tree_get_radius(c: &mut Cpu) {
    c.cycles += 6;
    let t = SphereTree(Addr(abi::arg(c, 0)));
    let def = ptr(c, t.def());
    set_word(c, V1, def.0);
    let scale = t.scale().read_fpu(c);
    let radius = ptr(c, def.off(0x20));
    set_word(c, V0, radius.0);
    let r = radius.read_fpu(c);
    abi::set_fpr(c, 1, scale);
    abi::set_fret(c, r * scale);
    abi::ret(c);
}

/// `SphereTree_Translate(t, d)` (0x32c630): `t->pos += *d` (VU0 `vadd`, all
/// four lanes). Leaves vf3 = old pos, vf4 = d, vf5 = new pos.
pub fn sphere_tree_translate(c: &mut Cpu) {
    c.cycles += 6;
    let t = SphereTree(Addr(abi::arg(c, 0)));
    let pos = t.pos().read_vec4(c);
    let d = Addr(abi::arg(c, 1)).read_vec4(c);
    let moved = add4(pos, d);
    t.pos().write_vec4(c, moved);
    vu0_scratch::set_vf(c, 3, 0b1111, pos);
    vu0_scratch::set_vf(c, 4, 0b1111, d);
    vu0_scratch::set_vf(c, 5, 0b1111, moved);
    abi::ret(c);
}

/// `Math_Det2x2(a, b, c, d)` (0x31b088): `a*d - b*c` (FPU, products rounded
/// separately) in f0; f12 is left holding `a*d`.
pub fn det2x2(c: &mut Cpu) {
    c.cycles += 4;
    let (a, b, cc, d) = (
        abi::farg(c, 0),
        abi::farg(c, 1),
        abi::farg(c, 2),
        abi::farg(c, 3),
    );
    let ad = a * d;
    abi::set_fpr(c, 12, ad);
    abi::set_fret(c, ad - b * cc);
    abi::ret(c);
}

/// `Path_GetLength(path)` (0x26ac48): the sum of the segment lengths
/// (`segments[i].len` at `+0x18 → i*0x10 + 0xc`, count at `+8` read once),
/// added in order with FPU `add.s`, in f0 (and f1). Inside the loop f0 holds
/// the current segment's length, v0 the next length's address and v1 the
/// remaining count. Structured loop at 0x26ac60.
pub fn path_get_length(c: &mut Cpu) {
    const HEAD: u32 = 0x0026_ac60;
    c.cycles += 4;
    let path = Addr(abi::arg(c, 0));
    let count = path.off(8).lw(c);
    c.g[V1][0] = count;
    let mut sum = FpuF32::ZERO;
    c.f[1] = sum.0;
    if count as i64 > 0 {
        c.cycles += 2;
        let lens = ptr(c, path.off(0x18)).off(0xc);
        set_word(c, V0, lens.0);
        loop {
            c.cycles += 7;
            let p = Addr(r32(c, V0));
            let len = p.read_fpu(c);
            let left = r32(c, V1).wrapping_sub(1);
            set_word(c, V1, left);
            set_word(c, V0, p.off(0x10).0);
            sum = FpuF32(c.f[1]) + len;
            c.f[0] = len.0;
            c.f[1] = sum.0;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    c.f[0] = c.f[1];
    abi::ret(c);
}

// ------------------------------------------------------ skeleton walkers

/// Size of a bone segment.
const SEGMENT_SIZE: u32 = 0x58;

/// A bone segment (0x58 bytes): a run of bones from `first_bone` whose
/// definitions (0x50 bytes each, parent index at +0x12) are at `defs`.
#[derive(Clone, Copy, Debug)]
pub struct Segment(pub Addr);

impl Segment {
    /// `+0x04` index of the first bone.
    pub fn first_bone(self) -> Addr {
        self.0.off(0x04)
    }
    /// `+0x38` pointer to the bone definitions.
    pub fn defs(self) -> Addr {
        self.0.off(0x38)
    }
    /// `+0x44` number of bones.
    pub fn bone_count(self) -> Addr {
        self.0.off(0x44)
    }
}

/// A bone definition is 0x50 bytes; `+0x12` holds the parent bone index
/// (i16, negative for a root bone).
const BONE_DEF_SIZE: u32 = 0x50;

/// `addu` of two registers into `rd` (32-bit, sign-extended); returns it.
fn addu_reg(c: &mut Cpu, rd: usize, a: usize, b: usize) -> u32 {
    let v = r32(c, a).wrapping_add(r32(c, b));
    set_word(c, rd, v);
    v
}

/// `slt rd, a, b` on whole registers; returns the result.
fn slt_reg(c: &mut Cpu, rd: usize, a: usize, b: usize) -> bool {
    let lt = (c.g[a][0] as i64) < (c.g[b][0] as i64);
    c.g[rd][0] = lt as u64;
    lt
}

/// `mult rd, a, b` (LO/HI set); returns the low word.
fn mult_reg(c: &mut Cpu, rd: usize, a: usize, b: usize) -> u32 {
    let lo = abi::mult(c, r32(c, a), r32(c, b));
    set_word(c, rd, lo);
    lo
}

/// The `row3 += delta` step of `RiderModel_Translate` for one matrix array
/// (`field` = `&model->local_matrices` or `&model->skin_matrices`, `off` =
/// bone * 0x40). The row goes through the stack frame: copied to sp+0x10
/// (GPR v0), added on VU0 into sp+0x20, copied to sp+0 and the matrix (the
/// matrix pointer is re-read for the store). Returns (matrix address used
/// for the load, raw pointer re-read, matrix address used for the store).
fn translate_row3(c: &mut Cpu, field: Addr, off: u32, delta: Addr) -> (u32, u32, u32) {
    let sp = Addr(crate::call::sp(c));
    let first = (field.lw(c) as u32).wrapping_add(off);
    c.lq(V0, first.wrapping_add(0x30));
    c.sq(V0, sp.off(0x10).0);
    let row = sp.off(0x10).read_vec4(c);
    let d = delta.read_vec4(c);
    let moved = add4(row, d);
    sp.off(0x20).write_vec4(c, moved);
    let raw = field.lw(c) as u32;
    c.lq(V0, sp.off(0x20).0);
    let second = raw.wrapping_add(off);
    c.sq(V0, sp.0);
    c.sq(V0, second.wrapping_add(0x30));
    vu0_scratch::set_vf(c, 3, 0b1111, row);
    vu0_scratch::set_vf(c, 4, 0b1111, d);
    vu0_scratch::set_vf(c, 5, 0b1111, moved);
    (first, raw, second)
}

/// `RiderModel_Translate(model, delta)` (0x310530): move the whole skeleton
/// by `*delta` (all four lanes, VU0 `vadd`): for every bone of every
/// segment, `bones[b].pos`, `local_matrices[b].row3` and
/// `skin_matrices[b].row3` each get `+= *delta` (in that order; `delta` is
/// re-read for each add).
///
/// Frame 0x30 of scratch only (the matrix rows pass through sp+0x10,
/// sp+0x20 and sp+0). Structured loops: segments at 0x310558, bones at
/// 0x310578. Registers are kept in the original's assignment (a2 model, t1
/// delta, a1 segment, t2 next segment, t0 segment offset, a3 bone in
/// segment) so event checks see them.
pub fn translate(c: &mut Cpu) {
    const OUTER: u32 = 0x0031_0558;
    const INNER: u32 = 0x0031_0578;
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A1][0];
    let model = RiderModel(Addr(r32(c, A2)));
    let delta = Addr(r32(c, T1));
    c.g[V0][0] = model.segment_count().lw(c);
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = 0;
    if c.g[V0][0] as i64 > 0 {
        c.cycles += 4;
        c.g[T4][0] = SEGMENT_SIZE as u64;
        c.g[T3][0] = SEGMENT_SIZE as u64;
        mult_reg(c, V1, A1, T4);
        loop {
            c.cycles += 6;
            c.g[A0][0] = model.segments().lw(c);
            c.g[A3][0] = 0;
            let seg = Segment(Addr(addu_reg(c, V1, V1, A0)));
            c.g[V0][0] = seg.bone_count().lw(c);
            set_word(c, T2, r32(c, A1).wrapping_add(1));
            if c.g[V0][0] as i64 > 0 {
                c.cycles += 2;
                mult_reg(c, T0, A1, T3);
                addu_reg(c, V0, T0, A0);
                loop {
                    c.cycles += 43;
                    let seg = Segment(Addr(r32(c, V0)));
                    let bones = model.bones().lw(c) as u32;
                    set_word(c, V1, bones);
                    let b = r32(c, A3).wrapping_add(seg.first_bone().lw(c) as u32);
                    set_word(c, A3, r32(c, A3).wrapping_add(1));
                    let bone = BoneXform(Addr((b << 5).wrapping_add(bones)));
                    set_word(c, V0, bone.0 .0);
                    let off = b << 6;
                    set_word(c, A1, off);
                    // bones[b].pos += delta
                    let pos = bone.pos().read_vec4(c);
                    let d = delta.read_vec4(c);
                    bone.pos().write_vec4(c, add4(pos, d));
                    // local_matrices[b].row3 += delta
                    let (first, _, second) = translate_row3(c, model.local_matrices(), off, delta);
                    set_word(c, V1, first);
                    set_word(c, A0, second);
                    // skin_matrices[b].row3 += delta
                    let (_, raw, second) = translate_row3(c, model.skin_matrices(), off, delta);
                    set_word(c, V1, raw);
                    set_word(c, A1, second);
                    // Next bone of this segment?
                    c.g[A0][0] = model.segments().lw(c);
                    let seg = Segment(Addr(addu_reg(c, V1, T0, A0)));
                    c.g[V0][0] = seg.bone_count().lw(c);
                    let more = slt_reg(c, V0, A3, V0);
                    addu_reg(c, V0, T0, A0);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, INNER);
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = model.segment_count().lw(c);
            c.g[A1][0] = c.g[T2][0];
            let more = slt_reg(c, V0, A1, V0);
            mult_reg(c, V1, A1, T4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, OUTER);
            }
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// The Hamilton product `a ⊗ b` as the game's VU0 sequence computes it:
///
/// ```text
/// xyz = (a.xyz * b.w + b.xyz * a.w) + a.xyz × b.xyz
/// w   = ((a.w*b.w - a.x*b.x) - a.y*b.y) - a.z*b.z
/// ```
///
/// (`vmul` p = a*b; OPMULA/OPMSUB for the cross product; MULA/MADDA for the
/// first two terms; SUBA/MSUBA/MSUB for w; MADD of the cross product by 1).
/// Returns the product and the VU0 intermediates.
struct QuatMul {
    q: Vec4,
    /// `a * b` lane-wise (vf7).
    p: Vec4,
    /// `a × b` (vf6.xyz).
    cross: [VuF32; 3],
    /// ACC afterwards (xyz: the MADDA sum, w: the MSUBA result).
    acc: Vec4,
}

fn quat_mul(a: Vec4, b: Vec4) -> QuatMul {
    let one = VuF32::ONE;
    let p = Vec4 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
        w: a.w * b.w,
    };
    let cross = [
        VuF32::msub(a.y * b.z, b.y, a.z),
        VuF32::msub(a.z * b.x, b.z, a.x),
        VuF32::msub(a.x * b.y, b.x, a.y),
    ];
    let acc = Vec4 {
        x: VuF32::madd(a.x * b.w, b.x, a.w),
        y: VuF32::madd(a.y * b.w, b.y, a.w),
        z: VuF32::madd(a.z * b.w, b.z, a.w),
        w: VuF32::msub(p.w - p.x, one, p.y),
    };
    let q = Vec4 {
        x: VuF32::madd(acc.x, cross[0], one),
        y: VuF32::madd(acc.y, cross[1], one),
        z: VuF32::madd(acc.z, cross[2], one),
        w: VuF32::msub(acc.w, one, p.z),
    };
    QuatMul { q, p, cross, acc }
}

/// The cross product `a × b` as OPMULA a, b / OPMSUB b, a computes it:
/// `(a.y*b.z - b.y*a.z, a.z*b.x - b.z*a.x, a.x*b.y - b.x*a.y)`.
fn vu_cross(a: Vec4, b: Vec4) -> [VuF32; 3] {
    [
        VuF32::msub(a.y * b.z, b.y, a.z),
        VuF32::msub(a.z * b.x, b.z, a.x),
        VuF32::msub(a.x * b.y, b.x, a.y),
    ]
}

/// `RiderModel_ApplyPose(model, roots)` (0x310200): forward kinematics.
/// For every bone `b` of every segment, with parent transform `P`
/// (`bones[parent]`, or for a root bone the next `{pos, quat}` of `roots`):
///
/// ```text
/// t    = local_translation[b] * scale        (FPU, per component)
/// rot  = P.rot ⊗ local_rotation[b]           (quat_mul)
/// v    = t - *(vec4*)0x4ff130
/// pos  = P.pos + v + 2·w·(q×v) + 2·q×(q×v)    (v rotated by q = P.rot)
/// bones[b] = {pos, rot}
/// ```
///
/// The rotation is the VU0 chain `ACC = v*1; ACC += c*w; ACC += c*w;
/// ACC += d*1; out = ACC + d*1` with `c = q×v`, `d = q×c`. Frame 0xb0:
/// every intermediate passes through stack slots (reproduced). Structured
/// loops: segments at 0x310228, bones at 0x310258; registers follow the
/// original's assignment (a2 model, t3 roots, a1 segment / bone, t4 next
/// segment, t2 segment offset, t1 bone definition, t0 bone in segment, a3
/// parent) so event checks see them.
pub fn apply_pose(c: &mut Cpu) {
    const OUTER: u32 = 0x0031_0228;
    const INNER: u32 = 0x0031_0258;
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[T3][0] = c.g[A1][0];
    let model = RiderModel(Addr(r32(c, A2)));
    c.g[V0][0] = model.segment_count().lw(c);
    let frame = Frame::open(c, 0xb0);
    c.g[A1][0] = 0;
    if c.g[V0][0] as i64 > 0 {
        c.cycles += 4;
        c.g[T7][0] = SEGMENT_SIZE as u64;
        c.g[T6][0] = SEGMENT_SIZE as u64;
        mult_reg(c, V1, A1, T7);
        loop {
            c.cycles += 7;
            c.g[A0][0] = model.segments().lw(c);
            c.g[T0][0] = 0;
            set_word(c, T4, r32(c, A1).wrapping_add(1));
            let seg = Segment(Addr(addu_reg(c, V1, V1, A0)));
            c.g[V0][0] = seg.bone_count().lw(c);
            c.g[A3][0] = seg.defs().lw(c);
            if c.g[V0][0] as i64 > 0 {
                c.cycles += 5;
                mult_reg(c, T2, A1, T6);
                c.g[T1][0] = c.g[A3][0];
                c.g[T5][0] = 0x0050_0000;
                let seg = Segment(Addr(addu_reg(c, V1, T2, A0)));
                c.g[V0][0] = seg.first_bone().lw(c);
                loop {
                    c.cycles += 3;
                    let def = Addr(r32(c, T1));
                    c.g[A0][0] = def.off(0x12).lh(c);
                    let b = addu_reg(c, A1, T0, V0);
                    c.cycles += 3;
                    let parent = if (c.g[A0][0] as i64) >= 0 {
                        c.g[V1][0] = model.bones().lw(c);
                        let off = (r32(c, A0)) << 5;
                        set_word(c, V0, off);
                        BoneXform(Addr(addu_reg(c, A3, V1, V0)))
                    } else {
                        // A root bone takes the next input transform.
                        c.g[A3][0] = c.g[T3][0];
                        set_word(c, T3, r32(c, A3).wrapping_add(0x20));
                        BoneXform(Addr(r32(c, A3)))
                    };
                    c.cycles += 85;
                    pose_bone(c, model, b, parent);
                    // Next bone of this segment?
                    c.g[A0][0] = model.segments().lw(c);
                    let seg = Segment(Addr(addu_reg(c, V1, T2, A0)));
                    c.g[V0][0] = seg.bone_count().lw(c);
                    if !slt_reg(c, V0, T0, V0) {
                        break;
                    }
                    c.g[V0][0] = seg.first_bone().lw(c);
                    if event_due(c) {
                        deliver_event_here(c, INNER);
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = model.segment_count().lw(c);
            c.g[A1][0] = c.g[T4][0];
            if !slt_reg(c, V0, A1, V0) {
                break;
            }
            mult_reg(c, V1, A1, T7);
            if event_due(c) {
                deliver_event_here(c, OUTER);
            }
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// The 85-cycle body of `RiderModel_ApplyPose` for bone `b` with parent
/// transform `parent` (see there), with the original's register, stack and
/// VU0 leftovers.
fn pose_bone(c: &mut Cpu, model: RiderModel, b: u32, parent: BoneXform) {
    let sp = Addr(crate::call::sp(c));
    let one = VuF32::ONE;

    // t = local translation * scale (FPU), assembled in sp+0x20 and copied
    // to sp+0; the local rotation is copied to sp+0x10.
    let lt_base = model.local_translations().lw(c) as u32;
    let lt = Addr(lt_base.wrapping_add(b << 4));
    let s = model.scale();
    let (sx_, sy_) = (s.read_fpu(c), s.off(4).read_fpu(c));
    let (lx, ly) = (lt.read_fpu(c), lt.off(4).read_fpu(c));
    set_word(c, T0, r32(c, T0).wrapping_add(1));
    set_word(c, T1, r32(c, T1).wrapping_add(BONE_DEF_SIZE));
    let tx = lx * sx_;
    let lw_ = lt.off(0xc).read_fpu(c);
    let ty = ly * sy_;
    let lz = lt.off(8).read_fpu(c);
    let (sz_, sw_) = (s.off(8).read_fpu(c), s.off(0xc).read_fpu(c));
    sp.off(0x20).write_fpu(c, tx);
    let tz = lz * sz_;
    sp.off(0x24).write_fpu(c, ty);
    let tw = lw_ * sw_;
    let lq_base = model.local_rotations().lw(c) as u32;
    sp.off(0x28).write_fpu(c, tz);
    sp.off(0x2c).write_fpu(c, tw);
    let lq_at = lq_base.wrapping_add(b << 4);
    set_word(c, A0, lq_at);
    c.lq(V1, sp.off(0x20).0);
    let bones = model.bones().lw(c) as u32;
    set_word(c, V0, bones);
    c.sq(V1, sp.0);
    let out = BoneXform(Addr((b << 5).wrapping_add(bones)));
    set_word(c, A1, out.0 .0);
    c.lq(V0, lq_at);
    c.sq(V0, sp.off(0x10).0);
    // FPU registers as the loads and muls leave them.
    for (r, v) in [
        (0, tx),
        (1, ty),
        (2, sz_),
        (3, sw_),
        (4, tz),
        (5, tw),
        (6, sy_),
    ] {
        abi::set_fpr(c, r, v);
    }

    // rot = P.rot ⊗ local rotation.
    let pq = parent.rot().read_vec4(c);
    let lrot = sp.off(0x10).read_vec4(c);
    let m = quat_mul(pq, lrot);
    let old6w = VuF32(c.bus.plat.vf(6)[3]);
    sp.off(0x80).write_vec4(c, m.q);
    c.lq(V0, sp.off(0x80).0);
    c.sq(V0, sp.off(0x60).0);
    c.sq(V0, sp.off(0x70).0);

    // v = t - origin (the vec4 at 0x4ff130), via sp+0xa0 and sp+0x90.
    let t = sp.read_vec4(c);
    let origin = Addr(0x004f_f130).read_vec4(c);
    let v = Vec4 {
        x: t.x - origin.x,
        y: t.y - origin.y,
        z: t.z - origin.z,
        w: t.w - origin.w,
    };
    sp.off(0xa0).write_vec4(c, v);
    c.lq(V0, sp.off(0xa0).0);
    c.sq(V0, sp.off(0x90).0);
    let q = parent.rot().read_vec4(c);
    let v = sp.off(0x90).read_vec4(c);

    // Rotate v by q: c = q×v, d = q×c, out = v + 2w·c + 2d (ACC chain).
    let cr = vu_cross(q, v);
    let cv = Vec4 {
        x: cr[0],
        y: cr[1],
        z: cr[2],
        w: old6w,
    };
    let d = vu_cross(q, cv);
    let lane = |l: usize| {
        let (vl, cl, dl) = ([v.x, v.y, v.z][l], cr[l], d[l]);
        let acc = vl * one;
        let acc = VuF32::madd(acc, cl, q.w);
        let acc = VuF32::madd(acc, cl, q.w);
        let acc = VuF32::madd(acc, dl, one);
        (acc, VuF32::madd(acc, dl, one))
    };
    let (l0, l1, l2) = (lane(0), lane(1), lane(2));
    let rotated = Vec4 {
        x: l0.1,
        y: l1.1,
        z: l2.1,
        w: VuF32::ZERO,
    };
    sp.off(0xa0).write_vec4(c, rotated);
    c.lq(V0, sp.off(0xa0).0);
    c.sq(V0, sp.off(0x80).0);

    // pos = rotated + P.pos.
    let rv = sp.off(0x80).read_vec4(c);
    let ppos = parent.pos().read_vec4(c);
    let pos = add4(rv, ppos);
    sp.off(0xa0).write_vec4(c, pos);
    c.lq(V1, sp.off(0xa0).0);
    c.lq(V0, sp.off(0x60).0);
    c.sq(V1, sp.off(0x70).0);
    c.sq(V0, sp.off(0x40).0);
    c.sq(V1, sp.off(0x50).0);
    c.sq(V1, sp.off(0x30).0);
    c.sq(V1, out.pos().0);
    c.lq(V0, sp.off(0x40).0);
    c.sq(V0, out.rot().0);

    // VU0 leftovers.
    vu0_scratch::set_vf(c, 3, 0b1111, rv);
    vu0_scratch::set_vf(c, 4, 0b1111, ppos);
    vu0_scratch::set_vf(c, 5, 0b1111, pos);
    vu0_scratch::set_vf(c, 6, 0b1110, cv);
    let d4 = Vec4 {
        x: d[0],
        y: d[1],
        z: d[2],
        w: m.p.w,
    };
    vu0_scratch::set_vf(c, 7, 0b1111, d4);
    vu0_scratch::set_vf(c, 8, 0b1111, rotated);
    let acc = Vec4 {
        x: l0.0,
        y: l1.0,
        z: l2.0,
        w: m.acc.w,
    };
    vu0_scratch::set_acc(c, 0b1111, acc);
}

// ----------------------------------------------- animation sequence lists

/// An `AnimSeqList` (8 bytes): `+0` count, `+4` first `AnimSequence`;
/// sequences are chained through `+0xc8`.
#[derive(Clone, Copy, Debug)]
pub struct AnimSeqList(pub Addr);

impl AnimSeqList {
    pub fn count(self) -> Addr {
        self.0
    }
    pub fn head(self) -> Addr {
        self.0.off(4)
    }
}

/// `AnimSequence +0xc8`: the next sequence in its list.
const SEQ_NEXT: u32 = 0xc8;

/// An `AnimController`: `+0x50` its six layer lists (`AnimSeqList[6]`),
/// `+0x54` the `RiderModel`, `+0x58` the per-sequence handler vtable.
#[derive(Clone, Copy, Debug)]
pub struct AnimController(pub Addr);

impl AnimController {
    pub fn layers(self) -> Addr {
        self.0.off(0x50)
    }
    pub fn model(self) -> Addr {
        self.0.off(0x54)
    }
    pub fn handler_vtable(self) -> Addr {
        self.0.off(0x58)
    }
}

/// Number of animation layers.
const LAYERS: i64 = 6;

/// `AnimSeqList_GetAt(list, n)` (0x314760): the `n`-th sequence of the list
/// (0 if the list is shorter). Returns it in v0 (and v1).
///
/// The original is a `match pc` dispatcher: at the back edge to the step
/// at 0x314768 a delivered event may ask for an unwind, and the trampoline
/// re-enters there with v1 = the current sequence and a1 = the remaining
/// count, already decremented. Blocks: entry 2, step 1, null test 2 (v0 =
/// a1), count test 2 (a1 -= 1), return 2.
pub fn anim_seq_list_get_at(c: &mut Cpu) {
    const ENTRY: u32 = 0x0031_4760;
    const STEP: u32 = 0x0031_4768;
    const END: u32 = 0x0031_4784;
    let mut step = match c.pc {
        ENTRY => {
            c.cycles += 2;
            let list = AnimSeqList(Addr(abi::arg(c, 0)));
            c.g[V1][0] = list.head().lw(c);
            false
        }
        STEP => true,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        if step {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, V1)).off(SEQ_NEXT).lw(c);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        if c.g[V1][0] == 0 {
            break;
        }
        c.cycles += 2;
        let more = c.g[V0][0] as i64 > 0;
        set_word(c, A1, r32(c, A1).wrapping_sub(1));
        if !more {
            break;
        }
        if crate::events::back_edge_unwinds(c, STEP) {
            return;
        }
        step = true;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

const ANIM_SEQUENCE_FREE_TRIGGERS: Callee = Callee {
    addr: 0x0031_3c08,
    f: ee::s10::AnimSequence_FreeTriggers,
};
const BUILTIN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

/// `AnimSeqList_Remove(list, seq)` (0x3145f8): unlink `seq` from the list
/// (the link is rewritten first), free its triggers, delete it and
/// decrement the count. Nothing happens if `seq` is not in the list.
///
/// Walks the link slots (`&list->head`, then `&node->next`), loading each
/// node's `next` before comparing. Frame 0x30 (s1 +0x10, s0 +0x20, ra +0);
/// structured loop at 0x314618.
pub fn anim_seq_list_remove(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_4618;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let list = AnimSeqList(Addr(r32(c, S1)));
        c.g[V0][0] = list.head().lw(c);
        set_word(c, V1, list.head().0);
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 3;
                let slot = Addr(r32(c, V1));
                c.g[S0][0] = slot.lw(c);
                let node = Addr(r32(c, S0));
                c.g[V0][0] = node.off(SEQ_NEXT).lw(c);
                if c.g[S0][0] == c.g[A1][0] {
                    // Found: unlink, free, count down.
                    c.cycles += 2;
                    slot.sw(c, c.g[V0][0]);
                    if c.g[S0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        ANIM_SEQUENCE_FREE_TRIGGERS.call(c, 0x0031_4634)?;
                        c.g[A0][0] = c.g[S0][0];
                        BUILTIN_DELETE.call(c, 0x0031_463c)?;
                    }
                    c.cycles += 4;
                    let n = list.count().lw(c) as u32;
                    set_word(c, V0, n.wrapping_sub(1));
                    list.count().sw(c, c.g[V0][0]);
                    break;
                }
                c.cycles += 2;
                set_word(c, V1, node.off(SEQ_NEXT).0);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const ANIM_SEQ_LIST_COLLECT_TRACKS: Callee = Callee {
    addr: 0x0031_4418,
    f: ee::s10::AnimSeqList_CollectTracks,
};
const RIDER_MODEL_BLEND_ANIM_POSE: Callee = Callee {
    addr: 0x0030_f2b0,
    f: ee::s10::RiderModel_BlendAnimPose,
};

/// Size of one track entry gathered by `AnimSeqList_CollectTracks`.
const TRACK_SIZE: u32 = 0x50;
/// Capacity of the track buffer on the stack.
const MAX_TRACKS: u32 = 0x40;

/// `AnimController_EvaluatePose(ac, out)` (0x312598): gather the active
/// tracks of all six layers into a 64-entry buffer on the stack and blend
/// them into the model's pose:
///
/// ```text
/// n = 0
/// for layer in 0..6: n += AnimSeqList_CollectTracks(&ac->layers[layer], &tracks[n], 64 - n)
/// RiderModel_BlendAnimPose(ac->model, n, tracks, out)
/// ```
///
/// Frame 0x1470: the track buffer at sp+0 (64 × 0x50), ra at +0x1400, s5..s0
/// at +0x1410..+0x1460. The buffer's (empty) element constructor loop
/// survives as 64 iterations of 7 cycles with an event check on each back
/// edge (head 0x3125d0); the layer loop's head is 0x3125f8.
pub fn anim_controller_evaluate_pose(c: &mut Cpu) {
    const CTOR_HEAD: u32 = 0x0031_25d0;
    const LAYER_HEAD: u32 = 0x0031_25f8;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x1470);
        frame.save(c, S1, 0x1450);
        frame.save(c, S2, 0x1440);
        frame.save(c, S5, 0x1410);
        frame.save(c, S0, 0x1460);
        frame.save(c, S3, 0x1430);
        frame.save(c, S4, 0x1420);
        frame.save_ra(c, 0x1400);
        c.g[V0][0] = 0x3f;
        c.g[V1][0] = u64::MAX;
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        let ac = AnimController(Addr(r32(c, S2)));
        // The track buffer's trivial constructor loop (v0 = 63 .. -1).
        loop {
            c.cycles += 7;
            set_word(c, V0, r32(c, V0).wrapping_sub(1));
            c.g[S0][0] = 0;
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, CTOR_HEAD);
            }
        }
        c.cycles += 3;
        c.g[S4][0] = TRACK_SIZE as u64;
        c.g[S3][0] = MAX_TRACKS as u64;
        mult_reg(c, V0, S1, S4);
        loop {
            c.cycles += 11;
            // Registers at the call: list, &tracks[n], room left.
            let layer = r32(c, S0);
            set_word(c, A2, r32(c, S3).wrapping_sub(r32(c, S1)));
            set_word(c, S0, layer.wrapping_add(1));
            set_word(c, A1, r32(c, V0).wrapping_add(crate::call::sp(c)));
            c.g[V0][0] = ac.layers().lw(c);
            set_word(c, A0, r32(c, V0).wrapping_add(layer << 3));
            ANIM_SEQ_LIST_COLLECT_TRACKS.call(c, 0x0031_2614)?;
            addu_reg(c, S1, S1, V0);
            let more = (c.g[S0][0] as i64) < LAYERS;
            c.g[V0][0] = more as u64;
            mult_reg(c, V0, S1, S4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LAYER_HEAD);
            }
        }
        c.cycles += 14;
        c.g[A0][0] = ac.model().lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[A2][0] = c.g[SP][0];
        RIDER_MODEL_BLEND_ANIM_POSE.call(c, 0x0031_2638)?;
        frame.restore(c, S0, 0x1460);
        frame.restore(c, S1, 0x1450);
        frame.restore(c, S2, 0x1440);
        frame.restore(c, S3, 0x1430);
        frame.restore(c, S4, 0x1420);
        frame.restore(c, S5, 0x1410);
        frame.restore_ra(c, 0x1400);
        frame.close(c);
        Ok(())
    })
}

const ANIM_SEQ_LIST_GET_AT: Callee = Callee {
    addr: 0x0031_4760,
    f: ee::s10::AnimSeqList_GetAt,
};
const ANIM_CONTROLLER_EVALUATE_POSE: Callee = Callee {
    addr: 0x0031_2598,
    f: ee::s10::AnimController_EvaluatePose,
};

/// `AnimController_Update(ac, out, dt)` (0x3123c0): advance every sequence
/// of the six layers, then evaluate the pose:
///
/// ```text
/// for layer in 0..6:
///     for seq = AnimSeqList_GetAt(&ac->layers[layer], 0); seq; seq = next:
///         next = seq->next                      // read before the call
///         ac->handlers->update(ac, &ac->layers[layer], seq, dt)   // vtable slot +0x10
/// AnimController_EvaluatePose(ac, out)
/// ```
///
/// The handler vtable and the layer array are re-read for every sequence.
/// Frame 0x70: s0..s4 at +0x50..+0x10, f20 (dt) at +0x60, ra at +0.
/// Structured loops: layers at 0x3123f8, sequences at 0x312420.
pub fn anim_controller_update(c: &mut Cpu) {
    const LAYER_HEAD: u32 = 0x0031_23f8;
    const SEQ_HEAD: u32 = 0x0031_2420;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        Addr(frame.at(c, 0x60)).write_fpu(c, FpuF32(c.f[20]));
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.f[20] = c.f[12];
        c.g[A0][0] = 0;
        let ac = AnimController(Addr(r32(c, S1)));
        loop {
            c.cycles += 9;
            c.g[V0][0] = ac.layers().lw(c);
            c.g[S2][0] = c.g[A0][0];
            c.g[A1][0] = 0;
            addu_reg(c, A0, V0, A0);
            set_word(c, S3, r32(c, S0).wrapping_add(1));
            ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_2410)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = ac.handler_vtable().lw(c);
                loop {
                    c.cycles += 11;
                    // Registers at the call: (ac + delta, list, seq, dt).
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = ac.layers().lw(c);
                    c.f[12] = c.f[20];
                    let slot = crate::call::VSlot::read(c, c.g[V0][0], 0x10);
                    c.g[A0][0] = slot.delta;
                    c.g[V1][0] = slot.func;
                    addu_reg(c, A1, A1, S2);
                    c.g[A0][0] = slot.this(c.g[S1][0]);
                    c.g[S0][0] = Addr(r32(c, S0)).off(SEQ_NEXT).lw(c);
                    crate::call::call_ptr(c, slot.func as u32, 0x0031_2444)?;
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S3][0];
                        break;
                    }
                    c.g[V0][0] = ac.handler_vtable().lw(c);
                    if event_due(c) {
                        deliver_event_here(c, SEQ_HEAD);
                    }
                }
            } else {
                c.g[S0][0] = c.g[S3][0];
            }
            c.cycles += 3;
            let more = (c.g[S0][0] as i64) < LAYERS;
            c.g[V0][0] = more as u64;
            set_word(c, A0, r32(c, S0) << 3);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LAYER_HEAD);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        ANIM_CONTROLLER_EVALUATE_POSE.call(c, 0x0031_2468)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        c.f[20] = Addr(frame.at(c, 0x60)).read_fpu(c).0;
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- vector & matrix helpers

/// Leave VU0 as the game's quaternion-product sequence ([`quat_mul`] of
/// `a` = vf4 and `b` = vf5) does: vf6.xyz = a × b, vf7 = a * b, vf8 = the
/// product, ACC = its accumulator.
fn quat_mul_leftovers(c: &mut Cpu, a: Vec4, b: Vec4, m: &QuatMul) {
    vu0_scratch::set_vf(c, 4, 0b1111, a);
    vu0_scratch::set_vf(c, 5, 0b1111, b);
    let cross = Vec4 {
        x: m.cross[0],
        y: m.cross[1],
        z: m.cross[2],
        w: VuF32::ZERO,
    };
    vu0_scratch::set_vf(c, 6, 0b1110, cross);
    vu0_scratch::set_vf(c, 7, 0b1111, m.p);
    vu0_scratch::set_vf(c, 8, 0b1111, m.q);
    vu0_scratch::set_acc(c, 0b1111, m.acc);
}

/// `PS2GraphicsMan_ProjectPoint(out, gm, p)` (0x37dee0): project a point to
/// the screen.
///
/// ```text
/// c   = p · worldClip            (MULA/MADDA/MADDA/MADD, rows by p.x..p.w)
/// n   = c * (1 / c.w)            (VU DIV into Q)
/// out = screenScale * n + screenOffset * 1
/// ```
///
/// `worldClip` is at `gm+0x58a0`, the scale at `+0x58e0`, the offset at
/// `+0x58f0`. Returns `out`; the result also passes through a 16-byte stack
/// slot and is left in v1 (all 128 bits). 24 cycles.
pub fn project_point(c: &mut Cpu) {
    c.cycles += 24;
    let out = Addr(abi::arg(c, 0));
    let gm = Addr(abi::arg(c, 1));
    let frame = Frame::open(c, 0x10);
    let world_clip = gm.off(0x58a0);
    set_word(c, V0, world_clip.0);
    let p = Addr(abi::arg(c, 2)).read_vec4(c);
    let m = read_mat(c, world_clip);
    let scale = gm.off(0x58e0).read_vec4(c);
    let offset = gm.off(0x58f0).read_vec4(c);
    let l = |v: Vec4| [v.x, v.y, v.z, v.w];
    let (m0, m1, m2, m3) = (l(m[0]), l(m[1]), l(m[2]), l(m[3]));
    let clip: [VuF32; 4] = std::array::from_fn(|i| {
        let acc = m0[i] * p.x;
        let acc = VuF32::madd(acc, m1[i], p.y);
        let acc = VuF32::madd(acc, m2[i], p.z);
        VuF32::madd(acc, m3[i], p.w)
    });
    let clip = Vec4 {
        x: clip[0],
        y: clip[1],
        z: clip[2],
        w: clip[3],
    };
    let q = VuF32::div(VuF32::ONE, clip.w);
    let n = scale4(clip, q);
    let s = l(scale);
    let nn = l(n);
    let o = l(offset);
    let acc: [VuF32; 4] = std::array::from_fn(|i| s[i] * nn[i]);
    let res: [VuF32; 4] = std::array::from_fn(|i| VuF32::madd(acc[i], o[i], VuF32::ONE));
    let res = Vec4 {
        x: res[0],
        y: res[1],
        z: res[2],
        w: res[3],
    };
    let sp = Addr(crate::call::sp(c));
    sp.write_vec4(c, res);
    c.lq(V1, sp.0);
    c.g[V0][0] = c.g[A0][0];
    c.sq(V1, out.0);
    for (i, row) in m.iter().enumerate() {
        vu0_scratch::set_vf(c, 1 + i, 0b1111, *row);
    }
    vu0_scratch::set_vf(c, 5, 0b1111, scale);
    vu0_scratch::set_vf(c, 6, 0b1111, offset);
    vu0_scratch::set_vf(c, 10, 0b1111, p);
    vu0_scratch::set_vf(c, 11, 0b1111, clip);
    vu0_scratch::set_vf(c, 12, 0b1111, n);
    vu0_scratch::set_vf(c, 13, 0b1111, res);
    vu0_scratch::set_acc(
        c,
        0b1111,
        Vec4 {
            x: acc[0],
            y: acc[1],
            z: acc[2],
            w: acc[3],
        },
    );
    vu0_scratch::set_q(c, q);
    frame.close(c);
}

const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};
/// `Math_SinCos(angle, &sin, &cos)`, called through the function table
/// (cross-shard).
const MATH_SIN_COS: u32 = 0x0031_be50;

/// `Rider_RotateAroundAxis(rider, axis, angle)` (0x11dfe0): turn the rider
/// by `angle` around `axis` and rebuild its world matrix:
///
/// ```text
/// (s, c)        = sincos(angle / 2)
/// rider->rot    = (axis * s, c) ⊗ rider->rot      (quat_mul)
/// return Rider_BuildWorldMatrix(rider)
/// ```
///
/// `axis * s` is FPU arithmetic. Frame 0x70 (s0 +0x60, s1 +0x50, ra
/// +0x40); sin/cos land at sp+0x30/0x34, the axis quaternion at sp+0x10,
/// the product at sp+0x20 and sp+0. 46 cycles plus the callees.
pub fn rider_rotate_around_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = half.0 as u64;
        c.f[0] = half.0;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.f[12] = (FpuF32(c.f[12]) * half).0;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        let sp = Addr(crate::call::sp(c));
        set_word(c, A0, sp.off(0x30).0);
        set_word(c, A1, sp.off(0x34).0);
        crate::call::call_ptr(c, MATH_SIN_COS, 0x0011_e010)?;

        let axis = Addr(r32(c, S0));
        let rider = crate::rider::Rider(Addr(r32(c, S1)));
        c.g[A0][0] = c.g[S1][0];
        let (ax, ay, az) = (
            axis.read_fpu(c),
            axis.off(4).read_fpu(c),
            axis.off(8).read_fpu(c),
        );
        let (s, co) = (sp.off(0x30).read_fpu(c), sp.off(0x34).read_fpu(c));
        let (rx, ry, rz) = (s * ax, s * ay, s * az);
        sp.off(0x1c).write_fpu(c, co);
        sp.off(0x18).write_fpu(c, rz);
        sp.off(0x10).write_fpu(c, rx);
        sp.off(0x14).write_fpu(c, ry);
        for (r, v) in [(0, ry), (1, rx), (2, co), (3, rz), (4, ay)] {
            abi::set_fpr(c, r, v);
        }

        let a = sp.off(0x10).read_vec4(c);
        let b = rider.rotation().read_vec4(c);
        let m = quat_mul(a, b);
        sp.off(0x20).write_vec4(c, m.q);
        c.lq(V0, sp.off(0x20).0);
        c.sq(V0, rider.rotation().0);
        c.sq(V0, sp.0);
        quat_mul_leftovers(c, a, b, &m);
        RIDER_BUILD_WORLD_MATRIX.call(c, 0x0011_e084)?;

        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

const MAT4_DETERMINANT: Callee = Callee {
    addr: 0x0031_b178,
    f: ee::s10::Mat4_Determinant,
};
const MAT4_ADJOINT: Callee = Callee {
    addr: 0x0031_b310,
    f: ee::s10::Mat4_Adjoint,
};

/// `Mat4_Inverse(m)` (0x31b6c8): invert a 4x4 matrix in place as
/// `adjoint(m) / det(m)`: `det = Mat4_Determinant(m)`, `Mat4_Adjoint(m)`,
/// then every row times `1 / det` (FPU `div.s`, so det = 0 gives the
/// clamped maximum; the scaling is VU0 `vmul`).
///
/// Frame 0x30 (s0 +0x10, f20 +0x20, ra +0). Returns the bits of `1/det` in
/// v0 (`mfc1`), which also land in vf3 through `qmtc2` (with v0's upper
/// half). 31 cycles plus the callees.
pub fn mat4_inverse(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        Addr(frame.at(c, 0x20)).write_fpu(c, FpuF32(c.f[20]));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        MAT4_DETERMINANT.call(c, 0x0031_b6e0)?;
        c.f[20] = c.f[0];
        c.g[A0][0] = c.g[S0][0];
        MAT4_ADJOINT.call(c, 0x0031_b6ec)?;

        c.g[AT][0] = FpuF32::ONE.0 as u64;
        let inv = FpuF32::ONE.div(FpuF32(c.f[20]));
        c.f[0] = inv.0;
        c.g[V0][0] = abi::mfc1(inv);
        let m = Addr(r32(c, S0));
        let rows = read_mat(c, m);
        let k = VuF32(inv.0);
        let scaled = rows.map(|r| scale4(r, k));
        for (i, row) in scaled.iter().enumerate() {
            m.off(0x10 * i as u32).write_vec4(c, *row);
        }
        let v0 = c.g[V0];
        let vf3 = [
            v0[0] as u32,
            (v0[0] >> 32) as u32,
            v0[1] as u32,
            (v0[1] >> 32) as u32,
        ];
        c.bus.plat.set_vf(3, vf3);
        for i in 0..4 {
            vu0_scratch::set_vf(c, 4 + i, 0b1111, rows[i]);
            vu0_scratch::set_vf(c, 8 + i, 0b1111, scaled[i]);
        }

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.f[20] = Addr(frame.at(c, 0x20)).read_fpu(c).0;
        frame.close(c);
        Ok(())
    })
}

const QUAT_FROM_MATRIX: Callee = Callee {
    addr: 0x0031_b7a8,
    f: ee::s10::Quat_FromMatrix,
};

/// `Mat4_ToPosQuat(out, m)` (0x31b748): split a rigid transform into its
/// translation (`out+0 = m.row3`) and rotation quaternion (`out+0x10 =
/// Quat_FromMatrix(m)`). Both go through the 16-byte scratch at sp+0
/// (the translation by FPU loads/stores, then `lq`/`sq`). Returns `out`;
/// leaves the quaternion in v1 (128 bits). Frame 0x30 (s0 +0x20, ra +0x10);
/// 24 cycles plus the callee.
pub fn mat4_to_pos_quat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        let m = Addr(r32(c, V0));
        let sp = Addr(crate::call::sp(c));
        let t: [FpuF32; 4] = std::array::from_fn(|i| m.off(0x30 + 4 * i as u32).read_fpu(c));
        for (i, v) in t.iter().enumerate() {
            sp.off(4 * i as u32).write_fpu(c, *v);
        }
        for (r, v) in [(3, t[0]), (2, t[1]), (0, t[2]), (1, t[3])] {
            abi::set_fpr(c, r, v);
        }
        let out = r32(c, S0);
        c.lq(V0, sp.0);
        c.sq(V0, out);
        QUAT_FROM_MATRIX.call(c, 0x0031_b78c)?;
        c.lq(V1, sp.0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        c.sq(V1, out.wrapping_add(0x10));
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `Vu0_RandNext(seed)` (0x36d400): one step of the VU0 random generator
/// seeded from `*seed`: `R = (bits & 0x7fffff) | 1.0`, then the LFSR step
/// `R = ((R << 1) ^ R[4] ^ R[22]) & 0x7fffff | 1.0`. Stores R back to
/// `*seed` and returns it as a float in [1, 2) in f0.
///
/// The value travels `lwc1 → mfc1 → qmtc2 vf3 → RINIT/RNEXT → qmfc2 →
/// mtc1`, so vf3 = (R, sign extension of the seed, v0's upper half) and v0
/// keeps the seed's sign extension above R. 9 cycles.
pub fn vu0_rand_next(c: &mut Cpu) {
    c.cycles += 9;
    let seed = Addr(abi::arg(c, 0));
    let bits = seed.read_u32(c);
    c.g[V0][0] = sx(bits);
    let mut r = (bits & 0x7f_ffff) | 0x3f80_0000;
    let x = (r >> 4) & 1;
    let y = (r >> 22) & 1;
    r = (((r << 1) ^ x ^ y) & 0x7f_ffff) | 0x3f80_0000;
    let v0 = c.g[V0];
    c.bus.plat.set_vf(
        3,
        [r, (v0[0] >> 32) as u32, v0[1] as u32, (v0[1] >> 32) as u32],
    );
    c.bus.plat.write_ctrl(20, r);
    c.g[V0][0] = (v0[0] & !0xffff_ffff) | r as u64;
    c.f[0] = r;
    seed.write_u32(c, r);
    abi::ret(c);
}

/// `SphereTree_SetPosition(t, p)` (0x32c5e0): `t->pos = (def->offset, 1.0)
/// + *p` (VU0 `vadd`, four lanes), with `def = t->def` (offset at
/// `def+0x14`). The offset is assembled in sp+0x10 by FPU stores (w = 1.0
/// first), the sum goes through sp+0x20, v0 (all 128 bits) and sp+0. Frame
/// 0x30 of scratch; 20 cycles.
pub fn sphere_tree_set_position(c: &mut Cpu) {
    c.cycles += 20;
    let t = SphereTree(Addr(abi::arg(c, 0)));
    let def = ptr(c, t.def());
    set_word(c, V0, def.0);
    let frame = Frame::open(c, 0x30);
    let sp = Addr(crate::call::sp(c));
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    let (ox, oy, oz) = (
        def.off(0x14).read_fpu(c),
        def.off(0x18).read_fpu(c),
        def.off(0x1c).read_fpu(c),
    );
    sp.off(0x10).write_fpu(c, ox);
    sp.off(0x14).write_fpu(c, oy);
    sp.off(0x1c).write_fpu(c, FpuF32::ONE);
    sp.off(0x18).write_fpu(c, oz);
    for (r, v) in [(0, oz), (1, oy), (2, FpuF32::ONE)] {
        abi::set_fpr(c, r, v);
    }
    let o = sp.off(0x10).read_vec4(c);
    let p = Addr(abi::arg(c, 1)).read_vec4(c);
    let pos = add4(o, p);
    sp.off(0x20).write_vec4(c, pos);
    c.lq(V0, sp.off(0x20).0);
    c.sq(V0, t.pos().0);
    c.sq(V0, sp.0);
    vu0_scratch::set_vf(c, 3, 0b1111, o);
    vu0_scratch::set_vf(c, 4, 0b1111, p);
    vu0_scratch::set_vf(c, 5, 0b1111, pos);
    frame.close(c);
}

const MATH_DET2X2: Callee = Callee {
    addr: 0x0031_b088,
    f: ee::s10::Math_Det2x2,
};

/// `Math_Det3x3(a, b, c, d, e, f, g, h, i)` (0x31b098): the determinant of
/// the rows [a b c; d e f; g h i] by cofactors of the first column:
///
/// ```text
/// ((a * det2(e, f, h, i)) - (d * det2(b, c, h, i))) + g * det2(b, c, e, f)
/// ```
///
/// a..h arrive in f12..f19, i on the caller's stack (sp+0). FPU
/// arithmetic; the three 2x2 determinants are calls to `Math_Det2x2`. Frame
/// 0x60: ra at +0, f20..f29 saved at +0x10, +0x18, ... +0x58 (they hold the
/// arguments across the calls). 56 cycles plus the callees.
pub fn det3x3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x60);
        let sp = Addr(crate::call::sp(c));
        for r in 20..30 {
            sp.off(0x10 + 8 * (r as u32 - 20))
                .write_fpu(c, FpuF32(c.f[r]));
        }
        frame.save_ra(c, 0);
        let f = |c: &Cpu, r: usize| FpuF32(c.f[r]);
        let (a, b, cc, d) = (f(c, 12), f(c, 13), f(c, 14), f(c, 15));
        let (e, ff, g, h) = (f(c, 16), f(c, 17), f(c, 18), f(c, 19));
        let i = sp.off(0x60).read_fpu(c);
        // Callee-saved homes of the arguments.
        for (r, v) in [
            (20, a),
            (21, d),
            (22, g),
            (23, h),
            (24, i),
            (26, b),
            (27, cc),
            (28, e),
            (29, ff),
        ] {
            abi::set_fpr(c, r, v);
        }
        let det2 = |c: &mut Cpu, args: [FpuF32; 4], ret: u32| {
            for (k, v) in args.iter().enumerate() {
                abi::set_fpr(c, 12 + k, *v);
            }
            MATH_DET2X2.call(c, ret).map(|_| FpuF32(c.f[0]))
        };
        let r1 = det2(c, [e, ff, h, i], 0x0031_b100)?;
        abi::set_fpr(c, 25, r1);
        let r2 = det2(c, [b, cc, h, i], 0x0031_b118)?;
        abi::set_fpr(c, 23, r2);
        let r3 = det2(c, [b, cc, e, ff], 0x0031_b130)?;
        let det = ((a * r1) - (d * r2)) + g * r3;
        frame.restore_ra(c, 0);
        for r in 20..30 {
            c.f[r] = sp.off(0x10 + 8 * (r as u32 - 20)).read_fpu(c).0;
        }
        c.f[0] = det.0;
        frame.close(c);
        Ok(())
    })
}

/// `c.eq.s a, b`: the compare result, latched in the FCC bit.
fn c_eq_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let eq = a.eq_s(b);
    abi::set_fcc(c, eq);
    eq
}

/// `Quat_FromMatrix(out, m)` (0x31b7a8): the rotation quaternion of a 3x3
/// rotation (row-vector convention, Shoemake's method), FPU arithmetic.
/// With `tr = (m00 + m11) + m22`:
///
/// ```text
/// tr > 0:  s = sqrt(tr + 1);  w = s/2;  k = 0.5/s
///          x = (m12 - m21)k,  y = (m20 - m02)k,  z = (m01 - m10)k
/// else:    i = largest diagonal (0; 1 if m00 < m11; 2 if m_ii < m22)
///          j = (i+1) % 3, k = (j+1) % 3        (integer div: HI)
///          s = sqrt(((m_ii - m_jj) - m_kk) + 1);  q[i] = s/2
///          s = s == 0 ? 0 : 0.5/s
///          w = (m_jk - m_kj)s,  q[j] = (m_ij + m_ji)s,  q[k] = (m_ik + m_ki)s
/// ```
///
/// The first case writes `out` directly (w first); the second builds the
/// quaternion in the 16-byte frame and copies it. Returns `out`. Cycles 40,
/// or 103..105 for the second case.
pub fn quat_from_matrix(c: &mut Cpu) {
    let one = FpuF32::ONE;
    let half = FpuF32(0x3f00_0000);
    c.cycles += 12;
    let out = Addr(abi::arg(c, 0));
    let m = Addr(abi::arg(c, 1));
    c.g[T2][0] = c.g[A1][0];
    c.g[T4][0] = c.g[A0][0];
    let el = |c: &mut Cpu, r: u32, col: u32| m.off(r * 0x10 + col * 4).read_fpu(c);
    let frame = Frame::open(c, 0x10);
    let zero = FpuF32::ZERO;
    let (m00, m11, m22) = (el(c, 0, 0), el(c, 1, 1), el(c, 2, 2));
    let tr = (m00 + m11) + m22;
    abi::set_fpr(c, 6, zero);
    abi::set_fpr(c, 3, m00);
    abi::set_fpr(c, 2, m11);
    abi::set_fpr(c, 4, m22);
    abi::set_fpr(c, 0, m00 + m11);
    abi::set_fpr(c, 1, tr);
    if abi::c_lt_s(c, zero, tr) {
        c.cycles += 25;
        c.g[AT][0] = half.0 as u64;
        let s = (tr + one).sqrt();
        let w = s * half;
        let k = half.div(s);
        out.off(0xc).write_fpu(c, w);
        let x = (el(c, 1, 2) - el(c, 2, 1)) * k;
        out.write_fpu(c, x);
        let y = (el(c, 2, 0) - el(c, 0, 2)) * k;
        out.off(4).write_fpu(c, y);
        let m10 = el(c, 1, 0);
        let z = (el(c, 0, 1) - m10) * k;
        out.off(8).write_fpu(c, z);
        for (r, v) in [(0, z), (1, y), (2, m10), (3, k)] {
            abi::set_fpr(c, r, v);
        }
    } else {
        let sp = Addr(crate::call::sp(c));
        // Largest diagonal element.
        c.cycles += 4;
        let mut i = 0u32;
        if abi::c_lt_s(c, m00, m11) {
            c.cycles += 1;
            i = 1;
        }
        c.cycles += 8;
        c.g[A3][0] = 0x14;
        let dii = abi::mult(c, i, 0x14);
        set_word(c, V1, dii);
        set_word(c, V0, dii.wrapping_add(m.0));
        let mii = el(c, i, i);
        abi::set_fpr(c, 0, mii);
        if abi::c_lt_s(c, mii, m22) {
            i = 2;
        }
        // j = (i + 1) % 3, k = (j + 1) % 3.
        c.cycles += 5;
        abi::div(c, i + 1, 3);
        c.cycles += 29;
        c.g[AT][0] = half.0 as u64;
        let j = c.hi[0] as u32;
        abi::mult(c, i, 0x14);
        let djj = abi::mult(c, j, 0x14);
        set_word(c, T0, djj);
        let (_, k) = abi::div(c, j + 1, 3);
        abi::mult(c, k, 0x14);
        let mut s = (((el(c, i, i) - el(c, j, j)) - el(c, k, k)) + one).sqrt();
        sp.off(4 * i).write_fpu(c, s * half);
        if !c_eq_s(c, s, zero) {
            c.cycles += 1;
            s = half.div(s);
        }
        c.cycles += 42;
        let w = (el(c, j, k) - el(c, k, j)) * s;
        let mji = el(c, j, i);
        let qj = (el(c, i, j) + mji) * s;
        let qk = (el(c, i, k) + el(c, k, i)) * s;
        sp.off(0xc).write_fpu(c, w);
        sp.off(4 * j).write_fpu(c, qj);
        sp.off(4 * k).write_fpu(c, qk);
        let q: [FpuF32; 4] = std::array::from_fn(|n| sp.off(4 * n as u32).read_fpu(c));
        for (n, v) in q.iter().enumerate() {
            out.off(4 * n as u32).write_fpu(c, *v);
        }
        set_word(c, V1, sp.0.wrapping_add(j << 2));
        for (r, v) in [(0, q[1]), (1, q[0]), (2, q[2]), (3, q[3]), (4, mji), (5, s)] {
            abi::set_fpr(c, r, v);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T4][0];
    frame.close(c);
}

const MATH_ASIN: Callee = Callee {
    addr: 0x0031_c128,
    f: ee::s10::Math_Asin,
};
const MATH_SIN_COS_DIRECT: Callee = Callee {
    addr: 0x0031_be50,
    f: ee::s10::Math_SinCos,
};

/// The VU0 4-lane dot product `((p.x + p.y) + 1*p.z) + 1*p.w` of `p = a*b`
/// (ADDA, MADDA, MADD with vf6.x = 1). Returns (p, ACC.x, dot).
fn vu_dot4(a: Vec4, b: Vec4) -> (Vec4, VuF32, VuF32) {
    let one = VuF32::ONE;
    let p = Vec4 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
        w: a.w * b.w,
    };
    let acc = VuF32::madd(p.x + p.y, one, p.z);
    (p, acc, VuF32::madd(acc, one, p.w))
}

/// `Quat_FromTwoVectors(q, u, v)` (0x31bb30): pre-multiply `q` by the
/// rotation taking direction `u` to direction `v`:
///
/// ```text
/// axis = u × v  (w = 0);  len = |axis|           (VU SQRT, read back from Q)
/// if len == 0: return
/// axis /= len;  s = min(len, 0.99999)            (VU DIV; FPU compare)
/// d = u · v  (all four lanes)
/// ang = d < 0 ? pi - asin(s) : asin(s)
/// (sn, cs) = sincos(ang / 2)
/// *q = (axis * sn, cs) ⊗ *q                      (quat_mul)
/// ```
///
/// 0.99999 and pi are the literals at gp-0x2ed0 / gp-0x2ecc. Frame 0x80
/// (s0 +0x70, ra +0x60): the axis passes through sp+0x10 and sp+0 (then
/// normalised in sp+0), sincos outputs at sp+0x50/0x54, the rotation
/// quaternion at sp+0x30, the product at sp+0x40 and sp+0x20. Cycles: 30 on
/// the early exit, else 27 + 12 + 12 + (5 or 2) + 34 + 3 plus the callees.
pub fn quat_from_two_vectors(c: &mut Cpu) {
    guest(c, |c| {
        let one = VuF32::ONE;
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        c.f[1] = 0;
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let sp = Addr(crate::call::sp(c));
        let (ua, va) = (Addr(abi::arg(c, 1)), Addr(abi::arg(c, 2)));
        let u = ua.read_vec4(c);
        let v = va.read_vec4(c);
        let cr = vu_cross(u, v);
        let axis = Vec4 {
            x: cr[0],
            y: cr[1],
            z: cr[2],
            w: VuF32::ZERO,
        };
        sp.off(0x10).write_vec4(c, axis);
        c.lq(V0, sp.off(0x10).0);
        c.sq(V0, sp.0);
        let axis = sp.read_vec4(c);
        let (sq, acc, len2) = vu_dot4(axis, axis);
        let len = VuF32::sqrt(len2);
        set_word(c, V1, len.0);
        let len_f = FpuF32(len.0);
        abi::set_fpr(c, 12, len_f);
        let vf6 = Vec4 {
            x: one,
            y: cr[1],
            z: cr[2],
            w: VuF32::ZERO,
        };
        vu0_scratch::set_vf(c, 3, 0b1111, axis);
        vu0_scratch::set_vf(c, 4, 0b1111, Vec4 { x: len2, ..sq });
        vu0_scratch::set_vf(c, 5, 0b1111, v);
        vu0_scratch::set_vf(c, 6, 0b1111, vf6);
        // ACC: x the dot product sum, y/z still the OPMULA products, w
        // untouched.
        let opm = Vec4 {
            x: acc,
            y: u.z * v.x,
            z: u.x * v.y,
            w: VuF32::ZERO,
        };
        vu0_scratch::set_acc(c, 0b1110, opm);
        vu0_scratch::set_q(c, len);
        if c_eq_s(c, len_f, FpuF32::ZERO) {
            frame.restore(c, S0, 0x70);
            c.cycles += 3;
            frame.restore_ra(c, 0x60);
            frame.close(c);
            return Ok(());
        }

        // Normalise the axis; clamp the sine.
        c.cycles += 12;
        let gp = r32(c, GP);
        let max_sin = Addr(gp.wrapping_sub(0x2ed0)).read_fpu(c);
        c.f[0] = max_sin.0;
        c.g[V0][0] = abi::mfc1(len_f);
        let v0 = c.g[V0];
        let vf3 = Vec4::from_bits(v0[0] as u128 | (v0[1] as u128) << 64);
        let inv = VuF32::div(one, vf3.x);
        let unit = scale4(axis, inv);
        sp.write_vec4(c, unit);
        if abi::c_lt_s(c, max_sin, len_f) {
            c.f[12] = max_sin.0;
        }

        // d = u · v over four lanes.
        c.cycles += 12;
        let u = ua.read_vec4(c);
        let v = va.read_vec4(c);
        let (p, acc, d) = vu_dot4(u, v);
        let vf4 = Vec4 { x: d, ..p };
        vu0_scratch::set_vf(c, 3, 0b1111, u);
        vu0_scratch::set_vf(c, 4, 0b1111, vf4);
        vu0_scratch::set_vf(c, 5, 0b1111, v);
        vu0_scratch::set_acc(c, 0b1000, Vec4 { x: acc, ..opm });
        vu0_scratch::set_q(c, inv);
        abi::set_gpr128(c, V0, vf4.to_bits());
        c.f[0] = d.0;
        let df = FpuF32(d.0);
        if abi::c_lt_s(c, df, FpuF32::ZERO) {
            c.cycles += 5;
            MATH_ASIN.call(c, 0x0031_bc04)?;
            c.f[1] = Addr(gp.wrapping_sub(0x2ecc)).read_u32(c);
            c.f[0] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
        } else {
            c.cycles += 2;
            MATH_ASIN.call(c, 0x0031_bc18)?;
        }

        // r = (axis * sin, cos) of half the angle; q = r ⊗ q.
        c.cycles += 34;
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = half.0 as u64;
        set_word(c, A0, sp.off(0x50).0);
        set_word(c, A1, sp.off(0x54).0);
        c.f[12] = (FpuF32(c.f[0]) * half).0;
        MATH_SIN_COS_DIRECT.call(c, 0x0031_bc30)?;
        let sn = sp.off(0x50).read_fpu(c);
        let (ax, ay, az) = (sp.read_fpu(c), sp.off(4).read_fpu(c), sp.off(8).read_fpu(c));
        let cs = sp.off(0x54).read_fpu(c);
        let (rx, ry, rz) = (sn * ax, sn * ay, sn * az);
        sp.off(0x3c).write_fpu(c, cs);
        sp.off(0x30).write_fpu(c, rx);
        sp.off(0x34).write_fpu(c, ry);
        sp.off(0x38).write_fpu(c, rz);
        for (r, v) in [(0, rz), (1, rx), (2, ry), (3, cs), (4, az)] {
            abi::set_fpr(c, r, v);
        }
        let a = sp.off(0x30).read_vec4(c);
        let qa = Addr(r32(c, S0));
        let b = qa.read_vec4(c);
        let m = quat_mul(a, b);
        sp.off(0x40).write_vec4(c, m.q);
        c.lq(V0, sp.off(0x40).0);
        c.sq(V0, qa.0);
        c.sq(V0, sp.off(0x20).0);
        quat_mul_leftovers(c, a, b, &m);
        frame.restore(c, S0, 0x70);
        c.cycles += 3;
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::{check, check_paths, compare, fill, fresh, put_vec, scramble, Rng, DATA};

    /// `check_events` for VU0 code: temporaries must match too, but the VU0
    /// status/MAC/clip flags (ctrl 16-18, not reproduced) are only
    /// reported, as in `check`. Requires that event seeds reach events.
    fn check_events_vu(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32) -> Events,
    ) {
        let (mut evented, mut event_seeds) = (0, 0);
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| {
                setup(c, seed);
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            let soft: Vec<_> = r
                .soft
                .iter()
                .filter(|m| !m.starts_with("vu0 ctrl"))
                .collect();
            assert!(soft.is_empty(), "seed {seed} (soft): {soft:#?}");
            let mut c = fresh();
            let mode = setup(&mut c, seed);
            let first = c.next_event;
            crate::difftest::run_generated(&mut c, generated);
            if mode != Events::Off {
                event_seeds += 1;
                if c.next_event != first {
                    evented += 1;
                }
            }
        }
        assert!(
            evented * 4 >= event_seeds,
            "only {evented} of {event_seeds} event seeds reached an event"
        );
    }

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }

    /// A float vector, sometimes with edge-case lanes.
    fn vec(r: &mut Rng, range: f32, seed: u32) -> [u32; 4] {
        if seed % 5 == 4 {
            [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]
        } else {
            [r.f32(range), r.f32(range), r.f32(range), r.f32(range)]
        }
    }

    /// A RiderModel at DATA with `bones` bones: transforms at DATA+0x1000,
    /// local matrices at DATA+0x2000, skin matrices at DATA+0x3000 (or
    /// aliased with the local ones), scale at +0x140.
    fn model(c: &mut Cpu, r: &mut Rng, seed: u32, bones: u32) {
        fill(c, r, DATA, 0x1400);
        c.bus.write32(DATA + 0x10, bones);
        c.bus.write32(DATA + 0x2c, DATA + 0x1000);
        c.bus.write32(DATA + 0x30, DATA + 0x2000);
        let skin = if seed % 7 == 3 {
            DATA + 0x2000
        } else {
            DATA + 0x3000
        };
        c.bus.write32(DATA + 0x34, skin);
        put_vec(c, DATA + 0x140, vec(r, 2.0, seed));
        for i in 0..bones.min(64) {
            put_vec(c, DATA + 0x1000 + 0x20 * i, vec(r, 1e4, seed));
            put_vec(c, DATA + 0x1010 + 0x20 * i, vec(r, 1.0, seed));
        }
    }

    #[test]
    fn build_bone_matrix() {
        check(
            ee::s10::f_00310120__generated,
            super::build_bone_matrix,
            400,
            &|c, seed| {
                let mut r = rng(seed, 1);
                scramble(c, &mut r);
                model(c, &mut r, seed, 16);
                let bone = r.next_u32() % 16;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = bone as u64 | (r.next_u32() as u64) << 32;
                c.g[A2][0] = (DATA + 0x1000 + 0x20 * bone) as u64;
            },
        );
    }

    #[test]
    fn build_bone_matrices() {
        check_paths(
            ee::s10::f_003103f0__generated,
            super::build_bone_matrices,
            300,
            &[13, 80, 145],
            &|c, seed| {
                let mut r = rng(seed, 2);
                scramble(c, &mut r);
                let n = [0, 1, 2, 13, (-3i32) as u32][(seed % 5) as usize];
                model(c, &mut r, seed, n);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    driven!(gen_build_bone_matrices, ee::s10::f_003103f0__generated);
    driven!(hand_build_bone_matrices, super::build_bone_matrices);

    #[test]
    fn build_bone_matrices_events() {
        check_events(
            gen_build_bone_matrices,
            hand_build_bone_matrices,
            300,
            &|c, seed| {
                let mut r = rng(seed, 3);
                scramble(c, &mut r);
                let n = 2 + r.next_u32() % 12;
                model(c, &mut r, seed, n);
                c.g[A0][0] = DATA as u64;
                schedule(c, &mut r, seed, 0x0031_03f0)
            },
        );
    }

    #[test]
    fn rider_sphere_tree_ctor() {
        check(
            ee::s10::f_00329910__generated,
            super::rider_sphere_tree_ctor,
            200,
            &|c, seed| {
                let mut r = rng(seed, 4);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = DATA as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    /// A sphere tree at DATA (bones at DATA+0x1000, 16 bones).
    fn sphere_tree(c: &mut Cpu, r: &mut Rng, count: u32) {
        fill(c, r, DATA, 0x200);
        fill(c, r, DATA + 0x1000, 0x80);
        c.bus.write32(DATA, DATA + 0x1000);
        c.bus.write32(DATA + 0x2c, count);
        c.bus.write32(DATA + 0x24, r.next_u32() % 16);
        for i in 0..count.min(8) {
            c.bus.write32(DATA + 0x44 + 0x20 * i, r.next_u32() % 16);
        }
    }

    #[test]
    fn rider_sphere_tree_update_from_bones() {
        let f = ee::s10::f_00329ae0__generated;
        check_paths(
            f,
            super::rider_sphere_tree_update_from_bones,
            300,
            &[11, 24, 35],
            &|c, seed| {
                let mut r = rng(seed, 5);
                scramble(c, &mut r);
                let n = [0, 1, 2, 5, (-1i32) as u32][(seed % 5) as usize];
                sphere_tree(c, &mut r, n);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    driven!(gen_update_from_bones, ee::s10::f_00329ae0__generated);
    driven!(
        hand_update_from_bones,
        super::rider_sphere_tree_update_from_bones
    );

    #[test]
    fn rider_sphere_tree_update_from_bones_events() {
        check_events(
            gen_update_from_bones,
            hand_update_from_bones,
            300,
            &|c, seed| {
                let mut r = rng(seed, 6);
                scramble(c, &mut r);
                let n = 3 + r.next_u32() % 6;
                sphere_tree(c, &mut r, n);
                c.g[A0][0] = DATA as u64;
                schedule(c, &mut r, seed, 0x0032_9ae0)
            },
        );
    }

    #[test]
    fn sphere_tree_small() {
        check(
            ee::s10::f_0032c590__generated,
            super::sphere_tree_get_radius,
            200,
            &|c, seed| {
                let mut r = rng(seed, 7);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(DATA + 0x98, DATA + 0x200);
                c.bus.write32(DATA + 0x220, DATA + 0x300);
                c.bus.write32(DATA + 0x300, r.edge_f32());
                c.bus.write32(DATA + 0x90, r.edge_f32());
                c.g[A0][0] = DATA as u64;
            },
        );
        check(
            ee::s10::f_0032c630__generated,
            super::sphere_tree_translate,
            200,
            &|c, seed| {
                let mut r = rng(seed, 8);
                scramble(c, &mut r);
                put_vec(c, DATA + 0x80, vec(&mut r, 1e4, seed));
                put_vec(c, DATA + 0x400, vec(&mut r, 1e4, seed));
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 9 == 0 {
                    DATA as u64 + 0x80
                } else {
                    DATA as u64 + 0x400
                };
            },
        );
    }

    #[test]
    fn det2x2() {
        check(
            ee::s10::f_0031b088__generated,
            super::det2x2,
            300,
            &|c, seed| {
                let mut r = rng(seed, 9);
                scramble(c, &mut r);
                for i in 12..16 {
                    c.f[i] = if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                }
            },
        );
    }

    /// A path at DATA with `n` segments (lengths at DATA+0x100+i*0x10+0xc).
    fn path(c: &mut Cpu, r: &mut Rng, n: u32) {
        fill(c, r, DATA, 0x400);
        c.bus.write32(DATA + 8, n);
        c.bus.write32(DATA + 0x18, DATA + 0x100);
        for i in 0..n.min(40) {
            c.bus.write32(DATA + 0x10c + 0x10 * i, r.f32(500.0));
        }
    }

    #[test]
    fn path_get_length() {
        check_paths(
            ee::s07::f_0026ac48__generated,
            super::path_get_length,
            300,
            &[6, 15, 22],
            &|c, seed| {
                let mut r = rng(seed, 10);
                scramble(c, &mut r);
                let n = [0, 1, 2, 7, (-4i32) as u32][(seed % 5) as usize];
                path(c, &mut r, n);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    driven!(gen_path_get_length, ee::s07::f_0026ac48__generated);
    driven!(hand_path_get_length, super::path_get_length);

    #[test]
    fn path_get_length_events() {
        check_events(
            gen_path_get_length,
            hand_path_get_length,
            300,
            &|c, seed| {
                let mut r = rng(seed, 11);
                scramble(c, &mut r);
                let n = 4 + r.next_u32() % 30;
                path(c, &mut r, n);
                c.g[A0][0] = DATA as u64;
                schedule(c, &mut r, seed, 0x0026_ac48)
            },
        );
    }

    /// A skeleton at DATA: `segs` segments (at DATA+0x400) of up to
    /// `max_bones` bones each over 16 bones; bone definitions with random
    /// parents (or roots), local translations/rotations, bones, matrices,
    /// root transforms at DATA+0x800, and the origin vector at 0x4ff130.
    fn skeleton(c: &mut Cpu, r: &mut Rng, seed: u32, segs: u32, max_bones: u32) {
        model(c, r, seed, 16);
        c.bus.write32(DATA + 0x08, segs);
        c.bus.write32(DATA + 0x0c, DATA + 0x400);
        c.bus.write32(DATA + 0x24, DATA + 0x5000);
        c.bus.write32(DATA + 0x28, DATA + 0x5100);
        for i in 0..16 {
            put_vec(c, DATA + 0x5000 + 16 * i, vec(r, 100.0, seed));
            put_vec(c, DATA + 0x5100 + 16 * i, vec(r, 1.0, seed));
        }
        for s in 0..segs.min(4) {
            let seg = DATA + 0x400 + 0x58 * s;
            let n = r.next_u32() % (max_bones + 1);
            c.bus.write32(seg + 4, r.next_u32() % (17 - n.min(16)));
            c.bus.write32(seg + 0x38, DATA + 0x5200 + 0x200 * s);
            c.bus.write32(seg + 0x44, n);
            for j in 0..n {
                let parent = if r.next_u32() % 3 == 0 {
                    -1i16 as u16
                } else {
                    (r.next_u32() % 16) as u16
                };
                c.bus.write16(DATA + 0x5212 + 0x200 * s + 0x50 * j, parent);
            }
        }
        for i in 0..24 {
            put_vec(c, DATA + 0x800 + 16 * i, vec(r, 1e3, seed));
        }
        put_vec(c, 0x004f_f130, vec(r, 10.0, seed));
        fill(c, r, crate::difftest::SP - 0xc0, 0x30);
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn translate() {
        check_paths(
            ee::s10::f_00310530__generated,
            super::translate,
            400,
            &[8, 23, 68],
            &|c, seed| {
                let mut r = rng(seed, 12);
                scramble(c, &mut r);
                let segs = [0, 1, 1, 2, 3, (-1i32) as u32][(seed % 6) as usize];
                let max = if seed % 6 == 1 { (seed / 6) % 2 } else { 4 };
                skeleton(c, &mut r, seed, segs, max);
                c.g[A1][0] = if seed % 11 == 0 {
                    DATA as u64 + 0x1000
                } else {
                    DATA as u64 + 0x900
                };
            },
        );
    }

    driven!(gen_translate, ee::s10::f_00310530__generated);
    driven!(hand_translate, super::translate);

    #[test]
    fn translate_events() {
        check_events_vu(gen_translate, hand_translate, 300, &|c, seed| {
            let mut r = rng(seed, 13);
            scramble(c, &mut r);
            skeleton(c, &mut r, seed, 2 + seed % 2, 4);
            c.g[A1][0] = DATA as u64 + 0x900;
            schedule(c, &mut r, seed, 0x0031_0530)
        });
    }

    #[test]
    fn apply_pose() {
        check_paths(
            ee::s10::f_00310200__generated,
            super::apply_pose,
            400,
            &[8, 24, 120],
            &|c, seed| {
                let mut r = rng(seed, 14);
                scramble(c, &mut r);
                let segs = [0, 1, 1, 2, 3, (-1i32) as u32][(seed % 6) as usize];
                let max = if seed % 6 == 1 { (seed / 6) % 2 } else { 5 };
                skeleton(c, &mut r, seed, segs, max);
                c.g[A1][0] = DATA as u64 + 0x800;
            },
        );
    }

    driven!(gen_apply_pose, ee::s10::f_00310200__generated);
    driven!(hand_apply_pose, super::apply_pose);

    #[test]
    fn apply_pose_events() {
        check_events_vu(gen_apply_pose, hand_apply_pose, 300, &|c, seed| {
            let mut r = rng(seed, 15);
            scramble(c, &mut r);
            skeleton(c, &mut r, seed, 2 + seed % 2, 4);
            c.g[A1][0] = DATA as u64 + 0x800;
            schedule(c, &mut r, seed, 0x0031_0200)
        });
    }

    /// A chain of `k` sequences (0x100 bytes each, zero except `next`) at
    /// `at`; returns the first (0 if `k == 0`).
    fn seq_chain(c: &mut Cpu, at: u32, k: u32) -> u32 {
        for i in 0..k {
            for w in 0..0x40 {
                c.bus.write32(at + 0x100 * i + 4 * w, 0);
            }
            let next = if i + 1 < k { at + 0x100 * (i + 1) } else { 0 };
            c.bus.write32(at + 0x100 * i + 0xc8, next);
        }
        if k == 0 {
            0
        } else {
            at
        }
    }

    driven!(gen_get_at, ee::s10::f_00314760__generated);
    driven!(hand_get_at, super::anim_seq_list_get_at);

    #[test]
    fn anim_seq_list_get_at() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = rng(seed, 16);
            scramble(c, &mut r);
            let k = r.next_u32() % 40;
            let head = seq_chain(c, DATA + 0x1000, k);
            c.bus.write32(DATA + 4, head);
            c.g[A0][0] = DATA as u64;
            let n = match r.next_u32() % 4 {
                0 => 0,
                1 => (r.next_u32() % 48) as u64,
                2 => r.next_u32() as i32 as i64 as u64,
                _ => (r.next_u32() % 48) as u64 | (r.next_u32() as u64) << 32,
            };
            c.g[A1][0] = n;
            schedule(c, &mut r, seed, 0x0031_4760)
        };
        check_events(gen_get_at, hand_get_at, 600, &setup);
        crate::difftest::require_paths(gen_get_at, 600, &[6, 8, 13], &|c, s| {
            setup(c, s);
        });
    }

    #[test]
    fn anim_seq_list_remove() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s10::f_003145f8__generated,
            super::anim_seq_list_remove,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 17, 4);
                let k = r.next_u32() % 6;
                let head = seq_chain(c, DATA + 0x1000, k);
                c.bus.write32(DATA, r.next_u32() % 8);
                c.bus.write32(DATA + 4, head);
                // Sometimes the sequence has a trigger to free.
                if k > 0 && r.next_u32() % 3 == 0 {
                    let s = DATA + 0x1000 + 0x100 * (r.next_u32() % k);
                    c.bus.write32(s + 0xac, DATA + 0x2000);
                    c.bus.write32(DATA + 0x200c, 0);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match r.next_u32() % 4 {
                    0 => 0,
                    1 => DATA as u64 + 0x1800,
                    _ if k > 0 => (DATA + 0x1000 + 0x100 * (r.next_u32() % k)) as u64,
                    _ => DATA as u64 + 0x1000,
                };
            },
        );
    }

    driven!(gen_remove, ee::s10::f_003145f8__generated);
    driven!(hand_remove, super::anim_seq_list_remove);

    #[test]
    fn anim_seq_list_remove_events() {
        check_events(gen_remove, hand_remove, 300, &|c, seed| {
            let mut r = rng(seed, 18);
            scramble(c, &mut r);
            let k = 3 + r.next_u32() % 8;
            let head = seq_chain(c, DATA + 0x1000, k);
            c.bus.write32(DATA + 4, head);
            c.g[A0][0] = DATA as u64;
            // Not in the list: the whole chain is walked.
            c.g[A1][0] = DATA as u64 + 0x800;
            schedule(c, &mut r, seed, 0x0031_45f8)
        });
    }

    /// An AnimController at DATA: six layer lists at DATA+0x100 with short
    /// chains of track-less sequences, a handler vtable (slot +0x10 fake),
    /// and a model at DATA+0x6000 (zeroed: no bones, no segments).
    fn anim_controller(c: &mut Cpu, r: &mut Rng, max_seqs: u32) {
        c.bus.write32(DATA + 0x50, DATA + 0x100);
        c.bus.write32(DATA + 0x54, DATA + 0x6000);
        for l in 0..6 {
            let k = r.next_u32() % (max_seqs + 1);
            let head = seq_chain(c, DATA + 0x1000 + 0x800 * l, k);
            c.bus.write32(DATA + 0x100 + 8 * l, k);
            c.bus.write32(DATA + 0x104 + 8 * l, head);
        }
        let vt = DATA + 0x5000;
        c.bus
            .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + 0x14, crate::difftest::fakes::FAKE);
        c.bus.write32(DATA + 0x58, vt);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = DATA as u64 + 0x7000;
        c.f[12] = r.f32(0.1);
    }

    #[test]
    fn anim_controller_evaluate_pose() {
        use crate::difftest::fakes::machine;
        check(
            ee::s10::f_00312598__generated,
            super::anim_controller_evaluate_pose,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 19, 0);
                anim_controller(c, &mut r, 2);
            },
        );
    }

    #[test]
    fn anim_controller_update() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s10::f_003123c0__generated,
            super::anim_controller_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 20, 6);
                anim_controller(c, &mut r, 2);
            },
        );
    }

    driven!(gen_evaluate_pose, ee::s10::f_00312598__generated);
    driven!(hand_evaluate_pose, super::anim_controller_evaluate_pose);

    #[test]
    fn anim_controller_evaluate_pose_events() {
        use crate::difftest::fakes::machine;
        check_events(gen_evaluate_pose, hand_evaluate_pose, 300, &|c, seed| {
            let mut r = machine(c, seed, 21, 0);
            anim_controller(c, &mut r, 2);
            schedule(c, &mut r, seed, 0x0031_2598)
        });
    }

    driven!(gen_update, ee::s10::f_003123c0__generated);
    driven!(hand_update, super::anim_controller_update);

    #[test]
    fn anim_controller_update_events() {
        use crate::difftest::fakes::machine;
        check_events(gen_update, hand_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 22, 0);
            anim_controller(c, &mut r, 4);
            schedule(c, &mut r, seed, 0x0031_23c0)
        });
    }

    #[test]
    fn project_point() {
        check(
            ee::s12::f_0037dee0__generated,
            super::project_point,
            400,
            &|c, seed| {
                let mut r = rng(seed, 23);
                scramble(c, &mut r);
                let gm = DATA;
                for i in 0..4 {
                    put_vec(c, gm + 0x58a0 + 16 * i, vec(&mut r, 2.0, seed));
                }
                put_vec(c, gm + 0x58e0, vec(&mut r, 300.0, seed));
                put_vec(c, gm + 0x58f0, vec(&mut r, 2048.0, seed));
                put_vec(c, DATA + 0x8000, vec(&mut r, 1e3, seed));
                if seed % 13 == 0 {
                    // w = 0 after the transform: division by zero.
                    for i in 0..4 {
                        c.bus.write32(gm + 0x58ac + 16 * i, 0);
                    }
                }
                fill(c, &mut r, crate::difftest::SP - 0x10, 4);
                c.g[A0][0] = if seed % 7 == 0 {
                    DATA as u64 + 0x8000
                } else {
                    DATA as u64 + 0x9000
                };
                c.g[A1][0] = gm as u64;
                c.g[A2][0] = DATA as u64 + 0x8000;
            },
        );
    }

    #[test]
    fn rider_rotate_around_axis() {
        check(
            ee::s00::f_0011dfe0__generated,
            super::rider_rotate_around_axis,
            300,
            &|c, seed| {
                let mut r = rng(seed, 24);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                fill(c, &mut r, DATA, 0x80);
                put_vec(c, DATA + 0x110, vec(&mut r, 1e4, seed));
                put_vec(c, DATA + 0x120, vec(&mut r, 1.0, seed));
                put_vec(c, DATA + 0x400, vec(&mut r, 1.0, seed));
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = DATA as u64 + 0x400;
                c.f[12] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(7.0)
                };
            },
        );
    }

    #[test]
    fn mat4_inverse() {
        check(
            ee::s10::f_0031b6c8__generated,
            super::mat4_inverse,
            300,
            &|c, seed| {
                let mut r = rng(seed, 25);
                scramble(c, &mut r);
                for i in 0..4 {
                    put_vec(c, DATA + 16 * i, vec(&mut r, 10.0, seed));
                }
                if seed % 11 == 0 {
                    // Singular: two equal rows.
                    let row = c.bus.read128(DATA);
                    c.bus.write128(DATA + 0x10, row);
                }
                fill(c, &mut r, crate::difftest::SP - 0x100, 0x40);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn mat4_to_pos_quat() {
        check(
            ee::s10::f_0031b748__generated,
            super::mat4_to_pos_quat,
            300,
            &|c, seed| {
                let mut r = rng(seed, 26);
                scramble(c, &mut r);
                for i in 0..4 {
                    put_vec(c, DATA + 16 * i, vec(&mut r, 1.0, seed));
                }
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
                c.g[A0][0] = DATA as u64 + 0x100;
                c.g[A1][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn vu0_rand_next() {
        check(
            ee::s12::f_0036d400__generated,
            super::vu0_rand_next,
            400,
            &|c, seed| {
                let mut r = rng(seed, 27);
                scramble(c, &mut r);
                c.bus.write32(DATA, r.next_u32());
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn sphere_tree_set_position() {
        check(
            ee::s10::f_0032c5e0__generated,
            super::sphere_tree_set_position,
            300,
            &|c, seed| {
                let mut r = rng(seed, 28);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(DATA + 0x98, DATA + 0x200);
                put_vec(c, DATA + 0x210, vec(&mut r, 100.0, seed));
                put_vec(c, DATA + 0x400, vec(&mut r, 1e4, seed));
                fill(c, &mut r, crate::difftest::SP - 0x30, 0xc);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = DATA as u64 + 0x400;
            },
        );
    }

    #[test]
    fn det3x3() {
        check(
            ee::s10::f_0031b098__generated,
            super::det3x3,
            400,
            &|c, seed| {
                let mut r = rng(seed, 29);
                scramble(c, &mut r);
                for i in 12..20 {
                    c.f[i] = if seed % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                }
                fill(c, &mut r, crate::difftest::SP - 0x60, 0x18);
                c.bus.write32(crate::difftest::SP, r.f32(50.0));
            },
        );
    }

    #[test]
    fn quat_from_matrix() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = rng(seed, 30);
            scramble(c, &mut r);
            for i in 0..4 {
                put_vec(c, DATA + 16 * i, vec(&mut r, 1.0, seed));
            }
            // Mostly a negative trace (the branchy path).
            if seed % 4 != 0 {
                for i in 0..3 {
                    let d = c.bus.read32(DATA + 0x14 * i) | 0x8000_0000;
                    c.bus.write32(DATA + 0x14 * i, d);
                }
                let i = (seed % 3) as u32;
                let d = c.bus.read32(DATA + 0x14 * i) & 0x7fff_ffff;
                if seed % 8 < 3 {
                    c.bus.write32(DATA + 0x14 * i, d);
                }
            }
            fill(c, &mut r, crate::difftest::SP - 0x10, 4);
            c.g[A0][0] = DATA as u64 + 0x100;
            c.g[A1][0] = DATA as u64;
        };
        crate::difftest::check_paths_lohi(
            ee::s10::f_0031b7a8__generated,
            super::quat_from_matrix,
            600,
            &[40, 104, 105],
            &setup,
        );
    }

    #[test]
    fn quat_from_two_vectors() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = rng(seed, 31);
            scramble(c, &mut r);
            let gp = crate::difftest::GAME_GP;
            c.g[GP][0] = gp as u64;
            crate::difftest::put_words(c, gp - 0x2ed0, &[0x3f7f_ff58, 0x4049_0fdb]);
            let u = vec(&mut r, 1.0, seed);
            let v = match seed % 6 {
                0 => u,
                1 => [0; 4],
                _ => vec(&mut r, 1.0, seed),
            };
            put_vec(c, DATA, u);
            put_vec(c, DATA + 0x10, v);
            put_vec(c, DATA + 0x20, vec(&mut r, 1.0, seed));
            fill(c, &mut r, crate::difftest::SP - 0x100, 0x40);
            c.g[A0][0] = DATA as u64 + 0x20;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = DATA as u64 + 0x10;
        };
        check_paths(
            ee::s10::f_0031bb30__generated,
            super::quat_from_two_vectors,
            400,
            &[30],
            &setup,
        );
    }
}
