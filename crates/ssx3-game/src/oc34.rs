//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider and AI collision (walls, rails, riders, ground
//! probes, sphere trees), each with a differential test against the
//! generated body.
//!
//! - **Sphere trees** (`RiderSphereTree_Intersect`): a root sphere and up to
//!   n child spheres per rider; the first overlapping child pair gives the
//!   push-out normal and contact point. Dispatcher-shaped in the original
//!   (its loop heads are resume entries).
//! - **Ground probe** (`RiderGroundState_ProbeGround`): a vertical segment
//!   under the rider against the overlap list, picking the best floor
//!   contact and re-orthonormalising the rider frame on it.
//! - **Walls** (`Rider_FindWallContact`, `Rider_HandleWallCollision`): the
//!   rider hull against the overlap list, the deepest wall contact, the
//!   averaged push-out normal, bounce and wall-hit sound.
//! - **Rails** (`Rider_HandleWallRailContact`, `Rider_TryStartRailslide`):
//!   rail segments found along a wall and the start of a railslide.
//! - **Rider vs rider** (`Rider_CheckRiderCollisions`): sphere-tree contacts
//!   with the other riders, bumps and knock-downs.
//! - **Forward-difference cache** (`AIFwdDiffCache_CollideEntry`): the
//!   cached triangle grid of an AI cache entry against a volume.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers, callees and event handlers see the same values), follow the
//! call protocol of [`crate::call`] (direct calls through the generated
//! aliases, `jalr` and cross-shard calls through the function table) and the
//! event rules of [`crate::events`]. VU0 macro ops are issued on `c.vu0()`
//! exactly as the originals do (flags and ACC are visible to later code).

use crate::call::{call_ptr, guest, Callee, Flow, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Not listed: `Rider_HandleWallRailContact` (0x106f78) and
/// `Rider_TryStartRailslide` (0x106848) are ported below, but their tests
/// only reach the paths before the first rail hit (that needs the real rail
/// finders on real rail data), so they stay on the generated code.
#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_9f98, rider_sphere_tree_intersect, "RiderSphereTree_Intersect"),
    (0x0013_d1b8, rider_ground_state_probe_ground, "RiderGroundState_ProbeGround"),
    (0x0010_4e70, rider_find_wall_contact, "Rider_FindWallContact"),
    (0x0010_5398, rider_handle_wall_collision, "Rider_HandleWallCollision"),
    (0x0010_7888, rider_check_rider_collisions, "Rider_CheckRiderCollisions"),
    (0x0032_b6e0, ai_fwd_diff_cache_collide_entry, "AIFwdDiffCache_CollideEntry"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    pub const SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::Rider_SetMainState,
    };
    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };
    pub const SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    pub const IS_ON_GROUND: Callee = Callee {
        addr: 0x0012_31a8,
        f: ee::s00::Rider_IsOnGround,
    };
    pub const HANDLE_WALL_RAIL_CONTACT: Callee = Callee {
        addr: 0x0010_6f78,
        f: ee::s00::Rider_HandleWallRailContact,
    };
    pub const FIND_WALL_CONTACT: Callee = Callee {
        addr: 0x0010_4e70,
        f: ee::s00::Rider_FindWallContact,
    };
    pub const BOUNCE_OFF_WALL: Callee = Callee {
        addr: 0x0010_57b8,
        f: ee::s00::Rider_BounceOffWall,
    };
    pub const FIND_RAIL: Callee = Callee {
        addr: 0x0010_8a48,
        f: ee::s00::Rider_FindRail,
    };
    pub const TRANSLATE: Callee = Callee {
        addr: 0x0010_6538,
        f: ee::s00::Rider_Translate,
    };
    pub const DEFLECT_ALONG_WALL: Callee = Callee {
        addr: 0x0010_65b0,
        f: ee::s00::Rider_DeflectAlongWall,
    };
    pub const BUILD_WORLD_MATRIX: Callee = Callee {
        addr: 0x0011_e098,
        f: ee::s00::Rider_BuildWorldMatrix,
    };
    pub const HANDLE_IMPACT: Callee = Callee {
        addr: 0x0010_5d98,
        f: ee::s00::Rider_HandleImpact,
    };
    pub const BANK_TRICK_BOOST: Callee = Callee {
        addr: 0x0010_e910,
        f: ee::s00::Rider_BankTrickBoost,
    };
    pub const LAND_TRICK: Callee = Callee {
        addr: 0x0011_9d40,
        f: ee::s00::RiderMetrix_LandTrick,
    };
    pub const ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::Rider_AddBoost,
    };
    pub const ALIGN_STANCE_TO_RAIL: Callee = Callee {
        addr: 0x0011_5358,
        f: ee::s00::Rider_AlignStanceToRail,
    };
    pub const ROTATE_AROUND_AXIS: Callee = Callee {
        addr: 0x0011_dfe0,
        f: ee::s00::Rider_RotateAroundAxis,
    };
    pub const GET_BUMP_STRENGTH: Callee = Callee {
        addr: 0x0011_ff98,
        f: ee::s00::Rider_GetBumpStrength,
    };
    pub const AI_GET_FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };
    pub const RESOLVE_RIDER_COLLISION: Callee = Callee {
        addr: 0x0010_7e70,
        f: ee::s00::Rider_ResolveRiderCollision,
    };

    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const COLL_SEGMENT_CTOR: u32 = 0x0032_e100;
    pub const COLL_HULL_CTOR: u32 = 0x0032_f650;
    pub const COLL_HULL_DTOR: u32 = 0x0032_f708;
    pub const OVERLAP_LIST_COLLIDE_VOLUME: u32 = 0x0033_3ef8;
    pub const OVERLAP_LIST_COLLIDE_OBJECTS: u32 = 0x0033_4458;
    pub const OVERLAP_LIST_FIND_NEAREST_RAIL: u32 = 0x0033_4680;
    pub const CAMERA_TRIGGER_UPDATE_FOR_RIDER: u32 = 0x0016_d320;
    pub const PLAY_WALL_HIT_SOUND: u32 = 0x0029_6088;
    pub const MATH_ATAN: u32 = 0x0031_c228;
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
    pub const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;
    pub const ANIM_ROTATE_ROOTS: u32 = 0x0031_1b48;
    pub const ANIM_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
    pub const ANIM_LAYER_SEQUENCE: u32 = 0x0031_1b20;
    pub const FLAGS64_TEST: u32 = 0x0014_46a0;
    pub const RAILSLIDE_PLAY_ENTER_ANIM: u32 = 0x0013_26c8;
    pub const RAILSLIDE_COMPUTE_ENTRY_ANGLE: u32 = 0x0013_adc0;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_CHAR_ATTRIB5_FACTOR: u32 = 0x0014_9038;
    pub const SPHERE_TREE_INTERSECT: u32 = 0x0032_9f98;
}

// ---------------------------------------------------------------------------
// Register and memory helpers (32-bit address arithmetic, sign-extended
// results, as the compiled code computes them). The pure ones take `&Cpu`;
// the ones that change state are methods of [`Ops`], so their address
// arguments can read the registers (two-phase borrows).

/// The low word of a register.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

/// The current stack pointer.
#[inline(always)]
fn sp(c: &Cpu) -> u32 {
    r(c, SP)
}

/// `slt`-style signed compare of two registers (64-bit).
#[inline(always)]
fn slt(c: &Cpu, a: usize, b: usize) -> u64 {
    ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
}

/// Register, memory and VU0-idiom operations, issued op for op as the
/// compiled code does.
trait Ops {
    /// `reg = sx(v)`.
    fn set_r(&mut self, reg: usize, v: u32);
    /// `reg += n` (`addiu`).
    fn inc_r(&mut self, reg: usize, n: u32);
    /// `lw reg, a`.
    fn lw_r(&mut self, reg: usize, a: u32);
    /// `sw reg, a`.
    fn sw_r(&mut self, a: u32, reg: usize);
    /// `lq reg, from; sq reg, to`.
    fn qcopy(&mut self, reg: usize, from: u32, to: u32);
    /// The dot-product tail: `vf6.w = 1`, `vf4 = vf3 * vf{rhs}`, then
    /// `vf4.w = vf4.x + vf4.y + vf4.z` through ACC.
    fn vdot_regs(&mut self, rhs: u8);
    /// `dot([a], [b])` into `vf4.w` (`vf3 = [a]`, `vf5 = [b]`), then
    /// `reg = vf4` (`qmfc2`) and `f{fr} = reg` (`mtc1`).
    fn vdot(&mut self, a: u32, b: u32, reg: usize, fr: usize);
    /// `|[a]|^2` into `vf4.w` (`vf3 = [a]`).
    fn vlen2(&mut self, a: u32);
    /// `|[a]|` (VU0 `sqrt`), into `reg` (`cfc2` of Q) and `f{fr}`.
    fn vlen(&mut self, a: u32, reg: usize, fr: usize);
    /// `vf5 = vf3 / |vf3|` (after [`Ops::vlen2`]).
    fn vnorm(&mut self);
    /// `vf5 = [v] * reg.x` (`vf4 = [v]`, `vf3 = reg`).
    fn vscale(&mut self, v: u32, reg: usize);
    /// `vf5 = [a] - [b]` (`vf3 = [a]`, `vf4 = [b]`).
    fn vsub(&mut self, a: u32, b: u32);
    /// `vf5 = [a] + [b]` (`vf3 = [a]`, `vf4 = [b]`).
    fn vadd(&mut self, a: u32, b: u32);
    /// `vf4 = [v] / reg` (`Q = vf0.w / vf3.x` with `vf3 = reg`).
    fn vdiv(&mut self, v: u32, reg: usize);
    /// `sqc2 vf{vf}, tmp`, `lq reg, tmp`, then `sq reg` to each of `to`
    /// (the compiler's VU0-to-memory round trip through a GPR).
    fn vstore(&mut self, vf: usize, tmp: u32, reg: usize, to: &[u32]);
}

impl Ops for Cpu {
    #[inline(always)]
    fn set_r(&mut self, reg: usize, v: u32) {
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn inc_r(&mut self, reg: usize, n: u32) {
        let v = r(self, reg).wrapping_add(n);
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn lw_r(&mut self, reg: usize, a: u32) {
        let v = self.lw(a);
        self.g[reg][0] = v;
    }
    #[inline(always)]
    fn sw_r(&mut self, a: u32, reg: usize) {
        let v = self.g[reg][0];
        self.sw(a, v);
    }
    #[inline(always)]
    fn qcopy(&mut self, reg: usize, from: u32, to: u32) {
        self.lq(reg, from);
        self.sq(reg, to);
    }
    #[inline]
    fn vdot_regs(&mut self, rhs: u8) {
        let v = self.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(rhs));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
    }
    #[inline]
    fn vdot(&mut self, a: u32, b: u32, reg: usize, fr: usize) {
        self.lqc2(3, a);
        self.lqc2(5, b);
        self.vdot_regs(5);
        self.qmfc2(reg, 4);
        self.mtc1(reg, fr);
    }
    #[inline]
    fn vlen2(&mut self, a: u32) {
        self.lqc2(3, a);
        self.vdot_regs(3);
    }
    #[inline]
    fn vlen(&mut self, a: u32, reg: usize, fr: usize) {
        self.vlen2(a);
        self.vu0().sqrt(4, 0);
        self.vu0().waitq();
        self.cfc2(reg, 22);
        self.mtc1(reg, fr);
    }
    #[inline]
    fn vnorm(&mut self) {
        self.vu0().rsqrt(0, 3, 4, 0);
        self.vu0().waitq();
        self.vu0().mul(0b1111, 5, 3, Src::Q);
    }
    #[inline]
    fn vscale(&mut self, v: u32, reg: usize) {
        self.lqc2(4, v);
        self.qmtc2(reg, 3);
        self.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    }
    #[inline]
    fn vsub(&mut self, a: u32, b: u32) {
        self.lqc2(3, a);
        self.lqc2(4, b);
        self.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    }
    #[inline]
    fn vadd(&mut self, a: u32, b: u32) {
        self.lqc2(3, a);
        self.lqc2(4, b);
        self.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    #[inline]
    fn vdiv(&mut self, v: u32, reg: usize) {
        self.qmtc2(reg, 3);
        self.vu0().div(0, 3, 3, 0);
        self.lqc2(4, v);
        self.vu0().waitq();
        self.vu0().mul(0b1111, 4, 4, Src::Q);
    }
    #[inline]
    fn vstore(&mut self, vf: usize, tmp: u32, reg: usize, to: &[u32]) {
        self.sqc2(vf, tmp);
        self.lq(reg, tmp);
        for &a in to {
            self.sq(reg, a);
        }
    }
}

/// The epilogue of the rider collision functions: callee-saved GPRs and
/// FPRs back, `ra`, `sp += size`, return.
fn epilogue(c: &mut Cpu, gprs: &[(usize, u32)], fprs: &[(usize, u32)], ra_at: u32, size: u32) {
    let s = sp(c);
    for &(reg, off) in gprs {
        c.lq(reg, p(s, off));
    }
    c.g[RA][0] = c.ld(p(s, ra_at));
    for &(f, off) in fprs {
        c.lwc1(f, p(s, off));
    }
    let ra = r(c, RA);
    c.set_r(SP, s.wrapping_add(size));
    c.pc = ra;
}

/// The prologue: `sp -= size`, the callee-saved GPRs (`sq`), `ra` (`sd`)
/// and FPRs (`swc1`).
fn prologue(c: &mut Cpu, size: u32, gprs: &[(usize, u32)], ra_at: u32, fprs: &[(usize, u32)]) {
    c.set_r(SP, sp(c).wrapping_sub(size));
    let s = sp(c);
    for &(reg, off) in gprs {
        c.sq(reg, p(s, off));
    }
    c.sd(p(s, ra_at), c.g[RA][0]);
    for &(f, off) in fprs {
        c.swc1(f, p(s, off));
    }
}

/// A gcc 2.9x virtual call on `obj` (`vt = obj->+0xc`... read by the
/// caller): `a0 = obj + lh(vt + slot)`, target `lw(vt + slot + 4)` in
/// `func_reg`, `jalr` returning to `ret`. `vt_reg` holds the vtable.
fn vcall(c: &mut Cpu, obj_reg: usize, vt_reg: usize, slot: u32, func_reg: usize, ret: u32) -> Flow {
    let vt = r(c, vt_reg);
    let delta = c.lh(p(vt, slot));
    c.g[A0][0] = delta;
    let func = c.lw(p(vt, slot + 4));
    c.g[func_reg][0] = func;
    let this = r(c, obj_reg).wrapping_add(r(c, A0));
    c.set_r(A0, this);
    call_ptr(c, func as u32, ret)
}

// ---------------------------------------------------------------------------
// Sphere trees.

/// `RiderSphereTree` fields: the root sphere and the child spheres
/// (`CHILD_SIZE` apart, from `CHILD`).
mod sphere_tree {
    pub const CENTER: u32 = 0x10;
    pub const RADIUS: u32 = 0x20;
    pub const COUNT: u32 = 0x2c;
    pub const CHILD_CENTER: u32 = 0x30;
    pub const CHILD_RADIUS: u32 = 0x40;
    pub const CHILD_SIZE: u32 = 0x20;
}

const ST_ENTRY: u32 = 0x0032_9f98;
/// The hit block (reached from the inner loop's back edge): resume entry.
const ST_HIT: u32 = 0x0032_a004;
/// Outer loop head (children of tree A against B's root): resume entry.
const ST_OUTER: u32 = 0x0032_a0b8;
/// Inner loop head (children of B against A's child): resume entry.
const ST_INNER: u32 = 0x0032_a130;
const ST_END: u32 = 0x0032_a1c0;

#[derive(Clone, Copy)]
enum SphereStage {
    Children,
    Outer,
    OuterReload,
    OuterNext,
    Inner,
    Hit,
    Miss,
    Done,
}

/// The inlined sphere-overlap test: `f1 = rb`, `f0 = ra`, `d = cb - ca`
/// (stored at `sp`, `sp+0x10`, `sp+0x20`), `f2 = rb + ra`, `f1 = f2^2`,
/// `f0 = |d|^2`, FCC = `f1 < f0`. True when the spheres are apart.
fn spheres_apart(c: &mut Cpu, b_center: u32, b_radius: u32, a_center: u32, a_radius: u32) -> bool {
    let s = sp(c);
    c.lwc1(1, b_radius);
    c.lwc1(0, a_radius);
    c.vsub(b_center, a_center);
    c.sqc2(5, p(s, 0x20));
    c.add_s(2, 1, 0);
    c.lq(V0, p(s, 0x20));
    c.sq(V0, s);
    c.mul_s(1, 2, 2);
    c.sq(V0, p(s, 0x10));
    c.lqc2(3, s);
    c.lqc2(5, s);
    c.vdot_regs(5);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(1, 0);
    c.fcc()
}

/// `bool RiderSphereTree_Intersect(SphereTree* a, SphereTree* b, Vec4*
/// push, Vec4* point)` (0x329f98): when the root spheres overlap, tests
/// every child sphere of `a` against `b`'s root and then against every
/// child of `b`; at the first overlapping pair writes the push-out vector
/// (`normal * penetration`) to `push` and the contact point (on `a`'s
/// child, `t2`) to `point` and returns true.
///
/// Dispatcher-shaped in the original: the loop heads and the hit block are
/// resume entries, all state lives in registers (t0-t4, f0/f2, the stack
/// scratch).
pub fn rider_sphere_tree_intersect(c: &mut Cpu) {
    use sphere_tree as st;
    let mut stage = match c.pc {
        ST_ENTRY => {
            c.cycles += 25;
            c.set_r(SP, sp(c).wrapping_sub(0x50));
            let (a, b) = (r(c, A0), r(c, A1));
            let apart = spheres_apart(
                c,
                p(b, st::CENTER),
                p(b, st::RADIUS),
                p(a, st::CENTER),
                p(a, st::RADIUS),
            );
            c.g[V0][0] = 0;
            if apart {
                c.cycles += 2;
                SphereStage::Done
            } else {
                SphereStage::Children
            }
        }
        ST_HIT => SphereStage::Hit,
        ST_OUTER => SphereStage::Outer,
        ST_INNER => SphereStage::Inner,
        pc => {
            c.jump_out(pc, ST_ENTRY, ST_END);
            return;
        }
    };
    loop {
        stage = match stage {
            SphereStage::Children => {
                c.cycles += 3;
                c.lw_r(V0, at(c, A0, st::COUNT));
                c.g[T4][0] = 0;
                if (c.g[V0][0] as i64) <= 0 {
                    SphereStage::Miss
                } else {
                    c.cycles += 2;
                    c.g[T3][0] = c.g[A0][0];
                    SphereStage::Outer
                }
            }
            // A's child t3 against B's root.
            SphereStage::Outer => {
                c.cycles += 24;
                let (b, child) = (r(c, A1), r(c, T3));
                let apart = spheres_apart(
                    c,
                    p(b, st::CENTER),
                    p(b, st::RADIUS),
                    p(child, st::CHILD_CENTER),
                    p(child, st::CHILD_RADIUS),
                );
                if apart {
                    c.lw_r(V0, at(c, A0, st::COUNT));
                    SphereStage::OuterNext
                } else {
                    c.cycles += 3;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[T1][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        SphereStage::OuterReload
                    } else {
                        c.cycles += 3;
                        c.g[T2][0] = c.g[T3][0];
                        c.g[T0][0] = c.g[A1][0];
                        SphereStage::Inner
                    }
                }
            }
            // B's child t0 against A's child t2.
            SphereStage::Inner => {
                c.cycles += 24;
                let (bc, ac) = (r(c, T0), r(c, T2));
                let apart = spheres_apart(
                    c,
                    p(bc, st::CHILD_CENTER),
                    p(bc, st::CHILD_RADIUS),
                    p(ac, st::CHILD_CENTER),
                    p(ac, st::CHILD_RADIUS),
                );
                c.inc_r(T1, 1);
                if !apart {
                    if back_edge_unwinds(c, ST_HIT) {
                        return;
                    }
                    SphereStage::Hit
                } else {
                    c.cycles += 4;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[V0][0] = slt(c, T1, V0);
                    c.inc_r(T0, st::CHILD_SIZE);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, ST_INNER) {
                            return;
                        }
                        SphereStage::Inner
                    } else {
                        SphereStage::OuterReload
                    }
                }
            }
            SphereStage::OuterReload => {
                c.cycles += 1;
                c.lw_r(V0, at(c, A0, st::COUNT));
                SphereStage::OuterNext
            }
            SphereStage::OuterNext => {
                c.cycles += 4;
                c.inc_r(T4, 1);
                c.g[V0][0] = slt(c, T4, V0);
                c.inc_r(T3, st::CHILD_SIZE);
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, ST_OUTER) {
                        return;
                    }
                    SphereStage::Outer
                } else {
                    SphereStage::Miss
                }
            }
            SphereStage::Hit => {
                sphere_tree_hit(c);
                SphereStage::Done
            }
            SphereStage::Miss => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                SphereStage::Done
            }
            SphereStage::Done => {
                c.cycles += 2;
                let ra = r(c, RA);
                c.set_r(SP, sp(c).wrapping_add(0x50));
                c.pc = ra;
                return;
            }
        };
    }
}

