//! `src/be/beintnetwork.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_4 as gp;
pub(crate) use crate::game::be::bedbreward::destroy_instance;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE;
pub(crate) use crate::game::be::bedbreward::POST_RACE_GP;
pub(crate) use crate::game::be::bedbreward::PRE_RACE_GP;
pub(crate) use crate::game::be::bedbreward::RACE_GP;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_2;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
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
    (
        0x0014_e0e0,
        be_network_int_get_instance,
        "BENetworkInt_GetInstance",
    ),
    (0x0014_e130, be_network_int_commit, "BENetworkInt_Commit"),
    (0x0014_e1f8, be_network_int_reset, "BENetworkInt_Reset"),
    (0x0014_e9c0, be_copy_profile, "BE_CopyProfile"),
    (
        0x0014_e9f8,
        be_prepare_net_profiles,
        "BE_PrepareNetProfiles",
    ),
    (
        0x0014_ea90,
        be_reset_net_interfaces,
        "BE_ResetNetInterfaces",
    ),
    (
        0x0014_eb08,
        beoption_int_swap_player_flags,
        "BEOptionInt_SwapPlayerFlags",
    ),
    (
        0x0014_ebb8,
        be_set_net_string_534fc8,
        "BE_SetNetString534fc8",
    ),
    (
        0x0014_ebd8,
        be_set_net_string_534fb8,
        "BE_SetNetString534fb8",
    ),
    (0x0014_ebf8, be_backup_player_setup, "BE_BackupPlayerSetup"),
    (
        0x0014_eca0,
        be_restore_player_setup,
        "BE_RestorePlayerSetup",
    ),
    (
        0x0014_edd8,
        be_get_char_db_interface,
        "BE_GetCharDBInterface",
    ),
    (0x0014_ee28, be_get_rider_char_name, "BE_GetRiderCharName"),
    (
        0x0014_ee58,
        be_get_rider_display_name,
        "BE_GetRiderDisplayName",
    ),
    (
        0x0014_eec8,
        be_get_char_display_name,
        "BE_GetCharDisplayName",
    ),
    (
        0x0014_ef70,
        be_get_rider_char_db_flag14,
        "BE_GetRiderCharDBFlag14",
    ),
    (
        0x0015_c250,
        be_network_int_destroy_instance,
        "BENetworkInt_DestroyInstance",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::ai::callee::AI_RESET_RIDERS;
    pub(crate) use crate::game::ai::ai::callee::AI_SET_RIDER_ORDER_MODE;
    pub(crate) use crate::game::ai::ai::callee::BE_COPY_PROFILE_CHARACTERS;
    pub(crate) use crate::game::ai::ai::callee::INTC_RESTORE_REGS;
    pub(crate) use crate::game::ai::ai::callee::INTC_SAVE_REGS;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_FIND_INDEX;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_GET_ENTRY;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER32;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::ai::ai::callee::RIDER_FIND_NEARBY_LIGHTS;
    pub(crate) use crate::game::ai::ai::callee::WORLD_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
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
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY as IRRADIANCE_REF_GET_ENTRY_2;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;

    /// `cMemMan_alloc` (0x317d70, another shard: `call_indirect`).
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;

    pub const C_BOLTON_FX_REFRESH_EMITTERS: Callee = Callee {
        addr: 0x002e_ccb8,
        f: ee::s09::cBoltOnFXInterface_RefreshEmitters,
    };

    pub const IRRADIANCE_REF_GET_ENTRY: Callee = Callee {
        addr: 0x002e_e010,
        f: ee::s09::IrradianceRef_GetEntry,
    };

    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME0: Callee = Callee {
        addr: 0x002e_f0e8,
        f: ee::s09::WorldPainter_GetIrradianceName0,
    };

    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME1: Callee = Callee {
        addr: 0x002e_f140,
        f: ee::s09::WorldPainter_GetIrradianceName1,
    };

    pub const PATH_QUERY_EVENTS: Callee = Callee {
        addr: 0x0026_aa80,
        f: ee::s07::Path_QueryEvents,
    };

    pub const SSX_AUDIO_SYSTEM_GET_APP: Callee = Callee {
        addr: 0x0028_b1c0,
        f: ee::s07::SSXAudioSystem_GetApp,
    };

    pub const SSX_AUDIO_SYSTEM_GET_VIEW_COUNT: Callee = Callee {
        addr: 0x0028_b1f8,
        f: ee::s07::SSXAudioSystem_GetViewCount,
    };

    pub const INPUT_VM_INIT: Callee = Callee {
        addr: 0x0032_5250,
        f: ee::s10::InputVM_Init,
    };

    pub const INPUT_VM_EXECUTE: Callee = Callee {
        addr: 0x0032_5450,
        f: ee::s10::InputVM_Execute,
    };

    pub const INPUT_MAP_EVAL_FLOAT: Callee = Callee {
        addr: 0x0032_0fa8,
        f: ee::s10::InputMap_EvalFloat,
    };
}

