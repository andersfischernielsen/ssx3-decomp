//! `src/bx/bxstring.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::gp as gp_3;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    (0x0031_7fe8, c_bx_string_ctor_copy, "cBXString_ctor_copy"),
    (
        0x0031_8040,
        cbx_string_alloc_buffer,
        "cBXString_AllocBuffer",
    ),
    (0x0031_80b8, c_bx_string_release, "cBXString_Release"),
    (
        0x0031_8120,
        c_bx_string_copy_before_write,
        "cBXString_CopyBeforeWrite",
    ),
    (
        0x0031_8188,
        c_bx_string_alloc_before_write,
        "cBXString_AllocBeforeWrite",
    ),
    (0x0031_81e8, cbx_string_alloc_copy, "cBXString_AllocCopy"),
    (0x0031_8260, cbx_string_dtor, "cBXString_dtor"),
    (0x0031_82d8, cbx_string_ctor_cstr, "cBXString_ctor_cstr"),
    (
        0x0031_8350,
        c_bx_string_ctor_cstr_n,
        "cBXString_ctor_cstr_n",
    ),
    (0x0031_83d8, cbx_string_assign_copy, "cBXString_AssignCopy"),
    (0x0031_8448, cbxstring_assign, "cBXString_assign"),
    (0x0031_8540, c_bx_string_concat_copy, "cBXString_ConcatCopy"),
    (
        0x0031_85c8,
        c_bx_string_op_plus_str_str,
        "cBXString_OpPlusStrStr",
    ),
    (
        0x0031_8630,
        cbxstring_op_plus_str_cstr,
        "cBXString_OpPlusStrCStr",
    ),
    (
        0x0031_86d0,
        c_bx_string_op_plus_c_str_str,
        "cBXString_OpPlusCStrStr",
    ),
    (0x0031_8910, c_bx_string_append_cstr, "cBXString_AppendCStr"),
    (0x0031_89a0, cbxstring_append_str, "cBXString_AppendStr"),
    (0x0031_89d0, cbx_string_get_buffer, "cBXString_GetBuffer"),
    (
        0x0031_8a88,
        cbx_string_release_buffer,
        "cBXString_ReleaseBuffer",
    ),
    (0x0031_8d28, cbx_string_make_upper, "cBXString_MakeUpper"),
    (0x0031_8e68, c_bx_string_set_at, "cBXString_SetAt"),
    (0x0031_9028, c_bx_string_right, "cBXString_Right"),
    (0x0031_90a8, cbx_string_left, "cBXString_Left"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET as UI_LIST_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
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
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
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
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT as BUDDY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
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
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
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
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
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
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY as CBX_STRING_CTOR_COPY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR as CBX_STRING_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE as WORLD_OBJ_REPLACE_WITH_DEAD_NODE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString_AppendStr(cBXString* self, char** str)` (0x3189a0): append the
/// C string `*str` (its length comes from the cBXString header at `*str-8`),
/// returning `self`.
pub fn cbxstring_append_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_sub(8)).lw(c);
        call_ptr(c, 0x0031_8830, 0x0031_89bc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bx strings and spheres.

/// `void cBXString_SetAt(cBXString* self, int idx, char ch)` (0x318e68): make
/// the buffer writable, then store `ch` at `idx`.
pub fn c_bx_string_set_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A2) << 24);
        frame.save(c, S2, 0x10);
        let str = r32(c, A0);
        let idx = r32(c, A1);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_8e90)?;
        c.g[V0][0] = Addr(str).lw(c);
        frame.restore(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(idx));
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `cBXString* cBXString_ctor_copy(cBXString* self, const cBXString* other)`
/// (0x317fe8): share `other`'s buffer when it is already shared, else copy.
pub fn c_bx_string_ctor_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
        let shared = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = c.g[A0][0];
        if shared {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(r32(c, A1)).lw(c);
            c.g[RA][0] = sx(0x0031_802c);
            callee::BXSTRING_ASSIGN_CSTR.call(c, 0x0031_802c)?;
        } else {
            c.cycles += 5;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            Addr(r32(c, V1)).off(0xffff_fff4).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_AppendCStr(cBXString* self, const char* str)`
/// (0x318910): append a C string in place.
pub fn c_bx_string_append_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_8934);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_8934)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8950);
        callee::BXSTRING_CONCAT_IN_PLACE.call(c, 0x0031_8950)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_AllocBeforeWrite(cBXString* s, int needed)` (0x318188):
/// reallocate the string's buffer when the reference is not unique or the
/// buffer is too small for `needed` bytes.
pub fn c_bx_string_alloc_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let s = c.g[A0][0];
        let needed = c.g[A1][0];
        c.g[S0][0] = s;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32_2(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32_2(c, V1)).off(0xffff_fff4).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        c.g[S1][0] = needed;
        let rebuild;
        if c.g[V0][0] == 0 {
            // `refcount >= 2`: the buffer is shared, rebuild without a
            // capacity test (the generated `t = V0 == 0` arm).
            rebuild = true;
        } else {
            // Unique reference: rebuild only when the buffer is too small.
            c.cycles += 4;
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0xffff_fffc).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            rebuild = c.g[V0][0] != 0;
        }
        if rebuild {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_80b8, 0x0031_81c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0031_8040, 0x0031_81d4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn ld(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).ld(c)
}

// ---------------------------------------------------------------------------
// cBXString refcounting.

/// `void cBXString_Release(cBXString* self)` (0x3180b8): drop the buffer's
/// reference count and free it at zero; a released string becomes the static
/// empty string (`D_004A3E90`).
pub fn c_bx_string_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x0048_dcd4;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let s = lw(c, this as u32);
        c.g[V1][0] = s;
        if s != c.g[V0][0] {
            c.cycles += 4;
            c.g[V0][0] = lw(c, (s as u32).wrapping_sub(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let live = c.g[V0][0] != 0;
            sw(c, (s as u32).wrapping_sub(0xc), c.g[V0][0]);
            let mut skip_reload = false;
            if !live {
                c.cycles += 4;
                c.g[V0][0] = lw(c, this as u32);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                let empty = c.g[A0][0] == 0;
                c.g[V0][0] = lw(c, gp_2(c, 0xda0));
                if empty {
                    skip_reload = true;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0031_7e98, 0x0031_8104)?;
                }
            }
            if !skip_reload {
                c.cycles += 1;
                c.g[V0][0] = lw(c, gp_2(c, 0xda0));
            }
            c.cycles += 2;
            sw(c, this as u32, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_CopyBeforeWrite(cBXString* self)` (0x318120): when the
