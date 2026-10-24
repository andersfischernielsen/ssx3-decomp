//! `src/ui/uimenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::camera::cameracontroller::delete_owner;
pub(crate) use crate::game::dirtysock::tags::lbu;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_3;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
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
    (0x0039_b6a0, ui_menu_render, "UIMenu_Render"),
    (0x0039_b760, uimenu_reset_scroll, "UIMenu_ResetScroll"),
    (
        0x0039_b7b0,
        ui_menu_find_first_selectable,
        "UIMenu_FindFirstSelectable",
    ),
    (0x0039_bad0, uipair_v04, "UIPair_v04"),
    (0x0039_bb50, ui_pair_set_visible, "UIPair_SetVisible"),
    (0x0039_bbd8, ui_pair_set_focus, "UIPair_SetFocus"),
    (0x0039_bd38, ui_pair_accept, "UIPair_Accept"),
    (0x0039_bd68, ui_pair_v03, "UIPair_v03"),
    (0x0039_be10, ui_pair_refresh_text, "UIPair_RefreshText"),
    (0x0039_be70, ui_pair_free_children, "UIPair_FreeChildren"),
    (0x0039_c130, ui_pair_render, "UIPair_Render"),
    (0x0039_c1c8, ui_pair_update_anim, "UIPair_UpdateAnim"),
    (0x0039_c240, c_ui_progress_dtor, "cUIProgress_dtor"),
    (0x0039_c2c8, cui_progress_load, "cUIProgress_Load"),
    (0x0039_c300, cui_progress_render, "cUIProgress_Render"),
    (
        0x0039_c398,
        c_ui_progress_update_anim,
        "cUIProgress_UpdateAnim",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
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
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
}

/// `UIPair::Accept(self, node)` (0x39bd38): `node->v02(self)`.
pub fn ui_pair_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_bd5c, None))
}

