//! Staging module oc596 (wave l586b.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc596`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_92a8,
        float_rail_node_update_mats,
        "cFloatRailNode_updateMats",
    ),
    (
        0x0034_98e8,
        float_rail_node_update_nodes,
        "cFloatRailNode_updateNodes",
    ),
];

/// `World_GetInstance` (installed; returns the world object).
const WORLD_GET_INSTANCE: u32 = 0x002d_1bd8;
/// `World_GetSecondsPerTick` (installed; the update time in f0).
const WORLD_SECONDS_PER_TICK: u32 = 0x002d_1c70;
/// `cFloatRailNode_getPosition` (generated only, no override slot).
const FLOAT_RAIL_NODE_GET_POSITION: Callee = Callee {
    addr: 0x0034_9220,
    f: ee::s11::f_00349220,
};

/// `base + off` as a guest address (the 32-bit wrap of a 64-bit register).
#[inline]
fn float_rail_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// One Catmull-Rom component pass (generated shape): load four float fields
/// (`loads`: fpu register, address), store the registers at `stack + off`
/// (`stores`), load the gathered vector from `stack + vec` into vf8, load
/// the basis matrix at `basis` into vf4..vf7, multiply on VU0 and store vf12
/// to `stack + out`.
fn float_rail_spline_pass(
    c: &mut Cpu,
    stack: u32,
    basis: u32,
    loads: &[(usize, u32)],
    stores: &[(usize, u32)],
    vec: u32,
    out: u32,
) {
    for &(f, a) in loads {
        c.lwc1(f, a);
    }
    for &(f, off) in stores {
        c.swc1(f, stack.wrapping_add(off));
    }
    c.lqc2(8, stack.wrapping_add(vec));
    c.lqc2(4, basis);
    c.lqc2(5, basis.wrapping_add(0x10));
    c.lqc2(6, basis.wrapping_add(0x20));
    c.lqc2(7, basis.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, stack.wrapping_add(out));
}

/// Transpose the 4x4 matrix at `m` in place: load the four rows into `x`,
/// interleave words and dwords through `t`, store the rows back.
fn float_rail_transpose(c: &mut Cpu, m: u32, x: [usize; 4], t: [usize; 4]) {
    for (i, &r) in x.iter().enumerate() {
        c.lq(r, m.wrapping_add(0x10 * i as u32));
    }
    c.pextlw(t[0], x[1], x[0]);
    c.pextuw(t[1], x[1], x[0]);
    c.pextlw(t[2], x[3], x[2]);
    c.pextuw(t[3], x[3], x[2]);
    c.pcpyld(x[0], t[2], t[0]);
    c.pcpyud(x[1], t[0], t[2]);
    c.pcpyld(x[2], t[3], t[1]);
    c.pcpyud(x[3], t[1], t[3]);
    for (i, &r) in x.iter().enumerate() {
        c.sq(r, m.wrapping_add(0x10 * i as u32));
    }
}

/// Write the matrix rows `Mats[i]` from the stack: rows 0..2 are the x, y and
/// z results, row 3 is (0, 0, 0, 1) (`one` holds 1.0), then transpose.
fn float_rail_store_rows(
    c: &mut Cpu,
    stack: u32,
    m: u32,
    one: usize,
    x: [usize; 4],
    t: [usize; 4],
) {
    c.lq(2, stack.wrapping_add(0x40));
    c.lq(3, stack);
    c.sq(2, stack.wrapping_add(0x20));
    c.sq(3, m);
    c.lq(2, stack.wrapping_add(0x10));
    c.sq(2, m.wrapping_add(0x10));
    c.lq(3, stack.wrapping_add(0x20));
    c.sq(3, m.wrapping_add(0x20));
    c.sw(stack.wrapping_add(0x30), 0);
    c.sw(stack.wrapping_add(0x34), 0);
    c.sw(stack.wrapping_add(0x38), 0);
    c.swc1(one, stack.wrapping_add(0x3c));
    c.lq(2, stack.wrapping_add(0x30));
    c.sq(2, m.wrapping_add(0x30));
    float_rail_transpose(c, m, x, t);
}

