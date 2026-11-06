//! `src/object/splinemodifier.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::object::debouncenode::p;
pub(crate) use crate::game::object::flagset::gp_float;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::gp;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::float;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_95d8, spline_modifier_ctor_2, "SplineModifier_ctor_2"),
    (
        0x0035_9688,
        spline_modifier_set_speed,
        "SplineModifier_SetSpeed",
    ),
    (0x0035_9cf8, spline_modifier_v22, "SplineModifier_v22"),
    (0x0035_9eb8, spline_modifier_v05, "SplineModifier_v05"),
    (
        0x0035_a250,
        multisplinemodifier_v01,
        "MultiSplineModifier_v01",
    ),
    (
        0x0035_a3f0,
        multi_spline_modifier_alloc_node_instances,
        "MultiSplineModifier_AllocNodeInstances",
    ),
    (
        0x0035_a550,
        multi_spline_modifier_set_speed,
        "MultiSplineModifier_SetSpeed",
    ),
    (
        0x0035_a560,
        multi_spline_modifier_update,
        "MultiSplineModifier_Update",
    ),
    (
        0x0035_a918,
        multi_spline_modifier_v24,
        "MultiSplineModifier_v24",
    ),
    (
        0x0035_aae0,
        multi_spline_modifier_release,
        "MultiSplineModifier_Release",
    ),
    (
        0x0035_aaf0,
        multi_spline_modifier_v19,
        "MultiSplineModifier_v19",
    ),
    (
        0x0035_ab60,
        multi_spline_modifier_v23,
        "MultiSplineModifier_v23",
    ),
    (
        0x0035_b418,
        multi_spline_modifier_v11,
        "MultiSplineModifier_v11",
    ),
    (
        0x0035_b670,
        rail_modifier_list_push_front,
        "RailModifier_ListPushFront",
    ),
    (
        0x0035_b690,
        rail_modifier_list_push_back,
        "RailModifier_ListPushBack",
    ),
    (
        0x0035_b6d0,
        rail_modifier_list_unlink,
        "RailModifier_ListUnlink",
    ),
    (
        0x0035_ac20,
        multi_spline_modifier_update_nodes,
        "MultiSplineModifier_UpdateNodes",
    ),
    (
        0x0035_9830,
        spline_modifier_update_transform,
        "SplineModifier_UpdateTransform",
    ),
    (0x0032_84b8, spatial_hash_remove, "SpatialHash_Remove"),
    (0x0035_9698, spline_modifier_update, "SplineModifier_Update"),
    (
        0x0035_b5a8,
        multi_spline_modifier_save,
        "MultiSplineModifier_Save",
    ),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
}

/// `MultiSplineModifier` fields: the element array (`+0x3c`, stride 0x60) and
/// instance pointer array (`+0x44`) are parallel; the box is `{min, max}` at
/// `+0x30`/`+0x40`.
pub(crate) mod spline {

    pub const COUNT: u32 = 0x4;

    pub const DIRTY: u32 = 0x30;

    pub const BOXES: u32 = 0x3c;

    pub const INSTS: u32 = 0x44;

    pub const MODEL: u32 = 0x40;

    pub const MODE: u32 = 0x38;

    pub const FOLLOW: u32 = 0x1c;

    pub const RADIUS: u32 = 0x18;

    pub const ELEM_SIZE: u32 = 0x60;
}

/// `void SplineModifier_SetSpeed(SplineModifier *m, float speed)` (0x359688):
/// store `speed * <constant>` into `m + 0x48`.
pub fn spline_modifier_set_speed(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.f[0] = gp_float(c, 0x2a60).0;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    this.off(0x48).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

/// `void MultiSplineModifier_SetSpeed(MultiSplineModifier *m, float speed)`
/// (0x35a550): store `speed * <constant>` into `m + 0x14`.
pub fn multi_spline_modifier_set_speed(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.f[0] = gp_float(c, 0x2a28).0;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    this.off(0x14).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

/// `void RailModifier_ListPushFront(RailModifierList *list, void *m)`
/// (0x35b670): push `m` at the head.
pub fn rail_modifier_list_push_front(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let m = c.g[A1][0];
    let head = list.lw(c);
    c.g[V0][0] = head;
    if head != 0 {
        Addr(head as u32).off(4).sw(c, m);
    }
    c.cycles += 5;
    Addr(m as u32).off(4).sw(c, list.0 as u64);
    c.g[V0][0] = list.lw(c);
    Addr(m as u32).sw(c, c.g[V0][0]);
    list.sw(c, m);
    abi::ret(c);
}

/// `void RailModifier_ListUnlink(void *m)` (0x35b6d0): unlink `m` from its
/// doubly linked list.
pub fn rail_modifier_list_unlink(c: &mut Cpu) {
    c.cycles += 3;
    let m = Addr(abi::arg(c, 0));
    c.g[V1][0] = m.lw(c);
    if c.g[V1][0] == 0 {
        c.g[V1][0] = m.off(4).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.off(4).lw(c);
        Addr(c.g[V1][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[V1][0] = m.off(4).lw(c);
    }
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        m.off(4).sw(c, 0);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.lw(c);
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        m.off(4).sw(c, 0);
    }
    c.cycles += 2;
    m.sw(c, 0);
    abi::ret(c);
}

/// `void MultiSplineModifier_v24(MultiSplineModifier *self)` (0x0035a918):
/// rebuild when dirty, then for every element recompute the world-space box
/// from the instance position and update the instance and spatial hash.
pub fn multi_spline_modifier_v24(c: &mut Cpu) {
    use spline::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, DIRTY));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.ldw(c.g[S3][0] as u32);
            c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
            c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = addu(c.g[S3][0], c.g[A0][0]);
            call_ptr(c, target, 0x0035_a958)?;
            c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
        }
        c.cycles += 4;
        c.g[S4][0] = 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let no_elems = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xb0);
        if !no_elems {
            c.cycles += 1;
            c.g[S1][0] = ELEM_SIZE as u64;
            loop {
                c.cycles += 80;
                c.g[A1][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.mult(0, S1, S4, S1);
                c.lwc1(0, off(c.g[S3][0] as u32, RADIUS));
                c.g[A0][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[S2][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S4][0] = sx(off(c.g[S4][0] as u32, 1));
                c.g[A0][0] = addu(c.g[S2][0], c.g[A0][0]);
                c.g[A1][0] = addu(c.g[S1][0], c.g[A1][0]);
                c.lq(2, off(c.g[A1][0] as u32, 0x30));
                c.sq(2, c.g[SP][0] as u32);
                c.lq(3, off(c.g[A1][0] as u32, 0x40));
                c.swc1(0, off(c.g[SP][0] as u32, 0x28));
                c.swc1(0, off(c.g[SP][0] as u32, 0x20));
                c.swc1(0, off(c.g[SP][0] as u32, 0x24));
                c.sq(3, off(c.g[SP][0] as u32, 0x10));
                c.stw(off(c.g[SP][0] as u32, 0x2c), 0);
                c.g[V1][0] = c.ldw(c.g[A0][0] as u32);
                c.lq(2, off(c.g[V1][0] as u32, 0x40));
                c.sq(2, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(3, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(4, off(c.g[SP][0] as u32, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, off(c.g[SP][0] as u32, 0x50));
                c.lq(2, off(c.g[SP][0] as u32, 0x50));
                c.sq(2, off(c.g[SP][0] as u32, 0x30));
                c.sq(2, off(c.g[A1][0] as u32, 0x30));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[A1][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                c.g[A1][0] = addu(c.g[S1][0], c.g[A1][0]);
                c.lq(2, off(c.g[A0][0] as u32, 0x40));
                c.sq(2, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(3, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(4, off(c.g[SP][0] as u32, 0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, off(c.g[SP][0] as u32, 0x50));
                c.lq(2, off(c.g[SP][0] as u32, 0x50));
                c.sq(2, off(c.g[SP][0] as u32, 0x30));
                c.sq(2, off(c.g[A1][0] as u32, 0x40));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.g[S0][0] = c.ldw(c.g[V1][0] as u32);
                callee::WORLD_GET_NASH.call(c, 0x0035_aa20)?;
                c.g[A3][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A3][0] = addu(c.g[A3][0], c.g[S1][0]);
                c.g[T0][0] = c.g[SP][0];
                c.g[A3][0] = sx(off(c.g[A3][0] as u32, 0x30));
                callee::SPATIAL_HASH_UPDATE.call(c, 0x0035_aa40)?;
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[V0][0] = addu(c.g[S1][0], c.g[V0][0]);
                c.g[V0][0] = sx(off(c.g[V0][0] as u32, 0x30));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.lwc1(0, c.g[V0][0] as u32);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                c.swc1(0, off(c.g[A0][0] as u32, 0x60));
                c.lwc1(1, off(c.g[V0][0] as u32, 4));
                c.swc1(1, off(c.g[A0][0] as u32, 0x64));
                c.lwc1(0, off(c.g[V0][0] as u32, 8));
                c.swc1(0, off(c.g[A0][0] as u32, 0x68));
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                c.g[S1][0] = sx(off(c.g[S1][0] as u32, 0x40));
                c.g[S2][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.lwc1(0, c.g[S1][0] as u32);
                c.g[V1][0] = c.ldw(c.g[S2][0] as u32);
                c.swc1(0, off(c.g[V1][0] as u32, 0x6c));
                c.lwc1(1, off(c.g[S1][0] as u32, 4));
                c.swc1(1, off(c.g[V1][0] as u32, 0x70));
                c.lwc1(0, off(c.g[S1][0] as u32, 8));
                c.swc1(0, off(c.g[V1][0] as u32, 0x74));
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = ELEM_SIZE as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a970);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiSplineModifier_v11(MultiSplineModifier *self)` (0x0035b418):
/// rebuild when dirty, then update every instance's box against the spatial
/// hash and select the model's vertex format.
pub fn multi_spline_modifier_v11(c: &mut Cpu) {
    use spline::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S4, 0x80);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, DIRTY));
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, MODEL));
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.ldw(c.g[S4][0] as u32);
            c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
            c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = addu(c.g[S4][0], c.g[A0][0]);
            call_ptr(c, target, 0x0035_b464)?;
            c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, MODEL));
        }
        c.cycles += 6;
        c.g[S6][0] = 1;
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
        c.g[S5][0] = c.ldw(off(c.g[V1][0] as u32, 0xc));
        c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let no_elems = c.g[V0][0] == 0;
        c.g[S7][0] = c.ldw(gp(c, 0x2a90));
        if !no_elems {
            c.cycles += 2;
            c.g[V0][0] = 0x60;
            loop {
                c.cycles += 23;
                c.g[S1][0] = c.ldw(off(c.g[S7][0] as u32, 0x10d8));
                c.mult(0, V0, S6, V0);
                c.g[S0][0] = c.ldw(off(c.g[S4][0] as u32, BOXES));
                c.g[S2][0] = c.ldh(off(c.g[S1][0] as u32, 0x2e8));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.ldh(off(c.g[S1][0] as u32, 0x158));
                c.g[V1][0] = c.ldw(off(c.g[S1][0] as u32, 0x15c));
                c.g[S2][0] = addu(c.g[S7][0], c.g[S2][0]);
                c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                c.g[A1][0] = addu(c.g[S7][0], c.g[A1][0]);
                c.g[S3][0] = sx(off(c.g[S0][0] as u32, 0x40));
                let target = c.g[V1][0] as u32;
                c.g[S0][0] = sx(off(c.g[S0][0] as u32, 0x30));
                call_ptr(c, target, 0x0035_b4bc)?;
                c.g[V0][0] = c.ldw(off(c.g[S1][0] as u32, 0x2ec));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                let target = c.g[V0][0] as u32;
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, target, 0x0035_b4d4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                let hit = c.g[S0][0];
                let hit_one = hit == c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                let mut skip_extra = false;
                if !hit_one {
                    c.cycles += 12;
                    c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, INSTS));
                    c.g[A3][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                    c.g[A2][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                    c.g[A1][0] = c.ldw(c.g[V0][0] as u32);
                    c.g[A0][0] = c.ldh(off(c.g[A3][0] as u32, 0x108));
                    c.g[V0][0] = c.ldw(off(c.g[A3][0] as u32, 0x10c));
                    c.g[A1][0] = sx(off(c.g[A1][0] as u32, 0x10));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0035_b50c)?;
                    let miss = c.g[S0][0] == 0;
                    c.g[A2][0] = 0x420;
                    if miss {
                        c.cycles += 7;
                        c.g[V0][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                        c.g[A2][0] = 0x400;
                        c.g[A1][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x28));
                        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x2c));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0035_b550)?;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                        c.g[A1][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x28));
                        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x2c));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0035_b52c)?;
                        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
                        skip_extra = true;
                    }
                }
                if !skip_extra {
                    c.cycles += 1;
                    c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
                }
                c.cycles += 4;
                c.g[S6][0] = sx(off(c.g[S6][0] as u32, 1));
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = 0x60;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_b488);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, FOLLOW));
        let no_follow = c.g[V0][0] == 0;
        c.g[A1][0] = sx(off(c.g[S4][0] as u32, 0x20));
        if !no_follow {
            c.cycles += 2;
            c.g[A0][0] = sx(off(c.g[S4][0] as u32, 0x48));
            callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH.call(c, 0x0035_b578)?;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `float World_GetSecondsPerTick(void)` (0x2d1c70): a cross-shard call the
/// generator emitted as `call_indirect`.
pub(crate) const WORLD_GET_SECONDS_PER_TICK: u32 = 0x002d_1c70;

/// `fN = v`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// `swc1 fN, a`.
#[inline(always)]
pub(crate) fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = FpuF32(c.f[n]);
    Addr(a).write_fpu(c, v);
}

/// `c.lt.s`/`c.le.s` then the FPU condition bit.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_Update.

/// `void MultiSplineModifier_Update(void* self)` (0x35a560): advance the
/// modifier's position `+0x10` by `+0x14 * World_GetSecondsPerTick()`,
/// wrapping into `[0, +0x54)` (`+0x54` is the path length), then set the
/// dirty flag `+0x30`.
pub fn multi_spline_modifier_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, WORLD_GET_SECONDS_PER_TICK, 0x0035_a574)?;
        let this = r32(c, S0);
        // t = pos + rate * dt
        let rate = ldf(c, 1, p(this, 0x14));
        let pos = ldf(c, 2, p(this, 0x10));
        setf(c, 1, rate * FpuF32(c.f[0]));
        c.mtc1(0, 0);
        let t = setf(c, 2, pos + FpuF32(c.f[1]));
        let negative = fcc(c, t.lt(FpuF32(c.f[0])));
        stf(c, 2, p(this, 0x10));
        if negative {
            // Negative: wrap past the end.
            c.cycles += 3;
            let len = ldf(c, 0, p(this, 0x54));
            setf(c, 0, t + len);
            c.cycles += 2;
            stf(c, 0, p(this, 0x10));
            c.g[V0][0] = 1;
        } else {
            c.cycles += 5;
            let len = ldf(c, 0, p(this, 0x54));
            let over = fcc(c, len.le(t));
            c.g[V0][0] = 1;
            if over {
                c.cycles += 1;
                setf(c, 0, t - len);
                c.cycles += 2;
                stf(c, 0, p(this, 0x10));
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        let v = c.g[V0][0];
        c.sw(p(this, 0x30), v);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void RailModifier_ListPushBack(Link** list, Link* item)` (0x35B690): append
/// `item` to a doubly linked list whose head word sits at `*list`.
pub fn rail_modifier_list_push_back(c: &mut Cpu) {
    c.cycles += 3;
    let head_slot = r(c, A0);
    let item = c.g[A1][0];
    let first = c.lw_r_2(V0, head_slot);
    let mut link = head_slot;
    if first != 0 {
        c.cycles += 1;
        link = c.lw_r_2(A0, head_slot) as u32;
        loop {
            c.cycles += 7;
            let next = c.lw_r_2(V0, link);
            if next == 0 {
                c.cycles += 1;
                Addr(item as u32).sw(c, 0);
                break;
            }
            link = c.lw_r_2(A0, link) as u32;
            if event_due(c) {
                deliver_event_here(c, 0x0035_b6a0);
            }
        }
    } else {
        Addr(item as u32).sw(c, 0);
    }
    c.cycles += 3;
    Addr((item as u32).wrapping_add(4)).sw(c, sx(link));
    Addr(link).sw(c, item);
    abi::ret(c);
}

/// `void SplineModifier_v22(SplineModifier* self, Body* body)` (0x00359cf8):
/// turn the spline point's tangent and curvature into a velocity and an
/// angular velocity for the body.
pub fn spline_modifier_v22(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    let body = abi::arg(c, 1);
    c.g[A2][0] = sx(self_);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x54));
    let inactive = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    if inactive {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x50));
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
        return;
    }
    // b = -d0/(d0+d4) * (c4/dir.x - dir.y*c0/d0)
    // a = (1/sqrt(d0+d4)) * (c8 - dir.z*(c0+c4)/(d0+d4))
    c.cycles += 8;
    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xb0)); // dir.x
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a30)); // 1e-5f
    c.abs_s(0, 2);
    c.mtc1(0, 5); // f5 = 0 (small-tangent case)
    c.c_lt_s(1, 0); // 1e-5 < |dir.x|
    let small = !c.fcc();
    c.mov_s(8, 5);
    if small {
        c.cycles += 69;
    } else {
        c.cycles += 24;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xc0));
        c.div_s(7, 1, 2); // d4 / dir.x
        c.lwc1(6, (c.g[A2][0] as u32).wrapping_add(0xd0));
        c.add_s(1, 0, 1); // c0 + c4
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0xd4));
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0xb4));
        c.neg_s(8, 6); // -d0
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xb8));
        c.add_s(3, 6, 3); // d0 + d4
        c.mul_s(4, 4, 0); // dir.y * c0
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 5);
        c.mul_s(2, 2, 1); // dir.z * (c0+c4)
        c.sqrt_s(0, 3);
        c.div_s(5, 5, 0); // 1 / sqrt(sum)
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc8));
        c.div_s(4, 4, 6); // *c0/d0
        c.div_s(2, 2, 3); // .../sum
        c.div_s(8, 8, 3); // -d0/sum
        c.sub_s(7, 7, 4);
        c.sub_s(1, 1, 2);
        c.mul_s(8, 8, 7);
        c.mul_s(5, 5, 1);
        c.cycles += 69;
    }
    // velocity = normalize(dir) * *(sub+0x38)
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x38));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.mfc1(4, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x48)); // speed
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mul_s(0, 5, 1); // a * speed -> w.y
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0); // w.x = 0
    c.mul_s(1, 8, 1); // b * speed -> w.z
    c.sq(2, c.g[SP][0] as u32);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
    // w = mat * w
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[V1][0] as u32);
    c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    // d = body->pos - origin; body->vel += vel + d x w; body->rot += w
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0xa0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lqc2(3, c.g[SP][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    abi::ret(c);
}

/// `void MultiSplineModifier_Release(MultiSplineModifier* m)` (0x35aae0):
/// decrement the reference count and store it back.
pub fn multi_spline_modifier_release(c: &mut Cpu) {
    c.cycles += 4;
    let m = abi::arg(c, 0);
    c.g[V0][0] = Addr(m.wrapping_add(0x34)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    let count = c.g[V0][0];
    Addr(m.wrapping_add(0x34)).sw(c, count);
    abi::ret(c);
}

/// `void MultiSplineModifier_AllocNodeInstances(MultiSplineModifier* self)`
/// (0x35a3f0): fill `self->instances[0 .. count)` with fresh instance-man
/// allocations.
pub fn multi_spline_modifier_alloc_node_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if !empty {
            loop {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0035_a418);
                call_ptr(c, 0x002d_1ce0, 0x0035_a418)?;
                c.g[RA][0] = sx(0x0035_a420);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0035_1170, 0x0035_a420)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a410);
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void MultiSplineModifier_v19(MultiSplineModifier* self, void* item)`
/// (0x35aaf0): unlink the spline object (when present) and notify the
/// current item when it is the modifier's own.
pub fn multi_spline_modifier_v19(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S0, 0x30)).lw(c);
        let no_spline = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_spline {
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x18)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x1c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_ab24)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(at(c, S0, 0x40)).lw(c);
        let same = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = sx(at(c, S1, 0x10));
        if same {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S1, 0xc)).lw(c);
            c.g[V1][0] = Addr(at(c, A1, 0xc)).lw(c);
            c.g[A0][0] = Addr(at(c, V1, 0xc0)).lh(c);
            c.g[V0][0] = Addr(at(c, V1, 0xc4)).lw(c);
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_ab48)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 f, off(reg)`.
#[inline]
pub(crate) fn lwc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.lwc1(f, a);
}