/// `void cUIProgress_Load(cUIProgress* self)` (0x39c2c8): load the base menu
/// object, then set the bar's min (10) and max (100).
pub fn cui_progress_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_c2dc)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x84).sw(c, v0);
        Addr(s0 as u32).off(0x80).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu_ResetScroll(UIMenu* m, int count)` (0x39b760): clear the
/// scroll cursor, remember the item count, flag the widget dirty (`+0x90 |=
/// 4`) and reset the first visible row.
pub fn uimenu_reset_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sb(r32(c, S0).wrapping_add(0x95), 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x74));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x90));
        c.sb(r32(c, S0).wrapping_add(0x96), c.g[A1][0]);
        c.g[V0][0] |= 4;
        c.sw(r32(c, S0).wrapping_add(0x90), c.g[V0][0]);
        callee::UI_LIST_COUNT.call(c, 0x0039_b78c)?;
        c.sb(r32(c, S0).wrapping_add(0x97), c.g[V0][0]);
        c.sb(r32(c, S0).wrapping_add(0x98), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_RefreshText(UIPair* pair)` (0x39be10): refresh the two text
/// widgets of the pair.
pub fn ui_pair_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let pair = c.g[A0][0];
        c.g[S0][0] = pair;
        c.g[A1][0] = Addr(at(c, S0, 0x78)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x84).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be40)?;
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        } else {
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        }
        'second: {
            c.cycles += 2;
            let none = c.g[A1][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'second;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x84).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_FreeChildren(UIPair* self)` (0x39be70): destroy the two child
/// objects (vtable slot 1 with argument 3) and clear the pointers.
pub fn ui_pair_free_children(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        let a_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !a_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bea0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0039_bea0)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let b_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !b_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bec4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0039_bec4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI pair / texture manager.

/// `func_0039C1C8` (0x39c1c8): `UIPair_UpdateAnim`. Updates the pair itself
/// (`UIObject_UpdateAnim`) and then each of its two children at `+0x78` and
/// `+0x7c` through `vtable[0x98]`. Signature: `void update_anim(this, flags)`.
pub fn ui_pair_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_c1e8);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_c1e8)?;
        c.g[A2][0] = lw_r(c, S0, 0x78);
        let no_first = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_first {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c208);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c208)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x7c);
        let no_second = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_second {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c228);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c228)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_v04(self, on)` (0x39bad0): store the pair's on flag
/// (`self+0x14`, bit 1) and notify the two child widgets through their
/// virtual `on` handlers.
pub fn uipair_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[V1][0] = lw_at(c, p(r32(c, S0), 0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x78));
        c.g[V1][0] &= c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sw(p(r32(c, S0), 0x14), c.g[V1][0]);
        c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
        let func = vslot(c, c.g[V0][0], 0x20, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, func, 0x0039_bb1c)?;
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x7c));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
            let func = vslot(c, c.g[V0][0], 0x20, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_bb3c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `void UIPair_SetVisible(UIPair* self, int on)` (0x39bb50): set the pair's
/// "visible" bit and forward the visibility to both children.
pub fn ui_pair_set_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffff_ffff_ffff_ffbf;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let null = c.g[A2][0] == 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bba0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bba0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bbc0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bbc0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_dtor(cUIProgress* self, int flags)` (0x39c240): release
/// the two child objects and run the base destructor.
pub fn c_ui_progress_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4408);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_c284)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_c2a4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_c2b0);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_c2b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_UpdateAnim(self, unsigned short ev)` (0x39c398): advance
/// the progress animator, then pass the event to the `+0x78` and `+0x74`
/// virtual objects.
pub fn c_ui_progress_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let ev = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0xc));
        let no_anim = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        if !no_anim {
            c.cycles += 7;
            c.g[A2][0] = c.lbu(at_2(c, S0, 0x10));
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, callee::UI_ANIM_APPLY_2, 0x0039_c3c8)?;
            c.g[A0][0] = c.lw(at_2(c, S0, 0xc));
            c.g[A1][0] = c.lbu(at_2(c, S0, 0x10));
            call_ptr(c, callee::UI_ANIM_ADVANCE_2, 0x0039_c3d4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at_2(c, S0, 0x78));
        let no_a = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_a {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at_2(c, A2, 8));
            c.g[A0][0] = c.lh(at_2(c, V0, 0x98));
            c.g[V1][0] = c.lw(at_2(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c3f4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at_2(c, S0, 0x74));
        let no_b = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_b {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at_2(c, A2, 8));
            c.g[A0][0] = c.lh(at_2(c, V0, 0x98));
            c.g[V1][0] = c.lw(at_2(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c414)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_Render(self, x, y)` (0x39c130): forward the pair's render to
/// the two child objects at_3 +0x78 and +0x7C, offset by the pair's own position.
pub fn ui_pair_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 21, 0x28);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, S0, 0x78);
        let none = c.g[A1][0] == 0;
        c.mov_s(20, 13);
        if !none {
            c.cycles += 9;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(12, 21, 12);
            c.add_s(13, 20, 13);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c17c)?;
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, S0, 0x7c);
        if c.g[A1][0] != 0 {
            c.cycles += 10;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(13, 20, 13);
            c.add_s(12, 21, 12);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c1ac)?;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_Render(self, x, y)` (0x39c300): render the progress pair's
/// two child objects when both are present.
pub fn cui_progress_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.g[A1][0] = lw(c, S0, 0x74);
        let mut draw = c.g[A1][0] != 0;
        if draw {
            c.cycles += 3;
            c.g[V0][0] = lw(c, S0, 0x78);
            draw = c.g[V0][0] != 0;
        }
        if draw {
            c.cycles += 20;
            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = lw(c, A1, 0x8);
            c.add_s(20, 12, 20);
            c.add_s(21, 13, 21);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.mov_s(12, 20);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.mov_s(13, 21);
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c35c)?;
            c.g[A1][0] = lw(c, S0, 0x78);
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[V0][0] = lw(c, A1, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c37c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* UIPair_v03(self, pred)` (0x39bd68): return `self` if the pair's
/// predicate matches, else the first matching child or next node.
pub fn ui_pair_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let pred = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, pred, 0x0039bd98)?;
        let hit = c.g[V0][0] != 0;
        let mut tail = false;
        if hit {
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
            let no_child = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if no_child {
                c.cycles += 1;
                c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                tail = true;
            } else {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdc0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
                } else {
                    c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                    tail = true;
                }
            }
        }
        if tail {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdf8)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// `UIMenu_Render` (0x0039b6a0).
