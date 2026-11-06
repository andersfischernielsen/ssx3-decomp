//! `src/object/effectlink.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::feasyncfile::lw_3;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp as gp_2;
pub(crate) use crate::game::fe::fereal::open;
use crate::game::lib::snd;
pub(crate) use crate::game::sound::asyncsys::lw as lw_2;
pub(crate) use crate::game::visualfx::boardwakefx::fcc;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::world::worldview::p_2;
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
    (
        0x0034_5700,
        modifier_list_push_front,
        "ModifierList_PushFront",
    ),
    (0x0034_5760, modifier_list_unlink, "ModifierList_Unlink"),
    (
        0x0034_5798,
        particle_modifier_base_ctor,
        "ParticleModifier_BaseCtor",
    ),
    (
        0x0034_57c8,
        particle_modifier_base_ctor_2,
        "ParticleModifier_BaseCtor_2",
    ),
    (
        0x0034_5828,
        particle_modifier_base_dtor,
        "ParticleModifier_BaseDtor",
    ),
    (
        0x0034_59a8,
        particle_modifier_ctor_2,
        "ParticleModifier_ctor_2",
    ),
    (0x0034_5ad0, particle_modifier_dtor, "ParticleModifier_dtor"),
    (
        0x0034_5b40,
        particle_modifier_update,
        "ParticleModifier_Update",
    ),
    (
        0x0034_5bc8,
        particle_modifier_render,
        "ParticleModifier_Render",
    ),
    (
        0x0034_5e88,
        dynamic_particle_modifier_dtor,
        "DynamicParticleModifier_dtor",
    ),
    (0x0034_61c0, halo_modifier_ctor_2, "HaloModifier_ctor_2"),
    (0x0034_6228, halo_modifier_dtor, "HaloModifier_dtor"),
    (0x0034_6258, halo_modifier_update, "HaloModifier_Update"),
    (
        0x0034_5f90,
        dynamic_particle_modifier_update,
        "DynamicParticleModifier_Update",
    ),
    (
        0x0034_5ef8,
        dynamic_particle_modifier_transform_points,
        "DynamicParticleModifier_TransformPoints",
    ),
    (
        0x0034_6070,
        dynamic_particle_modifier_render,
        "DynamicParticleModifier_Render",
    ),
    (0x0034_62a0, halo_modifier_render, "HaloModifier_Render"),
    (0x0034_5bf0, particle_modifier_save, "ParticleModifier_Save"),
    (
        0x0034_5890,
        particle_modifier_base_save,
        "ParticleModifier_BaseSave",
    ),
    (
        0x0034_60a0,
        dynamic_particle_modifier_save,
        "DynamicParticleModifier_Save",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::ANIM_SEQ_LIST_GET_AT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::EMITTER_UPDATE;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::FLAGS64_TEST;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::MATH_COS;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_COMPUTE_LOD;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_MODEL_RENDER;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::TRANSITION_EFFECT_UPDATE_SCREEN_RECT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::WORLD_PAINTER_QUERY_RESET_ALL;
}

// ---------------------------------------------------------------------------
// Modifier lists.

