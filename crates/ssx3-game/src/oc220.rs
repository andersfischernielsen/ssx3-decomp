//! Staging wave 201/202 (oc220): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0033_5d78, nash_collide_volume, "Nash_CollideVolume"),
    (
        0x0011_63b0,
        rider_update_tweak_layer_anim,
        "Rider_UpdateTweakLayerAnim",
    ),
    (0x0040_f5a8, kernel_tanf, "__kernel_tanf"),
    (
        0x0016_5938,
        chase_camera_algorithm_apply_shake,
        "ChaseCameraAlgorithm_ApplyShake",
    ),
    (0x0012_2898, rider_draw_shadow, "Rider_DrawShadow"),
    (
        0x0034_fed8,
        instance_get_node_matrix,
        "Instance_GetNodeMatrix",
    ),
    (
        0x0032_a1c0,
        rider_sphere_tree_collide_triangle,
        "RiderSphereTree_CollideTriangle",
    ),
    (0x003c_85d0, snd_ee_mix_block, "SND_EeMixBlock"),
    (
        0x0010_bd10,
        computer_actor_rider_select_target_rider,
        "ComputerActorRider_SelectTargetRider",
    ),
];

// ---------------------------------------------------------------------------
// Nash_CollideVolume

/// `int SpatialKey_ClassifyAABB(const CellKey* cell, const Box* box)`
/// (0x328360, through the function table): 0 = the box sits on the cell's
/// high corner ("inside": collect the whole tree), 1 = outside, 2 = partly
/// inside (recurse).
const CLASSIFY_AABB: u32 = 0x0032_8360;
/// `Cell_CollideInstance(ctx, instance)` (0x335960).
const COLLIDE_INSTANCE: Callee = Callee {
    addr: 0x0033_5960,
    f: ee::s11::Cell_CollideInstance,
};
/// `Cell_CollideObject(ctx, object)` (0x335b90).
const COLLIDE_OBJECT: Callee = Callee {
    addr: 0x0033_5b90,
    f: ee::s11::Cell_CollideObject,
};
/// `Cell_CollideVolumeSubtree(tree, ctx)` (0x340fa0).
const COLLIDE_VOLUME_SUBTREE: Callee = Callee {
    addr: 0x0034_0fa0,
    f: ee::s11::Cell_CollideVolumeSubtree,
};
/// `Cell_CollideVolumeRecursive(flag, ctx, ctx, box, tree, cell)` (0x33ccf8).
const COLLIDE_VOLUME_RECURSIVE: Callee = Callee {
    addr: 0x0033_ccf8,
    f: ee::s11::Cell_CollideVolumeRecursive,
};

/// The Nash grid: eight top-level cells of 0x14 bytes (`{ int level, x, y,
/// z; TreeNode* tree; }`) followed by the forward-difference cache.
mod nash {
    pub const CELL_SIZE: u32 = 0x14;
    pub const CELL_TREE: u32 = 0x10;
    pub const CACHE: u32 = 0xa4;
}

/// Tree node: `{ TreeNode* child[8]; Object* objects; Instance* instances; }`.
mod tree_node {
    pub const OBJECTS: u32 = 0x20;
    pub const INSTANCES: u32 = 0x24;
}

/// Frame layout of `Nash_CollideVolume` (0x100 bytes).
mod nash_frame {
    /// The collide context `{ q, hits, cap, &count, arg, cache }` (at sp+0).
    pub const CTX_HITS: u32 = 0x04;
    pub const CTX_CAP: u32 = 0x08;
    pub const CTX_COUNT: u32 = 0x0c;
    pub const CTX_ARG: u32 = 0x10;
    pub const CTX_CACHE: u32 = 0x14;
    /// The query box (`q+0x30` then `q+0x20`).
    pub const BOX: u32 = 0x20;
    /// The hit count (the result).
    pub const COUNT: u32 = 0x40;
    /// One byte per top-level cell handed to the recursion.
    pub const FLAGS: u32 = 0x44;
    /// Spilled pointers to cells 6 and 7.
    pub const CELL6: u32 = 0x4c;
    pub const CELL7: u32 = 0x50;
}

/// The part of one top-level cell after the classification call returned to
/// `r` (the return address of that call; every later call address follows
/// from it, four bytes earlier in the `short` cells 2-5 whose flag pointer
/// was set before the call): an inside cell collides its instance and object
/// lists and collects its eight subtrees, a partly-inside cell recurses with
/// its flag byte, an outside cell is skipped. Ends by loading the next cell's
/// tree (or, after the last cell, the hit count) from `next` into v0.
fn nash_cell_after_classify(
    c: &mut Cpu,
    r: u32,
    short: bool,
    flag: u64,
    cell: u64,
    next: u32,
) -> Flow {
    let r = if short { r - 4 } else { r };
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    if c.g[V1][0] == 0 {
        // Inside: instances, objects, then the eight subtrees.
        let tree = r32(c, S2);
        c.cycles += 3;
        c.g[S0][0] = c.lw(tree.wrapping_add(tree_node::INSTANCES));
        if c.g[S0][0] != 0 {
            c.cycles += if short { 1 } else { 2 };
            c.g[A1][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                COLLIDE_INSTANCE.call(c, r + 0x28)?;
                c.g[S0][0] = c.lw(r32(c, S0));
                c.g[A1][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, r + 0x20);
                }
            }
        }
        c.g[S0][0] = c.lw(tree.wrapping_add(tree_node::OBJECTS));
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                COLLIDE_OBJECT.call(c, r + 0x50)?;
                c.g[S0][0] = c.lw(r32(c, S0));
                c.g[A1][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, r + 0x48);
                }
            }
        }
        c.g[A0][0] = c.lw(tree);
        for i in 0..8u32 {
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += if i == 7 { 4 } else { 3 };
                c.g[A1][0] = c.g[S1][0];
                COLLIDE_VOLUME_SUBTREE.call(c, r + 0x70 + 0x14 * i)?;
            }
            if i < 7 {
                c.g[A0][0] = c.lw(tree.wrapping_add(4 * (i + 1)));
            }
        }
    } else if c.g[V1][0] != 2 {
        // Outside.
        c.cycles += 2;
    } else {
        // Partly inside: recurse with this cell's flag byte (the compare
        // block's 2 cycles, then 7).
        c.cycles += 2;
        c.cycles += 7;
        c.g[A0][0] = flag;
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = cell;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = sx(sp(c).wrapping_add(nash_frame::BOX));
        COLLIDE_VOLUME_RECURSIVE.call(c, r + 0x128)?;
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw(next);
    Ok(())
}

/// `int Nash_CollideVolume(Nash* grid, Query* q, Hit* hits, int cap, int
/// arg)` (0x335d78): collide a query volume with the world's Nash octree.
/// Builds the collide context and the query box in the frame, then visits
/// the grid's eight top-level cells in order: a cell with a tree is
/// classified against the box (`SpatialKey_ClassifyAABB`) and collected,
/// recursed into or skipped. Returns the number of hits written.
pub fn nash_collide_volume(c: &mut Cpu) {
    guest(c, |c| {
        use nash_frame::*;
        c.cycles += 28;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S1, 0xe0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save(c, FP, 0x70);
        frame.save_ra(c, 0x60);
        let s = sp(c);
        c.g[V1][0] = sx(s.wrapping_add(COUNT));
        c.g[S1][0] = c.g[SP][0];
        c.g[S4][0] = c.g[A0][0];
        let grid = r32(c, S4);
        let q = r32(c, A1);
        // The query's two box corners (a0/v0 keep all 128 bits of them).
        c.lq(A0, q.wrapping_add(0x20));
        c.g[T1][0] = c.lw(grid.wrapping_add(nash::CACHE));
        c.lq(V0, q.wrapping_add(0x30));
        c.g[S2][0] = c.lw(grid.wrapping_add(nash::CELL_TREE));
        c.sw(s.wrapping_add(CTX_HITS), c.g[A2][0]);
        c.sw(s.wrapping_add(CTX_CAP), c.g[A3][0]);
        c.sw(s.wrapping_add(CTX_COUNT), c.g[V1][0]);
        c.sw(s.wrapping_add(CTX_ARG), c.g[T0][0]);
        c.sw(s.wrapping_add(CTX_CACHE), c.g[T1][0]);
        c.sq(V0, s.wrapping_add(BOX));
        c.sq(A0, s.wrapping_add(BOX + 0x10));
        c.sw(s, c.g[A1][0]);
        c.sw(s.wrapping_add(COUNT), 0);
        let cell_at = |k: u32| grid.wrapping_add(nash::CELL_SIZE * k);
        let tree_of = |k: u32| cell_at(k).wrapping_add(nash::CELL_TREE);

        // Cell 0 (its tree already in s2).
        if c.g[S2][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(tree_of(1));
        } else {
            c.cycles += 7;
            c.g[S0][0] = sx(s.wrapping_add(FLAGS));
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = sx(s.wrapping_add(BOX));
            call_ptr(c, CLASSIFY_AABB, 0x0033_5df8)?;
            let (flag, cell) = (c.g[S0][0], c.g[S4][0]);
            nash_cell_after_classify(c, 0x0033_5df8, false, flag, cell, tree_of(1))?;
        }

        // Cell 1; also spreads the remaining cell pointers over s5-fp and
        // the frame.
        c.cycles += 10;
        c.g[V1][0] = sx(cell_at(6));
        c.sw(s.wrapping_add(CELL6), c.g[V1][0]);
        c.g[S0][0] = sx(cell_at(1));
        c.g[V1][0] = sx(cell_at(7));
        c.g[S5][0] = sx(cell_at(2));
        c.g[S6][0] = sx(cell_at(3));
        c.g[S7][0] = sx(cell_at(4));
        c.g[FP][0] = sx(cell_at(5));
        c.sw(s.wrapping_add(CELL7), c.g[V1][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(tree_of(2));
        } else {
            c.cycles += 8;
            c.g[S2][0] = c.lw(cell_at(1).wrapping_add(nash::CELL_TREE));
            c.g[S3][0] = sx(s.wrapping_add(FLAGS + 1));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(s.wrapping_add(BOX));
            call_ptr(c, CLASSIFY_AABB, 0x0033_5f60)?;
            let (flag, cell) = (c.g[S3][0], c.g[S0][0]);
            nash_cell_after_classify(c, 0x0033_5f60, false, flag, cell, tree_of(2))?;
        }

        // Cells 2-5, held in s5, s6, s7, fp.
        for (k, reg, r) in [
            (2u32, S5, 0x0033_60a4u32),
            (3, S6, 0x0033_61e4),
            (4, S7, 0x0033_6324),
            (5, FP, 0x0033_6464),
        ] {
            c.cycles += 2;
            let has_tree = c.g[V0][0] != 0;
            c.g[S0][0] = sx(s.wrapping_add(FLAGS + k));
            if !has_tree {
                c.cycles += 1;
                c.g[V0][0] = c.lw(tree_of(k + 1));
                continue;
            }
            c.cycles += 7;
            c.g[S2][0] = c.lw(r32(c, reg).wrapping_add(nash::CELL_TREE));
            c.g[A0][0] = c.g[reg][0];
            c.g[A1][0] = sx(s.wrapping_add(BOX));
            call_ptr(c, CLASSIFY_AABB, r)?;
            let (flag, cell) = (c.g[S0][0], c.g[reg][0]);
            nash_cell_after_classify(c, r, true, flag, cell, tree_of(k + 1))?;
        }

        // Cells 6 and 7, reloaded from the frame (into v0, then v1).
        for (k, reg, spill, r, next) in [
            (6u32, V0, CELL6, 0x0033_65a8u32, tree_of(7)),
            (7, V1, CELL7, 0x0033_66f0, s.wrapping_add(COUNT)),
        ] {
            c.cycles += 2;
            let has_tree = c.g[V0][0] != 0;
            c.g[reg][0] = c.lw(s.wrapping_add(spill));
            if !has_tree {
                c.cycles += 1;
                c.g[V0][0] = c.lw(next);
                continue;
            }
            c.cycles += 8;
            let cell = c.g[reg][0];
            c.g[S0][0] = sx(s.wrapping_add(FLAGS + k));
            c.g[A1][0] = sx(s.wrapping_add(BOX));
            c.g[S2][0] = c.lw((cell as u32).wrapping_add(nash::CELL_TREE));
            c.g[A0][0] = cell;
            call_ptr(c, CLASSIFY_AABB, r)?;
            let flag = c.g[S0][0];
            nash_cell_after_classify(c, r, false, flag, cell, next)?;
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_UpdateTweakLayerAnim

/// `int AnimController_GetLayerAnimEntry(ctrl, layer)` (0x311ae8): the state
/// of the animation playing on a layer (0 when none).
const LAYER_STATE: u32 = 0x0031_1ae8;
/// `AnimSequence* AnimController_GetLayerSequence(ctrl, layer)` (0x311b20).
const LAYER_SEQ: u32 = 0x0031_1b20;
/// `AnimController_PlayAnim(ctrl, id, flags, f12 = speed)` (0x3128e8).
const PLAY_ANIM: u32 = 0x0031_28e8;
/// `AnimController_FadeOutLayer(ctrl, layer, f12 = time)` (0x311e88).
const FADE_OUT_LAYER: u32 = 0x0031_1e88;
/// `int AnimController_GetLayerAnimId(ctrl, layer)` (0x312aa0).
const LAYER_ANIM_ID: u32 = 0x0031_2aa0;
/// `AnimSequence_SetWeightFade(seq, f12, f13)` (0x313a10).
const SET_WEIGHT_FADE: u32 = 0x0031_3a10;
/// `bool Flags64_Test(flags, bit)` (0x1446a0) and its second-word form
/// `Flags64_TestSecond` (0x1446b8).
const FLAGS_TEST: u32 = 0x0014_46a0;
const FLAGS_TEST_SECOND: u32 = 0x0014_46b8;

/// Rider fields used by the tweak layer.
mod tweak {
    pub const SIDE: u32 = 0x1a0;
    pub const TIME_SCALE: u32 = 0x300;
    pub const SWITCH_SIDE: u32 = 0x320;
    pub const ATTACK_DIR: u32 = 0x340;
    pub const BLEND: u32 = 0x350;
    pub const ANIM: u32 = 0x784;
    pub const EVENT_OWNER: u32 = 0x8c0;
    /// Controller: the owner word stamped on the events of a new animation.
    pub const CTRL_EVENT_OWNER: u32 = 0x20;
    /// Sequence fields.
    pub const SEQ_FADE: u32 = 0x90;
    pub const SEQ_WEIGHT: u32 = 0x94;
    pub const SEQ_TARGET: u32 = 0x98;
    pub const SEQ_FADE_TIME: u32 = 0x9c;
    pub const SEQ_FLAGS: u32 = 0xb0;
    /// The tweak animations.
    pub const ANIM_BOTH: u32 = 0x142;
    pub const ANIM_LEFT: u32 = 0x143;
    pub const ANIM_RIGHT: u32 = 0x144;
    pub const ANIM_BOTH_FROM: u32 = 0x145;
    pub const ANIM_LEFT_OUT: u32 = 0x146;
    pub const ANIM_RIGHT_OUT: u32 = 0x147;
    /// Animation states.
    pub const STATE_BOTH: u64 = 3;
    pub const STATE_ONE: u64 = 0xd;
}

/// `ctrl = rider->anim` (`lw` from the rider in s0).
fn tweak_anim(c: &mut Cpu) -> u64 {
    c.lw(r32(c, S0).wrapping_add(tweak::ANIM))
}

/// `-1.0f` into f12 through `at`.
fn tweak_minus_one_f12(c: &mut Cpu) {
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 12);
}

/// Play `id` on the controller with the rider's event owner stamped on it
/// (`v1` = controller, `a1` = id, `a2` = flags), then clear the owner.
fn tweak_play_stamped(c: &mut Cpu, ret: u32) -> Flow {
    let rider = r32(c, S0);
    c.g[V0][0] = c.ld(rider.wrapping_add(tweak::EVENT_OWNER));
    c.sd(r32(c, V1).wrapping_add(tweak::CTRL_EVENT_OWNER), c.g[V0][0]);
    c.g[A0][0] = tweak_anim(c);
    call_ptr(c, PLAY_ANIM, ret)
}

/// Test bits 2 of layer 1's sequence flags (both words), into `dst`
/// (s2 or s3): the sequence finished.
fn tweak_seq_done(c: &mut Cpu, dst: usize, rets: [u32; 3]) -> Flow {
    c.g[A1][0] = 1;
    c.g[dst][0] = 0;
    call_ptr(c, LAYER_SEQ, rets[0])?;
    c.g[S1][0] = sx(r32(c, V0).wrapping_add(tweak::SEQ_FLAGS));
    c.g[A1][0] = 2;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, FLAGS_TEST, rets[1])?;
    let set = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if set {
        c.cycles += 3;
        c.g[A1][0] = 2;
        call_ptr(c, FLAGS_TEST_SECOND, rets[2])?;
        c.g[dst][0] = (c.g[V0][0] != 0) as u64;
    }
    Ok(())
}

/// Layer 1's sequence weight fade = 1.0, and fade layer 0 out over the
/// time at `gp_off` (`A0` already holds the controller).
fn tweak_fade_out(c: &mut Cpu, gp_off: u32, rets: [u32; 2]) -> Flow {
    c.cycles += 9;
    c.g[A1][0] = 1;
    call_ptr(c, LAYER_SEQ, rets[0])?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[A1][0] = 0;
    c.lwc1(12, r32(c, GP).wrapping_sub(gp_off));
    c.swc1(0, r32(c, V0).wrapping_add(tweak::SEQ_FADE));
    c.g[A0][0] = tweak_anim(c);
    call_ptr(c, FADE_OUT_LAYER, rets[1])
}

/// Flags64 test of bit `bit` of layer 1's sequence (`a0` = controller):
/// `rets` = the sequence call's and the test's return addresses.
fn tweak_seq_flag(c: &mut Cpu, bit: u64, rets: [u32; 2]) -> Flow {
    c.g[A1][0] = 1;
    call_ptr(c, LAYER_SEQ, rets[0])?;
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(tweak::SEQ_FLAGS));
    c.g[A1][0] = bit;
    call_ptr(c, FLAGS_TEST, rets[1])
}

/// The rider's side vector negated through VU0 (`v0` = side, the GPR
/// `scratch` carries -1.0 to vf3), as the original does it.
fn tweak_negate_side(c: &mut Cpu, scratch: usize) {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    tweak_minus_one_at_f0(c);
    c.sq(V0, s.wrapping_add(0x10));
    c.mfc1(scratch, 0);
    c.lqc2(4, s.wrapping_add(0x10));
    c.qmtc2(scratch, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x20));
    c.lq(V0, s.wrapping_add(0x20));
}

fn tweak_minus_one_at_f0(c: &mut Cpu) {
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
}

/// Layer 1 in the "both hands" state (3): with both inputs held, once the
/// sequence finished, restart the "both" animation from layer 0's weights;
/// otherwise fade layer 0 out.
fn tweak_layer_both(c: &mut Cpu) -> Flow {
    use tweak::*;
    c.cycles += 2;
    c.g[A0][0] = tweak_anim(c);
    let both = c.g[S6][0] != 0 && {
        c.cycles += 2;
        c.g[S5][0] != 0
    };
    if both {
        c.cycles += 9;
        tweak_seq_done(c, S2, [0x0011_6420, 0x0011_6430, 0x0011_6440])?;
        c.cycles += 2;
        c.g[A0][0] = tweak_anim(c);
        if c.g[S2][0] == 0 {
            return Ok(());
        }
        c.cycles += 33;
        c.g[A1][0] = 1;
        call_ptr(c, LAYER_SEQ, 0x0011_6454)?;
        c.sw(r32(c, V0).wrapping_add(SEQ_FADE), 0);
        c.g[A2][0] = 0;
        tweak_minus_one_f12(c);
        c.g[A1][0] = ANIM_BOTH_FROM as u64;
        c.g[V1][0] = tweak_anim(c);
        tweak_play_stamped(c, 0x0011_647c)?;
        // Layer 1's weight, target and fade time, onto layer 0.
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 1;
        call_ptr(c, LAYER_SEQ, 0x0011_6488)?;
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 1;
        c.lwc1(20, r32(c, V0).wrapping_add(SEQ_WEIGHT));
        call_ptr(c, LAYER_SEQ, 0x0011_6498)?;
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 1;
        c.lwc1(22, r32(c, V0).wrapping_add(SEQ_TARGET));
        call_ptr(c, LAYER_SEQ, 0x0011_64a8)?;
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 0;
        c.lwc1(21, r32(c, V0).wrapping_add(SEQ_FADE_TIME));
        call_ptr(c, LAYER_SEQ, 0x0011_64b8)?;
        c.mtc1(0, 0);
        c.swc1(20, r32(c, V0).wrapping_add(SEQ_WEIGHT));
        c.c_eq_s(21, 0);
        let no_fade = c.fcc();
        c.swc1(20, r32(c, V0).wrapping_add(SEQ_TARGET));
        if !no_fade {
            c.cycles += 7;
            c.g[A0][0] = tweak_anim(c);
            c.g[A1][0] = 0;
            call_ptr(c, LAYER_SEQ, 0x0011_64dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.mov_s(12, 22);
            c.mov_s(13, 21);
            call_ptr(c, SET_WEIGHT_FADE, 0x0011_64ec)?;
        }
        c.cycles += 4;
        c.g[V1][0] = tweak_anim(c);
        c.g[V0][0] = u64::MAX;
        c.sd(r32(c, V1).wrapping_add(CTRL_EVENT_OWNER), c.g[V0][0]);
    } else {
        tweak_fade_out(c, 0x7b24, [0x0011_6504, 0x0011_6520])?;
    }
    c.cycles += 1;
    c.g[A0][0] = tweak_anim(c);
    Ok(())
}

/// Layer 1 in the "one hand" state (0xd): when exactly one input is held
/// and it matches the side being tweaked (or the other input for the other
/// side), finish the animation once its sequence is done, step layer 0's
/// blend towards 1 and point the attack direction to the side; otherwise
/// fade layer 0 out.
fn tweak_layer_one(c: &mut Cpu) -> Flow {
    use tweak::*;
    let rider = r32(c, S0);
    c.cycles += 7;
    c.g[A0][0] = tweak_anim(c);
    c.g[A1][0] = 1;
    call_ptr(c, LAYER_ANIM_ID, 0x0011_6544)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(rider.wrapping_add(SWITCH_SIDE));
    let switched = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S2][0] ^ ANIM_LEFT as u64;
    if !switched {
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0] ^ ANIM_RIGHT as u64;
    }
    // s4: the playing animation is the side's own.
    c.cycles += 2;
    c.g[S4][0] = (c.g[V0][0] < 1) as u64;
    let matches = c.g[S4][0] != 0;
    let active;
    if c.g[S6][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = tweak_anim(c);
        if c.g[S5][0] != 0 {
            active = false;
        } else {
            c.cycles += 2;
            active = matches;
            if !matches {
                c.cycles += 2;
            }
        }
    } else {
        c.cycles += 2;
        c.g[A0][0] = tweak_anim(c);
        if c.g[S5][0] == 0 {
            active = false;
        } else {
            c.cycles += 2;
            active = !matches;
        }
    }
    if !active {
        return tweak_fade_out(c, 0x7b1c, [0x0011_66dc, 0x0011_66f8]);
    }

    c.cycles += 9;
    tweak_seq_done(c, S3, [0x0011_6594, 0x0011_65a4, 0x0011_65b4])?;
    c.cycles += 2;
    c.g[A0][0] = tweak_anim(c);
    if c.g[S3][0] != 0 {
        // Finished: play the side's "out" animation.
        c.cycles += 8;
        c.g[A1][0] = 1;
        call_ptr(c, LAYER_SEQ, 0x0011_65c8)?;
        c.sw(r32(c, V0).wrapping_add(SEQ_FADE), 0);
        c.g[A0][0] = ANIM_LEFT as u64;
        c.g[V1][0] = tweak_anim(c);
        c.g[V0][0] = c.ld(rider.wrapping_add(EVENT_OWNER));
        let left = c.g[S2][0] == c.g[A0][0];
        c.sd(r32(c, V1).wrapping_add(CTRL_EVENT_OWNER), c.g[V0][0]);
        let (id, ret) = if left {
            c.cycles += 8;
            (ANIM_LEFT_OUT, 0x0011_65f8)
        } else {
            c.cycles += 7;
            (ANIM_RIGHT_OUT, 0x0011_6618)
        };
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = id as u64;
        tweak_minus_one_f12(c);
        c.g[A2][0] = 0;
        call_ptr(c, PLAY_ANIM, ret)?;
        c.g[V1][0] = tweak_anim(c);
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        c.sd(r32(c, V1).wrapping_add(CTRL_EVENT_OWNER), c.g[V0][0]);
        c.g[A0][0] = tweak_anim(c);
    }

    // Layer 0 also one-handed: step the blend towards 1 by
    // `time_scale * step` and copy it to layer 0's weight and target.
    c.cycles += 5;
    c.g[A1][0] = 0;
    call_ptr(c, LAYER_STATE, 0x0011_6630)?;
    c.g[V1][0] = STATE_ONE;
    if c.g[V0][0] == STATE_ONE {
        c.cycles += 11;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7b20));
        c.lwc1(1, rider.wrapping_add(TIME_SCALE));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        c.mul_s(1, 1, 0);
        c.lwc1(2, rider.wrapping_add(BLEND));
        c.add_s(0, 1, 3);
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.sub_s(0, 3, 1);
            c.cycles += 4;
            c.c_lt_s(2, 0);
            if c.fcc() {
                c.add_s(3, 2, 1);
            }
        } else {
            c.cycles += 2;
            c.sub_s(3, 2, 1);
        }
        c.cycles += 7;
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 0;
        c.swc1(3, rider.wrapping_add(BLEND));
        call_ptr(c, LAYER_SEQ, 0x0011_6690)?;
        c.lwc1(0, rider.wrapping_add(BLEND));
        c.swc1(0, r32(c, V0).wrapping_add(SEQ_WEIGHT));
        c.swc1(0, r32(c, V0).wrapping_add(SEQ_TARGET));
    }

    // Attack direction: the side vector, negated for the side's own anim.
    c.cycles += 2;
    c.lq(V0, rider.wrapping_add(SIDE));
    if c.g[S4][0] != 0 {
        c.cycles += 9;
        tweak_negate_side(c, V1);
    }
    c.cycles += 3;
    c.sq(V0, rider.wrapping_add(ATTACK_DIR));
    c.sq(V0, sp(c));
    Ok(())
}