/// buffer is shared, release it, allocate a private copy and memcpy the bytes
/// (length + 1) over.
pub fn c_bx_string_copy_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        let s = lw(c, this as u32);
        c.g[S0][0] = s;
        // The original leaves the `refcount < 2` test in v0; the copy path
        // overwrites it with the callees' results.
        let private = (lw(c, (s as u32).wrapping_sub(0xc)) as i64) < 2;
        c.g[V0][0] = private as u64;
        if !private {
            c.cycles += 11;
            call_ptr(c, 0x0031_80b8, 0x0031_8150)?;
            c.g[A1][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_8040, 0x0031_815c)?;
            c.g[A2][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A1][0] = s;
            c.g[A0][0] = lw(c, this as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0041_605c, 0x0031_8170)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_OpPlusStrStr(cBXString* self, cBXString* a,
/// cBXString* b)` (0x3185c8): concatenate the two strings' buffers into a
/// temporary, copy-construct `self` from it and destroy the temporary.
pub fn c_bx_string_op_plus_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[V1][0] = lw(c, gp_2(c, 0xda0));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        frame_save_16(c);
        c.g[S0][0] = c.g[A0][0];
        sw(c, c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        c.g[V0][0] = lw(c, c.g[A2][0] as u32);
        c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(8));
        c.g[A2][0] = c.g[V1][0];
        c.g[A3][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(8));
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0031_8540, 0x0031_8600)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_860c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_8618)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = ld(c, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `sd ra, 0x10(sp); sq s0, 0x20(sp)` of [`c_bx_string_op_plus_str_str`].
#[inline(always)]
pub(crate) fn frame_save_16(c: &mut Cpu) {
    let a = c.g[SP][0] as u32;
    c.sq(16, a.wrapping_add(0x20));
    let ra = c.g[RA][0];
    c.sd(a.wrapping_add(0x10), ra);
}

// ---------------------------------------------------------------------------
// String, world-cache, replay and camera helpers.

/// `void cBXString_MakeUpper(cBXString* self)` (0x318d28): copy the buffer if
/// shared, then uppercase every character (`tolower`-table bit 1).
pub fn cbx_string_make_upper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0031_8d3c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_8120, 0x0031_8d3c)?;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        let empty = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = 0x4a0000;
        if !empty {
            c.cycles += 2;
            // 0x499c81: the character-class table (bit 1 = lowercase).
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x637f));
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.lb((c.g[S0][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.sb((c.g[S0][0] as u32), c.g[V1][0]);
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_8d58);
                }
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void cBXString_AssignCopy(cBXString* self, int len, const char* str)`
/// (0x3183d8): make room, copy `len` bytes and terminate, unless the string
/// is the shared empty buffer.
pub fn cbx_string_assign_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0031_8188, 0x0031_83fc)?;
        c.g[V0][0] = 0x48dc_d4;
        c.g[A0][0] = Addr(at_2(c, S0, 0)).lw(c);
        let shared = c.g[A0][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        if !shared {
            c.cycles += 7;
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8418)?;
            c.g[V1][0] = Addr(at_2(c, S0, 0)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S1][0]);
            c.g[V0][0] = Addr(at_2(c, S0, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
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

/// `void cBXString_ReleaseBuffer(cBXString* self, int len)` (0x318a88):
/// detach the buffer, shrink it to `len` (`-1` uses the current length) and
/// terminate.
pub fn cbx_string_release_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_8120, 0x0031_8aa4)?;
        c.g[V0][0] = u64::MAX;
        let given = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x4900_00;
        if !given {
            c.cycles += 4;
            c.g[A0][0] = Addr(at_2(c, S1, 0)).lw(c);
            call_ptr(c, 0x0041_6810, 0x0031_8ab8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x4900_00;
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(at_2(c, S1, 0)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x232c));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 5;
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString_Realloc` (0x318040): `cBXString_AllocBuffer`. A zero size just
/// points the string at the shared empty buffer (`D_004A3E90`); otherwise
/// allocate `size + 0xd` bytes with `operator new(size, D_0048D850,
/// 0x20000000, 0)`, write the three-word header (refcount 1, size, capacity)
/// in front of the characters and terminate the string.
pub fn cbx_string_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, GP, 0xda0);
            sw_r(c, S1, 0, c.g[V0][0]);
        } else {
            c.cycles += 14;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0xd));
            c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x27b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[RA][0] = sx(0x0031_8080);
            c.g[A3][0] = 0;
            callee::OPERATOR_NEW_NAMED.call(c, 0x0031_8080)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx(low(c, V0).wrapping_add(0xc));
            sw_r(c, V0, 0, c.g[V1][0]);
            c.g[A1][0] = sx(low(c, A0).wrapping_add(low(c, S0)));
            sw_r(c, S1, 0, c.g[A0][0]);
            sb_r(c, A1, 0, 0);
            sw_r(c, V0, 0x8, c.g[S0][0]);
            sw_r(c, V0, 0x4, c.g[S0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString_Left` (0x3190a8): the first `count` characters of `src` (a
/// negative count means "all"; a longer count is clamped to the source
/// length stored in front of its characters, `*(int*)(*(char**)src - 8)`).
/// The copy is built in a stack temporary and moved into `dst`.
/// Signature: `cBXString* left(cBXString* dst, cBXString* src, int count)`.
pub fn cbx_string_left(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, A1)).lw(c);
            c.g[A0][0] = Addr(low(c, V0).wrapping_sub(8)).lw(c);
            let shorter = (c.g[A0][0] as i64) < (c.g[A2][0] as i64);
            c.g[V1][0] = shorter as u64;
            if shorter {
                c.g[A2][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = 0;
        }
        c.cycles += 18;
        c.g[V0][0] = lw_r(c, GP, 0xda0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_90f0);
        c.g[T0][0] = 0;
        callee::CBX_STRING_ALLOC_COPY.call(c, 0x0031_90f0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_90fc);
        c.g[A0][0] = c.g[S0][0];
        callee::CBX_STRING_CTOR_COPY.call(c, 0x0031_90fc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_9108);
        c.g[A1][0] = 0x2;
        callee::CBX_STRING_DTOR.call(c, 0x0031_9108)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_AllocCopy(dst, src, count, start, extra)` (0x3181e8):
/// allocate `count + extra` characters and copy them from `src->chars +
/// start`; a zero total aliases the shared empty string.
pub fn cbx_string_alloc_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[S1][0] = c.g[A2][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, S1).wrapping_add(low(c, T0)));
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        let empty = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if empty {
            c.cycles += 8;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8230)?;
            c.g[A1][0] = Addr(low(c, S2)).lw(c);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = Addr(low(c, S0)).lw(c);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(low(c, S3)));
            call_ptr(c, callee::MEMCPY, 0x0031_8244)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
            sw_r(c, S0, 0, c.g[V0][0]);
        }
        c.cycles += 7;
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_dtor(self, flags)` (0x318260): drop the character buffer's
/// reference count, free it at zero and delete the object when `flags & 1`.
pub fn cbx_string_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x232c));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V1][0] = Addr(low(c, S0)).lw(c);
        let shared = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut extra = true;
        let mut result;
        if shared {
            result = low(c, S1) & 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, V1).wrapping_sub(0xc)).lw(c);
            c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
            let alive = (c.g[V0][0] as i64) > 0;
            sw_r(c, V1, 0xffff_fff4, c.g[V0][0]);
            if alive {
                result = low(c, S1) & 1;
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(low(c, S0)).lw(c);
                c.g[A0][0] = sx(low(c, V0).wrapping_sub(0xc));
                let null = c.g[A0][0] == 0;
                result = low(c, S1) & 1;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                if null {
                    extra = false;
                } else {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x0031_82b0)?;
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE_2.call(c, 0x0031_82c4)?;
            c.lq(16, sp_2(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `cBXString* cBXString_ctor_cstr(self, str)` (0x3182d8): point at the shared
/// empty string, then copy `strlen(str)` characters when `str` is non-empty.
pub fn cbx_string_ctor_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp_2(c), c.g[RA][0]);
        let null = c.g[S1][0] == 0;
        sw_r(c, S2, 0, c.g[V0][0]);
        if null {
            c.cycles += 1;
            c.g[S0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::STRLEN, 0x0031_8308)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8324)?;
            c.g[A0][0] = Addr(low(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, callee::MEMCPY, 0x0031_8334)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, sp_2(c).wrapping_add(0x30));
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_Right(dst, src, n)` (0x00319028): dst = the last `n`
/// characters of the BX string `src` (all of it when `n` is negative).
pub fn c_bx_string_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) < 0 {
            c.cycles += 2;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(at_3(c, A1, 0)).lw(c);
            c.g[A0][0] = Addr(word(c, V0).wrapping_sub(8)).lw(c);
            c.g[V1][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = c.g[A0][0];
            }
        }
        c.cycles += 20;
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        c.g[A0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(at_3(c, A1, 0)).lw(c);
        c.g[T0][0] = 0;
        Addr(at_3(c, SP, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = Addr(word(c, V1).wrapping_sub(8)).lw(c);
        c.g[A3][0] = sx(word(c, A3).wrapping_sub(word(c, A2)));
        callee::BX_STRING_ALLOC_COPY.call(c, 0x0031_9078)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BX_STRING_CTOR_COPY.call(c, 0x0031_9084)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        callee::BX_STRING_DTOR.call(c, 0x0031_9090)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.lq(16, frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BX strings.

/// `cBXString* cBXString_ctor_cstr_n(cBXString* self, const char* str, int n)`
/// (0x318350): copy at most `n` bytes of `str` into a new BX string.
pub fn c_bx_string_ctor_cstr_n(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_8380);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x0031_8380)?;
        c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0031_8394);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_6810, 0x0031_8394)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let empty = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !empty {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0031_83a8);
            c.g[A1][0] = c.g[S0][0];
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_83a8)?;
            c.g[A0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_83b8);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x0031_83b8)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_ConcatCopy(cBXString* self, int len1, const char* s1,
/// int len2, const char* s2)` (0x318540): a new string of the two pieces (the
/// generated call order puts the lengths in `a1`/`a3`).
pub fn c_bx_string_concat_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = sx(p(r32(c, S1), r32(c, S3)));
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let empty = c.g[A1][0] == 0;
        c.g[S4][0] = c.g[T0][0];
        if !empty {
            c.cycles += 11;
            c.g[RA][0] = sx(0x0031_8580);
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_8580)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x0031_8590);
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8590)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0031_85a4);
            c.g[A0][0] = sx(p(r32(c, A0), r32(c, S1)));
            call_ptr(c, 0x0041_605c, 0x0031_85a4)?;
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
    })
}

