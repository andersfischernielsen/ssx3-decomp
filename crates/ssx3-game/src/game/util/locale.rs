//! `src/util/locale.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::be::be::gp;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_2;
pub(crate) use crate::game::fe::fepopup3::lb;
pub(crate) use crate::game::fe::fepopup3::p_2;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::ui::uiengine::lb_r;
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
    (0x0018_6518, c_fe_state_on_activate, "cFEState_OnActivate"),
    (
        0x0019_5948,
        fe_locale_unload_all_tables,
        "FELocale_UnloadAllTables",
    ),
    (0x0019_5a50, fe_locale_unload_table, "FELocale_UnloadTable"),
    (
        0x0019_5b70,
        fe_locale_load_common_table,
        "FELocale_LoadCommonTable",
    ),
    (
        0x0019_5d50,
        fe_locale_unload_fe_tables,
        "FELocale_UnloadFETables",
    ),
    (
        0x0019_5e58,
        fe_locale_unload_game_tables,
        "FELocale_UnloadGameTables",
    ),
    (0x0019_5eb8, fe_locale_v04, "FELocale_v04"),
    (0x0019_5f70, fe_locale_v05, "FELocale_v05"),
    (
        0x0019_6148,
        c_fe_state_audio_options_init,
        "cFEStateAudioOptions_Init",
    ),
    (
        0x0019_6228,
        cfestateaudiooptions_shutdown,
        "cFEStateAudioOptions_Shutdown",
    ),
    (
        0x0019_62a8,
        cfe_state_audio_options_transition_in,
        "cFEStateAudioOptions_TransitionIn",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2;
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
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_V29;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_ASYNC_MANAGER_GET_STATE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::GAME_MODULE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::HUD_LAYOUT_ICON_TEXT;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_REQUEST_CARD_INFO_2 as MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::fe::fepopup3::callee::MESSAGE_CENTER_GET_INBOX_3 as MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY as CBX_STRING_CTOR_COPY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR as CBX_STRING_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE as FE_LOCALE_UNLOAD_TABLE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE as WORLD_OBJ_REPLACE_WITH_DEAD_NODE_2;
}

