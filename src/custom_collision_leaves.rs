//! Native triangle queries over a relocated static mesh collision leaf.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_c0f8,
    collide_mesh_leaf_triangles,
    "Collide_MeshLeafTriangles",
)];

const QUERY_VTABLE: u32 = 0x50;
const FACE_COUNT: u32 = 0;
const INDICES: u32 = 4;
const BOXES: u32 = 8;
const VERTICES: u32 = 0xc;
const NORMALS: u32 = 0x10;
const TWO_SIDED: u32 = 0x24;
const OUTPUT: u32 = 0x28;
const HITS: u32 = 0x2c;
const BOX_COUNT: u32 = 0x30;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Set up this face's normal and apply the optional query-plane rejection.
fn face_is_rejected(c: &mut Cpu, threshold: u32) -> bool {
    c.cycles += 5;
    c.g[V0][0] = c.lw(r(c, S3).wrapping_add(NORMALS));
    c.g[V1][0] = sx(r(c, S1) << 4);
    c.g[A0][0] = c.lw(r(c, S2).wrapping_add(8));
    let no_cull = c.g[A0][0] == 0;
    c.g[T0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    if no_cull {
        return false;
    }
    c.cycles += 14;
    c.lwc1(0, r(c, GP).wrapping_sub(threshold));
    c.lqc2(3, r(c, T0));
    c.lqc2(5, r(c, S2).wrapping_add(0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    c.fcc()
}

/// Dispatch the query's single- or double-sided triangle method. T0 keeps
/// the face normal; the three byte indices address vec4 vertices.
fn query_triangle(c: &mut Cpu, grouped: bool) -> Flow {
    c.cycles += 14;
    c.g[V0][0] = c.lw(r(c, S3).wrapping_add(INDICES));
    c.g[A2][0] = c.lw(r(c, S3).wrapping_add(VERTICES));
    c.g[V0][0] = sx(r(c, S5).wrapping_add(r(c, V0)));
    c.g[V1][0] = c.lbu(r(c, V0).wrapping_add(2));
    c.g[A0][0] = c.lbu(r(c, V0));
    c.g[A1][0] = c.lbu(r(c, V0).wrapping_add(1));
    c.g[V1][0] = sx(r(c, V1) << 4);
    c.g[A3][0] = sx(r(c, A2).wrapping_add(r(c, V1)));
    c.g[A0][0] = sx(r(c, A0) << 4);
    c.g[A1][0] = sx(r(c, A1) << 4);
    c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(TWO_SIDED));
    let two_sided = c.g[V1][0] != 0;
    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
    let (slot, ret) = if two_sided {
        c.cycles += 11;
        (0x28, if grouped { 0x0032_c410 } else { 0x0032_c210 })
    } else {
        c.cycles += 9;
        (0x20, if grouped { 0x0032_c43c } else { 0x0032_c23c })
    };
    c.g[V1][0] = c.lw(r(c, S2).wrapping_add(QUERY_VTABLE));
    c.g[A1][0] = c.g[V0][0];
    c.g[T1][0] = c.g[SP][0];
    c.g[T2][0] = sx(r(c, SP).wrapping_add(0x10));
    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(slot));
    c.g[T3][0] = sx(r(c, SP).wrapping_add(0x20));
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(slot + 4));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
    call_ptr(c, target, ret)
}

/// Populate one 0x80-byte native collision result. Preserve the slightly
/// different load/store order of the original's two copies of this block.
fn append_contact(c: &mut Cpu, grouped: bool) {
    c.cycles += 27;
    let empty = if grouped { S6 } else { S4 };
    let zero_vector = if grouped { S7 } else { S6 };
    c.sw(r(c, S0).wrapping_add(0x54), 0);
    c.sw(r(c, S0).wrapping_add(0x44), c.g[V0][0]);
    c.g[A0][0] = c.lw(r(c, SP).wrapping_add(HITS));
    c.g[A0][0] = sx(r(c, A0).wrapping_add(1));
    c.sw(r(c, SP).wrapping_add(HITS), c.g[A0][0]);
    c.sw(r(c, S0).wrapping_add(0x60), c.g[empty][0]);
    c.sw(r(c, S0).wrapping_add(0x50), 0);
    c.sw(r(c, S0).wrapping_add(0x58), 0);
    c.sw(r(c, S0).wrapping_add(0x5c), c.g[empty][0]);
    c.sw(r(c, S0).wrapping_add(0x64), c.g[empty][0]);
    c.sw(r(c, S0).wrapping_add(0x4c), c.g[empty][0]);
    c.lq(V0, r(c, zero_vector));
    if grouped {
        c.sq(V0, r(c, S0).wrapping_add(0x20));
        c.lq(V1, r(c, zero_vector));
        c.sw(r(c, S0).wrapping_add(0x68), 0);
        c.sq(V1, r(c, S0).wrapping_add(0x30));
        c.sw(r(c, S0).wrapping_add(0x6c), 0);
        c.sw(r(c, S0).wrapping_add(0x70), 0);
        c.lq(V0, r(c, SP).wrapping_add(0x10));
        c.lq(V1, r(c, SP));
        c.sq(V0, r(c, S0).wrapping_add(0x10));
        c.sq(V1, r(c, S0));
        c.lwc1(0, r(c, SP).wrapping_add(0x20));
        c.sw(r(c, S0).wrapping_add(0x60), c.g[S1][0]);
        c.swc1(0, r(c, S0).wrapping_add(0x40));
    } else {
        c.lq(V1, r(c, SP));
        c.lq(A0, r(c, SP).wrapping_add(0x10));
        c.sq(V0, r(c, S0).wrapping_add(0x20));
        c.lwc1(0, r(c, SP).wrapping_add(0x20));
        c.lq(V0, r(c, zero_vector));
        c.sq(V1, r(c, S0));
        c.sq(V0, r(c, S0).wrapping_add(0x30));
        c.sq(A0, r(c, S0).wrapping_add(0x10));
        c.swc1(0, r(c, S0).wrapping_add(0x40));
        c.sw(r(c, S0).wrapping_add(0x68), 0);
        c.sw(r(c, S0).wrapping_add(0x6c), 0);
        c.sw(r(c, S0).wrapping_add(0x70), 0);
        c.sw(r(c, S0).wrapping_add(0x60), c.g[S1][0]);
    }
    c.sw(r(c, S0).wrapping_add(0x54), 0);
    c.g[S0][0] = sx(r(c, S0).wrapping_add(0x80));
}

