//! Staging module oc562 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc562`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_d470,
    sphere_tree_collide_triangle_node,
    "SphereTree_CollideTriangleNode",
)];

// Field offsets of the sphere tree node (`self`) and of its tree info.
/// `t4Vector axis[8]`: one 16-byte axis per child, indexed by child number.
const SCT_AXIS: u32 = 0x00;
/// `float scale`.
const SCT_SCALE: u32 = 0x90;
/// `float worstCase`: the closest plane offset found so far.
const SCT_WORST_CASE: u32 = 0x94;
/// `SphereTreeInfo* treeInfo`.
const SCT_TREE_INFO: u32 = 0x98;
/// `int sortAxis[8]`: the child order.
const SCT_SORT_AXIS: u32 = 0x9c;
/// `int depth` of the tree.
const SCT_INFO_DEPTH: u32 = 0x0c;
/// `Level* levels`, 12 bytes each: radius, cenOffset, indexOffset.
const SCT_INFO_LEVELS: u32 = 0x20;
/// `unsigned char* bitTree`: one byte per node, bit k set when child k exists.
const SCT_INFO_BIT_TREE: u32 = 0x28;
const SCT_LEVEL_STRIDE: u32 = 12;
const SCT_LEVEL_CEN_OFFSET: u32 = 4;
const SCT_LEVEL_INDEX_OFFSET: u32 = 8;

/// The node function itself: the recursion into a child.
pub(crate) const SCT_RECURSE: Callee = Callee {
    addr: 0x0032_d470,
    f: ee::s11::f_0032d470,
};

/// `PointInTriangle` (0x32d440): is the plane point inside the triangle.
pub(crate) const SCT_POINT_IN_TRIANGLE: Callee = Callee {
    addr: 0x0032_d440,
    f: ee::s11::f_0032d440,
};