/// The hit: `n = d / |d|` (d at `sp`), `pen = (rb + ra) - |d|` (f2 - f1),
/// `*push = n * pen`, `*point = ca - n * (ra - pen)` with `a`'s child t2.
fn sphere_tree_hit(c: &mut Cpu) {
    c.cycles += 40;
    let s = sp(c);
    c.vlen2(s);
    c.vnorm();
    c.sqc2(5, p(s, 0x20));
    c.sqrt_s(1, 0);
    c.lq(V1, p(s, 0x20));
    c.sub_s(1, 2, 1);
    c.g[V0][0] = 1;
    c.sq(V1, p(s, 0x10));
    c.mfc1(A0, 1);
    c.vscale(p(s, 0x10), A0);
    let push = r(c, A2);
    c.vstore(5, p(s, 0x30), V1, &[push, p(s, 0x20)]);
    let child = r(c, T2);
    c.lwc1(0, p(child, sphere_tree::CHILD_RADIUS));
    c.sub_s(0, 0, 1);
    c.mfc1(A0, 0);
    c.vscale(p(s, 0x10), A0);
    c.vstore(5, p(s, 0x40), V1, &[p(s, 0x30)]);
    c.vsub(p(child, sphere_tree::CHILD_CENTER), p(s, 0x30));
    let point = r(c, A3);
    c.vstore(5, p(s, 0x40), V1, &[point, p(s, 0x20)]);
}

// ---------------------------------------------------------------------------
// Ground probe.

/// `Contact` records (0x80 bytes) as the collision queries fill them.
mod contact {
    pub const SIZE: u32 = 0x80;
    pub const NORMAL: u32 = 0x10;
    pub const DEPTH: u32 = 0x40;
    pub const SURFACE: u32 = 0x50;
    pub const INSTANCE: u32 = 0x54;
    pub const MATERIAL: u32 = 0x5c;
}

/// Rider fields used by the collision code.
mod rider {
    pub const VEL: u32 = 0x1e0;
    pub const FWD_REF: u32 = 0x1b0;
    pub const POS: u32 = 0x110;
    pub const UP: u32 = 0x370;
    pub const UP_PREV: u32 = 0x380;
    pub const FRAME_FWD: u32 = 0x3a0;
    pub const FRAME_SIDE: u32 = 0x3b0;
    pub const GROUND_NORMAL2: u32 = 0x3d0;
    pub const GROUND_ID: u32 = 0x430;
    pub const GROUND_MATERIAL: u32 = 0x438;
    pub const GROUND_HEIGHT: u32 = 0x454;
    pub const GROUND_POINT: u32 = 0x460;
    pub const OVERLAP_LIST: u32 = 0x860;
    pub const OVERLAP_HINT: u32 = 0x864;
    pub const SPHERE_TREE: u32 = 0xaa0;
}

/// `.rodata` defaults: the up vector and the second ground normal.
const RODATA_UP: u32 = 0x004f_f160;
const RODATA_NORMAL2: u32 = 0x004f_f120;

const PG_FILL: u32 = 0x0013_d2f0;
const PG_PICK: u32 = 0x0013_d4a0;
const PG_COPY: u32 = 0x0013_d5a8;

/// `rider = state->rider` (`lw reg, 0x18(s1)`).
#[inline(always)]
fn ground_rider(c: &mut Cpu, reg: usize) -> u32 {
    c.lw_r(reg, at(c, S1, 0x18));
    r(c, reg)
}

/// Re-orthonormalise the rider frame on its up vector (`a1 = rider`):
/// `fwd = normalize(ref - up * dot(ref, up))`, `side = up x fwd`.
fn rebuild_rider_frame(c: &mut Cpu) {
    let s = sp(c);
    let rd = ground_rider(c, A1);
    c.qcopy(V1, p(rd, rider::FWD_REF), p(s, 0x2100));
    c.qcopy(A0, p(rd, rider::FWD_REF), p(s, 0x2120));
    c.vdot(p(s, 0x2120), p(rd, rider::UP), V1, 0);
    c.mfc1(A0, 0);
    c.vscale(p(rd, rider::UP), A0);
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x2110)]);
    c.vsub(p(s, 0x2100), p(s, 0x2110));
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x40)]);
    c.vlen2(p(s, 0x40));
    c.vnorm();
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x30), p(rd, rider::FRAME_FWD)]);
    let rd = ground_rider(c, A0);
    c.lqc2(4, p(rd, rider::UP));
    c.lqc2(5, p(rd, rider::FRAME_FWD));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.vstore(6, p(s, 0x40), V1, &[p(s, 0x30), p(rd, rider::FRAME_SIDE)]);
}

/// `bool RiderGroundState_ProbeGround(RiderGroundState* g, Contact* out,
/// float* height)` (0x13d1b8): casts a segment along the rider's up axis
/// (through the position shifted sideways by `speed * 45 * k`, from 100
/// below to 200 above, radius 0.5) against the rider's overlap list. No contact: resets the up vector and ground
/// normal to the defaults, rebuilds the frame and returns false with
/// `*height = 0`. Otherwise picks the contact whose normal is closest to
/// the floor (steep enough, depth nearest 0.5, ties by surface material /
/// instance id), copies it to `out`, records ground id, UV and flags,
/// takes its normal as the new up vector and returns true with the height
/// of the rider above it.
pub fn rider_ground_state_probe_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 78;
        prologue(
            c,
            0x2190,
            &[(S0, 0x2180), (S1, 0x2170), (S2, 0x2160), (S3, 0x2150)],
            0x2140,
            &[],
        );
        let s = sp(c);
        c.g[AT][0] = 0x4234_0000; // 45.0
        c.mtc1(AT, 3);
        c.g[A3][0] = 2;
        c.set_r(S0, p(s, 0x50));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[AT][0] = 0xffff_ffff_c2c8_0000; // -100.0
        c.mtc1(AT, 2);
        c.set_r(A1, p(s, 0x10));
        let rd = ground_rider(c, V1);
        c.set_r(A2, p(s, 0x20));
        c.g[AT][0] = 0x4348_0000; // 200.0
        c.mtc1(AT, 4);
        c.g[A0][0] = c.g[S0][0];
        c.lq(V0, p(rd, rider::UP));
        c.g[AT][0] = 0x3f00_0000; // 0.5
        c.mtc1(AT, 12);
        c.sq(V0, p(rd, rider::UP_PREV));
        // start = pos + side * (speed * 45 * k)
        let rd = ground_rider(c, V1);
        c.lwc1(1, p(rd, 0x1f0));
        c.lw_r(V0, p(rd, 0x780));
        c.mul_s(1, 1, 3);
        c.lwc1(0, at(c, V0, 0x140));
        c.mul_s(0, 0, 1);
        c.mfc1(T0, 0);
        c.vscale(p(rd, rider::FRAME_SIDE), T0);
        c.vstore(5, p(s, 0x20), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, p(s, 0x10));
        c.vadd(p(rd, rider::POS), p(s, 0x10));
        c.vstore(5, p(s, 0x20), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, s);
        // a = start + up * -100
        c.mfc1(T0, 2);
        c.vscale(p(rd, rider::UP), T0);
        c.vstore(5, p(s, 0x30), V0, &[p(s, 0x20)]);
        c.vadd(s, p(s, 0x20));
        c.vstore(5, p(s, 0x30), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, p(s, 0x10));
        // b = start + up * 200
        c.mfc1(T0, 4);
        c.vscale(p(rd, rider::UP), T0);
        c.vstore(5, p(s, 0x40), V0, &[p(s, 0x30)]);
        c.vadd(s, p(s, 0x30));
        c.vstore(5, p(s, 0x40), V0, &[]);
        c.g[RA][0] = sx(0x0013_d2e0);
        c.sq(V0, p(s, 0x20));
        // CollSegment_ctor(&seg, &a, &b, 2, 0.5)
        call_ptr(c, callee::COLL_SEGMENT_CTOR, 0x0013_d2e0)?;
        c.g[A1][0] = c.g[S0][0];
        c.set_r(A2, p(sp(c), 0x100));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x3f;
        // The (empty) constructor loop of the 64 contact records.
        loop {
            c.cycles += 7;
            c.inc_r(V0, u32::MAX);
            c.g[A3][0] = 0x40;
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, PG_FILL);
            }
        }
        // n = OverlapList_CollideVolume(rider->list, &seg, contacts, 64, rider->hint)
        c.cycles += 7;
        let rd = ground_rider(c, V0);
        c.lw_r(T0, p(rd, rider::OVERLAP_HINT));
        c.lw_r(A0, p(rd, rider::OVERLAP_LIST));
        call_ptr(c, callee::OVERLAP_LIST_COLLIDE_VOLUME, 0x0013_d31c)?;
        c.g[T3][0] = c.g[V0][0];
        if c.g[T3][0] == 0 {
            probe_ground_miss(c);
        } else {
            ground_rider(c, V1);
            probe_ground_pick(c);
            probe_ground_hit(c)?;
        }
        c.cycles += 7;
        epilogue(
            c,
            &[(S0, 0x2180), (S1, 0x2170), (S2, 0x2160), (S3, 0x2150)],
            &[],
            0x2140,
            0x2190,
        );
        Ok(())
    })
}

/// No ground: default up and ground normal, frame rebuilt, `*height = 0`.
fn probe_ground_miss(c: &mut Cpu) {
    c.cycles += 65;
    c.g[V0][0] = 0x50_0000;
    let rd = ground_rider(c, A2);
    c.set_r(V0, RODATA_UP);
    c.g[A1][0] = 0x50_0000;
    c.lq(A0, RODATA_UP);
    c.set_r(A1, RODATA_NORMAL2);
    c.g[V0][0] = 0;
    c.sq(A0, p(rd, rider::UP));
    c.lq(V1, RODATA_NORMAL2);
    let rd = ground_rider(c, A0);
    c.sq(V1, p(rd, rider::GROUND_NORMAL2));
    rebuild_rider_frame(c);
    let rd = ground_rider(c, A1);
    c.sw(p(rd, rider::GROUND_MATERIAL), 0);
    let out = r(c, S3);
    c.sw(out, 0);
}

/// Pick the best contact into t1 (t3 contacts at `sp+0x100`, v1 = rider).
fn probe_ground_pick(c: &mut Cpu) {
    let s = sp(c);
    // Contact 0: f1 = n.up, f2 = |depth - 0.5|, t4 = steep enough (f4 <= f1).
    c.cycles += 20;
    c.g[T1][0] = 0;
    c.lwc1(4, gp(c, -0x7068));
    c.vdot(p(s, 0x110), at(c, V1, rider::UP), V0, 1);
    c.lwc1(0, p(s, 0x140));
    c.g[T4][0] = 1;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 3);
    c.c_le_s(4, 1);
    c.sub_s(0, 0, 3);
    c.abs_s(2, 0);
    if !c.fcc() {
        c.g[T4][0] = 0;
    }
    c.cycles += 4;
    c.g[A3][0] = 1;
    c.g[V0][0] = slt(c, A3, T3);
    c.set_r(A2, p(s, 0x180));
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 5;
    c.mov_s(5, 3);
    c.g[T2][0] = c.g[V1][0];
    c.mov_s(3, 4);
    c.set_r(T0, p(s, 0x1c0));
    loop {
        // f1 = n.up, f0 = |depth - 0.5|
        c.cycles += 14;
        c.lwc1(0, r(c, T0));
        c.vdot(at(c, A2, contact::NORMAL), at(c, T2, rider::UP), V0, 1);
        c.sub_s(0, 0, 5);
        c.abs_s(0, 0);
        let mut advanced = false;
        let mut decided = false;
        if c.g[T4][0] != 0 {
            // A steep-enough best: skip contacts that are not.
            c.cycles += 4;
            c.c_lt_s(1, 3);
            if c.fcc() {
                c.inc_r(A3, 1);
                advanced = true;
                decided = true;
            }
        } else {
            c.cycles += 3;
            c.c_le_s(3, 1);
            if c.fcc() {
                // The first steep-enough contact replaces a flat best.
                c.cycles += 4;
                c.mov_s(2, 0);
                c.g[T1][0] = c.g[A3][0];
                c.g[T4][0] = 1;
                decided = true;
            }
        }
        if !decided {
            c.cycles += 3;
            c.c_lt_s(0, 2);
            if c.fcc() {
                c.cycles += 3;
                c.mov_s(2, 0);
                c.g[T1][0] = c.g[A3][0];
            } else {
                c.cycles += 4;
                c.c_eq_s(0, 2);
                let tie = c.fcc();
                c.set_r(V0, r(c, T1) << 7);
                if tie {
                    contact_tie_break(c, A2, 0x100);
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[T1][0] = c.g[A3][0];
                    }
                }
            }
        }
        if !advanced {
            c.cycles += 1;
            c.inc_r(A3, 1);
        }
        c.cycles += 4;
        c.inc_r(A2, contact::SIZE);
        c.g[V0][0] = slt(c, A3, T3);
        c.inc_r(T0, contact::SIZE);
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, PG_PICK);
        }
    }
}

/// Equal-score tie break between the current contact (`cur_reg`) and the
/// best one (`v0 = best << 7`, records at `sp + base`): leaves v0 = 1 when
/// the current one wins (it has a surface and the best has none, or a
/// lower surface id; without surfaces, a lower instance id).
fn contact_tie_break(c: &mut Cpu, cur_reg: usize, base: u32) {
    c.cycles += 4;
    c.lw_r(A0, at(c, cur_reg, contact::SURFACE));
    let best = r(c, V0).wrapping_add(sp(c));
    c.set_r(V1, best);
    c.set_r(V1, best.wrapping_add(base));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(V0, at(c, V1, contact::SURFACE));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            return;
        }
        c.lw_r(V1, at(c, A0, 0x78));
        c.cycles += 3;
        c.lw_r(V0, at(c, V0, 0x78));
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        return;
    }
    c.cycles += 3;
    c.lw_r(V0, at(c, V1, contact::SURFACE));
    let best_has_surface = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if best_has_surface {
        return;
    }
    c.cycles += 5;
    c.lw_r(V1, at(c, V1, contact::INSTANCE));
    c.lw_r(A0, at(c, cur_reg, contact::INSTANCE));
    c.lw_r(A1, at(c, V1, 0x150));
    c.lw_r(V0, at(c, A0, 0x150));
    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
}

/// Copy a 0x80-byte contact record (`v0` from, `v1` to, `a0` end) in four
/// 0x20 steps through a1/a2, with the loop's event checks at `head`.
fn copy_contact(c: &mut Cpu, head: u32) {
    loop {
        c.cycles += 7;
        let (from, to) = (r(c, V0), r(c, V1));
        c.lq(A1, from);
        c.lq(A2, p(from, 0x10));
        c.sq(A1, to);
        c.sq(A2, p(to, 0x10));
        c.inc_r(V0, 0x20);
        let more = c.g[V0][0] != c.g[A0][0];
        c.inc_r(V1, 0x20);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The best contact (t1) to `out`, ground id / UV / flags / material,
/// owner notified, up vector and frame from it, height above it.
fn probe_ground_hit(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let s = sp(c);
    let best = p(s, r(c, T1) << 7);
    c.set_r(V0, r(c, T1) << 7);
    c.g[V1][0] = c.g[S2][0];
    c.set_r(V0, best);
    c.set_r(A0, p(best, 0x180));
    c.set_r(V0, p(best, 0x100));
    copy_contact(c, PG_COPY);
    let out = r(c, S2);
    // Ground id and UV from the instance, or no id.
    c.cycles += 3;
    c.lw_r(V0, p(out, contact::INSTANCE));
    if c.g[V0][0] == 0 {
        let rd = ground_rider(c, V1);
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        c.sw_r(p(rd, rider::GROUND_ID), V0);
    } else {
        c.cycles += 14;
        c.lw_r(V1, at(c, V0, 0x150));
        let rd = ground_rider(c, V0);
        c.sw_r(p(rd, rider::GROUND_ID), V1);
        let rd = ground_rider(c, A0);
        c.lwc1(0, p(out, 0x6c));
        c.swc1(0, p(rd, 0xaac));
        let rd = ground_rider(c, V0);
        c.lwc1(0, p(out, 0x70));
        c.swc1(0, p(rd, 0xab0));
        c.lw_r(V1, p(out, contact::INSTANCE));
        let rd = ground_rider(c, A0);
        c.g[V0][0] = c.lhu(at(c, V1, 0xa));
        let flags = c.g[V0][0];
        c.sh(p(rd, 0x2d4), flags);
    }
    // Tell the surface's owner (virtual 0x150).
    c.cycles += 3;
    c.lw_r(A0, p(out, contact::SURFACE));
    if c.g[A0][0] == 0 {
        ground_rider(c, V1);
    } else {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S2][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0013_d640)?;
        }
        c.cycles += 1;
        ground_rider(c, V1);
    }
    // Material (-1: none).
    c.cycles += 7;
    c.g[A0][0] = u64::MAX;
    c.lw_r(V0, p(out, 0x4c));
    c.sw_r(at(c, V1, rider::GROUND_MATERIAL), V0);
    let rd = ground_rider(c, V1);
    c.lw_r(V0, p(rd, rider::GROUND_MATERIAL));
    if c.g[V0][0] == c.g[A0][0] {
        c.sw(p(rd, rider::GROUND_MATERIAL), 0);
    }
    // Up = contact normal, ground point and second normal; frame; height.
    c.cycles += 103;
    let s = sp(c);
    let rd = ground_rider(c, A1);
    c.g[V0][0] = 1;
    c.qcopy(V1, p(out, 0x10), p(rd, rider::UP));
    let rd = ground_rider(c, A0);
    c.qcopy(V1, out, p(rd, rider::GROUND_POINT));
    c.lq(V1, p(out, 0x20));
    let rd = ground_rider(c, A2);
    c.sq(V1, p(rd, rider::GROUND_NORMAL2));
    rebuild_rider_frame(c);
    c.vsub(s, out);
    c.sqc2(5, p(s, 0x40));
    c.lq(V1, p(s, 0x40));
    let rd = ground_rider(c, A0);
    c.sq(V1, p(s, 0x30));
    c.vdot(p(s, 0x30), p(rd, rider::UP), A1, 0);
    c.swc1(0, p(rd, rider::GROUND_HEIGHT));
    let rd = ground_rider(c, V1);
    c.lwc1(0, p(rd, rider::GROUND_HEIGHT));
    c.mfc1(A1, 0);
    c.vscale(p(rd, rider::UP), A1);
    c.vstore(5, p(s, 0x2110), A0, &[p(s, 0x2100)]);
    c.vsub(p(s, 0x30), p(s, 0x2100));
    c.vstore(5, p(s, 0x2110), V1, &[p(s, 0x40)]);
    c.vlen(p(s, 0x40), A0, 0);
    let height = r(c, S3);
    c.swc1(0, height);
    Ok(())
}

// ---------------------------------------------------------------------------
// Walls.

const FW_FILL: u32 = 0x0010_4ea8;
const FW_PICK: u32 = 0x0010_5008;
const FW_SEEN: u32 = 0x0010_5040;
const FW_COPY: u32 = 0x0010_522c;
const FW_SUM: u32 = 0x0010_5270;

/// The rider's list of wall surface ids touched this frame (`-1`
/// terminated).
const RIDER_WALL_IDS: u32 = 0x5b8;

