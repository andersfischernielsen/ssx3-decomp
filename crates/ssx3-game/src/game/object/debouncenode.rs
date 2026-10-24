//! `src/object/debouncenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::dirtysock::tags::lbu;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::ldf;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::load_gp;
pub(crate) use crate::game::visualfx::renderstateman::gp;
pub(crate) use crate::game::world::worldview::p_2 as p_3;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_2d10, debounce_ctor, "Debounce_ctor"),
    (0x0034_2d88, debounce_v15, "Debounce_v15"),
    (0x0034_2dd8, debounce_kill, "Debounce_Kill"),
    (0x0034_3010, conveyor_ctor, "Conveyor_ctor"),
    (0x0034_34e8, conveyor_update, "Conveyor_Update"),
    (0x0034_3718, conveyor_save, "Conveyor_Save"),
    (0x0034_3768, counter_ctor, "Counter_ctor"),
    (0x0034_37c0, counter_ctor_2, "Counter_ctor_2"),
    (0x0034_3820, counter_update, "Counter_Update"),
    (0x0034_3868, counter_handle_event, "Counter_HandleEvent"),
    (0x0034_3a18, counter_save, "Counter_Save"),
    (0x0034_3a68, crowd_man3d_ctor, "CrowdMan3d_ctor"),
    (0x0034_3b28, crowd_man3d_dtor, "CrowdMan3d_dtor"),
    (0x0034_3bc0, crowd_man3d_reset, "CrowdMan3d_Reset"),
    (
        0x0034_3c08,
        crowd_man3d_remove_section,
        "CrowdMan3d_RemoveSection",
    ),
    (0x0034_4240, crowd_man3d_load_state, "CrowdMan3d_LoadState"),
    (
        0x0034_42e0,
        crowd_man3d_group_reset,
        "CrowdMan3d_GroupReset",
    ),
    (0x0034_4368, crowd_man3d_group_init, "CrowdMan3d_GroupInit"),
    (
        0x0034_4730,
        crowd_man3d_group_set_mesh_frame,
        "CrowdMan3d_GroupSetMeshFrame",
    ),
    (
        0x0034_49f0,
        crowd_man3d_group_update,
        "CrowdMan3d_GroupUpdate",
    ),
    (0x0034_4fc0, crowd_man3d_group_save, "CrowdMan3d_GroupSave"),
    (0x0034_4ff8, crowd_man3d_group_load, "CrowdMan3d_GroupLoad"),
    (
        0x0034_5048,
        spline_modifier_eval_segment,
        "SplineModifier_EvalSegment",
    ),
    (
        0x0034_5248,
        spline_modifier_eval_path_at_distance,
        "SplineModifier_EvalPathAtDistance",
    ),
    (
        0x0034_5430,
        multi_spline_modifier_debug_draw_path,
        "MultiSplineModifier_DebugDrawPath",
    ),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
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
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS as RIDER_METRIX_AWARD_POINTS_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
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
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2;
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

    pub const HAS_OBJECT_SCRIPT1: Callee = Callee {
        addr: 0x0030_a270,
        f: ee::s10::f_0030a270,
    };

    pub const HAS_OBJECT_SCRIPT3: Callee = Callee {
        addr: 0x0030_a2e8,
        f: ee::s10::f_0030a2e8,
    };

    pub const GET_OBJECT_SLOT: Callee = Callee {
        addr: 0x003a_d120,
        f: ee::s13::f_003ad120,
    };

    pub const GET_PLAYER_INSTANCE: Callee = Callee {
        addr: 0x0014_5970,
        f: ee::s01::f_00145970,
    };

    pub const GET_HUMAN_RIDER: Callee = Callee {
        addr: 0x0014_6e98,
        f: ee::s01::f_00146e98,
    };

    pub const GET_CHARACTER: Callee = Callee {
        addr: 0x0014_7398,
        f: ee::s01::f_00147398,
    };

    pub const GET_TRACK_PEAK: Callee = Callee {
        addr: 0x0014_4c78,
        f: ee::s01::f_00144c78,
    };

    pub const GET_TRANSFORM_BOUNDS: Callee = Callee {
        addr: 0x0035_2b88,
        f: ee::s11::f_00352b88,
    };

    pub const EVAL_SEGMENT: Callee = Callee {
        addr: 0x0034_5048,
        f: ee::s11::f_00345048,
    };

    /// `void cTransitionMan_Stop(cTransitionMan*)` (0x2e4578): ends the fade.
    pub const TRANSITION_MAN_STOP: Callee = Callee {
        addr: 0x002e_4578,
        f: ee::s09::cTransitionMan_Stop,
    };

    /// `int SplineModifier_IsNodeVisible(SplineModifierNode*)` (0x345638).
    pub const SPLINE_MODIFIER_IS_NODE_VISIBLE: Callee = Callee {
        addr: 0x0034_5638,
        f: ee::s11::SplineModifier_IsNodeVisible,
    };

    /// `void ScriptViewObj_BindCamSlot(ScriptViewObj*, int, int)` (0x281ac8).
    pub const SCRIPT_VIEW_OBJ_BIND_CAM_SLOT: Callee = Callee {
        addr: 0x0028_1ac8,
        f: ee::s07::ScriptViewObj_BindCamSlot,
    };

    // Crowd / modifier nodes (shard 11).
    pub const CROWD_MAN3D_GROUP_FREE: Callee = Callee {
        addr: 0x0034_4348,
        f: ee::s11::f_00344348,
    };

    pub const CROWD_MAN3D_GROUP_RESET: Callee = Callee {
        addr: 0x0034_42e0,
        f: ee::s11::f_003442e0,
    };
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
}

// ---------------------------------------------------------------------------
// Splines.

/// One `basis * quad` apply of the Bezier evaluation: `lqc2 vf8, from;
/// lqc2 vf4..7, basis; vmulax/vmadday/vmaddaz/vmaddw; sqc2 vf12, to`.
#[inline(always)]
pub(crate) fn basis_apply(c: &mut Cpu, basis: u32, from: u32, to: u32) {
    c.lqc2(8, from);
    c.lqc2(4, basis);
    c.lqc2(5, basis.wrapping_add(0x10));
    c.lqc2(6, basis.wrapping_add(0x20));
    c.lqc2(7, basis.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, to);
}

