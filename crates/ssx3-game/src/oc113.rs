//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! `Cell_CollideVolumeRecursive` is asm-only in the reference decompilation
//! (`src/intersect/worldsphtree.cpp`), so its replacement is a block-for-block
//! translation of the generated body: the same FPU macro ops in the same
//! order, the same loads/stores, registers and cycle charges, and the same
//! inlined `Cell_CollideVolumeSubtree` body for each of the eight octants
//! (the compiler unrolled the recursion; the block structure is kept).
//! Calls go through the generated aliases ([`Callee::call`]) and the loops
//! deliver events at their back edges ([`crate::events`]).

use crate::abi;
use crate::call::{guest, Callee};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0033_ccf8,
        cell_collide_volume_recursive,
        "Cell_CollideVolumeRecursive",
    ),
    (
        0x0033_b748,
        cell_gather_all_recursive,
        "Cell_GatherAllRecursive",
    ),
];

/// The functions the original calls (by address, as the generated body does).
mod callee {
    use super::{ee, Callee};

    /// `Cell_CollideInstance(ctx, instance)` (0x00335960).
    pub const COLLIDE_INSTANCE: Callee = Callee {
        addr: 0x0033_5960,
        f: ee::s11::Cell_CollideInstance,
    };
    /// `Cell_CollideObject(ctx, object)` (0x00335b90).
    pub const COLLIDE_OBJECT: Callee = Callee {
        addr: 0x0033_5b90,
        f: ee::s11::Cell_CollideObject,
    };
    /// `Cell_CollideVolumeSubtree(tree, ctx)` (0x00340fa0): collects a whole
    /// subtree that lies inside the volume.
    pub const COLLIDE_VOLUME_SUBTREE: Callee = Callee {
        addr: 0x0034_0fa0,
        f: ee::s11::Cell_CollideVolumeSubtree,
    };
    /// This function itself.
    pub const COLLIDE_VOLUME_RECURSIVE: Callee = Callee {
        addr: 0x0033_ccf8,
        f: ee::s11::Cell_CollideVolumeRecursive,
    };
}

use self::callee::*;