/// `float Rider_FindWallContact(Rider* r, CollHull* hull, Contact* out)`
/// (0x104e70): collides the hull with the rider's overlap list (up to 64
/// contacts). None: returns -1. Otherwise keeps the touched wall surfaces
/// in the rider's id list, picks the best wall contact (real walls whose
/// normal faces the sphere tree first, then the depth closest to the
/// hull's reference depth, ties by surface / instance id), copies it to
/// `out`, averages the normals of all wall contacts into `out->normal`
/// when there are several, tells the surface's owner, and returns the
/// contact depth.
pub fn rider_find_wall_contact(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        prologue(
            c,
            0x20a0,
            &[
                (S0, 0x2090),
                (S1, 0x2080),
                (S2, 0x2070),
                (S5, 0x2040),
                (S3, 0x2060),
                (S4, 0x2050),
            ],
            0x2030,
            &[],
        );
        c.g[V0][0] = 0x3f;
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A2][0];
        // The (empty) constructor loop of the 64 contact records.
        loop {
            c.cycles += 7;
            c.inc_r(V0, u32::MAX);
            c.g[A1][0] = c.g[S0][0];
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, FW_FILL);
            }
        }
        // n = OverlapList_CollideObjects(r->list, hull, contacts, 64, 0)
        c.cycles += 8;
        c.lw_r(A0, at(c, S2, rider::OVERLAP_LIST));
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x40;
        c.g[T0][0] = 0;
        call_ptr(c, callee::OVERLAP_LIST_COLLIDE_OBJECTS, 0x0010_4ed8)?;
        c.g[S4][0] = c.g[V0][0];
        let s = sp(c);
        c.lw_r(V1, p(s, contact::SURFACE));
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 0);
            c.lq(S0, p(s, 0x2090));
        } else {
            find_wall_pick(c)?;
            c.cycles += 1;
            let s = sp(c);
            c.lq(S0, p(s, 0x2090));
        }
        c.cycles += 8;
        epilogue(
            c,
            &[(S1, 0x2080), (S2, 0x2070), (S3, 0x2060), (S4, 0x2050), (S5, 0x2040)],
            &[],
            0x2030,
            0x20a0,
        );
        Ok(())
    })
}

/// The contacts are in (s4 of them at `sp`, v1 = contact 0's surface).
fn find_wall_pick(c: &mut Cpu) -> Flow {
    let s = sp(c);
    // Contact 0's surface starts the id list.
    c.cycles += 2;
    c.set_r(V0, r(c, S1) << 2);
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.lw_r(V1, at(c, V1, 0x78));
        c.set_r(V0, r(c, V0).wrapping_add(r(c, S2)));
        c.g[S1][0] = 1;
        c.sw_r(at(c, V0, RIDER_WALL_IDS), V1);
    }
    // f0 = hull->vt[0x18](hull): the reference depth.
    c.cycles += 22;
    c.lw_r(V0, at(c, S0, 0x50));
    c.g[S3][0] = 0;
    vcall(c, S0, V0, 0x18, V1, 0x0010_4f24)?;
    c.lw_r(V1, p(s, contact::MATERIAL));
    c.g[V0][0] = 0xc;
    c.lw_r(A0, p(s, contact::SURFACE));
    c.mov_s(3, 0);
    c.mult(0, V1, V1, V0);
    c.lwc1(0, p(s, contact::DEPTH));
    c.lw_r(A1, at(c, A0, 0x88));
    c.sub_s(0, 0, 3);
    c.g[A0][0] = 0;
    c.abs_s(2, 0);
    c.inc_r(V1, 0x10);
    c.set_r(A1, r(c, A1).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A1, 0x8)) & 1;
    if c.g[V0][0] != 0 {
        // A wall material with a non-zero first field.
        c.cycles += 6;
        c.lwc1(1, r(c, A1));
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        if !c.fcc() {
            c.g[A0][0] = 1;
        }
        c.cycles += 1;
    }
    c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
    // a0 = dot(contact0.pos - tree.center, contact0.normal) < 0
    c.cycles += 22;
    c.g[T4][0] = c.g[A0][0];
    c.vsub(s, at(c, V0, 0x10));
    c.sqc2(5, p(s, 0x2010));
    c.mtc1(0, 1);
    c.lq(V0, p(s, 0x2010));
    c.g[A0][0] = 1;
    c.sq(V0, p(s, 0x2000));
    c.vdot(p(s, 0x2000), p(s, 0x10), V1, 0);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        c.g[A0][0] = 0;
    }
    // Contacts 1..n.
    c.cycles += 5;
    c.g[T1][0] = 1;
    c.g[T3][0] = c.g[A0][0];
    c.g[V0][0] = slt(c, T1, S4);
    c.g[T6][0] = ((c.g[S4][0] as i64) < 2) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.set_r(V0, r(c, S1) << 2);
        c.mov_s(4, 1);
        c.inc_r(V0, RIDER_WALL_IDS);
        c.g[T7][0] = 0xc;
        c.set_r(T2, r(c, V0).wrapping_add(r(c, S2)));
        c.set_r(V0, r(c, T1) << 7);
        loop {
            find_wall_pick_one(c);
            c.cycles += 3;
            c.g[V0][0] = slt(c, T1, S4);
            let more = c.g[V0][0] != 0;
            c.set_r(V0, r(c, T1) << 7);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, FW_PICK);
            }
        }
    }
    // *out = contacts[best]
    c.cycles += 5;
    let s = sp(c);
    c.set_r(V0, r(c, S3) << 7);
    c.set_r(T1, r(c, S1) << 2);
    c.set_r(V0, r(c, V0).wrapping_add(s));
    c.g[V1][0] = c.g[S5][0];
    c.set_r(A0, p(r(c, V0), contact::SIZE));
    copy_contact(c, FW_COPY);
    // Several contacts: the normalised sum of the wall normals.
    c.cycles += 2;
    let single = c.g[T6][0] != 0;
    c.g[V0][0] = 0x50_0000;
    if !single {
        c.cycles += 4;
        c.set_r(V0, RODATA_NORMAL2);
        c.qcopy(V1, RODATA_NORMAL2, p(s, 0x2000));
        if (c.g[S4][0] as i64) > 0 {
            c.cycles += 4;
            c.mtc1(0, 1);
            c.g[A2][0] = c.g[S4][0];
            c.g[T0][0] = 0xc;
            c.set_r(A1, p(s, 0x10));
            loop {
                c.cycles += 10;
                c.lw_r(V1, at(c, A1, contact::MATERIAL - contact::NORMAL));
                c.lw_r(V0, at(c, A1, contact::SURFACE - contact::NORMAL));
                c.mult(0, V1, V1, T0);
                c.lw_r(A0, at(c, V0, 0x88));
                c.inc_r(V1, 0x10);
                c.set_r(A0, r(c, A0).wrapping_add(r(c, V1)));
                c.g[V0][0] = c.lhu(at(c, A0, 0x8)) & 1;
                c.g[A3][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A0));
                    c.c_eq_s(0, 1);
                    if !c.fcc() {
                        c.g[A3][0] = 1;
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.cycles += 5;
                    c.vadd(p(s, 0x2000), r(c, A1));
                    c.sqc2(5, p(s, 0x2000));
                }
                c.inc_r(A2, u32::MAX);
                c.cycles += 2;
                let more = c.g[A2][0] != 0;
                c.inc_r(A1, contact::SIZE);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, FW_SUM);
                }
            }
        }
        c.cycles += 14;
        c.lwc1(0, gp(c, -0x7f38));
        c.vlen(p(s, 0x2000), V0, 1);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 10;
            c.mfc1(V1, 1);
            c.vdiv(p(s, 0x2000), V1);
            let out = r(c, S5);
            c.vstore(4, p(s, 0x2020), V0, &[p(out, contact::NORMAL), p(s, 0x2010)]);
        }
    }
    // Terminate the id list; tell the surface's owner (virtual 0x150).
    c.cycles += 7;
    c.g[V0][0] = u64::MAX;
    c.set_r(V1, r(c, T1).wrapping_add(r(c, S2)));
    c.sw_r(at(c, V1, RIDER_WALL_IDS), V0);
    let out = r(c, S5);
    c.lw_r(A0, p(out, contact::SURFACE));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S5][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0010_536c)?;
        }
        c.cycles += 1;
    }
    let out = r(c, S5);
    c.lwc1(0, p(out, contact::DEPTH));
    Ok(())
}

/// One contact (t1, at `sp + v0`) of the wall pick: records its surface id
/// if new, scores it (wall, facing, depth) against the best (s3).
fn find_wall_pick_one(c: &mut Cpu) {
    let s = sp(c);
    c.cycles += 5;
    c.g[T0][0] = c.g[V0][0];
    c.set_r(A0, s.wrapping_add(r(c, V0)));
    c.lw_r(V0, at(c, A0, contact::SURFACE));
    c.set_r(T5, r(c, T1).wrapping_add(1));
    if c.g[V0][0] != 0 {
        // Is the surface id already in the list?
        c.cycles += 2;
        let empty = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut known = false;
        if !empty {
            c.cycles += 4;
            c.lw_r(V1, at(c, V0, 0x78));
            c.lw_r(V0, at(c, S2, RIDER_WALL_IDS));
            let first = c.g[V0][0] == c.g[V1][0];
            c.set_r(A1, at(c, S2, RIDER_WALL_IDS));
            let mut found = first;
            if !first {
                c.cycles += 3;
                c.g[A3][0] = c.g[A0][0];
                c.inc_r(A2, 1);
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = slt(c, A2, S1);
                    let in_list = c.g[V0][0] != 0;
                    c.inc_r(A1, 4);
                    if !in_list {
                        break;
                    }
                    c.cycles += 5;
                    c.lw_r(V0, at(c, A3, contact::SURFACE));
                    c.lw_r(A0, r(c, A1));
                    c.lw_r(V1, at(c, V0, 0x78));
                    if c.g[A0][0] == c.g[V1][0] {
                        found = true;
                        break;
                    }
                    c.inc_r(A2, 1);
                    if event_due(c) {
                        deliver_event_here(c, FW_SEEN);
                    }
                }
            }
            if found {
                c.cycles += 3;
                c.g[V0][0] = slt(c, A2, S1);
                known = c.g[V0][0] != 0;
                c.set_r(V0, s.wrapping_add(r(c, T0)));
            }
        }
        if !known {
            c.cycles += 6;
            c.set_r(V0, s.wrapping_add(r(c, T0)));
            c.inc_r(S1, 1);
            c.lw_r(V1, at(c, V0, contact::SURFACE));
            c.lw_r(A0, at(c, V1, 0x78));
            c.sw_r(r(c, T2), A0);
            c.inc_r(T2, 4);
            c.cycles += 1;
            c.set_r(V0, s.wrapping_add(r(c, T0)));
        }
    } else {
        c.cycles += 1;
        c.set_r(V0, s.wrapping_add(r(c, T0)));
    }
    // a2 = a wall material with a non-zero first field; f1 = |depth - f3|.
    c.cycles += 14;
    c.g[A2][0] = 0;
    c.lw_r(V1, at(c, V0, contact::MATERIAL));
    c.lw_r(A0, at(c, V0, contact::SURFACE));
    c.mult(0, V1, V1, T7);
    c.lwc1(0, at(c, V0, contact::DEPTH));
    c.lw_r(A1, at(c, A0, 0x88));
    c.sub_s(0, 0, 3);
    c.abs_s(1, 0);
    c.inc_r(V1, 0x10);
    c.set_r(A1, r(c, A1).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A1, 0x8)) & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.lwc1(0, r(c, A1));
        c.c_eq_s(0, 4);
        if !c.fcc() {
            c.g[A2][0] = 1;
        }
    }
    // Walls beat non-walls (t4: the best is a wall).
    c.cycles += 2;
    if c.g[A2][0] != 0 {
        c.cycles += 2;
        if c.g[T4][0] == 0 {
            c.cycles += 4;
            c.g[S3][0] = c.g[T1][0];
            c.mov_s(2, 1);
            c.g[T4][0] = 1;
            pick_next(c);
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[T4][0] == 0 {
            c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
            find_wall_facing(c);
            return;
        }
    }
    c.cycles += 2;
    if c.g[A2][0] == 0 {
        c.g[T1][0] = c.g[T5][0];
        return;
    }
    c.cycles += 1;
    c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
    find_wall_facing(c);
}

/// `t1 = t5` (the next contact), through the shared block.
#[inline]
fn pick_next(c: &mut Cpu) {
    c.cycles += 1;
    c.g[T1][0] = c.g[T5][0];
}

/// The facing test (a1 = dot(pos - tree.center, normal) < 0) and the rest
/// of the scoring of one wall pick candidate (v0 = sphere tree).
fn find_wall_facing(c: &mut Cpu) {
    let s = sp(c);
    c.cycles += 21;
    c.set_r(V1, r(c, T0).wrapping_add(s));
    let cur = r(c, V1);
    c.vsub(cur, at(c, V0, 0x10));
    c.sqc2(5, p(s, 0x2010));
    c.g[A1][0] = 1;
    c.qcopy(V0, p(s, 0x2010), p(s, 0x2000));
    c.vdot(p(s, 0x2000), p(cur, contact::NORMAL), A0, 0);
    c.c_lt_s(0, 4);
    if !c.fcc() {
        c.g[A1][0] = 0;
    }
    // Facing beats not facing (t3: the best faces).
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        if c.g[T3][0] == 0 {
            c.cycles += 4;
            c.g[S3][0] = c.g[T1][0];
            c.mov_s(2, 1);
            c.g[T3][0] = 1;
            pick_next(c);
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[T3][0] == 0 {
            find_wall_depth(c);
            return;
        }
    }
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        c.g[T1][0] = c.g[T5][0];
        return;
    }
    find_wall_depth(c);
}

/// Depth closest to the reference wins (f1 against the best f2); ties by
/// surface / instance id.
fn find_wall_depth(c: &mut Cpu) {
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.g[S3][0] = c.g[T1][0];
        c.mov_s(2, 1);
        pick_next(c);
        return;
    }
    c.cycles += 4;
    c.c_eq_s(1, 2);
    let tie = c.fcc();
    let s = sp(c);
    c.set_r(V1, r(c, T0).wrapping_add(s));
    if tie {
        // v1 = current, a0 = best.
        c.cycles += 4;
        c.set_r(V0, r(c, S3) << 7);
        c.lw_r(A1, at(c, V1, contact::SURFACE));
        c.set_r(A0, r(c, V0).wrapping_add(s));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.lw_r(V0, at(c, A0, contact::SURFACE));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
            } else {
                c.lw_r(V1, at(c, A1, 0x78));
                c.cycles += 3;
                c.lw_r(V0, at(c, V0, 0x78));
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            }
        } else {
            c.cycles += 3;
            c.lw_r(V0, at(c, A0, contact::SURFACE));
            let best_has_surface = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !best_has_surface {
                c.cycles += 5;
                c.lw_r(V1, at(c, V1, contact::INSTANCE));
                c.lw_r(A0, at(c, A0, contact::INSTANCE));
                c.lw_r(A1, at(c, V1, 0x150));
                c.lw_r(V0, at(c, A0, 0x150));
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[S3][0] = c.g[T1][0];
        }
    }
    pick_next(c);
}

const WC_SAVE_CONTACT: u32 = 0x0010_56e0;

/// The global (`0x4c5830`) camera trigger manager.
const CAMERA_TRIGGER_MAN: u32 = 0x004c_5830;

/// `void Rider_HandleWallCollision(Rider* r, const Vec4* ground_normal)`
/// (0x105398): unless the rider is in sub-state 9, handles rail contacts
/// along walls, builds the rider's collision hull and finds the wall
/// contact. With a contact (depth >= 0): the push-out normal is flattened
/// onto the ground plane (when on the ground) or taken from the board when
/// grinding, the push-out depth is measured along the velocity, the
/// contact is saved in the rider (for walls that keep it), the rider
/// bounces off a bouncy wall and a non-silent wall plays the hit sound.
/// Always updates the camera triggers for the rider and destroys the hull.
pub fn rider_handle_wall_collision(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[(S2, 0x460), (S1, 0x470), (S0, 0x480), (S3, 0x450), (S4, 0x440)];
    guest(c, |c| {
        c.cycles += 15;
        prologue(c, 0x4a0, SAVED, 0x430, &[(21, 0x498), (20, 0x490)]);
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        callee::GET_SUB_STATE.call(c, 0x0010_53c8)?;
        c.g[V1][0] = 9;
        let skip = c.g[V0][0] == c.g[V1][0];
        let s = sp(c);
        c.lq(S0, p(s, 0x480));
        if !skip {
            wall_collision_body(c)?;
            // CameraTriggerMan_UpdateForRider(&cameraTriggers, r); CollHull_dtor(&hull, 2)
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, callee::CAMERA_TRIGGER_UPDATE_FOR_RIDER, 0x0010_5784)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 2;
            call_ptr(c, callee::COLL_HULL_DTOR, 0x0010_5790)?;
            let s = sp(c);
            c.lq(S0, p(s, 0x480));
        }
        c.cycles += 9;
        epilogue(
            c,
            &[(S1, 0x470), (S2, 0x460), (S3, 0x450), (S4, 0x440)],
            &[(21, 0x498), (20, 0x490)],
            0x430,
            0x4a0,
        );
        Ok(())
    })
}

/// Rail contacts, the hull, the wall contact and its response; ends with
/// a0 = the camera trigger manager.
fn wall_collision_body(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    c.mtc1(0, 20);
    c.g[A0][0] = c.g[S2][0];
    callee::HANDLE_WALL_RAIL_CONTACT.call(c, 0x0010_53e0)?;
    // CollHull_ctor(&hull, r->sphere_tree, 1)
    c.g[S3][0] = 0x4c_0000;
    let s = sp(c);
    c.set_r(S0, p(s, 0xc0));
    c.lw_r(A1, at(c, S2, rider::SPHERE_TREE));
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = 1;
    call_ptr(c, callee::COLL_HULL_CTOR, 0x0010_53f8)?;
    // depth = Rider_FindWallContact(r, &hull, &contact)
    c.g[S4][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0];
    callee::FIND_WALL_CONTACT.call(c, 0x0010_540c)?;
    c.mov_s(21, 0);
    c.c_le_s(20, 21);
    let hit = c.fcc();
    c.set_r(A0, CAMERA_TRIGGER_MAN);
    if !hit {
        return Ok(());
    }
    wall_push_normal(c)?;
    wall_push_depth(c);
    wall_response(c)
}

/// The push-out normal at `sp+0x10` (the contact normal): flattened onto
/// the ground plane on the ground, or the board normal when grinding.
fn wall_push_normal(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    callee::IS_ON_GROUND.call(c, 0x0010_5428)?;
    c.set_r(V1, at(c, S2, rider::UP));
    if c.g[V0][0] != 0 {
        c.g[S1][0] = c.g[V1][0];
    }
    let s = sp(c);
    if c.g[S1][0] != 0 {
        // n -= ground * dot(n, ground); renormalised when long enough.
        c.cycles += 36;
        let ground = r(c, S1);
        c.vdot(p(s, 0x10), ground, V0, 0);
        c.lwc1(1, gp(c, -0x7f34));
        c.mfc1(V1, 0);
        c.vscale(ground, V1);
        c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3f0)]);
        c.vsub(p(s, 0x10), p(s, 0x3f0));
        c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3e0)]);
        c.vlen(p(s, 0x3e0), V1, 0);
        c.c_lt_s(0, 1);
        if c.fcc() {
            // Too short: the ground normal itself.
            c.cycles += 3;
            c.qcopy(V0, ground, p(s, 0x10));
        } else {
            c.cycles += 11;
            c.mfc1(V1, 0);
            c.vdiv(p(s, 0x3e0), V1);
            c.vstore(4, p(s, 0x400), V0, &[p(s, 0x10), p(s, 0x3f0)]);
        }
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    callee::GET_MAIN_STATE.call(c, 0x0010_5508)?;
    c.g[V1][0] = 4;
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(());
    }
    // Grinding: the board normal, facing the contact normal.
    c.cycles += 13;
    let s = sp(c);
    c.lw_r(V1, at(c, S2, 0x77c));
    let board = at(c, V1, 0xb0);
    c.vdot(p(s, 0x10), board, V0, 0);
    c.c_lt_s(20, 0);
    if c.fcc() {
        c.cycles += 3;
        c.qcopy(V0, board, p(s, 0x10));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(board, V0);
        c.vstore(5, p(s, 0x3f0), V0, &[p(s, 0x10), p(s, 0x3e0)]);
    }
    Ok(())
}

