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
use crate::events::{deliver_event, event_due};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_0120, build_bone_matrix, "RiderModel_BuildBoneMatrix"),
    (0x0031_03f0, build_bone_matrices, "RiderModel_BuildBoneMatrices"),
    (0x0032_9910, rider_sphere_tree_ctor, "RiderSphereTree_ctor"),
    (0x0032_9ae0, rider_sphere_tree_update_from_bones, "RiderSphereTree_UpdateFromBones"),
    (0x0032_c590, sphere_tree_get_radius, "SphereTree_GetRadius"),
    (0x0032_c630, sphere_tree_translate, "SphereTree_Translate"),
    (0x0031_b088, det2x2, "Math_Det2x2"),
    (0x0026_ac48, path_get_length, "Path_GetLength"),
    (0x0031_0530, translate, "RiderModel_Translate"),
    (0x0031_0200, apply_pose, "RiderModel_ApplyPose"),
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

/// VU0's constant register vf0 = (0, 0, 0, 1).
const VF0: Vec4 = Vec4 { x: VuF32::ZERO, y: VuF32::ZERO, z: VuF32::ZERO, w: VuF32::ONE };

/// Lane-wise `vmul` of a vector by a scalar (broadcast field).
fn scale4(v: Vec4, s: VuF32) -> Vec4 {
    Vec4 { x: v.x * s, y: v.y * s, z: v.z * s, w: v.w * s }
}

/// Lane-wise `vadd`.
fn add4(a: Vec4, b: Vec4) -> Vec4 {
    Vec4 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w }
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
    let scaled = [scale4(l[0], s.x), scale4(l[1], s.y), scale4(l[2], s.z), scale4(l[3], s.w)];
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
    let acc = Vec4 { x: zero + zx, y: zero + yz, z: zero + zx, w: zero };
    vu0_scratch::set_acc(c, 0b1110, acc);
    abi::ret(c);
}

