//! Staging module oc657 (wave g37.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc657`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
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
        0x0032_81d0,
        sphere_tree_walk_levels_recursive,
        "SphereTree_WalkLevelsRecursive",
    ),
    (
        0x0032_cbf8,
        sphere_tree_collide_tree_node,
        "SphereTree_CollideTreeNode",
    ),
];

/// `void SphereTree_WalkLevelsRecursive(SphereTree* tree, int maxLevel, Vec4* pos,
/// int level, int maskOff)` (0x3281d0). While `level < maxLevel`, every child slot
/// whose bit is set in the tree's mask byte (`bitTree + maskOff`, shifted by the
/// slot) gets `pos` moved by the slot's centre (scaled by the level's scale) and
/// the walk recurses one level deeper; the mask offset advances by the level's
/// index offset times the slot. Below `maxLevel` the body makes one call through
/// the table at GP+0x2a90 (`+0x10d8`, then `+0x2bc`) and returns.
pub fn sphere_tree_walk_levels_recursive(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        let stack = sp(c);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S1, 0x90);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0xb0);
        c.g[S2][0] = c.g[A3][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = c.g[T0][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = 0xc;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
            c.mult(0, V0, S2, V0);
            c.g[A1][0] = 0;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[S6][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
            loop {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V0][0] =
                    sx(((c.g[V0][0] as u32) as i32 >> ((c.g[A1][0] as u32) & 31)) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let bit_set = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if bit_set {
                    c.cycles += 26;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.mfc1(3, 20);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, stack + 0x20);
                    c.g[A2][0] = c.g[SP][0];
                    c.lq(2, stack + 0x20);
                    c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.sq(2, stack + 0x10);
                    c.lqc2(3, c.g[S5][0] as u32);
                    c.lqc2(4, stack + 0x10);
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, stack + 0x20);
                    c.lq(3, stack + 0x20);
                    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.sq(3, stack);
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.mult(0, V0, S1, T0);
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    Callee {
                        addr: 0x0032_81d0,
                        f: ee::s10::SphereTree_WalkLevelsRecursive,
                    }
                    .call(c, 0x0032_82bc)?;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < 8) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_8238);
                    }
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xa0);
                    break;
                }
            }
        } else {
            c.cycles += 24;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
            c.mult(0, V0, S2, V0);
            c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.g[S5][0];
            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
            c.g[A2][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.g[A3][0] = 0x4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x90));
            c.lwc1(12, c.g[V0][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2b8));
            c.swc1(1, stack + 0x8);
            c.mul_s(12, 12, 0);
            c.swc1(1, stack);
            c.sw(stack + 0x4, 0);
            c.sw(stack + 0xc, 0);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2bc));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0032_8330)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `int SphereTree_CollideTreeNode(SphereTree* tree, Vec4* pos, Vec4* centre,
/// int level, int maskOff, ...)` (0x32cbf8). Collides the node's level: when
/// `level` is inside the tree and its mask byte is set, every child slot whose
/// bit is set is tested with `SphereTree_CollideSphereNode`; a hit there sends
/// the walk one level deeper through this function itself. Returns 1 on a hit,
/// else 0 (1 also for the early exits that leave the result at 1).
pub fn sphere_tree_collide_tree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x100);
        let stack = sp(c);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0xa0);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S5, 0x90);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x80);
        c.g[S5][0] = c.g[T2][0];
        frame.save(c, FP, 0x60);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, S2, 0xc0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S3, 0xb0);
        frame.save(c, S7, 0x70);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 20, 0xf0);
        c.sw(stack + 0x40, c.g[A2][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let outside = c.g[V1][0] == 0;
        c.g[S7][0] = c.lbu(c.g[V0][0] as u32);
        // Exits: `plus_one` is the early exit that adds a cycle and leaves
        // v0 = 1; the other breaks leave v0 as set.
        let mut plus_one = false;
        'body: {
            if outside {
                plus_one = true;
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
            let below_four = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !below_four {
                break 'body;
            }
            c.cycles += 2;
            let no_mask = c.g[S7][0] == 0;
            c.g[V0][0] = 0xc;
            if no_mask {
                plus_one = true;
                break 'body;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.mult(0, V0, S1, V0);
            c.g[A0][0] = 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx(((c.g[S7][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
            loop {
                'slot: {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let bit_set = c.g[V0][0] != 0;
                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if !bit_set {
                        break 'slot;
                    }
                    c.cycles += 35;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                    c.g[V1][0] = 0xc;
                    c.mult(0, S2, S1, V1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.mfc1(3, 20);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, stack + 0x30);
                    c.g[A0][0] = c.g[S4][0];
                    c.lq(2, stack + 0x30);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x80));
                    c.g[A3][0] = 0;
                    c.sq(2, stack + 0x20);
                    c.g[T0][0] = 0;
                    c.g[V0][0] = c.lw(stack + 0x40);
                    c.g[T1][0] = c.g[S6][0];
                    c.lqc2(3, c.g[V0][0] as u32);
                    c.lqc2(4, stack + 0x20);
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, stack + 0x30);
                    c.g[T2][0] = c.g[S5][0];
                    c.lq(2, stack + 0x30);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.sq(2, stack);
                    c.sq(2, stack + 0x10);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.lwc1(12, c.g[V0][0] as u32);
                    c.mul_s(12, 12, 0);
                    Callee {
                        addr: 0x0032_cdb0,
                        f: ee::s10::SphereTree_CollideSphereNode,
                    }
                    .call(c, 0x0032_cd20)?;
                    let miss = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if miss {
                        break 'slot;
                    }
                    c.cycles += 14;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[T1][0] = c.g[S6][0];
                    c.g[T2][0] = c.g[S5][0];
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.mult(0, V0, S3, T0);
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    Callee {
                        addr: 0x0032_cbf8,
                        f: ee::s10::SphereTree_CollideTreeNode,
                    }
                    .call(c, 0x0032_cd58)?;
                    let hit = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if hit {
                        break 'body;
                    }
                    break 'slot;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < 8) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(((c.g[S7][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_cc90);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
        }
        if plus_one {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 13;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Scramble the machine, then the register file the body reads: GP as in
    /// the game, the stack and return address at the test frame.
    fn walk_levels_base(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x3281_d000_0000_0001 ^ ((seed as u64) << 8));
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[RA][0] = sx(crate::difftest::RA);
        r
    }

    /// A tree at DATA+0x1000 (child centres at +0..0x90, the tree at +0x98),
    /// its info at DATA+0x1100, levels (12-byte records, index offsets at +8)
    /// at DATA+0x1200, mask bytes at DATA+0x1300. The table walker sits at
    /// DATA+0x20_0000 and calls a real leaf that returns 0.
    fn walk_levels_fixture(c: &mut Cpu, seed: u32, r: &mut Rng) {
        let this = DATA + 0x1000;
        let info = DATA + 0x1100;
        let levels = DATA + 0x1200;
        let masks = DATA + 0x1300;
        let pos = DATA + 0x1400;
        let table = DATA + 0x20_0000;
        let vtable = DATA + 0x20_2000;
        for i in 0..0x24u32 {
            c.bus.write32(this + 4 * i, r.f32(4.0));
        }
        c.bus.write32(this + 0x98, info);
        c.bus.write32(info + 0x20, levels);
        c.bus.write32(info + 0x28, masks);
        for d in 0..4u32 {
            let at = levels + 12 * d;
            c.bus.write32(at, r.f32(2.0));
            c.bus.write32(at + 4, r.f32(1.0));
            c.bus.write32(at + 8, 0x40 * (d + 1));
        }
        for i in 0..0x200u32 {
            let byte = if seed % 5 == 0 {
                0
            } else {
                r.next_u32() as u8
            };
            c.bus.write8(masks + i, byte);
        }
        for i in 0..4u32 {
            c.bus.write32(pos + 4 * i, r.f32(2.0));
        }
        c.bus.write32(GAME_GP + 0x2a90, table);
        c.bus.write32(table + 0x10d8, vtable);
        c.bus.write32(vtable + 0x2bc, 0x0014_06c0);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(1 + seed % 3);
        c.g[A2][0] = sx(pos);
        c.g[A3][0] = sx(seed % 4);
        c.g[T0][0] = sx((seed % 2) * 0x10);
        c.pc = 0x0032_81d0;
    }

    #[test]
    fn sphere_tree_walk_levels_recursive_matches() {
        check(
            ee::s10::f_003281d0__generated,
            sphere_tree_walk_levels_recursive,
            200,
            &|c, seed| {
                let mut r = walk_levels_base(c, seed);
                walk_levels_fixture(c, seed, &mut r);
            },
        );
    }

    fn tree_node_collide_base(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x32cb_f800_0000_0001 ^ ((seed as u64) << 8));
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[RA][0] = sx(crate::difftest::RA);
        r
    }

    /// The sphere walk's tree layout (as `sphere_tree_collide_sphere_node`'s
    /// fixture): two levels deep, mask bytes per node, child centres at self.
    fn tree_node_collide_fixture(c: &mut Cpu, seed: u32, r: &mut Rng) {
        let this = DATA + 0x1000;
        let info = DATA + 0x1100;
        let levels = DATA + 0x1200;
        let masks = DATA + 0x1300;
        let pos = DATA + 0x1400;
        let centre = DATA + 0x1420;
        for i in 0..0x24u32 {
            c.bus.write32(this + 4 * i, r.f32(2.0));
        }
        c.bus.write32(this + 0x98, info);
        c.bus.write32(info + 0xc, 2);
        c.bus.write32(info + 0x20, levels);
        c.bus.write32(info + 0x28, masks);
        for k in 0..3u32 {
            let at = levels + 12 * k;
            c.bus.write32(at, r.f32(2.0));
            c.bus.write32(at + 4, r.f32(1.0));
            c.bus.write32(at + 8, 0x40 * (k + 1));
        }
        for i in 0..0x200u32 {
            let byte = if seed % 4 == 0 {
                0
            } else {
                r.next_u32() as u8
            };
            c.bus.write8(masks + i, byte);
        }
        for i in 0..4u32 {
            c.bus.write32(pos + 4 * i, r.f32(2.0));
            c.bus.write32(centre + 4 * i, r.f32(2.0));
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(pos);
        c.g[A2][0] = sx(centre);
        c.g[A3][0] = sx(seed % 3);
        c.g[T0][0] = sx((seed % 2) * 0x10);
        c.g[T1][0] = sx(DATA + 0x1440);
        c.g[T2][0] = sx(DATA + 0x1460);
        c.f[12] = r.f32(8.0);
        c.pc = 0x0032_cbf8;
    }

    #[test]
    fn sphere_tree_collide_tree_node_matches() {
        check(
            ee::s10::f_0032cbf8__generated,
            sphere_tree_collide_tree_node,
            200,
            &|c, seed| {
                let mut r = tree_node_collide_base(c, seed);
                tree_node_collide_fixture(c, seed, &mut r);
            },
        );
    }
}
