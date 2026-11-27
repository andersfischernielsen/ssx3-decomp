//! `src/object/flexrailnode.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::gp;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::object::deadfadenode::ctor_with_vtable;
pub(crate) use crate::game::object::debouncenode::field_addr;
pub(crate) use crate::game::object::flexbridgenode::frail_at;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::world::worldview::p_2 as p;
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
    (0x0034_90b0, float_rail_v01, "FloatRail_v01"),
    (0x0034_9110, float_rail_v50, "FloatRail_v50"),
    (
        0x0034_9798,
        float_rail_apply_impulse,
        "FloatRail_ApplyImpulse",
    ),
    (
        0x0034_9840,
        float_rail_get_contact_velocity,
        "FloatRail_GetContactVelocity",
    ),
    (0x0034_9ad0, float_rail_save, "FloatRail_Save"),
    (0x0034_9eb8, spring_rail_v01, "SpringRail_v01"),
    (0x0034_9f18, spring_rail_v50, "SpringRail_v50"),
    (
        0x0034_a568,
        spring_rail_apply_impulse,
        "SpringRail_ApplyImpulse",
    ),
    (0x0034_ac10, spring_rail_save, "SpringRail_Save"),
    (0x0034_ac88, flag_read_params, "Flag_ReadParams"),
    (0x0034_add8, flag_ctor, "Flag_ctor"),
    (0x0034_ae68, flag_ctor_2, "Flag_ctor_2"),
    (
        0x0034_af20,
        c_flag_manager_group_init_key,
        "cFlagManager_GroupInitKey",
    ),
    (
        0x0034_af38,
        c_flag_manager_group_ctor,
        "cFlagManager_GroupCtor",
    ),
    (
        0x0034_afb8,
        c_flag_manager_group_dtor,
        "cFlagManager_GroupDtor",
    ),
    (
        0x0034_afe8,
        c_flag_manager_group_matches,
        "cFlagManager_GroupMatches",
    ),
    (
        0x0034_b168,
        c_flag_manager_group_remove_flag,
        "cFlagManager_GroupRemoveFlag",
    ),
    (0x0034_aea0, flag_dtor, "Flag_dtor"),
    (0x0034_aef8, flag_save, "Flag_Save"),
    (
        0x0034_b038,
        c_flag_manager_group_add_flag,
        "cFlagManager_GroupAddFlag",
    ),
    (
        0x0034_8b90,
        flex_rail_space_node_register,
        "FlexRail_RegisterSpaceNode",
    ),
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
    (0x0034_a8c8, spring_rail_node_update_nodes, "SpringRail_v47"),
    (0x0034_8fa0, float_rail_ctor_2, "FloatRail_ctor_2"),
    (
        0x0034_8c48,
        flex_rail_unregister_space_node,
        "FlexRail_UnregisterSpaceNode",
    ),
    (0x0034_8d98, float_rail_ctor, "FloatRail_ctor"),
    (
        0x0034_9220,
        float_rail_reset_pos_from_instance_2,
        "FloatRail_ResetPosFromInstance",
    ),
    (0x0034_9b48, spring_rail_ctor_3, "SpringRail_ctor"),
    (0x0034_9db0, spring_rail_ctor_2_2, "SpringRail_ctor_2"),
    (
        0x0034_a028,
        spring_rail_init_pos_from_instance_2,
        "SpringRail_InitPosFromInstance",
    ),
    (0x0034_a0b0, spring_rail_v48, "SpringRail_v48"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX as INSTANCE_GET_NODE_MATRIX_2;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;

    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;

    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;

    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3;

    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;

    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
}

/// `void cFlagManager_GroupInitKey(cFlagManager_Group *g)` (0x34af20): clear
/// the group's key fields (state -1 at +0x58, +0x5c = 0).
pub fn c_flag_manager_group_init_key(c: &mut Cpu) {
    c.cycles += 6;
    let g = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    g.off(0x5c).sw(c, 0);
    g.off(0x58).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `cFlagManager_GroupDtor` (0x34afb8): vtable at `+0x184`.
pub fn c_flag_manager_group_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x184, 0x0048_fbf8, 0x0034_afdc);
}

/// `void* Flag_ctor_2(Flag* self)` (0x34ae68).
pub fn flag_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ctor_with_vtable(c, 0x0034_ae7c, callee::WORLD_OBJ_CTOR, 0x0048_fc10)
    })
}

/// `int cFlagManager_GroupMatches(cFlagManager* self, const void* data,
/// cFlagContext* ctx)` (0x34afe8): true when the group is live, its id matches
/// and the flag bytes compare equal.
pub fn c_flag_manager_group_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        let dead = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !dead {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A2)).off(0x80).lw(c);
            c.g[A0][0] = Addr(this).off(0x58).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[A1][0];
                c.g[A2][0] = 0x58;
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0041_5fc8, 0x0034_b024)?;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rails (VU0).

/// `void FloatRail_ApplyImpulse(FloatRail* self, RailNode* node, Vec4* v)`
/// (0x00349798): move the two rail points the node sits between by
/// `v * (1 - t)` and `v * t`.
pub fn float_rail_apply_impulse(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    let v = abi::arg(c, 2);
    c.g[A3][0] = sx(node);
    c.g[T0][0] = sx(self_);
    let idx = Addr(p(node, 0x64)).lw(c);
    c.g[A0][0] = idx;
    let negative = (c.g[A0][0] as i64) < 0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.lwc1(1, p(node, 0x68)); // t
    c.g[A1][0] = 0x50;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mult(0, 4, 4, 5);
    c.sub_s(0, 0, 1); // 1 - t
    c.mfc1(3, 0);
    c.lqc2(4, v);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.g[A0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x64));
    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x68));
    c.mult(0, 3, 3, 5);
    c.mfc1(2, 0);
    c.lqc2(4, v);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void FloatRail_GetContactVelocity(FloatRail* self, RailNode* node,
/// Vec4* out)` (0x00349840): blend the two rail point velocities by `t` and
/// add the contact's correction at +0x20.
pub fn float_rail_get_contact_velocity(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    c.g[A2][0] = sx(node);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    let negative = (c.g[V1][0] as i64) < 0;
    c.g[A3][0] = sx(self_);
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.g[A0][0] = 0x50;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.sub_s(1, 1, 0); // 1 - t
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(4, 1);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void SpringRail_ApplyImpulse(SpringRail* self, RailNode* node, Vec4* dir)`
/// (0x0034a568): clamp the direction to length 200 and push every rail
/// point up to the node with a spring force.
pub fn spring_rail_apply_impulse(c: &mut Cpu) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.cycles += 17;
    let self_ = abi::arg(c, 0);
    let _node = abi::arg(c, 1);
    let dir = abi::arg(c, 2);
    c.g[AT][0] = 0x4348_0000; // 200.0f
    c.mtc1(1, 1);
    c.lqc2(3, dir);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0); // f0 = |dir|
    c.g[A3][0] = sx(self_);
    c.c_lt_s(1, 0); // 200 < |dir|
    let over = c.fcc();
    if over {
        c.cycles += 10;
        c.div_s(0, 1, 0); // 200 / |dir|
        c.mfc1(3, 0);
        c.lqc2(4, dir);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, c.g[SP][0] as u32);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    } else {
        c.cycles += 2;
        c.lq(2, dir);
        c.sq(2, c.g[SP][0] as u32);
    }
    // node->idx < 0: nothing to push.
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x64)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        abi::ret(c);
        return;
    }
    // a = node->pos - elems[0].pos, c = dir x a, alen = |a|
    c.cycles += 30;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.g[T0][0] = 0;
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, c.g[V0][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[SP][0] as u32);
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(4, 22);
    c.mtc1(4, 2); // f2 = |a|
    if (c.g[V1][0] as i64) <= 0 {
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    } else {
        c.cycles += 4;
        c.g[T1][0] = c.g[T2][0];
        c.g[A2][0] = 0;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
        loop {
            c.cycles += 69;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(4, 22);
            c.mtc1(4, 0); // f0 = |vel[i]|
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 1); // f1 = |a|
            c.div_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0); // f0 = |vel[i]|
            c.div_s(0, 0, 2); // / alen
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x8)); // k[2]
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mfc1(4, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
                break;
            }
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            if event_due(c) {
                deliver_event_here(c, 0x0034_a670);
            }
        }
    }
    // The node's own element.
    c.cycles += 42;
    c.g[V0][0] = 0x60;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 4, 4, 2); // idx * 0x60
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, c.g[V1][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
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
    c.div_s(0, 0, 2);
    c.mfc1(3, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8)); // k[2]
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v01(FloatRail* self, int flags)` (0x3490b0): detach the
/// rail from the spatial tree and run the base destructor. The `v01` in the
/// name marks this as the vector-deleting variant.
pub fn float_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_90d8)?;
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_90ec)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_90f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SpringRail_v01(SpringRail* self, int flags)` (0x349eb8): the same
/// teardown for a spring rail (its own vtable, base `FlexRail_dtor`).
pub fn spring_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_9ee0)?;
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_9ef4)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_9f00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_00349AD0` (0x349ad0): `FloatRail_Save`, the float rail's serialiser.
/// Saves the flex rail (0x348b40), writes the 0xc-byte float block through the
/// archive's virtual writer (`vtable[8]`, `this+0x60`), then the variable-size
/// block: `size = *(int*)(this+0x20) * 0xc / ...` -- the original multiplies
/// the count at `+0x20` by 0xc (`mult`), adds it to the object base and writes
/// `this+0x70` through the writer.
pub(crate) fn rail_save(c: &mut Cpu, tail_block: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 29;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[RA][0] = sx(rets[0]);
    c.g[S1][0] = c.g[A1][0];
    callee::FLEX_RAIL_SAVE.call(c, rets[0])?;
    c.g[V1][0] = lw_r(c, S1, 0);
    c.g[A1][0] = sx(low(c, S0).wrapping_add(0x60));
    c.g[A2][0] = 0xc;
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[1])?;
    c.g[A0][0] = lw_r(c, S0, 0x20);
    c.g[A2][0] = sx(tail_block);
    c.g[V1][0] = lw_r(c, S1, 0);
    c.mult(0, 0, A0, A2);
    c.g[A1][0] = lw_r(c, S0, 0x70);
    c.g[A0][0] = lh_r(c, V1, 0x8);
    c.g[V0][0] = lw_r(c, V1, 0xc);
    c.g[A2][0] = c.lo[0];
    let target = low(c, V0);
    c.g[RA][0] = sx(rets[2]);
    c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, A0)));
    call_ptr(c, target, rets[2])?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `FloatRail_Save` (0x349ad0): 0x50-byte variable block.
pub fn float_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x50, [0x0034_9aec, 0x0034_9b08, 0x0034_9b30])
    })
}

/// `SpringRail_Save` (0x34ac10): same shape, 0x60-byte variable block.
pub fn spring_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        rail_save(c, 0x60, [0x0034_ac2c, 0x0034_ac48, 0x0034_ac70])
    })
}

/// `void* cFlagManager_GroupCtor(self)` (0x0034af38): construct a flag group
/// and clear its 60-entry key table.
pub fn c_flag_manager_group_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FLAG_MANAGER_GROUP_INIT_KEY.call(c, 0x0034_af4c)?;
        c.g[V1][0] = sx(0x490000);
        Addr(at(c, S0, 0x60)).sw(c, 0);
        c.g[V1][0] = sx(0x490000u32.wrapping_sub(0x408));
        Addr(at(c, S0, 0x74)).sw(c, 0);
        Addr(at(c, S0, 0x184)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(word(c, S0).wrapping_add(0x180));
        c.g[V1][0] = 0x3b;
        loop {
            c.cycles += 7;
            Addr(at(c, V0, 0)).sw(c, 0);
            c.g[V1][0] = sx(word(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_af68);
                }
                continue;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(gp(c, 0xf20)).lw(c);
            c.g[V1][0] = sx(word(c, V0).wrapping_add(1));
            Addr(at(c, S0, 0x90)).sw(c, c.g[V0][0]);
            c.g[A0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let keep = c.g[A0][0] != 0;
            Addr(gp(c, 0xf20)).sw(c, c.g[V1][0]);
            if !keep {
                c.cycles += 1;
                Addr(gp(c, 0xf20)).sw(c, 0);
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[S0][0];
            c.g[RA][0] = c.ld(frame.at(c, 0));
            c.lq(16, frame.at(c, 0x10));
            frame.close(c);
            break;
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spring rails.

/// `void SpringRail_InitPosFromInstance(SpringRail* node, int bone)` (0x34a028):
/// resolve the node's model and bone, then copy the world matrix translation
/// into the node's position.
pub fn spring_rail_init_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[RA][0] = sx(0x0034_a03c);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x002d_1bd8, 0x0034_a03c)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A1][0] = Addr(p_3(r32(c, S0), 0x50)).lw(c);
        c.g[A0][0] = Addr(p_3(r32(c, V1), 8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(p_3(r32(c, V0), r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 8;
            c.g[A0][0] = Addr(p_3(r32(c, V1), 0x1c)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p_3(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let empty2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !empty2 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, V0) << 2);
            }
        }
        c.cycles += 10;
        c.g[A1][0] = Addr(p_3(r32(c, S0), 0x54)).lw(c);
        c.g[RA][0] = sx(0x0034_a090);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_a090)?;
        c.lq(2, p_3(r32(c, SP), 0x30));
        c.g[RA][0] = c.ld(p_3(r32(c, SP), 0x50));
        c.sq(2, r32(c, S0));
        c.lq(16, p_3(r32(c, SP), 0x60));
        c.sq(2, p_3(r32(c, SP), 0x40));
        let target = r32(c, RA);
        c.g[SP][0] = sx(p_3(r32(c, SP), 0x70));
        c.pc = target;
        Ok(())
    })
}

