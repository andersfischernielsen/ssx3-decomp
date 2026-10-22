//! `src/fe/debugfe.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use callee::*;
use crate::call::sp;
use crate::call::Flow;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use crate::events::deliver_event;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::wscript::wscriptman::gp;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_4;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_5;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_6;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::ai::rider::INPUT_BINDING_EVAL_BOOL;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_4;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_d008, debug_menu_set_game_mode, "DebugMenu_SetGameMode"),
    (0x0017_d200, lobby_menu_init_message_page, "LobbyMenu_InitMessagePage"),
    (0x0017_d260, lobby_menu_lobby_event_hook, "LobbyMenu_LobbyEventHook"),
    (0x0017_d688, lobby_menu_update, "LobbyMenu_Update"),
    (0x0017_e030, lobby_menu_create_account, "LobbyMenu_CreateAccount"),
    (0x0017_e118, lobby_menu_login, "LobbyMenu_Login"),
    (0x0017_e1c0, lobby_menu_create_persona, "LobbyMenu_CreatePersona"),
    (0x0017_e228, lobby_menu_delete_persona, "LobbyMenu_DeletePersona"),
    (0x0017_e290, lobby_menu_select_persona, "LobbyMenu_SelectPersona"),
    (0x0017_e2f8, lobby_menu_enter_room, "LobbyMenu_EnterRoom"),
    (0x0017_e368, lobby_menu_leave_occupants, "LobbyMenu_LeaveOccupants"),
    (0x0017_e3b8, lobby_menu_show_occupant_detail, "LobbyMenu_ShowOccupantDetail"),
    (0x0017_e418, lobby_menu_send_chat, "LobbyMenu_SendChat"),
    (0x0017_e490, lobby_menu_accept_challenge, "LobbyMenu_AcceptChallenge"),
    (0x0017_e4f0, lobby_menu_decline_challenge, "LobbyMenu_DeclineChallenge"),
    (0x0017_e558, lobby_menu_cancel_challenge, "LobbyMenu_CancelChallenge"),
    (0x0017_f2d0, debug_menu_module_update_cursor_trail, "DebugMenuModule_UpdateCursorTrail"),
    (0x0017_fa60, debug_menu_module_enter, "DebugMenuModule_Enter"),
    (0x0017_fad0, debug_menu_module_shutdown, "DebugMenuModule_Shutdown"),
    (0x0017_fb98, debug_menu_module_v06, "DebugMenuModule_v06"),
    (0x0017_fbf0, debug_menu_module_render, "DebugMenuModule_Render"),
    (0x0018_0300, fe_ui_input_is_any_button, "FEUIInput_IsAnyButton"),
    (0x0018_0478, feui_input_set_port, "FEUIInput_SetPort"),
    (0x0018_0498, feui_input_has_binding, "FEUIInput_HasBinding"),
    (0x0018_05d8, keyboard_screen_set_text, "KeyboardScreen_SetText"),
    (0x0018_0618, keyboard_screen_cursor_left, "KeyboardScreen_CursorLeft"),
    (0x0018_0678, keyboard_screen_cursor_right, "KeyboardScreen_CursorRight"),
    (0x0018_06d8, keyboard_screen_cursor_up, "KeyboardScreen_CursorUp"),
    (0x0018_0768, keyboard_screen_cursor_down, "KeyboardScreen_CursorDown"),
    (0x0018_07c8, keyboard_screen_toggle_caps, "KeyboardScreen_ToggleCaps"),
    (0x0018_09b0, keyboardscreen_get_key_object_name, "KeyboardScreen_GetKeyObjectName"),
    (0x0018_0a90, keyboard_screen_get_cursor_object_name, "KeyboardScreen_GetCursorObjectName"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
}


/// `int KeyboardScreen_ToggleCaps(KeyboardScreen* s)` (0x1807c8): flips the
/// caps-lock bit of the screen's flag word at +0xe4 and returns the new word.
pub fn keyboard_screen_toggle_caps(c: &mut Cpu) {
    c.cycles += 4;
    let flags = Addr(abi::arg(c, 0)).off(0xe4);
    let v = flags.lw(c) ^ 1;
    c.g[V0][0] = v;
    flags.sw(c, v);
    abi::ret(c);
}


/// `void LobbyMenu_LobbyEventHook(void* ev)` (0x17d260): forward the event to
/// the live lobby menu, then to the lobby manager's UI.
pub fn lobby_menu_lobby_event_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x1da0)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_menu = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_menu {
            c.cycles += 2;
            callee::LOBBY_MENU_ON_LOBBY_EVENT.call(c, 0x0017_d280)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_FORWARD_EVENT_TO_UI.call(c, 0x0017_d288)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void KeyboardScreen_SetText(Screen* s, const char* text)` (0x1805d8):
/// copy `text` into the 0x80-byte buffer at `+0xec` and cache its length at
/// `+0x16c`.
pub fn keyboard_screen_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(p(crate::abi::r32(c, S1), 0xec));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::STRCPY.call(c, 0x0018_05f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::STRLEN.call(c, 0x0018_0600)?;
        Addr(p(crate::abi::r32(c, S1), 0x16c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void DebugMenuModule_Enter()` (0x17fa60): release the current game
/// object, clear the debug menu's active flag, then ask the game object's
/// vtable slot 0x78/0x7c for a view-space point and pass it on.
pub fn debug_menu_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0017_fa74)?;
        Addr((c.g[GP][0] as u32).wrapping_add(0x1234)).sw(c, 0);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10d8).lw(c);
        let game = c.g[V0][0] as u32;
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x78).lh(c);
        Addr(c.g[SP][0] as u32).sw(c, 0);
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = sx(game.wrapping_add(c.g[A0][0] as u32));
        Addr((c.g[SP][0] as u32).wrapping_add(8)).sw(c, 0);
        c.g[V0][0] = Addr(vt).off(0x7c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017_faa0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, target, 0x0017_faa0)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Lobby menu.

/// `void LobbyMenu_LeaveOccupants(LobbyMenu* menu)` (0x17e368): destroy every
/// occupant in the list at +0x8, then mark the menu left (state 0xd at +0x4).
/// 8 (+5 per occupant) + 7 cycles plus the callees.
pub fn lobby_menu_leave_occupants(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let mut cur = p(self_, 8);
        let first = c.lw(p(self_, 8));
        c.g[V0][0] = first;
        c.g[S0][0] = sx(cur);
        if first != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(cur);
                call_ptr(c, 0x002c_c460, 0x0017_e390)?;
                cur = c.g[S0][0] as u32;
                c.g[V0][0] = c.lw(cur);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_e388);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0xd;
        frame.restore(c, 16, 0x20);
        c.sw(p(self_, 4), c.g[V0][0]);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void DebugMenuModule_v06(DebugMenuModule* self)` (0x17fb98): poll the
/// debug menu's child module (when no world is up) and update the cursor
/// trail.
pub fn debug_menu_module_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::APP_MODULE_UPDATE_FADE.call(c, 0x0017_fbac)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0017_fbd4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL.call(c, 0x0017_fbdc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void LobbyMenu_ShowOccupantDetail(LobbyMenu* self)` (0x17e3b8): drain the
/// occupant list and open the detail panel for the selected occupant.
pub fn lobby_menu_show_occupant_detail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this).off(0x8).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(this.wrapping_add(0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e3e0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::LOBBY_OCCUPANT_POP, 0x0017_e3e0)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e3d8);
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = 0xe;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x1534));
        Addr(r32(c, S1)).off(0x4).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0017_e400);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x8));
        call_ptr(c, callee::LOBBY_OCCUPANT_SELECT, 0x0017_e400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void KeyboardScreen_CursorDown(KeyboardScreen* self)` (0x180768): move
/// the cursor to the next row, wrapping through the hidden rows.
pub fn keyboard_screen_cursor_down(c: &mut Cpu) {
    c.cycles += 5;
    let a0 = r32(c, A0);
    c.g[V1][0] = Addr(a0).off(0x170).lw(c);
    c.g[A1][0] = sx(r32(c, V1).wrapping_sub(0x1e));
    c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
    let in_normal_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1c;
    if !in_normal_range {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(a0).off(0x170).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(a0).off(0xe0).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(a0).off(0x170).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x27;
                Addr(a0).off(0x170).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xa));
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(a0).off(0xd8).lw(c);
    let no_shift = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x14));
    c.cycles += 2;
    if no_shift {
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
    } else {
        Addr(a0).off(0x170).sw(c, c.g[A1][0]);
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Front-end input (fe/feuiinput.cpp).

/// `void FEUIInput_SetPort(FEUIInput* self, int port)` (0x180478): record the
/// controller port at `self+0x4` and bind the per-port binding object
/// (`world->[port]` at `+0xb0`) at `self+0x18`.
pub fn feui_input_set_port(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[A1][0] &= 0xff;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).lw(c);
    let this = r32(c, A0);
    Addr(this.wrapping_add(0x4)).sb(c, c.g[A1][0]);
    Addr(this.wrapping_add(0x18)).sw(c, c.g[A2][0]);
    abi::ret(c);
}


/// `int FEUIInput_HasBinding(FEUIInput* self)` (0x180498): whether the bound
/// object at `+0x18` (two pointer hops) holds a non-zero action count.
pub fn feui_input_has_binding(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x18)).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Small helpers.

/// `reg + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}