/// The input part: start the tweak animation for the held inputs (both:
/// 0x142; one: the side's or the other side's), unless layer 1 still plays
/// an unfinished tweak (then return 1 at once). Returns whether layer 1
/// ends in a tweak state, in v0.
fn tweak_start(c: &mut Cpu) -> Flow {
    use tweak::*;
    let rider = r32(c, S0);
    /// What follows the input classification.
    enum Then {
        Both,
        Single,
        Play,
        Final,
    }
    c.cycles += 2;
    let then = if c.g[S6][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = tweak_anim(c);
        if c.g[S5][0] != 0 {
            Then::Both
        } else {
            Then::Single
        }
    } else {
        c.cycles += 2;
        c.g[A0][0] = tweak_anim(c);
        if c.g[S5][0] == 0 {
            Then::Final
        } else {
            Then::Single
        }
    };
    let then = match then {
        Then::Both => {
            // Both inputs: busy while a one-handed tweak plays unfinished
            // or the "both" one has not ended.
            c.cycles += 5;
            c.g[A1][0] = 1;
            call_ptr(c, LAYER_STATE, 0x0011_6710)?;
            c.g[V1][0] = STATE_ONE;
            let one = c.g[V0][0] == STATE_ONE;
            c.g[A0][0] = tweak_anim(c);
            if one {
                c.cycles += 7;
                tweak_seq_flag(c, 0, [0x0011_6724, 0x0011_6730])?;
                let started = c.g[V0][0] != 0;
                c.g[A0][0] = tweak_anim(c);
                if started {
                    c.cycles += 7;
                    tweak_seq_flag(c, 1, [0x0011_6740, 0x0011_674c])?;
                    let done = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if !done {
                        return Ok(());
                    }
                    c.cycles += 1;
                    c.g[A0][0] = tweak_anim(c);
                }
            }
            c.cycles += 5;
            c.g[A1][0] = 1;
            call_ptr(c, LAYER_STATE, 0x0011_6760)?;
            c.g[V1][0] = STATE_BOTH;
            if c.g[V0][0] == STATE_BOTH {
                c.cycles += 8;
                c.g[A0][0] = tweak_anim(c);
                tweak_seq_flag(c, 1, [0x0011_6778, 0x0011_6784])?;
                let done = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !done {
                    return Ok(());
                }
                c.cycles += 1;
            }
            c.g[V1][0] = tweak_anim(c);
            c.cycles += 2;
            c.g[A1][0] = ANIM_BOTH as u64;
            Then::Play
        }
        other => other,
    };
    let then = match then {
        Then::Single => {
            // One input: busy while the "both" tweak plays unfinished or a
            // one-handed one has not ended.
            c.cycles += 5;
            c.g[A1][0] = 1;
            call_ptr(c, LAYER_STATE, 0x0011_67a8)?;
            c.g[V1][0] = STATE_BOTH;
            let both = c.g[V0][0] == STATE_BOTH;
            c.g[A0][0] = tweak_anim(c);
            if both {
                c.cycles += 7;
                tweak_seq_flag(c, 0, [0x0011_67bc, 0x0011_67c8])?;
                let started = c.g[V0][0] != 0;
                c.g[A0][0] = tweak_anim(c);
                if started {
                    c.cycles += 7;
                    tweak_seq_flag(c, 1, [0x0011_67d8, 0x0011_67e4])?;
                    let done = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if !done {
                        return Ok(());
                    }
                    c.cycles += 1;
                    c.g[A0][0] = tweak_anim(c);
                }
            }
            c.cycles += 5;
            c.g[A1][0] = 1;
            call_ptr(c, LAYER_STATE, 0x0011_67f8)?;
            c.g[V1][0] = STATE_ONE;
            if c.g[V0][0] == STATE_ONE {
                c.cycles += 8;
                c.g[A0][0] = tweak_anim(c);
                tweak_seq_flag(c, 1, [0x0011_6810, 0x0011_681c])?;
                let done = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !done {
                    return Ok(());
                }
            }
            // The animation for the held side, and the attack direction.
            c.cycles += 2;
            c.g[A2][0] = ANIM_RIGHT as u64;
            if c.g[S6][0] != 0 {
                c.cycles += 16;
                c.lq(V0, rider.wrapping_add(SIDE));
                c.g[A0][0] = c.lw(rider.wrapping_add(SWITCH_SIDE));
                c.g[V1][0] = ANIM_LEFT as u64;
                if c.g[A0][0] != 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
                tweak_negate_side(c, A1);
                c.sq(V0, rider.wrapping_add(ATTACK_DIR));
                c.sq(V0, sp(c));
            } else {
                c.cycles += 7;
                c.lq(V0, rider.wrapping_add(SIDE));
                c.g[A2][0] = ANIM_LEFT as u64;
                c.g[A0][0] = c.lw(rider.wrapping_add(SWITCH_SIDE));
                c.g[V1][0] = ANIM_RIGHT as u64;
                c.sq(V0, rider.wrapping_add(ATTACK_DIR));
                c.sq(V0, sp(c));
                if c.g[A0][0] != 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
            }
            c.cycles += 2;
            c.g[V1][0] = tweak_anim(c);
            c.g[A1][0] = c.g[A2][0];
            Then::Play
        }
        other => other,
    };
    if let Then::Play = then {
        c.cycles += 12;
        c.g[A2][0] = 1;
        tweak_minus_one_f12(c);
        tweak_play_stamped(c, 0x0011_68ac)?;
        c.g[V1][0] = tweak_anim(c);
        c.g[V0][0] = u64::MAX;
        c.sd(r32(c, V1).wrapping_add(CTRL_EVENT_OWNER), c.g[V0][0]);
        c.sw(rider.wrapping_add(BLEND), 0);
        c.g[A0][0] = tweak_anim(c);
    }

    // Result: layer 1 in a tweak state.
    c.cycles += 6;
    c.g[A1][0] = 1;
    c.g[S1][0] = 0;
    call_ptr(c, LAYER_STATE, 0x0011_68cc)?;
    c.g[V1][0] = STATE_BOTH;
    if c.g[V0][0] == STATE_BOTH {
        c.g[S1][0] = 1;
    } else {
        c.cycles += 6;
        c.g[A0][0] = tweak_anim(c);
        c.g[A1][0] = 1;
        call_ptr(c, LAYER_STATE, 0x0011_68e4)?;
        c.g[V1][0] = STATE_ONE;
        let one = c.g[V0][0] == STATE_ONE;
        c.g[V0][0] = c.g[S1][0];
        if !one {
            return Ok(());
        }
        c.cycles += 1;
        c.g[S1][0] = 1;
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S1][0];
    Ok(())
}

/// `bool Rider_UpdateTweakLayerAnim(Rider* rider, int left, int right)`
/// (0x1163b0): drive the "tweak" (grab-style hand) animations on the
/// rider's animation layers from the two tweak inputs: settle the layer-1
/// "both hands" (state 3) and "one hand" (state 0xd) animations, then start
/// the animation for the inputs held now. Returns whether layer 1 plays a
/// tweak afterwards (or 1 at once while one is still busy).
pub fn rider_update_tweak_layer_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[A1][0] = 1;
        c.g[A0][0] = tweak_anim(c);
        call_ptr(c, LAYER_STATE, 0x0011_63f8)?;
        c.g[V1][0] = tweak::STATE_BOTH;
        if c.g[V0][0] == tweak::STATE_BOTH {
            tweak_layer_both(c)?;
        } else {
            c.g[A0][0] = tweak_anim(c);
        }

        c.cycles += 5;
        c.g[A1][0] = 1;
        call_ptr(c, LAYER_STATE, 0x0011_652c)?;
        c.g[V1][0] = tweak::STATE_ONE;
        if c.g[V0][0] == tweak::STATE_ONE {
            tweak_layer_one(c)?;
        }

        tweak_start(c)?;

        c.cycles += 13;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __kernel_tanf

/// `float fabsf(float)` (0x40f8a0).
const FABSF: Callee = Callee {
    addr: 0x0040_f8a0,
    f: ee::s15::fabsf,
};

/// The polynomial coefficients `T[0..12]` of newlib's `__kernel_tanf`.
const TANF_T: u32 = 0x0049_9648;

/// The newlib `__kernel_tanf` tail for `|x| >= 0.6744`: undo the `pi/4`
/// reduction, `v = iy; w = x + r; (1 - ((hx >> 30) & 2)) * (v - 2(x -
/// (w*w/(w+v) - r)))` (f3 = r, f11 = w, f12 = x).
fn tanf_unreduce(c: &mut Cpu) {
    c.cycles += 19;
    c.mtc1(A0, 4);
    c.cvt_s_w(4, 4);
    c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> 30) as u32);
    c.mul_s(0, 11, 11);
    c.g[V1][0] &= 2;
    c.g[V0][0] = 1;
    c.add_s(1, 11, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    c.div_s(0, 0, 1);
    c.sub_s(0, 0, 3);
    c.sub_s(0, 12, 0);
    c.add_s(0, 0, 0);
    c.sub_s(0, 4, 0);
    c.mul_s(0, 2, 0);
}