/// `void SplineModifier_EvalSegment(sDebCurve* self, float t, sDebVec4* pos,
/// sDebVec4* vel, sDebVec4* acc)` (0x345048): evaluate the cubic
/// `u = ((c0*t + c1)*t + c2)*t + c3` (and its first/second derivatives) at
/// `t - self->t0` and map the Bezier position/velocity/acceleration vectors
/// through the key's 4x4 basis at `self + 0x10`.
pub fn spline_modifier_eval_segment(c: &mut Cpu) {
    c.cycles += 93;
    let curve = abi::arg(c, 0);
    let (pos, vel, acc) = (abi::arg(c, 1), abi::arg(c, 2), abi::arg(c, 3));
    let frame_sp = sp(c).wrapping_sub(0x50);
    c.lwc1(0, curve.wrapping_add(0x84)); // key start time
    c.g[SP][0] = sx(frame_sp);
    c.lwc1(1, curve.wrapping_add(0x50)); // c0
    c.g[V1][0] = sx(curve.wrapping_add(0x10)); // basis
    let basis = c.g[V1][0] as u32;
    c.sub_s(12, 12, 0); // t -= t0
    c.lwc1(2, curve.wrapping_add(0x54)); // c1
    c.lwc1(7, curve.wrapping_add(0x58)); // c2
    c.g[AT][0] = 0x4040_0000; // 3.0
    c.mtc1(1, 6);
    c.add_s(8, 2, 2); // 2*c1
    c.mul_s(0, 1, 12); // c0*t
    c.lwc1(9, curve.wrapping_add(0x5c)); // c3
    c.mul_s(5, 12, 6); // 3t
    c.g[AT][0] = 0x40c0_0000; // 6.0
    c.mtc1(1, 4);
    c.g[AT][0] = 0x4000_0000; // 2.0
    c.mtc1(1, 3);
    c.add_s(0, 0, 2);
    c.sw(frame_sp.wrapping_add(0x8), 0);
    c.mul_s(1, 1, 5); // c0*3t
    c.swc1(3, frame_sp.wrapping_add(0x4)); // 2.0
    c.sw(frame_sp.wrapping_add(0xc), 0);
    c.mul_s(0, 0, 12);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(1, 10);
    c.add_s(1, 1, 8); // c0*3t + 2c1
    c.add_s(0, 0, 7); // (c0 t + c1) t + c2
    c.mul_s(1, 1, 12); // *t
    c.mul_s(0, 0, 12); // *t
    c.add_s(1, 1, 7); // du = ((..)*t + c2)
    c.add_s(0, 0, 9); // u = (..)*t + c3
    c.mul_s(4, 0, 4); // u*6
    c.mul_s(2, 0, 0); // u*u
    c.add_s(5, 0, 0); // 2u
    c.swc1(4, frame_sp); // acceleration input x = u*6
                         // acc = basis * (u*6, 2, 0, 0) * du
    basis_apply(c, basis, frame_sp, frame_sp.wrapping_add(0x30));
    c.mul_s(3, 2, 0); // u^3
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.mul_s(6, 2, 6); // 3u^2
    c.sq(2, frame_sp.wrapping_add(0x20));
    c.mfc1(4, 1); // du bits into $a0 (low word of g4)
    c.lqc2(4, frame_sp.wrapping_add(0x20));
    c.qmtc2(4, 3); // vf3 = (du, *, *, *)
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame_sp.wrapping_add(0x30));
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.sq(2, frame_sp.wrapping_add(0x10));
    c.sq(2, acc);
    // pos = basis * (u^3, u^2, u, 1)
    c.swc1(3, frame_sp.wrapping_add(0x10));
    c.swc1(2, frame_sp.wrapping_add(0x14));
    c.swc1(0, frame_sp.wrapping_add(0x18));
    c.swc1(10, frame_sp.wrapping_add(0x1c));
    basis_apply(
        c,
        basis,
        frame_sp.wrapping_add(0x10),
        frame_sp.wrapping_add(0x30),
    );
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.sq(2, frame_sp.wrapping_add(0x20));
    c.sq(2, pos);
    // vel = basis * (3u^2, 2u, 1, 0)
    c.swc1(6, frame_sp.wrapping_add(0x20));
    c.swc1(5, frame_sp.wrapping_add(0x24));
    c.swc1(10, frame_sp.wrapping_add(0x28));
    c.sw(frame_sp.wrapping_add(0x2c), 0);
    basis_apply(
        c,
        basis,
        frame_sp.wrapping_add(0x20),
        frame_sp.wrapping_add(0x40),
    );
    c.lq(2, frame_sp.wrapping_add(0x40));
    c.sq(2, vel);
    c.sq(2, frame_sp.wrapping_add(0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(frame_sp.wrapping_add(0x50));
    c.pc = ra;
}

// Stages of `SplineModifier_EvalPathAtDistance` (the generated dispatcher's
// block labels; the two loop heads are re-entry points after an unwind).
pub(crate) const EP_ENTRY: u32 = 0x0034_5248;

pub(crate) const EP_TRUNC_NEG: u32 = 0x0034_5274;

pub(crate) const EP_SUB: u32 = 0x0034_5284;

pub(crate) const EP_BACK_ENTRY: u32 = 0x0034_5294;

pub(crate) const EP_BACK: u32 = 0x0034_5298;

pub(crate) const EP_LT: u32 = 0x0034_52b0;

pub(crate) const EP_BACK_TEST: u32 = 0x0034_52c0;

pub(crate) const EP_NEXT: u32 = 0x0034_52cc;

pub(crate) const EP_FWD_ENTRY: u32 = 0x0034_52d4;

pub(crate) const EP_FWD: u32 = 0x0034_52d8;

pub(crate) const EP_NEXT_TEST: u32 = 0x0034_52f0;

pub(crate) const EP_LE: u32 = 0x0034_52f8;

pub(crate) const EP_EVAL: u32 = 0x0034_530c;

/// `void SplineModifier_EvalPathAtDistance(SplineModifier* self, float t,
/// sDebVec4* pos, sDebVec4* vel, sDebVec4* acc)` (0x345248): reduce `t`
/// modulo the key's length, walk the key chain backwards to the segment that
/// starts at or before `t` (while `t < key->t0`), then forwards over segments
/// that end before `t`, and evaluate that segment. `self + 4` is the key
/// index, `self + 8` the current key, `self + 0xc` its length. Dispatcher-
/// shaped in the original: it is re-entered at the two walk loop heads
/// (0x345298, 0x3452d8) after an unwind.
pub fn spline_modifier_eval_path_at_distance(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                EP_ENTRY => {
                    c.cycles += 11;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
                    c.g[T0][0] = abi::arg64(c, 0);
                    c.sd(sp(c), c.g[RA][0]);
                    c.lwc1(2, Addr(c.g[T0][0] as u32).off(0xc).0);
                    c.div_s(0, 12, 2);
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        pc = EP_TRUNC_NEG;
                    } else {
                        c.mul_s(1, 1, 2);
                        pc = EP_SUB;
                    }
                }
                EP_TRUNC_NEG => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 2);
                    pc = EP_SUB;
                }
                EP_SUB => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).off(0x8).lw(c);
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    c.sub_s(12, 12, 1); // t -= f * length
                    pc = EP_LT;
                }
                EP_BACK_ENTRY => {
                    c.cycles += 1;
                    pc = EP_BACK;
                }
                EP_BACK => {
                    c.cycles += 6;
                    let self_ = c.g[T0][0] as u32;
                    c.g[V1][0] = Addr(self_).off(4).lw(c);
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x60).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    Addr(self_).off(4).sw(c, c.g[V1][0]);
                    Addr(self_).off(8).sw(c, c.g[V0][0]);
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    pc = EP_LT;
                }
                EP_LT => {
                    c.cycles += 4;
                    c.c_lt_s(12, 0);
                    if c.fcc() {
                        pc = EP_BACK_TEST;
                    } else {
                        c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                        pc = EP_NEXT;
                    }
                }
                EP_BACK_TEST => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x60).lw(c);
                    let has_prev = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                    if has_prev {
                        if crate::events::back_edge_unwinds(c, EP_BACK) {
                            return Ok(());
                        }
                        pc = EP_BACK;
                    } else {
                        pc = EP_NEXT;
                    }
                }
                EP_NEXT => {
                    c.cycles += 2;
                    let _always = true;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x64).lw(c);
                    pc = if _always { EP_NEXT_TEST } else { EP_FWD_ENTRY };
                }
                EP_FWD_ENTRY => {
                    c.cycles += 1;
                    pc = EP_FWD;
                }
                EP_FWD => {
                    c.cycles += 6;
                    let self_ = c.g[T0][0] as u32;
                    c.g[V1][0] = Addr(self_).off(4).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x64).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    Addr(self_).off(4).sw(c, c.g[V1][0]);
                    Addr(self_).off(8).sw(c, c.g[A0][0]);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x64).lw(c);
                    pc = EP_NEXT_TEST;
                }
                EP_NEXT_TEST => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        pc = EP_EVAL;
                    } else {
                        pc = EP_LE;
                    }
                }
                EP_LE => {
                    c.cycles += 5;
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    c.c_le_s(0, 12);
                    if c.fcc() {
                        c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                        if crate::events::back_edge_unwinds(c, EP_FWD) {
                            return Ok(());
                        }
                        pc = EP_FWD;
                    } else {
                        pc = EP_EVAL;
                    }
                }
                EP_EVAL => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0034_5314);
                    c.g[A0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                    callee::EVAL_SEGMENT.call(c, 0x0034_5314)?;
                    c.g[RA][0] = c.ld(sp(c));
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, EP_ENTRY, 0x0034_5320);
                    return Ok(());
                }
            }
        }
    })
}