/// f0 = how far the velocity runs into the wall: `max(dot(-n, contact -
/// vel), 0)`-style compares into f0 (as the original), v0 = `[sp]`.
fn wall_push_depth(c: &mut Cpu) {
    c.cycles += 29;
    let s = sp(c);
    let rd = r(c, S2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 1);
    c.mfc1(V0, 1);
    c.vscale(p(s, 0x10), V0);
    c.sqc2(5, p(s, 0x3f0));
    c.mtc1(0, 2);
    c.lq(V0, p(s, 0x3f0));
    c.sq(V0, p(s, 0x3e0));
    c.vsub(p(rd, rider::VEL), p(s, 0x20));
    c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3f0)]);
    c.vdot(p(s, 0x3e0), p(s, 0x3f0), V1, 0);
    c.c_le_s(2, 0);
    let ahead = c.fcc();
    c.mov_s(0, 2);
    if ahead {
        c.cycles += 24;
        c.mfc1(V1, 1);
        c.vscale(p(s, 0x10), V1);
        c.vstore(5, p(s, 0x410), V0, &[p(s, 0x400)]);
        c.vsub(p(rd, rider::VEL), p(s, 0x20));
        c.vstore(5, p(s, 0x420), V0, &[p(s, 0x410)]);
        c.vdot(p(s, 0x400), p(s, 0x410), V1, 0);
    } else {
        c.cycles += 1;
    }
    c.lq(V0, s);
}

/// Save the contact in the rider (walls with material bit 2, unless the
/// surface has flag 0x2000), bounce (material bit 1) and the hit sound
/// (unless flag 0x2000).
fn wall_response(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let s = sp(c);
    let rd = r(c, S2);
    c.g[V1][0] = 0xc;
    c.g[S3][0] = 0x4c_0000;
    c.sq(V0, p(s, 0x80));
    c.vlen2(p(rd, rider::VEL));
    c.vnorm();
    c.sqc2(5, p(s, 0x3f0));
    c.lq(A1, p(s, 0x3f0));
    c.lq(V0, p(s, 0x10));
    c.lw_r(A0, p(s, contact::MATERIAL));
    c.sq(A1, p(s, 0x90));
    c.mult(0, A0, A0, V1);
    c.sq(V0, p(s, 0xa0));
    c.swc1(0, p(s, 0xb0));
    c.sq(A1, p(s, 0x3e0));
    c.lw_r(A2, p(s, contact::SURFACE));
    c.lw_r(V1, at(c, A2, 0x88));
    c.set_r(A0, r(c, A0).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A0, 0x18)) & 2;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.lw_r(V0, at(c, A2, 0x8));
        c.g[V0][0] &= 0x2000;
        if c.g[V0][0] == 0 {
            // rider->wall_contact = contact (0x80), then the four
            // response vectors.
            c.cycles += 3;
            c.set_r(V1, p(rd, 0x9e0));
            c.g[V0][0] = c.g[SP][0];
            c.set_r(A0, p(s, 0x80));
            copy_contact(c, WC_SAVE_CONTACT);
            c.cycles += 9;
            c.lq(V0, p(s, 0x80));
            c.lq(V1, p(s, 0x90));
            c.lq(A0, p(s, 0xa0));
            c.lq(A1, p(s, 0xb0));
            c.sq(V0, p(rd, 0xa60));
            c.sq(V1, p(rd, 0xa70));
            c.sq(A0, p(rd, 0xa80));
            c.sq(A1, p(rd, 0xa90));
        }
    }
    c.lw_r(V1, p(s, contact::MATERIAL));
    // Bouncy wall: Rider_BounceOffWall(r, ?, &response, &contact, depth)
    c.cycles += 9;
    c.g[V0][0] = 0xc;
    c.lw_r(A1, p(s, contact::SURFACE));
    c.mult(0, V1, V1, V0);
    c.lw_r(A0, at(c, A1, 0x88));
    c.set_r(V1, r(c, V1).wrapping_add(r(c, A0)));
    c.g[V0][0] = c.lhu(at(c, V1, 0x18)) & 1;
    c.mov_s(12, 21);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.set_r(A2, p(s, 0x80));
        c.g[A3][0] = c.g[SP][0];
        callee::BOUNCE_OFF_WALL.call(c, 0x0010_5754)?;
    }
    // SSXAudioSystem_PlayWallHitSound(audio, r, ?, material)
    c.cycles += 5;
    let s = sp(c);
    c.lw_r(A2, p(s, contact::SURFACE));
    c.lw_r(V0, at(c, A2, 0x8));
    c.g[V0][0] &= 0x2000;
    let silent = c.g[V0][0] != 0;
    c.set_r(A0, CAMERA_TRIGGER_MAN);
    if !silent {
        c.cycles += 5;
        c.lw_r(A0, gp(c, 0x410));
        c.g[A1][0] = c.g[S2][0];
        c.lw_r(A3, p(s, contact::MATERIAL));
        call_ptr(c, callee::PLAY_WALL_HIT_SOUND, 0x0010_5778)?;
        c.set_r(A0, CAMERA_TRIGGER_MAN);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Rails.

/// The board's rail axis from the bone quaternion `q` at `s1` (`+0x10`
/// x, y, z, w): `(2(yw - zx)... )` as the compiler scheduled it, written
/// to `sp`, `sp+4`, `sp+8` (and `sp+0xc = 0`).
fn rail_axis_from_quat(c: &mut Cpu) {
    let (s, q) = (sp(c), r(c, S1));
    c.lwc1(5, p(q, 0x10));
    c.g[A1][0] = c.g[S1][0];
    c.lwc1(4, p(q, 0x18));
    c.g[A3][0] = 1;
    c.lwc1(0, p(q, 0x14));
    c.mul_s(8, 5, 5);
    c.lwc1(3, p(q, 0x1c));
    c.mul_s(1, 4, 4);
    c.mul_s(2, 0, 4);
    c.lw_r(A0, at(c, S0, rider::OVERLAP_LIST));
    c.mul_s(6, 3, 5);
    c.sw(p(s, 0xc), 0);
    c.add_s(1, 1, 8);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 4);
    c.add_s(2, 2, 6);
    c.add_s(1, 1, 1);
    c.sub_s(0, 0, 3);
    c.add_s(2, 2, 2);
    c.sub_s(7, 7, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, p(s, 0x8));
    c.swc1(7, p(s, 0x4));
    c.swc1(0, s);
}

/// `bool Rider_HandleWallRailContact(Rider* r)` (0x106f78): unless
/// grinding already, looks for a rail along the wall the board touches
/// (`OverlapList_FindNearestRail` with the board axis); when the rider is
/// slow or moving along the rail and close enough to it, and
/// `Rider_FindRail` finds none to jump on, the rider is pushed off the
/// rail, deflected along the wall and the impact handled. Returns whether
/// it was.
pub fn rider_handle_wall_rail_contact(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[(S0, 0x1f0), (S1, 0x1e0), (S2, 0x1d0), (S3, 0x1c0)];
    guest(c, |c| {
        c.cycles += 12;
        prologue(c, 0x210, SAVED, 0x1b0, &[(20, 0x200)]);
        c.g[S0][0] = c.g[A0][0];
        callee::GET_MAIN_STATE.call(c, 0x0010_6f9c)?;
        c.g[V1][0] = 4;
        let grinding = c.g[V0][0] == c.g[V1][0];
        c.set_r(A2, p(sp(c), 0x10));
        if grinding {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            wall_rail_contact_body(c)?;
        }
        c.cycles += 8;
        epilogue(c, SAVED, &[(20, 0x200)], 0x1b0, 0x210);
        Ok(())
    })
}

/// Everything after the state check; leaves v0 = the result.
fn wall_rail_contact_body(c: &mut Cpu) -> Flow {
    c.cycles += 38;
    let s = sp(c);
    c.lw_r(V1, at(c, S0, 0x780));
    c.lw_r(V0, at(c, S0, 0x89c));
    c.g[S3][0] = c.g[A2][0];
    c.lw_r(A0, at(c, V1, 0x2c));
    c.g[S2][0] = c.g[SP][0];
    c.set_r(V0, r(c, V0) << 5);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 7);
    c.set_r(S1, r(c, A0).wrapping_add(r(c, V0)));
    c.g[AT][0] = 0x4396_0000; // 300.0
    c.mtc1(AT, 12);
    rail_axis_from_quat(c);
    // OverlapList_FindNearestRail(r->list, bone, &contact, 1, axis, 300)
    call_ptr(c, callee::OVERLAP_LIST_FIND_NEAREST_RAIL, 0x0010_7038)?;
    let found = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !found {
        return Ok(());
    }
    let rd = r(c, S0);
    // Fast riders only when moving towards the rail.
    c.cycles += 14;
    c.lwc1(0, gp(c, -0x7ea4));
    c.vlen(p(rd, rider::VEL), V0, 2);
    c.c_lt_s(0, 2);
    if c.fcc() {
        c.cycles += 21;
        let bone = r(c, S1);
        c.vsub(p(s, 0x10), bone);
        c.sqc2(5, p(s, 0xa0));
        c.lwc1(1, gp(c, -0x7ea0));
        c.lq(V0, p(s, 0xa0));
        c.mul_s(1, 2, 1);
        c.sq(V0, p(s, 0x90));
        c.vdot(p(s, 0x90), p(rd, rider::VEL), V1, 0);
        c.c_lt_s(0, 1);
        let away = c.fcc();
        c.g[V0][0] = 0;
        if away {
            return Ok(());
        }
    }
    // Distance to the rail along the axis (clamped to +-50).
    c.cycles += 24;
    let bone = r(c, S1);
    let axis = r(c, S2);
    c.vsub(p(s, 0x10), bone);
    c.sqc2(5, p(s, 0xa0));
    c.g[AT][0] = 0xffff_ffff_c248_0000; // -50.0
    c.mtc1(AT, 0);
    c.lq(V0, p(s, 0xa0));
    c.mov_s(2, 0);
    c.g[AT][0] = 0x4248_0000; // 50.0
    c.mtc1(AT, 3);
    c.sq(V0, p(s, 0x90));
    c.vdot(axis, p(s, 0x90), V1, 1);
    c.c_le_s(0, 1);
    if c.fcc() {
        c.min_s(2, 1, 3);
    }
    c.cycles += 33;
    c.mfc1(V1, 2);
    c.vscale(axis, V1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
    c.vadd(bone, p(s, 0xb0));
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xa0)]);
    c.vsub(p(s, 0xa0), p(s, 0x10));
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0x90)]);
    c.vlen(p(s, 0x90), V1, 0);
    c.c_lt_s(3, 0);
    let far = c.fcc();
    c.g[V0][0] = 0;
    if far {
        return Ok(());
    }
    // A rail to jump on wins.
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    c.set_r(A1, p(s, 0xd0));
    callee::FIND_RAIL.call(c, 0x0010_71bc)?;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(());
    }
    wall_rail_push_off(c)
}

/// Push the rider off the rail: the contact normal facing the velocity,
/// the owner told, the push-out distance and direction, deflection and
/// impact. Returns with v0 = 1.
fn wall_rail_push_off(c: &mut Cpu) -> Flow {
    let s = sp(c);
    let rd = r(c, S0);
    // f20 = -|dot(vel, n)|, n flipped to face against the velocity.
    c.cycles += 14;
    c.mtc1(0, 0);
    c.vdot(p(rd, rider::VEL), p(s, 0x20), V0, 20);
    c.c_lt_s(20, 0);
    let towards = c.fcc();
    c.lw_r(A0, p(s, 0x60));
    if towards {
        c.cycles += 9;
        c.neg_s(20, 20);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0x20), V0);
        c.sqc2(5, p(s, 0x20));
        c.lw_r(A0, p(s, 0x60));
    }
    // contact.surface->owner->vt[0x150](owner, &contact)
    c.cycles += 2;
    c.lq(V0, p(s, 0x10));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S3][0];
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0010_7250)?;
            c.lq(V0, p(s, 0x10));
        }
    }
    // The impact record (sp+0x150..): contact point, velocity direction,
    // speed into the wall; the push-out vector at sp+0xa0.
    c.cycles += 46;
    c.g[A0][0] = c.g[S0][0];
    c.sq(V0, p(s, 0x150));
    c.vlen2(p(rd, rider::VEL));
    c.vnorm();
    c.sqc2(5, p(s, 0xb0));
    c.lq(V0, p(s, 0xb0));
    c.swc1(20, p(s, 0x180));
    c.sq(V0, p(s, 0xa0));
    c.sq(V0, p(s, 0x160));
    let bone = r(c, S1);
    c.vsub(bone, p(s, 0x10));
    c.vstore(5, p(s, 0xb0), V0, &[p(s, 0xa0)]);
    c.vdot(p(s, 0xa0), p(s, 0x20), V1, 0);
    c.mfc1(A1, 0);
    c.vscale(p(s, 0x20), A1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
    c.vsub(p(s, 0xa0), p(s, 0xb0));
    c.sqc2(5, p(s, 0xa0));
    callee::IS_ON_GROUND.call(c, 0x0010_7304)?;
    if c.g[V0][0] != 0 {
        // Flattened onto the ground plane.
        c.cycles += 20;
        c.vdot(p(s, 0xa0), p(rd, rider::UP), V0, 0);
        c.mfc1(V1, 0);
        c.vscale(p(rd, rider::UP), V1);
        c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
        c.vsub(p(s, 0xa0), p(s, 0xb0));
        c.sqc2(5, p(s, 0xa0));
    }
    // Normalised (or the contact normal when too short).
    c.cycles += 15;
    c.lwc1(0, gp(c, -0x7e9c));
    c.vlen(p(s, 0xa0), V0, 1);
    c.c_lt_s(1, 0);
    let short = c.fcc();
    c.lq(V0, p(s, 0x20));
    if short {
        c.cycles += 2;
        c.sq(V0, p(s, 0xa0));
    } else {
        c.cycles += 7;
        c.mfc1(V0, 1);
        c.qmtc2(V0, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, p(s, 0xa0));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, p(s, 0xa0));
    }
    // Translate out when closer than 50 along it.
    c.cycles += 17;
    c.lq(V0, p(s, 0xa0));
    c.g[AT][0] = 0x4248_0000; // 50.0
    c.mtc1(AT, 1);
    c.sq(V0, p(s, 0x170));
    c.vdot(p(s, 0x90), p(s, 0xa0), V1, 0);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A0][0] = c.g[S0][0];
    if inside {
        c.cycles += 10;
        c.sub_s(0, 1, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0xa0), V0);
        c.sqc2(5, p(s, 0xc0));
        c.set_r(A1, p(s, 0xb0));
        c.lq(V0, p(s, 0xc0));
        c.sq(V0, p(s, 0xb0));
        callee::TRANSLATE.call(c, 0x0010_7428)?;
    }
    // The velocity's component into the wall removed (plus a margin), the
    // per-frame push-out for the owner's virtual 0x158.
    c.cycles += 51;
    c.lq(V0, p(rd, rider::VEL));
    c.mtc1(0, 1);
    c.sq(V0, p(s, 0xb0));
    c.vsub(p(s, 0x30), p(rd, rider::VEL));
    c.sqc2(5, p(s, 0x190));
    c.lwc1(2, gp(c, -0x7e98));
    c.lq(V0, p(s, 0x190));
    c.sq(V0, p(s, 0xc0));
    c.vdot(p(s, 0xa0), p(s, 0xc0), V1, 0);
    c.max_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.mfc1(V1, 0);
    c.vscale(p(s, 0xa0), V1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0x190)]);
    c.vadd(p(rd, rider::VEL), p(s, 0x190));
    c.sqc2(5, p(rd, rider::VEL));
    c.vsub(p(s, 0xb0), p(rd, rider::VEL));
    c.sqc2(5, p(s, 0x1a0));
    c.lq(V0, p(s, 0x1a0));
    c.lw_r(V1, gp(c, -0x848));
    c.sq(V0, p(s, 0xc0));
    c.lwc1(0, at(c, V1, 0x10));
    c.cvt_s_w(0, 0);
    c.mfc1(A0, 0);
    c.vscale(p(s, 0xc0), A0);
    c.vstore(5, p(s, 0x1a0), V0, &[p(s, 0x190)]);
    c.lw_r(A0, p(s, 0x60));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A3, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S3][0];
        if c.g[A3][0] != 0 {
            c.cycles += 6;
            c.lw_r(V0, at(c, A3, 0xc));
            c.set_r(A2, p(s, 0x190));
            vcall(c, A3, V0, 0x158, V1, 0x0010_7518)?;
        }
    }
    // Not grinding: deflect along the wall and rebuild the matrix.
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    callee::GET_MAIN_STATE.call(c, 0x0010_7520)?;
    let state = c.g[V0][0];
    c.lw_r(A2, p(s, 0x5c));
    if state == 0 {
        c.cycles += 8;
        c.lq(V0, p(s, 0xa0));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.sq(V0, p(s, 0x20));
        callee::DEFLECT_ALONG_WALL.call(c, 0x0010_753c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::BUILD_WORLD_MATRIX.call(c, 0x0010_7544)?;
        c.lw_r(A2, p(s, 0x5c));
    }
    // Rider_HandleImpact(r, &impact)
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    c.set_r(A1, p(s, 0x150));
    callee::HANDLE_IMPACT.call(c, 0x0010_7554)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// `.rodata`: the board animation's rotation axis.
const RODATA_ANIM_AXIS: u32 = 0x004f_f130;

/// Rider rail fields.
mod rail {
    pub const KIND: u32 = 0x320;
    pub const DIR_FLAG: u32 = 0x324;
    pub const STANCE: u32 = 0x328;
    pub const SIDE: u32 = 0x330;
}

/// Two board axes from the bone quaternion at `s0` (+0x10..+0x1c), as the
/// compiler scheduled it: `sp+0x90..0x9c` (f22 = 0 from `sp+0x9c`) and
/// `sp+0xa0..0xac`.
fn railslide_board_axes(c: &mut Cpu) {
    let (s, q) = (sp(c), r(c, S0));
    c.lwc1(4, p(q, 0x14));
    c.lwc1(5, p(q, 0x18));
    c.lwc1(0, p(q, 0x10));
    c.mul_s(1, 4, 4);
    c.lwc1(3, p(q, 0x1c));
    c.mul_s(6, 5, 5);
    c.mul_s(2, 0, 5);
    c.sw(p(s, 0x9c), 0);
    c.mul_s(7, 3, 4);
    c.mul_s(3, 3, 5);
    c.lwc1(22, p(s, 0x9c));
    c.add_s(1, 1, 6);
    c.mul_s(0, 0, 4);
    c.sub_s(2, 2, 7);
    c.add_s(1, 1, 1);
    c.add_s(0, 0, 3);
    c.add_s(2, 2, 2);
    c.sub_s(1, 8, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, p(s, 0x98));
    c.swc1(1, p(s, 0x90));
    c.swc1(0, p(s, 0x94));
    c.lwc1(3, p(q, 0x10));
    c.lwc1(4, p(q, 0x14));
    c.lwc1(0, p(q, 0x18));
    c.mul_s(5, 3, 3);
    c.lwc1(2, p(q, 0x1c));
    c.mul_s(7, 4, 4);
    c.mul_s(1, 0, 4);
    c.sw(p(s, 0xac), 0);
    c.mul_s(6, 2, 3);
    c.add_s(5, 5, 7);
    c.mul_s(0, 0, 3);
    c.mul_s(2, 2, 4);
    c.sub_s(1, 1, 6);
    c.add_s(5, 5, 5);
    c.add_s(0, 0, 2);
    c.add_s(1, 1, 1);
    c.sub_s(8, 8, 5);
    c.add_s(0, 0, 0);
    c.swc1(1, p(s, 0xa4));
    c.swc1(8, p(s, 0xa8));
    c.swc1(0, p(s, 0xa0));
}

/// `bool Rider_TryStartRailslide(Rider* r)` (0x106848): when
/// `Rider_FindRail` finds a rail, puts the rider on it: the velocity along
/// the rail (speed clamped, vertical part damped), the owner told, the
/// board's angle to the rail picks the stance (`r+0x328`: 1-4, from the
/// trick state and the angle), tricks landed or the bank boost, main state
/// 4 (grinding), the sub-state and animation for the rail kind, the board
/// turned for a sideways stance, and the look direction set. Returns
/// whether the rider got on.
pub fn rider_try_start_railslide(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[(S3, 0xe0), (S0, 0x110), (S1, 0x100), (S2, 0xf0)];
    const FSAVED: &[(usize, u32)] = &[(23, 0x138), (22, 0x130), (21, 0x128), (20, 0x120)];
    guest(c, |c| {
        c.cycles += 15;
        prologue(c, 0x140, SAVED, 0xd0, FSAVED);
        c.g[A1][0] = c.g[SP][0];
        c.g[S3][0] = c.g[A0][0];
        callee::FIND_RAIL.call(c, 0x0010_687c)?;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if found {
            railslide_velocity(c)?;
            railslide_stance(c)?;
            railslide_enter(c)?;
        }
        c.cycles += 11;
        epilogue(c, SAVED, FSAVED, 0xd0, 0x140);
        Ok(())
    })
}

/// The rail direction (`sp+0x10`) along the velocity; the velocity along
/// it with the speed clamped (f23), the vertical part damped; the owner of
/// the rail told (virtual 0x158) with the velocity change.
fn railslide_velocity(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp(c), r(c, S3));
    c.cycles += 13;
    c.mtc1(0, 0);
    c.vdot(p(rd, rider::VEL), p(s, 0x10), V0, 12);
    c.c_lt_s(12, 0);
    if c.fcc() {
        c.cycles += 8;
        c.neg_s(12, 12);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0x10), V0);
        c.sqc2(5, p(s, 0x10));
    }
    c.cycles += 5;
    c.lwc1(1, gp(c, -0x7ed4));
    c.c_le_s(12, 1);
    let slow = c.fcc();
    c.mov_s(0, 12);
    if slow {
        c.cycles += 1;
        c.mov_s(0, 1);
    }
    c.cycles += 15;
    c.mov_s(12, 0);
    c.vlen(p(rd, rider::VEL), V0, 0);
    c.c_le_s(0, 12);
    let fast = c.fcc();
    c.mov_s(23, 12);
    if fast {
        c.cycles += 1;
        c.mov_s(23, 0);
    }
    c.cycles += 23;
    c.lq(V0, p(rd, rider::VEL));
    c.lwc1(1, gp(c, -0x7ed0));
    c.sq(V0, p(s, 0x80));
    c.mfc1(V1, 23);
    c.vscale(p(s, 0x10), V1);
    c.vstore(5, p(s, 0xa0), V0, &[p(rd, rider::VEL), p(s, 0x90)]);
    c.lwc1(0, p(rd, rider::VEL + 8));
    c.mul_s(0, 0, 1);
    c.swc1(0, p(rd, rider::VEL + 8));
    c.vsub(p(rd, rider::VEL), p(s, 0x80));
    c.sqc2(5, p(s, 0xa0));
    c.lq(V0, p(s, 0xa0));
    c.lw_r(A0, p(s, 0x50));
    c.sq(V0, p(s, 0x80));
    c.sq(V0, p(s, 0x90));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(T0, at(c, A0, 0xc));
        c.g[A1][0] = c.g[SP][0];
        if c.g[T0][0] != 0 {
            c.cycles += 15;
            c.lw_r(A3, at(c, T0, 0xc));
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0);
            c.set_r(A2, p(s, 0x90));
            c.mfc1(V1, 0);
            c.vscale(p(s, 0x80), V1);
            c.vstore(5, p(s, 0xa0), V0, &[p(s, 0x90)]);
            vcall(c, T0, A3, 0x158, V1, 0x0010_69d4)?;
        }
    }
    Ok(())
}