/// The low 32 bits of a register.
#[inline(always)]
pub(crate) fn r32_2(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}


// ---------------------------------------------------------------------------
// Menus.

/// `void LobbyMenu_InitMessagePage(LobbyMenu* menu)` (0x17d200): initialise
/// the message-page child widget and its two labels.
pub fn lobby_menu_init_message_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S0][0] = menu;
        call_ptr(c, 0x002c_aa58, 0x0017_d218)?;
        c.g[S1][0] = sx(at(c, S0, 0x130));
        c.g[V1][0] = 0x46_d740;
        c.g[A1][0] = 0x45_d060;
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c, S0, 0x12c)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x002c_cdf0, 0x0017_d238)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0017_d248)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void LobbyMenu_AcceptChallenge(LobbyMenu* menu)` (0x17e490): drain the
/// challenge queue, then show the accepted-challenge page.
pub fn lobby_menu_accept_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S1][0] = menu;
        c.g[V0][0] = Addr(at(c, S1, 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e4b8)?;
                c.g[V0][0] = Addr(r32_2(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e4b0);
                }
            }
        }
        c.cycles += 11;
        c.g[A1][0] = Addr(at(c, S1, 0x1c0c)).lw(c);
        c.g[V0][0] = 0xd;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A2][0] = 0;
        Addr(at(c, S1, 0x4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fb58, 0x0017_e4dc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void KeyboardScreen_CursorLeft(KeyboardScreen* s)` (0x180618): move the