/// `void cFloatRailNode_updateMats(cFloatRailNode* self)` (0x3492a8),
/// `FloatRail_v48`: rebuilds `Mats[i]` for every node but the last two, from
/// the positions of the four nodes `i-1 .. i+2`. The first matrix uses node 0
/// twice, the last one repeats node `i+1`.
///
/// `self`: `NumNodes` at +0x20, `Mats` at +0x50, `Nodes` at +0x70. A node is
/// 0x50 bytes with `Pos` at +0x20. Each component (x, y, z) of the four
/// positions is gathered into one vector and multiplied on VU0 by the basis
/// matrix in the game data at 0x4ffac0; the three results form the first
/// three rows of the matrix, which is then transposed.
///
/// The generated code uses several scratch registers and stack slots, and
/// those values are visible after the call, so the port keeps every write.
pub fn float_rail_node_update_mats(c: &mut Cpu) {
    c.cycles += 99;
    c.g[T5][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
    let stack = sp(c);
    c.g[V1][0] = c.lw(float_rail_ea(c.g[T5][0], 0x70));
    c.g[A1][0] = 0x500000;
    c.g[A0][0] = c.lw(float_rail_ea(c.g[T5][0], 0x50));
    c.g[T7][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x540));
    c.g[T6][0] = 1;
    let basis = c.g[T7][0] as u32;

    // Node 0 (the first matrix): (P0, P0, P1, P2) per component.
    let nodes = c.g[V1][0];
    float_rail_spline_pass(
        c,
        stack,
        basis,
        &[
            (2, float_rail_ea(nodes, 0x20)),
            (0, float_rail_ea(nodes, 0x70)),
            (1, float_rail_ea(nodes, 0xc0)),
        ],
        &[(2, 0x14), (0, 0x18), (1, 0x1c), (2, 0x10)],
        0x10,
        0x20,
    );
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lq(2, stack.wrapping_add(0x20));
    c.sq(2, stack);

    float_rail_spline_pass(
        c,
        stack,
        basis,
        &[
            (2, float_rail_ea(nodes, 0x24)),
            (0, float_rail_ea(nodes, 0x74)),
            (1, float_rail_ea(nodes, 0xc4)),
        ],
        &[(2, 0x24), (0, 0x28), (1, 0x2c), (2, 0x20)],
        0x20,
        0x30,
    );
    c.lq(2, stack.wrapping_add(0x30));
    c.sq(2, stack.wrapping_add(0x10));

    float_rail_spline_pass(
        c,
        stack,
        basis,
        &[
            (2, float_rail_ea(nodes, 0xc8)),
            (1, float_rail_ea(nodes, 0x28)),
            (0, float_rail_ea(nodes, 0x78)),
        ],
        &[(1, 0x34), (0, 0x38), (2, 0x3c), (1, 0x30)],
        0x30,
        0x40,
    );
    let mats = c.g[A0][0] as u32;
    float_rail_store_rows(c, stack, mats, 3, [12, 11, 10, 9], [8, 7, 6, 3]);

    c.g[V0][0] = c.lw(float_rail_ea(c.g[T5][0], 0x20));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
    c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[T8][0] = c.g[A1][0];

    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.mov_s(4, 3);
        c.g[T4][0] = 0x50;
        loop {
            // Node i (T6): the four neighbours i-1, i, i+1, i+2 in the three
            // component passes; T3 = Mats[i], T4 = 0x50 * i.
            c.cycles += 106;
            c.g[V1][0] = c.lw(float_rail_ea(c.g[T5][0], 0x70));
            c.g[T3][0] = sx((c.g[T6][0] as u32) << 6);
            c.g[V0][0] = c.lw(float_rail_ea(c.g[T5][0], 0x50));
            c.g[A2][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.g[A2][0];
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.g[A0][0];
            c.g[V1][0] = c.g[A0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc0));

            let (v1, a0, a1, a2) = (
                c.g[V1][0] as u32,
                c.g[A0][0] as u32,
                c.g[A1][0] as u32,
                c.g[A2][0] as u32,
            );
            float_rail_spline_pass(
                c,
                stack,
                basis,
                &[(1, v1), (3, a0), (0, a1), (2, a2)],
                &[(3, 0x10), (0, 0x14), (1, 0x18), (2, 0x1c)],
                0x10,
                0x20,
            );
            c.lq(2, stack.wrapping_add(0x20));
            c.sq(2, stack);

            float_rail_spline_pass(
                c,
                stack,
                basis,
                &[
                    (2, v1.wrapping_add(0x4)),
                    (0, a0.wrapping_add(0x4)),
                    (1, a1.wrapping_add(0x4)),
                    (3, a2.wrapping_add(0x4)),
                ],
                &[(0, 0x20), (1, 0x24), (2, 0x28), (3, 0x2c)],
                0x20,
                0x30,
            );
            c.lq(2, stack.wrapping_add(0x30));
            c.sq(2, stack.wrapping_add(0x10));

            float_rail_spline_pass(
                c,
                stack,
                basis,
                &[
                    (1, a0.wrapping_add(0x8)),
                    (2, v1.wrapping_add(0x8)),
                    (3, a2.wrapping_add(0x8)),
                    (0, a1.wrapping_add(0x8)),
                ],
                &[(1, 0x30), (0, 0x34), (2, 0x38), (3, 0x3c)],
                0x30,
                0x40,
            );
            let t3 = c.g[T3][0] as u32;
            float_rail_store_rows(c, stack, t3, 4, [10, 9, 8, 7], [6, 5, 4, 3]);

            c.g[V0][0] = c.lw(float_rail_ea(c.g[T5][0], 0x20));
            c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x50));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_9440);
            }
        }
    }

    // Last matrix (T6 = NumNodes - 2): (P(i-1), P(i), P(i+1), P(i+1)).
    c.cycles += 107;
    c.g[V1][0] = 0x50;
    c.g[V0][0] = c.lw(float_rail_ea(c.g[T5][0], 0x70));
    c.mult(0, 3, 14, 3);
    c.g[A1][0] = c.lw(float_rail_ea(c.g[T5][0], 0x50));
    c.g[A0][0] = sx((c.g[T6][0] as u32) << 6);
    c.g[T0][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x540));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 4);
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = c.g[A3][0];
    c.g[A2][0] = c.g[A1][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x70));

    let last_basis = c.g[T0][0] as u32;
    let (v1, a1, a2, a3) = (
        c.g[V1][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u32,
        c.g[A3][0] as u32,
    );
    float_rail_spline_pass(
        c,
        stack,
        last_basis,
        &[(2, v1), (0, a1), (1, a2), (3, a3)],
        &[(0, 0x10), (1, 0x14), (2, 0x18), (3, 0x1c)],
        0x10,
        0x20,
    );
    c.lq(2, stack.wrapping_add(0x20));
    c.sq(2, stack);

    float_rail_spline_pass(
        c,
        stack,
        last_basis,
        &[
            (2, v1.wrapping_add(0x4)),
            (0, a1.wrapping_add(0x4)),
            (1, a2.wrapping_add(0x4)),
            (3, a3.wrapping_add(0x4)),
        ],
        &[(0, 0x20), (1, 0x24), (2, 0x28), (3, 0x2c)],
        0x20,
        0x30,
    );
    c.lq(2, stack.wrapping_add(0x30));
    c.sq(2, stack.wrapping_add(0x10));

    float_rail_spline_pass(
        c,
        stack,
        last_basis,
        &[
            (1, a1.wrapping_add(0x8)),
            (2, v1.wrapping_add(0x8)),
            (3, a3.wrapping_add(0x8)),
            (0, a2.wrapping_add(0x8)),
        ],
        &[(1, 0x30), (0, 0x34), (2, 0x38), (3, 0x3c)],
        0x30,
        0x40,
    );
    let a0 = c.g[A0][0] as u32;
    float_rail_store_rows(c, stack, a0, 4, [11, 10, 9, 8], [7, 6, 5, 3]);

    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
    c.pc = ra;
}