/// The board-wake effect world (`D_004A5B80`, `gp+0x2a90`).
pub(crate) const BOARD_WAKE_WORLD: i32 = 0x2a90;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `lw` result as an address/value.
#[inline(always)]
pub(crate) fn lw32(c: &mut Cpu, a: u32) -> u32 {
    c.lw(a) as u32
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_DebugDrawPath.

/// `void MultiSplineModifier_DebugDrawPath(void* self, void* arg)` (0x345430):
/// draws every visible node of the modifier's node list (`self->+8->+0x68`:
/// count at `+0x20`, first node at `+0x24`, next at `+0x64`). The wake
/// controller (`D_004A5B80->+0xe84`) is put in mode 1 (`+4` bits 2..6
/// cleared, bit 2 set) and its mode half at `+0x10` is cleared to -1; each
/// visible node is drawn through vtable slot 114 (`+0x258`) with the node
/// offset, `arg` and priority 10.
pub fn multi_spline_modifier_debug_draw_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = u64::MAX;
        frame.save(c, S2, 0x20);
        c.g[A3][0] = sx(0xffff_ff83);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        let w = lw32(c, gp(c, BOARD_WAKE_WORLD));
        c.g[S2][0] = sx(w);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[A0][0] = Addr(w).off(0xe84).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x68).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x20).lw(c);
        c.g[S0][0] = Addr(r32(c, V0)).off(0x24).lw(c);
        let ctrl = r32(c, A0);
        let minus_one = c.g[A2][0];
        Addr(ctrl).off(0x10).sh(c, minus_one);
        c.g[V1][0] = Addr(w).off(0xe84).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(4).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] | 4;
        let count = c.g[A1][0];
        let mode = c.g[V0][0];
        c.sw(p(r32(c, V1), 4), mode);
        if (count as i64) <= 0 {
            // No nodes.
        } else {
            c.cycles += 2;
            c.g[S1][0] = count;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::SPLINE_MODIFIER_IS_NODE_VISIBLE.call(c, 0x0034_5498)?;
                let hidden = c.g[V0][0] == 0;
                c.g[A1][0] = sx(p(r32(c, S0), 0x10));
                if !hidden {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(w).off(0x10d8).lw(c);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A3][0] = 0xa;
                    c.g[A0][0] = Addr(r32(c, V0)).off(0x258).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(0x25c).lw(c);
                    let target = r32(c, V1);
                    let delta = r32(c, A0);
                    c.g[RA][0] = sx(0x0034_54bc);
                    c.g[A0][0] = sx(w.wrapping_add(delta));
                    call_ptr(c, target, 0x0034_54bc)?;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = Addr(r32(c, S0)).off(0x64).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_5490);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_GroupSave(CrowdMan3d* man, sWScriptStream* stream)`
/// (0x344fc0).
pub fn crowd_man3d_group_save(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0034_4fe8, 8, 0x10c, None))
}

/// `void CrowdMan3d_GroupInit(CrowdGroup* self, u32 arg)` (0x344368): seed
/// the group's inverse count (`1.0 / count` from the crowd manager at
/// gp+0x2a74) and build its meshes.
pub fn crowd_man3d_group_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        load_gp(c, V1, 0x2a74);
        c.sw(p(r32(c, V0), 4), c.g[A1][0]);
        c.sw(p(r32(c, V0), 0x1dc), 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.lwc1(1, p(r32(c, V1), 0x10));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, p(r32(c, V0), 0x1e4));
        callee::CROWD_MAN3D_GROUP_CREATE_MESHES.call(c, 0x0034_439c)?;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Counter_Update(DebounceNode* self)` (0x343820): when the debounce
/// timer (`+0x1c`) has not started, set it to -1 and tell the node
/// (`vtable` slot 0x110) that it was updated.
pub fn counter_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
            c.sw(r32(c, A2).wrapping_add(0x1c), u64::MAX);
            let vt = r32(c, V1);
            c.g[A1][0] = 1;
            c.g[A0][0] = c.lh(vt.wrapping_add(0x110));
            c.g[V0][0] = c.lw(vt.wrapping_add(0x114));
            let method = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0034_3858)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd / modifier objects.