/// `-1/w` computed carefully for `iy == -1` and small `|x|` (f3 = r,
/// f11 = w, f12 = x): `z = w` and `t = a = -1/w` truncated to 20 bits,
/// `s = 1 + t*z`, `t + a*(s + t*(r - (z - x)))`.
fn tanf_neg_recip(c: &mut Cpu) {
    c.cycles += 29;
    c.mfc1(V0, 11);
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff_ffff_ffff_f000;
    c.g[V1][0] &= c.g[V0][0];
    c.mtc1(V1, 14);
    c.sub_s(1, 14, 12);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.div_s(0, 0, 11);
    c.sub_s(4, 3, 1);
    c.mfc1(A0, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V1][0] &= c.g[V0][0];
    c.mtc1(V1, 0);
    c.mul_s(1, 0, 14);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.add_s(15, 1, 0);
    c.mtc1(V1, 1);
    c.mul_s(0, 1, 4);
    c.mtc1(A0, 1);
    c.add_s(0, 15, 0);
    c.mul_s(0, 1, 0);
    c.mtc1(V1, 1);
    c.add_s(0, 1, 0);
}

/// `float __kernel_tanf(float x, float y, int iy)` (0x40f5a8, newlib):
/// tan(x + y) on `[-pi/4, pi/4]` (`iy == 1`), or `-1/tan` (`iy == -1`).
/// Tiny `|x|` returns x (or `1/|x|` for zero x with `iy == -1`, else
/// `-1/x`); above 0.6744 the argument is reduced to `pi/4 - x` first; the
/// odd polynomial uses the 13 coefficients at 0x499648.
pub fn kernel_tanf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.mfc1(V0, 12);
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x7fff_ffff;
        c.g[V0][0] = 0x317f_ffff;
        c.g[A1][0] = c.g[A2][0] & c.g[V1][0];
        let tiny_limit = c.g[V0][0];
        c.g[V0][0] = ((tiny_limit as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save_ra(c, 0);
        let mut main = c.g[V0][0] != 0;
        if main {
            c.cycles += 1;
        } else {
            // |x| < 2**-28: x itself unless it does not truncate to 0.
            c.cycles += 5;
            c.cvt_w_s(0, 12);
            c.mfc1(V0, 0);
            main = c.g[V0][0] != 0;
        }
        c.g[V0][0] = 0x3f2c_0000;
        if !main {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] |= c.g[A1][0];
            let zero_neg = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if zero_neg {
                // x == 0 and iy == -1: 1/|x|.
                c.cycles += 9;
                FABSF.call(c, 0x0040_f600)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.div_s(0, 1, 0);
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                let tan = c.g[A0][0] == c.g[V0][0];
                frame.restore_ra(c, 0);
                if tan {
                    c.cycles += 2;
                    c.mov_s(0, 12);
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 7;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 0);
                    c.div_s(0, 0, 12);
                }
            }
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }

        // z = x*x, after reducing x (and y) to pi/4 - x for |x| >= 0.6744
        // (0x3f2ca13f).
        c.cycles += 4;
        c.g[V0][0] |= 0xa13f;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.mul_s(14, 12, 12);
        } else {
            c.cycles += 2;
            if (c.g[A2][0] as i64) < 0 {
                c.cycles += 2;
                c.neg_s(12, 12);
                c.neg_s(13, 13);
            }
            c.cycles += 11;
            c.g[AT][0] = 0x3f49_0fda; // pio4
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3322_2168; // pio4lo
            c.mtc1(AT, 1);
            c.sub_s(14, 0, 12);
            c.sub_s(11, 1, 13);
            c.mtc1(0, 13);
            c.add_s(12, 14, 11);
            c.mul_s(14, 12, 12);
        }

        // r = y + z*(s*(r + v) + y) + T[0]*s*x; w = x + r, with r the odd
        // and v the even half of the polynomial in w = z*z.
        c.cycles += 50;
        c.g[V1][0] = 0x4a_0000;
        c.g[V0][0] = sx(TANF_T);
        c.lwc1(9, TANF_T);
        c.lwc1(4, TANF_T + 0x30);
        c.g[V1][0] = 0x3f2c_0000;
        c.mul_s(11, 14, 14);
        c.lwc1(5, TANF_T + 0x2c);
        c.lwc1(6, TANF_T + 0x28);
        c.mul_s(15, 14, 12);
        c.lwc1(2, TANF_T + 0x24);
        c.g[V1][0] |= 0xa13f;
        c.mul_s(4, 11, 4);
        c.lwc1(3, TANF_T + 0x20);
        c.mul_s(5, 11, 5);
        c.lwc1(0, TANF_T + 0x1c);
        c.lwc1(7, TANF_T + 0x18);
        c.mul_s(9, 9, 15);
        c.add_s(6, 6, 4);
        c.lwc1(1, TANF_T + 0x14);
        c.add_s(2, 2, 5);
        c.lwc1(4, TANF_T + 0x10);
        c.lwc1(5, TANF_T + 0xc);
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.mul_s(6, 11, 6);
        c.lwc1(8, TANF_T + 0x8);
        c.mul_s(2, 11, 2);
        c.lwc1(10, TANF_T + 0x4);
        c.add_s(3, 3, 6);
        c.add_s(0, 0, 2);
        c.mul_s(3, 11, 3);
        c.mul_s(0, 11, 0);
        c.add_s(7, 7, 3);
        c.add_s(1, 1, 0);
        c.mul_s(7, 11, 7);
        c.mul_s(1, 11, 1);
        c.add_s(4, 4, 7);
        c.add_s(5, 5, 1);
        c.mul_s(4, 11, 4);
        c.mul_s(5, 11, 5);
        c.add_s(8, 8, 4);
        c.add_s(3, 10, 5);
        c.mul_s(4, 14, 8);
        c.add_s(0, 3, 4);
        c.mul_s(0, 15, 0);
        c.add_s(0, 0, 13);
        c.mul_s(0, 14, 0);
        c.add_s(3, 13, 0);
        c.add_s(3, 3, 9);
        let reduced = c.g[V1][0] != 0;
        c.add_s(11, 12, 3);
        if reduced {
            tanf_unreduce(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let tan = c.g[A0][0] == c.g[V0][0];
            c.mov_s(0, 11);
            if !tan {
                tanf_neg_recip(c);
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_ApplyShake

/// `u32 Rand_NextMain()` (0x3177f0): the main random stream.
const RAND_NEXT_MAIN: u32 = 0x0031_77f0;

/// Camera fields of the shake.
mod shake {
    pub const POS: u32 = 0x20;
    pub const TARGET: u32 = 0x40;
    /// Seconds of shake left.
    pub const TIME_LEFT: u32 = 0x2b4;
    /// Last frame's "shake requested" argument.
    pub const LAST_REQUEST: u32 = 0x2ec;
}

/// One of the six shake channels (x/y/z, slow and fast): a random target
/// value is re-drawn each time its timer passes its period, and the output
/// eases from the previous to the current target over the period.
struct ShakeChannel {
    timer: u32,
    period: u32,
    prev: u32,
    cur: u32,
    /// FPU registers: amplitude, period range and period base.
    amp: usize,
    range: usize,
    base: usize,
    /// The block charge and the two `Rand_NextMain` return addresses.
    cycles: u64,
    rets: [u32; 2],
}

const SHAKE_CHANNELS: [ShakeChannel; 6] = [
    ShakeChannel {
        timer: 0x278,
        period: 0x254,
        prev: 0x26c,
        cur: 0x260,
        amp: 22,
        range: 26,
        base: 24,
        cycles: 29,
        rets: [0x0016_5a54, 0x0016_5a90],
    },
    ShakeChannel {
        timer: 0x27c,
        period: 0x258,
        prev: 0x270,
        cur: 0x264,
        amp: 22,
        range: 26,
        base: 24,
        cycles: 30,
        rets: [0x0016_5ae0, 0x0016_5b1c],
    },
    ShakeChannel {
        timer: 0x280,
        period: 0x25c,
        prev: 0x274,
        cur: 0x268,
        amp: 23,
        range: 26,
        base: 24,
        cycles: 29,
        rets: [0x0016_5b70, 0x0016_5bac],
    },
    ShakeChannel {
        timer: 0x2a8,
        period: 0x284,
        prev: 0x29c,
        cur: 0x290,
        amp: 22,
        range: 24,
        base: 25,
        cycles: 29,
        rets: [0x0016_5c0c, 0x0016_5c48],
    },
    ShakeChannel {
        timer: 0x2ac,
        period: 0x288,
        prev: 0x2a0,
        cur: 0x294,
        amp: 22,
        range: 24,
        base: 25,
        cycles: 30,
        rets: [0x0016_5c98, 0x0016_5cd4],
    },
    ShakeChannel {
        timer: 0x2b0,
        period: 0x28c,
        prev: 0x2a4,
        cur: 0x298,
        amp: 23,
        range: 24,
        base: 25,
        cycles: 29,
        rets: [0x0016_5d20, 0x0016_5d5c],
    },
];

/// The channel's timer passed its period: new period `base + range *
/// (rand - 1)`, the current target becomes the previous one, new target
/// `-amp + 2 amp (rand - 1)`; the timer keeps the overshoot.
fn shake_retarget(c: &mut Cpu, ch: &ShakeChannel) -> Flow {
    let cam = r32(c, S2);
    c.cycles += ch.cycles;
    c.g[S0][0] = 0x7f_0000;
    call_ptr(c, RAND_NEXT_MAIN, ch.rets[0])?;
    c.g[S0][0] |= 0xffff;
    c.g[S1][0] = 0x3f80_0000;
    c.g[V0][0] &= c.g[S0][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 21);
    c.g[V0][0] |= c.g[S1][0];
    c.lwc1(1, cam.wrapping_add(ch.cur));
    c.mtc1(V0, 2);
    c.swc1(1, cam.wrapping_add(ch.prev));
    c.neg_s(20, ch.amp);
    c.sub_s(0, 2, 21);
    c.mul_s(0, ch.range, 0);
    c.add_s(0, ch.base, 0);
    c.swc1(0, cam.wrapping_add(ch.period));
    call_ptr(c, RAND_NEXT_MAIN, ch.rets[1])?;
    c.g[V0][0] &= c.g[S0][0];
    c.sub_s(2, ch.amp, 20);
    c.g[V0][0] |= c.g[S1][0];
    c.lwc1(1, cam.wrapping_add(ch.timer));
    c.mtc1(V0, 0);
    c.sub_s(21, 0, 21);
    c.lwc1(0, cam.wrapping_add(ch.period));
    c.mul_s(2, 2, 21);
    c.sub_s(1, 1, 0);
    c.add_s(20, 20, 2);
    c.swc1(1, cam.wrapping_add(ch.timer));
    c.swc1(20, cam.wrapping_add(ch.cur));
    Ok(())
}

/// The channel's ease weight into f`out`: 0 before the timer starts (f`zero`
/// = 0), 1 past the period, else smoothstep `3t^2 - 2t^3` of `t = timer /
/// period` capped at 1.
fn shake_ease(c: &mut Cpu, ch: &ShakeChannel, out: usize, zero: usize) {
    let cam = r32(c, S2);
    c.cycles += 6;
    c.mtc1(0, zero);
    c.lwc1(0, cam.wrapping_add(ch.timer));
    c.c_le_s(zero, 0);
    let started = c.fcc();
    c.lwc1(1, cam.wrapping_add(ch.period));
    if !started {
        c.cycles += 1;
        c.mov_s(out, zero);
        return;
    }
    c.cycles += 3;
    c.c_le_s(0, 1);
    if !c.fcc() {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, out);
        return;
    }
    c.cycles += 14;
    c.div_s(0, 0, 1);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(AT, 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.add_s(1, 0, 0);
    c.mul_s(0, 0, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(0, 0, 2);
    c.sub_s(0, 0, 1);
    c.c_lt_s(3, 0);
    if c.fcc() {
        c.mov_s(0, 3);
    }
    c.cycles += 2;
    c.mov_s(out, 0);
}

/// The distance from the camera to its target through VU0 (`|pos - target|`
/// into f0), with the difference spilled at `sp+tmp` and copied to
/// `sp+copy` as the original does.
fn shake_target_distance(c: &mut Cpu, tmp: u32, copy: u32, f1_from: Option<u32>) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    let cam = r32(c, S2);
    let s = sp(c);
    c.lqc2(3, cam.wrapping_add(shake::POS));
    c.lqc2(4, cam.wrapping_add(shake::TARGET));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(tmp));
    match f1_from {
        None => {
            c.g[AT][0] = 0x4316_0000; // 150.0
            c.mtc1(AT, 1);
        }
        Some(gp_off) => c.lwc1(1, r32(c, GP).wrapping_sub(gp_off)),
    }
    c.lq(V0, s.wrapping_add(tmp));
    c.sq(V0, s.wrapping_add(copy));
    c.lqc2(3, s.wrapping_add(copy));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
}

/// `void ChaseCameraAlgorithm_ApplyShake(Camera* cam, int request, int
/// scale_by_distance, float period_base f12, float period_range f13, float
/// amplitude f14)` (0x165938): camera shake. A new request (or the last one
/// ending) sets 2 s of shake; the time left runs down by the frame step.
/// Six channels (three axes at two speeds) re-draw random targets when
/// their timers pass their random periods and ease between targets; the
/// summed offset, scaled by half the time left (capped at 1), by 15 or a
/// distance-based factor (within 150 units of the target), moves the
/// camera position.
pub fn chase_camera_algorithm_apply_shake(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_fpr(c, 26, 0xb0);
        frame.save_fpr(c, 24, 0xa0);
        frame.save_fpr(c, 22, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 27, 0xb8);
        frame.save_fpr(c, 25, 0xa8);
        frame.save_fpr(c, 23, 0x98);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.mov_s(26, 13);
        c.mov_s(24, 12);
        c.mov_s(22, 14);
        let cam = r32(c, S2);
        let request = c.g[A1][0] != 0;

        // Time left: 0 while requested; 2 s when the request just ended.
        if request {
            c.cycles += 2;
            c.sw(cam.wrapping_add(shake::TIME_LEFT), 0);
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(cam.wrapping_add(shake::LAST_REQUEST));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(AT, 0);
                c.swc1(0, cam.wrapping_add(shake::TIME_LEFT));
                c.cycles += 1;
            }
        }
        c.lwc1(2, cam.wrapping_add(shake::TIME_LEFT));
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_lt_s(1, 2);
        let running = c.fcc();
        c.sw(cam.wrapping_add(shake::LAST_REQUEST), c.g[A1][0]);
        if running {
            c.cycles += 4;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x68d0));
            c.sub_s(0, 2, 0);
            c.max_s(0, 0, 1);
            c.swc1(0, cam.wrapping_add(shake::TIME_LEFT));
        }
        // f27: the fade factor, half the time left (1 while requested).
        c.cycles += 5;
        c.lwc1(0, cam.wrapping_add(shake::TIME_LEFT));
        c.c_lt_s(1, 0);
        let fading = c.fcc();
        c.mov_s(2, 0);
        if fading {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.mul_s(27, 2, 0);
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 27);
        }
        c.cycles += 2;
        if !request {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(0, 2);
            let left = c.fcc();
            frame.restore(c, S0, 0x70);
            if !left {
                return shake_epilogue(c, frame);
            }
        }

        // Advance the slow channels' timers by the frame step and re-draw
        // the channels whose timers passed their periods.
        c.cycles += 13;
        c.lwc1(1, r32(c, GP).wrapping_sub(0x68cc));
        c.lwc1(0, cam.wrapping_add(0x278));
        c.lwc1(3, cam.wrapping_add(0x27c));
        c.add_s(0, 0, 1);
        c.lwc1(2, cam.wrapping_add(0x280));
        c.lwc1(4, cam.wrapping_add(0x254));
        c.add_s(3, 3, 1);
        c.add_s(2, 2, 1);
        c.c_lt_s(4, 0);
        c.swc1(0, cam.wrapping_add(0x278));
        c.swc1(3, cam.wrapping_add(0x27c));
        let due = c.fcc();
        c.swc1(2, cam.wrapping_add(0x280));
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[0])?;
        }
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x27c));
        c.lwc1(0, cam.wrapping_add(0x258));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[1])?;
        }
        c.lwc1(0, cam.wrapping_add(0x280));
        // The z channel's amplitude is scaled by the constant at gp-0x68c8 (f23).
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x25c));
        c.lwc1(2, r32(c, GP).wrapping_sub(0x68c8));
        c.c_lt_s(1, 0);
        let due = c.fcc();
        c.mul_s(23, 22, 2);
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[2])?;
        }
        // The fast channels: half the period base, range and amplitudes.
        c.cycles += 10;
        c.lwc1(2, cam.wrapping_add(0x2a8));
        c.lwc1(1, cam.wrapping_add(0x284));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.c_lt_s(1, 2);
        c.mul_s(25, 24, 0);
        c.mul_s(22, 22, 0);
        c.mul_s(24, 26, 0);
        let due = c.fcc();
        c.mul_s(23, 23, 0);
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[3])?;
        }
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x2ac));
        c.lwc1(0, cam.wrapping_add(0x288));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[4])?;
        }
        c.lwc1(1, cam.wrapping_add(0x2b0));
        c.cycles += 4;
        c.lwc1(0, cam.wrapping_add(0x28c));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[5])?;
        }

        // Ease weights f20..f15 (zero registers f13, f11, f6, f4, f3, f2).
        for (ch, (out, zero)) in
            SHAKE_CHANNELS
                .iter()
                .zip([(20, 13), (19, 11), (18, 6), (17, 4), (16, 3), (15, 2)])
        {
            shake_ease(c, ch, out, zero);
        }

        // Offset = sum over both speeds of prev + w (cur - prev), as
        // (1 - w) prev + w cur (f13, f11, f6 = x, y, z).
        c.cycles += 42;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.lwc1(5, cam.wrapping_add(0x26c));
        c.sub_s(10, 0, 20);
        c.lwc1(2, cam.wrapping_add(0x270));
        c.sub_s(9, 0, 19);
        c.lwc1(1, cam.wrapping_add(0x274));
        c.sub_s(8, 0, 18);
        c.lwc1(13, cam.wrapping_add(0x260));
        c.sub_s(14, 0, 15);
        c.lwc1(11, cam.wrapping_add(0x264));
        c.sub_s(7, 0, 17);
        c.lwc1(6, cam.wrapping_add(0x268));
        c.mul_s(10, 10, 5);
        c.lwc1(4, cam.wrapping_add(0x290));
        c.mul_s(9, 9, 2);
        c.lwc1(12, cam.wrapping_add(0x29c));
        c.mul_s(8, 8, 1);
        c.lwc1(3, cam.wrapping_add(0x294));
        c.sub_s(0, 0, 16);
        c.lwc1(5, cam.wrapping_add(0x2a0));
        c.lwc1(2, cam.wrapping_add(0x298));
        c.mul_s(7, 7, 12);
        c.lwc1(1, cam.wrapping_add(0x2a4));
        c.mul_s(13, 20, 13);
        c.mul_s(0, 0, 5);
        c.mul_s(14, 14, 1);
        c.mul_s(11, 19, 11);
        c.mul_s(6, 18, 6);
        c.mul_s(4, 17, 4);
        c.mul_s(3, 16, 3);
        c.mul_s(2, 15, 2);
        c.add_s(13, 13, 10);
        c.add_s(11, 11, 9);
        c.add_s(6, 6, 8);
        c.add_s(4, 4, 7);
        c.add_s(3, 3, 0);
        c.add_s(2, 2, 14);
        c.add_s(13, 13, 4);
        c.add_s(11, 11, 3);
        let by_distance = c.g[S3][0] != 0;
        c.add_s(6, 6, 2);

        // Scale (f0): 15, or distance * k within 150 units of the target.
        let mut near = false;
        if by_distance {
            c.cycles += 21;
            shake_target_distance(c, 0x10, 0, None);
            c.c_le_s(0, 1);
            near = c.fcc();
            if near {
                c.cycles += 19;
                shake_target_distance(c, 0x20, 0x10, Some(0x68c4));
                c.mul_s(0, 0, 1);
            }
        }
        if !near {
            c.cycles += 2;
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(AT, 0);
        }

        // pos += offset * scale * fade.
        let s = sp(c);
        c.cycles += 17;
        c.swc1(13, s.wrapping_add(0x10));
        c.mul_s(0, 27, 0);
        c.swc1(11, s.wrapping_add(0x14));
        c.swc1(6, s.wrapping_add(0x18));
        c.sw(s.wrapping_add(0x1c), 0);
        c.mfc1(V1, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.lq(V0, s.wrapping_add(0x20));
        c.sq(V0, s);
        c.lqc2(3, cam.wrapping_add(shake::POS));
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam.wrapping_add(shake::POS));
        frame.restore(c, S0, 0x70);
        shake_epilogue(c, frame)
    })
}