/// The board's angle to the rail (f20) and the stance (`r+0x328`); leaves
/// s2 = the previous stance, v0 as the original.
fn railslide_stance(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp(c), r(c, S3));
    c.cycles += 71;
    c.lw_r(V1, p(rd, 0x780));
    c.lw_r(V0, p(rd, 0x8a0));
    c.lw_r(A0, at(c, V1, 0x2c));
    c.set_r(V0, r(c, V0) << 5);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 8);
    c.set_r(S0, r(c, A0).wrapping_add(r(c, V0)));
    railslide_board_axes(c);
    c.vdot(p(s, 0x90), p(s, 0x10), V0, 20);
    c.vdot(p(s, 0xa0), p(s, 0x10), V1, 21);
    // f20 = atan2-style angle of the rail in the board frame.
    c.c_eq_s(20, 22);
    let along_zero = c.fcc();
    let mut angle_done = false;
    if along_zero {
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let side_zero = c.fcc();
        c.mov_s(20, 21);
        if side_zero {
            c.cycles += 1;
            c.lw_r(S2, p(rd, rail::STANCE));
        } else {
            c.cycles += 4;
            c.c_le_s(22, 21);
            c.lwc1(20, gp(c, -0x7ecc));
            let positive = c.fcc();
            c.lw_r(S2, p(rd, rail::STANCE));
            if positive {
                c.cycles += 3;
                c.lwc1(20, gp(c, -0x7ec8));
            }
        }
        angle_done = true;
    }
    if !angle_done {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, callee::MATH_ATAN, 0x0010_6b28)?;
        c.c_lt_s(20, 22);
        let behind = c.fcc();
        c.mov_s(1, 0);
        if behind {
            c.cycles += 3;
            c.c_lt_s(22, 21);
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp(c, -0x7ec4));
                c.add_s(1, 1, 0);
            } else {
                c.cycles += 2;
                c.lwc1(0, gp(c, -0x7ec0));
                c.sub_s(1, 1, 0);
            }
        }
        c.cycles += 1;
        c.mov_s(20, 1);
        c.cycles += 1;
        c.lw_r(S2, p(rd, rail::STANCE));
    }
    railslide_pick_stance(c)
}

/// Where the stance pick ends.
enum Stance {
    /// `sw s1 (1)`, then done.
    One,
    /// `sw v0` if FCC, else `v0 = 3` and the plain store.
    IfFcc,
    /// `sw v0`.
    Store,
    /// Already stored.
    Done,
}

fn railslide_pick_stance(c: &mut Cpu) -> Flow {
    let rd = r(c, S3);
    c.cycles += 5;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_SUB_STATE.call(c, 0x0010_6b68)?;
    c.g[S1][0] = 1;
    let end = if c.g[V0][0] == c.g[S1][0] {
        // Sub-state 1: kind 0 gives stance 1, else 2.
        c.cycles += 3;
        c.lw_r(V0, p(rd, rail::KIND));
        let kind0 = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if kind0 {
            Stance::One
        } else {
            c.cycles += 2;
            c.sw_r(p(rd, rail::STANCE), V0);
            Stance::Done
        }
    } else {
        c.lw_r(V0, p(rd, rail::SIDE));
        let mut end = None;
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.lwc1(1, gp(c, -0x7ebc));
            c.abs_s(0, 20);
            c.c_lt_s(1, 0);
            let wide = c.fcc();
            c.g[V0][0] = 2;
            end = Some(if !wide {
                Stance::One
            } else {
                c.cycles += 2;
                c.sw_r(p(rd, rail::STANCE), V0);
                Stance::Done
            });
        }
        match end {
            Some(e) => e,
            None => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                callee::GET_SUB_STATE.call(c, 0x0010_6bb8)?;
                c.g[V1][0] = 0xc;
                let sub12 = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 3;
                if sub12 {
                    Stance::Store
                } else {
                    railslide_stance_from_angle(c)
                }
            }
        }
    };
    match end {
        Stance::One => {
            c.cycles += 2;
            c.sw_r(p(rd, rail::STANCE), S1);
        }
        Stance::IfFcc => {
            c.cycles += 2;
            if c.fcc() {
                c.sw_r(p(rd, rail::STANCE), V0);
            } else {
                c.cycles += 1;
                c.g[V0][0] = 3;
                c.cycles += 1;
                c.sw_r(p(rd, rail::STANCE), V0);
            }
        }
        Stance::Store => {
            c.cycles += 1;
            c.sw_r(p(rd, rail::STANCE), V0);
        }
        Stance::Done => {}
    }
    Ok(())
}

/// The stance from the board's up axis (`sp+0xb0`) and the angle.
fn railslide_stance_from_angle(c: &mut Cpu) -> Stance {
    let (s, q) = (sp(c), r(c, S0));
    c.cycles += 26;
    c.lwc1(5, p(q, 0x18));
    c.lwc1(3, p(q, 0x10));
    c.lwc1(1, p(q, 0x14));
    c.mul_s(2, 5, 5);
    c.lwc1(4, p(q, 0x1c));
    c.mul_s(7, 3, 3);
    c.mul_s(0, 1, 5);
    c.sw(p(s, 0xbc), 0);
    c.mul_s(8, 4, 3);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 6);
    c.add_s(2, 2, 7);
    c.mul_s(1, 1, 3);
    c.lwc1(7, p(s, 0xbc));
    c.mul_s(4, 4, 5);
    c.add_s(0, 0, 8);
    c.add_s(2, 2, 2);
    c.sub_s(1, 1, 4);
    c.add_s(0, 0, 0);
    c.sub_s(6, 6, 2);
    c.add_s(1, 1, 1);
    c.c_lt_s(0, 7);
    c.swc1(0, p(s, 0xb8));
    c.swc1(6, p(s, 0xb4));
    let down = c.fcc();
    c.swc1(1, p(s, 0xb0));
    if down {
        c.cycles += 3;
        c.c_lt_s(21, 7);
        c.g[V0][0] = 4;
        return Stance::IfFcc;
    }
    c.cycles += 6;
    c.lwc1(0, gp(c, -0x7eb8));
    c.abs_s(1, 20);
    c.c_lt_s(0, 1);
    let wide = c.fcc();
    c.g[V0][0] = 2;
    if wide {
        return Stance::Store;
    }
    c.cycles += 5;
    c.lwc1(0, gp(c, -0x7eb4));
    c.c_lt_s(1, 0);
    let narrow = c.fcc();
    c.g[V0][0] = 4;
    if narrow {
        return Stance::One;
    }
    c.cycles += 3;
    c.lwc1(0, gp(c, -0x7eb0));
    c.c_lt_s(20, 0);
    Stance::IfFcc
}

/// Tricks, main state 4, sub-state, animation, the board turned for a
/// sideways stance, the look direction; v0 = 1.
fn railslide_enter(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp(c), r(c, S3));
    // Ground material from the rail contact; land a trick or the bank boost.
    c.cycles += 7;
    c.lw_r(V0, p(s, 0x4c));
    c.g[A0][0] = c.g[S3][0];
    c.sw_r(p(rd, rider::GROUND_MATERIAL), V0);
    callee::GET_MAIN_STATE.call(c, 0x0010_6c98)?;
    c.g[V1][0] = 1;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 9;
        c.lw_r(A2, p(rd, rail::STANCE));
        c.mov_s(12, 23);
        c.lw_r(A3, p(rd, rail::SIDE));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::BANK_TRICK_BOOST.call(c, 0x0010_6cc0)?;
        c.lw_r(V1, gp(c, 0x410));
    } else {
        c.lw_r(A1, p(rd, rail::KIND));
        c.cycles += 13;
        c.g[A2][0] = 0;
        c.lw_r(V0, p(rd, rail::DIR_FLAG));
        c.lw_r(A0, p(rd, 0x790));
        c.g[A1][0] = c.g[A1][0] ^ c.g[V0][0];
        c.lw_r(A3, p(rd, rail::STANCE));
        c.lw_r(T0, p(rd, rail::SIDE));
        c.g[A1][0] = (0 < c.g[A1][0]) as u64;
        callee::LAND_TRICK.call(c, 0x0010_6ce8)?;
        c.mov_s(12, 0);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 1;
        callee::ADD_BOOST.call(c, 0x0010_6cf8)?;
        c.lw_r(V1, gp(c, 0x410));
    }
    // audio->rail_material = material; Rider_SetMainState(r, 4)
    c.cycles += 9;
    c.g[A0][0] = c.g[S3][0];
    c.lw_r(V0, p(s, 0x4c));
    c.g[A1][0] = 4;
    c.sw_r(at(c, V1, 0x598c), V0);
    callee::SET_MAIN_STATE.call(c, 0x0010_6d10)?;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_SUB_STATE.call(c, 0x0010_6d18)?;
    let mut fresh = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S3][0];
    let mut sub_4_or_5 = false;
    if !fresh {
        c.cycles += 4;
        c.g[S0][0] = 4;
        callee::GET_SUB_STATE.call(c, 0x0010_6d28)?;
        let is4 = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = c.g[S3][0];
        if is4 {
            sub_4_or_5 = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            callee::GET_SUB_STATE.call(c, 0x0010_6d38)?;
            c.g[V1][0] = 5;
            let is5 = c.g[V0][0] == c.g[V1][0];
            c.g[A0][0] = c.g[S3][0];
            if is5 {
                sub_4_or_5 = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                callee::GET_SUB_STATE.call(c, 0x0010_6d4c)?;
                c.g[V1][0] = 0xb;
                if c.g[V0][0] != c.g[V1][0] {
                    c.lw_r(V0, p(rd, rail::STANCE));
                    railslide_turn_board(c)?;
                    let rd = r(c, S3);
                    c.lw_r(A0, p(rd, 0x77c));
                    return railslide_look(c);
                }
                fresh = true;
            }
        }
    }
    if fresh && !sub_4_or_5 {
        c.cycles += 1;
        c.g[A0][0] = c.g[S3][0];
    }
    // From a trick (sub-state 4/5) or fresh: sub-state 0xd, stance aligned.
    c.cycles += 5;
    c.g[S0][0] = 0;
    callee::GET_SUB_STATE.call(c, 0x0010_6d64)?;
    c.g[V1][0] = 5;
    let mut tricking = false;
    if c.g[V0][0] == c.g[V1][0] {
        c.g[S0][0] = 1;
        tricking = true;
    } else {
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        callee::GET_SUB_STATE.call(c, 0x0010_6d78)?;
        c.g[V1][0] = 4;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A0][0] = c.g[S3][0];
        } else {
            c.cycles += 1;
            c.g[S0][0] = 1;
            tricking = true;
        }
    }
    if tricking {
        c.cycles += 1;
        c.g[A0][0] = c.g[S3][0];
    }
    c.cycles += 7;
    c.g[A1][0] = 0xd;
    callee::SET_SUB_STATE.call(c, 0x0010_6d94)?;
    c.g[A0][0] = c.g[S3][0];
    callee::ALIGN_STANCE_TO_RAIL.call(c, 0x0010_6d9c)?;
    let rd = r(c, S3);
    c.lw_r(V1, p(rd, rail::SIDE));
    let centred = c.g[V1][0] == 0;
    c.g[V0][0] = 1;
    if centred {
        // The railslide enter animation (from a trick or not); sub-state 7.
        c.cycles += 9;
        c.lw_r(A0, p(rd, 0x77c));
        c.g[A1][0] = c.g[S0][0];
        c.inc_r(A0, 0x2b0);
        call_ptr(c, callee::RAILSLIDE_PLAY_ENTER_ANIM, 0x0010_6e04)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 7;
        callee::SET_SUB_STATE.call(c, 0x0010_6e10)?;
    } else {
        // Side animation (0x1a / 0x22); sub-state 1.
        c.cycles += 2;
        let left = c.g[V1][0] == c.g[V0][0];
        c.lw_r(A0, p(rd, 0x784));
        if left {
            c.cycles += 7;
            c.g[A1][0] = 0x1a;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_PLAY_ANIM, 0x0010_6dc4)?;
        } else {
            c.cycles += 6;
            c.g[A1][0] = 0x22;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_PLAY_ANIM, 0x0010_6de0)?;
        }
        c.g[A0][0] = c.g[S3][0];
        c.cycles += 4;
        c.g[A1][0] = 1;
        callee::SET_SUB_STATE.call(c, 0x0010_6dec)?;
    }
    let rd = r(c, S3);
    c.lw_r(A0, p(rd, 0x77c));
    railslide_look(c)
}