/// `void CrowdMan3d_Reset(crowd)` (0x343bc0): free and reinitialise each of
/// the five 0x1f4-byte crowd groups (the loop from `crowd` to `crowd+0x9c4`).
pub fn crowd_man3d_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c4));
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            CROWD_MAN3D_GROUP_FREE.call(c, 0x0034_3be0)?;
            c.g[A0][0] = c.g[S0][0];
            CROWD_MAN3D_GROUP_RESET.call(c, 0x0034_3be8)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let again = c.g[V0][0] != 0;
            c.g[RA][0] = c.ld(frame.at(c, 0));
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_3bd8);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void Conveyor_Save(Conveyor* self, Stream* stream)` (0x343718): 0x2c bytes
/// at `+0x1c`.
pub fn conveyor_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::WORLD_OBJ_SAVE,
            0x1c,
            0x2c,
            0x0034_3734,
            0x0034_3750,
        )
    })
}

/// `void Counter_Save(Counter* self, Stream* stream)` (0x343a18): 8 bytes at
/// `+0x1c`.
pub fn counter_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 8, 0x0034_3a34, 0x0034_3a50)
    })
}

/// `void CrowdMan3d_GroupLoad(CrowdMan3dGroup* self, void* stream)`
/// (0x344ff8): let the group's stream read its chunk (0x10c bytes), then
/// rebuild the group's meshes.
pub fn crowd_man3d_group_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        let stream = r32(c, A1);
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A1][0];
        c.g[A2][0] = 0x10c;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(stream).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_5028)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN3D_GROUP_FREE_MESHES.call(c, 0x0034_5030)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2.call(c, 0x0034_5038)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World objects and particles.

/// `Counter* Counter_ctor(Counter* self, void* a1, void* a2, void* a3)`
/// (0x343768): `WorldObj_ctor(self, a1, 7, a2)`, then the Counter vtable,
/// the config word from `a3+4`, and a cleared counter at `+0x20`.
pub fn counter_ctor(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A3][0];
    c.g[S0][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = 7;
    if callee::WORLD_OBJ_CTOR.call(c, 0x0034_378c).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0049_03f0);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    c.sw(p_2(c, S0, 0xc), c.g[V1][0]);
    c.g[A0][0] = Addr(p_2(c, S1, 4)).lh(c);
    c.sw(p_2(c, S0, 0x20), 0);
    c.sw(p_2(c, S0, 0x1c), c.g[A0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// World and modifiers (s11).

/// `void CrowdMan3d_RemoveSection(CrowdMan3d* self, int section)` (0x343c08):
/// remove a section from each of the manager's five groups.
pub fn crowd_man3d_remove_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x9c4));
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0034_3c38);
            c.g[A1][0] = c.g[S2][0];
            callee::CROWD_MAN3D_GROUP_REMOVE_SECTION.call(c, 0x0034_3c38)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_3c30);
            }
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

/// `void Conveyor_Update(Conveyor* c)` (0x003434e8): advance the UV scroll
/// offsets and wrap each back into (-1, 1].
pub fn conveyor_update(c: &mut Cpu) {
    c.cycles += 12;
    let s = abi::arg(c, 0);
    c.lwc1(3, p_3(s, 0x40)); // u
    c.lwc1(2, p_3(s, 0x38)); // du
    c.lwc1(1, p_3(s, 0x44)); // v
    c.add_s(3, 3, 2);
    c.lwc1(0, p_3(s, 0x3c)); // dv
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 4);
    c.add_s(1, 1, 0);
    c.c_lt_s(4, 3); // 1.0 < u
    c.swc1(3, p_3(s, 0x40));
    let over_u = c.fcc();
    c.swc1(1, p_3(s, 0x44));
    if over_u {
        c.cycles += 2;
        c.sub_s(0, 3, 4);
        c.cycles += 2;
        c.swc1(0, p_3(s, 0x40));
        c.lwc1(1, p_3(s, 0x44));
    } else {
        c.cycles += 6;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 0);
        c.c_lt_s(3, 0); // u < -1.0
        if c.fcc() {
            c.cycles += 1;
            c.add_s(0, 3, 4);
            c.cycles += 2;
            c.swc1(0, p_3(s, 0x40));
            c.lwc1(1, p_3(s, 0x44));
        } else {
            c.lwc1(1, p_3(s, 0x44));
        }
    }
    c.cycles += 5;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 2);
    c.c_lt_s(2, 1); // 1.0 < v
    if c.fcc() {
        c.cycles += 3;
        c.sub_s(0, 1, 2);
        c.swc1(0, p_3(s, 0x44));
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
    c.mtc1(1, 0);
    c.c_lt_s(1, 0); // v < -1.0
    if c.fcc() {
        c.cycles += 2;
        c.add_s(0, 1, 2);
        c.swc1(0, p_3(s, 0x44));
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Counter_HandleEvent(Counter* self, int msg, float level)`
/// (0x00343868): latch one bit per level 1..10 at +0x20 and, when the latch
/// fires, count the counter at +0x1c down.
pub fn counter_handle_event(c: &mut Cpu) {
    let self_ = abi::arg(c, 0);
    let msg = abi::arg64(c, 1);
    // bit = 1 << (level - 1) for the levels 1..10, 0 otherwise, from the
    // equality chain against 1.0f .. 10.0f.
    const LEVELS: [(u32, u64); 8] = [
        (0x4000_0000, 0x2),   // 2.0f
        (0x4040_0000, 0x4),   // 3.0f
        (0x4080_0000, 0x8),   // 4.0f
        (0x40a0_0000, 0x10),  // 5.0f
        (0x40c0_0000, 0x20),  // 6.0f
        (0x40e0_0000, 0x40),  // 7.0f
        (0x4100_0000, 0x80),  // 8.0f
        (0x4110_0000, 0x100), // 9.0f
    ];
    c.cycles += 7;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.g[A2][0] = 0;
    c.c_eq_s(12, 0);
    let is_one = c.fcc();
    c.g[A3][0] = 0; // fire
    let mut bit = 0u64;
    if is_one {
        c.cycles += 2;
        bit = 1;
    } else {
        for (bits, mask) in LEVELS {
            c.cycles += 5;
            c.g[AT][0] = bits as u64;
            c.mtc1(1, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                c.cycles += 2;
                bit = mask;
                break;
            }
        }
        if bit == 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000; // 10.0f
            c.mtc1(1, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                bit = 0x200;
            }
        }
    }
    c.g[A2][0] = bit; // the latch bit the original keeps in a2
                      // The message updates the latch (+0x20): 0x1f4 presses, 0x1f5 releases,
                      // 0x1f6 is the "fire now" message.
    let mut store: Option<u64> = None;
    let mut fire = 0u64;
    c.cycles += 3;
    c.g[V0][0] = 0x1f4;
    let is_press = msg == 0x1f4;
    c.g[V0][0] = 0x1f5;
    if is_press {
        c.cycles += 4;
        let flags = Addr(p_3(self_, 0x20)).lw(c);
        c.g[V1][0] = flags;
        c.g[V0][0] = flags | bit;
        if flags & bit == 0 {
            c.cycles += 2;
            c.g[A3][0] = 1;
            fire = 1;
            store = Some(flags | bit);
        }
    } else {
        c.cycles += 2;
        let is_release = msg == 0x1f5;
        c.g[V0][0] = msg ^ 0x1f6;
        if !is_release {
            c.cycles += 1;
            c.g[A3][0] = (c.g[V0][0] < 1) as u64;
            fire = c.g[A3][0];
        } else {
            c.cycles += 4;
            let flags = Addr(p_3(self_, 0x20)).lw(c);
            c.g[V1][0] = flags;
            c.g[V0][0] = flags & bit;
            if flags & bit == 0 {
                c.cycles += 6;
                c.g[AT][0] = 0x3f80_0000; // 1.0f
                c.mtc1(1, 0);
                c.c_eq_s(12, 0);
                let level_one = c.fcc();
                c.g[V0][0] = flags | 1;
                let fires = if level_one {
                    true
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((bit as u32) as i32 >> 1) as u32);
                    c.g[V0][0] = flags & c.g[V0][0];
                    let lower_clear = c.g[V0][0] == 0;
                    c.g[V0][0] = flags | bit;
                    !lower_clear
                };
                if fires {
                    c.cycles += 1;
                    c.g[A3][0] = 1;
                    fire = 1;
                    store = Some(c.g[V0][0]);
                }
            }
        }
    }
    if let Some(v) = store {
        // The latch store and the test before it are separate blocks.
        c.cycles += 2;
        c.sw(p_3(self_, 0x20), v);
    }
    // The "fire" test is always executed; the counter update only when set.
    c.cycles += 2;
    if fire != 0 {
        c.cycles += 3;
        let count = Addr(p_3(self_, 0x1c)).lw(c);
        let expired = (count as i64) <= 0;
        c.g[V0][0] = sx((count as u32).wrapping_sub(1));
        if !expired {
            c.cycles += 1;
            c.sw(p_3(self_, 0x1c), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Particles, FX and world objects.

/// `Counter* Counter_ctor_2(Counter* self, int a1, Stream* stream)` (0x3437c0):
/// construct the base object, install the debounce vtable and register the
/// debounce record with the stream.
pub fn counter_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_fb90, 0x0034_37dc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3f0));
        c.g[A2][0] = 8;
        Addr(p_4(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p_4(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p_4(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_3804)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `CrowdMan3d_GroupReset(CrowdMan3d* self)` (0x3442e0): clear the group's
/// counters and reset the 32 slots (`{id = -1; value = 0}`, 8 bytes each).
pub fn crowd_man3d_group_reset(c: &mut Cpu) {
    c.cycles += 16;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    sw(c, this, 0);
    sw(c, p(this, 4), c.g[V0][0]);
    c.g[V1][0] = sx(this.wrapping_add(0x10));
    sw(c, p(this, 0x1dc), 0);
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    sw(c, p(this, 8), c.g[V0][0]);
    c.g[A1][0] = 0x1f;
    sw(c, p(this, 0x1e8), 0);
    sw(c, p(this, 0x1ec), 0);
    sw(c, p(this, 0x1f0), 0);
    sw(c, p(this, 0x114), 0);
    sw(c, p(this, 0x110), 0);
    loop {
        c.cycles += 7;
        sw(c, (c.g[V1][0] as u32).wrapping_sub(4), c.g[A2][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        sw(c, c.g[V1][0] as u32, 0);
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_4320);
        }
    }
}

/// `void* Debounce_ctor(self, unused, stream)` (0x342d10): base move-node
/// constructor, install the debounce vtable and read the two 4-byte debounce
/// fields through the stream reader.
pub fn debounce_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A2][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_CTOR_2.call(c, 0x0034_2d2c)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x2c));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x6f0));
        c.g[A2][0] = 0x4;
        sw_r(c, S1, 0xc, c.g[V1][0]);
        c.g[V0][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V0, 0x10);
        c.g[V1][0] = lw_r(c, V0, 0x14);
        let target = low(c, V1);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2d54)?;
        c.g[A3][0] = Addr(low(c, S0)).lw(c);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x30));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh_r(c, A3, 0x10);
        c.g[V0][0] = lw_r(c, A3, 0x14);
        let target = low(c, V0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2d70)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void CrowdMan3d_dtor(self, flags)` (0x343b28): release the five group
/// slots (twice, as the original does) and delete the object.
pub fn crowd_man3d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c4));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::CROWD_MAN_GROUP_FREE.call(c, 0x0034_3b58)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_3b50);
            }
        }
        c.cycles += 3;
        c.g[S2][0] = c.g[S3][0] & 1;
        let no_elems = c.g[S1][0] == 0;
        Addr(gp_2(c, 0xf00)).sw(c, 0);
        if !no_elems {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c4));
            let empty = c.g[S1][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1f4));
            if !empty {
                loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::CROWD_MAN_GROUP_FREE.call(c, 0x0034_3b88)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1f4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_3b80);
                    }
                }
            }
        }
        c.cycles += 2;
        let no_delete = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table_callee::OPERATOR_DELETE, 0x0034_3ba0)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Debounce_v15(cDebounceNode* self)` (0x342d88): the node's per-tick
