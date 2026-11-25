//! Staging module oc661 (wave 234): hand-written spring-rail node code, each
//! with its differential test against the generated body. The four functions
//! are the `cSpringRailNode` constructors (0x349b48, 0x349db0), the node
//! position init (0x34a028) and the segment matrix rebuild (0x34a0b0).
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc661`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
];

/// `cFlexRailNode::cFlexRailNode` (0x347d90): the base constructor.
const FLEX_RAIL_CTOR: Callee = Callee {
    addr: 0x0034_7d90,
    f: ee::s11::f_00347d90,
};

/// `cFlexRailNode::cFlexRailNode` second form (0x347ea8).
const SPRING_FLEX_RAIL_CTOR_2: Callee = Callee {
    addr: 0x0034_7ea8,
    f: ee::s11::f_00347ea8,
};

/// `cFlexRailNode_addSpaceHash` (0x348b90).
const FLEX_RAIL_REGISTER_SPACE: Callee = Callee {
    addr: 0x0034_8b90,
    f: ee::s11::f_00348b90,
};

/// `SpringRail_v50`: the bound box of the rail (0x349f18).
const SPRING_RAIL_BOUND_BOX: Callee = Callee {
    addr: 0x0034_9f18,
    f: ee::s11::f_00349f18,
};

/// `cSpringRailNode_getPosition` (0x34a028): this port.
const SPRING_RAIL_NODE_POS: Callee = Callee {
    addr: 0x0034_a028,
    f: ee::s11::f_0034a028,
};

/// `cSpringRailNode_updateMats` (0x34a0b0): this port.
const SPRING_RAIL_MATS: Callee = Callee {
    addr: 0x0034_a0b0,
    f: ee::s11::f_0034a0b0,
};

/// `World_GetInstance` (0x2d1bd8): the instance of the world, through the table.
const SPRING_WORLD_GET_INSTANCE: u32 = 0x002d_1bd8;

/// `Instance_GetNodeMatrix` (0x34fed8): the bone matrix of a mesh node.
const INSTANCE_NODE_MATRIX: Callee = Callee {
    addr: 0x0034_fed8,
    f: ee::s11::f_0034fed8,
};

/// The `operator new` the spring rail nodes are allocated with (0x317e30).
const NODE_ALLOC: u32 = 0x0031_7e30;

/// The VU0 matrix the segment transform multiplies by: `0x500000 - 0x540`
/// (the generated body's T7 and A3, both computed from the same constant).
const MATS_V48_XFORM: u32 = 0x004f_fac0;

/// `cSpringRailNode_cSpringRailNode(self, instRecord, desc)` (0x349b48),
/// `SpringRail_ctor`: builds the rail on the FlexRail base (type 21) and copies
/// the spring constants from the descriptor (+0x48..+0x50 into +0x60..+0x68).
/// It packs the descriptor's instance ids that are not -1 (and their meshes)
/// into a 0x40-byte scratch in the frame, allocates NumNodes 0x60-byte nodes
/// (Nodes at +0x70), clears the node fields, places each node from its
/// instance (`SpringRail_InitPosFromInstance`), sets the segment vectors,
/// then builds the matrices, the bound box and the space-hash registration.
/// Returns self in v0. Frame 0xc0: S0..S4 at +0xb0..+0x70, RA at +0x60.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn spring_rail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = 0x15;
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save_ra(c, 0x60);
        c.g[A3][0] = c.g[S0][0];
        FLEX_RAIL_CTOR.call(c, 0x0034_9b7c)?;

        // The vtable word of cSpringRailNode, and the default spring constants.
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);

        // Eight words of -1 at the frame's scratch (the id slots).
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = 0x7;
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x48));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x68));
        'l_349bb8: loop {
            c.cycles += 7;
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !t {
                break 'l_349bb8;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_9bb8);
            }
        }

        // Pack the instance ids that are not -1 (and their meshes).
        c.cycles += 7;
        c.g[A3][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[A3][0] = c.g[A3][0] | 0xffff;
        c.g[T0][0] = 0x7;
        c.g[A0][0] = c.g[SP][0];
        'l_349bf0: loop {
            'b_349c0c: {
                c.cycles += 3;
                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                let t = c.g[V1][0] == c.g[A3][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_349c0c;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            }
            c.cycles += 3;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
            let t = (c.g[T0][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            if !t {
                break 'l_349bf0;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_9bf0);
            }
        }

        // Allocate NumNodes nodes and clear their first word to -1.
        'b_349c74: {
            c.cycles += 12;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x60;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0x2000_0000;
            c.mult(0, 0, 16, 4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0u64;
            call_ptr(c, NODE_ALLOC, 0x0034_9c3c)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[V0][0];
            if t {
                break 'b_349c74;
            }
            c.cycles += 4;
            c.g[A1][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            'l_349c58: loop {
                c.cycles += 7;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                let t = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                if !t {
                    break 'b_349c74;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9c58);
                }
            }
        }

        // Nodes = the allocation; then place each node from its instance.
        'b_349d24: {
            c.cycles += 4;
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S4][0] = 0u64;
            if t {
                break 'b_349d24;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x500000;
            c.g[S2][0] = c.g[SP][0];
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            c.g[S0][0] = 0u64;
            'l_349c98: loop {
                c.cycles += 34;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                // The node's position, weight and tangent quads start as zero.
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.lq(3, c.g[S3][0] as u32);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.lq(2, c.g[S3][0] as u32);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.lq(2, c.g[S3][0] as u32);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.lq(2, c.g[S3][0] as u32);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                SPRING_RAIL_NODE_POS.call(c, 0x0034_9d10)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                if !t {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    break 'b_349d24;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9c98);
                }
            }
        }

        // The segment vectors: Nodes[k].V = Nodes[k+1].P - Nodes[k].P.
        'b_349d74: {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_349d74;
            }
            c.cycles += 2;
            c.g[A2][0] = 0u64;
            'l_349d38: loop {
                c.cycles += 15;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x60));
                c.lqc2(4, c.g[V0][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let t = c.g[A0][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                if !t {
                    break 'b_349d74;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9d38);
                }
            }
        }

        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        SPRING_RAIL_MATS.call(c, 0x0034_9d7c)?;
        c.g[A0][0] = c.g[S1][0];
        SPRING_RAIL_BOUND_BOX.call(c, 0x0034_9d84)?;
        c.g[A0][0] = c.g[S1][0];
        FLEX_RAIL_REGISTER_SPACE.call(c, 0x0034_9d8c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    });
}

/// `cSpringRailNode_cSpringRailNode1(self, bucket, stream)` (0x349db0),
/// `SpringRail_ctor_2`: the FlexRail base's second form, the vtable, the
/// spring constants read from the stream (12 bytes, +0x60), the node array
/// (NumNodes * 0x60) allocated and read from the stream, then the matrices
/// and the space-hash registration. Returns self in v0.
/// Frame 0x50: S0..S3 at +0x40..+0x10, RA at +0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn spring_rail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0x0);
        c.g[S2][0] = c.g[A2][0];
        SPRING_FLEX_RAIL_CTOR_2.call(c, 0x0034_9dd4)?;

        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x270));
        c.g[A2][0] = 0xc;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        // The stream's read method (a virtual call on the stream), 12 bytes
        // of spring constants.
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_9e00)?;

        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = 0x60;
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x2000_0000;
        c.mult(0, 0, 16, 4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0u64;
        call_ptr(c, NODE_ALLOC, 0x0034_9e24)?;

        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        let t = c.g[S0][0] == 0u64;
        c.g[V1][0] = c.g[V0][0];
        'b_349e5c: {
            if t {
                break 'b_349e5c;
            }
            c.cycles += 4;
            c.g[A1][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            'l_349e40: loop {
                c.cycles += 7;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                let t = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                if !t {
                    break 'b_349e5c;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9e40);
                }
            }
        }

        c.cycles += 23;
        c.sw(c.g[S3][0] as u32, c.g[V0][0]);
        c.g[V0][0] = 0x60;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.mult(0, 0, 6, 2);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.lo[0];
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_9e88)?;

        c.g[A0][0] = c.g[S1][0];
        SPRING_RAIL_MATS.call(c, 0x0034_9e90)?;
        c.g[A0][0] = c.g[S1][0];
        FLEX_RAIL_REGISTER_SPACE.call(c, 0x0034_9e98)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    });
}

/// `void cSpringRailNode_getPosition(self, node)` (0x34a028),
/// `SpringRail_InitPosFromInstance`: looks the node's instance up (the world's
/// instance table, by the low byte of the instance id, then by the high bits)
/// and copies the translation of the node's bone matrix (mesh `node+0x54`)
/// into the node's position `P` (+0x00). Frame 0x70: S0 at +0x60, RA at +0x50,
/// the bone matrix at +0x30.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn spring_rail_node_pos_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[RA][0] = sx(0x0034_a03c);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SPRING_WORLD_GET_INSTANCE, 0x0034_a03c)?;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        let t = c.g[V1][0] == 0u64;
        c.g[A0][0] = 0u64;
        'b_34a084: {
            if t {
                break 'b_34a084;
            }
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34a084;
            }
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        }
        c.cycles += 10;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[A2][0] = c.g[SP][0];
        INSTANCE_NODE_MATRIX.call(c, 0x0034_a090)?;
        c.lq(2, frame.at(c, 0x30));
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.sq(2, c.g[S0][0] as u32);
        c.lq(16, frame.at(c, 0x60));
        c.sq(2, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    });
}

/// One segment transform of the matrix rebuild: VF8 from `vec`, the matrix at
/// [`MATS_V48_XFORM`] in VF4..VF7, the result quad stored to `dst`.
fn spring_seg_xform_v48(c: &mut Cpu, vec: u32, dst: u32) {
    c.lqc2(8, vec);
    c.lqc2(4, MATS_V48_XFORM);
    c.lqc2(5, MATS_V48_XFORM + 0x10);
    c.lqc2(6, MATS_V48_XFORM + 0x20);
    c.lqc2(7, MATS_V48_XFORM + 0x30);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst);
}

/// `void cSpringRailNode_updateMats(cSpringRailNode* self)` (0x34a0b0),
/// `SpringRail_v48`: rebuilds the per-segment matrices (Mats, 0x40 bytes each,
/// at self+0x50) from the node array (Nodes, 0x60 bytes each, at self+0x70).
/// The first segment comes from nodes 0..2, the middle ones (while
/// `k < NumNodes - 2`) from k-1..k+2, and the last one from the final node.
/// Each segment is a VU0 transform of the node offset vectors; the
/// intermediate quads are spilled to the 0x50-byte frame in the generated
/// order, because those bytes are observable. Leaf: no calls.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn spring_rail_mats_v48(c: &mut Cpu) {
    c.cycles += 99;
    c.g[T5][0] = c.g[A0][0];
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    let st = c.g[SP][0] as u32;
    c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
    c.g[A1][0] = 0x500000;
    c.g[A0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
    c.g[T7][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x540));
    c.lwc1(2, c.g[V1][0] as u32);
    c.g[T6][0] = 0x1;
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc0));
    c.swc1(2, st.wrapping_add(0x14));
    c.swc1(0, st.wrapping_add(0x18));
    c.swc1(1, st.wrapping_add(0x1c));
    c.swc1(2, st.wrapping_add(0x10));
    spring_seg_xform_v48(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lq(2, st.wrapping_add(0x20));
    c.sq(2, st);
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc4));
    c.swc1(2, st.wrapping_add(0x24));
    c.swc1(0, st.wrapping_add(0x28));
    c.swc1(1, st.wrapping_add(0x2c));
    c.swc1(2, st.wrapping_add(0x20));
    spring_seg_xform_v48(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, st.wrapping_add(0x10));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc8));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.swc1(1, st.wrapping_add(0x34));
    c.swc1(0, st.wrapping_add(0x38));
    c.swc1(2, st.wrapping_add(0x3c));
    c.swc1(1, st.wrapping_add(0x30));
    spring_seg_xform_v48(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
    c.lq(2, st.wrapping_add(0x40));
    c.lq(3, st);
    c.sq(2, st.wrapping_add(0x20));
    c.sq(3, c.g[A0][0] as u32);
    c.lq(2, st.wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, st.wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sw(st.wrapping_add(0x30), 0u64);
    c.sw(st.wrapping_add(0x34), 0u64);
    c.sw(st.wrapping_add(0x38), 0u64);
    c.swc1(3, st.wrapping_add(0x3c));
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lq(12, c.g[A0][0] as u32);
    c.lq(11, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(10, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lq(9, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.pextlw(8, 11, 12);
    c.pextuw(7, 11, 12);
    c.pextlw(6, 9, 10);
    c.pextuw(3, 9, 10);
    c.pcpyld(12, 6, 8);
    c.pcpyud(11, 8, 6);
    c.pcpyld(10, 3, 7);
    c.pcpyud(9, 7, 3);
    c.sq(12, c.g[A0][0] as u32);
    c.sq(11, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x30));

    // The middle segments, while k < NumNodes - 2.
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
    c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[T8][0] = c.g[A1][0];
    'b_34a3dc: {
        if t {
            break 'b_34a3dc;
        }
        c.cycles += 3;
        c.mov_s(4, 3);
        c.g[T4][0] = 0x60;
        'l_34a248: loop {
            c.cycles += 101;
            c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
            c.g[T3][0] = sx((c.g[T6][0] as u32) << 6);
            c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V1][0] = c.g[A0][0];
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x60));
            c.lwc1(3, c.g[V1][0] as u32);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc0));
            c.lwc1(0, c.g[A0][0] as u32);
            c.swc1(3, st.wrapping_add(0x14));
            c.swc1(0, st.wrapping_add(0x10));
            c.swc1(1, st.wrapping_add(0x18));
            c.swc1(2, st.wrapping_add(0x1c));
            spring_seg_xform_v48(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
            c.lq(2, st.wrapping_add(0x20));
            c.sq(2, st);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x64));
            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.swc1(1, st.wrapping_add(0x24));
            c.swc1(0, st.wrapping_add(0x20));
            c.swc1(2, st.wrapping_add(0x28));
            c.swc1(3, st.wrapping_add(0x2c));
            spring_seg_xform_v48(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
            c.lq(2, st.wrapping_add(0x30));
            c.sq(2, st.wrapping_add(0x10));
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc8));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.swc1(1, st.wrapping_add(0x30));
            c.swc1(0, st.wrapping_add(0x34));
            c.swc1(2, st.wrapping_add(0x38));
            c.swc1(3, st.wrapping_add(0x3c));
            spring_seg_xform_v48(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
            c.lq(2, st.wrapping_add(0x40));
            c.lq(3, st);
            c.sq(2, st.wrapping_add(0x20));
            c.sq(3, c.g[T3][0] as u32);
            c.lq(2, st.wrapping_add(0x10));
            c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.lq(3, st.wrapping_add(0x20));
            c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.sw(st.wrapping_add(0x30), 0u64);
            c.sw(st.wrapping_add(0x34), 0u64);
            c.sw(st.wrapping_add(0x38), 0u64);
            c.swc1(4, st.wrapping_add(0x3c));
            c.lq(2, st.wrapping_add(0x30));
            c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.lq(10, c.g[T3][0] as u32);
            c.lq(9, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.lq(8, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.lq(7, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.pextlw(6, 9, 10);
            c.pextuw(5, 9, 10);
            c.pextlw(4, 7, 8);
            c.pextuw(3, 7, 8);
            c.pcpyld(10, 4, 6);
            c.pcpyud(9, 6, 4);
            c.pcpyld(8, 3, 5);
            c.pcpyud(7, 5, 3);
            c.sq(10, c.g[T3][0] as u32);
            c.sq(9, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.sq(8, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.sq(7, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x20));
            c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x60));
            if !t {
                break 'b_34a3dc;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_a248);
            }
        }
    }

    // The last segment: from node k-1 and the final node k = NumNodes - 1.
    c.cycles += 99;
    c.g[V1][0] = 0x60;
    c.g[A2][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 14, 3);
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
    c.g[A3][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x540));
    c.g[A0][0] = sx((c.g[T6][0] as u32) << 6);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = c.g[A1][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
    c.lwc1(2, c.g[V1][0] as u32);
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.lwc1(0, c.g[A1][0] as u32);
    c.swc1(2, st.wrapping_add(0x14));
    c.swc1(0, st.wrapping_add(0x10));
    c.swc1(1, st.wrapping_add(0x1c));
    c.swc1(1, st.wrapping_add(0x18));
    spring_seg_xform_v48(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.swc1(2, st.wrapping_add(0x24));
    c.swc1(1, st.wrapping_add(0x20));
    c.swc1(0, st.wrapping_add(0x2c));
    c.swc1(0, st.wrapping_add(0x28));
    spring_seg_xform_v48(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.swc1(0, st.wrapping_add(0x34));
    c.swc1(1, st.wrapping_add(0x30));
    c.swc1(2, st.wrapping_add(0x3c));
    c.swc1(2, st.wrapping_add(0x38));
    spring_seg_xform_v48(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
    c.lq(2, st.wrapping_add(0x40));
    c.lq(3, st);
    c.sq(2, st.wrapping_add(0x20));
    c.sq(3, c.g[A0][0] as u32);
    c.lq(2, st.wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, st.wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(3, st.wrapping_add(0x3c));
    c.sw(st.wrapping_add(0x30), 0u64);
    c.sw(st.wrapping_add(0x34), 0u64);
    c.sw(st.wrapping_add(0x38), 0u64);
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lq(11, c.g[A0][0] as u32);
    c.lq(10, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(9, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lq(8, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.pextlw(7, 10, 11);
    c.pextuw(6, 10, 11);
    c.pextlw(5, 8, 9);
    c.pextuw(3, 8, 9);
    c.pcpyld(11, 5, 7);
    c.pcpyud(10, 7, 5);
    c.pcpyld(9, 3, 6);
    c.pcpyud(8, 6, 3);
    c.sq(11, c.g[A0][0] as u32);
    c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sq(8, (c.g[A0][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use crate::game::movie::movieplayer::tests::alloc_base;

    /// The descriptor of a spring rail: `nodes` non-(-1) instance ids (the
    /// FlexRail base counts them as NumNodes), their meshes, the spring
    /// constants and the flags.
    fn rail_desc(c: &mut Cpu, r: &mut Rng, desc: u32, nodes: u32) {
        for i in 0..8u32 {
            let id = if i < nodes {
                0x100 + (r.next_u32() & 0xffff)
            } else {
                0xffff_ffff
            };
            c.bus.write32(desc + 4 + 4 * i, id);
            c.bus.write32(desc + 0x24 + 4 * i, r.next_u32() % 16);
        }
        c.bus.write32(desc + 0x44, r.next_u32());
        c.bus.write32(desc + 0x48, r.edge_f32());
        c.bus.write32(desc + 0x4c, r.edge_f32());
        c.bus.write32(desc + 0x50, r.edge_f32());
    }

    /// A stream whose read method (vtable +0x14) is a real leaf that returns.
    fn rail_stream(c: &mut Cpu, stream: u32, vt: u32) {
        c.bus.write32(stream, vt);
        c.bus.write16(vt + 0x10, 0);
        c.bus.write32(vt + 0x14, 0x0014_06c0);
    }








}
