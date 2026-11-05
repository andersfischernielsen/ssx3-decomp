//! `src/input/inputparse.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::world::worldview::p_2 as p_3;
pub(crate) use crate::game::world::wscriptcache::DELETE_2;
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
        0x0032_17d0,
        input_parser_lookup_modifier,
        "InputParser_LookupModifier",
    ),
    (
        0x0032_1a40,
        input_parser_find_identifier,
        "InputParser_FindIdentifier",
    ),
    (
        0x0032_1ad0,
        input_parser_builtin_operand,
        "InputParser_BuiltinOperand",
    ),
    (
        0x0032_1ad8,
        input_parser_pad_channel_operand,
        "InputParser_PadChannelOperand",
    ),
    (
        0x0032_45a8,
        input_parser_compile_identifier,
        "InputParser_CompileIdentifier",
    ),
    (0x0032_5250, input_vm_init, "InputVM_Init"),
    (0x0032_5260, input_vm_read_operand, "InputVM_ReadOperand"),
    (0x0032_52e8, input_vm_get_reg_ptr, "InputVM_GetRegPtr"),
    (0x0032_5430, input_vm_read_builtin, "InputVM_ReadBuiltin"),
    (0x0032_5450, input_vm_execute, "InputVM_Execute"),
    (
        0x0032_6308,
        input_preproc_write_string,
        "InputPreproc_WriteString",
    ),
    (0x0032_6360, input_preproc_write, "InputPreproc_Write"),
    (
        0x0032_6430,
        input_preproc_emit_line_marker,
        "InputPreproc_EmitLineMarker",
    ),
    (
        0x0032_6478,
        input_preproc_next_char,
        "InputPreproc_NextChar",
    ),
    (
        0x0032_65c0,
        input_preproc_skip_line_comment,
        "InputPreproc_SkipLineComment",
    ),
    (
        0x0032_6678,
        input_preproc_skip_line,
        "InputPreproc_SkipLine",
    ),
    (0x0032_6a28, pad_queue_init, "PadQueue_Init"),
    (0x0032_6a68, pad_queue_shutdown, "PadQueue_Shutdown"),
    (0x0032_6c60, pad_queue_flush, "PadQueue_Flush"),
    (
        0x0032_6cf0,
        pad_device_stop_actuators,
        "PadDevice_StopActuators",
    ),
    (0x0032_6d60, ps2_pad_create, "PS2Pad_Create"),
    (0x0032_6df0, ps2_pad_ctor, "PS2Pad_ctor"),
    (0x0032_6e50, ps2_pad_dtor, "PS2Pad_dtor"),
    (0x0032_7740, ps2pad_set_actuator, "PS2Pad_SetActuator"),
    (0x0032_77d0, pad_device_dtor, "PadDevice_dtor"),
    (0x0032_7810, ai_fwd_diff_cache_ctor, "AIFwdDiffCache_ctor"),
    (0x0032_7828, ai_fwd_diff_cache_dtor, "AIFwdDiffCache_dtor"),
    (0x0032_6eb0, ps2_pad_update, "PS2Pad_Update"),
    (
        0x0032_52f8,
        input_vm_read_pad_channel,
        "InputVM_ReadPadChannel",
    ),
    (0x0032_6b88, pad_queue_poll, "PadQueue_Poll"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT_2 as COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FLEX_RAIL_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FOCUS_MAN_COLLECT_OCTREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_FIND_NEAREST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_MODIFIER_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR as ACTIVE_CAMERA_TRIGGER_LIST_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;

    pub const GET_REG_PTR: Callee = Callee {
        addr: 0x0032_52e8,
        f: ee::s10::InputVM_GetRegPtr,
    };

    pub const READ_OPERAND: Callee = Callee {
        addr: 0x0032_5260,
        f: ee::s10::InputVM_ReadOperand,
    };

    pub const MATH_ATAN: Callee = Callee {
        addr: 0x0031_c228,
        f: ee::s10::Math_Atan,
    };

    pub const MATH_SIN: Callee = Callee {
        addr: 0x0031_bf60,
        f: ee::s10::Math_Sin,
    };

    pub const MATH_COS: Callee = Callee {
        addr: 0x0031_c040,
        f: ee::s10::Math_Cos,
    };

    /// `logf` and `expf`: cross-shard, so the original reaches them through
    /// the function table.
    pub const LOGF: u32 = 0x0040_da10;

    pub const EXPF: u32 = 0x0040_d758;
}

/// `gp`-relative literals the expression opcodes read: the atan2 half-plane
/// corrections (0x13) and the angle wrap (0x1d).
pub(crate) mod literal {

    pub const HALF_PI_NEG: i32 = -0x2e24;

    pub const HALF_PI: i32 = -0x2e20;

    pub const PI: i32 = -0x2e1c;

    pub const PI_SUB: i32 = -0x2e18;

    pub const INV_TWO_PI: i32 = -0x2e14;

    pub const TWO_PI: i32 = -0x2e10;
}

// ---------------------------------------------------------------------------
// Input system.

/// `int InputParser_PadChannelOperand(int player, int channel, int base)`
/// (0x321ad8): `base + player + channel * 6 + 0x60`.
pub fn input_parser_pad_channel_operand(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x6;
    let scaled = abi::mult(c, abi::arg(c, 1), 6);
    c.g[A0][0] = sx(scaled);
    let base = scaled.wrapping_add(abi::arg(c, 2));
    c.g[V0][0] = sx(base.wrapping_add(0x60));
    abi::ret(c);
}

/// `void InputVM_Init(InputVM *vm)` (0x325250): clear the first word and
/// return the object.
pub fn input_vm_init(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `float* InputVM_GetRegPtr(InputVM *vm, int index)` (0x3252e8): the register
/// slot at `vm + index*4 + 8`.
pub fn input_vm_get_reg_ptr(c: &mut Cpu) {
    c.cycles += 4;
    let off = abi::arg(c, 1).wrapping_shl(2).wrapping_add(8);
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(off));
    abi::ret(c);
}

/// `float InputVM_ReadBuiltin(InputVM *vm, int index)` (0x32525430): 0.0 for
/// index 0, otherwise the builtin float at `vm + index*4 + 0x108`.
pub fn input_vm_read_builtin(c: &mut Cpu) {
    c.cycles += 2;
    let index = abi::arg64(c, 1);
    let off = abi::arg(c, 1).wrapping_shl(2);
    c.g[V0][0] = sx(off);
    if index == 0 {
        c.cycles += 3;
        c.f[0] = 0;
    } else {
        c.cycles += 3;
        let ptr = abi::arg(c, 0).wrapping_add(off);
        c.g[V0][0] = sx(ptr);
        c.f[0] = Addr(ptr).off(0x108).read_fpu(c).0;
    }
    abi::ret(c);
}