/// keyboard cursor one key to the left (cursor ids 0,10,20,30,39..).
pub fn keyboard_screen_cursor_left(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 3;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        let at_zero = c.g[V1][0] == 0;
        c.g[V0][0] = 0xa;
        if at_zero {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x14;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1e;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x27;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x25;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void KeyboardScreen_CursorRight(KeyboardScreen* s)` (0x180678): move the
/// keyboard cursor one key to the right.
pub fn keyboard_screen_cursor_right(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 4;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        c.g[V0][0] = 9;
        let mut equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x13;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1d;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = 0x27;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[A1][0];
        c.g[V0][0] = 0x25;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 2;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[A1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `LobbyMenu_Update(menu)` 0x17d688: tick the two lobby sub-systems, update
/// the top of the menu stack when present, and run the menu's state machine.
pub fn lobby_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp_2(c, -0x238)).lw(c);
        call_ptr(c, 0x0025_5a20, 0x0017_d6a0)?;
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        call_ptr(c, 0x0025_ac50, 0x0017_d6a8)?;
        c.g[V0][0] = lw_at(c, this, 8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
            c.g[V0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(p(this, 8));
            call_ptr(c, 0x002c_c0c0, 0x0017_d6d8)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0017_db18, 0x0017_d6e0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `LobbyMenu_CreatePersona(menu, arg)` 0x17e1c0: drain the persona list, then
/// enter state 8 and ask the lobby layer to create the persona.
pub fn lobby_menu_create_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 8, 0x0017_e1f0, 0x0017_e1e8, 0x0017_e210, 0x0025_d1b8)
}


/// `LobbyMenu_DeletePersona(menu, arg)` 0x17e228: drain the list, enter state
/// 9 and ask the lobby layer to delete the persona.
pub fn lobby_menu_delete_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 9, 0x0017_e258, 0x0017_e250, 0x0017_e278, 0x0025_d428)
}


/// `LobbyMenu_SelectPersona(menu, arg)` 0x17e290: drain the list, enter state
/// 7 and ask the lobby layer to select the persona.
pub fn lobby_menu_select_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 7, 0x0017_e2c0, 0x0017_e2b8, 0x0017_e2e0, 0x0025_cd28)
}


/// The shared `list = &self->list; while (*list) pop(list); state = n; ...`
/// body of the three lobby persona hooks.
pub(crate) fn lobby_menu_drain(c: &mut Cpu, state: u32, loop_ret: u32, head: u32, ret: u32, call: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let this = abi::arg64(c, 0);
        let arg = abi::arg64(c, 1);
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        c.g[S2][0] = arg;
        c.g[V0][0] = lw_at(c, this as u32, 8);
        c.g[S0][0] = sx(p(this as u32, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, loop_ret)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, head);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = sx(state);
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        Addr(p(this as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, call, ret)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `LobbyMenu_DeclineChallenge(menu)` 0x17e4f0: drain the list, clear the
/// pending challenge, enter state 12 and send the decline message.
pub fn lobby_menu_decline_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        c.g[V0][0] = lw_at(c, this, 8);
        c.g[S0][0] = sx(p(this, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e518)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e510);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = sx(p(this, 8));
        c.g[A1][0] = sx(p(this, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e530)?;
        c.g[A1][0] = lw_at(c, this, 0x1c0c);
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fc28, 0x0017_e544)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Lobby menu.

/// `LobbyMenu::EnterRoom(self, room)` (0x17e2f8): pop every page off the page
/// stack, move to room `room` and set state 0xb.
pub fn lobby_menu_enter_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e328)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e320);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = 0xb;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_d800, 0x0017_e34c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `LobbyMenu::CancelChallenge(self)` (0x17e558): pop every page off the page
/// stack, push the challenge list, set state 0xc and cancel the challenge.
pub fn lobby_menu_cancel_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e580)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e578);
                }
            }
        }
        c.cycles += 14;
        c.g[A0][0] = sx(p(c.g[S1][0] as u32, 8));
        c.g[A1][0] = sx(p(c.g[S1][0] as u32, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e598)?;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_f628, 0x0017_e5b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end menus and screens.

/// `LobbyMenu_SendChat` (0x17E418): pop every queued page, push the lobby's
/// page back and hand the chat text to the network layer.
pub fn lobby_menu_send_chat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = lw_at_2(c, S1, 0x8);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = low(c, S1).wrapping_add(0x8);
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, addr::MENU_POP_PAGE, 0x0017_e448)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e440);
                }
            }
        }
        c.cycles += 14;
        c.g[A0][0] = low(c, S1).wrapping_add(0x8);
        c.g[A1][0] = low(c, S1).wrapping_add(0x1044);
        call_ptr(c, addr::MENU_PUSH_PAGE, 0x0017_e460)?;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        sw_at(c, S1, 0x4, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, addr::FE_NET_SEND, 0x0017_e474)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void KeyboardScreen_CursorUp(KeyboardScreen* self)` (0x1806d8): move the
/// keyboard cursor up one row (skipping the page-jump cases), wrapping to the
/// last row.
pub fn keyboard_screen_cursor_up(c: &mut Cpu) {
    c.cycles += 4;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(p(self_, 0x170)).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
    let in_first_row_band = c.g[V0][0] == 0;
    c.g[V0][0] = 8;
    if in_first_row_band {
        // Row 10..19.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0xa));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xd8)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = 0x12;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            let wrap = c.g[V0][0] == 0;
            c.g[V0][0] = 0x27;
            if wrap {
                // Straight to the common store: row 39.
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    } else {
        // Row 0..9.
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x27;
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1e));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    }
}


/// `void DebugMenu_SetGameMode(self, index)` (0x17d008): store the selected
/// mode into the debug slot table and apply it to the race interface.
pub fn debug_menu_set_game_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp_3(c, -0x848)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = lw(c, V0, 0x78);
        call_ptr(c, table::BE_GET_INTERFACE, 0x0017_d034)?;
        c.g[A0][0] = 0x0053_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0053_5bc8;
        c.g[A1][0] = 0x8c38;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x49).lb(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x48).lb(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, table::BE_NEW_RACE_SET_GAME_MODE, 0x0017_d068)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        call_ptr(c, table::BE_NEW_RACE_SET_EVENT_TYPE, 0x0017_d07c)?;
        c.g[V1][0] = lw(c, S1, 0xc);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
        c.g[V0][0] = lw(c, V1, 0xc);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0017_d090)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void LobbyMenu_Login(menu, a1, a2)` (0x17e118): pop the menu's pages,
/// copy the two account strings into the interface and log in.
pub fn lobby_menu_login(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 12;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(at_3(c.g[S1][0], 8));
        c.g[V0][0] = lw_2(c, at_3(c.g[S1][0], 8));
        let t = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        if !t {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e150);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e150)?;
                c.g[V0][0] = lw_2(c, c.g[S0][0] as u32);
                let t = c.g[V0][0] != 0;
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0017_e148);
                }
            }
        }
        c.cycles += 25;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0017_e168);
        c.g[A0][0] = sx(at_3(c.g[S1][0], 0x1c04));
        call_ptr(c, 0x0031_84e8, 0x0017_e168)?;
        c.g[A0][0] = sx(at_3(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e174);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e174)?;
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        c.g[V0][0] = 5;
        c.g[A1][0] = lw_2(c, at_3(c.g[S1][0], 0x1c04));
        c.g[A0][0] = sx(at_3(c.g[A0][0], 0x48));
        c.g[RA][0] = sx(0x0017_e18c);
        Addr(at_3(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_84e8, 0x0017_e18c)?;
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        c.g[A1][0] = lw_2(c, at_3(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e19c);
        c.g[A0][0] = sx(at_3(c.g[A0][0], 0x4c));
        call_ptr(c, 0x0031_84e8, 0x0017_e19c)?;
        c.g[RA][0] = sx(0x0017_e1a4);
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        call_ptr(c, 0x0025_c610, 0x0017_e1a4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `DebugMenuModule_Shutdown(this)` (0x17fad0): release the two debug
/// objects, clear the world's two debug counters and the font handle, and
/// stop the two debug sounds.
pub fn debug_menu_module_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = sx(this.wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;

        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            let absent = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !absent {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0017_fb10)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_faf0);
            }
        }

        c.cycles += 6;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(c.g[V0][0] as u32).off(0x80).sw(c, 0);
        Addr(c.g[V0][0] as u32).off(0x7c).sw(c, 0);

        c.g[A1][0] = Addr(this).off(0x20).lw(c);
        let no_sound20 = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound20 {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb48)?;
        }

        c.cycles += 3;
        c.g[A1][0] = Addr(this).off(0x1c).lw(c);
        let no_sound1c = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound1c {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb68)?;
        }

        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        let no_font = c.g[A0][0] == 0;
        c.lq(16, frame.at(c, 0x30));
        if !no_font {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_16c0, 0x0017_fb7c)?;
            c.lq(16, frame.at(c, 0x30));
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end screens.

/// `void KeyboardScreen_GetCursorObjectName(int key, char *out, char *dst)`
/// (0x180a90): copy an eight-byte prefix and append the character (or the
/// named function key) for `key` into `out`.
pub fn keyboard_screen_get_cursor_object_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_180b54: {
            'b_180ae4: {
                'b_180acc: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A0][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1368));
                    c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, (c.g[T1][0] as u32));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(6, (c.g[A0][0] as u32));
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_180acc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 36i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x37));
                        if t {
                            break 'b_180ae4;
                        } else {
                            break 'b_180acc;
                        }
                    }
                }
                c.cycles += 6;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), 0u64);
                c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                c.g[RA][0] = sx(0x00180adc);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x004162d0, 0x00180adc)?;
                let t = true;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_180b54;
                } else {
                    break 'b_180ae4;
                }
            }
            'b_180b04: {
                c.cycles += 3;
                c.g[V0][0] = 0x24;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x25;
                if t {
                    break 'b_180b04;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00180afc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1350));
                    call_ptr(c, 0x004162d0, 0x00180afc)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b04;
                    }
                }
            }
            'b_180b20: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x26;
                if t {
                    break 'b_180b20;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x460000;
                    c.g[RA][0] = sx(0x00180b18);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2aa0));
                    call_ptr(c, 0x004162d0, 0x00180b18)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b20;
                    }
                }
            }
            'b_180b3c: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x27;
                if t {
                    break 'b_180b3c;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00180b34);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1358));
                    call_ptr(c, 0x004162d0, 0x00180b34)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b3c;
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_180b54;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x4a0000;
                c.g[RA][0] = sx(0x00180b50);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1360));
                call_ptr(c, 0x004162d0, 0x00180b50)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_180b54;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}


/// `void KeyboardScreen_GetKeyObjectName(char* dst, int key, char* out)`
/// (0x1809b0): copy the key's object name (or its printable character) into
/// `out`.
pub fn keyboardscreen_get_key_object_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A2][0];
        c.sd(p(r32(c, SP), 0x10), c.g[RA][0]);
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
        c.g[T1][0] = sx(r32(c, V0).wrapping_sub(0x2ab0));
        c.ldl(6, p(r32(c, T1), 7));
        c.ldr(6, r32(c, T1));
        c.g[A3][0] = Addr(p(r32(c, T1), 8)).lb(c);
        c.g[T0][0] = Addr(p(r32(c, T1), 9)).lb(c);
        c.sdl(6, p(r32(c, A0), 7));
        c.sdr(6, r32(c, A0));
        Addr(p(r32(c, A0), 8)).sb(c, c.g[A3][0]);
        Addr(p(r32(c, A0), 9)).sb(c, c.g[T0][0]);
        let digit = c.g[V1][0] != 0;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x30));
        if !digit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 36i64) as u64;
            let alphabet = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x37));
            if !alphabet {
                self::ks_special_chain(c)?;
                return Ok(());
            }
        }
        c.cycles += 6;
        Addr(p(r32(c, SP), 1)).sb(c, 0);
        Addr(r32(c, SP)).sb(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0041_62d0, 0x0018_0a0c)?;
        ks_finish(c);
        Ok(())
    })
}


/// The `@ : * #` branch of [`keyboardscreen_get_key_object_name`].
pub(crate) fn ks_special_chain(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0x24;
    let not_at = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x25;
    if !not_at {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1350));
        call_ptr(c, 0x0041_62d0, 0x0018_0a2c)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_colon = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x26;
    if !not_colon {
        c.cycles += 5;
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2aa0));
        call_ptr(c, 0x0041_62d0, 0x0018_0a48)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_star = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x27;
    if !not_star {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1358));
        call_ptr(c, 0x0041_62d0, 0x0018_0a64)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_hash = c.g[A1][0] != c.g[V0][0];
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    if not_hash {
        ks_close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 0x4a0000;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1360));
    call_ptr(c, 0x0041_62d0, 0x0018_0a80)?;
    ks_finish(c);
    Ok(())
}