/// `swc1 f, off(reg)`.
#[inline]
pub(crate) fn swc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.swc1(f, a);
}

/// `func_0035A0xx` `SplineModifier_v05` (0x359eb8): the spline modifier's
/// event handler. Action 0x190 runs `SplineModifier_SetSpeed` and re-enables
/// the modifier; 0x191 does the same without the call; 0x192 negates the
/// speed at `+0x48`; anything else returns. `+0x50` is cleared and `+0x54` set
/// to 1 whenever it acts. Signature: `void handler(this, action, ...)`.
pub fn spline_modifier_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x190;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let not_first = c.g[A1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !not_first {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0035_9ed8);
            callee::SPLINE_MODIFIER_SET_SPEED.call(c, 0x0035_9ed8)?;
            c.g[V0][0] = 0x1;
            c.cycles += 3;
            sw_r(c, S0, 0x50, 0);
            sw_r(c, S0, 0x54, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x191;
            let not_second = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x192;
            if not_second {
                c.cycles += 2;
                if c.g[A1][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 6;
                    lwc1_r(c, 0, S0, 0x48);
                    c.g[V0][0] = 0x1;
                    sw_r(c, S0, 0x54, c.g[V0][0]);
                    c.f[0] = (-FpuF32(c.f[0])).0;
                    sw_r(c, S0, 0x50, 0);
                    swc1_r(c, 0, S0, 0x48);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                c.cycles += 3;
                sw_r(c, S0, 0x50, 0);
                sw_r(c, S0, 0x54, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MultiSplineModifier_v23(self, item)` (0x35ab60): whether `item` is the
/// current spline (and its end flag), or hand the item to the current spine's
/// virtual "add" (slot 0x10c) and return the modifier's count.
pub fn multi_spline_modifier_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(at_2(c, S0, 0x40));
        let is_current = c.g[A1][0] == c.g[V0][0];
        if is_current {
            c.cycles += 8;
            c.g[A1][0] = c.lw(at_2(c, A1, 0xc));
            c.g[V1][0] = c.lw(at_2(c, A1, 0xc));
            c.g[A0][0] = c.lh(at_2(c, V1, 0xd0));
            c.g[V0][0] = c.lw(at_2(c, V1, 0xd4));
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0035_ab94)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 9;
            c.g[V0][0] = c.lw(at_2(c, S0, 0x40));
            c.g[A1][0] = c.lw(at_2(c, V0, 0xc));
            c.g[V1][0] = c.lw(at_2(c, A1, 0xc));
            c.g[A0][0] = c.lh(at_2(c, V1, 0xd8));
            c.g[V0][0] = c.lw(at_2(c, V1, 0xdc));
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0035_abb8)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A3][0] = c.lw(at_2(c, V0, 0xc));
        c.cycles += 8;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.g[A2][0] = c.lw(at_2(c, S0, 0x38));
        c.g[V0][0] = c.lw(at_2(c, A3, 0xc));
        c.g[A0][0] = c.lh(at_2(c, V0, 0x108));
        c.g[V1][0] = c.lw(at_2(c, V0, 0x10c));
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        let target = r32(c, V1);
        call_ptr(c, target, 0x0035_abdc)?;
        c.g[V0][0] = c.lw(at_2(c, S0, 0x38));
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SplineModifier_ctor_2(self, path)` (0x3595d8): construct a spline
/// modifier for a path and load its spline.
pub fn spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdb0));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(c.g[S0][0] as u32, 0xd8), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw(p(c.g[S0][0] as u32, 0xe0), 0);
        c.g[A2][0] = 0x50;
        c.sw(p(c.g[S0][0] as u32, 0xdc), 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.sw(p(c.g[S0][0] as u32, 0xe4), 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_9638)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5570, 0x0035_9644)?;
        c.g[A1][0] = 0x500000;
        c.g[A2][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
        c.g[V0][0] = c.g[S0][0];
        c.lq(3, c.g[A1][0] as u32);
        frame.restore(c, S1, 0x20);
        c.sq(3, p(c.g[S0][0] as u32, 0xb0));
        frame.restore(c, S2, 0x10);
        c.lq(4, c.g[A1][0] as u32);
        c.sw(p(c.g[S0][0] as u32, 0x58), c.g[A2][0]);
        c.sq(4, p(c.g[S0][0] as u32, 0xc0));
        c.sw(p(c.g[S0][0] as u32, 0xd0), 0);
        c.sw(p(c.g[S0][0] as u32, 0xd4), 0);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spline modifier.

/// `MultiSplineModifier::~MultiSplineModifier(this, int flags)` (0x35a250):
/// remove the node instances, free the node/data/instance arrays, then free
/// the object when asked.
pub fn multisplinemodifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x490000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe98));
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0035_a780, 0x0035_a27c)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x38)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(p(r32(c, S1), 0x3c)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0035_a290)?;
            c.g[A0][0] = Addr(p(r32(c, S1), 0x3c)).lw(c);
        }
        c.cycles += 2;
        let have_data = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[S0][0] & 1;
        if have_data {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0035_a2a4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
        let mut none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !none {
            loop {
                c.cycles += 13;
                call_ptr(c, 0x002d_1ce0, 0x0035_a2b8)?;
                c.g[A1][0] = Addr(p(r32(c, S1), 0x44)).lw(c);
                c.g[V1][0] = sx(r32(c, S0) << 2);
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V1][0] = sx(p(r32(c, V1), r32(c, A1)));
                c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                call_ptr(c, 0x0035_1260, 0x0035_a2d4)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a2b0);
                }
            }
            none = true;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x44)).lw(c);
        let no_insts = c.g[A0][0] == 0;
        c.g[V0][0] = 0x490000;
        if !no_insts {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0035_a2f8)?;
            c.g[V0][0] = 0x490000;
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x13f8));
        let keep = c.g[S2][0] == 0;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0035_a310)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_UpdateNodes (0x35ac20, splinemodifier.cpp)