// D_0045A280

/// `cBENetworkInterface` fields the singleton constructor initialises.
pub(crate) mod network_interface {

    /// `field_0x8`.
    pub const FLAGS: u32 = 0x8;

    /// The vtable pointer.
    pub const VTABLE: u32 = 0xc;
}

// ---------------------------------------------------------------------------
// Front-end character/economy helpers.

/// `const char *BE_GetCharDisplayName(uint char, uint index)` (0x14eec8): for
/// character ids 0xa..0x1d uses the 8-byte name table based at 0x43fa38, else
/// the entry at `index*0x88 + 0x530990`.
pub fn be_get_char_display_name(c: &mut Cpu) {
    c.cycles += 4;
    let ch = abi::arg(c, 2);
    let in_name_table = sx(ch.wrapping_sub(0xa)) < 0x14;
    c.g[V0][0] = in_name_table as u64;
    if in_name_table {
        c.cycles += 4;
        c.g[V1][0] = sx(ch << 3);
        c.g[V0][0] = sx(0x44_0000u32.wrapping_sub(0x5c8));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = 0x88;
    c.g[V0][0] = 0x53_0000;
    let entry = abi::mult(c, abi::arg(c, 1), 0x88);
    c.g[V1][0] = sx(entry);
    c.g[V0][0] = sx(0x53_0000u32.wrapping_add(0x990));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BENetworkInt *BENetworkInt_GetInstance(void)` (0x14e0e0).
pub fn be_network_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eec));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x59b8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_e108)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5268));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eec), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eec));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider names.

/// `const char *BE_GetRiderCharName(int rider)` (0x14ee28): index the rider's
/// character record by 0x88 and return the name string inside it.
pub fn be_get_rider_char_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ee38)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x530000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x530000u32.wrapping_add(0x970));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `const char *BE_GetRiderDisplayName(int rider)` (0x14ee58): the rider's
/// character's display name; for indices 0xa..0x1d it comes from one table
/// (0x43fa38), otherwise from the per-rider name table at 0x530990.
pub fn be_get_rider_display_name(c: &mut Cpu) {
    guest(c, |c| {
        const NAME_TABLE: u32 = 0x0053_0990;
        const INDEX_TABLE: u32 = 0x0043_fa38;
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ee74)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0014_a0b0,
            f: ssx3_ee::s01::f_0014a0b0,
        }
        .call(c, 0x0014_ee80)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
        c.g[V0][0] = (c.g[V0][0] < 0x14u64) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0x440000;
        } else {
            c.cycles += 5;
            c.g[V1][0] = 0x88;
            c.g[V0][0] = 0x530000;
            c.mult(0, V1, S1, V1);
            c.g[V0][0] = sx(0x530000u32.wrapping_add(0x990));
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // In-range character index: name = INDEX_TABLE + (index << 3).
        c.cycles += 2;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x5c8));
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let _ = (NAME_TABLE, INDEX_TABLE);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE singletons.

/// The gp-relative global holding each singleton instance (address
/// `gp - K`), the object size, and the debug name literal EA's allocator
/// records.
pub(crate) const NETWORK_GP: u32 = 0x1eec;

pub(crate) const NEW_PLAYER_GP: u32 = 0x1f30;

pub(crate) const NEW_RACE_GP: u32 = 0x1f3c;

pub(crate) const OPTION_GP: u32 = 0x1ee4;

/// `BENetworkInt_DestroyInstance()` (0x15c250): `__builtin_delete` the
/// singleton at `gp - 0x1eec` and clear the global.
pub fn be_network_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NETWORK_GP, 0x0015_c264);
}

/// `gp`-relative globals the originals address through `gp`.
pub(crate) const BE_NETWORK_INTERFACE: i32 = -0x1ee8;
// -0.0333333f
pub(crate) const BE_NETWORK_INTERFACE_VTABLE: u32 = 0x0045_ad38;
// D_0045AD38
pub(crate) const BE_NETWORK_INTERFACE_INFO: u32 = 0x0045_a280;

// ---------------------------------------------------------------------------
// Char database.

/// `cBENetworkInterface *BE_GetCharDBInterface()` (0x14edd8): lazily create
/// the character-database network interface and return the singleton. The
/// object is 0x10 bytes with the type-info word at `+0xc`.
pub fn be_get_char_db_interface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let slot = gp(c, BE_NETWORK_INTERFACE);
        let cached = Addr(slot).lw(c);
        c.g[V0][0] = cached;
        frame.save_ra(c, 0);
        if cached == 0 {
            // `cMemMan_alloc(0x10, type_info, 0, 0)`, then initialise it.
            c.cycles += 11;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(BE_NETWORK_INTERFACE_INFO);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, callee::MEM_MAN_ALLOC, 0x0014_ee00)?;
            let mem = c.g[V0][0] as u32;
            c.g[V1][0] = 0x0046_0000;
            Addr(mem).off(network_interface::FLAGS).sw(c, 0);
            c.g[V1][0] = sx(BE_NETWORK_INTERFACE_VTABLE);
            c.sw(mem.wrapping_add(network_interface::VTABLE), c.g[V1][0]);
            c.sw(slot, c.g[V0][0]);
        }
        c.cycles += 4;
        let instance = Addr(slot).lw(c);
        c.g[V0][0] = instance;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetNetString534fc8(const char *src)` (0x0014ebb8):
/// `strcpy(D_00534FC8, src)`.
pub fn be_set_net_string_534fc8(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x53_0000u32.wrapping_add(0x4fc8)),
        callee::STRLEN_AND_STRCPY,
        0x0014_ebcc,
    )
}