// ---------------------------------------------------------------------------
// cBXString.

/// `cBXString* cBXString_OpPlusCStrStr(cBXString* self, char* str,
/// cBXString* b)` (0x3186d0): `*self = str + *b` through a temporary string.
pub fn c_bx_string_op_plus_c_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(gp_3(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        Addr(r32(c, SP)).sw(c, c.g[V0][0]);
        let mut len = 0u64;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8704)?;
            len = c.g[V0][0];
        } else {
            c.cycles += 1;
        }
        c.g[A1][0] = len;
        c.cycles += 19;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = Addr(r32(c, V0).wrapping_sub(8)).lw(c);
        c.g[T0][0] = c.g[V0][0];
        callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_8728)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        callee::CBX_STRING_CTOR_COPY_2.call(c, 0x0031_8734)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::CBX_STRING_DTOR_2.call(c, 0x0031_8740)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString& cBXString::operator=(const cBXString& other)` (0x318448):
/// share the other string when both headers are writable, otherwise copy it.
pub fn cbxstring_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = lw_2(c, c.g[S1][0] as u32);
        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
        let same = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = c.g[S1][0];
        if same {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4));
        let mut copy = false;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = lw_2(c, p(c.g[A0][0] as u32, 0xffff_fff4));
            c.cycles += 2;
            if (c.g[V0][0] as i64) < 0 {
                c.cycles += 1;
                c.g[A2][0] = lw_2(c, c.g[S0][0] as u32);
                copy = true;
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_dcd4;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A2][0] = lw_2(c, c.g[S0][0] as u32);
                copy = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = lw_2(c, p(c.g[A0][0] as u32, 0xffff_fff4));
                c.cycles += 2;
                copy = (c.g[V0][0] as i64) < 0;
            }
        }
        if copy {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw_2(c, p(c.g[A2][0] as u32, 0xffff_fff8));
            call_ptr(c, 0x0031_83d8, 0x0031_84a8)?; // AssignCopy
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_80b8, 0x0031_84b8)?; // Release
            c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
            sw_2(c, c.g[S1][0] as u32, c.g[V1][0]);
            c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            sw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cBXString cBXString::operator+(const char* rhs)` (0x318630): concatenate