/// `RiderRailslideState_ComputeEntryAngle(r->state + 0xb0, &rail_dir)`;
/// v0 = 1.
fn railslide_look(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.set_r(A1, p(sp(c), 0x10));
    c.inc_r(A0, 0xb0);
    call_ptr(c, callee::RAILSLIDE_COMPUTE_ENTRY_ANGLE, 0x0010_6f44)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// A stance change to 3 or 4 (from s2, the previous one; v0 = the new
/// one) turns the board by +-90 degrees (gp-0x7eac / gp-0x7ea8) and its
/// animation; ends with the stance aligned to the rail.
fn railslide_turn_board(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let same = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 3;
    if same {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 2;
    let to3 = c.g[S2][0] == c.g[V0][0];
    let s = sp(c);
    c.set_r(A1, p(s, 0xb0));
    let turn = if !to3 {
        c.cycles += 2;
        let to4 = c.g[S2][0] == c.g[S0][0];
        c.g[S0][0] = 0x50_0000;
        if !to4 {
            None
        } else {
            c.cycles += 27;
            Some((-0x7ea8, 0x0010_6ec0u32, 0x0010_6eccu32, 0x0010_6ef4u32, 0xc8u32))
        }
    } else {
        c.cycles += 29;
        Some((-0x7eac, 0x0010_6e48, 0x0010_6e54, 0x0010_6e7c, 0xc0))
    };
    if let Some((angle, ret_rot, ret_yaw, ret_sincos, out)) = turn {
        let rd = r(c, S3);
        c.lq(V0, p(rd, 0x1c0));
        c.lwc1(20, gp(c, angle));
        c.g[A0][0] = c.g[S3][0];
        c.sq(V0, p(s, 0xb0));
        c.g[S0][0] = 0x50_0000;
        c.mov_s(12, 20);
        c.set_r(S1, RODATA_UP);
        callee::ROTATE_AROUND_AXIS.call(c, ret_rot)?;
        let rd = r(c, S3);
        c.lw_r(A0, p(rd, 0x784));
        c.mov_s(12, 20);
        call_ptr(c, callee::ANIM_ROTATE_ROOTS, ret_yaw)?;
        c.mtc1(0, 12);
        c.g[V0][0] = 0x50_0000;
        c.set_r(V0, RODATA_ANIM_AXIS);
        let rd = r(c, S3);
        c.lw_r(S2, p(rd, 0x784));
        c.lq(V1, RODATA_ANIM_AXIS);
        c.neg_s(12, 12);
        c.set_r(A0, p(s, out));
        c.set_r(A1, p(s, out + 4));
        let anim = r(c, S2);
        c.sq(V1, p(anim, 0x30));
        call_ptr(c, callee::MATH_SIN_COS, ret_sincos)?;
        // The rotation quaternion (axis * sin, cos) into the animation.
        let (s1, s0) = (r(c, S1), r(c, S0));
        c.lwc1(1, p(s1, 0x8));
        c.lwc1(0, p(s, out));
        c.lwc1(2, s0.wrapping_sub(0xea0));
        c.lwc1(4, p(s1, 0x4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, p(s, out + 4));
        c.cycles += 7;
        c.mul_s(0, 0, 4);
        c.swc1(3, p(s, 0xbc));
        c.swc1(1, p(s, 0xb8));
        c.swc1(2, p(s, 0xb0));
        c.swc1(0, p(s, 0xb4));
        c.lq(V0, p(s, 0xb0));
        let anim = r(c, S2);
        c.sq(V0, p(anim, 0x40));
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    callee::ALIGN_STANCE_TO_RAIL.call(c, 0x0010_6f34)?;
    c.cycles += 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider against rider.

const RC_LOOP: u32 = 0x0010_7948;

/// `FLAGS64_TEST(layer(anim, 1) + 0xb0, which)` for `anim = [base +
/// 0x784]` (two calls), the shape the original uses to test the rider's
/// animation layer.
fn anim_layer_is(c: &mut Cpu, rider_reg: usize, which: u64, ret_layer: u32, ret_is: u32) -> Flow {
    c.lw_r(A0, at(c, rider_reg, 0x784));
    c.g[A1][0] = 1;
    call_ptr(c, callee::ANIM_LAYER_SEQUENCE, ret_layer)?;
    c.set_r(A0, p(r(c, V0), 0xb0));
    c.g[A1][0] = which;
    call_ptr(c, callee::FLAGS64_TEST, ret_is)
}

/// `void Rider_CheckRiderCollisions(Rider* r)` (0x107888): for each of the
/// six other rider slots (`r + 0x24 * i` non-zero, the AI rider list at
/// `world+0x84`), unless that rider is out (`+0x878`): when the pair's
/// bump timer allows, their sphere trees are intersected and the riders
/// pushed apart (bump strengths, flattened on the ground) with the
/// relative velocity resolved; then, unless this rider is protected (`sp
/// +0x60`: its animation), when the other rider's knock-down timer allows,
/// is not tumbling and is close and in front, its attribute factor read and the
/// other rider knocked down along the push direction.
pub fn rider_check_rider_collisions(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S1, 0xf0),
        (S0, 0x100),
        (S2, 0xe0),
        (S3, 0xd0),
        (S4, 0xc0),
        (S5, 0xb0),
        (S6, 0xa0),
        (S7, 0x90),
        (FP, 0x80),
    ];
    const FSAVED: &[(usize, u32)] = &[(22, 0x120), (24, 0x130), (23, 0x128), (21, 0x118), (20, 0x110)];
    guest(c, |c| {
        c.cycles += 26;
        prologue(c, 0x140, SAVED, 0x70, FSAVED);
        c.g[S1][0] = c.g[A0][0];
        let s = sp(c);
        c.sw(p(s, 0x60), 0);
        callee::GET_BUMP_STRENGTH.call(c, 0x0010_78d4)?;
        // protected = layer-1 anim entry != 0xd || sequence flag 1 set || flag 0 clear
        c.lw_r(A0, at(c, S1, 0x784));
        c.mov_s(22, 0);
        c.g[A1][0] = 1;
        call_ptr(c, callee::ANIM_LAYER_ANIM_ENTRY, 0x0010_78e4)?;
        c.g[V1][0] = 0xd;
        let mut store = true;
        let other = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 1;
        if !other {
            c.cycles += 8;
            anim_layer_is(c, S1, 1, 0x0010_78fc, 0x0010_7908)?;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !set {
                c.cycles += 8;
                anim_layer_is(c, S1, 0, 0x0010_791c, 0x0010_7928)?;
                let set = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                store = !set;
            }
        }
        if store {
            c.cycles += 1;
            let s = sp(c);
            c.sw_r(p(s, 0x60), V0);
        }
        c.cycles += 5;
        c.lwc1(24, gp(c, -0x7e80));
        c.g[S7][0] = 0;
        c.lwc1(23, gp(c, -0x7e7c));
        c.g[FP][0] = 0x24;
        c.g[S6][0] = c.g[S1][0];
        loop {
            rider_pair(c)?;
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S7][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.inc_r(S6, 0x24);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, RC_LOOP);
            }
        }
        c.cycles += 17;
        epilogue(
            c,
            &[
                (S0, 0x100),
                (S1, 0xf0),
                (S2, 0xe0),
                (S3, 0xd0),
                (S4, 0xc0),
                (S5, 0xb0),
                (S6, 0xa0),
                (S7, 0x90),
                (FP, 0x80),
            ],
            &[(24, 0x130), (23, 0x128), (22, 0x120), (21, 0x118), (20, 0x110)],
            0x70,
            0x140,
        );
        Ok(())
    })
}

/// One rider slot (s7, s6); ends with s7 advanced.
fn rider_pair(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.lw_r(V0, r(c, S6));
    let empty = c.g[V0][0] == 0;
    c.lw_r(A0, gp(c, -0x848));
    if empty {
        c.cycles += 1;
        c.inc_r(S7, 1);
        return Ok(());
    }
    // s0 = the other rider (world->ai_riders->list[s7]->rider)
    c.cycles += 8;
    c.set_r(A1, r(c, S7) << 2);
    c.lw_r(V1, at(c, A0, 0x84));
    c.lw_r(V0, at(c, V1, 0xc));
    c.set_r(V0, r(c, V0).wrapping_add(r(c, A1)));
    c.lw_r(S0, at(c, V0, 0x28));
    c.lw_r(V1, at(c, S0, 0x878));
    if c.g[V1][0] != 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    rider_pair_bump(c)?;
    // v0 = protected
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    rider_pair_knock_down(c)
}

/// The pair's bump: timers (`+0x14` per pair entry, `r+0x86c` index * 0x24
/// into the other's table), sphere trees, push apart and resolve. Ends
/// with v0 = `[sp+0x60]`.
fn rider_pair_bump(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.lw_r(V0, at(c, S1, 0x86c));
    c.mult(0, V1, V0, FP);
    c.set_r(S5, r(c, V1).wrapping_add(r(c, S0)));
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7984)?;
    c.lw_r(V1, at(c, S5, 0x14));
    c.g[V1][0] = slt(c, V1, V0);
    let due = c.g[V1][0] != 0;
    let s = sp(c);
    c.lw_r(V0, p(s, 0x60));
    if !due {
        return Ok(());
    }
    c.cycles += 14;
    c.lw_r(V0, at(c, S0, 0x86c));
    c.mult(0, V1, V0, FP);
    c.set_r(S2, r(c, V1).wrapping_add(r(c, S1)));
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_79a4)?;
    c.sw_r(at(c, S2, 0x14), V0);
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_79ac)?;
    c.sw_r(at(c, S5, 0x14), V0);
    c.g[A2][0] = c.g[SP][0];
    c.set_r(A3, p(s, 0x10));
    c.lw_r(A0, at(c, S1, rider::SPHERE_TREE));
    c.lw_r(A1, at(c, S0, rider::SPHERE_TREE));
    call_ptr(c, callee::SPHERE_TREE_INTERSECT, 0x0010_79c4)?;
    let hit = c.g[V0][0] != 0;
    c.lw_r(V0, p(s, 0x60));
    if !hit {
        return Ok(());
    }
    // Push vectors scaled by the bump factors, flattened on the ground.
    c.cycles += 23;
    c.g[A0][0] = c.g[S0][0];
    callee::GET_BUMP_STRENGTH.call(c, 0x0010_79d4)?;
    c.mfc1(V1, 24);
    c.vscale(s, V1);
    c.sqc2(5, p(s, 0x30));
    c.mov_s(21, 0);
    c.lq(V0, p(s, 0x30));
    c.set_r(S3, p(s, 0x30));
    c.set_r(S4, p(s, 0x40));
    c.g[A0][0] = c.g[S1][0];
    c.sq(V0, p(s, 0x20));
    c.mfc1(V1, 23);
    c.vscale(s, V1);
    c.vstore(5, p(s, 0x40), V0, &[p(s, 0x30)]);
    callee::IS_ON_GROUND.call(c, 0x0010_7a20)?;
    if c.g[V0][0] != 0 {
        c.cycles += 20;
        let up = at(c, S1, rider::UP);
        flatten_on(c, p(s, 0x20), up);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    callee::IS_ON_GROUND.call(c, 0x0010_7a80)?;
    let grounded = c.g[V0][0] != 0;
    c.set_r(A1, p(s, 0x20));
    if grounded {
        c.cycles += 21;
        let up = at(c, S0, rider::UP);
        flatten_on(c, p(s, 0x30), up);
        c.set_r(A1, p(s, 0x20));
    }
    // Rider_Translate both; resolve when the pair's resolve timer allows.
    c.cycles += 12;
    c.g[A0][0] = c.g[S1][0];
    callee::TRANSLATE.call(c, 0x0010_7ae4)?;
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S0][0];
    callee::TRANSLATE.call(c, 0x0010_7af0)?;
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7af8)?;
    c.lw_r(V1, at(c, S2, 0x10));
    c.inc_r(V0, (-3i32) as u32);
    c.g[V1][0] = slt(c, V1, V0);
    let due = c.g[V1][0] != 0;
    c.lw_r(V0, p(s, 0x60));
    if !due {
        return Ok(());
    }
    // n = normalize(push); impulse = (v0.n - v1.n) / (k0 + k1)
    c.cycles += 52;
    c.vlen2(s);
    c.vnorm();
    c.sqc2(5, p(s, 0x50));
    c.add_s(1, 22, 21);
    c.lq(V0, p(s, 0x50));
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S4][0];
    c.sq(V0, p(s, 0x40));
    c.g[A3][0] = 0;
    let (v_other, v_self) = (at(c, S0, rider::VEL), at(c, S1, rider::VEL));
    c.vdot(v_other, p(s, 0x40), V0, 20);
    c.vdot(v_self, p(s, 0x40), V1, 0);
    c.sub_s(20, 20, 0);
    c.div_s(20, 20, 1);
    c.mul_s(12, 21, 20);
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7ba8)?;
    c.neg_s(12, 22);
    c.g[A2][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    c.mul_s(12, 12, 20);
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7bc4)?;
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7bcc)?;
    c.sw_r(at(c, S2, 0x10), V0);
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7bd4)?;
    c.sw_r(at(c, S5, 0x10), V0);
    let s = sp(c);
    c.lw_r(V0, p(s, 0x60));
    Ok(())
}

/// `[v] -= up * dot([v], up)` through `sp+0x40`/`sp+0x50`.
fn flatten_on(c: &mut Cpu, v: u32, up: u32) {
    let s = sp(c);
    c.vdot(v, up, V0, 0);
    c.mfc1(V1, 0);
    c.vscale(up, V1);
    c.vstore(5, p(s, 0x50), V0, &[p(s, 0x40)]);
    c.vsub(v, p(s, 0x40));
    c.sqc2(5, v);
}

/// Knock the other rider (s0) down when its timer (`s5+0x18`) allows, it
/// is not tumbling, is within range and in front.
fn rider_pair_knock_down(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7bec)?;
    c.lw_r(V1, at(c, S5, 0x18));
    c.inc_r(V0, (-3i32) as u32);
    c.g[V1][0] = slt(c, V1, V0);
    if c.g[V1][0] == 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    // Not while tumbling (layer-1 anim entry 3 with flag 0 set and flag 1 clear).
    c.cycles += 6;
    c.lw_r(A0, at(c, S0, 0x784));
    c.g[A1][0] = 1;
    call_ptr(c, callee::ANIM_LAYER_ANIM_ENTRY, 0x0010_7c0c)?;
    c.g[V1][0] = 3;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 8;
        anim_layer_is(c, S0, 0, 0x0010_7c24, 0x0010_7c30)?;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            anim_layer_is(c, S0, 1, 0x0010_7c44, 0x0010_7c50)?;
            if c.g[V0][0] == 0 {
                c.inc_r(S7, 1);
                return Ok(());
            }
        }
    }
    // Close (150 < d is too far, d < gp-0x7e78 too close) and in front.
    c.cycles += 22;
    let s = sp(c);
    let (p_other, p_self) = (at(c, S0, rider::POS), at(c, S1, rider::POS));
    c.vsub(p_other, p_self);
    c.sqc2(5, p(s, 0x10));
    c.g[AT][0] = 0x4316_0000; // 150.0
    c.mtc1(AT, 0);
    c.qcopy(V0, p(s, 0x10), s);
    c.vlen(s, V1, 1);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    c.cycles += 5;
    c.lwc1(20, gp(c, -0x7e78));
    c.c_lt_s(1, 20);
    if c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    c.cycles += 23;
    c.mfc1(V0, 1);
    c.vdiv(s, V0);
    c.sqc2(4, p(s, 0x20));
    c.lq(V0, p(s, 0x20));
    c.mtc1(0, 1);
    c.sq(V0, p(s, 0x10));
    let fwd = at(c, S1, 0x340);
    c.vdot(p(s, 0x10), fwd, V1, 0);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    // The rider's attribute factor (behaviour library); the knock-down
    // impulse along the direction (plus the velocity when fast).
    c.cycles += 33;
    call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0010_7d28)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0010_7d34)?;
    c.lw_r(A1, at(c, S1, 0x86c));
    c.g[A0][0] = c.g[V0][0];
    c.lw_r(A2, at(c, S1, 0xb34));
    call_ptr(c, callee::BE_CHAR_ATTRIB5_FACTOR, 0x0010_7d44)?;
    let rd = r(c, S1);
    c.lwc1(2, gp(c, -0x7e74));
    c.vlen(p(rd, rider::VEL), V0, 5);
    c.g[AT][0] = 0x3f00_0000; // 0.5
    c.mtc1(AT, 4);
    c.c_lt_s(20, 5);
    c.mul_s(0, 0, 2);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 3);
    c.lwc1(1, p(rd, 0x350));
    c.lwc1(2, gp(c, -0x7e70));
    c.add_s(1, 1, 4);
    c.add_s(0, 0, 3);
    c.mul_s(0, 0, 1);
    let fast = c.fcc();
    c.mul_s(12, 0, 2);
    let s = sp(c);
    if fast {
        c.cycles += 22;
        c.mfc1(V0, 4);
        c.vscale(p(s, 0x10), V0);
        c.sqc2(5, p(s, 0x40));
        c.div_s(0, 4, 5);
        c.qcopy(V0, p(s, 0x40), p(s, 0x30));
        c.mfc1(V1, 0);
        c.vscale(p(rd, rider::VEL), V1);
        c.vstore(5, p(s, 0x50), V0, &[p(s, 0x40)]);
        c.vadd(p(s, 0x30), p(s, 0x40));
        c.vstore(5, p(s, 0x50), V0, &[p(s, 0x10), p(s, 0x20)]);
    }
    // Rider_ResolveRiderCollision(other, r, &dir, 1)
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.set_r(A2, p(s, 0x10));
    c.g[A3][0] = 1;
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7e10)?;
    callee::AI_GET_FRAME_COUNTER.call(c, 0x0010_7e18)?;
    c.sw_r(at(c, S5, 0x18), V0);
    c.cycles += 1;
    c.inc_r(S7, 1);
    Ok(())
}

// ---------------------------------------------------------------------------
// Forward-difference cache.

const FD_ROW: u32 = 0x0032_bbf0;
const FD_CELL: u32 = 0x0032_bc28;

/// A cache cell (0x40 bytes): two triangle normals (`+0`, `+0x10`; x =
/// the "not computed" sentinel `gp-0x2d94`, or the "degenerate" value
/// `gp-0x2d90`) and the cell's box (`+0x20` min, `+0x30` max).
mod cell {
    pub const NORMAL1: u32 = 0x10;
    pub const BOX_MIN: u32 = 0x20;
    pub const BOX_MAX: u32 = 0x30;
    pub const SIZE: u32 = 0x40;
}

/// The hint (`t3`): `{key, u16 row, u16 col, first_triangle, kind}`.
mod hint {
    pub const KEY: u32 = 0x0;
    pub const ROW: u32 = 0x4;
    pub const COL: u32 = 0x6;
    /// Non-zero: the cell's first triangle (else the second).
    pub const FIRST: u32 = 0x8;
    pub const KIND: u32 = 0xc;
}

/// The volume (`a0`): kind (`+0`), facing test (`+8`) with its direction
/// (`+0x10`), box (`+0x20` max, `+0x30` min... compared as the original),
/// vtable (`+0x50`: 0x20 triangle test, 0x40 UV refine).
mod volume {
    pub const KIND: u32 = 0x0;
    pub const FACING: u32 = 0x8;
    pub const DIR: u32 = 0x10;
    pub const VTABLE: u32 = 0x50;
}

/// The triangle normal of `(a, b, d)` on VU0: `vf5 = normalize((a - b) x
/// (b - d))`, `vf10.w = 1 / |n|`, `v0 = vf10` (`qmfc2`).
fn triangle_normal_regs(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    let v = c.vu0();
    v.sub(0b1111, 5, 2, Src::Vec(3));
    v.sub(0b1111, 6, 3, Src::Vec(4));
    v.opmula(5, 6);
    v.opmsub(7, 6, 5);
    v.sub(0b0001, 7, 7, Src::Vec(7));
    v.add(0b1000, 8, 0, Src::Bc(0, W));
    v.mul(0b1111, 9, 7, Src::Vec(7));
    v.adda(0b1000, 9, Src::Bc(9, Y));
    v.madda(0b1000, 8, Src::Bc(9, Z));
    v.madd(0b1000, 9, 8, Src::Bc(9, W));
    v.rsqrt(0, 3, 9, 0);
    v.waitq();
    v.mul(0b1111, 5, 7, Src::Q);
    v.add(0b1000, 10, 0, Src::Bc(0, W));
    v.mul(0b1111, 10, 10, Src::Q);
    c.qmfc2(V0, 10);
}

/// The box of the quad `(a, b, d, e)` into `cell+0x20` (min) / `+0x30`
/// (max).
fn quad_box(c: &mut Cpu, a: u32, b: u32, d: u32, e: u32, cell_at: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    c.lqc2(5, e);
    let v = c.vu0();
    v.max(0b1111, 6, 2, Src::Vec(3));
    v.mini(0b1111, 7, 2, Src::Vec(3));
    v.max(0b1111, 8, 4, Src::Vec(5));
    v.mini(0b1111, 9, 4, Src::Vec(5));
    v.max(0b1111, 6, 6, Src::Vec(8));
    v.mini(0b1111, 7, 7, Src::Vec(9));
    c.sqc2(6, p(cell_at, cell::BOX_MAX));
    c.sqc2(7, p(cell_at, cell::BOX_MIN));
}

/// The volume's UV refine (virtual 0x40) for kind-2 volumes: `vt[0x40](
/// vol, key, &u, &v, a3', t0', t1')` with the saved t5-t7.
fn uv_refine(c: &mut Cpu, uv: u32, ret: u32) -> Flow {
    c.cycles += 11;
    c.lw_r(V0, at(c, S3, volume::VTABLE));
    c.g[T0][0] = c.g[T5][0];
    let s = sp(c);
    c.lw_r(A1, p(s, 0x30));
    c.g[T1][0] = c.g[T6][0];
    c.g[T2][0] = c.g[T7][0];
    c.set_r(A2, p(s, uv));
    c.set_r(A3, p(s, uv + 4));
    vcall(c, S3, V0, 0x40, V1, ret)
}

/// The volume's triangle test (virtual 0x20) with t4-t7 (`t4` only in the
/// grid scan) saved around it; v0 = hit.
fn triangle_test(c: &mut Cpu, save_t4: bool, ret: u32) -> Flow {
    let s = sp(c);
    if save_t4 {
        c.sq(T4, p(s, 0x50));
    }
    c.sq(T5, p(s, 0x60));
    c.sq(T6, p(s, 0x70));
    c.sq(T7, p(s, 0x80));
    vcall(c, S3, V1, 0x20, V0, ret)?;
    let s = sp(c);
    if save_t4 {
        c.lq(T4, p(s, 0x50));
    }
    c.lq(T5, p(s, 0x60));
    c.lq(T6, p(s, 0x70));
    c.lq(T7, p(s, 0x80));
    Ok(())
}

/// Facing test of a triangle normal at `n` against the volume direction:
/// when the volume asks for it (`+8`), `f{fr} = dot(n, dir)` and `c.le.s
/// fr, limit` (`limit` loaded from `gp + limit_at` first when given). True
/// when the triangle is to be tested; leaves v1 = the volume vtable then.
fn faces_volume(c: &mut Cpu, n: u32, fr: usize, limit: usize, limit_at: Option<i32>, cycles: u64) -> bool {
    c.cycles += 3;
    c.lw_r(V0, at(c, S3, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += cycles;
        if let Some(off) = limit_at {
            c.lwc1(limit, gp(c, off));
        }
        let dir = at(c, S3, volume::DIR);
        c.lqc2(3, n);
        c.lqc2(5, dir);
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, fr);
        c.c_le_s(fr, limit);
        if !c.fcc() {
            return false;
        }
        c.cycles += 1;
    }
    c.lw_r(V1, at(c, S3, volume::VTABLE));
    true
}