/// `int Collide_MeshLeafTriangles(Query* query, MeshLeaf* mesh,
/// int twoSided, Contact* output)` (0x32c0f8).
///
/// Up to ten faces are tested directly. Larger leaves store one vec3 min/max
/// pair per consecutive ten faces, permitting a box query before each batch.
pub fn collide_mesh_leaf_triangles(c: &mut Cpu) {
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
        c.g[V0][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
        c.sw(r(c, SP).wrapping_add(TWO_SIDED), c.g[A2][0]);
        c.sw(r(c, SP).wrapping_add(OUTPUT), c.g[A3][0]);
        c.g[V0][0] = (c.g[V0][0] < 11) as u64;
        let small = c.g[V0][0] != 0;
        c.sw(r(c, SP).wrapping_add(HITS), 0);
        let mut direct_return = false;
        if small {
            c.cycles += 3;
            c.g[V0][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
            let none = c.g[V0][0] == 0;
            c.g[S1][0] = 0;
            if !none {
                c.cycles += 6;
                c.g[V0][0] = 0x0050_0000;
                c.g[S0][0] = c.g[A3][0];
                c.g[S6][0] = sx(r(c, V0).wrapping_sub(0xee0));
                c.g[S4][0] = u64::MAX;
                c.g[S5][0] = 0;
                loop {
                    if face_is_rejected(c, 0x2d88) {
                        c.g[V0][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
                    } else {
                        query_triangle(c, false)?;
                        c.cycles += 2;
                        let hit = c.g[V0][0] != 0;
                        c.g[V0][0] = 2;
                        if hit {
                            append_contact(c, false);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S5][0] = sx(r(c, S5).wrapping_add(3));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_c168);
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw(r(c, SP).wrapping_add(HITS));
                direct_return = true;
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
            c.g[V1][0] = 10;
            c.cycles += 6;
            c.g[S0][0] = 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_add(9));
            c.div(0, V0, V1);
            c.g[A0][0] = c.lo[0];
            let none = c.g[A0][0] == 0;
            c.sw(r(c, SP).wrapping_add(BOX_COUNT), c.g[A0][0]);
            if !none {
                c.cycles += 1;
                c.g[V1][0] = 24;
                loop {
                    c.cycles += 12;
                    c.g[A2][0] = c.lw(r(c, S2).wrapping_add(QUERY_VTABLE));
                    c.mult(0, V1, S0, V1);
                    c.g[A1][0] = c.lw(r(c, S3).wrapping_add(BOXES));
                    c.g[A0][0] = c.lh(r(c, A2).wrapping_add(0x48));
                    c.g[FP][0] = sx(r(c, S0).wrapping_add(1));
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x4c));
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
                    let target = r(c, V0);
                    c.g[A2][0] = sx(r(c, A1).wrapping_add(12));
                    call_ptr(c, target, 0x0032_c320)?;
                    let box_hit = c.g[V0][0] != 0;
                    c.g[V0][0] = 10;
                    if box_hit {
                        c.cycles += 8;
                        c.g[S4][0] = c.lhu(r(c, S3).wrapping_add(FACE_COUNT));
                        c.mult(0, A0, FP, V0);
                        c.mult(0, S1, S0, V0);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S4][0] = c.g[A0][0];
                        }
                        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let no_faces = c.g[V1][0] == 0;
                        c.g[V0][0] = 0x0050_0000;
                        if !no_faces {
                            c.cycles += 8;
                            c.g[A0][0] = sx(r(c, S1) << 1);
                            c.g[S7][0] = sx(r(c, V0).wrapping_sub(0xee0));
                            c.g[S5][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
                            c.g[V0][0] = c.lw(r(c, SP).wrapping_add(HITS));
                            c.g[S6][0] = u64::MAX;
                            c.g[A0][0] = c.lw(r(c, SP).wrapping_add(OUTPUT));
                            c.g[V1][0] = sx(r(c, V0) << 7);
                            c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                            loop {
                                if face_is_rejected(c, 0x2d84) {
                                    c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                                } else {
                                    query_triangle(c, true)?;
                                    c.cycles += 2;
                                    let hit = c.g[V0][0] != 0;
                                    c.g[V0][0] = 2;
                                    if hit {
                                        append_contact(c, true);
                                    }
                                    c.cycles += 1;
                                    c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[S5][0] = sx(r(c, S5).wrapping_add(3));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0032_c368);
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(BOX_COUNT));
                    c.g[S0][0] = c.g[FP][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = 24;
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_c2f8);
                    }
                }
            }
        }
        if !direct_return {
            c.cycles += 1;
            c.g[V0][0] = c.lw(r(c, SP).wrapping_add(HITS));
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