/// The shared return of [`keyboardscreen_get_key_object_name`]: `ra` is
/// already restored, add the trailing cycles and leave.
pub(crate) fn ks_close(c: &mut Cpu) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}


/// `ra = ld(sp+0x10); cycles += 2; sp += 0x20; return`.
pub(crate) fn ks_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    ks_close(c);
}


// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMenu_CreateAccount(void* self, char* user, char* pass)`
/// (0x17e030): drain the menu's page list, store the new account name and
/// password into the lobby's strings and fill the pending account request
/// (user/pass/pass2/host, port 0x79e), then hand it to the lobby manager.
pub fn lobby_menu_create_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(p(r(c, S1), 0x8));
        c.g[S3][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r(c, S1), 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);

        'drain: {
            if empty {
                break 'drain;
            }
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e068)?;
                c.g[V0][0] = Addr(r(c, S0)).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break 'drain;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e060);
                }
            }
        }

        c.cycles += 41;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c04));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e080)?;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c08));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e08c)?;

        c.g[S0][0] = Addr(gp_5(c, -0xc8)).lw(c);
        c.g[V0][0] = 0x4;
        c.sw(p(r(c, S1), 0x4), c.g[V0][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[S1][0] = sx(p(r(c, S0), 0x260));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0a8)?;
        c.g[A0][0] = sx(p(r(c, S0), 0x264));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0b4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(r(c, S0), 0x268));
        call_ptr(c, 0x0031_84e8, 0x0017_e0c0)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(p(r(c, S0), 0x26c));
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x2db8));
        call_ptr(c, 0x0031_84e8, 0x0017_e0d0)?;

        c.g[A0][0] = Addr(gp_5(c, -0xc8)).lw(c);
        c.g[V1][0] = 1;
        c.g[V0][0] = 0x79e;
        c.sw(p(r(c, S0), 0x29c), 0);
        c.sw(p(r(c, S0), 0x298), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(r(c, S0), 0x28c), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x290), c.g[V0][0]);
        c.sw(p(r(c, S0), 0x294), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x288), c.g[V1][0]);
        call_ptr(c, 0x0025_b9c8, 0x0017_e0fc)?;

        // The tail of 0x17e030 runs no extra block charge.
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `LobbyMenu_ctor(self)` (0x0017d298): construct the lobby menu screen and
/// its seven sub-panels, then bind the session and race-manager pointers.
pub fn lobby_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x0046_d350);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let this = c.g[S0][0];
        sw_2(c, this, 0x4, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(8));
        sw_2(c, this, 0, c.g[V0][0]);
        call_ptr(c, 0x002c_c018, 0x0017_d2c0)?;

        // `cUIWidget_ctor(widget, parent, ...)` pairs on the stack.
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        sw_2(c, c.g[SP][0], 0, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x70));
        sw_2(c, c.g[SP][0], 4, this);
        call_ptr(c, 0x001d_a988, 0x0017_d2d8)?;

        c.g[A0][0] = sx((this as u32).wrapping_add(0x604));
        Callee {
            addr: 0x0017_d200,
            f: ee::s02::LobbyMenu_InitMessagePage,
        }
        .call(c, 0x0017_d2e0)?;

        c.g[A1][0] = c.g[SP][0] | 8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        sw_2(c, c.g[SP][0], 8, this);
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        sw_2(c, c.g[SP][0], 0xc, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x74c));
        sw_2(c, c.g[SP][0], 0x10, this);
        call_ptr(c, 0x001d_acb8, 0x0017_d300)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        sw_2(c, c.g[SP][0], 0x14, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xd2c));
        call_ptr(c, 0x001d_ae20, 0x0017_d310)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        sw_2(c, c.g[SP][0], 0x18, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1044));
        sw_2(c, c.g[SP][0], 0x1c, this);
        call_ptr(c, 0x001d_af08, 0x0017_d328)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        sw_2(c, c.g[SP][0], 0x20, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1534));
        call_ptr(c, 0x001d_b028, 0x0017_d338)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
        sw_2(c, c.g[SP][0], 0x24, this);
        sw_2(c, c.g[SP][0], 0x28, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x18c0));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x001d_b148, 0x0017_d350)?;

        sw_2(c, c.g[SP][0], 0x2c, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1aac));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
        call_ptr(c, 0x001d_b2b0, 0x0017_d360)?;

        // Session / race-manager bindings.
        c.g[A0][0] = Addr(gp_6(c, 0xda0)).lw(c);
        c.g[V0][0] = this;
        c.g[A1][0] = Addr(gp_6(c, -0x848)).lw(c);
        sw_2(c, this, 0x1c0c, c.g[A0][0]);
        sw_2(c, this, 0x1c04, c.g[A0][0]);
        sw_2(c, this, 0x1c08, c.g[A0][0]);
        frame.restore_ra(c, 0x30);
        c.g[V1][0] = lw_3(c, c.g[A1][0], 0x80);
        c.g[A0][0] = lw_3(c, c.g[V1][0], 0x18);
        sw_2(c, this, 0x60, c.g[A0][0]);
        c.g[V1][0] = lw_3(c, c.g[A1][0], 0xb0);
        sw_2(c, this, 0x64, c.g[V1][0]);
        c.g[A0][0] = lw_3(c, c.g[A1][0], 0x80);
        c.g[V1][0] = lw_3(c, c.g[A0][0], 0x20);
        sw_2(c, this, 0x68, c.g[V1][0]);
        Addr(gp_6(c, 0x1da0)).sw(c, this);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// The bindings `FEUIInput_IsAnyButton` tests, in the original's order.
pub(crate) const ANY_BUTTON_CODES: [u64; 16] = [
    0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7d, 0x7c, 0x90, 0x92, 0x91, 0x71,
];