pub(crate) const MATH_SIN_COS: u32 = 0x0031_be50;

pub(crate) const MATH_ATAN: u32 = 0x0031_c228;

pub(crate) const SPLINE_EVAL_PATH_AT_DISTANCE: Callee = Callee {
    addr: 0x0034_5248,
    f: ee::s11::SplineModifier_EvalPathAtDistance,
};

/// MultiSplineModifier fields.
pub(crate) const MSM_COUNT: u32 = 0x04;

pub(crate) const MSM_MODE: u32 = 0x08;

pub(crate) const MSM_SPIN: u32 = 0x0c;

pub(crate) const MSM_START: u32 = 0x10;

pub(crate) const MSM_DIRTY: u32 = 0x30;

pub(crate) const MSM_STATES: u32 = 0x3c;

pub(crate) const MSM_NODES: u32 = 0x44;

pub(crate) const MSM_PATH: u32 = 0x48;

pub(crate) const MSM_LENGTH: u32 = 0x54;

/// Per-node state (0x60 bytes): position (0x00), direction (0x10),
/// third output (0x20), squared direction x/z (0x50/0x54).
pub(crate) const NS_DIR: u32 = 0x10;

pub(crate) const NS_DIR_SQ: u32 = 0x50;

pub(crate) const NS_STRIDE: u32 = 0x60;

/// The world up axis (a global vec3).
pub(crate) const UP_AXIS: u32 = 0x004f_f160;

/// gp-relative `.lit4`: pi (atan quadrant fix), the +-pi/2 yaw and pitch
/// values for a zero denominator.
pub(crate) const GP_PI: u32 = 0x2a24;

pub(crate) const GP_YAW_NEG: u32 = 0x2a20;

pub(crate) const GP_YAW_POS: u32 = 0x2a1c;

pub(crate) const GP_PITCH_NEG: u32 = 0x2a18;

pub(crate) const GP_PITCH_POS: u32 = 0x2a14;

/// Frame: per-node distances (0x000), spin matrix (0x400), node matrix
/// scratch (0x450, 0x490), angles (0x440..), sin/cos outputs (0x4d0..).
pub(crate) const MSM_F_SPIN: u32 = 0x400;

pub(crate) const MSM_F_TMP_A: u32 = 0x450;

pub(crate) const MSM_F_TMP_B: u32 = 0x490;

/// `void MultiSplineModifier_UpdateNodes(MultiSplineModifier* self)`
/// (0x35ac20): spreads the `count` nodes evenly over the path (`start +
/// i * length / count`, wrapped into `[0, length)`), evaluates the path at
/// each distance, derives the node's yaw (modes 2/3) and pitch (modes 1/3)
/// from the path direction and writes the node object's matrix (rows from
/// the angles' sines/cosines, translation = path position); a non-zero spin
/// angle additionally rotates every node about the world up axis. Clears
/// the dirty flag.
pub fn multi_spline_modifier_update_nodes(c: &mut Cpu) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x5d0);
        c.g[V0][0] = 1;
        for (reg, off) in [
            (S2, 0x560),
            (S0, 0x580),
            (S1, 0x570),
            (S3, 0x550),
            (S4, 0x540),
            (S5, 0x530),
            (S6, 0x520),
            (S7, 0x510),
            (FP, 0x500),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x4f0);
        for (f, off) in [
            (26, 0x5c0),
            (25, 0x5b8),
            (24, 0x5b0),
            (23, 0x5a8),
            (22, 0x5a0),
            (21, 0x598),
            (20, 0x590),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S2][0] = c.g[A0][0];
        let this = r32(c, S2);
        let at = |o: u32| this.wrapping_add(o);

        // dist[0] = start; step = length / count
        c.g[V1][0] = Addr(at(MSM_COUNT)).lw(c);
        c.lwc1(2, at(MSM_LENGTH));
        c.mtc1(V1, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, at(MSM_START));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.div_s(2, 2, 1);
        let a = frame.at(c, 0);
        c.swc1(0, a);
        if c.g[V0][0] != 0 {
            // dist[i] = dist[i - 1] + step
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
            c.g[V1][0] = sx(frame.at(c, 4));
            loop {
                c.cycles += 6;
                c.lwc1(0, r32(c, V1).wrapping_sub(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.add_s(0, 0, 2);
                c.swc1(0, r32(c, V1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_aca0);
                }
            }
        }

        // Wrap every distance into [0, length).
        c.cycles += 3;
        c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);
        c.g[A0][0] = sx(at(MSM_COUNT));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[V1][0] = c.g[V0][0];
            c.mtc1(0, 2); // f2 = 0
            c.g[V0][0] = c.g[SP][0];
            loop {
                c.cycles += 5;
                c.lwc1(1, r32(c, V0));
                c.c_lt_s(1, 2);
                let negative = c.fcc();
                c.lwc1(0, at(MSM_LENGTH));
                let store = if negative {
                    c.cycles += 2;
                    c.add_s(0, 1, 0);
                    true
                } else {
                    c.cycles += 4;
                    c.c_le_s(0, 1);
                    if c.fcc() {
                        c.cycles += 1;
                        c.sub_s(0, 1, 0);
                        true
                    } else {
                        false
                    }
                };
                if store {
                    c.cycles += 2;
                    c.swc1(0, r32(c, V0));
                }
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_acd0);
                }
            }
        }

        // The spin matrix: a rotation by `spin` about the up axis.
        c.cycles += 6;
        c.lwc1(0, at(MSM_SPIN));
        c.mtc1(0, 20); // f20 = 0
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.cycles += 55;
            c.lwc1(12, r32(c, A0).wrapping_add(8));
            c.g[A1][0] = sx(frame.at(c, 0x4d4));
            c.g[A0][0] = sx(frame.at(c, 0x4d0));
            c.g[S0][0] = 0x50_0000;
            call_ptr(c, MATH_SIN_COS, 0x0035_ad3c)?;
            msm_spin_matrix(c, frame);
        }
        c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);

        // Every node.
        c.cycles += 3;
        c.g[S4][0] = 0;
        c.g[S1][0] = Addr(at(MSM_STATES)).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.lwc1(25, r32(c, GP).wrapping_sub(GP_PI));
            c.mov_s(26, 20); // roll = 0
            c.g[S7][0] = sx(frame.at(c, MSM_F_TMP_B));
            c.g[FP][0] = sx(frame.at(c, MSM_F_SPIN));
            c.g[S6][0] = sx(frame.at(c, MSM_F_TMP_A));
            c.g[S5][0] = c.g[SP][0];
            loop {
                // EvalPathAtDistance(&path, dist[i], &st->pos, &st->dir, &st->c)
                c.cycles += 19;
                let st = r32(c, S1);
                c.lwc1(12, r32(c, S5));
                c.g[A0][0] = sx(at(MSM_PATH));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(st.wrapping_add(NS_DIR));
                c.mtc1(0, 23); // f23 = 0
                c.g[A3][0] = sx(st.wrapping_add(0x20));
                SPLINE_EVAL_PATH_AT_DISTANCE.call(c, 0x0035_ae44)?;
                c.g[S0][0] = sx(r32(c, S4) << 2);
                c.lwc1(21, st.wrapping_add(NS_DIR));
                c.lwc1(20, st.wrapping_add(NS_DIR + 4));
                c.mul_s(0, 21, 21);
                c.mul_s(1, 20, 20);
                c.swc1(0, st.wrapping_add(NS_DIR_SQ));
                c.swc1(1, st.wrapping_add(NS_DIR_SQ + 4));
                c.g[V0][0] = Addr(at(MSM_MODE)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(2));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                c.mov_s(24, 23); // pitch = 0
                if c.g[V0][0] != 0 {
                    // yaw = -atan2(dir.y, dir.x) (modes 2, 3)
                    msm_angle(c, 21, 20, 23, GP_YAW_NEG, GP_YAW_POS, 0x0035_aeb8, false)?;
                }

                c.cycles += 4;
                c.g[V1][0] = Addr(at(MSM_MODE)).lw(c);
                c.g[V0][0] = 1;
                let mut pitch = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if !pitch {
                    c.cycles += 2;
                    pitch = c.g[V1][0] == c.g[V0][0];
                }
                if pitch {
                    // pitch = atan2(dir.z, |dir.xy|) (modes 1, 3)
                    c.cycles += 9;
                    c.lwc1(0, st.wrapping_add(NS_DIR_SQ));
                    c.lwc1(1, st.wrapping_add(NS_DIR_SQ + 4));
                    c.mtc1(0, 22); // f22 = 0
                    c.add_s(0, 0, 1);
                    c.lwc1(21, st.wrapping_add(NS_DIR + 8));
                    c.sqrt_s(20, 0);
                    msm_angle(c, 20, 21, 22, GP_PITCH_NEG, GP_PITCH_POS, 0x0035_af54, true)?;
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr(at(MSM_NODES)).lw(c);

                // The node's matrix from (roll, pitch, yaw).
                c.cycles += 89;
                let a0 = frame.at(c, 0x4d8);
                c.g[A0][0] = sx(a0);
                let a = frame.at(c, 0x444);
                c.swc1(24, a);
                c.g[A1][0] = sx(frame.at(c, 0x4dc));
                let a = frame.at(c, 0x448);
                c.swc1(23, a);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                let a = frame.at(c, 0x440);
                c.swc1(26, a);
                c.mov_s(12, 26);
                c.g[S0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, MATH_SIN_COS, 0x0035_afa4)?;
                c.lwc1(12, frame.at(c, 0x444));
                c.g[A0][0] = sx(frame.at(c, 0x4e0));
                c.g[A1][0] = sx(frame.at(c, 0x4e4));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(0x10));
                call_ptr(c, MATH_SIN_COS, 0x0035_afb8)?;
                c.lwc1(12, frame.at(c, 0x448));
                c.g[A0][0] = sx(frame.at(c, 0x4e8));
                c.g[A1][0] = sx(frame.at(c, 0x4ec));
                call_ptr(c, MATH_SIN_COS, 0x0035_afc8)?;
                msm_node_matrix(c, frame);
                c.lwc1(0, at(MSM_SPIN));
                c.c_eq_s(0, 26);
                if !c.fcc() {
                    // node = node * spin (through two scratch copies)
                    c.cycles += 45;
                    let (m, spin) = (r32(c, S3), r32(c, FP));
                    for i in 0..4 {
                        c.lqc2(4 + i, m + 0x10 * i as u32);
                    }
                    for i in 0..4 {
                        c.lqc2(8 + i, spin + 0x10 * i as u32);
                    }
                    for i in 0..4 {
                        let row = 8 + i;
                        c.vu0().mula(0b1111, 4, Src::Bc(row, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
                        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(row, W));
                    }
                    let (b, a) = (r32(c, S7), r32(c, S6));
                    for i in 0..4 {
                        c.sqc2(12 + i, b + 0x10 * i as u32);
                    }
                    for (from, to) in [(b, a), (a, m)] {
                        for i in 0..4 {
                            c.lqc2(1 + i, from + 0x10 * i as u32);
                        }
                        for i in 0..4 {
                            c.sqc2(1 + i, to + 0x10 * i as u32);
                        }
                    }
                }
                c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);

                c.cycles += 5;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(NS_STRIDE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_ae28);
                }
            }
        }

        c.cycles += 20;
        Addr(at(MSM_DIRTY)).sw(c, 0);
        for (reg, off) in [
            (S0, 0x580),
            (S1, 0x570),
            (S2, 0x560),
            (S3, 0x550),
            (S4, 0x540),
            (S5, 0x530),
            (S6, 0x520),
            (S7, 0x510),
            (FP, 0x500),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x4f0);
        for (f, off) in [
            (26, 0x5c0),
            (25, 0x5b8),
            (24, 0x5b0),
            (23, 0x5a8),
            (22, 0x5a0),
            (21, 0x598),
            (20, 0x590),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// One angle of a node: `atan(num / den)` fixed up by +-pi into the right
/// quadrant, or +-pi/2 (by the sign of `num`) when `den` is 0, 0 when both
/// are. `zero` holds 0.0. The yaw form (`pitch == false`, den = dir.x in
/// f21, num = dir.y in f20) leaves `f23 = f24 - angle`; the pitch form (den
/// = |dir.xy| in f20, num = dir.z in f21) leaves `f24 += angle`. The
/// pitch form's final block cycle is charged by the caller.
#[allow(clippy::too_many_arguments)]
pub(crate) fn msm_angle(
    c: &mut Cpu,
    den: usize,
    num: usize,
    zero: usize,
    gp_neg: u32,
    gp_pos: u32,
    ret_addr: u32,
    pitch: bool,
) -> crate::call::Flow {
    // The angle accumulates into f23 (yaw) or f24 (pitch).
    let apply = |c: &mut Cpu| {
        if pitch {
            c.add_s(24, 24, 0);
        } else {
            c.sub_s(23, 24, 0);
        }
    };
    if !pitch {
        c.cycles += 3;
    }
    c.c_eq_s(den, zero);
    if c.fcc() {
        // A zero denominator.
        c.cycles += 4;
        c.c_eq_s(num, zero);
        let both_zero = c.fcc();
        c.mov_s(0, num);
        if both_zero {
            c.cycles += 1;
            apply(c);
            return Ok(());
        }
        c.cycles += 4;
        c.c_le_s(zero, num);
        c.lwc1(0, r32(c, GP).wrapping_sub(gp_neg));
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, r32(c, GP).wrapping_sub(gp_pos));
        }
        apply(c);
        return Ok(());
    }
    c.cycles += 7;
    c.div_s(12, num, den);
    call_ptr(c, MATH_ATAN, ret_addr)?;
    c.c_lt_s(den, zero);
    if !c.fcc() {
        apply(c);
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(zero, num);
    if c.fcc() {
        c.cycles += 1;
        c.add_s(0, 0, 25);
    } else {
        c.sub_s(0, 0, 25);
    }
    c.cycles += 1;
    apply(c);
    Ok(())
}

