//! Staging wave 206 (oc248): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// __adddf3 (0x411f18)

/// `double __adddf3(double a /*a0*/, double b /*a1*/)` (0x411f18): libgcc's
/// soft-float `a + b`: unpack both into the frame, add the parts, pack the
/// result (v0). Frame 0x90 (parts of a at 0, of b at 0x20, the sum at 0x40,
/// the packed arguments at 0x60 / 0x68).
pub fn adddf3(c: &mut Cpu) {
    use crate::game::lib::gcc::softfp::*;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        let sp0 = sp(c);
        c.sd(sp0 + A_PACKED, c.g[A0][0]);
        c.sd(sp0 + B_PACKED, c.g[A1][0]);
        c.g[A0][0] = sx(sp0 + A_PACKED);
        c.sd(sp0 + 0x70, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        UNPACK_D.call(c, 0x0041_1f38)?;
        let sp0 = sp(c);
        c.g[S0][0] = sx(sp0 + B_PARTS);
        c.g[A0][0] = sx(sp0 + B_PACKED);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_1f48)?;
        let sp0 = sp(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp0 + SUM_PARTS);
        c.g[A0][0] = c.g[SP][0];
        FPADD_PARTS_D.call(c, 0x0041_1f58)?;
        c.g[A0][0] = c.g[V0][0];
        PACK_D.call(c, 0x0041_1f60)?;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cDynamicSprayFX_QueueBurst (0x2e23e0)

/// The spray effect: its rider (+0), the queued burst's strength (+0xe0),
/// the queued position (+0xf0) and direction (+0x100), the burst's two
/// parameters (+0x110 / +0x114), the "short range" flag (+0x124).
const SPRAY_RIDER: u32 = 0x0;
const SPRAY_STRENGTH: u32 = 0xe0;
const SPRAY_POS: u32 = 0xf0;
const SPRAY_DIR: u32 = 0x100;
const SPRAY_PARAM_A: u32 = 0x110;
const SPRAY_PARAM_B: u32 = 0x114;
const SPRAY_SHORT_RANGE: u32 = 0x124;

/// The rider fields read for "carving": +0xac4 set, +0xad0 clear, +0xafc
/// set, then +0xb00 set.
const RIDER_CARVE_A: u32 = 0xac4;
const RIDER_CARVE_BLOCK: u32 = 0xad0;
const RIDER_CARVE_B: u32 = 0xafc;
const RIDER_CARVE_C: u32 = 0xb00;

/// `void cDynamicSprayFX_QueueBurst(cDynamicSprayFX* self /*a0*/,
/// sV4* pos /*a1*/, sV4* dir /*a2*/, int a /*a3*/, int b /*t0*/,
/// float speed /*f12*/)` (0x2e23e0): queue a snow-spray burst. A stronger
/// burst (|speed| above the queued strength) replaces the queued one when it
/// is far enough from the queued position (180, or 100 with the short-range
/// flag) or when the rider is carving; its strength is |speed| (doubled when
/// `b` is 0). Leaf, frame 0x20 (a scratch quadword for the distance).
pub fn dynamic_spray_queue_burst(c: &mut Cpu) {
    c.cycles += 24;
    let this = r32(c, A0);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let scratch = sp(c);
    // f1 = |speed|; distance = |queued pos - pos| through VU0.
    c.abs_s(1, 12);
    c.lqc2(3, this.wrapping_add(SPRAY_POS));
    c.lqc2(4, r32(c, A1));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, scratch.wrapping_add(0x10));
    c.g[AT][0] = 0x4334_0000; // 180.0
    c.mtc1(1, 3);
    c.lq(2, scratch.wrapping_add(0x10));
    c.g[V1][0] = c.lw(this.wrapping_add(SPRAY_SHORT_RANGE));
    c.sq(2, scratch);
    c.lqc2(3, scratch);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.abs_s(2, 0); // f2 = distance
    c.lwc1(0, this.wrapping_add(SPRAY_STRENGTH));
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        c.g[AT][0] = 0x42c8_0000; // 100.0
        c.mtc1(1, 3);
    }
    // f3 = range
    c.cycles += 3;
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.cycles += 4;
        c.c_lt_s(3, 2);
        let far = c.fcc();
        c.g[T2][0] = 0;
        let mut queue = far;
        if !far {
            // Carving test on the rider.
            c.cycles += 4;
            c.g[V1][0] = c.lw(this.wrapping_add(SPRAY_RIDER));
            let rider = r32(c, V1);
            c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_A));
            c.g[T1][0] = c.g[V1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_BLOCK));
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_B));
                    c.g[T2][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            if c.g[T2][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_C));
                queue = c.g[V0][0] != 0;
            }
        }
        if queue {
            c.cycles += 2;
            if c.g[T0][0] != 0 {
                c.swc1(1, this.wrapping_add(SPRAY_STRENGTH));
            } else {
                c.cycles += 2;
                c.add_s(0, 1, 1);
                c.swc1(0, this.wrapping_add(SPRAY_STRENGTH));
            }
            c.cycles += 6;
            c.lq(2, r32(c, A1));
            c.sq(2, this.wrapping_add(SPRAY_POS));
            c.lq(3, r32(c, A2));
            c.sw(this.wrapping_add(SPRAY_PARAM_B), c.g[T0][0]);
            c.sq(3, this.wrapping_add(SPRAY_DIR));
            c.sw(this.wrapping_add(SPRAY_PARAM_A), c.g[A3][0]);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    ret(c);
}

// ---------------------------------------------------------------------------
// ScriptEngine_PollAsyncLoads (0x278918)

/// The script engine: the number of async loads in flight (+0x620), the
/// filter file's async handle (+0x538), ten bank slots of 0x10 bytes whose
/// async handle is at +0x630 + 0x10 * i.
const SCRIPT_PENDING_LOADS: u32 = 0x620;
const SCRIPT_FILTER_HANDLE: u32 = 0x538;
const SCRIPT_BANK_HANDLES: u32 = 0x630;
const SCRIPT_BANK_SLOT_SIZE: u32 = 0x10;

/// `bool AsyncFile_IsDone(int handle)` (through the function table).
const ASYNC_FILE_IS_DONE: u32 = 0x003d_f980;

const SCRIPT_ON_FILTER_LOADED: Callee = Callee {
    addr: 0x0027_b6b0,
    f: ee::s07::ScriptEngine_OnFilterLoaded,
};