/// `int FEUIInput_IsAnyButton(FEUIInput *input)` (0x180300): non-zero when any
/// of the sixteen menu buttons is held.
pub fn fe_ui_input_is_any_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;

        let mut last_held = false;
        for (i, code) in ANY_BUTTON_CODES.iter().enumerate() {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[A1][0] = *code;
            call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0018_0324 + 0x14 * i as u32)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = 1;
                last_held = i + 1 == ANY_BUTTON_CODES.len();
                break;
            }
        }
        // A held button costs one extra cycle; the last one costs two (it
        // falls through the tail check as well).
        if last_held {
            c.cycles += 2;
        } else if c.g[S1][0] != 0 {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S1][0];

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int func_0017FBF0(void* self)` (0x17fbf0): render one debug-menu frame.
/// When the module render hook (render-settings slot 17) accepts, reset the
/// cursor trail, set up the 2D view, call the module's own draw and render
/// fade, then let the menu at_4 `self+0x14` draw itself.
pub fn debug_menu_module_render(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_17fdd4: {
        c.cycles += 14;
        c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, at_4(c, SP, 0x48));
        c.swc1(20, at_4(c, SP, 0x40));
        c.g[V1][0] = lw_4(c, at_4(c, A1, 0x10d8));
        c.g[A0][0] = lh(c, at_4(c, V1, 0x88));
        c.g[V0][0] = lw_4(c, at_4(c, V1, 0x8c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017fc20);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x0017fc20).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
            break 'b_17fdd4;
        } else {
            'b_17fdbc: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0017fc30);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x00231d60, 0x0017fc30).is_err() {
                    return;
                }
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                } else {
                    c.cycles += 97;
                    c.g[V0][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x10d8));
                    c.g[A0][0] = lh(c, at_4(c, V1, 0x78));
                    sw_3(c, at_4(c, SP, 0x14), 0u64);
                    c.swc1(21, at_4(c, SP, 0x10));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.lwc1(20, at_4(c, SP, 0x14));
                    sw_3(c, at_4(c, SP, 0x18), 0u64);
                    sw_3(c, at_4(c, SP, 0x1c), 0u64);
                    c.swc1(20, r32(c, SP));
                    c.swc1(20, at_4(c, SP, 0x4));
                    c.swc1(20, at_4(c, SP, 0x8));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x7c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017fc7c);
                    c.g[A1][0] = c.g[SP][0];
                    if call_ptr(c, t, 0x0017fc7c).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.mov_s(12, 20);
                    c.g[AT][0] = 0x44200000;
                    c.mtc1(1, 14);
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = lw_4(c, at_4(c, A3, 0x10d8));
                    c.mov_s(17, 21);
                    c.g[AT][0] = 0x43f00000;
                    c.mtc1(1, 15);
                    c.mov_s(13, 12);
                    c.g[A0][0] = lh(c, at_4(c, V0, 0xd0));
                    c.g[A1][0] = 0u64;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 16);
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0xd4));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fcc0);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fcc0).is_err() {
                        return;
                    }
                    c.g[A2][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0x500000;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xe60));
                    c.g[V0][0] = lw_4(c, at_4(c, A2, 0x10d8));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x110));
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x114));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fce0);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fce0).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[V1][0] = 0xfffffffffe7f0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[T1][0] = 0x1000000;
                    c.g[A0][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[A1][0] = 0xffffffffffcf0000;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[T2][0] = 0x300000;
                    c.g[V0][0] = lw_4(c, at_4(c, A0, 0x4));
                    c.g[A2][0] = 0xfffffffffff00000;
                    c.g[A2][0] = c.g[A2][0] | 0xfff;
                    c.g[T3][0] = 0xffffffffffffff83;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[T0][0] = 0xffffffffffbf0000;
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    c.g[T0][0] = c.g[T0][0] | 0xffff;
                    sw_3(c, at_4(c, A0, 0x4), c.g[V0][0]);
                    c.g[T1][0] = 0x400000;
                    c.g[A0][0] = sx(at_4(c, S0, 0x24));
                    c.g[V1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                    sw_3(c, at_4(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                    sw_3(c, at_4(c, A1, 0x4), c.g[V0][0]);
                    c.g[V1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    sw_3(c, at_4(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    sw_3(c, at_4(c, A1, 0x4), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0017fd84);
                    c.g[A1][0] = lw_4(c, at_4(c, S0, 0x1c));
                    if callee::C0017f500.call(c, 0x0017fd84).is_err() {
                        return;
                    }
                    c.g[V1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0xfffffffffffffc1f;
                    c.g[A0][0] = lw_4(c, at_4(c, V1, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A0, 0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x120;
                    sw_3(c, at_4(c, A0, 0x8), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, S0, 0x14));
                    c.g[V0][0] = lw_4(c, r32(c, A1));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x18));
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fdb8);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fdb8).is_err() {
                        return;
                    }
                    c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                }
            }
            c.cycles += 6;
            c.g[V0][0] = lw_4(c, at_4(c, A1, 0x10d8));
            c.g[A0][0] = lh(c, at_4(c, V0, 0xa0));
            c.g[V1][0] = lw_4(c, at_4(c, V0, 0xa4));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0017fdd0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0017fdd0).is_err() {
                return;
            }
            c.g[V0][0] = 0x1;
            break 'b_17fdd4;
        }
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    c.g[RA][0] = ld(c, at_4(c, SP, 0x20));
    c.lwc1(21, at_4(c, SP, 0x48));
    c.lwc1(20, at_4(c, SP, 0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x50));
    c.pc = t;
    return;
}


/// `void func_0017F2D0(void* p)` (0x17f2d0): advance the debug menu's firefly
/// cursor trail twice per frame: step the phase, wrap both the phase and the
/// position into `[0,1)`, advance the 64-entry ring at_4 `head` and store the
/// screen-space point.
pub fn debug_menu_module_update_cursor_trail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    c.cycles += 26;
    c.swc1(28, at_4(c, SP, 0x90));
    c.swc1(27, at_4(c, SP, 0x88));
    c.swc1(26, at_4(c, SP, 0x80));
    c.swc1(25, at_4(c, SP, 0x78));
    c.swc1(24, at_4(c, SP, 0x70));
    c.swc1(23, at_4(c, SP, 0x68));
    c.swc1(22, at_4(c, SP, 0x60));
    c.swc1(21, at_4(c, SP, 0x58));
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.g[S1][0] = 0x1;
    c.swc1(20, at_4(c, SP, 0x50));
    c.lwc1(28, r32(c, GP).wrapping_sub(0x5a54));
    c.lwc1(27, r32(c, GP).wrapping_sub(0x5a50));
    c.lwc1(26, r32(c, GP).wrapping_sub(0x5a4c));
    c.lwc1(25, r32(c, GP).wrapping_sub(0x5a48));
    c.lwc1(23, r32(c, GP).wrapping_sub(0x5a44));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 22);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 21);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 24);
    'l_17f338: loop {
        'b_17f3b8: {
            c.cycles += 31;
            c.g[RA][0] = sx(0x0017f340);
            c.lwc1(12, at_4(c, S0, 0x8));
            if call_ptr(c, 0x0031bf60, 0x0017f340).is_err() {
                return;
            }
            c.lwc1(12, at_4(c, S0, 0xc));
            c.g[RA][0] = sx(0x0017f34c);
            c.mov_s(20, 0);
            if call_ptr(c, 0x0031bf60, 0x0017f34c).is_err() {
                return;
            }
            c.lwc1(1, at_4(c, S0, 0x8));
            c.add_s(20, 20, 28);
            c.add_s(0, 0, 28);
            c.lwc1(4, at_4(c, S0, 0xc));
            c.add_s(6, 1, 26);
            c.lwc1(2, r32(c, S0));
            c.mul_s(20, 20, 27);
            c.lwc1(3, at_4(c, S0, 0x4));
            c.mul_s(0, 0, 27);
            c.swc1(26, at_4(c, SP, 0x10));
            c.mul_s(1, 6, 23);
            c.swc1(25, at_4(c, SP, 0x14));
            c.add_s(2, 2, 20);
            c.swc1(20, r32(c, SP));
            c.add_s(3, 3, 0);
            c.swc1(0, at_4(c, SP, 0x4));
            c.add_s(1, 1, 22);
            c.swc1(6, at_4(c, S0, 0x8));
            c.add_s(4, 4, 25);
            c.cvt_w_s(5, 1);
            c.cvt_s_w(5, 5);
            c.swc1(2, r32(c, S0));
            c.c_lt_s(1, 5);
            c.swc1(3, at_4(c, S0, 0x4));
            let t = !c.fcc();
            c.swc1(4, at_4(c, S0, 0xc));
            if t {
                break 'b_17f3b8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3b8;
            }
        }
        'b_17f3e8: {
            c.cycles += 11;
            c.lwc1(7, r32(c, GP).wrapping_sub(0x5a40));
            c.mul_s(0, 4, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.sub_s(1, 6, 1);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at_4(c, S0, 0x8));
            if t {
                break 'b_17f3e8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3e8;
            }
        }
        'b_17f414: {
            c.cycles += 10;
            c.mul_s(0, 2, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(1, 4, 1);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at_4(c, S0, 0xc));
            if t {
                break 'b_17f414;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f414;
            }
        }
        'b_17f440: {
            c.cycles += 10;
            c.mul_s(0, 3, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(12, 2, 1);
            c.cvt_w_s(4, 0);
            c.cvt_s_w(4, 4);
            c.c_lt_s(0, 4);
            let t = !c.fcc();
            c.swc1(12, r32(c, S0));
            if t {
                break 'b_17f440;
            } else {
                c.cycles += 1;
                c.sub_s(4, 4, 21);
                break 'b_17f440;
            }
        }
        c.cycles += 35;
        c.g[V0][0] = lw_4(c, at_4(c, S0, 0x210));
        c.mul_s(0, 4, 7);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
        c.g[A0][0] = sx(at_4(c, V0, 0x1));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(at_4(c, V0, 0x40));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 6) as u32);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.swc1(0, at_4(c, S0, 0x4));
        c.g[RA][0] = sx(0x0017f47c);
        sw_3(c, at_4(c, S0, 0x210), c.g[A0][0]);
        if call_ptr(c, 0x0031bf60, 0x0017f47c).is_err() {
            return;
        }
        c.mul_s(20, 0, 24);
        c.lwc1(12, at_4(c, S0, 0x4));
        c.lwc1(0, r32(c, GP).wrapping_sub(0x5a3c));
        c.g[RA][0] = sx(0x0017f490);
        c.add_s(20, 20, 0);
        if call_ptr(c, 0x0031bf60, 0x0017f490).is_err() {
            return;
        }
        c.mul_s(0, 0, 24);
        c.g[AT][0] = 0x43d20000;
        c.mtc1(1, 1);
        c.g[V0][0] = lw_4(c, at_4(c, S0, 0x210));
        c.swc1(20, at_4(c, SP, 0x10));
        c.add_s(0, 0, 1);
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.swc1(0, at_4(c, SP, 0x14));
        ldl(c, 3, at_4(c, SP, 0x10));
        sdl(c, 3, at_4(c, V0, 0x10));
        let t = (c.g[S1][0] as i64) >= 0;
        c.g[RA][0] = ld(c, at_4(c, SP, 0x20));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0017f338);
            }
            continue 'l_17f338;
        } else {
            c.cycles += 13;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            c.lwc1(28, at_4(c, SP, 0x90));
            c.lwc1(27, at_4(c, SP, 0x88));
            c.lwc1(26, at_4(c, SP, 0x80));
            c.lwc1(25, at_4(c, SP, 0x78));
            c.lwc1(24, at_4(c, SP, 0x70));
            c.lwc1(23, at_4(c, SP, 0x68));
            c.lwc1(22, at_4(c, SP, 0x60));
            c.lwc1(21, at_4(c, SP, 0x58));
            c.lwc1(20, at_4(c, SP, 0x50));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at_4(c, SP, 0xa0));
            c.pc = t;
            return;
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    use crate::difftest::seed_count;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::compare;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::base as base_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::bx::bxstring2::tests::W;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng as rng_2;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_4;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_6;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_5;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_3;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_5;
    pub(crate) use crate::game::fe::festaterewards::tests::W as W_4;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_7;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::be::beintstat::tests::call;
    pub(crate) use crate::game::be::beintstat::tests::pool_allocator as pool_allocator_4;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::camera::camera::tests::W_2 as W_5;
    pub(crate) use crate::game::camera::camera::tests::RM as RM_2;
    pub(crate) use crate::game::camera::camera::tests::BE_5 as BE_6;
    pub(crate) use crate::game::camera::camera::tests::run as run_3;
    pub(crate) use crate::game::camera::camera::tests::heap_init as heap_init_4;
    pub(crate) use crate::game::camera::camera::tests::pool_allocator_3 as pool_allocator_5;
    pub(crate) use crate::game::camera::camera::tests::alloc_base;
    pub(crate) use crate::game::main::debugmenu::tests::w as w_8;
    pub(crate) use crate::game::ai::rider::tests::input_map;
    pub(crate) use crate::game::ai::rider::tests::binding;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_10;


    #[test]
    fn keyboard_screen_toggle_caps_matches() {
        check(ee::s02::f_001807c8__generated, keyboard_screen_toggle_caps, SEEDS, &|c, seed| {
            let mut r = rng(0x0018_07c8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let flags = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0xe4, flags);
        });
    }


    #[test]
    fn lobby_menu_lobby_event_hook_matches() {
        check(
            ee::s02::f_0017d260__generated,
            lobby_menu_lobby_event_hook,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17d2_60, 0);
                let ev = DATA + 0x600;
                fill(c, &mut r, ev, 0x20);
                c.bus.write32(ev, 0);
                c.g[A1][0] = sx(ev);
                let menu = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(GAME_GP + 0x1da0, menu);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }


    /// A NUL-terminated ASCII string at `addr`.
    pub(crate) fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }


    #[test]
    fn keyboard_screen_set_text_matches() {
        check(
            ssx3_ee::s02::f_001805d8__generated,
            keyboard_screen_set_text,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x10, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x5000);
                put_str(
                    c,
                    DATA + 0x5000,
                    ["A", "hello", "a longer text value"][seed as usize % 3],
                );
            },
        );
    }


    // -----------------------------------------------------------------------
    // 0x17fa60 DebugMenuModule_Enter.

    #[test]
    fn debug_menu_module_enter_matches() {
        check(ee::s02::f_0017fa60__generated, debug_menu_module_enter, 8, &|c, seed| {
            let _r = machine(c, seed, 0x17fa_60, 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x854), 0);
        });
    }


    // ---------------------------------------------------------- lobby menu

    #[test]
    fn lobby_menu_leave_occupants_matches() {
        check(ee::s02::f_0017e368__generated, lobby_menu_leave_occupants, 60, &|c, seed| {
            let mut r = base(c, seed, 0x17e3);
            let menu = DATA;
            let n = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
            c.bus.write32(menu + 8, n);
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x30, 0x38]);
            for i in 0..3u32 {
                let item = DATA + 0x1000 + 0x200 * i;
                c.bus.write32(menu + 0xc + 4 * i, if i < n { item } else { 0 });
                c.bus.write32(item + 0x12c, vt);
            }
            c.g[A0][0] = sx(menu);
        });
    }


    #[test]
    fn debug_menu_module_v06_matches() {
        check(
            ee::s02::f_0017fb98__generated,
            debug_menu_module_v06,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fb_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let world = DATA + 0x2000;
                let obj = DATA + 0x800;
                let vt = vtable(c, &mut r, &[0x10]);
                clear(c, this, 0x100);
                clear(c, obj, 0x100);
                c.bus.write32(this + 0x14, obj);
                c.bus.write32(obj, vt);
                dummy_world(c, world);
                if seed % 2 == 0 {
                    c.bus.write32(world + 4, 0);
                } else {
                    c.bus.write32(world + 4, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn lobby_menu_show_occupant_detail_matches() {
        check(ee::s02::f_0017e3b8__generated, lobby_menu_show_occupant_detail, SEEDS_2, &|c, seed| {
            let _ = common(c, seed, 0x17e3b8, 2);
            let this = DATA + 0x100;
            clear(c, this, 0x1600 / 4);
            c.bus.write32(this + 8, (seed % 3) as u32);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }


    #[test]
    fn keyboard_screen_cursor_down_matches() {
        check(ee::s02::f_00180768__generated, keyboard_screen_cursor_down, SEEDS_2, &|c, seed| {
            let _ = base_2(c, seed, 0x180768);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x170, (seed % 48) as u32);
            c.bus.write32(this + 0xd8, ((seed / 48) % 2) as u32);
            c.bus.write32(this + 0xe0, ((seed / 96) % 2) as u32);
            c.g[A0][0] = sx(this);
        });
    }


    // ------------------------------------------------------------------
    // Front-end input.

    #[test]
    fn feui_input_set_port_matches() {
        check(ee::s02::f_00180478__generated, feui_input_set_port, 200, &|c, seed| {
            let mut r = setup(c, seed, 9);
            c.g[A0][0] = AUX as u64;
            let port = seed % 5;
            c.g[A1][0] = 0xabcd_ff00u64 | port as u64;
            put(c, W + 0xb0 + 4 * port, r.next_u32());
            gpf(c, -0x848, W);
        });
    }


    #[test]
    fn feui_input_has_binding_matches() {
        check(ee::s02::f_00180498__generated, feui_input_has_binding, 200, &|c, seed| {
            let mut r = setup(c, seed, 10);
            c.g[A0][0] = AUX as u64;
            match seed % 4 {
                0 => put(c, AUX + 0x18, 0),
                n => {
                    put(c, AUX + 0x18, AUX + 0x40);
                    put(c, AUX + 0x40, AUX + 0x80);
                    put(c, AUX + 0x80, if n == 1 { 0 } else { r.next_u32() });
                }
            }
        });
    }


    /// The object most fixtures live at.
    pub(crate) const OBJ: u32 = DATA + 0x1000;

    pub(crate) const OBJ2: u32 = DATA + 0x2000;

    pub(crate) const OBJ3: u32 = DATA + 0x3000;


    pub(crate) fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }


    /// The engine's size-class pool allocator: blocks up to 0x400 bytes come
    /// from per-size free lists hanging off `gp + 0x2a60..0x2a84` when the
    /// pools-ready flag at `gp + 0xdd8` is set.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 0); // (re-written below as 1)
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),   // 8-byte blocks
            (0x2a60, 0x2000),  // 16
            (0x2a6c, 0x32000), // 32
            (0x2a7c, 0x25800), // 64
            (0x2a84, 0x25800), // 128
            (0x2a78, 0x20000), // 256
            (0x2a68, 0x20000), // 512
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            w(c, GAME_GP + slot_off, pool);
        }
    }


    // ------------------------------------------------------------- keyboard

    #[test]
    fn keyboard_screen_cursor_left_matches() {
        check(
            ssx3_ee::s02::f_00180618__generated,
            keyboard_screen_cursor_left,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180618, 0);
                let cursor = [0u32, 10, 20, 30, 39, 0x25, 0x27, 5][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn keyboard_screen_cursor_right_matches() {
        check(
            ssx3_ee::s02::f_00180678__generated,
            keyboard_screen_cursor_right,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180678, 0);
                let cursor = [9u32, 19, 29, 39, 0x25, 0x27, 5, 0x1d][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    // ------------------------------------------------------- menus / audio

    #[test]
    fn lobby_menu_init_message_page_matches() {
        check(
            ssx3_ee::s02::f_0017d200__generated,
            lobby_menu_init_message_page,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17d200, 0);
                c.bus.write32(OBJ, seed);
                w(c, OBJ + 0x130, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn lobby_menu_accept_challenge_matches() {
        check(
            ssx3_ee::s02::f_0017e490__generated,
            lobby_menu_accept_challenge,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e490, 0);
                pool_allocator(c);
                let node = OBJ2;
                w(c, node, 0);
                w(c, node + 0x4, OBJ3);
                w(c, OBJ + 0x8, if seed % 2 == 0 { 0 } else { node });
                w(c, OBJ + 0x1c0c, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn lobby_menu_update_matches() {
        check(ee::s02::f_0017d688__generated, lobby_menu_update, 20, &|c, seed| {
            fakes::machine(c, seed, 0x17d688, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            w(c, OBJ + 8, (seed % 3) as u32);
            w(c, OBJ + 0xc, if seed % 2 == 0 { 0 } else { 1 });
            w(c, OBJ + 4, 0x19);
        });
    }


    pub(crate) fn lobby_persona_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            // Draining the list and `LobbyMan_SelectPersona` build strings and
            // tag fields, so the engine allocator has to be usable.
            pool_allocator_2(c);
            heap_init(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { (seed & 7) as u64 };
            if seed % 2 == 0 {
                w(c, OBJ + 8, 0);
            } else {
                w(c, OBJ + 8, 1);
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 0x12c, VT);
                slot(c, VT, 0x38, 0, fakes::FAKE);
            }
        });
    }


    #[test]
    fn lobby_menu_create_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e1c0__generated, lobby_menu_create_persona, 0x17e1c0);
    }


    #[test]
    fn lobby_menu_delete_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e228__generated, lobby_menu_delete_persona, 0x17e228);
    }


    #[test]
    fn lobby_menu_select_persona_matches() {
        lobby_persona_test(ee::s02::f_0017e290__generated, lobby_menu_select_persona, 0x17e290);
    }


    #[test]
    fn lobby_menu_decline_challenge_matches() {
        check(
            ee::s02::f_0017e4f0__generated,
            lobby_menu_decline_challenge,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e4f0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                if seed % 2 == 0 {
                    w(c, OBJ + 8, 0);
                } else {
                    w(c, OBJ + 8, 1);
                    w(c, OBJ + 0xc, OBJ2);
                    w(c, OBJ2 + 0x12c, VT);
                    slot(c, VT, 0x38, 0, fakes::FAKE);
                }
                w(c, OBJ + 0x1044, 0);
                w(c, OBJ + 0x1c0c, 0);
            },
        );
    }


    /// Run one generated function as a setup step, with `args` in a0..t1.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA_2 as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA_2, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// The main heap's dlmalloc arena over 256 KiB of test RAM (oc69).
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) const SENTINEL_RA_2: u32 = 0x0012_3450;


    pub(crate) fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }


    // ------------------------------------------------------------ lobby menu

    #[test]
    fn lobby_menu_enter_room_matches() {
        lobby_menu_test(
            ee::s02::f_0017e2f8__generated,
            lobby_menu_enter_room,
            0x0017_e2f8,
            true,
        );
    }


    #[test]
    fn lobby_menu_cancel_challenge_matches() {
        lobby_menu_test(
            ee::s02::f_0017e558__generated,
            lobby_menu_cancel_challenge,
            0x0017_e558,
            false,
        );
    }


    pub(crate) fn lobby_menu_test(gen: GuestFn, hand: GuestFn, salt: u64, enter: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base_3(c, seed, salt, 4);
            c.g[A0][0] = OBJ_2 as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            let count = seed % 3;
            w_2(c, OBJ_2 + 8, count);
            let vt = vtable(c, &mut r, &[0x38]);
            for i in 0..count {
                let item = DATA + 0x3000 + 0x100 * i;
                w_2(c, OBJ_2 + 0xc + 4 * i, item);
                w_2(c, item + 0x12c, vt);
            }
            if !enter {
                let vt2 = vtable(c, &mut r, &[0x30]);
                w_2(c, OBJ_2 + 0x1044 + 0x12c, vt2);
            }
        });
    }


    #[test]
    fn lobby_menu_send_chat_matches() {
        check(ee::s02::f_0017e418__generated, lobby_menu_send_chat, SEEDS_3, &|c, seed| {
            machine(c, seed, 0x17_e418, 4);
            belib(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64;
            c.pc = 0x0017_e418;
        });
    }


    #[test]
    fn keyboard_screen_cursor_up_matches() {
        check(ee::s02::f_001806d8__generated, keyboard_screen_cursor_up, 200, &|c, seed| {
            let _ = base_4(c, seed, 0x18_06d8);
            globals_3(c);
            c.bus.write32(DATA, 0);
            let row = match seed % 8 {
                0 => 0,
                1 => 9,
                2 => 10,
                3 => 19,
                4 => 18,
                5 => 8,
                6 => 39,
                _ => (seed * 7) % 45,
            };
            w_4(c, DATA + 0x170, row);
            w_4(c, DATA + 0xd8, if seed % 2 == 0 { 0 } else { 0x2222 });
            w_4(c, DATA + 0xe0, if seed % 3 == 0 { 0 } else { 0x3333 });
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn debug_menu_set_game_mode_matches() {
        check(ssx3_ee::s02::f_0017d008__generated, debug_menu_set_game_mode, 8, &|c, seed| {
            let mut r = base_5(c, seed, 0x17d008, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed % 4) as u64;
            let world = DATA + 0x8_0000;
            w(c, world + 0x78, DATA + 0x2000);
            c.bus.write8(0x0053_5bc8 + 0x48, (seed % 4) as u8);
            c.bus.write8(0x0053_5bc8 + 0x49, (seed % 3) as u8);
            for i in 0..4u32 {
                w(c, 0x004a_6ca8 + 4 * i, r.next_u32() % 4);
            }
            let iface = DATA + 0x3000;
            w(c, DATA + 0x30_1000 + 4, iface);
            w(c, iface + 0xc, DATA + 0x3100);
            w(c, DATA + 0x3100 + 8, 0);
            w(c, DATA + 0x3100 + 0xc, 0);
        });
    }

    /// Fewer seeds for tests whose real callees are heavy.
    pub(crate) const FEW: u32 = 16;


    // ---------------------------------------------------------- lobby login

    #[test]
    fn lobby_menu_login_matches() {
        check(ee::s02::f_0017e118__generated, lobby_menu_login, FEW, &|c, seed| {
            let _ = base_6(c, seed, 0x17e1_18, 8);
            let obj = SCRATCH;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (obj + 0x1000) as u64;
            c.g[A2][0] = (obj + 0x1100) as u64;
            w_5(c, obj + 8, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
            w_5(c, obj + 0x2000, 0);
            w_5(c, obj + 0x1c04, 0);
            w_5(c, obj + 0x1c08, 0);
            wb(c, obj + 0x1000, b'a');
            wb(c, obj + 0x1100, b'b');
        });
    }


    #[test]
    fn debug_menu_module_shutdown_matches() {
        check(
            ee::s02::f_0017fad0__generated,
            debug_menu_module_shutdown,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fa, 4);
                game_globals(c);
                c.bus.write32(GAME_GP - 0x854, DATA + 0x16_0000);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let o1 = DATA + 0x2000;
                let o2 = DATA + 0x2400;
                c.bus.write32(this + 0xc, o1);
                c.bus.write32(this + 0x10, o2);
                let vt1 = vtable(c, &mut r, &[8]);
                let vt2 = vtable(c, &mut r, &[8]);
                c.bus.write32(o1, vt1);
                c.bus.write32(o2, vt2);
                c.bus.write32(
                    this + 0x20,
                    if seed % 2 == 0 { 0xff_ffff } else { seed % 4 },
                );
                c.bus.write32(
                    this + 0x1c,
                    if seed % 3 == 0 {
                        0xff_ffff
                    } else {
                        1 + seed % 4
                    },
                );
                c.bus
                    .write32(this + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2800 });
                let vt = vtable(c, &mut r, &[0x190]);
                let m = DATA + 0x16_0000;
                c.bus.write32(m + 0x10d8, vt);
            },
        );
    }


    #[test]
    fn keyboard_screen_get_cursor_object_name_matches() {
        strict(
            ee::s02::f_00180a90__generated,
            keyboard_screen_get_cursor_object_name,
            100,
            &|c, seed| {
                let _r = base_7(c, seed, 0x180a);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 0x50) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                for i in 0..0x20 {
                    c.bus.write8(DATA + 0x2000 + i, 0);
                }
            },
        );
    }


    #[test]
    fn keyboardscreen_get_key_object_name_matches() {
        check(
            ee::s02::f_001809b0__generated,
            keyboardscreen_get_key_object_name,
            16,
            &|c, seed| {
                let _ = base_8(c, seed, 0x1809b0, 2);
                let dst = DATA;
                let out = DATA + 0x100;
                clear_2(c, dst, 0x100);
                clear_2(c, out, 0x100);
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 5,
                    2 => 10,
                    3 => 0x24,
                    4 => 0x25,
                    _ => (seed % 64) as u64,
                };
                c.g[A2][0] = out as u64;
            },
        );
    }


    // --------------------------------------------------------------- lobby

    #[test]
    fn lobby_menu_create_account_matches() {
        strict_2(
            ee::s02::f_0017e030__generated,
            lobby_menu_create_account,
            60,
            &|c, seed| {
                let _ = base_9(c, seed, 0x17e0, 4);
                allocator(c);
                let menu = DATA;
                let g = DATA + 0x3000;
                let buf = DATA + 0x6000;
                c.g[A0][0] = menu as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = (DATA + 0x940) as u64;
                for (i, b) in b"user5\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                for (i, b) in b"passw0rd\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x940 + i as u32, *b);
                }
                // Pre-sized BXString buffers (capacity and length large
                // enough) so the assigns take the no-allocation path.
                w_7(c, buf.wrapping_sub(0xc), 0x1000);
                w_7(c, buf.wrapping_sub(4), 0x1000);
                w_7(c, menu + 0x1c04, buf);
                w_7(c, menu + 0x1c08, buf);
                w_7(c, GAME_GP.wrapping_sub(0xc8), g);
                for off in [0x260u32, 0x264, 0x268, 0x26c] {
                    w_7(c, g + off, buf);
                }
                // The page list: empty, or one page that MenuSys_PopPage drains.
                if seed % 3 == 0 {
                    let page = DATA + 0x400;
                    let vt = DATA + 0x1000;
                    w_7(c, menu + 8, 1);
                    w_7(c, menu + 0xc, page);
                    w_7(c, page + 0x12c, vt);
                    w_7(c, vt + 0x38, 0);
                    w_7(c, vt + 0x3c, FAKE);
                }
            },
        );
    }


    #[test]
    fn lobby_menu_ctor_matches() {
        check_all(ee::s02::f_0017d298, lobby_menu_ctor, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x17d2);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x10_1000;
            c.bus.write32(W_5 + 0x80, obj);
            c.bus.write32(obj + 0x18, seed);
            c.bus.write32(obj + 0x20, seed * 3 + 1);
            c.bus.write32(W_5 + 0xb0, seed ^ 0x55);
            c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
        });
    }


    #[test]
    fn fe_ui_input_is_any_button_matches() {
        check(
            ee::s02::f_00180300__generated,
            fe_ui_input_is_any_button,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18_0300, 16);
                c.g[A0][0] = DATA as u64;
                let b = DATA + 0x2000;
                binding(c, b, DATA + 0x3000, 0);
                w_8(c, DATA + 0x18, b);
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                w_8(c, SP - 0x208, value.to_bits());
            },
        );
    }


    pub(crate) const DBG: u32 = DATA + 0x1_0000;

    pub(crate) const DBG_MENU: u32 = DATA + 0x1_1000;


    #[test]
    fn debug_menu_module_render_matches() {
        check_all_2(
            ee::s02::f_0017fbf0__generated,
            debug_menu_module_render,
            24,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x17fb_f0, 10);
                c.g[A0][0] = sx(DBG);
                c.bus.write32(DBG + 0x14, DBG_MENU);
                c.bus.write32(DBG + 0x1c, seed % 4);
                c.bus.write32(DBG + 0x24, r.next_u32() % 0x40);
                let vt4 = vtable(c, &mut r, &[0x18, 0x1c]);
                c.bus.write32(DBG_MENU, vt4);
                // The render-settings vtable at_4 `gp-0x854 + 0x10d8` (slot 17/15/26/
                // 34/20) stays empty, so those calls go to the fakes.
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x88, 0x8c]);
                        c.bus.write32(AUX_2 + 0x10d8, vt);
                }
            },
        );
    }


    pub(crate) const FLY: u32 = DATA + 0x1_0000;


    #[test]
    fn debug_menu_module_update_cursor_trail_matches() {
        check_all_2(
            ee::s02::f_0017f2d0__generated,
            debug_menu_module_update_cursor_trail,
            24,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x17f2_d0, 8);
                c.g[A0][0] = sx(FLY);
                c.bus.write32(FLY + 0x0, r.edge_f32()); // phase.x
                c.bus.write32(FLY + 0x4, r.edge_f32()); // phase.y
                c.bus.write32(FLY + 0x8, r.edge_f32()); // pos.x
                c.bus.write32(FLY + 0xc, r.edge_f32()); // pos.y
                c.bus.write32(FLY + 0x10, (seed % 64) as u32); // head
            },
        );
    }
}
