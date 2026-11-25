//! `src/object/flexbridgenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopupmisc::s32;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::util::menu::gp_5 as gp;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_6d38,
        flex_bridge_init_segment_midpoints,
        "FlexBridge_InitSegmentMidpoints",
    ),
    (0x0034_75a8, flex_bridge_update, "FlexBridge_Update"),
    (0x0034_7b80, flex_bridge_render, "FlexBridge_Render"),
    (0x0034_7d38, flex_bridge_save, "FlexBridge_Save"),
    (0x0034_7f90, flex_rail_dtor, "FlexRail_dtor"),
    (0x0034_8008, flex_rail_update, "FlexRail_Update"),
    (
        0x0034_8058,
        flex_rail_update_space_node,
        "FlexRail_UpdateSpaceNode",
    ),
    (0x0034_8b40, flex_rail_save, "FlexRail_Save"),
    (0x0034_66a0, flex_bridge_build_mesh, "FlexBridge_BuildMesh"),
    (
        0x0034_6e38,
        flex_bridge_apply_force,
        "FlexBridge_ApplyForce",
    ),
    (
        0x0034_70d0,
        flex_bridge_update_intersect_info,
        "FlexBridge_UpdateIntersectInfo",
    ),
    (0x0034_7268, flex_bridge_intersect, "FlexBridge_Intersect"),
    (
        0x0034_75d8,
        flexbridge_update_verts,
        "FlexBridge_UpdateVerts",
    ),
    (0x0034_7d90, flex_rail_node_ctor, "FlexRail_ctor"),
    (0x0034_7ea8, flex_rail_node_ctor_2, "FlexRail_ctor_2"),
    (0x0034_80c8, flex_rail_render, "FlexRail_Render"),
    (
        0x0034_8290,
        flex_rail_find_closest_point,
        "FlexRail_FindClosestPoint",
    ),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;

    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;

    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;

    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `FlexBridge_Update(FlexBridge* bridge)` (0x3475a8): one physics step of the