/// `void Cell_CollideVolumeRecursive(void* unused, CollideCtx* q,
/// CollideCtx* volume_q, const float* box, TreeNode* tree, const Cell* cell)`
/// (0x0033ccf8): collide the objects of one octree node against the volume,
/// then, when the node's cell is not at the deepest level (11), classify the
/// eight child cells against the query box and either collect a child's whole
/// subtree into the volume query (`volume_q`) when the child is fully inside,
/// or recurse into it when it is only partly inside. Children outside the box
/// are skipped.
///
/// A tree node is `{ TreeNode* child[8]; void** nodes; void** objects; }`
/// (`nodes` at +0x20, whose entries go to `Cell_CollideObject`, and `objects`
/// at +0x24, whose entries go to `Cell_CollideInstance`), the cell is
/// `{ int level, x, y, z; }` and the box is `{ float min[4]; float max[4]; }`.
/// The child cell is built in this frame at `sp+0` (`level-1`, `2x`, `2y`,
/// `2z`) and adjusted one octant at a time.
pub fn cell_collide_volume_recursive(c: &mut Cpu) {
    guest(c, |c| {
        'b_33cd5c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S1][0] = c.g[T1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[A3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S4][0] = c.g[A1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S5][0] = c.g[T0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
            let t = c.g[S0][0] == 0u64;
            c.g[S6][0] = c.g[A0][0];
            if t {
                break 'b_33cd5c;
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
                'l_33cd48: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    COLLIDE_INSTANCE.call(c, 0x0033cd50)?;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32));
                    let t = c.g[S0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0033cd48);
                        }
                        continue 'l_33cd48;
                    } else {
                        break 'b_33cd5c;
                    }
                }
            }
        }
        'b_33cd88: {
            c.cycles += 3;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
            if c.g[S0][0] == 0u64 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                break 'b_33cd88;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'l_33cd70: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    COLLIDE_OBJECT.call(c, 0x0033cd78)?;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32));
                    let t = c.g[S0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0033cd70);
                        }
                        continue 'l_33cd70;
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                        break 'b_33cd88;
                    }
                }
            }
        }
        // Classify the eight child cells against the query box; then visit
        // each child in z-order: collect it (`Cell_CollideVolumeSubtree`),
        // recurse into it, or skip it when it misses the box.
        'b_33dbbc: {
            'b_33dbb8: {
                c.cycles += 3;
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_33dbb8;
                } else {
                    'b_33ce18: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cf8));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, (c.g[S3][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                        c.g[FP][0] = 0u64;
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_33ce18;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            if c.fcc() {
                                c.g[FP][0] = 0x1;
                                break 'b_33ce18;
                            } else {
                                break 'b_33ce18;
                            }
                        }
                    }
                    'b_33ce78: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cf4));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32));
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_33ce78;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33ce78;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                break 'b_33ce78;
                            }
                        }
                    }
                    'b_33ce8c: {
                        c.cycles += 2;
                        let t = c.g[FP][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_33ce8c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33ce8c;
                            }
                        }
                    }
                    'b_33cee8: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cf0));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_33cee8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cee8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_33cee8;
                            }
                        }
                    }
                    'b_33cf3c: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cec));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_33cf3c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cf3c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_33cf3c;
                            }
                        }
                    }
                    'b_33cf54: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33cf54;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33cf54;
                            }
                        }
                    }
                    'b_33cfb0: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2ce8));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_33cfb0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cfb0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_33cfb0;
                            }
                        }
                    }
                    'b_33d004: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2ce4));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_33d004;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d004;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_33d004;
                            }
                        }
                    }
                    'b_33d018: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_33d018;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33d018;
                            }
                        }
                    }
                    'b_33d03c: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.g[S7][0] = 0u64;
                        if t {
                            break 'b_33d03c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            if c.fcc() {
                                c.g[S7][0] = 0x1;
                                break 'b_33d03c;
                            } else {
                                break 'b_33d03c;
                            }
                        }
                    }
                    'b_33d064: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        if t {
                            break 'b_33d064;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d064;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_33d064;
                            }
                        }
                    }
                    'b_33d08c: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        if t {
                            break 'b_33d08c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d08c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_33d08c;
                            }
                        }
                    }
                    'b_33d0b4: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_33d0b4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d0b4;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_33d0b4;
                            }
                        }
                    }
                    'b_33d0dc: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_33d0dc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d0dc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_33d0dc;
                            }
                        }
                    }
                    'b_33d104: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                        if t {
                            break 'b_33d104;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d104;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                break 'b_33d104;
                            }
                        }
                    }
                    'b_33d254: {
                        c.cycles += 3;
                        c.g[S1][0] = c.lw((c.g[S5][0] as u32));
                        let t = c.g[S1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33d254;
                        } else {
                            'b_33d250: {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33d250;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d250;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33d254;
                                        } else {
                                            'b_33d238: {
                                                'b_33d234: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33d234;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d234;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d238;
                                                            } else {
                                                                'b_33d168: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d168;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d150: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d158,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d150);
                                                                                }
                                                                                continue 'l_33d150;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d168;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d190: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d190;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d178: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d180,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d178);
                                                                                }
                                                                                continue 'l_33d178;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d190;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d1a4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d1a4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1a0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d1a4;
                                                                    }
                                                                }
                                                                'b_33d1b8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d1b8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1b4)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d1b8;
                                                                    }
                                                                }
                                                                'b_33d1cc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d1cc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1c8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d1cc;
                                                                    }
                                                                }
                                                                'b_33d1e0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d1e0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1dc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d1e0;
                                                                    }
                                                                }
                                                                'b_33d1f4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d1f4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1f0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d1f4;
                                                                    }
                                                                }
                                                                'b_33d208: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d208;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d204)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d208;
                                                                    }
                                                                }
                                                                'b_33d21c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d21c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d218)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d21c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d254;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d22c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d254;
                                                                    } else {
                                                                        break 'b_33d234;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d238;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d250)?;
                                            break 'b_33d250;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            break 'b_33d254;
                        }
                    }
                    'b_33d3ac: {
                        'b_33d3a8: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d3a8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33d3a8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d3a8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_33d3ac;
                                        } else {
                                            'b_33d390: {
                                                'b_33d38c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33d38c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d38c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d390;
                                                            } else {
                                                                'b_33d2c0: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d2c0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d2a8: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d2b0,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d2a8);
                                                                                }
                                                                                continue 'l_33d2a8;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d2c0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d2e8: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d2e8;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d2d0: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d2d8,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d2d0);
                                                                                }
                                                                                continue 'l_33d2d0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d2e8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d2fc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d2fc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d2f8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d2fc;
                                                                    }
                                                                }
                                                                'b_33d310: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d310;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d30c)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d310;
                                                                    }
                                                                }
                                                                'b_33d324: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d324;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d320)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d324;
                                                                    }
                                                                }
                                                                'b_33d338: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d338;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d334)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d338;
                                                                    }
                                                                }
                                                                'b_33d34c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d34c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d348)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d34c;
                                                                    }
                                                                }
                                                                'b_33d360: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d360;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d35c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d360;
                                                                    }
                                                                }
                                                                'b_33d374: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d374;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d370)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d374;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33d3ac;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d384)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33d3ac;
                                                                    } else {
                                                                        break 'b_33d38c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d390;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d3a8)?;
                                            break 'b_33d3a8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33d3ac;
                    }
                    'b_33d504: {
                        'b_33d500: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33d500;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d500;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d500;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33d504;
                                        } else {
                                            'b_33d4e8: {
                                                'b_33d4e4: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d4e4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d4e4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d4e8;
                                                            } else {
                                                                'b_33d418: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d418;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d400: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d408,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d400);
                                                                                }
                                                                                continue 'l_33d400;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d418;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d440: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d440;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d428: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d430,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d428);
                                                                                }
                                                                                continue 'l_33d428;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d440;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d454: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d454;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d450)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d454;
                                                                    }
                                                                }
                                                                'b_33d468: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d468;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d464)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d468;
                                                                    }
                                                                }
                                                                'b_33d47c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d47c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d478)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d47c;
                                                                    }
                                                                }
                                                                'b_33d490: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d490;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d48c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d490;
                                                                    }
                                                                }
                                                                'b_33d4a4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d4a4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4a0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d4a4;
                                                                    }
                                                                }
                                                                'b_33d4b8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d4b8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4b4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d4b8;
                                                                    }
                                                                }
                                                                'b_33d4cc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d4cc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4c8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d4cc;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d504;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d4dc)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d504;
                                                                    } else {
                                                                        break 'b_33d4e4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d4e8;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d500)?;
                                            break 'b_33d500;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33d504;
                    }
                    'b_33d65c: {
                        'b_33d658: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d658;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d658;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d658;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_33d65c;
                                        } else {
                                            'b_33d640: {
                                                'b_33d63c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d63c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d63c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d640;
                                                            } else {
                                                                'b_33d570: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d570;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d558: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d560,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d558);
                                                                                }
                                                                                continue 'l_33d558;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d570;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d598: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d598;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d580: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d588,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d580);
                                                                                }
                                                                                continue 'l_33d580;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d598;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d5ac: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d5ac;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5a8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d5ac;
                                                                    }
                                                                }
                                                                'b_33d5c0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d5c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5bc)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d5c0;
                                                                    }
                                                                }
                                                                'b_33d5d4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d5d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5d0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d5d4;
                                                                    }
                                                                }
                                                                'b_33d5e8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d5e8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5e4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d5e8;
                                                                    }
                                                                }
                                                                'b_33d5fc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d5fc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5f8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d5fc;
                                                                    }
                                                                }
                                                                'b_33d610: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d610;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d60c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d610;
                                                                    }
                                                                }
                                                                'b_33d624: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d624;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d620)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d624;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_33d65c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d634)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    if t {
                                                                        break 'b_33d65c;
                                                                    } else {
                                                                        break 'b_33d63c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d640;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d658)?;
                                            break 'b_33d658;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_33d65c;
                    }
                    'b_33d7b4: {
                        'b_33d7b0: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33d7b0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d7b0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d7b0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33d7b0;
                                        } else {
                                            'b_33d798: {
                                                'b_33d794: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d794;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d794;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d798;
                                                            } else {
                                                                'b_33d6c8: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d6c8;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d6b0: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d6b8,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d6b0);
                                                                                }
                                                                                continue 'l_33d6b0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d6c8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d6f0: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d6f0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d6d8: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d6e0,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d6d8);
                                                                                }
                                                                                continue 'l_33d6d8;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d6f0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d704: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d704;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d700)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d704;
                                                                    }
                                                                }
                                                                'b_33d718: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d718;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d714)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d718;
                                                                    }
                                                                }
                                                                'b_33d72c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d72c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d728)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d72c;
                                                                    }
                                                                }
                                                                'b_33d740: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d740;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d73c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d740;
                                                                    }
                                                                }
                                                                'b_33d754: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d754;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d750)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d754;
                                                                    }
                                                                }
                                                                'b_33d768: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d768;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d764)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d768;
                                                                    }
                                                                }
                                                                'b_33d77c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d77c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d778)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d77c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d7b4;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d78c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d7b4;
                                                                    } else {
                                                                        break 'b_33d794;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d798;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d7b0)?;
                                            break 'b_33d7b0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33d7b4;
                    }
                    'b_33d90c: {
                        'b_33d908: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d908;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d908;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d908;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33d908;
                                        } else {
                                            'b_33d8f0: {
                                                'b_33d8ec: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d8ec;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d8ec;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d8f0;
                                                            } else {
                                                                'b_33d820: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d820;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d808: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d810,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d808);
                                                                                }
                                                                                continue 'l_33d808;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d820;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d848: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d848;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d830: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d838,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d830);
                                                                                }
                                                                                continue 'l_33d830;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d848;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d85c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d85c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d858)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d85c;
                                                                    }
                                                                }
                                                                'b_33d870: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d870;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d86c)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d870;
                                                                    }
                                                                }
                                                                'b_33d884: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d884;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d880)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d884;
                                                                    }
                                                                }
                                                                'b_33d898: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d898;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d894)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d898;
                                                                    }
                                                                }
                                                                'b_33d8ac: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d8ac;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8a8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d8ac;
                                                                    }
                                                                }
                                                                'b_33d8c0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d8c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8bc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d8c0;
                                                                    }
                                                                }
                                                                'b_33d8d4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d8d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8d0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d8d4;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33d90c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d8e4)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33d90c;
                                                                    } else {
                                                                        break 'b_33d8ec;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d8f0;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d908)?;
                                            break 'b_33d908;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33d90c;
                    }
                    'b_33da64: {
                        'b_33da60: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33da60;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33da60;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33da60;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33da60;
                                        } else {
                                            'b_33da48: {
                                                'b_33da44: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33da44;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33da44;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33da48;
                                                            } else {
                                                                'b_33d978: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d978;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d960: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d968,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d960);
                                                                                }
                                                                                continue 'l_33d960;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d978;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d9a0: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d9a0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d988: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d990,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d988);
                                                                                }
                                                                                continue 'l_33d988;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d9a0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d9b4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d9b4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9b0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d9b4;
                                                                    }
                                                                }
                                                                'b_33d9c8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d9c8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9c4)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d9c8;
                                                                    }
                                                                }
                                                                'b_33d9dc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d9dc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9d8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d9dc;
                                                                    }
                                                                }
                                                                'b_33d9f0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d9f0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9ec)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d9f0;
                                                                    }
                                                                }
                                                                'b_33da04: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33da04;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da00)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33da04;
                                                                    }
                                                                }
                                                                'b_33da18: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33da18;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da14)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33da18;
                                                                    }
                                                                }
                                                                'b_33da2c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33da2c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da28)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33da2c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33da64;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033da3c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33da64;
                                                                    } else {
                                                                        break 'b_33da44;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33da48;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033da60)?;
                                            break 'b_33da60;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33da64;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_33dbb8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_33dbb8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                            if t {
                                break 'b_33dbb8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                if t {
                                    break 'b_33dbb8;
                                } else {
                                    'b_33dba0: {
                                        'b_33db9c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                            if t {
                                                break 'b_33db9c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_33db9c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S6][0];
                                                    if t {
                                                        break 'b_33dba0;
                                                    } else {
                                                        'b_33dad0: {
                                                            c.cycles += 3;
                                                            c.g[S0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            if c.g[S0][0] == 0u64 {
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                break 'b_33dad0;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                'l_33dab8: loop {
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    COLLIDE_INSTANCE
                                                                        .call(c, 0x0033dac0)?;
                                                                    c.g[S0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    let t = c.g[S0][0] != 0u64;
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0033dab8,
                                                                            );
                                                                        }
                                                                        continue 'l_33dab8;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33dad0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_33daf8: {
                                                            c.cycles += 2;
                                                            if c.g[S0][0] == 0u64 {
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[S1][0] as u32));
                                                                break 'b_33daf8;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                'l_33dae0: loop {
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    COLLIDE_OBJECT
                                                                        .call(c, 0x0033dae8)?;
                                                                    c.g[S0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    let t = c.g[S0][0] != 0u64;
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0033dae0,
                                                                            );
                                                                        }
                                                                        continue 'l_33dae0;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33daf8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_33db0c: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33db0c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db08)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33db0c;
                                                            }
                                                        }
                                                        'b_33db20: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_33db20;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db1c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_33db20;
                                                            }
                                                        }
                                                        'b_33db34: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_33db34;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db30)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_33db34;
                                                            }
                                                        }
                                                        'b_33db48: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_33db48;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db44)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_33db48;
                                                            }
                                                        }
                                                        'b_33db5c: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_33db5c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db58)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_33db5c;
                                                            }
                                                        }
                                                        'b_33db70: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_33db70;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db6c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_33db70;
                                                            }
                                                        }
                                                        'b_33db84: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_33db84;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db80)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_33db84;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        if t {
                                                            break 'b_33dbbc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A1][0] = c.g[S2][0];
                                                            COLLIDE_VOLUME_SUBTREE
                                                                .call(c, 0x0033db94)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xd0),
                                                            );
                                                            if t {
                                                                break 'b_33dbbc;
                                                            } else {
                                                                break 'b_33db9c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S6][0];
                                        break 'b_33dba0;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[T0][0] = c.g[S1][0];
                                    c.g[T1][0] = c.g[SP][0];
                                    COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033dbb8)?;
                                    break 'b_33dbb8;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            break 'b_33dbbc;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        abi::ret(c);
        Ok(())
    })
}