/// update (vtable slot 15, called by `WorldObj_Update` once per tick). While
/// the delay at `+0x2c` is positive it counts down by one; when it reaches 0
/// the node fires its vtable slot 34 with argument 1 and returns 0, else it
/// returns 1.
///
/// Tick rate: the delay comes from the level data in 60 Hz frames, so above
/// 60 it goes down once per 60 Hz frame of this node
/// ([`crate::tick::frame60_due`], keyed by `self + 0x2c`); a tick that is not
/// one returns 1 like a countdown still running. Nothing changes at 60.
pub fn debounce_v15(c: &mut Cpu) {
    const DELAY: u32 = 0x2c;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[V0][0] = Addr(this).off(DELAY).lw(c);
        let n = r32(c, V0);
        let running = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = sx(n.wrapping_sub(1));
        if running {
            c.cycles += 2;
            let key = this.wrapping_add(DELAY);
            if crate::tick::frame60_due(c, key, n) {
                Addr(this).off(DELAY).sw(c, c.g[V0][0]);
                crate::tick::frame60_ran(c, key, n.wrapping_sub(1));
                if c.g[V0][0] == 0 {
                    // Fire: vtable slot 34 (this + delta, 1).
                    c.cycles += 8;
                    c.g[V0][0] = Addr(this).off(0xc).lw(c);
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr(r32(c, V0)).off(0x110).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(0x114).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x0034_2dc0)?;
                    c.g[V0][0] = 0;
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Debounce_Kill(self)` (0x342dd8): kill a debounce node and, if it has
/// no queued callback, allocate and post the dead node.
pub fn debounce_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_2(c, A2, 0x2c);
        let dead = (c.g[V0][0] as i64) < 0;
        frame.restore(c, S0, 0x10);
        if dead {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw_2(c, A2, 0x30);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let no_parent = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !no_parent {
                c.cycles += 5;
                c.g[V0][0] = lw_2(c, A2, 0xc);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw_2(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0034_2e68)?;
            }
        } else {
            c.cycles += 2;
            let no_parent = c.g[A2][0] == 0;
            c.g[S0][0] = lw_2(c, A2, 0x18);
            if !no_parent {
                c.cycles += 6;
                c.g[V0][0] = lw_2(c, A2, 0xc);
                c.g[A1][0] = 3;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw_2(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0034_2e20)?;
            }
            c.cycles += 11;
            c.g[A1][0] = 0x0048_e738;
            c.g[A0][0] = 0x1c;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, table::EA_MEM_ALLOC_NAMED, 0x0034_2e38)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            callee::DEAD_NODE_CTOR.call(c, 0x0034_2e44)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_LoadState(self, stream)` (0x344240): reset the five crowd
/// groups and load the ones whose flag is not -1.
pub fn crowd_man3d_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[S0][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.g[S4][0] = sx(this.wrapping_add(0x9c4));
        callee::CROWD_MAN3D_RESET.call(c, 0x0034_4274)?;
        c.g[S3][0] |= 0xffff;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        'groups: loop {
            c.cycles += 10;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[V1][0] = lw_2(c, V0, 0x14);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_429c)?;
            c.g[V0][0] = lw_2(c, S1, 0x8);
            let done = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S1][0];
            if !done {
                c.cycles += 2;
                c.g[A1][0] = c.g[S2][0];
                callee::CROWD_MAN3D_GROUP_LOAD.call(c, 0x0034_42b0)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_4280);
                }
                continue 'groups;
            }
            c.cycles += 8;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// `CrowdMan3d_GroupUpdate` (0x3449f0): advance the group's debounce clock,