/// The spin rotation about the up axis from `sin`/`cos` (frame 0x4d0/0x4d4)
/// into the frame's 4x4 matrix at 0x400 (bottom row 0, 0, 0, 1). Verbatim
/// register order of the original (f0..f13).
pub(crate) fn msm_spin_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(11, s(0x4d4));
    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(3, UP_AXIS + 8);
    c.sub_s(1, 5, 11);
    c.lwc1(0, UP_AXIS);
    c.lwc1(6, UP_AXIS + 4);
    c.lwc1(8, s(0x4d0));
    c.mul_s(2, 1, 3);
    c.swc1(5, s(0x43c));
    c.mul_s(4, 1, 0);
    c.swc1(20, s(0x430));
    c.mul_s(1, 1, 6);
    c.swc1(20, s(0x434));
    c.mul_s(13, 8, 3);
    c.swc1(20, s(0x438));
    c.mul_s(12, 8, 0);
    c.swc1(20, s(0x40c));
    c.mul_s(9, 2, 3);
    c.swc1(20, s(0x41c));
    c.mul_s(10, 4, 3);
    c.swc1(20, s(0x42c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, s(0x408));
    c.add_s(7, 7, 11);
    c.swc1(0, s(0x404));
    c.add_s(4, 4, 13);
    c.swc1(2, s(0x418));
    c.sub_s(10, 10, 8);
    c.swc1(9, s(0x428));
    c.add_s(1, 1, 11);
    c.swc1(7, s(0x400));
    c.add_s(3, 3, 12);
    c.swc1(4, s(0x410));
    c.swc1(10, s(0x420));
    c.swc1(1, s(0x414));
    c.swc1(3, s(0x424));
}

/// The node object's matrix (s0 + 0x10): rotation rows from the sines and
/// cosines of roll (0x4d8/0x4dc), pitch (0x4e0/0x4e4) and yaw (0x4e8/0x4ec),
/// translation = the node state's position (s1), w = 1. Verbatim register
/// order of the original.
pub(crate) fn msm_node_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    let n = r32(c, S0);
    let m = |o: u32| n.wrapping_add(o);
    let st = r32(c, S1);
    c.lwc1(0, s(0x4ec));
    c.lwc1(1, s(0x4e4));
    c.lwc1(2, s(0x4e0));
    c.mul_s(1, 1, 0);
    c.lwc1(3, s(0x4d8));
    c.lwc1(0, s(0x4e8));
    c.lwc1(4, s(0x4dc));
    c.mul_s(3, 3, 2);
    c.swc1(1, m(0x10));
    c.mul_s(4, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(0, s(0x4e4));
    c.lwc1(1, s(0x4e8));
    c.neg_s(0, 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x14));
    c.lwc1(1, s(0x4e0));
    c.sw(m(0x1c), 0);
    c.swc1(1, m(0x18));
    c.lwc1(0, s(0x4ec));
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.swc1(0, m(0x20));
    c.lwc1(0, s(0x4ec));
    c.lwc1(2, s(0x4e8));
    c.lwc1(1, s(0x4dc));
    c.mul_s(3, 3, 2);
    c.mul_s(1, 1, 0);
    c.sub_s(1, 1, 3);
    c.swc1(1, m(0x24));
    c.lwc1(0, s(0x4d8));
    c.lwc1(1, s(0x4e4));
    c.neg_s(0, 0);
    c.sw(m(0x2c), 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x28));
    c.lwc1(2, s(0x4e0));
    c.lwc1(1, s(0x4dc));
    c.lwc1(0, s(0x4d8));
    c.mul_s(1, 1, 2);
    c.lwc1(3, s(0x4e8));
    c.lwc1(2, s(0x4ec));
    c.mul_s(0, 0, 3);
    c.mul_s(1, 1, 2);
    c.sub_s(0, 0, 1);
    c.swc1(0, m(0x30));
    c.lwc1(2, s(0x4e0));
    c.lwc1(1, s(0x4ec));
    c.lwc1(0, s(0x4d8));
    c.mul_s(4, 4, 2);
    c.mul_s(0, 0, 1);
    c.add_s(4, 4, 0);
    c.swc1(4, m(0x34));
    c.lwc1(1, s(0x4e4));
    c.lwc1(0, s(0x4dc));
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x38));
    c.sw(m(0x3c), 0);
    c.lwc1(0, st);
    c.swc1(0, m(0x40));
    c.lwc1(1, st.wrapping_add(4));
    c.swc1(1, m(0x44));
    c.lwc1(2, st.wrapping_add(8));
    c.swc1(5, m(0x4c));
    c.swc1(2, m(0x48));
}

// ---------------------------------------------------------------------------
// SplineModifier_UpdateTransform (0x359830, splinemodifier.cpp)

/// SplineModifier fields.
pub(crate) const SM_MODE: u32 = 0x34;

pub(crate) const SM_SPIN: u32 = 0x38;

pub(crate) const SM_DIST: u32 = 0x3c;

pub(crate) const SM_DIRTY: u32 = 0x58;

/// The driven object's 4x4 matrix (rows 0x60..0x9c).
pub(crate) const SM_MATRIX: u32 = 0x60;

pub(crate) const SM_POS: u32 = 0xa0;

pub(crate) const SM_DIR: u32 = 0xb0;

pub(crate) const SM_ACC: u32 = 0xc0;

/// dir.x^2, dir.y^2.
pub(crate) const SM_DIR_SQ: u32 = 0xd0;

pub(crate) const SM_PATH: u32 = 0xd8;

pub(crate) const SM_EVAL_PATH_AT_DISTANCE: Callee = Callee {
    addr: 0x0034_5248,
    f: ee::s11::SplineModifier_EvalPathAtDistance,
};

pub(crate) const SM_MATH_SIN_COS: u32 = 0x0031_be50;

pub(crate) const SM_MATH_ATAN: u32 = 0x0031_c228;

/// The world up axis (vec3) the spin rotates about.
pub(crate) const SM_UP_AXIS: u32 = 0x004f_f160;

/// One angle of the transform: `atan(y / x)` (f21 / f20) fixed into the
/// right quadrant with `+lits[2]` / `-lits[3]` (pi) when `x < 0`, or
/// `lits[0]` / `lits[1]` (-pi/2, +pi/2 by the sign of `y`) when `x == 0`,
/// or `y` itself (0) when both are 0; `zero` holds 0.0. Leaves the angle in
/// f0 and applies it: yaw (`pitch == false`) `f25 = f24 - angle`, pitch
/// `f24 += angle`. The caller charged the block that made the `x == 0`
/// compare.
pub(crate) fn sm_angle(
    c: &mut Cpu,
    zero: usize,
    lits: [u32; 4],
    ret_addr: u32,
    pitch: bool,
) -> crate::call::Flow {
    let apply = |c: &mut Cpu| {
        if pitch {
            c.add_s(24, 24, 0);
        } else {
            c.sub_s(25, 24, 0);
        }
    };
    let gp = r32(c, GP);
    if c.fcc() {
        // x == 0
        c.cycles += 4;
        c.c_eq_s(21, zero);
        let y_zero = c.fcc();
        c.mov_s(0, 21);
        if y_zero {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.c_le_s(zero, 21);
            c.lwc1(0, gp.wrapping_sub(lits[0]));
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp.wrapping_sub(lits[1]));
            }
        }
        apply(c);
        return Ok(());
    }
    c.cycles += 7;
    c.div_s(12, 21, 20);
    call_ptr(c, SM_MATH_ATAN, ret_addr)?;
    c.c_lt_s(20, zero);
    let x_neg = c.fcc();
    c.mov_s(12, 0);
    if x_neg {
        c.cycles += 3;
        c.c_lt_s(zero, 21);
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(lits[2]));
            c.add_s(12, 12, 0);
        } else {
            c.cycles += 2;
            c.lwc1(0, gp.wrapping_sub(lits[3]));
            c.sub_s(12, 12, 0);
        }
    }
    c.cycles += 2;
    c.mov_s(0, 12);
    apply(c);
    Ok(())
}