fn shake_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore_ra(c, 0x30);
    for (f, off) in [
        (27, 0xb8),
        (26, 0xb0),
        (25, 0xa8),
        (24, 0xa0),
        (23, 0x98),
        (22, 0x90),
        (21, 0x88),
        (20, 0x80),
    ] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_DrawShadow

/// `BoaderShadow_ComputeProjection(shadow, centre, up, light_dir, bones,
/// 0)` (0x374d00).
const SHADOW_COMPUTE_PROJECTION: u32 = 0x0037_4d00;
/// `BoaderShadow_Submit(shadow)` (0x375890).
const SHADOW_SUBMIT: u32 = 0x0037_5890;
/// `RiderModel_RenderShadow(model, 3, &colour_a, &colour_b)` (0x310948).
const RIDER_MODEL_RENDER_SHADOW: u32 = 0x0031_0948;
/// `int Rider_GetMainState(rider)` (0x11fe98).
const RIDER_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

/// Rider fields of the shadow.
mod rshadow {
    pub const MATRIX_UP: u32 = 0x1c0;
    pub const MODEL: u32 = 0x780;
    pub const BONE_A: u32 = 0x8e8;
    pub const BONE_B: u32 = 0x918;
    /// Set once the shadow projection was set up.
    pub const READY: u32 = 0xaa4;
    pub const SHADOW: u32 = 0xaa8;
    /// Model: the bone matrices (0x40 each, translation at +0x30).
    pub const MODEL_BONES: u32 = 0x30;
    /// Graphics manager: render-state stack pointer (0x14-byte entries)
    /// and the v-table.
    pub const GM_STATE_TOP: u32 = 0xe84;
    pub const GM_VTABLE: u32 = 0x10d8;
    /// Shadow: texture handle, projection matrix and its two corners.
    pub const SH_MATRIX: u32 = 0x110;
    pub const SH_MIN: u32 = 0x150;
    pub const SH_MAX: u32 = 0x160;
    pub const SH_TEXTURE: u32 = 0x170;
    /// The light direction (0, 0, -1), built once, and its flag.
    pub const LIGHT_DIR: u32 = 0x004a_5e40;
}

/// Call graphics-manager v-table slot `slot` (`{ i16 delta; fn }` at
/// `vt+slot`) with `this = gm + delta`, `gm` in register `gm_reg` and the
/// v-table in `vt_reg`, the function word into `fn_reg`.
fn gm_vcall(c: &mut Cpu, gm_reg: usize, vt_reg: usize, fn_reg: usize, slot: u32, ret: u32) -> Flow {
    let vt = r32(c, vt_reg);
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[fn_reg][0] = c.lw(vt.wrapping_add(slot + 4));
    let f = r32(c, fn_reg);
    c.g[A0][0] = sx(r32(c, gm_reg).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

/// The once-only part of `Rider_DrawShadow`: push a render state, build the
/// shadow projection from the midpoint of two bones, the rider's up axis and
/// the light direction, set the shadow render state (blend, z-test), draw
/// the rider model's shadow pass with the shadow texture and matrix, then
/// pop the render state.
fn rider_draw_shadow_setup(c: &mut Cpu) -> Flow {
    use rshadow::*;
    use ssx3_ee_rt::vu0::{Src, X};
    let rider = r32(c, S0);
    let s = sp(c);
    c.cycles += 55;
    c.g[T0][0] = 1;
    c.sw(rider.wrapping_add(READY), c.g[T0][0]);
    c.g[S1][0] = sx(s.wrapping_add(0x20));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.g[A3][0] = 0x4a_0000;
    // Push: copy the top render state (0x14 bytes) one entry up.
    let gm = r32(c, A0);
    c.g[V1][0] = c.lw(gm.wrapping_add(GM_STATE_TOP));
    let top = r32(c, V1);
    c.ldl(V0, top.wrapping_add(0x7));
    c.ldr(V0, top);
    c.ldl(A1, top.wrapping_add(0xf));
    c.ldr(A1, top.wrapping_add(0x8));
    c.g[A2][0] = c.lw(top.wrapping_add(0x10));
    c.sdl(V0, top.wrapping_add(0x1b));
    c.sdr(V0, top.wrapping_add(0x14));
    c.sdl(A1, top.wrapping_add(0x23));
    c.sdr(A1, top.wrapping_add(0x1c));
    c.sw(top.wrapping_add(0x24), c.g[A2][0]);
    c.g[V0][0] = c.lw(gm.wrapping_add(GM_STATE_TOP));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    c.sw(gm.wrapping_add(GM_STATE_TOP), c.g[V0][0]);
    // centre = (bone_b.pos + bone_a.pos) * 0.5, at sp+0.
    c.g[A2][0] = c.lw(rider.wrapping_add(MODEL));
    let model = r32(c, A2);
    c.g[V0][0] = c.lw(rider.wrapping_add(BONE_B));
    c.g[V1][0] = c.lw(model.wrapping_add(MODEL_BONES));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    c.g[A1][0] = c.lw(rider.wrapping_add(BONE_A));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lq(A0, r32(c, V1).wrapping_add(0x30));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
    c.sq(A0, s.wrapping_add(0x30));
    c.g[V0][0] = c.lw(model.wrapping_add(MODEL_BONES));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.lq(V1, r32(c, V0).wrapping_add(0x30));
    c.sq(V1, s.wrapping_add(0x40));
    c.lqc2(3, s.wrapping_add(0x30));
    c.lqc2(4, s.wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x50));
    c.lq(V0, s.wrapping_add(0x50));
    c.sq(V0, s.wrapping_add(0x20));
    c.mfc1(V1, 0);
    c.lqc2(4, s.wrapping_add(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x50));
    c.lq(V0, s.wrapping_add(0x50));
    c.sq(V0, s.wrapping_add(0x10));
    c.lwc1(0, s.wrapping_add(0x10));
    c.lwc1(1, s.wrapping_add(0x14));
    c.lwc1(2, s.wrapping_add(0x18));
    c.swc1(0, s);
    c.swc1(1, s.wrapping_add(0x4));
    c.swc1(2, s.wrapping_add(0x8));
    // The light direction (0, 0, -1), once.
    let gp = r32(c, GP);
    c.g[V0][0] = c.lw(gp.wrapping_add(0x1b10));
    if c.g[V0][0] == 0 {
        c.cycles += 9;
        c.mtc1(0, 0);
        c.g[V0][0] = sx(LIGHT_DIR);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 1);
        c.swc1(0, LIGHT_DIR + 0x4);
        c.swc1(1, LIGHT_DIR + 0x8);
        c.sw(gp.wrapping_add(0x1b10), c.g[T0][0]);
        c.swc1(0, LIGHT_DIR);
    }
    c.lq(V0, rider.wrapping_add(MATRIX_UP));

    // The projection: up = the rider matrix's third row (at sp+0x10).
    c.cycles += 91;
    c.g[A1][0] = c.g[SP][0];
    c.g[V1][0] = c.lw(rider.wrapping_add(MODEL));
    c.g[A2][0] = sx(s.wrapping_add(0x10));
    c.sq(V0, s.wrapping_add(0x20));
    c.g[A3][0] = sx(LIGHT_DIR);
    c.g[T1][0] = 0;
    c.lwc1(20, gp.wrapping_sub(0x78c0));
    c.lwc1(0, s.wrapping_add(0x20));
    c.lwc1(1, s.wrapping_add(0x24));
    c.lwc1(2, s.wrapping_add(0x28));
    c.g[A0][0] = c.lw(rider.wrapping_add(SHADOW));
    c.swc1(0, s.wrapping_add(0x10));
    c.swc1(1, s.wrapping_add(0x14));
    c.swc1(2, s.wrapping_add(0x18));
    c.g[T0][0] = c.lw(r32(c, V1).wrapping_add(MODEL_BONES));
    call_ptr(c, SHADOW_COMPUTE_PROJECTION, 0x0012_29fc)?;

    // Shadow render state in the pushed entry: +8 bits 5-9 = 1; +4: bits
    // 23-24 = 2, bits 20-21 = 0, bits 12-19 = 0x14, bit 22 = 1, bits 2-6 = 1.
    c.g[A1][0] = c.lw(gp.wrapping_sub(0x854));
    let gm = r32(c, A1);
    c.g[T0][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = 0xffff_ffff_fe7f_0000;
    c.g[T3][0] = 0x0100_0000;
    c.g[A0][0] = c.lw(gm.wrapping_add(GM_STATE_TOP));
    c.g[V1][0] |= 0xffff;
    c.g[A2][0] = 0xffff_ffff_ffcf_0000;
    c.g[T1][0] = 0xffff_ffff_fff0_0000;
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x8));
    c.g[A2][0] |= 0xffff;
    c.g[T1][0] |= 0xfff;
    c.g[A3][0] = 0x1_0000;
    c.g[V0][0] &= c.g[T0][0];
    c.g[A3][0] |= 0x4000;
    c.g[V0][0] |= 0x20;
    c.g[T0][0] = 0xffff_ffff_ffbf_0000;
    c.sw(r32(c, A0).wrapping_add(0x8), c.g[V0][0]);
    c.g[T0][0] |= 0xffff;
    c.g[T2][0] = 0x40_0000;
    c.g[T4][0] = 0xffff_ffff_ffff_ff83;
    // (and, or) pairs applied to +4 of the top entry, alternating the
    // register the entry pointer is loaded into.
    let steps: [(usize, usize, Option<usize>); 5] = [
        (A0, V1, Some(T3)),
        (V1, A2, None),
        (A0, T1, Some(A3)),
        (V1, T0, Some(T2)),
        (A0, T4, None),
    ];
    for (i, &(ptr, and, or)) in steps.iter().enumerate() {
        c.g[ptr][0] = c.lw(gm.wrapping_add(GM_STATE_TOP));
        let e = r32(c, ptr);
        c.g[V0][0] = c.lw(e.wrapping_add(0x4));
        c.g[V0][0] &= c.g[and][0];
        match or {
            Some(r) => c.g[V0][0] |= c.g[r][0],
            None if i == 4 => c.g[V0][0] |= 0x4,
            None => {}
        }
        c.sw(e.wrapping_add(0x4), c.g[V0][0]);
    }
    c.g[V1][0] = c.lw(gm.wrapping_add(GM_VTABLE));
    gm_vcall(c, A1, V1, V0, 0xf8, 0x0012_2ac0)?;

    // Texture: the shadow's.
    c.g[A3][0] = c.lw(gp.wrapping_sub(0x854));
    c.g[A2][0] = 1;
    c.g[V0][0] = c.lw(rider.wrapping_add(SHADOW));
    c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(GM_VTABLE));
    c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(SH_TEXTURE));
    gm_vcall(c, A3, V1, V0, 0x1a8, 0x0012_2ae4)?;

    // Texture-unit setup with a zero vector at sp+0x20.
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x854));
    c.g[A2][0] = c.g[S1][0];
    c.g[V0][0] = c.lw(rider.wrapping_add(SHADOW));
    c.g[A3][0] = c.lw(r32(c, V1).wrapping_add(GM_VTABLE));
    c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(SH_TEXTURE));
    let vt = r32(c, A3);
    c.g[A0][0] = c.lh(vt.wrapping_add(0x1b8));
    c.sw(s.wrapping_add(0x20), 0);
    c.sw(s.wrapping_add(0x24), 0);
    c.sw(s.wrapping_add(0x28), 0);
    c.sw(s.wrapping_add(0x2c), 0);
    c.g[V0][0] = c.lw(vt.wrapping_add(0x1bc));
    let f = r32(c, V0);
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, 0x0012_2b18)?;

    // Darkness: 0.3 (gp-0x78c0) unless the rider has no main state.
    c.g[A0][0] = c.g[S0][0];
    RIDER_GET_MAIN_STATE.call(c, 0x0012_2b20)?;
    let has_state = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(rider.wrapping_add(SHADOW));
    if !has_state {
        c.cycles += 1;
        c.mtc1(0, 20);
    }

    // Projection matrix (copied to sp+0x20) and corners to the manager.
    c.cycles += 55;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SH_MATRIX));
    let m = r32(c, V0);
    let dst = r32(c, S1);
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, m.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, dst.wrapping_add(0x10 * i));
    }
    c.g[A0][0] = c.lw(rider.wrapping_add(SHADOW));
    c.g[A2][0] = sx(s.wrapping_add(0x60));
    c.g[T0][0] = c.lw(gp.wrapping_sub(0x854));
    c.g[A3][0] = sx(s.wrapping_add(0x70));
    let shadow = r32(c, A0);
    c.lq(V1, shadow.wrapping_add(SH_MIN));
    c.g[A1][0] = c.g[S1][0];
    c.sq(V1, s.wrapping_add(0x60));
    c.lq(V0, shadow.wrapping_add(SH_MAX));
    c.sq(V0, s.wrapping_add(0x70));
    c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(GM_VTABLE));
    gm_vcall(c, T0, V1, V0, 0x328, 0x0012_2b88)?;

    // The model's shadow pass with colours (1, k, k, k) and (1, c, c, c).
    c.lwc1(0, gp.wrapping_sub(0x78bc));
    c.g[A1][0] = 3;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.g[A2][0] = sx(s.wrapping_add(0x80));
    c.g[A0][0] = c.lw(rider.wrapping_add(MODEL));
    c.g[A3][0] = sx(s.wrapping_add(0x90));
    c.swc1(20, s.wrapping_add(0x8c));
    c.swc1(1, s.wrapping_add(0x90));
    c.swc1(0, s.wrapping_add(0x9c));
    c.swc1(1, s.wrapping_add(0x80));
    c.swc1(20, s.wrapping_add(0x84));
    c.swc1(20, s.wrapping_add(0x88));
    c.swc1(0, s.wrapping_add(0x94));
    c.swc1(0, s.wrapping_add(0x98));
    call_ptr(c, RIDER_MODEL_RENDER_SHADOW, 0x0012_2bc8)?;

    c.g[A1][0] = c.lw(gp.wrapping_sub(0x854));
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(GM_VTABLE));
    gm_vcall(c, A1, V1, V0, 0x1b0, 0x0012_2be0)?;
    c.g[A1][0] = c.lw(gp.wrapping_sub(0x854));
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(GM_VTABLE));
    gm_vcall(c, A1, V0, V1, 0x100, 0x0012_2bf8)?;

    // Pop the render state.
    c.g[A0][0] = c.lw(gp.wrapping_sub(0x854));
    let gm = r32(c, A0);
    c.g[V0][0] = c.lw(gm.wrapping_add(GM_STATE_TOP));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    c.sw(gm.wrapping_add(GM_STATE_TOP), c.g[V0][0]);
    Ok(())
}