/// `bool AIFwdDiffCache_CollideEntry(Volume* vol, int key, Entry* e, a3,
/// t0, t1, float uv[2], Hint* hint)` (0x32b6e0): the forward-difference
/// grid of a cache entry (fine 3x3 cells of 0x15 vertices, or coarse 9x9
/// of 1) against a volume. The hint's cell (same key and kind) is tried
/// first; otherwise every cell whose box overlaps the volume's box has its
/// two triangle normals computed on first use (degenerate ones marked) and
/// its triangles, when facing, handed to the volume's triangle test. A hit
/// writes the cell's UV (refined by kind-2 volumes) and the hint, and
/// returns true.
pub fn ai_fwd_diff_cache_collide_entry(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S1, 0x110),
        (S3, 0xf0),
        (S6, 0xc0),
        (S0, 0x120),
        (S2, 0x100),
        (S4, 0xe0),
        (S5, 0xd0),
        (S7, 0xb0),
        (FP, 0xa0),
    ];
    const FSAVED: &[(usize, u32)] = &[
        (25, 0x158),
        (24, 0x150),
        (23, 0x148),
        (22, 0x140),
        (21, 0x138),
        (20, 0x130),
    ];
    guest(c, |c| {
        c.cycles += 27;
        prologue(c, 0x160, SAVED, 0x90, FSAVED);
        c.g[T5][0] = c.g[A3][0];
        c.g[T6][0] = c.g[T0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[T7][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T3][0];
        let s = sp(c);
        c.sw_r(p(s, 0x30), A1);
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(s, 0x34), T2);
        // The grid: sp+0x38 cells per side, sp+0x3c vertex rows per cell
        // row, s2 vertex step, s0 the cells, f22/f23 the UV scale/offset.
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A2][0] = 3;
            c.g[V0][0] = 0x15;
            c.g[S2][0] = 3;
            c.sw_r(p(s, 0x3c), V0);
            c.sw_r(p(s, 0x38), A2);
            c.lwc1(22, gp(c, -0x2dc0));
            c.lwc1(23, gp(c, -0x2dbc));
            c.set_r(S0, at(c, S1, 0x1a90));
        } else {
            c.cycles += 8;
            c.g[A2][0] = 1;
            c.g[V1][0] = 9;
            c.lwc1(22, gp(c, -0x2db8));
            c.g[S2][0] = 1;
            c.lwc1(23, gp(c, -0x2db4));
            c.set_r(S0, at(c, S1, 0x650));
            c.sw_r(p(s, 0x3c), A2);
            c.sw_r(p(s, 0x38), V1);
        }
        let hit = fd_try_hint(c)?;
        if !hit {
            fd_scan(c)?;
        }
        c.cycles += 18;
        epilogue(
            c,
            &[
                (S0, 0x120),
                (S1, 0x110),
                (S2, 0x100),
                (S3, 0xf0),
                (S4, 0xe0),
                (S5, 0xd0),
                (S6, 0xc0),
                (S7, 0xb0),
                (FP, 0xa0),
            ],
            &[
                (25, 0x158),
                (24, 0x150),
                (23, 0x148),
                (22, 0x140),
                (21, 0x138),
                (20, 0x130),
            ],
            0x90,
            0x160,
        );
        Ok(())
    })
}

/// The hint's cell first. True (v0 = 1) on a hit; otherwise the hint is
/// cleared (when it was this entry's) and v0 = cells per side.
fn fd_try_hint(c: &mut Cpu) -> Result<bool, Unwound> {
    let s = sp(c);
    c.cycles += 2;
    let no_hint = c.g[S6][0] == 0;
    c.lw_r(V1, p(s, 0x30));
    if no_hint {
        c.cycles += 1;
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::KEY));
    let other_key = c.g[V0][0] != c.g[V1][0];
    c.lw_r(V0, p(s, 0x38));
    if other_key {
        return Ok(false);
    }
    c.cycles += 4;
    c.lw_r(V1, at(c, S6, hint::KIND));
    c.lw_r(V0, at(c, S3, volume::KIND));
    if c.g[V1][0] != c.g[V0][0] {
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    // The cell (t1) and its quad's vertices a3, a2, a0, a1.
    c.cycles += 2;
    let coarse = c.g[V1][0] != 0;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    if !coarse {
        c.cycles += 6;
        c.g[V0][0] = 0xc0;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x1a90);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0x240;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x650);
    }
    c.cycles += 19;
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(T1, r(c, V0).wrapping_add(r(c, A1)));
    c.mult(0, V0, V1, A2);
    c.lwc1(0, at(c, T1, cell::BOX_MIN));
    c.mult(0, V1, A0, S2);
    c.lwc1(1, gp(c, -0x2db0));
    c.g[A0][0] = 0xa0;
    c.set_r(A1, r(c, S2) << 4);
    c.mult(0, A2, S2, A0);
    c.c_eq_s(0, 1);
    c.mult(0, V0, V0, A0);
    c.set_r(V1, r(c, V1) << 4);
    c.inc_r(V0, 0x10);
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(A3, r(c, V0).wrapping_add(r(c, V1)));
    c.set_r(A2, r(c, A3).wrapping_add(r(c, A2)));
    c.set_r(A0, r(c, A3).wrapping_add(r(c, A1)));
    let no_box = c.fcc();
    c.set_r(A1, r(c, A2).wrapping_add(r(c, A1)));
    if no_box {
        c.cycles += 12;
        let (a3, a2, a0, a1, cl) = (r(c, A3), r(c, A2), r(c, A0), r(c, A1), r(c, T1));
        quad_box(c, a3, a2, a0, a1, cl);
    }
    // The first triangle when the hint says so, else the second.
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::FIRST));
    let cl = r(c, T1);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.lwc1(0, cl);
        c.c_eq_s(0, 1);
        if c.fcc() {
            c.cycles += 27;
            let (a1, a2, a0) = (r(c, A1), r(c, A2), r(c, A0));
            triangle_normal_regs(c, a1, a2, a0);
            c.sqc2(5, cl);
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 0);
            c.mtc1(V0, 1);
            c.c_lt_s(0, 1);
            let tiny = c.fcc();
            let s = sp(c);
            c.sw_r(s, V0);
            if tiny {
                c.cycles += 2;
                c.lwc1(0, gp(c, -0x2dac));
                c.swc1(0, cl);
            }
            c.cycles += 1;
        }
        c.lwc1(1, cl);
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x2da8));
        c.c_eq_s(1, 0);
        if c.fcc() {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        if !faces_volume(c, cl, 1, 0, Some(-0x2da4), 14) {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        c.cycles += 16;
        c.g[A3][0] = c.g[A0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T1][0] = c.g[T5][0];
        c.g[T3][0] = c.g[T7][0];
        triangle_test(c, false, 0x0032_b988)?;
        if c.g[V0][0] == 0 {
            fd_drop_hint(c);
            return Ok(false);
        }
        fd_hint_uv(c, 0x10, 0x0032_ba00, 15, 4)?;
        return Ok(true);
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    c.cycles += 4;
    c.c_eq_s(0, 1);
    if c.fcc() {
        c.cycles += 27;
        let (a0, a2, a3) = (r(c, A0), r(c, A2), r(c, A3));
        triangle_normal_regs(c, a0, a2, a3);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0);
        c.mtc1(V0, 1);
        c.c_lt_s(0, 1);
        let tiny = c.fcc();
        let s = sp(c);
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 2;
            c.lwc1(0, gp(c, -0x2da0));
            c.swc1(0, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
    }
    c.lwc1(1, p(cl, cell::NORMAL1));
    c.cycles += 5;
    c.lwc1(0, gp(c, -0x2d9c));
    c.c_eq_s(1, 0);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        fd_drop_hint(c);
        return Ok(false);
    }
    if !faces_volume(c, p(cl, cell::NORMAL1), 1, 0, Some(-0x2d98), 14) {
        c.sw(r(c, S6), 0);
        fd_clear_hint(c);
        return Ok(false);
    }
    c.cycles += 15;
    c.g[A1][0] = c.g[A0][0];
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    triangle_test(c, false, 0x0032_bb20)?;
    if c.g[V0][0] == 0 {
        fd_drop_hint(c);
        return Ok(false);
    }
    fd_hint_uv(c, 0x18, 0x0032_bb98, 15, 3)?;
    Ok(true)
}

/// The hint cell's UV (`sp+uv`), refined for kind-2 volumes, written to
/// the caller's `uv`; v0 = 1.
fn fd_hint_uv(c: &mut Cpu, uv: u32, ret: u32, cycles: u64, tail: u64) -> Flow {
    c.cycles += cycles;
    let s = sp(c);
    c.g[V0][0] = c.lhu(at(c, S6, hint::COL));
    c.g[A1][0] = 2;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lw_r(A0, at(c, S3, volume::KIND));
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(1, 1, 22);
    c.mul_s(0, 0, 22);
    c.add_s(1, 1, 23);
    c.add_s(0, 0, 23);
    c.swc1(1, p(s, uv + 4));
    let refine = c.g[A0][0] == c.g[A1][0];
    c.swc1(0, p(s, uv));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += tail;
    let s = sp(c);
    c.lwc1(1, p(s, uv));
    c.g[V0][0] = 1;
    c.lwc1(0, p(s, uv + 4));
    c.cycles += 4;
    c.lw_r(V1, p(s, 0x34));
    let out = r(c, V1);
    c.swc1(1, out);
    c.swc1(0, p(out, 4));
    Ok(())
}

/// `hint->key = 0`, then [`fd_clear_hint`].
fn fd_drop_hint(c: &mut Cpu) {
    c.cycles += 1;
    c.sw(r(c, S6), 0);
    fd_clear_hint(c);
}

/// Clear the rest of the hint; v0 = cells per side.
fn fd_clear_hint(c: &mut Cpu) {
    c.cycles += 4;
    let h = r(c, S6);
    c.sh(p(h, hint::ROW), 0);
    c.sh(p(h, hint::COL), 0);
    c.sw(p(h, hint::FIRST), 0);
    c.sw(p(h, hint::KIND), 0);
    c.cycles += 1;
    let s = sp(c);
    c.lw_r(V0, p(s, 0x38));
}

/// Scan every cell (v0 = cells per side on entry); v0 = 1 on a hit, else 0.
fn fd_scan(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.set_r(S4, at(c, S1, 0x10));
    let empty = c.g[V0][0] == 0;
    c.g[FP][0] = 0;
    if empty {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 6;
    let s = sp(c);
    c.set_r(V1, r(c, S2) << 4);
    c.g[V0][0] = 0xa0;
    c.sw_r(p(s, 0x44), V1);
    c.mult(0, V1, S2, V0);
    c.sw_r(p(s, 0x40), V1);
    c.lw_r(V0, p(s, 0x40));
    loop {
        // Row fp: the quad corners s4, s1 (next row), s2, s5.
        c.cycles += 7;
        c.g[S7][0] = 0;
        c.lw_r(V1, p(s, 0x44));
        c.set_r(S1, r(c, S4).wrapping_add(r(c, V0)));
        c.lw_r(V0, p(s, 0x38));
        c.set_r(S2, r(c, S4).wrapping_add(r(c, V1)));
        let no_cells = c.g[V0][0] == 0;
        c.set_r(S5, r(c, S1).wrapping_add(r(c, V1)));
        if !no_cells {
            c.cycles += 7;
            c.lwc1(21, gp(c, -0x2d94));
            c.g[T4][0] = c.g[V1][0];
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 25);
            c.lwc1(20, gp(c, -0x2d90));
            c.lwc1(24, gp(c, -0x2d8c));
            loop {
                if fd_cell(c)? {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    return Ok(());
                }
                c.cycles += 8;
                c.inc_r(S7, 1);
                let step = r(c, T4);
                c.inc_r(S4, step);
                c.inc_r(S1, step);
                c.inc_r(S2, step);
                c.inc_r(S5, step);
                c.g[V0][0] = slt(c, S7, V1);
                let more = c.g[V0][0] != 0;
                c.inc_r(S0, cell::SIZE);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, FD_CELL);
                }
            }
        }
        c.cycles += 8;
        c.lw_r(V0, p(s, 0x3c));
        c.inc_r(FP, 1);
        c.set_r(V1, r(c, V0) << 4);
        c.set_r(S4, r(c, S4).wrapping_add(r(c, V1)));
        c.lw_r(V1, p(s, 0x38));
        c.g[V0][0] = slt(c, FP, V1);
        let more = c.g[V0][0] != 0;
        c.lw_r(V0, p(s, 0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, FD_ROW);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(())
}