/// `base + off` for a register: the address `off(reg)` as the compiled code
/// forms it (32-bit wrap).
#[inline]
fn sct_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void SphereTree_CollideTriangleNode(SphereTree* self, t4Vector* p1,
/// t4Vector* p2, t4Vector* p3, t4Vector* uNorm, t4Vector* pos, int depth,
/// int base, t4Vector* outA, t4Vector* outB)` (0x32d470): tests one node of
/// the sphere tree against a triangle. The plane distance of `pos` is
/// compared with the node's sphere (level radius times scale) and with
/// `worstCase`; the sphere's plane point is tested against the triangle, and
/// a leaf (or a node the triangle misses) that is the closest so far stores
/// its normal and plane point into `outA`/`outB` and raises `worstCase`. An
/// inner node recurses into its children, in `sortAxis` order, for the bits
/// set in its byte.
pub fn sphere_tree_collide_triangle_node(c: &mut Cpu) {
    guest(c, sct_body);
}

fn sct_body(c: &mut Cpu) -> Flow {
    'b_32da0c: {
        'b_32d53c: {
            c.cycles += 46;
            c.g[SP][0] = sx(sp(c).wrapping_sub(0x140));
            c.g[V1][0] = 0xc;
            c.sq(21, sct_at(c, SP, 0xd0));
            c.sq(30, sct_at(c, SP, 0xa0));
            c.g[S5][0] = c.g[A1][0];
            c.sq(17, sct_at(c, SP, 0x110));
            c.g[FP][0] = c.g[T1][0];
            c.sq(19, sct_at(c, SP, 0xf0));
            c.g[S1][0] = c.g[A0][0];
            c.sq(20, sct_at(c, SP, 0xe0));
            c.g[S3][0] = c.g[T0][0];
            c.sq(23, sct_at(c, SP, 0xb0));
            c.g[S4][0] = c.g[T2][0];
            c.swc1(20, sct_at(c, SP, 0x130));
            // depth * 12: the level's offset in `levels`.
            c.mult(0, 3, 20, 3);
            c.sq(16, sct_at(c, SP, 0x120));
            c.g[S7][0] = c.g[A2][0];
            c.sq(18, sct_at(c, SP, 0x100));
            c.sq(22, sct_at(c, SP, 0xc0));
            c.sd(sct_at(c, SP, 0x90), c.g[RA][0]);
            c.swc1(21, sct_at(c, SP, 0x138));
            c.lqc2(3, c.g[FP][0] as u32);
            c.lqc2(4, c.g[S5][0] as u32);
            // planePoint offset: pos - point1.
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sct_at(c, SP, 0x20));
            c.lq(2, sct_at(c, SP, 0x20));
            c.g[A0][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
            c.lwc1(1, sct_at(c, S1, SCT_SCALE));
            c.sq(2, sct_at(c, SP, 0x10));
            c.sw(sct_at(c, SP, 0x80), c.g[A3][0]);
            c.lqc2(3, c.g[S3][0] as u32);
            c.lqc2(5, sct_at(c, SP, 0x10));
            // planeDist = Dot(uNorm, pos - point1).
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 21);
            c.sw(sct_at(c, SP, 0x84), c.g[T3][0]);
            c.mov_s(2, 21);
            c.g[V0][0] = c.lw(sct_at(c, A0, SCT_INFO_LEVELS));
            c.g[V1][0] = sx(sct_at(c, V1, 0).wrapping_add(c.g[V0][0] as u32));
            // levelRadius = levels[depth].radius * self->scale.
            c.lwc1(0, c.g[V1][0] as u32);
            let t = c.g[S4][0] != 0u64;
            c.mul_s(20, 0, 1);
            if t {
                break 'b_32d53c;
            } else {
                // depth 0: a triangle behind the plane is not hit.
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(21, 0);
                let t = c.fcc();
                c.lq(16, sct_at(c, SP, 0x120));
                if t {
                    break 'b_32da0c;
                } else {
                    break 'b_32d53c;
                }
            }
        }
        'b_32da08: {
            // if (!(planeDist < levelRadius + worstCase)) return;
            c.cycles += 6;
            c.lwc1(0, sct_at(c, S1, SCT_WORST_CASE));
            c.add_s(0, 20, 0);
            c.c_lt_s(21, 0);
            let t = !c.fcc();
            c.g[S0][0] = 0u64;
            if t {
                break 'b_32da08;
            } else {
                // leaf (S0 = 1) unless the node has children below depth 3.
                'b_32d58c: {
                    'b_32d588: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(sct_at(c, A0, SCT_INFO_BIT_TREE));
                        c.g[V1][0] = c.lw(sct_at(c, A0, SCT_INFO_DEPTH));
                        c.g[A0][0] = c.lw(sct_at(c, SP, 0x84));
                        c.g[V1][0] = ((c.g[S4][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        let t = c.g[V1][0] == 0u64;
                        c.sw(sct_at(c, SP, 0x88), c.g[V0][0]);
                        if t {
                            break 'b_32d588;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw(sct_at(c, SP, 0x88));
                            if t {
                                break 'b_32d588;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_32d58c;
                                } else {
                                    break 'b_32d588;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                    break 'b_32d58c;
                }
                // The triangle test of the sphere (PointInTriangle on the
                // plane point), then the edge tests when it misses.
                'b_32d8dc: {
                    c.cycles += 22;
                    c.mfc1(2, 2);
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.g[A0][0] = c.g[S1][0];
                    c.lq(2, sct_at(c, SP, 0x30));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A3][0] = c.lw(sct_at(c, SP, 0x80));
                    c.g[A2][0] = c.g[S7][0];
                    c.sq(2, sct_at(c, SP, 0x20));
                    c.g[T0][0] = sx(sct_at(c, SP, 0x10));
                    c.lqc2(3, c.g[FP][0] as u32);
                    c.lqc2(4, sct_at(c, SP, 0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.mov_s(12, 20);
                    c.lq(2, sct_at(c, SP, 0x30));
                    c.sq(2, sct_at(c, SP, 0x10));
                    SCT_POINT_IN_TRIANGLE.call(c, 0x0032_d5dc)?;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_32d8dc;
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != 0u64 {
                            c.lq(16, sct_at(c, SP, 0x120));
                            break 'b_32da0c;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x41a00000;
                            c.mtc1(1, 0);
                            c.c_lt_s(20, 0);
                            if c.fcc() {
                                c.lq(16, sct_at(c, SP, 0x120));
                                break 'b_32da0c;
                            } else {
                                c.cycles += 19;
                                c.lqc2(3, c.g[FP][0] as u32);
                                c.lqc2(4, c.g[S5][0] as u32);
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, sct_at(c, SP, 0x30));
                                c.mul_s(3, 20, 20);
                                c.lq(2, sct_at(c, SP, 0x30));
                                c.sq(2, sct_at(c, SP, 0x20));
                                c.lqc2(3, sct_at(c, SP, 0x20));
                                c.lqc2(5, sct_at(c, SP, 0x20));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(3, 4);
                                c.mtc1(3, 5);
                                c.c_lt_s(3, 5);
                                let t = !c.fcc();
                                if t {
                                    break 'b_32d8dc;
                                } else {
                                    'b_32d70c: {
                                        'b_32d708: {
                                            c.cycles += 31;
                                            c.lqc2(3, c.g[S7][0] as u32);
                                            c.lqc2(4, c.g[S5][0] as u32);
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, sct_at(c, SP, 0x40));
                                            c.mtc1(0, 1);
                                            c.lq(2, sct_at(c, SP, 0x40));
                                            c.sq(2, sct_at(c, SP, 0x30));
                                            c.lqc2(3, sct_at(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(3, 22);
                                            c.mtc1(3, 4);
                                            c.lqc2(3, sct_at(c, SP, 0x20));
                                            c.lqc2(5, sct_at(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.div_s(2, 0, 4);
                                            c.c_lt_s(2, 1);
                                            let t = c.fcc();
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_32d708;
                                            } else {
                                                c.cycles += 6;
                                                c.mul_s(0, 2, 2);
                                                c.sub_s(0, 5, 0);
                                                c.c_lt_s(3, 0);
                                                let t = c.fcc();
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                if t {
                                                    break 'b_32d70c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.sub_s(0, 3, 0);
                                                    c.g[V0][0] = 0x1;
                                                    c.sqrt_s(0, 0);
                                                    c.sub_s(0, 2, 0);
                                                    c.c_lt_s(4, 0);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                    if t {
                                                        break 'b_32d70c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0u64;
                                                        break 'b_32d708;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                        break 'b_32d70c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = c.lw(sct_at(c, SP, 0x80));
                                    if t {
                                        break 'b_32d8dc;
                                    } else {
                                        'b_32d7d0: {
                                            'b_32d7cc: {
                                                c.cycles += 31;
                                                c.lqc2(3, c.g[V1][0] as u32);
                                                c.lqc2(4, c.g[S5][0] as u32);
                                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, sct_at(c, SP, 0x50));
                                                c.mtc1(0, 1);
                                                c.lq(2, sct_at(c, SP, 0x50));
                                                c.sq(2, sct_at(c, SP, 0x40));
                                                c.lqc2(3, sct_at(c, SP, 0x40));
                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                c.vu0().sqrt(4, 0);
                                                c.vu0().waitq();
                                                c.cfc2(3, 22);
                                                c.mtc1(3, 4);
                                                c.lqc2(3, sct_at(c, SP, 0x20));
                                                c.lqc2(5, sct_at(c, SP, 0x40));
                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                c.qmfc2(2, 4);
                                                c.mtc1(2, 0);
                                                c.div_s(2, 0, 4);
                                                c.c_lt_s(2, 1);
                                                let t = c.fcc();
                                                c.g[V0][0] = 0u64;
                                                if t {
                                                    break 'b_32d7cc;
                                                } else {
                                                    c.cycles += 6;
                                                    c.mul_s(0, 2, 2);
                                                    c.sub_s(0, 5, 0);
                                                    c.c_lt_s(3, 0);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                    if t {
                                                        break 'b_32d7d0;
                                                    } else {
                                                        c.cycles += 8;
                                                        c.sub_s(0, 3, 0);
                                                        c.g[V0][0] = 0x1;
                                                        c.sqrt_s(0, 0);
                                                        c.sub_s(0, 2, 0);
                                                        c.c_lt_s(4, 0);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                        if t {
                                                            break 'b_32d7d0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = 0u64;
                                                            break 'b_32d7cc;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            break 'b_32d7d0;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_32d8dc;
                                        } else {
                                            'b_32d8d4: {
                                                'b_32d8d0: {
                                                    c.cycles += 47;
                                                    c.lqc2(3, c.g[FP][0] as u32);
                                                    c.lqc2(4, c.g[S7][0] as u32);
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, sct_at(c, SP, 0x60));
                                                    c.mtc1(0, 1);
                                                    c.lq(2, sct_at(c, SP, 0x60));
                                                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x80));
                                                    c.sq(2, sct_at(c, SP, 0x50));
                                                    c.lqc2(3, sct_at(c, SP, 0x50));
                                                    c.lqc2(5, sct_at(c, SP, 0x50));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(3, 4);
                                                    c.mtc1(3, 5);
                                                    c.lqc2(3, c.g[A0][0] as u32);
                                                    c.lqc2(4, c.g[S7][0] as u32);
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, sct_at(c, SP, 0x70));
                                                    c.lq(2, sct_at(c, SP, 0x70));
                                                    c.sq(2, sct_at(c, SP, 0x60));
                                                    c.lqc2(3, sct_at(c, SP, 0x60));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(3, 22);
                                                    c.mtc1(3, 4);
                                                    c.lqc2(3, sct_at(c, SP, 0x50));
                                                    c.lqc2(5, sct_at(c, SP, 0x60));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.div_s(2, 0, 4);
                                                    c.c_lt_s(2, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_32d8d0;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.mul_s(0, 2, 2);
                                                        c.sub_s(0, 5, 0);
                                                        c.c_lt_s(3, 0);
                                                        let t = c.fcc();
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                        if t {
                                                            break 'b_32d8d4;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.sub_s(0, 3, 0);
                                                            c.g[V0][0] = 0x1;
                                                            c.sqrt_s(0, 0);
                                                            c.sub_s(0, 2, 0);
                                                            c.c_lt_s(4, 0);
                                                            let t = !c.fcc();
                                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                            if t {
                                                                break 'b_32d8d4;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[V0][0] = 0u64;
                                                                break 'b_32d8d0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                break 'b_32d8d4;
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != 0u64 {
                                                c.lq(16, sct_at(c, SP, 0x120));
                                                break 'b_32da0c;
                                            } else {
                                                break 'b_32d8dc;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Inner nodes recurse into the children (sortAxis order, the
                // child bit set in the node's byte); a leaf takes the plane.
                'b_32d9bc: {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.sub_s(12, 21, 20);
                        break 'b_32d9bc;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0xc;
                        c.g[V1][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
                        c.mult(0, 2, 20, 2);
                        c.g[S0][0] = sx(sct_at(c, S1, SCT_SORT_AXIS));
                        c.g[A0][0] = c.lw(sct_at(c, V1, SCT_INFO_LEVELS));
                        c.g[S2][0] = 0x7;
                        c.g[S6][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(20, sct_at(c, V0, SCT_LEVEL_STRIDE + SCT_LEVEL_CEN_OFFSET));
                        'l_32d908: loop {
                            'b_32d9a8: {
                                c.cycles += 6;
                                c.g[T4][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = c.lw(sct_at(c, SP, 0x88));
                                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32
                                    >> ((c.g[T4][0] as u32) & 31))
                                    as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[T4][0] as u32) << 4);
                                if t {
                                    break 'b_32d9a8;
                                } else {
                                    c.cycles += 34;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V0][0] =
                                        sx(sct_at(c, V0, SCT_AXIS).wrapping_add(c.g[S1][0] as u32));
                                    c.g[T4][0] = sx(sct_at(c, T4, 1));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, c.g[V0][0] as u32);
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, sct_at(c, SP, 0x40));
                                    c.g[A1][0] = c.g[S5][0];
                                    c.lq(2, sct_at(c, SP, 0x40));
                                    c.g[A2][0] = c.g[S7][0];
                                    c.g[A3][0] = c.lw(sct_at(c, SP, 0x80));
                                    c.g[T0][0] = c.g[S3][0];
                                    c.sq(2, sct_at(c, SP, 0x30));
                                    c.g[T1][0] = sx(sct_at(c, SP, 0x20));
                                    c.lqc2(3, c.g[FP][0] as u32);
                                    c.lqc2(4, sct_at(c, SP, 0x30));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, sct_at(c, SP, 0x40));
                                    c.g[T2][0] = sx(sct_at(c, S4, 1));
                                    c.lq(3, sct_at(c, SP, 0x40));
                                    c.g[T3][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
                                    c.sq(3, sct_at(c, SP, 0x20));
                                    c.g[V1][0] = c.lw(sct_at(c, SP, 0x148));
                                    c.g[V0][0] = c.lw(sct_at(c, T3, SCT_INFO_LEVELS));
                                    c.g[V0][0] =
                                        sx(sct_at(c, S6, 0).wrapping_add(c.g[V0][0] as u32));
                                    c.g[T3][0] = c.lw(sct_at(c, V0, SCT_LEVEL_INDEX_OFFSET));
                                    c.g[V0][0] = c.lw(sct_at(c, SP, 0x140));
                                    c.sw(sct_at(c, SP, 0x8), c.g[V1][0]);
                                    c.sw(sct_at(c, SP, 0), c.g[V0][0]);
                                    c.g[V0][0] = c.lw(sct_at(c, SP, 0x84));
                                    c.lo[0] = c.g[V0][0];
                                    // child base = (k + 1) * indexOffset + base
                                    c.madd(0, 11, 12, 11);
                                    SCT_RECURSE.call(c, 0x0032_d9a8)?;
                                    break 'b_32d9a8;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0032_d908);
                                }
                                continue 'l_32d908;
                            } else {
                                c.cycles += 2;
                                c.lq(16, sct_at(c, SP, 0x120));
                                break 'b_32da0c;
                            }
                        }
                    }
                }
                // The leaf (or the last child): the plane offset becomes the
                // new closest distance when it is smaller than worstCase.
                c.cycles += 5;
                c.lwc1(0, sct_at(c, S1, SCT_WORST_CASE));
                c.c_lt_s(12, 0);
                let t = !c.fcc();
                c.lq(16, sct_at(c, SP, 0x120));
                if t {
                    break 'b_32da0c;
                } else {
                    c.cycles += 14;
                    c.neg_s(0, 12);
                    c.mfc1(2, 0);
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.lq(3, sct_at(c, SP, 0x30));
                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x140));
                    c.lq(2, sct_at(c, SP, 0x10));
                    c.sq(3, c.g[A0][0] as u32);
                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x148));
                    c.sq(2, c.g[A0][0] as u32);
                    c.swc1(12, sct_at(c, S1, SCT_WORST_CASE));
                    c.sq(3, sct_at(c, SP, 0x20));
                    break 'b_32da08;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, sct_at(c, SP, 0x120));
        break 'b_32da0c;
    }
    c.cycles += 13;
    c.lq(17, sct_at(c, SP, 0x110));
    c.lq(18, sct_at(c, SP, 0x100));
    c.lq(19, sct_at(c, SP, 0xf0));
    c.lq(20, sct_at(c, SP, 0xe0));
    c.lq(21, sct_at(c, SP, 0xd0));
    c.lq(22, sct_at(c, SP, 0xc0));
    c.lq(23, sct_at(c, SP, 0xb0));
    c.lq(30, sct_at(c, SP, 0xa0));
    c.g[RA][0] = c.ld(sct_at(c, SP, 0x90));
    c.lwc1(21, sct_at(c, SP, 0x138));
    c.lwc1(20, sct_at(c, SP, 0x130));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sct_at(c, SP, 0x140));
    c.pc = t;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self as dt, check, scramble, Rng};

    /// Fixture layout (all inside `dt::DATA..dt::DATA + 0xfffff`).
    const SELF: u32 = dt::DATA;
    const TREE: u32 = dt::DATA + 0x200;
    const LEVELS: u32 = dt::DATA + 0x400;
    const BITS: u32 = dt::DATA + 0x600;
    const UNORM: u32 = dt::DATA + 0x800;
    const POS: u32 = dt::DATA + 0x820;
    const P1: u32 = dt::DATA + 0x840;
    const P2: u32 = dt::DATA + 0x860;
    const P3: u32 = dt::DATA + 0x880;
    const OUT_A: u32 = dt::DATA + 0x900;
    const OUT_B: u32 = dt::DATA + 0x920;

    fn set_vec(c: &mut Cpu, addr: u32, r: &mut Rng) {
        for i in 0..4 {
            c.bus.write32(addr + 4 * i, r.f32(8.0));
        }
    }

    #[test]
    fn sphere_tree_collide_triangle_node_matches() {
        check(
            ee::s11::f_0032d470__generated,
            sphere_tree_collide_triangle_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x5eed_0562 ^ seed as u64);
                scramble(c, &mut r);
                // The node: a unit scale, a worst case that sometimes lets
                // every plane offset through, child axes and order.
                c.bus.write32(
                    SELF + SCT_SCALE,
                    if seed % 2 == 0 {
                        0x3f80_0000
                    } else {
                        r.f32(2.0)
                    },
                );
                c.bus.write32(
                    SELF + SCT_WORST_CASE,
                    if seed % 3 == 0 {
                        0x7f7f_ffff
                    } else {
                        r.f32(4.0)
                    },
                );
                c.bus.write32(SELF + SCT_TREE_INFO, TREE);
                for k in 0..8u32 {
                    set_vec(c, SELF + SCT_AXIS + 16 * k, &mut r);
                    c.bus.write32(SELF + SCT_SORT_AXIS + 4 * k, (7 - k) % 8);
                }
                // The tree: depth 0..2, levels with radius and offsets.
                c.bus.write32(TREE + SCT_INFO_DEPTH, (seed % 3) + 1);
                c.bus.write32(TREE + SCT_INFO_LEVELS, LEVELS);
                c.bus.write32(TREE + SCT_INFO_BIT_TREE, BITS);
                for l in 0..4u32 {
                    let base = LEVELS + SCT_LEVEL_STRIDE * l;
                    c.bus.write32(base, r.f32(30.0) & 0x7fff_ffff);
                    c.bus.write32(base + SCT_LEVEL_CEN_OFFSET, r.f32(10.0));
                    c.bus.write32(base + SCT_LEVEL_INDEX_OFFSET, 1 + l);
                }
                for i in 0..4u32 {
                    c.bus
                        .write8(BITS + i, if seed % 5 == 0 { 0 } else { r.next_u32() as u8 });
                }
                set_vec(c, UNORM, &mut r);
                set_vec(c, POS, &mut r);
                set_vec(c, P1, &mut r);
                set_vec(c, P2, &mut r);
                set_vec(c, P3, &mut r);
                set_vec(c, OUT_A, &mut r);
                set_vec(c, OUT_B, &mut r);
                // Stack arguments 9 and 10.
                c.bus.write32(dt::SP + 0x140, OUT_A);
                c.bus.write32(dt::SP + 0x148, OUT_B);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(P1);
                c.g[A2][0] = sx(P2);
                c.g[A3][0] = sx(P3);
                c.g[T0][0] = sx(UNORM);
                c.g[T1][0] = sx(POS);
                c.g[T2][0] = sx(seed % 3);
                c.g[T3][0] = sx(seed % 2);
                c.g[SP][0] = sx(dt::SP);
                c.g[RA][0] = sx(dt::RA);
                c.pc = 0x0032_d470;
            },
        );
    }
}