/// `void Rider_DrawShadow(Rider* rider)` (0x122898): set up the rider's
/// projected ("boarder") shadow the first time, then submit it for drawing.
pub fn rider_draw_shadow(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 20, 0xd0);
        let rider = r32(c, S0);
        c.g[V0][0] = c.lw(rider.wrapping_add(rshadow::READY));
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        if c.g[V0][0] == 0 {
            rider_draw_shadow_setup(c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.lw(rider.wrapping_add(rshadow::SHADOW));
        call_ptr(c, SHADOW_SUBMIT, 0x0012_2c10)?;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Instance_GetNodeMatrix

/// Copy a 4x4 matrix through vf1-vf4 (`lqc2` all four rows, then `sqc2`).
fn vu0_copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, from.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, to.wrapping_add(0x10 * i));
    }
}

/// `out = b * a` row by row on VU0 (vf4-vf7 = a, vf8-vf11 = b, result in
/// vf12-vf15): each row of b combines a's rows by its x, y, z, w.
fn vu0_mul_matrix(c: &mut Cpu, a: u32, b: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, a.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, b.wrapping_add(0x10 * i));
    }
    for i in 0..4u8 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + i, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + i, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + i, Z));
        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(8 + i, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, out.wrapping_add(0x10 * i));
    }
}

/// Instance / skeleton fields.
mod inode {
    pub const OWNER: u32 = 0xc;
    pub const WORLD: u32 = 0x10;
    pub const SKELETON: u32 = 0x80;
    pub const SCALE: u32 = 0x84;
    /// Skeleton: node table (0x10-byte entries `{ int parent; ...; Mat*
    /// local at +0xc }`).
    pub const NODES: u32 = 0x8;
    /// Owner v-table slots: `HasNodeMatrix(node)`, `GetNodeMatrix(node)`,
    /// `GetWorldMatrix()`.
    pub const VT_HAS_NODE: u32 = 0xd0;
    pub const VT_NODE_MATRIX: u32 = 0xe8;
    pub const VT_WORLD: u32 = 0xc0;
}

/// Virtual call on the owner in a1 (v-table in v1): `{ i16 delta; fn }` at
/// `vt + slot`.
fn owner_vcall(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    let vt = r32(c, V1);
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V0][0] = c.lw(vt.wrapping_add(slot + 4));
    let f = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

/// `void Instance_GetNodeMatrix(Instance* inst, int node, Mat4* out)`
/// (0x34fed8): the world matrix of one skeleton node. An instance with an
/// owner object asks it (the owner's own node matrix when it has one, else
/// its world matrix stands in for the instance's). For node -1 the result
/// is the world matrix; otherwise the node's local matrices are chained up
/// to the root (translations scaled by the instance scale) and then
/// multiplied by the world matrix.
pub fn instance_get_node_matrix(c: &mut Cpu) {
    guest(c, |c| {
        use inode::*;
        c.cycles += 13;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S0, 0x150);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S3, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        let inst = r32(c, S1);
        let out = r32(c, S2);
        c.g[T1][0] = sx(inst.wrapping_add(WORLD));
        c.g[A1][0] = c.lw(inst.wrapping_add(OWNER));
        c.g[S3][0] = c.lw(inst.wrapping_add(SKELETON));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            let world_only = c.g[S0][0] == c.g[V0][0];
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(OWNER));
            if !world_only {
                c.cycles += 6;
                owner_vcall(c, VT_HAS_NODE, 0x0034_ff28)?;
                let has = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if has {
                    // The owner's own node matrix.
                    c.cycles += 16;
                    c.g[A2][0] = c.lw(inst.wrapping_add(OWNER));
                    let owner = r32(c, A2);
                    c.g[V1][0] = c.lw(owner.wrapping_add(OWNER));
                    let vt = r32(c, V1);
                    c.g[A0][0] = c.lh(vt.wrapping_add(VT_NODE_MATRIX));
                    c.g[V0][0] = c.lw(vt.wrapping_add(VT_NODE_MATRIX + 4));
                    let f = r32(c, V0);
                    c.g[A0][0] = sx(owner.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x0034_ff48)?;
                    vu0_copy_matrix(c, r32(c, V0), out);
                    frame.restore(c, S0, 0x150);
                    return node_matrix_epilogue(c, frame);
                }
                c.cycles += 2;
                c.g[A1][0] = c.lw(inst.wrapping_add(OWNER));
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(OWNER));
            }
            c.cycles += 5;
            owner_vcall(c, VT_WORLD, 0x0034_ff88)?;
            c.g[T1][0] = c.g[V0][0];
        }

        c.cycles += 2;
        c.g[T0][0] = c.lw(r32(c, S3).wrapping_add(NODES));
        if (c.g[S0][0] as i64) < 0 {
            c.cycles += 9;
            vu0_copy_matrix(c, r32(c, T1), out);
            frame.restore(c, S0, 0x150);
            return node_matrix_epilogue(c, frame);
        }

        // Start from the node's local matrix with its translation scaled by
        // (s, s, s, 1).
        let s = sp(c);
        c.cycles += 43;
        c.lwc1(0, inst.wrapping_add(SCALE));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.swc1(0, s.wrapping_add(0x8));
        c.swc1(1, s.wrapping_add(0xc));
        c.swc1(0, s);
        c.swc1(0, s.wrapping_add(0x4));
        let node = r32(c, V1);
        c.g[V0][0] = c.lw(node.wrapping_add(0xc));
        vu0_copy_matrix(c, r32(c, V0), out);
        c.lq(V0, out.wrapping_add(0x30));
        c.g[T3][0] = sx(s.wrapping_add(0x50));
        c.lwc1(0, s);
        c.sq(V0, s.wrapping_add(0x20));
        c.g[A0][0] = u64::MAX;
        c.lwc1(4, s.wrapping_add(0x4));
        c.lwc1(1, s.wrapping_add(0x20));
        c.lwc1(2, s.wrapping_add(0x24));
        c.mul_s(1, 1, 0);
        c.lwc1(3, s.wrapping_add(0x28));
        c.mul_s(2, 2, 4);
        c.lwc1(5, s.wrapping_add(0x8));
        c.lwc1(0, s.wrapping_add(0x2c));
        c.lwc1(4, s.wrapping_add(0xc));
        c.mul_s(3, 3, 5);
        c.swc1(1, s.wrapping_add(0x10));
        c.mul_s(0, 0, 4);
        c.swc1(2, s.wrapping_add(0x14));
        c.swc1(3, s.wrapping_add(0x18));
        c.swc1(0, s.wrapping_add(0x1c));
        c.lq(V0, s.wrapping_add(0x10));
        c.sq(V0, out.wrapping_add(0x30));
        c.g[S0][0] = c.lw(node);
        let root = c.g[S0][0] == c.g[A0][0];
        c.g[T5][0] = sx(s.wrapping_add(0x10));
        if !root {
            // Chain the parents: out = out * parent (scaled translation).
            c.cycles += 6;
            c.g[A3][0] = sx(s.wrapping_add(0x30));
            c.g[A2][0] = sx(s.wrapping_add(0xd0));
            c.g[T2][0] = sx(s.wrapping_add(0x70));
            c.g[T4][0] = u64::MAX;
            c.g[A1][0] = sx((c.g[S0][0] as u32) << 4);
            loop {
                c.cycles += 78;
                c.lwc1(6, s);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.lwc1(7, s.wrapping_add(0x4));
                c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(0xc));
                let m = r32(c, A0);
                c.lwc1(5, s.wrapping_add(0x8));
                c.lq(V1, m);
                c.lwc1(4, s.wrapping_add(0xc));
                c.sq(V1, s.wrapping_add(0xb0));
                c.sq(V1, s.wrapping_add(0x70));
                c.lq(V0, m.wrapping_add(0x10));
                c.sq(V0, s.wrapping_add(0xb0));
                c.sq(V0, s.wrapping_add(0x80));
                c.lq(V1, m.wrapping_add(0x20));
                c.sq(V1, s.wrapping_add(0xb0));
                c.sq(V1, s.wrapping_add(0x90));
                c.lq(V0, m.wrapping_add(0x30));
                c.sq(V0, s.wrapping_add(0xc0));
                c.lwc1(1, s.wrapping_add(0xc0));
                c.lwc1(2, s.wrapping_add(0xc4));
                c.lwc1(3, s.wrapping_add(0xc8));
                c.mul_s(1, 1, 6);
                c.lwc1(0, s.wrapping_add(0xcc));
                c.mul_s(2, 2, 7);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, s.wrapping_add(0xb0));
                c.swc1(2, s.wrapping_add(0xb4));
                c.swc1(3, s.wrapping_add(0xb8));
                c.swc1(0, s.wrapping_add(0xbc));
                c.lq(V0, s.wrapping_add(0xb0));
                c.sq(V0, s.wrapping_add(0xa0));
                vu0_mul_matrix(c, s.wrapping_add(0x70), out, s.wrapping_add(0xd0));
                vu0_copy_matrix(c, s.wrapping_add(0xd0), s.wrapping_add(0x30));
                vu0_copy_matrix(c, s.wrapping_add(0x30), out);
                c.g[S0][0] = c.lw(r32(c, A1));
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 4);
                if c.g[S0][0] == c.g[T4][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_0058);
                }
            }
        }
        // out = out * world.
        c.cycles += 47;
        c.g[V0][0] = c.g[T5][0];
        vu0_mul_matrix(c, r32(c, T1), out, s.wrapping_add(0x50));
        vu0_copy_matrix(c, s.wrapping_add(0x50), s.wrapping_add(0x10));
        vu0_copy_matrix(c, s.wrapping_add(0x10), out);
        frame.restore(c, S0, 0x150);
        node_matrix_epilogue(c, frame)
    })
}

fn node_matrix_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, S1, 0x140);
    frame.restore(c, S2, 0x130);
    frame.restore(c, S3, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderSphereTree_CollideTriangle

/// `bool VU0_TriangleIntersect(a, b, c, point)` (0x32b6a8).
const TRI_INTERSECT: Callee = Callee {
    addr: 0x0032_b6a8,
    f: ee::s10::VU0_TriangleIntersect,
};

/// `a - b` on VU0 (vf5 = vf3 - vf4), spilled to `tmp` and copied through v0
/// to `dst`.
fn vsub_spill(c: &mut Cpu, a: u32, b: u32, tmp: u32, dst: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp);
    c.lq(V0, tmp);
    c.sq(V0, dst);
}

/// The VU0 dot product of the vectors at `a` and `b` (xyz) into GPR `dst`
/// (`qmfc2` of vf4).
fn vdot(c: &mut Cpu, a: u32, b: u32, dst: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(dst, 4);
}

/// The VU0 length of the vector at `a` (`sqrt` into Q) into v1.
fn vlen(c: &mut Cpu, a: u32) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
}

/// The FPU registers of a sphere-vs-segment test: projection `t`, segment
/// length, squared centre distance, squared radius and a zero.
struct SegRegs {
    t: usize,
    len: usize,
    d2: usize,
    r2: usize,
    zero: usize,
}

const SEG_MAIN: SegRegs = SegRegs {
    t: 2,
    len: 4,
    d2: 5,
    r2: 3,
    zero: 1,
};
const SEG_KID: SegRegs = SegRegs {
    t: 1,
    len: 3,
    d2: 4,
    r2: 2,
    zero: 22,
};

/// The tail of a sphere-vs-segment test (f0 = dot(centre - start, edge)):
/// `t = f0 / len`; v0 = 1 when `t >= 0`, the centre is within the radius of
/// the line (`d2 - t^2 <= r2`) and the entry point `t - sqrt(r2 - (d2 -
/// t^2))` is not past the end.
fn seg_tail(c: &mut Cpu, r: &SegRegs) {
    c.div_s(r.t, 0, r.len);
    c.c_lt_s(r.t, r.zero);
    let behind = c.fcc();
    c.g[V0][0] = 0;
    if behind {
        return;
    }
    c.cycles += 5;
    c.mul_s(0, r.t, r.t);
    c.sub_s(0, r.d2, 0);
    c.c_lt_s(r.r2, 0);
    if c.fcc() {
        return;
    }
    c.cycles += 8;
    c.sub_s(0, r.r2, 0);
    c.g[V0][0] = 1;
    c.sqrt_s(0, 0);
    c.sub_s(0, r.t, 0);
    c.c_lt_s(r.len, 0);
    if c.fcc() {
        c.g[V0][0] = 0;
    }
}

/// Sphere vs the edge from vertex a (s2) to `end`; `cd` = centre - a in the
/// frame, the edge is spilled at `e_tmp`/`e`.
fn seg_from_a(c: &mut Cpu, end: u32, e_tmp: u32, e: u32, cd: u32, r: &SegRegs, main: bool) {
    c.lqc2(3, end);
    c.lqc2(4, r32(c, S2));
    c.vu0().sub(0b1111, 5, 3, ssx3_ee_rt::vu0::Src::Vec(4));
    c.sqc2(5, e_tmp);
    if main {
        c.mtc1(0, 1);
    }
    c.lq(V0, e_tmp);
    c.sq(V0, e);
    vlen(c, e);
    c.mtc1(V1, r.len);
    vdot(c, cd, e, V0);
    c.mtc1(V0, 0);
}

/// The hit: push = n * (r - dist) to `*out_push`, the contact point (at
/// sp+0) to `*out_point`, also kept at sp+0x10. `kid` selects the
/// registers the kid loop's copy uses.
fn sphere_tri_hit(c: &mut Cpu, kid: bool) {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let (push, point, scale, ptr0) = if kid {
        (V1, V0, V0, A0)
    } else {
        (A0, V1, V1, A1)
    };
    c.sub_s(0, 21, 20);
    if kid {
        c.mov_s(23, 20);
    } else {
        c.g[V0][0] = 1;
    }
    c.mfc1(scale, 0);
    c.lqc2(4, r32(c, S1));
    c.qmtc2(scale, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x20));
    c.lq(push, s.wrapping_add(0x20));
    c.g[ptr0][0] = c.lw(s.wrapping_add(0x70));
    c.lq(point, s);
    c.sq(push, r32(c, ptr0));
    c.g[A1][0] = c.lw(s.wrapping_add(0x74));
    c.sq(point, r32(c, A1));
    c.sq(push, s.wrapping_add(0x10));
}

/// Sphere-tree fields.
mod stree {
    pub const CENTRE: u32 = 0x10;
    pub const RADIUS: u32 = 0x20;
    pub const KID_MASK: u32 = 0x28;
    pub const KID_COUNT: u32 = 0x2c;
    /// Kids: 0x20 bytes each, `{ Vec4 pos; float radius; }`.
    pub const KIDS: u32 = 0x30;
}

/// One kid of the sphere tree against the triangle (s0 = the kid, fp = the
/// tree + 0x20 * index): when its plane distance is within its radius and
/// below the best so far and it touches the triangle (projected centre
/// inside, near vertex a, or near one of the three edges), it becomes the
/// hit.
fn sphere_tree_kid(c: &mut Cpu) -> Flow {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let kid = r32(c, S0);
    c.cycles += 21;
    vsub_spill(
        c,
        kid,
        r32(c, S2),
        s.wrapping_add(0x20),
        s.wrapping_add(0x10),
    );
    c.lwc1(21, kid.wrapping_add(0x10));
    vdot(c, r32(c, S1), s.wrapping_add(0x10), V1);
    c.mtc1(V1, 20);
    c.abs_s(0, 20);
    c.c_lt_s(0, 21);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(20, 23);
    if !c.fcc() {
        return Ok(());
    }

    // The projected centre inside the triangle?
    c.cycles += 24;
    c.mfc1(V0, 20);
    c.lqc2(4, r32(c, S1));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lwc1(12, r32(c, GP).wrapping_sub(0x2dd8));
    c.lq(V0, s.wrapping_add(0x30));
    c.g[S6][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S5][0];
    c.sq(V0, s.wrapping_add(0x20));
    c.g[A2][0] = c.g[S7][0];
    c.lqc2(3, kid);
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x30));
    c.g[A3][0] = c.g[SP][0];
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(T0, s.wrapping_add(0x80));
    c.sq(V0, s);
    c.sq(V0, s.wrapping_add(0x10));
    TRI_INTERSECT.call(c, 0x0032_a6b8)?;
    let inside = c.g[V0][0] != 0;
    c.lq(T0, s.wrapping_add(0x80));
    'touch: {
        if inside {
            c.cycles += 2;
            c.g[V1][0] = 1;
            break 'touch;
        }
        // Near vertex a?
        c.cycles += 20;
        c.lqc2(3, r32(c, FP).wrapping_add(0x30));
        c.lqc2(4, r32(c, S2));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s.wrapping_add(0x20));
        c.mul_s(2, 21, 21);
        c.lq(V0, s.wrapping_add(0x20));
        c.sq(V0, s.wrapping_add(0x10));
        vdot(c, s.wrapping_add(0x10), s.wrapping_add(0x10), V1);
        c.mtc1(V1, 4);
        c.c_lt_s(4, 2);
        c.g[V1][0] = 1;
        if c.fcc() {
            break 'touch;
        }
        // Near edge a-b, a-c?
        for end in [S5, S7] {
            c.cycles += 30;
            let e = r32(c, end);
            seg_from_a(
                c,
                e,
                s.wrapping_add(0x30),
                s.wrapping_add(0x20),
                s.wrapping_add(0x10),
                &SEG_KID,
                false,
            );
            seg_tail(c, &SEG_KID);
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            if end == S5 {
                c.g[V1][0] = 1;
            }
            if hit {
                if end == S7 {
                    c.cycles += 2;
                    c.g[V1][0] = 1;
                }
                break 'touch;
            }
        }
        // Near edge b-c?
        let r = &SEG_KID;
        c.cycles += 45;
        vsub_spill(
            c,
            r32(c, S6),
            r32(c, S5),
            s.wrapping_add(0x30),
            s.wrapping_add(0x20),
        );
        vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x20), V1);
        c.mtc1(V1, 4);
        vsub_spill(
            c,
            r32(c, S7),
            r32(c, S5),
            s.wrapping_add(0x40),
            s.wrapping_add(0x30),
        );
        vlen(c, s.wrapping_add(0x30));
        c.mtc1(V1, 3);
        vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x30), V0);
        c.mtc1(V0, 0);
        c.div_s(r.t, 0, r.len);
        c.c_lt_s(r.t, r.zero);
        let behind = c.fcc();
        c.g[V0][0] = 0;
        if !behind {
            c.cycles += 6;
            c.mul_s(0, 1, 1);
            c.sub_s(0, 4, 0);
            c.c_lt_s(2, 0);
            let off_line = c.fcc();
            c.g[V1][0] = c.g[T0][0];
            if off_line {
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0;
                }
                break 'touch;
            }
            c.cycles += 8;
            c.sub_s(0, 2, 0);
            c.g[V0][0] = 1;
            c.sqrt_s(0, 0);
            c.sub_s(0, 1, 0);
            c.c_lt_s(3, 0);
            if !c.fcc() {
                break 'touch;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // Missed: v1 = 1, cleared again since v0 is 0.
        c.cycles += 1;
        c.g[V1][0] = c.g[T0][0];
        c.cycles += 1;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 15;
        sphere_tri_hit(c, true);
    }
    Ok(())
}

