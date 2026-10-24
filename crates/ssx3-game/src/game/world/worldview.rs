//! `src/world/worldview.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::bx::bxstring2::vdot4;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::object::splinemodifier::lwc1_r;
pub(crate) use crate::game::object::splinemodifier::swc1_r;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::fr;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
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
        0x003a_9820,
        c_world_cache_streamer_get_section,
        "cWorldCache_StreamerGetSection",
    ),
    (
        0x003a_9858,
        c_world_cache_streamer_is_section_resident,
        "cWorldCache_StreamerIsSectionResident",
    ),
    (
        0x003a_9890,
        c_world_cache_streamer_is_section_unloaded,
        "cWorldCache_StreamerIsSectionUnloaded",
    ),
    (
        0x003a_99d8,
        c_world_cache_streamer_activate_section,
        "cWorldCache_StreamerActivateSection",
    ),
    (
        0x003a_9a98,
        c_world_cache_streamer_get_hull,
        "cWorldCache_StreamerGetHull",
    ),
    (
        0x003a_9ab0,
        c_world_cache_streamer_get_hull_count,
        "cWorldCache_StreamerGetHullCount",
    ),
    (
        0x003a_a028,
        world_cache_streamer_update_view_node,
        "cWorldCache_StreamerUpdateViewNode",
    ),
    (
        0x003a_a3f0,
        c_ps2_chunk_resolve_base_v05,
        "cPS2ChunkResolveBase_v05",
    ),
    (0x003a_a438, c_ps2_chunk_resolve_v06, "cPS2ChunkResolve_v06"),
    (0x003a_a520, cps2_chunk_resolve_v07, "cPS2ChunkResolve_v07"),
    (0x003a_a5c8, ps2_chunk_resolve_v14, "cPS2ChunkResolve_v14"),
    (0x003a_a608, c_ps2_chunk_resolve_v15, "cPS2ChunkResolve_v15"),
    (0x003a_a6e8, cps2_chunk_resolve_v16, "cPS2ChunkResolve_v16"),
    (0x003a_a700, cps2_chunk_resolve_v17, "cPS2ChunkResolve_v17"),
    (0x003a_a758, cps2_chunk_resolve_v09, "cPS2ChunkResolve_v09"),
    (0x003a_a780, cps2_chunk_resolve_v10, "cPS2ChunkResolve_v10"),
    (0x003a_a830, cps2_chunk_resolve_v11, "cPS2ChunkResolve_v11"),
    (0x003a_a8d8, cps2_chunk_resolve_v12, "cPS2ChunkResolve_v12"),
    (
        0x003a_a8f0,
        cps2_chunk_resolve_base_v13,
        "cPS2ChunkResolveBase_v13",
    ),
    (0x003a_a960, cps2_chunk_resolve_v18, "cPS2ChunkResolve_v18"),
    (0x003a_abd8, cps2_chunk_resolve_v20, "cPS2ChunkResolve_v20"),
    (0x003a_ac50, cps2_chunk_resolve_v21, "cPS2ChunkResolve_v21"),
    (0x003a_aca8, cps2_chunk_resolve_v22, "cPS2ChunkResolve_v22"),
    (
        0x003a_ad98,
        c_ps2_chunk_resolve_create,
        "cPS2ChunkResolve_Create",
    ),
    (
        0x003a_ade8,
        c_ps2_chunk_resolve_dtor,
        "cPS2ChunkResolve_dtor",
    ),
    (
        0x003a_ae40,
        cps2_chunk_resolve_init,
        "cPS2ChunkResolve_Init",
    ),
    (0x003a_be40, cps2chunkresolve_v05, "cPS2ChunkResolve_v05"),
    (0x003a_c2a8, cps2_chunk_resolve_v13, "cPS2ChunkResolve_v13"),
    (
        0x003a_c358,
        cps2chunkresolve_relocate_instance_dma,
        "cPS2ChunkResolve_RelocateInstanceDma",
    ),
    (0x003a_c7c8, cw_script_cache_ctor, "cWScriptCache_ctor"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET as UI_LIST_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::render::particle::callee::ADD_DIRECTIONAL;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BLEND_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::COMPUTE_COLOUR_FACTOR;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::DELETE;
    pub(crate) use crate::game::render::particle::callee::DYNAMIC_EMITTER_SPAWN;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::FONT_DRAW_TEXT;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE0;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE1;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE2;
    pub(crate) use crate::game::render::particle::callee::GET_LIGHTING28;
    pub(crate) use crate::game::render::particle::callee::HUD_ALIGN_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_PANEL_SEGMENT;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_SPRITE;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_COLOR;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_SIZE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_ADD;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::IS_HULL_RESIDENT;
    pub(crate) use crate::game::render::particle::callee::LERP_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POINT_BOX_DISTANCE;
    pub(crate) use crate::game::render::particle::callee::POINT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RANDOM;
    pub(crate) use crate::game::render::particle::callee::REQUEST_HULL;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SAMPLE_SURFACE_COLOURS;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SECTION_GET_EVENT_WORLD_SECTION;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_INSERT;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_MOVE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_REMOVE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_KEY_FROM_AABB;
    pub(crate) use crate::game::render::particle::callee::SPOT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::STREAMER_UPDATE_VIEW_NODE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
}

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {
    pub(crate) use crate::game::main::game::view::BOX;
    pub(crate) use crate::game::main::game::view::EYE;
    pub(crate) use crate::game::main::game::view::HULL_DIST;
    pub(crate) use crate::game::main::game::view::HULL_RANGE;
    pub(crate) use crate::game::main::game::view::LEFT_POINT;
    pub(crate) use crate::game::main::game::view::NEAREST;
    pub(crate) use crate::game::main::game::view::RIGHT_POINT;
    pub(crate) use crate::game::main::game::view::STRIDE;
}

/// Streamer tree node fields.
pub(crate) mod node {

    pub const BOX_MIN: u32 = 0x00;

    pub const BOX_MAX: u32 = 0x10;

    pub const LEFT_PLANE: u32 = 0x20;

    pub const RIGHT_PLANE: u32 = 0x30;

    pub const LEFT: u32 = 0x50;

    pub const RIGHT: u32 = 0x54;

    pub const HULL: u32 = 0x58;
}

// ---------------------------------------------------------------------------
// World cache streamer sections.