const BUILD_BONE_MATRIX: Callee = Callee { addr: 0x0031_0120, f: ee::s10::RiderModel_BuildBoneMatrix };

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
                    deliver_event(c, HEAD);
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
                deliver_event(c, HEAD);
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
    let (a, b, cc, d) = (abi::farg(c, 0), abi::farg(c, 1), abi::farg(c, 2), abi::farg(c, 3));
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
                deliver_event(c, HEAD);
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
                        deliver_event(c, INNER);
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
                deliver_event(c, OUTER);
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
    let p = Vec4 { x: a.x * b.x, y: a.y * b.y, z: a.z * b.z, w: a.w * b.w };
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
                        deliver_event(c, INNER);
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
                deliver_event(c, OUTER);
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
    for (r, v) in [(0, tx), (1, ty), (2, sz_), (3, sw_), (4, tz), (5, tw), (6, sy_)] {
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
    let v = Vec4 { x: t.x - origin.x, y: t.y - origin.y, z: t.z - origin.z, w: t.w - origin.w };
    sp.off(0xa0).write_vec4(c, v);
    c.lq(V0, sp.off(0xa0).0);
    c.sq(V0, sp.off(0x90).0);
    let q = parent.rot().read_vec4(c);
    let v = sp.off(0x90).read_vec4(c);

    // Rotate v by q: c = q×v, d = q×c, out = v + 2w·c + 2d (ACC chain).
    let cr = vu_cross(q, v);
    let cv = Vec4 { x: cr[0], y: cr[1], z: cr[2], w: old6w };
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
    let rotated = Vec4 { x: l0.1, y: l1.1, z: l2.1, w: VuF32::ZERO };
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
    let d4 = Vec4 { x: d[0], y: d[1], z: d[2], w: m.p.w };
    vu0_scratch::set_vf(c, 7, 0b1111, d4);
    vu0_scratch::set_vf(c, 8, 0b1111, rotated);
    let acc = Vec4 { x: l0.0, y: l1.0, z: l2.0, w: m.acc.w };
    vu0_scratch::set_acc(c, 0b1111, acc);
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
            let soft: Vec<_> = r.soft.iter().filter(|m| !m.starts_with("vu0 ctrl")).collect();
            assert!(soft.is_empty(), "seed {seed} (soft): {soft:#?}");
            let mut c = fresh();
            let mode = setup(&mut c, seed);
            let first = c.next_event;
            generated(&mut c);
            if mode != Events::Off {
                event_seeds += 1;
                if c.next_event != first {
                    evented += 1;
                }
            }
        }
        assert!(evented * 4 >= event_seeds, "only {evented} of {event_seeds} event seeds reached an event");
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
        let skin = if seed % 7 == 3 { DATA + 0x2000 } else { DATA + 0x3000 };
        c.bus.write32(DATA + 0x34, skin);
        put_vec(c, DATA + 0x140, vec(r, 2.0, seed));
        for i in 0..bones.min(64) {
            put_vec(c, DATA + 0x1000 + 0x20 * i, vec(r, 1e4, seed));
            put_vec(c, DATA + 0x1010 + 0x20 * i, vec(r, 1.0, seed));
        }
    }

    #[test]
    fn build_bone_matrix() {
        check(ee::s10::f_00310120, super::build_bone_matrix, 400, &|c, seed| {
            let mut r = rng(seed, 1);
            scramble(c, &mut r);
            model(c, &mut r, seed, 16);
            let bone = r.next_u32() % 16;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = bone as u64 | (r.next_u32() as u64) << 32;
            c.g[A2][0] = (DATA + 0x1000 + 0x20 * bone) as u64;
        });
    }

    #[test]
    fn build_bone_matrices() {
        check_paths(ee::s10::f_003103f0, super::build_bone_matrices, 300, &[13, 80, 145], &|c, seed| {
            let mut r = rng(seed, 2);
            scramble(c, &mut r);
            let n = [0, 1, 2, 13, (-3i32) as u32][(seed % 5) as usize];
            model(c, &mut r, seed, n);
            fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
            c.g[A0][0] = DATA as u64;
        });
    }

    driven!(gen_build_bone_matrices, ee::s10::f_003103f0);
    driven!(hand_build_bone_matrices, super::build_bone_matrices);

    #[test]
    fn build_bone_matrices_events() {
        check_events(gen_build_bone_matrices, hand_build_bone_matrices, 300, &|c, seed| {
            let mut r = rng(seed, 3);
            scramble(c, &mut r);
            let n = 2 + r.next_u32() % 12;
            model(c, &mut r, seed, n);
            c.g[A0][0] = DATA as u64;
            schedule(c, &mut r, seed, 0x0031_03f0)
        });
    }

    #[test]
    fn rider_sphere_tree_ctor() {
        check(ee::s10::f_00329910, super::rider_sphere_tree_ctor, 200, &|c, seed| {
            let mut r = rng(seed, 4);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x40);
            c.g[A0][0] = DATA as u64 | (r.next_u32() as u64) << 32;
        });
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
        let f = ee::s10::f_00329ae0;
        check_paths(f, super::rider_sphere_tree_update_from_bones, 300, &[11, 24, 35], &|c, seed| {
            let mut r = rng(seed, 5);
            scramble(c, &mut r);
            let n = [0, 1, 2, 5, (-1i32) as u32][(seed % 5) as usize];
            sphere_tree(c, &mut r, n);
            c.g[A0][0] = DATA as u64;
        });
    }

    driven!(gen_update_from_bones, ee::s10::f_00329ae0);
    driven!(hand_update_from_bones, super::rider_sphere_tree_update_from_bones);

    #[test]
    fn rider_sphere_tree_update_from_bones_events() {
        check_events(gen_update_from_bones, hand_update_from_bones, 300, &|c, seed| {
            let mut r = rng(seed, 6);
            scramble(c, &mut r);
            let n = 3 + r.next_u32() % 6;
            sphere_tree(c, &mut r, n);
            c.g[A0][0] = DATA as u64;
            schedule(c, &mut r, seed, 0x0032_9ae0)
        });
    }

    #[test]
    fn sphere_tree_small() {
        check(ee::s10::f_0032c590, super::sphere_tree_get_radius, 200, &|c, seed| {
            let mut r = rng(seed, 7);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x100);
            c.bus.write32(DATA + 0x98, DATA + 0x200);
            c.bus.write32(DATA + 0x220, DATA + 0x300);
            c.bus.write32(DATA + 0x300, r.edge_f32());
            c.bus.write32(DATA + 0x90, r.edge_f32());
            c.g[A0][0] = DATA as u64;
        });
        check(ee::s10::f_0032c630, super::sphere_tree_translate, 200, &|c, seed| {
            let mut r = rng(seed, 8);
            scramble(c, &mut r);
            put_vec(c, DATA + 0x80, vec(&mut r, 1e4, seed));
            put_vec(c, DATA + 0x400, vec(&mut r, 1e4, seed));
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 9 == 0 { DATA as u64 + 0x80 } else { DATA as u64 + 0x400 };
        });
    }

    #[test]
    fn det2x2() {
        check(ee::s10::f_0031b088, super::det2x2, 300, &|c, seed| {
            let mut r = rng(seed, 9);
            scramble(c, &mut r);
            for i in 12..16 {
                c.f[i] = if seed % 3 == 0 { r.edge_f32() } else { r.f32(100.0) };
            }
        });
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
        check_paths(ee::s07::f_0026ac48, super::path_get_length, 300, &[6, 15, 22], &|c, seed| {
            let mut r = rng(seed, 10);
            scramble(c, &mut r);
            let n = [0, 1, 2, 7, (-4i32) as u32][(seed % 5) as usize];
            path(c, &mut r, n);
            c.g[A0][0] = DATA as u64;
        });
    }

    driven!(gen_path_get_length, ee::s07::f_0026ac48);
    driven!(hand_path_get_length, super::path_get_length);

    #[test]
    fn path_get_length_events() {
        check_events(gen_path_get_length, hand_path_get_length, 300, &|c, seed| {
            let mut r = rng(seed, 11);
            scramble(c, &mut r);
            let n = 4 + r.next_u32() % 30;
            path(c, &mut r, n);
            c.g[A0][0] = DATA as u64;
            schedule(c, &mut r, seed, 0x0026_ac48)
        });
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
                let parent = if r.next_u32() % 3 == 0 { -1i16 as u16 } else { (r.next_u32() % 16) as u16 };
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
        check_paths(ee::s10::f_00310530, super::translate, 400, &[8, 23, 68], &|c, seed| {
            let mut r = rng(seed, 12);
            scramble(c, &mut r);
            let segs = [0, 1, 1, 2, 3, (-1i32) as u32][(seed % 6) as usize];
            let max = if seed % 6 == 1 { (seed / 6) % 2 } else { 4 };
            skeleton(c, &mut r, seed, segs, max);
            c.g[A1][0] = if seed % 11 == 0 { DATA as u64 + 0x1000 } else { DATA as u64 + 0x900 };
        });
    }

    driven!(gen_translate, ee::s10::f_00310530);
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
        check_paths(ee::s10::f_00310200, super::apply_pose, 400, &[8, 24, 120], &|c, seed| {
            let mut r = rng(seed, 14);
            scramble(c, &mut r);
            let segs = [0, 1, 1, 2, 3, (-1i32) as u32][(seed % 6) as usize];
            let max = if seed % 6 == 1 { (seed / 6) % 2 } else { 5 };
            skeleton(c, &mut r, seed, segs, max);
            c.g[A1][0] = DATA as u64 + 0x800;
        });
    }

    driven!(gen_apply_pose, ee::s10::f_00310200);
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
}