/// One cell (s0) of the scan: box, overlap with the volume, both
/// triangles. True on a hit (UV and hint written); otherwise v1 = cells
/// per side.
fn fd_cell(c: &mut Cpu) -> Result<bool, Unwound> {
    let cl = r(c, S0);
    let vol = r(c, S3);
    c.cycles += 5;
    c.lwc1(0, p(cl, cell::BOX_MIN));
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 14;
        let (s4, s1, s2, s5) = (r(c, S4), r(c, S1), r(c, S2), r(c, S5));
        quad_box(c, s4, s1, s2, s5, cl);
        c.lwc1(1, p(vol, 0x20));
        c.lwc1(0, p(cl, cell::BOX_MIN));
    } else {
        c.lwc1(1, p(vol, 0x20));
    }
    // Box overlap: cell min < vol max and vol min < cell max on each axis.
    let mut overlap = false;
    c.cycles += 4;
    c.c_lt_s(0, 1);
    c.g[V0][0] = 0;
    if c.fcc() {
        let tests: [(u32, u32, bool, u64); 5] = [
            (0x30, cell::BOX_MAX, true, 5),
            (0x24, cell::BOX_MIN + 4, false, 5),
            (0x34, cell::BOX_MAX + 4, true, 5),
            (0x28, cell::BOX_MIN + 8, false, 5),
            (0x38, cell::BOX_MAX + 8, true, 6),
        ];
        overlap = true;
        for (i, &(voff, coff, rev, cyc)) in tests.iter().enumerate() {
            c.cycles += cyc;
            c.lwc1(1, p(vol, voff));
            c.lwc1(0, p(cl, coff));
            if rev {
                c.c_lt_s(1, 0);
            } else {
                c.c_lt_s(0, 1);
            }
            if !c.fcc() {
                overlap = false;
                break;
            }
            if i == 4 {
                c.g[V0][0] = 1;
            }
        }
    }
    c.cycles += 2;
    let s = sp(c);
    c.lw_r(V1, p(s, 0x38));
    if !overlap {
        return Ok(false);
    }
    // First triangle (s5, s1, s2).
    c.cycles += 4;
    c.lwc1(0, cl);
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s5, s1, s2) = (r(c, S5), r(c, S1), r(c, S2));
        triangle_normal_regs(c, s5, s1, s2);
        c.sqc2(5, cl);
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, cl);
        }
        c.cycles += 1;
        c.lwc1(0, cl);
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let mut tested = false;
    if !c.fcc() && faces_volume(c, cl, 0, 24, None, 13) {
        c.cycles += 20;
        c.g[T1][0] = c.g[T5][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T3][0] = c.g[T7][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        triangle_test(c, true, 0x0032_be00)?;
        tested = true;
        if c.g[V0][0] != 0 {
            fd_cell_hit(c, 0x20, 0x0032_be74, true)?;
            return Ok(true);
        }
    }
    if tested {
        c.cycles += 1;
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    // Second triangle (s2, s1, s4).
    c.cycles += 4;
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s2, s1, s4) = (r(c, S2), r(c, S1), r(c, S4));
        triangle_normal_regs(c, s2, s1, s4);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
        c.lwc1(0, p(cl, cell::NORMAL1));
    } else {
        c.lwc1(0, p(cl, cell::NORMAL1));
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    // Facing (the original reloads v1 = cells per side when not facing).
    c.cycles += 3;
    c.lw_r(V0, p(vol, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += 13;
        c.lqc2(3, p(cl, cell::NORMAL1));
        c.lqc2(5, p(vol, volume::DIR));
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.c_le_s(0, 24);
        let facing = c.fcc();
        c.lw_r(V1, p(s, 0x38));
        if !facing {
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.lw_r(V1, p(vol, volume::VTABLE));
    c.cycles += 19;
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S4][0];
    triangle_test(c, true, 0x0032_bfb4)?;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    fd_cell_hit(c, 0x28, 0x0032_c028, false)?;
    Ok(true)
}

/// A hit in the scan: the cell's UV at `sp+uv` (refined for kind-2
/// volumes) to the caller's `uv`, the hint (`first`: the cell's first
/// triangle) with the volume kind.
fn fd_cell_hit(c: &mut Cpu, uv: u32, ret: u32, first: bool) -> Flow {
    c.cycles += 13;
    let s = sp(c);
    c.mtc1(FP, 0);
    c.cvt_s_w(0, 0);
    c.lw_r(V0, at(c, S3, volume::KIND));
    c.mtc1(S7, 1);
    c.cvt_s_w(1, 1);
    c.g[V1][0] = 2;
    c.mul_s(0, 0, 22);
    c.mul_s(1, 1, 22);
    c.add_s(0, 0, 23);
    c.add_s(1, 1, 23);
    c.swc1(0, p(s, uv));
    let refine = c.g[V0][0] == c.g[V1][0];
    c.swc1(1, p(s, uv + 4));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += 6;
    let s = sp(c);
    c.lwc1(0, p(s, uv));
    c.lwc1(1, p(s, uv + 4));
    c.lw_r(V0, p(s, 0x34));
    let out = r(c, V0);
    c.swc1(0, out);
    let has_hint = c.g[S6][0] != 0;
    c.swc1(1, p(out, 4));
    if has_hint {
        let h = r(c, S6);
        if first {
            c.cycles += 7;
            c.lw_r(V0, p(s, 0x30));
            c.g[V1][0] = 1;
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V0);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw_r(p(h, hint::FIRST), V1);
        } else {
            c.cycles += 5;
            c.lw_r(V1, p(s, 0x30));
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V1);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw(p(h, hint::FIRST), 0);
        }
        c.cycles += 2;
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(h, hint::KIND), V0);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, fake_call, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP, SP as STACK};

    const SEEDS: u32 = 250;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    fn put_v(c: &mut Cpu, at: u32, v: [f32; 4]) {
        for (k, x) in v.iter().enumerate() {
            c.bus.write32(at + 4 * k as u32, f(*x));
        }
    }

    fn rand_v(r: &mut Rng, lo: f32, hi: f32) -> [f32; 4] {
        [uf(r, lo, hi), uf(r, lo, hi), uf(r, lo, hi), 0.0]
    }

    fn gp_f32(c: &mut Cpu, off: i32, x: f32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), f(x));
    }

    // -----------------------------------------------------------------
    // Shared fixtures.

    /// Vtables: the collider sub-object (0x130 delegate, 0x138 contacts
    /// added), surface owners (0x150, 0x158), cache volumes (0x20 triangle
    /// test, 0x40 UV refine), hulls (0x18 reference depth).
    const VT_SUB: u32 = DATA + 0x3_0000;
    const VT_OWNER: u32 = DATA + 0x3_0400;
    const VT_VOL: u32 = DATA + 0x3_0600;
    const VT_HULL: u32 = DATA + 0x3_0700;
    const OWNER: u32 = DATA + 0x3_0800;
    /// Four surfaces (0x100 apart), their material table, four instances.
    const SURFS: u32 = DATA + 0x3_1000;
    const MATS: u32 = DATA + 0x3_2000;
    const INSTS: u32 = DATA + 0x3_3000;
    /// The overlap list and its objects (0x100 apart).
    const LIST: u32 = DATA + 0x3_4000;
    const OBJS: u32 = DATA + 0x3_5000;
    /// The rider, its state base (`+0x77c`), params (`+0x780`), sphere tree.
    const RIDER: u32 = DATA + 0x1_0000;
    const BASE: u32 = DATA + 0x2_0000;
    const PARAMS: u32 = DATA + 0x2_8000;
    const TREE: u32 = DATA + 0x2_c000;

    /// `fakes::fake_call`, then a result that depends on the method.
    fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x10 => c.g[V0][0] = h & 3,                      // contacts added
            0x60 => c.g[V0][0] = ((h >> 4) & 3 == 0) as u64, // triangle hit
            _ => {}
        }
    }

    fn put_slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, target);
    }

    /// Scrambled registers, the fake hooks and vtables, surfaces,
    /// materials and instances.
    fn setup_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = fake_varied;
        put_slot(c, VT_SUB, 0x130, FAKE);
        put_slot(c, VT_SUB, 0x138, FAKE + 0x10);
        put_slot(c, VT_OWNER, 0x150, FAKE + 0x40);
        put_slot(c, VT_OWNER, 0x158, FAKE + 0x50);
        put_slot(c, VT_VOL, 0x20, FAKE + 0x60);
        put_slot(c, VT_VOL, 0x40, FAKE + 0x70);
        put_slot(c, VT_HULL, 0x18, FAKE + 0x80);
        c.bus.write32(OWNER + 0xc, VT_OWNER);
        for k in 0..4 {
            let s = SURFS + 0x100 * k;
            c.bus.write32(s + 0x78, r.next_u32() % 3);
            c.bus.write32(s + 0xc, if k % 2 == 0 { OWNER } else { 0 });
            c.bus.write32(s + 0x88, MATS);
            c.bus.write32(s + 0x8, if r.next_u32() % 3 == 0 { 0x2000 } else { 0 });
            c.bus.write32(s + 0x8c, 0);
        }
        for i in 0..4 {
            let m = MATS + 0x10 + 0xc * i;
            c.bus.write32(m, if r.next_u32() % 3 == 0 { 0 } else { f(1.0) });
            c.bus.write32(m + 4, f(uf(&mut r, 0.0, 2.0)));
            c.bus.write16(m + 8, (r.next_u32() & 3) as u16);
        }
        for k in 0..4 {
            let i = INSTS + 0x200 * k;
            c.bus.write32(i + 0x150, r.next_u32() % 3);
            c.bus.write16(i + 0xa, r.next_u32() as u16);
        }
        // The behaviour library exists (a zeroed dummy object: no
        // construction or disc load on first use).
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        // A world whose race manager has one rider (this one): without it
        // `SSXAudioSystem_FindRider` (sound paths) walks a garbage rider
        // count read from low memory and runs away.
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, RIDER);
        c.bus.write32(arr + 0x7c, 1);
        r
    }

    /// `n` contact records at `base`: positions, normals (some repeated
    /// with their depth, for ties), depths, materials, surfaces, instances.
    fn put_contacts(c: &mut Cpu, r: &mut Rng, base: u32, n: u32) {
        let mut prev = ([0.0f32, 1.0, 0.0, 0.0], 0.5f32);
        for i in 0..n {
            let ct = base + 0x80 * i;
            put_v(c, ct, rand_v(r, -3.0, 3.0));
            let tie = r.next_u32() % 4 == 0;
            let normal = if tie {
                prev.0
            } else {
                [uf(r, -1.0, 1.0), uf(r, -0.2, 1.0), uf(r, -1.0, 1.0), 0.0]
            };
            let depth = if tie {
                prev.1
            } else {
                match r.next_u32() % 3 {
                    0 => 0.5,
                    1 => 0.25,
                    _ => uf(r, 0.0, 1.0),
                }
            };
            prev = (normal, depth);
            put_v(c, ct + 0x10, normal);
            put_v(c, ct + 0x20, rand_v(r, -1.0, 1.0));
            c.bus.write32(ct + 0x40, f(depth));
            c.bus.write32(
                ct + 0x4c,
                if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                },
            );
            let surface = if r.next_u32() % 3 == 0 {
                0
            } else {
                SURFS + 0x100 * (r.next_u32() % 4)
            };
            c.bus.write32(ct + 0x50, surface);
            let inst = if r.next_u32() % 4 == 0 {
                0
            } else {
                INSTS + 0x200 * (r.next_u32() % 4)
            };
            c.bus.write32(ct + 0x54, inst);
            c.bus.write32(ct + 0x5c, r.next_u32() % 4);
            c.bus.write32(ct + 0x6c, f(uf(r, 0.0, 1.0)));
            c.bus.write32(ct + 0x70, f(uf(r, 0.0, 1.0)));
        }
    }

    /// The overlap list: no instances, no rails, `objects` static objects
    /// with huge boxes whose collider delegate adds 0-3 contacts.
    fn put_list(c: &mut Cpu, objects: u32) {
        c.bus.write32(LIST, LIST + 0x800);
        c.bus.write32(LIST + 0x10c, 0);
        c.bus.write32(LIST + 0x210, 0);
        c.bus.write32(LIST + 8, objects);
        for i in 0..objects {
            let o = OBJS + 0x100 * i;
            c.bus.write32(LIST + 0xc + 4 * i, o);
            c.bus.write32(o + 8, 0x20);
            let sub = o + 0x90;
            c.bus.write32(o + 0xc, sub);
            c.bus.write32(sub + 0xc, VT_SUB);
            c.bus.write32(o + 0x88, o + 0xc0);
            put_v(c, o + 0x60, [-1.0e6, -1.0e6, -1.0e6, 0.0]);
            put_v(c, o + 0x6c, [1.0e6, 1.0e6, 1.0e6, 0.0]);
        }
    }

    /// The rider: vectors, list, sphere tree (one child), states.
    fn put_rider(c: &mut Cpu, r: &mut Rng, main: u32, sub: u32) {
        c.bus.write32(RIDER + 0x77c, BASE);
        c.bus.write32(RIDER + 0x780, PARAMS);
        put_v(c, RIDER + 0x370, [uf(r, -0.2, 0.2), 1.0, uf(r, -0.2, 0.2), 0.0]);
        put_v(c, RIDER + 0x110, rand_v(r, -5.0, 5.0));
        put_v(c, RIDER + 0x1e0, rand_v(r, -30.0, 30.0));
        put_v(c, RIDER + 0x1b0, rand_v(r, -1.0, 1.0));
        put_v(c, RIDER + 0x3b0, rand_v(r, -1.0, 1.0));
        put_v(c, RIDER + 0x340, rand_v(r, -1.0, 1.0));
        c.bus.write32(RIDER + 0x1f0, f(uf(r, 0.0, 20.0)));
        c.bus.write32(RIDER + 0x350, f(uf(r, 0.0, 1.0)));
        c.bus.write32(PARAMS + 0x140, f(uf(r, 0.0, 1.0)));
        c.bus.write32(RIDER + 0x860, LIST);
        c.bus.write32(RIDER + 0x864, 0);
        c.bus.write32(RIDER + 0x874, 0);
        c.bus.write32(RIDER + 0xaa0, TREE);
        put_v(c, TREE + 0x10, rand_v(r, -1.0, 1.0));
        c.bus.write32(TREE + 0x20, f(uf(r, 0.5, 2.0)));
        c.bus.write32(TREE + 0x2c, 1);
        put_v(c, TREE + 0x30, rand_v(r, -1.0, 1.0));
        c.bus.write32(TREE + 0x40, f(0.5));
        c.bus.write32(BASE + 0xde0, main);
        c.bus.write32(BASE + 0xde4, sub);
        put_v(c, BASE + 0xb0, [uf(r, -0.3, 0.3), 1.0, uf(r, -0.3, 0.3), 0.0]);
        for k in 0..6 {
            c.bus.write32(RIDER + 0x5b8 + 4 * k, r.next_u32() % 3);
        }
    }

    // -----------------------------------------------------------------
    // Sphere trees.

    fn put_sphere_tree(c: &mut Cpu, r: &mut Rng, t: u32, centre: [f32; 4], radius: f32, n: u32) {
        put_v(c, t + 0x10, centre);
        c.bus.write32(t + 0x20, f(radius));
        c.bus.write32(t + 0x2c, n);
        for i in 0..n {
            let ch = t + 0x20 * i;
            let off = rand_v(r, -1.5, 1.5);
            put_v(
                c,
                ch + 0x30,
                [centre[0] + off[0], centre[1] + off[1], centre[2] + off[2], 0.0],
            );
            c.bus.write32(ch + 0x40, f(uf(r, 0.1, 0.8)));
        }
    }

    fn sphere_setup(c: &mut Cpu, r: &mut Rng, na: u32, nb: u32, spread: f32) {
        let (a, b) = (DATA + 0x1000, DATA + 0x2000);
        let ca = rand_v(r, -2.0, 2.0);
        let off = rand_v(r, -spread, spread);
        let cb = [ca[0] + off[0], ca[1] + off[1], ca[2] + off[2], 0.0];
        let (ra, rb) = (uf(r, 1.0, 3.0), uf(r, 1.0, 3.0));
        put_sphere_tree(c, r, a, ca, ra, na);
        put_sphere_tree(c, r, b, cb, rb, nb);
        c.g[A0][0] = sx(a);
        c.g[A1][0] = sx(b);
        c.g[A2][0] = sx(DATA + 0x3000);
        c.g[A3][0] = sx(DATA + 0x3010);
        c.pc = ST_ENTRY;
    }

    driven!(gen_sphere_tree, ee::s10::f_00329f98__generated);
    driven!(hand_sphere_tree, rider_sphere_tree_intersect);

    #[test]
    fn rider_sphere_tree_intersect_matches() {
        check(ee::s10::f_00329f98__generated, rider_sphere_tree_intersect, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9f98, 0);
            let spread = if seed % 2 == 0 { 1.0 } else { 5.0 };
            sphere_setup(c, &mut r, seed % 5, (seed / 5) % 5, spread);
        });
        // Events at the loop heads and the hit block, with re-entry.
        check_events(gen_sphere_tree, hand_sphere_tree, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9f99, 0);
            sphere_setup(c, &mut r, 2 + seed % 4, 1 + (seed / 4) % 4, 0.5);
            schedule(c, &mut r, seed, ST_ENTRY)
        });
    }

    // -----------------------------------------------------------------
    // Ground probe.

    #[test]
    fn rider_ground_state_probe_ground_matches() {
        check(
            ee::s01::f_0013d1b8__generated,
            rider_ground_state_probe_ground,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0xd1b8, 6);
                put_rider(c, &mut r, 0, 0);
                put_list(c, seed % 4);
                // The probe's contact records (its frame at sp - 0x2190).
                put_contacts(c, &mut r, STACK - 0x2190 + 0x100, 10);
                gp_f32(c, -0x7068, uf(&mut r, 0.3, 0.9));
                let state = DATA + 0x200;
                c.bus.write32(state + 0x18, RIDER);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
            },
        );
    }

    // -----------------------------------------------------------------
    // Walls.

    const HULL: u32 = DATA + 0x600;

    #[test]
    fn rider_find_wall_contact_matches() {
        check(ee::s00::f_00104e70__generated, rider_find_wall_contact, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x4e70, 6);
            put_rider(c, &mut r, 0, 0);
            put_list(c, seed % 4);
            put_contacts(c, &mut r, STACK - 0x20a0, 10);
            c.bus.write32(HULL + 0x50, VT_HULL);
            gp_f32(c, -0x7f38, uf(&mut r, 0.0, 0.5));
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = sx(HULL);
            c.g[A2][0] = sx(DATA + 0x700);
        });
    }

    #[test]
    fn rider_handle_wall_collision_matches() {
        check(ee::s00::f_00105398__generated, rider_handle_wall_collision, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x5398, 8);
            // Sub-state 9 skips; main state 4 (grinding) takes the board
            // normal and skips the rail contacts; 2 / 0 the ground tests.
            let sub = if seed % 9 == 0 { 9 } else { 0 };
            let main = [4u32, 0, 2, 1][(seed % 4) as usize];
            put_rider(c, &mut r, main, sub);
            c.bus.write32(BASE + 0x30, r.next_u32() % 2);
            put_list(c, if seed % 5 == 0 { 0 } else { 1 + seed % 3 });
            // Rider_FindWallContact's records (its frame inside ours).
            put_contacts(c, &mut r, STACK - 0x4a0 - 0x20a0, 10);
            gp_f32(c, -0x7f34, uf(&mut r, 0.0, 0.5));
            gp_f32(c, -0x7f38, uf(&mut r, 0.0, 0.5));
            let ground = DATA + 0x800;
            put_v(c, ground, [uf(&mut r, -0.3, 0.3), 1.0, uf(&mut r, -0.3, 0.3), 0.0]);
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(ground) };
        });
    }

    // -----------------------------------------------------------------
    // Rails (reachable paths only: a rail contact needs the real rail
    // finders on real rail data).

    #[test]
    fn rider_handle_wall_rail_contact_matches() {
        check(
            ee::s00::f_00106f78,
            rider_handle_wall_rail_contact,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x6f78, 2);
                put_rider(c, &mut r, if seed % 2 == 0 { 4 } else { 0 }, 0);
                put_list(c, 0);
                let bones = DATA + 0x5000;
                c.bus.write32(PARAMS + 0x2c, bones);
                c.bus.write32(RIDER + 0x89c, r.next_u32() % 4);
                for i in 0..4 {
                    put_v(c, bones + 0x20 * i + 0x10, rand_v(&mut r, -1.0, 1.0));
                }
                c.g[A0][0] = sx(RIDER);
            },
        );
    }

    #[test]
    fn rider_try_start_railslide_matches() {
        check(ee::s00::f_00106848, rider_try_start_railslide, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x6848, 2);
            put_rider(c, &mut r, seed % 3, 0);
            put_list(c, 0);
            let bones = DATA + 0x5000;
            c.bus.write32(PARAMS + 0x2c, bones);
            c.bus.write32(RIDER + 0x8a0, r.next_u32() % 4);
            for i in 0..4 {
                put_v(c, bones + 0x20 * i + 0x10, rand_v(&mut r, -1.0, 1.0));
            }
            c.g[A0][0] = sx(RIDER);
        });
    }

    // -----------------------------------------------------------------
    // Rider against rider.

    const WORLD: u32 = DATA + 0x6_0000;
    const OTHERS: u32 = DATA + 0x4_0000;

    #[test]
    fn rider_check_rider_collisions_matches() {
        check(ee::s00::f_00107888__generated, rider_check_rider_collisions, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x7888, 12);
            put_rider(c, &mut r, seed % 3, 0);
            let (ail, arr) = (WORLD + 0x100, WORLD + 0x200);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD + 0x84, ail);
            c.bus.write32(ail + 0xc, arr);
            let frame = 10 + r.next_u32() % 50;
            c.bus.write32(arr + 8, frame);
            gp_f32(c, -0x7e80, uf(&mut r, 0.5, 2.0));
            gp_f32(c, -0x7e7c, uf(&mut r, 0.5, 2.0));
            gp_f32(c, -0x7e78, uf(&mut r, 0.0, 20.0));
            gp_f32(c, -0x7e74, uf(&mut r, 0.0, 1.0));
            gp_f32(c, -0x7e70, uf(&mut r, 0.0, 10.0));
            let me = r.next_u32() % 6;
            c.bus.write32(RIDER + 0x86c, me);
            let centre = rand_v(&mut r, -3.0, 3.0);
            let (radius, n) = (uf(&mut r, 1.0, 3.0), 1 + r.next_u32() % 2);
            put_sphere_tree(c, &mut r, TREE, centre, radius, n);
            put_v(c, RIDER + 0x110, centre);
            for i in 0..6u32 {
                // This rider's slot i: active, timers.
                let slot = RIDER + 0x24 * i;
                c.bus.write32(slot, (r.next_u32() % 4 != 0) as u32);
                for t in [0x10u32, 0x14, 0x18] {
                    c.bus.write32(slot + t, frame.wrapping_sub(r.next_u32() % 8));
                }
                // The other rider i.
                let o = OTHERS + 0x1000 * i;
                c.bus.write32(arr + 0x28 + 4 * i, o);
                c.bus.write32(o + 0x878, (r.next_u32() % 5 == 0) as u32);
                c.bus.write32(o + 0x86c, i);
                c.bus.write32(o + 0x77c, BASE);
                let tree = o + 0x800;
                c.bus.write32(o + 0xaa0, tree);
                let oc = rand_v(&mut r, -3.0, 3.0);
                let ocentre = [centre[0] + oc[0], centre[1] + oc[1], centre[2] + oc[2], 0.0];
                let (radius, n) = (uf(&mut r, 1.0, 3.0), 1 + r.next_u32() % 2);
                put_sphere_tree(c, &mut r, tree, ocentre, radius, n);
                put_v(c, o + 0x110, ocentre);
                put_v(c, o + 0x370, [0.0, 1.0, 0.0, 0.0]);
                put_v(c, o + 0x1e0, rand_v(&mut r, -30.0, 30.0));
                let pair = o + 0x24 * me;
                for t in [0x10u32, 0x14, 0x18] {
                    c.bus.write32(pair + t, frame.wrapping_sub(r.next_u32() % 8));
                }
            }
            c.g[A0][0] = sx(RIDER);
        });
    }

    // -----------------------------------------------------------------
    // Forward-difference cache.

    const SENTINEL: f32 = -99.0;
    const DEGENERATE: f32 = -98.0;

    /// An entry at `e`: 10x10 vertices from +0x10 (0x10 apart, rows 0xa0),
    /// coarse cells at +0x650, fine cells at +0x1a90; cells either
    /// uncomputed (sentinel) or with precomputed normals and boxes.
    fn put_entry(c: &mut Cpu, r: &mut Rng, e: u32) {
        for i in 0..100u32 {
            let (row, col) = ((i / 10) as f32, (i % 10) as f32);
            let y = if r.next_u32() % 8 == 0 { 0.0 } else { uf(r, -1.0, 1.0) };
            put_v(c, e + 0x10 + 0x10 * i, [col - 4.5, y, row - 4.5, 1.0]);
        }
        for k in 0..(81 + 9) {
            let cl = e + 0x650 + 0x40 * k;
            let fresh = r.next_u32() % 3 != 0;
            if fresh {
                c.bus.write32(cl, f(SENTINEL));
                c.bus.write32(cl + 0x10, f(SENTINEL));
                c.bus.write32(cl + 0x20, f(SENTINEL));
            } else {
                let n0 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                let n1 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                put_v(c, cl, [n0, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                put_v(c, cl + 0x10, [n1, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                let lo = rand_v(r, -5.0, 5.0);
                put_v(c, cl + 0x20, lo);
                put_v(
                    c,
                    cl + 0x30,
                    [
                        lo[0] + uf(r, 0.0, 2.0),
                        lo[1] + uf(r, 0.0, 2.0),
                        lo[2] + uf(r, 0.0, 2.0),
                        0.0,
                    ],
                );
            }
        }
    }

    #[test]
    fn ai_fwd_diff_cache_collide_entry_matches() {
        covered(
            ee::s10::f_0032b6e0__generated,
            ai_fwd_diff_cache_collide_entry,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0xb6e0, 6);
                let (vol, entry, hint, uv) = (DATA + 0x100, DATA + 0x4000, DATA + 0x200, DATA + 0x300);
                put_entry(c, &mut r, entry);
                for (off, x) in [(-0x2dc0, 0.1f32), (-0x2dbc, 0.05), (-0x2db8, 0.3), (-0x2db4, 0.15)] {
                    gp_f32(c, off, x);
                }
                for off in [-0x2db0, -0x2d94] {
                    gp_f32(c, off, SENTINEL);
                }
                for off in [-0x2dac, -0x2da8, -0x2da0, -0x2d9c, -0x2d90] {
                    gp_f32(c, off, DEGENERATE);
                }
                for off in [-0x2da4, -0x2d98, -0x2d8c] {
                    gp_f32(c, off, uf(&mut r, -0.5, 0.5));
                }
                let kind = seed % 3;
                c.bus.write32(vol, kind);
                c.bus.write32(vol + 8, (r.next_u32() % 2 == 0) as u32);
                put_v(c, vol + 0x10, [uf(&mut r, -0.5, 0.5), 1.0, uf(&mut r, -0.5, 0.5), 0.0]);
                let lo = rand_v(&mut r, -5.0, 3.0);
                let ext = uf(&mut r, 0.5, 4.0);
                put_v(c, vol + 0x20, [lo[0] + ext, lo[1] + ext, lo[2] + ext, 0.0]);
                put_v(c, vol + 0x30, lo);
                c.bus.write32(vol + 0x50, VT_VOL);
                let key = 7;
                let side = if kind == 0 { 3 } else { 9 };
                c.bus.write32(hint, if seed % 4 == 0 { key + 1 } else { key });
                c.bus.write16(hint + 4, (r.next_u32() % side) as u16);
                c.bus.write16(hint + 6, (r.next_u32() % side) as u16);
                c.bus.write32(hint + 8, r.next_u32() % 2);
                c.bus
                    .write32(hint + 0xc, if seed % 5 == 0 { (kind + 1) % 3 } else { kind });
                c.g[A0][0] = sx(vol);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = sx(entry);
                c.g[T2][0] = sx(uv);
                c.g[T3][0] = if seed % 7 == 0 { 0 } else { sx(hint) };
            },
        );
    }
}
