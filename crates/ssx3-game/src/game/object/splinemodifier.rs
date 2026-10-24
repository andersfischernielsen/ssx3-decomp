//! `src/object/splinemodifier.cpp`: hand-written replacements (moved from the
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
}