//
// `void UIMenu::Render(UIMenu* self, float x, float y)`: refresh the scroll
// range, then walk the item list, offsetting each item by the menu position
// and issuing its render call.

pub fn ui_menu_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_39b6dc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.g[S1][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.mov_s(21, 12);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
            let t = c.g[A0][0] == 0u64;
            c.mov_s(20, 13);
            if t {
                break 'b_39b6dc;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x98));
                c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x97));
                c.g[RA][0] = sx(0x0039b6dc);
                c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                call_ptr(c, 0x0039da20, 0x0039b6dc)?;
                break 'b_39b6dc;
            }
        }
        'b_39b744: {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0039b6e4);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
            call_ptr(c, 0x00397840, 0x0039b6e4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_39b744;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                'l_39b6f8: loop {
                    'b_39b730: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_39b730;
                        } else {
                            c.cycles += 10;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x44));
                            c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                            c.add_s(12, 12, 21);
                            c.add_s(13, 13, 20);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039b72c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0039b72c)?;
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_39b730;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039b738);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00397718, 0x0039b738)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0039b6f8);
                        }
                        continue 'l_39b6f8;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_39b744;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

/// `int UIMenu_FindFirstSelectable(UIMenu *self, u32 skip)` (0x39b7b0): the
/// first enabled list entry after `skip`. A resumable loop (0x39b7e8).
pub fn ui_menu_find_first_selectable(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        let mut frame = Frame { size: 0x30 };
        loop {
            match pc {
                0x0039_b7b0 => {
                    c.cycles += 13;
                    frame = Frame::open(c, 0x30);
                    c.g[A1][0] &= 0xff;
                    frame.save(c, S0, 0x20);
                    c.g[V1][0] = c.g[A1][0] ^ 0xff;
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = 0;
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x96)).lbu(c);
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xff;
                    if done {
                        pc = 0x0039_b864;
                    } else {
                        pc = 0x0039_b7e4;
                    }
                }
                0x0039_b7e4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    pc = 0x0039_b7e8;
                }
                0x0039_b7e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7870, 0x0039_b7f0)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x90)).lw(c);
                    c.g[V0][0] &= 0x4;
                    let no_flag = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    if no_flag {
                        pc = 0x0039_b838;
                    } else {
                        pc = 0x0039_b804;
                    }
                }
                0x0039_b804 => {
                    c.cycles += 11;
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x5c)).lw(c);
                    c.g[A2][0] = 1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = Addr(p_2(c.g[V0][0], 0xd0)).lw(c);
                    c.g[V1][0] = Addr(p_2(c.g[T0][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p_2(c.g[V1][0], 0xa0)).lh(c);
                    c.g[V0][0] = Addr(p_2(c.g[V1][0], 0xa4)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_b828)?;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        pc = 0x0039_b84c;
                    } else {
                        pc = 0x0039_b830;
                    }
                }
                0x0039_b830 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0039_b864;
                }
                0x0039_b838 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_2(c.g[V1][0], 0x14)).lw(c);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 5) as u32);
                    c.g[V0][0] &= 1;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b830;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b830;
                    } else {
                        pc = 0x0039_b84c;
                    }
                }
                0x0039_b84c => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x96)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    if more {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b7e8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b7e8;
                    } else {
                        pc = 0x0039_b860;
                    }
                }
                0x0039_b860 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xff;
                    pc = 0x0039_b864;
                }
                0x0039_b864 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_b7b0, 0x0039_b878);
                    return Ok(());
                }
            }
        }
    });
}