/// `void FELocale_UnloadTable(FELocale* self, int index)` (0x195a50): drop
/// the locale string table entry at `index`, if it is loaded.
pub fn fe_locale_unload_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        frame.save_ra(c, 0);
        // `sll a1, 24; sra a1, 22`: the byte index times 4.
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw(r32(c, S0));
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 3;
            call_ptr(c, callee::LOC_STRING_TABLE_DTOR, 0x0019_5a80)?;
            sw(c, r32(c, S0), 0, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FELocale_UnloadAllTables(FELocale* self)` (0x195948): free all 20 locale
/// tables (indexes 0..18; the counter starts at `0x1000000`, the loop shifts
/// it down by 24 bits and stops at 20). Returns `self`.
pub fn fe_locale_unload_all_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x0100_0000;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x0100_0000;
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5978)?;
            c.g[V0][0] = c.g[S0][0];
            c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 20i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_5970);
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

/// `FELocale_UnloadGameTables(FELocale* self)` (0x195e58): unless the locale
/// has the "game tables loaded" flag (byte `+3` bit 0), free tables 0xc..0xf.
pub fn fe_locale_unload_game_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lb(c, p_2(c.g[S0][0], 3));
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 12;
            c.g[A1][0] = 0xc;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e80)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xd;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e8c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xe;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e98)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xf;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5ea4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FELocale_LoadCommonTable(FELocale* self)` (0x195b70): map the
/// locale byte at +1 to a language index and load that table.
pub fn fe_locale_load_common_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        c.g[V1][0] = Addr(at(c, A0, 1)).lb(c);
        let byte = c.g[V1][0];
        c.g[A1][0] = u64::MAX;
        let mut lang = -1i64;
        if byte == 1 {
            // 0x195bbc falls into the tail that selects language 1.
            c.cycles += 2;
            lang = 1;
        } else if (byte as i64) < 2 {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if byte == 0 {
                lang = 0;
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[V0][0] = 3;
            if byte == 2 {
                c.cycles += 1;
                lang = 2;
            } else {
                c.cycles += 2;
                if byte == 3 {
                    lang = 3;
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.g[A1][0] = sx(lang as u32);
        c.cycles += 5;
        call_ptr(c, 0x0019_59a8, 0x0019_5bd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00195DE0` (0x195de0): `FELocale_LoadGameTables`. Picks the table id
/// `{0: 0xc, 1: 0xd, 2: 0xe, 3: 0xf, else -1}` from the language byte at
/// `this+1`, marks the locale (`+0x58 = 2`) and loads that table (`0x1959a8`).
/// Signature: `void load_game_tables(FELocale* this)`.
pub fn fe_locale_load_game_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = 0x2;
        c.sd(low(c, SP), c.g[RA][0]);
        sb_r(c, A0, 0x58, c.g[V1][0]);
        c.g[V0][0] = 0x1;
        c.g[A2][0] = lb_r(c, A0, 0x1);
        let one = c.g[A2][0] == c.g[V0][0];
        c.g[A1][0] = u64::MAX;
        if one {
            c.cycles += 2;
            c.g[A1][0] = 0xd;
        } else {
            c.cycles += 3;
            let below = (c.g[A2][0] as i64) < 2;
            c.g[V0][0] = 0x2;
            if below {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[A1][0] = 0xc;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let two = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                if two {
                    c.cycles += 1;
                    c.g[A1][0] = 0xe;
                } else {
                    c.cycles += 2;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[A1][0] = 0xf;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0019_5e48);
        callee::FE_LOCALE_LOAD_TABLE.call(c, 0x00195e48)?;
        c.g[RA][0] = c.ld(low(c, SP));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `int FELocale_v05(FELocale* loc, LocStringTable* table)` (0x00195f70): walk
/// the locale's table of string ids, resolving each until one is an entry in
/// the string table (a locale row at +8 per id), accumulating the table's
/// base offset; returns 0 when nothing resolves.
pub fn fe_locale_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.sq(S0, sp(c).wrapping_add(0x40));
        c.sq(S1, sp(c).wrapping_add(0x30));
        c.sq(S2, sp(c).wrapping_add(0x20));
        c.sq(S3, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        c.g[S1][0] = 0x0100_0000;
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = 0x0100_0000;
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[S1][0];
            let found = if c.g[A0][0] == 0 {
                false
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0019_5fac);
                call_ptr(c, 0x002c_6dc0, 0x0019_5fac)?; // LocStringTable_ResolveString
                if c.g[V0][0] != 0 {
                    c.lq(S0, sp(c).wrapping_add(0x40));
                    true
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                    false
                }
            };
            if found {
                break;
            }
            c.cycles += 5;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 20) as u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S3][0] as u32));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_5f98);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.lq(S0, sp(c).wrapping_add(0x40));
            break;
        }
        c.cycles += 6;
        c.lq(S1, sp(c).wrapping_add(0x30));
        c.lq(S2, sp(c).wrapping_add(0x20));
        c.lq(S3, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateAudioOptions_Shutdown(self)` (0x1962a0 area, 0x196228):
/// find the state's audio-options UI state and shut the screen down.
pub fn cfestateaudiooptions_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = lw_at(c, gw(c, -0x848));
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, p_3(c.g[V1][0] as u32, 0x84));
        let no_race = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_race {
            c.cycles += 11;
            c.g[S0][0] = lw_at(c, p_3(r32(c, S1), 0x10));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::STR_HASH, 0x0019_625c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_STATE_MGR_FIND_STATE.addr, 0x0019_6268)?;
            c.g[A3][0] = c.g[V0][0];
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = 4;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p_3(c.g[A3][0] as u32, 8));
                c.g[A2][0] = 0;
                let func = vslot(c, c.g[V1][0], 0xc0, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0019_628c)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN_2.addr, 0x0019_6294)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Locale.

/// `void FELocale_UnloadFETables(FELocale* self)` (0x195d50): drop the eight
/// front-end tables unless the locale is already detached.
pub fn fe_locale_unload_fe_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_4(r32(c, S0), 3)).lb(c);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 24;
            for (i, id) in [4u32, 5, 6, 7, 0x10, 0x11, 0x12, 0x13]
                .into_iter()
                .enumerate()
            {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(id);
                callee::FE_LOCALE_UNLOAD_TABLE_2.call(c, 0x0019_5d78 + 0xc * i as u32)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEState_OnActivate(cFEState* self, int a1)` (0x186518): select the
/// credits menu entry, then run the base activation.
pub fn c_fe_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p_4(r32(c, S1), 0x24));
        callee::UI_LIST_FIRST_2.call(c, 0x0018_653c)?;
        c.g[S0][0] = c.g[V0][0];
        let empty = c.g[S0][0] == 0;
        c.g[A0][0] = 0x004a_0000;
        if empty {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_6550)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_655c)?;
            c.g[S0][0] = c.g[V0][0];
            let found = c.g[S0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if found {
                c.cycles += 6;
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[A1][0] = Addr(p_4(r32(c, S1), 0xc)).lb(c);
                call_ptr(c, 0x001a_06f0, 0x0018_6574)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0] & 0xff;
                call_ptr(c, 0x0039_b960, 0x0018_6580)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e4c0, 0x0018_658c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FELocale_LoadTable(self, int lang)` (0x1959a8): lazily create the
/// locale file object for a language index.
pub fn fe_locale_load_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
        frame.save(c, S2, 0x10);
        let negative = (c.g[S0][0] as i64) < 0;
        frame.save_ra(c, 0);
        let mut one = !negative;
        if !negative {
            c.cycles += 6;
            c.g[V1][0] = sx(p_5(r32(c, A0), 8));
            c.g[S2][0] = sx(r32(c, S0) << 2);
            c.g[S1][0] = sx(p_5(r32(c, V1), r32(c, S2)));
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 21;
                c.g[A1][0] = sx(0x0046_03f0);
                c.g[A0][0] = 0x14;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001959f8)?;
                c.g[V1][0] = sx(0x0044_0000);
                c.g[A0][0] = sx(0x0044_0000);
                Addr(r32(c, S1)).sw(c, c.g[V0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1138));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1188));
                c.g[V1][0] = sx(p_5(r32(c, S2), r32(c, V1)));
                c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
                c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                c.g[A2][0] = Addr(r32(c, A0)).lb(c);
                c.g[A3][0] = sx(0xffff_ffff);
                c.g[A0][0] = c.g[V0][0];
                Addr(r32(c, V0)).sb(c, c.g[A3][0]);
                Addr(p_5(r32(c, V0), 4)).sw(c, 0);
                Addr(p_5(r32(c, V0), 0xc)).sw(c, 0);
                call_ptr(c, 0x002c_6e98, 0x00195a34)?;
            } else {
                one = false;
            }
        }
        if one {
            c.cycles += 1;
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

// ---------------------------------------------------------------------------
// FE locale.

/// `char* FELocale_v04(FELocale* l, int lang)` (0x195eb8): return the locale
/// string for `lang`, looking it up in the table at `+8` (the first byte of
/// the table entry must match `l->kind`) or falling back to the default
/// translation of the key at `+0x5a`.
pub fn fe_locale_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        'outer: {
            'mid: {
                frame.save(c, S1, 0x40);
                frame.save(c, S3, 0x20);
                let this = r(c, A0);
                c.g[S1][0] = sx(this);
                frame.save(c, S0, 0x50);
                frame.save(c, S2, 0x30);
                frame.save(c, S4, 0x10);
                c.g[S3][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = Addr(this).lb(c);
                let cached = c.g[V0][0] != 0;
                if cached {
                    break 'mid;
                }
                c.cycles += 3;
                c.g[S2][0] = 0x0100_0000;
                c.g[S0][0] = sx(this.wrapping_add(8));
                c.g[S4][0] = 0x0100_0000;
                loop {
                    'entry: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r(c, S0)).lw(c);
                        let empty = c.g[A0][0] == 0;
                        c.g[V0][0] = c.g[S2][0];
                        if empty {
                            break 'entry;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r(c, A0)).lb(c);
                        c.g[V0][0] = Addr(p_6(r(c, S1), 1)).lb(c);
                        let differs = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.g[S2][0];
                        if differs {
                            break 'entry;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0019_5f14);
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x002c_6de0, 0x0019_5f14)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, 16, 0x50);
                            break 'outer;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S2][0];
                        break 'entry;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                    c.g[V0][0] = sx((r(c, V0) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 20) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
                    if !more {
                        break 'mid;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_5ef0);
                    }
                }
            }
            c.cycles += 7;
            c.g[S0][0] = sx(p_6(r(c, S1), 0x5a));
            c.g[A1][0] = sx(0x004a_1390);
            c.g[RA][0] = sx(0x0019_5f48);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_27c0, 0x0019_5f48)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateAudioOptions_TransitionIn(cFEStateAudioOptions *self, int a1)`
/// (0x1962a8): refresh the option labels on the way in.
pub fn cfe_state_audio_options_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_196364: {
            'b_196360: {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[A1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_196360;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_196364;
                    } else {
                        c.cycles += 31;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        vslot(c, c.g[V0][0], 0x48, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001962e4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001962e4)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        vslot(c, c.g[V0][0], 0x48, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00196300);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00196300)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        vslot(c, c.g[V0][0], 0x48, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0019631c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019631c)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        vslot(c, c.g[V0][0], 0x48, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00196338);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00196338)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            break 'b_196364;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            vslot(c, c.g[V0][0], 0x48, V1);
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196360);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00196360)?;
                            break 'b_196360;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_196364;
        }
        c.cycles += 4;
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateAudioOptions_Init (0x0000196148).

/// `cFEStateAudioOptions_Init`: initialise the audio options screen: build its sliders and read the current mix levels.
pub fn c_fe_state_audio_options_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1961dc: {
            'b_1961d8: {
                'b_1961c0: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_1961c0;
                    } else {
                        c.cycles += 13;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = 0x460000;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x400));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
                        call_ptr(c, 0x00317670, 0x00196188)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039f9d8, 0x00196194)?;
                        c.g[A3][0] = c.g[V0][0];
                        let t = c.g[A3][0] == 0u64;
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_1961d8;
                        } else {
                            c.cycles += 8;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[RA][0] = sx(0x001961b8);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x001961b8 {
                                return Err(crate::call::Unwound);
                            }
                            let t = true;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_1961dc;
                            } else {
                                break 'b_1961c0;
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = 0x460000;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x410));
                call_ptr(c, 0x0028b180, 0x001961cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0028f140, 0x001961d8)?;
                break 'b_1961d8;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            break 'b_1961dc;
        }
        'b_196210: {
            c.cycles += 9;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001961e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001961f8)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_196210;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00196210)?;
                break 'b_196210;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_4 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festateoptions2::cfestaterequestline_shutdown;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::slot_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn fe_locale_unload_table_matches() {
        check(
            ssx3_ee::s02::f_00195a50__generated,
            fe_locale_unload_table,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x195a50, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let entry = DATA + 0x2000;
                let idx: u64 = [0, 1, 2, 7, 0x3c][seed as usize % 5];
                let slot = this_ + 8 + 4 * idx as u32;
                c.bus.write32(slot, if seed % 3 == 0 { 0 } else { entry });
                // A null nested pointer: the table's destructor only frees itself.
                c.bus.write32(entry + 4, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = idx;
            },
        );
    }

    /// Allocate `size` bytes and zero them.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        let p_2 = run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32;
        for i in (0..size).step_by(4) {
            c.bus.write32(p_2 + i, 0);
        }
        p_2
    }

    #[test]
    fn fe_locale_unload_all_tables_matches() {
        check(
            ssx3_ee::s02::f_00195948__generated,
            fe_locale_unload_all_tables,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x195948);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                // A couple of the 20 tables exist, so the frees run.
                for k in 0..3u32 {
                    let slot = ((seed + k * 7) % 20) as u32;
                    let block = alloc(c, 0x20);
                    c.bus.write32(DATA + 8 + 4 * slot, block);
                }
            },
        );
    }

    #[test]
    fn fe_locale_unload_game_tables_matches() {
        check(
            ssx3_ee::s02::f_00195e58__generated,
            fe_locale_unload_game_tables,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x195e58);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write8(DATA + 3, (seed % 2) as u8);
                for k in 0..2u32 {
                    let block = alloc(c, 0x20);
                    c.bus.write32(DATA + 8 + 4 * (0xc + (seed + k) % 4), block);
                }
            },
        );
    }

    #[test]
    fn fe_locale_load_common_table_matches() {
        check(
            ee::s02::f_00195b70__generated,
            fe_locale_load_common_table,
            40,
            &|c, seed| {
                base(c, seed, 0x195b_70);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA + 1, (seed % 6) as u8);
                for lang in 0..4u32 {
                    c.bus.write32(DATA + 8 + lang * 4, 1);
                }
            },
        );
    }

    #[test]
    fn fe_locale_v05_matches() {
        check(
            ee::s02::f_00195f70__generated,
            fe_locale_v05,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x195f, 3);
                // `locale+8` is a table of string ids; each id's `+4` is the
                // LocString body. With a zeroed body the resolve returns 0, so the
                // walk runs its full 20 rounds; one seed makes the first entry
                // resolve (a count word above the table pointer) for the early exit.
                let found = seed % 3 == 1;
                w(c, DATA + 0x804, DATA + 0x900);
                w(c, DATA + 0x900 + 0xc, if found { 0xffff_ffff } else { 0 });
                for i in 0..20u32 {
                    w(c, DATA + 8 + 4 * i, if i == 0 { DATA + 0x800 } else { 0 });
                }
                let _ = &mut r;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn cfestateaudiooptions_shutdown_matches() {
        check(
            ee::s02::f_00196228__generated,
            cfestateaudiooptions_shutdown,
            SEEDS,
            &|c, seed| {
                shutdown_fixture(c, seed, 0x6228);
            },
        );
        check(
            ee::s02::f_00196e80__generated,
            cfestaterequestline_shutdown,
            SEEDS,
            &|c, seed| {
                shutdown_fixture(c, seed, 0x6e80);
            },
        );
    }

    pub(crate) fn shutdown_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        base_3(c, seed, salt, 0);
        let s1 = DATA + 0x100;
        let obj = DATA + 0x200;
        c.g[A0][0] = sx(s1);
        c.bus.write32(s1 + 0x10, obj);
        if seed % 4 == 1 {
            c.bus.write32(DATA + 0x9000 + 0x84, 0);
        }
        let node = DATA + 0x300;
        c.bus.write32(obj + 0x18 + 4, 0);
        let list2 = obj + 0x18 + 0x1c;
        if seed % 3 != 0 {
            c.bus.write32(list2 + 4, node);
            c.bus.write32(node, 0);
            c.bus.write32(node + 4, node + 8);
            c.bus.write32(node + 0xc, 0);
            let vt = DATA + 0x340;
            slot_2(c, vt, 0xc0, 0, FAKE);
            c.bus.write32(node + 8, vt);
        }
        c.bus.write32(s1 + 0x24 + 4, 0);
        c.bus.write32(0x004a_18a8, 0);
    }

    #[test]
    fn fe_locale_unload_fe_tables_matches() {
        check(
            ee::s02::f_00195d50__generated,
            fe_locale_unload_fe_tables,
            60,
            &|c, seed| {
                let _ = base_4(c, seed, 0x19_5d50);
                globals(c);
                c.bus.write8(DATA + 3, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_state_on_activate_matches() {
        check(
            ee::s02::f_00186518__generated,
            c_fe_state_on_activate,
            100,
            &|c, seed| {
                let _ = base_4(c, seed, 0x18_6518);
                globals(c);
                const LIST: u32 = DATA + 0x2000;
                // List: head at +4; a sentinel-only list on some seeds.
                if seed % 3 == 0 {
                    w(c, LIST + 4, LIST);
                } else {
                    w(c, DATA + 0x1000 + 0x3c, 1);
                    w(c, DATA + 0x1000 + 0x38, DATA + 0x1100);
                    w(c, DATA + 0x1100, DATA + 0x1200);
                    w(c, DATA + 0x1200 + 0x38, 0x1234);
                    w(c, LIST + 4, DATA + 0x1000);
                }
                w(c, DATA + 0x24, LIST);
                w(c, DATA + 0xc, seed & 0x7f);
                w(c, GAME_GP.wrapping_add(0x1d90), 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seed);
            },
        );
    }

    #[test]
    fn fe_locale_v04_matches() {
        check(
            ee::s02::f_00195eb8__generated,
            fe_locale_v04,
            40,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x195e, 0);
                let this = DATA;
                wb(c, this, (seed % 4 == 0) as u8);
                wb(c, this + 1, 0x41);
                let lang = r.next_u32();
                for i in 0..20u32 {
                    let entry = DATA + 0x400 + 4 * i;
                    w(c, this + 8 + 4 * i, entry);
                    if i == 0 && seed % 3 == 1 {
                        wb(c, entry, 0x41);
                    } else {
                        w(c, entry, 0);
                    }
                }
                w(c, this + 0x5a, 0);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = lang as u64;
            },
        );
    }

    #[test]
    fn cfe_state_audio_options_transition_in_matches() {
        strict(
            ee::s02::f_001962a8__generated,
            cfe_state_audio_options_transition_in,
            60,
            &|c, seed| {
                let _r = base_6(c, seed, 0x1962);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x7c, 0);
                c.bus.write32(DATA + 0x74, (seed % 3 == 0) as u32);
                for (i, field) in [0x68u32, 0x6c, 0x50, 0x58, 0x64].into_iter().enumerate() {
                    let obj = DATA + 0x1000 + 0x100 * i as u32;
                    c.bus.write32(DATA + field, obj);
                    c.bus.write32(obj + 8, obj + 0x80);
                    c.bus.write16(obj + 0x80 + 0x48, 0);
                    c.bus.write32(obj + 0x80 + 0x4c, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_audio_options_init_matches() {
        diff(
            ee::s02::f_00196148__generated,
            c_fe_state_audio_options_init,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x19_6148);
                c.g[A0][0] = sx(DATA);
                let game = WORLD + 0x1000;
                w(c, game + 0x28, DATA + 0x2000);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, DATA + 0x2004, r.next_u32());
                w(c, DATA + 0x80, r.next_u32());
            },
        );
    }
}