/// `void BE_SetNetString534fb8(const char *src)` (0x0014ebd8):
/// `strcpy(D_00534FB8, src)`.
pub fn be_set_net_string_534fb8(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x53_0000u32.wrapping_add(0x4fb8)),
        callee::STRLEN_AND_STRCPY,
        0x0014_ebec,
    )
}

/// `void BE_CopyProfile(int dstIndex, int srcIndex)` (0x14e9c0): copy one
/// `sBEPlayerData` record (stride 0x9b50) out of `D_004A6CA8`.
pub fn be_copy_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x004a_0000;
        c.g[V1][0] = 0x9b50;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        let frame = Frame::open(c, 0x10);
        c.mult(0, 2, 6, 3);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.mult(0, 2, 5, 3);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        callee::BE_COPY_PROFILE_CHARACTERS.call(c, 0x0014_e9e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front end.

/// `bool BE_GetRiderCharDBFlag14(int riderIndex)` (0x14ef70): whether byte 0x44
/// of the rider's character-DB record is non-zero.
pub fn be_get_rider_char_db_flag14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_ef80)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x970));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Option-block copies (a struct assignment of a fixed size).

/// Copy `size` bytes from `v1` to `a0` the way gcc expands a known-size
/// `memcpy`: 8-byte words while `v1 != v0`, then an overlapping `ldl`/`ldr`
/// tail. `v1`, `v0` (the end address) and `a0` are the original's loop
/// registers and are left with the values the epilogue reads.
pub(crate) fn copy_block(c: &mut Cpu, head: u32, tail_cycles: u64, tail_pairs: u32) {
    if (c.g[V1][0] | c.g[A0][0]) & 7 == 0 {
        let end = c.g[V0][0];
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(ea(c.g[V1][0], 0));
            c.g[A2][0] = c.ld(ea(c.g[V1][0], 8));
            c.g[A3][0] = c.ld(ea(c.g[V1][0], 0x10));
            c.g[T0][0] = c.ld(ea(c.g[V1][0], 0x18));
            c.sd(ea(c.g[A0][0], 0), c.g[A1][0]);
            c.sd(ea(c.g[A0][0], 8), c.g[A2][0]);
            c.sd(ea(c.g[A0][0], 0x10), c.g[A3][0]);
            c.sd(ea(c.g[A0][0], 0x18), c.g[T0][0]);
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            let more = c.g[V1][0] != end;
            c.g[A0][0] = sx(ea(c.g[A0][0], 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    } else {
        let end = c.g[V0][0];
        loop {
            c.cycles += 19;
            c.ldl(5, ea(c.g[V1][0], 7));
            c.ldr(5, ea(c.g[V1][0], 0));
            c.ldl(6, ea(c.g[V1][0], 0xf));
            c.ldr(6, ea(c.g[V1][0], 8));
            c.ldl(7, ea(c.g[V1][0], 0x17));
            c.ldr(7, ea(c.g[V1][0], 0x10));
            c.ldl(8, ea(c.g[V1][0], 0x1f));
            c.ldr(8, ea(c.g[V1][0], 0x18));
            c.sdl(5, ea(c.g[A0][0], 7));
            c.sdr(5, ea(c.g[A0][0], 0));
            c.sdl(6, ea(c.g[A0][0], 0xf));
            c.sdr(6, ea(c.g[A0][0], 8));
            c.sdl(7, ea(c.g[A0][0], 0x17));
            c.sdr(7, ea(c.g[A0][0], 0x10));
            c.sdl(8, ea(c.g[A0][0], 0x1f));
            c.sdr(8, ea(c.g[A0][0], 0x18));
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            let more = c.g[V1][0] != end;
            c.g[A0][0] = sx(ea(c.g[A0][0], 0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += tail_cycles;
    c.ldl(2, ea(c.g[V1][0], 7));
    c.ldr(2, ea(c.g[V1][0], 0));
    if tail_pairs == 2 {
        c.ldl(5, ea(c.g[V1][0], 0xf));
        c.ldr(5, ea(c.g[V1][0], 8));
    }
    c.sdl(2, ea(c.g[A0][0], 7));
    c.sdr(2, ea(c.g[A0][0], 0));
    if tail_pairs == 2 {
        c.sdl(5, ea(c.g[A0][0], 0xf));
        c.sdr(5, ea(c.g[A0][0], 8));
    }
    abi::ret(c);
}

/// `void* BENetworkInt_Commit()` (0x14E130): copy the live network-option
/// block (0x534B30) over the committed one at 0x535088.
pub fn be_network_int_commit(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5088));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x4b30));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x4a0));
    copy_block(c, 0x0014_e150, 9, 2);
}