/// `void func_0039BBD8(void* pair, int on)` (0x39bbd8): show/hide a UI pair.
/// Stores `on` in bit 4 of the flags at_4 `+0x14`, tells the entry at_4 `+0x78`
/// (`+8` vtable, slot 7), and, when there is a keyframer object at_4 `+0x7c`,
/// looks up its event for the wanted state, plays that frame on the screen at_4
/// `+0x5c` when one is found, and finally re-sets the keyframer and the owner
/// (slot 19 with 1 or 2).
pub fn ui_pair_set_focus(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_39bc28: {
        c.cycles += 15;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffffffffffffffef;
        c.g[V1][0] = lw_3(c, at_4(c, S0, 0x14));
        c.g[V0][0] = sx(r32(c, V0) << 4);
        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x78));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        sw(c, at_4(c, S0, 0x14), c.g[V1][0]);
        if t {
            break 'b_39bc28;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bc28);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bc28).is_err() {
                return;
            }
            break 'b_39bc28;
        }
    }
    'b_39bd28: {
        c.cycles += 3;
        c.g[A0][0] = lw_3(c, at_4(c, S0, 0x7c));
        if c.g[A0][0] == 0u64 {
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        } else {
            'b_39bcb4: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;

                if t {
                    break 'b_39bcb4;
                } else {
                    'b_39bc70: {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0039bc44);
                        c.g[A1][0] = 0u64;
                        if callee::C0039ff78.call(c, 0x0039bc44).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                            break 'b_39bc70;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = lw_3(c, at_4(c, S0, 0x7c));
                            c.g[A0][0] = lw_3(c, at_4(c, V0, 0x5c));
                            if c.g[A0][0] == 0u64 {
                                c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                                break 'b_39bc70;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0039bc6c);
                                c.g[A2][0] = 0x1;
                                if callee::C0039c870.call(c, 0x0039bc6c).is_err() {
                                    return;
                                }
                                c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                                break 'b_39bc70;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
                    c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bc88);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bc88).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw_3(c, at_4(c, S0, 0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = lw_3(c, at_4(c, S0, 0x7c));
                    c.g[A3][0] = lw_3(c, at_4(c, A0, 0xd0));
                    c.g[V0][0] = lw_3(c, at_4(c, A3, 0x8));
                    c.g[A0][0] = lh_2(c, at_4(c, V0, 0x98));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0x9c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bcac);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bcac).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_39bd28;
                    } else {
                        break 'b_39bcb4;
                    }
                }
            }
            'b_39bce8: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x0039bcbc);
                c.g[A1][0] = 0x1;
                if callee::C0039ff78.call(c, 0x0039bcbc).is_err() {
                    return;
                }
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                if c.g[A1][0] == c.g[V0][0] {
                    c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                    break 'b_39bce8;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = lw_3(c, at_4(c, S0, 0x7c));
                    c.g[A0][0] = lw_3(c, at_4(c, V0, 0x5c));
                    if c.g[A0][0] == 0u64 {
                        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                        break 'b_39bce8;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0039bce4);
                        c.g[A2][0] = 0x1;
                        if callee::C0039c870.call(c, 0x0039bce4).is_err() {
                            return;
                        }
                        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                        break 'b_39bce8;
                    }
                }
            }
            c.cycles += 16;
            c.g[A1][0] = 0u64;
            c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd00);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd00).is_err() {
                return;
            }
            c.g[A0][0] = lw_3(c, at_4(c, S0, 0x5c));
            c.g[A2][0] = 0x2;
            c.g[A1][0] = lw_3(c, at_4(c, S0, 0x7c));
            c.g[A3][0] = lw_3(c, at_4(c, A0, 0xd0));
            c.g[V0][0] = lw_3(c, at_4(c, A3, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x98));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x9c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd24);
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd24).is_err() {
                return;
            }
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x30));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::camera::cameracontroller::tests::fake_child;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::CHILD;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fereal::tests::fake_slot;
    pub(crate) use crate::game::fe::fereal::tests::SELF2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_6;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;
    pub(crate) use crate::game::fe::festaterewards::tests::w16 as w16_2;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ui_pair_accept_matches() {
        check(
            ee::s13::f_0039bd38__generated,
            ui_pair_accept,
            SEEDS,
            &|c, seed| {
                virtual_setup(c, seed, 0x39bd_38, 0x10);
            },
        );
    }

    #[test]
    fn cui_progress_load_matches() {
        check(
            ee::s13::f_0039c2c8__generated,
            cui_progress_load,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c2_c8, 0);
                let obj = DATA + 0x400;
                let desc = DATA + 0x600;
                fill(c, &mut r, obj, 0x40);
                fill(c, &mut r, desc, 0x20);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(desc);
                c.bus.write32(obj + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x48);
            },
        );
    }

    #[test]
    fn uimenu_reset_scroll_matches() {
        check(
            ee::s13::f_0039b760__generated,
            uimenu_reset_scroll,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_b760, 0);
                let m = DATA + 0x1000;
                c.bus.write32(m + 0x90, r.next_u32());
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() % 0x100) as u64;
            },
        );
    }

    #[test]
    fn ui_pair_refresh_text_matches() {
        check(
            ssx3_ee::s13::f_0039be10__generated,
            ui_pair_refresh_text,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x39be10, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x80, 0);
                let first = if seed % 3 == 0 { 0 } else { OBJ2 };
                let second = if seed % 4 == 0 { 0 } else { OBJ3 };
                w(c, OBJ + 0x78, first);
                w(c, OBJ + 0x7c, second);
                w(c, OBJ2 + 0x8, VT);
                w(c, OBJ3 + 0x8, VT);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ui_pair_free_children_matches() {
        check(
            ee::s13::f_0039be70__generated,
            ui_pair_free_children,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_be70, 2);
                c.g[A0][0] = sx(DATA);
                let a = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
                let b = if seed % 4 == 0 { 0 } else { DATA + 0x300 };
                if a != 0 {
                    fake_slot(c, &mut r, a, 0x8, 0x8);
                }
                if b != 0 {
                    fake_slot(c, &mut r, b, 0x8, 0x8);
                }
                c.bus.write32(DATA + 0x78, a);
                c.bus.write32(DATA + 0x7c, b);
            },
        );
    }

    #[test]
    fn ui_pair_update_anim_matches() {
        check(
            ee::s13::f_0039c1c8__generated,
            ui_pair_update_anim,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x39c1c8, 4);
                be_library_2(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
                if seed % 2 == 0 {
                    w_2(c, this + 0x78, 0);
                } else {
                    // the pair child's vtable is the word after its next links
                    let a = DATA + 0x400;
                    w_2(c, this + 0x78, a);
                    w_2(c, a + 8, DATA + 0x2000);
                    slot_2(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
                if seed % 3 == 0 {
                    w_2(c, this + 0x7c, 0);
                } else {
                    let b = DATA + 0x500;
                    w_2(c, this + 0x7c, b);
                    w_2(c, b + 8, DATA + 0x2000);
                    slot_2(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
            },
        );
    }

    #[test]
    fn uipair_v04_matches() {
        check(
            ee::s13::f_0039bad0__generated,
            uipair_v04,
            SEEDS_3,
            &|c, seed| {
                base(c, seed, 0xbad0, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(s + 0x14, r_u32(seed, 1));
                let a = DATA + 0x200;
                let b = DATA + 0x240;
                c.bus.write32(s + 0x78, a);
                c.bus.write32(s + 0x7c, if seed % 3 == 0 { 0 } else { b });
                let vta = DATA + 0x400;
                slot_3(c, vta, 0x20, 0, FAKE);
                c.bus.write32(a + 8, vta);
                let vtb = DATA + 0x440;
                slot_3(c, vtb, 0x20, 0, FAKE + 0x10);
                c.bus.write32(b + 8, vtb);
            },
        );
    }

    #[test]
    fn c_ui_progress_dtor_matches() {
        check(
            ssx3_ee::s13::f_0039c240__generated,
            c_ui_progress_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c240, 3);
                be_library_3(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 != 0 {
                    fake_child(c, obj, 0x74, 8, 0);
                }
                if seed % 4 != 0 {
                    fake_child(c, obj, 0x78, 8, 1);
                }
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------
    // UI objects.

    #[test]
    fn ui_pair_set_visible_matches() {
        check(
            ssx3_ee::s13::f_0039bb50__generated,
            ui_pair_set_visible,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39bb50, 3);
                be_library_3(c);
                world(c);
                let pair = DATA + 0x100;
                c.g[A0][0] = pair as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w_3(c, pair + 0x14, if seed % 2 == 0 { 0x40 } else { 0 });
                w_3(c, pair + 0x78, 0);
                w_3(c, pair + 0x7c, 0);
                if seed % 3 != 0 {
                    let a = DATA + 0x300;
                    let vt = DATA + 0x500;
                    w_3(c, pair + 0x78, a);
                    w_3(c, a + 8, vt);
                    slot_4(c, vt, 0x48, 0, FAKE);
                    slot_4(c, vt, 0x4c, 0, FAKE + 0x10);
                }
                if seed % 4 != 0 {
                    let b = DATA + 0x400;
                    let vt = DATA + 0x600;
                    w_3(c, pair + 0x7c, b);
                    w_3(c, b + 8, vt);
                    slot_4(c, vt, 0x48, 0, FAKE + 0x20);
                    slot_4(c, vt, 0x4c, 0, FAKE + 0x30);
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_ui_progress_update_anim_matches() {
        check(
            ee::s13::f_0039c398__generated,
            c_ui_progress_update_anim,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x39c3, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x1000;
                slot_5(c, vt, 0x98, EMPTY);
                w32(c, this + 0xc, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                w8(c, this + 0x10, seed % 8);
                w32(
                    c,
                    this + 0x78,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                w32(
                    c,
                    this + 0x74,
                    if seed % 5 == 0 { 0 } else { DATA + 0x3100 },
                );
                w32(c, DATA + 0x3000 + 8, vt);
                w32(c, DATA + 0x3100 + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x1000) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI render / texture manager.

    #[test]
    fn ui_pair_render_matches() {
        check(
            ee::s13::f_0039c130__generated,
            ui_pair_render,
            100,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x39c130, 4);
                let obj = DATA + 0x1000;
                let a = DATA + 0x2000;
                let b = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_4(c, obj + 0x78, if seed % 4 == 0 { 0 } else { a });
                w_4(c, obj + 0x7c, if seed % 3 == 0 { 0 } else { b });
                w_4(c, obj + 0x44, r.edge_f32());
                w_4(c, obj + 0x48, r.edge_f32());
                w_4(c, a + 0x8, vt);
                w_4(c, b + 0x8, vt);
                slot_6(c, vt, 0x88, 8, FAKE);
            },
        );
    }

    #[test]
    fn cui_progress_render_matches() {
        check(
            ee::s13::f_0039c300__generated,
            cui_progress_render,
            100,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x39c300, 4);
                let obj = DATA + 0x1000;
                let a = DATA + 0x2000;
                let b = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_4(c, obj + 0x74, if seed % 4 == 0 { 0 } else { a });
                w_4(c, obj + 0x78, if seed % 3 == 0 { 0 } else { b });
                w_4(c, obj + 0x44, r.edge_f32());
                w_4(c, obj + 0x48, r.edge_f32());
                w_4(c, a + 0x8, vt);
                w_4(c, b + 0x8, vt);
                slot_6(c, vt, 0x88, 8, FAKE);
            },
        );
    }

    #[test]
    fn ui_pair_v03_matches() {
        check(
            ee::s13::f_0039bd68__generated,
            ui_pair_v03,
            10,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x39bd68, 3);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2_2 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                if seed % 2 == 1 {
                    // UIList_First on a zeroed list returns 0: take the child
                    // path instead of the predicate-hit one.
                    w_5(c, vt + 0x10, 0);
                    w_5(c, vt + 0x14, 0x0039_7840);
                }
                w_5(c, OBJ2_2, vt);
                w_5(c, OBJ_2 + 0x78, if seed % 4 == 0 { 0 } else { CHILD });
                w_5(c, OBJ_2 + 0x7c, if seed % 3 == 0 { 0 } else { SELF2 });
                let vt9 = vtable(c, &mut r, &[0x18, 0x1c]);
                w_5(c, CHILD + 8, vt9);
                let vt10 = vtable(c, &mut r, &[0x18, 0x1c]);
                w_5(c, SELF2 + 8, vt10);
            },
        );
    }

    #[test]
    fn ui_menu_render_matches() {
        strict(
            ee::s13::f_0039b6a0__generated,
            ui_menu_render,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39b6, 2);
                let this = DATA;
                c.bus.write32(this + 0x9c, 0);
                let list = this + 0x74;
                let (node0, node1, sent, obj) = (
                    DATA + 0x400u32,
                    DATA + 0x500u32,
                    DATA + 0x600u32,
                    DATA + 0x700u32,
                );
                if seed % 4 == 0 {
                    c.bus.write32(list + 4, list);
                } else {
                    c.bus.write32(list + 4, node0);
                    c.bus.write32(node0 + 4, node1);
                    c.bus.write32(node0 + 0x14, 0x40);
                    c.bus.write32(node0 + 8, obj);
                    c.bus.write32(node1 + 4, sent);
                    c.bus.write32(node1 + 0x14, 0);
                    c.bus.write32(sent + 4, sent);
                    c.bus.write16(obj + 0x88, 0);
                    c.bus.write32(obj + 0x8c, FAKE);
                }
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ui_menu_find_first_selectable_matches() {
        check(
            ee::s13::f_0039b7b0__generated,
            ui_menu_find_first_selectable,
            60,
            &|c, seed| {
                let _r = base_4(c, seed, 0x39b7);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write8(DATA + 0x96, (seed % 5) as u8);
                c.bus.write32(DATA + 0x74, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x90, if seed % 3 == 0 { 4 } else { 0 });
                c.bus.write32(DATA + 0x5c, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 8, DATA + 0x2200);
                c.bus.write16(DATA + 0x2200 + 0xa0, 0);
                c.bus.write32(DATA + 0x2200 + 0xa4, FAKE);
                c.bus.write32(DATA + 0x1000 + 0x14, (seed % 2) as u32);
            },
        );
    }

    pub(crate) const PAIR: u32 = DATA + 0x1_0000;

    pub(crate) const PAIR_A: u32 = DATA + 0x1_1000;

    pub(crate) const PAIR_B: u32 = DATA + 0x1_2000;

    pub(crate) const PAIR_SCREEN: u32 = DATA + 0x1_3000;

    #[test]
    fn ui_pair_set_focus_matches() {
        check_all(
            ee::s13::f_0039bbd8__generated,
            ui_pair_set_focus,
            30,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x39bb_d8, 10);
                c.g[A0][0] = sx(PAIR);
                c.g[A1][0] = sx(seed % 2);
                c.bus.write32(PAIR + 0x14, r.next_u32());
                if seed % 2 == 0 {
                    c.bus.write32(PAIR + 0x78, PAIR_A);
                    let vt1 = vtable(c, &mut r, &[0x38, 0x3c]);
                    c.bus.write32(PAIR_A + 8, vt1);
                }
                if seed % 3 != 0 {
                    c.bus.write32(PAIR + 0x7c, PAIR_B);
                    let vt2 = vtable(c, &mut r, &[0x38, 0x3c]);
                    c.bus.write32(PAIR_B + 8, vt2);
                    if seed % 2 == 0 {
                        c.bus.write32(PAIR_B + 0x5c, PAIR_SCREEN);
                        c.bus.write32(PAIR + 0x5c, PAIR_SCREEN);
                        c.bus.write32(PAIR_SCREEN + 0xd0, PAIR_A);
                        let vt3 = vtable(c, &mut r, &[0x98, 0x9c]);
                        c.bus.write32(PAIR_A + 8, vt3);
                    }
                }
            },
        );
    }
}