/// `void FloatRail_ResetPosFromInstance(FloatRail* self, Instance* inst)`
/// (0x349220): look the instance's node up in the world table and copy its
/// node matrix into the rail's transform.
pub fn float_rail_reset_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x70);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x60));
        c.sd(at_2(sp0, 0x50), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::WORLD_GET_INSTANCE.call(c, 0x0034_9234)?;
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let inst = c.g[S0][0];
        c.g[A1][0] = lw_at(c, inst, 0x40);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 8);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(at_2(c.g[V0][0] as u32, c.g[A0][0] as u32));
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let none = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 8;
            c.g[A0][0] = lw_at(c, c.g[V1][0], 0x1c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(at_2(c.g[V0][0] as u32, c.g[A0][0] as u32));
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let none2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !none2 {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
            }
        }
        c.cycles += 11;
        c.g[A1][0] = lw_at(c, inst, 0x44);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX_2.call(c, 0x0034_9288)?;
        c.lq(2, at_2(sp0, 0x30));
        c.g[RA][0] = c.ld(at_2(sp0, 0x50));
        c.sq(2, at_2(inst as u32, 0x20));
        c.sq(2, at_2(sp0, 0x40));
        c.sq(2, at_2(inst as u32, 0x30));
        c.lq(16, at_2(sp0, 0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `void* Flag_ctor(Flag* self, void* node, const Params* params, void*
/// owner)` (0x34add8): build the world object, then read the flag parameters
/// and register the flag with the flag manager.
pub fn flag_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0xa0);
        let self_ = abi::arg(c, 0);
        let node = abi::arg64(c, 1);
        let params = abi::arg64(c, 2);
        let owner = abi::arg64(c, 3);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = params;
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(self_);
        c.g[S2][0] = owner;
        frame.save_ra(c, 0x60);
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = node;
        c.g[A2][0] = 0xa;
        call_ptr(c, callee::WORLD_OBJ_CTOR_3, 0x0034_ae04)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3f0));
        c.g[A2][0] = c.g[A2][0] | 0xfffd;
        c.sw(at(c, S1, 0xc), c.g[V1][0]);
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 4;
        c.sw(at(c, S0, 8), c.g[V0][0]);
        call_ptr(c, callee::FLAG_READ_PARAMS, 0x0034_ae34)?;
        call_ptr(c, callee::GET_FLAG_MANAGER, 0x0034_ae3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::FLAG_MANAGER_ADD_FLAG, 0x0034_ae4c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_RegisterSpaceNode(self, node)` (0x348b90): allocate a rail
/// entry for a space node and register it with the spatial hash.
pub fn flex_rail_register_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x490000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17f8));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x0031_7d70, 0x0034_8bc0)?;
        c.g[V1][0] = 0x3;
        c.sw(p_4(c.g[V0][0] as u32, 0xc), c.g[S0][0]);
        c.sw(p_4(c.g[V0][0] as u32, 8), c.g[V1][0]);
        c.sw(p_4(c.g[S0][0] as u32, 0x54), c.g[V0][0]);
        call_ptr(c, 0x002d_1be0, 0x0034_8bd4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = Addr(p_4(c.g[S0][0] as u32, 0x54)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8f28, 0x0034_8be8)?;
        c.g[V0][0] = Addr(p_4(c.g[SP][0] as u32, 4)).lw(c);
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = Addr(p_4(c.g[SP][0] as u32, 8)).lw(c);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = Addr(p_4(c.g[SP][0] as u32, 0xc)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8c20, 0x0034_8c2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at_3(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A 32-bit register value.
#[inline(always)]
pub(crate) fn r32v(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `off(sp)`.
#[inline(always)]
pub(crate) fn spa(c: &Cpu, off: u32) -> u32 {
    r32v(c, SP).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v50(FloatRail* rail)` (0x349110): grow the rail's bounding
/// box (`+0x30`: min, `+0x40`: max) over the points at_3 `*(rail+0x70)`
/// (`+0x20` of that array is the count, points are 0x50 bytes each; the first
/// point is skipped), then widen it by 200 units in every direction.
pub fn float_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    // *(rail+0x70): point array; *(rail+0x20): point count.
    c.g[A1][0] = Addr(at_3(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at_3(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    // Seed both corners from the second point (+0x20 of the array).
    c.lq(2, at_3(points, 0x20));
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at_3(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at_3(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at_3(points, 0x70));
        let mut p = at_3(points, 0x70);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            // min: keep the smaller of each axis (x, y, z).
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x38));
            }
            // max: keep the larger of each axis.
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x50));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9140);
            }
            p = p.wrapping_add(0x50);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    // Widen by 200 units on each side: one VU0 subtract, one add.
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at_3(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x30));
    c.lqc2(3, at_3(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

/// `void SpringRail_v50(SpringRail* rail)` (0x349f18): as
/// [`float_rail_v50`], but the points live at_3 `*(rail+0x70)` themselves and
/// are 0x60 bytes each (six floats: position and velocity).
pub fn spring_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    c.g[A1][0] = Addr(at_3(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at_3(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    c.lq(2, points);
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at_3(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at_3(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at_3(points, 0x60));
        let mut p = at_3(points, 0x60);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x38));
            }
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at_3(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 4));
            c.lwc1(0, at_3(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at_3(p, 8));
            c.lwc1(0, at_3(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at_3(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x60));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9f48);
            }
            p = p.wrapping_add(0x60);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at_3(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x30));
    c.lqc2(3, at_3(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE profile events.

/// `sFrRailDesc* Flag_ReadParams(sFrRailDesc* out, const FlagSrc* src)`
/// (0x34ac88): convert a rail modifier's source (flag words, floats) into
/// the runtime descriptor: four flag bytes, p/q, r0/r1 and each `t[]` triple,
/// with the rates and the `a` components scaled by `1/framerate`
/// (`*(int*)(gp+0x2a74 + 0x10)`).
pub fn flag_read_params(c: &mut Cpu) {
    c.cycles += 83;
    c.g[A2][0] = c.ldw_2(A1, 0x4);
    c.g[V0][0] = c.g[A0][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A1, 0x8);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x1, c.g[V1][0]);
    c.g[A2][0] = c.ldw_2(A1, 0xc);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0x2, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A1, 0x64);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x3, c.g[V1][0]);
    c.lwc1(0, c.pa(A1, 0x40));
    c.lwc1(1, c.pa(A1, 0x44));
    c.lwc1(2, c.pa(A1, 0x48));
    c.swc1(0, c.pa(A0, 0x34));
    c.swc1(1, c.pa(A0, 0x38));
    c.swc1(2, c.pa(A0, 0x3c));
    c.g[V1][0] = c.ldw_2(GP, 0x2a74);
    c.lwc1(0, c.pa(A1, 0x4c));
    c.lwc1(1, c.pa(A1, 0x50));
    c.lwc1(2, c.pa(A1, 0x54));
    c.swc1(0, c.pa(A0, 0x40));
    c.swc1(2, c.pa(A0, 0x48));
    c.swc1(1, c.pa(A0, 0x44));
    c.lwc1(1, c.pa(A1, 0x58));
    c.swc1(1, c.pa(A0, 0x4c));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x5c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x50));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x60));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x54));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x10));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(0, c.pa(A1, 0x14));
    c.swc1(0, c.pa(A0, 0x14));
    c.lwc1(1, c.pa(A1, 0x18));
    c.swc1(1, c.pa(A0, 0x24));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x1c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x8));
    c.lwc1(0, c.pa(A1, 0x20));
    c.swc1(0, c.pa(A0, 0x18));
    c.lwc1(1, c.pa(A1, 0x24));
    c.swc1(1, c.pa(A0, 0x28));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x28));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0xc));
    c.lwc1(0, c.pa(A1, 0x2c));
    c.swc1(0, c.pa(A0, 0x1c));
    c.lwc1(1, c.pa(A1, 0x30));
    c.swc1(1, c.pa(A0, 0x2c));
    c.lwc1(2, c.pa(V1, 0x10));
    c.cvt_s_w(2, 2);
    c.lwc1(0, c.pa(A1, 0x34));
    c.div_s(3, 3, 2);
    c.mul_s(0, 0, 3);
    c.swc1(0, c.pa(A0, 0x10));
    c.lwc1(1, c.pa(A1, 0x38));
    c.swc1(1, c.pa(A0, 0x20));
    c.lwc1(0, c.pa(A1, 0x3c));
    let ra = c.g[RA][0] as u32;
    c.swc1(0, c.pa(A0, 0x30));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupRemoveFlag (0x34b168).

/// Flag group: `+0x58` the track it belongs to, `+0x5c` how many riders it
/// holds, `+0x94` the rider pointers.
pub(crate) const FLAG_GROUP_TRACK: u32 = 0x58;

pub(crate) const FLAG_GROUP_COUNT: u32 = 0x5c;

pub(crate) const FLAG_GROUP_RIDERS: u32 = 0x94;

/// Rider: `+0x78` its id, `+0x80` a pointer to its current track.
pub(crate) const FLAG_RIDER_ID: u32 = 0x78;

pub(crate) const FLAG_RIDER_TRACK_PTR: u32 = 0x80;

pub(crate) const FLAG_GROUP_FREE_MESH: Callee = Callee {
    addr: 0x0034_b7b8,
    f: ee::s11::cFlagManager_GroupFreeMesh,
};