/// bridge springs, then rebuild the rendered vertices.
pub fn flex_bridge_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        call_ptr(c, 0x0035_3fc0, 0x0034_75c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0034_75d8, 0x0034_75c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_Save(FlexRail* self, Stream* stream)` (0x348b40): 0x30 bytes
/// at `+0x20`.
pub fn flex_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::WORLD_OBJ_SAVE,
            0x20,
            0x30,
            0x0034_8b5c,
            0x0034_8b78,
        )
    })
}

/// `void FlexRail_Update(FlexRail* self)` (0x348008): run the two per-node
/// virtual updates, then re-place the space node.
pub fn flex_rail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x178).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x17c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_802c)?;
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x180).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x184).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_8040)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_RAIL_UPDATE_SPACE_NODE.call(c, 0x0034_8048)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_Save(FlexBridge* self, Stream* stream)` (0x347d38): save
/// the world object, the bridge springs and the 0x30 bytes at +0x20.
pub fn flex_bridge_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_7d54);
        c.g[S1][0] = c.g[A1][0];
        callee::WORLD_OBJ_SAVE_2.call(c, 0x0034_7d54)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[RA][0] = sx(0x0034_7d60);
        c.g[A1][0] = c.g[S1][0];
        callee::FLEX_BRIDGE_SPRINGS_SAVE.call(c, 0x0034_7d60)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x20));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_7d7c);
        c.g[A0][0] = this_with_delta(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0034_7d7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FlexRail_UpdateSpaceNode` (0x348058): refresh a rail's space node: call
/// its `vt[50]` update, get the world painter, and hand the rail's box to the
/// spatial hash (`SpatialHash_Insert(world, 2, id, box, &old)`).
pub fn flex_rail_update_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = low(c, S0).wrapping_add(0x30);
        c.lq(2, r32(c, S0).wrapping_add(0x40));
        c.lq(3, r32(c, S0).wrapping_add(0x30));
        c.sq(2, frame.at(c, 0x10));
        c.sq(3, frame.at(c, 0));
        c.g[V1][0] = lw_at(c, S0, 0xc);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x190)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x194)).lw(c);
        let update = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, update, 0x0034_8094)?;
        call_ptr(c, addr::WORLD_PAINTER_2D, 0x0034_809c)?;
        c.g[A2][0] = lw_at(c, S0, 0x54);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, addr::SPATIAL_INSERT, 0x0034_80b4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `func_00347F90` (0x347f90): `FlexRail_dtor`. Installs vtable `0x4900d0`,
/// frees the two arrays at `+0x50` (`0x317e98`) and destroys the object at
/// `+0x58` (`vtable[8]`, flag 3) before running the world object destructor.
pub fn flex_rail_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_r(c, S0, 0xc, c.g[V0][0]);
        c.g[A0][0] = lw_r(c, S0, 0x50);
        let no_array = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_array {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0034_7fc4);
            call_ptr(c, 0x0031_7e98, 0x0034_7fc4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x58);
        let no_obj = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !no_obj {
            c.cycles += 5;
            c.g[V0][0] = Addr(low_2(c, A2)).lw(c);
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low_2(c, V1);
            c.g[RA][0] = sx(0x0034_7fe4);
            c.g[A0][0] = sx(low_2(c, A2).wrapping_add(low_2(c, A0)));
            call_ptr(c, target, 0x0034_7fe4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0034_7ff0);
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_OBJ_DTOR.call(c, 0x0034_7ff0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00346D38(FlexBridge* self)` (0x346d38): place the midpoint of
/// every bridge segment (`pts[i]` to `pts[count + i]`) into `segs[i].pos`, then
/// recompute the bridge's start/end offsets from the first segment.
pub fn flex_bridge_init_segment_midpoints(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    let this = c.g[A1][0] as u32;
    c.g[T1][0] = c.lw(p(this, 0x50)); // count
    c.g[A2][0] = 0;
    let empty = s32(c.g[T1][0]) <= 0;
    c.g[A3][0] = c.lw(p(this, 0x74)); // segs
    if !empty {
        c.cycles += 4;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.g[T0][0] = sx((c.g[T1][0] as u32) << 4);
        'loop_segments: loop {
            c.cycles += 32;
            c.g[V0][0] = c.lw(p(this, 0x7c)); // pts
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V0][0] as u32); // pts[count + i]
            c.lqc2(4, c.g[V1][0] as u32); // pts[i]
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0x20));
            c.mfc1(3, 0); // v1 = 0.5
            c.lqc2(4, p(c.g[SP][0] as u32, 0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[V1][0] = c.lw(p(this, 0x7c));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sq(2, p(c.g[SP][0] as u32, 0x10));
            c.lqc2(3, c.g[A0][0] as u32);
            c.lqc2(4, p(c.g[SP][0] as u32, 0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0));
            c.sq(2, c.g[A3][0] as u32); // segs[i].pos
            c.g[V0][0] = c.lw(p(this, 0x50));
            c.g[V0][0] = ((s32(c.g[A2][0]) < s32(c.g[V0][0])) as u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
            if !more {
                break 'loop_segments;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_6d60);
            }
        }
    }
    c.cycles += 21;
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.g[A0][0] = sx((c.g[T1][0] as u32) << 4);
    c.g[A3][0] = c.lw(p(this, 0x74));
    c.lqc2(3, c.g[V0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(3, p(c.g[SP][0] as u32, 0x10));
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.sq(3, p(this, 0x30)); // start
    c.sq(3, p(c.g[SP][0] as u32, 0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(2, p(c.g[SP][0] as u32, 0x10));
    c.sq(2, p(this, 0x20)); // end
    c.sq(2, p(c.g[SP][0] as u32, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// 0x00347b80 FlexBridge_Render
// ---------------------------------------------------------------------------

/// `int FlexBridge_Render(FlexBridge* self)` (0x347b80).
///
/// Renders the flexing bridge segment: pushes the draw state, asks the
/// renderer's dispatch table for the segment transform, builds the two vertex
/// arrays and submits them, then restores the state. Returns 0.
pub fn flex_bridge_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        frame.save(c, 21, 0x50);
        frame.save(c, 16, 0xa0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save_ra(c, 0x40);

        'ret: {
            c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 1;
            if t {
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(c.g[A2][0] as u32 + c.g[V0][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x24));
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lh(p(c.g[A1][0] as u32, 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx(c.g[V1][0] as u32 + c.g[A2][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3f0));
            c.g[V1][0] = c.g[V1][0] ^ 3;
            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        }

        'body: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0;
            c.g[S4][0] = c.lw(gp(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 19;
            c.g[A0][0] = c.g[SP][0];
            c.g[S2][0] = sx(c.g[S5][0] as u32 + 0x90);
            c.g[S3][0] = sx(c.g[S5][0] as u32 + 0xa0);
            c.g[S0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[S1][0] = c.lh(p(c.g[S0][0] as u32, 0x2e8));
            c.g[A1][0] = c.lh(p(c.g[S0][0] as u32, 0x158));
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x15c));
            c.g[S1][0] = sx(c.g[S4][0] as u32 + c.g[S1][0] as u32);
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_7c28)?;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            let t = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_7c40)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.lq(16, p(c.g[SP][0] as u32, 0xa0));
            if t {
                break 'body;
            }
            'inner: {
                'no_anim: {
                    c.cycles += 28;
                    c.g[V1][0] = c.lw(p(c.g[S5][0] as u32, 0xc));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0xc0));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(c.g[S5][0] as u32 + c.g[A0][0] as u32);
                    call_ptr(c, t, 0x0034_7c60)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = 0xffff_ffff_ffff_ff83;
                    c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    c.ldl(4, p(c.g[V0][0] as u32, 7));
                    c.ldr(4, c.g[V0][0] as u32);
                    c.ldl(7, p(c.g[V0][0] as u32, 0xf));
                    c.ldr(7, p(c.g[V0][0] as u32, 8));
                    c.g[T0][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                    c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                    c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                    c.sdl(7, p(c.g[V0][0] as u32, 0x23));
                    c.sdr(7, p(c.g[V0][0] as u32, 0x1c));
                    c.sw(p(c.g[V0][0] as u32, 0x24), c.g[T0][0]);
                    c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                    c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                    c.g[T1][0] = c.lw(p(c.g[S5][0] as u32, 0x78));
                    let t = c.g[T1][0] == 0;
                    c.g[A1][0] = c.g[A2][0];
                    if t {
                        break 'no_anim;
                    }
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(p(c.g[T1][0] as u32, 4));
                    c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                    c.g[A3][0] = 0;
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
                    c.g[A2][0] = c.lw(p(c.g[T0][0] as u32, 0x94));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(c.g[T1][0] as u32 + c.g[A0][0] as u32);
                    call_ptr(c, t, 0x0034_7cdc)?;
                    let t = true;
                    c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    if t {
                        break 'inner;
                    }
                }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A2][0] = 0;
                c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7d08)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
            break 'body;
        }
        c.cycles += 1;
        c.lq(16, p(c.g[SP][0] as u32, 0xa0));

        c.cycles += 8;
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// Bridge node fields (offsets from `self`).
pub(crate) const NODE_DEF: u32 = 0x18;

// the instance record the mesh is built from
pub(crate) const NODE_COUNT: u32 = 0x50;

// segments; the grid has 2 rows of `count` vertices
pub(crate) const NODE_MESH: u32 = 0x78;

// the grid mesh object (its vt[6] fills the vertices)
pub(crate) const NODE_POS: u32 = 0x7c;

// pts[0]: 2 * count grid vertices (16 bytes each)
pub(crate) const NODE_NRM: u32 = 0x80;

// pts[1]: 2 * count normals (16 bytes each)
pub(crate) const NODE_IDX: u32 = 0x84;

// triangle index bytes, 3 per triangle
pub(crate) const NODE_BOX: u32 = 0x90;

// tBoundBox: min at +0x90, max at +0xa0

// Instance record and world fields.
pub(crate) const DEF_MIN: u32 = 0x60;

// def->min (x, y, z floats)
pub(crate) const DEF_MAX: u32 = 0x6c;

// def->max (x, y, z floats)
pub(crate) const GFX_MAN_GP: u32 = 0x2a90;

// gp-relative: the graphics manager
pub(crate) const GFX_MAN_WORLD: u32 = 0x10d8;

// graphics manager -> world
pub(crate) const WORLD_GRID_MESH_DELTA: u32 = 0x260;

// CreateGridMesh: this delta (halfword) ...
pub(crate) const WORLD_GRID_MESH_FN: u32 = 0x264;

// ... and the function (word)
pub(crate) const WORLD_GET_QUAD_DELTA: u32 = 0x308;

// GetMeshQuad: this delta (halfword) ...
pub(crate) const WORLD_GET_QUAD_FN: u32 = 0x30c;

// ... and the function (word)
pub(crate) const MESH_VT_DELTA: u32 = 0x30;

// mesh vtable slot 6: this delta (halfword) ...
pub(crate) const MESH_VT_FN: u32 = 0x34;

// ... and the vertex-fill function (word)

// Callees (guest addresses).
pub(crate) const OPERATOR_NEW: u32 = 0x0031_7e30;

pub(crate) const OPERATOR_DELETE: u32 = 0x0031_7e98;

pub(crate) const WORLD_GET_NASH: u32 = 0x002d_1be0;

pub(crate) const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;

/// Named blocks of the allocation names (`str_BridgeVerts`, ...): the
/// generated code passes them as `0x490000 - k`.
pub(crate) const NAME_BASE: u32 = 0x0049_0000;

/// `cFlexBridgeNode_setupGrid` (`FlexBridge_BuildMesh`, 0x3466a0): builds the
/// grid mesh of a flex bridge node. `count` segments give `2 * count`
/// vertices (two rows, one per side of the bridge) and `2 * (count - 1)`
/// triangles. The vertices and normals are interpolated from the two end
/// quads of the instance record (`GetMeshQuad`); the grid mesh fills its
/// vertices through vtable slot 6; the bounding box is the record's box
/// grown by 100 on every side; the spatial hash is updated with the box
/// before and after.
pub fn flex_bridge_build_mesh(c: &mut Cpu) {
    guest(c, build_mesh);
}

pub(crate) fn build_mesh(c: &mut Cpu) -> Flow {
    c.cycles += 42;
    let frame = Frame::open(c, 0x1b0);
    let stk = sp(c);
    let slot = |off: u32| stk.wrapping_add(off);
    c.g[A2][0] = 0x2;
    frame.save(c, S0, 0x190);
    c.g[A3][0] = 0x2000_0000;
    frame.save(c, S1, 0x180);
    frame.save(c, S2, 0x170);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S3, 0x160);
    frame.save(c, S5, 0x140);
    c.g[S3][0] = sx(r32(c, S1).wrapping_add(NODE_POS));
    frame.save(c, FP, 0x110);
    frame.save(c, S6, 0x130);
    frame.save(c, S7, 0x120);
    frame.save_ra(c, 0x100);
    frame.save_fpr(c, 21, 0x1a8);
    frame.save_fpr(c, 20, 0x1a0);
    frame.save(c, S4, 0x150);

    // Graphics manager -> world; n = count; nverts = 2n; ntris = 2(n-1).
    c.g[S5][0] = c.lw(r32(c, GP).wrapping_add(GFX_MAN_GP));
    c.g[S4][0] = c.lw(r32(c, S1).wrapping_add(NODE_COUNT));
    c.g[V1][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    c.g[V0][0] = sx(r32(c, S4).wrapping_sub(1));
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(WORLD_GRID_MESH_DELTA));
    c.g[S2][0] = sx(r32(c, V0) << 1);
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(WORLD_GRID_MESH_FN));
    c.g[S0][0] = sx(r32(c, S4) << 1);
    // self->mesh = world->CreateGridMesh(this, n, 2, 0x20000000)
    let create = r32(c, V0);
    c.g[RA][0] = sx(0x0034_6710);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, create, 0x0034_6710)?;
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A2][0] = 0x2000_0000;
    c.sw(r32(c, S1).wrapping_add(NODE_MESH), c.g[V0][0]);
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1888));
    c.g[A0][0] = sx(r32(c, S4) << 5);
    c.g[A3][0] = 0;
    // pts[0] = new sGridV4[2n]
    call_ptr(c, OPERATOR_NEW, 0x0034_672c)?;

    c.g[V1][0] = sx(slot(0x50));
    c.g[A2][0] = sx(slot(0x90));
    c.sw(slot(0xf4), c.g[V1][0]);
    c.g[FP][0] = sx(slot(0x30));
    c.sw(slot(0xfc), c.g[A2][0]);
    let s0_is_zero = c.g[S0][0] == 0;
    c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
    if !s0_is_zero {
        c.cycles += 2;
        c.g[A0][0] = u64::MAX;
    }

    // pts[0] stored in self; pts[1] = new sGridV4[2n]; idx = new u8[3 * ntris].
    c.cycles += 20;
    c.g[A0][0] = sx(r32(c, S0) << 4);
    c.g[A1][0] = NAME_BASE as u64;
    c.sw(r32(c, S3), c.g[V0][0]);
    c.g[S7][0] = c.g[A0][0];
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1878));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[S6][0] = sx(r32(c, S0) << 3);
    call_ptr(c, OPERATOR_NEW, 0x0034_6774)?;
    c.g[S3][0] = sx(r32(c, S1).wrapping_add(NODE_NRM));
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0).wrapping_sub(1));
    c.g[T3][0] = sx(r32(c, S1).wrapping_add(NODE_BOX));
    c.g[T4][0] = sx(r32(c, S2).wrapping_sub(1));
    c.g[A1][0] = sx(r32(c, S2) << 1);
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = c.g[A0][0];
    c.sw(slot(0xf0), c.g[T3][0]);
    c.sw(slot(0xf8), c.g[T4][0]);

    c.cycles += 14;
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, S2)));
    c.sw(r32(c, S3), c.g[V1][0]);
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A2][0] = 0x2000_0000;
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1868));
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_67bc)?;
    c.sw(r32(c, S1).wrapping_add(NODE_IDX), c.g[V0][0]);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0x3;

    // for (v0 = 3; v0 != -1; v0--) { 7 cycles }
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = NAME_BASE as u64;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_67d8);
        }
    }

    // tmp uv = new sGridV2[2n]
    c.cycles += 9;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1858));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_6808)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.g[S0][0];

    // tmp col = new sGridCol[2n]
    c.cycles += 11;
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1848));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_6830)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = c.g[S0][0];
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] == c.g[V0][0] {
        c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    }

    // world->GetMeshQuad(def, 0, pos, uvs, cols)
    c.cycles += 11;
    c.g[A3][0] = c.g[SP][0];
    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    c.g[T0][0] = c.g[FP][0];
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(WORLD_GET_QUAD_DELTA));
    c.g[A2][0] = 0;
    c.g[T1][0] = c.lw(slot(0xf4));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(WORLD_GET_QUAD_FN));
    let get_quad = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, get_quad, 0x0034_6870)?;

    let no_quads = (c.g[S4][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !no_quads {
        // Per segment i: the two rows of vertices, normals and colours, interpolated
        // between the end quads with t = i / (n - 1).
        c.cycles += 14;
        c.g[V0][0] = sx(r32(c, S4).wrapping_sub(1));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(r32(c, S4) << 4);
        c.g[V0][0] = sx(r32(c, S4) << 3);
        c.g[T2][0] = c.g[V1][0];
        c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
        c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
        c.div_s(20, 21, 0);
        c.g[A1][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S3][0];
        loop {
            c.cycles += 179;
            c.lwc1(9, slot(0x0));
            c.mtc1(7, 6);
            c.cvt_s_w(6, 6);
            c.lwc1(0, slot(0xc));
            c.g[V1][0] = sx(r32(c, A3) << 4);
            c.lwc1(8, slot(0x4));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.lwc1(7, slot(0x8));
            c.sub_s(0, 0, 9);
            c.lwc1(1, slot(0x10));
            c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.lwc1(2, slot(0x14));
            c.swc1(21, slot(0x9c));
            c.sub_s(1, 1, 8);
            c.sub_s(2, 2, 7);
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_POS));
            c.mul_s(3, 0, 6);
            c.swc1(0, slot(0xd0));
            c.mul_s(4, 1, 6);
            c.mul_s(0, 2, 6);
            c.swc1(1, slot(0xd4));
            c.swc1(2, slot(0xd8));
            c.mul_s(5, 3, 20);
            c.mul_s(1, 4, 20);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.mul_s(2, 0, 20);
            c.swc1(3, slot(0xc0));
            c.add_s(9, 9, 5);
            c.swc1(4, slot(0xc4));
            c.add_s(8, 8, 1);
            c.swc1(0, slot(0xc8));
            c.add_s(7, 7, 2);
            c.swc1(5, slot(0xb0));
            c.swc1(9, slot(0x90));
            c.swc1(8, slot(0x94));
            c.swc1(7, slot(0x98));
            c.swc1(1, slot(0xb4));
            c.lq(2, slot(0x90));
            c.swc1(2, slot(0xb8));
            c.swc1(9, slot(0xa0));
            c.swc1(8, slot(0xa4));
            c.swc1(7, slot(0xa8));
            c.sq(2, r32(c, V1));
            c.lwc1(5, slot(0x18));
            c.lwc1(7, slot(0x1c));
            c.lwc1(8, slot(0x20));
            c.lwc1(0, slot(0x24));
            c.lwc1(1, slot(0x28));
            c.lwc1(2, slot(0x2c));
            c.sub_s(0, 0, 5);
            c.sub_s(1, 1, 7);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_POS));
            c.sub_s(2, 2, 8);
            c.mul_s(3, 0, 6);
            c.swc1(0, slot(0xe0));
            c.mul_s(4, 1, 6);
            c.swc1(1, slot(0xe4));
            c.mul_s(10, 2, 6);
            c.swc1(21, slot(0x9c));
            c.mul_s(0, 3, 20);
            c.swc1(3, slot(0xc0));
            c.mul_s(1, 4, 20);
            c.swc1(4, slot(0xc4));
            c.mul_s(9, 10, 20);
            c.g[V1][0] = sx(r32(c, T2).wrapping_add(r32(c, V1)));
            c.add_s(5, 5, 0);
            c.swc1(0, slot(0xb0));
            c.add_s(7, 7, 1);
            c.swc1(1, slot(0xb4));
            c.add_s(8, 8, 9);
            c.swc1(2, slot(0xe8));
            c.swc1(5, slot(0x90));
            c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x10));
            c.swc1(7, slot(0x94));
            c.swc1(8, slot(0x98));
            c.swc1(5, slot(0xa0));
            c.lq(2, slot(0x90));
            c.swc1(7, slot(0xa4));
            c.swc1(10, slot(0xc8));
            c.swc1(9, slot(0xb8));
            c.swc1(8, slot(0xa8));
            c.sq(2, r32(c, V1));
            c.lwc1(5, slot(0x30));
            c.lwc1(7, slot(0x34));
            c.lwc1(0, slot(0x38));
            c.lwc1(1, slot(0x3c));
            c.sub_s(0, 0, 5);
            c.lwc1(15, slot(0x40));
            c.sub_s(1, 1, 7);
            c.lwc1(4, slot(0x48));
            c.lwc1(14, slot(0x44));
            c.mul_s(2, 0, 6);
            c.swc1(0, slot(0xc0));
            c.mul_s(3, 1, 6);
            c.swc1(1, slot(0xc4));
            c.sub_s(4, 4, 15);
            c.lwc1(10, slot(0x4c));
            c.mul_s(0, 2, 20);
            c.swc1(2, slot(0xb0));
            c.mul_s(1, 3, 20);
            c.swc1(3, slot(0xb4));
            c.sub_s(10, 10, 14);
            c.add_s(5, 5, 0);
            c.swc1(0, slot(0xa0));
            c.add_s(7, 7, 1);
            c.swc1(1, slot(0xa4));
            c.mul_s(16, 4, 6);
            c.swc1(5, slot(0x90));
            c.mul_s(17, 10, 6);
            c.swc1(7, slot(0x94));
            c.mul_s(18, 16, 20);
            c.ldl(2, slot(0x97));
            c.ldr(2, slot(0x90));
            c.sdl(2, r32(c, T0).wrapping_add(0x7));
            c.sdr(2, r32(c, T0));
            c.mul_s(19, 17, 20);
            c.swc1(4, slot(0xc0));
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x8));
            c.lwc1(11, slot(0x54));
            c.add_s(15, 15, 18);
            c.lwc1(12, slot(0x58));
            c.add_s(14, 14, 19);
            c.lwc1(13, slot(0x5c));
            c.lwc1(3, slot(0x64));
            c.lwc1(4, slot(0x68));
            c.lwc1(5, slot(0x6c));
            c.sub_s(3, 3, 11);
            c.lwc1(8, slot(0x74));
            c.sub_s(4, 4, 12);
            c.lwc1(9, slot(0x78));
            c.sub_s(5, 5, 13);
            c.lwc1(7, slot(0x7c));
            c.mul_s(3, 3, 6);
            c.lwc1(0, slot(0x84));
            c.mul_s(4, 4, 6);
            c.lwc1(1, slot(0x88));
            c.mul_s(5, 5, 6);
            c.lwc1(2, slot(0x8c));
            c.sub_s(0, 0, 8);
            c.sub_s(1, 1, 9);
            c.swc1(15, slot(0x90));
            c.sub_s(2, 2, 7);
            c.swc1(14, slot(0x94));
            c.mul_s(0, 0, 6);
            c.swc1(10, slot(0xc4));
            c.mul_s(1, 1, 6);
            c.swc1(16, slot(0xb0));
            c.mul_s(2, 2, 6);
            c.swc1(17, slot(0xb4));
            c.mul_s(3, 3, 20);
            c.swc1(18, slot(0xa0));
            c.mul_s(4, 4, 20);
            c.swc1(19, slot(0xa4));
            c.mul_s(5, 5, 20);
            c.swc1(21, r32(c, A1));
            c.mul_s(0, 0, 20);
            c.ldl(2, slot(0x97));
            c.ldr(2, slot(0x90));
            c.sdl(2, r32(c, T1).wrapping_add(0x7));
            c.sdr(2, r32(c, T1));
            c.mul_s(1, 1, 20);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x8));
            c.mul_s(2, 2, 20);
            c.add_s(11, 11, 3);
            c.add_s(12, 12, 4);
            c.add_s(13, 13, 5);
            c.add_s(8, 8, 0);
            c.swc1(11, r32(c, A1).wrapping_add(0x4));
            c.add_s(9, 9, 1);
            c.swc1(12, r32(c, A1).wrapping_add(0x8));
            c.add_s(7, 7, 2);
            c.swc1(13, r32(c, A1).wrapping_add(0xc));
            c.swc1(8, r32(c, A2).wrapping_add(0x4));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            c.swc1(9, r32(c, A2).wrapping_add(0x8));
            c.swc1(7, r32(c, A2).wrapping_add(0xc));
            c.swc1(21, r32(c, A2));
            let more = c.g[A0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_68b0);
            }
        }
    }

    // world->vt[...]: the mesh fills its vertices (vt[6]), then the temporaries go.
    c.cycles += 11;
    c.g[T0][0] = c.lw(r32(c, S1).wrapping_add(NODE_MESH));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[V0][0] = c.lw(r32(c, T0).wrapping_add(0x4));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(MESH_VT_DELTA));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(MESH_VT_FN));
    let fill = r32(c, V1);
    c.g[RA][0] = sx(0x0034_6ba0);
    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
    call_ptr(c, fill, 0x0034_6ba0)?;
    if c.g[S3][0] != 0 {
        c.cycles += 2;
        c.g[RA][0] = sx(0x0034_6bb0);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, OPERATOR_DELETE, 0x0034_6bb0)?;
    }

    c.cycles += 2;
    if c.g[S2][0] == 0 {
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0034_6bc0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, OPERATOR_DELETE, 0x0034_6bc0)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    }

    // box = def's min/max (w = 1), grown by 100 on every side; `old` keeps the
    // unexpanded box for the spatial hash.
    c.cycles += 49;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MIN));
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(1, 2);
    c.swc1(1, slot(0x90));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MIN + 0x4));
    c.swc1(0, slot(0x94));
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MIN + 0x8));
    c.swc1(3, slot(0x9c));
    c.swc1(1, slot(0x98));
    c.lq(3, slot(0x90));
    c.sq(3, r32(c, S1).wrapping_add(NODE_BOX));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MAX));
    c.swc1(0, slot(0x90));
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MAX + 0x4));
    c.swc1(1, slot(0x94));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MAX + 0x8));
    c.swc1(3, slot(0x9c));
    c.swc1(0, slot(0x98));
    c.swc1(2, slot(0xb8));
    c.lq(2, slot(0x90));
    c.sq(3, slot(0x90));
    c.sq(2, slot(0xa0));
    c.swc1(2, slot(0xb0));
    c.swc1(2, slot(0xb4));
    c.sw(slot(0xbc), 0);
    c.sq(2, r32(c, S1).wrapping_add(NODE_BOX + 0x10));
    c.lqc2(3, r32(c, S1).wrapping_add(NODE_BOX));
    c.lqc2(4, slot(0xb0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, S1).wrapping_add(NODE_BOX));
    c.lqc2(3, r32(c, S1).wrapping_add(NODE_BOX + 0x10));
    c.lqc2(4, slot(0xb0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, S1).wrapping_add(NODE_BOX + 0x10));

    // The spatial hash: nash = world->GetNash(), update with (def, box, old).
    c.g[RA][0] = sx(0x0034_6c5c);
    c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    call_ptr(c, WORLD_GET_NASH, 0x0034_6c5c)?;
    c.g[A3][0] = c.lw(slot(0xf0));
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(slot(0xfc));
    c.g[A2][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x0034_6c74);
    c.g[A1][0] = 0;
    call_ptr(c, SPATIAL_HASH_UPDATE, 0x0034_6c74)?;

    // Index buffer: two triangles per quad strip, indices (base, base+n, base+1)
    // and (base+n, base+n+1, base+1).
    c.g[V0][0] = c.lw(slot(0xf8));
    c.g[T2][0] = 0;
    c.g[A3][0] = 0;
    let no_tris = (c.g[V0][0] as i64) <= 0;
    c.g[T1][0] = 0;
    if !no_tris {
        c.cycles += 2;
        c.g[T0][0] = sx(r32(c, S4).wrapping_add(0x1));
        loop {
            c.cycles += 27;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[A1][0] = sx(r32(c, T2).wrapping_add(r32(c, S4)));
            c.g[V1][0] = c.lw(slot(0xf8));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x2));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[A0][0] = sx(r32(c, T2).wrapping_add(0x1));
            c.sb(r32(c, V0), c.g[T2][0]);
            c.g[A2][0] = ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[T2][0] = c.g[A0][0];
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x1), c.g[A1][0]);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
            c.sb(r32(c, V1).wrapping_add(0x2), c.g[A0][0]);
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x3), c.g[A1][0]);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
            c.sb(r32(c, V1).wrapping_add(0x4), c.g[T0][0]);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x1));
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x5), c.g[A0][0]);
            let more = c.g[A2][0] != 0;
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x6));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_6c90);
            }
        }
    }

    // Epilogue.
    c.cycles += 14;
    frame.restore(c, S0, 0x190);
    frame.restore(c, S1, 0x180);
    frame.restore(c, S2, 0x170);
    frame.restore(c, S3, 0x160);
    frame.restore(c, S4, 0x150);
    frame.restore(c, S5, 0x140);
    frame.restore(c, S6, 0x130);
    frame.restore(c, S7, 0x120);
    frame.restore(c, FP, 0x110);
    frame.restore_ra(c, 0x100);
    frame.restore_fpr(c, 21, 0x1a8);
    frame.restore_fpr(c, 20, 0x1a0);
    frame.close(c);
    Ok(())
}