/// `void ModifierList_PushFront(ModifierList *list, Modifier *m)`
/// (0x345700): push `m` at the head.
pub fn modifier_list_push_front(c: &mut Cpu) {
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

/// `void ModifierList_Unlink(Modifier *m)` (0x345760): unlink `m` from its
/// doubly linked list.
pub fn modifier_list_unlink(c: &mut Cpu) {
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

/// `ParticleModifier* ParticleModifier_BaseCtor(ParticleModifier *m, void *o,
/// ...)` (0x345798): initialise the base fields and bump the instance counter.
pub fn particle_modifier_base_ctor(c: &mut Cpu) {
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xf04)).lw(c);
    c.g[V0][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
    this.off(0xc).sw(c, c.g[A1][0]);
    this.off(8).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    this.sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(4).sw(c, 0);
    Addr((c.g[GP][0] as u32).wrapping_add(0xf04)).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// ParticleModifier_Update.

/// `void ParticleModifier_Update(ParticleModifier* m)` (0x345b40): while the
/// modifier is not done (`+0x1e4`), tick its emitter (`Emitter_Update(m+0x50,
/// dt)`); a negative life takes the single-tick path, otherwise the life is
/// re-checked and the modifier is marked done once it has run out. `f20` is
/// zeroed (the compare constant) and restored.
pub fn particle_modifier_update(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let m = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x20);
    guest(c, |c| {
        let done = lw(c, p(m, 0x1e4));
        c.g[V0][0] = done;
        if done == 0 {
            c.cycles += 6;
            let life = ldf(c, 0, p(m, 0x50));
            c.mtc1(0, 20); // f20 = 0.0
            let negative = fcc(c, life.lt(FpuF32::ZERO));
            c.g[A0][0] = sx(p(m, 0x50));
            if !negative {
                // life >= 0: tick, then mark done when it reached the end.
                c.cycles += 8;
                ldf(c, 12, gp(c, -0x2b50));
                call_ptr(c, callee::EMITTER_UPDATE, 0x0034_5b98)?;
                let life = ldf(c, 0, p(m, 0x50));
                let spent = fcc(c, life.le(FpuF32::ZERO));
                c.g[V0][0] = sx(1);
                if spent {
                    c.cycles += 1;
                    Addr(p(m, 0x1e4)).sw(c, 1);
                }
                c.cycles += 1;
            } else {
                // life < 0: one tick only.
                c.cycles += 5;
                ldf(c, 12, gp(c, -0x2b54));
                call_ptr(c, callee::EMITTER_UPDATE, 0x0034_5b84)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParticleModifier_Render.

/// `void ParticleModifier_Render(ParticleModifier* m)` (0x345bc8): render the
/// modifier's effect (`m + 0x50`) only while its render handle (`+0x1e4`) is
/// still unset.
pub fn particle_modifier_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1e4));
        let has_handle = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !has_handle {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
            call_ptr(c, 0x0037_08c0, 0x0034_5be4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Effects / modifiers.

/// `HaloModifier_dtor(this, flags)` (0x346228): republish the modifier vtable
/// at `+0x8`, drop the particle-modifier live count at `gp+0xf10`, then run
/// the base destructor (which unlinks the modifier and may delete it).
pub fn halo_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xf10).lw(c);
        c.g[V1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x1220));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let this = c.g[A0][0] as u32;
        let v = c.g[V1][0];
        Addr(this).off(8).sw(c, v);
        let gp = c.g[GP][0] as u32;
        let count = c.g[V0][0];
        Addr(gp).off(0xf10).sw(c, count);
        call_ptr(c, 0x0034_5828, 0x0034_624c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void HaloModifier_Update(HaloModifier* h)` (0x00346258): rotate by the
/// per-frame step (an int at +0x2c) and wrap the angle at +0x30 at +-360.
pub fn halo_modifier_update(c: &mut Cpu) {
    let h = abi::arg(c, 0);
    c.cycles += 10;
    c.lwc1(1, p_2(h, 0x2c));
    c.cvt_s_w(1, 1);
    // The step at +0x2c is degrees per 60 Hz frame: at a higher rate the same
    // rotation per second needs a proportionally smaller per-tick step (the
    // 60 Hz path keeps the original value untouched).
    let hz = crate::tick::rate(c);
    if hz != 60 {
        let k = FpuF32::from_f32(60.0 / hz as f32);
        c.f[1] = (FpuF32(c.f[1]) * k).0;
    }
    c.lwc1(0, p_2(h, 0x30));
    c.g[AT][0] = 0x43b4_0000; // 360.0f
    c.mtc1(1, 2);
    c.add_s(1, 0, 1);
    c.c_le_s(2, 1); // 360 <= angle
    let wrap = c.fcc();
    c.swc1(1, p_2(h, 0x30));
    if wrap {
        c.cycles += 1;
        c.sw(p_2(h, 0x30), 0);
    } else {
        c.cycles += 5;
        c.g[AT][0] = 0xffff_ffff_c3b4_0000; // -360.0f
        c.mtc1(1, 0);
        c.c_le_s(1, 0); // angle <= -360
        if c.fcc() {
            c.cycles += 1;
            c.sw(p_2(h, 0x30), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `ParticleModifier* ParticleModifier_BaseCtor_2(ParticleModifier* self,
/// void* obj)` (0x3457c8): link-list init, base class from `obj`, and the
/// global modifier counter.
pub fn particle_modifier_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p_3(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
        Addr(p_3(c.g[S0][0], 4)).sw(c, 0);
        c.g[V1][0] = lw_3(c, c.g[A1][0] as u32);
        c.g[A0][0] = Addr(p_3(c.g[V1][0], 0x18)).lh(c);
        c.g[V0][0] = lw_3(c, p_3(c.g[V1][0], 0x1c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_5800)?;
        c.g[V1][0] = lw_3(c, p_3(c.g[GP][0], 0xf04));
        Addr(p_3(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(p_3(c.g[GP][0], 0xf04)).sw(c, c.g[V1][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void ParticleModifier_BaseDtor(ParticleModifier* self, int flags)`
/// (0x345828): retag the vtable twice (this class, then its base), unlink the
/// object from the modifier list and, for `flags & 1`, delete it.
pub fn particle_modifier_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        open(c, 0x30);
        c.g[V0][0] = 0x490000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
        c.g[V1][0] = c.lw(gp_2(c, 0xf04));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = 0x490000;
        c.sw(gp_2(c, 0xf04), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1340));
        c.g[RA][0] = sx(0x0034_5868);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0034_5760, 0x0034_5868)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let free = c.g[S0][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if !free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0034_587c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `HaloModifier* HaloModifier_ctor_2(HaloModifier* self, sMbStream* stream)`
/// (0x3461c0): base particle-modifier constructor, install this class's
/// vtable, then let the stream's slot 2 (`+0x10`) initialise `self + 0x10`.
pub fn halo_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0034_61dc);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0034_57c8, 0x0034_61dc)?;
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1220));
        c.g[A2][0] = 0x30;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[A3][0] as u32;
        c.g[RA][0] = sx(0x0034_6204);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0034_6204)?;
        c.g[V1][0] = c.lw(gp_2(c, 0xf10));
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw(gp_2(c, 0xf10), c.g[V1][0]);
        close(c, 0x30);
        Ok(())
    })
}

/// `void ParticleModifier_dtor(ParticleModifier* self, int flags)`
/// (0x345ad0): free the modifier's point buffer, drop it from the emitter
/// list and run the base destructor.
pub fn particle_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4912_b0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x1e0)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0034_5b04)?;
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(gpr(c, 0xf08)).lw(c);
        c.g[A0][0] = sx(at(c, S0, 0x50));
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(gpr(c, 0xf08)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0037_0758, 0x0034_5b1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5828, 0x0034_5b28)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicParticleModifier_dtor(DynamicParticleModifier* self, int
/// flags)` (0x345e88): the dynamic-particle variant of the same teardown.
pub fn dynamic_particle_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4912_68;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x260)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0034_5ebc)?;
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(gpr(c, 0xf0c)).lw(c);
        c.g[A0][0] = sx(at(c, S0, 0x60));
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(gpr(c, 0xf0c)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0037_0c08, 0x0034_5ed4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5828, 0x0034_5ee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Flex bridges.

/// `FlexBridge* FlexBridge_ctor_2(FlexBridge* self, u32, void* stream)`
/// (0x3464e0): the instance node, the bridge's springs, mesh and vertices.
pub fn flex_bridge_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_6504);
        c.g[S2][0] = sx(p(r32(c, S0), 0x20));
        callee::WORLD_OBJ_CTOR_2.call(c, 0x0034_6504)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx(0x0049_0270);
        c.g[A0][0] = sx(p(r32(c, S0), 0x50));
        c.g[RA][0] = sx(0x0034_651c);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        callee::FLEX_BRIDGE_SPRINGS_LOAD.call(c, 0x0034_651c)?;
        c.g[RA][0] = sx(0x0034_6524);
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_BUILD_MESH.call(c, 0x0034_6524)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_6540);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0034_6540)?;
        c.g[RA][0] = sx(0x0034_6548);
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_UPDATE_VERTS.call(c, 0x0034_6548)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* func_003459A8(void* self, void* stream)` (0x3459a8): construct an
/// effect-link at_2 `self`: run the base ctor, install the vtable, read the
/// effect id and, when it is the "matrix" effect, copy the matrix and attach
/// the emitter, otherwise copy the identity matrix.
pub fn particle_modifier_ctor_2(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_345aa8: {
        'b_345a78: {
            c.cycles += 25;
            frame.save(c, S1, 0x30);
            frame.save(c, S2, 0x20);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S3, 0x10);
            frame.save(c, S0, 0x40);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x003459cc);
            c.g[S2][0] = c.g[A1][0];
            if callee::C003457c8.call(c, 0x003459cc).is_err() {
                return;
            }
            c.g[S3][0] = sx(at_2(c, S1, 0x50));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = sx(at_2(c, V1, 0x12b0));
            c.g[RA][0] = sx(0x003459e4);
            sw(c, at_2(c, S1, 0x8), c.g[V1][0]);
            if call_ptr(c, 0x00370018, 0x003459e4).is_err() {
                return;
            }
            c.g[A3][0] = lw(c, r32(c, S2));
            c.g[A1][0] = sx(at_2(c, S1, 0x1e4));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = lh(c, at_2(c, A3, 0x10));
            c.g[V1][0] = lw(c, at_2(c, A3, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00345a00);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00345a00).is_err() {
                return;
            }
            c.g[V0][0] = lw(c, at_2(c, S1, 0x1e4));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x500000;
            if t {
                break 'b_345a78;
            } else {
                c.cycles += 27;
                c.g[A0][0] = 0xd8;
                c.g[RA][0] = sx(0x00345a18);
                c.g[S0][0] = sx(at_2(c, S1, 0x10));
                if call_ptr(c, 0x00317d20, 0x00345a18).is_err() {
                    return;
                }
                sw(c, at_2(c, S1, 0x1e0), c.g[V0][0]);
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0xd8;
                c.g[V0][0] = lw(c, r32(c, S2));
                c.g[A0][0] = lh(c, at_2(c, V0, 0x10));
                c.g[V1][0] = lw(c, at_2(c, V0, 0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00345a38);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00345a38).is_err() {
                    return;
                }
                c.g[A3][0] = lw(c, r32(c, S2));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x40;
                c.g[A0][0] = lh(c, at_2(c, A3, 0x10));
                c.g[V0][0] = lw(c, at_2(c, A3, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00345a54);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00345a54).is_err() {
                    return;
                }
                c.g[A2][0] = lw(c, at_2(c, S1, 0x1e0));
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00345a64);
                c.g[A0][0] = c.g[S3][0];
                if call_ptr(c, 0x003705e0, 0x00345a64).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00345a70);
                c.g[A1][0] = c.g[S2][0];
                if call_ptr(c, 0x00370af8, 0x00345a70).is_err() {
                    return;
                }
                let t = true;
                c.g[V1][0] = lw(c, at_2(c, GP, 0xf08));
                if t {
                    break 'b_345aa8;
                } else {
                    break 'b_345a78;
                }
            }
        }
        c.cycles += 12;
        c.g[V1][0] = sx(at_2(c, S1, 0x10));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe60));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, at_2(c, V0, 0x10));
        c.lqc2(3, at_2(c, V0, 0x20));
        c.lqc2(4, at_2(c, V0, 0x30));
        c.sqc2(1, r32(c, V1));
        c.sqc2(2, at_2(c, V1, 0x10));
        c.sqc2(3, at_2(c, V1, 0x20));
        c.sqc2(4, at_2(c, V1, 0x30));
        sw(c, at_2(c, S1, 0x1e0), 0u64);
        c.g[V1][0] = lw(c, at_2(c, GP, 0xf08));
        break 'b_345aa8;
    }
    c.cycles += 10;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x40);
    c.g[V1][0] = sx(at_2(c, V1, 0x1));
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    sw(c, at_2(c, GP, 0xf08), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0x50));
    c.pc = t;
    return;
}

pub(crate) mod particle_mod {

    /// `+0x14` emit count (> 0: a continuous emitter).
    pub const EMIT_COUNT: u32 = 0x14;

    /// `+0x18` finished.
    pub const DONE: u32 = 0x18;

    /// `+0x30`, `+0x40` the transformed emitter points.
    pub const POINT_A: u32 = 0x30;

    pub const POINT_B: u32 = 0x40;

    /// `+0x60` the particle system (its first float: remaining life).
    pub const SYSTEM: u32 = 0x60;

    /// `+0x240` the system's live particle count.
    pub const LIVE: u32 = 0x240;

    /// `gp-0x2b48/-0x2b44/-0x2b40`: the frame step (1/60) of the three
    /// update calls.
    pub const GP_STEP_LIVE: u32 = 0x2b48;

    pub const GP_STEP_EXPIRED: u32 = 0x2b44;

    pub const GP_STEP_DYING: u32 = 0x2b40;
}

pub(crate) const DYNAMIC_PARTICLE_MODIFIER_TRANSFORM_POINTS: Callee = Callee {
    addr: 0x0034_5ef8,
    f: ee::s11::DynamicParticleModifier_TransformPoints,
};

/// `ParticleSystem_Update(system, a, b, emit, dt)` (0x3710d0, other shard).
pub(crate) const PARTICLE_SYSTEM_UPDATE: u32 = 0x0037_10d0;

/// `ParticleSystem_Update(self + 0x60, self + 0x30, self + 0x40, emit,
/// *(gp - step))` returning to `ret`.
pub(crate) fn particle_update(
    c: &mut Cpu,
    s: u32,
    emit: u64,
    step: u32,
    ret: u32,
) -> crate::call::Flow {
    use particle_mod::*;
    let gp = c.g[GP][0] as u32;
    c.lwc1(12, gp.wrapping_sub(step));
    c.g[A2][0] = sx(s.wrapping_add(POINT_B));
    c.g[A1][0] = sx(s.wrapping_add(POINT_A));
    c.g[A3][0] = emit;
    call_ptr(c, PARTICLE_SYSTEM_UPDATE, ret)
}

/// `void DynamicParticleModifier::Update()` (0x345f90): unless finished,
/// move the emitter points with the instance and update the particle
/// system: a continuous emitter (count > 0) updates while particles live
/// and finishes when none are left; a one-shot one updates with emission
/// on and finishes once its life (`system[0]`) runs out.
pub fn dynamic_particle_modifier_update(c: &mut Cpu) {
    use particle_mod::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let s = c.g[A0][0];
        c.g[S0][0] = s;
        frame.save_fpr(c, 20, 0x20);
        let s = s as u32;
        c.g[V0][0] = c.lw(s.wrapping_add(DONE));
        let finish = 'f: {
            if c.g[V0][0] != 0 {
                break 'f false;
            }
            c.cycles += 5;
            DYNAMIC_PARTICLE_MODIFIER_TRANSFORM_POINTS.call(c, 0x0034_5fb8)?;
            c.g[V0][0] = c.lw(s.wrapping_add(EMIT_COUNT));
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(s.wrapping_add(LIVE));
                c.g[A0][0] = sx(s.wrapping_add(SYSTEM));
                if (c.g[V0][0] as i64) <= 0 {
                    break 'f true;
                }
                c.cycles += 7;
                particle_update(c, s, 0, GP_STEP_LIVE, 0x0034_5fe4)?;
                break 'f false;
            }
            c.lwc1(0, s.wrapping_add(SYSTEM));
            c.cycles += 5;
            c.mtc1(0, 20);
            c.c_lt_s(0, 20);
            c.g[A0][0] = sx(s.wrapping_add(SYSTEM));
            if c.fcc() {
                c.cycles += 7;
                particle_update(c, s, 1, GP_STEP_EXPIRED, 0x0034_6014)?;
                break 'f false;
            }
            c.cycles += 10;
            particle_update(c, s, 1, GP_STEP_DYING, 0x0034_6030)?;
            c.lwc1(0, s.wrapping_add(SYSTEM));
            c.c_le_s(0, 20);
            c.fcc()
        };
        if finish {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(s.wrapping_add(DONE), 1);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DynamicParticleModifier_Render (0x346070)

/// `void DynamicEmitter_Render(DynamicEmitter*, int pass)` (through the
/// function table).
pub(crate) const DYNAMIC_EMITTER_RENDER: u32 = 0x0037_1380;

/// The modifier: its "hidden" word (+0x18) and its embedded emitter (+0x60).
pub(crate) const MODIFIER_HIDDEN: u32 = 0x18;

pub(crate) const MODIFIER_EMITTER: u32 = 0x60;

/// The render pass handed to the emitter.
pub(crate) const MODIFIER_RENDER_PASS: u64 = 7;

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

// ---------------------------------------------------------------------------
// DynamicParticleModifier_TransformPoints (0x345ef8)

/// Modifier: the instance it is attached to, and its node (bone) index.
pub(crate) const DPM_INSTANCE: u32 = 0xc;

pub(crate) const DPM_NODE: u32 = 0x10;

/// Modifier: the two emitter points in node space, and in world space.
pub(crate) const DPM_LOCAL_A: u32 = 0x20;

pub(crate) const DPM_WORLD_A: u32 = 0x30;

pub(crate) const DPM_WORLD_B: u32 = 0x40;

pub(crate) const DPM_LOCAL_B: u32 = 0x50;

pub(crate) const INSTANCE_GET_NODE_MATRIX: Callee = Callee {
    addr: 0x0034_fed8,
    f: ee::s11::Instance_GetNodeMatrix,
};

/// `void DynamicParticleModifier_TransformPoints(DynamicParticleModifier* m)`
/// (0x345ef8): fetch the world matrix of the modifier's node on its
/// instance into a stack matrix, and transform the two node-space points
/// (`+0x20`, `+0x50`) into world space (`+0x30`, `+0x40`) with VU0
/// `mula/madda/madd` (x, y, z, w columns).
pub fn dynamic_particle_modifier_transform_points(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(DPM_INSTANCE)).lw(c);
        c.g[A1][0] = Addr(this.wrapping_add(DPM_NODE)).lw(c);
        INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_5f18)?;
        // One point: vf12 = M * p, through the stack slot at +0x50.
        let transform = |c: &mut Cpu, local: u32| {
            let this = r32(c, S0);
            c.lqc2(8, this.wrapping_add(local));
            c.lqc2(4, frame.at(c, 0));
            c.lqc2(5, frame.at(c, 0x10));
            c.lqc2(6, frame.at(c, 0x20));
            c.lqc2(7, frame.at(c, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, frame.at(c, 0x50));
        };
        transform(c, DPM_LOCAL_A);
        frame.restore_ra(c, 0x60);
        c.lq(2, frame.at(c, 0x50));
        let this = r32(c, S0);
        c.sq(2, this.wrapping_add(DPM_WORLD_A));
        c.sq(2, frame.at(c, 0x40));
        transform(c, DPM_LOCAL_B);
        c.lq(2, frame.at(c, 0x50));
        let this = r32(c, S0);
        c.sq(2, this.wrapping_add(DPM_WORLD_B));
        frame.restore(c, S0, 0x70);
        c.sq(2, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------------- HaloModifier_Render

/// `Instance_GetNodeMatrix` (0x34fed8).
pub(crate) const HALO_INSTANCE_GET_NODE_MATRIX: Callee = Callee {
    addr: 0x0034_fed8,
    f: ee::s11::f_0034fed8,
};

/// `HaloModifier_DrawQuad` (0x2d1d10, cross-shard).
pub(crate) const HALO_DRAW_QUAD: u32 = 0x002d_1d10;

/// HaloModifier: instance (+0xc), texture quad (+0x10), size (+0x20),
/// mode (+0x24), node (+0x28), spin (+0x30).
pub(crate) const HALO_INSTANCE: u32 = 0xc;

pub(crate) const HALO_TEXTURE: u32 = 0x10;

pub(crate) const HALO_SIZE: u32 = 0x20;

pub(crate) const HALO_MODE: u32 = 0x24;

pub(crate) const HALO_NODE: u32 = 0x28;

pub(crate) const HALO_SPIN: u32 = 0x30;

/// `void HaloModifier_Render(HaloModifier* self)` (0x3462a0): draws the
/// halo quad at the translation of its instance node's matrix.
pub fn halo_modifier_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x80);
        frame.save(c, 16, 0x70);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, 17, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        let halo = r32(c, S0);
        c.g[S1][0] = sx(halo.wrapping_add(HALO_TEXTURE));
        c.g[A0][0] = Addr(halo.wrapping_add(HALO_INSTANCE)).lw(c);
        c.g[A1][0] = Addr(halo.wrapping_add(HALO_NODE)).lw(c);
        // The node matrix goes to sp+0; its translation row is at sp+0x30.
        HALO_INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_62c8)?;
        let halo = r32(c, S0);
        c.lq(V0, frame.at(c, 0x30));
        c.g[A1][0] = c.g[S1][0];
        c.lwc1(13, halo.wrapping_add(HALO_SPIN));
        c.g[A0][0] = sx(frame.at(c, 0x40));
        c.g[A2][0] = Addr(halo.wrapping_add(HALO_MODE)).lw(c);
        c.lwc1(12, halo.wrapping_add(HALO_SIZE));
        c.sq(V0, frame.at(c, 0x40));
        call_ptr(c, HALO_DRAW_QUAD, 0x0034_62e8)?;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParticleModifier_Save (0x345bf0).

/// `ParticleModifier_BaseSave(self, stream)`.
pub(crate) const PMOD_BASE_SAVE: Callee = Callee {
    addr: 0x0034_5890,
    f: ee::s11::ParticleModifier_BaseSave,
};

/// `Emitter_Save(emitter, stream)`.
pub(crate) const PMOD_EMITTER_SAVE: u32 = 0x0037_0aa8;

/// Stream vtable slot: `Write(this, const void* data, int size)`.
pub(crate) const PMOD_STREAM_WRITE: u32 = 0x8;

/// `ParticleModifier` field: the particle template (`void*`, 0xd8 bytes).
pub(crate) const PMOD_TEMPLATE: u32 = 0x1e0;

/// `ParticleModifier` field: `int` "uses a shared template" (nothing more
/// is saved when set).
pub(crate) const PMOD_SHARED: u32 = 0x1e4;

/// `ParticleModifier` field: a 0x40-byte block (the transform).
pub(crate) const PMOD_TRANSFORM: u32 = 0x10;

/// `ParticleModifier` field: the emitter (inline).
pub(crate) const PMOD_EMITTER: u32 = 0x50;

/// `void ParticleModifier_Save(ParticleModifier* self, Stream* stream)`
/// (0x345bf0): save the base part, then the "shared" flag (4 bytes); a
/// modifier with its own template also writes the template (0xd8 bytes),
/// its 0x40-byte transform and its emitter.
///
/// Frame 0x30 (s0 +0x20, s1 +0x10, ra +0).
pub fn particle_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S0, stream);
        frame.save_ra(c, 0);
        let this = reg(c, A0);
        set_reg(c, S1, this);
        PMOD_BASE_SAVE.call(c, 0x0034_5c0c)?;
        pmod_stream_write(c, PMOD_SHARED, false, 4, 0x0034_5c28)?;
        let shared = Addr(r32(c, S1)).off(PMOD_SHARED).lw(c);
        set_reg(c, V0, shared);
        if shared == 0 {
            c.cycles += 18;
            pmod_stream_write(c, PMOD_TEMPLATE, true, 0xd8, 0x0034_5c50)?;
            pmod_stream_write(c, PMOD_TRANSFORM, false, 0x40, 0x0034_5c6c)?;
            let emitter = addu(reg(c, S1), PMOD_EMITTER as u64);
            set_reg(c, A0, emitter);
            let stream = reg(c, S0);
            set_reg(c, A1, stream);
            call_ptr(c, PMOD_EMITTER_SAVE, 0x0034_5c78)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `stream->Write(self + field, size)` (or, with `deref`, the pointer
/// stored at `self + field`), with the original's register use: the
/// first write leaves the vtable in v0 and the function in v1, the later
/// ones the other way round.
pub(crate) fn pmod_stream_write(c: &mut Cpu, field: u32, deref: bool, size: u64, ret: u32) -> Flow {
    let stream = reg(c, S0);
    let this = reg(c, S1);
    let vt = Addr(stream as u32).lw(c);
    let data = if deref {
        Addr(this as u32).off(field).lw(c)
    } else {
        addu(this, field as u64)
    };
    set_reg(c, A1, data);
    set_reg(c, A2, size);
    let write = VSlot::read(c, vt, PMOD_STREAM_WRITE);
    if field == PMOD_SHARED {
        set_reg(c, V0, vt);
        set_reg(c, V1, write.func);
    } else {
        set_reg(c, V1, vt);
        set_reg(c, V0, write.func);
    }
    set_reg(c, A0, write.this(stream));
    call_ptr(c, write.func as u32, ret)
}

// ---------------------------------------------------------------------------
// ParticleModifier_BaseSave.

/// `ParticleModifier::type` (the word the base class saves).
pub(crate) const PARTICLE_MODIFIER_TYPE: u32 = 0xc;

/// The stream's "write word" virtual method slot.
pub(crate) const STREAM_VT_WRITE_WORD: u32 = 0x28;

/// `void ParticleModifier_BaseSave(ParticleModifier* self, Stream* s)`
/// (0x345890): the base part of a modifier's save: write the modifier's
/// type word (`+0xc`) through the stream's virtual write (slot 0x28).
pub fn particle_modifier_base_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c); // vtable
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(PARTICLE_MODIFIER_TYPE)).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, STREAM_VT_WRITE_WORD);
        c.g[A0][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[A2][0]);
        call_ptr(c, slot.func as u32, 0x0034_58b4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replay/save stream vtable: `Write(stream, src, size)` at slot 0x8.
pub(crate) const DPM_SAVE_STREAM_WRITE: u32 = 0x8;

/// `DynamicParticleModifier`: saved block at +0x10 (0x50 bytes), "static"
/// flag at +0x18, emitter state pointer at +0x260 (0xd8 bytes), embedded
/// `DynamicEmitter` at +0x60.
pub(crate) const DPM_SAVE_BLOCK: u32 = 0x10;

pub(crate) const DPM_SAVE_BLOCK_SIZE: u64 = 0x50;

pub(crate) const DPM_SAVE_STATIC: u32 = 0x18;

pub(crate) const DPM_SAVE_STATE: u32 = 0x260;

pub(crate) const DPM_SAVE_STATE_SIZE: u64 = 0xd8;

pub(crate) const DPM_SAVE_EMITTER: u32 = 0x60;

/// `stream->Write(src, size)` through the stream's vtable slot 0x8,
/// returning to `back` (v0 = vtable, v1 = method, as the original leaves
/// them).
pub(crate) fn dpm_save_stream_write(c: &mut Cpu, back: u32) -> crate::call::Flow {
    let stream = c.g[S0][0] as u32;
    let vt = Addr(stream).lw(c);
    c.g[V0][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(DPM_SAVE_STREAM_WRITE)).lh(c);
    let write = Addr((vt as u32).wrapping_add(DPM_SAVE_STREAM_WRITE + 4)).lw(c);
    c.g[V1][0] = write;
    c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
    call_ptr(c, write as u32, back)
}

/// `void DynamicParticleModifier_Save(DynamicParticleModifier* self,
/// Stream* stream)` (0x3460a0): the base modifier's save, then the 0x50
/// byte block at +0x10; a non-static modifier also writes its 0xd8 byte
/// state (+0x260) and saves its embedded emitter (`DynamicEmitter_Save`).
pub fn dynamic_particle_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Callee {
            addr: 0x0034_5890,
            f: ee::s11::ParticleModifier_BaseSave,
        }
        .call(c, 0x0034_60bc)?;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_BLOCK));
        c.g[A2][0] = DPM_SAVE_BLOCK_SIZE;
        dpm_save_stream_write(c, 0x0034_60d8)?;
        let is_static = Addr((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_STATIC)).lw(c);
        c.g[V0][0] = is_static;
        if is_static == 0 {
            c.cycles += 11;
            c.g[A2][0] = DPM_SAVE_STATE_SIZE;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_STATE)).lw(c);
            dpm_save_stream_write(c, 0x0034_6100)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_EMITTER));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0037_12b8, 0x0034_610c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
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
    use crate::difftest::compare;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD;
    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn modifier_list_push_front_matches() {
        check(
            ssx3_ee::s11::f_00345700,
            modifier_list_push_front,
            200,
            &|c, seed| {
                let mut r = Rng(0x1011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
                let head = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
                c.bus.write32(DATA, head);
            },
        );
    }

    #[test]
    fn modifier_list_unlink_matches() {
        check(
            ssx3_ee::s11::f_00345760,
            modifier_list_unlink,
            200,
            &|c, seed| {
                let mut r = Rng(0x1012 ^ seed as u64);
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
    fn particle_modifier_base_ctor_matches() {
        check(
            ssx3_ee::s11::f_00345798,
            particle_modifier_base_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0xf04, r.next_u32());
            },
        );
    }

    #[test]
    fn particle_modifier_update_matches() {
        check(
            ee::s11::f_00345b40__generated,
            particle_modifier_update,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x5b40, 0);
                let m = BASE;
                c.bus.write32(m + 0x1e4, (seed % 5 == 0) as u32);
                c.bus.write32(
                    m + 0x50,
                    match seed % 4 {
                        0 => f(-1.0),
                        1 => 0,
                        2 => f(0.5),
                        _ => f(-0.25),
                    },
                );
                // The emitter descriptor: zero burst budget, so Emitter_Update
                // returns without its generation loop.
                for i in 0..8 {
                    c.bus.write32(m + 0x60 + 4 * i, 0);
                }
                for off in [0x180u32, 0x184, 0x188] {
                    c.bus.write32(m + off, 0);
                }
                c.bus.write32(GAME_GP + 0x2a74, BASE + 0x1000);
                c.bus.write32(BASE + 0x1010, 60); // particles per second
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn particle_modifier_render_matches() {
        check(
            ssx3_ee::s11::f_00345bc8__generated,
            particle_modifier_render,
            150,
            &|c, seed| {
                machine(c, seed, 0x345b_c8, 3);
                // The effect submitter (0x3708c0) walks the render context's
                // vtables; a zeroed context makes both calls miss, and the second
                // one returns 1 so it stops after its "already submitted" check.
                c.bus.write32(GAME_GP + 0x2a90, DATA + 0x2000);
                c.bus.write32(LOG_RESULTS, 0);
                c.bus.write32(LOG_RESULTS + 4, 1);
                let m = DATA;
                c.g[A0][0] = sx(m);
                c.bus.write32(m + 0x1e4, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    // ------------------------------------------------------- effects / modifiers

    #[test]
    fn halo_modifier_dtor_matches() {
        check(
            ee::s11::f_00346228__generated,
            halo_modifier_dtor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x346228, 1);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(GAME_GP + 0xf10, 1 + r.next_u32() % 3);
            },
        );
    }

    #[test]
    fn halo_modifier_update_matches() {
        check(
            ssx3_ee::s11::f_00346258__generated,
            halo_modifier_update,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x346258, 0);
                c.g[A0][0] = sx(DATA);
                word(
                    c,
                    DATA + 0x2c,
                    pick(&mut r, &[0.0f32, 1.0, -1.0, 100.0, -100.0, 360.0, -360.0]),
                );
                word(
                    c,
                    DATA + 0x30,
                    pick(&mut r, &[0.0, 0.5, 259.0, 260.0, 359.9, -359.9, -355.0]),
                );
            },
        );
    }

    /// The angle after `ticks` updates at `hz` with one degree per 60 Hz frame.
    fn halo_angle(hz: u32, ticks: u32) -> f32 {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, hz);
        c.bus.write32(DATA + 0x2c, 1);
        c.bus.write32(DATA + 0x30, 0);
        for _ in 0..ticks {
            c.g[A0][0] = sx(DATA);
            halo_modifier_update(&mut c);
        }
        f32::from_bits(c.bus.read32(DATA + 0x30))
    }

    /// The halo's `+0x2c` step is degrees per 60 Hz frame: one second of
    /// rotation is the same angle at any rate.
    #[test]
    fn halo_modifier_update_follows_the_rate() {
        let a60 = halo_angle(60, 60);
        assert_eq!(a60, 60.0);
        assert_eq!(halo_angle(120, 120), a60);
        assert_eq!(halo_angle(240, 240), a60);
    }

    #[test]
    fn particle_modifier_base_ctor_2_matches() {
        check(
            ee::s11::f_003457c8__generated,
            particle_modifier_base_ctor_2,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x3457_c8);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let obj = DATA + 0x300;
                clear(c, obj, 0x20);
                let vt = DATA + 0x400;
                vtable_2(c, vt, &[(0x18, FAKE + 0x10), (0x1c, FAKE + 0x20)]);
                w(c, obj, vt as u32);
                w(c, gp_at(0xf04), r.next_u32());
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(obj);
            },
        );
    }

    #[test]
    fn particle_modifier_base_dtor_matches() {
        check(
            ee::s11::f_00345828__generated,
            particle_modifier_base_dtor,
            40,
            &|c, seed| {
                base_2(c, seed, 0x34_5828);
                c.g[A0][0] = sx(DATA);
                // Even flags: no `operator delete` on test data.
                c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
                c.bus.write32(GAME_GP + 0xf04, seed % 5);
            },
        );
    }

    #[test]
    fn halo_modifier_ctor_2_matches() {
        check(
            ee::s11::f_003461c0__generated,
            halo_modifier_ctor_2,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_61c0, 2);
                let stream = vtable(c, &mut r, &[0x10, 0x18]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(GAME_GP + 0xf04, 3);
                c.bus.write32(GAME_GP + 0xf10, 3);
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    #[test]
    fn particle_modifier_dtor_matches() {
        check(
            ee::s11::f_00345ad0__generated,
            particle_modifier_dtor,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x345a_d0);
                let obj = DATA + 0x1000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let buf = if seed % 2 == 0 { 0 } else { alloc(c, 0x20) };
                c.bus.write32(obj + 0x1e0, buf);
                c.bus.write32(obj, 0);
                c.bus.write32(obj + 4, 0);
                c.bus.write32(GAME_GP + 0xf08, rng_word(seed));
            },
        );
    }

    #[test]
    fn dynamic_particle_modifier_dtor_matches() {
        check(
            ee::s11::f_00345e88__generated,
            dynamic_particle_modifier_dtor,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x345e_88);
                let obj = DATA + 0x1000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let buf = if seed % 2 == 0 { 0 } else { alloc(c, 0x20) };
                c.bus.write32(obj + 0x260, buf);
                c.bus.write32(obj, 0);
                c.bus.write32(obj + 4, 0);
                c.bus.write32(GAME_GP + 0xf0c, rng_word(seed));
            },
        );
    }

    pub(crate) fn rng_word(seed: u32) -> u32 {
        0x9e37_79b9u32.wrapping_mul(seed.wrapping_add(1))
    }

    pub(crate) const EFFECT: u32 = DATA + 0x1_0000;

    pub(crate) const EFFECT_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn particle_modifier_ctor_2_matches() {
        check_all(
            ee::s11::f_003459a8__generated,
            particle_modifier_ctor_2,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x3459_a8, 10);
                c.g[A0][0] = sx(EFFECT);
                c.g[A1][0] = sx(EFFECT_STREAM);
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                c.bus.write32(EFFECT_STREAM, vt);
                c.bus.write32(EFFECT + 0x1e4, (seed % 2) as u32);
            },
        );
    }
    use super::*;
    use crate::difftest::{elf_image, fresh, require_paths};
    use ssx3_ee_rt::Cpu;

    /// DynamicParticleModifier_Update: finished; continuous emitter with
    /// and without live particles; one-shot emitter with life left, run
    /// out, or running out now. The instance has no owner and node -1 (the
    /// node matrix is the instance's own), the particle system has no
    /// particle array (its update takes the short path).
    #[test]
    fn dynamic_particle_modifier_update_matches() {
        use super::particle_mod::*;
        check(
            ee::s11::f_00345f90__generated,
            dynamic_particle_modifier_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5f90, 0);
                c.pc = 0x0034_5f90;
                let s = DATA + 0x1000;
                let inst = DATA + 0x2000;
                let skel = DATA + 0x2400;
                for off in (0..0x260).step_by(4) {
                    c.bus.write32(s + off, r.f32(10.0));
                }
                for off in (0..0x90).step_by(4) {
                    c.bus.write32(inst + off, r.f32(4.0));
                }
                c.bus.write32(inst + 0xc, 0);
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, DATA + 0x2800);
                c.bus.write32(s + 0xc, inst);
                c.bus.write32(s + 0x10, u32::MAX);
                c.bus.write32(s + DONE, if seed % 7 == 0 { 1 } else { 0 });
                c.bus.write32(
                    s + EMIT_COUNT,
                    [0, 1, 5, 0xffff_ffff][(seed / 7 % 4) as usize],
                );
                c.bus
                    .write32(s + LIVE, [0, 3, 0xffff_fffe][(r.next_u32() % 3) as usize]);
                // Remaining life: positive, zero or negative.
                let life = match r.next_u32() % 3 {
                    0 => 0,
                    1 => r.f32(1.0) & 0x7fff_ffff,
                    _ => r.f32(1.0) | 0x8000_0000,
                };
                c.bus.write32(s + SYSTEM, life);
                c.bus.write32(s + SYSTEM + 0x174, 0);
                for k in [GP_STEP_LIVE, GP_STEP_EXPIRED, GP_STEP_DYING] {
                    c.bus.write32(GAME_GP - k, r.f32(1.0));
                }
                c.g[A0][0] = s as u64;
            },
        );
    }
    use super::*;
    use super::*;

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

    /// `DynamicParticleModifier_TransformPoints`: an instance without an
    /// owner, node -1 (the world matrix) or a node with 0..2 parents (the
    /// real `Instance_GetNodeMatrix` chains them), random points.
    #[test]
    fn dynamic_particle_modifier_transform_points_matches() {
        const INST: u32 = DATA;
        const SKEL: u32 = DATA + 0x400;
        const NODES: u32 = DATA + 0x500;
        const MATS: u32 = DATA + 0x1000;
        const MODIFIER: u32 = DATA + 0x2000;
        check(
            ee::s11::f_00345ef8__generated,
            dynamic_particle_modifier_transform_points,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x345ef8, 0);
                c.pc = 0x0034_5ef8;
                for k in 0..0x10 {
                    c.bus.write32(INST + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(INST + 0xc, 0);
                c.bus.write32(INST + 0x84, r.f32(2.0));
                c.bus.write32(INST + 0x80, SKEL);
                c.bus.write32(SKEL + 8, NODES);
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(NODES + 0x10 * n, parent);
                    c.bus.write32(NODES + 0x10 * n + 0xc, MATS + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(MATS + 4 * k, r.f32(4.0));
                }
                fill(c, &mut r, MODIFIER, 0x18);
                for k in 0..4 {
                    c.bus.write32(MODIFIER + DPM_LOCAL_A + 4 * k, r.f32(10.0));
                    c.bus.write32(MODIFIER + DPM_LOCAL_B + 4 * k, r.edge_f32());
                }
                let node = match seed % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                c.bus.write32(MODIFIER + DPM_INSTANCE, INST);
                c.bus.write32(MODIFIER + DPM_NODE, node);
                c.g[A0][0] = sx(MODIFIER);
            },
        );
    }
    use super::*;

    /// `HaloModifier_Render`: the real `Instance_GetNodeMatrix` (no owner;
    /// node -1, the root or a node with one or two parents; the instance
    /// node test's fixture) and the real `HaloModifier_DrawQuad`, culled
    /// (a fake visibility method) or visible (a real `return 0`), modes
    /// 0..5, spin or not (the draw-quad test's fixture; its drawer methods
    /// are fakes, some of which unwind).
    #[test]
    fn halo_modifier_render_matches() {
        use crate::difftest::fakes::FAKE;
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(
            ssx3_ee::s11::f_003462a0__generated,
            halo_modifier_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3462_a0, 3);
                c.pc = 0x0034_62a0;
                // The instance, its skeleton nodes and matrices.
                let inst = DATA;
                let skel = DATA + 0x400;
                let nodes = DATA + 0x500;
                let mats = DATA + 0x1000;
                for k in 0..0x10 {
                    c.bus.write32(inst + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(inst + 0xc, 0);
                c.bus.write32(inst + 0x84, r.f32(2.0));
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, nodes);
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(nodes + 0x10 * n, parent);
                    c.bus.write32(nodes + 0x10 * n + 0xc, mats + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(mats + 4 * k, r.f32(4.0));
                }
                let node = match r.next_u32() % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                // The world (visibility method) and the draw context.
                let (world, wvt, ctx, cvt) = (
                    DATA + 0x10000,
                    DATA + 0x14000,
                    DATA + 0x18000,
                    DATA + 0x1c000,
                );
                let cur = DATA + 0x2000 + 4 * (seed % 4);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), world);
                c.bus.write32(world + 0x10d8, wvt);
                c.bus.write16(wvt + 0x2d8, (r.next_u32() & 0x7c) as u16);
                c.bus
                    .write32(wvt + 0x2dc, if seed % 5 == 0 { FAKE } else { RETURN_ZERO });
                c.bus.write32(GAME_GP + 0x2a90, ctx);
                fill(c, &mut r, ctx + 0xf50, 0x30);
                c.bus.write32(ctx + 0xe84, cur);
                c.bus.write32(ctx + 0x10d8, cvt);
                for (k, off) in [0x270u32, 0x278].into_iter().enumerate() {
                    c.bus.write16(cvt + off, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(cvt + off + 4, FAKE + 0x10 * (k as u32 + 1));
                }
                fill(c, &mut r, cur - 0x20, 0x20);
                fill(c, &mut r, 0x0050_1420, 6);
                c.bus.write32(GAME_GP.wrapping_sub(0x3f98), r.f32(1.0));
                // The halo.
                let halo = DATA + 0x8000;
                c.bus.write32(halo + HALO_INSTANCE, inst);
                fill(c, &mut r, halo + HALO_TEXTURE, 4);
                c.bus.write32(halo + HALO_SIZE, r.f32(50.0));
                c.bus.write32(halo + HALO_MODE, (seed / 2) % 6);
                c.bus.write32(halo + HALO_NODE, node);
                c.bus
                    .write32(halo + HALO_SPIN, if seed % 3 == 0 { 0 } else { r.f32(6.0) });
                c.g[A0][0] = sx(halo);
            },
        );
    }
    use super::*;
    use crate::game::lib::snd::tests as snd_tests;

    /// `ParticleModifier_Save`: the real `ParticleModifier_BaseSave` and
    /// `Emitter_Save`, the stream's writers fakes (some calls unwind);
    /// shared-template flag set or clear.
    #[test]
    fn particle_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::fill;
        covered(
            ee::s11::f_00345bf0__generated,
            particle_modifier_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_5bf0, 5);
                c.pc = 0x0034_5bf0;
                let this = DATA;
                let stream = DATA + 0x400;
                fill(c, &mut r, this, 0x80);
                c.bus.write32(
                    this + PMOD_SHARED,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let vt = vtable(c, &mut r, &[PMOD_STREAM_WRITE, 0x28]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }
    use super::*;

    /// ParticleModifier_BaseSave: a stream whose vtable slot 0x28 is a fake
    /// (random `this` delta) that unwinds on some seeds.
    #[test]
    fn particle_modifier_base_save_matches() {
        use crate::difftest::fakes::{machine, vtable};
        check(
            ee::s11::f_00345890__generated,
            particle_modifier_base_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_5890, 1);
                c.pc = 0x0034_5890;
                let modifier = DATA + 0x10;
                let stream = DATA + 0x200;
                c.bus
                    .write32(modifier + PARTICLE_MODIFIER_TYPE, r.next_u32());
                let vt = vtable(c, &mut r, &[STREAM_VT_WRITE_WORD]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(modifier);
                c.g[A1][0] = sx(stream);
            },
        );
    }
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::RA as SENTINEL;

    /// A dynamic particle modifier (static or not) and a stream whose
    /// vtable slots 0x8 (`Write`) and 0x28 (`WriteInt`, the base save) are
    /// fakes; the embedded emitter holds random words.
    pub(crate) fn dpm_save_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::fill;
        let is_static = seed % 3 == 0;
        let calls = if is_static { 2 } else { 4 };
        let mut r = machine(c, seed, 0x3460_a000, calls);
        let modifier = DATA;
        let stream = DATA + 0x1000;
        let vt = DATA + 0x1100;
        fill(c, &mut r, modifier, 0x110);
        c.bus.write32(
            modifier + DPM_SAVE_STATIC,
            if is_static { 1 + r.next_u32() % 9 } else { 0 },
        );
        c.bus.write32(modifier + DPM_SAVE_STATE, DATA + 0x2000);
        c.bus.write32(stream, vt);
        for (i, slot) in [DPM_SAVE_STREAM_WRITE, 0x28].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + slot, delta as u16);
            c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
        }
        c.g[A0][0] = sx(modifier);
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0034_60a0;
    }

    /// `DynamicParticleModifier_Save`: static and dynamic modifiers, fakes
    /// unwinding at any call.
    #[test]
    fn dynamic_particle_modifier_save_matches() {
        check(
            ee::s11::f_003460a0__generated,
            dynamic_particle_modifier_save,
            300,
            &dpm_save_fixture,
        );
    }
}