/// `void SplineModifier_UpdateTransform(SplineModifier* self)` (0x359830):
/// evaluates the path at the modifier's distance (position, direction,
/// third output), stores the squared direction x/y, derives the yaw
/// (`-atan2(dir.y, dir.x)` of the normalised direction, modes 2/3) and the
/// pitch (`atan2(dir.z, |dir.xy|)`, modes 1/3), writes the object matrix
/// from the sines/cosines of (roll 0, pitch, yaw) with the path position as
/// translation, and with a non-zero spin angle multiplies it by a rotation
/// about the world up axis. Clears the dirty flag. Register order of the
/// original.
pub fn spline_modifier_update_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S1, 0x130);
        frame.save_fpr(c, 25, 0x178);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S1);
        let at = |o: u32| this.wrapping_add(o);
        frame.save_fpr(c, 24, 0x170);
        frame.save_fpr(c, 23, 0x168);
        frame.save(c, S0, 0x140);
        frame.save(c, S2, 0x120);
        frame.save_ra(c, 0x110);
        frame.save_fpr(c, 22, 0x160);
        frame.save_fpr(c, 21, 0x158);
        frame.save_fpr(c, 20, 0x150);
        c.g[A0][0] = sx(at(SM_PATH));
        c.g[A1][0] = sx(at(SM_POS));
        c.g[A2][0] = sx(at(SM_DIR));
        c.g[A3][0] = sx(at(SM_ACC));
        c.mtc1(0, 23); // f23 = 0
        c.lwc1(12, at(SM_DIST));
        c.mov_s(24, 23); // pitch = 0
        c.mov_s(25, 23); // yaw = 0
        SM_EVAL_PATH_AT_DISTANCE.call(c, 0x0035_9884)?;

        // Squared direction x/y; the normalised direction to sp+0x10 and sp.
        c.lwc1(0, at(SM_DIR));
        c.lwc1(1, at(SM_DIR + 4));
        c.mul_s(0, 0, 0);
        c.mul_s(1, 1, 1);
        c.swc1(0, at(SM_DIR_SQ));
        c.swc1(1, at(SM_DIR_SQ + 4));
        c.lqc2(3, at(SM_DIR));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        let a = frame.at(c, 0x10);
        c.sqc2(5, a);
        c.g[V0][0] = Addr(at(SM_MODE)).lw(c);
        frame.restore(c, V1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(2));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        frame.save(c, V1, 0);
        if c.g[V0][0] != 0 {
            // yaw = -atan2(n.y, n.x) (modes 2, 3)
            c.cycles += 5;
            frame.restore_fpr(c, 20, 0);
            c.c_eq_s(20, 23);
            frame.restore_fpr(c, 21, 4);
            sm_angle(c, 23, [0x2a50, 0x2a4c, 0x2a48, 0x2a44], 0x0035_9928, false)?;
        }

        c.cycles += 4;
        c.g[V1][0] = Addr(at(SM_MODE)).lw(c);
        c.g[V0][0] = 1;
        let mut pitch = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if !pitch {
            c.cycles += 2;
            pitch = c.g[V1][0] == c.g[V0][0];
        }
        if pitch {
            // pitch = atan2(dir.z, |dir.xy|) (modes 1, 3)
            c.cycles += 9;
            c.lwc1(0, at(SM_DIR_SQ));
            c.lwc1(1, at(SM_DIR_SQ + 4));
            c.mtc1(0, 22); // f22 = 0
            c.add_s(0, 0, 1);
            c.lwc1(21, at(SM_DIR + 8));
            c.sqrt_s(20, 0);
            c.c_eq_s(20, 22);
            sm_angle(c, 22, [0x2a40, 0x2a3c, 0x2a38, 0x2a34], 0x0035_99d4, true)?;
            c.cycles += 1;
        }
        c.lwc1(2, at(SM_POS));

        // sin/cos of roll (0), pitch and yaw.
        c.cycles += 73;
        c.g[A0][0] = sx(frame.at(c, 0xf0));
        c.lwc1(1, at(SM_POS + 4));
        c.g[A1][0] = sx(frame.at(c, 0xf4));
        c.lwc1(0, at(SM_POS + 8));
        c.mov_s(12, 23);
        frame.save_fpr(c, 2, 0x20);
        c.g[S2][0] = sx(at(SM_MATRIX));
        frame.save_fpr(c, 1, 0x24);
        frame.save_fpr(c, 0, 0x28);
        frame.save_fpr(c, 24, 0x14);
        frame.save_fpr(c, 25, 0x18);
        frame.save_fpr(c, 23, 0x10);
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a44)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 21); // f21 = 1.0
        frame.restore_fpr(c, 12, 0x14);
        c.g[A0][0] = sx(frame.at(c, 0xf8));
        c.g[A1][0] = sx(frame.at(c, 0xfc));
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a5c)?;
        frame.restore_fpr(c, 12, 0x18);
        c.g[A0][0] = sx(frame.at(c, 0x100));
        c.g[A1][0] = sx(frame.at(c, 0x104));
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a6c)?;
        sm_object_matrix(c, frame);
        if !c.fcc() {
            // A spin angle: matrix = matrix * rotation(spin) about up.
            c.cycles += 99;
            c.lwc1(12, at(SM_SPIN));
            c.g[A0][0] = sx(frame.at(c, 0x108));
            c.g[A1][0] = sx(frame.at(c, 0x10c));
            c.g[S0][0] = 0x50_0000;
            call_ptr(c, SM_MATH_SIN_COS, 0x0035_9b48)?;
            sm_spin_matrix(c, frame);
            let (m, spin) = (r32(c, S2), frame.at(c, 0x30));
            for i in 0..4u8 {
                c.lqc2(4 + i as usize, m + 0x10 * i as u32);
            }
            for i in 0..4u8 {
                c.lqc2(8 + i as usize, spin + 0x10 * i as u32);
            }
            for i in 0..4u8 {
                let row = 8 + i;
                c.vu0().mula(0b1111, 4, Src::Bc(row, X));
                c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
                c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(row, W));
            }
            let (b, a) = (r32(c, V1), r32(c, A0));
            for i in 0..4u8 {
                c.sqc2(12 + i as usize, b + 0x10 * i as u32);
            }
            for (from, to) in [(b, a), (a, m)] {
                for i in 0..4u8 {
                    c.lqc2(1 + i as usize, from + 0x10 * i as u32);
                }
                for i in 0..4u8 {
                    c.sqc2(1 + i as usize, to + 0x10 * i as u32);
                }
            }
        }

        c.cycles += 13;
        Addr(at(SM_DIRTY)).sw(c, 0);
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore_ra(c, 0x110);
        for (f, off) in [
            (25, 0x178),
            (24, 0x170),
            (23, 0x168),
            (22, 0x160),
            (21, 0x158),
            (20, 0x150),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// The object matrix (s1 + 0x60) from the sines/cosines of roll (sp+0xf0/
/// 0xf4), pitch (0xf8/0xfc) and yaw (0x100/0x104), translation = the path
/// position (sp+0x20), w = 1.0 (f21); then compares the spin angle with 0
/// (the condition bit decides the spin rotation). Verbatim register order.
pub(crate) fn sm_object_matrix(c: &mut Cpu, frame: Frame) {
    let s1 = r32(c, S1);
    let m = |o: u32| s1.wrapping_add(o);
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(0, s(0xf0));
    c.g[V0][0] = sx(m(0x10));
    c.lwc1(9, s(0xf8));
    c.lwc1(6, s(0xf4));
    c.neg_s(14, 0);
    c.lwc1(10, s(0x100));
    c.mul_s(7, 0, 9);
    c.lwc1(8, s(0x104));
    c.mul_s(2, 6, 9);
    c.mul_s(17, 6, 10);
    c.lwc1(3, s(0xfc));
    c.mul_s(15, 0, 8);
    c.sw(m(0x6c), 0);
    c.mul_s(16, 7, 10);
    c.lwc1(18, m(SM_SPIN));
    c.neg_s(5, 3);
    c.lwc1(20, m(0x6c));
    c.mul_s(1, 6, 8);
    c.lwc1(11, s(0x20));
    c.mul_s(4, 17, 9);
    c.lwc1(12, s(0x24));
    c.mul_s(0, 0, 10);
    c.lwc1(13, s(0x28));
    c.mul_s(2, 2, 8);
    c.swc1(11, m(0x90));
    c.mul_s(7, 7, 8);
    c.swc1(12, m(0x94));
    c.mul_s(6, 6, 3);
    c.swc1(13, m(0x98));
    c.mul_s(14, 14, 3);
    c.swc1(9, m(0x68));
    c.sub_s(0, 0, 2);
    c.sw(m(0x7c), 0);
    c.add_s(4, 4, 15);
    c.swc1(6, m(0x88));
    c.sub_s(1, 1, 16);
    c.swc1(14, m(0x78));
    c.mul_s(5, 5, 10);
    c.swc1(0, m(0x80));
    c.add_s(7, 7, 17);
    c.swc1(4, m(0x84));
    c.mul_s(3, 3, 8);
    c.swc1(1, m(0x74));
    c.c_eq_s(18, 20);
    c.swc1(5, m(0x64));
    c.swc1(7, m(0x70));
    c.swc1(3, m(0x60));
    c.sw(m(0x8c), 0);
    c.swc1(21, m(0x9c));
}

/// The spin rotation about the world up axis from sin (sp+0x108) and cos
/// (sp+0x10c) into the frame's 4x4 matrix at sp+0x30 (bottom row 0, 0, 0,
/// 1); leaves v1 = sp+0xb0, a0 = sp+0x70, a1 = sp+0x30. Verbatim register
/// order.
pub(crate) fn sm_spin_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(11, s(0x10c));
    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    c.lwc1(4, SM_UP_AXIS + 8);
    c.g[A0][0] = sx(s(0x70));
    c.sub_s(1, 21, 11);
    c.lwc1(0, SM_UP_AXIS);
    c.lwc1(6, SM_UP_AXIS + 4);
    c.g[V1][0] = sx(s(0xb0));
    c.lwc1(8, s(0x108));
    c.g[A1][0] = sx(s(0x30));
    c.mul_s(2, 1, 4);
    c.swc1(20, s(0x5c));
    c.mul_s(3, 1, 0);
    c.swc1(21, s(0x6c));
    c.mul_s(1, 1, 6);
    c.swc1(20, s(0x60));
    c.mul_s(13, 8, 4);
    c.swc1(20, s(0x64));
    c.mul_s(12, 8, 0);
    c.swc1(20, s(0x68));
    c.mul_s(9, 2, 4);
    c.swc1(20, s(0x3c));
    c.mul_s(10, 3, 4);
    c.swc1(20, s(0x4c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(4, 1, 4);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, s(0x38));
    c.add_s(7, 7, 11);
    c.swc1(0, s(0x34));
    c.add_s(3, 3, 13);
    c.swc1(2, s(0x48));
    c.sub_s(10, 10, 8);
    c.swc1(9, s(0x58));
    c.add_s(1, 1, 11);
    c.swc1(7, s(0x30));
    c.add_s(4, 4, 12);
    c.swc1(3, s(0x40));
    c.swc1(10, s(0x50));
    c.swc1(1, s(0x44));
    c.swc1(4, s(0x54));
}

pub(crate) mod spline_mod {

    /// End behaviour: 0 stop, 2 bounce (ping-pong), 4 stop, else wrap.
    pub const MODE: u32 = 0x30;

    /// Distance along the spline.
    pub const POS: u32 = 0x3c;

    /// Acceleration while the acceleration time runs.
    pub const ACCEL: u32 = 0x40;

    /// Remaining acceleration time (seconds).
    pub const ACCEL_TIME: u32 = 0x44;

    /// Speed (distance per second).
    pub const SPEED: u32 = 0x48;

    /// Stopped flag.
    pub const STOPPED: u32 = 0x50;

    /// Active flag.
    pub const ACTIVE: u32 = 0x54;

    /// Transform needs an update.
    pub const DIRTY: u32 = 0x58;

    /// The spline's length.
    pub const LENGTH: u32 = 0xe4;

    /// `gp`-relative literals: the 0.1 margin below the end (modes 0, 2, 4).
    pub const LIT_END_MARGIN_STOP: u32 = 0x2a5c;

    pub const LIT_END_MARGIN_BOUNCE: u32 = 0x2a58;

    pub const LIT_END_MARGIN_STOP4: u32 = 0x2a54;

    /// `World_GetSecondsPerTick()` (f0, cross-shard).
    pub const SECONDS_PER_TICK: u32 = 0x002d_1c70;
}

// ---------------------------------------------------------------------------
// SpatialHash_Remove (0x3284b8)

/// Spatial hash (octree) node: eight child pointers (+0..+0x1c), then the
/// heads of the item lists (+0x20 + 4 * slot, three slots).
pub(crate) const SH_NODE_ITEMS: u32 = 0x20;

pub(crate) const SH_ITEM_SLOTS: i64 = 3;

/// Item link: next (+0), previous (+4).
pub(crate) const SH_LINK_NEXT: u32 = 0x0;

pub(crate) const SH_LINK_PREV: u32 = 0x4;

/// Keys are `{level, x, y, z}` words.
pub(crate) const SH_KEY_X: u32 = 0x4;

pub(crate) const SH_KEY_Y: u32 = 0x8;

pub(crate) const SH_KEY_Z: u32 = 0xc;

/// Resume points of the emptiness scan (event checks at its back edges).
pub(crate) const SH_REMOVE_ENTRY: u32 = 0x0032_84b8;

pub(crate) const SH_REMOVE_NOT_EMPTY: u32 = 0x0032_8614;

pub(crate) const SH_REMOVE_SCAN_ITEMS: u32 = 0x0032_8620;

pub(crate) const SH_REMOVE_END: u32 = 0x0032_8660;

/// `SpatialHash_Remove` itself (the recursion) and `__builtin_delete`.
pub(crate) const SPATIAL_HASH_REMOVE: Callee = Callee {
    addr: SH_REMOVE_ENTRY,
    f: ee::s10::SpatialHash_Remove,
};

pub(crate) const SH_BUILTIN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::f_00317e50,
};

/// `void SpatialHash_Remove(Node *node /*a0*/, int slot /*a1*/, Link *link
/// /*a2*/, Key *item /*a3*/, Key *at /*t0*/)` (0x3284b8): removes an item's
/// link from the octree. At the item's level the link is unlinked from its
/// list (the node's list head for `slot` when it was first); above it the
/// function recurses into the child containing the item (child key on the
/// stack: one level down, coordinates doubled plus the item's bit), and
/// deletes that child once it has no children and no items left. Frame 0x30
/// (child key at 0, ra at 0x10, s0 at 0x20). Dispatcher-shaped: the scan's
/// event checks may unwind, and it is re-entered at 0x328614 / 0x328620.
pub fn spatial_hash_remove(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| match entry {
        SH_REMOVE_ENTRY => spatial_hash_remove_body(c),
        SH_REMOVE_NOT_EMPTY | SH_REMOVE_SCAN_ITEMS => spatial_hash_remove_tail(c, entry),
        pc => {
            c.jump_out(pc, SH_REMOVE_ENTRY, SH_REMOVE_END);
            Ok(())
        }
    })
}