/// `void* BENetworkInt_Reset()` (0x14E1F8): copy the committed network-option
/// block back over the live one.
pub fn be_network_int_reset(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x4b30));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5088));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x4a0));
    copy_block(c, 0x0014_e218, 9, 2);
}

// ---------------------------------------------------------------------------
// Online (BE) system.

/// `func_0014EA90` (0x14ea90): `BE_ResetNetInterfaces`. Re-initialises two BE
/// interfaces (ids 3 and 0xc) by calling their `vtable[0x10]` method with the
/// argument in `a1`, then refreshes the player-flag option
/// (`BEOptionInt_SwapPlayerFlags(this)`).
pub fn be_reset_net_interfaces(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_eaa4);
        c.g[S0][0] = c.g[A0][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0014_eaa4)?;
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x0014_eab0);
        c.g[A0][0] = c.g[V0][0];
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0014_eab0)?;
        c.g[V1][0] = lw_r(c, V0, 0xc);
        c.g[A0][0] = lh_r(c, V1, 0x10);
        c.g[A1][0] = lw_r(c, V1, 0x14);
        let target = low(c, A1);
        c.g[RA][0] = sx(0x0014_eac4);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0014_eac4)?;
        c.g[RA][0] = sx(0x0014_eacc);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0014_eacc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014_ead8);
        c.g[A1][0] = 0xc;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0014_ead8)?;
        c.g[V1][0] = lw_r(c, V0, 0xc);
        c.g[A0][0] = lh_r(c, V1, 0x10);
        c.g[A1][0] = lw_r(c, V1, 0x14);
        let target = low(c, A1);
        c.g[RA][0] = sx(0x0014_eaec);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0014_eaec)?;
        c.g[RA][0] = sx(0x0014_eaf4);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS.call(c, 0x0014_eaf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_PrepareNetProfiles` (0x14e9f8): reset the profile characters and copy
/// the current profile into the second slot unless a net profile exists.
pub fn be_prepare_net_profiles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004c_0000;
        c.g[RA][0] = sx(0x0014_ea18);
        c.g[A0][0] = sx(0x004c_0000u32.wrapping_sub(0x5cb8));
        callee::BE_RESET_PROFILE_CHARACTERS.call(c, 0x0014_ea18)?;
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = Addr(0x0053_0000u32.wrapping_add(0x4b38)).lw(c);
        let have_net = c.g[V1][0] != 0;
        c.g[V1][0] = 0x0053_0000;
        if !have_net {
            c.cycles += 20;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5b3c));
            c.g[A1][0] = 0;
            c.g[T2][0] = sx(0x0053_0000u32.wrapping_add(0x5b3c));
            let v0 = r(c, V0);
            let t2 = r(c, T2);
            c.ldl(7, v0.wrapping_sub(0x15));
            c.ldr(7, v0.wrapping_sub(0x1c));
            c.ldl(8, v0.wrapping_sub(0xd));
            c.ldr(8, v0.wrapping_sub(0x14));
            c.ldl(9, v0.wrapping_sub(5));
            c.ldr(9, v0.wrapping_sub(0xc));
            c.sdl(7, t2.wrapping_add(7));
            c.sdr(7, t2);
            c.sdl(8, t2.wrapping_add(0xf));
            c.sdr(8, t2.wrapping_add(8));
            c.sdl(9, t2.wrapping_add(0x17));
            c.sdr(9, t2.wrapping_add(0x10));
            c.g[A3][0] = Addr(v0.wrapping_sub(4)).lw(c);
            Addr(t2.wrapping_add(0x18)).sw(c, c.g[A3][0]);
            c.g[RA][0] = sx(0x0014_ea78);
            c.g[A2][0] = 1;
            callee::BE_COPY_PROFILE.call(c, 0x0014_ea78)?;
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x0014_ea80);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS.call(c, 0x0014_ea80)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE character database backup.

/// `void BE_BackupPlayerSetup()` (0x14ebf8): back up the local player's
/// character and copy the six rival records into the backup table.
pub fn be_backup_player_setup(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 20;
        c.g[A0][0] = c.g[A1][0];
        c.g[V1][0] = 0x1c;
        c.mult(0, 3, 4, 3);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x9b50;
        c.mult(0, 4, 4, 2);
        c.g[S0][0] = sx(0x0053_0000);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(0x0053_5b20);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = sx(0x004a_0000);
        c.g[V0][0] = sx(0x004a_6ca8);
        c.g[A1][0] = Addr(at(c.g[V1][0], 0x11)).lb(c);
        c.g[RA][0] = sx(0x0014_ec38);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::BE_BACKUP_CHARACTER.call(c, 0x0014_ec38)?;
        c.g[V0][0] = sx(0x0053_0000);
        c.g[V1][0] = sx(0x0053_4a88);
        c.g[A0][0] = sx(at(c.g[V1][0], 0xa8));
        loop {
            c.cycles += 18;
            c.ldl(2, at(c.g[S0][0], 7));
            c.ldr(2, c.g[S0][0] as u32);
            c.ldl(5, at(c.g[S0][0], 0xf));
            c.ldr(5, at(c.g[S0][0], 8));
            c.ldl(6, at(c.g[S0][0], 0x17));
            c.ldr(6, at(c.g[S0][0], 0x10));
            c.g[A3][0] = lw(c, at(c.g[S0][0], 0x18));
            c.sdl(2, at(c.g[V1][0], 7));
            c.sdr(2, c.g[V1][0] as u32);
            c.sdl(5, at(c.g[V1][0], 0xf));
            c.sdr(5, at(c.g[V1][0], 8));
            c.sdl(6, at(c.g[V1][0], 0x17));
            c.sdr(6, at(c.g[V1][0], 0x10));
            Addr(at(c.g[V1][0], 0x18)).sw(c, c.g[A3][0]);
            c.g[V1][0] = sx(at(c.g[V1][0], 0x1c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx(at(c.g[S0][0], 0x1c));
            if !t {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0014_ec48);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// `BEOptionInt_SwapPlayerFlags` (0x14eb08): exchange the two network player
// slots' flag words and ids (a hand-unrolled 6-word swap).
pub fn beoption_int_swap_player_flags(c: &mut Cpu) {
    c.cycles += 43;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[T1][0] = 0x530000;
    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[SP][0] as u32));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A2][0] = 0xffff_ffff_ffff_fff7;
    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.g[V0][0] = c.ld((c.g[SP][0] as u32));
    c.g[T2][0] = 0xffff_ffff_ffff_fffe;
    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] << 29;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[T3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.g[V0][0] & 0x1;
    c.g[A0][0] = c.g[A3][0] & c.g[A2][0];
    c.g[T0][0] = c.g[A3][0] & 0x8;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[A2][0] = c.lw((c.g[SP][0] as u32));
    c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[A0][0] = c.g[A0][0] & c.g[T2][0];
    c.g[A3][0] = c.g[A3][0] & 0x1;
    c.g[T2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A2][0] = c.g[A2][0] & 0x1;
    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
    c.sw((c.g[T1][0] as u32).wrapping_add(0x8b8), c.g[V1][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[T3][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[T0][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[A0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `ldl rt, a+7; ldr rt, a`: one unaligned 8-byte load, merged into `rt`
/// (the pre-existing bits of `rt` matter, as in the original).
#[inline(always)]
pub(crate) fn ldl(c: &mut Cpu, rt: usize, a: u32) {
    c.ldl(rt, a.wrapping_add(7));
    c.ldr(rt, a);
}

/// `sdl rt, a+7; sdr rt, a`: one unaligned 8-byte store.
#[inline(always)]
pub(crate) fn sdl(c: &mut Cpu, rt: usize, a: u32) {
    c.sdl(rt, a.wrapping_add(7));
    c.sdr(rt, a);
}

/// `void func_0014ECA0(int self, int idx)` (0x14eca0): restore a BE player
/// character setup: `BE_RestoreCharacter(&D_004A6CA8[idx])`, mirror
/// `D_00535B20[0]` from `[1]` when `D_00534B38` is clear, copy
/// `D_00535B20[1..6]` from `D_00534A88`, then let the BE interfaces 1 and 6
/// act on it (vtable slot 2 of each).
pub fn be_restore_player_setup(c: &mut Cpu) {
    let frame = Frame::open(c, 0x10);
    'b_14ed14: {
        c.cycles += 12;
        c.g[V0][0] = 0x9b50;
        c.g[A0][0] = 0x4a0000;
        c.mult(0, A1, A1, V0);
        c.g[A0][0] = sx(at_2(c, A0, 0x6ca8));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x0014ecc0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if callee::C00156988.call(c, 0x0014ecc0).is_err() {
            return;
        }
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = lw_2(c, at_2(c, V0, 0x4b38));
        let t = c.g[V1][0] != 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_14ed14;
        } else {
            c.cycles += 17;
            c.g[V0][0] = sx(at_2(c, V1, 0x5b20));
            c.g[A3][0] = sx(at_2(c, V1, 0x5b20));
            ldl(c, 4, at_2(c, V0, 0x1c));
            ldl(c, 5, at_2(c, V0, 0x24));
            ldl(c, 6, at_2(c, V0, 0x2c));
            sdl(c, 4, r32(c, A3));
            sdl(c, 5, at_2(c, A3, 0x8));
            sdl(c, 6, at_2(c, A3, 0x10));
            c.g[A0][0] = lw_2(c, at_2(c, V0, 0x34));
            sw(c, at_2(c, A3, 0x18), c.g[A0][0]);
            c.g[V1][0] = 0x530000;
            break 'b_14ed14;
        }
    }
    c.cycles += 7;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = sx(at_2(c, V1, 0x5b20));
    c.g[V0][0] = sx(at_2(c, V0, 0x4a88));
    c.g[A1][0] = sx(at_2(c, V1, 0xa8));
    c.g[A0][0] = sx(at_2(c, V0, 0x1c));
    c.g[V1][0] = sx(at_2(c, V1, 0x1c));
    'l_14ed30: loop {
        c.cycles += 18;
        ldl(c, 8, r32(c, A0));
        ldl(c, 2, at_2(c, A0, 0x8));
        ldl(c, 6, at_2(c, A0, 0x10));
        c.g[A3][0] = lw_2(c, at_2(c, A0, 0x18));
        sdl(c, 8, r32(c, V1));
        sdl(c, 2, at_2(c, V1, 0x8));
        sdl(c, 6, at_2(c, V1, 0x10));
        sw(c, at_2(c, V1, 0x18), c.g[A3][0]);
        c.g[V1][0] = sx(at_2(c, V1, 0x1c));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A0][0] = sx(at_2(c, A0, 0x1c));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0014ed30);
            }
            continue 'l_14ed30;
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0014ed80);
            if callee::C0014dc80.call(c, 0x0014ed80).is_err() {
                return;
            }
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x0014ed8c);
            c.g[A0][0] = c.g[V0][0];
            if callee::C0014dd58.call(c, 0x0014ed8c).is_err() {
                return;
            }
            c.g[V1][0] = lw_2(c, at_2(c, V0, 0xc));
            c.g[A0][0] = lh(c, at_2(c, V1, 0x10));
            c.g[A1][0] = lw_2(c, at_2(c, V1, 0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014eda0);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0014eda0).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x0014eda8);
            if callee::C0014dc80.call(c, 0x0014eda8).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014edb4);
            c.g[A1][0] = 0x6;
            if callee::C0014dd58.call(c, 0x0014edb4).is_err() {
                return;
            }
            c.g[V1][0] = lw_2(c, at_2(c, V0, 0xc));
            c.g[A0][0] = lh(c, at_2(c, V1, 0x10));
            c.g[A1][0] = lw_2(c, at_2(c, V1, 0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014edc8);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0014edc8).is_err() {
                return;
            }
            c.g[RA][0] = ld(c, r32(c, SP));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at_2(c, SP, 0x10));
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
    use crate::difftest::compare;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::be::bedbreward::be_post_race_destroy_instance;
    pub(crate) use crate::game::be::bedbreward::be_pre_race_destroy_instance;
    pub(crate) use crate::game::be::bedbreward::be_race_destroy;
    pub(crate) use crate::game::be::beintmission::tests::instance_setup;
    pub(crate) use crate::game::be::beintnewrace::be_new_race_int_destroy_instance;
    pub(crate) use crate::game::be::beintoption::be_option_int_destroy_instance;
    pub(crate) use crate::game::be::beintplayer::be_new_player_int_destroy_instance;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn be_get_char_display_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014eec8,
            be_get_char_display_name,
            300,
            &|c, seed| {
                let mut r = Rng(0x4000_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let ch = match seed % 3 {
                    0 => 0xa + (seed % 0x14),
                    1 => seed % 0xa,
                    _ => r.next_u32(),
                };
                c.g[A2][0] = ch as u64;
            },
        );
    }

    #[test]
    fn be_network_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_0014e0e0,
            be_network_int_get_instance,
            90,
            &|c, seed| {
                instance_setup(c, seed, 0x1eec, 0xbe7);
            },
        );
    }

    #[test]
    fn be_get_rider_char_name_matches() {
        check(
            ssx3_ee::s01::f_0014ee28,
            be_get_rider_char_name,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xee28, 0);
                c.g[A1][0] = (seed % 12) as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn be_get_rider_display_name_matches() {
        check(
            ssx3_ee::s01::f_0014ee58,
            be_get_rider_display_name,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xee58, 0);
                c.g[A1][0] = (seed % 12) as u64;
                let _ = &mut r;
            },
        );
    }

    /// The allocator mutex; make it look already held by "this thread" so the
    /// real `EA_MemAllocNamed`/`__builtin_delete` take their fast path.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;

    pub(crate) fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    pub(crate) fn destroy_test(generated: GuestFn, hand: GuestFn, gp_off: u32, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, salt, 6);
            let _ = &mut r;
            alloc_setup(c, &mut r);
            c.bus.write32(
                GAME_GP - gp_off,
                if seed % 3 == 0 {
                    0
                } else {
                    0x0080_0000u32 + seed * 0x40
                },
            );
        });
    }

    #[test]
    fn be_destroy_instances_match() {
        destroy_test(
            ssx3_ee::s01::f_0015c250,
            be_network_int_destroy_instance,
            NETWORK_GP,
            1,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bc88,
            be_new_player_int_destroy_instance,
            NEW_PLAYER_GP,
            2,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bb20,
            be_new_race_int_destroy_instance,
            NEW_RACE_GP,
            3,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bd30,
            be_option_int_destroy_instance,
            OPTION_GP,
            4,
        );
        destroy_test(
            ssx3_ee::s01::f_0015c830,
            be_pre_race_destroy_instance,
            PRE_RACE_GP,
            5,
        );
        destroy_test(ssx3_ee::s01::f_0015c870, be_race_destroy, RACE_GP, 6);
        destroy_test(
            ssx3_ee::s01::f_0015c8b0,
            be_post_race_destroy_instance,
            POST_RACE_GP,
            7,
        );
    }

    // ------------------------------------------------- char database singleton

    #[test]
    fn be_get_char_db_interface_matches() {
        check(
            ee::s01::f_0014edd8__generated,
            be_get_char_db_interface,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xedd8, 0);
                // Mark the allocator's mutex as held by "this thread" (the fake
                // `GetThreadId` returns 0x42) so the real allocator takes its fast
                // path.
                c.bus.write32(MEM_MUTEX + 4, 0x42);
                c.bus.write32(LOG_RESULTS, 0x42);
                c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
                let slot = GAME_GP.wrapping_sub(0x1ee8);
                // Null hits the allocation path, non-null returns the singleton.
                c.bus
                    .write32(slot, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            },
        );
    }

    /// NUL-terminated `s` at `addr`.
    pub(crate) fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    pub(crate) fn net_string_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x14_ebb8, 0);
        let src = DATA + 0x800;
        match seed % 3 {
            0 => put_str(c, src, ""),
            1 => put_str(c, src, "ssx3"),
            _ => put_str(c, src, "a longer network string"),
        }
        c.g[A1][0] = src as u64;
    }

    #[test]
    fn be_set_net_string_534fc8_matches() {
        check(
            ee::s01::f_0014ebb8__generated,
            be_set_net_string_534fc8,
            200,
            &net_string_setup,
        );
    }

    #[test]
    fn be_set_net_string_534fb8_matches() {
        check(
            ee::s01::f_0014ebd8__generated,
            be_set_net_string_534fb8,
            200,
            &net_string_setup,
        );
    }

    #[test]
    fn be_copy_profile_matches() {
        check(
            ee::s01::f_0014e9c0__generated,
            be_copy_profile,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14e9_c0, 2);
                be_library(c);
                // Two of the four 0x9b50-byte player records (`D_004A6CA8`).
                c.g[A0][0] = (r.next_u32() % 4) as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    // ------------------------------------------------------------ front end

    #[test]
    fn be_get_rider_char_db_flag14_matches() {
        check(
            ee::s01::f_0014ef70__generated,
            be_get_rider_char_db_flag14,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14ef_70, 0);
                let idx = seed % 4;
                let ch = idx as u32;
                c.g[A1][0] = sx(idx);
                c.bus.write32(0x0053_05b0 + 4 * idx, ch);
                c.bus
                    .write8(0x0053_05b20 + ch + 0x11, (r.next_u32() & 3) as u8);
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(0x0053_0970 + ch * 0x88 + 0x44, flag);
            },
        );
    }

    // ------------------------------------------------------- option blocks

    #[test]
    fn be_network_int_commit_matches() {
        check(
            ee::s01::f_0014e130__generated,
            be_network_int_commit,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_e130, 0);
                belib(c);
                for i in 0..0x128u32 {
                    w(c, 0x0053_4b30 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_network_int_reset_matches() {
        check(
            ee::s01::f_0014e1f8__generated,
            be_network_int_reset,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_e1f8, 0);
                belib(c);
                for i in 0..0x128u32 {
                    w(c, 0x0053_5088 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_reset_net_interfaces_matches() {
        check(
            ee::s01::f_0014ea90__generated,
            be_reset_net_interfaces,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x14ea90, 6);
                world(c);
                be_library_2(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn be_prepare_net_profiles_matches() {
        check(
            ee::s01::f_0014e9f8__generated,
            be_prepare_net_profiles,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0xe9f8, 3);
                let this = DATA;
                w(c, 0x0053_4b38, (seed & 1) as u32);
                w(c, 0x0053_5b3c - 0x1c, 0);
                w(c, 0x0053_5b3c - 4, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    // -------------------------------------------------------------- BE_2 data

    #[test]
    fn be_backup_player_setup_matches() {
        check(
            ee::s01::f_0014ebf8__generated,
            be_backup_player_setup,
            SEEDS_5,
            &|c, seed| {
                let _ = base_2(c, seed, 0x14eb_f8, 4);
                c.g[A0][0] = 0x004a_6ca8u64;
                c.g[A1][0] = (seed % 4) as u64;
                // The character table at 0x535b20 (the source of the copy).
                for i in 0..0x100u32 {
                    w(c, 0x0053_5b20 + 4 * i, seed.wrapping_add(i));
                }
            },
        );
    }

    #[test]
    fn beoption_int_swap_player_flags_matches() {
        check(
            ee::s01::f_0014eb08__generated,
            beoption_int_swap_player_flags,
            100,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x14eb);
                for k in 0..0x20 / 4 {
                    c.bus.write32(0x0053_08b8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_restore_player_setup_matches() {
        check_all(
            ssx3_ee::s01::f_0014eca0__generated,
            be_restore_player_setup,
            40,
            &|c, seed| {
                let _ = base_4(c, seed, 0x14ec_a0, 8);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx((seed % 6) * 4); // &D_004A6CA8[idx]
                                                 // `D_00534B38` picks the mirrored-first-element branch.
                c.bus
                    .write32(0x0053_4b38, if seed % 3 == 0 { 0 } else { 1 });
                put_words(c, 0x0053_5b20, &[0, 0, 0, 0, 0, 0, 0, 0]);
            },
        );
    }
}