const SCRIPT_ON_BANK_LOADED: Callee = Callee {
    addr: 0x0027_b5d8,
    f: ee::s07::ScriptEngine_OnBankLoaded,
};

/// `void ScriptEngine_PollAsyncLoads(ScriptEngine* self /*a0*/)`
/// (0x278918): while loads are pending, hand every finished async load to
/// its handler: the filter file (`ScriptEngine_OnFilterLoaded`), then each
/// of the ten bank slots (`ScriptEngine_OnBankLoaded`). Frame 0x50 (ra at
/// 0, s3 / s2 / s1 / s0 at 0x10 / 0x20 / 0x30 / 0x40).
pub fn script_engine_poll_async_loads(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S2);
        c.g[V0][0] = c.lw(this.wrapping_add(SCRIPT_PENDING_LOADS));
        frame.restore(c, S0, 0x40);
        if c.g[V0][0] != 0 {
            // The filter file.
            c.cycles += 3;
            c.g[A0][0] = c.lw(this.wrapping_add(SCRIPT_FILTER_HANDLE));
            c.g[S1][0] = sx(this.wrapping_add(SCRIPT_BANK_HANDLES));
            let mut tail = true;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                call_ptr(c, ASYNC_FILE_IS_DONE, 0x0027_8954)?;
                c.g[S3][0] = 9;
                if c.g[V0][0] == 0 {
                    tail = false;
                } else {
                    c.cycles += 4;
                    let this = r32(c, S2);
                    c.g[A1][0] = c.lw(this.wrapping_add(SCRIPT_FILTER_HANDLE));
                    c.g[A0][0] = c.g[S2][0];
                    SCRIPT_ON_FILTER_LOADED.call(c, 0x0027_8968)?;
                    c.g[S1][0] = sx(r32(c, S2).wrapping_add(SCRIPT_BANK_HANDLES));
                }
            }
            if tail {
                c.cycles += 1;
                c.g[S3][0] = 9;
            }
            // The bank slots (s1: the slot's handle, s3: slots left - 1).
            loop {
                c.cycles += 3;
                c.g[S0][0] = c.lw(r32(c, S1));
                if c.g[S0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, ASYNC_FILE_IS_DONE, 0x0027_8984)?;
                    c.g[A1][0] = c.g[S0][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S2][0];
                        SCRIPT_ON_BANK_LOADED.call(c, 0x0027_8994)?;
                    }
                    c.cycles += 1;
                }
                c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                c.cycles += 2;
                let more = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(SCRIPT_BANK_SLOT_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8970);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Nash_ClassifyPatches (0x22a408)

const NASH_CLASSIFY_PATCH: Callee = Callee {
    addr: 0x0022_a128,
    f: ee::s05::Nash_ClassifyPatch,
};

/// A visible-list entry (8 bytes): the source (+0, whose first patch node
/// is at +0x24) and the VU frustum handed to the classifier (+4).
const VIS_SOURCE: u32 = 0x0;
const VIS_FRUSTUM: u32 = 0x4;
const VIS_ENTRY_SIZE: u32 = 0x8;
const SOURCE_FIRST_PATCH: u32 = 0x24;
/// A patch node's next link.
const PATCH_NEXT: u32 = 0x0;
/// The cell's pending list head (non-zero: one more classify with no node).
const CELL_LIST_HEAD: u32 = 0x78b0;