/// `Section* cWorldCache_StreamerGetSection(Streamer* self, int index)`
/// (0x3a9820): look the section up in the table at self+8; records are 0x58
/// bytes.
pub fn c_world_cache_streamer_get_section(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x58;
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = self_.off(0x8).lw(c);
            c.mult(0, V0, A1, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionResident(Streamer* self, int index)`
/// (0x3a9858): true when the section's state at +0x14 is 3 or 4.
pub fn c_world_cache_streamer_is_section_resident(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 5;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(5));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionUnloaded(Streamer* self, int index)`
/// (0x3a9890): true when the section's state at +0x14 is zero.
pub fn c_world_cache_streamer_is_section_unloaded(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Hull* cWorldCache_StreamerGetHull(Streamer* self, int index)`
/// (0x3a9a98): base of the hull array at self+0x10 plus index*0x44.
pub fn c_world_cache_streamer_get_hull(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = 0x44;
    c.g[V0][0] = Addr(self_.wrapping_add(0x10)).lw(c);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `int cWorldCache_StreamerGetHullCount(Streamer* self)` (0x3a9ab0): hull
/// count at the streamer's owner + 0x10.
pub fn c_world_cache_streamer_get_hull_count(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(self_.wrapping_add(0x4)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    abi::ret(c);
}

/// `v1 = view * 0x50` (`mult`, LO/HI as the original), `v0 = wc + v1`:
/// the view record (v0 = 0x50 on entry).
#[inline(always)]
pub(crate) fn view_record(c: &mut Cpu) -> u32 {
    c.mult(0, V1, S3, V0);
    c.g[V0][0] = add32(c.g[V1][0], c.g[S2][0]);
    r32(c, V0)
}

/// `void cWorldCache_StreamerUpdateViewNode(cWorldCache* wc, Node* node,
/// int view, int store_dist /*a3*/)` (0x3aa028): if the node's box
/// overlaps the view's box, request its hull with a priority from the
/// eye's distance to the node box (inside: a fixed priority; hull not
/// resident yet: the distance is not kept), keep the nearest distance per
/// view, and recurse into the children whose split plane faces the view.
pub fn world_cache_streamer_update_view_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = view::STRIDE as u64;
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.mult(0, V1, S3, V0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = add32(c.g[V1][0], c.g[S2][0]);
        frame.save_fpr(c, 23, 0x78);
        c.g[V0][0] = add32(c.g[V0][0], view::BOX as u64);
        frame.save_fpr(c, 22, 0x70);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let vbox = r32(c, V0);
        let nd = r32(c, S1);
        // Box overlap: view.min <= node.max and node.min <= view.max per
        // axis (x, then y, then z).
        c.g[V1][0] = 0;
        let mut overlap = true;
        for (i, (f1_at, f0_at, rev)) in [
            (p(nd, node::BOX_MAX), vbox, false),
            (p(vbox, 0x10), p(nd, node::BOX_MIN), false),
            (p(vbox, 4), p(nd, node::BOX_MAX + 4), true),
            (p(vbox, 0x14), p(nd, node::BOX_MIN + 4), false),
            (p(vbox, 8), p(nd, node::BOX_MAX + 8), true),
            (p(vbox, 0x18), p(nd, node::BOX_MIN + 8), false),
        ]
        .into_iter()
        .enumerate()
        {
            if i > 0 {
                c.cycles += if i == 5 { 6 } else { 5 };
            }
            let f1 = ldf(c, 1, f1_at);
            let f0 = ldf(c, 0, f0_at);
            let holds = if rev { le(c, f1, f0) } else { le(c, f0, f1) };
            if !holds {
                overlap = false;
                break;
            }
        }
        if overlap {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x50);
        if c.g[V1][0] != 0 && streamer_visit(c, frame)? {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x78);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// An overlapping node: hull request, then the children. `Ok(false)` when
/// the original leaves early (eye beyond the view's hull range: neither
/// request nor children; s0 already restored).
pub(crate) fn streamer_visit(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    let nd = r32(c, S1);
    let wc = r32(c, S2);
    c.cycles += 3;
    c.g[V0][0] = c.lw(p(nd, node::HULL));
    let has_hull = c.g[V0][0] != 0;
    c.g[S0][0] = view::STRIDE as u64;
    if has_hull {
        // dist = PointBoxDistance(wc, eye, box)
        c.cycles += 14;
        c.g[A0][0] = c.g[S2][0];
        c.mult(0, S0, S3, S0);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = sx(p(nd, node::BOX_MAX));
        setf(c, 22, FpuF32::ZERO);
        c.g[A1][0] = add32(c.g[S0][0], c.g[S2][0]);
        c.g[A1][0] = add32(c.g[A1][0], view::EYE as u64);
        callee::POINT_BOX_DISTANCE.call(c, 0x003a_a128)?;
        c.mov_s(20, 0);
        let dist = fr(c, 20);
        c.g[V0][0] = add32(c.g[S2][0], c.g[S0][0]);
        let rec = r32(c, V0);
        let inside = lt(c, dist, fr(c, 22));
        ldf(c, 21, p(rec, view::HULL_DIST));
        if inside {
            c.cycles += 7;
            hull_id(c, nd);
            ldf(c, 23, gp(c, -0x23c0));
            callee::IS_HULL_RESIDENT.call(c, 0x003a_a154)?;
            if c.g[V0][0] == 0 {
                c.mov_s(21, 22);
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 5;
            let range = ldf(c, 0, p(rec, view::HULL_RANGE));
            let beyond = lt(c, range, dist);
            frame.restore(c, S0, 0x50);
            if beyond {
                return Ok(false);
            }
            // priority = (1 - dist / range) * scale
            c.cycles += 8;
            let ratio = dist.div(range);
            setf(c, 0, ratio);
            let one = fconst(c, 1, 0x3f80_0000);
            let scale = ldf(c, 2, gp(c, -0x23bc));
            let nearer = lt(c, dist, fr(c, 21));
            let inv = one - ratio;
            setf(c, 1, inv);
            setf(c, 23, inv * scale);
            if nearer {
                c.cycles += 6;
                hull_id(c, nd);
                callee::IS_HULL_RESIDENT.call(c, 0x003a_a1a8)?;
                if c.g[V0][0] == 0 {
                    c.mov_s(21, 20);
                }
            }
        }
        // Store the distance (if asked) and keep the nearest.
        c.cycles += 2;
        let store = c.g[S4][0] != 0;
        c.g[V0][0] = view::STRIDE as u64;
        if store {
            c.cycles += 3;
            let rec = view_record(c);
            c.swc1(21, p(rec, view::HULL_DIST));
        }
        c.cycles += 8;
        c.g[V0][0] = view::STRIDE as u64;
        let rec = view_record(c);
        let nearest = ldf(c, 0, p(rec, view::NEAREST));
        if lt(c, fr(c, 21), nearest) {
            c.swc1(21, p(rec, view::NEAREST));
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(p(nd, node::HULL));
        c.mov_s(12, 23);
        c.g[A0][0] = c.lw(wc);
        c.g[A1][0] = c.lhu(at(c, V0, 2));
        callee::REQUEST_HULL.call(c, 0x003a_a1f8)?;
    }
    // Children whose plane (dotted with the view point) is negative.
    for (child, plane, point, ret) in [
        (
            node::LEFT,
            node::LEFT_PLANE,
            view::LEFT_POINT,
            0x003a_a250u32,
        ),
        (
            node::RIGHT,
            node::RIGHT_PLANE,
            view::RIGHT_POINT,
            0x003a_a2a8,
        ),
    ] {
        c.cycles += 3;
        c.g[A1][0] = c.lw(p(nd, child));
        let present = c.g[A1][0] != 0;
        c.g[V0][0] = view::STRIDE as u64;
        if !present {
            continue;
        }
        c.cycles += 16;
        setf(c, 1, FpuF32::ZERO);
        let rec = view_record(c);
        let side = vdot4(c, p(nd, plane), p(rec, point), V1);
        c.mtc1(V1, 0);
        let behind = lt(c, side, FpuF32::ZERO);
        c.g[A0][0] = c.g[S2][0];
        if behind {
            c.cycles += 3;
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S4][0];
            callee::STREAMER_UPDATE_VIEW_NODE.call(c, ret)?;
        }
    }
    Ok(true)
}

/// `v0 = node->hull; a0 = wc->cache; a1 = hull->id (lhu +2)`: the
/// arguments of the hull calls.
#[inline(always)]
pub(crate) fn hull_id(c: &mut Cpu, nd: u32) {
    c.g[V0][0] = c.lw(p(nd, node::HULL));
    c.g[A0][0] = c.lw(r32(c, S2));
    c.g[A1][0] = c.lhu(at(c, V0, 2));
}

/// `void* cWScriptCache_ctor(cWScriptCache* self, int count)` (0x3ac7c8):
/// stash the world, zero the list and allocate `count` script entries.
pub fn cw_script_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let world = c.g[A1][0];
        Addr(this).sw(c, world);
        Addr(this).off(4).sw(c, 0);
        c.g[A1][0] = 1;
        callee::CW_SCRIPT_CACHE_ALLOC.call(c, 0x003a_c7e8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_v14(void* self, void* chunk, int a2)` (0x3AA5C8): make
/// the chunk's relative pointer at +0x8 absolute and let the owner's virtual
/// handler at +0x7C resolve it; returns 1.
pub fn ps2_chunk_resolve_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let this = r(c, A0);
        let chunk = c.g[A1][0];
        c.g[V1][0] = chunk;
        frame.save_ra(c, 0);
        let offset = c.lw_r_2(V0, (chunk as u32).wrapping_add(8));
        let absolute = sx((offset as u32).wrapping_add(chunk as u32));
        c.g[V0][0] = absolute;
        Addr((chunk as u32).wrapping_add(8)).sw(c, absolute);
        let owner = c.lw_r_2(A2, this.wrapping_add(8));
        let delta = c.lh_r(V1, (owner as u32).wrapping_add(0x78));
        let target = c.lw_r_2(V0, (owner as u32).wrapping_add(0x7c));
        let t = target as u32;
        let a0 = sx(this.wrapping_add(delta as u32));
        c.g[A0][0] = a0;
        call_ptr(c, t, 0x003a_a5f4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolveBase_v05(self, chunk)` (0x3aa3f0): relocate the
/// chunk's two internal pointers to absolute addresses, hand the chunk to the
/// resolver's slot 0x30/0x34 and report success.
pub fn c_ps2_chunk_resolve_base_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let chunk = c.g[V0][0] as u32;
        c.g[A2][0] = Addr(chunk).off(8).lw(c);
        c.g[V1][0] = Addr(chunk).off(0xc).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(chunk));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(chunk));
        Addr(chunk).off(8).sw(c, c.g[A2][0]);
        Addr(chunk).off(0xc).sw(c, c.g[V1][0]);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let resolver = c.g[A2][0] as u32;
        c.g[V1][0] = Addr(resolver).off(0x30).lh(c);
        c.g[V0][0] = Addr(resolver).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003a_a428);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x003a_a428)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2ChunkResolve_Create(void)` (0x3aad98): allocate and construct a
/// chunk resolver.
pub fn c_ps2_chunk_resolve_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x49_4f70; // D_00494F70
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x003a_adbc);
        c.g[A0][0] = 0x14;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x003a_adbc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::CPS2_CHUNK_RESOLVE_CTOR.call(c, 0x003a_adc8)?;
        c.g[V1][0] = 0x49_5090; // D_00495090
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr(r32(c, S0)).off(8).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve_dtor(cPS2ChunkResolve* self, int flags)` (0x3aade8):
/// two-stage teardown with an optional free.
pub fn c_ps2_chunk_resolve_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(0x0049_5090);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003a_ae10);
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        callee::CPS2_CHUNK_RESOLVE_DTOR_STUB.call(c, 0x003a_ae10)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[S0][0] &= 0x1;
        c.g[V0][0] = sx(0x0049_5150);
        let keep = c.g[S0][0] == 0;
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_ae2c);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE_2.call(c, 0x003a_ae2c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` (a 32-bit address add).
#[inline(always)]
pub(crate) fn p_2(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

/// `int cPS2ChunkResolve_v16(void* self, ResolveNode* node)` (0x003aa6e8):
/// the node's `+0x20` word becomes its own address (a self-relative base).
pub fn cps2_chunk_resolve_v16(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    c.g[V1][0] = Addr(p_2(node, 0x20)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(node));
    c.sw(p_2(node, 0x20), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v12(void* self, ResolveNode* node, int v)`
/// (0x003aa8d8): `node->base += v`.
pub fn cps2_chunk_resolve_v12(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let v = abi::arg(c, 2);
    c.g[V1][0] = Addr(p_2(node, 0x4)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(v));
    c.sw(p_2(node, 0x4), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_Init(cPS2ChunkResolve* self, int n)` (0x003aae40):
/// remember the world and the chunk count.
pub fn cps2_chunk_resolve_init(c: &mut Cpu) {
    c.cycles += 8;
    let self_ = abi::arg(c, 0);
    let n = abi::arg(c, 1);
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8)); // the world
    c.sw(p_2(self_, 0xc), n as u64);
    c.sw(self_, c.g[A2][0]);
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2a));
    c.sw(p_2(self_, 0x10), c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve: the per-kind hash lookups.

/// `int cPS2ChunkResolve_v17(void* self, ResolveNode* node)` (0x003aa700):
/// resolve the hash at `node+0x64` through the `+0x24` table, store the
/// result and clear `node+0x84`.
pub fn cps2_chunk_resolve_v17(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p_2(node, 0x64)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0;
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 4;
    c.sw(p_2(node, 0x64), c.g[V0][0]);
    c.g[V0][0] = 1;
    c.sw(p_2(node, 0x84), 0);
    abi::ret(c);
}

/// `int cPS2ChunkResolveBase_v13(void* self, ResolveNode* node)`
/// (0x003aa8f0): resolve `node+0x80` through the `+0x14` table, then seed
/// `node+0x8c`/`+0x94` from the resolved structure.
pub fn cps2_chunk_resolve_base_v13(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p_2(node, 0x80)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] == 0 {
        // No section: the node keeps the null pointer (v1 = 0).
        c.cycles += 2;
        c.g[V1][0] = 0;
    } else {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 6;
    c.sw(p_2(node, 0x80), c.g[V1][0]);
    c.g[V0][0] = 1;
    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw(p_2(node, 0x8c), 0);
    c.sw(p_2(node, 0x94), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v09(void* self, ResolveNode* node)` (0x003aa758):
/// point `node+0x10` at `node+0x14` (or 0 when it was -1).
pub fn cps2_chunk_resolve_v09(c: &mut Cpu) {
    c.cycles += 4;
    let node = abi::arg(c, 1);
    c.g[V0][0] = Addr(p_2(node, 0x10)).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    let unset = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = sx(p_2(node, 0x14));
    if unset {
        c.cycles += 2;
        c.sw(p_2(node, 0x10), 0);
    } else {
        c.cycles += 1;
        c.sw(p_2(node, 0x10), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v10(void* self, int n, u32* data)` (0x003aa780):
/// resolve every hash of `n` counted arrays in place; `data` advances by
/// the array's own length.
pub fn cps2_chunk_resolve_v10(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T3][0] = c.g[A0][0]; // self
    let n = c.g[A1][0];
    c.g[A0][0] = 0; // g
    if (n as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // *data
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        let count = c.g[V0][0];
        c.g[T1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                // h = *hp; look it up; *hp = res
                c.cycles += 10;
                c.g[V1][0] = c.lw(c.g[T3][0] as u32); // *(self)
                c.g[A3][0] = c.lw(c.g[T0][0] as u32); // h
                c.g[A0][0] = c.lw(c.g[V1][0] as u32); // *(*(self))
                c.g[V0][0] = c.g[A3][0] & 0xff;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
                let h = c.g[A3][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = sx((h as u32) >> 8);
                let mut res = 0u64;
                if missing {
                    c.cycles += 1;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        res = c.g[V0][0];
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = res;
                c.sw(c.g[T0][0] as u32, c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_a7a8);
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[T2][0];
        c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a790);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v18(void* self, List* list)` (0x003aa960): rebuild
/// the ring indices (`prev`/`next` at +0x60/+0x64, owner at +0x68) of every
/// node of the list at +0x30.
pub fn cps2_chunk_resolve_v18(c: &mut Cpu) {
    c.cycles += 7;
    let list = abi::arg(c, 1);
    c.g[V0][0] = Addr(p_2(list, 0x20)).lw(c);
    c.g[T5][0] = sx(p_2(list, 0x30)); // nodes
    c.g[T1][0] = 1; // next index
    c.sw(p_2(list, 0x24), c.g[T5][0]);
    c.g[T0][0] = 0; // i
    let count = c.g[V0][0];
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1)); // prev index
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T3][0] = 0x90; // node stride
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = c.g[T5][0];
    loop {
        // node->prev: -1 becomes 0, else &nodes[prev]
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x60));
        let unset = c.g[V0][0] == c.g[T2][0];
        c.mult(0, 3, 7, 11); // prev * 0x90 -> v1
        if unset {
            c.cycles += 1;
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), c.g[V0][0]);
        }
        // node->next: the same for +0x64
        c.cycles += 6;
        c.g[A0][0] = 0x90;
        c.mult(0, 2, 8, 4); // i * 0x90 -> v0
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
        c.mult(0, 12, 9, 4); // next * 0x90 -> t4
        if c.g[V0][0] == c.g[T2][0] {
            c.cycles += 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x68), c.g[A1][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A3][0] = 0;
        }
        if c.g[A0][0] == 0 {
            c.g[T1][0] = 0;
        }
        c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x90));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a988);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v21(void* self, ResolveNode* node)` (0x003aac50):
/// clamp the resolved object's value at +0x1c to 5000 while it is still
/// initialising (0 < state < 3).
pub fn cps2_chunk_resolve_v21(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p_2(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(p_2(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.lwc1(0, p_2(node, 0x1c));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x23b8)); // 5000.0f
    c.c_lt_s(1, 0); // 5000 < value
    if c.fcc() {
        c.swc1(1, p_2(node, 0x1c));
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v22(void* self, ResolveNode* node)` (0x003aaca8):
/// true while the object is initialising (state 1 or 2) and not flagged.
pub fn cps2_chunk_resolve_v22(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p_2(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p_2(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
    let below3 = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if below3 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        if (c.g[A1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cWorldCache_StreamerActivateSection(cWorldCache* cache, int i)`
/// (0x3a99d8): move section `i`'s streamer state from 5 (requested) to 6
/// (active), flagging its objects; returns 1 when active.
pub fn c_world_cache_streamer_activate_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_3(r32(c, A3), 4)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(p_3(c.g[V0][0] as u32, 8)).lw(c);
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if !out_of_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = 5;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
                let requested = c.g[A2][0] == c.g[A0][0];
                if !requested {
                    c.g[V1][0] = 6;
                    c.cycles += 2;
                    let active = c.g[A2][0] == c.g[V1][0];
                    c.g[V0][0] = 1;
                    if active {
                        // returns 1
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 6;
                    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    call::call_ptr(c, 0x003a_8f10, 0x003a_9a28)?;
                    c.g[V0][0] = 1;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `func_003AABD8` (0x3aabd8): `cPS2ChunkResolve`. Runs the chunk resolve hook
/// on the chunk's flags (`*(this+0xc) & 0x70`), then fills the bounding box at
/// `+0x28`..`+0x3c` and `+0x34`..`+0x44` with the centre (`+0x1c`..`+0x24`) plus
/// and minus the half-extent already in `f0`; `v0` is left 1.
pub fn cps2_chunk_resolve_v20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = lw_r(c, S0, 0xc);
        c.g[RA][0] = sx(0x003a_abf4);
        c.g[A0][0] = c.g[A0][0] & 0x70;
        callee::WORLD_CHUNK_RESOLVE_HOOK.call(c, 0x003a_abf4)?;
        lwc1_r(c, 3, S0, 0x1c);
        c.g[V0][0] = 0x1;
        lwc1_r(c, 2, S0, 0x20);
        lwc1_r(c, 1, S0, 0x24);
        c.f[4] = (FpuF32(c.f[3]) + FpuF32(c.f[0])).0;
        c.f[5] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x44, 0);
        c.f[6] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x40, 0);
        c.f[3] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
        swc1_r(c, 4, S0, 0x34);
        c.f[2] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
        swc1_r(c, 5, S0, 0x38);
        c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
        swc1_r(c, 6, S0, 0x3c);
        swc1_r(c, 3, S0, 0x28);
        swc1_r(c, 2, S0, 0x2c);
        swc1_r(c, 1, S0, 0x30);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two chunk relocators: add `base` to every pointer
/// in the chunk and fix each entry through the object's method at `slot`.
pub(crate) fn chunk_resolve(c: &mut Cpu, slot: u32, entry: u32) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    c.g[S1][0] = 0;
    frame.save(c, S3, 0x20);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S4, 0x10);
    c.g[S3][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S4][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(r32(c, S3), 0x20)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
    c.g[S0][0] = c.g[V0][0];
    let none = (c.g[V1][0] as i64) <= 0;
    Addr(p(r32(c, S3), 0x20)).sw(c, c.g[V0][0]);
    if !none {
        c.cycles += 1;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 14;
            c.g[A2][0] = c.g[S2][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A1][0] = sx(p(r32(c, A1), r32(c, S2)));
            Addr(r32(c, S0)).sw(c, c.g[A1][0]);
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            c.g[V1][0] = Addr(p(r32(c, S4), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), slot)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), slot + 4)).lw(c);
            let fix = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S4), r32(c, A0)));
            call_ptr(c, fix, entry + 0x70)?;
            c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                break;
            }
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, entry + 0x48);
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x50);
    c.g[V0][0] = 1;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int cPS2ChunkResolve_v07(self, chunk, base)` (0x3aa520).
pub fn cps2_chunk_resolve_v07(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x40, 0x003a_a520))
}

/// `int cPS2ChunkResolve_v11(self, chunk, base)` (0x3aa830).
pub fn cps2_chunk_resolve_v11(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x60, 0x003a_a830))
}

// ---------------------------------------------------------------------------
// `cPS2ChunkResolve_v13` (0x3ac2a8): resolve a chunk instance through the
// two-level lookup tables, write the resolved pointers/flags into the
// instance and relocate its DMA chain. Returns 0 when the id has no
// resource, 1 when it does.
pub fn cps2_chunk_resolve_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = c.lw((c.g[T0][0] as u32));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        // The generated body has no early return: both misses charge one cycle,
        // zero V0 and fall into the relocate block below.
        if !empty {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let miss = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if miss {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 21;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x80), c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = c.g[T0][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0u64);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x94), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x88), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        callee::CHUNK_RESOLVE_RELOCATE_DMA.call(c, 0x003a_c344)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chunk resolver.