/// `int cFlagManager_GroupRemoveFlag(Group* group, Rider* rider)` (0x34b168):
/// remove `rider` from the group's rider list (matched by id; the last entry
/// moves into its slot) when the group is on the rider's track; free the
/// group's mesh when it becomes empty. Returns 1 when removed, else 0.
pub fn c_flag_manager_group_remove_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let group = Addr(r32(c, A3));
        let rider = Addr(r32(c, A1));
        c.g[A2][0] = group.off(FLAG_GROUP_COUNT).lw(c);
        c.g[V0][0] = 0;
        'out: {
            if c.g[A2][0] == 0 {
                break 'out;
            }
            c.cycles += 5;
            c.g[V0][0] = rider.off(FLAG_RIDER_TRACK_PTR).lw(c);
            c.g[A0][0] = group.off(FLAG_GROUP_TRACK).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0;
            if c.g[V1][0] != c.g[A0][0] {
                break 'out;
            }
            c.cycles += 2;
            c.g[T0][0] = 0;
            if (c.g[A2][0] as i64) <= 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[A2][0] = sx(group.0.wrapping_add(FLAG_GROUP_RIDERS));
            c.g[T1][0] = c.g[A2][0];
            loop {
                // (Reloaded from the registers: an event may have run.)
                let group = Addr(r32(c, A3));
                let rider = Addr(r32(c, A1));
                let riders = r32(c, T1);
                let slot = Addr(r32(c, A2));
                c.cycles += 5;
                c.g[V0][0] = slot.lw(c);
                c.g[A0][0] = rider.off(FLAG_RIDER_ID).lw(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(FLAG_RIDER_ID).lw(c);
                c.g[V0][0] = group.off(FLAG_GROUP_COUNT).lw(c);
                if c.g[V1][0] == c.g[A0][0] {
                    // Found: move the last entry here, clear the last slot.
                    c.cycles += 14;
                    let last = Addr(riders.wrapping_add(r32(c, V0).wrapping_sub(1) << 2));
                    c.g[V0][0] = sx(last.0);
                    c.g[V1][0] = last.lw(c);
                    slot.sw(c, c.g[V1][0]);
                    let count = group.off(FLAG_GROUP_COUNT).lw(c) as u32;
                    let last = Addr(riders.wrapping_add(count.wrapping_sub(1) << 2));
                    c.g[V0][0] = sx(last.0);
                    last.sw(c, 0);
                    let count = (group.off(FLAG_GROUP_COUNT).lw(c) as u32).wrapping_sub(1);
                    c.g[V1][0] = sx(count);
                    group.off(FLAG_GROUP_COUNT).sw(c, sx(count));
                    if count == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A3][0];
                        FLAG_GROUP_FREE_MESH.call(c, 0x0034_b1fc)?;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break 'out;
                }
                c.cycles += 4;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                let more = (c.g[T0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A2][0] = sx(slot.0.wrapping_add(4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'out;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_b1a8);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Flag: the vtable word at +0xc and the flag's instance at +0x18 (what the
/// flag manager tracks).
pub(crate) const FLAG_DTOR_VPTR: u32 = 0xc;

pub(crate) const FLAG_DTOR_INSTANCE: u32 = 0x18;

/// The vtable the destructor installs (0x48fc10).
pub(crate) const FLAG_DTOR_VTABLE: u32 = 0x0048_fc10;

/// `World_GetFlagManager` (another shard: through the table).
pub(crate) const OC308_WORLD_GET_FLAG_MANAGER: u32 = 0x002d_1cb0;

pub(crate) const C_FLAG_MANAGER_REMOVE_FLAG: Callee = Callee {
    addr: 0x0034_c600,
    f: ee::s11::cFlagManager_RemoveFlag,
};

pub(crate) const OC308_WORLD_OBJ_DTOR: Callee = Callee {
    addr: 0x0034_fbf0,
    f: ee::s11::WorldObj_dtor,
};

/// `void Flag_dtor(Flag* self, int flags)` (0x34aea0): reset the vtable
/// (+0xc), take the flag's instance (+0x18) out of the world's flag manager,
/// then destroy the WorldObj base (which frees on `flags & 1`).
pub fn flag_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(FLAG_DTOR_VTABLE);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.sw(this.wrapping_add(FLAG_DTOR_VPTR), c.g[V0][0]);
        call_ptr(c, OC308_WORLD_GET_FLAG_MANAGER, 0x0034_aec8)?;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(FLAG_DTOR_INSTANCE));
        c.g[A0][0] = c.g[V0][0];
        C_FLAG_MANAGER_REMOVE_FLAG.call(c, 0x0034_aed4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        OC308_WORLD_OBJ_DTOR.call(c, 0x0034_aee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0034aef8 Flag_Save

/// `void Flag_Save(Flag* self, Stream* stream)` (0x34aef8): a flag saves
/// only its world-object state (`WorldObj_Save`).
pub fn flag_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0034_fe90,
            f: ee::s11::WorldObj_Save,
        }),
        0x0034_af08,
    );
}

/// `void cFlagManager_GroupAddFlag(FlagGroup* self, Def* def, Mesh* mesh)`
/// (0x34b038): the first flag copies the 0x40-byte definition into the group,
/// takes the mesh's first word and builds the group mesh; every flag is
/// appended to the mesh table at +0x94 and counted at +0x5c.
pub fn c_flag_manager_group_add_flag(c: &mut Cpu) {
    const C_FLAG_MANAGER_GROUP_BUILD_MESH: Callee = Callee {
        addr: 0x0034b228,
        f: ee::s11::f_0034b228,
    };
    guest(c, |c| {
        'b_34b13c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] != 0u64;
            c.g[S1][0] = c.g[A2][0];
            if t {
                break 'b_34b13c;
            } else {
                'b_34b0f0: {
                    'b_34b0c0: {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[A1][0] | c.g[S0][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.g[S0][0];
                        if t {
                            break 'b_34b0c0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                            'l_34b06c: loop {
                                c.cycles += 19;
                                c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                                c.ldr(4, c.g[A1][0] as u32);
                                c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                                c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                                c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                                c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                                c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.sdr(4, c.g[V1][0] as u32);
                                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                let t = c.g[A1][0] != c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0034b06c);
                                    }
                                    continue 'l_34b06c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_34b0f0;
                                    } else {
                                        break 'b_34b0c0;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                    'l_34b0c4: loop {
                        c.cycles += 11;
                        c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                        c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0034b0c4);
                            }
                            continue 'l_34b0c4;
                        } else {
                            break 'b_34b0f0;
                        }
                    }
                }
                c.cycles += 19;
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[V1][0] as u32);
                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                C_FLAG_MANAGER_GROUP_BUILD_MESH.call(c, 0x0034b138)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                break 'b_34b13c;
            }
        }
        c.cycles += 11;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), c.g[S1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `World_GetInstance` (installed; returns the world object).
pub(crate) const WORLD_GET_INSTANCE: u32 = 0x002d_1bd8;

/// `World_GetSecondsPerTick` (installed; the update time in f0).
pub(crate) const WORLD_SECONDS_PER_TICK: u32 = 0x002d_1c70;

/// `cFloatRailNode_getPosition` (generated only, no override slot).
pub(crate) const FLOAT_RAIL_NODE_GET_POSITION: Callee = Callee {
    addr: 0x0034_9220,
    f: ee::s11::f_00349220,
};

/// `base + off` as a guest address (the 32-bit wrap of a 64-bit register).
#[inline]
pub(crate) fn float_rail_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// One Catmull-Rom component pass (generated shape): load four float fields
/// (`loads`: fpu register, address), store the registers at `stack + off`
/// (`stores`), load the gathered vector from `stack + vec` into vf8, load
/// the basis matrix at `basis` into vf4..vf7, multiply on VU0 and store vf12
/// to `stack + out`.
pub(crate) fn float_rail_spline_pass(
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
pub(crate) fn float_rail_transpose(c: &mut Cpu, m: u32, x: [usize; 4], t: [usize; 4]) {
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
pub(crate) fn float_rail_store_rows(
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
pub(crate) fn float_rail_refresh_node(c: &mut Cpu) -> Flow {
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

/// cSpringRailNode field offsets (the node array is 0x60 bytes per node).
pub(crate) const SR_NUM_NODES: u32 = 0x20;

pub(crate) const SR_SPRING: u32 = 0x60;

pub(crate) const SR_DAMP: u32 = 0x64;

pub(crate) const SR_NODES: u32 = 0x70;

/// `void cSpringRailNode_updateNodes(cSpringRailNode* self)` (0x34a8c8),
/// `SpringRail_v47`: one physics step of the rail's nodes. It reads the update
/// time (`cObjectInterface_getUpdateTime`, a real callee), then, for the
/// NumNodes-1 segments: scales the spring and damping torques into a per-segment
/// torque (0x90-byte scratch), integrates each node's orientation and angular
/// velocity, accumulates the offset and places the next node's position from
/// the segment vector (a normalised cross term); finally clears the node
/// tangents. Frame: 0x100 bytes, RA at +0xe0, S0 at +0xf0.
pub fn spring_rail_node_update_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x100));
        c.sq(16, sp(c).wrapping_add(0xf0));
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002d_1c70, 0x0034_a8dc)?;
        let st = sp(c);
        c.mov_s(1, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x7;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0u64;
        'b_34a988: {
            if t {
                break 'b_34a988;
            }
            c.cycles += 2;
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.g[SP][0];
            'l_34a900: loop {
                c.cycles += 34;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(SR_SPRING));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.neg_s(0, 0);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.mfc1(4, 0);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0x90));
                c.lq(3, st.wrapping_add(0x90));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(SR_DAMP));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(3, st.wrapping_add(0x80));
                c.neg_s(0, 0);
                c.sq(3, c.g[A1][0] as u32);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.mfc1(3, 0);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0x90));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                c.lq(2, st.wrapping_add(0x90));
                c.sq(2, st.wrapping_add(0x80));
                c.lqc2(3, c.g[A1][0] as u32);
                c.lqc2(4, st.wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.g[A1][0] as u32);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a900);
                    }
                    continue 'l_34a900;
                } else {
                    break 'b_34a988;
                }
            }
        }
        'b_34a9d4: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34a9d4;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[SP][0];
            c.g[A1][0] = 0u64;
            'l_34a9a0: loop {
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, c.g[V1][0] as u32);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a9a0);
                    }
                    continue 'l_34a9a0;
                } else {
                    break 'b_34a9d4;
                }
            }
        }
        'b_34aa80: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if t {
                break 'b_34aa80;
            }
            c.cycles += 3;
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.g[SP][0];
            'l_34a9f0: loop {
                c.cycles += 36;
                c.mfc1(2, 1);
                c.lqc2(4, c.g[A1][0] as u32);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xa0));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                c.lq(2, st.wrapping_add(0xa0));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, st.wrapping_add(0x90));
                c.sq(2, st.wrapping_add(0x80));
                c.mfc1(4, 1);
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xa0));
                c.lq(2, st.wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(2, st.wrapping_add(0x90));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, st.wrapping_add(0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lqc2(4, st.wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a9f0);
                    }
                    continue 'l_34a9f0;
                } else {
                    break 'b_34aa80;
                }
            }
        }
        'b_34abbc: {
            c.cycles += 8;
            c.g[V0][0] = 0x500000;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            c.g[A2][0] = 0u64;
            c.lq(4, c.g[V0][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) <= 0;
            c.sq(4, st.wrapping_add(0x80));
            if t {
                break 'b_34abbc;
            }
            c.cycles += 2;
            c.g[A1][0] = 0u64;
            'l_34aaa8: loop {
                c.cycles += 69;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, st.wrapping_add(0x80));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0x80));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(4, 22);
                c.mtc1(4, 0);
                c.g[V1][0] = c.g[V0][0];
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(5, st.wrapping_add(0x80));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, st.wrapping_add(0xd0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.lq(2, st.wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, st.wrapping_add(0xc0));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(4, st.wrapping_add(0xc0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.sq(2, st.wrapping_add(0xb0));
                c.lqc2(3, st.wrapping_add(0xb0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.sq(2, st.wrapping_add(0xa0));
                c.mfc1(3, 0);
                c.lqc2(4, st.wrapping_add(0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(2, st.wrapping_add(0x90));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.lqc2(3, c.g[A0][0] as u32);
                c.lqc2(4, st.wrapping_add(0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0xb0));
                c.lq(2, st.wrapping_add(0xb0));
                c.sq(2, st.wrapping_add(0xa0));
                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x60));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_aaa8);
                    }
                    continue 'l_34aaa8;
                } else {
                    break 'b_34abbc;
                }
            }
        }
        'b_34abfc: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34abfc;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x500000;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            'l_34abd8: loop {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.lq(2, c.g[A2][0] as u32);
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_abd8);
                    }
                    continue 'l_34abd8;
                } else {
                    break 'b_34abfc;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, st.wrapping_add(0xf0));
        c.g[RA][0] = c.ld(st.wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    });
}