/// `void Nash_ClassifyPatches(Cell* cell /*a0*/, VisEntry* list /*a1*/,
/// int count /*a2*/)` (0x22a408): run `Nash_ClassifyPatch(cell, node,
/// entry->frustum)` on every patch node of every visible-list entry, then
/// once more with no node when the cell's list head is set. Frame 0x60 (ra
/// at 0, s4..s0 at 0x10..0x50).
pub fn nash_classify_patches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if (c.g[A2][0] as i64) > 0 {
            loop {
                // s1: the entry, s3: entries left after it, s2: the next.
                c.cycles += 5;
                let entry = r32(c, S1);
                c.g[V0][0] = c.lw(entry.wrapping_add(VIS_SOURCE));
                c.g[S3][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(SOURCE_FIRST_PATCH));
                c.g[S2][0] = sx(entry.wrapping_add(VIS_ENTRY_SIZE));
                if c.g[S0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(entry.wrapping_add(VIS_FRUSTUM));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S4][0];
                        NASH_CLASSIFY_PATCH.call(c, 0x0022_a454)?;
                        c.g[S0][0] = c.lw(r32(c, S0).wrapping_add(PATCH_NEXT));
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(VIS_FRUSTUM));
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a448);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S2][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a430);
                }
            }
        }
        c.cycles += 3;
        let cell = r32(c, S4);
        c.g[V0][0] = c.lw(cell.wrapping_add(CELL_LIST_HEAD));
        c.g[A0][0] = c.g[S4][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            NASH_CLASSIFY_PATCH.call(c, 0x0022_a484)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RiderSphereTree_Scale (0x329b90)

/// A rider sphere tree: the root sphere's centre (+0x10) and radius
/// (+0x20), the child count (+0x2c), then the children (0x20 bytes each from
/// +0x30: centre at +0, radius at +0x10).
const TREE_ROOT_CENTER: u32 = 0x10;
const TREE_ROOT_RADIUS: u32 = 0x20;
const TREE_CHILD_COUNT: u32 = 0x2c;
const TREE_CHILDREN: u32 = 0x30;
const TREE_CHILD_RADIUS: u32 = 0x10;
const TREE_CHILD_SIZE: u32 = 0x20;

/// `vf12 = m * vf8` (VU0 `mula`/`madda`/`madd` by the four columns of the
/// matrix at `m`, loaded into vf4..vf7), with vf8 loaded from `center`.
fn sphere_center_times_matrix(c: &mut Cpu, center: u32, m: u32) {
    c.lqc2(8, center);
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
}

/// `void RiderSphereTree_Scale(SphereTree* tree /*a0*/, Matrix* m /*a1*/,
/// float scale /*f12*/)` (0x329b90): move every sphere of the tree (root,
/// then each child) through the matrix and scale it: the centre becomes
/// `(m * c) * (scale, scale, scale, 1)`, the radius `radius * scale`. Leaf,
/// frame 0x40 (the scale vector at 0, the scaled centre at 0x10, the
/// transformed one at 0x20 and 0x30).
pub fn rider_sphere_tree_scale(c: &mut Cpu) {
    c.cycles += 43;
    let tree = r32(c, A0);
    let m = r32(c, A1);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(1, 0);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let frame = sp(c);
    // The scale vector (scale, scale, scale, 1).
    c.swc1(12, frame);
    c.g[A2][0] = 0;
    c.swc1(0, frame.wrapping_add(0xc));
    c.swc1(12, frame.wrapping_add(0x4));
    c.swc1(12, frame.wrapping_add(0x8));
    // The root.
    sphere_center_times_matrix(c, tree.wrapping_add(TREE_ROOT_CENTER), m);
    c.sqc2(12, frame.wrapping_add(0x30));
    c.lq(2, frame.wrapping_add(0x30));
    c.lwc1(0, frame);
    c.sq(2, frame.wrapping_add(0x20));
    c.lwc1(1, frame.wrapping_add(0x4));
    c.lwc1(2, frame.wrapping_add(0x20));
    c.lwc1(3, frame.wrapping_add(0x24));
    c.mul_s(2, 2, 0);
    c.lwc1(4, frame.wrapping_add(0x28));
    c.lwc1(6, frame.wrapping_add(0x8));
    c.mul_s(3, 3, 1);
    c.lwc1(0, frame.wrapping_add(0x2c));
    c.lwc1(5, frame.wrapping_add(0xc));
    c.mul_s(4, 4, 6);
    c.lwc1(1, tree.wrapping_add(TREE_ROOT_RADIUS));
    c.mul_s(0, 0, 5);
    c.swc1(2, frame.wrapping_add(0x10));
    c.swc1(3, frame.wrapping_add(0x14));
    c.mul_s(1, 1, 12);
    c.swc1(4, frame.wrapping_add(0x18));
    c.swc1(0, frame.wrapping_add(0x1c));
    c.lq(2, frame.wrapping_add(0x10));
    c.g[V1][0] = c.lw(tree.wrapping_add(TREE_CHILD_COUNT));
    c.swc1(1, tree.wrapping_add(TREE_ROOT_RADIUS));
    c.sq(2, tree.wrapping_add(TREE_ROOT_CENTER));
    if (c.g[V1][0] as i64) > 0 {
        // The children (v1: the child minus 0x30, a2: its index).
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 38;
            let child = r32(c, V1).wrapping_add(TREE_CHILDREN);
            sphere_center_times_matrix(c, child, m);
            c.sqc2(12, frame.wrapping_add(0x30));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.lq(2, frame.wrapping_add(0x30));
            c.lwc1(2, frame.wrapping_add(0x4));
            c.sq(2, frame.wrapping_add(0x20));
            c.lwc1(0, frame);
            c.lwc1(4, frame.wrapping_add(0x24));
            c.lwc1(3, frame.wrapping_add(0x20));
            c.mul_s(4, 4, 2);
            c.lwc1(5, frame.wrapping_add(0x28));
            c.lwc1(6, frame.wrapping_add(0x8));
            c.mul_s(3, 3, 0);
            c.lwc1(1, frame.wrapping_add(0x2c));
            c.lwc1(2, frame.wrapping_add(0xc));
            c.mul_s(5, 5, 6);
            c.lwc1(0, child.wrapping_add(TREE_CHILD_RADIUS));
            c.mul_s(1, 1, 2);
            c.swc1(3, frame.wrapping_add(0x10));
            c.mul_s(0, 0, 12);
            c.swc1(4, frame.wrapping_add(0x14));
            c.swc1(5, frame.wrapping_add(0x18));
            c.swc1(1, frame.wrapping_add(0x1c));
            c.swc1(0, child.wrapping_add(TREE_CHILD_RADIUS));
            c.lq(2, frame.wrapping_add(0x10));
            c.sq(2, child);
            c.g[V0][0] = c.lw(tree.wrapping_add(TREE_CHILD_COUNT));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(TREE_CHILD_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9c40);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    ret(c);
}

// ---------------------------------------------------------------------------
// cWorldCache_StreamerSetView (0x3a9658)

/// A world cache streamer (0x50 bytes, from +0x244 of the cache): active
/// flag (+0x244), state (+0x248), view position (+0x250), the view box's
/// low (+0x260) and high (+0x270) corners, the clearance (+0x280) and two
/// zeroed floats (+0x284 / +0x288).
const STREAMER_SIZE: u32 = 0x50;
const STREAMER_ACTIVE: u32 = 0x244;
const STREAMER_STATE: u32 = 0x248;
const STREAMER_VIEW_POS: u32 = 0x250;
const STREAMER_BOX_LO: u32 = 0x260;
const STREAMER_BOX_HI: u32 = 0x270;
const STREAMER_CLEARANCE: u32 = 0x280;
const STREAMER_ZERO_A: u32 = 0x284;
const STREAMER_ZERO_B: u32 = 0x288;

/// `void cWorldCache_StreamerSetView(cWorldCache* self /*a0*/,
/// unsigned i /*a1*/, sVec4* pos /*a2*/, float clearance /*f12*/)`
/// (0x3a9658): point streamer `i` at `pos`: the view box is `pos -
/// (r, r, r, 0)` .. `pos + (r, r, r, 0)` with `r = max(clearance, 0)`
/// (the w components use 0), the streamer becomes active with state 0.
/// Leaf, frame 0x20 (the copy of `pos` and the radius vector).
pub fn world_cache_streamer_set_view(c: &mut Cpu) {
    c.cycles += 34;
    c.g[V0][0] = STREAMER_SIZE as u64;
    c.mtc1(0, 1); // f1 = 0
    c.mult(0, 3, 5, 2); // v1 = i * 0x50
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let frame = sp(c);
    let pos = r32(c, A2);
    c.lq(2, pos);
    c.max_s(0, 12, 1); // f0 = max(clearance, 0)
    c.g[A1][0] = 1;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    let streamer = r32(c, A0);
    c.sq(2, streamer.wrapping_add(STREAMER_VIEW_POS));
    c.swc1(0, streamer.wrapping_add(STREAMER_CLEARANCE));
    c.lq(2, pos);
    c.sq(2, frame);
    c.sq(2, frame.wrapping_add(0x10));
    c.sq(2, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lq(3, frame.wrapping_add(0x10));
    c.sq(3, streamer.wrapping_add(STREAMER_BOX_HI));
    // The radius vector (clearance, clearance, clearance, 0).
    c.swc1(12, frame.wrapping_add(0x8));
    c.swc1(12, frame);
    c.swc1(12, frame.wrapping_add(0x4));
    c.swc1(1, frame.wrapping_add(0xc));
    c.lqc2(3, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lqc2(4, frame);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lqc2(3, streamer.wrapping_add(STREAMER_BOX_HI));
    c.lqc2(4, frame);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, streamer.wrapping_add(STREAMER_BOX_HI));
    c.sw(streamer.wrapping_add(STREAMER_ACTIVE), c.g[A1][0]);
    c.swc1(1, streamer.wrapping_add(STREAMER_ZERO_B));
    c.sw(streamer.wrapping_add(STREAMER_STATE), 0);
    c.swc1(1, streamer.wrapping_add(STREAMER_ZERO_A));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_SetVolume (0x3b7e70)

const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

const SND_SET_VOLUME: Callee = Callee {
    addr: 0x003b_85f0,
    f: ee::s13::SND_SetVolume,
};

/// A sound stream: its sound handle (+4) and volume byte (+0xec).
const STREAM_SND_HANDLE: u32 = 0x4;
const STREAM_VOLUME: u32 = 0xec;

/// `int SNDSTRM_SetVolume(int stream /*a0*/, int volume /*a1*/)`
/// (0x3b7e70): store the stream's volume byte and set the volume of its
/// sound handle (`SND_SetVolume`); 0, or -8 when the stream does not exist.
/// Frame 0x20 (ra at 0, s0 = volume at 0x10).
pub fn sndstrm_set_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        SNDSTRM_GET_STREAM.call(c, 0x003b_7e84)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(-8i32 as u32);
        } else {
            c.cycles += 5;
            let stream = r32(c, V0);
            c.g[A0][0] = c.lw(stream.wrapping_add(STREAM_SND_HANDLE));
            c.g[A1][0] = c.g[S0][0];
            c.sb(stream.wrapping_add(STREAM_VOLUME), c.g[S0][0]);
            SND_SET_VOLUME.call(c, 0x003b_7e9c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cOVStateHUD1P_UpdateRaceProgress (0x20ed20)

const RACE_PROGRESS_UPDATE_RIDER: Callee = Callee {
    addr: 0x0021_0618,
    f: ee::s05::RaceProgress_UpdateRider,
};

/// The course (`gp-0x9a0`, null outside a race), the world (`gp-0x848`)
/// and the "progress changed" word (`gp-0x990`).
const GP_RACE_COURSE: u32 = 0x9a0;
const GP_WORLD: u32 = 0x848;
const GP_PROGRESS_DIRTY: u32 = 0x990;
/// world+0x84 -> racers, racers+0xc -> race, race+0x78: the rider count.
const WORLD_RACERS: u32 = 0x84;
const RACERS_RACE: u32 = 0xc;
const RACE_RIDER_COUNT: u32 = 0x78;
/// The race-progress slots (0x18 bytes each; the active word at +0).
const PROGRESS_SLOTS: u32 = 0x004c_8bc8;
const PROGRESS_SLOT_SIZE: u32 = 0x18;

/// `void cOVStateHUD1P_UpdateRaceProgress()` (0x20ed20): in a race, update
/// every rider's progress (`RaceProgress_UpdateRider(i)`) and clear its
/// slot's active word, then clear the "progress changed" word. Frame 0x40
/// (ra at 0, s2 / s1 / s0 at 0x10 / 0x20 / 0x30).
pub fn hud1p_update_race_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let gp = r32(c, GP);
        c.g[V0][0] = c.lw(gp.wrapping_sub(GP_RACE_COURSE));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = c.lw(gp.wrapping_sub(GP_WORLD));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(WORLD_RACERS));
            c.g[S2][0] = c.lw(r32(c, V1).wrapping_add(RACERS_RACE));
            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(RACE_RIDER_COUNT));
            c.g[S0][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                // s0: the rider, s1: its slot.
                c.cycles += 3;
                c.g[V0][0] = 0x004d_0000;
                c.g[S1][0] = sx(PROGRESS_SLOTS);
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    RACE_PROGRESS_UPDATE_RIDER.call(c, 0x0020_ed68)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.sw(r32(c, S1), 0);
                    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(RACE_RIDER_COUNT));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(PROGRESS_SLOT_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0020_ed60);
                    }
                }
            }
            c.cycles += 1;
            let gp = r32(c, GP);
            c.sw(gp.wrapping_sub(GP_PROGRESS_DIRTY), 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DLHeap_Memalign (0x31f2c8)

const DLHEAP_MALLOC: Callee = Callee {
    addr: 0x0031_e818,
    f: ee::s10::DLHeap_Malloc,
};

const DLHEAP_FREE: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};