/// `int cPS2ChunkResolve_v05(self, void* chunk)` (0x3abe40): relocate the
/// chunk's two offsets and every pointer table entry, then finish the mesh
/// DMA relocation.
pub fn cps2chunkresolve_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S0)));
        c.g[T0][0] = sx(p(r32(c, V1), r32(c, S0)));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[T0][0]);
        c.g[V0][0] = Addr(r32(c, T0)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = sx(p(r32(c, T0), 4));
        if !none {
            loop {
                c.cycles += 10;
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = c.g[A1][0] & 0xff;
                c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx(r32(c, A1) >> 8);
                let mut resolved = 0u64;
                if !empty {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(p(r32(c, V1), 4)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    if c.g[V0][0] != 0 {
                        resolved = sx(r32(c, V0) << 2);
                    }
                }
                c.cycles += 6;
                Addr(r32(c, A2)).sw(c, resolved);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, T0)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r32(c, A2), 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_be80);
                }
            }
        }
        c.cycles += 15;
        c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x003a_bef8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_bf20, 0x003a_bf04)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_RelocateInstanceDma(self, void* chunk)` (0x3ac358):
/// relocate the instance-DMA chain for every instance in the chunk.
pub fn cps2chunkresolve_relocate_instance_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S7, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[A0][0] = 0;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S2), 0x98)).lw(c);
        c.g[S5][0] = Addr(p(r32(c, S2), 0x80)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
        Addr(p(r32(c, S2), 0x98)).sw(c, c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[S6][0] = Addr(p(r32(c, S5), 4)).lw(c);
        let none = (c.g[S6][0] as i64) <= 0;
        c.g[V1][0] = Addr(p(r32(c, S5), 8)).lw(c);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[S1][0] = Addr(p(r32(c, V1), 4)).lw(c);
                c.g[S4][0] = sx(r32(c, A0).wrapping_add(1));
                let no_obj = c.g[S1][0] == 0;
                c.g[S3][0] = sx(p(r32(c, V1), 0x10));
                let mut count_le = false;
                if !no_obj {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                    c.g[S0][0] = 0;
                    count_le = (c.g[V0][0] as i64) <= 0;
                } else {
                    count_le = true;
                }
                if !count_le {
                    c.cycles += 1;
                    loop {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S7][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        call_ptr(c, 0x003a_c508, 0x003a_c3e4)?;
                        c.g[V1][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                        c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A3][0] = c.g[V0][0];
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_c3d0);
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c3b0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x80);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v15 (0x00003aa608).

/// `cPS2ChunkResolve_v15`: resolve a PlayStation 2 chunk descriptor (version 15) to data pointers.
pub fn c_ps2_chunk_resolve_v15(c: &mut Cpu) {
    guest(c, |c| {
        'b_3aa6c0: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_3aa6c0;
            } else {
                c.cycles += 5;
                c.g[S4][0] = 0xffffffffffff0000;
                c.g[V0][0] = 0x500000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[S3][0] = c.g[V1][0];
                c.g[S4][0] = c.g[S4][0] | 0xffff;
                'l_3aa650: loop {
                    'b_3aa674: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                        if t {
                            break 'b_3aa674;
                        } else {
                            c.cycles += 6;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[RA][0] = sx(0x003aa674);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa674 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa674;
                        }
                    }
                    'b_3aa68c: {
                        'b_3aa688: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != c.g[S4][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            if t {
                                break 'b_3aa688;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
                                if t {
                                    break 'b_3aa68c;
                                } else {
                                    break 'b_3aa688;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_3aa68c;
                    }
                    'b_3aa6b4: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                        if t {
                            break 'b_3aa6b4;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[RA][0] = sx(0x003aa6b4);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa6b4 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa6b4;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003aa650);
                        }
                        continue 'l_3aa650;
                    } else {
                        break 'b_3aa6c0;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v06 (0x00003aa438).

/// `cPS2ChunkResolve_v06`: resolve a PlayStation 2 chunk descriptor (version 6) to data pointers.
pub fn c_ps2_chunk_resolve_v06(c: &mut Cpu) {
    guest(c, |c| {
        'b_3aa4f4: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_3aa4f4;
            } else {
                c.cycles += 5;
                c.g[S4][0] = 0xffffffffffff0000;
                c.g[V0][0] = 0x500000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[S3][0] = c.g[V1][0];
                c.g[S4][0] = c.g[S4][0] | 0xffff;
                'l_3aa480: loop {
                    'b_3aa4a8: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        if t {
                            break 'b_3aa4a8;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.g[S1][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[RA][0] = sx(0x003aa4a8);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa4a8 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa4a8;
                        }
                    }
                    'b_3aa4c0: {
                        'b_3aa4bc: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != c.g[S4][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                            if t {
                                break 'b_3aa4bc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
                                if t {
                                    break 'b_3aa4c0;
                                } else {
                                    break 'b_3aa4bc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_3aa4c0;
                    }
                    'b_3aa4e8: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        if t {
                            break 'b_3aa4e8;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.g[S1][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[RA][0] = sx(0x003aa4e8);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa4e8 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa4e8;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003aa480);
                        }
                        continue 'l_3aa480;
                    } else {
                        break 'b_3aa4f4;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
        Ok(())
    });
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
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as TEST_SP;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;
    pub(crate) use crate::game::be::bedbreward::tests::call_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::camera::cameracontroller::tests::heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_allocator;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn c_world_cache_streamer_get_section_matches() {
        check(
            ssx3_ee::s13::f_003a9820,
            c_world_cache_streamer_get_section,
            200,
            &|c, seed| {
                let mut r = Rng(0x9999 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u64;
                c.g[A1][0] = index;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(DATA + 0x8, DATA + 0x1000);
                c.bus.write32(head + 0x8, count);
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_is_section_resident_matches() {
        check(
            ssx3_ee::s13::f_003a9858,
            c_world_cache_streamer_is_section_resident,
            200,
            &|c, seed| {
                let mut r = Rng(0xaaaa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(
                    DATA + index * 8 + 0x14,
                    if seed % 2 == 0 { 3 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_is_section_unloaded_matches() {
        check(
            ssx3_ee::s13::f_003a9890,
            c_world_cache_streamer_is_section_unloaded,
            200,
            &|c, seed| {
                let mut r = Rng(0xbbbb ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(
                    DATA + index * 8 + 0x14,
                    if seed % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_get_hull_matches() {
        check_lohi(
            ssx3_ee::s13::f_003a9a98,
            c_world_cache_streamer_get_hull,
            200,
            &|c, seed| {
                let mut r = Rng(0xcccc ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.bus.write32(DATA + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_get_hull_count_matches() {
        check(
            ssx3_ee::s13::f_003a9ab0,
            c_world_cache_streamer_get_hull_count,
            200,
            &|c, seed| {
                let mut r = Rng(0xdddd ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let owner = DATA + 0x100;
                c.bus.write32(DATA + 0x4, owner);
                c.bus.write32(owner + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn world_cache_streamer_update_view_node_matches() {
        check(
            ee::s13::f_003aa028__generated,
            world_cache_streamer_update_view_node,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3aa0);
                let (wc, cache, hulls) = (DATA, DATA + 0x8000, DATA + 0x3000);
                c.bus.write32(wc, cache);
                c.bus.write32(gp_at(0x2a74), DATA + 0x9000);
                c.bus.write32(gp_at(-0x23c0), f(uf(&mut r, 0.0, 4.0)));
                c.bus.write32(gp_at(-0x23bc), f(uf(&mut r, 0.0, 4.0)));
                for id in 0..8 {
                    c.bus.write32(cache + 0x3f0 + 0x18 * id, r.next_u32() % 5);
                }
                let view = r.next_u32() % 2;
                let rec = wc + 0x250 + 0x50 * view;
                put_vec3(c, &mut r, rec, -10.0, 10.0);
                c.bus.write32(rec + 0xc, f(1.0));
                for k in 0..3 {
                    let lo = uf(&mut r, -12.0, 4.0);
                    c.bus.write32(rec + 0x10 + 4 * k, f(lo));
                    c.bus
                        .write32(rec + 0x20 + 4 * k, f(lo + uf(&mut r, 0.0, 16.0)));
                }
                c.bus.write32(rec + 0x1c, f(uf(&mut r, -1.0, 1.0)));
                c.bus.write32(rec + 0x2c, f(uf(&mut r, -1.0, 1.0)));
                c.bus.write32(rec + 0x30, f(uf(&mut r, 1.0, 30.0)));
                c.bus.write32(rec + 0x34, f(uf(&mut r, 0.0, 30.0)));
                c.bus.write32(rec + 0x38, f(uf(&mut r, 0.0, 30.0)));
                // A root with up to two leaf children.
                for i in 0..3u32 {
                    let nd = DATA + 0x1000 + 0x100 * i;
                    for k in 0..3 {
                        let lo = uf(&mut r, -12.0, 8.0);
                        c.bus.write32(nd + 4 * k, f(lo));
                        c.bus
                            .write32(nd + 0x10 + 4 * k, f(lo + uf(&mut r, 0.0, 10.0)));
                    }
                    for k in 0..8 {
                        c.bus.write32(nd + 0x20 + 4 * k, f(uf(&mut r, -1.0, 1.0)));
                    }
                    let hull = hulls + 0x10 * i;
                    c.bus.write16(hull + 2, (r.next_u32() % 8) as u16);
                    c.bus
                        .write32(nd + 0x58, if r.next_u32() % 4 == 0 { 0 } else { hull });
                    let (left, right) = if i == 0 {
                        (DATA + 0x1100, DATA + 0x1200)
                    } else {
                        (0, 0)
                    };
                    c.bus
                        .write32(nd + 0x50, if seed % 3 == 0 { 0 } else { left });
                    c.bus
                        .write32(nd + 0x54, if seed % 5 == 0 { 0 } else { right });
                }
                c.g[A0][0] = sx(wc);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = view as u64;
                c.g[A3][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    #[test]
    fn cw_script_cache_ctor_matches() {
        check(
            ee::s13::f_003ac7c8__generated,
            cw_script_cache_ctor,
            60,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x3ac7_c8);
                let this = DATA + 0x400;
                fill(c, &mut r, this, 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    #[test]
    fn ps2_chunk_resolve_v14_matches() {
        check(
            ssx3_ee::s13::f_003aa5c8__generated,
            ps2_chunk_resolve_v14,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa5_c8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, DATA + 0x200 + 8, (seed as u32) << 4);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x78]);
                    w(c, DATA + 8, vt);
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3aa3f0 cPS2ChunkResolveBase_v05.

    #[test]
    fn c_ps2_chunk_resolve_base_v05_matches() {
        check(
            ee::s13::f_003aa3f0__generated,
            c_ps2_chunk_resolve_base_v05,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa3_f0, 1);
                let resolver = DATA + 0x100;
                let chunk = DATA + 0x200;
                c.g[A0][0] = sx(resolver);
                c.g[A1][0] = sx(chunk);
                c.bus.write32(resolver + 8, DATA + 0x300);
                c.bus.write32(chunk + 8, r.next_u32());
                c.bus.write32(chunk + 0xc, r.next_u32());
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(DATA + 0x300, vt);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_create_matches() {
        check(
            ee::s13::f_003aad98__generated,
            c_ps2_chunk_resolve_create,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3aad_98, 0);
                alloc_ready(c);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_dtor_matches() {
        check(
            ee::s13::f_003aade8__generated,
            c_ps2_chunk_resolve_dtor,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x3aade8, 2);
                alloc_ready_2(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) const SEEDS_5: u32 = 150;

    /// A float whose bits come from a small set, so every branch of a wrap or
    /// clamp function is reached.
    pub(crate) fn pick(r: &mut Rng, values: &[f32]) -> u32 {
        values[(r.next_u32() as usize) % values.len()].to_bits()
    }

    pub(crate) fn word(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn cps2_chunk_resolve_v16_matches() {
        check(
            ssx3_ee::s13::f_003aa6e8__generated,
            cps2_chunk_resolve_v16,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa6e8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(c, DATA + 0x120, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v12_matches() {
        check(
            ssx3_ee::s13::f_003aa8d8__generated,
            cps2_chunk_resolve_v12,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa8d8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = r.next_u32() as u64;
                word(c, DATA + 0x104, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_init_matches() {
        check(
            ssx3_ee::s13::f_003aae40__generated,
            cps2_chunk_resolve_init,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aae40, 0);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_add(0x16c8),
                    DATA + 0x400,
                );
                word(c, DATA + 0x400, DATA + 0x500);
                word(c, DATA + 0x500, DATA + 0x600);
                c.bus
                    .write16(DATA + 0x600 + 0x2a, (r.next_u32() & 0xffff) as u16);
                word(c, DATA + 0xc, r.next_u32());
                word(c, DATA + 0x10, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------ chunk resolver

    /// The hash structures the resolver walks: `self -> *(self) -> +8` section
    /// table, then each section's three inner tables (`+0x4`, `+0x14`,
    /// `+0x24`). Entry 0 of every inner table is the "missing" case.
    pub(crate) fn resolver(c: &mut Cpu, self_: u32) {
        let p1 = self_ + 0x100;
        let p2 = self_ + 0x200;
        let sections = self_ + 0x300;
        let sec = self_ + 0x800;
        word(c, self_, p1);
        word(c, p1, p2);
        word(c, p2 + 8, sections);
        for i in 0..256 {
            word(c, sections + 4 * i, sec);
        }
        word(c, sec + 0xc, 0x1234);
        for (off, inner) in [
            (0x24u32, self_ + 0x900),
            (0x14, self_ + 0xa00),
            (0x4, self_ + 0xb00),
        ] {
            word(c, sec + off, inner);
            for i in 0..256u32 {
                let v = if i == 0 { 0 } else { (i << 8) | 0x40 };
                word(c, inner + 4 * i, v);
            }
        }
    }

    #[test]
    fn cps2_chunk_resolve_v17_matches() {
        check(
            ssx3_ee::s13::f_003aa700__generated,
            cps2_chunk_resolve_v17,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa700, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                resolver(c, DATA);
                let h = match seed % 4 {
                    0 => 0,
                    1 => 0x0000_0042,
                    2 => 0x0000_1042,
                    _ => r.next_u32() & 0xffff,
                };
                word(c, DATA + 0x1064, h);
                word(c, DATA + 0x1084, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_base_v13_matches() {
        check(
            ssx3_ee::s13::f_003aa8f0__generated,
            cps2_chunk_resolve_base_v13,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa8f0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                resolver(c, DATA);
                let h = match seed % 4 {
                    0 => 0,
                    1 => 0x0000_0042,
                    2 => 0x0000_1042,
                    _ => r.next_u32() & 0xffff,
                };
                word(c, DATA + 0x1080, h);
                word(c, DATA + 0x108c, r.next_u32());
                word(c, DATA + 0x1094, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v09_matches() {
        check(
            ssx3_ee::s13::f_003aa758__generated,
            cps2_chunk_resolve_v09,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa758, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(
                    c,
                    DATA + 0x110,
                    match seed % 3 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
                word(c, DATA + 0x114, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v10_matches() {
        check(
            ssx3_ee::s13::f_003aa780__generated,
            cps2_chunk_resolve_v10,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa780, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64; // 0, 1 or 2 counted arrays
                resolver(c, DATA);
                let data = DATA + 0x1000;
                c.g[A2][0] = sx(data);
                let mut at = data;
                let n = (seed % 3).max(1);
                for _ in 0..n {
                    let count = 1 + (r.next_u32() % 3);
                    word(c, at, count);
                    for j in 0..count {
                        let h = match r.next_u32() % 4 {
                            0 => 0,
                            1 => 0x42,
                            2 => 0x1042,
                            _ => r.next_u32() & 0xffff,
                        };
                        word(c, at + 4 + 4 * j, h);
                    }
                    at += 4 * (count + 1);
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v18_matches() {
        check_lohi(
            ssx3_ee::s13::f_003aa960__generated,
            cps2_chunk_resolve_v18,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa960, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                let n = 1 + (seed % 3);
                word(c, DATA + 0x120, n);
                word(c, DATA + 0x124, r.next_u32());
                for i in 0..n {
                    let node = DATA + 0x200 + 0x90 * i;
                    let prev = match r.next_u32() % 4 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        2 => n - 1,
                        _ => r.next_u32() % n,
                    };
                    let next = match r.next_u32() % 4 {
                        0 => 0xffff_ffff,
                        1 => 1 % n,
                        _ => r.next_u32() % n,
                    };
                    word(c, node + 0x60, prev);
                    word(c, node + 0x64, next);
                    word(c, node + 0x68, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v21_matches() {
        check(
            ssx3_ee::s13::f_003aac50__generated,
            cps2_chunk_resolve_v21,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aac50, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x23b8),
                    5000.0f32.to_bits(),
                );
                word(
                    c,
                    DATA + 0x10c,
                    if seed % 3 == 0 { 0x100 } else { r.next_u32() },
                );
                word(
                    c,
                    DATA + 0x110,
                    match seed % 5 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => 0xffff_ffff,
                    },
                );
                word(
                    c,
                    DATA + 0x11c,
                    pick(&mut r, &[0.0, 4999.0, 5000.0, 5001.0, 1.0e9]),
                );
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v22_matches() {
        check(
            ssx3_ee::s13::f_003aaca8__generated,
            cps2_chunk_resolve_v22,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aaca8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(
                    c,
                    DATA + 0x10c,
                    if seed % 3 == 0 { 0x100 } else { r.next_u32() },
                );
                word(
                    c,
                    DATA + 0x110,
                    match seed % 6 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        4 => 4,
                        _ => 0xffff_ffff,
                    },
                );
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v10_delivers_events() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s13::f_003aa780__generated,
            cps2_chunk_resolve_v10,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa780, 0);
                resolver(c, DATA);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(DATA + 0x1000);
                let count = 6;
                word(c, DATA + 0x1000, count);
                for j in 0..count {
                    word(c, DATA + 0x1004 + 4 * j, r.next_u32() & 0xffff);
                }
                schedule(c, &mut r, seed, 0x003a_a780);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v18_delivers_events() {
        use crate::difftest::events::schedule;
        check_lohi(
            ssx3_ee::s13::f_003aa960__generated,
            cps2_chunk_resolve_v18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa960, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(c, DATA + 0x120, 6);
                for i in 0..6 {
                    let node = DATA + 0x200 + 0x90 * i;
                    word(c, node + 0x60, if i == 0 { 0xffff_ffff } else { i - 1 });
                    word(c, node + 0x64, if i == 5 { 0xffff_ffff } else { i + 1 });
                }
                schedule(c, &mut r, seed, 0x003a_a960);
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_activate_section_matches() {
        check(
            ssx3_ee::s13::f_003a99d8__generated,
            c_world_cache_streamer_activate_section,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a99d8, 0);
                let cache = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = idx as u64;
                let sections = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                w32(c, cache + 4, sections);
                w32(c, sections + 8, if seed % 4 == 0 { 2 } else { 8 });
                for i in 0..8u32 {
                    w32(
                        c,
                        cache + 0x14 + i * 8,
                        if i == idx {
                            (seed % 8) as u32
                        } else {
                            r.next_u32() % 8
                        },
                    );
                }
                // The flagging path needs a world whose section has no objects.
                let world = DATA + 0x3000;
                let table = DATA + 0x4000;
                let lists = DATA + 0x5000;
                w32(c, cache, world);
                w32(c, world + 0x14, table);
                w16(c, table + idx * 0x58 + 0x22, 0);
                w32(c, world + 8, lists);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v20_matches() {
        check(
            ee::s13::f_003aabd8__generated,
            cps2_chunk_resolve_v20,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x3aabd8, 2);
                be_library(c);
                world(c);
                let chunk = DATA;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0xc, (seed * 0x11) & 0x70);
                let mut r = Rng(0x5a5a_0000 ^ seed as u64);
                w(c, chunk + 0x1c, r.edge_f32());
                w(c, chunk + 0x20, r.edge_f32());
                w(c, chunk + 0x24, r.edge_f32());
                c.f[0] = r.edge_f32();
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v07_matches() {
        check(
            ee::s13::f_003aa520__generated,
            cps2_chunk_resolve_v07,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3aa520, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt7 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt7);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v11_matches() {
        check(
            ee::s13::f_003aa830__generated,
            cps2_chunk_resolve_v11,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3aa830, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt8 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt8);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v13_matches() {
        check(
            ee::s13::f_003ac2a8__generated,
            cps2_chunk_resolve_v13,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x3ac2);
                let instance = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(instance);
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x100, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 8, DATA + 0x400);
                let variant = seed % 4;
                let index = if variant == 0 { 0x55 } else { 0x34 };
                c.bus.write32(instance + 0x80, index);
                c.bus
                    .write32(DATA + 0x400 + (index & 0xff) * 4, DATA + 0x500);
                // The second-level value is a handle whose upper bits become a
                // small offset; keep the low RAM words plausible.
                c.bus.write32(0x20, 0x11);
                c.bus.write32(0x24, 0x22);
                fill(c, &mut r, instance + 0x8c, 0x10);
            },
        );
    }

    #[test]
    fn cps2chunkresolve_v05_matches() {
        check(
            ee::s13::f_003abe40__generated,
            cps2chunkresolve_v05,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x3abe40, 2);
                let this = DATA;
                let chunk = DATA + 0x200;
                clear_2(c, this, 0x400);
                clear_2(c, chunk, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 8, 0x10);
                w(c, chunk + 0xc, 0x20);
                w(c, chunk + 0x20, if seed % 2 == 0 { 0 } else { 0 });
                let vt = vtable(c, &mut r, &[0x30]);
                w(c, this + 8, vt);
            },
        );
    }

    #[test]
    fn cps2chunkresolve_relocate_instance_dma_matches() {
        check(
            ee::s13::f_003ac358__generated,
            cps2chunkresolve_relocate_instance_dma,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x3ac358, 2);
                let this = DATA;
                let chunk = DATA + 0x200;
                clear_2(c, this, 0x400);
                clear_2(c, chunk, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0x98, 0x10);
                w(c, chunk + 0x80, DATA + 0x600);
                clear_2(c, DATA + 0x600, 0x100);
                w(
                    c,
                    DATA + 0x600 + 4,
                    if seed % 2 == 0 { 0 } else { 0xffff_ffff },
                );
                w(c, DATA + 0x600 + 8, 0);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_v15_matches() {
        diff(
            ee::s13::f_003aa608__generated,
            c_ps2_chunk_resolve_v15,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x3a_a608);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x2008, DATA + 0x3000);
                w(c, DATA + 0x3000, r.next_u32());
                w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
                w(c, DATA + 0x3008, r.next_u32());
                w(c, DATA + 0x300c, 0xffff_ffff);
                w(c, DATA + 8, 0); // no vtable on the chunk: calls are fakes
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_v06_matches() {
        diff(
            ee::s13::f_003aa438__generated,
            c_ps2_chunk_resolve_v06,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x3a_a438);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(DATA);
                w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x2008, DATA + 0x3000);
                w(c, DATA + 0x3000, r.next_u32());
                w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
                w(c, DATA + 0x3008, r.next_u32());
                w(c, DATA + 8, 0);
            },
        );
    }
}