/// `void FlexRail_RegisterSpaceNode(self)` (0x348b90): allocate the space
/// node (`0x10` bytes, `+0xc` = self, `+8` = type 3), store it at `self+0x54`
/// and add it to the nash octant the bounding box falls in (x/y/z sign picks
/// one of eight octants of the cell at offset 0x14 each).
pub fn flex_rail_space_node_register(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17f8));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x0031_7d70, 0x0034_8bc0)?;
        c.g[V1][0] = 0x3;
        c.sw(frail_at(c.g[V0][0], 0xc), c.g[S0][0]);
        c.sw(frail_at(c.g[V0][0], 0x8), c.g[V1][0]);
        c.sw(frail_at(c.g[S0][0], 0x54), c.g[V0][0]);
        call_ptr(c, 0x002d_1be0, 0x0034_8bd4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = sx(frail_at(c.g[S0][0], 0x30));
        c.g[S2][0] = c.lw(frail_at(c.g[S0][0], 0x54));
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8f28, 0x0034_8be8)?;
        c.g[V0][0] = c.lw(frail_at(c.g[SP][0], 0x4));
        c.g[A3][0] = u64::MAX;
        c.g[V1][0] = sx(frail_at(c.g[S1][0], 0x50));
        c.g[A1][0] = c.lw(frail_at(c.g[SP][0], 0x8));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = c.lw(frail_at(c.g[SP][0], 0xc));
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx(frail_at(c.g[V1][0], 0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx(frail_at(c.g[A0][0], 0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8c20, 0x0034_8c2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const FLEX_RAIL_CTOR_2: Callee = Callee {
    addr: 0x0034_7ea8,
    f: ee::s11::f_00347ea8,
};

pub(crate) const FLOAT_RAIL_UPDATE_MATS: Callee = Callee {
    addr: 0x0034_92a8,
    f: ee::s11::f_003492a8,
};

pub(crate) const FLOAT_RAIL_V50: Callee = Callee {
    addr: 0x0034_9110,
    f: ee::s11::f_00349110,
};

pub(crate) const FLEX_RAIL_REGISTER_SPACE_NODE: Callee = Callee {
    addr: 0x0034_8b90,
    f: ee::s11::f_00348b90,
};

/// `void* FloatRail_ctor_2(FloatRail* self, void* bucket, cBxReplayFrame*
/// stream)` (0x348fa0): the stream constructor. Runs the base constructor,
/// reads the node count from `self+0x20`, allocates `count * 0x50` bytes for
/// the nodes and stores the block at `self+0x70`. Each node's `+0x40` word is
/// set to -1, then the rail updates its matrices, its position and its space
/// hash. Returns `self`.
pub fn float_rail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        FLEX_RAIL_CTOR_2.call(c, 0x0034_8fc4)?;
        c.g[S3][0] = sx(field_addr(c.g[S1][0], 0x70));
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx(field_addr(c.g[S1][0], 0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd0));
        c.g[A2][0] = 0xc;
        let vtable = c.g[V1][0];
        Addr(field_addr(c.g[S1][0], 0xc)).sw(c, vtable);
        // The stream reader: `read(stream + off)` through the record's vtable.
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr(field_addr(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = Addr(field_addr(c.g[V0][0], 0x14)).lw(c);
        let read_fn = c.g[V1][0] as u32;
        c.g[A0][0] = sx(field_addr(c.g[S2][0], c.g[A0][0] as u32));
        call_ptr(c, read_fn, 0x0034_8ff0)?;
        c.g[S0][0] = Addr(field_addr(c.g[S1][0], 0x20)).lw(c);
        c.g[A0][0] = 0x50;
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x2000_0000;
        c.mult(0, 0, 16, 4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17e0));
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0034_9014)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !empty {
            c.cycles += 4;
            c.g[A1][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = u64::MAX;
            c.g[A1][0] |= 0xffff;
            loop {
                c.cycles += 7;
                let minus_one = c.g[A1][0];
                Addr(field_addr(c.g[V1][0], 0x40)).sw(c, minus_one);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                let more = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx(field_addr(c.g[V1][0], 0x50));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9030);
                }
            }
        }
        c.cycles += 25;
        let block = c.g[V0][0];
        Addr(field_addr(c.g[S3][0], 0)).sw(c, block);
        c.g[V0][0] = 0x50;
        c.g[A2][0] = Addr(field_addr(c.g[S1][0], 0x20)).lw(c);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.mult(0, 0, 6, 2);
        c.g[A1][0] = Addr(field_addr(c.g[S1][0], 0x70)).lw(c);
        c.g[A0][0] = Addr(field_addr(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = Addr(field_addr(c.g[V1][0], 0x14)).lw(c);
        c.g[A2][0] = c.lo[0];
        let fill_fn = c.g[V0][0] as u32;
        c.g[A0][0] = sx(field_addr(c.g[S2][0], c.g[A0][0] as u32));
        call_ptr(c, fill_fn, 0x0034_9078)?;
        c.g[A0][0] = c.g[S1][0];
        FLOAT_RAIL_UPDATE_MATS.call(c, 0x0034_9080)?;
        c.g[A0][0] = c.g[S1][0];
        FLOAT_RAIL_V50.call(c, 0x0034_9088)?;
        c.g[A0][0] = c.g[S1][0];
        FLEX_RAIL_REGISTER_SPACE_NODE.call(c, 0x0034_9090)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_UnregisterSpaceNode(FlexRail* self)` (0x348c48,
/// `cFlexRailNode_remSpaceHash`): removes the rail's space node from the
/// Nash: bumps the delete count, finds the octant by the bounding box key,
/// removes the node from the cell, frees the root cell when it is empty,
/// then deletes the node and clears the pointer.
pub fn flex_rail_unregister_space_node(c: &mut Cpu) {
    if c.pc != 0x00348c48 {
        return ee::s11::f_00348c48__generated(c);
    }
    guest(c, |c| {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x00348c64);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x002d1be0, 0x00348c64)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = sx(0x00348c84);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
        call_ptr(c, 0x00328f28, 0x00348c84)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S0][0];
        }
        c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[A0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00348cd0);
        c.g[T0][0] = c.g[S0][0];
        call_ptr(c, 0x003284b8, 0x00348cd0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));

        // Is the node's cell list empty (7 words, then 3 more in a table)?
        // V0 is 0 when a word is set (the cell is kept), nonzero when all
        // are empty.
        'chk: {
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'chk;
            }
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18] {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(off));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'chk;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            if !t {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                break 'chk;
            }
            c.cycles += 1;
            c.g[A0][0] = 0u64;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    if back_edge_unwinds(c, 0x00348d34) {
                        return Ok(());
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'chk;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t {
                    if back_edge_unwinds(c, 0x00348d40) {
                        return Ok(());
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'chk;
            }
        }

        // 0x348d5c
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00348d6c);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317e50, 0x00348d6c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        }

        // 0x348d70
        c.cycles += 9;
        c.g[RA][0] = sx(0x00348d78);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
        call_ptr(c, 0x00317e50, 0x00348d78)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x54), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}

/// `void* FloatRail_ctor(FloatRail* self, void* bucket, Desc* desc)`
/// (0x348d98, `cFloatRailNode_cFloatRailNode`): builds the rail on the
/// FlexRail base (type 20), sets the vtable and the spring constants from the
/// descriptor, packs the descriptor's used instance ids and meshes, allocates
/// one node per id, places each from its instance
/// (`FloatRail_ResetPosFromInstance`) and finishes with the matrices, the
/// bound box and the space-hash registration. Returns `self`.
pub fn float_rail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = 0x14;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[RA][0] = sx(0x00348dcc);
        c.g[A3][0] = c.g[S0][0];
        Callee {
            addr: 0x00347d90,
            f: ee::s11::FlexRail_ctor,
        }
        .call(c, 0x00348dcc)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd0));
        c.g[A1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = 0x7;
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x48));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x68));
        'l_348e08: loop {
            c.cycles += 7;
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00348e08);
                }
                continue 'l_348e08;
            } else {
                c.cycles += 7;
                c.g[A3][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = 0x7;
                c.g[A0][0] = c.g[SP][0];
                'l_348e40: loop {
                    'b_348e5c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        let t = c.g[V1][0] == c.g[A3][0];
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_348e5c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            break 'b_348e5c;
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00348e40);
                        }
                        continue 'l_348e40;
                    } else {
                        'b_348ec4: {
                            c.cycles += 12;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[A0][0] = 0x50;
                            c.g[A1][0] = 0x490000;
                            c.g[A2][0] = 0x20000000;
                            c.mult(0, 0, 16, 4);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17e0));
                            c.g[A0][0] = c.lo[0];
                            c.g[RA][0] = sx(0x00348e8c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x00348e8c)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_348ec4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = 0xffffffffffff0000;
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A1][0] = c.g[A1][0] | 0xffff;
                                'l_348ea8: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[A0][0] != c.g[A2][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00348ea8);
                                        }
                                        continue 'l_348ea8;
                                    } else {
                                        break 'b_348ec4;
                                    }
                                }
                            }
                        }
                        'b_348f60: {
                            c.cycles += 4;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S3][0] = 0u64;
                            if t {
                                break 'b_348f60;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x500000;
                                c.g[S2][0] = c.g[SP][0];
                                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                                c.g[S0][0] = 0u64;
                                'l_348ee8: loop {
                                    c.cycles += 30;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(3, c.g[S4][0] as u32);
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, c.g[S4][0] as u32);
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.sq(2, c.g[A1][0] as u32);
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x00348f40);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    Callee {
                                        addr: 0x00349220,
                                        f: ee::s11::FloatRail_ResetPosFromInstance,
                                    }
                                    .call(c, 0x00348f40)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00348ee8);
                                        }
                                        continue 'l_348ee8;
                                    } else {
                                        break 'b_348f60;
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x00348f68);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x003492a8,
                            f: ee::s11::FloatRail_v48,
                        }
                        .call(c, 0x00348f68)?;
                        c.g[RA][0] = sx(0x00348f70);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00349110,
                            f: ee::s11::FloatRail_v50,
                        }
                        .call(c, 0x00348f70)?;
                        c.g[RA][0] = sx(0x00348f78);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00348b90,
                            f: ee::s11::FlexRail_RegisterSpaceNode,
                        }
                        .call(c, 0x00348f78)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void FloatRail_ResetPosFromInstance(FloatRail* self, Node* node)`
/// (0x349220, `cFloatRailNode_getPosition`): looks the node's instance up in
/// the world table (low byte, then the high bits of its id), fetches the
/// node's bone matrix (`Instance_GetNodeMatrix`) and copies the matrix's
/// translation row to the node's position and previous position.
pub fn float_rail_reset_pos_from_instance_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_34927c: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.g[RA][0] = sx(0x00349234);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x002d1bd8, 0x00349234)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34927c;
            } else {
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
                    break 'b_34927c;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    break 'b_34927c;
                }
            }
        }
        c.cycles += 11;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00349288);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0034fed8,
            f: ee::s11::Instance_GetNodeMatrix,
        }
        .call(c, 0x00349288)?;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void* SpringRail_ctor(SpringRail* self, void* bucket, Desc* desc)`
/// (0x349b48, `cSpringRailNode_cSpringRailNode`): the spring rail's
/// constructor: FlexRail base, vtable and spring constants, the used
/// instance ids packed into a frame scratch, the 0x60-byte node array
/// allocated and each node placed from its instance, then matrices, bound box
/// and space-hash registration. Returns `self`.
pub fn spring_rail_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = 0x15;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[RA][0] = sx(0x00349b7c);
        c.g[A3][0] = c.g[S0][0];
        Callee {
            addr: 0x00347d90,
            f: ee::s11::FlexRail_ctor,
        }
        .call(c, 0x00349b7c)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        c.g[A1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
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
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00349bb8);
                }
                continue 'l_349bb8;
            } else {
                c.cycles += 7;
                c.g[A3][0] = 0xffffffffffff0000;
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
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            break 'b_349c0c;
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00349bf0);
                        }
                        continue 'l_349bf0;
                    } else {
                        'b_349c74: {
                            c.cycles += 12;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[A0][0] = 0x60;
                            c.g[A1][0] = 0x490000;
                            c.g[A2][0] = 0x20000000;
                            c.mult(0, 0, 16, 4);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
                            c.g[A0][0] = c.lo[0];
                            c.g[RA][0] = sx(0x00349c3c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x00349c3c)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_349c74;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = 0xffffffffffff0000;
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A1][0] = c.g[A1][0] | 0xffff;
                                'l_349c58: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[A0][0] != c.g[A2][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349c58);
                                        }
                                        continue 'l_349c58;
                                    } else {
                                        break 'b_349c74;
                                    }
                                }
                            }
                        }
                        'b_349d24: {
                            c.cycles += 4;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S4][0] = 0u64;
                            if t {
                                break 'b_349d24;
                            } else {
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
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(3, c.g[S3][0] as u32);
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, c.g[S3][0] as u32);
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, c.g[S3][0] as u32);
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, c.g[S3][0] as u32);
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x00349d10);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    Callee {
                                        addr: 0x0034a028,
                                        f: ee::s11::SpringRail_InitPosFromInstance,
                                    }
                                    .call(c, 0x00349d10)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349c98);
                                        }
                                        continue 'l_349c98;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                        break 'b_349d24;
                                    }
                                }
                            }
                        }
                        'b_349d74: {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_349d74;
                            } else {
                                c.cycles += 2;
                                c.g[A2][0] = 0u64;
                                'l_349d38: loop {
                                    c.cycles += 15;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349d38);
                                        }
                                        continue 'l_349d38;
                                    } else {
                                        break 'b_349d74;
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x00349d7c);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0034a0b0,
                            f: ee::s11::SpringRail_v48,
                        }
                        .call(c, 0x00349d7c)?;
                        c.g[RA][0] = sx(0x00349d84);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00349f18,
                            f: ee::s11::SpringRail_v50,
                        }
                        .call(c, 0x00349d84)?;
                        c.g[RA][0] = sx(0x00349d8c);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00348b90,
                            f: ee::s11::FlexRail_RegisterSpaceNode,
                        }
                        .call(c, 0x00349d8c)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void* SpringRail_ctor_2(SpringRail* self, void* bucket, Stream* s)`