/// `cFlexBridgeNode` fields the update touches.
pub(crate) const FB_WIDTH: u32 = 0x50;

// Mesh.Width (spring node count)
pub(crate) const FB_NODES: u32 = 0x74;

// Mesh.Nodes, tSpringNode[Width]
pub(crate) const FB_NODE_SIZE: u32 = 0x50;

pub(crate) const FB_NODE_P: u32 = 0x00;

// node->P (vec4)
pub(crate) const FB_NODE_O: u32 = 0x20;

// node->O (the sway angle)
pub(crate) const FB_INSTANCE: u32 = 0x18;

// Nash object id
pub(crate) const FB_BOTTOM_LEFT: u32 = 0x20;

// BottomLeftOffset (vec4)
pub(crate) const FB_BOTTOM_RIGHT: u32 = 0x30;

// BottomRightOffset (vec4)
pub(crate) const FB_ROOT_VPTR: u32 = 0x0c;

// vtable of getRootTransform
pub(crate) const FB_GRID_MESH: u32 = 0x78;

// GridMesh object (vtable slot 3)
pub(crate) const FB_VERTS: u32 = 0x7c;

// Verts, vec4[2 * Width]
pub(crate) const FB_FACE_NORMS: u32 = 0x80;

// FaceNorms, vec4[2 * quads]
pub(crate) const FB_BOUND_MIN: u32 = 0x90;

// BoundBox.MinPoint
pub(crate) const FB_BOUND_MAX: u32 = 0xa0;

// BoundBox.MaxPoint
pub(crate) const FB_FRAME: u32 = 0x2d0;

// stack frame size

/// `void FlexBridge_UpdateVerts(cFlexBridgeNode* self)` (0x3475d8),
/// `cFlexBridgeNode::updateGrid`. Places the bridge's two rows of vertices
/// from its spring nodes: each node's `BottomRight` / `BottomLeft` offset
/// is turned by the node's angle `O` (`cos`) and lifted by the up / side
/// reference (`sin`). Then it builds the two face normals of every quad,
/// takes the bounding box of the verts after the root transform, stores it
/// in `BoundBox`, moves the object in the Nash spatial hash
/// (`nash_movenode(World_GetNash(), 0, Instance, &BoundBox, &old)`) and
/// hands the vertex positions to the grid mesh (vtable slot 3).
///
/// The stack frame (0x2d0 bytes): the references at +0x0 / +0x10 / +0x20..,
/// verts scratch at +0x20..+0x70, the sin/cos pair at +0x1f0 / +0x1f4, and
/// `pos[]` (12-byte points) at +0x70. Every register and cycle count follows
/// the generated body (VU0 macro ops are issued as the original does).
pub fn flexbridge_update_verts(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: save ra, s0-s7, fp and f20-f25 (`sq` is 128-bit).
        c.cycles += 62;
        c.g[SP][0] = sx(sp(c).wrapping_sub(FB_FRAME));
        c.sq(17, sp(c).wrapping_add(0x280));
        c.sq(18, sp(c).wrapping_add(0x270));
        c.g[S1][0] = c.g[A0][0];
        c.sq(21, sp(c).wrapping_add(0x240));
        c.g[A0][0] = 0x500000;
        c.sq(22, sp(c).wrapping_add(0x230));
        c.g[S2][0] = 0;
        c.sq(23, sp(c).wrapping_add(0x220));
        c.sq(30, sp(c).wrapping_add(0x210));
        c.g[S7][0] = sx(sp(c).wrapping_add(0x20));
        c.sq(16, sp(c).wrapping_add(0x290));
        c.g[FP][0] = sx(sp(c).wrapping_add(0x70));
        c.sq(20, sp(c).wrapping_add(0x250));
        c.sd(sp(c).wrapping_add(0x200), c.g[RA][0]);
        c.swc1(25, sp(c).wrapping_add(0x2c8));
        c.swc1(24, sp(c).wrapping_add(0x2c0));
        c.swc1(23, sp(c).wrapping_add(0x2b8));
        c.swc1(22, sp(c).wrapping_add(0x2b0));
        c.swc1(21, sp(c).wrapping_add(0x2a8));
        c.swc1(20, sp(c).wrapping_add(0x2a0));
        c.sq(19, sp(c).wrapping_add(0x260));

        // n = Width. upRef = ZAxis * |BottomRight| (at sp+0), sideRef =
        // ZAxis * |BottomLeft| (at sp+0x10), both through VU0.
        c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(FB_WIDTH));
        c.lqc2(3, r32(c, S1).wrapping_add(FB_BOTTOM_RIGHT));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, A0).wrapping_sub(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lq(2, sp(c).wrapping_add(0x10));

        c.lqc2(3, r32(c, S1).wrapping_add(FB_BOTTOM_LEFT));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(5, 22);
        c.mtc1(5, 0);
        c.g[V1][0] = sx(r32(c, S3).wrapping_sub(1));
        c.g[S6][0] = sx(r32(c, S3) << 1);
        c.sq(2, sp(c));
        c.g[S5][0] = sx(r32(c, V1) << 1);
        c.mfc1(2, 0);
        c.lqc2(4, r32(c, A0).wrapping_sub(0xea0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(FB_NODES));
        let no_nodes = (c.g[S3][0] as i64) <= 0;
        c.sq(2, sp(c).wrapping_add(0x10));

        // Vertex rows: verts[i] = P + BR*cos + upRef*sin and
        // verts[n + i] = P + BL*cos - sideRef*sin, for each node i.
        if !no_nodes {
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S3) << 4);
            loop {
                c.cycles += 72;
                c.lwc1(12, r32(c, S0).wrapping_add(FB_NODE_O));
                c.g[A0][0] = sx(sp(c).wrapping_add(0x1f0));
                c.g[RA][0] = sx(0x0034_76e8);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x1f4));
                // fSinCos(&sin, &cos, node->O)
                call_ptr(c, 0x0031_be50, 0x0034_76e8)?;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A0][0] = sx(r32(c, S2) << 4);
                c.lwc1(0, sp(c).wrapping_add(0x1f4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1));
                c.mfc1(2, 0);
                c.lqc2(4, r32(c, S1).wrapping_add(FB_BOTTOM_RIGHT));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[A2][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.sq(2, sp(c).wrapping_add(0x40));
                c.lqc2(3, r32(c, S0).wrapping_add(FB_NODE_P));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.lwc1(0, sp(c).wrapping_add(0x1f0));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.lqc2(4, sp(c).wrapping_add(0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                c.sq(2, r32(c, A0));
                c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.lwc1(0, sp(c).wrapping_add(0x1f4));
                c.mfc1(2, 0);
                c.lqc2(4, r32(c, S1).wrapping_add(FB_BOTTOM_LEFT));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(0x10));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.lqc2(3, r32(c, S0).wrapping_add(FB_NODE_P));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FB_NODE_SIZE));
                c.lwc1(0, sp(c).wrapping_add(0x1f0));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.lqc2(4, sp(c).wrapping_add(0x50));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                let more = c.g[A2][0] != 0;
                c.sq(2, r32(c, A1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_76d8);
                }
            }
        }

        // Face normals: for each quad j (k = 2j, ntris = 2 * (n - 1)),
        // normals of the two triangles, normalised (rsqrt).
        c.cycles += 4;
        c.g[T3][0] = 0;
        c.g[T2][0] = 0;
        let no_tris = (c.g[S5][0] as i64) <= 0;
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(FB_BOUND_MIN));
        if !no_tris {
            c.cycles += 2;
            c.g[T1][0] = sx(r32(c, S3) << 4);
            loop {
                c.cycles += 89;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A1][0] = sx(r32(c, T3) << 4);
                c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(FB_FACE_NORMS));
                c.g[A3][0] = sx(r32(c, T2) << 4);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x2));
                c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
                c.g[A2][0] = sx(r32(c, A3).wrapping_add(r32(c, A2)));
                c.lqc2(3, r32(c, V0).wrapping_add(0x10));
                c.lqc2(4, r32(c, V1));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[T0][0] = ((c.g[T2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
                c.lqc2(3, r32(c, V1));
                c.lqc2(4, r32(c, A0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.lqc2(5, sp(c).wrapping_add(0x50));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                c.sq(2, r32(c, A2));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(FB_FACE_NORMS));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[V1][0] = c.g[V0][0];
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                c.lqc2(3, r32(c, V1).wrapping_add(0x10));
                c.lqc2(4, r32(c, V0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
                c.lqc2(3, r32(c, V1).wrapping_add(0x10));
                c.lqc2(4, r32(c, A1));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x10));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.lqc2(5, sp(c).wrapping_add(0x50));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                let more = c.g[T0][0] != 0;
                c.sq(2, r32(c, A3).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7810);
                }
            }
        }

        // Bounds: the verts (3 floats each, into pos[] at sp+0x70) through
        // the root transform (`getRootTransform`, virtual at this+0xc);
        // min / max per axis in f22 / f20 / f23 (min) and f24 / f21 / f25 (max).
        c.cycles += 16;
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(FB_ROOT_VPTR));
        c.g[V0][0] = 0x1f;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(0xc0));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xc4));
        c.lwc1(20, r32(c, GP).wrapping_sub(0x2b3c));
        c.lwc1(21, r32(c, GP).wrapping_sub(0x2b38));
        let root_fn = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, root_fn, 0x0034_7998)?;
        c.g[A2][0] = c.g[V0][0];
        c.mov_s(22, 20);
        c.mov_s(23, 20);
        c.mov_s(24, 21);
        c.mov_s(25, 21);
        let no_verts = (c.g[S6][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !no_verts {
            c.cycles += 1;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x70));
            loop {
                // Copy vert i (x, y, z) to pos[i], transform it by the root
                // matrix (A2), and fold its x / y / z into the min / max.
                c.cycles += 27;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[V1][0] = sx(r32(c, A1) << 4);
                c.mov_s(2, 22);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V1));
                c.swc1(0, r32(c, A0));
                c.lwc1(1, r32(c, V1).wrapping_add(0x4));
                c.swc1(1, r32(c, A0).wrapping_add(0x4));
                c.lwc1(0, r32(c, V1).wrapping_add(0x8));
                c.swc1(0, r32(c, A0).wrapping_add(0x8));
                c.lqc2(8, r32(c, V1));
                c.lqc2(4, r32(c, A2));
                c.lqc2(5, r32(c, A2).wrapping_add(0x10));
                c.lqc2(6, r32(c, A2).wrapping_add(0x20));
                c.lqc2(7, r32(c, A2).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp(c).wrapping_add(0x30));
                c.lq(2, sp(c).wrapping_add(0x30));
                c.sq(2, sp(c).wrapping_add(0x20));
                // minX: w.x < minX ? w.x : minX (f2 holds the candidate)
                c.lwc1(0, sp(c).wrapping_add(0x20));
                c.c_lt_s(0, 22);
                if c.fcc() {
                    c.mov_s(2, 0);
                }
                // maxX: maxX < w.x ? w.x : maxX
                c.cycles += 4;
                c.c_lt_s(24, 0);
                c.mov_s(22, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 24);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                // minY
                c.cycles += 7;
                c.lwc1(0, sp(c).wrapping_add(0x24));
                c.mov_s(24, 1);
                c.mov_s(2, 20);
                c.c_lt_s(0, 20);
                let not_lt = !c.fcc();
                c.mov_s(1, 0);
                if !not_lt {
                    c.cycles += 1;
                    c.mov_s(2, 1);
                }
                // maxY
                c.cycles += 4;
                c.c_lt_s(21, 0);
                c.mov_s(20, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 21);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                // minZ
                c.cycles += 7;
                c.lwc1(0, sp(c).wrapping_add(0x28));
                c.mov_s(21, 1);
                c.mov_s(2, 23);
                c.c_lt_s(0, 23);
                let not_lt = !c.fcc();
                c.mov_s(1, 0);
                if !not_lt {
                    c.cycles += 1;
                    c.mov_s(2, 1);
                }
                // maxZ
                c.cycles += 4;
                c.c_lt_s(25, 0);
                c.mov_s(23, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 25);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                c.cycles += 5;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1));
                c.mov_s(25, 1);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_79b8);
                }
            }
        }

        // Store the box: corner = (min, 1.0) into BoundBox.MinPoint, and
        // (max, 1.0) into MaxPoint; the old box is copied to the stack first.
        c.cycles += 50;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.swc1(22, sp(c).wrapping_add(0x40));
        c.swc1(20, sp(c).wrapping_add(0x44));
        c.swc1(23, sp(c).wrapping_add(0x48));
        c.swc1(0, sp(c).wrapping_add(0x4c));
        c.lq(4, r32(c, S1).wrapping_add(FB_BOUND_MIN));
        c.lq(5, sp(c).wrapping_add(0x40));
        c.swc1(24, sp(c).wrapping_add(0x40));
        c.swc1(21, sp(c).wrapping_add(0x44));
        c.swc1(25, sp(c).wrapping_add(0x48));
        c.lq(2, r32(c, S1).wrapping_add(FB_BOUND_MAX));
        c.sq(4, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.sq(3, r32(c, S1).wrapping_add(FB_BOUND_MAX));
        c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(FB_INSTANCE));
        c.sq(5, r32(c, S1).wrapping_add(FB_BOUND_MIN));

        // nash_movenode(World_GetNash(), 0, Instance, &BoundBox, &old)
        call_ptr(c, 0x002d_1be0, 0x0034_7b00)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S7][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0032_91e0, 0x0034_7b18)?;

        // gridMesh->vtable[3](gridMesh + delta, pos)
        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(FB_GRID_MESH));
        c.g[A1][0] = c.g[FP][0];
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x18));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
        let grid_fn = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, grid_fn, 0x0034_7b34)?;

        // Epilogue: restore s0-s7 / fp / f20-f25 and ra, pop the frame.
        c.lq(16, sp(c).wrapping_add(0x290));
        c.lq(17, sp(c).wrapping_add(0x280));
        c.lq(18, sp(c).wrapping_add(0x270));
        c.lq(19, sp(c).wrapping_add(0x260));
        c.lq(20, sp(c).wrapping_add(0x250));
        c.lq(21, sp(c).wrapping_add(0x240));
        c.lq(22, sp(c).wrapping_add(0x230));
        c.lq(23, sp(c).wrapping_add(0x220));
        c.lq(30, sp(c).wrapping_add(0x210));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x200));
        c.lwc1(25, sp(c).wrapping_add(0x2c8));
        c.lwc1(24, sp(c).wrapping_add(0x2c0));
        c.lwc1(23, sp(c).wrapping_add(0x2b8));
        c.lwc1(22, sp(c).wrapping_add(0x2b0));
        c.lwc1(21, sp(c).wrapping_add(0x2a8));
        c.lwc1(20, sp(c).wrapping_add(0x2a0));
        let t = r32(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(FB_FRAME));
        c.pc = t;
        Ok(())
    })
}