/// `rhs` after this string and return the result object.
pub fn cbxstring_op_plus_str_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = lw_2(c, gp(c, 0xda0));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = lw_2(c, c.g[S1][0] as u32);
        sw_2(c, c.g[SP][0] as u32, c.g[V0][0]);
        let nil = c.g[S0][0] == 0;
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(8));
        let mut len = 0u64;
        if !nil {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8670)?; // strlen
            len = c.g[V0][0];
        } else {
            c.cycles += 2;
            len = 0;
        }
        c.g[A2][0] = lw_2(c, c.g[S1][0] as u32);
        c.cycles += 19;
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = lw_2(c, c.g[S2][0] as u32);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = len;
        call_ptr(c, 0x0031_8540, 0x0031_8694)?; // ConcatCopy
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_86a0)?; // ctor_copy
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_86ac)?; // dtor
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cBXString.

/// `char* cBXString_GetBuffer(cBXString* s, int min_len)` (0x3189d0): return a
/// writable buffer of at least `min_len` bytes, copying a shared buffer and
/// growing a short one first.
pub fn cbx_string_get_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'shared: {
                frame.save(c, S1, 0x30);
                frame.save(c, S2, 0x20);
                let s = r(c, A0);
                c.g[S1][0] = sx(s);
                frame.save(c, S0, 0x40);
                frame.save_ra(c, 0x10);
                c.g[V1][0] = Addr(s).lw(c);
                c.g[V0][0] = Addr(p(r(c, V1), (-0xc_i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let shared = c.g[V0][0] == 0;
                c.g[S2][0] = c.g[A1][0];
                if shared {
                    break 'shared;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(p(r(c, V1), (-4i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, S1)).lw(c);
                    break 'outer;
                }
            }
            c.cycles += 25;
            c.g[A0][0] = sx(r(c, SP));
            c.g[RA][0] = sx(0x0031_8a18);
            c.g[A1][0] = c.g[S1][0];
            callee::BXSTRING_CTOR_COPY.call(c, 0x0031_8a18)?;
            c.g[V1][0] = Addr(r(c, SP)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_8a28);
            c.g[S0][0] = Addr(p(r(c, V1), (-8i32) as u32)).lw(c);
            callee::BXSTRING_RELEASE.call(c, 0x0031_8a28)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[S0][0];
            }
            c.g[RA][0] = sx(0x0031_8a3c);
            c.g[A0][0] = c.g[S1][0];
            callee::BXSTRING_ALLOC_BUFFER.call(c, 0x0031_8a3c)?;
            c.g[A0][0] = Addr(r(c, S1)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0031_8a4c);
            c.g[A1][0] = Addr(r(c, SP)).lw(c);
            call_ptr(c, 0x0041_605c, 0x0031_8a4c)?;
            c.g[V1][0] = Addr(r(c, S1)).lw(c);
            c.g[A0][0] = sx(r(c, SP));
            c.g[A1][0] = 0x2;
            Addr(p(r(c, V1), (-8i32) as u32)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
            c.g[RA][0] = sx(0x0031_8a6c);
            Addr(r(c, V0)).sb(c, 0);
            callee::BXSTRING_DTOR.call(c, 0x0031_8a6c)?;
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
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
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;
    pub(crate) use crate::game::be::bedbreward::tests::call_setup;
    pub(crate) use crate::game::fe::debugfe::tests::pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::gamemode::tests::w as w_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_4;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_7;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    pub(crate) const SEEDS: u32 = 200;

    // ------------------------------------------------------- strings

    #[test]
    fn cbxstring_append_str_matches() {
        check(
            ee::s10::f_003189a0__generated,
            cbxstring_append_str,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3189a0, 0);
                // Destination cBXString: `self->text` at DATA+0x200, header at
                // -0xc (>= 2 selects the in-place path), -8 (length), -4
                // (capacity, large enough for the append).
                let buffer = DATA + 0x200;
                let length = (seed % 5) as u32;
                c.bus.write32(DATA, buffer);
                c.bus.write32(buffer - 0xc, 2);
                c.bus.write32(buffer - 8, length);
                c.bus.write32(buffer - 4, 0x1000);
                let src = DATA + 0x300;
                c.bus.write32(DATA + 0x40, src);
                c.bus.write32(src - 8, 1 + (seed % 7));
                for i in 0..16u32 {
                    c.bus.write8(src + i, (r.next_u32() & 0x7f) as u8);
                    c.bus
                        .write8(buffer + length + i, (r.next_u32() & 0x7f) as u8);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x40) as u64;
            },
        );
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn step(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        step(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x1_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    // ------------------------------------------------------------------
    // Bx strings and spheres.

    #[test]
    fn c_bx_string_set_at_matches() {
        check(
            ee::s10::f_00318e68__generated,
            c_bx_string_set_at,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318e_68, 1);
                alloc_ready(c);
                let str_ = DATA + 0x100;
                let buf = DATA + 0x1000;
                clear(c, str_, 0x10);
                c.bus.write32(str_, buf);
                for i in 0..0x20u32 {
                    c.bus.write8(buf + i, r.next_u32() as u8 | 1);
                }
                // The buffer's reference count sits 0xc bytes before it: 0/1
                // keeps the buffer (the writable path).
                c.bus.write32(buf - 0xc, (seed % 2) as u32);
                c.bus.write32(buf - 8, 0x10);
                c.g[A0][0] = sx(str_);
                c.g[A1][0] = (seed % 0x10) as u64;
                c.g[A2][0] = [0x41u64, 0x7f, 0x81, 0x100][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn c_bx_string_ctor_copy_matches() {
        check(
            ee::s10::f_00317fe8__generated,
            c_bx_string_ctor_copy,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x317fe8, 1);
                alloc_ready_2(c);
                let this = DATA + 0x100;
                let other = DATA + 0x200;
                let data = DATA + 0x300;
                clear(c, this, 0x40);
                clear(c, other, 0x40);
                clear(c, data - 0x10, 0x40);
                c.bus.write32(other, data);
                c.bus
                    .write32(data - 0xc, if seed % 3 == 0 { 1 } else { 0xffff_ffff });
                c.bus.write8(data, b'a');
                c.bus.write8(data + 1, b'b');
                c.bus.write32(gp_at(0xda0), data + 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(other);
            },
        );
    }

    #[test]
    fn c_bx_string_append_cstr_matches() {
        check(
            ee::s10::f_00318910__generated,
            c_bx_string_append_cstr,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x318910, 2);
                alloc_ready_2(c);
                let this = DATA + 0x100;
                let data = DATA + 0x300;
                let text = DATA + 0x400;
                clear(c, this, 0x40);
                clear(c, data - 0x10, 0x80);
                clear(c, text, 0x40);
                c.bus.write32(this, data);
                c.bus.write32(data - 0xc, 1);
                c.bus.write32(data - 4, 0x20);
                c.bus.write8(data, b'a');
                c.bus.write8(text, b'h');
                c.bus.write8(text + 1, b'i');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
            },
        );
    }

    #[test]
    fn c_bx_string_alloc_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318188__generated,
            c_bx_string_alloc_before_write,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x318188, 0);
                pool_allocator(c);
                let buf = OBJ2;
                // Cover all three paths of the reference/capacity test:
                //  seed%4 == 0/1: unique reference, buffer big enough  -> early
                //                 return after the (charged) size test;
                //  seed%4 == 2:   shared reference (refcount >= 2)     -> rebuild
                //                 with no size test;
                //  seed%4 == 3:   refcount sign-extends negative, so the size
                //                 test runs and rebuilds.  (`refcount == 1`
                //                 would make `cBXString_Release` free the fake
                //                 buffer, so it is left to its own test.)
                let (refcount, capacity) = match seed % 4 {
                    0 => (1u32, 0x1000u32),
                    1 => (0, 0x1000),
                    2 => (5, 0x10),
                    _ => (0xffff_ffff, 0x10),
                };
                w(c, buf - 0xc, refcount);
                w(c, buf - 4, capacity);
                w(c, OBJ, buf);
                w(c, buf, 0x41);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = ((seed * 37) % 200) as u64;
            },
        );
    }

    #[test]
    fn c_bx_string_release_matches() {
        check(
            ssx3_ee::s10::f_003180b8__generated,
            c_bx_string_release,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x3180b8);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA, EMPTY),
                    1 => {
                        c.bus.write32(DATA, DATA + 0x40);
                        c.bus.write32(DATA + 0x34, 2 + seed % 3);
                        c.bus.write32(DATA + 0x38, 0);
                    }
                    _ => {
                        // A buffer at 0xc: its header is at zero, so the release
                        // drops the count without calling the allocator.
                        c.bus.write32(DATA, 0xc);
                        c.bus.write32(0, 1);
                    }
                }
            },
        );
    }

    #[test]
    fn c_bx_string_copy_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318120__generated,
            c_bx_string_copy_before_write,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x318120);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    // Shared buffer (refcount 2): release, reallocate zero bytes
                    // (the static empty string) and copy one byte.
                    c.bus.write32(DATA, DATA + 0x40);
                    c.bus.write32(DATA + 0x34, 2);
                    c.bus.write32(DATA + 0x38, 0);
                } else {
                    // Private buffer (refcount 1, header at zero): the early out,
                    // which leaves the `refcount < 2` result in v0.
                    c.bus.write32(DATA, 0xc);
                    c.bus.write32(0, 1);
                    c.bus.write32(4, 0);
                }
            },
        );
    }

    #[test]
    fn c_bx_string_op_plus_str_str_matches() {
        check(
            ssx3_ee::s10::f_003185c8__generated,
            c_bx_string_op_plus_str_str,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x3185c8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                // Two empty strings: the concatenation stays the static empty one.
                c.bus.write32(DATA + 0x100, DATA + 0x300);
                c.bus.write32(DATA + 0x2f4, 0xffff_ffff);
                c.bus.write32(DATA + 0x2f8, 0);
                c.bus.write32(DATA + 0x300, 0);
                c.bus.write32(DATA + 0x200, DATA + 0x340);
                c.bus.write32(DATA + 0x334, 0xffff_ffff);
                c.bus.write32(DATA + 0x338, 0);
                c.bus.write32(DATA + 0x340, 0);
            },
        );
    }

    #[test]
    fn cbx_string_make_upper_matches() {
        check(
            ee::s10::f_00318d28__generated,
            cbx_string_make_upper,
            60,
            &|c, seed| {
                base(c, seed, 0x31_8d28);
                let buf = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, buf);
                // Private buffer: `CopyBeforeWrite` takes its early out, so the
                // characters are uppercased in place.
                c.bus.write32(buf - 0xc, 1);
                let len = seed % 6;
                c.bus.write32(buf - 8, len);
                for i in 0..len {
                    c.bus.write8(buf + i, b"aZ0_~"[((seed + i) % 5) as usize]);
                }
                // The class table at 0x499c81: bit 1 marks lowercase letters.
                for ch in [b'a', b'z'] {
                    c.bus
                        .write8(0x0049_9c81 + ch as u32, if seed % 2 == 0 { 2 } else { 0 });
                }
                c.bus.write8(0x0049_9c81 + b'A' as u32, 0);
            },
        );
    }

    pub(crate) fn put_str(c: &mut Cpu, at_2: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at_2 + i as u32, *b);
        }
    }

    #[test]
    fn cbx_string_assign_copy_matches() {
        check(
            ee::s10::f_003183d8__generated,
            cbx_string_assign_copy,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3183_d8);
                let len = 4 + (seed % 9);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                c.g[A1][0] = len as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 8);
            },
        );
    }

    #[test]
    fn cbx_string_release_buffer_matches() {
        check(
            ee::s10::f_00318a88__generated,
            cbx_string_release_buffer,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x318a_88);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                put_str(c, buf, b"hello\0");
                c.g[A1][0] = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (seed % 12) as u64
                };
                fill(c, &mut r, DATA + 0x400, 4);
            },
        );
    }

    #[test]
    fn cbx_string_alloc_buffer_matches() {
        check(
            ee::s10::f_00318040__generated,
            cbx_string_alloc_buffer,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x318040, 2);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
                let size = if seed % 4 == 0 { 0 } else { 1 + seed % 0x30 };
                c.g[A1][0] = size as u64;
            },
        );
    }

    #[test]
    fn cbx_string_left_matches() {
        check(
            ee::s10::f_003190a8__generated,
            cbx_string_left,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x3190a8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let src = DATA + 0x400;
                string(c, src, "hello");
                w_2(c, src - 8, 5);
                w_2(c, DATA + 0x200, src);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 0x10,
                    _ => u32::MAX, // -1: "all"
                } as u64;
            },
        );
    }

    #[test]
    fn cbx_string_alloc_copy_matches() {
        check(
            ee::s10::f_003181e8__generated,
            cbx_string_alloc_copy,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3181e8, 2);
                be_library(c);
                heap_init(c);
                let dst = DATA;
                let src = DATA + 0x800;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                c.g[T0][0] = if seed % 4 == 0 { 0 } else { 1 };
                w_3(c, src, DATA + 0x1000);
                string(c, DATA + 0x1000, "abcdefgh");
            },
        );
    }

    #[test]
    fn cbx_string_dtor_matches() {
        check(
            ee::s10::f_00318260__generated,
            cbx_string_dtor,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x318260, 2);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                if seed % 4 == 0 {
                    w_3(c, obj, 0x0048_dcd4);
                } else {
                    let buf = DATA + 0x1000;
                    w_3(c, obj, buf);
                    w_3(c, buf - 0xc, if seed % 4 == 1 { 1 } else { 2 + seed % 3 });
                }
            },
        );
    }

    #[test]
    fn cbx_string_ctor_cstr_matches() {
        check(
            ee::s10::f_003182d8__generated,
            cbx_string_ctor_cstr,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3182d8, 2);
                be_library(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x1000) as u64
                };
                string(c, DATA + 0x1000, if seed % 2 == 0 { "hello" } else { "" });
            },
        );
    }

    #[test]
    fn c_bx_string_right_matches() {
        check(
            ee::s10::f_00319028__generated,
            c_bx_string_right,
            12,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x3190_28, 4);
                let dst = DATA;
                let src = DATA + 0x1000;
                let buf = DATA + 0x2000;
                let len = 1 + r.next_u32() % 12;
                w_4(c, src, buf);
                w_4(c, buf - 8, len);
                for i in 0..len {
                    wb(c, buf + i, b'a' + (i % 26) as u8);
                }
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => (r.next_u32() % 16) as u64,
                    1 => 0,
                    _ => (-1i64) as u64,
                };
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    pub(crate) fn heap_init_3(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }

    // ------------------------------------------------------------------
    // BX strings.

    #[test]
    fn c_bx_string_ctor_cstr_n_matches() {
        check(
            ssx3_ee::s10::f_00318350__generated,
            c_bx_string_ctor_cstr_n,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318350, 3);
                be_library(c);
                world(c);
                heap_init_3(c);
                let this = DATA + 0x100;
                c.g[A0][0] = this as u64;
                let s = DATA + 0x200;
                let len = (r.next_u32() % 8) as usize;
                for i in 0..0x20 {
                    b(c, s + i as u32, b'a' + (i % 26) as u8);
                }
                b(c, s + len as u32, 0);
                c.g[A1][0] = s as u64;
                c.g[A2][0] = (r.next_u32() % 0x20) as u64;
            },
        );
    }

    #[test]
    fn c_bx_string_concat_copy_matches() {
        check(
            ssx3_ee::s10::f_00318540__generated,
            c_bx_string_concat_copy,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318540, 3);
                be_library(c);
                world(c);
                heap_init_3(c);
                let this = DATA + 0x100;
                let s1 = DATA + 0x200;
                let s2 = DATA + 0x280;
                for i in 0..0x40 {
                    b(c, s1 + i, b'a' + (i % 26) as u8);
                    b(c, s2 + i, b'A' + (i % 26) as u8);
                }
                c.g[A0][0] = this as u64;
                // (len1, s1, len2, s2) as the generated call order shows.
                c.g[A1][0] = (r.next_u32() % 0x20) as u64;
                c.g[A2][0] = s1 as u64;
                c.g[A3][0] = (r.next_u32() % 0x20) as u64;
                c.g[T0][0] = s2 as u64;
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena.
    pub(crate) fn heap_init_4(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0xdead_0100;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    /// The engine's size-class pool allocator: `gp_3 + 0xdd8` ready plus eight
    /// free nodes per size class.
    pub(crate) fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_6(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_4 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_4 + 0x400 * n;
                w_6(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_6(c, pool + head, base_4);
            w_6(c, GAME_GP.wrapping_add(slot), pool);
        }
    }

    #[test]
    fn c_bx_string_op_plus_c_str_str_matches() {
        check(
            ee::s10::f_003186d0__generated,
            c_bx_string_op_plus_c_str_str,
            100,
            &|c, seed| {
                let _ = base_4(c, seed, 0x31_86d0);
                globals(c);
                pool(c);
                heap_init_4(c);
                // A cBXString at DATA+0x2000 whose buffer has a length word at -8.
                const BUF: u32 = DATA + 0x2100;
                w_6(c, DATA + 0x2000, BUF);
                w_6(c, BUF - 8, 4);
                for i in 0..8u32 {
                    c.bus.write8(BUF + i, b'x' + i as u8);
                }
                // The global stack temporary (a cBXString) starts out empty.
                w_6(c, 0x004a_3e90, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { sx(DATA + 0x3000) } else { 0 };
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x3000 + i, b'a' + i as u8);
                }
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for the
    /// paths that allocate.
    pub(crate) fn heap(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0x0012_3450;
        for (i, a) in [MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000]
            .into_iter()
            .enumerate()
        {
            c.g[[A0, A1, A2][i]][0] = a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_ff60);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    pub(crate) fn put_str_2(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().chain([0u8]).enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
    }

    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = size as u64;
        c.g[A1][0] = 0x004a_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7d70;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7d70);
        assert_eq!(c.pc, RET, "EA_MemAllocNamed did not return");
        c.g[V0][0] as u32
    }

    /// A cBXString block at `data`: header (refcount, length, capacity) and
    /// text.
    pub(crate) fn string_block(c: &mut Cpu, data: u32, refcount: i32, text: &str) {
        clear_2(c, data - 0x10, 0x10);
        w_7(c, data - 0xc, refcount as u32);
        w_7(c, data - 8, text.len() as u32);
        w_7(c, data - 4, 0x100);
        put_str_2(c, data, text);
    }

    // ------------------------------------------------------------- equipment

    // ---------------------------------------------------------------- strings

    #[test]
    fn cbxstring_assign_matches() {
        check(
            ssx3_ee::s10::f_00318448__generated,
            cbxstring_assign,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0031_8448);
                heap(c);
                let a = DATA + 0x100;
                let b = DATA + 0x110;
                let da = alloc(c, 0x120);
                let da = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
                string_block(c, da + 0x10, if seed % 3 == 2 { -1 } else { 2 }, "left");
                let db = DATA + 0x3000;
                string_block(c, db, 2, "right");
                w_7(c, a, if seed % 5 == 0 { db } else { da + 0x10 });
                w_7(c, b, db);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
            },
        );
    }

    #[test]
    fn cbxstring_op_plus_str_cstr_matches() {
        check(
            ssx3_ee::s10::f_00318630__generated,
            cbxstring_op_plus_str_cstr,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0031_8630);
                heap(c);
                let src_obj = DATA + 0x100;
                let dst_obj = DATA + 0x110;
                let da = alloc(c, 0x120);
                let data = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
                string_block(c, data, 2, "hello");
                w_7(c, src_obj, data);
                w_7(c, dst_obj, 0);
                put_str_2(c, DATA + 0x3000, " world");
                w_7(c, GAME_GP.wrapping_add(0xda0), 0x0048_dcd4);
                c.g[A0][0] = sx(dst_obj);
                c.g[A1][0] = sx(src_obj);
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x3000) };
            },
        );
    }

    // ---------------------------------------------------------- cBXString

    #[test]
    fn cbx_string_get_buffer_matches() {
        check(
            ee::s10::f_003189d0__generated,
            cbx_string_get_buffer,
            40,
            &|c, seed| {
                alloc_base_2(c, seed, 0x3189, 4);
                let s = DATA;
                let buf = DATA + 0x1_000;
                for i in 0..0x40u32 {
                    w_8(c, buf + 4 * i, 0x41);
                }
                w_8(c, s, buf);
                // 3 = shared buffer (copy path), 1 = unique with room (fast path).
                let shared = seed % 2 == 0;
                w_8(c, buf.wrapping_sub(0xc), if shared { 3 } else { 1 });
                w_8(c, buf.wrapping_sub(4), 0x100);
                w_8(c, buf.wrapping_sub(8), 4);
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (if shared { 8 } else { 2 }) as u64;
            },
        );
    }
}