pub(crate) fn spatial_hash_remove_body(c: &mut Cpu) -> Flow {
    let frame = Frame { size: 0x30 };
    c.cycles += 11;
    Frame::open(c, 0x30);
    c.g[T3][0] = c.g[A3][0];
    frame.save(c, S0, 0x20);
    c.g[V1][0] = c.g[T0][0];
    frame.save_ra(c, 0x10);
    c.g[T4][0] = c.g[A0][0];
    c.g[T5][0] = c.g[A1][0];
    let item_level = Addr(r32(c, T3)).lw(c);
    c.g[V0][0] = item_level;
    let level = Addr(r32(c, V1)).lw(c);
    c.g[T1][0] = level;
    c.g[T2][0] = c.g[A2][0];
    let link = Addr(r32(c, T2));

    if item_level == level {
        // At the item's level: unlink it.
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, T5) << 2);
        let next = link.off(SH_LINK_NEXT).lw(c);
        c.g[V1][0] = next;
        let head = sx(r32(c, V0)
            .wrapping_add(r32(c, T4))
            .wrapping_add(SH_NODE_ITEMS));
        c.g[A0][0] = head;
        if next != 0 {
            c.cycles += 2;
            let prev = link.off(SH_LINK_PREV).lw(c);
            c.g[V0][0] = prev;
            Addr(next as u32).off(SH_LINK_PREV).sw(c, prev);
        }
        c.cycles += 3;
        let prev = link.off(SH_LINK_PREV).lw(c);
        c.g[V1][0] = prev;
        let next = link.off(SH_LINK_NEXT).lw(c);
        c.g[V0][0] = next;
        c.cycles += 2;
        if prev != 0 {
            Addr(prev as u32).off(SH_LINK_NEXT).sw(c, next);
        } else {
            Addr(head as u32).sw(c, next);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        spatial_hash_remove_epilogue(c, frame);
        return Ok(());
    }

    // Above it: the child key (one level down) and the child slot.
    c.cycles += 41;
    let at = Addr(r32(c, V1));
    let item = Addr(r32(c, T3));
    let shift = (level as u32)
        .wrapping_sub(item_level as u32)
        .wrapping_sub(1);
    c.g[A0][0] = sx(shift);
    let sh = shift & 31;
    let x2 = sx((at.off(SH_KEY_X).lw(c) as u32) << 1);
    let z2 = sx((at.off(SH_KEY_Z).lw(c) as u32) << 1);
    let y2 = sx((at.off(SH_KEY_Y).lw(c) as u32) << 1);
    let ix = sx(((item.off(SH_KEY_X).lw(c) as u32) as i32 >> sh) as u32);
    let iy = sx(((item.off(SH_KEY_Y).lw(c) as u32) as i32 >> sh) as u32);
    let iz = sx(((item.off(SH_KEY_Z).lw(c) as u32) as i32 >> sh) as u32);
    let bx = sx((ix as u32).wrapping_sub(x2 as u32));
    let by = sx((iy as u32).wrapping_sub(y2 as u32));
    let bz = sx((iz as u32).wrapping_sub(z2 as u32));
    let cy = sx((y2 as u32).wrapping_add(by as u32));
    let cx = sx((x2 as u32).wrapping_add(bx as u32));
    let cz = sx((z2 as u32).wrapping_add(bz as u32));
    let off_y = sx((by as u32) << 3);
    let off_z = sx((bz as u32) << 2);
    let slot = sx(r32(c, T4)
        .wrapping_add((bx as u32) << 4)
        .wrapping_add(off_y as u32));
    let child_level = sx((level as u32).wrapping_sub(1));
    c.g[T1][0] = child_level;
    let key = Addr(sp(c));
    key.off(SH_KEY_X).sw(c, cx);
    key.off(SH_KEY_Y).sw(c, cy);
    key.off(SH_KEY_Z).sw(c, cz);
    let child_slot = sx((slot as u32).wrapping_add(off_z as u32));
    c.g[S0][0] = child_slot;
    key.sw(c, child_level);
    c.g[V0][0] = off_y;
    c.g[V1][0] = off_z;
    c.g[A1][0] = c.g[T5][0];
    c.g[A2][0] = c.g[T2][0];
    c.g[A3][0] = c.g[T3][0];
    c.g[A0][0] = Addr(child_slot as u32).lw(c);
    c.g[T0][0] = c.g[SP][0];
    SPATIAL_HASH_REMOVE.call(c, 0x0032_85b0)?;

    // Is the child empty now? First its eight child pointers.
    let child = Addr(r32(c, S0)).lw(c);
    c.g[V1][0] = child;
    for i in 0..8u32 {
        if i > 0 {
            c.cycles += 3;
        }
        let v = Addr(child as u32).off(4 * i).lw(c);
        c.g[V0][0] = v;
        if v != 0 {
            if i == 7 {
                return spatial_hash_remove_tail(c, SH_REMOVE_NOT_EMPTY);
            }
            c.g[V0][0] = 0;
            return spatial_hash_remove_tail(c, 0x0032_863c);
        }
        if i < 7 {
            c.g[V0][0] = 0;
        }
    }
    c.g[V1][0] = sx((child as u32).wrapping_add(SH_NODE_ITEMS));
    c.cycles += 1;
    c.g[A0][0] = 0;
    spatial_hash_remove_tail(c, SH_REMOVE_SCAN_ITEMS)
}

/// The end of the emptiness scan from `at`: the item slots (from
/// 0x328620, v1 = the slot, a0 = slots seen), "not empty" (0x328614) or the
/// decision (0x32863c, v0 = empty): delete the child and clear its pointer.
pub(crate) fn spatial_hash_remove_tail(c: &mut Cpu, mut at: u32) -> Flow {
    let frame = Frame { size: 0x30 };
    loop {
        match at {
            SH_REMOVE_NOT_EMPTY => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                at = 0x0032_863c;
            }
            SH_REMOVE_SCAN_ITEMS => {
                c.cycles += 3;
                let v = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = v;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if v != 0 {
                    if back_edge_unwinds(c, SH_REMOVE_NOT_EMPTY) {
                        return Err(crate::call::Unwound);
                    }
                    at = SH_REMOVE_NOT_EMPTY;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < SH_ITEM_SLOTS) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, SH_REMOVE_SCAN_ITEMS) {
                        return Err(crate::call::Unwound);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                at = 0x0032_863c;
            }
            _ => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    SH_BUILTIN_DELETE.call(c, 0x0032_864c)?;
                    Addr(r32(c, S0)).sw(c, 0);
                    c.cycles += 1;
                }
                frame.restore(c, S0, 0x20);
                spatial_hash_remove_epilogue(c, frame);
                return Ok(());
            }
        }
    }
}