/// `int* __errno()` (through the function table).
const ERRNO: u32 = 0x0041_5fb8;
const ENOMEM: u64 = 0xc;

/// dlmalloc chunk header bits: previous chunk in use, chunk is mmapped.
const PREV_INUSE: u64 = 0x1;
const IS_MMAPPED: u64 = 0x2;

/// `void* DLHeap_Memalign(DLHeap* heap /*a0*/, uint align /*a1*/,
/// uint size /*a2*/)` (0x31f2c8): dlmalloc's `memalign`. An alignment of 16
/// or less is a plain `DLHeap_Malloc`; otherwise the alignment is raised to
/// at least 16 and to a power of two, the request padded to a chunk size
/// (`ENOMEM` and null when it is too large), and `size + align + 16` bytes
/// are allocated. A misaligned result gives up its leading part (as a free
/// chunk, or by moving the offset of an mmapped chunk), a large enough tail
/// is split off and freed. Frame 0x50 (ra at 0, s3 / s2 / s1 / s0 at
/// 0x10 .. 0x40).
pub fn dlheap_memalign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A1][0] = c.g[A2][0];
        c.g[V0][0] = (c.g[S0][0] < 0x11) as u64;
        frame.save_ra(c, 0);
        'tail: {
            if c.g[V0][0] != 0 {
                // Small alignment: plain malloc (a0 heap, a1 size).
                c.cycles += 4;
                DLHEAP_MALLOC.call(c, 0x0031_f2fc)?;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            // s0 = the alignment, at least 16 and a power of two.
            c.cycles += 8;
            c.g[V0][0] = 0xf;
            c.g[A0][0] = 0x10;
            c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[V1][0] &= c.g[S0][0];
            let pow2 = c.g[V1][0] == 0;
            c.g[V1][0] = 0x20;
            if !pow2 {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[V1][0];
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = sx(r32(c, V1) << 1);
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            c.g[S0][0] = c.g[V1][0];
                            break;
                        }
                        c.g[V1][0] = sx(r32(c, V1) << 1);
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_f338);
                        }
                    }
                }
            }
            // The request as a chunk size (s2), or ENOMEM.
            c.cycles += 5;
            c.g[V0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x13));
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                call_ptr(c, ERRNO, 0x0031_f374)?;
                c.g[V1][0] = ENOMEM;
                c.sw(r32(c, V0), c.g[V1][0]);
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
            c.g[S2][0] = 0x10;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
            }
            // Over-allocate: size + align + 16.
            c.cycles += 7;
            c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, S0)));
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            DLHEAP_MALLOC.call(c, 0x0031_f3ac)?;
            c.g[A3][0] = c.g[V0][0];
            if c.g[A3][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'tail;
            }
            c.divu(0, A3, S0); // HI = misalignment
            c.cycles += 2;
            c.cycles += 3;
            c.g[V0][0] = c.hi[0];
            let misaligned = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, A3).wrapping_sub(8)); // the chunk
            if misaligned {
                // s0: the aligned chunk, t0: the leading size.
                c.cycles += 18;
                let mem = r32(c, A3);
                let align = r32(c, S0);
                c.g[V0][0] = sx(mem.wrapping_add(align));
                c.g[V1][0] = sx(0u32.wrapping_sub(align));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[A1][0] = 0xf;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A2][0] = c.lw(mem.wrapping_sub(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                c.g[V1][0] = 0xffff_ffff_ffff_0000;
                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S1)));
                c.g[S0][0] = sx(r32(c, V0).wrapping_add(align));
                c.g[A1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                c.g[V1][0] |= 0xfffc;
                if c.g[A1][0] != 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
                c.g[V1][0] = c.g[A2][0] & c.g[V1][0];
                c.g[T0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S1)));
                c.g[A2][0] &= IS_MMAPPED;
                let mmapped = c.g[A2][0] != 0;
                c.g[A0][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
                let chunk = r32(c, S0);
                if mmapped {
                    // Move the mmapped chunk's offset forward.
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(mem.wrapping_sub(8));
                    c.g[V0][0] = c.g[A0][0] | IS_MMAPPED;
                    c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T0)));
                    c.g[V0][0] = sx(chunk.wrapping_add(8));
                    c.sw(chunk, c.g[V1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break 'tail;
                }
                // Give the leading part back.
                c.cycles += 14;
                c.g[V0][0] = c.g[A0][0] | PREV_INUSE;
                c.g[A2][0] = sx(chunk.wrapping_add(r32(c, A0)));
                c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                c.g[A1][0] = c.g[A3][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S0][0];
                let next = r32(c, A2);
                c.g[V0][0] = c.lw(next.wrapping_add(4));
                c.g[V0][0] |= PREV_INUSE;
                c.sw(next.wrapping_add(4), c.g[V0][0]);
                c.g[V1][0] = c.lw(mem.wrapping_sub(4));
                c.g[V1][0] &= PREV_INUSE;
                c.g[V1][0] |= c.g[T0][0];
                c.sw(mem.wrapping_sub(4), c.g[V1][0]);
                DLHEAP_FREE.call(c, 0x0031_f470)?;
            }
            // s1: the aligned chunk.
            c.cycles += 4;
            let chunk = r32(c, S1);
            c.g[A0][0] = c.lw(chunk.wrapping_add(4));
            c.g[V0][0] = c.g[A0][0] & IS_MMAPPED;
            let mmapped = c.g[V0][0] != 0;
            c.g[V0][0] = sx(chunk.wrapping_add(8));
            if !mmapped {
                // Split off and free a large enough tail.
                c.cycles += 7;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x10));
                c.g[V0][0] |= 0xfffc;
                c.g[V0][0] &= c.g[A0][0];
                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                let split = c.g[V1][0] != 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S2)));
                if split {
                    c.cycles += 10;
                    let need = r32(c, S2);
                    c.g[A1][0] = sx(chunk.wrapping_add(need));
                    c.g[V0][0] |= PREV_INUSE;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(r32(c, A1).wrapping_add(4), c.g[V0][0]);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(8));
                    c.g[V0][0] = c.lw(chunk.wrapping_add(4));
                    c.g[V0][0] &= PREV_INUSE;
                    c.g[V0][0] |= c.g[S2][0];
                    c.sw(chunk.wrapping_add(4), c.g[V0][0]);
                    DLHEAP_FREE.call(c, 0x0031_f4c4)?;
                }
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(8));
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DynamicParticleModifier_Render (0x346070)