/// `int InputPreproc_NextChar(InputPreproc *p, InputPreproc* /*cursor*/)`
/// (0x326478): advance the read cursor past the byte at the end of the
/// buffered range; store the byte (or -1) in the output word.
pub fn input_preproc_next_char(c: &mut Cpu) {
    c.cycles += 6;
    let out = Addr(abi::arg(c, 0));
    let cur = Addr(abi::arg(c, 1));
    let start = cur.lw(c);
    let len = cur.off(4).lw(c);
    let ptr = cur.off(8).lw(c);
    let end = sx((start as u32).wrapping_add(len as u32));
    c.g[V1][0] = len;
    c.g[A2][0] = ptr;
    c.g[V0][0] = end;
    if ptr != end {
        let byte = Addr(ptr as u32).lbu(c);
        c.cycles += 4;
        c.g[V0][0] = byte;
        let next = sx((ptr as u32).wrapping_add(1));
        c.g[V1][0] = next;
        out.off(0xc).sw(c, byte);
        cur.off(8).sw(c, next);
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        out.off(0xc).sw(c, u64::MAX);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pad queue.

/// `int PadQueue_Flush(PadQueue *q)` (0x326c60): wrap the write cursor at
/// `+0x2ee4` by 30 and return the remainder of that against the count at
/// `+0x2ee0`.
pub fn pad_queue_flush(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    let cursor = q.off(0x2ee4).lw(c);
    c.g[V1][0] = cursor;
    c.g[A1][0] = 0x1e;
    c.cycles += 12;
    c.g[T0][0] = 0x1e;
    let next = sx((cursor as u32).wrapping_add(0x1d));
    c.g[V0][0] = next;
    let count = q.off(0x2ee0).lw(c);
    c.g[A2][0] = count;
    let (_, rem) = abi::div(c, next as u32, 0x1e);
    let wrapped = sx((cursor as u32)
        .wrapping_sub(count as u32)
        .wrapping_add(0x1e));
    c.g[V1][0] = wrapped;
    c.g[A3][0] = sx(rem);
    let (_, rem2) = abi::div(c, wrapped as u32, 0x1e);
    q.off(0x2ee0).sw(c, sx(rem));
    c.g[V0][0] = sx(rem2);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forward-difference cache.

/// `AIFwdDiffCache* AIFwdDiffCache_ctor(AIFwdDiffCache *c)` (0x327810): zero
/// the four head words.
pub fn ai_fwd_diff_cache_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(4).sw(c, 0);
    this.off(8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

pub(crate) const EA_MEM_FREE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::EA_MemFree,
};

/// `AIFwdDiffCache *AIFwdDiffCache_dtor(AIFwdDiffCache *this, uint flags)`
/// (0x327828): free the two owned blocks (`this+0` and `this+8`) and, when
/// `flags & 1`, delete `this` itself.
pub fn ai_fwd_diff_cache_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let has_first = c.g[A0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_first {
            c.cycles += 2;
            EA_MEM_FREE.call(c, 0x0032_7850)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        let has_second = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if has_second {
            c.cycles += 3;
            EA_MEM_FREE.call(c, 0x0032_7864)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0032_7878)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input VM: operand values.

/// `float InputVM_ReadOperand(sInputVM* self, int id)` (0x00325260): the
/// value of one operand. `id < 0x40` is a script variable (`self->v[id]`),
/// `0x40..0x60` a builtin, `0x60..0x3fc` controller channel
/// `(id - 0x60) / 6` reading `(id - 0x60) % 6`, and anything else 0.
pub fn input_vm_read_operand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = (id < 0x40) as u64;
        let not_variable = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !not_variable {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            let at = p(c.g[V0][0] as u32, 0x8);
            c.lwc1(0, at);
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x40..0x60: a builtin.
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = (c.g[V1][0] < 0x20) as u64;
        let not_builtin = c.g[V0][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
        if !not_builtin {
            c.cycles += 4;
            c.g[A1][0] = c.g[V1][0];
            callee::INPUT_VM_READ_BUILTIN.call(c, 0x0032_5298)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x60..0x3fc: controller channel `id / 6`, reading `id % 6`.
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < 0x39c) as u64;
        let padded = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6;
        if !padded {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = 0x6;
        c.div(0, A1, V0);
        c.cycles += 4;
        c.g[A1][0] = c.lo[0];
        c.g[A2][0] = c.hi[0];
        callee::INPUT_VM_READ_PAD_CHANNEL.call(c, 0x0032_52d8)?;
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Misc small accessors.

/// `char* InputParser_BuiltinOperand(InputParser* p, int index)` (0x321ad0):
/// the built-in operand block for a pad, 0x40 bytes into the parser.
pub fn input_parser_builtin_operand(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_add(0x40));
    abi::ret(c);
}

/// `PadDevice_dtor` (0x3277d0): vtable at `+0`.
pub fn pad_device_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_e4b8, 0x0032_77f4);
}

/// `void PadQueue_Init(PadQueue* self)` (0x326A28): create the four pad queues
/// at +0x2EEC and set the queue parameters (0x1D, 0).
pub fn pad_queue_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 4;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r(c, S0).wrapping_add(0x2eec));
        callee::PS2_PAD_CREATE.call(c, 0x0032_6a44)?;
        c.g[V1][0] = 0x1d;
        c.sw_r_2(r(c, S0).wrapping_add(0x2ee8), V0);
        Addr(r(c, S0).wrapping_add(0x2ee0)).sw(c, 0x1d);
        Addr(r(c, S0).wrapping_add(0x2ee4)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_EmitLineMarker(InputPreproc* p, int line, char* text)`
/// (0x326430): append `# <line> "<text>"` to the preprocessed output.
pub fn input_preproc_emit_line_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0xc0);
        c.g[A2][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0xb0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1c38));
        callee::SPRINTF_2.call(c, 0x0032_6458)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::INPUT_PREPROC_WRITE_STRING.call(c, 0x0032_6464)?;
        frame.restore(c, S0, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_WriteString(InputPreproc* in, const char* s)` (0x326308):
/// call `InputPreproc_Write(in, *s++)` for every character up to the NUL. The
/// back edge re-enters at 0x326328 after a thread switch.
pub fn input_preproc_write_string(c: &mut Cpu) {
    loop {
        match c.pc {
            0x0032_6308 => {
                c.cycles += 7;
                let frame = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A0][0];
                c.pc = 0x0032_633c;
            }
            0x0032_6324 => {
                c.cycles += 1;
                c.pc = 0x0032_6328;
            }
            0x0032_6328 => {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if callee::INPUT_PREPROC_WRITE.call(c, 0x0032_633c).is_err() {
                    return;
                }
                c.pc = 0x0032_633c;
            }
            0x0032_633c => {
                // Registers at the call were set by the previous block.
                c.cycles += 3;
                c.g[V0][0] = Addr(p_2(c, S0, 0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(p_2(c, S0, 0)).lbu(c);
                if more {
                    if back_edge_unwinds(c, 0x0032_6328) {
                        return;
                    }
                    c.pc = 0x0032_6328;
                } else {
                    c.pc = 0x0032_6348;
                }
            }
            0x0032_6348 => {
                c.cycles += 5;
                let frame = Frame { size: 0x30 };
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0032_6308, 0x0032_635c);
                return;
            }
        }
    }
}

/// `void PS2Pad_SetActuator(PS2Pad* pad, int mode, float strength)`
/// (0x00327740): with mode 1 store whether the actuator is on at +0x30;
/// otherwise scale `strength` to a motor value clamped to 50..255 at +0x38.
pub fn ps2pad_set_actuator(c: &mut Cpu) {
    c.cycles += 3;
    let pad = abi::arg(c, 0);
    let mode = abi::arg64(c, 1);
    c.g[V0][0] = 1;
    if mode == 1 {
        // *(int*)(pad + 0x30) = strength != 0
        c.cycles += 5;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_eq_s(12, 0);
        if c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        c.sw(p_3(pad, 0x30), c.g[V0][0]);
        abi::ret(c);
        return;
    }
    // *(int*)(pad + 0x38) = clamp((int)(strength * 205 + 50), 50, 255)
    c.cycles += 5;
    c.mtc1(0, 0); // f0 = 0.0
    c.c_eq_s(12, 0);
    let off = c.fcc();
    c.g[A0][0] = sx(p_3(pad, 0x38));
    let value = if off {
        c.cycles += 1;
        c.g[V0][0] = 0;
        0u64
    } else {
        c.cycles += 11;
        c.g[AT][0] = 0x434d_0000; // 205.0f
        c.mtc1(1, 0);
        c.g[AT][0] = 0x4248_0000; // 50.0f
        c.mtc1(1, 1);
        c.mul_s(0, 12, 0);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1); // v1 = (int)(strength * 205 + 50)
        c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
        let below = c.g[V0][0] != 0;
        c.g[A1][0] = 0xff;
        if below {
            c.cycles += 1;
            c.g[V1][0] = 0x32; // 50
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[V1][0];
        c.g[V1][0]
    };
    c.cycles += 2;
    c.sw(c.g[A0][0] as u32, value);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pads and strings.

/// `void PS2Pad_ctor(PS2Pad* pad, int port, int slot)` (0x326df0): install the
/// vtable, remember the port/slot and clear the pad state.
pub fn ps2_pad_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0048_e488);
        let self_ = c.g[A0][0];
        let port = c.g[A1][0];
        let slot = c.g[A2][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0x4)).sw(c, port);
        c.g[A0][0] = sx(at(c, S0, 0xc));
        Addr(at(c, S0, 0x8)).sw(c, slot);
        c.g[A1][0] = 0;
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x24;
        call_ptr(c, 0x0041_6210, 0x0032_6e24)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x54)).sw(c, 0);
        Addr(at(c, S0, 0x58)).sw(c, 0);
        Addr(at(c, S0, 0x50)).sw(c, 0);
        Addr(at(c, S0, 0x5c)).sw(c, 0);
        Addr(at(c, S0, 0x48)).sw(c, 0);
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2Pad_dtor(PS2Pad* pad, int free)` (0x326e50): restore the vtable,
/// tear the pad down and optionally delete it.
pub fn ps2_pad_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_e488;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0048_e488);
        frame.save(c, S1, 0x10);
        let free = c.g[A1][0];
        c.g[S0][0] = free;
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S1][0] = self_;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S1, 0x4)).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
        call_ptr(c, 0x003f_f8f0, 0x0032_6e80)?;
        c.g[V1][0] = 0x48_e4b8;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        c.g[V1][0] = sx(0x0048_e4b8);
        let keep = c.g[S0][0] == 0;
        Addr(at(c, S1, 0x0)).sw(c, c.g[V1][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_6e9c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PadQueue_StopAllActuators(PadQueue* self)` (0x326ae0): stop the actuators
/// of every queued pad device.
pub fn pad_queue_stop_all_actuators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(p(this as u32, 0x2eec));
            c.g[A0][0] = lw(c, p(this as u32, 0x2eec));
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                call_ptr(c, 0x0032_6cf0, 0x0032_6b1c)?;
                c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6b10);
                }
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

/// The opcode dispatch table (`jpt_3254c4`). The port matches on the opcode;
/// the address is only kept to reproduce what the original leaves in `v1`.
pub(crate) const OPCODES: u32 = 0x0048_e330;

/// Opcode that ends the run (`s6 = 1`).
pub(crate) const OP_STOP: u32 = 0x20;

/// Loop head: the resume address of the back-edge event check.
pub(crate) const HEAD: u32 = 0x0032_5498;

/// The shared "store `f0` into the destination" block, and the event check
/// that resumes there.
pub(crate) const STORE: u32 = 0x0032_5524;

/// The block that reads the first operand again before that store.
pub(crate) const STORE_OPERAND: u32 = 0x0032_5518;

/// `parser` fields: the incoming argument (`+4`) and the virtual register
/// file (`+8`, one 32-bit float each).
pub(crate) const PARSER_ARG: u32 = 0x4;

#[allow(dead_code)] // the register file base is only named in the test
pub(crate) const PARSER_REGS: u32 = 0x8;

// ---------------------------------------------------------------------------
// The interpreter's state: the values the original keeps in s0..s6.

pub(crate) struct Vm {
    /// s2: the parser (its virtual registers at `+8`, four bytes each).
    parser: u32,
    /// s5: the opcode stream.
    stream: u32,
    /// s1: the current instruction.
    ins: u32,
    /// s4: the index of the word after the current instruction.
    idx: u32,
    /// s0: destination of most opcodes (a 0/1 flag in 0x04/0x05).
    dst: u32,
    /// s3: destination of 0x04/0x05, which use `s0` as a flag.
    alt: u32,
    /// s6: set by opcode 0x20 to end the run.
    stop: u64,
}

impl Vm {
    /// Put the loop state where a callee (or an event handler) expects it: the
    /// original keeps these in the callee-saved registers throughout.
    fn spill(&self, c: &mut Cpu) {
        c.g[S0][0] = sx(self.dst);
        c.g[S1][0] = sx(self.ins);
        c.g[S2][0] = sx(self.parser);
        c.g[S3][0] = sx(self.alt);
        c.g[S4][0] = sx(self.idx);
        c.g[S5][0] = sx(self.stream);
        c.g[S6][0] = sx(self.stop as u32);
    }
}

// ---------------------------------------------------------------------------
// FPU and call helpers.

/// FPU register `n`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// `lwc1 fN, gp+off`.
#[inline(always)]
pub(crate) fn gp_float(c: &mut Cpu, off: i32, n: usize) -> FpuF32 {
    let a = r32(c, GP).wrapping_add(off as u32);
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// Set the FPU condition bit; returns it.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `InputVM_GetRegPtr(parser, idx)`: the address of virtual register `idx`.
pub(crate) fn dst_of(c: &mut Cpu, vm: &mut Vm, idx: u32, ret: u32) -> Result<u32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::GET_REG_PTR.call(c, ret)?;
    Ok(r32(c, V0))
}

/// `f0 = InputVM_ReadOperand(parser, idx)`.
pub(crate) fn operand(c: &mut Cpu, vm: &Vm, idx: u32, ret: u32) -> Result<FpuF32, Unwound> {
    vm.spill(c);
    c.g[A0][0] = sx(vm.parser);
    c.g[A1][0] = sx(idx);
    callee::READ_OPERAND.call(c, ret)?;
    Ok(fr(c, 0))
}

/// The shared `0x00325524` tail: store `f0` in the destination.
pub(crate) fn store_result(c: &mut Cpu, vm: &Vm) {
    c.cycles += 2;
    let v = fr(c, 0);
    Addr(vm.dst).write_fpu(c, v);
}

/// The shared `0x00325518` tail: `f0 = ReadOperand(parser, idx)`, then store.
pub(crate) fn store_operand(c: &mut Cpu, vm: &Vm, idx: u32) -> Flow {
    c.cycles += 3;
    operand(c, vm, idx, STORE)?;
    store_result(c, vm);
    Ok(())
}

// ---------------------------------------------------------------------------
// InputVM_Execute.

/// `float InputVM_Execute(InputVM* parser, void* arg, const u32* code)`
/// (0x325450): run the opcode stream at `code` (kept at `parser + 4`) up to
/// opcode 0x20, then return operand 0 of the parser's register file.
pub fn input_vm_execute(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0xa0 frame, s0-s3 and f20-f22 saved.
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x50);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x90);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        // s2/s5 are set here, s4/s6 are zeroed; s0/s1/s3 keep the incoming
        // values (as the prologue leaves them, visible only if a callee
        // unwinds before the epilogue).
        let mut vm = Vm {
            parser: r32(c, A0),
            stream: r32(c, A2),
            ins: r32(c, S1),
            idx: 0,
            dst: r32(c, S0),
            alt: r32(c, S3),
            stop: 0,
        };
        Addr(vm.parser.wrapping_add(PARSER_ARG)).sw(c, c.g[A1][0]);
        c.g[V0][0] = 0;

        'run: loop {
            // 0x00325498: fetch and dispatch.
            c.cycles += 6;
            vm.ins = vm.stream.wrapping_add(vm.idx.wrapping_shl(2));
            let word = c.lw(vm.ins) as u32;
            let op = word & 0x3f;
            c.g[V0][0] = sx(word);
            c.g[A0][0] = sx(op);
            let dispatched = op <= OP_STOP;
            c.g[V1][0] = dispatched as u64;
            vm.idx = vm.idx.wrapping_add(1);
            if dispatched {
                // 0x003254b0: the jump table (`v1` keeps its entry address).
                c.cycles += 7;
                c.g[V0][0] = sx(OPCODES);
                c.g[V1][0] = sx(OPCODES.wrapping_add(op << 2));
                run_op(c, &mut vm, op, word)?;
            }

            // 0x00325efc: loop while opcode 0x20 has not been seen.
            c.cycles += 2;
            c.g[V0][0] = sx(vm.idx.wrapping_shl(2));
            if vm.stop != 0 {
                break 'run;
            }
            if event_due(c) && deliver_event(c, HEAD) {
                return Ok(());
            }
        }

        // 0x00325f04: the result is operand 0.
        c.cycles += 16;
        operand(c, &vm, 0, 0x0032_5f10)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x90);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    });
}

/// One dispatched opcode. The cycle charges are the original's per-block
/// costs; `w` is the instruction word.
pub(crate) fn run_op(c: &mut Cpu, vm: &mut Vm, op: u32, w: u32) -> Flow {
    let d_idx = (w >> 6) & 0x3f;
    let a_idx = (w >> 12) & 0x3ff;
    let b_idx = w >> 22;
    match op {
        // 0x04cc: `dst = the float word following the instruction`.
        0 => {
            c.cycles += 11;
            let dst = dst_of(c, vm, d_idx, 0x0032_54e0)?;
            vm.dst = dst;
            let at = vm.stream.wrapping_add(vm.idx.wrapping_shl(2));
            vm.idx = vm.idx.wrapping_add(1);
            c.g[V1][0] = sx(at);
            let v = Addr(at).read_fpu(c);
            setf(c, 0, v);
            Addr(dst).write_fpu(c, v);
        }
        // 0x04f8: `dst = operand(a)`.
        1 => {
            c.cycles += 8;
            let dst = dst_of(c, vm, d_idx, 0x0032_550c)?;
            vm.dst = dst;
            store_operand(c, vm, a_idx)?;
        }
        // 0x052c: skip `(word >> 16)` words when operand `(word >> 6) & 0x3ff`
        // is zero.
        2 => {
            c.cycles += 9;
            let v = operand(c, vm, (w >> 6) & 0x3ff, 0x0032_5540)?;
            setf(c, 1, FpuF32::ZERO);
            if fcc(c, v.eq_s(fr(c, 1))) {
                skip_words(c, vm);
            }
        }
        // 0x0550: skip forward unconditionally.
        3 => skip_words(c, vm),
        // 0x055c: `dst = (a != 0) || (b != 0)`.
        4 => {
            c.cycles += 17;
            vm.dst = 0;
            setf(c, 20, FpuF32::ZERO);
            let dst = dst_of(c, vm, d_idx, 0x0032_5578)?;
            vm.alt = dst;
            let a = operand(c, vm, a_idx, 0x0032_5590)?;
            let zero = fr(c, 20);
            let a_is_zero = fcc(c, a.eq_s(zero));
            let nonzero = if !a_is_zero {
                c.cycles += 1;
                true
            } else {
                c.cycles += 6;
                let b = operand(c, vm, b_idx, 0x0032_55ac)?;
                let b_is_zero = fcc(c, b.eq_s(zero));
                if !b_is_zero {
                    c.cycles += 1;
                }
                !b_is_zero
            };
            // 0x003255bc/0x0032562c + `mtc1 s0, f0; cvt.s.w f0, f0`.
            vm.dst = nonzero as u32;
            c.cycles += 2;
            c.cycles += 4;
            let v = setf(c, 0, FpuF32::from_i32(nonzero as i32));
            Addr(dst).write_fpu(c, v);
        }
        // 0x05cc: `dst = (a != 0) && (b != 0)`.
        5 => {
            c.cycles += 17;
            vm.dst = 0;
            setf(c, 20, FpuF32::ZERO);
            let dst = dst_of(c, vm, d_idx, 0x0032_55e8)?;
            vm.alt = dst;
            let a = operand(c, vm, a_idx, 0x0032_5600)?;
            let zero = fr(c, 20);
            let a_is_zero = fcc(c, a.eq_s(zero));
            let nonzero = if a_is_zero {
                false
            } else {
                c.cycles += 7;
                let b = operand(c, vm, b_idx, 0x0032_561c)?;
                let b_is_zero = fcc(c, b.eq_s(zero));
                !b_is_zero
            };
            // 0x003255bc/0x0032562c + `mtc1 s0, f0; cvt.s.w f0, f0`.
            vm.dst = nonzero as u32;
            c.cycles += 2;
            c.cycles += 4;
            let v = setf(c, 0, FpuF32::from_i32(nonzero as i32));
            Addr(dst).write_fpu(c, v);
        }
        // 0x0658: `dst = a > b` (the compare is `op2 < op1`).
        6 => compare(
            c,
            vm,
            w,
            [0x0032_566c, 0x0032_5684, 0x0032_5698],
            true,
            |a, b| b.lt(a),
        )?,
        // 0x06b8: `dst = a >= b` (the compare is `op2 <= op1`).
        7 => compare(
            c,
            vm,
            w,
            [0x0032_56cc, 0x0032_56e4, 0x0032_56f8],
            true,
            |a, b| b.le(a),
        )?,
        // 0x0718: `dst = a < b`.
        8 => compare(
            c,
            vm,
            w,
            [0x0032_572c, 0x0032_5744, 0x0032_5758],
            true,
            |a, b| a.lt(b),
        )?,
        // 0x0778: `dst = a <= b`.
        9 => compare(
            c,
            vm,
            w,
            [0x0032_578c, 0x0032_57a4, 0x0032_57b8],
            true,
            |a, b| a.le(b),
        )?,
        // 0x07d8: `dst = a == b`.
        10 => compare(
            c,
            vm,
            w,
            [0x0032_57ec, 0x0032_5804, 0x0032_5818],
            true,
            |a, b| a.eq_s(b),
        )?,
        // 0x0838: `dst = a != b` (the compare is an equality test).
        11 => compare(
            c,
            vm,
            w,
            [0x0032_584c, 0x0032_5864, 0x0032_5878],
            false,
            |a, b| a.eq_s(b),
        )?,
        // 0x0898: `dst = a + b` (the result stays in f20).
        12 => binary(c, vm, w, 0x0032_58ac, 0x0032_58c4, 0x0032_58d8, |a, b| {
            a + b
        })?,
        // 0x08e4: `dst = a - b`.
        13 => binary(c, vm, w, 0x0032_58f8, 0x0032_5910, 0x0032_5924, |a, b| {
            a - b
        })?,
        // 0x0930: `dst = a * b`.
        14 => binary(c, vm, w, 0x0032_5944, 0x0032_595c, 0x0032_5970, |a, b| {
            a * b
        })?,
        // 0x097c: `dst = a / b`.
        15 => binary(c, vm, w, 0x0032_5990, 0x0032_59a8, 0x0032_59bc, |a, b| {
            a.div(b)
        })?,
        // 0x09c8: `dst = max(a, b)` — the larger operand is read again.
        16 => {
            c.cycles += 20;
            let dst = dst_of(c, vm, d_idx, 0x0032_59dc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_59f4)?;
            setf(c, 20, a);
            let b = operand(c, vm, b_idx, 0x0032_5a08)?;
            let a = fr(c, 20);
            if fcc(c, b.lt(a)) {
                c.cycles += 2;
                if event_due(c) && deliver_event(c, STORE_OPERAND) {
                    return Ok(());
                }
                store_operand(c, vm, a_idx)?;
            } else {
                c.cycles += 5;
                operand(c, vm, b_idx, 0x0032_5a2c)?;
                let v = fr(c, 0);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0a34: `dst = min(a, b)`.
        17 => {
            c.cycles += 20;
            let dst = dst_of(c, vm, d_idx, 0x0032_5a48)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5a60)?;
            setf(c, 20, a);
            let b = operand(c, vm, b_idx, 0x0032_5a74)?;
            let a = fr(c, 20);
            if fcc(c, a.lt(b)) {
                c.cycles += 2;
                if event_due(c) && deliver_event(c, STORE_OPERAND) {
                    return Ok(());
                }
                store_operand(c, vm, a_idx)?;
            } else {
                c.cycles += 5;
                operand(c, vm, b_idx, 0x0032_5a98)?;
                let v = fr(c, 0);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0aa0: `dst = pow(a, b)` = `exp(log(a) * b)`.
        18 => {
            c.cycles += 21;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ab4)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5acc)?;
            setf(c, 21, a);
            let b = operand(c, vm, b_idx, 0x0032_5ae0)?;
            setf(c, 20, b);
            setf(c, 12, fr(c, 21));
            vm.spill(c);
            call_ptr(c, callee::LOGF, 0x0032_5aec)?;
            let (log, exp) = (fr(c, 0), fr(c, 20));
            setf(c, 12, log * exp);
            exp_into(c, vm, dst)?;
        }
        // 0x0af4: `atan2(a, b)`, with the two half-plane corrections.
        19 => {
            c.cycles += 21;
            let dst = dst_of(c, vm, d_idx, 0x0032_5b0c)?;
            vm.dst = dst;
            setf(c, 22, FpuF32::ZERO);
            let a = operand(c, vm, a_idx, 0x0032_5b24)?;
            setf(c, 21, a);
            let b = operand(c, vm, b_idx, 0x0032_5b38)?;
            setf(c, 20, b);
            let (b, zero) = (fr(c, 20), fr(c, 22));
            if !fcc(c, b.eq_s(zero)) {
                c.cycles += 7;
                let a = fr(c, 21);
                setf(c, 12, a.div(b));
                callee::MATH_ATAN.call(c, 0x0032_5b80)?;
                let below = fcc(c, b.lt(zero));
                let at = fr(c, 0);
                setf(c, 12, at);
                if below {
                    c.cycles += 3;
                    let a = fr(c, 21);
                    if fcc(c, zero.lt(a)) {
                        c.cycles += 3;
                        let v = gp_float(c, literal::PI, 0);
                        let t = fr(c, 12);
                        setf(c, 12, t + v);
                    } else {
                        c.cycles += 2;
                        let v = gp_float(c, literal::PI_SUB, 0);
                        let t = fr(c, 12);
                        setf(c, 12, t - v);
                    }
                }
                c.cycles += 2;
                let t = fr(c, 12);
                setf(c, 0, t);
                if event_due(c) && deliver_event(c, STORE) {
                    return Ok(());
                }
                store_result(c, vm);
            } else {
                c.cycles += 4;
                let (a, zero) = (fr(c, 21), fr(c, 22));
                let a_is_zero = fcc(c, a.eq_s(zero));
                setf(c, 0, a);
                if a_is_zero {
                    if event_due(c) && deliver_event(c, STORE) {
                        return Ok(());
                    }
                    store_result(c, vm);
                } else {
                    c.cycles += 4;
                    let (a, zero) = (fr(c, 21), fr(c, 22));
                    let above = fcc(c, zero.le(a));
                    if !above {
                        let v = gp_float(c, literal::HALF_PI_NEG, 0);
                        Addr(dst).write_fpu(c, v);
                    } else {
                        c.cycles += 3;
                        let v = gp_float(c, literal::HALF_PI, 0);
                        Addr(dst).write_fpu(c, v);
                    }
                }
            }
        }
        // 0x0bb8: `dst = atan(a)`.
        20 => unary(
            c,
            vm,
            w,
            0x0032_5bcc,
            0x0032_5be4,
            &callee::MATH_ATAN,
            0x0032_5bec,
        )?,
        // 0x0bf4: `dst = sin(a)`.
        21 => unary(
            c,
            vm,
            w,
            0x0032_5c08,
            0x0032_5c20,
            &callee::MATH_SIN,
            0x0032_5c28,
        )?,
        // 0x0c30: `dst = cos(a)`.
        22 => unary(
            c,
            vm,
            w,
            0x0032_5c44,
            0x0032_5c5c,
            &callee::MATH_COS,
            0x0032_5c64,
        )?,
        // 0x0c6c: `dst = exp(a)`.
        23 => {
            c.cycles += 12;
            let dst = dst_of(c, vm, d_idx, 0x0032_5c80)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5c98)?;
            setf(c, 12, a);
            exp_into(c, vm, dst)?;
        }
        // 0x0cac: `dst = log(a)`.
        24 => {
            c.cycles += 15;
            let dst = dst_of(c, vm, d_idx, 0x0032_5cc0)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5cd8)?;
            setf(c, 12, a);
            vm.spill(c);
            call_ptr(c, callee::LOGF, 0x0032_5ce0)?;
            let v = fr(c, 0);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0ce8: `dst = floor(a)`.
        25 => {
            c.cycles += 17;
            let dst = dst_of(c, vm, d_idx, 0x0032_5cfc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5d14)?;
            let n = setf(c, 1, FpuF32::from_i32(a.to_i32()));
            if fcc(c, a.lt(n)) {
                c.cycles += 4;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n - one);
                c.cycles += 2;
            }
            let v = fr(c, 1);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0d3c: `dst = ceil(a)`.
        26 => {
            c.cycles += 17;
            let dst = dst_of(c, vm, d_idx, 0x0032_5d50)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5d68)?;
            let n = setf(c, 1, FpuF32::from_i32(a.to_i32()));
            if fcc(c, n.lt(a)) {
                c.cycles += 4;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n + one);
                c.cycles += 2;
            }
            let v = fr(c, 1);
            Addr(dst).write_fpu(c, v);
        }
        // 0x0d90: `dst = abs(a)`.
        27 => {
            c.cycles += 14;
            let dst = dst_of(c, vm, d_idx, 0x0032_5da4)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5dbc)?;
            let v = setf(c, 0, a.abs());
            Addr(dst).write_fpu(c, v);
        }
        // 0x0dc8: `dst = sqrt(a)`.
        28 => {
            c.cycles += 14;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ddc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5df4)?;
            let v = setf(c, 0, a.sqrt());
            Addr(dst).write_fpu(c, v);
        }
        // 0x0e00: `dst = a - round(a / 2pi) * 2pi` (wrap into (-pi, pi]).
        29 => {
            c.cycles += 22;
            let dst = dst_of(c, vm, d_idx, 0x0032_5e14)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5e2c)?;
            setf(c, 2, a);
            li_f(c, 1, 0x3f00_0000);
            let inv = gp_float(c, literal::INV_TWO_PI, 0);
            let turns = setf(c, 0, a * inv);
            let half = fr(c, 1);
            let scaled = setf(c, 0, turns + half);
            let n = setf(c, 1, FpuF32::from_i32(scaled.to_i32()));
            if fcc(c, scaled.lt(n)) {
                c.cycles += 3;
                li_f(c, 0, 0x3f80_0000);
                let one = fr(c, 0);
                setf(c, 1, n - one);
            }
            c.cycles += 4;
            let two_pi = gp_float(c, literal::TWO_PI, 0);
            let n = fr(c, 1);
            let whole = setf(c, 0, n * two_pi);
            let a = fr(c, 2);
            setf(c, 0, a - whole);
            if event_due(c) && deliver_event(c, STORE) {
                return Ok(());
            }
            store_result(c, vm);
        }
        // 0x0e74: `dst = -a`.
        30 => {
            c.cycles += 13;
            let dst = dst_of(c, vm, d_idx, 0x0032_5e88)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5ea0)?;
            setf(c, 0, -a);
            if event_due(c) && deliver_event(c, STORE) {
                return Ok(());
            }
            store_result(c, vm);
        }
        // 0x0ea8: `dst = (a != 0) ? 1.0 : 0.0`.
        31 => {
            c.cycles += 16;
            let dst = dst_of(c, vm, d_idx, 0x0032_5ebc)?;
            vm.dst = dst;
            let a = operand(c, vm, a_idx, 0x0032_5ed4)?;
            setf(c, 1, FpuF32::ZERO);
            let a_is_zero = fcc(c, a.eq_s(fr(c, 1)));
            if !a_is_zero {
                let v = fr(c, 1);
                Addr(dst).write_fpu(c, v);
            } else {
                c.cycles += 2;
                li_f(c, 1, 0x3f80_0000);
                c.cycles += 2;
                let v = fr(c, 1);
                Addr(dst).write_fpu(c, v);
            }
        }
        // 0x0ef8: stop.
        32 => {
            c.cycles += 1;
            vm.stop = 1;
        }
        _ => {}
    }
    Ok(())
}

/// Opcodes 0x02/0x03: `idx += *(u16*)(ins + 2)`.
pub(crate) fn skip_words(c: &mut Cpu, vm: &mut Vm) {
    c.cycles += 3;
    let n = c.lhu(vm.ins.wrapping_add(2)) as u32;
    c.g[V0][0] = sx(n);
    vm.idx = vm.idx.wrapping_add(n);
}

/// Opcodes 6..11: `dst = (op1 OP op2) ? 1.0 : 0.0`, where `one_if_compare`
/// says which side of the compare stores 1.0 (0x0b tests equality and stores
/// 1.0 when it fails).
pub(crate) fn compare(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    rets: [u32; 3],
    one_if_compare: bool,
    cond: fn(FpuF32, FpuF32) -> bool,
) -> Flow {
    c.cycles += 20;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, rets[0])?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, rets[1])?;
    setf(c, 20, a);
    let b = operand(c, vm, w >> 22, rets[2])?;
    let a = fr(c, 20);
    let held = fcc(c, cond(a, b));
    if held == one_if_compare {
        c.cycles += 4;
        li_f(c, 0, 0x3f80_0000);
    } else {
        setf(c, 0, FpuF32::ZERO);
    }
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// Opcodes 0x0c..0x0f: `dst = op1 OP op2`, left in `f20`.
pub(crate) fn binary(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    ret_dst: u32,
    ret_a: u32,
    ret_b: u32,
    op: fn(FpuF32, FpuF32) -> FpuF32,
) -> Flow {
    c.cycles += 19;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, ret_dst)?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, ret_a)?;
    setf(c, 20, a);
    let b = operand(c, vm, w >> 22, ret_b)?;
    let a = fr(c, 20);
    let r = setf(c, 20, op(a, b));
    Addr(dst).write_fpu(c, r);
    Ok(())
}

/// Opcodes 0x14..0x16: `dst = f(a)` through one of the maths callees.
pub(crate) fn unary(
    c: &mut Cpu,
    vm: &mut Vm,
    w: u32,
    ret_dst: u32,
    ret_arg: u32,
    target: &Callee,
    ret_call: u32,
) -> Flow {
    c.cycles += 15;
    let dst = dst_of(c, vm, (w >> 6) & 0x3f, ret_dst)?;
    vm.dst = dst;
    let a = operand(c, vm, (w >> 12) & 0x3ff, ret_arg)?;
    setf(c, 12, a);
    target.call(c, ret_call)?;
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// The `0x00325c9c` tail shared by opcodes 0x12 and 0x17: `dst = exp(f12)`.
pub(crate) fn exp_into(c: &mut Cpu, vm: &Vm, dst: u32) -> Flow {
    c.cycles += 4;
    vm.spill(c);
    call_ptr(c, callee::EXPF, 0x0032_5ca4)?;
    let v = fr(c, 0);
    Addr(dst).write_fpu(c, v);
    Ok(())
}

/// `void InputPreproc_SkipLine(InputPreproc* p_4, Stream* s)` (0x326678): consume
/// characters up to and including the next newline (0x0a), or until the
/// stream ends; then advance the stream's line counter.
pub fn input_preproc_skip_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xa;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
        loop {
            c.cycles += 2;
            let eof = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S0][0];
            if eof {
                break;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            call::call_ptr(c, 0x0032_6478, 0x0032_66b0)?;
            c.g[V0][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
            let not_newline = c.g[V0][0] != c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !not_newline {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0032_6478, 0x0032_66c4)?;
                c.g[V0][0] = Addr(p_4(r32(c, S1), 0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(p_4(r32(c, S1), 0xc)).sw(c, c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_66a0);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void PadDevice_StopActuators(PadDevice* d)` (0x326cf0): call the actuator
/// setter with 0.0f for every actuator the device reports.
pub fn pad_device_stop_actuators(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0032_6cf0 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0;
                    pc = 0x0032_6d24;
                }
                0x0032_6d0c => {
                    c.cycles += 1;
                    pc = 0x0032_6d10;
                }
                0x0032_6d10 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x20)).lh(c);
                    c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d24)?;
                    pc = 0x0032_6d24;
                }
                0x0032_6d24 => {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V1][0] as u32, 0x18)).lh(c);
                    c.g[V0][0] = Addr(p_4(c.g[V1][0] as u32, 0x1c)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                    call::call_ptr(c, target, 0x0032_6d38)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.mtc1(0, 12);
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    if more {
                        if crate::events::back_edge_unwinds(c, 0x0032_6d10) {
                            return Ok(());
                        }
                        pc = 0x0032_6d10;
                    } else {
                        pc = 0x0032_6d4c;
                    }
                }
                0x0032_6d4c => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032_6cf0, 0x0032_6d60);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// List walks and buffer management.