/// The node loop's test `if (getRailInstance(n->InstId)->m_pActiveNode)
/// cFloatRailNode_getPosition(self, n)` (the `0x349938` and `0x349990` blocks
/// of the generated body). `S0` is the node, `S1` the rail node; the lookup
/// tables are read as the generated body does.
fn float_rail_refresh_node(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    call_ptr(c, WORLD_GET_INSTANCE, 0x0034_9938)?;
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = c.lw(float_rail_ea(c.g[S0][0], 0x40));
    c.g[A0][0] = c.lw(float_rail_ea(c.g[V1][0], 0x8));
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let rail_missing = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
    let mut found = false;
    if !rail_missing {
        c.cycles += 7;
        c.g[A0][0] = c.lw(float_rail_ea(c.g[V1][0], 0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        found = nonzero;
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    // The `m_pActiveNode` field, read through the pointer in V0.
    c.cycles += 3;
    c.g[V0][0] = c.lw(float_rail_ea(c.g[V0][0], 0xc));
    let inactive = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S1][0];
    if !inactive {
        c.cycles += 2;
        c.g[A1][0] = c.g[S0][0];
        FLOAT_RAIL_NODE_GET_POSITION.call(c, 0x0034_9990)?;
    }
    Ok(())
}

/// `void cFloatRailNode_updateNodes(cFloatRailNode* self)` (0x3498e8),
/// `FloatRail_v47`: time-steps every node as a damped spring toward its
/// rest position: `acc = -k * (Pos - OPos) + Force * ForceScale - Vel *
/// LDamp`, then `Pos += Vel * dt`, `Vel += acc * dt`, and `Force = 0`. Before
/// the step a node whose rail instance has an active node is refreshed with
/// `cFloatRailNode_getPosition`.
///
/// `self`: `NumNodes` at +0x20, `Nodes` at +0x70, `LSpring` at +0x60,
/// `LDamp` at +0x64, `ForceScale` at +0x68. A node is 0x50 bytes: `Force`
/// +0x00, `Vel` +0x10, `Pos` +0x20, `OPos` +0x30, `InstId` +0x40. `dt` is
/// `World_GetSecondsPerTick()`.
pub fn float_rail_node_update_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 17, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0xa0);
        frame.save(c, 18, 0x80);
        frame.save_fpr(c, 20, 0xb0);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S2][0] = 0;
        call_ptr(c, WORLD_SECONDS_PER_TICK, 0x0034_9914)?;

        c.g[V0][0] = c.lw(float_rail_ea(c.g[S1][0], 0x20));
        c.mov_s(20, 0);
        let no_nodes = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = c.lw(float_rail_ea(c.g[S1][0], 0x70));
        if !no_nodes {
            c.cycles += 3;
            c.g[V0][0] = 0x500000;
            c.g[S3][0] = sx(float_rail_ea(c.g[S1][0], 0x60));
            c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            loop {
                float_rail_refresh_node(c)?;

                c.cycles += 71;
                c.lwc1(0, float_rail_ea(c.g[S1][0], 0x60));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.lqc2(3, float_rail_ea(c.g[S0][0], 0x20));
                c.lqc2(4, float_rail_ea(c.g[S0][0], 0x30));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x30));
                c.lq(2, sp(c).wrapping_add(0x30));
                c.neg_s(0, 0);
                c.sq(2, sp(c).wrapping_add(0x20));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c).wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x30));
                c.lq(2, sp(c).wrapping_add(0x30));
                c.lwc1(0, float_rail_ea(c.g[S3][0], 0x8));
                c.sq(2, sp(c).wrapping_add(0x10));
                c.mfc1(3, 0);
                c.lqc2(4, float_rail_ea(c.g[S0][0], 0x0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x40));
                c.lq(2, sp(c).wrapping_add(0x40));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.lqc2(3, sp(c).wrapping_add(0x10));
                c.lqc2(4, sp(c).wrapping_add(0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x40));
                c.lwc1(0, float_rail_ea(c.g[S1][0], 0x64));
                c.lq(2, sp(c).wrapping_add(0x40));
                c.neg_s(0, 0);
                c.sq(2, sp(c));
                c.mfc1(3, 0);
                c.lqc2(4, float_rail_ea(c.g[S0][0], 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, sp(c));
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c));
                c.mfc1(3, 20);
                c.lqc2(4, float_rail_ea(c.g[S0][0], 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, float_rail_ea(c.g[S0][0], 0x20));
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, float_rail_ea(c.g[S0][0], 0x20));
                c.mfc1(3, 20);
                c.lqc2(4, sp(c));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, float_rail_ea(c.g[S0][0], 0x10));
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, float_rail_ea(c.g[S0][0], 0x10));
                c.lq(2, c.g[S4][0] as u32);
                c.sq(2, c.g[S0][0] as u32);

                c.g[V0][0] = c.lw(float_rail_ea(c.g[S1][0], 0x20));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9930);
                }
            }
        }

        c.cycles += 9;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The rail node (`self`) and its node array, `Mats` and the lookup tables.
    const SELF: u32 = DATA + 0x4000;
    const MATS: u32 = DATA + 0x2000;
    const NODES: u32 = DATA + 0x3000;

    /// Gate for the fakes: a magic word in this machine's RAM.
    const CTRL: u32 = DATA + 0x20000;
    const MAGIC: u32 = 0x0596_0596;
    const TBL: u32 = DATA + 0x6000;
    const RAIL: u32 = DATA + 0x6400;
    const TBL2: u32 = DATA + 0x6800;
    const WORLD: u32 = DATA + 0x6c00;
    const WORLD_OBJ: u32 = DATA + 0x6d00;
    const ACTIVE: u32 = DATA + 0x9000;

    static OC596_PREV_WORLD: AtomicUsize = AtomicUsize::new(0);
    static OC596_PREV_DT: AtomicUsize = AtomicUsize::new(0);
    static OC596_PREV_MATRIX: AtomicUsize = AtomicUsize::new(0);

    /// Gated `World_GetInstance`: returns the fixture's world object.
    fn oc596_fake_world_instance(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_WORLD, ee::s09::f_002d1bd8__generated);
        }
        c.g[V0][0] = WORLD as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `World_GetSecondsPerTick`: the step is set in the fixture.
    fn oc596_fake_seconds_per_tick(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_DT, ee::s09::f_002d1c70__generated);
        }
        c.f[0] = c.bus.read32(CTRL + 4);
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `Instance_GetNodeMatrix`: leaves the output on the stack alone.
    fn oc596_fake_node_matrix(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_MATRIX, ee::s11::f_0034fed8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc596_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BD8),
            oc596_fake_world_instance,
            &OC596_PREV_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1C70),
            oc596_fake_seconds_per_tick,
            &OC596_PREV_DT,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FED8),
            oc596_fake_node_matrix,
            &OC596_PREV_MATRIX,
        );
    }

    #[test]
    fn float_rail_node_update_mats_matches() {
        check(
            ee::s11::f_003492a8__generated,
            float_rail_node_update_mats,
            240,
            &|c, seed| {
                let mut r = Rng(0x0034_92a8 ^ ((seed as u64) << 8) | 1);
                scramble(c, &mut r);
                elf_image(c);
                fill(c, &mut r, MATS, 0x80);
                fill(c, &mut r, NODES, 0x100);
                // NumNodes 1..6: one and two nodes skip the loop.
                c.bus.write32(SELF + 0x20, 1 + seed % 6);
                c.bus.write32(SELF + 0x50, MATS);
                c.bus.write32(SELF + 0x70, NODES);
                c.g[A0][0] = sx(SELF);
                c.pc = 0x0034_92a8;
            },
        );
    }

    #[test]
    fn float_rail_node_update_nodes_matches() {
        oc596_install();
        check(
            ee::s11::f_003498e8__generated,
            float_rail_node_update_nodes,
            200,
            &|c, seed| {
                let mut r = Rng(0x0034_98e8 ^ ((seed as u64) << 8) | 1);
                scramble(c, &mut r);
                elf_image(c);
                fill(c, &mut r, SELF, 0x40);
                fill(c, &mut r, NODES, 0x100);
                // NumNodes 1..5: one and two nodes skip the loop.
                c.bus.write32(SELF + 0x20, 1 + seed % 5);
                c.bus.write32(SELF + 0x70, NODES);
                for i in 0..5u32 {
                    // InstId: small ids, so the table index is InstId & 0xff.
                    c.bus.write32(NODES + 0x50 * i + 0x40, (seed + i) % 4);
                }
                for id in 0..4u32 {
                    let has_rail = (seed + id) % 5 != 0;
                    c.bus
                        .write32(TBL + 4 * id, if has_rail { RAIL + 0x40 * id } else { 0 });
                    c.bus.write32(RAIL + 0x40 * id + 0x1c, TBL2);
                }
                // The second table entry decides the active node: none, or a
                // pointer whose field at +0xc is zero or not.
                let low = r.next_u32() & 0xff;
                let second = if seed % 3 == 0 {
                    low
                } else {
                    ((ACTIVE >> 2) << 8) | low
                };
                c.bus.write32(TBL2, second);
                c.bus
                    .write32(ACTIVE + 0xc, if seed % 4 == 0 { 0 } else { 0x1234 + seed });
                c.bus.write32(WORLD, WORLD_OBJ);
                c.bus.write32(WORLD_OBJ + 8, TBL);
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, 0x3c88_8889);
                c.g[A0][0] = sx(SELF);
                c.pc = 0x0034_98e8;
            },
        );
    }
}