/// `bool RiderSphereTree_CollideTriangle(SphereTree* tree, Vec4* a, Vec4* b,
/// Vec4* c, Vec4* n, Vec4* out_push, Vec4* out_point)` (0x32a1c0): the
/// rider's sphere tree against a triangle with normal n. The root sphere
/// must be within its radius of the triangle's plane and touch the
/// triangle (projected centre inside, near vertex a, or near an edge). With
/// no kids the root is the hit; otherwise every kid in the mask is tried
/// and the nearest touching one is the hit. On a hit the push-out
/// `n * (r - dist)` and the contact point are written.
pub fn rider_sphere_tree_collide_triangle(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x150);
        for (reg, off) in [
            (S2, 0x100),
            (S4, 0xe0),
            (S1, 0x110),
            (S5, 0xd0),
            (S7, 0xb0),
            (S0, 0x120),
            (S3, 0xf0),
            (S6, 0xc0),
            (FP, 0xa0),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x90);
        for (f, off) in [(23, 0x148), (22, 0x140), (21, 0x138), (20, 0x130)] {
            frame.save_fpr(c, f, off);
        }
        c.g[S2][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A0][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[S7][0] = c.g[A3][0];
        let tree = r32(c, S4);
        let s = sp(c);
        c.g[V1][0] = sx(tree.wrapping_add(stree::CENTRE));
        // dist = n . (centre - a), against the root radius (f21).
        vsub_spill(
            c,
            tree.wrapping_add(stree::CENTRE),
            r32(c, S2),
            s.wrapping_add(0x10),
            s,
        );
        c.lwc1(21, tree.wrapping_add(stree::RADIUS));
        vdot(c, r32(c, S1), s, V0);
        c.mtc1(V0, 20);
        c.mov_s(1, 20);
        c.sw(s.wrapping_add(0x70), c.g[T1][0]);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let off_plane = c.fcc();
        c.sw(s.wrapping_add(0x74), c.g[T2][0]);
        let mut touch = false;
        if !off_plane {
            // The centre projected onto the plane (at sp+0) inside the
            // triangle?
            c.cycles += 22;
            c.mfc1(V0, 1);
            c.lqc2(4, r32(c, S1));
            c.qmtc2(V0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s.wrapping_add(0x20));
            c.lwc1(12, r32(c, GP).wrapping_sub(0x2de0));
            c.lq(V0, s.wrapping_add(0x20));
            c.g[S0][0] = c.g[V1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S5][0];
            c.sq(V0, s.wrapping_add(0x10));
            c.g[A2][0] = c.g[S7][0];
            c.lqc2(3, tree.wrapping_add(stree::CENTRE));
            c.lqc2(4, s.wrapping_add(0x10));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0x20));
            c.g[A3][0] = c.g[SP][0];
            c.lq(V0, s.wrapping_add(0x20));
            c.sq(V0, s);
            TRI_INTERSECT.call(c, 0x0032_a2c0)?;
            'test: {
                let inside = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if inside {
                    break 'test;
                }
                // Near vertex a?
                c.cycles += 20;
                c.lqc2(3, tree.wrapping_add(stree::CENTRE));
                c.lqc2(4, r32(c, S2));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0x40));
                c.mul_s(3, 21, 21);
                c.lq(V0, s.wrapping_add(0x40));
                c.sq(V0, s.wrapping_add(0x30));
                vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x30), V1);
                c.mtc1(V1, 5);
                c.c_lt_s(5, 3);
                let near = c.fcc();
                c.g[V0][0] = 1;
                if near {
                    break 'test;
                }
                // Near edge a-b, a-c?
                for end in [S5, S7] {
                    c.cycles += 31;
                    let e = r32(c, end);
                    seg_from_a(
                        c,
                        e,
                        s.wrapping_add(0x50),
                        s.wrapping_add(0x40),
                        s.wrapping_add(0x30),
                        &SEG_MAIN,
                        true,
                    );
                    seg_tail(c, &SEG_MAIN);
                    c.cycles += 2;
                    let hit = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if hit {
                        break 'test;
                    }
                }
                // Near edge b-c?
                c.cycles += 46;
                c.lqc2(3, r32(c, S0));
                c.lqc2(4, r32(c, S5));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0x50));
                c.mtc1(0, 1);
                c.lq(V0, s.wrapping_add(0x50));
                c.sq(V0, s.wrapping_add(0x40));
                vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x40), V1);
                c.mtc1(V1, 5);
                vsub_spill(
                    c,
                    r32(c, S7),
                    r32(c, S5),
                    s.wrapping_add(0x60),
                    s.wrapping_add(0x50),
                );
                vlen(c, s.wrapping_add(0x50));
                c.mtc1(V1, 4);
                vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
                c.mtc1(V0, 0);
                seg_tail(c, &SEG_MAIN);
            }
            c.cycles += 2;
            touch = c.g[V0][0] != 0;
        }
        if !touch {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return sphere_tri_epilogue(c, frame);
        }

        c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_MASK));
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            // No kids: the root is the hit.
            c.cycles += 15;
            sphere_tri_hit(c, false);
            return sphere_tri_epilogue(c, frame);
        }
        c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_COUNT));

        // The kids: f23 = the best plane distance so far.
        c.cycles += 3;
        c.lwc1(23, r32(c, GP).wrapping_sub(0x2ddc));
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 5;
            c.mtc1(0, 22);
            c.g[T0][0] = 1;
            c.g[FP][0] = c.g[S4][0];
            c.g[S0][0] = sx(tree.wrapping_add(stree::KIDS));
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_MASK));
                c.g[V1][0] = sx((c.g[T0][0] as u32) << ((c.g[S3][0] as u32) & 31));
                c.g[V0][0] &= c.g[V1][0];
                if c.g[V0][0] != 0 {
                    sphere_tree_kid(c)?;
                }
                c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_COUNT));
                c.cycles += 5;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_a5e8);
                }
            }
        }
        // Hit when the best distance moved off its start value.
        c.cycles += 5;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x2dd4));
        c.c_eq_s(23, 0);
        let none = c.fcc();
        c.g[V0][0] = 1;
        if none {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        sphere_tri_epilogue(c, frame)
    })
}

fn sphere_tri_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 16;
    for (reg, off) in [
        (S0, 0x120),
        (S1, 0x110),
        (S2, 0x100),
        (S3, 0xf0),
        (S4, 0xe0),
        (S5, 0xd0),
        (S6, 0xc0),
        (S7, 0xb0),
        (FP, 0xa0),
    ] {
        frame.restore(c, reg, off);
    }
    frame.restore_ra(c, 0x90);
    for (f, off) in [(23, 0x148), (22, 0x140), (21, 0x138), (20, 0x130)] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_EeMixBlock

/// `SND_MixDeclick16(float* gain, short* buf)` (0x3c7600).
const SND_MIX_DECLICK16: Callee = Callee {
    addr: 0x003c_7600,
    f: ee::s14::SND_MixDeclick16,
};
/// `int SND_EeVoiceMixRamp(Voice*)` (0x3c7ab0): mixes the 16-sample ramp of
/// a voice whose gains changed; returns the samples done.
const SND_EE_VOICE_MIX_RAMP: Callee = Callee {
    addr: 0x003c_7ab0,
    f: ee::s14::SND_EeVoiceMixRamp,
};
/// `SND_EeVoiceFinish(int voice)` (0x3c8430).
const SND_EE_VOICE_FINISH: Callee = Callee {
    addr: 0x003c_8430,
    f: ee::s14::SND_EeVoiceFinish,
};
/// `memset` (0x416210).
const MEMSET: u32 = 0x0041_6210;

/// The EE software mixer's state at 0x515b40.
mod mixer {
    pub const BASE: u32 = 0x0051_5b40;
    pub const VOICE_COUNT: u32 = 0x4;
    pub const BUS_COUNT: u32 = 0x5;
    /// The declick bus enable byte.
    pub const DECLICK_ON: u32 = 0xf;
    pub const POST_MIX: u32 = 0x10;
    pub const SCRATCH: u32 = 0xb0;
    pub const SAMPLES: u32 = 0xb4;
    /// Bus buffers, then their declick gains at +0x18 from each.
    pub const BUS_BUF: u32 = 0xd0;
    pub const BUS_DECLICK: u32 = 0xe8;
    /// Per bus: an effect and the output stage (relative to BUS_BUF).
    pub const BUS_FX: u32 = 0x4c;
    pub const BUS_OUT: u32 = 0x34;
    pub const DECLICK_GAIN: u32 = 0x100;
    pub const VOICES: u32 = 0x1dc;
    pub const MIX_FN: u32 = 0x1e0;
    /// The declick bus buffer.
    pub const DECLICK_BUF: u32 = 0x0051_5d80;
    /// Voices: 0x60 bytes; state byte (2 = playing), "gains changed" byte,
    /// send gains at +0x1c (per bus) and +0x38 (declick), last sample at
    /// +0x3c, the source object at +0x40.
    pub const VOICE_SIZE: u64 = 0x60;
}

/// One playing voice (s1): finish its gain ramp, render the rest of the
/// block from its source and mix it into every bus it sends to (and the
/// declick bus).
fn mix_voice(c: &mut Cpu) -> Flow {
    use mixer::*;
    let voice = r32(c, S1);
    let s = sp(c);
    c.cycles += 3;
    c.g[V0][0] = c.lb(voice.wrapping_add(1));
    c.g[S6][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[A1][0] = c.g[FP][0];
    } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        SND_EE_VOICE_MIX_RAMP.call(c, 0x003c_8728)?;
        c.g[S6][0] = c.g[V0][0];
        c.sb(voice.wrapping_add(1), 0);
        c.g[A1][0] = sx(r32(c, FP).wrapping_sub(r32(c, S6)));
    }
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
    c.sw(s.wrapping_add(0x4), c.g[V0][0]);
    if c.g[A1][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    // Render: n = source->render(source, count, scratch, samples, 0).
    c.cycles += 11;
    c.g[A0][0] = c.lw(voice.wrapping_add(0x40));
    // (v1 keeps only the `lui` half of the mixer address here.)
    c.g[V1][0] = 0x51_0000;
    c.g[S3][0] = sx(BASE);
    c.g[T0][0] = 0;
    c.g[A2][0] = c.lw(BASE + SCRATCH);
    c.g[V0][0] = c.lw(r32(c, A0));
    let f = r32(c, V0);
    c.g[A3][0] = c.lw(BASE + SAMPLES);
    call_ptr(c, f, 0x003c_8768)?;
    c.g[S5][0] = c.g[V0][0];
    if (c.g[S5][0] as i64) < 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        SND_EE_VOICE_FINISH.call(c, 0x003c_877c)?;
        return Ok(());
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, S5) << 2);
    if c.g[S5][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    // Remember the last sample; mix into the buses with non-zero sends.
    c.cycles += 7;
    c.g[V1][0] = c.lw(BASE + SAMPLES);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.lwc1(0, r32(c, V0).wrapping_sub(4));
    c.swc1(0, voice.wrapping_add(0x3c));
    c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
    c.g[S2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.mtc1(0, 20);
        c.g[S7][0] = sx(r32(c, S6) << 2);
        c.g[S4][0] = sx(BASE + BUS_BUF);
        c.g[S0][0] = sx(voice.wrapping_add(0x1c));
        loop {
            c.cycles += 5;
            c.lwc1(12, r32(c, S0));
            c.c_eq_s(12, 20);
            if !c.fcc() {
                c.cycles += 7;
                c.g[A2][0] = c.lw(r32(c, S4));
                c.g[A0][0] = c.g[S5][0];
                c.g[V0][0] = c.lw(BASE + MIX_FN);
                c.g[A1][0] = c.lw(BASE + SAMPLES);
                let f = r32(c, V0);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, S7)));
                call_ptr(c, f, 0x003c_87e4)?;
            }
            c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
            c.cycles += 5;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_87b8);
            }
        }
    }
    c.cycles += 9;
    c.mtc1(0, 20);
    c.g[S3][0] = sx(BASE);
    c.g[S6][0] = sx(r32(c, S6) << 2);
    c.g[V0][0] = 0x51_0000;
    c.g[S7][0] = sx(BASE + DECLICK_ON);
    c.g[S4][0] = sx(DECLICK_BUF);
    c.g[S0][0] = sx(voice.wrapping_add(0x38));
    c.g[S2][0] = 0;
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S7)));
        c.g[V1][0] = c.lb(r32(c, V0));
        if c.g[V1][0] != 0 {
            c.cycles += 5;
            c.lwc1(12, r32(c, S0));
            c.c_eq_s(12, 20);
            if !c.fcc() {
                c.cycles += 7;
                c.g[V0][0] = c.lw(BASE + MIX_FN);
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.lw(BASE + SAMPLES);
                let f = r32(c, V0);
                c.g[A2][0] = sx(r32(c, S6).wrapping_add(DECLICK_BUF));
                call_ptr(c, f, 0x003c_8858)?;
                c.sw(DECLICK_BUF + 0x1800, 0);
            }
        }
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
        c.cycles += 2;
        let again = (c.g[S2][0] as i64) <= 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_8820);
        }
    }
    c.cycles += 1;
    Ok(())
}

/// `void SND_EeMixBlock(short** outs, int count)` (0x3c85d0): one block of
/// the EE software mixer: fade the declick bus, clear every bus buffer
/// (declicking it), mix each playing voice into its buses, run the post-mix
/// hook and send every bus through its effect (if any) to its output.
pub fn snd_ee_mix_block(c: &mut Cpu) {
    use mixer::*;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xc0);
        c.g[V1][0] = 0x51_0000;
        frame.save_fpr(c, 20, 0xb0);
        c.g[V0][0] = sx(BASE);
        for (reg, off) in [
            (S0, 0xa0),
            (S1, 0x90),
            (S2, 0x80),
            (S3, 0x70),
            (FP, 0x20),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx(BASE + DECLICK_GAIN);
        c.g[S1][0] = 0x51_0000;
        c.g[S2][0] = 0;
        c.g[S3][0] = sx(BASE + DECLICK_ON);
        c.g[FP][0] = c.g[A1][0];
        c.mtc1(0, 20);
        let s = sp(c);
        c.sw(s, c.g[A0][0]);

        // The declick bus fades (one bus).
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S3)));
            c.g[V1][0] = c.lb(r32(c, V0));
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.lwc1(0, r32(c, S0));
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(DECLICK_BUF);
                    SND_MIX_DECLICK16.call(c, 0x003c_8658)?;
                }
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.cycles += 2;
            let again = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_8628);
            }
        }

        // Clear every bus buffer (count samples) and declick it.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = sx(BASE);
        c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[S1][0] = c.g[V1][0];
            c.mtc1(0, 20);
            c.g[S0][0] = sx(BASE + BUS_DECLICK);
            loop {
                c.cycles += 9;
                let gain = r32(c, S0);
                c.g[A0][0] = c.lw(gain.wrapping_sub(0x18));
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(r32(c, FP) << 2);
                call_ptr(c, MEMSET, 0x003c_8698)?;
                c.lwc1(0, gain);
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw(gain.wrapping_sub(0x18));
                    c.g[A0][0] = c.g[S0][0];
                    SND_MIX_DECLICK16.call(c, 0x003c_86b8)?;
                }
                c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(gain.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8688);
                }
            }
        }

        // The playing voices.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = sx(BASE);
        c.g[V0][0] = c.lbu(BASE + VOICE_COUNT);
        c.g[S0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = VOICE_SIZE;
            loop {
                c.cycles += 11;
                c.g[V0][0] = 0x51_0000;
                c.mult(0, V1, S0, V1);
                c.g[V0][0] = sx(BASE);
                c.g[A0][0] = c.lw(BASE + VOICES);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                c.sw(s.wrapping_add(0x4), c.g[V0][0]);
                c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = c.lb(r32(c, S1));
                let playing = c.g[V0][0] == 2;
                c.g[V1][0] = 0x51_0000;
                if playing {
                    mix_voice(c)?;
                    c.g[V1][0] = 0x51_0000;
                }
                c.cycles += 6;
                c.g[S0][0] = c.lw(s.wrapping_add(0x4));
                c.g[V1][0] = sx(BASE);
                c.g[V0][0] = c.lbu(BASE + VOICE_COUNT);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = VOICE_SIZE;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_86e8);
                }
            }
        }

        // The post-mix hook.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[S0][0] = sx(BASE);
        c.g[V0][0] = c.lw(BASE + POST_MIX);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            let f = r32(c, V0);
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, f, 0x003c_88a0)?;
        }
        c.g[V0][0] = c.lbu(BASE + BUS_COUNT);

        // Each bus through its effect (if any) to its output.
        c.cycles += 2;
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S3][0] = c.g[S0][0];
            c.g[S0][0] = c.lw(s);
            c.g[S1][0] = sx(BASE + BUS_BUF);
            loop {
                let bus = r32(c, S1);
                c.cycles += 3;
                c.g[V0][0] = c.lw(bus.wrapping_add(BUS_FX));
                c.g[A0][0] = c.g[V0][0];
                let ret = if c.g[V0][0] != 0 {
                    // fx->process(fx, count, bus, scratch, 0), then the output
                    // from the scratch buffer.
                    c.cycles += 15;
                    c.g[A2][0] = c.lw(bus);
                    c.g[A3][0] = c.lw(BASE + SCRATCH);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[V0][0] = c.lw(r32(c, V0));
                    let f = r32(c, V0);
                    c.g[T0][0] = 0;
                    call_ptr(c, f, 0x003c_88dc)?;
                    c.g[A0][0] = c.lw(bus.wrapping_add(BUS_OUT));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.lw(BASE + SCRATCH);
                    0x003c_88f8
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(bus.wrapping_add(BUS_OUT));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.lw(bus);
                    0x003c_891c
                };
                // out->process(out, count, src, outs[i], 0).
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(r32(c, A0));
                let f = r32(c, V0);
                c.g[A3][0] = c.lw(r32(c, S0));
                call_ptr(c, f, ret)?;
                c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(bus.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_88b8);
                }
            }
        }

        c.cycles += 13;
        for (reg, off) in [
            (S0, 0xa0),
            (S1, 0x90),
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_SelectTargetRider

/// `int AI_GetFrameCounter()` (0x1298c8).
const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::AI_GetFrameCounter,
};
/// `bool ComputerActorRider_IsRivalTarget(actor, index)` (0x10d9e8).
const IS_RIVAL_TARGET: Callee = Callee {
    addr: 0x0010_d9e8,
    f: ee::s00::ComputerActorRider_IsRivalTarget,
};
/// `float atanf(float)` (0x31c228) and `float cosf(float)` (0x31c040).
const ATANF: u32 = 0x0031_c228;
const COSF: u32 = 0x0031_c040;