/// `func_00326A68` (0x326a68): shut down the pad queue. For each of the
/// `count` entries at `this+0x2eec` (a null entry is skipped) call its virtual
/// destructor (`vtable[8]`, flag 3). The loop's back edge carries an event
/// check. Signature: `void shutdown(PadQueue* this)`.
pub fn pad_queue_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_r(c, S2, 0x2ee8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x2eec));
            loop {
                c.cycles += 3;
                c.g[A2][0] = lw_r(c, S0, 0);
                let skip = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !skip {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(low(c, A2)).lw(c);
                    c.g[A0][0] = lh_r(c, V0, 0x8);
                    c.g[V1][0] = lw_r(c, V0, 0xc);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0032_6ab0);
                    c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0032_6ab0)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_r(c, S2, 0x2ee8);
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6a90);
                }
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

// ---------------------------------------------------------------------------
// Input parser.

/// `InputEntry* InputParser_FindIdentifier(InputParser* self, char* name)`
/// (0x321a40): look `name` up in the parser's 0x44-byte entry table.
pub fn input_parser_find_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x8000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xe4)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if !empty {
            c.cycles += 4;
            c.g[V0][0] = 0x80e8;
            c.g[S2][0] = c.g[V1][0];
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_aa88, 0x0032_1a8c)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    c.cycles += 7;
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.g[V0][0] = Addr(p(r32(c, S2), 0xe4)).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(p(r32(c, S0), 0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1a80);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0;
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

/// `int PS2Pad_Create(PS2Pad** out, int n)` (0x326d60): allocate and build up
/// to two pads, storing them in `out`, and return how many were made.
pub fn ps2_pad_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let out = abi::arg(c, 0);
        let n = abi::arg64(c, 1);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = 2;
        frame.save(c, S1, 0x30);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0;
        let none = (c.g[S2][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[S1][0] = sx(out);
            c.g[S3][0] = 0x004a_0000;
            loop {
                c.cycles += 14;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx(p(r32(c, S3), 0x3fb0));
                c.g[A2][0] = 0x400_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC, 0x0032_6dac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                call_ptr(c, callee::PS2_PAD_CTOR, 0x0032_6dc0)?;
                c.sw(r32(c, S1), c.g[V0][0]);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6d98);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InputParser_LookupModifier(const char* name, void* parser)` (0x3217d0):
/// map a modifier name to its index (0..3), the "any" modifier (4), a
/// fallback (6) or "no modifier" (0xff... never: 0 for the first name).
pub fn input_parser_lookup_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let name = c.g[A1][0];
        c.g[S0][0] = name;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3ef8;
        call_ptr(c, 0x0041_aa88, 0x0032_17f0)?;
        c.g[A1][0] = 0x4a_0000;
        let differs = c.g[V0][0] != 0;
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f00;
        call_ptr(c, 0x0041_aa88, 0x0032_180c)?;
        c.g[A1][0] = 0x4a_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f08;
        call_ptr(c, 0x0041_aa88, 0x0032_1828)?;
        c.g[A1][0] = 0x49_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 2;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x48ddd8;
        call_ptr(c, 0x0041_aa88, 0x0032_1844)?;
        c.g[A1][0] = 0x4a_0000;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 3;
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A0][0] = name;
        c.g[A1][0] = 0x4a3f10;
        call_ptr(c, 0x0041_aa88, 0x0032_1858)?;
        c.g[A0][0] = 0x4;
        c.g[V1][0] = 0x6;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input preprocessor.

/// `void InputPreproc_SkipLineComment(InputPreproc* p, Cursor* cur, void* a,
/// void* b)` (0x3265c0): copy the rest of a `//` comment line to the output,
/// stopping at the newline (which is written) or at end of input.
pub fn input_preproc_skip_line_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let pre = r(c, A0);
        c.g[S0][0] = sx(pre);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xa;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[V0][0] = Addr(p(pre, 0xc)).lw(c);
        'outer_scan: loop {
            'body: {
                'newline: {
                    c.cycles += 2;
                    let is_nl = c.g[V0][0] != c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    if is_nl {
                        break 'newline;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    Addr(p(r(c, S1), 0xc)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x0032_6614);
                    c.g[A1][0] = Addr(p(r(c, S0), 0xc)).lb(c);
                    callee::INPUT_PREPROC_WRITE.call(c, 0x0032_6614)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0032_6620);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6620)?;
                    frame.restore(c, 16, 0x50);
                    break 'body;
                }
                'eof: {
                    c.cycles += 2;
                    let at_end = (c.g[V0][0] as i64) < 0;
                    c.g[T0][0] = sx(0x0049_0000);
                    if at_end {
                        break 'eof;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0032_6638);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6638)?;
                    c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_65f8);
                    }
                    continue 'outer_scan;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0032_6658);
                c.g[T0][0] = sx(r(c, T0).wrapping_sub(0x1c28));
                callee::INPUT_PREPROC_EMPTY_STUB.call(c, 0x0032_6658)?;
                frame.restore(c, 16, 0x50);
                break 'body;
            }
            c.cycles += 7;
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `int InputParser_CompileIdentifier(InputParser *self, ...)` (0x3245a8):
/// look the identifier up and either emit its node or raise the parser error.
pub fn input_parser_compile_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[T0][0];
        call_ptr(c, 0x0032_1a40, 0x0032_45dc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A1][0] = 0x0048_e208;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, 0x0032_1500, 0x0032_45f8)?;
            c.g[V0][0] = u64::MAX;
        } else {
            c.g[A1][0] = Addr(p_5(c.g[V0][0], 0x40)).lw(c);
            c.cycles += 12;
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[T1][0] = c.g[S3][0];
            Addr(frame.at(c, 0x0)).sw(c, 0);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0032_4678, 0x0032_461c)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let empty = (c.g[S0][0] as i64) <= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if empty {
                c.cycles += 5;
                c.g[A1][0] = 0x0048_e228;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1500, 0x0032_4650)?;
                c.g[V0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x20;
                Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_Write(InputPreproc *self, int c)` (0x326360): append a
/// character to the growing buffer.
pub fn input_preproc_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x4)).lw(c);
        let not_full = c.g[V1][0] != c.g[V0][0];
        c.g[S3][0] = sx((((c.g[A1][0] as u32) as i32) >> 24) as u32);
        if !not_full {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x400));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = 0x0048_e3b8;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = 0;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 1) as u32);
            c.g[A3][0] = 0;
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[S1][0] = c.g[V0][0];
            }
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e30, 0x0032_63c8)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
            c.g[A2][0] = Addr(p_5(c.g[S0][0], 0x4)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0041_605c, 0x0032_63dc)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0032_63f0)?;
            }
            Addr(p_5(c.g[S0][0], 0x8)).sw(c, c.g[S2][0]);
            c.cycles += 2;
            Addr(p_5(c.g[S0][0], 0x4)).sw(c, c.g[S1][0]);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        c.cycles += 12;
        c.g[V0][0] = Addr(p_5(c.g[S0][0], 0x8)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore(c, S2, 0x20);
        Addr(c.g[V0][0] as u32).sb(c, c.g[S3][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod ps2pad {

    pub const PORT: u32 = 0x04;

    pub const SLOT: u32 = 0x08;

    /// Requested small / big motor values and the last ones sent.
    pub const ACT_SMALL: u32 = 0x30;

    pub const LAST_SMALL: u32 = 0x34;

    pub const ACT_BIG: u32 = 0x38;

    pub const LAST_BIG: u32 = 0x3c;

    /// The 6-byte actuator block handed to `scePadSetActDirect`.
    pub const ACT_DIRECT: u32 = 0x40;

    /// Connection state machine phase (0 = port closed, 10 = reading).
    pub const PHASE: u32 = 0x48;

    pub const RETRY: u32 = 0x4c;

    pub const MODE_SET: u32 = 0x50;

    /// Pressure-sensitive buttons enabled (else digital ones).
    pub const PRESSURE: u32 = 0x54;

    pub const ANALOG: u32 = 0x58;

    pub const ACTUATOR: u32 = 0x5c;

    /// The 64-byte aligned DMA buffer given to `scePadPortOpen`.
    pub const DMA_BUF: u32 = 0x80;

    pub const PORT_OPEN: u32 = 0x003f_f708;

    pub const GET_STATE: u32 = 0x003f_fbc0;

    pub const GET_REQ_STATE: u32 = 0x003f_fcd8;

    pub const INFO_ACT: u32 = 0x003f_fd60;

    pub const INFO_MODE: u32 = 0x003f_ffa0;

    pub const SET_MAIN_MODE: u32 = 0x0040_00d8;

    pub const SET_ACT_DIRECT: u32 = 0x0040_0190;

    pub const SET_ACT_ALIGN: u32 = 0x0040_0250;

    pub const INFO_PRESS_MODE: u32 = 0x0040_0490;

    pub const ENTER_PRESS_MODE: u32 = 0x0040_04f0;

    pub const READ: u32 = 0x003f_fa58;

    /// The `switch (phase)` jump table in `.rodata` and its case labels
    /// (phases 1..=10).
    pub const JUMP_TABLE: u32 = 0x0048_e460;

    pub const CASES: [u32; 10] = [
        0x0032_6f70,
        0x0032_7028,
        0x0032_707c,
        0x0032_709c,
        0x0032_70c8,
        0x0032_70e8,
        0x0032_712c,
        0x0032_715c,
        0x0032_7188,
        0x0032_71d8,
    ];

    /// `D_004A3FB8`, the actuator alignment (gp-relative).
    pub const ACT_ALIGN_GP: u32 = 0xec8;

    /// `1/255` scale for pressure bytes and for the stick ramps (gp-relative).
    pub const PRESSURE_SCALE_GP: u32 = 0x2e0c;

    pub const STICK_SCALE_GP: u32 = 0x2e08;

    /// Number of output floats written when the read succeeded.
    pub const OUT_COUNT: u64 = 0x18;
}

/// 1.0f
pub(crate) const PAD_ONE: u32 = 0x3f80_0000;

/// One active-low digital button as a float: `f0` = 1.0 when its bit is
/// clear (pressed), else 0.0; stored at `dst`. The caller charges the test.
pub(crate) fn pad_button(c: &mut Cpu, bits: u64, mask: u64, dst: u32) {
    c.f[0] = 0;
    if bits & mask == 0 {
        c.cycles += 3;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
    c.swc1(0, dst);
}

/// `mtc1 zero, f0` then 1.0 into `f0` (2 cycles) when the button is
/// pressed; no store (the next block stores it).
pub(crate) fn pad_button_late(c: &mut Cpu, pressed: bool) {
    c.f[0] = 0;
    if pressed {
        c.cycles += 2;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
}

/// `div n, 0x4f` (pipeline 0): sets LO/HI, returns the quotient.
pub(crate) fn pad_div79(c: &mut Cpu, n: u64) -> u64 {
    let n = n as i32;
    c.lo[0] = sx((n / 0x4f) as u32);
    c.hi[0] = sx((n % 0x4f) as u32);
    c.lo[0]
}

/// One stick axis byte: the two ramp numerators `(0x4f - b) * 255` and
/// `(b - 0xb0) * 255` (computed `x << 8 - x`) and their `/ 0x4f`
/// quotients clamped at 0 (the clamp flag is `-1 < q`). Returns
/// `(neg_quotient, neg_flag, pos_quotient, pos_flag, pos_delta)`.
pub(crate) fn pad_stick(c: &mut Cpu, b: u64) -> (u64, u64, u64, u64, u64) {
    let neg = sx(0x4fu32.wrapping_sub(b as u32));
    let pos = sx((b as u32).wrapping_sub(0xb0));
    let neg_num = sx(((neg as u32) << 8).wrapping_sub(neg as u32));
    let pos_num = sx(((pos as u32) << 8).wrapping_sub(pos as u32));
    let qn = pad_div79(c, neg_num);
    let qp = pad_div79(c, pos_num);
    let fneg = ((qn as i64) > -1) as u64;
    let fpos = ((qp as i64) > -1) as u64;
    (
        if fneg == 0 { 0 } else { qn },
        fneg,
        if fpos == 0 { 0 } else { qp },
        fpos,
        pos,
    )
}

/// `int PS2Pad_Update(PS2Pad* pad, float* out)` (0x326eb0): one frame of
/// a controller. Opens the port if needed, then steps the libpad
/// connection state machine (`pad->phase`): probe the controller id and
/// mode (DualShock 2 / analog), lock analog mode, enter pressure mode,
/// query and align the vibration motors, and finally read (phase 10). On
/// a good read it fills 24 floats in `out`: d-pad, the 12 face/shoulder
/// buttons (pressure bytes / 255 when pressure mode is on, else 0/1 from
/// the active-low bits) and the 8 stick half-axes (ramp outside the dead
/// zone 0x4f..0xb0, scaled by 1/255), and returns 0x18; else returns 0.
/// While reading, sends changed motor values with `scePadSetActDirect`.
pub fn ps2_pad_update(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use ps2pad::*;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let pad = Addr(c.g[S0][0] as u32);
        let out = c.g[S1][0] as u32;
        let gp = c.g[GP][0] as u32;

        // Phase 0: open the port; success resets the state.
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;
        c.g[S3][0] = 0;
        if phase != 0 {
            c.cycles += 1;
        } else {
            c.cycles += 8;
            c.g[A0][0] = pad.off(PORT).lw(c);
            c.g[A2][0] = sx(pad.0.wrapping_add(DMA_BUF));
            c.g[A1][0] = pad.off(SLOT).lw(c);
            call_ptr(c, PORT_OPEN, 0x0032_6eec)?;
            let opened = c.g[V0][0];
            c.g[V1][0] = opened;
            c.g[V0][0] = 1;
            if opened == 1 {
                c.cycles += 7 + 1;
                c.g[V0][0] = u64::MAX;
                pad.off(PHASE).sw(c, opened);
                pad.off(LAST_BIG).sw(c, u64::MAX);
                pad.off(MODE_SET).sw(c, 0);
                pad.off(ACT_SMALL).sw(c, 0);
                pad.off(LAST_SMALL).sw(c, u64::MAX);
                pad.off(ACT_BIG).sw(c, 0);
            }
        }
        c.g[A1][0] = pad.off(SLOT).lw(c);

        // Disconnected (0) or error (7): start over at probing.
        c.cycles += 5;
        c.g[A0][0] = pad.off(PORT).lw(c);
        call_ptr(c, GET_STATE, 0x0032_6f24)?;
        let state = c.g[V0][0];
        c.g[A1][0] = state;
        c.g[V0][0] = 7;
        if state != 0 {
            c.cycles += 2;
        }
        if state == 0 || state == 7 {
            c.cycles += 4;
            c.g[V0][0] = 1;
            pad.off(MODE_SET).sw(c, 0);
            pad.off(PHASE).sw(c, 1);
        }
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;

        c.cycles += 4;
        let index = sx((phase as u32).wrapping_sub(1));
        c.g[A0][0] = index;
        c.g[V1][0] = (index < 10) as u64;
        c.g[V0][0] = 0x0049_0000;
        let mut read_ok = false;
        if index >= 10 {
            c.cycles += 1;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(JUMP_TABLE.wrapping_add((index as u32) << 2));
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[A0][0] = CASES[index as usize] as u64;
            let port = |c: &mut Cpu| pad.off(PORT).lw(c);
            let slot = |c: &mut Cpu| pad.off(SLOT).lw(c);
            match index + 1 {
                // Stable (6) or busy (2): probe the controller id and mode.
                1 => {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    let mut probe = state == 6;
                    if !probe {
                        // `bne a1, 2` with `move s2, zero` in its delay
                        // slot: s2 is 0 at the InfoMode call (callees
                        // save it on their frames)
                        c.cycles += 2;
                        c.g[S2][0] = 0;
                        probe = state == 2;
                    }
                    if probe {
                        c.cycles += 11;
                        pad.off(PRESSURE).sw(c, 0);
                        c.g[A2][0] = 1;
                        pad.off(ANALOG).sw(c, 0);
                        c.g[A3][0] = 0;
                        pad.off(ACTUATOR).sw(c, 0);
                        c.g[A0][0] = port(c);
                        c.g[A1][0] = slot(c);
                        call_ptr(c, INFO_MODE, 0x0032_6fa4)?;
                        let id = c.g[V0][0];
                        c.g[S2][0] = id;
                        c.g[A2][0] = 2;
                        if id == 0 {
                            c.cycles += 1;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = port(c);
                            c.g[A1][0] = slot(c);
                            c.g[A3][0] = 0;
                            call_ptr(c, INFO_MODE, 0x0032_6fc0)?;
                            let ext = c.g[V0][0];
                            c.g[V1][0] = ext;
                            c.g[A0][0] = 4;
                            c.g[V0][0] = ((ext as i64) < 5) as u64;
                            if ext == 4 {
                                // digital DualShock: set the analog mode
                                c.cycles += 3 + 1;
                                c.g[V0][0] = 2;
                                pad.off(PHASE).sw(c, 2);
                            } else if (ext as i64) >= 5 {
                                c.cycles += 2 + 2;
                                c.g[V0][0] = 7;
                                if ext == 7 {
                                    // analog DualShock
                                    c.cycles += 5 + 1;
                                    c.g[V0][0] = 1;
                                    c.g[V1][0] = 2;
                                    pad.off(ANALOG).sw(c, 1);
                                    pad.off(PHASE).sw(c, 2);
                                } else {
                                    c.cycles += 2;
                                }
                            } else {
                                c.cycles += 2 + 2;
                                if ext != 0 {
                                    c.cycles += 2;
                                } else {
                                    c.cycles += 2;
                                    if id == 4 {
                                        // plain digital pad: read directly
                                        c.cycles += 2 + 2 + 1;
                                        c.g[V0][0] = 0xa;
                                        pad.off(PHASE).sw(c, 0xa);
                                    }
                                }
                            }
                        }
                    }
                }
                // Lock analog mode (unless already set for an analog pad).
                2 => {
                    c.cycles += 3;
                    let analog = pad.off(ANALOG).lw(c);
                    c.g[V0][0] = analog;
                    let mut set_mode = true;
                    if analog != 0 {
                        c.cycles += 3;
                        let mode_set = pad.off(MODE_SET).lw(c);
                        c.g[V0][0] = 4;
                        if mode_set != 0 {
                            set_mode = false;
                            c.cycles += 2 + 1;
                            pad.off(PHASE).sw(c, 4);
                        } else {
                            c.cycles += 1;
                        }
                    }
                    if set_mode {
                        c.g[A0][0] = port(c);
                        c.cycles += 8;
                        c.g[A2][0] = 1;
                        c.g[A1][0] = slot(c);
                        c.g[A3][0] = 3;
                        call_ptr(c, SET_MAIN_MODE, 0x0032_7054)?;
                        let r = c.g[V0][0];
                        c.g[V1][0] = r;
                        c.g[V0][0] = 1;
                        if r == 1 {
                            c.cycles += 4;
                            c.g[V0][0] = 3;
                            pad.off(MODE_SET).sw(c, r);
                            pad.off(PHASE).sw(c, 3);
                        }
                    }
                }
                // Wait for the mode change to finish.
                3 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_STATE, 0x0032_7088)?;
                    c.g[V1][0] = 5;
                    let done = c.g[V0][0] == 5;
                    c.g[V0][0] = 1;
                    if !done {
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
                // Pressure mode available?
                4 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, INFO_PRESS_MODE, 0x0032_70a8)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] != 1 {
                        c.g[V0][0] = 7;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 7);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 5;
                        pad.off(PHASE).sw(c, 5);
                    }
                    c.cycles += 1;
                }
                // Enter pressure mode.
                5 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, ENTER_PRESS_MODE, 0x0032_70d4)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 2 + 3 + 1;
                        c.g[V0][0] = 6;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 6);
                    }
                }
                // Wait for the request; 5 failures: pressure mode on.
                6 | 9 => {
                    let ret = if index + 1 == 6 {
                        0x0032_70f4
                    } else {
                        0x0032_7194
                    };
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_REQ_STATE, ret)?;
                    c.g[A0][0] = 1;
                    let req = c.g[V0][0];
                    if req == 1 {
                        if index + 1 == 9 {
                            c.cycles += 1;
                        }
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                        pad.off(MODE_SET).sw(c, 0);
                    } else {
                        c.cycles += 2;
                        if req != 0 {
                            pad.off(RETRY).sw(c, 0);
                        } else {
                            c.cycles += 5;
                            let tries = sx((pad.off(RETRY).lw(c) as u32).wrapping_add(1));
                            c.g[V0][0] = tries;
                            c.g[V1][0] = ((tries as i64) < 5) as u64;
                            pad.off(RETRY).sw(c, tries);
                            if (tries as i64) >= 5 {
                                c.cycles += 4;
                                if index + 1 == 6 {
                                    c.g[V0][0] = 7;
                                    pad.off(PRESSURE).sw(c, 1);
                                    pad.off(PHASE).sw(c, 7);
                                } else {
                                    c.g[V0][0] = 0xa;
                                    pad.off(ACTUATOR).sw(c, 1);
                                    pad.off(PHASE).sw(c, 0xa);
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                }
                // Motors present? Align them, else go reading.
                7 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = u64::MAX;
                    c.g[A1][0] = slot(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, INFO_ACT, 0x0032_7140)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1 + 2;
                        c.g[V0][0] = 0xa;
                        pad.off(PHASE).sw(c, 0xa);
                    } else {
                        c.g[V0][0] = 8;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 8);
                    }
                    c.cycles += 1;
                }
                8 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = sx(gp.wrapping_add(ACT_ALIGN_GP));
                    c.g[A1][0] = slot(c);
                    call_ptr(c, SET_ACT_ALIGN, 0x0032_716c)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 1 + 3 + 1;
                        c.g[V0][0] = 9;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 9);
                    }
                }
                // Read; a bad read or a non-zero status byte reprobes.
                _ => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = slot(c);
                    call_ptr(c, READ, 0x0032_71e8)?;
                    let len = c.g[V0][0];
                    let status = Addr(sp(c)).lbu(c);
                    c.g[V0][0] = status;
                    if len != 0 {
                        c.cycles += 2;
                        read_ok = status == 0;
                    }
                    if read_ok {
                        c.g[S3][0] = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        pad.off(MODE_SET).sw(c, 0);
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
            }
        }
        c.g[S2][0] = 0;

        c.cycles += 2;
        let data = Addr(sp(c));
        c.g[V0][0] = data.off(2).lbu(c);
        let mut result = 0;
        if read_ok {
            result = OUT_COUNT;
            c.g[S2][0] = OUT_COUNT;
            let b2 = data.off(2).lbu(c);
            // d-pad
            c.cycles += 5;
            pad_button_late(c, b2 & 0x01 == 0);
            c.cycles += 6;
            c.g[V1][0] = b2;
            c.swc1(0, out);
            pad_button(c, b2, 0x08, out + 0x04);
            c.cycles += 4;
            pad_button(c, b2, 0x02, out + 0x08);
            c.cycles += 4;
            pad_button(c, b2, 0x04, out + 0x0c);

            c.cycles += 3;
            if pad.off(PRESSURE).lw(c) != 0 {
                // 12 pressure bytes * (1/255)
                c.cycles += 55 + 6;
                let scale = Addr(gp.wrapping_sub(PRESSURE_SCALE_GP)).read_fpu(c);
                // destination FPRs of bytes 8..=0x13, in order
                const FREGS: [usize; 12] = [2, 3, 7, 8, 9, 10, 11, 12, 6, 4, 5, 1];
                for (i, &f) in FREGS.iter().enumerate() {
                    let b = data.off(8 + i as u32).lbu(c);
                    let v = FpuF32::from_i32(b as i32) * scale;
                    c.f[f] = v.0;
                    Addr(out + 0x10 + 4 * i as u32).write_fpu(c, v);
                }
                c.f[0] = scale.0;
                c.g[V0][0] = data.off(0x12).lbu(c);
                c.g[V1][0] = data.off(0x13).lbu(c);
                c.g[A0][0] = data.off(0x0e).lbu(c);
                c.g[A1][0] = data.off(0x0f).lbu(c);
                c.g[A2][0] = data.off(0x10).lbu(c);
                c.g[A3][0] = data.off(0x11).lbu(c);
            } else {
                // digital face and shoulder buttons
                c.cycles += 4;
                pad_button(c, b2, 0x20, out + 0x10);
                c.cycles += 4;
                pad_button(c, b2, 0x80, out + 0x14);
                c.cycles += 4;
                pad_button(c, b2, 0x10, out + 0x18);
                c.cycles += 4;
                let b3 = data.off(3).lbu(c);
                pad_button_late(c, b2 & 0x40 == 0);
                c.cycles += 5;
                c.g[V1][0] = b3;
                c.swc1(0, out + 0x1c);
                pad_button(c, b3, 0x10, out + 0x20);
                c.cycles += 4;
                pad_button(c, b3, 0x20, out + 0x24);
                c.cycles += 4;
                pad_button(c, b3, 0x40, out + 0x28);
                c.cycles += 4;
                pad_button(c, b3, 0x80, out + 0x2c);
                c.cycles += 4;
                pad_button(c, b3, 0x04, out + 0x30);
                c.cycles += 4;
                pad_button_late(c, b3 & 0x08 == 0);
                c.cycles += 6;
                c.swc1(0, out + 0x34);
                pad_button(c, b3, 0x01, out + 0x38);
                c.cycles += 4;
                pad_button(c, b3, 0x02, out + 0x3c);
                c.g[V0][0] = b3 & 0x02;
            }

            // stick half-axes
            c.cycles += 3;
            let analog = pad.off(ANALOG).lw(c);
            c.g[V0][0] = analog;
            c.g[V1][0] = data.off(4).lbu(c);
            if analog == 0 {
                c.cycles += 8;
                for i in 0..8 {
                    Addr(out + 0x40 + 4 * i).sw(c, 0);
                }
            } else {
                c.cycles += 6 + 103;
                let scale = Addr(gp.wrapping_sub(STICK_SCALE_GP)).read_fpu(c);
                c.f[2] = scale.0;
                // bytes 4..=7: left x, left y, right x, right y
                let axis = |c: &mut Cpu, i: u32| {
                    let b = data.off(4 + i).lbu(c);
                    let ramps = pad_stick(c, b);
                    let neg = FpuF32::from_i32(ramps.0 as i32) * scale;
                    let pos = FpuF32::from_i32(ramps.2 as i32) * scale;
                    Addr(out + 0x40 + 8 * i).write_fpu(c, neg);
                    Addr(out + 0x44 + 8 * i).write_fpu(c, pos);
                    c.f[0] = neg.0;
                    c.f[1] = pos.0;
                    ramps
                };
                axis(c, 0);
                axis(c, 1);
                axis(c, 2);
                let (qn, fneg, qp, fpos, pos) = axis(c, 3);
                c.g[V1][0] = qn;
                c.g[A0][0] = fneg;
                c.g[V0][0] = qp;
                c.g[A3][0] = fpos;
                c.g[A1][0] = pos;
                c.g[A2][0] = 0x4f;
                for r in [T0, T1, T2, T3, T4, T5] {
                    c.g[r][0] = 0x4f;
                }
            }
        }

        // While reading: send changed motor values.
        c.cycles += 4;
        c.g[V1][0] = pad.off(PHASE).lw(c);
        c.g[V0][0] = result;
        if c.g[V1][0] == 0xa {
            c.cycles += 4;
            let act_small = pad.off(ACT_SMALL).lw(c);
            c.g[T0][0] = act_small;
            c.g[V0][0] = pad.off(LAST_SMALL).lw(c);
            let mut changed = act_small != c.g[V0][0];
            if !changed {
                c.cycles += 4;
                c.g[V1][0] = pad.off(ACT_BIG).lw(c);
                let last_big = pad.off(LAST_BIG).lw(c);
                c.g[V0][0] = result;
                if c.g[V1][0] != last_big {
                    c.cycles += 1;
                    changed = true;
                }
            }
            if changed {
                c.g[V0][0] = pad.off(ACT_SMALL).lbu(c);
                c.cycles += 11;
                c.g[A2][0] = sx(pad.0.wrapping_add(ACT_DIRECT));
                c.g[V1][0] = pad.off(ACT_BIG).lbu(c);
                c.g[A3][0] = pad.off(ACT_BIG).lw(c);
                c.g[A0][0] = pad.off(PORT).lw(c);
                c.g[A1][0] = pad.off(SLOT).lw(c);
                let (small, big) = (c.g[V0][0], c.g[V1][0]);
                pad.off(ACT_DIRECT).sb(c, small);
                pad.off(ACT_DIRECT + 1).sb(c, big);
                pad.off(LAST_SMALL).sw(c, act_small);
                let a3 = c.g[A3][0];
                pad.off(LAST_BIG).sw(c, a3);
                call_ptr(c, SET_ACT_DIRECT, 0x0032_7718)?;
                c.g[V0][0] = result;
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputVM_ReadPadChannel (0x003252f8)
// ---------------------------------------------------------------------------

/// The `switch (kind)` jump table of `InputVM_ReadPadChannel` in `.rodata`
/// and its case labels (kinds 0..=5).
pub(crate) const PAD_CHANNEL_TABLE: u32 = 0x0048_e310;

pub(crate) const PAD_CHANNEL_CASES: [u32; 6] = [
    0x0032_5320,
    0x0032_5344,
    0x0032_539c,
    0x0032_53d4,
    0x0032_53f8,
    0x0032_541c,
];

/// `InputVM.defKind`: the kind used for kind 5 ("default").
pub(crate) const PADVM_DEF_KIND: u32 = 0x0;

/// `InputVM.list`: the pad channel list (`{ int count; Entry e[]; }`).
pub(crate) const PADVM_LIST: u32 = 0x4;

/// One channel entry is 0x1c bytes; the list's entries start at +4.
pub(crate) const PAD_ENTRY_SIZE: u32 = 0x1c;

/// Entry fields (relative to `list + idx * 0x1c`): the float value (+4) and
/// the unsigned words read by kinds 1..=4.
pub(crate) const PAD_ENTRY_VALUE: u32 = 0x4;

pub(crate) const PAD_ENTRY_KIND_WORD: [u32; 5] = [0, 0x10, 0x8, 0xc, 0x14];

/// `float InputVM_ReadPadChannel(InputVM* self, int idx, int kind)`
/// (0x003252f8): the value of channel `idx` of the input VM's pad channel
/// list, as a float. Kind 0 reads the entry's float, kinds 1..=4 one of its
/// unsigned words (converted unsigned to float), kind 5 retries with the
/// VM's default kind; an index past the count gives 0 (kinds 0) or the
/// conversion of 0, an unknown kind 0.0.
pub fn input_vm_read_pad_channel(c: &mut Cpu) {
    c.cycles += 1;
    c.g[A3][0] = c.g[A0][0];
    let this = r32(c, A3);
    let idx = c.g[A1][0];

    // 0x003252fc: the switch head (kind 5 comes back here).
    loop {
        c.cycles += 3;
        let kind = c.g[A2][0];
        if kind >= 6 {
            c.g[V0][0] = 0x49_0000;
            break;
        }
        c.cycles += 6;
        let entry = PAD_CHANNEL_TABLE.wrapping_add((kind as u32) << 2);
        c.g[V1][0] = sx(entry);
        c.g[V0][0] = sx(PAD_CHANNEL_TABLE);
        c.g[A0][0] = sx(PAD_CHANNEL_CASES[kind as usize]);

        if kind == 5 {
            // 0x0032541c: kind = self->defKind, back to the head.
            c.cycles += 2;
            c.g[A2][0] = Addr(this.wrapping_add(PADVM_DEF_KIND)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0032_52fc);
            }
            continue;
        }

        // Every case: `idx < list->count` (64-bit signed compare).
        c.cycles += if kind == 1 { 3 } else { 5 };
        let list = Addr(this.wrapping_add(PADVM_LIST)).lw(c);
        c.g[A0][0] = list;
        let count = Addr(list as u32).lw(c);
        let in_range = (idx as i64) < (count as i64);
        if kind == 1 {
            c.cycles += 2;
        }
        c.g[V0][0] = PAD_ENTRY_SIZE as u64;

        if kind == 0 {
            if !in_range {
                break;
            }
            // 0x00325334: f0 = entry.value.
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            c.f[0] = Addr(at.wrapping_add(PAD_ENTRY_VALUE)).read_u32(c);
            abi::ret(c);
            return;
        }

        // Kinds 1..=4: the entry's word, or 0 past the count.
        let word = if in_range {
            c.cycles += 4;
            let off = abi::mult(c, idx as u32, PAD_ENTRY_SIZE);
            c.g[V1][0] = sx(off);
            let at = off.wrapping_add(list as u32);
            c.g[V0][0] = sx(at);
            let w = Addr(at.wrapping_add(PAD_ENTRY_KIND_WORD[kind as usize])).lw(c);
            if kind == 3 || kind == 4 {
                c.g[V0][0] = w;
                if event_due(c) {
                    deliver_event_here(c, 0x0032_53c4);
                }
            }
            w
        } else {
            // (Kinds 3 and 4 jump back to 0x003253c0.)
            if kind >= 3 && event_due(c) {
                deliver_event_here(c, 0x0032_53c0);
            }
            c.cycles += 1;
            0
        };
        c.g[V0][0] = word;

        // 0x0032536c / 0x003253c4: unsigned int to float.
        c.cycles += 2;
        let half = sx((word as u32) >> 1);
        c.g[V1][0] = half;
        if (word as i64) >= 0 {
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5374);
            }
            // 0x00325374
            c.cycles += 4;
            c.f[0] = FpuF32::from_i32(word as i32).0;
        } else {
            // (x >> 1 | x & 1) converted and doubled.
            c.cycles += if kind == 1 { 1 } else { 2 };
            c.g[V0][0] = word & 1;
            if kind != 1 && event_due(c) {
                deliver_event_here(c, 0x0032_5388);
            }
            // 0x00325388
            c.cycles += 5;
            let v = c.g[V0][0] | half;
            c.g[V0][0] = v;
            let f = FpuF32::from_i32(v as i32);
            c.f[0] = (f + f).0;
        }
        abi::ret(c);
        return;
    }

    // 0x00325424: no value.
    c.cycles += 3;
    c.f[0] = 0;
    abi::ret(c);
}

pub(crate) mod pad_queue {

    pub const TAIL: u32 = 0x2ee4;

    pub const COUNT: u32 = 0x2ee8;

    pub const DEVICES: u32 = 0x2eec;

    pub const FRAME_SIZE: u64 = 0x190;

    pub const RECORD_SIZE: u32 = 0x64;

    pub const READ_SIZE: u64 = 0x18;

    pub const FRAMES: u64 = 30;

    /// Device vtable: `read(data, size)` at `+0x10`.
    pub const VT_READ: u32 = 0x10;
}

/// `void PadQueue_Poll(PadQueue* q)` (0x326b88): read every device into
/// the queue's current frame (`record.result = dev->read(record.data,
/// 0x18)`), then advance the write index modulo 30.
pub fn pad_queue_poll(c: &mut Cpu) {
    use pad_queue::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let q = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S4][0] = FRAME_SIZE;
            c.g[S2][0] = 0;
            c.g[S1][0] = sx(q.wrapping_add(DEVICES));
            loop {
                // Device s3 (s1 its pointer, s2 its record offset).
                c.cycles += 23;
                c.g[A1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.g[A2][0] = READ_SIZE;
                c.g[A3][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.mult(0, V0, A1, S4);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                let dev = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(dev).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt.wrapping_add(VT_READ)).lh(c);
                let record = (c.g[V0][0] as u32)
                    .wrapping_add(q)
                    .wrapping_add(c.g[S2][0] as u32);
                c.g[A1][0] = sx(record);
                c.g[V0][0] = Addr(vt.wrapping_add(VT_READ + 4)).lw(c);
                c.g[A0][0] = sx(dev.wrapping_add(c.g[A0][0] as u32));
                let read = c.g[V0][0] as u32;
                c.g[A1][0] = sx(record.wrapping_add(4));
                call_ptr(c, read, 0x0032_6bf8)?;
                // The record of the (re-read) current frame gets the result.
                c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.mult(0, A0, V1, S4);
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = sx(q.wrapping_add(c.g[V1][0] as u32));
                let result = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, result);
                c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(RECORD_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
        c.g[V0][0] = FRAMES;
        c.cycles += 12;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.divu(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        let tail = c.g[A0][0];
        Addr(q.wrapping_add(TAIL)).sw(c, tail);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x50);
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
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_2;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn input_parser_pad_channel_operand_matches() {
        check_lohi(
            ssx3_ee::s10::f_00321ad8,
            input_parser_pad_channel_operand,
            200,
            &|c, seed| {
                let mut r = Rng(0x1007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn input_vm_init_matches() {
        check(ssx3_ee::s10::f_00325250, input_vm_init, 200, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn input_vm_get_reg_ptr_matches() {
        check(
            ssx3_ee::s10::f_003252e8,
            input_vm_get_reg_ptr,
            200,
            &|c, seed| {
                let mut r = Rng(0x1009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }

    #[test]
    fn input_vm_read_builtin_matches() {
        check(
            ssx3_ee::s10::f_00325430,
            input_vm_read_builtin,
            200,
            &|c, seed| {
                let mut r = Rng(0x100a ^ seed as u64);
                scramble(c, &mut r);
                let index = if seed % 3 == 0 { 0 } else { (seed % 16) as u32 };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                let base = DATA + index * 4;
                c.bus.write32(base + 0x108, r.edge_f32());
            },
        );
    }

    #[test]
    fn input_preproc_next_char_matches() {
        check(
            ssx3_ee::s10::f_00326478,
            input_preproc_next_char,
            200,
            &|c, seed| {
                let mut r = Rng(0x100b ^ seed as u64);
                scramble(c, &mut r);
                let out = DATA;
                let st = DATA + 0x100;
                let start = DATA + 0x400;
                let len = (r.next_u32() % 16) as u32;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = st as u64;
                c.bus.write32(st, start);
                c.bus.write32(st + 4, len);
                let ptr = if seed % 3 == 0 {
                    start.wrapping_add(len) // cursor at end -> -1 branch
                } else {
                    start // cursor inside -> byte branch
                };
                c.bus.write32(st + 8, ptr);
                c.bus.write8(ptr, (r.next_u32() & 0xff) as u8);
            },
        );
    }

    #[test]
    fn pad_queue_flush_matches() {
        check_lohi(
            ssx3_ee::s10::f_00326c60,
            pad_queue_flush,
            200,
            &|c, seed| {
                let mut r = Rng(0x100c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x2ee4, r.next_u32());
                c.bus.write32(DATA + 0x2ee0, r.next_u32());
            },
        );
    }

    #[test]
    fn ai_fwd_diff_cache_ctor_matches() {
        check(
            ssx3_ee::s10::f_00327810,
            ai_fwd_diff_cache_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x100d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ai_fwd_diff_cache_dtor_matches() {
        check(
            ee::s10::f_00327828,
            ai_fwd_diff_cache_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7828, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                // Cover null and non-null for both owned blocks.
                c.bus
                    .write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
                c.bus
                    .write32(DATA + 8, if seed % 5 == 0 { 0 } else { DATA + 0x200 });
            },
        );
    }

    #[test]
    fn input_vm_read_operand_matches() {
        check(
            ssx3_ee::s10::f_00325260__generated,
            input_vm_read_operand,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4002, 0);
                c.g[A0][0] = sx(DATA);
                let id = match seed % 4 {
                    0 => r.next_u32() % 0x40,          // a script variable
                    1 => 0x40 + r.next_u32() % 0x20,   // a builtin
                    2 => 0x60 + r.next_u32() % 0x39c,  // a controller channel
                    _ => 0x3fc + r.next_u32() % 0x100, // past the end
                };
                c.g[A1][0] = sx(id);
                c.bus.write32(DATA + 4, DATA + 0x400); // pad state
                c.bus.write32(DATA + 0x400, 8); // channel count
                for k in 0..0x90 {
                    c.bus.write32(DATA + 8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn input_parser_builtin_operand_matches() {
        check(
            ee::s10::f_00321ad0__generated,
            input_parser_builtin_operand,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x0032_1ad0, seed);
                scramble(c, &mut r);
                let v = r.next_u32();
                c.g[A1][0] = mixed(&mut r, seed, v);
            },
        );
    }

    #[test]
    fn pad_device_dtor_matches() {
        dtor_case(ee::s10::f_003277d0__generated, pad_device_dtor, 0x3277d0);
    }

    #[test]
    fn pad_queue_init_matches() {
        check(
            ssx3_ee::s10::f_00326a28__generated,
            pad_queue_init,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x326a_28, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn input_preproc_emit_line_marker_matches() {
        check(
            ee::s10::f_00326430__generated,
            input_preproc_emit_line_marker,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_6430, 0);
                // The line-marker format lives in the game's data image:
                // `# %d "%s"\n` as UTF-16.
                let fmt = [
                    0x23u16, 0x20, 0x25, 0x64, 0x20, 0x22, 0x25, 0x73, 0x22, 0x0a, 0,
                ];
                for (i, w) in fmt.iter().enumerate() {
                    c.bus.write16(0x0048_e3c8 + 2 * i as u32, *w);
                }
                let text = DATA + 0x2000;
                for (i, b) in b"hello world\0".iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                let p = DATA + 0x1000;
                c.bus.write32(p, 0);
                c.bus.write32(p + 4, 0x1000);
                c.bus.write32(p + 8, DATA + 0x4000);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = (r.next_u32() % 1000) as u64;
                c.g[A2][0] = sx(text);
            },
        );
    }

    #[test]
    fn input_preproc_write_string_matches() {
        check(
            ssx3_ee::s10::f_00326308__generated,
            input_preproc_write_string,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6308, 0);
                heap_init(c);
                let inp = DATA + 0x400;
                let s = DATA + 0x500;
                let n = (seed % 8) as u32;
                for i in 0..n {
                    c.bus.write8(s + i, b'A' + (i as u8 % 26));
                }
                c.bus.write8(s + n, 0);
                if seed % 3 == 0 {
                    // Empty buffer: the write grows it (operator_new_named).
                    c.bus.write32(inp, 0);
                    c.bus.write32(inp + 4, 0);
                    c.bus.write32(inp + 8, 0);
                } else {
                    let count = r.next_u32() % 4;
                    c.bus.write32(inp, count);
                    c.bus.write32(inp + 4, count + 1 + r.next_u32() % 4);
                    c.bus.write32(inp + 8, DATA + 0x600);
                }
                c.g[A0][0] = inp as u64;
                c.g[A1][0] = s as u64;
                // The loop is entered at its own address (it re-enters at
                // 0x326328 after a thread switch).
                c.pc = 0x0032_6308;
            },
        );
    }

    #[test]
    fn ps2pad_set_actuator_matches() {
        check(
            ssx3_ee::s10::f_00327740__generated,
            ps2pad_set_actuator,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x327740, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { (seed % 5) as u64 };
                c.f[12] = pick(
                    &mut r,
                    &[
                        0.0, -0.0, 0.244, 0.5, 0.899, 0.9, 1.0, -1.0, 1.0e9, -1.0e9, 1.0e-30,
                    ],
                );
                for i in 0..0x40 {
                    word(c, DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_pad_ctor_matches() {
        check(
            ssx3_ee::s10::f_00326df0__generated,
            ps2_pad_ctor,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x326df0, 0);
                fill(c, &mut r, OBJ, 0x20);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn ps2_pad_dtor_matches() {
        check(
            ssx3_ee::s10::f_00326e50__generated,
            ps2_pad_dtor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x326e50, 0);
                w_2(c, OBJ + 0x4, seed.wrapping_mul(7));
                w_2(c, OBJ + 0x8, seed.wrapping_mul(11));
                w_2(c, OBJ, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    /// The parser object (`+8`: 64 virtual registers).
    pub(crate) const PARSER: u32 = DATA;

    /// The opcode stream.
    pub(crate) const STREAM: u32 = DATA + 0x1000;

    /// The object the original receives as its second argument (stored at
    /// `parser + 4`; the pad-channel operands read through it).
    pub(crate) const PAD: u32 = DATA + 0x2000;

    /// Eight instructions whose opcodes are chosen by the seed (every opcode
    /// lands in every position over 33 consecutive seeds), followed by
    /// opcode 0x20 padding. Skips are one word, so a run stays inside.
    pub(crate) fn put_code(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let mut at = STREAM;
        // A reserved opcode (0x21..0x3f): not dispatched, the run continues.
        c.bus.write32(at, 0x21 + seed % 31);
        at += 4;
        for i in 0..8u32 {
            let op = seed.wrapping_mul(5).wrapping_add(i * 11) % 33;
            let dst = seed.wrapping_add(i * 7) & 0x3f;
            let a = seed.wrapping_add(i * 13) & 0x3f;
            let b = seed.wrapping_add(i * 29) & 0x3f;
            let mut ins = op | (dst << 6) | (a << 12);
            if op == 2 || op == 3 {
                ins |= 1 << 16; // skip one word
            } else {
                ins |= b << 22;
            }
            c.bus.write32(at, ins);
            at += 4;
            if op == 0 {
                // The immediate float. Its low bits are forced to opcode 1
                // with in-range operands, so a skip that lands on it is inert.
                let imm = (r.next_u32() & 0xffc0_0000)
                    | 1
                    | ((r.next_u32() & 0x3f) << 6)
                    | ((r.next_u32() & 0x3f) << 12);
                c.bus.write32(at, imm);
                at += 4;
            }
        }
        at
    }

    #[test]
    fn input_vm_execute_matches() {
        check(
            ee::s10::f_00325450__generated,
            input_vm_execute,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3254_5000 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                // The game's data: the `gp` tables the float opcodes and the maths
                // callees read, and the opcode dispatch table.
                elf_image(c);
                c.pc = 0x0032_5450;
                c.g[GP][0] = sx(GAME_GP);
                for i in 0..0x40u32 {
                    let v = if i % 2 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1000.0)
                    };
                    c.bus.write32(PARSER + PARSER_REGS + 4 * i, v);
                }
                c.g[A0][0] = sx(PARSER);
                c.g[A1][0] = sx(PAD);
                c.g[A2][0] = sx(STREAM);
                let mut at = put_code(c, &mut r, seed);
                // Opcode 0x20 (stop): where every run ends.
                for _ in 0..64 {
                    c.bus.write32(at, OP_STOP);
                    at += 4;
                }
            },
        );
    }

    /// An empty guest function (returns immediately, 2 cycles).
    pub(crate) const EMPTY: u32 = 0x0038_dc60;

    #[test]
    fn input_preproc_skip_line_matches() {
        check(
            ssx3_ee::s10::f_00326678__generated,
            input_preproc_skip_line,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x326678, 0);
                let p_4 = DATA + 0x1000;
                let s = DATA + 0x2000;
                let buf = DATA + 0x3000;
                c.g[A0][0] = p_4 as u64;
                c.g[A1][0] = s as u64;
                // A 8-byte stream with a newline at position seed % 8.
                w32(c, s, 8);
                w32(c, s + 4, 0);
                w32(c, s + 8, buf);
                w32(c, s + 0xc, 0);
                let nl = (seed % 8) as u32;
                w8(c, p_4 + 0xc, b'x');
                for i in 0..8u32 {
                    w8(c, buf + i, if i == nl { 0x0a } else { b'a' });
                }
            },
        );
    }

    #[test]
    fn pad_device_stop_actuators_matches() {
        check(
            ssx3_ee::s10::f_00326cf0__generated,
            pad_device_stop_actuators,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x326cf0, 0);
                let d = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = d as u64;
                c.pc = 0x0032_6cf0;
                w32(c, d, vt);
                slot(c, vt, 0x18, 0, LOAD_B0);
                slot(c, vt, 0x20, 0, EMPTY);
                w32(c, d + 0xb0, (seed % 4) as u32);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn pad_queue_shutdown_matches() {
        check(
            ee::s10::f_00326a68__generated,
            pad_queue_shutdown,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x326a68, 6);
                be_library(c);
                world(c);
                let q = DATA;
                c.g[A0][0] = q as u64;
                let n = seed % 4;
                w(c, q + 0x2ee8, n);
                for i in 0..n {
                    let e = q + 0x2f00 + i * 0x40;
                    let ptr = if (seed + i) % 3 == 0 { 0 } else { e };
                    w(c, q + 0x2eec + i * 4, ptr);
                    fake_slot(c, e, DATA + 0x3000, 8);
                }
            },
        );
    }

    #[test]
    fn input_parser_find_identifier_matches() {
        check(
            ee::s10::f_00321a40__generated,
            input_parser_find_identifier,
            150,
            &|c, seed| {
                let _ = base(c, seed, 0x32_1a40);
                globals(c);
                const TABLE: u32 = DATA + 0x80e8;
                w(c, DATA + 0x8000 + 0xe4, if seed % 3 == 0 { 0 } else { 3 });
                for i in 0..3u32 {
                    let e = TABLE + 0x44 * i;
                    for (j, b) in [
                        b"alpha\0".as_slice(),
                        b"beta\0".as_slice(),
                        b"gamma\0".as_slice(),
                    ][i as usize]
                        .iter()
                        .enumerate()
                    {
                        c.bus.write8(e + j as u32, *b);
                    }
                }
                // The name looked up.
                let name = b"beta\0";
                for (j, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x200 + j as u32, *b);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                if seed % 3 == 1 {
                    c.bus.write8(DATA + 0x200, b'B');
                }
            },
        );
    }

    #[test]
    fn ps2_pad_create_matches() {
        check(
            ee::s10::f_00326d60__generated,
            ps2_pad_create,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x326d, 0);
                env(c);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // ------------------------------------------------------- input and bolts

    pub(crate) fn wstr(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    #[test]
    fn input_parser_lookup_modifier_matches() {
        const NAME: u32 = DATA + 0x1000;
        check(
            ee::s10::f_003217d0__generated,
            input_parser_lookup_modifier,
            12,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0032_17d0, 0);
                let globals = [
                    0x004a_3ef8u32,
                    0x004a_3f00,
                    0x004a_3f08,
                    0x0048_ddd8,
                    0x004a_3f10,
                ];
                for (i, g) in globals.iter().enumerate() {
                    let name = [b'a' + i as u8];
                    wstr(c, *g, &name);
                }
                let pick = seed % 6;
                if pick < 5 {
                    wstr(c, NAME, &[b'a' + pick as u8]);
                } else {
                    wstr(c, NAME, b"zz");
                }
                c.g[A1][0] = NAME as u64;
            },
        );
    }

    // ------------------------------------------------------ input preproc

    #[test]
    fn input_preproc_skip_line_comment_matches() {
        check(
            ee::s10::f_003265c0__generated,
            input_preproc_skip_line_comment,
            SEEDS_9,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x3265, 0);
                let pre = DATA;
                let cur = DATA + 0x100;
                if seed % 3 == 0 {
                    // End of input: the reader reports -1.
                    w(c, pre + 0xc, 0);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 0);
                    w(c, cur + 8, DATA + 0x400);
                } else {
                    // Newline: the comment is written and the cursor moves.
                    w(c, pre + 0xc, 0xa);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 4);
                    w(c, cur + 8, DATA + 0x400);
                    wb(c, DATA + 0x400, 0xa);
                }
                w(c, pre, r.next_u32());
                c.g[A0][0] = pre as u64;
                c.g[A1][0] = cur as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn input_parser_compile_identifier_matches() {
        check(
            ee::s10::f_003245a8__generated,
            input_parser_compile_identifier,
            40,
            &|c, seed| {
                let _r = base_4(c, seed, 0x3245);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (seed % 8) as u64;
                c.g[T0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn input_preproc_write_matches() {
        check(
            ee::s10::f_00326360__generated,
            input_preproc_write,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x3263);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                let cap = if seed % 4 == 0 { 8 } else { 0x100 };
                let pos = if seed % 4 == 0 {
                    cap
                } else {
                    (seed % 8) as u32
                };
                c.bus.write32(DATA, pos);
                c.bus.write32(DATA + 4, cap);
                c.bus
                    .write32(DATA + 8, if seed % 8 == 0 { 0 } else { DATA + 0x2000 });
                for i in 0..0x20 {
                    c.bus.write8(DATA + 0x2000 + i, 0);
                }
            },
        );
    }
    use super::*;

    // --------------------------------------------------------- PS2Pad_Update

    /// `PS2Pad_Update` against the generated body, with the real libpad
    /// callees on their short paths: the pad's libpad port entry points at
    /// a test DMA frame (state, request state, mode bytes and pad data
    /// chosen per seed) or is closed (every call returns 0); the SIF RPC
    /// packet pool is empty so `scePadPortOpen`, `scePadSetMainMode` and
    /// `scePadSetActAlign` fail at once. Every phase 0..=10 plus junk
    /// phases, every state/req-state/id/ext answer, retry counts at the
    /// limit, pressure and digital buttons, analog sticks on both sides of
    /// the dead zone (and off), changed / unchanged motor values.
    #[test]
    fn ps2_pad_update_matches() {
        use super::ps2pad::*;
        use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

        const OUT: u32 = DATA + 0x1000;
        /// The libpad frame pair (2 x 0x80) of the pad's port.
        const DMA: u32 = DATA + 0x2000;
        /// libpad's port table: entry `port * 0x70 + slot * 0x1c` holds the
        /// DMA buffer (+0) and the opened flag (+0x10).
        const LIBPAD_PORTS: u32 = 0x0052_6250;
        const SIF_RPC_PACKETS: u32 = 0x0052_d680;
        const LIBPAD_PRINTF: u32 = 0x0045_15cc;
        // DMA frame fields
        const F_FRAME: u32 = 0x58;
        const F_LENGTH: u32 = 0x60;
        const F_MODE_TABLE: u32 = 0x50;
        const F_MODE_CONFIG: u32 = 0x64;
        const F_MODE_CUR_ID: u32 = 0x65;
        const F_MODE_CUR_OFFS: u32 = 0x69;
        const F_STATE: u32 = 0x70;
        const F_REQ_STATE: u32 = 0x71;
        const F_OK: u32 = 0x72;

        check(
            ee::s10::f_00326eb0__generated,
            ps2_pad_update,
            400,
            &|c, seed| {
                let mut r = Rng(0x0326_eb0d ^ (seed as u64) << 4);
                scramble(c, &mut r);
                let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];
                c.g[GP][0] = GAME_GP as u64;
                put_words(c, JUMP_TABLE, &CASES);
                let scale = if seed % 5 == 0 {
                    r.f32(2.0)
                } else {
                    0x3b80_8081
                };
                put_words(c, GAME_GP - PRESSURE_SCALE_GP, &[scale]);
                put_words(c, GAME_GP - STICK_SCALE_GP, &[scale ^ (seed & 1)]);
                put_words(c, LIBPAD_PRINTF, &[0]);
                put_words(c, SIF_RPC_PACKETS, &[0, 0, 0, 0]);

                // the pad (DMA buffer at +0x80 misaligned for some seeds)
                let pad = DATA + if seed % 7 == 3 { 0x10 } else { 0 };
                let port = r.next_u32() % 2;
                let slot = r.next_u32() % 2;
                fill(c, &mut r, pad, 0x30);
                c.bus.write32(pad + PORT, port);
                c.bus.write32(pad + SLOT, slot);
                let phase = match seed % 13 {
                    11 => r.next_u32(),
                    12 => 10,
                    p => p,
                };
                c.bus.write32(pad + PHASE, phase);
                let retry = pick(&mut r, &[0, 3, 4, 4, 7]);
                c.bus.write32(pad + RETRY, retry);
                for f in [MODE_SET, PRESSURE, ANALOG, ACTUATOR] {
                    let v = pick(&mut r, &[0, 1, 1]);
                    c.bus.write32(pad + f, v);
                }
                let mut acts = [0u32; 4];
                for a in acts.iter_mut() {
                    *a = pick(&mut r, &[0, 1, 0xff, 0xffff_ffff]);
                }

                // libpad port entry and its DMA frame (frame 0 is the newer)
                let entry = LIBPAD_PORTS + port * 0x70 + slot * 0x1c;
                fill(c, &mut r, entry, 7);
                c.bus.write32(entry, DMA);
                c.bus.write32(entry + 0xc, 0);
                c.bus.write32(entry + 0x10, (seed % 6 != 0) as u32);
                fill(c, &mut r, DMA, 0x40);
                c.bus.write32(DMA + F_FRAME, 1);
                c.bus.write32(DMA + 0x80 + F_FRAME, 0);
                let len = pick(&mut r, &[0, 0x12, 0x20, 0x20]);
                c.bus.write32(DMA + F_LENGTH, len);
                if r.next_u32() % 3 != 0 {
                    c.bus.write8(DMA, 0);
                }
                for i in 4..8 {
                    let any = r.next_u32() & 0xff;
                    let b = pick(
                        &mut r,
                        &[0, 0x4e, 0x4f, 0x50, 0x80, 0xaf, 0xb0, 0xb1, 0xff, any],
                    );
                    c.bus.write8(DMA + i, b as u8);
                }
                let state = pick(&mut r, &[0, 1, 2, 5, 6, 6, 6, 7, 2]);
                c.bus.write8(DMA + F_STATE, state as u8);
                let req = pick(&mut r, &[0, 1, 2, 3]);
                c.bus.write8(DMA + F_REQ_STATE, req as u8);
                let ok = pick(&mut r, &[0, 1, 1, 1, 2]);
                c.bus.write8(DMA + F_OK, ok as u8);
                let any = r.next_u32() & 0xff;
                let config = pick(&mut r, &[0, 1, 2, any]);
                c.bus.write8(DMA + F_MODE_CONFIG, config as u8);
                let any = r.next_u32() & 0xff;
                let id = pick(&mut r, &[0x41, 0x73, 0x79, 0x23, 0xf3, any]);
                c.bus.write8(DMA + F_MODE_CUR_ID, id as u8);
                let offs = r.next_u32() % 4;
                c.bus.write8(DMA + F_MODE_CUR_OFFS, offs as u8);
                for i in 0..4 {
                    let ext = pick(&mut r, &[0, 4, 7, 5, 1]);
                    c.bus.write16(DMA + F_MODE_TABLE + 2 * i, ext as u16);
                }
                // keep scePadSetActDirect off its SIF DMA path (needs the
                // frame's ok byte 1): no motor change then
                if ok == 1 {
                    acts[1] = acts[0];
                    acts[3] = acts[2];
                }
                for (i, a) in acts.iter().enumerate() {
                    c.bus.write32(pad + ACT_SMALL + 4 * i as u32, *a);
                }

                fill(c, &mut r, OUT, 0x18);
                c.g[A0][0] = sx(pad);
                c.g[A1][0] = sx(OUT);
                c.pc = 0x0032_6eb0;
            },
        );
    }
    use super::*;
    use crate::difftest::put_words;

    /// `InputVM_ReadPadChannel`: every kind (0..=5 and out of range), the
    /// index inside and past the count, words with and without bit 31 (the
    /// unsigned conversion's two paths); kind 5 retries with the default
    /// kind (never 5 again).
    #[test]
    fn input_vm_read_pad_channel_matches() {
        check_lohi(
            ee::s10::f_003252f8__generated,
            input_vm_read_pad_channel,
            300,
            &|c, seed| {
                let mut r = Rng(0x3252_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_52f8;
                put_words(c, PAD_CHANNEL_TABLE, &PAD_CHANNEL_CASES);
                let this = DATA;
                let list = DATA + 0x100;
                let count = 1 + r.next_u32() % 6;
                let def = [0u32, 1, 2, 3, 4, 6, 9][(seed / 7 % 7) as usize];
                c.bus.write32(this + PADVM_DEF_KIND, def);
                c.bus.write32(this + PADVM_LIST, list);
                c.bus.write32(list, count);
                for i in 0..(count * PAD_ENTRY_SIZE / 4 + 8) {
                    let w = match r.next_u32() % 3 {
                        0 => r.next_u32() | 0x8000_0000,
                        1 => r.next_u32() & 0x7fff_ffff,
                        _ => r.next_u32() % 300,
                    };
                    c.bus.write32(list + 4 + i * 4, w);
                }
                c.g[A0][0] = sx(this);
                let idx = match seed % 4 {
                    0 => count,
                    // A negative index passes the signed compare.
                    1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
                    _ => r.next_u32() % count,
                };
                c.g[A1][0] = sx(idx);
                c.g[A2][0] = match seed % 7 {
                    6 => [6u64, 0xffff_ffff_ffff_ffff, 1 << 40][(seed / 7 % 3) as usize],
                    k => k as u64,
                };
            },
        );
    }
    use super::*;

    /// PadQueue_Poll: 0-4 devices (and a negative count), write index 0-29
    /// (29 wraps); the devices' read methods are fakes (some unwind).
    #[test]
    fn pad_queue_poll_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        check(
            ee::s10::f_00326b88__generated,
            pad_queue_poll,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_6b88, 4);
                c.pc = 0x0032_6b88;
                let q = DATA;
                let n = if seed % 9 == 0 { u32::MAX } else { seed % 5 };
                c.bus.write32(q + 0x2ee8, n);
                c.bus.write32(
                    q + 0x2ee4,
                    if seed % 3 == 0 { 29 } else { r.next_u32() % 30 },
                );
                for i in 0..4u32 {
                    let dev = DATA + 0x4000 + 0x40 * i;
                    let vt = DATA + 0x5000 + 0x40 * i;
                    c.bus.write32(q + 0x2eec + 4 * i, dev);
                    c.bus.write32(dev, vt);
                    c.bus
                        .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + 0x14, FAKE + 0x10 * i);
                }
                c.g[A0][0] = sx(q);
            },
        );
    }
}