/// The other gathers this function calls (through their generated aliases).
mod gather_callee {
    use super::{ee, Callee};

    /// `Cell_GatherObjects(ctx, tree)` (0x003309d8): append the node's
    /// objects that overlap `ctx->nodes.box` to `ctx->nodes`.
    pub const GATHER_OBJECTS: Callee = Callee {
        addr: 0x0033_09d8,
        f: ee::s11::Cell_GatherObjects,
    };
    /// `Cell_GatherAllSubtree(tree, ctx)` (0x00340dc0): gather a whole
    /// subtree (objects, all-list and children).
    pub const GATHER_ALL_SUBTREE: Callee = Callee {
        addr: 0x0034_0dc0,
        f: ee::s11::Cell_GatherAllSubtree,
    };
    /// This function itself.
    pub const GATHER_ALL_RECURSIVE: Callee = Callee {
        addr: 0x0033_b748,
        f: ee::s11::Cell_GatherAllRecursive,
    };
}

use self::gather_callee::*;

/// `void Cell_GatherAllRecursive(void* unused, GatherCtx* q,
/// GatherCtx* collect_q, const float* box, TreeNode* tree, const Cell* cell)`
/// (0x0033b748): gather one octree node of the world sphere tree, then (unless
/// the cell is at the deepest level, 11) classify its eight child cells
/// against `box`. A child wholly inside the box has its whole subtree
/// gathered into `collect_q` (`Cell_GatherAllSubtree`, inlined by the
/// compiler for every octant), a partly-inside child is recursed into, and a
/// child outside the box is skipped. `q` gathers the objects of the nodes on
/// the descent path.
///
/// A tree node is `{ TreeNode* child[8]; void** nodes; void** objects;
/// void** all; }` (`nodes` at +0x20, `objects` at +0x24, `all` at +0x28); a
/// gather context is three `{ void** list; int* count; int pad; float* box; }`
/// sub-structs at +0 (`nodes`), +0x10 (`objects`) and +0x20 (`all`).
pub fn cell_gather_all_recursive(c: &mut Cpu) {
    guest(c, |c| {
        'b_33b85c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[T0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S0][0] = c.g[T1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S1][0] = c.g[A2][0];
            c.g[S2][0] = c.g[A3][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.g[S4][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[A1][0] = c.g[S3][0];
            GATHER_OBJECTS.call(c, 0x0033b798)?;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_33b85c;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                'l_33b7a8: loop {
                    'b_33b828: {
                        c.cycles += 6;
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.c_le_s(0, 1);
                        let t = !c.fcc();
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_33b828;
                        } else {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                            c.c_le_s(0, 1);
                            let t = !c.fcc();

                            if t {
                                break 'b_33b828;
                            } else {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                c.c_le_s(1, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_33b828;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                    c.c_le_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_33b828;
                                    } else {
                                        c.cycles += 5;
                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                        c.c_le_s(1, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_33b828;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                            c.c_le_s(0, 1);
                                            if c.fcc() {
                                                c.g[V1][0] = 0x1;
                                                break 'b_33b828;
                                            } else {
                                                break 'b_33b828;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    'b_33b854: {
                        c.cycles += 2;
                        if c.g[V1][0] == 0u64 {
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                            break 'b_33b854;
                        } else {
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                            c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                            break 'b_33b854;
                        }
                    }
                    c.cycles += 2;
                    if c.g[A2][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x0033b7a8);
                        }
                        continue 'l_33b7a8;
                    } else {
                        break 'b_33b85c;
                    }
                }
            }
        }
        'b_33b898: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_33b898;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                'l_33b870: loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                    c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                    if c.g[A2][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x0033b870);
                        }
                        continue 'l_33b870;
                    } else {
                        break 'b_33b898;
                    }
                }
            }
        }
        // Classify the eight child cells against the query box; then visit
        // each child in z-order: gather it (`Cell_GatherAllSubtree`), recurse
        // into it, or skip it when it misses the box.
        'b_33ccc8: {
            'b_33ccc4: {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_33ccc4;
                } else {
                    'b_33b92c: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d10));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, (c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                        c.g[S7][0] = 0u64;
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_33b92c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            if c.fcc() {
                                c.g[S7][0] = 0x1;
                                break 'b_33b92c;
                            } else {
                                break 'b_33b92c;
                            }
                        }
                    }
                    'b_33b98c: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d0c));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32));
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_33b98c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33b98c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                break 'b_33b98c;
                            }
                        }
                    }
                    'b_33b9a0: {
                        c.cycles += 2;
                        let t = c.g[S7][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_33b9a0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33b9a0;
                            }
                        }
                    }
                    'b_33b9fc: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d08));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_33b9fc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33b9fc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_33b9fc;
                            }
                        }
                    }
                    'b_33ba4c: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d04));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.g[FP][0] = 0u64;
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_33ba4c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            if c.fcc() {
                                c.g[FP][0] = 0x1;
                                break 'b_33ba4c;
                            } else {
                                break 'b_33ba4c;
                            }
                        }
                    }
                    'b_33ba60: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33ba60;
                        } else {
                            c.cycles += 2;
                            let t = c.g[FP][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33ba60;
                            }
                        }
                    }
                    'b_33babc: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d00));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_33babc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33babc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_33babc;
                            }
                        }
                    }
                    'b_33bb10: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cfc));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_33bb10;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb10;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_33bb10;
                            }
                        }
                    }
                    'b_33bb24: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        if t {
                            break 'b_33bb24;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33bb24;
                            }
                        }
                    }
                    'b_33bb48: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.g[S6][0] = 0u64;
                        if t {
                            break 'b_33bb48;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            if c.fcc() {
                                c.g[S6][0] = 0x1;
                                break 'b_33bb48;
                            } else {
                                break 'b_33bb48;
                            }
                        }
                    }
                    'b_33bb70: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        if t {
                            break 'b_33bb70;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb70;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_33bb70;
                            }
                        }
                    }
                    'b_33bb98: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        if t {
                            break 'b_33bb98;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb98;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_33bb98;
                            }
                        }
                    }
                    'b_33bbc0: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        if t {
                            break 'b_33bbc0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bbc0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_33bbc0;
                            }
                        }
                    }
                    'b_33bbe8: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_33bbe8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bbe8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_33bbe8;
                            }
                        }
                    }
                    'b_33bc10: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_33bc10;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bc10;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_33bc10;
                            }
                        }
                    }
                    'b_33be20: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32));
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33be20;
                        } else {
                            'b_33be1c: {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33be1c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33be1c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33be20;
                                        } else {
                                            'b_33be00: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33be00;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33be00;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33be00;
                                                        } else {
                                                            'b_33bd1c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033bc54)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33bd1c;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33bc68: loop {
                                                                        'b_33bce8: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33bce8;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33bce8;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33bce8;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33bce8;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33bce8;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33bce8;
} else {
break 'b_33bce8;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33bd14: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bd14;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bd14;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bc68,
                                                                                );
                                                                            }
                                                                            continue 'l_33bc68;
                                                                        } else {
                                                                            break 'b_33bd1c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bd5c: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33bd5c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33bd30: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bd30,
                                                                                );
                                                                            }
                                                                            continue 'l_33bd30;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33bd5c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bd70: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bd70;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd6c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bd70;
                                                                }
                                                            }
                                                            'b_33bd84: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bd84;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd80)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bd84;
                                                                }
                                                            }
                                                            'b_33bd98: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bd98;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd94)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bd98;
                                                                }
                                                            }
                                                            'b_33bdac: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bdac;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bda8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bdac;
                                                                }
                                                            }
                                                            'b_33bdc0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bdc0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bdbc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bdc0;
                                                                }
                                                            }
                                                            'b_33bdd4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bdd4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bdd0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bdd4;
                                                                }
                                                            }
                                                            'b_33bde8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33bde8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bde4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33bde8;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33be20;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033bdf8)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33be20;
                                                                } else {
                                                                    break 'b_33be00;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033be1c)?;
                                            break 'b_33be1c;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            break 'b_33be20;
                        }
                    }
                    'b_33c038: {
                        'b_33c034: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c034;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33c034;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c034;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_33c038;
                                        } else {
                                            'b_33c018: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33c018;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c018;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c018;
                                                        } else {
                                                            'b_33bf34: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033be68)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33bf34;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33be80: loop {
                                                                        'b_33bf00: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33bf00;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33bf00;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33bf00;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33bf00;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33bf00;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33bf00;
} else {
break 'b_33bf00;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33bf2c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bf2c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bf2c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033be80,
                                                                                );
                                                                            }
                                                                            continue 'l_33be80;
                                                                        } else {
                                                                            break 'b_33bf34;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bf74: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33bf74;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33bf48: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bf48,
                                                                                );
                                                                            }
                                                                            continue 'l_33bf48;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33bf74;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bf88: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bf88;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bf84)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bf88;
                                                                }
                                                            }
                                                            'b_33bf9c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bf9c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bf98)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bf9c;
                                                                }
                                                            }
                                                            'b_33bfb0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bfb0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfac)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bfb0;
                                                                }
                                                            }
                                                            'b_33bfc4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bfc4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfc0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bfc4;
                                                                }
                                                            }
                                                            'b_33bfd8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bfd8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfd4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bfd8;
                                                                }
                                                            }
                                                            'b_33bfec: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bfec;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfe8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bfec;
                                                                }
                                                            }
                                                            'b_33c000: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c000;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bffc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c000;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_33c038;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c010)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33c038;
                                                                } else {
                                                                    break 'b_33c018;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c034)?;
                                            break 'b_33c034;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33c038;
                    }
                    'b_33c250: {
                        'b_33c24c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33c24c;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_33c250;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c24c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33c250;
                                        } else {
                                            'b_33c230: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c230;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c230;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c230;
                                                        } else {
                                                            'b_33c14c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c080)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c14c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c098: loop {
                                                                        'b_33c118: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c118;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c118;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c118;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c118;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c118;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c118;
} else {
break 'b_33c118;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c144: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c144;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c144;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c098,
                                                                                );
                                                                            }
                                                                            continue 'l_33c098;
                                                                        } else {
                                                                            break 'b_33c14c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c18c: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c18c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c160: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c160,
                                                                                );
                                                                            }
                                                                            continue 'l_33c160;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c18c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c1a0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c1a0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c19c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c1a0;
                                                                }
                                                            }
                                                            'b_33c1b4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c1b4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1b0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c1b4;
                                                                }
                                                            }
                                                            'b_33c1c8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c1c8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1c4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c1c8;
                                                                }
                                                            }
                                                            'b_33c1dc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c1dc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1d8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c1dc;
                                                                }
                                                            }
                                                            'b_33c1f0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c1f0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1ec)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c1f0;
                                                                }
                                                            }
                                                            'b_33c204: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c204;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c200)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c204;
                                                                }
                                                            }
                                                            'b_33c218: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c218;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c214)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c218;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33c250;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c228)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33c250;
                                                                } else {
                                                                    break 'b_33c230;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c24c)?;
                                            break 'b_33c24c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33c250;
                    }
                    'b_33c468: {
                        'b_33c464: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c464;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_33c468;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33c464;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_33c468;
                                        } else {
                                            'b_33c448: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c448;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33c448;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c448;
                                                        } else {
                                                            'b_33c364: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c298)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c364;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c2b0: loop {
                                                                        'b_33c330: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c330;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c330;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c330;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c330;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c330;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c330;
} else {
break 'b_33c330;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c35c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c35c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c35c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c2b0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c2b0;
                                                                        } else {
                                                                            break 'b_33c364;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c3a4: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c3a4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c378: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c378,
                                                                                );
                                                                            }
                                                                            continue 'l_33c378;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c3a4;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c3b8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c3b8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3b4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c3b8;
                                                                }
                                                            }
                                                            'b_33c3cc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c3cc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3c8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c3cc;
                                                                }
                                                            }
                                                            'b_33c3e0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c3e0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3dc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c3e0;
                                                                }
                                                            }
                                                            'b_33c3f4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c3f4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3f0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c3f4;
                                                                }
                                                            }
                                                            'b_33c408: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c408;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c404)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c408;
                                                                }
                                                            }
                                                            'b_33c41c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c41c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c418)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c41c;
                                                                }
                                                            }
                                                            'b_33c430: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c430;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c42c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c430;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x4));
                                                            if t {
                                                                break 'b_33c468;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c440)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_33c468;
                                                                } else {
                                                                    break 'b_33c448;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c464)?;
                                            break 'b_33c464;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_33c468;
                    }
                    'b_33c680: {
                        'b_33c67c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33c67c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_33c680;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33c67c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33c67c;
                                        } else {
                                            'b_33c660: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c660;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33c660;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c660;
                                                        } else {
                                                            'b_33c57c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c4b4)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c57c;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c4c8: loop {
                                                                        'b_33c548: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c548;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c548;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c548;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c548;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c548;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c548;
} else {
break 'b_33c548;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c574: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c574;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c574;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c4c8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c4c8;
                                                                        } else {
                                                                            break 'b_33c57c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c5bc: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c5bc;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c590: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c590,
                                                                                );
                                                                            }
                                                                            continue 'l_33c590;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c5bc;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c5d0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c5d0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5cc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c5d0;
                                                                }
                                                            }
                                                            'b_33c5e4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c5e4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5e0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c5e4;
                                                                }
                                                            }
                                                            'b_33c5f8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c5f8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5f4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c5f8;
                                                                }
                                                            }
                                                            'b_33c60c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c60c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c608)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c60c;
                                                                }
                                                            }
                                                            'b_33c620: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c620;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c61c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c620;
                                                                }
                                                            }
                                                            'b_33c634: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c634;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c630)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c634;
                                                                }
                                                            }
                                                            'b_33c648: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c648;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c644)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c648;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33c680;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c658)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33c680;
                                                                } else {
                                                                    break 'b_33c660;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c67c)?;
                                            break 'b_33c67c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33c680;
                    }
                    'b_33c898: {
                        'b_33c894: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c894;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_33c898;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c894;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33c894;
                                        } else {
                                            'b_33c878: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c878;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c878;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c878;
                                                        } else {
                                                            'b_33c794: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c6cc)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c794;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c6e0: loop {
                                                                        'b_33c760: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c760;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c760;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c760;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c760;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c760;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c760;
} else {
break 'b_33c760;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c78c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c78c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c78c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c6e0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c6e0;
                                                                        } else {
                                                                            break 'b_33c794;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c7d4: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c7d4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c7a8: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c7a8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c7a8;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c7d4;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c7e8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c7e8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c7e4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c7e8;
                                                                }
                                                            }
                                                            'b_33c7fc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c7fc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c7f8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c7fc;
                                                                }
                                                            }
                                                            'b_33c810: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c810;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c80c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c810;
                                                                }
                                                            }
                                                            'b_33c824: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c824;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c820)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c824;
                                                                }
                                                            }
                                                            'b_33c838: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c838;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c834)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c838;
                                                                }
                                                            }
                                                            'b_33c84c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c84c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c848)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c84c;
                                                                }
                                                            }
                                                            'b_33c860: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c860;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c85c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c860;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_33c898;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c870)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33c898;
                                                                } else {
                                                                    break 'b_33c878;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c894)?;
                                            break 'b_33c894;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33c898;
                    }
                    'b_33cab0: {
                        'b_33caac: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33caac;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33caac;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33caac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33caac;
                                        } else {
                                            'b_33ca90: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33ca90;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33ca90;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33ca90;
                                                        } else {
                                                            'b_33c9ac: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c8e4)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c9ac;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c8f8: loop {
                                                                        'b_33c978: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c978;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c978;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c978;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c978;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c978;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c978;
} else {
break 'b_33c978;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c9a4: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c9a4;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c9a4;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c8f8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c8f8;
                                                                        } else {
                                                                            break 'b_33c9ac;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c9ec: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c9ec;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c9c0: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c9c0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c9c0;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c9ec;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33ca00: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33ca00;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c9fc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33ca00;
                                                                }
                                                            }
                                                            'b_33ca14: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33ca14;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca10)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33ca14;
                                                                }
                                                            }
                                                            'b_33ca28: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33ca28;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca24)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33ca28;
                                                                }
                                                            }
                                                            'b_33ca3c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33ca3c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca38)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33ca3c;
                                                                }
                                                            }
                                                            'b_33ca50: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33ca50;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca4c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33ca50;
                                                                }
                                                            }
                                                            'b_33ca64: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33ca64;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca60)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33ca64;
                                                                }
                                                            }
                                                            'b_33ca78: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33ca78;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca74)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33ca78;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33cab0;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033ca88)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33cab0;
                                                                } else {
                                                                    break 'b_33ca90;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033caac)?;
                                            break 'b_33caac;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33cab0;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_33ccc4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_33ccc4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_33ccc4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33ccc4;
                                } else {
                                    'b_33cca8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33cca8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                            if t {
                                                break 'b_33cca8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_33cca8;
                                                } else {
                                                    'b_33cbc4: {
                                                        c.cycles += 6;
                                                        c.g[A1][0] = c.g[S0][0];
                                                        GATHER_OBJECTS.call(c, 0x0033cafc)?;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x24));
                                                        c.g[T0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = c.g[A2][0] == 0u64;
                                                        c.g[A3][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            break 'b_33cbc4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            'l_33cb10: loop {
                                                                'b_33cb90: {
                                                                    c.cycles += 6;
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[A2][0] as u32)
                                                                            .wrapping_add(0x164),
                                                                    );
                                                                    c.lwc1(0, (c.g[V0][0] as u32));
                                                                    c.c_le_s(0, 1);
                                                                    let t = !c.fcc();
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_33cb90;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[A2][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x158,
                                                                                ),
                                                                        );
                                                                        c.c_le_s(0, 1);
                                                                        let t = !c.fcc();

                                                                        if t {
                                                                            break 'b_33cb90;
                                                                        } else {
                                                                            c.cycles += 5;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[V0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x168,
                                                                                    ),
                                                                            );
                                                                            c.c_le_s(1, 0);
                                                                            let t = !c.fcc();

                                                                            if t {
                                                                                break 'b_33cb90;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33cb90;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33cb90;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_33cb90;
                                                                                        } else {
                                                                                            break 'b_33cb90;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33cbbc: {
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] == 0u64 {
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        break 'b_33cbbc;
                                                                    } else {
                                                                        c.cycles += 9;
                                                                        c.g[A1][0] = c
                                                                            .lw((c.g[T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[T0][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        break 'b_33cbbc;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[A2][0] != 0u64 {
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0033cb10,
                                                                        );
                                                                    }
                                                                    continue 'l_33cb10;
                                                                } else {
                                                                    break 'b_33cbc4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_33cc04: {
                                                        c.cycles += 3;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        if c.g[A2][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                            break 'b_33cc04;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[A1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                            'l_33cbd8: loop {
                                                                c.cycles += 10;
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[A3][0] as u32));
                                                                c.g[V1][0] =
                                                                    c.lw((c.g[A1][0] as u32));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.sw(
                                                                    (c.g[V0][0] as u32),
                                                                    c.g[A2][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[A1][0] as u32),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[A2][0] =
                                                                    c.lw((c.g[A2][0] as u32));
                                                                if c.g[A2][0] != 0u64 {
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0033cbd8,
                                                                        );
                                                                    }
                                                                    continue 'l_33cbd8;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33cc04;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_33cc18: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_33cc18;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc14)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_33cc18;
                                                        }
                                                    }
                                                    'b_33cc2c: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_33cc2c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc28)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_33cc2c;
                                                        }
                                                    }
                                                    'b_33cc40: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_33cc40;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc3c)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_33cc40;
                                                        }
                                                    }
                                                    'b_33cc54: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_33cc54;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc50)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_33cc54;
                                                        }
                                                    }
                                                    'b_33cc68: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_33cc68;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc64)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_33cc68;
                                                        }
                                                    }
                                                    'b_33cc7c: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33cc7c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc78)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33cc7c;
                                                        }
                                                    }
                                                    'b_33cc90: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_33cc90;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc8c)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_33cc90;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] == 0u64;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                    );
                                                    if t {
                                                        break 'b_33ccc8;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        GATHER_ALL_SUBTREE.call(c, 0x0033cca0)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        if t {
                                                            break 'b_33ccc8;
                                                        } else {
                                                            break 'b_33cca8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[T0][0] = c.g[S0][0];
                                    c.g[T1][0] = c.g[SP][0];
                                    GATHER_ALL_RECURSIVE.call(c, 0x0033ccc4)?;
                                    break 'b_33ccc4;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            break 'b_33ccc8;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    #[test]
    fn cell_collide_volume_recursive_matches() {
        check(
            ee::s11::f_0033ccf8__generated,
            cell_collide_volume_recursive,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x33cc);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // The BELibrary singleton and the world singleton (callees).
                let belib = DATA + 0x3_0000;
                let world = DATA + 0x3_1000;
                c.bus.write32(gp_at(-0x1f04), belib);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x7c, 0);
                // The 0.2f constants the classification multiplies with.
                for k in 0..16 {
                    c.bus.write32(gp_at(-0x2d10 + 4 * k), f(0.2));
                }

                let root = DATA + 0x1000;
                let cell = DATA + 0x2000;
                let pos = DATA + 0x2100;
                // One instance and one object on the root so both list loops run
                // (each callee returns from its first validity test).
                c.bus.write32(root + 0x24, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, 0);
                c.bus.write16(DATA + 0x3000 + 0xa, 0);
                c.bus.write32(root + 0x20, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                c.bus.write32(DATA + 0x3100 + 0x88, 0);
                // All eight children present; the first two also have one leaf
                // grandchild (reached by `Cell_CollideVolumeSubtree`).
                for i in 0..8u32 {
                    let node = DATA + 0x1100 + 0x100 * i;
                    c.bus.write32(root + 4 * i, node);
                    if i < 2 {
                        c.bus.write32(node, DATA + 0x1500 + 0x100 * i);
                    }
                }
                let level = seed % 12;
                let (x, y, z) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(cell, level);
                c.bus.write32(cell + 4, x);
                c.bus.write32(cell + 8, y);
                c.bus.write32(cell + 0xc, z);
                // The query box: containing (collect the subtree), disjoint (skip
                // the child) or straddling the first octant (recurse).
                let s = f32::from_bits((level + 0x7e) << 23);
                for k in 0..3u32 {
                    let coord = [x, y, z][k as usize] as f32;
                    let lo = (2.0 * coord - 0.2) * s;
                    let hi = (2.0 * coord + 1.2) * s;
                    let (min, max) = match seed % 3 {
                        0 => (-1.0e6f32, 1.0e6f32),
                        1 => (1.0e6f32, 1.0e6f32 + 1.0),
                        _ => (0.5 * (lo + hi), 1.0e6f32),
                    };
                    c.bus.write32(pos + 4 * k, f(min));
                    c.bus.write32(pos + 0x10 + 4 * k, f(max));
                }
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x3200);
                c.g[A2][0] = sx(DATA + 0x3300);
                c.g[A3][0] = sx(pos);
                c.g[T0][0] = sx(root);
                c.g[T1][0] = sx(cell);
            },
        );
    }

    #[test]
    fn cell_gather_all_recursive_matches() {
        check(
            ee::s11::f_0033b748__generated,
            cell_gather_all_recursive,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0xb748);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let belib = DATA + 0x3_0000;
                let world = DATA + 0x3_1000;
                c.bus.write32(gp_at(-0x1f04), belib);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x7c, 0);
                for k in 0..16 {
                    c.bus.write32(gp_at(-0x2d10 + 4 * k), f(0.2));
                }

                let root = DATA + 0x1000;
                let cell = DATA + 0x2000;
                let box_at = DATA + 0x2100;
                let gbox = DATA + 0x2200;
                let (q, cq) = (DATA + 0x5000, DATA + 0x5200);
                // The two gather contexts: list/count/box triples at +0
                // (nodes), +0x10 (objects) and +0x20 (all).
                for (qi, ctx) in [q, cq].into_iter().enumerate() {
                    for k in 0..3u32 {
                        let sub = ctx + 0x10 * k;
                        c.bus
                            .write32(sub, DATA + 0x5_4000 + 0x400 * qi as u32 + 0x100 * k);
                        c.bus
                            .write32(sub + 4, DATA + 0x5_c000 + 0x400 * qi as u32 + 0x100 * k);
                        c.bus.write32(sub + 0xc, gbox);
                    }
                }
                // The gather box contains the root's object.
                for k in 0..3 {
                    c.bus.write32(gbox + 4 * k, f(-1.0e6));
                    c.bus.write32(gbox + 0x10 + 4 * k, f(1.0e6));
                }
                c.bus.write32(root + 0x20, 0);
                c.bus.write32(root + 0x24, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, 0);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x3000 + 0x158 + 4 * k, f(-1.0));
                    c.bus.write32(DATA + 0x3000 + 0x164 + 4 * k, f(1.0));
                }
                c.bus.write32(root + 0x28, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                // All eight children present; the first two have a leaf
                // grandchild for the inlined `Cell_GatherAllSubtree`.
                for i in 0..8u32 {
                    let node = DATA + 0x1100 + 0x100 * i;
                    c.bus.write32(root + 4 * i, node);
                    if i < 2 {
                        c.bus.write32(node, DATA + 0x1500 + 0x100 * i);
                    }
                }
                let level = seed % 12;
                let (x, y, z) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(cell, level);
                c.bus.write32(cell + 4, x);
                c.bus.write32(cell + 8, y);
                c.bus.write32(cell + 0xc, z);
                // Containing / disjoint / straddling query box.
                let s = f32::from_bits((level + 0x7e) << 23);
                for k in 0..3u32 {
                    let coord = [x, y, z][k as usize] as f32;
                    let lo = (2.0 * coord - 0.2) * s;
                    let hi = (2.0 * coord + 1.2) * s;
                    let (min, max) = match seed % 3 {
                        0 => (-1.0e6f32, 1.0e6f32),
                        1 => (1.0e6f32, 1.0e6f32 + 1.0),
                        _ => (0.5 * (lo + hi), 1.0e6f32),
                    };
                    c.bus.write32(box_at + 4 * k, f(min));
                    c.bus.write32(box_at + 0x10 + 4 * k, f(max));
                }
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(q);
                c.g[A2][0] = sx(cq);
                c.g[A3][0] = sx(box_at);
                c.g[T0][0] = sx(root);
                c.g[T1][0] = sx(cell);
            },
        );
    }
}