// wrapping past the period, update every item's matrix, then rebuild the
// group's matrices.
pub fn crowd_man3d_group_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        // time += step, wrap at period.
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x1e0));
        c.add_s(0, 1, 0);
        c.c_lt_s(2, 0);
        // fcc: period < time + step -> past the period, so wrap.
        let wrap = c.fcc();
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        if wrap {
            c.cycles += 2;
            c.sub_s(0, 0, 2);
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10c));
        let no_items = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if !no_items {
            c.cycles += 3;
            c.g[S2][0] = 0u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ec));
            loop {
                c.cycles += 13;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xd0));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0034_4a6c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ec));
                if event_due(c) {
                    deliver_event_here(c, 0x0034_4a48);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00344aa0, 0x0034_4a84)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Conveyor_ctor` (0x00343010).
//
// `void Conveyor::Conveyor(self, int, void* params, void* dir)`: build the
// world object, copy the eight direction bytes and tag the render flags.

pub fn conveyor_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        let dir = c.g[A3][0];
        frame.save(c, 17, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[S1][0] = sx(this);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = 0xf;
        c.g[S0][0] = dir;
        call_ptr(c, 0x0034_fb00, 0x0034_3034)?;
        c.g[V0][0] = 0x490000;
        c.g[A3][0] = sx(this.wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x570));
        c.g[A2][0] = sx((dir as u32).wrapping_add(0x10));
        sw_2(c, this, 0xc, c.g[V0][0]);
        c.g[A1][0] = 0;
        ldf(c, 0, dir as u32, 4);
        c.swc1(0, this.wrapping_add(0x1c));
        ldf(c, 1, dir as u32, 8);
        ldf(c, 0, dir as u32, 0xc);
        c.swc1(1, sp(c));
        c.swc1(0, sp(c).wrapping_add(4));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x3f));
        c.sdr(2, this.wrapping_add(0x38));
        sw_2(c, this, 0x40, 0);
        sw_2(c, this, 0x44, 0);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = lbu(c, c.g[A2][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if more {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3080);
                }
                continue;
            }
            c.cycles += 13;
            c.g[A1][0] = lw_3(c, this, 0x18);
            c.g[A0][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = c.g[A0][0] | 0xfffd;
            c.g[V0][0] = sx(this);
            c.g[V1][0] = lw_3(c, c.g[A1][0] as u32, 8);
            frame.restore(c, 16, 0x30);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            frame.restore(c, 17, 0x20);
            c.g[V1][0] = c.g[V1][0] | 4;
            frame.restore_ra(c, 0x10);
            sw_2(c, c.g[A1][0] as u32, 8, c.g[V1][0]);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// `CrowdMan3d_ctor` (0x00343a68).
//
// `void CrowdMan3d::CrowdMan3d(self)`: initialise the four crowd groups and
// their slot records and reset each group.

pub fn crowd_man3d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[S0][0] | 0xffff;
        frame.save_ra(c, 0);
        c.g[S3][0] = this;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        let mut a0 = c.g[A0][0];
        let mut v0: u32 = 4;
        loop {
            c.cycles += 6;
            c.g[S2][0] = sx(v0.wrapping_sub(1));
            sw_2(c, a0 as u32, 4, c.g[S0][0]);
            c.g[S1][0] = sx((a0 as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((a0 as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x1f;
            let mut v0r = c.g[V0][0] as u32;
            loop {
                c.cycles += 7;
                sw_2(c, v0r, 0, c.g[S0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let t = c.g[V1][0] != c.g[S4][0];
                v0r = v0r.wrapping_add(8);
                c.g[V0][0] = sx(v0r);
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3ab0);
                }
            }
            c.cycles += 5;
            call_ptr(c, 0x0034_42e0, 0x0034_3ad4)?;
            c.g[V0][0] = c.g[S2][0];
            let t = c.g[V0][0] != c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            if !t {
                break;
            }
            v0 = c.g[V0][0] as u32;
            a0 = c.g[A0][0];
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3a98);
            }
        }
        c.cycles += 4;
        sw_2(c, gp(c, 0xf00), 0, c.g[S3][0]);
        c.g[S0][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x9c4));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0034_42e0, 0x0034_3af8)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3af0);
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd meshes and playback teardown.

/// `void CrowdMan3d_GroupSetMeshFrame(CrowdMan3d *self, int frame)`
/// (0x344730): copy every animated mesh's frame index into the group's
/// per-instance table.
pub fn crowd_man3d_group_set_mesh_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x002d_1bd8, 0x0034_474c)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A1][0] = Addr(p_5(c.g[S0][0], 0x4)).lw(c);
        c.g[A0][0] = Addr(p_5(c.g[V1][0], 0x8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let mut t0;
        if c.g[V1][0] != 0 {
            c.g[A0][0] = Addr(p_5(c.g[V1][0], 0x1c)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            t0 = c.g[V0][0];
        } else {
            c.cycles += 2;
            t0 = 0;
        }
        c.g[T0][0] = t0;
        c.cycles += 3;
        c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x110)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x13c));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(p_5(c.g[T0][0], 0x94)).lw(c);
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(p_5(c.g[V0][0], 0x4)).lw(c);
                c.g[V0][0] = Addr(p_5(c.g[A0][0], 0x10)).lw(c);
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if !none {
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p_5(c.g[V0][0], 0x4)).lhu(c);
                    Addr(c.g[A3][0] as u32).sh(c, c.g[V1][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x110)).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_47b0);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
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
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_5;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_2;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_3;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_stream;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;
    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::h;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w as w_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;
    use ssx3_ee as ee;

    pub(crate) const SEEDS: u32 = 250;

    /// A float bit pattern in `[lo, hi)`.
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// Scrambled registers, the game's gp, and optional fake-call hooks.
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    #[test]
    fn spline_modifier_eval_segment_matches() {
        check(
            ee::s11::f_00345048__generated,
            spline_modifier_eval_segment,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3450);
                let curve = DATA;
                for i in 0..4u32 {
                    c.bus.write32(curve + 0x50 + 4 * i, r.edge_f32());
                }
                c.bus.write32(curve + 0x84, r.edge_f32());
                for q in 0..4u32 {
                    put_vec(
                        c,
                        curve + 0x10 + 0x10 * q,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
                c.g[A0][0] = sx(curve);
                c.f[12] = r.edge_f32();
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1100);
                c.g[A3][0] = sx(DATA + 0x1200);
            },
        );
    }

    #[test]
    fn spline_modifier_eval_path_at_distance_matches() {
        check(
            ee::s11::f_00345248__generated,
            spline_modifier_eval_path_at_distance,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3452);
                let node = DATA;
                let keys = [DATA + 0x1000, DATA + 0x2000, DATA + 0x3000];
                for (i, &k) in keys.iter().enumerate() {
                    c.bus.write32(k + 0x84, f(i as f32));
                    c.bus
                        .write32(k + 0x60, if i == 0 { 0 } else { keys[i - 1] });
                    c.bus
                        .write32(k + 0x64, if i == 2 { 0 } else { keys[i + 1] });
                    for j in 0..4u32 {
                        c.bus.write32(k + 0x50 + 4 * j, r.edge_f32());
                    }
                    for q in 0..4u32 {
                        put_vec(
                            c,
                            k + 0x10 + 0x10 * q,
                            [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                        );
                    }
                }
                let start = (seed % 3) as usize;
                c.bus.write32(node + 0xc, f(1.0 + (seed % 4) as f32));
                c.bus.write32(node + 4, start as u32);
                c.bus.write32(node + 8, keys[start]);
                c.f[12] = uf(&mut r, -2.0, 4.5);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x5100);
                c.g[A3][0] = sx(DATA + 0x5200);
                c.pc = EP_ENTRY;
            },
        );
    }

    // ------------------------------------------ MultiSplineModifier_DebugDrawPath

    #[test]
    fn multi_spline_modifier_debug_draw_path_matches() {
        fakes::covered(
            ee::s11::f_00345430__generated,
            multi_spline_modifier_debug_draw_path,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0e50_00e5, 2);
                let this = DATA + 0x1000;
                let world = DATA + 0x2000;
                let ctrl = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x5000) as u64;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), world);
                c.bus.write32(world + 0xe84, ctrl);
                c.bus.write32(world + 0x10d8, DATA + 0x8000);
                c.bus.write32(this + 8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6000 + 0x68, list);
                let n = seed % 4;
                c.bus.write32(list + 0x20, n);
                // A chain of four nodes.
                let base = DATA + 0x9000;
                c.bus.write32(list + 0x24, base);
                for i in 0..4u32 {
                    let node = base + i * 0x80;
                    let next = if i + 1 < 4 { base + (i + 1) * 0x80 } else { 0 };
                    c.bus.write32(node + 0x64, next);
                    c.bus.write32(node + 0x6c, r.edge_f32());
                    c.bus.write32(node + 0x70, r.edge_f32());
                    c.bus.write32(node + 0x74, r.edge_f32());
                    c.bus.write32(node + 0x78, r.edge_f32());
                    c.bus.write32(node + 0x7c, r.edge_f32());
                    c.bus.write32(node + 0x80, r.edge_f32());
                }
                c.bus.write32(ctrl + 4, r.next_u32());
                c.bus.write32(ctrl + 0x10, r.next_u32());
                // The wake world's vtable: the draw slot, the visibility slot
                // and the slot inside SplineModifier_IsNodeVisible.
                let vt = DATA + 0x8000;
                for (i, off) in [0x258u32, 0x2e8, 0x158].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    #[test]
    fn crowd_man3d_group_save_matches() {
        check(
            ee::s11::f_00344fc0__generated,
            crowd_man3d_group_save,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x344f_c0, 8);
            },
        );
    }

    #[test]
    fn crowd_man3d_group_init_matches() {
        check(
            ee::s11::f_00344368__generated,
            crowd_man3d_group_init,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x344368, 0);
                let mgr = DATA + 0x300;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(GAME_GP + 0x2a74, mgr);
                c.bus.write32(mgr + 0x10, 1 + seed % 4);
            },
        );
    }

    #[test]
    fn counter_update_matches() {
        covered(
            ee::s11::f_00343820__generated,
            counter_update,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_3820, 1);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj + 0x1c, if seed % 2 == 0 { 0 } else { r.next_u32() });
                let vt = vtable(c, &mut r, &[0x110]);
                c.bus.write32(obj + 0xc, vt);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x343bc0 CrowdMan3d_Reset.

    #[test]
    fn crowd_man3d_reset_matches() {
        check(
            ee::s11::f_00343bc0__generated,
            crowd_man3d_reset,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x343b_c0, 2);
                c.g[A0][0] = sx(DATA);
                // Five zeroed groups: GroupFree finds no mesh lists, GroupReset
                // only writes fields.
                for i in 0..5u32 {
                    c.bus.write32(DATA + i * 0x1f4 + 0x1ec, 0);
                    c.bus.write32(DATA + i * 0x1f4 + 0x1e8, 0);
                }
            },
        );
    }

    #[test]
    fn conveyor_save_matches() {
        save_test(
            ee::s11::f_00343718__generated,
            conveyor_save,
            0x3437_18,
            0x1c,
        );
    }

    #[test]
    fn counter_save_matches() {
        save_test(
            ee::s11::f_00343a18__generated,
            counter_save,
            0x343a_18,
            0x1c,
        );
    }

    #[test]
    fn crowd_man3d_group_load_matches() {
        check(
            ee::s11::f_00344ff8__generated,
            crowd_man3d_group_load,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x344f_f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let stream = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, stream, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // ------------------------------------------------------------ objects

    #[test]
    fn counter_ctor_matches() {
        check(
            ssx3_ee::s11::f_00343768__generated,
            counter_ctor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x343768, 0);
                let this = DATA;
                let host = DATA + 0x200;
                let cfg = DATA + 0x300;
                // +8 bit 0x100 takes the short path of WorldObj_ctor.
                c.bus.write32(host + 8, 0x100 | (seed & 0xff));
                c.bus.write32(cfg + 4, 0x7fff_0000 | (seed & 0xff));
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = host as u64;
                c.g[A3][0] = cfg as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // World and modifiers.

    #[test]
    fn crowd_man3d_remove_section_matches() {
        check(
            ee::s11::f_00343c08__generated,
            crowd_man3d_remove_section,
            20,
            &|c, seed| {
                let mut r = common(c, seed, 0x343c08, 5);
                let this = DATA + 0x100;
                clear(c, this, 0x9c4 / 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn conveyor_update_matches() {
        check(
            ssx3_ee::s11::f_003434e8__generated,
            conveyor_update,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3434e8, 0);
                c.g[A0][0] = sx(DATA);
                let u = pick(&mut r, &[0.0, 0.5, 0.9, 1.0, -1.0, 1.5, -1.5, 2.0, -0.2]);
                let du = pick(&mut r, &[0.0, 0.1, -0.1, 0.5, 2.0, -2.0, 1.0]);
                let v = pick(&mut r, &[0.0, 0.5, 0.9, 1.0, -1.0, 1.5, -1.5, -0.7]);
                let dv = pick(&mut r, &[0.0, 0.2, -0.2, 3.0, -3.0, 0.05]);
                word(c, DATA + 0x40, u);
                word(c, DATA + 0x38, du);
                word(c, DATA + 0x44, v);
                word(c, DATA + 0x3c, dv);
            },
        );
    }

    #[test]
    fn counter_handle_event_matches() {
        check(
            ssx3_ee::s11::f_00343868__generated,
            counter_handle_event,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x343868, 0);
                c.g[A0][0] = sx(DATA);
                c.f[12] = pick(
                    &mut r,
                    &[
                        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 0.5, 11.0, -2.0, 0.0,
                    ],
                );
                c.g[A1][0] = [0x1f4u64, 0x1f5, 0x1f6, 0x100][(seed % 4) as usize];
                let flags = match seed % 7 {
                    0 => 0,
                    1 => 0x2 | 0x8,
                    2 => 0x1,
                    3 => 0x2,
                    4 => r.next_u32(),
                    5 => 1u32 << (seed % 11),
                    _ => 0xffff_ffff,
                };
                word(c, DATA + 0x20, flags);
                let count = match seed % 4 {
                    0 => 0,
                    1 => 3,
                    2 => 0xffff_ffff,
                    _ => r.next_u32() & 0xff,
                };
                word(c, DATA + 0x1c, count);
            },
        );
    }

    /// A vtable_2 at `at` whose slots `(offset, target)` hold `{delta 0, target}`.
    pub(crate) fn vtable_2(c: &mut Cpu, at: u32, slots: &[(u32, u32)]) {
        clear(c, at, 0x40);
        for &(off, target) in slots {
            c.bus.write16(at + off, 0);
            c.bus.write32(at + off + 4, target);
        }
    }

    // ------------------------------------------------------------------
    // World objects, FX, rails.

    #[test]
    fn counter_ctor_2_matches() {
        check(
            ee::s11::f_003437c0__generated,
            counter_ctor_2,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3437_c0);
                let s = DATA + 0x100;
                clear(c, s, 0x100);
                let stream = DATA + 0x300;
                clear(c, stream, 0x20);
                let vt = DATA + 0x400;
                vtable_2(
                    c,
                    vt,
                    &[
                        (0x10, FAKE + 0x10),
                        (0x14, FAKE + 0x20),
                        (0x20, FAKE + 0x30),
                        (0x24, FAKE + 0x40),
                    ],
                );
                w(c, stream, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(stream);
            },
        );
    }

    // --------------------------------------------------------------- crowd/pads

    #[test]
    fn crowd_man3d_group_reset_matches() {
        check(
            ssx3_ee::s11::f_003442e0__generated,
            crowd_man3d_group_reset,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x3442e0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x200u32 / 4 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// Zero the words `machine` filled with junk below the stack pointer, for
    /// tests where a callee writes a value that decides control flow into a
    /// stack slot (the fakes do not).
    pub(crate) fn clear_stack(c: &mut Cpu) {
        for i in 0..0x50u32 {
            w_2(c, crate::difftest::SP - 0x140 + 4 * i, 0);
        }
    }

    #[test]
    fn debounce_ctor_matches() {
        check(
            ee::s11::f_00342d10__generated,
            debounce_ctor,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x342d10, 3);
                be_library(c);
                clear_stack(c);
                let obj = DATA;
                let stream = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = stream as u64;
                let vt = DATA + 0x3000;
                w_2(c, stream, vt);
                slot(c, vt, 0x10, 0x10, FAKE);
            },
        );
    }

    #[test]
    fn crowd_man3d_dtor_matches() {
        check(
            ee::s11::f_00343b28__generated,
            crowd_man3d_dtor,
            20,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x343b28, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                // Flags 0: the delete branch would free a pointer that did not come
                // from the test heap, which walks garbage (not a boundable fixture).
                c.g[A1][0] = 0;
                for i in 0..5u32 {
                    let e = obj + 0x1f4 * i;
                    w8(c, e + 0x1e, (seed >> i) as u8 & 1);
                    w_3(c, e, r.next_u32());
                    w_3(c, e + 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn debounce_kill_matches() {
        check(
            ssx3_ee::s11::f_00342dd8__generated,
            debounce_kill,
            30,
            &|c, seed| {
                let _r = base_4(c, seed, 0x342dd8, 3);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                w(
                    c,
                    this + 0x2c,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, this + 0x30, if (seed / 3) % 2 == 0 { 0 } else { 1 });
                w(c, this + 0x18, r32_seed(seed));
                w(c, this + 0xc, DATA + 0x2000);
                w(c, DATA + 0x2000, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 8, 0);
                w(c, DATA + 0x2100 + 0xc, 0);
            },
        );
    }

    #[test]
    fn crowd_man3d_load_state_matches() {
        check(
            ssx3_ee::s11::f_00344240__generated,
            crowd_man3d_load_state,
            8,
            &|c, seed| {
                let _r = base_4(c, seed, 0x344240, 3);
                let this = DATA + 0x1000;
                let stream = DATA + 0x5000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                w(c, stream, DATA + 0x6000);
                c.bus.write16(DATA + 0x6000 + 0x10, 0);
                w(c, DATA + 0x6000 + 0x14, 0);
                for i in 0..5u32 {
                    w(
                        c,
                        this + 0x1f4 * i + 8,
                        if seed % 2 == 0 { 0xffff_ffff } else { 0 },
                    );
                }
            },
        );
    }

    #[test]
    fn crowd_man3d_group_update_matches() {
        check(
            ee::s11::f_003449f0__generated,
            crowd_man3d_group_update,
            30,
            &|c, seed| {
                let _r = base_5(c, seed, 0x3449);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10c, (seed % 3) as u32);
                c.bus.write32(DATA + 0x1ec, DATA + 0x1000);
                c.bus.write32(DATA + 0x1dc, f(1.0));
                c.bus.write32(DATA + 0x1e0, f(10.0));
                c.bus.write32(DATA + 0x1e4, f(0.5));
                for i in 0..3 {
                    let item = DATA + 0x1000 + 0xd0 * i;
                    c.bus.write32(item + 0x84, DATA + 0x2000 + 0x100 * i);
                    c.bus.write32(DATA + 0x2000 + 0x100 * i + 0x10, 0);
                    c.bus
                        .write32(DATA + 0x2000 + 0x100 * i + 0x14, FAKE + 0x10 * i);
                }
            },
        );
    }

    #[test]
    fn conveyor_ctor_matches() {
        check(
            ee::s11::f_00343010__generated,
            conveyor_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3430, 4);
                let this = DATA;
                let params = DATA + 0x1000;
                let dir = DATA + 0x2000;
                for k in 0..4 {
                    c.bus.write32(dir + 4 * k, r.edge_f32());
                    c.bus
                        .write32(dir + 0x10 + 4 * k, (r.next_u32() % 256) as u32);
                }
                c.bus.write32(this + 0x18, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 8, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(params);
                c.g[A3][0] = sx(dir);
            },
        );
    }

    #[test]
    fn crowd_man3d_ctor_matches() {
        check(
            ee::s11::f_00343a68__generated,
            crowd_man3d_ctor,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x343a, 4);
                let this = DATA;
                for k in 0..4 {
                    c.bus.write32(this + 0x1f4 * k + 0x18, DATA + 0x8000);
                }
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP + 0xf00, r.next_u32());
            },
        );
    }

    #[test]
    fn crowd_man3d_group_set_mesh_frame_matches() {
        check(
            ee::s11::f_00344730__generated,
            crowd_man3d_group_set_mesh_frame,
            40,
            &|c, seed| {
                let _r = base_6(c, seed, 0x3447);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(DATA + 0x110, 0);
                c.bus.write32(gp_at(0x16c8), DATA + 0x3000);
                c.bus.write32(DATA + 0x3008, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
            },
        );
    }

    #[test]
    fn debounce_v15_matches() {
        check(
            ssx3_ee::s11::f_00342d88__generated,
            debounce_v15,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x342d88, 1);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                let n = match seed % 4 {
                    0 => 1,
                    1 => 0,
                    2 => (r.next_u32() | 0x8000_0000) as u32,
                    _ => 2 + r.next_u32() % 100,
                };
                c.bus.write32(this + 0x2c, n);
                let vt = vtable(c, &mut r, &[0x110]);
                c.bus.write32(this + 0xc, vt);
            },
        );
    }

    /// Above 60 Hz the delay counts 60 Hz frames: a delay of 10 fires after
    /// 10 ticks at 60, after `10 * rate/60 - (rate/60 - 1)` ticks at 120/240.
    #[test]
    fn debounce_v15_follows_the_rate() {
        fn ticks(hz: u32) -> u32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 1, 0x342d89, 0);
            crate::tick::set_for_test(c, hz);
            let this = DATA + 0x1000;
            c.bus.write32(this + 0x2c, 10);
            let vt = vtable(c, &mut r, &[0x110]);
            c.bus.write32(this + 0xc, vt);
            let mut n = 0;
            loop {
                n += 1;
                c.g[A0][0] = this as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                debounce_v15(c);
                assert_eq!(c.pc, crate::difftest::RA);
                if c.g[V0][0] == 0 {
                    return n;
                }
                assert!(n < 10_000);
            }
        }
        assert_eq!(ticks(60), 10);
        for hz in [120u32, 240] {
            let q = hz / 60;
            assert_eq!(ticks(hz), 10 * q - (q - 1), "{hz} Hz");
        }
    }
}