pub(crate) fn spatial_hash_remove_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `void SplineModifier_Update(SplineModifier* self)` (0x359698): spend
/// one tick of the acceleration time (speeding up by `accel * dt`); then,
/// if active and not stopped, advance the position by `speed * dt` and
/// handle running off either end by the mode (stop and reverse, bounce,
/// or wrap around), and mark the transform dirty.
pub fn spline_modifier_update(c: &mut Cpu) {
    use spline_mod::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        let gp = r32(c, GP);
        c.mtc1(0, 20); // f20 = 0.0
        c.lwc1(0, this + ACCEL_TIME);
        c.c_lt_s(20, 0);
        if c.fcc() {
            // Accelerating: accel_time -= dt; speed += accel * dt.
            c.cycles += 12;
            call_ptr(c, SECONDS_PER_TICK, 0x0035_96cc)?;
            c.lwc1(1, this + ACCEL_TIME);
            c.sub_s(1, 1, 0);
            c.swc1(1, this + ACCEL_TIME);
            call_ptr(c, SECONDS_PER_TICK, 0x0035_96dc)?;
            c.lwc1(1, this + ACCEL);
            c.lwc1(2, this + SPEED);
            c.mul_s(1, 1, 0);
            c.add_s(2, 2, 1);
            c.swc1(2, this + SPEED);
        }
        c.g[V0][0] = c.lw(this + ACTIVE);
        'out: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(this + STOPPED);
            if c.g[V0][0] != 0 {
                break 'out;
            }
            // pos += speed * dt (f3 speed, f2 the new position).
            c.cycles += 10;
            call_ptr(c, SECONDS_PER_TICK, 0x0035_9710)?;
            c.lwc1(3, this + SPEED);
            c.lwc1(1, this + POS);
            c.mul_s(0, 3, 0);
            c.add_s(2, 1, 0);
            c.c_lt_s(2, 20);
            let before_start = c.fcc();
            c.swc1(2, this + POS);
            // `stop`: stop at the start, reversed; `store_pos`: f0 holds the
            // position to store.
            let mut stop = false;
            let mut store_pos = false;
            if before_start {
                c.cycles += 3;
                c.g[V1][0] = c.lw(this + MODE);
                c.g[V0][0] = 2;
                if c.g[V1][0] == 0 {
                    stop = true;
                } else {
                    c.cycles += 2;
                    let bounce = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 4;
                    if bounce {
                        // Bounce: speed = -speed, pos = -pos.
                        c.cycles += 5;
                        c.neg_s(0, 3);
                        c.neg_s(1, 2);
                        c.swc1(0, this + SPEED);
                        c.swc1(1, this + POS);
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            // Wrap: pos += length.
                            c.lwc1(0, this + LENGTH);
                            c.cycles += 2;
                            c.add_s(0, 2, 0);
                            store_pos = true;
                        } else {
                            stop = true;
                        }
                    }
                }
                if stop {
                    // Stop at the start, reversed.
                    c.cycles += 7;
                    c.neg_s(0, 3);
                    c.g[V0][0] = 1;
                    c.swc1(20, this + POS);
                    c.sw(this + STOPPED, c.g[V0][0]);
                    c.swc1(0, this + SPEED);
                    c.sw(this + ACTIVE, 0);
                }
            } else {
                c.cycles += 5;
                c.lwc1(4, this + LENGTH);
                c.c_le_s(4, 2);
                let past_end = c.fcc();
                c.g[V0][0] = 1;
                if past_end {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(this + MODE);
                    c.g[V0][0] = 2;
                    store_pos = true;
                    if c.g[V1][0] == 0 {
                        // Stop just before the end, reversed.
                        c.cycles += 8;
                        c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_STOP));
                        c.neg_s(1, 3);
                        c.g[V0][0] = 1;
                        c.sw(this + ACTIVE, 0);
                        c.sub_s(0, 4, 0);
                        c.sw(this + STOPPED, c.g[V0][0]);
                        c.swc1(1, this + SPEED);
                    } else {
                        c.cycles += 2;
                        let bounce = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 4;
                        if bounce {
                            // Bounce: speed = -speed, pos -= pos - (length - 0.1).
                            c.cycles += 7;
                            c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_BOUNCE));
                            c.neg_s(1, 3);
                            c.sub_s(0, 4, 0);
                            c.swc1(1, this + SPEED);
                            c.sub_s(0, 2, 0);
                            c.sub_s(0, 2, 0);
                        } else {
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                // Wrap: pos -= length.
                                c.sub_s(0, 2, 4);
                            } else {
                                // Mode 4: stop just before the end, reversed.
                                c.cycles += 7;
                                c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_STOP4));
                                c.neg_s(1, 3);
                                c.g[V0][0] = 1;
                                c.sw(this + ACTIVE, 0);
                                c.sub_s(0, 4, 0);
                                c.sw(this + STOPPED, c.g[V0][0]);
                                c.swc1(1, this + SPEED);
                            }
                        }
                    }
                }
            }
            if store_pos {
                c.cycles += 1;
                c.swc1(0, this + POS);
            }
            if store_pos || (before_start && !stop) {
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
            c.cycles += 2;
            c.sw(this + DIRTY, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiSplineModifier::Save(cStream* s)` (0x35b5a8): writes the
/// modifier's 0x34-byte header (+4: count first) with `s->Write` (vtable
/// +8), its path (+0x48, `SplineModifier_SavePath`), the pointer at +0x40
/// with `s->WritePtr` (vtable +0x28), then for each of the `count` splines
/// (array at +0x44) the word at +0x78.
pub fn multi_spline_modifier_save(c: &mut Cpu) {
    const COUNT: u32 = 0x4;
    const OWNER: u32 = 0x40;
    const SPLINES: u32 = 0x44;
    const PATH: u32 = 0x48;
    const SPLINE_WORD: u32 = 0x78;
    const SAVE_PATH: Callee = Callee {
        addr: 0x0034_5538,
        f: ee::s11::SplineModifier_SavePath,
    };
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0x34;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        // s->Write(&self->count, 0x34)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0] as u32;
        c.g[A1][0] = sx(me.wrapping_add(COUNT));
        let vt = c.lw(stream);
        c.g[V1][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V0][0] = write;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0035_b5e4)?;
        // SplineModifier_SavePath(&self->path, s)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0];
        c.g[A0][0] = sx(me.wrapping_add(PATH));
        c.g[A1][0] = stream;
        SAVE_PATH.call(c, 0x0035_b5f0)?;
        // s->WritePtr(self->owner)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0] as u32;
        let vt = c.lw(stream);
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lw(me.wrapping_add(OWNER));
        let delta = c.lh((vt as u32).wrapping_add(0x28));
        let write_ptr = c.lw((vt as u32).wrapping_add(0x2c));
        c.g[V1][0] = write_ptr;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write_ptr as u32, 0x0035_b608)?;
        let count = c.lw((c.g[S2][0] as u32).wrapping_add(COUNT));
        c.g[V0][0] = count;
        if count as i64 > 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SPLINES));
            loop {
                // s->Write(&splines[i]->word78, 4)
                c.cycles += 15;
                let i = c.g[S0][0] as u32;
                let stream = c.g[S1][0] as u32;
                let at = sx((i << 2).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = at;
                let vt = c.lw(stream);
                c.g[A3][0] = vt;
                c.g[A2][0] = 0x4;
                c.g[S0][0] = sx(i.wrapping_add(1));
                let spline = c.lw(at as u32);
                let delta = c.lh((vt as u32).wrapping_add(0x8));
                let write = c.lw((vt as u32).wrapping_add(0xc));
                c.g[V1][0] = write;
                c.g[A1][0] = sx((spline as u32).wrapping_add(SPLINE_WORD));
                c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
                call_ptr(c, write as u32, 0x0035_b644)?;
                let count = c.lw((c.g[S2][0] as u32).wrapping_add(COUNT));
                let more = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                c.g[V0][0] = more;
                if more == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SPLINES));
                if event_due(c) {
                    deliver_event_here(c, 0x0035_b618);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festatecredits::tests::RET_ZERO;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_3;
    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn spline_modifier_set_speed_matches() {
        check(
            ssx3_ee::s11::f_00359688,
            spline_modifier_set_speed,
            200,
            &|c, seed| {
                let mut r = Rng(0x1027 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(GAME_GP - 0x2a60, r.edge_f32());
            },
        );
    }

    #[test]
    fn multi_spline_modifier_set_speed_matches() {
        check(
            ssx3_ee::s11::f_0035a550,
            multi_spline_modifier_set_speed,
            200,
            &|c, seed| {
                let mut r = Rng(0x1028 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(GAME_GP - 0x2a28, r.edge_f32());
            },
        );
    }

    #[test]
    fn rail_modifier_list_push_front_matches() {
        check(
            ssx3_ee::s11::f_0035b670,
            rail_modifier_list_push_front,
            200,
            &|c, seed| {
                let mut r = Rng(0x1029 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((DATA + 0x100) as u64) | ((r.next_u32() as u64) << 32);
                c.bus
                    .write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
            },
        );
    }

    #[test]
    fn rail_modifier_list_unlink_matches() {
        check(
            ssx3_ee::s11::f_0035b6d0,
            rail_modifier_list_unlink,
            200,
            &|c, seed| {
                let mut r = Rng(0x102a ^ seed as u64);
                scramble(c, &mut r);
                let m = DATA;
                let prev = DATA + 0x100;
                let next = DATA + 0x200;
                c.g[A0][0] = m as u64;
                c.bus.write32(m, if seed % 4 >= 2 { prev } else { 0 });
                c.bus.write32(m + 4, if seed % 2 == 0 { 0 } else { next });
                c.bus.write32(prev + 4, r.next_u32());
                c.bus.write32(next, r.next_u32());
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v24_matches() {
        covered(
            ssx3_ee::s11::f_0035a918__generated,
            multi_spline_modifier_v24,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35a918, 1);
                let vt = vtable(c, &mut r, &[0x18]);
                let selfp = DATA;
                let world = DATA + 0x4000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x4100);
                c.bus.write32(DATA + 0x4100 + 0x20, DATA + 0x4400);
                let elems = DATA + 0x1000;
                let insts = DATA + 0x3000;
                c.g[A0][0] = selfp as u64;
                c.bus
                    .write32(selfp + 0x30, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(selfp + 4, if seed % 3 == 0 { 1 } else { 2 });
                c.bus.write32(selfp + 0x3c, elems);
                c.bus.write32(selfp + 0x44, insts);
                put_f32(c, selfp + 0x18, 0);
                if seed % 2 != 0 {
                    c.bus.write32(selfp, vt);
                }
                for i in 0..2 {
                    c.bus
                        .write32(insts + 4 * i, DATA + 0x5000 + 0x100 * i as u32);
                }
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v11_matches() {
        covered(
            ssx3_ee::s11::f_0035b418__generated,
            multi_spline_modifier_v11,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35b418, 5);
                let selfp = DATA;
                let world = DATA + 0x4000;
                let model = DATA + 0x7000;
                let modelobj = DATA + 0x8000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), world);
                // vt used for slots 43 and 93 (w + 0x10d8).
                let vt1 = DATA + 0x5000;
                c.bus.write32(world + 0x10d8, vt1);
                c.bus.write16(vt1 + 0x158, 0);
                c.bus.write32(vt1 + 0x15c, FAKE);
                c.bus.write16(vt1 + 0x2e8, 0);
                c.bus.write32(vt1 + 0x2ec, FAKE + 0x10);
                // model -> object vtable slots 33 and 5.
                c.bus.write32(model + 0xc, modelobj);
                let vt2 = DATA + 0x6000;
                c.bus.write32(modelobj + 0xc, vt2);
                c.bus.write16(vt2 + 0x108, 0);
                c.bus.write32(vt2 + 0x10c, FAKE + 0x20);
                c.bus.write16(vt2 + 0x28, 0);
                c.bus.write32(vt2 + 0x2c, FAKE + 0x30);
                c.g[A0][0] = selfp as u64;
                c.bus.write32(selfp, 0);
                c.bus.write32(selfp + 4, if seed % 4 == 0 { 1 } else { 2 });
                c.bus
                    .write32(selfp + 0x30, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(selfp + 0x38, r.next_u32() % 2);
                c.bus.write32(selfp + 0x3c, DATA + 0x1000);
                c.bus.write32(selfp + 0x40, model);
                c.bus.write32(selfp + 0x44, DATA + 0x3000);
                c.bus.write32(selfp + 0x1c, 0);
                let forced = if seed % 3 == 0 { 1 } else { 2 };
                c.bus.write32(LOG_RESULTS + 4, forced);
                c.bus.write32(LOG_RESULTS + 8, forced);
                for i in 0..2 {
                    c.bus
                        .write32(DATA + 0x3000 + 4 * i, DATA + 0x9000 + 0x100 * i as u32);
                }
            },
        );
    }

    // -------------------------------------------------- MultiSplineModifier_Update

    #[test]
    fn multi_spline_modifier_update_matches() {
        check(
            ee::s11::f_0035a560__generated,
            multi_spline_modifier_update,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0e50_00d4, 1);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                // World_GetSecondsPerTick reads world->+0x14.
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x14, r.edge_f32());
                c.bus.write32(
                    this + 0x10,
                    match seed % 4 {
                        0 => 0,
                        1 => 0x3f80_0000,
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(
                    this + 0x14,
                    match seed % 5 {
                        0 => 0x7f80_0000, // largest, so the position overflows
                        1 => 0x8000_0000, // -0.0
                        2 => 0x3f80_0000,
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(
                    this + 0x54,
                    match seed % 3 {
                        0 => 0,
                        1 => 0x3f80_0000,
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(this + 0x30, r.next_u32());
            },
        );
    }

    #[test]
    fn rail_modifier_list_push_back_matches() {
        check(
            ssx3_ee::s11::f_0035b690__generated,
            rail_modifier_list_push_back,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x35b6_90, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                match seed % 3 {
                    0 => {}
                    1 => {
                        w(c, DATA, DATA + 0x20);
                        w(c, DATA + 0x20, 0);
                    }
                    _ => {
                        w(c, DATA, DATA + 0x20);
                        w(c, DATA + 0x20, DATA + 0x40);
                        w(c, DATA + 0x40, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn spline_modifier_v22_matches() {
        check_lohi(
            ssx3_ee::s11::f_00359cf8__generated,
            spline_modifier_v22,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x359cf8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x2a30),
                    1.0e-5f32.to_bits(),
                );
                // the modifier struct: active, stopped and the spline parameters
                for k in 0..0x38 {
                    word(c, DATA + 4 * k, r.edge_f32());
                }
                word(c, DATA + 0x54, if seed % 5 == 0 { 0 } else { 1 });
                word(c, DATA + 0x50, if seed % 5 == 1 { 1 } else { 0 });
                word(
                    c,
                    DATA + 0xb0,
                    match seed % 4 {
                        0 => 0.0f32.to_bits(),
                        1 => 1.0e-9f32.to_bits(),
                        2 => 1.0f32.to_bits(),
                        _ => r.edge_f32(),
                    },
                );
                for off in [0xb4u32, 0xb8, 0xc0, 0xc4, 0xc8, 0xd0, 0xd4] {
                    word(
                        c,
                        DATA + off,
                        pick(&mut r, &[0.0, 1.0, -1.0, 0.5, 2.0, -0.25]),
                    );
                }
                word(c, DATA + 0x48, pick(&mut r, &[0.0, 1.0, -2.5]));
                word(c, DATA + 0x38, pick(&mut r, &[0.0, 1.0, -1.0, 3.5]));
                // the body
                for k in 0..0x40 {
                    word(c, DATA + 0x1000 + 4 * k, r.f32(10.0));
                }
            },
        );
    }

    #[test]
    fn multi_spline_modifier_release_matches() {
        leaf(
            ssx3_ee::s11::f_0035aae0__generated,
            multi_spline_modifier_release,
            30,
        );
    }

    #[test]
    fn multi_spline_modifier_alloc_node_instances_matches() {
        check(
            ee::s11::f_0035a3f0__generated,
            multi_spline_modifier_alloc_node_instances,
            40,
            &|c, seed| {
                base(c, seed, 0x35_a3f0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x4, (seed % 4) as u32);
                c.bus.write32(DATA + 0x44, DATA + 0x200);
                // gp-0x848 -> world, +0x84 -> module, +0x88 -> instance manager.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0x88, DATA + 0x1200);
                // Empty instance-manager free list.
                c.bus.write32(DATA + 0x1200, 0);
                c.bus.write32(DATA + 0x1208, 0);
                c.bus.write32(DATA + 0x120c, 0);
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v19_matches() {
        check(
            ee::s11::f_0035aaf0__generated,
            multi_spline_modifier_v19,
            60,
            &|c, seed| {
                machine(c, seed, 0x35aa_f0, 0);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(item);
                let spline = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(obj + 0x30, spline);
                if spline != 0 {
                    put_vtable(c, spline, &[0x18]);
                }
                c.bus
                    .write32(obj + 0x40, if seed % 2 == 0 { item } else { 0 });
                let a1 = DATA + 0x2400;
                c.bus.write32(item + 0xc, a1);
                put_vtable(c, a1, &[0xc0]);
            },
        );
    }

    #[test]
    fn spline_modifier_v05_matches() {
        check(
            ee::s11::f_00359eb8__generated,
            spline_modifier_v05,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x359eb8, 2);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = [0x190, 0x191, 0x192, 0x200][(seed % 4) as usize] as u64;
                let mut r = Rng(0x359e_0000 ^ seed as u64);
                w(c, DATA + 0x48, r.edge_f32());
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v23_matches() {
        check(
            ee::s11::f_0035ab60__generated,
            multi_spline_modifier_v23,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x35ab, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let other = DATA + 0x900;
                let cur = [item, other, 0][(seed % 3) as usize];
                let vt = DATA + 0x1000;
                for off in [0xd0u32, 0xd8, 0x108] {
                    slot(c, vt, off, if seed % 2 == 0 { EMPTY } else { RET_ZERO });
                }
                w32(c, this + 0x40, cur);
                w32(c, this + 0x38, 0x21);
                w32(c, item + 0xc, DATA + 0x1200);
                w32(c, DATA + 0x1200 + 0xc, vt);
                w32(c, other + 0xc, DATA + 0x1300);
                w32(c, DATA + 0x1300 + 0xc, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn spline_modifier_ctor_2_matches() {
        check(
            ee::s11::f_003595d8__generated,
            spline_modifier_ctor_2,
            4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0035_95d8, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, OBJ2, OBJ3);
                c.bus.write16(OBJ3 + 0x10, (seed as u16) & !3);
                w(c, OBJ3 + 0x14, FAKE);
                w(c, OBJ + 0xa0, 0);
                w(c, OBJ + 0xb0, 0);
                w(c, OBJ + 0xc0, 0);
                w(c, OBJ + 0xd8, 0);
            },
        );
    }

    #[test]
    fn multisplinemodifier_v01_matches() {
        check(
            ee::s11::f_0035a250__generated,
            multisplinemodifier_v01,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x35a250, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                w(c, this + 0x4, if seed % 2 == 0 { 0 } else { -1i32 as u32 });
                w(c, this + 0x38, 0);
                w(c, this + 0x3c, 0);
                w(c, this + 0x44, 0);
            },
        );
    }
    use super::*;

    /// MultiSplineModifier_UpdateNodes: 0-5 nodes, every mode (0: neither
    /// angle, 1: pitch, 2: yaw, 3: both, 4: neither), with and without a
    /// spin angle, start distances below 0 and beyond the length (both
    /// wraps). The path is one straight-forward segment whose direction
    /// components are zeroed per seed (dir.x/dir.y/dir.z = 0 reach the
    /// zero-denominator and zero-angle branches; negative components the
    /// quadrant fixes). Real callees (EvalPathAtDistance, Math_SinCos,
    /// Math_Atan) on the game image's constants.
    #[test]
    fn multi_spline_modifier_update_nodes_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{elf_image, put_vec, DATA};
        const SELF: u32 = DATA;
        const STATES: u32 = DATA + 0x1000;
        const NODES: u32 = DATA + 0x2000;
        const NODE_OBJS: u32 = DATA + 0x2100;
        const SEG: u32 = DATA + 0x3000;
        crate::difftest::check(
            ee::s11::f_0035ac20__generated,
            multi_spline_modifier_update_nodes,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35ac_20, 0);
                elf_image(c);
                c.pc = 0x0035_ac20;
                let f = |x: f32| x.to_bits();
                let n = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + r.next_u32() % 4,
                };
                let len = 10.0 + (r.next_u32() % 100) as f32;
                let start = match seed % 3 {
                    0 => -((r.next_u32() % 50) as f32) - 0.5,
                    1 => len + (r.next_u32() % 50) as f32,
                    _ => (r.next_u32() % 10) as f32,
                };
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x4, n);
                c.bus.write32(SELF + 0x8, (seed / 3) % 5);
                let spin = if seed % 4 == 0 { 0 } else { r.f32(3.0) };
                c.bus.write32(SELF + 0xc, spin);
                c.bus.write32(SELF + 0x10, f(start));
                c.bus.write32(SELF + 0x30, 1);
                c.bus.write32(SELF + 0x3c, STATES);
                c.bus.write32(SELF + 0x44, NODES);
                c.bus.write32(SELF + 0x54, f(len));
                for i in 0..6 {
                    c.bus.write32(NODES + 4 * i, NODE_OBJS + 0x80 * i);
                }
                // The path: one segment covering everything.
                c.bus.write32(SELF + 0x48 + 0x4, 0);
                c.bus.write32(SELF + 0x48 + 0x8, SEG);
                c.bus.write32(SELF + 0x48 + 0xc, f(len * 2.0));
                c.bus.write32(SEG + 0x84, 0);
                c.bus.write32(SEG + 0x60, 0);
                c.bus.write32(SEG + 0x64, 0);
                for k in 0..4 {
                    c.bus
                        .write32(SEG + 0x50 + 4 * k, f((r.next_u32() % 8) as f32 * 0.125));
                }
                // Direction columns, with components zeroed by seed.
                let zero = (seed / 15) % 8; // bit 0: x, 1: y, 2: z
                for row in 0..4u32 {
                    let mut v = [
                        r.f32(4.0),
                        r.f32(4.0),
                        r.f32(4.0),
                        f(if row == 3 { 1.0 } else { 0.0 }),
                    ];
                    for k in 0..3usize {
                        if zero & (1 << k) != 0 {
                            v[k] = 0;
                        }
                    }
                    put_vec(c, SEG + 0x10 + 0x10 * row, v);
                }
                // The world up axis.
                put_vec(c, 0x004f_f160, [f(0.0), f(0.0), f(1.0), 0]);
                if seed % 5 == 0 {
                    put_vec(c, 0x004f_f160, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
            },
        );
    }
    use super::*;

    /// SplineModifier_UpdateTransform: every mode (0: no angle, 1: pitch,
    /// 2: yaw, 3: both, 4: none), with and without a spin angle. The path is
    /// one segment whose direction components are zeroed per seed (x / y / z
    /// = 0 reach the zero-denominator and zero-angle branches; negative
    /// components the quadrant fixes). Real callees (EvalPathAtDistance,
    /// Math_SinCos, Math_Atan) on the game image's constants.
    #[test]
    fn spline_modifier_update_transform_matches() {
        const SELF: u32 = DATA;
        const SEG: u32 = DATA + 0x3000;
        check(
            ee::s11::f_00359830__generated,
            spline_modifier_update_transform,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_9830, 0);
                elf_image(c);
                c.pc = 0x0035_9830;
                let f = |x: f32| x.to_bits();
                let len = 10.0 + (r.next_u32() % 100) as f32;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + SM_MODE, seed % 5);
                let spin = if seed % 4 == 0 { 0 } else { r.f32(3.0) };
                c.bus.write32(SELF + SM_SPIN, spin);
                c.bus.write32(SELF + SM_DIST, f((r.next_u32() % 10) as f32));
                c.bus.write32(SELF + SM_DIRTY, 1);
                // The path: one segment covering everything.
                c.bus.write32(SELF + SM_PATH + 0x4, 0);
                c.bus.write32(SELF + SM_PATH + 0x8, SEG);
                c.bus.write32(SELF + SM_PATH + 0xc, f(len * 2.0));
                c.bus.write32(SEG + 0x84, 0);
                c.bus.write32(SEG + 0x60, 0);
                c.bus.write32(SEG + 0x64, 0);
                for k in 0..4 {
                    c.bus
                        .write32(SEG + 0x50 + 4 * k, f((r.next_u32() % 8) as f32 * 0.125));
                }
                // Direction columns, with components zeroed by seed.
                let zero = (seed / 5) % 8; // bit 0: x, 1: y, 2: z
                for row in 0..4u32 {
                    let mut v = [
                        r.f32(4.0),
                        r.f32(4.0),
                        r.f32(4.0),
                        f(if row == 3 { 1.0 } else { 0.0 }),
                    ];
                    for k in 0..3usize {
                        if zero & (1 << k) != 0 {
                            v[k] = 0;
                        }
                    }
                    put_vec(c, SEG + 0x10 + 0x10 * row, v);
                }
                // The world up axis.
                put_vec(c, SM_UP_AXIS, [f(0.0), f(0.0), f(1.0), 0]);
                if seed % 3 == 0 {
                    put_vec(c, SM_UP_AXIS, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
            },
        );
    }
    use super::*;
    use super::*;

    /// Runs a guest function to completion as a setup step (returns v0).
    pub(crate) fn oc259_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// SpatialHash_Remove on a real octree path: the root key one to four
    /// levels above the item (or at it), nodes from the real main heap so
    /// the real `__builtin_delete` frees emptied ones; every node on the
    /// path randomly kept alive by another child or item; the link first,
    /// last, middle or alone in its list. Some seeds resume at the scan's
    /// event-check heads (0x328614 / 0x328620) inside a frame.
    #[test]
    fn spatial_hash_remove_matches() {
        const MAIN_HEAP: u32 = 0x004f_f250;
        const ROOT_KEY: u32 = DATA + 0x100;
        const ITEM_KEY: u32 = DATA + 0x110;
        const LINK: u32 = DATA + 0x200;
        const ROOT: u32 = DATA + 0x1000;
        check(
            ee::s10::f_003284b8__generated,
            spatial_hash_remove,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_84b8, 0);
                let ra = c.g[RA][0];
                // __builtin_delete -> MEM_FreeInternal: no pools, the main heap.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                for slot in [0x2a60u32, 0x2a68, 0x2a6c, 0x2a70, 0x2a78, 0x2a7c, 0x2a84] {
                    c.bus.write32(GAME_GP + slot, 0);
                }
                oc259_run(
                    c,
                    0x0031_ff60,
                    ee::s10::DLHeap_InitArena,
                    &[MAIN_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
                );
                let new_node = |c: &mut Cpu, r: &mut Rng| -> u32 {
                    let n = oc259_run(
                        c,
                        0x0031_e818,
                        ee::s10::DLHeap_Malloc,
                        &[MAIN_HEAP as u64, 0x2c],
                    ) as u32;
                    for i in 0..11 {
                        c.bus.write32(n + 4 * i, 0);
                    }
                    match r.next_u32() % 3 {
                        0 => c.bus.write32(n + 4 * (r.next_u32() % 8), r.next_u32() | 1),
                        1 => c
                            .bus
                            .write32(n + SH_NODE_ITEMS + 4 * (r.next_u32() % 3), r.next_u32() | 1),
                        _ => {}
                    }
                    n
                };
                c.g[RA][0] = ra;

                if seed % 10 == 9 {
                    // Resume inside a frame at a scan head.
                    let child = new_node(c, &mut r);
                    let frame_sp = crate::difftest::SP - 0x30;
                    c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(frame_sp);
                    c.bus.write64(frame_sp + 0x10, crate::difftest::RA as u64);
                    let parent_slot = ROOT + 4 * (r.next_u32() % 8);
                    c.bus.write32(parent_slot, child);
                    c.g[S0][0] = sx(parent_slot);
                    let j = r.next_u32() % 3;
                    c.g[A0][0] = j as u64;
                    c.g[V1][0] = sx(child + SH_NODE_ITEMS + 4 * j);
                    c.pc = if seed % 20 == 9 {
                        SH_REMOVE_SCAN_ITEMS
                    } else {
                        SH_REMOVE_NOT_EMPTY
                    };
                    return;
                }

                let top = 1 + r.next_u32() % 4;
                let item_level = r.next_u32() % (top + 1);
                let span = 1u32 << (top - item_level);
                let xyz = [
                    r.next_u32() % span,
                    r.next_u32() % span,
                    r.next_u32() % span,
                ];
                put_words(c, ITEM_KEY, &[item_level, xyz[0], xyz[1], xyz[2]]);
                put_words(c, ROOT_KEY, &[top, 0, 0, 0]);
                for i in 0..11 {
                    c.bus.write32(ROOT + 4 * i, 0);
                }
                // The path from the root down to the item's level.
                let mut node = ROOT;
                let mut k = [0u32; 3];
                for lev in (item_level + 1..=top).rev() {
                    let shift = lev - item_level - 1;
                    let b: Vec<u32> = (0..3).map(|i| (xyz[i] >> shift) - 2 * k[i]).collect();
                    let child = new_node(c, &mut r);
                    c.bus
                        .write32(node + (b[0] << 4) + (b[1] << 3) + (b[2] << 2), child);
                    for i in 0..3 {
                        k[i] = 2 * k[i] + b[i];
                    }
                    node = child;
                }
                let slot = r.next_u32() % 3;
                let next = if r.next_u32() % 2 == 0 {
                    DATA + 0x210
                } else {
                    0
                };
                let prev = if r.next_u32() % 2 == 0 {
                    DATA + 0x220
                } else {
                    0
                };
                put_words(c, LINK, &[next, prev]);
                put_words(c, DATA + 0x210, &[r.next_u32(), r.next_u32()]);
                put_words(c, DATA + 0x220, &[r.next_u32(), r.next_u32()]);
                if prev == 0 {
                    c.bus.write32(node + SH_NODE_ITEMS + 4 * slot, LINK);
                }
                c.pc = SH_REMOVE_ENTRY;
                c.g[A0][0] = sx(ROOT);
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(LINK);
                c.g[A3][0] = sx(ITEM_KEY);
                c.g[T0][0] = sx(ROOT_KEY);
            },
        );
    }

    /// SplineModifier_Update: acceleration time positive or not, active /
    /// stopped flags (one seed in five each), modes 0..4, a speed and
    /// position that leave the spline at the start, at the end, or stay
    /// inside; the world's seconds-per-tick (1/60 or random) and the 0.1
    /// literals.
    #[test]
    fn spline_modifier_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{DATA, GAME_GP};
        use spline_mod::*;
        const SELF: u32 = DATA + 0x3_0000;
        const WORLD: u32 = DATA + 0x3_1000;
        crate::difftest::check(
            ee::s11::f_00359698__generated,
            spline_modifier_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_9698, 0);
                c.pc = 0x0035_9698;
                let fl = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP - 0x848, WORLD);
                let dt = if seed % 4 == 0 {
                    r.f32(0.5)
                } else {
                    fl(1.0 / 60.0)
                };
                c.bus.write32(WORLD + 0x14, dt);
                for lit in [
                    LIT_END_MARGIN_STOP,
                    LIT_END_MARGIN_BOUNCE,
                    LIT_END_MARGIN_STOP4,
                ] {
                    let v = if seed % 3 == 0 { r.f32(1.0) } else { fl(0.1) };
                    c.bus.write32(GAME_GP - lit, v);
                }
                let accel_time = match seed % 4 {
                    0 => 0,
                    1 => fl(-0.5),
                    _ => r.f32(2.0),
                };
                c.bus.write32(SELF + ACCEL_TIME, accel_time);
                c.bus.write32(SELF + ACCEL, r.f32(100.0));
                let length = 50.0 + (r.next_u32() % 100) as f32;
                c.bus.write32(SELF + LENGTH, fl(length));
                // Inside, just past the end, just before the start.
                let (pos, speed) = match (seed / 2) % 3 {
                    0 => (length * 0.5, 30.0),
                    1 => (length - 0.2, 600.0),
                    _ => (0.2, -600.0),
                };
                let speed = if seed % 7 == 0 { -speed } else { speed };
                c.bus.write32(SELF + POS, fl(pos));
                c.bus.write32(SELF + SPEED, fl(speed));
                c.bus.write32(SELF + MODE, r.next_u32() % 5);
                c.bus.write32(
                    SELF + ACTIVE,
                    if seed % 5 == 1 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    },
                );
                c.bus
                    .write32(SELF + STOPPED, if seed % 5 == 2 { 1 } else { 0 });
                c.bus.write32(SELF + DIRTY, r.next_u32());
                c.g[A0][0] = SELF as u64;
            },
        );
    }
    use super::*;

    /// `MultiSplineModifier_Save`: a stream whose vtable `Write` (+8) and
    /// `WritePtr` (+0x28) are fakes (logging every register, sometimes
    /// unwinding); `SplineModifier_SavePath` runs for real. The spline
    /// count is -1..5 (no loop for <= 0), the spline array holds random
    /// pointers.
    #[test]
    fn multi_spline_modifier_save_matches() {
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        const SPLINES: u32 = DATA + 0x400;
        covered(
            ee::s11::f_0035b5a8__generated,
            multi_spline_modifier_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35b5a8, 6);
                c.pc = 0x0035_b5a8;
                let count = (seed % 7) as i32 - 1;
                c.bus.write32(ME + 4, count as u32);
                c.bus.write32(ME + 0x40, r.next_u32());
                c.bus.write32(ME + 0x44, SPLINES);
                for i in 0..6 {
                    c.bus
                        .write32(SPLINES + 4 * i, DATA + 0x1000 + (r.next_u32() & 0xfff0));
                }
                c.bus.write32(STREAM, VT);
                for (k, off) in [8u32, 0x28].into_iter().enumerate() {
                    c.bus.write16(VT + off, (r.next_u32() % 0x20) as u16 & !3);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * k as u32);
                }
                c.g[A0][0] = sx(ME);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }
}