/// The 32-bit address `r + off` (the generated `(c.g[r][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn fb_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `void FlexBridgeNode::applyForce(const tIntersectInfo &, const t4Vector &)`
/// (0x346e38): skipped when the weight at `intInfo+0x60` is negative; otherwise
/// transforms the force by the node's matrix (vtable call on `this+0xc`).
pub fn flex_bridge_apply_force(c: &mut Cpu) {
    guest(c, apply_force)
}

/// `void FlexBridgeNode::updateIntersectInfo(tIntersectInfo &)` (0x3470d0).
pub fn flex_bridge_update_intersect_info(c: &mut Cpu) {
    guest(c, update_intersect_info)
}

/// `int FlexBridgeNode::intersect(cWorldIntersectType &, tIntersectInfo *, int, int)` (0x347268).
pub fn flex_bridge_intersect(c: &mut Cpu) {
    guest(c, intersect)
}

pub(crate) fn apply_force(c: &mut Cpu) -> Flow {
    'b_3470b4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.sq(16, fb_ea(c, SP, 0xd0));
        c.sq(17, fb_ea(c, SP, 0xc0));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, fb_ea(c, SP, 0xb0));
        c.g[S1][0] = c.g[A0][0];
        c.sd(fb_ea(c, SP, 0xa0), c.g[RA][0]);
        c.g[V0][0] = c.lw(fb_ea(c, S0, 0x60));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = c.g[A2][0];
        if t {
            break 'b_3470b4;
        } else {
            c.cycles += 149;
            c.g[V1][0] = c.lw(fb_ea(c, S1, 0xc));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00346e74);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00346e74)?;
            c.lq(3, fb_ea(c, V0, 0x30));
            c.g[T0][0] = 0x50;
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(3, c.g[S0][0] as u32);
            c.lqc2(4, fb_ea(c, SP, 0x10));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, c.g[SP][0] as u32);
            c.lqc2(3, fb_ea(c, S1, 0x30));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x20));
            c.lqc2(8, fb_ea(c, SP, 0x20));
            c.lqc2(4, c.g[V0][0] as u32);
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0x60));
            c.g[A1][0] = c.lw(fb_ea(c, S1, 0x80));
            c.lq(4, fb_ea(c, SP, 0x30));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sq(4, fb_ea(c, SP, 0x10));
            c.lqc2(8, c.g[V1][0] as u32);
            c.lqc2(4, c.g[V0][0] as u32);
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.lq(4, fb_ea(c, SP, 0x30));
            c.g[A1][0] = c.lw(fb_ea(c, S0, 0x60));
            c.sq(4, fb_ea(c, SP, 0x20));
            c.lqc2(3, c.g[SP][0] as u32);
            c.lqc2(5, fb_ea(c, SP, 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 8);
            c.g[V1][0] = sx((c.g[A1][0] as u32) >> 31);
            c.lq(4, fb_ea(c, V0, 0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = c.lw(fb_ea(c, S1, 0x74));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
            c.lwc1(0, fb_ea(c, S2, 0xc));
            c.mult(0, 3, 5, 8);
            c.lwc1(7, fb_ea(c, S1, 0x64));
            c.sq(4, fb_ea(c, SP, 0x80));
            c.mult(0, 7, 5, 8);
            c.mfc1(8, 0);
            c.lqc2(4, fb_ea(c, SP, 0x80));
            c.qmtc2(8, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x90));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.lq(3, fb_ea(c, SP, 0x90));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sq(3, fb_ea(c, SP, 0x70));
            c.lqc2(3, c.g[S2][0] as u32);
            c.lqc2(4, fb_ea(c, SP, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x90));
            c.lq(3, fb_ea(c, SP, 0x90));
            c.lwc1(6, fb_ea(c, S2, 0xc));
            c.sq(3, fb_ea(c, SP, 0x60));
            c.lwc1(1, c.g[V0][0] as u32);
            c.lwc1(2, fb_ea(c, V0, 0x4));
            c.lwc1(4, fb_ea(c, SP, 0x60));
            c.lwc1(5, fb_ea(c, SP, 0x64));
            c.mul_s(1, 1, 4);
            c.lwc1(0, fb_ea(c, V0, 0x8));
            c.mul_s(2, 2, 5);
            c.lwc1(3, fb_ea(c, SP, 0x68));
            c.mul_s(0, 0, 3);
            c.add_s(1, 1, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, SP, 0x50));
            c.lwc1(0, fb_ea(c, V0, 0x10));
            c.lwc1(2, fb_ea(c, V0, 0x14));
            c.mul_s(0, 0, 4);
            c.lwc1(1, fb_ea(c, V0, 0x18));
            c.mul_s(2, 2, 5);
            c.mul_s(1, 1, 3);
            c.add_s(0, 0, 2);
            c.add_s(0, 0, 1);
            c.swc1(0, fb_ea(c, SP, 0x54));
            c.lwc1(1, fb_ea(c, V0, 0x20));
            c.lwc1(2, fb_ea(c, V0, 0x24));
            c.mul_s(1, 1, 4);
            c.lwc1(0, fb_ea(c, V0, 0x28));
            c.mul_s(2, 2, 5);
            c.swc1(6, fb_ea(c, SP, 0x5c));
            c.mul_s(0, 0, 3);
            c.add_s(1, 1, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, SP, 0x58));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x40));
            c.mfc1(3, 7);
            c.lqc2(4, fb_ea(c, SP, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x30));
            c.lqc2(3, fb_ea(c, A3, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, A3, 0x30));
            c.lwc1(0, fb_ea(c, S1, 0x68));
            c.lqc2(3, fb_ea(c, SP, 0x20));
            c.lqc2(5, c.g[S2][0] as u32);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 2);
            c.lwc1(1, fb_ea(c, A1, 0x40));
            c.mul_s(0, 0, 2);
            c.mul_s(0, 0, 8);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, A1, 0x40));
            break 'b_3470b4;
        }
    }
    c.cycles += 6;
    c.lq(16, fb_ea(c, SP, 0xd0));
    c.lq(17, fb_ea(c, SP, 0xc0));
    c.lq(18, fb_ea(c, SP, 0xb0));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0xe0));
    c.pc = t;
    Ok(())
}

pub(crate) fn update_intersect_info(c: &mut Cpu) -> Flow {
    'b_34724c: {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(18, fb_ea(c, SP, 0x70));
        c.sq(16, fb_ea(c, SP, 0x90));
        c.g[S2][0] = c.g[A1][0];
        c.sq(17, fb_ea(c, SP, 0x80));
        c.sd(fb_ea(c, SP, 0x60), c.g[RA][0]);
        c.g[V0][0] = c.lw(fb_ea(c, S2, 0x60));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = c.g[A0][0];
        if t {
            break 'b_34724c;
        } else {
            c.cycles += 86;
            c.g[V1][0] = c.lw(fb_ea(c, A1, 0xc));
            c.g[S0][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[S1][0] = c.lw(fb_ea(c, A1, 0x74));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00347118);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00347118)?;
            c.g[V1][0] = 0x50;
            c.mult(0, 16, 16, 3);
            c.g[V1][0] = 0x500000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.lwc1(0, fb_ea(c, S0, 0x24));
            c.mfc1(4, 0);
            c.lqc2(4, c.g[V1][0] as u32);
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(8, fb_ea(c, SP, 0x10));
            c.lqc2(4, c.g[V0][0] as u32);
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, c.g[SP][0] as u32);
            c.lqc2(8, c.g[S0][0] as u32);
            c.lqc2(4, c.g[V0][0] as u32);
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x20));
            c.lqc2(3, c.g[S2][0] as u32);
            c.lqc2(4, fb_ea(c, SP, 0x20));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(8, fb_ea(c, S0, 0x10));
            c.lqc2(4, c.g[V0][0] as u32);
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x40));
            c.lq(2, fb_ea(c, SP, 0x40));
            c.sq(2, fb_ea(c, SP, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x10));
            c.lqc2(5, c.g[SP][0] as u32);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x40));
            c.lqc2(3, fb_ea(c, SP, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x20));
            c.lqc2(3, fb_ea(c, S2, 0x20));
            c.lqc2(4, fb_ea(c, SP, 0x20));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, S2, 0x20));
            c.lqc2(3, fb_ea(c, S2, 0x30));
            c.lqc2(4, c.g[SP][0] as u32);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, S2, 0x30));
            break 'b_34724c;
        }
    }
    c.cycles += 6;
    c.lq(16, fb_ea(c, SP, 0x90));
    c.lq(17, fb_ea(c, SP, 0x80));
    c.lq(18, fb_ea(c, SP, 0x70));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0xa0));
    c.pc = t;
    Ok(())
}