/// `void DynamicEmitter_Render(DynamicEmitter*, int pass)` (through the
/// function table).
const DYNAMIC_EMITTER_RENDER: u32 = 0x0037_1380;

/// The modifier: its "hidden" word (+0x18) and its embedded emitter (+0x60).
const MODIFIER_HIDDEN: u32 = 0x18;
const MODIFIER_EMITTER: u32 = 0x60;
/// The render pass handed to the emitter.
const MODIFIER_RENDER_PASS: u64 = 7;

/// `void DynamicParticleModifier_Render(DynamicParticleModifier* self
/// /*a0*/)` (0x346070): unless hidden, render the embedded emitter
/// (`DynamicEmitter_Render(&self->emitter, 7)`). Frame 0x10 (ra at 0).
pub fn dynamic_particle_modifier_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = c.lw(this.wrapping_add(MODIFIER_HIDDEN));
        frame.restore_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(this.wrapping_add(MODIFIER_EMITTER));
            c.g[A1][0] = MODIFIER_RENDER_PASS;
            call_ptr(c, DYNAMIC_EMITTER_RENDER, 0x0034_6090)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_1f18, adddf3, "__adddf3"),
    (
        0x002e_23e0,
        dynamic_spray_queue_burst,
        "cDynamicSprayFX_QueueBurst",
    ),
    (
        0x0027_8918,
        script_engine_poll_async_loads,
        "ScriptEngine_PollAsyncLoads",
    ),
    (0x0022_a408, nash_classify_patches, "Nash_ClassifyPatches"),
    (
        0x0032_9b90,
        rider_sphere_tree_scale,
        "RiderSphereTree_Scale",
    ),
    (
        0x003a_9658,
        world_cache_streamer_set_view,
        "cWorldCache_StreamerSetView",
    ),
    (0x003b_7e70, sndstrm_set_volume, "SNDSTRM_SetVolume"),
    (
        0x0020_ed20,
        hud1p_update_race_progress,
        "cOVStateHUD1P_UpdateRaceProgress",
    ),
    (0x0031_f2c8, dlheap_memalign, "DLHeap_Memalign"),
    (
        0x0034_6070,
        dynamic_particle_modifier_render,
        "DynamicParticleModifier_Render",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------- __adddf3

    /// `__adddf3`: random doubles and edge cases (zeros of both signs,
    /// infinities, NaNs, denormals, opposite operands) through the real
    /// soft-float helpers.
    #[test]
    fn adddf3_matches() {
        use crate::difftest::{check, scramble, Rng};
        let edge = |r: &mut Rng| -> u64 {
            let sign = ((r.next_u32() & 1) as u64) << 63;
            let mant = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & ((1u64 << 52) - 1);
            sign | match r.next_u32() % 8 {
                0 => 0,
                1 => mant,
                2 => 0x7ff0_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000 | mant | 1,
                4 => 1.0f64.to_bits(),
                _ => (((r.next_u32() % 200 + 923) as u64) << 52) | mant,
            }
        };
        check(ee::s15::f_00411f18__generated, adddf3, 300, &|c, seed| {
            let mut r = Rng(0x0041_1f18 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0041_1f18;
            let a = edge(&mut r);
            let b = match seed % 5 {
                0 => a,
                1 => a ^ (1 << 63),
                _ => edge(&mut r),
            };
            c.g[A0][0] = a;
            c.g[A1][0] = b;
        });
    }

    // ------------------------------------------------ cDynamicSprayFX_QueueBurst

    /// `cDynamicSprayFX_QueueBurst`: weaker and stronger bursts, near and
    /// far positions, the short-range flag, every step of the rider's
    /// carving test, `b` zero and non-zero.
    #[test]
    fn dynamic_spray_queue_burst_matches() {
        use crate::difftest::{check, put_vec, scramble, Rng, DATA};
        check(
            ee::s09::f_002e23e0__generated,
            dynamic_spray_queue_burst,
            300,
            &|c, seed| {
                let mut r = Rng(0x002e_23e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002e_23e0;
                let this = DATA;
                let rider = DATA + 0x1000;
                let pos = DATA + 0x400;
                let dir = DATA + 0x410;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(dir);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                let speed = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(50.0)
                };
                c.f[12] = speed;
                c.bus.write32(
                    this + SPRAY_STRENGTH,
                    if seed % 5 == 0 {
                        r.f32(100.0)
                    } else {
                        r.f32(10.0)
                    },
                );
                c.bus.write32(
                    this + SPRAY_SHORT_RANGE,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(this + SPRAY_RIDER, rider);
                // Queued position near or far from the new one.
                let spread = match seed % 4 {
                    0 => 30.0,
                    1 => 120.0,
                    2 => 300.0,
                    _ => 1000.0,
                };
                put_vec(
                    c,
                    this + SPRAY_POS,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    this + SPRAY_DIR,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_vec(
                    c,
                    pos,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    dir,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                let k = seed / 4;
                let pick = |bit: u32, r: &mut Rng| if k & bit == 0 { 0 } else { r.next_u32() | 1 };
                let a = pick(1, &mut r);
                let block = if k & 2 == 0 { 0 } else { r.next_u32() | 1 };
                let b = pick(4, &mut r);
                let cc = pick(8, &mut r);
                c.bus.write32(rider + RIDER_CARVE_A, a);
                c.bus.write32(rider + RIDER_CARVE_BLOCK, block);
                c.bus.write32(rider + RIDER_CARVE_B, b);
                c.bus.write32(rider + RIDER_CARVE_C, cc);
            },
        );
    }

    // ---------------------------------------------- ScriptEngine_PollAsyncLoads

    /// `ScriptEngine_PollAsyncLoads`: no pending loads; a filter handle that
    /// is unknown (`AsyncFile_IsDone` -1: the handler runs), still loading
    /// (0: skipped) or absent; bank slots mixing empty, loading and finished
    /// handles. The handlers take their short paths: `AsyncFile_GetResult`
    /// of an unknown handle leaves the size word on the stack, which the
    /// fixture zeroes (filter: nothing loaded; bank: the slot is cleared).
    /// The mutex syscalls are fakes, some of which unwind.
    #[test]
    fn script_engine_poll_async_loads_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, SP};
        /// `AsyncFile_FromHandle`'s table: count and pointer to 0x30-byte
        /// entries (handle at +0, request state at +0x1c).
        const FILE_COUNT: u32 = 0x0051_9bf0;
        const FILE_TABLE_PTR: u32 = 0x0045_0c34;
        const FILE_TABLE: u32 = DATA + 0x20_0000;
        check(
            ee::s07::f_00278918__generated,
            script_engine_poll_async_loads,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_8918, 12);
                c.pc = 0x0027_8918;
                let this = DATA;
                c.g[A0][0] = sx(this);
                // Four known files, all still loading (state non-zero).
                c.bus.write32(FILE_COUNT, 4);
                c.bus.write32(FILE_TABLE_PTR, FILE_TABLE);
                let loading = |i: u32| 0x100 * (i + 1) + i;
                for i in 0..4 {
                    c.bus.write32(FILE_TABLE + 0x30 * i, loading(i));
                    c.bus
                        .write32(FILE_TABLE + 0x30 * i + 0x1c, r.next_u32() | 1);
                }
                let unknown =
                    |r: &mut crate::difftest::Rng| 0x1000 + (r.next_u32() & 0xff00) + 0x10;
                c.bus.write32(
                    this + SCRIPT_PENDING_LOADS,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let filter = match seed % 3 {
                    0 => 0,
                    1 => loading(r.next_u32() % 4),
                    _ => unknown(&mut r),
                };
                c.bus.write32(this + SCRIPT_FILTER_HANDLE, filter);
                for i in 0..10 {
                    let h = match r.next_u32() % 4 {
                        0 | 1 => 0,
                        2 => loading(r.next_u32() % 4),
                        _ => unknown(&mut r),
                    };
                    c.bus
                        .write32(this + SCRIPT_BANK_HANDLES + SCRIPT_BANK_SLOT_SIZE * i, h);
                }
                // The handlers' `AsyncFile_GetResult` size words (their frames
                // below ours): zero, so they take the short paths.
                c.bus.write32(SP - 0x50 - 0x30 + 0x4, 0);
                c.bus.write32(SP - 0x50 - 0xf0 + 0x84, 0);
            },
        );
    }

    // ---------------------------------------------------- Nash_ClassifyPatches

    /// `Nash_ClassifyPatches`: zero, negative and up to four list entries,
    /// sources with no patches and with chains of one to three; the cell's
    /// list head set or clear (the extra no-node classify, whose `VI1`
    /// handle picks every arm of the patch table). The patch nodes take the
    /// classifier's early misses (negative handle word, or a slot word that
    /// is not 6), so no VU0 micro program runs.
    #[test]
    fn nash_classify_patches_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, put_words, DATA};
        use crate::game::main::ssxapp::tests::NASH_PATCH_HANDLERS;
        use crate::game::main::ssxapp::NASH_PATCH_TABLE;
        check(
            ee::s05::f_0022a408__generated,
            nash_classify_patches,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_a408, 0);
                c.pc = 0x0022_a408;
                put_words(c, NASH_PATCH_TABLE, &NASH_PATCH_HANDLERS);
                c.bus.plat.write_ctrl(1, seed % 8);
                let cell = DATA + 0x1_0000;
                let block = DATA + 0x2_0000;
                let heap = DATA + 0x3_0000;
                let list = DATA + 0x4_0000;
                let sources = DATA + 0x5_0000;
                let nodes = DATA + 0x6_0000;
                c.g[A0][0] = sx(cell);
                c.g[A1][0] = sx(list);
                // The classifier's heap: slot words all 0 (not 6), so a node with
                // a non-negative handle misses too.
                c.bus.write32(cell + 0x90, block);
                c.bus.write32(block, heap);
                c.bus.write32(cell + 0x50a8, r.next_u32() & 3);
                c.bus.write32(cell + 0x70ac, r.next_u32() & 3);
                c.bus.write32(
                    cell + CELL_LIST_HEAD,
                    if seed % 3 == 0 { 0 } else { nodes + 0x8000 },
                );
                let count = match seed % 7 {
                    0 => 0,
                    1 => (-(r.next_u32() as i32 & 0xff) - 1) as u32,
                    _ => 1 + r.next_u32() % 4,
                };
                c.g[A2][0] = sx(count);
                let entries = if (count as i32) > 0 { count } else { 0 };
                let mut node = nodes;
                for i in 0..entries {
                    let src = sources + 0x40 * i;
                    c.bus.write32(list + VIS_ENTRY_SIZE * i + VIS_SOURCE, src);
                    c.bus
                        .write32(list + VIS_ENTRY_SIZE * i + VIS_FRUSTUM, r.next_u32());
                    let chain = r.next_u32() % 4;
                    let mut link = src + SOURCE_FIRST_PATCH;
                    for _ in 0..chain {
                        c.bus.write32(link, node);
                        let handle = if r.next_u32() % 2 == 0 {
                            0xffff
                        } else {
                            r.next_u32() as u16 & 0x7
                        };
                        c.bus.write16(node + 0x156, handle);
                        c.bus.write8(node + 0x155, r.next_u32() as u8);
                        link = node + PATCH_NEXT;
                        node += 0x200;
                    }
                    c.bus.write32(link, 0);
                }
            },
        );
    }

    // --------------------------------------------------- RiderSphereTree_Scale

    /// `RiderSphereTree_Scale`: random matrices, scales (edge cases too) and
    /// trees with no, a negative count of, and up to six children.
    #[test]
    fn rider_sphere_tree_scale_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ee::s10::f_00329b90__generated,
            rider_sphere_tree_scale,
            300,
            &|c, seed| {
                let mut r = Rng(0x0032_9b90 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0032_9b90;
                let tree = DATA;
                let m = DATA + 0x1000;
                c.g[A0][0] = sx(tree);
                c.g[A1][0] = sx(m);
                c.f[12] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                for i in 0..16 {
                    c.bus.write32(
                        m + 4 * i,
                        if seed % 9 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(100.0)
                        },
                    );
                }
                fill(c, &mut r, tree, 0x30 + 8 * 8);
                for i in 0..4 + 8 * 4 {
                    let a = tree + TREE_ROOT_CENTER + 4 * i;
                    if a != tree + TREE_CHILD_COUNT {
                        c.bus.write32(a, r.f32(500.0));
                    }
                }
                let count = match seed % 6 {
                    0 => 0,
                    1 => (-(r.next_u32() as i32 & 0xff) - 1) as u32,
                    _ => 1 + r.next_u32() % 6,
                };
                c.bus.write32(tree + TREE_CHILD_COUNT, count);
            },
        );
    }

    // --------------------------------------------- cWorldCache_StreamerSetView

    /// `cWorldCache_StreamerSetView`: streamer indices 0..7, random view
    /// positions and clearances (negative and edge values too: the `max`
    /// with 0).
    #[test]
    fn world_cache_streamer_set_view_matches() {
        use crate::difftest::{check, put_vec, scramble, Rng, DATA};
        check(
            ee::s13::f_003a9658__generated,
            world_cache_streamer_set_view,
            300,
            &|c, seed| {
                let mut r = Rng(0x003a_9658 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003a_9658;
                let cache = DATA;
                let pos = DATA + 0x2000;
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = sx(pos);
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(300.0)
                };
                put_vec(
                    c,
                    pos,
                    [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.next_u32()],
                );
            },
        );
    }

    // ------------------------------------------------------- SNDSTRM_SetVolume

    /// `SNDSTRM_SetVolume`: stream indices in range (one slot empty), past
    /// the count and negative; the stream's sound handle and the channels
    /// come from `SND_SetVolume`'s own fixture (bad handles, unchanged
    /// volume, one or more channels), which runs for real.
    #[test]
    fn sndstrm_set_volume_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        use crate::game::lib::snd::tests::set_volume_setup;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        const STREAMS: u32 = 0x0050_a0a0;
        check(ee::s13::f_003b7e70__generated, sndstrm_set_volume, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x003b_7e70, 0);
            c.pc = 0x003b_7e70;
            set_volume_setup(c, &mut r);
            let handle = r32(c, A0);
            c.bus.write8(STREAM_COUNT, 4);
            for i in 0..4u32 {
                let stream = DATA + 0x8_0000 + 0x200 * i;
                c.bus
                    .write32(STREAMS + 4 * i, if i == 2 { 0 } else { stream });
                c.bus.write32(stream + STREAM_SND_HANDLE, handle);
                c.bus.write8(stream + STREAM_VOLUME, r.next_u32() as u8);
            }
            c.g[A0][0] = match seed % 8 {
                0 => sx(0x8000_0000 | r.next_u32()),
                1 => 4 + (r.next_u32() % 100) as u64,
                _ => (r.next_u32() % 4) as u64,
            };
        });
    }

    // ---------------------------------------- cOVStateHUD1P_UpdateRaceProgress

    /// `cOVStateHUD1P_UpdateRaceProgress`: no course; a course with zero to
    /// four riders, each updated by the real `RaceProgress_UpdateRider` on
    /// its fixture (world, course, segments, riders 700 from the finish;
    /// its projection is a fake, which may unwind).
    #[test]
    fn hud1p_update_race_progress_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, GAME_GP};
        use crate::game::fe::ovstates7::tests::{
            put_course, put_rider, put_world, COURSE, COURSE_SEGS, RACE, RIDER,
        };
        check(
            ee::s05::f_0020ed20__generated,
            hud1p_update_race_progress,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_ed20, 4);
                c.pc = 0x0020_ed20;
                put_world(c);
                put_course(c);
                c.bus.write32(GAME_GP - 0x99c, COURSE_SEGS);
                c.bus.write32(GAME_GP - 0x54ac, 138.8889f32.to_bits());
                c.bus.write32(
                    COURSE + 0x10,
                    (1100.0f32 + f32::from_bits(r.f32(400.0))).to_bits(),
                );
                let riders = seed % 5;
                c.bus.write32(RACE + RACE_RIDER_COUNT, riders);
                for i in 0..riders {
                    put_rider(c, i);
                    c.bus.write32(RIDER + 0x100 * i + 0x4d0, 700.0f32.to_bits());
                    let slot = PROGRESS_SLOTS + PROGRESS_SLOT_SIZE * i;
                    c.bus.write32(slot, r.next_u32() % 2);
                    c.bus.write32(slot + 4, 0);
                    c.bus.write32(slot + 8, r.f32(1000.0));
                }
                c.bus.write32(GAME_GP - GP_PROGRESS_DIRTY, r.next_u32());
                if seed % 7 == 0 {
                    c.bus.write32(GAME_GP - GP_RACE_COURSE, 0);
                }
            },
        );
    }

    // --------------------------------------------------------- DLHeap_Memalign

    /// `DLHeap_Memalign` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, then a few random mallocs so results start at
    /// different alignments): small alignments (plain malloc), powers of two
    /// and other values up to 4 KiB (rounded up), sizes from 0 to beyond the
    /// arena (malloc fails), and sizes above the limit (`ENOMEM` through
    /// `__errno`). Not reached: the mmapped-chunk paths (the heap never
    /// maps).
    #[test]
    fn dlheap_memalign_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        const BACK: u32 = 0x0000_7770;
        fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
            for (i, a) in args.iter().enumerate() {
                c.g[[A0, A1, A2][i]][0] = *a;
            }
            c.g[RA][0] = BACK as u64;
            c.pc = addr;
            f(c);
            assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
            c.g[V0][0]
        }
        check(ee::s10::f_0031f2c8__generated, dlheap_memalign, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_f2c8, 0);
            let ra = c.g[RA][0];
            run(
                c,
                0x0031_ff60,
                ee::s10::DLHeap_InitArena,
                &[HEAP as u64, ARENA as u64, 0x4_0000],
            );
            for _ in 0..r.next_u32() % 4 {
                let n = (r.next_u32() % 0x300) as u64;
                run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, n]);
            }
            c.g[RA][0] = ra;
            c.pc = 0x0031_f2c8;
            c.g[A0][0] = sx(HEAP);
            c.g[A1][0] = match seed % 6 {
                0 => (r.next_u32() % 17) as u64,
                1 => 1u64 << (5 + r.next_u32() % 8),
                _ => (17 + r.next_u32() % 4096) as u64,
            };
            c.g[A2][0] = match seed % 11 {
                0 => 0,
                1 => 0x10_0000 + (r.next_u32() % 0x1000) as u64,
                2 => sx(0xffff_ffe0 + r.next_u32() % 0x20),
                _ => (r.next_u32() % 0x2000) as u64,
            };
        });
    }

    // ------------------------------------------ DynamicParticleModifier_Render

    /// `DynamicParticleModifier_Render`: hidden, and visible with the real
    /// `DynamicEmitter_Render` taking its early outs (no particles at
    /// +0x174, no texture at +0x1e0, or a negative texture slot).
    #[test]
    fn dynamic_particle_modifier_render_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s11::f_00346070__generated,
            dynamic_particle_modifier_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_6070, 0);
                c.pc = 0x0034_6070;
                let this = DATA;
                let emitter = this + MODIFIER_EMITTER;
                let table = DATA + 0x2_0000;
                c.g[A0][0] = sx(this);
                c.bus.write32(
                    this + MODIFIER_HIDDEN,
                    if seed % 4 == 0 { r.next_u32() | 1 } else { 0 },
                );
                let k = r.next_u32() % 16;
                match seed % 3 {
                    0 => c.bus.write32(emitter + 0x174, 0),
                    1 => {
                        c.bus.write32(emitter + 0x174, r.next_u32() | 1);
                        c.bus.write32(emitter + 0x1e0, 0);
                    }
                    _ => {
                        c.bus.write32(emitter + 0x174, r.next_u32() | 1);
                        c.bus.write32(emitter + 0x1e0, r.next_u32() | 1);
                        c.bus.write32(GAME_GP + 0x2a90, table);
                        c.bus.write32(emitter + 0x4, 0);
                        c.bus.write32(emitter + 0x10, (k as f32).to_bits());
                        c.bus
                            .write32(table + 4 * k + 0xf50, 0x8000_0000 | r.next_u32());
                    }
                }
            },
        );
    }
}