/// Actor / rider-table fields of the target choice.
mod target {
    /// The chosen target rider (-1: none).
    pub const TARGET: u32 = 0xe70;
    /// The actor's AI view, starting with the table of the six riders
    /// (0x24 each: `{ Rider* rider; ?; float distance; float heading; ...
    /// }`).
    pub const TABLE: u32 = 0x18;
    pub const ENTRY_SIZE: u32 = 0x24;
    pub const ENTRY_DIST: u32 = 0x8;
    pub const ENTRY_HEADING: u32 = 0xc;
    /// Also in the AI view: the actor's velocity and its distance to the
    /// finish.
    pub const RIDER_VELOCITY: u32 = 0x1b0;
    pub const RIDER_FINISH_DIST: u32 = 0x4c8;
}

/// The target-rider score of table entry `s1`/index `s2` (f22 = its
/// distance): distance weighted by how far its heading is from the actor's
/// travel direction (`|cos|` + 1 for rivals, 1 - cos for others); the best
/// (smallest) score becomes the target and the rival flag goes to `*s4`.
fn score_target(c: &mut Cpu) -> Flow {
    use target::*;
    let actor = r32(c, S0);
    let gp = r32(c, GP);
    let s = sp(c);
    c.cycles += 15;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    IS_RIVAL_TARGET.call(c, 0x0010_be44)?;
    c.mtc1(0, 23);
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    let rider = r32(c, V0);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.sq(V1, s);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.lwc1(21, s.wrapping_add(0x4));
    c.sq(V1, s.wrapping_add(0x10));
    c.lwc1(20, s.wrapping_add(0x10));
    // f12 = the travel angle atan2-style from (x f20, y f21).
    c.c_eq_s(20, 23);
    let mut reload = true;
    if !c.fcc() {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, ATANF, 0x0010_beac)?;
        c.c_lt_s(20, 23);
        let negative_x = c.fcc();
        c.mov_s(12, 0);
        if negative_x {
            c.cycles += 4;
            c.c_lt_s(23, 21);
            if !c.fcc() {
                c.sub_s(0, 0, 25);
            } else {
                c.cycles += 1;
                c.add_s(0, 0, 25);
            }
            c.cycles += 1;
            c.mov_s(12, 0);
        }
    } else {
        c.cycles += 4;
        c.c_eq_s(21, 23);
        let zero = c.fcc();
        c.mov_s(12, 21);
        if !zero {
            // +-pi/2.
            c.cycles += 4;
            c.c_le_s(23, 21);
            c.lwc1(12, gp.wrapping_sub(0x7df4));
            let up = c.fcc();
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            if up {
                c.cycles += 3;
                c.lwc1(12, gp.wrapping_sub(0x7df0));
            }
            reload = false;
        }
    }
    if reload {
        c.cycles += 1;
        c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    }
    // The heading difference wrapped to a turn: d - round(d / 2pi) * 2pi.
    c.cycles += 14;
    c.lwc1(1, gp.wrapping_sub(0x7dec));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_HEADING));
    c.sub_s(0, 0, 12);
    c.mul_s(1, 0, 1);
    c.add_s(1, 1, 2);
    c.cvt_w_s(2, 1);
    c.cvt_s_w(2, 2);
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.sub_s(2, 2, 24);
    }
    c.cycles += 6;
    c.lwc1(12, gp.wrapping_sub(0x7de8));
    c.mul_s(12, 2, 12);
    c.sub_s(12, 0, 12);
    call_ptr(c, COSF, 0x0010_bf20)?;
    if c.g[S3][0] == 0 {
        c.sub_s(0, 24, 0);
    } else {
        c.cycles += 2;
        c.abs_s(0, 0);
        c.add_s(0, 0, 24);
    }
    c.cycles += 5;
    c.mul_s(22, 22, 0);
    c.c_le_s(22, 26);
    if c.fcc() {
        c.cycles += 4;
        c.sw(actor.wrapping_add(TARGET), c.g[S2][0]);
        c.mov_s(26, 22);
        c.sw(r32(c, S4), c.g[S3][0]);
    }
    Ok(())
}