/// (0x349db0, `cSpringRailNode_cSpringRailNode1`): the stream form: second
/// FlexRail base constructor, the spring constants and the node array read
/// from the stream, then matrices and the space-hash registration. Returns
/// `self`.
pub fn spring_rail_ctor_2_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_349e5c: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00349dd4);
            c.g[S2][0] = c.g[A2][0];
            Callee {
                addr: 0x00347ea8,
                f: ee::s11::FlexRail_ctor_2,
            }
            .call(c, 0x00349dd4)?;
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x270));
            c.g[A2][0] = 0xc;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00349e00);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00349e00)?;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x60;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0x20000000;
            c.mult(0, 0, 16, 4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
            c.g[A0][0] = c.lo[0];
            c.g[RA][0] = sx(0x00349e24);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x00349e24)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[V0][0];
            if t {
                break 'b_349e5c;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                'l_349e40: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00349e40);
                        }
                        continue 'l_349e40;
                    } else {
                        break 'b_349e5c;
                    }
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
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00349e88);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00349e88)?;
        c.g[RA][0] = sx(0x00349e90);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0034a0b0,
            f: ee::s11::SpringRail_v48,
        }
        .call(c, 0x00349e90)?;
        c.g[RA][0] = sx(0x00349e98);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x00348b90,
            f: ee::s11::FlexRail_RegisterSpaceNode,
        }
        .call(c, 0x00349e98)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void SpringRail_InitPosFromInstance(SpringRail* node, Instance* inst)`
/// (0x34a028): looks the instance's handle (`inst + 0x50`: low byte, then the
/// rest) up in the world's two-level instance table (`World_GetInstance`),
/// asks `Instance_GetNodeMatrix` for the node matrix (`inst + 0x54`) and
/// copies its translation row to the first quad of `inst`.
pub fn spring_rail_init_pos_from_instance_2(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34a084: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.g[RA][0] = sx(0x0034a03c);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x002d1bd8, 0x0034a03c)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34a084;
            } else {
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
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    break 'b_34a084;
                }
            }
        }
        c.cycles += 10;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[RA][0] = sx(0x0034a090);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0034fed8,
            f: ee::s11::Instance_GetNodeMatrix,
        }
        .call(c, 0x0034a090)?;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(2, c.g[S0][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void SpringRail_v48(SpringRail* self)` (0x34a0b0, leaf): rebuilds the
/// per-segment matrices (`Mats`, 0x40 bytes each, at `self + 0x50`) from the
/// node array (`Nodes`, 0x60 bytes each, at `self + 0x70`): the first segment,
/// the middle ones while `i < NumNodes - 2`, then the last. Each segment is
/// three VU0 transforms by the matrix at 0x4ffac0 and a transposition, with
/// the intermediate quads spilled to the 0x50-byte frame (observable bytes).
/// Transcribed from the generated body.
pub fn spring_rail_v48(c: &mut Cpu) {
    'b_34a3dc: {
        c.cycles += 99;
        c.g[T5][0] = c.g[A0][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = 0x500000;
        c.g[A0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
        c.g[T7][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x540));
        c.lwc1(2, c.g[V1][0] as u32);
        c.g[T6][0] = 0x1;
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc0));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(4, c.g[T7][0] as u32);
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc4));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x2c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, c.g[T7][0] as u32);
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc8));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x68));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(4, c.g[T7][0] as u32);
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(3, c.g[SP][0] as u32);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
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
        c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[T8][0] = c.g[A1][0];
        if t {
            break 'b_34a3dc;
        } else {
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
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, c.g[T7][0] as u32);
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(2, c.g[SP][0] as u32);
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x64));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc4));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, c.g[T7][0] as u32);
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc8));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(4, c.g[T7][0] as u32);
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(3, c.g[SP][0] as u32);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(3, c.g[T3][0] as u32);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x10));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x3c));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
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
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034a248);
                    }
                    continue 'l_34a248;
                } else {
                    break 'b_34a3dc;
                }
            }
        }
    }
    c.cycles += 99;
    c.g[V1][0] = 0x60;
    c.g[A2][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 14, 3);
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
    c.g[A3][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x540));
    c.g[A0][0] = sx((c.g[T6][0] as u32) << 6);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = c.g[A1][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
    c.lwc1(2, c.g[V1][0] as u32);
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.lwc1(0, c.g[A1][0] as u32);
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[A3][0] as u32);
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, c.g[SP][0] as u32);
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[A3][0] as u32);
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(4, c.g[A3][0] as u32);
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(3, c.g[SP][0] as u32);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(3, c.g[A0][0] as u32);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
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
    return;
}

#[cfg(test)]
pub(crate) const SR_MATS: u32 = 0x50;