pub(crate) fn intersect(c: &mut Cpu) -> Flow {
    'b_347584: {
        'b_3472a8: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.g[T0][0] = c.g[T0][0] & 0x2;
            c.sq(16, fb_ea(c, SP, 0xf0));
            c.sq(17, fb_ea(c, SP, 0xe0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, fb_ea(c, SP, 0xd0));
            c.g[S1][0] = c.g[A1][0];
            c.sq(20, fb_ea(c, SP, 0xb0));
            c.g[S2][0] = c.g[A3][0];
            c.sq(19, fb_ea(c, SP, 0xc0));
            c.g[S4][0] = c.g[A2][0];
            c.sd(fb_ea(c, SP, 0xa0), c.g[RA][0]);
            let t = c.g[T0][0] != 0u64;
            c.swc1(20, fb_ea(c, SP, 0x100));
            if t {
                break 'b_3472a8;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_347584;
                } else {
                    break 'b_3472a8;
                }
            }
        }
        'b_347580: {
            c.cycles += 93;
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0x50));
            c.g[A0][0] = c.lw(fb_ea(c, S0, 0x7c));
            c.g[A2][0] = c.lw(fb_ea(c, S0, 0x84));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.lw(fb_ea(c, S0, 0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.sh(c.g[SP][0] as u32, c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.sw(fb_ea(c, SP, 0x4), c.g[A2][0]);
            c.sw(fb_ea(c, SP, 0xc), c.g[A0][0]);
            c.sw(fb_ea(c, SP, 0x10), c.g[A1][0]);
            c.sh(fb_ea(c, SP, 0x2), c.g[V1][0]);
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0xc));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003472ec);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x003472ec)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = sx(fb_ea(c, SP, 0x20));
            c.lqc2(1, c.g[S3][0] as u32);
            c.lqc2(2, fb_ea(c, S3, 0x10));
            c.lqc2(3, fb_ea(c, S3, 0x20));
            c.lqc2(4, fb_ea(c, S3, 0x30));
            c.sqc2(1, c.g[V1][0] as u32);
            c.sqc2(2, fb_ea(c, V1, 0x10));
            c.sqc2(3, fb_ea(c, V1, 0x20));
            c.sqc2(4, fb_ea(c, V1, 0x30));
            c.lwc1(3, fb_ea(c, SP, 0x24));
            c.g[A2][0] = sx(fb_ea(c, SP, 0x60));
            c.lwc1(2, fb_ea(c, SP, 0x30));
            c.g[A1][0] = c.g[V1][0];
            c.swc1(3, fb_ea(c, SP, 0x30));
            c.lwc1(3, fb_ea(c, SP, 0x28));
            c.lwc1(1, fb_ea(c, SP, 0x40));
            c.swc1(3, fb_ea(c, SP, 0x40));
            c.lwc1(0, fb_ea(c, SP, 0x44));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.lwc1(3, fb_ea(c, SP, 0x38));
            c.swc1(0, fb_ea(c, SP, 0x38));
            c.sq(2, fb_ea(c, SP, 0x70));
            c.swc1(2, fb_ea(c, SP, 0x24));
            c.swc1(1, fb_ea(c, SP, 0x28));
            c.swc1(3, fb_ea(c, SP, 0x44));
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 0);
            c.mfc1(4, 0);
            c.lqc2(4, fb_ea(c, SP, 0x70));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x80));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.lq(2, fb_ea(c, SP, 0x80));
            c.sq(2, fb_ea(c, SP, 0x60));
            c.sw(fb_ea(c, SP, 0x6c), 0u64);
            c.lqc2(8, fb_ea(c, SP, 0x60));
            c.lqc2(4, c.g[V1][0] as u32);
            c.lqc2(5, fb_ea(c, V1, 0x10));
            c.lqc2(6, fb_ea(c, V1, 0x20));
            c.lqc2(7, fb_ea(c, V1, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, A2, 0x20));
            c.lq(2, fb_ea(c, SP, 0x80));
            c.g[A0][0] = c.lw(fb_ea(c, S0, 0x18));
            c.sq(2, fb_ea(c, SP, 0x60));
            c.sq(2, fb_ea(c, SP, 0x70));
            c.swc1(20, fb_ea(c, A2, 0xc));
            c.lq(2, fb_ea(c, SP, 0x60));
            c.g[V1][0] = c.lw(fb_ea(c, S1, 0x50));
            c.sq(2, fb_ea(c, SP, 0x50));
            c.lwc1(12, fb_ea(c, A0, 0x84));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0x8));
            c.div_s(12, 20, 12);
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003473e4);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x003473e4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S2][0];
            c.g[A2][0] = 0x1;
            c.g[A3][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003473fc);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0032c0f8, 0x003473fc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(fb_ea(c, S1, 0x50));
            c.g[A0][0] = c.lh(fb_ea(c, V0, 0x10));
            c.g[V1][0] = c.lw(fb_ea(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00347414);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00347414)?;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[V0][0] = 0x500000;
            if t {
                break 'b_347580;
            } else {
                c.cycles += 9;
                c.mov_s(12, 20);
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = 0xffffffffffffffff;
                c.g[T3][0] = 0x2;
                c.g[T2][0] = 0x1;
                c.g[T1][0] = 0xc;
                c.g[A1][0] = c.g[S2][0];
                'l_347440: loop {
                    c.cycles += 80;
                    c.sw(fb_ea(c, A2, 0x54), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sw(fb_ea(c, A2, 0x50), 0u64);
                    c.sw(fb_ea(c, A2, 0x58), 0u64);
                    c.sw(fb_ea(c, A2, 0x5c), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x60), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x64), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x44), c.g[T3][0]);
                    c.sw(fb_ea(c, A2, 0x4c), c.g[A3][0]);
                    c.lwc1(4, c.g[A2][0] as u32);
                    c.lq(2, c.g[T0][0] as u32);
                    c.lwc1(3, fb_ea(c, A2, 0x4));
                    c.sq(2, fb_ea(c, A2, 0x20));
                    c.lwc1(2, fb_ea(c, A2, 0x8));
                    c.lq(2, c.g[T0][0] as u32);
                    c.sw(fb_ea(c, A2, 0x68), 0u64);
                    c.sq(2, fb_ea(c, A2, 0x30));
                    c.sw(fb_ea(c, A2, 0x6c), 0u64);
                    c.sw(fb_ea(c, A2, 0x70), 0u64);
                    c.lwc1(1, fb_ea(c, A2, 0xc));
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.lwc1(0, fb_ea(c, V0, 0x84));
                    c.swc1(1, fb_ea(c, SP, 0x7c));
                    c.mul_s(2, 2, 0);
                    c.swc1(0, fb_ea(c, SP, 0x80));
                    c.mul_s(4, 4, 0);
                    c.swc1(0, fb_ea(c, SP, 0x84));
                    c.mul_s(3, 3, 0);
                    c.swc1(0, fb_ea(c, SP, 0x88));
                    c.swc1(2, fb_ea(c, SP, 0x78));
                    c.swc1(4, fb_ea(c, SP, 0x70));
                    c.swc1(3, fb_ea(c, SP, 0x74));
                    c.swc1(12, fb_ea(c, SP, 0x8c));
                    c.lqc2(8, fb_ea(c, SP, 0x70));
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.lqc2(5, fb_ea(c, S3, 0x10));
                    c.lqc2(6, fb_ea(c, S3, 0x20));
                    c.lqc2(7, fb_ea(c, S3, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, fb_ea(c, SP, 0x90));
                    c.lq(2, fb_ea(c, SP, 0x90));
                    c.sq(2, fb_ea(c, SP, 0x60));
                    c.sq(2, c.g[A2][0] as u32);
                    c.lqc2(8, fb_ea(c, A2, 0x10));
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.lqc2(5, fb_ea(c, S3, 0x10));
                    c.lqc2(6, fb_ea(c, S3, 0x20));
                    c.lqc2(7, fb_ea(c, S3, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, fb_ea(c, SP, 0x70));
                    c.lq(2, fb_ea(c, SP, 0x70));
                    c.sw(fb_ea(c, A2, 0x44), c.g[T2][0]);
                    c.sq(2, fb_ea(c, A2, 0x10));
                    c.sq(2, fb_ea(c, SP, 0x60));
                    c.g[V1][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.sw(fb_ea(c, A2, 0x50), c.g[V1][0]);
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.g[V1][0] = c.lw(fb_ea(c, V0, 0x80));
                    c.g[A0][0] = c.lw(fb_ea(c, V1, 0x4));
                    c.sw(fb_ea(c, A2, 0x48), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw(fb_ea(c, A2, 0x5c), c.g[A0][0]);
                    c.mult(0, 4, 4, 9);
                    c.lq(2, c.g[T0][0] as u32);
                    c.sq(2, fb_ea(c, A2, 0x20));
                    c.lq(3, c.g[T0][0] as u32);
                    c.sq(3, fb_ea(c, A2, 0x30));
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.g[V1][0] = c.lw(fb_ea(c, V0, 0x88));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(fb_ea(c, A0, 0x1a));
                    c.sw(fb_ea(c, A2, 0x4c), c.g[V0][0]);
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx(fb_ea(c, A2, 0x80));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00347440);
                        }
                        continue 'l_347440;
                    } else {
                        break 'b_347580;
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0];
        break 'b_347584;
    }
    c.cycles += 9;
    c.lq(16, fb_ea(c, SP, 0xf0));
    c.lq(17, fb_ea(c, SP, 0xe0));
    c.lq(18, fb_ea(c, SP, 0xd0));
    c.lq(19, fb_ea(c, SP, 0xc0));
    c.lq(20, fb_ea(c, SP, 0xb0));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0xa0));
    c.lwc1(20, fb_ea(c, SP, 0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0x110));
    c.pc = t;
    Ok(())
}

/// The base constructors the rail node constructors call first.
pub(crate) const FR_WORLD_OBJ_CTOR: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};

pub(crate) const FR_WORLD_OBJ_CTOR_2: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::f_0034fb90,
};