/// `bool ComputerActorRider_SelectTargetRider(ComputerActor* actor, int*
/// rival)` (0x10bd10): keep the current target rider while it is within
/// range and the actor within 600 units of the finish; otherwise, every 12th
/// AI frame and while within 540 units of the finish, pick the best-scoring
/// rider within 700 units. Returns whether there is a target.
pub fn computer_actor_rider_select_target_rider(c: &mut Cpu) {
    use target::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S0][0] = c.g[A0][0];
        let actor = r32(c, S0);
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(actor.wrapping_add(TARGET));
        c.g[S4][0] = c.g[A1][0];
        'done: {
            if (c.g[V1][0] as i64) >= 0 {
                // Drop the target when it is beyond the range at gp-0x7e00
                // or the actor is more than 600 units from the finish.
                c.cycles += 10;
                c.g[V0][0] = ENTRY_SIZE as u64;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.mult(0, V0, V1, V0);
                c.lwc1(1, gp.wrapping_sub(0x7e00));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_DIST));
                c.c_lt_s(1, 0);
                let mut drop = c.fcc();
                c.g[V0][0] = u64::MAX;
                if !drop {
                    c.cycles += 7;
                    c.lwc1(1, r32(c, A0).wrapping_add(RIDER_FINISH_DIST));
                    c.g[AT][0] = 0x4416_0000; // 600.0
                    c.mtc1(AT, 0);
                    c.c_lt_s(0, 1);
                    drop = c.fcc();
                }
                if drop {
                    c.cycles += 2;
                    c.sw(actor.wrapping_add(TARGET), c.g[V0][0]);
                }
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                c.cycles += 2;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                    break 'done;
                }
            }
            // Choose only below 540 units to the finish, every 12th frame.
            c.cycles += 8;
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            c.g[AT][0] = 0x4407_0000; // 540.0
            c.mtc1(AT, 1);
            c.lwc1(0, r32(c, V0).wrapping_add(RIDER_FINISH_DIST));
            c.c_lt_s(0, 1);
            if !c.fcc() {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            AI_GET_FRAME_COUNTER.call(c, 0x0010_bdd4)?;
            c.g[V1][0] = 0xc;
            c.div(0, V0, V1);
            c.cycles += 3;
            c.g[A0][0] = c.hi[0];
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            c.lwc1(26, gp.wrapping_sub(0x7dfc));
            c.g[S2][0] = 0;
            c.lwc1(25, gp.wrapping_sub(0x7df8));
            c.g[S1][0] = 0;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 24);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                let entry = r32(c, V0);
                c.g[V1][0] = c.lw(entry);
                if c.g[V1][0] != 0 {
                    c.cycles += 7;
                    c.g[AT][0] = 0x442f_0000; // 700.0
                    c.mtc1(AT, 0);
                    c.lwc1(22, entry.wrapping_add(ENTRY_DIST));
                    c.c_lt_s(0, 22);
                    if !c.fcc() {
                        score_target(c)?;
                    }
                }
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(ENTRY_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_be08);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
        }
        // v0 = target >= 0.
        c.cycles += 17;
        c.g[V0][0] = !c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// Nash_CollideVolume: the query box is the point (1,1,1) and the
    /// classification margin is 0, so a level-0 cell at (0,0,0) is "inside"
    /// (collect), one at (1,1,1) is partly inside (recurse) and one at
    /// (5,5,5) is outside. Each of the eight cells gets a random kind and a
    /// tree or none; trees hold one instance, one object and empty subtrees
    /// (all callees are real and take their short paths).
    #[test]
    fn nash_collide_volume_matches() {
        check(ee::s11::f_00335d78__generated, nash_collide_volume, 200, &|c, seed| {
            let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x35d7);
            scramble(c, &mut r);
            c.pc = 0x0033_5d78;
            c.g[GP][0] = GAME_GP as u64;
            let belib = DATA + 0x3_0000;
            let world = DATA + 0x3_1000;
            c.bus.write32(gp_at(-0x1f04), belib);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, world + 0x100);
            c.bus.write32(world + 0x100 + 0x7c, 0);
            // Classification margin (SpatialKey_ClassifyAABB) and the
            // recursion's 0.2 constants.
            c.bus.write32(gp_at(-0x2dfc), 0);
            for k in 0..16 {
                c.bus.write32(gp_at(-0x2d10 + 4 * k), f(0.2));
            }

            let grid = DATA;
            let q = DATA + 0x400;
            for k in 0..4 {
                c.bus
                    .write32(q + 0x20 + 4 * k, if k < 3 { f(1.0) } else { r.next_u32() });
                c.bus
                    .write32(q + 0x30 + 4 * k, if k < 3 { f(1.0) } else { r.next_u32() });
            }
            c.bus.write32(grid + 0xa4, r.next_u32());
            // One instance (not collidable) and one object, shared.
            let inst = DATA + 0x3000;
            let obj = DATA + 0x3100;
            c.bus.write32(inst, 0);
            c.bus.write16(inst + 0xa, 0);
            c.bus.write32(obj, 0);
            c.bus.write32(obj + 0x88, 0);
            let empty = DATA + 0x3200; // an all-zero tree node
            for k in 0..8u32 {
                let cell = grid + 0x14 * k;
                let xyz = [0u32, 1, 5][(r.next_u32() % 3) as usize];
                c.bus.write32(cell, 0);
                for a in 1..4 {
                    c.bus.write32(cell + 4 * a, xyz);
                }
                let tree = DATA + 0x1000 + 0x100 * k;
                let present = (seed + k) % 5 != 0;
                c.bus.write32(cell + 0x10, if present { tree } else { 0 });
                let bits = r.next_u32();
                c.bus
                    .write32(tree + 0x24, if bits & 1 != 0 { inst } else { 0 });
                c.bus
                    .write32(tree + 0x20, if bits & 2 != 0 { obj } else { 0 });
                for i in 0..8 {
                    let child = bits >> (2 + i) & 1 != 0;
                    c.bus.write32(tree + 4 * i, if child { empty } else { 0 });
                }
            }
            c.g[A0][0] = sx(grid);
            c.g[A1][0] = sx(q);
            c.g[A2][0] = sx(DATA + 0x5000);
            c.g[A3][0] = r.next_u32() as u64 % 8;
            c.g[T0][0] = r.next_u32() as u64;
        });
    }

    /// Rider_UpdateTweakLayerAnim: the rider at DATA with its controller
    /// (layer ids at +0/+4, sequence lists at +0x50, v-table at +0x58 whose
    /// play slot is a fake answering "no animation" (0x207), so
    /// `AnimController_PlayAnim` returns at once). Layer 1 plays a left /
    /// right / other / no animation whose table state is "both" (3), "one"
    /// (0xd) or neither; layer 0 is one-handed or not; layer 1's sequence
    /// flags and the two inputs are random, so every settle, early-return
    /// and start path is taken.
    #[test]
    fn rider_update_tweak_layer_anim_matches() {
        covered(
            ee::s00::f_001163b0__generated,
            rider_update_tweak_layer_anim,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1163b0, 3);
                c.pc = 0x0011_63b0;
                let rider = DATA;
                let ctrl = DATA + 0x4000;
                let lists = DATA + 0x4100;
                let seq0 = DATA + 0x4200;
                let seq1 = DATA + 0x4400;
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                for i in 0..8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * i, 0x207);
                }
                c.bus.write32(rider + 0x784, ctrl);
                c.bus.write32(ctrl + 0x58, vt);
                c.bus.write32(ctrl + 0x50, lists);
                c.bus.write32(lists + 4, seq0);
                c.bus.write32(lists + 0xc, seq1);
                for seq in [seq0, seq1] {
                    for off in [0x90, 0x94, 0x98, 0x9c] {
                        c.bus.write32(seq + off, r.edge_f32());
                    }
                }
                if seed % 5 == 0 {
                    c.bus.write32(seq1 + 0x9c, 0);
                }
                c.bus.write64(seq1 + 0xb0, (r.next_u32() & 7) as u64);
                c.bus.write64(seq1 + 0xb8, (r.next_u32() & 7) as u64);
                // Layer ids and the states the animation table gives them.
                let table = 0x0044_6990u32;
                let id1 = [0x143u32, 0x144, 0x10, 0x1b6][(r.next_u32() % 4) as usize];
                let id0 = [0x20u32, 0x1b6][(r.next_u32() % 2) as usize];
                c.bus.write32(ctrl + 4, id1);
                c.bus.write32(ctrl, id0);
                let state1 = [3u32, 0xd, 5][(r.next_u32() % 3) as usize];
                let state0 = [0xdu32, 2][(r.next_u32() % 2) as usize];
                c.bus.write32(table + id1 * 0x1c, state1);
                c.bus.write32(table + id0 * 0x1c, state0);
                // Rider state.
                c.bus.write32(rider + 0x320, r.next_u32() % 2);
                c.bus.write32(rider + 0x300, r.f32(4.0));
                let blend = [0.5f32, 1.0, 1.5, -1.0, 0.995][(r.next_u32() % 5) as usize];
                c.bus.write32(rider + 0x350, blend.to_bits());
                put_vec(
                    c,
                    rider + 0x1a0,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.next_u32()],
                );
                c.bus.write64(
                    rider + 0x8c0,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                c.bus.write32(GAME_GP - 0x7b24, r.f32(1.0));
                c.bus.write32(GAME_GP - 0x7b20, 0.011_111_112f32.to_bits());
                c.bus.write32(GAME_GP - 0x7b1c, r.f32(1.0));
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.g[A2][0] = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// __kernel_tanf: x in f12 from tiny (zero, 1e-10, values that do not
    /// truncate to 0 cannot be tiny, so also NaN/inf edges), the middle range
    /// and the reduced range (|x| >= 0.6744, both signs), y in f13, and iy
    /// 1, -1 or other; the real coefficient table is written.
    #[test]
    fn kernel_tanf_matches() {
        check(ee::s15::f_0040f5a8__generated, kernel_tanf, 300, &|c, seed| {
            let mut r = Rng(0x40f5_a8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0040_f5a8;
            let coeffs = [
                3.333_333_4e-1f32,
                1.333_340_8e-1,
                5.396_875_4e-2,
                2.186_948_8e-2,
                8.863_239_5e-3,
                3.592_079_3e-3,
                1.456_209_9e-3,
                5.880_412_3e-4,
                2.464_631_3e-4,
                7.817_944_4e-5,
                7.140_725_2e-5,
                -1.855_863_9e-5,
                2.590_730_7e-5,
            ];
            for (i, k) in coeffs.iter().enumerate() {
                c.bus.write32(0x0049_9648 + 4 * i as u32, k.to_bits());
            }
            let x = match seed % 8 {
                0 => 0.0f32.to_bits() | (r.next_u32() & 0x8000_0000),
                1 => (1.0e-10f32 * if r.next_u32() % 2 == 0 { 1.0 } else { -1.0 }).to_bits(),
                2 => r.edge_f32(),
                3 | 4 => r.f32(0.6),
                _ => {
                    let v = 0.68 + (r.next_u32() % 1000) as f32 * 1.0e-4;
                    (if r.next_u32() % 2 == 0 { v } else { -v }).to_bits()
                }
            };
            c.f[12] = x;
            c.f[13] = r.f32(1.0e-8);
            c.g[A0][0] = match r.next_u32() % 3 {
                0 => 1,
                1 => u64::MAX,
                _ => r.next_u32() as u64,
            };
        });
    }

    /// ChaseCameraAlgorithm_ApplyShake: shake requested or not (with the
    /// last request set or not), time left around 0..2, every channel's
    /// timer before 0, inside or past its period (so re-draws through the
    /// real `Rand_NextMain` and all ease branches run), and the distance
    /// scale off, near or far (150 units).
    #[test]
    fn chase_camera_algorithm_apply_shake_matches() {
        check(
            ee::s01::f_00165938__generated,
            chase_camera_algorithm_apply_shake,
            300,
            &|c, seed| {
                let mut r = Rng(0x1659_38 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0016_5938;
                c.g[GP][0] = GAME_GP as u64;
                let cam = DATA;
                // The main random stream's state.
                for k in 0..6 {
                    c.bus.write32(0x004f_f018 + 4 * k, r.next_u32());
                }
                c.bus.write32(GAME_GP - 0x68d0, (1.0f32 / 60.0).to_bits());
                c.bus.write32(GAME_GP - 0x68cc, (1.0f32 / 60.0).to_bits());
                c.bus.write32(GAME_GP - 0x68c8, 0.5f32.to_bits());
                c.bus.write32(GAME_GP - 0x68c4, 0.1f32.to_bits());
                c.bus.write32(cam + 0x2ec, r.next_u32() % 2);
                let left = [0.0f32, 0.01, 0.5, 1.9, 2.5][(r.next_u32() % 5) as usize];
                c.bus.write32(cam + 0x2b4, left.to_bits());
                for i in 0..6u32 {
                    let (timer, period) = [
                        (0x278, 0x254),
                        (0x27c, 0x258),
                        (0x280, 0x25c),
                        (0x2a8, 0x284),
                        (0x2ac, 0x288),
                        (0x2b0, 0x28c),
                    ][i as usize];
                    let p = 0.2 + (r.next_u32() % 100) as f32 * 0.01;
                    c.bus.write32(cam + period, p.to_bits());
                    let t = match r.next_u32() % 4 {
                        0 => -0.1f32,
                        1 => p * 0.5,
                        2 => p + 0.3,
                        _ => p - 0.005,
                    };
                    c.bus.write32(cam + timer, t.to_bits());
                }
                for off in (0x260..0x26c).step_by(4).chain((0x26c..0x278).step_by(4)) {
                    c.bus.write32(cam + off, r.f32(3.0));
                }
                for off in (0x290..0x2a8).step_by(4) {
                    c.bus.write32(cam + off, r.f32(3.0));
                }
                put_vec(
                    c,
                    cam + 0x20,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                );
                let far = if r.next_u32() % 2 == 0 { 50.0 } else { 400.0 };
                put_vec(
                    c,
                    cam + 0x40,
                    [r.f32(far), r.f32(far), r.f32(far), r.f32(1.0)],
                );
                c.g[A0][0] = sx(cam);
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64 | 1
                } else {
                    0
                };
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { 1 };
                c.f[12] = 0.5f32.to_bits();
                c.f[13] = 1.0f32.to_bits();
                c.f[14] = r.f32(2.0);
            },
        );
    }

    /// Rider_DrawShadow: the shadow set up already (submit only) or not
    /// (the whole setup; the light direction built or not, the rider with
    /// or without a main state). The graphics manager (gp-0x854 and
    /// gp+0x2a90) has a fake v-table, so every manager call is logged and
    /// may unwind; the shadow projection and the model's shadow pass run
    /// for real on random bones (the model has no meshes).
    #[test]
    fn rider_draw_shadow_matches() {
        covered(ee::s00::f_00122898__generated, rider_draw_shadow, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x122898, 8);
            c.pc = 0x0012_2898;
            let rider = DATA;
            let gm = DATA + 0x8000;
            let stack = DATA + 0x9000;
            let shadow = DATA + 0xa000;
            let model = DATA + 0xb000;
            let bones = DATA + 0xc000;
            let state = DATA + 0xd000;
            let vt = fakes::vtable(
                c,
                &mut r,
                &[0xf8, 0x100, 0x1a8, 0x1b0, 0x1b8, 0x218, 0x320, 0x328, 0x378],
            );
            c.bus.write32(GAME_GP - 0x854, gm);
            c.bus.write32(GAME_GP + 0x2a90, gm);
            c.bus.write32(gm + 0x10d8, vt);
            c.bus.write32(gm + 0xe84, stack);
            for k in 0..5 {
                c.bus.write32(stack + 4 * k, r.next_u32());
            }
            c.bus
                .write32(rider + 0xaa4, if seed % 3 == 0 { 1 } else { 0 });
            c.bus.write32(rider + 0xaa8, shadow);
            c.bus.write32(rider + 0x780, model);
            c.bus.write32(rider + 0x77c, state);
            c.bus.write32(state + 0xde0, r.next_u32() % 2);
            c.bus.write32(rider + 0x8e8, r.next_u32() % 4);
            c.bus.write32(rider + 0x918, r.next_u32() % 4);
            put_vec(
                c,
                rider + 0x1c0,
                [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
            );
            c.bus.write32(model + 0x30, bones);
            for k in 0..0x40 {
                c.bus.write32(bones + 4 * k, r.f32(10.0));
            }
            for k in 0..0x5c {
                c.bus.write32(shadow + 4 * k, r.f32(10.0));
            }
            c.bus.write32(shadow + 0x170, r.next_u32());
            c.bus.write32(GAME_GP + 0x1b10, r.next_u32() % 2);
            c.bus.write32(GAME_GP - 0x78c0, 0.3f32.to_bits());
            c.bus.write32(GAME_GP - 0x78bc, 0.3f32.to_bits());
            c.g[A0][0] = sx(rider);
        });
    }

    /// Instance_GetNodeMatrix: no owner, or an owner (fake v-table) that
    /// has the node's matrix or not; node -1, the root node or a node with
    /// one or two parents; random matrices and scale.
    #[test]
    fn instance_get_node_matrix_matches() {
        covered(
            ee::s11::f_0034fed8__generated,
            instance_get_node_matrix,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fed8, 2);
                c.pc = 0x0034_fed8;
                let inst = DATA;
                let skel = DATA + 0x400;
                let nodes = DATA + 0x500;
                let mats = DATA + 0x1000;
                let out = DATA + 0x3000;
                let owner = DATA + 0x600;
                for k in 0..0x10 {
                    c.bus.write32(inst + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(inst + 0x84, r.f32(2.0));
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, nodes);
                // Nodes 0..3: 0 is the root, 1 -> 0, 2 -> 1, 3 -> 0.
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(nodes + 0x10 * n, parent);
                    c.bus.write32(nodes + 0x10 * n + 0xc, mats + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(mats + 4 * k, r.f32(4.0));
                }
                let with_owner = seed % 3 == 0;
                if with_owner {
                    let vt = fakes::vtable(c, &mut r, &[0xc0, 0xd0, 0xe8]);
                    c.bus.write32(inst + 0xc, owner);
                    c.bus.write32(owner + 0xc, vt);
                    // HasNodeMatrix: a fake answering non-zero, or the real
                    // `Rider_ReturnZero_25b8` (0x1125b8). The matrix calls answer
                    // matrix pointers.
                    if r.next_u32() % 2 == 0 {
                        c.bus.write32(vt + 0xd4, 0x0011_25b8);
                        c.bus.write32(fakes::LOG_RESULTS, mats + 0x40);
                    } else {
                        c.bus.write32(fakes::LOG_RESULTS, mats + 0x80);
                        c.bus.write32(fakes::LOG_RESULTS + 4, mats + 0x40);
                    }
                }
                let node = match r.next_u32() % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = sx(node);
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// RiderSphereTree_CollideTriangle: the triangle (0,0,0), (4,0,0),
    /// (0,4,0) with normal +z; the root sphere and up to four kids at random
    /// places around it (above/below the plane, near the vertices and
    /// edges, or far), random masks. `VU0_TriangleIntersect` runs a stub
    /// microprogram at 0x6e0 (`vi2 = vi14`), so its answer is chosen per
    /// seed through vi14.
    #[test]
    fn rider_sphere_tree_collide_triangle_matches() {
        check(
            ee::s10::f_0032a1c0__generated,
            rider_sphere_tree_collide_triangle,
            300,
            &|c, seed| {
                let mut r = Rng(0x32a1_c0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x0032_a1c0;
                c.g[GP][0] = GAME_GP as u64;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .fallback = Some(ssx3_ps2::vu::reference::run_program);
                c.bus.write32(0x1100_0000 + 0x6e0, 0x8000_70b0); // IADD vi2, vi14, vi0
                c.bus.write32(0x1100_0000 + 0x6e4, 0x4000_02ff); // NOP, E
                c.g[14][0] = if seed % 4 == 0 { 1 } else { 0 };
                c.ctc2(14, 14);
                c.bus.write32(GAME_GP - 0x2de0, 0.001f32.to_bits());
                c.bus.write32(GAME_GP - 0x2dd8, 0.001f32.to_bits());
                c.bus.write32(GAME_GP - 0x2ddc, 1.0e10f32.to_bits());
                c.bus.write32(GAME_GP - 0x2dd4, 1.0e10f32.to_bits());
                let tree = DATA;
                let (a, b, cc, n) = (DATA + 0x400, DATA + 0x410, DATA + 0x420, DATA + 0x430);
                put_vec(c, a, [f(0.0), f(0.0), f(0.0), 0]);
                put_vec(c, b, [f(4.0), f(0.0), f(0.0), 0]);
                put_vec(c, cc, [f(0.0), f(4.0), f(0.0), 0]);
                put_vec(c, n, [f(0.0), f(0.0), f(1.0), 0]);
                let place = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    let p = match r.next_u32() % 6 {
                        0 => [-0.5, -0.5],
                        1 => [2.0, -0.6],
                        2 => [-0.6, 2.0],
                        3 => [2.3, 2.3],
                        4 => [1.0, 1.0],
                        _ => [9.0, 9.0],
                    };
                    let jitter = |r: &mut Rng| (r.next_u32() % 100) as f32 * 0.004 - 0.2;
                    let z = (r.next_u32() % 100) as f32 * 0.03 - 1.5;
                    put_vec(c, at, [f(p[0] + jitter(r)), f(p[1] + jitter(r)), f(z), 0]);
                    c.bus
                        .write32(at + 0x10, f(0.3 + (r.next_u32() % 100) as f32 * 0.012));
                };
                place(c, &mut r, tree + 0x10);
                let count = r.next_u32() % 5;
                for k in 0..count {
                    place(c, &mut r, tree + 0x30 + 0x20 * k);
                }
                c.bus.write32(
                    tree + 0x28,
                    if seed % 3 == 0 { 0 } else { r.next_u32() & 0xf },
                );
                c.bus.write32(tree + 0x2c, count);
                c.g[A0][0] = sx(tree);
                c.g[A1][0] = sx(a);
                c.g[A2][0] = sx(b);
                c.g[A3][0] = sx(cc);
                c.g[T0][0] = sx(n);
                c.g[T1][0] = sx(DATA + 0x500);
                c.g[T2][0] = sx(DATA + 0x510);
            },
        );
    }

    /// SND_EeMixBlock: the mixer at 0x515b40 with 0-2 buses (with or
    /// without an effect, declick gain zero or not), the declick bus on or
    /// off, 0-2 voices (playing or not, gains changed or not) whose sources
    /// render through a fake (small sample counts forced), return 0
    /// (`Rider_ReturnZero_25b8`) or -1 (`cFEStateRewardGallery_v26`, so the
    /// voice is finished), sends zero or not, and the post-mix hook set or
    /// not. All mix / effect / output functions are fakes.
    #[test]
    fn snd_ee_mix_block_matches() {
        covered(ee::s14::f_003c85d0__generated, snd_ee_mix_block, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3c85d0, 4);
            c.pc = 0x003c_85d0;
            let m = 0x0051_5b40u32;
            let fake = |k: u32| fakes::FAKE + 0x10 * k;
            for (i, v) in [2u32, 1, 3, 2, 1, 2, 3, 1].iter().enumerate() {
                c.bus.write32(fakes::LOG_RESULTS + 4 * i as u32, *v);
            }
            let objs = DATA + 0x6000;
            let obj = |c: &mut Cpu, k: u32, f: u32| -> u32 {
                let o = objs + 0x10 * k;
                c.bus.write32(o, f);
                o
            };
            let buses = r.next_u32() % 3;
            let voices = r.next_u32() % 3;
            let gain = |r: &mut Rng| {
                if r.next_u32() % 2 == 0 {
                    0
                } else {
                    0.5f32.to_bits()
                }
            };
            c.bus.write8(m + 4, voices as u8);
            c.bus.write8(m + 5, buses as u8);
            c.bus.write8(m + 0xf, (r.next_u32() % 2) as u8);
            c.bus.write32(m + 8, fake(7));
            c.bus.write32(m + 0x100, gain(&mut r));
            c.bus
                .write32(m + 0x10, if r.next_u32() % 2 == 0 { 0 } else { fake(6) });
            c.bus.write32(m + 0xb0, DATA + 0x2000);
            c.bus.write32(m + 0xb4, DATA + 0x3000);
            for k in 0..8 {
                c.bus.write32(DATA + 0x3000 + 4 * k, r.f32(1.0));
            }
            c.bus.write32(m + 0x1e0, fake(5));
            for i in 0..buses {
                c.bus.write32(m + 0xd0 + 4 * i, DATA + 0x1000 + 0x400 * i);
                c.bus.write32(m + 0xe8 + 4 * i, gain(&mut r));
                let fx = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    obj(c, i, fake(1))
                };
                c.bus.write32(m + 0x11c + 4 * i, fx);
                let out = obj(c, 2 + i, fake(2));
                c.bus.write32(m + 0x104 + 4 * i, out);
            }
            let table = DATA + 0x4000;
            c.bus.write32(m + 0x1dc, table);
            for v in 0..voices {
                let e = table + 0x60 * v;
                c.bus.write8(e, if r.next_u32() % 4 == 0 { 1 } else { 2 });
                let dirty = buses < 2 && v == 0 && r.next_u32() % 2 == 0;
                c.bus.write8(e + 1, dirty as u8);
                for k in 0..3 {
                    c.bus.write32(e + 0x1c + 4 * k, gain(&mut r));
                    c.bus.write32(e + 0x4 + 4 * k, gain(&mut r));
                }
                c.bus.write32(e + 0x38, gain(&mut r));
                let src = match r.next_u32() % 4 {
                    0 => 0x0011_25b8,
                    1 => 0x001d_faa0,
                    _ => fake(3),
                };
                let o = obj(c, 4 + v, src);
                c.bus.write32(e + 0x40, o);
            }
            let outs = DATA + 0x5000;
            for i in 0..2 {
                c.bus.write32(outs + 4 * i, DATA + 0x5100 + 0x200 * i);
            }
            c.g[A0][0] = sx(outs);
            c.g[A1][0] = [16u64, 32][(seed % 2) as usize];
        });
    }

    /// ComputerActorRider_SelectTargetRider: a current target (none, near,
    /// far) and the actor's distance to the finish (below/above 540 and
    /// 600), the AI frame counter (a multiple of 12 or not), six table
    /// entries (empty, near, beyond 700), the rival flags and the actor's
    /// travel direction (zero x and/or y, either sign) so every heading
    /// branch and the real `atanf`/`cosf` run.
    #[test]
    fn computer_actor_rider_select_target_rider_matches() {
        check(
            ee::s00::f_0010bd10__generated,
            computer_actor_rider_select_target_rider,
            300,
            &|c, seed| {
                let mut r = Rng(0x10bd_10 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.pc = 0x0010_bd10;
                c.g[GP][0] = GAME_GP as u64;
                let actor = DATA;
                let table = DATA + 0x400;
                let flag = DATA + 0x700;
                // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
                let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                c.bus.write32(GAME_GP - 0x848, a);
                c.bus.write32(a + 0x84, b);
                c.bus.write32(b + 0xc, d);
                c.bus.write32(
                    d + 8,
                    if seed % 3 == 0 {
                        7
                    } else {
                        12 * (r.next_u32() % 50)
                    },
                );
                // Rivals: IsRivalTarget reads the actor's table +0xf0 / +0xf8.
                c.bus.write32(table + 0xf0, r.next_u32() % 2);
                c.bus.write32(table + 0xf8, r.next_u32() % 6);
                for (off, v) in [
                    (0x7e00u32, 500.0f32),
                    (0x7dfc, 1.0e9),
                    (0x7df8, std::f32::consts::PI),
                    (0x7df4, -std::f32::consts::FRAC_PI_2),
                    (0x7df0, std::f32::consts::FRAC_PI_2),
                    (0x7dec, 1.0 / (2.0 * std::f32::consts::PI)),
                    (0x7de8, 2.0 * std::f32::consts::PI),
                ] {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                c.bus.write32(actor + 0x18, table);
                c.bus
                    .write32(actor + 0xe70, [u32::MAX, 1, 3][(r.next_u32() % 3) as usize]);
                for k in 0..6u32 {
                    let e = table + 0x24 * k;
                    c.bus.write32(
                        e,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            DATA + 0x900
                        },
                    );
                    let dist = [100.0f32, 650.0, 800.0][(r.next_u32() % 3) as usize];
                    c.bus.write32(e + 8, dist.to_bits());
                    c.bus.write32(e + 0xc, r.f32(4.0));
                }
                let finish = [300.0f32, 560.0, 700.0][(r.next_u32() % 3) as usize];
                c.bus.write32(table + 0x4c8, finish.to_bits());
                let comp = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    _ => r.f32(10.0),
                };
                put_vec(
                    c,
                    table + 0x1b0,
                    [comp(&mut r), comp(&mut r), r.next_u32(), r.next_u32()],
                );
                c.g[A0][0] = sx(actor);
                c.g[A1][0] = sx(flag);
            },
        );
    }
}