#[cfg(test)]
pub(crate) const SR_SEG_XFORM: u32 = 0x004f_ffc0;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;

    pub(crate) use crate::game::object::deadfadenode::tests::ctor_setup;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    pub(crate) use crate::game::world::worldview::tests::word;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_flag_manager_group_init_key_matches() {
        check(
            ssx3_ee::s11::f_0034af20,
            c_flag_manager_group_init_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x1014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x58, r.next_u32());
                c.bus.write32(DATA + 0x5c, r.next_u32());
            },
        );
    }

    #[test]
    fn c_flag_manager_group_dtor_matches() {
        dtor_case(
            ee::s11::f_0034afb8__generated,
            c_flag_manager_group_dtor,
            0x34afb8,
        );
    }

    #[test]
    fn flag_ctor_2_matches() {
        check(
            ee::s11::f_0034ae68__generated,
            flag_ctor_2,
            80,
            &|c, seed| {
                ctor_setup(c, seed, 0x34ae_68, 0x20, DATA + 0x300);
            },
        );
    }

    #[test]
    fn c_flag_manager_group_matches_matches() {
        check(
            ee::s11::f_0034afe8__generated,
            c_flag_manager_group_matches,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34af_e8, 1);
                let this = DATA + 0x100;
                let data = DATA + 0x400;
                let ctx = DATA + 0x600;
                clear(c, this, 0x80);
                clear(c, data, 0x20);
                clear(c, ctx, 0x90);
                c.bus
                    .write32(this + 0x5c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x58, r.next_u32());
                let group = c.bus.read32(this + 0x58);
                c.bus.write32(ctx + 0x80, DATA + 0x700);
                c.bus.write32(
                    DATA + 0x700,
                    if seed % 3 == 1 { group } else { r.next_u32() },
                );
                // The 0x58-byte flag block lives at `self`, so the copy at
                // `data` compares equal on some seeds.
                for i in 0..0x16u32 {
                    let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
                    c.bus.write32(this + 4 * i, v);
                    c.bus.write32(data + 4 * i, v);
                }
                if seed % 4 != 0 {
                    c.bus.write32(data + 0x30, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = sx(ctx);
            },
        );
    }

    // ------------------------------------------------------------------- rails

    /// A rail: `elems` at +0x70 holding `n` 0x60-byte points, plus a node
    /// (idx at +0x64, blend at +0x68).
    pub(crate) fn rail(c: &mut Cpu, r: &mut Rng, self_: u32, node: u32, elems: u32, n: u32) {
        word(c, self_ + 0x70, elems);
        // the spring stiffness coefficients at +0x60..+0x70
        for k in (0..0x10).step_by(4) {
            word(c, self_ + 0x60 + k, r.edge_f32());
        }
        word(c, node + 0x64, 0xffff_ffff);
        word(c, node + 0x68, r.edge_f32());
        for i in 0..n {
            for k in (0..0x60).step_by(4) {
                word(c, elems + 0x60 * i + k, r.edge_f32());
            }
        }
        // the node's own vectors
        for k in 0..8 {
            word(c, node + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x20 + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x40 + 4 * k, r.edge_f32());
        }
        // the trailing k[] coefficient the spring reads at elems + n*0x60 + 8
        word(c, elems + 0x60 * n + 8, r.edge_f32());
    }

    #[test]
    fn float_rail_apply_impulse_matches() {
        check_lohi(
            ssx3_ee::s11::f_00349798__generated,
            float_rail_apply_impulse,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x349798, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
                if seed % 4 != 0 {
                    word(c, DATA + 0x1064, (seed % 3) as u32);
                }
                for k in 0..4 {
                    word(c, DATA + 0x1800 + 4 * k, r.f32(10.0));
                }
            },
        );
    }

    #[test]
    fn float_rail_get_contact_velocity_matches() {
        check_lohi(
            ssx3_ee::s11::f_00349840__generated,
            float_rail_get_contact_velocity,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x349840, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
                if seed % 4 != 0 {
                    word(c, DATA + 0x1064, (seed % 3) as u32);
                }
            },
        );
    }

    #[test]
    fn spring_rail_apply_impulse_matches() {
        check_lohi(
            ssx3_ee::s11::f_0034a568__generated,
            spring_rail_apply_impulse,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34a568, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1800);
                rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 4);
                let idx = match seed % 5 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 1,
                    3 => 2,
                    _ => 3,
                };
                word(c, DATA + 0x1064, idx);
                // directions: tiny, medium and over the 200 length clamp
                let scale: f32 = match seed % 4 {
                    0 => 1.0e-6,
                    1 => 1.0,
                    2 => 100.0,
                    _ => 1000.0,
                };
                for k in 0..4 {
                    word(c, DATA + 0x1800 + 4 * k, r.f32(scale));
                }
            },
        );
    }

    #[test]
    fn float_rail_v01_matches() {
        check(
            ee::s11::f_003490b0__generated,
            float_rail_v01,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x3490_b0);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                // `FlexRail_UnregisterSpaceNode` -> `World_GetNash()` and
                // `SpatialHash_Remove`: if the cell's level word does not match
                // the key's (11, from `SpatialKey_FromAABB`), the latter descends
                // into the (null) child forever. Give it a real cell whose level
                // matches, so the non-recursive path is taken.
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let cell = DATA + 0x3000;
                clear(c, world, 0x40);
                clear(c, man, 0x40);
                clear(c, cell, 0x40);
                w(c, cell, 11);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x20, cell as u32);
                // No node and no owned memory: no allocator at all on this path.
                w(c, s + 0x54, 0);
                w(c, s + 0x70, 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn spring_rail_v01_matches() {
        check(
            ee::s11::f_00349eb8__generated,
            spring_rail_v01,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x349e_b8);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                // See `float_rail_v01_matches`: match the spatial hash's level.
                let world = DATA + 0x1000;
                let man = DATA + 0x2000;
                let cell = DATA + 0x3000;
                clear(c, world, 0x40);
                clear(c, man, 0x40);
                clear(c, cell, 0x40);
                w(c, cell, 11);
                w(c, gp_at(-0x848), world as u32);
                w(c, world + 0x84, man as u32);
                w(c, man + 0x20, cell as u32);
                w(c, s + 0x54, 0);
                w(c, s + 0x70, 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn float_rail_save_matches() {
        check(
            ee::s11::f_00349ad0__generated,
            float_rail_save,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x349ad0, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w_2(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w_2(c, this + 0x20, seed % 5);
                w_2(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn spring_rail_save_matches() {
        check(
            ee::s11::f_0034ac10__generated,
            spring_rail_save,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x34ac10, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w_2(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w_2(c, this + 0x20, seed % 5);
                w_2(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn c_flag_manager_group_ctor_matches() {
        check(
            ee::s11::f_0034af38__generated,
            c_flag_manager_group_ctor,
            12,
            &|c, seed| {
                let _ = base_2(c, seed, 0x34af_38, 4);
                w_3(c, GAME_GP.wrapping_add(0xf20), (seed % 3) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn flag_ctor_matches() {
        check(
            ee::s11::f_0034add8__generated,
            flag_ctor,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34ad, 0);
                env(c);
                let this = DATA + 0x400;
                let node = DATA + 0x800;
                let src = DATA + 0x1000;
                w32(c, gpat(-0xc8), 0);
                w32(c, node + 8, r.next_u32());
                for k in 0..8u32 {
                    w32(c, src + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(node);
                c.g[A2][0] = sx(src);
                c.g[A3][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// Scrambled registers and a game-like `gp`.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x143_0000 ^ ((seed as u64) << 11) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Compare hard state, caller-saved "soft" state and LO/HI for every seed.
    pub(crate) fn check_all(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    pub(crate) fn floats(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.edge_f32());
        }
    }

    // -----------------------------------------------------------------------

    #[test]
    fn float_rail_v50_matches() {
        check_all(
            ee::s11::f_00349110__generated,
            float_rail_v50,
            300,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x34_9110);
                c.g[A0][0] = sx(DATA);
                let count = if seed % 5 == 0 {
                    seed % 2
                } else {
                    2 + r.next_u32() % 40
                };
                c.bus.write32(DATA + 0x70, DATA + 0x2000);
                c.bus.write32(DATA + 0x20, count);
                // The first point comes from +0x20 of the array, the rest from
                // +0x70 with a 0x50 stride.
                floats(c, &mut r, DATA + 0x2000, count * 0x50 / 4 + 8);
                floats(c, &mut r, DATA, 2); // the `lq` seeds from +0x30/from +0x40
                floats(c, &mut r, DATA + 0x10, 2);
                for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                    c.bus.write32(DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn spring_rail_v50_matches() {
        check_all(
            ee::s11::f_00349f18__generated,
            spring_rail_v50,
            300,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x34_9f18);
                c.g[A0][0] = sx(DATA);
                let count = if seed % 5 == 0 {
                    seed % 2
                } else {
                    2 + r.next_u32() % 40
                };
                c.bus.write32(DATA + 0x70, DATA + 0x2000);
                c.bus.write32(DATA + 0x20, count);
                floats(c, &mut r, DATA + 0x2000, count * 0x60 / 4 + 8);
                for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                    c.bus.write32(DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn flag_read_params_matches() {
        same(
            ssx3_ee::s11::f_0034ac88__generated,
            flag_read_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x34ac88 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let dst = DATA + 0x100;
                let src = DATA + 0x300;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for i in 0..0x68u32 / 4 {
                    c.bus.write32(src + 4 * i, r.next_u32());
                }
                let hz = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, hz);
                let rate = if seed % 5 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 240) as i32
                };
                c.bus.write32(hz + 0x10, rate as u32);
            },
        );
    }

    use crate::difftest::fakes;

    /// GroupRemoveFlag: a group of 0..6 riders (ids from a small set so
    /// matches and misses both happen), a rider on the group's track or not;
    /// the matched entry moved, the count reaching 0 calls the real
    /// `GroupFreeMesh` on its short path (no mesh, no buffer).
    #[test]
    fn c_flag_manager_group_remove_flag_matches() {
        const GROUP: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const TRACK_CELL: u32 = DATA + 0x2000;
        const MEMBERS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_0034b168__generated,
            c_flag_manager_group_remove_flag,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34b168, 0);
                fill(c, &mut r, GROUP, 0x40);
                let count: i32 = match seed % 8 {
                    0 => 0,
                    1 => -1,
                    2 => 1,
                    _ => (r.next_u32() % 6 + 1) as i32,
                };
                let track = r.next_u32();
                c.bus.write32(GROUP + 0x58, track);
                c.bus.write32(GROUP + 0x5c, count as u32);
                // Mesh and buffer null: GroupFreeMesh takes its short path.
                c.bus.write32(GROUP + 0x60, 0);
                c.bus.write32(GROUP + 0x74, 0);
                c.bus.write32(RIDER + 0x80, TRACK_CELL);
                let same_track = seed % 5 != 3;
                c.bus
                    .write32(TRACK_CELL, if same_track { track } else { track ^ 1 });
                let id = r.next_u32() % 4;
                c.bus.write32(RIDER + 0x78, id);
                for i in 0..6u32 {
                    let m = MEMBERS + 0x100 * i;
                    c.bus.write32(GROUP + 0x94 + 4 * i, m);
                    c.bus.write32(m + 0x78, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(GROUP);
                c.g[A1][0] = sx(RIDER);
                c.pc = 0x0034_b168;
            },
        );
    }

    pub(crate) use crate::game::object::flagset::tests::oc308_flag_manager;

    /// The world's flag manager (World at gp-0x848, +0x84, +0x70) with the
    /// flag's instance in some groups; the WorldObj base destructor runs for
    /// real (no free: `flags & 1` clear).
    #[test]
    fn flag_dtor_matches() {
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s11::f_0034aea0__generated,
            flag_dtor,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_aea0, 0);
                let obj = DATA;
                let inst = DATA + 0x100;
                let world = DATA + 0x3_0000;
                let ai = DATA + 0x3_1000;
                let mgr = DATA + 0x4_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, ai);
                c.bus.write32(ai + 0x70, mgr);
                oc308_flag_manager(c, &mut r, mgr, inst);
                // WorldObj part (as in WorldObj_dtor's test).
                crate::difftest::fill(c, &mut r, obj, 8);
                c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, 0);
                } else {
                    c.bus.write32(obj + 4, DATA + 0x200);
                    c.bus
                        .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x300 });
                }
                c.bus.write32(obj + FLAG_DTOR_INSTANCE, inst);
                c.bus.write32(inst + 0x8, r.next_u32());
                c.bus
                    .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (r.next_u32() & !1) as u64;
                c.pc = 0x0034_aea0;
            },
        );
    }

    /// `Flag_Save`: the real `WorldObj_Save` / `SyncProcess_Save` on a
    /// random object; the stream's write methods are fakes, some unwinding
    /// (fixture as in `mod_obj_save_matches`).
    #[test]
    fn flag_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{fill, DATA};
        covered(
            ee::s11::f_0034aef8__generated,
            flag_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_aef8, 4);
                c.pc = 0x0034_aef8;
                let (this, stream) = (DATA + 0x100, DATA + 0x400);
                fill(c, &mut r, this, 0x10);
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    /// `cFlagManager_GroupAddFlag`: the first flag (copy the definition,
    /// aligned and unaligned, build the mesh: the graphics vtable's mesh
    /// call returns 0, so the build returns early) and later flags.
    #[test]
    fn c_flag_manager_group_add_flag_matches() {
        check(
            ee::s11::f_0034b038__generated,
            c_flag_manager_group_add_flag,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b038, 0);
                c.pc = 0x0034_b038;
                let this = DATA + 0x100;
                let def = DATA + 0x600 + (seed % 2);
                let mesh = DATA + 0x700;
                let count = if seed % 3 == 0 { 0 } else { seed % 5 };
                c.bus.write32(this + 0x5c, count);
                for k in 0..0x40u32 {
                    c.bus.write8(def + k, r.next_u32() as u8);
                }
                c.bus.write32(mesh + 0x80, DATA + 0x780);
                c.bus.write32(DATA + 0x780, r.next_u32());
                // The graphics manager: vtable slot 97 is a fake, slot 76 a
                // real leaf that returns 0 (no mesh).
                let mgr = DATA + 0x4000;
                let vt = DATA + 0x4100;
                c.bus.write32(GAME_GP + 0x2a90, mgr);
                c.bus.write32(mgr + 0x10d8, vt);
                c.bus.write16(vt + 0x308, 0);
                c.bus.write32(vt + 0x30c, FAKE);
                c.bus.write16(vt + 0x260, 0);
                c.bus.write32(vt + 0x264, 0x0014_06c0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(def);
                c.g[A2][0] = sx(mesh);
            },
        );
    }

    use crate::difftest::elf_image;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    use crate::difftest;

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::object::flexbridgenode::tests::OC603_CTRL;
    pub(crate) use crate::game::object::flexbridgenode::tests::OC603_KEY;
    pub(crate) use crate::game::object::flexbridgenode::tests::OC603_MAGIC;
    pub(crate) use crate::game::object::flexbridgenode::tests::OC603_OBJ;

    pub(crate) use crate::game::object::flexbridgenode::tests::oc603_install;

    /// The rail node (`self`) and its node array, `Mats` and the lookup tables.
    pub(crate) const SELF: u32 = DATA + 0x4000;

    pub(crate) const MATS: u32 = DATA + 0x2000;

    pub(crate) const NODES: u32 = DATA + 0x3000;

    /// Gate for the fakes: a magic word in this machine's RAM.
    pub(crate) const CTRL: u32 = DATA + 0x20000;

    pub(crate) const MAGIC: u32 = 0x0596_0596;

    pub(crate) const TBL: u32 = DATA + 0x6000;

    pub(crate) const RAIL: u32 = DATA + 0x6400;

    pub(crate) const TBL2: u32 = DATA + 0x6800;

    pub(crate) const WORLD: u32 = DATA + 0x6c00;

    pub(crate) const WORLD_OBJ: u32 = DATA + 0x6d00;

    pub(crate) const ACTIVE: u32 = DATA + 0x9000;

    pub(crate) static OC596_PREV_WORLD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC596_PREV_DT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC596_PREV_MATRIX: AtomicUsize = AtomicUsize::new(0);

    /// Gated `World_GetInstance`: returns the fixture's world object.
    pub(crate) fn oc596_fake_world_instance(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_WORLD, ee::s09::f_002d1bd8__generated);
        }
        c.g[V0][0] = WORLD as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `World_GetSecondsPerTick`: the step is set in the fixture.
    pub(crate) fn oc596_fake_seconds_per_tick(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_DT, ee::s09::f_002d1c70__generated);
        }
        c.f[0] = c.bus.read32(CTRL + 4);
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `Instance_GetNodeMatrix`: leaves the output on the stack alone.
    pub(crate) fn oc596_fake_node_matrix(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC596_PREV_MATRIX, ee::s11::f_0034fed8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc596_install() {
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

    /// Fixture for both functions: a rail of `1..=5` nodes (the loops take every
    /// branch), random node and matrix data, and the self pointer at `DATA`.
    pub(crate) fn spring_rail_fixture(c: &mut Cpu, seed: u32, r: &mut Rng, entry: u32) {
        scramble(c, r);
        c.pc = entry;
        c.g[RA][0] = sx(difftest::RA);
        c.g[SP][0] = sx(difftest::SP);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + SR_NUM_NODES, seed % 5 + 1);
        c.bus.write32(DATA + SR_MATS, DATA + 0x1000);
        c.bus.write32(DATA + SR_NODES, DATA + 0x2000);
        let edge = seed % 5 == 0;
        // The two spring/damp floats; +0x70 holds the Nodes pointer, so stop at +0x64.
        for k in 0..2 {
            let v = if edge { r.edge_f32() } else { r.f32(2.0) };
            c.bus.write32(DATA + SR_SPRING + 4 * k, v);
        }
        fill(c, r, DATA + 0x1000, 0x100);
        fill(c, r, DATA + 0x2000, 0x180);
        fill(c, r, SR_SEG_XFORM, 16);
        fill(c, r, 0x004f_f120, 16);
    }

    #[test]
    fn spring_rail_node_update_nodes_matches() {
        check(
            ee::s11::f_0034a8c8__generated,
            spring_rail_node_update_nodes,
            300,
            &|c, seed| {
                let mut r = Rng(0x34a8_c8aa ^ seed as u64);
                spring_rail_fixture(c, seed, &mut r, 0x0034_a8c8);
                // The update-time callee reads world->+0x14 through GP-0x848.
                c.bus.write32(GAME_GP - 0x848, DATA + 0x4000);
                c.bus.write32(DATA + 0x4014, r.f32(2.0));
            },
        );
    }

    #[test]
    fn flex_rail_space_node_register_matches() {
        oc603_install();
        covered(
            ee::s11::f_00348b90__generated,
            flex_rail_space_node_register,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8b90, 4);
                c.pc = 0x0034_8b90;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // The octant is picked by the key's signs: each of x, y, z is
                // negative on some seeds.
                for k in 0..3u32 {
                    let v = r.next_u32() & 0xff;
                    let neg = (seed >> k) & 1 == 1;
                    c.bus
                        .write32(OC603_KEY + 4 * k, if neg { 0xffff_ff00 | v } else { v });
                }
                fill(c, &mut r, OC603_OBJ + 0x30, 6);
                c.g[A0][0] = sx(OC603_OBJ);
            },
        );
    }

    /// Gate word for the rail fakes (written into the machine's own RAM).
    pub(crate) const OC660_CTRL: u32 = DATA + 0x2_0000;

    pub(crate) const OC660_MAGIC: u32 = 0x0660_0660;

    /// The node block `operator_new` hands back while the fakes are on.
    pub(crate) const OC660_ALLOC: u32 = DATA + 0x8000;

    pub(crate) static OC660_PREV_FLEX_CTOR_2: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC660_PREV_UPDATE_MATS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC660_PREV_V50: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC660_PREV_SPACE_NODE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC660_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    /// Gated `FlexRail_ctor_2`: returns at once.
    pub(crate) fn oc660_fake_flex_ctor_2(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_FLEX_CTOR_2, ee::s11::f_00347ea8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FloatRail_updateMats`: returns at once.
    pub(crate) fn oc660_fake_update_mats(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_UPDATE_MATS, ee::s11::f_003492a8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FloatRail_v50`: returns at once.
    pub(crate) fn oc660_fake_v50(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_V50, ee::s11::f_00349110__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FlexRail_RegisterSpaceNode`: returns at once.
    pub(crate) fn oc660_fake_space_node(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_SPACE_NODE, ee::s11::f_00348b90__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `operator_new_named`: hands back the fixture's node block.
    pub(crate) fn oc660_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        c.g[V0][0] = OC660_ALLOC as u64;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc660_install() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347EA8),
            oc660_fake_flex_ctor_2,
            &OC660_PREV_FLEX_CTOR_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003492A8),
            oc660_fake_update_mats,
            &OC660_PREV_UPDATE_MATS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349110),
            oc660_fake_v50,
            &OC660_PREV_V50,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348B90),
            oc660_fake_space_node,
            &OC660_PREV_SPACE_NODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc660_fake_new,
            &OC660_PREV_NEW,
        );
    }

    #[test]
    fn float_rail_ctor_2_matches() {
        oc660_install();
        check(
            ee::s11::f_00348fa0__generated,
            float_rail_ctor_2,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x348f, 0);
                c.bus.write32(OC660_CTRL, OC660_MAGIC);
                let this = DATA + 0x1000;
                let stream = DATA + 0x2800;
                let record = DATA + 0x2900;
                c.pc = 0x0034_8fa0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = stream as u64;
                // The stream's record: its reader is a logged fake call.
                c.bus.write32(stream, record);
                c.bus.write16(record + 0x10, (4 * (seed % 8)) as u16);
                c.bus.write32(record + 0x14, FAKE + 0x40);
                // Node count: 0 (no block) to 4.
                c.bus.write32(this + 0x20, seed % 5);
            },
        );
    }

    /// The descriptor of a spring rail: `nodes` non-(-1) instance ids (the
    /// FlexRail base counts them as NumNodes), their meshes, the spring
    /// constants and the flags.
    #[allow(dead_code)]
    pub(crate) fn rail_desc(c: &mut Cpu, r: &mut Rng, desc: u32, nodes: u32) {
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
    #[allow(dead_code)]
    pub(crate) fn rail_stream(c: &mut Cpu, stream: u32, vt: u32) {
        c.bus.write32(stream, vt);
        c.bus.write16(vt + 0x10, 0);
        c.bus.write32(vt + 0x14, 0x0014_06c0);
    }

    /// Gate word of the oc686 callee fakes (in the machine's own RAM).
    pub(crate) const OC686_CTRL: u32 = DATA + 0x000e_8000;

    pub(crate) const OC686_MAGIC: u32 = 0x0686_0686;

    /// What the gated `World_GetInstance` fake returns.
    pub(crate) const OC686_WORLD: u32 = DATA + 0x0001_0000;

    /// The instance object the second table level points to (its scale at +0x84).
    pub(crate) const OC686_INST_OBJ: u32 = DATA + 0x0001_8000;

    /// A gated logging fake for one callee slot: with the gate off the body
    /// that was in the slot runs.
    macro_rules! oc686_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $post:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC686_CTRL) != OC686_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let args = [c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0]];
                fakes::fake_call(c, $addr);
                let post: fn(&mut Cpu, [u64; 4]) = $post;
                post(c, args);
            }
        };
    }

    oc686_fake!(
        oc686_f_world,
        OC686_PREV_WORLD,
        0x002d_1bd8,
        ee::s09::f_002d1bd8__generated,
        |c, _| c.g[V0][0] = OC686_WORLD as u64
    );

    // `Instance_GetNodeMatrix(inst, mesh, out)`: a deterministic matrix at a2.
    oc686_fake!(
        oc686_f_node_matrix,
        OC686_PREV_NODE_MATRIX,
        0x0034_fed8,
        ee::s11::f_0034fed8__generated,
        |c, a| {
            for i in 0..16u32 {
                let w = fakes::mix(a[0] ^ a[1] << 32 ^ i as u64) as u32;
                c.bus.write32(a[2] as u32 + 4 * i, w);
            }
        }
    );

    pub(crate) fn oc686_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BD8),
            oc686_f_world,
            &OC686_PREV_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FED8),
            oc686_f_node_matrix,
            &OC686_PREV_NODE_MATRIX,
        );
        oc686_install_more();
    }

    /// The world's two-level instance table the node placement walks:
    /// `World_GetInstance()` -> object (+0) -> table (+8) indexed by the low
    /// byte of an instance id -> entry (+0x1c) -> table indexed by the rest
    /// of the id; ids are `(a << 8) | b` with `a, b < 4`. `gaps` leaves some
    /// slots empty on either level.
    pub(crate) fn oc686_world_tables(c: &mut Cpu, r: &mut Rng, seed: u32, gaps: bool) {
        let (obj, tbl, ent, tbl2) = (DATA + 0x3000, DATA + 0x4000, DATA + 0x5000, DATA + 0x6000);
        c.bus.write32(OC686_WORLD, obj);
        c.bus.write32(obj + 8, tbl);
        let has_ent = !gaps || seed % 5 != 0;
        for i in 0..4u32 {
            c.bus.write32(tbl + 4 * i, if has_ent { ent } else { 0 });
        }
        c.bus.write32(ent + 0x1c, tbl2);
        for i in 0..4u32 {
            let low = r.next_u32() & 0xff;
            let v = if gaps && (seed / 5 + i) % 3 == 0 {
                low
            } else {
                ((OC686_INST_OBJ >> 2) << 8) | low
            };
            c.bus.write32(tbl2 + 4 * i, v);
        }
    }

    oc686_fake!(
        oc686_f_nash,
        OC686_PREV_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_find_key,
        OC686_PREV_FIND_KEY,
        0x0032_8f28,
        ee::s10::f_00328f28__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_cell_remove,
        OC686_PREV_CELL_REMOVE,
        0x0032_84b8,
        ee::s10::f_003284b8__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_delete,
        OC686_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_new,
        OC686_PREV_NEW,
        0x0031_7e30,
        ee::s10::f_00317e30__generated,
        |c, _| c.g[V0][0] = sx(OC686_ALLOC)
    );

    /// The block `operator_new` hands out.
    pub(crate) const OC686_ALLOC: u32 = DATA + 0x000a_0000;

    pub(crate) fn oc686_install_more() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc686_f_nash,
            &OC686_PREV_NASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc686_f_find_key,
            &OC686_PREV_FIND_KEY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc686_f_cell_remove,
            &OC686_PREV_CELL_REMOVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc686_f_delete,
            &OC686_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc686_f_new,
            &OC686_PREV_NEW,
        );
    }

    /// `FlexRail_UnregisterSpaceNode(self)`: bumps the Nash's delete count,
    /// takes the node out of the cell the bounding box key selects (one of
    /// eight octants, by the key's signs, read from uninitialised frame
    /// memory), deletes the octant's root cell when its 11 slots are all
    /// empty, then deletes the node. The Nash lookup, key finder, cell
    /// removal and `operator_delete` are gated fakes; unwinds at any of them.
    #[test]
    fn flex_rail_unregister_space_node_matches() {
        oc686_install();
        covered(
            ee::s11::f_00348c48__generated,
            flex_rail_unregister_space_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8c48, 5);
                c.pc = 0x0034_8c48;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, nash, root) = (DATA + 0x1000, DATA + 0x4000, DATA + 0x5000);
                c.g[A0][0] = sx(this);
                c.bus.write32(fakes::LOG_RESULTS, nash);
                c.bus.write32(this + 0x54, r.next_u32());
                fill(c, &mut r, this + 0x30, 6);
                c.bus.write32(nash + 0xa0, r.next_u32());
                // Every octant points at one root cell: empty, or with one
                // used slot among the 11 the emptiness test reads.
                for o in 0..8u32 {
                    c.bus.write32(nash + 0x14 * o + 0x10, root);
                }
                for i in 0..12u32 {
                    c.bus.write32(root + 4 * i, 0);
                }
                let used = seed % 12;
                if used < 11 {
                    c.bus.write32(root + 4 * used, r.next_u32() | 1);
                }
            },
        );
    }

    oc686_fake!(
        oc686_f_flex_ctor,
        OC686_PREV_FLEX_CTOR,
        0x0034_7d90,
        ee::s11::f_00347d90__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_flex_ctor_2,
        OC686_PREV_FLEX_CTOR_2,
        0x0034_7ea8,
        ee::s11::f_00347ea8__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_float_mats,
        OC686_PREV_FLOAT_MATS,
        0x0034_92a8,
        ee::s11::f_003492a8__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_float_bounds,
        OC686_PREV_FLOAT_BOUNDS,
        0x0034_9110,
        ee::s11::f_00349110__generated,
        |_, _| {}
    );

    oc686_fake!(
        oc686_f_space_register,
        OC686_PREV_SPACE_REGISTER,
        0x0034_8b90,
        ee::s11::f_00348b90__generated,
        |_, _| {}
    );

    pub(crate) fn oc686_install_rails() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347D90),
            oc686_f_flex_ctor,
            &OC686_PREV_FLEX_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347EA8),
            oc686_f_flex_ctor_2,
            &OC686_PREV_FLEX_CTOR_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003492A8),
            oc686_f_float_mats,
            &OC686_PREV_FLOAT_MATS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349110),
            oc686_f_float_bounds,
            &OC686_PREV_FLOAT_BOUNDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348B90),
            oc686_f_space_register,
            &OC686_PREV_SPACE_REGISTER,
        );
    }

    /// A rail descriptor: `used` of the eight instance id slots (+4) hold ids
    /// (`(a << 8) | b`, `a, b < 4`), the others are -1; meshes at +0x24, the
    /// three spring constants at +0x48.
    pub(crate) fn oc686_rail_desc(c: &mut Cpu, r: &mut Rng, desc: u32, used: u32) {
        let mut left = used;
        for i in 0..8u32 {
            // Spread the used slots: slot i is used with the odds left/(8-i).
            let take = left > 0 && r.next_u32() % (8 - i) < left;
            let id = if take {
                left -= 1;
                (r.next_u32() % 4) << 8 | (r.next_u32() % 4)
            } else {
                0xffff_ffff
            };
            c.bus.write32(desc + 4 + 4 * i, id);
            c.bus.write32(desc + 0x24 + 4 * i, r.next_u32() % 16);
        }
        for k in 0..3u32 {
            c.bus.write32(desc + 0x48 + 4 * k, r.edge_f32());
        }
    }

    /// `FloatRail_ctor(self, bucket, desc)`: the base constructor, the vtable
    /// and spring constants, the descriptor's ids packed into a frame scratch
    /// (0..8 used), the node block allocated and each node placed through
    /// `FloatRail_ResetPosFromInstance` (real, with the world fakes), then
    /// matrices, bound box and space registration. The base constructor, the
    /// allocator and the three finishers are gated fakes; unwinds at any call.
    #[test]
    fn float_rail_ctor_matches() {
        oc686_install();
        oc686_install_rails();
        covered(
            ee::s11::f_00348d98__generated,
            float_rail_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8d98, 7);
                c.pc = 0x0034_8d98;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, desc) = (DATA + 0x1000, DATA + 0x1800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(desc);
                let used = seed % 9;
                oc686_rail_desc(c, &mut r, desc, used);
                // The base constructor (a fake) leaves the node count in the rail.
                c.bus.write32(this + 0x20, used);
                oc686_world_tables(c, &mut r, seed, false);
            },
        );
    }

    /// `FloatRail_ResetPosFromInstance(self, node)`: the node's instance is
    /// found through the world's two-level table (it may be missing at either
    /// level), its bone matrix is fetched into the frame and the translation
    /// row is stored into the node. `World_GetInstance` and
    /// `Instance_GetNodeMatrix` are gated fakes; unwinds at either call.
    #[test]
    fn float_rail_reset_pos_from_instance_matches() {
        oc686_install();
        covered(
            ee::s11::f_00349220__generated,
            float_rail_reset_pos_from_instance_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9220, 2);
                c.pc = 0x0034_9220;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(node);
                // InstId: low byte indexes the first table, the rest the second.
                let id = (r.next_u32() % 4) << 8 | (r.next_u32() % 4);
                c.bus.write32(node + 0x40, id);
                c.bus.write32(node + 0x44, r.next_u32());
                oc686_world_tables(c, &mut r, seed, true);
            },
        );
    }

    oc686_fake!(
        oc686_f_spring_bounds,
        OC686_PREV_SPRING_BOUNDS,
        0x0034_9f18,
        ee::s11::f_00349f18__generated,
        |_, _| {}
    );

    /// The rail fixture both spring constructors share: the matrix array
    /// (`self+0x50`, written by the real `SpringRail_v48`), the constants
    /// it reads and the node block `operator_new` returns.
    pub(crate) fn oc686_spring_rail(c: &mut Cpu, r: &mut Rng, this: u32, nodes: u32) {
        c.bus.write32(this + 0x20, nodes);
        c.bus.write32(this + 0x50, DATA + 0x7000);
        fill(c, r, DATA + 0x7000, 0x100);
        fill(c, r, OC686_ALLOC, 0xc0);
        fill(c, r, 0x004f_fac0, 16);
    }

    /// `SpringRail_ctor(self, bucket, desc)`: the spring rail builder. The
    /// descriptor's used ids are packed into the frame, the node block
    /// (0x60 bytes per node) allocated, each node placed by the real
    /// `SpringRail_InitPosFromInstance`, the segment vectors built, then the
    /// real `SpringRail_v48` (matrices) and the fakes for the base
    /// constructor, allocator, `SpringRail_v50` and the space registration.
    #[test]
    fn spring_rail_ctor_matches() {
        oc686_install();
        oc686_install_rails();
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349F18),
            oc686_f_spring_bounds,
            &OC686_PREV_SPRING_BOUNDS,
        );
        covered(
            ee::s11::f_00349b48__generated,
            spring_rail_ctor_3,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9b48, 8);
                c.pc = 0x0034_9b48;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, desc) = (DATA + 0x1000, DATA + 0x1800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(desc);
                let used = seed % 9;
                oc686_rail_desc(c, &mut r, desc, used);
                oc686_spring_rail(c, &mut r, this, used);
                oc686_world_tables(c, &mut r, seed, false);
            },
        );
    }

    /// `SpringRail_ctor_2(self, bucket, stream)`: the stream form. The two
    /// stream reads (spring constants, then the node block) are virtual
    /// calls into fake methods; the base constructor, the allocator and the
    /// space registration are gated fakes; `SpringRail_v48` runs for real.
    #[test]
    fn spring_rail_ctor_2_matches() {
        oc686_install();
        oc686_install_rails();
        covered(
            ee::s11::f_00349db0__generated,
            spring_rail_ctor_2_2,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9db0, 6);
                c.pc = 0x0034_9db0;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, stream, record) = (DATA + 0x1000, DATA + 0x2800, DATA + 0x2900);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(stream);
                oc686_spring_rail(c, &mut r, this, seed % 7);
                // The stream's read method: a vtable slot (delta, fake address).
                c.bus.write32(stream, record);
                c.bus.write16(record + 0x10, (4 * (seed % 8)) as u16);
                c.bus.write32(record + 0x14, fakes::FAKE + 0x40);
            },
        );
    }

    /// Gate of the fake callees below: written into the test machine's own
    /// RAM, so other tests' machines reach the bodies the slots held.
    pub(crate) const OC687_GATE: u32 = DATA + 0xe_0000;

    /// The instance handle `RailModifier_BuildRailNodes` (fake) stores.
    pub(crate) const OC687_HANDLE: u32 = DATA + 0xe_0100;

    pub(crate) const OC687_MAGIC: u32 = 0x0687_c0de;

    /// Fixture data lives above the area the fakes' returned pointers point
    /// into (`DATA..DATA + 0x40000`).
    pub(crate) const FIX: u32 = DATA + 0x4_0000;

    /// Gated fakes for the callees that have override slots: with the gate
    /// set they log the call and return a pointer (or unwind); otherwise
    /// they run the body the slot held.
    macro_rules! oc687_fakes {
        ($($fake:ident, $shard:ident, $slot:ident, $gen:ident, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC687_GATE) != OC687_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc687_install_plain_fakes() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc687_fakes! {
        oc687_fake_get_nash, s09, OVERRIDE_002D1BE0, f_002d1be0__generated, 0x002d_1be0;
        oc687_fake_node_matrix, s11, OVERRIDE_0034FED8, f_0034fed8__generated, 0x0034_fed8;
        oc687_fake_world_ctor_2, s11, OVERRIDE_0034FB90, f_0034fb90__generated, 0x0034_fb90;
        oc687_fake_alloc_named, s10, OVERRIDE_00317D70, f_00317d70__generated, 0x0031_7d70;
        oc687_fake_new_named, s10, OVERRIDE_00317E30, f_00317e30__generated, 0x0031_7e30;
        oc687_fake_delete, s10, OVERRIDE_00317E50, f_00317e50__generated, 0x0031_7e50;
        oc687_fake_spline_ctor, s11, OVERRIDE_003595D8, f_003595d8__generated, 0x0035_95d8;
        oc687_fake_multi_spline_ctor, s11, OVERRIDE_0035A118, f_0035a118__generated, 0x0035_a118;
        oc687_fake_parent_ctor, s11, OVERRIDE_00357090, f_00357090__generated, 0x0035_7090;
        oc687_fake_position_ctor, s11, OVERRIDE_00356F68, f_00356f68__generated, 0x0035_6f68;
        oc687_fake_roller_ctor, s11, OVERRIDE_0035DF70, f_0035df70__generated, 0x0035_df70;
        oc687_fake_magnet_ctor, s11, OVERRIDE_00357358, f_00357358__generated, 0x0035_7358;
        oc687_fake_ava_ctor, s11, OVERRIDE_00357798, f_00357798__generated, 0x0035_7798;
        oc687_fake_uv_ctor, s12, OVERRIDE_0035F788, f_0035f788__generated, 0x0035_f788;
        oc687_fake_texflip_ctor, s12, OVERRIDE_0035F1A8, f_0035f1a8__generated, 0x0035_f1a8;
        oc687_fake_boost_ctor, s11, OVERRIDE_00342808, f_00342808__generated, 0x0034_2808;
        oc687_fake_particle_ctor, s11, OVERRIDE_003459A8, f_003459a8__generated, 0x0034_59a8;
        oc687_fake_dyn_particle_ctor, s11, OVERRIDE_00345D80, f_00345d80__generated, 0x0034_5d80;
        oc687_fake_halo_ctor, s11, OVERRIDE_003461C0, f_003461c0__generated, 0x0034_61c0;
        oc687_fake_list_push_back, s11, OVERRIDE_00345720, f_00345720__generated, 0x0034_5720;
        oc687_fake_rail_list_push_back, s11, OVERRIDE_0035B690, f_0035b690__generated, 0x0035_b690;
        oc687_fake_key_from_aabb, s10, OVERRIDE_00328F28, f_00328f28__generated, 0x0032_8f28;
        oc687_fake_hash_insert, s10, OVERRIDE_00328C20, f_00328c20__generated, 0x0032_8c20;
        oc687_fake_hash_remove, s10, OVERRIDE_003284B8, f_003284b8__generated, 0x0032_84b8;
        oc687_fake_hash_update, s10, OVERRIDE_003291E0, f_003291e0__generated, 0x0032_91e0;
        oc687_fake_mod_obj_v50, s11, OVERRIDE_003568B0, f_003568b0__generated, 0x0035_68b0;
        oc687_fake_anim_v52, s11, OVERRIDE_0034E348, f_0034e348__generated, 0x0034_e348;
        oc687_fake_alloc_node_info, s11, OVERRIDE_0034E448, f_0034e448__generated, 0x0034_e448;
        oc687_fake_alloc_bound_box, s11, OVERRIDE_0034EEB8, f_0034eeb8__generated, 0x0034_eeb8;
        oc687_fake_create_block, s11, OVERRIDE_00355468, f_00355468__generated, 0x0035_5468;
        oc687_fake_mod_obj_v48, s11, OVERRIDE_00356780, f_00356780__generated, 0x0035_6780;
        oc687_fake_anim_v15, s11, OVERRIDE_00341D48, f_00341d48__generated, 0x0034_1d48;
        oc687_fake_free_block, s12, OVERRIDE_003663D8, f_003663d8__generated, 0x0036_63d8;
    }

    /// `RailModifier_BuildRailNodes` (fake): logs the call like the others,
    /// and stores the handle the fixture chose at `rail + 0x30` (the real one
    /// stores the instance handle there).
    pub(crate) fn oc687_fake_build_rail_nodes(c: &mut Cpu) {
        if c.bus.read32(OC687_GATE) != OC687_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc687_fake_build_rail_nodes),
                ee::s11::f_0035c0e8__generated,
            );
        }
        let rail = c.g[A0][0] as u32;
        let handle = c.bus.read32(OC687_HANDLE);
        fakes::fake_call(c, 0x0035_c0e8);
        c.bus.write32(rail.wrapping_add(0x30), handle);
    }

    pub(crate) fn oc687_install_fakes() {
        oc687_install_plain_fakes();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0035C0E8),
            oc687_fake_build_rail_nodes,
            fakes::prev_cell(oc687_fake_build_rail_nodes),
        );
    }

    /// Turn the fakes on in this machine.
    pub(crate) fn oc687_gate(c: &mut Cpu) {
        c.bus.write32(OC687_GATE, OC687_MAGIC);
    }

    /// `SpringRail_InitPosFromInstance`: the instance's handle (low byte and
    /// the rest) walks the world's two-level instance table (entry missing,
    /// or second level empty, or found); the node matrix call is a fake.
    #[test]
    fn spring_rail_init_pos_from_instance_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034a028__generated,
            spring_rail_init_pos_from_instance_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_a028, 1);
                oc687_gate(c);
                c.pc = 0x0034_a028;
                let (inst, world, w1, t1, e, t2) = (
                    FIX,
                    FIX + 0x1000,
                    FIX + 0x1100,
                    FIX + 0x1200,
                    FIX + 0x1300,
                    FIX + 0x1400,
                );
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, w1);
                c.bus.write32(w1 + 8, t1);
                let (lo, hi) = (r.next_u32() % 8, r.next_u32() % 8);
                c.bus.write32(inst + 0x50, lo | hi << 8);
                c.bus.write32(inst + 0x54, r.next_u32());
                c.bus
                    .write32(t1 + lo * 4, if seed % 4 == 0 { 0 } else { e });
                c.bus.write32(e + 0x1c, t2);
                let x = if seed % 3 == 0 {
                    r.next_u32() & 0xff
                } else {
                    r.next_u32() | 0x100
                };
                c.bus.write32(t2 + hi * 4, x);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = inst as u64;
            },
        );
    }

    /// `SpringRail_v48` (leaf): 1..6 nodes (no loop, one pass, several), random
    /// node floats and matrices; the matrix constant at 0x4ffac0.
    #[test]
    fn spring_rail_v48_matches() {
        check_lohi(
            ee::s11::f_0034a0b0__generated,
            spring_rail_v48,
            300,
            &|c, seed| {
                let mut r = Rng(0x34a0_b0aa ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0034_a0b0;
                c.g[GP][0] = GAME_GP as u64;
                let (rail, mats, nodes) = (FIX, FIX + 0x1000, FIX + 0x2000);
                c.g[A0][0] = rail as u64;
                c.bus.write32(rail + 0x20, seed % 6 + 1);
                c.bus.write32(rail + 0x50, mats);
                c.bus.write32(rail + 0x70, nodes);
                fill(c, &mut r, mats, 0x100);
                for i in 0..0x180 {
                    let v = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(4.0)
                    };
                    c.bus.write32(nodes + 4 * i, v);
                }
                for i in 0..16 {
                    c.bus.write32(0x004f_fac0 + 4 * i, r.f32(2.0));
                }
            },
        );
    }
}