/// `base + off` as a 32-bit guest address.
#[inline(always)]
pub(crate) fn frail_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `gp - off` as a 32-bit guest address (the game's globals sit below gp).
#[inline(always)]
pub(crate) fn frail_gp_at(gp: u64, off: u32) -> u32 {
    (gp as u32).wrapping_sub(off)
}

/// `cFlexRailNode::cFlexRailNode(self, type, nodeType, desc)` (0x347d90):
/// base-object constructor, vtable, then the node count (the descriptor's
/// eight instance ids that are not -1), the flags, the segment array
/// (`NumNodes - 1` entries of 0x40 bytes, stored at +0x50) and the tube.
pub fn flex_rail_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        FR_WORLD_OBJ_CTOR.call(c, 0x0034_7dbc)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(frail_at(c.g[V1][0], 0xd0));
        c.g[A0][0] |= 0xffff;
        c.sw(frail_at(c.g[S1][0], 0xc), c.g[V1][0]);
        c.g[A1][0] = 0x7;
        c.g[V1][0] = sx(frail_at(c.g[S0][0], 0x4));
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x44));
        c.sw(frail_at(c.g[S1][0], 0x20), 0);
        c.sw(frail_at(c.g[S1][0], 0x24), c.g[V0][0]);
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[V0][0] == c.g[A0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
                c.g[V0][0] = sx(frail_at(c.g[V0][0], 0x1));
                c.sw(frail_at(c.g[S1][0], 0x20), c.g[V0][0]);
            }
            c.cycles += 2;
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[V1][0] = sx(frail_at(c.g[V1][0], 0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_7de8);
            }
        }
        c.cycles += 14;
        c.g[S0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        c.g[S2][0] = sx(frail_at(c.g[S1][0], 0x50));
        call_ptr(c, 0x0031_7e30, 0x0034_7e2c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = u64::MAX;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[A1][0] = 0x4a_0000;
        if !t {
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A1][0] = 0x4a_0000;
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7e40);
                }
            }
        }
        c.cycles += 19;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(frail_at(c.g[A1][0], 0x4008));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0034_7e74)?;
        c.g[A1][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x9));
        call_ptr(c, 0x0037_2520, 0x0034_7e88)?;
        c.sw(frail_at(c.g[S1][0], 0x58), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFlexRailNode::cFlexRailNode` second form (0x347ea8): the same node
/// setup, but the base constructor is `WorldObj_ctor_2`, and the node count
/// comes from a virtual call on the descriptor (`desc->vt[+0x14]`, this at
/// `desc + delta`), which writes the count at `self+0x20`.
pub fn flex_rail_node_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(frail_at(c.g[S1][0], 0x50));
        FR_WORLD_OBJ_CTOR_2.call(c, 0x0034_7ecc)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = sx(frail_at(c.g[S1][0], 0x20));
        c.g[V1][0] = sx(frail_at(c.g[V1][0], 0xd0));
        c.g[A2][0] = 0x30;
        c.sw(frail_at(c.g[S1][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh(frail_at(c.g[V0][0], 0x10));
        c.g[V1][0] = c.lw(frail_at(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(frail_at(c.g[S0][0], c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_7ef4)?;
        c.g[S0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_7f14)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = u64::MAX;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[A1][0] = 0x4a_0000;
        if !t {
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A1][0] = 0x4a_0000;
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7f28);
                }
            }
        }
        c.cycles += 19;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(frail_at(c.g[A1][0], 0x4008));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0034_7f5c)?;
        c.g[A1][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x9));
        call_ptr(c, 0x0037_2520, 0x0034_7f70)?;
        c.sw(frail_at(c.g[S1][0], 0x58), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlexRailNode_render(self)` (0x3480c8): cull the node's bounds
/// against the current view (`vt[93]`, with the matrix from `vt[43]`); if
/// visible, make sure the shared colour table is set up once, switch the
/// current draw state to texture 0 (-1) with the group bits cleared to 4, and
/// walk the `NumNodes - 1` segments (an empty inner loop of 8 steps each).
pub fn flex_rail_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S4, 0x60);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S1, 0x90);
        c.g[S5][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[A0][0] = c.g[SP][0];
        c.g[S4][0] = c.lw(frail_at(c.g[GP][0], 0x2a90));
        c.g[S3][0] = sx(frail_at(c.g[S5][0], 0x30));
        frame.save(c, S0, 0xa0);
        c.g[S2][0] = sx(frail_at(c.g[S5][0], 0x40));
        c.g[S0][0] = c.lw(frail_at(c.g[S4][0], 0x10d8));
        c.g[S1][0] = c.lh(frail_at(c.g[S0][0], 0x2e8));
        c.g[A1][0] = c.lh(frail_at(c.g[S0][0], 0x158));
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x15c));
        c.g[S1][0] = sx(frail_at(c.g[S4][0], c.g[S1][0] as u32));
        let get_mat = c.g[V0][0] as u32;
        c.g[A1][0] = sx(frail_at(c.g[S4][0], c.g[A1][0] as u32));
        call_ptr(c, get_mat, 0x0034_8118)?;
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x2ec));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        let cull = c.g[V0][0] as u32;
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, cull, 0x0034_8130)?;
        c.g[A0][0] = 0x1;
        let culled = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = c.lw(frail_at(c.g[GP][0], 0x2890));
        'done: {
            if culled {
                break 'done;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 47;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[V1][0] = 0x50_0000;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x940));
                c.lwc1(3, frail_gp_at(c.g[GP][0], 0x2b34));
                c.lwc1(1, frail_gp_at(c.g[GP][0], 0x2b30));
                c.lwc1(2, frail_gp_at(c.g[GP][0], 0x2b2c));
                c.swc1(0, frail_at(c.g[V0][0], 0x7c));
                c.lwc1(4, frail_gp_at(c.g[GP][0], 0x2b28));
                c.lwc1(5, frail_gp_at(c.g[GP][0], 0x2b24));
                c.lwc1(6, frail_gp_at(c.g[GP][0], 0x2b20));
                c.lwc1(7, frail_gp_at(c.g[GP][0], 0x2b1c));
                c.lwc1(8, frail_gp_at(c.g[GP][0], 0x2b18));
                c.lwc1(9, frail_gp_at(c.g[GP][0], 0x2b14));
                c.sw((c.g[V1][0] as u32).wrapping_sub(0x940), 0);
                c.swc1(4, frail_at(c.g[V0][0], 0x10));
                c.swc1(5, frail_at(c.g[V0][0], 0x14));
                c.swc1(6, frail_at(c.g[V0][0], 0x18));
                c.swc1(7, frail_at(c.g[V0][0], 0x20));
                c.swc1(8, frail_at(c.g[V0][0], 0x24));
                c.swc1(9, frail_at(c.g[V0][0], 0x28));
                c.swc1(3, frail_at(c.g[V0][0], 0x70));
                c.swc1(1, frail_at(c.g[V0][0], 0x74));
                c.swc1(2, frail_at(c.g[V0][0], 0x78));
                c.sw(frail_at(c.g[GP][0], 0x2890), c.g[A0][0]);
                c.sw(frail_at(c.g[V0][0], 0x4), 0);
                c.sw(frail_at(c.g[V0][0], 0x8), 0);
                c.swc1(0, frail_at(c.g[V0][0], 0xc));
                c.swc1(0, frail_at(c.g[V0][0], 0x1c));
                c.swc1(0, frail_at(c.g[V0][0], 0x2c));
                c.swc1(3, frail_at(c.g[V0][0], 0x30));
                c.swc1(1, frail_at(c.g[V0][0], 0x34));
                c.swc1(2, frail_at(c.g[V0][0], 0x38));
                c.swc1(0, frail_at(c.g[V0][0], 0x3c));
                c.swc1(3, frail_at(c.g[V0][0], 0x40));
                c.swc1(1, frail_at(c.g[V0][0], 0x44));
                c.swc1(2, frail_at(c.g[V0][0], 0x48));
                c.swc1(0, frail_at(c.g[V0][0], 0x4c));
                c.swc1(3, frail_at(c.g[V0][0], 0x50));
                c.swc1(1, frail_at(c.g[V0][0], 0x54));
                c.swc1(2, frail_at(c.g[V0][0], 0x58));
                c.swc1(0, frail_at(c.g[V0][0], 0x5c));
                c.swc1(3, frail_at(c.g[V0][0], 0x60));
                c.swc1(1, frail_at(c.g[V0][0], 0x64));
                c.swc1(2, frail_at(c.g[V0][0], 0x68));
                c.swc1(0, frail_at(c.g[V0][0], 0x6c));
            }
            c.g[V1][0] = c.lw(frail_at(c.g[S4][0], 0xe84));
            c.cycles += 12;
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = 0xffff_ffff_ffff_ff83;
            c.sh(frail_at(c.g[V1][0], 0x10), c.g[V0][0]);
            c.g[A0][0] = c.lw(frail_at(c.g[S4][0], 0xe84));
            c.g[V0][0] = c.lw(frail_at(c.g[A0][0], 0x4));
            c.g[V0][0] &= c.g[A1][0];
            c.g[V0][0] |= 0x4;
            c.sw(frail_at(c.g[A0][0], 0x4), c.g[V0][0]);
            c.g[V1][0] = c.lw(frail_at(c.g[S5][0], 0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if empty {
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            'outer: loop {
                c.cycles += 2;
                c.g[A1][0] = 0x7;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                'inner: loop {
                    c.cycles += 7;
                    if (c.g[A1][0] as i64) >= 0 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_8240);
                        }
                        continue 'inner;
                    }
                    c.cycles += 4;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if !again {
                        break 'done;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_8238);
                    }
                    continue 'outer;
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `FlexRail_FindClosestPoint` (0x00348290): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn flex_rail_find_closest_point(c: &mut Cpu) {
    if c.pc != 0x00348290 {
        return ee::s11::f_00348290__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00348290;
        loop {
            match pc {
                0x00348290 => {
                    c.cycles += 23;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[S0][0] = c.g[T3][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S1][0] = c.g[A3][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S5][0] = c.g[T2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x188));
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x178));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = c.g[T1][0];
                    if t {
                        pc = 0x00348b00;
                        continue;
                    }
                    pc = 0x003482ec;
                    continue;
                }
                0x003482ec => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x00348b00;
                        continue;
                    }
                    pc = 0x003482fc;
                    continue;
                }
                0x003482fc => {
                    c.cycles += 13;
                    c.g[V0][0] = 0x500000;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 23);
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.mtc1(0, 22);
                    c.g[S4][0] = 0xffffffffffffffff;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    pc = 0x00348330;
                    continue;
                }
                0x00348330 => {
                    c.cycles += 42;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 6);
                    c.mtc1(0, 9);
                    c.g[A2][0] = 0u64;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 10);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = 0x1;
                    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, c.g[SP][0] as u32);
                    c.lqc2(3, c.g[S1][0] as u32);
                    c.lqc2(4, c.g[SP][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    pc = 0x003483d8;
                    continue;
                }
                0x003483d8 => {
                    c.cycles += 59;
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.mov_s(2, 22);
                    c.mul_s(0, 0, 23);
                    c.mul_s(1, 0, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.mul_s(0, 0, 1);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, c.g[SP][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, c.g[S1][0] as u32);
                    c.lqc2(4, c.g[SP][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.div_s(1, 1, 0);
                    c.c_le_s(22, 1);

                    if c.fcc() {
                        c.min_s(2, 1, 21);
                        pc = 0x003484c4;
                        continue;
                    }
                    pc = 0x003484c4;
                    continue;
                }
                0x003484c4 => {
                    c.cycles += 34;
                    c.mfc1(3, 2);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, c.g[S1][0] as u32);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00348554;
                        continue;
                    }
                    pc = 0x0034854c;
                    continue;
                }
                0x0034854c => {
                    c.cycles += 2;
                    c.mov_s(3, 0);
                    c.g[A2][0] = c.g[A1][0];
                    pc = 0x00348554;
                    continue;
                }
                0x00348554 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.sq(2, c.g[SP][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x003483d8) {
                            return Ok(());
                        }
                        pc = 0x003483d8;
                        continue;
                    }
                    pc = 0x00348564;
                    continue;
                }
                0x00348564 => {
                    c.cycles += 167;
                    c.g[A1][0] = 0x490000;
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2b10));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1828));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x37));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x3f));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[A1][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1810));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[V1][0] as u32);
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x67));
                    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x6f));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x68));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                    c.mov_s(13, 3);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2b0c));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.lwc1(10, c.g[V1][0] as u32);
                    c.lwc1(9, c.g[V0][0] as u32);
                    c.mov_s(12, 1);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(5, 10, 10);
                    c.sub_s(0, 10, 9);
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mul_s(2, 9, 9);
                    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x2b08));
                    c.mul_s(6, 10, 5);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 11);
                    c.mul_s(3, 0, 3);
                    c.g[A1][0] = 0u64;
                    c.mul_s(4, 9, 2);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.mul_s(0, 0, 1);
                    c.add_s(7, 9, 3);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.add_s(4, 9, 0);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.mul_s(1, 7, 7);
                    c.sq(2, c.g[SP][0] as u32);
                    c.mul_s(3, 4, 4);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.mul_s(0, 7, 1);
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mov_s(20, 4);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mul_s(2, 4, 3);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, c.g[SP][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 8);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, c.g[SP][0] as u32);
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mov_s(2, 3);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, c.g[SP][0] as u32);
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    let t = true;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        pc = 0x00348974;
                        continue;
                    }
                    pc = 0x00348800;
                    continue;
                }
                0x00348800 => {
                    c.cycles += 4;
                    c.c_lt_s(3, 8);

                    if !c.fcc() {
                        c.mov_s(10, 4);
                        pc = 0x003488c4;
                        continue;
                    }
                    pc = 0x00348810;
                    continue;
                }
                0x00348810 => {
                    c.cycles += 45;
                    c.mov_s(9, 7);
                    c.mul_s(2, 10, 13);
                    c.mov_s(7, 4);
                    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(1, 7, 12);
                    c.mov_s(8, 3);
                    c.add_s(4, 1, 2);
                    c.mul_s(0, 4, 4);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mov_s(20, 4);
                    c.mul_s(1, 4, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, c.g[SP][0] as u32);
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.mov_s(3, 0);
                    let t = true;
                    c.mov_s(2, 3);
                    if t {
                        pc = 0x00348970;
                        continue;
                    }
                    pc = 0x003488c4;
                    continue;
                }
                0x003488c4 => {
                    c.cycles += 43;
                    c.mul_s(2, 9, 13);
                    c.mov_s(4, 7);
                    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(1, 4, 12);
                    c.mov_s(3, 8);
                    c.add_s(7, 1, 2);
                    c.mul_s(0, 7, 7);
                    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mov_s(20, 7);
                    c.mul_s(1, 7, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, c.g[SP][0] as u32);
                    c.lqc2(3, c.g[SP][0] as u32);
                    c.lqc2(4, c.g[S1][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.mov_s(8, 0);
                    c.mov_s(2, 8);
                    pc = 0x00348970;
                    continue;
                }
                0x00348970 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    pc = 0x00348974;
                    continue;
                }
                0x00348974 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 24i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                        pc = 0x00348998;
                        continue;
                    }
                    pc = 0x00348980;
                    continue;
                }
                0x00348980 => {
                    c.cycles += 5;
                    c.sub_s(0, 10, 9);
                    c.abs_s(0, 0);
                    c.c_le_s(0, 14);
                    let t = !c.fcc();

                    if t {
                        if back_edge_unwinds(c, 0x00348800) {
                            return Ok(());
                        }
                        pc = 0x00348800;
                        continue;
                    }
                    pc = 0x00348994;
                    continue;
                }
                0x00348994 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    pc = 0x00348998;
                    continue;
                }
                0x00348998 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x003489b4;
                        continue;
                    }
                    pc = 0x003489a0;
                    continue;
                }
                0x003489a0 => {
                    c.cycles += 5;
                    c.lwc1(0, c.g[S5][0] as u32);
                    c.c_lt_s(2, 0);

                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                        pc = 0x00348aec;
                        continue;
                    }
                    pc = 0x003489b4;
                    continue;
                }
                0x003489b4 => {
                    c.cycles += 76;
                    c.g[V1][0] = 0x2;
                    c.sw(c.g[S6][0] as u32, c.g[V0][0]);
                    c.add_s(1, 20, 20);
                    c.swc1(2, c.g[S5][0] as u32);
                    c.g[A1][0] = c.g[S3][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S4][0]);
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 0);
                    c.lq(2, c.g[FP][0] as u32);
                    c.lq(3, c.g[SP][0] as u32);
                    c.mul_s(0, 20, 0);
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
                    c.mul_s(0, 20, 0);
                    c.lq(2, c.g[FP][0] as u32);
                    c.sq(3, c.g[S0][0] as u32);
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x30));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x188));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00348aac);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00348aac)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
                    c.g[V1][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[S3][0]);
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = 0xc;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                    c.mult(0, 2, 2, 3);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x1a));

                    if c.g[V0][0] != c.g[S4][0] {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                        pc = 0x00348ae8;
                        continue;
                    }
                    pc = 0x00348ae4;
                    continue;
                }
                0x00348ae4 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                    pc = 0x00348ae8;
                    continue;
                }
                0x00348ae8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    pc = 0x00348aec;
                    continue;
                }
                0x00348aec => {
                    c.cycles += 5;
                    c.g[S3][0] = c.g[S7][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                        if back_edge_unwinds(c, 0x00348330) {
                            return Ok(());
                        }
                        pc = 0x00348330;
                        continue;
                    }
                    pc = 0x00348b00;
                    continue;
                }
                0x00348b00 => {
                    c.cycles += 16;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x188));
                    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x178));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x170));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00348290, 0x00348b40);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::put_vec;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::fb;

    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base;

    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;

    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_5;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn flex_bridge_update_matches() {
        check(
            ee::s11::f_003475a8__generated,
            flex_bridge_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3475a8, 0);
                c.g[A0][0] = DATA as u64;
                // The frame-clock singleton the step length comes from.
                c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
                c.bus.write32(DATA + 0x310, 0x42f0_0000); // 120.0f
                                                          // A bridge with no nodes: both physics passes return immediately.
                c.bus.write32(DATA + 0x50, 0);
                c.bus.write32(DATA + 0x74, r.next_u32());
            },
        );
    }

    #[test]
    fn flex_rail_save_matches() {
        save_test(
            ee::s11::f_00348b40__generated,
            flex_rail_save,
            0x348b_40,
            0x20,
        );
    }

    #[test]
    fn flex_rail_update_matches() {
        check(
            ee::s11::f_00348008__generated,
            flex_rail_update,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3480_08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let vt = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, vt, 0x200);
                c.bus.write32(this + 0xc, vt);
                c.bus
                    .write32(vt + 0x178, (r.next_u32() as i16 as i32 & !3) as u32);
                c.bus.write32(vt + 0x17c, FAKE);
                c.bus
                    .write32(vt + 0x180, (r.next_u32() as i16 as i32 & !3) as u32);
                c.bus.write32(vt + 0x184, FAKE + 0x10);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn flex_bridge_save_matches() {
        check(
            ee::s11::f_00347d38__generated,
            flex_bridge_save,
            SEEDS_4,
            &|c, seed| {
                let mut r = common(c, seed, 0x347d38, 3);
                let this = DATA + 0x100;
                let stream = DATA + 0x200;
                let vt = DATA + 0x300;
                clear(c, this, 0x200);
                clear(c, stream, 0x100);
                clear(c, vt, 0x200);
                c.bus.write32(stream, vt);
                vtable_2(c, vt, &[(0x8, 0, FAKE)]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn flex_rail_update_space_node_matches() {
        check(
            ee::s11::f_00348058__generated,
            flex_rail_update_space_node,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x34_8058, 1);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0xc, DATA + 0x2200);
                slot(c, DATA + 0x2200, 0x190, 0, FAKE);
                for (i, v) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
                    w(c, DATA + 0x2000 + 0x30 + 4 * i as u32, fb(*v));
                }
                for (i, v) in [-1.0f32, -2.0, -3.0, 0.0].iter().enumerate() {
                    w(c, DATA + 0x2000 + 0x40 + 4 * i as u32, fb(*v));
                }
                w(c, DATA + 0x2000 + 0x54, 1 + seed % 3);
                c.pc = 0x0034_8058;
            },
        );
    }

    #[test]
    fn flex_rail_dtor_matches() {
        check(
            ee::s11::f_00347f90__generated,
            flex_rail_dtor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x347f90, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                w(c, this + 0x50, 0); // no array free
                if seed % 2 == 0 {
                    w(c, this + 0x58, 0);
                } else {
                    let obj = DATA + 0x400;
                    w(c, this + 0x58, obj);
                    fake_slot(c, obj, DATA + 0x2000, 8);
                }
            },
        );
    }

    #[test]
    fn flex_bridge_init_segment_midpoints_matches() {
        strict(
            ee::s11::f_00346d38__generated,
            flex_bridge_init_segment_midpoints,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x6d38, 0);
                let this = DATA + 0x1000;
                let pts = DATA + 0x4000;
                let segs = DATA + 0x6000;
                let count = (seed % 4) as u32;
                w(c, this + 0x50, count);
                w(c, this + 0x7c, pts);
                w(c, this + 0x74, segs);
                for k in 0..(2 * count).max(1) {
                    let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                    put_vec(c, pts + 0x10 * k, v);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // -- 0x00347b80 FlexBridge_Render ----------------------------------

    #[test]
    fn flex_bridge_render_matches() {
        covered(
            ssx3_ee::s11::f_00347b80__generated,
            flex_bridge_render,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x347b80, 4);
                let world = DATA + 0x8_0000;
                let bridge = DATA + 0x1000;
                let board = DATA + 0x2000;
                let manager = DATA + 0x3000;
                let obj = DATA + 0x4000;
                let draw = DATA + 0x6000;
                w(c, GAME_GP.wrapping_add(0x16c8), manager);
                w(c, GAME_GP.wrapping_add(0x2a90), world);
                w(c, bridge + 0x18, board);
                w(c, bridge + 0xc, board);
                w(c, bridge + 0x78, 0);
                c.g[A0][0] = sx(bridge);
                c.bus
                    .write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
                c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
                let table = DATA + 0x7000;
                w(c, manager, table);
                for i in 0..4u32 {
                    w(c, table + 8 * i + 0x24, 6);
                    w(c, table + 0x18 * i + 0x3f0, 3);
                }
                w(c, world + 0xe84, draw);
                w(c, world + 0x10d8, obj);
                w(c, obj + 0x2e8, 0);
                w(c, obj + 0x158, 0);
                w(c, obj + 0x15c, FAKE + 0x20);
                w(c, obj + 0x2ec, FAKE + 0x30);
                w(c, obj + 0x300, 0);
                w(c, obj + 0x304, FAKE + 0x40);
                w(c, board + 0xc0, 0);
                w(c, board + 0xc4, FAKE + 0x50);
                for i in 0..0x30u32 {
                    c.bus.write8(draw + i, i as u8 | 1);
                }
                w(c, draw + 0x18, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::put_words;
    use crate::difftest::Rng;

    use crate::difftest::fill;

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    // Fixture layout (inside the test data area, clear of the fakes' log).
    pub(crate) const OC582_NODE: u32 = DATA + 0x5000;

    pub(crate) const OC582_DEF: u32 = DATA + 0x5400;

    pub(crate) const OC582_MESH: u32 = DATA + 0x5800;

    pub(crate) const OC582_MESH_VT: u32 = DATA + 0x5900;

    pub(crate) const OC582_WORLD: u32 = DATA + 0x5c00;

    pub(crate) const OC582_APP: u32 = DATA + 0x6000;

    pub(crate) const OC582_MODULE: u32 = DATA + 0x6200;

    pub(crate) const OC582_NASH: u32 = DATA + 0x6400;

    pub(crate) const OC582_GFX: u32 = DATA + 0x8000;

    /// Bump-allocated blocks for the fake `operator new`.
    pub(crate) const OC582_HEAP: u32 = DATA + 0x3_0000;

    /// Gate word and the bump pointer of the fakes.
    pub(crate) const OC582_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC582_MAGIC: u32 = 0x4f42_5245;

    pub(crate) static OC582_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC582_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC582_PREV_SPATIAL: AtomicUsize = AtomicUsize::new(0);

    /// Gated `operator new`: a bump allocation in the machine's RAM.
    pub(crate) fn oc582_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OPERATOR_NEW, ra);
        if c.pc == ra {
            let bump = c.bus.read32(OC582_CTRL + 4);
            c.bus.write32(OC582_CTRL + 4, bump + 0x400);
            c.g[V0][0] = (OC582_HEAP + bump) as u64;
        }
    }

    /// Gated `operator delete`: logged, nothing freed.
    pub(crate) fn oc582_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_DELETE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OPERATOR_DELETE, ra);
    }

    /// Gated `SpatialHash_Update`: logged, nothing updated.
    pub(crate) fn oc582_fake_spatial(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_SPATIAL, ee::s10::f_003291e0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPATIAL_HASH_UPDATE, ra);
    }

    pub(crate) fn oc582_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc582_fake_new,
            &OC582_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc582_fake_delete,
            &OC582_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003291E0),
            oc582_fake_spatial,
            &OC582_PREV_SPATIAL,
        );
    }

    /// The bridge node with `seed % 7` segments (0 covers the empty path),
    /// its instance record, the graphics manager and world with fake
    /// CreateGridMesh/GetMeshQuad/vertex-fill callees, and the real
    /// `World_GetNash` reading the app's module.
    pub(crate) fn oc582_setup(c: &mut Cpu, seed: u32) {
        oc582_install();
        let mut r = fakes::machine(c, seed, 0x0582_f1ab, 10);
        c.bus.write32(OC582_CTRL, OC582_MAGIC);
        c.bus.write32(OC582_CTRL + 4, 0);

        // gp-relative graphics manager -> world; world's grid and quad functions are fakes.
        c.bus.write32(GAME_GP + GFX_MAN_GP, OC582_GFX);
        c.bus.write32(OC582_GFX + GFX_MAN_WORLD, OC582_WORLD);
        c.bus.write16(OC582_WORLD + WORLD_GRID_MESH_DELTA, 0);
        c.bus
            .write32(OC582_WORLD + WORLD_GRID_MESH_FN, fakes::FAKE + 0x40);
        c.bus.write16(OC582_WORLD + WORLD_GET_QUAD_DELTA, 0);
        c.bus
            .write32(OC582_WORLD + WORLD_GET_QUAD_FN, fakes::FAKE + 0x80);
        // CreateGridMesh (call 0) returns the mesh; its vertex fill is a fake.
        c.bus.write32(LOG_RESULTS, OC582_MESH);
        c.bus.write32(OC582_MESH + 4, OC582_MESH_VT);
        c.bus.write16(OC582_MESH_VT + MESH_VT_DELTA, 0);
        c.bus
            .write32(OC582_MESH_VT + MESH_VT_FN, fakes::FAKE + 0x100);

        // The app -> module -> nash chain read by World_GetNash.
        c.bus.write32(GAME_GP - 0x848, OC582_APP);
        c.bus.write32(OC582_APP + 0x84, OC582_MODULE);
        c.bus.write32(OC582_MODULE + 0x20, OC582_NASH);

        // The bridge node and its instance record (min/max floats).
        c.bus.write32(OC582_NODE + NODE_DEF, OC582_DEF);
        c.bus.write32(OC582_NODE + NODE_COUNT, seed % 7);
        for k in 0..6u32 {
            c.bus.write32(OC582_DEF + DEF_MIN + 4 * k, r.f32(50.0));
        }
        c.g[A0][0] = OC582_NODE as u64;
        c.pc = 0x0034_66a0;
    }

    #[test]
    fn flex_bridge_build_mesh_matches() {
        fakes::covered(
            ee::s11::f_003466a0__generated,
            flex_bridge_build_mesh,
            120,
            &|c, seed| oc582_setup(c, seed),
        );
    }

    pub(crate) const FB_TEST_SEEDS: u32 = 200;

    /// The bridge object, its spring nodes, verts, face normals, the grid
    /// mesh and the two vtables (root transform at this+0xc, grid mesh at +4).
    pub(crate) const FB_TEST_SELF: u32 = DATA + 0x1000;

    pub(crate) const FB_TEST_NODES: u32 = DATA + 0x2000;

    pub(crate) const FB_TEST_VERTS: u32 = DATA + 0x4000;

    pub(crate) const FB_TEST_NORMS: u32 = DATA + 0x6000;

    pub(crate) const FB_TEST_GRID: u32 = DATA + 0x8000;

    pub(crate) const FB_TEST_GRID_VT: u32 = DATA + 0x9000;

    pub(crate) const FB_TEST_ROOT_VT: u32 = DATA + 0xa000;

    /// A random bridge with 0..=3 spring nodes (0 and 1 skip the loops that
    /// need two or more verts), fake virtual methods, and the Nash grid
    /// constants that SpatialKey_FromAABB needs.
    pub(crate) fn put_flexbridge(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let width = seed % 4;
        c.bus.write32(FB_TEST_SELF + FB_WIDTH, width);
        c.bus.write32(FB_TEST_SELF + FB_NODES, FB_TEST_NODES);
        c.bus
            .write32(FB_TEST_SELF + FB_INSTANCE, seed.wrapping_add(5));
        c.bus.write32(FB_TEST_SELF + FB_GRID_MESH, FB_TEST_GRID);
        c.bus.write32(FB_TEST_SELF + FB_VERTS, FB_TEST_VERTS);
        c.bus.write32(FB_TEST_SELF + FB_FACE_NORMS, FB_TEST_NORMS);
        c.bus.write32(FB_TEST_SELF + FB_ROOT_VPTR, FB_TEST_ROOT_VT);
        c.bus.write32(FB_TEST_GRID + 4, FB_TEST_GRID_VT);
        // Slot 3 of the grid mesh vtable: this delta and a fake method.
        let grid_delta = ((r.next_u32() % 64) & !3) as u16;
        c.bus.write16(FB_TEST_GRID_VT + 0x18, grid_delta);
        c.bus.write32(FB_TEST_GRID_VT + 0x1c, FAKE + 0x40);
        // getRootTransform: slot at +0xc0 (delta) / +0xc4 (fake method).
        let root_delta = ((r.next_u32() % 64) & !3) as u16;
        c.bus.write16(FB_TEST_ROOT_VT + 0xc0, root_delta);
        c.bus.write32(FB_TEST_ROOT_VT + 0xc4, FAKE + 0x80);
        put_vec(
            c,
            FB_TEST_SELF + FB_BOTTOM_RIGHT,
            [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000],
        );
        put_vec(
            c,
            FB_TEST_SELF + FB_BOTTOM_LEFT,
            [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000],
        );
        for i in 0..width {
            let node = FB_TEST_NODES + FB_NODE_SIZE * i;
            put_vec(
                c,
                node + FB_NODE_P,
                [r.f32(20.0), r.f32(20.0), r.f32(20.0), 0x3f80_0000],
            );
            c.bus.write32(node + FB_NODE_O, r.f32(3.0));
        }
        // The Nash grid constants (gp-0x2df8 / -0x2df4) for SpatialKey_FromAABB.
        put_words(c, GAME_GP - 0x2df8, &[0x3f36_db6e, 0x3e4c_cccd]);
        c.g[A0][0] = FB_TEST_SELF as u64;
    }

    #[test]
    fn flexbridge_update_verts_matches() {
        covered(
            ssx3_ee::s11::f_003475d8__generated,
            flexbridge_update_verts,
            FB_TEST_SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3475_d8, 6);
                put_flexbridge(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn flexbridge_update_verts_events() {
        check_events(
            ssx3_ee::s11::f_003475d8__generated,
            flexbridge_update_verts,
            FB_TEST_SEEDS,
            &|c, seed| -> Events {
                let mut r = machine(c, seed, 0x3475_d8 ^ 0xe7e7, 0);
                put_flexbridge(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0034_75d8)
            },
        );
    }

    // Fixture layout in the test data area (clear of the fakes' log at 0x900000).
    pub(crate) const NODE: u32 = DATA + 0x20_0000;

    pub(crate) const INFO: u32 = DATA + 0x20_0100;

    pub(crate) const FORCE: u32 = DATA + 0x20_0200;

    pub(crate) const VT: u32 = DATA + 0x20_0300;

    pub(crate) const VT2: u32 = DATA + 0x20_0400;

    pub(crate) const DEF: u32 = DATA + 0x20_0500;

    pub(crate) const DEF_V: u32 = DATA + 0x20_0600;

    pub(crate) const OUT: u32 = DATA + 0x20_1000;

    pub(crate) const FB_CTRL: u32 = DATA + 0x20_2000;

    pub(crate) const FB_MAGIC: u32 = 0x6f62_3630;

    pub(crate) static FB_PREV_MESH: AtomicUsize = AtomicUsize::new(0);

    /// A vtable slot at `vt + off`: a small delta and the function `fake`.
    pub(crate) fn fb_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, fake: u32) {
        c.bus.write16(vt + off, (r.next_u32() as u16) & !3);
        c.bus.write32(vt + off + 4, fake);
    }

    pub(crate) fn apply_force_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0602_0043, 1);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        fill(c, &mut r, FORCE, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x74, 0x40);
        c.bus.write32(NODE + 0x80, OUT);
        let weight = if seed % 4 == 0 {
            0x8000_0000
        } else {
            r.next_u32() & 0x3f
        };
        c.bus.write32(INFO + 0x60, weight);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.g[A2][0] = sx(FORCE);
        c.pc = 0x0034_6e38;
    }

    pub(crate) fn update_intersect_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0602_0042, 1);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x74, OUT);
        let weight = if seed % 4 == 0 {
            0x8000_0000
        } else {
            r.next_u32() & 0x3e
        };
        c.bus.write32(INFO + 0x60, weight);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.pc = 0x0034_70d0;
    }

    /// Gated IsMeshInt (`0x32c0f8`): logs the call and returns the count the setup wrote.
    pub(crate) fn fb_mesh_fake(c: &mut Cpu) {
        if c.bus.read32(FB_CTRL) != FB_MAGIC {
            return fakes::fall_back(c, &FB_PREV_MESH, ee::s10::f_0032c0f8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_c0f8, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(FB_CTRL + 4) as i32 as i64 as u64;
        }
    }

    pub(crate) fn fb_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0032C0F8),
            fb_mesh_fake,
            &FB_PREV_MESH,
        );
    }

    pub(crate) fn intersect_setup(c: &mut Cpu, seed: u32) {
        fb_install();
        let mut r = fakes::machine(c, seed, 0x0602_0039, 5);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        fill(c, &mut r, OUT, 0x80);
        fill(c, &mut r, DEF, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x18, DEF);
        c.bus.write32(DEF + 0x80, DEF_V);
        c.bus.write32(DEF + 0x88, 0x40);
        c.bus.write32(DEF_V + 4, DATA + 0x100);
        c.bus.write32(INFO + 0x50, VT2);
        fb_vtable(c, &mut r, VT2, 0x8, fakes::FAKE);
        fb_vtable(c, &mut r, VT2, 0x10, fakes::FAKE + 0x10);
        c.bus.write32(FB_CTRL, FB_MAGIC);
        let count = [0u32, 1, 2, 3, u32::MAX][((seed / 7) % 5) as usize];
        c.bus.write32(FB_CTRL + 4, count);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.g[A2][0] = sx(OUT);
        c.g[A3][0] = r.next_u32() as u64;
        let early = if seed % 7 == 0 { 0 } else { 2 };
        c.g[T0][0] = (r.next_u32() as u64 & !2) | early;
        c.pc = 0x0034_7268;
    }

    #[test]
    fn flex_bridge_apply_force_matches() {
        fakes::covered(
            ee::s11::f_00346e38__generated,
            flex_bridge_apply_force,
            120,
            &apply_force_setup,
        );
    }

    #[test]
    fn flex_bridge_update_intersect_info_matches() {
        fakes::covered(
            ee::s11::f_003470d0__generated,
            flex_bridge_update_intersect_info,
            120,
            &update_intersect_info_setup,
        );
    }

    #[test]
    fn flex_bridge_intersect_matches() {
        fakes::covered(
            ee::s11::f_00347268__generated,
            flex_bridge_intersect,
            120,
            &intersect_setup,
        );
    }

    /// The gate word: the fakes below answer only while it holds `OC603_MAGIC`
    /// (other tests run through them with the gate off).
    pub(crate) const OC603_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC603_MAGIC: u32 = 0x0603_0603;

    /// The key `findkey` writes (x, y, z) for the space-node test.
    pub(crate) const OC603_KEY: u32 = DATA + 0x20_0010;

    /// The node (`self`), its descriptor and the descriptor's vtable.
    pub(crate) const OC603_OBJ: u32 = DATA + 0x20_1000;

    pub(crate) const OC603_DESC: u32 = DATA + 0x20_2000;

    pub(crate) const OC603_DESC_VT: u32 = DATA + 0x20_3000;

    /// The render context (`gp+0x2a90`), its vtable and the draw state.
    pub(crate) const OC603_CTX: u32 = DATA + 0x20_8000;

    pub(crate) const OC603_CTX_VT: u32 = DATA + 0x20_5000;

    pub(crate) const OC603_DRAW: u32 = DATA + 0x20_6000;

    /// A gated fake: records the call to `$target` (the callee's entry) while
    /// the gate is up, else runs the body it replaced.
    macro_rules! oc603_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC603_CTRL) != OC603_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                let ret = c.g[RA][0] as u32;
                fake(c, $target, ret);
            }
        };
    }

    oc603_fake!(
        oc603_fake_world_obj,
        OC603_PREV_WORLD_OBJ,
        0x0034_fb00,
        ee::s11::f_0034fb00__generated
    );

    oc603_fake!(
        oc603_fake_world_obj_2,
        OC603_PREV_WORLD_OBJ_2,
        0x0034_fb90,
        ee::s11::f_0034fb90__generated
    );

    oc603_fake!(
        oc603_fake_alloc,
        OC603_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc603_fake!(
        oc603_fake_new_array,
        OC603_PREV_NEW_ARRAY,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc603_fake!(
        oc603_fake_tube,
        OC603_PREV_TUBE,
        0x0037_2520,
        ee::s12::f_00372520__generated
    );

    oc603_fake!(
        oc603_fake_get_nash,
        OC603_PREV_GET_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated
    );

    oc603_fake!(
        oc603_fake_add_node,
        OC603_PREV_ADD_NODE,
        0x0032_8c20,
        ee::s10::f_00328c20__generated
    );

    pub(crate) static OC603_PREV_FIND_KEY: AtomicUsize = AtomicUsize::new(0);

    /// `nash_Nash_findkey(key, box)`: writes the key's x, y, z from the test's
    /// words, then records the call.
    pub(crate) fn oc603_fake_find_key(c: &mut Cpu) {
        if c.bus.read32(OC603_CTRL) != OC603_MAGIC {
            return fall_back(c, &OC603_PREV_FIND_KEY, ee::s10::f_00328f28__generated);
        }
        let key = c.g[A0][0] as u32;
        for i in 0..3u32 {
            let v = c.bus.read32(OC603_KEY + 4 * i);
            c.bus.write32(key + 4 + 4 * i, v);
        }
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0032_8f28, ret);
    }

    pub(crate) fn oc603_install() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc603_fake_world_obj,
            &OC603_PREV_WORLD_OBJ,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB90),
            oc603_fake_world_obj_2,
            &OC603_PREV_WORLD_OBJ_2,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc603_fake_alloc,
            &OC603_PREV_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc603_fake_new_array,
            &OC603_PREV_NEW_ARRAY,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00372520),
            oc603_fake_tube,
            &OC603_PREV_TUBE,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc603_fake_get_nash,
            &OC603_PREV_GET_NASH,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc603_fake_find_key,
            &OC603_PREV_FIND_KEY,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328C20),
            oc603_fake_add_node,
            &OC603_PREV_ADD_NODE,
        );
    }

    #[test]
    fn flex_rail_node_ctor_matches() {
        oc603_install();
        covered(
            ee::s11::f_00347d90__generated,
            flex_rail_node_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_7d90, 4);
                c.pc = 0x0034_7d90;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // Instance ids 1..8 are -1 or random; id 0 never is, so the
                // node count is at least one.
                c.bus.write32(OC603_DESC + 4, r.next_u32() & 0x7fff_ffff);
                for i in 1..8u32 {
                    let v = if (seed >> i) & 1 == 1 {
                        0xffff_ffff
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(OC603_DESC + 4 + 4 * i, v);
                }
                c.bus.write32(OC603_DESC + 0x44, r.next_u32());
                c.g[A0][0] = sx(OC603_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(OC603_DESC);
            },
        );
    }

    #[test]
    fn flex_rail_node_ctor_2_matches() {
        oc603_install();
        covered(
            ee::s11::f_00347ea8__generated,
            flex_rail_node_ctor_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_7ea8, 4);
                c.pc = 0x0034_7ea8;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // The descriptor's vtable: this + 0x30 is the method's object,
                // its target is a fake (a miss hook) that writes nothing, so
                // the node count is preset.
                c.bus.write32(OC603_DESC, OC603_DESC_VT);
                c.bus.write16(OC603_DESC_VT + 0x10, 0x30);
                c.bus.write32(OC603_DESC_VT + 0x14, FAKE + 0x40);
                c.bus.write32(OC603_OBJ + 0x20, 1 + seed % 6);
                c.g[A0][0] = sx(OC603_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(OC603_DESC);
                c.g[A3][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn flex_rail_render_matches() {
        oc603_install();
        covered(
            ee::s11::f_003480c8__generated,
            flex_rail_render,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_80c8, 2);
                c.pc = 0x0034_80c8;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // The render context and its vtable: GetMat is slot 43, Cull
                // slot 93 (8-byte entries: a 16-bit this-delta, then the
                // target). Both targets are fakes.
                c.bus.write32(GAME_GP + 0x2a90, OC603_CTX);
                c.bus.write32(OC603_CTX + 0x10d8, OC603_CTX_VT);
                c.bus.write32(OC603_CTX + 0xe84, OC603_DRAW);
                c.bus.write16(OC603_CTX_VT + 0x158, 0x40);
                c.bus.write32(OC603_CTX_VT + 0x15c, FAKE + 0x10);
                c.bus.write16(OC603_CTX_VT + 0x2e8, 0x80);
                c.bus.write32(OC603_CTX_VT + 0x2ec, FAKE + 0x20);
                c.bus.write32(OC603_DRAW + 4, r.next_u32());
                c.bus.write32(OC603_DRAW + 0x10, r.next_u32());
                for off in [
                    0x2b34u32, 0x2b30, 0x2b2c, 0x2b28, 0x2b24, 0x2b20, 0x2b1c, 0x2b18, 0x2b14,
                ] {
                    c.bus.write32(GAME_GP - off, r.next_u32());
                }
                // The colour-table flag: clear on odd seeds (the table is
                // built), set on even ones.
                c.bus.write32(GAME_GP + 0x2890, 1 - seed % 2);
                // Cull is call 1: force "culled" on every fourth seed.
                if seed % 4 == 3 {
                    c.bus.write32(LOG_RESULTS + 4, 1);
                }
                fill(c, &mut r, OC603_OBJ + 0x30, 6);
                c.bus.write32(OC603_OBJ + 0x20, 1 + seed % 4);
                c.g[A0][0] = sx(OC603_OBJ);
            },
        );
    }

    #[test]
    fn flex_rail_find_closest_point_matches() {
        check(
            ee::s11::f_00348290__generated,
            flex_rail_find_closest_point,
            200,
            &|c, seed| {
                machine(c, seed, 0x8290, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00348290;
            },
        );
    }
}
